//! `src/ai/ai.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::rider::rate_step_60;
pub(crate) use crate::game::camera::camera::gp;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_5;
pub(crate) use crate::game::dirtysock::tags2::gp_8 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::a;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_2;
pub(crate) use crate::game::fe::feasyncfile::gp_12 as gp_4;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_5;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_6;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw;
pub(crate) use crate::game::fe::feasyncfile::sw_6 as sw_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_7;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::gp as gp_3;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_3b48, ai_select_race_state, "AI_SelectRaceState"),
    (
        0x0011_75b8,
        rider_metrix_reset_stats,
        "RiderMetrix_ResetStats",
    ),
    (0x0012_87b8, ai_dtor, "AI_dtor"),
    (
        0x0012_8958,
        ai_setup_computer_riders_2,
        "AI_SetupComputerRiders",
    ),
    (0x0012_8998, ai_setup_actor_riders, "AI_SetupActorRiders"),
    (
        0x0012_8a10,
        ai_reset_riders_and_order,
        "AI_ResetRidersAndOrder",
    ),
    (0x0012_9160, ai_update_rider_poses, "AI_UpdateRiderPoses"),
    (0x0012_96f8, ai_load_next_rider, "AI_LoadNextRider"),
    (0x0012_9768, ai_restart, "AI_Restart"),
    (0x0012_97c8, ai_reset_riders, "AI_ResetRiders"),
    (
        0x0012_a490,
        ai_path_data_init_if_early_section,
        "AI_PathDataInitIfEarlySection",
    ),
    (
        0x0012_ab20,
        ai_reset_computer_render_slots,
        "AI_ResetComputerRenderSlots",
    ),
    (
        0x0012_abd0,
        ai_computer_riders_have_render_slots,
        "AI_ComputerRidersHaveRenderSlots",
    ),
    (
        0x0012_ac48,
        ai_reset_actor_render_slots,
        "AI_ResetActorRenderSlots",
    ),
    (
        0x0012_acf8,
        ai_actor_riders_have_render_slots,
        "AI_ActorRidersHaveRenderSlots",
    ),
    (
        0x0012_ae38,
        ai_destroy_computer_riders,
        "AI_DestroyComputerRiders",
    ),
    (
        0x0012_aed0,
        ai_destroy_actor_riders,
        "AI_DestroyActorRiders",
    ),
    (
        0x0012_af68,
        ai_destroy_mission_riders,
        "AI_DestroyMissionRiders",
    ),
    (0x0012_b000, ai_remove_actor_riders, "AI_RemoveActorRiders"),
    (
        0x0012_b030,
        ai_remove_computer_riders,
        "AI_RemoveComputerRiders",
    ),
    (
        0x0012_b060,
        ai_remove_mission_riders,
        "AI_RemoveMissionRiders",
    ),
    (0x0012_b090, ai_destroy_all_riders, "AI_DestroyAllRiders"),
    (0x0012_b180, ai_reset_rider_stats, "AI_ResetRiderStats"),
    (0x0012_b200, ai_free_anim_sources, "AI_FreeAnimDataSources"),
    (0x0012_b340, ai_force_finish_riders, "AI_ForceFinishRiders"),
    (0x0012_b420, ai_get_sync_checksum, "AI_GetSyncChecksum"),
    (
        0x0012_b698,
        ai_apply_computer_rider_settings,
        "AI_ApplyComputerRiderSettings",
    ),
    (
        0x0012_b788,
        ai_refresh_rider_overlaps,
        "AI_RefreshRiderOverlaps",
    ),
    (0x0012_bac0, ai_set_sync_words, "AI_SetSyncWords"),
    (0x0012_baf0, ai_get_sync_words, "AI_GetSyncWords"),
    (
        0x0012_c028,
        ridergate_sub_is_start_time_reached,
        "RiderGateSub_IsStartTimeReached",
    ),
    (
        0x0012_c078,
        rider_gate_sub_can_start,
        "RiderGateSub_CanStart",
    ),
    (
        0x0012_c0c0,
        rider_gate_sub_update_wait_push,
        "RiderGateSub_UpdateWaitPush",
    ),
    (
        0x0012_c130,
        rider_gate_sub_update_push,
        "RiderGateSub_UpdatePush",
    ),
    (
        0x0012_c408,
        rider_gate_sub_update_launch,
        "RiderGateSub_UpdateLaunch",
    ),
    (
        0x0012_c590,
        rider_gate_sub_load_state,
        "RiderGateSub_LoadState",
    ),
    (
        0x0012_c5c8,
        rider_finish_line_sub_enter,
        "RiderFinishLineSub_Enter",
    ),
    (
        0x0012_c9f8,
        rider_finish_line_sub_load_state,
        "RiderFinishLineSub_LoadState",
    ),
    (
        0x0012_d9d8,
        rider_wipeout_sub_update_phase4,
        "RiderWipeoutSub_UpdatePhase4",
    ),
    (
        0x0012_da88,
        rider_wipeout_sub_select_tumble_anim,
        "RiderWipeoutSub_SelectTumbleAnim",
    ),
    (
        0x0012_dd98,
        rider_wipeout_sub_switch_to_anim_set_a,
        "RiderWipeoutSub_SwitchToAnimSetA",
    ),
    (
        0x0012_df48,
        rider_wipeout_sub_play_end_anim_a,
        "RiderWipeoutSub_PlayEndAnimA",
    ),
    (
        0x0012_e740,
        rider_wipeout_sub_load_state,
        "RiderWipeoutSub_LoadState",
    ),
    (0x0012_e9b0, rider_prewind_sub_exit, "RiderPrewindSub_Exit"),
    (0x0012_f230, rider_reset_sub_enter, "RiderResetSub_Enter"),
    (
        0x0012_f588,
        rider_reset_sub_is_local_player,
        "RiderResetSub_IsLocalPlayer",
    ),
    (
        0x0012_f5e8,
        rider_reset_sub_load_state,
        "RiderResetSub_LoadState",
    ),
    (
        0x0012_fc28,
        rider_natural_air_sub_load_state,
        "RiderNaturalAirSub_LoadState",
    ),
    (
        0x0012_fc60,
        rider_board_press_sub_enter,
        "RiderBoardPressSub_Enter",
    ),
    (
        0x0012_fec8,
        rider_board_press_sub_flip_stance,
        "RiderBoardPressSub_FlipStance",
    ),
    (
        0x0013_04d0,
        rider_board_press_sub_update_phase2,
        "RiderBoardPressSub_UpdatePhase2",
    ),
    (
        0x0013_1200,
        rider_board_press_sub_check_release,
        "RiderBoardPressSub_CheckRelease",
    ),
    (
        0x0013_1348,
        rider_board_press_sub_cancel,
        "RiderBoardPressSub_Cancel",
    ),
    (
        0x0013_13a8,
        rider_board_press_sub_settle_slew,
        "RiderBoardPressSub_SettleSlew",
    ),
    (
        0x0013_15d0,
        rider_board_press_sub_load_state,
        "RiderBoardPressSub_LoadState",
    ),
    (0x0013_1608, rider_cruise_sub_enter, "RiderCruiseSub_Enter"),
    (
        0x0013_2048,
        rider_railslide_sub_exit,
        "RiderRailslideSub_Exit",
    ),
    (
        0x0013_2620,
        rider_railslide_sub_set_rail,
        "RiderRailslideSub_SetRail",
    ),
    (
        0x0013_2878,
        rider_railslide_sub_load_state,
        "RiderRailslideSub_LoadState",
    ),
    (
        0x0013_2f98,
        rider_handplant_sub_exit,
        "RiderHandplantSub_Exit",
    ),
    (
        0x0013_30f0,
        rider_handplant_sub_load_state,
        "RiderHandplantSub_LoadState",
    ),
    (
        0x0013_5eb0,
        rider_spin_sub_is_trick_anim12,
        "RiderSpinSub_IsTrickAnim12",
    ),
    (
        0x0013_5ef0,
        rider_spin_sub_is_trick_anim13,
        "RiderSpinSub_IsTrickAnim13",
    ),
    (
        0x0013_5f30,
        rider_spin_sub_is_trick_anim14,
        "RiderSpinSub_IsTrickAnim14",
    ),
    (
        0x0013_5f70,
        rider_spin_sub_get_hud_dir_angle,
        "RiderSpinSub_GetHUDDirAngle",
    ),
    (
        0x0013_6100,
        rider_spin_sub_get_spin_hud_blend,
        "RiderSpinSub_GetSpinHUDBlend",
    ),
    (
        0x0013_6230,
        rider_spin_sub_load_state,
        "RiderSpinSub_LoadState",
    ),
    (
        0x0013_6920,
        rider_uber_grind_sub_load_state,
        "RiderUberGrindSub_LoadState",
    ),
];

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
}

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::ANIM_GET_LAYER_SEQUENCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_ADD_TIMER_2 as BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_BAGT_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_CHAR_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_NEW_PLAYER_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_RESET_PROFILE_CHARACTERS_2 as BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBE_GET_INTERFACE as CBE_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_MEM_MAN_ALLOC as C_MEM_MAN_ALLOC_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::FORMAT_TAGGED;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152948;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152BA8;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C2540;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C26D0;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_0039E2A0;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_ALT_CHAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_CHAR_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_FINISH_SPEECH_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_RIDER_VOICE_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_TRANSITION_MAN;
    pub(crate) use crate::game::dirtysock::tags2::callee::GM_PICK_OPPONENTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_LOCAL_PLAYER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_PA_SPEECH_ON;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_RACE_TYPE_4OR5;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBYMAN_SEND_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_PRODUCT_CODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_SAVE_TYPE_TAG;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_SET_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_PLAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_WRAP;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMSET_3 as MEMSET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_IS_FLAG_SET_2 as MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_5;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_REMOVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::RIDER_123168;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUPMAN_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_QUEUE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_6 as SPRINTF_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_HASH_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_INTERN;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_GET_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::TEXT_OBJ;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_DEACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2 as APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_DECODE_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_AUDIO_DONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_FRAME_DUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_ALL_TIMERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_BACKUP_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2 as BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5 as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_5 as EA_MEM_FREE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_LIST_GET_AT_2 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2 as MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2 as MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2 as SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_7 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_4 as SSX_AUDIO_SYSTEM_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_GET_ACTIVE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_START_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3 as UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3 as UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2 as UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2 as USTR_CPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2 as USTR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C0017f500;
    pub(crate) use crate::game::fe::fepopup::callee::C0019e2b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0019f878;
    pub(crate) use crate::game::fe::fepopup::callee::C001bb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001c84a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001c8568;
    pub(crate) use crate::game::fe::fepopup::callee::C001c9b68;
    pub(crate) use crate::game::fe::fepopup::callee::C001cb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbc48;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbce8;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbfc0;
    pub(crate) use crate::game::fe::fepopup::callee::C001ce560;
    pub(crate) use crate::game::fe::fepopup::callee::C001d32e0;
    pub(crate) use crate::game::fe::fepopup::callee::C001e1a30;
    pub(crate) use crate::game::fe::fepopup::callee::C001e3388;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91f8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e9290;
    pub(crate) use crate::game::fe::fepopup::callee::C001f10f8;
    pub(crate) use crate::game::fe::fepopup::callee::C0020e900;
    pub(crate) use crate::game::fe::fepopup::callee::C00224c78;
    pub(crate) use crate::game::fe::fepopup::callee::C00224da0;
    pub(crate) use crate::game::fe::fepopup::callee::C00224df0;
    pub(crate) use crate::game::fe::fepopup::callee::C002259a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00227f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00231f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00238bf8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026d4d8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026db88;
    pub(crate) use crate::game::fe::fepopup::callee::C0026e448;
    pub(crate) use crate::game::fe::fepopup::callee::C0026f4a0;
    pub(crate) use crate::game::fe::fepopup::callee::C002702f8;
    pub(crate) use crate::game::fe::fepopup::callee::C002703f0;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER as MPEG2_FLUSH_BUFFER_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS as MPEG2_SHOW_BITS_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2 as UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_3;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE as CPS2_MEM_CARD_SET_STATE_2;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR as MULTI_PARTICLE_DTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID as SCRIPT_ENGINE_GET_SEQ_PROCESS_ID_2;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN as STRLEN_3;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE as CBE_GET_BE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES as FLEX_BRIDGE_ALLOC_SPRING_NODES_2;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS as RIDER_METRIX_RESET_STATS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY_3;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR_3;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST as RIDER_ADD_BOOST_2;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS as RIDER_METRIX_AWARD_POINTS_2;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AUDIO_SYSTEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUILD_BUDDY_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2 as CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FIND_FREE_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_RACE_MAN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GOTO_NEXT_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::HANDLE_NET_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LEVEL_TO_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LOAD_BANK_ASYNC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_BASE_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ONLINE_CHARACTER_SELECT_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SET_CONTROLLER_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REQUEST_IMAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_SAY_20B9;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2 as SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SEND_PACKET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SHOW_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2 as SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::START_RIDER_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;

    /// `IntcHandler_SaveRegs` (0x362120).
    pub const INTC_SAVE_REGS: Callee = Callee {
        addr: 0x0036_2120,
        f: ee::s12::f_00362120,
    };

    /// `IntcHandler_RestoreRegs` (0x362230).
    pub const INTC_RESTORE_REGS: Callee = Callee {
        addr: 0x0036_2230,
        f: ee::s12::f_00362230,
    };

    /// `Irradiance_FindIndex` (0x38ac50).
    pub const IRRADIANCE_FIND_INDEX: Callee = Callee {
        addr: 0x0038_ac50,
        f: ee::s12::f_0038ac50,
    };

    /// `Irradiance_GetEntry` (0x38abf8).
    pub const IRRADIANCE_GET_ENTRY: Callee = Callee {
        addr: 0x0038_abf8,
        f: ee::s12::f_0038abf8,
    };

    /// `cWorldCache_Free` (0x3a7e98).
    pub const WORLD_CACHE_FREE: Callee = Callee {
        addr: 0x003a_7e98,
        f: ee::s13::f_003a7e98,
    };

    /// `MPEG2_Flush_Buffer` (0x3b1290).
    pub const MPEG2_FLUSH_BUFFER: Callee = Callee {
        addr: 0x003b_1290,
        f: ee::s13::f_003b1290,
    };

    /// `MPEG2_Show_Bits` (0x3b1258).
    pub const MPEG2_SHOW_BITS: Callee = Callee {
        addr: 0x003b_1258,
        f: ee::s13::f_003b1258,
    };

    /// `MPEG2_Flush_Buffer32` (0x3b1340).
    pub const MPEG2_FLUSH_BUFFER32: Callee = Callee {
        addr: 0x003b_1340,
        f: ee::s13::f_003b1340,
    };

    /// `AI_ResetRiders` (0x1297c8).
    pub const AI_RESET_RIDERS: Callee = Callee {
        addr: 0x0012_97c8,
        f: ee::s00::f_001297c8,
    };

    /// `AI_SetRiderOrderMode` (0x128a48).
    pub const AI_SET_RIDER_ORDER_MODE: Callee = Callee {
        addr: 0x0012_8a48,
        f: ee::s00::f_00128a48,
    };

    /// `Rider_FindNearbyLights` (0x122088).
    pub const RIDER_FIND_NEARBY_LIGHTS: Callee = Callee {
        addr: 0x0012_2088,
        f: ee::s00::f_00122088,
    };

    /// `BE_CopyProfileCharacters` (0x156858).
    pub const BE_COPY_PROFILE_CHARACTERS: Callee = Callee {
        addr: 0x0015_6858,
        f: ee::s01::f_00156858,
    };

    pub const EA_FILE_FREE_FILE_HANDLE: Callee = Callee {
        addr: 0x003d_e8c0,
        f: ee::s14::EAFile_FreeFileHandle,
    };

    pub const EA_FILE_FREE_REQUEST: Callee = Callee {
        addr: 0x003d_e7b0,
        f: ee::s14::EAFile_FreeRequest,
    };

    pub const SPCH_FIND_BANK_TYPE: Callee = Callee {
        addr: 0x003d_8330,
        f: ee::s14::SPCH_FindBankType,
    };

    pub const RIDER_WIPEOUT_UPDATE_ANIM_SPEED: Callee = Callee {
        addr: 0x0012_e528,
        f: ee::s00::RiderWipeoutSub_UpdateAnimSpeed,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE00: Callee = Callee {
        addr: 0x0010_4ca0,
        f: ee::s00::RiderAnim_SeqEndType00,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE01: Callee = Callee {
        addr: 0x0010_4a40,
        f: ee::s00::RiderAnim_SeqEndType01,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE02: Callee = Callee {
        addr: 0x0010_4a60,
        f: ee::s00::RiderAnim_SeqEndType02,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE03: Callee = Callee {
        addr: 0x0010_4b78,
        f: ee::s00::RiderAnim_SeqEndType03,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE04: Callee = Callee {
        addr: 0x0010_4c18,
        f: ee::s00::RiderAnim_SeqEndType04,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE05: Callee = Callee {
        addr: 0x0010_4b48,
        f: ee::s00::RiderAnim_SeqEndType05,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE06: Callee = Callee {
        addr: 0x0010_4c38,
        f: ee::s00::RiderAnim_SeqEndType06,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE07: Callee = Callee {
        addr: 0x0010_4b98,
        f: ee::s00::RiderAnim_SeqEndType07,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE08: Callee = Callee {
        addr: 0x0010_4bb8,
        f: ee::s00::RiderAnim_SeqEndType08,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE09: Callee = Callee {
        addr: 0x0010_4bd8,
        f: ee::s00::RiderAnim_SeqEndType09,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE0A: Callee = Callee {
        addr: 0x0010_4c80,
        f: ee::s00::RiderAnim_SeqEndType0A,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
}

/// `bool RiderResetSub_IsLocalPlayer(RiderResetSub *self)` (0x12f588): false
/// unless the object (+0x8 -> +0x87c) is present, else whether the player id at
/// +0x870 is non-negative.
pub fn rider_reset_sub_is_local_player(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 0));
    let handle = obj.off(0x8).lw(c);
    c.g[V1][0] = handle;
    let present = Addr(handle as u32).off(0x87c).lw(c);
    let mut result = 0u64;
    if present != 0 {
        c.cycles += 3;
        let id = Addr(handle as u32).off(0x870).lw(c);
        result = ((id as i64) >= 0) as u64;
    }
    c.cycles += 2;
    c.g[A0][0] = result;
    c.g[V0][0] = result;
    abi::ret(c);
}

/// `void RiderBoardPressSub_UpdatePhase2(RiderBoardPressSub *self)`
/// (0x1304d0): set the phase field at +0 to 1.
pub fn rider_board_press_sub_update_phase2(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    this.sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rider sub-states.

/// `void RiderBoardPressSub_Enter(RiderBoardPressSub *self)` (0x12fc60):
/// reset the sub-state and its rider's board-press words.
pub fn rider_board_press_sub_enter(c: &mut Cpu) {
    c.cycles += 8;
    let this = Addr(abi::arg(c, 0));
    let rider = this.off(0x14).lw(c);
    c.g[V0][0] = rider;
    this.sw(c, 0);
    Addr((rider as u32).wrapping_add(0x360)).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `void RiderCruiseSub_Enter(RiderCruiseSub *self)` (0x131608): clear the
/// two board-press words of the rider referenced at +0.
pub fn rider_cruise_sub_enter(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    let handle = this.lw(c);
    c.g[V0][0] = handle;
    Addr((handle as u32).wrapping_add(0x35c)).sw(c, 0);
    let handle = this.lw(c);
    c.g[V1][0] = handle;
    Addr((handle as u32).wrapping_add(0x360)).sw(c, 0);
    abi::ret(c);
}

/// `void RiderRailslideSub_Exit(RiderRailslideSub *self)` (0x132048): clear
/// the railslide word and restore its pitch constant.
pub fn rider_railslide_sub_exit(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    let rider = this.off(0x8).lw(c);
    c.g[V0][0] = rider;
    let pitch = Addr((c.g[GP][0] as u32).wrapping_sub(0x7594)).read_fpu(c);
    c.f[0] = pitch.0;
    Addr((rider as u32).wrapping_add(0x240)).sw(c, 0);
    Addr((rider as u32).wrapping_add(0x23c)).write_fpu(c, pitch);
    abi::ret(c);
}

/// `void RiderHandplantSub_Exit(RiderHandplantSub *self)` (0x132f98): clear
/// the handplant words and restore its pitch constant.
pub fn rider_handplant_sub_exit(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    let rider = this.off(0x14).lw(c);
    c.g[V0][0] = rider;
    let pitch = Addr((c.g[GP][0] as u32).wrapping_sub(0x754c)).read_fpu(c);
    c.f[0] = pitch.0;
    Addr((rider as u32).wrapping_add(0x32c)).sw(c, 0);
    let rider = this.off(0x14).lw(c);
    c.g[V1][0] = rider;
    Addr((rider as u32).wrapping_add(0x248)).write_fpu(c, pitch);
    Addr((rider as u32).wrapping_add(0x24c)).sw(c, 0);
    abi::ret(c);
}

pub(crate) const AI_SET_RACE_STATE: Callee = Callee {
    addr: 0x0011_3b10,
    f: ssx3_ee::s00::f_00113b10,
};

/// `void AI_SelectRaceState(AI *ai, int state)` (0x113b48): look up the
/// entry point for `state` in the dispatch table at `0x450000 + 0x6ca0`
/// (`(state-1) * 4`) and store the state's handler address (+0xa0), mode
/// (+0) and label pointer (+0x98); out-of-range states zero those fields.
/// The `__entries` twin handles the table's mid-function targets.
pub fn ai_select_race_state(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = sx(abi::arg(c, 1).wrapping_sub(1));
    c.g[V0][0] = (c.g[V1][0] < 7) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[A2][0] = c.g[A0][0];
    if out_of_range {
        c.cycles += 4;
        Addr(c.g[A2][0] as u32).sw(c, 0);
        Addr(c.g[A2][0] as u32).off(0x98).sw(c, 0);
        Addr(c.g[A2][0] as u32).off(0xa0).sw(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[V0][0] = 0x0045_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca0));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let target = c.g[A0][0] as u32;
    // (mode, handler address, label offset).
    let (mode, handler, label): (u64, u32, u32) = match target {
        0x0011_3b74 => (1, 0x0045_0000 + 0x6c58, 0xac),
        0x0011_3b94 => (2, 0x0045_0000 + 0x6c68, 0xb0),
        0x0011_3ba8 => (3, 0x004a_0000 + 0xe90, 0xb4),
        0x0011_3bbc => (4, 0x0045_0000 + 0x6c78, 0xbc),
        0x0011_3bd0 => (5, 0x004a_0000 + 0xe98, 0xc0),
        0x0011_3be4 => (6, 0x004a_0000 + 0xea0, 0xc4),
        0x0011_3bf8 => (7, 0x0045_0000 + 0x6c88, 0xc8),
        _ => {
            c.jump_out(target, 0x0011_3b48, 0x0011_3c1c);
            return;
        }
    };
    c.cycles += if mode == 1 { 4 } else { 5 };
    c.g[A0][0] = mode;
    c.g[V0][0] = sx(handler);
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(label));
    c.cycles += 4;
    Addr(c.g[A2][0] as u32).off(0x98).sw(c, c.g[V1][0]);
    Addr(c.g[A2][0] as u32).sw(c, c.g[A0][0]);
    Addr(c.g[A2][0] as u32).off(0xa0).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// AI manager: rider setup/teardown wrappers. The destroy/reset callees take
// their short path when the AI object's rider counts are zero.

pub(crate) const AI_DESTROY_MISSION_RIDERS: Callee = Callee {
    addr: 0x0012_af68,
    f: ssx3_ee::s00::f_0012af68,
};

pub(crate) const AI_DESTROY_ACTOR_RIDERS: Callee = Callee {
    addr: 0x0012_aed0,
    f: ssx3_ee::s00::f_0012aed0,
};

pub(crate) const AI_DESTROY_COMPUTER_RIDERS: Callee = Callee {
    addr: 0x0012_ae38,
    f: ssx3_ee::s00::f_0012ae38,
};

pub(crate) const AI_DESTROY_ALL_RIDERS: Callee = Callee {
    addr: 0x0012_b090,
    f: ssx3_ee::s00::f_0012b090,
};

pub(crate) const AI_CREATE_COMPUTER_RIDERS: Callee = Callee {
    addr: 0x0012_9e20,
    f: ssx3_ee::s00::f_00129e20,
};

pub(crate) const AI_CREATE_MISSION_RIDER: Callee = Callee {
    addr: 0x0012_9c00,
    f: ssx3_ee::s00::f_00129c00,
};

pub(crate) const AI_RESET_RIDERS: Callee = Callee {
    addr: 0x0012_97c8,
    f: ssx3_ee::s00::f_001297c8,
};

pub(crate) const AI_RESET_RIDER_RELATIONS: Callee = Callee {
    addr: 0x0010_f3b8,
    f: ssx3_ee::s00::f_0010f3b8,
};

pub(crate) const AI_FREE_ANIM_SOURCES: Callee = Callee {
    addr: 0x0012_b200,
    f: ssx3_ee::s00::f_0012b200,
};

/// `void AI_SetupComputerRiders(AI *ai)` (0x128958): tear down any mission
/// riders, then recreate the computer riders.
pub fn ai_setup_computer_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(c.g[S0][0] as u32).off(0x94).sw(c, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x88).lw(c);
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            AI_DESTROY_MISSION_RIDERS.call(c, 0x0012_8980)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        AI_CREATE_COMPUTER_RIDERS.call(c, 0x0012_8988)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_SetupMissionRider(AI *ai)` (0x128ac0): create the mission rider
/// and reset all rider records.
pub fn ai_setup_mission_rider(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_CREATE_MISSION_RIDER.call(c, 0x0012_8ad4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        AI_RESET_RIDERS.call(c, 0x0012_8ae0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_RemoveActorRiders(AI *ai)` (0x12b000): destroy the actor riders
/// and refresh the rider relations.
pub fn ai_remove_actor_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_DESTROY_ACTOR_RIDERS.call(c, 0x0012_b014)?;
        c.g[A0][0] = c.g[S0][0];
        AI_RESET_RIDER_RELATIONS.call(c, 0x0012_b01c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_RemoveComputerRiders(AI *ai)` (0x12b030): destroy the computer
/// riders, refresh relations and clear the actor state.
pub fn ai_remove_computer_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_DESTROY_COMPUTER_RIDERS.call(c, 0x0012_b044)?;
        c.g[A0][0] = c.g[S0][0];
        AI_RESET_RIDER_RELATIONS.call(c, 0x0012_b04c)?;
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_RemoveMissionRiders(AI *ai)` (0x12b060): destroy the mission
/// riders, refresh relations and clear the actor state.
pub fn ai_remove_mission_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_DESTROY_MISSION_RIDERS.call(c, 0x0012_b074)?;
        c.g[A0][0] = c.g[S0][0];
        AI_RESET_RIDER_RELATIONS.call(c, 0x0012_b07c)?;
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_dtor(AI *ai, int free_memory)` (0x1287b8): destroy all riders,
/// delete the focus/danger managers, and (when `free_memory` is odd) the AI
/// itself.
pub fn ai_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7b78));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(c.g[S1][0] as u32).off(0xcc).sw(c, c.g[V0][0]);
        AI_DESTROY_ALL_RIDERS.call(c, 0x0012_87e0)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0xa4).lw(c);
        call::call_ptr(c, 0x0031_7e50, 0x0012_87e8)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0xa8).lw(c);
        call::call_ptr(c, 0x0031_7e50, 0x0012_87f0)?;
        c.g[S0][0] &= 1;
        let free_memory = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if free_memory {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call::call_ptr(c, 0x0031_7e50, 0x0012_8804)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of `AI_Destroy{Computer,Actor,Mission}Riders`: iterate the
/// `AIController` pointer array at `base_off` (count at `count_off`), call
/// each rider's vtable destructor (`rider[0x18] -> +0x6c0` slot), clear the
/// slot, then fold the count into the total at +0x78 and clear +`clear_off`.
pub(crate) fn ai_destroy_riders(
    c: &mut Cpu,
    count_off: u32,
    base_off: u32,
    clear_off: u32,
    head: u32,
    ret: u32,
) -> call::Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(count_off).lw(c);
    c.g[S2][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 1;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(base_off));
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = 0x3;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lw(c);
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
                c.g[A2][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = c.g[A2][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                call::call_ptr(c, target, ret)?;
            }
            c.cycles += 6;
            Addr(c.g[S0][0] as u32).sw(c, 0);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(count_off).lw(c);
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                crate::events::deliver_event_here(c, head);
            }
        }
    }
    c.cycles += 12;
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x78).lw(c);
    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(count_off).lw(c);
    Addr(c.g[S1][0] as u32).off(clear_off).sw(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    Addr(c.g[S1][0] as u32).off(count_off).sw(c, 0);
    Addr(c.g[S1][0] as u32).off(0x78).sw(c, c.g[V0][0]);
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void AI_DestroyComputerRiders(AI *ai)` (0x12ae38).
pub fn ai_destroy_computer_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        ai_destroy_riders(c, 0x80, 0x48, 0x14, 0x0012_ae60, 0x0012_ae88)
    })
}

/// `void AI_DestroyActorRiders(AI *ai)` (0x12aed0).
pub fn ai_destroy_actor_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        ai_destroy_riders(c, 0x84, 0x5c, 0x8c, 0x0012_aef8, 0x0012_af20)
    })
}

/// `void AI_DestroyMissionRiders(AI *ai)` (0x12af68).
pub fn ai_destroy_mission_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        ai_destroy_riders(c, 0x88, 0x70, 0x90, 0x0012_af90, 0x0012_afb8)
    })
}

/// `void AI_DestroyAllRiders(AI *ai)` (0x12b090): destroy the record and
/// actor riders (vtable destructors), the mission/computer/actor rider sets,
/// free the animation data sources and clear the counts.
pub fn ai_destroy_all_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x78).lw(c);
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x790).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xfc));
                call::call_ptr(c, 0x0015_4a58, 0x0012_b0d8)?;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x78).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_b0c0);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x7c).lw(c);
        c.g[S2][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A1][0] = 0x3;
                if c.g[V0][0] != 0 {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lw(c);
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
                    c.g[A2][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                    let target = c.g[A2][0] as u32;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call::call_ptr(c, target, 0x0012_b120)?;
                }
                c.cycles += 6;
                Addr(c.g[S0][0] as u32).sw(c, 0);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x7c).lw(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_b0f8);
                }
            }
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S1][0];
        AI_DESTROY_MISSION_RIDERS.call(c, 0x0012_b140)?;
        c.g[A0][0] = c.g[S1][0];
        AI_DESTROY_COMPUTER_RIDERS.call(c, 0x0012_b148)?;
        c.g[A0][0] = c.g[S1][0];
        AI_DESTROY_ACTOR_RIDERS.call(c, 0x0012_b150)?;
        Addr(c.g[S1][0] as u32).off(0x7c).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr(c.g[S1][0] as u32).off(0x78).sw(c, 0);
        AI_FREE_ANIM_SOURCES.call(c, 0x0012_b160)?;
        Addr(c.g[S1][0] as u32).off(0x74).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_FreeAnimDataSources(AI *ai)` (0x12b200): release animation data
/// source slots (the original walks a global source table; the loop only
/// ever visits slot 0).
pub fn ai_free_anim_sources(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[S2][0] & 0xff;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[GP][0] as u32).off(0xd08).lw(c);
            c.g[S1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A1][0] = c.g[S0][0];
            if c.g[S0][0] != 0 {
                c.cycles += 9;
                c.g[A0][0] = Addr(c.g[GP][0] as u32).off(0xd8c).lw(c);
                call::call_ptr(c, 0x0031_4fe8, 0x0012_b244)?;
                c.g[A0][0] = Addr(c.g[GP][0] as u32).off(0xd08).lw(c);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                Addr(c.g[V0][0] as u32).sw(c, 0);
                call::call_ptr(c, 0x0031_1110, 0x0012_b254)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x3;
                call::call_ptr(c, 0x0031_12c8, 0x0012_b260)?;
            }
            c.cycles += 2;
            let wrap = (c.g[S2][0] as i64) <= 0;
            c.g[V1][0] = c.g[S2][0] & 0xff;
            if !wrap {
                break;
            }
            if event_due(c) {
                crate::events::deliver_event_here(c, 0x0012_b220);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const AI_RESET_RIDER_RELATIONS_2: Callee = Callee {
    addr: 0x0010_f398,
    f: ssx3_ee::s00::f_0010f398,
};

pub(crate) const RIDER_RESET: Callee = Callee {
    addr: 0x0011_d390,
    f: ssx3_ee::s00::f_0011d390,
};

pub(crate) const FOCUSMAN_RESET: Callee = Callee {
    addr: 0x0010_3358,
    f: ssx3_ee::s00::f_00103358,
};

pub(crate) const DANGERMAN_RESET: Callee = Callee {
    addr: 0x0010_1688,
    f: ssx3_ee::s00::f_00101688,
};

/// `void AI_ResetRiders(AI *ai, int clear)` (0x1297c8): reset relations and
/// zero the counters (+8, +0xc when `clear`), then reset every record
/// rider (via its vtable query) and every mission rider.
pub fn ai_reset_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        AI_RESET_RIDER_RELATIONS_2.call(c, 0x0012_97ec)?;
        let clear = c.g[S3][0] != 0;
        Addr(c.g[S2][0] as u32).off(0x8).sw(c, 0);
        if clear {
            c.cycles += 1;
            Addr(c.g[S2][0] as u32).off(0xc).sw(c, 0);
        }
        c.cycles += 6;
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
        Addr(c.g[S2][0] as u32).off(0x18).sw(c, u64::MAX);
        c.g[S1][0] = 0;
        let none = (c.g[V1][0] as i64) <= 0;
        Addr(c.g[S2][0] as u32).off(0x1c).sw(c, 0);
        if !none {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            loop {
                c.cycles += 9;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call::call_ptr(c, target, 0x0012_9834)?;
                let reset = if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    true
                } else {
                    c.cycles += 2;
                    if c.g[S3][0] != 0 {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        true
                    } else {
                        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
                        false
                    }
                };
                if reset {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x790).lw(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xfc));
                    call::call_ptr(c, 0x0015_4a58, 0x0012_9858)?;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    RIDER_RESET.call(c, 0x0012_9860)?;
                    c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_9818);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x88).lw(c);
        let no_missions = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !no_missions {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            loop {
                c.cycles += 8;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lw(c);
                RIDER_RESET.call(c, 0x0012_9898)?;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x88).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_9888);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_Restart(AI *ai, int state)` (0x129768): reset the focus and
/// danger managers, clear the path pointer and reselect the race state.
pub fn ai_restart(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xa4).lw(c);
        FOCUSMAN_RESET.call(c, 0x0012_9788)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xa8).lw(c);
        DANGERMAN_RESET.call(c, 0x0012_9790)?;
        Addr(c.g[S0][0] as u32).off(0x98).sw(c, 0);
        c.g[A1][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        AI_SET_RACE_STATE.call(c, 0x0012_97a4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        AI_RESET_RIDERS.call(c, 0x0012_97b0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const RIDER_REFRESH_OVERLAPS: Callee = Callee {
    addr: 0x0012_0e50,
    f: ssx3_ee::s00::f_00120e50,
};

/// `void AI_RefreshRiderOverlaps(AI *ai)` (0x12b788): refresh the overlap
/// sets of every record rider.
pub fn ai_refresh_rider_overlaps(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            loop {
                c.cycles += 7;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                RIDER_REFRESH_OVERLAPS.call(c, 0x0012_b7c4)?;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_b7b8);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_SetSyncWords(AI* self, int* src)` (0x12bac0): copy two words into
/// the AI's sync block at `+0x20`.
pub fn ai_set_sync_words(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x20));
    c.g[V1][0] = 1;
    loop {
        c.cycles += 7;
        let src = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(src).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[A1][0] = sx(src.wrapping_add(4));
        let dst = c.g[A0][0] as u32;
        let v = c.g[V0][0];
        Addr(dst).sw(c, v);
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A0][0] = sx(dst.wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0012_bac8);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void AI_GetSyncWords(AI* self, int* dst)` (0x12baf0): copy the AI's sync
/// block at `+0x20` into `dst`.
pub fn ai_get_sync_words(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x20));
    c.g[V1][0] = 1;
    loop {
        c.cycles += 7;
        let src = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(src).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[A0][0] = sx(src.wrapping_add(4));
        let dst = c.g[A1][0] as u32;
        let v = c.g[V0][0];
        Addr(dst).sw(c, v);
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A1][0] = sx(dst.wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0012_baf8);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// The "tail-call virtual slot 2 of an interface object" shape.

/// `vt = *(VTable**)obj; vt[2].fn((char*)obj + vt[2].delta, self, tag)` — the
/// shared body of the twenty interface forwarders below. `obj` is the second
/// argument (`a1`), which the compiler keeps in `a2` (or `a3` when it also has
/// to materialize the extra constant argument in `a2`); `self` (the first
/// argument) becomes the method's second argument.
pub(crate) fn virtual_call_slot(c: &mut Cpu, ret: u32, tag: Option<u32>) -> Flow {
    c.cycles += if tag.is_some() { 13 } else { 12 };
    let frame = Frame::open(c, 0x10);
    let obj;
    if let Some(k) = tag {
        c.g[A3][0] = c.g[A1][0];
        obj = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = sx(k);
    } else {
        c.g[A2][0] = c.g[A1][0];
        obj = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
    }
    c.g[V1][0] = Addr(obj as u32).lw(c);
    let slot = VSlot::read(c, c.g[V1][0], 0x10);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(obj);
    call_ptr(c, slot.func as u32, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `RiderGateSub_LoadState(self, iface)` (0x12c590): `iface->v02(self, 0x14)`.
pub fn rider_gate_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_c5b8, Some(0x14)))
}

/// `RiderFinishLineSub_LoadState(self, iface)` (0x12c9f8): `0x4`.
pub fn rider_finish_line_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_ca20, Some(0x4)))
}

/// `RiderWipeoutSub_LoadState(self, iface)` (0x12e740): `0x80`.
pub fn rider_wipeout_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_e768, Some(0x80)))
}

/// `RiderResetSub_LoadState(self, iface)` (0x12f5e8): `0x8`.
pub fn rider_reset_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_f610, Some(0x8)))
}

/// `RiderNaturalAirSub_LoadState(self, iface)` (0x12fc28): `0x14`.
pub fn rider_natural_air_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_fc50, Some(0x14)))
}

/// `RiderBoardPressSub_LoadState(self, iface)` (0x1315d0): `0x14`.
pub fn rider_board_press_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_15f8, Some(0x14)))
}

/// `RiderRailslideSub_LoadState(self, iface)` (0x132878): `0x8`.
pub fn rider_railslide_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_28a0, Some(0x8)))
}

/// `RiderHandplantSub_LoadState(self, iface)` (0x1330f0): `0x14`.
pub fn rider_handplant_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_3118, Some(0x14)))
}

/// `RiderSpinSub_LoadState(self, iface)` (0x136230): `0x58`.
pub fn rider_spin_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_6258, Some(0x58)))
}

/// `RiderUberGrindSub_LoadState(self, iface)` (0x136920): `0x8`.
pub fn rider_uber_grind_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_6948, Some(0x8)))
}

/// `void AI_ResetRidersAndOrder(AI* ai)` (0x128a10): reset every rider and
/// then put the order mode back to 0.
pub fn ai_reset_riders_and_order(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::AI_RESET_RIDERS.call(c, 0x0012_8a28)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::AI_SET_RIDER_ORDER_MODE.call(c, 0x0012_8a34)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SectionMan_FindLogicalSection` (0x22e0e0).
pub(crate) const FIND_LOGICAL_SECTION: u32 = 0x0022_e0e0;

/// `PathData_EmptyHook` (0x26ada0).
pub(crate) const PATH_DATA_EMPTY_HOOK: u32 = 0x0026_ada0;

// ---------------------------------------------------------------------------
// AI_PathDataInitIfEarlySection.

/// `void AI_PathDataInitIfEarlySection(void* p, uchar id)` (0x12a490) — the
/// reference has no name for it: when the logical section that the world's
/// rider manager names at `+0x78` is one of the first 22, initialise the path
/// data.
pub fn ai_path_data_init_if_early_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        let world = gp(c, -0x848);
        c.g[V0][0] = Addr(world).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x78).lw(c);
        call_ptr(c, FIND_LOGICAL_SECTION, 0x0012_a4a8)?;
        c.g[V0][0] = ((c.g[V0][0] as i64) < 22i64) as u64;
        let early = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if early {
            c.cycles += 4;
            c.g[A0][0] = 0x004d_0000;
            c.g[A0][0] = sx(0x004d_33a0);
            call_ptr(c, PATH_DATA_EMPTY_HOOK, 0x0012_a4c0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider metrics and AI setup.

/// `void RiderMetrix_ResetStats(RiderMetrix* m)` (0x1175b8): clear the 0xac
/// byte stat block at `+0xfc`, mark `+0x18c` as "no trick" (−1) and carry
/// `+0x1c8` into `+0x198`.
pub fn rider_metrix_reset_stats(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0xac;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(crate::abi::r32(c, S0), 0xfc));
        callee::EA_MEM_SET.call(c, 0x0011_75d8)?;
        let this = crate::abi::r32(c, S0);
        c.g[V1][0] = lw(c, this, 0x1c8);
        c.g[V0][0] = u64::MAX;
        Addr(p(this, 0x18c)).sw(c, c.g[V0][0]);
        Addr(p(this, 0x198)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_SetupComputerRiders(cAI* ai)` (0x128958): kill the previous
/// computer riders when there were any, then create a fresh set.
pub fn ai_setup_computer_riders_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        Addr(p(this, 0x94)).sw(c, 0);
        c.g[V0][0] = lw(c, this, 0x88);
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            callee::AI_DESTROY_MISSION_RIDERS.call(c, 0x0012_8980)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::AI_CREATE_COMPUTER_RIDERS.call(c, 0x0012_8988)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The three `RiderSpinSub_IsTrickAnim1x` predicates: `kind` is the spin
/// sub-state (`+0x4`) and `trick` the animation id they test for. The
/// animation entry comes from the rider's controller at `+0x58` → `+0x784`.
pub(crate) fn rider_spin_sub_is_trick_anim(c: &mut Cpu, ret: u32, kind: u64, trick: u64) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = crate::abi::arg(c, 0);
        c.g[V0][0] = kind;
        c.g[V1][0] = lw(c, this, 4);
        let same = c.g[V1][0] == c.g[V0][0];
        c.g[A1][0] = 0;
        if same {
            c.cycles += 6;
            c.g[V0][0] = lw(c, this, 0x58);
            c.g[A1][0] = 2;
            let ctrl = crate::abi::r32(c, V0);
            c.g[A0][0] = lw(c, ctrl, 0x784);
            callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY.call(c, ret)?;
            c.g[V0][0] ^= trick;
            c.g[A1][0] = (c.g[V0][0] < 1) as u64;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[A1][0];
        frame.close(c);
        Ok(())
    })
}

/// `int RiderSpinSub_IsTrickAnim12(SpinSub* s)` (0x135eb0): sub-state 2 with
/// animation entry 0x12. Returns 0 instead of `gcc`'s `1` when the entry is
/// not a trick (`v0` is set from `v1`, as the original does).
pub fn rider_spin_sub_is_trick_anim12(c: &mut Cpu) {
    rider_spin_sub_is_trick_anim(c, 0x0013_5ed8, 2, 0x12)
}

/// `int RiderSpinSub_IsTrickAnim13(SpinSub* s)` (0x135ef0): sub-state 4 with
/// animation entry 0x13.
pub fn rider_spin_sub_is_trick_anim13(c: &mut Cpu) {
    rider_spin_sub_is_trick_anim(c, 0x0013_5f18, 4, 0x13)
}

/// `int RiderSpinSub_IsTrickAnim14(SpinSub* s)` (0x135f30): sub-state 4 with
/// animation entry 0x14.
pub fn rider_spin_sub_is_trick_anim14(c: &mut Cpu) {
    rider_spin_sub_is_trick_anim(c, 0x0013_5f58, 4, 0x14)
}

/// `bool RiderGateSub_CanStart(GateSub* g)` (0x12c078): false only when the
/// race is running (mode != 5) and the gate's timer (`g+0x14 -> +0x470`) is
/// still negative.
pub fn rider_gate_sub_can_start(c: &mut Cpu) {
    c.cycles += 8;
    let world = c.lw(gp(c, -0x848)) as u32;
    c.g[A3][0] = c.g[A0][0];
    c.g[A2][0] = 5;
    c.g[V0][0] = c.lw(world.wrapping_add(0x84));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
    c.g[A0][0] = c.lw(r32(c, V1));
    let running = c.g[A0][0] != c.g[A2][0];
    c.g[V0][0] = 1;
    if running {
        c.cycles += 6;
        c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(0x14));
        c.f[1] = 0;
        let timer = Addr(r32(c, V1).wrapping_add(0x470)).read_fpu(c);
        c.f[0] = timer.0;
        let ready = FpuF32::ZERO.le(timer);
        abi::set_fcc(c, ready);
        if !ready {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// The FPU constant the rider-gate timing multiplies the race clock by
/// (1/60: seconds per frame), at `gp_2 - 0x7840`.
pub(crate) const FRAME_SECONDS_GP: i32 = -0x7840;

/// The game-module global (`D_004A28A8`, the world/app pointer).
pub(crate) const GAME_MODULE_GP: i32 = -0x848;

// ---------------------------------------------------------------------------
// Rider gate timing.

/// `bool RiderGateSub_IsStartTimeReached(RiderGateSub* gate)` (0x12c028):
/// only while the race manager's state is 5, true when the race clock
/// (frames × 1/60) has reached the gate's start time (`gate->+0x14 ->
/// +0xb30`). 8 (+10 when racing) + 2 cycles.
pub fn ridergate_sub_is_start_time_reached(c: &mut Cpu) {
    c.cycles += 8;
    let module = c.lw(gp_2(c, GAME_MODULE_GP));
    c.g[A1][0] = c.g[A0][0];
    c.g[A0][0] = 5;
    let world = c.lw(p(module as u32, 0x84));
    let race = c.lw(p(world as u32, 0xc));
    let state = c.lw(race as u32);
    c.g[V1][0] = state;
    c.g[A2][0] = 0;
    let mut reached = 0u64;
    if state == c.g[A0][0] {
        c.cycles += 10;
        let frames = Addr(p(race as u32, 0xc)).read_fpu(c);
        c.f[0] = frames.0;
        c.cvt_s_w(0, 0);
        let seconds = Addr(gp_2(c, FRAME_SECONDS_GP)).read_fpu(c);
        c.f[1] = seconds.0;
        let rider = c.lw(p(r32(c, A1), 0x14));
        c.g[V1][0] = rider;
        c.mul_s(0, 0, 1);
        c.g[A0][0] = c.lw(p(rider as u32, 0xb30));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
        reached = c.g[V0][0] ^ 1;
    }
    c.cycles += 2;
    c.g[A2][0] = reached;
    c.g[V0][0] = reached;
    abi::ret(c);
}

/// `void AI_SetupActorRiders(AI* ai, int n)` (0x128998): clear the BE
/// interface, and unless the "computer players" flag is set, store `n` and
/// spawn the computer actor riders.
pub fn ai_setup_actor_riders(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    if call_ptr(c, 0x0014_dc80, 0x0012_89b4).is_err() {
        return;
    }
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    if call_ptr(c, 0x0014_dd58, 0x0012_89c0).is_err() {
        return;
    }
    c.g[V1][0] = 0x0053_0000;
    c.g[V0][0] = Addr(p_2(c, V1, 0x5c11)).lb(c);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.sw(p_2(c, S0, 0x94), c.g[S1][0]);
        c.g[A0][0] = c.g[S0][0];
        if callee::AI_CREATE_ACTOR_RIDERS.call(c, 0x0012_89dc).is_err() {
            return;
        }
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void RiderBoardPressSub_Cancel(RiderBoardPress* self)` (0x131348): cancel
/// a board press, finishing the metric and granting the boost when the press
/// had been held for one or two frames.
pub fn rider_board_press_sub_cancel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0013_135c);
        c.g[S0][0] = c.g[A0][0];
        callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW.call(c, 0x0013_135c)?;
        c.g[RA][0] = sx(0x0013_1364);
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS.call(c, 0x0013_1364)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(0x14).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x330).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.g[RA][0] = sx(0x0013_1384);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x790).lw(c);
            callee::RIDER_METRIX_FINISH_BOARD_PRESS.call(c, 0x0013_1384)?;
            c.g[A0][0] = Addr(r32(c, S0)).off(0x14).lw(c);
            c.mov_s(12, 0);
            c.g[RA][0] = sx(0x0013_1394);
            c.g[A1][0] = 1;
            callee::RIDER_ADD_BOOST_2.call(c, 0x0013_1394)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float RiderSpinSub_GetSpinHUDBlend(RiderSpinSub* self)` (0x136100): spin
/// amount in turns, clamped to [0, 1], eased by the square root curve.
pub fn rider_spin_sub_get_spin_hud_blend(c: &mut Cpu) {
    c.cycles += 11;
    c.lwc1(3, p(c.g[A0][0] as u32, 0x18));
    c.lwc1(0, gp(c, -0x7304));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 4);
    c.mul_s(0, 3, 0);
    c.add_s(0, 0, 4);
    c.cvt_w_s(1, 0);
    c.cvt_s_w(1, 1);
    c.c_lt_s(0, 1);
    if c.fcc() {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.sub_s(1, 1, 0);
    }
    c.cycles += 11;
    c.lwc1(0, gp(c, -0x7300));
    c.lwc1(2, gp(c, -0x72fc));
    c.mul_s(0, 1, 0);
    c.mtc1(0, 1);
    c.sub_s(0, 3, 0);
    c.abs_s(0, 0);
    c.mul_s(0, 0, 2);
    c.sub_s(0, 0, 4);
    c.max_s(0, 0, 1);
    c.add_s(0, 0, 0);
    c.pc = c.g[RA][0] as u32;
}

pub(crate) const ANIM_PLAYER_GET_ANIM_FIELD: u32 = 0x0031_2aa0;

pub(crate) const ANIM_CONTROLLER_SET_ANIM: u32 = 0x0031_28e8;

/// `void RiderPrewindSub_Exit(RiderPrewindSub* s)` (0x12e9b0): nothing to tear
/// down.
pub fn rider_prewind_sub_exit(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// RiderWipeoutSub animation selectors: look the current/next animation id up
// in a `.rodata` table and, when it actually changes, restart it through the
// rider's anim controller and rescale its speed.

/// The `RiderWipeoutSub` animation-id table (`.rodata 0x4580a0`): case block to
/// the id it selects.
pub(crate) const SWITCH_TO_ANIM_SET_A_IDS: [(u32, u64); 10] = [
    (0x0012_ddf8, 0x16a),
    (0x0012_de00, 0x16b),
    (0x0012_de08, 0x16c),
    (0x0012_de10, 0x16d),
    (0x0012_de18, 0x16e),
    (0x0012_de20, 0x16f),
    (0x0012_de28, 0x170),
    (0x0012_de30, 0x171),
    (0x0012_de38, 0x172),
    (0x0012_de40, 0x173),
];

/// `void RiderWipeoutSub_SwitchToAnimSetA(RiderWipeoutSub* w)` (0x12dd98):
/// pick animation set A's id for the rider's current animation and, if that is
/// not what is playing, restart it at speed -1 and update the anim speed.
pub fn rider_wipeout_sub_switch_to_anim_set_a(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0012_dd98 => {
                    c.cycles += 14;
                    let frame = Frame::open(c, 0x20);
                    c.g[A1][0] = 0x2;
                    frame.save(c, 16, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_PLAYER_GET_ANIM_FIELD, 0x0012_ddb8)?;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x150).lw(c);
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let same = c.g[V0][0] == 0;
                    c.g[A2][0] = c.g[A1][0];
                    pc = if same { 0x0012_de48 } else { 0x0012_ddd0 };
                }
                0x0012_ddd0 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x175));
                    c.g[V0][0] = (c.g[V1][0] < 0xfu64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x46_0000;
                    pc = if out { 0x0012_de4c } else { 0x0012_dde0 };
                }
                0x0012_dde0 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f60));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match SWITCH_TO_ANIM_SET_A_IDS.iter().find(|e| e.0 == target) {
                        Some(e) => {
                            c.cycles += 2;
                            c.g[A1][0] = e.1;
                            0x0012_de4c
                        }
                        // The table's 0x17f entry keeps the id unchanged.
                        None if target == 0x0012_de4c => 0x0012_de4c,
                        None => {
                            c.jump_out(target, 0x0012_dd98, 0x0012_de80);
                            return Ok(());
                        }
                    };
                }
                0x0012_ddf8 | 0x0012_de00 | 0x0012_de08 | 0x0012_de10 | 0x0012_de18
                | 0x0012_de20 | 0x0012_de28 | 0x0012_de30 | 0x0012_de38 | 0x0012_de40 => {
                    c.cycles += 2;
                    let id = SWITCH_TO_ANIM_SET_A_IDS
                        .iter()
                        .find(|e| e.0 == pc)
                        .unwrap()
                        .1;
                    c.g[A1][0] = id;
                    pc = 0x0012_de4c;
                }
                0x0012_de48 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x174;
                    pc = 0x0012_de4c;
                }
                0x0012_de4c => {
                    c.cycles += 2;
                    let same = c.g[A1][0] == c.g[A2][0];
                    c.g[A2][0] = 0;
                    pc = if same { 0x0012_de70 } else { 0x0012_de54 };
                }
                0x0012_de54 => {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 12);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_CONTROLLER_SET_ANIM, 0x0012_de68)?;
                    c.g[A0][0] = c.g[S0][0];
                    callee::RIDER_WIPEOUT_UPDATE_ANIM_SPEED.call(c, 0x0012_de70)?;
                    pc = 0x0012_de70;
                }
                0x0012_de70 => {
                    c.cycles += 4;
                    let frame = Frame { size: 0x20 };
                    frame.restore(c, 16, 0x10);
                    frame.restore_ra(c, 0);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0012_dd98, 0x0012_de80);
                    return Ok(());
                }
            }
        }
    })
}

/// `RiderWipeoutSub_PlayEndAnimA` (0x12df48): pick the end-animation id for the
/// rider's current animation (table `.rodata 0x458110`) and, if it changes,
/// restart it at speed -1.
pub fn rider_wipeout_sub_play_end_anim_a(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0012_df48 => {
                    c.cycles += 13;
                    let frame = Frame::open(c, 0x20);
                    c.g[A1][0] = 0x2;
                    frame.save(c, 16, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_PLAYER_GET_ANIM_FIELD, 0x0012_df68)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16a));
                    c.g[V0][0] = (c.g[V1][0] < 0x1au64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[A2][0] = c.g[A1][0];
                    pc = if out { 0x0012_dfe4 } else { 0x0012_df7c };
                }
                0x0012_df7c => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x46_0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ef0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x0012_df98 => 0x0012_df98,
                        0x0012_dfa0 => 0x0012_dfa0,
                        0x0012_dfa8 => 0x0012_dfa8,
                        0x0012_dfb0 => 0x0012_dfb0,
                        0x0012_dfb8 => 0x0012_dfb8,
                        0x0012_dfc0 => 0x0012_dfc0,
                        0x0012_dfc8 => 0x0012_dfc8,
                        0x0012_dfd0 => 0x0012_dfd0,
                        0x0012_dfd8 => 0x0012_dfd8,
                        0x0012_dfe0 => 0x0012_dfe0,
                        0x0012_dfe4 => 0x0012_dfe4,
                        _ => {
                            c.jump_out(target, 0x0012_df48, 0x0012_e010);
                            return Ok(());
                        }
                    };
                }
                0x0012_df98 | 0x0012_dfa0 | 0x0012_dfa8 | 0x0012_dfb0 | 0x0012_dfb8
                | 0x0012_dfc0 | 0x0012_dfc8 | 0x0012_dfd0 | 0x0012_dfd8 => {
                    c.cycles += 2;
                    let id = match pc {
                        0x0012_df98 => 0x185,
                        0x0012_dfa0 => 0x186,
                        0x0012_dfa8 => 0x187,
                        0x0012_dfb0 => 0x188,
                        0x0012_dfb8 => 0x189,
                        0x0012_dfc0 => 0x18a,
                        0x0012_dfc8 => 0x18b,
                        0x0012_dfd0 => 0x18c,
                        _ => 0x18d,
                    };
                    c.g[A1][0] = id;
                    pc = 0x0012_dfe4;
                }
                0x0012_dfe0 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x18e;
                    pc = 0x0012_dfe4;
                }
                0x0012_dfe4 => {
                    c.cycles += 2;
                    let same = c.g[A1][0] == c.g[A2][0];
                    c.g[A2][0] = 0;
                    pc = if same { 0x0012_e000 } else { 0x0012_dfec };
                }
                0x0012_dfec => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 12);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_CONTROLLER_SET_ANIM, 0x0012_e000)?;
                    pc = 0x0012_e000;
                }
                0x0012_e000 => {
                    c.cycles += 4;
                    let frame = Frame { size: 0x20 };
                    frame.restore(c, 16, 0x10);
                    frame.restore_ra(c, 0);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0012_df48, 0x0012_e010);
                    return Ok(());
                }
            }
        }
    })
}

/// The tumble-animation table (`.rodata 0x457fa0`): case block, animation id,
/// its cycle charge and the alternate table entry the ground flag selects.
pub(crate) const TUMBLE_ANIM_IDS: [(u32, u64, u64, u64); 33] = [
    (0x0012_dae8, 0x16a, 3, 0x175),
    (0x0012_daf4, 0x16b, 3, 0x176),
    (0x0012_db00, 0x16c, 3, 0x177),
    (0x0012_db0c, 0x170, 3, 0x17b),
    (0x0012_db18, 0x16e, 3, 0x179),
    (0x0012_db24, 0x16f, 3, 0x17a),
    (0x0012_db30, 0x172, 3, 0x17d),
    (0x0012_db3c, 0x173, 3, 0x17e),
    (0x0012_db48, 0x16d, 3, 0x178),
    (0x0012_db54, 0x171, 3, 0x17c),
    (0x0012_db60, 0x16a, 3, 0x175),
    (0x0012_db6c, 0x16b, 3, 0x176),
    (0x0012_db78, 0x16c, 3, 0x177),
    (0x0012_db84, 0x170, 3, 0x17b),
    (0x0012_db90, 0x16e, 3, 0x179),
    (0x0012_db9c, 0x173, 3, 0x17e),
    (0x0012_dba8, 0x171, 3, 0x17c),
    (0x0012_dbb4, 0x16d, 3, 0x178),
    (0x0012_dbc0, 0x16f, 3, 0x17a),
    (0x0012_dbcc, 0x172, 3, 0x17d),
    (0x0012_dbd8, 0x170, 3, 0x17b),
    (0x0012_dbe4, 0x171, 3, 0x17c),
    (0x0012_dbf0, 0x16c, 3, 0x177),
    (0x0012_dbfc, 0x16e, 3, 0x179),
    (0x0012_dc08, 0x173, 3, 0x17e),
    (0x0012_dc14, 0x170, 3, 0x17b),
    (0x0012_dc20, 0x170, 3, 0x17b),
    (0x0012_dc2c, 0x172, 3, 0x17d),
    (0x0012_dc38, 0x16b, 3, 0x176),
    (0x0012_dc44, 0x16a, 3, 0x175),
    (0x0012_dc50, 0x170, 3, 0x17b),
    (0x0012_dc5c, 0x172, 3, 0x17d),
    (0x0012_dc68, 0x174, 2, 0x17f),
];

/// `RiderWipeoutSub_SelectTumbleAnim` (0x12da88): choose the tumble animation
/// for the rider's current animation (table `.rodata 0x457fa0`) and, when it
/// changes, restart it at speed -1 and update the anim speed. `s0` records
/// whether the rider was on the ground (animation index 0), which selects the
/// second table column.
pub fn rider_wipeout_sub_select_tumble_anim(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0012_da88 => {
                    c.cycles += 17;
                    let frame = Frame::open(c, 0x30);
                    c.g[A1][0] = 0x2;
                    frame.save(c, 16, 0x20);
                    frame.save(c, 17, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A0][0];
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x80).lw(c);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x784).lw(c);
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    c.g[S0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    call_ptr(c, ANIM_PLAYER_GET_ANIM_FIELD, 0x0012_dab8)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x148));
                    c.g[V0][0] = (c.g[V1][0] < 0x22u64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[A2][0] = c.g[A1][0];
                    pc = if out { 0x0012_dc74 } else { 0x0012_dacc };
                }
                0x0012_dacc => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x45_0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7fa0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    if let Some(e) = TUMBLE_ANIM_IDS.iter().find(|e| e.0 == target) {
                        c.cycles += e.2;
                        c.g[A1][0] = e.1;
                        c.g[V0][0] = e.3;
                        pc = 0x0012_dc70;
                    } else {
                        c.jump_out(target, 0x0012_da88, 0x0012_dcac);
                        return Ok(());
                    }
                }
                0x0012_dae8 | 0x0012_daf4 | 0x0012_db00 | 0x0012_db0c | 0x0012_db18
                | 0x0012_db24 | 0x0012_db30 | 0x0012_db3c | 0x0012_db48 | 0x0012_db54
                | 0x0012_db60 | 0x0012_db6c | 0x0012_db78 | 0x0012_db84 | 0x0012_db90
                | 0x0012_db9c | 0x0012_dba8 | 0x0012_dbb4 | 0x0012_dbc0 | 0x0012_dbcc
                | 0x0012_dbd8 | 0x0012_dbe4 | 0x0012_dbf0 | 0x0012_dbfc | 0x0012_dc08
                | 0x0012_dc14 | 0x0012_dc20 | 0x0012_dc2c | 0x0012_dc38 | 0x0012_dc44
                | 0x0012_dc50 | 0x0012_dc5c | 0x0012_dc68 => {
                    let e = TUMBLE_ANIM_IDS.iter().find(|e| e.0 == pc).unwrap();
                    c.cycles += e.2;
                    c.g[A1][0] = e.1;
                    c.g[V0][0] = e.3;
                    pc = 0x0012_dc70;
                }
                0x0012_dc70 => {
                    c.cycles += 1;
                    if c.g[S0][0] != 0 {
                        c.g[A1][0] = c.g[V0][0];
                    }
                    pc = 0x0012_dc74;
                }
                0x0012_dc74 => {
                    c.cycles += 2;
                    let same = c.g[A1][0] == c.g[A2][0];
                    c.g[A2][0] = 0;
                    pc = if same { 0x0012_dc98 } else { 0x0012_dc7c };
                }
                0x0012_dc7c => {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x80).lw(c);
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 12);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_CONTROLLER_SET_ANIM, 0x0012_dc90)?;
                    c.g[A0][0] = c.g[S1][0];
                    callee::RIDER_WIPEOUT_UPDATE_ANIM_SPEED.call(c, 0x0012_dc98)?;
                    pc = 0x0012_dc98;
                }
                0x0012_dc98 => {
                    c.cycles += 5;
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, 16, 0x20);
                    frame.restore(c, 17, 0x10);
                    frame.restore_ra(c, 0);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0012_da88, 0x0012_dcac);
                    return Ok(());
                }
            }
        }
    })
}

/// `AI_LoadNextRider(self)` (0x1296f8): find the first roster rider that is
/// not loaded and load its step. A `match pc` dispatcher re-enterable at its
/// loop head (0x129720) after an unwind.
pub fn ai_load_next_rider(c: &mut Cpu) {
    const ENTRY: u32 = 0x0012_96f8;
    const END: u32 = 0x0012_9764;
    const AT_HEAD: u32 = 0x0012_9720;
    const AT_DEAD: u32 = 0x0012_971c;
    guest(c, |c| {
        let mut stage = match c.pc {
            ENTRY => 0u8,
            AT_HEAD => 1,
            AT_DEAD => 6,
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        };
        // The frame is opened at the entry stage; resume stages reuse it.
        let mut frame = Frame { size: 0x40 };
        loop {
            match stage {
                0 => {
                    c.cycles += 9;
                    frame = Frame::open(c, 0x40);
                    frame.save(c, S1, 0x20);
                    frame.save(c, S0, 0x30);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S2, 0x10);
                    c.g[S0][0] = sx(p(c.g[S1][0] as u32, 0x28));
                    frame.save_ra(c, 0);
                    c.g[S2][0] = 0;
                    stage = 2;
                }
                2 => {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x78)).lw(c);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        frame.restore(c, S0, 0x30);
                        stage = 5;
                    } else {
                        stage = 3;
                    }
                }
                3 => {
                    c.cycles += 4;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    call_ptr(c, 0x0011_d640, 0x0012_973c)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                        if back_edge_unwinds(c, AT_HEAD) {
                            return Ok(());
                        }
                        stage = 1;
                    } else {
                        stage = 4;
                    }
                }
                4 => {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    call_ptr(c, 0x0011_c298, 0x0012_974c)?;
                    frame.restore(c, S0, 0x30);
                    stage = 5;
                }
                1 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    stage = 2;
                }
                6 => {
                    c.cycles += 1;
                    stage = 1;
                }
                _ => {
                    c.cycles += 5;
                    let frame = Frame { size: 0x40 };
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
    })
}

/// `void RiderGateSub_UpdateWaitPush(RiderGateSub* g, AiBits* bits)`
/// (0x12c0c0): when the signed 6-bit field at `bits+0` scaled by 0.032258 is
/// positive, play the landing animation (id 2) and set `g->state = 3`.
pub fn rider_gate_sub_update_wait_push(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.lwc1(2, gp(c, -0x783c));
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.mtc1(0, 1);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 14);
        let v = c.g[V0][0] as u32;
        c.g[V0][0] = sx(((v as i32) >> 26) as u32);
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 2);
        c.c_lt_s(1, 0);
        let not_positive = !c.fcc();
        c.g[S0][0] = c.g[A0][0];
        if !not_positive {
            c.cycles += 9;
            c.g[V0][0] = Addr(p_3(r32(c, S0), 0x14)).lw(c);
            c.g[A1][0] = 2;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr(p_3(c.g[V0][0] as u32, 0x784)).lw(c);
            call::call_ptr(c, 0x0031_28e8, 0x0012_c118)?;
            c.g[V1][0] = 3;
            Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
        }
        c.cycles += 4;
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `AI_ComputerRidersHaveRenderSlots` (0x12ABD0): true when every computer
/// rider in the rider list has a render slot.
pub fn ai_computer_riders_have_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, S2, 0x80);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 1;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = low(c, S2).wrapping_add(0x48);
            loop {
                c.cycles += 9;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                c.g[S1][0] = low(c, S1).wrapping_add(1);
                c.g[S0][0] = low(c, S0).wrapping_add(4);
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x18)).lw(c);
                callee::RIDER_HAS_RENDER_SLOT.call(c, 0x0012_ac14)?;
                c.g[V1][0] = lw_at(c, S2, 0x80);
                c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                if c.g[V0][0] == 0 {
                    c.g[S3][0] = 0;
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_ac00);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AI_ActorRidersHaveRenderSlots` (0x12ACF8): the same query for the actor
/// rider list (count at +0x84, riders at +0x5c).
pub fn ai_actor_riders_have_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, S2, 0x84);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 1;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = low(c, S2).wrapping_add(0x5c);
            loop {
                c.cycles += 9;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                c.g[S1][0] = low(c, S1).wrapping_add(1);
                c.g[S0][0] = low(c, S0).wrapping_add(4);
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x18)).lw(c);
                callee::RIDER_HAS_RENDER_SLOT.call(c, 0x0012_ad3c)?;
                c.g[V1][0] = lw_at(c, S2, 0x84);
                c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                if c.g[V0][0] == 0 {
                    c.g[S3][0] = 0;
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_ad28);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AI_GetSyncChecksum(self)` (0x12b420): start from the AI's random value
/// and add the sync checksum of each rider in the array at `+0x28`.
pub fn ai_get_sync_checksum(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x50));
        c.sq(19, sp_2(c).wrapping_add(0x10));
        c.sq(17, sp_2(c).wrapping_add(0x30));
        c.g[S3][0] = c.g[A0][0];
        c.sq(18, sp_2(c).wrapping_add(0x20));
        c.sq(16, sp_2(c).wrapping_add(0x40));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S2][0] = 0;
        call_ptr(c, callee::AI_RAND, 0x0012_b444)?;
        c.g[V1][0] = lw_r(c, S3, 0x78);
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[S1][0] = c.g[V0][0];
        if !empty {
            c.cycles += 2;
            c.g[S0][0] = sx(low_2(c, S3).wrapping_add(0x28));
            loop {
                c.cycles += 8;
                c.g[A0][0] = Addr(low_2(c, S0)).lw(c);
                c.g[S2][0] = sx(low_2(c, S2).wrapping_add(1));
                c.g[S0][0] = sx(low_2(c, S0).wrapping_add(4));
                callee::RIDER_GET_SYNC_CHECKSUM.call(c, 0x0012_b468)?;
                c.g[V1][0] = lw_r(c, S3, 0x78);
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                c.g[S1][0] = sx(low_2(c, S1).wrapping_add(low_2(c, V0)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_b458);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, sp_2(c).wrapping_add(0x40));
        c.lq(17, sp_2(c).wrapping_add(0x30));
        c.lq(18, sp_2(c).wrapping_add(0x20));
        c.lq(19, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low_2(c, RA);
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void AI_ResetRiderStats(race*)` (0x0012b180): reset the score and metric
/// stats of every rider in the race.
pub fn ai_reset_rider_stats(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, S2, 0x78)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S0][0] = sx(word(c, S2).wrapping_add(0x28));
            c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
            loop {
                c.cycles += 13;
                c.g[A0][0] = c.g[S1][0];
                c.g[S1][0] = sx(word(c, S1).wrapping_add(1));
                c.g[A1][0] = Addr(at(c, V0, 0x790)).lw(c);
                c.g[RA][0] = sx(0x0012_b1c4);
                c.g[A1][0] = sx(word(c, A1).wrapping_add(0xfc));
                callee::BE_SCORE_INT_UPDATE_BEST_STATS.call(c, 0x0012_b1c4)?;
                c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
                c.g[S0][0] = sx(word(c, S0).wrapping_add(4));
                c.g[RA][0] = sx(0x0012_b1d4);
                c.g[A0][0] = Addr(at(c, V0, 0x790)).lw(c);
                callee::RIDER_METRIX_RESET_STATS_2.call(c, 0x0012_b1d4)?;
                c.g[V0][0] = Addr(at(c, S2, 0x78)).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0012_b1b0);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderBoardPressSub_SettleSlew(rider)` (0x001313a8): clamp the board
/// press sub-state's slew against ±0.5, or snap it to the 1.0/0.3 targets.
pub fn rider_board_press_sub_settle_slew(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = Addr(at(c, A0, 0x14)).lw(c);
    c.g[AT][0] = 0xffff_ffff_bf00_0000;
    c.mtc1(AT, 0);
    c.lwc1(1, at(c, V0, 0x280));
    c.c_lt_s(1, 0);
    if !c.fcc() {
        c.cycles += 6;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.c_lt_s(0, 1);
        let below = !c.fcc();
        c.lwc1(0, at(c, V0, 0x300));
        if below {
            c.cycles += 5;
            c.lwc1(1, gp_3(c, -0x75f8));
            // The slew's step is 0.025 per 60 Hz tick (times the time scale).
            c.f[1] = rate_step_60(c, FpuF32(c.f[1])).0;
            Addr(at(c, V0, 0x288)).sw(c, 0);
            c.mul_s(0, 0, 1);
            let ra = word(c, RA);
            c.swc1(0, at(c, V0, 0x284));
            c.pc = ra;
            return;
        }
        c.cycles += 5;
        c.lwc1(1, gp_3(c, -0x75fc));
        // 0.025 per 60 Hz tick.
        c.f[1] = rate_step_60(c, FpuF32(c.f[1])).0;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        c.mul_s(0, 0, 1);
    } else {
        c.cycles += 5;
        c.lwc1(0, at(c, V0, 0x300));
        c.lwc1(1, gp_3(c, -0x7600));
        // 0.025 per 60 Hz tick.
        c.f[1] = rate_step_60(c, FpuF32(c.f[1])).0;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 2);
        c.mul_s(0, 0, 1);
    }
    c.cycles += 3;
    c.swc1(2, at(c, V0, 0x288));
    let ra = word(c, RA);
    c.swc1(0, at(c, V0, 0x284));
    c.pc = ra;
}

/// `void AI_UpdateRiderPoses(World* w)` (0x00129160): tick, update the
/// skeleton and rebuild the bone matrices for the world's first `+0x78`
/// riders (the rider pointers start at `+0x28`).
pub fn ai_update_rider_poses(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x78));
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            loop {
                c.cycles += 14;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 12);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0011_eb60, 0x0012_91a0)?; // Rider_TickAnimation
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                call_ptr(c, 0x0011_eb98, 0x0012_91a8)?; // Rider_UpdateSkeleton
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x780));
                call_ptr(c, 0x0031_03f0, 0x0012_91b8)?; // RiderModel_BuildBoneMatrices
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x78));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0012_9190);
                }
            }
        }
        c.cycles += 6;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `float RiderSpinSub_GetFlipHUDBlend(SpinSub* self)` (0x136168): blend of
/// the rider's spin angle folded into a quarter turn, then scaled.
pub fn rider_spin_sub_get_flip_hud_blend(c: &mut Cpu) {
    c.cycles += 11;
    c.lwc1(3, at(c, A0, 0x1c));
    c.lwc1(0, gp_4(c, -0x72f8));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 1);
    c.mul_s(0, 3, 0);
    c.add_s(0, 0, 1);
    c.cvt_w_s(1, 0);
    c.cvt_s_w(1, 1);
    c.c_lt_s(0, 1);
    if !c.fcc() {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.sub_s(1, 1, 0);
    }
    c.cycles += 11;
    c.lwc1(0, gp_4(c, -0x72f4));
    c.lwc1(2, gp_4(c, -0x72f0));
    c.mul_s(0, 1, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.sub_s(0, 3, 0);
    c.abs_s(0, 0);
    c.mul_s(2, 0, 2);
    c.c_lt_s(1, 2);
    if !c.fcc() {
        c.cycles += 3;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(AT, 0);
        c.sub_s(2, 2, 0);
    }
    c.cycles += 8;
    c.g[AT][0] = 0x3e80_0000;
    c.mtc1(AT, 0);
    c.mtc1(ZERO, 1);
    c.sub_s(2, 2, 0);
    c.lwc1(0, gp_4(c, -0x72ec));
    c.max_s(2, 2, 1);
    let ra = r32(c, RA);
    c.mul_s(0, 2, 0);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Rider.

/// `int RiderRailslideSub_SetRail(self, int id)` (0x132620): remember the
/// rail id, ask the rider's rail state to play its enter animation when the
/// rail flag is set, and return 1 in that case.
pub fn rider_railslide_sub_set_rail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0xffff_ffff);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let none = c.g[S1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if none {
            c.cycles += 1;
            Addr(p(r32(c, S0), 4)).sw(c, c.g[S1][0]);
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.mtc1(0, 1);
            c.lwc1(0, p(r32(c, V1), 0x2f0));
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, V1), 0xb2c)).lw(c);
                c.g[V0][0] = sx(((r32(c, V0) as i32) >> 1) as u32);
                c.g[V0][0] = c.g[V0][0] & 1;
                let rail_off = c.g[V0][0] == 0;
                c.g[A1][0] = 0xc;
                if !rail_off {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(p(r32(c, V1), 0x77c)).lw(c);
                    Addr(p(r32(c, V0), 0x394)).sw(c, c.g[S1][0]);
                    c.g[A0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                    call_ptr(c, 0x0011_fec8, 0x00132680)?;
                    c.g[V0][0] = 1;
                    c.cycles += 5;
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            }
            c.cycles += 2;
            if c.g[S1][0] == c.g[V0][0] {
                Addr(p(r32(c, S0), 4)).sw(c, c.g[S1][0]);
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 5;
                call_ptr(c, 0x0028_b180, 0x0013269c)?;
                c.g[A1][0] = Addr(p(r32(c, S0), 8)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0029_9b70, 0x001326a8)?;
                c.cycles += 1;
                Addr(p(r32(c, S0), 4)).sw(c, c.g[S1][0]);
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderRailslideSub_PlayEnterAnim(self, int alt)` (0x1326c8): play
/// the railslide enter animation matching the rider's state and side.
pub fn rider_railslide_sub_play_enter_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 4;
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, A0), 8)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x328)).lw(c);
        let not4 = c.g[A0][0] != c.g[V0][0];
        c.g[V0][0] = 3;
        // 1: the `alt` entry (b_132748), 2: the shared 0x12 entry (b_132730).
        let mut tail = 0u32;
        if not4 {
            c.cycles += 2;
            let not3 = c.g[A0][0] != c.g[V0][0];
            if not3 {
                c.cycles += 2;
                let alt0 = c.g[A1][0] == 0;
                c.g[A1][0] = 0x44;
                if alt0 {
                    tail = 1;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                    tail = 2;
                }
            } else {
                c.cycles += 2;
                let alt0 = c.g[A1][0] == 0;
                c.g[A1][0] = 0x46;
                if alt0 {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                    c.g[A1][0] = 0x13;
                    tail = 2;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                    tail = 2;
                }
            }
        } else {
            c.cycles += 2;
            let alt0 = c.g[A1][0] == 0;
            c.g[A1][0] = 0x45;
            if alt0 {
                c.cycles += 3;
                c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                c.g[A1][0] = 0x14;
                tail = 2;
            } else {
                c.cycles += 2;
                c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                tail = 2;
            }
        }
        if tail == 1 {
            c.cycles += 6;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_28e8, 0x00132740)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        } else {
            c.cycles += 7;
            c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
            c.g[A1][0] = 0x12;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_28e8, 0x00132760)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider finish line / wipeout.

/// `void RiderFinishLineSub_Enter(self)` (0x12c5c8): reset the rider metrics,
/// compute the finish direction and mark whether the finish line has been
/// crossed.
pub fn rider_finish_line_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'end: {
            c.cycles += 24;
            c.g[A1][0] = 1;
            frame.save(c, S0, 0x10);
            frame.save_ra(c, 0);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = lw_3(c, at_2(c.g[S0][0], 4));
            c.g[RA][0] = sx(0x0012_c5e8);
            c.g[A0][0] = lw_3(c, at_2(c.g[V0][0], 0x790));
            callee::RIDER_METRIX_ON_RIDER_RESET.call(c, 0x0012_c5e8)?;
            c.g[V1][0] = lw_3(c, at_2(c.g[S0][0], 4));
            c.lwc1(1, gp_5(c, 0x1b9c));
            c.lqc2(3, at_2(c.g[V1][0], 0x1e0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 0);
            c.c_lt_s(1, 0);
            if c.fcc() {
                Addr(c.g[S0][0] as u32).sw(c, 0);
                break 'end;
            }
            c.cycles += 6;
            c.lwc1(1, at_2(c.g[V1][0], 0x2dc));
            c.mtc1(0, 0);
            c.c_eq_s(1, 0);
            if !c.fcc() {
                Addr(c.g[S0][0] as u32).sw(c, 0);
                break 'end;
            }
            'check: {
                c.cycles += 6;
                c.lwc1(1, at_2(c.g[V1][0], 0x378));
                c.lwc1(0, gp_5(c, -0x77fc));
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = 1;
                if t {
                    break 'check;
                }
                c.cycles += 2;
                Addr(c.g[S0][0] as u32).sw(c, 0);
                break 'end;
            }
            c.cycles += 1;
            Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            break 'end;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderWipeoutSub_UpdatePhase4(self)` (0x12d9d8): advance the wipeout
/// recovery phase; once the animation is over (or the timer expired) apply the
/// recovery boost and re-check the reset path.
pub fn rider_wipeout_sub_update_phase4(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'first: {
            'restart: {
                c.cycles += 12;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 1);
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.lwc1(0, at_2(c.g[S0][0], 0x70));
                c.c_le_s(1, 0);
                let t = c.fcc();
                c.g[S1][0] = 0;
                if t {
                    break 'restart;
                }
                {
                    c.cycles += 7;
                    c.g[V0][0] = lw_3(c, at_2(c.g[S0][0], 0x80));
                    c.mtc1(0, 1);
                    c.lwc1(0, at_2(c.g[V0][0], 0x470));
                    c.c_le_s(1, 0);
                    let t = c.fcc();
                    c.g[V1][0] = 1;
                    if !t {
                        c.cycles += 1;
                        c.g[V1][0] = 0;
                    }
                }
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    c.g[A0][0] = lw_3(c, at_2(c.g[V0][0], 0x784));
                    break 'first;
                }
                break 'restart;
            }
            c.cycles += 3;
            c.g[V0][0] = lw_3(c, at_2(c.g[S0][0], 0x80));
            c.g[S1][0] = 1;
            c.g[A0][0] = lw_3(c, at_2(c.g[V0][0], 0x784));
            break 'first;
        }
        'second: {
            'play: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0012_da44);
                c.g[A1][0] = 2;
                callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED.call(c, 0x0012_da44)?;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = lw_3(c, at_2(c.g[S0][0], 0x80));
                    break 'play;
                }
                c.cycles += 2;
                if c.g[S1][0] == 0 {
                    frame.restore(c, S0, 0x20);
                    break 'second;
                }
                c.cycles += 1;
                c.g[A0][0] = lw_3(c, at_2(c.g[S0][0], 0x80));
                break 'play;
            }
            c.cycles += 7;
            c.g[RA][0] = sx(0x0012_da60);
            c.g[A1][0] = c.g[S1][0];
            callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST.call(c, 0x0012_da60)?;
            c.g[A0][0] = lw_3(c, at_2(c.g[S0][0], 0x80));
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x0012_da70);
            c.g[A2][0] = 2;
            callee::RIDER_CHECK_RESET_PATH.call(c, 0x0012_da70)?;
            frame.restore(c, S0, 0x20);
            break 'second;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `AI_ResetComputerRenderSlots`/`AI_ResetActorRenderSlots`: clear the "render
// slot taken" word of every rider of the AI's list, release the slot in the
// global graphics manager and store the slot number + 4.
pub(crate) fn ai_reset_render_slots(
    c: &mut Cpu,
    zero_off: u32,
    count_off: u32,
    items_off: u32,
    ret: u32,
) -> crate::call::Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    frame.save(c, S2, 0x30);
    frame.save(c, S3, 0x20);
    frame.save(c, S4, 0x10);
    frame.save_ra(c, 0);
    c.g[S3][0] = c.g[A0][0];
    c.sw((c.g[S3][0] as u32).wrapping_add(zero_off), 0u64);
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(count_off));
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[S2][0] = 0u64;
    if !none {
        c.cycles += 2;
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(items_off));
        loop {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x884));
            if c.g[V1][0] != 0 {
                c.cycles += 13;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x888), c.g[S4][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x884), 0u64);
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[S0][0] = c.lw((c.g[S1][0] as u32));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x390));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x394));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, target, ret)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x888), c.g[V0][0]);
            }
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(count_off));
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, ret - 0x38);
            }
        }
    }
    c.cycles += 8;
    c.lq(16, frame.at(c, 0x50));
    c.lq(17, frame.at(c, 0x40));
    c.lq(18, frame.at(c, 0x30));
    c.lq(19, frame.at(c, 0x20));
    c.lq(20, frame.at(c, 0x10));
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `AI_ResetComputerRenderSlots` (0x12ab20): free every computer rider's
/// render slot.
pub fn ai_reset_computer_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        ai_reset_render_slots(c, 0x14, 0x80, 0x48, 0x0012_ab90)
    })
}

/// `AI_ResetActorRenderSlots` (0x12ac48): free every actor rider's render slot.
pub fn ai_reset_actor_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        ai_reset_render_slots(c, 0x8c, 0x84, 0x5c, 0x0012_acb8)
    })
}

// ---------------------------------------------------------------------------
// AI_ForceFinishRiders (0x000012b340).

/// `AI_ForceFinishRiders`: when the AI race state is 5, force every rider that has not finished to the end of the course.
pub fn ai_force_finish_riders(c: &mut Cpu) {
    guest(c, |c| {
        'b_12b400: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V1][0] = 0x5;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = c.lw((c.g[S3][0] as u32));
            let t = c.g[V0][0] != c.g[V1][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_12b400;
            } else {
                'b_12b3fc: {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    c.g[S2][0] = 0u64;
                    call_ptr(c, 0x00258950, 0x0012b380)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x7c));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S4][0] = 0x1;
                    if t {
                        break 'b_12b3fc;
                    } else {
                        c.cycles += 3;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 20);
                        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x40));
                        'l_12b398: loop {
                            'b_12b3ec: {
                                c.cycles += 5;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x87c));
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x7c));
                                    break 'b_12b3ec;
                                } else {
                                    'b_12b3c4: {
                                        c.cycles += 5;
                                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x470));
                                        c.c_le_s(20, 0);
                                        let t = c.fcc();
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_12b3c4;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] = 0u64;
                                            break 'b_12b3c4;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_12b3fc;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                        call_ptr(c, 0x0026fa50, 0x0012b3d8)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x480), c.g[S4][0]);
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x00125108, 0x0012b3e8)?;
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x7c));
                                        break 'b_12b3ec;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0012b398);
                                }
                                continue 'l_12b398;
                            } else {
                                break 'b_12b3fc;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_12b400;
            }
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

/// `sra` (arithmetic), then `andi 1`.
#[inline(always)]
pub(crate) fn bit(v: u64, n: u32) -> u64 {
    (((v as u32) as i32 >> n) as u32 & 1) as u64
}

/// `srl`, then `andi 1`.
#[inline(always)]
pub(crate) fn ubit(v: u64, n: u32) -> u64 {
    ((v as u32) >> n & 1) as u64
}

/// `AI_ApplyComputerRiderSettings(self, rider)` (0x12b698): apply the AI
/// options to a computer rider. The boost setting comes from option bits 3,
/// 0 and 5 (3 disables, 0 zeroes, 5 lowers it, else 1); then the boost meter
/// is reset, the boost time and boost enabling follow bits 3 and 1, and the
/// rider's handicap is stored last.
pub fn ai_apply_computer_rider_settings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let rider = abi::arg(c, 1);
        c.g[V0][0] = 0x53_0000;
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[V0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
        c.g[S0][0] = c.g[A1][0];
        c.g[V1][0] = lw_5(c, p(c.g[V0][0] as u32, 0x18));
        c.g[V0][0] = bit(c.g[V1][0], 3) as u64;
        c.g[A1][0] = 1;
        if c.g[V0][0] == 0 {
            // option bit 3 clear: test bit 0 of the same word
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & 1;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[V1][0] = 2;
                c.g[V0][0] = ubit(lw_5(c, p(c.g[S1][0] as u32, 0x8b8)), 5);
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = c.g[V1][0];
                }
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0;
            }
        } else {
            c.cycles += 2;
            c.g[A1][0] = 3;
        }
        c.cycles += 8;
        let state = lw_5(c, p(rider, 0x18));
        c.g[A0][0] = state;
        c.sw(p(state as u32, 0x304), c.g[A1][0]);
        Callee {
            addr: 0x0012_0d58,
            f: ee::s00::Rider_ResetBoostMeter,
        }
        .call(c, 0x0012_b704)?;
        c.g[V0][0] = ubit(lw_5(c, p(c.g[S1][0] as u32, 0x8b8)), 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8b8));
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = lw_5(c, p(rider, 0x18));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.swc1(0, p(c.g[V0][0] as u32, 0xb24));
        }
        c.cycles += 5;
        c.g[V0][0] = lw_5(c, p(c.g[A0][0] as u32, 0x18));
        c.g[V0][0] = bit(c.g[V0][0], 4) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = 2;
        if !t {
            c.cycles += 2;
            c.g[V1][0] = lw_5(c, p(rider, 0x18));
            c.sw(p(c.g[V1][0] as u32, 0xb28), c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[V0][0] = lw_5(c, p(c.g[A0][0] as u32, 0x18));
        c.g[V0][0] = bit(c.g[V0][0], 1) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = lw_5(c, p(rider, 0x18));
            c.g[A1][0] = 1;
            Callee {
                addr: 0x0012_5958,
                f: ee::s00::Rider_SetBoostEnabled,
            }
            .call(c, 0x0012_b764)?;
        }
        c.cycles += 9;
        call_ptr(c, 0x0014_f7e8, 0x0012_b76c)?;
        c.g[V1][0] = lw_5(c, p(rider, 0x18));
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        c.sw(p(c.g[V1][0] as u32, 0xb30), c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `RiderGateSub_UpdatePush(self)` (0x12c130): the rider-gate push
/// sub-state. Snap the slew targets to 1/0, then either the gate animation
/// finished (go to the sub-state after the push) or, while the rider is in
/// main state 3, wait for the push animation to pass halfway before jumping
/// to main state 1 and sub-state 0.
pub fn rider_gate_sub_update_push(c: &mut Cpu) {
    const SET_SLEW_TARGETS: Callee = Callee {
        addr: 0x0011_3f88,
        f: ee::s00::Rider_SetSlew3And4Targets,
    };
    const GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };
    const SET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe78,
        f: ee::s00::Rider_SetMainState,
    };
    const SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::Rider_SetSubState,
    };
    const ANIM_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;
    const ANIM_GET_LAYER_PROGRESS: u32 = 0x0031_2ab0;

    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12); // f12 = 1.0
        frame.save(c, 16, 0x40);
        frame.save_ra(c, 0x30);
        c.g[S0][0] = c.g[A0][0];
        c.mtc1(0, 13); // f13 = 0.0
        c.g[A0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
        SET_SLEW_TARGETS.call(c, 0x0012_c154)?;
        c.g[V0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
        c.g[A1][0] = 2;
        c.g[A0][0] = lw_6(c, p(c.g[V0][0] as u32, 0x784));
        call_ptr(c, ANIM_GET_LAYER_ANIM_ID, 0x0012_c164)?;
        c.g[V1][0] = 2;
        let anim_done = c.g[V0][0] != c.g[V1][0];
        if anim_done {
            // leave the push: main state 1 -> sub-state 4; else sub-state 0
            c.cycles += 5;
            c.g[A0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
            GET_MAIN_STATE.call(c, 0x0012_c1f8)?;
            c.g[V1][0] = 1;
            let not_state_one = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
            if !not_state_one {
                c.cycles += 4;
                c.g[A1][0] = 4;
                SET_SUB_STATE.call(c, 0x0012_c20c)?;
                sw_2(c, c.g[S0][0] as u32, 0);
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0;
                SET_SUB_STATE.call(c, 0x0012_c21c)?;
                sw_2(c, c.g[S0][0] as u32, 0);
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x40));
        } else {
            c.cycles += 5;
            c.g[A0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
            GET_MAIN_STATE.call(c, 0x0012_c178)?;
            c.g[V1][0] = 3;
            if c.g[V0][0] != c.g[V1][0] {
                c.lq(16, frame.at(c, 0x40));
            } else {
                c.cycles += 10;
                c.g[V0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
                c.g[A1][0] = 2;
                c.g[A0][0] = lw_6(c, p(c.g[V0][0] as u32, 0x784));
                call_ptr(c, ANIM_GET_LAYER_PROGRESS, 0x0012_c194)?;
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 1); // f1 = 0.5
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    // progress < 0.5: keep pushing
                    c.lq(16, frame.at(c, 0x40));
                } else {
                    c.cycles += 17;
                    c.g[V1][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
                    c.g[A1][0] = 0;
                    c.lwc1(0, gp(c, -0x7838));
                    c.lq(2, p(c.g[V1][0] as u32, 0x1b0));
                    c.sq(2, frame.at(c, 0x10));
                    c.mfc1(4, 0);
                    c.lqc2(4, frame.at(c, 0x10));
                    c.qmtc2(4, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x20));
                    c.lq(2, frame.at(c, 0x20));
                    c.sq(2, frame.at(c, 0));
                    c.sq(2, p(c.g[V1][0] as u32, 0x1e0));
                    c.g[A0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
                    SET_MAIN_STATE.call(c, 0x0012_c1e8)?;
                    c.lq(16, frame.at(c, 0x40));
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
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
            c.g[V1][0] = c.ldw_2(A1, 0x4);
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
                    c.stw_2(A0, 0xc, 0u64);
                    break 'b_131280;
                } else {
                    c.cycles += 8;
                    c.g[V0][0] = c.ldw_2(A0, 0x14);
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
        c.stw_2(A0, 0xc, 0u64);
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
                c.g[V1][0] = c.ldw_2(A0, 0x14);
                if t {
                    break 'b_1312e0;
                } else {
                    'b_1312c4: {
                        c.cycles += 4;
                        c.g[A1][0] = c.ldw_2(V1, 0x330);
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
                        c.g[V1][0] = c.ldw_2(A0, 0x14);
                        break 'b_131320;
                    } else {
                        c.cycles += 5;
                        c.lwc1(0, c.pa(GP, -0x7608));
                        c.c_lt_s(4, 0);
                        if !c.fcc() {
                            c.g[V1][0] = c.ldw_2(A0, 0x14);
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
        c.stw_2(V1, 0x27c, 0u64);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 2);
        c.mul_s(0, 0, 1);
        c.swc1(0, c.pa(V1, 0x278));
        c.g[V0][0] = c.ldw_2(A0, 0x14);
        c.lwc1(0, c.pa(V0, 0x274));
        c.c_lt_s(0, 2);
        if !c.fcc() {
            c.g[V1][0] = c.ldw_2(A0, 0x14);
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
                c.stw_2(A1, 0, c.g[V0][0]);
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
        c.g[V0][0] = c.ldw_2(S3, 0x14);
        c.g[V1][0] = c.ldw_2(V0, 0x320);
        c.g[V1][0] = c.g[V1][0] ^ 0x1;
        c.stw_2(V0, 0x320, c.g[V1][0]);
        c.g[V1][0] = c.ldw_2(S3, 0x14);
        c.g[V0][0] = c.ldw_2(V1, 0x320);
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
            c.g[S2][0] = c.ldw_2(V1, 0x784);
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
            c.g[V1][0] = c.ldw_2(S3, 0x14);
            c.g[A0][0] = c.ldw_2(V1, 0x320);
            c.g[V0][0] = c.ldw_2(V1, 0x784);
            c.stw_2(V0, 0x18, c.g[A0][0]);
            c.g[V1][0] = c.ldw_2(S3, 0x14);
            c.g[A0][0] = c.ldw_2(V1, 0x328);
            let t = c.g[A0][0] != c.g[A1][0];
            c.g[V0][0] = 0x2;
            if t {
                break 'b_12ffa8;
            } else {
                c.cycles += 2;
                let t = true;
                c.stw_2(V1, 0x328, c.g[V0][0]);
                if t {
                    break 'b_12ffb0;
                } else {
                    break 'b_12ffa8;
                }
            }
        }
        c.cycles += 2;
        if c.g[A0][0] == c.g[V0][0] {
            c.stw_2(V1, 0x328, c.g[A1][0]);
            break 'b_12ffb0;
        } else {
            break 'b_12ffb0;
        }
    }
    c.cycles += 17;
    c.g[V0][0] = c.ldw_2(S3, 0x14);
    c.lwc1(0, c.pa(V0, 0x280));
    c.neg_s(0, 0);
    c.swc1(0, c.pa(V0, 0x280));
    c.g[V1][0] = c.ldw_2(S3, 0x14);
    c.lwc1(0, c.pa(V1, 0x288));
    c.neg_s(0, 0);
    c.swc1(0, c.pa(V1, 0x288));
    c.g[RA][0] = sx(0x0012ffd8);
    c.g[A0][0] = c.ldw_2(S3, 0x14);
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
            c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
            if callee::C00115ab0.call(c, 0x0012c420).is_err() {
                return;
            }
            c.g[V0][0] = lw_7(c, at(c, S0, 0x14));
            c.lwc1(1, r32(c, GP).wrapping_sub(0x780c));
            c.swc1(0, at(c, V0, 0x2c8));
            sw(c, at(c, V0, 0x2cc), 0u64);
            c.swc1(0, at(c, V0, 0x2d0));
            c.lwc1(3, r32(c, GP).wrapping_sub(0x7808));
            // The launch's progress steps 0.015 per 60 Hz tick (not a 1/60
            // entry: the scan missed it). At 1.0 the rider goes back to
            // riding; 4x early at 240 Hz that is still in the air off the
            // gate, where a held stick starts a flip and the landing crashes.
            crate::game::ai::rider::per_tick_60(c, 3);
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
        c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 12);
        c.mtc1(0, 13);
        c.g[RA][0] = sx(0x0012c484);
        c.swc1(1, at(c, S0, 0x10));
        if callee::C00113f88.call(c, 0x0012c484).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x0012c48c);
        c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
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
                c.g[V1][0] = lw_7(c, at(c, S0, 0x14));
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
                c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
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
                    c.g[V0][0] = lw_7(c, at(c, S0, 0x14));
                    c.g[A1][0] = 0x6;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 12);
                    c.g[RA][0] = sx(0x0012c518);
                    c.g[A0][0] = lw_7(c, at(c, V0, 0x784));
                    if call_ptr(c, 0x003128e8, 0x0012c518).is_err() {
                        return;
                    }
                    c.g[RA][0] = sx(0x0012c520);
                    c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
                    if callee::C0011fe98.call(c, 0x0012c520).is_err() {
                        return;
                    }
                    c.g[V1][0] = 0x1;
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
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

/// `void RiderResetSub_Enter(RiderResetSub *self)` (0x12f230): clear the
/// reset flag and the animation's +0x90 word; for the local player, stop a
/// running transition and start a white fade from two 1.0-alpha effects.
pub fn rider_reset_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        c.g[A1][0] = 2;
        frame.save(c, 16, 0x70);
        frame.save(c, 20, 0x30);
        let this64 = abi::arg64(c, 0);
        c.g[S0][0] = this64;
        frame.save(c, 17, 0x60);
        let this = this64 as u32;
        c.g[S4][0] = sx(this.wrapping_add(4));
        frame.save(c, 18, 0x50);
        frame.save(c, 19, 0x40);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0x80);
        c.sw(this.wrapping_add(4), 0);
        c.g[V0][0] = c.lw(this.wrapping_add(8));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        callee::ANIM_GET_LAYER_SEQUENCE.call(c, 0x0012_f26c)?;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x90), 0);

        c.g[A0][0] = c.g[S0][0];
        callee::IS_LOCAL_PLAYER.call(c, 0x0012_f278)?;
        let not_local = c.g[V0][0] == 0;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if !not_local {
            c.cycles += 8;
            c.g[V0][0] = c.lw(this.wrapping_add(8));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x870));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            callee::GET_TRANSITION_MAN.call(c, 0x0012_f290)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x44));
            let idle = c.g[V0][0] == 0;
            if !idle {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                callee::TRANSITION_STOP.call(c, 0x0012_f2a8)?;
            }

            c.cycles += 51;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 20); // f20 = 1.0
            c.g[A0][0] = 0x14;
            callee::EFFECT_ALLOC.call(c, 0x0012_f2b8)?;
            c.g[S2][0] = c.g[V0][0];
            // The white colour {1, 1, 1, 1}.
            for off in [0u32, 4, 8, 0xc] {
                c.swc1(20, frame.at(c, off));
            }
            c.g[A0][0] = c.g[S2][0];
            callee::EFFECT_CTOR.call(c, 0x0012_f2d4)?;
            c.g[S0][0] = 0x0049_0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7f40));
            c.sw(c.g[S2][0] as u32, c.g[S0][0]);
            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(7));
            c.ldr(2, c.g[SP][0] as u32);
            c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
            c.ldr(3, (c.g[SP][0] as u32).wrapping_add(8));
            c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0xb));
            c.sdr(2, (c.g[S2][0] as u32).wrapping_add(4));
            c.sdl(3, (c.g[S2][0] as u32).wrapping_add(0x13));
            c.sdr(3, (c.g[S2][0] as u32).wrapping_add(0xc));

            c.g[A0][0] = 0x14;
            callee::EFFECT_ALLOC.call(c, 0x0012_f308)?;
            c.g[S1][0] = c.g[V0][0];
            for off in [0x1c, 0x10, 0x14, 0x18] {
                c.swc1(20, frame.at(c, off));
            }
            c.g[A0][0] = c.g[S1][0];
            callee::EFFECT_CTOR.call(c, 0x0012_f324)?;

            // The start call: 0.5 fade, 0.0 delay, 0.5 out.
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S3][0];
            c.sw(c.g[S1][0] as u32, c.g[S0][0]);
            c.g[A2][0] = c.g[S2][0];
            c.mtc1(0, 13);
            c.g[A3][0] = c.g[S1][0];
            c.g[T1][0] = c.g[S4][0];
            c.g[A1][0] = 1;
            c.mov_s(14, 12);
            c.ldl(10, (c.g[SP][0] as u32).wrapping_add(0x17));
            c.ldr(10, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.ldl(11, (c.g[SP][0] as u32).wrapping_add(0x1f));
            c.ldr(11, (c.g[SP][0] as u32).wrapping_add(0x18));
            c.sdl(10, (c.g[S1][0] as u32).wrapping_add(0xb));
            c.sdr(10, (c.g[S1][0] as u32).wrapping_add(4));
            c.sdl(11, (c.g[S1][0] as u32).wrapping_add(0x13));
            c.sdr(11, (c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[T0][0] = 0;
            callee::TRANSITION_START.call(c, 0x0012_f374)?;
        }

        c.cycles += 9;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::control::handplantcontrol::rider_air_state_load_state;
    pub(crate) use crate::game::ai::control::handplantcontrol::rider_ground_state_load_state;
    pub(crate) use crate::game::ai::control::handplantcontrol::rider_handplant_state_load_state;
    pub(crate) use crate::game::ai::control::handplantcontrol::rider_railslide_state_load_state;
    pub(crate) use crate::game::ai::motion::wipeoutmotion::rider_wipeout_state_load_state;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_5;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2 as slot_4;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_4;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::globals_3 as globals_6;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::pool_4;
    pub(crate) use crate::game::dirtysock::tags2::tests::belib_3 as belib;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_13 as globals_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_11 as pool_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_17 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_8 as WORLD_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_6;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2 as globals;
    pub(crate) use crate::game::fe::ovstatepause::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::ovstatepause::tests::WORLD;
    pub(crate) use crate::game::lib::kernel::tests::SEEDS;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::clear as clear_2;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;
    use ssx3_ee as ee;
    use ssx3_ee::s00;
    use ssx3_ee::s01;
    use ssx3_ee::s02;
    use ssx3_ee::s03;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn rider_reset_sub_is_local_player_matches() {
        check(
            ssx3_ee::s00::f_0012f588__generated,
            rider_reset_sub_is_local_player,
            300,
            &|c, seed| {
                let mut r = Rng(0x7171_7171 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let handle = DATA + 0x100;
                c.bus.write32(DATA + 0x8, handle);
                c.bus
                    .write32(handle + 0x87c, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(
                    handle + 0x870,
                    if seed % 5 == 0 {
                        0x8000_0000
                    } else {
                        r.next_u32() & 0x7fff_ffff
                    },
                );
            },
        );
    }

    #[test]
    fn rider_board_press_sub_update_phase2_matches() {
        check(
            ssx3_ee::s00::f_001304d0__generated,
            rider_board_press_sub_update_phase2,
            50,
            &|c, seed| {
                let mut r = Rng(0x8181_8181 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_board_press_sub_enter_matches() {
        check(
            ssx3_ee::s00::f_0012fc60,
            rider_board_press_sub_enter,
            200,
            &|c, seed| {
                let mut r = Rng(0x1001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x14, DATA + 0x400);
            },
        );
    }

    #[test]
    fn rider_cruise_sub_enter_matches() {
        check(
            ssx3_ee::s00::f_00131608,
            rider_cruise_sub_enter,
            200,
            &|c, seed| {
                let mut r = Rng(0x1002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, DATA + 0x400);
            },
        );
    }

    #[test]
    fn rider_railslide_sub_exit_matches() {
        check(
            ssx3_ee::s01::f_00132048,
            rider_railslide_sub_exit,
            200,
            &|c, seed| {
                let mut r = Rng(0x1003 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x7594), r.next_u32());
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x8, DATA + 0x400);
            },
        );
    }

    #[test]
    fn rider_handplant_sub_exit_matches() {
        check(
            ssx3_ee::s01::f_00132f98,
            rider_handplant_sub_exit,
            200,
            &|c, seed| {
                let mut r = Rng(0x1004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x754c), r.next_u32());
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x14, DATA + 0x400);
            },
        );
    }

    // -------------------------------------------------------- AI race state

    /// The dispatch table at 0x450000 + 0x6ca0, valid or pointed at an
    /// unknown address.
    pub(crate) fn race_state_table(c: &mut Cpu, seed: u32) {
        const VALID: [u32; 7] = [
            0x0011_3b74,
            0x0011_3b94,
            0x0011_3ba8,
            0x0011_3bbc,
            0x0011_3bd0,
            0x0011_3be4,
            0x0011_3bf8,
        ];
        for (i, v) in VALID.iter().enumerate() {
            let val = if seed % 3 == 0 { 0xdead_beef } else { *v };
            c.bus.write32(0x0045_0000 + 0x6ca0 + 4 * i as u32, val);
        }
    }

    #[test]
    fn ai_select_race_state_matches() {
        check(
            ssx3_ee::s00::f_00113b48,
            ai_select_race_state,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 73, 0);
                race_state_table(c, seed);
                c.pc = 0x0011_3b48;
                let ai = DATA + 0x400;
                c.bus.write32(ai, 1 + r.next_u32() % 8);
                c.bus.write32(ai + 0x98, r.next_u32());
                c.g[A0][0] = ai as u64;
                c.g[A1][0] = (seed % 10) as u64;
            },
        );
    }

    // ---------------------------------------------------- AI setup/teardown

    /// An AI object at `DATA + 0x400` with the rider counts zeroed (so the
    /// destroy/create/reset callees take their short paths), plus noise.
    pub(crate) fn ai_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        let ai = DATA + 0x400;
        fill(c, &mut r, ai + 0x70, 0x30);
        for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x94] {
            c.bus.write32(ai + off, 0);
        }
        c.bus.write32(ai + 0xa4, DATA + 0x1000);
        c.bus.write32(ai + 0xa8, DATA + 0x2000);
        c.g[A0][0] = ai as u64;
        r
    }

    #[test]
    fn ai_remove_actor_riders_matches() {
        check(
            ssx3_ee::s00::f_0012b000,
            ai_remove_actor_riders,
            SEEDS,
            &|c, seed| {
                ai_setup(c, seed, 77, 2);
            },
        );
    }

    #[test]
    fn ai_remove_computer_riders_matches() {
        check(
            ssx3_ee::s00::f_0012b030,
            ai_remove_computer_riders,
            SEEDS,
            &|c, seed| {
                ai_setup(c, seed, 78, 2);
            },
        );
    }

    #[test]
    fn ai_remove_mission_riders_matches() {
        check(
            ssx3_ee::s00::f_0012b060,
            ai_remove_mission_riders,
            SEEDS,
            &|c, seed| {
                ai_setup(c, seed, 79, 2);
            },
        );
    }

    #[test]
    fn ai_dtor_matches() {
        check(ssx3_ee::s00::f_001287b8, ai_dtor, SEEDS, &|c, seed| {
            let mut r = ai_setup(c, seed, 80, 3);
            c.g[A1][0] = (r.next_u32() & 1) as u64;
        });
    }

    /// Build `count` rider pointers at `ai + base_off`, each with the vtable
    /// chain the destructor walks; the destructor target is an unregistered
    /// address that lands in the fake-callee hook.
    pub(crate) fn ai_rider_entries(c: &mut Cpu, base_off: u32, count: u32) {
        let ai = DATA + 0x400;
        for i in 0..count {
            let rider = DATA + 0x2000 + i * 0x100;
            let vtobj = DATA + 0x8000 + i * 0x100;
            let vt = DATA + 0x9000;
            c.bus.write32(ai + base_off + 4 * i, rider);
            c.bus.write32(rider + 0x18, vtobj);
            c.bus.write32(vtobj + 0x6c0, vt);
            c.bus.write16(vt + 8, 0);
            c.bus.write32(vt + 0xc, 0x0000_5000);
        }
    }

    #[test]
    fn ai_destroy_computer_riders_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 81, 1);
            let ai = DATA + 0x400;
            let count = 1 + r.next_u32() % 8;
            c.bus.write32(ai + 0x80, count);
            c.bus.write32(ai + 0x78, count + (r.next_u32() % 4));
            c.bus.write32(ai + 0x14, r.next_u32());
            ai_rider_entries(c, 0x48, count);
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_ae38)
        };
        check_events(
            ssx3_ee::s00::f_0012ae38,
            ai_destroy_computer_riders,
            SEEDS,
            &setup,
        );
    }

    #[test]
    fn ai_destroy_actor_riders_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 82, 1);
            let ai = DATA + 0x400;
            let count = 1 + r.next_u32() % 6;
            c.bus.write32(ai + 0x84, count);
            c.bus.write32(ai + 0x78, count + (r.next_u32() % 4));
            c.bus.write32(ai + 0x8c, r.next_u32());
            ai_rider_entries(c, 0x5c, count);
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_aed0)
        };
        check_events(
            ssx3_ee::s00::f_0012aed0,
            ai_destroy_actor_riders,
            SEEDS,
            &setup,
        );
    }

    #[test]
    fn ai_destroy_mission_riders_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 83, 1);
            let ai = DATA + 0x400;
            let count = 1 + r.next_u32() % 4;
            c.bus.write32(ai + 0x88, count);
            c.bus.write32(ai + 0x78, count + (r.next_u32() % 4));
            c.bus.write32(ai + 0x90, r.next_u32());
            ai_rider_entries(c, 0x70, count);
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_af68)
        };
        check_events(
            ssx3_ee::s00::f_0012af68,
            ai_destroy_mission_riders,
            SEEDS,
            &setup,
        );
    }

    #[test]
    fn ai_destroy_all_riders_matches() {
        check(
            ssx3_ee::s00::f_0012b090,
            ai_destroy_all_riders,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 84, 4);
                let ai = DATA + 0x400;
                fill(c, &mut r, ai, 0x60);
                for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88] {
                    c.bus.write32(ai + off, 0);
                }
                c.bus.write32(DATA + 0x100, r.next_u32()); // free-anim slot 0 = null
                c.bus.write32(GAME_GP + 0xd08, 0);
                c.g[A0][0] = ai as u64;
            },
        );
    }

    #[test]
    fn ai_free_anim_sources_matches() {
        check(
            ssx3_ee::s00::f_0012b200,
            ai_free_anim_sources,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 85, 3);
                c.bus.write32(GAME_GP + 0xd08, 0);
                c.g[A0][0] = (DATA + 0x400) as u64;
            },
        );
    }

    #[test]
    fn ai_reset_riders_matches() {
        check(
            ssx3_ee::s00::f_001297c8,
            ai_reset_riders,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 87, 2);
                let ai = DATA + 0x400;
                fill(c, &mut r, ai, 0x100);
                c.bus.write32(ai + 0x78, 0);
                c.bus.write32(ai + 0x88, 0);
                c.g[A0][0] = ai as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn ai_restart_matches() {
        check(ssx3_ee::s00::f_00129768, ai_restart, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 88, 3);
            let ai = DATA + 0x400;
            fill(c, &mut r, ai, 0x100);
            c.bus.write32(ai + 0x78, 0);
            c.bus.write32(ai + 0x88, 0);
            // Focus and danger managers with zero counts / in-range fields.
            let focus = DATA + 0x3000;
            let danger = DATA + 0x2000;
            fill(c, &mut r, focus, 0x120);
            fill(c, &mut r, danger, 0xc00);
            c.bus.write32(focus + 0xd8, 0);
            c.bus.write32(ai + 0xa4, focus);
            c.bus.write32(ai + 0xa8, danger);
            c.g[A0][0] = ai as u64;
            c.g[A1][0] = (r.next_u32() % 10) as u64;
        });
    }

    #[test]
    fn ai_refresh_rider_overlaps_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 89, 1);
            let ai = DATA + 0x400;
            let count = r.next_u32() % 4;
            c.bus.write32(ai + 0x78, count);
            for i in 0..count {
                let rider = DATA + 0x2000 + i * 0x100;
                c.bus.write32(ai + 0x28 + 4 * i, rider);
            }
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_b788)
        };
        check_events(
            ssx3_ee::s00::f_0012b788,
            ai_refresh_rider_overlaps,
            SEEDS,
            &setup,
        );
    }

    /// The song instance / process / block a wrapper loads from its object.
    pub(crate) const CHILD: u32 = DATA + 0x1000;

    #[test]
    fn ai_set_sync_words_matches() {
        check(
            s00::f_0012bac0__generated,
            ai_set_sync_words,
            200,
            &|c, seed| {
                let mut r = Rng(0x12ba_c0aa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(CHILD);
                clear(c, OBJ, 0x40);
                c.bus.write32(CHILD, r.next_u32());
                c.bus.write32(CHILD + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn ai_get_sync_words_matches() {
        check(
            s00::f_0012baf0__generated,
            ai_get_sync_words,
            200,
            &|c, seed| {
                let mut r = Rng(0x12ba_f0aa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(CHILD);
                clear(c, OBJ, 0x40);
                clear(c, CHILD, 0x40);
                c.bus.write32(OBJ + 0x20, r.next_u32());
                c.bus.write32(OBJ + 0x24, r.next_u32());
            },
        );
    }

    pub(crate) const SEEDS_2: u32 = 40;

    /// The lazily built BELibrary singleton (`gp - 0x1f04`): a zeroed dummy so
    /// the file/allocator paths fail fast instead of waiting on the disc.
    pub(crate) fn be_library(c: &mut Cpu) {
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    /// The shared interface-forwarder fixture: `a1` is an object whose first
    /// word is a vtable built by [`fakes::vtable`] (slot 0x10 -> a `call_ptr`
    /// target in empty RAM, so it lands in the fake hook), `a0` the receiver.
    pub(crate) fn virtual_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        let mut r = machine(c, seed, salt, 2);
        be_library(c);
        let obj = DATA + 0x800;
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(obj);
        let vt = fakes::vtable(c, &mut r, &[slot]);
        c.bus.write32(obj, vt);
    }

    #[test]
    fn ai_reset_riders_and_order_matches() {
        check(
            ee::s00::f_00128a10__generated,
            ai_reset_riders_and_order,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x128a_10, 2);
                be_library(c);
                // No riders (+0x78) and no missions (+0x88): both reset walks skip.
                c.g[A0][0] = sx(DATA + 0x400);
            },
        );
    }

    macro_rules! load_state_tests {
        ($(($test:ident, $func:ident, $shard:ident, $addr:ident, $salt:expr)),* $(,)?) => {
            $(
                #[test]
                fn $test() {
                    check(ee::$shard::$addr, $func, SEEDS_2, &|c, seed| {
                        virtual_setup(c, seed, $salt, 0x10);
                    });
                }
            )*
        };
    }
    pub(crate) use load_state_tests;

    load_state_tests! {
        (rider_gate_sub_load_state_matches, rider_gate_sub_load_state, s00, f_0012c590, 0x12c5_90),
        (rider_finish_line_sub_load_state_matches, rider_finish_line_sub_load_state, s00, f_0012c9f8, 0x12c9_f8),
        (rider_wipeout_sub_load_state_matches, rider_wipeout_sub_load_state, s00, f_0012e740, 0x12e7_40),
        (rider_reset_sub_load_state_matches, rider_reset_sub_load_state, s00, f_0012f5e8, 0x12f5_e8),
        (rider_natural_air_sub_load_state_matches, rider_natural_air_sub_load_state, s00, f_0012fc28, 0x12fc_28),
        (rider_board_press_sub_load_state_matches, rider_board_press_sub_load_state, s00, f_001315d0, 0x1315_d0),
        (rider_railslide_sub_load_state_matches, rider_railslide_sub_load_state, s01, f_00132878, 0x1328_78),
        (rider_handplant_sub_load_state_matches, rider_handplant_sub_load_state, s01, f_001330f0, 0x1330_f0),
        (rider_spin_sub_load_state_matches, rider_spin_sub_load_state, s01, f_00136230, 0x1362_30),
        (rider_uber_grind_sub_load_state_matches, rider_uber_grind_sub_load_state, s01, f_00136920, 0x1369_20),
        (rider_wipeout_state_load_state_matches, rider_wipeout_state_load_state, s01, f_00138b10, 0x138b_10),
        (rider_handplant_state_load_state_matches, rider_handplant_state_load_state, s01, f_001399a8, 0x1399_a8),
        (rider_air_state_load_state_matches, rider_air_state_load_state, s01, f_0013ace8, 0x13ac_e8),
        (rider_railslide_state_load_state_matches, rider_railslide_state_load_state, s01, f_0013c618, 0x13c6_18),
        (rider_ground_state_load_state_matches, rider_ground_state_load_state, s01, f_0013f880, 0x13f8_80),
    }

    // -----------------------------------------------------------------------
    // AI_PathDataInitIfEarlySection.

    #[test]
    fn ai_path_data_init_if_early_section_matches() {
        check(
            ssx3_ee::s00::f_0012a490__generated,
            ai_path_data_init_if_early_section,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12a4_90, 0);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x1000;
                let manager = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, manager);
                // The logical-section table at 0x442168 (all zero in a fresh
                // machine): entry 0 matches a zero id, nothing matches a
                // non-zero one (the search then reports "not found", 0x32).
                let section = if seed % 2 == 0 { 0 } else { 0x9999 };
                c.bus.write32(manager + 0x78, section);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
            },
        );
    }

    // ------------------------------------------------- rider metrics and AI

    #[test]
    fn rider_metrix_reset_stats_matches() {
        check(
            ssx3_ee::s00::f_001175b8__generated,
            rider_metrix_reset_stats,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x05, 1);
                c.g[A0][0] = sx(DATA);
                put_words(
                    c,
                    DATA + 0x1c8,
                    &[if seed % 2 == 0 { 0 } else { r.next_u32() }],
                );
                put_words(c, DATA + 0xfc, &[r.next_u32(), r.next_u32()]);
            },
        );
    }

    #[test]
    fn ai_setup_computer_riders_matches() {
        check(
            ssx3_ee::s00::f_00128958__generated,
            ai_setup_computer_riders_2,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x08, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // No previous computer riders when `+0x88` is not positive.
                c.bus.write32(
                    DATA + 0x88,
                    if seed % 2 == 0 {
                        0
                    } else {
                        r.next_u32() | 0x8000_0000
                    },
                );
                c.bus.write32(DATA + 0x80, 0);
                c.bus.write32(DATA + 0x78, 0);
            },
        );
    }

    /// The three spin-sub predicates share a fixture: either the sub-state
    /// does not match (early 0) or the animation entry is read through the
    /// controller (`+0x58` → `+0x784`).
    pub(crate) fn spin_sub_test(generated: GuestFn, hand: GuestFn, kind: u32, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            c.g[A0][0] = sx(DATA);
            let matching = seed % 2 == 0;
            let sub = if matching { kind } else { kind ^ 1 };
            c.bus.write32(DATA + 4, sub);
            c.bus.write32(DATA + 0x58, DATA + 0x2000);
            // The controller's animation table pointer, then the entry slot.
            c.bus.write32(DATA + 0x2784, DATA + 0x3000);
            c.bus.write32(DATA + 0x3008, 0x1b6);
        });
    }

    #[test]
    fn rider_spin_sub_is_trick_anim12_matches() {
        spin_sub_test(
            ssx3_ee::s01::f_00135eb0__generated,
            rider_spin_sub_is_trick_anim12,
            2,
            0x09,
        );
    }

    #[test]
    fn rider_spin_sub_is_trick_anim13_matches() {
        spin_sub_test(
            ssx3_ee::s01::f_00135ef0__generated,
            rider_spin_sub_is_trick_anim13,
            4,
            0x0a,
        );
    }

    #[test]
    fn rider_spin_sub_is_trick_anim14_matches() {
        spin_sub_test(
            ssx3_ee::s01::f_00135f30__generated,
            rider_spin_sub_is_trick_anim14,
            4,
            0x0b,
        );
    }

    #[test]
    fn rider_gate_sub_can_start_matches() {
        check(
            ee::s00::f_0012c078__generated,
            rider_gate_sub_can_start,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_c078, 0);
                let world = DATA + 0x4000;
                let mgr = DATA + 0x5000;
                let race = DATA + 0x6000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, mgr);
                c.bus.write32(mgr + 0xc, race);
                c.bus
                    .write32(race, if seed % 3 == 0 { 5 } else { r.next_u32() % 8 });
                let gate = DATA + 0x1000;
                let timer = DATA + 0x2000;
                c.bus.write32(gate + 0x14, timer);
                c.bus.write32(
                    timer + 0x470,
                    match seed % 4 {
                        0 => 0xbf80_0000,
                        1 => 0,
                        _ => r.next_u32(),
                    },
                );
                c.g[A0][0] = sx(gate);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    // ----------------------------------------------------------- rider gate

    #[test]
    fn ridergate_sub_is_start_time_reached_matches() {
        check(
            ee::s00::f_0012c028__generated,
            ridergate_sub_is_start_time_reached,
            80,
            &|c, seed| {
                let mut r = Rng(0x12c0_2844 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let (module, world, race) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                let gate = DATA + 0x4000;
                let rider = DATA + 0x5000;
                c.bus.write32(gp_at(-0x848), module);
                c.bus.write32(module + 0x84, world);
                c.bus.write32(world + 0xc, race);
                c.bus
                    .write32(race, if seed % 3 == 0 { 5 } else { seed % 7 });
                c.bus.write32(race + 0xc, r.next_u32() % 100_000);
                c.bus.write32(gp_at(-0x7840), 0.016_666_668f32.to_bits());
                c.bus.write32(gate + 0x14, rider);
                c.bus.write32(rider + 0xb30, r.next_u32() % 2000);
                c.g[A0][0] = sx(gate);
            },
        );
    }

    /// The BELibrary singleton (gp-0x1f04) already exists; its interface slot
    /// 0 (`gp-0x1efc`) is an object whose vtable slot at `+8` is a fake.
    pub(crate) fn belibrary(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x1000);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        let vt = vtable(c, r, &[8]);
        c.bus.write32(0x0053_08a8, DATA + 0x1100);
        c.bus.write32(DATA + 0x1104, vt);
    }

    #[test]
    fn ai_setup_actor_riders_matches() {
        check(
            ssx3_ee::s00::f_00128998__generated,
            ai_setup_actor_riders,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x128998, 2);
                belibrary(c, &mut r);
                c.bus.write8(0x0053_5c11, (seed % 2) as u8);
                let ai = DATA;
                // No new riders to spawn: `AI_CreateActorRiders` breaks at once.
                c.bus.write32(ai + 0x84, 0x10);
                c.bus.write32(ai + 0x94, 0x10);
                c.bus.write32(ai + 0x78, 0);
                c.g[A0][0] = ai as u64;
                c.g[A1][0] = 0x10;
            },
        );
    }

    #[test]
    fn rider_board_press_sub_cancel_matches() {
        check(
            ee::s00::f_00131348__generated,
            rider_board_press_sub_cancel,
            SEEDS_3,
            &|c, seed| {
                let mut r = common(c, seed, 0x131348, 4);
                let this = DATA + 0x100;
                let rider = DATA + 0x2000;
                clear_2(c, this, 0x100);
                clear_2(c, rider, 0x800 / 4);
                c.bus.write32(this + 0x14, rider);
                c.bus.write32(rider + 0x330, (seed % 4) as u32);
                c.bus.write32(rider + 0x790, DATA + 0x3000);
                c.g[A0][0] = sx(this);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn rider_spin_sub_get_spin_hud_blend_matches() {
        check(
            ee::s01::f_00136100__generated,
            rider_spin_sub_get_spin_hud_blend,
            200,
            &|c, seed| {
                let mut r = Rng(0x1361_00d1 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x18, r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x7304), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x7300), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x72fc), r.edge_f32());
            },
        );
    }

    pub(crate) fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 8) ^ salt)
    }

    /// Scrambled registers, game `gp`, a zeroed data area, and four pointers
    /// into it. Enough for the pure leaves (loads/stores within `DATA`).
    pub(crate) fn leaf_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = rng(seed, salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        for i in 0..0x400 {
            c.bus.write32(DATA + i * 4, r.next_u32());
        }
        for i in 0..4 {
            c.g[[A0, A1, A2, A3][i]][0] = (DATA + (r.next_u32() & 0x3f0)) as u64;
        }
    }

    pub(crate) fn leaf(gen: GuestFn, hand: GuestFn, salt: u64) {
        check(gen, hand, SEEDS, &|c, seed| leaf_setup(c, seed, salt));
    }

    /// A test machine with fake call/jump-out/syscall hooks.
    pub(crate) fn fake_check(
        gen: GuestFn,
        hand: GuestFn,
        salt: u64,
        setup: &dyn Fn(&mut Cpu, u32, &mut Rng),
    ) {
        check(gen, hand, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            setup(c, seed, &mut r);
        });
    }

    /// [`check`] plus soft state and LO/HI: the replacement reproduces every
    /// register write of the generated body, so the caller-saved temporaries
    /// and the multiply pipeline must match too.
    pub(crate) fn strict_check(
        gen: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(gen, hand, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard {:#?} soft {:#?} lohi {:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    #[test]
    fn rider_prewind_sub_exit_matches() {
        leaf(
            ssx3_ee::s00::f_0012e9b0__generated,
            rider_prewind_sub_exit,
            12,
        );
    }

    // ------------------------------------------ RiderWipeoutSub anim selectors

    pub(crate) fn anim_object(c: &mut Cpu, id: u32) {
        c.g[SP][0] = (STACK - 0x100) as u64;
        c.bus.write32(DATA + 0x80, DATA + 0x200);
        c.bus.write32(DATA + 0x984, DATA + 0x300);
        c.bus.write32(DATA + 0x308, id);
        c.bus.write32(DATA, 0);
    }

    #[test]
    fn rider_wipeout_sub_play_end_anim_a_matches() {
        fake_check(
            ssx3_ee::s00::f_0012df48__generated,
            rider_wipeout_sub_play_end_anim_a,
            48,
            &|c, seed, _r| {
                let arms = [
                    0x0012_df98,
                    0x0012_dfa0,
                    0x0012_dfa8,
                    0x0012_dfb0,
                    0x0012_dfb8,
                    0x0012_dfc0,
                    0x0012_dfc8,
                    0x0012_dfd0,
                    0x0012_dfd8,
                    0x0012_dfe0,
                ];
                let ids = [
                    0x185, 0x186, 0x187, 0x188, 0x189, 0x18a, 0x18b, 0x18c, 0x18d, 0x18e,
                ];
                match seed % 3 {
                    0 => {
                        // In range; the (empty) end-anim table sends it to jump_out.
                        anim_object(c, 0x16a);
                        c.g[A0][0] = DATA as u64;
                    }
                    1 => {
                        // Out of table range and equal ids: straight to epilogue.
                        anim_object(c, 0);
                        c.g[A0][0] = DATA as u64;
                    }
                    _ => {
                        // Resume at a case block with the same id already playing.
                        let k = (seed as usize / 3) % arms.len();
                        c.pc = arms[k];
                        c.g[A2][0] = ids[k];
                    }
                }
            },
        );
    }

    #[test]
    fn rider_wipeout_sub_select_tumble_anim_matches() {
        fake_check(
            ssx3_ee::s00::f_0012da88__generated,
            rider_wipeout_sub_select_tumble_anim,
            49,
            &|c, seed, _r| {
                let arm = [
                    0x0012_dae8,
                    0x0012_daf4,
                    0x0012_db00,
                    0x0012_db0c,
                    0x0012_db18,
                    0x0012_db24,
                    0x0012_db30,
                    0x0012_db3c,
                    0x0012_db48,
                    0x0012_db54,
                    0x0012_db60,
                    0x0012_db6c,
                    0x0012_db78,
                    0x0012_db84,
                    0x0012_db90,
                    0x0012_db9c,
                    0x0012_dba8,
                    0x0012_dbb4,
                    0x0012_dbc0,
                    0x0012_dbcc,
                    0x0012_dbd8,
                    0x0012_dbe4,
                    0x0012_dbf0,
                    0x0012_dbfc,
                    0x0012_dc08,
                    0x0012_dc14,
                    0x0012_dc20,
                    0x0012_dc2c,
                    0x0012_dc38,
                    0x0012_dc44,
                    0x0012_dc50,
                    0x0012_dc5c,
                    0x0012_dc68,
                ];
                let a1 = [
                    0x16a, 0x16b, 0x16c, 0x170, 0x16e, 0x16f, 0x172, 0x173, 0x16d, 0x171, 0x16a,
                    0x16b, 0x16c, 0x170, 0x16e, 0x173, 0x171, 0x16d, 0x16f, 0x172, 0x170, 0x171,
                    0x16c, 0x16e, 0x173, 0x170, 0x170, 0x172, 0x16b, 0x16a, 0x170, 0x172, 0x174,
                ];
                match seed % 3 {
                    0 => {
                        // Anim id 0 has a non-empty alternate column: dispatch to
                        // the (empty) table -> jump_out.
                        anim_object(c, 0x148);
                        c.g[A0][0] = DATA as u64;
                    }
                    1 => {
                        // Out of table range and equal ids: straight to epilogue.
                        anim_object(c, 0);
                        c.g[A0][0] = DATA as u64;
                    }
                    _ => {
                        let k = (seed as usize / 3) % arm.len();
                        c.pc = arm[k];
                        c.g[S0][0] = 0; // keep the primary id
                        c.g[A2][0] = a1[k];
                    }
                }
            },
        );
    }

    #[test]
    fn rider_wipeout_sub_switch_to_anim_set_a_matches() {
        strict_check(
            ssx3_ee::s00::f_0012dd98__generated,
            rider_wipeout_sub_switch_to_anim_set_a,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 51, 1);
                c.pc = 0x0012_dd98;
                // `w + 0x80` -> anim player, its `+0x784` the layer controller and
                // its `+0x150` the "this animation is already playing" flag.
                c.bus
                    .write32(DATA + 0x350, if seed % 3 == 0 { 1 } else { 0 });
                let id = match seed % 4 {
                    0 => 0x174,
                    1 => 0x175 + (seed as u32 % 0xf),
                    2 => 0x148,
                    _ => r.next_u32() % 0x200,
                };
                anim_object(c, id);
                c.g[A0][0] = sx(DATA);
                // The animation-id table the dispatch reads (`.rodata 0x4580a0`):
                // ten id blocks, then the "keep the id" entry for 0x17f and the
                // duplicate blocks for 0x180..0x183.
                put_words(
                    c,
                    0x0045_80a0,
                    &[
                        0x0012_ddf8,
                        0x0012_de00,
                        0x0012_de08,
                        0x0012_de10,
                        0x0012_de18,
                        0x0012_de20,
                        0x0012_de28,
                        0x0012_de30,
                        0x0012_de38,
                        0x0012_de40,
                        0x0012_de4c,
                        0x0012_de28,
                        0x0012_de30,
                        0x0012_de38,
                        0x0012_de40,
                    ],
                );
                // A restart calls AnimController_PlayAnim, whose first act is the
                // layer getter through the controller's vtable (a fake). Unwinding
                // there keeps the animation cache and the DLHeap out of the test,
                // so a free/consolidate can never run on fixture data.
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(DATA + 0x300 + 0x58, vt);
                c.bus.write32(LOG_UNWIND_AT, 0);
            },
        );
    }

    #[test]
    fn ai_load_next_rider_matches() {
        check(
            ee::s00::f_001296f8__generated,
            ai_load_next_rider,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x0012_96f8, 3);
                c.g[A0][0] = OBJ as u64;
                let count = seed % 4;
                w(c, OBJ + 0x78, count);
                for i in 0..count {
                    let rider = DATA + 0x3000 + 0x900 * i;
                    w(c, OBJ + 0x28 + 4 * i, rider);
                    let loaded = if seed % 2 == 0 { 7 } else { 0 };
                    w(c, rider + 0x880, loaded);
                }
            },
        );
    }

    /// `ScriptList_GetFieldB0`: returns `*(a0 + 0xb0)`; used to hand a vtable
    /// method a controlled non-zero result cheaply.
    pub(crate) const LOAD_B0: u32 = 0x0027_72b8;

    pub(crate) fn w16(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }

    /// `{ short delta; short; void (*fn)(); }` at `vt + off`.
    pub(crate) fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        w16(c, vt + off, delta as u16);
        w32(c, vt + off + 4, func);
    }

    // ------------------------------------------------------------------
    // Wave 36, batch 2.

    #[test]
    fn rider_gate_sub_update_wait_push_matches() {
        check(
            ssx3_ee::s00::f_0012c0c0__generated,
            rider_gate_sub_update_wait_push,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12c0c0, 0);
                let g = DATA + 0x1000;
                let bits = DATA + 0x2000;
                c.g[A0][0] = g as u64;
                c.g[A1][0] = bits as u64;
                w32(c, GAME_GP.wrapping_sub(0x783c), 0.032_258_064f32.to_bits());
                let signed = (seed as i32 % 17) - 8;
                let raw = ((signed as u32) & 0x3f) << 12 | (r.next_u32() & 0xfff);
                w32(c, bits, raw);
                let model = DATA + 0x3000;
                let anim = DATA + 0x4000;
                w32(c, g + 0x14, model);
                w32(c, model + 0x784, anim);
                let vt = DATA + 0x5000;
                w32(c, anim + 0x58, vt);
                slot(c, vt, 0x20, 0, LOAD_B0);
                w32(c, anim + 0xb0, 0x207);
            },
        );
    }

    #[test]
    fn ai_computer_riders_have_render_slots_matches() {
        check(
            ee::s00::f_0012abd0__generated,
            ai_computer_riders_have_render_slots,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x12_abd0, 3);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                let n = (seed % 4) as u32;
                w(c, DATA + 0x2000 + 0x80, n);
                for i in 0..4u32 {
                    let rider = DATA + 0x2400 + 0x40 * i;
                    w(c, DATA + 0x2000 + 0x48 + 4 * i, rider);
                    w(c, rider + 0x18, DATA + 0x2600 + 0x40 * i);
                    w(
                        c,
                        DATA + 0x2600 + 0x40 * i + 0x4,
                        if i % 2 == 0 { 1 } else { 0 },
                    );
                }
                c.pc = 0x0012_abd0;
            },
        );
    }

    #[test]
    fn ai_actor_riders_have_render_slots_matches() {
        check(
            ee::s00::f_0012acf8__generated,
            ai_actor_riders_have_render_slots,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x12_acf8, 3);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                let n = (seed % 4) as u32;
                w(c, DATA + 0x2000 + 0x84, n);
                for i in 0..4u32 {
                    let rider = DATA + 0x2400 + 0x40 * i;
                    w(c, DATA + 0x2000 + 0x5c + 4 * i, rider);
                    w(c, rider + 0x18, DATA + 0x2600 + 0x40 * i);
                    w(
                        c,
                        DATA + 0x2600 + 0x40 * i + 0x4,
                        if i % 2 == 0 { 1 } else { 0 },
                    );
                }
                c.pc = 0x0012_acf8;
            },
        );
    }

    #[test]
    fn ai_get_sync_checksum_matches() {
        check(
            ee::s00::f_0012b420__generated,
            ai_get_sync_checksum,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x12b420, 0);
                be_library_2(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                let n = seed % 4;
                w(c, obj + 0x78, n);
                for i in 0..n {
                    w(c, obj + 0x28 + 4 * i, DATA + 0x1000 + 0x40 * i);
                    w(c, DATA + 0x1000 + 0x40 * i + 4, seed + i);
                }
            },
        );
    }

    #[test]
    fn ai_reset_rider_stats_matches() {
        check(
            ee::s00::f_0012b180__generated,
            ai_reset_rider_stats,
            24,
            &|c, seed| {
                let _ = base_2(c, seed, 0x12b1_80, 2);
                let self_ = DATA;
                let n = (seed % 4) as u32;
                w(c, self_ + 0x78, n);
                for i in 0..n {
                    let rider = DATA + 0x1000 + 0x100 * i;
                    w(c, self_ + 0x28 + 4 * i, rider);
                    w(c, rider + 0x790, DATA + 0x2000 + 0x100 * i);
                }
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    #[test]
    fn rider_board_press_sub_settle_slew_matches() {
        use crate::difftest::put_words;
        check(
            ee::s00::f_001313a8__generated,
            rider_board_press_sub_settle_slew,
            24,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1313_a8, 0);
                let st = DATA + 0x1000;
                w(c, DATA + 0x14, st);
                let slew = match seed % 4 {
                    0 => (-1.0f32).to_bits(),
                    1 => 0.0f32.to_bits(),
                    2 => 1.0f32.to_bits(),
                    _ => r.edge_f32(),
                };
                w(c, st + 0x280, slew);
                w(c, st + 0x300, r.edge_f32());
                put_words(
                    c,
                    GAME_GP.wrapping_sub(0x7600),
                    &[r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// `RiderBoardPressSub_SettleSlew` writes the slew's `maxStep` from the
    /// rider's time scale and 0.025 **per tick**: at 120/240 Hz the step is
    /// halved/quartered, so the board press settles in the same real time.
    #[test]
    fn rider_board_press_sub_settle_slew_follows_the_rate() {
        use crate::tick;
        const STATE: u32 = 0x0080_0000;
        const RIDER: u32 = 0x0080_1000;
        // The game's 0.025 at the three sites (gp-0x7600/-0x75fc/-0x75f8).
        const STEP_60: u32 = 0x3ccc_ccce;
        let step = |hz: u32, amount: f32| -> (u32, u32) {
            let mut c = crate::difftest::fresh();
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = STATE as u64;
            c.bus.write32(STATE + 0x14, RIDER);
            c.bus.write32(RIDER + 0x280, amount.to_bits());
            c.bus.write32(RIDER + 0x300, 1.0f32.to_bits()); // time scale
            for off in [0x7600u32, 0x75fc, 0x75f8] {
                c.bus.write32(GAME_GP.wrapping_sub(off), STEP_60);
            }
            tick::set_for_test(&mut c, hz);
            rider_board_press_sub_settle_slew(&mut c);
            (c.bus.read32(RIDER + 0x284), c.bus.read32(RIDER + 0x288))
        };
        // 60 Hz: the step as stored, and the snap direction of each branch.
        assert_eq!(step(60, 0.2), (STEP_60, 0), "below");
        assert_eq!(step(60, 0.8), (STEP_60, 0x3f80_0000), "above");
        // Below -0.5 (the clamp) it snaps down; exactly -0.5 takes the
        // `|slew| < 0.5` branch and leaves the snap direction at 0.
        assert_eq!(step(60, -0.5), (STEP_60, 0), "at the clamp");
        assert_eq!(step(60, -0.6), (STEP_60, 0xbf80_0000), "negative");
        for (hz, want) in [(120u32, 0x3c4c_ccceu32), (240, 0x3bcc_ccce)] {
            for amount in [0.2f32, 0.8, -0.6] {
                let (s, _) = step(hz, amount);
                assert_eq!(s, want, "{hz} Hz, amount {amount}");
            }
        }
    }

    #[test]
    fn ai_update_rider_poses_matches() {
        check(
            ee::s00::f_00129160__generated,
            ai_update_rider_poses,
            40,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1291, 3);
                let wld = DATA;
                let n = (seed % 3) as u32;
                w(c, wld + 0x78, n);
                for i in 0..3u32 {
                    w(
                        c,
                        wld + 0x28 + 4 * i,
                        if i < n { DATA + 0x1000 + 0x800 * i } else { 0 },
                    );
                }
                fill(c, &mut r, DATA + 0x1000, 0x40);
                fill(c, &mut r, DATA + 0x1800, 0x40);
                c.g[A0][0] = sx(wld);
            },
        );
    }

    pub(crate) const RIDER: u32 = DATA + 0x14000;

    pub(crate) fn wf(c: &mut Cpu, a: u32, f: f32) {
        c.bus.write32(a, f.to_bits());
    }

    // ---------------------------------------------------------------- rider

    #[test]
    fn rider_railslide_sub_set_rail_matches() {
        check(
            ee::s01::f_00132620__generated,
            rider_railslide_sub_set_rail,
            10,
            &|c, seed| {
                let _ = base(c, seed, 0x132620, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => u64::MAX,
                    1 => 7,
                    _ => (seed % 5) as u64,
                };
                w(c, OBJ + 8, RIDER);
                w(c, OBJ + 4, (seed % 4) as u32);
                wf(c, RIDER + 0x2f0, if seed % 2 == 0 { 0.0 } else { 1.5 });
                w(c, RIDER + 0xb2c, if seed % 3 == 0 { 2 } else { 0 });
                w(c, RIDER + 0x77c, OBJ2);
            },
        );
    }

    pub(crate) fn wf_2(c: &mut Cpu, a: u32, v: f32) {
        c.bus.write32(a, v.to_bits());
    }

    // ------------------------------------------------------ rider finish line

    #[test]
    fn rider_finish_line_sub_enter_matches() {
        check(
            ee::s00::f_0012c5c8__generated,
            rider_finish_line_sub_enter,
            SEEDS_6,
            &|c, seed| {
                let _ = base_4(c, seed, 0x12c5_c8, 4);
                let obj = SCRATCH;
                let rider = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 4, rider);
                w(c, rider + 0x790, rider + 0x2000);
                // The finish-line normal and the two test values.
                for i in 0..4u32 {
                    wf_2(
                        c,
                        rider + 0x1e0 + 4 * i,
                        match i {
                            0 => 0.0,
                            1 => 0.0,
                            2 => 1.0,
                            _ => 0.0,
                        },
                    );
                }
                wf_2(c, rider + 0x2dc, if seed % 2 == 0 { 0.0 } else { 0.5 });
                wf_2(c, rider + 0x378, if seed % 3 == 0 { 0.0 } else { 1.0 });
                wf_2(
                    c,
                    GAME_GP.wrapping_add(0x1b9c),
                    if seed % 2 == 0 { 0.0 } else { 2.0 },
                );
                wf_2(
                    c,
                    GAME_GP.wrapping_sub(0x77fc),
                    if seed % 3 == 0 { 5.0 } else { 0.0 },
                );
                w(c, obj, 0);
            },
        );
    }

    // --------------------------------------------------- rider wipeout phase

    #[test]
    fn rider_wipeout_sub_update_phase4_matches() {
        check(
            ee::s00::f_0012d9d8__generated,
            rider_wipeout_sub_update_phase4,
            SEEDS_6,
            &|c, seed| {
                let _ = base_4(c, seed, 0x12d9_d8, 6);
                let obj = SCRATCH;
                let rider = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x80, rider);
                w(c, rider + 0x784, rider + 0x2000);
                wf_2(
                    c,
                    obj + 0x70,
                    match seed % 4 {
                        0 => 0.0,
                        1 => 2.0,
                        _ => 1.0,
                    },
                );
                wf_2(c, rider + 0x470, if seed % 5 == 0 { 2.0 } else { 0.0 });
                w(c, obj + 0x40, 0);
                w(c, obj + 0xc, 0);
                w(c, obj, 0);
            },
        );
    }

    #[test]
    fn ai_reset_computer_render_slots_matches() {
        check(
            ee::s00::f_0012ab20__generated,
            ai_reset_computer_render_slots,
            60,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x12ab);
                c.g[A0][0] = sx(DATA);
                let n = (seed % 3) as u32;
                c.bus.write32(DATA + 0x80, n);
                c.bus.write32(gp_at(-0x854), DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x10d8, DATA + 0x4000);
                slot_4(c, DATA + 0x4000, 0x390, 1);
                for i in 0..3u32 {
                    let rider = DATA + 0x2000 + 0x100 * i;
                    c.bus.write32(DATA + 0x48 + 4 * i, rider);
                    c.bus.write32(rider + 0x18, DATA + 0x1000 + 0x100 * i);
                    c.bus
                        .write32(DATA + 0x1000 + 0x100 * i + 0x884, (i + 1) as u32);
                }
                fill(c, &mut r, DATA + 0x1000, 0x300);
            },
        );
    }

    #[test]
    fn ai_reset_actor_render_slots_matches() {
        check(
            ee::s00::f_0012ac48__generated,
            ai_reset_actor_render_slots,
            60,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x12ac);
                c.g[A0][0] = sx(DATA);
                let n = (seed % 3) as u32;
                c.bus.write32(DATA + 0x84, n);
                c.bus.write32(gp_at(-0x854), DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x10d8, DATA + 0x4000);
                slot_4(c, DATA + 0x4000, 0x390, 1);
                for i in 0..3u32 {
                    let rider = DATA + 0x2000 + 0x100 * i;
                    c.bus.write32(DATA + 0x5c + 4 * i, rider);
                    c.bus.write32(rider + 0x18, DATA + 0x1000 + 0x100 * i);
                    c.bus
                        .write32(DATA + 0x1000 + 0x100 * i + 0x884, (i + 1) as u32);
                }
                fill(c, &mut r, DATA + 0x1000, 0x300);
            },
        );
    }

    /// A float bit pattern in [-range, range).
    pub(crate) fn uf(r: &mut Rng, range: f32) -> u32 {
        r.f32(range)
    }

    #[test]
    fn ai_force_finish_riders_matches() {
        diff(
            ee::s00::f_0012b340__generated,
            ai_force_finish_riders,
            40,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x12_b340);
                c.g[A0][0] = sx(DATA);
                w(c, DATA, if seed % 5 == 0 { r.next_u32() } else { 5 });
                w(c, DATA + 0x7c, 2);
                for i in 0..2u32 {
                    w(c, DATA + 0x40 + 4 * i, DATA + 0x1000 + 0x100 * i);
                }
                for i in 0..2u32 {
                    let rider = DATA + 0x1000 + 0x100 * i;
                    let r_ = DATA + 0x2000 + 0x100 * i;
                    w(c, rider + 0x18, r_);
                    w(c, r_ + 0x87c, if seed % 2 == 0 { 0 } else { 1 });
                    w(
                        c,
                        r_ + 0x470,
                        if i == 0 {
                            0.0f32.to_bits()
                        } else {
                            uf(&mut r, 100.0)
                        },
                    );
                    w(c, r_ + 0x480, 0);
                }
                w(c, GAME_GP.wrapping_sub(0x204), DATA + 0x3000);
            },
        );
    }

    /// Every compared state must match: hard, caller-saved (`soft`) and LO/HI.
    pub(crate) fn strict(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(g, h, &|c| setup(c, seed));
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
    fn ai_apply_computer_rider_settings_matches() {
        strict(
            ee::s00::f_0012b698__generated,
            ai_apply_computer_rider_settings,
            SEEDS,
            &|c, seed| {
                let mut r = base_6(c, seed, 0xb698, 0);
                let rider = DATA + 0x100;
                let state = DATA + 0x2000;
                w(c, rider + 0x18, state);
                w(
                    c,
                    0x0053_08d0,
                    if seed % 3 == 0 {
                        8
                    } else if seed % 3 == 1 {
                        1
                    } else {
                        r.next_u32() & !9
                    },
                );
                w(c, 0x0053_08b8, r.next_u32());
                w(c, 0x0053_08d4, r.next_u32() % 5);
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(rider);
            },
        );
    }

    #[test]
    fn rider_gate_sub_update_push_matches() {
        strict(
            ssx3_ee::s00::f_0012c130__generated,
            rider_gate_sub_update_push,
            SEEDS_7,
            &|c, seed| {
                let mut r = base_7(c, seed, 0xc130);
                let gate = DATA + 0x1000;
                let rider = DATA + 0x2000;
                let base_7 = DATA + 0x3000;
                let anim = DATA + 0x4000;
                w(c, gate + 0x14, rider);
                w(c, rider + 0x77c, base_7);
                w(c, base_7 + 0xde0, [1u32, 3, 7, 8][(seed % 4) as usize]);
                w(c, rider + 0x784, anim);
                // Layer 2's animation id: 2 keeps pushing, otherwise done.
                w(
                    c,
                    anim + 8,
                    if seed % 2 == 0 { 2 } else { r.next_u32() % 8 },
                );
                // A two-entry sequence list whose last entry is at 1.0.
                let list = DATA + 0x5000;
                let n0 = DATA + 0x5100;
                let n1 = DATA + 0x5200;
                w(c, anim + 0x50, list);
                w(c, list + 4, n0);
                w(c, n0 + 0xc8, n1);
                w(c, n1 + 8, 0x3f80_0000);
                w(c, n1 + 0x10, 0x3f80_0000);
                c.g[A0][0] = sx(gate);
            },
        );
    }

    /// Generic fixture: scramble the register file, point every argument at a
    /// writable object, and fill those objects with small values (counts,
    /// modes) plus a few pointers into the data area at the offsets where
    /// structures usually hold a vtable or a buffer. Also builds the library
    /// singletons the game's callees expect.
    pub(crate) fn generic(c: &mut Cpu, seed: u32, words: u32) {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        globals_8(c, &mut r);
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
    pub(crate) fn globals_8(c: &mut Cpu, r: &mut Rng) {
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

    pub(crate) const GATE: u32 = DATA + 0x1_0000;

    pub(crate) const GATE_RIDER: u32 = DATA + 0x1_1000;

    pub(crate) const GATE_RIDERBASE: u32 = DATA + 0x1_2000;

    #[test]
    fn rider_gate_sub_update_launch_matches() {
        check_all(
            ee::s00::f_0012c408__generated,
            rider_gate_sub_update_launch,
            30,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x12c4_08, 10);
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

    /// `RiderGateSub_UpdateLaunch` steps the launch's progress (`+0x10`)
    /// 0.015 per 60 Hz tick: four ticks at 240 Hz move it as far as one at
    /// 60, so the rider leaves the gate state after the same real time.
    #[test]
    fn rider_gate_sub_update_launch_follows_the_rate() {
        let run = |hz: u32, start: f32| -> f32 {
            let mut c = crate::difftest::fresh();
            let _ = base_8(&mut c, 1, 0x12c4_08, 10);
            c.bus.write32(GATE + 0x14, GATE_RIDER);
            c.bus.write32(GATE + 0xc, 1.0f32.to_bits());
            c.bus.write32(GATE_RIDER + 0x77c, GATE_RIDERBASE);
            c.bus.write32(GATE_RIDER + 0x784, 0);
            c.bus.write32(GATE_RIDERBASE + 0xde0, 0); // not main state 3: no launch
            put_words(&mut c, GATE_RIDER + 0x1b0, &[0]);
            c.bus.write32(GATE + 0x10, start.to_bits());
            // The game's constants (gp-0x780c..-0x7800: 1.015, 0.015, 0.985,
            // 0.62).
            c.g[GP][0] = GAME_GP as u64;
            for (off, v) in [
                (0x780cu32, 0x3f81_eb85u32),
                (0x7808, 0x3c75_c291),
                (0x7804, 0x3f7c_28f6),
                (0x7800, 0x3f1e_b852),
            ] {
                c.bus.write32(GAME_GP.wrapping_sub(off), v);
            }
            crate::tick::set_for_test(&mut c, hz);
            for _ in 0..hz / 60 {
                c.g[A0][0] = sx(GATE);
                rider_gate_sub_update_launch(&mut c);
            }
            f32::from_bits(c.bus.read32(GATE + 0x10))
        };
        for start in [0.31f32, 0.62] {
            let want = run(60, start);
            assert!(
                (want - (start + 0.015)).abs() < 1e-6,
                "60 Hz from {start}: {want}"
            );
            for hz in [120, 240] {
                let got = run(hz, start);
                assert!(
                    (got - want).abs() < 1e-5,
                    "{hz} Hz from {start}: {got} vs {want}"
                );
            }
        }
    }

    /// The dummy world (and its nested objects).
    pub(crate) const WORLD_7: u32 = DATA + 0x8_0000;

    #[test]
    fn rider_reset_sub_enter_matches() {
        diff(
            ee::s00::f_0012f230__generated,
            rider_reset_sub_enter,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_f230, 4);
                belib(c);
                let this = DATA + 0x1_0000;
                let anim = DATA + 0x2_0000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 8, anim);
                c.bus.write32(anim + 0x784, DATA + 0x2_1000);
                // The layer sequence object's +0x90 is written.
                c.bus.write32(DATA + 0x2_1000, DATA + 0x2_2000);
                // IsLocalPlayer: present and id >= 0 on two seeds in three.
                c.bus
                    .write32(anim + 0x87c, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(anim + 0x870, if seed % 6 < 3 { 0x100 } else { 0xffff_ffff });
                c.bus.write32(GAME_GP - 0x848, WORLD_7);
                let game = WORLD_7 + 0x1000;
                c.bus.write32(WORLD_7 + 0x84, game);
                let transition = DATA + 0x4_0000;
                c.bus.write32(game + 0xc, transition);
                c.bus
                    .write32(transition + 0x44, if seed % 2 == 0 { 0 } else { 1 });
                let _ = r.next_u32();
            },
        );
    }
}
