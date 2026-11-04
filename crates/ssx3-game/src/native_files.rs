//! Phase 3, subsystem 1: game files from the host instead of the emulated CD.
//!
//! The seam is EA's file platform layer (`docs/investigation/phase3-boundaries.md`
//! §1, `docs/investigation/phase3-files.md`). Everything above it stays the
//! generated game code: the request queue, BIG archives, async and streamed
//! files, callbacks. Below it, the emulated CD thread, `sceCdRead`, the cdvd
//! SIF RPC and the disc model are no longer used for data:
//!
//! - `EAFile_PlatformOpen` resolves a `cd:` path natively and opens it on the
//!   host, as device 3 ([`DEV_NATIVE`]). Other devices (host0:) keep the
//!   generated body.
//! - `EAFile_PlatformRead` on a device-3 handle copies the bytes straight into
//!   EE RAM and completes the request (`EAFile_CompleteRequest`), as the CD
//!   path does for a sector it has cached.
//! - `EAFile_PlatformGetSize` / `EAFile_PlatformClose` answer for device 3.
//!
//! What still touches the emulated CD: `EAFile_CdInit` at boot (sceCdInit,
//! disc type, TOC) and its directory walk for the CD name table, 20 sectors.
//! The walk cannot simply be skipped: `EAFile_CdInit` returns the number of
//! entries and startup treats 0 as "no file system".
//!
//! Files come from the disc image (`SSX3_ISO`, by ISO9660 name) or, with
//! `SSX3_DATA_DIR`, from an extracted copy of the disc (any case), which makes
//! files replaceable on the host.
//!
//! On by default; `SSX3_NATIVE_FILES=0` keeps the emulated CD. `SSX3_FILE_LOG=path`
//! (either mode) logs every completed read as `name offset size fnv64`, so a
//! native run can be compared with an emulated one (`tools/compare-file-logs.py`).
//! These are not byte-identical replacements: loading timing changes.

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use ssx3_core::iso::Disc;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;

use crate::abi;
use crate::call::{guest, Callee, Frame};

/// Device number of a native handle (`handle >> 24`; the game uses 1 = cd,
/// 2 = host0:).
pub const DEV_NATIVE: u32 = 3;

const PLATFORM_OPEN: u32 = 0x003e_3208;
const PLATFORM_READ: u32 = 0x003e_3350;
const PLATFORM_CLOSE: u32 = 0x003e_32f8;
const PLATFORM_GET_SIZE: u32 = 0x003e_34e8;
const COMPLETE_REQUEST: u32 = 0x003d_e420;
const HOST_READ_SERVICE: u32 = 0x003e_33b0;

/// `EAFile_CompleteRequest(ok)`, through its slot (so the read log sees it).
const CALL_COMPLETE_REQUEST: Callee = Callee {
    addr: COMPLETE_REQUEST,
    f: ssx3_ee::s14::f_003de420,
};
/// Where `EAFile_PlatformRead` returns from its `EAFile_CdRead` call. The
/// native read uses the same frame (0x10, ra at 0) and return address, so a
/// guest thread switched out inside the completion callback resumes in the
/// generated body correctly.
const PLATFORM_READ_RET: u32 = 0x003e_33a0;

// EAFile globals (docs/investigation/phase3-boundaries.md §1.3).
const DEFAULT_DEVICE: u32 = 0x0045_0c44;
const CD_PREFIX_PTR: u32 = 0x0045_0c3c;
const HOST_PREFIX_PTR: u32 = 0x0045_0c50;
const ROOT_PATH: u32 = 0x0051_9af0;
const ACTIVE_REQUEST: u32 = 0x0051_9ad4;

enum Source {
    Iso(Disc),
    Dir(PathBuf),
}

struct OpenFile {
    name: String,
    size: u32,
    loc: Loc,
}

enum Loc {
    /// First sector on the disc image.
    Sector(u32),
    Host(File),
}

struct State {
    src: Source,
    files: Vec<Option<OpenFile>>,
    /// Name lookups (upper-case path -> disc entry), cached.
    found: HashMap<String, Option<(u32, u32)>>,
    /// A read issued while a native read is being completed (from the
    /// completion callback): run by the loop in [`platform_read`] instead of
    /// recursing.
    pending: Option<(u32, u32, u32, u32)>,
    /// A pending read whose completion loop was cut short by a guest thread
    /// switch; finished by [`host_read_service`].
    stranded: Option<(u32, u32, u32, u32)>,
    busy: bool,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);
static LOG: Mutex<Option<File>> = Mutex::new(None);

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Install the native file layer if `SSX3_NATIVE_FILES` is set, and the read
/// log if `SSX3_FILE_LOG` is. Call after [`crate::install`]. Returns whether
/// native files are on.
pub fn install() -> bool {
    if let Ok(p) = std::env::var("SSX3_FILE_LOG") {
        match File::create(&p) {
            Ok(f) => {
                *LOG.lock().unwrap() = Some(f);
                assert!(ssx3_ee::set_override(
                    COMPLETE_REQUEST,
                    complete_request_logged
                ));
            }
            Err(e) => eprintln!("SSX3_FILE_LOG {p}: {e}"),
        }
    }
    // On by default; SSX3_NATIVE_FILES=0 keeps the emulated CD.
    if std::env::var("SSX3_NATIVE_FILES").is_ok_and(|v| v == "0") {
        return false;
    }
    let src = match std::env::var_os("SSX3_DATA_DIR") {
        Some(d) => {
            let d = PathBuf::from(d);
            if !d.is_dir() {
                eprintln!(
                    "native files: SSX3_DATA_DIR {} is not a directory; keeping the emulated CD",
                    d.display()
                );
                return false;
            }
            Source::Dir(d)
        }
        None => {
            let iso = ssx3_core::platform_paths::data_file(
                "SSX3_ISO",
                "SSX 3 (USA).iso",
                repo_root().join("SSX 3 (USA).iso"),
            );
            match Disc::open(&iso) {
                Ok(d) => Source::Iso(d),
                Err(e) => {
                    eprintln!(
                        "native files: cannot open {}: {e}; keeping the emulated CD",
                        iso.display()
                    );
                    return false;
                }
            }
        }
    };
    *STATE.lock().unwrap() = Some(State {
        src,
        files: Vec::new(),
        found: HashMap::new(),
        pending: None,
        stranded: None,
        busy: false,
    });
    for (addr, f) in [
        (PLATFORM_OPEN, platform_open as fn(&mut Cpu)),
        (PLATFORM_READ, platform_read),
        (PLATFORM_CLOSE, platform_close),
        (PLATFORM_GET_SIZE, platform_get_size),
        (HOST_READ_SERVICE, host_read_service),
    ] {
        assert!(
            ssx3_ee::set_override(addr, f),
            "{addr:#x}: no override slot (overrides.txt)"
        );
    }
    true
}

/// `EAFile_ResolvePath` (0x3e3098), natively: the device a path names and
/// the full path (root prepended, `\` turned into `/`). `None` where the
/// game's version leaves the output empty.
fn resolve_path(c: &mut Cpu, path: &str) -> Option<(u32, String)> {
    let mut dev = c.bus.read32(DEFAULT_DEVICE);
    let (cd_ptr, host_ptr) = (c.bus.read32(CD_PREFIX_PTR), c.bus.read32(HOST_PREFIX_PTR));
    let cd = c.bus.read_cstr(cd_ptr);
    let host = c.bus.read_cstr(host_ptr);
    let mut p = path;
    if p.contains(':') {
        let cd3 = &cd.as_bytes()[..cd.len().min(3)];
        if p.as_bytes().starts_with(cd3) {
            dev = 1;
            p = &p[3.min(p.len())..];
        } else if p.starts_with(&host) {
            dev = 2;
            p = &p[host.len()..];
        } else if dev == 1 || p.as_bytes().get(2) != Some(&b':') {
            return None;
        }
    }
    let mut out = String::new();
    if dev == 2 {
        out.push_str(&host);
    }
    if !p.starts_with(['\\', '/', '~']) {
        out.push_str(&c.bus.read_cstr(ROOT_PATH));
    }
    out.push_str(p);
    Some((dev, out.replace('\\', "/")))
}

/// `EAFile_CdLookup`'s name rule: upper case, no leading `/`.
fn cd_name(full: &str) -> String {
    full.trim_start_matches('/').to_ascii_uppercase()
}

impl State {
    fn open(&mut self, name: &str) -> Option<OpenFile> {
        match &mut self.src {
            Source::Iso(disc) => {
                let hit = *self.found.entry(name.to_string()).or_insert_with(|| {
                    disc.find(name)
                        .filter(|e| !e.dir)
                        .map(|e| (e.sector, e.size))
                });
                let (sector, size) = hit?;
                Some(OpenFile {
                    name: name.to_string(),
                    size,
                    loc: Loc::Sector(sector),
                })
            }
            Source::Dir(root) => {
                let path = find_ci(root, name)?;
                let f = File::open(&path).ok()?;
                let size = f.metadata().ok()?.len() as u32;
                Some(OpenFile {
                    name: name.to_string(),
                    size,
                    loc: Loc::Host(f),
                })
            }
        }
    }

    fn read(&mut self, idx: usize, off: u32, out: &mut [u8]) -> bool {
        let Some(Some(f)) = self.files.get_mut(idx) else {
            return false;
        };
        match (&mut f.loc, &mut self.src) {
            (Loc::Sector(s), Source::Iso(disc)) => {
                // Whole sectors around [off, off + len).
                let first = off / 2048;
                let skip = (off % 2048) as usize;
                let count = (skip + out.len()).div_ceil(2048) as u32;
                let buf = disc.read_vec(*s + first, count);
                out.copy_from_slice(&buf[skip..skip + out.len()]);
                true
            }
            (Loc::Host(file), _) => {
                file.seek(SeekFrom::Start(off as u64)).is_ok() && file.read_exact(out).is_ok()
            }
            _ => false,
        }
    }
}

/// Case-insensitive path lookup under `root` (`DATA/FOO.BIG` matches
/// `data/foo.big`).
fn find_ci(root: &Path, name: &str) -> Option<PathBuf> {
    let mut cur = root.to_path_buf();
    for part in name.split('/').filter(|s| !s.is_empty()) {
        let part = part.split(';').next().unwrap();
        let direct = cur.join(part);
        cur = if direct.exists() {
            direct
        } else {
            std::fs::read_dir(&cur)
                .ok()?
                .filter_map(|e| e.ok())
                .find(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case(part))?
                .path()
        };
    }
    cur.is_file().then_some(cur)
}

/// `bool EAFile_PlatformOpen(const char *path, int mode, u32 *handle)`.
pub fn platform_open(c: &mut Cpu) {
    let path = c.bus.read_cstr(abi::arg(c, 0));
    let out = abi::arg(c, 2);
    let resolved = resolve_path(c, &path);
    let Some((1, full)) = resolved else {
        // host0: (and unresolvable paths) as before.
        return ssx3_ee::s14::f_003e3208__generated(c);
    };
    let name = cd_name(&full);
    let mut g = STATE.lock().unwrap();
    let st = g.as_mut().unwrap();
    let ok = match st.open(&name) {
        Some(f) => {
            let idx = match st.files.iter().position(Option::is_none) {
                Some(i) => {
                    st.files[i] = Some(f);
                    i
                }
                None => {
                    st.files.push(Some(f));
                    st.files.len() - 1
                }
            };
            c.bus.write32(out, DEV_NATIVE << 24 | idx as u32);
            true
        }
        None => {
            c.bus.write32(out, 0);
            false
        }
    };
    abi::set_ret(c, ok as u32);
    abi::ret(c);
}

fn native_idx(handle: u32) -> Option<usize> {
    (handle >> 24 == DEV_NATIVE).then_some((handle & 0x00ff_ffff) as usize)
}

/// `bool EAFile_PlatformRead(u32 handle, void *buf, int offset, int size)`.
pub fn platform_read(c: &mut Cpu) {
    let handle = abi::arg(c, 0);
    if native_idx(handle).is_none() {
        // cd/host0: handles (none after boot) keep the phase 2 replacement.
        return crate::streaming::eafile_platform_read(c);
    }
    let req = (handle, abi::arg(c, 1), abi::arg(c, 2), abi::arg(c, 3));
    {
        let mut g = STATE.lock().unwrap();
        let st = g.as_mut().unwrap();
        if st.busy {
            // Issued from inside the completion below (callback ->
            // ServiceQueue): the loop runs it, so long chains of reads do
            // not nest.
            st.pending = Some(req);
            abi::set_ret(c, 1);
            abi::ret(c);
            return;
        }
        st.busy = true;
    }
    guest(c, |c| complete_loop(c, req));
}

/// Read and complete `first`, then every read issued while completing,
/// inside a frame shaped like `EAFile_PlatformRead`'s (0x10, ra at 0) and
/// returning to its call site: if a callback switches guest threads, the
/// generated body resumes and returns from it correctly.
fn complete_loop(c: &mut Cpu, first: (u32, u32, u32, u32)) -> crate::call::Flow {
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let mut next = Some(first);
    while let Some((handle, buf, off, size)) = next.take() {
        let ok = read_into(c, handle, buf, off, size);
        c.g[A0][0] = ok as u64;
        let r = CALL_COMPLETE_REQUEST.call(c, PLATFORM_READ_RET);
        let mut g = STATE.lock().unwrap();
        let st = g.as_mut().unwrap();
        if r.is_err() {
            // A read queued just before the switch is finished from the
            // next EAFile_HostReadService (timer task, every frame).
            st.busy = false;
            st.stranded = st.pending.take();
            return r;
        }
        next = st.pending.take();
    }
    STATE.lock().unwrap().as_mut().unwrap().busy = false;
    abi::set_ret(c, 1);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `EAFile_HostReadService` (timer task): first finishes a native read left
/// over by a thread switch, then runs the generated body (host0: reads).
pub fn host_read_service(c: &mut Cpu) {
    let stranded = {
        let mut g = STATE.lock().unwrap();
        let st = g.as_mut().unwrap();
        let s = if st.busy { None } else { st.stranded.take() };
        if s.is_some() {
            st.busy = true;
        }
        s
    };
    if let Some(req) = stranded {
        if complete_loop(c, req).is_err() {
            return;
        }
    }
    ssx3_ee::s14::f_003e33b0__generated(c)
}

/// Copy `size` bytes at `off` of a native file into guest memory.
fn read_into(c: &mut Cpu, handle: u32, buf: u32, off: u32, size: u32) -> bool {
    let Some(idx) = native_idx(handle) else {
        return false;
    };
    let mut data = vec![0u8; size as usize];
    let ok = STATE
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .read(idx, off, &mut data);
    if ok {
        c.bus.write_bytes(buf, &data);
    }
    ok
}

/// `void EAFile_PlatformClose(u32 handle)`.
pub fn platform_close(c: &mut Cpu) {
    let Some(idx) = native_idx(abi::arg(c, 0)) else {
        return ssx3_ee::s14::f_003e32f8__generated(c);
    };
    if let Some(slot) = STATE.lock().unwrap().as_mut().unwrap().files.get_mut(idx) {
        *slot = None;
    }
    abi::set_ret(c, 0);
    abi::ret(c);
}

/// `int EAFile_PlatformGetSize(u32 handle)`.
pub fn platform_get_size(c: &mut Cpu) {
    let Some(idx) = native_idx(abi::arg(c, 0)) else {
        return ssx3_ee::s14::f_003e34e8__generated(c);
    };
    let size = STATE
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .files
        .get(idx)
        .and_then(|f| f.as_ref())
        .map_or(0, |f| f.size);
    abi::set_ret(c, size);
    abi::ret(c);
}

/// `EAFile_CompleteRequest(ok)` with the read log: logs completed reads, then
/// runs the generated body.
pub fn complete_request_logged(c: &mut Cpu) {
    let req = c.bus.read32(ACTIVE_REQUEST);
    if req != 0 && abi::arg(c, 0) != 0 && c.bus.read32(req + 4) == 0 {
        let op = c.bus.read32(req) >> 20 & 0xf;
        if op == 4 {
            let fh = c.bus.read32(req + 0x24);
            let off = c.bus.read32(req + 0x18);
            let size = c.bus.read32(req + 0x1c);
            let buf = c.bus.read32(req + 0x20);
            let name = c.bus.read_cstr(fh + 0x10);
            let data = c.bus.read_bytes(buf, size as usize);
            let mut h: u64 = 0xcbf2_9ce4_8422_2325;
            for b in data {
                h = (h ^ b as u64).wrapping_mul(0x100_0000_01b3);
            }
            if let Some(f) = LOG.lock().unwrap().as_mut() {
                let _ = writeln!(f, "{name}\t{off:#x}\t{size:#x}\t{h:016x}");
            }
        }
    }
    ssx3_ee::s14::f_003de420__generated(c)
}
