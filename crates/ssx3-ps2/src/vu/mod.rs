//! Vector Units (VU0 and VU1): register file, the per-instruction semantic
//! library that statically recompiled microcode calls, and the COP2 macro-mode
//! operations the EE issues.
//!
//! INTERFACE CONTRACT:
//! - `Vu::new(index)`: index 0 (4 KiB code / 4 KiB data) or 1 (16 KiB / 16 KiB).
//! - fields `vf: [[u32; 4]; 32]` (raw float bits, x,y,z,w), `vi: [u16; 16]`,
//!   `code`, `data` (byte vectors), `tpc`, `running`.
//! - `macro_op(word)`: execute one EE COP2 "CO" instruction (bit 25 set),
//!   except VCALLMS/VCALLMSR which the EE handles by calling `start(pc)`.
//! - `read_ctrl(reg)` / `write_ctrl(reg, val)`: cfc2/ctc2 control registers
//!   (0..15 = VI, 16 status, 17 MAC, 18 clip, 20 R, 21 I, 22 Q, 26 TPC, ...).
//! - `start(pc)` then `run(max_cycles, kick)`: run micro code until the E bit
//!   ends it (returns true) or the budget expires (returns false, still running).
//!   `kick(&[Qword])` receives each XGKICK GIF packet (VU1 only).
//!
//! ## Execution model (static recompilation)
//!
//! Micro programs are *not* interpreted at runtime. `recomp::translate_program`
//! turns each microcode upload into Rust (`generated/vu/`), one function per
//! upload with a `match pc` over basic blocks. Each instruction pair becomes
//!
//! ```text
//! vu.begin_pair(&[(fs, fieldmask), ...]); // stall on not-yet-ready VF fields,
//!                                         // retire Q/P/flag pipelines
//! vu.loi(imm);                            // only for I-bit pairs
//! vu.madd(0b1110, 1, 2, Src::Bc(3, W));   // upper: computed from OLD state, staged
//! vu.lq(0b1111, 4, 1, 2);                 // lower: executes against OLD state
//! vu.end_pair();                          // commit upper (wins over a lower write
//!                                         // to the same VF), flags, cycle += 1
//! ```
//!
//! `run()` finds the translated function through [`Vu::programs`] keyed by the
//! FNV-1a hash of the uploaded code segment containing the start pc (see
//! [`Vu::write_code`]). With no translation, the VU logs an error and stops
//! (unless an explicit [`Vu::fallback`] executor is installed — the test
//! oracle in `reference` is one).

pub mod decode;
pub mod exec;
pub mod float;
pub mod recomp;
#[cfg(test)]
pub mod reference;
#[cfg(test)]
mod tests;

/// Statically recompiled microcode (written by `src/bin/vu_recomp.rs`).
#[path = "../../../../generated/vu/mod.rs"]
pub mod generated;

use crate::Qword;
use float::{Exc, ABS, MAX, ONE, SIGN};
use std::collections::{HashMap, VecDeque};

/// Field indices for broadcast operands.
pub const X: u8 = 0;
pub const Y: u8 = 1;
pub const Z: u8 = 2;
pub const W: u8 = 3;

/// Second operand of an FMAC op.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    /// ft, lane by lane.
    Vec(u8),
    /// ft.field broadcast (field 0..3 = x..w).
    Bc(u8, u8),
    /// The Q register.
    Q,
    /// The I register.
    I,
}

/// How a translated (or fallback) program returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VuExit {
    /// The E bit ended the program (`tpc` holds the resume address).
    End,
    /// Control reached a pc this function does not cover; re-dispatch.
    Jump(u32),
    /// The pair budget ran out at this block start; resume later.
    Budget(u32),
}

/// A translated program: `func(vu, start_pc, kick)`.
pub type ProgramFn = fn(&mut Vu, u32, &mut dyn FnMut(&[Qword])) -> VuExit;

/// One translated microcode upload.
pub struct ProgramEntry {
    /// `program_hash(start, bytes)` of the upload.
    pub hash: u64,
    /// Load address (bytes) and length of the upload.
    pub start: u32,
    pub len: u32,
    /// Sorted basic-block leader pcs (informational: every pair of the
    /// segment is accepted as a start pc).
    pub leaders: &'static [u32],
    pub func: ProgramFn,
}

/// FNV-1a over the load address (LE) followed by the code bytes.
pub fn program_hash(start: u32, bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in start.to_le_bytes().iter().chain(bytes.iter()) {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    h
}

/// A contiguous range of code memory written by one run of MPG uploads.
#[derive(Clone, Debug)]
pub struct CodeSegment {
    pub start: u32,
    pub end: u32,
    serial: u64,
    hash: Option<u64>,
}

#[derive(Clone, Copy, Debug)]
struct Pending {
    value: u32,
    ready: u64,
    invalid: bool,
    div0: bool,
}

#[derive(Clone, Copy, Debug)]
struct FlagSnap {
    ready: u64,
    mac: u32,
    status: u32,
    clip: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Target {
    None,
    Vf(u8),
    Acc,
}

#[derive(Clone, Copy, Debug)]
struct Stage {
    target: Target,
    dest: u8,
    value: [u32; 4],
    mac: Option<u32>,
    clip: Option<u32>,
}

const EMPTY_STAGE: Stage = Stage {
    target: Target::None,
    dest: 0,
    value: [0; 4],
    mac: None,
    clip: None,
};

/// FMAC result latency (cycles) for the stall model.
const FMAC_LATENCY: u64 = 4;

/// Clip flags become visible to FC* ops 3 pairs after CLIP (as in
/// ps2-web-recomp's `FlagPipe`; MAC/status use FMAC_LATENCY = 4).
/// Override with SSX3_VU_CLIP_LATENCY for experiments.
fn clip_latency() -> u64 {
    static L: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *L.get_or_init(|| {
        std::env::var("SSX3_VU_CLIP_LATENCY").ok().and_then(|v| v.parse().ok()).unwrap_or(3)
    })
}
const DIV_LATENCY: u64 = 7;
const SQRT_LATENCY: u64 = 7;
const RSQRT_LATENCY: u64 = 13;

/// Status bits fed by the FMAC pipeline (Z S U O and their sticky copies).
const STATUS_FMAC: u32 = 0x3cf;
/// Status bits fed by the DIV unit (I D and sticky IS DS).
const STATUS_DIV: u32 = 0xc30;

pub struct Vu {
    pub index: usize,
    pub vf: [[u32; 4]; 32],
    pub vi: [u16; 16],
    pub code: Vec<u8>,
    pub data: Vec<u8>,
    /// Byte address of the next pair to execute (cfc2 reg 26 reports tpc/8).
    pub tpc: u32,
    pub running: bool,

    pub acc: [u32; 4],
    pub q: u32,
    pub p: u32,
    pub i: u32,
    /// R register, stored as `0x3f800000 | 23-bit LFSR`.
    pub r: u32,
    /// Architectural flags (as of the last issued instruction).
    pub mac: u32,
    pub status: u32,
    pub clip: u32,
    /// Flags as seen by FMAND/FSAND/FCAND... (4-cycle FMAC latency).
    pub vis_mac: u32,
    pub vis_status: u32,
    pub vis_clip: u32,

    /// cfc2/ctc2 bookkeeping.
    pub cmsar0: u32,
    pub fbrst: u32,
    /// Set by the machine so VPU-STAT (reg 29) can report the other VU.
    pub peer_running: bool,
    /// A ctc2 to CMSAR1 (reg 31) on VU0: byte address the machine should
    /// `start()` VU1 at. Taken by the machine.
    pub cmsar1_request: Option<u32>,

    /// VIF TOP / ITOP as seen by XTOP / XITOP (set by the VIF before MSCAL).
    pub vif_top: u32,
    pub vif_itop: u32,

    /// Cycle counter of the timing model (1 per pair + stalls).
    pub cycle: u64,
    /// Total pairs executed.
    pub pairs: u64,
    /// Remaining pair budget during `run`.
    pub budget: u64,

    /// Code segments written via `write_code` (MPG).
    pub segments: Vec<CodeSegment>,
    seg_serial: u64,
    /// Translated programs by hash.
    pub programs: HashMap<u64, &'static ProgramEntry>,
    /// Explicit executor used when no translation exists (None at runtime).
    pub fallback: Option<ProgramFn>,
    /// Count of runtime errors (missing translation, unknown op ...).
    pub errors: u64,
    pub last_error: Option<String>,

    vf_ready: [[u64; 4]; 32],
    q_pend: Option<Pending>,
    p_pend: Option<Pending>,
    flag_pipe: VecDeque<FlagSnap>,
    /// Clip flags have their own (shorter) latency: (ready cycle, value).
    clip_pipe: VecDeque<(u64, u32)>,
    stage: Stage,
    vi_backup: Option<(u8, u16)>,
    vi_backup_next: Option<(u8, u16)>,
}

impl Vu {
    pub fn new(index: usize) -> Self {
        let size = if index == 0 { 4096 } else { 16384 };
        let mut vu = Self {
            index,
            vf: [[0; 4]; 32],
            vi: [0; 16],
            code: vec![0; size],
            data: vec![0; size],
            tpc: 0,
            running: false,
            acc: [0; 4],
            q: 0,
            p: 0,
            i: 0,
            r: ONE,
            mac: 0,
            status: 0,
            clip: 0,
            vis_mac: 0,
            vis_status: 0,
            vis_clip: 0,
            cmsar0: 0,
            fbrst: 0,
            peer_running: false,
            cmsar1_request: None,
            vif_top: 0,
            vif_itop: 0,
            cycle: 0,
            pairs: 0,
            budget: 0,
            segments: Vec::new(),
            seg_serial: 0,
            programs: HashMap::new(),
            fallback: None,
            errors: 0,
            last_error: None,
            vf_ready: [[0; 4]; 32],
            q_pend: None,
            p_pend: None,
            flag_pipe: VecDeque::with_capacity(8),
            clip_pipe: VecDeque::with_capacity(8),
            stage: EMPTY_STAGE,
            vi_backup: None,
            vi_backup_next: None,
        };
        vu.vf[0] = [0, 0, 0, ONE];
        vu.register_programs(generated::PROGRAMS);
        vu
    }

    /// Reset registers (FBRST.RS); memories and the program registry are kept.
    pub fn reset(&mut self) {
        self.vf = [[0; 4]; 32];
        self.vf[0] = [0, 0, 0, ONE];
        self.vi = [0; 16];
        self.acc = [0; 4];
        self.q = 0;
        self.p = 0;
        self.i = 0;
        self.r = ONE;
        self.mac = 0;
        self.status = 0;
        self.clip = 0;
        self.vis_mac = 0;
        self.vis_status = 0;
        self.vis_clip = 0;
        self.tpc = 0;
        self.running = false;
        self.q_pend = None;
        self.p_pend = None;
        self.flag_pipe.clear();
        self.clip_pipe.clear();
        self.stage = EMPTY_STAGE;
        self.vi_backup = None;
        self.vi_backup_next = None;
        self.vf_ready = [[0; 4]; 32];
    }

    pub fn register_programs(&mut self, table: &'static [ProgramEntry]) {
        for e in table {
            self.programs.insert(e.hash, e);
        }
    }

    fn error(&mut self, msg: String) {
        self.errors += 1;
        if self.errors <= 64 {
            eprintln!("[vu{}] {}", self.index, msg);
        }
        self.last_error = Some(msg);
    }

    #[inline]
    fn code_mask(&self) -> u32 {
        self.code.len() as u32 - 1
    }

    #[inline]
    fn data_qwords(&self) -> u32 {
        (self.data.len() / 16) as u32
    }

    // ------------------------------------------------------------------
    // Code memory and the program registry
    // ------------------------------------------------------------------

    /// Write microcode (MPG / direct writes). Tracks upload segments: a
    /// write starting where the most recent segment ends extends it, every
    /// other write starts a new segment; overlapped parts of older segments
    /// are trimmed.
    pub fn write_code(&mut self, addr: u32, bytes: &[u8]) {
        let size = self.code.len() as u32;
        let a = addr & (size - 1);
        if a as usize + bytes.len() > size as usize {
            let first = (size - a) as usize;
            self.write_code(a, &bytes[..first]);
            self.write_code(0, &bytes[first..]);
            return;
        }
        if bytes.is_empty() {
            return;
        }
        let b = a + bytes.len() as u32;
        self.code[a as usize..b as usize].copy_from_slice(bytes);
        let mut out = Vec::with_capacity(self.segments.len() + 2);
        for s in self.segments.drain(..) {
            if s.end <= a || s.start >= b {
                out.push(s);
                continue;
            }
            if s.start < a {
                out.push(CodeSegment {
                    start: s.start,
                    end: a,
                    serial: s.serial,
                    hash: None,
                });
            }
            if s.end > b {
                out.push(CodeSegment {
                    start: b,
                    end: s.end,
                    serial: s.serial,
                    hash: None,
                });
            }
        }
        self.segments = out;
        let latest = self.seg_serial;
        if let Some(s) = self
            .segments
            .iter_mut()
            .find(|s| s.serial == latest && s.end == a && latest != 0)
        {
            s.end = b;
            s.hash = None;
        } else {
            self.seg_serial += 1;
            self.segments.push(CodeSegment {
                start: a,
                end: b,
                serial: self.seg_serial,
                hash: None,
            });
        }
    }

    /// Hash of the code segment containing `pc` (cached).
    pub fn segment_hash(&mut self, pc: u32) -> Option<u64> {
        let idx = self
            .segments
            .iter()
            .position(|s| s.start <= pc && pc < s.end)?;
        let s = &self.segments[idx];
        if let Some(h) = s.hash {
            return Some(h);
        }
        let h = program_hash(s.start, &self.code[s.start as usize..s.end as usize]);
        self.segments[idx].hash = Some(h);
        Some(h)
    }

    /// The translated function covering `pc`, if any.
    /// Debug: hash of the upload containing `pc` (0 if none).
    pub fn segment_hash_pub(&mut self, pc: u32) -> u64 {
        self.segment_hash(pc).unwrap_or(0)
    }

    pub fn lookup(&mut self, pc: u32) -> Option<ProgramFn> {
        let h = self.segment_hash(pc)?;
        let e = self.programs.get(&h)?;
        (pc >= e.start && pc < e.start + e.len && pc & 7 == 0).then_some(e.func)
    }

    // ------------------------------------------------------------------
    // Micro execution entry points
    // ------------------------------------------------------------------

    pub fn start(&mut self, pc: u32) {
        self.tpc = pc & self.code_mask() & !7;
        self.running = true;
        self.stage = EMPTY_STAGE;
        self.vi_backup = None;
        self.vi_backup_next = None;
    }

    /// Run the started program for at most `max_cycles` instruction pairs.
    pub fn run(&mut self, max_cycles: u64, kick: &mut dyn FnMut(&[Qword])) -> bool {
        if !self.running {
            return true;
        }
        self.budget = max_cycles;
        let mut hops = 0u32;
        loop {
            let pc = self.tpc & self.code_mask();
            let func = match self.lookup(pc) {
                Some(f) => f,
                None => match self.fallback {
                    Some(f) => f,
                    None => {
                        let h = self.segment_hash(pc);
                        self.error(format!(
                            "no translated microcode for pc {pc:#06x} (segment hash {h:?}); stopping VU"
                        ));
                        self.flush();
                        self.running = false;
                        return true;
                    }
                },
            };
            match func(self, pc, kick) {
                VuExit::End => {
                    self.running = false;
                    return true;
                }
                VuExit::Budget(t) => {
                    self.tpc = t;
                    return false;
                }
                VuExit::Jump(t) => {
                    self.tpc = t;
                    hops += 1;
                    if hops > 1_000_000 {
                        self.error(format!("dispatch loop at {t:#06x}"));
                        self.running = false;
                        return true;
                    }
                }
            }
        }
    }

    /// Called by translated code at a block start; consumes `n` pairs of
    /// budget, returning true (and consuming nothing) if it is exhausted.
    #[inline]
    pub fn out_of_budget(&mut self, n: u64) -> bool {
        if self.budget < n {
            return true;
        }
        self.budget -= n;
        false
    }

    /// The E bit's delay slot finished: record the resume address and drain
    /// the pipelines.
    pub fn end_program(&mut self, next_pc: u32) {
        self.tpc = next_pc & self.code_mask();
        self.flush();
        self.running = false;
    }

    /// Unknown micro instruction (translated as a call to this).
    pub fn unknown_op(&mut self, pc: u32, word: u32) {
        self.error(format!("unknown VU instruction {word:#010x} at {pc:#06x}"));
    }

    // ------------------------------------------------------------------
    // Pair framing and pipelines
    // ------------------------------------------------------------------

    /// Start an instruction pair: stall until the listed VF fields are
    /// written back, then retire Q/P/flag results that are due.
    #[inline]
    pub fn begin_pair(&mut self, reads: &[(u8, u8)]) {
        let mut need = self.cycle;
        for &(r, m) in reads {
            let ready = &self.vf_ready[r as usize & 31];
            for (f, &rdy) in ready.iter().enumerate() {
                if m & (8 >> f) != 0 && rdy > need {
                    need = rdy;
                }
            }
        }
        self.cycle = need;
        self.advance();
        self.stage = EMPTY_STAGE;
    }

    /// LOI: the I bit's immediate, visible to the upper op of the same pair.
    #[inline]
    pub fn loi(&mut self, imm: u32) {
        self.i = imm;
    }

    /// Finish a pair: commit the staged upper result and flags.
    #[inline]
    pub fn end_pair(&mut self) {
        self.commit_stage(true);
        self.vi_backup = self.vi_backup_next.take();
        self.cycle += 1;
        self.pairs += 1;
    }

    fn commit_stage(&mut self, pipelined: bool) {
        let st = self.stage;
        self.stage = EMPTY_STAGE;
        match st.target {
            Target::Vf(r) if r != 0 => {
                for l in 0..4 {
                    if st.dest & (8 >> l) != 0 {
                        self.vf[r as usize][l] = st.value[l];
                        self.vf_ready[r as usize][l] = self.cycle + FMAC_LATENCY;
                    }
                }
            }
            Target::Acc => {
                for l in 0..4 {
                    if st.dest & (8 >> l) != 0 {
                        self.acc[l] = st.value[l];
                    }
                }
            }
            _ => {}
        }
        let mut changed = false;
        if let Some(mac) = st.mac {
            self.mac = mac;
            let mut s = 0;
            if mac & 0x000f != 0 {
                s |= 1;
            }
            if mac & 0x00f0 != 0 {
                s |= 2;
            }
            if mac & 0x0f00 != 0 {
                s |= 4;
            }
            if mac & 0xf000 != 0 {
                s |= 8;
            }
            self.status = (self.status & !0xf) | s | (s << 6);
            changed = true;
        }
        if let Some(c) = st.clip {
            self.clip = ((self.clip << 6) | c) & 0xff_ffff;
            changed = true;
        }
        if let (Some(_), true) = (st.clip, pipelined) {
            let c = self.clip;
            self.clip_pipe.push_back((self.cycle + clip_latency(), c));
        }
        if changed {
            if pipelined {
                self.flag_pipe.push_back(FlagSnap {
                    ready: self.cycle + FMAC_LATENCY,
                    mac: self.mac,
                    status: self.status,
                    clip: self.clip,
                });
            } else {
                self.vis_mac = self.mac;
                self.vis_status = (self.vis_status & STATUS_DIV) | (self.status & STATUS_FMAC);
                self.vis_clip = self.clip;
            }
        }
    }

    fn apply_q(&mut self, p: Pending) {
        self.q = p.value;
        let bits = (p.invalid as u32) << 4 | (p.div0 as u32) << 5;
        for s in [&mut self.status, &mut self.vis_status] {
            *s = (*s & !0x30) | bits | (bits << 6);
        }
    }

    fn advance(&mut self) {
        if let Some(p) = self.q_pend {
            if p.ready <= self.cycle {
                self.q_pend = None;
                self.apply_q(p);
            }
        }
        if let Some(p) = self.p_pend {
            if p.ready <= self.cycle {
                self.p_pend = None;
                self.p = p.value;
            }
        }
        while let Some(f) = self.flag_pipe.front() {
            if f.ready > self.cycle {
                break;
            }
            let f = *f;
            self.flag_pipe.pop_front();
            self.vis_mac = f.mac;
            self.vis_status = (self.vis_status & STATUS_DIV) | (f.status & STATUS_FMAC);
        }
        while let Some(&(ready, c)) = self.clip_pipe.front() {
            if ready > self.cycle {
                break;
            }
            self.clip_pipe.pop_front();
            self.vis_clip = c;
        }
    }

    /// Drain every pipeline (end of program, macro mode).
    pub fn flush(&mut self) {
        if let Some(p) = self.q_pend.take() {
            self.apply_q(p);
        }
        if let Some(p) = self.p_pend.take() {
            self.p = p.value;
        }
        self.flag_pipe.clear();
        self.clip_pipe.clear();
        self.vis_mac = self.mac;
        self.vis_status = self.status;
        self.vis_clip = self.clip;
        if let Some(c) = self.vf_ready.iter().flatten().max() {
            if *c > self.cycle {
                self.cycle = *c;
            }
        }
    }

    // ------------------------------------------------------------------
    // Register helpers
    // ------------------------------------------------------------------

    #[inline]
    fn src(&self, src: Src, lane: usize) -> u32 {
        match src {
            Src::Vec(t) => self.vf[t as usize][lane],
            Src::Bc(t, f) => self.vf[t as usize][f as usize & 3],
            Src::Q => self.q,
            Src::I => self.i,
        }
    }

    /// Write VI (vi0 is hardwired); remembers the old value for the
    /// branch-reads-old-VI quirk.
    #[inline]
    fn set_vi(&mut self, r: u8, v: u16) {
        let r = r & 15;
        if r == 0 {
            return;
        }
        self.vi[r as usize] = v;
    }

    /// VI write that a following conditional branch observes as the *old*
    /// value. Per PCSX2 (`_vuBackupVI` in VUops.cpp) only the integer ALU ops
    /// (IADD/IADDI/IADDIU/ISUB/ISUBIU/IAND/IOR), MTIR and the auto-increment
    /// LQI/LQD/SQI/SQD do this — not ILW/ILWR, flag ops or links. Repeated
    /// writes to the same register keep the oldest value.
    #[inline]
    fn set_vi_bk(&mut self, r: u8, v: u16) {
        let r = r & 15;
        if r == 0 {
            return;
        }
        let old = match self.vi_backup {
            Some((b, o)) if b == r => o,
            _ => self.vi[r as usize],
        };
        self.vi_backup_next = Some((r, old));
        self.vi[r as usize] = v;
    }

    /// VI as read by a branch: a VI written by the immediately preceding
    /// pair reads its old value (PCSX2 `VI_BACKUP` behaviour).
    #[inline]
    fn vi_br(&self, r: u8) -> u16 {
        let r = r & 15;
        if !vi_backup_enabled() {
            return self.vi[r as usize];
        }
        match self.vi_backup {
            Some((b, old)) if b == r => old,
            _ => self.vi[r as usize],
        }
    }

    /// Lower-op VF write: discarded when the upper of the same pair targets
    /// the same register (the upper result wins).
    #[inline]
    fn lower_write(&mut self, ft: u8, dest: u8, v: [u32; 4]) {
        let ft = ft & 31;
        if ft == 0 || self.stage.target == Target::Vf(ft) {
            return;
        }
        for (l, val) in v.iter().enumerate() {
            if dest & (8 >> l) != 0 {
                self.vf[ft as usize][l] = *val;
                self.vf_ready[ft as usize][l] = self.cycle + FMAC_LATENCY;
            }
        }
    }

    #[inline]
    fn qaddr(&self, a: i32) -> usize {
        ((a as u32) & (self.data_qwords() - 1)) as usize * 16
    }

    #[inline]
    fn rd32(&self, off: usize) -> u32 {
        u32::from_le_bytes(self.data[off..off + 4].try_into().unwrap())
    }

    #[inline]
    fn wr32(&mut self, off: usize, v: u32) {
        self.data[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }

    fn load_q(&self, a: i32) -> [u32; 4] {
        let o = self.qaddr(a);
        [
            self.rd32(o),
            self.rd32(o + 4),
            self.rd32(o + 8),
            self.rd32(o + 12),
        ]
    }

    fn store_q(&mut self, a: i32, dest: u8, v: [u32; 4]) {
        let o = self.qaddr(a);
        for (l, val) in v.iter().enumerate() {
            if dest & (8 >> l) != 0 {
                self.wr32(o + l * 4, *val);
            }
        }
    }

    fn first_lane(dest: u8) -> usize {
        (0..4).find(|l| dest & (8 >> l) != 0).unwrap_or(0)
    }

    // ------------------------------------------------------------------
    // UPPER ops (stage a result; committed by end_pair)
    // ------------------------------------------------------------------

    fn stage_fmac(&mut self, target: Target, dest: u8, f: impl Fn(&Self, usize) -> (u32, Exc)) {
        let mut value = [0u32; 4];
        let mut mac = 0u32;
        for (l, slot) in value.iter_mut().enumerate() {
            if dest & (8 >> l) == 0 {
                continue;
            }
            let (v, e) = f(self, l);
            *slot = v;
            let b = 3 - l as u32;
            if v & ABS == 0 {
                mac |= 1 << b;
            }
            if v & SIGN != 0 {
                mac |= 1 << (b + 4);
            }
            if e.u {
                mac |= 1 << (b + 8);
            }
            if e.o {
                mac |= 1 << (b + 12);
            }
        }
        self.stage = Stage {
            target,
            dest,
            value,
            mac: Some(mac),
            clip: None,
        };
    }

    fn stage_plain(&mut self, target: Target, dest: u8, f: impl Fn(&Self, usize) -> u32) {
        let mut value = [0u32; 4];
        for (l, slot) in value.iter_mut().enumerate() {
            if dest & (8 >> l) != 0 {
                *slot = f(self, l);
            }
        }
        self.stage = Stage {
            target,
            dest,
            value,
            mac: None,
            clip: None,
        };
    }

    pub fn add(&mut self, dest: u8, fd: u8, fs: u8, src: Src) {
        self.stage_fmac(Target::Vf(fd), dest, |v, l| {
            float::add(v.vf[fs as usize][l], v.src(src, l))
        });
    }
    pub fn adda(&mut self, dest: u8, fs: u8, src: Src) {
        self.stage_fmac(Target::Acc, dest, |v, l| {
            float::add(v.vf[fs as usize][l], v.src(src, l))
        });
    }
    pub fn sub(&mut self, dest: u8, fd: u8, fs: u8, src: Src) {
        self.stage_fmac(Target::Vf(fd), dest, |v, l| {
            float::sub(v.vf[fs as usize][l], v.src(src, l))
        });
    }
    pub fn suba(&mut self, dest: u8, fs: u8, src: Src) {
        self.stage_fmac(Target::Acc, dest, |v, l| {
            float::sub(v.vf[fs as usize][l], v.src(src, l))
        });
    }
    pub fn mul(&mut self, dest: u8, fd: u8, fs: u8, src: Src) {
        self.stage_fmac(Target::Vf(fd), dest, |v, l| {
            float::mul(v.vf[fs as usize][l], v.src(src, l))
        });
    }
    pub fn mula(&mut self, dest: u8, fs: u8, src: Src) {
        self.stage_fmac(Target::Acc, dest, |v, l| {
            float::mul(v.vf[fs as usize][l], v.src(src, l))
        });
    }
    pub fn madd(&mut self, dest: u8, fd: u8, fs: u8, src: Src) {
        self.stage_fmac(Target::Vf(fd), dest, |v, l| {
            float::madd(v.acc[l], v.vf[fs as usize][l], v.src(src, l))
        });
    }
    pub fn madda(&mut self, dest: u8, fs: u8, src: Src) {
        self.stage_fmac(Target::Acc, dest, |v, l| {
            float::madd(v.acc[l], v.vf[fs as usize][l], v.src(src, l))
        });
    }
    pub fn msub(&mut self, dest: u8, fd: u8, fs: u8, src: Src) {
        self.stage_fmac(Target::Vf(fd), dest, |v, l| {
            float::msub(v.acc[l], v.vf[fs as usize][l], v.src(src, l))
        });
    }
    pub fn msuba(&mut self, dest: u8, fs: u8, src: Src) {
        self.stage_fmac(Target::Acc, dest, |v, l| {
            float::msub(v.acc[l], v.vf[fs as usize][l], v.src(src, l))
        });
    }
    pub fn max(&mut self, dest: u8, fd: u8, fs: u8, src: Src) {
        self.stage_plain(Target::Vf(fd), dest, |v, l| {
            float::max(v.vf[fs as usize][l], v.src(src, l))
        });
    }
    pub fn mini(&mut self, dest: u8, fd: u8, fs: u8, src: Src) {
        self.stage_plain(Target::Vf(fd), dest, |v, l| {
            float::min(v.vf[fs as usize][l], v.src(src, l))
        });
    }
    /// OPMULA.xyz ACC, fs, ft (outer product, first half).
    pub fn opmula(&mut self, fs: u8, ft: u8) {
        let (s, t) = (self.vf[fs as usize], self.vf[ft as usize]);
        self.stage_fmac(Target::Acc, 0b1110, |_, l| match l {
            0 => float::mul(s[1], t[2]),
            1 => float::mul(s[2], t[0]),
            _ => float::mul(s[0], t[1]),
        });
    }
    /// OPMSUB.xyz fd, fs, ft (outer product, second half).
    pub fn opmsub(&mut self, fd: u8, fs: u8, ft: u8) {
        let (s, t) = (self.vf[fs as usize], self.vf[ft as usize]);
        self.stage_fmac(Target::Vf(fd), 0b1110, |v, l| match l {
            0 => float::msub(v.acc[0], s[1], t[2]),
            1 => float::msub(v.acc[1], s[2], t[0]),
            _ => float::msub(v.acc[2], s[0], t[1]),
        });
    }
    pub fn itof(&mut self, frac: u8, dest: u8, ft: u8, fs: u8) {
        self.stage_plain(Target::Vf(ft), dest, |v, l| {
            float::from_fixed(v.vf[fs as usize][l] as i32, frac as i32)
        });
    }
    pub fn ftoi(&mut self, frac: u8, dest: u8, ft: u8, fs: u8) {
        self.stage_plain(Target::Vf(ft), dest, |v, l| {
            float::to_fixed(v.vf[fs as usize][l], frac as i32)
        });
    }
    pub fn abs(&mut self, dest: u8, ft: u8, fs: u8) {
        self.stage_plain(Target::Vf(ft), dest, |v, l| v.vf[fs as usize][l] & ABS);
    }
    /// CLIP.xyz fs, ft.w: shifts 6 new judgement bits into the clip flags.
    pub fn clipw(&mut self, fs: u8, ft: u8) {
        let w = float::norm_in(self.vf[ft as usize][3]) & ABS;
        let wk = w as i32;
        let mut flags = 0u32;
        for l in 0..3 {
            let x = float::norm_in(self.vf[fs as usize][l]);
            let k = if x & SIGN != 0 {
                -((x & ABS) as i32)
            } else {
                (x & ABS) as i32
            };
            if k > wk {
                flags |= 1 << (l * 2);
            }
            if k < -wk {
                flags |= 2 << (l * 2);
            }
        }
        self.stage = Stage {
            target: Target::None,
            dest: 0,
            value: [0; 4],
            mac: None,
            clip: Some(flags),
        };
    }

    // ------------------------------------------------------------------
    // LOWER ops (execute immediately against the pre-pair state)
    // ------------------------------------------------------------------

    pub fn lq(&mut self, dest: u8, ft: u8, is: u8, imm: i16) {
        let v = self.load_q(self.vi[is as usize & 15] as i32 + imm as i32);
        self.lower_write(ft, dest, v);
    }
    pub fn sq(&mut self, dest: u8, fs: u8, it: u8, imm: i16) {
        let v = self.vf[fs as usize & 31];
        self.store_q(self.vi[it as usize & 15] as i32 + imm as i32, dest, v);
    }
    pub fn ilw(&mut self, dest: u8, it: u8, is: u8, imm: i16) {
        let v = self.load_q(self.vi[is as usize & 15] as i32 + imm as i32);
        self.set_vi(it, v[Self::first_lane(dest)] as u16);
    }
    pub fn isw(&mut self, dest: u8, it: u8, is: u8, imm: i16) {
        let x = self.vi[it as usize & 15] as u32;
        self.store_q(self.vi[is as usize & 15] as i32 + imm as i32, dest, [x; 4]);
    }
    pub fn ilwr(&mut self, dest: u8, it: u8, is: u8) {
        self.ilw(dest, it, is, 0);
    }
    pub fn iswr(&mut self, dest: u8, it: u8, is: u8) {
        self.isw(dest, it, is, 0);
    }
    pub fn lqi(&mut self, dest: u8, ft: u8, is: u8) {
        let a = self.vi[is as usize & 15];
        let v = self.load_q(a as i32);
        self.lower_write(ft, dest, v);
        self.set_vi_bk(is, a.wrapping_add(1));
    }
    pub fn lqd(&mut self, dest: u8, ft: u8, is: u8) {
        let a = self.vi[is as usize & 15].wrapping_sub(1);
        self.set_vi_bk(is, a);
        let v = self.load_q(a as i32);
        self.lower_write(ft, dest, v);
    }
    pub fn sqi(&mut self, dest: u8, fs: u8, it: u8) {
        let a = self.vi[it as usize & 15];
        let v = self.vf[fs as usize & 31];
        self.store_q(a as i32, dest, v);
        self.set_vi_bk(it, a.wrapping_add(1));
    }
    pub fn sqd(&mut self, dest: u8, fs: u8, it: u8) {
        let a = self.vi[it as usize & 15].wrapping_sub(1);
        self.set_vi_bk(it, a);
        let v = self.vf[fs as usize & 31];
        self.store_q(a as i32, dest, v);
    }

    pub fn iaddiu(&mut self, it: u8, is: u8, imm: u16) {
        self.set_vi_bk(it, self.vi[is as usize & 15].wrapping_add(imm));
    }
    pub fn isubiu(&mut self, it: u8, is: u8, imm: u16) {
        self.set_vi_bk(it, self.vi[is as usize & 15].wrapping_sub(imm));
    }
    pub fn iadd(&mut self, id: u8, is: u8, it: u8) {
        self.set_vi_bk(
            id,
            self.vi[is as usize & 15].wrapping_add(self.vi[it as usize & 15]),
        );
    }
    pub fn isub(&mut self, id: u8, is: u8, it: u8) {
        self.set_vi_bk(
            id,
            self.vi[is as usize & 15].wrapping_sub(self.vi[it as usize & 15]),
        );
    }
    pub fn iaddi(&mut self, it: u8, is: u8, imm: i8) {
        self.set_vi_bk(
            it,
            self.vi[is as usize & 15].wrapping_add(imm as i16 as u16),
        );
    }
    pub fn iand(&mut self, id: u8, is: u8, it: u8) {
        self.set_vi_bk(id, self.vi[is as usize & 15] & self.vi[it as usize & 15]);
    }
    pub fn ior(&mut self, id: u8, is: u8, it: u8) {
        self.set_vi_bk(id, self.vi[is as usize & 15] | self.vi[it as usize & 15]);
    }

    pub fn fceq(&mut self, imm: u32) {
        self.set_vi(1, ((self.vis_clip & 0xff_ffff) == (imm & 0xff_ffff)) as u16);
    }
    pub fn fcand(&mut self, imm: u32) {
        self.set_vi(1, (self.vis_clip & imm & 0xff_ffff != 0) as u16);
    }
    pub fn fcor(&mut self, imm: u32) {
        self.set_vi(1, ((self.vis_clip | imm) & 0xff_ffff == 0xff_ffff) as u16);
    }
    pub fn fcset(&mut self, imm: u32) {
        let c = imm & 0xff_ffff;
        self.clip = c;
        self.vis_clip = c;
        for s in self.flag_pipe.iter_mut() {
            s.clip = c;
        }
        self.clip_pipe.clear();
    }
    pub fn fcget(&mut self, it: u8) {
        self.set_vi(it, (self.vis_clip & 0xfff) as u16);
    }
    pub fn fseq(&mut self, it: u8, imm: u16) {
        self.set_vi(it, ((self.vis_status & 0xfff) == imm as u32 & 0xfff) as u16);
    }
    pub fn fsand(&mut self, it: u8, imm: u16) {
        self.set_vi(it, (self.vis_status & imm as u32 & 0xfff) as u16);
    }
    pub fn fsor(&mut self, it: u8, imm: u16) {
        self.set_vi(it, ((self.vis_status | imm as u32) & 0xfff == 0xfff) as u16);
    }
    /// FSSET: write the sticky status bits (11..6).
    pub fn fsset(&mut self, imm: u16) {
        let sticky = imm as u32 & 0xfc0;
        self.status = (self.status & 0x3f) | sticky;
        self.vis_status = (self.vis_status & 0x3f) | sticky;
        for s in self.flag_pipe.iter_mut() {
            s.status = (s.status & 0x3f) | sticky;
        }
    }
    pub fn fmeq(&mut self, it: u8, is: u8) {
        self.set_vi(
            it,
            ((self.vis_mac & 0xffff) == self.vi[is as usize & 15] as u32) as u16,
        );
    }
    pub fn fmand(&mut self, it: u8, is: u8) {
        self.set_vi(it, (self.vis_mac & self.vi[is as usize & 15] as u32) as u16);
    }
    pub fn fmor(&mut self, it: u8, is: u8) {
        self.set_vi(
            it,
            ((self.vis_mac | self.vi[is as usize & 15] as u32) & 0xffff == 0xffff) as u16,
        );
    }

    // Branch conditions (evaluated in the branch's pair, VI-backup aware).
    pub fn ibeq(&self, it: u8, is: u8) -> bool {
        self.vi_br(is) == self.vi_br(it)
    }
    pub fn ibne(&self, it: u8, is: u8) -> bool {
        self.vi_br(is) != self.vi_br(it)
    }
    pub fn ibltz(&self, is: u8) -> bool {
        (self.vi_br(is) as i16) < 0
    }
    pub fn ibgtz(&self, is: u8) -> bool {
        (self.vi_br(is) as i16) > 0
    }
    pub fn iblez(&self, is: u8) -> bool {
        (self.vi_br(is) as i16) <= 0
    }
    pub fn ibgez(&self, is: u8) -> bool {
        (self.vi_br(is) as i16) >= 0
    }
    /// JR / JALR target (byte address).
    pub fn jr_target(&self, is: u8) -> u32 {
        (self.vi_br(is) as u32 * 8) & self.code_mask()
    }
    /// BAL / JALR link: `it = return_pc / 8`.
    pub fn link(&mut self, it: u8, return_pc: u32) {
        self.set_vi(it, (return_pc / 8) as u16);
    }

    pub fn vmove(&mut self, dest: u8, ft: u8, fs: u8) {
        let v = self.vf[fs as usize & 31];
        self.lower_write(ft, dest, v);
    }
    pub fn mr32(&mut self, dest: u8, ft: u8, fs: u8) {
        let s = self.vf[fs as usize & 31];
        self.lower_write(ft, dest, [s[1], s[2], s[3], s[0]]);
    }
    pub fn mtir(&mut self, it: u8, fs: u8, fsf: u8) {
        self.set_vi_bk(it, self.vf[fs as usize & 31][fsf as usize & 3] as u16);
    }
    pub fn mfir(&mut self, dest: u8, ft: u8, is: u8) {
        let v = self.vi[is as usize & 15] as i16 as i32 as u32;
        self.lower_write(ft, dest, [v; 4]);
    }

    fn div_issue(&mut self, r: float::DivResult, latency: u64, macro_mode: bool) {
        if let Some(p) = self.q_pend.take() {
            // A new DIV-unit op waits for the previous one.
            if p.ready > self.cycle {
                self.cycle = p.ready;
            }
            self.apply_q(p);
        }
        let p = Pending {
            value: r.value,
            ready: self.cycle + latency,
            invalid: r.invalid,
            div0: r.div0,
        };
        if macro_mode {
            self.apply_q(p);
        } else {
            self.q_pend = Some(p);
        }
    }
    pub fn div(&mut self, fs: u8, fsf: u8, ft: u8, ftf: u8) {
        let r = float::div(
            self.vf[fs as usize][fsf as usize & 3],
            self.vf[ft as usize][ftf as usize & 3],
        );
        self.div_issue(r, DIV_LATENCY, false);
    }
    pub fn sqrt(&mut self, ft: u8, ftf: u8) {
        let r = float::sqrt(self.vf[ft as usize][ftf as usize & 3]);
        self.div_issue(r, SQRT_LATENCY, false);
    }
    pub fn rsqrt(&mut self, fs: u8, fsf: u8, ft: u8, ftf: u8) {
        let r = float::rsqrt(
            self.vf[fs as usize][fsf as usize & 3],
            self.vf[ft as usize][ftf as usize & 3],
        );
        self.div_issue(r, RSQRT_LATENCY, false);
    }
    pub fn waitq(&mut self) {
        if let Some(p) = self.q_pend.take() {
            if p.ready > self.cycle {
                self.cycle = p.ready;
            }
            self.apply_q(p);
        }
    }

    fn lfsr_step(&mut self) {
        let x = (self.r >> 4) & 1;
        let y = (self.r >> 22) & 1;
        self.r = (((self.r << 1) ^ x ^ y) & 0x7f_ffff) | ONE;
    }
    pub fn rnext(&mut self, dest: u8, ft: u8) {
        self.lfsr_step();
        let r = self.r;
        self.lower_write(ft, dest, [r; 4]);
    }
    pub fn rget(&mut self, dest: u8, ft: u8) {
        let r = self.r;
        self.lower_write(ft, dest, [r; 4]);
    }
    pub fn rinit(&mut self, fs: u8, fsf: u8) {
        self.r = (self.vf[fs as usize][fsf as usize & 3] & 0x7f_ffff) | ONE;
    }
    pub fn rxor(&mut self, fs: u8, fsf: u8) {
        self.r = ((self.r ^ self.vf[fs as usize][fsf as usize & 3]) & 0x7f_ffff) | ONE;
    }

    pub fn mfp(&mut self, dest: u8, ft: u8) {
        let p = self.p;
        self.lower_write(ft, dest, [p; 4]);
    }
    pub fn xtop(&mut self, it: u8) {
        self.set_vi(it, (self.vif_top & 0x3ff) as u16);
    }
    pub fn xitop(&mut self, it: u8) {
        let m = if self.index == 0 { 0xff } else { 0x3ff };
        self.set_vi(it, (self.vif_itop & m) as u16);
    }

    fn efu_issue(&mut self, value: u32, latency: u64) {
        if let Some(p) = self.p_pend.take() {
            if p.ready > self.cycle {
                self.cycle = p.ready;
            }
            self.p = p.value;
        }
        self.p_pend = Some(Pending {
            value,
            ready: self.cycle + latency,
            invalid: false,
            div0: false,
        });
    }
    /// EFU op (ESADD, ERSADD, ELENG, ERLENG, EATANxy, EATANxz, ESUM) on fs.
    pub fn efu_vec(&mut self, op: decode::Efu, fs: u8) {
        use decode::Efu::*;
        let v = self.vf[fs as usize];
        let (x, y, z, w) = (
            float::to_f64(v[0]),
            float::to_f64(v[1]),
            float::to_f64(v[2]),
            float::to_f64(v[3]),
        );
        let sq = x * x + y * y + z * z;
        let r = match op {
            Esadd => sq,
            Ersadd => 1.0 / sq,
            Eleng => sq.sqrt(),
            Erleng => 1.0 / sq.sqrt(),
            Eatanxy => y.atan2(x),
            Eatanxz => z.atan2(x),
            Esum => x + y + z + w,
            _ => 0.0,
        };
        self.efu_issue(float::efu(r), op.latency());
    }
    /// EFU op (ESQRT, ERSQRT, ERCPR, ESIN, EATAN, EEXP) on fs.fsf.
    pub fn efu_scalar(&mut self, op: decode::Efu, fs: u8, fsf: u8) {
        use decode::Efu::*;
        let x = float::to_f64(self.vf[fs as usize][fsf as usize & 3]);
        let r = match op {
            Esqrt => x.abs().sqrt(),
            Ersqrt => 1.0 / x.abs().sqrt(),
            Ercpr => 1.0 / x,
            Esin => x.sin(),
            Eatan => x.atan(),
            Eexp => (-x).exp(),
            _ => 0.0,
        };
        self.efu_issue(float::efu(r), op.latency());
    }
    pub fn waitp(&mut self) {
        if let Some(p) = self.p_pend.take() {
            if p.ready > self.cycle {
                self.cycle = p.ready;
            }
            self.p = p.value;
        }
    }

    /// XGKICK: collect the GIF packet at vi[is]*16 (tags until EOP,
    /// wrapping at the end of data memory) and hand it to `kick`.
    pub fn xgkick(&mut self, is: u8, kick: &mut dyn FnMut(&[Qword])) {
        if self.index == 0 {
            return;
        }
        let pkt = self.gif_packet(self.vi[is as usize & 15] as u32);
        kick(&pkt);
    }

    /// The GIF packet starting at qword `addr` of data memory.
    pub fn gif_packet(&self, addr: u32) -> Vec<Qword> {
        let n = self.data_qwords();
        let mut q = addr & (n - 1);
        let mut out = Vec::new();
        let read = |q: u32| -> Qword {
            let o = (q as usize) * 16;
            u128::from_le_bytes(self.data[o..o + 16].try_into().unwrap())
        };
        loop {
            let tag = read(q);
            out.push(tag);
            q = (q + 1) & (n - 1);
            let lo = tag as u64;
            let nloop = (lo & 0x7fff) as u32;
            let eop = (lo >> 15) & 1 != 0;
            let flg = (lo >> 58) & 3;
            let mut nreg = ((lo >> 60) & 0xf) as u32;
            if nreg == 0 {
                nreg = 16;
            }
            let body = match flg {
                0 => nloop * nreg,
                1 => (nloop * nreg).div_ceil(2),
                _ => nloop,
            };
            for _ in 0..body {
                if out.len() as u32 >= n {
                    break;
                }
                out.push(read(q));
                q = (q + 1) & (n - 1);
            }
            if eop || out.len() as u32 >= n {
                break;
            }
        }
        out
    }

    // ------------------------------------------------------------------
    // Macro mode (EE COP2) and control registers
    // ------------------------------------------------------------------

    /// Execute one COP2 macro instruction. VCALLMS/VCALLMSR are ignored (the
    /// EE calls `start`). Results and flags are visible immediately.
    /// Statically translated macro ops (see `recomp::macro_source`) bracket
    /// their per-op call with these, matching `macro_op`.
    #[inline]
    pub fn macro_begin(&mut self) {
        self.stage = EMPTY_STAGE;
    }
    #[inline]
    pub fn macro_end_upper(&mut self) {
        self.commit_stage(false);
        self.flush();
    }
    #[inline]
    pub fn macro_end_lower(&mut self) {
        self.vi_backup_next = None;
        self.flush();
    }

    pub fn macro_op(&mut self, word: u32) {
        match decode::decode_macro(word) {
            decode::Macro::Upper(u) => {
                self.stage = EMPTY_STAGE;
                exec::exec_upper(self, &u);
                self.commit_stage(false);
            }
            decode::Macro::Lower(l) => {
                self.stage = EMPTY_STAGE;
                let mut nokick = |_: &[Qword]| {};
                exec::exec_lower(self, &l, 0, &mut nokick);
                self.vi_backup_next = None;
            }
            decode::Macro::CallMs => return,
            decode::Macro::Unknown(w) => self.error(format!("unknown COP2 macro op {w:#010x}")),
        }
        self.flush();
    }

    /// cfc2. 0..15 VI, 16 status, 17 MAC, 18 clip, 20 R, 21 I, 22 Q, 23 P,
    /// 26 TPC (pairs), 27 CMSAR0, 28 FBRST, 29 VPU-STAT (VU0 bit 0, VU1 bit 8).
    pub fn read_ctrl(&self, reg: u32) -> u32 {
        match reg {
            0..=15 => self.vi[reg as usize] as u32,
            16 => self.status & 0xfff,
            17 => self.mac & 0xffff,
            18 => self.clip & 0xff_ffff,
            20 => self.r & 0x7f_ffff,
            21 => self.i,
            22 => self.q,
            23 => self.p,
            26 => self.tpc / 8,
            27 => self.cmsar0,
            28 => self.fbrst & 0x0c0c,
            29 => {
                let me = (self.running as u32) << (self.index * 8);
                let other = (self.peer_running as u32) << ((1 - self.index.min(1)) * 8);
                me | other
            }
            _ => 0,
        }
    }

    /// ctc2. Writes to 17 (MAC), 23 (P), 26 (TPC), 29 are ignored. 16
    /// writes the sticky status bits. 28 FBRST: bit 1 resets VU0, bit 9
    /// VU1, bits 0/8 force-break; this instance only acts on its own bits
    /// (the machine forwards the VU1 half). 31 CMSAR1 sets
    /// `cmsar1_request = Some((val & 0xffff) * 8)` for the machine to start VU1.
    pub fn write_ctrl(&mut self, reg: u32, val: u32) {
        match reg {
            1..=15 => self.vi[reg as usize] = val as u16,
            16 => {
                self.status = (self.status & 0x3f) | (val & 0xfc0);
                self.vis_status = self.status;
            }
            18 => {
                self.clip = val & 0xff_ffff;
                self.vis_clip = self.clip;
            }
            20 => self.r = (val & 0x7f_ffff) | ONE,
            21 => self.i = val,
            22 => {
                self.q_pend = None;
                self.q = val;
            }
            27 => self.cmsar0 = val & 0xffff,
            28 => {
                self.fbrst = val;
                let shift = self.index * 8;
                if (val >> shift) & 2 != 0 {
                    self.reset();
                }
                if (val >> shift) & 1 != 0 {
                    self.running = false;
                }
            }
            31 => self.cmsar1_request = Some((val & 0xffff) * 8),
            _ => {}
        }
    }
}

/// Clamp helper exposed for tests and the EE FPU (`MAX` = largest PS2 float).
pub const PS2_MAX: u32 = MAX;

/// The branch-reads-old-VI quirk (see `set_vi_bk`). On by default;
/// SSX3_VU_VI_BACKUP=0 disables it for experiments.
fn vi_backup_enabled() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_VU_VI_BACKUP").is_ok_and(|v| v != "0"))
}
