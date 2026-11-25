//! Staging module oc556 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc556`.
//!
//! The script engine's one-call slots: a `WScriptProcess_vNN` or
//! `WScriptTask_vNN` virtual entry tests one field of its object and, when it
//! is set, forwards the object to `WScriptMan_CallProcessFunc` /
//! `WScriptMan_CallTaskFunc`. `WScriptProcess_LookupFunc` is a plain
//! forwarder to `WScriptMan_LookupFunc`.

use crate::call::{guest, tail_wrapper, Callee, Flow, Frame, Target};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0030_7bb0, wscript_process_v14, "WScriptProcess_v14"),
    (0x0030_7be0, wscript_process_v15, "WScriptProcess_v15"),
    (0x0030_7c10, wscript_process_v16, "WScriptProcess_v16"),
    (0x0030_7d00, wscript_process_v05, "WScriptProcess_v05"),
    (
        0x0030_8018,
        wscript_process_lookup_func,
        "WScriptProcess_LookupFunc",
    ),
    (0x0030_84d0, wscript_task_v07, "WScriptTask_v07"),
    (0x0030_8500, wscript_task_v08, "WScriptTask_v08"),
    (0x0030_8530, wscript_task_v09, "WScriptTask_v09"),
    (0x0030_8560, wscript_task_v10, "WScriptTask_v10"),
    (0x0030_8590, wscript_task_v11, "WScriptTask_v11"),
    (0x0030_85c0, wscript_task_v12, "WScriptTask_v12"),
    (0x0030_85f0, wscript_task_v13, "WScriptTask_v13"),
    (0x0030_8620, wscript_task_v14, "WScriptTask_v14"),
    (0x0030_8650, wscript_task_v15, "WScriptTask_v15"),
    (0x0030_8680, wscript_task_v16, "WScriptTask_v16"),
    (
        0x0030_86f0,
        wscript_task_run_update_handler,
        "WScriptTask_RunUpdateHandler",
    ),
];

/// The script manager's dispatchers and lookup (the generated bodies).
mod wscript_callee {
    use super::{ee, Callee};

    /// `WScriptMan_CallProcessFunc(object, process)`.
    pub const CALL_PROCESS_FUNC: Callee = Callee {
        addr: 0x0030_ac98,
        f: ee::s10::f_0030ac98,
    };
    /// `WScriptMan_CallTaskFunc(object, task)`.
    pub const CALL_TASK_FUNC: Callee = Callee {
        addr: 0x0030_ada8,
        f: ee::s10::f_0030ada8,
    };
    /// `WScriptMan_LookupFunc(name)`.
    pub const LOOKUP_FUNC: Callee = Callee {
        addr: 0x0030_9b70,
        f: ee::s10::f_00309b70,
    };
}

/// A process slot (`WScriptProcess_vNN`, 44 bytes) with `this` in a0: when
/// `this->field` is set, `WScriptMan_CallProcessFunc(this->+0x10)` returns to
/// `ret`. The callee gets `a1 = this` and `a2` = the field, and the epilogue
/// reloads `ra`.
fn wscript_process_forward(c: &mut Cpu, field: u32, ret: u32) -> Flow {
    c.cycles += 6;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let this = c.g[A0][0] as u32;
    c.g[A1][0] = c.g[A0][0];
    let set = Addr(this.wrapping_add(field)).lw(c);
    c.g[A2][0] = set;
    if set != 0 {
        c.cycles += 3;
        c.g[A0][0] = Addr(this.wrapping_add(0x10)).lw(c);
        wscript_callee::CALL_PROCESS_FUNC.call(c, ret)?;
    }
    c.cycles += 2;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// A task slot (`WScriptTask_vNN`, 44 bytes) with `this` in a0: when
/// `this->field` is set, `WScriptMan_CallTaskFunc(this->+0x3c->+0x10)`
/// returns to `ret`. The callee's leftover `v0` is what the caller sees.
fn wscript_task_forward(c: &mut Cpu, field: u32, ret: u32) -> Flow {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    let this = c.g[A0][0] as u32;
    let set = Addr(this.wrapping_add(field)).lw(c);
    c.g[A2][0] = set;
    c.g[A1][0] = c.g[A0][0];
    if set != 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(this.wrapping_add(0x3c)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
        wscript_callee::CALL_TASK_FUNC.call(c, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `WScriptProcess_v14` (0x307bb0): forwards when `this+0x50` is set.
pub fn wscript_process_v14(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_7bb0);
    guest(c, |c| wscript_process_forward(c, 0x50, 0x0030_7bd0));
}

/// `WScriptProcess_v15` (0x307be0): forwards when `this+0x54` is set.
pub fn wscript_process_v15(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_7be0);
    guest(c, |c| wscript_process_forward(c, 0x54, 0x0030_7c00));
}

/// `WScriptProcess_v16` (0x307c10): forwards when `this+0x58` is set.
pub fn wscript_process_v16(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_7c10);
    guest(c, |c| wscript_process_forward(c, 0x58, 0x0030_7c30));
}

/// `WScriptProcess_v05` (0x307d00): forwards when `this+0x2c` is set.
pub fn wscript_process_v05(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_7d00);
    guest(c, |c| wscript_process_forward(c, 0x2c, 0x0030_7d20));
}

/// `WScriptProcess_LookupFunc` (0x308018): `WScriptMan_LookupFunc(this->+0x10)`
/// as a tail-style forwarder (the Ghidra pseudo-C says the same).
pub fn wscript_process_lookup_func(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_8018);
    tail_wrapper(
        c,
        Some(0x10),
        Target::Direct(wscript_callee::LOOKUP_FUNC),
        0x0030_8028,
    );
}

/// `WScriptTask_v07` (0x3084d0): forwards when `this+0x14` is set.
pub fn wscript_task_v07(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_84d0);
    guest(c, |c| wscript_task_forward(c, 0x14, 0x0030_84f0));
}

/// `WScriptTask_v08` (0x308500): forwards when `this+0x18` is set.
pub fn wscript_task_v08(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_8500);
    guest(c, |c| wscript_task_forward(c, 0x18, 0x0030_8520));
}

/// `WScriptTask_v09` (0x308530): forwards when `this+0x1c` is set.
pub fn wscript_task_v09(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_8530);
    guest(c, |c| wscript_task_forward(c, 0x1c, 0x0030_8550));
}

/// `WScriptTask_v10` (0x308560): forwards when `this+0x20` is set.
pub fn wscript_task_v10(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_8560);
    guest(c, |c| wscript_task_forward(c, 0x20, 0x0030_8580));
}

/// `WScriptTask_v11` (0x308590): forwards when `this+0x24` is set.
pub fn wscript_task_v11(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_8590);
    guest(c, |c| wscript_task_forward(c, 0x24, 0x0030_85b0));
}

/// `WScriptTask_v12` (0x3085c0): forwards when `this+0x28` is set.
pub fn wscript_task_v12(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_85c0);
    guest(c, |c| wscript_task_forward(c, 0x28, 0x0030_85e0));
}

/// `WScriptTask_v13` (0x3085f0): forwards when `this+0x2c` is set.
pub fn wscript_task_v13(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_85f0);
    guest(c, |c| wscript_task_forward(c, 0x2c, 0x0030_8610));
}

/// `WScriptTask_v14` (0x308620): forwards when `this+0x30` is set.
pub fn wscript_task_v14(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_8620);
    guest(c, |c| wscript_task_forward(c, 0x30, 0x0030_8640));
}

/// `WScriptTask_v15` (0x308650): forwards when `this+0x34` is set.
pub fn wscript_task_v15(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_8650);
    guest(c, |c| wscript_task_forward(c, 0x34, 0x0030_8670));
}

/// `WScriptTask_v16` (0x308680): forwards when `this+0x38` is set.
pub fn wscript_task_v16(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_8680);
    guest(c, |c| wscript_task_forward(c, 0x38, 0x0030_86a0));
}

/// `WScriptTask_RunUpdateHandler` (0x3086f0): forwards when `this+0x0c` is set.
pub fn wscript_task_run_update_handler(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030_86f0);
    guest(c, |c| wscript_task_forward(c, 0x0c, 0x0030_8710));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// The process the forwarders are given (its `+0x10` is the object).
    const WS_PROC: u32 = DATA + 0x100;
    /// The object `WScriptMan_CallProcessFunc` / `CallTaskFunc` works on.
    const WS_OBJ: u32 = DATA + 0x400;
    /// The task the task forwarders are given (its `+0x3c` is a process).
    const WS_TASK: u32 = DATA + 0x200;
    /// That process (`+0x10` is the object).
    const WS_TASK_PROC: u32 = DATA + 0x300;
    /// Gate word for the lookup fake: only tests that write the magic see it.
    const WS_LOOKUP_GATE: u32 = DATA + 0x500;
    const WS_LOOKUP_MAGIC: u32 = 0x5c0f_f1e1;

    /// The process forwarder fixture: the object pointer, the process's run
    /// flag (non-zero: the callee takes its call path), and the field set on
    /// two seeds out of three.
    fn wscript_process_setup(c: &mut Cpu, seed: u32, field: u32) {
        let mut r = Rng(0x7751_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(WS_PROC);
        c.bus.write32(WS_PROC + 0x10, WS_OBJ);
        c.bus.write32(WS_PROC + 0x1c, 1);
        let set = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
        c.bus.write32(WS_PROC + field, set);
    }

    /// The task forwarder fixture: the task's process at `+0x3c` holds the
    /// object at `+0x10`, with the run flag at `+0x1c`.
    fn wscript_task_setup(c: &mut Cpu, seed: u32, field: u32) {
        let mut r = Rng(0x7752_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(WS_TASK);
        c.bus.write32(WS_TASK + 0x3c, WS_TASK_PROC);
        c.bus.write32(WS_TASK_PROC + 0x10, WS_OBJ);
        c.bus.write32(WS_TASK_PROC + 0x1c, 1);
        let set = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
        c.bus.write32(WS_TASK + field, set);
    }

    /// The lookup fake, gated on [`WS_LOOKUP_MAGIC`]: it logs the call like
    /// any fake. With the gate off it runs the body it replaced.
    fn wscript_lookup_fake(c: &mut Cpu) {
        if c.bus.read32(WS_LOOKUP_GATE) != WS_LOOKUP_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(wscript_lookup_fake),
                ee::s10::f_00309b70__generated,
            );
        }
        fakes::fake_call(c, 0x0030_9b70);
    }

    #[test]
    fn wscript_process_v14_matches() {
        check(
            ee::s10::f_00307bb0__generated,
            wscript_process_v14,
            300,
            &|c, seed| wscript_process_setup(c, seed, 0x50),
        );
    }

    #[test]
    fn wscript_process_v15_matches() {
        check(
            ee::s10::f_00307be0__generated,
            wscript_process_v15,
            300,
            &|c, seed| wscript_process_setup(c, seed, 0x54),
        );
    }

    #[test]
    fn wscript_process_v16_matches() {
        check(
            ee::s10::f_00307c10__generated,
            wscript_process_v16,
            300,
            &|c, seed| wscript_process_setup(c, seed, 0x58),
        );
    }

    #[test]
    fn wscript_process_v05_matches() {
        check(
            ee::s10::f_00307d00__generated,
            wscript_process_v05,
            300,
            &|c, seed| wscript_process_setup(c, seed, 0x2c),
        );
    }

    #[test]
    fn wscript_process_lookup_func_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00309B70),
            wscript_lookup_fake,
            fakes::prev_cell(wscript_lookup_fake),
        );
        check(
            ee::s10::f_00308018__generated,
            wscript_process_lookup_func,
            300,
            &|c, seed| {
                let mut r = Rng(0x7753_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(WS_LOOKUP_GATE, WS_LOOKUP_MAGIC);
                c.g[A0][0] = sx(WS_PROC);
                c.bus.write32(WS_PROC + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn wscript_task_v07_matches() {
        check(
            ee::s10::f_003084d0__generated,
            wscript_task_v07,
            300,
            &|c, seed| wscript_task_setup(c, seed, 0x14),
        );
    }

    #[test]
    fn wscript_task_v08_matches() {
        check(
            ee::s10::f_00308500__generated,
            wscript_task_v08,
            300,
            &|c, seed| wscript_task_setup(c, seed, 0x18),
        );
    }

    #[test]
    fn wscript_task_v09_matches() {
        check(
            ee::s10::f_00308530__generated,
            wscript_task_v09,
            300,
            &|c, seed| wscript_task_setup(c, seed, 0x1c),
        );
    }

    #[test]
    fn wscript_task_v10_matches() {
        check(
            ee::s10::f_00308560__generated,
            wscript_task_v10,
            300,
            &|c, seed| wscript_task_setup(c, seed, 0x20),
        );
    }

    #[test]
    fn wscript_task_v11_matches() {
        check(
            ee::s10::f_00308590__generated,
            wscript_task_v11,
            300,
            &|c, seed| wscript_task_setup(c, seed, 0x24),
        );
    }

    #[test]
    fn wscript_task_v12_matches() {
        check(
            ee::s10::f_003085c0__generated,
            wscript_task_v12,
            300,
            &|c, seed| wscript_task_setup(c, seed, 0x28),
        );
    }

    #[test]
    fn wscript_task_v13_matches() {
        check(
            ee::s10::f_003085f0__generated,
            wscript_task_v13,
            300,
            &|c, seed| wscript_task_setup(c, seed, 0x2c),
        );
    }

    #[test]
    fn wscript_task_v14_matches() {
        check(
            ee::s10::f_00308620__generated,
            wscript_task_v14,
            300,
            &|c, seed| wscript_task_setup(c, seed, 0x30),
        );
    }

    #[test]
    fn wscript_task_v15_matches() {
        check(
            ee::s10::f_00308650__generated,
            wscript_task_v15,
            300,
            &|c, seed| wscript_task_setup(c, seed, 0x34),
        );
    }

    #[test]
    fn wscript_task_v16_matches() {
        check(
            ee::s10::f_00308680__generated,
            wscript_task_v16,
            300,
            &|c, seed| wscript_task_setup(c, seed, 0x38),
        );
    }

    #[test]
    fn wscript_task_run_update_handler_matches() {
        check(
            ee::s10::f_003086f0__generated,
            wscript_task_run_update_handler,
            300,
            &|c, seed| wscript_task_setup(c, seed, 0x0c),
        );
    }
}
