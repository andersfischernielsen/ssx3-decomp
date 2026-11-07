//! `src/bx/execman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_0ad8, ps2_exec_man_v01, "PS2ExecMan_v01"),
    (0x0032_0b68, math_ceil_log2, "Math_CeilLog2"),
    (
        0x0032_0c48,
        input_binding_eval_bool,
        "InputBinding_EvalBool",
    ),
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

// ---------------------------------------------------------------------------
// InputBinding_EvalBool (0x320c48)

/// `InputMap_EvalBool` (0x321108).
pub(crate) const INPUT_MAP_EVAL_BOOL: Callee = Callee {
    addr: 0x0032_1108,
    f: ee::s10::InputMap_EvalBool,
};

/// `bool InputBinding_EvalBool(InputBinding *binding, int index)`
/// (0x320c48): evaluates the binding's input map (+4) as a bool, with the
/// binding's first word and `index` as the map's arguments.
pub fn input_binding_eval_bool(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let binding = Addr(r32(c, A0));
        c.g[A1][0] = binding.lw(c);
        c.g[A0][0] = binding.off(0x4).lw(c);
        INPUT_MAP_EVAL_BOOL.call(c, 0x0032_0c60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;

    use ssx3_ee as ee;

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

    /// InputBinding_EvalBool through the real InputMap_EvalBool and a VM
    /// program that stops at once (the camera tests' input map), with the
    /// stopped VM's value 0 or 1.
    #[test]
    fn input_binding_eval_bool_matches() {
        check(
            ee::s10::f_00320c48__generated,
            input_binding_eval_bool,
            200,
            &|c, seed| {
                let mut r = Rng(0x0032_0c48 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0032_0c48;
                let binding = DATA + 0x1000;
                let map = DATA + 0x2000;
                c.g[A0][0] = binding as u64;
                c.g[A1][0] = (seed % 4) as u64; // the map's binding index
                c.bus.write32(binding, r.next_u32());
                c.bus.write32(binding + 4, map);
                crate::game::camera::camera::tests::input_map(c, map);
                // The stopped VM leaves `*(parser + 8)` in f0 (parser = the
                // map's frame below this function's 0x10 frame).
                let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
                c.bus
                    .write32(crate::difftest::SP - 0x10 - 0x1d0 + 8, value.to_bits());
            },
        );
    }
}
