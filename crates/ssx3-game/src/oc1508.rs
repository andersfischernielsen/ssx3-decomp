//! Staging module oc1508 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1508`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0024_5690,
        c_game_load_state_out_lodge_on_widget_created_00245690,
        "cGameLoadStateOutLodge_OnWidgetCreated",
    ),
    (
        0x0024_58b0,
        c_game_load_state_conquer_on_widget_created_002458b0,
        "cGameLoadStateConquer_OnWidgetCreated",
    ),
    (
        0x0024_5b50,
        c_fe_load_hint_state_on_widget_created_00245b50,
        "cFELoadHintState_OnWidgetCreated",
    ),
    (
        0x0024_5db8,
        c_fe_load_state_in_lodge_on_widget_created_00245db8,
        "cFELoadStateInLodge_OnWidgetCreated",
    ),
    (
        0x0024_d6d0,
        visual_effects_menu_debug_printf_stub,
        "VisualEffectsMenu_DebugPrintfStub",
    ),
    (
        0x0025_0aa0,
        ai_visual_effects_menu_debug_printf_stub,
        "AIVisualEffectsMenu_DebugPrintfStub",
    ),
    (
        0x0025_5668,
        vd_sink_base_flush_00255668,
        "VD_SinkBase_Flush",
    ),
    (0x0025_6250, dnas_auth_empty_hook, "DNASAuth_EmptyHook"),
    (0x0025_91b8, net_get_conn_manager, "Net_GetConnManager"),
    (0x0025_91d0, net_get_game_setup, "Net_GetGameSetup"),
    (
        0x0026_9ec0,
        path_init_ai_fields_00269ec0,
        "Path_InitAIFields",
    ),
    (
        0x0026_a068,
        path_init_track_fields_0026a068,
        "Path_InitTrackFields",
    ),
    (0x0026_ada0, path_data_empty_hook, "PathData_EmptyHook"),
    (0x0026_c410, path_init_fields_0026c410, "Path_InitFields"),
    (0x0026_c418, path_read_field_0026c418, "Path_ReadField"),
    (
        0x0027_2680,
        in_seq_curve_track_stop_00272680,
        "InSeq_CurveTrackStop",
    ),
    (
        0x0027_52f0,
        in_seq_curve_track_get_value_ptr_002752f0,
        "InSeq_CurveTrackGetValuePtr",
    ),
    (
        0x0027_52f8,
        in_seq_curve_track_v05_002752f8,
        "InSeq_CurveTrack_v05",
    ),
    (
        0x0027_5300,
        in_seq_curve_track_get_next_event_time_00275300,
        "InSeq_CurveTrackGetNextEventTime",
    ),
    (
        0x0027_5308,
        in_seq_curve_track_get_type_00275308,
        "InSeq_CurveTrackGetType",
    ),
    (
        0x0027_5310,
        in_seq_curve_track_get_channel_00275310,
        "InSeq_CurveTrackGetChannel",
    ),
    (
        0x0027_5318,
        in_seq_curve_track_is_started_00275318,
        "InSeq_CurveTrackIsStarted",
    ),
    (
        0x0027_5328,
        in_seq_span_track_get_value_ptr_00275328,
        "InSeq_SpanTrackGetValuePtr",
    ),
    (
        0x0027_5330,
        in_seq_span_track_v05_00275330,
        "InSeq_SpanTrack_v05",
    ),
    (
        0x0027_5338,
        in_seq_span_track_get_type_00275338,
        "InSeq_SpanTrackGetType",
    ),
    (
        0x0027_5340,
        in_seq_span_track_get_channel_00275340,
        "InSeq_SpanTrackGetChannel",
    ),
    (
        0x0027_5348,
        in_seq_span_track_is_started_00275348,
        "InSeq_SpanTrackIsStarted",
    ),
    (
        0x0027_5358,
        in_seq_event_track_get_value_ptr_00275358,
        "InSeq_EventTrackGetValuePtr",
    ),
    (
        0x0027_5360,
        in_seq_event_track_v05_00275360,
        "InSeq_EventTrack_v05",
    ),
    (
        0x0027_5368,
        in_seq_event_track_get_type_00275368,
        "InSeq_EventTrackGetType",
    ),
    (
        0x0027_5370,
        in_seq_event_track_get_channel_00275370,
        "InSeq_EventTrackGetChannel",
    ),
    (
        0x0027_5378,
        in_seq_event_track_is_started_00275378,
        "InSeq_EventTrackIsStarted",
    ),
    (
        0x0027_5388,
        in_seq_ramp_track_get_value_ptr_00275388,
        "InSeq_RampTrackGetValuePtr",
    ),
    (
        0x0027_5390,
        in_seq_ramp_track_v05_00275390,
        "InSeq_RampTrack_v05",
    ),
    (
        0x0027_5398,
        in_seq_ramp_track_get_type_00275398,
        "InSeq_RampTrackGetType",
    ),
    (
        0x0027_53a0,
        in_seq_ramp_track_get_channel_002753a0,
        "InSeq_RampTrackGetChannel",
    ),
    (
        0x0027_53a8,
        in_seq_ramp_track_is_started_002753a8,
        "InSeq_RampTrackIsStarted",
    ),
    (
        0x0028_1168,
        script_view_obj_get_binding_count_00281168,
        "ScriptViewObj_GetBindingCount",
    ),
    (
        0x0028_3be0,
        script_movie_debug_printf,
        "ScriptMovie_DebugPrintf",
    ),
    (
        0x0028_47a8,
        data_instance_man_on_data_added_002847a8,
        "DataInstanceMan_OnDataAdded",
    ),
    (
        0x0028_47b0,
        data_instance_man_on_data_removed_002847b0,
        "DataInstanceMan_OnDataRemoved",
    ),
    (
        0x0028_47b8,
        data_instance_man_v04_002847b8,
        "DataInstanceMan_v04",
    ),
    (
        0x0028_47c0,
        data_instance_man_v05_002847c0,
        "DataInstanceMan_v05",
    ),
    (
        0x0028_47e8,
        script_engine_seq_listener_on_advance_002847e8,
        "ScriptEngine_SeqListenerOnAdvance",
    ),
    (
        0x0028_47f0,
        script_engine_seq_listener_on_end_002847f0,
        "ScriptEngine_SeqListenerOnEnd",
    ),
];

/// `cGameLoadStateOutLodge_OnWidgetCreated` (0x0024_5690): straight-line leaf. Mechanical port of the generated body.
pub fn c_game_load_state_out_lodge_on_widget_created_00245690(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cGameLoadStateConquer_OnWidgetCreated` (0x0024_58b0): straight-line leaf. Mechanical port of the generated body.
pub fn c_game_load_state_conquer_on_widget_created_002458b0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFELoadHintState_OnWidgetCreated` (0x0024_5b50): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_load_hint_state_on_widget_created_00245b50(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFELoadStateInLodge_OnWidgetCreated` (0x0024_5db8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_load_state_in_lodge_on_widget_created_00245db8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `VisualEffectsMenu_DebugPrintfStub` (0x0024_d6d0): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn visual_effects_menu_debug_printf_stub(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `AIVisualEffectsMenu_DebugPrintfStub` (0x0025_0aa0): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn ai_visual_effects_menu_debug_printf_stub(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `VD_SinkBase_Flush` (0x0025_5668): straight-line leaf. Mechanical port of the generated body.
pub fn vd_sink_base_flush_00255668(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DNASAuth_EmptyHook` (0x0025_6250): straight-line leaf. Mechanical port of the generated body.
pub fn dnas_auth_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Net_GetConnManager` (0x0025_91b8): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn net_get_conn_manager(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[V0][0] = 0xb0000;
    c.g[V0][0] = c.g[V0][0] | 0x5ab0;
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    ret(c);
}

/// `Net_GetGameSetup` (0x0025_91d0): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn net_get_game_setup(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[V0][0] = 0xb0000;
    c.g[V0][0] = c.g[V0][0] | 0x5abc;
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    ret(c);
}

/// `Path_InitAIFields` (0x0026_9ec0): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn path_init_ai_fields_00269ec0(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x2e;
    c.g[V1][0] = 0x1;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x38), c.g[V0][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x3c), c.g[V1][0]);
    ret(c);
}

/// `Path_InitTrackFields` (0x0026_a068): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn path_init_track_fields_0026a068(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x38), 0u64);
    ret(c);
}

/// `PathData_EmptyHook` (0x0026_ada0): straight-line leaf. Mechanical port of the generated body.
pub fn path_data_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Path_InitFields` (0x0026_c410): straight-line leaf. Mechanical port of the generated body.
pub fn path_init_fields_0026c410(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Path_ReadField` (0x0026_c418): straight-line leaf. Mechanical port of the generated body.
pub fn path_read_field_0026c418(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `InSeq_CurveTrackStop` (0x0027_2680): straight-line leaf: 8 store(s). Mechanical port of the generated body.
pub fn in_seq_curve_track_stop_00272680(c: &mut Cpu) {
    c.cycles += 9;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x24), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x18), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x20), 0u64);
    ret(c);
}

/// `InSeq_CurveTrackGetValuePtr` (0x0027_52f0): straight-line leaf. Mechanical port of the generated body.
pub fn in_seq_curve_track_get_value_ptr_002752f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c));
    ret(c);
}

/// `InSeq_CurveTrack_v05` (0x0027_52f8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_curve_track_v05_002752f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `InSeq_CurveTrackGetNextEventTime` (0x0027_5300): straight-line leaf. Mechanical port of the generated body.
pub fn in_seq_curve_track_get_next_event_time_00275300(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xffffffffffffffff;
    ret(c);
}

/// `InSeq_CurveTrackGetType` (0x0027_5308): straight-line leaf. Mechanical port of the generated body.
pub fn in_seq_curve_track_get_type_00275308(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `InSeq_CurveTrackGetChannel` (0x0027_5310): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_curve_track_get_channel_00275310(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    ret(c);
}

/// `InSeq_CurveTrackIsStarted` (0x0027_5318): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_curve_track_is_started_00275318(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    ret(c);
}

/// `InSeq_SpanTrackGetValuePtr` (0x0027_5328): straight-line leaf. Mechanical port of the generated body.
pub fn in_seq_span_track_get_value_ptr_00275328(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `InSeq_SpanTrack_v05` (0x0027_5330): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_span_track_v05_00275330(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
    ret(c);
}

/// `InSeq_SpanTrackGetType` (0x0027_5338): straight-line leaf. Mechanical port of the generated body.
pub fn in_seq_span_track_get_type_00275338(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x2;
    ret(c);
}

/// `InSeq_SpanTrackGetChannel` (0x0027_5340): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_span_track_get_channel_00275340(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    ret(c);
}

/// `InSeq_SpanTrackIsStarted` (0x0027_5348): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_span_track_is_started_00275348(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    ret(c);
}

/// `InSeq_EventTrackGetValuePtr` (0x0027_5358): straight-line leaf. Mechanical port of the generated body.
pub fn in_seq_event_track_get_value_ptr_00275358(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `InSeq_EventTrack_v05` (0x0027_5360): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_event_track_v05_00275360(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
    ret(c);
}

/// `InSeq_EventTrackGetType` (0x0027_5368): straight-line leaf. Mechanical port of the generated body.
pub fn in_seq_event_track_get_type_00275368(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x3;
    ret(c);
}

/// `InSeq_EventTrackGetChannel` (0x0027_5370): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_event_track_get_channel_00275370(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    ret(c);
}

/// `InSeq_EventTrackIsStarted` (0x0027_5378): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_event_track_is_started_00275378(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    ret(c);
}

/// `InSeq_RampTrackGetValuePtr` (0x0027_5388): straight-line leaf. Mechanical port of the generated body.
pub fn in_seq_ramp_track_get_value_ptr_00275388(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24));
    ret(c);
}

/// `InSeq_RampTrack_v05` (0x0027_5390): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_ramp_track_v05_00275390(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `InSeq_RampTrackGetType` (0x0027_5398): straight-line leaf. Mechanical port of the generated body.
pub fn in_seq_ramp_track_get_type_00275398(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `InSeq_RampTrackGetChannel` (0x0027_53a0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_ramp_track_get_channel_002753a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    ret(c);
}

/// `InSeq_RampTrackIsStarted` (0x0027_53a8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn in_seq_ramp_track_is_started_002753a8(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    ret(c);
}

/// `ScriptViewObj_GetBindingCount` (0x0028_1168): straight-line leaf. Mechanical port of the generated body.
pub fn script_view_obj_get_binding_count_00281168(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xa;
    ret(c);
}

/// `ScriptMovie_DebugPrintf` (0x0028_3be0): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn script_movie_debug_printf(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `DataInstanceMan_OnDataAdded` (0x0028_47a8): straight-line leaf. Mechanical port of the generated body.
pub fn data_instance_man_on_data_added_002847a8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DataInstanceMan_OnDataRemoved` (0x0028_47b0): straight-line leaf. Mechanical port of the generated body.
pub fn data_instance_man_on_data_removed_002847b0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DataInstanceMan_v04` (0x0028_47b8): straight-line leaf. Mechanical port of the generated body.
pub fn data_instance_man_v04_002847b8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DataInstanceMan_v05` (0x0028_47c0): straight-line leaf. Mechanical port of the generated body.
pub fn data_instance_man_v05_002847c0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ScriptEngine_SeqListenerOnAdvance` (0x0028_47e8): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_seq_listener_on_advance_002847e8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ScriptEngine_SeqListenerOnEnd` (0x0028_47f0): straight-line leaf. Mechanical port of the generated body.
pub fn script_engine_seq_listener_on_end_002847f0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_lohi, Rng};

    /// An edge-case 64-bit register value (0, 1, -1, small, random).
    #[allow(dead_code)]
    fn leaf_val(r: &mut Rng) -> u64 {
        match r.next_u32() % 8 {
            0 => 0,
            1 => 1,
            2 => u64::MAX,
            3 => (r.next_u32() % 64) as u64,
            4 => sx(r.next_u32()),
            5 => r.next_u32() as u64,
            6 => 0x8000_0000,
            _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        }
    }

    /// Fill `size` bytes at `start` with zero / small / random words, or, when
    /// `chase` is set, with pointers (16-byte aligned) into `ptr_lo..ptr_lo + 0x200`.
    #[allow(dead_code)]
    fn leaf_fill(c: &mut Cpu, r: &mut Rng, start: u32, size: u32, chase: bool, ptr_lo: u32) {
        for i in 0..size / 4 {
            let w = if chase {
                ptr_lo + 16 * (r.next_u32() % 32)
            } else {
                match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.next_u32() % 16,
                    _ => r.next_u32(),
                }
            };
            c.bus.write32(start + 4 * i, w);
        }
    }

    /// `cGameLoadStateOutLodge_OnWidgetCreated` (0x0024_5690) against the generated body.
    #[test]
    fn c_game_load_state_out_lodge_on_widget_created_00245690_matches() {
        check(
            ee::s06::f_00245690__generated,
            c_game_load_state_out_lodge_on_widget_created_00245690,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x245690, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cGameLoadStateConquer_OnWidgetCreated` (0x0024_58b0) against the generated body.
    #[test]
    fn c_game_load_state_conquer_on_widget_created_002458b0_matches() {
        check(
            ee::s06::f_002458b0__generated,
            c_game_load_state_conquer_on_widget_created_002458b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2458b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFELoadHintState_OnWidgetCreated` (0x0024_5b50) against the generated body.
    #[test]
    fn c_fe_load_hint_state_on_widget_created_00245b50_matches() {
        check(
            ee::s06::f_00245b50__generated,
            c_fe_load_hint_state_on_widget_created_00245b50,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x245b50, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFELoadStateInLodge_OnWidgetCreated` (0x0024_5db8) against the generated body.
    #[test]
    fn c_fe_load_state_in_lodge_on_widget_created_00245db8_matches() {
        check(
            ee::s06::f_00245db8__generated,
            c_fe_load_state_in_lodge_on_widget_created_00245db8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x245db8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `VisualEffectsMenu_DebugPrintfStub` (0x0024_d6d0) against the generated body.
    #[test]
    fn visual_effects_menu_debug_printf_stub_matches() {
        check(
            ee::s06::f_0024d6d0__generated,
            visual_effects_menu_debug_printf_stub,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x24d6d0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[A1][0] = leaf_val(&mut r);
                c.g[A2][0] = leaf_val(&mut r);
                c.g[A3][0] = leaf_val(&mut r);
                c.g[T0][0] = leaf_val(&mut r);
                c.g[T1][0] = leaf_val(&mut r);
                c.g[T2][0] = leaf_val(&mut r);
                c.g[T3][0] = leaf_val(&mut r);
                c.f[12] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[16] = r.edge_f32();
                c.f[18] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AIVisualEffectsMenu_DebugPrintfStub` (0x0025_0aa0) against the generated body.
    #[test]
    fn ai_visual_effects_menu_debug_printf_stub_matches() {
        check(
            ee::s06::f_00250aa0__generated,
            ai_visual_effects_menu_debug_printf_stub,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x250aa0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[A1][0] = leaf_val(&mut r);
                c.g[A2][0] = leaf_val(&mut r);
                c.g[A3][0] = leaf_val(&mut r);
                c.g[T0][0] = leaf_val(&mut r);
                c.g[T1][0] = leaf_val(&mut r);
                c.g[T2][0] = leaf_val(&mut r);
                c.g[T3][0] = leaf_val(&mut r);
                c.f[12] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[16] = r.edge_f32();
                c.f[18] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `VD_SinkBase_Flush` (0x0025_5668) against the generated body.
    #[test]
    fn vd_sink_base_flush_00255668_matches() {
        check(
            ee::s06::f_00255668__generated,
            vd_sink_base_flush_00255668,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x255668, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DNASAuth_EmptyHook` (0x0025_6250) against the generated body.
    #[test]
    fn dnas_auth_empty_hook_matches() {
        check(
            ee::s06::f_00256250__generated,
            dnas_auth_empty_hook,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x256250, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Net_GetConnManager` (0x0025_91b8) against the generated body.
    #[test]
    fn net_get_conn_manager_matches() {
        check(
            ee::s06::f_002591b8__generated,
            net_get_conn_manager,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2591b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Net_GetGameSetup` (0x0025_91d0) against the generated body.
    #[test]
    fn net_get_game_setup_matches() {
        check(
            ee::s06::f_002591d0__generated,
            net_get_game_setup,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2591d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Path_InitAIFields` (0x0026_9ec0) against the generated body.
    #[test]
    fn path_init_ai_fields_00269ec0_matches() {
        check(
            ee::s07::f_00269ec0__generated,
            path_init_ai_fields_00269ec0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x269ec0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Path_InitTrackFields` (0x0026_a068) against the generated body.
    #[test]
    fn path_init_track_fields_0026a068_matches() {
        check(
            ee::s07::f_0026a068__generated,
            path_init_track_fields_0026a068,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26a068, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PathData_EmptyHook` (0x0026_ada0) against the generated body.
    #[test]
    fn path_data_empty_hook_matches() {
        check(
            ee::s07::f_0026ada0__generated,
            path_data_empty_hook,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x26ada0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Path_InitFields` (0x0026_c410) against the generated body.
    #[test]
    fn path_init_fields_0026c410_matches() {
        check(
            ee::s07::f_0026c410__generated,
            path_init_fields_0026c410,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x26c410, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Path_ReadField` (0x0026_c418) against the generated body.
    #[test]
    fn path_read_field_0026c418_matches() {
        check(
            ee::s07::f_0026c418__generated,
            path_read_field_0026c418,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x26c418, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_CurveTrackStop` (0x0027_2680) against the generated body.
    #[test]
    fn in_seq_curve_track_stop_00272680_matches() {
        check(
            ee::s07::f_00272680__generated,
            in_seq_curve_track_stop_00272680,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x272680, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_CurveTrackGetValuePtr` (0x0027_52f0) against the generated body.
    #[test]
    fn in_seq_curve_track_get_value_ptr_002752f0_matches() {
        check(
            ee::s07::f_002752f0__generated,
            in_seq_curve_track_get_value_ptr_002752f0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2752f0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_CurveTrack_v05` (0x0027_52f8) against the generated body.
    #[test]
    fn in_seq_curve_track_v05_002752f8_matches() {
        check(
            ee::s07::f_002752f8__generated,
            in_seq_curve_track_v05_002752f8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2752f8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_CurveTrackGetNextEventTime` (0x0027_5300) against the generated body.
    #[test]
    fn in_seq_curve_track_get_next_event_time_00275300_matches() {
        check(
            ee::s07::f_00275300__generated,
            in_seq_curve_track_get_next_event_time_00275300,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x275300, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_CurveTrackGetType` (0x0027_5308) against the generated body.
    #[test]
    fn in_seq_curve_track_get_type_00275308_matches() {
        check(
            ee::s07::f_00275308__generated,
            in_seq_curve_track_get_type_00275308,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x275308, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_CurveTrackGetChannel` (0x0027_5310) against the generated body.
    #[test]
    fn in_seq_curve_track_get_channel_00275310_matches() {
        check(
            ee::s07::f_00275310__generated,
            in_seq_curve_track_get_channel_00275310,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275310, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_CurveTrackIsStarted` (0x0027_5318) against the generated body.
    #[test]
    fn in_seq_curve_track_is_started_00275318_matches() {
        check(
            ee::s07::f_00275318__generated,
            in_seq_curve_track_is_started_00275318,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275318, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_SpanTrackGetValuePtr` (0x0027_5328) against the generated body.
    #[test]
    fn in_seq_span_track_get_value_ptr_00275328_matches() {
        check(
            ee::s07::f_00275328__generated,
            in_seq_span_track_get_value_ptr_00275328,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275328, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_SpanTrack_v05` (0x0027_5330) against the generated body.
    #[test]
    fn in_seq_span_track_v05_00275330_matches() {
        check(
            ee::s07::f_00275330__generated,
            in_seq_span_track_v05_00275330,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275330, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_SpanTrackGetType` (0x0027_5338) against the generated body.
    #[test]
    fn in_seq_span_track_get_type_00275338_matches() {
        check(
            ee::s07::f_00275338__generated,
            in_seq_span_track_get_type_00275338,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x275338, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_SpanTrackGetChannel` (0x0027_5340) against the generated body.
    #[test]
    fn in_seq_span_track_get_channel_00275340_matches() {
        check(
            ee::s07::f_00275340__generated,
            in_seq_span_track_get_channel_00275340,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275340, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_SpanTrackIsStarted` (0x0027_5348) against the generated body.
    #[test]
    fn in_seq_span_track_is_started_00275348_matches() {
        check(
            ee::s07::f_00275348__generated,
            in_seq_span_track_is_started_00275348,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275348, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_EventTrackGetValuePtr` (0x0027_5358) against the generated body.
    #[test]
    fn in_seq_event_track_get_value_ptr_00275358_matches() {
        check(
            ee::s07::f_00275358__generated,
            in_seq_event_track_get_value_ptr_00275358,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275358, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_EventTrack_v05` (0x0027_5360) against the generated body.
    #[test]
    fn in_seq_event_track_v05_00275360_matches() {
        check(
            ee::s07::f_00275360__generated,
            in_seq_event_track_v05_00275360,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275360, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_EventTrackGetType` (0x0027_5368) against the generated body.
    #[test]
    fn in_seq_event_track_get_type_00275368_matches() {
        check(
            ee::s07::f_00275368__generated,
            in_seq_event_track_get_type_00275368,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x275368, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_EventTrackGetChannel` (0x0027_5370) against the generated body.
    #[test]
    fn in_seq_event_track_get_channel_00275370_matches() {
        check(
            ee::s07::f_00275370__generated,
            in_seq_event_track_get_channel_00275370,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275370, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_EventTrackIsStarted` (0x0027_5378) against the generated body.
    #[test]
    fn in_seq_event_track_is_started_00275378_matches() {
        check(
            ee::s07::f_00275378__generated,
            in_seq_event_track_is_started_00275378,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275378, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_RampTrackGetValuePtr` (0x0027_5388) against the generated body.
    #[test]
    fn in_seq_ramp_track_get_value_ptr_00275388_matches() {
        check(
            ee::s07::f_00275388__generated,
            in_seq_ramp_track_get_value_ptr_00275388,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275388, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_RampTrack_v05` (0x0027_5390) against the generated body.
    #[test]
    fn in_seq_ramp_track_v05_00275390_matches() {
        check(
            ee::s07::f_00275390__generated,
            in_seq_ramp_track_v05_00275390,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x275390, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_RampTrackGetType` (0x0027_5398) against the generated body.
    #[test]
    fn in_seq_ramp_track_get_type_00275398_matches() {
        check(
            ee::s07::f_00275398__generated,
            in_seq_ramp_track_get_type_00275398,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x275398, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_RampTrackGetChannel` (0x0027_53a0) against the generated body.
    #[test]
    fn in_seq_ramp_track_get_channel_002753a0_matches() {
        check(
            ee::s07::f_002753a0__generated,
            in_seq_ramp_track_get_channel_002753a0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2753a0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `InSeq_RampTrackIsStarted` (0x0027_53a8) against the generated body.
    #[test]
    fn in_seq_ramp_track_is_started_002753a8_matches() {
        check(
            ee::s07::f_002753a8__generated,
            in_seq_ramp_track_is_started_002753a8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2753a8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptViewObj_GetBindingCount` (0x0028_1168) against the generated body.
    #[test]
    fn script_view_obj_get_binding_count_00281168_matches() {
        check(
            ee::s07::f_00281168__generated,
            script_view_obj_get_binding_count_00281168,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x281168, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptMovie_DebugPrintf` (0x0028_3be0) against the generated body.
    #[test]
    fn script_movie_debug_printf_matches() {
        check(
            ee::s07::f_00283be0__generated,
            script_movie_debug_printf,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x283be0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[A1][0] = leaf_val(&mut r);
                c.g[A2][0] = leaf_val(&mut r);
                c.g[A3][0] = leaf_val(&mut r);
                c.g[T0][0] = leaf_val(&mut r);
                c.g[T1][0] = leaf_val(&mut r);
                c.g[T2][0] = leaf_val(&mut r);
                c.g[T3][0] = leaf_val(&mut r);
                c.f[12] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[16] = r.edge_f32();
                c.f[18] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DataInstanceMan_OnDataAdded` (0x0028_47a8) against the generated body.
    #[test]
    fn data_instance_man_on_data_added_002847a8_matches() {
        check(
            ee::s07::f_002847a8__generated,
            data_instance_man_on_data_added_002847a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2847a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DataInstanceMan_OnDataRemoved` (0x0028_47b0) against the generated body.
    #[test]
    fn data_instance_man_on_data_removed_002847b0_matches() {
        check(
            ee::s07::f_002847b0__generated,
            data_instance_man_on_data_removed_002847b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2847b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DataInstanceMan_v04` (0x0028_47b8) against the generated body.
    #[test]
    fn data_instance_man_v04_002847b8_matches() {
        check(
            ee::s07::f_002847b8__generated,
            data_instance_man_v04_002847b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2847b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DataInstanceMan_v05` (0x0028_47c0) against the generated body.
    #[test]
    fn data_instance_man_v05_002847c0_matches() {
        check(
            ee::s07::f_002847c0__generated,
            data_instance_man_v05_002847c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2847c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptEngine_SeqListenerOnAdvance` (0x0028_47e8) against the generated body.
    #[test]
    fn script_engine_seq_listener_on_advance_002847e8_matches() {
        check(
            ee::s07::f_002847e8__generated,
            script_engine_seq_listener_on_advance_002847e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2847e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ScriptEngine_SeqListenerOnEnd` (0x0028_47f0) against the generated body.
    #[test]
    fn script_engine_seq_listener_on_end_002847f0_matches() {
        check(
            ee::s07::f_002847f0__generated,
            script_engine_seq_listener_on_end_002847f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2847f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
