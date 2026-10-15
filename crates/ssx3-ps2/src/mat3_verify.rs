//! Phase 3 rendering, stage 2A check (`SSX3_VERIFY_MAT3=1`): the native type
//! 3 VU1 program (`ssx3_render::mat3`) must produce exactly the XGKICK
//! packets and the data memory of the translated microprogram
//! (`d159716bb1281fbe`).
//!
//! On every VIF1 microprogram start in that program, [`begin`] copies VU1
//! data memory; the translated program then runs as usual while its kicks
//! are collected ([`Pending::kick`]); [`end`] runs the native program on the
//! copy and compares the kick sequence and the whole data memory. Counts per
//! entry (and ITOP pass) are printed at exit ([`summary`]); the first
//! mismatches are printed with details. Emulation itself is unchanged.

use std::collections::BTreeMap;
use std::sync::Mutex;

use ssx3_render::mat3;

use crate::vu::Vu;
use crate::Qword;

pub fn enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("SSX3_VERIFY_MAT3").is_some())
}

#[derive(Default, Clone, Copy)]
struct Counts {
    ok: u64,
    mismatch: u64,
    not_implemented: u64,
    /// Calls that kicked clipped fans (more than one packet).
    with_fans: u64,
}

#[derive(Default)]
struct State {
    /// Keyed by (entry, ITOP != 0).
    counts: BTreeMap<(u32, bool), Counts>,
    /// Not implemented, by (entry, mesh flags at qw34.w after the call).
    declined: BTreeMap<(u32, u32), u64>,
    reported: u32,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

/// One call in flight: the input copy and the translated program's kicks.
pub struct Pending {
    entry: u32,
    top: u32,
    itop: u32,
    mem: Box<[u8; 16384]>,
    kicks: Vec<Vec<Qword>>,
}

impl Pending {
    pub fn kick(&mut self, p: &[Qword]) {
        self.kicks.push(p.to_vec());
    }
}

/// Before the translated program runs (VIF1 only).
pub fn begin(vu: &mut Vu, pc: u32, top: u32, itop: u32) -> Option<Pending> {
    if !enabled() || vu.segment_hash_pub(pc) != mat3::HASH {
        return None;
    }
    Some(Pending {
        entry: pc,
        top,
        itop,
        mem: vu.data[..].try_into().map(Box::new).ok()?,
        kicks: Vec::new(),
    })
}

/// After the translated program finished: run the native one and compare.
pub fn end(mut p: Pending, vu: &Vu) {
    let mut native: Vec<Vec<Qword>> = Vec::new();
    let mut regs = mat3::VuState::default();
    let implemented = mat3::run(p.entry, p.top, p.itop, &mut p.mem, &mut regs, &mut |k| native.push(k.to_vec()));
    let mut g = STATE.lock().unwrap();
    let s = g.get_or_insert_with(State::default);
    let c = s.counts.entry((p.entry, p.itop != 0)).or_default();
    if !implemented {
        c.not_implemented += 1;
        let flags = u32::from_le_bytes(vu.data[34 * 16 + 12..34 * 16 + 16].try_into().unwrap());
        *s.declined.entry((p.entry, flags)).or_default() += 1;
        return;
    }
    if p.kicks.len() > 1 {
        c.with_fans += 1;
    }
    let kicks_ok = native == p.kicks;
    let mem_ok = p.mem[..] == vu.data[..];
    if kicks_ok && mem_ok {
        c.ok += 1;
        return;
    }
    c.mismatch += 1;
    if s.reported >= 10 {
        return;
    }
    s.reported += 1;
    let frame = crate::vu::capture::frame();
    eprintln!(
        "[mat3-verify] mismatch #{}: entry {:#06x} top {:#05x} itop {:#x} frame {frame}: kicks native {} translated {}",
        s.reported,
        p.entry,
        p.top,
        p.itop,
        native.len(),
        p.kicks.len()
    );
    if let Some(i) = (0..native.len().max(p.kicks.len())).find(|&i| native.get(i) != p.kicks.get(i)) {
        let (a, b) = (native.get(i), p.kicks.get(i));
        eprintln!(
            "  kick {i}: native {} qw, translated {} qw",
            a.map_or(0, |k| k.len()),
            b.map_or(0, |k| k.len())
        );
        if let (Some(a), Some(b)) = (a, b) {
            for (j, (x, y)) in a.iter().zip(b).enumerate().filter(|(_, (x, y))| x != y).take(6) {
                eprintln!("    qw {j}: native {x:032x} translated {y:032x}");
            }
        }
    }
    let diff: Vec<usize> = (0..1024).filter(|q| p.mem[q * 16..q * 16 + 16] != vu.data[q * 16..q * 16 + 16]).collect();
    if !diff.is_empty() {
        eprintln!("  data memory: {} qwords differ, first at {:?}", diff.len(), &diff[..diff.len().min(8)]);
    }
}

/// Counts per entry, for the exit report (None when the check is off).
pub fn summary() -> Option<String> {
    let g = STATE.lock().unwrap();
    let s = g.as_ref()?;
    let mut out = String::from("[mat3-verify] native type 3 vs translated d159716b (entry/pass: ok, mismatch, not implemented):");
    let mut total = Counts::default();
    for (&(entry, pass2), c) in &s.counts {
        out += &format!(
            "\n  {entry:#06x}{}: ok {} mismatch {} not-implemented {} (calls with clipped fans: {})",
            if pass2 { " itop!=0" } else { "" },
            c.ok,
            c.mismatch,
            c.not_implemented,
            c.with_fans
        );
        total.ok += c.ok;
        total.mismatch += c.mismatch;
        total.not_implemented += c.not_implemented;
    }
    for (&(entry, flags), n) in &s.declined {
        out += &format!("\n  not implemented: entry {entry:#06x} flags {flags:#x}: {n}");
    }
    out += &format!(
        "\n  total: ok {} mismatch {} not-implemented {}",
        total.ok, total.mismatch, total.not_implemented
    );
    Some(out)
}
