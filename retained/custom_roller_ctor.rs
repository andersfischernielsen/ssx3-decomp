//! Native RollerModifier_ctor (0x0035da70), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, cargo, game or replays were
//! added or run, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, roller_modifier_ctor, "RollerModifier_ctor")];

const ENTRY: u32 = 0x0035_da70;
const AFTER_MEMBER_CTOR: u32 = 0x0035_dac4;
const AFTER_SHAPE_CTOR: u32 = 0x0035_dad8;
const AFTER_SHAPE_COPY: u32 = 0x0035_daf0;
const AFTER_FRAME: u32 = 0x0035_dc08;
const AFTER_POSE: u32 = 0x0035_dcac;
const AFTER_INERTIA: u32 = 0x0035_dd3c;
const AFTER_MOMENTUM: u32 = 0x0035_dd44;
const AFTER_INIT_BODY: u32 = 0x0035_ddb8;
const END: u32 = 0x0035_dde8;

const BODY: u32 = 0x30;
const SHAPE: u32 = 0xe0;
const INSTANCE: u32 = 0xd0;
const INVERSE_MASS: u32 = 0x70;
const LOCAL_CENTRE: u32 = 0x2b0;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

#[inline]
fn half(c: &mut Cpu, register: usize) {
    c.f[register] = (FpuF32(c.f[register]) * FpuF32(c.f[3])).0;
}

/// `RollerModifier* RollerModifier_ctor(self, desc, contactA, contactB)`.
/// Preserve the native 0x2d0-byte modifier/body layout and original VU pose
/// kernels. The instance comes from contactA+0x50; the copied shape is at
/// contactA+0x48. desc+4 supplies the scalar stored at self+8 and its inverse
/// at self+0x70; desc+8 is copied to self+0xc. contactB and contactA are then
/// forwarded unchanged to InitBody. Their remaining interpretation is not
/// inferred here. Every native-call continuation resumes the same frame.
pub fn roller_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xe0 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 221;
                    Frame::open(c, 0xe0);
                    c.g[V0][0] = 0x490000;
                    frame.save(c, S0, 0xd0);
                    c.g[V0][0] = sx(at(c, V0, 0x13f8));
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, S5, 0x80);
                    frame.save(c, S1, 0xc0);
                    c.g[S5][0] = sx(at(c, S0, BODY));
                    frame.save(c, S2, 0xb0);
                    c.g[A0][0] = c.g[S5][0];
                    frame.save(c, S3, 0xa0);
                    c.g[S2][0] = c.g[A1][0];
                    frame.save(c, S4, 0x90);
                    c.g[S1][0] = sx(at(c, S0, SHAPE));
                    frame.save(c, S6, 0x70);
                    c.g[S4][0] = c.g[A2][0];
                    frame.save_ra(c, 0x60);
                    c.g[S6][0] = c.g[A3][0];
                    c.sw(r(c, S0), c.g[V0][0]);
                    c.g[S3][0] = sx(at(c, S0, 4));
                    call_ptr(c, 0x0035_cfe8, AFTER_MEMBER_CTOR)?;
                    pc = AFTER_MEMBER_CTOR;
                }
                AFTER_MEMBER_CTOR => {
                    c.g[V0][0] = 0x490000;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(0xf80));
                    c.sw(r(c, S0), c.g[V0][0]);
                    call_ptr(c, 0x0032_c508, AFTER_SHAPE_CTOR)?;
                    pc = AFTER_SHAPE_CTOR;
                }
                AFTER_SHAPE_CTOR => {
                    c.sw(at(c, S0, 0x20), 0);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw(at(c, S4, 0x50));
                    c.sw(at(c, S0, INSTANCE), c.g[V0][0]);
                    c.g[A1][0] = c.lw(at(c, S4, 0x48));
                    call_ptr(c, 0x0032_7cc8, AFTER_SHAPE_COPY)?;
                    pc = AFTER_SHAPE_COPY;
                }
                AFTER_SHAPE_COPY => {
                    c.lwc1(0, at(c, S2, 4));
                    c.g[A0][0] = sx(at(c, SP, 0x10));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(AT, 1);
                    c.swc1(0, at(c, S0, 8));
                    c.g[V1][0] = c.lw(at(c, S0, 0x178));
                    c.lwc1(2, at(c, S2, 8));
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(AT, 3);
                    c.swc1(2, at(c, S0, 0xc));
                    c.lwc1(0, at(c, S3, 4));
                    c.f[1] = FpuF32(c.f[1]).div(FpuF32(c.f[0])).0;
                    c.swc1(1, at(c, S0, INVERSE_MASS));
                    c.lwc1(0, at(c, V1, 0x2c));
                    c.swc1(0, r(c, SP));
                    c.lwc1(1, at(c, V1, 0x30));
                    c.swc1(1, at(c, SP, 4));
                    c.lwc1(0, at(c, V1, 0x34));
                    c.sw(at(c, SP, 0xc), 0);
                    c.swc1(0, at(c, SP, 8));
                    c.lq(V0, r(c, SP));
                    c.sq(V0, at(c, S0, LOCAL_CENTRE));
                    c.ldl(V0, at(c, V1, 0x63));
                    c.ldr(V0, at(c, V1, 0x5c));
                    c.ldl(A1, at(c, V1, 0x6b));
                    c.ldr(A1, at(c, V1, 0x64));
                    c.ldl(A2, at(c, V1, 0x73));
                    c.ldr(A2, at(c, V1, 0x6c));
                    c.ldl(A3, at(c, V1, 0x7b));
                    c.ldr(A3, at(c, V1, 0x74));
                    c.sdl(V0, at(c, S0, 0x287));
                    c.sdr(V0, at(c, S0, 0x280));
                    c.sdl(A1, at(c, S0, 0x28f));
                    c.sdr(A1, at(c, S0, 0x288));
                    c.sdl(A2, at(c, S0, 0x297));
                    c.sdr(A2, at(c, S0, 0x290));
                    c.sdl(A3, at(c, S0, 0x29f));
                    c.sdr(A3, at(c, S0, 0x298));
                    c.g[V0][0] = c.lw(at(c, V1, 0x7c));
                    c.sw(at(c, S0, 0x2a0), c.g[V0][0]);
                    c.lwc1(5, at(c, S0, 0x280));
                    c.lwc1(7, at(c, S0, 0x284));
                    c.lwc1(8, at(c, S0, 0x288));
                    half(c, 5);
                    c.lwc1(6, at(c, S0, 0x28c));
                    half(c, 7);
                    c.lwc1(4, at(c, S0, 0x290));
                    half(c, 8);
                    c.lwc1(2, at(c, S0, 0x294));
                    half(c, 6);
                    c.lwc1(0, at(c, S0, 0x298));
                    half(c, 4);
                    c.lwc1(1, at(c, S0, 0x29c));
                    half(c, 2);
                    half(c, 0);
                    c.swc1(5, at(c, S0, 0x280));
                    half(c, 1);
                    c.swc1(7, at(c, S0, 0x284));
                    c.swc1(8, at(c, S0, 0x288));
                    c.swc1(6, at(c, S0, 0x28c));
                    c.swc1(4, at(c, S0, 0x290));
                    c.swc1(2, at(c, S0, 0x294));
                    c.swc1(0, at(c, S0, 0x298));
                    c.swc1(1, at(c, S0, 0x29c));
                    c.lwc1(0, at(c, S0, 0x2a0));
                    c.g[A1][0] = c.lw(at(c, S0, INSTANCE));
                    half(c, 0);
                    c.g[A1][0] = sx(at(c, A1, 0x10));
                    c.swc1(0, at(c, S0, 0x2a0));
                    call_ptr(c, 0x0031_b748, AFTER_FRAME)?;
                    pc = AFTER_FRAME;
                }
                AFTER_FRAME => {
                    c.lqc2(4, at(c, SP, 0x20));
                    c.lqc2(5, at(c, S0, LOCAL_CENTRE));
                    c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().opmula(4, 6);
                    c.vu0().opmsub(7, 6, 4);
                    c.vu0().mula(0b1110, 5, Src::Bc(0, W));
                    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                    c.vu0().madda(0b1110, 7, Src::Bc(0, W));
                    c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
                    c.sqc2(8, at(c, SP, 0x30));
                    c.g[A0][0] = sx(at(c, SP, 0x30));
                    c.lq(V0, at(c, SP, 0x30));
                    c.g[A1][0] = c.g[S0][0];
                    c.sq(V0, r(c, SP));
                    c.lqc2(4, at(c, SP, 0x20));
                    c.lqc2(5, r(c, SP));
                    c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().opmula(4, 6);
                    c.vu0().opmsub(7, 6, 4);
                    c.vu0().mula(0b1110, 5, Src::Bc(0, W));
                    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                    c.vu0().madda(0b1110, 7, Src::Bc(0, W));
                    c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
                    c.sqc2(8, at(c, SP, 0x50));
                    c.lq(V0, at(c, SP, 0x50));
                    c.sq(V0, at(c, SP, 0x40));
                    c.lqc2(3, at(c, SP, 0x10));
                    c.lqc2(4, at(c, SP, 0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x10));
                    c.lq(V0, at(c, SP, 0x10));
                    c.lq(V1, at(c, SP, 0x20));
                    c.sq(V0, at(c, S0, BODY));
                    c.sq(V1, at(c, S0, 0x40));
                    call_ptr(c, 0x0035_e770, AFTER_POSE)?;
                    pc = AFTER_POSE;
                }
                AFTER_POSE => {
                    c.g[V0][0] = sx(at(c, S0, 0x240));
                    c.lqc2(4, at(c, SP, 0x40));
                    c.lqc2(3, at(c, SP, 0x30));
                    c.vu0().add(0b1110, 1, 0, Src::Bc(0, W));
                    c.vu0().add(0b1110, 5, 4, Src::Vec(4));
                    c.vu0().sub(0b0001, 10, 10, Src::Vec(10));
                    c.vu0().sub(0b0001, 11, 11, Src::Vec(11));
                    c.vu0().sub(0b0001, 12, 12, Src::Vec(12));
                    c.vu0().mul(0b1110, 6, 5, Src::Vec(4));
                    c.vu0().mul(0b1110, 7, 5, Src::Bc(4, W));
                    c.vu0().opmula(5, 4);
                    c.vu0().madd(0b1110, 8, 0, Src::Vec(0));
                    c.vu0().suba(0b1000, 1, Src::Bc(6, Y));
                    c.vu0().msub(0b1000, 10, 1, Src::Bc(6, Z));
                    c.vu0().suba(0b0100, 1, Src::Bc(6, Z));
                    c.vu0().msub(0b0100, 11, 1, Src::Bc(6, X));
                    c.vu0().suba(0b0010, 1, Src::Bc(6, X));
                    c.vu0().msub(0b0010, 12, 1, Src::Bc(6, Y));
                    c.vu0().adda(0b0100, 0, Src::Bc(8, Z));
                    c.vu0().madd(0b0100, 10, 1, Src::Bc(7, Z));
                    c.vu0().adda(0b0010, 0, Src::Bc(8, X));
                    c.vu0().madd(0b0010, 11, 1, Src::Bc(7, X));
                    c.vu0().adda(0b0100, 0, Src::Bc(8, X));
                    c.vu0().msub(0b0100, 12, 1, Src::Bc(7, X));
                    c.vu0().adda(0b0010, 0, Src::Bc(8, Y));
                    c.vu0().msub(0b0010, 10, 1, Src::Bc(7, Y));
                    c.vu0().adda(0b1000, 0, Src::Bc(8, Z));
                    c.vu0().msub(0b1000, 11, 1, Src::Bc(7, Z));
                    c.vu0().adda(0b1000, 0, Src::Bc(8, Y));
                    c.vu0().madd(0b1000, 12, 1, Src::Bc(7, Y));
                    c.sqc2(3, at(c, V0, 0x30));
                    c.sqc2(10, r(c, V0));
                    c.sqc2(11, at(c, V0, 0x10));
                    c.sqc2(12, at(c, V0, 0x20));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0035_e248, AFTER_INERTIA)?;
                    pc = AFTER_INERTIA;
                }
                AFTER_INERTIA => {
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x0035_d288, AFTER_MOMENTUM)?;
                    pc = AFTER_MOMENTUM;
                }
                AFTER_MOMENTUM => {
                    c.lwc1(0, r(c, GP).wrapping_sub(0x29fc));
                    c.g[A3][0] = 0xffff_ffff_ffff_0000;
                    c.lwc1(1, r(c, GP).wrapping_sub(0x29f8));
                    c.g[A3][0] |= 0xffdf;
                    c.lwc1(2, r(c, GP).wrapping_sub(0x29f4));
                    c.g[T0][0] = 0xffff_ffff_ffff_0000;
                    c.swc1(0, at(c, S0, 0x14));
                    c.g[T0][0] |= 0xfffd;
                    c.swc1(1, at(c, S0, 0x18));
                    c.g[A1][0] = c.g[S6][0];
                    c.swc1(2, at(c, S0, 0x1c));
                    c.g[A2][0] = c.g[S4][0];
                    c.sw(at(c, S0, 0x10), 0);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V1][0] = c.lw(at(c, S0, INSTANCE));
                    c.g[V0][0] = c.lw(at(c, V1, 8));
                    c.g[V0][0] &= c.g[A3][0];
                    c.g[V0][0] |= 0x40;
                    c.sw(at(c, V1, 8), c.g[V0][0]);
                    c.g[A3][0] = c.lw(at(c, S0, INSTANCE));
                    c.g[V0][0] = c.lw(at(c, A3, 8));
                    c.g[V0][0] &= c.g[T0][0];
                    c.g[V0][0] |= 4;
                    c.sw(at(c, A3, 8), c.g[V0][0]);
                    c.g[V1][0] = c.lw(at(c, S0, INSTANCE));
                    c.g[V0][0] = c.lw(at(c, V1, 8));
                    c.g[V0][0] |= 1;
                    c.sw(at(c, V1, 8), c.g[V0][0]);
                    call_ptr(c, 0x0035_dde8, AFTER_INIT_BODY)?;
                    pc = AFTER_INIT_BODY;
                }
                AFTER_INIT_BODY => {
                    c.g[V0][0] = c.g[S0][0];
                    frame.restore(c, S1, 0xc0);
                    frame.restore(c, S0, 0xd0);
                    frame.restore(c, S2, 0xb0);
                    frame.restore(c, S3, 0xa0);
                    frame.restore(c, S4, 0x90);
                    frame.restore(c, S5, 0x80);
                    frame.restore(c, S6, 0x70);
                    frame.restore_ra(c, 0x60);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
