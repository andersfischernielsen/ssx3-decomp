//! Phase 3 rendering, stage 0 check (`SSX3_VERIFY_VUCALLS=1`): the EE-level
//! frame reader (`ssx3-render`) must see exactly what the emulated VIF1 hands
//! each VU1 microprogram.
//!
//! When a VIF1 chain DMA starts, `ssx3_render` walks the same chain from EE
//! memory (VIF registers seeded from the emulated VIF1) and queues the
//! [`VuCall`]s it finds. Every emulated microprogram start on VIF1 then takes
//! the next expected call and compares the entry, TOP/ITOP and every input
//! quadword with VU1 data memory. Counts are printed at exit.

use std::collections::VecDeque;
use std::sync::Mutex;

use ssx3_render::chain::{self, Mem};
use ssx3_render::vif::{Event, Vif1, VifRegs, VuCall};

use crate::vif::Vif;

#[derive(Default)]
struct State {
    expected: VecDeque<VuCall>,
    chains: u64,
    calls_ok: u64,
    mismatches: u64,
    unexpected: u64,
    directs: u64,
    reported: u32,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

pub fn enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("SSX3_VERIFY_VUCALLS").is_some())
}

struct EeMem<'a> {
    ram: &'a [u8],
    spr: &'a [u8],
}

impl Mem for EeMem<'_> {
    fn qword(&self, addr: u32) -> u128 {
        crate::hw::mem_q(self.ram, self.spr, addr, addr & 0x8000_0000 != 0)
    }
}

fn regs_of(v: &Vif) -> VifRegs {
    VifRegs {
        cycle: v.cycle,
        mode: v.mode,
        mask: v.mask,
        row: v.row,
        col: v.col,
        base: v.base,
        ofst: v.ofst,
        tops: v.tops,
        itops: v.itops,
        dbf: v.dbf,
    }
}

/// A VIF1 chain DMA starts at `tadr` (after `pending` quadwords of a
/// previous transfer). Walk it at the EE level and queue the calls.
#[allow(clippy::too_many_arguments)]
pub fn chain_start(vif1: &Vif, ram: &[u8], spr: &[u8], pending: &[u128], tadr: u32, tte: bool, tie: bool, asp: u32, asr: [u32; 2]) {
    let mut w = Vif1::new(regs_of(vif1));
    if !pending.is_empty() {
        w.write(pending);
    }
    let mut asr = asr;
    let mem = EeMem { ram, spr };
    chain::walk(&mem, tadr, tte, tie, asp, &mut asr, &mut |q| w.write(q));
    let mut g = STATE.lock().unwrap();
    let s = g.get_or_insert_with(State::default);
    s.chains += 1;
    for e in w.events {
        match e {
            Event::Call(c) => s.expected.push_back(c),
            Event::Direct(_) => s.directs += 1,
            _ => {}
        }
    }
}

/// The emulated VIF1 starts a microprogram at `pc` with `top`/`itop`;
/// `mem` is VU1 data memory at that moment.
pub fn program_start(pc: Option<u32>, top: u32, itop: u32, mem: &[u8]) {
    let mut g = STATE.lock().unwrap();
    let s = g.get_or_insert_with(State::default);
    let Some(exp) = s.expected.pop_front() else {
        s.unexpected += 1;
        return;
    };
    let mut bad = Vec::new();
    if exp.entry.is_some() && exp.entry != pc {
        bad.push(format!("entry {:?} vs {:?}", exp.entry, pc));
    }
    if exp.top != top || exp.itop != itop {
        bad.push(format!("top/itop {:#x}/{:#x} vs {top:#x}/{itop:#x}", exp.top, exp.itop));
    }
    let mut wrong = 0;
    for (a, v) in exp.writes() {
        let o = *a as usize * 16;
        let got = u128::from_le_bytes(mem[o..o + 16].try_into().unwrap());
        if got != *v {
            if wrong == 0 {
                bad.push(format!("qword {a:#x}: walker {v:#034x} vu {got:#034x}"));
            }
            wrong += 1;
        }
    }
    if wrong > 0 {
        bad.push(format!("{wrong}/{} input qwords differ", exp.writes().count()));
    }
    if bad.is_empty() {
        s.calls_ok += 1;
    } else {
        s.mismatches += 1;
        if s.reported < 20 {
            s.reported += 1;
            eprintln!("[vucall-verify] mismatch at call {} (pc {pc:?}): {}", s.calls_ok + s.mismatches, bad.join("; "));
        }
    }
}

/// Summary line.
pub fn summary() -> Option<String> {
    let g = STATE.lock().unwrap();
    let s = g.as_ref()?;
    Some(format!(
        "[vucall-verify] chains {} calls ok {} mismatches {} unexpected {} still queued {} direct packets {}",
        s.chains,
        s.calls_ok,
        s.mismatches,
        s.unexpected,
        s.expected.len(),
        s.directs
    ))
}
