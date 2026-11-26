//! Staging module oc728 (wave wave240): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc728`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002e_2860, lensfx_flare_ctor_stub, "LensFX_FlareCtorStub"),
    (0x0039_c428, cuiprogress_set_value, "cUIProgress_SetValue"),
];

/// `LensFX_FlareCtorStub` (0x2e2860): a constructor that does nothing and
/// returns its `this` pointer.
pub fn lensfx_flare_ctor_stub(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// cUIProgress field offsets.
pub(crate) const CUIPROGRESS_ATTR: u32 = 0x14;
pub(crate) const CUIPROGRESS_BACKGROUND: u32 = 0x74;
pub(crate) const CUIPROGRESS_KNOB: u32 = 0x78;
pub(crate) const CUIPROGRESS_VALUE: u32 = 0x7c;
pub(crate) const CUIPROGRESS_NUM_VALUES: u32 = 0x80;

/// `cUIProgress_SetValue` (0x39c428): stores the value, reads the background
/// widget's size (vtable slot 0xa0, or the zero vector `0x4ff0d8` copy when
/// there is no background) and scales its size
/// by value / numValues (x when attr bit 0 is clear, y when set), then hands that size to the knob widget (vtable
/// slot 0xa8). No knob or no numValues: nothing is set.
pub fn cuiprogress_set_value(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0050_0000;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        let a1 = c.g[A1][0];
        c.sw(this.wrapping_add(CUIPROGRESS_VALUE), a1);
        c.g[A2][0] = c.lw(this.wrapping_add(CUIPROGRESS_BACKGROUND)) as u64;
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
        let zero = c.g[A3][0] as u32;
        c.ldl(3, zero.wrapping_add(0x7));
        c.ldr(3, zero);
        c.g[A0][0] = c.lw(zero.wrapping_add(0x8));
        let buf = sp(c);
        c.sdl(3, buf.wrapping_add(0x7));
        c.sdr(3, buf);
        let a0 = c.g[A0][0];
        c.sw(buf.wrapping_add(0x8), a0);
        c.g[A1][0] = c.g[SP][0];
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            let bg = c.g[A2][0] as u32;
            c.g[V0][0] = c.lw(bg.wrapping_add(0x8));
            let vt = c.g[V0][0] as u32;
            c.g[A0][0] = c.lh(vt.wrapping_add(0xa0));
            c.g[V1][0] = c.lw(vt.wrapping_add(0xa4));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(bg.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0039_c47c)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CUIPROGRESS_KNOB));
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CUIPROGRESS_NUM_VALUES));
            if c.g[V1][0] == 0 {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 5;
                let this = c.g[S0][0] as u32;
                c.g[V0][0] = c.lw(this.wrapping_add(CUIPROGRESS_ATTR));
                c.g[V0][0] ^= 0x1;
                c.g[V0][0] &= 0x1;
                let y_axis = c.g[V0][0] == 0;
                let buf = sp(c);
                c.lwc1(1, buf);
                if !y_axis {
                    c.cycles += 8;
                    c.lwc1(0, this.wrapping_add(CUIPROGRESS_VALUE));
                    c.cvt_s_w(0, 0);
                    c.mtc1(3, 2);
                    c.cvt_s_w(2, 2);
                    c.div_s(0, 0, 2);
                    c.mul_s(1, 1, 0);
                    c.swc1(1, buf);
                } else {
                    c.cycles += 8;
                    c.lwc1(0, this.wrapping_add(CUIPROGRESS_VALUE));
                    c.cvt_s_w(0, 0);
                    c.lwc1(1, buf.wrapping_add(0x4));
                    c.mtc1(3, 2);
                    c.cvt_s_w(2, 2);
                    c.div_s(0, 0, 2);
                    c.mul_s(1, 1, 0);
                    c.swc1(1, buf.wrapping_add(0x4));
                }
                c.cycles += 8;
                c.g[A2][0] = c.lw(this.wrapping_add(CUIPROGRESS_KNOB)) as u64;
                c.g[A1][0] = c.g[SP][0];
                let knob = c.g[A2][0] as u32;
                c.g[V0][0] = c.lw(knob.wrapping_add(0x8));
                let vt = c.g[V0][0] as u32;
                c.g[A0][0] = c.lh(vt.wrapping_add(0xa8));
                c.g[V1][0] = c.lw(vt.wrapping_add(0xac));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(knob.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0039_c504)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// The flare constructor stub returns `this`.
    #[test]
    fn lensfx_flare_ctor_stub_matches() {
        check(
            ee::s09::f_002e2860__generated,
            lensfx_flare_ctor_stub,
            20,
            &|c, seed| {
                let mut r = Rng(0x2e28_60 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    /// SetValue with and without background and knob, both bar directions,
    /// zero and non-zero numValues; the widget calls are fakes (some unwind).
    #[test]
    fn cuiprogress_set_value_matches() {
        covered(
            ee::s13::f_0039c428__generated,
            cuiprogress_set_value,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39c428, 2);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (r.next_u32() % 50) as u64;
                let bg = if seed % 5 == 0 { 0 } else { DATA + 0x2000 };
                let knob = if seed % 7 == 0 { 0 } else { DATA + 0x3000 };
                c.bus.write32(this + CUIPROGRESS_ATTR, r.next_u32());
                c.bus.write32(this + CUIPROGRESS_BACKGROUND, bg);
                c.bus.write32(this + CUIPROGRESS_KNOB, knob);
                c.bus.write32(
                    this + CUIPROGRESS_NUM_VALUES,
                    if seed % 3 == 0 { 0 } else { r.next_u32() % 40 },
                );
                let vt = fakes::vtable(c, &mut r, &[0xa0, 0xa8]);
                c.bus.write32(DATA + 0x2008, vt);
                c.bus.write32(DATA + 0x3008, vt);
                // The zero vector the size buffer is initialised from.
                let zero = 0x0050_0000u32 - 0xf28;
                for i in 0..3 {
                    c.bus.write32(zero + 4 * i, r.f32(100.0));
                }
            },
        );
    }
}
