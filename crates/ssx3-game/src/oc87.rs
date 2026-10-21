//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for UI widget flags and vec3 accessors, camera tuning blocks,
//! graphics-manager accessors and small codec/list helpers, each with a
//! differential test against the generated body.
//!
//! - **UI flags** (`UIObject_*`, `UIText_v*`, ...): the eight `bitN: 1`
//!   fields of the widget flag words at `+0x14` and `+0x74`
//!   (`sUIVectorFlags` in the reference): read the word, mask one bit out,
//!   OR the boolean in, store it back.
//! - **UI vectors** (`UIVector_v20` / `_v21`): the x/y extent of the point
//!   array at `+0x80`, and the four-corner "fan out from corner 0" helper.
//! - **Camera tuning** (`*ChaseCameraAlgorithm_GetTuning`,
//!   `GetEyeDrop`): the four tuning floats plus the `1` flag of a
//!   `gp`-relative per-algorithm parameter block.
//! - **Graphics manager** (`PS2GraphicsMan_*`): display mode info, viewport
//!   (float→int→float round trip), fov/near/far/aspect, fog mode/params,
//!   texture validity/size from `tex0`, the VU0 box clip, the corner
//!   projector, and the view-matrix copy.
//!
//! The replacements use [`Cpu`]'s own load/store, FPU and COP2 operations in
//! the same order as the generated body, so registers, cycles, memory and
//! VU0 state end as the original leaves them. `PS2_SONY_CODEC_INTERNAL_NodataCallback`
//! is tested with a fixture that keeps its callee on the short "no bitstream
//! data" path (see the comment on the test).

use crate::abi;
use crate::call::{guest, Callee};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_8d50, mid_left_chase_camera_algorithm_get_tuning, "MidLeftChaseCameraAlgorithm_GetTuning"),
    (0x0017_6358, script_camera_algo_get_look_at, "ScriptCameraAlgo_GetLookAt"),
    (0x0017_6808, camera_point_bound_obj_clamp_point, "CameraPointBoundObj_ClampPoint"),
    (0x0017_6ad0, default_2_chase_camera_algorithm_get_eye_drop, "Default_2ChaseCameraAlgorithm_GetEyeDrop"),
    (0x0017_6ae0, default_2_chase_camera_algorithm_get_tuning, "Default_2ChaseCameraAlgorithm_GetTuning"),
    (0x0017_6dd0, default_3_chase_camera_algorithm_get_eye_drop, "Default_3ChaseCameraAlgorithm_GetEyeDrop"),
    (0x0017_6de0, default_3_chase_camera_algorithm_get_tuning, "Default_3ChaseCameraAlgorithm_GetTuning"),
    (0x0017_70d0, default_4_chase_camera_algorithm_get_eye_drop, "Default_4ChaseCameraAlgorithm_GetEyeDrop"),
    (0x0017_70e0, default_4_chase_camera_algorithm_get_tuning, "Default_4ChaseCameraAlgorithm_GetTuning"),
    (0x0017_7558, direction_ne_chase_camera_algorithm_get_tuning, "DirectionNeChaseCameraAlgorithm_GetTuning"),
    (0x0017_7eb8, spoke_chase_camera_algorithm_get_eye_drop, "SpokeChaseCameraAlgorithm_GetEyeDrop"),
    (0x001c_aed8, c_ui_state_buy_popup_on_widget_notify, "cUIStateBuyPopup_OnWidgetNotify"),
    (0x0024_2978, occlusion_man_compare_by_distance, "OcclusionMan_CompareByDistance"),
    (0x0024_43f0, generic_stream_pause_v02, "GenericStreamPause_v02"),
    (0x0024_53e0, c_fe_state_progressive_on_widget_notify, "cFEStateProgressive_OnWidgetNotify"),
    (0x0037_6560, ps2_graphics_man_get_display_mode_info, "PS2GraphicsMan_GetDisplayModeInfo"),
    (0x0037_6b90, ps2_graphics_man_get_viewport, "PS2GraphicsMan_GetViewport"),
    (0x0037_6c10, ps2_graphics_man_get_fov, "PS2GraphicsMan_GetFov"),
    (0x0037_6c28, ps2_graphics_man_get_near_far_aspect, "PS2GraphicsMan_GetNearFarAspect"),
    (0x0037_7950, ps2_graphics_man_set_fog_mode, "PS2GraphicsMan_SetFogMode"),
    (0x0037_79e0, ps2_graphics_man_set_fog_params, "PS2GraphicsMan_SetFogParams"),
    (0x0037_cb90, ps2_graphics_man_is_texture_valid, "PS2GraphicsMan_IsTextureValid"),
    (0x0037_d758, ps2_graphics_man_add_light, "PS2GraphicsMan_AddLight"),
    (0x0037_d908, ps2_graphics_man_get_texture_width, "PS2GraphicsMan_GetTextureWidth"),
    (0x0037_d938, ps2_graphics_man_get_texture_height, "PS2GraphicsMan_GetTextureHeight"),
    (0x0037_de88, ps2_graphics_man_clip_test_box, "PS2GraphicsMan_ClipTestBox"),
    (0x0038_0380, ps2_graphics_man_v97, "PS2GraphicsMan_v97"),
    (0x0038_6688, ps2_graphics_man_add_view, "PS2GraphicsMan_AddView"),
    (0x003a_3d70, ui_vector_v20, "UIVector_v20"),
    (0x003a_3e38, ui_vector_v21, "UIVector_v21"),
    (0x003a_49e8, ui_object_v04, "UIObject_v04"),
    (0x003a_4a08, ui_object_v05, "UIObject_v05"),
    (0x003a_4a28, ui_object_v06, "UIObject_v06"),
    (0x003a_4a48, ui_object_set_focus, "UIObject_SetFocus"),
    (0x003a_4a68, ui_object_v08, "UIObject_v08"),
    (0x003a_4a88, ui_object_set_visible, "UIObject_SetVisible"),
    (0x003a_4af8, ui_object_set_color, "UIObject_SetColor"),
    (0x003a_4c90, ui_object_get_vec3_at60, "UIObject_GetVec3At60"),
    (0x003a_4cb0, ui_object_set_vec3_at60, "UIObject_SetVec3At60"),
    (0x003a_5708, ui_list_box_v24, "UIListBox_v24"),
    (0x003a_5e68, ui_text_v23, "UIText_v23"),
    (0x003a_5e88, ui_text_v24, "UIText_v24"),
    (0x003a_5ea8, ui_text_v25, "UIText_v25"),
    (0x003a_5ec8, ui_text_v26, "UIText_v26"),
    (0x003a_5ee8, ui_text_v27, "UIText_v27"),
    (0x003a_6378, c_ui_text_scroll_v23, "cUITextScroll_v23"),
    (0x003a_6398, c_ui_text_scroll_v24, "cUITextScroll_v24"),
    (0x003b_0b10, ps2_sony_codec_internal_nodata_callback, "PS2_SONY_CODEC_INTERNAL_NodataCallback"),
    (0x003b_0fa8, ps2_sony_codec_internal_v06, "PS2_SONY_CODEC_INTERNAL_v06"),
    (0x003b_1140, ps2_sony_codec_internal_release_frame, "PS2_SONY_CODEC_INTERNAL_ReleaseFrame"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    /// `PS2_SONY_CODEC_INTERNAL_FeedBitstream` (only used by
    /// `PS2_SONY_CODEC_INTERNAL_NodataCallback`).
    pub const FEED_BITSTREAM: Callee = Callee { addr: 0x003b_0b40, f: ee::s13::PS2_SONY_CODEC_INTERNAL_FeedBitstream };
}

// ---------------------------------------------------------------------------
// Field offsets.

/// The UI widget flag words: `UIObject` at `+0x14`, its subclasses at `+0x74`.
const UI_FLAGS_OBJECT: u32 = 0x14;
const UI_FLAGS_WIDGET: u32 = 0x74;

/// `sVec3` at `+0x60` of a UI object, `+0x1c` the colour, `+0x80` the point
/// array, `+0x74` the `{lo:17, count:15}` size field.
const UI_VEC3: u32 = 0x60;
const UI_COLOR: u32 = 0x1c;
const UI_POINTS: u32 = 0x80;
const UI_SIZE: u32 = 0x74;

/// `PS2GraphicsMan` fields.
const GM_DISPLAY_MODE: u32 = 0x10dc;
const GM_VIEWPORTS: u32 = 0x6d20;
const GM_VIEWPORT_STRIDE: u32 = 0x230;
const GM_FOV: u32 = 0x6d38;
const GM_FOG_MODE: u32 = 0x6b94;
const GM_FOG_NEAR: u32 = 0x6b98;
const GM_FOG_FAR: u32 = 0x6b9c;
const GM_FOG_COLOR_A: u32 = 0x6ba0;
const GM_FOG_COLOR_B: u32 = 0x6ba4;
const GM_TEXTURES: u32 = 0x18f4;
const GM_LIGHTS: u32 = 0x13ec;
const GM_LIGHT_STRIDE: u32 = 0xa0;
const GM_VIEW_MATRICES: u32 = 0x6af0;
const GM_VIEW_ENTRIES: u32 = 0x5930;
const GM_VIEW_PAIRS: u32 = 0x59b0;
const GM_VIEW_PAIR_SRC: u32 = 0x5900;
const GM_VIEW_COUNT: u32 = 0x59c0;
const GM_PHASE2_TEX: u32 = 0x1440;
const GM_PHASE2_VTX: u32 = 0x1400;
const GM_PHASE2_Q: u32 = 0x13f0;
const GM_PHASE2_F: u32 = 0x1480;
const GM_PHASE2_I: u32 = 0x1484;

// ---------------------------------------------------------------------------
// Small helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// The low 32 bits of a register (a pointer).
#[inline(always)]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r(c, GP).wrapping_add(off as u32)
}

/// `x + y` as `addu` leaves it.
#[inline(always)]
fn add32(x: u64, y: u64) -> u64 {
    sx((x as u32).wrapping_add(y as u32))
}

/// `lui at, bits; mtc1 at, f` — a float constant through `at`.
#[inline(always)]
fn fconst(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = bits as u64;
    c.f[f] = bits;
}

// ---------------------------------------------------------------------------
// UI widget flags.

/// A `bitN: 1` setter: `w = *(self + off); w = (w & !bit) | (enable != 0) << n;
/// *(self + off) = w`, leaving the word in `v0` and the mask/value in `v1`/`a1`
/// as the original does.
fn set_ui_flag(c: &mut Cpu, off: u32, bit: u32) {
    c.cycles += 8;
    let ui = abi::arg(c, 0);
    c.g[V0][0] = c.lw(p(ui, off));
    c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
    c.g[V1][0] = sx(!(1u32 << bit));
    c.g[A1][0] = sx((c.g[A1][0] as u32) << bit);
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= c.g[A1][0];
    let ra = r(c, RA);
    c.sw(p(ui, off), c.g[V0][0]);
    c.pc = ra;
}

/// `UIObject_v04` (0x003a49e8): `bit1 = enable` of the flag word at `+0x14`.
pub fn ui_object_v04(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 1);
}

/// `UIObject_v05` (0x003a4a08): `bit2 = enable`.
pub fn ui_object_v05(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 2);
}

/// `UIObject_v06` (0x003a4a28): `bit3 = enable`.
pub fn ui_object_v06(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 3);
}

/// `UIObject_SetFocus` (0x003a4a48): `bit4 = enable` (the focused state).
pub fn ui_object_set_focus(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 4);
}

/// `UIObject_v08` (0x003a4a68): `bit5 = enable`.
pub fn ui_object_v08(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 5);
}

/// `UIObject_SetVisible` (0x003a4a88): `bit6 = enable` (the visible state).
pub fn ui_object_set_visible(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_OBJECT, 6);
}

/// `UIListBox_v24` (0x003a5708): `bit5 = enable` of the widget flag word at
/// `+0x74`.
pub fn ui_list_box_v24(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 5);
}

/// `UIText_v23` (0x003a5e68): `bit2 = enable` of the widget flag word.
pub fn ui_text_v23(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 2);
}

/// `UIText_v24` (0x003a5e88): `bit3 = enable`.
pub fn ui_text_v24(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 3);
}

/// `UIText_v25` (0x003a5ea8): `bit4 = enable`.
pub fn ui_text_v25(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 4);
}

/// `UIText_v26` (0x003a5ec8): `bit5 = enable`.
pub fn ui_text_v26(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 5);
}

/// `UIText_v27` (0x003a5ee8): `bit6 = enable`.
pub fn ui_text_v27(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 6);
}

/// `cUITextScroll_v23` (0x003a6378): `bit3 = enable`.
pub fn c_ui_text_scroll_v23(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 3);
}

/// `cUITextScroll_v24` (0x003a6398): `bit4 = enable`.
pub fn c_ui_text_scroll_v24(c: &mut Cpu) {
    set_ui_flag(c, UI_FLAGS_WIDGET, 4);
}

// ---------------------------------------------------------------------------
// UI vec3 accessors and colour.

/// `UIObject_SetColor` (0x003a4af8): `*(self + 0x1c) = *in` (16 bytes).
pub fn ui_object_set_color(c: &mut Cpu) {
    c.cycles += 10;
    let ui = abi::arg(c, 0);
    let color = abi::arg(c, 1);
    c.ldl(2, p(color, 7));
    c.ldr(2, color);
    c.ldl(3, p(color, 0xf));
    c.ldr(3, p(color, 8));
    c.sdl(2, p(ui, UI_COLOR + 7));
    c.sdr(2, p(ui, UI_COLOR));
    c.sdl(3, p(ui, UI_COLOR + 0xf));
    c.sdr(3, p(ui, UI_COLOR + 8));
    abi::ret(c);
}

/// `UIObject_GetVec3At60` (0x003a4c90): `*out = *(sVec3*)(self + 0x60)`.
pub fn ui_object_get_vec3_at60(c: &mut Cpu) {
    c.cycles += 8;
    let ui = abi::arg(c, 0);
    let out = abi::arg(c, 1);
    c.ldl(2, p(ui, UI_VEC3 + 7));
    c.ldr(2, p(ui, UI_VEC3));
    c.g[V1][0] = c.lw(p(ui, UI_VEC3 + 8));
    c.sdl(2, p(out, 7));
    c.sdr(2, out);
    c.sw(p(out, 8), c.g[V1][0]);
    abi::ret(c);
}

/// `UIObject_SetVec3At60` (0x003a4cb0): `*(sVec3*)(self + 0x60) = *in`.
pub fn ui_object_set_vec3_at60(c: &mut Cpu) {
    c.cycles += 8;
    let ui = abi::arg(c, 0);
    let v = abi::arg(c, 1);
    c.ldl(2, p(v, 7));
    c.ldr(2, v);
    c.g[V1][0] = c.lw(p(v, 8));
    c.sdl(2, p(ui, UI_VEC3 + 7));
    c.sdr(2, p(ui, UI_VEC3));
    c.sw(p(ui, UI_VEC3 + 8), c.g[V1][0]);
    abi::ret(c);
}

/// `UIVector_v20` (0x003a3d70): `getBounds(out)` — copy the default 12-byte
/// extent to `out`, then grow it to the x/y extent of the `count = size>>17`
/// points at `+0x80` (1000 / -1000 sentinels). No data: return at once.
pub fn ui_vector_v20(c: &mut Cpu) {
    c.cycles += 3;
    let vec = abi::arg(c, 0);
    let out = abi::arg(c, 1);
    c.g[V0][0] = c.lw(p(vec, UI_POINTS));
    let no_points = c.g[V0][0] == 0;
    c.g[V0][0] = 0x0050_0000;
    if no_points {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    // `D_004FF0D8`, the 12-byte default extent.
    c.cycles += 17;
    let default_extent = 0x0050_0000u32.wrapping_sub(0xf28);
    c.g[AT][0] = 0x447a_0000;
    c.mtc1(1, 3); // f3 = 1000.0
    c.g[T0][0] = sx(default_extent);
    c.ldl(3, p(default_extent, 7));
    c.ldr(3, default_extent);
    c.g[A2][0] = c.lw(p(default_extent, 8));
    c.sdl(3, p(out, 7));
    c.sdr(3, out);
    c.sw(p(out, 8), c.g[A2][0]);
    c.g[AT][0] = sx(0xc47a_0000);
    c.mtc1(1, 0); // f0 = -1000.0
    c.g[V1][0] = 0;
    c.g[V0][0] = c.lw(p(vec, UI_SIZE));
    c.mov_s(4, 3); // f4 = 1000.0
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 17);
    c.mov_s(1, 0); // f1 = -1000.0
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A0][0] = c.lw(p(vec, UI_POINTS));
        c.g[A2][0] = c.g[V0][0];
        loop {
            c.cycles += 5;
            c.lwc1(2, r(c, A0));
            c.c_lt_s(2, 3);
            if c.fcc() {
                c.mov_s(3, 2); // minX = x
            }
            c.cycles += 4;
            c.c_lt_s(0, 2);
            if c.fcc() {
                c.mov_s(0, 2); // maxX = x
            }
            c.cycles += 5;
            c.lwc1(2, p(r(c, A0), 4));
            c.c_lt_s(2, 4);
            if c.fcc() {
                c.mov_s(4, 2); // minY = y
            }
            c.cycles += 4;
            c.c_lt_s(1, 2);
            if c.fcc() {
                c.mov_s(1, 2); // maxY = y
            }
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_3dc8);
            }
        }
    }
    c.cycles += 4;
    c.sub_s(0, 0, 3); // maxX - minX
    c.sub_s(1, 1, 4); // maxY - minY
    c.swc1(0, out);
    c.swc1(1, p(out, 4));
    c.cycles += 2;
    abi::ret(c);
}

/// `UIVector_v21` (0x003a3e38): when the vector holds exactly four points,
/// fan them out from point 0 and add the x/y offset pair to points 1/3 and
/// 2; then `*(self + 0x60) = *off`. The `{lo:17, count:15}` field at `+0x74`
/// must read 4.
pub fn ui_vector_v21(c: &mut Cpu) {
    c.cycles += 7;
    let vec = abi::arg(c, 0);
    let off = abi::arg(c, 1);
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = sx(0xfffe_0000);
    c.g[V1][0] = c.lw(p(vec, UI_SIZE));
    c.g[A0][0] = 0x8_0000;
    c.g[V1][0] &= c.g[V0][0];
    let not_four = c.g[V1][0] != c.g[A0][0];
    c.g[T0][0] = 0x0200_0000;
    if !not_four {
        c.cycles += 3;
        c.g[A3][0] = 0xc;
        c.g[T1][0] = 0x0100_0000;
        loop {
            c.cycles += 14;
            c.g[V0][0] = c.lw(p(vec, UI_POINTS));
            c.g[V1][0] = c.g[T0][0];
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(r(c, V0)));
            c.g[V1][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
            c.ldl(10, p(r(c, V0), 7));
            c.ldr(10, r(c, V0));
            c.g[T3][0] = c.lw(p(r(c, V0), 8));
            c.sdl(10, p(r(c, A0), 7));
            c.sdr(10, r(c, A0));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
            let more = c.g[V1][0] != 0;
            c.sw(p(r(c, A0), 8), c.g[T3][0]);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003a_3e60);
                }
                continue;
            }
            c.cycles += 20;
            c.g[V0][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, off);
            c.lwc1(0, p(r(c, V0), 0xc));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V0), 0xc));
            c.g[V1][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, p(off, 4));
            c.lwc1(0, p(r(c, V1), 0x28));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V1), 0x28));
            c.g[V0][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, off);
            c.lwc1(0, p(r(c, V0), 0x18));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V0), 0x18));
            c.g[V1][0] = c.lw(p(vec, UI_POINTS));
            c.lwc1(1, p(off, 4));
            c.lwc1(0, p(r(c, V1), 0x1c));
            c.add_s(0, 0, 1);
            c.swc1(0, p(r(c, V1), 0x1c));
            break;
        }
    }
    c.cycles += 8;
    c.ldl(2, p(off, 7));
    c.ldr(2, off);
    c.g[V1][0] = c.lw(p(off, 8));
    c.sdl(2, p(vec, UI_VEC3 + 7));
    c.sdr(2, p(vec, UI_VEC3));
    c.sw(p(vec, UI_VEC3 + 8), c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Camera tuning blocks.

/// The four tuning floats and the `1` flag block at `gp + base`:
/// `y, x, w, z` in address order, written to `out + 4, 0, 0xc, 8`, plus the
/// `1` at `out + 0x10`.
fn write_tuning(c: &mut Cpu, out: u32, base: i32) {
    c.cycles += 11;
    c.lwc1(3, gp(c, base));
    c.g[V0][0] = 1;
    c.lwc1(2, gp(c, base + 4));
    c.lwc1(0, gp(c, base + 8));
    c.lwc1(1, gp(c, base + 0xc));
    c.sw(p(out, 0x10), 1);
    c.swc1(3, p(out, 4));
    c.swc1(2, out);
    c.swc1(0, p(out, 0xc));
    c.swc1(1, p(out, 8));
    abi::ret(c);
}

/// The single float at `gp + base` returned in `f0` (the camera's eye drop).
fn return_eye_drop(c: &mut Cpu, base: i32) {
    c.cycles += 3;
    c.lwc1(0, gp(c, base));
    abi::ret(c);
}

/// `MidLeftChaseCameraAlgorithm_GetTuning` (0x00168d50).
pub fn mid_left_chase_camera_algorithm_get_tuning(c: &mut Cpu) {
    write_tuning(c, abi::arg(c, 1), -0x6608);
}

/// `Default_2ChaseCameraAlgorithm_GetTuning` (0x00176ae0).
pub fn default_2_chase_camera_algorithm_get_tuning(c: &mut Cpu) {
    write_tuning(c, abi::arg(c, 1), -0x61e8);
}

/// `Default_3ChaseCameraAlgorithm_GetTuning` (0x00176de0).
pub fn default_3_chase_camera_algorithm_get_tuning(c: &mut Cpu) {
    write_tuning(c, abi::arg(c, 1), -0x6130);
}

/// `Default_4ChaseCameraAlgorithm_GetTuning` (0x001770e0).
pub fn default_4_chase_camera_algorithm_get_tuning(c: &mut Cpu) {
    write_tuning(c, abi::arg(c, 1), -0x6074);
}

/// `DirectionNeChaseCameraAlgorithm_GetTuning` (0x00177558).
pub fn direction_ne_chase_camera_algorithm_get_tuning(c: &mut Cpu) {
    write_tuning(c, abi::arg(c, 1), -0x5f48);
}

/// `Default_2ChaseCameraAlgorithm_GetEyeDrop` (0x00176ad0): returns the
/// camera's eye-drop height (61.688).
pub fn default_2_chase_camera_algorithm_get_eye_drop(c: &mut Cpu) {
    return_eye_drop(c, -0x61ec);
}

/// `Default_3ChaseCameraAlgorithm_GetEyeDrop` (0x00176dd0).
pub fn default_3_chase_camera_algorithm_get_eye_drop(c: &mut Cpu) {
    return_eye_drop(c, -0x6134);
}

/// `Default_4ChaseCameraAlgorithm_GetEyeDrop` (0x001770d0).
pub fn default_4_chase_camera_algorithm_get_eye_drop(c: &mut Cpu) {
    return_eye_drop(c, -0x6078);
}

/// `SpokeChaseCameraAlgorithm_GetEyeDrop` (0x00177eb8).
pub fn spoke_chase_camera_algorithm_get_eye_drop(c: &mut Cpu) {
    return_eye_drop(c, -0x5e20);
}

// ---------------------------------------------------------------------------
// Camera object helpers.

/// `ScriptCameraAlgo_GetLookAt` (0x00176358): `*self = *(cQuad128*)(src + 0x20)`,
/// returning `self`.
pub fn script_camera_algo_get_look_at(c: &mut Cpu) {
    c.cycles += 4;
    let self_ = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.lq(3, p(src, 0x20));
    c.g[V0][0] = c.g[A0][0];
    c.sq(3, self_);
    abi::ret(c);
}

/// `CameraPointBoundObj_ClampPoint` (0x00176808): `*out = {in->x, in->y,
/// in->z, 1.0}` through the stack, returning the out pointer.
pub fn camera_point_bound_obj_clamp_point(c: &mut Cpu) {
    c.cycles += 14;
    let v = abi::arg(c, 0);
    let out = abi::arg(c, 1);
    c.lwc1(2, p(v, 8));
    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
    c.lwc1(1, p(v, 4));
    c.lwc1(0, v);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3); // f3 = 1.0
    c.swc1(0, r(c, SP));
    c.swc1(1, p(r(c, SP), 4));
    c.swc1(2, p(r(c, SP), 8));
    c.swc1(3, p(r(c, SP), 0xc));
    c.lq(2, r(c, SP));
    c.sq(2, out);
    c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Graphics manager.

/// `PS2GraphicsMan_GetDisplayModeInfo` (0x00376560): the display-mode record
/// for mode `idx` (0xa4 bytes each) after the pointer at `+0x59cc`.
pub fn ps2_graphics_man_get_display_mode_info(c: &mut Cpu) {
    c.cycles += 5;
    let gm = abi::arg(c, 0);
    let idx = abi::arg(c, 1);
    c.g[V1][0] = 0xa4;
    c.g[V0][0] = c.lw(p(gm, 0x59cc));
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = add32(c.g[V0][0], c.g[A1][0]);
    abi::ret(c);
}

/// `PS2GraphicsMan_GetViewport` (0x00376b90): copy viewport `mode`'s four
/// floats (0x230-byte records at `+0x6d20`) to `out` as integers converted
/// back to floats. The manager is `a1` and `out` is `a0`, as the original
/// reads them.
pub fn ps2_graphics_man_get_viewport(c: &mut Cpu) {
    c.cycles += 31;
    let out = abi::arg(c, 0);
    let gm = abi::arg(c, 1);
    c.g[V1][0] = c.lw(p(gm, GM_DISPLAY_MODE));
    c.g[A2][0] = GM_VIEWPORT_STRIDE as u64;
    c.g[V0][0] = c.g[A0][0];
    c.mult(0, V1, V1, A2);
    c.g[V1][0] = add32(c.g[V1][0], GM_VIEWPORTS as u64);
    c.g[A1][0] = add32(gm as u64, c.g[V1][0]);
    let vp = r(c, A1);
    c.lwc1(0, p(vp, 0xc));
    c.lwc1(2, vp);
    c.cvt_w_s(1, 0);
    c.mfc1(7, 1);
    c.lwc1(3, p(vp, 4));
    c.lwc1(1, p(vp, 8));
    c.cvt_w_s(0, 2);
    c.mfc1(3, 0);
    c.cvt_w_s(0, 3);
    c.mfc1(5, 0);
    c.cvt_w_s(0, 1);
    c.mfc1(6, 0);
    c.mtc1(3, 0);
    c.cvt_s_w(0, 0);
    c.mtc1(5, 1);
    c.cvt_s_w(1, 1);
    c.mtc1(6, 2);
    c.cvt_s_w(2, 2);
    c.mtc1(7, 3);
    c.cvt_s_w(3, 3);
    c.swc1(0, out);
    c.swc1(1, p(out, 4));
    c.swc1(2, p(out, 8));
    c.swc1(3, p(out, 0xc));
    abi::ret(c);
}

/// `PS2GraphicsMan_GetFov` (0x00376c10): the field of view of the current
/// display mode (float at `+0x6d38` of its viewport record), returned in `f0`.
pub fn ps2_graphics_man_get_fov(c: &mut Cpu) {
    c.cycles += 6;
    let gm = abi::arg(c, 0);
    c.g[V0][0] = c.lw(p(gm, GM_DISPLAY_MODE));
    c.g[V1][0] = GM_VIEWPORT_STRIDE as u64;
    c.mult(0, A1, V0, V1);
    c.g[V0][0] = add32(c.g[A1][0], c.g[A0][0]);
    c.lwc1(0, p(r(c, V0), GM_FOV));
    abi::ret(c);
}

/// `PS2GraphicsMan_GetNearFarAspect` (0x00376c28): the z/x/y of display mode
/// `mode`'s near/far/aspect record into `a`, `b`, `c`.
pub fn ps2_graphics_man_get_near_far_aspect(c: &mut Cpu) {
    c.cycles += 12;
    let gm = abi::arg(c, 0);
    let (out_a, out_b, out_c) = (abi::arg(c, 1), abi::arg(c, 2), abi::arg(c, 3));
    c.g[V0][0] = c.lw(p(gm, GM_DISPLAY_MODE));
    c.g[V1][0] = GM_VIEWPORT_STRIDE as u64;
    c.mult(0, V0, V0, V1);
    c.g[V0][0] = add32(c.g[V0][0], GM_VIEWPORTS as u64);
    c.g[A0][0] = add32(gm as u64, c.g[V0][0]);
    let rp = r(c, A0);
    c.lwc1(0, p(rp, 0x18));
    c.swc1(0, out_a);
    c.lwc1(1, p(rp, 0x10));
    c.swc1(1, out_b);
    c.lwc1(0, p(rp, 0x14));
    c.swc1(0, out_c);
    abi::ret(c);
}

/// `PS2GraphicsMan_SetFogMode` (0x00377950): store the fog mode and set its
/// three fog colour scales/offsets. Modes 1 and 2 have their own triples;
/// mode 0 and everything else falls back to the default white.
pub fn ps2_graphics_man_set_fog_mode(c: &mut Cpu) {
    c.cycles += 3;
    let gm = abi::arg(c, 0);
    let mode = abi::arg64(c, 1);
    c.g[V0][0] = 1;
    let is_one = mode == c.g[V0][0];
    c.sw(p(gm, GM_FOG_MODE), mode);
    if is_one {
        // Mode 1: 0.125 towards 0.75.
        c.cycles += 7;
        fconst(c, 0, 0x3f40_0000);
        fconst(c, 1, 0x3e00_0000);
        c.swc1(0, p(gm, GM_FOG_COLOR_B));
        c.swc1(1, p(gm, GM_FOG_NEAR));
        c.cycles += 3;
        c.swc1(0, p(gm, GM_FOG_FAR));
        c.swc1(0, p(gm, GM_FOG_COLOR_A));
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = ((mode as i64) < 2i64) as u64;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = 2;
        c.cycles += 2;
        if mode == c.g[V0][0] {
            // Mode 2: near 0, far/alpha 1, 0.75 in the colour slot.
            c.cycles += 9;
            fconst(c, 1, 0x3f80_0000);
            fconst(c, 0, 0x3f40_0000);
            c.swc1(1, p(gm, GM_FOG_COLOR_B));
            c.swc1(0, p(gm, GM_FOG_COLOR_A));
            c.sw(p(gm, GM_FOG_NEAR), 0);
            c.swc1(1, p(gm, GM_FOG_FAR));
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if mode == 0 {
        c.sw(p(gm, GM_FOG_NEAR), 0);
        c.cycles += 3;
        fconst(c, 0, 0x3f80_0000);
        c.swc1(0, p(gm, GM_FOG_COLOR_B));
        c.cycles += 3;
        c.swc1(0, p(gm, GM_FOG_FAR));
        c.swc1(0, p(gm, GM_FOG_COLOR_A));
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `PS2GraphicsMan_SetFogParams` (0x003779e0): `*(self + 0x6b94) = *src`
/// (20 bytes: the fog record).
pub fn ps2_graphics_man_set_fog_params(c: &mut Cpu) {
    c.cycles += 12;
    let gm = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.ldl(2, p(src, 7));
    c.ldr(2, src);
    c.ldl(3, p(src, 0xf));
    c.ldr(3, p(src, 8));
    c.g[A2][0] = c.lw(p(src, 0x10));
    c.sdl(2, p(gm, GM_FOG_MODE + 7));
    c.sdr(2, p(gm, GM_FOG_MODE));
    c.sdl(3, p(gm, GM_FOG_MODE + 0xf));
    c.sdr(3, p(gm, GM_FOG_MODE + 8));
    c.sw(p(gm, GM_FOG_MODE + 0x10), c.g[A2][0]);
    abi::ret(c);
}

/// `PS2GraphicsMan_IsTextureValid` (0x0037cb90): negative indices are always
/// valid (the default texture); indices past 1999 are not; otherwise the
/// table entry must be non-null.
pub fn ps2_graphics_man_is_texture_valid(c: &mut Cpu) {
    c.cycles += 2;
    let gm = abi::arg(c, 0);
    let idx = abi::arg64(c, 1);
    let negative = (idx as i64) < 0;
    c.g[V1][0] = c.lw(p(gm, GM_TEXTURES));
    if negative {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = ((idx as i64) < 2000i64) as u64;
    let in_range = c.g[V0][0] != 0;
    c.g[V0][0] = sx((idx as u32) << 2);
    if !in_range {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = c.lw(p(r(c, V0), 8));
    c.g[V0][0] = (0u64 < c.g[V1][0]) as u64;
    abi::ret(c);
}

/// `PS2GraphicsMan_AddLight` (0x0037d758): append a light (quad, two
/// matrices, a float, an int) to the table at `+0x13ec`/`+0x13f0`.
pub fn ps2_graphics_man_add_light(c: &mut Cpu) {
    c.cycles += 42;
    let gm = abi::arg(c, 0);
    let quad = abi::arg(c, 1);
    let m0 = abi::arg(c, 2);
    let m1 = abi::arg(c, 3);
    c.g[V1][0] = c.lw(p(gm, GM_LIGHTS));
    c.g[T2][0] = GM_LIGHT_STRIDE as u64;
    c.lq(9, quad);
    c.mult(0, V0, V1, T2);
    c.g[V1][0] = add32(c.g[V0][0], gm as u64);
    c.sq(9, p(r(c, V1), GM_PHASE2_Q));
    c.g[V0][0] = c.lw(p(gm, GM_LIGHTS));
    c.mult(0, V1, V0, T2);
    c.g[V0][0] = add32(c.g[V1][0], gm as u64);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(GM_PHASE2_VTX));
    c.lqc2(1, m0);
    c.lqc2(2, p(m0, 0x10));
    c.lqc2(3, p(m0, 0x20));
    c.lqc2(4, p(m0, 0x30));
    let vtx = r(c, V0);
    c.sqc2(1, vtx);
    c.sqc2(2, p(vtx, 0x10));
    c.sqc2(3, p(vtx, 0x20));
    c.sqc2(4, p(vtx, 0x30));
    c.g[V1][0] = c.lw(p(gm, GM_LIGHTS));
    c.mult(0, V0, V1, T2);
    c.g[V1][0] = add32(c.g[V0][0], gm as u64);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(GM_PHASE2_TEX));
    c.lqc2(1, m1);
    c.lqc2(2, p(m1, 0x10));
    c.lqc2(3, p(m1, 0x20));
    c.lqc2(4, p(m1, 0x30));
    let tex = r(c, V1);
    c.sqc2(1, tex);
    c.sqc2(2, p(tex, 0x10));
    c.sqc2(3, p(tex, 0x20));
    c.sqc2(4, p(tex, 0x30));
    c.g[V0][0] = c.lw(p(gm, GM_LIGHTS));
    c.mult(0, V1, V0, T2);
    c.g[V0][0] = add32(c.g[V1][0], gm as u64);
    c.swc1(12, p(r(c, V0), GM_PHASE2_F));
    c.g[V1][0] = c.lw(p(gm, GM_LIGHTS));
    c.mult(0, V0, V1, T2);
    c.g[V1][0] = add32(c.g[V0][0], gm as u64);
    c.sw(p(r(c, V1), GM_PHASE2_I), c.g[T0][0]);
    c.g[V0][0] = c.lw(p(gm, GM_LIGHTS));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.sw(p(gm, GM_LIGHTS), c.g[V0][0]);
    abi::ret(c);
}

/// The `tex0` T-bit field (`>> shift, & 0xf`) of texture `idx`, as
/// `1 << field`: the texture's width/height in pixels.
fn texture_size(c: &mut Cpu, shift: u32) {
    c.cycles += 12;
    let gm = abi::arg(c, 0);
    let idx = abi::arg(c, 1);
    c.g[V0][0] = c.lw(p(gm, GM_TEXTURES));
    c.g[A1][0] = sx((idx as u32) << 2);
    c.g[A0][0] = 1;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = c.lw(p(r(c, V0), 8));
    c.g[V0][0] = c.ld(p(r(c, V1), 0x38));
    c.g[V0][0] >>= shift;
    c.g[V0][0] &= 0xf;
    c.g[V0][0] <<= 32;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    c.g[V0][0] = sx((c.g[A0][0] as u32) << ((c.g[V0][0] as u32) & 31));
    abi::ret(c);
}

/// `PS2GraphicsMan_GetTextureWidth` (0x0037d908): the width (`tex0.TW`) of
/// texture `idx`.
pub fn ps2_graphics_man_get_texture_width(c: &mut Cpu) {
    texture_size(c, 26);
}

/// `PS2GraphicsMan_GetTextureHeight` (0x0037d938): the height
/// (`tex0.TH`) of texture `idx`.
pub fn ps2_graphics_man_get_texture_height(c: &mut Cpu) {
    texture_size(c, 30);
}

/// `PS2GraphicsMan_ClipTestBox` (0x0037de88): run the VU0 box-clip
/// microprogram at `D_570 >> 3` on the two points (`a1`, `a2`) and the clip
/// matrix (`a3`), then report 1 for the far side, 2 for the near side, 0 for
/// "no clip".
pub fn ps2_graphics_man_clip_test_box(c: &mut Cpu) {
    c.cycles += 18;
    let a = abi::arg(c, 1);
    let b = abi::arg(c, 2);
    let m = abi::arg(c, 3);
    c.g[V0][0] = 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x570));
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
    c.lqc2(15, a);
    c.lqc2(16, b);
    c.lqc2(10, m);
    c.lqc2(11, p(m, 0x10));
    c.lqc2(12, p(m, 0x20));
    c.lqc2(13, p(m, 0x30));
    c.ctc2(V0, 27);
    c.vu0().vnop();
    c.vu0().vnop();
    c.vcallmsr();
    c.cfc2(4, 1);
    c.cfc2(3, 2);
    let far = c.g[V1][0] != 0;
    c.g[V0][0] = 2;
    if far {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `PS2GraphicsMan_v97` (0x00380380): project an object's four corner
/// texcoords/colours into `pos` (3 floats each), `uv` (2 floats each) and
/// `col` (4 floats each), scaling the model's corner table by its scale.
///
/// `obj` (+0x80 model, +0x98 extra offset), `idx` (corner base), `pos`,
/// `uv`, `col` are passed in `a1`, `a2`, `a3`, `t0`, `t1` as the original
/// reads them.
pub fn ps2_graphics_man_v97(c: &mut Cpu) {
    c.cycles += 42;
    let obj = abi::arg(c, 1);
    let pos = abi::arg(c, 3);
    let col = abi::arg64(c, 5);
    c.g[T2][0] = c.lw(p(obj, 0x80)); // model
    c.g[A2][0] = sx((abi::arg(c, 2) as u32) << 4);
    c.g[T3][0] = col;
    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x20));
    c.g[V0][0] = c.lw(p(r(c, T2), 8)); // corner table offset
    c.g[T4][0] = 3;
    c.g[T1][0] = c.lw(p(obj, 0x98)); // extra offset
    c.g[A2][0] = add32(c.g[A2][0], c.g[V0][0]);
    c.g[A1][0] = c.lw(p(r(c, T2), 0x24)); // vertex base offset
    c.g[V0][0] = c.lw(p(r(c, A2), 4));
    c.lwc1(3, gp(c, -0x2760)); // 3.0517578e-5
    c.g[A0][0] = c.lw(p(r(c, V0), 0x20));
    c.g[AT][0] = 0x3980_0000;
    c.mtc1(1, 5); // f5 = 1/4096
    c.g[V1][0] = c.lw(r(c, A0));
    c.lwc1(4, gp(c, -0x275c)); // 0.032258
    c.g[V0][0] = c.lw(p(r(c, V1), 4));
    c.g[T1][0] = add32(c.g[V0][0], c.g[T1][0]); // data
    c.g[V0][0] = add32(c.g[V0][0], c.g[A1][0]);
    c.g[V1][0] = c.lw(p(r(c, T1), 0x14)); // colour record
    c.g[A0][0] = c.lw(p(r(c, V0), 4)); // vertex source
    c.ldl(2, p(r(c, T2), 0x1f));
    c.ldr(2, p(r(c, T2), 0x18)); // scale x, y
    c.g[A1][0] = c.lw(p(r(c, T2), 0x20)); // scale z
    c.sdl(2, p(r(c, SP), 0x17));
    c.sdr(2, p(r(c, SP), 0x10));
    c.sw(p(r(c, SP), 0x18), c.g[A1][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
    let vbase = r(c, A0);
    c.g[A1][0] = sx(vbase.wrapping_add(0xa0)); // uv source
    c.lwc1(0, p(r(c, SP), 0x10));
    c.g[A0][0] = sx(vbase.wrapping_add(0x80)); // texcoord source
    c.lwc1(1, p(r(c, SP), 0x14));
    c.lwc1(2, p(r(c, SP), 0x18));
    c.mul_s(0, 0, 3);
    c.mul_s(1, 1, 3);
    c.mul_s(2, 2, 3);
    c.swc1(0, r(c, SP)); // k.x
    c.mov_s(6, 0);
    c.mov_s(3, 1); // k.y
    c.swc1(1, p(r(c, SP), 4));
    c.swc1(2, p(r(c, SP), 8));
    c.mov_s(1, 2); // k.z
    let mut pos = pos;
    loop {
        c.cycles += 58;
        // Position: the corner shorts scaled into world units.
        c.g[V0][0] = c.lh(r(c, A1));
        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(1));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 6);
        c.swc1(0, pos);
        c.g[V0][0] = c.lh(p(r(c, A1), 2));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 3);
        c.swc1(0, p(pos, 4));
        c.g[V0][0] = c.lh(p(r(c, A1), 4));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(6));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 1);
        c.swc1(0, p(pos, 8));
        pos = pos.wrapping_add(0xc);
        c.g[A3][0] = sx(pos);
        // UV: the texcoord shorts scaled to 0..1.
        c.g[V0][0] = c.lh(r(c, A0));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 5);
        c.swc1(0, r(c, T0));
        c.g[V0][0] = c.lh(p(r(c, A0), 2));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 5);
        c.swc1(0, p(r(c, T0), 4));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(8));
        // Colour: the pack16 record's four channels unpacked.
        c.g[V0][0] = c.lhu(r(c, V1));
        c.g[V0][0] &= 0x1f;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 4);
        c.swc1(0, p(r(c, T3), 4));
        c.g[V0][0] = c.lhu(r(c, V1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
        c.g[V0][0] &= 0x1f;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 4);
        c.swc1(0, p(r(c, T3), 8));
        c.g[V0][0] = c.lhu(r(c, V1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 10);
        c.g[V0][0] &= 0x1f;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 4);
        c.swc1(0, p(r(c, T3), 0xc));
        c.g[V0][0] = c.lhu(r(c, V1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(2));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 15);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.swc1(0, r(c, T3));
        let more = (c.g[T4][0] as i64) >= 0;
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x10));
        if !more {
            c.cycles += 2;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_0428);
        }
    }
}

/// `PS2GraphicsMan_AddView` (0x00386688): copy view `idx`'s two matrices
/// from `+0x6af0` into the array at `+0x5930`, copy its pair word, and bump
/// the count.
pub fn ps2_graphics_man_add_view(c: &mut Cpu) {
    c.cycles += 21;
    let gm = abi::arg(c, 0);
    let idx = abi::arg(c, 1);
    c.g[V0][0] = sx((idx as u32) << 6);
    c.g[A2][0] = sx(gm.wrapping_add(GM_VIEW_MATRICES));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(gm));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(GM_VIEW_ENTRIES));
    let src = r(c, A2);
    let dst = r(c, V0);
    c.lqc2(1, src);
    c.lqc2(2, p(src, 0x10));
    c.lqc2(3, p(src, 0x20));
    c.lqc2(4, p(src, 0x30));
    c.sqc2(1, dst);
    c.sqc2(2, p(dst, 0x10));
    c.sqc2(3, p(dst, 0x20));
    c.sqc2(4, p(dst, 0x30));
    c.g[V1][0] = sx((idx as u32) << 3);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(gm));
    c.g[A1][0] = sx((idx as u32).wrapping_add(1));
    c.ldl(2, p(gm, GM_VIEW_PAIR_SRC + 7));
    c.ldr(2, p(gm, GM_VIEW_PAIR_SRC));
    c.sdl(2, p(r(c, V1), GM_VIEW_PAIRS + 7));
    c.sdr(2, p(r(c, V1), GM_VIEW_PAIRS));
    c.sw(p(gm, GM_VIEW_COUNT), c.g[A1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sort comparator and stream helpers.

/// `OcclusionMan_CompareByDistance` (0x00242978): -1 / 0 / 1 comparison of
/// the float at `+0x4` of each entry (a `qsort`-style comparator).
pub fn occlusion_man_compare_by_distance(c: &mut Cpu) {
    c.cycles += 6;
    let a = abi::arg(c, 0);
    let b = abi::arg(c, 1);
    c.lwc1(1, p(a, 4));
    c.lwc1(0, p(b, 4));
    c.c_lt_s(1, 0);
    let less = c.fcc();
    c.g[V0][0] = sx(0xffff_ffff);
    if less {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.c_lt_s(0, 1);
    let greater = c.fcc();
    c.g[V0][0] = 1;
    if greater {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `cUIStateBuyPopup_OnWidgetNotify` (0x001caed8): the occlusion key class
/// (`1,1` for keys below 7, `1,0` for 7..9, `1,1` for 10 and up) — 0x101 /
/// 0x100.
pub fn c_ui_state_buy_popup_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    let key = abi::arg64(c, 2);
    c.g[V1][0] = (key < 10) as u64;
    let far = c.g[V1][0] == 0;
    c.g[V0][0] = 0x101;
    if !far {
        c.cycles += 4;
        c.g[A0][0] = (key < 7) as u64;
        c.g[V1][0] = 0x101;
        c.g[V0][0] = 0x100;
        if c.g[A0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `GenericStreamPause_v02` (0x002443f0): returns its second argument.
pub fn generic_stream_pause_v02(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    abi::ret(c);
}

/// `cFEStateProgressive_OnWidgetNotify` (0x002453e0): 0x101 when the widget
/// id is 6, else 0.
pub fn c_fe_state_progressive_on_widget_notify(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] ^= 0x6;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `PS2_SONY_CODEC_INTERNAL_v06` (0x003b0fa8): the codec singleton's float
/// (`D_00509508`).
pub fn ps2_sony_codec_internal_v06(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x0051_0000;
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_sub(0x6af8));
    abi::ret(c);
}

/// `PS2_SONY_CODEC_INTERNAL_ReleaseFrame` (0x003b1140): drop a reference to a
/// frame; the last reference unlinks it from the frame ring and appends it
/// to the codec's free list.
pub fn ps2_sony_codec_internal_release_frame(c: &mut Cpu) {
    c.cycles += 5;
    let codec = abi::arg(c, 0);
    let item = abi::arg(c, 1);
    c.g[T0][0] = c.g[A1][0];
    c.g[V0][0] = c.lw(p(item, 0x10));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    let still_used = (c.g[V0][0] as i64) > 0;
    c.sw(p(item, 0x10), c.g[V0][0]);
    if !still_used {
        c.cycles += 16;
        c.g[V1][0] = sx(item.wrapping_add(8));
        c.g[A2][0] = c.lw(p(item, 8));
        c.g[A1][0] = c.lw(p(r(c, V1), 4));
        c.g[A3][0] = 0xb;
        c.g[A0][0] = sx(codec.wrapping_add(0x20));
        c.g[V0][0] = 0;
        c.sw(r(c, A1), c.g[A2][0]);
        if c.g[T0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        c.sw(p(r(c, A2), 4), c.g[A1][0]);
        c.sw(p(r(c, T0), 8), c.g[A3][0]);
        c.sw(p(r(c, V1), 4), c.g[A3][0]);
        c.g[A1][0] = c.lw(p(r(c, A0), 4));
        c.sw(r(c, A1), c.g[V0][0]);
        c.sw(p(r(c, A0), 4), c.g[V0][0]);
        c.sw(r(c, V0), c.g[A0][0]);
        c.sw(p(r(c, V0), 4), c.g[A1][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `PS2_SONY_CODEC_INTERNAL_NodataCallback` (0x003b0b10): the buy popup's
/// widget notification — hand it to the codec's bitstream feeder and report
/// success.
///
/// The test fixture keeps the callee on its short path (a chunk with no
/// bitstream data, freed through `AV_ChunkInit`, then one DMA chain entry);
/// the feeder's unconditional tail reaches `libmpeg_AddBsDmaChain`, whose
/// `FlushCache`/`DIntr`/`EIntr` calls are kernel no-ops on a fresh machine.
pub fn ps2_sony_codec_internal_nodata_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        // `func_003B0B40(a2, self, a1)`: the popup hands the notification to
        // the bitstream feeder and reports success.
        let ui = c.g[A0][0];
        let target = c.g[A1][0];
        let payload = c.g[A2][0];
        c.g[V0][0] = ui;
        c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = target;
        c.g[A0][0] = payload;
        c.sd(r(c, SP), c.g[RA][0]);
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[V1][0];
        callee::FEED_BITSTREAM.call(c, 0x003b_0b30)?;
        c.g[RA][0] = c.ld(r(c, SP));
        c.g[V0][0] = 1;
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, put_vec, scramble, Rng, DATA, GAME_GP};

    /// A fixture with `gp` set to the game's value and scrambled registers.
    fn start(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x2545_f491_4f6c_dd1d ^ (seed as u64) << 4 ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    // -------------------------------------------------------------- UI flags

    /// One shared test body for the `bitN = enable` setters.
    fn check_flag_setter(hand: GuestFn, generated: GuestFn, off: u32) {
        check(generated, hand, 200, &|c, seed| {
            let mut r = start(c, seed, off as u64);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { r.next_u32() as u64 | (r.next_u32() as u64) << 32 };
            c.bus.write32(DATA + off, r.next_u32());
        });
    }

    #[test]
    fn ui_object_v04_matches() {
        check_flag_setter(ui_object_v04, ssx3_ee::s13::f_003a49e8__generated, UI_FLAGS_OBJECT);
    }

    #[test]
    fn ui_object_v05_matches() {
        check_flag_setter(ui_object_v05, ssx3_ee::s13::f_003a4a08__generated, UI_FLAGS_OBJECT);
    }

    #[test]
    fn ui_object_v06_matches() {
        check_flag_setter(ui_object_v06, ssx3_ee::s13::f_003a4a28__generated, UI_FLAGS_OBJECT);
    }

    #[test]
    fn ui_object_set_focus_matches() {
        check_flag_setter(ui_object_set_focus, ssx3_ee::s13::f_003a4a48__generated, UI_FLAGS_OBJECT);
    }

    #[test]
    fn ui_object_v08_matches() {
        check_flag_setter(ui_object_v08, ssx3_ee::s13::f_003a4a68__generated, UI_FLAGS_OBJECT);
    }

    #[test]
    fn ui_object_set_visible_matches() {
        check_flag_setter(ui_object_set_visible, ssx3_ee::s13::f_003a4a88__generated, UI_FLAGS_OBJECT);
    }

    #[test]
    fn ui_list_box_v24_matches() {
        check_flag_setter(ui_list_box_v24, ssx3_ee::s13::f_003a5708__generated, UI_FLAGS_WIDGET);
    }

    #[test]
    fn ui_text_v23_matches() {
        check_flag_setter(ui_text_v23, ssx3_ee::s13::f_003a5e68__generated, UI_FLAGS_WIDGET);
    }

    #[test]
    fn ui_text_v24_matches() {
        check_flag_setter(ui_text_v24, ssx3_ee::s13::f_003a5e88__generated, UI_FLAGS_WIDGET);
    }

    #[test]
    fn ui_text_v25_matches() {
        check_flag_setter(ui_text_v25, ssx3_ee::s13::f_003a5ea8__generated, UI_FLAGS_WIDGET);
    }

    #[test]
    fn ui_text_v26_matches() {
        check_flag_setter(ui_text_v26, ssx3_ee::s13::f_003a5ec8__generated, UI_FLAGS_WIDGET);
    }

    #[test]
    fn ui_text_v27_matches() {
        check_flag_setter(ui_text_v27, ssx3_ee::s13::f_003a5ee8__generated, UI_FLAGS_WIDGET);
    }

    #[test]
    fn c_ui_text_scroll_v23_matches() {
        check_flag_setter(c_ui_text_scroll_v23, ssx3_ee::s13::f_003a6378__generated, UI_FLAGS_WIDGET);
    }

    #[test]
    fn c_ui_text_scroll_v24_matches() {
        check_flag_setter(c_ui_text_scroll_v24, ssx3_ee::s13::f_003a6398__generated, UI_FLAGS_WIDGET);
    }

    // ------------------------------------------------------- UI vec3 copies

    #[test]
    fn ui_object_set_color_matches() {
        check(ssx3_ee::s13::f_003a4af8__generated, ui_object_set_color, 200, &|c, seed| {
            let mut r = start(c, seed, 0x1c);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..4 {
                c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
            }
            // The unaligned pairs also read-modify-write the neighbouring
            // words of the 16-byte destination at DATA + 0x41c.
            for i in 0..6 {
                c.bus.write32(DATA + 0x418 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn ui_object_get_vec3_at60_matches() {
        check(ssx3_ee::s13::f_003a4c90__generated, ui_object_get_vec3_at60, 200, &|c, seed| {
            let mut r = start(c, seed, 0x60);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..3 {
                c.bus.write32(DATA + 0x400 + 0x60 + 4 * i, r.next_u32());
            }
            // The 12-byte destination at DATA + 0x100, including the
            // neighbouring words its unaligned pairs read-modify-write.
            for i in 0..6 {
                c.bus.write32(DATA + 0x0f8 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn ui_object_set_vec3_at60_matches() {
        check(ssx3_ee::s13::f_003a4cb0__generated, ui_object_set_vec3_at60, 200, &|c, seed| {
            let mut r = start(c, seed, 0x60);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..3 {
                c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
            }
            for i in 0..6 {
                c.bus.write32(DATA + 0x458 + 4 * i, r.next_u32());
            }
        });
    }

    // --------------------------------------------------------- UI vectors

    #[test]
    fn ui_vector_v20_matches() {
        check(ssx3_ee::s13::f_003a3d70__generated, ui_vector_v20, 200, &|c, seed| {
            let mut r = start(c, seed, 0x3d70);
            let vec = DATA + 0x400;
            c.g[A0][0] = vec as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let n = match seed % 4 {
                0 => 0, // no data pointer at all
                1 => 0, // count 0: only the default extent
                _ => 1 + seed % 3,
            };
            c.bus.write32(vec + 0x80, if seed % 4 == 0 { 0 } else { DATA + 0x800 });
            c.bus.write32(vec + 0x74, (n << 17) | (r.next_u32() & 0x1_ffff));
            for i in 0..n {
                c.bus.write32(DATA + 0x800 + 0xc * i, r.edge_f32());
                c.bus.write32(DATA + 0x804 + 0xc * i, r.edge_f32());
            }
            // The default extent constant.
            for i in 0..3 {
                c.bus.write32(0x004f_f0d8 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn ui_vector_v21_matches() {
        check(ssx3_ee::s13::f_003a3e38__generated, ui_vector_v21, 200, &|c, seed| {
            let mut r = start(c, seed, 0x3e38);
            let vec = DATA + 0x400;
            c.g[A0][0] = vec as u64;
            c.g[A1][0] = (DATA + 0x300) as u64;
            c.bus.write32(vec + 0x74, if seed % 2 == 0 { 0x8_0000 } else { r.next_u32() & 0xfffe_0000 });
            c.bus.write32(vec + 0x80, DATA + 0x800);
            for i in 0..4 * 3 {
                c.bus.write32(DATA + 0x800 + 4 * i, r.edge_f32());
            }
            for i in 0..3 {
                c.bus.write32(DATA + 0x300 + 4 * i, r.edge_f32());
            }
            // The 12-byte copy into the vec3 at +0x60.
            for i in 0..4 {
                c.bus.write32(DATA + 0x45c + 4 * i, r.next_u32());
            }
        });
    }

    // ------------------------------------------------------ camera tuning

    #[test]
    fn mid_left_chase_camera_algorithm_get_tuning_matches() {
        check(ssx3_ee::s02::f_00168d50__generated, mid_left_chase_camera_algorithm_get_tuning, 100, &|c, seed| {
            let mut r = start(c, seed, 0x6608);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..4 {
                c.bus.write32((GAME_GP as i32 - 0x6608 + 4 * i) as u32, r.edge_f32());
            }
            for i in 0..5 {
                c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
            }
        });
    }

    /// One shared test body for the `GetTuning` writers.
    fn check_tuning(hand: GuestFn, generated: GuestFn, base: i32) {
        check(generated, hand, 100, &|c, seed| {
            let mut r = start(c, seed, base as u64);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..4 {
                c.bus.write32((GAME_GP as i32 + base + 4 * i) as u32, r.edge_f32());
            }
            for i in 0..5 {
                c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn default_2_chase_camera_algorithm_get_tuning_matches() {
        check_tuning(default_2_chase_camera_algorithm_get_tuning, ssx3_ee::s02::f_00176ae0__generated, -0x61e8);
    }

    #[test]
    fn default_3_chase_camera_algorithm_get_tuning_matches() {
        check_tuning(default_3_chase_camera_algorithm_get_tuning, ssx3_ee::s02::f_00176de0__generated, -0x6130);
    }

    #[test]
    fn default_4_chase_camera_algorithm_get_tuning_matches() {
        check_tuning(default_4_chase_camera_algorithm_get_tuning, ssx3_ee::s02::f_001770e0__generated, -0x6074);
    }

    #[test]
    fn direction_ne_chase_camera_algorithm_get_tuning_matches() {
        check_tuning(direction_ne_chase_camera_algorithm_get_tuning, ssx3_ee::s02::f_00177558__generated, -0x5f48);
    }

    /// One shared test body for the `GetEyeDrop` constants.
    fn check_eye_drop(hand: GuestFn, generated: GuestFn, base: i32) {
        check(generated, hand, 100, &|c, seed| {
            let mut r = start(c, seed, base as u64);
            c.bus.write32((GAME_GP as i32 + base) as u32, r.edge_f32());
        });
    }

    #[test]
    fn default_2_chase_camera_algorithm_get_eye_drop_matches() {
        check_eye_drop(default_2_chase_camera_algorithm_get_eye_drop, ssx3_ee::s02::f_00176ad0__generated, -0x61ec);
    }

    #[test]
    fn default_3_chase_camera_algorithm_get_eye_drop_matches() {
        check_eye_drop(default_3_chase_camera_algorithm_get_eye_drop, ssx3_ee::s02::f_00176dd0__generated, -0x6134);
    }

    #[test]
    fn default_4_chase_camera_algorithm_get_eye_drop_matches() {
        check_eye_drop(default_4_chase_camera_algorithm_get_eye_drop, ssx3_ee::s02::f_001770d0__generated, -0x6078);
    }

    #[test]
    fn spoke_chase_camera_algorithm_get_eye_drop_matches() {
        check_eye_drop(spoke_chase_camera_algorithm_get_eye_drop, ssx3_ee::s02::f_00177eb8__generated, -0x5e20);
    }

    // ------------------------------------------------------- camera objects

    #[test]
    fn script_camera_algo_get_look_at_matches() {
        check(ssx3_ee::s02::f_00176358__generated, script_camera_algo_get_look_at, 200, &|c, seed| {
            let mut r = start(c, seed, 0x20);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..8 {
                c.bus.write32(DATA + 0x120 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn camera_point_bound_obj_clamp_point_matches() {
        check(ssx3_ee::s02::f_00176808__generated, camera_point_bound_obj_clamp_point, 200, &|c, seed| {
            let mut r = start(c, seed, 0x10);
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            for i in 0..3 {
                c.bus.write32(DATA + 0x100 + 4 * i, r.edge_f32());
            }
            for i in 0..4 {
                c.bus.write32(DATA + 0x3fc + 4 * i, r.next_u32());
            }
        });
    }

    // -------------------------------------------------- graphics manager

    #[test]
    fn ps2_graphics_man_get_display_mode_info_matches() {
        check_lohi(ssx3_ee::s12::f_00376560__generated, ps2_graphics_man_get_display_mode_info, 150, &|c, seed| {
            let mut r = start(c, seed, 0x59cc);
            let gm = DATA + 0x400;
            c.g[A0][0] = gm as u64;
            c.g[A1][0] = (seed % 5) as u64;
            c.bus.write32(gm + 0x59cc, DATA + 0x800);
            for i in 0..8 {
                c.bus.write32(DATA + 0x800 + 0xa4 * (seed % 5) + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn ps2_graphics_man_get_viewport_matches() {
        check_lohi(ssx3_ee::s12::f_00376b90__generated, ps2_graphics_man_get_viewport, 150, &|c, seed| {
            let mut r = start(c, seed, 0x6d20);
            let gm = DATA + 0x400;
            let mode = seed % 4;
            // `out` is a0, the manager (mode + viewport records) is a1.
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = gm as u64;
            c.bus.write32(gm + 0x10dc, mode);
            for i in 0..4 {
                c.bus.write32(gm + 0x6d20 + 0x230 * mode + 4 * i, r.edge_f32());
            }
            for i in 0..4 {
                c.bus.write32(DATA + 0x0fc + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn ps2_graphics_man_get_fov_matches() {
        check_lohi(ssx3_ee::s12::f_00376c10__generated, ps2_graphics_man_get_fov, 150, &|c, seed| {
            let mut r = start(c, seed, 0x6d38);
            let gm = DATA + 0x400;
            let mode = seed % 4;
            c.g[A0][0] = gm as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write32(gm + 0x10dc, mode);
            c.bus.write32(gm + 0x6d38 + 0x230 * mode, r.edge_f32());
        });
    }

    #[test]
    fn ps2_graphics_man_get_near_far_aspect_matches() {
        check_lohi(ssx3_ee::s12::f_00376c28__generated, ps2_graphics_man_get_near_far_aspect, 150, &|c, seed| {
            let mut r = start(c, seed, 0x6c28);
            let gm = DATA + 0x400;
            let mode = seed % 4;
            c.g[A0][0] = gm as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            c.g[A3][0] = (DATA + 0x300) as u64;
            c.bus.write32(gm + 0x10dc, mode);
            for i in 0..3 {
                c.bus.write32(gm + 0x6d20 + 0x230 * mode + 0x10 + 4 * i, r.edge_f32());
            }
        });
    }

    #[test]
    fn ps2_graphics_man_set_fog_mode_matches() {
        check(ssx3_ee::s12::f_00377950__generated, ps2_graphics_man_set_fog_mode, 200, &|c, seed| {
            let mut r = start(c, seed, 0x77950);
            let gm = DATA + 0x400;
            c.g[A0][0] = gm as u64;
            c.g[A1][0] = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 5,
                _ => u64::MAX, // -1
            };
            for i in 0..5 {
                c.bus.write32(gm + 0x6b94 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn ps2_graphics_man_set_fog_params_matches() {
        check(ssx3_ee::s12::f_003779e0__generated, ps2_graphics_man_set_fog_params, 200, &|c, seed| {
            let mut r = start(c, seed, 0x779e0);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..5 {
                c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
            }
            for i in 0..6 {
                c.bus.write32(DATA + 0x3f4 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn ps2_graphics_man_is_texture_valid_matches() {
        check(ssx3_ee::s12::f_0037cb90__generated, ps2_graphics_man_is_texture_valid, 200, &|c, seed| {
            let mut r = start(c, seed, 0x7cb90);
            let gm = DATA + 0x400;
            c.g[A0][0] = gm as u64;
            let idx = match seed % 4 {
                0 => u64::MAX, // -1
                1 => 2000,
                2 => 2,
                _ => seed as u64 % 4,
            };
            c.g[A1][0] = idx;
            c.bus.write32(gm + 0x18f4, DATA + 0x1000);
            for i in 0..4 {
                c.bus.write32(DATA + 0x1000 + 4 * i, if seed % 2 == 0 { 0 } else { DATA + 0x2000 + 0x10 * i });
                c.bus.write32(DATA + 0x2008 + 0x10 * i, (seed + i) as u32 % 3);
            }
        });
    }

    #[test]
    fn ps2_graphics_man_add_light_matches() {
        check_lohi(ssx3_ee::s12::f_0037d758__generated, ps2_graphics_man_add_light, 100, &|c, seed| {
            let mut r = start(c, seed, 0x7d758);
            let gm = DATA + 0x400;
            c.g[A0][0] = gm as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            c.g[A3][0] = (DATA + 0x300) as u64;
            c.bus.write32(gm + 0x13ec, seed % 3);
            for i in 0..4 {
                c.bus.write128(DATA + 0x100 + 0x10 * i, (r.next_u32() as u128) << 96 | 0x1111);
            }
            for i in 0..16 {
                c.bus.write32(DATA + 0x200 + 4 * i, r.next_u32());
                c.bus.write32(DATA + 0x300 + 4 * i, r.next_u32());
            }
            for i in 0..0x200 {
                c.bus.write32(DATA + 0x13f0 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn ps2_graphics_man_get_texture_width_matches() {
        check(ssx3_ee::s12::f_0037d908__generated, ps2_graphics_man_get_texture_width, 200, &|c, seed| {
            let mut r = start(c, seed, 0x7d908);
            let gm = DATA + 0x400;
            c.g[A0][0] = gm as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(gm + 0x18f4, DATA + 0x1000);
            for i in 0..4 {
                c.bus.write32(DATA + 0x1000 + 4 * i, if seed % 3 == 0 { 0 } else { DATA + 0x2000 + 0x40 * i });
                c.bus.write32(DATA + 0x2000 + 0x40 * i + 8, DATA + 0x3000 + 0x40 * i);
                c.bus.write64(DATA + 0x3000 + 0x40 * i + 0x38, (r.next_u32() as u64) << 32 | r.next_u32() as u64);
            }
        });
    }

    #[test]
    fn ps2_graphics_man_get_texture_height_matches() {
        check(ssx3_ee::s12::f_0037d938__generated, ps2_graphics_man_get_texture_height, 200, &|c, seed| {
            let mut r = start(c, seed, 0x7d938);
            let gm = DATA + 0x400;
            c.g[A0][0] = gm as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(gm + 0x18f4, DATA + 0x1000);
            for i in 0..4 {
                c.bus.write32(DATA + 0x1000 + 4 * i, if seed % 3 == 0 { 0 } else { DATA + 0x2000 + 0x40 * i });
                c.bus.write32(DATA + 0x2000 + 0x40 * i + 8, DATA + 0x3000 + 0x40 * i);
                c.bus.write64(DATA + 0x3000 + 0x40 * i + 0x38, (r.next_u32() as u64) << 32 | r.next_u32() as u64);
            }
        });
    }

    #[test]
    fn ps2_graphics_man_clip_test_box_matches() {
        // The VU0 microprogram at 0x570 (`box_clip`) runs for real; it is
        // cheap, but 40 seeds keep the test well under the budget.
        check(ssx3_ee::s12::f_0037de88__generated, ps2_graphics_man_clip_test_box, 40, &|c, seed| {
            let mut r = start(c, seed, 0x7de88);
            c.g[A0][0] = (DATA + 0x000) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            c.g[A3][0] = (DATA + 0x300) as u64;
            put_vec(c, DATA + 0x100, [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
            put_vec(c, DATA + 0x200, [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
            for i in 0..4 {
                put_vec(c, DATA + 0x300 + 0x10 * i, [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()]);
            }
        });
    }

    #[test]
    fn ps2_graphics_man_v97_matches() {
        check(ssx3_ee::s12::f_00380380__generated, ps2_graphics_man_v97, 100, &|c, seed| {
            let mut r = start(c, seed, 0x380380);
            let obj = DATA + 0x400;
            c.g[A0][0] = 0;
            c.g[A1][0] = obj as u64;
            c.g[A2][0] = (seed % 2) as u64;
            c.g[A3][0] = (DATA + 0x5000) as u64;
            c.g[T0][0] = (DATA + 0x6000) as u64;
            c.g[T1][0] = (DATA + 0x7000) as u64;
            // obj -> model (+0x80), extra offset (+0x98).
            c.bus.write32(obj + 0x80, DATA + 0x8000);
            c.bus.write32(obj + 0x98, 0x20);
            // Model: corner-table offset, vertex base, scale.
            c.bus.write32(DATA + 0x8000 + 8, 0x100);
            c.bus.write32(DATA + 0x8000 + 0x24, 0x180);
            for i in 0..3 {
                c.bus.write32(DATA + 0x8000 + 0x18 + 4 * i, r.edge_f32());
            }
            // Corner entry -> node -> vertex data (indices 0 and 1).
            c.bus.write32(DATA + 0x8100 + 4, DATA + 0x8200);
            c.bus.write32(DATA + 0x8110 + 4, DATA + 0x8200);
            c.bus.write32(DATA + 0x8200 + 0x20, DATA + 0x8300);
            c.bus.write32(DATA + 0x8300, DATA + 0x8400);
            c.bus.write32(DATA + 0x8400 + 4, DATA + 0x8500);
            // Vertex block: colour record pointer at +0x14 (data = +0x8520),
            // and the texcoord base at +0x4 of DATA+0x8500+0x180.
            c.bus.write32(DATA + 0x8520 + 0x14, DATA + 0x8600);
            c.bus.write32(DATA + 0x8680 + 4, DATA + 0x8780);
            for i in 0..4 {
                for j in 0..3 {
                    c.bus.write16(DATA + 0x8820 + 2 * (3 * i + j), r.next_u32() as u16);
                    c.bus.write16(DATA + 0x8800 + 2 * (2 * i + j), r.next_u32() as u16);
                }
            }
            for i in 0..8 {
                c.bus.write16(DATA + 0x8600 + 2 * i, r.next_u32() as u16);
            }
            // The two gp constants.
            c.bus.write32((GAME_GP as i32 - 0x2760) as u32, 0x3780_0000);
            c.bus.write32((GAME_GP as i32 - 0x275c) as u32, 0x3d04_2108);
            // Outputs (positions, uv, colours).
            for i in 0..16 {
                c.bus.write32(DATA + 0x4ff0 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn ps2_graphics_man_add_view_matches() {
        check(ssx3_ee::s12::f_00386688__generated, ps2_graphics_man_add_view, 200, &|c, seed| {
            let mut r = start(c, seed, 0x386688);
            let gm = DATA + 0x400;
            c.g[A0][0] = gm as u64;
            c.g[A1][0] = (seed % 4) as u64;
            for i in 0..16 {
                c.bus.write32(gm + 0x6af0 + 4 * i, r.next_u32());
            }
            for i in 0..4 {
                c.bus.write32(gm + 0x58fc + 4 * i, r.next_u32());
            }
            for i in 0..0x40 {
                c.bus.write32(gm + 0x5930 + 4 * i, r.next_u32());
            }
        });
    }

    // ------------------------------------------------- comparator / streams

    #[test]
    fn occlusion_man_compare_by_distance_matches() {
        check(ssx3_ee::s06::f_00242978__generated, occlusion_man_compare_by_distance, 200, &|c, seed| {
            let mut r = start(c, seed, 0x42978);
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            let (fa, fb) = match seed % 4 {
                0 => (r.edge_f32(), r.edge_f32()),
                1 => {
                    let v = r.edge_f32();
                    (v, v) // equal: returns 0
                }
                2 => (r.edge_f32(), r.edge_f32()),
                _ => (r.edge_f32(), r.edge_f32()),
            };
            c.bus.write32(DATA + 0x104, fa);
            c.bus.write32(DATA + 0x204, fb);
        });
    }

    #[test]
    fn c_ui_state_buy_popup_on_widget_notify_matches() {
        check(ssx3_ee::s04::f_001caed8__generated, c_ui_state_buy_popup_on_widget_notify, 200, &|c, seed| {
            let _r = start(c, seed, 0xcaed8);
            c.g[A2][0] = match seed % 6 {
                0 => 0,
                1 => 6,
                2 => 7,
                3 => 9,
                4 => 10,
                5 => 0xffff,
                _ => u64::MAX,
            };
        });
    }

    #[test]
    fn generic_stream_pause_v02_matches() {
        check(ssx3_ee::s06::f_002443f0__generated, generic_stream_pause_v02, 200, &|c, seed| {
            let _r = start(c, seed, 0x443f0);
            c.g[A1][0] = (seed as u64) << 32 | 0x1234_5678;
        });
    }

    #[test]
    fn c_fe_state_progressive_on_widget_notify_matches() {
        check(ssx3_ee::s06::f_002453e0__generated, c_fe_state_progressive_on_widget_notify, 200, &|c, seed| {
            let _r = start(c, seed, 0x453e0);
            c.g[A2][0] = match seed % 4 {
                0 => 6,
                1 => 0,
                2 => 0x1_0000_0006,
                _ => 7,
            };
        });
    }

    // ------------------------------------------------------------- codec

    #[test]
    fn ps2_sony_codec_internal_nodata_callback_matches() {
        // Keep the feeder on its short path: the chunk at `codec + 0x28` has
        // no bitstream data (`+0x4 == 0`), so it only fills the "no data"
        // words, releases the chunk and queues one DMA chain entry.
        check(ssx3_ee::s13::f_003b0b10__generated, ps2_sony_codec_internal_nodata_callback, 60, &|c, seed| {
            let mut r = start(c, seed, 0xb0b10);
            let popup = DATA + 0x400; // arg0 -> the feeder's a1 (s4)
            let notif = DATA + 0x500; // arg1 -> the feeder's a2
            let codec = DATA + 0x600; // arg2 -> the feeder's a0 (s1)
            c.g[A0][0] = popup as u64;
            c.g[A1][0] = notif as u64;
            c.g[A2][0] = codec as u64;
            // The chunk holder and the chunk it returns.
            c.bus.write32(codec + 0x28, DATA + 0x700);
            c.bus.write32(DATA + 0x700, DATA + 0x800);
            c.bus.write32(DATA + 0x804, 0); // no bitstream data
            c.bus.write32(DATA + 0x70c, 0x003b_0720); // the chunk's free hook
            c.bus.write32(codec + 0x7c, DATA + 0x900); // "no data" fill target
            c.bus.write32(codec + 0x78, r.next_u32()); // DMA tag source
            c.bus.write32(codec + 0x14, r.next_u32() & 0xff); // the counter
            c.bus.write32(popup + 0x40, DATA + 0xa00); // the DMA chain base
            for i in 0..4 {
                c.bus.write32(DATA + 0x900 + 4 * i, r.next_u32());
                c.bus.write32(codec + 4 * i, r.next_u32());
            }
            c.bus.write32(DATA + 0x1280, r.next_u32());
            c.bus.write32(DATA + 0x1284, r.next_u32());
        });
    }

    #[test]
    fn ps2_sony_codec_internal_v06_matches() {
        check(ssx3_ee::s13::f_003b0fa8__generated, ps2_sony_codec_internal_v06, 200, &|c, seed| {
            let mut r = start(c, seed, 0xb0fa8);
            c.bus.write32(0x0050_9508, r.edge_f32());
        });
    }

    #[test]
    fn ps2_sony_codec_internal_release_frame_matches() {
        check(ssx3_ee::s13::f_003b1140__generated, ps2_sony_codec_internal_release_frame, 200, &|c, seed| {
            let mut r = start(c, seed, 0xb1140);
            let codec = DATA + 0x400;
            let item = DATA + 0x500;
            c.g[A0][0] = codec as u64;
            c.g[A1][0] = item as u64;
            c.bus.write32(item + 0x10, if seed % 2 == 0 { 2 } else { 1 });
            c.bus.write32(item + 8, DATA + 0x600);
            c.bus.write32(item + 0xc, DATA + 0x640);
            c.bus.write32(codec + 0x20, DATA + 0x680);
            c.bus.write32(codec + 0x24, DATA + 0x700);
            for i in 0..4 {
                c.bus.write32(DATA + 0x600 + 4 * i, r.next_u32());
                c.bus.write32(DATA + 0x640 + 4 * i, r.next_u32());
                c.bus.write32(DATA + 0x680 + 4 * i, r.next_u32());
                c.bus.write32(DATA + 0x700 + 4 * i, r.next_u32());
            }
        });
    }
}
