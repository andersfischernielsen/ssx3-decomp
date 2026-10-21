//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for GS packet builders, UI/menu and FE helpers, the render
//! side's emitters and modifiers, and small BE/audio/intersection utilities,
//! each with a differential test against the generated body.
//!
//! The replacements keep the generated body's structure: the same blocks in
//! the same order with the same cycle charges, the same guest registers left
//! for callers (the originals interleave the register file with their
//! branches), memory access through the [`RegMem`] accessors (`c.ldw(A0,
//! 0x10)` is the original's `lw a0, 0x10`) and calls through the function
//! table with the original's return-address check. VU0 code is issued as the
//! original's macro operations.

use crate::abi::r32;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0036_d1f0,
        emitter_set_desc_path_coeffs_scaled,
        "Emitter_SetDescPathCoeffsScaled",
    ),
    (0x0034_ac88, flag_read_params, "Flag_ReadParams"),
    (0x002d_aa78, c_board_spark_fx_reset, "cBoardSparkFX_Reset"),
    (
        0x0036_aa60,
        ps2_graphics_man_emit_depth_fog_clut_upload,
        "PS2GraphicsMan_EmitDepthFogClutUpload",
    ),
    (0x002c_b350, menu_sys_build_quad, "MenuSys_BuildQuad"),
    (0x0036_27a8, gs_pkt_finish, "GsPkt_Finish"),
    (0x0039_74b0, ui_anim_advance, "UIAnim_Advance"),
    (
        0x0013_1200,
        rider_board_press_sub_check_release,
        "RiderBoardPressSub_CheckRelease",
    ),
    (
        0x0032_fc08,
        coll_sphere_overlaps_aabb,
        "CollSphere_OverlapsAABB",
    ),
    (0x002d_64d8, avalanche_clamp_vector, "Avalanche_ClampVector"),
    (
        0x001c_7738,
        c_fe_popup_confirm_apply_background_scale,
        "cFEPopupConfirm_ApplyBackgroundScale",
    ),
    (
        0x0036_9a78,
        ps2_graphics_man_match_blended_matrix,
        "PS2GraphicsMan_MatchBlendedMatrix",
    ),
    (
        0x0028_7a10,
        ssx_audio_system_set_fade_gain,
        "SSXAudioSystem_SetFadeGain",
    ),
    (0x0034_1fe8, anim_object_v36, "AnimObject_v36"),
    (
        0x002a_62f0,
        crowd_audio_instance_remove_source,
        "CrowdAudio_InstanceRemoveSource",
    ),
    (0x0038_4e50, line_strip_finish, "LineStrip_Finish"),
    (
        0x0035_f410,
        tex_flip_modifier_update,
        "TexFlipModifier_Update",
    ),
    (
        0x0014_64d0,
        be_new_player_int_get_peak_flag_b,
        "BENewPlayerInt_GetPeakFlagB",
    ),
    (
        0x002a_7468,
        crowd_audio_push_play_params,
        "CrowdAudio_PushPlayParams",
    ),
    (
        0x0039_5d60,
        ps2_graphics_man_rotate_axis,
        "PS2GraphicsMan_RotateAxis",
    ),
    (
        0x0015_8f60,
        be_reward_int_grant_rival_rewards,
        "BERewardInt_GrantRivalRewards",
    ),
    (
        0x0013_5f70,
        rider_spin_sub_get_hud_dir_angle,
        "RiderSpinSub_GetHUDDirAngle",
    ),
    (
        0x0036_84f0,
        tex_man_emit_clut_upload,
        "TexMan_EmitClutUpload",
    ),
    (
        0x0012_fec8,
        rider_board_press_sub_flip_stance,
        "RiderBoardPressSub_FlipStance",
    ),
    (
        0x0032_8030,
        sphere_tree_collide_triangle2_sided,
        "SphereTree_CollideTriangle2Sided",
    ),(0x0035_73f8, magnet_modifier_update, "MagnetModifier_Update"),
];

// ---------------------------------------------------------------------------
// Memory accessors: the original's `lw rt, off(base)` and friends.

/// Register-relative guest memory, for the accessors below: the base is read
/// from the register file each time, as the compiled code does.
trait RegMem {
    fn pa(&self, base: usize, off: i32) -> u32;
    fn ldw(&mut self, base: usize, off: i32) -> u64;
    fn ldwu(&mut self, base: usize, off: i32) -> u64;
    fn ldh(&mut self, base: usize, off: i32) -> u64;
    fn ldhu(&mut self, base: usize, off: i32) -> u64;
    fn ldb(&mut self, base: usize, off: i32) -> u64;
    fn ldbu(&mut self, base: usize, off: i32) -> u64;
    fn ldd(&mut self, base: usize, off: i32) -> u64;
    fn stw(&mut self, base: usize, off: i32, v: u64);
    fn sth(&mut self, base: usize, off: i32, v: u64);
    fn stb(&mut self, base: usize, off: i32, v: u64);
    fn std(&mut self, base: usize, off: i32, v: u64);
}

impl RegMem for Cpu {
    #[inline(always)]
    fn pa(&self, base: usize, off: i32) -> u32 {
        r32(self, base).wrapping_add(off as u32)
    }
    #[inline(always)]
    fn ldw(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lw(self)
    }
    #[inline(always)]
    fn ldwu(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lwu(self)
    }
    #[inline(always)]
    fn ldh(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lh(self)
    }
    #[inline(always)]
    fn ldhu(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lhu(self)
    }
    #[inline(always)]
    fn ldb(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lb(self)
    }
    #[inline(always)]
    fn ldbu(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).lbu(self)
    }
    #[inline(always)]
    fn ldd(&mut self, base: usize, off: i32) -> u64 {
        let a = self.pa(base, off);
        Addr(a).ld(self)
    }
    #[inline(always)]
    fn stw(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sw(self, v)
    }
    #[inline(always)]
    fn sth(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sh(self, v)
    }
    #[inline(always)]
    fn stb(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sb(self, v)
    }
    #[inline(always)]
    fn std(&mut self, base: usize, off: i32, v: u64) {
        let a = self.pa(base, off);
        Addr(a).sd(self, v)
    }
}

// ---------------------------------------------------------------------------
// Functions.

/// `void Emitter_SetDescPathCoeffsScaled(sGmFrustum* self, const Vec4* a,
/// const Vec4* b, const Vec4* c, const Vec4* d)` (0x36d1f0): store each
/// direction divided by `self->scale` (`+0x4c`) at `+0x80`, `+0x90`, `+0xa0`,
/// and `a/scale - 1.5*(b+c+d)/scale` at `+0x70` (VU0 multiplies by the
/// reciprocal of the scale).
pub fn emitter_set_desc_path_coeffs_scaled(c: &mut Cpu) {
    c.cycles += 74;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(A2, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x10));
    c.g[AT][0] = 0x3fc00000;
    c.mtc1(1, 1);
    c.lq(2, c.pa(SP, 0x10));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.sq(2, c.pa(A0, 0x80));
    c.sq(2, c.pa(SP, 0));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(A3, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x10));
    c.lq(2, c.pa(SP, 0x10));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.sq(2, c.pa(A0, 0x90));
    c.sq(2, c.pa(SP, 0));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(T0, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x10));
    c.lq(2, c.pa(SP, 0x10));
    c.lwc1(0, c.pa(A0, 0x4c));
    c.sq(2, c.pa(A0, 0xa0));
    c.sq(2, c.pa(SP, 0));
    c.mfc1(3, 0);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, c.pa(A1, 0));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, c.pa(SP, 0x20));
    c.lq(2, c.pa(SP, 0x20));
    c.sq(2, c.pa(SP, 0x10));
    c.lqc2(3, c.pa(A0, 0x80));
    c.lqc2(4, c.pa(A0, 0x90));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(SP, 0x40));
    c.lqc2(3, c.pa(SP, 0x40));
    c.lqc2(4, c.pa(A0, 0xa0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(SP, 0x30));
    c.mfc1(3, 1);
    c.lqc2(4, c.pa(SP, 0x30));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(SP, 0x20));
    c.lqc2(3, c.pa(SP, 0x10));
    c.lqc2(4, c.pa(SP, 0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.pa(SP, 0x50));
    c.lq(2, c.pa(SP, 0x50));
    c.sq(2, c.pa(A0, 0x70));
    c.sq(2, c.pa(SP, 0));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = ra;
    return;
}

/// `sFrRailDesc* Flag_ReadParams(sFrRailDesc* out, const FlagSrc* src)`
/// (0x34ac88): convert a rail modifier's source (flag words, floats) into
/// the runtime descriptor: four flag bytes, p/q, r0/r1 and each `t[]` triple,
/// with the rates and the `a` components scaled by `1/framerate`
/// (`*(int*)(gp+0x2a74 + 0x10)`).
pub fn flag_read_params(c: &mut Cpu) {
    c.cycles += 83;
    c.g[A2][0] = c.ldw(A1, 0x4);
    c.g[V0][0] = c.g[A0][0];
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 3);
    c.g[A2][0] = (0u64 < c.g[A2][0]) as u64;
    c.stb(A0, 0, c.g[A2][0]);
    c.g[V1][0] = c.ldw(A1, 0x8);
    c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
    c.stb(A0, 0x1, c.g[V1][0]);
    c.g[A2][0] = c.ldw(A1, 0xc);
    c.g[A2][0] = (0u64 < c.g[A2][0]) as u64;
    c.stb(A0, 0x2, c.g[A2][0]);
    c.g[V1][0] = c.ldw(A1, 0x64);
    c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
    c.stb(A0, 0x3, c.g[V1][0]);
    c.lwc1(0, c.pa(A1, 0x40));
    c.lwc1(1, c.pa(A1, 0x44));
    c.lwc1(2, c.pa(A1, 0x48));
    c.swc1(0, c.pa(A0, 0x34));
    c.swc1(1, c.pa(A0, 0x38));
    c.swc1(2, c.pa(A0, 0x3c));
    c.g[V1][0] = c.ldw(GP, 0x2a74);
    c.lwc1(0, c.pa(A1, 0x4c));
    c.lwc1(1, c.pa(A1, 0x50));
    c.lwc1(2, c.pa(A1, 0x54));
    c.swc1(0, c.pa(A0, 0x40));
    c.swc1(2, c.pa(A0, 0x48));
    c.swc1(1, c.pa(A0, 0x44));
    c.lwc1(1, c.pa(A1, 0x58));
    c.swc1(1, c.pa(A0, 0x4c));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x5c));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x50));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x60));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x54));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x10));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x4));
    c.lwc1(0, c.pa(A1, 0x14));
    c.swc1(0, c.pa(A0, 0x14));
    c.lwc1(1, c.pa(A1, 0x18));
    c.swc1(1, c.pa(A0, 0x24));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x1c));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0x8));
    c.lwc1(0, c.pa(A1, 0x20));
    c.swc1(0, c.pa(A0, 0x18));
    c.lwc1(1, c.pa(A1, 0x24));
    c.swc1(1, c.pa(A0, 0x28));
    c.lwc1(0, c.pa(V1, 0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, c.pa(A1, 0x28));
    c.div_s(0, 3, 0);
    c.mul_s(1, 1, 0);
    c.swc1(1, c.pa(A0, 0xc));
    c.lwc1(0, c.pa(A1, 0x2c));
    c.swc1(0, c.pa(A0, 0x1c));
    c.lwc1(1, c.pa(A1, 0x30));
    c.swc1(1, c.pa(A0, 0x2c));
    c.lwc1(2, c.pa(V1, 0x10));
    c.cvt_s_w(2, 2);
    c.lwc1(0, c.pa(A1, 0x34));
    c.div_s(3, 3, 2);
    c.mul_s(0, 0, 3);
    c.swc1(0, c.pa(A0, 0x10));
    c.lwc1(1, c.pa(A1, 0x38));
    c.swc1(1, c.pa(A0, 0x20));
    c.lwc1(0, c.pa(A1, 0x3c));
    let ra = c.g[RA][0] as u32;
    c.swc1(0, c.pa(A0, 0x30));
    c.pc = ra;
    return;
}

/// `void cBoardSparkFX_Reset(SparkFx* self)` (0x2daa78): reset a board spark

/// effect: the default position (`D_004FF130`), three zeroed words and two
/// copies of the default GS descriptor (`D_00501420`) with the blend factor,
/// texture page and alpha patched (the second with the CLUT bit). The
/// texture handle comes from `D_004A289C + 0xfa8`.
pub fn c_board_spark_fx_reset(c: &mut Cpu) {
    c.cycles += 83;
    c.g[V0][0] = 0x500000;
    c.g[T3][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
    c.g[T1][0] = 0xffffffffffbf0000;
    c.lq(3, c.pa(V0, 0));
    c.g[T1][0] = c.g[T1][0] | 0xffff;
    c.stw(A0, 0x70, 0u64);
    c.g[T6][0] = 0x400000;
    c.sq(3, c.pa(A0, 0x10));
    c.g[T0][0] = 0xfffffffffe7f0000;
    c.stw(A0, 0x74, 0u64);
    c.g[T0][0] = c.g[T0][0] | 0xffff;
    c.stw(A0, 0x78, 0u64);
    c.g[A3][0] = 0xfffffffffff00000;
    c.g[A2][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1420));
    c.ldl(2, c.pa(A2, 0x7));
    c.ldr(2, c.pa(A2, 0));
    c.ldl(3, c.pa(A2, 0xf));
    c.ldr(3, c.pa(A2, 0x8));
    c.g[A1][0] = c.ldw(A2, 0x10);
    c.sdl(2, c.pa(A0, 0x83));
    c.sdr(2, c.pa(A0, 0x7c));
    c.sdl(3, c.pa(A0, 0x8b));
    c.sdr(3, c.pa(A0, 0x84));
    c.stw(A0, 0x8c, c.g[A1][0]);
    c.g[A3][0] = c.g[A3][0] | 0xfff;
    c.g[A2][0] = 0xffffffffffcf0000;
    c.g[T5][0] = 0xfffffffffffffffc;
    c.g[V0][0] = c.ldw(A0, 0x80);
    c.g[A2][0] = c.g[A2][0] | 0xffff;
    c.g[V1][0] = c.ldw(A0, 0x84);
    c.g[T2][0] = 0xfffffffffffffc1f;
    c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
    c.g[T4][0] = 0xffffffffffffff83;
    c.g[V0][0] = c.g[V0][0] | c.g[T6][0];
    c.g[V1][0] = c.g[V1][0] & c.g[T2][0];
    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
    c.g[A1][0] = c.ldw(GP, -0x854);
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.g[V1][0] = c.g[V1][0] | 0xe0;
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.stw(A0, 0x84, c.g[V1][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
    c.g[V0][0] = c.g[V0][0] | 0x2;
    c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
    c.g[V0][0] = c.g[V0][0] | 0x1c;
    c.stw(A0, 0x80, c.g[V0][0]);
    c.g[V1][0] = c.ldw(A1, 0xfa8);
    c.sth(A0, 0x8c, c.g[V1][0]);
    c.g[A1][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1420));
    c.ldl(15, c.pa(A1, 0x7));
    c.ldr(15, c.pa(A1, 0));
    c.ldl(2, c.pa(A1, 0xf));
    c.ldr(2, c.pa(A1, 0x8));
    c.g[V1][0] = c.ldw(A1, 0x10);
    c.sdl(15, c.pa(A0, 0x97));
    c.sdr(15, c.pa(A0, 0x90));
    c.sdl(2, c.pa(A0, 0x9f));
    c.sdr(2, c.pa(A0, 0x98));
    c.stw(A0, 0xa0, c.g[V1][0]);
    c.g[V0][0] = c.ldw(A0, 0x94);
    c.g[V1][0] = c.ldw(A0, 0x98);
    c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
    c.g[T3][0] = c.ldw(GP, -0x854);
    c.g[V0][0] = c.g[V0][0] | c.g[T6][0];
    c.g[V1][0] = c.g[V1][0] & c.g[T2][0];
    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
    c.g[V1][0] = c.g[V1][0] | 0xe0;
    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
    c.stw(A0, 0x98, c.g[V1][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.g[A1][0] = c.ldw(A0, 0x90);
    c.g[V0][0] = c.g[V0][0] | 0x9000;
    c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
    c.g[A1][0] = c.g[A1][0] | 0xc;
    c.g[V0][0] = c.g[V0][0] | 0x2;
    c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
    c.g[V0][0] = c.g[V0][0] | 0x1c;
    c.stw(A0, 0x94, c.g[V0][0]);
    c.g[V1][0] = c.ldw(T3, 0xfa8);
    c.stw(A0, 0x90, c.g[A1][0]);
    let ra = c.g[RA][0] as u32;
    c.sth(A0, 0xa0, c.g[V1][0]);
    c.pc = ra;
    return;
}

/// `void PS2GraphicsMan_EmitDepthFogClutUpload(void*, int x, int y, char***
/// pp)` (0x36aa60): append the 0xB0-byte depth/fog CLUT-upload packet (six
/// GIF registers writing one CLUT at (x, y)) to the stream at `*pp`, then
/// advance the cursor past it. Leaf.
pub fn ps2_graphics_man_emit_depth_fog_clut_upload(c: &mut Cpu) {
    c.cycles += 79;
    c.g[A0][0] = c.ldw(A3, 0);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.sq(16, c.pa(SP, 0));
    c.por(3, 0, 0);
    c.g[V1][0] = 0x10000000;
    c.g[V1][0] = c.g[V1][0] | 0x6;
    c.sq(3, c.pa(A0, 0));
    c.g[V0][0] = 0x8000;
    c.g[V0][0] = c.g[V0][0] << 33;
    c.g[A2][0] = c.g[A2][0] << 32;
    c.g[T0][0] = 0x10000000;
    c.g[T0][0] = c.g[T0][0] << 32;
    c.g[T0][0] = c.g[T0][0] | 0x4;
    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
    c.g[T7][0] = 0xe;
    c.g[V0][0] = c.ldw(A3, 0);
    c.g[A1][0] = c.g[A1][0] << 32;
    c.g[T1][0] = 0x50;
    c.g[T2][0] = 0x51;
    c.std(V0, 0x10, c.g[T0][0]);
    c.g[T3][0] = 0x10;
    c.g[T3][0] = c.g[T3][0] << 32;
    c.g[T3][0] = c.g[T3][0] | 0x10;
    c.g[V0][0] = 0x30000000;
    c.g[V0][0] = c.g[V0][0] | 0x40;
    c.g[T4][0] = 0x52;
    c.g[V1][0] = c.ldw(A3, 0);
    c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
    c.g[T5][0] = 0x53;
    c.por(4, 0, 0);
    c.g[A0][0] = 0x8000000;
    c.g[A0][0] = c.g[A0][0] << 32;
    c.g[A0][0] = c.g[A0][0] | 0x40;
    c.std(V1, 0x18, c.g[T7][0]);
    c.por(8, 0, 0);
    c.g[T0][0] = 0x10000000;
    c.g[T0][0] = c.g[T0][0] | 0x2;
    c.g[T6][0] = 0x10000000;
    c.g[T6][0] = c.g[T6][0] << 32;
    c.g[T6][0] = c.g[T6][0] | 0x1;
    c.g[S0][0] = 0x3f;
    c.g[V0][0] = c.ldw(A3, 0);
    c.std(V0, 0x20, c.g[A2][0]);
    c.g[V1][0] = c.ldw(A3, 0);
    c.std(V1, 0x28, c.g[T1][0]);
    c.g[V0][0] = c.ldw(A3, 0);
    c.std(V0, 0x30, 0u64);
    c.g[V1][0] = c.ldw(A3, 0);
    c.std(V1, 0x38, c.g[T2][0]);
    c.g[V0][0] = c.ldw(A3, 0);
    c.std(V0, 0x40, c.g[T3][0]);
    c.g[V1][0] = c.ldw(A3, 0);
    c.std(V1, 0x48, c.g[T4][0]);
    c.g[V0][0] = c.ldw(A3, 0);
    c.std(V0, 0x50, 0u64);
    c.g[V1][0] = c.ldw(A3, 0);
    c.std(V1, 0x58, c.g[T5][0]);
    c.g[V0][0] = c.ldw(A3, 0);
    c.sq(4, c.pa(V0, 0x60));
    c.g[V1][0] = c.ldw(A3, 0);
    c.std(V1, 0x70, c.g[A1][0]);
    c.g[V0][0] = c.ldw(A3, 0);
    c.std(V0, 0x78, 0u64);
    c.g[V1][0] = c.ldw(A3, 0);
    c.sq(8, c.pa(V1, 0x80));
    c.g[V0][0] = c.ldw(A3, 0);
    c.std(V0, 0x90, c.g[T6][0]);
    c.g[V1][0] = c.ldw(A3, 0);
    c.std(V1, 0x98, c.g[T7][0]);
    c.g[V0][0] = c.ldw(A3, 0);
    c.std(V0, 0xa0, 0u64);
    c.g[V1][0] = c.ldw(A3, 0);
    c.std(V1, 0xa8, c.g[S0][0]);
    c.lq(16, c.pa(SP, 0));
    c.g[V0][0] = c.ldw(A3, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
    c.stw(A3, 0, c.g[V0][0]);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
    return;
}

/// `void MenuSys_BuildQuad(sVtx* v, const float* r, const float* t)`
/// (0x2cb350): fill a menu quad's vertices from the rectangle (`r` = x, y,
/// w, h) and the transform (`t` = x, y, z, w), then hand it to the draw
/// module's submit routine (slot at `*(D_004A289C+0x10d8) + 0x23c`, `this`
/// adjusted by the slot's `lh` delta).
pub fn menu_sys_build_quad(c: &mut Cpu) {
    c.cycles += 82;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[V1][0] = c.g[A1][0];
    c.std(SP, 0x10, c.g[RA][0]);
    c.g[T0][0] = c.g[A2][0];
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 3);
    c.g[A2][0] = c.g[A0][0];
    c.lwc1(0, c.pa(V1, 0));
    c.g[A1][0] = 0x4;
    c.lwc1(1, c.pa(V1, 0x4));
    c.g[A3][0] = 0u64;
    c.swc1(3, c.pa(SP, 0xc));
    c.swc1(0, c.pa(SP, 0));
    c.stw(SP, 0x8, 0u64);
    c.swc1(1, c.pa(SP, 0x4));
    c.lq(2, c.pa(SP, 0));
    c.stw(SP, 0x8, 0u64);
    c.sq(2, c.pa(A0, 0x20));
    c.swc1(3, c.pa(SP, 0xc));
    c.lwc1(0, c.pa(T0, 0));
    c.swc1(0, c.pa(A0, 0));
    c.lwc1(1, c.pa(T0, 0x4));
    c.swc1(1, c.pa(A0, 0x4));
    c.lwc1(1, c.pa(V1, 0x8));
    c.lwc1(0, c.pa(V1, 0));
    c.lwc1(2, c.pa(V1, 0x4));
    c.add_s(0, 0, 1);
    c.swc1(2, c.pa(SP, 0x4));
    c.swc1(0, c.pa(SP, 0));
    c.lq(2, c.pa(SP, 0));
    c.stw(SP, 0x8, 0u64);
    c.sq(2, c.pa(A0, 0x50));
    c.swc1(3, c.pa(SP, 0xc));
    c.lwc1(1, c.pa(T0, 0x8));
    c.lwc1(0, c.pa(T0, 0));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x30));
    c.lwc1(1, c.pa(T0, 0x4));
    c.swc1(1, c.pa(A0, 0x34));
    c.lwc1(1, c.pa(V1, 0xc));
    c.lwc1(0, c.pa(V1, 0x4));
    c.lwc1(2, c.pa(V1, 0));
    c.add_s(0, 0, 1);
    c.swc1(2, c.pa(SP, 0));
    c.swc1(0, c.pa(SP, 0x4));
    c.lq(2, c.pa(SP, 0));
    c.sq(2, c.pa(A0, 0x80));
    c.lwc1(0, c.pa(T0, 0));
    c.swc1(3, c.pa(SP, 0xc));
    c.swc1(0, c.pa(A0, 0x60));
    c.stw(SP, 0x8, 0u64);
    c.g[T1][0] = c.ldw(GP, -0x854);
    c.lwc1(1, c.pa(T0, 0xc));
    c.lwc1(0, c.pa(T0, 0x4));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x64));
    c.lwc1(3, c.pa(V1, 0xc));
    c.lwc1(2, c.pa(V1, 0x8));
    c.lwc1(1, c.pa(V1, 0));
    c.lwc1(0, c.pa(V1, 0x4));
    c.add_s(1, 1, 2);
    c.add_s(0, 0, 3);
    c.swc1(1, c.pa(SP, 0));
    c.swc1(0, c.pa(SP, 0x4));
    c.lq(2, c.pa(SP, 0));
    c.sq(2, c.pa(A0, 0xb0));
    c.lwc1(1, c.pa(T0, 0x8));
    c.lwc1(0, c.pa(T0, 0));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x90));
    c.lwc1(1, c.pa(T0, 0xc));
    c.lwc1(0, c.pa(T0, 0x4));
    c.add_s(0, 0, 1);
    c.swc1(0, c.pa(A0, 0x94));
    c.g[V0][0] = c.ldw(T1, 0x10d8);
    c.g[A0][0] = c.ldh(V0, 0x238);
    c.g[V1][0] = c.ldw(V0, 0x23c);
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x002cb48c);
    c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.pc = t;
    call_indirect(c, t);
    if c.pc != 0x002cb48c {
        return;
    }
    c.g[RA][0] = c.ldd(SP, 0x10);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = ra;
    return;
}

/// `void GsPkt_Finish(GsPktWriter* w, void** pp, int mode)` (0x3627a8):
/// close the writer's current packet block. An empty writer does nothing.
/// Mode 1 emits the opening block and leaves the old cursor on `*pp`; modes
/// 0 and 2 emit a block at the static buffer (`D_00505A80` for mode 0) plus
/// the two-quadword DMA kick on `*pp`; any other mode is a no-op.
pub fn gs_pkt_finish(c: &mut Cpu) {
    'b_3627d8: {
        c.cycles += 4;
        c.g[T1][0] = c.g[A0][0];
        c.g[V1][0] = c.ldw(T1, 0);
        let t = c.g[V1][0] == 0u64;
        c.g[T2][0] = c.g[A1][0];
        if t {
            break 'b_3627d8;
        } else {
            'b_362938: {
                'b_3628e4: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    if c.g[A2][0] == c.g[V0][0] {
                        c.g[V0][0] = c.ldw(T1, 0x4);
                        c.cycles += 20;
                        c.g[V1][0] = 0x10000000;
                        c.g[A1][0] = 0x50000000;
                        c.g[A2][0] = 0x8000;
                        c.g[A2][0] = c.g[A2][0] << 45;
                        c.stw(T2, 0, c.g[V0][0]);
                        c.g[A3][0] = 0xe;
                        c.g[V0][0] = c.ldw(T1, 0);
                        c.g[A0][0] = c.ldw(T1, 0x8);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        c.stw(T1, 0x4, c.g[A0][0]);
                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                        c.std(A0, 0, c.g[V0][0]);
                        c.g[V1][0] = c.ldw(T1, 0x4);
                        c.std(V1, 0x8, 0u64);
                        c.g[A0][0] = c.ldw(T1, 0x4);
                        c.std(A0, 0x10, 0u64);
                        c.g[V0][0] = c.ldw(T1, 0);
                        c.g[A0][0] = c.ldw(T1, 0x4);
                        let t = true;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_362938;
                        } else {
                            break 'b_3628e4;
                        }
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A2][0] as i64) < 2i64) as u64;
                        if c.g[V0][0] == 0u64 {
                            c.g[V0][0] = 0x2;
                            c.cycles += 2;
                            if c.g[A2][0] == c.g[V0][0] {
                                c.g[V0][0] = c.ldw(T1, 0x4);
                                break 'b_3628e4;
                            } else {
                                c.cycles += 2;
                                let ra = c.g[RA][0] as u32;
                                c.pc = ra;
                                return;
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[A2][0] == 0u64;
                            c.g[A0][0] = 0x500000;
                            if t {
                                c.cycles += 41;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5a80));
                                c.g[V0][0] = 0x60000000;
                                c.stw(T1, 0x4, c.g[A1][0]);
                                c.g[V0][0] = c.g[V1][0] | c.g[V0][0];
                                c.std(A0, 0x5a80, c.g[V0][0]);
                                c.g[A1][0] = c.g[A1][0] << 32;
                                c.g[V0][0] = 0x50000000;
                                c.g[A2][0] = 0x50000000;
                                c.g[V1][0] = c.ldw(T1, 0x4);
                                c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                                c.g[A3][0] = 0x8000;
                                c.g[A3][0] = c.g[A3][0] << 45;
                                c.g[T0][0] = 0xe;
                                c.std(V1, 0x8, 0u64);
                                c.g[V1][0] = c.ldw(T1, 0x4);
                                c.std(V1, 0x10, 0u64);
                                c.g[V0][0] = c.ldw(T1, 0);
                                c.g[A0][0] = c.ldw(T1, 0x4);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                                c.g[V0][0] = c.g[V0][0] << 32;
                                c.std(A0, 0x18, c.g[V0][0]);
                                c.g[V1][0] = c.ldw(T1, 0);
                                c.g[V0][0] = c.ldw(T1, 0x4);
                                c.g[V1][0] = c.g[V1][0] | 0x8000;
                                c.g[V1][0] = c.g[V1][0] | c.g[A3][0];
                                c.std(V0, 0x20, c.g[V1][0]);
                                c.g[A0][0] = c.ldw(T1, 0x4);
                                c.std(A0, 0x28, c.g[T0][0]);
                                c.g[V0][0] = c.ldw(T1, 0x4);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                                c.stw(T1, 0x4, c.g[V0][0]);
                                c.g[V1][0] = c.ldw(T2, 0);
                                c.std(V1, 0, c.g[A1][0]);
                                c.g[A0][0] = c.ldw(T2, 0);
                                c.std(A0, 0x8, 0u64);
                                c.g[V0][0] = c.ldw(T2, 0);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                                let ra = c.g[RA][0] as u32;
                                c.stw(T2, 0, c.g[V0][0]);
                                c.pc = ra;
                                return;
                            } else {
                                break 'b_3627d8;
                            }
                        }
                    }
                }
                c.cycles += 21;
                c.g[V1][0] = 0x10000000;
                c.g[A1][0] = 0x6c000000;
                c.g[A2][0] = 0x8000;
                c.g[A2][0] = c.g[A2][0] << 45;
                c.stw(T2, 0, c.g[V0][0]);
                c.g[A1][0] = c.g[A1][0] | 0x3f2;
                c.g[A3][0] = 0xe;
                c.g[V0][0] = c.ldw(T1, 0);
                c.g[A0][0] = c.ldw(T1, 0x8);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                c.stw(T1, 0x4, c.g[A0][0]);
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.std(A0, 0, c.g[V0][0]);
                c.g[V1][0] = c.ldw(T1, 0x4);
                c.std(V1, 0x8, 0u64);
                c.g[A0][0] = c.ldw(T1, 0x4);
                c.std(A0, 0x10, 0u64);
                c.g[V0][0] = c.ldw(T1, 0);
                c.g[A0][0] = c.ldw(T1, 0x4);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                break 'b_362938;
            }
            c.cycles += 15;
            c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] << 32;
            c.std(A0, 0x18, c.g[V0][0]);
            c.g[V1][0] = c.ldw(T1, 0);
            c.g[V0][0] = c.ldw(T1, 0x4);
            c.g[V1][0] = c.g[V1][0] | 0x8000;
            c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
            c.std(V0, 0x20, c.g[V1][0]);
            c.g[A0][0] = c.ldw(T1, 0x4);
            c.std(A0, 0x28, c.g[A3][0]);
            c.g[V0][0] = c.ldw(T1, 0x4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
            c.stw(T1, 0x4, c.g[V0][0]);
            let ra = c.g[RA][0] as u32;
            c.pc = ra;
            return;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
}

/// `void UIAnim_Advance(UIAnim* self, unsigned flags)` (0x3974b0): step the
/// animation by `flags & 7` (mode 1 forward, 2 back, 3 bounce using
/// `self->dir`), clamping to `data->count`; bit 8 wraps instead of clamping.
/// The frame is at `+0xa`, the direction at `+0xe`, the data at `+0`.
pub fn ui_anim_advance(c: &mut Cpu) {
    'b_3974e0: {
        c.cycles += 3;
        c.g[A2][0] = c.ldw(A0, 0);
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = c.g[A1][0] & 0xff;
        if t {
            break 'b_3974e0;
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.g[A1][0] & 0x7;
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0x2;
            if t {
                break 'b_3974e0;
            } else {
                'b_39752c: {
                    'b_397520: {
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                        if t {
                            c.cycles += 5;
                            c.g[V0][0] = c.ldhu(A0, 0xa);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
                            let t = (c.g[V1][0] as i64) >= 0;
                            c.sth(A0, 0xa, c.g[V0][0]);
                            if t {
                                break 'b_3974e0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[A1][0] & 0x8;
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_39752c;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.ldhu(A2, 0x2);
                                    break 'b_397520;
                                }
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                c.cycles += 3;
                                c.g[V0][0] = 0x3;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.ldb(A0, 0xe);
                                    'b_397578: {
                                        'b_397574: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V0][0] = c.ldhu(A0, 0xa);
                                            if t {
                                                break 'b_397574;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_397578;
                                                } else {
                                                    break 'b_397574;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        break 'b_397578;
                                    }
                                    'b_3975b4: {
                                        'b_3975b0: {
                                            c.cycles += 4;
                                            c.sth(A0, 0xa, c.g[V0][0]);
                                            c.g[V0][0] = c.ldb(A0, 0xe);
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A2][0] = c.ldbu(A0, 0xe);
                                            if t {
                                                break 'b_3975b0;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A1][0] = c.ldw(A0, 0);
                                                c.g[V0][0] = c.ldh(A0, 0xa);
                                                c.g[V1][0] = c.ldhu(A1, 0x2);
                                                c.g[V0][0] = ((c.g[V0][0] as i64)
                                                    < (c.g[V1][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                                                if t {
                                                    break 'b_3975b4;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V1][0] = 0x1;
                                                    c.stb(A0, 0xe, c.g[V1][0]);
                                                    let t = true;
                                                    c.g[V0][0] = c.ldhu(A1, 0x2);
                                                    if t {
                                                        break 'b_397520;
                                                    } else {
                                                        break 'b_3975b0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                                        break 'b_3975b4;
                                    }
                                    'b_3975d8: {
                                        c.cycles += 4;
                                        c.g[V1][0] = 0x1;
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                        let t = c.g[V0][0] != c.g[V1][0];

                                        if t {
                                            break 'b_3975d8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.ldh(A0, 0xa);
                                            let t = (c.g[V0][0] as i64) >= 0;

                                            if t {
                                                break 'b_3975d8;
                                            } else {
                                                c.cycles += 2;
                                                c.sth(A0, 0xa, 0u64);
                                                c.stb(A0, 0xe, 0u64);
                                                break 'b_3975d8;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let ra = c.g[RA][0] as u32;
                                    c.pc = ra;
                                    return;
                                } else {
                                    c.cycles += 2;
                                    let ra = c.g[RA][0] as u32;
                                    c.pc = ra;
                                    return;
                                }
                            } else {
                                c.cycles += 2;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.ldhu(A0, 0xa);
                                    c.cycles += 8;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.sth(A0, 0xa, c.g[V0][0]);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                                    c.g[V1][0] = c.ldhu(A2, 0x2);
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.g[A1][0] & 0x8;
                                    if t {
                                        break 'b_3974e0;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                        if t {
                                            break 'b_39752c;
                                        } else {
                                            c.cycles += 2;
                                            let ra = c.g[RA][0] as u32;
                                            c.sth(A0, 0xa, c.g[V0][0]);
                                            c.pc = ra;
                                            return;
                                        }
                                    }
                                } else {
                                    break 'b_3974e0;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let ra = c.g[RA][0] as u32;
                    c.sth(A0, 0xa, c.g[V0][0]);
                    c.pc = ra;
                    return;
                }
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.sth(A0, 0xa, 0u64);
                c.pc = ra;
                return;
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
}

/// `int RiderBoardPressSub_CheckRelease(Ai* self, const AiStick* stick)`
/// (0x131200): hold-to-charge release check: accumulate the press timer
/// while the stick is neutral (|y| < 0.5), scale it by the rider's rate, and
/// report a release (return 1) when the timer, the rider's lean state or the
/// current charge says so.
pub fn rider_board_press_sub_check_release(c: &mut Cpu) {
    'b_131280: {
        'b_13127c: {
            c.cycles += 16;
            c.g[V1][0] = c.ldw(A1, 0x4);
            c.lwc1(3, c.pa(GP, -0x7614));
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 26);
            c.mtc1(0, 2);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 20);
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 26) as u32);
            c.mtc1(3, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(4, 1, 3);
            c.c_eq_s(4, 2);
            let t = !c.fcc();
            c.mul_s(0, 0, 3);
            if t {
                break 'b_13127c;
            } else {
                c.cycles += 7;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 1);
                c.abs_s(0, 0);
                c.c_lt_s(0, 1);
                if !c.fcc() {
                    c.stw(A0, 0xc, 0u64);
                    break 'b_131280;
                } else {
                    c.cycles += 8;
                    c.g[V0][0] = c.ldw(A0, 0x14);
                    c.lwc1(2, c.pa(GP, -0x7610));
                    c.lwc1(0, c.pa(V0, 0x300));
                    c.lwc1(1, c.pa(A0, 0xc));
                    c.mul_s(0, 0, 2);
                    c.add_s(1, 1, 0);
                    let t = true;
                    c.swc1(1, c.pa(A0, 0xc));
                    if t {
                        break 'b_131280;
                    } else {
                        break 'b_13127c;
                    }
                }
            }
        }
        c.cycles += 1;
        c.stw(A0, 0xc, 0u64);
        break 'b_131280;
    }
    'b_131320: {
        'b_1312e4: {
            'b_1312e0: {
                c.cycles += 7;
                c.lwc1(1, c.pa(A0, 0xc));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.c_lt_s(0, 1);
                let t = c.fcc();
                c.g[V1][0] = c.ldw(A0, 0x14);
                if t {
                    break 'b_1312e0;
                } else {
                    'b_1312c4: {
                        c.cycles += 4;
                        c.g[A1][0] = c.ldw(V1, 0x330);
                        c.g[V0][0] = 0x2;
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_1312c4;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, c.pa(GP, -0x760c));
                            c.c_lt_s(0, 4);
                            if c.fcc() {
                                c.lwc1(0, c.pa(V1, 0x300));
                                break 'b_1312e4;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0x1;
                                break 'b_1312c4;
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[A1][0] != c.g[V0][0] {
                        c.g[V1][0] = c.ldw(A0, 0x14);
                        break 'b_131320;
                    } else {
                        c.cycles += 5;
                        c.lwc1(0, c.pa(GP, -0x7608));
                        c.c_lt_s(4, 0);
                        if !c.fcc() {
                            c.g[V1][0] = c.ldw(A0, 0x14);
                            break 'b_131320;
                        } else {
                            break 'b_1312e0;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lwc1(0, c.pa(V1, 0x300));
            break 'b_1312e4;
        }
        c.cycles += 13;
        c.lwc1(1, c.pa(GP, -0x7604));
        c.add_s(0, 0, 0);
        c.stw(V1, 0x27c, 0u64);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 2);
        c.mul_s(0, 0, 1);
        c.swc1(0, c.pa(V1, 0x278));
        c.g[V0][0] = c.ldw(A0, 0x14);
        c.lwc1(0, c.pa(V0, 0x274));
        c.c_lt_s(0, 2);
        if !c.fcc() {
            c.g[V1][0] = c.ldw(A0, 0x14);
            break 'b_131320;
        } else {
            c.cycles += 2;
            let ra = c.g[RA][0] as u32;
            c.g[V0][0] = 0x1;
            c.pc = ra;
            return;
        }
    }
    'b_13133c: {
        c.cycles += 6;
        c.mtc1(0, 1);
        c.lwc1(0, c.pa(V1, 0x274));
        c.c_eq_s(0, 1);
        let t = c.fcc();
        c.g[V0][0] = 0x1;
        if t {
            break 'b_13133c;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_13133c;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
}

/// `bool CollSphere_OverlapsAABB(const CollSphere* s, const Vec4* mn,
/// const Vec4* mx)` (0x32fc08): the sphere's capsule (`+0x60` centre,
/// `+0x70` radius) grown by the radius on every axis is tested against the
/// box; true when they overlap.
pub fn coll_sphere_overlaps_aabb(c: &mut Cpu) {
    'b_32fd38: {
        'b_32fc78: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.stw(SP, 0xc, 0u64);
            c.lwc1(0, c.pa(A0, 0x70));
            c.swc1(0, c.pa(SP, 0x8));
            c.swc1(0, c.pa(SP, 0));
            c.swc1(0, c.pa(SP, 0x4));
            c.lqc2(3, c.pa(A0, 0x60));
            c.lqc2(4, c.pa(SP, 0));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, c.pa(SP, 0x20));
            c.lq(2, c.pa(SP, 0x20));
            c.sq(2, c.pa(SP, 0x10));
            c.lqc2(3, c.pa(A0, 0x60));
            c.lqc2(4, c.pa(SP, 0));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, c.pa(SP, 0x30));
            c.lwc1(1, c.pa(A1, 0));
            c.lwc1(2, c.pa(SP, 0x10));
            c.lq(2, c.pa(SP, 0x30));
            c.c_lt_s(2, 1);
            let t = !c.fcc();
            c.sq(2, c.pa(SP, 0x20));
            if t {
                break 'b_32fc78;
            } else {
                c.cycles += 5;
                c.lwc1(0, c.pa(SP, 0x20));
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fc78;
                }
            }
        }
        'b_32fc9c: {
            c.cycles += 5;
            c.lwc1(1, c.pa(A2, 0));
            c.c_lt_s(1, 2);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x20));
            if t {
                break 'b_32fc9c;
            } else {
                c.cycles += 4;
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fc9c;
                }
            }
        }
        'b_32fcc4: {
            c.cycles += 6;
            c.lwc1(1, c.pa(A1, 0x4));
            c.lwc1(2, c.pa(SP, 0x14));
            c.c_lt_s(2, 1);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x24));
            if t {
                break 'b_32fcc4;
            } else {
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fcc4;
                }
            }
        }
        'b_32fce8: {
            c.cycles += 5;
            c.lwc1(1, c.pa(A2, 0x4));
            c.c_lt_s(1, 2);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x24));
            if t {
                break 'b_32fce8;
            } else {
                c.cycles += 4;
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fce8;
                }
            }
        }
        'b_32fd10: {
            c.cycles += 6;
            c.lwc1(1, c.pa(A1, 0x8));
            c.lwc1(2, c.pa(SP, 0x18));
            c.c_lt_s(2, 1);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x28));
            if t {
                break 'b_32fd10;
            } else {
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.g[V0][0] = 0u64;
                    break 'b_32fd38;
                } else {
                    break 'b_32fd10;
                }
            }
        }
        'b_32fd34: {
            c.cycles += 5;
            c.lwc1(1, c.pa(A2, 0x8));
            c.c_lt_s(1, 2);
            let t = !c.fcc();
            c.lwc1(0, c.pa(SP, 0x28));
            if t {
                break 'b_32fd34;
            } else {
                c.cycles += 4;
                c.c_lt_s(1, 0);
                let t = c.fcc();
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_32fd38;
                } else {
                    break 'b_32fd34;
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x1;
        break 'b_32fd38;
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    return;
}

/// `void Avalanche_ClampVector(void* self, Vec4* v)` (0x2d64d8): clamp the
/// vector to the avalanche speed limit (`D_004A3A94`); when any component is
/// a PS2 inf/NaN, replace the vector with (10, 10, 10, 0).
pub fn avalanche_clamp_vector(c: &mut Cpu) {
    'b_2d6568: {
        c.cycles += 15;
        c.lwc1(0, c.pa(GP, 0x9a4));
        c.lqc2(3, c.pa(A1, 0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 1);
        c.c_lt_s(0, 1);
        let t = !c.fcc();
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        if t {
            break 'b_2d6568;
        } else {
            c.cycles += 21;
            c.lqc2(3, c.pa(A1, 0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, c.pa(SP, 0x20));
            c.lq(2, c.pa(SP, 0x20));
            c.lwc1(0, c.pa(GP, 0x9a4));
            c.sq(2, c.pa(SP, 0x10));
            c.mfc1(3, 0);
            c.lqc2(4, c.pa(SP, 0x10));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, c.pa(SP, 0x20));
            c.lq(2, c.pa(SP, 0x20));
            c.sq(2, c.pa(A1, 0));
            c.sq(2, c.pa(SP, 0));
            break 'b_2d6568;
        }
    }
    'b_2d65a4: {
        'b_2d65a0: {
            c.cycles += 5;
            c.g[V1][0] = c.ldw(A1, 0);
            c.g[A2][0] = 0u64;
            c.g[V0][0] = 0x7f800000;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = 0u64;
            if t {
                break 'b_2d65a0;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x7fc00000;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[A0][0] = 0x1;
                    break 'b_2d65a4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffff800000;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[A0][0] = 0x1;
                        break 'b_2d65a4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xffffffffffc00000;
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_2d65a4;
                        } else {
                            break 'b_2d65a0;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[A0][0] = 0x1;
        break 'b_2d65a4;
    }
    'b_2d666c: {
        c.cycles += 2;
        if c.g[A0][0] != 0u64 {
            c.g[A2][0] = 0x1;
            break 'b_2d666c;
        } else {
            'b_2d65e4: {
                'b_2d65e0: {
                    c.cycles += 4;
                    c.g[V1][0] = c.ldw(A1, 0x4);
                    c.g[V0][0] = 0x7f800000;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0u64;
                    if t {
                        break 'b_2d65e0;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x7fc00000;
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[A0][0] = 0x1;
                            break 'b_2d65e4;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0xffffffffff800000;
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[A0][0] = 0x1;
                                break 'b_2d65e4;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0xffffffffffc00000;
                                let t = c.g[V1][0] != c.g[V0][0];

                                if t {
                                    break 'b_2d65e4;
                                } else {
                                    break 'b_2d65e0;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = 0x1;
                break 'b_2d65e4;
            }
            c.cycles += 2;
            if c.g[A0][0] != 0u64 {
                c.g[A2][0] = 0x1;
                break 'b_2d666c;
            } else {
                'b_2d6624: {
                    'b_2d6620: {
                        c.cycles += 4;
                        c.g[V1][0] = c.ldw(A1, 0x8);
                        c.g[V0][0] = 0x7f800000;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[A0][0] = 0u64;
                        if t {
                            break 'b_2d6620;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x7fc00000;
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[A0][0] = 0x1;
                                break 'b_2d6624;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0xffffffffff800000;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[A0][0] = 0x1;
                                    break 'b_2d6624;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0xffffffffffc00000;
                                    let t = c.g[V1][0] != c.g[V0][0];

                                    if t {
                                        break 'b_2d6624;
                                    } else {
                                        break 'b_2d6620;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = 0x1;
                    break 'b_2d6624;
                }
                c.cycles += 2;
                if c.g[A0][0] != 0u64 {
                    c.g[A2][0] = 0x1;
                    break 'b_2d666c;
                } else {
                    'b_2d6664: {
                        'b_2d6660: {
                            c.cycles += 4;
                            c.g[V1][0] = c.ldw(A1, 0xc);
                            c.g[V0][0] = 0x7f800000;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[A0][0] = 0u64;
                            if t {
                                break 'b_2d6660;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x7fc00000;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[A0][0] = 0x1;
                                    break 'b_2d6664;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0xffffffffff800000;
                                    if c.g[V1][0] == c.g[V0][0] {
                                        c.g[A0][0] = 0x1;
                                        break 'b_2d6664;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0xffffffffffc00000;
                                        let t = c.g[V1][0] != c.g[V0][0];

                                        if t {
                                            break 'b_2d6664;
                                        } else {
                                            break 'b_2d6660;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = 0x1;
                        break 'b_2d6664;
                    }
                    c.cycles += 2;
                    if c.g[A0][0] != 0u64 {
                        c.g[A2][0] = 0x1;
                        break 'b_2d666c;
                    } else {
                        break 'b_2d666c;
                    }
                }
            }
        }
    }
    'b_2d6694: {
        c.cycles += 2;
        let t = c.g[A2][0] == 0u64;

        if t {
            break 'b_2d6694;
        } else {
            c.cycles += 8;
            c.g[AT][0] = 0x41200000;
            c.mtc1(1, 0);
            c.stw(SP, 0xc, 0u64);
            c.swc1(0, c.pa(SP, 0x8));
            c.swc1(0, c.pa(SP, 0));
            c.swc1(0, c.pa(SP, 0x4));
            c.lq(2, c.pa(SP, 0));
            c.sq(2, c.pa(A1, 0));
            break 'b_2d6694;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = ra;
    return;
}

/// `void cFEPopupConfirm_ApplyBackgroundScale(self, const sColor* c)`
/// (0x1c7738): copy a colour to the popup's background objects, chosen by
/// the popup style at `+0x31c`: two objects for style 0, five for style 1,
/// one for style 2. Each object gets the colour at `+0x50`.
pub fn c_fe_popup_confirm_apply_background_scale(c: &mut Cpu) {
    'b_1c78b4: {
        'b_1c7894: {
            'b_1c77d8: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[V1][0] = c.ldw(A0, 0x31c);
                c.ldl(6, c.pa(A1, 0x7));
                c.ldr(6, c.pa(A1, 0));
                c.g[A3][0] = c.ldw(A1, 0x8);
                c.sdl(6, c.pa(SP, 0x7));
                c.sdr(6, c.pa(SP, 0));
                c.g[V0][0] = 0x1;
                let t = c.g[V1][0] == c.g[V0][0];
                c.stw(SP, 0x8, c.g[A3][0]);
                if t {
                    break 'b_1c77d8;
                } else {
                    'b_1c778c: {
                        'b_1c777c: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = 0x2;
                                break 'b_1c777c;
                            } else {
                                c.cycles += 2;
                                if c.g[V1][0] == 0u64 {
                                    c.g[V0][0] = c.ldw(A0, 0x48);
                                    break 'b_1c778c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_1c78b4;
                                    } else {
                                        break 'b_1c777c;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[A0][0] = c.ldw(A0, 0x68);
                            break 'b_1c7894;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_1c78b4;
                            } else {
                                break 'b_1c778c;
                            }
                        }
                    }
                    'b_1c77b0: {
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[A0][0] = c.ldw(A0, 0x4c);
                            break 'b_1c77b0;
                        } else {
                            c.cycles += 7;
                            c.ldl(3, c.pa(SP, 0x7));
                            c.ldr(3, c.pa(SP, 0));
                            c.g[A1][0] = c.ldw(SP, 0x8);
                            c.sdl(3, c.pa(V0, 0x57));
                            c.sdr(3, c.pa(V0, 0x50));
                            c.stw(V0, 0x58, c.g[A1][0]);
                            c.g[A0][0] = c.ldw(A0, 0x4c);
                            break 'b_1c77b0;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;

                    if t {
                        break 'b_1c78b4;
                    } else {
                        c.cycles += 8;
                        c.ldl(8, c.pa(SP, 0x7));
                        c.ldr(8, c.pa(SP, 0));
                        c.g[T1][0] = c.ldw(SP, 0x8);
                        c.sdl(8, c.pa(A0, 0x57));
                        c.sdr(8, c.pa(A0, 0x50));
                        c.stw(A0, 0x58, c.g[T1][0]);
                        let t = true;

                        if t {
                            break 'b_1c78b4;
                        } else {
                            break 'b_1c77d8;
                        }
                    }
                }
            }
            'b_1c7800: {
                c.cycles += 3;
                c.g[V0][0] = c.ldw(A0, 0x58);
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = c.ldw(A0, 0x5c);
                    break 'b_1c7800;
                } else {
                    c.cycles += 7;
                    c.ldl(5, c.pa(SP, 0x7));
                    c.ldr(5, c.pa(SP, 0));
                    c.g[A2][0] = c.ldw(SP, 0x8);
                    c.sdl(5, c.pa(V0, 0x57));
                    c.sdr(5, c.pa(V0, 0x50));
                    c.stw(V0, 0x58, c.g[A2][0]);
                    c.g[V0][0] = c.ldw(A0, 0x5c);
                    break 'b_1c7800;
                }
            }
            'b_1c7824: {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = c.ldw(A0, 0x50);
                    break 'b_1c7824;
                } else {
                    c.cycles += 7;
                    c.ldl(9, c.pa(SP, 0x7));
                    c.ldr(9, c.pa(SP, 0));
                    c.g[V1][0] = c.ldw(SP, 0x8);
                    c.sdl(9, c.pa(V0, 0x57));
                    c.sdr(9, c.pa(V0, 0x50));
                    c.stw(V0, 0x58, c.g[V1][0]);
                    c.g[V0][0] = c.ldw(A0, 0x50);
                    break 'b_1c7824;
                }
            }
            'b_1c7848: {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = c.ldw(A0, 0x54);
                    break 'b_1c7848;
                } else {
                    c.cycles += 7;
                    c.ldl(7, c.pa(SP, 0x7));
                    c.ldr(7, c.pa(SP, 0));
                    c.g[T0][0] = c.ldw(SP, 0x8);
                    c.sdl(7, c.pa(V0, 0x57));
                    c.sdr(7, c.pa(V0, 0x50));
                    c.stw(V0, 0x58, c.g[T0][0]);
                    c.g[V0][0] = c.ldw(A0, 0x54);
                    break 'b_1c7848;
                }
            }
            'b_1c786c: {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[A0][0] = c.ldw(A0, 0x60);
                    break 'b_1c786c;
                } else {
                    c.cycles += 7;
                    c.ldl(5, c.pa(SP, 0x7));
                    c.ldr(5, c.pa(SP, 0));
                    c.g[A2][0] = c.ldw(SP, 0x8);
                    c.sdl(5, c.pa(V0, 0x57));
                    c.sdr(5, c.pa(V0, 0x50));
                    c.stw(V0, 0x58, c.g[A2][0]);
                    c.g[A0][0] = c.ldw(A0, 0x60);
                    break 'b_1c786c;
                }
            }
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;

            if t {
                break 'b_1c78b4;
            } else {
                c.cycles += 8;
                c.ldl(9, c.pa(SP, 0x7));
                c.ldr(9, c.pa(SP, 0));
                c.g[V0][0] = c.ldw(SP, 0x8);
                c.sdl(9, c.pa(A0, 0x57));
                c.sdr(9, c.pa(A0, 0x50));
                c.stw(A0, 0x58, c.g[V0][0]);
                let t = true;

                if t {
                    break 'b_1c78b4;
                } else {
                    break 'b_1c7894;
                }
            }
        }
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;

        if t {
            break 'b_1c78b4;
        } else {
            c.cycles += 6;
            c.ldl(6, c.pa(SP, 0x7));
            c.ldr(6, c.pa(SP, 0));
            c.g[A3][0] = c.ldw(SP, 0x8);
            c.sdl(6, c.pa(A0, 0x57));
            c.sdr(6, c.pa(A0, 0x50));
            c.stw(A0, 0x58, c.g[A3][0]);
            break 'b_1c78b4;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
    return;
}

/// `int PS2GraphicsMan_MatchBlendedMatrix(Ent* a, Ent* b, int tol0, int tol1,
/// int tol2)` (0x369a78): do two render entries share a blended matrix?
/// Same type and key id, then type-specific equality shortcuts, then either
/// the same key ids with coordinates within the tolerances, or a swapped
/// (x/z, y) match.
pub fn ps2_graphics_man_match_blended_matrix(c: &mut Cpu) {
    'b_369c1c: {
        c.cycles += 4;
        c.g[T3][0] = c.ldw(A0, 0);
        c.g[V0][0] = c.ldw(A1, 0);
        let t = c.g[T3][0] != c.g[V0][0];
        c.g[V0][0] = 0u64;
        if t {
            break 'b_369c1c;
        } else {
            'b_369c18: {
                c.cycles += 7;
                c.g[V0][0] = c.ldw(A0, 0x4);
                c.g[V1][0] = c.ldw(A1, 0x4);
                c.g[T1][0] = c.g[V0][0];
                c.g[A0][0] = c.ldhu(V0, 0x2);
                c.g[V0][0] = c.ldhu(V1, 0x2);
                let t = c.g[A0][0] != c.g[V0][0];
                c.g[T2][0] = c.g[V1][0];
                if t {
                    break 'b_369c18;
                } else {
                    'b_369ba0: {
                        'b_369b20: {
                            'b_369ae4: {
                                'b_369b1c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x2;
                                    let t = c.g[T3][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[T3][0] as i64) < 3i64) as u64;
                                    if t {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.ldh(T1, 0);
                                        c.g[V0][0] = c.ldh(T2, 0);
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V1][0] = c.ldhu(T1, 0x6);
                                        if t {
                                            break 'b_369b20;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.ldhu(T2, 0x6);
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[V1][0] = c.ldhu(T1, 0xa);
                                                break 'b_369ba0;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V1][0] = c.ldh(T1, 0x4);
                                                let t = true;
                                                c.g[V0][0] = c.ldh(T2, 0x4);
                                                if t {
                                                    break 'b_369ae4;
                                                } else {
                                                    break 'b_369b1c;
                                                }
                                            }
                                        }
                                    } else {
                                        'b_369adc: {
                                            'b_369ac8: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_369ac8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[T3][0] == c.g[V0][0];
                                                    c.g[V0][0] = 0u64;
                                                    if t {
                                                        break 'b_369adc;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_369c1c;
                                                        } else {
                                                            break 'b_369ac8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x3;
                                            let t = c.g[T3][0] == c.g[V0][0];
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_369b1c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_369c1c;
                                                } else {
                                                    break 'b_369adc;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[V1][0] = c.ldh(T1, 0);
                                        c.g[V0][0] = c.ldh(T2, 0);
                                        break 'b_369ae4;
                                    }
                                }
                                c.cycles += 1;
                                c.g[V1][0] = c.ldhu(T1, 0x6);
                                break 'b_369b20;
                            }
                            c.cycles += 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V1][0] = c.ldhu(T1, 0x6);
                                break 'b_369b20;
                            } else {
                                c.cycles += 2;
                                let ra = c.g[RA][0] as u32;
                                c.g[V0][0] = 0x1;
                                c.pc = ra;
                                return;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.ldhu(T2, 0x6);
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V1][0] = c.ldhu(T1, 0xa);
                        if t {
                            break 'b_369ba0;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.ldhu(T2, 0xa);
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V0][0] = c.ldhu(T2, 0x6);
                                break 'b_369ba0;
                            } else {
                                'b_369b4c: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.ldh(T1, 0);
                                    c.g[V1][0] = c.ldh(T2, 0);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                                    if (c.g[V0][0] as i64) < 0 {
                                        c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                                        break 'b_369b4c;
                                    } else {
                                        break 'b_369b4c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_369c1c;
                                } else {
                                    'b_369b6c: {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.ldh(T1, 0x4);
                                        c.g[V1][0] = c.ldh(T2, 0x4);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_sub((c.g[V1][0] as u32)));
                                        if (c.g[V0][0] as i64) < 0 {
                                            c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                                            break 'b_369b6c;
                                        } else {
                                            break 'b_369b6c;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_369c1c;
                                    } else {
                                        'b_369b8c: {
                                            c.cycles += 5;
                                            c.g[V0][0] = c.ldh(T1, 0x8);
                                            c.g[V1][0] = c.ldh(T2, 0x8);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_sub((c.g[V1][0] as u32)));
                                            if (c.g[V0][0] as i64) < 0 {
                                                c.g[V0][0] =
                                                    sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                                                break 'b_369b8c;
                                            } else {
                                                break 'b_369b8c;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_369c1c;
                                        } else {
                                            c.cycles += 2;
                                            let ra = c.g[RA][0] as u32;
                                            c.g[V0][0] = 0x1;
                                            c.pc = ra;
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_369c1c;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.ldhu(T1, 0x6);
                        c.g[V0][0] = c.ldhu(T2, 0xa);
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_369c1c;
                        } else {
                            'b_369bcc: {
                                c.cycles += 5;
                                c.g[V0][0] = c.ldh(T1, 0);
                                c.g[V1][0] = c.ldh(T2, 0);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                                if (c.g[V0][0] as i64) < 0 {
                                    c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                                    break 'b_369bcc;
                                } else {
                                    break 'b_369bcc;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_369c1c;
                            } else {
                                'b_369bec: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.ldh(T1, 0x8);
                                    c.g[V1][0] = c.ldh(T2, 0x4);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                                    if (c.g[V0][0] as i64) < 0 {
                                        c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
                                        break 'b_369bec;
                                    } else {
                                        break 'b_369bec;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_369c1c;
                                } else {
                                    'b_369c0c: {
                                        c.cycles += 5;
                                        c.g[V1][0] = c.ldh(T1, 0x4);
                                        c.g[A0][0] = c.ldh(T2, 0x8);
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_sub((c.g[A0][0] as u32)));
                                        if (c.g[V1][0] as i64) < 0 {
                                            c.g[V1][0] = sx(0u32.wrapping_sub((c.g[V1][0] as u32)));
                                            break 'b_369c0c;
                                        } else {
                                            break 'b_369c0c;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V1][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_369c1c;
                                    } else {
                                        break 'b_369c18;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_369c1c;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
}

/// `void SSXAudioSystem_SetFadeGain(SaMix* self, int mask, float time,
/// float v)` (0x287a10): with `time == 0` set the channels selected by
/// `mask` immediately and, for the master channel (bit 2), recompute the ten
/// mix outputs from its value; otherwise store a fade (mask/target/rate) and
/// mark it running when the rate is non-zero.
pub fn ssx_audio_system_set_fade_gain(c: &mut Cpu) {
    'b_287c3c: {
        c.cycles += 5;
        c.mtc1(0, 2);
        c.c_eq_s(13, 2);
        let t = !c.fcc();
        c.g[V0][0] = c.g[A1][0] & 0x1;
        if t {
            c.cycles += 11;
            c.lwc1(0, c.pa(A0, 0x63d0));
            c.lwc1(1, c.pa(GP, -0x4620));
            c.sub_s(0, 12, 0);
            c.stw(A0, 0x63c4, c.g[A1][0]);
            c.swc1(12, c.pa(A0, 0x63cc));
            c.div_s(0, 0, 13);
            c.mul_s(0, 0, 1);
            c.c_eq_s(0, 2);
            let t = c.fcc();
            c.swc1(0, c.pa(A0, 0x63c8));
            if t {
                break 'b_287c3c;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                c.stw(A0, 0x63c0, c.g[V0][0]);
                break 'b_287c3c;
            }
        } else {
            'b_287a48: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x2;
                if t {
                    break 'b_287a48;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x62d0));
                    c.lwc1(1, c.pa(A0, 0x63e8));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x6398));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x62d4));
                    c.swc1(1, c.pa(A0, 0x62d8));
                    break 'b_287a48;
                }
            }
            'b_287a6c: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x4;
                if t {
                    break 'b_287a6c;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6310));
                    c.lwc1(1, c.pa(A0, 0x63f8));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63a8));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6314));
                    c.swc1(1, c.pa(A0, 0x6318));
                    break 'b_287a6c;
                }
            }
            'b_287b04: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.lwc1(1, c.pa(GP, 0x43c));
                if t {
                    break 'b_287b04;
                } else {
                    c.cycles += 36;
                    c.lwc1(11, c.pa(A0, 0x6300));
                    c.lwc1(0, c.pa(A0, 0x63f4));
                    c.mul_s(11, 11, 12);
                    c.lwc1(2, c.pa(GP, 0x440));
                    c.lwc1(4, c.pa(GP, 0x444));
                    c.mul_s(0, 11, 0);
                    c.lwc1(5, c.pa(GP, 0x448));
                    c.lwc1(7, c.pa(GP, 0x44c));
                    c.lwc1(9, c.pa(GP, 0x450));
                    c.lwc1(10, c.pa(GP, 0x454));
                    c.mul_s(1, 0, 1);
                    c.lwc1(8, c.pa(GP, 0x458));
                    c.mul_s(2, 0, 2);
                    c.lwc1(6, c.pa(GP, 0x45c));
                    c.mul_s(4, 0, 4);
                    c.lwc1(3, c.pa(GP, 0x460));
                    c.mul_s(5, 0, 5);
                    c.mul_s(7, 0, 7);
                    c.swc1(1, c.pa(A0, 0x636c));
                    c.mul_s(3, 0, 3);
                    c.swc1(2, c.pa(A0, 0x6370));
                    c.mul_s(9, 0, 9);
                    c.swc1(4, c.pa(A0, 0x6374));
                    c.mul_s(10, 0, 10);
                    c.swc1(5, c.pa(A0, 0x6378));
                    c.mul_s(8, 0, 8);
                    c.swc1(7, c.pa(A0, 0x637c));
                    c.mul_s(6, 0, 6);
                    c.swc1(9, c.pa(A0, 0x6380));
                    c.swc1(10, c.pa(A0, 0x6384));
                    c.swc1(8, c.pa(A0, 0x6388));
                    c.swc1(6, c.pa(A0, 0x638c));
                    c.swc1(3, c.pa(A0, 0x6390));
                    c.swc1(12, c.pa(A0, 0x63a4));
                    c.swc1(11, c.pa(A0, 0x6304));
                    c.swc1(0, c.pa(A0, 0x6308));
                    break 'b_287b04;
                }
            }
            'b_287b2c: {
                c.cycles += 3;
                c.g[V0][0] = c.g[A1][0] & 0x8;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x80;
                if t {
                    break 'b_287b2c;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x62e0));
                    c.lwc1(1, c.pa(A0, 0x63ec));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x639c));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x62e4));
                    c.swc1(1, c.pa(A0, 0x62e8));
                    break 'b_287b2c;
                }
            }
            'b_287b50: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x20;
                if t {
                    break 'b_287b50;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x62f0));
                    c.lwc1(1, c.pa(A0, 0x63f0));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63a0));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x62f4));
                    c.swc1(1, c.pa(A0, 0x62f8));
                    break 'b_287b50;
                }
            }
            'b_287b74: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x40;
                if t {
                    break 'b_287b74;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6320));
                    c.lwc1(1, c.pa(A0, 0x63fc));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63ac));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6324));
                    c.swc1(1, c.pa(A0, 0x6328));
                    break 'b_287b74;
                }
            }
            'b_287b98: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x10;
                if t {
                    break 'b_287b98;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6330));
                    c.lwc1(1, c.pa(A0, 0x6400));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63b0));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6334));
                    c.swc1(1, c.pa(A0, 0x6338));
                    break 'b_287b98;
                }
            }
            'b_287bbc: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x100;
                if t {
                    break 'b_287bbc;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6340));
                    c.lwc1(1, c.pa(A0, 0x6404));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63b4));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6344));
                    c.swc1(1, c.pa(A0, 0x6348));
                    break 'b_287bbc;
                }
            }
            'b_287be0: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x200;
                if t {
                    break 'b_287be0;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6350));
                    c.lwc1(1, c.pa(A0, 0x6408));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63b8));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6354));
                    c.swc1(1, c.pa(A0, 0x6358));
                    break 'b_287be0;
                }
            }
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_287c3c;
            } else {
                c.cycles += 8;
                c.lwc1(0, c.pa(A0, 0x6360));
                c.lwc1(1, c.pa(A0, 0x640c));
                c.mul_s(0, 0, 12);
                c.swc1(12, c.pa(A0, 0x63bc));
                c.mul_s(1, 0, 1);
                c.swc1(0, c.pa(A0, 0x6364));
                let ra = c.g[RA][0] as u32;
                c.swc1(1, c.pa(A0, 0x6368));
                c.pc = ra;
                return;
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
}

/// `void AnimObject_v36(ClampFloat* self, int msg, float v)` (0x341fe8):
/// the clamp object's message handler: 0x64 sets the frame count from the
/// frame rate, 0x65/0x68 set the value/min, 0x66 the speed, 0x67 the max;
/// 0x69 clears the first word; anything else forwards to the base object at
/// `+0x30`.
pub fn anim_object_v36(c: &mut Cpu) {
    'b_342144: {
        'b_342018: {
            c.cycles += 4;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.g[V0][0] = 0x64;
            let t = c.g[A1][0] != c.g[V0][0];
            c.std(SP, 0, c.g[RA][0]);
            if t {
                break 'b_342018;
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.ldw(GP, 0x2a74);
                c.lwc1(0, c.pa(V0, 0x10));
                c.cvt_s_w(0, 0);
                c.mul_s(0, 12, 0);
                c.cvt_w_s(1, 0);
                c.swc1(1, c.pa(A0, 0xc));
                let t = true;
                c.g[RA][0] = c.ldd(SP, 0);
                if t {
                    break 'b_342144;
                } else {
                    break 'b_342018;
                }
            }
        }
        'b_342140: {
            'b_342128: {
                'b_342118: {
                    'b_342050: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x65;
                        if c.g[A1][0] != c.g[V0][0] {
                            c.g[V0][0] = 0x66;
                            break 'b_342050;
                        } else {
                            c.cycles += 7;
                            c.lwc1(0, c.pa(GP, -0x2b7c));
                            c.lwc1(1, c.pa(A0, 0x14));
                            c.mul_s(12, 12, 0);
                            c.c_le_s(1, 12);
                            let t = !c.fcc();
                            c.g[RA][0] = c.ldd(SP, 0);
                            if t {
                                break 'b_342144;
                            } else {
                                c.cycles += 4;
                                c.lwc1(0, c.pa(A0, 0x18));
                                c.c_le_s(12, 0);
                                let t = true;

                                if t {
                                    break 'b_342118;
                                } else {
                                    break 'b_342050;
                                }
                            }
                        }
                    }
                    'b_342078: {
                        c.cycles += 2;
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x67;
                        if t {
                            break 'b_342078;
                        } else {
                            c.cycles += 8;
                            c.g[V0][0] = c.ldw(GP, 0x2a74);
                            c.lwc1(1, c.pa(GP, -0x2b78));
                            c.lwc1(0, c.pa(V0, 0x10));
                            c.cvt_s_w(0, 0);
                            c.mul_s(12, 12, 1);
                            c.div_s(0, 12, 0);
                            let t = true;
                            c.swc1(0, c.pa(A0, 0x10));
                            if t {
                                break 'b_342140;
                            } else {
                                break 'b_342078;
                            }
                        }
                    }
                    'b_3420c8: {
                        c.cycles += 2;
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x68;
                        if t {
                            break 'b_3420c8;
                        } else {
                            'b_3420a4: {
                                c.cycles += 9;
                                c.g[V0][0] = c.ldw(A0, 0x48);
                                c.lwc1(0, c.pa(GP, -0x2b74));
                                c.g[V1][0] = c.ldw(V0, 0x80);
                                c.mul_s(12, 12, 0);
                                c.lwc1(0, c.pa(V1, 0x14));
                                c.c_lt_s(0, 12);
                                if c.fcc() {
                                    c.mov_s(12, 0);
                                    break 'b_3420a4;
                                } else {
                                    break 'b_3420a4;
                                }
                            }
                            'b_3420b8: {
                                c.cycles += 5;
                                c.mtc1(0, 0);
                                c.c_lt_s(12, 0);
                                if c.fcc() {
                                    c.mov_s(12, 0);
                                    break 'b_3420b8;
                                } else {
                                    break 'b_3420b8;
                                }
                            }
                            c.cycles += 4;
                            c.lwc1(0, c.pa(A0, 0x1c));
                            c.c_lt_s(12, 0);
                            let t = true;
                            c.swc1(12, c.pa(A0, 0x18));
                            if t {
                                break 'b_342118;
                            } else {
                                break 'b_3420c8;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x69;
                    if t {
                        break 'b_342128;
                    } else {
                        'b_3420f4: {
                            c.cycles += 9;
                            c.g[V0][0] = c.ldw(A0, 0x48);
                            c.lwc1(0, c.pa(GP, -0x2b70));
                            c.g[V1][0] = c.ldw(V0, 0x80);
                            c.mul_s(12, 12, 0);
                            c.lwc1(0, c.pa(V1, 0x14));
                            c.c_lt_s(0, 12);
                            if c.fcc() {
                                c.mov_s(12, 0);
                                break 'b_3420f4;
                            } else {
                                break 'b_3420f4;
                            }
                        }
                        'b_342108: {
                            c.cycles += 5;
                            c.mtc1(0, 0);
                            c.c_lt_s(12, 0);
                            if c.fcc() {
                                c.mov_s(12, 0);
                                break 'b_342108;
                            } else {
                                break 'b_342108;
                            }
                        }
                        c.cycles += 4;
                        c.lwc1(0, c.pa(A0, 0x1c));
                        c.swc1(12, c.pa(A0, 0x14));
                        c.c_lt_s(0, 12);
                        break 'b_342118;
                    }
                }
                c.cycles += 2;
                if c.fcc() {
                    c.swc1(12, c.pa(A0, 0x1c));
                    break 'b_342140;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[RA][0] = c.ldd(SP, 0);
                    if t {
                        break 'b_342144;
                    } else {
                        break 'b_342128;
                    }
                }
            }
            'b_342138: {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];

                if t {
                    break 'b_342138;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.sth(A0, 0, 0u64);
                    if t {
                        break 'b_342140;
                    } else {
                        break 'b_342138;
                    }
                }
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x00342140);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
            c.pc = 0x00356b08;
            call_indirect(c, 0x00356b08);
            if c.pc != 0x00342140 {
                return;
            }
            break 'b_342140;
        }
        c.cycles += 1;
        c.g[RA][0] = c.ldd(SP, 0);
        break 'b_342144;
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
    return;
}

/// `void CrowdAudio_InstanceRemoveSource(SsMgr* self, int idx, float fade)`
/// (0x2a62f0): drop voice `idx` (12-byte entries at `+0x20`): release the
/// reference, free its stream slot when the last reference goes, and when no
/// active voices remain reset the manager (clamped volume, stream id,
/// state).
pub fn crowd_audio_instance_remove_source(c: &mut Cpu) {
    'b_2a6418: {
        'b_2a6414: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, c.pa(SP, 0x20));
            c.swc1(20, c.pa(SP, 0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, c.pa(SP, 0x10));
            c.mov_s(20, 12);
            let t = (c.g[A1][0] as i64) < 0;
            c.std(SP, 0, c.g[RA][0]);
            if t {
                break 'b_2a6414;
            } else {
                c.cycles += 7;
                c.g[V0][0] = 0xc;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                c.mult(0, 5, 5, 2);
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V0][0] = c.ldw(A0, 0);
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28));
                if t {
                    break 'b_2a6414;
                } else {
                    'b_2a6390: {
                        c.cycles += 11;
                        c.g[V0][0] = c.ldw(S0, 0xd4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        c.g[S1][0] = c.ldw(V1, 0);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.stw(S0, 0xd4, c.g[V0][0]);
                        c.stw(A0, 0, 0u64);
                        c.stw(V1, 0, 0u64);
                        c.g[V0][0] = c.ldw(S1, 0x8);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        let t = c.g[V0][0] != 0u64;
                        c.stw(S1, 0x8, c.g[V0][0]);
                        if t {
                            break 'b_2a6390;
                        } else {
                            'b_2a6374: {
                                c.cycles += 3;
                                c.g[A1][0] = c.ldw(S1, 0x4);
                                let t = (c.g[A1][0] as i64) < 0;
                                c.g[V0][0] = c.ldw(GP, 0x608);
                                if t {
                                    break 'b_2a6374;
                                } else {
                                    c.cycles += 4;
                                    c.g[A2][0] = 0x1;
                                    c.g[A0][0] = c.ldw(V0, 0);
                                    c.g[RA][0] = sx(0x002a6374);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                    c.pc = 0x002ad5f0;
                                    call_indirect(c, 0x002ad5f0);
                                    if c.pc != 0x002a6374 {
                                        return;
                                    }
                                    break 'b_2a6374;
                                }
                            }
                            c.cycles += 7;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.stw(S1, 0, c.g[V0][0]);
                            c.stw(S1, 0x4, c.g[V0][0]);
                            c.stw(S0, 0x18c, 0u64);
                            c.g[V0][0] = c.ldw(S0, 0x170);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            c.stw(S0, 0x170, c.g[V0][0]);
                            break 'b_2a6390;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.ldw(S0, 0xd4);
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, c.pa(SP, 0x20));
                        break 'b_2a6418;
                    } else {
                        'b_2a63b8: {
                            c.cycles += 7;
                            c.lwc1(0, c.pa(S0, 0x188));
                            c.g[AT][0] = 0x3e800000;
                            c.mtc1(1, 1);
                            c.c_lt_s(1, 0);
                            if c.fcc() {
                                c.swc1(1, c.pa(S0, 0x188));
                                break 'b_2a63b8;
                            } else {
                                break 'b_2a63b8;
                            }
                        }
                        'b_2a640c: {
                            c.cycles += 3;
                            c.g[A1][0] = c.ldw(S0, 0x174);
                            let t = (c.g[A1][0] as i64) < 0;
                            c.g[V0][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_2a640c;
                            } else {
                                'b_2a63ec: {
                                    c.cycles += 5;
                                    c.mtc1(0, 0);
                                    c.c_eq_s(20, 0);
                                    let t = !c.fcc();
                                    c.g[V0][0] = c.ldw(GP, 0x608);
                                    if t {
                                        break 'b_2a63ec;
                                    } else {
                                        c.cycles += 5;
                                        c.mov_s(12, 20);
                                        c.g[A2][0] = 0x2;
                                        c.g[A0][0] = c.ldw(V0, 0);
                                        c.g[RA][0] = sx(0x002a63ec);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                        c.pc = 0x002ad5f0;
                                        call_indirect(c, 0x002ad5f0);
                                        if c.pc != 0x002a63ec {
                                            return;
                                        }
                                        break 'b_2a63ec;
                                    }
                                }
                                c.cycles += 5;
                                c.g[V0][0] = 0xffffffffffffffff;
                                c.g[A0][0] = c.ldw(S0, 0x180);
                                c.g[V1][0] = 0x2;
                                let t = c.g[A0][0] != c.g[V1][0];
                                c.stw(S0, 0x174, c.g[V0][0]);
                                if t {
                                    break 'b_2a640c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x1;
                                    c.stw(S0, 0x180, c.g[V0][0]);
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    break 'b_2a640c;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.stw(S0, 0x178, 0u64);
                        c.stw(S0, 0, c.g[V0][0]);
                        break 'b_2a6414;
                    }
                }
            }
        }
        c.cycles += 1;
        c.lq(16, c.pa(SP, 0x20));
        break 'b_2a6418;
    }
    c.cycles += 5;
    c.lq(17, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    c.lwc1(20, c.pa(SP, 0x30));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    return;
}

/// `void LineStrip_Finish(PktBuilder* self, int flags)` (0x384e50): close
/// the strip's packet: flush the open block, emit the two-quadword draw kick
/// from the current node (batching under 2600 vertices), advance the
/// cursors, and optionally release the builder's packet.
pub fn line_strip_finish(c: &mut Cpu) {
    'b_384fa8: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, c.pa(SP, 0x20));
        c.sq(17, c.pa(SP, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.std(SP, 0, c.g[RA][0]);
        c.g[V0][0] = c.ldw(S0, 0x8);
        let t = c.g[V0][0] == 0u64;
        c.g[S1][0] = c.g[A1][0];
        if t {
            break 'b_384fa8;
        } else {
            'b_384e88: {
                c.cycles += 3;
                c.g[V0][0] = c.ldw(S0, 0xc);
                if c.g[V0][0] == 0u64 {
                    c.g[A0][0] = c.ldw(S0, 0x8);
                    break 'b_384e88;
                } else {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00384e84);
                    c.pc = 0x00384dc0;
                    call_indirect(c, 0x00384dc0);
                    if c.pc != 0x00384e84 {
                        return;
                    }
                    c.g[A0][0] = c.ldw(S0, 0x8);
                    break 'b_384e88;
                }
            }
            'b_384f94: {
                c.cycles += 34;
                c.g[A3][0] = 0x10000000;
                c.g[V0][0] = c.ldw(S0, 0x4);
                c.g[A2][0] = 0x50000000;
                c.g[A1][0] = 0xfff0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[V1][0] = c.g[V1][0] | c.g[A3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
                c.std(A0, 0, c.g[V1][0]);
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V1][0] = c.ldw(S0, 0x8);
                c.std(V1, 0x8, 0u64);
                c.g[A0][0] = c.ldw(S0, 0x8);
                c.std(A0, 0x10, 0u64);
                c.g[V1][0] = c.ldw(S0, 0x8);
                c.std(V1, 0x18, c.g[V0][0]);
                c.g[V0][0] = c.ldw(S0, 0x4);
                c.g[A0][0] = c.ldw(S0, 0x8);
                c.g[V1][0] = c.ldw(GP, 0x2a90);
                c.g[T1][0] = c.g[V0][0] & c.g[A1][0];
                c.g[T2][0] = c.g[A0][0] & c.g[A1][0];
                c.stw(S0, 0x4, c.g[T1][0]);
                c.stw(S0, 0x8, c.g[T2][0]);
                c.g[A3][0] = c.ldw(V1, 0x18f0);
                c.g[V0][0] = c.ldw(A3, 0);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[T0][0] = c.ldw(V1, 0xe84);
                if t {
                    break 'b_384f94;
                } else {
                    c.cycles += 33;
                    c.g[A1][0] = 0x60000;
                    c.g[A0][0] = c.ldw(T0, 0x8);
                    c.g[A1][0] = c.g[A1][0] | 0x8000;
                    c.g[V1][0] = 0xffffffffffffffe0;
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                    c.g[V0][0] = c.ldw(A1, 0x1cc4);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = 0x30000000;
                    c.g[V0][0] = c.g[V0][0] & 0x1f;
                    c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
                    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                    c.stw(T0, 0x8, c.g[A0][0]);
                    c.g[V1][0] = c.ldw(A3, 0);
                    c.g[A0][0] = c.ldw(A1, 0x1cc0);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.stw(A3, 0, c.g[V1][0]);
                    c.ldl(3, c.pa(T0, 0x7));
                    c.ldr(3, c.pa(T0, 0));
                    c.ldl(5, c.pa(T0, 0xf));
                    c.ldr(5, c.pa(T0, 0x8));
                    c.g[A2][0] = c.ldw(T0, 0x10);
                    c.sdl(3, c.pa(V0, 0x7));
                    c.sdr(3, c.pa(V0, 0));
                    c.sdl(5, c.pa(V0, 0xf));
                    c.sdr(5, c.pa(V0, 0x8));
                    c.stw(V0, 0x10, c.g[A2][0]);
                    c.stw(V0, 0x14, c.g[T2][0]);
                    c.stw(V0, 0x18, c.g[T1][0]);
                    c.sth(V0, 0x1c, c.g[A0][0]);
                    c.sth(V0, 0x1e, 0u64);
                    break 'b_384f94;
                }
            }
            c.cycles += 5;
            c.g[V0][0] = c.ldw(S0, 0x4);
            c.g[V1][0] = c.ldw(S0, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            c.stw(S0, 0x4, c.g[V0][0]);
            c.stw(V1, 0, c.g[V0][0]);
            break 'b_384fa8;
        }
    }
    'b_384fc0: {
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0u64 {
            c.lq(16, c.pa(SP, 0x20));
            break 'b_384fc0;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x00384fbc);
            c.g[A0][0] = c.g[S0][0];
            c.pc = 0x00317e50;
            call_indirect(c, 0x00317e50);
            if c.pc != 0x00384fbc {
                return;
            }
            c.lq(16, c.pa(SP, 0x20));
            break 'b_384fc0;
        }
    }
    c.cycles += 4;
    c.lq(17, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = ra;
    return;
}

/// `void TexFlipModifier_Update(FrameMod* self)` (0x35f410): advance the
/// flip timer; when it reaches the period, switch the texture page (picking
/// between the two page variants from the frame parity), re-arm the timer
/// and notify the owner.
pub fn tex_flip_modifier_update(c: &mut Cpu) {
    'b_35f584: {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.mtc1(0, 0);
        c.sq(16, c.pa(SP, 0x20));
        c.sq(17, c.pa(SP, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.std(SP, 0, c.g[RA][0]);
        c.swc1(20, c.pa(SP, 0x30));
        c.lwc1(1, c.pa(S0, 0x8));
        c.c_lt_s(0, 1);
        if !c.fcc() {
            c.lq(16, c.pa(SP, 0x20));
            break 'b_35f584;
        } else {
            'b_35f580: {
                c.cycles += 8;
                c.lwc1(0, c.pa(S0, 0x4));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.add_s(0, 0, 1);
                c.c_le_s(20, 0);
                let t = !c.fcc();
                c.swc1(0, c.pa(S0, 0x4));
                if t {
                    break 'b_35f580;
                } else {
                    'b_35f578: {
                        'b_35f574: {
                            'b_35f4b4: {
                                c.cycles += 4;
                                c.sub_s(0, 0, 20);
                                c.g[V1][0] = c.ldw(S0, 0);
                                let t = c.g[V1][0] != 0u64;
                                c.swc1(0, c.pa(S0, 0x4));
                                if t {
                                    break 'b_35f4b4;
                                } else {
                                    'b_35f498: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.ldw(S0, 0x10);
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = c.ldw(S0, 0x14);
                                        if t {
                                            break 'b_35f498;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V1][0] = c.ldw(S0, 0x20);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.g[V1][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            let t = c.g[V1][0] != 0u64;
                                            c.stw(S0, 0x14, c.g[V0][0]);
                                            if t {
                                                break 'b_35f574;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.stw(S0, 0x14, 0u64);
                                                if t {
                                                    break 'b_35f574;
                                                } else {
                                                    break 'b_35f498;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    let t = (c.g[V0][0] as i64) >= 0;
                                    c.stw(S0, 0x14, c.g[V0][0]);
                                    if t {
                                        break 'b_35f574;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.ldw(S0, 0x20);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        let t = true;
                                        c.stw(S0, 0x14, c.g[V0][0]);
                                        if t {
                                            break 'b_35f574;
                                        } else {
                                            break 'b_35f4b4;
                                        }
                                    }
                                }
                            }
                            'b_35f504: {
                                c.cycles += 3;
                                c.g[S1][0] = 0x1;
                                let t = c.g[V1][0] != c.g[S1][0];
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_35f504;
                                } else {
                                    c.cycles += 15;
                                    c.g[V1][0] = c.ldw(GP, 0x2a74);
                                    c.g[A0][0] = 0x2;
                                    c.g[AT][0] = 0x41200000;
                                    c.mtc1(1, 0);
                                    c.lwc1(1, c.pa(V1, 0x10));
                                    c.cvt_s_w(1, 1);
                                    c.g[V0][0] = c.ldw(S0, 0x14);
                                    c.g[V1][0] = c.ldw(S0, 0x20);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    c.stw(S0, 0, c.g[A0][0]);
                                    c.div_s(0, 0, 1);
                                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    c.stw(S0, 0x14, c.g[V0][0]);
                                    let t = c.g[V1][0] != 0u64;
                                    c.swc1(0, c.pa(S0, 0x8));
                                    if t {
                                        break 'b_35f574;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.stw(S0, 0x14, 0u64);
                                        if t {
                                            break 'b_35f574;
                                        } else {
                                            break 'b_35f504;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_35f578;
                            } else {
                                c.cycles += 25;
                                c.g[RA][0] = sx(0x0035f514);
                                c.pc = 0x003177f0;
                                call_indirect(c, 0x003177f0);
                                if c.pc != 0x0035f514 {
                                    return;
                                }
                                c.g[V1][0] = 0x7f0000;
                                c.g[A1][0] = 0x3f800000;
                                c.g[V1][0] = c.g[V1][0] | 0xffff;
                                c.g[AT][0] = 0x3f400000;
                                c.mtc1(1, 2);
                                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                c.g[AT][0] = 0x3e800000;
                                c.mtc1(1, 3);
                                c.g[A0][0] = c.g[V0][0] | c.g[A1][0];
                                c.lwc1(1, c.pa(S0, 0xc));
                                c.mtc1(4, 4);
                                c.g[V0][0] = c.ldw(S0, 0x14);
                                c.sub_s(0, 4, 20);
                                c.g[A0][0] = c.ldw(S0, 0x20);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.stw(S0, 0, c.g[S1][0]);
                                c.g[A0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                c.stw(S0, 0x14, c.g[V0][0]);
                                c.mul_s(0, 0, 2);
                                c.add_s(0, 0, 3);
                                c.mul_s(1, 1, 0);
                                let t = c.g[A0][0] != 0u64;
                                c.swc1(1, c.pa(S0, 0x8));
                                if t {
                                    break 'b_35f574;
                                } else {
                                    c.cycles += 1;
                                    c.stw(S0, 0x14, 0u64);
                                    break 'b_35f574;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S0][0];
                        break 'b_35f578;
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0035f580);
                    c.g[A1][0] = c.ldw(A0, 0x14);
                    c.pc = 0x0035f378;
                    call_indirect(c, 0x0035f378);
                    if c.pc != 0x0035f580 {
                        return;
                    }
                    break 'b_35f580;
                }
            }
            c.cycles += 1;
            c.lq(16, c.pa(SP, 0x20));
            break 'b_35f584;
        }
    }
    c.cycles += 5;
    c.lq(17, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    c.lwc1(20, c.pa(SP, 0x30));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    return;
}

/// `int BENewPlayerInt_GetPeakFlagB(void* self, int index, int c, int kind,
/// int flag)` (0x1464d0): one bit of a character record's flags word
/// (`D_004A6CA8 + profile*0x9b50 + c*0xf88 + 0x278`), bit 6/7/8 of `kind`
/// 0/1/2 when `flag`, bits 9/10/11 otherwise. The profile index comes from
/// `cBELibrary_getProfileIndex(index)`.
pub fn be_new_player_int_get_peak_flag_b(c: &mut Cpu) {
    'b_146684: {
        'b_1465c4: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A0][0] = c.g[A1][0];
            c.sq(16, c.pa(SP, 0x30));
            c.sq(17, c.pa(SP, 0x20));
            c.g[S0][0] = c.g[T0][0];
            c.sq(18, c.pa(SP, 0x10));
            c.g[S1][0] = c.g[A3][0];
            c.std(SP, 0, c.g[RA][0]);
            c.g[RA][0] = sx(0x001464f8);
            c.g[S2][0] = c.g[A2][0];
            c.pc = 0x0014a0e0;
            call_indirect(c, 0x0014a0e0);
            if c.pc != 0x001464f8 {
                return;
            }
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = c.g[V0][0];
            if t {
                break 'b_1465c4;
            } else {
                'b_146594: {
                    'b_146564: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[S1][0] == c.g[V0][0];
                        c.g[V1][0] = 0xf88;
                        if t {
                            break 'b_146564;
                        } else {
                            'b_146538: {
                                'b_146528: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = 0x2;
                                        break 'b_146528;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == 0u64;
                                        c.g[A0][0] = 0x9b50;
                                        if t {
                                            break 'b_146538;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_146684;
                                            } else {
                                                break 'b_146528;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S1][0] == c.g[V0][0];
                                c.g[V1][0] = 0xf88;
                                if t {
                                    break 'b_146594;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_146684;
                                    } else {
                                        break 'b_146538;
                                    }
                                }
                            }
                            c.cycles += 11;
                            c.mult(0, 3, 18, 3);
                            c.mult(0, 4, 5, 4);
                            c.g[V0][0] = 0x4a0000;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[V0][0] = c.ldd(V1, 0x278);
                            c.g[V0][0] = c.g[V0][0] << 26;
                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                            let t = true;
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            if t {
                                break 'b_146684;
                            } else {
                                break 'b_146564;
                            }
                        }
                    }
                    c.cycles += 12;
                    c.g[A0][0] = 0x9b50;
                    c.mult(0, 3, 18, 3);
                    c.mult(0, 4, 5, 4);
                    c.g[V0][0] = 0x4a0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V0][0] = c.ldd(V1, 0x278);
                    c.g[V0][0] = c.g[V0][0] << 25;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    if t {
                        break 'b_146684;
                    } else {
                        break 'b_146594;
                    }
                }
                c.cycles += 12;
                c.g[A0][0] = 0x9b50;
                c.mult(0, 3, 18, 3);
                c.mult(0, 4, 5, 4);
                c.g[V0][0] = 0x4a0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V0][0] = c.ldd(V1, 0x278);
                c.g[V0][0] = c.g[V0][0] << 24;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                let t = true;
                c.g[V0][0] = c.g[V0][0] & 0x1;
                if t {
                    break 'b_146684;
                } else {
                    break 'b_1465c4;
                }
            }
        }
        'b_146658: {
            'b_146628: {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let t = c.g[S1][0] == c.g[V0][0];
                c.g[V1][0] = 0xf88;
                if t {
                    break 'b_146628;
                } else {
                    'b_1465fc: {
                        'b_1465ec: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = 0x2;
                                break 'b_1465ec;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S1][0] == 0u64;
                                c.g[A0][0] = 0x9b50;
                                if t {
                                    break 'b_1465fc;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_146684;
                                    } else {
                                        break 'b_1465ec;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S1][0] == c.g[V0][0];
                        c.g[V1][0] = 0xf88;
                        if t {
                            break 'b_146658;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_146684;
                            } else {
                                break 'b_1465fc;
                            }
                        }
                    }
                    c.cycles += 11;
                    c.mult(0, 3, 18, 3);
                    c.mult(0, 4, 5, 4);
                    c.g[V0][0] = 0x4a0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V0][0] = c.ldd(V1, 0x278);
                    c.g[V0][0] = c.g[V0][0] << 23;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    if t {
                        break 'b_146684;
                    } else {
                        break 'b_146628;
                    }
                }
            }
            c.cycles += 12;
            c.g[A0][0] = 0x9b50;
            c.mult(0, 3, 18, 3);
            c.mult(0, 4, 5, 4);
            c.g[V0][0] = 0x4a0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V0][0] = c.ldd(V1, 0x278);
            c.g[V0][0] = c.g[V0][0] << 22;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            let t = true;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            if t {
                break 'b_146684;
            } else {
                break 'b_146658;
            }
        }
        c.cycles += 11;
        c.g[A0][0] = 0x9b50;
        c.mult(0, 3, 18, 3);
        c.mult(0, 4, 5, 4);
        c.g[V0][0] = 0x4a0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V0][0] = c.ldd(V1, 0x278);
        c.g[V0][0] = c.g[V0][0] << 21;
        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
        c.g[V0][0] = c.g[V0][0] & 0x1;
        break 'b_146684;
    }
    c.cycles += 6;
    c.lq(16, c.pa(SP, 0x30));
    c.lq(17, c.pa(SP, 0x20));
    c.lq(18, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    return;
}

/// `void CrowdAudio_PushPlayParams(CrowdAudio* self)` (0x2a7468): append the
/// crowd voice's play parameters to its ring buffers (the current slot is
/// `+0x1f4`, the template at `+0x1d8`), clearing the per-slot fields the
/// mixer starts from.
pub fn crowd_audio_push_play_params(c: &mut Cpu) {
    c.cycles += 132;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.g[T1][0] = 0x18;
    c.sq(17, c.pa(SP, 0x10));
    c.g[A1][0] = 0u64;
    c.std(SP, 0, c.g[RA][0]);
    c.g[A2][0] = 0x8;
    c.sq(16, c.pa(SP, 0x20));
    c.g[S0][0] = c.ldw(A0, 0);
    c.g[V0][0] = c.ldw(S0, 0x1f4);
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.stw(S0, 0x1f4, c.g[V0][0]);
    c.g[V1][0] = c.ldw(S0, 0x1f4);
    c.g[V0][0] = c.ldw(S1, 0x20);
    c.mult(0, 3, 3, 9);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.ldl(2, c.pa(S0, 0x1e3));
    c.ldr(2, c.pa(S0, 0x1dc));
    c.ldl(4, c.pa(S0, 0x1eb));
    c.ldr(4, c.pa(S0, 0x1e4));
    c.ldl(7, c.pa(S0, 0x1f3));
    c.ldr(7, c.pa(S0, 0x1ec));
    c.sdl(2, c.pa(V1, 0x7));
    c.sdr(2, c.pa(V1, 0));
    c.sdl(4, c.pa(V1, 0xf));
    c.sdr(4, c.pa(V1, 0x8));
    c.sdl(7, c.pa(V1, 0x17));
    c.sdr(7, c.pa(V1, 0x10));
    c.g[V0][0] = c.ldw(S0, 0x1f4);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw(V0, 0x1fc, 0u64);
    c.g[V1][0] = c.ldw(S0, 0x1f4);
    c.g[V0][0] = c.ldw(S1, 0x34);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw(V1, 0, 0u64);
    c.g[A3][0] = c.ldw(S0, 0x1f4);
    c.g[A0][0] = c.ldw(S0, 0x1f4);
    c.g[T0][0] = c.ldw(S1, 0x20);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 9);
    c.g[V0][0] = c.ldw(S1, 0x38);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
    c.g[V1][0] = c.ldb(A0, 0);
    c.stw(A3, 0, c.g[V1][0]);
    c.g[V0][0] = c.ldw(S0, 0x1f4);
    c.g[V1][0] = c.ldw(S1, 0x3c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw(V0, 0, 0u64);
    c.g[A0][0] = c.ldw(S0, 0x1f4);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x002a754c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.pc = 0x00416210;
    call_indirect(c, 0x00416210);
    if c.pc != 0x002a754c {
        return;
    }
    c.g[V1][0] = c.ldw(S0, 0x1f4);
    c.g[A0][0] = 0x64;
    c.g[V0][0] = c.ldw(S1, 0x60);
    c.g[A1][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A2][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A3][0] = 0x7f;
    c.stw(V1, 0, 0u64);
    c.g[T0][0] = 0x1;
    c.g[V0][0] = c.ldw(S0, 0x1f4);
    c.g[V1][0] = c.ldw(S1, 0x64);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw(V0, 0, 0u64);
    c.g[V1][0] = c.ldw(S0, 0x1f4);
    c.g[V0][0] = c.ldw(S1, 0x68);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw(V1, 0, 0u64);
    c.g[V0][0] = c.ldw(S0, 0x1f4);
    c.g[V1][0] = c.ldw(S1, 0x6c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw(V0, 0, 0u64);
    c.g[V1][0] = c.ldw(S0, 0x1f4);
    c.g[V0][0] = c.ldw(S1, 0x70);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw(V1, 0x4, 0u64);
    c.g[V0][0] = c.ldw(S0, 0x1f4);
    c.g[V1][0] = c.ldw(S1, 0x70);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw(V0, 0, c.g[A0][0]);
    c.g[V1][0] = c.ldw(S0, 0x1f4);
    c.g[V0][0] = c.ldw(S1, 0x70);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw(V1, 0x8, c.g[A1][0]);
    c.g[V0][0] = c.ldw(S0, 0x1f4);
    c.g[V1][0] = c.ldw(S1, 0x70);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw(V0, 0xc, c.g[A2][0]);
    c.g[V1][0] = c.ldw(S0, 0x1f4);
    c.g[V0][0] = c.ldw(S1, 0x74);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw(V1, 0, 0u64);
    c.g[V0][0] = c.ldw(S0, 0x1f4);
    c.g[V1][0] = c.ldw(S1, 0x80);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw(V0, 0, 0u64);
    c.g[V1][0] = c.ldw(S0, 0x1f4);
    c.g[V0][0] = c.ldw(S1, 0x84);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw(V1, 0, 0u64);
    c.g[V0][0] = c.ldw(S0, 0x1f4);
    c.g[V1][0] = c.ldw(S1, 0x78);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw(V0, 0, c.g[A3][0]);
    c.g[V1][0] = c.ldw(S0, 0x1f4);
    c.g[V0][0] = c.ldw(S1, 0x7c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.lq(16, c.pa(SP, 0x20));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lq(17, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    c.stw(V1, 0, c.g[T0][0]);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = ra;
    return;
}

/// `void PS2GraphicsMan_RotateAxis(GraphicsMan* self, const float* ang)`
/// (0x395d60): build the rotation matrix for the three angles through
/// `Math_SinCos` and transform the manager's `+0x13e4` matrix in place, then
/// clear `+0x6b90`.
pub fn ps2_graphics_man_rotate_axis(c: &mut Cpu) {
    c.cycles += 94;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
    c.sq(16, c.pa(SP, 0x80));
    c.sq(17, c.pa(SP, 0x70));
    c.g[S0][0] = c.g[A1][0];
    c.std(SP, 0x50, c.g[RA][0]);
    c.g[S1][0] = c.g[A0][0];
    c.sq(18, c.pa(SP, 0x60));
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S2][0] = c.ldw(S1, 0x13e4);
    c.g[RA][0] = sx(0x00395d8c);
    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x44));
    c.pc = 0x0031be50;
    call_indirect(c, 0x0031be50);
    if c.pc != 0x00395d8c {
        return;
    }
    c.lwc1(11, c.pa(SP, 0x44));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 5);
    c.lwc1(4, c.pa(S0, 0x8));
    c.sub_s(1, 5, 11);
    c.lwc1(0, c.pa(S0, 0));
    c.lwc1(6, c.pa(S0, 0x4));
    c.lwc1(8, c.pa(SP, 0x40));
    c.mul_s(2, 1, 4);
    c.swc1(5, c.pa(SP, 0x3c));
    c.mul_s(3, 1, 0);
    c.stw(SP, 0x30, 0u64);
    c.mul_s(1, 1, 6);
    c.stw(SP, 0x34, 0u64);
    c.mul_s(13, 8, 4);
    c.stw(SP, 0x38, 0u64);
    c.mul_s(12, 8, 0);
    c.stw(SP, 0xc, 0u64);
    c.mul_s(9, 2, 4);
    c.stw(SP, 0x1c, 0u64);
    c.mul_s(10, 3, 4);
    c.stw(SP, 0x2c, 0u64);
    c.mul_s(5, 2, 0);
    c.mul_s(7, 3, 0);
    c.mul_s(4, 1, 4);
    c.mul_s(0, 1, 0);
    c.mul_s(8, 8, 6);
    c.mul_s(2, 2, 6);
    c.mul_s(3, 3, 6);
    c.mul_s(1, 1, 6);
    c.add_s(5, 5, 8);
    c.sub_s(0, 0, 13);
    c.sub_s(2, 2, 12);
    c.add_s(9, 9, 11);
    c.swc1(5, c.pa(SP, 0x8));
    c.add_s(7, 7, 11);
    c.swc1(0, c.pa(SP, 0x4));
    c.add_s(3, 3, 13);
    c.swc1(2, c.pa(SP, 0x18));
    c.sub_s(10, 10, 8);
    c.swc1(9, c.pa(SP, 0x28));
    c.add_s(1, 1, 11);
    c.swc1(7, c.pa(SP, 0));
    c.add_s(4, 4, 12);
    c.swc1(3, c.pa(SP, 0x10));
    c.swc1(10, c.pa(SP, 0x20));
    c.swc1(1, c.pa(SP, 0x14));
    c.swc1(4, c.pa(SP, 0x24));
    c.lqc2(4, c.pa(S2, 0));
    c.lqc2(5, c.pa(S2, 0x10));
    c.lqc2(6, c.pa(S2, 0x20));
    c.lqc2(7, c.pa(S2, 0x30));
    c.lqc2(8, c.pa(SP, 0));
    c.lqc2(9, c.pa(SP, 0x10));
    c.lqc2(10, c.pa(SP, 0x20));
    c.lqc2(11, c.pa(SP, 0x30));
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
    c.sqc2(12, c.pa(S2, 0));
    c.sqc2(13, c.pa(S2, 0x10));
    c.sqc2(14, c.pa(S2, 0x20));
    c.sqc2(15, c.pa(S2, 0x30));
    c.stw(S1, 0x6b90, 0u64);
    c.lq(16, c.pa(SP, 0x80));
    c.lq(17, c.pa(SP, 0x70));
    c.lq(18, c.pa(SP, 0x60));
    c.g[RA][0] = c.ldd(SP, 0x50);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = ra;
    return;
}

/// `void BERewardInt_GrantRivalRewards(void* self, int profile, int c, int
/// bit, int mode)` (0x158f60): set a rival-reward bit in the character
/// record's flags (`D_004A6CA8 + profile*0x9b50 + c*0xf88 + 0x278`, bit 13
/// for mode 2, bit 12 for mode 1) if it is clear, then post the reward
/// message (`q = D_004C3EF0`, row `c*3+2` or `c*3+1`).
pub fn be_reward_int_grant_rival_rewards(c: &mut Cpu) {
    'b_159068: {
        'b_159008: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[V0][0] = 0x4c0000;
            c.sq(19, c.pa(SP, 0x10));
            c.sq(16, c.pa(SP, 0x40));
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3ef0));
            c.sq(17, c.pa(SP, 0x30));
            c.g[S0][0] = c.g[A2][0];
            c.sq(18, c.pa(SP, 0x20));
            c.g[S1][0] = c.g[A1][0];
            c.std(SP, 0, c.g[RA][0]);
            c.g[V0][0] = 0x1;
            let t = c.g[T0][0] == c.g[V0][0];
            c.g[S2][0] = c.g[A3][0];
            if t {
                break 'b_159008;
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T0][0] as i64) < 2i64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, c.pa(SP, 0x40));
                    break 'b_159068;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    if c.g[T0][0] != c.g[V0][0] {
                        c.lq(16, c.pa(SP, 0x40));
                        break 'b_159068;
                    } else {
                        c.cycles += 14;
                        c.g[V1][0] = 0xf88;
                        c.g[A0][0] = 0x9b50;
                        c.mult(0, 4, 17, 4);
                        c.mult(0, 3, 16, 3);
                        c.g[V0][0] = 0x4a0000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A0][0] = c.ldd(V1, 0x278);
                        c.g[V0][0] = c.g[A0][0] << 19;
                        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0xffffffffffffdfff;
                        if t {
                            break 'b_159008;
                        } else {
                            c.cycles += 9;
                            c.g[A3][0] = sx((c.g[S0][0] as u32) << 1);
                            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                            c.std(V1, 0x278, c.g[V0][0]);
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2));
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x00159008);
                            c.g[A2][0] = 0x2;
                            c.pc = 0x0015a628;
                            call_indirect(c, 0x0015a628);
                            if c.pc != 0x00159008 {
                                return;
                            }
                            break 'b_159008;
                        }
                    }
                }
            }
        }
        'b_159064: {
            c.cycles += 14;
            c.g[V1][0] = 0xf88;
            c.g[A0][0] = 0x9b50;
            c.mult(0, 4, 17, 4);
            c.mult(0, 3, 16, 3);
            c.g[V0][0] = 0x4a0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A2][0] = c.ldd(V1, 0x278);
            c.g[V0][0] = c.g[A2][0] << 20;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffefff;
            if t {
                break 'b_159064;
            } else {
                c.cycles += 9;
                c.g[A3][0] = sx((c.g[S0][0] as u32) << 1);
                c.g[V0][0] = c.g[A2][0] & c.g[V0][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.std(V1, 0x278, c.g[V0][0]);
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[RA][0] = sx(0x00159064);
                c.g[A2][0] = 0x2;
                c.pc = 0x0015a628;
                call_indirect(c, 0x0015a628);
                if c.pc != 0x00159064 {
                    return;
                }
                break 'b_159064;
            }
        }
        c.cycles += 1;
        c.lq(16, c.pa(SP, 0x40));
        break 'b_159068;
    }
    c.cycles += 6;
    c.lq(17, c.pa(SP, 0x30));
    c.lq(18, c.pa(SP, 0x20));
    c.lq(19, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = ra;
    return;
}

/// `float RiderSpinSub_GetHUDDirAngle(self, int* out)` (0x135f70): the HUD
/// ring direction: 0 when both signs are zero (and `*out = 1`), else the
/// angle of (y, x) from `Math_Atan2`, wrapped into [-pi, pi] (or
/// [-0.5pi, 1.5pi]) depending on the sign combination.
pub fn rider_spin_sub_get_hud_dir_angle(c: &mut Cpu) {
    'b_135fc4: {
        'b_135fc0: {
            'b_135fb8: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.mtc1(0, 12);
                c.std(SP, 0, c.g[RA][0]);
                c.swc1(22, c.pa(SP, 0x20));
                c.swc1(21, c.pa(SP, 0x18));
                c.swc1(20, c.pa(SP, 0x10));
                c.lwc1(0, c.pa(A0, 0x18));
                c.c_le_s(12, 0);
                let t = !c.fcc();

                if t {
                    break 'b_135fb8;
                } else {
                    c.cycles += 4;
                    c.c_lt_s(12, 0);
                    let t = !c.fcc();
                    c.mov_s(21, 12);
                    if t {
                        break 'b_135fc0;
                    } else {
                        c.cycles += 4;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 21);
                        let t = true;
                        c.lwc1(1, c.pa(A0, 0x1c));
                        if t {
                            break 'b_135fc4;
                        } else {
                            break 'b_135fb8;
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[AT][0] = 0xffffffffbf800000;
            c.mtc1(1, 21);
            break 'b_135fc0;
        }
        c.cycles += 1;
        c.lwc1(1, c.pa(A0, 0x1c));
        break 'b_135fc4;
    }
    'b_136000: {
        'b_135ffc: {
            'b_135ff4: {
                c.cycles += 4;
                c.mtc1(0, 0);
                c.c_le_s(0, 1);
                let t = !c.fcc();

                if t {
                    break 'b_135ff4;
                } else {
                    c.cycles += 4;
                    c.c_lt_s(0, 1);
                    let t = !c.fcc();
                    c.mov_s(22, 0);
                    if t {
                        break 'b_135ffc;
                    } else {
                        c.cycles += 4;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 22);
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_136000;
                        } else {
                            break 'b_135ff4;
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[AT][0] = 0xffffffffbf800000;
            c.mtc1(1, 22);
            break 'b_135ffc;
        }
        c.cycles += 1;
        c.g[V0][0] = 0u64;
        break 'b_136000;
    }
    'b_13601c: {
        c.cycles += 3;
        c.c_eq_s(21, 0);
        let t = !c.fcc();

        if t {
            break 'b_13601c;
        } else {
            c.cycles += 4;
            c.c_eq_s(22, 0);
            if c.fcc() {
                c.g[V0][0] = 0x1;
                break 'b_13601c;
            } else {
                break 'b_13601c;
            }
        }
    }
    'b_1360b4: {
        'b_1360b0: {
            'b_136030: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.stw(A1, 0, c.g[V0][0]);
                if t {
                    break 'b_136030;
                } else {
                    c.cycles += 3;
                    c.mtc1(0, 0);
                    let t = true;
                    c.g[RA][0] = c.ldd(SP, 0);
                    if t {
                        break 'b_1360b0;
                    } else {
                        break 'b_136030;
                    }
                }
            }
            'b_1360ac: {
                'b_13606c: {
                    c.cycles += 4;
                    c.mtc1(0, 20);
                    c.c_eq_s(22, 20);
                    let t = !c.fcc();

                    if t {
                        break 'b_13606c;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 20);
                        let t = c.fcc();
                        c.mov_s(0, 21);
                        if t {
                            break 'b_1360ac;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(20, 21);
                            c.lwc1(0, c.pa(GP, -0x7318));
                            let t = !c.fcc();
                            c.g[RA][0] = c.ldd(SP, 0);
                            if t {
                                break 'b_1360ac;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, c.pa(GP, -0x7314));
                                let t = true;
                                c.lwc1(22, c.pa(SP, 0x20));
                                if t {
                                    break 'b_1360b4;
                                } else {
                                    break 'b_13606c;
                                }
                            }
                        }
                    }
                }
                'b_1360a8: {
                    c.cycles += 7;
                    c.div_s(12, 21, 22);
                    c.g[RA][0] = sx(0x00136078);
                    c.pc = 0x0031c228;
                    call_indirect(c, 0x0031c228);
                    if c.pc != 0x00136078 {
                        return;
                    }
                    c.c_lt_s(22, 20);
                    let t = !c.fcc();
                    c.mov_s(12, 0);
                    if t {
                        break 'b_1360a8;
                    } else {
                        'b_1360a0: {
                            c.cycles += 3;
                            c.c_lt_s(20, 21);
                            let t = !c.fcc();

                            if t {
                                break 'b_1360a0;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, c.pa(GP, -0x7310));
                                let t = true;
                                c.add_s(12, 12, 0);
                                if t {
                                    break 'b_1360a8;
                                } else {
                                    break 'b_1360a0;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, c.pa(GP, -0x730c));
                        c.sub_s(12, 12, 0);
                        break 'b_1360a8;
                    }
                }
                c.cycles += 1;
                c.mov_s(0, 12);
                break 'b_1360ac;
            }
            c.cycles += 1;
            c.g[RA][0] = c.ldd(SP, 0);
            break 'b_1360b0;
        }
        c.cycles += 1;
        c.lwc1(22, c.pa(SP, 0x20));
        break 'b_1360b4;
    }
    c.cycles += 4;
    c.lwc1(21, c.pa(SP, 0x18));
    c.lwc1(20, c.pa(SP, 0x10));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = ra;
    return;
}

/// `void TexMan_EmitClutUpload(Ps2UpMgr* self, int idx, ulong** dma, ulong**
/// gif)` (0x3684f0): append the 0xB0-byte texture CLUT-upload packet for
/// `self->texs[idx]` to the GIF stream (`w`/`h` 8x2 or 16x16 depending on
/// the texture PSM), then hand it to the DMA kick builder.
pub fn tex_man_emit_clut_upload(c: &mut Cpu) {
    'b_368554: {
        'b_368548: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[T5][0] = c.g[A0][0];
            c.sq(17, c.pa(SP, 0x30));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
            c.sq(18, c.pa(SP, 0x20));
            c.g[A1][0] = sx((c.g[T5][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.sq(16, c.pa(SP, 0x40));
            c.g[S2][0] = c.g[A2][0];
            c.std(SP, 0x10, c.g[RA][0]);
            c.g[S1][0] = c.g[A3][0];
            c.g[T4][0] = c.ldw(A1, 0x8);
            c.g[V1][0] = c.ldw(S2, 0);
            c.g[V0][0] = c.ldw(T4, 0xc);
            c.stw(SP, 0, c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = c.ldw(S1, 0);
            if t {
                break 'b_368548;
            } else {
                c.cycles += 4;
                c.g[A2][0] = 0x8;
                c.g[A0][0] = 0x2;
                let t = true;
                c.g[T2][0] = 0x10;
                if t {
                    break 'b_368554;
                } else {
                    break 'b_368548;
                }
            }
        }
        c.cycles += 3;
        c.g[A2][0] = 0x10;
        c.g[A0][0] = 0x10;
        c.g[T2][0] = 0x40;
        break 'b_368554;
    }
    c.cycles += 66;
    c.g[T3][0] = 0xe;
    c.g[V0][0] = 0x10000000;
    c.g[V0][0] = c.g[V0][0] | 0x6;
    c.g[V1][0] = 0x10000000;
    c.g[V1][0] = c.g[V1][0] << 32;
    c.g[V1][0] = c.g[V1][0] | 0x4;
    c.std(S0, 0x18, c.g[T3][0]);
    c.std(S0, 0x10, c.g[V1][0]);
    c.g[A0][0] = c.g[A0][0] << 32;
    c.std(S0, 0, c.g[V0][0]);
    c.g[A0][0] = c.g[A2][0] | c.g[A0][0];
    c.std(S0, 0x8, 0u64);
    c.g[A3][0] = 0x8000;
    c.g[A3][0] = c.g[A3][0] << 33;
    c.g[A1][0] = 0x8000;
    c.g[A1][0] = c.g[A1][0] << 44;
    c.g[T0][0] = 0x52;
    c.g[V0][0] = c.ldw(T4, 0x2c);
    c.g[A1][0] = c.g[T2][0] | c.g[A1][0];
    c.g[T1][0] = 0x53;
    c.g[V1][0] = 0x50;
    c.g[V0][0] = c.g[V0][0] << 32;
    c.g[A2][0] = 0x51;
    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
    c.std(S0, 0x28, c.g[V1][0]);
    c.std(S0, 0x38, c.g[A2][0]);
    c.g[V1][0] = 0x30000000;
    c.std(S0, 0x40, c.g[A0][0]);
    c.g[V1][0] = c.g[V1][0] | c.g[T2][0];
    c.std(S0, 0x48, c.g[T0][0]);
    c.g[T2][0] = 0x10000000;
    c.g[T2][0] = c.g[T2][0] | 0x2;
    c.std(S0, 0x58, c.g[T1][0]);
    c.g[A2][0] = 0x10000000;
    c.g[A2][0] = c.g[A2][0] << 32;
    c.g[A2][0] = c.g[A2][0] | 0x8001;
    c.std(S0, 0x60, c.g[A1][0]);
    c.g[A3][0] = 0x3f;
    c.std(S0, 0x20, c.g[V0][0]);
    c.g[A0][0] = c.g[T5][0];
    c.std(S0, 0x30, 0u64);
    c.g[A1][0] = c.g[SP][0];
    c.std(S0, 0x50, 0u64);
    c.std(S0, 0x68, 0u64);
    c.g[V0][0] = c.ldw(T4, 0x20);
    c.std(S0, 0x78, 0u64);
    c.g[V0][0] = c.g[V0][0] << 32;
    c.std(S0, 0x88, 0u64);
    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
    c.std(S0, 0xa0, 0u64);
    c.std(S0, 0x70, c.g[V1][0]);
    c.std(S0, 0x80, c.g[T2][0]);
    c.std(S0, 0x90, c.g[A2][0]);
    c.std(S0, 0x98, c.g[T3][0]);
    c.g[RA][0] = sx(0x00368634);
    c.std(S0, 0xa8, c.g[A3][0]);
    c.pc = 0x00368138;
    call_indirect(c, 0x00368138);
    if c.pc != 0x00368634 {
        return;
    }
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb0));
    c.g[V0][0] = c.ldw(SP, 0);
    c.g[RA][0] = c.ldd(SP, 0x10);
    c.stw(S2, 0, c.g[V0][0]);
    c.stw(S1, 0, c.g[S0][0]);
    c.lq(16, c.pa(SP, 0x40));
    c.lq(17, c.pa(SP, 0x30));
    c.lq(18, c.pa(SP, 0x20));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = ra;
    return;
}

/// `void RiderBoardPressSub_FlipStance(Ai* self)` (0x12fec8): the board-press
/// sub-state's stance flip: toggle the rider's stance bit, build the
/// stance-scaled leg spread from `sin/cos` of the board angle, mirror the
/// rider's two roll axes, and notify the board-press handler.
pub fn rider_board_press_sub_flip_stance(c: &mut Cpu) {
    'b_12ff10: {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.mtc1(0, 0);
        c.sq(19, c.pa(SP, 0x30));
        c.sq(16, c.pa(SP, 0x60));
        c.g[S3][0] = c.g[A0][0];
        c.sq(17, c.pa(SP, 0x50));
        c.sq(18, c.pa(SP, 0x40));
        c.std(SP, 0x20, c.g[RA][0]);
        c.g[V0][0] = c.ldw(S3, 0x14);
        c.g[V1][0] = c.ldw(V0, 0x320);
        c.g[V1][0] = c.g[V1][0] ^ 0x1;
        c.stw(V0, 0x320, c.g[V1][0]);
        c.g[V1][0] = c.ldw(S3, 0x14);
        c.g[V0][0] = c.ldw(V1, 0x320);
        if c.g[V0][0] == 0u64 {
            c.neg_s(0, 0);
            break 'b_12ff10;
        } else {
            c.cycles += 2;
            c.lwc1(0, c.pa(GP, -0x76cc));
            c.neg_s(0, 0);
            break 'b_12ff10;
        }
    }
    'b_12ffb0: {
        'b_12ffa8: {
            c.cycles += 36;
            c.g[V0][0] = 0x500000;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 12);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
            c.g[S2][0] = c.ldw(V1, 0x784);
            c.g[S1][0] = 0x500000;
            c.lq(3, c.pa(V0, 0));
            c.mul_s(12, 0, 12);
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
            c.sq(3, c.pa(S2, 0x30));
            c.g[RA][0] = sx(0x0012ff44);
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xea0));
            c.pc = 0x0031be50;
            call_indirect(c, 0x0031be50);
            if c.pc != 0x0012ff44 {
                return;
            }
            c.lwc1(1, c.pa(S0, 0x8));
            c.g[A1][0] = 0x1;
            c.lwc1(0, c.pa(SP, 0x10));
            c.lwc1(2, c.pa(S1, -0xea0));
            c.lwc1(4, c.pa(S0, 0x4));
            c.mul_s(1, 0, 1);
            c.mul_s(2, 0, 2);
            c.lwc1(3, c.pa(SP, 0x14));
            c.mul_s(0, 0, 4);
            c.swc1(3, c.pa(SP, 0xc));
            c.swc1(1, c.pa(SP, 0x8));
            c.swc1(2, c.pa(SP, 0));
            c.swc1(0, c.pa(SP, 0x4));
            c.lq(2, c.pa(SP, 0));
            c.sq(2, c.pa(S2, 0x40));
            c.g[V1][0] = c.ldw(S3, 0x14);
            c.g[A0][0] = c.ldw(V1, 0x320);
            c.g[V0][0] = c.ldw(V1, 0x784);
            c.stw(V0, 0x18, c.g[A0][0]);
            c.g[V1][0] = c.ldw(S3, 0x14);
            c.g[A0][0] = c.ldw(V1, 0x328);
            let t = c.g[A0][0] != c.g[A1][0];
            c.g[V0][0] = 0x2;
            if t {
                break 'b_12ffa8;
            } else {
                c.cycles += 2;
                let t = true;
                c.stw(V1, 0x328, c.g[V0][0]);
                if t {
                    break 'b_12ffb0;
                } else {
                    break 'b_12ffa8;
                }
            }
        }
        c.cycles += 2;
        if c.g[A0][0] == c.g[V0][0] {
            c.stw(V1, 0x328, c.g[A1][0]);
            break 'b_12ffb0;
        } else {
            break 'b_12ffb0;
        }
    }
    c.cycles += 17;
    c.g[V0][0] = c.ldw(S3, 0x14);
    c.lwc1(0, c.pa(V0, 0x280));
    c.neg_s(0, 0);
    c.swc1(0, c.pa(V0, 0x280));
    c.g[V1][0] = c.ldw(S3, 0x14);
    c.lwc1(0, c.pa(V1, 0x288));
    c.neg_s(0, 0);
    c.swc1(0, c.pa(V1, 0x288));
    c.g[RA][0] = sx(0x0012ffd8);
    c.g[A0][0] = c.ldw(S3, 0x14);
    c.pc = 0x00116930;
    call_indirect(c, 0x00116930);
    if c.pc != 0x0012ffd8 {
        return;
    }
    c.lq(16, c.pa(SP, 0x60));
    c.lq(17, c.pa(SP, 0x50));
    c.lq(18, c.pa(SP, 0x40));
    c.lq(19, c.pa(SP, 0x30));
    c.g[RA][0] = c.ldd(SP, 0x20);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    c.pc = ra;
    return;
}

/// `int SphereTree_CollideTriangle2Sided(void* tree, const Vec4* p, const
/// Vec4* dir, ...)` (0x328030): the two-sided triangle test: `dot` of the
/// direction from the tree's centre (`+0x80`) to `p` against `dir`; when the
/// tree's radius is smaller than |dot| there is no hit, otherwise the
/// collision runs with the direction (or its negation when `dot < 0`).
pub fn sphere_tree_collide_triangle2_sided(c: &mut Cpu) {
    'b_328140: {
        c.cycles += 39;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.sq(17, c.pa(SP, 0x80));
        c.sq(18, c.pa(SP, 0x70));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, c.pa(SP, 0x90));
        c.g[S2][0] = c.g[A1][0];
        c.sq(19, c.pa(SP, 0x60));
        c.g[S0][0] = c.g[T0][0];
        c.sq(20, c.pa(SP, 0x50));
        c.g[S3][0] = c.g[A2][0];
        c.sq(21, c.pa(SP, 0x40));
        c.g[S4][0] = c.g[A3][0];
        c.sq(22, c.pa(SP, 0x30));
        c.g[S5][0] = c.g[T1][0];
        c.std(SP, 0x20, c.g[RA][0]);
        c.g[S6][0] = c.g[T2][0];
        c.swc1(20, c.pa(SP, 0xa0));
        c.lqc2(3, c.pa(S1, 0x80));
        c.lqc2(4, c.pa(S2, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, c.pa(SP, 0x10));
        c.lq(2, c.pa(SP, 0x10));
        c.sq(2, c.pa(SP, 0));
        c.lqc2(3, c.pa(S0, 0));
        c.lqc2(5, c.pa(SP, 0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 20);
        c.g[RA][0] = sx(0x003280b8);
        c.pc = 0x0032c590;
        call_indirect(c, 0x0032c590);
        if c.pc != 0x003280b8 {
            return;
        }
        c.abs_s(1, 20);
        c.c_lt_s(0, 1);
        let t = c.fcc();
        c.g[V0][0] = 0u64;
        if t {
            break 'b_328140;
        } else {
            'b_328138: {
                'b_3280fc: {
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_lt_s(20, 0);
                    let t = c.fcc();
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        break 'b_3280fc;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A3][0] = c.g[S4][0];
                        c.g[T0][0] = c.g[S0][0];
                        c.g[T1][0] = c.g[S5][0];
                        let t = true;
                        c.g[T2][0] = c.g[S6][0];
                        if t {
                            break 'b_328138;
                        } else {
                            break 'b_3280fc;
                        }
                    }
                }
                c.cycles += 15;
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 0);
                c.mfc1(2, 0);
                c.lqc2(4, c.pa(S0, 0));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, c.pa(SP, 0x10));
                c.g[A1][0] = c.g[S2][0];
                c.lq(2, c.pa(SP, 0x10));
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[T1][0] = c.g[S5][0];
                c.sq(2, c.pa(SP, 0));
                c.g[T2][0] = c.g[S6][0];
                c.g[T0][0] = c.g[SP][0];
                break 'b_328138;
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x00328140);
            c.pc = 0x0032ca78;
            call_indirect(c, 0x0032ca78);
            if c.pc != 0x00328140 {
                return;
            }
            break 'b_328140;
        }
    }
    c.cycles += 11;
    c.lq(16, c.pa(SP, 0x90));
    c.lq(17, c.pa(SP, 0x80));
    c.lq(18, c.pa(SP, 0x70));
    c.lq(19, c.pa(SP, 0x60));
    c.lq(20, c.pa(SP, 0x50));
    c.lq(21, c.pa(SP, 0x40));
    c.lq(22, c.pa(SP, 0x30));
    c.g[RA][0] = c.ldd(SP, 0x20);
    c.lwc1(20, c.pa(SP, 0xa0));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
    c.pc = ra;
    return;
}

/// `void* AnimTeeter_ctor(Teeter* self, int a1, void* obj, const Def* def)`
/// (0x3421a0): construct a teeter animation: the base flag-set at `+0x30`,
/// then the rates from the definition (`+4..+0x24`) and the object (`obj +
/// 0x80`), scaled by the frame rate where the original does.
pub fn anim_teeter_ctor(c: &mut Cpu) {
    'b_3421fc: {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[T0][0] = 0u64;
        c.sq(16, c.pa(SP, 0x30));
        c.sq(17, c.pa(SP, 0x20));
        c.g[S0][0] = c.g[A2][0];
        c.sq(18, c.pa(SP, 0x10));
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x30));
        c.g[A3][0] = c.g[S0][0];
        c.std(SP, 0, c.g[RA][0]);
        c.g[RA][0] = sx(0x003421d4);
        c.g[A2][0] = 0x4;
        c.pc = 0x0034d9b0;
        call_indirect(c, 0x0034d9b0);
        if c.pc != 0x003421d4 {
            return;
        }
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = c.ldw(S1, 0x78);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8f8));
        c.mtc1(0, 2);
        c.stw(S1, 0x50, c.g[V0][0]);
        let t = c.g[A0][0] == 0u64;
        c.g[S0][0] = c.ldw(S0, 0x80);
        if t {
            break 'b_3421fc;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003421f8);
            c.pc = 0x00351508;
            call_indirect(c, 0x00351508);
            if c.pc != 0x003421f8 {
                return;
            }
            c.mov_s(2, 0);
            break 'b_3421fc;
        }
    }
    'b_342224: {
        'b_342220: {
            c.cycles += 6;
            c.lwc1(1, c.pa(S2, 0x20));
            c.mtc1(0, 0);
            c.c_le_s(0, 1);
            let t = !c.fcc();
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
            if t {
                break 'b_342220;
            } else {
                c.cycles += 3;
                c.lwc1(0, c.pa(GP, -0x2b6c));
                let t = true;
                c.mul_s(0, 1, 0);
                if t {
                    break 'b_342224;
                } else {
                    break 'b_342220;
                }
            }
        }
        c.cycles += 1;
        c.mov_s(0, 2);
        break 'b_342224;
    }
    'b_342250: {
        'b_342248: {
            c.cycles += 7;
            c.swc1(0, c.pa(V0, 0));
            c.mtc1(0, 0);
            c.lwc1(1, c.pa(S2, 0x24));
            c.c_lt_s(1, 0);
            let t = !c.fcc();
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
            if t {
                break 'b_342248;
            } else {
                c.cycles += 2;
                let t = true;
                c.lwc1(0, c.pa(S0, 0x14));
                if t {
                    break 'b_342250;
                } else {
                    break 'b_342248;
                }
            }
        }
        c.cycles += 2;
        c.lwc1(0, c.pa(GP, -0x2b68));
        c.mul_s(0, 1, 0);
        break 'b_342250;
    }
    c.cycles += 37;
    c.swc1(0, c.pa(V0, 0));
    c.g[V1][0] = c.ldw(GP, 0x2a74);
    c.g[V0][0] = c.g[S1][0];
    c.lwc1(0, c.pa(S2, 0x8));
    c.lwc1(2, c.pa(GP, -0x2b64));
    c.swc1(0, c.pa(S1, 0xc));
    c.lwc1(3, c.pa(GP, -0x2b60));
    c.lwc1(1, c.pa(S2, 0x4));
    c.g[A0][0] = c.ldhu(S1, 0x56);
    c.mul_s(1, 1, 2);
    c.lq(16, c.pa(SP, 0x30));
    c.g[A0][0] = c.g[A0][0] | 0x1;
    c.g[RA][0] = c.ldd(SP, 0);
    c.swc1(1, c.pa(S1, 0x8));
    c.lwc1(0, c.pa(S2, 0xc));
    c.lwc1(1, c.pa(V1, 0x10));
    c.cvt_s_w(1, 1);
    c.mul_s(0, 0, 3);
    c.div_s(0, 0, 1);
    c.swc1(0, c.pa(S1, 0x18));
    c.g[V1][0] = c.ldw(S2, 0x1c);
    c.stw(S1, 0x2c, c.g[V1][0]);
    c.lwc1(0, c.pa(S2, 0x14));
    c.mul_s(0, 0, 3);
    c.swc1(0, c.pa(S1, 0x24));
    c.lwc1(1, c.pa(S2, 0x18));
    c.swc1(1, c.pa(S1, 0x1c));
    c.lwc1(0, c.pa(S2, 0x10));
    c.sth(S1, 0x56, c.g[A0][0]);
    c.swc1(0, c.pa(S1, 0x20));
    c.stw(S1, 0, 0u64);
    c.stw(S1, 0x4, 0u64);
    c.stw(S1, 0x28, 0u64);
    c.lq(17, c.pa(SP, 0x20));
    c.lq(18, c.pa(SP, 0x10));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    return;
}

/// `void MagnetModifier_Update(MagnetMod* self)` (0x3573f8): advance the
/// magnet modifier's pull: move the target by the per-frame step, then add
/// the clamped displacement to the current position (and mark the modifier
/// when the target was reached).
pub fn magnet_modifier_update(c: &mut Cpu) {
    'b_35751c: {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, c.pa(SP, 0x40));
        c.std(SP, 0x30, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.ldw(S0, 0x44);
        if (c.g[V0][0] as i64) < 0 {
            c.lq(16, c.pa(SP, 0x40));
            break 'b_35751c;
        } else {
            'b_357514: {
                'b_357490: {
                    c.cycles += 29;
                    c.g[RA][0] = sx(0x0035741c);
                    c.pc = 0x002d1c70;
                    call_indirect(c, 0x002d1c70);
                    if c.pc != 0x0035741c {
                        return;
                    }
                    c.lwc1(1, c.pa(S0, 0x4c));
                    c.g[A0][0] = c.ldw(S0, 0x44);
                    c.add_s(1, 1, 0);
                    c.g[RA][0] = sx(0x00357430);
                    c.swc1(1, c.pa(S0, 0x4c));
                    c.pc = 0x002d1b58;
                    call_indirect(c, 0x002d1b58);
                    if c.pc != 0x00357430 {
                        return;
                    }
                    c.lqc2(3, c.pa(V0, 0));
                    c.lqc2(4, c.pa(S0, 0x10));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, c.pa(SP, 0x10));
                    c.g[AT][0] = 0x42480000;
                    c.mtc1(1, 0);
                    c.lq(2, c.pa(SP, 0x10));
                    c.sq(2, c.pa(SP, 0));
                    c.lqc2(3, c.pa(SP, 0));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 2);
                    c.c_lt_s(2, 0);
                    let t = !c.fcc();
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_357490;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.stw(S0, 0x54, c.g[V0][0]);
                        if t {
                            break 'b_357514;
                        } else {
                            break 'b_357490;
                        }
                    }
                }
                'b_357500: {
                    c.cycles += 25;
                    c.lwc1(0, c.pa(S0, 0x48));
                    c.lwc1(1, c.pa(S0, 0x4c));
                    c.div_s(0, 0, 2);
                    c.mul_s(0, 0, 1);
                    c.mfc1(3, 0);
                    c.lqc2(4, c.pa(SP, 0));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, c.pa(SP, 0x20));
                    c.lq(2, c.pa(SP, 0x20));
                    c.sq(2, c.pa(SP, 0x10));
                    c.lqc2(3, c.pa(SP, 0x10));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(2, 0);
                    let t = !c.fcc();
                    c.lq(3, c.pa(SP, 0));
                    if t {
                        break 'b_357500;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        c.stw(S0, 0x54, c.g[V0][0]);
                        c.sq(3, c.pa(SP, 0x10));
                        break 'b_357500;
                    }
                }
                c.cycles += 5;
                c.lqc2(3, c.pa(S0, 0x10));
                c.lqc2(4, c.pa(SP, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, c.pa(S0, 0x10));
                c.g[V0][0] = 0x1;
                break 'b_357514;
            }
            c.cycles += 2;
            c.stw(S0, 0x50, c.g[V0][0]);
            c.lq(16, c.pa(SP, 0x40));
            break 'b_35751c;
        }
    }
    c.cycles += 3;
    c.g[RA][0] = c.ldd(SP, 0x30);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = ra;
    return;
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{compare, put_vec, put_words, scramble, Rng, DATA, GAME_GP};

    /// A deterministic float for fixture fields.
    fn edge(seed: u32, k: u32) -> u32 {
        let mut r = Rng(0x328030 ^ seed as u64 ^ (k as u64) << 32);
        r.f32(10.0)
    }

    /// Compare every observable: hard registers, caller-saved temporaries,
    /// LO/HI, cycles, pc and memory.
    fn same(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lohi: {:#?}", r.lohi);
        }
    }

    /// Generic fixture: scramble the register file, point every argument at a
    /// writable object, and fill those objects with small values (counts,
    /// modes) plus a few pointers into the data area at the offsets where
    /// structures usually hold a vtable or a buffer. Also builds the library
    /// singletons the game's callees expect.
    fn generic(c: &mut Cpu, seed: u32, words: u32) {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        globals(c, &mut r);
        for i in 0..8u32 {
            let base = DATA + 0x400 * (i + 1);
            c.g[[A0, A1, A2, A3, T0, T1, T2, T3][i as usize]][0] = base as u64;
            for k in 0..words {
                c.bus.write32(base + 4 * k, r.next_u32() % 8);
            }
            for k in 0..4u32 {
                c.bus
                    .write32(base + 4 * k, DATA + 0x1000 + 0x40 * (i * 4 + k));
            }
        }
    }

    /// The game's singletons in a test machine: the BELibrary dummy, the
    /// world (with the race-manager chain) and the frame-rate structure.
    fn globals(c: &mut Cpu, r: &mut Rng) {
        let world = DATA + 0x9000;
        let game = DATA + 0x9400;
        let race = DATA + 0x9800;
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9c00);
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(GAME_GP - 0x854, DATA + 0x9200);
        c.bus.write32(world + 0x84, game);
        c.bus.write32(game + 0xc, race);
        c.bus.write32(race + 0x7c, 0);
        let hz = DATA + 0x9300;
        c.bus.write32(GAME_GP + 0x2a74, hz);
        c.bus.write32(hz + 0x10, 30 + (r.next_u32() % 120));
        // The audio system pointer some sound code follows.
        c.bus.write32(GAME_GP + 0x608, DATA + 0xa000);
    }

    #[test]
    fn emitter_set_desc_path_coeffs_scaled_matches() {
        same(
            ssx3_ee::s12::f_0036d1f0__generated,
            emitter_set_desc_path_coeffs_scaled,
            200,
            &|c, seed| {
                let mut r = Rng(0x36d1f0 ^ seed as u64);
                scramble(c, &mut r);
                let self_at = DATA + 0x100;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x240) as u64;
                c.g[A3][0] = (DATA + 0x280) as u64;
                c.g[T0][0] = (DATA + 0x2c0) as u64;
                let scale = if seed % 7 == 0 { 0 } else { r.edge_f32() };
                c.bus.write32(self_at + 0x4c, scale);
                for base in [0x200u32, 0x240, 0x280, 0x2c0, 0x80, 0x90, 0xa0] {
                    put_vec(
                        c,
                        DATA + base,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                }
            },
        );
    }

    #[test]
    fn flag_read_params_matches() {
        same(
            ssx3_ee::s11::f_0034ac88__generated,
            flag_read_params,
            200,
            &|c, seed| {
                let mut r = Rng(0x34ac88 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let dst = DATA + 0x100;
                let src = DATA + 0x300;
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                for i in 0..0x68u32 / 4 {
                    c.bus.write32(src + 4 * i, r.next_u32());
                }
                let hz = DATA + 0x500;
                c.bus.write32(GAME_GP + 0x2a74, hz);
                let rate = if seed % 5 == 0 {
                    0
                } else {
                    1 + (r.next_u32() % 240) as i32
                };
                c.bus.write32(hz + 0x10, rate as u32);
            },
        );
    }

    #[test]
    fn c_board_spark_fx_reset_matches() {
        same(
            ssx3_ee::s09::f_002daa78__generated,
            c_board_spark_fx_reset,
            200,
            &|c, seed| {
                let mut r = Rng(0x2daa78 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let d = DATA + 0x100;
                c.g[A0][0] = d as u64;
                for i in 0..0x28u32 / 4 {
                    c.bus.write32(d + 0x80 + 4 * i, r.next_u32());
                }
                put_words(
                    c,
                    0x004f_f130,
                    &[r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                put_words(
                    c,
                    0x0050_1420,
                    &[r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                let texman = DATA + 0x600;
                c.bus.write32(GAME_GP - 0x854, texman);
                c.bus.write32(texman + 0xfa8, r.next_u32());
            },
        );
    }

    #[test]
    fn ps2_graphics_man_emit_depth_fog_clut_upload_matches() {
        same(
            ssx3_ee::s12::f_0036aa60__generated,
            ps2_graphics_man_emit_depth_fog_clut_upload,
            200,
            &|c, seed| {
                let mut r = Rng(0x36aa60 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (r.next_u32() % 4096) as u64;
                c.g[A2][0] = (r.next_u32() % 4096) as u64;
                c.g[A3][0] = (DATA + 0x400) as u64;
                let dst = DATA + 0x1000;
                c.bus.write32(DATA + 0x400, dst);
                for i in 0..0x30 {
                    c.bus.write8(dst + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn menu_sys_build_quad_matches() {
        same(
            ssx3_ee::s09::f_002cb350__generated,
            menu_sys_build_quad,
            200,
            &|c, seed| {
                let mut r = Rng(0x2cb350 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let v = DATA + 0x100;
                c.g[A0][0] = v as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x240) as u64;
                for i in 0..8 {
                    c.bus.write32(DATA + 0x200 + 4 * i, r.f32(1000.0));
                }
                for i in 0..4 {
                    c.bus.write32(DATA + 0x240 + 4 * i, r.f32(1000.0));
                }
                // D_004A289C -> draw module; slot[0x23c] = a short generated leaf
                // (MenuSys_IsActive) called with this = draw + 0x100.
                let draw = DATA + 0x2000;
                c.bus.write32(GAME_GP - 0x854, draw);
                let slot = DATA + 0x4000;
                c.bus.write32(draw + 0x10d8, slot);
                c.bus.write32(slot + 0x238, 0x100);
                c.bus.write32(slot + 0x23c, 0x002c_c098);
                c.bus.write32(draw + 0x100, 0);
            },
        );
    }

    #[test]
    fn gs_pkt_finish_matches() {
        same(ssx3_ee::s12::f_003627a8__generated, gs_pkt_finish, 200, &|c, seed| {
            let mut r = Rng(0x3627a8 ^ seed as u64);
            scramble(c, &mut r);
            let w = DATA + 0x100;
            let pp = DATA + 0x200;
            let base = DATA + 0x800;
            let cur = DATA + 0x900;
            c.g[A0][0] = w as u64;
            c.g[A1][0] = pp as u64;
            c.g[A2][0] = (if seed % 9 == 0 { 3 } else { seed % 4 }) as u64;
            let count = if seed % 7 == 0 {
                0
            } else {
                1 + r.next_u32() % 6
            };
            c.bus.write32(w, count);
            c.bus.write32(w + 4, cur);
            c.bus.write32(w + 8, base);
            c.bus.write32(pp, DATA + 0x1400);
            for i in 0..0x80 {
                c.bus.write8(DATA + 0x1400 + i, r.next_u32() as u8);
            }
        });
    }

    #[test]
    fn ui_anim_advance_matches() {
        same(
            ssx3_ee::s13::f_003974b0__generated,
            ui_anim_advance,
            200,
            &|c, seed| {
                let mut r = Rng(0x3974b0 ^ seed as u64);
                scramble(c, &mut r);
                let anim = DATA + 0x100;
                let data = DATA + 0x200;
                c.g[A0][0] = anim as u64;
                c.g[A1][0] = ((seed % 0x10) | ((seed % 3) << 3)) as u64;
                c.bus.write32(anim, data);
                c.bus.write16(data + 2, 1 + (r.next_u32() % 8) as u16);
                let frame = r.next_u32() % 10;
                c.bus.write16(
                    anim + 0xa,
                    if (seed & 1) == 0 {
                        frame as u16
                    } else {
                        0xffff
                    },
                );
                c.bus.write8(anim + 0xe, (seed % 2) as u8);
            },
        );
    }

    #[test]
    fn rider_board_press_sub_check_release_matches() {
        same(
            ssx3_ee::s00::f_00131200__generated,
            rider_board_press_sub_check_release,
            200,
            &|c, seed| {
                let mut r = Rng(0x131200 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let self_at = DATA + 0x100;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.bus.write32(DATA + 0x300, r.next_u32());
                c.bus.write32(DATA + 0x304, r.next_u32());
                let rider = DATA + 0x200;
                c.bus.write32(self_at + 0x14, rider);
                let timer = if seed % 5 == 0 {
                    0.6f32.to_bits()
                } else {
                    r.f32(1.0)
                };
                c.bus.write32(self_at + 0xc, timer);
                for off in [0x274u32, 0x278, 0x27c, 0x300, 0x330] {
                    c.bus.write32(rider + off, r.next_u32());
                }
                for off in [-0x7614i32, -0x7610, -0x760c, -0x7608, -0x7604] {
                    c.bus
                        .write32(GAME_GP.wrapping_add(off as u32), r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn coll_sphere_overlaps_aabb_matches() {
        same(
            ssx3_ee::s11::f_0032fc08__generated,
            coll_sphere_overlaps_aabb,
            200,
            &|c, seed| {
                let mut r = Rng(0x32fc08 ^ seed as u64);
                scramble(c, &mut r);
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.g[A2][0] = (DATA + 0x240) as u64;
                put_vec(
                    c,
                    s + 0x60,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                c.bus.write32(s + 0x70, r.edge_f32());
                for i in 0..3 {
                    c.bus.write32(DATA + 0x200 + 4 * i, r.edge_f32());
                    c.bus.write32(DATA + 0x240 + 4 * i, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn avalanche_clamp_vector_matches() {
        same(
            ssx3_ee::s09::f_002d64d8__generated,
            avalanche_clamp_vector,
            200,
            &|c, seed| {
                let mut r = Rng(0x2d64d8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let v = DATA + 0x100;
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = v as u64;
                c.bus.write32(GAME_GP + 0x9a4, r.f32(100.0));
                let mut q = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                if seed % 3 == 0 {
                    q[(seed as usize) % 4] = 0x7f80_0000;
                }
                put_vec(c, v, q);
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_apply_background_scale_matches() {
        same(
            ssx3_ee::s03::f_001c7738__generated,
            c_fe_popup_confirm_apply_background_scale,
            200,
            &|c, seed| {
                let mut r = Rng(0x1c7738 ^ seed as u64);
                scramble(c, &mut r);
                let self_at = DATA + 0x100;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + 0x300 + 4 * i, r.next_u32());
                }
                c.bus.write32(self_at + 0x31c, (seed % 4) as u32);
                for (i, off) in [0x48u32, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x68]
                    .iter()
                    .enumerate()
                {
                    let p = if (seed as usize + i) % 3 == 0 {
                        0
                    } else {
                        DATA + 0x400 + 0x40 * i as u32
                    };
                    c.bus.write32(self_at + *off, p);
                    for k in 0..4 {
                        c.bus.write32(p + 0x50 + 4 * k, r.next_u32());
                    }
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_match_blended_matrix_matches() {
        same(
            ssx3_ee::s12::f_00369a78__generated,
            ps2_graphics_man_match_blended_matrix,
            200,
            &|c, seed| {
                let mut r = Rng(0x369a78 ^ seed as u64);
                scramble(c, &mut r);
                let a = DATA + 0x100;
                let b = DATA + 0x140;
                let ka = DATA + 0x200;
                let kb = DATA + 0x240;
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
                c.g[A2][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = (r.next_u32() % 4) as u64;
                c.g[T0][0] = (r.next_u32() % 4) as u64;
                let ty = (seed % 5) as u32;
                c.bus.write32(a, ty);
                c.bus
                    .write32(b, if seed % 4 == 0 { (ty + 1) % 5 } else { ty });
                c.bus.write32(a + 4, ka);
                c.bus.write32(b + 4, kb);
                // Key: id u16 at +2, x/y/z s16 at +0/+4/+8, idA/idB u16 at +6/+0xa.
                for k in [ka, kb] {
                    c.bus.write16(k, (r.next_u32() % 9) as u16);
                    c.bus.write16(k + 2, 1 + (r.next_u32() % 3) as u16);
                    c.bus.write16(k + 4, (r.next_u32() % 9) as u16);
                    c.bus.write16(k + 6, (r.next_u32() % 3) as u16);
                    c.bus.write16(k + 8, (r.next_u32() % 9) as u16);
                    c.bus.write16(k + 0xa, (r.next_u32() % 3) as u16);
                }
                if seed % 2 == 0 {
                    for off in [0u32, 2, 4, 8] {
                        let v = c.bus.read16(ka + off);
                        c.bus.write16(kb + off, v);
                    }
                }
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_fade_gain_matches() {
        same(
            ssx3_ee::s07::f_00287a10__generated,
            ssx_audio_system_set_fade_gain,
            200,
            &|c, seed| {
                let mut r = Rng(0x287a10 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = (seed % 0x400) as u64;
                c.f[12] = if seed % 3 == 0 { 0 } else { r.f32(2.0) };
                c.f[13] = r.f32(1.0);
                for off in [0x63c4u32, 0x63c8, 0x63cc, 0x63d0] {
                    c.bus.write32(s + off, r.edge_f32());
                }
                for off in [
                    -0x4620i32, 0x43c, 0x440, 0x444, 0x448, 0x44c, 0x450, 0x454, 0x458, 0x45c,
                    0x460,
                ] {
                    c.bus
                        .write32(GAME_GP.wrapping_add(off as u32), r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn anim_object_v36_matches() {
        same(
            ssx3_ee::s11::f_00341fe8__generated,
            anim_object_v36,
            200,
            &|c, seed| {
                let mut r = Rng(0x341fe8 ^ seed as u64);
                generic(c, seed, 0x20);
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = match seed % 7 {
                    0 => 0x64,
                    1 => 0x65,
                    2 => 0x66,
                    3 => 0x67,
                    4 => 0x68,
                    5 => 0x69,
                    _ => 0x6a,
                } as u64;
                c.f[12] = r.edge_f32();
                for off in [0u32, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x48] {
                    c.bus.write32(s + off, r.edge_f32());
                }
                // self->+0x48 -> object -> +0x80 -> limiter (+0x14)
                let base = DATA + 0x300;
                let o = DATA + 0x400;
                c.bus.write32(s + 0x48, base);
                c.bus.write32(base + 0x80, o);
                c.bus.write32(o + 0x14, r.f32(5.0));
            },
        );
    }

    #[test]
    fn crowd_audio_instance_remove_source_matches() {
        same(
            ssx3_ee::s08::f_002a62f0__generated,
            crowd_audio_instance_remove_source,
            200,
            &|c, seed| {
                let mut r = Rng(0x2a62f0 ^ seed as u64);
                generic(c, seed, 0x60);
                let s = DATA + 0x100;
                let idx = (seed % 4) as u64;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = if seed % 8 == 0 { 8 } else { idx };
                c.f[12] = if seed % 2 == 0 { 0 } else { r.f32(1.0) };
                for off in [0xd4u32, 0x170, 0x174, 0x178, 0x180, 0x188, 0x18c] {
                    c.bus.write32(s + off, (r.next_u32() % 4) as u32);
                }
                c.bus.write32(s + 0x188, r.f32(1.0));
                let ent = s + 0x20 + (idx as u32) * 0xc;
                c.bus.write32(ent, if seed % 9 == 0 { 0 } else { 1 });
                let voice = DATA + 0x500 + (seed % 4) * 0x100;
                c.bus.write32(ent + 4, voice);
                c.bus
                    .write32(voice + 8, if seed % 3 == 0 { 0 } else { 1 + seed % 3 });
                c.bus.write32(voice + 4, (seed % 4) as u32);
                c.bus.write32(voice, 0);
                // The audio system singleton.
                let sys = DATA + 0x740;
                c.bus.write32(GAME_GP + 0x608, sys);
                c.bus.write32(sys, DATA + 0x780);
                for off in [0u32, 4, 8, 0xc, 0x10] {
                    c.bus.write32(DATA + 0x780 + off, r.next_u32() % 8);
                }
            },
        );
    }

    #[test]
    fn line_strip_finish_matches() {
        same(
            ssx3_ee::s12::f_00384e50__generated,
            line_strip_finish,
            200,
            &|c, seed| {
                let mut r = Rng(0x384e50 ^ seed as u64);
                generic(c, seed, 0x20);
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(s + 4, DATA + 0x1000);
                c.bus.write32(s + 8, DATA + 0x1400);
                c.bus.write32(s + 0xc, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(s + 0, DATA + 0x1800);
                let m = DATA + 0x1c00;
                c.bus.write32(GAME_GP + 0x2a90, m);
                for off in [0u32, 0x8, 0xe84, 0x18f0, 0x1cc0, 0x1cc4] {
                    c.bus.write32(m + off, (r.next_u32() % 64) as u32);
                }
                for i in 0..0x40u32 {
                    c.bus.write32(DATA + 0x1000 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn tex_flip_modifier_update_matches() {
        same(
            ssx3_ee::s12::f_0035f410__generated,
            tex_flip_modifier_update,
            200,
            &|c, seed| {
                let mut r = Rng(0x35f410 ^ seed as u64);
                generic(c, seed, 0x20);
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                for off in [0u32, 4, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20] {
                    c.bus.write32(s + off, (r.next_u32() % 8) as u32);
                }
                c.bus.write32(s + 8, r.f32(1.0));
                let hz = DATA + 0x500;
                c.bus.write32(GAME_GP + 0x2a74, hz);
                c.bus.write32(hz + 0x10, 30 + (r.next_u32() % 120));
            },
        );
    }

    #[test]
    fn be_new_player_int_get_peak_flag_b_matches() {
        same(
            ssx3_ee::s01::f_001464d0__generated,
            be_new_player_int_get_peak_flag_b,
            200,
            &|c, seed| {
                let mut r = Rng(0x1464d0 ^ seed as u64);
                generic(c, seed, 0x20);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                c.g[T0][0] = ((seed / 4) % 2) as u64;
                // Profile table: index -> profile.
                for i in 0..4u32 {
                    c.bus.write32(0x0053_05b0 + 4 * i, (i % 4) as u32);
                }
                for profile in 0..4u32 {
                    for ch in 0..3u32 {
                        let p = 0x004a_6ca8 + profile * 0x9b50 + ch * 0xf88 + 0x278;
                        c.bus.write32(p, r.next_u32());
                        c.bus.write32(p + 4, r.next_u32());
                    }
                }
            },
        );
    }

    #[test]
    fn crowd_audio_push_play_params_matches() {
        same(
            ssx3_ee::s08::f_002a7468__generated,
            crowd_audio_push_play_params,
            200,
            &|c, seed| {
                let mut r = Rng(0x2a7468 ^ seed as u64);
                generic(c, seed, 0x20);
                let self_at = DATA + 0x100;
                let s = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at, s);
                for i in 0..0x100u32 / 4 {
                    c.bus.write32(s + 0x1d8 + 4 * i, r.next_u32() % 4);
                }
                c.bus.write32(s + 0x1f4, (seed % 16) as u32);
                // Ring buffers the template points at.
                for k in 0..8u32 {
                    c.bus.write32(s + 0x1f8 + 4 * k, DATA + 0x4000 + 0x200 * k);
                    c.bus.write32(s + 0x218 + 4 * k, DATA + 0x4400 + 0x200 * k);
                }
                for (i, off) in [
                    0x238u32, 0x23c, 0x240, 0x244, 0x248, 0x24c, 0x250, 0x254, 0x258, 0x25c,
                ]
                .iter()
                .enumerate()
                {
                    c.bus.write32(s + *off, DATA + 0x5000 + 0x200 * i as u32);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_rotate_axis_matches() {
        same(
            ssx3_ee::s13::f_00395d60__generated,
            ps2_graphics_man_rotate_axis,
            200,
            &|c, seed| {
                let mut r = Rng(0x395d60 ^ seed as u64);
                scramble(c, &mut r);
                let m = DATA + 0x100;
                let man = DATA + 0x2000;
                c.g[A0][0] = man as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.bus.write32(man + 0x13e4, m);
                put_vec(
                    c,
                    m,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    m + 0x10,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    m + 0x20,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    m + 0x30,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                for i in 0..4 {
                    c.bus.write32(DATA + 0x300 + 4 * i, r.edge_f32());
                }
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn be_reward_int_grant_rival_rewards_matches() {
        same(
            ssx3_ee::s01::f_00158f60__generated,
            be_reward_int_grant_rival_rewards,
            200,
            &|c, seed| {
                let mut r = Rng(0x158f60 ^ seed as u64);
                generic(c, seed, 0x10);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = (seed % 2) as u64;
                c.g[T0][0] = (seed % 5) as u64;
                for profile in 0..4u32 {
                    for ch in 0..3u32 {
                        let p = 0x004a_6ca8 + profile * 0x9b50 + ch * 0xf88 + 0x278;
                        c.bus.write32(p, r.next_u32());
                        c.bus.write32(p + 4, r.next_u32());
                    }
                }
            },
        );
    }

    #[test]
    fn rider_spin_sub_get_hud_dir_angle_matches() {
        same(
            ssx3_ee::s01::f_00135f70__generated,
            rider_spin_sub_get_hud_dir_angle,
            200,
            &|c, seed| {
                let mut r = Rng(0x135f70 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.bus
                    .write32(DATA + 0x118, if seed % 4 == 0 { 0 } else { r.f32(1.0) });
                c.bus
                    .write32(DATA + 0x11c, if seed % 3 == 0 { 0 } else { r.f32(1.0) });
                for off in [-0x7318i32, -0x7314, -0x7310, -0x730c] {
                    c.bus
                        .write32(GAME_GP.wrapping_add(off as u32), r.edge_f32());
                }
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn tex_man_emit_clut_upload_matches() {
        same(
            ssx3_ee::s12::f_003684f0__generated,
            tex_man_emit_clut_upload,
            200,
            &|c, seed| {
                let mut r = Rng(0x3684f0 ^ seed as u64);
                scramble(c, &mut r);
                let mgr = DATA + 0x100;
                let tex = DATA + 0x800;
                c.g[A0][0] = mgr as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = (DATA + 0x300) as u64;
                c.g[A3][0] = (DATA + 0x340) as u64;
                c.bus.write32(mgr + 8 + 4 * (seed % 2), tex);
                c.bus
                    .write32(tex + 0xc, if seed % 3 == 0 { 0xc } else { 0x13 });
                c.bus.write32(tex + 0x20, r.next_u32() % 0x1000);
                c.bus.write32(tex + 0x2c, r.next_u32() % 0x1000);
                let dma = DATA + 0x400;
                let gif = DATA + 0x1000;
                c.bus.write32(DATA + 0x300, dma);
                c.bus.write32(DATA + 0x340, gif);
                for i in 0..0x30u32 {
                    c.bus.write8(gif + i, r.next_u32() as u8);
                }
                for i in 0..0x40u32 {
                    c.bus.write8(dma + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn rider_board_press_sub_flip_stance_matches() {
        same(
            ssx3_ee::s00::f_0012fec8__generated,
            rider_board_press_sub_flip_stance,
            200,
            &|c, seed| {
                let mut r = Rng(0x12fec8 ^ seed as u64);
                scramble(c, &mut r);
                generic(c, seed, 0x20);
                let self_at = DATA + 0x100;
                let rider = DATA + 0x200;
                let anim = DATA + 0x300;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x14, rider);
                c.bus.write32(rider + 0x320, (seed % 2) as u32);
                c.bus.write32(rider + 0x328, (seed % 4) as u32);
                c.bus.write32(rider + 0x784, anim);
                c.bus.write32(rider + 0x280, r.edge_f32());
                c.bus.write32(rider + 0x288, r.edge_f32());
                put_vec(
                    c,
                    anim + 0x30,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    anim + 0x40,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_words(
                    c,
                    0x004f_f160,
                    &[r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                for off in [0u32, 4, 8, 0xc] {
                    c.bus.write32(0x004f_f130 + off, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn sphere_tree_collide_triangle2_sided_matches() {
        same(
            ssx3_ee::s10::f_00328030__generated,
            sphere_tree_collide_triangle2_sided,
            200,
            &|c, seed| {
                generic(c, seed, 0x40);
                let mut r = crate::difftest::Rng((seed as u64) ^ 0x9e37_79b9);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[T0][0] = (DATA + 0x900) as u64;
                put_vec(
                    c,
                    DATA + 0x180,
                    [edge(seed, 1), edge(seed, 2), edge(seed, 3), 0],
                );
            },
        );
    }

        #[test]
    fn magnet_modifier_update_matches() {
        same(
            ssx3_ee::s11::f_003573f8__generated,
            magnet_modifier_update,
            200,
            &|c, seed| {
                let mut r = Rng(0x3573f8 ^ seed as u64);
                scramble(c, &mut r);
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.bus.write32(s + 0x44, (seed % 8) as u32);
                c.bus.write32(s + 0x48, r.edge_f32());
                c.bus.write32(s + 0x4c, r.edge_f32());
                c.bus.write32(s + 0x50, (seed % 2) as u32);
                c.bus.write32(s + 0x54, (seed % 2) as u32);
                put_vec(
                    c,
                    s + 0x10,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
            },
        );
    }
}
