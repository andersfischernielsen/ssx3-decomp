//! Recorded pipeline descriptors and their background writer.
//! No driver objects or game data.

use super::{PipeKey, Prewarm};
use ssx3_render::gpu::Program;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, OnceLock};
use std::time::{Duration, Instant};

/// Key-file magic; [`renderer_hash`] covers format changes.
const MAGIC: &[u8; 8] = b"SSX3PLK1";
/// Header layout: magic, renderer hash, key count, checksum.
const RENDERER: std::ops::Range<usize> = 8..16;
const COUNT: std::ops::Range<usize> = 16..18;
const SUM: std::ops::Range<usize> = 18..26;
const HEADER: usize = SUM.end;
const RECORD: usize = 16;
// Record layout: kind, blend flag, blend factors and operation, depth state,
// program, lighting paths, zero padding.
const KIND: usize = 0;
const BLEND_FLAG: usize = 1;
const BLEND_SRC: usize = 2;
const BLEND_DST: usize = 3;
const BLEND_OP: usize = 4;
const DEPTH: usize = 5;
const DEPTH_WRITE: usize = 6;
const WRITE_MASK: usize = 7;
/// The widest mask (`wgpu::ColorWrites` red, green, blue and alpha).
const WRITE_MASK_MAX: u8 = 15;
const PROGRAM: usize = 8;
const LIT: std::ops::Range<usize> = 9..13;
/// `b[PAD..]` is zero.
const PAD: usize = 13;
/// Keys past this are created on demand, not recorded; 512 bounds the file to 8 KiB.
const MAX_KEYS: usize = 512;
const FILE: &str = "ssx3-pipeline-keys.bin";
/// Present from the start of pipeline setup until its prewarm ends ([`start`]).
const MARKER: &str = "ssx3-pipeline-prewarm.running";

const FNV_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

fn hash(bytes: &[u8], mut h: u64) -> u64 {
    for &b in bytes {
        h = (h ^ u64::from(b)).wrapping_mul(FNV_PRIME);
    }
    h
}

/// Descriptor, shader or layout changes invalidate saved keys.
pub(super) fn renderer_hash() -> u64 {
    static HASH: OnceLock<u64> = OnceLock::new();
    *HASH.get_or_init(|| {
        let mut h = hash(include_bytes!("../draw.rs"), FNV_BASIS);
        // This file (the record format and what it maps to) invalidates too.
        h = hash(include_bytes!("persist.rs"), h);
        for source in super::GPUVU_SOURCES {
            h = hash(source.as_bytes(), h);
        }
        for source in [
            include_str!("../shadow.wgsl"),
            include_str!("../gpuvu/clip_depth.wgsl"),
        ] {
            h = hash(source.as_bytes(), h);
        }
        // The program numbers saved in the records, and what they draw.
        for v in 0..=u8::MAX {
            if let Some(p) = Program::from_u8(v) {
                h = hash(&[v], h);
                h = hash(p.vs_entry().as_bytes(), h);
            }
        }
        h = hash(
            &(std::mem::size_of::<crate::gs::Vtx>() as u64).to_le_bytes(),
            h,
        );
        hash(&super::BATCH_BYTES.to_le_bytes(), h)
    })
}

/// One table per enum, read both ways, so the file cannot drift from wgpu's
/// numbering: `decode` indexes, `encode` takes the position.
const FACTORS: [wgpu::BlendFactor; 13] = {
    use wgpu::BlendFactor as F;
    [
        F::Zero,
        F::One,
        F::Src,
        F::OneMinusSrc,
        F::SrcAlpha,
        F::OneMinusSrcAlpha,
        F::Dst,
        F::OneMinusDst,
        F::DstAlpha,
        F::OneMinusDstAlpha,
        F::SrcAlphaSaturated,
        F::Constant,
        F::OneMinusConstant,
    ]
};
const OPERATIONS: [wgpu::BlendOperation; 5] = {
    use wgpu::BlendOperation as O;
    [O::Add, O::Subtract, O::ReverseSubtract, O::Min, O::Max]
};
const COMPARES: [wgpu::CompareFunction; 8] = {
    use wgpu::CompareFunction as C;
    [
        C::Never,
        C::Less,
        C::Equal,
        C::LessEqual,
        C::Greater,
        C::NotEqual,
        C::GreaterEqual,
        C::Always,
    ]
};

/// A value wgpu would not accept encodes out of range, which `decode` refuses.
fn table_index<T: PartialEq>(table: &[T], v: T) -> u8 {
    table
        .iter()
        .position(|e| *e == v)
        .unwrap_or(u8::MAX as usize) as u8
}

fn factor(v: u8) -> Option<wgpu::BlendFactor> {
    FACTORS.get(v as usize).copied()
}

fn operation(v: u8) -> Option<wgpu::BlendOperation> {
    OPERATIONS.get(v as usize).copied()
}

fn compare(v: u8) -> Option<wgpu::CompareFunction> {
    COMPARES.get(v.checked_sub(1)? as usize).copied()
}

/// The record kinds ([`Prewarm`]).
const KIND_CPU: u8 = 0;
const KIND_GPU: u8 = 1;
const KIND_SHADOW: u8 = 2;

fn encode(key: Prewarm) -> [u8; RECORD] {
    let (kind, pipe, program, lit) = match key {
        Prewarm::Cpu(k) => (KIND_CPU, Some(k), 0, 0),
        Prewarm::Gpu(k, p, lit) => (KIND_GPU, Some(k), p as u8, lit),
        Prewarm::Shadow(p) => (KIND_SHADOW, None, p as u8, 0),
    };
    let mut b = [0; RECORD];
    b[KIND] = kind;
    if let Some(k) = pipe {
        if let Some((src, dst, op)) = k.blend {
            b[BLEND_FLAG] = 1;
            b[BLEND_SRC] = table_index(&FACTORS, src);
            b[BLEND_DST] = table_index(&FACTORS, dst);
            b[BLEND_OP] = table_index(&OPERATIONS, op);
        }
        // The file numbers compares from 1, as wgpu does.
        b[DEPTH] = table_index(&COMPARES, k.depth).wrapping_add(1);
        b[DEPTH_WRITE] = u8::from(k.depth_write);
        b[WRITE_MASK] = k.write;
    }
    b[PROGRAM] = program;
    b[LIT].copy_from_slice(&lit.to_le_bytes());
    b
}

fn decode(b: &[u8]) -> Option<Prewarm> {
    if b.len() != RECORD || b[PAD..].iter().any(|&v| v != 0) {
        return None;
    }
    if b[KIND] == KIND_SHADOW {
        if b[BLEND_FLAG..PROGRAM].iter().any(|&v| v != 0) || b[LIT].iter().any(|&v| v != 0) {
            return None;
        }
        return Some(Prewarm::Shadow(Program::from_u8(b[PROGRAM])?));
    }
    let blend = match b[BLEND_FLAG] {
        0 if b[BLEND_SRC..=BLEND_OP].iter().all(|&v| v == 0) => None,
        1 => Some((
            factor(b[BLEND_SRC])?,
            factor(b[BLEND_DST])?,
            operation(b[BLEND_OP])?,
        )),
        _ => return None,
    };
    // wgpu rejects Min and Max unless both factors are One.
    if let Some((s, d, wgpu::BlendOperation::Min | wgpu::BlendOperation::Max)) = blend {
        if (s, d) != (wgpu::BlendFactor::One, wgpu::BlendFactor::One) {
            return None;
        }
    }
    if b[DEPTH_WRITE] > 1 || b[WRITE_MASK] > WRITE_MASK_MAX {
        return None;
    }
    let pipe = PipeKey {
        blend,
        depth: compare(b[DEPTH])?,
        depth_write: b[DEPTH_WRITE] != 0,
        write: b[WRITE_MASK],
    };
    let lit = u32::from_le_bytes(b[LIT].try_into().ok()?);
    match b[KIND] {
        KIND_CPU if b[PROGRAM] == 0 && lit == 0 => Some(Prewarm::Cpu(pipe)),
        KIND_GPU if lit <= super::ALL_LIT_PATHS => {
            Some(Prewarm::Gpu(pipe, Program::from_u8(b[PROGRAM])?, lit))
        }
        _ => None,
    }
}

fn serialize(keys: &[Prewarm], renderer: u64) -> Vec<u8> {
    let mut body = Vec::with_capacity(keys.len() * RECORD);
    for &key in keys {
        body.extend_from_slice(&encode(key));
    }
    let mut out = Vec::with_capacity(HEADER + body.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&renderer.to_le_bytes());
    out.extend_from_slice(&(keys.len() as u16).to_le_bytes());
    out.extend_from_slice(&hash(&body, FNV_BASIS).to_le_bytes());
    out.extend_from_slice(&body);
    out
}

fn parse(bytes: &[u8], renderer: u64) -> Option<Vec<Prewarm>> {
    if bytes.len() < HEADER || bytes.len() > HEADER + MAX_KEYS * RECORD || &bytes[..8] != MAGIC {
        return None;
    }
    if u64::from_le_bytes(bytes[RENDERER].try_into().ok()?) != renderer {
        return None;
    }
    let count = u16::from_le_bytes(bytes[COUNT].try_into().ok()?) as usize;
    let body = &bytes[HEADER..];
    if count > MAX_KEYS || body.len() != count * RECORD {
        return None;
    }
    if u64::from_le_bytes(bytes[SUM].try_into().ok()?) != hash(body, FNV_BASIS) {
        return None;
    }
    let mut seen = HashSet::new();
    let mut keys = Vec::with_capacity(count);
    for record in body.chunks_exact(RECORD) {
        let key = decode(record)?;
        if !seen.insert(key) {
            return None;
        }
        keys.push(key);
    }
    Some(keys)
}

/// The cache folder, if any; none keeps nothing.
pub(super) fn dir() -> Option<PathBuf> {
    ssx3_core::platform_paths::cache_dir()
}

/// Older than this, a temp file is a crashed run's.
const STALE_TMP_AGE: Duration = Duration::from_secs(3600);

/// Remove temporary files a crashed run left behind ([`STALE_TMP_AGE`]).
fn sweep(dir: &Path) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let old = entry
            .metadata()
            .and_then(|m| m.modified())
            .is_ok_and(|t| t.elapsed().is_ok_and(|age| age > STALE_TMP_AGE));
        if old && name.starts_with("ssx3-") && name.ends_with(".tmp") {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

/// A file's bytes, unless it is missing or unreadable. A file over `max`
/// comes back empty, which every parser rejects as corrupt.
pub(super) fn read_capped(path: &Path, max: usize) -> Option<Vec<u8>> {
    use std::io::Read;
    let mut bytes = Vec::new();
    std::fs::File::open(path)
        .ok()?
        .take(max as u64 + 1)
        .read_to_end(&mut bytes)
        .ok()?;
    if bytes.len() > max {
        bytes.clear();
    }
    Some(bytes)
}

/// Atomic replace; the temporary name is per process so two running copies
/// cannot write over each other's half-finished file.
pub(super) fn write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let temp = path.with_extension(format!("{}.tmp", std::process::id()));
    std::fs::write(&temp, bytes)?;
    std::fs::rename(temp, path)
}

type Job = Box<dyn FnOnce() + Send>;

/// Writes run on their own thread; a cut-off write leaves the last complete file.
pub(super) struct Writer {
    tx: Option<mpsc::Sender<Job>>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Writer {
    /// `None` if the thread cannot start: nothing is written then.
    pub(super) fn start() -> Option<Self> {
        let (tx, rx) = mpsc::channel::<Job>();
        let thread = std::thread::Builder::new()
            .name("pipeline-writer".into())
            .spawn(move || rx.into_iter().for_each(|job| job()))
            .ok()?;
        Some(Self {
            tx: Some(tx),
            thread: Some(thread),
        })
    }

    pub(super) fn run(&self, job: impl FnOnce() + Send + 'static) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(Box::new(job));
        }
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        self.tx = None;
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// Mark setup's start; `true` if the last start did not finish.
pub(super) fn start() -> Option<(StartMark, bool)> {
    let dir = dir()?;
    eprintln!("[gfx] pipeline files in {}", dir.display());
    Some(start_in(&dir))
}

fn start_in(dir: &Path) -> (StartMark, bool) {
    let mark = dir.join(MARKER);
    let found = mark.exists();
    // Without a marker nothing would guard this start: begin without the
    // saved files as well.
    let written = write(&mark, b"");
    if found {
        eprintln!(
            "[gfx] the last start did not finish preparing its pipelines: starting without the saved pipelines"
        );
    } else if let Err(e) = &written {
        eprintln!(
            "[gfx] cannot write {}: {e}: starting without the saved pipelines",
            mark.display()
        );
    }
    (StartMark(mark), found || written.is_err())
}

/// Setup marker: removed on drop, kept on panic.
pub(super) struct StartMark(PathBuf);

impl Drop for StartMark {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let _ = std::fs::remove_file(&self.0);
        }
    }
}

/// At most one key-file write per interval; the rest waits for the next flush.
const KEY_FLUSH_EVERY: Duration = Duration::from_secs(1);

pub(super) struct KeyStore {
    path: PathBuf,
    renderer: u64,
    keys: Vec<Prewarm>,
    seen: HashSet<Prewarm>,
    dirty: bool,
    last_write: Option<Instant>,
    writer: Arc<Writer>,
}

impl KeyStore {
    /// The saved keys, or none with `fresh` (the file is then rewritten
    /// empty).
    pub(super) fn open(writer: Arc<Writer>, fresh: bool) -> Option<Self> {
        let folder = dir()?;
        let path = folder.join(FILE);
        writer.run(move || sweep(&folder));
        Some(Self::at(path, renderer_hash(), writer, fresh))
    }

    fn at(path: PathBuf, renderer: u64, writer: Arc<Writer>, fresh: bool) -> Self {
        let bytes = read_capped(&path, HEADER + MAX_KEYS * RECORD);
        let keys = (!fresh)
            .then(|| bytes.as_deref().and_then(|b| parse(b, renderer)))
            .flatten();
        if bytes.is_some() && keys.is_none() {
            let (path, empty) = (path.clone(), serialize(&[], renderer));
            writer.run(move || {
                let _ = write(&path, &empty);
            });
        }
        let keys = keys.unwrap_or_default();
        let seen = keys.iter().copied().collect();
        Self {
            path,
            renderer,
            keys,
            seen,
            dirty: false,
            last_write: None,
            writer,
        }
    }

    pub(super) fn keys(&self) -> Vec<Prewarm> {
        self.keys.clone()
    }

    pub(super) fn record(&mut self, key: Prewarm) {
        if self.keys.len() < MAX_KEYS && self.seen.insert(key) {
            self.keys.push(key);
            self.dirty = true;
            self.flush_due();
        }
    }

    pub(super) fn flush_due(&mut self) {
        let due = self
            .last_write
            .is_none_or(|t| t.elapsed() >= KEY_FLUSH_EVERY);
        if self.dirty && due {
            self.dirty = false;
            self.last_write = Some(Instant::now());
            let (path, bytes) = (self.path.clone(), serialize(&self.keys, self.renderer));
            self.writer.run(move || {
                let _ = write(&path, &bytes);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Prewarm {
        Prewarm::Gpu(
            PipeKey {
                blend: Some((
                    wgpu::BlendFactor::One,
                    wgpu::BlendFactor::DstAlpha,
                    wgpu::BlendOperation::Add,
                )),
                depth: wgpu::CompareFunction::GreaterEqual,
                depth_write: true,
                write: 15,
            },
            Program::Mat5Snow,
            3,
        )
    }

    /// Both directions read one table per enum: every blend and depth state
    /// wgpu accepts must come back the same, and Min or Max with other
    /// factors than One (which wgpu rejects) not at all.
    #[test]
    fn every_key_value_round_trips() {
        use wgpu::{BlendFactor as F, BlendOperation as O, CompareFunction as C};
        let factors = [
            F::Zero,
            F::One,
            F::Src,
            F::OneMinusSrc,
            F::SrcAlpha,
            F::OneMinusSrcAlpha,
            F::Dst,
            F::OneMinusDst,
            F::DstAlpha,
            F::OneMinusDstAlpha,
            F::SrcAlphaSaturated,
            F::Constant,
            F::OneMinusConstant,
        ];
        let ops = [O::Add, O::Subtract, O::ReverseSubtract, O::Min, O::Max];
        let depths = [
            C::Never,
            C::Less,
            C::Equal,
            C::LessEqual,
            C::Greater,
            C::NotEqual,
            C::GreaterEqual,
            C::Always,
        ];
        for &src in &factors {
            for &dst in &factors {
                for &op in &ops {
                    for &depth in &depths {
                        let key = PipeKey {
                            blend: Some((src, dst, op)),
                            depth,
                            depth_write: true,
                            write: 15,
                        };
                        let key = Prewarm::Cpu(key);
                        let valid =
                            !matches!(op, O::Min | O::Max) || (src, dst) == (F::One, F::One);
                        assert_eq!(decode(&encode(key)), valid.then_some(key));
                    }
                }
            }
        }
        let mut dual_source = encode(sample());
        dual_source[BLEND_SRC] = 13;
        assert_eq!(decode(&dual_source), None);
    }

    #[test]
    fn an_unfinished_start_begins_afresh() {
        let dir = std::env::temp_dir().join(format!("ssx3-start-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (file, mark) = (dir.join(FILE), dir.join(MARKER));
        let open = |fresh| KeyStore::at(file.clone(), 9, Arc::new(Writer::start().unwrap()), fresh);
        let mut store = open(false);
        store.record(sample());
        drop(store);
        let (started, unfinished) = start_in(&dir);
        assert!(!unfinished && mark.exists());
        drop(started); // Finished: the marker goes.
        assert!(!mark.exists());
        assert_eq!(open(false).keys(), vec![sample()]);
        std::mem::forget(start_in(&dir).0); // The process died.
        let (started, unfinished) = start_in(&dir);
        assert!(unfinished);
        assert!(open(unfinished).keys().is_empty());
        drop(started);
        assert!(!mark.exists());
        assert!(open(false).keys().is_empty()); // The file was rewritten empty.
        std::fs::remove_file(&file).unwrap();
        std::fs::remove_dir(&dir).unwrap();
    }

    #[test]
    fn an_unwritable_marker_begins_afresh() {
        let id = std::process::id();
        let dir = std::env::temp_dir().join(format!("ssx3-unwritable-{id}"));
        let file = std::env::temp_dir().join(format!("ssx3-unwritable-keys-{id}.bin"));
        // A regular file where the folder should be: the marker cannot be
        // written under it.
        std::fs::write(&dir, b"").unwrap();
        let open = |fresh| KeyStore::at(file.clone(), 9, Arc::new(Writer::start().unwrap()), fresh);
        let mut store = open(false);
        store.record(sample());
        drop(store);
        let (started, unfinished) = start_in(&dir);
        assert!(unfinished);
        drop(started);
        assert_eq!(open(false).keys(), vec![sample()]); // The key file is valid,
        assert!(open(unfinished).keys().is_empty()); // but not used.
        std::fs::remove_file(&file).unwrap();
        std::fs::remove_file(&dir).unwrap();
    }

    #[test]
    fn sweep_removes_only_old_temp_files() {
        let dir = std::env::temp_dir().join(format!("ssx3-sweep-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let old = std::time::SystemTime::now() - Duration::from_secs(7200);
        let make = |name: &str, aged: bool| {
            let f = std::fs::File::create(dir.join(name)).unwrap();
            if aged {
                f.set_modified(old).unwrap();
            }
        };
        make("ssx3-pipeline-keys.123.tmp", true); // A crashed run's: removed.
        make("ssx3-pipeline-keys.456.tmp", false); // Still being written.
        make("other.789.tmp", true); // Not ours.
        make("ssx3-pipeline-keys.bin", true);
        sweep(&dir);
        let mut left: Vec<_> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
        left.sort();
        assert_eq!(
            left,
            [
                "other.789.tmp",
                "ssx3-pipeline-keys.456.tmp",
                "ssx3-pipeline-keys.bin"
            ]
        );
        for name in left {
            std::fs::remove_file(dir.join(name)).unwrap();
        }
        std::fs::remove_dir(&dir).unwrap();
    }

    #[test]
    fn out_of_range_records_are_refused() {
        let good = encode(sample());
        assert_eq!(decode(&good), Some(sample()));
        let refused = |at: usize, v: u8| {
            let mut b = good;
            b[at] = v;
            decode(&b).is_none()
        };
        assert!(refused(KIND, 3)); // kind
        assert!(refused(KIND, KIND_CPU)); // a CPU key with a program and lighting paths
        assert!(refused(KIND, KIND_SHADOW)); // a shadow key with blend and depth fields
        assert!(refused(BLEND_FLAG, 2)); // blend flag
        assert!(refused(BLEND_FLAG, 0)); // blend off with its factors left
        assert!(refused(BLEND_SRC, 13)); // dual-source factor
        assert!(refused(BLEND_OP, 5)); // blend operation
        assert!(refused(DEPTH, 0) && refused(DEPTH, 9)); // depth compare
        assert!(refused(DEPTH_WRITE, 2)); // depth write
        assert!(refused(WRITE_MASK, 16)); // write mask
        assert!(refused(PROGRAM, 255)); // program
        assert!(refused(LIT.start, 64)); // lighting paths
        assert!(refused(PAD, 1)); // padding
    }

    #[test]
    fn round_trip_stale_corrupt_and_cap() {
        let path = std::env::temp_dir().join(format!(
            "ssx3-pipeline-keys-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        // Dropping a store's writer finishes its queued writes.
        let open = |renderer| {
            KeyStore::at(
                path.clone(),
                renderer,
                Arc::new(Writer::start().unwrap()),
                false,
            )
        };
        let mut store = open(9);
        assert!(store.keys().is_empty()); // No file yet: nothing to prewarm.
        store.record(sample());
        // A second key within a second waits for the next flush; a run that
        // ends first loses it (the engine never drops its renderer).
        store.record(Prewarm::Shadow(Program::Mat6Skin));
        drop(store);
        assert_eq!(open(9).keys(), vec![sample()]);
        assert!(open(10).keys().is_empty()); // Stale: rewritten empty.
        assert_eq!(parse(&std::fs::read(&path).unwrap(), 10), Some(vec![]));
        std::fs::write(&path, b"bad file").unwrap();
        assert!(open(10).keys().is_empty());
        assert_eq!(parse(&std::fs::read(&path).unwrap(), 10), Some(vec![]));
        let oversized = vec![0; HEADER + MAX_KEYS * RECORD + 1];
        assert!(parse(&oversized, 10).is_none());
        std::fs::write(&path, &oversized).unwrap();
        assert_eq!(read_capped(&path, oversized.len() - 1), Some(vec![]));
        assert!(open(10).keys().is_empty());
        assert_eq!(parse(&std::fs::read(&path).unwrap(), 10), Some(vec![]));
        std::fs::remove_file(path).unwrap();
    }
}
