//! Native volume/object collision dispatch (0x00334888), retained uninstalled.
//!
//! Generated EE owns registers, cycles, calls and VU macro order. No tests,
//! builds, replays or collision-policy changes were made by this writer.

use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, collide_volume_object, "Collide_VolumeObject")];

const ENTRY: u32 = 0x0033_4888;
const END: u32 = 0x0033_5124;
const FRAME_BYTES: u32 = 0x27d0;
const INSTANCE_NODE: u32 = 0xc;
const INSTANCE_MODEL: u32 = 0x80;
const INSTANCE_SCALE: u32 = 0x84;
const INSTANCE_PROPERTY: u32 = 0x88;
// Addressed after adding partIndex * 12 to the property base.
const PART_SURFACE: u32 = 0x1a;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `int Collide_VolumeObject(volume, instance, hits, capacity, queryMode)`.
/// Delegate to a native actor when it handles queries, otherwise traverse the
/// instance's model parts using its static/animated intersect transforms.
/// Property mode 3 queries native sphere trees; mesh modes query their face
/// lists. Each 0x80-byte result keeps the source instance and part, normal,
/// position, surface and optional sphere body. Property mass is not read.
///
/// Query mode 2 accepts solid surfaces, mode 1 accepts only signed surface -1;
/// other modes accept both. This distinction is independent of the property's
/// collision-mode enum. S5 is model-part index, S6 collider index, S7 hit count,
/// S1 instance and S2 volume throughout the part loop. The full dispatcher
/// retains every call-return and back-edge resume point in the native body.
pub fn collide_volume_object(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME_BYTES };
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0033_4888 => {
                    c.cycles += 20;
                    Frame::open(c, FRAME_BYTES);
                    frame.save(c, 16, 0x27b0);
                    frame.save(c, 17, 0x27a0);
                    c.g[S0][0] = c.g[A3][0];
                    frame.save(c, 18, 0x2790);
                    c.g[S1][0] = c.g[A1][0];
                    frame.save(c, 19, 0x2780);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, 20, 0x2770);
                    frame.save(c, 21, 0x2760);
                    frame.save(c, 22, 0x2750);
                    frame.save(c, 23, 0x2740);
                    frame.save(c, 30, 0x2730);
                    frame.save_ra(c, 0x2720);
                    c.swc1(21, at(c, SP, 0x27c8));
                    c.swc1(20, at(c, SP, 0x27c0));
                    c.g[A1][0] = c.lw(at(c, S1, INSTANCE_NODE));
                    c.sw(at(c, SP, 0x26f0), c.g[A2][0]);
                    let t = c.g[A1][0] == 0u64;
                    c.sw(at(c, SP, 0x26f4), c.g[T0][0]);
                    if t {
                        pc = 0x0033_4920;
                        continue;
                    }
                    pc = 0x0033_48d8;
                    continue;
                }
                0x0033_48d8 => {
                    // A node may handle the whole query through its native virtual interface.
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(at(c, A1, 0xc));
                    c.g[A0][0] = c.lh(at(c, V1, 0x130));
                    c.g[V0][0] = c.lw(at(c, V1, 0x134));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0033_48ec);
                    c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                    call_ptr(c, t, 0x0033_48ec)?;
                    pc = 0x0033_48ec;
                    continue;
                }
                0x0033_48ec => {
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S2][0];
                    if t {
                        pc = 0x0033_4920;
                        continue;
                    }
                    pc = 0x0033_48f4;
                    continue;
                }
                0x0033_48f4 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.lw(at(c, S1, INSTANCE_NODE));
                    c.g[A2][0] = c.lw(at(c, SP, 0x26f0));
                    c.g[A3][0] = c.g[S0][0];
                    c.g[V1][0] = c.lw(at(c, T1, 0xc));
                    c.g[T0][0] = c.lw(at(c, SP, 0x26f4));
                    c.g[A0][0] = c.lh(at(c, V1, 0x138));
                    c.g[V0][0] = c.lw(at(c, V1, 0x13c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0033_4918);
                    c.g[A0][0] = sx(r(c, T1).wrapping_add(r(c, A0)));
                    call_ptr(c, t, 0x0033_4918)?;
                    pc = 0x0033_4918;
                    continue;
                }
                0x0033_4918 => {
                    frame.restore(c, 16, 0x27b0);
                    pc = 0x0033_50f0;
                    continue;
                }
                0x0033_4920 => {
                    // Otherwise process static property modes 1 (mesh), 2 and 3 (sphere tree).
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(at(c, S1, INSTANCE_PROPERTY));
                    c.g[V1][0] = 0x3;
                    c.g[FP][0] = c.lw(r(c, V0));
                    let t = c.g[FP][0] == c.g[V1][0];
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x0033_4944;
                        continue;
                    }
                    pc = 0x0033_4934;
                    continue;
                }
                0x0033_4934 => {
                    c.cycles += 2;
                    let t = c.g[FP][0] == c.g[V0][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x0033_4944;
                        continue;
                    }
                    pc = 0x0033_493c;
                    continue;
                }
                0x0033_493c => {
                    c.cycles += 2;
                    let t = c.g[FP][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0033_50ec;
                        continue;
                    }
                    pc = 0x0033_4944;
                    continue;
                }
                0x0033_4944 => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw(at(c, S1, INSTANCE_NODE));
                    c.g[V1][0] = 0x17;
                    c.g[V0][0] = sx(at(c, S1, 0x10));
                    c.g[A0][0] = 0xffffffffffffffff;
                    pc = 0x0033_4958;
                    continue;
                }
                0x0033_4958 => {
                    c.cycles += 7;
                    c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x1));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[S3][0] = c.g[SP][0];
                    if t {
                        if back_edge_unwinds(c, 0x0033_4958) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0033_4958;
                        continue;
                    }
                    pc = 0x0033_4974;
                    continue;
                }
                0x0033_4974 => {
                    c.cycles += 4;
                    c.sw(at(c, SP, 0x26f8), c.g[V0][0]);
                    c.g[V0][0] = 0x1;
                    let t = c.g[S0][0] == 0u64;
                    c.sw(at(c, SP, 0x26fc), c.g[V0][0]);
                    if t {
                        pc = 0x0033_49d8;
                        continue;
                    }
                    pc = 0x0033_4984;
                    continue;
                }
                0x0033_4984 => {
                    // Native actors supply either all part matrices or one intersect matrix.
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(at(c, S0, 0xc));
                    c.g[A0][0] = c.lh(at(c, V1, 0xd0));
                    c.g[V0][0] = c.lw(at(c, V1, 0xd4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0033_4998);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, t, 0x0033_4998)?;
                    pc = 0x0033_4998;
                    continue;
                }
                0x0033_4998 => {
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x0033_49c0;
                        continue;
                    }
                    pc = 0x0033_49a0;
                    continue;
                }
                0x0033_49a0 => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(at(c, S0, 0xc));
                    c.sw(at(c, SP, 0x26fc), 0u64);
                    c.g[A0][0] = c.lh(at(c, V1, 0xe0));
                    c.g[V0][0] = c.lw(at(c, V1, 0xe4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0033_49b8);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, t, 0x0033_49b8)?;
                    pc = 0x0033_49b8;
                    continue;
                }
                0x0033_49b8 => {
                    c.g[S3][0] = c.g[V0][0];
                    pc = 0x0033_49d8;
                    continue;
                }
                0x0033_49c0 => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw(at(c, S0, 0xc));
                    c.g[A0][0] = c.lh(at(c, V1, 0xc8));
                    c.g[V0][0] = c.lw(at(c, V1, 0xcc));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0033_49d4);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, t, 0x0033_49d4)?;
                    pc = 0x0033_49d4;
                    continue;
                }
                0x0033_49d4 => {
                    c.sw(at(c, SP, 0x26f8), c.g[V0][0]);
                    pc = 0x0033_49d8;
                    continue;
                }
                0x0033_49d8 => {
                    c.cycles += 16;
                    c.g[V1][0] = c.lw(at(c, S1, INSTANCE_MODEL));
                    c.g[S4][0] = c.g[S3][0];
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 12);
                    c.g[S5][0] = 0u64;
                    c.sw(at(c, SP, 0x2700), c.g[V1][0]);
                    c.g[S6][0] = 0u64;
                    c.lwc1(0, at(c, S1, INSTANCE_SCALE));
                    c.g[S0][0] = c.lw(at(c, V1, 0x8));
                    c.swc1(0, at(c, SP, 0x608));
                    c.swc1(0, at(c, SP, 0x600));
                    c.swc1(0, at(c, SP, 0x604));
                    c.swc1(12, at(c, SP, 0x60c));
                    c.g[V0][0] = c.lw(at(c, V1, 0x4));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S7][0] = 0u64;
                    if t {
                        pc = 0x0033_50e8;
                        continue;
                    }
                    pc = 0x0033_4a18;
                    continue;
                }
                0x0033_4a18 => {
                    c.cycles += 10;
                    c.g[A0][0] = sx(at(c, SP, 0x610));
                    c.g[A1][0] = sx(at(c, SP, 0x6b0));
                    c.g[T6][0] = sx(at(c, SP, 0x650));
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 21);
                    c.sw(at(c, SP, 0x2704), c.g[A0][0]);
                    c.f[20] = c.f[12];
                    c.sw(at(c, SP, 0x2708), c.g[A1][0]);
                    c.sw(at(c, SP, 0x270c), c.g[T6][0]);
                    pc = 0x0033_4a40;
                    continue;
                }
                0x0033_4a40 => {
                    // Build the model-part transform, preserving the original VU macro operations.
                    c.cycles += 3;
                    c.g[T7][0] = c.lw(at(c, SP, 0x26fc));
                    let t = c.g[T7][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x0033_4ce0;
                        continue;
                    }
                    pc = 0x0033_4a4c;
                    continue;
                }
                0x0033_4a4c => {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw(r(c, S0));
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[A1][0] = sx(r(c, A1) << 6);
                    if t {
                        pc = 0x0033_4ba0;
                        continue;
                    }
                    pc = 0x0033_4a58;
                    continue;
                }
                0x0033_4a58 => {
                    c.cycles += 82;
                    c.g[A0][0] = c.lw(at(c, S0, 0xc));
                    c.lwc1(7, at(c, SP, 0x600));
                    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, SP)));
                    c.lq(2, r(c, A0));
                    c.lwc1(6, at(c, SP, 0x604));
                    c.sq(2, at(c, SP, 0x690));
                    c.sq(2, at(c, SP, 0x650));
                    c.lwc1(5, at(c, SP, 0x608));
                    c.lq(2, at(c, A0, 0x10));
                    c.lwc1(4, at(c, SP, 0x60c));
                    c.sq(2, at(c, SP, 0x690));
                    c.sq(2, at(c, SP, 0x660));
                    c.lq(3, at(c, A0, 0x20));
                    c.sq(3, at(c, SP, 0x690));
                    c.sq(3, at(c, SP, 0x670));
                    c.g[V1][0] = c.lw(at(c, SP, 0x2708));
                    c.lq(2, at(c, A0, 0x30));
                    c.g[A0][0] = c.lw(at(c, SP, 0x270c));
                    c.sq(2, at(c, SP, 0x6a0));
                    c.lwc1(1, at(c, SP, 0x6a0));
                    c.lwc1(2, at(c, SP, 0x6a4));
                    c.lwc1(3, at(c, SP, 0x6a8));
                    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[7])).0;
                    c.lwc1(0, at(c, SP, 0x6ac));
                    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[6])).0;
                    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[5])).0;
                    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[4])).0;
                    c.swc1(1, at(c, SP, 0x690));
                    c.swc1(2, at(c, SP, 0x694));
                    c.swc1(3, at(c, SP, 0x698));
                    c.swc1(0, at(c, SP, 0x69c));
                    c.lq(2, at(c, SP, 0x690));
                    c.sq(2, at(c, SP, 0x680));
                    c.lqc2(4, r(c, A1));
                    c.lqc2(5, at(c, A1, 0x10));
                    c.lqc2(6, at(c, A1, 0x20));
                    c.lqc2(7, at(c, A1, 0x30));
                    c.lqc2(8, r(c, A0));
                    c.lqc2(9, at(c, A0, 0x10));
                    c.lqc2(10, at(c, A0, 0x20));
                    c.lqc2(11, at(c, A0, 0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                    c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                    c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                    c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                    c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                    c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                    c.sqc2(12, r(c, V1));
                    c.sqc2(13, at(c, V1, 0x10));
                    c.sqc2(14, at(c, V1, 0x20));
                    c.sqc2(15, at(c, V1, 0x30));
                    c.g[A1][0] = c.lw(at(c, SP, 0x2704));
                    c.lqc2(1, r(c, V1));
                    c.lqc2(2, at(c, V1, 0x10));
                    c.lqc2(3, at(c, V1, 0x20));
                    c.lqc2(4, at(c, V1, 0x30));
                    c.sqc2(1, r(c, A1));
                    c.sqc2(2, at(c, A1, 0x10));
                    c.sqc2(3, at(c, A1, 0x20));
                    c.sqc2(4, at(c, A1, 0x30));
                    c.g[V0][0] = sx(r(c, S5) << 6);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, SP)));
                    c.lqc2(1, r(c, A1));
                    c.lqc2(2, at(c, A1, 0x10));
                    c.lqc2(3, at(c, A1, 0x20));
                    c.lqc2(4, at(c, A1, 0x30));
                    c.sqc2(1, r(c, V0));
                    c.sqc2(2, at(c, V0, 0x10));
                    c.sqc2(3, at(c, V0, 0x20));
                    c.sqc2(4, at(c, V0, 0x30));
                    c.g[V0][0] = c.lw(at(c, S0, 0x4));
                    pc = 0x0033_4ce4;
                    continue;
                }
                0x0033_4ba0 => {
                    c.cycles += 80;
                    c.g[A0][0] = c.lw(at(c, S0, 0xc));
                    c.lwc1(6, at(c, SP, 0x600));
                    c.lq(2, r(c, A0));
                    c.lwc1(7, at(c, SP, 0x604));
                    c.sq(2, at(c, SP, 0x690));
                    c.sq(2, at(c, SP, 0x650));
                    c.lwc1(5, at(c, SP, 0x608));
                    c.lq(2, at(c, A0, 0x10));
                    c.lwc1(4, at(c, SP, 0x60c));
                    c.sq(2, at(c, SP, 0x690));
                    c.sq(2, at(c, SP, 0x660));
                    c.g[T6][0] = c.lw(at(c, SP, 0x2708));
                    c.lq(3, at(c, A0, 0x20));
                    c.g[T7][0] = c.lw(at(c, SP, 0x26f8));
                    c.sq(3, at(c, SP, 0x690));
                    c.sq(3, at(c, SP, 0x670));
                    c.lq(2, at(c, A0, 0x30));
                    c.sq(2, at(c, SP, 0x6a0));
                    c.lwc1(1, at(c, SP, 0x6a0));
                    c.lwc1(2, at(c, SP, 0x6a4));
                    c.lwc1(3, at(c, SP, 0x6a8));
                    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[6])).0;
                    c.lwc1(0, at(c, SP, 0x6ac));
                    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[7])).0;
                    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[5])).0;
                    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[4])).0;
                    c.swc1(1, at(c, SP, 0x690));
                    c.swc1(2, at(c, SP, 0x694));
                    c.swc1(3, at(c, SP, 0x698));
                    c.swc1(0, at(c, SP, 0x69c));
                    c.lq(2, at(c, SP, 0x690));
                    c.sq(2, at(c, SP, 0x680));
                    c.g[V0][0] = c.lw(at(c, SP, 0x270c));
                    c.lqc2(4, r(c, T7));
                    c.lqc2(5, at(c, T7, 0x10));
                    c.lqc2(6, at(c, T7, 0x20));
                    c.lqc2(7, at(c, T7, 0x30));
                    c.lqc2(8, r(c, V0));
                    c.lqc2(9, at(c, V0, 0x10));
                    c.lqc2(10, at(c, V0, 0x20));
                    c.lqc2(11, at(c, V0, 0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                    c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                    c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                    c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                    c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                    c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                    c.sqc2(12, r(c, T6));
                    c.sqc2(13, at(c, T6, 0x10));
                    c.sqc2(14, at(c, T6, 0x20));
                    c.sqc2(15, at(c, T6, 0x30));
                    c.g[V1][0] = c.lw(at(c, SP, 0x2704));
                    c.lqc2(1, r(c, T6));
                    c.lqc2(2, at(c, T6, 0x10));
                    c.lqc2(3, at(c, T6, 0x20));
                    c.lqc2(4, at(c, T6, 0x30));
                    c.sqc2(1, r(c, V1));
                    c.sqc2(2, at(c, V1, 0x10));
                    c.sqc2(3, at(c, V1, 0x20));
                    c.sqc2(4, at(c, V1, 0x30));
                    c.g[V0][0] = sx(r(c, S5) << 6);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, SP)));
                    c.lqc2(1, r(c, V1));
                    c.lqc2(2, at(c, V1, 0x10));
                    c.lqc2(3, at(c, V1, 0x20));
                    c.lqc2(4, at(c, V1, 0x30));
                    c.sqc2(1, r(c, V0));
                    c.sqc2(2, at(c, V0, 0x10));
                    c.sqc2(3, at(c, V0, 0x20));
                    c.sqc2(4, at(c, V0, 0x30));
                    pc = 0x0033_4ce0;
                    continue;
                }
                0x0033_4ce0 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S0, 0x4));
                    pc = 0x0033_4ce4;
                    continue;
                }
                0x0033_4ce4 => {
                    c.cycles += 7;
                    c.g[A0][0] = sx(at(c, S5, 0x1));
                    c.g[A1][0] = sx(at(c, S0, 0x10));
                    c.g[T6][0] = sx(at(c, S4, 0x40));
                    c.sw(at(c, SP, 0x2714), c.g[A0][0]);
                    c.sw(at(c, SP, 0x2718), c.g[A1][0]);
                    let t = c.g[V0][0] == 0u64;
                    c.sw(at(c, SP, 0x271c), c.g[T6][0]);
                    if t {
                        pc = 0x0033_50cc;
                        continue;
                    }
                    pc = 0x0033_4d00;
                    continue;
                }
                0x0033_4d00 => {
                    // Query 2 excludes surface -1; query 1 includes only surface -1.
                    // Mass is not consulted here: non-solid scripted parts use query 1.
                    c.cycles += 4;
                    c.g[T7][0] = c.lw(at(c, SP, 0x26f4));
                    c.g[V0][0] = 0x2;

                    if c.g[T7][0] != c.g[V0][0] {
                        c.g[T6][0] = c.lw(at(c, SP, 0x26f4));
                        pc = 0x0033_4d38;
                        continue;
                    }
                    pc = 0x0033_4d10;
                    continue;
                }
                0x0033_4d10 => {
                    c.cycles += 8;
                    c.g[V1][0] = 0xc;
                    c.g[A1][0] = 0xffffffffffffffff;
                    c.mult(0, 2, 21, 3);
                    c.g[V1][0] = c.lw(at(c, S1, INSTANCE_PROPERTY));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A0][0] = c.lh(at(c, V0, PART_SURFACE));
                    let t = c.g[A0][0] != c.g[A1][0];
                    c.g[V1][0] = sx(at(c, S5, 0x1));
                    if t {
                        pc = 0x0033_4d74;
                        continue;
                    }
                    pc = 0x0033_4d30;
                    continue;
                }
                0x0033_4d30 => {
                    c.cycles += 2;
                    c.g[V1][0] = sx(at(c, S6, 0x1));
                    pc = 0x0033_4d64;
                    continue;
                }
                0x0033_4d38 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[T6][0] != c.g[V0][0];
                    c.g[V1][0] = sx(at(c, S5, 0x1));
                    if t {
                        pc = 0x0033_4d74;
                        continue;
                    }
                    pc = 0x0033_4d44;
                    continue;
                }
                0x0033_4d44 => {
                    c.cycles += 8;
                    c.g[T7][0] = 0xc;
                    c.g[V1][0] = c.lw(at(c, S1, INSTANCE_PROPERTY));
                    c.mult(0, 2, 21, 15);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A0][0] = c.lh(at(c, V0, PART_SURFACE));
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.g[V1][0] = sx(at(c, S6, 0x1));
                    if t {
                        pc = 0x0033_4d70;
                        continue;
                    }
                    pc = 0x0033_4d64;
                    continue;
                }
                0x0033_4d64 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[FP][0] ^ 0x2;
                    if c.g[V0][0] != 0 {
                        c.g[S6][0] = c.g[V1][0];
                    }
                    pc = 0x0033_50cc;
                    continue;
                }
                0x0033_4d70 => {
                    c.cycles += 1;
                    c.g[V1][0] = sx(at(c, S5, 0x1));
                    pc = 0x0033_4d74;
                    continue;
                }
                0x0033_4d74 => {
                    c.cycles += 68;
                    c.g[A1][0] = sx(at(c, S0, 0x10));
                    c.g[T6][0] = sx(at(c, S4, 0x40));
                    c.sw(at(c, SP, 0x2714), c.g[V1][0]);
                    c.sw(at(c, SP, 0x2718), c.g[A1][0]);
                    c.g[V0][0] = 0x3f;
                    c.sw(at(c, SP, 0x271c), c.g[T6][0]);
                    c.g[A0][0] = sx(at(c, SP, 0x610));
                    c.g[A2][0] = sx(at(c, SP, 0x650));
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.lqc2(1, r(c, S4));
                    c.lqc2(2, at(c, S4, 0x10));
                    c.lqc2(3, at(c, S4, 0x20));
                    c.lqc2(4, at(c, S4, 0x30));
                    c.sqc2(1, r(c, A0));
                    c.sqc2(2, at(c, A0, 0x10));
                    c.sqc2(3, at(c, A0, 0x20));
                    c.sqc2(4, at(c, A0, 0x30));
                    c.lwc1(3, at(c, SP, 0x614));
                    c.g[A1][0] = c.g[A0][0];
                    c.lwc1(2, at(c, SP, 0x620));
                    c.g[S3][0] = 0u64;
                    c.swc1(3, at(c, SP, 0x620));
                    c.lwc1(3, at(c, SP, 0x618));
                    c.lwc1(1, at(c, SP, 0x630));
                    c.swc1(3, at(c, SP, 0x630));
                    c.lq(2, at(c, SP, 0x640));
                    c.lwc1(3, at(c, SP, 0x628));
                    c.lwc1(0, at(c, SP, 0x634));
                    c.sq(2, at(c, SP, 0x660));
                    c.sw(at(c, SP, 0x2710), 0u64);
                    c.swc1(2, at(c, SP, 0x614));
                    c.swc1(1, at(c, SP, 0x618));
                    c.swc1(0, at(c, SP, 0x628));
                    c.swc1(3, at(c, SP, 0x634));
                    c.mfc1(3, 21);
                    c.lqc2(4, at(c, SP, 0x660));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0x670));
                    c.lq(2, at(c, SP, 0x670));
                    c.sq(2, at(c, SP, 0x650));
                    c.sw(at(c, SP, 0x65c), 0u64);
                    c.lqc2(8, at(c, SP, 0x650));
                    c.lqc2(4, r(c, A0));
                    c.lqc2(5, at(c, A0, 0x10));
                    c.lqc2(6, at(c, A0, 0x20));
                    c.lqc2(7, at(c, A0, 0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, at(c, A2, 0x20));
                    c.lq(2, at(c, SP, 0x670));
                    c.lwc1(12, at(c, S1, INSTANCE_SCALE));
                    c.sq(2, at(c, SP, 0x650));
                    c.sq(2, at(c, SP, 0x660));
                    c.f[12] = FpuF32(c.f[20]).div(FpuF32(c.f[12])).0;
                    c.swc1(20, at(c, A2, 0xc));
                    c.g[A2][0] = c.lw(at(c, S2, 0x50));
                    c.lq(2, at(c, SP, 0x650));
                    c.sq(2, at(c, SP, 0x640));
                    c.g[A0][0] = c.lh(at(c, A2, 0x8));
                    c.g[V1][0] = c.lw(at(c, A2, 0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0033_4e78);
                    c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
                    call_ptr(c, t, 0x0033_4e78)?;
                    pc = 0x0033_4e78;
                    continue;
                }
                0x0033_4e78 => {
                    c.g[V0][0] = 0x2;
                    let t = c.g[FP][0] != c.g[V0][0];
                    c.g[V1][0] = c.lw(at(c, S2, 0x50));
                    if t {
                        pc = 0x0033_4eac;
                        continue;
                    }
                    pc = 0x0033_4e84;
                    continue;
                }
                0x0033_4e84 => {
                    c.cycles += 10;
                    c.g[A3][0] = sx(at(c, SP, 0x6f0));
                    c.g[A1][0] = c.lw(at(c, S0, 0x4));
                    c.g[T0][0] = 0x40;
                    c.g[A0][0] = c.lh(at(c, V1, 0x38));
                    c.g[V0][0] = c.lw(at(c, V1, 0x3c));
                    c.g[A2][0] = sx(at(c, A1, 0xc));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0033_4ea4);
                    c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
                    call_ptr(c, t, 0x0033_4ea4)?;
                    pc = 0x0033_4ea4;
                    continue;
                }
                0x0033_4ea4 => {
                    c.g[S3][0] = c.g[V0][0];
                    pc = 0x0033_4f50;
                    continue;
                }
                0x0033_4eac => {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw(at(c, S0, 0x4));
                    c.g[A0][0] = c.lh(at(c, V1, 0x48));
                    c.g[V0][0] = c.lw(at(c, V1, 0x4c));
                    c.g[A2][0] = sx(at(c, A1, 0xc));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0033_4ec4);
                    c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
                    call_ptr(c, t, 0x0033_4ec4)?;
                    pc = 0x0033_4ec4;
                    continue;
                }
                0x0033_4ec4 => {
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x0033_4f4c;
                        continue;
                    }
                    pc = 0x0033_4ecc;
                    continue;
                }
                0x0033_4ecc => {
                    c.cycles += 2;
                    let t = c.g[FP][0] != c.g[V0][0];
                    c.g[A0][0] = c.lw(at(c, GP, 0x16c8));
                    if t {
                        pc = 0x0033_4f14;
                        continue;
                    }
                    pc = 0x0033_4ed4;
                    continue;
                }
                0x0033_4ed4 => {
                    // Mode 3 selects the matching native sphere tree, not a mesh face list.
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x0033_4edc);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x003a_6cd8, 0x0033_4edc)?;
                    pc = 0x0033_4edc;
                    continue;
                }
                0x0033_4edc => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0033_4ee8);
                    c.g[A1][0] = c.g[S6][0];
                    call_ptr(c, 0x003a_6ce8, 0x0033_4ee8)?;
                    pc = 0x0033_4ee8;
                    continue;
                }
                0x0033_4ee8 => {
                    c.g[V1][0] = c.lw(at(c, S2, 0x50));
                    c.g[A2][0] = sx(at(c, SP, 0x6f0));
                    c.sw(at(c, SP, 0x2710), c.g[V0][0]);
                    c.g[A3][0] = 0x40;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.lh(at(c, V1, 0x30));
                    c.g[V0][0] = c.lw(at(c, V1, 0x34));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0033_4f0c);
                    c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
                    call_ptr(c, t, 0x0033_4f0c)?;
                    pc = 0x0033_4f0c;
                    continue;
                }
                0x0033_4f0c => {
                    c.g[S3][0] = c.g[V0][0];
                    pc = 0x0033_4f4c;
                    continue;
                }
                0x0033_4f14 => {
                    // Mesh mode: resolve the geometry and dispatch its native collision query.
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x0033_4f1c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x003a_6cc8, 0x0033_4f1c)?;
                    pc = 0x0033_4f1c;
                    continue;
                }
                0x0033_4f1c => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0033_4f28);
                    c.g[A1][0] = c.g[S6][0];
                    call_ptr(c, 0x003a_6d00, 0x0033_4f28)?;
                    pc = 0x0033_4f28;
                    continue;
                }
                0x0033_4f28 => {
                    c.g[V1][0] = c.lw(at(c, S0, 0x4));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A3][0] = sx(at(c, SP, 0x6f0));
                    c.g[A2][0] = c.lw(at(c, V1, 0x18));
                    c.g[T0][0] = 0x40;
                    c.g[RA][0] = sx(0x0033_4f48);
                    c.g[A2][0] = c.g[A2][0] & 0x1;
                    call_ptr(c, 0x0032_c0f8, 0x0033_4f48)?;
                    pc = 0x0033_4f48;
                    continue;
                }
                0x0033_4f48 => {
                    c.g[S3][0] = c.g[V0][0];
                    pc = 0x0033_4f4c;
                    continue;
                }
                0x0033_4f4c => {
                    c.cycles += 1;
                    c.g[S6][0] = sx(at(c, S6, 0x1));
                    pc = 0x0033_4f50;
                    continue;
                }
                0x0033_4f50 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(at(c, S2, 0x50));
                    c.g[A0][0] = c.lh(at(c, V0, 0x10));
                    c.g[V1][0] = c.lw(at(c, V0, 0x14));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0033_4f64);
                    c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
                    call_ptr(c, t, 0x0033_4f64)?;
                    pc = 0x0033_4f64;
                    continue;
                }
                0x0033_4f64 => {
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x0033_50cc;
                        continue;
                    }
                    pc = 0x0033_4f6c;
                    continue;
                }
                0x0033_4f6c => {
                    c.cycles += 7;
                    c.g[T7][0] = 0xc;
                    c.g[V0][0] = 0x500000;
                    c.mult(0, 3, 21, 15);
                    c.g[T3][0] = sx(r(c, V0).wrapping_sub(0xee0));
                    c.g[T5][0] = 0x1;
                    c.g[T2][0] = 0xffffffffffffffff;
                    c.g[T4][0] = sx(at(c, V1, 0x10));
                    pc = 0x0033_4f88;
                    continue;
                }
                0x0033_4f88 => {
                    // Convert hits back to world space and retain instance/part/surface context.
                    c.cycles += 23;
                    c.g[V0][0] = sx(r(c, A1) << 7);
                    c.g[T0][0] = sx(at(c, A1, 0x1));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, SP)));
                    c.g[A0][0] = sx(r(c, S7) << 7);
                    c.lqc2(8, at(c, V0, 0x700));
                    c.lqc2(4, r(c, S4));
                    c.lqc2(5, at(c, S4, 0x10));
                    c.lqc2(6, at(c, S4, 0x20));
                    c.lqc2(7, at(c, S4, 0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, at(c, SP, 0x660));
                    c.g[A3][0] = c.g[A0][0];
                    c.lq(3, at(c, SP, 0x660));
                    c.g[A2][0] = sx(at(c, V0, 0x770));
                    c.g[A1][0] = c.lw(at(c, SP, 0x26f0));
                    c.g[T1][0] = sx(at(c, S7, 0x1));
                    c.sq(3, at(c, SP, 0x650));
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A1)));
                    c.sq(3, at(c, V0, 0x700));
                    c.g[A1][0] = sx(at(c, V0, 0x6f0));
                    pc = 0x0033_4fe4;
                    continue;
                }
                0x0033_4fe4 => {
                    c.cycles += 7;
                    c.lq(14, r(c, A1));
                    c.lq(15, at(c, A1, 0x10));
                    c.sq(14, r(c, A0));
                    c.sq(15, at(c, A0, 0x10));
                    c.g[A1][0] = sx(at(c, A1, 0x20));
                    let t = c.g[A1][0] != c.g[A2][0];
                    c.g[A0][0] = sx(at(c, A0, 0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0033_4fe4) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0033_4fe4;
                        continue;
                    }
                    pc = 0x0033_5000;
                    continue;
                }
                0x0033_5000 => {
                    c.cycles += 47;
                    c.g[A1][0] = c.lw(at(c, SP, 0x26f0));
                    c.lwc1(0, at(c, SP, 0x600));
                    c.g[A0][0] = sx(r(c, A3).wrapping_add(r(c, A1)));
                    c.lwc1(1, at(c, SP, 0x608));
                    c.lwc1(2, r(c, A0));
                    c.lwc1(6, at(c, SP, 0x60c));
                    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[0])).0;
                    c.lwc1(4, at(c, A0, 0x4));
                    c.lwc1(0, at(c, SP, 0x604));
                    c.lwc1(5, at(c, A0, 0x8));
                    c.lwc1(3, at(c, A0, 0xc));
                    c.f[4] = (FpuF32(c.f[4]) * FpuF32(c.f[0])).0;
                    c.f[5] = (FpuF32(c.f[5]) * FpuF32(c.f[1])).0;
                    c.swc1(2, at(c, SP, 0x660));
                    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[6])).0;
                    c.swc1(4, at(c, SP, 0x664));
                    c.swc1(5, at(c, SP, 0x668));
                    c.swc1(3, at(c, SP, 0x66c));
                    c.lqc2(8, at(c, SP, 0x660));
                    c.lqc2(4, r(c, S4));
                    c.lqc2(5, at(c, S4, 0x10));
                    c.lqc2(6, at(c, S4, 0x20));
                    c.lqc2(7, at(c, S4, 0x30));
                    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                    c.sqc2(12, at(c, SP, 0x670));
                    c.lq(3, at(c, SP, 0x670));
                    c.sw(at(c, A0, 0x44), c.g[T5][0]);
                    c.sq(3, r(c, A0));
                    c.sw(at(c, A0, 0x50), c.g[S1][0]);
                    c.sw(at(c, A0, 0x5c), c.g[S5][0]);
                    c.sw(at(c, A0, 0x4c), c.g[T2][0]);
                    c.g[T6][0] = c.lw(at(c, SP, 0x2710));
                    c.sw(at(c, A0, 0x48), c.g[T6][0]);
                    c.sq(3, at(c, SP, 0x650));
                    c.lq(2, r(c, T3));
                    c.sq(2, at(c, A0, 0x20));
                    c.lq(3, r(c, T3));
                    c.sw(at(c, A0, 0x54), 0u64);
                    c.sq(3, at(c, A0, 0x30));
                    c.g[V0][0] = c.lw(at(c, S1, INSTANCE_PROPERTY));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T4)));
                    c.g[V0][0] = c.lh(at(c, V0, 0xa));

                    if c.g[V0][0] != c.g[T2][0] {
                        c.sw(at(c, A0, 0x4c), c.g[V0][0]);
                        pc = 0x0033_50bc;
                        continue;
                    }
                    pc = 0x0033_50bc;
                    continue;
                }
                0x0033_50bc => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[T0][0];
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S7][0] = c.g[T1][0];
                    if t {
                        if back_edge_unwinds(c, 0x0033_4f88) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0033_4f88;
                        continue;
                    }
                    pc = 0x0033_50cc;
                    continue;
                }
                0x0033_50cc => {
                    c.cycles += 7;
                    c.g[T7][0] = c.lw(at(c, SP, 0x2700));
                    c.g[S5][0] = c.lw(at(c, SP, 0x2714));
                    c.g[V0][0] = c.lw(at(c, T7, 0x4));
                    c.g[S0][0] = c.lw(at(c, SP, 0x2718));
                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S4][0] = c.lw(at(c, SP, 0x271c));
                    if t {
                        if back_edge_unwinds(c, 0x0033_4a40) {
                            return Err(crate::call::Unwound);
                        }
                        pc = 0x0033_4a40;
                        continue;
                    }
                    pc = 0x0033_50e8;
                    continue;
                }
                0x0033_50e8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S7][0];
                    pc = 0x0033_50ec;
                    continue;
                }
                0x0033_50ec => {
                    c.cycles += 1;
                    frame.restore(c, 16, 0x27b0);
                    pc = 0x0033_50f0;
                    continue;
                }
                0x0033_50f0 => {
                    // Native epilogue; direct object-query delegates arrive with S0 restored.
                    c.cycles += 13;
                    frame.restore(c, 17, 0x27a0);
                    frame.restore(c, 18, 0x2790);
                    frame.restore(c, 19, 0x2780);
                    frame.restore(c, 20, 0x2770);
                    frame.restore(c, 21, 0x2760);
                    frame.restore(c, 22, 0x2750);
                    frame.restore(c, 23, 0x2740);
                    frame.restore(c, 30, 0x2730);
                    frame.restore_ra(c, 0x2720);
                    c.lwc1(21, at(c, SP, 0x27c8));
                    c.lwc1(20, at(c, SP, 0x27c0));
                    let t = c.g[RA][0] as u32;
                    frame.close(c);
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Err(crate::call::Unwound);
                }
            }
        }
    })
}
