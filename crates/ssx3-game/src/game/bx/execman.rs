//! `src/bx/execman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
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
    (0x0032_0920, exec_man_base_dtor, "ExecManBase_dtor"),
    (0x0032_0950, exec_man_base_v11_00320950, "ExecManBase_v11"),
    (0x0032_0958, exec_man_base_v12_00320958, "ExecManBase_v12"),
    (
        0x0032_0c18,
        input_binding_eval_float_args,
        "InputBinding_EvalFloatArgs",
    ),
    (
        0x0032_0c70,
        input_binding_eval_bool_args,
        "InputBinding_EvalBoolArgs",
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

/// Shared body of the `FEUIInput_Is*` button queries: `InputBinding_EvalBool(
/// *(input + 0x18), button)` (0x320c48); `ret` is the call's return address.
pub(crate) fn oc821_feui_eval(c: &mut Cpu, button: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = button;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0032_0c48, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Shared body of the `FEUIInput_Is*` button queries: `InputBinding_EvalBool(
/// *(input + 0x18), button)` (0x320c48); `ret` is the call's return address.
pub(crate) fn oc822_feui_eval(c: &mut Cpu, button: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = button;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0032_0c48, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Shared body of the `FEUIInput_Is*` d-pad queries: `InputBinding_EvalBool(
/// *(input + 0x18), button)` (0x320c48); `ret` is the call's return address.
pub(crate) fn oc823_feui_eval(c: &mut Cpu, button: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = button;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0032_0c48, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Shared body of the `FEUIInput_Is*` shoulder queries: `InputBinding_EvalBool(
/// *(input + 0x18), button)` (0x320c48); `ret` is the call's return address.
pub(crate) fn oc824_feui_eval(c: &mut Cpu, button: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = button;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0032_0c48, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ExecManBase_v11` (0x0032_0950): straight-line leaf. Mechanical port of the generated body.
pub fn exec_man_base_v11_00320950(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ExecManBase_v12` (0x0032_0958): straight-line leaf. Mechanical port of the generated body.
pub fn exec_man_base_v12_00320958(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ExecManBase::~ExecManBase(this, flags)` (0x320920): stores the vtable
/// word (0x48dc30) at `this + 8`, then deletes the object when bit 0 of
/// `flags` is set.
pub fn exec_man_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x23d0));
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let delete_object = c.g[A1][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        if delete_object {
            c.cycles += 2;
            Callee {
                addr: 0x0031_7e50,
                f: ee::s10::__builtin_delete,
            }
            .call(c, 0x0032_0944)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `InputBinding_EvalFloatArgs(binding, a, b)` (0x320c18): calls
/// `InputMap_EvalFloatArgs(binding[1], binding[0], a, b)`, the binding's
/// words at +4 and +0 first.
pub fn input_binding_eval_float_args(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let binding = c.g[A0][0] as u32;
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A2][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = Addr(binding).lw(c);
        c.g[A0][0] = Addr(binding.wrapping_add(4)).lw(c);
        Callee {
            addr: 0x0032_1010,
            f: ee::s10::InputMap_EvalFloatArgs,
        }
        .call(c, 0x0032_0c38)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `InputBinding_EvalBoolArgs(binding, a, b)` (0x320c70): the boolean twin
/// of [`input_binding_eval_float_args`], calling `InputMap_EvalBoolArgs`.
pub fn input_binding_eval_bool_args(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let binding = c.g[A0][0] as u32;
        c.g[V0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A2][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = Addr(binding).lw(c);
        c.g[A0][0] = Addr(binding.wrapping_add(4)).lw(c);
        Callee {
            addr: 0x0032_1188,
            f: ee::s10::InputMap_EvalBoolArgs,
        }
        .call(c, 0x0032_0c90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
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

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// `ExecManBase_v11` (0x0032_0950) against the generated body.
    #[test]
    fn exec_man_base_v11_00320950_matches() {
        check(
            ee::s10::f_00320950__generated,
            exec_man_base_v11_00320950,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x320950, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ExecManBase_v12` (0x0032_0958) against the generated body.
    #[test]
    fn exec_man_base_v12_00320958_matches() {
        check(
            ee::s10::f_00320958__generated,
            exec_man_base_v12_00320958,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x320958, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// Gate words of the fakes below: one per fake, each in the machine's
    /// own RAM (beyond the fakes' call log), armed with `OC1580_MAGIC`.
    pub(crate) const OC1580_MAGIC: u32 = 0x1580_0001;

    pub(crate) const OC1580_DELETE_CTRL: u32 = DATA + 0x2f_0000;

    pub(crate) const OC1580_INPUT_FLOAT_CTRL: u32 = DATA + 0x2f_1000;

    pub(crate) const OC1580_INPUT_BOOL_CTRL: u32 = DATA + 0x2f_2000;

    pub(crate) static OC1580_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1580_PREV_INPUT_FLOAT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1580_PREV_INPUT_BOOL: AtomicUsize = AtomicUsize::new(0);

    /// `__builtin_delete`: a logged no-op call.
    pub(crate) fn oc1580_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC1580_DELETE_CTRL) != OC1580_MAGIC {
            return fakes::fall_back(c, &OC1580_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    /// `InputMap_EvalFloatArgs`: a logged no-op call.
    pub(crate) fn oc1580_fake_input_float(c: &mut Cpu) {
        if c.bus.read32(OC1580_INPUT_FLOAT_CTRL) != OC1580_MAGIC {
            return fakes::fall_back(c, &OC1580_PREV_INPUT_FLOAT, ee::s10::f_00321010__generated);
        }
        fakes::fake_call(c, 0x0032_1010);
    }

    /// `InputMap_EvalBoolArgs`: a logged no-op call.
    pub(crate) fn oc1580_fake_input_bool(c: &mut Cpu) {
        if c.bus.read32(OC1580_INPUT_BOOL_CTRL) != OC1580_MAGIC {
            return fakes::fall_back(c, &OC1580_PREV_INPUT_BOOL, ee::s10::f_00321188__generated);
        }
        fakes::fake_call(c, 0x0032_1188);
    }

    pub(crate) fn oc1580_install_delete() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1580_fake_delete,
            &OC1580_PREV_DELETE,
        );
    }

    pub(crate) fn oc1580_install_input() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00321010),
            oc1580_fake_input_float,
            &OC1580_PREV_INPUT_FLOAT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00321188),
            oc1580_fake_input_bool,
            &OC1580_PREV_INPUT_BOOL,
        );
    }

    #[test]
    fn exec_man_base_dtor_matches() {
        check(
            ee::s10::f_00320920__generated,
            exec_man_base_dtor,
            200,
            &|c, seed| {
                oc1580_install_delete();
                let mut r = machine(c, seed, 0x0032_0920, 0);
                c.bus.write32(OC1580_DELETE_CTRL, OC1580_MAGIC);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                if seed % 2 == 0 {
                    c.g[A1][0] &= !1u64;
                } else {
                    c.g[A1][0] |= 1;
                }
            },
        );
    }

    #[test]
    fn input_binding_eval_float_args_matches() {
        check(
            ee::s10::f_00320c18__generated,
            input_binding_eval_float_args,
            200,
            &|c, seed| {
                oc1580_install_input();
                let mut r = machine(c, seed, 0x0032_0c18, 1);
                c.bus.write32(OC1580_INPUT_FLOAT_CTRL, OC1580_MAGIC);
                let binding = DATA + 0x1000;
                c.bus.write32(binding, r.next_u32());
                c.bus.write32(binding + 4, r.next_u32());
                c.g[A0][0] = binding as u64;
            },
        );
    }

    #[test]
    fn input_binding_eval_bool_args_matches() {
        check(
            ee::s10::f_00320c70__generated,
            input_binding_eval_bool_args,
            200,
            &|c, seed| {
                oc1580_install_input();
                let mut r = machine(c, seed, 0x0032_0c70, 1);
                c.bus.write32(OC1580_INPUT_BOOL_CTRL, OC1580_MAGIC);
                let binding = DATA + 0x1000;
                c.bus.write32(binding, r.next_u32());
                c.bus.write32(binding + 4, r.next_u32());
                c.g[A0][0] = binding as u64;
            },
        );
    }
}
