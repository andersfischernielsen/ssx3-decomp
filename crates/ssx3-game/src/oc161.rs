//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for front-end state machine updates, BE (reward/economy)
//! helpers, replay/render helpers and audio system teardown, each with a
//! differential test against the generated body.
//!
//! Everything here is cold-path code: UI screens and popups, the recompiled
//! replay player, BELibrary reward bookkeeping, and destructors. The
//! replacements copy the generated body's block cycle charges, the values
//! left in `v0`/`v1` at return, the stack frame shape (save slots, offsets,
//! order) and the call protocol ([`crate::call`]); loops with a back edge
//! deliver events exactly where the original does.
//!
//! The control flow keeps the generated block labels (`'b_1cb9e0:`) so the
//! differential test can be read against the generated body line by line.

use crate::abi::r32;
use crate::call::{call_ptr, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001c_b9b0,
        c_keyboard_popup_move_down,
        "cKeyboardPopup_MoveDown",
    ),
    (
        0x0014_eca0,
        be_restore_player_setup,
        "BE_RestorePlayerSetup",
    ),
    (
        0x0019_a098,
        c_fe_state_char_equip_get_preview_texture,
        "cFEStateCharEquip_GetPreviewTexture",
    ),
    (
        0x0020_9370,
        ov_screen_session_update,
        "OVScreenSession_Update",
    ),
    (0x001f_22e8, hud_draw_text_elem_w, "HUD_DrawTextElemW"),
    (
        0x001e_1ad0,
        message_center_pick_foreshadow_message,
        "MessageCenter_PickForeshadowMessage",
    ),
    (0x0022_5ca0, luno_vm_register_read, "LunoVMRegister_Read"),
    (
        0x0012_c408,
        rider_gate_sub_update_launch,
        "RiderGateSub_UpdateLaunch",
    ),
    (0x0039_bbd8, ui_pair_set_focus, "UIPair_SetFocus"),
    (
        0x0015_7bf0,
        be_reward_int_is_peak_goal_complete,
        "BERewardInt_IsPeakGoalComplete",
    ),
    (0x0028_fea0, midi_banks_base_dtor, "MIDIBanks_BaseDtor"),(0x0035_8998, rail_man_enable_rail, "RailMan_EnableRail"),
    (
        0x0017_fbf0,
        debug_menu_module_render,
        "DebugMenuModule_Render",
    ),
    (
        0x0017_f2d0,
        debug_menu_module_update_cursor_trail,
        "DebugMenuModule_UpdateCursorTrail",
    ),
    (
        0x0023_ae00,
        gm_handler8_on_rider_finished,
        "GMHandler8_OnRiderFinished",
    ),
    (
        0x0027_5b98,
        data_instance_man_add_big_entries,
        "DataInstanceMan_AddBigEntries",
    ),
    (
        0x0020_9e78,
        ov_screen_session_update_indicators,
        "OVScreenSession_UpdateIndicators",
    ),
    (0x0027_80b8, script_engine_dtor, "ScriptEngine_dtor"),
    (
        0x0019_f908,
        fe_module_rider_preview_load_shape_textures,
        "FEModule_RiderPreviewLoadShapeTextures",
    ),(
        0x0010_e5d8,
        rider_handle_race_path_event,
        "Rider_HandleRacePathEvent",
    ),
    (
        0x001d_2f68,
        c_fe_state_mountain_room_on_widget_action,
        "cFEStateMountainRoom_OnWidgetAction",
    ),
    (0x0036_8660, tex_man_bind_textures, "TexMan_BindTextures"),
    (0x0035_7e80, multi_particle_ctor_2, "MultiParticle_ctor_2"),
    (
        0x0023_2348,
        pre_start_screen_update,
        "PreStartScreen_Update",
    ),
    (
        0x0015_9b08,
        be_reward_int_on_mission_completed,
        "BERewardInt_OnMissionCompleted",
    ),
    (
        0x0015_99a0,
        be_reward_int_on_collectible_collected,
        "BERewardInt_OnCollectibleCollected",
    ),
    (
        0x0027_aaf8,
        script_engine_queue_intro_scripts,
        "ScriptEngine_QueueIntroScripts",
    ),
    (
        0x0029_2b48,
        ssx_audio_system_stop_board_loops,
        "SSXAudioSystem_StopBoardLoops",
    ),
    (
        0x001f_f3b0,
        ov_screen_save_records_on_widget_action,
        "OVScreenSaveRecords_OnWidgetAction",
    ),
    (
        0x003b_1ab0,
        mpeg2_sequence_extension,
        "MPEG2_sequence_extension",
    ),
    (
        0x0034_59a8,
        particle_modifier_ctor_2,
        "ParticleModifier_ctor_2",
    ),(
        0x0016_9db0,
        target_script_camera_algo_update,
        "TargetScriptCameraAlgo_Update",
    ),
    (
        0x0016_9570,
        manual_script_camera_algo_update,
        "ManualScriptCameraAlgo_Update",
    ),
    (
        0x001b_b840,
        c_fe_state_create_step2_fill_day_list,
        "cFEStateCreateStep2_FillDayList",
    ),
    (
        0x0017_6890,
        camera_trigger_man_collect_octree_node_triggers,
        "CameraTriggerMan_CollectOctreeNodeTriggers",
    ),
    (0x0014_b560, be_bolt_int_unlock_bolt, "BEBoltInt_UnlockBolt"),
    (
        0x0036_4360,
        defer_man_emit_default_state,
        "DeferMan_EmitDefaultState",
    ),
    (
        0x001c_9210,
        c_fe_popup_confirm_get_text_box_size,
        "cFEPopupConfirm_GetTextBoxSize",
    ),(
        0x001c_9038,
        c_fe_popup_confirm_get_text_box_size_w,
        "cFEPopupConfirm_GetTextBoxSizeW",
    ),
    (
        0x0018_7a38,
        c_fe_state_mem_card_scan_step,
        "cFEStateMemCard_ScanStep",
    ),
    (0x002c_c0c0, menu_sys_update, "MenuSys_Update"),
    (0x0034_2880, boost_modifier_v02, "BoostModifier_v02"),
    (
        0x001f_85c0,
        ov_screen_pause_freeride_on_activate,
        "OVScreenPauseFreeride_OnActivate",
    ),
    (0x0034_15d0, boost_update, "Boost_Update"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

mod callee {
    use super::{ee, Callee};
    pub const C00113f88: Callee = Callee {
        addr: 0x00113f88,
        f: ee::s00::Rider_SetSlew3And4Targets,
    };
    pub const C00115ab0: Callee = Callee {
        addr: 0x00115ab0,
        f: ee::s00::Rider_GetGateChargeAngle,
    };
    pub const C0011fe78: Callee = Callee {
        addr: 0x0011fe78,
        f: ee::s00::Rider_SetMainState,
    };
    pub const C0011fe98: Callee = Callee {
        addr: 0x0011fe98,
        f: ee::s00::Rider_GetMainState,
    };
    pub const C0011fec8: Callee = Callee {
        addr: 0x0011fec8,
        f: ee::s00::Rider_SetSubState,
    };
    pub const C00125108: Callee = Callee {
        addr: 0x00125108,
        f: ee::s00::Rider_OnFinishRace,
    };
    pub const C0014a080: Callee = Callee {
        addr: 0x0014a080,
        f: ee::s01::BE_GetRiderCharacter,
    };
    pub const C0014a0e0: Callee = Callee {
        addr: 0x0014a0e0,
        f: ee::s01::BE_GetRiderProfile,
    };
    pub const C0014bdb8: Callee = Callee {
        addr: 0x0014bdb8,
        f: ee::s01::Bolts_GetDB,
    };
    pub const C0014d448: Callee = Callee {
        addr: 0x0014d448,
        f: ee::s01::Bolts_FindBoltIndex,
    };
    pub const C0014d988: Callee = Callee {
        addr: 0x0014d988,
        f: ee::s01::Bolts_GetCharBoltCount,
    };
    pub const C0014d998: Callee = Callee {
        addr: 0x0014d998,
        f: ee::s01::Bolts_GetCharBolts,
    };
    pub const C0014dc80: Callee = Callee {
        addr: 0x0014dc80,
        f: ee::s01::BELibrary_GetInstance,
    };
    pub const C0014dd58: Callee = Callee {
        addr: 0x0014dd58,
        f: ee::s01::BELibrary_GetInterface,
    };
    pub const C00150928: Callee = Callee {
        addr: 0x00150928,
        f: ee::s01::BEEconInt_GetMoney,
    };
    pub const C00150b48: Callee = Callee {
        addr: 0x00150b48,
        f: ee::s01::BEEconInt_SpendMoney,
    };
    pub const C00151c48: Callee = Callee {
        addr: 0x00151c48,
        f: ee::s01::BE_SetCharProfileBoltOwned,
    };
    pub const C001535c0: Callee = Callee {
        addr: 0x001535c0,
        f: ee::s01::BEMissionInt_CountCollectedOnPeak,
    };
    pub const C001543e0: Callee = Callee {
        addr: 0x001543e0,
        f: ee::s01::BEMissionInt_CountCompletedOnPeak,
    };
    pub const C00156988: Callee = Callee {
        addr: 0x00156988,
        f: ee::s01::BE_RestoreCharacter,
    };
    pub const C00156a60: Callee = Callee {
        addr: 0x00156a60,
        f: ee::s01::BERewardInt_IsBetterPlace,
    };
    pub const C001577e0: Callee = Callee {
        addr: 0x001577e0,
        f: ee::s01::BERewardInt_IsPeakFullyCompleted,
    };
    pub const C00157bf0: Callee = Callee {
        addr: 0x00157bf0,
        f: ee::s01::BERewardInt_IsPeakGoalComplete,
    };
    pub const C00158910: Callee = Callee {
        addr: 0x00158910,
        f: ee::s01::BERewardInt_GetEventBestPlace,
    };
    pub const C001589b0: Callee = Callee {
        addr: 0x001589b0,
        f: ee::s01::BERewardInt_GetCollectibleTier,
    };
    pub const C00158a50: Callee = Callee {
        addr: 0x00158a50,
        f: ee::s01::BERewardInt_GetMissionTier,
    };
    pub const C00158af0: Callee = Callee {
        addr: 0x00158af0,
        f: ee::s01::BERewardInt_HasPointsTier,
    };
    pub const C00158c80: Callee = Callee {
        addr: 0x00158c80,
        f: ee::s01::BERewardInt_GetNextCollectibleThreshold,
    };
    pub const C00158d58: Callee = Callee {
        addr: 0x00158d58,
        f: ee::s01::BERewardInt_GetNextMissionThreshold,
    };
    pub const C00159170: Callee = Callee {
        addr: 0x00159170,
        f: ee::s01::BERewardInt_GrantMedalReward,
    };
    pub const C00159cd0: Callee = Callee {
        addr: 0x00159cd0,
        f: ee::s01::BERewardInt_GrantReward,
    };
    pub const C0015a2e0: Callee = Callee {
        addr: 0x0015a2e0,
        f: ee::s01::BERewardInt_IsRewardGranted,
    };
    pub const C0016c6b0: Callee = Callee {
        addr: 0x0016c6b0,
        f: ee::s02::ActiveCameraTriggerList_Find,
    };
    pub const C0016c718: Callee = Callee {
        addr: 0x0016c718,
        f: ee::s02::ActiveCameraTriggerList_Add,
    };
    pub const C00176890: Callee = Callee {
        addr: 0x00176890,
        f: ee::s02::CameraTriggerMan_CollectOctreeNodeTriggers,
    };
    pub const C0017f500: Callee = Callee {
        addr: 0x0017f500,
        f: ee::s02::DebugMenuModule_DrawCursorTrail,
    };
    pub const C0019e2b0: Callee = Callee {
        addr: 0x0019e2b0,
        f: ee::s03::BoltFile_GetData,
    };
    pub const C0019f878: Callee = Callee {
        addr: 0x0019f878,
        f: ee::s03::FEModule_RiderPreviewFindTexture,
    };
    pub const C001bb9b0: Callee = Callee {
        addr: 0x001bb9b0,
        f: ee::s03::cFEStateCreateStep2_GetDaysInMonth,
    };
    pub const C001c84a8: Callee = Callee {
        addr: 0x001c84a8,
        f: ee::s03::cFEPopupConfirm_MeasureLine,
    };
    pub const C001c8568: Callee = Callee {
        addr: 0x001c8568,
        f: ee::s03::cFEPopupConfirm_MeasureLineW,
    };
    pub const C001c9b68: Callee = Callee {
        addr: 0x001c9b68,
        f: ee::s03::cFEPopupConfirm_AccumulateSize,
    };
    pub const C001cb9b0: Callee = Callee {
        addr: 0x001cb9b0,
        f: ee::s04::cKeyboardPopup_MoveDown,
    };
    pub const C001cbc48: Callee = Callee {
        addr: 0x001cbc48,
        f: ee::s04::cKeyboardPopup_MoveDownFromTopRow,
    };
    pub const C001cbce8: Callee = Callee {
        addr: 0x001cbce8,
        f: ee::s04::cKeyboardPopup_MoveDownFromBottomRow,
    };
    pub const C001cbfc0: Callee = Callee {
        addr: 0x001cbfc0,
        f: ee::s04::cKeyboardPopup_MoveDownFromRow4,
    };
    pub const C001ce560: Callee = Callee {
        addr: 0x001ce560,
        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
    };
    pub const C001d32e0: Callee = Callee {
        addr: 0x001d32e0,
        f: ee::s04::cFEStateMountainRoom_ShowImage,
    };
    pub const C001e1a30: Callee = Callee {
        addr: 0x001e1a30,
        f: ee::s04::MessageCenter_MatchCharacter,
    };
    pub const C001e3388: Callee = Callee {
        addr: 0x001e3388,
        f: ee::s04::MessageCenter_FindMessage,
    };
    pub const C001e91a8: Callee = Callee {
        addr: 0x001e91a8,
        f: ee::s04::HUD_GetElemColor,
    };
    pub const C001e91f8: Callee = Callee {
        addr: 0x001e91f8,
        f: ee::s04::HUD_GetElemPos,
    };
    pub const C001e9290: Callee = Callee {
        addr: 0x001e9290,
        f: ee::s04::HUD_GetElemScale,
    };
    pub const C001f10f8: Callee = Callee {
        addr: 0x001f10f8,
        f: ee::s04::HUD_AlignElemPos,
    };
    pub const C0020e900: Callee = Callee {
        addr: 0x0020e900,
        f: ee::s05::cOVStateOverlay_Update,
    };
    pub const C00224c78: Callee = Callee {
        addr: 0x00224c78,
        f: ee::s05::LunoVMRegister_ReadPtr,
    };
    pub const C00224da0: Callee = Callee {
        addr: 0x00224da0,
        f: ee::s05::cLunoTable_ctor,
    };
    pub const C00224df0: Callee = Callee {
        addr: 0x00224df0,
        f: ee::s05::cLunoTable_dtor,
    };
    pub const C00225068: Callee = Callee {
        addr: 0x00225068,
        f: ee::s05::cLunoTable_Assign,
    };
    pub const C002259a8: Callee = Callee {
        addr: 0x002259a8,
        f: ee::s05::cLunoTable_Read,
    };
    pub const C00227f80: Callee = Callee {
        addr: 0x00227f80,
        f: ee::s05::MCManager_Get,
    };
    pub const C00231f80: Callee = Callee {
        addr: 0x00231f80,
        f: ee::s06::LoadIndicator_Advance,
    };
    pub const C00238bf8: Callee = Callee {
        addr: 0x00238bf8,
        f: ee::s06::GameModeMan_SortByTime,
    };
    pub const C0026d4d8: Callee = Callee {
        addr: 0x0026d4d8,
        f: ee::s07::RunList_Seek,
    };
    pub const C0026db88: Callee = Callee {
        addr: 0x0026db88,
        f: ee::s07::Replay_RestoreSnapshot,
    };
    pub const C0026e448: Callee = Callee {
        addr: 0x0026e448,
        f: ee::s07::Replay_FindSnapshot,
    };
    pub const C0026f4a0: Callee = Callee {
        addr: 0x0026f4a0,
        f: ee::s07::Replay_SetState,
    };
    pub const C002702f8: Callee = Callee {
        addr: 0x002702f8,
        f: ee::s07::Replay_SeekToSnapshot,
    };
    pub const C002703f0: Callee = Callee {
        addr: 0x002703f0,
        f: ee::s07::Replay_StepFrame,
    };
    pub const C00275498: Callee = Callee {
        addr: 0x00275498,
        f: ee::s07::ScriptAnimBankMan_dtor,
    };
    pub const C002758c0: Callee = Callee {
        addr: 0x002758c0,
        f: ee::s07::DataInstanceMan_dtor,
    };
    pub const C00275b98: Callee = Callee {
        addr: 0x00275b98,
        f: ee::s07::DataInstanceMan_AddBigEntries,
    };
    pub const C00275cf8: Callee = Callee {
        addr: 0x00275cf8,
        f: ee::s07::DataInstanceMan_AllocInfo,
    };
    pub const C00278308: Callee = Callee {
        addr: 0x00278308,
        f: ee::s07::ScriptEngine_UnloadMaster,
    };
    pub const C00278e50: Callee = Callee {
        addr: 0x00278e50,
        f: ee::s07::ScriptEngine_ListAddChoice,
    };
    pub const C00278f38: Callee = Callee {
        addr: 0x00278f38,
        f: ee::s07::ScriptEngine_ListPrepare,
    };
    pub const C00278f68: Callee = Callee {
        addr: 0x00278f68,
        f: ee::s07::ScriptEngine_ListStart,
    };
    pub const C00281ed8: Callee = Callee {
        addr: 0x00281ed8,
        f: ee::s07::ScriptEngine_ScriptDbDtor,
    };
    pub const C00283228: Callee = Callee {
        addr: 0x00283228,
        f: ee::s07::ScriptObj_dtor,
    };
    pub const C00283518: Callee = Callee {
        addr: 0x00283518,
        f: ee::s07::ScriptSoundBankMan_dtor,
    };
    pub const C002837c8: Callee = Callee {
        addr: 0x002837c8,
        f: ee::s07::ScriptMovie_dtor,
    };
    pub const C0028b180: Callee = Callee {
        addr: 0x0028b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };
    pub const C0028bb10: Callee = Callee {
        addr: 0x0028bb10,
        f: ee::s07::SndBankMan_dtor,
    };
    pub const C0028e8c0: Callee = Callee {
        addr: 0x0028e8c0,
        f: ee::s07::SSXAudioSystem_HandleMusicEvent,
    };
    pub const C002913d8: Callee = Callee {
        addr: 0x002913d8,
        f: ee::s07::SSXAudioSystem_StopAmbienceLoop,
    };
    pub const C003457c8: Callee = Callee {
        addr: 0x003457c8,
        f: ee::s11::ParticleModifier_BaseCtor_2,
    };
    pub const C00358020: Callee = Callee {
        addr: 0x00358020,
        f: ee::s11::MultiParticle_CreateEmitter,
    };
    pub const C00362478: Callee = Callee {
        addr: 0x00362478,
        f: ee::s12::GsPkt_AddAlpha,
    };
    pub const C003625c0: Callee = Callee {
        addr: 0x003625c0,
        f: ee::s12::GsPkt_AddClamp,
    };
    pub const C00362660: Callee = Callee {
        addr: 0x00362660,
        f: ee::s12::GsPkt_AddZbuf,
    };
    pub const C003626d8: Callee = Callee {
        addr: 0x003626d8,
        f: ee::s12::GsPkt_AddTest,
    };
    pub const C003627a8: Callee = Callee {
        addr: 0x003627a8,
        f: ee::s12::GsPkt_Finish,
    };
    pub const C00364cd0: Callee = Callee {
        addr: 0x00364cd0,
        f: ee::s12::DeferMan_EmitMaterialSetup,
    };
    pub const C00366ce0: Callee = Callee {
        addr: 0x00366ce0,
        f: ee::s12::VramHeap_Alloc,
    };
    pub const C00368170: Callee = Callee {
        addr: 0x00368170,
        f: ee::s12::TexMan_EmitImageUpload,
    };
    pub const C003684f0: Callee = Callee {
        addr: 0x003684f0,
        f: ee::s12::TexMan_EmitClutUpload,
    };
    pub const C00368970: Callee = Callee {
        addr: 0x00368970,
        f: ee::s12::TexMan_EmitTex0,
    };
    pub const C0038f460: Callee = Callee {
        addr: 0x0038f460,
        f: ee::s12::SprStream_BeginBlock,
    };
    pub const C0038f668: Callee = Callee {
        addr: 0x0038f668,
        f: ee::s12::SprStream_EndBlockAligned,
    };
    pub const C0039c870: Callee = Callee {
        addr: 0x0039c870,
        f: ee::s13::UIScreen_StartThread,
    };
    pub const C0039ff78: Callee = Callee {
        addr: 0x0039ff78,
        f: ee::s13::UIObject_GetEventThread,
    };
    pub const C003b1300: Callee = Callee {
        addr: 0x003b1300,
        f: ee::s13::MPEG2_Get_Bits,
    };
    pub const C003b2360: Callee = Callee {
        addr: 0x003b2360,
        f: ee::s13::MPEG2_marker_bit,
    };
}

// ---------------------------------------------------------------------------
// Register, memory and frame helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `lw` (sign-extended to 64 bits).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}
/// `sw`.
#[inline(always)]
fn sw(c: &mut Cpu, a: u32, v: u64) {
    c.sw(a, v);
}
/// `lh` (sign-extended).
#[inline(always)]
fn lh(c: &mut Cpu, a: u32) -> u64 {
    c.lh(a)
}
/// `lhu`.
#[inline(always)]
fn lhu(c: &mut Cpu, a: u32) -> u64 {
    c.lhu(a)
}
/// `sh`.
#[inline(always)]
fn sh(c: &mut Cpu, a: u32, v: u64) {
    c.sh(a, v);
}
/// `lb` (sign-extended).
#[inline(always)]
fn lb(c: &mut Cpu, a: u32) -> u64 {
    c.lb(a)
}
/// `lbu`.
#[inline(always)]
fn lbu(c: &mut Cpu, a: u32) -> u64 {
    c.lbu(a)
}
/// `sb`.
#[inline(always)]
fn sb(c: &mut Cpu, a: u32, v: u64) {
    c.sb(a, v);
}
/// `ld`.
#[inline(always)]
fn ld(c: &mut Cpu, a: u32) -> u64 {
    c.ld(a)
}
/// `sd`.
#[inline(always)]
fn sd(c: &mut Cpu, a: u32, v: u64) {
    c.sd(a, v);
}
/// `lq`.
#[inline(always)]
fn lq(c: &mut Cpu, rt: usize, a: u32) {
    c.lq(rt, a);
}
/// `sq`.
#[inline(always)]
fn sq(c: &mut Cpu, rt: usize, a: u32) {
    c.sq(rt, a);
}
/// `ldl rt, a+7; ldr rt, a`: one unaligned 8-byte load, merged into `rt`
/// (the pre-existing bits of `rt` matter, as in the original).
#[inline(always)]
fn ldl(c: &mut Cpu, rt: usize, a: u32) {
    c.ldl(rt, a.wrapping_add(7));
    c.ldr(rt, a);
}
/// `sdl rt, a+7; sdr rt, a`: one unaligned 8-byte store.
#[inline(always)]
fn sdl(c: &mut Cpu, rt: usize, a: u32) {
    c.sdl(rt, a.wrapping_add(7));
    c.sdr(rt, a);
}

/// `void func_001CB9B0(void* popup, int a1)` (0x1cb9b0): move the on-screen
/// keyboard's selection down one key. Reads the popup's "shifted" flag
/// (`+0x68`/`+0x6c`), remembers the old row (`+0x434`) and whether the key
/// was enabled (`+0x144`), and for a1 != 0 stores the row (when
/// `IsKeyEnabled(old, 0)` finds the key). Then, depending on the row index at
/// `+0x140`, calls the row/column stepping helper (rows 56..69, 70..83 or
/// below 14), advances `+0x140` by 14 (or wraps 70..83 back to 0..13),
/// skips disabled keys marked 0x2a in `D_00441390`, and finally re-checks the
/// key with `IsKeyEnabled(idx, &value)`; if it stays disabled it re-enters
/// itself and sets `+0x144` from the flag.
pub fn c_keyboard_popup_move_down(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_1cb9e4: {
        'b_1cb9e0: {
            c.cycles += 10;
            frame.save(c, S0, 0x40);
            frame.save(c, S1, 0x30);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, S2, 0x20);
            frame.save_ra(c, 0x10);
            c.g[V0][0] = lw(c, at(c, S0, 0x68));
            c.g[V1][0] = lw(c, at(c, S0, 0x6c));
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            if t {
                break 'b_1cb9e0;
            } else {
                c.cycles += 2;
                let t = true;
                c.g[S1][0] = c.g[V0][0] ^ 0x1;
                if t {
                    break 'b_1cb9e4;
                } else {
                    break 'b_1cb9e0;
                }
            }
        }
        c.cycles += 1;
        c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
        break 'b_1cb9e4;
    }
    'b_1cba00: {
        c.cycles += 5;
        c.g[A0][0] = lw(c, at(c, S0, 0x140));
        c.g[V0][0] = sx(r32(c, A0).wrapping_sub(0xf));
        c.g[V0][0] = (c.g[V0][0] < 0xcu64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[S2][0] = lw(c, at(c, S0, 0x434));
        if t {
            break 'b_1cba00;
        } else {
            c.cycles += 2;
            c.g[V0][0] = lw(c, at(c, S0, 0x6c));
            c.g[S1][0] = c.g[V0][0] ^ 0x1;
            break 'b_1cba00;
        }
    }
    'b_1cbb5c: {
        'b_1cbb58: {
            'b_1cba6c: {
                'b_1cba68: {
                    c.cycles += 2;
                    let t = c.g[A1][0] == 0u64;
                    c.g[V1][0] = lw(c, at(c, S0, 0x430));
                    if t {
                        break 'b_1cba68;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = 0x1;
                        sw(c, at(c, S0, 0x434), c.g[A0][0]);
                        let t = c.g[V1][0] != 0u64;
                        sw(c, at(c, S0, 0x430), c.g[V0][0]);
                        if t {
                            break 'b_1cba68;
                        } else {
                            'b_1cba4c: {
                                c.cycles += 4;
                                c.g[V1][0] = lw(c, at(c, S0, 0x144));
                                c.g[V0][0] = 0x2;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[A0][0] = c.g[S0][0];
                                    break 'b_1cba4c;
                                } else {
                                    'b_1cba40: {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == 0u64;

                                        if t {
                                            break 'b_1cba40;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[A0][0] = c.g[S0][0];
                                            if t {
                                                break 'b_1cba4c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V1][0] = lw(c, at(c, S0, 0x140));
                                                if t {
                                                    break 'b_1cba6c;
                                                } else {
                                                    break 'b_1cba40;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[V1][0] != 0u64 {
                                        c.g[V1][0] = lw(c, at(c, S0, 0x140));
                                        break 'b_1cba6c;
                                    } else {
                                        c.cycles += 1;
                                        c.g[A0][0] = c.g[S0][0];
                                        break 'b_1cba4c;
                                    }
                                }
                            }
                            c.cycles += 5;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cba58);
                            c.g[A2][0] = 0u64;
                            if callee::C001ce560.call(c, 0x001cba58).is_err() {
                                return;
                            }
                            if c.g[V0][0] == 0u64 {
                                c.g[V1][0] = lw(c, at(c, S0, 0x140));
                                break 'b_1cba6c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                sw(c, at(c, S0, 0x140), c.g[S2][0]);
                                if t {
                                    break 'b_1cbb58;
                                } else {
                                    break 'b_1cba68;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = lw(c, at(c, S0, 0x140));
                break 'b_1cba6c;
            }
            'b_1cbac4: {
                'b_1cbabc: {
                    'b_1cba8c: {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x38));
                        c.g[V0][0] = (c.g[V0][0] < 0xeu64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_1cba8c;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001cba84);
                            c.g[A1][0] = c.g[S1][0];
                            if callee::C001cbfc0.call(c, 0x001cba84).is_err() {
                                return;
                            }
                            let t = true;

                            if t {
                                break 'b_1cbabc;
                            } else {
                                break 'b_1cba8c;
                            }
                        }
                    }
                    'b_1cbaac: {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x46));
                        c.g[V0][0] = (c.g[V0][0] < 0xeu64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
                        if t {
                            break 'b_1cbaac;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001cbaa4);
                            c.g[A1][0] = c.g[S1][0];
                            if callee::C001cbce8.call(c, 0x001cbaa4).is_err() {
                                return;
                            }
                            let t = true;

                            if t {
                                break 'b_1cbabc;
                            } else {
                                break 'b_1cbaac;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_1cbac4;
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x001cbabc);
                        c.g[A1][0] = c.g[S1][0];
                        if callee::C001cbc48.call(c, 0x001cbabc).is_err() {
                            return;
                        }
                        break 'b_1cbabc;
                    }
                }
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x40);
                    break 'b_1cbb5c;
                } else {
                    break 'b_1cbac4;
                }
            }
            'b_1cbae4: {
                'b_1cbae0: {
                    c.cycles += 5;
                    c.g[A0][0] = lw(c, at(c, S0, 0x140));
                    c.g[V1][0] = sx(r32(c, A0).wrapping_sub(0x46));
                    c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx(at(c, A0, 0xe));
                    if t {
                        break 'b_1cbae0;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        sw(c, at(c, S0, 0x140), c.g[V1][0]);
                        if t {
                            break 'b_1cbae4;
                        } else {
                            break 'b_1cbae0;
                        }
                    }
                }
                c.cycles += 1;
                sw(c, at(c, S0, 0x140), c.g[V0][0]);
                break 'b_1cbae4;
            }
            'b_1cbb2c: {
                c.cycles += 8;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = lw(c, at(c, S0, 0x140));
                c.g[A2][0] = sx(at(c, V0, 0x1390));
                c.g[A0][0] = 0x2a;
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, A2)));
                c.g[V1][0] = lb(c, r32(c, V0));
                if c.g[V1][0] != c.g[A0][0] {
                    c.g[A0][0] = c.g[S0][0];
                    break 'b_1cbb2c;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = sx(at(c, A1, 0x1));
                    'l_1cbb08: loop {
                        c.cycles += 7;
                        c.g[A1][0] = c.g[V0][0];
                        sw(c, at(c, S0, 0x140), c.g[V0][0]);
                        c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, A2)));
                        c.g[V0][0] = lb(c, r32(c, V1));
                        let t = c.g[V0][0] == c.g[A0][0];
                        c.g[V0][0] = sx(at(c, A1, 0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001cbb08);
                            }
                            continue 'l_1cbb08;
                        } else {
                            c.cycles += 2;
                            c.g[A1][0] = lw(c, at(c, S0, 0x140));
                            c.g[A0][0] = c.g[S0][0];
                            break 'b_1cbb2c;
                        }
                    }
                }
            }
            'b_1cbb54: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001cbb34);
                c.g[A2][0] = c.g[SP][0];
                if callee::C001ce560.call(c, 0x001cbb34).is_err() {
                    return;
                }
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x2;
                if t {
                    break 'b_1cbb54;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001cbb48);
                    c.g[A1][0] = 0u64;
                    if callee::C001cb9b0.call(c, 0x001cbb48).is_err() {
                        return;
                    }
                    c.g[V0][0] = lw(c, r32(c, SP));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[S1][0] < 0x1u64) as u64;
                    if t {
                        break 'b_1cbb58;
                    } else {
                        break 'b_1cbb54;
                    }
                }
            }
            c.cycles += 1;
            sw(c, at(c, S0, 0x144), c.g[V0][0]);
            break 'b_1cbb58;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x40);
        break 'b_1cbb5c;
    }
    c.cycles += 5;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x50));
    c.pc = t;
    return;
}

/// `void func_0014ECA0(int self, int idx)` (0x14eca0): restore a BE player
/// character setup: `BE_RestoreCharacter(&D_004A6CA8[idx])`, mirror
/// `D_00535B20[0]` from `[1]` when `D_00534B38` is clear, copy
/// `D_00535B20[1..6]` from `D_00534A88`, then let the BE interfaces 1 and 6
/// act on it (vtable slot 2 of each).
pub fn be_restore_player_setup(c: &mut Cpu) {
    let frame = Frame::open(c, 0x10);
    'b_14ed14: {
        c.cycles += 12;
        c.g[V0][0] = 0x9b50;
        c.g[A0][0] = 0x4a0000;
        c.mult(0, A1, A1, V0);
        c.g[A0][0] = sx(at(c, A0, 0x6ca8));
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[RA][0] = sx(0x0014ecc0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        if callee::C00156988.call(c, 0x0014ecc0).is_err() {
            return;
        }
        c.g[V0][0] = 0x530000;
        c.g[V1][0] = lw(c, at(c, V0, 0x4b38));
        let t = c.g[V1][0] != 0u64;
        c.g[V1][0] = 0x530000;
        if t {
            break 'b_14ed14;
        } else {
            c.cycles += 17;
            c.g[V0][0] = sx(at(c, V1, 0x5b20));
            c.g[A3][0] = sx(at(c, V1, 0x5b20));
            ldl(c, 4, at(c, V0, 0x1c));
            ldl(c, 5, at(c, V0, 0x24));
            ldl(c, 6, at(c, V0, 0x2c));
            sdl(c, 4, r32(c, A3));
            sdl(c, 5, at(c, A3, 0x8));
            sdl(c, 6, at(c, A3, 0x10));
            c.g[A0][0] = lw(c, at(c, V0, 0x34));
            sw(c, at(c, A3, 0x18), c.g[A0][0]);
            c.g[V1][0] = 0x530000;
            break 'b_14ed14;
        }
    }
    c.cycles += 7;
    c.g[V0][0] = 0x530000;
    c.g[V1][0] = sx(at(c, V1, 0x5b20));
    c.g[V0][0] = sx(at(c, V0, 0x4a88));
    c.g[A1][0] = sx(at(c, V1, 0xa8));
    c.g[A0][0] = sx(at(c, V0, 0x1c));
    c.g[V1][0] = sx(at(c, V1, 0x1c));
    'l_14ed30: loop {
        c.cycles += 18;
        ldl(c, 8, r32(c, A0));
        ldl(c, 2, at(c, A0, 0x8));
        ldl(c, 6, at(c, A0, 0x10));
        c.g[A3][0] = lw(c, at(c, A0, 0x18));
        sdl(c, 8, r32(c, V1));
        sdl(c, 2, at(c, V1, 0x8));
        sdl(c, 6, at(c, V1, 0x10));
        sw(c, at(c, V1, 0x18), c.g[A3][0]);
        c.g[V1][0] = sx(at(c, V1, 0x1c));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[A0][0] = sx(at(c, A0, 0x1c));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0014ed30);
            }
            continue 'l_14ed30;
        } else {
            c.cycles += 23;
            c.g[RA][0] = sx(0x0014ed80);
            if callee::C0014dc80.call(c, 0x0014ed80).is_err() {
                return;
            }
            c.g[A1][0] = 0x1;
            c.g[RA][0] = sx(0x0014ed8c);
            c.g[A0][0] = c.g[V0][0];
            if callee::C0014dd58.call(c, 0x0014ed8c).is_err() {
                return;
            }
            c.g[V1][0] = lw(c, at(c, V0, 0xc));
            c.g[A0][0] = lh(c, at(c, V1, 0x10));
            c.g[A1][0] = lw(c, at(c, V1, 0x14));
            let t = c.g[A1][0] as u32;
            c.g[RA][0] = sx(0x0014eda0);
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0014eda0).is_err() {
                return;
            }
            c.g[RA][0] = sx(0x0014eda8);
            if callee::C0014dc80.call(c, 0x0014eda8).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0014edb4);
            c.g[A1][0] = 0x6;
            if callee::C0014dd58.call(c, 0x0014edb4).is_err() {
                return;
            }
            c.g[V1][0] = lw(c, at(c, V0, 0xc));
            c.g[A0][0] = lh(c, at(c, V1, 0x10));
            c.g[A1][0] = lw(c, at(c, V1, 0x14));
            let t = c.g[A1][0] as u32;
            c.g[RA][0] = sx(0x0014edc8);
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0014edc8).is_err() {
                return;
            }
            c.g[RA][0] = ld(c, r32(c, SP));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at(c, SP, 0x10));
            c.pc = t;
            return;
        }
    }
}

/// `int func_0019A098(void* self, const char* name)` (0x19a098): find the
/// equipment package entry called `name` for the character in `self` (equip
/// table at `world+0x7c + 0x1a70`, slots at `+0xbc`/`+0xa64`) and return its
/// model id. A missing package or a miss returns -1; a hit binds the entry's
/// data through slot 46 or 48 depending on whether an id was already stored
/// at `self+0xa68`.
pub fn c_fe_state_char_equip_get_preview_texture(c: &mut Cpu) {
    let frame = Frame::open(c, 0xa0);
    'b_19a1b0: {
        'b_19a1ac: {
            c.cycles += 14;
            frame.save(c, S3, 0x60);
            frame.save(c, S5, 0x40);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S0, 0x90);
            frame.save(c, S1, 0x80);
            frame.save(c, S2, 0x70);
            frame.save(c, S4, 0x50);
            frame.save(c, S6, 0x30);
            frame.save(c, S7, 0x20);
            frame.save_ra(c, 0x10);
            c.g[V0][0] = lw(c, at(c, S3, 0xa60));
            let t = c.g[V0][0] == 0u64;
            c.g[S5][0] = c.g[A1][0];
            if t {
                break 'b_19a1ac;
            } else {
                c.cycles += 9;
                c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                c.g[A1][0] = lw(c, at(c, S3, 0xbc));
                c.g[A0][0] = lw(c, at(c, V0, 0x7c));
                c.g[A2][0] = lw(c, at(c, S3, 0xa64));
                c.g[RA][0] = sx(0x0019a0e8);
                c.g[A0][0] = sx(at(c, A0, 0x1a70));
                if callee::C0019e2b0.call(c, 0x0019a0e8).is_err() {
                    return;
                }
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_19a1b0;
                } else {
                    c.cycles += 3;
                    c.g[S4][0] = lw(c, at(c, S1, 0x8));
                    let t = (c.g[S4][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        break 'b_19a1b0;
                    } else {
                        c.cycles += 4;
                        c.g[S6][0] = 0xffffffffffffffff;
                        c.g[S7][0] = 0x4a0000;
                        c.g[S2][0] = sx(at(c, S1, 0x14));
                        'l_19a110: loop {
                            'b_19a1a0: {
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0019a120);
                                c.g[A2][0] = c.g[SP][0];
                                if call_ptr(c, 0x003b3d00, 0x0019a120).is_err() {
                                    return;
                                }
                                sb(c, at(c, SP, 0x4), 0u64);
                                c.g[A0][0] = c.g[SP][0];
                                c.g[RA][0] = sx(0x0019a130);
                                c.g[A1][0] = c.g[S5][0];
                                if call_ptr(c, 0x004165a8, 0x0019a130).is_err() {
                                    return;
                                }
                                let t = c.g[V0][0] != 0u64;
                                c.g[S0][0] = sx(at(c, S0, 0x1));
                                if t {
                                    break 'b_19a1a0;
                                } else {
                                    'b_19a198: {
                                        'b_19a17c: {
                                            c.cycles += 4;
                                            c.g[V0][0] = lw(c, r32(c, S2));
                                            c.g[A1][0] = lw(c, at(c, S3, 0xa68));
                                            let t = c.g[A1][0] != c.g[S6][0];
                                            c.g[A2][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                            if t {
                                                break 'b_19a17c;
                                            } else {
                                                c.cycles += 13;
                                                c.g[T2][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                                                c.g[A1][0] = c.g[A2][0];
                                                c.g[A2][0] = sx(at(c, S7, 0x1950));
                                                c.g[A3][0] = 0u64;
                                                c.g[V1][0] = lw(c, at(c, T2, 0x10d8));
                                                c.g[T0][0] = 0x1;
                                                c.g[T1][0] = 0xffffffffffffffff;
                                                c.g[A0][0] = lh(c, at(c, V1, 0x170));
                                                c.g[V0][0] = lw(c, at(c, V1, 0x174));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x0019a174);
                                                c.g[A0][0] =
                                                    sx(r32(c, T2).wrapping_add(r32(c, A0)));
                                                if call_ptr(c, t, 0x0019a174).is_err() {
                                                    return;
                                                }
                                                let t = true;
                                                sw(c, at(c, S3, 0xa68), c.g[V0][0]);
                                                if t {
                                                    break 'b_19a198;
                                                } else {
                                                    break 'b_19a17c;
                                                }
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[T0][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                                        c.g[A3][0] = 0x1;
                                        c.g[V0][0] = lw(c, at(c, T0, 0x10d8));
                                        c.g[A0][0] = lh(c, at(c, V0, 0x180));
                                        c.g[V1][0] = lw(c, at(c, V0, 0x184));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x0019a198);
                                        c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
                                        if call_ptr(c, t, 0x0019a198).is_err() {
                                            return;
                                        }
                                        break 'b_19a198;
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = lw(c, at(c, S3, 0xa68));
                                    if t {
                                        break 'b_19a1b0;
                                    } else {
                                        break 'b_19a1a0;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S2][0] = sx(at(c, S2, 0x8));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0019a110);
                                }
                                continue 'l_19a110;
                            } else {
                                break 'b_19a1ac;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0xffffffffffffffff;
        break 'b_19a1b0;
    }
    c.cycles += 11;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xa0));
    c.pc = t;
    return;
}

/// `void func_00209370(sMap_9370* self)` (0x209370): per-frame update of the
/// session/loading overlay: run the overlay's base update, then either fade
/// the image in (`fade += 0.1` up to 1) and push it to the colour setter, or
/// clear it; poll the async file status and, once the file is ready, hand the
/// loaded data to the image; then bob the eight map icons (sinusoidal scale
/// and offset) around the icon positions.
pub fn ov_screen_session_update(c: &mut Cpu) {
    let frame = Frame::open(c, 0x80);
    'b_2095c8: {
        c.cycles += 13;
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0x20);
        c.swc1(21, at(c, SP, 0x78));
        c.g[RA][0] = sx(0x00209398);
        c.swc1(20, at(c, SP, 0x70));
        if callee::C0020e900.call(c, 0x00209398).is_err() {
            return;
        }
        c.g[V0][0] = lw(c, at(c, S0, 0xdc));
        if c.g[V0][0] != 0u64 {
            frame.restore(c, S0, 0x60);
            break 'b_2095c8;
        } else {
            'b_209460: {
                'b_209440: {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, at(c, S0, 0xc8));
                    if c.g[V0][0] == 0u64 {
                        sw(c, at(c, S0, 0xcc), 0u64);
                        break 'b_209440;
                    } else {
                        'b_2093dc: {
                            c.cycles += 7;
                            c.lwc1(1, at(c, S0, 0xcc));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.c_lt_s(1, 0);
                            if !c.fcc() {
                                c.g[A0][0] = lw(c, at(c, S0, 0xd0));
                                break 'b_2093dc;
                            } else {
                                c.cycles += 4;
                                c.lwc1(0, r32(c, GP).wrapping_sub(0x5504));
                                c.add_s(0, 1, 0);
                                c.swc1(0, at(c, S0, 0xcc));
                                c.g[A0][0] = lw(c, at(c, S0, 0xd0));
                                break 'b_2093dc;
                            }
                        }
                        c.cycles += 25;
                        c.g[A1][0] = c.g[SP][0];
                        c.lwc1(0, at(c, S0, 0xcc));
                        ldl(c, 2, at(c, A0, 0x1c));
                        ldl(c, 3, at(c, A0, 0x24));
                        sdl(c, 2, r32(c, SP));
                        sdl(c, 3, at(c, SP, 0x8));
                        c.swc1(0, r32(c, SP));
                        c.g[V0][0] = lw(c, at(c, A0, 0x8));
                        c.g[V1][0] = lh(c, at(c, V0, 0x58));
                        c.g[A2][0] = lw(c, at(c, V0, 0x5c));
                        let t = c.g[A2][0] as u32;
                        c.g[RA][0] = sx(0x0020941c);
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                        if call_ptr(c, t, 0x0020941c).is_err() {
                            return;
                        }
                        c.g[A3][0] = lw(c, at(c, S0, 0xd0));
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = lw(c, at(c, A3, 0x8));
                        c.g[A0][0] = lh(c, at(c, V0, 0x48));
                        c.g[V1][0] = lw(c, at(c, V0, 0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00209438);
                        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x00209438).is_err() {
                            return;
                        }
                        let t = true;
                        c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                        if t {
                            break 'b_209460;
                        } else {
                            break 'b_209440;
                        }
                    }
                }
                c.cycles += 8;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = lw(c, at(c, S0, 0xd0));
                c.g[V0][0] = lw(c, at(c, A2, 0x8));
                c.g[A0][0] = lh(c, at(c, V0, 0x48));
                c.g[V1][0] = lw(c, at(c, V0, 0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0020945c);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x0020945c).is_err() {
                    return;
                }
                c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                break 'b_209460;
            }
            'b_2094bc: {
                c.cycles += 7;
                c.g[A1][0] = lw(c, at(c, S0, 0xc4));
                c.g[S1][0] = lw(c, at(c, V0, 0x11c));
                c.g[RA][0] = sx(0x00209470);
                c.g[A0][0] = c.g[S1][0];
                if call_ptr(c, 0x001a39c0, 0x00209470).is_err() {
                    return;
                }
                c.g[V1][0] = 0x3;
                if c.g[V0][0] != c.g[V1][0] {
                    c.lwc1(1, at(c, S0, 0x1d0));
                    break 'b_2094bc;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, at(c, S0, 0xc8));
                    if c.g[V0][0] != 0u64 {
                        c.lwc1(1, at(c, S0, 0x1d0));
                        break 'b_2094bc;
                    } else {
                        'b_2094b8: {
                            c.cycles += 3;
                            c.g[A2][0] = lw(c, at(c, S0, 0xd0));
                            let t = c.g[A2][0] == 0u64;
                            c.g[V1][0] = 0x11c;
                            if t {
                                break 'b_2094b8;
                            } else {
                                c.cycles += 9;
                                c.g[V0][0] = lw(c, at(c, S0, 0xc4));
                                c.g[A1][0] = 0x1;
                                c.mult(0, A0, V0, V1);
                                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                                c.g[A0][0] = lw(c, at(c, V0, 0x110));
                                sw(c, at(c, A2, 0x7c), 0u64);
                                sw(c, at(c, A2, 0x78), c.g[A0][0]);
                                sw(c, at(c, S0, 0xc8), c.g[A1][0]);
                                sw(c, at(c, S0, 0xcc), 0u64);
                                break 'b_2094b8;
                            }
                        }
                        c.cycles += 1;
                        c.lwc1(1, at(c, S0, 0x1d0));
                        break 'b_2094bc;
                    }
                }
            }
            'b_209518: {
                'b_209514: {
                    'b_2094f4: {
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();
                        c.lwc1(1, at(c, S0, 0x1cc));
                        if t {
                            break 'b_2094f4;
                        } else {
                            c.cycles += 6;
                            c.g[AT][0] = 0x41ac0000;
                            c.mtc1(1, 0);
                            c.c_lt_s(0, 1);
                            if !c.fcc() {
                                c.lwc1(0, at(c, S0, 0x1cc));
                                break 'b_209518;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, r32(c, GP).wrapping_sub(0x5500));
                                let t = true;
                                c.swc1(0, at(c, S0, 0x1d0));
                                if t {
                                    break 'b_209514;
                                } else {
                                    break 'b_2094f4;
                                }
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[AT][0] = 0x41900000;
                    c.mtc1(1, 0);
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.lwc1(0, at(c, S0, 0x1cc));
                        break 'b_209518;
                    } else {
                        c.cycles += 2;
                        c.lwc1(0, r32(c, GP).wrapping_sub(0x54fc));
                        c.swc1(0, at(c, S0, 0x1d0));
                        break 'b_209514;
                    }
                }
                c.cycles += 1;
                c.lwc1(0, at(c, S0, 0x1cc));
                break 'b_209518;
            }
            c.cycles += 10;
            c.g[S2][0] = sx(at(c, S0, 0x124));
            c.lwc1(1, at(c, S0, 0x1d0));
            c.g[S1][0] = sx(at(c, S0, 0x104));
            c.g[AT][0] = 0x41900000;
            c.mtc1(1, 21);
            c.g[S3][0] = 0x7;
            c.add_s(0, 0, 1);
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 20);
            c.swc1(0, at(c, S0, 0x1cc));
            'l_209540: loop {
                'b_2095b8: {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, r32(c, S1));
                    if c.g[V0][0] == 0u64 {
                        c.g[S2][0] = sx(at(c, S2, 0xc));
                        break 'b_2095b8;
                    } else {
                        c.cycles += 27;
                        c.lwc1(0, at(c, S0, 0x1cc));
                        c.lwc1(2, r32(c, S2));
                        c.sub_s(0, 0, 21);
                        c.lwc1(1, at(c, S2, 0x4));
                        c.mul_s(0, 0, 20);
                        c.sub_s(1, 1, 0);
                        c.sub_s(2, 2, 0);
                        c.swc1(1, at(c, SP, 0x4));
                        c.swc1(2, r32(c, SP));
                        ldl(c, 6, r32(c, SP));
                        c.g[A3][0] = lw(c, at(c, SP, 0x8));
                        sdl(c, 6, at(c, V0, 0x44));
                        sw(c, at(c, V0, 0x4c), c.g[A3][0]);
                        c.g[V0][0] = lw(c, r32(c, S1));
                        c.lwc1(0, at(c, S0, 0x1cc));
                        c.g[V1][0] = lw(c, at(c, V0, 0x8));
                        c.g[A0][0] = lh(c, at(c, V1, 0xa8));
                        c.swc1(0, at(c, SP, 0x14));
                        c.swc1(0, at(c, SP, 0x10));
                        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                        sw(c, at(c, SP, 0x18), 0u64);
                        c.g[V0][0] = lw(c, at(c, V1, 0xac));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x002095b4);
                        c.g[A1][0] = sx(at(c, SP, 0x10));
                        if call_ptr(c, t, 0x002095b4).is_err() {
                            return;
                        }
                        c.g[S2][0] = sx(at(c, S2, 0xc));
                        break 'b_2095b8;
                    }
                }
                c.cycles += 3;
                c.g[S3][0] = sx(r32(c, S3).wrapping_sub(0x1));
                let t = (c.g[S3][0] as i64) >= 0;
                c.g[S1][0] = sx(at(c, S1, 0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00209540);
                    }
                    continue 'l_209540;
                } else {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x60);
                    break 'b_2095c8;
                }
            }
        }
    }
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    c.lwc1(21, at(c, SP, 0x78));
    c.lwc1(20, at(c, SP, 0x70));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x80));
    c.pc = t;
    return;
}

/// `void HUD_DrawTextElemW(void* ctx, void* font, const char* str, ...)`
/// (0x1f22e8): draw one HUD text element: measure the definition's position
/// and scale, scale the font, measure and align the text, add the position
/// offset, fill the clip rect, pick the colour (definition or given), patch
/// the GS alpha field from the definition's flags, and draw the string twice
/// (once through the deferred packet, once with the final tint).
pub fn hud_draw_text_elem_w(c: &mut Cpu) {
    let frame = Frame::open(c, 0x140);
    'b_1f2398: {
        c.cycles += 37;
        c.g[V0][0] = 0x24;
        frame.save(c, S5, 0xe0);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S4, 0xf0);
        c.mult(0, V1, S5, V0);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S1, 0x120);
        frame.save(c, S2, 0x110);
        c.g[S1][0] = c.g[A1][0];
        sw(c, at(c, SP, 0x90), c.g[A0][0]);
        c.g[A1][0] = sx(at(c, SP, 0x10));
        c.g[S2][0] = sx(r32(c, V1).wrapping_add(r32(c, S4)));
        frame.save(c, S0, 0x130);
        frame.save(c, S3, 0x100);
        c.g[A0][0] = c.g[S2][0];
        frame.save(c, S6, 0xd0);
        c.g[S0][0] = c.g[T2][0];
        frame.save(c, S7, 0xc0);
        sq(c, 30, at(c, SP, 0xb0));
        sw(c, at(c, SP, 0x94), c.g[T0][0]);
        sw(c, at(c, SP, 0x98), c.g[T1][0]);
        c.swc1(0, at(c, SP, 0x4));
        c.g[S6][0] = lw(c, at(c, SP, 0x148));
        c.g[S7][0] = lw(c, at(c, SP, 0x150));
        c.g[FP][0] = lw(c, at(c, SP, 0x158));
        c.swc1(0, r32(c, SP));
        frame.save_ra(c, 0xa0);
        c.g[RA][0] = sx(0x001f2368);
        c.g[S3][0] = c.g[T3][0];
        if callee::C001e91f8.call(c, 0x001f2368).is_err() {
            return;
        }
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001f2374);
        c.g[A1][0] = sx(at(c, SP, 0x20));
        if callee::C001e9290.call(c, 0x001f2374).is_err() {
            return;
        }
        let t = c.g[S0][0] == 0u64;
        c.lwc1(1, at(c, SP, 0x20));
        if t {
            break 'b_1f2398;
        } else {
            c.cycles += 7;
            c.lwc1(3, at(c, S0, 0x4));
            c.lwc1(2, r32(c, S0));
            c.lwc1(0, at(c, SP, 0x24));
            c.mul_s(1, 1, 2);
            c.mul_s(0, 0, 3);
            c.swc1(1, at(c, SP, 0x20));
            c.swc1(0, at(c, SP, 0x24));
            break 'b_1f2398;
        }
    }
    'b_1f23e4: {
        c.cycles += 18;
        c.lwc1(3, at(c, SP, 0x20));
        c.g[A0][0] = c.g[S1][0];
        c.lwc1(2, at(c, SP, 0x24));
        c.g[A2][0] = sx(at(c, SP, 0x30));
        c.lwc1(1, at(c, S1, 0x30));
        c.g[A3][0] = 0u64;
        c.lwc1(0, at(c, S1, 0x34));
        c.mul_s(1, 1, 3);
        c.g[A1][0] = lw(c, at(c, SP, 0x94));
        c.mul_s(0, 0, 2);
        c.mov_s(12, 1);
        c.swc1(1, at(c, S1, 0x38));
        c.mov_s(13, 0);
        c.g[RA][0] = sx(0x001f23d4);
        c.swc1(0, at(c, S1, 0x3c));
        if call_ptr(c, 0x003921f0, 0x001f23d4).is_err() {
            return;
        }
        c.g[V0][0] = 0x7f;
        let t = c.g[S7][0] != c.g[V0][0];
        c.g[T1][0] = c.g[S7][0];
        if t {
            break 'b_1f23e4;
        } else {
            c.cycles += 1;
            c.g[T1][0] = lb(c, at(c, S2, 0x20));
            break 'b_1f23e4;
        }
    }
    'b_1f23fc: {
        c.cycles += 2;
        let t = c.g[FP][0] != c.g[V0][0];
        c.g[T2][0] = c.g[FP][0];
        if t {
            break 'b_1f23fc;
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x24;
            c.mult(0, V1, S5, V0);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S4)));
            c.g[T2][0] = lb(c, at(c, V0, 0x21));
            break 'b_1f23fc;
        }
    }
    'b_1f244c: {
        c.cycles += 13;
        c.lwc1(0, at(c, SP, 0x38));
        c.g[A2][0] = sx(at(c, SP, 0x10));
        c.lwc1(1, at(c, SP, 0x3c));
        c.g[A1][0] = sx(at(c, SP, 0x40));
        c.g[A0][0] = lw(c, at(c, SP, 0x90));
        c.g[A3][0] = sx(at(c, SP, 0x50));
        c.g[T3][0] = lw(c, at(c, SP, 0x98));
        c.g[T0][0] = c.g[SP][0];
        c.swc1(0, at(c, SP, 0x50));
        c.g[RA][0] = sx(0x001f2428);
        c.swc1(1, at(c, SP, 0x54));
        if callee::C001f10f8.call(c, 0x001f2428).is_err() {
            return;
        }
        let t = c.g[S3][0] == 0u64;
        c.lwc1(1, at(c, SP, 0x40));
        if t {
            break 'b_1f244c;
        } else {
            c.cycles += 7;
            c.lwc1(3, at(c, S3, 0x4));
            c.lwc1(2, r32(c, S3));
            c.lwc1(0, at(c, SP, 0x44));
            c.add_s(1, 1, 2);
            c.add_s(0, 0, 3);
            c.swc1(1, at(c, SP, 0x40));
            c.swc1(0, at(c, SP, 0x44));
            break 'b_1f244c;
        }
    }
    'b_1f2470: {
        c.cycles += 2;
        let t = c.g[S6][0] == 0u64;
        c.lwc1(1, at(c, SP, 0x40));
        if t {
            break 'b_1f2470;
        } else {
            c.cycles += 7;
            c.lwc1(2, at(c, SP, 0x44));
            c.lwc1(3, at(c, SP, 0x38));
            c.lwc1(0, at(c, SP, 0x3c));
            c.swc1(1, r32(c, S6));
            c.swc1(0, at(c, S6, 0xc));
            c.swc1(2, at(c, S6, 0x4));
            c.swc1(3, at(c, S6, 0x8));
            break 'b_1f2470;
        }
    }
    'b_1f24bc: {
        'b_1f2498: {
            c.cycles += 3;
            c.g[A0][0] = lw(c, at(c, SP, 0x140));
            let t = c.g[A0][0] != 0u64;
            c.g[V1][0] = lw(c, at(c, SP, 0x140));
            if t {
                break 'b_1f2498;
            } else {
                c.cycles += 7;
                c.g[A0][0] = 0x24;
                c.g[A1][0] = sx(at(c, SP, 0x60));
                c.mult(0, V0, S5, A0);
                c.g[RA][0] = sx(0x001f2490);
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
                if callee::C001e91a8.call(c, 0x001f2490).is_err() {
                    return;
                }
                let t = true;
                c.g[A2][0] = 0x24;
                if t {
                    break 'b_1f24bc;
                } else {
                    break 'b_1f2498;
                }
            }
        }
        c.cycles += 9;
        ldl(c, 4, r32(c, V1));
        ldl(c, 5, at(c, V1, 0x8));
        sdl(c, 4, at(c, SP, 0x60));
        sdl(c, 5, at(c, SP, 0x68));
        c.g[A2][0] = 0x24;
        break 'b_1f24bc;
    }
    c.cycles += 72;
    c.lwc1(0, at(c, SP, 0x60));
    c.mult(0, V0, S5, A2);
    sw(c, at(c, SP, 0x74), 0u64);
    c.swc1(0, at(c, SP, 0x70));
    c.g[T0][0] = 0xfffffffffffffc1f;
    sw(c, at(c, SP, 0x78), 0u64);
    sw(c, at(c, SP, 0x7c), 0u64);
    c.g[A2][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
    c.g[A1][0] = lw(c, at(c, SP, 0x94));
    c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
    c.g[V1][0] = lb(c, at(c, A2, 0x22));
    c.g[A3][0] = lw(c, at(c, V0, 0xe84));
    c.g[V1][0] = sx(r32(c, V1) << 5);
    c.g[V0][0] = lw(c, at(c, A3, 0x8));
    c.g[V1][0] = c.g[V1][0] & 0x3e0;
    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    sw(c, at(c, A3, 0x8), c.g[V0][0]);
    ldl(c, 3, at(c, SP, 0x60));
    ldl(c, 6, at(c, SP, 0x68));
    sdl(c, 3, at(c, S1, 0x40));
    sdl(c, 6, at(c, S1, 0x48));
    c.lwc1(12, at(c, SP, 0x40));
    c.lwc1(13, at(c, SP, 0x44));
    ldl(c, 3, at(c, SP, 0x70));
    ldl(c, 6, at(c, SP, 0x78));
    sdl(c, 3, at(c, S1, 0x50));
    sdl(c, 6, at(c, S1, 0x58));
    c.g[RA][0] = sx(0x001f2554);
    c.g[A0][0] = c.g[S1][0];
    if call_ptr(c, 0x00391e30, 0x001f2554).is_err() {
        return;
    }
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    sw(c, at(c, SP, 0x84), 0u64);
    c.swc1(0, at(c, SP, 0x80));
    sw(c, at(c, SP, 0x88), 0u64);
    sw(c, at(c, SP, 0x8c), 0u64);
    c.lwc1(3, r32(c, SP));
    ldl(c, 3, at(c, SP, 0x80));
    ldl(c, 4, at(c, SP, 0x88));
    sdl(c, 3, at(c, S1, 0x50));
    sdl(c, 4, at(c, S1, 0x58));
    frame.restore(c, S0, 0x130);
    c.lwc1(2, at(c, SP, 0x4));
    c.lwc1(0, at(c, S1, 0x30));
    c.lwc1(1, at(c, S1, 0x34));
    c.mul_s(0, 0, 3);
    frame.restore(c, S2, 0x110);
    c.mul_s(1, 1, 2);
    frame.restore(c, S3, 0x100);
    frame.restore(c, S4, 0xf0);
    c.swc1(0, at(c, S1, 0x38));
    c.swc1(1, at(c, S1, 0x3c));
    frame.restore(c, S1, 0x120);
    frame.restore(c, S5, 0xe0);
    frame.restore(c, S6, 0xd0);
    frame.restore(c, S7, 0xc0);
    lq(c, 30, at(c, SP, 0xb0));
    c.g[RA][0] = ld(c, at(c, SP, 0xa0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x140));
    c.pc = t;
    return;
}

/// `int func_001E1AD0(sRangeK1E1AD0* r, const char* arg)` (0x1e1ad0): pick a
/// random foreshadow candidate message. Allocates an index list, walks
/// `r->start .. r->start + r->count` entries of `D_004C6C08`, keeping those
/// whose character (`+0xc`) does not match `arg` and which
/// `MessageCenter_FindMessage` reports as unused (`-1`), then returns
/// `0x100` when none matched or `list[BXrand() % n]` otherwise.
pub fn message_center_pick_foreshadow_message(c: &mut Cpu) {
    let frame = Frame::open(c, 0x90);
    'b_1e1ba4: {
        c.cycles += 27;
        c.g[A2][0] = 0x100;
        frame.save(c, S0, 0x80);
        c.g[A3][0] = 0u64;
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = 0u64;
        frame.save(c, S6, 0x20);
        frame.save(c, S1, 0x70);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S2, 0x60);
        c.g[A1][0] = 0x470000;
        frame.save(c, S7, 0x10);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x2208));
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[A0][0] = lw(c, at(c, S3, 0x8));
        c.g[RA][0] = sx(0x001e1b20);
        c.g[A0][0] = sx(r32(c, A0) << 2);
        if call_ptr(c, 0x00317e30, 0x001e1b20).is_err() {
            return;
        }
        c.g[S5][0] = c.g[V0][0];
        c.g[S0][0] = lw(c, at(c, S3, 0x4));
        c.g[V0][0] = lw(c, at(c, S3, 0x8));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x18;
        if t {
            break 'b_1e1ba4;
        } else {
            c.cycles += 7;
            c.g[V1][0] = 0x4c0000;
            c.mult(0, V0, S0, V0);
            c.g[V1][0] = sx(at(c, V1, 0x6c08));
            c.g[S7][0] = 0xffffffffffffffff;
            c.g[S2][0] = c.g[S5][0];
            c.g[V0][0] = sx(at(c, V0, 0xc));
            c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            'l_1e1b58: loop {
                'b_1e1b8c: {
                    c.cycles += 5;
                    c.g[A0][0] = lw(c, r32(c, S1));
                    c.g[RA][0] = sx(0x001e1b64);
                    c.g[A1][0] = c.g[S6][0];
                    if callee::C001e1a30.call(c, 0x001e1b64).is_err() {
                        return;
                    }
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = lw(c, at(c, S3, 0x8));
                        break 'b_1e1b8c;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e1b74);
                        c.g[A0][0] = c.g[S0][0];
                        if callee::C001e3388.call(c, 0x001e1b74).is_err() {
                            return;
                        }
                        if c.g[V0][0] != c.g[S7][0] {
                            c.g[V0][0] = lw(c, at(c, S3, 0x8));
                            break 'b_1e1b8c;
                        } else {
                            c.cycles += 4;
                            sw(c, r32(c, S2), c.g[S0][0]);
                            c.g[S4][0] = sx(at(c, S4, 0x1));
                            c.g[S2][0] = sx(at(c, S2, 0x4));
                            c.g[V0][0] = lw(c, at(c, S3, 0x8));
                            break 'b_1e1b8c;
                        }
                    }
                }
                c.cycles += 6;
                c.g[S0][0] = sx(at(c, S0, 0x1));
                c.g[V1][0] = lw(c, at(c, S3, 0x4));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S1][0] = sx(at(c, S1, 0x18));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001e1b58);
                    }
                    continue 'l_1e1b58;
                } else {
                    break 'b_1e1ba4;
                }
            }
        }
    }
    'b_1e1be0: {
        c.cycles += 2;
        let t = c.g[S4][0] == 0u64;
        c.g[V0][0] = 0x100;
        if t {
            break 'b_1e1be0;
        } else {
            'b_1e1bc0: {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001e1bb4);
                if call_ptr(c, 0x003177f0, 0x001e1bb4).is_err() {
                    return;
                }
                c.divu(0, 2, 20);
                if c.g[S4][0] == 0u64 {
                    break 'b_1e1bc0;
                } else {
                    break 'b_1e1bc0;
                }
            }
            'b_1e1bdc: {
                c.cycles += 5;
                c.g[V1][0] = c.hi[0];
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S5)));
                let t = c.g[S5][0] == 0u64;
                c.g[S0][0] = lw(c, r32(c, V1));
                if t {
                    break 'b_1e1bdc;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001e1bdc);
                    c.g[A0][0] = c.g[S5][0];
                    if call_ptr(c, 0x00317e98, 0x001e1bdc).is_err() {
                        return;
                    }
                    break 'b_1e1bdc;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
            break 'b_1e1be0;
        }
    }
    c.cycles += 11;
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, S6, 0x20);
    frame.restore(c, S7, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x90));
    c.pc = t;
    return;
}

/// `void func_00225CA0(cLunoValue* self, sVEntry00224C00** stream, int base)`
/// (0x225ca0): read one Luno value from `stream` into `self`. A stream type of
/// 5 reads a pair, 4 a table, anything else is stored as the value type
/// after calling the stream's "read" slot; a previous table value at `self`
/// is destroyed first.
pub fn luno_vm_register_read(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_225dcc: {
        'b_225dc8: {
            'b_225d44: {
                c.cycles += 20;
                frame.save(c, S0, 0x60);
                frame.save(c, S1, 0x50);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x40);
                c.g[S1][0] = c.g[A1][0];
                frame.save(c, S3, 0x30);
                c.g[A1][0] = sx(at(c, SP, 0x10));
                frame.save_ra(c, 0x20);
                c.g[S3][0] = c.g[A2][0];
                c.g[A2][0] = 0x4;
                c.g[V1][0] = lw(c, r32(c, S1));
                c.g[A0][0] = lh(c, at(c, V1, 0x10));
                c.g[V0][0] = lw(c, at(c, V1, 0x14));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00225ce0);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00225ce0).is_err() {
                    return;
                }
                c.g[S2][0] = lw(c, at(c, SP, 0x10));
                c.g[V0][0] = 0x5;
                let t = c.g[S2][0] != c.g[V0][0];
                c.g[V0][0] = 0x4;
                if t {
                    break 'b_225d44;
                } else {
                    'b_225d28: {
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[S3][0];
                        sw(c, r32(c, SP), 0u64);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00225d08);
                        sw(c, at(c, SP, 0x4), 0u64);
                        if callee::C00224c78.call(c, 0x00225d08).is_err() {
                            return;
                        }
                        c.g[V1][0] = lw(c, at(c, S0, 0x8));
                        c.g[V0][0] = 0x4;
                        if c.g[V1][0] != c.g[V0][0] {
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d28;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00225d24);
                            c.g[A1][0] = 0x2;
                            if callee::C00224df0.call(c, 0x00225d24).is_err() {
                                return;
                            }
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d28;
                        }
                    }
                    c.cycles += 7;
                    sw(c, r32(c, S0), 0u64);
                    ldl(c, 2, r32(c, SP));
                    sdl(c, 2, r32(c, S0));
                    let t = true;
                    sw(c, at(c, S0, 0x8), c.g[S2][0]);
                    if t {
                        break 'b_225dc8;
                    } else {
                        break 'b_225d44;
                    }
                }
            }
            'b_225dac: {
                c.cycles += 2;
                if c.g[S2][0] != c.g[V0][0] {
                    sw(c, at(c, S0, 0x8), c.g[S2][0]);
                    break 'b_225dac;
                } else {
                    'b_225d84: {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00225d58);
                        c.g[A1][0] = 0x20;
                        if callee::C00224da0.call(c, 0x00225d58).is_err() {
                            return;
                        }
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x00225d68);
                        c.g[A0][0] = c.g[SP][0];
                        if callee::C002259a8.call(c, 0x00225d68).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw(c, at(c, S0, 0x8));
                        if c.g[V0][0] != c.g[S2][0] {
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d84;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00225d80);
                            c.g[A1][0] = 0x2;
                            if callee::C00224df0.call(c, 0x00225d80).is_err() {
                                return;
                            }
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d84;
                        }
                    }
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S0][0];
                    sw(c, r32(c, S0), 0u64);
                    c.g[RA][0] = sx(0x00225d94);
                    c.g[A1][0] = c.g[SP][0];
                    if callee::C00225068.call(c, 0x00225d94).is_err() {
                        return;
                    }
                    sw(c, at(c, S0, 0x8), c.g[S2][0]);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00225da4);
                    c.g[A1][0] = 0x2;
                    if callee::C00224df0.call(c, 0x00225da4).is_err() {
                        return;
                    }
                    let t = true;
                    frame.restore(c, S0, 0x60);
                    if t {
                        break 'b_225dcc;
                    } else {
                        break 'b_225dac;
                    }
                }
            }
            c.cycles += 7;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x4;
            c.g[V0][0] = lw(c, r32(c, S1));
            c.g[A0][0] = lh(c, at(c, V0, 0x10));
            c.g[V1][0] = lw(c, at(c, V0, 0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00225dc8);
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x00225dc8).is_err() {
                return;
            }
            break 'b_225dc8;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x60);
        break 'b_225dcc;
    }
    c.cycles += 6;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}

/// `void func_0012C408(void* gate, void* a1)` (0x12c408): per-frame launch
/// gate charge update. Resets the rider's gate-charge fields to the charge
/// angle, ramps the charge at `+0x10` towards 1 by 0.015 (clamped), sets the
/// slew targets, and when the rider is launched (state 3, charge >= 0.62)
/// scales the rider's rotation and re-enters the idle state; at exactly 1.0
/// it plays the gate animation and switches the sub-state.
pub fn rider_gate_sub_update_launch(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_12c46c: {
        'b_12c450: {
            c.cycles += 16;
            frame.save(c, S0, 0x40);
            frame.save_ra(c, 0x30);
            c.g[S0][0] = c.g[A0][0];
            c.g[RA][0] = sx(0x0012c420);
            c.g[A0][0] = lw(c, at(c, S0, 0x14));
            if callee::C00115ab0.call(c, 0x0012c420).is_err() {
                return;
            }
            c.g[V0][0] = lw(c, at(c, S0, 0x14));
            c.lwc1(1, r32(c, GP).wrapping_sub(0x780c));
            c.swc1(0, at(c, V0, 0x2c8));
            sw(c, at(c, V0, 0x2cc), 0u64);
            c.swc1(0, at(c, V0, 0x2d0));
            c.lwc1(3, r32(c, GP).wrapping_sub(0x7808));
            c.lwc1(2, at(c, S0, 0x10));
            c.c_lt_s(1, 2);
            let t = !c.fcc();

            if t {
                break 'b_12c450;
            } else {
                c.cycles += 2;
                let t = true;
                c.sub_s(1, 2, 3);
                if t {
                    break 'b_12c46c;
                } else {
                    break 'b_12c450;
                }
            }
        }
        c.cycles += 7;
        c.lwc1(0, r32(c, GP).wrapping_sub(0x7804));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 1);
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.add_s(1, 2, 3);
            break 'b_12c46c;
        } else {
            break 'b_12c46c;
        }
    }
    'b_12c4e8: {
        c.cycles += 11;
        c.g[A0][0] = lw(c, at(c, S0, 0x14));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 12);
        c.mtc1(0, 13);
        c.g[RA][0] = sx(0x0012c484);
        c.swc1(1, at(c, S0, 0x10));
        if callee::C00113f88.call(c, 0x0012c484).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x0012c48c);
        c.g[A0][0] = lw(c, at(c, S0, 0x14));
        if callee::C0011fe98.call(c, 0x0012c48c).is_err() {
            return;
        }
        c.g[V1][0] = 0x3;
        let t = c.g[V0][0] != c.g[V1][0];
        c.lwc1(1, at(c, S0, 0x10));
        if t {
            break 'b_12c4e8;
        } else {
            c.cycles += 5;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x7800));
            c.c_le_s(0, 1);
            let t = !c.fcc();
            c.g[A1][0] = 0u64;
            if t {
                break 'b_12c4e8;
            } else {
                c.cycles += 15;
                c.g[V1][0] = lw(c, at(c, S0, 0x14));
                c.lwc1(0, at(c, S0, 0xc));
                lq(c, 2, at(c, V1, 0x1b0));
                sq(c, 2, at(c, SP, 0x10));
                c.mfc1(4, 0);
                c.lqc2(4, at(c, SP, 0x10));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x20));
                lq(c, 2, at(c, SP, 0x20));
                sq(c, 2, r32(c, SP));
                sq(c, 2, at(c, V1, 0x1e0));
                c.g[RA][0] = sx(0x0012c4e4);
                c.g[A0][0] = lw(c, at(c, S0, 0x14));
                if callee::C0011fe78.call(c, 0x0012c4e4).is_err() {
                    return;
                }
                c.lwc1(1, at(c, S0, 0x10));
                break 'b_12c4e8;
            }
        }
    }
    'b_12c548: {
        'b_12c544: {
            c.cycles += 6;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.c_eq_s(1, 0);
            let t = !c.fcc();
            c.g[A2][0] = 0u64;
            if t {
                break 'b_12c544;
            } else {
                'b_12c53c: {
                    c.cycles += 11;
                    c.g[V0][0] = lw(c, at(c, S0, 0x14));
                    c.g[A1][0] = 0x6;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 12);
                    c.g[RA][0] = sx(0x0012c518);
                    c.g[A0][0] = lw(c, at(c, V0, 0x784));
                    if call_ptr(c, 0x003128e8, 0x0012c518).is_err() {
                        return;
                    }
                    c.g[RA][0] = sx(0x0012c520);
                    c.g[A0][0] = lw(c, at(c, S0, 0x14));
                    if callee::C0011fe98.call(c, 0x0012c520).is_err() {
                        return;
                    }
                    c.g[V1][0] = 0x1;
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[A0][0] = lw(c, at(c, S0, 0x14));
                    if t {
                        break 'b_12c53c;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0012c534);
                        c.g[A1][0] = 0x4;
                        if callee::C0011fec8.call(c, 0x0012c534).is_err() {
                            return;
                        }
                        let t = true;
                        frame.restore(c, S0, 0x40);
                        if t {
                            break 'b_12c548;
                        } else {
                            break 'b_12c53c;
                        }
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x0012c544);
                c.g[A1][0] = 0u64;
                if callee::C0011fec8.call(c, 0x0012c544).is_err() {
                    return;
                }
                break 'b_12c544;
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x40);
        break 'b_12c548;
    }
    c.cycles += 3;
    c.g[RA][0] = ld(c, at(c, SP, 0x30));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x50));
    c.pc = t;
    return;
}

/// `void func_0039BBD8(void* pair, int on)` (0x39bbd8): show/hide a UI pair.
/// Stores `on` in bit 4 of the flags at `+0x14`, tells the entry at `+0x78`
/// (`+8` vtable, slot 7), and, when there is a keyframer object at `+0x7c`,
/// looks up its event for the wanted state, plays that frame on the screen at
/// `+0x5c` when one is found, and finally re-sets the keyframer and the owner
/// (slot 19 with 1 or 2).
pub fn ui_pair_set_focus(c: &mut Cpu) {
    let frame = Frame::open(c, 0x30);
    'b_39bc28: {
        c.cycles += 15;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
        c.g[A0][0] = 0xffffffffffffffef;
        c.g[V1][0] = lw(c, at(c, S0, 0x14));
        c.g[V0][0] = sx(r32(c, V0) << 4);
        c.g[A2][0] = lw(c, at(c, S0, 0x78));
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        let t = c.g[A2][0] == 0u64;
        sw(c, at(c, S0, 0x14), c.g[V1][0]);
        if t {
            break 'b_39bc28;
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw(c, at(c, A2, 0x8));
            c.g[A0][0] = lh(c, at(c, V0, 0x38));
            c.g[V1][0] = lw(c, at(c, V0, 0x3c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039bc28);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0039bc28).is_err() {
                return;
            }
            break 'b_39bc28;
        }
    }
    'b_39bd28: {
        c.cycles += 3;
        c.g[A0][0] = lw(c, at(c, S0, 0x7c));
        if c.g[A0][0] == 0u64 {
            frame.restore(c, S0, 0x20);
            break 'b_39bd28;
        } else {
            'b_39bcb4: {
                c.cycles += 2;
                let t = c.g[S1][0] == 0u64;

                if t {
                    break 'b_39bcb4;
                } else {
                    'b_39bc70: {
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x0039bc44);
                        c.g[A1][0] = 0u64;
                        if callee::C0039ff78.call(c, 0x0039bc44).is_err() {
                            return;
                        }
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        if c.g[A1][0] == c.g[V0][0] {
                            c.g[A2][0] = lw(c, at(c, S0, 0x7c));
                            break 'b_39bc70;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = lw(c, at(c, S0, 0x7c));
                            c.g[A0][0] = lw(c, at(c, V0, 0x5c));
                            if c.g[A0][0] == 0u64 {
                                c.g[A2][0] = lw(c, at(c, S0, 0x7c));
                                break 'b_39bc70;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x0039bc6c);
                                c.g[A2][0] = 0x1;
                                if callee::C0039c870.call(c, 0x0039bc6c).is_err() {
                                    return;
                                }
                                c.g[A2][0] = lw(c, at(c, S0, 0x7c));
                                break 'b_39bc70;
                            }
                        }
                    }
                    c.cycles += 17;
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = lw(c, at(c, A2, 0x8));
                    c.g[A0][0] = lh(c, at(c, V0, 0x38));
                    c.g[V1][0] = lw(c, at(c, V0, 0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0039bc88);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0039bc88).is_err() {
                        return;
                    }
                    c.g[A0][0] = lw(c, at(c, S0, 0x5c));
                    c.g[A2][0] = 0x1;
                    c.g[A1][0] = lw(c, at(c, S0, 0x7c));
                    c.g[A3][0] = lw(c, at(c, A0, 0xd0));
                    c.g[V0][0] = lw(c, at(c, A3, 0x8));
                    c.g[A0][0] = lh(c, at(c, V0, 0x98));
                    c.g[V1][0] = lw(c, at(c, V0, 0x9c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0039bcac);
                    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0039bcac).is_err() {
                        return;
                    }
                    let t = true;
                    frame.restore(c, S0, 0x20);
                    if t {
                        break 'b_39bd28;
                    } else {
                        break 'b_39bcb4;
                    }
                }
            }
            'b_39bce8: {
                c.cycles += 6;
                c.g[RA][0] = sx(0x0039bcbc);
                c.g[A1][0] = 0x1;
                if callee::C0039ff78.call(c, 0x0039bcbc).is_err() {
                    return;
                }
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                if c.g[A1][0] == c.g[V0][0] {
                    c.g[A2][0] = lw(c, at(c, S0, 0x7c));
                    break 'b_39bce8;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = lw(c, at(c, S0, 0x7c));
                    c.g[A0][0] = lw(c, at(c, V0, 0x5c));
                    if c.g[A0][0] == 0u64 {
                        c.g[A2][0] = lw(c, at(c, S0, 0x7c));
                        break 'b_39bce8;
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x0039bce4);
                        c.g[A2][0] = 0x1;
                        if callee::C0039c870.call(c, 0x0039bce4).is_err() {
                            return;
                        }
                        c.g[A2][0] = lw(c, at(c, S0, 0x7c));
                        break 'b_39bce8;
                    }
                }
            }
            c.cycles += 16;
            c.g[A1][0] = 0u64;
            c.g[V0][0] = lw(c, at(c, A2, 0x8));
            c.g[A0][0] = lh(c, at(c, V0, 0x38));
            c.g[V1][0] = lw(c, at(c, V0, 0x3c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039bd00);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0039bd00).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at(c, S0, 0x5c));
            c.g[A2][0] = 0x2;
            c.g[A1][0] = lw(c, at(c, S0, 0x7c));
            c.g[A3][0] = lw(c, at(c, A0, 0xd0));
            c.g[V0][0] = lw(c, at(c, A3, 0x8));
            c.g[A0][0] = lh(c, at(c, V0, 0x98));
            c.g[V1][0] = lw(c, at(c, V0, 0x9c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0039bd24);
            c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0039bd24).is_err() {
                return;
            }
            frame.restore(c, S0, 0x20);
            break 'b_39bd28;
        }
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x30));
    c.pc = t;
    return;
}

/// `int func_00157BF0(void* self, int a, int b, int c, int k)` (0x157bf0):
/// is the peak's goal complete for medal `k`? Kind 0/1 walks the five tracks
/// of `D_0045AAD8[k][k]` and compares their medals; kind 2 compares the
/// collectible and mission tiers; kind 3 compares the earnings medal;
/// anything else is false.
pub fn be_reward_int_is_peak_goal_complete(c: &mut Cpu) {
    let frame = Frame::open(c, 0x60);
    'b_157d40: {
        'b_157d3c: {
            'b_157d38: {
                'b_157d0c: {
                    'b_157cb0: {
                        c.cycles += 13;
                        c.g[V0][0] = 0x2;
                        frame.save(c, S0, 0x50);
                        frame.save(c, S1, 0x40);
                        c.g[S0][0] = c.g[A3][0];
                        frame.save(c, S3, 0x20);
                        c.g[S1][0] = c.g[A0][0];
                        frame.save(c, S4, 0x10);
                        c.g[S3][0] = c.g[A1][0];
                        frame.save(c, S2, 0x30);
                        c.g[S4][0] = c.g[A2][0];
                        let t = c.g[T0][0] == c.g[V0][0];
                        sd(c, r32(c, SP), c.g[RA][0]);
                        if t {
                            break 'b_157cb0;
                        } else {
                            'b_157c40: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[T0][0] as i64) < 3i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x3;
                                if t {
                                    break 'b_157c40;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[T0][0] == c.g[V0][0];
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_157d0c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        frame.restore(c, S0, 0x50);
                                        if t {
                                            break 'b_157d40;
                                        } else {
                                            break 'b_157c40;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = (c.g[T0][0] as i64) < 0;
                            c.g[V1][0] = 0xa;
                            if t {
                                break 'b_157d38;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x28;
                                c.mult(0, A0, S0, A0);
                                c.mult(0, V1, T0, V1);
                                c.g[V0][0] = 0x460000;
                                c.g[S2][0] = 0u64;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5528));
                                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                                c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                                'l_157c68: loop {
                                    'b_157ca8: {
                                        c.cycles += 3;
                                        c.g[A3][0] = lb(c, r32(c, S0));
                                        let t = (c.g[A3][0] as i64) < 0;
                                        c.g[T0][0] = lb(c, at(c, S0, 0x1));
                                        if t {
                                            break 'b_157ca8;
                                        } else {
                                            c.cycles += 10;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[RA][0] = sx(0x00157c84);
                                            c.g[A2][0] = c.g[S4][0];
                                            if callee::C00158910.call(c, 0x00157c84).is_err() {
                                                return;
                                            }
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[RA][0] = sx(0x00157c94);
                                            c.g[A1][0] = 0xffffffffffffffff;
                                            if callee::C00156a60.call(c, 0x00157c94).is_err() {
                                                return;
                                            }
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[S2][0] = sx(at(c, S2, 0x1));
                                            if t {
                                                break 'b_157d38;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[S0][0] = sx(at(c, S0, 0x2));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x00157c68);
                                                    }
                                                    continue 'l_157c68;
                                                } else {
                                                    break 'b_157ca8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_157d3c;
                                    } else {
                                        break 'b_157cb0;
                                    }
                                }
                            }
                        }
                    }
                    'b_157d04: {
                        c.cycles += 12;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S3][0];
                        c.g[A2][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x00157cc4);
                        c.g[A3][0] = c.g[S0][0];
                        if callee::C001589b0.call(c, 0x00157cc4).is_err() {
                            return;
                        }
                        c.g[S2][0] = 0u64;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00157cd8);
                        c.g[A1][0] = 0xffffffffffffffff;
                        if callee::C00156a60.call(c, 0x00157cd8).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[S3][0];
                        if t {
                            break 'b_157d04;
                        } else {
                            c.cycles += 9;
                            c.g[A2][0] = c.g[S4][0];
                            c.g[A3][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00157cf0);
                            c.g[A0][0] = c.g[S1][0];
                            if callee::C00158a50.call(c, 0x00157cf0).is_err() {
                                return;
                            }
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00157d00);
                            c.g[A1][0] = 0xffffffffffffffff;
                            if callee::C00156a60.call(c, 0x00157d00).is_err() {
                                return;
                            }
                            c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
                            break 'b_157d04;
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.g[S2][0];
                    if t {
                        break 'b_157d3c;
                    } else {
                        break 'b_157d0c;
                    }
                }
                c.cycles += 11;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00157d20);
                c.g[A3][0] = c.g[S0][0];
                if callee::C00158af0.call(c, 0x00157d20).is_err() {
                    return;
                }
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00157d30);
                c.g[A1][0] = 0xffffffffffffffff;
                if callee::C00156a60.call(c, 0x00157d30).is_err() {
                    return;
                }
                let t = true;
                frame.restore(c, S0, 0x50);
                if t {
                    break 'b_157d40;
                } else {
                    break 'b_157d38;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_157d3c;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x50);
        break 'b_157d40;
    }
    c.cycles += 7;
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x60));
    c.pc = t;
    return;
}

/// `void func_0028FEA0(cMIDIBanks* self, int flags)` (0x28fea0): destroying
/// destructor of the MIDI bank object: install the base vtables, free the
/// two owned buffers, clear the global, and for `flags & 2` tear down the
/// sound manager, for `flags & 1` free `self`.
pub fn midi_banks_base_dtor(c: &mut Cpu) {
    let frame = Frame::open(c, 0x90);
    'b_28ff9c: {
        c.cycles += 23;
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x80);
        c.g[V0][0] = sx(at(c, V0, 0x3a48));
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x60);
        c.g[A3][0] = 0x480000;
        sw(c, at(c, S0, 0xc), c.g[V0][0]);
        c.g[A2][0] = sx(at(c, A3, 0x3a60));
        c.g[V1][0] = lw(c, r32(c, S0));
        c.g[A0][0] = 0x480000;
        c.g[A0][0] = sx(at(c, A0, 0x3b48));
        c.g[T0][0] = 0x480000;
        c.g[V0][0] = lw(c, at(c, V1, 0x1d8));
        c.g[T5][0] = sx(at(c, T0, 0x3a80));
        c.g[S1][0] = c.g[A1][0];
        sw(c, at(c, V0, 0x4), c.g[A2][0]);
        c.g[V0][0] = lw(c, r32(c, S0));
        sw(c, at(c, V0, 0xab0), c.g[A0][0]);
        c.g[V1][0] = lw(c, r32(c, S0));
        let t = c.g[S1][0] != 0u64;
        sw(c, at(c, V1, 0x1d4), c.g[T5][0]);
        if t {
            break 'b_28ff9c;
        } else {
            c.cycles += 40;
            c.g[A1][0] = ld(c, at(c, A3, 0x3a60));
            c.g[T4][0] = sx(at(c, SP, 0x20));
            c.g[V1][0] = ld(c, at(c, A2, 0x10));
            c.g[A0][0] = ld(c, at(c, A2, 0x18));
            c.g[V0][0] = ld(c, at(c, A2, 0x8));
            sd(c, r32(c, SP), c.g[A1][0]);
            sd(c, at(c, SP, 0x18), c.g[A0][0]);
            sd(c, at(c, SP, 0x10), c.g[V1][0]);
            sd(c, at(c, SP, 0x8), c.g[V0][0]);
            c.g[V0][0] = lw(c, r32(c, S0));
            c.g[A0][0] = lhu(c, at(c, A2, 0x8));
            c.g[V1][0] = lw(c, at(c, V0, 0x1d8));
            c.g[T3][0] = ld(c, at(c, T0, 0x3a80));
            sw(c, at(c, V1, 0x4), c.g[SP][0]);
            c.g[V1][0] = ld(c, at(c, T5, 0x28));
            c.g[T1][0] = lw(c, r32(c, S0));
            c.g[A3][0] = ld(c, at(c, T5, 0x8));
            c.g[V0][0] = lw(c, at(c, T1, 0x1d8));
            c.g[T2][0] = ld(c, at(c, T5, 0x10));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x10));
            c.g[T0][0] = ld(c, at(c, T5, 0x18));
            c.g[V0][0] = sx(r32(c, S0).wrapping_sub(r32(c, V0)));
            c.g[A2][0] = ld(c, at(c, T5, 0x20));
            c.g[A1][0] = ld(c, at(c, T5, 0x30));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            sd(c, at(c, SP, 0x28), c.g[A3][0]);
            sd(c, at(c, SP, 0x48), c.g[V1][0]);
            sh(c, at(c, SP, 0x8), c.g[A0][0]);
            sd(c, at(c, SP, 0x20), c.g[T3][0]);
            sd(c, at(c, SP, 0x30), c.g[T2][0]);
            sd(c, at(c, SP, 0x38), c.g[T0][0]);
            sd(c, at(c, SP, 0x40), c.g[A2][0]);
            sd(c, at(c, SP, 0x50), c.g[A1][0]);
            sw(c, at(c, T1, 0x1d4), c.g[T4][0]);
            c.g[V1][0] = lhu(c, at(c, T5, 0x8));
            c.g[V0][0] = lw(c, r32(c, S0));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x18));
            c.g[V0][0] = sx(r32(c, S0).wrapping_sub(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            sh(c, at(c, SP, 0x28), c.g[V1][0]);
            break 'b_28ff9c;
        }
    }
    'b_28ffb4: {
        c.cycles += 3;
        c.g[A0][0] = lw(c, at(c, S0, 0x4));
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at(c, S0, 0x8));
            break 'b_28ffb4;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0028ffb0);
            if call_ptr(c, 0x00317e98, 0x0028ffb0).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at(c, S0, 0x8));
            break 'b_28ffb4;
        }
    }
    'b_28ffc8: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = c.g[S1][0] & 0x2;
        if t {
            break 'b_28ffc8;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0028ffc4);
            if call_ptr(c, 0x00317e98, 0x0028ffc4).is_err() {
                return;
            }
            c.g[V0][0] = c.g[S1][0] & 0x2;
            break 'b_28ffc8;
        }
    }
    'b_28ffec: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        sw(c, at(c, GP, 0x574), 0u64);
        if t {
            break 'b_28ffec;
        } else {
            c.cycles += 7;
            c.g[A0][0] = lw(c, r32(c, S0));
            c.g[RA][0] = sx(0x0028ffdc);
            c.g[A1][0] = 0u64;
            if callee::C0028bb10.call(c, 0x0028ffdc).is_err() {
                return;
            }
            c.g[V0][0] = lw(c, r32(c, S0));
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x0028ffec);
            c.g[A0][0] = lw(c, at(c, V0, 0x1d8));
            if call_ptr(c, 0x002adee8, 0x0028ffec).is_err() {
                return;
            }
            break 'b_28ffec;
        }
    }
    'b_290004: {
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0u64 {
            frame.restore(c, S0, 0x80);
            break 'b_290004;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x00290000);
            c.g[A0][0] = c.g[S0][0];
            if call_ptr(c, 0x00317e50, 0x00290000).is_err() {
                return;
            }
            frame.restore(c, S0, 0x80);
            break 'b_290004;
        }
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x70);
    c.g[RA][0] = ld(c, at(c, SP, 0x60));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x90));
    c.pc = t;
    return;
}

/// `void func_0025D538(LobbyMan* self, sMessage* msg)` (0x25d538): handle the
/// reply to "delete persona": mark the screen busy and post either the
/// "deleted" (msg+8 == 0) or the "failed" message, shifting the remaining
/// persona names down in the first case.
pub fn lobby_man_on_delete_persona_reply(c: &mut Cpu) {
    let frame = Frame::open(c, 0x80);
    'b_25d664: {
        'b_25d614: {
            c.cycles += 14;
            c.g[V0][0] = 0x1;
            frame.save(c, S3, 0x40);
            frame.save(c, S1, 0x60);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S0, 0x70);
            c.g[S1][0] = c.g[A1][0];
            frame.save(c, S2, 0x50);
            frame.save(c, S4, 0x30);
            frame.save_ra(c, 0x20);
            sw(c, r32(c, S3), c.g[V0][0]);
            c.g[V0][0] = lw(c, at(c, S1, 0x8));
            let t = c.g[V0][0] != 0u64;
            c.g[A1][0] = 0x480000;
            if t {
                break 'b_25d614;
            } else {
                'b_25d5c8: {
                    c.cycles += 7;
                    c.g[V0][0] = lw(c, at(c, S3, 0x6c));
                    c.g[S4][0] = sx(at(c, S3, 0xec));
                    c.g[S2][0] = lw(c, at(c, S3, 0x50));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                    c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V1][0] == 0u64;
                    sw(c, at(c, S3, 0x6c), c.g[V0][0]);
                    if t {
                        break 'b_25d5c8;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = sx(r32(c, S2) << 2);
                        c.g[V1][0] = sx(at(c, V0, 0x70));
                        c.g[V0][0] = sx(at(c, V0, 0x74));
                        c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
                        c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                        'l_25d5a0: loop {
                            c.cycles += 9;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0025d5ac);
                            c.g[A1][0] = c.g[S1][0];
                            if call_ptr(c, 0x00318448, 0x0025d5ac).is_err() {
                                return;
                            }
                            c.g[S2][0] = sx(at(c, S2, 0x1));
                            c.g[V0][0] = lw(c, at(c, S3, 0x6c));
                            c.g[S1][0] = sx(at(c, S1, 0x4));
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S0][0] = sx(at(c, S0, 0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0025d5a0);
                                }
                                continue 'l_25d5a0;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = 0x480000;
                                break 'b_25d5c8;
                            }
                        }
                    }
                }
                c.cycles += 19;
                c.g[A2][0] = 0x20000000;
                c.g[A1][0] = sx(at(c, A1, 0x488));
                c.g[A0][0] = 0x8;
                c.g[RA][0] = sx(0x0025d5dc);
                c.g[A3][0] = 0u64;
                if call_ptr(c, 0x00317d70, 0x0025d5dc).is_err() {
                    return;
                }
                c.g[S0][0] = c.g[V0][0];
                c.g[A2][0] = lw(c, at(c, S4, 0x4));
                c.g[V0][0] = 0x480000;
                c.g[V1][0] = 0xd8;
                c.g[V0][0] = sx(at(c, V0, 0x12b0));
                sw(c, r32(c, S0), c.g[V1][0]);
                sw(c, at(c, S0, 0x4), c.g[V0][0]);
                c.g[A1][0] = 0x480000;
                sw(c, at(c, SP, 0x10), c.g[A2][0]);
                c.g[A1][0] = sx(at(c, A1, 0x4a8));
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x20000000;
                let t = true;
                c.g[A3][0] = 0u64;
                if t {
                    break 'b_25d664;
                } else {
                    break 'b_25d614;
                }
            }
        }
        c.cycles += 20;
        c.g[A0][0] = 0xc;
        c.g[A1][0] = sx(at(c, A1, 0x488));
        c.g[A2][0] = 0x20000000;
        c.g[RA][0] = sx(0x0025d628);
        c.g[A3][0] = 0u64;
        if call_ptr(c, 0x00317d70, 0x0025d628).is_err() {
            return;
        }
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = lw(c, at(c, S1, 0x8));
        c.g[V0][0] = 0x480000;
        c.g[A2][0] = lw(c, at(c, S3, 0xf0));
        c.g[V0][0] = sx(at(c, V0, 0x1290));
        c.g[V1][0] = 0x102;
        sw(c, at(c, S0, 0x8), c.g[A0][0]);
        c.g[A1][0] = 0x480000;
        sw(c, r32(c, S0), c.g[V1][0]);
        c.g[A1][0] = sx(at(c, A1, 0x4a8));
        sw(c, at(c, S0, 0x4), c.g[V0][0]);
        c.g[A0][0] = 0xc;
        sw(c, at(c, SP, 0x10), c.g[A2][0]);
        c.g[A3][0] = 0u64;
        c.g[A2][0] = 0x20000000;
        break 'b_25d664;
    }
    'b_25d67c: {
        c.cycles += 6;
        c.g[RA][0] = sx(0x0025d66c);
        if call_ptr(c, 0x00317d70, 0x0025d66c).is_err() {
            return;
        }
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at(c, A1, 0x8));
        if c.g[V0][0] != 0u64 {
            sw(c, at(c, A1, 0x8), c.g[S0][0]);
            break 'b_25d67c;
        } else {
            break 'b_25d67c;
        }
    }
    c.cycles += 17;
    c.g[V0][0] = lw(c, at(c, SP, 0x10));
    sw(c, r32(c, A1), c.g[V0][0]);
    c.g[V1][0] = lw(c, at(c, V0, 0x4));
    sw(c, at(c, A1, 0x4), c.g[V1][0]);
    c.g[A0][0] = lw(c, at(c, V0, 0x4));
    sw(c, r32(c, A0), c.g[A1][0]);
    c.g[V0][0] = lw(c, at(c, SP, 0x10));
    sw(c, at(c, V0, 0x4), c.g[A1][0]);
    sw(c, r32(c, SP), c.g[A1][0]);
    frame.restore(c, S0, 0x70);
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x80));
    c.pc = t;
    return;
}

/// `void func_00358998(void* railman, int idx)` (0x358998): enable rail
/// `idx`: fetch its part from the world, disable each collider node's box
/// (inserted into the spatial hash), then mark the reference unused
/// (`-1`).
pub fn rail_man_enable_rail(c: &mut Cpu) {
    let frame = Frame::open(c, 0xc0);
    'b_358a24: {
        'b_358a00: {
            c.cycles += 24;
            frame.save(c, S1, 0x90);
            frame.save(c, S6, 0x40);
            c.g[S1][0] = c.g[A1][0];
            c.g[S6][0] = c.g[A0][0];
            frame.save(c, S2, 0x80);
            frame.save(c, S3, 0x70);
            c.g[V0][0] = sx(r32(c, S1) << 2);
            frame.save(c, S4, 0x60);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S6)));
            frame.save(c, S5, 0x50);
            frame.save_ra(c, 0x30);
            c.swc1(20, at(c, SP, 0xb0));
            frame.save(c, S0, 0xa0);
            c.g[RA][0] = sx(0x003589d8);
            c.g[S0][0] = lw(c, at(c, V0, 0x18));
            if call_ptr(c, 0x002d1bd8, 0x003589d8).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, r32(c, V0));
            c.g[V0][0] = c.g[S0][0] & 0xff;
            c.g[V1][0] = lw(c, at(c, A0, 0x8));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[V1][0] = lw(c, r32(c, V0));
            if c.g[V1][0] != 0u64 {
                c.g[A0][0] = lw(c, at(c, V1, 0x44));
                break 'b_358a00;
            } else {
                c.cycles += 2;
                let t = true;
                c.g[S4][0] = 0u64;
                if t {
                    break 'b_358a24;
                } else {
                    break 'b_358a00;
                }
            }
        }
        'b_358a20: {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, S0) >> 8);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = lw(c, r32(c, V0));
            c.g[V0][0] = sx(r32(c, V1) >> 8);
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            if t {
                break 'b_358a20;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_358a20;
            }
        }
        c.cycles += 1;
        c.g[S4][0] = c.g[V0][0];
        break 'b_358a24;
    }
    'b_358ae8: {
        c.cycles += 5;
        c.g[S5][0] = sx(r32(c, S1) << 2);
        c.g[V0][0] = lw(c, at(c, S4, 0x20));
        c.g[S1][0] = lw(c, at(c, S4, 0x24));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S2][0] = 0u64;
        if t {
            break 'b_358ae8;
        } else {
            c.cycles += 4;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 20);
            c.g[S3][0] = 0xffffffffffffffff;
            'l_358a48: loop {
                c.cycles += 40;
                c.lwc1(1, at(c, S1, 0x6c));
                c.g[S2][0] = sx(at(c, S2, 0x1));
                c.swc1(1, r32(c, SP));
                c.lwc1(0, at(c, S1, 0x70));
                c.swc1(0, at(c, SP, 0x4));
                c.lwc1(1, at(c, S1, 0x74));
                c.swc1(20, at(c, SP, 0xc));
                c.swc1(1, at(c, SP, 0x8));
                c.lwc1(0, at(c, S1, 0x78));
                c.swc1(0, at(c, SP, 0x10));
                c.lwc1(1, at(c, S1, 0x7c));
                c.swc1(1, at(c, SP, 0x14));
                c.lwc1(0, at(c, S1, 0x80));
                c.swc1(20, at(c, SP, 0x1c));
                c.g[RA][0] = sx(0x00358a88);
                c.swc1(0, at(c, SP, 0x18));
                if call_ptr(c, 0x002d1be0, 0x00358a88).is_err() {
                    return;
                }
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx(at(c, SP, 0x20));
                c.g[RA][0] = sx(0x00358a98);
                c.g[A1][0] = c.g[SP][0];
                if call_ptr(c, 0x00328f28, 0x00358a98).is_err() {
                    return;
                }
                c.g[V0][0] = lw(c, at(c, SP, 0x24));
                c.g[V1][0] = sx(at(c, S0, 0x50));
                c.g[A1][0] = lw(c, at(c, SP, 0x28));
                c.g[A2][0] = c.g[S1][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[T0][0] = lw(c, at(c, SP, 0x2c));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[S0][0];
                }
                c.g[A1][0] = ((c.g[S3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[A0][0] = sx(at(c, V1, 0x28));
                c.g[T0][0] = ((c.g[S3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                if c.g[A1][0] != 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
                c.g[A3][0] = sx(at(c, SP, 0x20));
                c.g[V0][0] = sx(at(c, A0, 0x14));
                c.g[A1][0] = 0x2;
                c.g[RA][0] = sx(0x00358ad8);
                if c.g[T0][0] == 0 {
                    c.g[A0][0] = c.g[V0][0];
                }
                if call_ptr(c, 0x00328c20, 0x00358ad8).is_err() {
                    return;
                }
                c.g[V0][0] = lw(c, at(c, S4, 0x20));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S1][0] = lw(c, at(c, S1, 0x64));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00358a48);
                    }
                    continue 'l_358a48;
                } else {
                    break 'b_358ae8;
                }
            }
        }
    }
    c.cycles += 15;
    c.g[V1][0] = sx(r32(c, S5).wrapping_add(r32(c, S6)));
    c.g[V0][0] = 0xffffffffffff0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    c.g[RA][0] = ld(c, at(c, SP, 0x30));
    c.lwc1(20, at(c, SP, 0xb0));
    sw(c, at(c, V1, 0x18), c.g[V0][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xc0));
    c.pc = t;
    return;
}

/// `int func_0017FBF0(void* self)` (0x17fbf0): render one debug-menu frame.
/// When the module render hook (render-settings slot 17) accepts, reset the
/// cursor trail, set up the 2D view, call the module's own draw and render
/// fade, then let the menu at `self+0x14` draw itself.
pub fn debug_menu_module_render(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_17fdd4: {
        c.cycles += 14;
        c.g[A1][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.swc1(21, at(c, SP, 0x48));
        c.swc1(20, at(c, SP, 0x40));
        c.g[V1][0] = lw(c, at(c, A1, 0x10d8));
        c.g[A0][0] = lh(c, at(c, V1, 0x88));
        c.g[V0][0] = lw(c, at(c, V1, 0x8c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0017fc20);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x0017fc20).is_err() {
            return;
        }
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0u64;
        if t {
            break 'b_17fdd4;
        } else {
            'b_17fdbc: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0017fc30);
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, 0x00231d60, 0x0017fc30).is_err() {
                    return;
                }
                if c.g[V0][0] != 0u64 {
                    c.g[A1][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                    break 'b_17fdbc;
                } else {
                    c.cycles += 97;
                    c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 21);
                    c.g[V1][0] = lw(c, at(c, V0, 0x10d8));
                    c.g[A0][0] = lh(c, at(c, V1, 0x78));
                    sw(c, at(c, SP, 0x14), 0u64);
                    c.swc1(21, at(c, SP, 0x10));
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.lwc1(20, at(c, SP, 0x14));
                    sw(c, at(c, SP, 0x18), 0u64);
                    sw(c, at(c, SP, 0x1c), 0u64);
                    c.swc1(20, r32(c, SP));
                    c.swc1(20, at(c, SP, 0x4));
                    c.swc1(20, at(c, SP, 0x8));
                    c.g[V0][0] = lw(c, at(c, V1, 0x7c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0017fc7c);
                    c.g[A1][0] = c.g[SP][0];
                    if call_ptr(c, t, 0x0017fc7c).is_err() {
                        return;
                    }
                    c.g[A3][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                    c.mov_s(12, 20);
                    c.g[AT][0] = 0x44200000;
                    c.mtc1(1, 14);
                    c.g[A2][0] = 0u64;
                    c.g[V0][0] = lw(c, at(c, A3, 0x10d8));
                    c.mov_s(17, 21);
                    c.g[AT][0] = 0x43f00000;
                    c.mtc1(1, 15);
                    c.mov_s(13, 12);
                    c.g[A0][0] = lh(c, at(c, V0, 0xd0));
                    c.g[A1][0] = 0u64;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 16);
                    c.g[V1][0] = lw(c, at(c, V0, 0xd4));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017fcc0);
                    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0017fcc0).is_err() {
                        return;
                    }
                    c.g[A2][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[A1][0] = 0x500000;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xe60));
                    c.g[V0][0] = lw(c, at(c, A2, 0x10d8));
                    c.g[A0][0] = lh(c, at(c, V0, 0x110));
                    c.g[V1][0] = lw(c, at(c, V0, 0x114));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017fce0);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0017fce0).is_err() {
                        return;
                    }
                    c.g[A3][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[V1][0] = 0xfffffffffe7f0000;
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.g[T1][0] = 0x1000000;
                    c.g[A0][0] = lw(c, at(c, A3, 0xe84));
                    c.g[A1][0] = 0xffffffffffcf0000;
                    c.g[A1][0] = c.g[A1][0] | 0xffff;
                    c.g[T2][0] = 0x300000;
                    c.g[V0][0] = lw(c, at(c, A0, 0x4));
                    c.g[A2][0] = 0xfffffffffff00000;
                    c.g[A2][0] = c.g[A2][0] | 0xfff;
                    c.g[T3][0] = 0xffffffffffffff83;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[T0][0] = 0xffffffffffbf0000;
                    c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                    c.g[T0][0] = c.g[T0][0] | 0xffff;
                    sw(c, at(c, A0, 0x4), c.g[V0][0]);
                    c.g[T1][0] = 0x400000;
                    c.g[A0][0] = sx(at(c, S0, 0x24));
                    c.g[V1][0] = lw(c, at(c, A3, 0xe84));
                    c.g[V0][0] = lw(c, at(c, V1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                    sw(c, at(c, V1, 0x4), c.g[V0][0]);
                    c.g[A1][0] = lw(c, at(c, A3, 0xe84));
                    c.g[V0][0] = lw(c, at(c, A1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                    c.g[V0][0] = c.g[V0][0] | 0xd000;
                    sw(c, at(c, A1, 0x4), c.g[V0][0]);
                    c.g[V1][0] = lw(c, at(c, A3, 0xe84));
                    c.g[V0][0] = lw(c, at(c, V1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                    c.g[V0][0] = c.g[V0][0] | 0x14;
                    sw(c, at(c, V1, 0x4), c.g[V0][0]);
                    c.g[A1][0] = lw(c, at(c, A3, 0xe84));
                    c.g[V0][0] = lw(c, at(c, A1, 0x4));
                    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                    sw(c, at(c, A1, 0x4), c.g[V0][0]);
                    c.g[RA][0] = sx(0x0017fd84);
                    c.g[A1][0] = lw(c, at(c, S0, 0x1c));
                    if callee::C0017f500.call(c, 0x0017fd84).is_err() {
                        return;
                    }
                    c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                    c.g[A1][0] = 0xfffffffffffffc1f;
                    c.g[A0][0] = lw(c, at(c, V1, 0xe84));
                    c.g[V0][0] = lw(c, at(c, A0, 0x8));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | 0x120;
                    sw(c, at(c, A0, 0x8), c.g[V0][0]);
                    c.g[A1][0] = lw(c, at(c, S0, 0x14));
                    c.g[V0][0] = lw(c, r32(c, A1));
                    c.g[A0][0] = lh(c, at(c, V0, 0x18));
                    c.g[V1][0] = lw(c, at(c, V0, 0x1c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0017fdb8);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x0017fdb8).is_err() {
                        return;
                    }
                    c.g[A1][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                    break 'b_17fdbc;
                }
            }
            c.cycles += 6;
            c.g[V0][0] = lw(c, at(c, A1, 0x10d8));
            c.g[A0][0] = lh(c, at(c, V0, 0xa0));
            c.g[V1][0] = lw(c, at(c, V0, 0xa4));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0017fdd0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x0017fdd0).is_err() {
                return;
            }
            c.g[V0][0] = 0x1;
            break 'b_17fdd4;
        }
    }
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    c.lwc1(21, at(c, SP, 0x48));
    c.lwc1(20, at(c, SP, 0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x50));
    c.pc = t;
    return;
}

/// `void func_0017F2D0(void* p)` (0x17f2d0): advance the debug menu's firefly
/// cursor trail twice per frame: step the phase, wrap both the phase and the
/// position into `[0,1)`, advance the 64-entry ring at `head` and store the
/// screen-space point.
pub fn debug_menu_module_update_cursor_trail(c: &mut Cpu) {
    let frame = Frame::open(c, 0xa0);
    c.cycles += 26;
    c.swc1(28, at(c, SP, 0x90));
    c.swc1(27, at(c, SP, 0x88));
    c.swc1(26, at(c, SP, 0x80));
    c.swc1(25, at(c, SP, 0x78));
    c.swc1(24, at(c, SP, 0x70));
    c.swc1(23, at(c, SP, 0x68));
    c.swc1(22, at(c, SP, 0x60));
    c.swc1(21, at(c, SP, 0x58));
    frame.save(c, S0, 0x40);
    frame.save(c, S1, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);
    c.g[S1][0] = 0x1;
    c.swc1(20, at(c, SP, 0x50));
    c.lwc1(28, r32(c, GP).wrapping_sub(0x5a54));
    c.lwc1(27, r32(c, GP).wrapping_sub(0x5a50));
    c.lwc1(26, r32(c, GP).wrapping_sub(0x5a4c));
    c.lwc1(25, r32(c, GP).wrapping_sub(0x5a48));
    c.lwc1(23, r32(c, GP).wrapping_sub(0x5a44));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 22);
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 21);
    c.g[AT][0] = 0x41a00000;
    c.mtc1(1, 24);
    'l_17f338: loop {
        'b_17f3b8: {
            c.cycles += 31;
            c.g[RA][0] = sx(0x0017f340);
            c.lwc1(12, at(c, S0, 0x8));
            if call_ptr(c, 0x0031bf60, 0x0017f340).is_err() {
                return;
            }
            c.lwc1(12, at(c, S0, 0xc));
            c.g[RA][0] = sx(0x0017f34c);
            c.mov_s(20, 0);
            if call_ptr(c, 0x0031bf60, 0x0017f34c).is_err() {
                return;
            }
            c.lwc1(1, at(c, S0, 0x8));
            c.add_s(20, 20, 28);
            c.add_s(0, 0, 28);
            c.lwc1(4, at(c, S0, 0xc));
            c.add_s(6, 1, 26);
            c.lwc1(2, r32(c, S0));
            c.mul_s(20, 20, 27);
            c.lwc1(3, at(c, S0, 0x4));
            c.mul_s(0, 0, 27);
            c.swc1(26, at(c, SP, 0x10));
            c.mul_s(1, 6, 23);
            c.swc1(25, at(c, SP, 0x14));
            c.add_s(2, 2, 20);
            c.swc1(20, r32(c, SP));
            c.add_s(3, 3, 0);
            c.swc1(0, at(c, SP, 0x4));
            c.add_s(1, 1, 22);
            c.swc1(6, at(c, S0, 0x8));
            c.add_s(4, 4, 25);
            c.cvt_w_s(5, 1);
            c.cvt_s_w(5, 5);
            c.swc1(2, r32(c, S0));
            c.c_lt_s(1, 5);
            c.swc1(3, at(c, S0, 0x4));
            let t = !c.fcc();
            c.swc1(4, at(c, S0, 0xc));
            if t {
                break 'b_17f3b8;
            } else {
                c.cycles += 1;
                c.sub_s(5, 5, 21);
                break 'b_17f3b8;
            }
        }
        'b_17f3e8: {
            c.cycles += 11;
            c.lwc1(7, r32(c, GP).wrapping_sub(0x5a40));
            c.mul_s(0, 4, 23);
            c.mul_s(1, 5, 7);
            c.add_s(0, 0, 22);
            c.cvt_w_s(5, 0);
            c.cvt_s_w(5, 5);
            c.sub_s(1, 6, 1);
            c.c_lt_s(0, 5);
            let t = !c.fcc();
            c.swc1(1, at(c, S0, 0x8));
            if t {
                break 'b_17f3e8;
            } else {
                c.cycles += 1;
                c.sub_s(5, 5, 21);
                break 'b_17f3e8;
            }
        }
        'b_17f414: {
            c.cycles += 10;
            c.mul_s(0, 2, 23);
            c.mul_s(1, 5, 7);
            c.add_s(0, 0, 22);
            c.sub_s(1, 4, 1);
            c.cvt_w_s(5, 0);
            c.cvt_s_w(5, 5);
            c.c_lt_s(0, 5);
            let t = !c.fcc();
            c.swc1(1, at(c, S0, 0xc));
            if t {
                break 'b_17f414;
            } else {
                c.cycles += 1;
                c.sub_s(5, 5, 21);
                break 'b_17f414;
            }
        }
        'b_17f440: {
            c.cycles += 10;
            c.mul_s(0, 3, 23);
            c.mul_s(1, 5, 7);
            c.add_s(0, 0, 22);
            c.sub_s(12, 2, 1);
            c.cvt_w_s(4, 0);
            c.cvt_s_w(4, 4);
            c.c_lt_s(0, 4);
            let t = !c.fcc();
            c.swc1(12, r32(c, S0));
            if t {
                break 'b_17f440;
            } else {
                c.cycles += 1;
                c.sub_s(4, 4, 21);
                break 'b_17f440;
            }
        }
        c.cycles += 35;
        c.g[V0][0] = lw(c, at(c, S0, 0x210));
        c.mul_s(0, 4, 7);
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x1));
        c.g[A0][0] = sx(at(c, V0, 0x1));
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V0][0] = sx(at(c, V0, 0x40));
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A0][0];
        }
        c.sub_s(0, 3, 0);
        c.g[V0][0] = sx((r32(c, V0) as i32 >> 6) as u32);
        c.g[V0][0] = sx(r32(c, V0) << 6);
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V0)));
        c.swc1(0, at(c, S0, 0x4));
        c.g[RA][0] = sx(0x0017f47c);
        sw(c, at(c, S0, 0x210), c.g[A0][0]);
        if call_ptr(c, 0x0031bf60, 0x0017f47c).is_err() {
            return;
        }
        c.mul_s(20, 0, 24);
        c.lwc1(12, at(c, S0, 0x4));
        c.lwc1(0, r32(c, GP).wrapping_sub(0x5a3c));
        c.g[RA][0] = sx(0x0017f490);
        c.add_s(20, 20, 0);
        if call_ptr(c, 0x0031bf60, 0x0017f490).is_err() {
            return;
        }
        c.mul_s(0, 0, 24);
        c.g[AT][0] = 0x43d20000;
        c.mtc1(1, 1);
        c.g[V0][0] = lw(c, at(c, S0, 0x210));
        c.swc1(20, at(c, SP, 0x10));
        c.add_s(0, 0, 1);
        c.g[V0][0] = sx(r32(c, V0) << 3);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
        c.swc1(0, at(c, SP, 0x14));
        ldl(c, 3, at(c, SP, 0x10));
        sdl(c, 3, at(c, V0, 0x10));
        let t = (c.g[S1][0] as i64) >= 0;
        c.g[RA][0] = ld(c, at(c, SP, 0x20));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x0017f338);
            }
            continue 'l_17f338;
        } else {
            c.cycles += 13;
            frame.restore(c, S0, 0x40);
            frame.restore(c, S1, 0x30);
            c.lwc1(28, at(c, SP, 0x90));
            c.lwc1(27, at(c, SP, 0x88));
            c.lwc1(26, at(c, SP, 0x80));
            c.lwc1(25, at(c, SP, 0x78));
            c.lwc1(24, at(c, SP, 0x70));
            c.lwc1(23, at(c, SP, 0x68));
            c.lwc1(22, at(c, SP, 0x60));
            c.lwc1(21, at(c, SP, 0x58));
            c.lwc1(20, at(c, SP, 0x50));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at(c, SP, 0xa0));
            c.pc = t;
            return;
        }
    }
}

/// `void func_0023AE00(sSelf* self, int idx)` (0x23ae00): rider `idx` finished
/// the race: store the finish time (or 360000 for a DNF), then when every
/// player is finished also fill in the estimate for the others, sort the
/// results by time and mark the handler done.
pub fn gm_handler8_on_rider_finished(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    'b_23af0c: {
        'b_23ae80: {
            c.cycles += 21;
            c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
            frame.save(c, S7, 0x20);
            sq(c, 30, at(c, SP, 0x10));
            c.g[S7][0] = c.g[A0][0];
            frame.save(c, S0, 0x90);
            c.g[FP][0] = c.g[A1][0];
            frame.save(c, S1, 0x80);
            frame.save(c, S2, 0x70);
            frame.save(c, S3, 0x60);
            frame.save(c, S4, 0x50);
            frame.save(c, S5, 0x40);
            frame.save(c, S6, 0x30);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.swc1(20, at(c, SP, 0xa0));
            c.g[V0][0] = lw(c, at(c, V1, 0x84));
            c.g[S0][0] = lw(c, at(c, V0, 0xc));
            c.g[RA][0] = sx(0x0023ae4c);
            c.g[A0][0] = c.g[S0][0];
            if call_ptr(c, 0x0012a250, 0x0023ae4c).is_err() {
                return;
            }
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx(r32(c, FP) << 2);
            if t {
                break 'b_23ae80;
            } else {
                c.cycles += 11;
                c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                c.g[S4][0] = c.g[V0][0];
                c.g[A0][0] = lw(c, at(c, V1, 0x28));
                c.g[A1][0] = lw(c, at(c, A0, 0x6c0));
                c.g[A0][0] = sx(at(c, A0, 0x6c0));
                c.g[V1][0] = lh(c, at(c, A1, 0x48));
                c.g[V0][0] = lw(c, at(c, A1, 0x4c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0023ae78);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                if call_ptr(c, t, 0x0023ae78).is_err() {
                    return;
                }
                let t = c.g[V0][0] != 0u64;

                if t {
                    break 'b_23af0c;
                } else {
                    break 'b_23ae80;
                }
            }
        }
        'b_23aedc: {
            c.cycles += 10;
            c.g[A1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
            c.g[A0][0] = sx(r32(c, FP) << 2);
            c.g[S4][0] = c.g[A0][0];
            c.g[V1][0] = lw(c, at(c, A1, 0x84));
            c.g[V0][0] = lw(c, at(c, V1, 0xc));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = lw(c, at(c, V0, 0x28));
            c.g[A0][0] = lw(c, at(c, V1, 0x480));
            let t = c.g[A0][0] == 0u64;
            c.g[V1][0] = 0x530000;
            if t {
                break 'b_23aedc;
            } else {
                c.cycles += 13;
                c.g[A0][0] = 0x50000;
                c.g[A0][0] = c.g[A0][0] | 0x7e40;
                c.g[A1][0] = sx(r32(c, S7).wrapping_add(r32(c, S4)));
                c.g[V0][0] = 0x530000;
                c.g[V1][0] = sx(at(c, V1, 0x6640));
                sw(c, at(c, A1, 0x4), c.g[A0][0]);
                c.g[V0][0] = sx(at(c, V0, 0x66a8));
                c.g[V1][0] = sx(r32(c, S4).wrapping_add(r32(c, V1)));
                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                c.g[A1][0] = 0x1;
                sw(c, r32(c, V1), c.g[A0][0]);
                let t = true;
                sw(c, r32(c, V0), c.g[A1][0]);
                if t {
                    break 'b_23af0c;
                } else {
                    break 'b_23aedc;
                }
            }
        }
        c.cycles += 12;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, S4)));
        c.g[A2][0] = lw(c, at(c, A0, 0x28));
        c.g[V0][0] = 0x530000;
        c.g[V1][0] = sx(at(c, V1, 0x6640));
        c.g[A0][0] = sx(r32(c, S7).wrapping_add(r32(c, S4)));
        c.g[A1][0] = lw(c, at(c, A2, 0x478));
        c.g[V0][0] = sx(at(c, V0, 0x66a8));
        c.g[V1][0] = sx(r32(c, S4).wrapping_add(r32(c, V1)));
        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        sw(c, at(c, A0, 0x4), c.g[A1][0]);
        sw(c, r32(c, V1), c.g[A1][0]);
        sw(c, r32(c, V0), 0u64);
        break 'b_23af0c;
    }
    'b_23b000: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x0023af14);
        c.g[A0][0] = c.g[S0][0];
        if call_ptr(c, 0x0012a250, 0x0023af14).is_err() {
            return;
        }
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
        if t {
            break 'b_23b000;
        } else {
            'b_23afb4: {
                c.cycles += 4;
                c.g[V0][0] = lw(c, at(c, V1, 0x10));
                c.g[V0][0] = sx(at(c, V0, 0x2));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S1][0] = 0u64;
                if t {
                    break 'b_23afb4;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = 0x530000;
                    c.mtc1(0, 20);
                    c.g[S6][0] = sx(at(c, V0, 0x6640));
                    c.g[S3][0] = sx(at(c, S0, 0x28));
                    c.g[S5][0] = 0u64;
                    c.g[S2][0] = sx(at(c, S7, 0x4));
                    'l_23af48: loop {
                        'b_23af64: {
                            c.cycles += 6;
                            c.g[A0][0] = lw(c, r32(c, S3));
                            c.lwc1(0, at(c, A0, 0x470));
                            c.c_le_s(20, 0);
                            let t = c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_23af64;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0u64;
                                break 'b_23af64;
                            }
                        }
                        'b_23af84: {
                            'b_23af80: {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S0][0] = sx(at(c, S7, 0x4));
                                if t {
                                    break 'b_23af80;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == c.g[FP][0];
                                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, S5)));
                                    if t {
                                        break 'b_23af84;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = sx(0x0023af7c);
                                        if call_ptr(c, 0x00122d78, 0x0023af7c).is_err() {
                                            return;
                                        }
                                        sw(c, r32(c, S2), c.g[V0][0]);
                                        break 'b_23af80;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, S5)));
                            break 'b_23af84;
                        }
                        c.cycles += 12;
                        c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                        c.g[V1][0] = lw(c, r32(c, V0));
                        c.g[S1][0] = sx(at(c, S1, 0x1));
                        c.g[S5][0] = sx(at(c, S5, 0x4));
                        c.g[S2][0] = sx(at(c, S2, 0x4));
                        sw(c, r32(c, S6), c.g[V1][0]);
                        c.g[S3][0] = sx(at(c, S3, 0x4));
                        c.g[V0][0] = lw(c, at(c, A0, 0x10));
                        c.g[V0][0] = sx(at(c, V0, 0x2));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S6][0] = sx(at(c, S6, 0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0023af48);
                            }
                            continue 'l_23af48;
                        } else {
                            break 'b_23afb4;
                        }
                    }
                }
            }
            'b_23aff8: {
                c.cycles += 10;
                c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                c.g[RA][0] = sx(0x0023afc0);
                c.g[A1][0] = 0xa;
                if callee::C00238bf8.call(c, 0x0023afc0).is_err() {
                    return;
                }
                c.g[V0][0] = 0x530000;
                c.g[V0][0] = sx(at(c, V0, 0x6730));
                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                c.g[V1][0] = lw(c, r32(c, V0));
                c.g[V1][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                let t = c.g[V1][0] != 0u64;
                c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                if t {
                    break 'b_23aff8;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                    c.g[A0][0] = lw(c, at(c, V1, 0x84));
                    c.g[V0][0] = lw(c, at(c, A0, 0xc));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
                    c.g[V1][0] = lw(c, at(c, V0, 0x28));
                    sw(c, at(c, V1, 0x100), 0u64);
                    c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                    break 'b_23aff8;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            sw(c, at(c, V1, 0x9c), c.g[V0][0]);
            break 'b_23b000;
        }
    }
    c.cycles += 13;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    lq(c, 30, at(c, SP, 0x10));
    c.g[RA][0] = ld(c, r32(c, SP));
    c.lwc1(20, at(c, SP, 0xa0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xb0));
    c.pc = t;
    return;
}

/// `sDmNode* func_00275B98(void* self, char* big, sDmNode* prev)` (0x275b98):
/// append every entry of the BIG archive `big` to the data-instance manager's
/// list, recursing into entries that are themselves archives, and emit each
/// added node through vtable slot 2.
pub fn data_instance_man_add_big_entries(c: &mut Cpu) {
    let frame = Frame::open(c, 0xa0);
    'b_275ca0: {
        c.cycles += 25;
        frame.save(c, S3, 0x60);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[A0][0];
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S7, 0x20);
        c.g[A0][0] = c.g[S5][0];
        frame.save(c, S0, 0x90);
        c.g[A1][0] = 0u64;
        frame.save(c, S1, 0x80);
        c.g[A2][0] = 0u64;
        frame.save(c, S6, 0x30);
        c.g[T0][0] = c.g[SP][0] | 0x4;
        frame.save_ra(c, 0x10);
        c.g[S7][0] = 0x1;
        sw(c, r32(c, SP), 0u64);
        c.g[RA][0] = sx(0x00275bf0);
        sw(c, at(c, SP, 0x4), 0u64);
        if call_ptr(c, 0x003e2768, 0x00275bf0).is_err() {
            return;
        }
        c.g[S2][0] = c.g[V0][0];
        let t = c.g[S2][0] == 0u64;
        c.g[S6][0] = 0x450000;
        if t {
            break 'b_275ca0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = lw(c, r32(c, SP));
            'l_275c00: loop {
                'b_275c78: {
                    'b_275c34: {
                        c.cycles += 7;
                        c.g[S0][0] = lw(c, at(c, S6, 0xdd4));
                        c.g[S1][0] = sx(r32(c, S5).wrapping_add(r32(c, V0)));
                        sw(c, at(c, S6, 0xdd4), 0u64);
                        c.g[RA][0] = sx(0x00275c14);
                        c.g[A0][0] = c.g[S1][0];
                        if call_ptr(c, 0x003e2580, 0x00275c14).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] == 0u64;
                        sw(c, at(c, S6, 0xdd4), c.g[S0][0]);
                        if t {
                            break 'b_275c34;
                        } else {
                            c.cycles += 6;
                            c.g[A2][0] = c.g[S3][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00275c2c);
                            c.g[A0][0] = c.g[S4][0];
                            if callee::C00275b98.call(c, 0x00275c2c).is_err() {
                                return;
                            }
                            let t = true;
                            c.g[S3][0] = c.g[V0][0];
                            if t {
                                break 'b_275c78;
                            } else {
                                break 'b_275c34;
                            }
                        }
                    }
                    c.cycles += 17;
                    c.g[RA][0] = sx(0x00275c3c);
                    c.g[A0][0] = c.g[S4][0];
                    if callee::C00275cf8.call(c, 0x00275c3c).is_err() {
                        return;
                    }
                    sw(c, at(c, V0, 0x8), c.g[S1][0]);
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V1][0] = lw(c, at(c, SP, 0x4));
                    sw(c, at(c, V0, 0xc), c.g[V1][0]);
                    sw(c, at(c, S3, 0x14), c.g[V0][0]);
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V1][0] = lw(c, at(c, S4, 0xc));
                    c.g[T0][0] = c.g[S3][0];
                    c.g[A3][0] = lw(c, at(c, SP, 0x4));
                    c.g[T1][0] = sx(at(c, S3, 0x4));
                    c.g[A0][0] = lh(c, at(c, V1, 0x10));
                    c.g[V0][0] = lw(c, at(c, V1, 0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00275c78);
                    c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x00275c78).is_err() {
                        return;
                    }
                    break 'b_275c78;
                }
                c.cycles += 10;
                c.g[A2][0] = c.g[S7][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = 0u64;
                c.g[A3][0] = c.g[SP][0];
                c.g[T0][0] = 0u64;
                c.g[RA][0] = sx(0x00275c94);
                c.g[S7][0] = sx(at(c, S7, 0x1));
                if call_ptr(c, 0x003e2768, 0x00275c94).is_err() {
                    return;
                }
                c.g[S2][0] = c.g[V0][0];
                let t = c.g[S2][0] != 0u64;
                c.g[V0][0] = lw(c, r32(c, SP));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00275c00);
                    }
                    continue 'l_275c00;
                } else {
                    break 'b_275ca0;
                }
            }
        }
    }
    c.cycles += 12;
    c.g[V0][0] = c.g[S3][0];
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xa0));
    c.pc = t;
    return;
}

/// `void func_00209E78(void* self)` (0x209e78): update the session map's
/// indicator icons: show the cursor, read the list box's selection out of the
/// screen's object table and show the "selected" icon (and hide the other)
/// for the selected entry, the opposite for the rest.
pub fn ov_screen_session_update_indicators(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_209ebc: {
        c.cycles += 12;
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[A2][0] = lw(c, at(c, S2, 0xe0));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = 0x1;
        if t {
            break 'b_209ebc;
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw(c, at(c, A2, 0x8));
            c.g[A0][0] = lh(c, at(c, V0, 0x48));
            c.g[V1][0] = lw(c, at(c, V0, 0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00209ebc);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x00209ebc).is_err() {
                return;
            }
            break 'b_209ebc;
        }
    }
    'b_209ee4: {
        c.cycles += 9;
        c.g[A0][0] = 0x470000;
        c.g[RA][0] = sx(0x00209ec8);
        c.g[A0][0] = sx(at(c, A0, 0x1850));
        if call_ptr(c, 0x00317670, 0x00209ec8).is_err() {
            return;
        }
        c.g[A0][0] = lw(c, at(c, S2, 0xd4));
        c.g[RA][0] = sx(0x00209ed4);
        c.g[A1][0] = c.g[V0][0];
        if call_ptr(c, 0x0039d860, 0x00209ed4).is_err() {
            return;
        }
        c.g[V0][0] = lw(c, at(c, V0, 0xa0));
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = 0x1;
        if t {
            break 'b_209ee4;
        } else {
            c.cycles += 1;
            c.g[V1][0] = lw(c, at(c, V0, 0x18));
            break 'b_209ee4;
        }
    }
    'b_209fa8: {
        c.cycles += 3;
        c.g[V0][0] = lw(c, at(c, S2, 0xc0));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0u64;
        if t {
            break 'b_209fa8;
        } else {
            c.cycles += 4;
            c.g[S5][0] = sx(r32(c, V1).wrapping_sub(0x1));
            c.g[S4][0] = sx(at(c, S2, 0x104));
            c.g[S0][0] = sx(at(c, S2, 0xe4));
            c.g[S3][0] = 0u64;
            'l_209f00: loop {
                'b_209f94: {
                    'b_209f90: {
                        'b_209f50: {
                            c.cycles += 2;
                            let t = c.g[S1][0] != c.g[S5][0];
                            c.g[A2][0] = lw(c, r32(c, S0));
                            if t {
                                break 'b_209f50;
                            } else {
                                'b_209f24: {
                                    c.cycles += 2;
                                    let t = c.g[A2][0] == 0u64;
                                    c.g[A1][0] = 0u64;
                                    if t {
                                        break 'b_209f24;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = lw(c, at(c, A2, 0x8));
                                        c.g[A0][0] = lh(c, at(c, V0, 0x48));
                                        c.g[V1][0] = lw(c, at(c, V0, 0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00209f24);
                                        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                        if call_ptr(c, t, 0x00209f24).is_err() {
                                            return;
                                        }
                                        break 'b_209f24;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, S3)));
                                c.g[A2][0] = lw(c, r32(c, V0));
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_209f90;
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] = lw(c, at(c, A2, 0x8));
                                    c.g[A0][0] = lh(c, at(c, V0, 0x48));
                                    c.g[V1][0] = lw(c, at(c, V0, 0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00209f48);
                                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x00209f48).is_err() {
                                        return;
                                    }
                                    let t = true;
                                    c.g[V0][0] = lw(c, at(c, S2, 0xc0));
                                    if t {
                                        break 'b_209f94;
                                    } else {
                                        break 'b_209f50;
                                    }
                                }
                            }
                        }
                        'b_209f6c: {
                            c.cycles += 2;
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_209f6c;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = lw(c, at(c, A2, 0x8));
                                c.g[A0][0] = lh(c, at(c, V0, 0x48));
                                c.g[V1][0] = lw(c, at(c, V0, 0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00209f6c);
                                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                if call_ptr(c, t, 0x00209f6c).is_err() {
                                    return;
                                }
                                break 'b_209f6c;
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, S3)));
                        c.g[A2][0] = lw(c, r32(c, V0));
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_209f90;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = lw(c, at(c, A2, 0x8));
                            c.g[A0][0] = lh(c, at(c, V0, 0x48));
                            c.g[V1][0] = lw(c, at(c, V0, 0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209f90);
                            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                            if call_ptr(c, t, 0x00209f90).is_err() {
                                return;
                            }
                            break 'b_209f90;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = lw(c, at(c, S2, 0xc0));
                    break 'b_209f94;
                }
                c.cycles += 5;
                c.g[S1][0] = sx(at(c, S1, 0x1));
                c.g[S0][0] = sx(at(c, S0, 0x4));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S3][0] = sx(at(c, S3, 0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00209f00);
                    }
                    continue 'l_209f00;
                } else {
                    break 'b_209fa8;
                }
            }
        }
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}

/// `void func_002780B8(void* self, int flags)` (0x2780b8): destroying
/// destructor of the script engine: install the base vtables, unload the
/// master script list, tear down the sound-bank, movie and script-object
/// managers, clear the per-frame state slots, and finally free the engine.
pub fn script_engine_dtor(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_278120: {
        c.cycles += 24;
        c.g[V0][0] = 0x480000;
        frame.save(c, S1, 0x50);
        c.g[V1][0] = 0x480000;
        frame.save(c, S5, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[A0][0] = 0x480000;
        frame.save(c, S2, 0x40);
        c.g[A2][0] = 0x480000;
        frame.save(c, S3, 0x30);
        c.g[V0][0] = sx(at(c, V0, 0x2100));
        frame.save(c, S4, 0x20);
        c.g[V1][0] = sx(at(c, V1, 0x21a0));
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[A0][0] = sx(at(c, A0, 0x2168));
        c.g[A2][0] = sx(at(c, A2, 0x21b8));
        c.g[S5][0] = c.g[A1][0];
        c.g[A3][0] = lw(c, at(c, S1, 0x52c));
        sw(c, at(c, S1, 0x2ac), c.g[V0][0]);
        sw(c, at(c, S1, 0x518), c.g[V1][0]);
        sw(c, at(c, S1, 0x528), c.g[A0][0]);
        let t = c.g[A3][0] == 0u64;
        sw(c, at(c, S1, 0x2a8), c.g[A2][0]);
        if t {
            break 'b_278120;
        } else {
            c.cycles += 2;
            c.g[RA][0] = sx(0x00278120);
            c.g[A0][0] = c.g[S1][0];
            if callee::C00278308.call(c, 0x00278120).is_err() {
                return;
            }
            break 'b_278120;
        }
    }
    'b_2781cc: {
        c.cycles += 31;
        c.g[A0][0] = sx(at(c, S1, 0xa38));
        c.g[A1][0] = 0x2;
        sw(c, at(c, GP, 0x3c0), 0u64);
        c.g[RA][0] = sx(0x00278134);
        c.g[S0][0] = sx(at(c, S1, 0x910));
        if callee::C00283518.call(c, 0x00278134).is_err() {
            return;
        }
        c.g[S2][0] = sx(at(c, S1, 0x6d0));
        c.g[A0][0] = sx(at(c, S1, 0xa28));
        c.g[RA][0] = sx(0x00278144);
        c.g[A1][0] = 0x2;
        if callee::C00275498.call(c, 0x00278144).is_err() {
            return;
        }
        c.g[S4][0] = sx(at(c, S1, 0x51c));
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = sx(at(c, V0, 0x2298));
        c.g[V1][0] = sx(at(c, V1, 0x2260));
        sw(c, at(c, S1, 0x944), c.g[V0][0]);
        c.g[A0][0] = sx(at(c, S1, 0x938));
        sw(c, at(c, S1, 0x92c), c.g[V1][0]);
        c.g[RA][0] = sx(0x0027816c);
        c.g[A1][0] = 0u64;
        if callee::C00283228.call(c, 0x0027816c).is_err() {
            return;
        }
        c.g[S3][0] = sx(at(c, S1, 0x500));
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = sx(at(c, V0, 0x2318));
        c.g[V1][0] = sx(at(c, V1, 0x22e0));
        sw(c, at(c, S1, 0x928), c.g[V0][0]);
        c.g[A0][0] = sx(at(c, S1, 0x91c));
        sw(c, at(c, S1, 0x910), c.g[V1][0]);
        c.g[RA][0] = sx(0x00278194);
        c.g[A1][0] = 0u64;
        if callee::C00283228.call(c, 0x00278194).is_err() {
            return;
        }
        let t = c.g[S2][0] == 0u64;
        c.g[A0][0] = c.g[S4][0];
        if t {
            break 'b_2781cc;
        } else {
            c.cycles += 2;
            let t = c.g[S2][0] == c.g[S0][0];
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x120));
            if t {
                break 'b_2781cc;
            } else {
                c.cycles += 1;
                'l_2781a8: loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = lw(c, at(c, S0, 0xc));
                    c.g[A0][0] = lh(c, at(c, V0, 0x8));
                    c.g[V1][0] = lw(c, at(c, V0, 0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002781c0);
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x002781c0).is_err() {
                        return;
                    }
                    let t = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x120));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002781a8);
                        }
                        continue 'l_2781a8;
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S4][0];
                        break 'b_2781cc;
                    }
                }
            }
        }
    }
    c.cycles += 17;
    c.g[RA][0] = sx(0x002781d4);
    c.g[A1][0] = 0u64;
    if callee::C002758c0.call(c, 0x002781d4).is_err() {
        return;
    }
    c.g[A0][0] = c.g[S3][0];
    c.g[RA][0] = sx(0x002781e0);
    c.g[A1][0] = 0u64;
    if callee::C002837c8.call(c, 0x002781e0).is_err() {
        return;
    }
    c.g[A0][0] = c.g[S1][0];
    c.g[RA][0] = sx(0x002781ec);
    c.g[A1][0] = c.g[S5][0];
    if callee::C00281ed8.call(c, 0x002781ec).is_err() {
        return;
    }
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}

/// `void func_0019F908(void* self, sPkg* pkg, int replace)` (0x19f908): load a
/// rider preview's shape textures: for every entry of the package, bind the
/// data to the already-known model id (replace) or append the entry's name
/// and a fresh id to `self`'s table.
pub fn fe_module_rider_preview_load_shape_textures(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    'b_19fa48: {
        c.cycles += 17;
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x60);
        c.g[S2][0] = 0u64;
        frame.save(c, S7, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S6, 0x40);
        sq(c, 30, at(c, SP, 0x20));
        frame.save_ra(c, 0x10);
        frame.save(c, S5, 0x50);
        c.g[S5][0] = lw(c, at(c, S4, 0x8));
        let t = (c.g[S5][0] as i64) <= 0;
        c.g[S7][0] = c.g[A2][0];
        if t {
            break 'b_19fa48;
        } else {
            c.cycles += 3;
            c.g[S6][0] = 0xc;
            c.g[S3][0] = sx(at(c, S4, 0x14));
            c.g[FP][0] = 0x4a0000;
            'l_19f958: loop {
                'b_19fa3c: {
                    'b_19fa38: {
                        'b_19f9dc: {
                            c.cycles += 21;
                            c.g[V0][0] = lw(c, r32(c, S3));
                            c.g[V1][0] = 0x4a0000;
                            c.g[A0][0] = sx(at(c, SP, 0x2));
                            c.g[A1][0] = 0u64;
                            c.g[S1][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                            c.g[T1][0] = sx(at(c, V1, 0x19a8));
                            c.g[V0][0] = lb(c, r32(c, T1));
                            c.g[A3][0] = lb(c, at(c, T1, 0x1));
                            sb(c, r32(c, SP), c.g[V0][0]);
                            sb(c, at(c, SP, 0x1), c.g[A3][0]);
                            c.g[RA][0] = sx(0x0019f988);
                            c.g[A2][0] = 0x6;
                            if call_ptr(c, 0x00416210, 0x0019f988).is_err() {
                                return;
                            }
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x0019f998);
                            c.g[A2][0] = c.g[SP][0];
                            if call_ptr(c, 0x003b3d00, 0x0019f998).is_err() {
                                return;
                            }
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0019f9a4);
                            c.g[A1][0] = c.g[SP][0];
                            if callee::C0019f878.call(c, 0x0019f9a4).is_err() {
                                return;
                            }
                            if (c.g[V0][0] as i64) < 0 {
                                c.g[A0][0] = lw(c, at(c, S0, 0x18));
                                break 'b_19f9dc;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[T0][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                                if t {
                                    break 'b_19fa38;
                                } else {
                                    c.cycles += 10;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[A3][0] = 0x1;
                                    c.g[V0][0] = lw(c, at(c, T0, 0x10d8));
                                    c.g[A0][0] = lh(c, at(c, V0, 0x180));
                                    c.g[V1][0] = lw(c, at(c, V0, 0x184));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0019f9d4);
                                    c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x0019f9d4).is_err() {
                                        return;
                                    }
                                    let t = true;
                                    c.g[S2][0] = sx(at(c, S2, 0x1));
                                    if t {
                                        break 'b_19fa3c;
                                    } else {
                                        break 'b_19f9dc;
                                    }
                                }
                            }
                        }
                        c.cycles += 23;
                        c.g[A1][0] = c.g[SP][0];
                        c.mult(0, V0, A0, S6);
                        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                        c.g[RA][0] = sx(0x0019f9f0);
                        c.g[A0][0] = sx(at(c, A0, 0x20));
                        if call_ptr(c, 0x004166f4, 0x0019f9f0).is_err() {
                            return;
                        }
                        c.g[T2][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = sx(at(c, FP, 0x19b0));
                        c.g[A3][0] = 0u64;
                        c.g[V1][0] = lw(c, at(c, T2, 0x10d8));
                        c.g[T0][0] = 0x1;
                        c.g[T1][0] = 0xffffffffffffffff;
                        c.g[A0][0] = lh(c, at(c, V1, 0x170));
                        c.g[V0][0] = lw(c, at(c, V1, 0x174));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0019fa1c);
                        c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x0019fa1c).is_err() {
                            return;
                        }
                        c.g[V1][0] = lw(c, at(c, S0, 0x18));
                        c.mult(0, A0, V1, S6);
                        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
                        sw(c, at(c, V1, 0x1c), c.g[V0][0]);
                        c.g[V0][0] = lw(c, at(c, S0, 0x18));
                        c.g[V0][0] = sx(at(c, V0, 0x1));
                        sw(c, at(c, S0, 0x18), c.g[V0][0]);
                        break 'b_19fa38;
                    }
                    c.cycles += 1;
                    c.g[S2][0] = sx(at(c, S2, 0x1));
                    break 'b_19fa3c;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S3][0] = sx(at(c, S3, 0x8));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019f958);
                    }
                    continue 'l_19f958;
                } else {
                    break 'b_19fa48;
                }
            }
        }
    }
    c.cycles += 12;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    lq(c, 30, at(c, SP, 0x20));
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xb0));
    c.pc = t;
    return;
}

/// `void func_0025B848(LobbyMan* self, sMessage* msg)` (0x25b848): handle the
/// reply to "connect": mark the screen busy and post either the "ok"/"cancel"
/// message (msg+8 == 0) after clearing the name field, or the failure message
/// with the status, switching the screen to its "connecting" state.
pub fn lobby_man_on_connect_reply(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_25b9a8: {
        'b_25b920: {
            c.cycles += 13;
            frame.save(c, S2, 0x40);
            frame.save(c, S3, 0x30);
            c.g[S2][0] = c.g[A0][0];
            frame.save(c, S1, 0x50);
            c.g[S3][0] = 0x1;
            frame.save(c, S0, 0x60);
            c.g[S1][0] = c.g[A1][0];
            frame.save_ra(c, 0x20);
            sw(c, r32(c, S2), c.g[S3][0]);
            c.g[V0][0] = lw(c, at(c, S1, 0x8));
            let t = c.g[V0][0] != 0u64;
            c.g[A1][0] = 0x480000;
            if t {
                break 'b_25b920;
            } else {
                'b_25b8fc: {
                    c.cycles += 32;
                    c.g[A0][0] = lw(c, at(c, S2, 0x54));
                    c.g[A2][0] = 0x480000;
                    c.g[A1][0] = 0x73650000;
                    c.g[T0][0] = 0u64;
                    c.g[A1][0] = c.g[A1][0] | 0x6c65;
                    c.g[A2][0] = sx(at(c, A2, 0x848));
                    c.g[RA][0] = sx(0x0025b89c);
                    c.g[A3][0] = 0u64;
                    if call_ptr(c, 0x003e8b10, 0x0025b89c).is_err() {
                        return;
                    }
                    c.g[A1][0] = 0x480000;
                    c.g[A2][0] = 0x20000000;
                    c.g[A1][0] = sx(at(c, A1, 0x488));
                    c.g[A0][0] = 0x8;
                    c.g[RA][0] = sx(0x0025b8b4);
                    c.g[A3][0] = 0u64;
                    if call_ptr(c, 0x00317d70, 0x0025b8b4).is_err() {
                        return;
                    }
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A2][0] = lw(c, at(c, S2, 0xf0));
                    c.g[V0][0] = 0x480000;
                    c.g[V1][0] = 0xc9;
                    c.g[V0][0] = sx(at(c, V0, 0x12b0));
                    sw(c, r32(c, S0), c.g[V1][0]);
                    sw(c, at(c, S0, 0x4), c.g[V0][0]);
                    c.g[A1][0] = 0x480000;
                    sw(c, at(c, SP, 0x10), c.g[A2][0]);
                    c.g[A1][0] = sx(at(c, A1, 0x4a8));
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = 0x20000000;
                    c.g[RA][0] = sx(0x0025b8ec);
                    c.g[A3][0] = 0u64;
                    if call_ptr(c, 0x00317d70, 0x0025b8ec).is_err() {
                        return;
                    }
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx(at(c, A1, 0x8));
                    if c.g[V0][0] != 0u64 {
                        sw(c, at(c, A1, 0x8), c.g[S0][0]);
                        break 'b_25b8fc;
                    } else {
                        break 'b_25b8fc;
                    }
                }
                c.cycles += 9;
                c.g[V0][0] = lw(c, at(c, SP, 0x10));
                sw(c, r32(c, A1), c.g[V0][0]);
                c.g[V1][0] = lw(c, at(c, V0, 0x4));
                sw(c, at(c, A1, 0x4), c.g[V1][0]);
                c.g[A0][0] = lw(c, at(c, V0, 0x4));
                sw(c, r32(c, A0), c.g[A1][0]);
                c.g[V0][0] = lw(c, at(c, SP, 0x10));
                let t = true;
                sw(c, at(c, V0, 0x4), c.g[A1][0]);
                if t {
                    break 'b_25b9a8;
                } else {
                    break 'b_25b920;
                }
            }
        }
        'b_25b984: {
            c.cycles += 25;
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(at(c, A1, 0x488));
            c.g[A2][0] = 0x20000000;
            c.g[RA][0] = sx(0x0025b934);
            c.g[A3][0] = 0u64;
            if call_ptr(c, 0x00317d70, 0x0025b934).is_err() {
                return;
            }
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = lw(c, at(c, S1, 0x8));
            c.g[V0][0] = 0x480000;
            c.g[A2][0] = lw(c, at(c, S2, 0xf0));
            c.g[V0][0] = sx(at(c, V0, 0x1290));
            c.g[V1][0] = 0x102;
            sw(c, at(c, S0, 0x8), c.g[A0][0]);
            c.g[A1][0] = 0x480000;
            sw(c, r32(c, S0), c.g[V1][0]);
            c.g[A1][0] = sx(at(c, A1, 0x4a8));
            sw(c, at(c, S0, 0x4), c.g[V0][0]);
            c.g[A0][0] = 0xc;
            sw(c, at(c, SP, 0x10), c.g[A2][0]);
            c.g[A3][0] = 0u64;
            c.g[RA][0] = sx(0x0025b974);
            c.g[A2][0] = 0x20000000;
            if call_ptr(c, 0x00317d70, 0x0025b974).is_err() {
                return;
            }
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx(at(c, A1, 0x8));
            if c.g[V0][0] != 0u64 {
                sw(c, at(c, A1, 0x8), c.g[S0][0]);
                break 'b_25b984;
            } else {
                break 'b_25b984;
            }
        }
        c.cycles += 9;
        c.g[V0][0] = lw(c, at(c, SP, 0x10));
        sw(c, r32(c, A1), c.g[V0][0]);
        c.g[V1][0] = lw(c, at(c, V0, 0x4));
        sw(c, at(c, A1, 0x4), c.g[V1][0]);
        c.g[A0][0] = lw(c, at(c, V0, 0x4));
        sw(c, r32(c, A0), c.g[A1][0]);
        c.g[V0][0] = lw(c, at(c, SP, 0x10));
        sw(c, at(c, V0, 0x4), c.g[A1][0]);
        sw(c, at(c, S2, 0x10), c.g[S3][0]);
        break 'b_25b9a8;
    }
    c.cycles += 8;
    sw(c, r32(c, SP), c.g[A1][0]);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}

/// `void func_0027A860(void* engine, int online, int intro)` (0x27a860):
/// queue the script choices for the end of a run: choices 0xc..0xf (or
/// 0x12..0x15 when online), with the first one only when `intro`, then
/// prepare and start the choice list and tell the audio system (music event
/// 0x14) to stop the race music.
pub fn script_engine_queue_finish_scripts(c: &mut Cpu) {
    let frame = Frame::open(c, 0x20);
    'b_27a9ac: {
        'b_27a98c: {
            'b_27a974: {
                'b_27a8f8: {
                    c.cycles += 5;
                    frame.save(c, S0, 0x10);
                    sd(c, r32(c, SP), c.g[RA][0]);
                    let t = c.g[A1][0] == 0u64;
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        break 'b_27a8f8;
                    } else {
                        'b_27a898: {
                            c.cycles += 2;
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_27a898;
                            } else {
                                c.cycles += 7;
                                c.g[A2][0] = 0x12;
                                c.g[A3][0] = 0x1;
                                c.g[T0][0] = 0x1;
                                c.g[T1][0] = 0xffffffffffffffff;
                                c.g[T2][0] = 0xffffffffffffffff;
                                c.g[RA][0] = sx(0x0027a898);
                                c.g[T3][0] = 0xffffffffffffffff;
                                if callee::C00278e50.call(c, 0x0027a898).is_err() {
                                    return;
                                }
                                break 'b_27a898;
                            }
                        }
                        'b_27a8e8: {
                            c.cycles += 16;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0x1;
                            c.g[A2][0] = 0x13;
                            c.g[A3][0] = 0u64;
                            c.g[T0][0] = 0u64;
                            c.g[T1][0] = 0xffffffffffffffff;
                            c.g[T2][0] = 0xffffffffffffffff;
                            c.g[RA][0] = sx(0x0027a8bc);
                            c.g[T3][0] = 0xffffffffffffffff;
                            if callee::C00278e50.call(c, 0x0027a8bc).is_err() {
                                return;
                            }
                            c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                            c.g[A1][0] = 0x1;
                            c.g[V0][0] = lw(c, at(c, A0, 0x84));
                            c.g[V1][0] = lw(c, at(c, V0, 0xc));
                            c.g[A0][0] = lw(c, at(c, V1, 0x7c));
                            let t = c.g[A0][0] != c.g[A1][0];
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_27a8e8;
                            } else {
                                c.cycles += 4;
                                c.g[A2][0] = 0x14;
                                c.g[A3][0] = 0x8;
                                let t = true;
                                c.g[T0][0] = 0u64;
                                if t {
                                    break 'b_27a974;
                                } else {
                                    break 'b_27a8e8;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[A2][0] = 0x15;
                        c.g[A3][0] = 0x8;
                        let t = true;
                        c.g[T0][0] = 0u64;
                        if t {
                            break 'b_27a974;
                        } else {
                            break 'b_27a8f8;
                        }
                    }
                }
                'b_27a920: {
                    c.cycles += 2;
                    let t = c.g[A2][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_27a920;
                    } else {
                        c.cycles += 8;
                        c.g[A1][0] = 0x1;
                        c.g[A2][0] = 0xc;
                        c.g[A3][0] = 0x1;
                        c.g[T0][0] = 0x1;
                        c.g[T1][0] = 0xffffffffffffffff;
                        c.g[T2][0] = 0xffffffffffffffff;
                        c.g[RA][0] = sx(0x0027a920);
                        c.g[T3][0] = 0xffffffffffffffff;
                        if callee::C00278e50.call(c, 0x0027a920).is_err() {
                            return;
                        }
                        break 'b_27a920;
                    }
                }
                c.cycles += 18;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0xd;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0x1;
                c.g[T1][0] = 0xffffffffffffffff;
                c.g[T2][0] = 0xffffffffffffffff;
                c.g[RA][0] = sx(0x0027a944);
                c.g[T3][0] = 0xffffffffffffffff;
                if callee::C00278e50.call(c, 0x0027a944).is_err() {
                    return;
                }
                c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                sw(c, at(c, V1, 0xf44), c.g[A1][0]);
                c.g[V0][0] = lw(c, at(c, A0, 0x84));
                c.g[V1][0] = lw(c, at(c, V0, 0xc));
                c.g[A0][0] = lw(c, at(c, V1, 0x7c));
                let t = c.g[A0][0] != c.g[A1][0];
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_27a98c;
                } else {
                    c.cycles += 3;
                    c.g[A2][0] = 0xe;
                    c.g[A3][0] = 0x8;
                    c.g[T0][0] = 0x1;
                    break 'b_27a974;
                }
            }
            c.cycles += 6;
            c.g[T1][0] = 0xffffffffffffffff;
            c.g[T2][0] = 0xffffffffffffffff;
            c.g[RA][0] = sx(0x0027a984);
            c.g[T3][0] = 0xffffffffffffffff;
            if callee::C00278e50.call(c, 0x0027a984).is_err() {
                return;
            }
            let t = true;
            c.g[A0][0] = c.g[S0][0];
            if t {
                break 'b_27a9ac;
            } else {
                break 'b_27a98c;
            }
        }
        c.cycles += 8;
        c.g[A2][0] = 0xf;
        c.g[A3][0] = 0x8;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0xffffffffffffffff;
        c.g[T2][0] = 0xffffffffffffffff;
        c.g[RA][0] = sx(0x0027a9a8);
        c.g[T3][0] = 0xffffffffffffffff;
        if callee::C00278e50.call(c, 0x0027a9a8).is_err() {
            return;
        }
        c.g[A0][0] = c.g[S0][0];
        break 'b_27a9ac;
    }
    c.cycles += 17;
    c.g[RA][0] = sx(0x0027a9b4);
    c.g[A1][0] = 0x1;
    if callee::C00278f38.call(c, 0x0027a9b4).is_err() {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0x1;
    c.g[A2][0] = 0u64;
    c.g[RA][0] = sx(0x0027a9c8);
    c.g[A3][0] = 0x1;
    if callee::C00278f68.call(c, 0x0027a9c8).is_err() {
        return;
    }
    c.g[RA][0] = sx(0x0027a9d0);
    if callee::C0028b180.call(c, 0x0027a9d0).is_err() {
        return;
    }
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x14;
    c.g[RA][0] = sx(0x0027a9e0);
    c.g[A2][0] = 0x1;
    if callee::C0028e8c0.call(c, 0x0027a9e0).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x20));
    c.pc = t;
    return;
}

/// `void func_0010E5D8(void* rider, int* arg, int a2, int a3)` (0x10e5d8): a
/// race-path event for a rider: decide whether the race state allows the
/// event, and for event 1 finish a still-running rider, for event 11 forward
/// the event's index to the active camera trigger list.
pub fn rider_handle_race_path_event(c: &mut Cpu) {
    let frame = Frame::open(c, 0x40);
    'b_10e6d8: {
        'b_10e6d4: {
            'b_10e6d0: {
                c.cycles += 21;
                frame.save(c, S0, 0x30);
                frame.save(c, S2, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S1, 0x20);
                sd(c, r32(c, SP), c.g[RA][0]);
                c.g[RA][0] = sx(0x0010e5f8);
                c.g[S2][0] = c.g[A1][0];
                if call_ptr(c, 0x0014dc80, 0x0010e5f8).is_err() {
                    return;
                }
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0010e604);
                c.g[A1][0] = 0u64;
                if call_ptr(c, 0x0014dd58, 0x0010e604).is_err() {
                    return;
                }
                c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0010e614);
                c.g[S1][0] = lw(c, at(c, V1, 0xc0));
                if call_ptr(c, 0x00144bc0, 0x0010e614).is_err() {
                    return;
                }
                c.g[A0][0] = lw(c, r32(c, V0));
                c.g[V0][0] = 0x530000;
                c.g[T0][0] = sx(at(c, V0, 0x5bc8));
                c.g[V1][0] = lb(c, at(c, T0, 0x48));
                let t = c.g[V1][0] == 0u64;
                c.g[T1][0] = 0u64;
                if t {
                    break 'b_10e6d0;
                } else {
                    c.cycles += 3;
                    c.g[A3][0] = 0x1;
                    let t = c.g[V1][0] == c.g[A3][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        break 'b_10e6d0;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_10e6d0;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[A2][0] = 0x5;
                            if t {
                                break 'b_10e6d0;
                            } else {
                                'b_10e660: {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[A2][0];
                                    c.g[A1][0] = 0x6;
                                    if t {
                                        break 'b_10e660;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = lw(c, at(c, S1, 0x98));
                                        if c.g[V0][0] != 0u64 {
                                            c.g[T1][0] = 0x1;
                                            break 'b_10e6d4;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A1][0] = 0x6;
                                            break 'b_10e660;
                                        }
                                    }
                                }
                                'b_10e674: {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[A1][0];

                                    if t {
                                        break 'b_10e674;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = lw(c, at(c, S1, 0x98));
                                        if c.g[V0][0] != 0u64 {
                                            c.g[T1][0] = 0x1;
                                            break 'b_10e6d4;
                                        } else {
                                            break 'b_10e674;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                if c.g[A0][0] == c.g[A2][0] {
                                    c.g[T1][0] = 0x1;
                                    break 'b_10e6d4;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A0][0] == c.g[A1][0];
                                    c.g[V0][0] = 0x7;
                                    if t {
                                        break 'b_10e6d0;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A0][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x8;
                                        if t {
                                            break 'b_10e6d0;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A0][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x9;
                                            if t {
                                                break 'b_10e6d0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A0][0] == c.g[V0][0];
                                                c.g[V0][0] = 0xa;
                                                if t {
                                                    break 'b_10e6d0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[A0][0] == c.g[V0][0];
                                                    c.g[V0][0] = 0xb;
                                                    if t {
                                                        break 'b_10e6d0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A0][0] == c.g[V0][0];
                                                        c.g[V0][0] = 0xc;
                                                        if t {
                                                            break 'b_10e6d0;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == c.g[V0][0];
                                                            c.g[V0][0] = 0xd;
                                                            if t {
                                                                break 'b_10e6d0;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[A0][0] == c.g[V0][0] {
                                                                    c.g[T1][0] = 0x1;
                                                                    break 'b_10e6d4;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[A0][0] == c.g[A3][0] {
                                                                        c.g[T1][0] = 0x1;
                                                                        break 'b_10e6d4;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] =
                                                                            lb(c, at(c, T0, 0x49));
                                                                        if c.g[V0][0] == 0u64 {
                                                                            c.g[V1][0] =
                                                                                lw(c, r32(c, S2));
                                                                            break 'b_10e6d8;
                                                                        } else {
                                                                            break 'b_10e6d0;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[T1][0] = 0x1;
            break 'b_10e6d4;
        }
        c.cycles += 1;
        c.g[V1][0] = lw(c, r32(c, S2));
        break 'b_10e6d8;
    }
    'b_10e718: {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V1][0] = lw(c, r32(c, S2));
            break 'b_10e718;
        } else {
            c.cycles += 2;
            if c.g[T1][0] == 0u64 {
                c.g[V1][0] = lw(c, r32(c, S2));
                break 'b_10e718;
            } else {
                'b_10e704: {
                    c.cycles += 6;
                    c.lwc1(1, at(c, S0, 0x470));
                    c.mtc1(0, 0);
                    c.c_le_s(0, 1);
                    if !c.fcc() {
                        c.g[V0][0] = 0u64;
                        break 'b_10e704;
                    } else {
                        break 'b_10e704;
                    }
                }
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.g[V1][0] = lw(c, r32(c, S2));
                    break 'b_10e718;
                } else {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0010e714);
                    c.g[A0][0] = c.g[S0][0];
                    if callee::C00125108.call(c, 0x0010e714).is_err() {
                        return;
                    }
                    c.g[V1][0] = lw(c, r32(c, S2));
                    break 'b_10e718;
                }
            }
        }
    }
    'b_10e75c: {
        c.cycles += 3;
        c.g[V0][0] = 0xb;
        if c.g[V1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x30);
            break 'b_10e75c;
        } else {
            c.cycles += 14;
            c.g[A1][0] = lw(c, at(c, S0, 0x6c0));
            c.g[A2][0] = sx(at(c, S0, 0x6c0));
            c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
            c.g[A0][0] = lh(c, at(c, A1, 0x38));
            c.g[V1][0] = lw(c, at(c, V0, 0x84));
            c.g[V0][0] = lw(c, at(c, A1, 0x3c));
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0010e748);
            c.g[S0][0] = lw(c, at(c, V1, 0x28));
            if call_ptr(c, t, 0x0010e748).is_err() {
                return;
            }
            c.g[A2][0] = lw(c, at(c, S2, 0x4));
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0010e758);
            c.g[A1][0] = c.g[V0][0];
            if call_ptr(c, 0x00270ab0, 0x0010e758).is_err() {
                return;
            }
            frame.restore(c, S0, 0x30);
            break 'b_10e75c;
        }
    }
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x40));
    c.pc = t;
    return;
}

/// `void func_001D2F68(void* self, void* widget, int msg)` (0x1d2f68): the
/// mountain room screen's widget action handler: message 5 replaces a room
/// image object with the next one from the factory, 6 does the same for a
/// specific index, and 1 sets the caption text of `self+0x54` from the
/// widget's index.
pub fn c_fe_state_mountain_room_on_widget_action(c: &mut Cpu) {
    let frame = Frame::open(c, 0x30);
    'b_1d3110: {
        'b_1d310c: {
            c.cycles += 7;
            c.g[V1][0] = c.g[A1][0];
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            c.g[S0][0] = c.g[A0][0];
            let t = c.g[V1][0] == 0u64;
            sd(c, r32(c, SP), c.g[RA][0]);
            if t {
                break 'b_1d310c;
            } else {
                'b_1d3040: {
                    'b_1d302c: {
                        'b_1d3000: {
                            'b_1d2fbc: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
                                if t {
                                    break 'b_1d2fbc;
                                } else {
                                    'b_1d2fa8: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_1d2fa8;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[A2][0] == c.g[V0][0] {
                                                c.g[V0][0] = lw(c, at(c, S0, 0x54));
                                                break 'b_1d3040;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                frame.restore(c, S0, 0x20);
                                                if t {
                                                    break 'b_1d3110;
                                                } else {
                                                    break 'b_1d2fa8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x6;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V0][0] = lw(c, at(c, S0, 0x10));
                                        break 'b_1d3000;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        frame.restore(c, S0, 0x20);
                                        if t {
                                            break 'b_1d3110;
                                        } else {
                                            break 'b_1d2fbc;
                                        }
                                    }
                                }
                            }
                            'b_1d2fd8: {
                                c.cycles += 3;
                                c.g[A1][0] = lw(c, at(c, V1, 0x18));
                                if c.g[A1][0] != 0u64 {
                                    c.g[V0][0] = lw(c, at(c, S0, 0x10));
                                    break 'b_1d2fd8;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001d2fd0);
                                    c.g[A0][0] = c.g[S0][0];
                                    if callee::C001d32e0.call(c, 0x001d2fd0).is_err() {
                                        return;
                                    }
                                    let t = true;
                                    frame.restore(c, S0, 0x20);
                                    if t {
                                        break 'b_1d3110;
                                    } else {
                                        break 'b_1d2fd8;
                                    }
                                }
                            }
                            c.cycles += 10;
                            c.g[A2][0] = sx(r32(c, A1).wrapping_sub(0x1));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A3][0] = lw(c, r32(c, V0));
                            c.g[V1][0] = lw(c, at(c, A3, 0x4));
                            c.g[A0][0] = lh(c, at(c, V1, 0x20));
                            c.g[V0][0] = lw(c, at(c, V1, 0x24));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001d2ff8);
                            c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                            if call_ptr(c, t, 0x001d2ff8).is_err() {
                                return;
                            }
                            let t = true;
                            c.g[A0][0] = lw(c, at(c, S0, 0x10));
                            if t {
                                break 'b_1d302c;
                            } else {
                                break 'b_1d3000;
                            }
                        }
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = lw(c, at(c, V1, 0x18));
                        c.g[A3][0] = lw(c, r32(c, V0));
                        c.g[V1][0] = lw(c, at(c, A3, 0x4));
                        c.g[A0][0] = lh(c, at(c, V1, 0x28));
                        c.g[V0][0] = lw(c, at(c, V1, 0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001d3020);
                        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x001d3020).is_err() {
                            return;
                        }
                        if c.g[V0][0] == 0u64 {
                            frame.restore(c, S0, 0x20);
                            break 'b_1d3110;
                        } else {
                            c.cycles += 1;
                            c.g[A0][0] = lw(c, at(c, S0, 0x10));
                            break 'b_1d302c;
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d3038);
                    c.g[A0][0] = sx(at(c, A0, 0x18));
                    if call_ptr(c, 0x0039f400, 0x001d3038).is_err() {
                        return;
                    }
                    let t = true;
                    frame.restore(c, S0, 0x20);
                    if t {
                        break 'b_1d3110;
                    } else {
                        break 'b_1d3040;
                    }
                }
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    frame.restore(c, S0, 0x20);
                    break 'b_1d3110;
                } else {
                    'b_1d30e8: {
                        'b_1d30d8: {
                            'b_1d30bc: {
                                'b_1d309c: {
                                    'b_1d3090: {
                                        c.cycles += 3;
                                        c.g[A1][0] = lw(c, at(c, V1, 0x18));
                                        let t = c.g[A1][0] == c.g[A2][0];
                                        c.g[S1][0] = 0u64;
                                        if t {
                                            break 'b_1d3090;
                                        } else {
                                            'b_1d3088: {
                                                'b_1d3070: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        ((c.g[A1][0] as i64) < 2i64) as u64;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x2;
                                                    if t {
                                                        break 'b_1d3070;
                                                    } else {
                                                        c.cycles += 2;
                                                        if c.g[A1][0] == 0u64 {
                                                            c.g[A0][0] = 0x460000;
                                                            break 'b_1d3088;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A2][0] = lw(c, at(c, S0, 0x54));
                                                            if t {
                                                                break 'b_1d30e8;
                                                            } else {
                                                                break 'b_1d3070;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_1d309c;
                                                } else {
                                                    c.cycles += 2;
                                                    if c.g[A1][0] == c.g[V0][0] {
                                                        c.g[V0][0] = lw(c, at(c, S0, 0xb4));
                                                        break 'b_1d30bc;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A2][0] = lw(c, at(c, S0, 0x54));
                                                        if t {
                                                            break 'b_1d30e8;
                                                        } else {
                                                            break 'b_1d3088;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] = sx(at(c, A0, 0x7450));
                                            if t {
                                                break 'b_1d30d8;
                                            } else {
                                                break 'b_1d3090;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx(at(c, A0, 0x7460));
                                    if t {
                                        break 'b_1d30d8;
                                    } else {
                                        break 'b_1d309c;
                                    }
                                }
                                'b_1d30b4: {
                                    c.cycles += 3;
                                    c.g[V0][0] = lw(c, at(c, S0, 0xb4));
                                    if (c.g[V0][0] as i64) <= 0 {
                                        c.g[A0][0] = 0x460000;
                                        break 'b_1d30b4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x460000;
                                        let t = true;
                                        c.g[A0][0] = sx(at(c, A0, 0x7460));
                                        if t {
                                            break 'b_1d30d8;
                                        } else {
                                            break 'b_1d30b4;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[A0][0] = sx(at(c, A0, 0x7478));
                                if t {
                                    break 'b_1d30d8;
                                } else {
                                    break 'b_1d30bc;
                                }
                            }
                            'b_1d30d4: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1d30d4;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx(at(c, A0, 0x7460));
                                    if t {
                                        break 'b_1d30d8;
                                    } else {
                                        break 'b_1d30d4;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = sx(at(c, A0, 0x7490));
                            break 'b_1d30d8;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001d30e0);
                        if call_ptr(c, 0x00317670, 0x001d30e0).is_err() {
                            return;
                        }
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A2][0] = lw(c, at(c, S0, 0x54));
                        break 'b_1d30e8;
                    }
                    c.cycles += 9;
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = lw(c, at(c, A2, 0x8));
                    c.g[A0][0] = lh(c, at(c, V0, 0x48));
                    c.g[V1][0] = lw(c, at(c, V0, 0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d3100);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x001d3100).is_err() {
                        return;
                    }
                    c.g[A0][0] = lw(c, at(c, S0, 0x54));
                    c.g[RA][0] = sx(0x001d310c);
                    c.g[A1][0] = c.g[S1][0];
                    if call_ptr(c, 0x003a1000, 0x001d310c).is_err() {
                        return;
                    }
                    break 'b_1d310c;
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        break 'b_1d3110;
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x30));
    c.pc = t;
    return;
}

/// `void func_00368660(void* self, int a, int b, ...)` (0x368660): bind (upload)
/// textures `a` and `b` of the upload manager: allocate heap nodes for the
/// image and its CLUT, emit the image/CLUT upload packets and finally the
/// texture setup for both; with nothing to send it writes an empty packet.
pub fn tex_man_bind_textures(c: &mut Cpu) {
    let frame = Frame::open(c, 0xe0);
    'b_368940: {
        'b_3686cc: {
            'b_3686c0: {
                c.cycles += 20;
                frame.save(c, S3, 0xa0);
                frame.save(c, S6, 0x70);
                c.g[S3][0] = c.g[A0][0];
                sq(c, 30, at(c, SP, 0x50));
                c.g[S6][0] = c.g[A2][0];
                frame.save(c, S0, 0xd0);
                c.g[FP][0] = c.g[A1][0];
                frame.save(c, S1, 0xc0);
                frame.save(c, S2, 0xb0);
                frame.save(c, S4, 0x90);
                frame.save(c, S5, 0x80);
                frame.save(c, S7, 0x60);
                frame.save_ra(c, 0x40);
                sw(c, at(c, SP, 0x1c), c.g[T0][0]);
                c.g[V0][0] = lw(c, r32(c, T0));
                sw(c, at(c, SP, 0x18), c.g[A3][0]);
                sw(c, at(c, SP, 0x20), c.g[T1][0]);
                let t = (c.g[FP][0] as i64) >= 0;
                sw(c, at(c, SP, 0x10), c.g[V0][0]);
                if t {
                    break 'b_3686c0;
                } else {
                    c.cycles += 2;
                    let t = (c.g[S6][0] as i64) < 0;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_368940;
                    } else {
                        c.cycles += 2;
                        let t = true;

                        if t {
                            break 'b_3686cc;
                        } else {
                            break 'b_3686c0;
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, FP) << 2);
            c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
            c.g[V0][0] = lw(c, at(c, V0, 0x8));
            break 'b_3686cc;
        }
        'b_3686e8: {
            'b_3686e4: {
                c.cycles += 2;
                let t = (c.g[S6][0] as i64) < 0;
                sw(c, r32(c, SP), c.g[V0][0]);
                if t {
                    break 'b_3686e4;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, S6) << 2);
                    c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                    let t = true;
                    c.g[V0][0] = lw(c, at(c, V0, 0x8));
                    if t {
                        break 'b_3686e8;
                    } else {
                        break 'b_3686e4;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_3686e8;
        }
        c.cycles += 6;
        sw(c, at(c, SP, 0x4), c.g[V0][0]);
        c.g[A3][0] = 0u64;
        sw(c, at(c, SP, 0x14), 0u64);
        c.g[S7][0] = 0u64;
        c.g[S2][0] = c.g[FP][0];
        'l_368700: loop {
            'b_368894: {
                'b_368890: {
                    c.cycles += 3;
                    if c.g[S7][0] != 0 {
                        c.g[S2][0] = c.g[S6][0];
                    }
                    let t = (c.g[S2][0] as i64) < 0;
                    c.g[V0][0] = sx(r32(c, S2) << 2);
                    if t {
                        break 'b_368890;
                    } else {
                        'b_36873c: {
                            'b_368734: {
                                c.cycles += 7;
                                c.g[S4][0] = c.g[S6][0];
                                c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                                c.g[S1][0] = lw(c, at(c, V0, 0x8));
                                c.g[V1][0] = 0x9;
                                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                                let t = c.g[V0][0] != c.g[V1][0];
                                if c.g[S7][0] != 0 {
                                    c.g[S4][0] = c.g[FP][0];
                                }
                                if t {
                                    break 'b_368734;
                                } else {
                                    c.cycles += 3;
                                    c.g[S0][0] = sx(at(c, S3, 0x4350));
                                    let t = true;
                                    c.g[S5][0] = 0x2;
                                    if t {
                                        break 'b_36873c;
                                    } else {
                                        break 'b_368734;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[S0][0] = sx(at(c, S3, 0x1f60));
                            c.g[S5][0] = 0u64;
                            break 'b_36873c;
                        }
                        'b_3687d8: {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, at(c, S1, 0x28));
                            if (c.g[V0][0] as i64) >= 0 {
                                c.g[V0][0] = lw(c, at(c, S1, 0x20));
                                break 'b_3687d8;
                            } else {
                                'b_3687bc: {
                                    c.cycles += 22;
                                    c.g[V0][0] = sx(r32(c, S7) << 2);
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                                    c.g[A0][0] = c.g[S0][0];
                                    sw(c, at(c, V0, 0x1f4c), c.g[V1][0]);
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[A3][0] = c.g[S4][0];
                                    c.g[T1][0] = 0x1;
                                    c.g[A1][0] = lw(c, at(c, S1, 0x14));
                                    c.g[RA][0] = sx(0x00368774);
                                    c.g[T0][0] = lw(c, at(c, SP, 0x18));
                                    if callee::C00366ce0.call(c, 0x00368774).is_err() {
                                        return;
                                    }
                                    c.g[V1][0] = 0x1c;
                                    c.g[A0][0] = lw(c, at(c, S0, 0x1ff0));
                                    c.mult(0, A1, V0, V1);
                                    c.g[A2][0] = lw(c, at(c, SP, 0x14));
                                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                                    c.g[V1][0] = lw(c, r32(c, A0));
                                    sw(c, at(c, S1, 0x30), c.g[V0][0]);
                                    c.g[V1][0] = sx((r32(c, V1) as i32 >> 8) as u32);
                                    c.g[V1][0] = sx(r32(c, V1) << (r32(c, S5) & 31));
                                    let t = c.g[A2][0] != 0u64;
                                    sw(c, at(c, S1, 0x28), c.g[V1][0]);
                                    if t {
                                        break 'b_3687bc;
                                    } else {
                                        c.cycles += 7;
                                        c.g[V0][0] = lw(c, at(c, SP, 0x20));
                                        c.g[A2][0] = 0xffffffffffffffff;
                                        c.g[A0][0] = lw(c, at(c, GP, 0x1384));
                                        c.g[A3][0] = 0u64;
                                        c.g[RA][0] = sx(0x003687b8);
                                        c.g[A1][0] = lw(c, r32(c, V0));
                                        if callee::C0038f460.call(c, 0x003687b8).is_err() {
                                            return;
                                        }
                                        sw(c, at(c, SP, 0x14), c.g[V0][0]);
                                        break 'b_3687bc;
                                    }
                                }
                                c.cycles += 7;
                                c.g[A3][0] = sx(at(c, SP, 0x14));
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A1][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x003687d0);
                                c.g[A2][0] = sx(at(c, SP, 0x10));
                                if callee::C00368170.call(c, 0x003687d0).is_err() {
                                    return;
                                }
                                c.g[A3][0] = 0x1;
                                c.g[V0][0] = lw(c, at(c, S1, 0x20));
                                break 'b_3687d8;
                            }
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[S7][0] = sx(at(c, S7, 0x1));
                            break 'b_368894;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, at(c, S1, 0x2c));
                            if (c.g[V0][0] as i64) >= 0 {
                                c.g[S7][0] = sx(at(c, S7, 0x1));
                                break 'b_368894;
                            } else {
                                'b_36887c: {
                                    c.cycles += 28;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    c.g[A1][0] = 0x400;
                                    sw(c, at(c, S3, 0x1f54), c.g[V1][0]);
                                    c.g[A0][0] = sx(at(c, S3, 0x1f60));
                                    c.g[V1][0] = 0x1;
                                    c.g[A3][0] = c.g[S4][0];
                                    c.g[V0][0] = lw(c, at(c, S1, 0xc));
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[T0][0] = lw(c, at(c, SP, 0x18));
                                    c.g[T1][0] = 0u64;
                                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xb));
                                    c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                                    c.g[V0][0] = 0x100;
                                    c.g[RA][0] = sx(0x00368828);
                                    if c.g[V1][0] == 0 {
                                        c.g[A1][0] = c.g[V0][0];
                                    }
                                    if callee::C00366ce0.call(c, 0x00368828).is_err() {
                                        return;
                                    }
                                    c.g[V1][0] = 0x1c;
                                    c.g[A1][0] = lw(c, at(c, S3, 0x3f50));
                                    c.mult(0, A0, V0, V1);
                                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                                    c.g[V1][0] = lw(c, r32(c, A1));
                                    c.g[V1][0] = c.g[V1][0] | 0x4;
                                    sw(c, r32(c, A1), c.g[V1][0]);
                                    c.g[V1][0] = sx((r32(c, V1) as i32 >> 8) as u32);
                                    sw(c, at(c, S1, 0x34), c.g[V0][0]);
                                    sw(c, at(c, S1, 0x2c), c.g[V1][0]);
                                    c.g[V0][0] = lw(c, at(c, SP, 0x14));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A3][0] = sx(at(c, SP, 0x14));
                                    if t {
                                        break 'b_36887c;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V0][0] = lw(c, at(c, SP, 0x20));
                                        c.g[A2][0] = 0xffffffffffffffff;
                                        c.g[A0][0] = lw(c, at(c, GP, 0x1384));
                                        c.g[A3][0] = 0u64;
                                        c.g[RA][0] = sx(0x00368874);
                                        c.g[A1][0] = lw(c, r32(c, V0));
                                        if callee::C0038f460.call(c, 0x00368874).is_err() {
                                            return;
                                        }
                                        sw(c, at(c, SP, 0x14), c.g[V0][0]);
                                        c.g[A3][0] = sx(at(c, SP, 0x14));
                                        break 'b_36887c;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A0][0] = c.g[S3][0];
                                c.g[RA][0] = sx(0x0036888c);
                                c.g[A2][0] = sx(at(c, SP, 0x10));
                                if callee::C003684f0.call(c, 0x0036888c).is_err() {
                                    return;
                                }
                                c.g[A3][0] = 0x1;
                                break 'b_368890;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[S7][0] = sx(at(c, S7, 0x1));
                break 'b_368894;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S7][0] as i64) < 2i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S2][0] = c.g[FP][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00368700);
                }
                continue 'l_368700;
            } else {
                'b_3688c4: {
                    c.cycles += 3;
                    c.g[A1][0] = lw(c, at(c, SP, 0x14));
                    let t = c.g[A1][0] == 0u64;
                    c.g[A0][0] = lw(c, at(c, GP, 0x1384));
                    if t {
                        break 'b_3688c4;
                    } else {
                        c.cycles += 6;
                        c.g[A2][0] = 0x4;
                        c.g[RA][0] = sx(0x003688b8);
                        sq(c, 7, at(c, SP, 0x30));
                        if callee::C0038f668.call(c, 0x003688b8).is_err() {
                            return;
                        }
                        c.g[V1][0] = lw(c, at(c, SP, 0x20));
                        sw(c, r32(c, V1), c.g[V0][0]);
                        lq(c, 7, at(c, SP, 0x30));
                        break 'b_3688c4;
                    }
                }
                'b_368904: {
                    c.cycles += 2;
                    let t = c.g[A3][0] != 0u64;
                    c.g[A0][0] = lw(c, at(c, SP, 0x10));
                    if t {
                        break 'b_368904;
                    } else {
                        c.cycles += 14;
                        c.g[V1][0] = 0x10000000;
                        c.g[V1][0] = c.g[V1][0] | 0x1;
                        c.g[A1][0] = 0x8800;
                        c.g[A1][0] = c.g[A1][0] << 45;
                        sd(c, r32(c, A0), c.g[V1][0]);
                        c.g[V0][0] = lw(c, at(c, SP, 0x10));
                        sd(c, at(c, V0, 0x8), 0u64);
                        c.g[V1][0] = lw(c, at(c, SP, 0x10));
                        sd(c, at(c, V1, 0x10), 0u64);
                        c.g[A0][0] = lw(c, at(c, SP, 0x10));
                        sd(c, at(c, A0, 0x18), c.g[A1][0]);
                        c.g[V0][0] = lw(c, at(c, SP, 0x10));
                        c.g[V0][0] = sx(at(c, V0, 0x20));
                        sw(c, at(c, SP, 0x10), c.g[V0][0]);
                        break 'b_368904;
                    }
                }
                'b_36891c: {
                    c.cycles += 2;
                    let t = (c.g[FP][0] as i64) < 0;
                    c.g[A1][0] = c.g[FP][0];
                    if t {
                        break 'b_36891c;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A2][0] = 0u64;
                        c.g[RA][0] = sx(0x0036891c);
                        c.g[A3][0] = sx(at(c, SP, 0x10));
                        if callee::C00368970.call(c, 0x0036891c).is_err() {
                            return;
                        }
                        break 'b_36891c;
                    }
                }
                'b_368934: {
                    c.cycles += 2;
                    let t = (c.g[S6][0] as i64) < 0;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        break 'b_368934;
                    } else {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S6][0];
                        c.g[A2][0] = 0x1;
                        c.g[RA][0] = sx(0x00368934);
                        c.g[A3][0] = sx(at(c, SP, 0x10));
                        if callee::C00368970.call(c, 0x00368934).is_err() {
                            return;
                        }
                        break 'b_368934;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c, SP, 0x10));
                c.g[V1][0] = lw(c, at(c, SP, 0x1c));
                sw(c, r32(c, V1), c.g[V0][0]);
                break 'b_368940;
            }
        }
    }
    c.cycles += 12;
    frame.restore(c, S0, 0xd0);
    frame.restore(c, S1, 0xc0);
    frame.restore(c, S2, 0xb0);
    frame.restore(c, S3, 0xa0);
    frame.restore(c, S4, 0x90);
    frame.restore(c, S5, 0x80);
    frame.restore(c, S6, 0x70);
    frame.restore(c, S7, 0x60);
    lq(c, 30, at(c, SP, 0x50));
    c.g[RA][0] = ld(c, at(c, SP, 0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xe0));
    c.pc = t;
    return;
}

/// `void* func_00357E80(void* mem, void* stream)` (0x357e80): construct a
/// multi-particle at `mem` from a stream: read the id and the 0xd8-byte
/// emitter data into a fresh allocation, set up the particle, load the
/// descriptor from the stream and read the position, boxes and points.
pub fn multi_particle_ctor_2(c: &mut Cpu) {
    let frame = Frame::open(c, 0x40);
    'b_357f80: {
        c.cycles += 58;
        c.g[A2][0] = 0x4;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[S0][0] = sx(at(c, S2, 0x20));
        c.g[V1][0] = lw(c, r32(c, S1));
        c.g[A0][0] = lh(c, at(c, V1, 0x10));
        c.g[V0][0] = lw(c, at(c, V1, 0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00357ebc);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x00357ebc).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, r32(c, S1));
        c.g[A1][0] = sx(at(c, S2, 0x4));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = lh(c, at(c, V1, 0x10));
        c.g[V0][0] = lw(c, at(c, V1, 0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00357ed8);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x00357ed8).is_err() {
            return;
        }
        c.g[A1][0] = 0x490000;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0xd8;
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x15f0));
        c.g[RA][0] = sx(0x00357ef0);
        c.g[A2][0] = 0x20000000;
        if call_ptr(c, 0x00317d70, 0x00357ef0).is_err() {
            return;
        }
        sw(c, at(c, S2, 0x8), c.g[V0][0]);
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0xd8;
        c.g[V0][0] = lw(c, r32(c, S1));
        c.g[A0][0] = lh(c, at(c, V0, 0x10));
        c.g[V1][0] = lw(c, at(c, V0, 0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00357f10);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x00357f10).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x00357f18);
        c.g[A0][0] = c.g[S2][0];
        if callee::C00358020.call(c, 0x00357f18).is_err() {
            return;
        }
        c.g[A0][0] = lw(c, at(c, S2, 0x10));
        c.g[RA][0] = sx(0x00357f24);
        c.g[A1][0] = c.g[S1][0];
        if call_ptr(c, 0x00370af8, 0x00357f24).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, r32(c, S1));
        c.g[A1][0] = sx(at(c, S2, 0x40));
        c.g[A2][0] = 0x10;
        c.g[A0][0] = lh(c, at(c, V1, 0x10));
        c.g[V0][0] = lw(c, at(c, V1, 0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00357f40);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x00357f40).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, r32(c, S1));
        c.g[A2][0] = 0x20;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = lh(c, at(c, V1, 0x10));
        c.g[V0][0] = lw(c, at(c, V1, 0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00357f5c);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        if call_ptr(c, t, 0x00357f5c).is_err() {
            return;
        }
        c.g[A2][0] = lw(c, at(c, S2, 0x4));
        let t = (c.g[A2][0] as i64) <= 0;
        c.g[A2][0] = sx(r32(c, A2) << 2);
        if t {
            break 'b_357f80;
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw(c, r32(c, S1));
            c.g[A1][0] = lw(c, at(c, S2, 0xc));
            c.g[A0][0] = lh(c, at(c, V0, 0x10));
            c.g[V1][0] = lw(c, at(c, V0, 0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00357f80);
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x00357f80).is_err() {
                return;
            }
            break 'b_357f80;
        }
    }
    c.cycles += 7;
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x40));
    c.pc = t;
    return;
}

/// `void func_00232348(void* self)` (0x232348): per-frame update of the
/// pre-start screen: tick the loading indicator, step the fade at
/// `world+0xa4+4` towards 1, spawn the next module once the app has none, and
/// advance the screen timer (faster when the world is loading).
pub fn pre_start_screen_update(c: &mut Cpu) {
    let frame = Frame::open(c, 0x30);
    'b_232398: {
        c.cycles += 19;
        c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
        frame.save(c, S0, 0x10);
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.swc1(20, at(c, SP, 0x20));
        c.g[RA][0] = sx(0x00232368);
        c.g[A0][0] = lw(c, at(c, V0, 0xa4));
        if callee::C00231f80.call(c, 0x00232368).is_err() {
            return;
        }
        c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
        c.lwc1(1, r32(c, GP).wrapping_sub(0x5198));
        c.g[V0][0] = lw(c, at(c, V0, 0xa4));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 2);
        c.lwc1(0, at(c, V0, 0x4));
        c.add_s(0, 0, 1);
        c.c_lt_s(2, 0);
        let t = !c.fcc();
        c.swc1(0, at(c, V0, 0x4));
        if t {
            break 'b_232398;
        } else {
            c.cycles += 1;
            c.swc1(2, at(c, V0, 0x4));
            break 'b_232398;
        }
    }
    'b_2323e8: {
        c.cycles += 4;
        c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
        c.g[V1][0] = lw(c, at(c, V0, 0x4));
        if c.g[V1][0] != 0u64 {
            c.lwc1(1, at(c, S0, 0x4));
            break 'b_2323e8;
        } else {
            c.cycles += 16;
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x4738));
            c.g[A2][0] = 0x100;
            c.g[RA][0] = sx(0x002323c0);
            c.g[A3][0] = 0u64;
            if call_ptr(c, 0x00317d70, 0x002323c0).is_err() {
                return;
            }
            c.g[V1][0] = 0x480000;
            c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x2710));
            sw(c, at(c, V0, 0x4), 0u64);
            sw(c, r32(c, V0), c.g[V1][0]);
            c.g[A1][0] = c.g[V0][0];
            sw(c, at(c, V0, 0x8), 0u64);
            c.g[RA][0] = sx(0x002323e4);
            sw(c, at(c, S0, 0x8), c.g[V0][0]);
            if call_ptr(c, 0x00316ef8, 0x002323e4).is_err() {
                return;
            }
            c.lwc1(1, at(c, S0, 0x4));
            break 'b_2323e8;
        }
    }
    'b_232478: {
        c.cycles += 7;
        c.lwc1(0, r32(c, GP).wrapping_sub(0x5194));
        c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x850));
        c.add_s(1, 1, 0);
        c.swc1(1, at(c, S0, 0x4));
        c.g[V0][0] = lw(c, at(c, A0, 0x2ee8));
        if (c.g[V0][0] as i64) <= 0 {
            frame.restore(c, S0, 0x10);
            break 'b_232478;
        } else {
            c.cycles += 8;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x5190));
            c.g[A1][0] = 0u64;
            c.add_s(0, 1, 0);
            c.g[RA][0] = sx(0x00232418);
            c.swc1(0, at(c, S0, 0x4));
            if call_ptr(c, 0x00326ca0, 0x00232418).is_err() {
                return;
            }
            c.g[V0][0] = ((c.g[V0][0] as i64) < 11i64) as u64;
            if c.g[V0][0] != 0u64 {
                frame.restore(c, S0, 0x10);
                break 'b_232478;
            } else {
                c.cycles += 9;
                c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x850));
                c.mtc1(0, 20);
                c.g[RA][0] = sx(0x00232434);
                c.g[A1][0] = 0u64;
                if call_ptr(c, 0x00326cc8, 0x00232434).is_err() {
                    return;
                }
                c.lwc1(0, at(c, V0, 0x20));
                c.c_eq_s(0, 20);
                if c.fcc() {
                    frame.restore(c, S0, 0x10);
                    break 'b_232478;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x850));
                    c.g[RA][0] = sx(0x00232454);
                    c.g[A1][0] = 0u64;
                    if call_ptr(c, 0x00326cc8, 0x00232454).is_err() {
                        return;
                    }
                    c.lwc1(0, at(c, V0, 0x28));
                    c.c_eq_s(0, 20);
                    if c.fcc() {
                        frame.restore(c, S0, 0x10);
                        break 'b_232478;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = lw(c, at(c, S0, 0x8));
                        c.g[V0][0] = 0x1;
                        sw(c, at(c, V1, 0x4), c.g[V0][0]);
                        frame.restore(c, S0, 0x10);
                        break 'b_232478;
                    }
                }
            }
        }
    }
    c.cycles += 4;
    c.g[RA][0] = ld(c, r32(c, SP));
    c.lwc1(20, at(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x30));
    c.pc = t;
    return;
}

/// `void func_00159B08(void* self, int rider, int k)` (0x159b08): mission `k`
/// was completed for `rider`: when the current count reached the stored
/// threshold, recompute the goal state, store the next threshold, grant the
/// medal when it is the best so far, and grant the mission reward once.
pub fn be_reward_int_on_mission_completed(c: &mut Cpu) {
    let frame = Frame::open(c, 0x80);
    'b_159c44: {
        'b_159c40: {
            c.cycles += 27;
            frame.save(c, S0, 0x70);
            frame.save(c, S3, 0x40);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S4, 0x30);
            c.g[S3][0] = c.g[A2][0];
            frame.save(c, S5, 0x20);
            frame.save(c, S6, 0x10);
            frame.save(c, S1, 0x60);
            frame.save(c, S2, 0x50);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.g[RA][0] = sx(0x00159b3c);
            c.g[S4][0] = c.g[A0][0];
            if callee::C0014dc80.call(c, 0x00159b3c).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00159b48);
            c.g[A1][0] = 0xa;
            if callee::C0014dd58.call(c, 0x00159b48).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00159b54);
            c.g[A1][0] = c.g[S3][0];
            if callee::C001543e0.call(c, 0x00159b54).is_err() {
                return;
            }
            c.g[S6][0] = c.g[V0][0];
            c.g[V1][0] = sx(r32(c, S3) << 2);
            c.g[V0][0] = 0x440000;
            c.g[V0][0] = sx(at(c, V0, 0xf58));
            c.g[S5][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[V0][0] = lw(c, r32(c, S5));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if t {
                break 'b_159c40;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x70);
                    break 'b_159c44;
                } else {
                    'b_159c00: {
                        'b_159bfc: {
                            c.cycles += 20;
                            c.g[RA][0] = sx(0x00159b84);
                            c.g[A0][0] = c.g[S0][0];
                            if callee::C0014a0e0.call(c, 0x00159b84).is_err() {
                                return;
                            }
                            c.g[S2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00159b90);
                            c.g[A0][0] = c.g[S0][0];
                            if callee::C0014a080.call(c, 0x00159b90).is_err() {
                                return;
                            }
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A3][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x00159bac);
                            c.g[T0][0] = 0x2;
                            if callee::C00157bf0.call(c, 0x00159bac).is_err() {
                                return;
                            }
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00159bc4);
                            c.g[A3][0] = c.g[S3][0];
                            if callee::C00158d58.call(c, 0x00159bc4).is_err() {
                                return;
                            }
                            let t = c.g[S0][0] == 0u64;
                            sw(c, r32(c, S5), c.g[V0][0]);
                            if t {
                                break 'b_159bfc;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[A0][0] = sx(r32(c, S3) << 4);
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4fe8));
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                                c.g[V1][0] = lw(c, at(c, V0, 0xc));
                                let t = c.g[S6][0] != c.g[V1][0];
                                c.g[A0][0] = c.g[S4][0];
                                if t {
                                    break 'b_159c00;
                                } else {
                                    c.cycles += 5;
                                    c.g[A3][0] = c.g[S3][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00159bfc);
                                    c.g[T0][0] = 0x2;
                                    if callee::C00159170.call(c, 0x00159bfc).is_err() {
                                        return;
                                    }
                                    break 'b_159bfc;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S4][0];
                        break 'b_159c00;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00159c0c);
                    c.g[A2][0] = c.g[S1][0];
                    if callee::C001577e0.call(c, 0x00159c0c).is_err() {
                        return;
                    }
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S4][0];
                    if t {
                        break 'b_159c40;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00159c24);
                        c.g[A3][0] = 0u64;
                        if callee::C0015a2e0.call(c, 0x00159c24).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] != 0u64;
                        frame.restore(c, S0, 0x70);
                        if t {
                            break 'b_159c44;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00159c40);
                            c.g[A3][0] = 0u64;
                            if callee::C00159cd0.call(c, 0x00159c40).is_err() {
                                return;
                            }
                            break 'b_159c40;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x70);
        break 'b_159c44;
    }
    c.cycles += 9;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore(c, S5, 0x20);
    frame.restore(c, S6, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x80));
    c.pc = t;
    return;
}

/// `void func_001599A0(void* self, int rider, int k)` (0x1599a0): collectible
/// `k` was collected for `rider`: same shape as the mission-completed
/// handler, with the collectible thresholds at `D_00440F48`.
pub fn be_reward_int_on_collectible_collected(c: &mut Cpu) {
    let frame = Frame::open(c, 0x80);
    'b_159ae0: {
        'b_159adc: {
            c.cycles += 28;
            frame.save(c, S0, 0x70);
            frame.save(c, S3, 0x40);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S4, 0x30);
            c.g[S3][0] = c.g[A2][0];
            frame.save(c, S5, 0x20);
            frame.save(c, S6, 0x10);
            frame.save(c, S1, 0x60);
            frame.save(c, S2, 0x50);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.g[RA][0] = sx(0x001599d4);
            c.g[S4][0] = c.g[A0][0];
            if callee::C0014dc80.call(c, 0x001599d4).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001599e0);
            c.g[A1][0] = 0xa;
            if callee::C0014dd58.call(c, 0x001599e0).is_err() {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001599f0);
            c.g[A2][0] = c.g[S3][0];
            if callee::C001535c0.call(c, 0x001599f0).is_err() {
                return;
            }
            c.g[S6][0] = c.g[V0][0];
            c.g[V1][0] = sx(r32(c, S3) << 2);
            c.g[V0][0] = 0x440000;
            c.g[V0][0] = sx(at(c, V0, 0xf48));
            c.g[S5][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[V0][0] = lw(c, r32(c, S5));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if t {
                break 'b_159adc;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x70);
                    break 'b_159ae0;
                } else {
                    'b_159a9c: {
                        'b_159a98: {
                            c.cycles += 20;
                            c.g[RA][0] = sx(0x00159a20);
                            c.g[A0][0] = c.g[S0][0];
                            if callee::C0014a0e0.call(c, 0x00159a20).is_err() {
                                return;
                            }
                            c.g[S2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00159a2c);
                            c.g[A0][0] = c.g[S0][0];
                            if callee::C0014a080.call(c, 0x00159a2c).is_err() {
                                return;
                            }
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A3][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x00159a48);
                            c.g[T0][0] = 0x2;
                            if callee::C00157bf0.call(c, 0x00159a48).is_err() {
                                return;
                            }
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00159a60);
                            c.g[A3][0] = c.g[S3][0];
                            if callee::C00158c80.call(c, 0x00159a60).is_err() {
                                return;
                            }
                            let t = c.g[S0][0] == 0u64;
                            sw(c, r32(c, S5), c.g[V0][0]);
                            if t {
                                break 'b_159a98;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[A0][0] = sx(r32(c, S3) << 4);
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5018));
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                                c.g[V1][0] = lw(c, at(c, V0, 0xc));
                                let t = c.g[S6][0] != c.g[V1][0];
                                c.g[A0][0] = c.g[S4][0];
                                if t {
                                    break 'b_159a9c;
                                } else {
                                    c.cycles += 5;
                                    c.g[A3][0] = c.g[S3][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00159a98);
                                    c.g[T0][0] = 0x2;
                                    if callee::C00159170.call(c, 0x00159a98).is_err() {
                                        return;
                                    }
                                    break 'b_159a98;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S4][0];
                        break 'b_159a9c;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00159aa8);
                    c.g[A2][0] = c.g[S1][0];
                    if callee::C001577e0.call(c, 0x00159aa8).is_err() {
                        return;
                    }
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S4][0];
                    if t {
                        break 'b_159adc;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00159ac0);
                        c.g[A3][0] = 0u64;
                        if callee::C0015a2e0.call(c, 0x00159ac0).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] != 0u64;
                        frame.restore(c, S0, 0x70);
                        if t {
                            break 'b_159ae0;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00159adc);
                            c.g[A3][0] = 0u64;
                            if callee::C00159cd0.call(c, 0x00159adc).is_err() {
                                return;
                            }
                            break 'b_159adc;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x70);
        break 'b_159ae0;
    }
    c.cycles += 9;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore(c, S5, 0x20);
    frame.restore(c, S6, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x80));
    c.pc = t;
    return;
}

/// `void func_0027AAF8(void* engine, int state)` (0x27aaf8): queue the intro
/// script choices for the script-engine state `state`. State 0 walks the
/// `D_00445488[...]`/`D_00481E80` choice table (pairs `<kind>, <arg>`, ended
/// by a negative kind); states 1 and 2 queue choice 5, state 3 queues 4 and
/// then 5, and anything else does nothing.
pub fn script_engine_queue_intro_scripts(c: &mut Cpu) {
    let frame = Frame::open(c, 0x20);
    'b_27ac58: {
        'b_27ac54: {
            'b_27ac08: {
                'b_27abe0: {
                    'b_27ab38: {
                        'b_27ab24: {
                            c.cycles += 5;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 3i64) as u64;
                            frame.save(c, S0, 0x10);
                            let t = c.g[V0][0] == 0u64;
                            sd(c, r32(c, SP), c.g[RA][0]);
                            if t {
                                break 'b_27ab24;
                            } else {
                                c.cycles += 2;
                                let t = (c.g[A1][0] as i64) > 0;
                                c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x84c));
                                if t {
                                    break 'b_27abe0;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A1][0] == 0u64;
                                    frame.restore(c, S0, 0x10);
                                    if t {
                                        break 'b_27ab38;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[RA][0] = ld(c, r32(c, SP));
                                        if t {
                                            break 'b_27ac58;
                                        } else {
                                            break 'b_27ab24;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = c.g[A1][0] == c.g[V0][0];
                        frame.restore(c, S0, 0x10);
                        if t {
                            break 'b_27ac08;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[RA][0] = ld(c, r32(c, SP));
                            if t {
                                break 'b_27ac58;
                            } else {
                                break 'b_27ab38;
                            }
                        }
                    }
                    'b_27ab8c: {
                        c.cycles += 20;
                        c.g[RA][0] = sx(0x0027ab40);
                        if call_ptr(c, 0x0014dc80, 0x0027ab40).is_err() {
                            return;
                        }
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x0027ab4c);
                        c.g[A1][0] = 0u64;
                        if call_ptr(c, 0x0014dd58, 0x0027ab4c).is_err() {
                            return;
                        }
                        c.g[A0][0] = 0x530000;
                        c.g[A1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                        c.g[V0][0] = lb(c, at(c, A0, 0x5c11));
                        c.g[V1][0] = 0x440000;
                        c.g[V1][0] = sx(at(c, V1, 0x5488));
                        c.g[A0][0] = lw(c, at(c, A1, 0x78));
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        c.g[A1][0] = 0x7;
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        c.g[RA][0] = sx(0x0027ab78);
                        c.g[S0][0] = lw(c, r32(c, V0));
                        if call_ptr(c, 0x0014dd58, 0x0027ab78).is_err() {
                            return;
                        }
                        c.g[V1][0] = 0x530000;
                        c.g[V0][0] = lw(c, at(c, V1, 0x4b30));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x480000;
                        if t {
                            break 'b_27ab8c;
                        } else {
                            c.cycles += 1;
                            c.g[S0][0] = sx(at(c, V0, 0x1e80));
                            break 'b_27ab8c;
                        }
                    }
                    c.cycles += 2;
                    if c.g[S0][0] == 0u64 {
                        frame.restore(c, S0, 0x10);
                        break 'b_27ac54;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, r32(c, S0));
                        if (c.g[V0][0] as i64) < 0 {
                            frame.restore(c, S0, 0x10);
                            break 'b_27ac54;
                        } else {
                            c.cycles += 2;
                            c.g[A2][0] = lw(c, r32(c, S0));
                            'l_27aba8: loop {
                                c.cycles += 12;
                                c.g[A1][0] = 0u64;
                                c.g[A3][0] = lw(c, at(c, S0, 0x4));
                                c.g[T0][0] = 0u64;
                                c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x84c));
                                c.g[T1][0] = 0xffffffffffffffff;
                                c.g[T2][0] = 0xffffffffffffffff;
                                c.g[RA][0] = sx(0x0027abc8);
                                c.g[T3][0] = 0xffffffffffffffff;
                                if callee::C00278e50.call(c, 0x0027abc8).is_err() {
                                    return;
                                }
                                c.g[S0][0] = sx(at(c, S0, 0x8));
                                c.g[V0][0] = lw(c, r32(c, S0));
                                if (c.g[V0][0] as i64) >= 0 {
                                    c.g[A2][0] = lw(c, r32(c, S0));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0027aba8);
                                    }
                                    continue 'l_27aba8;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    frame.restore(c, S0, 0x10);
                                    if t {
                                        break 'b_27ac54;
                                    } else {
                                        break 'b_27abe0;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 10;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0x5;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[T1][0] = 0xffffffffffffffff;
                c.g[T2][0] = 0xffffffffffffffff;
                c.g[RA][0] = sx(0x0027ac00);
                c.g[T3][0] = 0xffffffffffffffff;
                if callee::C00278e50.call(c, 0x0027ac00).is_err() {
                    return;
                }
                let t = true;
                frame.restore(c, S0, 0x10);
                if t {
                    break 'b_27ac54;
                } else {
                    break 'b_27ac08;
                }
            }
            c.cycles += 19;
            c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x84c));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x4;
            c.g[A3][0] = 0x3;
            c.g[T0][0] = 0u64;
            c.g[T1][0] = 0xffffffffffffffff;
            c.g[T2][0] = 0xffffffffffffffff;
            c.g[RA][0] = sx(0x0027ac2c);
            c.g[T3][0] = 0xffffffffffffffff;
            if callee::C00278e50.call(c, 0x0027ac2c).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x84c));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x5;
            c.g[A3][0] = 0u64;
            c.g[T0][0] = 0u64;
            c.g[T1][0] = 0xffffffffffffffff;
            c.g[T2][0] = 0xffffffffffffffff;
            c.g[RA][0] = sx(0x0027ac50);
            c.g[T3][0] = 0xffffffffffffffff;
            if callee::C00278e50.call(c, 0x0027ac50).is_err() {
                return;
            }
            frame.restore(c, S0, 0x10);
            break 'b_27ac54;
        }
        c.cycles += 1;
        c.g[RA][0] = ld(c, r32(c, SP));
        break 'b_27ac58;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x20));
    c.pc = t;
    return;
}

/// `void func_00292B48(void* self)` (0x292b48): stop the board loop sounds:
/// stop the ambience loop, send the six "stop loop" sound keys, and stop the
/// handle at `+0x59e0` when its count is positive.
pub fn ssx_audio_system_stop_board_loops(c: &mut Cpu) {
    let frame = Frame::open(c, 0x20);
    'b_292c50: {
        c.cycles += 59;
        frame.save(c, S0, 0x10);
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[RA][0] = sx(0x00292b5c);
        c.g[S0][0] = c.g[A0][0];
        if callee::C002913d8.call(c, 0x00292b5c).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, at(c, S0, 0x118));
        c.g[V0][0] = 0x4a0000;
        ldl(c, 5, at(c, V0, 0x3688));
        c.g[A0][0] = lw(c, r32(c, V1));
        c.g[V0][0] = lw(c, at(c, A0, 0xab0));
        c.g[A0][0] = sx(at(c, A0, 0x1d8));
        c.g[V1][0] = lh(c, at(c, V0, 0x18));
        c.g[A2][0] = lw(c, at(c, V0, 0x1c));
        let t = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x00292b88);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        if call_ptr(c, t, 0x00292b88).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, at(c, S0, 0x118));
        c.g[V0][0] = 0x4a0000;
        ldl(c, 5, at(c, V0, 0x3690));
        c.g[A0][0] = lw(c, r32(c, V1));
        c.g[V0][0] = lw(c, at(c, A0, 0xab0));
        c.g[A0][0] = sx(at(c, A0, 0x1d8));
        c.g[V1][0] = lh(c, at(c, V0, 0x18));
        c.g[A2][0] = lw(c, at(c, V0, 0x1c));
        let t = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x00292bb4);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        if call_ptr(c, t, 0x00292bb4).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, at(c, S0, 0x118));
        c.g[V0][0] = 0x4a0000;
        ldl(c, 5, at(c, V0, 0x3698));
        c.g[A0][0] = lw(c, r32(c, V1));
        c.g[V0][0] = lw(c, at(c, A0, 0xab0));
        c.g[A0][0] = sx(at(c, A0, 0x1d8));
        c.g[V1][0] = lh(c, at(c, V0, 0x18));
        c.g[A2][0] = lw(c, at(c, V0, 0x1c));
        let t = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x00292be0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        if call_ptr(c, t, 0x00292be0).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, at(c, S0, 0x118));
        c.g[V0][0] = 0x4a0000;
        ldl(c, 5, at(c, V0, 0x36a0));
        c.g[A0][0] = lw(c, r32(c, V1));
        c.g[V0][0] = lw(c, at(c, A0, 0xab0));
        c.g[A0][0] = sx(at(c, A0, 0x1d8));
        c.g[V1][0] = lh(c, at(c, V0, 0x18));
        c.g[A2][0] = lw(c, at(c, V0, 0x1c));
        let t = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x00292c0c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        if call_ptr(c, t, 0x00292c0c).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, at(c, S0, 0x118));
        c.g[V0][0] = 0x4a0000;
        ldl(c, 5, at(c, V0, 0x36a8));
        c.g[A0][0] = lw(c, r32(c, V1));
        c.g[RA][0] = sx(0x00292c28);
        c.g[A0][0] = sx(at(c, A0, 0x1d8));
        if call_ptr(c, 0x002adde0, 0x00292c28).is_err() {
            return;
        }
        c.g[V0][0] = lw(c, at(c, S0, 0x59e4));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = 0x1;
        if t {
            break 'b_292c50;
        } else {
            c.cycles += 7;
            c.g[V0][0] = lw(c, at(c, S0, 0x118));
            c.g[A1][0] = lw(c, at(c, S0, 0x59e0));
            c.g[A0][0] = lw(c, r32(c, V0));
            c.mtc1(0, 12);
            c.g[RA][0] = sx(0x00292c4c);
            c.g[A0][0] = sx(at(c, A0, 0x1d8));
            if call_ptr(c, 0x002ad5f0, 0x00292c4c).is_err() {
                return;
            }
            sw(c, at(c, S0, 0x59e4), 0u64);
            break 'b_292c50;
        }
    }
    c.cycles += 26;
    c.g[V1][0] = lw(c, at(c, S0, 0x118));
    c.g[V0][0] = 0x4a0000;
    ldl(c, 5, at(c, V0, 0x36b0));
    c.g[A0][0] = lw(c, r32(c, V1));
    c.g[V0][0] = lw(c, at(c, A0, 0xab0));
    c.g[A0][0] = sx(at(c, A0, 0x1d8));
    c.g[V1][0] = lh(c, at(c, V0, 0x18));
    c.g[A2][0] = lw(c, at(c, V0, 0x1c));
    let t = c.g[A2][0] as u32;
    c.g[RA][0] = sx(0x00292c7c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    if call_ptr(c, t, 0x00292c7c).is_err() {
        return;
    }
    c.g[V1][0] = lw(c, at(c, S0, 0x118));
    c.g[V0][0] = 0x4a0000;
    ldl(c, 5, at(c, V0, 0x36b8));
    c.g[A0][0] = lw(c, r32(c, V1));
    c.g[V0][0] = lw(c, at(c, A0, 0xab0));
    c.g[A0][0] = sx(at(c, A0, 0x1d8));
    c.g[V1][0] = lh(c, at(c, V0, 0x18));
    c.g[A2][0] = lw(c, at(c, V0, 0x1c));
    let t = c.g[A2][0] as u32;
    c.g[RA][0] = sx(0x00292ca8);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    if call_ptr(c, t, 0x00292ca8).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x20));
    c.pc = t;
    return;
}

/// `void func_001FF3B0(void* self, void* msg, int type)` (0x1ff3b0): the save
/// records screen's widget action: type 5 stores the selected slot from the
/// message and starts the "save" movie sequence, type 6 cancels the movie
/// when the screen is in the right state, and type 7 does the same as 5 once
/// the memory card is ready, then starts the sequence.
pub fn ov_screen_save_records_on_widget_action(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_1ff578: {
        'b_1ff574: {
            c.cycles += 9;
            frame.save(c, S0, 0x40);
            frame.save(c, S2, 0x20);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, S1, 0x30);
            c.g[S2][0] = c.g[A1][0];
            frame.save(c, S3, 0x10);
            let t = c.g[S2][0] == 0u64;
            sd(c, r32(c, SP), c.g[RA][0]);
            if t {
                break 'b_1ff574;
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c, S0, 0x1a8));
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x40);
                    break 'b_1ff578;
                } else {
                    'b_1ff4d4: {
                        'b_1ff46c: {
                            c.cycles += 3;
                            c.g[S1][0] = 0x6;
                            let t = c.g[A2][0] == c.g[S1][0];
                            c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                            if t {
                                break 'b_1ff46c;
                            } else {
                                'b_1ff418: {
                                    'b_1ff404: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x5;
                                        if t {
                                            break 'b_1ff404;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[A2][0] == c.g[V0][0] {
                                                c.g[V0][0] = lw(c, at(c, S0, 0x1c0));
                                                break 'b_1ff418;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                frame.restore(c, S0, 0x40);
                                                if t {
                                                    break 'b_1ff578;
                                                } else {
                                                    break 'b_1ff404;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x7;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V0][0] = lw(c, at(c, S0, 0x1c0));
                                        break 'b_1ff4d4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        frame.restore(c, S0, 0x40);
                                        if t {
                                            break 'b_1ff578;
                                        } else {
                                            break 'b_1ff418;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == c.g[S1][0];
                                c.g[S3][0] = 0x1;
                                if t {
                                    break 'b_1ff574;
                                } else {
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x001ff428);
                                    c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                                    if callee::C00227f80.call(c, 0x001ff428).is_err() {
                                        return;
                                    }
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[S1][0] = lw(c, at(c, S0, 0x214));
                                    c.g[V0][0] = 0x3;
                                    if c.g[S1][0] != c.g[V0][0] {
                                        frame.restore(c, S0, 0x40);
                                        break 'b_1ff578;
                                    } else {
                                        c.cycles += 12;
                                        c.g[V0][0] = lw(c, at(c, S2, 0x18));
                                        c.g[A0][0] = c.g[V1][0];
                                        c.g[A1][0] = 0x2;
                                        c.g[A2][0] = 0xffffffffffffffff;
                                        sw(c, at(c, S0, 0x1c4), c.g[V0][0]);
                                        sw(c, at(c, V1, 0xf8), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001ff45c);
                                        sw(c, at(c, S0, 0x1a8), c.g[S3][0]);
                                        if call_ptr(c, 0x0023cae8, 0x001ff45c).is_err() {
                                            return;
                                        }
                                        sw(c, at(c, S0, 0x1c0), c.g[S1][0]);
                                        sw(c, at(c, S0, 0x1a8), c.g[S3][0]);
                                        let t = true;
                                        sw(c, at(c, S0, 0x1dc), c.g[S3][0]);
                                        if t {
                                            break 'b_1ff574;
                                        } else {
                                            break 'b_1ff46c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x001ff474);
                        c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                        if callee::C00227f80.call(c, 0x001ff474).is_err() {
                            return;
                        }
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = 0x1;
                        c.g[V0][0] = lw(c, at(c, S0, 0x1a8));
                        if c.g[V0][0] == c.g[A0][0] {
                            frame.restore(c, S0, 0x40);
                            break 'b_1ff578;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = lw(c, at(c, S0, 0x1c0));
                            if c.g[V1][0] == c.g[S1][0] {
                                frame.restore(c, S0, 0x40);
                                break 'b_1ff578;
                            } else {
                                'b_1ff4cc: {
                                    'b_1ff4b4: {
                                        'b_1ff4b0: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == 0u64;
                                            sw(c, at(c, S0, 0x1e4), c.g[A0][0]);
                                            if t {
                                                break 'b_1ff4b0;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x2;
                                                if c.g[V1][0] == c.g[V0][0] {
                                                    c.g[V0][0] = lw(c, at(c, A2, 0x748));
                                                    break 'b_1ff4b4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] != c.g[A0][0];
                                                    c.g[V0][0] = 0x6;
                                                    if t {
                                                        break 'b_1ff4cc;
                                                    } else {
                                                        break 'b_1ff4b0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = lw(c, at(c, A2, 0x748));
                                        break 'b_1ff4b4;
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = lh(c, at(c, V0, 0x8));
                                    c.g[V1][0] = lw(c, at(c, V0, 0xc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001ff4c8);
                                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x001ff4c8).is_err() {
                                        return;
                                    }
                                    c.g[V0][0] = 0x6;
                                    break 'b_1ff4cc;
                                }
                                c.cycles += 2;
                                let t = true;
                                sw(c, at(c, S0, 0x1c0), c.g[V0][0]);
                                if t {
                                    break 'b_1ff574;
                                } else {
                                    break 'b_1ff4d4;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == c.g[S1][0];
                    c.g[S3][0] = 0x1;
                    if t {
                        break 'b_1ff574;
                    } else {
                        c.cycles += 12;
                        c.g[RA][0] = sx(0x001ff4e4);
                        c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                        if callee::C00227f80.call(c, 0x001ff4e4).is_err() {
                            return;
                        }
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A2][0] = lw(c, at(c, S1, 0x434));
                        c.g[A1][0] = lw(c, at(c, S1, 0x428));
                        c.g[V1][0] = lw(c, r32(c, A2));
                        c.g[A0][0] = lh(c, at(c, V1, 0x1b0));
                        c.g[V0][0] = lw(c, at(c, V1, 0x1b4));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001ff504);
                        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x001ff504).is_err() {
                            return;
                        }
                        if c.g[V0][0] == 0u64 {
                            frame.restore(c, S0, 0x40);
                            break 'b_1ff578;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = lw(c, at(c, S2, 0x18));
                            sw(c, at(c, S0, 0x1c4), c.g[V0][0]);
                            sw(c, at(c, S1, 0xf8), c.g[V0][0]);
                            c.g[S2][0] = lw(c, at(c, S0, 0x1c0));
                            if c.g[S2][0] != c.g[S3][0] {
                                frame.restore(c, S0, 0x40);
                                break 'b_1ff578;
                            } else {
                                c.cycles += 9;
                                c.g[A2][0] = lw(c, at(c, S1, 0x434));
                                c.g[A1][0] = lw(c, at(c, S0, 0x1c4));
                                c.g[V1][0] = lw(c, r32(c, A2));
                                c.g[A0][0] = lh(c, at(c, V1, 0x218));
                                c.g[V0][0] = lw(c, at(c, V1, 0x21c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001ff540);
                                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                if call_ptr(c, t, 0x001ff540).is_err() {
                                    return;
                                }
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x4;
                                if t {
                                    break 'b_1ff574;
                                } else {
                                    c.cycles += 11;
                                    sw(c, at(c, S0, 0x1a8), c.g[S2][0]);
                                    c.g[A0][0] = c.g[S1][0];
                                    sw(c, at(c, S0, 0x1c0), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001ff55c);
                                    c.g[A1][0] = 0u64;
                                    if call_ptr(c, 0x0023d570, 0x001ff55c).is_err() {
                                        return;
                                    }
                                    c.g[V0][0] = lw(c, at(c, S1, 0x748));
                                    c.g[A1][0] = 0x18;
                                    c.g[A0][0] = lh(c, at(c, V0, 0x8));
                                    c.g[V1][0] = lw(c, at(c, V0, 0xc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001ff574);
                                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x001ff574).is_err() {
                                        return;
                                    }
                                    break 'b_1ff574;
                                }
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x40);
        break 'b_1ff578;
    }
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x50));
    c.pc = t;
    return;
}

/// `void func_003B1AB0(void)` (0x3b1ab0): decode an MPEG-2 sequence-extension
/// header: read the extension start code's fields (horizontal/vertical/
/// bit-rate extensions, the display-extension flags) into the sequence
/// globals and recompute the frame rate.
pub fn mpeg2_sequence_extension(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_3b1c28: {
        'b_3b1c0c: {
            c.cycles += 76;
            c.g[V0][0] = 0x510000;
            c.g[V1][0] = lw(c, r32(c, V0).wrapping_sub(0x5f78));
            c.g[A1][0] = 0x1;
            frame.save(c, S0, 0x60);
            c.g[A2][0] = 0x510000;
            frame.save(c, S1, 0x50);
            c.g[A0][0] = 0x8;
            frame.save(c, S2, 0x40);
            c.g[S0][0] = 0x510000;
            frame.save(c, S3, 0x30);
            c.g[S1][0] = 0x510000;
            frame.save(c, S4, 0x20);
            frame.save(c, S5, 0x10);
            sd(c, r32(c, SP), c.g[RA][0]);
            sw(c, at(c, V1, 0x420), c.g[A1][0]);
            sw(c, at(c, V1, 0x424), 0u64);
            c.g[RA][0] = sx(0x003b1afc);
            sw(c, r32(c, A2).wrapping_sub(0x6a24), 0u64);
            if callee::C003b1300.call(c, 0x003b1afc).is_err() {
                return;
            }
            c.g[A0][0] = 0x1;
            c.g[RA][0] = sx(0x003b1b08);
            sw(c, r32(c, S0).wrapping_sub(0x6ae0), c.g[V0][0]);
            if callee::C003b1300.call(c, 0x003b1b08).is_err() {
                return;
            }
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b18);
            sw(c, r32(c, V1).wrapping_sub(0x6adc), c.g[V0][0]);
            if callee::C003b1300.call(c, 0x003b1b18).is_err() {
                return;
            }
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b28);
            sw(c, r32(c, V1).wrapping_sub(0x6ad8), c.g[V0][0]);
            if callee::C003b1300.call(c, 0x003b1b28).is_err() {
                return;
            }
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b34);
            c.g[S4][0] = c.g[V0][0];
            if callee::C003b1300.call(c, 0x003b1b34).is_err() {
                return;
            }
            c.g[A0][0] = 0xc;
            c.g[RA][0] = sx(0x003b1b40);
            c.g[S5][0] = c.g[V0][0];
            if callee::C003b1300.call(c, 0x003b1b40).is_err() {
                return;
            }
            c.g[A0][0] = 0x490000;
            c.g[S2][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003b1b50);
            c.g[A0][0] = sx(at(c, A0, 0x54b8));
            if callee::C003b2360.call(c, 0x003b1b50).is_err() {
                return;
            }
            c.g[RA][0] = sx(0x003b1b58);
            c.g[A0][0] = 0x8;
            if callee::C003b1300.call(c, 0x003b1b58).is_err() {
                return;
            }
            c.g[A0][0] = 0x1;
            c.g[RA][0] = sx(0x003b1b64);
            c.g[S3][0] = c.g[V0][0];
            if callee::C003b1300.call(c, 0x003b1b64).is_err() {
                return;
            }
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 0x2;
            c.g[RA][0] = sx(0x003b1b74);
            sw(c, r32(c, V1).wrapping_sub(0x6ad4), c.g[V0][0]);
            if callee::C003b1300.call(c, 0x003b1b74).is_err() {
                return;
            }
            c.g[A0][0] = 0x5;
            c.g[RA][0] = sx(0x003b1b80);
            sw(c, r32(c, S1).wrapping_sub(0x6ad0), c.g[V0][0]);
            if callee::C003b1300.call(c, 0x003b1b80).is_err() {
                return;
            }
            c.g[A1][0] = lw(c, r32(c, S1).wrapping_sub(0x6ad0));
            c.g[V1][0] = sx(at(c, V0, 0x1));
            c.mtc1(3, 2);
            c.cvt_s_w(2, 2);
            c.g[A2][0] = 0x510000;
            c.g[A1][0] = sx(at(c, A1, 0x1));
            c.g[A0][0] = lw(c, r32(c, A2).wrapping_sub(0x6af0));
            c.mtc1(5, 1);
            c.cvt_s_w(1, 1);
            c.g[V1][0] = 0x450000;
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3c78));
            c.g[A0][0] = sx(r32(c, A0) << 2);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.g[A3][0] = lw(c, r32(c, S0).wrapping_sub(0x6ae0));
            c.div_s(1, 1, 2);
            c.lwc1(0, r32(c, A0));
            c.g[A1][0] = 0x510000;
            c.g[A2][0] = 0x510000;
            c.g[V1][0] = sx((r32(c, A3) as i32 >> 7) as u32);
            sw(c, r32(c, A1).wrapping_sub(0x6acc), c.g[V0][0]);
            c.g[V1][0] = c.g[V1][0] & 0x1;
            c.mul_s(0, 0, 1);
            let t = c.g[V1][0] == 0u64;
            c.swc1(0, r32(c, A2).wrapping_sub(0x6af8));
            if t {
                break 'b_3b1c0c;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.g[A3][0] & 0xf;
                c.g[V0][0] = 0x5;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[T4][0] = 0x510000;
                if t {
                    break 'b_3b1c28;
                } else {
                    c.cycles += 7;
                    c.g[A1][0] = 0x510000;
                    c.g[V1][0] = 0x85;
                    c.g[A0][0] = 0x510000;
                    c.g[V0][0] = 0x8;
                    sw(c, r32(c, A1).wrapping_sub(0x6b14), c.g[V1][0]);
                    let t = true;
                    sw(c, r32(c, A0).wrapping_sub(0x6b10), c.g[V0][0]);
                    if t {
                        break 'b_3b1c28;
                    } else {
                        break 'b_3b1c0c;
                    }
                }
            }
        }
        c.cycles += 7;
        c.g[A1][0] = c.g[A3][0] & 0xf;
        c.g[A0][0] = 0x510000;
        c.g[V0][0] = sx((r32(c, A3) as i32 >> 4) as u32);
        c.g[V1][0] = 0x510000;
        sw(c, r32(c, A0).wrapping_sub(0x6b14), c.g[V0][0]);
        sw(c, r32(c, V1).wrapping_sub(0x6b10), c.g[A1][0]);
        c.g[T4][0] = 0x510000;
        break 'b_3b1c28;
    }
    c.cycles += 37;
    c.g[V0][0] = sx(r32(c, S2) << 18);
    c.g[T0][0] = lw(c, r32(c, T4).wrapping_sub(0x6aec));
    c.g[T3][0] = 0x510000;
    c.g[T2][0] = 0x510000;
    c.g[V1][0] = lw(c, r32(c, T3).wrapping_sub(0x6b08));
    c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
    c.g[AT][0] = 0x43c80000;
    c.mtc1(1, 1);
    c.mtc1(8, 0);
    c.cvt_s_w(0, 0);
    c.g[A0][0] = lw(c, r32(c, T2).wrapping_sub(0x6b0c));
    c.g[T1][0] = 0x510000;
    c.g[V0][0] = sx(r32(c, S5) << 12);
    c.g[A3][0] = lw(c, r32(c, T1).wrapping_sub(0x6ae8));
    c.g[V1][0] = c.g[V1][0] & 0xfff;
    c.mul_s(0, 0, 1);
    c.g[A2][0] = sx(r32(c, S3) << 10);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[A1][0] = sx(r32(c, S4) << 12);
    c.g[A0][0] = c.g[A0][0] & 0xfff;
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, A2)));
    c.g[A1][0] = c.g[A1][0] | c.g[A0][0];
    c.g[V1][0] = 0x510000;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    sw(c, r32(c, T2).wrapping_sub(0x6b0c), c.g[A1][0]);
    sw(c, r32(c, T3).wrapping_sub(0x6b08), c.g[V0][0]);
    c.swc1(0, r32(c, V1).wrapping_sub(0x6afc));
    sw(c, r32(c, T1).wrapping_sub(0x6ae8), c.g[A3][0]);
    sw(c, r32(c, T4).wrapping_sub(0x6aec), c.g[T0][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}

/// `void* func_003459A8(void* self, void* stream)` (0x3459a8): construct an
/// effect-link at `self`: run the base ctor, install the vtable, read the
/// effect id and, when it is the "matrix" effect, copy the matrix and attach
/// the emitter, otherwise copy the identity matrix.
pub fn particle_modifier_ctor_2(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_345aa8: {
        'b_345a78: {
            c.cycles += 25;
            frame.save(c, S1, 0x30);
            frame.save(c, S2, 0x20);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S3, 0x10);
            frame.save(c, S0, 0x40);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.g[RA][0] = sx(0x003459cc);
            c.g[S2][0] = c.g[A1][0];
            if callee::C003457c8.call(c, 0x003459cc).is_err() {
                return;
            }
            c.g[S3][0] = sx(at(c, S1, 0x50));
            c.g[V1][0] = 0x490000;
            c.g[A0][0] = c.g[S3][0];
            c.g[V1][0] = sx(at(c, V1, 0x12b0));
            c.g[RA][0] = sx(0x003459e4);
            sw(c, at(c, S1, 0x8), c.g[V1][0]);
            if call_ptr(c, 0x00370018, 0x003459e4).is_err() {
                return;
            }
            c.g[A3][0] = lw(c, r32(c, S2));
            c.g[A1][0] = sx(at(c, S1, 0x1e4));
            c.g[A2][0] = 0x4;
            c.g[A0][0] = lh(c, at(c, A3, 0x10));
            c.g[V1][0] = lw(c, at(c, A3, 0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00345a00);
            c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x00345a00).is_err() {
                return;
            }
            c.g[V0][0] = lw(c, at(c, S1, 0x1e4));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x500000;
            if t {
                break 'b_345a78;
            } else {
                c.cycles += 27;
                c.g[A0][0] = 0xd8;
                c.g[RA][0] = sx(0x00345a18);
                c.g[S0][0] = sx(at(c, S1, 0x10));
                if call_ptr(c, 0x00317d20, 0x00345a18).is_err() {
                    return;
                }
                sw(c, at(c, S1, 0x1e0), c.g[V0][0]);
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = 0xd8;
                c.g[V0][0] = lw(c, r32(c, S2));
                c.g[A0][0] = lh(c, at(c, V0, 0x10));
                c.g[V1][0] = lw(c, at(c, V0, 0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00345a38);
                c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00345a38).is_err() {
                    return;
                }
                c.g[A3][0] = lw(c, r32(c, S2));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x40;
                c.g[A0][0] = lh(c, at(c, A3, 0x10));
                c.g[V0][0] = lw(c, at(c, A3, 0x14));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00345a54);
                c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00345a54).is_err() {
                    return;
                }
                c.g[A2][0] = lw(c, at(c, S1, 0x1e0));
                c.g[A1][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00345a64);
                c.g[A0][0] = c.g[S3][0];
                if call_ptr(c, 0x003705e0, 0x00345a64).is_err() {
                    return;
                }
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00345a70);
                c.g[A1][0] = c.g[S2][0];
                if call_ptr(c, 0x00370af8, 0x00345a70).is_err() {
                    return;
                }
                let t = true;
                c.g[V1][0] = lw(c, at(c, GP, 0xf08));
                if t {
                    break 'b_345aa8;
                } else {
                    break 'b_345a78;
                }
            }
        }
        c.cycles += 12;
        c.g[V1][0] = sx(at(c, S1, 0x10));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xe60));
        c.lqc2(1, r32(c, V0));
        c.lqc2(2, at(c, V0, 0x10));
        c.lqc2(3, at(c, V0, 0x20));
        c.lqc2(4, at(c, V0, 0x30));
        c.sqc2(1, r32(c, V1));
        c.sqc2(2, at(c, V1, 0x10));
        c.sqc2(3, at(c, V1, 0x20));
        c.sqc2(4, at(c, V1, 0x30));
        sw(c, at(c, S1, 0x1e0), 0u64);
        c.g[V1][0] = lw(c, at(c, GP, 0xf08));
        break 'b_345aa8;
    }
    c.cycles += 10;
    c.g[V0][0] = c.g[S1][0];
    frame.restore(c, S0, 0x40);
    c.g[V1][0] = sx(at(c, V1, 0x1));
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    sw(c, at(c, GP, 0xf08), c.g[V1][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x50));
    c.pc = t;
    return;
}

/// `void func_002703F0(void* self, int reset)` (0x2703f0): step the replay
/// player one frame: on reset re-find the first snapshot and skip a finished
/// one, re-seek both streams, then either restore the current snapshot (reset)
/// or advance the position, and at the end of the replay loop back to the
/// start or stop at the end.
pub fn replay_step_frame(c: &mut Cpu) {
    let frame = Frame::open(c, 0x40);
    'b_27041c: {
        c.cycles += 8;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        let t = c.g[S1][0] == 0u64;
        sd(c, r32(c, SP), c.g[RA][0]);
        if t {
            break 'b_27041c;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x00270418);
            c.g[A1][0] = lw(c, at(c, S0, 0x484));
            if callee::C0026e448.call(c, 0x00270418).is_err() {
                return;
            }
            sw(c, at(c, S0, 0x488), c.g[V0][0]);
            break 'b_27041c;
        }
    }
    'b_27043c: {
        c.cycles += 3;
        c.g[V1][0] = lw(c, at(c, S0, 0x488));
        let t = c.g[V1][0] == 0u64;

        if t {
            break 'b_27043c;
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw(c, at(c, S0, 0x3d0));
            let t = c.g[V1][0] != c.g[V0][0];

            if t {
                break 'b_27043c;
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw(c, at(c, V1, 0x18));
                sw(c, at(c, S0, 0x488), c.g[V0][0]);
                break 'b_27043c;
            }
        }
    }
    'b_270520: {
        'b_27048c: {
            c.cycles += 2;
            let t = c.g[S1][0] == 0u64;
            c.g[S1][0] = sx(at(c, S0, 0x48c));
            if t {
                break 'b_27048c;
            } else {
                c.cycles += 5;
                c.g[V1][0] = lw(c, at(c, S0, 0x488));
                c.g[S2][0] = 0x1;
                c.g[V0][0] = lw(c, at(c, V1, 0x30));
                sw(c, at(c, S0, 0x484), c.g[V0][0]);
                'l_270458: loop {
                    'b_270470: {
                        c.cycles += 3;
                        c.g[A0][0] = lw(c, r32(c, S1));
                        if c.g[A0][0] == 0u64 {
                            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(0x1));
                            break 'b_270470;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x0027046c);
                            c.g[A1][0] = lw(c, at(c, S0, 0x484));
                            if callee::C0026d4d8.call(c, 0x0027046c).is_err() {
                                return;
                            }
                            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(0x1));
                            break 'b_270470;
                        }
                    }
                    c.cycles += 2;
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00270458);
                        }
                        continue 'l_270458;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00270484);
                        c.g[A1][0] = lw(c, at(c, A0, 0x488));
                        if callee::C0026db88.call(c, 0x00270484).is_err() {
                            return;
                        }
                        let t = true;
                        frame.restore(c, S0, 0x30);
                        if t {
                            break 'b_270520;
                        } else {
                            break 'b_27048c;
                        }
                    }
                }
            }
        }
        'b_270518: {
            c.cycles += 5;
            c.g[V1][0] = lw(c, at(c, S0, 0x484));
            c.g[A0][0] = lw(c, at(c, S0, 0x3cc));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = sx(at(c, V1, 0x1));
            if t {
                break 'b_270518;
            } else {
                'b_270504: {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, at(c, S0, 0x61c));
                    if c.g[V0][0] == 0u64 {
                        sw(c, at(c, S0, 0x484), c.g[A0][0]);
                        break 'b_270504;
                    } else {
                        c.cycles += 22;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002704b8);
                        c.g[A1][0] = 0u64;
                        if callee::C002702f8.call(c, 0x002704b8).is_err() {
                            return;
                        }
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002704c4);
                        c.g[A1][0] = 0x1;
                        if callee::C002703f0.call(c, 0x002704c4).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                        c.g[V1][0] = lw(c, at(c, V0, 0x84));
                        c.g[S1][0] = lw(c, at(c, V1, 0x84));
                        c.g[V0][0] = lw(c, at(c, S1, 0x4));
                        c.g[S0][0] = lw(c, at(c, V0, 0xa8));
                        c.g[RA][0] = sx(0x002704e0);
                        c.g[A0][0] = c.g[S0][0];
                        if call_ptr(c, 0x001620d0, 0x002704e0).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw(c, at(c, S0, 0x14));
                        c.g[A0][0] = lh(c, at(c, V0, 0x20));
                        c.g[V1][0] = lw(c, at(c, V0, 0x24));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002704f4);
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x002704f4).is_err() {
                            return;
                        }
                        c.g[RA][0] = sx(0x002704fc);
                        c.g[A0][0] = c.g[S1][0];
                        if call_ptr(c, 0x0022e840, 0x002704fc).is_err() {
                            return;
                        }
                        let t = true;
                        frame.restore(c, S0, 0x30);
                        if t {
                            break 'b_270520;
                        } else {
                            break 'b_270504;
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = 0x3;
                c.g[RA][0] = sx(0x00270510);
                c.g[A0][0] = c.g[S0][0];
                if callee::C0026f4a0.call(c, 0x00270510).is_err() {
                    return;
                }
                let t = true;
                frame.restore(c, S0, 0x30);
                if t {
                    break 'b_270520;
                } else {
                    break 'b_270518;
                }
            }
        }
        c.cycles += 2;
        sw(c, at(c, S0, 0x484), c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        break 'b_270520;
    }
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x40));
    c.pc = t;
    return;
}

/// `void func_00169DB0(void* p)` (0x169db0): update a target-script camera
/// algorithm: take the target camera's position and orientation, convert
/// degrees to radians, add the rig's yaw/pitch offsets, build the position
/// from the orbit matrix, aim at the target point and apply the wobble
/// parameters.
pub fn target_script_camera_algo_update(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    c.cycles += 117;
    frame.save(c, S0, 0xa0);
    frame.save(c, S1, 0x90);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S3, 0x70);
    frame.save(c, S4, 0x60);
    frame.save_ra(c, 0x50);
    c.g[S4][0] = sx(at(c, S0, 0x74));
    frame.save(c, S2, 0x80);
    c.g[V1][0] = lw(c, at(c, S0, 0x14));
    c.g[V0][0] = lw(c, at(c, V1, 0x14));
    c.g[S2][0] = lw(c, at(c, V0, 0x8));
    c.g[RA][0] = sx(0x00169de8);
    c.g[A0][0] = c.g[S2][0];
    if call_ptr(c, 0x0027c2a8, 0x00169de8).is_err() {
        return;
    }
    c.g[S1][0] = c.g[V0][0];
    c.g[S3][0] = lw(c, at(c, S0, 0x14));
    c.lwc1(1, r32(c, S1));
    c.g[A0][0] = c.g[S2][0];
    c.g[S3][0] = sx(at(c, S3, 0x30));
    c.g[A1][0] = sx(at(c, S0, 0x4c));
    c.swc1(1, at(c, S0, 0x4));
    c.lwc1(0, at(c, S1, 0x4));
    c.g[RA][0] = sx(0x00169e10);
    c.swc1(0, at(c, S0, 0x8));
    if call_ptr(c, 0x0027c300, 0x00169e10).is_err() {
        return;
    }
    c.lwc1(1, r32(c, S1));
    c.g[A0][0] = sx(at(c, SP, 0x40));
    c.lwc1(0, at(c, S0, 0x68));
    c.g[A1][0] = sx(at(c, SP, 0x44));
    c.lwc1(4, r32(c, GP).wrapping_sub(0x6520));
    c.swc1(1, at(c, S0, 0x4));
    c.mul_s(0, 0, 4);
    c.lwc1(2, at(c, S0, 0x60));
    c.lwc1(1, at(c, S1, 0x4));
    c.g[V0][0] = lw(c, at(c, S0, 0x14));
    c.mul_s(2, 2, 4);
    c.swc1(1, at(c, S0, 0x8));
    c.swc1(0, r32(c, S0));
    c.lwc1(1, at(c, S0, 0x5c));
    c.lwc1(0, at(c, V0, 0x78));
    c.mul_s(1, 1, 4);
    c.lwc1(3, at(c, S0, 0x64));
    c.add_s(2, 2, 0);
    c.lwc1(5, at(c, S0, 0x4c));
    c.mul_s(3, 3, 4);
    c.lwc1(6, at(c, S0, 0x50));
    c.lwc1(7, at(c, S0, 0x54));
    c.swc1(2, at(c, S0, 0x44));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 4);
    c.lwc1(0, at(c, V0, 0x74));
    c.add_s(1, 1, 0);
    c.swc1(1, at(c, S0, 0x48));
    c.lwc1(0, at(c, V0, 0x70));
    c.swc1(5, at(c, SP, 0x10));
    c.add_s(3, 3, 0);
    c.swc1(6, at(c, SP, 0x14));
    c.swc1(7, at(c, SP, 0x18));
    c.swc1(4, at(c, SP, 0x1c));
    c.swc1(3, at(c, S0, 0x40));
    c.lqc2(8, at(c, SP, 0x10));
    c.lqc2(4, r32(c, S3));
    c.lqc2(5, at(c, S3, 0x10));
    c.lqc2(6, at(c, S3, 0x20));
    c.lqc2(7, at(c, S3, 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, at(c, SP, 0x20));
    lq(c, 2, at(c, SP, 0x20));
    c.lwc1(12, at(c, S0, 0x44));
    sq(c, 2, at(c, S0, 0x20));
    c.g[RA][0] = sx(0x00169ed8);
    sq(c, 2, r32(c, SP));
    if call_ptr(c, 0x0031be50, 0x00169ed8).is_err() {
        return;
    }
    c.lwc1(12, at(c, S0, 0x48));
    c.g[A0][0] = sx(at(c, SP, 0x48));
    c.g[RA][0] = sx(0x00169ee8);
    c.g[A1][0] = sx(at(c, SP, 0x4c));
    if call_ptr(c, 0x0031be50, 0x00169ee8).is_err() {
        return;
    }
    c.lwc1(1, at(c, SP, 0x4c));
    c.g[A0][0] = c.g[S4][0];
    c.lwc1(0, at(c, SP, 0x44));
    c.lwc1(2, at(c, SP, 0x40));
    c.mul_s(0, 1, 0);
    c.lwc1(3, at(c, SP, 0x48));
    c.mul_s(1, 1, 2);
    sw(c, at(c, SP, 0xc), 0u64);
    c.swc1(3, at(c, SP, 0x8));
    c.swc1(0, r32(c, SP));
    c.swc1(1, at(c, SP, 0x4));
    c.lwc1(0, at(c, S0, 0x58));
    c.mfc1(3, 0);
    c.lqc2(4, r32(c, SP));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at(c, SP, 0x30));
    lq(c, 2, at(c, SP, 0x30));
    sq(c, 2, at(c, SP, 0x20));
    c.lqc2(3, at(c, S0, 0x20));
    c.lqc2(4, at(c, SP, 0x20));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x30));
    lq(c, 2, at(c, SP, 0x30));
    c.lwc1(12, at(c, S0, 0x6c));
    sq(c, 2, at(c, S0, 0x30));
    c.g[RA][0] = sx(0x00169f58);
    sq(c, 2, at(c, SP, 0x10));
    if call_ptr(c, 0x00160028, 0x00169f58).is_err() {
        return;
    }
    c.lwc1(12, at(c, S0, 0x70));
    c.g[RA][0] = sx(0x00169f64);
    c.g[A0][0] = c.g[S4][0];
    if call_ptr(c, 0x00160130, 0x00169f64).is_err() {
        return;
    }
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    c.g[RA][0] = ld(c, at(c, SP, 0x50));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xb0));
    c.pc = t;
    return;
}

/// `void func_00169570(void* p)` (0x169570): update a manual-script camera
/// algorithm: take the camera's offset and the rig matrix, place the target,
/// compute the eye position from the distance/angles and derive the yaw and
/// pitch from the view vector, then apply the wobble parameters.
pub fn manual_script_camera_algo_update(c: &mut Cpu) {
    let frame = Frame::open(c, 0x90);
    'b_169744: {
        'b_16973c: {
            'b_169700: {
                c.cycles += 89;
                frame.save(c, S1, 0x50);
                frame.save(c, S2, 0x40);
                c.swc1(22, at(c, SP, 0x80));
                c.g[S2][0] = c.g[A0][0];
                c.swc1(21, at(c, SP, 0x78));
                c.swc1(20, at(c, SP, 0x70));
                frame.save(c, S0, 0x60);
                frame.save_ra(c, 0x30);
                c.mtc1(0, 22);
                c.g[V0][0] = lw(c, at(c, S2, 0x14));
                c.g[V1][0] = lw(c, at(c, V0, 0x14));
                c.g[S0][0] = lw(c, at(c, V1, 0x8));
                c.g[RA][0] = sx(0x001695ac);
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, 0x0027c098, 0x001695ac).is_err() {
                    return;
                }
                c.lwc1(1, r32(c, V0));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = lw(c, at(c, S2, 0x14));
                c.g[A1][0] = sx(at(c, S2, 0x4c));
                c.swc1(1, at(c, S2, 0x4));
                c.g[S1][0] = sx(at(c, S1, 0x30));
                c.lwc1(0, at(c, V0, 0x4));
                c.g[RA][0] = sx(0x001695d0);
                c.swc1(0, at(c, S2, 0x8));
                if call_ptr(c, 0x0027c0f0, 0x001695d0).is_err() {
                    return;
                }
                c.lwc1(0, at(c, S2, 0x68));
                c.lwc1(4, r32(c, GP).wrapping_sub(0x6544));
                c.lwc1(1, at(c, S2, 0x4c));
                c.lwc1(2, at(c, S2, 0x50));
                c.mul_s(0, 0, 4);
                c.lwc1(3, at(c, S2, 0x54));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 5);
                c.swc1(1, r32(c, SP));
                c.swc1(2, at(c, SP, 0x4));
                c.swc1(3, at(c, SP, 0x8));
                c.swc1(5, at(c, SP, 0xc));
                c.swc1(0, r32(c, S2));
                lq(c, 2, r32(c, SP));
                sq(c, 2, at(c, S2, 0x30));
                c.lqc2(8, at(c, S2, 0x30));
                c.lqc2(4, r32(c, S1));
                c.lqc2(5, at(c, S1, 0x10));
                c.lqc2(6, at(c, S1, 0x20));
                c.lqc2(7, at(c, S1, 0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, at(c, SP, 0x10));
                lq(c, 2, at(c, SP, 0x10));
                c.lwc1(0, at(c, S2, 0x64));
                sq(c, 2, at(c, S2, 0x30));
                sq(c, 2, r32(c, SP));
                c.mul_s(0, 0, 4);
                c.g[V0][0] = lw(c, at(c, S2, 0x14));
                c.lwc1(4, at(c, S2, 0x58));
                c.lwc1(1, at(c, V0, 0x70));
                c.lwc1(2, at(c, S2, 0x5c));
                c.add_s(0, 0, 1);
                c.lwc1(3, at(c, S2, 0x60));
                c.swc1(4, at(c, SP, 0x10));
                c.swc1(2, at(c, SP, 0x14));
                c.swc1(0, at(c, S2, 0x40));
                c.swc1(3, at(c, SP, 0x18));
                c.swc1(5, at(c, SP, 0x1c));
                c.lqc2(8, at(c, SP, 0x10));
                c.lqc2(4, r32(c, S1));
                c.lqc2(5, at(c, S1, 0x10));
                c.lqc2(6, at(c, S1, 0x20));
                c.lqc2(7, at(c, S1, 0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, at(c, SP, 0x20));
                lq(c, 2, at(c, SP, 0x20));
                sq(c, 2, r32(c, SP));
                sq(c, 2, at(c, S2, 0x20));
                c.lqc2(3, at(c, S2, 0x20));
                c.lqc2(4, at(c, S2, 0x30));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, SP, 0x10));
                lq(c, 2, at(c, SP, 0x10));
                sq(c, 2, r32(c, SP));
                c.lwc1(20, r32(c, SP));
                c.c_eq_s(20, 22);
                let t = !c.fcc();
                c.lwc1(21, at(c, SP, 0x4));
                if t {
                    break 'b_169700;
                } else {
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let t = c.fcc();
                    c.mov_s(12, 21);
                    if t {
                        break 'b_16973c;
                    } else {
                        c.cycles += 4;
                        c.c_le_s(22, 21);
                        c.lwc1(12, r32(c, GP).wrapping_sub(0x6540));
                        let t = !c.fcc();
                        c.lwc1(0, r32(c, SP));
                        if t {
                            break 'b_16973c;
                        } else {
                            c.cycles += 3;
                            c.lwc1(12, r32(c, GP).wrapping_sub(0x653c));
                            let t = true;
                            c.lwc1(1, at(c, SP, 0x4));
                            if t {
                                break 'b_169744;
                            } else {
                                break 'b_169700;
                            }
                        }
                    }
                }
            }
            c.cycles += 7;
            c.div_s(12, 21, 20);
            c.g[RA][0] = sx(0x0016970c);
            if call_ptr(c, 0x0031c228, 0x0016970c).is_err() {
                return;
            }
            c.c_lt_s(20, 22);
            let t = !c.fcc();
            c.mov_s(12, 0);
            if t {
                break 'b_16973c;
            } else {
                'b_169734: {
                    c.cycles += 3;
                    c.c_lt_s(22, 21);
                    let t = !c.fcc();

                    if t {
                        break 'b_169734;
                    } else {
                        c.cycles += 3;
                        c.lwc1(0, r32(c, GP).wrapping_sub(0x6538));
                        let t = true;
                        c.add_s(12, 12, 0);
                        if t {
                            break 'b_16973c;
                        } else {
                            break 'b_169734;
                        }
                    }
                }
                c.cycles += 2;
                c.lwc1(0, r32(c, GP).wrapping_sub(0x6534));
                c.sub_s(12, 12, 0);
                break 'b_16973c;
            }
        }
        c.cycles += 2;
        c.lwc1(0, r32(c, SP));
        c.lwc1(1, at(c, SP, 0x4));
        break 'b_169744;
    }
    'b_1697e8: {
        'b_1697e0: {
            'b_1697a0: {
                c.cycles += 12;
                c.mul_s(2, 0, 0);
                c.swc1(12, at(c, S2, 0x44));
                c.mul_s(3, 1, 1);
                c.swc1(0, at(c, SP, 0x10));
                c.mtc1(0, 22);
                c.swc1(1, at(c, SP, 0x14));
                c.add_s(2, 2, 3);
                c.lwc1(21, at(c, SP, 0x8));
                c.sqrt_s(20, 2);
                c.c_eq_s(20, 22);
                let t = !c.fcc();

                if t {
                    break 'b_1697a0;
                } else {
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let t = c.fcc();
                    c.mov_s(0, 21);
                    if t {
                        break 'b_1697e0;
                    } else {
                        c.cycles += 4;
                        c.c_le_s(22, 21);
                        c.lwc1(0, r32(c, GP).wrapping_sub(0x6530));
                        let t = !c.fcc();
                        c.g[S0][0] = sx(at(c, S2, 0x74));
                        if t {
                            break 'b_1697e0;
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, r32(c, GP).wrapping_sub(0x652c));
                            let t = true;
                            c.lwc1(12, at(c, S2, 0x6c));
                            if t {
                                break 'b_1697e8;
                            } else {
                                break 'b_1697a0;
                            }
                        }
                    }
                }
            }
            'b_1697dc: {
                c.cycles += 7;
                c.div_s(12, 21, 20);
                c.g[RA][0] = sx(0x001697ac);
                if call_ptr(c, 0x0031c228, 0x001697ac).is_err() {
                    return;
                }
                c.c_lt_s(20, 22);
                let t = !c.fcc();
                c.mov_s(12, 0);
                if t {
                    break 'b_1697dc;
                } else {
                    'b_1697d4: {
                        c.cycles += 3;
                        c.c_lt_s(22, 21);
                        let t = !c.fcc();

                        if t {
                            break 'b_1697d4;
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, r32(c, GP).wrapping_sub(0x6528));
                            let t = true;
                            c.add_s(12, 12, 0);
                            if t {
                                break 'b_1697dc;
                            } else {
                                break 'b_1697d4;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.lwc1(0, r32(c, GP).wrapping_sub(0x6524));
                    c.sub_s(12, 12, 0);
                    break 'b_1697dc;
                }
            }
            c.cycles += 1;
            c.mov_s(0, 12);
            break 'b_1697e0;
        }
        c.cycles += 2;
        c.g[S0][0] = sx(at(c, S2, 0x74));
        c.lwc1(12, at(c, S2, 0x6c));
        break 'b_1697e8;
    }
    c.cycles += 15;
    c.swc1(0, at(c, S2, 0x48));
    c.g[RA][0] = sx(0x001697f4);
    c.g[A0][0] = c.g[S0][0];
    if call_ptr(c, 0x00160028, 0x001697f4).is_err() {
        return;
    }
    c.lwc1(12, at(c, S2, 0x70));
    c.g[RA][0] = sx(0x00169800);
    c.g[A0][0] = c.g[S0][0];
    if call_ptr(c, 0x00160130, 0x00169800).is_err() {
        return;
    }
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    c.g[RA][0] = ld(c, at(c, SP, 0x30));
    c.lwc1(22, at(c, SP, 0x80));
    c.lwc1(21, at(c, SP, 0x78));
    c.lwc1(20, at(c, SP, 0x70));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x90));
    c.pc = t;
    return;
}

/// `void func_001BB840(void* self, void* box, int month, int year, int sel)`
/// (0x1bb840): fill the creation screen's day list box: clear it, then for
/// every day of the given month add a numbered entry and select the requested
/// day (clamped to the month's length).
pub fn c_fe_state_create_step2_fill_day_list(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    'b_1bb8ec: {
        c.cycles += 29;
        frame.save(c, S1, 0x90);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S6, 0x40);
        c.g[A1][0] = c.g[A2][0];
        frame.save(c, S2, 0x80);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S7, 0x30);
        c.g[A2][0] = c.g[A3][0];
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save_ra(c, 0x20);
        c.g[RA][0] = sx(0x001bb884);
        c.g[S0][0] = 0x1;
        if callee::C001bb9b0.call(c, 0x001bb884).is_err() {
            return;
        }
        c.g[S3][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001bb890);
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, 0x0039a670, 0x001bb890).is_err() {
            return;
        }
        c.g[V0][0] = lw(c, at(c, S1, 0x14));
        c.g[V1][0] = 0xfffffffffffffffe;
        c.g[A1][0] = sx(at(c, S3, 0x1));
        c.g[S7][0] = sx(at(c, SP, 0x10));
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A0][0] = ((c.g[S0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[V0][0] = c.g[V0][0] | 0x80;
        let t = c.g[A0][0] == 0u64;
        sw(c, at(c, S1, 0x14), c.g[V0][0]);
        if t {
            break 'b_1bb8ec;
        } else {
            c.cycles += 3;
            c.g[S4][0] = c.g[A1][0];
            c.g[S5][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            'l_1bb8c0: loop {
                c.cycles += 11;
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001bb8cc);
                c.g[A1][0] = sx(at(c, S5, 0x12c0));
                if call_ptr(c, 0x00417828, 0x001bb8cc).is_err() {
                    return;
                }
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x001bb8e0);
                c.g[S0][0] = sx(at(c, S0, 0x1));
                if call_ptr(c, 0x0039a3f8, 0x001bb8e0).is_err() {
                    return;
                }
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.g[SP][0];
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001bb8c0);
                    }
                    continue 'l_1bb8c0;
                } else {
                    break 'b_1bb8ec;
                }
            }
        }
    }
    'b_1bb904: {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S2][0] as i64)) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[A2][0] = c.g[S7][0];
        if t {
            break 'b_1bb904;
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw(c, at(c, S6, 0x6d0));
            c.g[S2][0] = c.g[S3][0];
            sw(c, at(c, V0, 0x2c), c.g[S3][0]);
            break 'b_1bb904;
        }
    }
    c.cycles += 24;
    c.g[A1][0] = c.g[S2][0];
    sb(c, at(c, SP, 0x10), 0u64);
    c.g[RA][0] = sx(0x001bb914);
    c.g[A0][0] = c.g[S1][0];
    if call_ptr(c, 0x0039a768, 0x001bb914).is_err() {
        return;
    }
    c.g[A1][0] = c.lbu(at(c, SP, 0x10));
    c.g[RA][0] = sx(0x001bb920);
    c.g[A0][0] = c.g[S1][0];
    if call_ptr(c, 0x0039a7a8, 0x001bb920).is_err() {
        return;
    }
    c.g[V0][0] = lw(c, at(c, S1, 0x8));
    c.g[A1][0] = 0x1;
    c.g[A0][0] = lh(c, at(c, V0, 0x48));
    c.g[V1][0] = lw(c, at(c, V0, 0x4c));
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x001bb938);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    if call_ptr(c, t, 0x001bb938).is_err() {
        return;
    }
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xb0));
    c.pc = t;
    return;
}

/// `void func_00176890(void* node, void* ctx)` (0x176890): walk an octree node
/// (children at `+0..+0x1c`, triggers at `+0x28`), activating every camera
/// trigger of kind 4 whose bounds accept the rider and inserting it into the
/// active list, then recurse into the eight children.
pub fn camera_trigger_man_collect_octree_node_triggers(c: &mut Cpu) {
    let frame = Frame::open(c, 0x90);
    'b_176958: {
        c.cycles += 14;
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S0, 0x80);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        sd(c, r32(c, SP), c.g[RA][0]);
        frame.save(c, S2, 0x60);
        c.g[S2][0] = lw(c, at(c, S5, 0x28));
        let t = c.g[S2][0] == 0u64;
        c.g[S4][0] = c.g[A1][0];
        if t {
            break 'b_176958;
        } else {
            c.cycles += 4;
            c.g[S7][0] = 0x4;
            c.g[S6][0] = 0x1;
            c.g[V0][0] = lw(c, at(c, S2, 0x8));
            'l_1768d8: loop {
                'b_176950: {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S7][0] {
                        c.g[S2][0] = lw(c, r32(c, S2));
                        break 'b_176950;
                    } else {
                        c.cycles += 18;
                        c.g[S3][0] = lw(c, at(c, S2, 0xc));
                        c.g[A0][0] = lw(c, at(c, S4, 0x4));
                        c.g[S0][0] = lw(c, at(c, S3, 0x8));
                        c.g[A2][0] = lw(c, at(c, A0, 0x6c0));
                        c.g[S1][0] = lw(c, at(c, S0, 0x24));
                        c.g[A0][0] = sx(at(c, A0, 0x6c0));
                        c.g[V1][0] = lh(c, at(c, A2, 0x28));
                        c.g[A1][0] = lh(c, at(c, S1, 0x38));
                        c.g[V0][0] = lw(c, at(c, A2, 0x2c));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x00176910);
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, A1)));
                        if call_ptr(c, t, 0x00176910).is_err() {
                            return;
                        }
                        c.g[V1][0] = lw(c, at(c, S1, 0x3c));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00176920);
                        c.g[A1][0] = c.g[V0][0];
                        if call_ptr(c, t, 0x00176920).is_err() {
                            return;
                        }
                        if c.g[V0][0] == 0u64 {
                            c.g[S2][0] = lw(c, r32(c, S2));
                            break 'b_176950;
                        } else {
                            'b_17694c: {
                                'b_176944: {
                                    c.cycles += 5;
                                    c.g[A0][0] = lw(c, r32(c, S4));
                                    c.g[RA][0] = sx(0x00176934);
                                    c.g[A1][0] = c.g[S3][0];
                                    if callee::C0016c6b0.call(c, 0x00176934).is_err() {
                                        return;
                                    }
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A0][0] = lw(c, r32(c, S4));
                                        break 'b_176944;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        sw(c, at(c, V0, 0xc), c.g[S6][0]);
                                        if t {
                                            break 'b_17694c;
                                        } else {
                                            break 'b_176944;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x0017694c);
                                c.g[A1][0] = c.g[S3][0];
                                if callee::C0016c718.call(c, 0x0017694c).is_err() {
                                    return;
                                }
                                break 'b_17694c;
                            }
                            c.cycles += 1;
                            c.g[S2][0] = lw(c, r32(c, S2));
                            break 'b_176950;
                        }
                    }
                }
                c.cycles += 2;
                if c.g[S2][0] != 0u64 {
                    c.g[V0][0] = lw(c, at(c, S2, 0x8));
                    if event_due(c) {
                        deliver_event_here(c, 0x001768d8);
                    }
                    continue 'l_1768d8;
                } else {
                    break 'b_176958;
                }
            }
        }
    }
    'b_176970: {
        c.cycles += 3;
        c.g[A0][0] = lw(c, r32(c, S5));
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at(c, S5, 0x4));
            break 'b_176970;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0017696c);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x0017696c).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at(c, S5, 0x4));
            break 'b_176970;
        }
    }
    'b_176984: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at(c, S5, 0x8));
            break 'b_176984;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x00176980);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x00176980).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at(c, S5, 0x8));
            break 'b_176984;
        }
    }
    'b_176998: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at(c, S5, 0xc));
            break 'b_176998;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x00176994);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x00176994).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at(c, S5, 0xc));
            break 'b_176998;
        }
    }
    'b_1769ac: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at(c, S5, 0x10));
            break 'b_1769ac;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001769a8);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x001769a8).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at(c, S5, 0x10));
            break 'b_1769ac;
        }
    }
    'b_1769c0: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at(c, S5, 0x14));
            break 'b_1769c0;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001769bc);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x001769bc).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at(c, S5, 0x14));
            break 'b_1769c0;
        }
    }
    'b_1769d4: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at(c, S5, 0x18));
            break 'b_1769d4;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001769d0);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x001769d0).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at(c, S5, 0x18));
            break 'b_1769d4;
        }
    }
    'b_1769e8: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at(c, S5, 0x1c));
            break 'b_1769e8;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001769e4);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x001769e4).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at(c, S5, 0x1c));
            break 'b_1769e8;
        }
    }
    'b_1769fc: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        frame.restore(c, S0, 0x80);
        if t {
            break 'b_1769fc;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001769f8);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x001769f8).is_err() {
                return;
            }
            frame.restore(c, S0, 0x80);
            break 'b_1769fc;
        }
    }
    c.cycles += 10;
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, S6, 0x20);
    frame.restore(c, S7, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x90));
    c.pc = t;
    return;
}

/// `void func_0014B560(void* self, int rider, int chr, int key, int flag)`
/// (0x14b560): unlock bolt `key` of character `chr`: mark the entry unlocked,
/// unlock every bolt whose scale is the negative key, then either spend the
/// bolt's price or grant it for free.
pub fn be_bolt_int_unlock_bolt(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    'b_14b650: {
        c.cycles += 44;
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        sq(c, 30, at(c, SP, 0x20));
        sw(c, r32(c, SP), c.g[T0][0]);
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S7, 0x30);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x0014b5a0);
        c.g[S6][0] = c.g[A1][0];
        if callee::C0014dc80.call(c, 0x0014b5a0).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014b5ac);
        c.g[A1][0] = 0xb;
        if callee::C0014dd58.call(c, 0x0014b5ac).is_err() {
            return;
        }
        c.g[V1][0] = 0xf88;
        c.g[A0][0] = 0x9b50;
        c.mult(0, V1, S4, V1);
        c.g[FP][0] = c.g[V0][0];
        c.mult(0, A0, S6, A0);
        c.g[V0][0] = 0x4a0000;
        c.g[V0][0] = sx(at(c, V0, 0x6ca8));
        c.g[A2][0] = 0x1;
        c.g[A1][0] = c.g[S5][0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[S2][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        c.g[RA][0] = sx(0x0014b5e0);
        c.g[A0][0] = c.g[S2][0];
        if callee::C00151c48.call(c, 0x0014b5e0).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x0014b5e8);
        if callee::C0014bdb8.call(c, 0x0014b5e8).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014b5f4);
        c.g[A1][0] = c.g[S4][0];
        if callee::C0014d998.call(c, 0x0014b5f4).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x0014b5fc);
        c.g[S0][0] = c.g[V0][0];
        if callee::C0014bdb8.call(c, 0x0014b5fc).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0014b608);
        c.g[A1][0] = c.g[S4][0];
        if callee::C0014d988.call(c, 0x0014b608).is_err() {
            return;
        }
        let t = (c.g[V0][0] as i64) <= 0;

        if t {
            break 'b_14b650;
        } else {
            c.cycles += 4;
            c.g[S1][0] = c.g[V0][0];
            c.g[S7][0] = 0xa;
            c.g[S3][0] = sx(0u32.wrapping_sub(r32(c, S5)));
            'l_14b620: loop {
                'b_14b62c: {
                    c.cycles += 3;
                    c.g[V0][0] = lh(c, at(c, S0, 0xe));
                    if (c.g[V0][0] as i64) > 0 {
                        c.mult(0, V0, V0, S7);
                        break 'b_14b62c;
                    } else {
                        break 'b_14b62c;
                    }
                }
                'b_14b648: {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x1));
                        break 'b_14b648;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = lh(c, at(c, S0, 0x4));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x0014b644);
                        c.g[A2][0] = 0x1;
                        if callee::C00151c48.call(c, 0x0014b644).is_err() {
                            return;
                        }
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(0x1));
                        break 'b_14b648;
                    }
                }
                c.cycles += 2;
                let t = c.g[S1][0] != 0u64;
                c.g[S0][0] = sx(at(c, S0, 0x38));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014b620);
                    }
                    continue 'l_14b620;
                } else {
                    break 'b_14b650;
                }
            }
        }
    }
    'b_14b68c: {
        'b_14b67c: {
            c.cycles += 9;
            c.g[RA][0] = sx(0x0014b658);
            if callee::C0014bdb8.call(c, 0x0014b658).is_err() {
                return;
            }
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0014b66c);
            c.g[A1][0] = c.g[S4][0];
            if callee::C0014d448.call(c, 0x0014b66c).is_err() {
                return;
            }
            let t = (c.g[V0][0] as i64) >= 0;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_14b67c;
            } else {
                c.cycles += 2;
                let t = true;
                c.g[S0][0] = 0u64;
                if t {
                    break 'b_14b68c;
                } else {
                    break 'b_14b67c;
                }
            }
        }
        c.cycles += 4;
        c.g[V0][0] = 0x38;
        c.g[V1][0] = lw(c, at(c, S0, 0x4));
        c.mult(0, V0, A0, V0);
        c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        break 'b_14b68c;
    }
    'b_14b6d0: {
        'b_14b6c0: {
            c.cycles += 3;
            c.g[V0][0] = lw(c, r32(c, SP));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[FP][0];
            if t {
                break 'b_14b6c0;
            } else {
                'b_14b6ac: {
                    c.cycles += 3;
                    c.g[A3][0] = lh(c, at(c, S0, 0xe));
                    let t = (c.g[A3][0] as i64) <= 0;
                    c.g[V0][0] = 0xa;
                    if t {
                        break 'b_14b6ac;
                    } else {
                        c.cycles += 2;
                        c.mult(0, A3, A3, V0);
                        c.g[A0][0] = c.g[FP][0];
                        break 'b_14b6ac;
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x0014b6b8);
                c.g[A2][0] = c.g[S4][0];
                if callee::C00150b48.call(c, 0x0014b6b8).is_err() {
                    return;
                }
                let t = true;
                frame.restore(c, S0, 0xa0);
                if t {
                    break 'b_14b6d0;
                } else {
                    break 'b_14b6c0;
                }
            }
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[S6][0];
        c.g[RA][0] = sx(0x0014b6cc);
        c.g[A2][0] = c.g[S4][0];
        if callee::C00150928.call(c, 0x0014b6cc).is_err() {
            return;
        }
        frame.restore(c, S0, 0xa0);
        break 'b_14b6d0;
    }
    c.cycles += 11;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    lq(c, 30, at(c, SP, 0x20));
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xb0));
    c.pc = t;
    return;
}

/// `void func_00364360(void* self, void** pp)` (0x364360): emit the default
/// render state: a state packet from the GS state word, the alpha, clamp,
/// zbuffer and test packets for both passes, and the finishing packet.
pub fn defer_man_emit_default_state(c: &mut Cpu) {
    let frame = Frame::open(c, 0x90);
    c.cycles += 16;
    c.g[A2][0] = 0xffffffffffffffff;
    frame.save(c, S2, 0x60);
    c.g[V1][0] = sx(at(c, SP, 0x10));
    frame.save(c, S3, 0x50);
    c.g[S2][0] = c.g[A1][0];
    frame.save(c, S4, 0x40);
    c.g[A1][0] = 0u64;
    frame.save(c, S1, 0x70);
    c.g[S4][0] = c.g[A0][0];
    frame.save(c, S0, 0x80);
    c.g[A0][0] = c.g[SP][0];
    frame.save_ra(c, 0x30);
    c.g[S1][0] = sx(at(c, SP, 0x20));
    c.g[S3][0] = lw(c, at(c, GP, 0x2a90));
    'l_3643a0: loop {
        c.cycles += 7;
        sh(c, r32(c, V1), c.g[A2][0]);
        c.g[A1][0] = sx(at(c, A1, 0x1));
        c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[V1][0] = sx(at(c, V1, 0x2));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x003643a0);
            }
            continue 'l_3643a0;
        } else {
            c.cycles += 126;
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x003643c8);
            c.g[A2][0] = 0x10;
            if call_ptr(c, 0x00416210, 0x003643c8).is_err() {
                return;
            }
            c.g[S0][0] = 0xf0000;
            c.g[A0][0] = lw(c, r32(c, S2));
            c.g[V0][0] = 0x500000;
            c.g[T0][0] = sx(at(c, V0, 0x1420));
            ldl(c, 5, r32(c, T0));
            ldl(c, 6, at(c, T0, 0x8));
            c.g[A3][0] = lw(c, at(c, T0, 0x10));
            sdl(c, 5, r32(c, SP));
            sdl(c, 6, at(c, SP, 0x8));
            sw(c, at(c, SP, 0x10), c.g[A3][0]);
            c.g[V1][0] = 0x10000000;
            c.g[V1][0] = c.g[V1][0] | 0x1;
            sd(c, r32(c, A0), c.g[V1][0]);
            c.g[A1][0] = 0x8000;
            c.g[A1][0] = c.g[A1][0] << 45;
            c.g[T1][0] = 0x3fff0000;
            c.g[T0][0] = c.g[S2][0];
            c.g[V1][0] = lw(c, r32(c, S2));
            c.g[T1][0] = c.g[T1][0] | 0xffff;
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] = 0u64;
            sd(c, at(c, V1, 0x8), 0u64);
            c.g[S0][0] = c.g[S0][0] | 0xf000;
            c.g[V0][0] = lw(c, r32(c, S2));
            sd(c, at(c, V0, 0x10), 0u64);
            c.g[V1][0] = lw(c, r32(c, S2));
            sd(c, at(c, V1, 0x18), c.g[A1][0]);
            c.g[A1][0] = lw(c, r32(c, SP));
            c.g[V0][0] = lw(c, r32(c, S2));
            c.g[A3][0] = lw(c, at(c, SP, 0xc));
            c.g[A1][0] = c.g[A1][0] & 0x3c0;
            c.g[V0][0] = sx(at(c, V0, 0x20));
            c.g[A1][0] = sx(r32(c, A1) >> 6);
            c.g[A3][0] = c.g[A3][0] & c.g[T1][0];
            c.g[RA][0] = sx(0x00364468);
            sw(c, r32(c, S2), c.g[V0][0]);
            if callee::C00364cd0.call(c, 0x00364468).is_err() {
                return;
            }
            c.g[V1][0] = lw(c, r32(c, S2));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = lw(c, r32(c, SP));
            c.g[A2][0] = 0u64;
            c.g[V0][0] = sx(at(c, V1, 0x30));
            sw(c, at(c, SP, 0x20), 0u64);
            c.g[A1][0] = c.g[A1][0] & 0xc;
            sw(c, at(c, SP, 0x24), c.g[V0][0]);
            sw(c, at(c, SP, 0x28), c.g[V1][0]);
            c.g[RA][0] = sx(0x00364494);
            c.g[A1][0] = sx(r32(c, A1) >> 2);
            if callee::C003625c0.call(c, 0x00364494).is_err() {
                return;
            }
            c.g[A1][0] = lw(c, at(c, SP, 0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = lw(c, r32(c, SP));
            c.g[A3][0] = 0u64;
            c.g[A1][0] = c.g[A1][0] & 0x7c;
            c.g[A2][0] = sx(r32(c, A2) >> 12);
            c.g[A1][0] = sx(r32(c, A1) >> 2);
            c.g[RA][0] = sx(0x003644b8);
            c.g[A2][0] = c.g[A2][0] & 0xff;
            if callee::C00362478.call(c, 0x003644b8).is_err() {
                return;
            }
            c.g[A3][0] = lw(c, at(c, SP, 0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = lw(c, at(c, S3, 0x5a80));
            c.g[T0][0] = 0u64;
            c.g[A2][0] = lw(c, at(c, S3, 0x5a40));
            c.g[A3][0] = sx(r32(c, A3) >> 22);
            c.g[RA][0] = sx(0x003644d8);
            c.g[A3][0] = c.g[A3][0] & 0x1;
            if callee::C00362660.call(c, 0x003644d8).is_err() {
                return;
            }
            c.g[A2][0] = lw(c, at(c, SP, 0x4));
            c.g[T0][0] = 0u64;
            c.g[A0][0] = c.g[S1][0];
            c.g[A3][0] = c.g[A2][0] & c.g[S0][0];
            c.g[A1][0] = sx(r32(c, A2) >> 23);
            c.g[A2][0] = sx(r32(c, A2) >> 20);
            c.g[A3][0] = sx(r32(c, A3) >> 12);
            c.g[A1][0] = c.g[A1][0] & 0x3;
            c.g[RA][0] = sx(0x00364500);
            c.g[A2][0] = c.g[A2][0] & 0x3;
            if callee::C003626d8.call(c, 0x00364500).is_err() {
                return;
            }
            c.g[A1][0] = lw(c, r32(c, SP));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0x1;
            c.g[A1][0] = c.g[A1][0] & 0x30;
            c.g[RA][0] = sx(0x00364518);
            c.g[A1][0] = sx(r32(c, A1) >> 4);
            if callee::C003625c0.call(c, 0x00364518).is_err() {
                return;
            }
            c.g[A1][0] = lw(c, at(c, SP, 0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = lw(c, r32(c, SP));
            c.g[A3][0] = 0x1;
            c.g[A1][0] = c.g[A1][0] & 0xf80;
            c.g[A2][0] = sx(r32(c, A2) >> 20);
            c.g[A1][0] = sx(r32(c, A1) >> 7);
            c.g[RA][0] = sx(0x0036453c);
            c.g[A2][0] = c.g[A2][0] & 0xff;
            if callee::C00362478.call(c, 0x0036453c).is_err() {
                return;
            }
            c.g[A3][0] = lw(c, at(c, SP, 0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = lw(c, at(c, S3, 0x5a40));
            c.g[T0][0] = 0x1;
            c.g[A1][0] = lw(c, at(c, S3, 0x5a80));
            c.g[A3][0] = sx(r32(c, A3) >> 23);
            c.g[RA][0] = sx(0x0036455c);
            c.g[A3][0] = c.g[A3][0] & 0x1;
            if callee::C00362660.call(c, 0x0036455c).is_err() {
                return;
            }
            c.g[A2][0] = lw(c, at(c, SP, 0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[T0][0] = 0x1;
            c.g[S0][0] = c.g[A2][0] & c.g[S0][0];
            c.g[A1][0] = sx(r32(c, A2) >> 25);
            c.g[A2][0] = sx(r32(c, A2) >> 22);
            c.g[A3][0] = sx(r32(c, S0) >> 12);
            c.g[A1][0] = c.g[A1][0] & 0x3;
            c.g[RA][0] = sx(0x00364584);
            c.g[A2][0] = c.g[A2][0] & 0x3;
            if callee::C003626d8.call(c, 0x00364584).is_err() {
                return;
            }
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x00364594);
            c.g[A2][0] = 0x1;
            if callee::C003627a8.call(c, 0x00364594).is_err() {
                return;
            }
            frame.restore(c, S0, 0x80);
            frame.restore(c, S1, 0x70);
            frame.restore(c, S2, 0x60);
            frame.restore(c, S3, 0x50);
            frame.restore(c, S4, 0x40);
            c.g[RA][0] = ld(c, at(c, SP, 0x30));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at(c, SP, 0x90));
            c.pc = t;
            return;
        }
    }
}

/// `void func_001C9210(void* self, void* text, const char* str, float* size)`
/// (0x1c9210): measure a popup text box for a `\n`/`\\`-separated string: split
/// it into lines, measure each line with the text object, accumulate the size
/// (adding 3 pixels per line break) and free the temporary buffer.
pub fn c_fe_popup_confirm_get_text_box_size(c: &mut Cpu) {
    let frame = Frame::open(c, 0xc0);
    'b_1c933c: {
        c.cycles += 33;
        frame.save(c, S4, 0x60);
        frame.save(c, S7, 0x30);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S2, 0x80);
        c.g[S7][0] = c.g[A2][0];
        frame.save(c, S5, 0x50);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S0, 0xa0);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0x90);
        c.g[A0][0] = c.g[S7][0];
        frame.save(c, S3, 0x70);
        c.g[S0][0] = 0u64;
        frame.save(c, S6, 0x40);
        c.g[S3][0] = 0u64;
        sq(c, 30, at(c, SP, 0x20));
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x001c9260);
        c.swc1(20, at(c, SP, 0xb0));
        if call_ptr(c, 0x00416810, 0x001c9260).is_err() {
            return;
        }
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = sx(at(c, V0, 0x1));
        c.g[A1][0] = sx(at(c, A1, 0x60d8));
        c.g[A2][0] = 0x100;
        c.g[RA][0] = sx(0x001c9278);
        c.g[A3][0] = 0u64;
        if call_ptr(c, 0x00317e30, 0x001c9278).is_err() {
            return;
        }
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x001c9288);
        sb(c, r32(c, S1), 0u64);
        if call_ptr(c, 0x00416810, 0x001c9288).is_err() {
            return;
        }
        c.g[S6][0] = c.g[V0][0];
        let t = (c.g[S6][0] as i64) <= 0;
        c.g[V0][0] = sx(r32(c, S7).wrapping_add(r32(c, S3)));
        if t {
            break 'b_1c933c;
        } else {
            c.cycles += 3;
            c.g[AT][0] = 0x40400000;
            c.mtc1(1, 20);
            c.g[FP][0] = 0xd;
            'l_1c92a0: loop {
                'b_1c9330: {
                    'b_1c9324: {
                        'b_1c9300: {
                            'b_1c92dc: {
                                c.cycles += 5;
                                c.g[A0][0] = lb(c, at(c, V0, 0x1));
                                c.g[V1][0] = lb(c, r32(c, V0));
                                c.g[V0][0] = 0x5c;
                                let t = c.g[V1][0] != c.g[V0][0];

                                if t {
                                    break 'b_1c92dc;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A0][0] != c.g[V1][0];
                                    c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                                    if t {
                                        break 'b_1c92dc;
                                    } else {
                                        c.cycles += 8;
                                        c.g[A0][0] = c.g[S4][0];
                                        sb(c, r32(c, V0), 0u64);
                                        c.g[A1][0] = c.g[S5][0];
                                        c.g[A2][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x001c92d4);
                                        c.g[A3][0] = c.g[SP][0];
                                        if callee::C001c84a8.call(c, 0x001c92d4).is_err() {
                                            return;
                                        }
                                        let t = true;
                                        c.g[S3][0] = sx(at(c, S3, 0x2));
                                        if t {
                                            break 'b_1c9300;
                                        } else {
                                            break 'b_1c92dc;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[FP][0];
                            c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                            if t {
                                break 'b_1c9324;
                            } else {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S4][0];
                                sb(c, r32(c, V0), 0u64);
                                c.g[A1][0] = c.g[S5][0];
                                c.g[A2][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001c92fc);
                                c.g[A3][0] = c.g[SP][0];
                                if callee::C001c84a8.call(c, 0x001c92fc).is_err() {
                                    return;
                                }
                                c.g[S3][0] = sx(at(c, S3, 0x1));
                                break 'b_1c9300;
                            }
                        }
                        c.cycles += 9;
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x001c9314);
                        c.g[S0][0] = 0u64;
                        if callee::C001c9b68.call(c, 0x001c9314).is_err() {
                            return;
                        }
                        c.lwc1(0, at(c, S2, 0x4));
                        c.add_s(0, 0, 20);
                        let t = true;
                        c.swc1(0, at(c, S2, 0x4));
                        if t {
                            break 'b_1c9330;
                        } else {
                            break 'b_1c9324;
                        }
                    }
                    c.cycles += 3;
                    c.g[S3][0] = sx(at(c, S3, 0x1));
                    sb(c, r32(c, V0), c.g[V1][0]);
                    c.g[S0][0] = sx(at(c, S0, 0x1));
                    break 'b_1c9330;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = sx(r32(c, S7).wrapping_add(r32(c, S3)));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001c92a0);
                    }
                    continue 'l_1c92a0;
                } else {
                    break 'b_1c933c;
                }
            }
        }
    }
    'b_1c936c: {
        c.cycles += 2;
        let t = c.g[S0][0] == 0u64;
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
        if t {
            break 'b_1c936c;
        } else {
            c.cycles += 10;
            c.g[A1][0] = c.g[S5][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S1][0];
            sb(c, r32(c, V0), 0u64);
            c.g[RA][0] = sx(0x001c935c);
            c.g[A3][0] = c.g[SP][0];
            if callee::C001c84a8.call(c, 0x001c935c).is_err() {
                return;
            }
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001c936c);
            c.g[A2][0] = c.g[SP][0];
            if callee::C001c9b68.call(c, 0x001c936c).is_err() {
                return;
            }
            break 'b_1c936c;
        }
    }
    'b_1c9380: {
        c.cycles += 2;
        let t = c.g[S1][0] == 0u64;
        frame.restore(c, S0, 0xa0);
        if t {
            break 'b_1c9380;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001c937c);
            c.g[A0][0] = c.g[S1][0];
            if call_ptr(c, 0x00317e98, 0x001c937c).is_err() {
                return;
            }
            frame.restore(c, S0, 0xa0);
            break 'b_1c9380;
        }
    }
    c.cycles += 12;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    lq(c, 30, at(c, SP, 0x20));
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    c.lwc1(20, at(c, SP, 0xb0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xc0));
    c.pc = t;
    return;
}

/// `void func_00267128(sPtrVec* self, void** pos, void* const& x)` (0x267128):
/// insert `x` into the pointer vector before `pos`, growing the buffer to
/// twice its size (or 1) when it is full.
pub fn buddy_man_message_vector_insert(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_2672a4: {
        'b_2671a4: {
            c.cycles += 14;
            frame.save(c, S1, 0x50);
            frame.save(c, S3, 0x30);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S4, 0x20);
            c.g[S3][0] = c.g[A1][0];
            frame.save(c, S0, 0x60);
            frame.save(c, S2, 0x40);
            frame.save(c, S5, 0x10);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.g[V1][0] = lw(c, at(c, S1, 0x8));
            c.g[V0][0] = lw(c, at(c, S1, 0xc));
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[S4][0] = c.g[A2][0];
            if t {
                break 'b_2671a4;
            } else {
                'b_267174: {
                    c.cycles += 2;
                    if c.g[V1][0] == 0u64 {
                        c.g[A0][0] = lw(c, at(c, S1, 0x8));
                        break 'b_267174;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, r32(c, V1).wrapping_sub(0x4));
                        sw(c, r32(c, V1), c.g[V0][0]);
                        c.g[A0][0] = lw(c, at(c, S1, 0x8));
                        break 'b_267174;
                    }
                }
                c.cycles += 12;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = sx(r32(c, A0).wrapping_sub(0x4));
                c.g[V0][0] = sx(at(c, A0, 0x4));
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, S3)));
                sw(c, at(c, S1, 0x8), c.g[V0][0]);
                c.g[A2][0] = sx((r32(c, A2) as i32 >> 2) as u32);
                c.g[A2][0] = sx(r32(c, A2) << 2);
                c.g[S0][0] = lw(c, r32(c, S4));
                c.g[RA][0] = sx(0x0026719c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, A2)));
                if call_ptr(c, 0x0041610c, 0x0026719c).is_err() {
                    return;
                }
                let t = true;
                sw(c, r32(c, S3), c.g[S0][0]);
                if t {
                    break 'b_2672a4;
                } else {
                    break 'b_2671a4;
                }
            }
        }
        'b_26720c: {
            'b_2671ec: {
                c.cycles += 9;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                c.g[S0][0] = 0x1;
                c.g[A0][0] = 0x1;
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(r32(c, V0)));
                c.g[V0][0] = sx((r32(c, V0) as i32 >> 2) as u32);
                c.g[V1][0] = sx(r32(c, V0) << 1);
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = c.g[V1][0];
                }
                let t = c.g[S0][0] != c.g[A0][0];
                c.g[S0][0] = sx(r32(c, S0) << 2);
                if t {
                    break 'b_2671ec;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0x4;
                    c.g[A1][0] = sx(at(c, A1, 0x4a8));
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x002671e4);
                    c.g[S5][0] = 0x4;
                    if call_ptr(c, 0x00317d70, 0x002671e4).is_err() {
                        return;
                    }
                    let t = true;
                    c.g[S2][0] = c.g[V0][0];
                    if t {
                        break 'b_26720c;
                    } else {
                        break 'b_2671ec;
                    }
                }
            }
            c.cycles += 8;
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx(at(c, A1, 0x4a8));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0x20000000;
            c.g[RA][0] = sx(0x00267204);
            c.g[A3][0] = 0u64;
            if call_ptr(c, 0x00317e30, 0x00267204).is_err() {
                return;
            }
            c.g[S5][0] = c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            break 'b_26720c;
        }
        'b_267238: {
            c.cycles += 9;
            c.g[A1][0] = lw(c, at(c, S1, 0x4));
            c.g[A0][0] = c.g[S2][0];
            c.g[V0][0] = sx(r32(c, S3).wrapping_sub(r32(c, A1)));
            c.g[A2][0] = sx((r32(c, V0) as i32 >> 2) as u32);
            c.g[S0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
            c.g[RA][0] = sx(0x00267228);
            c.g[A2][0] = sx(r32(c, A2) << 2);
            if call_ptr(c, 0x0041610c, 0x00267228).is_err() {
                return;
            }
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[S0][0];
            if t {
                break 'b_267238;
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw(c, r32(c, S4));
                sw(c, r32(c, S0), c.g[V0][0]);
                break 'b_267238;
            }
        }
        'b_267298: {
            'b_267294: {
                c.cycles += 14;
                c.g[A2][0] = lw(c, at(c, S1, 0x8));
                c.g[S0][0] = sx(at(c, V1, 0x4));
                c.g[A1][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, S3)));
                c.g[S3][0] = sx(r32(c, S0).wrapping_add(r32(c, A2)));
                c.g[A2][0] = sx((r32(c, A2) as i32 >> 2) as u32);
                c.g[RA][0] = sx(0x0026725c);
                c.g[A2][0] = sx(r32(c, A2) << 2);
                if call_ptr(c, 0x0041610c, 0x0026725c).is_err() {
                    return;
                }
                c.g[A0][0] = lw(c, at(c, S1, 0x4));
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
                let t = c.g[A0][0] == 0u64;
                c.g[V1][0] = sx((r32(c, V0) as i32 >> 2) as u32);
                if t {
                    break 'b_267294;
                } else {
                    'b_26728c: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_26728c;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00267284);
                            if call_ptr(c, 0x00317e50, 0x00267284).is_err() {
                                return;
                            }
                            let t = true;
                            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S5)));
                            if t {
                                break 'b_267298;
                            } else {
                                break 'b_26728c;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00267294);
                    if call_ptr(c, 0x00317e98, 0x00267294).is_err() {
                        return;
                    }
                    break 'b_267294;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S5)));
            break 'b_267298;
        }
        c.cycles += 3;
        sw(c, at(c, S1, 0x8), c.g[S3][0]);
        sw(c, at(c, S1, 0xc), c.g[V0][0]);
        sw(c, at(c, S1, 0x4), c.g[S2][0]);
        break 'b_2672a4;
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}

/// `void func_002672C8(sPtrVec* self, void** pos, void* const& x)` (0x2672c8):
/// the challenger list's insert, same shape as the message-list one.
pub fn buddy_man_challenger_vector_insert(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_267444: {
        'b_267344: {
            c.cycles += 14;
            frame.save(c, S1, 0x50);
            frame.save(c, S3, 0x30);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S4, 0x20);
            c.g[S3][0] = c.g[A1][0];
            frame.save(c, S0, 0x60);
            frame.save(c, S2, 0x40);
            frame.save(c, S5, 0x10);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.g[V1][0] = lw(c, at(c, S1, 0x8));
            c.g[V0][0] = lw(c, at(c, S1, 0xc));
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[S4][0] = c.g[A2][0];
            if t {
                break 'b_267344;
            } else {
                'b_267314: {
                    c.cycles += 2;
                    if c.g[V1][0] == 0u64 {
                        c.g[A0][0] = lw(c, at(c, S1, 0x8));
                        break 'b_267314;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, r32(c, V1).wrapping_sub(0x4));
                        sw(c, r32(c, V1), c.g[V0][0]);
                        c.g[A0][0] = lw(c, at(c, S1, 0x8));
                        break 'b_267314;
                    }
                }
                c.cycles += 12;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = sx(r32(c, A0).wrapping_sub(0x4));
                c.g[V0][0] = sx(at(c, A0, 0x4));
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, S3)));
                sw(c, at(c, S1, 0x8), c.g[V0][0]);
                c.g[A2][0] = sx((r32(c, A2) as i32 >> 2) as u32);
                c.g[A2][0] = sx(r32(c, A2) << 2);
                c.g[S0][0] = lw(c, r32(c, S4));
                c.g[RA][0] = sx(0x0026733c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, A2)));
                if call_ptr(c, 0x0041610c, 0x0026733c).is_err() {
                    return;
                }
                let t = true;
                sw(c, r32(c, S3), c.g[S0][0]);
                if t {
                    break 'b_267444;
                } else {
                    break 'b_267344;
                }
            }
        }
        'b_2673ac: {
            'b_26738c: {
                c.cycles += 9;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                c.g[S0][0] = 0x1;
                c.g[A0][0] = 0x1;
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(r32(c, V0)));
                c.g[V0][0] = sx((r32(c, V0) as i32 >> 2) as u32);
                c.g[V1][0] = sx(r32(c, V0) << 1);
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = c.g[V1][0];
                }
                let t = c.g[S0][0] != c.g[A0][0];
                c.g[S0][0] = sx(r32(c, S0) << 2);
                if t {
                    break 'b_26738c;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0x4;
                    c.g[A1][0] = sx(at(c, A1, 0x4a8));
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00267384);
                    c.g[S5][0] = 0x4;
                    if call_ptr(c, 0x00317d70, 0x00267384).is_err() {
                        return;
                    }
                    let t = true;
                    c.g[S2][0] = c.g[V0][0];
                    if t {
                        break 'b_2673ac;
                    } else {
                        break 'b_26738c;
                    }
                }
            }
            c.cycles += 8;
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx(at(c, A1, 0x4a8));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0x20000000;
            c.g[RA][0] = sx(0x002673a4);
            c.g[A3][0] = 0u64;
            if call_ptr(c, 0x00317e30, 0x002673a4).is_err() {
                return;
            }
            c.g[S5][0] = c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            break 'b_2673ac;
        }
        'b_2673d8: {
            c.cycles += 9;
            c.g[A1][0] = lw(c, at(c, S1, 0x4));
            c.g[A0][0] = c.g[S2][0];
            c.g[V0][0] = sx(r32(c, S3).wrapping_sub(r32(c, A1)));
            c.g[A2][0] = sx((r32(c, V0) as i32 >> 2) as u32);
            c.g[S0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
            c.g[RA][0] = sx(0x002673c8);
            c.g[A2][0] = sx(r32(c, A2) << 2);
            if call_ptr(c, 0x0041610c, 0x002673c8).is_err() {
                return;
            }
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[S0][0];
            if t {
                break 'b_2673d8;
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw(c, r32(c, S4));
                sw(c, r32(c, S0), c.g[V0][0]);
                break 'b_2673d8;
            }
        }
        'b_267438: {
            'b_267434: {
                c.cycles += 14;
                c.g[A2][0] = lw(c, at(c, S1, 0x8));
                c.g[S0][0] = sx(at(c, V1, 0x4));
                c.g[A1][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, S3)));
                c.g[S3][0] = sx(r32(c, S0).wrapping_add(r32(c, A2)));
                c.g[A2][0] = sx((r32(c, A2) as i32 >> 2) as u32);
                c.g[RA][0] = sx(0x002673fc);
                c.g[A2][0] = sx(r32(c, A2) << 2);
                if call_ptr(c, 0x0041610c, 0x002673fc).is_err() {
                    return;
                }
                c.g[A0][0] = lw(c, at(c, S1, 0x4));
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
                let t = c.g[A0][0] == 0u64;
                c.g[V1][0] = sx((r32(c, V0) as i32 >> 2) as u32);
                if t {
                    break 'b_267434;
                } else {
                    'b_26742c: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_26742c;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00267424);
                            if call_ptr(c, 0x00317e50, 0x00267424).is_err() {
                                return;
                            }
                            let t = true;
                            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S5)));
                            if t {
                                break 'b_267438;
                            } else {
                                break 'b_26742c;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00267434);
                    if call_ptr(c, 0x00317e98, 0x00267434).is_err() {
                        return;
                    }
                    break 'b_267434;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S5)));
            break 'b_267438;
        }
        c.cycles += 3;
        sw(c, at(c, S1, 0x8), c.g[S3][0]);
        sw(c, at(c, S1, 0xc), c.g[V0][0]);
        sw(c, at(c, S1, 0x4), c.g[S2][0]);
        break 'b_267444;
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}

/// `void func_001C9038(void* self, void* text, const u16* str, float* size)`
/// (0x1c9038): the wide-character twin of `cFEPopupConfirm_GetTextBoxSize`:
/// split the UTF-16 string at `\r`/`\\`, measure each line, accumulate the size
/// and free the temporary buffer.
pub fn c_fe_popup_confirm_get_text_box_size_w(c: &mut Cpu) {
    let frame = Frame::open(c, 0xc0);
    'b_1c9180: {
        c.cycles += 34;
        frame.save(c, S0, 0xa0);
        frame.save(c, S5, 0x50);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x70);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S7, 0x30);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S2, 0x80);
        c.g[S7][0] = c.g[A1][0];
        frame.save(c, S4, 0x60);
        c.g[A0][0] = c.g[S0][0];
        frame.save(c, S6, 0x40);
        c.g[S2][0] = 0u64;
        sq(c, 30, at(c, SP, 0x20));
        c.g[S6][0] = 0u64;
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x001c9088);
        c.swc1(20, at(c, SP, 0xb0));
        if call_ptr(c, 0x003dbca0, 0x001c9088).is_err() {
            return;
        }
        c.g[V0][0] = sx(at(c, V0, 0x1));
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = sx(r32(c, V0) << 1);
        c.g[A1][0] = sx(at(c, A1, 0x60d8));
        c.g[A2][0] = 0x100;
        c.g[RA][0] = sx(0x001c90a4);
        c.g[A3][0] = 0u64;
        if call_ptr(c, 0x00317e30, 0x001c90a4).is_err() {
            return;
        }
        c.g[S4][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001c90b4);
        sh(c, r32(c, S4), 0u64);
        if call_ptr(c, 0x003dbca0, 0x001c90b4).is_err() {
            return;
        }
        c.g[FP][0] = c.g[V0][0];
        let t = (c.g[FP][0] as i64) <= 0;

        if t {
            break 'b_1c9180;
        } else {
            c.cycles += 6;
            c.g[S1][0] = c.g[S0][0];
            c.g[AT][0] = 0x40400000;
            c.mtc1(1, 20);
            c.g[S0][0] = c.g[S4][0];
            c.g[V1][0] = lhu(c, r32(c, S1));
            'l_1c90d8: loop {
                'b_1c9174: {
                    'b_1c9164: {
                        'b_1c913c: {
                            'b_1c9114: {
                                'b_1c9110: {
                                    c.cycles += 3;
                                    c.g[V0][0] = lb(c, r32(c, GP).wrapping_sub(0x14b8));
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A0][0] = lhu(c, at(c, S1, 0x2));
                                    if t {
                                        break 'b_1c9110;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[A0][0] != c.g[V1][0];
                                        c.g[V0][0] = 0xd;
                                        if t {
                                            break 'b_1c9114;
                                        } else {
                                            c.cycles += 9;
                                            sh(c, r32(c, S0), 0u64);
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] = c.g[S7][0];
                                            c.g[A2][0] = c.g[S4][0];
                                            c.g[A3][0] = c.g[SP][0];
                                            c.g[RA][0] = sx(0x001c9108);
                                            c.g[S1][0] = sx(at(c, S1, 0x4));
                                            if callee::C001c8568.call(c, 0x001c9108).is_err() {
                                                return;
                                            }
                                            let t = true;
                                            c.g[S6][0] = sx(at(c, S6, 0x2));
                                            if t {
                                                break 'b_1c913c;
                                            } else {
                                                break 'b_1c9110;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = 0xd;
                                break 'b_1c9114;
                            }
                            c.cycles += 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                sh(c, r32(c, S0), c.g[V1][0]);
                                break 'b_1c9164;
                            } else {
                                c.cycles += 8;
                                sh(c, r32(c, S0), 0u64);
                                c.g[A0][0] = c.g[S5][0];
                                c.g[A1][0] = c.g[S7][0];
                                c.g[A2][0] = c.g[S4][0];
                                c.g[A3][0] = c.g[SP][0];
                                c.g[RA][0] = sx(0x001c9138);
                                c.g[S1][0] = sx(at(c, S1, 0x2));
                                if callee::C001c8568.call(c, 0x001c9138).is_err() {
                                    return;
                                }
                                c.g[S6][0] = sx(at(c, S6, 0x1));
                                break 'b_1c913c;
                            }
                        }
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A1][0] = c.g[S3][0];
                        c.g[A2][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x001c9150);
                        c.g[S0][0] = c.g[S4][0];
                        if callee::C001c9b68.call(c, 0x001c9150).is_err() {
                            return;
                        }
                        c.g[S2][0] = 0u64;
                        c.lwc1(0, at(c, S3, 0x4));
                        c.add_s(0, 0, 20);
                        let t = true;
                        c.swc1(0, at(c, S3, 0x4));
                        if t {
                            break 'b_1c9174;
                        } else {
                            break 'b_1c9164;
                        }
                    }
                    c.cycles += 4;
                    c.g[S1][0] = sx(at(c, S1, 0x2));
                    c.g[S6][0] = sx(at(c, S6, 0x1));
                    c.g[S0][0] = sx(at(c, S0, 0x2));
                    c.g[S2][0] = sx(at(c, S2, 0x1));
                    break 'b_1c9174;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[FP][0] as i64)) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[V1][0] = lhu(c, r32(c, S1));
                    if event_due(c) {
                        deliver_event_here(c, 0x001c90d8);
                    }
                    continue 'l_1c90d8;
                } else {
                    break 'b_1c9180;
                }
            }
        }
    }
    'b_1c91c8: {
        c.cycles += 2;
        let t = c.g[S2][0] == 0u64;
        c.g[V0][0] = sx(r32(c, S2) << 1);
        if t {
            break 'b_1c91c8;
        } else {
            c.cycles += 16;
            c.g[A1][0] = c.g[S7][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
            c.g[A0][0] = c.g[S5][0];
            sh(c, r32(c, V0), 0u64);
            c.g[A2][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x001c91a4);
            c.g[A3][0] = c.g[SP][0];
            if callee::C001c8568.call(c, 0x001c91a4).is_err() {
                return;
            }
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001c91b4);
            c.g[A2][0] = c.g[SP][0];
            if callee::C001c9b68.call(c, 0x001c91b4).is_err() {
                return;
            }
            c.lwc1(0, at(c, S3, 0x4));
            c.g[AT][0] = 0x40400000;
            c.mtc1(1, 1);
            c.add_s(0, 0, 1);
            c.swc1(0, at(c, S3, 0x4));
            break 'b_1c91c8;
        }
    }
    'b_1c91dc: {
        c.cycles += 2;
        let t = c.g[S4][0] == 0u64;
        frame.restore(c, S0, 0xa0);
        if t {
            break 'b_1c91dc;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001c91d8);
            c.g[A0][0] = c.g[S4][0];
            if call_ptr(c, 0x00317e98, 0x001c91d8).is_err() {
                return;
            }
            frame.restore(c, S0, 0xa0);
            break 'b_1c91dc;
        }
    }
    c.cycles += 12;
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    lq(c, 30, at(c, SP, 0x20));
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    c.lwc1(20, at(c, SP, 0xb0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xc0));
    c.pc = t;
    return;
}

/// `void func_00187A38(void* self)` (0x187a38): one step of the memory-card
/// scan state machine: drive the card manager's scan (state in `D_004A14BC`),
/// turn the screen's flags off when the card call reports busy, and save or
/// restore the screen's saved flags.
pub fn c_fe_state_mem_card_scan_step(c: &mut Cpu) {
    let frame = Frame::open(c, 0x40);
    'b_187a78: {
        c.cycles += 15;
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
        frame.save(c, S1, 0x20);
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[RA][0] = sx(0x00187a5c);
        if call_ptr(c, 0x00227f80, 0x00187a5c).is_err() {
            return;
        }
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = lw(c, at(c, S1, 0x434));
        c.g[RA][0] = sx(0x00187a6c);
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, 0x0023c8d0, 0x00187a6c).is_err() {
            return;
        }
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x3;
        if t {
            break 'b_187a78;
        } else {
            c.cycles += 1;
            sw(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
            break 'b_187a78;
        }
    }
    'b_187c00: {
        'b_187bfc: {
            'b_187bf8: {
                'b_187be0: {
                    'b_187bac: {
                        'b_187b14: {
                            c.cycles += 4;
                            c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x1c34));
                            c.g[V0][0] = 0x1;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_187b14;
                            } else {
                                'b_187ab8: {
                                    'b_187aa0: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_187aa0;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0u64 {
                                                c.g[V1][0] = lw(c, r32(c, S0));
                                                break 'b_187ab8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                frame.restore(c, S0, 0x30);
                                                if t {
                                                    break 'b_187bfc;
                                                } else {
                                                    break 'b_187aa0;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_187bac;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        frame.restore(c, S0, 0x30);
                                        if t {
                                            break 'b_187be0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            frame.restore(c, S1, 0x20);
                                            if t {
                                                break 'b_187c00;
                                            } else {
                                                break 'b_187ab8;
                                            }
                                        }
                                    }
                                }
                                'b_187adc: {
                                    c.cycles += 6;
                                    c.g[A0][0] = lh(c, at(c, V1, 0x68));
                                    c.g[V0][0] = lw(c, at(c, V1, 0x6c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00187ac8);
                                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x00187ac8).is_err() {
                                        return;
                                    }
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = 0xfffffffffffffffd;
                                    if t {
                                        break 'b_187adc;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = lw(c, at(c, S2, 0x22c));
                                        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                        sw(c, at(c, S2, 0x22c), c.g[V0][0]);
                                        break 'b_187adc;
                                    }
                                }
                                c.cycles += 14;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x00187ae8);
                                c.g[A1][0] = 0x1;
                                if call_ptr(c, 0x0023c8f0, 0x00187ae8).is_err() {
                                    return;
                                }
                                c.g[V0][0] = lw(c, at(c, S1, 0x748));
                                c.g[A1][0] = 0x33;
                                c.g[A0][0] = lh(c, at(c, V0, 0x8));
                                c.g[V1][0] = lw(c, at(c, V0, 0xc));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00187b00);
                                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                                if call_ptr(c, t, 0x00187b00).is_err() {
                                    return;
                                }
                                c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x1c34));
                                c.g[V0][0] = sx(at(c, V0, 0x1));
                                sw(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
                                let t = true;
                                sw(c, at(c, S2, 0x1bc), 0u64);
                                if t {
                                    break 'b_187bf8;
                                } else {
                                    break 'b_187b14;
                                }
                            }
                        }
                        'b_187b3c: {
                            c.cycles += 7;
                            c.g[V1][0] = lw(c, r32(c, S0));
                            c.g[A0][0] = lh(c, at(c, V1, 0x68));
                            c.g[V0][0] = lw(c, at(c, V1, 0x6c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x00187b28);
                            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                            if call_ptr(c, t, 0x00187b28).is_err() {
                                return;
                            }
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0xfffffffffffffffe;
                            if t {
                                break 'b_187b3c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = lw(c, at(c, S2, 0x22c));
                                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                sw(c, at(c, S2, 0x22c), c.g[V0][0]);
                                break 'b_187b3c;
                            }
                        }
                        'b_187b74: {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, at(c, S1, 0x428));
                            if c.g[V0][0] != 0u64 {
                                c.g[A0][0] = c.g[S1][0];
                                break 'b_187b74;
                            } else {
                                'b_187b70: {
                                    c.cycles += 9;
                                    c.g[A1][0] = lw(c, at(c, S1, 0x434));
                                    c.g[V1][0] = lw(c, r32(c, A1));
                                    c.g[A0][0] = lh(c, at(c, V1, 0x88));
                                    c.g[V0][0] = lw(c, at(c, V1, 0x8c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00187b60);
                                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x00187b60).is_err() {
                                        return;
                                    }
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    sw(c, r32(c, GP).wrapping_sub(0x1c3c), c.g[V0][0]);
                                    if t {
                                        break 'b_187b70;
                                    } else {
                                        c.cycles += 1;
                                        sw(c, r32(c, GP).wrapping_sub(0x1c3c), 0u64);
                                        break 'b_187b70;
                                    }
                                }
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S1][0];
                                break 'b_187b74;
                            }
                        }
                        c.cycles += 14;
                        c.g[RA][0] = sx(0x00187b7c);
                        c.g[A1][0] = 0u64;
                        if call_ptr(c, 0x0023c8f0, 0x00187b7c).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw(c, at(c, S1, 0x748));
                        c.g[A1][0] = 0x33;
                        c.g[A0][0] = lh(c, at(c, V0, 0x8));
                        c.g[V1][0] = lw(c, at(c, V0, 0xc));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00187b94);
                        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x00187b94).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x1c34));
                        c.g[V1][0] = 0x1;
                        c.g[V0][0] = sx(at(c, V0, 0x1));
                        sw(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
                        let t = true;
                        sw(c, at(c, S2, 0x1bc), c.g[V1][0]);
                        if t {
                            break 'b_187bf8;
                        } else {
                            break 'b_187bac;
                        }
                    }
                    'b_187bd4: {
                        c.cycles += 7;
                        c.g[V1][0] = lw(c, r32(c, S0));
                        c.g[A0][0] = lh(c, at(c, V1, 0x68));
                        c.g[V0][0] = lw(c, at(c, V1, 0x6c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x00187bc0);
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x00187bc0).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0xfffffffffffffffb;
                        if t {
                            break 'b_187bd4;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, at(c, S2, 0x22c));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            sw(c, at(c, S2, 0x22c), c.g[V0][0]);
                            break 'b_187bd4;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x1c34));
                    c.g[V0][0] = sx(at(c, V0, 0x1));
                    sw(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
                    break 'b_187be0;
                }
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c, S1, 0x428));
                if c.g[V0][0] != 0u64 {
                    sw(c, at(c, S2, 0x234), 0u64);
                    break 'b_187bf8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, at(c, S2, 0x22c));
                    sw(c, at(c, S2, 0x238), c.g[V0][0]);
                    sw(c, at(c, S2, 0x234), 0u64);
                    break 'b_187bf8;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            break 'b_187bfc;
        }
        c.cycles += 1;
        frame.restore(c, S1, 0x20);
        break 'b_187c00;
    }
    c.cycles += 4;
    frame.restore(c, S2, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x40));
    c.pc = t;
    return;
}

/// `void func_002CC0C0(void* stack)` (0x2cc0c0): the menu system's per-frame
/// update, driven as a resumable block dispatcher (re-entered at the `pc` the
/// trampoline resumes): walk the eight menu entries of the active page, call
/// each one's update slot, and after the loop run the page's own update.
pub fn menu_sys_update(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x002cc0c0 => {
                c.cycles += 8;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
                sq(c, 16, at(c, SP, 0x20));
                sq(c, 17, at(c, SP, 0x10));
                c.g[S0][0] = c.g[A0][0];
                sd(c, r32(c, SP), c.g[RA][0]);
                c.g[V0][0] = lw(c, r32(c, S0));

                if c.g[V0][0] == 0u64 {
                    lq(c, 16, at(c, SP, 0x20));
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc0e0;
                continue;
            }
            0x002cc0e0 => {
                c.cycles += 4;
                c.g[A0][0] = lw(c, at(c, S0, 0x5c));
                c.g[V0][0] = lw(c, r32(c, A0));
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = 0u64;
                if t {
                    pc = 0x002cc0f8;
                    continue;
                }
                pc = 0x002cc0f0;
                continue;
            }
            0x002cc0f0 => {
                c.cycles += 2;
                c.g[V0][0] = lw(c, r32(c, V0));
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                pc = 0x002cc0f8;
                continue;
            }
            0x002cc0f8 => {
                c.cycles += 2;

                if c.g[V1][0] == 0u64 {
                    lq(c, 16, at(c, SP, 0x20));
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc100;
                continue;
            }
            0x002cc100 => {
                c.cycles += 11;
                c.g[RA][0] = sx(0x002cc108);
                c.g[A1][0] = 0x3d;
                if call_ptr(c, 0x00320bf0, 0x002cc108).is_err() {
                return;
}
                c.cvt_w_s(1, 0);
                c.mfc1(17, 1);
                c.g[A2][0] = sx(at(c, S0, 0x4c));
                c.g[V0][0] = 0xffffffffffffffff;
                c.g[A3][0] = sx(at(c, S0, 0x50));
                sw(c, at(c, S0, 0x4c), c.g[V0][0]);
                sw(c, at(c, S0, 0x50), c.g[V0][0]);
                let t = c.g[S1][0] != c.g[V0][0];
                sw(c, at(c, S0, 0x48), c.g[S1][0]);
                if t {
                    pc = 0x002cc1c4;
                    continue;
                }
                pc = 0x002cc12c;
                continue;
            }
            0x002cc12c => {
                c.cycles += 3;
                c.g[A0][0] = lw(c, at(c, GP, 0x300));

                if c.g[A0][0] == 0u64 {
                    c.g[V0][0] = lw(c, r32(c, S0));
                    pc = 0x002cc1c8;
                    continue;
                }
                pc = 0x002cc138;
                continue;
            }
            0x002cc138 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002cc140);
                c.g[A1][0] = 0u64;
                if call_ptr(c, 0x00266ba8, 0x002cc140).is_err() {
                return;
}

                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = lw(c, r32(c, S0));
                    pc = 0x002cc1c8;
                    continue;
                }
                pc = 0x002cc148;
                continue;
            }
            0x002cc148 => {
                c.cycles += 4;
                c.g[V0][0] = lw(c, at(c, S0, 0x50));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;

                if c.g[V0][0] != 0u64 {
                    sw(c, at(c, S0, 0x50), c.g[S1][0]);
                    pc = 0x002cc158;
                    continue;
                }
                pc = 0x002cc158;
                continue;
            }
            0x002cc158 => {
                c.cycles += 5;
                c.g[V0][0] = lw(c, at(c, S0, 0x4c));
                c.g[A0][0] = sx(r32(c, V0).wrapping_sub(0xd));
                c.g[V1][0] = (c.g[A0][0] < 0x22u64) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x480000;
                if t {
                    pc = 0x002cc1c4;
                    continue;
                }
                pc = 0x002cc16c;
                continue;
            }
            0x002cc16c => {
                c.cycles += 6;
                c.g[V1][0] = sx(r32(c, A0) << 2);
                c.g[V0][0] = sx(at(c, V0, 0x6630));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = lw(c, r32(c, V1));
                let t = c.g[A0][0] as u32;

                pc = t;
                continue;
            }
            0x002cc184 => {
                c.cycles += 2;
                let t = true;
                sw(c, at(c, S0, 0x48), 0u64);
                if t {
                    pc = 0x002cc1c4;
                    continue;
                }
                pc = 0x002cc18c;
                continue;
            }
            0x002cc18c => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x1;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc194;
                continue;
            }
            0x002cc194 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x2;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc19c;
                continue;
            }
            0x002cc19c => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x3;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1a4;
                continue;
            }
            0x002cc1a4 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x9;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1ac;
                continue;
            }
            0x002cc1ac => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0xa;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1b4;
                continue;
            }
            0x002cc1b4 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x6;
                if t {
                    pc = 0x002cc1c0;
                    continue;
                }
                pc = 0x002cc1bc;
                continue;
            }
            0x002cc1bc => {
                c.cycles += 1;
                c.g[V0][0] = 0x7;
                pc = 0x002cc1c0;
                continue;
            }
            0x002cc1c0 => {
                c.cycles += 1;
                sw(c, at(c, S0, 0x48), c.g[V0][0]);
                pc = 0x002cc1c4;
                continue;
            }
            0x002cc1c4 => {
                c.cycles += 1;
                c.g[V0][0] = lw(c, r32(c, S0));
                pc = 0x002cc1c8;
                continue;
            }
            0x002cc1c8 => {
                c.cycles += 13;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                c.g[A1][0] = lw(c, at(c, V0, 0x4));
                c.g[V1][0] = lw(c, at(c, A1, 0x12c));
                c.g[A0][0] = lh(c, at(c, V1, 0x10));
                c.g[V0][0] = lw(c, at(c, V1, 0x14));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x002cc1ec);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x002cc1ec).is_err() {
                return;
                }
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 0x1;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x2;
                if t {
                    pc = 0x002cc20c;
                    continue;
                }
                pc = 0x002cc1fc;
                continue;
            }
            0x002cc1fc => {
                c.cycles += 2;

                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = lw(c, r32(c, S0));
                    pc = 0x002cc22c;
                    continue;
                }
                pc = 0x002cc204;
                continue;
            }
            0x002cc204 => {
                c.cycles += 2;
                let t = true;
                lq(c, 16, at(c, SP, 0x20));
                if t {
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc20c;
                continue;
            }
            0x002cc20c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002cc214);
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, 0x002cc460, 0x002cc214).is_err() {
                return;
}
                let t = true;
                lq(c, 16, at(c, SP, 0x20));
                if t {
                    pc = 0x002cc238;
                    continue;
                }
                pc = 0x002cc21c;
                continue;
            }
            0x002cc21c => {
                c.cycles += 1;
                pc = 0x002cc220;
                continue;
            }
            0x002cc220 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002cc228);
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, 0x002cc460, 0x002cc228).is_err() {
                return;
}
                c.g[V0][0] = lw(c, r32(c, S0));
                pc = 0x002cc22c;
                continue;
            }
            0x002cc22c => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;

                if t {
                    if back_edge_unwinds(c, 0x002cc220) {
                        return;
                    }
                    pc = 0x002cc220;
                    continue;
                }
                pc = 0x002cc234;
                continue;
            }
            0x002cc234 => {
                c.cycles += 1;
                lq(c, 16, at(c, SP, 0x20));
                pc = 0x002cc238;
                continue;
            }
            0x002cc238 => {
                c.cycles += 4;
                lq(c, 17, at(c, SP, 0x10));
                c.g[RA][0] = ld(c, r32(c, SP));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(c, SP, 0x30));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x002cc0c0, 0x002cc248);
                return;
            }
        }
    }
}

/// `void BoostModifier_v02(void* mod)` (0x342880): the boost modifier's
/// update: step the boost meter for the current rider (gain while boosting,
/// decay otherwise), and drive the effect's fade and sound.
pub fn boost_modifier_v02(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x00342880 => {
                c.cycles += 12;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x60));
                sq(c, 17, at(c, SP, 0x30));
                sq(c, 16, at(c, SP, 0x40));
                c.g[S1][0] = c.g[A0][0];
                sq(c, 18, at(c, SP, 0x20));
                sq(c, 19, at(c, SP, 0x10));
                sd(c, r32(c, SP), c.g[RA][0]);
                c.swc1(20, at(c, SP, 0x50));
                sw(c, r32(c, S1), 0u64);
                c.g[V0][0] = lw(c, at(c, S1, 0x30));

                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = lw(c, at(c, S1, 0x2c));
                    pc = 0x003428bc;
                    continue;
                }
                pc = 0x003428b0;
                continue;
            }
            0x003428b0 => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                sw(c, r32(c, S1), c.g[V0][0]);
                c.g[V0][0] = lw(c, at(c, S1, 0x2c));
                pc = 0x003428bc;
                continue;
            }
            0x003428bc => {
                c.cycles += 2;
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                if t {
                    pc = 0x003428d4;
                    continue;
                }
                pc = 0x003428c4;
                continue;
            }
            0x003428c4 => {
                c.cycles += 2;
                let t = (c.g[V0][0] as i64) <= 0;
                sw(c, at(c, S1, 0x2c), c.g[V0][0]);
                if t {
                    pc = 0x003428d4;
                    continue;
                }
                pc = 0x003428cc;
                continue;
            }
            0x003428cc => {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                sw(c, r32(c, S1), c.g[V0][0]);
                pc = 0x003428d4;
                continue;
            }
            0x003428d4 => {
                c.cycles += 9;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                c.g[A0][0] = lh(c, at(c, V0, 0x48));
                c.g[V1][0] = lw(c, at(c, V0, 0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003428e8);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x003428e8).is_err() {
                return;
                }
                c.g[A0][0] = lw(c, at(c, S1, 0x30));
                c.g[V0][0] = 0x1;
                let t = c.g[A0][0] == c.g[V0][0];

                if t {
                    pc = 0x00342904;
                    continue;
                }
                pc = 0x003428f8;
                continue;
            }
            0x003428f8 => {
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c, S1, 0x2c));

                if (c.g[V0][0] as i64) > 0 {
                    c.g[V0][0] = lw(c, at(c, S1, 0x4));
                    pc = 0x003429ac;
                    continue;
                }
                pc = 0x00342904;
                continue;
            }
            0x00342904 => {
                c.cycles += 4;
                c.mtc1(0, 20);
                c.g[S2][0] = 0u64;
                let t = true;
                c.g[S3][0] = 0x1;
                if t {
                    pc = 0x00342994;
                    continue;
                }
                pc = 0x00342914;
                continue;
            }
            0x00342914 => {
                c.cycles += 1;
                pc = 0x00342918;
                continue;
            }
            0x00342918 => {
                c.cycles += 11;
                c.g[RA][0] = sx(0x00342920);
                c.g[A0][0] = c.g[S2][0];
                if call_ptr(c, 0x002d1b08, 0x00342920).is_err() {
                return;
}
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = lw(c, at(c, S1, 0x40));
                c.g[V1][0] = lw(c, r32(c, S0));
                c.g[A0][0] = lh(c, at(c, V1, 0x18));
                c.g[V0][0] = lw(c, at(c, V1, 0x1c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0034293c);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x0034293c).is_err() {
                return;
                }

                if c.g[V0][0] == 0u64 {
                    c.g[S2][0] = sx(at(c, S2, 0x1));
                    pc = 0x00342994;
                    continue;
                }
                pc = 0x00342944;
                continue;
            }
            0x00342944 => {
                c.cycles += 5;
                c.lwc1(0, at(c, S1, 0x24));
                c.c_lt_s(0, 20);
                let t = !c.fcc();
                sw(c, r32(c, S1), c.g[S3][0]);
                if t {
                    pc = 0x00342978;
                    continue;
                }
                pc = 0x00342958;
                continue;
            }
            0x00342958 => {
                c.cycles += 8;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = lh(c, at(c, V0, 0x40));
                c.g[V1][0] = lw(c, at(c, V0, 0x44));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00342970);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00342970).is_err() {
                return;
                }
                let t = true;
                c.g[S2][0] = sx(at(c, S2, 0x1));
                if t {
                    pc = 0x00342994;
                    continue;
                }
                pc = 0x00342978;
                continue;
            }
            0x00342978 => {
                c.cycles += 7;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = lh(c, at(c, V0, 0x38));
                c.g[V1][0] = lw(c, at(c, V0, 0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00342990);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00342990).is_err() {
                return;
                }
                c.g[S2][0] = sx(at(c, S2, 0x1));
                pc = 0x00342994;
                continue;
            }
            0x00342994 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x0034299c);
                if call_ptr(c, 0x002d1af0, 0x0034299c).is_err() {
                return;
}
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;

                if t {
                    if back_edge_unwinds(c, 0x00342918) {
                        return;
                    }
                    pc = 0x00342918;
                    continue;
                }
                pc = 0x003429a8;
                continue;
            }
            0x003429a8 => {
                c.cycles += 1;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                pc = 0x003429ac;
                continue;
            }
            0x003429ac => {
                c.cycles += 12;
                c.g[A0][0] = lh(c, at(c, V0, 0x50));
                c.g[V1][0] = lw(c, at(c, V0, 0x54));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003429bc);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x003429bc).is_err() {
                return;
                }
                lq(c, 16, at(c, SP, 0x40));
                lq(c, 17, at(c, SP, 0x30));
                lq(c, 18, at(c, SP, 0x20));
                lq(c, 19, at(c, SP, 0x10));
                c.g[RA][0] = ld(c, r32(c, SP));
                c.lwc1(20, at(c, SP, 0x50));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(c, SP, 0x60));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x00342880, 0x003429dc);
                return;
            }
        }
    }
}

/// `void func_001F85C0(void* self, void* a1)` (0x1f85c0): activate the pause
/// menu's freeride variant: for each of the eight entries at `+0xd8`, pick the
/// text/colour pair (the "selected" or "normal" one), format the entry's
/// label and hand it to the screen's text setter; then let the screen finish
/// the activation.
pub fn ov_screen_pause_freeride_on_activate(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x001f85c0 => {
                c.cycles += 21;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0xc0));
                sq(c, 17, at(c, SP, 0xa0));
                c.g[S1][0] = c.g[A0][0];
                sq(c, 16, at(c, SP, 0xb0));
                sq(c, 18, at(c, SP, 0x90));
                c.g[S0][0] = sx(at(c, S1, 0x9c));
                sq(c, 19, at(c, SP, 0x80));
                c.g[S2][0] = 0u64;
                sq(c, 20, at(c, SP, 0x70));
                c.g[S3][0] = sx(at(c, S1, 0xd8));
                sq(c, 21, at(c, SP, 0x60));
                c.g[S4][0] = c.g[A1][0];
                sq(c, 22, at(c, SP, 0x50));
                c.g[S5][0] = 0x4d0000;
                sq(c, 23, at(c, SP, 0x40));
                c.g[S6][0] = 0x4d0000;
                sq(c, 30, at(c, SP, 0x30));
                c.g[S7][0] = 0x4d0000;
                sd(c, at(c, SP, 0x20), c.g[RA][0]);
                let t = true;
                c.g[FP][0] = 0x4d0000;
                if t {
                    pc = 0x001f86a0;
                    continue;
                }
                pc = 0x001f8614;
                continue;
            }
            0x001f8614 => {
                c.cycles += 1;
                pc = 0x001f8618;
                continue;
            }
            0x001f8618 => {
                c.cycles += 2;
                let t = c.g[A2][0] == 0u64;
                c.g[V0][0] = c.lbu(r32(c, GP).wrapping_sub(0xc88));
                if t {
                    pc = 0x001f8698;
                    continue;
                }
                pc = 0x001f8620;
                continue;
            }
            0x001f8620 => {
                c.cycles += 2;
                let t = c.g[S2][0] != c.g[V0][0];
                c.g[V0][0] = lw(c, r32(c, S0));
                if t {
                    pc = 0x001f8650;
                    continue;
                }
                pc = 0x001f8628;
                continue;
            }
            0x001f8628 => {
                c.cycles += 5;
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                c.g[V1][0] = lw(c, r32(c, V0));
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = sx(r32(c, FP).wrapping_sub(0x7508));
                if t {
                    pc = 0x001f8644;
                    continue;
                }
                pc = 0x001f863c;
                continue;
            }
            0x001f863c => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = lw(c, at(c, A2, 0x8));
                if t {
                    pc = 0x001f8668;
                    continue;
                }
                pc = 0x001f8644;
                continue;
            }
            0x001f8644 => {
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c, A2, 0x8));
                let t = true;
                c.g[A1][0] = sx(r32(c, S7).wrapping_sub(0x7518));
                if t {
                    pc = 0x001f8668;
                    continue;
                }
                pc = 0x001f8650;
                continue;
            }
            0x001f8650 => {
                c.cycles += 5;
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                c.g[V1][0] = lw(c, r32(c, V0));
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = sx(r32(c, S6).wrapping_sub(0x74e8));
                if t {
                    pc = 0x001f8680;
                    continue;
                }
                pc = 0x001f8664;
                continue;
            }
            0x001f8664 => {
                c.cycles += 1;
                c.g[V0][0] = lw(c, at(c, A2, 0x8));
                pc = 0x001f8668;
                continue;
            }
            0x001f8668 => {
                c.cycles += 6;
                c.g[A0][0] = lh(c, at(c, V0, 0x58));
                c.g[V1][0] = lw(c, at(c, V0, 0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f8678);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x001f8678).is_err() {
                return;
                }
                let t = true;
                c.g[S0][0] = sx(at(c, S0, 0x4));
                if t {
                    pc = 0x001f869c;
                    continue;
                }
                pc = 0x001f8680;
                continue;
            }
            0x001f8680 => {
                c.cycles += 6;
                c.g[V0][0] = lw(c, at(c, A2, 0x8));
                c.g[A1][0] = sx(r32(c, S5).wrapping_sub(0x74f8));
                c.g[A0][0] = lh(c, at(c, V0, 0x58));
                c.g[V1][0] = lw(c, at(c, V0, 0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f8698);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x001f8698).is_err() {
                return;
                }
                pc = 0x001f8698;
                continue;
            }
            0x001f8698 => {
                c.cycles += 1;
                c.g[S0][0] = sx(at(c, S0, 0x4));
                pc = 0x001f869c;
                continue;
            }
            0x001f869c => {
                c.cycles += 1;
                c.g[S2][0] = sx(at(c, S2, 0x1));
                pc = 0x001f86a0;
                continue;
            }
            0x001f86a0 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 8i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x4a0000;
                if t {
                    pc = 0x001f86e4;
                    continue;
                }
                pc = 0x001f86ac;
                continue;
            }
            0x001f86ac => {
                c.cycles += 14;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f86bc);
                c.g[A1][0] = sx(at(c, V0, 0x2138));
                if call_ptr(c, 0x00417828, 0x001f86bc).is_err() {
                return;
}
                c.g[RA][0] = sx(0x001f86c4);
                c.g[A0][0] = c.g[SP][0];
                if call_ptr(c, 0x00317670, 0x001f86c4).is_err() {
                return;
}
                c.g[A0][0] = lw(c, at(c, S1, 0xc0));
                c.g[RA][0] = sx(0x001f86d0);
                c.g[A1][0] = c.g[V0][0];
                if call_ptr(c, 0x0039d860, 0x001f86d0).is_err() {
                return;
}
                c.g[A2][0] = c.g[V0][0];
                c.g[V0][0] = lw(c, at(c, S1, 0xbc));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;

                if t {
                    if back_edge_unwinds(c, 0x001f8618) {
                        return;
                    }
                    pc = 0x001f8618;
                    continue;
                }
                pc = 0x001f86e4;
                continue;
            }
            0x001f86e4 => {
                c.cycles += 15;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001f86f0);
                c.g[A1][0] = c.g[S4][0];
                if call_ptr(c, 0x0039e4c0, 0x001f86f0).is_err() {
                return;
}
                lq(c, 16, at(c, SP, 0xb0));
                lq(c, 17, at(c, SP, 0xa0));
                lq(c, 18, at(c, SP, 0x90));
                lq(c, 19, at(c, SP, 0x80));
                lq(c, 20, at(c, SP, 0x70));
                lq(c, 21, at(c, SP, 0x60));
                lq(c, 22, at(c, SP, 0x50));
                lq(c, 23, at(c, SP, 0x40));
                lq(c, 30, at(c, SP, 0x30));
                c.g[RA][0] = ld(c, at(c, SP, 0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(c, SP, 0xc0));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x001f85c0, 0x001f8720);
                return;
            }
        }
    }
}

/// `void Boost_Update(void* boost)` (0x3415d0): per-frame boost update: read
/// the rider, advance the boost timers and meter, and spawn/refresh the
/// boost effect.
pub fn boost_update(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003415d0 => {
                c.cycles += 16;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x70));
                sq(c, 16, at(c, SP, 0x50));
                sq(c, 17, at(c, SP, 0x40));
                c.g[S0][0] = 0x1;
                sq(c, 18, at(c, SP, 0x30));
                c.g[S1][0] = c.g[A0][0];
                sq(c, 19, at(c, SP, 0x20));
                c.g[S2][0] = c.g[S0][0];
                sq(c, 20, at(c, SP, 0x10));
                sd(c, r32(c, SP), c.g[RA][0]);
                c.swc1(20, at(c, SP, 0x60));
                c.g[V0][0] = lw(c, at(c, S1, 0x40));
                c.g[V1][0] = lw(c, at(c, S1, 0x3c));
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                let t = (c.g[V1][0] as i64) <= 0;
                if c.g[V0][0] == 0 {
                    c.g[S2][0] = 0u64;
                }
                if t {
                    pc = 0x00341620;
                    continue;
                }
                pc = 0x00341610;
                continue;
            }
            0x00341610 => {
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x1));
                c.g[V1][0] = ((c.g[V0][0] as i64) < 1i64) as u64;
                sw(c, at(c, S1, 0x3c), c.g[V0][0]);
                if c.g[V1][0] == 0 {
                    c.g[S2][0] = c.g[S0][0];
                }
                pc = 0x00341620;
                continue;
            }
            0x00341620 => {
                c.cycles += 8;
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[A0][0] = lh(c, at(c, V0, 0x188));
                c.g[V1][0] = lw(c, at(c, V0, 0x18c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00341634);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00341634).is_err() {
                return;
                }
                c.g[V0][0] = lw(c, at(c, S1, 0x40));
                let t = c.g[V0][0] == c.g[S0][0];

                if t {
                    pc = 0x0034164c;
                    continue;
                }
                pc = 0x00341640;
                continue;
            }
            0x00341640 => {
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c, S1, 0x3c));

                if (c.g[V0][0] as i64) > 0 {
                    c.g[V0][0] = lw(c, at(c, S1, 0xc));
                    pc = 0x00341718;
                    continue;
                }
                pc = 0x0034164c;
                continue;
            }
            0x0034164c => {
                c.cycles += 4;
                c.mtc1(0, 20);
                c.g[S3][0] = 0u64;
                let t = true;
                c.g[S4][0] = 0x2;
                if t {
                    pc = 0x00341700;
                    continue;
                }
                pc = 0x0034165c;
                continue;
            }
            0x0034165c => {
                c.cycles += 1;
                pc = 0x00341660;
                continue;
            }
            0x00341660 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x00341668);
                c.g[A0][0] = c.g[S3][0];
                if call_ptr(c, 0x002d1b08, 0x00341668).is_err() {
                return;
}
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = lw(c, at(c, S1, 0x40));
                let t = c.g[V0][0] != c.g[S4][0];
                c.g[V1][0] = lw(c, r32(c, S0));
                if t {
                    pc = 0x00341694;
                    continue;
                }
                pc = 0x00341678;
                continue;
            }
            0x00341678 => {
                c.cycles += 6;
                c.g[A0][0] = lh(c, at(c, V1, 0x48));
                c.g[V0][0] = lw(c, at(c, V1, 0x4c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00341688);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00341688).is_err() {
                return;
                }

                if c.g[V0][0] != 0u64 {
                    c.g[S3][0] = sx(at(c, S3, 0x1));
                    pc = 0x00341700;
                    continue;
                }
                pc = 0x00341690;
                continue;
            }
            0x00341690 => {
                c.cycles += 1;
                c.g[V1][0] = lw(c, r32(c, S0));
                pc = 0x00341694;
                continue;
            }
            0x00341694 => {
                c.cycles += 7;
                c.g[A1][0] = lw(c, at(c, S1, 0x18));
                c.g[A0][0] = lh(c, at(c, V1, 0x18));
                c.g[V0][0] = lw(c, at(c, V1, 0x1c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003416a8);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x003416a8).is_err() {
                return;
                }

                if c.g[V0][0] == 0u64 {
                    c.g[S3][0] = sx(at(c, S3, 0x1));
                    pc = 0x00341700;
                    continue;
                }
                pc = 0x003416b0;
                continue;
            }
            0x003416b0 => {
                c.cycles += 5;
                c.lwc1(0, at(c, S1, 0x34));
                c.c_lt_s(0, 20);
                let t = !c.fcc();
                c.g[S2][0] = 0x1;
                if t {
                    pc = 0x003416e4;
                    continue;
                }
                pc = 0x003416c4;
                continue;
            }
            0x003416c4 => {
                c.cycles += 8;
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = lh(c, at(c, V0, 0x180));
                c.g[V1][0] = lw(c, at(c, V0, 0x184));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003416dc);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x003416dc).is_err() {
                return;
                }
                let t = true;
                c.g[S3][0] = sx(at(c, S3, 0x1));
                if t {
                    pc = 0x00341700;
                    continue;
                }
                pc = 0x003416e4;
                continue;
            }
            0x003416e4 => {
                c.cycles += 7;
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = lh(c, at(c, V0, 0x178));
                c.g[V1][0] = lw(c, at(c, V0, 0x17c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003416fc);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x003416fc).is_err() {
                return;
                }
                c.g[S3][0] = sx(at(c, S3, 0x1));
                pc = 0x00341700;
                continue;
            }
            0x00341700 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x00341708);
                if call_ptr(c, 0x002d1af0, 0x00341708).is_err() {
                return;
}
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;

                if t {
                    if back_edge_unwinds(c, 0x00341660) {
                        return;
                    }
                    pc = 0x00341660;
                    continue;
                }
                pc = 0x00341714;
                continue;
            }
            0x00341714 => {
                c.cycles += 1;
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                pc = 0x00341718;
                continue;
            }
            0x00341718 => {
                c.cycles += 6;
                c.g[A0][0] = lh(c, at(c, V0, 0x190));
                c.g[V1][0] = lw(c, at(c, V0, 0x194));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00341728);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00341728).is_err() {
                return;
                }
                let t = c.g[S2][0] != 0u64;
                lq(c, 16, at(c, SP, 0x50));
                if t {
                    pc = 0x0034174c;
                    continue;
                }
                pc = 0x00341730;
                continue;
            }
            0x00341730 => {
                c.cycles += 7;
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = lh(c, at(c, V0, 0x110));
                c.g[V1][0] = lw(c, at(c, V0, 0x114));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00341748);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00341748).is_err() {
                return;
                }
                lq(c, 16, at(c, SP, 0x50));
                pc = 0x0034174c;
                continue;
            }
            0x0034174c => {
                c.cycles += 8;
                lq(c, 17, at(c, SP, 0x40));
                lq(c, 18, at(c, SP, 0x30));
                lq(c, 19, at(c, SP, 0x20));
                lq(c, 20, at(c, SP, 0x10));
                c.g[RA][0] = ld(c, r32(c, SP));
                c.lwc1(20, at(c, SP, 0x60));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(c, SP, 0x70));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003415d0, 0x0034176c);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{compare, fill, put_words, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::funcs::GuestFn;

    const BEL: u32 = DATA + 0x10_0000;
    const WORLD: u32 = DATA + 0x11_0000;
    const AUX: u32 = DATA + 0x12_0000;
    const SYS: u32 = DATA + 0x13_0000;
    const RACE: u32 = DATA + 0x14_0000;
    const RMAN: u32 = DATA + 0x15_0000;

    /// Stricter than `check`: caller-saved registers and LO/HI must
    /// match too (a fake callee hashes every GPR and the cycles).
    fn check_all(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} (soft): {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }

    /// The scaffolding most of these functions read: fake callees, the
    /// BELibrary singleton, the world chain (`world -> +0x84 -> race ->
    /// +0xc -> race manager`, rider count at `+0x7c`), the script
    /// engine at `gp-0x84c` and a dummy at `gp-0x854`.
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(GAME_GP - 0x1f04, BEL);
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(GAME_GP - 0x854, AUX);
        c.bus.write32(GAME_GP - 0x84c, SYS);
        c.bus.write32(WORLD + 0x84, RACE);
        c.bus.write32(RACE + 0xc, RMAN);
        c.bus.write32(RMAN + 0x7c, 1);
        r
    }

    const POPUP: u32 = DATA + 0x1_0000;

    #[test]
    fn c_keyboard_popup_move_down_matches() {
        check_all(
            ssx3_ee::s04::f_001cb9b0__generated,
            c_keyboard_popup_move_down,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1cb9_b0, 8);
                c.g[A0][0] = sx(POPUP);
                c.g[A1][0] = sx(seed % 2);
                c.bus.write32(POPUP + 0x68, (seed % 2) as u32);
                c.bus.write32(POPUP + 0x6c, ((seed / 2) % 2) as u32);
                let idx = [0u32, 14, 56, 70, 100][(seed as usize / 3) % 5];
                c.bus.write32(POPUP + 0x140, idx);
                c.bus
                    .write32(POPUP + 0x144, if seed % 4 == 0 { 2 } else { 1 });
                c.bus.write32(POPUP + 0x430, (seed % 3) as u32);
                c.bus.write32(POPUP + 0x434, r.next_u32() % 0x40);
                // The key table: no 0x2a runs (one seed walks the skip loop once).
                for i in 0..0x100 {
                    c.bus.write8(0x0044_1390 + i, 0);
                }
                if seed % 3 == 0 {
                    c.bus.write8(0x0044_1390 + idx, 0x2a);
                }
                // `IsKeyEnabled` reads `+0x148`/`+0x298` at `idx*4`: keep every
                // slot non-zero so the tail never re-enters the function.
                for i in 0..0x400 {
                    c.bus.write32(POPUP + 0x148 + i * 4, 1);
                }
            },
        );
    }

    #[test]
    fn be_restore_player_setup_matches() {
        check_all(
            ssx3_ee::s01::f_0014eca0__generated,
            be_restore_player_setup,
            40,
            &|c, seed| {
                let _ = base(c, seed, 0x14ec_a0, 8);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx((seed % 6) * 4); // &D_004A6CA8[idx]
                                                 // `D_00534B38` picks the mirrored-first-element branch.
                c.bus
                    .write32(0x0053_4b38, if seed % 3 == 0 { 0 } else { 1 });
                put_words(c, 0x0053_5b20, &[0, 0, 0, 0, 0, 0, 0, 0]);
            },
        );
    }

    const EQUIP: u32 = DATA + 0x1_0000;
    const EQUIP_TABLE: u32 = DATA + 0x1_1000;
    const EQUIP_NAME: u32 = DATA + 0x1_2000;

    #[test]
    fn c_fe_state_char_equip_get_preview_texture_matches() {
        check_all(
            ee::s03::f_0019a098__generated,
            c_fe_state_char_equip_get_preview_texture,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x19a0_98, 8);
                c.g[A0][0] = sx(EQUIP);
                c.g[A1][0] = sx(EQUIP_NAME);
                c.bus.write32(EQUIP + 0xa60, (seed % 2) as u32);
                c.bus.write32(EQUIP + 0xbc, seed % 4);
                c.bus.write32(EQUIP + 0xa64, (seed / 2) % 3);
                c.bus.write32(EQUIP + 0xa68, 0xffff_ffff);
                c.bus.write8(EQUIP_NAME, b'a' + (seed % 3) as u8);
                // The equip table the package lookup starts from.
                c.bus.write32(WORLD + 0x7c, EQUIP_TABLE);
                if seed % 3 == 0 {
                    c.bus.write32(EQUIP_TABLE + 0x1a70, DATA + 0x1_3000);
                }
            },
        );
    }

    const SESSION: u32 = DATA + 0x1_0000;
    const SESSION_IMG: u32 = DATA + 0x1_1000;

    #[test]
    fn ov_screen_session_update_matches() {
        check_all(
            ee::s05::f_00209370__generated,
            ov_screen_session_update,
            30,
            &|c, seed| {
                let mut r = base(c, seed, 0x2093_70, 6);
                c.g[A0][0] = sx(SESSION);
                c.bus.write32(SESSION + 0xdc, (seed % 2) as u32); // busy
                c.bus.write32(SESSION + 0xc8, ((seed / 2) % 2) as u32); // loaded
                c.bus.write32(SESSION + 0xcc, r.edge_f32()); // fade
                c.bus
                    .write32(SESSION + 0xd0, if seed % 3 == 0 { 0 } else { SESSION_IMG });
                c.bus.write32(SESSION + 0xc4, (seed % 4) as u32); // async file
                c.bus.write32(SESSION + 0x1cc, r.edge_f32()); // bob
                c.bus.write32(SESSION + 0x1d0, r.f32(1.0)); // bob velocity
                                                            // The eight icons at +0x104 stay null: the loop only advances
                                                            // the icon positions.
            },
        );
    }

    const HUD_CTX: u32 = DATA + 0x1_0000;
    const HUD_FONT: u32 = DATA + 0x1_1000;
    const HUD_DEFS: u32 = DATA + 0x1_2000;
    const HUD_TEXT: u32 = DATA + 0x1_3000;
    const HUD_RECT: u32 = DATA + 0x1_4000;
    const HUD_COLOR: u32 = DATA + 0x1_5000;

    #[test]
    fn hud_draw_text_elem_w_matches() {
        check_all(ee::s04::f_001f22e8__generated, hud_draw_text_elem_w, 20, &|c, seed| {
            let mut r = base(c, seed, 0x001f_22e8, 12);
            let sp = c.g[SP][0] as u32;
            c.g[A0][0] = sx(HUD_CTX);
            c.g[A1][0] = sx(HUD_FONT);
            c.g[A2][0] = sx(HUD_TEXT);
            c.g[A3][0] = sx(if seed % 2 == 0 { 0 } else { HUD_RECT });
            c.g[T0][0] = sx(seed % 4); // element index
            c.g[T1][0] = sx(if seed % 3 == 0 { 0 } else { HUD_COLOR });
            c.g[T2][0] = sx(if seed % 2 == 0 { 0 } else { HUD_DEFS });
            c.g[T3][0] = sx(seed % 3);
            // Stack arguments: the colour pointer, definition index, flags
            // and align (read back at `sp+0x140`.. after the frame opens).
            c.bus
                .write32(sp + 0x0, if seed % 4 == 1 { HUD_COLOR } else { 0 });
            c.bus.write32(sp + 0x8, seed % 4);
            c.bus
                .write32(sp + 0x10, [0u32, 1, 0x7f, 0xff][seed as usize % 4]);
            c.bus
                .write32(sp + 0x18, [0u32, 1, 0x7f, 0xff][(seed / 4) as usize % 4]);
            c.bus.write8(HUD_TEXT, b'H');
            c.bus.write8(HUD_TEXT + 1, b'i');
            c.bus.write8(HUD_TEXT + 2, 0);
            c.bus.write32(HUD_FONT + 0x30, r.f32(1.0));
            c.bus.write32(HUD_FONT + 0x34, r.f32(1.0));
            for i in 0..6 {
                c.bus
                    .write32(HUD_DEFS + 0x24 * (seed % 4) + 4 * i, r.next_u32() % 0x40);
            }
            c.bus.write32(HUD_DEFS + 0x22, r.next_u32() % 0x10);
        });
    }

    const RANGE: u32 = DATA + 0x1_0000;
    const NAMES: u32 = DATA + 0x2_0000;

    #[test]
    fn message_center_pick_foreshadow_message_matches() {
        check_all(
            ssx3_ee::s04::f_001e1ad0__generated,
            message_center_pick_foreshadow_message,
            30,
            &|c, seed| {
                let mut r = base(c, seed, 0x1e1a_d0, 10);
                c.g[A0][0] = sx(RANGE);
                c.g[A1][0] = sx(NAMES);
                c.bus.write32(RANGE + 0x4, (seed % 4) as u32);
                c.bus.write32(RANGE + 0x8, (seed % 3) as u32);
                // The candidate table: 0x18-byte entries whose character pointer
                // at +0xc is a string in the test area.
                for i in 0..6u32 {
                    c.bus
                        .write32(0x004c_6c08 + 0x18 * i + 0xc, NAMES + 0x40 * i);
                    c.bus
                        .write8(NAMES + 0x40 * i, b'a' + (r.next_u32() % 4) as u8);
                    c.bus.write8(NAMES + 0x40 * i + 1, 0);
                }
            },
        );
    }

    const LUNO_VALUE: u32 = DATA + 0x1_0000;
    const LUNO_STREAM: u32 = DATA + 0x1_1000;

    #[test]
    fn luno_vm_register_read_matches() {
        check_all(
            ee::s05::f_00225ca0__generated,
            luno_vm_register_read,
            30,
            &|c, seed| {
                let mut r = base(c, seed, 0x225c_a0, 8);
                c.g[A0][0] = sx(LUNO_VALUE);
                c.g[A1][0] = sx(LUNO_STREAM);
                c.g[A2][0] = sx(seed % 4);
                c.bus.write32(LUNO_VALUE, r.next_u32());
                c.bus.write32(LUNO_VALUE + 0x8, (seed % 6) as u32); // old type
                if seed % 2 == 0 {
                    let vt0 = vtable(c, &mut r, &[0x10, 0x14]);
                    c.bus.write32(LUNO_STREAM, vt0);
                }
            },
        );
    }

    const GATE: u32 = DATA + 0x1_0000;
    const GATE_RIDER: u32 = DATA + 0x1_1000;
    const GATE_RIDERBASE: u32 = DATA + 0x1_2000;

    #[test]
    fn rider_gate_sub_update_launch_matches() {
        check_all(
            ee::s00::f_0012c408__generated,
            rider_gate_sub_update_launch,
            30,
            &|c, seed| {
                let mut r = base(c, seed, 0x12c4_08, 10);
                c.g[A0][0] = sx(GATE);
                c.g[A1][0] = sx(DATA + 0x1_3000);
                c.bus.write32(GATE + 0x14, GATE_RIDER);
                c.bus.write32(GATE + 0x10, r.f32(0.05)); // charge around 1.0
                c.bus.write32(GATE + 0xc, r.f32(1.0));
                c.bus.write32(GATE_RIDER + 0x77c, GATE_RIDERBASE);
                c.bus.write32(GATE_RIDER + 0x86c, r.next_u32() % 0x100);
                c.bus.write32(GATE_RIDER + 0x784, 0);
                c.bus.write32(
                    GATE_RIDERBASE + 0xde0,
                    [0u32, 1, 3, 0xff][seed as usize % 4],
                );
                put_words(c, GATE_RIDER + 0x1b0, &[0]);
            },
        );
    }

    const PAIR: u32 = DATA + 0x1_0000;
    const PAIR_A: u32 = DATA + 0x1_1000;
    const PAIR_B: u32 = DATA + 0x1_2000;
    const PAIR_SCREEN: u32 = DATA + 0x1_3000;

    #[test]
    fn ui_pair_set_focus_matches() {
        check_all(ee::s13::f_0039bbd8__generated, ui_pair_set_focus, 30, &|c, seed| {
            let mut r = base(c, seed, 0x39bb_d8, 10);
            c.g[A0][0] = sx(PAIR);
            c.g[A1][0] = sx(seed % 2);
            c.bus.write32(PAIR + 0x14, r.next_u32());
            if seed % 2 == 0 {
                c.bus.write32(PAIR + 0x78, PAIR_A);
                let vt1 = vtable(c, &mut r, &[0x38, 0x3c]);
                c.bus.write32(PAIR_A + 8, vt1);
            }
            if seed % 3 != 0 {
                c.bus.write32(PAIR + 0x7c, PAIR_B);
                let vt2 = vtable(c, &mut r, &[0x38, 0x3c]);
                c.bus.write32(PAIR_B + 8, vt2);
                if seed % 2 == 0 {
                    c.bus.write32(PAIR_B + 0x5c, PAIR_SCREEN);
                    c.bus.write32(PAIR + 0x5c, PAIR_SCREEN);
                    c.bus.write32(PAIR_SCREEN + 0xd0, PAIR_A);
                    let vt3 = vtable(c, &mut r, &[0x98, 0x9c]);
                    c.bus.write32(PAIR_A + 8, vt3);
                }
            }
        });
    }

    const REWARD: u32 = DATA + 0x1_0000;

    #[test]
    fn be_reward_int_is_peak_goal_complete_matches() {
        check_all(
            ee::s01::f_00157bf0__generated,
            be_reward_int_is_peak_goal_complete,
            30,
            &|c, seed| {
                let mut r = base(c, seed, 0x157b_f0, 12);
                c.g[A0][0] = sx(REWARD);
                c.g[A1][0] = sx(seed % 3);
                c.g[A2][0] = sx((seed / 3) % 3);
                c.g[A3][0] = sx((seed / 9) % 4);
                c.g[T0][0] = sx(seed % 5); // the medal kind
                                           // The five-track table stays empty, so the loop starts on track 0
                                           // and runs until the medal comparison fails.
                c.bus.write32(REWARD + 0x54, r.next_u32() % 8);
            },
        );
    }

    const BANKS: u32 = DATA + 0x1_0000;
    const BANK_OWNER: u32 = DATA + 0x1_1000;

    #[test]
    fn midi_banks_base_dtor_matches() {
        check_all(ee::s07::f_0028fea0__generated, midi_banks_base_dtor, 30, &|c, seed| {
            let mut r = base(c, seed, 0x28fe_a0, 8);
            c.g[A0][0] = sx(BANKS);
            c.g[A1][0] = sx(seed % 4); // destructor flags
            c.bus
                .write32(BANKS, if seed % 2 == 0 { 0 } else { BANK_OWNER });
            c.bus
                .write32(BANKS + 0x4, if seed % 3 == 0 { 0 } else { DATA + 0x1_2000 });
            c.bus
                .write32(BANKS + 0x8, if seed % 5 == 0 { 0 } else { DATA + 0x1_3000 });
            c.bus.write32(BANK_OWNER + 0x1d8, DATA + 0x1_4000);
            c.bus.write32(BANK_OWNER + 0x1d4, DATA + 0x1_5000);
            let _ = &mut r;
        });
    }

    const LOBBY: u32 = DATA + 0x1_0000;
    const LOBBY_MSG: u32 = DATA + 0x1_1000;
    const LOBBY_NAMES: u32 = DATA + 0x1_2000;

        const RAIL_MAN: u32 = DATA + 0x1_0000;
    const RAIL_REF: u32 = DATA + 0x1_1000;
    const RAIL_PART: u32 = DATA + 0x1_2000;
    const RAIL_NODE: u32 = DATA + 0x1_3000;

    #[test]
    fn rail_man_enable_rail_matches() {
        check_all(ee::s11::f_00358998__generated, rail_man_enable_rail, 24, &|c, seed| {
            let mut r = base(c, seed, 0x3589_98, 8);
            c.g[A0][0] = sx(RAIL_MAN);
            c.g[A1][0] = sx(seed % 3);
            // `railman+0x18` is an array of 8-byte references whose object
            // (a multi-part) yields the collider list.
            c.bus.write32(RAIL_MAN + 0x18 + (seed % 3) * 8, RAIL_REF);
            c.bus.write32(RAIL_MAN + 0x1c + (seed % 3) * 8, 0);
            c.bus.write32(RAIL_PART, RAIL_NODE);
            c.bus
                .write32(RAIL_PART + 0x4, if seed % 2 == 0 { 0 } else { 2 });
            c.bus.write32(
                RAIL_NODE + 0x64,
                if seed % 2 == 0 { 0 } else { RAIL_NODE + 0x100 },
            );
            for i in 0..6 {
                c.bus.write32(RAIL_NODE + 0x6c + 4 * i, r.f32(10.0));
            }
        });
    }

    const DBG: u32 = DATA + 0x1_0000;
    const DBG_MENU: u32 = DATA + 0x1_1000;
    const DBG_VT: u32 = DATA + 0x1_2000;

    #[test]
    fn debug_menu_module_render_matches() {
        check_all(
            ee::s02::f_0017fbf0__generated,
            debug_menu_module_render,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x17fb_f0, 10);
                c.g[A0][0] = sx(DBG);
                c.bus.write32(DBG + 0x14, DBG_MENU);
                c.bus.write32(DBG + 0x1c, seed % 4);
                c.bus.write32(DBG + 0x24, r.next_u32() % 0x40);
                let vt4 = vtable(c, &mut r, &[0x18, 0x1c]);
                c.bus.write32(DBG_MENU, vt4);
                // The render-settings vtable at `gp-0x854 + 0x10d8` (slot 17/15/26/
                // 34/20) stays empty, so those calls go to the fakes.
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x88, 0x8c]);
                        c.bus.write32(AUX + 0x10d8, vt);
                }
            },
        );
    }

    const FLY: u32 = DATA + 0x1_0000;

    #[test]
    fn debug_menu_module_update_cursor_trail_matches() {
        check_all(
            ee::s02::f_0017f2d0__generated,
            debug_menu_module_update_cursor_trail,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x17f2_d0, 8);
                c.g[A0][0] = sx(FLY);
                c.bus.write32(FLY + 0x0, r.edge_f32()); // phase.x
                c.bus.write32(FLY + 0x4, r.edge_f32()); // phase.y
                c.bus.write32(FLY + 0x8, r.edge_f32()); // pos.x
                c.bus.write32(FLY + 0xc, r.edge_f32()); // pos.y
                c.bus.write32(FLY + 0x10, (seed % 64) as u32); // head
            },
        );
    }

    const HANDLER: u32 = DATA + 0x1_0000;
    const RIDERS: u32 = DATA + 0x1_1000;

    #[test]
    fn gm_handler8_on_rider_finished_matches() {
        check_all(
            ee::s06::f_0023ae00__generated,
            gm_handler8_on_rider_finished,
            20,
            &|c, seed| {
                let _ = base(c, seed, 0x23ae_00, 14);
                c.g[A0][0] = sx(HANDLER);
                c.g[A1][0] = sx(seed % 2);
                // The race and its riders come from the world singleton chain:
                // keep the rider list short by leaving the count at 1.
                c.bus.write32(RACE + 0xc, RMAN);
                c.bus.write32(RMAN + 0x7c, 1);
                for i in 0..2u32 {
                    c.bus.write32(RIDERS + 4 * i, DATA + 0x1_2000 + 0x1000 * i);
                    c.bus.write32(DATA + 0x1_3000 + 4 * i, 0);
                }
            },
        );
    }

    const DMAN: u32 = DATA + 0x1_0000;
    const BIG: u32 = DATA + 0x1_1000;

    #[test]
    fn data_instance_man_add_big_entries_matches() {
        check_all(
            ee::s07::f_00275b98__generated,
            data_instance_man_add_big_entries,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x275b_98, 10);
                c.g[A0][0] = sx(DMAN);
                c.g[A1][0] = sx(BIG);
                c.g[A2][0] = 0;
                // The archive walker: an empty archive stops immediately; one
                // seed gets a single non-archive entry.
                if seed % 2 == 0 {
                    put_words(c, BIG, &[0, 0, 0, 0]);
                } else {
                    put_words(c, BIG, &[1, 0, 0x20, 0]);
                }
                let vt5 = vtable(c, &mut r, &[0x10, 0x14]);
                c.bus.write32(DMAN + 0xc, vt5);
            },
        );
    }

    const MAP_IND: u32 = DATA + 0x1_0000;
    const MAP_SCREEN: u32 = DATA + 0x1_1000;
    const MAP_LIST: u32 = DATA + 0x1_2000;
    const MAP_ICONS: u32 = DATA + 0x1_3000;

    #[test]
    fn ov_screen_session_update_indicators_matches() {
        check_all(
            ee::s05::f_00209e78__generated,
            ov_screen_session_update_indicators,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x209e_78, 12);
                c.g[A0][0] = sx(MAP_IND);
                c.bus
                    .write32(MAP_IND + 0x0, if seed % 2 == 0 { 0 } else { MAP_ICONS });
                c.bus.write32(MAP_IND + 0x4, MAP_SCREEN);
                c.bus.write32(MAP_IND + 0x8, (seed % 4) as u32); // count
                c.bus.write32(MAP_IND + 0xc, MAP_LIST); // list object
                c.bus.write32(MAP_SCREEN + 0xa0, MAP_LIST);
                c.bus
                    .write32(MAP_LIST + 0x18, ((seed % 4) as u32).wrapping_add(1));
                for i in 0..4u32 {
                    c.bus.write32(MAP_IND + 0x10 + 4 * i, MAP_ICONS + 0x100 * i);
                    c.bus
                        .write32(MAP_IND + 0x20 + 4 * i, MAP_ICONS + 0x800 + 0x100 * i);
                }
            },
        );
    }

    const ENGINE: u32 = DATA + 0x1_0000;

    #[test]
    fn script_engine_dtor_matches() {
        check_all(ee::s07::f_002780b8__generated, script_engine_dtor, 20, &|c, seed| {
            let _ = base(c, seed, 0x2780_b8, 16);
            c.g[A0][0] = sx(ENGINE);
            c.g[A1][0] = sx(seed % 4); // destructor flags
            c.bus.write32(ENGINE + 0x52c, (seed % 2) as u32);
            // The per-frame state slots at +0x6d0..+0x910: no vtables, so the
            // slot destructor loop only decrements.
            c.bus.write32(ENGINE + 0x910, 0);
        });
    }

    const RIDER_TABLE: u32 = DATA + 0x1_0000;
    const TEX_PKG: u32 = DATA + 0x1_1000;

    #[test]
    fn fe_module_rider_preview_load_shape_textures_matches() {
        check_all(
            ee::s03::f_0019f908__generated,
            fe_module_rider_preview_load_shape_textures,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x19f9_08, 10);
                c.g[A0][0] = sx(RIDER_TABLE);
                c.g[A1][0] = sx(TEX_PKG);
                c.g[A2][0] = sx(seed % 2);
                c.bus.write32(TEX_PKG + 0x8, (seed % 3) as u32); // entry count
                c.bus.write32(RIDER_TABLE + 0x4, seed % 3); // stored entry count
                if seed % 2 == 0 {
                    c.bus.write32(TEX_PKG + 0xc, 0x10); // first entry offset
                    c.bus.write32(TEX_PKG + 0x10, 0x20);
                }
                let _ = &mut r;
            },
        );
    }

    const LOBBY2: u32 = DATA + 0x1_0000;
    const LOBBY2_MSG: u32 = DATA + 0x1_1000;

        const SCRIPT_SYS: u32 = DATA + 0x1_0000;

        const RACE_EVENT_RIDER: u32 = DATA + 0x1_0000;
    const RACE_EVENT_ARG: u32 = DATA + 0x1_1000;
    const RACE_EVENT_STATE: u32 = DATA + 0x1_2000;

    #[test]
    fn rider_handle_race_path_event_matches() {
        check_all(
            ee::s00::f_0010e5d8__generated,
            rider_handle_race_path_event,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x10e5_d8, 10);
                c.g[A0][0] = sx(RACE_EVENT_RIDER);
                c.g[A1][0] = sx(RACE_EVENT_ARG);
                c.g[A2][0] = sx(DATA + 0x1_3000);
                c.g[A3][0] = sx(seed);
                c.bus
                    .write32(RACE_EVENT_ARG, [1u32, 11, 3][seed as usize % 3]);
                c.bus.write32(RACE_EVENT_ARG + 4, seed % 4);
                c.bus.write32(RACE_EVENT_RIDER + 0x470, 0xbf80_0000); // -1.0
                                                                      // world+0xc0 is the race-path state object.
                c.bus.write32(WORLD + 0xc0, RACE_EVENT_STATE);
                c.bus.write32(RACE_EVENT_STATE + 0x98, (seed % 2) as u32);
            },
        );
    }

    const ROOM: u32 = DATA + 0x1_0000;
    const ROOM_WIDGET: u32 = DATA + 0x1_1000;
    const ROOM_FACTORY: u32 = DATA + 0x1_2000;

    #[test]
    fn c_fe_state_mountain_room_on_widget_action_matches() {
        check_all(
            ee::s04::f_001d2f68__generated,
            c_fe_state_mountain_room_on_widget_action,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x1d2f_68, 8);
                c.g[A0][0] = sx(ROOM);
                c.g[A1][0] = sx(if seed % 5 == 0 { 0 } else { ROOM_WIDGET });
                c.g[A2][0] = sx([5u32, 6, 1, 0, 2][seed as usize % 5]);
                c.bus.write32(ROOM_WIDGET + 0x18, r.next_u32() % 4);
                c.bus.write32(ROOM + 0x10, ROOM_FACTORY);
                let vt = vtable(c, &mut r, &[0x20, 0x24, 0x28, 0x2c]);
                    c.bus.write32(ROOM_FACTORY, vt);
                c.bus
                    .write32(ROOM + 0x54, if seed % 3 == 0 { 0 } else { DATA + 0x1_3000 });
                c.bus.write32(ROOM + 0xb4, (seed % 3) as u32);
                let vt = vtable(c, &mut r, &[0x48, 0x4c]);
                    c.bus.write32(DATA + 0x1_3000 + 8, vt);
            },
        );
    }

    const UPMGR: u32 = DATA + 0x1_0000;
    const UPMGR_TEX: u32 = DATA + 0x1_2000;

    #[test]
    fn tex_man_bind_textures_matches() {
        check_all(
            ee::s12::f_00368660__generated,
            tex_man_bind_textures,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x3686_60, 10);
                c.g[A0][0] = sx(UPMGR);
                c.g[A1][0] = sx((seed % 3).wrapping_sub(1)); // -1, 0, 1
                c.g[A2][0] = sx(((seed / 3) % 3).wrapping_sub(1));
                c.g[A3][0] = sx(0);
                // `texs[]`: entries point at small descriptors whose `addr` is
                // negative, so the bind path only allocates heap nodes.
                for i in 0..2u32 {
                    c.bus.write32(UPMGR + 4 * i, UPMGR_TEX + 0x40 * i);
                    c.bus.write32(UPMGR_TEX + 0x40 * i + 0x0, 0xffff_ffff); // addr
                    c.bus
                        .write32(UPMGR_TEX + 0x40 * i + 0x4, r.next_u32() % 0x100);
                    c.bus.write32(UPMGR_TEX + 0x40 * i + 0x8, (seed % 2) as u32); // clut
                    c.bus.write32(UPMGR_TEX + 0x40 * i + 0xc, (seed % 3) as u32);
                    // psm
                }
            },
        );
    }

    const MPART: u32 = DATA + 0x1_0000;
    const MPART_STREAM: u32 = DATA + 0x1_1000;

    #[test]
    fn multi_particle_ctor_2_matches() {
        check_all(
            ee::s11::f_00357e80__generated,
            multi_particle_ctor_2,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x357e_80, 10);
                c.g[A0][0] = sx(MPART);
                c.g[A1][0] = sx(MPART_STREAM);
                let vt6 = vtable(c, &mut r, &[0x8, 0xc]);
                c.bus.write32(MPART_STREAM, vt6);
                c.bus.write32(MPART_STREAM + 0x4, (seed % 3) as u32);
                let _ = &mut r;
            },
        );
    }

    const PRE_START: u32 = DATA + 0x1_0000;
    const PRE_MODULE: u32 = DATA + 0x1_1000;

    #[test]
    fn pre_start_screen_update_matches() {
        check_all(
            ee::s06::f_00232348__generated,
            pre_start_screen_update,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x2323_48, 8);
                c.g[A0][0] = sx(PRE_START);
                c.bus.write32(PRE_START + 0x0, (seed % 4) as u32);
                c.bus.write32(PRE_START + 0x4, r.edge_f32());
                c.bus.write32(WORLD + 0xa4, PRE_MODULE);
                c.bus.write32(PRE_MODULE + 0x4, r.edge_f32());
                c.bus.write32(WORLD + 0x4, (seed % 2) as u32);
            },
        );
    }

    const REWARD2: u32 = DATA + 0x1_0000;

    #[test]
    fn be_reward_int_on_mission_completed_matches() {
        check_all(
            ee::s01::f_00159b08__generated,
            be_reward_int_on_mission_completed,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x159b_08, 14);
                c.g[A0][0] = sx(REWARD2);
                c.g[A1][0] = sx(seed % 3);
                c.g[A2][0] = sx((seed / 3) % 4);
                // The stored mission threshold at `D_00440F58[k]`.
                c.bus.write32(
                    0x0044_0f58 + 4 * (((seed / 3) % 4) as u32),
                    if seed % 2 == 0 { 0 } else { seed % 5 },
                );
                c.bus.write32(REWARD2 + 0x54, seed % 4);
            },
        );
    }

    const REWARD3: u32 = DATA + 0x1_0000;

    #[test]
    fn be_reward_int_on_collectible_collected_matches() {
        check_all(
            ee::s01::f_001599a0__generated,
            be_reward_int_on_collectible_collected,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x1599_a0, 14);
                c.g[A0][0] = sx(REWARD3);
                c.g[A1][0] = sx(seed % 3);
                c.g[A2][0] = sx((seed / 3) % 4);
                c.bus.write32(
                    0x0044_0f48 + 4 * (((seed / 3) % 4) as u32),
                    if seed % 2 == 0 { 0 } else { seed % 5 },
                );
                c.bus.write32(REWARD3 + 0x54, seed % 4);
            },
        );
    }

    const INTRO_ENGINE: u32 = DATA + 0x1_0000;

    #[test]
    fn script_engine_queue_intro_scripts_matches() {
        check_all(
            ssx3_ee::s07::f_0027aaf8__generated,
            script_engine_queue_intro_scripts,
            50,
            &|c, seed| {
                let _ = base(c, seed, 0x27aa_f8, 6);
                c.g[A0][0] = sx(INTRO_ENGINE);
                c.g[A1][0] = sx(seed % 5);
                // State 0: `D_00534B30` set means the fixed intro table is used;
                // otherwise the table lookup comes from `D_00445488`.
                if seed % 2 == 0 {
                    c.bus.write32(0x0053_4b30, 1);
                    put_words(c, 0x0048_1e80, &[3, 4, 0xffff_ffff]);
                } else {
                    c.bus.write32(0x0053_4b30, 0);
                    put_words(c, 0x0044_5488, &[0]);
                }
            },
        );
    }

    const AUDIO: u32 = DATA + 0x1_0000;
    const AUDIO_OWNER: u32 = DATA + 0x1_1000;

    #[test]
    fn ssx_audio_system_stop_board_loops_matches() {
        check_all(
            ee::s07::f_00292b48__generated,
            ssx_audio_system_stop_board_loops,
            20,
            &|c, seed| {
                let _ = base(c, seed, 0x292b_48, 12);
                c.g[A0][0] = sx(AUDIO);
                c.bus.write32(AUDIO + 0x118, DATA + 0x1_2000);
                c.bus.write32(DATA + 0x1_2000, AUDIO_OWNER);
                c.bus.write32(AUDIO + 0x59e0, seed % 0x100);
                c.bus.write32(
                    AUDIO + 0x59e4,
                    if seed % 2 == 0 { 0 } else { (seed % 4) as u32 },
                );
            },
        );
    }

    const SAVE_REC: u32 = DATA + 0x1_0000;
    const SAVE_REC_MSG: u32 = DATA + 0x1_1000;
    const SAVE_MC: u32 = DATA + 0x1_2000;

    #[test]
    fn ov_screen_save_records_on_widget_action_matches() {
        check_all(
            ee::s05::f_001ff3b0__generated,
            ov_screen_save_records_on_widget_action,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x1ff3_b0, 12);
                c.g[A0][0] = sx(SAVE_REC);
                c.g[A1][0] = sx(if seed % 5 == 0 { 0 } else { SAVE_REC_MSG });
                c.g[A2][0] = sx([5u32, 6, 7, 0, 4][seed as usize % 5]);
                c.bus.write32(SAVE_REC_MSG + 0x18, r.next_u32() % 4);
                c.bus.write32(SAVE_REC + 0x1a8, ((seed / 5) % 3) as u32);
                c.bus.write32(SAVE_REC + 0x1c0, ((seed / 7) % 8) as u32);
                c.bus.write32(SAVE_REC + 0x214, ((seed / 11) % 4) as u32);
                c.bus.write32(SAVE_REC + 0x1c4, r.next_u32() % 4);
                // The movie manager: the object at +0x434 carries the vtable the
                // card-ready check goes through; +0x428 is its state.
                c.bus.write32(SAVE_MC + 0x428, ((seed / 3) % 3) as u32);
                c.bus.write32(SAVE_MC + 0x434, DATA + 0x1_3000);
                c.bus.write32(SAVE_MC + 0x748, 0);
                c.bus.write32(DATA + 0x1_3000 + 8, 0);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn mpeg2_sequence_extension_matches() {
        check_all(
            ee::s13::f_003b1ab0__generated,
            mpeg2_sequence_extension,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x3b1a_b0, 8);
                // The bit reader state: an 8-byte buffer with a few extension bits.
                c.bus.write32(0x0050_a088, DATA + 0x1_0000);
                put_words(c, DATA + 0x1_0000, &[r.next_u32(), r.next_u32()]);
                c.bus.write32(0x0050_9510, 1);
                c.bus.write32(DATA + 0x1_0000 + 0x420, 0);
                put_words(c, DATA + 0x1_0000 + 0x424, &[r.next_u32()]);
            },
        );
    }

    const EFFECT: u32 = DATA + 0x1_0000;
    const EFFECT_STREAM: u32 = DATA + 0x1_1000;

    #[test]
    fn particle_modifier_ctor_2_matches() {
        check_all(
            ee::s11::f_003459a8__generated,
            particle_modifier_ctor_2,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x3459_a8, 10);
                c.g[A0][0] = sx(EFFECT);
                c.g[A1][0] = sx(EFFECT_STREAM);
                let vt = vtable(c, &mut r, &[0x10, 0x14]);
                    c.bus.write32(EFFECT_STREAM, vt);
                c.bus.write32(EFFECT + 0x1e4, (seed % 2) as u32);
            },
        );
    }

    const REPLAY: u32 = DATA + 0x1_0000;
    const REPLAY_FRAME: u32 = DATA + 0x1_1000;
    const REPLAY_VIEW: u32 = DATA + 0x1_2000;

        const TARGET_CAM: u32 = DATA + 0x1_0000;
    const TARGET_RIG: u32 = DATA + 0x1_1000;
    const TARGET_ORBIT: u32 = DATA + 0x1_2000;

    #[test]
    fn target_script_camera_algo_update_matches() {
        check_all(
            ee::s02::f_00169db0__generated,
            target_script_camera_algo_update,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x169d_b0, 10);
                c.g[A0][0] = sx(TARGET_CAM);
                c.bus.write32(TARGET_CAM + 0x14, TARGET_RIG);
                c.bus.write32(TARGET_RIG + 0x14, TARGET_ORBIT);
                c.bus.write32(TARGET_ORBIT + 0x8, DATA + 0x1_3000);
                c.bus.write32(TARGET_RIG + 0x30, r.next_u32() % 0x40);
                for i in 0..0x10 {
                    c.bus.write32(TARGET_RIG + 0x30 + 4 * i, r.f32(1.0));
                }
                for i in 0..0x20 {
                    c.bus.write32(TARGET_CAM + 0x40 + 4 * i, r.f32(10.0));
                }
            },
        );
    }

    const MAN_CAM: u32 = DATA + 0x1_0000;
    const MAN_RIG: u32 = DATA + 0x1_1000;
    const MAN_ORBIT: u32 = DATA + 0x1_2000;

    #[test]
    fn manual_script_camera_algo_update_matches() {
        check_all(
            ee::s02::f_00169570__generated,
            manual_script_camera_algo_update,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x1695_70, 10);
                c.g[A0][0] = sx(MAN_CAM);
                c.bus.write32(MAN_CAM + 0x14, MAN_RIG);
                c.bus.write32(MAN_RIG + 0x14, MAN_ORBIT);
                c.bus.write32(MAN_ORBIT + 0x8, DATA + 0x1_3000);
                for i in 0..0x10 {
                    c.bus.write32(MAN_RIG + 0x30 + 4 * i, r.f32(1.0));
                }
                for i in 0..0x20 {
                    c.bus.write32(MAN_CAM + 0x40 + 4 * i, r.f32(10.0));
                }
            },
        );
    }

    const DAY_SELF: u32 = DATA + 0x1_0000;
    const DAY_BOX: u32 = DATA + 0x1_1000;
    const DAY_SEL: u32 = DATA + 0x1_2000;

    #[test]
    fn c_fe_state_create_step2_fill_day_list_matches() {
        check_all(
            ee::s03::f_001bb840__generated,
            c_fe_state_create_step2_fill_day_list,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x1bb8_40, 10);
                c.g[A0][0] = sx(DAY_SELF);
                c.g[A1][0] = sx(DAY_BOX);
                c.g[A2][0] = sx(seed % 12); // month
                c.g[A3][0] = sx(2003 + (seed % 3)); // year
                c.bus.write32(DAY_SELF + 0x6d0, DAY_SEL);
                c.bus.write32(DAY_SEL + 0x2c, seed % 40);
                c.bus.write32(DAY_BOX + 0x14, r.next_u32());
                let _ = &mut r;
            },
        );
    }

    const OCT_NODE: u32 = DATA + 0x1_0000;
    const OCT_CTX: u32 = DATA + 0x1_1000;
    const OCT_ENTRY: u32 = DATA + 0x1_2000;

    #[test]
    fn camera_trigger_man_collect_octree_node_triggers_matches() {
        check_all(
            ee::s02::f_00176890__generated,
            camera_trigger_man_collect_octree_node_triggers,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x1768_90, 10);
                c.g[A0][0] = sx(OCT_NODE);
                c.g[A1][0] = sx(OCT_CTX);
                c.bus.write32(OCT_CTX + 0x0, DATA + 0x1_3000);
                c.bus.write32(OCT_CTX + 0x4, DATA + 0x1_4000); // world
                if seed % 2 == 0 {
                    c.bus.write32(OCT_NODE + 0x28, OCT_ENTRY);
                    c.bus.write32(OCT_ENTRY + 0x8, 4);
                    c.bus.write32(OCT_ENTRY + 0xc, DATA + 0x1_5000);
                    let vt = vtable(c, &mut r, &[0x28, 0x2c]);
                        c.bus.write32(DATA + 0x1_4000 + 0x6c0, vt);
                    let vt = vtable(c, &mut r, &[0x38, 0x3c]);
                        c.bus.write32(DATA + 0x1_5000 + 8, vt);
                }
                // One child so the recursion is exercised.
                if seed % 3 == 0 {
                    c.bus.write32(OCT_NODE, OCT_NODE + 0x200);
                }
            },
        );
    }

    const BOLT_SELF: u32 = DATA + 0x1_0000;

    #[test]
    fn be_bolt_int_unlock_bolt_matches() {
        check_all(
            ee::s01::f_0014b560__generated,
            be_bolt_int_unlock_bolt,
            20,
            &|c, seed| {
                let _ = base(c, seed, 0x14b5_60, 12);
                c.g[A0][0] = sx(BOLT_SELF);
                c.g[A1][0] = sx(seed % 3); // rider slot
                c.g[A2][0] = sx(seed % 4); // character
                c.g[A3][0] = sx((seed / 4) % 6); // bolt key
                c.g[T0][0] = sx(seed % 2); // free
            },
        );
    }

    const DEFER: u32 = DATA + 0x1_0000;

    #[test]
    fn defer_man_emit_default_state_matches() {
        check_all(
            ee::s12::f_00364360__generated,
            defer_man_emit_default_state,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x3643_60, 10);
                let sp = c.g[SP][0] as u32;
                let pp = DATA + 0x1_1000;
                c.g[A0][0] = sx(DEFER);
                c.g[A1][0] = sx(pp);
                c.bus.write32(pp, DATA + 0x1_2000);
                // The GS state words the packet is built from.
                put_words(
                    c,
                    0x0050_1420,
                    &[r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                put_words(c, DATA + 0x1_2000, &[0, 0, 0, 0, 0, 0, 0, 0]);
                put_words(c, 0x0050_1400, &[0, 0, 0, 0]);
                let _ = sp;
            },
        );
    }

    const POPUP2: u32 = DATA + 0x1_0000;
    const POPUP2_TEXT: u32 = DATA + 0x1_1000;
    const POPUP2_STR: u32 = DATA + 0x1_2000;
    const POPUP2_SIZE: u32 = DATA + 0x1_3000;

    #[test]
    fn c_fe_popup_confirm_get_text_box_size_matches() {
        check_all(
            ee::s03::f_001c9210__generated,
            c_fe_popup_confirm_get_text_box_size,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x1c92_10, 10);
                c.g[A0][0] = sx(POPUP2);
                c.g[A1][0] = sx(POPUP2_TEXT);
                c.g[A2][0] = sx(POPUP2_STR);
                c.g[A3][0] = sx(POPUP2_SIZE);
                let s: &[u8] = match seed % 4 {
                    0 => b"ab\r\ncd",
                    1 => b"hello",
                    2 => b"a\\\\b",
                    _ => b"\r\r",
                };
                for (i, b) in s.iter().enumerate() {
                    c.bus.write8(POPUP2_STR + i as u32, *b);
                }
                c.bus.write8(POPUP2_STR + s.len() as u32, 0);
                c.bus.write32(POPUP2_SIZE, r.f32(1.0));
                c.bus.write32(POPUP2_SIZE + 4, r.f32(1.0));
            },
        );
    }

    const PTR_VEC: u32 = DATA + 0x1_0000;
    const PTR_BUF: u32 = DATA + 0x1_1000;

        const CHAL_VEC: u32 = DATA + 0x1_0000;
    const CHAL_BUF: u32 = DATA + 0x1_1000;

        const POPUP3: u32 = DATA + 0x1_0000;
    const POPUP3_TEXT: u32 = DATA + 0x1_1000;
    const POPUP3_STR: u32 = DATA + 0x1_2000;
    const POPUP3_SIZE: u32 = DATA + 0x1_3000;

    #[test]
    fn c_fe_popup_confirm_get_text_box_size_w_matches() {
        check_all(
            ee::s03::f_001c9038__generated,
            c_fe_popup_confirm_get_text_box_size_w,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x1c90_38, 10);
                c.g[A0][0] = sx(POPUP3);
                c.g[A1][0] = sx(POPUP3_TEXT);
                c.g[A2][0] = sx(POPUP3_STR);
                c.g[A3][0] = sx(POPUP3_SIZE);
                let s: &[u16] = match seed % 4 {
                    0 => &[b'a' as u16, b'b' as u16, 13, b'c' as u16, b'd' as u16],
                    1 => &[b'h' as u16, b'i' as u16],
                    2 => &[b'a' as u16, 92, 92, b'b' as u16],
                    _ => &[13, 13],
                };
                for (i, w) in s.iter().enumerate() {
                    c.bus.write16(POPUP3_STR + 2 * i as u32, *w);
                }
                c.bus.write16(POPUP3_STR + 2 * s.len() as u32, 0);
                c.bus.write32(POPUP3_SIZE, r.f32(1.0));
                c.bus.write32(POPUP3_SIZE + 4, r.f32(1.0));
            },
        );
    }

    const MEMCARD: u32 = DATA + 0x1_0000;
    const MEMCARD_MP: u32 = DATA + 0x1_1000;
    const MEMCARD_CARD: u32 = DATA + 0x1_2000;

    #[test]
    fn c_fe_state_mem_card_scan_step_matches() {
        check_all(
            ee::s02::f_00187a38__generated,
            c_fe_state_mem_card_scan_step,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x187a_38, 10);
                c.g[A0][0] = sx(MEMCARD);
                c.bus.write32(0x004a_28a8, WORLD);
                c.bus.write32(WORLD + 0xa4, MEMCARD_MP);
                c.bus.write32(MEMCARD_MP + 0x434, MEMCARD_CARD);
                c.bus.write32(MEMCARD_MP + 0x428, (seed % 2) as u32);
                c.bus.write32(0x004a_14bc, seed % 4); // scan step
                c.bus.write32(MEMCARD + 0x1bc, r.next_u32());
                c.bus.write32(MEMCARD + 0x234, r.next_u32() % 4);
                let _ = &mut r;
            },
        );
    }

    const MENU: u32 = DATA + 0x1_0000;
    const MENU_ENTRY: u32 = DATA + 0x1_1000;

    #[test]
    fn menu_sys_update_matches() {
        check_all(ee::s09::f_002cc0c0__generated, menu_sys_update, 20, &|c, seed| {
            let mut r = base(c, seed, 0x2cc0_c0, 16);
            c.pc = 0x002c_c0c0; // the dispatcher is entered at its entry block
            c.g[A0][0] = sx(MENU);
            c.g[A1][0] = sx(seed);
            c.bus.write32(MENU + 0xbc, (seed % 4) as u32);
            c.bus
                .write32(MENU + 0xc0, if seed % 3 == 0 { 0 } else { MENU_ENTRY });
            c.bus.write8(GAME_GP - 0xc88, (seed % 2) as u8);
            for i in 0..8u32 {
                c.bus.write32(
                    MENU + 0xd8 + 4 * i,
                    if seed % 2 == 0 {
                        0
                    } else {
                        MENU_ENTRY + 0x100 * i
                    },
                );
            }
            c.bus.write32(MENU_ENTRY + 0x8, 0);
            c.bus.write32(MENU + 0x9c, DATA + 0x1_2000);
            let _ = &mut r;
        });
    }

    const BOOST_MOD: u32 = DATA + 0x1_0000;

    #[test]
    fn boost_modifier_v02_matches() {
        check_all(ee::s11::f_00342880__generated, boost_modifier_v02, 20, &|c, seed| {
            let mut r = base(c, seed, 0x3428_80, 14);
            c.pc = 0x0034_2880;
            c.g[A0][0] = sx(BOOST_MOD);
            c.bus.write32(BOOST_MOD + 0x0, (seed % 4) as u32);
            c.bus.write32(BOOST_MOD + 0x4, r.edge_f32());
            c.bus.write32(BOOST_MOD + 0x8, r.f32(1.0));
            c.bus.write32(BOOST_MOD + 0x10, (seed % 3) as u32);
            c.bus.write32(BOOST_MOD + 0x1c, r.f32(2.0));
            c.bus.write32(RMAN + 0x7c, 1);
        });
    }

    const PAUSE_MENU: u32 = DATA + 0x1_0000;
    const PAUSE_ENTRY: u32 = DATA + 0x1_1000;
    const PAUSE_TEXT: u32 = DATA + 0x1_2000;

    #[test]
    fn ov_screen_pause_freeride_on_activate_matches() {
        check_all(
            ee::s04::f_001f85c0__generated,
            ov_screen_pause_freeride_on_activate,
            20,
            &|c, seed| {
                let mut r = base(c, seed, 0x1f85_c0, 16);
                c.pc = 0x001f_85c0;
                c.g[A0][0] = sx(PAUSE_MENU);
                c.g[A1][0] = sx(DATA + 0x1_3000);
                c.bus.write32(PAUSE_MENU + 0xbc, (seed % 4) as u32);
                c.bus.write8(GAME_GP - 0xc88, (seed % 2) as u8);
                for i in 0..8u32 {
                    let e = if seed % 2 == 0 {
                        0
                    } else {
                        PAUSE_ENTRY + 0x40 * i
                    };
                    c.bus.write32(PAUSE_MENU + 0xd8 + 4 * i, e);
                    if e != 0 {
                        c.bus.write32(e + 0x8, 0);
                    }
                }
                c.bus.write32(PAUSE_MENU + 0xc0, PAUSE_TEXT);
                let _ = &mut r;
            },
        );
    }

    const BOOST: u32 = DATA + 0x1_0000;

    #[test]
    fn boost_update_matches() {
        check_all(ee::s11::f_003415d0__generated, boost_update, 20, &|c, seed| {
            let mut r = base(c, seed, 0x3415_d0, 14);
            c.pc = 0x0034_15d0;
            c.g[A0][0] = sx(BOOST);
            c.bus.write32(BOOST + 0x0, (seed % 4) as u32);
            c.bus.write32(BOOST + 0x4, r.edge_f32());
            c.bus.write32(BOOST + 0x8, r.f32(1.0));
            c.bus.write32(BOOST + 0xc, r.edge_f32());
            c.bus.write32(BOOST + 0x10, (seed % 3) as u32);
            c.bus.write32(BOOST + 0x1c, r.f32(2.0));
            c.bus.write32(RMAN + 0x7c, 1);
        });
    }
}
