//! Staging module oc1566 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1566`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::lib::gcc::callee::{CP_PUSH_EXCEPTION, THROW};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0041_2d08, f_00412d08, "f_00412d08"),
    (0x0041_2d88, f_00412d88, "f_00412d88"),
    (0x0042_1f08, f_00421f08, "f_00421f08"),
];

/// Shared body of the `__throw_bad_cast` / `__throw_bad_typeid` tails
/// (0x412d08 / 0x412d88, resumable at `base`, `base+8`, `base+0x10`): store the
/// vtable pointer into the new exception object, call `__throw`-prep
/// `(*vtable)[1]()` for the type_info, `__cp_push_exception(obj, ti, handler)`,
/// `__throw`, then tail into `tail`. The two siblings differ only in the
/// handler offset (`a2_off`) and the tail function.
pub(crate) fn throw_obj_tail(c: &mut Cpu, base: u32, a2_off: u32, tail: (u32, GuestFn)) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            if pc == base {
                c.cycles += 2;
                c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                pc = base + 0x10;
                continue;
            }
            if pc == base + 8 {
                c.cycles += 2;
                c.g[RA][0] = sx(base + 0x10);
                THROW.call(c, base + 0x10)?;
                pc = base + 0x10;
                continue;
            }
            if pc == base + 0x10 {
                c.cycles += 13;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6748));
                c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(base + 0x28);
                call_ptr(c, t, base + 0x28)?;
                c.g[A2][0] = 0x410000;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(a2_off));
                c.g[RA][0] = sx(base + 0x3c);
                c.g[A0][0] = c.g[S0][0];
                CP_PUSH_EXCEPTION.call(c, base + 0x3c)?;
                c.g[RA][0] = sx(base + 0x44);
                THROW.call(c, base + 0x44)?;
                c.pc = tail.0;
                (tail.1)(c);
                return Ok(());
            }
            c.jump_out(pc, base, tail.0);
            return Ok(());
        }
    })
}

/// `__throw_bad_cast` tail (0x412d08): throws the allocated `bad_cast`.
pub fn f_00412d08(c: &mut Cpu) {
    throw_obj_tail(c, 0x0041_2d08, 0x5120, (0x0041_2d4c, ee::s15::f_00412d4c));
}

/// `__throw_bad_typeid` tail (0x412d88): throws the allocated `bad_typeid`.
pub fn f_00412d88(c: &mut Cpu) {
    throw_obj_tail(c, 0x0041_2d88, 0x5088, (0x0041_2dcc, ee::s15::f_00412dcc));
}

/// Epilogue `return s0` of a function (0x421f08, resumable at 0x421f0c):
/// reload `ra` and `s0`, drop the 0x20-byte frame.
pub fn f_00421f08(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x0042_1f08 => {
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
                pc = 0x0042_1f0c;
                continue;
            }
            0x0042_1f0c => {
                c.cycles += 4;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0042_1f08, 0x0042_1f1c);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::DATA;

    /// Fixture for the throw tails: exception object at `DATA+0x200`, its
    /// vtable pointer (stored from `s1`) at `DATA+0x300` with a real leaf as
    /// the type_info getter; entry `base`, `base+8` or `base+0x10` by seed.
    fn throw_tail_setup(c: &mut Cpu, seed: u32, base: u32) {
        machine(c, seed, base as u64, 0);
        c.bus.write32(LOG_UNWIND_AT, 1);
        c.g[S0][0] = (DATA + 0x200) as u64;
        c.g[S1][0] = (DATA + 0x300) as u64;
        c.bus.write32(DATA + 0x304, 0x0014_06c0);
        c.pc = base + [0, 8, 0x10][(seed % 3) as usize];
    }

    /// 0x412d08 against the generated tail, from all three entries.
    #[test]
    fn f_00412d08_matches() {
        check(ee::s15::f_00412d08__generated, f_00412d08, 24, &|c, seed| {
            throw_tail_setup(c, seed, 0x0041_2d08)
        });
    }

    /// 0x412d88 against the generated tail, from all three entries.
    #[test]
    fn f_00412d88_matches() {
        check(ee::s15::f_00412d88__generated, f_00412d88, 24, &|c, seed| {
            throw_tail_setup(c, seed, 0x0041_2d88)
        });
    }

    /// 0x421f08 (`return s0` epilogue) from both entries.
    #[test]
    fn f_00421f08_matches() {
        check(ee::s15::f_00421f08__generated, f_00421f08, 16, &|c, seed| {
            machine(c, seed, 0x0042_1f08, 0);
            let sp = c.g[SP][0] as u32;
            for i in 0..8 {
                c.bus.write32(sp + i * 4, seed.wrapping_mul(0x9e37_79b1) ^ i);
            }
            c.pc = if seed % 2 == 0 { 0x0042_1f08 } else { 0x0042_1f0c };
        });
    }
}
