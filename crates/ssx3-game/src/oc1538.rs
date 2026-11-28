//! Staging module oc1538 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1538`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_bda0, math_wrap_angle_deg, "Math_WrapAngleDeg"),
    (0x0016_bde8, math_wrap_angle_clamp_deg, "Math_WrapAngleClampDeg"),
    (0x0017_fe4c, lbl_0017fe4c, "lbl_0017FE4C"),
    (0x0017_fe54, lbl_0017fe54, "lbl_0017FE54"),
    (0x0017_fe5c, lbl_0017fe5c, "lbl_0017FE5C"),
    (0x0018_0508, keyboard_screen_ctor, "KeyboardScreen_ctor"),
    (0x0019_5be0, fe_locale_load_fe_tables, "FELocale_LoadFETables"),
];

/// `Math_WrapAnglePi` (0x16bce0): wraps the angle in `*a0` to [-pi, pi].
const OC1538_WRAP_ANGLE_PI: Callee = Callee {
    addr: 0x0016_bce0,
    f: ee::s02::f_0016bce0,
};

/// `Math_WrapAngleClampHalfPi` (0x16bd30): clamps the angle in `*a0` to [-pi/2, pi/2].
const OC1538_WRAP_ANGLE_CLAMP_HALF_PI: Callee = Callee {
    addr: 0x0016_bd30,
    f: ee::s02::f_0016bd30,
};

/// `cFELocale_addFile` (0x1959a8, `FELocale_LoadTable`): `(self, table id)`.
const OC1538_FE_LOCALE_ADD_FILE: Callee = Callee {
    addr: 0x0019_59a8,
    f: ee::s02::f_001959a8,
};

/// Shared body of `Math_WrapAngleDeg` and `Math_WrapAngleClampDeg`: converts
/// the float at `*a0` with the gp constant at `gp - in_gp`, passes it to
/// `wrap` through a stack slot (`a0 = sp`), and converts the result back with
/// the constant at `gp - out_gp`, storing it through `a0` (saved in s0).
fn wrap_angle_scaled(c: &mut Cpu, in_gp: u32, out_gp: u32, wrap: Callee, ret_addr: u32) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0x30);
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(in_gp));
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = c.g[SP][0];
    c.lwc1(0, c.g[S0][0] as u32);
    c.mul_s(0, 0, 1);
    let slot = frame.at(c, 0);
    c.swc1(0, slot);
    wrap.call(c, ret_addr)?;
    c.lwc1(0, slot);
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(out_gp));
    frame.restore_ra(c, 0x10);
    c.mul_s(0, 0, 1);
    c.swc1(0, c.g[S0][0] as u32);
    frame.restore(c, S0, 0x20);
    frame.close(c);
    Ok(())
}

/// `Math_WrapAngleDeg` (0x16bda0): the float at `*a0`, scaled by `gp-0x6460`,
/// wrapped to [-pi, pi] by `Math_WrapAnglePi`, scaled back by `gp-0x645c`.
pub fn math_wrap_angle_deg(c: &mut Cpu) {
    guest(c, |c| {
        wrap_angle_scaled(c, 0x6460, 0x645c, OC1538_WRAP_ANGLE_PI, 0x0016_bdc8)
    })
}

/// `Math_WrapAngleClampDeg` (0x16bde8): the float at `*a0`, scaled by
/// `gp-0x6458`, clamped to [-pi/2, pi/2] by `Math_WrapAngleClampHalfPi`,
/// scaled back by `gp-0x6454`.
pub fn math_wrap_angle_clamp_deg(c: &mut Cpu) {
    guest(c, |c| {
        wrap_angle_scaled(
            c,
            0x6458,
            0x6454,
            OC1538_WRAP_ANGLE_CLAMP_HALF_PI,
            0x0016_be10,
        )
    })
}

/// The three label stubs at 0x17fe4c, 0x17fe54 and 0x17fe5c: `a1 = id` and a
/// tail jump to 0x17fe68 (the resume arm of `0x17fe64`) through the function
/// table. No frame and no return check: the arm returns to our caller.
fn tail_to_fe68(c: &mut Cpu, id: u64) {
    guest(c, |c| {
        c.cycles += 2;
        c.g[A1][0] = id;
        c.pc = 0x0017_fe68;
        call_indirect(c, 0x0017_fe68);
        Ok(())
    })
}

/// `lbl_0017FE4C`: tail jump with `a1 = 0x76`.
pub fn lbl_0017fe4c(c: &mut Cpu) {
    tail_to_fe68(c, 0x76)
}

/// `lbl_0017FE54`: tail jump with `a1 = 0x77`.
pub fn lbl_0017fe54(c: &mut Cpu) {
    tail_to_fe68(c, 0x77)
}

/// `lbl_0017FE5C`: tail jump with `a1 = 0x78`.
pub fn lbl_0017fe5c(c: &mut Cpu) {
    tail_to_fe68(c, 0x78)
}

/// The object pointer in `s0` (the `KeyboardScreen` constructor), read again
/// after each call as the generated code does.
fn kb_self(c: &Cpu) -> u32 {
    c.g[S0][0] as u32
}

/// `KeyboardScreen::KeyboardScreen(self, parent state)` (0x180508): links the
/// object into itself, constructs its screen attributes, thread list, root
/// group and object list, then sets the keyboard state (shift on, first key
/// 1, current key 10, max 45 characters, cursor 0). Returns `self`.
pub fn keyboard_screen_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 51;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4670));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw(kb_self(c).wrapping_add(0x4), c.g[S0][0]);
        c.g[A0][0] = sx(kb_self(c).wrapping_add(0x14));
        c.sw(kb_self(c), c.g[S0][0]);
        c.sw(kb_self(c).wrapping_add(0xc), 0);
        c.sw(kb_self(c).wrapping_add(0x10), 0);
        c.sw(kb_self(c).wrapping_add(0x8), c.g[V0][0]);
        // Attribute screen constructor (0x1dd580).
        call_ptr(c, 0x001d_d580, 0x0018_0544)?;
        c.g[A0][0] = sx(kb_self(c).wrapping_add(0x18));
        // Thread list constructor (0x3977e8).
        call_ptr(c, 0x0039_77e8, 0x0018_054c)?;
        c.sw(kb_self(c).wrapping_add(0x34), 0);
        c.g[A1][0] = 0;
        c.sw(kb_self(c).wrapping_add(0x38), 0);
        c.g[A2][0] = 0;
        c.sw(kb_self(c).wrapping_add(0x3c), 0);
        c.g[A0][0] = sx(kb_self(c).wrapping_add(0x40));
        // Root group constructor (0x39fb30).
        call_ptr(c, 0x0039_fb30, 0x0018_0568)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A0][0] = sx(kb_self(c).wrapping_add(0xb4));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x49e8));
        c.sw(kb_self(c).wrapping_add(0x48), c.g[V1][0]);
        // Object list constructor (0x3977e8).
        call_ptr(c, 0x0039_77e8, 0x0018_057c)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[A2][0] = 0x1;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2e60));
        c.g[A0][0] = 0xa;
        c.g[A1][0] = 0x2d;
        c.sw(kb_self(c).wrapping_add(0xd0), c.g[S1][0]);
        c.sw(kb_self(c).wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(kb_self(c).wrapping_add(0xe8), c.g[A2][0]);
        c.sw(kb_self(c).wrapping_add(0x170), c.g[A0][0]);
        c.sw(kb_self(c).wrapping_add(0xd4), c.g[A1][0]);
        c.sw(kb_self(c).wrapping_add(0xd8), 0);
        c.sw(kb_self(c).wrapping_add(0xdc), 0);
        c.sw(kb_self(c).wrapping_add(0xe0), 0);
        c.sw(kb_self(c).wrapping_add(0xe4), c.g[A2][0]);
        c.sb(kb_self(c).wrapping_add(0xec), 0);
        c.sw(kb_self(c).wrapping_add(0x16c), 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFELocale::loadFE(self)` (0x195be0): the FE locale's three table loads.
/// Reads the language byte at `self+1` (signed: 0..3 select a table, any
/// other value takes the default) and loads one table per switch through
/// `FELocale_LoadTable` (0x1959a8): ids 4..7 (`cFELocale_addFile`), 8..11,
/// then 0x10..0x13. Sets `self+0x58 = 1`. The byte is read again before each
/// switch, as the generated code does. The switch chains are transcribed
/// from the generated block structure.
#[allow(unused_labels, unreachable_code)]
pub fn fe_locale_load_fe_tables(c: &mut Cpu) {
    guest(c, |c| {
        'b_195c60: {
        'b_195c5c: {
        'b_195c58: {
        'b_195c50: {
        'b_195c48: {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V1][0] = 0x1;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sb((c.g[S1][0] as u32).wrapping_add(0x58), c.g[V1][0]);
        c.g[A0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x1));
        let t = c.g[A0][0] == c.g[V0][0];
        c.g[S0][0] = 0xffffffffffffffff;
        if t {
        break 'b_195c48;
        } else {
        'b_195c40: {
        'b_195c28: {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < 2i64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x2;
        if t {
        break 'b_195c28;
        } else {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[A0][0] = c.g[S1][0];
        if t {
        break 'b_195c40;
        } else {
        c.cycles += 2;
        let t = true;

        if t {
        break 'b_195c60;
        } else {
        break 'b_195c28;
        }
        }
        }
        }
        c.cycles += 2;
        let t = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = 0x3;
        if t {
        break 'b_195c50;
        } else {
        c.cycles += 2;
        let t = c.g[A0][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if t {
        break 'b_195c58;
        } else {
        c.cycles += 2;
        let t = true;

        if t {
        break 'b_195c60;
        } else {
        break 'b_195c40;
        }
        }
        }
        }
        c.cycles += 2;
        let t = true;
        c.g[S0][0] = 0x4;
        if t {
        break 'b_195c5c;
        } else {
        break 'b_195c48;
        }
        }
        }
        c.cycles += 2;
        let t = true;
        c.g[S0][0] = 0x5;
        if t {
        break 'b_195c5c;
        } else {
        break 'b_195c50;
        }
        }
        c.cycles += 2;
        let t = true;
        c.g[S0][0] = 0x6;
        if t {
        break 'b_195c5c;
        } else {
        break 'b_195c58;
        }
        }
        c.cycles += 1;
        c.g[S0][0] = 0x7;
        break 'b_195c5c;
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[S1][0];
        break 'b_195c60;
        }
        'b_195cc8: {
        'b_195cc4: {
        'b_195cc0: {
        'b_195cb8: {
        'b_195cb0: {
        c.cycles += 6;
        c.g[A1][0] = c.g[S0][0];
        OC1538_FE_LOCALE_ADD_FILE.call(c, 0x00195c68)?;
        c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = 0x1;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
        if t {
        break 'b_195cb0;
        } else {
        'b_195ca8: {
        'b_195c90: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x2;
        if t {
        break 'b_195c90;
        } else {
        c.cycles += 2;
        let t = c.g[V1][0] == 0u64;
        c.g[A1][0] = c.g[S0][0];
        if t {
        break 'b_195ca8;
        } else {
        c.cycles += 2;
        let t = true;

        if t {
        break 'b_195cc8;
        } else {
        break 'b_195c90;
        }
        }
        }
        }
        c.cycles += 2;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x3;
        if t {
        break 'b_195cb8;
        } else {
        c.cycles += 2;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        if t {
        break 'b_195cc0;
        } else {
        c.cycles += 2;
        let t = true;

        if t {
        break 'b_195cc8;
        } else {
        break 'b_195ca8;
        }
        }
        }
        }
        c.cycles += 2;
        let t = true;
        c.g[S0][0] = 0x8;
        if t {
        break 'b_195cc4;
        } else {
        break 'b_195cb0;
        }
        }
        }
        c.cycles += 2;
        let t = true;
        c.g[S0][0] = 0x9;
        if t {
        break 'b_195cc4;
        } else {
        break 'b_195cb8;
        }
        }
        c.cycles += 2;
        let t = true;
        c.g[S0][0] = 0xa;
        if t {
        break 'b_195cc4;
        } else {
        break 'b_195cc0;
        }
        }
        c.cycles += 1;
        c.g[S0][0] = 0xb;
        break 'b_195cc4;
        }
        c.cycles += 1;
        c.g[A1][0] = c.g[S0][0];
        break 'b_195cc8;
        }
        'b_195d34: {
        'b_195d30: {
        'b_195d2c: {
        'b_195d24: {
        'b_195d1c: {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        OC1538_FE_LOCALE_ADD_FILE.call(c, 0x00195cd0)?;
        c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = 0x1;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[S0][0] = 0xffffffffffffffff;
        if t {
        break 'b_195d1c;
        } else {
        'b_195d14: {
        'b_195cfc: {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x2;
        if t {
        break 'b_195cfc;
        } else {
        c.cycles += 2;
        let t = c.g[V1][0] == 0u64;
        c.g[A0][0] = c.g[S1][0];
        if t {
        break 'b_195d14;
        } else {
        c.cycles += 2;
        let t = true;

        if t {
        break 'b_195d34;
        } else {
        break 'b_195cfc;
        }
        }
        }
        }
        c.cycles += 2;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x3;
        if t {
        break 'b_195d24;
        } else {
        c.cycles += 2;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if t {
        break 'b_195d2c;
        } else {
        c.cycles += 2;
        let t = true;

        if t {
        break 'b_195d34;
        } else {
        break 'b_195d14;
        }
        }
        }
        }
        c.cycles += 2;
        let t = true;
        c.g[S0][0] = 0x10;
        if t {
        break 'b_195d30;
        } else {
        break 'b_195d1c;
        }
        }
        }
        c.cycles += 2;
        let t = true;
        c.g[S0][0] = 0x11;
        if t {
        break 'b_195d30;
        } else {
        break 'b_195d24;
        }
        }
        c.cycles += 2;
        let t = true;
        c.g[S0][0] = 0x12;
        if t {
        break 'b_195d30;
        } else {
        break 'b_195d2c;
        }
        }
        c.cycles += 1;
        c.g[S0][0] = 0x13;
        break 'b_195d30;
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[S1][0];
        break 'b_195d34;
        }
        c.cycles += 7;
        c.g[A1][0] = c.g[S0][0];
        OC1538_FE_LOCALE_ADD_FILE.call(c, 0x00195d3c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, machine_rt};
    use crate::difftest::{check, elf_image, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Scramble, gp, the wrap-angle constants (`gp - 0x6454 .. gp - 0x6478`)
    /// and the float at `DATA` that `a0` points at.
    fn wrap_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        for off in [
            0x6454u32, 0x6458, 0x645c, 0x6460, 0x6464, 0x6468, 0x646c, 0x6470, 0x6474, 0x6478,
        ] {
            c.bus.write32(GAME_GP.wrapping_sub(off), r.f32(4.0));
        }
        c.g[A0][0] = DATA as u64;
        let v = if seed % 3 == 0 {
            r.edge_f32()
        } else {
            r.f32(1000.0)
        };
        c.bus.write32(DATA, v);
    }

    #[test]
    fn math_wrap_angle_deg_matches() {
        check(
            ssx3_ee::s02::f_0016bda0__generated,
            math_wrap_angle_deg,
            200,
            &|c, seed| wrap_fixture(c, seed, 0x1538_0bda),
        );
    }

    #[test]
    fn math_wrap_angle_clamp_deg_matches() {
        check(
            ssx3_ee::s02::f_0016bde8__generated,
            math_wrap_angle_clamp_deg,
            200,
            &|c, seed| wrap_fixture(c, seed, 0x1538_0bde),
        );
    }

    /// The gated fake of `InputBinding_EvalBool` (0x320c48): the resume arm of
    /// `0x17fe64` calls it, and the real one runs the bytecode VM. With the
    /// gate word set it returns the forced word; otherwise the body it replaced.
    const EVAL_GATE: u32 = DATA + 0x5_1538;
    const EVAL_RESULT: u32 = DATA + 0x5_153c;
    const EVAL_MAGIC: u32 = 0x1538_e7a1;
    static EVAL_PREV: AtomicUsize = AtomicUsize::new(0);

    fn eval_fake(c: &mut Cpu) {
        if c.bus.read32(EVAL_GATE) != EVAL_MAGIC {
            return fakes::fall_back(c, &EVAL_PREV, ee::s10::f_00320c48__generated);
        }
        c.cycles += 9;
        c.g[V0][0] = c.bus.read32(EVAL_RESULT) as u64;
        ret(c);
    }

    fn hook_eval() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320C48),
            eval_fake,
            &EVAL_PREV,
        );
    }

    fn tail_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.g[A2][0] = DATA as u64;
        // The arm loads its return address from the frame the stub did not open.
        c.bus.write64(crate::difftest::SP, crate::difftest::RA as u64);
        c.bus.write32(EVAL_GATE, EVAL_MAGIC);
        c.bus.write32(EVAL_RESULT, r.next_u32());
    }

    #[test]
    fn lbl_0017fe4c_matches() {
        hook_eval();
        check(
            ssx3_ee::s02::f_0017fe4c__generated,
            lbl_0017fe4c,
            60,
            &|c, seed| tail_fixture(c, seed, 0x1538_4c00),
        );
    }

    #[test]
    fn lbl_0017fe54_matches() {
        hook_eval();
        check(
            ssx3_ee::s02::f_0017fe54__generated,
            lbl_0017fe54,
            60,
            &|c, seed| tail_fixture(c, seed, 0x1538_5400),
        );
    }

    #[test]
    fn lbl_0017fe5c_matches() {
        hook_eval();
        check(
            ssx3_ee::s02::f_0017fe5c__generated,
            lbl_0017fe5c,
            60,
            &|c, seed| tail_fixture(c, seed, 0x1538_5c00),
        );
    }

    #[test]
    fn keyboard_screen_ctor_matches() {
        check(
            ssx3_ee::s02::f_00180508__generated,
            keyboard_screen_ctor,
            60,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1538_0508, 0);
                elf_image(c);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
            },
        );
    }

    /// The gated fake of `FELocale_LoadTable` (0x1959a8): logs the call
    /// through the fake machinery when the gate word is set.
    const LOADTABLE_GATE: u32 = DATA + 0x5_1600;
    const LOADTABLE_MAGIC: u32 = 0x1539_7ab1;
    static LOADTABLE_PREV: AtomicUsize = AtomicUsize::new(0);

    fn loadtable_fake(c: &mut Cpu) {
        if c.bus.read32(LOADTABLE_GATE) != LOADTABLE_MAGIC {
            return fakes::fall_back(c, &LOADTABLE_PREV, ee::s02::f_001959a8__generated);
        }
        fakes::fake_call(c, 0x0019_59a8);
    }

    #[test]
    fn fe_locale_load_fe_tables_matches() {
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_001959A8),
            loadtable_fake,
            &LOADTABLE_PREV,
        );
        check(
            ssx3_ee::s02::f_00195be0__generated,
            fe_locale_load_fe_tables,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1539_0195, 0);
                c.bus.write32(LOADTABLE_GATE, LOADTABLE_MAGIC);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                // Language byte: 0..3 select a table, the rest take the default.
                let lang = [0u8, 1, 2, 3, 4, 0xff][(seed % 6) as usize];
                c.bus.write8(DATA + 0x1001, lang);
            },
        );
    }
}
