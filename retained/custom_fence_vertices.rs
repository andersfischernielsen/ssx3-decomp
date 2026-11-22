//! Native Fence_UpdateVerts34cf98, retained uninstalled.
//!
//! Generated EE owns registers, cycles, VU operations and call order.
//! No automated tests, builds, replays or game/window control were used.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0034_cf98;
const EMPTY_CONSTRUCTORS: u32 = 0x0034_cff8;
const SCALE_OFFSETS: u32 = 0x0034_d020;
const MOVE_BOTH_ROWS: u32 = 0x0034_d088;
const MOVE_TOP_ROW: u32 = 0x0034_d148;
const UPLOAD_POINTS: u32 = 0x0034_d1c4;
const RETURN: u32 = 0x0034_d1d8;
const END: u32 = 0x0034_d1e8;

const MODE: u32 = 0x20;
const SCALE: u32 = 0x24;
const SWAY: u32 = 0x2c;
const MESH: u32 = 0x40;
const TOP: u32 = 0x44;
const BOTTOM: u32 = 0xa4;
const OFFSET_WEIGHTS: u32 = 0x0044_afd0;

fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Copy the temporary Vec3 at stack+90 into the current destination.
/// T1/T2 retain the generated unaligned-load/store register results.
fn copy_point(c: &mut Cpu, destination: usize) {
    c.ldl(T1, at(c, SP, 0x97));
    c.ldr(T1, at(c, SP, 0x90));
    c.g[T2][0] = c.lw(at(c, SP, 0x98));
    c.sdl(T1, at(c, destination, 7));
    c.sdr(T1, r(c, destination));
    c.sw(at(c, destination, 8), c.g[T2][0]);
}

/// `void Fence_UpdateVerts(Fence* self)` (0x34cf98).
/// Form eight weighted offsets from `(200,0,0,0) * sway2c * scale24`.
/// Mode20==1 offsets both eight-point rows; other modes keep the lower
/// row fixed. Supply the resulting16 Vec3 points to the mesh's virtual
/// slot3 (+18/+1c). The reference points are already in BuildMesh's frame;
/// this routine does not rotate the displacement through an instance matrix.
pub fn fence_update_vertices(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x180 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 5;
                    Frame::open(c, 0x180);
                    frame.save_ra(c, 0x170);
                    c.g[V0][0] = c.lw(at(c, A0, MESH));
                    let missing_mesh = c.g[V0][0] == 0;
                    c.g[A2][0] = 7;
                    if missing_mesh { pc = RETURN; continue }
                    c.cycles += 19;
                    c.lwc1(0, at(c, A0, SWAY));
                    c.lwc1(1, at(c, A0, SCALE));
                    c.g[V1][0] = sx(at(c, SP, 0x10));
                    c.g[AT][0] = 0x4348_0000;
                    c.mtc1(AT, 2);
                    c.g[T1][0] = sx(at(c, SP, 0x110));
                    c.mul_s(0, 0, 1);
                    c.sw(at(c, SP, 4), 0);
                    c.swc1(2, r(c, SP));
                    c.g[A1][0] = sx(at(c, SP, 0xb0));
                    c.sw(at(c, SP, 8), 0);
                    c.g[A3][0] = u64::MAX;
                    c.sw(at(c, SP, 12), 0);
                    c.mfc1(V0, 0);
                    c.lqc2(4, r(c, SP));
                    c.qmtc2(V0, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, r(c, SP));
                    pc = EMPTY_CONSTRUCTORS;
                }
                EMPTY_CONSTRUCTORS => {
                    // The eight Vec4 constructors are empty; the original
                    // keeps this eight-iteration counter loop and its cycles.
                    c.cycles += 7;
                    c.g[A2][0] = sx(r(c, A2).wrapping_sub(1));
                    let more = c.g[A2][0] != c.g[A3][0];
                    c.g[V0][0] = 0x0045_0000;
                    if more {
                        if event_due(c) { deliver_event_here(c, EMPTY_CONSTRUCTORS); }
                        continue;
                    }
                    c.cycles += 3;
                    c.g[A2][0] = c.g[V1][0];
                    c.g[T0][0] = sx(OFFSET_WEIGHTS);
                    c.g[A3][0] = 7;
                    pc = SCALE_OFFSETS;
                }
                SCALE_OFFSETS => {
                    c.cycles += 14;
                    c.lwc1(0, r(c, T0));
                    c.g[V1][0] = c.g[A2][0];
                    c.mfc1(V0, 0);
                    c.lqc2(4, r(c, SP));
                    c.qmtc2(V0, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0xa0));
                    c.g[A2][0] = sx(at(c, A2, 0x10));
                    c.lq(V0, at(c, SP, 0xa0));
                    c.g[T0][0] = sx(at(c, T0, 4));
                    c.g[A3][0] = sx(r(c, A3).wrapping_sub(1));
                    c.sq(V0, at(c, SP, 0x90));
                    let more = (c.g[A3][0] as i64) >= 0;
                    c.sq(V0, r(c, V1));
                    if more {
                        if event_due(c) { deliver_event_here(c, SCALE_OFFSETS); }
                        continue;
                    }
                    c.cycles += 7;
                    c.g[A2][0] = c.lw(at(c, A0, MODE));
                    c.g[V0][0] = 15;
                    c.g[V1][0] = u64::MAX;
                    c.g[V0][0] = 1;
                    let move_both = c.g[A2][0] == c.g[V0][0];
                    c.g[A3][0] = c.g[T1][0];
                    if move_both {
                        c.cycles += 5;
                        c.g[V1][0] = sx(at(c, SP, 0x10));
                        c.g[V0][0] = sx(at(c, A0, BOTTOM));
                        c.g[A2][0] = sx(at(c, SP, 0xb0));
                        c.g[T0][0] = 7;
                        pc = MOVE_BOTH_ROWS;
                    } else {
                        c.cycles += 4;
                        c.g[A2][0] = sx(at(c, SP, 0xb0));
                        c.g[V1][0] = sx(at(c, SP, 0x10));
                        c.g[V0][0] = sx(at(c, A0, TOP));
                        c.g[T0][0] = 7;
                        pc = MOVE_TOP_ROW;
                    }
                }
                MOVE_BOTH_ROWS => {
                    c.cycles += 42;
                    c.lwc1(4, r(c, V1));
                    c.g[T0][0] = sx(r(c, T0).wrapping_sub(1));
                    c.lwc1(3, at(c, V1, 4));
                    c.lwc1(6, at(c, V1, 8));
                    c.lwc1(0, r(c, V0).wrapping_sub(0x60));
                    c.lwc1(1, r(c, V0).wrapping_sub(0x5c));
                    c.lwc1(2, r(c, V0).wrapping_sub(0x58));
                    c.add_s(0, 0, 4);
                    c.add_s(1, 1, 3);
                    c.lwc1(5, r(c, V0));
                    c.add_s(2, 2, 6);
                    c.lwc1(4, at(c, V0, 4));
                    c.swc1(0, at(c, SP, 0x90));
                    c.swc1(1, at(c, SP, 0x94));
                    c.swc1(2, at(c, SP, 0x98));
                    copy_point(c, A2);
                    c.g[A2][0] = sx(at(c, A2, 12));
                    c.lwc1(1, at(c, V0, 8));
                    c.lwc1(2, r(c, V1));
                    c.g[V0][0] = sx(at(c, V0, 12));
                    c.lwc1(0, at(c, V1, 4));
                    c.lwc1(3, at(c, V1, 8));
                    c.add_s(5, 5, 2);
                    c.add_s(4, 4, 0);
                    c.g[V1][0] = sx(at(c, V1, 0x10));
                    c.add_s(1, 1, 3);
                    c.swc1(5, at(c, SP, 0x90));
                    c.swc1(4, at(c, SP, 0x94));
                    c.swc1(1, at(c, SP, 0x98));
                    copy_point(c, A3);
                    let more = (c.g[T0][0] as i64) >= 0;
                    c.g[A3][0] = sx(at(c, A3, 12));
                    if more {
                        if event_due(c) { deliver_event_here(c, MOVE_BOTH_ROWS); }
                        continue;
                    }
                    c.cycles += 2;
                    c.g[A2][0] = c.lw(at(c, A0, MESH));
                    pc = UPLOAD_POINTS;
                }
                MOVE_TOP_ROW => {
                    c.cycles += 30;
                    c.lwc1(5, r(c, V1));
                    c.g[T0][0] = sx(r(c, T0).wrapping_sub(1));
                    c.lwc1(3, at(c, V1, 4));
                    c.lwc1(4, at(c, V1, 8));
                    c.lwc1(2, r(c, V0));
                    c.g[V1][0] = sx(at(c, V1, 0x10));
                    c.lwc1(0, at(c, V0, 4));
                    c.lwc1(1, at(c, V0, 8));
                    c.add_s(2, 2, 5);
                    c.add_s(0, 0, 3);
                    c.add_s(1, 1, 4);
                    c.swc1(2, at(c, SP, 0x90));
                    c.swc1(0, at(c, SP, 0x94));
                    c.swc1(1, at(c, SP, 0x98));
                    copy_point(c, A2);
                    c.ldl(T1, at(c, V0, 0x67));
                    c.ldr(T1, at(c, V0, 0x60));
                    c.g[T2][0] = c.lw(at(c, V0, 0x68));
                    c.sdl(T1, at(c, A3, 7));
                    c.sdr(T1, r(c, A3));
                    c.sw(at(c, A3, 8), c.g[T2][0]);
                    c.g[A2][0] = sx(at(c, A2, 12));
                    c.g[A3][0] = sx(at(c, A3, 12));
                    let more = (c.g[T0][0] as i64) >= 0;
                    c.g[V0][0] = sx(at(c, V0, 12));
                    if more {
                        if event_due(c) { deliver_event_here(c, MOVE_TOP_ROW); }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A2][0] = c.lw(at(c, A0, MESH));
                    pc = UPLOAD_POINTS;
                }
                UPLOAD_POINTS => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(at(c, A2, 4));
                    c.g[A0][0] = c.lh(at(c, V0, 0x18));
                    c.g[V1][0] = c.lw(at(c, V0, 0x1c));
                    let target = r(c, V1);
                    c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                    call_ptr(c, target, RETURN)?;
                    pc = RETURN;
                }
                RETURN => {
                    c.cycles += 3;
                    frame.restore_ra(c, 0x170);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()) }
            }
        }
    });
}
