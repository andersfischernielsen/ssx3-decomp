//! VU program invocation recorder, for differential testing against an
//! independent VU interpreter (`tools/vu-diff/`, see
//! `docs/investigation/vu-differential.md`).
//!
//! `SSX3_VU_CAPTURE=<path>:<max>[:<from_frame>[:<stride>[:<to_frame>]]]` appends one
//! [`Record`] per captured MSCAL/MSCNT (VU1, via the VIF) or VCALLMS (VU0) to
//! `<path>`: the architectural state at program start, code and data memory,
//! the code segment layout, then everything the program produced (XGKICK
//! packets, final state, final data memory). Invocations are captured from
//! frame `from_frame` (default 0, set by the frontend via [`set_frame`]) up to
//! `to_frame`: the first 3 of every distinct (unit, program hash, start pc),
//! plus every `stride`-th other one (default 50), up to `max` records.
//!
//! The file format is a sequence of records, little-endian throughout; see
//! [`Record::write`].

use super::Vu;
use crate::Qword;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

const MAGIC: u32 = 0x5243_5556; // "VUCR"
const VERSION: u32 = 1;

static FRAME: AtomicU64 = AtomicU64::new(0);

/// Current frame number (for the capture window); set by the frontend.
pub fn set_frame(f: u64) {
    FRAME.store(f, Ordering::Relaxed);
}

/// The frame number last given to `set_frame`.
pub fn frame() -> u64 {
    FRAME.load(Ordering::Relaxed)
}

/// Architectural VU state (what a program can observe or produce).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ArchState {
    pub vf: [[u32; 4]; 32],
    pub acc: [u32; 4],
    pub vi: [u16; 16],
    pub q: u32,
    pub p: u32,
    pub i: u32,
    pub r: u32,
    pub mac: u32,
    pub status: u32,
    pub clip: u32,
    pub tpc: u32,
}

impl ArchState {
    pub fn of(vu: &Vu) -> Self {
        Self {
            vf: vu.vf,
            acc: vu.acc,
            vi: vu.vi,
            q: vu.q,
            p: vu.p,
            i: vu.i,
            r: vu.r,
            mac: vu.mac,
            status: vu.status,
            clip: vu.clip,
            tpc: vu.tpc,
        }
    }

    /// Load into `vu` (all pipelines drained, visible flags = architectural).
    pub fn apply(&self, vu: &mut Vu) {
        vu.vf = self.vf;
        vu.acc = self.acc;
        vu.vi = self.vi;
        vu.q = self.q;
        vu.p = self.p;
        vu.i = self.i;
        vu.r = self.r;
        vu.mac = self.mac;
        vu.status = self.status;
        vu.clip = self.clip;
        vu.tpc = self.tpc;
        vu.flush();
    }

    fn write(&self, w: &mut Vec<u8>) {
        for v in self.vf.iter().chain(std::iter::once(&self.acc)) {
            for x in v {
                put32(w, *x);
            }
        }
        for x in self.vi {
            w.extend_from_slice(&x.to_le_bytes());
        }
        for x in [self.q, self.p, self.i, self.r, self.mac, self.status, self.clip, self.tpc] {
            put32(w, x);
        }
    }

    fn read(r: &mut Reader) -> Option<Self> {
        let mut s = Self::default();
        for v in s.vf.iter_mut() {
            for x in v.iter_mut() {
                *x = r.u32()?;
            }
        }
        for x in s.acc.iter_mut() {
            *x = r.u32()?;
        }
        for x in s.vi.iter_mut() {
            *x = r.u16()?;
        }
        s.q = r.u32()?;
        s.p = r.u32()?;
        s.i = r.u32()?;
        s.r = r.u32()?;
        s.mac = r.u32()?;
        s.status = r.u32()?;
        s.clip = r.u32()?;
        s.tpc = r.u32()?;
        Some(s)
    }
}

/// One captured program invocation.
#[derive(Clone, Debug, Default)]
pub struct Record {
    pub unit: u32,
    pub pc: u32,
    pub top: u32,
    pub itop: u32,
    pub frame: u64,
    pub seq: u64,
    pub hash: u64,
    /// Code upload segments `(start, end)` at program start (registry key).
    pub segments: Vec<(u32, u32)>,
    pub start: ArchState,
    pub code: Vec<u8>,
    pub data_in: Vec<u8>,
    /// Outputs.
    pub kicks: Vec<Vec<Qword>>,
    pub end: ArchState,
    pub data_out: Vec<u8>,
    pub pairs: u64,
    pub cycles: u64,
    pub errors: u64,
}

fn put32(w: &mut Vec<u8>, v: u32) {
    w.extend_from_slice(&v.to_le_bytes());
}
fn put64(w: &mut Vec<u8>, v: u64) {
    w.extend_from_slice(&v.to_le_bytes());
}
fn put_bytes(w: &mut Vec<u8>, b: &[u8]) {
    put32(w, b.len() as u32);
    w.extend_from_slice(b);
}

/// Little-endian cursor over a byte slice.
pub struct Reader<'a> {
    pub buf: &'a [u8],
    pub pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }
    pub fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let s = self.buf.get(self.pos..self.pos + n)?;
        self.pos += n;
        Some(s)
    }
    pub fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().ok()?))
    }
    pub fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    pub fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.take(8)?.try_into().ok()?))
    }
    pub fn bytes(&mut self) -> Option<Vec<u8>> {
        let n = self.u32()? as usize;
        Some(self.take(n)?.to_vec())
    }
    pub fn qwords(&mut self) -> Option<Vec<Qword>> {
        let n = self.u32()? as usize;
        let b = self.take(n * 16)?;
        Some(b.chunks_exact(16).map(|c| u128::from_le_bytes(c.try_into().unwrap())).collect())
    }
}

/// Serialize kick packets: u32 count, then per packet u32 qwords + data.
pub fn write_kicks(w: &mut Vec<u8>, kicks: &[Vec<Qword>]) {
    put32(w, kicks.len() as u32);
    for k in kicks {
        put32(w, k.len() as u32);
        for q in k {
            w.extend_from_slice(&q.to_le_bytes());
        }
    }
}

pub fn read_kicks(r: &mut Reader) -> Option<Vec<Vec<Qword>>> {
    let n = r.u32()? as usize;
    (0..n).map(|_| r.qwords()).collect()
}

impl Record {
    /// Layout: magic, version, unit, pc, top, itop, frame u64, seq u64,
    /// hash u64, nseg u32 + (start,end) u32 pairs, start state, code (u32
    /// len + bytes), data_in (len + bytes), kicks, end state, data_out,
    /// pairs u64, cycles u64, errors u64.
    ///
    /// ArchState = vf[32][4] u32, acc[4] u32, vi[16] u16, q p i r mac
    /// status clip tpc (u32 each).
    pub fn write(&self, w: &mut Vec<u8>) {
        put32(w, MAGIC);
        put32(w, VERSION);
        for x in [self.unit, self.pc, self.top, self.itop] {
            put32(w, x);
        }
        put64(w, self.frame);
        put64(w, self.seq);
        put64(w, self.hash);
        put32(w, self.segments.len() as u32);
        for &(a, b) in &self.segments {
            put32(w, a);
            put32(w, b);
        }
        self.start.write(w);
        put_bytes(w, &self.code);
        put_bytes(w, &self.data_in);
        write_kicks(w, &self.kicks);
        self.end.write(w);
        put_bytes(w, &self.data_out);
        put64(w, self.pairs);
        put64(w, self.cycles);
        put64(w, self.errors);
    }

    pub fn read(r: &mut Reader) -> Option<Self> {
        if r.u32()? != MAGIC || r.u32()? != VERSION {
            return None;
        }
        let mut rec = Record {
            unit: r.u32()?,
            pc: r.u32()?,
            top: r.u32()?,
            itop: r.u32()?,
            frame: r.u64()?,
            seq: r.u64()?,
            hash: r.u64()?,
            ..Default::default()
        };
        let n = r.u32()?;
        for _ in 0..n {
            rec.segments.push((r.u32()?, r.u32()?));
        }
        rec.start = ArchState::read(r)?;
        rec.code = r.bytes()?;
        rec.data_in = r.bytes()?;
        rec.kicks = read_kicks(r)?;
        rec.end = ArchState::read(r)?;
        rec.data_out = r.bytes()?;
        rec.pairs = r.u64()?;
        rec.cycles = r.u64()?;
        rec.errors = r.u64()?;
        Some(rec)
    }

    /// All records in a capture file.
    pub fn read_all(bytes: &[u8]) -> Vec<Record> {
        let mut r = Reader::new(bytes);
        let mut out = Vec::new();
        while r.pos < bytes.len() {
            match Record::read(&mut r) {
                Some(rec) => out.push(rec),
                None => break,
            }
        }
        out
    }

    /// A fresh VU set up exactly as at the captured program start (state,
    /// memories, code segment layout, VIF TOP/ITOP), started at `pc`.
    pub fn setup(&self) -> Vu {
        let mut vu = Vu::new(self.unit as usize);
        vu.code.copy_from_slice(&self.code);
        vu.data.copy_from_slice(&self.data_in);
        vu.set_segments(&self.segments);
        self.start.apply(&mut vu);
        vu.vif_top = self.top;
        vu.vif_itop = self.itop;
        vu.start(self.pc);
        vu
    }
}

struct Recorder {
    file: std::fs::File,
    max: u64,
    from: u64,
    to: u64,
    stride: u64,
    written: u64,
    seen: u64,
    per_entry: HashMap<(u32, u64, u32), u32>,
}

static REC: Mutex<Option<Recorder>> = Mutex::new(None);
static INIT: std::sync::Once = std::sync::Once::new();

fn init() {
    INIT.call_once(|| {
        let Ok(spec) = std::env::var("SSX3_VU_CAPTURE") else {
            return;
        };
        let mut p = spec.split(':');
        let path = p.next().unwrap_or_default().to_string();
        let num = |s: Option<&str>, d: u64| s.and_then(|v| v.parse().ok()).unwrap_or(d);
        let max = num(p.next(), 200);
        let from = num(p.next(), 0);
        let stride = num(p.next(), 50).max(1);
        let to = num(p.next(), u64::MAX);
        match std::fs::File::create(&path) {
            Ok(file) => {
                eprintln!("[vu-capture] -> {path}: max {max}, frames {from}..{to}, stride {stride}");
                *REC.lock().unwrap() = Some(Recorder {
                    file,
                    max,
                    from,
                    to,
                    stride,
                    written: 0,
                    seen: 0,
                    per_entry: HashMap::new(),
                });
            }
            Err(e) => eprintln!("[vu-capture] {path}: {e}"),
        }
    });
}

/// An in-flight capture (between program start and end).
pub struct Pending {
    rec: Record,
    pub kicks: Vec<Vec<Qword>>,
    pairs0: u64,
    cycles0: u64,
    errors0: u64,
}

/// Call right after `vu.start(pc)`: Some(..) if this invocation is captured.
pub fn begin(vu: &mut Vu, pc: u32) -> Option<Pending> {
    init();
    let mut g = REC.lock().unwrap();
    let r = g.as_mut()?;
    let frame = FRAME.load(Ordering::Relaxed);
    if r.written >= r.max || frame < r.from || frame >= r.to {
        return None;
    }
    let hash = vu.segment_hash_pub(pc);
    let n = r.per_entry.entry((vu.index as u32, hash, pc)).or_insert(0);
    *n += 1;
    r.seen += 1;
    if *n > 3 && r.seen % r.stride != 0 {
        return None;
    }
    r.written += 1;
    let seq = r.seen;
    drop(g);
    Some(Pending {
        rec: Record {
            unit: vu.index as u32,
            pc,
            top: vu.vif_top,
            itop: vu.vif_itop,
            frame,
            seq,
            hash,
            segments: vu.segments.iter().map(|s| (s.start, s.end)).collect(),
            start: ArchState::of(vu),
            code: vu.code.clone(),
            data_in: vu.data.clone(),
            ..Default::default()
        },
        kicks: Vec::new(),
        pairs0: vu.pairs,
        cycles0: vu.cycle,
        errors0: vu.errors,
    })
}

/// Call after the program ran to its end.
pub fn end(p: Pending, vu: &Vu) {
    let mut rec = p.rec;
    rec.kicks = p.kicks;
    rec.end = ArchState::of(vu);
    rec.data_out = vu.data.clone();
    rec.pairs = vu.pairs - p.pairs0;
    rec.cycles = vu.cycle - p.cycles0;
    rec.errors = vu.errors - p.errors0;
    let mut buf = Vec::with_capacity(70_000);
    rec.write(&mut buf);
    if let Some(r) = REC.lock().unwrap().as_mut() {
        let _ = r.file.write_all(&buf);
        let _ = r.file.flush();
    }
}

/// Read a whole capture file.
pub fn load(path: &str) -> std::io::Result<Vec<Record>> {
    let mut b = Vec::new();
    std::fs::File::open(path)?.read_to_end(&mut b)?;
    Ok(Record::read_all(&b))
}
