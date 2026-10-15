//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Hand-written, typed replacements for small leaf functions from the PS2
//! graphics manager, the emitter/deferred-delete helpers and the UI library.
//! Each replacement reproduces the generated body's cycle charging, every
//! observable register (including scratch v0/v1/a0) and FPU/memory effects.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0036_91b0, tex_man_set_tex_function, "TexMan_SetTexFunction"),
    (0x0036_9890, ps2_graphics_man_set_shape_banks, "PS2GraphicsMan_SetShapeBanks"),
    (0x0036_a1c0, deferred_delete_push, "DeferredDelete_Push"),
    (0x0036_a3b0, morton_spread8, "Morton_Spread8"),
    (0x0036_aba0, ps2_graphics_man_reset_depth_fog_params, "PS2GraphicsMan_ResetDepthFogParams"),
    (0x0036_d3d8, emitter_set_desc_time, "Emitter_SetDescTime"),
    (0x0036_d3e8, emitter_advance_desc_time, "Emitter_AdvanceDescTime"),
    (0x0037_2b30, bezier_patch_cache_ctor, "BezierPatchCache_ctor"),
    (0x0037_4ce0, boader_shadow_set_params, "BoaderShadow_SetParams"),
    (0x0037_58f8, boader_shadow_ctor, "BoaderShadow_ctor"),
    (0x0037_59e8, detail_system_ctor, "DetailSystem_ctor"),
    (0x0038_4d98, line_strip_init, "LineStrip_Init"),
    (0x0038_4dc0, line_strip_close, "LineStrip_Close"),
    (0x0038_4fd0, line_strip_add_vertex, "LineStrip_AddVertex"),
    (0x0038_5a38, cps2_grid_mesh_pack_position, "cPS2GridMesh_PackPosition"),
    (0x0038_5ba0, cps2_grid_mesh_pack_uv, "cPS2GridMesh_PackUV"),
    (0x0038_a530, irradiance_spot_light_vector, "Irradiance_SpotLightVector"),
    (0x0038_a618, irradiance_point_light_vector, "Irradiance_PointLightVector"),
    (0x0038_ae10, light_man_ctor, "LightMan_ctor"),
    (0x0038_e0a8, render2d_transform_point, "Render2D_TransformPoint"),
    (0x0039_0458, ps2_graphics_man_reset_color_filter, "PS2GraphicsMan_ResetColorFilter"),
    (0x0039_0ec8, vfx_screen_tint_trigger_flash, "VfxScreenTint_TriggerFlash"),
    (0x0039_0ef8, vfx_screen_tint_poll_flash_sound, "VfxScreenTint_PollFlashSound"),
    (0x0039_7278, uianim_get_frame_count_safe, "UIAnim_GetFrameCountSafe"),
    (0x0039_7440, uianim_is_done, "UIAnim_IsDone"),
    (0x0039_7458, uianim_get_frame_count, "UIAnim_GetFrameCount"),
    (0x0039_7468, uianim_set_play_mode, "UIAnim_SetPlayMode"),
    (0x0039_7858, uilist_last, "UIList_Last"),
    (0x0039_78f0, uilist_push_front, "UIList_PushFront"),
    (0x0039_7910, uilist_push_back, "UIList_PushBack"),
    (0x0039_7930, uilist_reset, "UIList_Reset"),
    (0x0039_84b0, uifont_set_add_font, "UIFontSet_AddFont"),
    (0x0039_a708, uilistbox_get_selected_data, "UIListBox_GetSelectedData"),
    (0x0039_a738, uilistbox_get_selected_value, "UIListBox_GetSelectedValue"),
    (0x0039_a7a8, uilistbox_set_selection, "UIListBox_SetSelection"),
    (0x0039_ab00, uimenu_set_nav_buttons, "UIMenu_SetNavButtons"),
    (0x0039_cbc8, uiscreen_cmd_stop_thread, "UIScreen_CmdStopThread"),
    (0x0039_ce20, uiscreen_cmd_end_transition_in, "UIScreen_CmdEndTransitionIn"),
    (0x0039_ce98, uiscreen_cmd_end_transition_out, "UIScreen_CmdEndTransitionOut"),
    (0x0039_feb8, uiobject_apply_alignment, "UIObject_ApplyAlignment"),
    (0x0039_ff60, uiobject_find_by_id, "UIObject_FindById"),
    (0x0039_ff78, uiobject_get_event_thread, "UIObject_GetEventThread"),
    (0x003a_0318, uiobject_get_user_data, "UIObject_GetUserData"),
    (0x003a_0330, uiobject_set_user_data, "UIObject_SetUserData"),
    (0x003a_04f0, uitext_get_font, "UIText_GetFont"),
    (0x003a_12d0, c_ui_text_scroll_get_font, "cUITextScroll_GetFont"),
    (0x003a_1310, c_ui_text_scroll_set_nav_buttons, "cUITextScroll_SetNavButtons"),
    (0x003a_2068, c_ui_text_scroll_get_line, "cUITextScroll_GetLine"),
    (0x003a_7058, cstream_man_set_rate, "cStreamMan_SetRate"),
    (0x003a_7768, world_block_pool_alloc_block, "WorldBlockPool_AllocBlock"),
];

// ---------------------------------------------------------------------------
// Texture manager

/// `void TexMan_SetTexFunction(TexMan *man, int index, uint mode)` (0x3691b0):
/// if `index` is not -1, replace bits 32..34 of the function pointer stored at
/// `man + index*4 + 8`'s `+0x38` with `mode & 3`.
pub fn tex_man_set_tex_function(c: &mut Cpu) {
    c.cycles += 3;
    let index = c.g[A1][0];
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let is_current = index == 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = sx((index as u32) << 2);
    if !is_current {
        c.cycles += 13;
        c.g[V1][0] = c.g[A2][0] & 0x3;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let mut mask = 0xffff_ffff_ffff_ffe7u64;
        mask <<= 16;
        mask |= 0xffff;
        mask <<= 16;
        mask |= 0xffff;
        c.g[A1][0] = mask;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = c.g[V1][0] << 35;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x38).ld(c);
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        Addr(c.g[A0][0] as u32).off(0x38).sd(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// PS2 graphics manager

/// `void PS2GraphicsMan_SetShapeBanks(PS2GraphicsMan *man, uint a, uint b)`
/// (0x369890): store the two banks and clear the count.
pub fn ps2_graphics_man_set_shape_banks(c: &mut Cpu) {
    c.cycles += 4;
    let man = Addr(abi::arg(c, 0));
    man.off(0x27c).sw(c, c.g[A2][0]);
    man.off(0x278).sw(c, c.g[A1][0]);
    man.off(0xe80).sw(c, 0);
    abi::ret(c);
}

/// `void DeferredDelete_Push(void *obj)` (0x36a1c0): append `obj` to the
/// deferred-delete ring at 0x53ab40 indexed by the count at `gp + 0x2a98`,
/// linking it to the previous head.
pub fn deferred_delete_push(c: &mut Cpu) {
    c.cycles += 9;
    const RING: u32 = 0x54_0000 - 0x54c0;
    let count = Addr((c.g[GP][0] as u32).wrapping_add(0x2a98)).lw(c);
    c.g[A1][0] = count;
    c.g[V0][0] = 0x54_0000;
    c.g[V0][0] = sx(0x54_0000u32.wrapping_sub(0x54c0));
    c.g[A1][0] = sx((count as u32) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let slot = c.g[A1][0] as u32;
    c.g[V1][0] = Addr(slot).lw(c);
    Addr(abi::arg(c, 0)).sw(c, c.g[V1][0]);
    Addr(slot).sw(c, c.g[A0][0]);
    debug_assert_eq!(slot, RING.wrapping_add((count as u32).wrapping_shl(2)));
    abi::ret(c);
}

/// `uint Morton_Spread8(uint x)` (0x36a3b0): spread the low 8 bits of `x` into
/// every other bit.
pub fn morton_spread8(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = c.g[A0][0] & 0xff;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] & 0xf0f;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] & 0x3333;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] & 0x5555;
    abi::ret(c);
}

/// `void PS2GraphicsMan_ResetDepthFogParams(PS2GraphicsMan *man, uint index)`
/// (0x36aba0): fill the 0x20-byte depth-fog record for `index` from the
/// `gp + 0x123c` defaults, copying the 0x10-byte table entry at 0x547230 in
/// after the first four words.
pub fn ps2_graphics_man_reset_depth_fog_params(c: &mut Cpu) {
    c.cycles += 23;
    let gp = c.g[GP][0] as u32;
    let f1 = Addr(gp.wrapping_add(0x123c)).read_fpu(c);
    c.f[1] = f1.0;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 5);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c64));
    c.g[V1][0] = 0x50_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let dst = c.g[A0][0] as u32;
    Addr(dst).write_fpu(c, f1);
    let f0 = Addr(gp.wrapping_add(0x1240)).read_fpu(c);
    c.f[0] = f0.0;
    Addr(dst).off(0x4).write_fpu(c, f0);
    c.g[V0][0] = Addr(gp.wrapping_add(0x1244)).lw(c);
    Addr(dst).off(0x8).sw(c, c.g[V0][0]);
    let f0 = Addr(gp.wrapping_add(0x1248)).read_fpu(c);
    c.f[0] = f0.0;
    Addr(dst).off(0xc).write_fpu(c, f0);
    c.g[A3][0] = sx(0x50_0000u32.wrapping_add(0x4720));
    let src = c.g[A3][0] as u32;
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.ldl(5, src.wrapping_add(0xf));
    c.ldr(5, src.wrapping_add(8));
    c.sdl(2, dst.wrapping_add(0x17));
    c.sdr(2, dst.wrapping_add(0x10));
    c.sdl(5, dst.wrapping_add(0x1f));
    c.sdr(5, dst.wrapping_add(0x18));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Emitters

/// `void Emitter_SetDescTime(Emitter *e, float time)` (0x36d3d8): set the
/// current description time to `time * rate` (rate at +0x4c).
pub fn emitter_set_desc_time(c: &mut Cpu) {
    c.cycles += 4;
    let emitter = Addr(abi::arg(c, 0));
    let rate = emitter.off(0x4c).read_fpu(c);
    c.f[0] = rate.0;
    let result = abi::farg(c, 0) * rate;
    c.f[12] = result.0;
    emitter.off(0x8).write_fpu(c, result);
    abi::ret(c);
}

/// `void Emitter_AdvanceDescTime(Emitter *e, float dt)` (0x36d3e8): advance
/// the description time at +0x8 by `dt * rate` (rate at +0x4c).
pub fn emitter_advance_desc_time(c: &mut Cpu) {
    c.cycles += 6;
    let emitter = Addr(abi::arg(c, 0));
    let rate = emitter.off(0x4c).read_fpu(c);
    c.f[1] = rate.0;
    let current = emitter.off(0x8).read_fpu(c);
    c.f[0] = current.0;
    let delta = abi::farg(c, 0) * rate;
    c.f[12] = delta.0;
    let result = current + delta;
    c.f[0] = result.0;
    emitter.off(0x8).write_fpu(c, result);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bezier cache / border shadow

/// `BezierPatchCache_ctor(BezierPatchCache *self)` (0x372b30): clear the
/// three-word list at +0xc.
pub fn bezier_patch_cache_ctor(c: &mut Cpu) {
    c.cycles += 17;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    let this = Addr(abi::arg(c, 0));
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    c.g[V0][0] = c.g[A0][0];
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `void BoaderShadow_SetParams(BoaderShadow *s, uint a, uint b, uint c,
/// uint d, uint e)` (0x374ce0): store the five words at +0x190 (first) and
/// +0x17c..+0x18c (the rest, in order).
pub fn boader_shadow_set_params(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    this.off(0x190).sw(c, c.g[T2][0]);
    this.off(0x17c).sw(c, c.g[A1][0]);
    this.off(0x180).sw(c, c.g[A2][0]);
    this.off(0x184).sw(c, c.g[A3][0]);
    this.off(0x188).sw(c, c.g[T0][0]);
    this.off(0x18c).sw(c, c.g[T1][0]);
    abi::ret(c);
}

/// `BoaderShadow_ctor(BoaderShadow *s)` (0x3758f8): initialise the shadow
/// (count -1, extent 0x80) and leave `self` in v0.
pub fn boader_shadow_ctor(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = 0x80;
    let this = Addr(abi::arg(c, 0));
    this.off(0x170).sw(c, c.g[V0][0]);
    this.off(0x178).sw(c, c.g[V1][0]);
    c.g[V0][0] = c.g[A0][0];
    this.off(0x174).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Detail system

/// `DetailSystem_ctor(DetailSystem *s)` (0x3759e8): install the vtable at
/// 0x493000.
pub fn detail_system_ctor(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x49_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(0x49_0000u32.wrapping_add(0x3000));
    Addr(abi::arg(c, 0)).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Line strips

/// `void LineStrip_Init(LineStrip *s, uint *buffer)` (0x384d98): store the
/// buffer pointer, its first word tagged with 0x30000000, and clear the counts.
pub fn line_strip_init(c: &mut Cpu) {
    c.cycles += 9;
    let this = Addr(abi::arg(c, 0));
    let buffer = abi::arg64(c, 1);
    this.sw(c, buffer);
    c.g[A2][0] = 0x3000_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = Addr(buffer as u32).lw(c);
    this.off(0x8).sw(c, 0);
    c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
    this.off(0xc).sw(c, 0);
    this.off(0x4).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void LineStrip_Close(LineStrip *s)` (0x384dc0): optionally append a
/// closing vertex, then write the 0x30-byte primitive header into the buffer.
pub fn line_strip_close(c: &mut Cpu) {
    c.cycles += 5;
    let t0 = c.g[A0][0] as u32;
    c.g[T0][0] = c.g[A0][0];
    c.g[V0][0] = Addr(t0).off(0x10).lw(c);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    if c.g[V0][0] == 0 {
        c.g[V1][0] = Addr(t0).off(0xc).lw(c);
    } else {
        c.cycles += 5;
        c.g[V0][0] = Addr(t0).off(0x4).lw(c);
        c.sd(c.g[V0][0] as u32, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
        Addr(t0).off(0x4).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(t0).off(0xc).lw(c);
    }
    c.cycles += 25;
    c.g[V0][0] = 0x1400_0000;
    c.g[V0][0] = c.g[V0][0] << 32;
    c.g[V0][0] = c.g[V0][0] | 0x1;
    c.g[A1][0] = 0x4a;
    c.g[A2][0] = 0xd000;
    c.g[A2][0] = c.g[A2][0] << 46;
    c.sd(c.g[V1][0] as u32, c.g[V0][0]);
    c.g[A3][0] = 0x521;
    c.g[V0][0] = Addr(t0).off(0xc).lw(c);
    c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0);
    c.g[V1][0] = Addr(t0).off(0xc).lw(c);
    c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
    c.g[A0][0] = Addr(t0).off(0xc).lw(c);
    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), 0);
    c.g[V0][0] = Addr(t0).off(0x10).lw(c);
    c.g[A0][0] = Addr(t0).off(0xc).lw(c);
    c.g[V0][0] = c.g[V0][0] | 0x8000;
    c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
    c.sd((c.g[A0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
    c.g[V1][0] = Addr(t0).off(0xc).lw(c);
    c.sd((c.g[V1][0] as u32).wrapping_add(0x28), c.g[A3][0]);
    c.g[V0][0] = Addr(t0).off(0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
    Addr(t0).off(0xc).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void LineStrip_AddVertex(LineStrip *s, void *vtx, void *colour)`
/// (0x384fd0): pack the vertex fields (position, colour, UV) into the buffer,
/// scaling by the colour at `colour + 0xc`.
pub fn line_strip_add_vertex(c: &mut Cpu) {
    c.cycles += 53;
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    c.g[V1][0] = Addr(a1).off(0x14).lwu(c);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = Addr(a1).off(0x10).lwu(c);
    c.g[A3][0] = Addr(a1).off(0x18).lwu(c);
    c.g[V1][0] = c.g[V1][0] << 8;
    c.g[T0][0] = Addr(a1).off(0x1c).lwu(c);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V1][0] = Addr(a2).off(0xc).lw(c);
    c.g[A3][0] = c.g[A3][0] << 16;
    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
    c.g[T0][0] = c.g[T0][0] << 24;
    c.g[A3][0] = Addr(a0).off(0x4).lw(c);
    c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
    c.g[V1][0] = c.g[V1][0] << 32;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd(c.g[A3][0] as u32, c.g[V0][0]);
    let colour = Addr(a2).off(0xc).read_fpu(c);
    c.f[0] = colour.0;
    let x = Addr(a1).read_fpu(c);
    c.f[2] = x.0;
    let y = Addr(a1).off(0x4).read_fpu(c);
    c.f[1] = y.0;
    let scaled_x = x * colour;
    c.f[2] = scaled_x.0;
    c.g[A3][0] = Addr(a0).off(0x4).lw(c);
    let scaled_y = y * colour;
    c.f[1] = scaled_y.0;
    Addr(c.g[SP][0] as u32).write_fpu(c, scaled_x);
    c.g[A1][0] = abi::mfc1(scaled_y);
    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
    c.g[V0][0] = c.g[A1][0] << 32;
    c.g[V0][0] = c.g[V1][0] | c.g[V0][0];
    c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    let colour = Addr(a2).off(0xc).read_fpu(c);
    c.f[1] = colour.0;
    let w = Addr(a2).off(0x8).read_fpu(c);
    c.f[0] = w.0;
    let z = Addr(a2).off(0x4).read_fpu(c);
    c.f[2] = z.0;
    let raw_w = Addr(a2).read_fpu(c);
    c.f[3] = raw_w.0;
    let scaled_w = w * colour;
    c.f[0] = scaled_w.0;
    let zi = z.to_i32();
    c.f[1] = zi as u32;
    c.g[V0][0] = sx(zi as u32);
    c.g[A2][0] = Addr(a0).off(0x4).lw(c);
    let wi = raw_w.to_i32();
    c.f[1] = wi as u32;
    c.g[A1][0] = sx(wi as u32);
    let packed = scaled_w.to_i32();
    c.f[1] = packed as u32;
    c.g[V1][0] = sx(packed as u32);
    c.g[V0][0] = c.g[V0][0] << 16;
    c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
    c.g[V1][0] = c.g[V1][0] << 32;
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    c.g[V1][0] = Addr(a0).off(0x4).lw(c);
    c.g[V0][0] = Addr(a0).off(0x10).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    Addr(a0).off(0x4).sw(c, c.g[V1][0]);
    Addr(a0).off(0x10).sw(c, c.g[V0][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Grid mesh packing

/// `void cPS2GridMesh_PackPosition(cPS2GridMesh *m, int base, int x, int y,
/// const float *pos)` (0x385a38): scale the three floats at `pos` by the
/// matrix scale at `this + 0x198` and store the three 16-bit results at
/// `base + (x*3<<1) + y*2`... (`base + x*6 + base_offset`).
pub fn cps2_grid_mesh_pack_position(c: &mut Cpu) {
    c.cycles += 26;
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0] as u32;
    let scale = Addr(a0).off(0x198).read_fpu(c);
    c.f[3] = scale.0;
    c.g[V0][0] = sx(a2 << 1);
    let z = Addr(a3).off(0x8).read_fpu(c);
    c.f[0] = z.0;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(a2));
    let x = Addr(a3).read_fpu(c);
    c.f[1] = x.0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    let y = Addr(a3).off(0x4).read_fpu(c);
    c.f[2] = y.0;
    let zs = z * scale;
    c.f[0] = zs.0;
    let xs = x * scale;
    c.f[1] = xs.0;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(a1));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    let ys = y * scale;
    c.f[2] = ys.0;
    let sp = c.g[SP][0] as u32;
    Addr(sp).off(0x8).write_fpu(c, zs);
    let zi = zs.to_i32();
    c.f[3] = zi as u32;
    c.g[A1][0] = sx(zi as u32);
    let xi = xs.to_i32();
    c.f[0] = xi as u32;
    c.g[A0][0] = sx(xi as u32);
    Addr(sp).write_fpu(c, xs);
    let yi = ys.to_i32();
    c.f[0] = yi as u32;
    c.g[V1][0] = sx(yi as u32);
    Addr(sp).off(0x4).write_fpu(c, ys);
    let base = c.g[V0][0] as u32;
    Addr(base).off(0x4).sh(c, c.g[A1][0]);
    Addr(base).sh(c, c.g[A0][0]);
    Addr(base).off(0x2).sh(c, c.g[V1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

/// `void cPS2GridMesh_PackUV(uint *dst, int base, int index, const float *uv)`
/// (0x385ba0): scale the two floats at `uv` by 4096 and store them as 16-bit
/// at `base + index*4`.
pub fn cps2_grid_mesh_pack_uv(c: &mut Cpu) {
    c.cycles += 15;
    c.g[AT][0] = 0x4580_0000;
    c.f[1] = c.g[AT][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0] as u32;
    let dst = a1.wrapping_add(a2.wrapping_shl(2));
    c.g[A2][0] = sx((a2 << 2).wrapping_add(a1));
    let u = Addr(a3).read_fpu(c);
    let scaled_u = u * FpuF32(0x4580_0000);
    c.f[0] = scaled_u.0;
    let ui = scaled_u.to_i32();
    c.f[2] = ui as u32;
    c.g[V0][0] = sx(ui as u32);
    Addr(dst).sh(c, c.g[V0][0]);
    let v = Addr(a3).off(0x4).read_fpu(c);
    let scaled_v = v * FpuF32(0x4580_0000);
    c.f[0] = scaled_v.0;
    let vi = scaled_v.to_i32();
    c.f[1] = vi as u32;
    c.g[V0][0] = sx(vi as u32);
    Addr(dst).off(0x2).sh(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Irradiance

/// `bool Irradiance_SpotLightVector(Vec3 *out, Vec3 *surf, const Light *l,
/// float *atten, float *spot)` (0x38a530): compute the normalised direction and
/// scaled colour, returning false when the surface is outside the spot cone.
pub fn irradiance_spot_light_vector(c: &mut Cpu) {
    c.cycles += 20;
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0] as u32;
    let t0 = c.g[T0][0] as u32;
    let t1 = c.g[T1][0] as u32;
    c.f[2] = Addr(a2).off(0x38).read_fpu(c).0;
    c.f[1] = Addr(a0).read_fpu(c).0;
    c.f[3] = Addr(a2).off(0x3c).read_fpu(c).0;
    c.f[0] = Addr(a0).off(0x4).read_fpu(c).0;
    c.f[8] = (FpuF32(c.f[2]) - FpuF32(c.f[1])).0;
    c.f[2] = Addr(a2).off(0x40).read_fpu(c).0;
    c.f[7] = (FpuF32(c.f[3]) - FpuF32(c.f[0])).0;
    c.f[4] = Addr(a0).off(0x8).read_fpu(c).0;
    c.f[1] = (FpuF32(c.f[8]) * FpuF32(c.f[8])).0;
    c.f[0] = Addr(a2).off(0x1c).read_fpu(c).0;
    c.f[6] = (FpuF32(c.f[2]) - FpuF32(c.f[4])).0;
    c.f[3] = (FpuF32(c.f[7]) * FpuF32(c.f[7])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[0])).0;
    c.f[2] = (FpuF32(c.f[6]) * FpuF32(c.f[6])).0;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[3])).0;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[2])).0;
    abi::set_fcc(c, FpuF32(c.f[0]).lt(FpuF32(c.f[1])));
    c.g[V0][0] = 0;
    if c.fcc() {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.f[0] = c.g[0][0] as u32;
    abi::set_fcc(c, FpuF32(c.f[1]).eq_s(FpuF32(c.f[0])));
    if c.fcc() {
        c.cycles += 4;
        c.g[AT][0] = 0x3f80_0000;
        c.f[4] = c.g[1][0] as u32;
        c.f[5] = c.f[4];
        c.f[8] = (FpuF32(c.f[8]) * FpuF32(c.f[4])).0;
    } else {
        c.cycles += 6;
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = c.g[1][0] as u32;
        c.f[5] = FpuF32(c.f[1]).sqrt().0;
        c.f[4] = FpuF32(c.f[0]).div(FpuF32(c.f[5])).0;
        c.f[8] = (FpuF32(c.f[8]) * FpuF32(c.f[4])).0;
    }
    c.cycles += 22;
    c.f[0] = Addr(a2).off(0x2c).read_fpu(c).0;
    c.f[7] = (FpuF32(c.f[7]) * FpuF32(c.f[4])).0;
    c.f[2] = Addr(a2).off(0x30).read_fpu(c).0;
    c.f[6] = (FpuF32(c.f[6]) * FpuF32(c.f[4])).0;
    c.f[1] = Addr(a2).off(0x34).read_fpu(c).0;
    c.f[0] = (FpuF32(c.f[8]) * FpuF32(c.f[0])).0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[3] = c.g[1][0] as u32;
    c.f[2] = (FpuF32(c.f[7]) * FpuF32(c.f[2])).0;
    c.g[V0][0] = 0x1;
    c.f[1] = (FpuF32(c.f[6]) * FpuF32(c.f[1])).0;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[2])).0;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
    c.f[0] = (-FpuF32(c.f[0])).0;
    Addr(t1).write_fpu(c, FpuF32(c.f[0]));
    Addr(a3).write_fpu(c, FpuF32(c.f[5]));
    Addr(t0).write_fpu(c, FpuF32(c.f[4]));
    Addr(a1).off(0xc).write_fpu(c, FpuF32(c.f[3]));
    Addr(a1).write_fpu(c, FpuF32(c.f[8]));
    Addr(a1).off(0x4).write_fpu(c, FpuF32(c.f[7]));
    Addr(a1).off(0x8).write_fpu(c, FpuF32(c.f[6]));
    abi::ret(c);
}

/// `bool Irradiance_PointLightVector(Vec3 *out, Vec3 *surf, const Light *l,
/// float *atten, float *dist)` (0x38a618): normalise the surface direction and
/// scale the colour, returning false when the surface is too far away.
pub fn irradiance_point_light_vector(c: &mut Cpu) {
    c.cycles += 19;
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0] as u32;
    let t0 = c.g[T0][0] as u32;
    c.f[2] = Addr(a2).off(0x38).read_fpu(c).0;
    c.f[1] = Addr(a0).read_fpu(c).0;
    c.f[3] = Addr(a2).off(0x3c).read_fpu(c).0;
    c.f[0] = Addr(a0).off(0x4).read_fpu(c).0;
    c.f[6] = (FpuF32(c.f[2]) - FpuF32(c.f[1])).0;
    c.f[2] = Addr(a0).off(0x8).read_fpu(c).0;
    c.f[5] = (FpuF32(c.f[3]) - FpuF32(c.f[0])).0;
    c.f[4] = Addr(a2).off(0x40).read_fpu(c).0;
    c.f[1] = (FpuF32(c.f[6]) * FpuF32(c.f[6])).0;
    c.f[0] = Addr(a2).off(0x1c).read_fpu(c).0;
    c.f[4] = (FpuF32(c.f[4]) - FpuF32(c.f[2])).0;
    c.f[3] = (FpuF32(c.f[5]) * FpuF32(c.f[5])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[0])).0;
    c.f[2] = (FpuF32(c.f[4]) * FpuF32(c.f[4])).0;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[3])).0;
    c.f[2] = (FpuF32(c.f[1]) + FpuF32(c.f[2])).0;
    abi::set_fcc(c, FpuF32(c.f[0]).lt(FpuF32(c.f[2])));
    c.g[V0][0] = 0;
    if c.fcc() {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 15;
    c.g[AT][0] = 0x3f80_0000;
    c.f[1] = c.g[1][0] as u32;
    c.f[0] = FpuF32(c.f[2]).sqrt().0;
    Addr(a3).write_fpu(c, FpuF32(c.f[0]));
    c.g[V0][0] = 0x1;
    c.f[0] = FpuF32(c.f[1]).div(FpuF32(c.f[0])).0;
    c.f[6] = (FpuF32(c.f[6]) * FpuF32(c.f[0])).0;
    Addr(t0).write_fpu(c, FpuF32(c.f[0]));
    c.f[5] = (FpuF32(c.f[5]) * FpuF32(c.f[0])).0;
    Addr(a1).off(0xc).write_fpu(c, FpuF32(c.f[1]));
    c.f[4] = (FpuF32(c.f[4]) * FpuF32(c.f[0])).0;
    Addr(a1).write_fpu(c, FpuF32(c.f[6]));
    Addr(a1).off(0x4).write_fpu(c, FpuF32(c.f[5]));
    Addr(a1).off(0x8).write_fpu(c, FpuF32(c.f[4]));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Light manager

/// `LightMan_ctor(LightMan *m)` (0x38ae10): install the vtable and register
/// the manager in the global at `gp + 0x2a9c`.
pub fn light_man_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x49_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(0x49_0000u32.wrapping_add(0x36d8));
    Addr(abi::arg(c, 0)).sw(c, c.g[V1][0]);
    Addr((c.g[GP][0] as u32).wrapping_add(0x2a9c)).sw(c, c.g[A0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// 2D rendering

/// `void Render2D_TransformPoint(Vec4 *out, const Vec2 *p, const Mat4 *m)`
/// (0x38e0a8): transform a 2D point by `m` into `out` (w = 1).
pub fn render2d_transform_point(c: &mut Cpu) {
    c.cycles += 28;
    let a0 = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    c.f[6] = Addr(a1).off(0x4).read_fpu(c).0;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.f[2] = Addr(a2).off(0xc).read_fpu(c).0;
    c.g[V0][0] = c.g[A0][0];
    c.f[0] = Addr(a2).off(0x4).read_fpu(c).0;
    c.f[3] = Addr(a2).off(0x10).read_fpu(c).0;
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[6])).0;
    c.f[4] = Addr(a1).read_fpu(c).0;
    c.f[1] = Addr(a2).read_fpu(c).0;
    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[6])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[4])).0;
    c.f[7] = Addr(a2).off(0x1c).read_fpu(c).0;
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[4])).0;
    c.f[5] = Addr(a2).off(0x18).read_fpu(c).0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[4] = c.g[1][0] as u32;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[3])).0;
    let sp = c.g[SP][0] as u32;
    Addr(sp).off(0x8).sw(c, 0);
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[2])).0;
    Addr(sp).off(0xc).write_fpu(c, FpuF32(c.f[4]));
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[7])).0;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[5])).0;
    Addr(sp).off(0x4).write_fpu(c, FpuF32(c.f[0]));
    Addr(sp).write_fpu(c, FpuF32(c.f[1]));
    c.lq(3, sp);
    c.sq(3, a0);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// PS2 graphics manager (colour filter)

/// `void PS2GraphicsMan_ResetColorFilter(PS2GraphicsMan *man, int index)`
/// (0x390458): write the six default colour-filter floats from `gp + 0x14f4`
/// into the record at `man + index*0x18 + 0x6ca4`.
pub fn ps2_graphics_man_reset_color_filter(c: &mut Cpu) {
    c.cycles += 17;
    c.g[V0][0] = 0x18;
    let gp = c.g[GP][0] as u32;
    c.f[1] = Addr(gp.wrapping_add(0x14f4)).read_fpu(c).0;
    c.mult(0, 5, 5, 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6ca4));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let dst = c.g[A0][0] as u32;
    Addr(dst).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = Addr(gp.wrapping_add(0x14f8)).read_fpu(c).0;
    Addr(dst).off(0x4).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = Addr(gp.wrapping_add(0x14fc)).read_fpu(c).0;
    Addr(dst).off(0x8).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = Addr(gp.wrapping_add(0x1500)).read_fpu(c).0;
    Addr(dst).off(0xc).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = Addr(gp.wrapping_add(0x1504)).read_fpu(c).0;
    Addr(dst).off(0x10).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = Addr(gp.wrapping_add(0x1508)).read_fpu(c).0;
    Addr(dst).off(0x14).write_fpu(c, FpuF32(c.f[0]));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Screen tint

/// `void VfxScreenTint_TriggerFlash(float intensity)` (0x390ec8): when the
/// screen tint is active, start a flash: store the scaled intensity and its
/// integer value in the global tint block.
pub fn vfx_screen_tint_trigger_flash(c: &mut Cpu) {
    c.cycles += 3;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(0x151c)).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        let rate = Addr(gp.wrapping_sub(0x2698)).read_fpu(c);
        c.f[0] = rate.0;
        Addr(gp.wrapping_add(0x15a8)).sw(c, 0);
        let scaled = abi::farg(c, 0) * rate;
        c.f[0] = scaled.0;
        Addr(gp.wrapping_add(0x15ac)).sw(c, 0);
        Addr(gp.wrapping_add(0x15c0)).write_fpu(c, abi::farg(c, 0));
        let i = scaled.to_i32();
        c.f[1] = i as u32;
        Addr(gp.wrapping_add(0x15c4)).write_fpu(c, FpuF32(c.f[1]));
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool VfxScreenTint_PollFlashSound(void)` (0x390ef8): true once while the
/// flash is active and its countdown has reached zero, clearing the countdown.
pub fn vfx_screen_tint_poll_flash_sound(c: &mut Cpu) {
    c.cycles += 3;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(0x151c)).lw(c);
    let inactive = c.g[V0][0] == 0;
    c.g[V0][0] = Addr(gp.wrapping_add(0x15c4)).lw(c);
    if inactive {
        // fall through to the false return
    } else {
        c.cycles += 2;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        if c.g[V0][0] != 0 {
            // counting down: still false
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            Addr(gp.wrapping_add(0x15c4)).sw(c, c.g[V1][0]);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI animation

/// `int UIAnim_GetFrameCountSafe(UIAnim **anim)` (0x397278): frame count at
/// `*anim + 2`, or 0 when the animation is null.
pub fn uianim_get_frame_count_safe(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(abi::arg(c, 0)).lw(c);
    c.g[V0][0] = 0;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x2).lhu(c);
    abi::ret(c);
}

/// `bool UIAnim_IsDone(UIAnim *anim)` (0x397440): true when the current frame
/// (signed 16-bit at +0xa) has reached the frame count at `*anim + 2`.
pub fn uianim_is_done(c: &mut Cpu) {
    c.cycles += 6;
    let anim = Addr(abi::arg(c, 0));
    c.g[A1][0] = anim.lw(c);
    c.g[V0][0] = anim.off(0xa).lh(c);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x2).lhu(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    abi::ret(c);
}

/// `int UIAnim_GetFrameCount(UIAnim *anim)` (0x397458): signed frame count at
/// `*anim + 2`.
pub fn uianim_get_frame_count(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x2).lh(c);
    abi::ret(c);
}

/// `void UIAnim_SetPlayMode(UIAnim *anim, int mode, int loops)` (0x397468):
/// set the play mode byte and initialise the loop counter.
pub fn uianim_set_play_mode(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[A1][0] = c.g[A1][0] & 0x7;
    c.g[A2][0] = c.g[A2][0] & 0xffff;
    if c.g[A1][0] == 0 {
        c.cycles += 1;
        Addr(c.g[A3][0] as u32).off(0xc).sh(c, c.g[A2][0]);
    }
    c.cycles += 3;
    c.g[V0][0] = 0x2;
    if c.g[A1][0] != c.g[V0][0] {
        Addr(c.g[A3][0] as u32).off(0xa).sh(c, 0);
        c.cycles += 2;
        Addr(c.g[A3][0] as u32).off(0xe).sb(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = Addr(c.g[A3][0] as u32).lw(c);
    Addr(c.g[A3][0] as u32).off(0xe).sb(c, c.g[V0][0]);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x2).lhu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
    Addr(c.g[A3][0] as u32).off(0xa).sh(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI list

/// `UIObject *UIList_Last(UIList *list)` (0x397858): the last node, or null
/// when the list is empty (its head points at itself).
pub fn uilist_last(c: &mut Cpu) {
    c.cycles += 5;
    let list = Addr(abi::arg(c, 0));
    c.g[V0][0] = list.off(0xc).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V1][0] = c.g[V1][0] ^ c.g[V0][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `void UIList_PushFront(UIList *list, UIObject *item)` (0x3978f0): insert
/// `item` at the front of the list.
pub fn uilist_push_front(c: &mut Cpu) {
    c.cycles += 7;
    let list = Addr(abi::arg(c, 0));
    let item = abi::arg(c, 1);
    c.g[V0][0] = list.off(0x4).lw(c);
    Addr(item).off(0x4).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    Addr(item).sw(c, c.g[V1][0]);
    Addr(c.g[V0][0] as u32).sw(c, c.g[A1][0]);
    list.off(0x4).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void UIList_PushBack(UIList *list, UIObject *item)` (0x397910): insert
/// `item` at the back of the list.
pub fn uilist_push_back(c: &mut Cpu) {
    c.cycles += 7;
    let list = Addr(abi::arg(c, 0));
    let item = abi::arg(c, 1);
    c.g[V0][0] = list.off(0xc).lw(c);
    Addr(item).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4).lw(c);
    Addr(item).off(0x4).sw(c, c.g[V1][0]);
    Addr(c.g[V0][0] as u32).off(0x4).sw(c, c.g[A1][0]);
    list.off(0xc).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void UIList_Reset(UIList *list)` (0x397930): make the list empty (head and
/// tail point at the embedded sentinel at +0xc).
pub fn uilist_reset(c: &mut Cpu) {
    c.cycles += 6;
    let list = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
    list.sw(c, c.g[A0][0]);
    list.off(0x10).sw(c, c.g[V0][0]);
    list.off(0x4).sw(c, c.g[V0][0]);
    list.off(0xc).sw(c, c.g[A0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI font set

/// `void UIFontSet_AddFont(UIFontSet *set, int index, void *a, void *b)`
/// (0x3984b0): while fewer than 4 fonts are registered, store the two words at
/// `set + index*0xc + 4` and bump the count at +0x34.
pub fn uifont_set_add_font(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x34).lb(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
    let full = c.g[V0][0] == 0;
    c.g[V0][0] = 0xc;
    if !full {
        c.cycles += 8;
        c.mult(0, 8, 5, 2);
        c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(this));
        Addr(c.g[V1][0] as u32).off(0x8).sw(c, c.g[A2][0]);
        c.g[V0][0] = c.g[V1][0];
        Addr(c.g[V0][0] as u32).off(0x4).sw(c, c.g[A3][0]);
        c.g[V1][0] = Addr(this).off(0x34).lbu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        Addr(this).off(0x34).sb(c, c.g[V1][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI list box

/// Shared body of the `UIListBox_GetSelected*` accessors: if the selected
/// index (+0x319) is below the count (+0x318) return the word at
/// `this + selected*0x14 + field`, else 0.
fn uilistbox_get_selected(c: &mut Cpu, field: u32) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x319).lbu(c);
    c.g[V1][0] = Addr(this).off(0x318).lbu(c);
    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = Addr(this).off(0x319).lbu(c);
        c.cycles += 5;
        c.g[V0][0] = 0x14;
        c.mult(0, 5, 3, 2);
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(this));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(field).lw(c);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void *UIListBox_GetSelectedData(UIListBox *box)` (0x39a708).
pub fn uilistbox_get_selected_data(c: &mut Cpu) {
    uilistbox_get_selected(c, 0xa0);
}

/// `int UIListBox_GetSelectedValue(UIListBox *box)` (0x39a738).
pub fn uilistbox_get_selected_value(c: &mut Cpu) {
    uilistbox_get_selected(c, 0xa4);
}

/// `void UIListBox_SetSelection(UIListBox *box, int index)` (0x39a7a8):
/// store the index if it is below the item count.
pub fn uilistbox_set_selection(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x318).lbu(c);
    c.g[A1][0] = c.g[A1][0] & 0xff;
    c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    if c.g[V0][0] != 0 {
        Addr(this).off(0x319).sb(c, c.g[A1][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI menu

/// `void UIMenu_SetNavButtons(UIMenu *menu, int enabled)` (0x39ab00): store
/// the four navigation-button indices, choosing the enabled or disabled set.
pub fn uimenu_set_nav_buttons(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    let enabled = c.g[A1][0] != 0;
    c.g[A2][0] = c.g[A0][0];
    let menu = c.g[A2][0] as u32;
    if enabled {
        c.cycles += 9;
        c.g[V1][0] = 0x4;
        c.g[A0][0] = 0x5;
        c.g[A1][0] = 0x6;
        c.g[V0][0] = 0x7;
        Addr(menu).off(0x94).sb(c, c.g[V0][0]);
        Addr(menu).off(0x91).sb(c, c.g[V1][0]);
        Addr(menu).off(0x92).sb(c, c.g[A0][0]);
        Addr(menu).off(0x93).sb(c, c.g[A1][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = 0x2;
    c.g[V1][0] = 0x3;
    Addr(menu).off(0x92).sb(c, c.g[V0][0]);
    Addr(menu).off(0x94).sb(c, c.g[V1][0]);
    Addr(menu).off(0x93).sb(c, c.g[A0][0]);
    Addr(menu).off(0x91).sb(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI screen commands

/// `void UIScreen_CmdStopThread(void *ctx)` (0x39cbc8): clear the "thread
/// running" bit (bit 16) of the flag word at `a3 + 0x10`.
pub fn uiscreen_cmd_stop_thread(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x10).lw(c);
    c.g[V0][0] = 0xffff_ffff_feff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
    Addr(c.g[A3][0] as u32).off(0x10).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void UIScreen_CmdEndTransitionIn(void *ctx)` (0x39ce20): set the transition
/// state bits (0x300) of the screen's flag word when the screen exists.
pub fn uiscreen_cmd_end_transition_in(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0xd0).lw(c);
    let present = c.g[A0][0] != 0;
    c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
    if present {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x300;
        Addr(c.g[A0][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void UIScreen_CmdEndTransitionOut(void *ctx)` (0x39ce98): clear the thread
/// bit of `a3 + 0x10` and set state bits 0x700 on the screen's flag word.
pub fn uiscreen_cmd_end_transition_out(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x10).lw(c);
    c.g[V0][0] = 0xffff_ffff_feff_0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
    Addr(c.g[A3][0] as u32).off(0x10).sw(c, c.g[V1][0]);
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(0xd0).lw(c);
    let present = c.g[A0][0] != 0;
    c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
    if present {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0x700;
        Addr(c.g[A0][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI object

/// `void UIObject_ApplyAlignment(UIObject *obj, Vec2 *dst, const Vec2 *size)`
/// (0x39feb8): apply the object's alignment flags (bits 13..18 of the word at
/// +0x14) to adjust `dst` against the size.
pub fn uiobject_apply_alignment(c: &mut Cpu) {
    c.cycles += 6;
    let obj = c.g[A0][0] as u32;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    c.g[V0][0] = Addr(obj).off(0x14).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 13);
    c.g[A0][0] = c.g[V0][0] & 0x3f;
    c.g[V1][0] = c.g[A0][0] & 0x1;
    let centered = c.g[V1][0] != 0;
    c.g[V0][0] = c.g[A0][0] & 0x8;
    if !centered {
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0] & 0x4;
        if c.g[V0][0] == 0 {
            c.f[0] = Addr(a2).off(0x4).read_fpu(c).0;
            c.cycles += 6;
            c.g[AT][0] = 0x3f00_0000;
            c.f[2] = c.g[1][0] as u32;
            c.f[1] = Addr(a1).off(0x4).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
            c.f[1] = (FpuF32(c.f[1]) - FpuF32(c.f[0])).0;
            Addr(a1).off(0x4).write_fpu(c, FpuF32(c.f[1]));
        } else {
            c.cycles += 5;
            c.f[1] = Addr(a2).off(0x4).read_fpu(c).0;
            c.f[0] = Addr(a1).off(0x4).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[1])).0;
            Addr(a1).off(0x4).write_fpu(c, FpuF32(c.f[0]));
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0] & 0x8;
    }
    c.cycles += 2;
    let skip_horizontal = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[A0][0] & 0x20;
    if !skip_horizontal {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.f[0] = Addr(a2).read_fpu(c).0;
            c.cycles += 6;
            c.g[AT][0] = 0x3f00_0000;
            c.f[2] = c.g[1][0] as u32;
            c.f[1] = Addr(a1).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
            c.f[1] = (FpuF32(c.f[1]) - FpuF32(c.f[0])).0;
            Addr(a1).write_fpu(c, FpuF32(c.f[1]));
        } else {
            c.cycles += 5;
            c.f[1] = Addr(a2).read_fpu(c).0;
            c.f[0] = Addr(a1).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[0]) - FpuF32(c.f[1])).0;
            Addr(a1).write_fpu(c, FpuF32(c.f[0]));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `UIObject *UIObject_FindById(UIObject *obj, int id)` (0x39ff60): `obj` when
/// its id (+0x38) equals `id`, else null.
pub fn uiobject_find_by_id(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x38).lw(c);
    c.g[V0][0] = 0;
    c.g[V1][0] = c.g[V1][0] ^ c.g[A1][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI object user data / event thread

/// `int UIObject_GetEventThread(UIObject *obj, int index)` (0x39ff78): look up
/// the signed byte index in the 16-bit table at `obj + 0x3c`, or 0xffff.
pub fn uiobject_get_event_thread(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x3c).lw(c);
    let present = c.g[V0][0] != 0;
    c.g[V1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
    if present {
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0xffff;
    abi::ret(c);
}

/// `uint UIObject_GetUserData(UIObject *obj, int slot)` (0x3a0318): the word at
/// `obj + slot*4 + 0x6c`.
pub fn uiobject_get_user_data(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 22) as u32);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x6c).lw(c);
    abi::ret(c);
}

/// `void UIObject_SetUserData(UIObject *obj, int slot, uint value)`
/// (0x3a0330): store `value` at `obj + slot*4 + 0x6c`.
pub fn uiobject_set_user_data(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 22) as u32);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x6c).sw(c, c.g[A2][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UI text

/// `void *UIText_GetFont(UIText *text)` (0x3a04f0): the font record selected
/// by the low two bits at +0x74.
pub fn uitext_get_font(c: &mut Cpu) {
    c.cycles += 5;
    let text = abi::arg(c, 0);
    c.g[V0][0] = Addr(text).off(0x5c).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xd0).lw(c);
    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
    if c.g[A1][0] != 0 {
        c.g[V1][0] = Addr(text).off(0x74).lbu(c);
        c.cycles += 7;
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(c.g[A1][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = c.g[V1][0] & 0x3;
        c.mult(0, 3, 3, 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void *cUITextScroll_GetFont(cUITextScroll *text)` (0x3a12d0): the font
/// record selected by bits 1..2 of the word at +0x74.
pub fn c_ui_text_scroll_get_font(c: &mut Cpu) {
    c.cycles += 5;
    let text = abi::arg(c, 0);
    c.g[V0][0] = Addr(text).off(0x5c).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xd0).lw(c);
    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
    if c.g[A1][0] != 0 {
        c.g[V1][0] = Addr(text).off(0x74).lw(c);
        c.cycles += 8;
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(c.g[A1][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
        c.g[V1][0] = c.g[V1][0] & 0x3;
        c.mult(0, 3, 3, 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void cUITextScroll_SetNavButtons(cUITextScroll *text, int enabled)`
/// (0x3a1310): store the four navigation-button indices at +0x88.
pub fn c_ui_text_scroll_set_nav_buttons(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    let enabled = c.g[A1][0] != 0;
    c.g[A2][0] = c.g[A0][0];
    let text = c.g[A2][0] as u32;
    if enabled {
        c.cycles += 9;
        c.g[V1][0] = 0x4;
        c.g[A0][0] = 0x5;
        c.g[A1][0] = 0x6;
        c.g[V0][0] = 0x7;
        Addr(text).off(0x8b).sb(c, c.g[V0][0]);
        Addr(text).off(0x88).sb(c, c.g[V1][0]);
        Addr(text).off(0x89).sb(c, c.g[A0][0]);
        Addr(text).off(0x8a).sb(c, c.g[A1][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = 0x2;
    c.g[V1][0] = 0x3;
    Addr(text).off(0x89).sb(c, c.g[V0][0]);
    Addr(text).off(0x8b).sb(c, c.g[V1][0]);
    Addr(text).off(0x8a).sb(c, c.g[A0][0]);
    Addr(text).off(0x88).sb(c, 0);
    abi::ret(c);
}

/// `void *cUITextScroll_GetLine(cUITextScroll *text, int index)`
/// (0x3a2068): the line pointer at `text + index*4`, or null when out of range.
pub fn c_ui_text_scroll_get_line(c: &mut Cpu) {
    c.cycles += 3;
    let text = abi::arg(c, 0);
    c.g[A2][0] = Addr(text).off(0xbc).lw(c);
    let lines_present = c.g[A2][0] != 0;
    if lines_present {
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(text).off(0xc8).lhu(c);
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            if !out_of_range {
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Stream manager

/// `void cStreamMan_SetRate(cStreamMan *man, int rate)` (0x3a7058): clamp the
/// rate to at least 0x8000 and store it and the tick period (from the global
/// timer) at +0xd8/+0xdc.
pub fn cstream_man_set_rate(c: &mut Cpu) {
    c.cycles += 16;
    let man = c.g[A0][0] as u32;
    c.g[V0][0] = 0x8000;
    c.g[AT][0] = 0x3800_0000;
    c.f[2] = c.g[1][0] as u32;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V0][0] == 0 {
        c.g[V1][0] = c.g[A1][0];
    }
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.f[1] = FpuF32::from_i32(c.g[V1][0] as u32 as i32).0;
    Addr(man).off(0xd8).sw(c, c.g[V1][0]);
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[2])).0;
    c.f[0] = Addr(c.g[V0][0] as u32).off(0x10).read_fpu(c).0;
    c.f[0] = FpuF32::from_i32(c.f[0] as i32).0;
    c.f[0] = FpuF32(c.f[0]).div(FpuF32(c.f[1])).0;
    Addr(man).off(0xdc).write_fpu(c, FpuF32(c.f[0]));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World block pool

/// `void *WorldBlockPool_AllocBlock(WorldBlockPool *pool)` (0x3a7768): pop a
/// block off the free list at +0x10, mark it used and return it in v0.
pub fn world_block_pool_alloc_block(c: &mut Cpu) {
    c.cycles += 10;
    let pool = Addr(abi::arg(c, 0));
    c.g[A1][0] = pool.off(0x10).lw(c);
    c.g[A3][0] = 0x1;
    c.g[V1][0] = pool.off(0x8).lw(c);
    c.g[A2][0] = Addr(c.g[A1][0] as u32).off(0x14).lw(c);
    c.g[V0][0] = c.g[A1][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
    pool.off(0x8).sw(c, c.g[V1][0]);
    pool.off(0x10).sw(c, c.g[A2][0]);
    Addr(c.g[A1][0] as u32).off(0x10).sw(c, c.g[A3][0]);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn tex_man_set_tex_function_matches() {
        check(ssx3_ee::s12::f_003691b0, tex_man_set_tex_function, 200, &|c, seed| {
            let mut r = Rng(0x3691_b000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            if seed % 3 == 0 {
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
            } else {
                let idx = (seed % 8) as u32;
                c.g[A1][0] = idx as u64;
                c.bus.write32(DATA + idx * 4 + 8, DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0x38, r.next_u32());
                c.bus.write32(DATA + 0x100 + 0x3c, r.next_u32());
            }
        });
    }

    #[test]
    fn ps2_graphics_man_set_shape_banks_matches() {
        check(ssx3_ee::s12::f_00369890, ps2_graphics_man_set_shape_banks, 200, &|c, seed| {
            let mut r = Rng(0x3698_9000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            c.g[A2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
        });
    }

    #[test]
    fn deferred_delete_push_matches() {
        check(ssx3_ee::s12::f_0036a1c0, deferred_delete_push, 200, &|c, seed| {
            let mut r = Rng(0x36a1_c000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            let count = r.next_u32() % 8;
            c.bus.write32(GAME_GP + 0x2a98, count);
            c.bus.write32(0x53ab40 + count * 4, r.next_u32());
        });
    }

    #[test]
    fn morton_spread8_matches() {
        check(ssx3_ee::s12::f_0036a3b0, morton_spread8, 200, &|c, seed| {
            let mut r = Rng(0x36a3_b000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
        });
    }

    #[test]
    fn ps2_graphics_man_reset_depth_fog_params_matches() {
        check(ssx3_ee::s12::f_0036aba0, ps2_graphics_man_reset_depth_fog_params, 200, &|c, seed| {
            let mut r = Rng(0x36ab_a000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(GAME_GP + 0x123c, r.edge_f32());
            c.bus.write32(GAME_GP + 0x1240, r.edge_f32());
            c.bus.write32(GAME_GP + 0x1244, r.next_u32());
            c.bus.write32(GAME_GP + 0x1248, r.edge_f32());
        });
    }

    #[test]
    fn emitter_set_desc_time_matches() {
        check(ssx3_ee::s12::f_0036d3d8, emitter_set_desc_time, 200, &|c, seed| {
            let mut r = Rng(0x36d3_d800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.f[12] = r.edge_f32();
            c.bus.write32(DATA + 0x4c, r.edge_f32());
        });
    }

    #[test]
    fn emitter_advance_desc_time_matches() {
        check(ssx3_ee::s12::f_0036d3e8, emitter_advance_desc_time, 200, &|c, seed| {
            let mut r = Rng(0x36d3_e800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.f[12] = r.edge_f32();
            c.bus.write32(DATA + 0x4c, r.edge_f32());
            c.bus.write32(DATA + 0x8, r.edge_f32());
        });
    }

    #[test]
    fn bezier_patch_cache_ctor_matches() {
        check(ssx3_ee::s12::f_00372b30, bezier_patch_cache_ctor, 200, &|c, seed| {
            let mut r = Rng(0x372b_3000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            for k in 0..3 {
                c.bus.write32(DATA + 0xc + k * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn boader_shadow_set_params_matches() {
        check(ssx3_ee::s12::f_00374ce0, boader_shadow_set_params, 200, &|c, seed| {
            let mut r = Rng(0x374c_e000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            c.g[A2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            c.g[A3][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            c.g[T0][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            c.g[T1][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            c.g[T2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
        });
    }

    #[test]
    fn boader_shadow_ctor_matches() {
        check(ssx3_ee::s12::f_003758f8, boader_shadow_ctor, 200, &|c, seed| {
            let mut r = Rng(0x3758_f800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            for k in 0..4 {
                c.bus.write32(DATA + 0x170 + k * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn detail_system_ctor_matches() {
        check(ssx3_ee::s12::f_003759e8, detail_system_ctor, 200, &|c, seed| {
            let mut r = Rng(0x3759_e800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn line_strip_init_matches() {
        check(ssx3_ee::s12::f_00384d98, line_strip_init, 200, &|c, seed| {
            let mut r = Rng(0x384d_9800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x100, r.next_u32());
        });
    }

    #[test]
    fn line_strip_close_matches() {
        check(ssx3_ee::s12::f_00384dc0, line_strip_close, 200, &|c, seed| {
            let mut r = Rng(0x384d_c000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let buf = DATA + 0x400;
            c.bus.write32(DATA + 0xc, buf);
            if seed % 2 == 0 {
                c.bus.write32(DATA + 0x10, 0);
            } else {
                c.bus.write32(DATA + 0x10, 1);
                c.bus.write32(DATA + 0x4, buf);
            }
            c.bus.write32(buf + 0x30, r.next_u32());
        });
    }

    #[test]
    fn line_strip_add_vertex_matches() {
        check(ssx3_ee::s12::f_00384fd0, line_strip_add_vertex, 200, &|c, seed| {
            let mut r = Rng(0x384f_d000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA + 0x4, DATA + 0x800);
            c.bus.write32(DATA + 0x10, r.next_u32() & 0xff);
            for k in 0..8 {
                c.bus.write32(DATA + 0x100 + k * 4, r.edge_f32());
            }
            for k in 0..4 {
                c.bus.write32(DATA + 0x200 + k * 4, r.edge_f32());
            }
        });
    }

    #[test]
    fn cps2_grid_mesh_pack_position_matches() {
        check(ssx3_ee::s12::f_00385a38, cps2_grid_mesh_pack_position, 200, &|c, seed| {
            let mut r = Rng(0x385a_3800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x1000) as u64;
            c.g[A2][0] = (seed % 8) as u64;
            c.g[A3][0] = (DATA + 0x2000) as u64;
            c.bus.write32(DATA + 0x198, r.edge_f32());
            for k in 0..3 {
                c.bus.write32(DATA + 0x2000 + k * 4, r.edge_f32());
            }
        });
    }

    #[test]
    fn cps2_grid_mesh_pack_uv_matches() {
        check(ssx3_ee::s12::f_00385ba0, cps2_grid_mesh_pack_uv, 200, &|c, seed| {
            let mut r = Rng(0x385b_a000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (DATA + 0x1000) as u64;
            c.g[A2][0] = (seed % 8) as u64;
            c.g[A3][0] = (DATA + 0x2000) as u64;
            for k in 0..2 {
                c.bus.write32(DATA + 0x2000 + k * 4, r.edge_f32());
            }
        });
    }

    /// Shared setup for the two irradiance functions: `a0` a surface point,
    /// `a2` the light, `a1` the output vector.
    fn irradiance_setup(c: &mut Cpu, seed: u32, diffs_are_zero: bool, radius_zero: bool) {
        let mut r = Rng(0x38a0_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = (DATA + 0x100) as u64;
        c.g[A1][0] = (DATA + 0x300) as u64;
        c.g[A2][0] = (DATA + 0x200) as u64;
        c.g[A3][0] = (DATA + 0x400) as u64;
        c.g[T0][0] = (DATA + 0x500) as u64;
        c.g[T1][0] = (DATA + 0x600) as u64;
        for k in 0..3 {
            c.bus.write32(DATA + 0x100 + k * 4, r.edge_f32());
        }
        for k in 0..3 {
            let v = if diffs_are_zero {
                c.bus.read32(DATA + 0x100 + k * 4)
            } else {
                r.edge_f32()
            };
            c.bus.write32(DATA + 0x200 + 0x38 + k * 4, v);
        }
        c.bus.write32(DATA + 0x200 + 0x1c, if radius_zero { 0 } else { r.edge_f32() });
        for off in [0x2c, 0x30, 0x34] {
            c.bus.write32(DATA + 0x200 + off, r.edge_f32());
        }
    }

    #[test]
    fn irradiance_spot_light_vector_matches() {
        check(ssx3_ee::s12::f_0038a530, irradiance_spot_light_vector, 200, &|c, seed| {
            match seed % 4 {
                0 => irradiance_setup(c, seed, false, true),
                1 => irradiance_setup(c, seed, true, true),
                _ => irradiance_setup(c, seed, false, false),
            }
        });
    }

    #[test]
    fn irradiance_point_light_vector_matches() {
        check(ssx3_ee::s12::f_0038a618, irradiance_point_light_vector, 200, &|c, seed| {
            match seed % 3 {
                0 => irradiance_setup(c, seed, false, true),
                _ => irradiance_setup(c, seed, false, false),
            }
        });
    }

    #[test]
    fn light_man_ctor_matches() {
        check(ssx3_ee::s12::f_0038ae10, light_man_ctor, 200, &|c, seed| {
            let mut r = Rng(0x38ae_1000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(GAME_GP + 0x2a9c, r.next_u32());
        });
    }

    #[test]
    fn render2d_transform_point_matches() {
        check(ssx3_ee::s12::f_0038e0a8, render2d_transform_point, 200, &|c, seed| {
            let mut r = Rng(0x38e0_a800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            for k in 0..2 {
                c.bus.write32(DATA + 0x100 + k * 4, r.edge_f32());
            }
            for off in [0, 4, 0xc, 0x10, 0x18, 0x1c] {
                c.bus.write32(DATA + 0x200 + off, r.edge_f32());
            }
        });
    }

    #[test]
    fn ps2_graphics_man_reset_color_filter_matches() {
        check_lohi(ssx3_ee::s12::f_00390458, ps2_graphics_man_reset_color_filter, 200, &|c, seed| {
            let mut r = Rng(0x3904_5800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 4) as u64;
            for off in [0x14f4, 0x14f8, 0x14fc, 0x1500, 0x1504, 0x1508] {
                c.bus.write32(GAME_GP + off, r.edge_f32());
            }
        });
    }

    #[test]
    fn vfx_screen_tint_trigger_flash_matches() {
        check(ssx3_ee::s12::f_00390ec8, vfx_screen_tint_trigger_flash, 200, &|c, seed| {
            let mut r = Rng(0x390e_c800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.f[12] = r.edge_f32();
            c.bus.write32(GAME_GP + 0x151c, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(GAME_GP - 0x2698, r.edge_f32());
        });
    }

    #[test]
    fn vfx_screen_tint_poll_flash_sound_matches() {
        check(ssx3_ee::s12::f_00390ef8, vfx_screen_tint_poll_flash_sound, 200, &|c, seed| {
            let mut r = Rng(0x390e_f800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0x151c, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(GAME_GP + 0x15c4, if seed % 4 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn uianim_get_frame_count_safe_matches() {
        check(ssx3_ee::s13::f_00397278, uianim_get_frame_count_safe, 200, &|c, seed| {
            let mut r = Rng(0x3972_7800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            if seed % 3 == 0 {
                c.bus.write32(DATA, 0);
            } else {
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write16(DATA + 0x102, r.next_u32() as u16);
            }
        });
    }

    #[test]
    fn uianim_is_done_matches() {
        check(ssx3_ee::s13::f_00397440, uianim_is_done, 200, &|c, seed| {
            let mut r = Rng(0x3974_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write16(DATA + 0xa, if seed % 5 == 0 { 0x8000 } else { (r.next_u32() & 0x7fff) as u16 });
            c.bus.write16(DATA + 0x102, (r.next_u32() & 0x7fff) as u16);
        });
    }

    #[test]
    fn uianim_get_frame_count_matches() {
        check(ssx3_ee::s13::f_00397458, uianim_get_frame_count, 200, &|c, seed| {
            let mut r = Rng(0x3974_5800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write16(DATA + 0x102, if seed % 5 == 0 { 0x8000 } else { (r.next_u32() & 0x7fff) as u16 });
        });
    }

    #[test]
    fn uianim_set_play_mode_matches() {
        check(ssx3_ee::s13::f_00397468, uianim_set_play_mode, 200, &|c, seed| {
            let mut r = Rng(0x3974_6800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((seed % 3) as u64) | ((r.next_u32() as u64) << 32);
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write16(DATA + 0x102, r.next_u32() as u16);
        });
    }

    #[test]
    fn uilist_last_matches() {
        check(ssx3_ee::s13::f_00397858, uilist_last, 200, &|c, seed| {
            let mut r = Rng(0x3978_5800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let node = DATA + 0x100;
            c.bus.write32(DATA + 0xc, if seed % 2 == 0 { node } else { 0 });
            c.bus.write32(node, if seed % 3 == 0 { node } else { r.next_u32() });
        });
    }

    #[test]
    fn uilist_push_front_matches() {
        check(ssx3_ee::s13::f_003978f0, uilist_push_front, 200, &|c, seed| {
            let mut r = Rng(0x3978_f000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let node = DATA + 0x200;
            c.bus.write32(DATA + 4, node);
            c.bus.write32(node, r.next_u32());
            c.bus.write32(DATA + 0x100, r.next_u32());
            c.bus.write32(DATA + 0x104, r.next_u32());
        });
    }

    #[test]
    fn uilist_push_back_matches() {
        check(ssx3_ee::s13::f_00397910, uilist_push_back, 200, &|c, seed| {
            let mut r = Rng(0x3979_1000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let node = DATA + 0x200;
            c.bus.write32(DATA + 0xc, node);
            c.bus.write32(node + 4, r.next_u32());
            c.bus.write32(DATA + 0x100, r.next_u32());
            c.bus.write32(DATA + 0x104, r.next_u32());
        });
    }

    #[test]
    fn uilist_reset_matches() {
        check(ssx3_ee::s13::f_00397930, uilist_reset, 200, &|c, seed| {
            let mut r = Rng(0x3979_3000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            for k in 0..5 {
                c.bus.write32(DATA + k * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn uifont_set_add_font_matches() {
        check_lohi(ssx3_ee::s13::f_003984b0, uifont_set_add_font, 200, &|c, seed| {
            let mut r = Rng(0x3984_b000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 5) as u64;
            c.g[A2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            c.g[A3][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            c.bus.write8(DATA + 0x34, if seed % 4 == 0 { 4 } else { (seed % 4) as u8 });
        });
    }

    /// Shared setup for the two `UIListBox_GetSelected*` accessors.
    fn listbox_selected_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x39a7_0000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        let (count, sel) = match seed % 4 {
            0 => (0u8, 5u8),
            1 => (8, 0),
            2 => (8, 8),
            _ => (8, 3),
        };
        c.bus.write8(DATA + 0x318, count);
        c.bus.write8(DATA + 0x319, sel);
        for k in 0..10 {
            c.bus.write32(DATA + k * 0x14 + 0xa0, r.next_u32());
        }
    }

    #[test]
    fn uilistbox_get_selected_data_matches() {
        check_lohi(ssx3_ee::s13::f_0039a708, uilistbox_get_selected_data, 200, &listbox_selected_setup);
    }

    #[test]
    fn uilistbox_get_selected_value_matches() {
        check_lohi(ssx3_ee::s13::f_0039a738, uilistbox_get_selected_value, 200, &listbox_selected_setup);
    }

    #[test]
    fn uilistbox_set_selection_matches() {
        check(ssx3_ee::s13::f_0039a7a8, uilistbox_set_selection, 200, &|c, seed| {
            let mut r = Rng(0x39a7_a800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | (seed % 10) as u64;
            c.bus.write8(DATA + 0x318, (r.next_u32() % 10) as u8);
        });
    }

    #[test]
    fn uimenu_set_nav_buttons_matches() {
        check(ssx3_ee::s13::f_0039ab00, uimenu_set_nav_buttons, 200, &|c, seed| {
            let mut r = Rng(0x39ab_0000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | if seed % 2 == 0 { 0 } else { (seed % 7) as u64 + 1 };
            for k in 0x91..=0x94 {
                c.bus.write8(DATA + k, r.next_u32() as u8);
            }
        });
    }

    #[test]
    fn uiscreen_cmd_stop_thread_matches() {
        check(ssx3_ee::s13::f_0039cbc8, uiscreen_cmd_stop_thread, 200, &|c, seed| {
            let mut r = Rng(0x39cb_c800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A3][0] = DATA as u64;
            c.bus.write32(DATA + 0x10, r.next_u32());
        });
    }

    #[test]
    fn uiscreen_cmd_end_transition_in_matches() {
        check(ssx3_ee::s13::f_0039ce20, uiscreen_cmd_end_transition_in, 200, &|c, seed| {
            let mut r = Rng(0x39ce_2000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0xd0, 0);
            } else {
                let screen = DATA + 0x400;
                c.bus.write32(DATA + 0xd0, screen);
                c.bus.write32(screen + 0x1c, r.next_u32());
            }
        });
    }

    #[test]
    fn uiscreen_cmd_end_transition_out_matches() {
        check(ssx3_ee::s13::f_0039ce98, uiscreen_cmd_end_transition_out, 200, &|c, seed| {
            let mut r = Rng(0x39ce_9800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A3][0] = (DATA + 0x800) as u64;
            c.bus.write32(DATA + 0x810, r.next_u32());
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0xd0, 0);
            } else {
                let screen = DATA + 0x400;
                c.bus.write32(DATA + 0xd0, screen);
                c.bus.write32(screen + 0x1c, r.next_u32());
            }
        });
    }

    #[test]
    fn uiobject_apply_alignment_matches() {
        check(ssx3_ee::s13::f_0039feb8, uiobject_apply_alignment, 300, &|c, seed| {
            let mut r = Rng(0x39fe_b800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA + 0x14, (r.next_u32() & 0x3f) << 13);
            c.bus.write32(DATA + 0x100, r.edge_f32());
            c.bus.write32(DATA + 0x104, r.edge_f32());
            c.bus.write32(DATA + 0x200, r.edge_f32());
            c.bus.write32(DATA + 0x204, r.edge_f32());
        });
    }

    #[test]
    fn uiobject_find_by_id_matches() {
        check(ssx3_ee::s13::f_0039ff60, uiobject_find_by_id, 300, &|c, seed| {
            let mut r = Rng(0x39ff_6000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            let id = r.next_u32();
            c.g[A1][0] = if seed % 3 == 0 { id as u64 } else { (id as u64) | ((r.next_u32() as u64) << 32) };
            c.bus.write32(DATA + 0x38, id);
        });
    }

    #[test]
    fn uiobject_get_event_thread_matches() {
        check(ssx3_ee::s13::f_0039ff78, uiobject_get_event_thread, 200, &|c, seed| {
            let mut r = Rng(0x39ff_7800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | (seed % 8) as u64;
            let table = DATA + 0x400;
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0x3c, 0);
            } else {
                c.bus.write32(DATA + 0x3c, table);
                for k in 0..20 {
                    c.bus.write16(table + k * 2, r.next_u32() as u16);
                }
            }
        });
    }

    #[test]
    fn uiobject_get_user_data_matches() {
        check(ssx3_ee::s13::f_003a0318, uiobject_get_user_data, 200, &|c, seed| {
            let mut r = Rng(0x3a03_1800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | (seed % 8) as u64;
            for k in 0..8 {
                c.bus.write32(DATA + k * 4 + 0x6c, r.next_u32());
            }
        });
    }

    #[test]
    fn uiobject_set_user_data_matches() {
        check(ssx3_ee::s13::f_003a0330, uiobject_set_user_data, 200, &|c, seed| {
            let mut r = Rng(0x3a03_3000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | (seed % 8) as u64;
            c.g[A2][0] = (r.next_u32() as u64) | (r.next_u32() as u64) << 32;
            for k in 0..8 {
                c.bus.write32(DATA + k * 4 + 0x6c, r.next_u32());
            }
        });
    }

    /// Shared setup for the two font getters (`UIText` / `cUITextScroll`).
    fn get_font_setup(c: &mut Cpu, seed: u32, at_74_is_word: bool) {
        let mut r = Rng(0x3a04_f000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        let fontlist = DATA + 0x100;
        let fonts = DATA + 0x200;
        let base = DATA + 0x400;
        c.bus.write32(DATA + 0x5c, fontlist);
        c.bus.write32(fontlist + 0xd0, fonts);
        if seed % 3 == 0 {
            c.bus.write32(fonts + 0x10, 0);
        } else {
            c.bus.write32(fonts + 0x10, fonts);
            c.bus.write32(fonts + 0x8, base);
        }
        if at_74_is_word {
            c.bus.write32(DATA + 0x74, (r.next_u32() << 3) | (r.next_u32() & 0x7));
        } else {
            c.bus.write8(DATA + 0x74, (r.next_u32() & 0xff) as u8);
        }
        for k in 0..4 {
            c.bus.write32(base + k * 12 + 0xc, r.next_u32());
        }
    }

    #[test]
    fn uitext_get_font_matches() {
        check_lohi(ssx3_ee::s13::f_003a04f0, uitext_get_font, 200, &|c, seed| {
            get_font_setup(c, seed, false)
        });
    }

    #[test]
    fn c_ui_text_scroll_get_font_matches() {
        check_lohi(ssx3_ee::s13::f_003a12d0, c_ui_text_scroll_get_font, 200, &|c, seed| {
            get_font_setup(c, seed, true)
        });
    }

    #[test]
    fn c_ui_text_scroll_set_nav_buttons_matches() {
        check(ssx3_ee::s13::f_003a1310, c_ui_text_scroll_set_nav_buttons, 200, &|c, seed| {
            let mut r = Rng(0x3a13_1000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | if seed % 2 == 0 { 0 } else { (seed % 7) as u64 + 1 };
            for k in 0x88..=0x8b {
                c.bus.write8(DATA + k, r.next_u32() as u8);
            }
        });
    }

    #[test]
    fn c_ui_text_scroll_get_line_matches() {
        check(ssx3_ee::s13::f_003a2068, c_ui_text_scroll_get_line, 300, &|c, seed| {
            let mut r = Rng(0x3a20_6800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let lines = DATA + 0x400;
            c.g[A1][0] = match seed % 4 {
                0 => (r.next_u32() as u64) | 0x8000_0000_0000_0000, // negative
                1 => 0,
                2 => 20, // out of range below
                _ => (seed % 10) as u64,
            };
            if seed % 5 == 0 {
                c.bus.write32(DATA + 0xbc, 0);
            } else {
                c.bus.write32(DATA + 0xbc, lines);
                for k in 0..12 {
                    c.bus.write32(lines + k * 4, r.next_u32());
                }
            }
            c.bus.write16(DATA + 0xc8, 10);
        });
    }

    #[test]
    fn cstream_man_set_rate_matches() {
        check(ssx3_ee::s13::f_003a7058, cstream_man_set_rate, 200, &|c, seed| {
            let mut r = Rng(0x3a70_5800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = match seed % 3 {
                0 => (seed % 100) as u64,
                1 => 0x8000,
                _ => (r.next_u32() as u64) | ((r.next_u32() as u64) << 32),
            };
            c.bus.write32(GAME_GP + 0x2a74, DATA + 0x400);
            c.bus.write32(DATA + 0x410, r.next_u32());
        });
    }

    #[test]
    fn world_block_pool_alloc_block_matches() {
        check(ssx3_ee::s13::f_003a7768, world_block_pool_alloc_block, 200, &|c, seed| {
            let mut r = Rng(0x3a77_6800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let block = DATA + 0x400;
            c.bus.write32(DATA + 0x10, block);
            c.bus.write32(DATA + 0x8, r.next_u32());
            c.bus.write32(block + 0x14, r.next_u32());
            c.bus.write32(block + 0x10, r.next_u32());
        });
    }
}
