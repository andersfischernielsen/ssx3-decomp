//! Native RollerModifier_InitBody (0x0035dde8), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, cargo, game or replays were
//! added or run, in accordance with the user's instruction.

use crate::abi;
use crate::call::{call_ptr, guest, Frame};
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    roller_modifier_init_body,
    "RollerModifier_InitBody",
)];

const ENTRY: u32 = 0x0035_dde8;
const AFTER_ANGULAR_MOMENTUM: u32 = 0x0035_df0c;
const END: u32 = 0x0035_df70;
const MASS: u32 = 8;
const ACTIVITY: u32 = 4;
const BODY: u32 = 0x30;
const INVERSE_MASS: u32 = 0x70;
const LINEAR_MOMENTUM: u32 = 0x50;
const ANGULAR_MOMENTUM: u32 = 0x60;
const LINEAR_VELOCITY: u32 = 0xb0;
const ANGULAR_VELOCITY: u32 = 0xc0;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void RollerModifier_InitBody(self, contactB, contactA)`.
/// Only contactB is consumed here. The native initial linear vector is
/// `contactB[+10] * contactB[+30] / 2 + (0,0,min(600,5000/mass+100),0)`.
/// The 5000 constant is loaded from native gp-29f0. Preserve all four VU
/// lanes, rather than recomputing that description with host floats.
/// Store its momentum-scaled and velocity forms at +50/+b0, then pass
/// `contactB[+20] * 10` to the existing angular-momentum helper on self+30.
/// No independent named launch-vector field is read from contactA or desc.
pub fn roller_modifier_init_body(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x90 };
        match c.pc {
            ENTRY => {
                c.cycles += 17;
                Frame::open(c, 0x90);
                c.lwc1(0, r(c, GP).wrapping_sub(0x29f0));
                frame.save(c, S0, 0x80);
                c.g[A3][0] = c.g[A1][0];
                frame.save_ra(c, 0x70);
                c.g[S0][0] = c.g[A0][0];
                c.g[AT][0] = 0x42c8_0000; // 100
                c.mtc1(AT, 2);
                c.lwc1(1, at(c, S0, MASS));
                c.f[0] = FpuF32(c.f[0]).div(FpuF32(c.f[1])).0;
                c.g[AT][0] = 0x4416_0000; // 600
                c.mtc1(AT, 1);
                c.f[3] = (FpuF32(c.f[0]) + FpuF32(c.f[2])).0;
                abi::set_fcc(c, FpuF32(c.f[1]).lt(FpuF32(c.f[3])));
                c.sw(at(c, S0, 0x28), 0);
                if c.fcc() {
                    c.cycles += 1;
                    c.f[3] = c.f[1];
                }

                c.cycles += 79;
                // The linear kick is the source contact vector scaled by
                // its scalar, halved, then given the native vertical bias.
                c.lwc1(0, at(c, A3, 0x30));
                c.g[V1][0] = sx(at(c, S0, BODY));
                c.mfc1(V0, 0);
                c.lqc2(4, at(c, A3, 0x10));
                c.qmtc2(V0, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x30));
                c.g[AT][0] = 0x4000_0000; // 2
                c.mtc1(AT, 1);
                c.lq(V0, at(c, SP, 0x30));
                c.g[A0][0] = c.g[V1][0];
                c.g[AT][0] = 0x4120_0000; // 10
                c.mtc1(AT, 2);
                c.g[A1][0] = c.g[SP][0];
                c.sq(V0, at(c, SP, 0x20));
                c.mfc1(A2, 1);
                c.qmtc2(A2, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, at(c, SP, 0x20));
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, at(c, SP, 0x30));
                c.lq(V0, at(c, SP, 0x30));
                c.swc1(3, at(c, SP, 0x38));
                c.sq(V0, at(c, SP, 0x10));
                c.sw(at(c, SP, 0x30), 0);
                c.sw(at(c, SP, 0x34), 0);
                c.sw(at(c, SP, 0x3c), 0);
                c.lqc2(3, at(c, SP, 0x10));
                c.lqc2(4, at(c, SP, 0x30));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x40));
                c.lq(V0, at(c, SP, 0x40));
                c.sq(V0, r(c, SP));
                c.lwc1(0, at(c, V1, INVERSE_MASS - BODY));
                c.mfc1(V0, 0);
                c.qmtc2(V0, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, r(c, SP));
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, at(c, SP, 0x60));
                c.lq(V1, at(c, SP, 0x60));
                c.lq(V0, r(c, SP));
                c.sq(V1, at(c, S0, LINEAR_MOMENTUM));
                c.sq(V0, at(c, S0, LINEAR_VELOCITY));
                c.sq(V1, at(c, SP, 0x50));
                // This overwrites the incoming contactA register. Its
                // original value has never been dereferenced by this body.
                c.mfc1(A2, 2);
                c.lqc2(4, at(c, A3, 0x20));
                c.qmtc2(A2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x10));
                c.lq(V0, at(c, SP, 0x10));
                c.g[RA][0] = sx(AFTER_ANGULAR_MOMENTUM);
                c.sq(V0, r(c, SP));
                call_ptr(c, 0x0035_cff0, AFTER_ANGULAR_MOMENTUM)?;
            }
            AFTER_ANGULAR_MOMENTUM => {}
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        }

        // Native VU dot reductions retain their original lane/ACC effects.
        c.lqc2(3, at(c, S0, LINEAR_VELOCITY));
        c.lqc2(5, at(c, S0, LINEAR_MOMENTUM));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(V0, 4);
        c.mtc1(V0, 0);
        c.lqc2(3, at(c, S0, ANGULAR_VELOCITY));
        c.lqc2(5, at(c, S0, ANGULAR_MOMENTUM));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(V1, 4);
        c.mtc1(V1, 1);
        c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
        frame.restore_ra(c, 0x70);
        c.swc1(0, at(c, S0, ACTIVITY));
        frame.restore(c, S0, 0x80);
        frame.close(c);
        Ok(())
    });
}
