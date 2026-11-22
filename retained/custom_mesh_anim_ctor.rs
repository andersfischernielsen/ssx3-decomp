//! Native MeshAnim constructor (0x00351b40), retained uninstalled.
//!
//! No automated tests or replays were added or run.
//! Generated EE is the authority for registers, cycles and call order.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0035_1b40, mesh_anim_ctor, "MeshAnim_ctor_2")];

const INSTANCE: u32 = 0x18;
const INSTANCE_FLAGS: u32 = 8;
const INSTANCE_MDR: u32 = 0x80;
const INSTANCE_SCALE: u32 = 0x84;
const DURATION: u32 = 0x1c;
const ANGULAR_STEP: u32 = 0x20;
const PARAMETER_30: u32 = 0x24;
const PART_QUATERNIONS: u32 = 0x30;
const VECTOR_A: u32 = 0x390;
const VECTOR_B: u32 = 0x3a0;
const PART_POSITIONS: u32 = 0x3b0;
const MODE: u32 = 0x6b0;
const FRAME_STEP: u32 = 0x6b4;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Invoke one of the source owner's matrix virtuals. The caller charges
/// the original block and consumes the result before the following call.
fn owner_matrix(c: &mut Cpu, slot: u32, back: u32) -> Flow {
    c.g[V1][0] = c.lw(at(c, S0, 0xc));
    c.g[A0][0] = c.lh(at(c, V1, slot));
    c.g[V0][0] = c.lw(at(c, V1, slot + 4));
    let target = r(c, V0);
    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
    call_ptr(c, target, back)
}

/// Build a scaled local bone matrix and compose it with its parent/root.
/// The two original branches use opposite f6/f7 scale registers and their
/// own output registers. Keep the original COP2 operations and scratch copies.
fn build_bone_matrix(c: &mut Cpu, root: bool) {
    c.cycles += if root { 75 } else { 77 };
    c.g[A0][0] = c.lw(at(c, A2, 0xc));
    let (fx, fy) = if root { (6, 7) } else { (7, 6) };
    c.lwc1(fx, at(c, SP, 0x600));
    if !root {
        c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, SP)));
    }
    c.lq(V0, r(c, A0));
    c.lwc1(fy, at(c, SP, 0x604));
    c.sq(V0, at(c, SP, 0x690));
    c.sq(V0, at(c, SP, 0x650));
    c.lwc1(5, at(c, SP, 0x608));
    c.lq(V0, at(c, A0, 0x10));
    c.lwc1(4, at(c, SP, 0x60c));
    c.sq(V0, at(c, SP, 0x690));
    c.sq(V0, at(c, SP, 0x660));
    c.lq(V1, at(c, A0, 0x20));
    c.sq(V1, at(c, SP, 0x690));
    c.sq(V1, at(c, SP, 0x670));
    c.lq(V0, at(c, A0, 0x30));
    c.sq(V0, at(c, SP, 0x6a0));
    c.lwc1(1, at(c, SP, 0x6a0));
    c.lwc1(2, at(c, SP, 0x6a4));
    c.lwc1(3, at(c, SP, 0x6a8));
    c.mul_s(1, 1, fx);
    c.lwc1(0, at(c, SP, 0x6ac));
    c.mul_s(2, 2, fy);
    c.mul_s(3, 3, 5);
    c.mul_s(0, 0, 4);
    c.swc1(1, at(c, SP, 0x690));
    c.swc1(2, at(c, SP, 0x694));
    c.swc1(3, at(c, SP, 0x698));
    c.swc1(0, at(c, SP, 0x69c));
    c.lq(V0, at(c, SP, 0x690));
    c.sq(V0, at(c, SP, 0x680));
    let parent = r(c, if root { S2 } else { A1 });
    c.lqc2(4, parent);
    c.lqc2(5, parent.wrapping_add(0x10));
    c.lqc2(6, parent.wrapping_add(0x20));
    c.lqc2(7, parent.wrapping_add(0x30));
    c.lqc2(8, r(c, T4));
    c.lqc2(9, at(c, T4, 0x10));
    c.lqc2(10, at(c, T4, 0x20));
    c.lqc2(11, at(c, T4, 0x30));
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
    c.sqc2(12, r(c, A3));
    c.sqc2(13, at(c, A3, 0x10));
    c.sqc2(14, at(c, A3, 0x20));
    c.sqc2(15, at(c, A3, 0x30));
    for row in 0..4 {
        c.lqc2(1 + row, at(c, A3, row as u32 * 16));
    }
    for row in 0..4 {
        c.sqc2(1 + row, at(c, T0, row as u32 * 16));
    }
    for row in 0..4 {
        c.lqc2(1 + row, at(c, T0, row as u32 * 16));
    }
    let destination = if root { T2 } else { T1 };
    for row in 0..4 {
        c.sqc2(1 + row, at(c, destination, row as u32 * 16));
    }
    c.g[V0][0] = c.lw(at(c, S5, 4));
}

/// `MeshAnim* MeshAnim_ctor(self, process_arg, Instance*, Params*,
/// ContactB*, previous_owner)` (0x351b40).
///
/// Raw params: +4 mode -> self+6b0; +8 frame step -> +6b4; +10 duration ->
/// +1c; +2c angular speed in degrees/sec -> radians/tick at +20; +30 ->
/// +24. Vectors +14 and +20 become self+390/+3a0; only the latter receives
/// contact.axis(+10) * abs(contact.scalar(+30)) * params(+c).
/// +34 optionally selects another Instance RID's owner; +38 optionally
/// selects one matrix of that owner's skeleton as the root. Otherwise use
/// the instance root and compose every native MDR part's hierarchy, then
/// decompose its matrix into the piece's position and quaternion.
/// The prior owner passed in T1 is deleted after capturing those matrices.
/// Finally convert the instance to dynamic ownership, clear instance bits
/// 0x60 and invoke native MeshAnim_RandomizeNodes. The constructor reserves
/// fixed arrays for 24 parts; it does not validate the MDR part count here.
pub fn mesh_anim_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x790);
        frame.save(c, S4, 0x730);
        frame.save(c, S3, 0x740);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S6, 0x710);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S7, 0x700);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S0, 0x770);
        frame.save(c, S1, 0x760);
        c.g[S7][0] = c.g[T0][0];
        frame.save(c, S2, 0x750);
        c.g[S6][0] = c.g[T1][0];
        frame.save(c, S5, 0x720);
        c.g[A2][0] = 11;
        frame.save_ra(c, 0x6f0);
        frame.save_fpr(c, 20, 0x780);
        call_ptr(c, 0x0034_fb00, 0x0035_1b88)?;
        c.g[V0][0] = 0x0049_0000;
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = sx(0x0048_f6d8);
        c.g[V1][0] = 0x17;
        c.sw(at(c, S3, 0xc), c.g[V0][0]);

        // Native placement-array constructors are empty, but retain their
        // counted loop, scratch registers and event-delivery sites.
        loop {
            c.cycles += 7;
            c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A0][0];
            c.g[V0][0] = 0x17;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_1ba0);
            }
        }
        c.cycles += 25;
        c.g[A0][0] = c.lw(at(c, S4, 4));
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0x17;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0x17;
        c.g[V1][0] = u64::MAX;
        c.sw(at(c, S3, MODE), c.g[A0][0]);
        c.lwc1(2, r(c, GP).wrapping_sub(0x2a8c));
        c.lwc1(0, at(c, S4, 0x10));
        c.swc1(0, at(c, S3, DURATION));
        c.lwc1(1, at(c, S4, 0x30));
        c.swc1(1, at(c, S3, PARAMETER_30));
        c.lwc1(0, at(c, S4, 8));
        c.swc1(0, at(c, S3, FRAME_STEP));
        c.lwc1(20, at(c, S4, 0x2c));
        c.mul_s(20, 20, 2);
        call_ptr(c, 0x002d_1c70, 0x0035_1c04)?; // World_GetSecondsPerTick
        c.g[V0][0] = c.lw(at(c, S3, INSTANCE));
        c.g[A0][0] = u64::MAX;
        c.mul_s(20, 20, 0);
        c.g[V1][0] = 0x17;
        c.swc1(20, at(c, S3, ANGULAR_STEP));
        c.g[S5][0] = c.lw(at(c, V0, INSTANCE_MDR));
        loop {
            c.cycles += 7;
            c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A0][0];
            c.g[S1][0] = c.g[SP][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_1c20);
            }
        }

        c.cycles += 6;
        c.g[V1][0] = c.lw(at(c, S3, INSTANCE));
        c.g[V0][0] = c.lw(at(c, S4, 0x34));
        c.g[S0][0] = c.g[S6][0];
        c.g[V0][0] = !c.g[V0][0];
        let select_other_instance = c.g[V0][0] != 0;
        c.g[S2][0] = sx(at(c, V1, 0x10));
        if select_other_instance {
            c.cycles += 11;
            call_ptr(c, 0x002d_1bd8, 0x0035_1c5c)?; // World_GetInstance
            c.g[V1][0] = c.lw(r(c, V0));
            c.g[A1][0] = c.lw(at(c, S4, 0x34));
            c.g[A0][0] = c.lw(at(c, V1, 8));
            c.g[V0][0] = c.g[A1][0] & 255;
            c.g[V0][0] = sx(r(c, V0) << 2);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
            c.g[V1][0] = c.lw(r(c, V0));
            let has_bank = c.g[V1][0] != 0;
            c.g[V0][0] = sx(r(c, A1) >> 8);
            let mut resident = false;
            if has_bank {
                c.cycles += 7;
                c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                c.g[V0][0] = sx(r(c, V0) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = c.lw(r(c, V0));
                c.g[V0][0] = sx(r(c, V1) >> 8);
                resident = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r(c, V0) << 2);
            }
            if !resident {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw(at(c, V0, 0xc));
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = c.g[V0][0];
                }
            }
        }
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 13;
            owner_matrix(c, 0xc0, 0x0035_1ccc)?;
            c.g[V1][0] = c.lw(at(c, S0, 0xc));
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = c.lh(at(c, V1, 0xd0));
            c.g[V0][0] = c.lw(at(c, V1, 0xd4));
            let target = r(c, V0);
            c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0035_1ce4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 10;
                owner_matrix(c, 0xd8, 0x0035_1d00)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(at(c, S4, 0x38));
                c.g[V0][0] = u64::MAX;
                let select_bone = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = sx(r(c, V1) << 6);
                if select_bone {
                    c.cycles += 2;
                    c.g[S2][0] = sx(r(c, S1).wrapping_add(r(c, V0)));
                    c.g[S1][0] = c.g[SP][0];
                }
            }
        }

        c.cycles += 2;
        if c.g[S1][0] != c.g[SP][0] {
            c.g[V0][0] = c.lw(at(c, S5, 4));
        } else {
            c.cycles += 12;
            c.g[V0][0] = c.lw(at(c, S3, INSTANCE));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1);
            c.lwc1(0, at(c, V0, INSTANCE_SCALE));
            c.g[A2][0] = c.lw(at(c, S5, 8));
            c.swc1(0, at(c, SP, 0x608));
            c.swc1(1, at(c, SP, 0x60c));
            c.swc1(0, at(c, SP, 0x600));
            c.swc1(0, at(c, SP, 0x604));
            c.g[V0][0] = c.lw(at(c, S5, 4));
            let have_parts = (c.g[V0][0] as i64) > 0;
            c.g[T3][0] = 0;
            if have_parts {
                c.cycles += 7;
                c.g[T5][0] = u64::MAX;
                c.g[T0][0] = sx(at(c, SP, 0x610));
                c.g[A3][0] = sx(at(c, SP, 0x6b0));
                c.g[T4][0] = sx(at(c, SP, 0x650));
                c.g[T2][0] = c.g[SP][0];
                c.g[T1][0] = c.g[SP][0];
                loop {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw(r(c, A2));
                    let root = c.g[A1][0] == c.g[T5][0];
                    c.g[A1][0] = sx(r(c, A1) << 6);
                    build_bone_matrix(c, root);
                    c.cycles += 6;
                    c.g[T3][0] = sx(r(c, T3).wrapping_add(1));
                    c.g[T2][0] = sx(at(c, T2, 0x40));
                    c.g[T1][0] = sx(at(c, T1, 0x40));
                    c.g[V0][0] = ((c.g[T3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A2][0] = sx(at(c, A2, 0x10));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_1d70);
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, S5, 4));
            }
        }

        c.cycles += 2;
        let have_parts = (c.g[V0][0] as i64) > 0;
        c.g[S2][0] = 0;
        if have_parts {
            c.cycles += 2;
            c.g[S0][0] = c.g[S3][0];
            loop {
                c.cycles += 13;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = sx(at(c, SP, 0x600));
                call_ptr(c, 0x0031_b748, 0x0035_2014)?; // Mat4_ToPosQuat
                c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
                c.lq(V0, at(c, SP, 0x600));
                c.g[S1][0] = sx(at(c, S1, 0x40));
                c.lq(V1, at(c, SP, 0x610));
                c.sq(V0, at(c, S0, PART_POSITIONS));
                c.sq(V1, at(c, S0, PART_QUATERNIONS));
                c.g[V0][0] = c.lw(at(c, S5, 4));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(at(c, S0, 0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_2008);
                }
            }
        }
        c.cycles += 2;
        let release_owner = c.g[S6][0] != 0;
        c.g[A1][0] = 3;
        if release_owner {
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(c, S6, 0xc));
            c.g[A0][0] = c.lh(at(c, V0, 8));
            c.g[V1][0] = c.lw(at(c, V0, 0xc));
            let target = r(c, V1);
            c.g[A0][0] = sx(r(c, S6).wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0035_2058)?;
        }

        c.cycles += 19;
        c.lwc1(0, at(c, S4, 0x14));
        c.lwc1(1, at(c, S4, 0x18));
        c.lwc1(2, at(c, S4, 0x1c));
        c.swc1(0, at(c, SP, 0x600));
        c.swc1(1, at(c, SP, 0x604));
        c.swc1(2, at(c, SP, 0x608));
        c.sw(at(c, SP, 0x60c), 0);
        c.lq(V0, at(c, SP, 0x600));
        c.sw(at(c, SP, 0x60c), 0);
        c.sq(V0, at(c, S3, VECTOR_A));
        c.lwc1(0, at(c, S4, 0x20));
        c.lwc1(1, at(c, S4, 0x24));
        c.lwc1(2, at(c, S4, 0x28));
        c.swc1(0, at(c, SP, 0x600));
        c.swc1(1, at(c, SP, 0x604));
        c.swc1(2, at(c, SP, 0x608));
        c.lq(V0, at(c, SP, 0x600));
        let have_contact = c.g[S7][0] != 0;
        c.sq(V0, at(c, S3, VECTOR_B));
        if have_contact {
            c.cycles += 21;
            c.lwc1(0, at(c, S7, 0x30));
            c.abs_s(0, 0);
            c.mfc1(V1, 0);
            c.lqc2(4, at(c, S7, 0x10));
            c.qmtc2(V1, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(c, SP, 0x620));
            c.lq(V0, at(c, SP, 0x620));
            c.lwc1(0, at(c, S4, 0xc));
            c.sq(V0, at(c, SP, 0x610));
            c.mfc1(V1, 0);
            c.lqc2(4, at(c, SP, 0x610));
            c.qmtc2(V1, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(c, SP, 0x620));
            c.lq(V0, at(c, SP, 0x620));
            c.sq(V0, at(c, SP, 0x600));
            c.lqc2(3, at(c, S3, VECTOR_B));
            c.lqc2(4, at(c, SP, 0x600));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(c, S3, VECTOR_B));
        }
        c.cycles += 28;
        c.g[A2][0] = c.lw(at(c, S3, INSTANCE));
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] |= 0xfffd;
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = c.lw(at(c, A2, INSTANCE_FLAGS));
        c.g[A1][0] |= 0xff9f;
        c.g[A0][0] = c.g[S3][0];
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= 4;
        c.sw(at(c, A2, INSTANCE_FLAGS), c.g[V0][0]);
        c.g[V1][0] = c.lw(at(c, S3, INSTANCE));
        c.g[V0][0] = c.lw(at(c, V1, INSTANCE_FLAGS));
        c.g[V0][0] &= c.g[A1][0];
        c.sw(at(c, V1, INSTANCE_FLAGS), c.g[V0][0]);
        call_ptr(c, 0x0035_2230, 0x0035_2134)?; // MeshAnim_RandomizeNodes
        c.g[V0][0] = c.g[S3][0];
        for (register, offset) in [
            (S0, 0x770),
            (S1, 0x760),
            (S2, 0x750),
            (S3, 0x740),
            (S4, 0x730),
            (S5, 0x720),
            (S6, 0x710),
            (S7, 0x700),
        ] {
            frame.restore(c, register, offset);
        }
        frame.restore_ra(c, 0x6f0);
        frame.restore_fpr(c, 20, 0x780);
        frame.close(c);
        Ok(())
    })
}
