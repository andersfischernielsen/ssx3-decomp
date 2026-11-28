//! Staging module oc1594 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1594`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::lib::kernel::syscall_wrapper;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// `void SetGsCrt(int interlace, int mode, int field)` (0x4239a0): syscall 2.
pub fn set_gs_crt(c: &mut Cpu) {
    syscall_wrapper(c, 0x2, 0x0042_39a4, 0x0042_39a8);
}

/// `void SetVTLBRefillHandler(int which, void (*handler)(void))` (0x423a50):
/// syscall 0xd.
pub fn set_vtlb_refill_handler(c: &mut Cpu) {
    syscall_wrapper(c, 0xd, 0x0042_3a54, 0x0042_3a58);
}

/// `void SetAlarm(usec ticks, handler, arg)` (0x423b20): syscall 0xfc.
pub fn set_alarm(c: &mut Cpu) {
    syscall_wrapper(c, 0xfc, 0x0042_3b24, 0x0042_3b28);
}

/// `void iSetAlarm(int ticks, handler, arg)` (0x423b80): interrupt-disabled
/// syscall -254.
pub fn i_set_alarm(c: &mut Cpu) {
    syscall_wrapper(c, (-254i32) as u32, 0x0042_3b84, 0x0042_3b88);
}

/// `void TerminateThread(int thid)` (0x423bf0): syscall 0x25.
pub fn terminate_thread(c: &mut Cpu) {
    syscall_wrapper(c, 0x25, 0x0042_3bf4, 0x0042_3bf8);
}

/// `void iTerminateThread(int thid)` (0x423c00): interrupt-disabled syscall
/// -38.
pub fn i_terminate_thread(c: &mut Cpu) {
    syscall_wrapper(c, (-38i32) as u32, 0x0042_3c04, 0x0042_3c08);
}

/// The `%g` conversion tail of the printf family (0x421ed4): `_gcvt(reent,
/// value, ndigits, buf)` with `ndigits` the caller's `a3` or 1 and `buf` the
/// output pointer in `s0`; then it resumes the shared printf epilogue at
/// 0x421f0c (a tail jump, so nothing returns here).
pub fn printf_g_tail(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 8;
        c.g[A2][0] = 0x1;
        c.g[A0][0] = c.g[T3][0];
        if c.g[A3][0] != 0 {
            c.g[A2][0] = c.g[A3][0];
        }
        c.g[T1][0] = c.g[T2][0];
        c.g[A3][0] = c.g[S0][0];
        Callee {
            addr: 0x0042_1b98,
            f: ee::s15::f_00421b98,
        }
        .call(c, 0x0042_1eec)?;
        c.g[V0][0] = c.g[S0][0];
        c.pc = 0x0042_1f0c;
        call_indirect(c, 0x0042_1f0c);
        Ok(())
    });
}

/// The `%e` conversion tail of the printf family (0x421ef4): `print_e(reent,
/// buf, value, precision)` with `buf = s0` and `value` the caller's `a1`;
/// then it tail-jumps to the shared printf epilogue at 0x421f08.
pub fn printf_e_tail(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 5;
        c.g[A2][0] = c.g[A1][0];
        c.g[A0][0] = c.g[T3][0];
        c.g[T1][0] = c.g[T2][0];
        c.g[A1][0] = c.g[S0][0];
        Callee {
            addr: 0x0042_16e8,
            f: ee::s15::f_004216e8,
        }
        .call(c, 0x0042_1f08)?;
        c.pc = 0x0042_1f08;
        ee::s15::f_00421f08(c);
        Ok(())
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_39a0, set_gs_crt, "SetGsCrt"),
    (0x0042_3a50, set_vtlb_refill_handler, "SetVTLBRefillHandler"),
    (0x0042_3b20, set_alarm, "SetAlarm"),
    (0x0042_3b80, i_set_alarm, "iSetAlarm"),
    (0x0042_3bf0, terminate_thread, "TerminateThread"),
    (0x0042_3c00, i_terminate_thread, "iTerminateThread"),
    (0x0042_1ed4, printf_g_tail, "lbl_00421ED4"),
    (0x0042_1ef4, printf_e_tail, "lbl_00421EF4"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, Rng};
    use crate::game::lib::kernel::tests::syscall_test;
    use crate::game::lib::libc::tests::{dbl, dbl_2, heap_init, DST, REENT_3};

    #[test]
    fn set_gs_crt_matches() {
        syscall_test(ee::s15::f_004239a0__generated, set_gs_crt, 0x1594_01);
    }

    #[test]
    fn set_vtlb_refill_handler_matches() {
        syscall_test(
            ee::s15::f_00423a50__generated,
            set_vtlb_refill_handler,
            0x1594_02,
        );
    }

    #[test]
    fn set_alarm_matches() {
        syscall_test(ee::s15::f_00423b20__generated, set_alarm, 0x1594_03);
    }

    #[test]
    fn i_set_alarm_matches() {
        syscall_test(ee::s15::f_00423b80__generated, i_set_alarm, 0x1594_04);
    }

    #[test]
    fn terminate_thread_matches() {
        syscall_test(ee::s15::f_00423bf0__generated, terminate_thread, 0x1594_05);
    }

    #[test]
    fn i_terminate_thread_matches() {
        syscall_test(
            ee::s15::f_00423c00__generated,
            i_terminate_thread,
            0x1594_06,
        );
    }

    /// `%g` tail: `_gcvt` writes the digits at `s0`, then the epilogue at
    /// 0x421f0c returns through the frame's saved `ra`.
    #[test]
    fn printf_g_tail_matches() {
        check(ee::s15::f_00421ed4__generated, printf_g_tail, 160, &|c, seed| {
            let mut r = Rng(0x421e_d4 ^ seed as u64);
            heap_init(c, &mut r);
            for i in 0..0x80 {
                c.bus.write8(DST + i, 0xcc);
            }
            c.g[T3][0] = REENT_3 as u64;
            c.g[T2][0] = [0, 1][seed as usize % 2] as u64;
            c.g[S0][0] = DST as u64;
            c.g[A1][0] = dbl_2(seed, &mut r);
            c.g[A3][0] = [0, 3, 6][seed as usize % 3] as u64;
            if seed % 4 == 3 {
                c.g[A1][0] = 123.456f64.to_bits();
                c.g[A3][0] = 2;
            }
        });
    }

    /// `%e` tail: `print_e` writes the digits at `s0`, then the epilogue at
    /// 0x421f08 returns through the frame's saved `ra`.
    #[test]
    fn printf_e_tail_matches() {
        check(ee::s15::f_00421ef4__generated, printf_e_tail, 120, &|c, seed| {
            let mut r = Rng(0x421e_f4 ^ seed as u64);
            heap_init(c, &mut r);
            for i in 0..0x200 {
                c.bus.write8(DST + i, 0xcc);
            }
            c.g[T3][0] = REENT_3 as u64;
            c.g[T2][0] = [0, 1][seed as usize % 2] as u64;
            c.g[S0][0] = DST as u64;
            c.g[A1][0] = dbl(seed, &mut r);
            c.g[A3][0] = [0, 1, 2, 3, 6, 10, 17, 30][seed as usize % 8] as u64;
            if seed % 8 == 2 {
                c.g[A1][0] = 123.456f64.to_bits();
                c.g[A3][0] = 3;
            }
        });
    }
}
