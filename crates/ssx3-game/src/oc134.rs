//! Phase 2 wave 47 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for 50 small cold-path functions (tag serialisation, UI/FE
//! widget plumbing, camera/anim/lobby helpers, memory destructors), each with
//! a differential test against the generated body.

#![allow(unused_parens, dead_code)]

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_c300, tag_field_set_binary, "TagFieldSetBinary"),
    (0x0016_90d0, script_cam_controller_dtor, "ScriptCamController_dtor"),
    (0x0039_8078, ui_engine_texture_set_dtor, "UIEngine_TextureSet_dtor"),
    (0x0015_ae00, rewards_file_unload, "RewardsFile_Unload"),
    (0x0039_b6a0, ui_menu_render, "UIMenu_Render"),(0x001d_3e08, fe_state_peak_room_on_widget_notify, "cFEStatePeakRoom_OnWidgetNotify"),
    (0x0016_d260, camera_trigger_man_pop_trigger_stack, "CameraTriggerMan_PopTriggerStack"),(0x003a_0528, ui_text_refresh_text, "UIText_RefreshText"),(0x001a_9578, net_fe_state_on_session_lost_popup_event, "NetFEState_OnSessionLostPopupEvent"),
    (0x001a_ca88, fe_use_existing_account_on_abort_popup_event, "cFEUseExistingAccount_OnAbortPopupEvent"),
    (0x001b_f378, fe_state_login_v33, "cFEStateLogin_v33"),
    (0x001c_4f58, fe_state_stats_format_time, "cFEStateStats_FormatTime"),
    (0x001e_2dc0, message_center_get_index_in_type, "MessageCenter_GetIndexInType"),
    (0x001e_8f00, ov_screen_multiplayer_standings_init, "OVScreenMultiplayerStandings_Init"),
    (0x0025_dac0, lobby_man_create_room, "LobbyMan_CreateRoom"),(0x0026_5398, setup_man_host_recv_cancel, "cSetupMan_HostRecvCancel"),
    (0x0027_3208, in_seq_event_track_start, "InSeq_EventTrackStart"),
    (0x0016_1fa0, chase_cam_controller_begin_script_camera, "ChaseCamController_BeginScriptCamera"),
    (0x0023_ea90, mc_manager_on_operation_done, "MCManager_OnOperationDone"),
    (0x0023_ff00, mc_manager_state22_load, "MCManager_State22_Load"),
    (0x001c_84a8, fe_popup_confirm_measure_line, "cFEPopupConfirm_MeasureLine"),
    (0x001c_8568, fe_popup_confirm_measure_line_w, "cFEPopupConfirm_MeasureLineW"),(0x0019_cf40, bolt_file_cache_shutdown, "BoltFile_CacheShutdown"),(0x0028_f200, ssx_audio_system_on_replay_begin, "SSXAudioSystem_OnReplayBegin"),
    (0x0030_7738, w_script_process_dtor, "WScriptProcess_dtor"),(0x0014_c3c8, bolts_index_table3_by_char, "Bolts_IndexTable3ByChar"),
    (0x0014_c488, bolts_index_table4_by_char, "Bolts_IndexTable4ByChar"),
    (0x0039_de68, ui_scroll_set_parts_visible, "cUIScroll_SetPartsVisible"),
    (0x0034_3010, conveyor_ctor, "Conveyor_ctor"),
    (0x0034_3a68, crowd_man3d_ctor, "CrowdMan3d_ctor"),
    (0x001d_4ea8, fe_state_rewards_room_on_widget_action, "cFEStateRewardsRoom_OnWidgetAction"),
    (0x0026_eb48, replay_stream_read_object_ref, "ReplayStream_ReadObjectRef"),
    (0x003a_1030, ui_text_get_text_size, "UIText_GetTextSize"),
    (0x001f_4910, fe_state_buy_attrib_on_widget_created, "cFEStateBuyAttrib_OnWidgetCreated"),
    (0x002d_9a80, avalanche_free_histories, "Avalanche_FreeHistories"),
    (0x002d_9b40, avalanche_destroy_all, "Avalanche_DestroyAll"),
    (0x0039_f400, ui_state_mgr_switch_state, "UIStateMgr_SwitchState"),
    (0x002c_6f78, console_clock_format_time, "ConsoleClock_FormatTime"),
    (0x0015_db58, game_camera_reset_all, "GameCamera_ResetAll"),
];

// ---------------------------------------------------------------------------
// `ScriptCamController_dtor` (0x001690d0).
//
// `void ScriptCamController::~ScriptCamController()`: install the class
// vtable, release and `delete` the two script elements, free the element
// array, then run the base destructor.

pub fn script_cam_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {

        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[V0][0] = 0x460000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3bd8));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        'l_169108: loop {
        'b_169140: {
        c.cycles += 9;
        c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[RA][0] = sx(0x00169118);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
        call_ptr(c, 0x002832d8, 0x00169118)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[S0][0] = c.lw((c.g[S0][0] as u32));
        let t = c.g[S0][0] == 0u64;
        c.g[A1][0] = 0x3;
        if t {
        break 'b_169140;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x8, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00169140);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00169140)?;
        break 'b_169140;
        }
        }
        c.cycles += 4;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
        if c.g[V0][0] != 0u64 {
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x00169108); }
        continue 'l_169108;
        } else {
        'b_169168: {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        if c.g[A0][0] == 0u64 {
        c.g[A0][0] = c.g[S1][0];
        break 'b_169168;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x00169164);
        call_ptr(c, 0x00317e98, 0x00169164)?;
        c.g[A0][0] = c.g[S1][0];
        break 'b_169168;
        }
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x00169170);
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0015cc10, 0x00169170)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t; return Ok(());
        }
        }

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIEngine_TextureSet_dtor` (0x00398078).
//
// `void UIEngine::TextureSet::~TextureSet(void* self, int flags)`: release
// every texture through the engine's virtual `textureRelease`, free the two
// arrays, and (flags & 1) delete the set itself.

pub fn ui_engine_texture_set_dtor(c: &mut Cpu) {
    guest(c, |c| {

        'b_3980e0: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = 0u64;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S2][0] = c.g[A1][0];
        if t {
        break 'b_3980e0;
        } else {
        c.cycles += 2;
        c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        'l_3980a8: loop {
        c.cycles += 14;
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[A2][0] = vslot(c, c.g[V0][0], 0x190, A2);
        c.g[A1][0] = c.lw((c.g[V1][0] as u32));
        let t = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x003980d0);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x003980d0)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x003980a8); }
        continue 'l_3980a8;
        } else {
        break 'b_3980e0;
        }
        }
        }
        }
        'b_3980f8: {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        if c.g[A0][0] == 0u64 {
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        break 'b_3980f8;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x003980f4);
        call_ptr(c, 0x00317e98, 0x003980f4)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        break 'b_3980f8;
        }
        }
        'b_39810c: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        if t {
        break 'b_39810c;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x00398108);
        call_ptr(c, 0x00317e98, 0x00398108)?;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        break 'b_39810c;
        }
        }
        'b_398120: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_398120;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x0039811c);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00317e50, 0x0039811c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_398120;
        }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `RewardsFile_Unload` (0x0015ae00).
//
// `void RewardsFile::Unload(RewardsFile* self)`: drop the open sound handle
// from the BE sound object, free the loaded data and clear the file record.

pub fn rewards_file_unload(c: &mut Cpu) {
    guest(c, |c| {

        'b_15ae44: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A2][0] = c.lw((c.g[S1][0] as u32));
        let t = (c.g[A2][0] as i64) < 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if t {
        break 'b_15ae44;
        } else {
        c.cycles += 9;
        c.g[A1][0] = 0u64;
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a70));
        c.g[RA][0] = sx(0x0015ae34);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0019dc20, 0x0015ae34)?;
        c.g[A2][0] = c.lw((c.g[S1][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0015ae44);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0019d4b8, 0x0015ae44)?;
        break 'b_15ae44;
        }
        }
        'b_15ae5c: {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0xffffffffffffffff;
        if t {
        break 'b_15ae5c;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x0015ae58);
        call_ptr(c, 0x00317e98, 0x0015ae58)?;
        c.g[V0][0] = 0xffffffffffffffff;
        break 'b_15ae5c;
        }
        }
        c.cycles += 24;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), 0u64);
        c.sw((c.g[S1][0] as u32), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x24), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x2c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x30), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x34), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x38), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x3c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x44), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIMenu_Render` (0x0039b6a0).
//
// `void UIMenu::Render(UIMenu* self, float x, float y)`: refresh the scroll
// range, then walk the item list, offsetting each item by the menu position
// and issuing its render call.

pub fn ui_menu_render(c: &mut Cpu) {
    guest(c, |c| {

        'b_39b6dc: {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.g[S1][0] = c.g[A0][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.mov_s(21, 12);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
        let t = c.g[A0][0] == 0u64;
        c.mov_s(20, 13);
        if t {
        break 'b_39b6dc;
        } else {
        c.cycles += 4;
        c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x98));
        c.g[A2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x97));
        c.g[RA][0] = sx(0x0039b6dc);
        c.g[A3][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
        call_ptr(c, 0x0039da20, 0x0039b6dc)?;
        break 'b_39b6dc;
        }
        }
        'b_39b744: {
        c.cycles += 5;
        c.g[RA][0] = sx(0x0039b6e4);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
        call_ptr(c, 0x00397840, 0x0039b6e4)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0u64 {
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_39b744;
        } else {
        c.cycles += 2;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        'l_39b6f8: loop {
        'b_39b730: {
        c.cycles += 4;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        if c.g[V0][0] == 0u64 {
        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        break 'b_39b730;
        } else {
        c.cycles += 10;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x44));
        c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x48));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
        c.add_s(12, 12, 21);
        c.add_s(13, 13, 20);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0039b72c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039b72c)?;
        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        break 'b_39b730;
        }
        }
        c.cycles += 4;
        c.g[RA][0] = sx(0x0039b738);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00397718, 0x0039b738)?;
        if c.g[V0][0] == 0u64 {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x0039b6f8); }
        continue 'l_39b6f8;
        } else {
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_39b744;
        }
        }
        }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// The two-instruction virtual-method read the originals use:
/// `a0 = lh(vt+off)`, `func = lw(vt+off+4)`. The original leaves `func` in
/// `vreg` (v0, v1, a2 ... depending on the site), which later code and the
/// fakes can see, so the register is written here too. Returns `func`.
#[inline(always)]
fn vslot(c: &mut Cpu, vt: u64, off: u32, vreg: usize) -> u64 {
    let base = (vt as u32).wrapping_add(off);
    c.g[A0][0] = Addr(base).lh(c);
    let func = Addr(base.wrapping_add(4)).lw(c);
    c.g[vreg][0] = func;
    func
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// `lw` (sign-extended) of `a + off`.
#[inline(always)]
fn lw(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lw(c)
}

/// `sw v` to `a + off`.
#[inline(always)]
fn sw(c: &mut Cpu, a: u32, off: u32, v: u64) {
    Addr(p(a, off)).sw(c, v);
}

/// `lbu` of `a + off`.
#[inline(always)]
fn lbu(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lbu(c)
}

/// `lh` of `a + off`.
#[inline(always)]
fn lh(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lh(c)
}

/// `lb` of `a + off`.
#[inline(always)]
fn lb(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lb(c)
}

/// `lwc1 fN, a+off`.
#[inline(always)]
fn ldf(c: &mut Cpu, n: usize, a: u32, off: u32) -> FpuF32 {
    let v = Addr(p(a, off)).read_fpu(c);
    c.f[n] = v.0;
    v
}

/// FPU register `n`.
#[inline(always)]
fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

// ---------------------------------------------------------------------------
// Callees (direct calls). Indirect calls go through [`call_ptr`].

mod callee {
    use super::{ee, Callee};

    pub const TAG_FIELD_COPY_TAG: Callee = Callee { addr: 0x003e_b198, f: ee::s14::f_003eb198 };
    pub const TAG_FIELD_SET_RAW: Callee = Callee { addr: 0x003e_b1e0, f: ee::s14::f_003eb1e0 };
}

// ---------------------------------------------------------------------------
// `TagFieldSetBinary` (0x003ec300).
//
// `int TagFieldSetBinary(Record* r, int len, int, const u8* src, int size)`:
// append the field name, '$', then the two-character hex encoding of every
// source byte, NUL-terminate, and store the raw text in the record.

const HEX_HIGH: u32 = 0x0049_5fc0; // D_00495FC0: high nibble -> character
const HEX_LOW: u32 = 0x0049_60c0; // D_004960C0: low nibble -> character

pub fn tag_field_set_binary(c: &mut Cpu) {
    guest(c, |c| {

        'b_3ec38c: {
        c.cycles += 17;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1150));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1120));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1110));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1140));
        c.g[S3][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1130));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x1100), c.g[RA][0]);
        c.g[S1][0] = c.g[T0][0];
        c.g[S0][0] = c.g[A3][0];
        c.g[RA][0] = sx(0x003ec330);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003eb198, 0x003ec330)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x24;
        c.sb((c.g[A0][0] as u32), c.g[V0][0]);
        let t = (c.g[S1][0] as i64) <= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        if t {
        break 'b_3ec38c;
        } else {
        c.cycles += 5;
        c.g[V0][0] = 0x490000;
        c.g[V1][0] = 0x490000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5fc0));
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60c0));
        'l_3ec358: loop {
        c.cycles += 13;
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
        c.sb((c.g[A0][0] as u32), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
        c.sb((c.g[A0][0] as u32), c.g[V1][0]);
        let t = (c.g[S1][0] as i64) > 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x003ec358); }
        continue 'l_3ec358;
        } else {
        break 'b_3ec38c;
        }
        }
        }
        }
        c.cycles += 12;
        c.sb((c.g[A0][0] as u32), 0u64);
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x003ec3a0);
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x003eb1e0, 0x003ec3a0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1140));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1130));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1120));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1150));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStatePeakRoom_OnWidgetNotify` (0x001d3e08).
//
// `int cFEStatePeakRoom::OnWidgetNotify(cFEState* self, UIWidget* item,
//  int key, int id)`: keys 8 and 9 report "handled with 0x100"; key 6 routes
// the selection into the lodge screen and reports 0x10/0x101 by whether the
// comparison succeeded; everything else reports 0x101.

pub fn fe_state_peak_room_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {

        'b_1d3ea8: {
        'b_1d3ea4: {
        'b_1d3e5c: {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[V0][0] = 0x6;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A3][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.g[A1][0];
        let t = c.g[A2][0] == c.g[V0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        if t {
        break 'b_1d3e5c;
        } else {
        c.cycles += 3;
        c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0x101;
        if t {
        break 'b_1d3ea8;
        } else {
        c.cycles += 3;
        c.g[V0][0] = (c.g[A2][0] < 0xau64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = (c.g[A2][0] < 0x8u64) as u64;
        if t {
        break 'b_1d3ea4;
        } else {
        c.cycles += 2;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0x101;
        if t {
        break 'b_1d3ea8;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = 0x100;
        if t {
        break 'b_1d3ea8;
        } else {
        break 'b_1d3e5c;
        }
        }
        }
        }
        }
        }
        c.cycles += 18;
        c.g[RA][0] = sx(0x001d3e64);
        call_ptr(c, 0x0014dc80, 0x001d3e64)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d3e70);
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014dd58, 0x001d3e70)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x74));
        c.g[RA][0] = sx(0x001d3e80);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x00397870, 0x001d3e80)?;
        c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
        c.g[A0][0] = c.g[S0][0];
        c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
        c.g[RA][0] = sx(0x001d3e98);
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xdc));
        call_ptr(c, 0x00157bf0, 0x001d3e98)?;
        c.g[V1][0] = c.g[V0][0];
        let t = c.g[V1][0] == 0u64;
        c.g[V0][0] = 0x10;
        if t {
        break 'b_1d3ea8;
        } else {
        break 'b_1d3ea4;
        }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x101;
        break 'b_1d3ea8;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `CameraTriggerMan_PopTriggerStack` (0x0016d260).
//
// `int CameraTriggerMan::PopTriggerStack(TriggerStack* s, int key, int* changed,
//  int* out)`: remove `key` from the stack; report whether the top changed.

pub fn camera_trigger_man_pop_trigger_stack(c: &mut Cpu) {
    c.cycles += 3;
    let stack = c.g[A0][0] as u32;
    let key = c.g[A1][0];
    let changed = c.g[A2][0] as u32;
    let out = c.g[A3][0] as u32;
    let mut t1 = lw(c, stack, 0);
    c.g[T1][0] = t1;
    c.g[T0][0] = 0;
    if t1 != 0 {
        c.cycles += 3;
        let mut i: u32 = 0;
        let mut v1 = stack.wrapping_add(8);
        let t2 = v1;
        c.g[V1][0] = sx(v1);
        c.g[T2][0] = sx(t2);
        loop {
            c.cycles += 3;
            let v0 = lw(c, v1, 0);
            if v0 != key {
                t1 = lw(c, stack, 0);
                c.g[T1][0] = t1;
            } else {
                c.cycles += 4;
                let mut j = i.wrapping_add(1);
                c.g[A1][0] = sx(j);
                let in_range = (j as u64) < t1;
                c.g[V0][0] = in_range as u64;
                c.g[V0][0] = sx(j << 2);
                if in_range {
                    c.cycles += 1;
                    v1 = (j << 2).wrapping_add(t2);
                    c.g[V1][0] = sx(v1);
                    loop {
                        c.cycles += 7;
                        let v = lw(c, v1, 0);
                        j = j.wrapping_add(1);
                        c.g[A1][0] = sx(j);
                        sw(c, v1.wrapping_sub(4), 0, v);
                        c.g[V0][0] = lw(c, stack, 0);
                        c.g[V0][0] = ((j as u64) < c.g[V0][0]) as u64;
                        let more = c.g[V0][0] != 0;
                        v1 = v1.wrapping_add(4);
                        c.g[V1][0] = sx(v1);
                        if !more {
                            break;
                        }
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0016_d298);
                        }
                    }
                }
                c.g[A1][0] = sx(i.wrapping_add(1));
                c.cycles += 4;
                c.g[V0][0] = lw(c, stack, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                sw(c, stack, 0, c.g[V0][0]);
                break;
            }
            c.cycles += 4;
            c.g[T0][0] = sx(i.wrapping_add(1));
            i = i.wrapping_add(1);
            c.g[V0][0] = ((i as u64) < c.g[T1][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(v1.wrapping_add(4));
            v1 = v1.wrapping_add(4);
            if !more {
                break;
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0016_d278);
            }
        }
    }
    c.cycles += 3;
    let count = lw(c, stack, 0);
    let v0 = sx((count as u32).wrapping_sub(1));
    if count == 0 {
        c.cycles += 3;
        sw(c, changed, 0, 0);
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    let top = lw(c, stack, 4);
    c.cycles += 6;
    let slot = (v0 as u32) << 2;
    let addr = (stack as u64).wrapping_add(slot as u64);
    let item = lw(c, addr as u32, 8);
    c.g[A0][0] = item;
    c.g[V1][0] = top;
    c.g[V0][0] = 1;
    if item != top {
        c.cycles += 2;
        sw(c, changed, 0, 1);
        sw(c, out, 0, item);
    } else {
        c.cycles += 2;
        sw(c, changed, 0, 0);
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// `ScriptMovie_Open` (0x002838e8).
//
// `int ScriptMovie_Open(ScriptMovie* self, int file)`: if a movie is already
// open, report 0; otherwise allocate a video object, set up its parameters and
// start playback (0 on failure, 1 on success).

pub fn script_movie_open(c: &mut Cpu) {
    guest(c, |c| {

        'b_283990: {
        'b_283910: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        let t = c.g[V0][0] == 0u64;
        c.g[S1][0] = c.g[A1][0];
        if t {
        break 'b_283910;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = 0u64;
        if t {
        break 'b_283990;
        } else {
        break 'b_283910;
        }
        }
        }
        c.cycles += 25;
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = 0x100;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1fc8));
        c.g[A3][0] = 0u64;
        c.g[RA][0] = sx(0x00283928);
        c.g[A0][0] = 0x44;
        call_ptr(c, 0x00317d70, 0x00283928)?;
        c.g[RA][0] = sx(0x00283930);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002533c8, 0x00283930)?;
        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028393c);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x00253390, 0x0028393c)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
        c.g[V0][0] = 0x1;
        c.g[V1][0] = 0x100;
        c.g[A2][0] = 0x9;
        c.sw((c.g[SP][0] as u32), c.g[S1][0]);
        c.g[A1][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
        c.g[RA][0] = sx(0x0028396c);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        call_ptr(c, 0x002534a8, 0x0028396c)?;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0x1;
        if t {
        break 'b_283990;
        } else {
        'b_28398c: {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
        if c.g[A0][0] == 0u64 {
        c.sw((c.g[S0][0] as u32), 0u64);
        break 'b_28398c;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x00283988);
        c.g[A1][0] = 0x3;
        call_ptr(c, 0x00253418, 0x00283988)?;
        c.sw((c.g[S0][0] as u32), 0u64);
        break 'b_28398c;
        }
        }
        c.cycles += 1;
        c.g[V0][0] = 0u64;
        break 'b_283990;
        }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `AnimController_PlayAnimDirect` (0x003129e0).
//
// `void AnimController::PlayAnimDirect(AnimController* self, AnimTable* table,
//  int anim, AnimSeqList* seq, float speed, int flags)`: resolve the table
// entry, look up its bank, play the animation and record the sequence.

pub fn anim_controller_play_anim_direct(c: &mut Cpu) {
    guest(c, |c| {

        'b_312a38: {
        c.cycles += 19;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A0][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = c.g[A2][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.mov_s(20, 12);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x00312a18);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003115c8, 0x00312a18)?;
        c.mtc1(0, 0);
        c.c_lt_s(20, 0);
        let t = !c.fcc();
        c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        if t {
        break 'b_312a38;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x00312a34);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003115c8, 0x00312a34)?;
        c.lwc1(20, (c.g[V0][0] as u32).wrapping_add(0x10));
        break 'b_312a38;
        }
        }
        c.cycles += 25;
        c.g[V0][0] = 0x490000;
        c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x27f8));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A1][0] = sx((c.g[S3][0] as u32) << 3);
        c.g[T1][0] = c.lw((c.g[V0][0] as u32));
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S4][0];
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[RA][0] = sx(0x00312a70);
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x00311f00, 0x00312a70)?;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIText_RefreshText` (0x003a0528).
//
// `void UIText::RefreshText(UIText* self)`: resolve the current string and
// either re-wrap it or pass it to the wide-string setter.

pub fn ui_text_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[S0][0] = sx(this);
        c.g[A1][0] = lw(c, this, 0xb0);
        let string = c.g[A1][0];
        // The two branches that return early (no text or no owner) share this
        // body; the rest of the function only runs for a live string.
        let mut done = false;
        if string == 0 {
            done = true;
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw(c, this, 0x5c);
            c.g[V1][0] = lw(c, c.g[V0][0] as u32, 0xd0);
            c.g[A0][0] = lw(c, c.g[V1][0] as u32, 0x10);
            c.g[A2][0] = lw(c, c.g[A0][0] as u32, 0x10);
            if c.g[A2][0] == 0 {
                done = true;
            } else {
                c.cycles += 8;
                c.g[V1][0] = lw(c, c.g[A2][0] as u32, 4);
                c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x20);
                c.g[V0][0] = lw(c, c.g[V1][0] as u32, 0x24);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x003a_0570)?;
                c.g[A1][0] = c.g[V0][0];
                if c.g[A1][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, this, 8);
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = lw(c, this, 0x14);
                    c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 7) as u32);
                    c.g[V0][0] = c.g[V0][0] & 1;
                    let wrapped = c.g[V0][0] != 0;
                    let wide = if wrapped {
                        true
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = lw(c, this, 0x74);
                        c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 6) as u32);
                        c.g[V0][0] = c.g[V0][0] & 1;
                        c.g[V0][0] != 0
                    };
                    if wide {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x003a_0d00, 0x003a_05ac)?;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x003a_0eb0, 0x003a_05bc)?;
                    }
                    c.g[V0][0] = lw(c, this, 8);
                }
                // Common tail: notify the owner through its virtual slot.
                c.cycles += 6;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = lh(c, c.g[V0][0] as u32, 0x28);
                c.g[V1][0] = lw(c, c.g[V0][0] as u32, 0x2c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x003a_05d4)?;
                done = true;
            }
        }
        let _ = done;
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `FEModule_LoadAnimData` (0x001a2b00).
//
// `void cRealFE::loadCharAnimations()`: load the character-animation table
// file and install its (single) entry.

pub fn fe_module_load_anim_data(c: &mut Cpu) {
    guest(c, |c| {

        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[A0][0] = 0x460000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1330));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A2][0] = 0x64;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[A3][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x001a2b30);
        c.g[S0][0] = 0u64;
        call_ptr(c, 0x003ded50, 0x001a2b30)?;
        c.g[S2][0] = 0x460000;
        c.g[S1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1730));
        c.g[A2][0] = 0u64;
        'l_1a2b40: loop {
        c.cycles += 20;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x18;
        c.g[RA][0] = sx(0x001a2b50);
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1348));
        call_ptr(c, 0x00317d70, 0x001a2b50)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001a2b60);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        call_ptr(c, 0x00311250, 0x001a2b60)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd08));
        c.g[A0][0] = c.g[S0][0] & 0xff;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd8c));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[RA][0] = sx(0x001a2b88);
        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
        call_ptr(c, 0x00314f30, 0x001a2b88)?;
        let t = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = 0u64;
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x001a2b40); }
        continue 'l_1a2b40;
        } else {
        c.cycles += 11;
        c.g[RA][0] = sx(0x001a2b98);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd8c));
        call_ptr(c, 0x003150b0, 0x001a2b98)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32));
        c.g[RA][0] = sx(0x001a2ba4);
        c.g[A1][0] = 0x64;
        call_ptr(c, 0x003dedc0, 0x001a2ba4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t; return Ok(());
        }
        }

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `NetFEState_OnSessionLostPopupEvent` (0x001a9578).
//
// `int NetFEState::OnSessionLostPopupEvent(NetFEState* self, int, int key)`:
// keys 0xf/0x14 close the messenger and move on to the chat/online menu,
// everything else forwards to the default child handler.

pub fn net_fe_state_on_session_lost_popup_event(c: &mut Cpu) {
    guest(c, |c| {

        'b_1a962c: {
        'b_1a9620: {
        'b_1a959c: {
        c.cycles += 6;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[V0][0] = 0xf;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        let t = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if t {
        break 'b_1a959c;
        } else {
        c.cycles += 3;
        c.g[V0][0] = 0x14;
        let t = c.g[A2][0] != c.g[V0][0];

        if t {
        break 'b_1a9620;
        } else {
        break 'b_1a959c;
        }
        }
        }
        'b_1a9610: {
        'b_1a95ec: {
        'b_1a95e8: {
        c.cycles += 6;
        c.g[RA][0] = sx(0x001a95a4);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001a93d0, 0x001a95a4)?;
        c.g[RA][0] = sx(0x001a95ac);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025da90, 0x001a95ac)?;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1d98));
        if t {
        break 'b_1a95e8;
        } else {
        c.cycles += 2;
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = 0x460000;
        if t {
        break 'b_1a95ec;
        } else {
        c.cycles += 11;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = 0x73c;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf0));
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x001a95d4);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x001a95d4)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x001a95e0);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001b1f88, 0x001a95e0)?;
        let t = true;
        c.g[A0][0] = c.g[S0][0];
        if t {
        break 'b_1a9610;
        } else {
        break 'b_1a95e8;
        }
        }
        }
        }
        c.cycles += 1;
        c.g[A1][0] = 0x460000;
        break 'b_1a95ec;
        }
        c.cycles += 9;
        c.g[A0][0] = 0x6d0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x001a9600);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x001a9600)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x001a960c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001bfbb8, 0x001a960c)?;
        c.g[A0][0] = c.g[S0][0];
        break 'b_1a9610;
        }
        c.cycles += 4;
        c.g[RA][0] = sx(0x001a9618);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x001a8a40, 0x001a9618)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        if t {
        break 'b_1a962c;
        } else {
        break 'b_1a9620;
        }
        }
        c.cycles += 3;
        c.g[RA][0] = sx(0x001a9628);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001a93d0, 0x001a9628)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        break 'b_1a962c;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEUseExistingAccount_OnAbortPopupEvent` (0x001aca88).
//
// `int cFEUseExistingAccount::OnAbortPopupEvent(self, widget, key)`: 0xf/0x14
// close the network messenger popups, everything else goes to the default
// child event handler.

pub fn fe_use_existing_account_on_abort_popup_event(c: &mut Cpu) {
    guest(c, |c| {

        'b_1acb34: {
        'b_1acb24: {
        'b_1acabc: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0xf;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A2][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if t {
        break 'b_1acabc;
        } else {
        c.cycles += 3;
        c.g[V0][0] = 0x14;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if t {
        break 'b_1acb24;
        } else {
        break 'b_1acabc;
        }
        }
        }
        'b_1acad0: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_1acad0;
        } else {
        c.cycles += 2;
        c.g[RA][0] = sx(0x001acad0);
        call_ptr(c, 0x00260f80, 0x001acad0)?;
        break 'b_1acad0;
        }
        }
        'b_1acaec: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x001acad8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x00261460, 0x001acad8)?;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if t {
        break 'b_1acaec;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x001acae8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x002613c8, 0x001acae8)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        break 'b_1acaec;
        }
        }
        'b_1acafc: {
        c.cycles += 2;
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_1acafc;
        } else {
        c.cycles += 2;
        c.g[RA][0] = sx(0x001acafc);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025b800, 0x001acafc)?;
        break 'b_1acafc;
        }
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x001acb04);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x00261408, 0x001acb04)?;
        c.g[RA][0] = sx(0x001acb0c);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025b800, 0x001acb0c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001acb1c);
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x001a97b8, 0x001acb1c)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1acb34;
        } else {
        break 'b_1acb24;
        }
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001acb30);
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x001a97b8, 0x001acb30)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_1acb34;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateLogin_v33` (0x001bf378): the same popup-close sequence as
// `cFEUseExistingAccount::OnAbortPopupEvent`.
pub fn fe_state_login_v33(c: &mut Cpu) {
    guest(c, |c| {

        'b_1bf424: {
        'b_1bf414: {
        'b_1bf3ac: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0xf;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A2][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if t {
        break 'b_1bf3ac;
        } else {
        c.cycles += 3;
        c.g[V0][0] = 0x14;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if t {
        break 'b_1bf414;
        } else {
        break 'b_1bf3ac;
        }
        }
        }
        'b_1bf3c0: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_1bf3c0;
        } else {
        c.cycles += 2;
        c.g[RA][0] = sx(0x001bf3c0);
        call_ptr(c, 0x00260f80, 0x001bf3c0)?;
        break 'b_1bf3c0;
        }
        }
        'b_1bf3dc: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x001bf3c8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x00261460, 0x001bf3c8)?;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if t {
        break 'b_1bf3dc;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x001bf3d8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x002613c8, 0x001bf3d8)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        break 'b_1bf3dc;
        }
        }
        'b_1bf3ec: {
        c.cycles += 2;
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_1bf3ec;
        } else {
        c.cycles += 2;
        c.g[RA][0] = sx(0x001bf3ec);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025b800, 0x001bf3ec)?;
        break 'b_1bf3ec;
        }
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x001bf3f4);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x00261408, 0x001bf3f4)?;
        c.g[RA][0] = sx(0x001bf3fc);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025b800, 0x001bf3fc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001bf40c);
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x001a97b8, 0x001bf40c)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1bf424;
        } else {
        break 'b_1bf414;
        }
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001bf420);
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x001a97b8, 0x001bf420)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_1bf424;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateStats_FormatTime` (0x001c4f58).
//
// `void FormatTime(void* self, char* buf, int frames)`: format a frame count
// as "h:mm:ss.mmm" (or "m:ss.mmm" when under an hour).

pub fn fe_state_stats_format_time(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.mtc1(6, 0);
        c.cvt_s_w(0, 0);
        ldf(c, 1, gp(c, -0x590c), 0);
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x3c;
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x3c;
        c.mul_s(0, 0, 1);
        c.cycles += 22;
        c.g[AT][0] = 0x447a0000;
        c.mtc1(1, 2);
        let buf = c.g[A1][0];
        c.g[T2][0] = buf;
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[T0][0] = c.g[V0][0];
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.div(0, 8, 3);
        c.sub_s(0, 0, 1);
        c.mul_s(0, 0, 2);
        c.cvt_w_s(1, 0);
        c.mfc1(9, 1);
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = c.g[A0][0];
        c.div(0, 7, 3);
        c.g[A2][0] = c.lo[0];
        c.mult(0, 2, 7, 3);
        c.mult(0, 3, 6, 3);
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        let hours = c.g[A2][0] as i64;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        if hours > 0 {
            c.cycles += 6;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = c.g[T2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f00));
            call_ptr(c, 0x0041_7828, 0x001c_4fe8)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 8;
            c.g[A2][0] = c.g[A3][0];
            c.g[A1][0] = 0x460000;
            c.g[A3][0] = c.g[T0][0];
            c.g[A0][0] = c.g[T2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f18));
            c.g[T0][0] = c.g[T1][0];
            call_ptr(c, 0x0041_7828, 0x001c_500c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MessageCenter_GetIndexInType` (0x001e2dc0).
//
// `int MessageCenter::GetIndexInType(int type, int id)`: walk the message
// type's entries to the one holding `id` and return its running index.

pub fn message_center_get_index_in_type(c: &mut Cpu) {
    guest(c, |c| {

        'b_1e2e78: {
        c.cycles += 15;
        c.g[A3][0] = c.g[A0][0];
        c.g[V1][0] = 0x18;
        c.mult(0, 3, 7, 3);
        c.g[V0][0] = 0x4c0000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c08));
        c.g[A0][0] = 0x14;
        c.g[V0][0] = 0x440000;
        c.g[T1][0] = 0u64;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
        c.g[T0][0] = 0u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.mult(0, 2, 2, 4);
        let t = (c.g[A3][0] as i64) < 0;
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        if t {
        break 'b_1e2e78;
        } else {
        c.cycles += 5;
        c.g[T5][0] = c.g[A2][0];
        c.g[T6][0] = c.g[A1][0];
        c.g[T4][0] = 0x18;
        c.g[T3][0] = 0x14;
        'l_1e2e10: loop {
        'b_1e2e5c: {
        c.cycles += 12;
        c.mult(0, 3, 8, 12);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T5][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.mult(0, 2, 3, 11);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T6][0] as u32)));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
        if c.g[V0][0] != 0u64 {
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        break 'b_1e2e5c;
        } else {
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] != c.g[V0][0];
        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
        if t {
        break 'b_1e2e78;
        } else {
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
        let t = true;
        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        if t {
        break 'b_1e2e78;
        } else {
        break 'b_1e2e5c;
        }
        }
        }
        }
        c.cycles += 7;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
        c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
        let t = c.g[A1][0] == 0u64;
        if c.g[V0][0] == 0 { c.g[T1][0] = c.g[A0][0]; }
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x001e2e10); }
        continue 'l_1e2e10;
        } else {
        break 'b_1e2e78;
        }
        }
        }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `OVScreenMultiplayerStandings_Init` (0x001e8f00).
//
// `void OVScreenMultiplayerStandings::Init(self)`: build the standings and
// header widgets and hook up the audio cue.

pub fn ov_screen_multiplayer_standings_init(c: &mut Cpu) {
    guest(c, |c| {

        'b_1e8f50: {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1640));
        c.g[RA][0] = sx(0x001e8f24);
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x00317670, 0x001e8f24)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001e8f38);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00397cc0, 0x001e8f38)?;
        let t = c.g[V0][0] == 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if t {
        break 'b_1e8f50;
        } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x001e8f50);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x0039c870, 0x001e8f50)?;
        break 'b_1e8f50;
        }
        }
        'b_1e8f88: {
        c.cycles += 11;
        c.g[A0][0] = 0x470000;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x001e8f60);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
        call_ptr(c, 0x00317670, 0x001e8f60)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001e8f74);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00397cc0, 0x001e8f74)?;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[V0][0];
        if t {
        break 'b_1e8f88;
        } else {
        c.cycles += 3;
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x001e8f88);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x0039c870, 0x001e8f88)?;
        break 'b_1e8f88;
        }
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x001e8f90);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001e8fc0, 0x001e8f90)?;
        c.g[RA][0] = sx(0x001e8f98);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020a380, 0x001e8f98)?;
        c.g[RA][0] = sx(0x001e8fa0);
        call_ptr(c, 0x0028b180, 0x001e8fa0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001e8fac);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x00294f78, 0x001e8fac)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateNetMessenger_SubmitKeyboardInput` (0x001a4750).
//
// `void cFEStateNetMessenger::SubmitKeyboardInput(self)`: if the messenger
// is in the "typing" or "waiting" state, submit the text through the active
// dialog.

pub fn fe_state_net_messenger_submit_keyboard_input(c: &mut Cpu) {
    guest(c, |c| {

        'b_1a4804: {
        'b_1a4800: {
        'b_1a479c: {
        'b_1a4788: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0x1;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x69c));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x2;
        if t {
        break 'b_1a4788;
        } else {
        c.cycles += 2;
        let t = c.g[V1][0] == c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        if t {
        break 'b_1a479c;
        } else {
        c.cycles += 2;
        let t = true;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        if t {
        break 'b_1a4804;
        } else {
        break 'b_1a4788;
        }
        }
        }
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001a4794);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
        call_ptr(c, 0x001a4810, 0x001a4794)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        if t {
        break 'b_1a4800;
        } else {
        break 'b_1a479c;
        }
        }
        'b_1a47b0: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_1a47b0;
        } else {
        c.cycles += 2;
        c.g[RA][0] = sx(0x001a47b0);
        call_ptr(c, 0x00260f80, 0x001a47b0)?;
        break 'b_1a47b0;
        }
        }
        'b_1a47cc: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x001a47b8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x00261460, 0x001a47b8)?;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if t {
        break 'b_1a47cc;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x001a47c8);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        call_ptr(c, 0x002613c8, 0x001a47c8)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        break 'b_1a47cc;
        }
        }
        'b_1a47e0: {
        c.cycles += 2;
        let t = c.g[V0][0] != 0u64;
        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if t {
        break 'b_1a47e0;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x001a47dc);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025b800, 0x001a47dc)?;
        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        break 'b_1a47e0;
        }
        }
        c.cycles += 8;
        c.g[RA][0] = sx(0x001a47e8);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001a98b0, 0x001a47e8)?;
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x69c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001a47fc);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x74));
        call_ptr(c, 0x00262690, 0x001a47fc)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_1a4800;
        }
        c.cycles += 1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        break 'b_1a4804;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `LobbyMan_CreateRoom` (0x0025dac0).
//
// `void LobbyMan_CreateRoom(LobbyMan* self, char* name, int players, int mode)`:
// format the room name, append the optional password and send the create
// request.

pub fn lobby_man_create_room(c: &mut Cpu) {
    guest(c, |c| {

        'b_25db68: {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x240));
        c.g[V1][0] = c.g[A2][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x220));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x210));
        c.g[S1][0] = c.g[A3][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x230));
        c.g[S2][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x200), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[A1][0];
        if t {
        break 'b_25db68;
        } else {
        'b_25db40: {
        c.cycles += 16;
        c.g[A2][0] = c.g[A0][0];
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[A1][0] = 0x4a0000;
        c.g[A3][0] = c.g[V1][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3150));
        c.g[RA][0] = sx(0x0025db08);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00417828, 0x0025db08)?;
        c.g[A2][0] = 0x4a0000;
        c.g[A3][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f58));
        c.sb((c.g[SP][0] as u32), 0u64);
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0025db24);
        c.g[A1][0] = 0x100;
        call_ptr(c, 0x003ec180, 0x0025db24)?;
        let t = c.g[S1][0] == 0u64;
        c.g[A2][0] = 0x4a0000;
        if t {
        break 'b_25db40;
        } else {
        c.cycles += 5;
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3040));
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0025db40);
        c.g[A1][0] = 0x100;
        call_ptr(c, 0x003ec180, 0x0025db40)?;
        break 'b_25db40;
        }
        }
        c.cycles += 10;
        c.g[T0][0] = 0x270000;
        c.g[A2][0] = 0x726f0000;
        c.g[AT][0] = 0x41a00000;
        c.mtc1(1, 12);
        c.g[A0][0] = c.g[S2][0];
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x72a0));
        c.g[A1][0] = 0x11;
        c.g[A2][0] = c.g[A2][0] | 0x6f6d;
        c.g[RA][0] = sx(0x0025db68);
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, 0x0025b608, 0x0025db68)?;
        break 'b_25db68;
        }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x230));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x220));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x210));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x200));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x240));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `LobbyMan_GetErrorString` (0x00260e50).
//
// `char* LobbyMan_GetErrorString(LobbyMan* self, char* buf, int code)`:
// decode the packed error/status word into text.

const MSG_LOBBY_DUPLICATE: u32 = 0x6475_706c; // "udpl"
const MSG_LOBBY_ITEM: u32 = 0x7469_6d65; // "item"

pub fn lobby_man_get_error_string(c: &mut Cpu) {
    guest(c, |c| {

        'b_260f00: {
        'b_260eb8: {
        'b_260ea0: {
        'b_260e88: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0x64750000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[T1][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[V0][0] = c.g[V0][0] | 0x706c;
        let t = c.g[T1][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if t {
        break 'b_260e88;
        } else {
        c.cycles += 4;
        c.g[V0][0] = 0x74690000;
        c.g[V0][0] = c.g[V0][0] | 0x6d65;
        let t = c.g[T1][0] == c.g[V0][0];
        c.g[A2][0] = c.g[T1][0];
        if t {
        break 'b_260ea0;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        if t {
        break 'b_260eb8;
        } else {
        break 'b_260e88;
        }
        }
        }
        }
        c.cycles += 6;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x00260e98);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe10));
        call_ptr(c, 0x003182d8, 0x00260e98)?;
        let t = true;
        c.g[V0][0] = c.g[S0][0];
        if t {
        break 'b_260f00;
        } else {
        break 'b_260ea0;
        }
        }
        c.cycles += 6;
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x00260eb0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3320));
        call_ptr(c, 0x003182d8, 0x00260eb0)?;
        let t = true;
        c.g[V0][0] = c.g[S0][0];
        if t {
        break 'b_260f00;
        } else {
        break 'b_260eb8;
        }
        }
        c.cycles += 18;
        c.g[T0][0] = sx((c.g[T1][0] as u32) >> 16);
        c.g[A1][0] = 0x480000;
        c.g[T1][0] = sx((c.g[T1][0] as u32) >> 8);
        c.g[A0][0] = c.g[SP][0];
        c.g[T0][0] = c.g[T0][0] & 0xff;
        c.g[T1][0] = c.g[T1][0] & 0xff;
        c.g[A3][0] = sx((c.g[A2][0] as u32) >> 24);
        c.g[T2][0] = c.g[A2][0] & 0xff;
        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
        c.g[RA][0] = sx(0x00260ee4);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe20));
        call_ptr(c, 0x003194a8, 0x00260ee4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x00260ef0);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x00317fe8, 0x00260ef0)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00260efc);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x00260efc)?;
        c.g[V0][0] = c.g[S0][0];
        break 'b_260f00;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cSetupMan_HostRecvCancel` (0x00265398).
//
// `void cSetupMan_HostRecvCancel(cSetupMan* self, Packet* out)`: acknowledge a
// cancelled join and pass the queue on.

pub fn setup_man_host_recv_cancel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x3;
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save_ra(c, 0x20);
        let this = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(this).lw(c);
        c.g[A1][0] = 0x480000;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 2;
            c.g[V0][0] = 0x2;
            sw(c, this, 0, c.g[V0][0]);
        }
        c.cycles += 24;
        c.g[S0][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_53d8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0xf7;
        c.g[V0][0] = 0x480000;
        sw(c, c.g[S1][0] as u32, 0, c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = 0x480000;
        sw(c, c.g[S1][0] as u32, 4, c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        let s0 = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(s0.wrapping_add(0xf0)).lw(c);
        c.g[A3][0] = 0;
        sw(c, sp(c), 0x10, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_5410)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0 {
            sw(c, c.g[A1][0] as u32, 8, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw(c, sp(c), 0x10);
        frame.restore(c, 16, 0x40);
        sw(c, c.g[A1][0] as u32, 0, c.g[V0][0]);
        frame.restore(c, 17, 0x30);
        c.g[V1][0] = lw(c, c.g[V0][0] as u32, 4);
        frame.restore_ra(c, 0x20);
        sw(c, c.g[A1][0] as u32, 4, c.g[V1][0]);
        c.g[A0][0] = lw(c, c.g[V0][0] as u32, 4);
        sw(c, c.g[A0][0] as u32, 0, c.g[A1][0]);
        c.g[V0][0] = lw(c, sp(c), 0x10);
        sw(c, c.g[V0][0] as u32, 4, c.g[A1][0]);
        sw(c, sp(c), 0, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `InSeq_EventTrackStart` (0x00273208).
//
// `int InSeq_EventTrackStart(EventTrack* t, void* seq, void* source, void* owner)`:
// start firing the sequence's keys.

pub fn in_seq_event_track_start(c: &mut Cpu) {
    guest(c, |c| {

        'b_2732b0: {
        'b_2732ac: {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.g[V0][0] = c.lhu((c.g[A1][0] as u32));
        let t = c.g[V0][0] == 0u64;
        c.g[S1][0] = c.g[A3][0];
        if t {
        break 'b_2732ac;
        } else {
        c.cycles += 12;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
        c.g[RA][0] = sx(0x0027324c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x00272308, 0x0027324c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
        let t = c.g[V0][0] == 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        if t {
        break 'b_2732ac;
        } else {
        c.cycles += 3;
        c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x6));
        if c.g[V0][0] != 0u64 {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        break 'b_2732b0;
        } else {
        c.cycles += 2;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
        'l_273278: loop {
        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = c.g[V0][0] | 0x10;
        c.sw((c.g[S1][0] as u32), c.g[V0][0]);
        c.g[RA][0] = sx(0x0027328c);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        call_ptr(c, 0x00273418, 0x0027328c)?;
        c.g[RA][0] = sx(0x00273294);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00273468, 0x00273294)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] == 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
        if t {
        break 'b_2732ac;
        } else {
        c.cycles += 3;
        c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
        if c.g[V0][0] == 0u64 {
        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x00273278); }
        continue 'l_273278;
        } else {
        break 'b_2732ac;
        }
        }
        }
        }
        }
        }
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        break 'b_2732b0;
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ChaseCamController_BeginScriptCamera` (0x00161fa0).
//
// `void ChaseCamController::BeginScriptCamera(self, int state)`: latch the
// new script-camera state and install the matching algorithm.

pub fn chase_cam_controller_begin_script_camera(c: &mut Cpu) {
    guest(c, |c| {

        'b_16204c: {
        'b_161fe8: {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V1][0] = 0x4c;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = c.g[V0][0] | 0x2;
        let t = c.g[S1][0] != c.g[V1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[A0][0]);
        if t {
        break 'b_161fe8;
        } else {
        c.cycles += 7;
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x00161fe0);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0015d050, 0x00161fe0)?;
        let t = true;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[S1][0]);
        if t {
        break 'b_16204c;
        } else {
        break 'b_161fe8;
        }
        }
        }
        'b_16203c: {
        c.cycles += 3;
        c.g[V0][0] = 0x5d;
        if c.g[S1][0] != c.g[V0][0] {
        c.g[A0][0] = c.g[S0][0];
        break 'b_16203c;
        } else {
        'b_162018: {
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0] & 0x1;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[A0][0] & 0x4;
        if t {
        break 'b_162018;
        } else {
        c.cycles += 6;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x00162010);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
        call_ptr(c, 0x0015d050, 0x00162010)?;
        let t = true;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[S1][0]);
        if t {
        break 'b_16204c;
        } else {
        break 'b_162018;
        }
        }
        }
        'b_16202c: {
        'b_162028: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[S0][0];
        if t {
        break 'b_162028;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[A1][0] = 0x42;
        if t {
        break 'b_16202c;
        } else {
        break 'b_162028;
        }
        }
        }
        c.cycles += 1;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        break 'b_16202c;
        }
        c.cycles += 4;
        c.g[RA][0] = sx(0x00162034);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x0015d050, 0x00162034)?;
        let t = true;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[S1][0]);
        if t {
        break 'b_16204c;
        } else {
        break 'b_16203c;
        }
        }
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x00162048);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x0015d050, 0x00162048)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[S1][0]);
        break 'b_16204c;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MCManager_OnOperationDone` (0x0023ea90).
//
// `void MCManager::OnOperationDone(MCManager* self)`: finish a memory-card
// operation, refresh the slot and return to the slot state.

pub fn mc_manager_on_operation_done(c: &mut Cpu) {
    guest(c, |c| {

        'b_23eb44: {
        'b_23eb18: {
        'b_23eb00: {
        'b_23eadc: {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
        if c.g[V0][0] == 0u64 {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x12c));
        break 'b_23eadc;
        } else {
        'b_23ead0: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
        let t = c.g[V0][0] == 0u64;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x478), 0u64);
        if t {
        break 'b_23ead0;
        } else {
        c.cycles += 6;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x160, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0023ead0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0023ead0)?;
        break 'b_23ead0;
        }
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
        let t = true;
        c.g[A1][0] = 0x2;
        if t {
        break 'b_23eb00;
        } else {
        break 'b_23eadc;
        }
        }
        }
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x440), 0u64);
        if t {
        break 'b_23eb18;
        } else {
        c.cycles += 7;
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x0023eaf0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023fb18, 0x0023eaf0)?;
        c.g[RA][0] = sx(0x0023eaf8);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023fae0, 0x0023eaf8)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
        c.g[A1][0] = 0x31;
        break 'b_23eb00;
        }
        }
        c.cycles += 6;
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x8, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0023eb10);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0023eb10)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        if t {
        break 'b_23eb44;
        } else {
        break 'b_23eb18;
        }
        }
        'b_23eb38: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x0023eb20);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023c8d0, 0x0023eb20)?;
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_23eb38;
        } else {
        c.cycles += 4;
        c.g[RA][0] = sx(0x0023eb30);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00241fd0, 0x0023eb30)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        if t {
        break 'b_23eb44;
        } else {
        break 'b_23eb38;
        }
        }
        }
        c.cycles += 3;
        c.g[RA][0] = sx(0x0023eb40);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00242050, 0x0023eb40)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        break 'b_23eb44;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MCManager_State22_Load` (0x0023ff00).
//
// `void MCManager::State22_Load(MCManager* self)`: poll the memory-card
// operation and either finish it or start the next step.

pub fn mc_manager_state22_load(c: &mut Cpu) {
    guest(c, |c| {

        'b_23ffb4: {
        'b_23ffb0: {
        'b_23ff24: {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x108));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        if t {
        break 'b_23ff24;
        } else {
        c.cycles += 2;
        let t = true;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x108), c.g[V0][0]);
        if t {
        break 'b_23ffb0;
        } else {
        break 'b_23ff24;
        }
        }
        }
        c.cycles += 8;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
        c.g[V0][0] = vslot(c, c.g[V1][0], 0x58, V0);
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023ff3c);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0023ff3c)?;
        if c.g[V0][0] != 0u64 {
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        break 'b_23ffb4;
        } else {
        'b_23ff8c: {
        c.cycles += 8;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
        c.g[V0][0] = vslot(c, c.g[V1][0], 0x248, V0);
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023ff5c);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0023ff5c)?;
        let t = c.g[V0][0] == 0u64;
        c.g[A1][0] = 0x24;
        if t {
        break 'b_23ff8c;
        } else {
        c.cycles += 10;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x8, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0023ff78);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0023ff78)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023ff84);
        call_ptr(c, t, 0x0023ff84)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        if t {
        break 'b_23ffb4;
        } else {
        break 'b_23ff8c;
        }
        }
        }
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
        c.g[A1][0] = 0x23;
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x8, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0023ffa4);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0023ffa4)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023ffb0);
        call_ptr(c, t, 0x0023ffb0)?;
        break 'b_23ffb0;
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        break 'b_23ffb4;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEPopupConfirm_MeasureLine` (0x001c84a8).
//
// `void cFEPopupConfirm::MeasureLine(self, TextMetrics* m, float* out, void* dst)`:
// measure one line of the popup text and scale it by the font metrics.

pub fn fe_popup_confirm_measure_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        let metrics = c.g[A1][0] as u32;
        frame.save(c, 18, 0x20);
        let out = c.g[A2][0];
        frame.save_ra(c, 0x10);
        let dst = c.g[A3][0];
        c.g[S0][0] = sx(metrics);
        c.g[S1][0] = out;
        c.g[S2][0] = dst;
        c.ldl(2, metrics.wrapping_add(0x57));
        c.ldr(2, metrics.wrapping_add(0x50));
        c.g[V1][0] = lw(c, metrics, 0x58);
        c.sdl(2, sp(c).wrapping_add(7));
        c.sdr(2, sp(c));
        sw(c, sp(c), 8, c.g[V1][0]);
        c.g[A0][0] = sx(metrics);
        call_ptr(c, 0x003a_04f0, 0x001c_84e8)?;
        ldf(c, 2, sp(c), 0);
        c.g[A0][0] = sx(metrics);
        ldf(c, 3, sp(c), 4);
        let v0 = c.g[V0][0] as u32;
        ldf(c, 0, v0, 0x30);
        ldf(c, 1, v0, 0x34);
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 3);
        c.swc1(0, v0.wrapping_add(0x38));
        c.swc1(1, v0.wrapping_add(0x3c));
        call_ptr(c, 0x003a_04f0, 0x001c_8510)?;
        let v0 = c.g[V0][0] as u32;
        c.g[A0][0] = sx(v0);
        c.g[A1][0] = c.g[S1][0];
        ldf(c, 12, v0, 0x38);
        c.g[A2][0] = c.g[S2][0];
        ldf(c, 13, v0, 0x3c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_1fb0, 0x001c_852c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003a_04f0, 0x001c_8534)?;
        ldf(c, 0, c.g[V0][0] as u32, 0x14);
        c.cvt_s_w(0, 0);
        ldf(c, 1, c.g[V0][0] as u32, 0x34);
        ldf(c, 2, sp(c), 4);
        frame.restore(c, 16, 0x40);
        c.mul_s(0, 0, 1);
        frame.restore(c, 17, 0x30);
        c.mul_s(2, 2, 0);
        c.swc1(2, c.g[S2][0] as u32 + 0xc);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEPopupConfirm_MeasureLineW` (0x001c8568): the wide-string variant of
// [`fe_popup_confirm_measure_line`].
pub fn fe_popup_confirm_measure_line_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        let metrics = c.g[A1][0] as u32;
        frame.save(c, 18, 0x20);
        let out = c.g[A2][0];
        frame.save_ra(c, 0x10);
        let dst = c.g[A3][0];
        c.g[S0][0] = sx(metrics);
        c.g[S1][0] = out;
        c.g[S2][0] = dst;
        c.ldl(2, metrics.wrapping_add(0x57));
        c.ldr(2, metrics.wrapping_add(0x50));
        c.g[V1][0] = lw(c, metrics, 0x58);
        c.sdl(2, sp(c).wrapping_add(7));
        c.sdr(2, sp(c));
        sw(c, sp(c), 8, c.g[V1][0]);
        c.g[A0][0] = sx(metrics);
        call_ptr(c, 0x003a_04f0, 0x001c_85a8)?;
        ldf(c, 2, sp(c), 0);
        c.g[A0][0] = sx(metrics);
        ldf(c, 3, sp(c), 4);
        let v0 = c.g[V0][0] as u32;
        ldf(c, 0, v0, 0x30);
        ldf(c, 1, v0, 0x34);
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 3);
        c.swc1(0, v0.wrapping_add(0x38));
        c.swc1(1, v0.wrapping_add(0x3c));
        call_ptr(c, 0x003a_04f0, 0x001c_85d0)?;
        let v0 = c.g[V0][0] as u32;
        c.g[A0][0] = sx(v0);
        c.g[A1][0] = c.g[S1][0];
        ldf(c, 12, v0, 0x38);
        c.g[A2][0] = c.g[S2][0];
        ldf(c, 13, v0, 0x3c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_21f0, 0x001c_85ec)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003a_04f0, 0x001c_85f4)?;
        ldf(c, 0, c.g[V0][0] as u32, 0x14);
        c.cvt_s_w(0, 0);
        ldf(c, 1, c.g[V0][0] as u32, 0x34);
        ldf(c, 2, sp(c), 4);
        frame.restore(c, 16, 0x40);
        c.mul_s(0, 0, 1);
        frame.restore(c, 17, 0x30);
        c.mul_s(2, 2, 0);
        c.swc1(2, c.g[S2][0] as u32 + 0xc);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cSetupMan_ClientRecvMsg0` (0x00269240) and `cSetupMan_ClientRecvMsg2`
// (0x00269300): store the received remote character and queue a reply.

/// The shared reply tail: link the freshly allocated packet at `a1` into the
/// pending queue and restore the frame.
fn client_recv_reply(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = lw(c, sp(c), 0x10);
    let a16 = sp(c).wrapping_add(0x50);
    c.lq(16, a16);
    sw(c, c.g[A1][0] as u32, 0, c.g[V0][0]);
    let a17 = sp(c).wrapping_add(0x40);
    let v0 = c.g[V0][0] as u32;
    c.g[V1][0] = lw(c, v0, 4);
    let a18 = sp(c).wrapping_add(0x30);
    c.lq(18, a18);
    sw(c, c.g[A1][0] as u32, 4, c.g[V1][0]);
    c.g[A0][0] = lw(c, v0, 4);
    sw(c, c.g[A0][0] as u32, 0, c.g[A1][0]);
    c.g[V0][0] = lw(c, sp(c), 0x10);
    sw(c, c.g[V0][0] as u32, 4, c.g[A1][0]);
    sw(c, sp(c), 0, c.g[A1][0]);
    let _ = a17;
}

fn recv_msg(c: &mut Cpu, reply: u64, character: u64, ret1: u32, ret2: u32, ret3: u32) -> Flow {
    c.cycles += 33;
    let frame = Frame::open(c, 0x60);
    frame.save(c, 16, 0x50);
    frame.save(c, 17, 0x40);
    c.g[S0][0] = reply;
    frame.save(c, 18, 0x30);
    frame.save_ra(c, 0x20);
    let this = c.g[A0][0] as u32;
    sw(c, this, 0, c.g[A2][0]);
    c.g[A2][0] = character;
    call_ptr(c, 0x0026_4d80, ret1)?;
    c.g[A1][0] = 0x480000;
    let s1 = Addr(gp(c, -0xc8)).lw(c) as u32;
    c.g[S1][0] = sx(s1);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
    c.g[A0][0] = 0x8;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, ret2)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = 0x480000;
    c.g[V0][0] = 0x480000;
    sw(c, c.g[S2][0] as u32, 0, c.g[S0][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    sw(c, c.g[S2][0] as u32, 4, c.g[V0][0]);
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] = lw(c, s1, 0xf0);
    sw(c, sp(c), 0x10, c.g[V0][0]);
    call_ptr(c, 0x0031_7d70, ret3)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
    if c.g[V0][0] != 0 {
        sw(c, c.g[A1][0] as u32, 8, c.g[S2][0]);
    }
    client_recv_reply(c);
    let a17 = sp(c).wrapping_add(0x40);
    c.lq(17, a17);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

pub fn setup_man_client_recv_msg0(c: &mut Cpu) {
    guest(c, |c| {

        'b_2692c4: {
        c.cycles += 33;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = 0xf3;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.sw((c.g[A0][0] as u32), c.g[A2][0]);
        c.g[RA][0] = sx(0x00269264);
        c.g[A2][0] = c.g[A3][0];
        call_ptr(c, 0x00264d80, 0x00269264)?;
        c.g[A1][0] = 0x480000;
        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x8;
        c.g[A2][0] = 0x20000000;
        c.g[RA][0] = sx(0x00269280);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x00269280)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = 0x480000;
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S2][0] as u32), c.g[S0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = sx(0x002692b4);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x002692b4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0u64 {
        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
        break 'b_2692c4;
        } else {
        break 'b_2692c4;
        }
        }
        c.cycles += 15;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sw((c.g[A1][0] as u32), c.g[V0][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[A0][0] as u32), c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t; return Ok(());

        Ok(())
    })
}

pub fn setup_man_client_recv_msg2(c: &mut Cpu) {
    guest(c, |c| {

        'b_269384: {
        c.cycles += 33;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = 0xf5;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.sw((c.g[A0][0] as u32), c.g[A2][0]);
        c.g[RA][0] = sx(0x00269324);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x00264d80, 0x00269324)?;
        c.g[A1][0] = 0x480000;
        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x8;
        c.g[A2][0] = 0x20000000;
        c.g[RA][0] = sx(0x00269340);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x00269340)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = 0x480000;
        c.g[V0][0] = 0x480000;
        c.sw((c.g[S2][0] as u32), c.g[S0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = sx(0x00269374);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x00269374)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0u64 {
        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
        break 'b_269384;
        } else {
        break 'b_269384;
        }
        }
        c.cycles += 15;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sw((c.g[A1][0] as u32), c.g[V0][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[A0][0] as u32), c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `BoltFile_CacheShutdown` (0x0019cf40).
//
// `void BoltFile_CacheShutdown(BoltFile* self)`: free every cached slot of
// every bolt file and drop the cache.

pub fn bolt_file_cache_shutdown(c: &mut Cpu) {
    guest(c, |c| {

        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V0][0] = 0xb0000;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.g[V0][0] | 0x4008;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = c.g[A0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = 0u64;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S6][0] = 0x1;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        'l_19cf80: loop {
        c.cycles += 4;
        c.g[S0][0] = 0u64;
        c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
        c.g[A2][0] = c.g[S0][0];
        'l_19cf90: loop {
        c.cycles += 7;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0019cfa0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        call_ptr(c, 0x0019d4b8, 0x0019cfa0)?;
        c.g[V0][0] = ((c.g[S0][0] as i64) < 256i64) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[A2][0] = c.g[S0][0];
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x0019cf90); }
        continue 'l_19cf90;
        } else {
        c.cycles += 5;
        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[S2][0] as u32)));
        c.g[S1][0] = c.g[S3][0];
        c.g[V0][0] = ((c.g[S1][0] as i64) < 10i64) as u64;
        let t = c.g[V0][0] != 0u64;
        c.sw((c.g[V1][0] as u32), c.g[S6][0]);
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x0019cf80); }
        continue 'l_19cf80;
        } else {
        'b_19cfd8: {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
        let t = c.g[A0][0] == 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        if t {
        break 'b_19cfd8;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x0019cfd4);
        call_ptr(c, 0x00317e50, 0x0019cfd4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        break 'b_19cfd8;
        }
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1788), 0u64);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t; return Ok(());
        }
        }
        }
        }

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `GameStateEventSwitchLoad_Enter` (0x002368a0).
//
// `void GameStateEventSwitchLoad::Enter(self)`: load the game-state data and
// start playing its event tracks.

pub fn game_state_event_switch_load_enter(c: &mut Cpu) {
    guest(c, |c| {

        'b_236944: {
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x002368c4);
        c.g[A0][0] = 0u64;
        call_ptr(c, 0x0014de68, 0x002368c4)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
        c.g[RA][0] = sx(0x002368d0);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x0026f228, 0x002368d0)?;
        c.g[RA][0] = sx(0x002368d8);
        call_ptr(c, 0x0014dc80, 0x002368d8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002368e4);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014dd58, 0x002368e4)?;
        c.g[V1][0] = 0x530000;
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
        c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
        if c.g[V0][0] != 0u64 {
        c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
        break 'b_236944;
        } else {
        c.cycles += 9;
        c.g[RA][0] = sx(0x00236900);
        c.g[S0][0] = 0u64;
        call_ptr(c, 0x0014dc80, 0x00236900)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023690c);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x0023690c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
        if (c.g[V0][0] as i64) <= 0 {
        c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
        break 'b_236944;
        } else {
        c.cycles += 1;
        c.g[A1][0] = c.g[S0][0];
        'l_236920: loop {
        c.cycles += 8;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x00236930);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        call_ptr(c, 0x00145cb0, 0x00236930)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = c.g[S0][0];
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x00236920); }
        continue 'l_236920;
        } else {
        c.cycles += 1;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
        break 'b_236944;
        }
        }
        }
        }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `SSXAudioSystem_OnReplayBegin` (0x0028f200).
//
// `void SSXAudioSystem::OnReplayBegin(self)`: start the replay music unless it
// is already playing.

pub fn ssx_audio_system_on_replay_begin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(this);
        let mon = this.wrapping_add(0x118);
        c.g[S1][0] = sx(mon);
        sw(c, this, 0x6294, 0);
        c.g[V0][0] = lw(c, this, 0x530);
        let t = c.g[V0][0] == 0;
        sw(c, this, 0x6298, 0);
        if !t {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x002b_49e0, 0x0028_f230)?;
            c.g[V1][0] = 0xc9;
            let not_playing = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 1;
            if not_playing {
                c.cycles += 1;
                sw(c, this, 0x6298, c.g[V0][0]);
                c.cycles += 5;
                frame.restore(c, 16, 0x20);
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_d960, 0x0028_f244)?;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = 1;
        if !t {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x002b_49e0, 0x0028_f254)?;
            c.g[V1][0] = 0xc9;
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V0][0] = lw(c, this, 0x118);
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, this, 0x62ac);
                sw(c, this, 0x508, c.g[V0][0]);
                c.g[V0][0] = lw(c, this, 0x118);
            }
            c.cycles += 13;
            c.g[V1][0] = 0x4a0000;
            c.ldl(6, 0x4a364f);
            c.ldr(6, 0x4a3648);
            c.g[A1][0] = 0xa;
            c.g[A0][0] = lw(c, c.g[V0][0] as u32, 0);
            c.g[A3][0] = 0;
            c.g[T0][0] = 3;
            c.g[T1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            call_ptr(c, 0x002a_dca0, 0x0028_f29c)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 2;
        sw(c, this, 0x6294, c.g[V0][0]);
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `WScriptProcess_dtor` (0x00307738).
//
// `void WScriptProcess::~WScriptProcess(WScriptProcess* self, int flags)`:
// destroy the element array, release the script object and optionally delete
// the process.

pub fn w_script_process_dtor(c: &mut Cpu) {
    guest(c, |c| {

        'b_3077b8: {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0x490000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6490));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        let t = c.g[A2][0] == 0u64;
        c.g[S2][0] = c.g[A1][0];
        if t {
        break 'b_3077b8;
        } else {
        'b_3077b0: {
        c.cycles += 6;
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0x44;
        c.mult(0, 4, 3, 2);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        if c.g[A2][0] == c.g[S0][0] {
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        break 'b_3077b0;
        } else {
        c.cycles += 2;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x44));
        'l_307788: loop {
        c.cycles += 9;
        c.g[A1][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x8, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x003077a0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x003077a0)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        if c.g[V0][0] != c.g[S0][0] {
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x44));
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x00307788); }
        continue 'l_307788;
        } else {
        c.cycles += 1;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        break 'b_3077b0;
        }
        }
        }
        }
        c.cycles += 2;
        c.g[RA][0] = sx(0x003077b8);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
        call_ptr(c, 0x00317e98, 0x003077b8)?;
        break 'b_3077b8;
        }
        }
        'b_3077d0: {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        if t {
        break 'b_3077d0;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x003077cc);
        c.g[A1][0] = 0x3;
        call_ptr(c, 0x00224df0, 0x003077cc)?;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        break 'b_3077d0;
        }
        }
        'b_3077e4: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_3077e4;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x003077e0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00317e50, 0x003077e0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_3077e4;
        }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `WScriptProcess_v04` (0x00307c40).
//
// `void WScriptProcess::v04(WScriptProcess* self)`: (re)initialise the
// process's element storage and run its entry function.

pub fn w_script_process_v04(c: &mut Cpu) {
    guest(c, |c| {

        'b_307c68: {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        if c.g[A0][0] == 0u64 {
        c.g[A1][0] = 0x490000;
        break 'b_307c68;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x00307c64);
        c.g[A1][0] = 0x3;
        call_ptr(c, 0x00224df0, 0x00307c64)?;
        c.g[A1][0] = 0x490000;
        break 'b_307c68;
        }
        }
        'b_307cac: {
        c.cycles += 14;
        c.g[A2][0] = 0x20000000;
        c.g[A0][0] = 0x4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6778));
        c.g[RA][0] = sx(0x00307c7c);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x00307c7c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00307c88);
        c.g[A1][0] = 0x8;
        call_ptr(c, 0x00224da0, 0x00307c88)?;
        c.g[V1][0] = 0x1;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32), c.g[V1][0]);
        let t = c.g[A2][0] == 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
        if t {
        break 'b_307cac;
        } else {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x00307cac);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0030ac98, 0x00307cac)?;
        break 'b_307cac;
        }
        }
        'b_307cf4: {
        c.cycles += 8;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x00307cb8);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x00309a60, 0x00307cb8)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0u64 {
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        break 'b_307cf4;
        } else {
        c.cycles += 10;
        c.g[A0][0] = 0x44;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.mult(0, 4, 5, 4);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
        let t = c.g[A1][0] as u32;
        c.g[RA][0] = sx(0x00307cf0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        call_ptr(c, t, 0x00307cf0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        break 'b_307cf4;
        }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `OVScreenInGameError_OnWidgetAction` (0x001fb0c0).
//
// `int OVScreenInGameError::OnWidgetAction(self, UIWidget* w, int action)`:
// map the dialog buttons onto the highlight/select handlers.

pub fn ov_screen_in_game_error_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {

        'b_1fb16c: {
        'b_1fb168: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.g[A0][0];
        let t = c.g[S1][0] == 0u64;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        if t {
        break 'b_1fb168;
        } else {
        'b_1fb104: {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        let t = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = 0x5;
        if t {
        break 'b_1fb104;
        } else {
        c.cycles += 2;
        let t = c.g[A2][0] != c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1fb16c;
        } else {
        c.cycles += 4;
        c.g[RA][0] = sx(0x001fb0fc);
        c.g[A2][0] = 0x5;
        call_ptr(c, 0x001fb2b8, 0x001fb0fc)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1fb16c;
        } else {
        break 'b_1fb104;
        }
        }
        }
        }
        'b_1fb13c: {
        c.cycles += 6;
        c.g[A0][0] = 0x470000;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x001fb114);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x570));
        call_ptr(c, 0x00317670, 0x001fb114)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x470000;
        if t {
        break 'b_1fb13c;
        } else {
        c.cycles += 8;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001fb128);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x001fb588, 0x001fb128)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001fb134);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x001fb458, 0x001fb134)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1fb16c;
        } else {
        break 'b_1fb13c;
        }
        }
        }
        c.cycles += 5;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x001fb148);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4b8));
        call_ptr(c, 0x00317670, 0x001fb148)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1fb16c;
        } else {
        c.cycles += 6;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001fb15c);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x001fb458, 0x001fb15c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001fb168);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x001fb588, 0x001fb168)?;
        break 'b_1fb168;
        }
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_1fb16c;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `BERewardInt_IsPeakFullyCompleted` (0x001577e0).
//
// `int BERewardInt::IsPeakFullyCompleted(self, peak, medal, index)`: true when
// all three peak medals are complete and the record levels add up to 24.

pub fn be_reward_int_is_peak_fully_completed(c: &mut Cpu) {
    guest(c, |c| {

        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = 0u64;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A2][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S3][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S3][0];
        'l_157810: loop {
        'b_157884: {
        'b_157830: {
        c.cycles += 6;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x00157820);
        c.g[A3][0] = c.g[S0][0];
        call_ptr(c, 0x00157920, 0x00157820)?;
        let t = c.g[V0][0] != 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        if t {
        break 'b_157830;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = 0u64;
        if t {
        break 'b_157884;
        } else {
        break 'b_157830;
        }
        }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
        if c.g[V0][0] != 0u64 {
        c.g[A0][0] = c.g[S3][0];
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x00157810); }
        continue 'l_157810;
        } else {
        c.cycles += 9;
        c.g[RA][0] = sx(0x00157844);
        c.g[S1][0] = 0u64;
        call_ptr(c, 0x0014dc80, 0x00157844)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00157850);
        c.g[A1][0] = 0x8;
        call_ptr(c, 0x0014dd58, 0x00157850)?;
        c.g[S0][0] = 0u64;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        'l_157860: loop {
        c.cycles += 7;
        c.g[RA][0] = sx(0x00157868);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x001552b0, 0x00157868)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 8i64) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = c.g[S0][0];
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x00157860); }
        continue 'l_157860;
        } else {
        c.cycles += 2;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        break 'b_157884;
        }
        }
        }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t; return Ok(());
        }

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Bolts_IndexTable3ByChar` (0x0014c3c8) / `Bolts_IndexTable4ByChar`
// (0x0014c488): clear a per-character index table and rebuild it from the
// character bytes, keeping the first occurrence of each character.

pub fn bolts_index_table3_by_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x40);
        c.g[A2][0] = 0x78;
        let this = c.g[A0][0] as u32;
        frame.save(c, 17, 0x30);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = sx(this.wrapping_add(0x2fc));
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[A0][0] = sx(this.wrapping_add(0x2fc));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x374));
        call_ptr(c, 0x003e_6448, 0x0014_c3fc)?;
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = sx(this.wrapping_add(0x374));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x78;
        call_ptr(c, 0x003e_6448, 0x0014_c410)?;
        c.g[V0][0] = lw(c, this, 0x24);
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !t {
            c.cycles += 3;
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[V1][0] = lw(c, this, 0xc);
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = lb(c, c.g[V0][0] as u32, 0);
                c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = lw(c, c.g[V1][0] as u32, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                let same = c.g[S3][0] == c.g[A1][0];
                sw(c, c.g[V1][0] as u32, 0, c.g[V0][0]);
                if !same {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[S3][0] = c.g[A1][0];
                    sw(c, c.g[V0][0] as u32, 0, c.g[A3][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = lw(c, this, 0x24);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw(c, this, 0xc);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0014_c428);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub fn bolts_index_table4_by_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x40);
        c.g[A2][0] = 0x78;
        let this = c.g[A0][0] as u32;
        frame.save(c, 17, 0x30);
        c.g[S0][0] = sx(this);
        c.g[S1][0] = sx(this.wrapping_add(0x3ec));
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[A0][0] = sx(this.wrapping_add(0x3ec));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x464));
        call_ptr(c, 0x003e_6448, 0x0014_c4bc)?;
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        c.g[A0][0] = sx(this.wrapping_add(0x464));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x78;
        call_ptr(c, 0x003e_6448, 0x0014_c4d0)?;
        c.g[V0][0] = lw(c, this, 0x28);
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !t {
            c.cycles += 3;
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[V1][0] = lw(c, this, 0x10);
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = lb(c, c.g[V0][0] as u32, 0);
                c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = lw(c, c.g[V1][0] as u32, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                let same = c.g[S3][0] == c.g[A1][0];
                sw(c, c.g[V1][0] as u32, 0, c.g[V0][0]);
                if !same {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[S3][0] = c.g[A1][0];
                    sw(c, c.g[V0][0] as u32, 0, c.g[A3][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = lw(c, this, 0x28);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw(c, this, 0x10);
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0014_c4e8);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cUIScroll_SetPartsVisible` (0x0039de68).
//
// `void cUIScroll::SetPartsVisible(self, bool visible)`: forward the
// visibility flag to the scroll bar, track and thumb objects.

pub fn ui_scroll_set_parts_visible(c: &mut Cpu) {
    guest(c, |c| {

        'b_39de9c: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
        let t = c.g[A2][0] == 0u64;
        c.g[S1][0] = c.g[A1][0];
        if t {
        break 'b_39de9c;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039de9c);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039de9c)?;
        break 'b_39de9c;
        }
        }
        'b_39debc: {
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = c.g[S1][0];
        if t {
        break 'b_39debc;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039debc);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039debc)?;
        break 'b_39debc;
        }
        }
        'b_39dedc: {
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = c.g[S1][0];
        if t {
        break 'b_39dedc;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039dedc);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039dedc)?;
        break 'b_39dedc;
        }
        }
        c.cycles += 19;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039def8);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039def8)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x48, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039df14);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039df14)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Conveyor_ctor` (0x00343010).
//
// `void Conveyor::Conveyor(self, int, void* params, void* dir)`: build the
// world object, copy the eight direction bytes and tag the render flags.

pub fn conveyor_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        let dir = c.g[A3][0];
        frame.save(c, 17, 0x20);
        let this = c.g[A0][0] as u32;
        c.g[S1][0] = sx(this);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A2][0] = 0xf;
        c.g[S0][0] = dir;
        call_ptr(c, 0x0034_fb00, 0x0034_3034)?;
        c.g[V0][0] = 0x490000;
        c.g[A3][0] = sx(this.wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x570));
        c.g[A2][0] = sx((dir as u32).wrapping_add(0x10));
        sw(c, this, 0xc, c.g[V0][0]);
        c.g[A1][0] = 0;
        ldf(c, 0, dir as u32, 4);
        c.swc1(0, this.wrapping_add(0x1c));
        ldf(c, 1, dir as u32, 8);
        ldf(c, 0, dir as u32, 0xc);
        c.swc1(1, sp(c));
        c.swc1(0, sp(c).wrapping_add(4));
        c.ldl(2, sp(c).wrapping_add(7));
        c.ldr(2, sp(c));
        c.sdl(2, this.wrapping_add(0x3f));
        c.sdr(2, this.wrapping_add(0x38));
        sw(c, this, 0x40, 0);
        sw(c, this, 0x44, 0);
        loop {
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = lbu(c, c.g[A2][0] as u32, 0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            c.g[V0][0] = ((c.g[A1][0] as i64) < 8) as u64;
            let more = c.g[V0][0] != 0;
            Addr(c.g[A0][0] as u32).sb(c, c.g[V1][0]);
            if more {
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0034_3080);
                }
                continue;
            }
            c.cycles += 13;
            c.g[A1][0] = lw(c, this, 0x18);
            c.g[A0][0] = 0xffff_ffff_ffff_0000;
            c.g[A0][0] = c.g[A0][0] | 0xfffd;
            c.g[V0][0] = sx(this);
            c.g[V1][0] = lw(c, c.g[A1][0] as u32, 8);
            frame.restore(c, 16, 0x30);
            c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
            frame.restore(c, 17, 0x20);
            c.g[V1][0] = c.g[V1][0] | 4;
            frame.restore_ra(c, 0x10);
            sw(c, c.g[A1][0] as u32, 8, c.g[V1][0]);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// `CrowdMan3d_ctor` (0x00343a68).
//
// `void CrowdMan3d::CrowdMan3d(self)`: initialise the four crowd groups and
// their slot records and reset each group.

pub fn crowd_man3d_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 4;
        frame.save(c, 16, 0x50);
        frame.save(c, 19, 0x20);
        c.g[S0][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, 20, 0x10);
        let this = c.g[A0][0];
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        c.g[S0][0] = c.g[S0][0] | 0xffff;
        frame.save_ra(c, 0);
        c.g[S3][0] = this;
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        let mut a0 = c.g[A0][0];
        let mut v0: u32 = 4;
        loop {
            c.cycles += 6;
            c.g[S2][0] = sx(v0.wrapping_sub(1));
            sw(c, a0 as u32, 4, c.g[S0][0]);
            c.g[S1][0] = sx((a0 as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((a0 as u32).wrapping_add(0xc));
            c.g[V1][0] = 0x1f;
            let mut v0r = c.g[V0][0] as u32;
            loop {
                c.cycles += 7;
                sw(c, v0r, 0, c.g[S0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                let t = c.g[V1][0] != c.g[S4][0];
                v0r = v0r.wrapping_add(8);
                c.g[V0][0] = sx(v0r);
                if !t {
                    break;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0034_3ab0);
                }
            }
            c.cycles += 5;
            call_ptr(c, 0x0034_42e0, 0x0034_3ad4)?;
            c.g[V0][0] = c.g[S2][0];
            let t = c.g[V0][0] != c.g[S4][0];
            c.g[A0][0] = c.g[S1][0];
            if !t {
                break;
            }
            v0 = c.g[V0][0] as u32;
            a0 = c.g[A0][0];
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0034_3a98);
            }
        }
        c.cycles += 4;
        sw(c, gp(c, 0xf00), 0, c.g[S3][0]);
        c.g[S0][0] = c.g[S3][0];
        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x9c4));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0034_42e0, 0x0034_3af8)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S3][0];
            if !more {
                break;
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0034_3af0);
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateRewardsRoom_OnWidgetAction` (0x001d4ea8).
//
// `int cFEStateRewardsRoom::OnWidgetAction(self, widget, action)`: route the
// rewards-room buttons.

pub fn fe_state_rewards_room_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {

        'b_1d4f5c: {
        'b_1d4f58: {
        c.cycles += 6;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[V1][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        let t = c.g[V1][0] == 0u64;
        c.g[S0][0] = c.g[A0][0];
        if t {
        break 'b_1d4f58;
        } else {
        'b_1d4f24: {
        'b_1d4ee8: {
        c.cycles += 3;
        c.g[V0][0] = 0x5;
        let t = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
        if t {
        break 'b_1d4ee8;
        } else {
        c.cycles += 2;
        if c.g[V0][0] != 0u64 {
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        break 'b_1d4f5c;
        } else {
        c.cycles += 3;
        c.g[V0][0] = 0x6;
        if c.g[A2][0] == c.g[V0][0] {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        break 'b_1d4f24;
        } else {
        c.cycles += 2;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        if t {
        break 'b_1d4f5c;
        } else {
        break 'b_1d4ee8;
        }
        }
        }
        }
        }
        c.cycles += 15;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[A3][0] = c.lw((c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = vslot(c, c.g[V1][0], 0x20, V0);
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x001d4f0c);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x001d4f0c)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d4f1c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039f400, 0x001d4f1c)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        if t {
        break 'b_1d4f5c;
        } else {
        break 'b_1d4f24;
        }
        }
        c.cycles += 10;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[A3][0] = c.lw((c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = vslot(c, c.g[V1][0], 0x28, V0);
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x001d4f44);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x001d4f44)?;
        let t = c.g[V0][0] == 0u64;
        c.g[A1][0] = c.g[V0][0];
        if t {
        break 'b_1d4f58;
        } else {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x001d4f58);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039f400, 0x001d4f58)?;
        break 'b_1d4f58;
        }
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        break 'b_1d4f5c;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ReplayStream_ReadObjectRef` (0x0026eb48).
//
// `void* ReplayStream::ReadObjectRef(self)`: read an object reference from
// the stream and resolve it against the current layer's object table.

pub fn replay_stream_read_object_ref(c: &mut Cpu) {
    guest(c, |c| {

        'b_26ebf8: {
        'b_26ebf4: {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0xffffffffffff0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x4;
        c.g[S0][0] = 0u64;
        c.g[A3][0] = c.lw((c.g[A0][0] as u32));
        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
        c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0026eb80);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        call_ptr(c, t, 0x0026eb80)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
        c.g[V0][0] = !(0u64 | c.g[A1][0]);
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if t {
        break 'b_26ebf4;
        } else {
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[RA][0] = sx(0x0026eb9c);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
        call_ptr(c, 0x003512c0, 0x0026eb9c)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] != 0u64;
        c.g[V0][0] = c.g[S0][0];
        if t {
        break 'b_26ebf8;
        } else {
        'b_26ebf0: {
        'b_26ebec: {
        c.cycles += 10;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        let t = c.g[V1][0] == 0u64;
        c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
        if t {
        break 'b_26ebec;
        } else {
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if t {
        break 'b_26ebf0;
        } else {
        break 'b_26ebec;
        }
        }
        }
        c.cycles += 1;
        c.g[V0][0] = 0u64;
        break 'b_26ebf0;
        }
        c.cycles += 1;
        c.g[S0][0] = c.g[V0][0];
        break 'b_26ebf4;
        }
        }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S0][0];
        break 'b_26ebf8;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIText_GetTextSize` (0x003a1030).
//
// `void UIText::GetTextSize(self, V2* out)`: return the measured text size,
// from the layout object when it is available and from the font otherwise.

pub fn ui_text_get_text_size(c: &mut Cpu) {
    guest(c, |c| {

        'b_3a1078: {
        'b_3a1064: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb4));
        let t = c.g[V0][0] == 0u64;
        c.g[S1][0] = c.g[A1][0];
        if t {
        break 'b_3a1064;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_3a1078;
        } else {
        break 'b_3a1064;
        }
        }
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot(c, c.g[V0][0], 0x80, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x003a1078);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x003a1078)?;
        break 'b_3a1078;
        }
        'b_3a10dc: {
        'b_3a10c0: {
        c.cycles += 5;
        c.g[RA][0] = sx(0x003a1080);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003a04f0, 0x003a1080)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb4));
        if c.g[A1][0] == 0u64 {
        c.g[V0][0] = 0x500000;
        break 'b_3a10c0;
        } else {
        'b_3a10bc: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[V0][0];
        if t {
        break 'b_3a10bc;
        } else {
        c.cycles += 10;
        c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x54));
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[A2][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003a10a8);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x003921f0, 0x003a10a8)?;
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.swc1(1, (c.g[S1][0] as u32));
        let t = true;
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
        if t {
        break 'b_3a10dc;
        } else {
        break 'b_3a10bc;
        }
        }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x500000;
        break 'b_3a10c0;
        }
        }
        c.cycles += 7;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
        c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
        c.ldr(3, (c.g[A2][0] as u32));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x7));
        c.sdr(3, (c.g[S1][0] as u32));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        break 'b_3a10dc;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateBuyAttrib_OnWidgetCreated` (0x001f4910).
//
// `int cFEStateBuyAttrib::OnWidgetCreated(self, UIWidget* w)`: install the
// created widget into the state's slots by its id.

pub fn fe_state_buy_attrib_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {

        'b_1f49bc: {
        'b_1f4964: {
        c.cycles += 13;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = 0x470000;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x001f493c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xff8));
        call_ptr(c, 0x00317670, 0x001f493c)?;
        if c.g[S0][0] != c.g[V0][0] {
        c.g[A0][0] = 0x470000;
        break 'b_1f4964;
        } else {
        c.cycles += 8;
        c.g[A0][0] = 0x470000;
        c.g[RA][0] = sx(0x001f4950);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfe0));
        call_ptr(c, 0x00317670, 0x001f4950)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001f495c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a1000, 0x001f495c)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1f49bc;
        } else {
        break 'b_1f4964;
        }
        }
        }
        'b_1f49b8: {
        'b_1f4980: {
        c.cycles += 5;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x001f4970);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfc0));
        call_ptr(c, 0x00317670, 0x001f4970)?;
        if c.g[S0][0] != c.g[V0][0] {
        c.g[A0][0] = 0x4a0000;
        break 'b_1f4980;
        } else {
        c.cycles += 2;
        let t = true;
        c.sw((c.g[S2][0] as u32).wrapping_add(0xac), c.g[S1][0]);
        if t {
        break 'b_1f49b8;
        } else {
        break 'b_1f4980;
        }
        }
        }
        'b_1f499c: {
        c.cycles += 5;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x001f498c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2320));
        call_ptr(c, 0x00317670, 0x001f498c)?;
        if c.g[S0][0] != c.g[V0][0] {
        c.g[A0][0] = 0x4a0000;
        break 'b_1f499c;
        } else {
        c.cycles += 2;
        let t = true;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
        if t {
        break 'b_1f49b8;
        } else {
        break 'b_1f499c;
        }
        }
        }
        c.cycles += 5;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x001f49a8);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2328));
        call_ptr(c, 0x00317670, 0x001f49a8)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1f49bc;
        } else {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        break 'b_1f49b8;
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_1f49bc;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `UIStateMgr_SwitchState` (0x0039f400).
//
// `void UIStateMgr::SwitchState(UIStateMgr* m, UIListNode* n)`: push the new
// state node in front of the current one and patch its visibility bits.

pub fn ui_state_mgr_switch_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        let mgr = c.g[A0][0];
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        let node = c.g[A1][0];
        c.g[S1][0] = mgr;
        c.g[S0][0] = node;
        call_ptr(c, 0x0039_7840, 0x0039_f420)?;
        c.g[A0][0] = c.g[V0][0];
        let a0 = c.g[A0][0];
        c.g[S2][0] = 0;
        let mut v0;
        if a0 == 0 {
            v0 = c.g[V0][0];
        } else {
            c.cycles += 5;
            let v1 = lw(c, a0 as u32, 0x1c);
            c.g[V1][0] = v1;
            c.g[V0][0] = sx((((v1 as u32) as i32) >> 5) as u32);
            c.g[V0][0] = c.g[V0][0] & 1;
            let clear5 = c.g[V0][0] == 0;
            c.g[V0][0] = sx((((v1 as u32) as i32) >> 6) as u32);
            if clear5 {
                v0 = c.g[V0][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 1;
                let set6 = c.g[V0][0] != 0;
                c.g[V0][0] = sx((v1 as u32) >> 8);
                if set6 {
                    v0 = c.g[V0][0];
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
                    let big = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                    if big {
                        v0 = c.g[V0][0];
                    } else {
                        c.cycles += 4;
                        c.g[S2][0] = 1;
                        c.g[V0][0] = v1 & c.g[V0][0];
                        c.g[V0][0] = c.g[V0][0] | 0x680;
                        sw(c, a0 as u32, 0x1c, c.g[V0][0]);
                        v0 = c.g[V0][0];
                    }
                }
            }
        }
        // b_39f48c / b_39f498.
        c.cycles += 2;
        let s2 = c.g[S2][0] == 0;
        c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
        if s2 {
            c.cycles += 3;
            c.g[V0][0] = lw(c, node as u32, 0x1c);
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x220;
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw(c, node as u32, 0x1c);
            c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x100;
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        }
        let _ = v0;
        c.cycles += 10;
        sw(c, node as u32, 0x1c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0039_78f0, 0x0039_f4a8)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ConsoleClock_FormatTime` (0x002c6f78).
//
// `void ConsoleClock::FormatTime(char* out, int pad, int h, int m, int s,
//  int mode)`: format the clock into `out`.

pub fn console_clock_format_time(c: &mut Cpu) {
    guest(c, |c| {

        'b_2c701c: {
        'b_2c7018: {
        'b_2c6fcc: {
        'b_2c6fb4: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = c.g[A2][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[A2][0] = c.g[A3][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        c.g[A3][0] = c.g[T0][0];
        c.g[S0][0] = c.g[A1][0];
        let t = c.g[T2][0] == 0u64;
        c.g[T0][0] = c.g[T1][0];
        if t {
        break 'b_2c6fb4;
        } else {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        let t = c.g[T2][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if t {
        break 'b_2c6fcc;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[A2][0] = c.g[S1][0];
        if t {
        break 'b_2c701c;
        } else {
        break 'b_2c6fb4;
        }
        }
        }
        }
        c.cycles += 6;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x002c6fc4);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6500));
        call_ptr(c, 0x002c27c0, 0x002c6fc4)?;
        let t = true;
        c.g[A0][0] = c.g[S0][0];
        if t {
        break 'b_2c7018;
        } else {
        break 'b_2c6fcc;
        }
        }
        'b_2c6fec: {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < 12i64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[SP][0];
        if t {
        break 'b_2c6fec;
        } else {
        c.cycles += 5;
        c.g[A1][0] = 0x480000;
        c.g[RA][0] = sx(0x002c6fe4);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6510));
        call_ptr(c, 0x002c27c0, 0x002c6fe4)?;
        let t = true;
        c.g[A0][0] = c.g[S0][0];
        if t {
        break 'b_2c7018;
        } else {
        break 'b_2c6fec;
        }
        }
        }
        'b_2c7000: {
        c.cycles += 5;
        c.g[V0][0] = 0xd;
        c.g[A1][0] = 0x480000;
        c.div(0, 6, 2);
        if c.g[V0][0] == 0u64 {
        break 'b_2c7000;
        } else {
        break 'b_2c7000;
        }
        }
        c.cycles += 6;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6528));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.hi[0];
        c.g[RA][0] = sx(0x002c7014);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xc));
        call_ptr(c, 0x002c27c0, 0x002c7014)?;
        c.g[A0][0] = c.g[S0][0];
        break 'b_2c7018;
        }
        c.cycles += 1;
        c.g[A2][0] = c.g[S1][0];
        break 'b_2c701c;
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x002c7024);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x002c25b8, 0x002c7024)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t; return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `GameCamera_ResetAll` (0x0015db58).
//
// `void GameCamera_ResetAll()`: reset the chase camera state of both camera
// slots of the race manager.

pub fn game_camera_reset_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = 0x4c0000;
        frame.save(c, 17, 0x10);
        c.g[S0][0] = sx(0x4c0000u32.wrapping_add(0x5830));
        frame.save_ra(c, 0);
        call_ptr(c, 0x0016_c968, 0x0015_db78)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_d1b8, 0x0015_db80)?;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = lw(c, c.g[V0][0] as u32, 0x84);
        let s1 = lw(c, c.g[V1][0] as u32, 0x84) as u32;
        c.g[S1][0] = sx(s1);
        c.g[V0][0] = lw(c, s1, 4);
        c.g[S0][0] = lw(c, s1, 0);
        c.g[A0][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        call_ptr(c, 0x0016_2170, 0x0015_db9c)?;
        c.g[V0][0] = lw(c, s1, 4);
        c.g[A1][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        c.g[V1][0] = lw(c, c.g[A1][0] as u32, 0x14);
        c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x20);
        let target = lw(c, c.g[V1][0] as u32, 0x24);
        let t = target as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0015_dbb8)?;
        c.g[V1][0] = lw(c, s1, 4);
        c.g[A0][0] = 1;
        c.g[S0][0] = ((c.g[S0][0] as i64) < 2) as u64;
        sw(c, c.g[V1][0] as u32, 0x460, 0);
        c.g[V0][0] = lw(c, s1, 4);
        sw(c, c.g[V0][0] as u32, 0x4a4, c.g[A0][0]);
        if c.g[S0][0] != 0 {
            c.cycles += 5;
            frame.restore(c, 16, 0x20);
            frame.restore(c, 17, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 12;
        c.g[V0][0] = lw(c, s1, 8);
        c.g[A0][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        call_ptr(c, 0x0016_2170, 0x0015_dbe0)?;
        c.g[V0][0] = lw(c, s1, 8);
        c.g[A2][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        c.g[V1][0] = lw(c, c.g[A2][0] as u32, 0x14);
        c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x20);
        let target = lw(c, c.g[V1][0] as u32, 0x24);
        let t = target as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0015_dbfc)?;
        c.g[V0][0] = lw(c, s1, 8);
        sw(c, c.g[V0][0] as u32, 0x460, 0);
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Avalanche_FreeHistories` (0x002d9a80).
//
// `void Avalanche::FreeHistories()`: stop every avalanche that has a history
// and release the history links.

pub fn avalanche_free_histories(c: &mut Cpu) {
    guest(c, |c| {

        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0x540000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x76c8));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[V0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c0));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        'l_2d9aa8: loop {
        'b_2d9abc: {
        c.cycles += 2;
        if c.g[V0][0] == 0u64 {
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        break 'b_2d9abc;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x002d9ab8);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002d81b0, 0x002d9ab8)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        break 'b_2d9abc;
        }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
        if c.g[V0][0] != 0u64 {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x002d9aa8); }
        continue 'l_2d9aa8;
        } else {
        'b_2d9b30: {
        'b_2d9b2c: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c8));
        let t = c.g[V0][0] == 0u64;
        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
        if t {
        break 'b_2d9b2c;
        } else {
        c.cycles += 2;
        let t = c.g[S1][0] == 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        if t {
        break 'b_2d9b30;
        } else {
        c.cycles += 1;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        'l_2d9ae0: loop {
        'b_2d9b24: {
        c.cycles += 2;
        if c.g[S0][0] == 0u64 {
        c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        break 'b_2d9b24;
        } else {
        c.cycles += 2;
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xf3));
        'l_2d9af0: loop {
        'b_2d9b10: {
        c.cycles += 2;
        if c.g[V0][0] == 0u64 {
        c.sb((c.g[S0][0] as u32).wrapping_add(0xf3), 0u64);
        break 'b_2d9b10;
        } else {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xfc));
        if c.g[A0][0] == 0u64 {
        c.sb((c.g[S0][0] as u32).wrapping_add(0xf3), 0u64);
        break 'b_2d9b10;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x002d9b0c);
        call_ptr(c, 0x00317e98, 0x002d9b0c)?;
        c.sb((c.g[S0][0] as u32).wrapping_add(0xf3), 0u64);
        break 'b_2d9b10;
        }
        }
        }
        c.cycles += 4;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xfc), 0u64);
        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf4));
        if c.g[S0][0] != 0u64 {
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xf3));
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x002d9af0); }
        continue 'l_2d9af0;
        } else {
        c.cycles += 1;
        c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        break 'b_2d9b24;
        }
        }
        }
        }
        c.cycles += 2;
        if c.g[S1][0] != 0u64 {
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x002d9ae0); }
        continue 'l_2d9ae0;
        } else {
        break 'b_2d9b2c;
        }
        }
        }
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_2d9b30;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());
        }
        }

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Avalanche_DestroyAll` (0x002d9b40).
//
// `void Avalanche::DestroyAll()`: free every avalanche's node list and clear
// the manager's tables.

pub fn avalanche_destroy_all(c: &mut Cpu) {
    guest(c, |c| {

        'b_2d9ba8: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x002d9b5c);
        call_ptr(c, 0x002d9a80, 0x002d9b5c)?;
        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
        let t = c.g[S1][0] == 0u64;
        c.g[A0][0] = 0x540000;
        if t {
        break 'b_2d9ba8;
        } else {
        c.cycles += 2;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        'l_2d9b70: loop {
        'b_2d9b90: {
        c.cycles += 2;
        let t = c.g[S0][0] == 0u64;
        c.g[S2][0] = c.g[S1][0];
        if t {
        break 'b_2d9b90;
        } else {
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        'l_2d9b80: loop {
        c.cycles += 4;
        c.g[RA][0] = sx(0x002d9b88);
        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf4));
        call_ptr(c, 0x00317e50, 0x002d9b88)?;
        let t = c.g[S0][0] != 0u64;
        c.g[A0][0] = c.g[S0][0];
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x002d9b80); }
        continue 'l_2d9b80;
        } else {
        break 'b_2d9b90;
        }
        }
        }
        }
        c.cycles += 5;
        c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[RA][0] = sx(0x002d9b9c);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x00317e50, 0x002d9b9c)?;
        if c.g[S1][0] != 0u64 {
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x002d9b70); }
        continue 'l_2d9b70;
        } else {
        c.cycles += 1;
        c.g[A0][0] = 0x540000;
        break 'b_2d9ba8;
        }
        }
        }
        }
        'b_2d9be8: {
        'b_2d9be4: {
        c.cycles += 9;
        c.sw((c.g[GP][0] as u32).wrapping_add(0x9c0), 0u64);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7bb0));
        c.sw((c.g[GP][0] as u32).wrapping_add(0x9d0), 0u64);
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x002d9bc0);
        c.g[A2][0] = 0x400;
        call_ptr(c, 0x00416210, 0x002d9bc0)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c4));
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c8));
        if t {
        break 'b_2d9be4;
        } else {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_2d9be8;
        } else {
        c.cycles += 4;
        c.g[RA][0] = sx(0x002d9bdc);
        call_ptr(c, 0x00317e98, 0x002d9bdc)?;
        c.sw((c.g[GP][0] as u32).wrapping_add(0x9c4), 0u64);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x9c8), 0u64);
        break 'b_2d9be4;
        }
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_2d9be8;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.sw((c.g[GP][0] as u32).wrapping_add(0x9cc), 0u64);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, elf_image, scramble, Rng, DATA, GAME_GP};

                        /// First size-class pool region (8-byte blocks): freeds must land here.
    const POOL0: u32 = DATA + 0x40_0000;

    /// The engine's size-class pool allocator: the "pools ready" flag, seven
    /// class regions with a free-list head past their blocks, and a short
    /// free chain inside each region.
    fn pool8(c: &mut Cpu, _hint: u32) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        c.bus.write32(GAME_GP + 0xdd8, 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x40_0000 + 0x4_0000 * i as u32;
            for n in 0..7u32 {
                let node = pool + 0x100 + 0x100 * n;
                c.bus.write32(node, if n == 6 { 0 } else { node + 0x100 });
            }
            c.bus.write32(pool + head, pool + 0x100);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    fn slot(c: &mut Cpu, vt: u32, off: u32, f: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, f);
    }

        #[test]
    fn camera_trigger_man_pop_trigger_stack_matches() {
        check(ee::s02::f_0016d260__generated, camera_trigger_man_pop_trigger_stack, 150, &|c, seed| {
            let mut r = Rng(0x0016_d260 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let stack = DATA;
            let count = (seed % 4) as u32;
            c.bus.write32(stack, count);
            c.bus.write32(stack + 4, r.next_u32());
            for i in 0..4 {
                c.bus.write32(stack + 8 + 4 * i, (r.next_u32() % 4) as u32);
            }
            c.g[A0][0] = sx(stack);
            c.g[A1][0] = (r.next_u32() % 5) as u64;
            c.g[A2][0] = sx(DATA + 0x100);
            c.g[A3][0] = sx(DATA + 0x200);
        });
    }

            #[test]
    fn ui_text_refresh_text_matches() {
        check(ee::s13::f_003a0528__generated, ui_text_refresh_text, 100, &|c, seed| {
            let mut r = Rng(0x003a_0528 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let this = DATA;
            c.bus.write32(this + 0xb0, if seed % 3 == 0 { 0 } else { 1 });
            let a = DATA + 0x100;
            let b = DATA + 0x200;
            let d = DATA + 0x300;
            c.bus.write32(this + 0x5c, a);
            c.bus.write32(a + 0xd0, b);
            c.bus.write32(b + 0x10, d);
            c.bus.write32(d + 0x10, 0); // hit the "no owner" early return
            c.g[A0][0] = sx(this);
        });
    }

                    #[test]
    fn fe_state_stats_format_time_matches() {
        check(ee::s03::f_001c4f58__generated, fe_state_stats_format_time, 80, &|c, seed| {
            let mut r = Rng(0x001c_4f58 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x590c, (1.0f32 / 60.0).to_bits());
            c.g[A1][0] = sx(DATA);
            c.g[A2][0] = ((r.next_u32() % 200_000) as i32) as u64;
        });
    }

                        #[test]
    fn setup_man_host_recv_cancel_matches() {
        check(ee::s07::f_00265398__generated, setup_man_host_recv_cancel, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x2653, 8);
            let pool = DATA + 0x30_0000;
            pool8(c, pool);
            let s0 = DATA + 0x1000;
            c.bus.write32(GAME_GP - 0xc8, s0);
            c.bus.write32(s0 + 0xf0, DATA + 0x2000);
            c.bus.write32(DATA + 0x2000 + 4, DATA + 0x3000);
            c.bus.write32(DATA, [0u32, 3][(seed % 2) as usize]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x400);
        });
    }

                    #[test]
    fn fe_popup_confirm_measure_line_matches() {
        check(ee::s03::f_001c84a8__generated, fe_popup_confirm_measure_line, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1c84, 4);
            let metrics = DATA + 0x100;
            let out = DATA + 0x200;
            for k in 0..4 {
                c.bus.write32(metrics + 0x30 + 4 * k, r.edge_f32());
                c.bus.write32(metrics + 0x50 + 4 * k, r.edge_f32());
                c.bus.write32(out + 4 * k, r.edge_f32());
            }
            c.bus.write32(metrics + 0x14, r.next_u32());
            c.bus.write32(metrics + 0x58, r.next_u32());
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(metrics);
            c.g[A2][0] = sx(out);
            c.g[A3][0] = sx(DATA + 0x300);
        });
    }

    #[test]
    fn fe_popup_confirm_measure_line_w_matches() {
        check(ee::s03::f_001c8568__generated, fe_popup_confirm_measure_line_w, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1c85, 4);
            let metrics = DATA + 0x100;
            let out = DATA + 0x200;
            for k in 0..4 {
                c.bus.write32(metrics + 0x30 + 4 * k, r.edge_f32());
                c.bus.write32(metrics + 0x50 + 4 * k, r.edge_f32());
                c.bus.write32(out + 4 * k, r.edge_f32());
            }
            c.bus.write32(metrics + 0x14, r.next_u32());
            c.bus.write32(metrics + 0x58, r.next_u32());
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(metrics);
            c.g[A2][0] = sx(out);
            c.g[A3][0] = sx(DATA + 0x300);
        });
    }

                    #[test]
    fn ssx_audio_system_on_replay_begin_matches() {
        check(ee::s07::f_0028f200__generated, ssx_audio_system_on_replay_begin, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x28f2, 6);
            elf_image(c);
            let this = DATA;
            c.bus.write32(this + 0x530, [0u32, 1][(seed % 2) as usize]);
            c.bus.write32(this + 0x118, DATA + 0x2000);
            c.bus.write32(this + 0x62ac, r.next_u32());
            c.bus.write32(this + 0x508, r.next_u32());
            c.g[A0][0] = sx(this);
        });
    }

                    #[test]
    fn bolts_index_table3_by_char_matches() {
        check(ee::s01::f_0014c3c8__generated, bolts_index_table3_by_char, 40, &|c, seed| {
            let mut r = Rng(0x0014_c3c8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let this = DATA;
            c.bus.write32(this + 0x24, (seed % 4) as u32);
            c.bus.write32(this + 0xc, DATA + 0x1000);
            for i in 0..4 {
                c.bus.write32(DATA + 0x1000 + 8 * i, r.next_u32());
                c.bus.write8(DATA + 0x1000 + 8 * i, (r.next_u32() % 5) as u8);
            }
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn bolts_index_table4_by_char_matches() {
        check(ee::s01::f_0014c488__generated, bolts_index_table4_by_char, 40, &|c, seed| {
            let mut r = Rng(0x0014_c488 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let this = DATA;
            c.bus.write32(this + 0x28, (seed % 4) as u32);
            c.bus.write32(this + 0x10, DATA + 0x1000);
            for i in 0..4 {
                c.bus.write32(DATA + 0x1000 + 8 * i, r.next_u32());
                c.bus.write8(DATA + 0x1000 + 8 * i, (r.next_u32() % 5) as u8);
            }
            c.g[A0][0] = sx(this);
        });
    }

        #[test]
    fn conveyor_ctor_matches() {
        check(ee::s11::f_00343010__generated, conveyor_ctor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x3430, 4);
            let this = DATA;
            let params = DATA + 0x1000;
            let dir = DATA + 0x2000;
            for k in 0..4 {
                c.bus.write32(dir + 4 * k, r.edge_f32());
                c.bus.write32(dir + 0x10 + 4 * k, (r.next_u32() % 256) as u32);
            }
            c.bus.write32(this + 0x18, DATA + 0x3000);
            c.bus.write32(DATA + 0x3000 + 8, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = sx(params);
            c.g[A3][0] = sx(dir);
        });
    }

    #[test]
    fn crowd_man3d_ctor_matches() {
        check(ee::s11::f_00343a68__generated, crowd_man3d_ctor, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x343a, 4);
            let this = DATA;
            for k in 0..4 {
                c.bus.write32(this + 0x1f4 * k + 0x18, DATA + 0x8000);
            }
            c.g[A0][0] = sx(this);
            c.bus.write32(GAME_GP + 0xf00, r.next_u32());
        });
    }

                    #[test]
    fn ui_state_mgr_switch_state_matches() {
        check(ee::s13::f_0039f400__generated, ui_state_mgr_switch_state, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x39f4, 4);
            let mgr = DATA;
            let node = DATA + 0x1000;
            c.bus.write32(mgr, mgr);
            c.bus.write32(mgr + 4, node);
            c.bus.write32(node + 0x1c, r.next_u32());
            c.bus.write32(node + 4, mgr);
            c.g[A0][0] = sx(mgr);
            c.g[A1][0] = sx(node);
        });
    }

        #[test]
    fn game_camera_reset_all_matches() {
        check(ee::s01::f_0015db58__generated, game_camera_reset_all, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x15db, 8);
            let w = DATA + 0x1000;
            let x = DATA + 0x2000;
            let rm = DATA + 0x3000;
            c.bus.write32(GAME_GP - 0x848, w);
            c.bus.write32(w + 0x84, x);
            c.bus.write32(x + 0x84, rm);
            c.bus.write32(rm, [0u32, 1, 2][(seed % 3) as usize]);
            c.bus.write32(rm + 4, DATA + 0x4000);
            c.bus.write32(rm + 8, DATA + 0x5000);
            for cam in [DATA + 0x4000, DATA + 0x5000] {
                c.bus.write32(cam + 0xa8, DATA + 0x6000);
                c.bus.write32(DATA + 0x6000 + 0x14, DATA + 0x7000);
                c.bus.write16(DATA + 0x7000 + 0x20, 0);
                c.bus.write32(DATA + 0x7000 + 0x24, FAKE);
            }
            c.g[A0][0] = sx(DATA);
            c.bus.write32(0x004e_5830, r.next_u32());
        });
    }

    /// Compare against the generated body and require *everything* to match:
    /// hard state, caller-saved registers (`soft`) and LO/HI.
    fn strict(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = crate::difftest::compare(g, h, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    #[test]
    fn tag_field_set_binary_matches() {
        strict(ee::s14::f_003ec300__generated, tag_field_set_binary, 120, &|c, seed| {
            let mut r = Rng(0x003e_c300 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            let record = DATA;
            c.bus.write32(record, 0);
            let name = DATA + 0x100;
            let len = if seed % 3 == 0 { 0 } else { 1 + (r.next_u32() % 6) as usize };
            for i in 0..len {
                c.bus.write8(name + i as u32, b'A' + (r.next_u32() % 26) as u8);
            }
            c.bus.write8(name + len as u32, 0);
            c.bus.write8(name + len as u32 + 1, b'Q');
            for i in 0..16u32 {
                c.bus.write8(DATA + 0x200 + i, (r.next_u32() % 40 + 32) as u8);
            }
            let size: i32 = [0, 1, 3, -1][(seed / 2 % 4) as usize];
            c.g[A0][0] = sx(record);
            c.g[A1][0] = 0;
            c.g[A2][0] = if seed % 3 == 0 { 0 } else { sx(name) };
            c.g[A3][0] = sx(DATA + 0x200);
            c.g[T0][0] = sx(size as u32);
        });
    }

    #[test]
    fn script_cam_controller_dtor_matches() {
        strict(ee::s02::f_001690d0__generated, script_cam_controller_dtor, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x1690, 4);
            let this = DATA;
            let array = DATA + 0x400;
            let vt = DATA + 0x600;
            c.bus.write32(this + 0x1c, array);
            for i in 0..2u32 {
                let elem = DATA + 0x800 + 0x100 * i;
                c.bus.write32(array + 4 * i, elem);
                c.bus.write32(elem + 4, r.next_u32());
                c.bus.write32(elem + 0xc, vt);
            }
            c.bus.write16(vt + 8, 0);
            c.bus.write32(vt + 0xc, FAKE);
            let se = DATA + 0x1000;
            let mgr = DATA + 0x1400;
            c.bus.write32(GAME_GP + 0x3ec, se);
            c.bus.write32(se + 0x2a8, mgr);
            c.bus.write16(mgr + 0x18, 0);
            c.bus.write32(mgr + 0x1c, FAKE);
            c.bus.write32(this + 8, 0);
            c.bus.write32(this + 0x14, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed as u64) & !1;
        });
    }

    #[test]
    fn ui_engine_texture_set_dtor_matches() {
        strict(ee::s13::f_00398078__generated, ui_engine_texture_set_dtor, 120, &|c, seed| {
            let _r = machine(c, seed, 0x3980, 12);
            let pool = POOL0;
            let (this, array, extra) = (pool + 0x300, pool + 0x400, pool + 0x500);
            c.bus.write32(GAME_GP + 0x2a70, pool);
            c.bus.write32(pool + 0x800, 0);
            c.bus.write32(GAME_GP + 0xdd8, 1);
            let obj = DATA + 0x2000;
            let vt = DATA + 0x2100;
            c.bus.write16(vt + 0x190, 0);
            c.bus.write32(vt + 0x194, FAKE);
            c.bus.write32(GAME_GP - 0x854, obj);
            c.bus.write32(obj + 0x10d8, vt);
            c.bus.write32(this + 8, (seed % 4) as u32);
            c.bus.write32(this + 4, array);
            c.bus.write32(this + 0xc, if seed % 5 == 0 { 0 } else { extra });
            for i in 0..8u32 {
                c.bus.write32(array + 4 * i, DATA + 0x800 + 0x40 * i);
                c.bus.write32(extra + 4 * i, DATA + 0x900 + 0x40 * i);
            }
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn rewards_file_unload_matches() {
        strict(ee::s01::f_0015ae00__generated, rewards_file_unload, 120, &|c, seed| {
            let _r = machine(c, seed, 0x15ae, 8);
            let pool = DATA + 0x20_0000;
            c.bus.write32(GAME_GP + 0x2a70, pool);
            c.bus.write32(pool + 0x800, 0);
            c.bus.write32(GAME_GP + 0xdd8, 1);
            let this = DATA;
            c.bus.write32(this, 0xffff_ffff);
            c.bus.write32(this + 4, if seed % 2 == 0 { 0 } else { pool + 0x100 });
            c.bus.write32(GAME_GP - 0x848, DATA + 0x1000);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn ui_menu_render_matches() {
        strict(ee::s13::f_0039b6a0__generated, ui_menu_render, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x39b6, 2);
            let this = DATA;
            c.bus.write32(this + 0x9c, 0);
            let list = this + 0x74;
            let (node0, node1, sent, obj) =
                (DATA + 0x400u32, DATA + 0x500u32, DATA + 0x600u32, DATA + 0x700u32);
            if seed % 4 == 0 {
                c.bus.write32(list + 4, list);
            } else {
                c.bus.write32(list + 4, node0);
                c.bus.write32(node0 + 4, node1);
                c.bus.write32(node0 + 0x14, 0x40);
                c.bus.write32(node0 + 8, obj);
                c.bus.write32(node1 + 4, sent);
                c.bus.write32(node1 + 0x14, 0);
                c.bus.write32(sent + 4, sent);
                c.bus.write16(obj + 0x88, 0);
                c.bus.write32(obj + 0x8c, FAKE);
            }
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            c.g[A0][0] = sx(this);
        });
    }

        #[test]
    fn fe_state_peak_room_on_widget_notify_matches() {
        strict(ee::s04::f_001d3e08__generated, fe_state_peak_room_on_widget_notify, 120, &|c, seed| {
            let mut r = Rng(0x001d_3e08 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = [0u64, 5, 7, 8, 9, 10, 12][(seed % 7) as usize];
            c.g[A3][0] = sx(DATA + 0x200);
        });
    }

                #[test]
    fn net_fe_state_on_session_lost_popup_event_matches() {
        strict(ee::s03::f_001a9578__generated, net_fe_state_on_session_lost_popup_event, 40, &|c, seed| {
            let mut r = Rng(0x001a_9578 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0x1d98, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(GAME_GP - 0xc8, DATA + 0x1000);
            c.g[A0][0] = sx(DATA);
            c.g[A2][0] = [0u64, 0xf, 0x14, 3][(seed % 4) as usize];
        });
    }

    #[test]
    fn fe_use_existing_account_on_abort_popup_event_matches() {
        strict(ee::s03::f_001aca88__generated, fe_use_existing_account_on_abort_popup_event, 40, &|c, seed| {
            let mut r = Rng(0x001a_ca88 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0x238, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            c.bus.write32(GAME_GP - 0xc8, DATA + 0x1800);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = [0u64, 2, 0xf, 0x14][(seed % 4) as usize];
        });
    }

    #[test]
    fn fe_state_login_v33_matches() {
        strict(ee::s03::f_001bf378__generated, fe_state_login_v33, 40, &|c, seed| {
            let mut r = Rng(0x001b_f378 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP + 0x238, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            c.bus.write32(GAME_GP - 0xc8, DATA + 0x1800);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = [0u64, 2, 0xf, 0x14][(seed % 4) as usize];
        });
    }

    #[test]
    fn message_center_get_index_in_type_matches() {
        strict(ee::s04::f_001e2dc0__generated, message_center_get_index_in_type, 40, &|c, seed| {
            let mut r = Rng(0x001e_2dc0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.bus.write32(0x004c_6c08 + 4, 0);
            c.bus.write32(0x0044_1630 + 4, 0);
            c.bus.write32(0x0044_1630 + 8, 1);
            c.bus.write32(0x0044_1630 + 0xc, r.next_u32());
            c.g[A0][0] = 0;
        });
    }

    #[test]
    fn ov_screen_multiplayer_standings_init_matches() {
        strict(ee::s04::f_001e8f00__generated, ov_screen_multiplayer_standings_init, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x1e8f, 8);
            let this = DATA;
            c.bus.write32(this + 0x10, DATA + 0x1000);
            c.bus.write32(DATA + 0x1000 + 0x38, r.next_u32());
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn lobby_man_create_room_matches() {
        strict(ee::s06::f_0025dac0__generated, lobby_man_create_room, 30, &|c, seed| {
            let mut r = Rng(0x0025_dac0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(DATA, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = (r.next_u32() % 4) as u64;
            c.g[A3][0] = (r.next_u32() % 2) as u64;
        });
    }

        #[test]
    fn in_seq_event_track_start_matches() {
        strict(ee::s07::f_00273208__generated, in_seq_event_track_start, 30, &|c, seed| {
            let mut r = Rng(0x0027_3208 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let seq = DATA + 0x100;
            c.bus.write16(seq, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write16(seq + 2, (r.next_u32() % 4) as u16);
            c.bus.write16(seq + 6, (r.next_u32() % 4) as u16);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(seq);
            c.g[A2][0] = sx(DATA + 0x200);
            c.g[A3][0] = sx(DATA + 0x300);
        });
    }

    #[test]
    fn chase_cam_controller_begin_script_camera_matches() {
        strict(ee::s01::f_00161fa0__generated, chase_cam_controller_begin_script_camera, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x161f, 4);
            let this = DATA;
            c.bus.write32(this + 0x30, r.next_u32() & 7);
            c.bus.write32(this + 0x24, r.next_u32());
            c.bus.write32(this + 0x28, r.next_u32());
            c.bus.write32(this + 0x34, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = [0x4cu64, 0x5d, 0x42, 1][(seed % 4) as usize];
        });
    }

    #[test]
    fn mc_manager_on_operation_done_matches() {
        strict(ee::s06::f_0023ea90__generated, mc_manager_on_operation_done, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x23ea, 10);
            let this = DATA;
            c.bus.write32(this + 0xbc, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(this + 0x12c, (seed % 3) as u32);
            c.bus.write32(this + 0x118, (r.next_u32() % 2) as u32);
            c.bus.write32(this + 0x434, DATA + 0x1000);
            c.bus.write32(DATA + 0x1000, DATA + 0x2000);
            c.bus.write16(DATA + 0x2000 + 0x160, 0);
            c.bus.write32(DATA + 0x2000 + 0x164, FAKE);
            c.bus.write32(this + 0x748, DATA + 0x3000);
            c.bus.write16(DATA + 0x3000 + 8, 0);
            c.bus.write32(DATA + 0x3000 + 0xc, FAKE);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn mc_manager_state22_load_matches() {
        strict(ee::s06::f_0023ff00__generated, mc_manager_state22_load, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x23ff, 10);
            let this = DATA;
            c.bus.write32(this + 0x108, [0u32, 1, 5][(seed % 3) as usize]);
            c.bus.write32(this + 0x434, DATA + 0x1000);
            c.bus.write32(DATA + 0x1000, DATA + 0x2000);
            c.bus.write16(DATA + 0x2000 + 0x58, 0);
            c.bus.write32(DATA + 0x2000 + 0x5c, FAKE);
            c.bus.write16(DATA + 0x2000 + 0x248, 0);
            c.bus.write32(DATA + 0x2000 + 0x24c, FAKE);
            c.bus.write32(this + 0x748, DATA + 0x3000);
            c.bus.write16(DATA + 0x3000 + 8, 0);
            c.bus.write32(DATA + 0x3000 + 0xc, FAKE);
            c.bus.write32(this + 0x2c, FAKE);
            c.bus.write32(this + 0x28, FAKE);
            c.g[A0][0] = sx(this);
        });
    }

            #[test]
    fn bolt_file_cache_shutdown_matches() {
        strict(ee::s03::f_0019cf40__generated, bolt_file_cache_shutdown, 6, &|c, _seed| {
            let _r = machine(c, 0, 0x19cf, 8);
            c.bus.write32(GAME_GP - 0x1788, 0);
            c.g[A0][0] = sx(DATA);
        });
    }

        #[test]
    fn w_script_process_dtor_matches() {
        strict(ee::s10::f_00307738__generated, w_script_process_dtor, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x3077, 10);
            pool8(c, POOL0);
            let this = POOL0 + 0x400;
            let arr = POOL0 + 0x100;
            c.bus.write32(arr - 0x10, 1);
            c.bus.write32(arr, r.next_u32());
            c.bus.write32(this + 0x14, if seed % 3 == 0 { 0 } else { arr });
            c.bus.write32(this + 0x1c, if seed % 4 == 0 { 0 } else { DATA + 0x2000 });
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

                #[test]
    fn ui_scroll_set_parts_visible_matches() {
        strict(ee::s13::f_0039de68__generated, ui_scroll_set_parts_visible, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x39de, 12);
            let this = DATA;
            let vt = DATA + 0x1000;
            slot(c, vt, 0x48, FAKE);
            for (i, off) in [0x7c, 0x80, 0x84, 0x78, 0x74].into_iter().enumerate() {
                let obj = DATA + 0x2000 + 0x100 * i as u32;
                c.bus.write32(this + off, if (seed as usize >> i) & 1 == 0 { 0 } else { obj });
                c.bus.write32(obj + 8, vt);
            }
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(r.next_u32() & 1);
        });
    }

    #[test]
    fn fe_state_rewards_room_on_widget_action_matches() {
        strict(ee::s04::f_001d4ea8__generated, fe_state_rewards_room_on_widget_action, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x1d4e, 6);
            let this = DATA;
            let widget = DATA + 0x1000;
            c.bus.write32(this + 0x10, DATA + 0x2000);
            c.bus.write32(DATA + 0x2000, DATA + 0x3000);
            c.bus.write32(DATA + 0x3000 + 4, DATA + 0x4000);
            c.bus.write32(DATA + 0x4000 + 4, DATA + 0x5000);
            c.bus.write16(DATA + 0x5000 + 0x20, 0);
            c.bus.write32(DATA + 0x5000 + 0x24, FAKE);
            c.bus.write16(DATA + 0x5000 + 0x28, 0);
            c.bus.write32(DATA + 0x5000 + 0x2c, FAKE);
            c.bus.write32(widget + 0x18, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(widget) };
            c.g[A2][0] = [0u64, 1, 5, 6, 7][(seed % 5) as usize];
        });
    }

    #[test]
    fn replay_stream_read_object_ref_matches() {
        strict(ee::s07::f_0026eb48__generated, replay_stream_read_object_ref, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x26eb, 8);
            let this = DATA;
            let obj = DATA + 0x1000;
            c.bus.write32(this, obj);
            c.bus.write16(obj + 0x10, 0);
            c.bus.write32(obj + 0x14, FAKE);
            c.bus.write32(GAME_GP - 0x848, DATA + 0x2000);
            c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
            c.bus.write32(DATA + 0x3000 + 0x88, FAKE);
            c.bus.write32(GAME_GP + 0x16c8, DATA + 0x4000);
            c.bus.write32(DATA + 0x4000, DATA + 0x5000);
            c.bus.write32(DATA + 0x5000 + 8, DATA + 0x6000);
            c.g[A0][0] = sx(this);
            c.g[A2][0] = sx(r.next_u32() & 0xff);
        });
    }

    #[test]
    fn ui_text_get_text_size_matches() {
        strict(ee::s13::f_003a1030__generated, ui_text_get_text_size, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x3a10, 6);
            let this = DATA;
            c.bus.write32(this + 0xb4, if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(this + 0x14, r.next_u32());
            c.bus.write32(this + 8, DATA + 0x1000);
            c.bus.write16(DATA + 0x1000 + 0x80, 0);
            c.bus.write32(DATA + 0x1000 + 0x84, FAKE);
            c.bus.write32(this + 0x50, r.edge_f32());
            c.bus.write32(this + 0x54, r.edge_f32());
            c.bus.write32(DATA + 0x2000 + 8, r.next_u32());
            c.bus.write32(DATA + 0x2000, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(DATA + 0x200);
        });
    }

    #[test]
    fn fe_state_buy_attrib_on_widget_created_matches() {
        strict(ee::s04::f_001f4910__generated, fe_state_buy_attrib_on_widget_created, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1f49, 4);
            let this = DATA;
            let widget = DATA + 0x1000;
            c.bus.write32(widget + 0x38, if seed % 4 == 0 { 0 } else { r.next_u32() });
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(widget);
        });
    }

    #[test]
    fn avalanche_free_histories_matches() {
        strict(ee::s09::f_002d9a80__generated, avalanche_free_histories, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x2d9a, 8);
            let base = DATA + 0x1000;
            for k in 0..16 {
                c.bus.write32(base + 0x1c * k, (r.next_u32() % 2) as u32);
            }
            c.bus.write32(GAME_GP + 0x9c0, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
            c.bus.write32(GAME_GP + 0x9c8, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(DATA + 0x2000 + 4, DATA + 0x3000);
            c.bus.write32(DATA + 0x3000 + 0xf4, 0);
            c.bus.write32(DATA + 0x3000 + 0xfc, 0);
            c.bus.write8(DATA + 0x3000 + 0xf3, 1);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn avalanche_destroy_all_matches() {
        strict(ee::s09::f_002d9b40__generated, avalanche_destroy_all, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x2d9b, 10);
            pool8(c, POOL0);
            for k in 0..16 {
                c.bus.write32(DATA + 0x1c * k, (r.next_u32() % 2) as u32);
            }
            c.bus.write32(GAME_GP + 0x9c0, if seed % 3 == 0 { 0 } else { POOL0 + 0x600 });
            c.bus.write32(POOL0 + 0x600 + 4, POOL0 + 0x100);
            c.bus.write32(POOL0 + 0x100 + 0xf4, 0);
            c.bus.write32(GAME_GP + 0x9c4, if seed % 2 == 0 { 0 } else { POOL0 + 0x200 });
            c.bus.write32(GAME_GP + 0x9c8, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(GAME_GP + 0x9cc, r.next_u32());
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn console_clock_format_time_matches() {
        strict(ee::s08::f_002c6f78__generated, console_clock_format_time, 80, &|c, seed| {
            let mut r = Rng(0x002c_6f78 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A1][0] = sx(DATA);
            c.g[A2][0] = (r.next_u32() % 16) as u64;
            c.g[A3][0] = (r.next_u32() % 60) as u64;
            c.g[T0][0] = (r.next_u32() % 60) as u64;
            c.g[T1][0] = (r.next_u32() % 8) as u64;
            c.g[T2][0] = [0u64, 1, 2][(seed % 3) as usize];
        });
    }

        }
