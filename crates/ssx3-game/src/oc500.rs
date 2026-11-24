//! Staging module oc500 (wave wave228): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc500`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0017_3e40, bounded_camera_algorithm_base_get_view, "BoundedCameraAlgorithmBase_GetView"),
    (0x0017_41d8, bounded_camera_algorithm_dtor, "BoundedCameraAlgorithm_dtor"),
    (0x0017_4200, bounded_camera_algorithm_update, "BoundedCameraAlgorithm_Update"),
    (0x0017_44b0, bounded_camera_algorithm_reset_for_rider, "BoundedCameraAlgorithm_ResetForRider"),
    (0x0017_6b10, default_2_chase_camera_algorithm_update, "Default_2ChaseCameraAlgorithm_Update"),
    (0x0017_7110, default_4_chase_camera_algorithm_update, "Default_4ChaseCameraAlgorithm_Update"),
    (0x0017_7368, direction_ne_chase_camera_algorithm_update, "DirectionNeChaseCameraAlgorithm_Update"),
    (0x0017_83e0, spoke_chase_camera_algorithm_reset_for_rider, "SpokeChaseCameraAlgorithm_ResetForRider"),
    (0x0017_6270, camera_trigger_man_member45bde0_dtor, "CameraTriggerMan_Member45bde0_dtor"),
    (0x0017_62f0, camera_trigger_man_member10_call_v05, "CameraTriggerMan_Member10_CallV05"),
    (0x0017_6328, script_camera_algo_dtor, "ScriptCameraAlgo_dtor"),
    (0x0017_63c8, subject_script_camera_algo_dtor, "SubjectScriptCameraAlgo_dtor"),
    (0x0017_6a28, camera_trigger_man_global_ctors, "CameraTriggerMan_GlobalCtors"),
    (0x0017_6a48, camera_trigger_man_global_dtors, "CameraTriggerMan_GlobalDtors"),
    (0x0017_6aa8, default_2_chase_camera_algorithm_dtor, "Default_2ChaseCameraAlgorithm_dtor"),
    (0x0017_70a8, default_4_chase_camera_algorithm_dtor, "Default_4ChaseCameraAlgorithm_dtor"),
    (0x0017_7520, direction_ne_chase_camera_algorithm_dtor, "DirectionNeChaseCameraAlgorithm_dtor"),
    (0x0017_7e30, direction_ne_chase_camera_algorithm_global_ctor, "DirectionNeChaseCameraAlgorithm_GlobalCtor"),
    (0x0017_7e90, spoke_chase_camera_algorithm_dtor, "SpokeChaseCameraAlgorithm_dtor"),
    (0x0017_8588, superpipe_chase_camera_algorithm_dtor, "SuperpipeChaseCameraAlgorithm_dtor"),
];

/// The vtable word the camera-algorithm destructors store at `this+0x10`
/// for `0x460000 - 0x4220` (the `ScriptCameraAlgo`-family vtable).
pub(crate) const ALGO_DTOR_VTABLE_45DDE0: u32 = 0x0046_0000 - 0x4220;
/// `operator delete(void*)` (0x317e50), called by the deleting destructors.
pub(crate) const OPERATOR_DELETE_317E50: u32 = 0x0031_7e50;
/// `CameraTriggerMan_StaticInitDestroy` (0x175a20).
pub(crate) const CAMERA_TRIGGER_MAN_STATIC_INIT_DESTROY: u32 = 0x0017_5a20;
/// `DirectionNeChaseCameraAlgorithm_StaticInit` (0x177650).
pub(crate) const DIRECTION_NE_CHASE_STATIC_INIT: u32 = 0x0017_7650;
/// The chase camera base destructor (0x162458), tail-called by the chase
/// algorithms' destructors.
pub(crate) const CHASE_CAMERA_BASE_DTOR_162458: u32 = 0x0016_2458;

/// Destructor shape shared by the bounded and chase camera algorithms:
/// `this->vptr = vtable` (`this+0x10`), then the base destructor is
/// tail-called with the same `this` and returns to `ret`. Charges 10 cycles.
pub(crate) fn forward_to_base_dtor(c: &mut Cpu, vtable: u32, base: u32, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx(vtable);
    let (this, word) = (c.g[V1][0] as u32, c.g[V0][0]);
    Addr(this.wrapping_add(0x10)).sw(c, word);
    call_ptr(c, base, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// Deleting destructor shape (`ScriptCameraAlgo`, `SubjectScriptCameraAlgo`,
/// `CameraTriggerMan_Member45bde0`): `this->vptr = ALGO_DTOR_VTABLE_45DDE0`,
/// then `operator delete(this)` when bit 0 of `a1` is set. `delete_ret` is
/// the return address of that call. Charges 7 cycles, plus 2 on the delete
/// path, and 3 on the way out.
pub(crate) fn vptr_delete_dtor(c: &mut Cpu, delete_ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = sx(ALGO_DTOR_VTABLE_45DDE0);
    frame.save_ra(c, 0);
    c.g[A1][0] &= 1;
    let (this, word) = (c.g[A0][0] as u32, c.g[V0][0]);
    Addr(this.wrapping_add(0x10)).sw(c, word);
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        call_ptr(c, OPERATOR_DELETE_317E50, delete_ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// A global constructor/destructor call (`CameraTriggerMan_GlobalCtors` and
/// friends): `callee(a0, a1)` with the arguments set here, returning to
/// `ret`. Charges 8 cycles.
pub(crate) fn static_init_call(c: &mut Cpu, a0: u64, a1: u64, callee: u32, ret: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = a0;
    c.g[A1][0] = a1;
    call_ptr(c, callee, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void BoundedCameraAlgorithm_dtor(void* this)` (0x1741d8): set the bounded
/// algorithm's vtable and tail-call the base destructor. Not deleting.
pub fn bounded_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(c, 0x0046_0000 - 0x3ba8, 0x0017_32b8, 0x0017_41f4)
    })
}

/// `void CameraTriggerMan_Member45bde0_dtor(void* this, int flags)` (0x176270).
pub fn camera_trigger_man_member45bde0_dtor(c: &mut Cpu) {
    guest(c, |c| vptr_delete_dtor(c, 0x0017_6294))
}

/// `void CameraTriggerMan_Member10_CallV05(void* this)` (0x1762f0): call the
/// virtual at `vtable+0x2c` (`this` adjusted by the `short` delta at `+0x28`).
pub fn camera_trigger_man_member10_call_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(this.wrapping_add(0x10)).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(vt.wrapping_add(0x28)).lh(c);
        c.g[A1][0] = Addr(vt.wrapping_add(0x2c)).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0017_630c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptCameraAlgo_dtor(void* this, int flags)` (0x176328).
pub fn script_camera_algo_dtor(c: &mut Cpu) {
    guest(c, |c| vptr_delete_dtor(c, 0x0017_634c))
}

/// `void SubjectScriptCameraAlgo_dtor(void* this, int flags)` (0x1763c8).
pub fn subject_script_camera_algo_dtor(c: &mut Cpu) {
    guest(c, |c| vptr_delete_dtor(c, 0x0017_63ec))
}

/// `CameraTriggerMan_GlobalCtors` (0x176a28): static init with `(1, 0xffff)`.
pub fn camera_trigger_man_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        static_init_call(c, 1, 0xffff, CAMERA_TRIGGER_MAN_STATIC_INIT_DESTROY, 0x0017_6a3c)
    })
}

/// `CameraTriggerMan_GlobalDtors` (0x176a48): static destroy with `(0, 0xffff)`.
pub fn camera_trigger_man_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        static_init_call(c, 0, 0xffff, CAMERA_TRIGGER_MAN_STATIC_INIT_DESTROY, 0x0017_6a5c)
    })
}

/// `void Default_2ChaseCameraAlgorithm_dtor(void* this)` (0x176aa8).
pub fn default_2_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(c, 0x0046_0000 - 0x3550, CHASE_CAMERA_BASE_DTOR_162458, 0x0017_6ac4)
    })
}

/// `void Default_4ChaseCameraAlgorithm_dtor(void* this)` (0x1770a8).
pub fn default_4_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(c, 0x0046_0000 - 0x3640, CHASE_CAMERA_BASE_DTOR_162458, 0x0017_70c4)
    })
}

/// `void DirectionNeChaseCameraAlgorithm_dtor(void* this)` (0x177520).
pub fn direction_ne_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(c, 0x0046_0000 - 0x34d8, CHASE_CAMERA_BASE_DTOR_162458, 0x0017_753c)
    })
}

/// `DirectionNeChaseCameraAlgorithm_GlobalCtor` (0x177e30): static init with `(1, 0xffff)`.
pub fn direction_ne_chase_camera_algorithm_global_ctor(c: &mut Cpu) {
    guest(c, |c| {
        static_init_call(c, 1, 0xffff, DIRECTION_NE_CHASE_STATIC_INIT, 0x0017_7e44)
    })
}

/// `void SpokeChaseCameraAlgorithm_dtor(void* this)` (0x177e90).
pub fn spoke_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(c, 0x0046_0000 - 0x3338, CHASE_CAMERA_BASE_DTOR_162458, 0x0017_7eac)
    })
}

/// `void SuperpipeChaseCameraAlgorithm_dtor(void* this)` (0x178588).
pub fn superpipe_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(c, 0x0046_0000 - 0x33b0, CHASE_CAMERA_BASE_DTOR_162458, 0x0017_85a4)
    })
}

/// `$gp - k`: the address of a `$gp`-relative float constant or global.
pub(crate) fn camera_gp_slot(c: &Cpu, k: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(k)
}

/// `VF[dst] = M * VF[k]` for the 4x4 matrix in VF4..VF7: the four VU0 ops
/// (`mula`, `madda`, `madda`, `madd`) with one broadcast component each.
pub(crate) fn camera_mat4_times_vec(c: &mut Cpu, k: u8, dst: u8) {
    c.vu0().mula(0b1111, 4, Src::Bc(k, X));
    c.vu0().madda(0b1111, 5, Src::Bc(k, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(k, Z));
    c.vu0().madd(0b1111, dst, 7, Src::Bc(k, W));
}

/// `void BoundedCameraAlgorithmBase_GetView(void* this, void* out)` (0x173e40):
/// the view pose of the bounded camera. The camera matrix at `this+0x40` is
/// copied to the stack, transformed twice by the VU0 matrix product (once per
/// `.lit4` basis at `gp-0xeb0`/`0x500000-0xea0`), the results stored as the
/// eight-float pose at `out`, and `tPose_tPose` (0x31b748) finishes it.
/// Block structure and the scratch registers mirror the generated body.
pub fn bounded_camera_algorithm_base_get_view(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 211;
        let frame = Frame::open(c, 0x110);
        c.g[V0][0] = 0x0050_0000;
        frame.save(c, S2, 0xd0);
        c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0xe60));
        frame.save(c, S5, 0xa0);
        c.g[S2][0] = c.g[A1][0];
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S0, 0xf0);
        frame.save(c, S1, 0xe0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 20, 0x100);
        let table = c.g[V0][0] as u32;
        c.lqc2(1, table);
        c.lqc2(2, table.wrapping_add(0x10));
        c.lqc2(3, table.wrapping_add(0x20));
        c.lqc2(4, table.wrapping_add(0x30));
        c.sqc2(1, frame.at(c, 0));
        c.sqc2(2, frame.at(c, 0x10));
        c.sqc2(3, frame.at(c, 0x20));
        c.sqc2(4, frame.at(c, 0x30));
        c.lqc2(4, frame.at(c, 0));
        c.lqc2(5, frame.at(c, 0x10));
        c.lqc2(6, frame.at(c, 0x20));
        c.lqc2(7, frame.at(c, 0x30));
        c.lqc2(8, (c.g[S2][0] as u32).wrapping_add(0x40));
        camera_mat4_times_vec(c, 8, 12);
        c.sqc2(12, frame.at(c, 0x30));
        c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x54));
        c.g[S3][0] = c.g[SP][0];
        c.g[A0][0] = sx(frame.at(c, 0x80));
        c.g[A1][0] = sx(frame.at(c, 0x84));
        c.neg_s(12, 12);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 20);
        c.g[S0][0] = 0x0050_0000;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xeb0));
        call_ptr(c, 0x0031_be50, 0x0017_3ee4)?;
        c.lwc1(11, frame.at(c, 0x84));
        c.g[S4][0] = sx(frame.at(c, 0x40));
        c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.sub_s(1, 20, 11);
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_sub(0xeb0));
        c.lwc1(6, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.lwc1(8, frame.at(c, 0x80));
        c.mul_s(2, 1, 3);
        c.sw(frame.at(c, 0x70), 0);
        c.mul_s(4, 1, 0);
        c.sw(frame.at(c, 0x74), 0);
        c.mul_s(1, 1, 6);
        c.sw(frame.at(c, 0x78), 0);
        c.mul_s(13, 8, 3);
        c.sw(frame.at(c, 0x4c), 0);
        c.mul_s(12, 8, 0);
        c.sw(frame.at(c, 0x5c), 0);
        c.mul_s(9, 2, 3);
        c.sw(frame.at(c, 0x6c), 0);
        c.mul_s(10, 4, 3);
        c.swc1(20, frame.at(c, 0x7c));
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
        c.swc1(5, frame.at(c, 0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, frame.at(c, 0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, frame.at(c, 0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, frame.at(c, 0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, frame.at(c, 0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, frame.at(c, 0x50));
        c.swc1(10, frame.at(c, 0x60));
        c.swc1(1, frame.at(c, 0x54));
        c.swc1(3, frame.at(c, 0x64));
        let s3 = c.g[S3][0] as u32;
        let s4 = c.g[S4][0] as u32;
        c.lqc2(4, s3);
        c.lqc2(5, s3.wrapping_add(0x10));
        c.lqc2(6, s3.wrapping_add(0x20));
        c.lqc2(7, s3.wrapping_add(0x30));
        c.lqc2(8, s4);
        c.lqc2(9, s4.wrapping_add(0x10));
        c.lqc2(10, s4.wrapping_add(0x20));
        c.lqc2(11, s4.wrapping_add(0x30));
        camera_mat4_times_vec(c, 8, 12);
        camera_mat4_times_vec(c, 9, 13);
        camera_mat4_times_vec(c, 10, 14);
        camera_mat4_times_vec(c, 11, 15);
        c.sqc2(12, s3);
        c.sqc2(13, s3.wrapping_add(0x10));
        c.sqc2(14, s3.wrapping_add(0x20));
        c.sqc2(15, s3.wrapping_add(0x30));
        c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x50));
        c.g[S0][0] = c.g[SP][0];
        c.g[A0][0] = sx(frame.at(c, 0x88));
        c.g[A1][0] = sx(frame.at(c, 0x8c));
        call_ptr(c, 0x0031_be50, 0x0017_4024)?;
        c.lwc1(11, frame.at(c, 0x8c));
        c.g[V0][0] = 0x0050_0000;
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_sub(0xea0));
        c.sub_s(1, 20, 11);
        c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.lwc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.lwc1(8, frame.at(c, 0x88));
        c.mul_s(2, 1, 3);
        c.swc1(20, frame.at(c, 0x7c));
        c.mul_s(4, 1, 0);
        c.sw(frame.at(c, 0x70), 0);
        c.mul_s(1, 1, 6);
        c.sw(frame.at(c, 0x74), 0);
        c.mul_s(13, 8, 3);
        c.sw(frame.at(c, 0x78), 0);
        c.mul_s(12, 8, 0);
        c.sw(frame.at(c, 0x4c), 0);
        c.mul_s(9, 2, 3);
        c.sw(frame.at(c, 0x5c), 0);
        c.mul_s(10, 4, 3);
        c.sw(frame.at(c, 0x6c), 0);
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
        c.swc1(5, frame.at(c, 0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, frame.at(c, 0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, frame.at(c, 0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, frame.at(c, 0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, frame.at(c, 0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, frame.at(c, 0x50));
        c.swc1(10, frame.at(c, 0x60));
        c.swc1(1, frame.at(c, 0x54));
        c.swc1(3, frame.at(c, 0x64));
        let s0 = c.g[S0][0] as u32;
        let s4 = c.g[S4][0] as u32;
        c.lqc2(4, s0);
        c.lqc2(5, s0.wrapping_add(0x10));
        c.lqc2(6, s0.wrapping_add(0x20));
        c.lqc2(7, s0.wrapping_add(0x30));
        c.lqc2(8, s4);
        c.lqc2(9, s4.wrapping_add(0x10));
        c.lqc2(10, s4.wrapping_add(0x20));
        c.lqc2(11, s4.wrapping_add(0x30));
        camera_mat4_times_vec(c, 8, 12);
        camera_mat4_times_vec(c, 9, 13);
        camera_mat4_times_vec(c, 10, 14);
        camera_mat4_times_vec(c, 11, 15);
        c.sqc2(12, s0);
        c.sqc2(13, s0.wrapping_add(0x10));
        c.sqc2(14, s0.wrapping_add(0x20));
        c.sqc2(15, s0.wrapping_add(0x30));
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0031_b748, 0x0017_4160)?;
        c.g[V0][0] = c.g[S5][0];
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void Default_2ChaseCameraAlgorithm_Update(void* this)` (0x176b10): a
/// straight run of the chase camera's per-frame steps (`0x162568` ..
/// `0x1643a8`), each with its `.lit4` float operands loaded from `$gp`.
/// The stack holds the step outputs (`sp+0..0x80`); `a1` is `sp` for every
/// step, as in the generated body.
pub fn default_2_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 116;
        let frame = Frame::open(c, 0xa0);
        c.lwc1(19, camera_gp_slot(c, 0x61d8));
        frame.save(c, S0, 0x80);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(12, camera_gp_slot(c, 0x61d4));
        c.lwc1(13, camera_gp_slot(c, 0x61d0));
        c.lwc1(14, camera_gp_slot(c, 0x61cc));
        c.lwc1(15, camera_gp_slot(c, 0x61c8));
        c.lwc1(16, camera_gp_slot(c, 0x61c4));
        c.lwc1(17, camera_gp_slot(c, 0x61c0));
        c.lwc1(18, camera_gp_slot(c, 0x61bc));
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        call_ptr(c, 0x0016_2568, 0x0017_6b50)?;
        c.mtc1(0, 20);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2998, 0x0017_6b60)?;
        c.lwc1(21, camera_gp_slot(c, 0x61b8));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2a20, 0x0017_6b70)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b80, 0x0017_6b7c)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2c78, 0x0017_6b88)?;
        c.lwc1(17, camera_gp_slot(c, 0x61b4));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(18, camera_gp_slot(c, 0x61b0));
        c.lwc1(12, camera_gp_slot(c, 0x61ac));
        c.lwc1(13, camera_gp_slot(c, 0x61a8));
        c.lwc1(14, camera_gp_slot(c, 0x61a4));
        c.g[AT][0] = 0x40a8_0000;
        c.mtc1(AT, 15);
        c.lwc1(16, camera_gp_slot(c, 0x61a0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3010, 0x0017_6bb4)?;
        c.lwc1(16, camera_gp_slot(c, 0x619c));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, camera_gp_slot(c, 0x6198));
        c.lwc1(13, camera_gp_slot(c, 0x6194));
        c.lwc1(14, camera_gp_slot(c, 0x6190));
        c.lwc1(15, camera_gp_slot(c, 0x618c));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3270, 0x0017_6bd4)?;
        c.lwc1(13, camera_gp_slot(c, 0x6188));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 14);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(15, camera_gp_slot(c, 0x6184));
        c.mov_s(12, 20);
        call_ptr(c, 0x0016_33b0, 0x0017_6bf4)?;
        c.lwc1(18, camera_gp_slot(c, 0x6180));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(19, camera_gp_slot(c, 0x617c));
        c.mov_s(17, 21);
        c.lwc1(12, camera_gp_slot(c, 0x6178));
        c.mov_s(16, 20);
        c.lwc1(13, camera_gp_slot(c, 0x6174));
        c.lwc1(14, camera_gp_slot(c, 0x6170));
        c.lwc1(15, camera_gp_slot(c, 0x616c));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3450, 0x0017_6c20)?;
        c.lwc1(16, camera_gp_slot(c, 0x6168));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, camera_gp_slot(c, 0x6164));
        c.lwc1(13, camera_gp_slot(c, 0x6160));
        c.lwc1(14, camera_gp_slot(c, 0x615c));
        c.lwc1(15, camera_gp_slot(c, 0x6158));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b90, 0x0017_6c40)?;
        c.lwc1(14, camera_gp_slot(c, 0x6154));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(15, camera_gp_slot(c, 0x6150));
        c.lwc1(13, camera_gp_slot(c, 0x614c));
        c.lwc1(12, camera_gp_slot(c, 0x6148));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_35f8, 0x0017_6c5c)?;
        c.lwc1(12, camera_gp_slot(c, 0x6144));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_4878, 0x0017_6c6c)?;
        c.lwc1(12, camera_gp_slot(c, 0x6140));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(13, camera_gp_slot(c, 0x613c));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_46a0, 0x0017_6c80)?;
        c.g[AT][0] = 0x4416_0000;
        c.mtc1(AT, 12);
        c.mov_s(15, 21);
        c.g[AT][0] = 0x4396_0000;
        c.mtc1(AT, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 14);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0016_41c0, 0x0017_6ca8)?;
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(AT, 12);
        c.g[A1][0] = c.g[SP][0];
        c.mov_s(14, 13);
        call_ptr(c, 0x0016_43a8, 0x0017_6cc8)?;
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// `void Default_4ChaseCameraAlgorithm_Update(void* this)` (0x177110): the
/// same per-frame step list as the Default_2 chase camera, with its own
/// `$gp` constants and callee return addresses.
pub fn default_4_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 115;
        let frame = Frame::open(c, 0xa0);
        c.lwc1(19, camera_gp_slot(c, 0x6064));
        frame.save(c, S0, 0x80);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(12, camera_gp_slot(c, 0x6060));
        c.lwc1(13, camera_gp_slot(c, 0x605c));
        c.lwc1(14, camera_gp_slot(c, 0x6058));
        c.lwc1(15, camera_gp_slot(c, 0x6054));
        c.lwc1(16, camera_gp_slot(c, 0x6050));
        c.lwc1(17, camera_gp_slot(c, 0x604c));
        c.lwc1(18, camera_gp_slot(c, 0x6048));
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        call_ptr(c, 0x0016_2568, 0x0017_7150)?;
        c.mtc1(0, 20);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2998, 0x0017_7160)?;
        c.lwc1(21, camera_gp_slot(c, 0x6044));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2a20, 0x0017_7170)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b80, 0x0017_717c)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2c78, 0x0017_7188)?;
        c.lwc1(17, camera_gp_slot(c, 0x6040));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(18, camera_gp_slot(c, 0x603c));
        c.lwc1(12, camera_gp_slot(c, 0x6038));
        c.lwc1(13, camera_gp_slot(c, 0x6034));
        c.lwc1(14, camera_gp_slot(c, 0x6030));
        c.lwc1(15, camera_gp_slot(c, 0x602c));
        c.lwc1(16, camera_gp_slot(c, 0x6028));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3010, 0x0017_71b0)?;
        c.lwc1(16, camera_gp_slot(c, 0x6024));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, camera_gp_slot(c, 0x6020));
        c.lwc1(13, camera_gp_slot(c, 0x601c));
        c.lwc1(14, camera_gp_slot(c, 0x6018));
        c.lwc1(15, camera_gp_slot(c, 0x6014));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3270, 0x0017_71d0)?;
        c.lwc1(13, camera_gp_slot(c, 0x6010));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 14);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(15, camera_gp_slot(c, 0x600c));
        c.mov_s(12, 20);
        call_ptr(c, 0x0016_33b0, 0x0017_71f0)?;
        c.lwc1(18, camera_gp_slot(c, 0x6008));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(19, camera_gp_slot(c, 0x6004));
        c.mov_s(17, 21);
        c.lwc1(12, camera_gp_slot(c, 0x6000));
        c.mov_s(16, 20);
        c.lwc1(13, camera_gp_slot(c, 0x5ffc));
        c.lwc1(14, camera_gp_slot(c, 0x5ff8));
        c.lwc1(15, camera_gp_slot(c, 0x5ff4));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3450, 0x0017_721c)?;
        c.lwc1(16, camera_gp_slot(c, 0x5ff0));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, camera_gp_slot(c, 0x5fec));
        c.lwc1(13, camera_gp_slot(c, 0x5fe8));
        c.lwc1(14, camera_gp_slot(c, 0x5fe4));
        c.lwc1(15, camera_gp_slot(c, 0x5fe0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b90, 0x0017_723c)?;
        c.lwc1(14, camera_gp_slot(c, 0x5fdc));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(15, camera_gp_slot(c, 0x5fd8));
        c.lwc1(13, camera_gp_slot(c, 0x5fd4));
        c.lwc1(12, camera_gp_slot(c, 0x5fd0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_35f8, 0x0017_7258)?;
        c.lwc1(12, camera_gp_slot(c, 0x5fcc));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_4878, 0x0017_7268)?;
        c.lwc1(12, camera_gp_slot(c, 0x5fc8));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(13, camera_gp_slot(c, 0x5fc4));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_46a0, 0x0017_727c)?;
        c.g[AT][0] = 0x4416_0000;
        c.mtc1(AT, 12);
        c.mov_s(15, 21);
        c.g[AT][0] = 0x4396_0000;
        c.mtc1(AT, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 14);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0016_41c0, 0x0017_72a4)?;
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(AT, 12);
        c.g[A1][0] = c.g[SP][0];
        c.mov_s(14, 13);
        call_ptr(c, 0x0016_43a8, 0x0017_72c4)?;
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// `void DirectionNeChaseCameraAlgorithm_Update(void* this)` (0x177368): the
/// chase camera steps again, with the second `.lit4` set and the `0x165540`
/// read of `this+0x390` at the end. Saves only `s0` and `f20`.
pub fn direction_ne_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 86;
        let frame = Frame::open(c, 0xa0);
        c.lwc1(19, camera_gp_slot(c, 0x5fbc));
        frame.save(c, S0, 0x80);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(12, camera_gp_slot(c, 0x5fb8));
        c.lwc1(13, camera_gp_slot(c, 0x5fb4));
        c.lwc1(14, camera_gp_slot(c, 0x5fb0));
        c.lwc1(15, camera_gp_slot(c, 0x5fac));
        c.lwc1(16, camera_gp_slot(c, 0x5fa8));
        c.lwc1(17, camera_gp_slot(c, 0x5fa4));
        c.lwc1(18, camera_gp_slot(c, 0x5fa0));
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 20, 0x90);
        call_ptr(c, 0x0016_2568, 0x0017_73a4)?;
        c.mtc1(0, 20);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2998, 0x0017_73b4)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2a20, 0x0017_73c0)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b80, 0x0017_73cc)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2c78, 0x0017_73d8)?;
        c.lwc1(17, camera_gp_slot(c, 0x5f9c));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(18, camera_gp_slot(c, 0x5f98));
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(12, camera_gp_slot(c, 0x5f94));
        c.mov_s(13, 20);
        c.lwc1(14, camera_gp_slot(c, 0x5f90));
        c.lwc1(16, camera_gp_slot(c, 0x5f8c));
        c.mov_s(15, 20);
        call_ptr(c, 0x0016_3010, 0x0017_7400)?;
        c.lwc1(16, camera_gp_slot(c, 0x5f88));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(13, camera_gp_slot(c, 0x5f84));
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.mov_s(14, 20);
        c.mov_s(15, 20);
        call_ptr(c, 0x0016_3270, 0x0017_7420)?;
        c.lwc1(13, camera_gp_slot(c, 0x5f80));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 14);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(15, camera_gp_slot(c, 0x5f7c));
        c.mov_s(12, 20);
        call_ptr(c, 0x0016_33b0, 0x0017_7440)?;
        c.lwc1(16, camera_gp_slot(c, 0x5f78));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, camera_gp_slot(c, 0x5f74));
        c.lwc1(13, camera_gp_slot(c, 0x5f70));
        c.lwc1(14, camera_gp_slot(c, 0x5f6c));
        c.lwc1(15, camera_gp_slot(c, 0x5f68));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b90, 0x0017_7460)?;
        c.lwc1(14, camera_gp_slot(c, 0x5f64));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(15, camera_gp_slot(c, 0x5f60));
        c.lwc1(13, camera_gp_slot(c, 0x5f5c));
        c.lwc1(12, camera_gp_slot(c, 0x5f58));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_35f8, 0x0017_747c)?;
        c.lwc1(12, camera_gp_slot(c, 0x5f54));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_4878, 0x0017_748c)?;
        c.lwc1(13, camera_gp_slot(c, 0x5f50));
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0016_46a0, 0x0017_74a0)?;
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x390));
        call_ptr(c, 0x0016_5540, 0x0017_74ac)?;
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// `void BoundedCameraAlgorithm_Update(void* this)` (0x174200): the bounded
/// camera's per-frame update. Three regions with the generated block
/// structure: the first picks the target from `this+0x80` (with a shortcut
/// when its `+0x24` flag is 1), then the VU0 offset of the rider's pose; the
/// second computes the blend from the `+0x74`/`+0x20` fields with a
/// division by `.lit4` constants; the third clamps `this+0x54` against the
/// `.lit4` limits, with a square root for the step length.
pub fn bounded_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xb0);
        'b_17432c: {
            'b_174280: {
                c.g[V1][0] = 0x1;
                frame.save(c, S0, 0x80);
                frame.save_ra(c, 0x70);
                c.g[S0][0] = c.g[A0][0];
                frame.save_fpr(c, 22, 0xa0);
                frame.save_fpr(c, 21, 0x98);
                frame.save_fpr(c, 20, 0x90);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x70));
                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x18));
                c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x6c));
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x1c));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x78));
                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x14));
                c.swc1(1, (c.g[S0][0] as u32));
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x74));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                    break 'b_174280;
                } else {
                    c.cycles += 10;
                    c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x28));
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 2);
                    c.swc1(1, frame.at(c, 0));
                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x2c));
                    c.swc1(0, frame.at(c, 4));
                    c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.swc1(2, frame.at(c, 0xc));
                    c.swc1(1, frame.at(c, 8));
                    break 'b_17432c;
                }
            }
            c.cycles += 43;
            c.g[A0][0] = sx(frame.at(c, 0x20));
            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0017_4298)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx(frame.at(c, 0x40));
            c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x74));
            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0017_42b8)?;
            c.mfc1(4, 20);
            c.lqc2(4, frame.at(c, 0x40));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x50));
            c.lq(2, frame.at(c, 0x50));
            c.sq(2, frame.at(c, 0x30));
            c.lqc2(3, frame.at(c, 0x20));
            c.lqc2(4, frame.at(c, 0x30));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x50));
            c.lq(2, frame.at(c, 0x50));
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6c));
            c.sq(2, frame.at(c, 0x10));
            c.g[V0][0] = 0x0050_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[V0][0] as u32));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x60));
            c.lq(2, frame.at(c, 0x60));
            c.sq(2, frame.at(c, 0x50));
            c.lqc2(3, frame.at(c, 0x10));
            c.lqc2(4, frame.at(c, 0x50));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x60));
            c.lq(2, frame.at(c, 0x60));
            c.sq(2, frame.at(c, 0));
            break 'b_17432c;
        }
        'b_1743f4: {
            'b_1743ec: {
                'b_1743b0: {
                    c.cycles += 22;
                    c.lq(2, frame.at(c, 0));
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
                    c.mtc1(0, 22);
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017_4358)?;
                    c.lqc2(3, frame.at(c, 0));
                    c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x20));
                    c.lq(2, frame.at(c, 0x20));
                    c.sq(2, frame.at(c, 0x10));
                    c.lwc1(20, frame.at(c, 0x10));
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    c.lwc1(21, frame.at(c, 0x14));
                    if t {
                        break 'b_1743b0;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(12, 21);
                        if t {
                            break 'b_1743ec;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(12, camera_gp_slot(c, 0x635c));
                            let t = !c.fcc();
                            c.lwc1(0, frame.at(c, 0x10));
                            if t {
                                break 'b_1743ec;
                            } else {
                                c.cycles += 3;
                                c.lwc1(12, camera_gp_slot(c, 0x6358));
                                c.lwc1(1, frame.at(c, 0x14));
                                break 'b_1743f4;
                            }
                        }
                    }
                }
                c.cycles += 7;
                c.div_s(12, 21, 20);
                call_ptr(c, 0x0031_c228, 0x0017_43bc)?;
                c.c_lt_s(20, 22);
                let t = !c.fcc();
                c.mov_s(12, 0);
                if t {
                    break 'b_1743ec;
                } else {
                    'b_1743e4: {
                        c.cycles += 3;
                        c.c_lt_s(22, 21);
                        let t = !c.fcc();
                        if t {
                            break 'b_1743e4;
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, camera_gp_slot(c, 0x6354));
                            c.add_s(12, 12, 0);
                            break 'b_1743ec;
                        }
                    }
                    c.cycles += 2;
                    c.lwc1(0, camera_gp_slot(c, 0x6350));
                    c.sub_s(12, 12, 0);
                    break 'b_1743ec;
                }
            }
            c.cycles += 2;
            c.lwc1(0, frame.at(c, 0x10));
            c.lwc1(1, frame.at(c, 0x14));
            break 'b_1743f4;
        }
        'b_174494: {
            'b_174490: {
                'b_174450: {
                    c.cycles += 12;
                    c.mul_s(2, 0, 0);
                    c.swc1(12, (c.g[S0][0] as u32).wrapping_add(0x50));
                    c.mul_s(3, 1, 1);
                    c.swc1(0, frame.at(c, 0x20));
                    c.mtc1(0, 22);
                    c.swc1(1, frame.at(c, 0x24));
                    c.add_s(2, 2, 3);
                    c.lwc1(21, frame.at(c, 0x18));
                    c.sqrt_s(20, 2);
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    if t {
                        break 'b_174450;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(0, 21);
                        if t {
                            break 'b_174490;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(0, camera_gp_slot(c, 0x634c));
                            if !c.fcc() {
                                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
                                break 'b_174494;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, camera_gp_slot(c, 0x6348));
                                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
                                break 'b_174494;
                            }
                        }
                    }
                }
                'b_17448c: {
                    c.cycles += 7;
                    c.div_s(12, 21, 20);
                    call_ptr(c, 0x0031_c228, 0x0017_445c)?;
                    c.c_lt_s(20, 22);
                    let t = !c.fcc();
                    c.mov_s(12, 0);
                    if t {
                        break 'b_17448c;
                    } else {
                        'b_174484: {
                            c.cycles += 3;
                            c.c_lt_s(22, 21);
                            let t = !c.fcc();
                            if t {
                                break 'b_174484;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, camera_gp_slot(c, 0x6344));
                                c.add_s(12, 12, 0);
                                break 'b_17448c;
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, camera_gp_slot(c, 0x6340));
                        c.sub_s(12, 12, 0);
                        break 'b_17448c;
                    }
                }
                c.cycles += 1;
                c.mov_s(0, 12);
                break 'b_174490;
            }
            c.cycles += 1;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
            break 'b_174494;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 22, 0xa0);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// `void BoundedCameraAlgorithmBase_ResetForRider(void* this, void* rider)`
/// (0x1744b0): reset the bounded camera for a rider. Calls the rider's
/// pose query (`this+0x30` -> `+0x4` -> `+0x6c0` vtable `+0x38/+0x3c`), then
/// either finishes early (the rider's object is not the one `this` tracks),
/// or copies the rider's `.lit4` offsets into the camera and runs the
/// base reset (`0x1732e8`) and the same clamp logic as the update.
pub fn bounded_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xc0);
        'b_174780: {
            frame.save(c, S0, 0x90);
            frame.save(c, S1, 0x80);
            c.g[S0][0] = c.g[A0][0];
            frame.save_ra(c, 0x70);
            c.g[S1][0] = c.g[A1][0];
            frame.save_fpr(c, 22, 0xb0);
            frame.save_fpr(c, 21, 0xa8);
            frame.save_fpr(c, 20, 0xa0);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0017_44f4)?;
            if c.g[V0][0] != c.g[S1][0] {
                frame.restore(c, S0, 0x90);
                break 'b_174780;
            } else {
                'b_1745e4: {
                    'b_174538: {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                        c.g[V1][0] = 0x1;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
                        if c.g[V0][0] != c.g[V1][0] {
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                            break 'b_174538;
                        } else {
                            c.cycles += 10;
                            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x28));
                            c.g[AT][0] = 0x3f80_0000;
                            c.mtc1(AT, 2);
                            c.swc1(1, frame.at(c, 0));
                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x2c));
                            c.swc1(0, frame.at(c, 4));
                            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x30));
                            c.swc1(2, frame.at(c, 0xc));
                            c.swc1(1, frame.at(c, 8));
                            break 'b_1745e4;
                        }
                    }
                    c.cycles += 43;
                    c.g[A0][0] = sx(frame.at(c, 0x20));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                    c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x0017_4550)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = sx(frame.at(c, 0x40));
                    c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x74));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                    c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                    let t = c.g[V1][0] as u32;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x0017_4570)?;
                    c.mfc1(4, 20);
                    c.lqc2(4, frame.at(c, 0x40));
                    c.qmtc2(4, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x50));
                    c.lq(2, frame.at(c, 0x50));
                    c.sq(2, frame.at(c, 0x30));
                    c.lqc2(3, frame.at(c, 0x20));
                    c.lqc2(4, frame.at(c, 0x30));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x50));
                    c.lq(2, frame.at(c, 0x50));
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6c));
                    c.sq(2, frame.at(c, 0x10));
                    c.g[V0][0] = 0x0050_0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[V0][0] as u32));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x60));
                    c.lq(2, frame.at(c, 0x60));
                    c.sq(2, frame.at(c, 0x50));
                    c.lqc2(3, frame.at(c, 0x10));
                    c.lqc2(4, frame.at(c, 0x50));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x60));
                    c.lq(2, frame.at(c, 0x60));
                    c.sq(2, frame.at(c, 0));
                    break 'b_1745e4;
                }
                'b_1746dc: {
                    'b_1746d4: {
                        'b_174698: {
                            c.cycles += 34;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.mtc1(0, 22);
                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x70));
                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x6c));
                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x78));
                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                            c.swc1(1, (c.g[S0][0] as u32));
                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x74));
                            call_ptr(c, 0x0017_32e8, 0x0017_4620)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x24));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0017_4640)?;
                            c.lqc2(3, frame.at(c, 0));
                            c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x40));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x20));
                            c.lq(2, frame.at(c, 0x20));
                            c.sq(2, frame.at(c, 0x10));
                            c.lwc1(20, frame.at(c, 0x10));
                            c.c_eq_s(20, 22);
                            let t = !c.fcc();
                            c.lwc1(21, frame.at(c, 0x14));
                            if t {
                                break 'b_174698;
                            } else {
                                c.cycles += 4;
                                c.c_eq_s(21, 22);
                                let t = c.fcc();
                                c.mov_s(12, 21);
                                if t {
                                    break 'b_1746d4;
                                } else {
                                    c.cycles += 4;
                                    c.c_le_s(22, 21);
                                    c.lwc1(12, camera_gp_slot(c, 0x633c));
                                    let t = !c.fcc();
                                    c.lwc1(0, frame.at(c, 0x10));
                                    if t {
                                        break 'b_1746d4;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(12, camera_gp_slot(c, 0x6338));
                                        c.lwc1(1, frame.at(c, 0x14));
                                        break 'b_1746dc;
                                    }
                                }
                            }
                        }
                        c.cycles += 7;
                        c.div_s(12, 21, 20);
                        call_ptr(c, 0x0031_c228, 0x0017_46a4)?;
                        c.c_lt_s(20, 22);
                        let t = !c.fcc();
                        c.mov_s(12, 0);
                        if t {
                            break 'b_1746d4;
                        } else {
                            'b_1746cc: {
                                c.cycles += 3;
                                c.c_lt_s(22, 21);
                                let t = !c.fcc();
                                if t {
                                    break 'b_1746cc;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, camera_gp_slot(c, 0x6334));
                                    c.add_s(12, 12, 0);
                                    break 'b_1746d4;
                                }
                            }
                            c.cycles += 2;
                            c.lwc1(0, camera_gp_slot(c, 0x6330));
                            c.sub_s(12, 12, 0);
                            break 'b_1746d4;
                        }
                    }
                    c.cycles += 2;
                    c.lwc1(0, frame.at(c, 0x10));
                    c.lwc1(1, frame.at(c, 0x14));
                    break 'b_1746dc;
                }
                'b_17477c: {
                    'b_174778: {
                        'b_174738: {
                            c.cycles += 12;
                            c.mul_s(2, 0, 0);
                            c.swc1(12, (c.g[S0][0] as u32).wrapping_add(0x50));
                            c.mul_s(3, 1, 1);
                            c.swc1(0, frame.at(c, 0x20));
                            c.mtc1(0, 22);
                            c.swc1(1, frame.at(c, 0x24));
                            c.add_s(2, 2, 3);
                            c.lwc1(21, frame.at(c, 0x18));
                            c.sqrt_s(20, 2);
                            c.c_eq_s(20, 22);
                            let t = !c.fcc();
                            if t {
                                break 'b_174738;
                            } else {
                                c.cycles += 4;
                                c.c_eq_s(21, 22);
                                let t = c.fcc();
                                c.mov_s(0, 21);
                                if t {
                                    break 'b_174778;
                                } else {
                                    c.cycles += 4;
                                    c.c_le_s(22, 21);
                                    c.lwc1(0, camera_gp_slot(c, 0x632c));
                                    if !c.fcc() {
                                        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
                                        break 'b_17477c;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(0, camera_gp_slot(c, 0x6328));
                                        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
                                        break 'b_17477c;
                                    }
                                }
                            }
                        }
                        'b_174774: {
                            c.cycles += 7;
                            c.div_s(12, 21, 20);
                            call_ptr(c, 0x0031_c228, 0x0017_4744)?;
                            c.c_lt_s(20, 22);
                            let t = !c.fcc();
                            c.mov_s(12, 0);
                            if t {
                                break 'b_174774;
                            } else {
                                'b_17476c: {
                                    c.cycles += 3;
                                    c.c_lt_s(22, 21);
                                    let t = !c.fcc();
                                    if t {
                                        break 'b_17476c;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(0, camera_gp_slot(c, 0x6324));
                                        c.add_s(12, 12, 0);
                                        break 'b_174774;
                                    }
                                }
                                c.cycles += 2;
                                c.lwc1(0, camera_gp_slot(c, 0x6320));
                                c.sub_s(12, 12, 0);
                                break 'b_174774;
                            }
                        }
                        c.cycles += 1;
                        c.mov_s(0, 12);
                        break 'b_174778;
                    }
                    c.cycles += 1;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
                    break 'b_17477c;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x90);
                break 'b_174780;
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 22, 0xb0);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// `void SpokeChaseCameraAlgorithm_ResetForRider(void* this, void* rider)`
/// (0x1783e0): the spoke chase camera's rider reset. Reads the rider's pose
/// as the bounded reset does; when it matches, it computes the desired eye
/// and focus point, scales the rider's offset through VU0, and runs the chase
/// base reset (`0x166c60`) and init (`0x166550`) before copying the offset
/// back into `this+0x20`.
pub fn spoke_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x40);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, t, 0x0017_841c)?;
        let rider_differs = c.g[S2][0] != c.g[V0][0];
        frame.restore(c, S0, 0x70);
        if !rider_differs {
            c.cycles += 58;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.g[SP][0];
            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0017_8440)?;
            c.lq(5, frame.at(c, 0));
            c.g[S0][0] = sx(frame.at(c, 0x10));
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.g[S0][0];
            c.sq(5, (c.g[S1][0] as u32).wrapping_add(0x390));
            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0017_8468)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0017_7fc0, 0x0017_8478)?;
            c.lq(2, frame.at(c, 0));
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x40));
            call_ptr(c, 0x0017_7ef8, 0x0017_848c)?;
            c.g[V1][0] = 0x0050_0000;
            c.g[AT][0] = 0x4214_0000;
            c.mtc1(AT, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
            c.g[A1][0] = c.g[S2][0];
            c.mfc1(2, 0);
            c.lqc2(4, (c.g[V1][0] as u32));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x30));
            c.g[A0][0] = c.g[S1][0];
            c.lq(2, frame.at(c, 0x30));
            c.sq(2, frame.at(c, 0x20));
            c.lqc2(3, frame.at(c, 0x10));
            c.lqc2(4, frame.at(c, 0x20));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x30));
            c.lq(2, frame.at(c, 0x30));
            c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x20));
            c.sq(2, frame.at(c, 0));
            call_ptr(c, 0x0016_6c60, 0x0017_84e0)?;
            c.lwc1(12, camera_gp_slot(c, 0x5dc4));
            c.g[AT][0] = 0x4396_0000;
            c.mtc1(AT, 13);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0016_6550, 0x0017_84f4)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0017_8508)?;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::{check, Rng, DATA};

    /// The object the tests point `a0` at, clear of the random vtables
    /// `vtable` draws from (`DATA..DATA+0xfffff`).
    const OBJ: u32 = DATA + 0x20_0000;

    /// Zero the first `words` words of the test object.
    fn zero_object(c: &mut Cpu, words: u32) {
        for i in 0..words {
            c.bus.write32(OBJ + 4 * i, 0);
        }
    }

    /// `words` random floats (within +-4) at `base`.
    fn floats(c: &mut Cpu, r: &mut Rng, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, r.f32(4.0));
        }
    }

    /// The update fixture: `this` zeroed, `this+0x80` points at a record of
    /// random floats whose `+0x24` flag is `seed & 1` (both branches).
    fn update_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        zero_object(c, 0x40);
        let rec = OBJ + 0x300;
        floats(c, r, rec, 0x14);
        c.bus.write32(rec + 0x24, seed & 1);
        c.bus.write32(OBJ + 0x80, rec);
        c.g[A0][0] = OBJ as u64;
    }

    /// The reset fixture: `a1` is the rider. The first fake call (the rider's
    /// pose query, `call 0`) returns the rider on even seeds (the equal
    /// branch) and something else on odd ones.
    fn reset_fixture(c: &mut Cpu, seed: u32) {
        zero_object(c, 0x40);
        let rider = OBJ + 0x400;
        c.g[A0][0] = OBJ as u64;
        c.g[A1][0] = rider as u64;
        let forced = if seed % 2 == 0 { rider } else { rider + 0x10 };
        c.bus.write32(crate::difftest::fakes::LOG_RESULTS, forced);
    }

    #[test]
    fn bounded_camera_algorithm_base_get_view_matches() {
        check(
            ee::s02::f_00173e40__generated,
            bounded_camera_algorithm_base_get_view,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x173e40, 3);
                zero_object(c, 0x40);
                let camera = OBJ + 0x100;
                floats(c, &mut r, camera, 0x20);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = camera as u64;
            },
        );
    }

    #[test]
    fn bounded_camera_algorithm_update_matches() {
        check(
            ee::s02::f_00174200__generated,
            bounded_camera_algorithm_update,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x174200, 3);
                update_fixture(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn bounded_camera_algorithm_reset_for_rider_matches() {
        check(
            ee::s02::f_001744b0__generated,
            bounded_camera_algorithm_reset_for_rider,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x1744b0, 3);
                reset_fixture(c, seed);
            },
        );
    }

    #[test]
    fn default_2_chase_camera_algorithm_update_matches() {
        check(
            ee::s02::f_00176b10__generated,
            default_2_chase_camera_algorithm_update,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x176b10, 3);
                update_fixture(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn default_4_chase_camera_algorithm_update_matches() {
        check(
            ee::s02::f_00177110__generated,
            default_4_chase_camera_algorithm_update,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x177110, 3);
                update_fixture(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn direction_ne_chase_camera_algorithm_update_matches() {
        check(
            ee::s02::f_00177368__generated,
            direction_ne_chase_camera_algorithm_update,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x177368, 3);
                update_fixture(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn spoke_chase_camera_algorithm_reset_for_rider_matches() {
        check(
            ee::s02::f_001783e0__generated,
            spoke_chase_camera_algorithm_reset_for_rider,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x1783e0, 3);
                reset_fixture(c, seed);
            },
        );
    }

    #[test]
    fn bounded_camera_algorithm_dtor_matches() {
        check(ee::s02::f_001741d8__generated, bounded_camera_algorithm_dtor, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x1741d8, 1);
            zero_object(c, 0x40);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn camera_trigger_man_member45bde0_dtor_matches() {
        check(
            ee::s02::f_00176270__generated,
            camera_trigger_man_member45bde0_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x176270, 1);
                c.g[A0][0] = OBJ as u64;
                // Even flags: no `operator delete` call (the delete path
                // would free into the empty test heap).
                c.g[A1][0] = (r.next_u32() & !1) as u64;
            },
        );
    }

    #[test]
    fn camera_trigger_man_member10_call_v05_matches() {
        check(
            ee::s02::f_001762f0__generated,
            camera_trigger_man_member10_call_v05,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1762f0, 1);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(OBJ + 0x10, vt);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn script_camera_algo_dtor_matches() {
        check(ee::s02::f_00176328__generated, script_camera_algo_dtor, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x176328, 1);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = (r.next_u32() & !1) as u64;
        });
    }

    #[test]
    fn subject_script_camera_algo_dtor_matches() {
        check(ee::s02::f_001763c8__generated, subject_script_camera_algo_dtor, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x1763c8, 1);
            c.g[A0][0] = OBJ as u64;
            c.g[A1][0] = (r.next_u32() & !1) as u64;
        });
    }

    #[test]
    fn camera_trigger_man_global_ctors_matches() {
        check(ee::s02::f_00176a28__generated, camera_trigger_man_global_ctors, 20, &|c, seed| {
            let _ = machine(c, seed, 0x176a28, 1);
        });
    }

    #[test]
    fn camera_trigger_man_global_dtors_matches() {
        check(ee::s02::f_00176a48__generated, camera_trigger_man_global_dtors, 20, &|c, seed| {
            let _ = machine(c, seed, 0x176a48, 1);
        });
    }

    #[test]
    fn default_2_chase_camera_algorithm_dtor_matches() {
        check(
            ee::s02::f_00176aa8__generated,
            default_2_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x176aa8, 1);
                zero_object(c, 0x40);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn default_4_chase_camera_algorithm_dtor_matches() {
        check(
            ee::s02::f_001770a8__generated,
            default_4_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1770a8, 1);
                zero_object(c, 0x40);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn direction_ne_chase_camera_algorithm_dtor_matches() {
        check(
            ee::s02::f_00177520__generated,
            direction_ne_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x177520, 1);
                zero_object(c, 0x40);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn direction_ne_chase_camera_algorithm_global_ctor_matches() {
        check(
            ee::s02::f_00177e30__generated,
            direction_ne_chase_camera_algorithm_global_ctor,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x177e30, 1);
            },
        );
    }

    #[test]
    fn spoke_chase_camera_algorithm_dtor_matches() {
        check(
            ee::s02::f_00177e90__generated,
            spoke_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x177e90, 1);
                zero_object(c, 0x40);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn superpipe_chase_camera_algorithm_dtor_matches() {
        check(
            ee::s02::f_00178588__generated,
            superpipe_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x178588, 1);
                zero_object(c, 0x40);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }
}
