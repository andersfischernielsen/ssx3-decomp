//! Staging module oc700 (wave wave238): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc700`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::game::lib::kernel::{do_syscall, event_if_due, DINTR};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_4950, disable_dmac_wrapper, "DisableDmac"),
    (0x0042_49b8, enable_dmac_wrapper, "EnableDmac"),
    (0x0042_4c50, i_wakeup_thread_signal, "iWakeupThread"),
];

/// `_DisableDmac(int channel)` (0x423b10), the syscall wrapper.
const DISABLE_DMAC_CALL: Callee = Callee {
    addr: 0x0042_3b10,
    f: ee::s15::f_00423b10,
};

/// `_EnableDmac(int channel)` (0x423b00), the syscall wrapper.
const ENABLE_DMAC_CALL: Callee = Callee {
    addr: 0x0042_3b00,
    f: ee::s15::f_00423b00,
};

/// `EIntr()` (0x42c0c0): enable interrupts.
const EINTR_CALL: Callee = Callee {
    addr: 0x0042_c0c0,
    f: ee::s15::f_0042c0c0,
};

/// `_iWakeupThread(int thid)` (0x423ce0).
const I_WAKEUP_THREAD_CALL: Callee = Callee {
    addr: 0x0042_3ce0,
    f: ee::s15::f_00423ce0,
};

/// `iSignalSema(int sema)` (0x423dd0).
const I_SIGNAL_SEMA_CALL: Callee = Callee {
    addr: 0x0042_3dd0,
    f: ee::s15::f_00423dd0,
};

/// Shared shape of `DisableDmac` (0x424950) and `EnableDmac` (0x4249b8):
/// run the syscall wrapper `dmac` with interrupts off. When Status.EIE
/// (COP0 r12 bit 16) was set on entry, `DIntr` turns them off first and
/// `EIntr` turns them back on after the call. The wrapper's result is left
/// in v0. `rets` are the return addresses of the `DIntr`, `dmac` and
/// `EIntr` calls, in that order.
fn dmac_wrapper_with_intr(c: &mut Cpu, dmac: Callee, rets: (u32, u32, u32)) {
    let (intr_off_ret, dmac_ret, intr_on_ret) = rets;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        let at = frame.at(c, 0x10);
        c.sd(at, c.g[S1][0]);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let at = frame.at(c, 0);
        c.sd(at, c.g[S0][0]);
        c.mfc0(S0, 12);
        c.g[V0][0] = 0x10000;
        c.g[S0][0] = c.g[S0][0] & c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            DINTR.call(c, intr_off_ret)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        dmac.call(c, dmac_ret)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = c.g[S1][0];
        // S0 is read after the call, as the compiled code does.
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            EINTR_CALL.call(c, intr_on_ret)?;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x20);
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int DisableDmac(int channel)` (0x424950): `_DisableDmac` with the
/// interrupts off (see `dmac_wrapper_with_intr`).
pub fn disable_dmac_wrapper(c: &mut Cpu) {
    dmac_wrapper_with_intr(
        c,
        DISABLE_DMAC_CALL,
        (0x0042_4980, 0x0042_4988, 0x0042_49a0),
    );
}

/// `int EnableDmac(int channel)` (0x4249b8): `_EnableDmac` with the
/// interrupts off (see `dmac_wrapper_with_intr`).
pub fn enable_dmac_wrapper(c: &mut Cpu) {
    dmac_wrapper_with_intr(c, ENABLE_DMAC_CALL, (0x0042_49e8, 0x0042_49f0, 0x0042_4a08));
}

/// `iWakeupThread`-shaped wrapper (0x424c50, the generated name). It makes
/// the `-47` kernel call, which returns an id in v0 (kept in s0). If that id
/// equals the argument, the id is checked against 0x100 and the gate word at
/// 0x455170: when both pass, the id is queued in the 512-entry ring at
/// 0x52b3a8 (counter word at +4; each 2-byte entry at +8 is a zero flag byte
/// then the id), the semaphore whose id sits at 0x52b3a0 is signalled, and v0
/// is the id; otherwise v0 is -1. A different argument goes to
/// `_iWakeupThread` (0x423ce0) and its result is returned. Field meanings
/// are inferred from the generated code (the reference body at this address
/// is an unrelated hash function), so the names are provisional.
pub fn i_wakeup_thread_signal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        let at = frame.at(c, 0);
        c.sd(at, c.g[S0][0]);
        c.g[V1][0] = sx((-47i32) as u32);
        if !do_syscall(c, 0x0042_4c60, 0x0042_4c64) {
            return Err(Unwound);
        }
        event_if_due(c);
        c.g[S0][0] = c.g[V0][0];
        let same = c.g[S0][0] == c.g[A0][0];
        c.g[V0][0] = (c.g[S0][0] < 0x100) as u64;
        if !same {
            c.cycles += 4;
            I_WAKEUP_THREAD_CALL.call(c, 0x0042_4c78)?;
            frame.restore_ra(c, 0x10);
            c.cycles += 3;
            c.g[S0][0] = c.ld(frame.at(c, 0));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let out_of_range = c.g[V0][0] == 0;
        c.g[V0][0] = 0x450000;
        let mut queued = false;
        if !out_of_range {
            c.cycles += 3;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5170));
            let gate_set = c.g[V1][0] != 0;
            c.g[V1][0] = 0x530000;
            queued = gate_set;
        }
        if queued {
            c.cycles += 14;
            c.g[A1][0] = 0x530000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4c58));
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x4c60));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] &= 0x1ff;
            c.g[A2][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[V1][0] = c.g[A1][0];
            c.sb((c.g[A1][0] as u32).wrapping_add(0x8), 0u64);
            c.sb((c.g[V1][0] as u32).wrapping_add(0x9), c.g[S0][0]);
            I_SIGNAL_SEMA_CALL.call(c, 0x0042_4cd0)?;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 1;
        frame.restore_ra(c, 0x10);
        c.cycles += 3;
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, LOG_RESULTS};
    use crate::difftest::Rng;

    /// The DMAC wrappers: real `DIntr`/`EIntr`, fake syscalls (the wrapper's
    /// trap may unwind). Status.EIE is set on half the seeds.
    fn dmac_wrapper_syscall_test(gen: GuestFn, hand: GuestFn, entry: u32, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 1);
            c.pc = entry;
            c.g[A0][0] = (r.next_u32() % 8) as u64;
            let status = r.next_u32() & !0x1_0000;
            c.cop0[12] = if seed % 2 == 0 {
                status | 0x1_0000
            } else {
                status
            };
        };
        covered(gen, hand, 200, &setup);
    }

    #[test]
    fn disable_dmac_wrapper_matches() {
        dmac_wrapper_syscall_test(
            ee::s15::f_00424950__generated,
            disable_dmac_wrapper,
            0x0042_4950,
            0x4950,
        );
    }

    #[test]
    fn enable_dmac_wrapper_matches() {
        dmac_wrapper_syscall_test(
            ee::s15::f_004249b8__generated,
            enable_dmac_wrapper,
            0x0042_49b8,
            0x49b8,
        );
    }

    /// `iWakeupThread`: the `-47` call's id (forced through the fake's first
    /// result) equals the argument on two seeds in three, is in range on
    /// four in five, and the gate word is set on half the seeds; the other
    /// id paths call `_iWakeupThread`. Fake syscalls may unwind.
    #[test]
    fn i_wakeup_thread_signal_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x4c50, 3);
            c.pc = 0x0042_4c50;
            let id = if seed % 5 == 0 {
                0x100 + r.next_u32() % 0x1000
            } else {
                1 + r.next_u32() % 0xff
            };
            c.bus.write32(LOG_RESULTS, id);
            let same = seed % 3 != 0;
            c.g[A0][0] = if same { id as u64 } else { (id ^ 1) as u64 };
            let gate = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(0x0045_5170, gate);
        };
        covered(
            ee::s15::f_00424c50__generated,
            i_wakeup_thread_signal,
            200,
            &setup,
        );
    }
}
