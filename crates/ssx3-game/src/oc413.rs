//! Staging module oc413 (wave wave218): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc413`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0027_f9f8,
    rider_script_obj_activate,
    "RiderScriptObj_Activate",
)];

/// Direct callees of `RiderScriptObj_Activate`.
mod rider_activate_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const SCRIPT_OBJ_ACTIVATE: Callee = Callee {
        addr: 0x0028_3430,
        f: ee::s07::ScriptObj_Activate,
    };
    pub const SCRIPT_RIDER_GET_PARAMS: Callee = Callee {
        addr: 0x0027_c958,
        f: ee::s07::ScriptRider_GetParams,
    };
    pub const SCRIPT_ENGINE_GET_TARGET_POSE: Callee = Callee {
        addr: 0x0027_a0d8,
        f: ee::s07::ScriptEngine_GetTargetPose,
    };
    pub const SCRIPT_DATA_GET_INFO: Callee = Callee {
        addr: 0x0027_c070,
        f: ee::s07::ScriptData_GetInfo,
    };
    pub const SCRIPT_ENGINE_FIND_ANIM_BANK: Callee = Callee {
        addr: 0x0027_9ed8,
        f: ee::s07::ScriptEngine_FindAnimBank,
    };
    pub const SCRIPT_ENGINE_FIND_SOUND_BANK: Callee = Callee {
        addr: 0x0027_9ef8,
        f: ee::s07::ScriptEngine_FindSoundBank,
    };
}

/// Identity matrix constant, and the X/Y/Z axis vectors (`t4vect_*Axis`).
pub(crate) const RIDER_ACTIVATE_IDENTITY: u32 = 0x004f_f1a0;
pub(crate) const RIDER_ACTIVATE_Z_AXIS: u32 = 0x004f_f160;
pub(crate) const RIDER_ACTIVATE_Y_AXIS: u32 = 0x004f_f150;
pub(crate) const RIDER_ACTIVATE_X_AXIS: u32 = 0x004f_f140;
/// `gp - 0x478c`: degrees to radians (pi / 180).
pub(crate) const RIDER_ACTIVATE_DEG2RAD_GP: u32 = 0x478c;
/// `gp - 0x84c`: the script engine pointer.
pub(crate) const RIDER_ACTIVATE_ENGINE_GP: u32 = 0x84c;

/// Row-vector times 4x4 matrix chain `dst = a * b` with the VU0 macro ops the
/// compiled code uses (`a` rows in vf4..vf7, `b` rows in vf8..vf11, result rows
/// vf12..vf15 stored at `dst`).
fn rider_activate_mat_mul(c: &mut Cpu, a: u32, b: u32, dst: u32) {
    c.lqc2(4, a);
    c.lqc2(5, a.wrapping_add(0x10));
    c.lqc2(6, a.wrapping_add(0x20));
    c.lqc2(7, a.wrapping_add(0x30));
    c.lqc2(8, b);
    c.lqc2(9, b.wrapping_add(0x10));
    c.lqc2(10, b.wrapping_add(0x20));
    c.lqc2(11, b.wrapping_add(0x30));
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
    c.sqc2(12, dst);
    c.sqc2(13, dst.wrapping_add(0x10));
    c.sqc2(14, dst.wrapping_add(0x20));
    c.sqc2(15, dst.wrapping_add(0x30));
}

/// Copy the 4x4 matrix at `src` to `dst` through vf1..vf4.
fn rider_activate_mat_copy(c: &mut Cpu, src: u32, dst: u32) {
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
}

/// The axis-angle rotation matrix written at `m` after `Math_SinCos` has stored
/// sin at `sin_p` and cos at `cos_p` (f1 = 1 - cos; f20 = 1.0). `axis` is the
/// unit axis vector's address.
fn rider_activate_rot_matrix(c: &mut Cpu, sin_p: u32, cos_p: u32, axis: u32, m: u32) {
    c.lwc1(11, cos_p);
    c.lwc1(3, axis.wrapping_add(0x8));
    c.sub_s(1, 20, 11);
    c.lwc1(0, axis);
    c.lwc1(6, axis.wrapping_add(0x4));
    c.lwc1(8, sin_p);
    c.mul_s(2, 1, 3);
    c.sw(m.wrapping_add(0x30), 0u64);
    c.mul_s(4, 1, 0);
    c.sw(m.wrapping_add(0x34), 0u64);
    c.mul_s(1, 1, 6);
    c.sw(m.wrapping_add(0x38), 0u64);
    c.mul_s(13, 8, 3);
    c.sw(m.wrapping_add(0xc), 0u64);
    c.mul_s(12, 8, 0);
    c.sw(m.wrapping_add(0x1c), 0u64);
    c.mul_s(9, 2, 3);
    c.sw(m.wrapping_add(0x2c), 0u64);
    c.mul_s(10, 4, 3);
    c.swc1(20, m.wrapping_add(0x3c));
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
    c.swc1(5, m.wrapping_add(0x8));
    c.add_s(7, 7, 11);
    c.swc1(0, m.wrapping_add(0x4));
    c.add_s(4, 4, 13);
    c.swc1(2, m.wrapping_add(0x18));
    c.sub_s(10, 10, 8);
    c.swc1(9, m.wrapping_add(0x28));
    c.add_s(1, 1, 11);
    c.swc1(7, m);
    c.add_s(3, 3, 12);
    c.swc1(4, m.wrapping_add(0x10));
    c.swc1(10, m.wrapping_add(0x20));
    c.swc1(1, m.wrapping_add(0x14));
    c.swc1(3, m.wrapping_add(0x24));
}

/// `void RiderScriptObj_Activate(RiderScriptObj *self)` (0x27f9f8,
/// `cScriptObjRider_OnStartObjectUse`): activate the base script object
/// (`self + 0xc`), then build the object's local transform at `self + 0x30`
/// from the rider's script params (base offset and rotation in degrees at
/// info +0x8..+0x1c, converted to radians into `self + 0x70..0x78`), compose it
/// with the target pose the script engine returns for the object, add the
/// pose's rotation to the stored angles, and look up the animation and sound
/// banks of the object's script data (`self + 0x1c`, `self + 0x20`).
pub fn rider_script_obj_activate(c: &mut Cpu) {
    use rider_activate_callee::*;
    guest(c, |c| {
        c.cycles += 638;
        let frame = Frame::open(c, 0x1c0);
        let sp0 = sp(c);
        frame.save(c, 17, 0x190);
        c.g[S1][0] = c.g[A0][0];
        let this = c.g[S1][0] as u32;
        frame.save(c, 16, 0x1a0);
        frame.save(c, 20, 0x160);
        c.g[A0][0] = sx(this.wrapping_add(0xc));
        frame.save(c, 18, 0x180);
        c.g[S4][0] = sx(this.wrapping_add(0x30));
        let mat = this.wrapping_add(0x30);
        frame.save(c, 19, 0x170);
        frame.save(c, 21, 0x150);
        frame.save(c, 22, 0x140);
        frame.save(c, 23, 0x130);
        frame.save(c, 30, 0x120);
        frame.save_ra(c, 0x110);
        frame.save_fpr(c, 20, 0x1b0);
        SCRIPT_OBJ_ACTIVATE.call(c, 0x0027_fa38)?;

        // sp+0x10c: the script object (self + 8)
        c.g[V0][0] = c.lw(this.wrapping_add(0x8));
        c.g[S0][0] = 0x0050_0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xe60));
        c.sw(sp0.wrapping_add(0x10c), c.g[V0][0]);
        rider_activate_mat_copy(c, RIDER_ACTIVATE_IDENTITY, mat);
        c.sw(this.wrapping_add(0x70), 0u64);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
        c.g[A0][0] = c.lw(sp0.wrapping_add(0x10c));
        c.sw(this.wrapping_add(0x74), 0u64);
        c.sw(this.wrapping_add(0x78), 0u64);
        SCRIPT_RIDER_GET_PARAMS.call(c, 0x0027_fa84)?;
        c.g[S2][0] = c.g[V0][0];
        let info = c.g[S2][0] as u32;
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(RIDER_ACTIVATE_DEG2RAD_GP));
        c.lwc1(1, info.wrapping_add(0x1c));
        let local = sp0.wrapping_add(0x10); // local matrix m
        c.g[S3][0] = sx(local);
        c.mul_s(1, 1, 2);
        c.swc1(1, this.wrapping_add(0x70));
        c.lwc1(0, info.wrapping_add(0x18));
        c.mul_s(0, 0, 2);
        c.swc1(0, this.wrapping_add(0x74));
        c.lwc1(1, info.wrapping_add(0x14));
        c.mul_s(1, 1, 2);
        c.swc1(1, this.wrapping_add(0x78));
        // base offset vector (x, y, z, 1) at sp+0
        c.lwc1(0, info.wrapping_add(0x8));
        c.lwc1(1, info.wrapping_add(0xc));
        c.lwc1(2, info.wrapping_add(0x10));
        c.swc1(0, sp0);
        c.swc1(1, sp0.wrapping_add(0x4));
        c.swc1(2, sp0.wrapping_add(0x8));
        c.swc1(20, sp0.wrapping_add(0xc));
        // m = identity; m.row3 = offset * m
        let identity = (c.g[S0][0]) as u32;
        rider_activate_mat_copy(c, identity, local);
        c.lqc2(4, local);
        c.lqc2(5, local.wrapping_add(0x10));
        c.lqc2(6, local.wrapping_add(0x20));
        c.lqc2(7, local.wrapping_add(0x30));
        c.lqc2(8, sp0);
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, local.wrapping_add(0x30));

        // Rotate m about Z by -self.rz (angle in f12)
        c.lwc1(12, this.wrapping_add(0x78));
        c.g[V1][0] = 0x0050_0000;
        c.g[S7][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
        c.g[A0][0] = sx(sp0.wrapping_add(0xd0));
        c.neg_s(12, 12);
        c.g[A1][0] = sx(sp0.wrapping_add(0xd4));
        call_ptr(c, 0x0031_be50, 0x0027_fb34)?;
        c.g[V0][0] = 0x0050_0000;
        c.g[S0][0] = sx(sp0.wrapping_add(0x50));
        let s7 = c.g[S7][0] as u32;
        rider_activate_rot_matrix(
            c,
            sp0.wrapping_add(0xd0),
            sp0.wrapping_add(0xd4),
            s7,
            sp0.wrapping_add(0x50),
        );
        rider_activate_mat_mul(c, local, sp0.wrapping_add(0x50), local);

        // about Y by self.ry
        c.lwc1(12, this.wrapping_add(0x74));
        c.g[A0][0] = sx(sp0.wrapping_add(0xd8));
        c.g[A1][0] = sx(sp0.wrapping_add(0xdc));
        call_ptr(c, 0x0031_be50, 0x0027_fc74)?;
        c.g[V1][0] = 0x0050_0000;
        c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeb0));
        let s6 = c.g[S6][0] as u32;
        rider_activate_rot_matrix(
            c,
            sp0.wrapping_add(0xd8),
            sp0.wrapping_add(0xdc),
            s6,
            sp0.wrapping_add(0x50),
        );
        rider_activate_mat_mul(c, local, sp0.wrapping_add(0x50), local);

        // about X by self.rx
        c.lwc1(12, this.wrapping_add(0x70));
        c.g[A0][0] = sx(sp0.wrapping_add(0xe0));
        c.g[A1][0] = sx(sp0.wrapping_add(0xe4));
        c.g[FP][0] = 0x0050_0000;
        call_ptr(c, 0x0031_be50, 0x0027_fdb8)?;
        c.g[S5][0] = sx((c.g[FP][0] as u32).wrapping_sub(0xec0));
        let s5 = c.g[S5][0] as u32;
        rider_activate_rot_matrix(
            c,
            sp0.wrapping_add(0xe0),
            sp0.wrapping_add(0xe4),
            s5,
            sp0.wrapping_add(0x50),
        );
        rider_activate_mat_mul(c, local, sp0.wrapping_add(0x50), local);

        // target pose: (pos, rx, ry, rz) into sp+0x50, 0xe8, 0xec, 0xf0
        c.g[V0][0] = c.lw(sp0.wrapping_add(0x10c));
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = c.lb(info.wrapping_add(0x1));
        c.g[T0][0] = sx(sp0.wrapping_add(0xe8));
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[T1][0] = sx(sp0.wrapping_add(0xec));
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(RIDER_ACTIVATE_ENGINE_GP));
        c.g[T2][0] = sx(sp0.wrapping_add(0xf0));
        SCRIPT_ENGINE_GET_TARGET_POSE.call(c, 0x0027_ff08)?;
        // mat.row3 = pos * mat
        c.lqc2(4, mat);
        c.lqc2(5, mat.wrapping_add(0x10));
        c.lqc2(6, mat.wrapping_add(0x20));
        c.lqc2(7, mat.wrapping_add(0x30));
        c.lqc2(8, sp0.wrapping_add(0x50));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, mat.wrapping_add(0x30));
        // about Z by -rz
        c.lwc1(12, sp0.wrapping_add(0xf0));
        c.g[A0][0] = sx(sp0.wrapping_add(0xf4));
        c.g[A1][0] = sx(sp0.wrapping_add(0xf8));
        c.neg_s(12, 12);
        call_ptr(c, 0x0031_be50, 0x0027_ff44)?;
        c.g[V1][0] = 0x0050_0000;
        c.g[V0][0] = sx(sp0.wrapping_add(0x90));
        rider_activate_rot_matrix(
            c,
            sp0.wrapping_add(0xf4),
            sp0.wrapping_add(0xf8),
            s7,
            sp0.wrapping_add(0x90),
        );
        rider_activate_mat_mul(c, mat, sp0.wrapping_add(0x90), mat);
        // about Y by ry
        c.lwc1(12, sp0.wrapping_add(0xec));
        c.g[A0][0] = sx(sp0.wrapping_add(0xfc));
        c.g[A1][0] = sx(sp0.wrapping_add(0x100));
        call_ptr(c, 0x0031_be50, 0x0028_0084)?;
        c.g[V0][0] = 0x0050_0000;
        c.g[S0][0] = sx(sp0.wrapping_add(0x60));
        rider_activate_rot_matrix(
            c,
            sp0.wrapping_add(0xfc),
            sp0.wrapping_add(0x100),
            s6,
            sp0.wrapping_add(0x60),
        );
        rider_activate_mat_mul(c, mat, sp0.wrapping_add(0x60), mat);
        // about X by rx
        c.lwc1(12, sp0.wrapping_add(0xe8));
        c.g[A0][0] = sx(sp0.wrapping_add(0x104));
        c.g[A1][0] = sx(sp0.wrapping_add(0x108));
        call_ptr(c, 0x0031_be50, 0x0028_01c4)?;
        rider_activate_rot_matrix(
            c,
            sp0.wrapping_add(0x104),
            sp0.wrapping_add(0x108),
            s5,
            sp0.wrapping_add(0x60),
        );
        rider_activate_mat_mul(c, mat, sp0.wrapping_add(0x60), mat);
        // mat = mat * m
        rider_activate_mat_mul(c, mat, local, mat);
        // angles += pose rotation
        c.lwc1(3, sp0.wrapping_add(0xe8));
        c.lwc1(4, sp0.wrapping_add(0xec));
        c.lwc1(5, sp0.wrapping_add(0xf0));
        c.lwc1(0, this.wrapping_add(0x70));
        c.lwc1(1, this.wrapping_add(0x74));
        c.lwc1(2, this.wrapping_add(0x78));
        c.add_s(0, 0, 3);
        c.add_s(1, 1, 4);
        c.add_s(2, 2, 5);
        c.swc1(0, this.wrapping_add(0x70));
        c.swc1(1, this.wrapping_add(0x74));
        c.swc1(2, this.wrapping_add(0x78));
        // anim / sound banks of the script data
        c.g[V1][0] = c.lw(sp0.wrapping_add(0x10c));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        SCRIPT_DATA_GET_INFO.call(c, 0x0028_0398)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(RIDER_ACTIVATE_ENGINE_GP));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        SCRIPT_ENGINE_FIND_ANIM_BANK.call(c, 0x0028_03a8)?;
        c.sw(this.wrapping_add(0x1c), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(RIDER_ACTIVATE_ENGINE_GP));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        SCRIPT_ENGINE_FIND_SOUND_BANK.call(c, 0x0028_03b8)?;
        c.sw(this.wrapping_add(0x20), c.g[V0][0]);
        frame.restore(c, 16, 0x1a0);
        frame.restore(c, 17, 0x190);
        frame.restore(c, 18, 0x180);
        frame.restore(c, 19, 0x170);
        frame.restore(c, 20, 0x160);
        frame.restore(c, 21, 0x150);
        frame.restore(c, 22, 0x140);
        frame.restore(c, 23, 0x130);
        frame.restore(c, 30, 0x120);
        frame.restore_ra(c, 0x110);
        frame.restore_fpr(c, 20, 0x1b0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// `RiderScriptObj_Activate`: the object at DATA, its script object at
    /// DATA+0x1000, the params record at DATA+0x2000 (info +1 = 0: the
    /// target-pose default path), engine at DATA+0x3000 (empty bank lists).
    #[test]
    fn rider_script_obj_activate_matches() {
        check(
            ee::s07::f_0027f9f8__generated,
            rider_script_obj_activate,
            200,
            &|c, seed| {
                let mut r = Rng(0x5eed_1234 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x0027_f9f8;
                c.g[GP][0] = sx(GAME_GP);
                let this = DATA;
                let obj = DATA + 0x1000;
                let info = DATA + 0x2000;
                let engine = DATA + 0x3000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x8, obj);
                c.bus.write32(obj + 0x8, DATA + 0x5000); // script thing
                c.bus.write32(DATA + 0x5000 + 0x14, DATA + 0x4000); // its data
                c.bus.write32(DATA + 0x4000 + 0x1c, r.next_u32() & 0xff);
                c.bus.write32(obj + 0x20, info);
                c.bus.write32(GAME_GP.wrapping_sub(RIDER_ACTIVATE_ENGINE_GP), engine);
                c.bus.write32(
                    GAME_GP.wrapping_sub(RIDER_ACTIVATE_DEG2RAD_GP),
                    0.017453292f32.to_bits(),
                );
                // info: byte +1 selects the pose kind: 0 or out of range
                c.bus.write8(info + 1, if seed % 2 == 0 { 0 } else { 0x7f });
                for off in [0x8u32, 0xc, 0x10, 0x14, 0x18, 0x1c] {
                    c.bus.write32(info + off, r.f32(180.0));
                }
                // constants: identity matrix and axes (random identity row
                // variation to exercise the multiplies)
                for i in 0..16u32 {
                    let v = if seed % 3 == 0 {
                        if i % 5 == 0 { 1.0f32.to_bits() } else { 0 }
                    } else {
                        r.f32(2.0)
                    };
                    c.bus.write32(RIDER_ACTIVATE_IDENTITY + 4 * i, v);
                }
                let axes = [
                    (RIDER_ACTIVATE_X_AXIS, [1.0f32, 0.0, 0.0, 0.0]),
                    (RIDER_ACTIVATE_Y_AXIS, [0.0, 1.0, 0.0, 0.0]),
                    (RIDER_ACTIVATE_Z_AXIS, [0.0, 0.0, 1.0, 0.0]),
                ];
                for (a, v) in axes {
                    for (k, f) in v.iter().enumerate() {
                        let f = if seed % 4 == 1 && k < 3 { f * 0.6 } else { *f };
                        c.bus.write32(a + 4 * k as u32, f.to_bits());
                    }
                }
            },
        );
    }
}
