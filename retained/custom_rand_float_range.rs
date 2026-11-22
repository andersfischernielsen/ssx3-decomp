//! Native AIrandf / Rand_FloatRange (0x00317830), retained uninstalled.
//!
//! Generated EE is authoritative. No automated tests, builds or replays were
//! added or run by this writer, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, rand_float_range, "Rand_FloatRange")];

const ENTRY: u32 = 0x0031_7830;
const RETURN_FROM_RANDOM: u32 = 0x0031_784c;
const END: u32 = 0x0031_7890;

/// `float AIrandf(float lower /*f12*/, float upper /*f13*/) -> f0`.
/// Consume one value from the existing game RNG through Rand_NextGame.
/// Its low 23 bits form a uniformly spaced mantissa: `u = bits/2^23`.
/// Return `lower + (upper-lower)*u` with the original separate PS2 FPU
/// rounding steps. For [0,1] this is exactly [0,1); arbitrary bounds are
/// neither sorted nor validated. Keep native scratch registers and saved
/// f20/f21, including the native return continuation if its callee yields.
pub fn rand_float_range(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        match c.pc {
            ENTRY => {
                c.cycles += 24;
                Frame::open(c, 0x20);
                frame.save_fpr(c, 21, 0x18);
                frame.save_fpr(c, 20, 0x10);
                c.f[21] = c.f[12];
                frame.save_ra(c, 0);
                c.f[20] = c.f[13];
                call_ptr(c, 0x0031_7810, RETURN_FROM_RANDOM)?;
            }
            RETURN_FROM_RANDOM => {}
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        }

        c.g[V1][0] = 0x007f_0000;
        c.g[A0][0] = 0x3f80_0000;
        c.g[V1][0] |= 0xffff;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[V0][0] &= c.g[V1][0];
        c.f[20] = (FpuF32(c.f[20]) - FpuF32(c.f[21])).0;
        c.g[V0][0] |= c.g[A0][0];
        frame.restore_ra(c, 0);
        c.mtc1(V0, 1);
        c.f[0] = (FpuF32(c.f[1]) - FpuF32(c.f[0])).0;
        c.f[20] = (FpuF32(c.f[20]) * FpuF32(c.f[0])).0;
        c.f[0] = (FpuF32(c.f[21]) + FpuF32(c.f[20])).0;
        frame.restore_fpr(c, 21, 0x18);
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    });
}
