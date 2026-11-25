//! Staging module oc678 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc678`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_6a10,
        be_reward_int_load_rewards_file_3,
        "BERewardInt_LoadRewardsFile",
    ),
    (
        0x0015_f9b8,
        manual_cam_algo_base_get_view,
        "ManualCamAlgoBase_GetView",
    ),
    (0x0015_fd50, manual_cam_algo_update, "ManualCamAlgo_Update"),
    (
        0x0016_04b8,
        manual_chase_camera_algorithm_update,
        "ManualChaseCameraAlgorithm_Update",
    ),
    (
        0x0016_1728,
        idle_chase_camera_algorithm_update,
        "IdleChaseCameraAlgorithm_Update",
    ),
    (
        0x0016_1848,
        idle_chase_camera_algorithm_reset_for_rider,
        "IdleChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0016_8338,
        lazy_chase_camera_algorithm_update,
        "LazyChaseCameraAlgorithm_Update",
    ),
    (
        0x0016_8508,
        lazy_chase_camera_algorithm_reset_for_rider,
        "LazyChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0016_86d0,
        look_back_chase_camera_algorithm_update,
        "LookBackChaseCameraAlgorithm_Update",
    ),
    (
        0x0016_8a70,
        mid_right_chase_camera_algorithm_update,
        "MidRightChaseCameraAlgorithm_Update",
    ),
];

/// `void BERewardInt_LoadRewardsFile(void)` (0x156a10): (re)load the rewards
/// database (`0x4c3e98`) through `RewardsFile_Load(db, 1)`.
///
/// The real loader runs away on zeroed test RAM (the `a1 == 1` branch walks the
/// bolt-file equip table), so its test fakes `RewardsFile_Load` (gated). Two
/// older dead copies of this function sit in `game/be/beintreward.rs`
/// (`be_reward_int_load_rewards_file`, `..._2`): delete them at the reorg.
pub fn be_reward_int_load_rewards_file_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004c_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3e98));
        Callee {
            addr: 0x0015_ad58,
            f: ee::s01::RewardsFile_Load,
        }
        .call(c, 0x0015_6a28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `tPose* ManualCamAlgoBase_GetView(tPose* out, const ManualCamAlgoBase* src)`
/// (0x15f9b8): the free camera's pose. Builds a 4x4 matrix from the identity
/// (0x4ff1a0), translates it by `src->Position` (+0x20), rotates it by
/// `-src->Pitch` (+0x30) about the axis at 0x4ff150 and by `src->Heading`
/// (+0x34) about the axis at 0x4ff160 (axis-angle matrices from `Math_SinCos`,
/// multiplied on with VU0), and converts the result to `out` with
/// `Mat4_ToPosQuat`. Returns `out`.
///
/// The matrix lives at sp, its scratch copy at sp+0x40, the rotation at
/// sp+0x40..0x70: the original's stack layout and FPU temporaries.
pub fn manual_cam_algo_base_get_view(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 211;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.g[V0][0] = 0x500000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S2][0] = c.g[A1][0];
        c.g[S5][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lqc2(1, (c.g[V0][0] as u32));
        c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.sqc2(1, (c.g[SP][0] as u32));
        c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(4, (c.g[SP][0] as u32));
        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(8, (c.g[S2][0] as u32).wrapping_add(0x20));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[SP][0];
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x84));
        c.neg_s(12, 12);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.g[S0][0] = 0x500000;
        c.g[RA][0] = sx(0x0015fa5c);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xeb0));
        call_ptr(c, 0x0031be50, 0x0015fa5c)?;
        c.lwc1(11, (c.g[SP][0] as u32).wrapping_add(0x84));
        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.sub_s(1, 20, 11);
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_sub(0xeb0));
        c.lwc1(6, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.mul_s(2, 1, 3);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x70), 0u64);
        c.mul_s(4, 1, 0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x74), 0u64);
        c.mul_s(1, 1, 6);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x78), 0u64);
        c.mul_s(13, 8, 3);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
        c.mul_s(12, 8, 0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), 0u64);
        c.mul_s(9, 2, 3);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x6c), 0u64);
        c.mul_s(10, 4, 3);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x7c));
        c.mul_s(5, 2, 0);
        c.mul_s(7, 4, 0);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 6);
        c.mul_s(2, 2, 6);
        c.mul_s(4, 4, 6);
        c.mul_s(1, 1, 6);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x64));
        c.lqc2(4, (c.g[S3][0] as u32));
        c.lqc2(5, (c.g[S3][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[S3][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[S3][0] as u32).wrapping_add(0x30));
        c.lqc2(8, (c.g[S4][0] as u32));
        c.lqc2(9, (c.g[S4][0] as u32).wrapping_add(0x10));
        c.lqc2(10, (c.g[S4][0] as u32).wrapping_add(0x20));
        c.lqc2(11, (c.g[S4][0] as u32).wrapping_add(0x30));
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
        c.sqc2(12, (c.g[S3][0] as u32));
        c.sqc2(13, (c.g[S3][0] as u32).wrapping_add(0x10));
        c.sqc2(14, (c.g[S3][0] as u32).wrapping_add(0x20));
        c.sqc2(15, (c.g[S3][0] as u32).wrapping_add(0x30));
        c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x34));
        c.g[S0][0] = c.g[SP][0];
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x88));
        c.g[RA][0] = sx(0x0015fb9c);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8c));
        call_ptr(c, 0x0031be50, 0x0015fb9c)?;
        c.lwc1(11, (c.g[SP][0] as u32).wrapping_add(0x8c));
        c.g[V0][0] = 0x500000;
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_sub(0xea0));
        c.sub_s(1, 20, 11);
        c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.lwc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x88));
        c.mul_s(2, 1, 3);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x7c));
        c.mul_s(4, 1, 0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x70), 0u64);
        c.mul_s(1, 1, 6);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x74), 0u64);
        c.mul_s(13, 8, 3);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x78), 0u64);
        c.mul_s(12, 8, 0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
        c.mul_s(9, 2, 3);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), 0u64);
        c.mul_s(10, 4, 3);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x6c), 0u64);
        c.mul_s(5, 2, 0);
        c.mul_s(7, 4, 0);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 6);
        c.mul_s(2, 2, 6);
        c.mul_s(4, 4, 6);
        c.mul_s(1, 1, 6);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x64));
        c.lqc2(4, (c.g[S0][0] as u32));
        c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x10));
        c.lqc2(6, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.lqc2(7, (c.g[S0][0] as u32).wrapping_add(0x30));
        c.lqc2(8, (c.g[S4][0] as u32));
        c.lqc2(9, (c.g[S4][0] as u32).wrapping_add(0x10));
        c.lqc2(10, (c.g[S4][0] as u32).wrapping_add(0x20));
        c.lqc2(11, (c.g[S4][0] as u32).wrapping_add(0x30));
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
        c.sqc2(12, (c.g[S0][0] as u32));
        c.sqc2(13, (c.g[S0][0] as u32).wrapping_add(0x10));
        c.sqc2(14, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.sqc2(15, (c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = c.g[S5][0];
        c.g[RA][0] = sx(0x0015fcd8);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0031b748, 0x0015fcd8)?;
        c.g[V0][0] = c.g[S5][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        return Ok(());
    });
}

/// `void ManualCamAlgo_Update(ManualCamAlgo* this)` (0x15fd50): the free-fly
/// debug camera. Reads the first player's manual-camera pad (through the
/// world at `gp-0x848`) with `InputBinding_EvalFloat` (0x34 speed, 0x31/0x30
/// look, 0x32/0x33 move), turns pitch (+0x30, clamped to +-pi/2) and heading
/// (+0x34, wrapped to [0, 2pi)) by `max(speed, 2) * look * 0.628 / 60`, and
/// moves the position (+0x20) along the look direction and its right vector
/// by `speed * move * 25`.
pub fn manual_cam_algo_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_15fe2c: {
            c.cycles += 54;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
            c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A1][0] = 0x34;
            c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[S1][0] = c.g[A0][0];
            c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0xb8));
            c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xa8));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x98));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa4));
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[RA][0] = sx(0x0015fda4);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00320bf0, 0x0015fda4)?;
            c.mov_s(23, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0015fdb4);
            c.g[A1][0] = 0x31;
            call_ptr(c, 0x00320bf0, 0x0015fdb4)?;
            c.mov_s(20, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0015fdc4);
            c.g[A1][0] = 0x30;
            call_ptr(c, 0x00320bf0, 0x0015fdc4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x32;
            c.g[RA][0] = sx(0x0015fdd4);
            c.mov_s(21, 0);
            call_ptr(c, 0x00320bf0, 0x0015fdd4)?;
            c.mov_s(25, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0015fde4);
            c.g[A1][0] = 0x33;
            call_ptr(c, 0x00320bf0, 0x0015fde4)?;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x6c44));
            c.mov_s(26, 0);
            c.g[AT][0] = 0x40000000;
            c.mtc1(1, 0);
            c.mul_s(20, 20, 5);
            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x6c40));
            c.max_s(3, 23, 0);
            c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x30));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6c3c));
            c.mul_s(20, 3, 20);
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x6c38));
            c.mul_s(20, 20, 6);
            c.add_s(2, 2, 20);
            c.c_le_s(1, 2);
            let t = !c.fcc();
            c.mov_s(24, 1);
            if t {
                break 'b_15fe2c;
            } else {
                c.cycles += 1;
                c.min_s(24, 2, 4);
                break 'b_15fe2c;
            }
        }
        'b_15fe68: {
            c.cycles += 12;
            c.mul_s(0, 21, 5);
            c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x34));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6c34));
            c.mul_s(0, 3, 0);
            c.mul_s(0, 0, 6);
            c.add_s(22, 2, 0);
            c.mul_s(1, 22, 1);
            c.cvt_w_s(2, 1);
            c.cvt_s_w(2, 2);
            c.c_lt_s(1, 2);
            let t = !c.fcc();
            if t {
                break 'b_15fe68;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.sub_s(2, 2, 0);
                break 'b_15fe68;
            }
        }
        c.cycles += 81;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6c30));
        c.mov_s(12, 24);
        c.mul_s(0, 2, 0);
        c.g[RA][0] = sx(0x0015fe7c);
        c.sub_s(22, 22, 0);
        call_ptr(c, 0x0031c040, 0x0015fe7c)?;
        c.mov_s(21, 0);
        c.g[RA][0] = sx(0x0015fe88);
        c.mov_s(12, 22);
        call_ptr(c, 0x0031c040, 0x0015fe88)?;
        c.mul_s(21, 21, 0);
        c.g[RA][0] = sx(0x0015fe94);
        c.mov_s(12, 24);
        call_ptr(c, 0x0031c040, 0x0015fe94)?;
        c.mov_s(12, 22);
        c.g[RA][0] = sx(0x0015fea0);
        c.mov_s(20, 0);
        call_ptr(c, 0x0031bf60, 0x0015fea0)?;
        c.mul_s(20, 20, 0);
        c.g[RA][0] = sx(0x0015feac);
        c.mov_s(12, 24);
        call_ptr(c, 0x0031bf60, 0x0015feac)?;
        c.swc1(21, (c.g[SP][0] as u32));
        c.mov_s(12, 22);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[RA][0] = sx(0x0015fec4);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
        call_ptr(c, 0x0031bf60, 0x0015fec4)?;
        c.mov_s(20, 0);
        c.g[RA][0] = sx(0x0015fed0);
        c.mov_s(12, 22);
        call_ptr(c, 0x0031c040, 0x0015fed0)?;
        c.g[AT][0] = 0x41c80000;
        c.mtc1(1, 1);
        c.neg_s(0, 0);
        c.lq(2, (c.g[S1][0] as u32).wrapping_add(0x20));
        c.mul_s(1, 25, 1);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
        c.mul_s(1, 23, 1);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
        c.mfc1(2, 1);
        c.lqc2(4, (c.g[SP][0] as u32));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[AT][0] = 0xffffffffc1c80000;
        c.mtc1(1, 0);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.mul_s(0, 26, 0);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.mul_s(0, 23, 0);
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.mfc1(3, 0);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0xb8));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0xa8));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x98));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.swc1(24, (c.g[S1][0] as u32).wrapping_add(0x30));
        c.swc1(22, (c.g[S1][0] as u32).wrapping_add(0x34));
        c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        return Ok(());
    });
}

/// `void ManualChaseCameraAlgorithm_Update(ManualChaseCameraAlgorithm* this)`
/// (0x1604b8): the debug orbit camera. Gathers the rider state, reads the
/// controlling pad's three analog inputs (0xbe, 0xbf, 0xc0), and integrates
/// them (squared, times 0.05) into the globals heading (`gp-0x1e74`, wrapped
/// to [0, 2pi)), pitch (`gp-0x1e70`, clamped to [0, 1.508]) and distance
/// (`gp-0x1e6c`, scaled by `1 + 0.05 * z`, clamped to [150, 850]). Stores
/// pitch and heading at +0x54 / +0x50, the look direction from
/// `ChaseCameraAlgorithm_RotateByPitch`, the target at +0x20 and the eye at
/// +0x40 (target - direction * distance).
pub fn manual_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_1605a8: {
            'b_1605a4: {
                'b_160590: {
                    c.cycles += 52;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6c18));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[A1][0] = c.g[SP][0];
                    c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6c14));
                    c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x6c10));
                    c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x6c0c));
                    c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x6c08));
                    c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x6c04));
                    c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x6c00));
                    c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x6bfc));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xe8));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
                    c.g[RA][0] = sx(0x00160500);
                    c.g[S1][0] = c.g[A0][0];
                    Callee {
                        addr: 0x00162568,
                        f: ee::s01::ChaseCameraAlgorithm_GatherRiderState,
                    }
                    .call(c, 0x00160500)?;
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0xbe;
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xb0));
                    c.g[RA][0] = sx(0x00160528);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00320bf0, 0x00160528)?;
                    c.mov_s(20, 0);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00160538);
                    c.g[A1][0] = 0xbf;
                    call_ptr(c, 0x00320bf0, 0x00160538)?;
                    c.mov_s(21, 0);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00160548);
                    c.g[A1][0] = 0xc0;
                    call_ptr(c, 0x00320bf0, 0x00160548)?;
                    c.abs_s(1, 20);
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6bf8));
                    c.mul_s(20, 20, 1);
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x1e74));
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x6bf4));
                    c.neg_s(22, 0);
                    c.abs_s(0, 21);
                    c.abs_s(1, 22);
                    c.mul_s(20, 20, 2);
                    c.mul_s(21, 21, 0);
                    c.mul_s(22, 22, 1);
                    c.add_s(3, 3, 20);
                    c.c_lt_s(4, 3);
                    let t = !c.fcc();
                    c.swc1(3, (c.g[GP][0] as u32).wrapping_sub(0x1e74));
                    if t {
                        break 'b_160590;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sub_s(0, 3, 4);
                        if t {
                            break 'b_1605a4;
                        } else {
                            break 'b_160590;
                        }
                    }
                }
                c.cycles += 4;
                c.mtc1(0, 0);
                c.c_lt_s(3, 0);
                let t = !c.fcc();
                if t {
                    break 'b_1605a8;
                } else {
                    c.cycles += 1;
                    c.add_s(0, 3, 4);
                    break 'b_1605a4;
                }
            }
            c.cycles += 1;
            c.swc1(0, (c.g[GP][0] as u32).wrapping_sub(0x1e74));
            break 'b_1605a8;
        }
        'b_1605d8: {
            c.cycles += 11;
            c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x6bf0));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x1e70));
            c.mul_s(1, 21, 20);
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x6bec));
            c.mtc1(0, 0);
            c.add_s(1, 2, 1);
            c.mov_s(2, 0);
            c.c_le_s(0, 1);
            let t = !c.fcc();
            c.swc1(1, (c.g[GP][0] as u32).wrapping_sub(0x1e70));
            if t {
                break 'b_1605d8;
            } else {
                c.cycles += 1;
                c.min_s(2, 1, 3);
                break 'b_1605d8;
            }
        }
        'b_160670: {
            c.cycles += 37;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x1e74));
            c.g[A0][0] = c.g[S1][0];
            c.swc1(2, (c.g[GP][0] as u32).wrapping_sub(0x1e70));
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
            c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x54));
            c.g[RA][0] = sx(0x001605f4);
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x50));
            call_ptr(c, 0x001668b8, 0x001605f4)?;
            c.mul_s(1, 22, 20);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 3);
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 0);
            c.g[AT][0] = 0x43160000;
            c.mtc1(1, 2);
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x6be8));
            c.add_s(1, 1, 3);
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x1e6c));
            c.mul_s(0, 0, 1);
            c.c_le_s(2, 0);
            let t = !c.fcc();
            c.mov_s(3, 2);
            if t {
                break 'b_160670;
            } else {
                c.cycles += 1;
                c.min_s(3, 0, 4);
                break 'b_160670;
            }
        }
        c.cycles += 25;
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(3, (c.g[GP][0] as u32).wrapping_sub(0x1e6c));
        c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x20));
        c.mfc1(3, 3);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xe8));
        c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.pc = t;
        return Ok(());
    });
}

/// `void IdleChaseCameraAlgorithm_Update(IdleChaseCameraAlgorithm* this)`
/// (0x161728): gathers the rider state, puts the look-at target 13.5 units
/// above the owner's position (+0x20), and, when the eye (+0x40) is not on
/// it, sets the field of view (+0x0) to `min(531.66 / distance, 1) * pi/4`.
pub fn idle_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_161838: {
            c.cycles += 59;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6b5c));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6b58));
            c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x6b54));
            c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x6b50));
            c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x6b4c));
            c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x6b48));
            c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x6b44));
            c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x6b40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
            c.g[RA][0] = sx(0x00161760);
            c.g[S0][0] = c.g[A0][0];
            Callee {
                addr: 0x00162568,
                f: ee::s01::ChaseCameraAlgorithm_GatherRiderState,
            }
            .call(c, 0x00161760)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0016177c);
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            call_ptr(c, t, 0x0016177c)?;
            c.g[V1][0] = 0x500000;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6b3c));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
            c.mtc1(0, 1);
            c.mfc1(2, 0);
            c.lqc2(4, (c.g[V1][0] as u32));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x20));
            c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x40));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 2);
            c.c_lt_s(1, 2);
            if !c.fcc() {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                break 'b_161838;
            } else {
                c.cycles += 9;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6b38));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 1);
                c.div_s(0, 0, 2);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6b34));
                c.min_s(0, 0, 1);
                c.mul_s(0, 0, 2);
                c.swc1(0, (c.g[S0][0] as u32));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                break 'b_161838;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        return Ok(());
    });
}

/// `void IdleChaseCameraAlgorithm_ResetForRider(IdleChaseCameraAlgorithm* this,
/// int id)` (0x161848): if `id` is the AI index of the rider this camera
/// follows, puts the eye (+0x40) at the owner's position plus 100.05 along
/// the Z axis plus the X axis, runs `ChaseCameraAlgorithm_ResetForRider`, then
/// `ChaseCameraAlgorithm_InitFromRider(this, 559.74, 300)` and the virtual
/// update at `this+0x10`.
pub fn idle_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        'b_161940: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00161880);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            call_ptr(c, t, 0x00161880)?;
            if c.g[S1][0] != c.g[V0][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                break 'b_161940;
            } else {
                c.cycles += 46;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001618a4);
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                call_ptr(c, t, 0x001618a4)?;
                c.g[V1][0] = 0x500000;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6b30));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
                c.g[A1][0] = c.g[S1][0];
                c.mfc1(2, 0);
                c.lqc2(4, (c.g[V1][0] as u32));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = c.g[S0][0];
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 0x500000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(4, (c.g[V0][0] as u32));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x00161914);
                c.sq(2, (c.g[SP][0] as u32));
                call_ptr(c, 0x00166c60, 0x00161914)?;
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6b2c));
                c.g[AT][0] = 0x43960000;
                c.mtc1(1, 13);
                c.g[RA][0] = sx(0x00161928);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00166550, 0x00161928)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0016193c);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0016193c)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                break 'b_161940;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `void LazyChaseCameraAlgorithm_Update(LazyChaseCameraAlgorithm* this)`
/// (0x168338): gathers the rider state, puts the target (+0x20) 2.36 above the
/// gathered position, and when the eye (+0x40) is off the target moves it:
/// `offset = target - eye`, `wanted = target - offset * 180.19 / len`,
/// `blend` ramps 0 to 1 over len 188.07..775.46, and the eye becomes
/// `eye * k + wanted * (1 - k)` with `k = (1 - blend) * 0.9917 + blend * 0.8601`.
pub fn lazy_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_1684fc: {
            c.cycles += 52;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6774));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6770));
            c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x676c));
            c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x6768));
            c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x6764));
            c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x6760));
            c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x675c));
            c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x6758));
            c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
            c.g[RA][0] = sx(0x00168370);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x00162568, 0x00168370)?;
            c.g[V0][0] = 0x500000;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6754));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
            c.mtc1(0, 3);
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[V0][0] as u32));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x20));
            c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x40));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 1);
            c.c_lt_s(3, 1);
            if !c.fcc() {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                break 'b_1684fc;
            } else {
                'b_168480: {
                    'b_16847c: {
                        c.cycles += 16;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6750));
                        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x674c));
                        c.div_s(0, 0, 1);
                        c.mfc1(2, 0);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.qmtc2(2, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.c_le_s(2, 1);
                        c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x20));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        let t = !c.fcc();
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        if t {
                            break 'b_16847c;
                        } else {
                            'b_16846c: {
                                c.cycles += 4;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6748));
                                c.c_le_s(1, 0);
                                let t = !c.fcc();
                                if t {
                                    break 'b_16846c;
                                } else {
                                    c.cycles += 5;
                                    c.sub_s(1, 1, 2);
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6744));
                                    c.div_s(4, 1, 0);
                                    let t = true;
                                    if t {
                                        break 'b_168480;
                                    } else {
                                        break 'b_16846c;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 4);
                            let t = true;
                            if t {
                                break 'b_168480;
                            } else {
                                break 'b_16847c;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.mov_s(4, 3);
                    break 'b_168480;
                }
                c.cycles += 31;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x6740));
                c.sub_s(0, 2, 4);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x673c));
                c.mul_s(1, 4, 1);
                c.mul_s(0, 0, 3);
                c.add_s(0, 0, 1);
                c.mfc1(3, 0);
                c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sub_s(2, 2, 0);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.mfc1(3, 2);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x40));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                break 'b_1684fc;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.pc = t;
        return Ok(());
    });
}

/// `void LazyChaseCameraAlgorithm_ResetForRider(LazyChaseCameraAlgorithm* this,
/// int id)` (0x168508): if `id` is the AI index of the rider this camera
/// follows, puts the eye (+0x40) at the owner's position plus 100 along the Z
/// axis minus 200 along its front vector, runs
/// `ChaseCameraAlgorithm_ResetForRider`, then
/// `ChaseCameraAlgorithm_InitFromRider(this, 100, 100)` and the virtual
/// update at `this+0x10`.
pub fn lazy_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        'b_16863c: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00168544);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            call_ptr(c, t, 0x00168544)?;
            if c.g[S1][0] != c.g[V0][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                break 'b_16863c;
            } else {
                c.cycles += 60;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[AT][0] = 0x42c80000;
                c.mtc1(1, 20);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00168570);
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                call_ptr(c, t, 0x00168570)?;
                c.g[V1][0] = 0x500000;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
                c.mfc1(2, 20);
                c.lqc2(4, (c.g[V1][0] as u32));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001685c8);
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                call_ptr(c, t, 0x001685c8)?;
                c.g[AT][0] = 0x43480000;
                c.mtc1(1, 0);
                c.g[A1][0] = c.g[S1][0];
                c.mfc1(2, 0);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A0][0] = c.g[S0][0];
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x00168614);
                c.sq(2, (c.g[SP][0] as u32));
                Callee {
                    addr: 0x00166c60,
                    f: ee::s02::ChaseCameraAlgorithm_ResetForRider,
                }
                .call(c, 0x00168614)?;
                c.mov_s(12, 20);
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00168624);
                c.mov_s(13, 12);
                Callee {
                    addr: 0x00166550,
                    f: ee::s02::ChaseCameraAlgorithm_InitFromRider,
                }
                .call(c, 0x00168624)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00168638);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00168638)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                break 'b_16863c;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void LookBackChaseCameraAlgorithm_Update(LookBackChaseCameraAlgorithm* this)`
/// (0x1686d0): gathers the rider state and runs the chase passes (half-pipe
/// set-up, target, direction, follow distance, drop with speed, fov with speed,
/// late target, slow-speed hold, lateral shift) with this camera's tuning,
/// then pushes the eye (+0x40) back along the eye-to-target line by 294.94
/// and slides the target (+0x20) to the point of that line closest to the
/// owner's position.
pub fn look_back_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 155;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
        c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x6738));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6734));
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6730));
        c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x672c));
        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x6728));
        c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x6724));
        c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x6720));
        c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x671c));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
        c.g[RA][0] = sx(0x0016870c);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
        call_ptr(c, 0x00162568, 0x0016870c)?;
        c.mtc1(0, 20);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0016871c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00162a20, 0x0016871c)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00168728);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00162b80, 0x00168728)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00168734);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00162c78, 0x00168734)?;
        c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x6718));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x6714));
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6710));
        c.mov_s(13, 20);
        c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x670c));
        c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x6708));
        c.g[RA][0] = sx(0x0016875c);
        c.mov_s(15, 20);
        call_ptr(c, 0x00163010, 0x0016875c)?;
        c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x6704));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6700));
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.mov_s(14, 20);
        c.g[RA][0] = sx(0x0016877c);
        c.mov_s(15, 20);
        call_ptr(c, 0x00163270, 0x0016877c)?;
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x66fc));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 14);
        c.mov_s(12, 20);
        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x66f8));
        c.g[RA][0] = sx(0x0016879c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001633b0, 0x0016879c)?;
        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x66f4));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x66f0));
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x66ec));
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x66e8));
        c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x66e4));
        c.g[RA][0] = sx(0x001687bc);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00162b90, 0x001687bc)?;
        c.g[AT][0] = 0x3fc00000;
        c.mtc1(1, 13);
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x41700000;
        c.mtc1(1, 12);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001687dc);
        c.mov_s(14, 13);
        call_ptr(c, 0x001643a8, 0x001687dc)?;
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x66e0));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x66dc));
        c.g[RA][0] = sx(0x001687f0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00163158, 0x001687f0)?;
        c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x66d8));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 1);
        c.add_s(0, 1, 0);
        c.div_s(0, 0, 1);
        c.mfc1(2, 0);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x40));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00168884);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        call_ptr(c, t, 0x00168884)?;
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 1);
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 0);
        c.div_s(0, 0, 1);
        c.mfc1(3, 0);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x40));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.pc = t;
        return Ok(());
    });
}

/// `void MidRightChaseCameraAlgorithm_Update(MidRightChaseCameraAlgorithm* this)`
/// (0x168a70): gathers the rider state and runs the chase passes (handplant
/// blend, half-pipe set-up, target, direction, follow distance, drop with
/// speed, fov with speed, handplant, late target, air cam, half-pipe cam,
/// rail cam, view cam, slow-speed hold, lateral shift) with this camera's
/// tuning constants.
pub fn mid_right_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 120;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x66bc));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x66b8));
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x66b4));
        c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x66b0));
        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x66ac));
        c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x66a8));
        c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x66a4));
        c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x66a0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x98));
        c.g[RA][0] = sx(0x00168ab0);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        call_ptr(c, 0x00162568, 0x00168ab0)?;
        c.mtc1(0, 20);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00168ac0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00162998, 0x00168ac0)?;
        c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x669c));
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00168ad0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00162a20, 0x00168ad0)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00168adc);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00162b80, 0x00168adc)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00168ae8);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00162c78, 0x00168ae8)?;
        c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x6698));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x6694));
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6690));
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x668c));
        c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x6688));
        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x6684));
        c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x6680));
        c.g[RA][0] = sx(0x00168b10);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00163010, 0x00168b10)?;
        c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x667c));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6678));
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6674));
        c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x6670));
        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x666c));
        c.g[RA][0] = sx(0x00168b30);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00163270, 0x00168b30)?;
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6668));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 14);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x6664));
        c.g[RA][0] = sx(0x00168b50);
        c.mov_s(12, 20);
        call_ptr(c, 0x001633b0, 0x00168b50)?;
        c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x6660));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x665c));
        c.mov_s(17, 21);
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6658));
        c.mov_s(16, 20);
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6654));
        c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x6650));
        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x664c));
        c.g[RA][0] = sx(0x00168b7c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00163450, 0x00168b7c)?;
        c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x6648));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6644));
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6640));
        c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x663c));
        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x6638));
        c.g[RA][0] = sx(0x00168b9c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00162b90, 0x00168b9c)?;
        c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x6634));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x6630));
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x662c));
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6628));
        c.g[RA][0] = sx(0x00168bb8);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001635f8, 0x00168bb8)?;
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6624));
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00168bc8);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00164878, 0x00168bc8)?;
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6620));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x661c));
        c.g[RA][0] = sx(0x00168bdc);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001646a0, 0x00168bdc)?;
        c.g[AT][0] = 0x44160000;
        c.mtc1(1, 12);
        c.mov_s(15, 21);
        c.g[AT][0] = 0x43960000;
        c.mtc1(1, 13);
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x42c80000;
        c.mtc1(1, 14);
        c.g[RA][0] = sx(0x00168c04);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001641c0, 0x00168c04)?;
        c.g[AT][0] = 0x3fc00000;
        c.mtc1(1, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x41700000;
        c.mtc1(1, 12);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00168c24);
        c.mov_s(14, 13);
        call_ptr(c, 0x001643a8, 0x00168c24)?;
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6618));
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6614));
        c.g[RA][0] = sx(0x00168c38);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x00163158, 0x00168c38)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x98));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, elf_image, fill, put_vec, Rng, DATA, GAME_GP, SP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word in the machine's RAM: the oc678 callee fakes act only when it
    /// holds [`OC678_MAGIC`] (other units' tests run the real callees).
    const OC678_CTRL: u32 = DATA + 0x7_0100;
    const OC678_MAGIC: u32 = 0x0678_0a11;
    /// 16 float results of the `InputBinding_EvalFloat` fake, by call index.
    const OC678_VALUES: u32 = OC678_CTRL + 0x40;
    /// 16 sin/cos results of the `Math_SinCos` fake (a pair per call).
    const OC678_SINCOS: u32 = OC678_CTRL + 0x80;
    /// The 16 words the `GatherRiderState` fake writes into the info struct.
    const OC678_INFO: u32 = OC678_CTRL + 0xc0;

    /// Gated logging fakes for callees with an override slot: with the gate
    /// off they run what the slot held before.
    macro_rules! oc678_plain_fakes {
        ($(($name:ident, $prev:ident, $addr:expr, $gen:expr, $slot:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC678_CTRL) != OC678_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc678_install_plain() {
                $( fakes::hook(addr_of_mut!($slot), $name, &$prev); )*
            }
        };
    }

    oc678_plain_fakes! {
        (oc678_fake_0015ad58, OC678_PREV_0015AD58, 0x0015_ad58, ee::s01::f_0015ad58__generated, ee::s01::OVERRIDE_0015AD58),
        (oc678_fake_00162998, OC678_PREV_00162998, 0x0016_2998, ee::s01::f_00162998__generated, ee::s01::OVERRIDE_00162998),
        (oc678_fake_00162a20, OC678_PREV_00162A20, 0x0016_2a20, ee::s01::f_00162a20__generated, ee::s01::OVERRIDE_00162A20),
        (oc678_fake_00162b80, OC678_PREV_00162B80, 0x0016_2b80, ee::s01::f_00162b80__generated, ee::s01::OVERRIDE_00162B80),
        (oc678_fake_00162b90, OC678_PREV_00162B90, 0x0016_2b90, ee::s01::f_00162b90__generated, ee::s01::OVERRIDE_00162B90),
        (oc678_fake_00162c78, OC678_PREV_00162C78, 0x0016_2c78, ee::s01::f_00162c78__generated, ee::s01::OVERRIDE_00162C78),
        (oc678_fake_00163010, OC678_PREV_00163010, 0x0016_3010, ee::s01::f_00163010__generated, ee::s01::OVERRIDE_00163010),
        (oc678_fake_00163158, OC678_PREV_00163158, 0x0016_3158, ee::s01::f_00163158__generated, ee::s01::OVERRIDE_00163158),
        (oc678_fake_00163270, OC678_PREV_00163270, 0x0016_3270, ee::s01::f_00163270__generated, ee::s01::OVERRIDE_00163270),
        (oc678_fake_001633b0, OC678_PREV_001633B0, 0x0016_33b0, ee::s01::f_001633b0__generated, ee::s01::OVERRIDE_001633B0),
        (oc678_fake_00163450, OC678_PREV_00163450, 0x0016_3450, ee::s01::f_00163450__generated, ee::s01::OVERRIDE_00163450),
        (oc678_fake_001635f8, OC678_PREV_001635F8, 0x0016_35f8, ee::s01::f_001635f8__generated, ee::s01::OVERRIDE_001635F8),
        (oc678_fake_001641c0, OC678_PREV_001641C0, 0x0016_41c0, ee::s01::f_001641c0__generated, ee::s01::OVERRIDE_001641C0),
        (oc678_fake_001643a8, OC678_PREV_001643A8, 0x0016_43a8, ee::s01::f_001643a8__generated, ee::s01::OVERRIDE_001643A8),
        (oc678_fake_001646a0, OC678_PREV_001646A0, 0x0016_46a0, ee::s01::f_001646a0__generated, ee::s01::OVERRIDE_001646A0),
        (oc678_fake_00164878, OC678_PREV_00164878, 0x0016_4878, ee::s01::f_00164878__generated, ee::s01::OVERRIDE_00164878),
        (oc678_fake_00166c60, OC678_PREV_00166C60, 0x0016_6c60, ee::s02::f_00166c60__generated, ee::s02::OVERRIDE_00166C60),
        (oc678_fake_00166550, OC678_PREV_00166550, 0x0016_6550, ee::s02::f_00166550__generated, ee::s02::OVERRIDE_00166550),
        (oc678_fake_0031b748, OC678_PREV_0031B748, 0x0031_b748, ee::s10::f_0031b748__generated, ee::s10::OVERRIDE_0031B748),
    }

    static OC678_PREV_GATHER: AtomicUsize = AtomicUsize::new(0);
    /// `ChaseCameraAlgorithm_GatherRiderState(this, info)`: logs the call and
    /// writes the 16 words at [`OC678_INFO`] into `info` (a1).
    fn oc678_fake_gather(c: &mut Cpu) {
        if c.bus.read32(OC678_CTRL) != OC678_MAGIC {
            return fakes::fall_back(c, &OC678_PREV_GATHER, ee::s01::f_00162568__generated);
        }
        let info = c.g[A1][0] as u32;
        fakes::fake_call(c, 0x0016_2568);
        if c.pc >> 16 != 0xdead {
            for i in 0..16 {
                let w = c.bus.read32(OC678_INFO + 4 * i);
                c.bus.write32(info + 4 * i, w);
            }
        }
    }

    static OC678_PREV_INPUT: AtomicUsize = AtomicUsize::new(0);
    /// `InputBinding_EvalFloat(pad, id)`: logs the call and returns
    /// `OC678_VALUES[call index]` in f0.
    fn oc678_fake_input(c: &mut Cpu) {
        if c.bus.read32(OC678_CTRL) != OC678_MAGIC {
            return fakes::fall_back(c, &OC678_PREV_INPUT, ee::s10::f_00320bf0__generated);
        }
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, 0x0032_0bf0);
        if c.pc >> 16 != 0xdead {
            c.f[0] = c.bus.read32(OC678_VALUES + 4 * (n & 15));
        }
    }

    static OC678_PREV_SINCOS: AtomicUsize = AtomicUsize::new(0);
    /// `Math_SinCos(angle, &sin, &cos)`-shaped call (a0, a1 are the output
    /// pointers): logs the call and writes `OC678_SINCOS[2n..2n+2]`.
    fn oc678_fake_sincos(c: &mut Cpu) {
        if c.bus.read32(OC678_CTRL) != OC678_MAGIC {
            return fakes::fall_back(c, &OC678_PREV_SINCOS, ee::s10::f_0031be50__generated);
        }
        let (sin_at, cos_at) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, 0x0031_be50);
        if c.pc >> 16 != 0xdead {
            let s = c.bus.read32(OC678_SINCOS + 8 * (n & 7));
            let k = c.bus.read32(OC678_SINCOS + 8 * (n & 7) + 4);
            c.bus.write32(sin_at, s);
            c.bus.write32(cos_at, k);
        }
    }

    /// Hooks every oc678 fake into its callee's override slot (once per
    /// process; with the gate clear they run what the slots held before).
    fn oc678_install() {
        oc678_install_plain();
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00162568),
            oc678_fake_gather,
            &OC678_PREV_GATHER,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320BF0),
            oc678_fake_input,
            &OC678_PREV_INPUT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0031BE50),
            oc678_fake_sincos,
            &OC678_PREV_SINCOS,
        );
    }

    /// A machine with the game image (float constants at `gp-...`), the gate
    /// set, random fake results, and the axis / identity vectors the camera
    /// code loads from 0x4ff140.. (bss: empty in the image).
    fn oc678_begin(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        elf_image(c);
        c.bus.write32(OC678_CTRL, OC678_MAGIC);
        for i in 0..16 {
            c.bus.write32(OC678_VALUES + 4 * i, r.f32(2.0));
            c.bus.write32(OC678_SINCOS + 4 * i, r.f32(1.0));
            c.bus.write32(OC678_INFO + 4 * i, r.f32(500.0));
        }
        let one = 1.0f32.to_bits();
        put_vec(c, 0x004f_f140, [one, 0, 0, 0]);
        put_vec(c, 0x004f_f150, [0, one, 0, 0]);
        put_vec(c, 0x004f_f160, [0, 0, one, 0]);
        put_vec(c, 0x004f_f1a0, [one, 0, 0, 0]);
        put_vec(c, 0x004f_f1b0, [0, one, 0, 0]);
        put_vec(c, 0x004f_f1c0, [0, 0, one, 0]);
        put_vec(c, 0x004f_f1d0, [0, 0, 0, one]);
        r
    }

    /// A virtual-call slot at `vt + off`: `{delta, FAKE + 0x10 * n}`.
    fn oc678_vslot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, n: u32) {
        c.bus.write16(vt + off, (r.next_u32() as u16) & 0xfffc);
        c.bus.write32(vt + off + 4, FAKE + 0x10 * n);
    }

    fn oc678_f(c: &mut Cpu, addr: u32) -> f32 {
        f32::from_bits(c.bus.read32(addr))
    }

    /// The owner (`this+0x30`) object at `DATA+0x2000` with its vtable at
    /// `DATA+0x3000`, which holds a fake at `+0x8/+0xc` (the position getter).
    fn oc678_owner(c: &mut Cpu, r: &mut Rng, this: u32) {
        let (own, vt) = (DATA + 0x2000, DATA + 0x3000);
        c.bus.write32(this + 0x30, own);
        c.bus.write32(own, vt);
        oc678_vslot(c, r, vt, 0x8, 1);
    }

    /// `BERewardInt_LoadRewardsFile`: one call to the (faked) loader, which
    /// may unwind.
    #[test]
    fn be_reward_int_load_rewards_file_3_matches() {
        oc678_install();
        covered(
            ee::s01::f_00156a10__generated,
            be_reward_int_load_rewards_file_3,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x0015_6a10, 1);
                c.bus.write32(OC678_CTRL, OC678_MAGIC);
                c.pc = 0x0015_6a10;
            },
        );
    }

    /// `ManualCamAlgoBase_GetView`: random position and angles; the two
    /// `Math_SinCos` calls and `Mat4_ToPosQuat` are fakes (any may unwind).
    #[test]
    fn manual_cam_algo_base_get_view_matches() {
        oc678_install();
        covered(
            ee::s01::f_0015f9b8__generated,
            manual_cam_algo_base_get_view,
            200,
            &|c, seed| {
                let mut r = oc678_begin(c, seed, 0x0015_f9b8, 3);
                c.pc = 0x0015_f9b8;
                let (out, src) = (DATA + 0x100, DATA + 0x200);
                c.g[A0][0] = out as u64;
                c.g[A1][0] = src as u64;
                for i in 0..4 {
                    c.bus.write32(src + 0x20 + 4 * i, r.f32(500.0));
                }
                c.bus.write32(src + 0x30, r.f32(1.6));
                c.bus.write32(src + 0x34, r.f32(3.2));
            },
        );
    }

    /// `ManualCamAlgo_Update`: the five pad reads are fakes with random
    /// values; pitch starts at, or near, both clamps and heading near both
    /// wrap points (and anywhere else, of both signs).
    #[test]
    fn manual_cam_algo_update_matches() {
        oc678_install();
        covered(
            ee::s01::f_0015fd50__generated,
            manual_cam_algo_update,
            300,
            &|c, seed| {
                let mut r = oc678_begin(c, seed, 0x0015_fd50, 5);
                c.pc = 0x0015_fd50;
                let this = DATA + 0x400;
                c.g[A0][0] = this as u64;
                let (world, x1, x2, x3, x4, pad) = (
                    DATA + 0x1_0000,
                    DATA + 0x1_0100,
                    DATA + 0x1_0200,
                    DATA + 0x1_0300,
                    DATA + 0x1_0400,
                    DATA + 0x1_0500,
                );
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, x1);
                c.bus.write32(x1 + 0x84, x2);
                c.bus.write32(x2 + 4, x3);
                c.bus.write32(x3 + 0xa4, x4);
                c.bus.write32(x4 + 0xc, pad);
                for i in 0..4 {
                    c.bus.write32(this + 0x20 + 4 * i, r.f32(500.0));
                }
                let pitch = match seed % 4 {
                    0 => 1.5707f32.to_bits(),
                    1 => (-1.5707f32).to_bits(),
                    _ => r.f32(1.6),
                };
                let yaw = match seed % 3 {
                    0 => 6.28f32.to_bits(),
                    1 => 0.01f32.to_bits(),
                    _ => r.f32(12.0),
                };
                c.bus.write32(this + 0x30, pitch);
                c.bus.write32(this + 0x34, yaw);
            },
        );
    }

    /// `ManualChaseCameraAlgorithm_Update`: the pad (found through the world
    /// at `gp-0x848`), `GatherRiderState` and `Math_SinCos` are fakes; the
    /// heading / pitch / distance globals start at or near their wrap and
    /// clamp points.
    #[test]
    fn manual_chase_camera_algorithm_update_matches() {
        oc678_install();
        covered(
            ee::s01::f_001604b8__generated,
            manual_chase_camera_algorithm_update,
            300,
            &|c, seed| {
                let mut r = oc678_begin(c, seed, 0x0016_04b8, 6);
                c.pc = 0x0016_04b8;
                let this = DATA + 0x400;
                c.g[A0][0] = this as u64;
                fill(c, &mut r, this, 0x40);
                let (world, x1, x2) = (DATA + 0x1_0000, DATA + 0x1_0200, DATA + 0x1_0300);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, x1);
                c.bus.write32(x1 + 0x28, x2);
                c.bus.write32(x2 + 0x28, seed % 4);
                for k in 0..4 {
                    c.bus
                        .write32(world + 0xb0 + 4 * k, DATA + 0x1_0400 + 0x40 * k);
                }
                let yaw = match seed % 5 {
                    0 => 6.25f32.to_bits(),
                    1 => 0.0f32.to_bits(),
                    2 => 0.02f32.to_bits(),
                    3 => 6.283f32.to_bits(),
                    _ => r.f32(7.0),
                };
                let pitch = match seed % 4 {
                    0 => 0.0f32.to_bits(),
                    1 => 0.01f32.to_bits(),
                    2 => 1.507f32.to_bits(),
                    _ => (r.f32(1.5) & 0x7fff_ffff),
                };
                let dist = match seed % 6 {
                    0 => 150.0f32.to_bits(),
                    1 => 160.0f32.to_bits(),
                    2 => 800.0f32.to_bits(),
                    3 => 849.0f32.to_bits(),
                    4 => 850.0f32.to_bits(),
                    _ => 220.0f32.to_bits(),
                };
                c.bus.write32(GAME_GP - 0x1e74, yaw);
                c.bus.write32(GAME_GP - 0x1e70, pitch);
                c.bus.write32(GAME_GP - 0x1e6c, dist);
            },
        );
    }

    /// `IdleChaseCameraAlgorithm_Update`: the gather and the owner's position
    /// getter are fakes (the position stays zero); every third seed puts the
    /// eye on the target (`0 < len` false).
    #[test]
    fn idle_chase_camera_algorithm_update_matches() {
        oc678_install();
        covered(
            ee::s01::f_00161728__generated,
            idle_chase_camera_algorithm_update,
            200,
            &|c, seed| {
                let mut r = oc678_begin(c, seed, 0x0016_1728, 2);
                c.pc = 0x0016_1728;
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                fill(c, &mut r, this, 0x40);
                oc678_owner(c, &mut r, this);
                let height = oc678_f(c, GAME_GP - 0x6b3c).to_bits();
                let eye = if seed % 3 == 0 {
                    [0, 0, height, 0]
                } else {
                    [r.f32(500.0), r.f32(500.0), r.f32(500.0), 0]
                };
                put_vec(c, this + 0x40, eye);
            },
        );
    }

    /// The owner / rider / vtable chain of the `ResetForRider` functions: the
    /// rider's AI-index getter (`FAKE`), the owner's getters at vtable +0x8
    /// (and +0x20 when `front`), and the camera's own update at `this+0x10`.
    fn oc678_reset_chain(c: &mut Cpu, r: &mut Rng, this: u32, front: bool) {
        let (own, vt2, rider, vt1, vt3) = (
            DATA + 0x2000,
            DATA + 0x3000,
            DATA + 0x4000,
            DATA + 0x5000,
            DATA + 0x5800,
        );
        c.bus.write32(this + 0x30, own);
        c.bus.write32(this + 0x10, vt3);
        c.bus.write32(own, vt2);
        c.bus.write32(own + 4, rider);
        c.bus.write32(rider + 0x6c0, vt1);
        oc678_vslot(c, r, vt1, 0x38, 0);
        oc678_vslot(c, r, vt2, 0x8, 1);
        if front {
            oc678_vslot(c, r, vt2, 0x20, 2);
        }
        oc678_vslot(c, r, vt3, 0x28, 3);
    }

    /// `IdleChaseCameraAlgorithm_ResetForRider`: the rider's AI index (faked
    /// to equal `id` on even seeds, a pointer otherwise), then the owner
    /// position getter, `ResetForRider`, `InitFromRider` and the update.
    #[test]
    fn idle_chase_camera_algorithm_reset_for_rider_matches() {
        oc678_install();
        covered(
            ee::s01::f_00161848__generated,
            idle_chase_camera_algorithm_reset_for_rider,
            200,
            &|c, seed| {
                let mut r = oc678_begin(c, seed, 0x0016_1848, 5);
                c.pc = 0x0016_1848;
                let this = DATA + 0x1000;
                fill(c, &mut r, this, 0x40);
                oc678_reset_chain(c, &mut r, this, false);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 7;
                if seed % 2 == 0 {
                    c.bus.write32(LOG_RESULTS, 7);
                }
            },
        );
    }

    /// `LazyChaseCameraAlgorithm_Update`: the gather is a fake; the eye is put
    /// at distances 0 (eye on the target), below, inside and above the
    /// 188..775 blend band from the target.
    #[test]
    fn lazy_chase_camera_algorithm_update_matches() {
        oc678_install();
        covered(
            ee::s02::f_00168338__generated,
            lazy_chase_camera_algorithm_update,
            300,
            &|c, seed| {
                let mut r = oc678_begin(c, seed, 0x0016_8338, 1);
                c.pc = 0x0016_8338;
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                fill(c, &mut r, this, 0x40);
                let k = oc678_f(c, GAME_GP - 0x6754);
                let want = [
                    0.0f32, 20.0, 100.0, 150.0, 250.0, 400.0, 700.0, 1000.0, 3000.0,
                ][seed as usize % 9];
                let jitter = 1.0 + (r.next_u32() % 8) as f32 / 16.0;
                let a = want * jitter / 1.7;
                let eye = if want == 0.0 {
                    // The gathered position zero: target = (0, 0, k, 0) exactly.
                    for i in 0..4 {
                        c.bus.write32(OC678_INFO + 0x10 + 4 * i, 0);
                    }
                    [0, 0, k.to_bits(), 0]
                } else {
                    let t = |c: &mut Cpu, i: u32| oc678_f(c, OC678_INFO + 0x10 + 4 * i);
                    let (tx, ty, tz) = (t(c, 0), t(c, 1), t(c, 2));
                    [
                        (tx + a).to_bits(),
                        (ty + a).to_bits(),
                        (tz + k + a).to_bits(),
                        0,
                    ]
                };
                put_vec(c, this + 0x40, eye);
            },
        );
    }

    /// `LazyChaseCameraAlgorithm_ResetForRider`: like the idle one, with the
    /// owner's front getter as well.
    #[test]
    fn lazy_chase_camera_algorithm_reset_for_rider_matches() {
        oc678_install();
        covered(
            ee::s02::f_00168508__generated,
            lazy_chase_camera_algorithm_reset_for_rider,
            200,
            &|c, seed| {
                let mut r = oc678_begin(c, seed, 0x0016_8508, 6);
                c.pc = 0x0016_8508;
                let this = DATA + 0x1000;
                fill(c, &mut r, this, 0x40);
                oc678_reset_chain(c, &mut r, this, true);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 7;
                if seed % 2 == 0 {
                    c.bus.write32(LOG_RESULTS, 7);
                }
            },
        );
    }

    /// `LookBackChaseCameraAlgorithm_Update`: the gather, the nine chase
    /// passes and the owner's position getter are fakes (any may unwind);
    /// the vector arithmetic after them runs on random target and eye.
    #[test]
    fn look_back_chase_camera_algorithm_update_matches() {
        oc678_install();
        covered(
            ee::s02::f_001686d0__generated,
            look_back_chase_camera_algorithm_update,
            200,
            &|c, seed| {
                let mut r = oc678_begin(c, seed, 0x0016_86d0, 11);
                c.pc = 0x0016_86d0;
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                fill(c, &mut r, this, 0x40);
                oc678_owner(c, &mut r, this);
                for off in [0x20, 0x40] {
                    for i in 0..3 {
                        c.bus.write32(this + off + 4 * i, r.f32(500.0));
                    }
                    c.bus.write32(this + off + 12, 0);
                }
            },
        );
    }

    /// `MidRightChaseCameraAlgorithm_Update`: a chain of sixteen calls (the
    /// gather and the chase passes), all fakes, any of which may unwind.
    #[test]
    fn mid_right_chase_camera_algorithm_update_matches() {
        oc678_install();
        covered(
            ee::s02::f_00168a70__generated,
            mid_right_chase_camera_algorithm_update,
            200,
            &|c, seed| {
                let mut r = oc678_begin(c, seed, 0x0016_8a70, 16);
                c.pc = 0x0016_8a70;
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                fill(c, &mut r, this, 0x40);
            },
        );
    }
}
