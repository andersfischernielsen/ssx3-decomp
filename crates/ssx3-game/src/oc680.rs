//! Staging module oc680 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc680`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::game::camera::camera::{camera_frame_at, camera_gp_f32, mid_chase_epilogue};
use crate::game::luno::lunovm::oc640_field;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x00168d80,
        oc680_mid_left_chase_camera_update,
        "MidLeftChaseCameraAlgorithm_Update",
    ),
    (
        0x00171fa8,
        oc680_camera_trigger_find_boundary_point,
        "CameraTrigger_FindBoundaryPoint",
    ),
    (
        0x001732e8,
        oc680_bounded_camera_algorithm_base_reset_for_rider,
        "BoundedCameraAlgorithmBase_ResetForRider",
    ),
    (
        0x0017ae00,
        oc680_debug_menu_draw_background,
        "DebugMenu_DrawBackground",
    ),
];

/// `cMidLeftChaseAlgorithm_update(self)` (0x168d80): the left mid chase
/// camera's per-frame update. The same chase pipeline as the right one
/// (`mid_right_chase_camera_update`), with the left camera's gp constants
/// and return addresses.
pub fn oc680_mid_left_chase_camera_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 120;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0xa0));
        camera_gp_f32(c, 19, 0x65f8);
        let a80 = camera_frame_at(c, 0x80);
        c.sq(16, a80);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        camera_gp_f32(c, 12, 0x65f4);
        camera_gp_f32(c, 13, 0x65f0);
        camera_gp_f32(c, 14, 0x65ec);
        camera_gp_f32(c, 15, 0x65e8);
        camera_gp_f32(c, 16, 0x65e4);
        camera_gp_f32(c, 17, 0x65e0);
        camera_gp_f32(c, 18, 0x65dc);
        let a70 = camera_frame_at(c, 0x70);
        c.sd(a70, c.g[RA][0]);
        let a98 = camera_frame_at(c, 0x98);
        c.swc1(21, a98);
        let a90 = camera_frame_at(c, 0x90);
        c.swc1(20, a90);
        // setupChaseData(self, &info, the eight tuning floats)
        call_ptr(c, 0x0016_2568, 0x0016_8dc0)?;
        c.mtc1(0, 20);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2998, 0x0016_8dd0)?;
        camera_gp_f32(c, 21, 0x65d8);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2a20, 0x0016_8de0)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b80, 0x0016_8dec)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2c78, 0x0016_8df8)?;
        camera_gp_f32(c, 17, 0x65d4);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 18, 0x65d0);
        camera_gp_f32(c, 12, 0x65cc);
        camera_gp_f32(c, 13, 0x65c8);
        camera_gp_f32(c, 14, 0x65c4);
        camera_gp_f32(c, 15, 0x65c0);
        camera_gp_f32(c, 16, 0x65bc);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3010, 0x0016_8e20)?;
        camera_gp_f32(c, 16, 0x65b8);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 12, 0x65b4);
        camera_gp_f32(c, 13, 0x65b0);
        camera_gp_f32(c, 14, 0x65ac);
        camera_gp_f32(c, 15, 0x65a8);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3270, 0x0016_8e40)?;
        camera_gp_f32(c, 13, 0x65a4);
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 14);
        c.g[A0][0] = c.g[S0][0];
        camera_gp_f32(c, 15, 0x65a0);
        c.mov_s(12, 20);
        call_ptr(c, 0x0016_33b0, 0x0016_8e60)?;
        camera_gp_f32(c, 18, 0x659c);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 19, 0x6598);
        c.mov_s(17, 21);
        camera_gp_f32(c, 12, 0x6594);
        c.mov_s(16, 20);
        camera_gp_f32(c, 13, 0x6590);
        camera_gp_f32(c, 14, 0x658c);
        camera_gp_f32(c, 15, 0x6588);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3450, 0x0016_8e8c)?;
        camera_gp_f32(c, 16, 0x6584);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 12, 0x6580);
        camera_gp_f32(c, 13, 0x657c);
        camera_gp_f32(c, 14, 0x6578);
        camera_gp_f32(c, 15, 0x6574);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b90, 0x0016_8eac)?;
        camera_gp_f32(c, 14, 0x6570);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 15, 0x656c);
        camera_gp_f32(c, 13, 0x6568);
        camera_gp_f32(c, 12, 0x6564);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_35f8, 0x0016_8ec8)?;
        camera_gp_f32(c, 12, 0x6560);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_4878, 0x0016_8ed8)?;
        camera_gp_f32(c, 12, 0x655c);
        c.g[A1][0] = c.g[SP][0];
        camera_gp_f32(c, 13, 0x6558);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_46a0, 0x0016_8eec)?;
        c.g[AT][0] = 0x4416_0000;
        c.mtc1(1, 12);
        c.mov_s(15, 21);
        c.g[AT][0] = 0x4396_0000;
        c.mtc1(1, 13);
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(1, 14);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_41c0, 0x0016_8f14)?;
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(1, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(1, 12);
        c.g[A1][0] = c.g[SP][0];
        c.mov_s(14, 13);
        call_ptr(c, 0x0016_43a8, 0x0016_8f34)?;
        camera_gp_f32(c, 12, 0x6554);
        c.g[A0][0] = c.g[S0][0];
        camera_gp_f32(c, 13, 0x6550);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0016_3158, 0x0016_8f48)?;
        mid_chase_epilogue(c);
        Ok(())
    })
}

/// `t4Vector* CameraTrigger_FindBoundaryPoint(out, vol, pos, step)`
/// (0x171fa8; `cCTVolume_getBoundaryPointClosestTo`): finds the point on the
/// trigger volume's boundary along the ray from the volume's centre through
/// `pos`. The centre is the midpoint of the volume's bounding box (vtable slot
/// 6, `boundBox`); starting at `pos`, the probe doubles its distance from the
/// centre until the volume's `containsPoint` (slot 7) says outside, then
/// `ceil(log(len / step)) + 1` bisection steps between the last inside and the
/// first outside point narrow it down. Returns `out` (the outside point).
///
/// The body is resumable in the original (a due event at a loop back edge
/// unwinds and re-enters at the loop head), so a `pc` other than the entry
/// runs the generated body; at install, name the generated twin
/// `f_00171fa8__generated` there.
pub fn oc680_camera_trigger_find_boundary_point(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{W, Y, Z};
    if c.pc != 0x0017_1fa8 {
        return ee::s02::f_00171fa8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x80);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0xb8);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xb0);
        c.g[A0][0] = sx(frame.at(c, 0x10));
        frame.save_ra(c, 0x70);
        c.mov_s(21, 12);
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 20);
        // volume->boundBox(out = sp+0x10 (min), sp+0x20 (max))
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0017_1ff4)?;
        // centre = (min + max) * 0.5, at sp+0
        c.lqc2(3, frame.at(c, 0x10));
        c.lqc2(4, frame.at(c, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x40));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 0);
        c.lq(2, frame.at(c, 0x40));
        c.sq(2, frame.at(c, 0x30));
        c.mfc1(3, 0);
        c.lqc2(4, frame.at(c, 0x30));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x40));
        c.lq(2, frame.at(c, 0x40));
        c.sq(2, frame.at(c, 0));
        // dir = pos - centre (sp+0x10), outside = pos (sp+0x20), inside = centre (sp+0x30)
        c.lqc2(3, c.g[S0][0] as u32);
        c.lqc2(4, frame.at(c, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x20));
        c.lq(4, c.g[S0][0] as u32);
        c.lq(2, frame.at(c, 0x20));
        c.lq(3, frame.at(c, 0));
        c.sq(2, frame.at(c, 0x10));
        c.sq(4, frame.at(c, 0x20));
        c.sq(3, frame.at(c, 0x30));
        c.g[S0][0] = sx(frame.at(c, 0x20));
        // Grow: while volume->containsPoint(&outside), inside = outside,
        // outside = centre + dir * k, k *= 2.
        loop {
            c.cycles += 8;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0017_20b8)?;
            let inside = c.g[V0][0] != 0;
            c.lq(2, frame.at(c, 0x20));
            if !inside {
                break;
            }
            if back_edge_unwinds(c, 0x0017_2060) {
                return Ok(());
            }
            c.cycles += 16;
            c.sq(2, frame.at(c, 0x30));
            c.mfc1(3, 20);
            c.lqc2(4, frame.at(c, 0x10));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x50));
            c.lq(2, frame.at(c, 0x50));
            c.add_s(20, 20, 20);
            c.sq(2, frame.at(c, 0x40));
            c.lqc2(3, frame.at(c, 0));
            c.lqc2(4, frame.at(c, 0x40));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x60));
            c.lq(2, frame.at(c, 0x60));
            c.sq(2, frame.at(c, 0x20));
            c.sq(2, frame.at(c, 0x50));
        }
        // steps = (int)logf(|outside - inside| / step) + 1
        c.cycles += 24;
        c.lqc2(3, frame.at(c, 0x20));
        c.lqc2(4, frame.at(c, 0x30));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x50));
        c.lq(2, frame.at(c, 0x50));
        c.sq(2, frame.at(c, 0x40));
        c.lqc2(3, frame.at(c, 0x40));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 12);
        c.div_s(12, 12, 21);
        call_ptr(c, 0x0040_da10, 0x0017_210c)?;
        c.cvt_w_s(1, 0);
        c.mfc1(3, 1);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        let no_steps = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = c.g[V0][0];
        if !no_steps {
            c.cycles += 2;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 20);
            // Bisect: mid = (outside + inside) * 0.5; inside or outside = mid.
            loop {
                c.cycles += 21;
                c.lqc2(3, frame.at(c, 0x20));
                c.lqc2(4, frame.at(c, 0x30));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x60));
                c.g[A1][0] = sx(frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x60));
                c.sq(2, frame.at(c, 0x50));
                c.mfc1(3, 20);
                c.lqc2(4, frame.at(c, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x60));
                c.lq(2, frame.at(c, 0x60));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.sq(2, frame.at(c, 0x40));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0017_2174)?;
                let outside = c.g[V0][0] == 0;
                c.lq(2, frame.at(c, 0x40));
                if outside {
                    c.cycles += 1;
                    c.sq(2, frame.at(c, 0x20));
                } else {
                    c.cycles += 2;
                    c.sq(2, frame.at(c, 0x30));
                }
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                if c.g[S0][0] == 0 {
                    break;
                }
                if back_edge_unwinds(c, 0x0017_2128) {
                    return Ok(());
                }
            }
        }
        // *out = outside
        c.cycles += 11;
        c.lq(3, frame.at(c, 0x20));
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0xa0);
        c.sq(3, c.g[S2][0] as u32);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `void BoundedCameraAlgorithmBase_ResetForRider(self, key)` (0x1732e8;
/// reference `cTriggerAlgorithm::snap`): re-aims the trigger camera for the
/// rider with AI index `key` (nothing happens for another rider). Reads the
/// target's position and front vector through its vtable, offsets the focus
/// by `HeightOffset` (+0x6c) along the world Z axis and by `LeadDistance`
/// (+0x74) along the front, derives `Heading` (+0x50) and `Pitch` (+0x54)
/// from the front (two inlined `atan2`s with their quadrant fixes and the
/// angle wrap), places the camera at `focus - dir * IdealDist` (+0x40, from
/// the heading and pitch sin/cos) and calls `update()` (vtable slot at +0x28).
/// The body is the generated labelled-block structure: every register, cycle
/// and call is kept.
pub fn oc680_bounded_camera_algorithm_base_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        'b_17365c: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S0][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xa8));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0017332c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            call_ptr(c, t, 0x0017332c)?;
            let t = c.g[S0][0] != c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            if t {
                break 'b_17365c;
            } else {
                'b_17346c: {
                    'b_17342c: {
                        c.cycles += 51;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.g[S0][0];
                        c.mtc1(0, 22);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let t = c.g[A2][0] as u32;
                        c.g[RA][0] = sx(0x00173358);
                        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        call_ptr(c, t, 0x00173358)?;
                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                        c.sq(3, (c.g[SP][0] as u32));
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0017337c);
                        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        call_ptr(c, t, 0x0017337c)?;
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x6c));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = 0x500000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
                        c.mfc1(3, 0);
                        c.lqc2(4, (c.g[V0][0] as u32));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lqc2(3, (c.g[SP][0] as u32));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32));
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x74));
                        c.mfc1(3, 0);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lqc2(3, (c.g[SP][0] as u32));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32));
                        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.c_eq_s(20, 22);
                        let t = !c.fcc();
                        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_17342c;
                        } else {
                            c.cycles += 4;
                            c.c_eq_s(21, 22);
                            let t = c.fcc();
                            c.mov_s(12, 21);
                            if t {
                                break 'b_17346c;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(22, 21);
                                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6428));
                                let t = !c.fcc();

                                if t {
                                    break 'b_17346c;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6424));
                                    let t = true;

                                    if t {
                                        break 'b_17346c;
                                    } else {
                                        break 'b_17342c;
                                    }
                                }
                            }
                        }
                    }
                    'b_173468: {
                        c.cycles += 7;
                        c.div_s(12, 21, 20);
                        c.g[RA][0] = sx(0x00173438);
                        call_ptr(c, 0x0031c228, 0x00173438)?;
                        c.c_lt_s(20, 22);
                        let t = !c.fcc();
                        c.mov_s(2, 0);
                        if t {
                            break 'b_173468;
                        } else {
                            'b_173460: {
                                c.cycles += 3;
                                c.c_lt_s(22, 21);
                                let t = !c.fcc();

                                if t {
                                    break 'b_173460;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6420));
                                    let t = true;
                                    c.add_s(2, 2, 0);
                                    if t {
                                        break 'b_173468;
                                    } else {
                                        break 'b_173460;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x641c));
                            c.sub_s(2, 2, 0);
                            break 'b_173468;
                        }
                    }
                    c.cycles += 1;
                    c.mov_s(12, 2);
                    break 'b_17346c;
                }
                'b_1734a8: {
                    c.cycles += 11;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6418));
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 1);
                    c.mul_s(0, 12, 0);
                    c.add_s(0, 0, 1);
                    c.cvt_w_s(5, 0);
                    c.cvt_s_w(5, 5);
                    c.c_lt_s(0, 5);
                    let t = !c.fcc();
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_1734a8;
                    } else {
                        c.cycles += 4;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.sub_s(5, 5, 0);
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'b_1734a8;
                    }
                }
                'b_17355c: {
                    'b_173558: {
                        'b_173554: {
                            'b_173514: {
                                c.cycles += 16;
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6414));
                                c.mul_s(3, 0, 0);
                                c.mul_s(4, 1, 1);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.mul_s(2, 5, 2);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                                c.mtc1(0, 22);
                                c.add_s(3, 3, 4);
                                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x18));
                                c.sqrt_s(20, 3);
                                c.sub_s(2, 12, 2);
                                c.c_eq_s(20, 22);
                                let t = !c.fcc();
                                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_173514;
                                } else {
                                    c.cycles += 4;
                                    c.c_eq_s(21, 22);
                                    let t = c.fcc();
                                    c.mov_s(12, 21);
                                    if t {
                                        break 'b_173554;
                                    } else {
                                        c.cycles += 4;
                                        c.c_le_s(22, 21);
                                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6410));
                                        let t = !c.fcc();
                                        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x78));
                                        if t {
                                            break 'b_173558;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x640c));
                                            let t = true;
                                            c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x58));
                                            if t {
                                                break 'b_17355c;
                                            } else {
                                                break 'b_173514;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_173550: {
                                c.cycles += 7;
                                c.div_s(12, 21, 20);
                                c.g[RA][0] = sx(0x00173520);
                                call_ptr(c, 0x0031c228, 0x00173520)?;
                                c.c_lt_s(20, 22);
                                let t = !c.fcc();
                                c.mov_s(1, 0);
                                if t {
                                    break 'b_173550;
                                } else {
                                    'b_173548: {
                                        c.cycles += 3;
                                        c.c_lt_s(22, 21);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_173548;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6408));
                                            let t = true;
                                            c.add_s(1, 1, 0);
                                            if t {
                                                break 'b_173550;
                                            } else {
                                                break 'b_173548;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6404));
                                    c.sub_s(1, 1, 0);
                                    break 'b_173550;
                                }
                            }
                            c.cycles += 1;
                            c.mov_s(12, 1);
                            break 'b_173554;
                        }
                        c.cycles += 1;
                        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x78));
                        break 'b_173558;
                    }
                    c.cycles += 1;
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x58));
                    break 'b_17355c;
                }
                'b_1735a8: {
                    c.cycles += 16;
                    c.neg_s(2, 2);
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x6400));
                    c.neg_s(1, 1);
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x63fc));
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 4);
                    c.mul_s(2, 2, 3);
                    c.mul_s(1, 12, 1);
                    c.add_s(1, 1, 2);
                    c.mul_s(0, 1, 0);
                    c.add_s(0, 0, 4);
                    c.cvt_w_s(2, 0);
                    c.cvt_s_w(2, 2);
                    c.c_lt_s(0, 2);
                    let t = !c.fcc();

                    if t {
                        break 'b_1735a8;
                    } else {
                        c.cycles += 3;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.sub_s(2, 2, 0);
                        break 'b_1735a8;
                    }
                }
                c.cycles += 45;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x63f8));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x64));
                c.mul_s(0, 2, 0);
                c.sub_s(0, 1, 0);
                c.g[RA][0] = sx(0x001735c8);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x54));
                call_ptr(c, 0x0031be50, 0x001735c8)?;
                c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x54));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x68));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x6c));
                c.g[RA][0] = sx(0x001735dc);
                c.neg_s(12, 12);
                call_ptr(c, 0x0031be50, 0x001735dc)?;
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x6c));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.mul_s(1, 0, 1);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                c.mul_s(0, 0, 3);
                c.neg_s(2, 2);
                c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x70));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.mfc1(3, 3);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x40));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00173658);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00173658)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                break 'b_17365c;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xa8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// `void DebugMenu_DrawBackground(self)` (0x17ae00; reference
/// `cBackgroundMan::renderImage`): draws the active background image as a
/// full-screen (640x480) textured quad through the graphics manager: sets up
/// the 2D state (render-state words of the context's top entry, an
/// orthographic rectangle, the identity matrix), builds four grey vertices
/// with the corner positions and uvs, sets the alpha and texture of the top
/// entry and submits the strip (vtable slot at +0x238), then pops the context.
/// Does nothing when the active image index (+8) is outside `0..count` (+4).
pub fn oc680_debug_menu_draw_background(c: &mut Cpu) {
    guest(c, |c| {
        'b_17b184: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.g[S7][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
            let t = (c.g[V1][0] as i64) < 0;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
            if t {
                break 'b_17b184;
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                if t {
                    break 'b_17b184;
                } else {
                    c.cycles += 114;
                    c.g[S3][0] = 0xfffffffffffffc1f;
                    c.g[S0][0] = 0xfffffffffe7f0000;
                    c.g[S4][0] = 0x800000;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                    c.g[S0][0] = c.g[S0][0] | 0xffff;
                    c.g[S1][0] = 0xffffffffffcf0000;
                    c.g[S5][0] = 0x300000;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa8));
                    c.g[S1][0] = c.g[S1][0] | 0xffff;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xac));
                    c.g[S2][0] = 0xfffffffffff00000;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017ae80);
                    c.g[S2][0] = c.g[S2][0] | 0xfff;
                    call_ptr(c, t, 0x0017ae80)?;
                    c.g[S6][0] = 0xffffffffffffff83;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xf8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xfc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017ae9c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0017ae9c)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    c.g[A1][0] = 0u64;
                    c.mtc1(0, 12);
                    c.g[A2][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                    c.g[AT][0] = 0x44200000;
                    c.mtc1(1, 14);
                    c.mov_s(13, 12);
                    c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
                    c.ldr(4, (c.g[V0][0] as u32));
                    c.ldl(7, (c.g[V0][0] as u32).wrapping_add(0xf));
                    c.ldr(7, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x1b));
                    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x14));
                    c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x23));
                    c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x1c));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[T0][0]);
                    c.mov_s(16, 12);
                    c.g[AT][0] = 0x43f00000;
                    c.mtc1(1, 15);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 17);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                    c.g[V0][0] = c.g[V0][0] | 0x120;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[S4][0];
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[S5][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
                    c.g[V0][0] = c.g[V0][0] | 0xd000;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[S6][0];
                    c.g[V0][0] = c.g[V0][0] | 0x14;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xd0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0017af80);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0017af80)?;
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    c.g[A1][0] = 0x500000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x110));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x114));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017afa0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0017afa0)?;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    c.g[A3][0] = 0xffffffffffffffff;
                    c.g[A2][0] = 0x3;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                    c.g[V0][0] = c.g[V0][0] | 0x120;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[S4][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[S5][0];
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
                    c.g[V0][0] = c.g[V0][0] | 0xd000;
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[S6][0];
                    c.g[V0][0] = c.g[V0][0] | 0x14;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    'l_17b010: loop {
                        c.cycles += 7;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                        let t = c.g[A2][0] != c.g[A3][0];
                        c.g[A0][0] = 0x80;
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0017b010);
                            }
                            continue 'l_17b010;
                        } else {
                            c.cycles += 5;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.g[V1][0] = c.g[SP][0];
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                            'l_17b040: loop {
                                c.cycles += 8;
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0017b040);
                                    }
                                    continue 'l_17b040;
                                } else {
                                    c.cycles += 73;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 0);
                                    c.g[T1][0] = 0xfffffffffffffc1f;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), 0u64);
                                    c.g[A1][0] = 0x4;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                    c.g[A2][0] = c.g[SP][0];
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), 0u64);
                                    c.g[A3][0] = 0u64;
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xcc));
                                    c.g[AT][0] = 0x44200000;
                                    c.mtc1(1, 2);
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), 0u64);
                                    c.g[AT][0] = 0x43f00000;
                                    c.mtc1(1, 1);
                                    c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), 0u64);
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc4));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                    c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
                                    c.sw((c.g[SP][0] as u32), 0u64);
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x60), 0u64);
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x64));
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                                    c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                                    c.g[V0][0] = c.g[V0][0] | 0x300;
                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lw((c.g[S7][0] as u32));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[T1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                    c.sh((c.g[T1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                    c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0017b144);
                                    c.g[A0][0] =
                                        sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x0017b144)?;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
                                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10d8));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                                    c.sw((c.g[A0][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                                    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x100));
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x104));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x0017b168);
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    call_ptr(c, t, 0x0017b168)?;
                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xb0));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb4));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0017b180);
                                    c.g[A0][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x0017b180)?;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                                    break 'b_17b184;
                                }
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{DATA, GAME_GP};

    /// Runs `body` in a child process whose function table is empty, so every
    /// call the function makes through the table lands in the fake hook
    /// (callees with no override slot cannot be faked in-process). The parent
    /// re-runs the named test in the child and checks that it passed.
    pub(crate) fn oc680_in_child(test: &str, body: &dyn Fn()) {
        const ENV: &str = "SSX3_OC680_CHILD";
        if std::env::var_os(ENV).is_some() {
            ssx3_ee_rt::funcs::register_functions(&[]);
            return body();
        }
        let name = format!(
            "{}::{}",
            module_path!().split_once("::").map_or("", |p| p.1),
            test
        );
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
    }

    /// `cMidLeftChaseAlgorithm_update(self)`: the chase pipeline of 16 calls.
    /// Every callee is a logged fake (empty function table in a child
    /// process), with unwinds at random calls.
    #[test]
    fn oc680_mid_left_chase_camera_update_matches() {
        oc680_in_child("oc680_mid_left_chase_camera_update_matches", &|| {
            covered(
                ee::s02::f_00168d80__generated,
                oc680_mid_left_chase_camera_update,
                120,
                &|c, seed| {
                    let _ = machine(c, seed, 0x0680_0001, 16);
                    c.g[A0][0] = sx(DATA + 0x100);
                },
            );
        });
    }

    /// Control words of the boundary-point fixture: +0 the number of leading
    /// `containsPoint` calls that say inside, +4 the salt of the later answers,
    /// +8 the bits of the `logf` result.
    const OC680_FIND_CTRL: u32 = DATA + 0x20_0000;

    /// The miss hook of the boundary-point test: the volume's `boundBox`
    /// writes a fixed box, `containsPoint` follows the script, `logf` returns
    /// the fixture's value (the plain fake would return garbage that makes the
    /// bisection run for ever); every call is logged as usual.
    fn oc680_find_hook(c: &mut Cpu, target: u32) {
        let out = c.g[A0][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        let (inside_calls, salt, logf) = (
            c.bus.read32(OC680_FIND_CTRL),
            c.bus.read32(OC680_FIND_CTRL + 4),
            c.bus.read32(OC680_FIND_CTRL + 8),
        );
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        if target == fakes::FAKE {
            let b = |v: f32| v.to_bits();
            for (i, v) in [-8.0, -6.0, -4.0, 1.0, 8.0, 6.0, 4.0, 1.0]
                .into_iter()
                .enumerate()
            {
                c.bus.write32(out + 4 * i as u32, b(v));
            }
        } else if target == fakes::FAKE + 0x10 {
            c.g[V0][0] = if n < inside_calls {
                1
            } else {
                fakes::mix(salt as u64 + n as u64) & 1
            };
        } else if target == 0x0040_da10 {
            c.f[0] = logf;
        }
    }

    /// `CameraTrigger_FindBoundaryPoint(out, volume, pos, step)`: the volume's
    /// `boundBox`, `containsPoint` and `logf` are logged fakes with a scripted
    /// answer. Fixture: 0-3 leading "inside" answers (the doubling loop runs
    /// 0-3 times), then random answers for the bisection; `logf` values that
    /// give no steps, one step and several; unwinds at random calls.
    #[test]
    fn oc680_camera_trigger_find_boundary_point_matches() {
        oc680_in_child("oc680_camera_trigger_find_boundary_point_matches", &|| {
            covered(
                ee::s02::f_00171fa8__generated,
                oc680_camera_trigger_find_boundary_point,
                200,
                &|c, seed| {
                    let mut r = machine(c, seed, 0x0680_0002, 24);
                    c.hooks.miss = oc680_find_hook;
                    c.pc = 0x0017_1fa8;
                    c.g[RA][0] = crate::difftest::RA as u64;
                    let (out, vol, pos, vt) =
                        (DATA + 0x100, DATA + 0x200, DATA + 0x300, DATA + 0x400);
                    c.g[A0][0] = sx(out);
                    c.g[A1][0] = sx(vol);
                    c.g[A2][0] = sx(pos);
                    c.f[12] = [0.5f32, 1.0, 0.25, 2.0][(seed % 4) as usize].to_bits();
                    c.bus.write32(vol + 0x24, vt);
                    for (off, i) in [(0x30u32, 0u32), (0x38, 1)] {
                        c.bus.write16(vt + off, ((r.next_u32() & 0x1c) as u16) & !3);
                        c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i);
                    }
                    for i in 0..4u32 {
                        let v = (r.next_u32() % 41) as f32 - 20.0;
                        c.bus
                            .write32(pos + 4 * i, if i == 3 { 1.0f32 } else { v }.to_bits());
                    }
                    c.bus.write32(OC680_FIND_CTRL, seed / 4 % 4);
                    c.bus.write32(OC680_FIND_CTRL + 4, r.next_u32());
                    let logf = [-3.0f32, 0.0, 0.4, 2.5, 5.0][(seed / 3 % 5) as usize];
                    c.bus.write32(OC680_FIND_CTRL + 8, logf.to_bits());
                },
            );
        });
    }

    /// Control block of the debug-menu / reset fixtures (clear of the data
    /// the tests build): +0 returned key, +4 salt for scripted answers, +8
    /// "the file loads", +0xc file length, +0x10.. front/position vectors.
    const OC680_CTRL: u32 = DATA + 0x21_0000;
    const OC680_BUF: u32 = DATA + 0x22_0000;
    const OC680_STR: u32 = DATA + 0x23_0000;

    /// The miss hook of the debug-menu and trigger-reset fixtures: logs every
    /// call like the plain fake and gives the callees whose results steer the
    /// control flow a scripted answer (the plain fake returns a non-zero
    /// pointer for everything, which never takes the zero branches).
    fn oc680_hook(c: &mut Cpu, target: u32) {
        let (a0, a1, a2) = (c.g[A0][0] as u32, c.g[A1][0] as u32, c.g[A2][0] as u32);
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ctrl = OC680_CTRL;
        let salt = c.bus.read32(ctrl + 4) as u64;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        let bit = |k: u64| fakes::mix(salt.wrapping_add(k)) >> 7 & 1;
        match target {
            // The trigger's rider: AI index, position, front vector.
            t if t == fakes::FAKE => c.g[V0][0] = c.bus.read32(ctrl) as u64,
            t if t == fakes::FAKE + 0x10 || t == fakes::FAKE + 0x20 => {
                let src = ctrl + (t - fakes::FAKE);
                for i in 0..4 {
                    let w = c.bus.read32(src + 4 * i);
                    c.bus.write32(a0 + 4 * i, w);
                }
            }
            // atan(y/x): a fixed angle; fSinCos(&sin, &cos, angle).
            0x0031_c228 => c.f[0] = 0.5f32.to_bits(),
            0x0031_be50 => {
                c.bus.write32(a0, 0.25f32.to_bits());
                c.bus.write32(a1, 0.75f32.to_bits());
            }
            // strcasecmp: equal (0) or not.
            0x0041_aa88 => c.g[V0][0] = bit(n as u64),
            // isValidMap(base, n, names): marks entries found, returns any.
            0x003a_6948 => {
                for k in 0..a1.min(32) {
                    let found = bit(0x100 + n as u64 * 8 + k as u64);
                    c.bus.write32(a2 + 8 * k + 4, found as u32);
                }
                c.g[V0][0] = bit(0x200 + n as u64);
            }
            // FILE_loadsizez(path, &size, flags) / FILE_loadz(path, flags).
            0x003e_1a70 => {
                let loads = c.bus.read32(ctrl + 8) != 0;
                c.g[V0][0] = if loads { OC680_BUF as u64 } else { 0 };
                let len = c.bus.read32(ctrl + 0xc);
                c.bus.write32(a1, len);
            }
            0x003e_1908 => {
                let loads = c.bus.read32(ctrl + 8) != 0;
                c.g[V0][0] = if loads { OC680_BUF as u64 } else { 0 };
            }
            // cBXString(&str, start, len): the string pointer is the first word.
            0x0031_8350 => c.bus.write32(a0, OC680_STR),
            // SHAPE_unpack(shape): the same pointer, nothing or another.
            0x003b_3578 => {
                c.g[V0][0] = match bit(0x300 + n as u64) + bit(0x400 + n as u64) {
                    0 => a0 as u64,
                    1 => 0,
                    _ => (a0 + 4) as u64,
                }
            }
            // The uber-trick menu: some screen objects are missing.
            0x0039_d860 => {
                if bit(0x500 + n as u64) == 0 {
                    c.g[V0][0] = 0;
                }
            }
            _ => {}
        }
    }

    /// Common set-up of the hook fixtures: scrambled machine, our hook, entry
    /// `pc`, the return address and the control block.
    fn oc680_start(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        entry: u32,
    ) -> crate::difftest::Rng {
        let r = machine(c, seed, salt, calls);
        c.hooks.miss = oc680_hook;
        c.pc = entry;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.bus
            .write32(OC680_CTRL + 4, seed.wrapping_mul(0x9e37_79b1));
        r
    }

    /// `BoundedCameraAlgorithmBase_ResetForRider(self, key)`: the rider's AI
    /// index (a scripted vtable call) matches `key` or not; the front vector
    /// takes all 27 sign combinations of (x, y, z) in {0, +, -}, which reach
    /// every quadrant and zero case of the two inlined `atan2`s and the
    /// pitch wrap; the callees are logged fakes; unwinds at random calls.
    #[test]
    fn oc680_bounded_camera_algorithm_base_reset_for_rider_matches() {
        oc680_in_child(
            "oc680_bounded_camera_algorithm_base_reset_for_rider_matches",
            &|| {
                covered(
                    ee::s02::f_001732e8__generated,
                    oc680_bounded_camera_algorithm_base_reset_for_rider,
                    270,
                    &|c, seed| {
                        let mut r = oc680_start(c, seed, 0x0680_0003, 8, 0x0017_32e8);
                        let (this, target, rider, vt_r, vt_t, vt_s) = (
                            DATA + 0x100,
                            DATA + 0x200,
                            DATA + 0x300,
                            DATA + 0x400,
                            DATA + 0x500,
                            DATA + 0x600,
                        );
                        let key = r.next_u32() & 0xff;
                        c.g[A0][0] = sx(this);
                        c.g[A1][0] = key as u64;
                        c.bus
                            .write32(OC680_CTRL, if seed % 5 == 0 { key ^ 1 } else { key });
                        c.bus.write32(this + 0x30, target);
                        c.bus.write32(this + 0x10, vt_s);
                        c.bus.write32(target, vt_t);
                        c.bus.write32(target + 4, rider);
                        c.bus.write32(rider + 0x6c0, vt_r);
                        for (vt, off, i) in [
                            (vt_r, 0x38u32, 0u32),
                            (vt_t, 0x8, 1),
                            (vt_t, 0x20, 2),
                            (vt_s, 0x28, 3),
                        ] {
                            c.bus.write16(vt + off, 0);
                            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i);
                        }
                        for (off, v) in [
                            (0x6cu32, 1.5f32),
                            (0x70, 2.0),
                            (0x74, 3.0),
                            (0x78, 0.5),
                            (0x58, 0.25),
                        ] {
                            c.bus
                                .write32(this + off, (v * (1.0 + (seed % 7) as f32)).to_bits());
                        }
                        // World Z axis (gp - 0xea0) and the angle constants of the gp pool.
                        c.bus.write32(0x004f_f160 + 8, 1.0f32.to_bits());
                        let pi = std::f32::consts::PI;
                        for (off, v) in [
                            (0x6428u32, -pi / 2.0),
                            (0x6424, pi / 2.0),
                            (0x6420, pi),
                            (0x641c, pi),
                            (0x6418, 0.159_154_94),
                            (0x6414, 2.0 * pi),
                            (0x6410, -pi / 2.0),
                            (0x640c, pi / 2.0),
                            (0x6408, pi),
                            (0x6404, pi),
                            (0x6400, 0.017_453_292),
                            (0x63fc, 0.159_154_94),
                            (0x63f8, 2.0 * pi),
                        ] {
                            c.bus.write32(GAME_GP - off, v.to_bits());
                        }
                        let pick = |k: u32, v: f32| [0.0, v, -v][(k % 3) as usize].to_bits();
                        let combo = seed % 27;
                        let front = [
                            pick(combo, 2.0),
                            pick(combo / 3, 3.0),
                            pick(combo / 9, 1.5),
                            0,
                        ];
                        for i in 0..4 {
                            c.bus.write32(OC680_CTRL + 0x20 + 4 * i, front[i as usize]);
                            c.bus.write32(
                                OC680_CTRL + 0x10 + 4 * i,
                                ((r.next_u32() % 200) as f32 - 100.0).to_bits(),
                            );
                        }
                    },
                );
            },
        );
    }

    /// `DebugMenu_InitWorldList(self)`: the 22 names from the BE interface
    /// `DebugMenu_CheckWorldFiles(self, n, names, tag)`: `n` names against 0-3
    /// `DebugMenu_LoadBackgrounds(self)`: a missing file, or a file of 0-3
    /// `DebugMenu_DrawBackground(self)`: the active image out of range
    /// (negative, past the count) or valid; the graphics manager's vtable
    /// calls are fakes, its context stack top is plain memory.
    #[test]
    fn oc680_debug_menu_draw_background_matches() {
        oc680_in_child("oc680_debug_menu_draw_background_matches", &|| {
            covered(
                ee::s02::f_0017ae00__generated,
                oc680_debug_menu_draw_background,
                120,
                &|c, seed| {
                    let mut r = oc680_start(c, seed, 0x0680_0008, 12, 0x0017_ae00);
                    let (this, gr, vt, top, ids) = (
                        DATA + 0x100,
                        DATA + 0x200,
                        DATA + 0x300,
                        DATA + 0x1000,
                        DATA + 0x400,
                    );
                    c.g[A0][0] = sx(this);
                    c.bus.write32(this, ids);
                    c.bus.write32(this + 4, 3);
                    c.bus
                        .write32(this + 8, [-1i32, 0, 1, 2, 3, 5][(seed % 6) as usize] as u32);
                    for i in 0..3 {
                        c.bus.write32(ids + 4 * i, r.next_u32() & 0xffff);
                    }
                    c.bus.write32(GAME_GP - 0x854, gr);
                    c.bus.write32(gr + 0x10d8, vt);
                    c.bus.write32(gr + 0xe84, top);
                    for off in [0xa8u32, 0xf8, 0xd0, 0x110, 0x238, 0x100, 0xb0] {
                        c.bus.write16(vt + off, 0);
                        c.bus.write32(vt + off + 4, fakes::FAKE + off);
                    }
                    for i in 0..0x30 {
                        c.bus.write32(top + 4 * i, r.next_u32());
                    }
                },
            );
        });
    }
}
