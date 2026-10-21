//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for menu-item value reset/measure helpers, replay-cache track
//! event-time queries, PS2 graphics-matrix/font helpers, the memory-card port
//! state queries, and camera getters, each with a differential test against
//! the generated body.
//!
//! Everything here is leaf code except `cPS2MemCard_HasSlotName` (it calls
//! `strlen`) and the three loops (`InSeq_*GetNextEventTime`,
//! `PS2GraphicsMan_SetLighting`), which deliver events at their back edges the
//! way the originals do.

use crate::abi;
use crate::call::{call_ptr, guest};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_d3a0, menu_sys_spacer_item_measure, "MenuSysSpacerItem_Measure"),
    (0x002c_d578, menu_sys_int8_item_reset_value, "MenuSysInt8Item_ResetValue"),
    (0x002c_d9b8, menu_sys_int_item_reset_value, "MenuSysIntItem_ResetValue"),
    (0x002c_df78, menu_sys_float_item_reset_value, "MenuSysFloatItem_ResetValue"),
    (0x0027_3040, in_seq_span_track_get_next_event_time, "InSeq_SpanTrackGetNextEventTime"),
    (0x0027_33e0, in_seq_event_track_get_next_event_time, "InSeq_EventTrackGetNextEventTime"),
    (0x0027_37e0, in_seq_ramp_track_get_next_event_time, "InSeq_RampTrackGetNextEventTime"),
    (0x0039_3fb8, graphics_man_set_screen_offset, "GraphicsMan_SetScreenOffset"),
    (0x0039_3fd8, graphics_man_get_screen_offset_x, "GraphicsMan_GetScreenOffsetX"),
    (0x0039_3ff0, graphics_man_get_screen_offset_y, "GraphicsMan_GetScreenOffsetY"),
    (0x0039_42a0, graphics_man_v58, "GraphicsMan_v58"),
    (0x0039_4d18, light_man_next_id, "LightMan_NextId"),
    (0x0039_5330, ps2_graphics_man_get_screen_width, "PS2GraphicsMan_GetScreenWidth"),
    (0x0039_5350, ps2_graphics_man_get_screen_height, "PS2GraphicsMan_GetScreenHeight"),
    (0x0039_54d0, ps2_graphics_man_set_lighting, "PS2GraphicsMan_SetLighting"),
    (0x0039_56e8, ps2_graphics_man_push_matrix, "PS2GraphicsMan_PushMatrix"),
    (0x0039_5c38, ps2_graphics_man_load_matrix, "PS2GraphicsMan_LoadMatrix"),
    (0x0039_5c70, ps2_graphics_man_mult_matrix, "PS2GraphicsMan_MultMatrix"),
    (0x0039_5cf0, ps2_graphics_man_load_identity, "PS2GraphicsMan_LoadIdentity"),
    (0x0039_5d28, ps2_graphics_man_translate, "PS2GraphicsMan_Translate"),
    (0x0039_6110, c_ps2_grid_mesh_set_scale, "cPS2GridMesh_SetScale"),
    (0x0039_68d0, rail_tube_set_point, "RailTube_SetPoint"),
    (0x002c_44e0, c_ps2_mem_card_clear_port_changed, "cPS2MemCard_ClearPortChanged"),
    (0x002c_4748, c_ps2_mem_card_is_state34, "cPS2MemCard_IsState34"),
    (0x002c_4880, c_ps2_mem_card_is_state35, "cPS2MemCard_IsState35"),
    (0x002c_5140, c_ps2_mem_card_is_busy, "cPS2MemCard_IsBusy"),
    (0x002c_5168, c_ps2_mem_card_is_port_card_error, "cPS2MemCard_IsPortCardError"),
    (0x002c_5300, c_ps2_mem_card_is_port_not_ps2_card, "cPS2MemCard_IsPortNotPS2Card"),
    (0x002c_5358, c_ps2_mem_card_v52, "cPS2MemCard_v52"),
    (0x002c_53b0, c_ps2_mem_card_is_port_formatted, "cPS2MemCard_IsPortFormatted"),
    (0x002c_5528, c_ps2_mem_card_has_slot_name, "cPS2MemCard_HasSlotName"),
    (0x002c_5558, c_ps2_mem_card_mark_slot_used, "cPS2MemCard_MarkSlotUsed"),
    (0x002c_5588, c_ps2_mem_card_is_slot_empty, "cPS2MemCard_IsSlotEmpty"),
    (0x002c_55c0, c_ps2_mem_card_slot_has_1ac, "cPS2MemCard_SlotHas1AC"),
    (0x002c_66d8, c_ps2_mem_card_is_port_result_m11, "cPS2MemCard_IsPortResultM11"),
    (0x002c_6758, c_ps2_mem_card_calc_save_size_kb, "cPS2MemCard_CalcSaveSizeKB"),
    (0x0015_f598, rider_camera_target_get_orientation, "RiderCameraTarget_GetOrientation"),
    (0x0015_f5f8, rider_camera_target_get_forward, "RiderCameraTarget_GetForward"),
    (0x0015_f610, rider_camera_target_get_side, "RiderCameraTarget_GetSide"),
    (0x0015_f628, rider_camera_target_get_up, "RiderCameraTarget_GetUp"),
    (0x0015_f650, rider_camera_target_get_ground_normal, "RiderCameraTarget_GetGroundNormal"),
    (0x0015_f668, rider_camera_target_get_prev_ground_normal, "RiderCameraTarget_GetPrevGroundNormal"),
    (0x0015_f6b0, rider_camera_target_get_air_time_remaining, "RiderCameraTarget_GetAirTimeRemaining"),
    (0x0015_f6c8, rider_camera_target_get_air_time_9c_remaining, "RiderCameraTarget_GetAirTime9cRemaining"),
    (0x0015_f720, rider_camera_target_is_landing_predicted, "RiderCameraTarget_IsLandingPredicted"),
    (0x0016_7e40, camera_algorithm_get_look_at, "CameraAlgorithm_GetLookAt"),
    (0x0016_7fa0, rider_camera_algorithm_get_look_at, "RiderCameraAlgorithm_GetLookAt"),
    (0x0016_8a30, mid_right_chase_camera_algorithm_get_eye_drop, "MidRightChaseCameraAlgorithm_GetEyeDrop"),
    (0x0016_8a40, mid_right_chase_camera_algorithm_get_tuning, "MidRightChaseCameraAlgorithm_GetTuning"),
    (0x0016_8d40, mid_left_chase_camera_algorithm_get_eye_drop, "MidLeftChaseCameraAlgorithm_GetEyeDrop"),
];

/// Return -1 (the "no event" sentinel): the shared tail of the three
/// `InSeq_*TrackGetNextEventTime` queries.
fn ret_minus_one(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Menu items.

/// `s2CD3A0Vec MenuSysSpacerItem_Measure(self, item)` (0x2cd3a0): the spacer's
/// height (the int at `item + 0x14`) as a vec4 `{0, 0, 0, (float)h}`.
pub fn menu_sys_spacer_item_measure(c: &mut Cpu) {
    c.cycles += 19;
    c.mtc1(0, 0);
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x10);
    c.g[SP][0] = sx(sp);
    c.g[V0][0] = c.g[A0][0];
    let out = c.g[A0][0] as u32;
    let height = Addr(abi::arg(c, 1)).off(0x14).0;
    c.swc1(0, sp);
    c.swc1(0, sp.wrapping_add(0x8));
    c.swc1(0, sp.wrapping_add(0x4));
    c.lwc1(1, height);
    c.cvt_s_w(1, 1);
    c.swc1(1, sp.wrapping_add(0xc));
    // The original copies the four words of the local vector with
    // ldl/ldr + sdl/sdr pairs; keep the exact (possibly misaligned) ops.
    c.ldl(3, sp.wrapping_add(7));
    c.ldr(3, sp);
    c.ldl(5, sp.wrapping_add(0xf));
    c.ldr(5, sp.wrapping_add(8));
    c.sdl(3, out.wrapping_add(7));
    c.sdr(3, out);
    c.sdl(5, out.wrapping_add(0xf));
    c.sdr(5, out.wrapping_add(8));
    c.g[SP][0] = sx(sp.wrapping_add(0x10));
    abi::ret(c);
}

/// `void MenuSysInt8Item_ResetValue(self)` (0x2cd578): store the item's
/// signed-byte value (`+0x1c`) through the pointer at `+0x18`, or 0 when it is
/// not positive.
pub fn menu_sys_int8_item_reset_value(c: &mut Cpu) {
    c.cycles += 3;
    let item = Addr(abi::arg(c, 0));
    c.g[V0][0] = item.off(0x1c).lb(c);
    let positive = (c.g[V0][0] as i64) > 0;
    c.g[V1][0] = item.off(0x1c).lbu(c);
    if positive {
        c.cycles += 3;
        c.g[V0][0] = item.off(0x18).lw(c);
        let dst = c.g[V0][0] as u32;
        let v = c.g[V1][0];
        Addr(dst).sb(c, v);
    } else {
        c.cycles += 3;
        c.g[V0][0] = item.off(0x18).lw(c);
        let dst = c.g[V0][0] as u32;
        Addr(dst).sb(c, 0);
    }
    abi::ret(c);
}

/// `void MenuSysIntItem_ResetValue(self)` (0x2cd9b8): store the item's int
/// value (`+0x1c`) through the pointer at `+0x18`, or 0 when it is not
/// positive.
pub fn menu_sys_int_item_reset_value(c: &mut Cpu) {
    c.cycles += 3;
    let item = Addr(abi::arg(c, 0));
    c.g[V1][0] = item.off(0x1c).lw(c);
    let positive = (c.g[V1][0] as i64) > 0;
    c.g[V0][0] = item.off(0x18).lw(c);
    let dst = c.g[V0][0] as u32;
    if positive {
        c.cycles += 2;
        let v = c.g[V1][0];
        Addr(dst).sw(c, v);
    } else {
        c.cycles += 2;
        Addr(dst).sw(c, 0);
    }
    abi::ret(c);
}

/// `void MenuSysFloatItem_ResetValue(self)` (0x2cdf78): store the item's
/// float value (`+0x1c`) through the pointer at `+0x18`, or +0.0 when it is
/// not positive.
pub fn menu_sys_float_item_reset_value(c: &mut Cpu) {
    c.cycles += 6;
    let item = Addr(abi::arg(c, 0));
    c.mtc1(0, 1);
    c.lwc1(0, item.off(0x1c).0);
    c.c_le_s(0, 1);
    let positive = !c.fcc();
    c.g[V0][0] = item.off(0x18).lw(c);
    let dst = c.g[V0][0] as u32;
    if positive {
        c.cycles += 2;
        c.swc1(0, dst);
    } else {
        c.cycles += 2;
        c.swc1(1, dst);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Replay-cache track event times.

/// `int InSeq_SpanTrackGetNextEventTime(track)` (0x273040): the earliest event
/// time in the span track's node list (`+0x24`, time at `+0xc`, next at
/// `+0x18`) or on the current node (`+0x14`, time at `+4`); -1 when the track
/// or its event table is empty.
pub fn in_seq_span_track_get_next_event_time(c: &mut Cpu) {
    let track = Addr(abi::arg(c, 0));
    c.cycles += 3;
    c.g[V0][0] = track.off(0x8).lw(c);
    if c.g[V0][0] == 0 {
        return ret_minus_one(c);
    }
    c.g[V0][0] = track.off(0x10).lw(c);
    c.cycles += 3;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
    let empty = c.g[V1][0] == 0;
    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
    if empty {
        return ret_minus_one(c);
    }

    c.cycles += 3;
    c.g[V1][0] = track.off(0x24).lw(c);
    let no_nodes = c.g[V1][0] == 0;
    c.g[A0][0] = track.off(0x14).lw(c);
    if !no_nodes {
        c.cycles += 1;
        'walk: loop {
            c.cycles += 2;
            let best_negative = (c.g[A2][0] as i64) < 0;
            let node = c.g[V1][0] as u32;
            c.g[A1][0] = Addr(node).off(0xc).lw(c);
            let mut improve = best_negative;
            if !best_negative {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    improve = true;
                }
            }
            if improve {
                c.cycles += 2;
                c.g[A2][0] = c.g[A1][0];
            }
            let node = c.g[V1][0] as u32;
            c.g[V1][0] = Addr(node).off(0x18).lw(c);
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_3070);
            }
        }
    }

    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 2;
        let best_negative = (c.g[A2][0] as i64) < 0;
        let current = c.g[A0][0] as u32;
        c.g[A0][0] = Addr(current).off(0x4).lhu(c);
        let mut improve = best_negative;
        if !best_negative {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            improve = c.g[V0][0] != 0;
        }
        if improve {
            c.cycles += 1;
            c.g[A2][0] = c.g[A0][0];
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

/// `int InSeq_EventTrackGetNextEventTime(track)` (0x2733e0): the current event
/// node's time (`track + 0x14`, +2), or -1.
pub fn in_seq_event_track_get_next_event_time(c: &mut Cpu) {
    let track = Addr(abi::arg(c, 0));
    'fallback: {
        c.cycles += 3;
        c.g[V0][0] = track.off(0x8).lw(c);
        if c.g[V0][0] == 0 {
            break 'fallback;
        }
        c.g[V0][0] = track.off(0x10).lw(c);
        c.cycles += 3;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
        if c.g[V1][0] == 0 {
            break 'fallback;
        }
        c.cycles += 3;
        c.g[A0][0] = track.off(0x14).lw(c);
        let no_current = c.g[A0][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !no_current {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x2).lhu(c);
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    ret_minus_one(c);
}

/// `int InSeq_RampTrackGetNextEventTime(track)` (0x2737e0): the same query for
/// the ramp track (nodes at `+0x20`, time at `+8`, next at `+0x20`).
pub fn in_seq_ramp_track_get_next_event_time(c: &mut Cpu) {
    let track = Addr(abi::arg(c, 0));
    c.cycles += 3;
    c.g[V0][0] = track.off(0x8).lw(c);
    if c.g[V0][0] == 0 {
        return ret_minus_one(c);
    }
    c.g[V0][0] = track.off(0x10).lw(c);
    c.cycles += 3;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
    let empty = c.g[V1][0] == 0;
    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
    if empty {
        return ret_minus_one(c);
    }

    c.cycles += 3;
    c.g[V1][0] = track.off(0x20).lw(c);
    let no_nodes = c.g[V1][0] == 0;
    c.g[A0][0] = track.off(0x14).lw(c);
    if !no_nodes {
        c.cycles += 1;
        'walk: loop {
            c.cycles += 2;
            let best_negative = (c.g[A2][0] as i64) < 0;
            let node = c.g[V1][0] as u32;
            c.g[A1][0] = Addr(node).off(0x8).lw(c);
            let mut improve = best_negative;
            if !best_negative {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    improve = true;
                }
            }
            if improve {
                c.cycles += 2;
                c.g[A2][0] = c.g[A1][0];
            }
            let node = c.g[V1][0] as u32;
            c.g[V1][0] = Addr(node).off(0x20).lw(c);
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_3810);
            }
        }
    }

    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 2;
        let best_negative = (c.g[A2][0] as i64) < 0;
        let current = c.g[A0][0] as u32;
        c.g[A0][0] = Addr(current).off(0x4).lhu(c);
        let mut improve = best_negative;
        if !best_negative {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            improve = c.g[V0][0] != 0;
        }
        if improve {
            c.cycles += 1;
            c.g[A2][0] = c.g[A0][0];
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Graphics manager / font helpers.

/// `void GraphicsMan_SetScreenOffset(self, x, y)` (0x393fb8): store the
/// integer arguments as the screen-offset floats.
pub fn graphics_man_set_screen_offset(c: &mut Cpu) {
    c.cycles += 7;
    c.mtc1(5, 0);
    c.cvt_s_w(0, 0);
    c.mtc1(6, 1);
    c.cvt_s_w(1, 1);
    let this = Addr(abi::arg(c, 0));
    c.swc1(0, this.0);
    c.swc1(1, this.off(0x4).0);
    abi::ret(c);
}

/// `int GraphicsMan_GetScreenOffsetX(self)` (0x393fd8): truncate the float at
/// `+0`.
pub fn graphics_man_get_screen_offset_x(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    c.lwc1(0, this.0);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    abi::ret(c);
}

/// `int GraphicsMan_GetScreenOffsetY(self)` (0x393ff0): truncate the float at
/// `+4`.
pub fn graphics_man_get_screen_offset_y(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    c.lwc1(0, this.off(0x4).0);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    abi::ret(c);
}

/// `self GraphicsMan_v58(self)` (0x3942a0): initialise a colour/scale object
/// to `{1.0f, 1.0f, 0, 1.0f}` and return it.
pub fn graphics_man_v58(c: &mut Cpu) {
    c.cycles += 8;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[V0][0] = c.g[A0][0];
    let this = Addr(abi::arg(c, 0));
    this.off(0x8).sw(c, 0);
    c.swc1(0, this.off(0xc).0);
    c.swc1(0, this.0);
    c.swc1(0, this.off(0x4).0);
    abi::ret(c);
}

/// `void* LightMan_NextId(self)` (0x394d18): `base + index` and increment the
/// index (`+0xc`).
pub fn light_man_next_id(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0xc).lw(c);
    c.g[V0][0] = this.off(0x8).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    let next = c.g[V1][0];
    this.off(0xc).sw(c, next);
    abi::ret(c);
}

/// `int PS2GraphicsMan_GetScreenWidth(self)` (0x395330): the current font
/// entry's `+0x88` (entry size 0xa4).
pub fn ps2_graphics_man_get_screen_width(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x59c8).lw(c);
    c.g[A1][0] = 0xa4;
    c.g[V0][0] = this.off(0x59cc).lw(c);
    c.mult(0, V1, V1, A1);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x88).lw(c);
    abi::ret(c);
}

/// `int PS2GraphicsMan_GetScreenHeight(self)` (0x395350): the current font
/// entry's `+0x8c`.
pub fn ps2_graphics_man_get_screen_height(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x59c8).lw(c);
    c.g[A1][0] = 0xa4;
    c.g[V0][0] = this.off(0x59cc).lw(c);
    c.mult(0, V1, V1, A1);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x8c).lw(c);
    abi::ret(c);
}

/// `void PS2GraphicsMan_SetLighting(self, src)` (0x3954d0): copy the 0xa0-byte
/// lighting block to `self + 0x6bb0`, a quadword at a time.
pub fn ps2_graphics_man_set_lighting(c: &mut Cpu) {
    c.cycles += 2;
    let dst = (c.g[A0][0] as u32).wrapping_add(0x6bb0);
    c.g[A0][0] = sx(dst);
    let end = (c.g[A1][0] as u32).wrapping_add(0xa0);
    c.g[V0][0] = sx(end);
    loop {
        // Reload the loop state from the registers: an event delivered at
        // the back edge may have switched the guest thread.
        let src = c.g[A1][0] as u32;
        let dst = c.g[A0][0] as u32;
        c.cycles += 7;
        c.lq(3, src);
        c.lq(6, src.wrapping_add(0x10));
        c.sq(3, dst);
        c.sq(6, dst.wrapping_add(0x10));
        c.g[A1][0] = sx(src.wrapping_add(0x20));
        let more = c.g[A1][0] != c.g[V0][0];
        c.g[A0][0] = sx(dst.wrapping_add(0x20));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_54d8);
        }
    }
}

/// `void PS2GraphicsMan_PushMatrix(self)` (0x3956e8): copy the current matrix
/// to `top + 0x40` and advance the stack (`+0x13e4`) and depth (`+0x13e0`).
pub fn ps2_graphics_man_push_matrix(c: &mut Cpu) {
    c.cycles += 17;
    let this = Addr(abi::arg(c, 0));
    c.g[A1][0] = this.off(0x13e4).lw(c);
    let src = c.g[A1][0] as u32;
    c.g[V0][0] = sx(src.wrapping_add(0x40));
    let dst = c.g[V0][0] as u32;
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    c.g[V1][0] = this.off(0x13e4).lw(c);
    c.g[V0][0] = this.off(0x13e0).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    let top = c.g[V1][0];
    this.off(0x13e4).sw(c, top);
    let depth = c.g[V0][0];
    this.off(0x13e0).sw(c, depth);
    abi::ret(c);
}

/// `void PS2GraphicsMan_LoadMatrix(self, m)` (0x395c38): copy `m` over the
/// current matrix and drop the cached matrix flag (`+0x6b90`).
pub fn ps2_graphics_man_load_matrix(c: &mut Cpu) {
    c.cycles += 11;
    let this = Addr(abi::arg(c, 0));
    let m = abi::arg(c, 1) as u32;
    c.g[V0][0] = this.off(0x13e4).lw(c);
    let dst = c.g[V0][0] as u32;
    c.lqc2(1, m);
    c.lqc2(2, m.wrapping_add(0x10));
    c.lqc2(3, m.wrapping_add(0x20));
    c.lqc2(4, m.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void PS2GraphicsMan_MultMatrix(self, m)` (0x395c70): current = current * m
/// (four VU0 dot products).
pub fn ps2_graphics_man_mult_matrix(c: &mut Cpu) {
    c.cycles += 31;
    let this = Addr(abi::arg(c, 0));
    let m = abi::arg(c, 1) as u32;
    c.g[V0][0] = this.off(0x13e4).lw(c);
    let dst = c.g[V0][0] as u32;
    c.lqc2(4, dst);
    c.lqc2(5, dst.wrapping_add(0x10));
    c.lqc2(6, dst.wrapping_add(0x20));
    c.lqc2(7, dst.wrapping_add(0x30));
    c.lqc2(8, m);
    c.lqc2(9, m.wrapping_add(0x10));
    c.lqc2(10, m.wrapping_add(0x20));
    c.lqc2(11, m.wrapping_add(0x30));
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
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void PS2GraphicsMan_LoadIdentity(self)` (0x395cf0): copy the identity
/// matrix at `0x4ff1a0` over the current matrix.
pub fn ps2_graphics_man_load_identity(c: &mut Cpu) {
    c.cycles += 13;
    c.g[V0][0] = 0x50_0000;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = this.off(0x13e4).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    let identity = c.g[V0][0] as u32;
    let dst = c.g[V1][0] as u32;
    c.lqc2(1, identity);
    c.lqc2(2, identity.wrapping_add(0x10));
    c.lqc2(3, identity.wrapping_add(0x20));
    c.lqc2(4, identity.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void PS2GraphicsMan_Translate(self, v)` (0x395d28): translate the current
/// matrix by the vector at `v` (VU0 dot product into row 3).
pub fn ps2_graphics_man_translate(c: &mut Cpu) {
    c.cycles += 13;
    let this = Addr(abi::arg(c, 0));
    let v = abi::arg(c, 1) as u32;
    c.g[V0][0] = this.off(0x13e4).lw(c);
    let dst = c.g[V0][0] as u32;
    c.lqc2(4, dst);
    c.lqc2(5, dst.wrapping_add(0x10));
    c.lqc2(6, dst.wrapping_add(0x20));
    c.lqc2(7, dst.wrapping_add(0x30));
    c.lqc2(8, v);
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, dst.wrapping_add(0x30));
    this.off(0x6b90).sw(c, 0);
    abi::ret(c);
}

/// `void cPS2GridMesh_SetScale(self, range)` (0x396110): store the range and
/// the reciprocal scale `32767 / range`.
pub fn c_ps2_grid_mesh_set_scale(c: &mut Cpu) {
    c.cycles += 5;
    let gp = c.g[GP][0] as u32;
    c.lwc1(0, gp.wrapping_sub(0x254c));
    let this = Addr(abi::arg(c, 0));
    c.swc1(12, this.off(0x194).0);
    c.div_s(0, 0, 12);
    c.swc1(0, this.off(0x198).0);
    abi::ret(c);
}

/// `void RailTube_SetPoint(self, index, point)` (0x3968d0): copy the quadword
/// `point` into the tube's point array (`ptr + index * 0x10`).
pub fn rail_tube_set_point(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x8).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
    let point = abi::arg(c, 2) as u32;
    c.lq(3, point);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let dst = c.g[A1][0] as u32;
    c.sq(3, dst);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory-card port/slot state.

/// `void cPS2MemCard_ClearPortChanged(self)` (0x2c44e0): clear the port's
/// `+0x18c` field of the current level and, if it was -1, its `+0x184` field.
pub fn c_ps2_mem_card_clear_port_changed(c: &mut Cpu) {
    c.cycles += 13;
    let this = abi::arg(c, 0) as u32;
    c.g[V1][0] = Addr(this).off(0xc).lw(c);
    c.g[A1][0] = 0x14;
    c.g[A2][0] = sx(this.wrapping_add(0x184));
    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
    c.mult(0, V0, V1, A1);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
    let level = c.g[V1][0] as u32;
    Addr(level).off(0x18c).sw(c, 0);
    c.g[V0][0] = Addr(this).off(0xc).lw(c);
    c.mult(0, V0, V0, A1);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let word = c.g[A2][0] as u32;
    c.g[V1][0] = Addr(word).lw(c);
    if c.g[V1][0] == c.g[A3][0] {
        Addr(word).sw(c, 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsState34(self)` (0x2c4748): state word `+4` equals 0x34.
pub fn c_ps2_mem_card_is_state34(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x4).lw(c);
    c.g[V0][0] ^= 0x34;
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsState35(self)` (0x2c4880): state word `+4` equals 0x35.
pub fn c_ps2_mem_card_is_state35(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x4).lw(c);
    c.g[V0][0] ^= 0x35;
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsBusy(self)` (0x2c5140): busy while the state word at
/// `+4` or the flag at `+0x40` is non-zero.
pub fn c_ps2_mem_card_is_busy(c: &mut Cpu) {
    'busy: {
        c.cycles += 3;
        let this = Addr(abi::arg(c, 0));
        c.g[V0][0] = this.off(0x4).lw(c);
        if c.g[V0][0] != 0 {
            break 'busy;
        }
        c.cycles += 3;
        c.g[V0][0] = this.off(0x40).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        break 'busy;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortCardError(self)` (0x2c5168): whether the current
/// level's error word is one of -1, -3, -8..-10, -10001, -10002.
pub fn c_ps2_mem_card_is_port_card_error(c: &mut Cpu) {
    'outer: {
        'inner: {
            c.cycles += 8;
            let this = abi::arg(c, 0) as u32;
            c.g[V0][0] = Addr(this).off(0xc).lw(c);
            c.g[V1][0] = 0x14;
            c.mult(0, A1, V0, V1);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(this));
            let level = c.g[V0][0] as u32;
            c.g[A0][0] = Addr(level).off(0x184).lw(c);
            c.g[V1][0] = ((c.g[A0][0] as i64) < -7) as u64;
            let ge_minus7 = c.g[V1][0] == 0;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -10) as u64;
            if ge_minus7 {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffd;
                let is_minus3 = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                if is_minus3 {
                    break 'outer;
                }
                c.cycles += 2;
                let not_minus1 = c.g[A0][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if not_minus1 {
                    break 'inner;
                }
                break 'outer;
            }
            c.cycles += 2;
            let ge_minus10 = c.g[V0][0] == 0;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -10000) as u64;
            if ge_minus10 {
                break 'outer;
            }
            c.cycles += 2;
            let ge_minus10000 = c.g[V0][0] == 0;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -10002) as u64;
            if ge_minus10000 {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'inner;
            }
            c.cycles += 2;
            let lt_minus10002 = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if lt_minus10002 {
                break 'inner;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortNotPS2Card(self, port)` (0x2c5300): the port's
/// `+0x184` word equals -10002.
pub fn c_ps2_mem_card_is_port_not_ps2_card(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x14;
    c.g[V1][0] = 0xffff_ffff_ffff_d8ee;
    c.mult(0, A2, A1, V0);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
    let level = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(level).off(0x184).lw(c);
    c.g[V0][0] ^= c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_v52(self, port)` (0x2c5358): the level error word is -7,
/// -10001 or in {-5..0}.
pub fn c_ps2_mem_card_v52(c: &mut Cpu) {
    'outer: {
        c.cycles += 7;
        let this = abi::arg(c, 0) as u32;
        c.g[V0][0] = 0x14;
        c.g[V1][0] = 0xffff_ffff_ffff_fff9;
        c.mult(0, A2, A1, V0);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
        let level = c.g[A0][0] as u32;
        c.g[A0][0] = Addr(level).off(0x184).lw(c);
        let is_minus7 = c.g[A0][0] == c.g[V1][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < -6) as u64;
        if is_minus7 {
            break 'outer;
        }
        'inner: {
            'check: {
                c.cycles += 2;
                let ge_minus6 = c.g[V0][0] == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_d8ef;
                if ge_minus6 {
                    break 'check;
                }
                c.cycles += 2;
                let is_minus10001 = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 0;
                if is_minus10001 {
                    break 'outer;
                }
                c.cycles += 2;
                break 'inner;
            }
            c.cycles += 2;
            let positive = (c.g[A0][0] as i64) > 0;
            c.g[V0][0] = 0;
            if positive {
                break 'inner;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -5) as u64;
            let lt_minus5 = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if lt_minus5 {
                break 'inner;
            }
            break 'outer;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortFormatted(self, port)` (0x2c53b0): the port's
/// `+0x188` word is non-zero.
pub fn c_ps2_mem_card_is_port_formatted(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x14;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
    let level = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(level).off(0x188).lw(c);
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_HasSlotName(self, slot)` (0x2c5528): whether the slot's
/// name at `+0x1b0` is non-empty.
pub fn c_ps2_mem_card_has_slot_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let this = abi::arg(c, 0) as u32;
        c.g[V0][0] = 0x78;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        // V1 = slot * 0x78 (the slot stride); V0 keeps 0x78 as the multiplier.
        c.mult(0, V1, A1, V0);
        let sp = c.g[SP][0] as u32;
        c.sd(sp, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b0));
        call_ptr(c, 0x0041_6810, 0x002c_5544)?;
        c.g[RA][0] = c.ld(sp);
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        c.g[SP][0] = sx(sp.wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    });
}

/// `void cPS2MemCard_MarkSlotUsed(self, slot)` (0x2c5558): set the slot's
/// `+0x1a4` flag.
pub fn c_ps2_mem_card_mark_slot_used(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x78;
    c.g[V1][0] = 0x1;
    c.mult(0, A2, A1, V0);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
    let slot = c.g[A0][0] as u32;
    c.g[V1][0] = 0x1;
    Addr(slot).off(0x1a4).sw(c, 1);
    abi::ret(c);
}

/// `bool cPS2MemCard_IsSlotEmpty(self, slot)` (0x2c5588): whether both the
/// `+0x1a8` and `+0x1a4` slot words are zero.
pub fn c_ps2_mem_card_is_slot_empty(c: &mut Cpu) {
    'taken: {
        c.cycles += 7;
        c.g[V1][0] = 0x78;
        c.g[A2][0] = c.g[A0][0];
        c.mult(0, A0, A1, V1);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let slot = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(slot).off(0x1a8).lw(c);
        if c.g[V1][0] != 0 {
            break 'taken;
        }
        c.cycles += 3;
        c.g[V1][0] = Addr(slot).off(0x1a4).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        break 'taken;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_SlotHas1AC(self, slot)` (0x2c55c0): the slot's `+0x1ac`
/// word is non-zero.
pub fn c_ps2_mem_card_slot_has_1ac(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x78;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
    let slot = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(slot).off(0x1ac).lw(c);
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortResultM11(self)` (0x2c66d8): the current level's
/// error word equals -11.
pub fn c_ps2_mem_card_is_port_result_m11(c: &mut Cpu) {
    c.cycles += 9;
    let this = abi::arg(c, 0) as u32;
    c.g[V1][0] = Addr(this).off(0xc).lw(c);
    c.g[V0][0] = 0x14;
    c.g[A1][0] = 0xffff_ffff_ffff_fff5;
    c.mult(0, A2, V1, V0);
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
    let level = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(level).off(0x184).lw(c);
    c.g[V0][0] ^= c.g[A1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `int cPS2MemCard_CalcSaveSizeKB(size)` (0x2c6758): the save size in KiB
/// (1 KiB blocks, plus 77 KiB of header/overhead).
pub fn c_ps2_mem_card_calc_save_size_kb(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3ff));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7fe));
    if c.g[V0][0] != 0 {
        c.g[A1][0] = c.g[V1][0];
    }
    c.g[A1][0] = sx((((c.g[A1][0] as u32) as i32) >> 10) as u32);
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4d));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Camera getters.

/// Copy the quadword at `*(self + 4) + off` to `out` and return `out`.
fn camera_copy_quad(c: &mut Cpu, off: u32) {
    c.cycles += 5;
    let out = abi::arg(c, 0) as u32;
    let owner = Addr(abi::arg(c, 1) as u32);
    c.g[A2][0] = owner.off(0x4).lw(c);
    c.g[V0][0] = c.g[A0][0];
    let src = Addr(c.g[A2][0] as u32).off(off);
    c.lq(3, src.0);
    c.sq(3, out);
    abi::ret(c);
}

/// `void* RiderCameraTarget_GetOrientation(out, target)` (0x15f598).
pub fn rider_camera_target_get_orientation(c: &mut Cpu) {
    camera_copy_quad(c, 0x120);
}

/// `void* RiderCameraTarget_GetForward(out, target)` (0x15f5f8).
pub fn rider_camera_target_get_forward(c: &mut Cpu) {
    camera_copy_quad(c, 0x1b0);
}

/// `void* RiderCameraTarget_GetSide(out, target)` (0x15f610).
pub fn rider_camera_target_get_side(c: &mut Cpu) {
    camera_copy_quad(c, 0x1a0);
}

/// `void* RiderCameraTarget_GetUp(out, target)` (0x15f628).
pub fn rider_camera_target_get_up(c: &mut Cpu) {
    camera_copy_quad(c, 0x1c0);
}

/// `void* RiderCameraTarget_GetGroundNormal(out, target)` (0x15f650).
pub fn rider_camera_target_get_ground_normal(c: &mut Cpu) {
    camera_copy_quad(c, 0x370);
}

/// `void* RiderCameraTarget_GetPrevGroundNormal(out, target)` (0x15f668).
pub fn rider_camera_target_get_prev_ground_normal(c: &mut Cpu) {
    camera_copy_quad(c, 0x380);
}

/// `float RiderCameraTarget_GetAirTimeRemaining(self)` (0x15f6b0): the `+0x98`
/// timer minus the `+0xa0` limit.
pub fn rider_camera_target_get_air_time_remaining(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x4).lw(c);
    let rider = c.g[V0][0] as u32;
    c.g[V1][0] = Addr(rider).off(0x788).lw(c);
    let state = c.g[V1][0] as u32;
    c.lwc1(1, Addr(state).off(0xa0).0);
    c.lwc1(0, Addr(state).off(0x98).0);
    c.sub_s(0, 0, 1);
    abi::ret(c);
}

/// `float RiderCameraTarget_GetAirTime9cRemaining(self)` (0x15f6c8): the
/// `+0x9c` timer minus the `+0xa0` limit.
pub fn rider_camera_target_get_air_time_9c_remaining(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x4).lw(c);
    let rider = c.g[V0][0] as u32;
    c.g[V1][0] = Addr(rider).off(0x788).lw(c);
    let state = c.g[V1][0] as u32;
    c.lwc1(1, Addr(state).off(0xa0).0);
    c.lwc1(0, Addr(state).off(0x9c).0);
    c.sub_s(0, 0, 1);
    abi::ret(c);
}

/// `bool RiderCameraTarget_IsLandingPredicted(self)` (0x15f720): surface
/// state `+0xac` is 1 or 3.
pub fn rider_camera_target_is_landing_predicted(c: &mut Cpu) {
    'outer: {
        'inner: {
            c.cycles += 6;
            let this = Addr(abi::arg(c, 0));
            c.g[V0][0] = this.off(0x4).lw(c);
            c.g[A0][0] = 0x1;
            let rider = c.g[V0][0] as u32;
            c.g[V1][0] = Addr(rider).off(0x788).lw(c);
            let state = c.g[V1][0] as u32;
            c.g[V1][0] = Addr(state).off(0xac).lw(c);
            let is_one = c.g[V1][0] == c.g[A0][0];
            c.g[A1][0] = 0;
            if is_one {
                break 'inner;
            }
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            let is_three = c.g[V1][0] == c.g[V0][0];
            if !is_three {
                break 'outer;
            }
        }
        c.cycles += 1;
        c.g[A1][0] = 0x1;
        break 'outer;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    abi::ret(c);
}

/// `void* CameraAlgorithm_GetLookAt(out)` (0x167e40): copy the camera
/// algorithm's look-at vector from the `.lit4` at `0x4ff120`.
pub fn camera_algorithm_get_look_at(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x50_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xee0));
    let src = c.g[V1][0] as u32;
    let out = c.g[A0][0] as u32;
    c.lq(5, src);
    c.sq(5, out);
    abi::ret(c);
}

/// `void* RiderCameraAlgorithm_GetLookAt(out, self)` (0x167fa0): copy the
/// look-at vector at `self + 0x20`.
pub fn rider_camera_algorithm_get_look_at(c: &mut Cpu) {
    c.cycles += 4;
    let owner = Addr(abi::arg(c, 1));
    let out = abi::arg(c, 0) as u32;
    c.lq(3, owner.off(0x20).0);
    c.g[V0][0] = c.g[A0][0];
    c.sq(3, out);
    abi::ret(c);
}

/// `float MidRightChaseCameraAlgorithm_GetEyeDrop(self)` (0x168a30): the
/// constant eye drop.
pub fn mid_right_chase_camera_algorithm_get_eye_drop(c: &mut Cpu) {
    c.cycles += 3;
    let gp = c.g[GP][0] as u32;
    c.lwc1(0, gp.wrapping_sub(0x66d0));
    abi::ret(c);
}

/// `void MidRightChaseCameraAlgorithm_GetTuning(self, out)` (0x168a40):
/// fill the tuning struct.
pub fn mid_right_chase_camera_algorithm_get_tuning(c: &mut Cpu) {
    c.cycles += 11;
    let gp = c.g[GP][0] as u32;
    c.lwc1(3, gp.wrapping_sub(0x66cc));
    c.g[V0][0] = 0x1;
    c.lwc1(2, gp.wrapping_sub(0x66c8));
    c.lwc1(0, gp.wrapping_sub(0x66c4));
    c.lwc1(1, gp.wrapping_sub(0x66c0));
    let out = abi::arg(c, 1) as u32;
    Addr(out).off(0x10).sw(c, 1);
    c.swc1(3, out.wrapping_add(0x4));
    c.swc1(2, out);
    c.swc1(0, out.wrapping_add(0xc));
    c.swc1(1, out.wrapping_add(0x8));
    abi::ret(c);
}

/// `float MidLeftChaseCameraAlgorithm_GetEyeDrop(self)` (0x168d40): the
/// constant eye drop.
pub fn mid_left_chase_camera_algorithm_get_eye_drop(c: &mut Cpu) {
    c.cycles += 3;
    let gp = c.g[GP][0] as u32;
    c.lwc1(0, gp.wrapping_sub(0x660c));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::{check, put_vec, scramble, Rng, DATA, GAME_GP};

    /// Write `words` at `addr`.
    fn words(c: &mut Cpu, addr: u32, v: &[u32]) {
        for (i, w) in v.iter().enumerate() {
            c.bus.write32(addr + 4 * i as u32, *w);
        }
    }

    fn rand_vec(c: &mut Cpu, r: &mut Rng, addr: u32) {
        put_vec(c, addr, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
    }

    #[test]
    fn menu_sys_spacer_item_measure_matches() {
        check(ssx3_ee::s09::f_002cd3a0__generated, menu_sys_spacer_item_measure, 200, &|c, seed| {
            let mut r = Rng(0x2cd3_a000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA + 0x200) as u64;
            c.g[A1][0] = (DATA + 0x300) as u64;
            words(c, DATA + 0x300, &[
                r.next_u32(),
                r.next_u32(),
                r.next_u32(),
                r.next_u32(),
                r.next_u32(),
                r.next_u32(),
            ]);
        });
    }

    #[test]
    fn menu_sys_int8_item_reset_value_matches() {
        check(ssx3_ee::s09::f_002cd578__generated, menu_sys_int8_item_reset_value, 200, &|c, seed| {
            let mut r = Rng(0x2cd5_7800 ^ seed as u64);
            scramble(c, &mut r);
            let item = DATA;
            let dst = DATA + 0x400;
            c.g[A0][0] = item as u64;
            c.bus.write32(item + 0x18, dst);
            let byte = match seed % 4 {
                0 => 0,
                1 => 0x7f,
                2 => 0x80,
                _ => r.next_u32() as u8,
            };
            c.bus.write8(item + 0x1c, byte);
            c.bus.write8(dst, r.next_u32() as u8);
        });
    }

    #[test]
    fn menu_sys_int_item_reset_value_matches() {
        check(ssx3_ee::s09::f_002cd9b8__generated, menu_sys_int_item_reset_value, 200, &|c, seed| {
            let mut r = Rng(0x2cd9_b800 ^ seed as u64);
            scramble(c, &mut r);
            let item = DATA;
            let dst = DATA + 0x400;
            c.g[A0][0] = item as u64;
            c.bus.write32(item + 0x18, dst);
            let v = match seed % 4 {
                0 => 0,
                1 => -1i32,
                2 => r.next_u32() as i32,
                _ => (r.next_u32() % 1000) as i32,
            };
            c.bus.write32(item + 0x1c, v as u32);
            c.bus.write32(dst, r.next_u32());
        });
    }

    #[test]
    fn menu_sys_float_item_reset_value_matches() {
        check(ssx3_ee::s09::f_002cdf78__generated, menu_sys_float_item_reset_value, 200, &|c, seed| {
            let mut r = Rng(0x2cdf_7800 ^ seed as u64);
            scramble(c, &mut r);
            let item = DATA;
            let dst = DATA + 0x400;
            c.g[A0][0] = item as u64;
            c.bus.write32(item + 0x18, dst);
            let v = match seed % 5 {
                0 => 0.0f32.to_bits(),
                1 => (-1.0f32).to_bits(),
                2 => 2.5f32.to_bits(),
                3 => f32::MIN_POSITIVE.to_bits(),
                _ => r.edge_f32(),
            };
            c.bus.write32(item + 0x1c, v);
            c.bus.write32(dst, r.next_u32());
        });
    }

    /// Build `n` linked event nodes for the span/ramp track queries.
    fn build_nodes(c: &mut Cpu, r: &mut Rng, addr: u32, n: u32, time_off: u32, next_off: u32) {
        for i in 0..n {
            let node = addr + i * 0x40;
            c.bus.write32(node + time_off, r.next_u32());
            c.bus.write32(node + next_off, if i + 1 < n { node + 0x40 } else { 0 });
        }
    }

    #[test]
    fn in_seq_span_track_get_next_event_time_matches() {
        check_events(
            ssx3_ee::s07::f_00273040__generated,
            in_seq_span_track_get_next_event_time,
            120,
            &|c, seed| {
                let mut r = Rng(0x2730_4000 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA;
                c.g[A0][0] = track as u64;
                let has_table = seed % 5 != 0;
                c.bus.write32(track + 0x8, if has_table { 1 } else { 0 });
                c.bus.write32(track + 0x10, DATA + 0x100);
                c.bus.write16(DATA + 0x100, if seed % 7 == 0 { 0 } else { 1 });
                let n = match seed % 4 {
                    0 => 0,
                    1 | 2 => 2,
                    _ => 4,
                };
                let head = if n == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(track + 0x24, head);
                build_nodes(c, &mut r, DATA + 0x1000, n, 0xc, 0x18);
                let current = if seed % 3 == 0 { 0 } else { DATA + 0x2000 };
                c.bus.write32(track + 0x14, current);
                if current != 0 {
                    c.bus.write16(current + 0x4, r.next_u32() as u16);
                }
                let mode = schedule(c, &mut r, seed, 0x0027_3040);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn in_seq_event_track_get_next_event_time_matches() {
        check(ssx3_ee::s07::f_002733e0__generated, in_seq_event_track_get_next_event_time, 160, &|c, seed| {
            let mut r = Rng(0x2733_e000 ^ seed as u64);
            scramble(c, &mut r);
            let track = DATA;
            c.g[A0][0] = track as u64;
            c.bus.write32(track + 0x8, if seed % 5 == 0 { 0 } else { DATA + 0x100 });
            c.bus.write16(
                DATA + 0x100,
                if seed % 3 == 0 { 0 } else { r.next_u32() as u16 },
            );
            let current = if seed % 4 == 0 { 0 } else { DATA + 0x200 };
            c.bus.write32(track + 0x14, current);
            if current != 0 {
                c.bus.write16(current + 0x2, r.next_u32() as u16);
            }
        });
    }

    #[test]
    fn in_seq_ramp_track_get_next_event_time_matches() {
        check_events(
            ssx3_ee::s07::f_002737e0__generated,
            in_seq_ramp_track_get_next_event_time,
            120,
            &|c, seed| {
                let mut r = Rng(0x2737_e000 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA;
                c.g[A0][0] = track as u64;
                let has_table = seed % 5 != 0;
                c.bus.write32(track + 0x8, if has_table { 1 } else { 0 });
                c.bus.write32(track + 0x10, DATA + 0x100);
                c.bus.write16(DATA + 0x100, if seed % 7 == 0 { 0 } else { 1 });
                let n = match seed % 4 {
                    0 => 0,
                    1 | 2 => 2,
                    _ => 4,
                };
                let head = if n == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(track + 0x20, head);
                build_nodes(c, &mut r, DATA + 0x1000, n, 0x8, 0x20);
                let current = if seed % 3 == 0 { 0 } else { DATA + 0x2000 };
                c.bus.write32(track + 0x14, current);
                if current != 0 {
                    c.bus.write16(current + 0x4, r.next_u32() as u16);
                }
                let mode = schedule(c, &mut r, seed, 0x0027_37e0);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn graphics_man_set_screen_offset_matches() {
        check(ssx3_ee::s13::f_00393fb8__generated, graphics_man_set_screen_offset, 160, &|c, seed| {
            let mut r = Rng(0x393f_b800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = sx(r.next_u32());
            words(c, DATA, &[r.next_u32(), r.next_u32()]);
        });
    }

    #[test]
    fn graphics_man_get_screen_offset_x_matches() {
        check(ssx3_ee::s13::f_00393fd8__generated, graphics_man_get_screen_offset_x, 200, &|c, seed| {
            let mut r = Rng(0x393f_d800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.edge_f32());
        });
    }

    #[test]
    fn graphics_man_get_screen_offset_y_matches() {
        check(ssx3_ee::s13::f_00393ff0__generated, graphics_man_get_screen_offset_y, 200, &|c, seed| {
            let mut r = Rng(0x393f_f000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 4, r.edge_f32());
        });
    }

    #[test]
    fn graphics_man_v58_matches() {
        check(ssx3_ee::s13::f_003942a0__generated, graphics_man_v58, 160, &|c, seed| {
            let mut r = Rng(0x3942_a000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            words(c, DATA, &[r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
        });
    }

    #[test]
    fn light_man_next_id_matches() {
        check(ssx3_ee::s13::f_00394d18__generated, light_man_next_id, 200, &|c, seed| {
            let mut r = Rng(0x394d_1800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 8, r.next_u32());
            c.bus.write32(DATA + 0xc, r.next_u32() % 100);
        });
    }

    fn screen_entry(c: &mut Cpu, r: &mut Rng, off: u32) {
        c.g[A0][0] = DATA as u64;
        let idx = r.next_u32() % 3;
        let entry = DATA + 0x400;
        c.bus.write32(DATA + 0x59c8, idx);
        c.bus.write32(DATA + 0x59cc, entry);
        c.bus.write32(entry + idx * 0xa4 + off, r.next_u32());
        c.bus.write32(entry + idx * 0xa4 + 0x88, r.next_u32());
        c.bus.write32(entry + idx * 0xa4 + 0x8c, r.next_u32());
    }

    #[test]
    fn ps2_graphics_man_get_screen_width_matches() {
        check(ssx3_ee::s13::f_00395330__generated, ps2_graphics_man_get_screen_width, 160, &|c, seed| {
            let mut r = Rng(0x3953_3000 ^ seed as u64);
            scramble(c, &mut r);
            screen_entry(c, &mut r, 0x88);
        });
    }

    #[test]
    fn ps2_graphics_man_get_screen_height_matches() {
        check(ssx3_ee::s13::f_00395350__generated, ps2_graphics_man_get_screen_height, 160, &|c, seed| {
            let mut r = Rng(0x3953_5000 ^ seed as u64);
            scramble(c, &mut r);
            screen_entry(c, &mut r, 0x8c);
        });
    }

    #[test]
    fn ps2_graphics_man_set_lighting_matches() {
        check_events(
            ssx3_ee::s13::f_003954d0__generated,
            ps2_graphics_man_set_lighting,
            120,
            &|c, seed| {
                let mut r = Rng(0x3954_d000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let src = DATA + 0x800;
                c.g[A1][0] = src as u64;
                for i in 0..10 {
                    rand_vec(c, &mut r, src + i * 0x10);
                }
                let mode = schedule(c, &mut r, seed, 0x0039_54d0);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn ps2_graphics_man_push_matrix_matches() {
        check(ssx3_ee::s13::f_003956e8__generated, ps2_graphics_man_push_matrix, 160, &|c, seed| {
            let mut r = Rng(0x3956_e800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let top = DATA + 0x1000;
            c.bus.write32(DATA + 0x13e4, top);
            c.bus.write32(DATA + 0x13e0, r.next_u32() % 16);
            for i in 0..4 {
                rand_vec(c, &mut r, top + i * 0x10);
                rand_vec(c, &mut r, top + 0x40 + i * 0x10);
            }
        });
    }

    #[test]
    fn ps2_graphics_man_load_matrix_matches() {
        check(ssx3_ee::s13::f_00395c38__generated, ps2_graphics_man_load_matrix, 160, &|c, seed| {
            let mut r = Rng(0x395c_3800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let top = DATA + 0x1000;
            let m = DATA + 0x2000;
            c.bus.write32(DATA + 0x13e4, top);
            c.bus.write32(DATA + 0x6b90, r.next_u32());
            for i in 0..4 {
                rand_vec(c, &mut r, top + i * 0x10);
                rand_vec(c, &mut r, m + i * 0x10);
            }
        });
    }

    #[test]
    fn ps2_graphics_man_mult_matrix_matches() {
        check(ssx3_ee::s13::f_00395c70__generated, ps2_graphics_man_mult_matrix, 160, &|c, seed| {
            let mut r = Rng(0x395c_7000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let top = DATA + 0x1000;
            let m = DATA + 0x2000;
            c.bus.write32(DATA + 0x13e4, top);
            c.bus.write32(DATA + 0x6b90, r.next_u32());
            for i in 0..4 {
                rand_vec(c, &mut r, top + i * 0x10);
                rand_vec(c, &mut r, m + i * 0x10);
            }
        });
    }

    #[test]
    fn ps2_graphics_man_load_identity_matches() {
        check(ssx3_ee::s13::f_00395cf0__generated, ps2_graphics_man_load_identity, 160, &|c, seed| {
            let mut r = Rng(0x395c_f000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let top = DATA + 0x1000;
            c.bus.write32(DATA + 0x13e4, top);
            c.bus.write32(DATA + 0x6b90, r.next_u32());
            // The .lit4 identity the original copies.
            put_vec(c, 0x004f_f1a0, [0x3f80_0000, 0, 0, 0]);
            put_vec(c, 0x004f_f1b0, [0, 0x3f80_0000, 0, 0]);
            put_vec(c, 0x004f_f1c0, [0, 0, 0x3f80_0000, 0]);
            put_vec(c, 0x004f_f1d0, [0, 0, 0, 0x3f80_0000]);
            for i in 0..4 {
                rand_vec(c, &mut r, top + i * 0x10);
            }
        });
    }

    #[test]
    fn ps2_graphics_man_translate_matches() {
        check(ssx3_ee::s13::f_00395d28__generated, ps2_graphics_man_translate, 160, &|c, seed| {
            let mut r = Rng(0x395d_2800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let top = DATA + 0x1000;
            let v = DATA + 0x2000;
            c.bus.write32(DATA + 0x13e4, top);
            c.bus.write32(DATA + 0x6b90, r.next_u32());
            for i in 0..4 {
                rand_vec(c, &mut r, top + i * 0x10);
            }
            rand_vec(c, &mut r, v);
        });
    }

    #[test]
    fn c_ps2_grid_mesh_set_scale_matches() {
        check(ssx3_ee::s13::f_00396110__generated, c_ps2_grid_mesh_set_scale, 200, &|c, seed| {
            let mut r = Rng(0x3961_1000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP - 0x254c, 32767.0f32.to_bits());
            c.f[12] = if seed % 4 == 0 { 0 } else { r.edge_f32() };
            c.bus.write32(DATA + 0x194, r.next_u32());
            c.bus.write32(DATA + 0x198, r.next_u32());
        });
    }

    #[test]
    fn rail_tube_set_point_matches() {
        check(ssx3_ee::s13::f_003968d0__generated, rail_tube_set_point, 160, &|c, seed| {
            let mut r = Rng(0x3968_d000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            c.g[A2][0] = (DATA + 0x2000) as u64;
            c.bus.write32(DATA + 8, DATA + 0x1000);
            rand_vec(c, &mut r, DATA + 0x2000);
            for i in 0..4 {
                rand_vec(c, &mut r, DATA + 0x1000 + i * 0x10);
            }
        });
    }

    #[test]
    fn c_ps2_mem_card_clear_port_changed_matches() {
        check(ssx3_ee::s08::f_002c44e0__generated, c_ps2_mem_card_clear_port_changed, 160, &|c, seed| {
            let mut r = Rng(0x2c44_e000 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let level = seed % 4;
            c.bus.write32(this + 0xc, level);
            for i in 0..4u32 {
                c.bus.write32(this + 0x184 + i * 0x14, r.next_u32());
                c.bus.write32(this + 0x18c + i * 0x14, r.next_u32());
            }
            c.bus.write32(this + 0x184 + level * 0x14, if seed % 3 == 0 { u32::MAX } else { r.next_u32() });
        });
    }

    #[test]
    fn c_ps2_mem_card_is_state34_matches() {
        check(ssx3_ee::s08::f_002c4748__generated, c_ps2_mem_card_is_state34, 160, &|c, seed| {
            let mut r = Rng(0x2c47_4800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 4, if seed % 3 == 0 { 0x34 } else { r.next_u32() });
        });
    }

    #[test]
    fn c_ps2_mem_card_is_state35_matches() {
        check(ssx3_ee::s08::f_002c4880__generated, c_ps2_mem_card_is_state35, 160, &|c, seed| {
            let mut r = Rng(0x2c48_8000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 4, if seed % 3 == 0 { 0x35 } else { r.next_u32() });
        });
    }

    #[test]
    fn c_ps2_mem_card_is_busy_matches() {
        check(ssx3_ee::s08::f_002c5140__generated, c_ps2_mem_card_is_busy, 160, &|c, seed| {
            let mut r = Rng(0x2c51_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 4, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(DATA + 0x40, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
        });
    }

    #[test]
    fn c_ps2_mem_card_is_port_card_error_matches() {
        check(ssx3_ee::s08::f_002c5168__generated, c_ps2_mem_card_is_port_card_error, 220, &|c, seed| {
            let mut r = Rng(0x2c51_6800 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let level = seed % 3;
            c.bus.write32(this + 0xc, level);
            let values = [
                0xffff_ffffu32,
                0xffff_fffd,
                0xffff_fff8,
                0xffff_fff7,
                0xffff_fff6,
                0x0000_d8ef,
                0x0000_d8ee,
                0xffff_d8ef,
                0xffff_d8ee,
                0,
                5,
                0x8000_0000,
            ];
            c.bus.write32(this + 0x184 + level * 0x14, values[(seed as usize) % values.len()]);
        });
    }

    #[test]
    fn c_ps2_mem_card_is_port_not_ps2_card_matches() {
        check(ssx3_ee::s08::f_002c5300__generated, c_ps2_mem_card_is_port_not_ps2_card, 160, &|c, seed| {
            let mut r = Rng(0x2c53_0000 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let port = seed % 3;
            c.g[A1][0] = port as u64;
            c.bus.write32(
                this + 0x184 + port * 0x14,
                if seed % 2 == 0 { 0xffff_d8ee } else { r.next_u32() },
            );
        });
    }

    #[test]
    fn c_ps2_mem_card_v52_matches() {
        check(ssx3_ee::s08::f_002c5358__generated, c_ps2_mem_card_v52, 220, &|c, seed| {
            let mut r = Rng(0x2c53_5800 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let port = seed % 3;
            c.g[A1][0] = port as u64;
            let values = [
                0xffff_fff9u32, // -7
                0xffff_d8ef,    // -10001
                0xffff_fffb,    // -5
                0xffff_fffc,    // -4
                0xffff_fffd,    // -3
                0xffff_fffe,    // -2
                0xffff_ffff,    // -1
                0,
                0xffff_fff8, // -8
                5,
                0xffff_fffa, // -6
                0x8000_0000,
            ];
            c.bus.write32(this + 0x184 + port * 0x14, values[(seed as usize) % values.len()]);
        });
    }

    #[test]
    fn c_ps2_mem_card_is_port_formatted_matches() {
        check(ssx3_ee::s08::f_002c53b0__generated, c_ps2_mem_card_is_port_formatted, 160, &|c, seed| {
            let mut r = Rng(0x2c53_b000 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let port = seed % 3;
            c.g[A1][0] = port as u64;
            c.bus.write32(
                this + 0x188 + port * 0x14,
                if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
            );
        });
    }

    #[test]
    fn c_ps2_mem_card_has_slot_name_matches() {
        check(ssx3_ee::s08::f_002c5528__generated, c_ps2_mem_card_has_slot_name, 120, &|c, seed| {
            let mut r = Rng(0x2c55_2800 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let slot = r.next_u32() % 3;
            c.g[A1][0] = slot as u64;
            let name = this + slot * 0x78 + 0x1b0;
            let len = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 5,
                _ => 12,
            };
            for i in 0..len {
                c.bus.write8(name + i, b'A' + (seed % 26) as u8);
            }
            c.bus.write8(name + len, 0);
        });
    }

    #[test]
    fn c_ps2_mem_card_mark_slot_used_matches() {
        check(ssx3_ee::s08::f_002c5558__generated, c_ps2_mem_card_mark_slot_used, 160, &|c, seed| {
            let mut r = Rng(0x2c55_5800 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let slot = r.next_u32() % 3;
            c.g[A1][0] = slot as u64;
            c.bus.write32(this + slot * 0x78 + 0x1a4, r.next_u32());
        });
    }

    #[test]
    fn c_ps2_mem_card_is_slot_empty_matches() {
        check(ssx3_ee::s08::f_002c5588__generated, c_ps2_mem_card_is_slot_empty, 160, &|c, seed| {
            let mut r = Rng(0x2c55_8800 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let slot = r.next_u32() % 3;
            c.g[A1][0] = slot as u64;
            c.bus.write32(this + slot * 0x78 + 0x1a8, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(this + slot * 0x78 + 0x1a4, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
        });
    }

    #[test]
    fn c_ps2_mem_card_slot_has_1ac_matches() {
        check(ssx3_ee::s08::f_002c55c0__generated, c_ps2_mem_card_slot_has_1ac, 160, &|c, seed| {
            let mut r = Rng(0x2c55_c000 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let slot = r.next_u32() % 3;
            c.g[A1][0] = slot as u64;
            c.bus.write32(
                this + slot * 0x78 + 0x1ac,
                if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
            );
        });
    }

    #[test]
    fn c_ps2_mem_card_is_port_result_m11_matches() {
        check(ssx3_ee::s08::f_002c66d8__generated, c_ps2_mem_card_is_port_result_m11, 160, &|c, seed| {
            let mut r = Rng(0x2c66_d800 ^ seed as u64);
            scramble(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            let level = seed % 3;
            c.bus.write32(this + 0xc, level);
            c.bus.write32(
                this + 0x184 + level * 0x14,
                if seed % 2 == 0 { 0xffff_fff5 } else { r.next_u32() },
            );
        });
    }

    #[test]
    fn c_ps2_mem_card_calc_save_size_kb_matches() {
        check(ssx3_ee::s08::f_002c6758__generated, c_ps2_mem_card_calc_save_size_kb, 200, &|c, seed| {
            let mut r = Rng(0x2c67_5800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = sx(r.next_u32());
        });
    }

    fn camera_target_fixture(c: &mut Cpu, r: &mut Rng, off: u32) {
        scramble(c, r);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (DATA + 0x100) as u64;
        c.bus.write32(DATA + 0x104, DATA + 0x400);
        rand_vec(c, r, DATA + 0x400 + off);
    }

    #[test]
    fn rider_camera_target_get_orientation_matches() {
        check(ssx3_ee::s01::f_0015f598__generated, rider_camera_target_get_orientation, 120, &|c, seed| {
            let mut r = Rng(0x15f5_9800 ^ seed as u64);
            camera_target_fixture(c, &mut r, 0x120);
        });
    }

    #[test]
    fn rider_camera_target_get_forward_matches() {
        check(ssx3_ee::s01::f_0015f5f8__generated, rider_camera_target_get_forward, 120, &|c, seed| {
            let mut r = Rng(0x15f5_f800 ^ seed as u64);
            camera_target_fixture(c, &mut r, 0x1b0);
        });
    }

    #[test]
    fn rider_camera_target_get_side_matches() {
        check(ssx3_ee::s01::f_0015f610__generated, rider_camera_target_get_side, 120, &|c, seed| {
            let mut r = Rng(0x15f6_1000 ^ seed as u64);
            camera_target_fixture(c, &mut r, 0x1a0);
        });
    }

    #[test]
    fn rider_camera_target_get_up_matches() {
        check(ssx3_ee::s01::f_0015f628__generated, rider_camera_target_get_up, 120, &|c, seed| {
            let mut r = Rng(0x15f6_2800 ^ seed as u64);
            camera_target_fixture(c, &mut r, 0x1c0);
        });
    }

    #[test]
    fn rider_camera_target_get_ground_normal_matches() {
        check(ssx3_ee::s01::f_0015f650__generated, rider_camera_target_get_ground_normal, 120, &|c, seed| {
            let mut r = Rng(0x15f6_5000 ^ seed as u64);
            camera_target_fixture(c, &mut r, 0x370);
        });
    }

    #[test]
    fn rider_camera_target_get_prev_ground_normal_matches() {
        check(ssx3_ee::s01::f_0015f668__generated, rider_camera_target_get_prev_ground_normal, 120, &|c, seed| {
            let mut r = Rng(0x15f6_6800 ^ seed as u64);
            camera_target_fixture(c, &mut r, 0x380);
        });
    }

    fn air_time_fixture(c: &mut Cpu, r: &mut Rng, low: u32) {
        scramble(c, r);
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 4, DATA + 0x400);
        c.bus.write32(DATA + 0x400 + 0x788, DATA + 0x800);
        c.bus.write32(DATA + 0x800 + low, r.edge_f32());
        c.bus.write32(DATA + 0x800 + 0xa0, r.edge_f32());
    }

    #[test]
    fn rider_camera_target_get_air_time_remaining_matches() {
        check(ssx3_ee::s01::f_0015f6b0__generated, rider_camera_target_get_air_time_remaining, 200, &|c, seed| {
            let mut r = Rng(0x15f6_b000 ^ seed as u64);
            air_time_fixture(c, &mut r, 0x98);
        });
    }

    #[test]
    fn rider_camera_target_get_air_time_9c_remaining_matches() {
        check(ssx3_ee::s01::f_0015f6c8__generated, rider_camera_target_get_air_time_9c_remaining, 200, &|c, seed| {
            let mut r = Rng(0x15f6_c800 ^ seed as u64);
            air_time_fixture(c, &mut r, 0x9c);
        });
    }

    #[test]
    fn rider_camera_target_is_landing_predicted_matches() {
        check(ssx3_ee::s01::f_0015f720__generated, rider_camera_target_is_landing_predicted, 160, &|c, seed| {
            let mut r = Rng(0x15f7_2000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 4, DATA + 0x400);
            c.bus.write32(DATA + 0x400 + 0x788, DATA + 0x800);
            let state = match seed % 4 {
                0 => 1,
                1 => 3,
                2 => 0,
                _ => r.next_u32() as i32,
            };
            c.bus.write32(DATA + 0x800 + 0xac, state as u32);
        });
    }

    #[test]
    fn camera_algorithm_get_look_at_matches() {
        check(ssx3_ee::s02::f_00167e40__generated, camera_algorithm_get_look_at, 120, &|c, seed| {
            let mut r = Rng(0x167e_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            put_vec(c, 0x004f_f120, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            rand_vec(c, &mut r, DATA);
        });
    }

    #[test]
    fn rider_camera_algorithm_get_look_at_matches() {
        check(ssx3_ee::s02::f_00167fa0__generated, rider_camera_algorithm_get_look_at, 120, &|c, seed| {
            let mut r = Rng(0x167f_a000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            rand_vec(c, &mut r, DATA + 0x120);
            rand_vec(c, &mut r, DATA);
        });
    }

    #[test]
    fn mid_right_chase_camera_algorithm_get_eye_drop_matches() {
        check(ssx3_ee::s02::f_00168a30__generated, mid_right_chase_camera_algorithm_get_eye_drop, 120, &|c, seed| {
            let mut r = Rng(0x168a_3000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x66d0, r.edge_f32());
        });
    }

    #[test]
    fn mid_right_chase_camera_algorithm_get_tuning_matches() {
        check(ssx3_ee::s02::f_00168a40__generated, mid_right_chase_camera_algorithm_get_tuning, 160, &|c, seed| {
            let mut r = Rng(0x168a_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A1][0] = DATA as u64;
            c.bus.write32(GAME_GP - 0x66cc, 6.6548877f32.to_bits());
            c.bus.write32(GAME_GP - 0x66c8, 16.140755f32.to_bits());
            c.bus.write32(GAME_GP - 0x66c4, 0.84256876f32.to_bits());
            c.bus.write32(GAME_GP - 0x66c0, 0.92642629f32.to_bits());
            words(c, DATA, &[r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
        });
    }

    #[test]
    fn mid_left_chase_camera_algorithm_get_eye_drop_matches() {
        check(ssx3_ee::s02::f_00168d40__generated, mid_left_chase_camera_algorithm_get_eye_drop, 120, &|c, seed| {
            let mut r = Rng(0x168d_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x660c, r.edge_f32());
        });
    }
}
