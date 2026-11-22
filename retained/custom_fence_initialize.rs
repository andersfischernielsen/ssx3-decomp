//! Native Fence_BuildMesh0x0034d1e8, retained uninstalled.
//!
//! Generated EE owns registers, cycles, scratch writes and call order.
//! Automated tests, cargo and game runs are disabled for this writer.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0034_d1e8;
const AFTER_MESH: u32 = 0x0034_d25c;
const QUAD_SCRATCH_LOOP: u32 = 0x0034_d2a0;
const AFTER_QUAD: u32 = 0x0034_d2d4;
const STRIP_SCRATCH_LOOP: u32 = 0x0034_d2e8;
const SLICE_LOOP: u32 = 0x0034_d328;
const AFTER_UPLOAD: u32 = 0x0034_d618;
const AFTER_UPDATE: u32 = 0x0034_d620;
const END: u32 = 0x0034_d64c;

const INSTANCE: u32 = 0x18;
const MESH: u32 = 0x40;
const LEFT_NEIGHBOUR: u32 = 0x104;
const RIGHT_NEIGHBOUR: u32 = 0x108;
const GRAPHICS_MANAGER_GP: u32 = 0x2a90;

fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Interpolate one of eight columns in the original operation order.
/// S1 is Fence*, T1 is the column, F9 is1/7. Two XYZ rows go into the
/// fence at+44/+a4; UV and RGBA rows go into the temporary upload arrays.
fn interpolate_slice(c: &mut Cpu) {
    c.lwc1(7, r(c, SP));
    c.mtc1(T1, 8);
    c.cvt_s_w(8, 8);
    c.lwc1(0, at(c, SP, 0xc));
    c.g[V0][0] = 0xc;
    c.lwc1(6, at(c, SP, 0x4));
    c.mult(0, V1, T1, V0);
    c.lwc1(5, at(c, SP, 0x8));
    c.sub_s(0, 0, 7);
    c.lwc1(1, at(c, SP, 0x10));
    c.lwc1(2, at(c, SP, 0x14));
    c.sub_s(1, 1, 6);
    c.swc1(0, at(c, SP, 0x240));
    c.sub_s(2, 2, 5);
    c.g[V0][0] = sx(r(c, V1).wrapping_add(r(c, S1)));
    c.mul_s(3, 0, 8);
    c.g[V1][0] = sx(r(c, T1) << 3);
    c.mul_s(4, 1, 8);
    c.swc1(1, at(c, SP, 0x244));
    c.mul_s(0, 2, 8);
    c.swc1(2, at(c, SP, 0x248));
    c.mul_s(1, 3, 9);
    c.swc1(3, at(c, SP, 0x230));
    c.mul_s(2, 4, 9);
    c.swc1(4, at(c, SP, 0x234));
    c.mul_s(3, 0, 9);
    c.swc1(0, at(c, SP, 0x238));
    c.add_s(7, 7, 1);
    c.swc1(1, at(c, SP, 0x220));
    c.add_s(6, 6, 2);
    c.swc1(2, at(c, SP, 0x224));
    c.add_s(5, 5, 3);
    c.swc1(3, at(c, SP, 0x228));
    c.swc1(7, at(c, SP, 0x210));
    c.g[V1][0] = sx(r(c, S5).wrapping_add(r(c, V1)));
    c.swc1(6, at(c, SP, 0x214));
    c.g[T1][0] = sx(at(c, T1, 0x1));
    c.swc1(5, at(c, SP, 0x218));
    c.g[A0][0] = ((c.g[T1][0] as i64) < 8i64) as u64;
    c.ldl(T3, at(c, SP, 0x217));
    c.ldr(T3, at(c, SP, 0x210));
    c.g[T4][0] = c.lw(at(c, SP, 0x218));
    c.sdl(T3, at(c, V0, 0x4b));
    c.sdr(T3, at(c, V0, 0x44));
    c.sw(at(c, V0, 0x4c), c.g[T4][0]);
    // Bottom edge positions.
    c.lwc1(5, at(c, SP, 0x18));
    c.lwc1(0, at(c, SP, 0x24));
    c.lwc1(6, at(c, SP, 0x1c));
    c.lwc1(7, at(c, SP, 0x20));
    c.sub_s(0, 0, 5);
    c.lwc1(1, at(c, SP, 0x28));
    c.lwc1(2, at(c, SP, 0x2c));
    c.sub_s(1, 1, 6);
    c.swc1(0, at(c, SP, 0x250));
    c.sub_s(2, 2, 7);
    c.mul_s(3, 0, 8);
    c.mul_s(4, 1, 8);
    c.swc1(1, at(c, SP, 0x254));
    c.mul_s(0, 2, 8);
    c.swc1(2, at(c, SP, 0x258));
    c.mul_s(1, 3, 9);
    c.swc1(3, at(c, SP, 0x230));
    c.mul_s(2, 4, 9);
    c.swc1(4, at(c, SP, 0x234));
    c.mul_s(3, 0, 9);
    c.swc1(0, at(c, SP, 0x238));
    c.add_s(5, 5, 1);
    c.swc1(1, at(c, SP, 0x220));
    c.add_s(6, 6, 2);
    c.swc1(2, at(c, SP, 0x224));
    c.add_s(7, 7, 3);
    c.swc1(3, at(c, SP, 0x228));
    c.swc1(5, at(c, SP, 0x210));
    c.swc1(6, at(c, SP, 0x214));
    c.swc1(7, at(c, SP, 0x218));
    c.ldl(T3, at(c, SP, 0x217));
    c.ldr(T3, at(c, SP, 0x210));
    c.g[T4][0] = c.lw(at(c, SP, 0x218));
    c.sdl(T3, at(c, V0, 0xab));
    c.sdr(T3, at(c, V0, 0xa4));
    c.sw(at(c, V0, 0xac), c.g[T4][0]);
    // Top and bottom texture coordinates.
    c.lwc1(4, at(c, SP, 0x30));
    c.lwc1(5, at(c, SP, 0x34));
    c.lwc1(0, at(c, SP, 0x38));
    c.lwc1(1, at(c, SP, 0x3c));
    c.sub_s(0, 0, 4);
    c.sub_s(1, 1, 5);
    c.mul_s(2, 0, 8);
    c.swc1(0, at(c, SP, 0x260));
    c.mul_s(3, 1, 8);
    c.swc1(1, at(c, SP, 0x264));
    c.mul_s(0, 2, 9);
    c.swc1(2, at(c, SP, 0x230));
    c.mul_s(1, 3, 9);
    c.swc1(3, at(c, SP, 0x234));
    c.add_s(4, 4, 0);
    c.swc1(0, at(c, SP, 0x220));
    c.add_s(5, 5, 1);
    c.swc1(1, at(c, SP, 0x224));
    c.swc1(4, at(c, SP, 0x210));
    c.swc1(5, at(c, SP, 0x214));
    c.ldl(V0, at(c, SP, 0x217));
    c.ldr(V0, at(c, SP, 0x210));
    c.sdl(V0, at(c, T2, 0x7));
    c.sdr(V0, r(c, T2));
    c.g[T2][0] = sx(at(c, T2, 0x8));
    c.lwc1(4, at(c, SP, 0x40));
    c.lwc1(5, at(c, SP, 0x44));
    c.lwc1(0, at(c, SP, 0x48));
    c.lwc1(1, at(c, SP, 0x4c));
    c.sub_s(0, 0, 4);
    c.sub_s(1, 1, 5);
    c.mul_s(2, 0, 8);
    c.swc1(0, at(c, SP, 0x240));
    c.mul_s(3, 1, 8);
    c.swc1(1, at(c, SP, 0x244));
    c.mul_s(0, 2, 9);
    c.swc1(2, at(c, SP, 0x230));
    c.mul_s(1, 3, 9);
    c.swc1(3, at(c, SP, 0x234));
    c.add_s(4, 4, 0);
    c.swc1(0, at(c, SP, 0x220));
    c.add_s(5, 5, 1);
    c.swc1(1, at(c, SP, 0x224));
    c.swc1(4, at(c, SP, 0x210));
    c.swc1(5, at(c, SP, 0x214));
    c.ldl(T6, at(c, SP, 0x217));
    c.ldr(T6, at(c, SP, 0x210));
    c.sdl(T6, at(c, V1, 0x7));
    c.sdr(T6, r(c, V1));
    // RGB interpolation; both rows force alpha to1.
    c.swc1(10, r(c, A1));
    c.lwc1(1, at(c, SP, 0x54));
    c.lwc1(0, at(c, SP, 0x64));
    c.sub_s(0, 0, 1);
    c.mul_s(0, 0, 8);
    c.mul_s(0, 0, 9);
    c.add_s(1, 1, 0);
    c.swc1(1, r(c, A2));
    c.lwc1(2, at(c, SP, 0x58));
    c.lwc1(0, at(c, SP, 0x68));
    c.sub_s(0, 0, 2);
    c.mul_s(0, 0, 8);
    c.mul_s(0, 0, 9);
    c.add_s(2, 2, 0);
    c.swc1(2, r(c, A3));
    c.lwc1(1, at(c, SP, 0x5c));
    c.lwc1(0, at(c, SP, 0x6c));
    c.sub_s(0, 0, 1);
    c.mul_s(0, 0, 8);
    c.mul_s(0, 0, 9);
    c.add_s(1, 1, 0);
    c.swc1(1, r(c, T0));
    c.swc1(10, at(c, A1, 0x80));
    c.g[A1][0] = sx(at(c, A1, 0x10));
    c.lwc1(1, at(c, SP, 0x74));
    c.lwc1(0, at(c, SP, 0x84));
    c.sub_s(0, 0, 1);
    c.mul_s(0, 0, 8);
    c.mul_s(0, 0, 9);
    c.add_s(1, 1, 0);
    c.swc1(1, at(c, A2, 0x80));
    c.g[A2][0] = sx(at(c, A2, 0x10));
    c.lwc1(1, at(c, SP, 0x78));
    c.lwc1(0, at(c, SP, 0x88));
    c.sub_s(0, 0, 1);
    c.mul_s(0, 0, 8);
    c.mul_s(0, 0, 9);
    c.add_s(1, 1, 0);
    c.swc1(1, at(c, A3, 0x80));
    c.g[A3][0] = sx(at(c, A3, 0x10));
    c.lwc1(1, at(c, SP, 0x7c));
    c.lwc1(0, at(c, SP, 0x8c));
    c.sub_s(0, 0, 1);
    c.mul_s(0, 0, 8);
    c.mul_s(0, 0, 9);
    c.add_s(1, 1, 0);
    c.swc1(1, at(c, T0, 0x80));
    c.g[T0][0] = sx(r(c, T0).wrapping_add(0x10));
}

/// `void Fence_BuildMesh(Fence* self)` (0x34d1e8).
/// Hide the original static instance, create a native8x2 render mesh and
/// ask graphics slot97 for four source corners, UVs and colours (index0).
/// Build eight samples on each opposing edge, upload UV/colour data, then
/// let Fence_UpdateVerts populate the deformable positions. This routine
/// never reads descriptor fields+20/+24 and does not interpret arbitrary
/// model vertices itself; the native quad getter owns that extraction.
pub fn fence_build_mesh(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x300 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 46;
                    Frame::open(c, frame.size);
                    c.g[V1][0] = 0xffff_ffff_ffff_0000;
                    c.sq(S0, at(c, SP, 0x2f0));
                    c.g[V1][0] |= 0xfffd;
                    c.sq(S1, at(c, SP, 0x2e0));
                    c.g[A1][0] = 8;
                    c.sq(S2, at(c, SP, 0x2d0));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(S3, at(c, SP, 0x2c0));
                    c.g[A2][0] = 2;
                    c.sq(S4, at(c, SP, 0x2b0));
                    c.g[A3][0] = 0x6000_0000;
                    c.sq(S5, at(c, SP, 0x2a0));
                    c.sq(S6, at(c, SP, 0x290));
                    c.sq(S7, at(c, SP, 0x280));
                    frame.save_ra(c, 0x270);
                    c.sw(at(c, S1, LEFT_NEIGHBOUR), 0);
                    c.sw(at(c, S1, RIGHT_NEIGHBOUR), 0);
                    c.g[A0][0] = c.lw(at(c, S1, INSTANCE));
                    c.g[S0][0] = c.lw(at(c, GP, GRAPHICS_MANAGER_GP));
                    c.g[V0][0] = c.lw(at(c, A0, 8));
                    c.g[V0][0] &= c.g[V1][0];
                    c.g[V0][0] |= 4;
                    c.sw(at(c, A0, 8), c.g[V0][0]);
                    c.g[V1][0] = c.lw(at(c, S0, 0x10d8));
                    c.g[A0][0] = c.lh(at(c, V1, 0x260));
                    c.g[V0][0] = c.lw(at(c, V1, 0x264));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, target, AFTER_MESH)?;
                    pc = AFTER_MESH;
                }
                AFTER_MESH => {
                    c.sw(at(c, S1, MESH), c.g[V0][0]);
                    c.g[T0][0] = sx(at(c, SP, 0x30));
                    c.g[T1][0] = sx(at(c, SP, 0x50));
                    c.g[S5][0] = sx(at(c, SP, 0xd0));
                    c.g[S6][0] = sx(at(c, SP, 0x110));
                    c.g[S4][0] = sx(at(c, SP, 0x114));
                    c.g[S3][0] = sx(at(c, SP, 0x118));
                    c.g[S2][0] = sx(at(c, SP, 0x11c));
                    c.g[S7][0] = sx(at(c, SP, 0x90));
                    c.g[V1][0] = u64::MAX;
                    c.g[V0][0] = 3;
                    c.g[A1][0] = c.lw(at(c, S0, 0x10d8));
                    c.g[V0][0] = 3;
                    c.g[V1][0] = u64::MAX;
                    c.g[V0][0] = 3;
                    c.g[V1][0] = u64::MAX;
                    pc = QUAD_SCRATCH_LOOP;
                }
                QUAD_SCRATCH_LOOP => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    c.g[A2][0] = 0;
                    if more {
                        if event_due(c) { deliver_event_here(c, QUAD_SCRATCH_LOOP); }
                    } else {
                        c.cycles += 11;
                        c.g[A0][0] = c.lh(at(c, A1, 0x308));
                        c.g[V0][0] = c.lw(at(c, A1, 0x30c));
                        c.g[A3][0] = c.g[SP][0];
                        c.g[A1][0] = c.lw(at(c, S1, INSTANCE));
                        let target = r(c, V0);
                        c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                        call_ptr(c, target, AFTER_QUAD)?;
                        pc = AFTER_QUAD;
                    }
                }
                AFTER_QUAD => {
                    c.g[V1][0] = u64::MAX;
                    c.g[V0][0] = 15;
                    c.g[V0][0] = 15;
                    c.g[V1][0] = u64::MAX;
                    pc = STRIP_SCRATCH_LOOP;
                }
                STRIP_SCRATCH_LOOP => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    c.g[T0][0] = c.g[S2][0];
                    if more {
                        if event_due(c) { deliver_event_here(c, STRIP_SCRATCH_LOOP); }
                    } else {
                        c.cycles += 9;
                        c.lwc1(9, r(c, GP).wrapping_sub(0x2abc));
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 10);
                        c.g[A3][0] = c.g[S3][0];
                        c.g[A2][0] = c.g[S4][0];
                        c.g[T1][0] = 0;
                        c.g[A1][0] = c.g[S6][0];
                        c.g[T2][0] = sx(at(c, SP, 0x90));
                        pc = SLICE_LOOP;
                    }
                }
                SLICE_LOOP => {
                    c.cycles += 179;
                    interpolate_slice(c);
                    if c.g[A0][0] != 0 {
                        if event_due(c) { deliver_event_here(c, SLICE_LOOP); }
                    } else {
                        c.cycles += 22;
                        c.g[T0][0] = c.lw(at(c, S1, MESH));
                        c.g[A2][0] = c.g[S6][0];
                        c.g[A3][0] = c.g[S7][0];
                        c.g[A1][0] = c.g[SP][0];
                        c.g[V0][0] = c.lw(at(c, T0, 4));
                        c.g[A0][0] = c.lh(at(c, V0, 0x30));
                        c.g[V1][0] = c.lw(at(c, V0, 0x34));
                        let target = r(c, V1);
                        c.g[A0][0] = sx(r(c, T0).wrapping_add(r(c, A0)));
                        call_ptr(c, target, AFTER_UPLOAD)?;
                        pc = AFTER_UPLOAD;
                    }
                }
                AFTER_UPLOAD => {
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0034_cf98, AFTER_UPDATE)?;
                    pc = AFTER_UPDATE;
                }
                AFTER_UPDATE => {
                    for (register, offset) in [(S0,0x2f0),(S1,0x2e0),(S2,0x2d0),(S3,0x2c0),
                        (S4,0x2b0),(S5,0x2a0),(S6,0x290),(S7,0x280)] {
                        c.lq(register, at(c, SP, offset));
                    }
                    frame.restore_ra(c, 0x270);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
