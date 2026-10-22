//! `src/bx/execman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::float;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events;
pub(crate) use crate::game::visualfx::worldlightman::DELETE;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_0ad8, ps2_exec_man_v01, "PS2ExecMan_v01"),
    (0x0032_0b68, math_ceil_log2, "Math_CeilLog2"),
];


// ---------------------------------------------------------------------------
// Math.

/// `int Math_CeilLog2(uint x)` (0x320b68): bit-smear `x` to one less than the
/// next power of two and return the number of leading zeros of that
/// (i.e. the ceiling log2, 31 for 0/1).
pub fn math_ceil_log2(c: &mut Cpu) {
    c.cycles += 33;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[V1][0] = 0xffff_ffff_ff00_0000;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = 0xffff_ffff_f000_0000;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 31) as u32);
    c.g[A3][0] = 0xffff_ffff_c000_0000;
    c.g[T0][0] = c.g[V0][0] & 0x10;
    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
    c.g[V0][0] = sx((c.g[A0][0] as u32) << (c.g[T0][0] as u32 & 31));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = c.g[A0][0] & c.g[A1][0];
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 31) as u32);
    c.g[A0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
    c.g[V1][0] = c.g[V1][0] & 0x8;
    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 31) as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << (c.g[V1][0] as u32 & 31));
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 31) as u32);
    c.g[V1][0] = c.g[A2][0] & 0x4;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << (c.g[V1][0] as u32 & 31));
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32 >> 31) as u32);
    c.g[V1][0] = c.g[A3][0] & 0x2;
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << (c.g[V1][0] as u32 & 31));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 31) as u32);
    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
    c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = c.g[T0][0] ^ 0x1f;
    abi::ret(c);
}


/// `PS2ExecMan_v01` (0x320ad8): vtable at `+0x8`.
pub fn ps2_exec_man_v01(c: &mut Cpu) {
    deleting_dtor(c, 0x8, 0x0048_dc30, 0x0032_0afc);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use ssx3_ee as ee;
    pub(crate) use crate::game::visualfx::worldlightman::tests::MEM_MUTEX;
    pub(crate) use crate::game::visualfx::worldlightman::tests::alloc_setup;
    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;


    #[test]
    fn math_ceil_log2_matches() {
        check(ssx3_ee::s10::f_00320b68, math_ceil_log2, 256, &|c, seed| {
            let mut r = Rng(0x1006 ^ seed as u64);
            scramble(c, &mut r);
            let x = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 0xffff_ffff,
                3 => 0x8000_0000,
                _ => r.next_u32(),
            };
            c.g[A0][0] = x as u64;
        });
    }


    #[test]
    fn ps2_exec_man_v01_matches() {
        dtor_case(ee::s10::f_00320ad8__generated, ps2_exec_man_v01, 0x320ad8);
    }
}
