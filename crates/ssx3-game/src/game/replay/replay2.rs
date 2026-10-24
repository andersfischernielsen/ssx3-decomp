//! `src/replay/replay2.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_3;
pub(crate) use crate::game::dirtysock::tags2::at;
pub(crate) use crate::game::dirtysock::tags2::deleting_dtor;
pub(crate) use crate::game::dirtysock::tags2::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tags2::p as p_2;
pub(crate) use crate::game::dirtysock::tags2::p_2 as p;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::dirtysock::tags2::OPERATOR_DELETE;
pub(crate) use crate::game::fe::feasyncfile::addu;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p as p_7;
pub(crate) use crate::game::fe::feasyncfile::p as p_9;
pub(crate) use crate::game::fe::feasyncfile::p as p_11;
pub(crate) use crate::game::fe::feasyncfile::p as p_12;
pub(crate) use crate::game::fe::feasyncfile::p as p_13;
pub(crate) use crate::game::fe::feasyncfile::p as p_16;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_4;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_14;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sub;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_5;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_5;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_8;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_10;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p_3;
pub(crate) use crate::game::sound::soundsys::p as p_15;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0016_bd30,
        math_wrap_angle_clamp_half_pi,
        "Math_WrapAngleClampHalfPi",
    ),
    (
        0x0027_0070,
        player_alloc_replay_cache,
        "Player_AllocReplayCache",
    ),
    (
        0x0027_00d8,
        player_free_replay_cache,
        "Player_FreeReplayCache",
    ),
    (0x0027_0280, replay_is_paused, "Replay_IsPaused"),
    (0x0027_0378, replay_seek_to_end, "Replay_SeekToEnd"),
    (
        0x0027_05b0,
        replay_check_instant_replay,
        "Replay_CheckInstantReplay",
    ),
    (
        0x0027_0658,
        replay_request_instant_replay,
        "Replay_RequestInstantReplay",
    ),
    (
        0x0027_0670,
        replay_end_instant_replay,
        "Replay_EndInstantReplay",
    ),
    (0x0027_06b8, replay_open_menu, "Replay_OpenMenu"),
    (
        0x0027_0ab0,
        replay_on_rider_checkpoint,
        "Replay_OnRiderCheckpoint",
    ),
    (
        0x0027_0bd8,
        replay_count_highlights,
        "Replay_CountHighlights",
    ),
    (
        0x0027_0c40,
        replay_get_max_highlights,
        "Replay_GetMaxHighlights",
    ),
    (
        0x0027_0de0,
        replay_cancel_highlight,
        "Replay_CancelHighlight",
    ),
    (
        0x0027_0e18,
        replay_clear_all_highlights,
        "Replay_ClearAllHighlights",
    ),
    (0x0027_0e68, replay_begin_save, "Replay_BeginSave"),
    (
        0x0027_0f20,
        replay_alloc_save_buffers,
        "Replay_AllocSaveBuffers",
    ),
    (
        0x0027_0f78,
        replay_free_save_buffers,
        "Replay_FreeSaveBuffers",
    ),
    (
        0x0027_0fc0,
        replay_flush_save_output,
        "Replay_FlushSaveOutput",
    ),
    (0x0027_1058, replay_save_emit, "Replay_SaveEmit"),
    (
        0x0027_1228,
        replay_save_build_header,
        "Replay_SaveBuildHeader",
    ),
    (
        0x0027_12c0,
        replay_save_write_header,
        "Replay_SaveWriteHeader",
    ),
    (
        0x0027_12f8,
        replay_count_save_snapshots,
        "Replay_CountSaveSnapshots",
    ),
    (
        0x0027_1348,
        replay_save_begin_snapshots,
        "Replay_SaveBeginSnapshots",
    ),
    (
        0x0027_14e0,
        replay_save_begin_caches,
        "Replay_SaveBeginCaches",
    ),
    (
        0x0027_1620,
        replay_save_compute_padding,
        "Replay_SaveComputePadding",
    ),
    (
        0x0027_1688,
        replay_save_write_padding,
        "Replay_SaveWritePadding",
    ),
    (
        0x0027_1708,
        replay_save_write_footer,
        "Replay_SaveWriteFooter",
    ),
    (0x0027_21d0, replay_stream_dtor, "ReplayStream_dtor"),
    (
        0x0027_2258,
        world_state_stream_dtor,
        "WorldStateStream_dtor",
    ),
    (
        0x0027_2288,
        replay_highlight_slot_init,
        "Replay_HighlightSlotInit",
    ),
    (0x0027_23a8, in_seq_heap_free, "InSeq_HeapFree"),
    (0x0027_2570, math_eval_cubic, "Math_EvalCubic"),
    (
        0x0027_25b8,
        in_seq_curve_track_ctor,
        "InSeq_CurveTrack_ctor",
    ),
    (
        0x0027_25f0,
        in_seq_curve_track_start,
        "InSeq_CurveTrackStart",
    ),
    (
        0x0027_26a8,
        in_seq_curve_track_update,
        "InSeq_CurveTrackUpdate",
    ),
    (0x0027_28d0, in_seq_dtor, "InSeq_dtor"),
    (0x0027_2938, in_seq_start_sequence, "InSeq_StartSequence"),
    (0x0027_29a8, in_seq_free_sequence, "InSeq_FreeSequence"),
    (0x0027_2ad0, in_seq_play, "InSeq_Play"),
    (0x0027_2b58, in_seq_stop, "InSeq_Stop"),
    (0x0027_2d68, in_seq_span_track_ctor, "InSeq_SpanTrack_ctor"),
    (0x0027_2da0, inseq_span_track_dtor, "InSeq_SpanTrack_dtor"),
    (0x0027_2df0, inseq_span_track_start, "InSeq_SpanTrackStart"),
    (0x0027_2ec0, in_seq_span_track_stop, "InSeq_SpanTrackStop"),
    (
        0x0027_3040,
        in_seq_span_track_get_next_event_time,
        "InSeq_SpanTrackGetNextEventTime",
    ),
    (
        0x0027_30c0,
        in_seq_span_track_activate_key,
        "InSeq_SpanTrackActivateKey",
    ),
    (
        0x0027_3140,
        in_seq_span_track_next_key,
        "InSeq_SpanTrackNextKey",
    ),
    (
        0x0027_3180,
        in_seq_event_track_ctor,
        "InSeq_EventTrack_ctor",
    ),
    (0x0027_31b8, inseq_event_track_dtor, "InSeq_EventTrack_dtor"),
    (
        0x0027_3208,
        in_seq_event_track_start,
        "InSeq_EventTrackStart",
    ),
    (0x0027_32c8, in_seq_event_track_stop, "InSeq_EventTrackStop"),
    (
        0x0027_33e0,
        in_seq_event_track_get_next_event_time,
        "InSeq_EventTrackGetNextEventTime",
    ),
    (
        0x0027_3418,
        in_seq_event_track_fire_key,
        "InSeq_EventTrackFireKey",
    ),
    (
        0x0027_3468,
        in_seq_event_track_next_key,
        "InSeq_EventTrackNextKey",
    ),
    (0x0027_34a8, in_seq_ramp_track_ctor, "InSeq_RampTrack_ctor"),
    (0x0027_34e0, inseq_ramp_track_dtor, "InSeq_RampTrack_dtor"),
    (0x0027_3630, in_seq_ramp_track_stop, "InSeq_RampTrackStop"),
    (
        0x0027_37e0,
        in_seq_ramp_track_get_next_event_time,
        "InSeq_RampTrackGetNextEventTime",
    ),
    (
        0x0027_3860,
        in_seq_ramp_track_activate_key,
        "InSeq_RampTrackActivateKey",
    ),
    (
        0x0027_38f8,
        in_seq_ramp_track_next_key,
        "InSeq_RampTrackNextKey",
    ),
    (0x0027_3938, in_seq_channel_ctor, "InSeq_Channel_ctor"),
    (0x0027_3970, in_seq_channel_dtor, "InSeq_Channel_dtor"),
    (0x0027_39b8, in_seq_channel_setup, "InSeq_ChannelSetup"),
    (0x0027_3a00, in_seq_channel_release, "InSeq_ChannelRelease"),
    (
        0x0027_3a60,
        in_seq_channel_select_block,
        "InSeq_ChannelSelectBlock",
    ),
    (
        0x0027_3d20,
        in_seq_channel_flush_events,
        "InSeq_ChannelFlushEvents",
    ),
    (
        0x0027_4240,
        in_seq_channel_set_listener,
        "InSeq_ChannelSetListener",
    ),
    (0x0027_4518, in_seq_instance_ctor, "InSeq_Instance_ctor"),
    (0x0027_4830, in_seq_instance_start, "InSeq_InstanceStart"),
    (0x0027_49e8, in_seq_instance_pause, "InSeq_InstancePause"),
    (0x0027_4a08, in_seq_instance_resume, "InSeq_InstanceResume"),
    (
        0x0027_4c20,
        in_seq_instance_get_channel,
        "InSeq_InstanceGetChannel",
    ),
    (0x0027_4c38, in_seq_get_header_extra, "InSeq_GetHeaderExtra"),
    (
        0x0027_4c70,
        in_seq_instance_get_length,
        "InSeq_InstanceGetLength",
    ),
    (
        0x0027_4d08,
        in_seq_instance_get_remaining_steps,
        "InSeq_InstanceGetRemainingSteps",
    ),
    (
        0x0027_4d70,
        in_seq_instance_get_listener,
        "InSeq_InstanceGetListener",
    ),
    (0x0027_4a30, in_seq_instance_update, "InSeq_InstanceUpdate"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY as BUDDY_MAN_GET_ROSTER_ENTRY_2;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_ALL_FIELDS_FILLED;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_BACKUP_ACCOUNT;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_CONFIRM_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_EMAIL;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_INFORM;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_SHARE_INFO;
    pub(crate) use crate::game::ai::rider::callee::CROWD_MAN_GROUP_FREE;
    pub(crate) use crate::game::ai::rider::callee::GAME_UI_OPEN_SCREEN;
    pub(crate) use crate::game::ai::rider::callee::IN_SEQ_PLAY;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_ACCOUNT_MEM_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_DISCONNECT_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_GET_LIST_INDEX;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_OPEN_VIEW_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::NET_ACCOUNT_MEM_CTOR;
    pub(crate) use crate::game::ai::rider::callee::RAIL_MAN_ENABLE_RAIL;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHAR_STATS;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHECKING_CARD_POPUP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::ai::rider::callee::START_CARD_SCAN;
    pub(crate) use crate::game::ai::rider::callee::STREAMER_GET_SECTION_COUNT;
    pub(crate) use crate::game::ai::rider::callee::SYNC_PROCESS_DTOR;
    pub(crate) use crate::game::ai::rider::callee::TEX_MAN_EVICT_TEXTURE;
    pub(crate) use crate::game::ai::rider::callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_LOAD;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_RESET;
    pub(crate) use crate::game::ai::rider::callee::WSCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_IS_GAME_MUSIC_ENABLED;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;
    pub(crate) use crate::game::be::beintmission::callee::BANK_MONITOR_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BE_NEW_PLAYER_GET_INBOX;
    pub(crate) use crate::game::be::beintmission::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_RESET_STATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::be::beintmission::callee::CROWD_AUDIO_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintmission::callee::FIND_MISSION;
    pub(crate) use crate::game::be::beintmission::callee::GET_RIDER_PREVIEW;
    pub(crate) use crate::game::be::beintmission::callee::LOBBY_MENU_REQUEST_JOIN_ROOM;
    pub(crate) use crate::game::be::beintmission::callee::MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::be::beintmission::callee::MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::be::beintmission::callee::POPUP_CONFIRM_CFG_RESET;
    pub(crate) use crate::game::be::beintmission::callee::RACE_PROGRESS_RESET_ALL;
    pub(crate) use crate::game::be::beintmission::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_CHECK_REWARDS_READY;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_POLL_SHAPE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_DEACTIVATE_BINDING;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SLOGIN_DELETE;
    pub(crate) use crate::game::be::beintmission::callee::SND_BANK_SET_MEMORY;
    pub(crate) use crate::game::be::beintmission::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;
    pub(crate) use crate::game::be::beintoption::callee::BE_OPTION_INT_GET_MAX_HANDICAP;
    pub(crate) use crate::game::be::beintoption::callee::C_WORLD_SOUND_RELOCATE_EMITTERS;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_GET_INSTANCE;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_INSTANCE_STOP;
    pub(crate) use crate::game::be::beintoption::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::be::beintoption::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::be::beintoption::callee::SCRIPT_ENGINE_CLEAR_PROCESS;
    pub(crate) use crate::game::be::beintoption::callee::STREAM_ARRAY_STOP;
    pub(crate) use crate::game::be::beintoption::callee::USTR_FROM_ASCII_2;
    pub(crate) use crate::game::be::beintoption::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME2;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_COUNT_MESSAGES_FROM_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_FIND_ROSTER_ENTRY;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_BLOCKED_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_BUDDY_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_ONLINE_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_SAME_PRODUCT_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_TEMP_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_PLAY_FALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_PLAY_TRICK;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_STOP_ANTICIPATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CSETUP_MAN_SEND_PACKET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CVOIP_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::EATIMER_REMOVE_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2 as FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2 as FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_GET_INSTANCE as INSEQ_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_INSTANCE_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_VISIBLE_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_GAME_UTIL_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_ADD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_SKIP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_EMPTY_CROWD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_RIDER_SAY_20B9;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2 as UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2 as APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_DECODE_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_AUDIO_DONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_FRAME_DUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_ALL_TIMERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_BACKUP_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_NEXT_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_PREV_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_NODE_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILD_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5 as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_GET_RIDER_PREVIEW_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUTSYNC_IS_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_HEAP_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_FIND_ENABLED_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_LIST_GET_AT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED_3 as MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2 as MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_FIND_BANK_SLOT as PATH_PLAT_FIND_BANK_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_GRID_MESH_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BASE_RESET_FX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2 as SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_CATEGORY_TITLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_BUY_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_EQUIP_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME as SND_BANK_IS_LOADED_WITH_NAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD as SND_BANK_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK as SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_GET_PATH_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_7 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_4 as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::START_SCREEN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_GET_ACTIVE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_START_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3 as UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3 as UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2 as UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2 as USTR_CPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2 as USTR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
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
    pub(crate) use crate::game::fe::festaterewards::callee::AI_VISUAL_EFFECTS_MAN_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::CUI_TEXT_SCROLL_SET_NAV_BUTTONS;
    pub(crate) use crate::game::fe::festaterewards::callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::fe::festaterewards::callee::IN_SEQ_INSTANCE_SETUP;
    pub(crate) use crate::game::fe::festaterewards::callee::LIGHT_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_FORMAT_FLOAT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_DRAW_ROW;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PUSH_PAGE;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_BINDING_CLEAR_LISTENER;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_STOPPED;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_STOP;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_LIST_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_OBJECT_CTOR;
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
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_4;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR as MULTI_PARTICLE_DTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT as REPLAY_FIND_SNAPSHOT_2;
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
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2 as CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT as REPLAY_SAVE_EMIT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2 as SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
    pub(crate) const GET_LENGTH: Callee = Callee {
        addr: 0x0027_4c70,
        f: ee::s07::f_00274c70,
    };

    pub(crate) const TICK: Callee = Callee {
        addr: 0x0027_4de0,
        f: ee::s07::f_00274de0,
    };

    pub(crate) const STOP: Callee = Callee {
        addr: 0x0027_4918,
        f: ee::s07::f_00274918,
    };

    pub(crate) const RESTART: Callee = Callee {
        addr: 0x0027_4810,
        f: ee::s07::f_00274810,
    };

    pub(crate) const PAUSE: Callee = Callee {
        addr: 0x0027_49e8,
        f: ee::s07::f_002749e8,
    };
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {
    pub(crate) use crate::game::be::bedbreward::table_callee::EA_MEMSET;
    pub(crate) use crate::game::be::bedbreward::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::be::bedbreward::table_callee::WORLD_CREATE_DEAD_NODE;
}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {
    pub(crate) use crate::game::ai::computer::table::DELETE;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_FREE;
    pub(crate) use crate::game::ai::computer::table::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::ai::computer::table::SLEEP_THREAD;
    pub(crate) use crate::game::ai::computer::table::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::ai::computer::table::STR_HASH;
    pub(crate) use crate::game::ai::computer::table::WAIT_SEMA;
    pub(crate) use crate::game::ai::computer::table::WAKEUP_THREAD;
}

// ---------------------------------------------------------------------------
// InSeq_CurveTrackUpdate (0x002726a8).

/// `InSeq` curve-track fields (`InSeq_CurveTrackUpdate`).
pub(crate) mod in_seq_curve {

    /// The current curve segment (`{u16 start, u16 end, f32 p[4]}`).
    pub const CURVE: u32 = 0x24;

    /// The float value sampled last time.
    pub const VALUE: u32 = 0x1c;

    /// The segment being played.
    pub const SEGMENT: u32 = 0x14;

    /// The key index inside that segment.
    pub const KEY: u32 = 0x18;

    /// The current value's address inside the curve.
    pub const CURSOR: u32 = 0x20;

    /// The curve track's length in keys.
    pub const LENGTH: u32 = 0x10;
}

/// `void Replay_RequestInstantReplay(Replay *replay)` (0x270658): request an
/// instant replay only when one is not already active.
pub fn replay_request_instant_replay(c: &mut Cpu) {
    const ACTIVE: u32 = 0x620;
    const REQUESTED: u32 = 0x624;

    c.cycles += 3;
    let replay = Addr(abi::arg(c, 0));
    let active = replay.off(ACTIVE).lw(c);
    c.g[V0][0] = 1;
    if active == 0 {
        c.cycles += 1;
        replay.off(REQUESTED).sw(c, 1);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Replay recording.

/// `void Replay_SaveBeginSnapshots(Replay* r)` (0x271348): select the current
/// snapshot (the two pointers at +0x3b0/+0x3d0; when they are equal advance
/// through the node's +0x14 link into +0x65c), bump the record serial at
/// +0x638 and clear the per-record flags.
pub fn replay_save_begin_snapshots(c: &mut Cpu) {
    const FIRST: u32 = 0x3b0;
    const LAST: u32 = 0x3d0;
    const NODE_NEXT: u32 = 0x14;
    const CURRENT: u32 = 0x65c;
    const SERIAL: u32 = 0x638;
    const FLAG_660: u32 = 0x660;
    const FLAG_644: u32 = 0x644;
    const FLAG_648: u32 = 0x648;
    const FLAG_668: u32 = 0x668;
    c.cycles += 4;
    let r = Addr(abi::arg(c, 0));
    let first = r.off(FIRST).lw(c);
    c.g[V1][0] = first;
    let last = r.off(LAST).lw(c);
    c.g[V0][0] = last;
    let differs = first != last;
    r.off(CURRENT).sw(c, first);
    if !differs {
        c.cycles += 2;
        let next = Addr(first as u32).off(NODE_NEXT).lw(c);
        c.g[V0][0] = next;
        r.off(CURRENT).sw(c, next);
    }
    c.cycles += 8;
    let serial = r.off(SERIAL).lw(c);
    c.g[V0][0] = serial;
    r.off(FLAG_668).sw(c, 0);
    c.g[V0][0] = sx((serial as u32).wrapping_add(1));
    r.off(FLAG_660).sw(c, 0);
    r.off(SERIAL).sw(c, c.g[V0][0]);
    r.off(FLAG_644).sw(c, 0);
    r.off(FLAG_648).sw(c, 0);
    abi::ret(c);
}

/// `void Replay_SaveBeginCaches(Replay* r)` (0x2714e0): mark the cache phase
/// (state +0x658 = 2), bump the record serial at +0x638 and clear the cache
/// bookkeeping fields.
pub fn replay_save_begin_caches(c: &mut Cpu) {
    const SERIAL: u32 = 0x638;
    const STATE: u32 = 0x658;
    const CACHE_A: u32 = 0x654;
    const CACHE_B: u32 = 0x674;
    const CACHE_C: u32 = 0x66c;
    const CACHE_D: u32 = 0x670;
    const FLAG_644: u32 = 0x644;
    const FLAG_648: u32 = 0x648;
    c.cycles += 12;
    let r = Addr(abi::arg(c, 0));
    let serial = r.off(SERIAL).lw(c);
    c.g[V0][0] = serial;
    c.g[V1][0] = 0x2;
    r.off(STATE).sw(c, c.g[V1][0]);
    c.g[V0][0] = sx((serial as u32).wrapping_add(1));
    r.off(CACHE_A).sw(c, 0);
    r.off(SERIAL).sw(c, c.g[V0][0]);
    r.off(CACHE_B).sw(c, 0);
    r.off(CACHE_C).sw(c, 0);
    r.off(CACHE_D).sw(c, 0);
    r.off(FLAG_644).sw(c, 0);
    r.off(FLAG_648).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Cubic evaluation.

/// `float Math_EvalCubic(const float* coeffs, float t)` (0x272570): evaluate
/// `((c0*t + c1)*t + c2)*t + c3` (f12 = t), returning `c3` immediately when
/// `t` compares equal to zero.
pub fn math_eval_cubic(c: &mut Cpu) {
    const C1: u32 = 0x4;
    const C2: u32 = 0x8;
    const C3: u32 = 0xc;
    c.cycles += 4;
    let t = FpuF32(c.f[12]);
    // mtc1 f0, r0 / c.eq.s f12, f0.
    c.f[0] = 0;
    let eq = t.eq_s(FpuF32::ZERO);
    abi::set_fcc(c, eq);
    let coeffs = Addr(abi::arg(c, 0));
    if eq {
        c.cycles += 2;
        let c3 = coeffs.off(C3).read_fpu(c);
        c.f[0] = c3.0;
        abi::ret(c);
        return;
    }
    c.cycles += 11;
    let c0 = coeffs.read_fpu(c);
    let c1 = coeffs.off(C1).read_fpu(c);
    let mut f1 = c0 * t;
    let c2 = coeffs.off(C2).read_fpu(c);
    let c3 = coeffs.off(C3).read_fpu(c);
    let mut f0 = f1 + c1;
    f0 = f0 * t;
    f0 = f0 + c2;
    f1 = f0 * t;
    f0 = f1 + c3;
    c.f[0] = f0.0;
    c.f[1] = f1.0;
    c.f[2] = c1.0;
    c.f[3] = c2.0;
    c.f[4] = c3.0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Span tracks.

/// `void InSeq_SpanTrackActivateKey(SpanTrack* track, SpanKey* key)`
/// (0x2730c0): copy a span key into the track's `index * 0x1c` record at
/// `track + 0x20` and link it into the active list at `track + 0x24`.
pub fn in_seq_span_track_activate_key(c: &mut Cpu) {
    const ACTIVE_LIST: u32 = 0x20;
    const LINK: u32 = 0x24;
    const COUNT: u32 = 0x28;
    const RECORD: u32 = 0x1c;
    const KEY_INDEX: u32 = 0x2;
    const KEY_START: u32 = 0x8;
    const KEY_END: u32 = 0xc;
    const KEY_FLAGS: u32 = 0x4;
    const KEY_KIND: u32 = 0x6;
    c.cycles += 22;
    let key = c.g[A1][0];
    c.g[A2][0] = key;
    let track = c.g[A0][0];
    c.g[A3][0] = track;
    let index = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index;
    c.g[A1][0] = RECORD as u64;
    let base = Addr(track as u32).off(ACTIVE_LIST).lw(c);
    c.g[A0][0] = base;
    let scaled = abi::mult(c, index as u32, RECORD);
    c.g[V0][0] = sx(scaled);
    let start = Addr(key as u32).off(KEY_START).lw(c);
    c.g[V1][0] = start;
    let slot = sx((base as u32).wrapping_add(scaled));
    c.g[A0][0] = slot;
    Addr(slot as u32).sw(c, start);
    let index2 = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index2;
    Addr(slot as u32).off(0x4).sw(c, index2);
    let end = Addr(key as u32).off(KEY_END).lw(c);
    c.g[V1][0] = end;
    Addr(slot as u32).off(0x10).sw(c, end);
    let flags = Addr(key as u32).off(KEY_FLAGS).lhu(c);
    c.g[V0][0] = flags;
    Addr(slot as u32).off(0x8).sw(c, flags);
    let kind = Addr(key as u32).off(KEY_KIND).lhu(c);
    c.g[V1][0] = kind;
    Addr(slot as u32).off(0x14).sw(c, 0);
    Addr(slot as u32).off(0xc).sw(c, kind);
    let kind_field = Addr(key as u32).lhu(c);
    c.g[V0][0] = kind_field;
    let short = (kind_field < 0x11) as u64;
    c.g[V0][0] = short;
    if short != 0 {
        c.g[V1][0] = Addr(track as u32).off(LINK).lw(c);
    } else {
        c.cycles += 3;
        let payload = sx((key as u32).wrapping_add(0x10));
        c.g[V0][0] = payload;
        Addr(slot as u32).off(0x14).sw(c, payload);
        c.g[V1][0] = Addr(track as u32).off(LINK).lw(c);
    }
    c.cycles += 7;
    c.g[V0][0] = c.g[A0][0];
    Addr(slot as u32).off(0x18).sw(c, c.g[V1][0]);
    Addr(track as u32).off(LINK).sw(c, c.g[A0][0]);
    let count = Addr(track as u32).off(COUNT).lw(c);
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(track as u32).off(COUNT).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void* InSeq_SpanTrackNextKey(SpanTrack* track)` (0x273140): advance the
/// cursor at `track + 0x18` to the next span (adding the current span's length)
/// or clear the cursor at +0x14 at the end of the list.
pub fn in_seq_span_track_next_key(c: &mut Cpu) {
    const KEYS: u32 = 0x10;
    const CURSOR: u32 = 0x14;
    const INDEX: u32 = 0x18;
    c.cycles += 7;
    let track = c.g[A0][0];
    c.g[A1][0] = track;
    let keys = Addr(track as u32).off(KEYS).lw(c);
    c.g[V1][0] = keys;
    let cur = Addr(track as u32).off(INDEX).lw(c);
    c.g[A0][0] = cur;
    let count = Addr(keys as u32).lhu(c);
    c.g[V0][0] = count;
    let last = sx((count as u32).wrapping_sub(1));
    c.g[V0][0] = last;
    if cur != last {
        let next = Addr(track as u32).off(CURSOR).lw(c);
        c.g[V0][0] = next;
        c.cycles += 6;
        c.g[V1][0] = sx((cur as u32).wrapping_add(1));
        let len = Addr(next as u32).lhu(c);
        c.g[A0][0] = len;
        Addr(track as u32).off(INDEX).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((next as u32).wrapping_add(len as u32));
        Addr(track as u32).off(CURSOR).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(track as u32).off(CURSOR).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Event tracks.

/// `void InSeq_EventTrackFireKey(EventTrack* track, EventKey* key)` (0x273418):
/// store a key into the event array element `track + 0x1c + index*8` and bump
/// the element count at +0x20.
pub fn in_seq_event_track_fire_key(c: &mut Cpu) {
    const ELEMS: u32 = 0x1c;
    const COUNT: u32 = 0x20;
    const KEY_VALUE: u32 = 0x4;
    c.cycles += 12;
    let track = c.g[A0][0];
    c.g[A2][0] = track;
    let key = c.g[A1][0];
    let value = Addr(key as u32).off(KEY_VALUE).lw(c);
    c.g[V1][0] = value;
    let index = Addr(track as u32).off(COUNT).lw(c);
    c.g[V0][0] = index;
    let elems = Addr(track as u32).off(ELEMS).lw(c);
    c.g[A0][0] = elems;
    let delta = sx((index as u32).wrapping_shl(3));
    c.g[V0][0] = delta;
    let slot = sx((elems as u32).wrapping_add(delta as u32));
    c.g[A0][0] = slot;
    Addr(slot as u32).sw(c, value);
    Addr(slot as u32).off(0x4).sw(c, 0);
    let kind = Addr(key as u32).lhu(c);
    c.g[V0][0] = kind;
    let short = (kind < 9) as u64;
    c.g[V0][0] = short;
    let count;
    if short != 0 {
        count = Addr(track as u32).off(COUNT).lw(c);
        c.g[V1][0] = count;
    } else {
        c.cycles += 3;
        let payload = sx((key as u32).wrapping_add(0x8));
        c.g[V0][0] = payload;
        Addr(slot as u32).off(0x4).sw(c, payload);
        count = Addr(track as u32).off(COUNT).lw(c);
        c.g[V1][0] = count;
    }
    c.cycles += 4;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(track as u32).off(COUNT).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void* InSeq_EventTrackNextKey(EventTrack* track)` (0x273468): same cursor
/// walk as the span track, over the event list.
pub fn in_seq_event_track_next_key(c: &mut Cpu) {
    const KEYS: u32 = 0x10;
    const CURSOR: u32 = 0x14;
    const INDEX: u32 = 0x18;
    c.cycles += 7;
    let track = c.g[A0][0];
    c.g[A1][0] = track;
    let keys = Addr(track as u32).off(KEYS).lw(c);
    c.g[V1][0] = keys;
    let cur = Addr(track as u32).off(INDEX).lw(c);
    c.g[A0][0] = cur;
    let count = Addr(keys as u32).lhu(c);
    c.g[V0][0] = count;
    let last = sx((count as u32).wrapping_sub(1));
    c.g[V0][0] = last;
    if cur != last {
        let next = Addr(track as u32).off(CURSOR).lw(c);
        c.g[V0][0] = next;
        c.cycles += 6;
        c.g[V1][0] = sx((cur as u32).wrapping_add(1));
        let len = Addr(next as u32).lhu(c);
        c.g[A0][0] = len;
        Addr(track as u32).off(INDEX).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((next as u32).wrapping_add(len as u32));
        Addr(track as u32).off(CURSOR).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(track as u32).off(CURSOR).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Ramp tracks.

/// `void InSeq_RampTrackActivateKey(RampTrack* track, RampKey* key)`
/// (0x273860): copy a ramp key (including its converted integer/float fields)
/// into the `index * 0x24` record at `track + 0x1c` and link it into the active
/// list at +0x20.
pub fn in_seq_ramp_track_activate_key(c: &mut Cpu) {
    const ACTIVE_LIST: u32 = 0x1c;
    const LINK: u32 = 0x20;
    const COUNT: u32 = 0x24;
    const RECORD: u32 = 0x24;
    const KEY_INDEX: u32 = 0x2;
    const KEY_START: u32 = 0x8;
    const KEY_INT: u32 = 0xc;
    const KEY_FLOAT: u32 = 0x10;
    const KEY_END: u32 = 0x14;
    const KEY_FLAGS: u32 = 0x4;
    const KEY_KIND: u32 = 0x6;
    c.cycles += 27;
    let key = c.g[A1][0];
    c.g[A3][0] = key;
    let track = c.g[A0][0];
    c.g[T0][0] = track;
    let index = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index;
    c.g[A1][0] = RECORD as u64;
    let base = Addr(track as u32).off(ACTIVE_LIST).lw(c);
    c.g[A0][0] = base;
    let scaled = abi::mult(c, index as u32, RECORD);
    c.g[V0][0] = sx(scaled);
    let start = Addr(key as u32).off(KEY_START).lw(c);
    c.g[V1][0] = start;
    let slot = sx((base as u32).wrapping_add(scaled));
    c.g[A2][0] = slot;
    Addr(slot as u32).sw(c, start);
    let end = Addr(key as u32).off(KEY_END).lw(c);
    c.g[V0][0] = end;
    Addr(slot as u32).off(0x18).sw(c, end);
    let converted = FpuF32::from_i32(Addr(key as u32).off(KEY_INT).read_fpu(c).0 as i32);
    c.f[0] = converted.0;
    Addr(slot as u32).off(0x10).write_fpu(c, converted);
    let rate = Addr(key as u32).off(KEY_FLOAT).read_fpu(c);
    c.f[1] = rate.0;
    Addr(slot as u32).off(0x14).write_fpu(c, rate);
    let index2 = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index2;
    Addr(slot as u32).off(0xc).sw(c, index2);
    let flags = Addr(key as u32).off(KEY_FLAGS).lhu(c);
    c.g[V1][0] = flags;
    Addr(slot as u32).off(0x4).sw(c, flags);
    let kind = Addr(key as u32).off(KEY_KIND).lhu(c);
    c.g[A0][0] = kind;
    Addr(slot as u32).off(0x1c).sw(c, 0);
    Addr(slot as u32).off(0x8).sw(c, kind);
    let kind_field = Addr(key as u32).lhu(c);
    c.g[V0][0] = kind_field;
    let short = (kind_field < 0x19) as u64;
    c.g[V0][0] = short;
    let link;
    if short != 0 {
        link = Addr(track as u32).off(LINK).lw(c);
        c.g[V1][0] = link;
    } else {
        c.cycles += 3;
        let payload = sx((key as u32).wrapping_add(0x18));
        c.g[V0][0] = payload;
        Addr(slot as u32).off(0x1c).sw(c, payload);
        link = Addr(track as u32).off(LINK).lw(c);
        c.g[V1][0] = link;
    }
    c.cycles += 7;
    c.g[V0][0] = c.g[A2][0];
    Addr(slot as u32).off(0x20).sw(c, c.g[V1][0]);
    Addr(track as u32).off(LINK).sw(c, c.g[A2][0]);
    let count = Addr(track as u32).off(COUNT).lw(c);
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(track as u32).off(COUNT).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void* InSeq_RampTrackNextKey(RampTrack* track)` (0x2738f8): same cursor
/// walk as the span track, over the ramp list.
pub fn in_seq_ramp_track_next_key(c: &mut Cpu) {
    const KEYS: u32 = 0x10;
    const CURSOR: u32 = 0x14;
    const INDEX: u32 = 0x18;
    c.cycles += 7;
    let track = c.g[A0][0];
    c.g[A1][0] = track;
    let keys = Addr(track as u32).off(KEYS).lw(c);
    c.g[V1][0] = keys;
    let cur = Addr(track as u32).off(INDEX).lw(c);
    c.g[A0][0] = cur;
    let count = Addr(keys as u32).lhu(c);
    c.g[V0][0] = count;
    let last = sx((count as u32).wrapping_sub(1));
    c.g[V0][0] = last;
    if cur != last {
        let next = Addr(track as u32).off(CURSOR).lw(c);
        c.g[V0][0] = next;
        c.cycles += 6;
        c.g[V1][0] = sx((cur as u32).wrapping_add(1));
        let len = Addr(next as u32).lhu(c);
        c.g[A0][0] = len;
        Addr(track as u32).off(INDEX).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((next as u32).wrapping_add(len as u32));
        Addr(track as u32).off(CURSOR).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(track as u32).off(CURSOR).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Channels.

/// `InSeq_Channel* InSeq_Channel_ctor(InSeq_Channel* self)` (0x273938): zero the
/// non-head fields (offsets 4..0x2c) and return the channel.
pub fn in_seq_channel_ctor(c: &mut Cpu) {
    c.cycles += 13;
    let ch = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    ch.off(0x4).sw(c, 0);
    ch.off(0x8).sw(c, 0);
    ch.off(0xc).sw(c, 0);
    ch.off(0x10).sw(c, 0);
    ch.off(0x14).sw(c, 0);
    ch.off(0x18).sw(c, 0);
    ch.off(0x1c).sw(c, 0);
    ch.off(0x20).sw(c, 0);
    ch.off(0x24).sw(c, 0);
    ch.off(0x28).sw(c, 0);
    ch.off(0x2c).sw(c, 0);
    abi::ret(c);
}

/// `void InSeq_ChannelSetup(InSeq_Channel* ch, void* data, void* listener)`
/// (0x2739b8): bind the data block and listener, record the first track at
/// `+0x1c` (+0x20 when the block's kind is short) and return the bound data.
pub fn in_seq_channel_setup(c: &mut Cpu) {
    const DATA: u32 = 0x4;
    const LISTENER: u32 = 0x8;
    const DATA_SAVED: u32 = 0x10;
    const DATA_END: u32 = 0x14;
    const TRACK: u32 = 0x1c;
    const TRACK_LONG: u32 = 0x20;
    const DATA_OFFSET: u32 = 0x8;
    c.cycles += 13;
    let ch = Addr(abi::arg(c, 0));
    let data = c.g[A1][0];
    c.g[V0][0] = sx((data as u32).wrapping_add(DATA_OFFSET));
    ch.off(DATA).sw(c, data);
    ch.off(DATA_SAVED).sw(c, data);
    ch.off(LISTENER).sw(c, c.g[A2][0]);
    ch.off(DATA_END).sw(c, c.g[V0][0]);
    ch.off(0x18).sw(c, 0);
    let delta = Addr(data as u32).off(DATA_OFFSET).lw(c);
    c.g[V0][0] = delta;
    let track = sx((data as u32).wrapping_add(delta as u32));
    c.g[A1][0] = track;
    ch.off(TRACK).sw(c, track);
    let kind = Addr(track as u32).lhu(c);
    c.g[V0][0] = kind;
    let short = (kind < 9) as u64;
    c.g[V0][0] = short;
    if short != 0 {
        ch.off(TRACK_LONG).sw(c, 0);
    } else {
        c.cycles += 2;
        c.g[V0][0] = sx((track as u32).wrapping_add(DATA_OFFSET));
        ch.off(TRACK_LONG).sw(c, c.g[V0][0]);
    }
    c.cycles += 3;
    let saved = ch.off(DATA_SAVED).lw(c);
    c.g[V1][0] = saved;
    c.g[V0][0] = Addr(saved as u32).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Instances.

/// `InSeq_Instance* InSeq_Instance_ctor(InSeq_Instance* self)` (0x274518):
/// initialise the instance (gain 1.0 at +0x24, -1 at +0/+0x30/+0x34, zero
/// elsewhere) and return it.
pub fn in_seq_instance_ctor(c: &mut Cpu) {
    const GAIN: u32 = 0x24;
    const MINUS_ONE_A: u32 = 0x30;
    const MINUS_ONE_B: u32 = 0x34;
    c.cycles += 23;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = c.g[AT][0] as u32;
    let minus_one = 0xffff_ffff_ffff_ffffu64;
    c.g[V1][0] = minus_one;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    this.off(MINUS_ONE_B).sw(c, minus_one);
    this.off(GAIN).write_fpu(c, FpuF32(c.f[0]));
    this.sw(c, minus_one);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x18).sw(c, 0);
    this.off(0x1c).sw(c, 0);
    this.off(0x20).sw(c, 0);
    this.off(0x28).sw(c, 0);
    this.off(0x2c).sw(c, 0);
    this.off(MINUS_ONE_A).sw(c, minus_one);
    this.off(0x38).sw(c, 0);
    this.off(0x3c).sw(c, 0);
    this.off(0x40).sw(c, 0);
    this.off(0x44).sw(c, 0);
    abi::ret(c);
}

/// `void InSeq_InstanceStart(InSeq_Instance* inst, void* data)` (0x274830):
/// start playback, recording the current track at +0x1c; if the instance is
/// already running and has a pending start request at +0x3c, clear +0x40.
pub fn in_seq_instance_start(c: &mut Cpu) {
    const ACTIVE: u32 = 0x20;
    const TRACK: u32 = 0x1c;
    const PENDING: u32 = 0x3c;
    const REQUEST: u32 = 0x40;
    c.cycles += 3;
    let inst = Addr(abi::arg(c, 0));
    let active = inst.off(ACTIVE).lw(c);
    c.g[V0][0] = active;
    let one = 0x1u64;
    c.g[V0][0] = one;
    if active == 0 {
        c.cycles += 2;
        inst.off(TRACK).sw(c, c.g[A1][0]);
        inst.off(ACTIVE).sw(c, one);
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    let pending = inst.off(PENDING).lw(c);
    c.g[V0][0] = pending;
    if pending != 0 {
        c.cycles += 2;
        inst.off(REQUEST).sw(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void InSeq_InstancePause(InSeq_Instance* inst)` (0x2749e8): while running,
/// increment the pause counter at +0x2c.
pub fn in_seq_instance_pause(c: &mut Cpu) {
    const ACTIVE: u32 = 0x20;
    const PAUSE_COUNT: u32 = 0x2c;
    c.cycles += 3;
    let inst = Addr(abi::arg(c, 0));
    let active = inst.off(ACTIVE).lw(c);
    c.g[V0][0] = active;
    if active != 0 {
        c.cycles += 3;
        let count = inst.off(PAUSE_COUNT).lw(c);
        c.g[V0][0] = sx((count as u32).wrapping_add(1));
        inst.off(PAUSE_COUNT).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void InSeq_InstanceResume(InSeq_Instance* inst)` (0x274a08): while running,
/// decrement the pause counter at +0x2c, clamping at zero.
pub fn in_seq_instance_resume(c: &mut Cpu) {
    const ACTIVE: u32 = 0x20;
    const PAUSE_COUNT: u32 = 0x2c;
    c.cycles += 3;
    let inst = Addr(abi::arg(c, 0));
    let active = inst.off(ACTIVE).lw(c);
    c.g[V0][0] = active;
    if active != 0 {
        c.cycles += 4;
        let count = inst.off(PAUSE_COUNT).lw(c);
        c.g[V0][0] = sx((count as u32).wrapping_sub(1));
        let value = c.g[V0][0];
        inst.off(PAUSE_COUNT).sw(c, value);
        if (value as i64) < 0 {
            c.cycles += 1;
            inst.off(PAUSE_COUNT).sw(c, 0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sequences.

/// `InSeqChannel *InSeq_InstanceGetChannel(InSeq *self, int index)` (0x274c20):
/// the `index`-th channel record (0x3c bytes each) whose array starts at
/// `self + 0x18`. Returns a pointer computed as `base + index*0x3c`.
pub fn in_seq_instance_get_channel(c: &mut Cpu) {
    c.cycles += 5;
    const CHANNELS: u32 = 0x18;
    const CHANNEL_SIZE: u32 = 0x3c;
    c.g[V1][0] = CHANNEL_SIZE as u64;
    let base = Addr(abi::arg(c, 0)).off(CHANNELS).lw(c);
    c.g[V0][0] = base;
    let offset = abi::mult(c, abi::arg(c, 1), CHANNEL_SIZE);
    c.g[A1][0] = sx(offset);
    c.g[V0][0] = sx((base as u32).wrapping_add(offset));
    abi::ret(c);
}

/// `InSeqListener *InSeq_InstanceGetListener(InSeq *self)` (0x274d70): the
/// listener cached at `self + 0x44` when set, otherwise the object reached
/// through `self + 0x8` (`*(self->+0x8)`).
pub fn in_seq_instance_get_listener(c: &mut Cpu) {
    c.cycles += 3;
    const CACHE: u32 = 0x44;
    const LINK: u32 = 0x8;
    let self_ = Addr(abi::arg(c, 0));
    let cached = self_.off(CACHE).lw(c);
    c.g[V0][0] = cached;
    if cached != 0 {
        c.cycles += 2;
    } else {
        c.cycles += 3;
        let link = self_.off(LINK).lw(c);
        c.g[V1][0] = link;
        c.g[V0][0] = Addr(link as u32).lw(c);
    }
    abi::ret(c);
}

/// `float Math_WrapAngleClampHalfPi(float angle)` (0x16bd30): wrap an angle
/// into [-pi/2, pi/2) using a rounding step and a final clamp.
pub fn math_wrap_angle_clamp_half_pi(c: &mut Cpu) {
    let angle = Addr(abi::arg(c, 0));
    let half = FpuF32(0x3f00_0000);
    c.cycles += 11;
    let scale = Addr((c.g[GP][0] as u32).wrapping_sub(0x6470)).read_fpu(c);
    let input = angle.read_fpu(c);
    c.f[4] = input.0;
    c.g[AT][0] = 0x3f00_0000;
    c.f[1] = half.0;
    let f0 = scale * input + half;
    let n = f0.to_i32();
    let mut f2 = FpuF32::from_i32(n);
    c.f[2] = f2.0;
    let lt = f0.lt(f2);
    abi::set_fcc(c, lt);
    if lt {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = 0x3f80_0000;
        f2 = f2 - FpuF32::ONE;
        c.f[2] = f2.0;
    }
    c.cycles += 10;
    let a = Addr((c.g[GP][0] as u32).wrapping_sub(0x646c)).read_fpu(c);
    c.f[0] = a.0;
    let b = Addr((c.g[GP][0] as u32).wrapping_sub(0x6468)).read_fpu(c);
    c.f[1] = b.0;
    let mut f0 = a * f2;
    let cc = Addr((c.g[GP][0] as u32).wrapping_sub(0x6464)).read_fpu(c);
    c.f[2] = cc.0;
    c.f[3] = b.0;
    f0 = input - f0;
    c.f[0] = f0.0;
    let le = b.le(f0);
    abi::set_fcc(c, le);
    angle.write_fpu(c, f0);
    if le {
        c.cycles += 1;
        c.f[3] = f0.min(cc).0;
    }
    c.cycles += 2;
    angle.write_fpu(c, FpuF32(c.f[3]));
    abi::ret(c);
}

/// `int Replay_GetMaxHighlights(Replay* r, int unused)` (0x270c40): the
/// highlight count at +0xc, or 1 when there is no replay.
pub fn replay_get_max_highlights(c: &mut Cpu) {
    c.cycles += 2;
    let unused = c.g[A1][0];
    c.g[V0][0] = 0x1;
    if unused == 0 {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        let selfp = abi::arg(c, 0);
        c.g[V0][0] = c.lw(selfp.wrapping_add(0xc));
        abi::ret(c);
    }
}

/// `void *InSeq_GetHeaderExtra(SeqHeader *h)` (0x00274c38): the extra header
/// at `+0xc` for format versions >= 13, else null.
pub fn in_seq_get_header_extra(c: &mut Cpu) {
    c.cycles += 4;
    let version = Addr((c.g[A0][0] as u32).wrapping_add(2)).lhu(c);
    let old = (version < 0xd) as u64;
    c.g[V0][0] = old;
    c.cycles += 2;
    if old != 0 {
        c.g[V0][0] = 0;
    } else {
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Deleting destructors.

/// The shared body of the `(flags & 1)` deleting destructors: delete `this`
/// when the low bit of `a1` is set (the frame is still opened either way).
pub(crate) fn destroy_if_flag(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[A1][0] &= 1;
    frame.save_ra(c, 0);
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        call_ptr(c, table_callee::OPERATOR_DELETE, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void InSeq_dtor(InSeq *this, int flags)` (0x2728d0).
pub fn in_seq_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x0027_28e8))
}

// ---------------------------------------------------------------------------
// Sequence / audio / object helpers.

/// `void InSeq_Stop(InSeq *self, int slot)` (0x272b58): fetch the sequence
/// instance for `slot` and stop it.
pub fn in_seq_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::INSEQ_GET_INSTANCE.call(c, 0x0027_2b68)?;
        c.g[A0][0] = c.g[V0][0];
        callee::INSEQ_INSTANCE_STOP.call(c, 0x0027_2b70)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ReplayStream_dtor(ReplayStream *self, int flags)` (0x2721d0):
/// republishes `D_00481898` at `self+0`, deletes on bit 0.
pub fn replay_stream_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_1898, 0, 0x0027_21f4))
}

/// `void WorldStateStream_dtor(WorldStateStream *self, int flags)`
/// (0x272258): republishes `D_00481898` at `self+0`, deletes on bit 0.
pub fn world_state_stream_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_1898, 0, 0x0027_227c))
}

// ---------------------------------------------------------------------------
// Script / sequence.

/// `void InSeq_Play(Channel *ch, int index, void *owner)` (0x272ad0): fetch
/// the instrument-sequence instance for the slot and start it with `a2`.
pub fn in_seq_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        callee::INSEQ_GET_INSTANCE_2.call(c, 0x0027_2ae4)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        callee::INSEQ_INSTANCE_START.call(c, 0x0027_2af0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_HighlightSlotInit(ReplaySlot* slot)` (0x272288): mark the
/// slot empty and clear its 0xac-byte payload.
pub fn replay_highlight_slot_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = u64::MAX;
        Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        Addr((c.g[V0][0] as u32).wrapping_add(0xb0)).sw(c, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0xac;
        call_ptr(c, callee::EA_MEMSET, 0x0027_22b0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// InSeq track constructors: install the class vtable at +4 and clear the
// following fields.

/// Shared body: install the class vtable at +4, clear the following fields
/// and return `self` (the ctors are leaves, so `pc` goes to `ra`).
pub(crate) fn in_seq_track_ctor(c: &mut Cpu, vtable: u32, zero_offsets: &[u32]) {
    c.g[V1][0] = 0x0048_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(vtable);
    Addr((c.g[A0][0] as u32).wrapping_add(8)).sw(c, 0);
    Addr((c.g[A0][0] as u32).wrapping_add(4)).sw(c, c.g[V1][0]);
    for &off in zero_offsets {
        Addr((c.g[A0][0] as u32).wrapping_add(off)).sw(c, 0);
    }
    crate::abi::ret(c);
}

/// `InSeq_CurveTrack* InSeq_CurveTrack_ctor(InSeq_CurveTrack* self)`
/// (0x2725b8).
pub fn in_seq_curve_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_1a10, &[0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24]);
}

/// `InSeq_SpanTrack* InSeq_SpanTrack_ctor(InSeq_SpanTrack* self)`
/// (0x272d68).
pub fn in_seq_span_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_19b0, &[0xc, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28]);
}

/// `InSeq_EventTrack* InSeq_EventTrack_ctor(InSeq_EventTrack* self)`
/// (0x273180).
pub fn in_seq_event_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_1950, &[0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24]);
}

/// `InSeq_RampTrack* InSeq_RampTrack_ctor(InSeq_RampTrack* self)`
/// (0x2734a8).
pub fn in_seq_ramp_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_18f0, &[0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24]);
}

/// `void Replay_OpenMenu(ReplayCache* self)` (0x2706b8): open the replay menu
/// unless a dialog is active or it is already open.
pub fn replay_open_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x61c).lw(c);
        let busy = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if busy {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[V0][0] = 9;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = 9;
        callee::REPLAY_SET_STATE.call(c, 0x0027_06e4)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_CancelHighlight(sRcSlots* self, int slot)` (0x270de0): clear the
/// highlight flag of slot `i` when the replay is idle.
pub fn replay_cancel_highlight(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        let active = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !active {
            c.cycles += 6;
            c.g[A0][0] = 0xb4;
            c.mult(0, A0, A1, A0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x494));
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            callee::REPLAY_HIGHLIGHT_SLOT_CLEAR.call(c, 0x0027_0e0c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SaveWriteHeader(ReplayCache* self)` (0x2712c0): emit the 0x28
/// byte save header and bump the record counter.
pub fn replay_save_write_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x28;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x640).lw(c);
        callee::REPLAY_SAVE_EMIT.call(c, 0x0027_12dc)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x638).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let v0 = c.g[V0][0];
        let s0 = c.g[S0][0];
        Addr(s0 as u32).off(0x638).sw(c, v0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_EndInstantReplay(Replay* self)` (0x270670): when an instant
/// replay is running, end the playback and let the rider resume.
pub fn replay_end_instant_replay(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(at(c, A0, 0x620));
        let idle = c.g[V0][0] == 0;
        c.g[RA][0] = c.ld(p(sp(c), 0));
        if !idle {
            c.cycles += 10;
            callee::REPLAY_END_PLAYBACK.call(c, 0x0027_068c)?;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[V1][0] = c.lw(at(c, V0, 0x84));
            c.g[A0][0] = c.lw(at(c, V1, 0x0c));
            c.g[V0][0] = c.lw(at(c, A0, 0x40));
            c.g[V1][0] = c.lw(at(c, V0, 0x18));
            c.g[A0][0] = c.lw(at(c, V1, 0x790));
            call_ptr(c, callee::APP_SOMETHING, 0x0027_06a8)?;
            c.g[RA][0] = c.ld(p(sp(c), 0));
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_FreeSaveBuffers(Replay* self)` (0x270f78): free both save
/// buffers and clear the fields that point at them.
pub fn replay_free_save_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(at(c, S0, 0x63c));
        call_ptr(c, callee::BUILTIN_DELETE_2, 0x0027_0f90)?;
        c.g[A0][0] = c.lw(at(c, S0, 0x640));
        if c.g[A0][0] == 0 {
            sw(c, r32(c, S0), 0x640, 0);
        } else {
            c.cycles += 3;
            call_ptr(c, callee::EA_MEM_FREE_2, 0x0027_0fa4)?;
            sw(c, r32(c, S0), 0x640, 0);
        }
        c.cycles += 5;
        sw(c, r32(c, S0), 0x63c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// In-sequence channel data.

/// `void InSeq_Channel_dtor(InSeq_Channel* self, int flags)` (0x273970):
/// release the channel's block, then delete it from the heap when `flags &
/// 1`.
pub fn in_seq_channel_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::IN_SEQ_CHANNEL_RELEASE.call(c, 0x0027_398c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::IN_SEQ_HEAP_DELETE.call(c, 0x0027_39a0)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void InSeq_ChannelSelectBlock(InSeq_Channel* self, int index)`
/// (0x273a60): select the sequence block at `index` and point at its first
/// event record.
pub fn in_seq_channel_select_block(c: &mut Cpu) {
    c.cycles += 13;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V1][0] = c.lw(at(c, A2, 0x14));
    sw(c, r32(c, A2), 0x18, c.g[A1][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = c.lw(at(c, A2, 0x04));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    sw(c, r32(c, A2), 0x1c, c.g[A0][0]);
    c.g[V0][0] = c.lhu(r32(c, A0));
    c.g[V0][0] = (c.g[V0][0] < 9u64) as u64;
    let small = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x08));
    if small {
        c.cycles += 2;
        sw(c, r32(c, A2), 0x20, 0);
    } else {
        c.cycles += 2;
        sw(c, r32(c, A2), 0x20, c.g[V0][0]);
    }
    abi::ret(c);
}

/// `void Replay_SaveEmit(ReplayCache* self, const signed char* data, int
/// size)` (0x271058): account `size` bytes (and their byte sum) and remember
/// the last emitted block.
pub fn replay_save_emit(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.g[V0][0] = Addr(this).off(0x680).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
    let empty = (c.g[A2][0] as i64) <= 0;
    Addr(this).off(0x680).sw(c, c.g[V0][0]);
    if !empty {
        c.cycles += 2;
        c.g[T0][0] = c.g[A1][0];
        c.g[A3][0] = c.g[A2][0];
        loop {
            c.cycles += 7;
            c.g[V1][0] = c.lb(r32(c, T0));
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
            c.g[V0][0] = Addr(this).off(0x684).lw(c);
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            let more = c.g[A3][0] != 0;
            Addr(this).off(0x684).sw(c, c.g[V0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_1070);
            }
        }
    }
    c.cycles += 6;
    c.g[V0][0] = Addr(this).off(0x678).lw(c);
    Addr(this).off(0x644).sw(c, c.g[A1][0]);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
    Addr(this).off(0x648).sw(c, c.g[A2][0]);
    Addr(this).off(0x678).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `int Replay_CountSaveSnapshots(ReplayCache* self)` (0x2712f8): walk the
/// snapshot chain from `+0x3b0`, skipping the one at `+0x3d0`, and return how
/// many others there are.
pub fn replay_count_save_snapshots(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.g[V0][0] = Addr(this).off(0x3b0).lw(c);
    c.g[V1][0] = 0;
    let empty = c.g[V0][0] == 0;
    Addr(this).off(0x65c).sw(c, c.g[V0][0]);
    if !empty {
        c.cycles += 2;
        c.g[A1][0] = Addr(this).off(0x3d0).lw(c);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x65c).lw(c);
            if c.g[V0][0] != c.g[A1][0] {
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V0)).off(0x14).lw(c);
                Addr(this).off(0x65c).sw(c, c.g[V0][0]);
                c.g[V0][0] = Addr(this).off(0x65c).lw(c);
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V0)).off(0x14).lw(c);
            let more = c.g[V0][0] != 0;
            Addr(this).off(0x65c).sw(c, c.g[V0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_1310);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

/// `void Replay_SaveWriteFooter(ReplayCache* self)` (0x271708): clear the
/// 8-byte header block and emit it as the save footer.
pub fn replay_save_write_footer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 8;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = sx(this.wrapping_add(0x680));
        c.g[A0][0] = Addr(this).off(0x640).lw(c);
        call_ptr(c, 0x003e_6574, 0x0027_1728)?;
        c.g[A1][0] = Addr(this).off(0x640).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 8;
        callee::REPLAY_SAVE_EMIT_2.call(c, 0x0027_1738)?;
        c.g[V0][0] = Addr(this).off(0x638).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        Addr(this).off(0x638).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_ClearAllHighlights(Replay* r)` (0x270e18): unless a replay is
/// running, clear the two highlight slots at +0x494 and +0x548. 7 (+3, 5×2+1)
/// + 4 cycles plus the callees.
pub fn replay_clear_all_highlights(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        let state = c.lw(self_);
        c.g[V0][0] = state;
        if state == 0 {
            c.cycles += 3;
            let mut slot = p_2(self_, 0x494);
            c.g[S0][0] = sx(slot);
            c.g[S1][0] = 1;
            loop {
                c.cycles += 5;
                c.g[A0][0] = sx(slot);
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2.call(c, 0x0027_0e4c)?;
                let more = c.g[S1][0] as i64 >= 0;
                slot = p_2(slot, 0xb4);
                c.g[S0][0] = sx(slot);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_0e40);
                    }
                    continue;
                }
                c.cycles += 1;
                frame.restore(c, 16, 0x20);
                break;
            }
        } else {
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sequencer tracks.

/// The shared body of the three `InSeq_*Track` destructors: install the class
/// vtable at +4, stop the track and, when bit 0 of `flags` is set, delete the
/// object. 13 + 4 (+3) cycles plus the callees.
pub(crate) fn inseq_track_dtor(c: &mut Cpu, vt: u32, stop: Callee, ret: u32) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(vt);
    c.sq(17, p_2(sp(c), 0x10));
    c.sq(16, p_2(sp(c), 0x20));
    c.g[S1][0] = c.g[A0][0];
    c.sd(p_2(sp(c), 0), c.g[RA][0]);
    let flags = c.g[A1][0];
    c.g[S0][0] = flags;
    c.sw(p_2(r32(c, S1), 4), c.g[V0][0]);
    stop.call(c, ret)?;
    c.g[S0][0] = c.g[S0][0] & 1;
    let keep = c.g[S0][0] == 0;
    c.lq(16, p_2(sp(c), 0x20));
    if !keep {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        callee::INSEQ_HEAP_DELETE.call(c, ret + 0x14)?;
        c.lq(16, p_2(sp(c), 0x20));
    }
    c.cycles += 4;
    c.lq(17, p_2(sp(c), 0x10));
    c.g[RA][0] = c.ld(p_2(sp(c), 0));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
    c.pc = ra;
    Ok(())
}

/// `void InSeq_SpanTrack_dtor(InSeq_SpanTrack* t, int flags)` (0x272da0).
pub fn inseq_span_track_dtor(c: &mut Cpu) {
    guest(c, |c| {
        inseq_track_dtor(c, 0x0048_19b0, callee::INSEQ_SPAN_TRACK_STOP, 0x0027_2dc8)
    })
}

/// `void InSeq_EventTrack_dtor(InSeq_EventTrack* t, int flags)` (0x2731b8).
pub fn inseq_event_track_dtor(c: &mut Cpu) {
    guest(c, |c| {
        inseq_track_dtor(c, 0x0048_1950, callee::INSEQ_EVENT_TRACK_STOP, 0x0027_31e0)
    })
}

/// `void InSeq_RampTrack_dtor(InSeq_RampTrack* t, int flags)` (0x2734e0).
pub fn inseq_ramp_track_dtor(c: &mut Cpu) {
    guest(c, |c| {
        inseq_track_dtor(c, 0x0048_18f0, callee::INSEQ_RAMP_TRACK_STOP, 0x0027_3508)
    })
}

/// `void Player_FreeReplayCache(Player* p_3, void* cache)` (0x2700d8): find
/// `cache` in the player's two replay slots, destroy it and clear the slot.
pub fn player_free_replay_cache(c: &mut Cpu) {
    c.cycles += 4;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    let found_at = c.g[A1][0];
    frame.save_ra(c, 0);
    if found_at != 0 {
        c.cycles += 2;
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48c));
        c.g[V1][0] = 0;
        loop {
            c.cycles += 3;
            c.g[A0][0] = Addr(p_3(c, S0, 0)).lw(c);
            if c.g[A0][0] != c.g[A1][0] {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.sw(p_3(c, S0, 0), 0);
                    break;
                }
                c.cycles += 4;
                c.g[A1][0] = 3;
                if callee::REPLAY_CACHE_DTOR.call(c, 0x0027_010c).is_err() {
                    return;
                }
                c.sw(p_3(c, S0, 0), 0);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_00f0);
            }
        }
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void Replay_AllocSaveBuffers(Replay* r)` (0x270f20): allocate the 16 KiB
/// replay buffer and the 50 KiB save buffer.
pub fn replay_alloc_save_buffers(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = sx(0x0048_1800);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = 0x4000;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    if call_ptr(c, table::EA_MEM_ALLOC_NAMED, 0x0027_0f48).is_err() {
        return;
    }
    c.g[A1][0] = sx(0x0048_1810);
    c.sw(p_3(c, S0, 0x63c), c.g[V0][0]);
    c.g[A0][0] = 0xc800;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    if call_ptr(c, table::OPERATOR_NEW_NAMED, 0x0027_0f64).is_err() {
        return;
    }
    c.sw(p_3(c, S0, 0x640), c.g[V0][0]);
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// Return -1 (the "no event" sentinel): the shared tail of the three
/// `InSeq_*TrackGetNextEventTime` queries.
pub(crate) fn ret_minus_one(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
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

/// `void InSeq_ChannelRelease(InSeqChannel* self)` (0x273a00): stop and clear
/// the channel when one is bound.
pub fn in_seq_channel_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p_4(c.g[S0][0], 4));
        if c.g[V0][0] != 0 {
            c.cycles += 14;
            call_ptr(c, 0x0027_3dc0, 0x0027_3a24)?;
            Addr(p_4(c.g[S0][0], 0x38)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 4)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 8)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0xc)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x10)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x14)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x18)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x1c)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x20)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x30)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x34)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Replay.

/// `void Replay_SaveComputePadding(Replay* self)` (0x271620): clear the save
/// buffer, measure the payload and leave the padding count in `+0x67c`.
pub fn replay_save_compute_padding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0xc800;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p_5(c.g[S0][0] as u32, 0x640)).lw(c);
        call_ptr(c, 0x003e_6448, 0x0027_1640)?;
        call_ptr(c, 0x0014_dc80, 0x0027_1648)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_e048, 0x0027_1650)?;
        c.g[A1][0] = Addr(p_5(c.g[S0][0] as u32, 0x678)).lw(c);
        c.g[V1][0] = 0x0007_0000;
        c.g[A0][0] = Addr(p_5(c.g[S0][0] as u32, 0x638)).lw(c);
        c.g[V1][0] = c.g[V1][0] | 0xfff8;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        Addr(p_5(c.g[S0][0] as u32, 0x638)).sw(c, c.g[A0][0]);
        Addr(p_5(c.g[S0][0] as u32, 0x67c)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `void Replay_SaveWritePadding(Replay* self)` (0x271688): emit at most 0x4000
/// bytes of pending padding, or count an empty write.
pub fn replay_save_write_padding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p_5(c.g[S1][0] as u32, 0x67c)).lw(c);
        let no_padding = (c.g[V1][0] as i64) <= 0;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 16385) as u64;
        if no_padding {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_5(c.g[S1][0] as u32, 0x638)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(p_5(c.g[S1][0] as u32, 0x638)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 9;
            c.g[S0][0] = 0x4000;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.g[V1][0];
            }
            c.g[A1][0] = Addr(p_5(c.g[S1][0] as u32, 0x640)).lw(c);
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0027_1058, 0x0027_16bc)?;
            c.g[V0][0] = Addr(p_5(c.g[S1][0] as u32, 0x67c)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
            Addr(p_5(c.g[S1][0] as u32, 0x67c)).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int InSeq_InstanceGetRemainingSteps(InSeq* self)` (0x274d08): the number
/// of remaining steps, rounded up.
pub fn in_seq_instance_get_remaining_steps(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0027_4c70, 0x0027_4d1c)?;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.lwc1(1, p_5(c.g[S0][0] as u32, 0x28));
        c.lwc1(2, p_5(c.g[S0][0] as u32, 0x24));
        c.sub_s(0, 0, 1);
        c.div_s(0, 0, 2);
        c.cvt_w_s(1, 0);
        c.cvt_s_w(1, 1);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.cycles += 3;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.add_s(1, 1, 0);
        }
        c.cycles += 6;
        c.cvt_w_s(0, 1);
        c.mfc1(2, 0);
        frame.restore(c, S0, 0x10);
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `Replay_IsPaused`'s state table (`0x480000 + 0x17c0`), indexed by the
/// replay object's first word (only states < 0x10 are looked up).
pub(crate) const REPLAY_TABLE: u32 = 0x0048_17c0;

// ---------------------------------------------------------------------------
// Replay / audio predicates.

/// `int Replay_IsPaused(Replay *r)` (0x270280): the replay state word at
/// `+0` picks a handler from the table at [`REPLAY_TABLE`]. States 1 and 7
/// compare two counters (`+0x484` against `+0x3cc`, and `+0x604` must be
/// zero); states 3 and 5 return true, state 4 returns `+0x484 >= +0x3cc`,
/// most states return false. States >= 0x10 return false without a lookup.
///
/// One block per table target, in the generated code's order. The function
/// makes no calls and checks no events, so its only entry is the top.
pub fn replay_is_paused(c: &mut Cpu) {
    c.cycles += 5;
    let replay = r32(c, A0);
    c.g[A2][0] = c.g[A0][0];
    c.g[V1][0] = Addr(replay).lw(c);
    c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[A3][0] = 0;
    if !out_of_range {
        c.cycles += 7;
        c.g[V0][0] = 0x0048_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx(REPLAY_TABLE);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        match c.g[A0][0] as u32 {
            // State 1 / 7: a pause request is visible in the two counters.
            0x0027_02b0 => {
                c.cycles += 9;
                c.g[A0][0] = 0x1;
                c.g[A1][0] = Addr(replay).off(0x3cc).lw(c);
                c.g[V1][0] = Addr(replay).off(0x604).lw(c);
                c.g[A3][0] = c.g[A0][0];
                c.g[V0][0] = Addr(replay).off(0x484).lw(c);
                if c.g[V1][0] != 0 {
                    c.g[A3][0] = 0;
                }
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[A0][0];
                }
            }
            // State 4: not yet caught up.
            0x0027_02d4 => {
                c.cycles += 5;
                c.g[V1][0] = Addr(replay).off(0x3cc).lw(c);
                c.g[V0][0] = Addr(replay).off(0x484).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[A3][0] = c.g[V0][0] ^ 0x1;
            }
            // States 3 and 5: paused.
            0x0027_02e8 => {
                c.cycles += 1;
                c.g[A3][0] = 0x1;
            }
            // Any other table entry keeps the zero result.
            0x0027_02ec => {}
            target => {
                c.jump_out(target, 0x0027_0280, 0x0027_02f4);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    abi::ret(c);
}

/// The three `InSeq_*TrackStop` bodies: release the track object through the
/// sequence's heap and clear the track's eight fields when an owner exists.
pub(crate) fn in_seq_track_stop(c: &mut Cpu, field: u32, clear: &[u32], ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        let owner = lw_at(c, this, 0xc);
        c.g[V0][0] = owner;
        if owner == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A1][0] = lw_at(c, this, field);
            let track = c.g[A1][0];
            if track == 0 {
                Addr(p_6(this, field + 8)).sw(c, 0);
            } else {
                c.cycles += 5;
                c.g[V0][0] = lw_at(c, owner as u32, 8);
                c.g[A0][0] = Addr(p_6(r32(c, V0), 8)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
                call_ptr(c, 0x0027_23a8, ret)?;
                Addr(p_6(this, field + 8)).sw(c, 0);
            }
            c.cycles += 8;
            for &off in clear {
                Addr(p_6(this, off)).sw(c, 0);
            }
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_SpanTrackStop(self)` 0x272ec0: release the span track at `+0x20`.
pub fn in_seq_span_track_stop(c: &mut Cpu) {
    in_seq_track_stop(
        c,
        0x20,
        &[0x8, 0xc, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28],
        0x0027_2ef8,
    )
}

/// `InSeq_EventTrackStop(self)` 0x2732c8: release the event track at `+0x1c`.
pub fn in_seq_event_track_stop(c: &mut Cpu) {
    in_seq_track_stop(
        c,
        0x1c,
        &[0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20],
        0x0027_3300,
    )
}

/// `InSeq_RampTrackStop(self)` 0x273630: release the ramp track at `+0x1c`.
pub fn in_seq_ramp_track_stop(c: &mut Cpu) {
    in_seq_track_stop(
        c,
        0x1c,
        &[0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20],
        0x0027_3668,
    )
}

/// `Player_AllocReplayCache(player)` 0x270070: allocate the first empty replay
/// cache slot (two at `+0x48c`), or return 0 when both are taken.
pub fn player_alloc_replay_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(p_6(this, 0x48c));
        loop {
            let slot = r32(c, S0);
            c.cycles += 3;
            c.g[V0][0] = Addr(slot).lw(c);
            let taken = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if !taken {
                c.cycles += 9;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x17b0));
                c.g[A0][0] = 0x10;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0027_00a8)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0026_d0a0, 0x0027_00b0)?;
                let cache = c.g[V0][0];
                Addr(slot).sw(c, cache);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_0088);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_CountHighlights(replay, bit, positive)` 0x270bd8: count the nodes
/// whose value bit is set and whose value has the requested sign.
pub fn replay_count_highlights(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    let bit = abi::arg(c, 1);
    let positive = abi::arg64(c, 2);
    c.g[A3][0] = lw_at(c, this, 0x3b0);
    c.g[T0][0] = 0;
    if c.g[A3][0] != 0 {
        c.cycles += 3;
        c.g[T1][0] = sx(bit << 2);
        c.g[T2][0] = u64::MAX;
        c.g[V0][0] = Addr(p_6(r32(c, A3), 0x1e)).lbu(c);
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> (bit & 31)) as u32);
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                c.g[A3][0] = Addr(p_6(r32(c, A3), 0x14)).lw(c);
            } else {
                c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, T1)));
                if positive == 0 {
                    c.cycles += 2;
                    c.cycles += 5;
                    c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, T1)));
                    c.g[A0][0] = sx(r32(c, T0).wrapping_add(1));
                    c.g[V1][0] = Addr(p_6(r32(c, V0), 0x20)).lw(c);
                    c.g[V1][0] = ((c.g[T2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[T0][0] = c.g[A0][0];
                    }
                } else {
                    c.cycles += 2;
                    c.cycles += 4;
                    c.g[V0][0] = Addr(p_6(r32(c, V1), 0x20)).lw(c);
                    c.g[V0][0] = ((0i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                }
                c.cycles += 1;
                c.g[A3][0] = Addr(p_6(r32(c, A3), 0x14)).lw(c);
            }
            c.cycles += 2;
            if c.g[A3][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(p_6(r32(c, A3), 0x1e)).lbu(c);
            if event_due(c) {
                deliver_event_here(c, 0x0027_0bf0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T0][0];
    abi::ret(c);
}

/// `Replay_BeginSave(self)` (0x270e68): allocate the save buffers and clear
/// the replay writer state (ending in the `0xdeadbeef` sentinel).
pub fn replay_begin_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0027_0f20, 0x0027_0e7c)?;
        c.g[V0][0] = 0xffff_ffff_dead_0000;
        Addr(p_7(c.g[S0][0] as u32, 0x638)).sw(c, 0);
        c.g[V0][0] = c.g[V0][0] | 0xbeef;
        Addr(p_7(c.g[S0][0] as u32, 0x644)).sw(c, 0);
        Addr(p_7(c.g[S0][0] as u32, 0x684)).sw(c, c.g[V0][0]);
        for off in [
            0x648u32, 0x64c, 0x650, 0x654, 0x658, 0x65c, 0x660, 0x664, 0x66c, 0x674, 0x678, 0x67c,
            0x680,
        ] {
            Addr(p_7(c.g[S0][0] as u32, off)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int InSeq_StartSequence(InSeq* seq, void* src)` (0x272938): take the first
/// free channel node, detach it from the free list, set up the instance and
/// push it onto the used list; returns the channel id.
pub fn in_seq_start_sequence(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        sqsp(c, 16, 0x20);
        c.g[S0][0] = Addr(p_8(r32(c, S1), 0x18)).lw(c);
        c.g[V0][0] = Addr(p_8(c.g[S0][0] as u32, 0x50)).lw(c);
        Addr(p_8(r32(c, S1), 0x18)).sw(c, c.g[V0][0]);
        Addr(p_8(r32(c, S0), 0x50)).sw(c, 0);
        c.g[V0][0] = Addr(p_8(r32(c, S1), 0x18)).lw(c);
        if c.g[V0][0] != 0 {
            Addr(p_8(c.g[V0][0] as u32, 0x54)).sw(c, 0);
        }
        c.cycles += 9;
        c.g[A3][0] = Addr(p_8(r32(c, S0), 4)).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A2][0] = c.g[S1][0];
        callee::IN_SEQ_INSTANCE_SETUP.call(c, 0x0027_2978)?;
        c.g[V1][0] = Addr(p_8(r32(c, S1), 0x1c)).lw(c);
        Addr(p_8(r32(c, S0), 0x50)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(p_8(r32(c, S1), 0x1c)).lw(c);
        if c.g[V0][0] != 0 {
            Addr(p_8(c.g[V0][0] as u32, 0x54)).sw(c, c.g[S0][0]);
        }
        c.cycles += 7;
        Addr(p_8(r32(c, S1), 0x1c)).sw(c, c.g[S0][0]);
        lqsp(c, 17, 0x10);
        c.g[V0][0] = Addr(p_8(r32(c, S0), 4)).lw(c);
        lqsp(c, 16, 0x20);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void Replay_SeekToEnd(self)` (0x270378): find the oldest snapshot (frame
/// limit 100000000), record it and its position, then seek the two stream
/// run lists to that position.
pub fn replay_seek_to_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[A1][0] = 0x5f5_0000;
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.g[A1][0] = c.g[A1][0] | 0xe100;
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, sp_2(c).wrapping_add(0x20));
        c.sq(18, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S1][0] = sx(low(c, S0).wrapping_add(0x48c));
        sw_r(c, S0, 0x604, 0);
        c.g[S2][0] = 0x1;
        callee::REPLAY_FIND_SNAPSHOT_2.call(c, 0x0027_03a8)?;
        sw_r(c, S0, 0x488, c.g[V0][0]);
        c.g[V1][0] = lw_r(c, V0, 0x30);
        sw_r(c, S0, 0x484, c.g[V1][0]);
        c.g[V0][0] = Addr(low(c, S1)).lw(c);
        loop {
            c.cycles += 4;
            c.g[S2][0] = sx(low(c, S2).wrapping_sub(1));
            c.g[S1][0] = sx(low(c, S1).wrapping_add(4));
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !none {
                c.cycles += 2;
                c.g[A1][0] = lw_r(c, S0, 0x484);
                callee::RUN_LIST_SEEK.call(c, 0x0027_03d0)?;
            }
            c.cycles += 2;
            if (c.g[S2][0] as i64) >= 0 {
                c.g[V0][0] = Addr(low(c, S1)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_03b8);
                }
                continue;
            }
            c.cycles += 6;
            c.lq(16, sp_2(c).wrapping_add(0x30));
            c.lq(17, sp_2(c).wrapping_add(0x20));
            c.lq(18, sp_2(c).wrapping_add(0x10));
            c.g[RA][0] = Addr(sp_2(c)).ld(c);
            let t = low(c, RA);
            c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
            c.pc = t;
            break;
        }
        Ok(())
    })
}

/// `void Replay_CheckInstantReplay(self, obj, alt)` (0x2705b0): when a request
/// is pending and the race manager has exactly one rider, fire an instant
/// replay once the recorded span reaches 1000 ms, otherwise clear the request.
pub fn replay_check_instant_replay(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[A3][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[V0][0] = lw_r(c, A3, 0x3d4);
        let idle = c.g[V0][0] == 0;
        c.g[T0][0] = c.g[A1][0];
        if idle {
            c.cycles += 1;
            c.g[RA][0] = Addr(sp_2(c)).ld(c);
        } else {
            c.cycles += 7;
            c.g[A0][0] = Addr(gpo(c, -0x848)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[V0][0] = lw_r(c, A0, 0x84);
            c.g[V1][0] = lw_r(c, V0, 0xc);
            c.g[A0][0] = lw_r(c, V1, 0x7c);
            let multi = c.g[A0][0] != c.g[A1][0];
            c.g[RA][0] = Addr(sp_2(c)).ld(c);
            if !multi {
                let now;
                let start;
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    c.g[V0][0] = lw_r(c, T0, 0xa4);
                    c.cycles += 1;
                    c.g[V1][0] = lw_r(c, A3, 0x47c);
                    now = c.g[V0][0];
                    start = c.g[V1][0];
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw_r(c, T0, 0x9c);
                    c.g[V1][0] = lw_r(c, A3, 0x474);
                    now = c.g[V0][0];
                    start = c.g[V1][0];
                }
                c.cycles += 4;
                c.g[V0][0] = sx((now as u32).wrapping_sub(start as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 1000) as u64;
                if c.g[V0][0] != 0 {
                    sw_r(c, A3, 0x3d4, 0);
                    c.cycles += 1;
                    c.g[RA][0] = Addr(sp_2(c)).ld(c);
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[A3][0];
                    callee::REPLAY_REQUEST_INSTANT_REPLAY.call(c, 0x0027_0614)?;
                    c.g[RA][0] = Addr(sp_2(c)).ld(c);
                }
            }
        }
        c.cycles += 2;
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// In-sequence channels.

/// `void* InSeq_ChannelSetListener(InSeqChannel* self, void* listener)`
/// (0x274240): release the previous listener, store the new one and hand it
/// its owner; returns the previous listener.
pub fn in_seq_channel_set_listener(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = Addr(p_9(r32(c, S0), 0x2c)).lw(c);
        let null = c.g[S2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 8;
            c.g[V0][0] = Addr(r32(c, S2)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = Addr(p_9(r32(c, S0), 0x1c)).lw(c);
            c.g[A0][0] = Addr(p_9(r32(c, V0), 0x10)).lh(c);
            c.g[A3][0] = Addr(p_9(r32(c, V0), 0x14)).lw(c);
            c.g[A2][0] = Addr(p_9(r32(c, V1), 3)).lbu(c);
            let target = r32(c, A3);
            c.g[RA][0] = sx(0x0027_4284);
            c.g[A0][0] = sx(p_9(r32(c, S2), r32(c, A0)));
            call_ptr(c, target, 0x0027_4284)?;
        }
        c.cycles += 2;
        let none = c.g[S1][0] == 0;
        Addr(p_9(r32(c, S0), 0x2c)).sw(c, c.g[S1][0]);
        if !none {
            c.cycles += 8;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = Addr(p_9(r32(c, S0), 0x1c)).lw(c);
            c.g[A0][0] = Addr(p_9(r32(c, V0), 8)).lh(c);
            c.g[A3][0] = Addr(p_9(r32(c, V0), 0xc)).lw(c);
            c.g[A2][0] = Addr(p_9(r32(c, V1), 3)).lbu(c);
            let target = r32(c, A3);
            c.g[RA][0] = sx(0x0027_42ac);
            c.g[A0][0] = sx(p_9(r32(c, S1), r32(c, A0)));
            call_ptr(c, target, 0x0027_42ac)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay curve tracks.

/// `int InSeq_CurveTrackStart(void* self, CurveTrack* data, void* owner)`
/// (0x2725f0): attach a curve track (its key/value table and first segment)
/// to the in-sequence object and evaluate the first value.
pub fn in_seq_curve_track_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = sx(r32(c, A1).wrapping_add(4));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p_10(r32(c, S0), 0xc)).sw(c, c.g[A2][0]);
        Addr(p_10(r32(c, S0), 8)).sw(c, c.g[A1][0]);
        Addr(p_10(r32(c, S0), 0x10)).sw(c, c.g[A1][0]);
        Addr(p_10(r32(c, S0), 0x14)).sw(c, c.g[V1][0]);
        Addr(p_10(r32(c, S0), 0x18)).sw(c, 0);
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        if c.g[V0][0] != 0 {
            Addr(p_10(r32(c, S0), 0x24)).sw(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_10(r32(c, A1), 4)).lhu(c);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            Addr(p_10(r32(c, S0), 0x24)).sw(c, c.g[V0][0]);
        }
        c.cycles += 10;
        c.g[A0][0] = Addr(p_10(r32(c, S0), 0x14)).lw(c);
        c.mtc1(0, 12);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        callee::MATH_EVAL_CUBIC.call(c, 0x0027_2644)?;
        c.g[V1][0] = Addr(p_10(r32(c, S0), 0x14)).lw(c);
        c.swc1(0, p_10(r32(c, S0), 0x1c));
        c.g[V0][0] = Addr(r32(c, V1)).lhu(c);
        c.g[V0][0] = (c.g[V0][0] < 0x15u64) as u64;
        if c.g[V0][0] != 0 {
            Addr(p_10(r32(c, S0), 0x20)).sw(c, 0);
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x14));
            Addr(p_10(r32(c, S0), 0x20)).sw(c, c.g[V0][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(p_10(r32(c, S0), 0x24)).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `Highlight* Replay_FindWeakestHighlight(Replay* self, int i, Highlight*
/// best, int positive)` (0x270c58): scan the highlight list for the entry
/// with the weakest value in column `i` (`positive`: the strongest).
pub fn replay_find_weakest_highlight(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T0][0] = c.lw(p_11(r32(c, A0), 0x3b0));
    let empty = c.g[T0][0] == 0;
    c.g[T1][0] = c.g[A2][0];
    if !empty {
        c.cycles += 1;
        c.g[V0][0] = c.lbu(at(c, T0, 0x1e));
        loop {
            c.cycles += 4;
            let shift = r32(c, A1) & 31;
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> shift) as u32);
            c.g[V0][0] = c.g[V0][0] & 1;
            if c.g[V0][0] == 0 {
                c.g[T0][0] = c.lw(at(c, T0, 0x14));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(at(c, T0, 0x1f));
                if c.g[V0][0] != 0 {
                    c.g[T0][0] = c.lw(at(c, T0, 0x14));
                } else {
                    // Take the element or skip it: `land` is where the
                    // original's `break` goes (1: re-test V0, 2: skip).
                    let next = c.lw(at(c, T0, 0x14));
                    c.cycles += 2;
                    let negative = c.g[A3][0] == 0;
                    c.g[V1][0] = sx(r32(c, A1) << 2);
                    let mut land = 0u8;
                    if negative {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
                        c.g[A0][0] = c.lw(at(c, V0, 0x20));
                        if (c.g[A0][0] as i64) >= 0 {
                            land = 2;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
                            c.g[V0][0] = c.lw(at(c, V1, 0x20));
                            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            land = 1;
                        }
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
                        c.g[A0][0] = c.lw(at(c, V0, 0x20));
                        let not_positive = (c.g[A0][0] as i64) <= 0;
                        c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
                        if not_positive {
                            land = 2;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(at(c, V1, 0x20));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                            land = 1;
                        }
                    }
                    if land == 1 {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.g[T0][0] = next;
                        } else {
                            c.cycles += 2;
                            c.g[T1][0] = c.g[T0][0];
                            c.g[A2][0] = c.g[T1][0];
                            c.cycles += 1;
                            c.g[T0][0] = next;
                        }
                    } else if land == 2 {
                        c.cycles += 1;
                        c.g[T0][0] = next;
                    } else {
                        c.g[T0][0] = next;
                    }
                }
            }
            c.cycles += 2;
            if c.g[T0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lbu(at(c, T0, 0x1e));
            if event_due(c) {
                deliver_event_here(c, 0x0027_0c68);
            }
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[V0][0] = c.g[T1][0];
    c.pc = ra;
}

/// `void Replay_RecycleOldestSnapshot(Replay* self)` (0x2701a0): free the
/// oldest unlocked, unused snapshot in a two-pass walk of the ring.
pub fn replay_recycle_oldest_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0027_01a0 => {
                    c.cycles += 4;
                    let frame = Frame::open(c, 0x10);
                    c.g[A2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[A3][0] = 0;
                    pc = 0x0027_01b0;
                }
                0x0027_01b0 => {
                    c.cycles += 2;
                    c.g[V0][0] = lw_2(c, A2, 0x3ac);
                    pc = 0x0027_01f4;
                }
                0x0027_01b8 => {
                    c.cycles += 4;
                    c.g[A1][0] = lw_2(c, V0, 0x14);
                    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x1f).lbu(c);
                    let used = c.g[V1][0] != 0;
                    c.g[A0][0] = c.g[V0][0];
                    pc = if used { 0x0027_01ec } else { 0x0027_01c8 };
                }
                0x0027_01c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x1e).lbu(c);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw_2(c, A0, 0x14);
                        pc = 0x0027_01f0;
                    } else {
                        pc = 0x0027_01d4;
                    }
                }
                0x0027_01d4 => {
                    c.cycles += 6;
                    c.g[V0][0] = lw_2(c, A1, 0x14);
                    c.g[A0][0] = c.g[A2][0];
                    Addr(c.g[A2][0] as u32).off(0x3ac).sw(c, c.g[V0][0]);
                    callee::REPLAY_FREE_SNAPSHOT.call(c, 0x0027_01e4)?;
                    c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
                    pc = 0x0027_022c;
                }
                0x0027_01ec => {
                    c.cycles += 1;
                    c.g[V0][0] = lw_2(c, A0, 0x14);
                    pc = 0x0027_01f0;
                }
                0x0027_01f0 => {
                    c.cycles += 1;
                    Addr(c.g[A2][0] as u32).off(0x3ac).sw(c, c.g[V0][0]);
                    pc = 0x0027_01f4;
                }
                0x0027_01f4 => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = lw_2(c, A2, 0x3b0);
                        pc = 0x0027_0218;
                    } else {
                        pc = 0x0027_01fc;
                    }
                }
                0x0027_01fc => {
                    c.cycles += 3;
                    c.g[V0][0] = lw_2(c, V0, 0x14);
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = lw_2(c, A2, 0x3b0);
                        pc = 0x0027_0218;
                    } else {
                        pc = 0x0027_0208;
                    }
                }
                0x0027_0208 => {
                    c.cycles += 3;
                    c.g[V0][0] = lw_2(c, V0, 0x14);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw_2(c, A2, 0x3ac);
                        if back_edge_unwinds(c, 0x0027_01b8) {
                            return Ok(());
                        }
                    }
                    pc = 0x0027_01b8;
                }
                0x0027_0214 => {
                    c.cycles += 1;
                    c.g[V1][0] = lw_2(c, A2, 0x3b0);
                    pc = 0x0027_0218;
                }
                0x0027_0218 => {
                    c.cycles += 4;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
                    let again = c.g[V0][0] != 0;
                    Addr(c.g[A2][0] as u32).off(0x3ac).sw(c, c.g[V1][0]);
                    if again && back_edge_unwinds(c, 0x0027_01b0) {
                        return Ok(());
                    }
                    pc = if again { 0x0027_01b0 } else { 0x0027_0228 };
                }
                0x0027_0228 => {
                    c.cycles += 1;
                    c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
                    pc = 0x0027_022c;
                }
                0x0027_022c => {
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0027_01a0, 0x0027_0234);
                    return Ok(());
                }
            }
        }
    })
}

/// `Replay_FlushSaveOutput` (0x270fc0): append the pending replay snapshot
/// bytes to the file buffer (at most 0x4000 per call) and advance the source
/// and count fields.
pub fn replay_flush_save_output(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(p_12(this, 0x644)).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.g[A2][0] = Addr(p_12(this, 0x648)).lw(c);
            let empty = (c.g[A2][0] as i64) <= 0;
            c.g[V1][0] = 0x4000;
            if empty {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(p_12(this, 0x64c)).lw(c);
                c.g[V1][0] = sx(0x4000u32.wrapping_sub(r(c, V0)));
                c.g[A0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                c.g[S1][0] = c.g[V1][0];
                if c.g[A0][0] == 0 {
                    c.g[S1][0] = c.g[A2][0];
                }
                if (c.g[S1][0] as i64) <= 0 {
                    frame.restore(c, S0, 0x20);
                } else {
                    c.cycles += 15;
                    c.g[A0][0] = Addr(p_12(this, 0x650)).lw(c);
                    c.g[RA][0] = sx(0x0027_1014);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, callee::MEMCPY_2, 0x0027_1014)?;
                    c.g[V1][0] = Addr(p_12(this, 0x650)).lw(c);
                    c.g[A0][0] = Addr(p_12(this, 0x644)).lw(c);
                    c.g[A1][0] = Addr(p_12(this, 0x648)).lw(c);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, S1)));
                    c.g[V0][0] = Addr(p_12(this, 0x64c)).lw(c);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S1)));
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(r(c, S1)));
                    Addr(p_12(this, 0x650)).sw(c, c.g[V1][0]);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                    Addr(p_12(this, 0x644)).sw(c, c.g[A0][0]);
                    Addr(p_12(this, 0x64c)).sw(c, c.g[V0][0]);
                    Addr(p_12(this, 0x648)).sw(c, c.g[A1][0]);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                }
            }
        } else {
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_SaveBuildHeader` (0x271228): write the replay header (magic, build
/// id, world/rider fields) into the save buffer and reset the write cursor.
pub fn replay_save_build_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x20);
        c.g[RA][0] = sx(0x0027_1244);
        c.g[S0][0] = Addr(p_12(this, 0x640)).lw(c);
        callee::REPLAY_COUNT_SAVE_SNAPSHOTS.call(c, 0x0027_1244)?;
        let buf = r(c, S0);
        Addr(p_12(buf, 4)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0xffff_ffff_abcd_0000;
        c.g[A3][0] = Addr(gp_2(c, -0x848)).lw(c);
        c.g[A2][0] = c.g[A2][0] | 2;
        c.g[V1][0] = Addr(p_12(this, 0x18)).lw(c);
        c.g[A1][0] = sx(buf.wrapping_add(0x20));
        Addr(p_12(buf, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(p_12(this, 0x610)).lw(c);
        Addr(p_12(buf, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(p_12(this, 0x3c8)).lw(c);
        Addr(p_12(buf, 0x10)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(p_12(this, 0x3cc)).lw(c);
        Addr(p_12(buf, 0x14)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(p_12(r(c, A3), 0x84)).lw(c);
        c.g[V0][0] = Addr(p_12(r(c, V1), 0xc)).lw(c);
        c.g[A0][0] = Addr(p_12(r(c, V0), 0x7c)).lw(c);
        Addr(buf).sw(c, c.g[A2][0]);
        Addr(p_12(buf, 0x18)).sw(c, c.g[A0][0]);
        c.g[V0][0] = Addr(p_12(r(c, A3), 0x84)).lw(c);
        c.g[RA][0] = sx(0x0027_1298);
        c.g[A0][0] = Addr(p_12(r(c, V0), 0xc)).lw(c);
        call_ptr(c, callee::REPLAY_GET_WORLD, 0x0027_1298)?;
        c.g[V0][0] = Addr(p_12(r(c, S1), 0x638)).lw(c);
        Addr(p_12(r(c, S1), 0x644)).sw(c, 0);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
        Addr(p_12(r(c, S1), 0x648)).sw(c, 0);
        Addr(p_12(r(c, S1), 0x638)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_InstanceGetLength` (0x274c70): total length of the instance's
/// channels (sum over the channel records), cached at `+0x30`.
pub fn in_seq_instance_get_length(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_12(this, 0x30)).lw(c);
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 5;
            c.g[V1][0] = Addr(p_12(this, 0x10)).lw(c);
            c.g[S3][0] = 0;
            c.g[V0][0] = Addr(p_12(r(c, V1), 1)).lbu(c);
            let no_channels = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if !no_channels {
                c.cycles += 1;
                c.g[S2][0] = 0;
                loop {
                    c.cycles += 11;
                    c.g[A0][0] = Addr(p_12(this, 0x18)).lw(c);
                    c.g[S3][0] = sx(r(c, S3).wrapping_add(1));
                    c.g[RA][0] = sx(0x0027_4cc0);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S2)));
                    callee::IN_SEQ_CHANNEL_GET_LENGTH.call(c, 0x0027_4cc0)?;
                    c.g[S2][0] = sx(r(c, S2).wrapping_add(0x3c));
                    c.g[A1][0] = Addr(p_12(this, 0x10)).lw(c);
                    c.g[A0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[V1][0] = Addr(p_12(r(c, A1), 1)).lbu(c);
                    c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    if c.g[A0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_4cb0);
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p_12(this, 0x20)).lw(c);
            if c.g[V0][0] != 0 {
                Addr(p_12(this, 0x30)).sw(c, c.g[S0][0]);
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[S0][0];
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// MenuSys_SetActive's direct callee is defined in `callee` above.

/// `int InSeq_ChannelFlushEvents(self)` (0x273d20): tell the channel's two
/// listeners to drop their events and clear the queued event count.
pub fn in_seq_channel_flush_events(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[T1][0] = lw_3(c, S0, 0x2c);
        let has = c.g[T1][0] != 0;
        c.g[S1][0] = lw_3(c, S0, 0x30);
        if has {
            c.cycles += 3;
            c.g[A3][0] = lw_3(c, S0, 0x34);
            let handler = c.g[A3][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if handler {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                c.g[V1][0] = lw_3(c, S0, 0x1c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lh(c);
                c.g[T0][0] = lw_3(c, V0, 0x1c);
                c.g[A2][0] = Addr(c.g[V1][0] as u32).off(3).lbu(c);
                c.mtc1(0, 12);
                let target = c.g[T0][0] as u32;
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_3d6c)?;
            }
            c.cycles += 12;
            c.g[A3][0] = lw_3(c, S0, 0x2c);
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = lw_3(c, S0, 0x1c);
            c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
            c.mtc1(0, 12);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
            c.g[V1][0] = lw_3(c, V0, 0x24);
            c.mov_s(13, 12);
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A2][0] = Addr(c.g[T0][0] as u32).off(3).lbu(c);
            let target = c.g[V1][0] as u32;
            c.g[A3][0] = lw_3(c, S0, 0x34);
            call_ptr(c, target, 0x0027_3d9c)?;
        }
        c.cycles += 9;
        sw_2(c, S0, 0x38, 0);
        c.g[V0][0] = c.g[S1][0];
        sw_2(c, S0, 0x30, 0);
        sw_2(c, S0, 0x34, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InSeq: unlink and free a sequence instance.

/// `void InSeq_FreeSequence(mgr, handle)` (0x2729a8): stop the sequence,
/// release its instance and unlink it from the active and idle lists.
pub fn in_seq_free_sequence(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'outer: {
            'mid: {
                'inner: {
                    c.cycles += 16;
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save_ra(c, 0);
                    c.g[RA][0] = sx(0x0027_29c4);
                    c.g[S1][0] = c.g[A0][0];
                    callee::INSEQ_STOP.call(c, 0x0027_29c4)?;
                    c.g[V0][0] = 0x58;
                    c.g[V1][0] = lw(c, at_3(c.g[S1][0], 0x14));
                    c.mult(0, 16, 16, 2);
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[RA][0] = sx(0x0027_29dc);
                    c.g[A0][0] = sx(at_3(c.g[S0][0], 8));
                    callee::INSEQ_INSTANCE_RELEASE.call(c, 0x0027_29dc)?;
                    c.g[V1][0] = lw(c, at_3(c.g[S0][0], 0x54));
                    let t = c.g[V1][0] != 0;
                    c.g[V0][0] = lw(c, at_3(c.g[S0][0], 0x50));
                    if t {
                        break 'inner;
                    }
                    c.cycles += 5;
                    Addr(at_3(c.g[S1][0], 0x1c)).sw(c, c.g[V0][0]);
                    Addr(at_3(c.g[S0][0], 0x50)).sw(c, 0);
                    c.g[V0][0] = lw(c, at_3(c.g[S1][0], 0x1c));
                    if c.g[V0][0] != 0 {
                        Addr(at_3(c.g[V0][0], 0x54)).sw(c, 0);
                        break 'mid;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = lw(c, at_3(c.g[S1][0], 0x18));
                    break 'outer;
                }
                {
                    c.cycles += 4;
                    Addr(at_3(c.g[V1][0], 0x50)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = lw(c, at_3(c.g[S0][0], 0x50));
                    if c.g[V1][0] == 0 {
                        Addr(at_3(c.g[S0][0], 0x50)).sw(c, 0);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, at_3(c.g[S0][0], 0x54));
                        Addr(at_3(c.g[V1][0], 0x54)).sw(c, c.g[V0][0]);
                        Addr(at_3(c.g[S0][0], 0x50)).sw(c, 0);
                    }
                }
                c.cycles += 1;
                Addr(at_3(c.g[S0][0], 0x54)).sw(c, 0);
                break 'mid;
            }
            c.cycles += 1;
            c.g[V0][0] = lw(c, at_3(c.g[S1][0], 0x18));
            break 'outer;
        }
        {
            c.cycles += 4;
            Addr(at_3(c.g[S0][0], 0x50)).sw(c, c.g[V0][0]);
            c.g[V0][0] = lw(c, at_3(c.g[S1][0], 0x18));
            if c.g[V0][0] != 0 {
                Addr(at_3(c.g[V0][0], 0x54)).sw(c, c.g[S0][0]);
            }
        }
        c.cycles += 6;
        Addr(at_3(c.g[S1][0], 0x18)).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
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
                                        if crate::events::event_due(c) {
                                            crate::events::deliver_event_here(c, 0x00273278);
                                        }
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
        c.pc = t;
        return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sequencing and script processes.

/// `int InSeq_SpanTrackStart(SpanTrack *self, u16 *key, void *owner,
/// u32 *flags)` (0x272df0): start playing a span track, allocating and
/// activating its first key. Returns whether the track has a first key.
pub fn inseq_span_track_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p_14(c.g[S0][0], 0x8)).sw(c, c.g[A1][0]);
        Addr(p_14(c.g[S0][0], 0xc)).sw(c, c.g[A2][0]);
        Addr(p_14(c.g[S0][0], 0x10)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lhu(c);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A3][0];
        let mut extra = true;
        if !empty {
            c.cycles += 14;
            c.g[V1][0] = Addr(p_14(c.g[A1][0], 0x2)).lhu(c);
            c.g[A1][0] = 0x1c;
            c.g[V0][0] = Addr(p_14(c.g[A2][0], 0x8)).lw(c);
            abi::mult(c, c.g[V1][0] as u32, c.g[A1][0] as u32);
            c.g[A0][0] = Addr(p_14(c.g[V0][0], 0x8)).lw(c);
            c.g[A1][0] = c.lo[0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
            callee::ISEQ_HEAP_ALLOC.call(c, 0x0027_2e3c)?;
            c.g[V1][0] = Addr(p_14(c.g[S0][0], 0x10)).lw(c);
            Addr(p_14(c.g[S0][0], 0x20)).sw(c, c.g[V0][0]);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            Addr(p_14(c.g[S0][0], 0x18)).sw(c, 0);
            let none = c.g[V0][0] == 0;
            Addr(p_14(c.g[S0][0], 0x14)).sw(c, c.g[V0][0]);
            if !none {
                c.cycles += 3;
                c.g[V0][0] = Addr(p_14(c.g[V1][0], 0x8)).lhu(c);
                if c.g[V0][0] != 0 {
                    extra = false;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    loop {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[V0][0] |= 0x4;
                        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
                        c.g[A1][0] = Addr(p_14(c.g[S0][0], 0x14)).lw(c);
                        callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY.call(c, 0x0027_2e7c)?;
                        c.g[A0][0] = c.g[S0][0];
                        callee::ISEQ_SPAN_TRACK_NEXT_KEY.call(c, 0x0027_2e84)?;
                        c.g[V0][0] = Addr(p_14(c.g[S0][0], 0x14)).lw(c);
                        if c.g[V0][0] == 0 {
                            extra = false;
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p_14(c.g[V0][0], 0x4)).lhu(c);
                        if c.g[V0][0] != 0 {
                            break;
                        }
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_2e68);
                        }
                    }
                }
            }
        }
        if extra {
            c.cycles += 1;
        }
        c.g[V1][0] = Addr(p_14(c.g[S0][0], 0x10)).lw(c);
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = (c.g[V0][0] > 0) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay hooks.

/// `void Replay_OnRiderEvent(Replay* self, int rider, int kind)` (0x2709d8):
/// ignore the event while the replay is busy; otherwise find the rider and
/// highlight the matching checkpoint bit, or empty the rider hook.
pub fn replay_on_rider_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[V0][0] = Addr(p_15(r32(c, S0), 0x61c)).lw(c);
        let busy = c.g[V0][0] != 0;
        let mut empty = false;
        let mut highlight = false;
        let mut ret = false;
        if !busy {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_15(r32(c, S0), 0x620)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(p_15(r32(c, S0), 0x610)).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p_15(r32(c, S0), 0x60c)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                        if c.g[V0][0] == 0 {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x0027_0730, 0x0027_0a38)?;
                            c.g[A3][0] = c.g[V0][0];
                            let negative = (c.g[A3][0] as i64) < 0;
                            c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
                            if negative {
                                ret = true;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = sx(r32(c, S1) << 2);
                                c.g[V1][0] = Addr(p_15(r32(c, A0), 0x84)).lw(c);
                                c.g[V0][0] = Addr(p_15(r32(c, V1), 0xc)).lw(c);
                                c.g[V0][0] = sx(p_15(r32(c, V0), r32(c, A1)));
                                let cond = c.g[S2][0] == 0;
                                c.g[A0][0] = Addr(p_15(r32(c, V0), 0x28)).lw(c);
                                if cond {
                                    empty = true;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 1;
                                    if c.g[S2][0] == c.g[V0][0] {
                                        c.g[A2][0] = Addr(p_15(r32(c, A0), 0x790)).lw(c);
                                        highlight = true;
                                    } else {
                                        c.cycles += 2;
                                        ret = true;
                                    }
                                }
                            }
                        } else {
                            ret = true;
                        }
                    } else {
                        ret = true;
                    }
                } else {
                    ret = true;
                }
            } else {
                ret = true;
            }
        } else {
            ret = true;
        }
        if empty {
            c.cycles += 6;
            c.g[A1][0] = Addr(p_15(r32(c, A0), 0x790)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(p_15(r32(c, A1), 0xfc));
            call_ptr(c, 0x0027_05a8, 0x0027_0a80)?;
            ret = true;
        } else if highlight {
            c.cycles += 4;
            c.g[A1][0] = c.g[A3][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx(p_15(r32(c, A2), 0xfc));
            call_ptr(c, 0x0027_0b88, 0x0027_0a98)?;
            ret = true;
        }
        if !ret {
            // (every remaining arm already loaded s0 and returned)
        }
        if busy || highlight {
            c.cycles += 1;
        }
        c.lq(S0, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_OnRiderCheckpoint(Replay* self, int rider, int bit)` (0x270ab0):
/// set the rider's checkpoint bit unless the replay is busy, the rider is
/// invalid or the bit is already set; clears the "pending" flag.
pub fn replay_on_rider_checkpoint(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p_15(r32(c, S0), 0x61c)).lw(c);
        c.g[S1][0] = c.g[A2][0];
        let mut plus_one = false;
        'checks: {
            if c.g[V0][0] != 0 {
                plus_one = true;
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p_15(r32(c, S0), 0x620)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p_15(r32(c, S0), 0x610)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p_15(r32(c, S0), 0x60c)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 5;
            call_ptr(c, 0x0027_0730, 0x0027_0b0c)?;
            c.g[A2][0] = c.g[V0][0];
            if (c.g[A2][0] as i64) < 0 {
                break 'checks;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                break 'checks;
            }
            c.cycles += 4;
            c.g[V0][0] = Addr(p_15(r32(c, S0), 0x10)).lw(c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let in_range = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32(c, A2) << 2);
            if !in_range {
                break 'checks;
            }
            c.cycles += 8;
            c.g[V1][0] = sx(p_15(r32(c, S0), 0x5fc));
            c.g[A3][0] = sx(p_15(r32(c, V1), r32(c, A0)));
            c.g[V0][0] = Addr(r32(c, A3)).lw(c);
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> (r32(c, S1) & 31)) as u32);
            c.g[V0][0] = c.g[V0][0] ^ 1;
            c.g[V0][0] = c.g[V0][0] & 1;
            let clear = c.g[V0][0] == 0;
            c.g[A0][0] = 1;
            if clear {
                break 'checks;
            }
            c.cycles += 9;
            c.g[A1][0] = Addr(p_15(r32(c, S0), 0x3b4)).lw(c);
            c.g[V1][0] = sx(r32(c, A0) << (r32(c, A2) & 31));
            c.g[V0][0] = Addr(p_15(r32(c, A1), 0x1f)).lbu(c);
            c.g[A0][0] = sx(r32(c, A0) << (r32(c, S1) & 31));
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            Addr(p_15(r32(c, A1), 0x1f)).sb(c, c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, A3)).lw(c);
            c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
            Addr(r32(c, A3)).sw(c, c.g[V1][0]);
            plus_one = true;
        }
        if plus_one {
            c.cycles += 1;
        }
        c.lq(S0, frame.at(c, 0x20));
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float InSeq_CurveTrackUpdate(InSeqCurveTrack* self, float t)`:
/// advance the curve track's playhead by `t` keys, evaluate the cubic through
/// the four surrounding keys and return the value; 0 when the track ended.
pub fn in_seq_curve_track_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_272778: {
            c.cycles += 8;
            c.g[SP][0] = sub(c.g[SP][0], 0x20);
            c.mov_s(1, 12);
            c.sq(16, p_16(c.g[SP][0] as u32, 0x10));
            c.sd(p_16(c.g[SP][0] as u32, 0), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[A0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::CURVE));
            let ended = c.g[A0][0] == 0;
            c.g[V0][0] = 0;
            if ended {
                break 'b_272778;
            }
            'b_272758: {
                c.cycles += 7;
                c.g[V0][0] = c.lhu(p_16(c.g[A0][0] as u32, 2));
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.c_le_s(0, 1);
                if !c.fcc() {
                    c.g[A0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                    break 'b_272758;
                }
                'b_272714: {
                    c.cycles += 9;
                    c.g[V1][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::KEY));
                    sw_4(
                        c,
                        p_16(c.g[S0][0] as u32, in_seq_curve::SEGMENT),
                        c.g[A0][0],
                    );
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    sw_4(c, p_16(c.g[S0][0] as u32, in_seq_curve::CURSOR), 0);
                    sw_4(c, p_16(c.g[S0][0] as u32, in_seq_curve::KEY), c.g[V1][0]);
                    c.g[V0][0] = c.lhu(c.g[A0][0] as u32);
                    c.g[V0][0] = (c.g[V0][0] < 0x15) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::LENGTH));
                        break 'b_272714;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                    sw_4(c, p_16(c.g[S0][0] as u32, in_seq_curve::CURSOR), c.g[V0][0]);
                    c.g[V0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::LENGTH));
                    break 'b_272714;
                }
                'b_272748: {
                    c.cycles += 5;
                    c.g[A0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::KEY));
                    c.g[V1][0] = lw(c, c.g[V0][0] as u32);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    if c.g[A0][0] != c.g[V1][0] {
                        c.g[V0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                        break 'b_272748;
                    }
                    c.cycles += 8;
                    c.g[A0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                    sw_4(c, p_16(c.g[S0][0] as u32, in_seq_curve::CURVE), 0);
                    c.mtc1(0, 12);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                    call_ptr(c, 0x0027_2570, 0x0027_273c)?;
                    c.swc1(0, p_16(c.g[S0][0] as u32, in_seq_curve::VALUE));
                    c.g[V0][0] = 0;
                    break 'b_272778;
                }
                c.cycles += 4;
                c.g[V1][0] = c.lhu(c.g[V0][0] as u32);
                c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
                sw_4(c, p_16(c.g[S0][0] as u32, in_seq_curve::CURVE), c.g[V0][0]);
                c.g[A0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                break 'b_272758;
            }
            c.cycles += 8;
            c.g[V0][0] = c.lhu(p_16(c.g[A0][0] as u32, 2));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.mtc1(2, 12);
            c.cvt_s_w(12, 12);
            c.sub_s(12, 1, 12);
            call_ptr(c, 0x0027_2570, 0x0027_2770)?;
            c.swc1(0, p_16(c.g[S0][0] as u32, in_seq_curve::VALUE));
            c.g[V0][0] = 1;
            break 'b_272778;
        }
        c.cycles += 4;
        c.lq(16, p_16(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p_16(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// InSeq_HeapFree (0x00002723a8).

/// `InSeq_HeapFree`: return a block to an InSeq heap, coalescing with the neighbouring free blocks.
pub fn in_seq_heap_free(c: &mut Cpu) {
    guest(c, |c| {
        'b_272480: {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;

            if t {
                break 'b_272480;
            } else {
                'b_2723bc: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x8));
                    if t {
                        c.cycles += 3;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[T0][0] = 0u64;
                        c.g[V0][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
                        'l_2723d0: loop {
                            'b_272478: {
                                'b_272470: {
                                    'b_27241c: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                            break 'b_27241c;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A3][0] as u32)));
                                            let t = c.g[V1][0] != c.g[A2][0];

                                            if t {
                                                c.cycles += 2;
                                                if c.g[T0][0] == 0u64 {
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                                    break 'b_272470;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                    if t {
                                                        break 'b_272478;
                                                    } else {
                                                        break 'b_27241c;
                                                    }
                                                }
                                            } else {
                                                c.cycles += 6;
                                                c.g[V1][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                                c.sw(
                                                    (c.g[A1][0] as u32).wrapping_add(0x4),
                                                    c.g[V1][0],
                                                );
                                                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                                c.g[V0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                let t = c.g[T0][0] == 0u64;
                                                c.sw((c.g[A1][0] as u32), c.g[V0][0]);
                                                if t {
                                                    break 'b_2723bc;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[RA][0] as u32;
                                                    c.sw(
                                                        (c.g[T0][0] as u32).wrapping_add(0x4),
                                                        c.g[A1][0],
                                                    );
                                                    c.pc = t;
                                                    return Ok(());
                                                }
                                            }
                                        }
                                    }
                                    'b_27245c: {
                                        'b_272458: {
                                            c.cycles += 4;
                                            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A3][0] as u32)));
                                            if c.g[V1][0] != c.g[A1][0] {
                                                c.g[T0][0] = c.g[A2][0];
                                                break 'b_272458;
                                            } else {
                                                'b_272450: {
                                                    c.cycles += 7;
                                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                                    c.g[A1][0] = c.g[A2][0];
                                                    c.g[V0][0] = sx((c.g[A3][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.sw((c.g[A2][0] as u32), c.g[V0][0]);
                                                    c.g[A2][0] =
                                                        c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                                    let t = c.g[T0][0] != 0u64;
                                                    c.sw(
                                                        (c.g[A1][0] as u32).wrapping_add(0x4),
                                                        0u64,
                                                    );
                                                    if t {
                                                        break 'b_272450;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                            c.g[A2][0],
                                                        );
                                                        if t {
                                                            break 'b_27245c;
                                                        } else {
                                                            break 'b_272450;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.sw(
                                                    (c.g[T0][0] as u32).wrapping_add(0x4),
                                                    c.g[A2][0],
                                                );
                                                if t {
                                                    break 'b_27245c;
                                                } else {
                                                    break 'b_272458;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                        break 'b_27245c;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[A2][0] != 0u64;
                                    c.g[V0][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002723d0);
                                        }
                                        continue 'l_2723d0;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[T0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                            break 'b_272478;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                            break 'b_272470;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                if t {
                                    break 'b_2723bc;
                                } else {
                                    break 'b_272478;
                                }
                            }
                            c.cycles += 2;
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                            break 'b_272480;
                        }
                    } else {
                        break 'b_2723bc;
                    }
                }
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.pc = t;
                return Ok(());
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        return Ok(());
        Ok(())
    });
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
            c.g[A1][0] = lw_5(c, at(c, S0, 0x484));
            if callee::C0026e448.call(c, 0x00270418).is_err() {
                return;
            }
            sw_5(c, at(c, S0, 0x488), c.g[V0][0]);
            break 'b_27041c;
        }
    }
    'b_27043c: {
        c.cycles += 3;
        c.g[V1][0] = lw_5(c, at(c, S0, 0x488));
        let t = c.g[V1][0] == 0u64;

        if t {
            break 'b_27043c;
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw_5(c, at(c, S0, 0x3d0));
            let t = c.g[V1][0] != c.g[V0][0];

            if t {
                break 'b_27043c;
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw_5(c, at(c, V1, 0x18));
                sw_5(c, at(c, S0, 0x488), c.g[V0][0]);
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
                c.g[V1][0] = lw_5(c, at(c, S0, 0x488));
                c.g[S2][0] = 0x1;
                c.g[V0][0] = lw_5(c, at(c, V1, 0x30));
                sw_5(c, at(c, S0, 0x484), c.g[V0][0]);
                'l_270458: loop {
                    'b_270470: {
                        c.cycles += 3;
                        c.g[A0][0] = lw_5(c, r32(c, S1));
                        if c.g[A0][0] == 0u64 {
                            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(0x1));
                            break 'b_270470;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x0027046c);
                            c.g[A1][0] = lw_5(c, at(c, S0, 0x484));
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
                        c.g[A1][0] = lw_5(c, at(c, A0, 0x488));
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
            c.g[V1][0] = lw_5(c, at(c, S0, 0x484));
            c.g[A0][0] = lw_5(c, at(c, S0, 0x3cc));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = sx(at(c, V1, 0x1));
            if t {
                break 'b_270518;
            } else {
                'b_270504: {
                    c.cycles += 3;
                    c.g[V0][0] = lw_5(c, at(c, S0, 0x61c));
                    if c.g[V0][0] == 0u64 {
                        sw_5(c, at(c, S0, 0x484), c.g[A0][0]);
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
                        c.g[V0][0] = lw_5(c, r32(c, GP).wrapping_sub(0x848));
                        c.g[V1][0] = lw_5(c, at(c, V0, 0x84));
                        c.g[S1][0] = lw_5(c, at(c, V1, 0x84));
                        c.g[V0][0] = lw_5(c, at(c, S1, 0x4));
                        c.g[S0][0] = lw_5(c, at(c, V0, 0xa8));
                        c.g[RA][0] = sx(0x002704e0);
                        c.g[A0][0] = c.g[S0][0];
                        if call_ptr(c, 0x001620d0, 0x002704e0).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw_5(c, at(c, S0, 0x14));
                        c.g[A0][0] = lh(c, at(c, V0, 0x20));
                        c.g[V1][0] = lw_5(c, at(c, V0, 0x24));
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
        sw_5(c, at(c, S0, 0x484), c.g[V0][0]);
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

/// `a + off` on a guest pointer.
#[inline(always)]
pub(crate) fn pa(a: u64, off: u32) -> u32 {
    (a as u32).wrapping_add(off)
}

/// The speeds [`script_speed_for_rate`] wrote: instance → (its own speed,
/// the speed written for the tick).
struct ScriptSpeed {
    speed: u32,
    tick_speed: u32,
}

thread_local! {
    static SCRIPT_SPEEDS: std::cell::RefCell<std::collections::HashMap<u32, ScriptSpeed>> = Default::default();
}

/// Script time follows the tick rate: the per-tick advance of a sequence
/// (`InSeq_InstanceTick`: `pos += speed`, the channels advanced to `pos`) is
/// authored in 60 Hz frames, so above 60 the tick runs with `speed · 60 /
/// rate` in `+0x24` (the seek above does the same with its partial steps).
/// Returns the instance's own speed and the one written, for
/// [`script_speed_restore`]; `None` at 60, where nothing changes.
///
/// The instance's own speed is also kept here: if the tick unwinds (the
/// generated code resumes after it, without the restore), the next tick
/// finds the written speed still in place and takes the kept one.
fn script_speed_for_rate(c: &mut Cpu, this: u32) -> Option<(u32, u32)> {
    let r = crate::tick::rate(c);
    if r == 60 {
        return None;
    }
    let at = this.wrapping_add(0x24);
    let now = c.bus.read32(at);
    let speed = SCRIPT_SPEEDS.with(|m| match m.borrow().get(&this) {
        Some(k) if k.tick_speed == now => k.speed,
        _ => now,
    });
    let tick_speed = (f32::from_bits(speed) * 60.0 / r as f32).to_bits();
    c.bus.write32(at, tick_speed);
    SCRIPT_SPEEDS.with(|m| {
        let mut m = m.borrow_mut();
        if m.len() > 1024 {
            m.clear();
        }
        m.insert(this, ScriptSpeed { speed, tick_speed });
    });
    Some((speed, tick_speed))
}

/// After the tick: the instance's own speed back, unless the tick set a new
/// one (a script command).
fn script_speed_restore(c: &mut Cpu, this: u32, speed: u32, tick_speed: u32) {
    let at = this.wrapping_add(0x24);
    if c.bus.read32(at) == tick_speed {
        c.bus.write32(at, speed);
    }
    SCRIPT_SPEEDS.with(|m| m.borrow_mut().remove(&this));
}

/// `void InSeq_InstanceUpdate(InSeq_Instance* self)` (0x274a30): apply a
/// pending seek (`+0x34`) or a step count (`+0x38`), else advance exactly one
/// script frame; keeps the short step budget (`+0x2c`) and pauses when it runs
/// out.
///
/// `+0x1c` loop, `+0x20` playing, `+0x24` speed, `+0x28` pos (frames),
/// `+0x2c` frame budget, `+0x34` seek target, `+0x38` pending steps.
pub fn in_seq_instance_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x60);
        frame.save_fpr(c, 21, 0x58);
        frame.save_fpr(c, 20, 0x50);
        c.g[V1][0] = c.lw(pa(this, 0x34));
        // `need` (`s2`): the final one-frame tick is due unless a seek or a
        // step run already handled the frame. It stays in `s2` because the
        // callees save that register to the stack.
        c.g[S2][0] = 1;
        if (c.g[V1][0] as i64) >= 0 {
            // 0x274a64: a pending seek.
            c.cycles += 9;
            c.g[V0][0] = u64::MAX;
            c.g[S1][0] = c.g[V1][0];
            c.sw(pa(this, 0x34), c.g[V0][0]);
            callee::GET_LENGTH.call(c, 0x0027_4a74)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[S3][0] = c.lw(pa(this, 0x2c)); // the saved frame budget
            c.g[V0][0] = c.lw(pa(this, 0x1c));
            let no_loop = c.g[V0][0] == 0;
            c.sw(pa(this, 0x2c), 0);
            if !no_loop {
                // 0x274a88: target %= length.
                c.cycles += 3;
                c.div(0, S1, V1);
                c.cycles += 2;
                c.g[V0][0] = c.hi[0];
                c.g[S1][0] = c.g[V0][0];
            }
            // 0x274a9c: f20 = (float)target.
            c.cycles += 5;
            c.g[V0][0] = c.lw(pa(this, 0x20));
            c.mtc1(17, 20);
            c.cvt_s_w(20, 20);
            if c.g[V0][0] != 1 {
                c.lwc1(0, pa(this, 0x28));
            } else {
                // 0x274ab0: one tick before the seek comparison.
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::TICK.call(c, 0x0027_4ab8)?;
                c.lwc1(0, pa(this, 0x28));
            }
            // 0x274abc: already at the target?
            c.cycles += 4;
            c.c_eq_s(20, 0);
            if c.fcc() {
                // 0x274b6c: keep the budget while playing.
                c.g[V0][0] = c.lw(pa(this, 0x20));
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(pa(this, 0x2c), c.g[S3][0]);
                }
            } else {
                // 0x274acc: target < pos: rewind by restarting.
                c.cycles += 4;
                c.c_lt_s(20, 0);
                if c.fcc() {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    callee::STOP.call(c, 0x0027_4ae4)?;
                    c.g[A0][0] = c.g[S0][0];
                    callee::RESTART.call(c, 0x0027_4aec)?;
                    c.g[A0][0] = c.g[S0][0];
                    callee::TICK.call(c, 0x0027_4af4)?;
                }
                c.lwc1(0, pa(this, 0x28));
                // 0x274af8: advance in 1.0 steps until the target is reached.
                c.cycles += 12;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 21); // f21 = 1.0
                c.cvt_w_s(1, 0); // f1 = (int)pos
                c.mfc1(2, 1);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.mtc1(2, 1);
                c.cvt_s_w(1, 1); // f1 = (float)((int)pos + 1)
                c.sub_s(1, 1, 0); // f1 = fraction to the next frame
                c.c_lt_s(1, 21); // fraction < 1.0
                let partial = c.fcc();
                c.lwc1(22, pa(this, 0x24)); // f22 = speed
                if partial {
                    // 0x274b28: finish the current frame at the partial step.
                    c.cycles += 3;
                    c.swc1(1, pa(this, 0x24));
                    c.g[A0][0] = c.g[S0][0];
                    callee::TICK.call(c, 0x0027_4b34)?;
                }
                // 0x274b34: full steps until pos reaches the target.
                c.cycles += 3;
                c.g[V0][0] = c.lw(pa(this, 0x20));
                c.swc1(21, pa(this, 0x24));
                loop {
                    // 0x274b4c
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.swc1(22, pa(this, 0x24));
                        break;
                    }
                    // 0x274b54: pos < target?
                    c.cycles += 4;
                    c.lwc1(0, pa(this, 0x28));
                    c.c_lt_s(0, 20);
                    if !c.fcc() {
                        c.cycles += 1;
                        c.swc1(22, pa(this, 0x24));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_4b40);
                    }
                    // 0x274b40
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    callee::TICK.call(c, 0x0027_4b48)?;
                    c.g[V0][0] = c.lw(pa(this, 0x20));
                }
                // 0x274b68: keep the budget while playing.
                c.cycles += 1;
                c.g[V0][0] = c.lw(pa(this, 0x20));
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(pa(this, 0x2c), c.g[S3][0]);
                }
            }
            // 0x274b74
            c.cycles += 1;
            c.g[S2][0] = 0;
        }
        // 0x274b78: pending steps.
        c.cycles += 3;
        let mut steps = c.lw(pa(this, 0x38));
        c.g[V0][0] = steps;
        c.g[S1][0] = steps;
        if steps != 0 {
            // 0x274b84
            c.cycles += 5;
            let saved_f2c = c.lw(pa(this, 0x2c));
            c.g[S2][0] = saved_f2c;
            c.g[V0][0] = c.lw(pa(this, 0x20));
            c.sw(pa(this, 0x38), 0);
            let playing = c.g[V0][0];
            c.sw(pa(this, 0x2c), 0);
            if playing != 0 {
                // 0x274b98
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                loop {
                    // 0x274ba0
                    c.cycles += 5;
                    steps = sx((steps as u32).wrapping_sub(1));
                    c.g[S1][0] = steps;
                    callee::TICK.call(c, 0x0027_4ba8)?;
                    c.g[V0][0] = c.lw(pa(this, 0x20));
                    if c.g[V0][0] == 0 {
                        // The stop path jumps straight to 0x274bc4.
                        c.g[V0][0] = c.lw(pa(this, 0x2c));
                        break;
                    }
                    // 0x274bb4
                    c.cycles += 2;
                    let more = c.g[S1][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !more {
                        // 0x274bbc: the steps ran out while still playing.
                        c.cycles += 1;
                        c.sw(pa(this, 0x2c), c.g[S2][0]);
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(pa(this, 0x2c));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_4ba0);
                    }
                }
            } else {
                // 0x274bc0
                c.cycles += 1;
                c.g[V0][0] = c.lw(pa(this, 0x2c));
            }
            // 0x274bc4: a positive budget keeps playing.
            c.cycles += 2;
            let positive = (c.g[V0][0] as i64) > 0;
            c.g[S2][0] = 0;
            if !positive {
                // 0x274bcc: the budget ran out: pause.
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::PAUSE.call(c, 0x0027_4bd4)?;
                c.g[S2][0] = 0;
            }
        }
        // 0x274bd8: the final one-frame tick (skipped after a seek or steps).
        c.cycles += 2;
        if c.g[S2][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            let scaled = script_speed_for_rate(c, this as u32);
            callee::TICK.call(c, 0x0027_4be8)?;
            if let Some((speed, tick_speed)) = scaled {
                script_speed_restore(c, this as u32, speed, tick_speed);
            }
        }
        c.lq(S0, frame.at(c, 0x40));
        // 0x274bec
        c.cycles += 9;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x60);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
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
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::uf;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::dirtysock::tags2::tests::base as base_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tags2::tests::dtor_setup as dtor_setup_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::fe::debugfe::tests::FEW;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_6 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_3;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::fe::festateloadscreen::tests::r32_seed;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::w as w_3;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_6;
    pub(crate) use crate::game::main::ssxapp::tests::strict_check;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA_SIZE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::MEM_HEAP;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_5;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_4;
    pub(crate) use crate::game::sound::soundsys::tests::w as w_8;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn replay_request_instant_replay_matches() {
        check(
            ssx3_ee::s07::f_00270658,
            replay_request_instant_replay,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c13_0014 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(
                    DATA + 0x620,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(DATA + 0x624, r.next_u32());
            },
        );
    }

    #[test]
    fn replay_save_begin_snapshots_matches() {
        check(
            ssx3_ee::s07::f_00271348,
            replay_save_begin_snapshots,
            200,
            &|c, seed| {
                let mut r = Rng(0x71348 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let a = DATA + 0x100;
                let b = if seed % 2 == 0 { a } else { DATA + 0x140 };
                c.bus.write32(DATA + 0x3b0, a);
                c.bus.write32(DATA + 0x3d0, b);
                c.bus.write32(a + 0x14, r.next_u32());
                c.bus.write32(DATA + 0x638, r.next_u32());
            },
        );
    }

    #[test]
    fn replay_save_begin_caches_matches() {
        check(
            ssx3_ee::s07::f_002714e0,
            replay_save_begin_caches,
            100,
            &|c, seed| {
                let mut r = Rng(0x714e0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x638, r.next_u32());
            },
        );
    }

    #[test]
    fn math_eval_cubic_matches() {
        check(
            ssx3_ee::s07::f_00272570,
            math_eval_cubic,
            300,
            &|c, seed| {
                let mut r = Rng(0x72570 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + i * 4, r.edge_f32());
                }
                // Cover t == 0, a denormal (compares as zero) and ordinary values.
                c.f[12] = match seed % 4 {
                    0 => 0,
                    1 => r.next_u32() | 0x3f80_0000,
                    2 => r.next_u32() & 0x007f_ffff,
                    _ => r.edge_f32(),
                };
            },
        );
    }

    #[test]
    fn in_seq_span_track_activate_key_matches() {
        check_lohi(
            ssx3_ee::s07::f_002730c0,
            in_seq_span_track_activate_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x730c0 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA + 0x200;
                let key = DATA;
                let base = DATA + 0x1000;
                c.g[A0][0] = track as u64;
                c.g[A1][0] = key as u64;
                c.bus.write32(track + 0x20, base);
                c.bus.write32(track + 0x24, r.next_u32());
                c.bus.write32(track + 0x28, r.next_u32());
                c.bus.write32(key + 0x8, r.next_u32());
                c.bus.write32(key + 0xc, r.next_u32());
                let index = seed % 4;
                c.bus.write16(key + 2, index as u16);
                c.bus.write16(key + 4, (r.next_u32() & 0xffff) as u16);
                c.bus.write16(key + 6, (r.next_u32() & 0xffff) as u16);
                let kind = if seed % 2 == 0 {
                    seed % 0x11
                } else {
                    0x11 + seed % 8
                };
                c.bus.write16(key, kind as u16);
            },
        );
    }

    #[test]
    fn in_seq_span_track_next_key_matches() {
        check(
            ssx3_ee::s07::f_00273140,
            in_seq_span_track_next_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x73140 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let keys = DATA + 0x100;
                let next = DATA + 0x200;
                c.bus.write32(DATA + 0x10, keys);
                c.bus.write32(DATA + 0x14, next);
                let count = (seed % 7) as u32 + 1;
                c.bus.write16(keys, count as u16);
                c.bus.write16(next, (r.next_u32() & 0x3f) as u16);
                let cur = if seed % 2 == 0 { count - 1 } else { count + 3 };
                c.bus.write32(DATA + 0x18, cur);
            },
        );
    }

    #[test]
    fn in_seq_event_track_fire_key_matches() {
        check(
            ssx3_ee::s07::f_00273418,
            in_seq_event_track_fire_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x73418 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA;
                let key = DATA + 0x100;
                c.g[A0][0] = track as u64;
                c.g[A1][0] = key as u64;
                c.bus.write32(track + 0x1c, DATA + 0x1000);
                c.bus.write32(track + 0x20, (seed % 4) as u32);
                c.bus.write32(key + 0x4, r.next_u32());
                let kind = if seed % 3 == 0 {
                    seed % 9
                } else {
                    9 + seed % 8
                };
                c.bus.write16(key, kind as u16);
            },
        );
    }

    #[test]
    fn in_seq_event_track_next_key_matches() {
        check(
            ssx3_ee::s07::f_00273468,
            in_seq_event_track_next_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x73468 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let keys = DATA + 0x100;
                let next = DATA + 0x200;
                c.bus.write32(DATA + 0x10, keys);
                c.bus.write32(DATA + 0x14, next);
                let count = (seed % 7) as u32 + 1;
                c.bus.write16(keys, count as u16);
                c.bus.write16(next, (r.next_u32() & 0x3f) as u16);
                let cur = if seed % 2 == 0 { count - 1 } else { count + 3 };
                c.bus.write32(DATA + 0x18, cur);
            },
        );
    }

    #[test]
    fn in_seq_ramp_track_activate_key_matches() {
        check_lohi(
            ssx3_ee::s07::f_00273860,
            in_seq_ramp_track_activate_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x73860 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA + 0x200;
                let key = DATA;
                let base = DATA + 0x1000;
                c.g[A0][0] = track as u64;
                c.g[A1][0] = key as u64;
                c.bus.write32(track + 0x1c, base);
                c.bus.write32(track + 0x20, r.next_u32());
                c.bus.write32(track + 0x24, r.next_u32());
                c.bus.write32(key + 0x8, r.next_u32());
                c.bus.write32(key + 0xc, r.next_u32());
                c.bus.write32(key + 0x10, r.edge_f32());
                c.bus.write32(key + 0x14, r.next_u32());
                let index = seed % 4;
                c.bus.write16(key + 2, index as u16);
                c.bus.write16(key + 4, (r.next_u32() & 0xffff) as u16);
                c.bus.write16(key + 6, (r.next_u32() & 0xffff) as u16);
                let kind = if seed % 2 == 0 {
                    seed % 0x19
                } else {
                    0x19 + seed % 8
                };
                c.bus.write16(key, kind as u16);
            },
        );
    }

    #[test]
    fn in_seq_ramp_track_next_key_matches() {
        check(
            ssx3_ee::s07::f_002738f8,
            in_seq_ramp_track_next_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x738f8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let keys = DATA + 0x100;
                let next = DATA + 0x200;
                c.bus.write32(DATA + 0x10, keys);
                c.bus.write32(DATA + 0x14, next);
                let count = (seed % 7) as u32 + 1;
                c.bus.write16(keys, count as u16);
                c.bus.write16(next, (r.next_u32() & 0x3f) as u16);
                let cur = if seed % 2 == 0 { count - 1 } else { count + 3 };
                c.bus.write32(DATA + 0x18, cur);
            },
        );
    }

    #[test]
    fn in_seq_channel_ctor_matches() {
        check(
            ssx3_ee::s07::f_00273938,
            in_seq_channel_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x73938 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..12 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn in_seq_channel_setup_matches() {
        check(
            ssx3_ee::s07::f_002739b8,
            in_seq_channel_setup,
            200,
            &|c, seed| {
                let mut r = Rng(0x739b8 ^ seed as u64);
                scramble(c, &mut r);
                let data = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = data as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                let extra = if seed % 2 == 0 { 0 } else { 0x10 };
                c.bus.write32(data + 8, extra);
                c.bus.write32(data, r.next_u32());
                let track = data + extra;
                let kind = if seed % 4 == 0 {
                    seed % 9
                } else {
                    9 + seed % 8
                };
                c.bus.write16(track, kind as u16);
            },
        );
    }

    #[test]
    fn in_seq_instance_ctor_matches() {
        check(
            ssx3_ee::s07::f_00274518,
            in_seq_instance_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x74518 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..0x12 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn in_seq_instance_start_matches() {
        check(
            ssx3_ee::s07::f_00274830,
            in_seq_instance_start,
            300,
            &|c, seed| {
                let mut r = Rng(0x74830 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                // Path 0: not active. Path 1: active, no pending start. Path 2:
                // active with a pending start request.
                let path = seed % 3;
                c.bus
                    .write32(DATA + 0x20, if path == 0 { 0 } else { r.next_u32() | 1 });
                c.bus
                    .write32(DATA + 0x3c, if path == 2 { r.next_u32() | 1 } else { 0 });
            },
        );
    }

    #[test]
    fn in_seq_instance_pause_matches() {
        check(
            ssx3_ee::s07::f_002749e8,
            in_seq_instance_pause,
            200,
            &|c, seed| {
                let mut r = Rng(0x749e8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(
                    DATA + 0x20,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(DATA + 0x2c, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_instance_resume_matches() {
        check(
            ssx3_ee::s07::f_00274a08,
            in_seq_instance_resume,
            300,
            &|c, seed| {
                let mut r = Rng(0x74a08 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let path = seed % 3;
                c.bus
                    .write32(DATA + 0x20, if path == 0 { 0 } else { r.next_u32() | 1 });
                let count = match path {
                    0 => r.next_u32(),
                    1 => 1 + r.next_u32() % 100,
                    _ => 0,
                };
                c.bus.write32(DATA + 0x2c, count);
            },
        );
    }

    #[test]
    fn in_seq_instance_get_channel_matches() {
        check_lohi(
            ssx3_ee::s07::f_00274c20,
            in_seq_instance_get_channel,
            200,
            &|c, seed| {
                let mut r = Rng(0x274c_20 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 12) as u64;
                c.bus.write32(DATA + 0x18, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_instance_get_listener_matches() {
        check(
            ssx3_ee::s07::f_00274d70,
            in_seq_instance_get_listener,
            200,
            &|c, seed| {
                let mut r = Rng(0x274d_70 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 0x44, if seed % 3 == 0 { 0 } else { r.next_u32() });
                let link = DATA + 0x200;
                c.bus.write32(DATA + 0x8, link);
                c.bus.write32(link, r.next_u32());
            },
        );
    }

    #[test]
    fn math_wrap_angle_clamp_half_pi_matches() {
        check(
            ssx3_ee::s02::f_0016bd30,
            math_wrap_angle_clamp_half_pi,
            300,
            &|c, seed| {
                let mut r = Rng(0x16bd ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x6470), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x646c), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x6468), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x6464), r.edge_f32());
                c.bus.write32(DATA, r.edge_f32());
            },
        );
    }

    #[test]
    fn replay_get_max_highlights_matches() {
        check(
            ee::s07::f_00270c40__generated,
            replay_get_max_highlights,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0027_0c40, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let v = r.next_u32();
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    mixed(&mut r, seed, v)
                };
                c.bus.write32(DATA + 0xc, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_get_header_extra_matches() {
        check(
            ee::s07::f_00274c38__generated,
            in_seq_get_header_extra,
            200,
            &|c, seed| {
                machine(c, seed, 0x27_4c38, 0);
                c.g[A0][0] = DATA as u64;
                let version = (seed % 20) as u16;
                c.bus.write16(DATA + 2, version);
            },
        );
    }

    // ------------------------------------------------------------ destructors

    /// `flags = seed % 2` so both paths run; on the deleting path `this` is a
    /// plain data pointer (the allocator tolerates it, as in its own test).
    pub(crate) fn dtor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = game(c, seed, salt, 6);
        c.g[A0][0] = if seed % 8 == 0 { 0 } else { ptr(&mut r) as u64 };
        c.g[A1][0] = (seed % 2) as u64;
    }

    #[test]
    fn in_seq_dtor_matches() {
        check(
            ee::s07::f_002728d0__generated,
            in_seq_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x2728d0),
        );
    }

    // ----------------------------------------------- sequence / audio helpers

    #[test]
    fn in_seq_stop_matches() {
        check(
            ee::s07::f_00272b58__generated,
            in_seq_stop,
            100,
            &|c, seed| {
                let _r = game(c, seed, 0x272b58, 2);
                let array = SCRATCH + 0x1000;
                c.g[A0][0] = SCRATCH as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(SCRATCH + 0x14, array);
                for i in 0..4u32 {
                    // Half the instances exist, half have no instance at all.
                    c.bus
                        .write32(array + 0x58 * i + 0xc, ((seed + i) % 2) as u32);
                    // `+0x20 == 0` stops the instance immediately.
                    c.bus.write32(array + 0x58 * i + 0x28, 0);
                }
            },
        );
    }

    #[test]
    fn replay_stream_dtor_matches() {
        check(
            ee::s07::f_002721d0__generated,
            replay_stream_dtor,
            60,
            &|c, seed| dtor_setup_2(c, seed, 0x2721d0),
        );
    }

    #[test]
    fn world_state_stream_dtor_matches() {
        check(
            ee::s07::f_00272258__generated,
            world_state_stream_dtor,
            60,
            &|c, seed| dtor_setup_2(c, seed, 0x272258),
        );
    }

    // ------------------------------------------------------- script / sequence

    /// An instrument-sequence channel at `SCRATCH` whose `+0x14` table has one
    /// live record (so `InSeq_GetInstance` returns `record + 8`).
    pub(crate) fn in_seq_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let _r = machine(c, seed, salt, 2);
        let table = SCRATCH + 0x1000;
        let record = table + (seed % 4) as u32 * 0x58;
        c.g[A0][0] = SCRATCH as u64;
        c.g[A1][0] = (seed % 4) as u64;
        c.g[A2][0] = if seed % 2 == 0 {
            0
        } else {
            (SCRATCH + 0x800) as u64
        };
        c.bus.write32(SCRATCH + 0x14, table);
        c.bus.write32(record + 0xc, 1);
    }

    #[test]
    fn in_seq_play_matches() {
        check(
            ee::s07::f_00272ad0__generated,
            in_seq_play,
            100,
            &|c, seed| in_seq_setup(c, seed, 0x272ad0),
        );
    }

    #[test]
    fn replay_highlight_slot_init_matches() {
        check(
            ssx3_ee::s07::f_00272288__generated,
            replay_highlight_slot_init,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x272288, 0);
                let slot = DATA + 0x100;
                fill(c, &mut r, slot, 0x40);
                c.g[A0][0] = sx(slot);
            },
        );
    }

    // ------------------------------------------------------------------
    // InSeq track constructors.

    #[test]
    fn in_seq_curve_track_ctor_matches() {
        check(
            ssx3_ee::s07::f_002725b8__generated,
            in_seq_curve_track_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2725b8, 0);
                let track = DATA + 0x100;
                fill(c, &mut r, track, 0x20);
                c.g[A0][0] = sx(track);
            },
        );
    }

    #[test]
    fn in_seq_span_track_ctor_matches() {
        check(
            ssx3_ee::s07::f_00272d68__generated,
            in_seq_span_track_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x272d68, 0);
                let track = DATA + 0x100;
                fill(c, &mut r, track, 0x20);
                c.g[A0][0] = sx(track);
            },
        );
    }

    #[test]
    fn in_seq_event_track_ctor_matches() {
        check(
            ssx3_ee::s07::f_00273180__generated,
            in_seq_event_track_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x273180, 0);
                let track = DATA + 0x100;
                fill(c, &mut r, track, 0x20);
                c.g[A0][0] = sx(track);
            },
        );
    }

    #[test]
    fn in_seq_ramp_track_ctor_matches() {
        check(
            ssx3_ee::s07::f_002734a8__generated,
            in_seq_ramp_track_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2734a8, 0);
                let track = DATA + 0x100;
                fill(c, &mut r, track, 0x20);
                c.g[A0][0] = sx(track);
            },
        );
    }

    #[test]
    fn replay_open_menu_matches() {
        check(
            ee::s07::f_002706b8__generated,
            replay_open_menu,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2706_b8, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x700);
                c.g[A0][0] = sx(self_);
                c.bus
                    .write32(self_ + 0x61c, if seed % 3 == 0 { 0 } else { 1 });
                let state = if seed % 2 == 0 { 9 } else { seed % 9 };
                c.bus.write32(self_, state);
            },
        );
    }

    #[test]
    fn replay_cancel_highlight_matches() {
        check(
            ee::s07::f_00270de0__generated,
            replay_cancel_highlight,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x270d_e0, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x800);
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx((seed % 4) as u32);
                c.bus.write32(self_, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn replay_save_write_header_matches() {
        check(
            ee::s07::f_002712c0__generated,
            replay_save_write_header,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2712_c0, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x700);
                fill(c, &mut r, DATA + 0x600, 0x10);
                c.g[A0][0] = sx(self_);
                c.bus.write32(self_ + 0x638, r.next_u32());
                c.bus
                    .write32(self_ + 0x640, if seed % 2 == 0 { 0 } else { DATA + 0x600 });
            },
        );
    }

    #[test]
    fn replay_end_instant_replay_matches() {
        check(
            ssx3_ee::s07::f_00270670__generated,
            replay_end_instant_replay,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x270670, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // Only the idle path: an active playback runs the whole replay
                // end-of-playback chain, which needs a live replay object.
                c.bus.write32(this_ + 0x620, 0);
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn replay_free_save_buffers_matches() {
        check(
            ssx3_ee::s07::f_00270f78__generated,
            replay_free_save_buffers,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x270f78, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x63c, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                c.bus
                    .write32(this_ + 0x640, if seed % 2 == 0 { 0 } else { DATA + 0x2400 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn in_seq_channel_dtor_matches() {
        check(
            ssx3_ee::s07::f_00273970__generated,
            in_seq_channel_dtor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x273970, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus.write32(this_ + 0x14, DATA + 0x2000);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn in_seq_channel_select_block_matches() {
        check(
            ssx3_ee::s07::f_00273a60__generated,
            in_seq_channel_select_block,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x273a60, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let offsets = DATA + 0x2000;
                let blocks = DATA + 0x3000;
                c.bus.write32(this_ + 0x04, blocks);
                c.bus.write32(this_ + 0x14, offsets);
                // The selected block's first event record has a small/large count.
                let idx: u32 = seed % 4;
                c.bus.write32(offsets + 4 * idx, 0x40 * idx);
                c.bus
                    .write16(blocks + 0x40 * idx, if seed % 3 == 0 { 0 } else { 12 });
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = idx as u64;
            },
        );
    }

    #[test]
    fn replay_save_emit_matches() {
        check(
            ee::s07::f_00271058__generated,
            replay_save_emit,
            SEEDS_3,
            &|c, seed| {
                let mut r = base(c, seed, 0x2710_58);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let data = DATA + 0x400;
                for i in 0..0x40u32 {
                    c.bus.write8(data + i, r.next_u32() as u8);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(data);
                c.g[A2][0] = [0u64, 1, 8, 0x40][(seed % 4) as usize];
                c.bus.write32(this + 0x680, r.next_u32());
                c.bus.write32(this + 0x684, r.next_u32());
                c.bus.write32(this + 0x678, r.next_u32());
            },
        );
    }

    #[test]
    fn replay_count_save_snapshots_matches() {
        check(
            ee::s07::f_002712f8__generated,
            replay_count_save_snapshots,
            SEEDS_3,
            &|c, seed| {
                let mut r = base(c, seed, 0x2712_f8);
                let this = DATA + 0x100;
                clear(c, this, 0x700);
                // A chain of 0..4 snapshots; `skip` is one of them.
                let n = seed % 5;
                let nodes: Vec<u32> = (0..n).map(|i| DATA + 0x200 + 0x20 * i).collect();
                for (i, &node) in nodes.iter().enumerate() {
                    let next = nodes.get(i + 1).copied().unwrap_or(0);
                    c.bus.write32(node + 0x14, next);
                    c.bus.write32(node + 4, r.next_u32());
                }
                c.bus
                    .write32(this + 0x3b0, nodes.first().copied().unwrap_or(0));
                c.bus.write32(
                    this + 0x3d0,
                    nodes
                        .get((seed as usize) % (n as usize + 1))
                        .copied()
                        .unwrap_or(0),
                );
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn replay_save_write_footer_matches() {
        check(
            ee::s07::f_00271708__generated,
            replay_save_write_footer,
            SEEDS_3,
            &|c, seed| {
                let mut r = base(c, seed, 0x2717_08);
                let this = DATA + 0x100;
                clear(c, this, 0x700);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x638, r.next_u32());
                c.bus
                    .write32(this + 0x640, if seed % 3 == 0 { 0 } else { DATA + 0x900 });
                clear(c, DATA + 0x900, 0x40);
            },
        );
    }

    #[test]
    fn replay_clear_all_highlights_matches() {
        check(
            ee::s07::f_00270e18__generated,
            replay_clear_all_highlights,
            60,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x270e);
                let s = DATA;
                c.bus.write32(s, 1 + r.next_u32() % 3);
                for i in 0..2u32 {
                    c.bus.write32(s + 0x494 + 0xb4 * i, r.next_u32());
                    c.bus.write32(s + 0x494 + 0xb4 * i + 4, r.next_u32());
                }
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn inseq_span_track_dtor_matches() {
        check(
            ee::s07::f_00272da0__generated,
            inseq_span_track_dtor,
            60,
            &|c, seed| {
                base_2(c, seed, 0x272d);
                let flags = if seed % 3 == 0 { 1u32 } else { 0 };
                // The delete path frees `self`: use a small (pooled) address.
                let obj = if flags == 1 { 0x100 } else { DATA };
                c.bus.write32(gp_at(0x398), 0);
                // No live track data: the stop routine takes its early exit.
                c.bus.write32(obj + 0xc, 0);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(flags);
            },
        );
    }

    #[test]
    fn inseq_event_track_dtor_matches() {
        check(
            ee::s07::f_002731b8__generated,
            inseq_event_track_dtor,
            60,
            &|c, seed| {
                base_2(c, seed, 0x2731);
                let flags = if seed % 3 == 0 { 1u32 } else { 0 };
                let obj = if flags == 1 { 0x100 } else { DATA };
                c.bus.write32(gp_at(0x398), 0);
                c.bus.write32(obj + 0xc, 0);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(flags);
            },
        );
    }

    #[test]
    fn inseq_ramp_track_dtor_matches() {
        check(
            ee::s07::f_002734e0__generated,
            inseq_ramp_track_dtor,
            60,
            &|c, seed| {
                base_2(c, seed, 0x2734);
                let flags = if seed % 3 == 0 { 1u32 } else { 0 };
                let obj = if flags == 1 { 0x100 } else { DATA };
                c.bus.write32(gp_at(0x398), 0);
                c.bus.write32(obj + 0xc, 0);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(flags);
            },
        );
    }

    #[test]
    fn player_free_replay_cache_matches() {
        check(
            ssx3_ee::s07::f_002700d8__generated,
            player_free_replay_cache,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x2700d8, 1);
                heap_init(c);
                let player = DATA;
                let cache = alloc(c, 0x40);
                c.bus.write32(cache + 0xc, 0);
                let slot1 = if seed % 2 == 0 { cache } else { DATA + 0x300 };
                c.bus.write32(player + 0x48c, slot1);
                c.bus
                    .write32(player + 0x490, if seed % 3 == 0 { cache } else { 0 });
                c.g[A0][0] = player as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => cache as u64,
                    _ => (DATA + 0x300) as u64,
                };
            },
        );
    }

    #[test]
    fn replay_alloc_save_buffers_matches() {
        check(
            ssx3_ee::s07::f_00270f20__generated,
            replay_alloc_save_buffers,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x270f20, 2);
                heap_init(c);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.bus.write32(DATA + 0x400, seed);
            },
        );
    }

    /// Build `n` linked event nodes for the span/ramp track queries.
    pub(crate) fn build_nodes(
        c: &mut Cpu,
        r: &mut Rng,
        addr: u32,
        n: u32,
        time_off: u32,
        next_off: u32,
    ) {
        for i in 0..n {
            let node = addr + i * 0x40;
            c.bus.write32(node + time_off, r.next_u32());
            c.bus
                .write32(node + next_off, if i + 1 < n { node + 0x40 } else { 0 });
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
                c.bus
                    .write16(DATA + 0x100, if seed % 7 == 0 { 0 } else { 1 });
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
        check(
            ssx3_ee::s07::f_002733e0__generated,
            in_seq_event_track_get_next_event_time,
            160,
            &|c, seed| {
                let mut r = Rng(0x2733_e000 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA;
                c.g[A0][0] = track as u64;
                c.bus
                    .write32(track + 0x8, if seed % 5 == 0 { 0 } else { DATA + 0x100 });
                c.bus.write16(
                    DATA + 0x100,
                    if seed % 3 == 0 {
                        0
                    } else {
                        r.next_u32() as u16
                    },
                );
                let current = if seed % 4 == 0 { 0 } else { DATA + 0x200 };
                c.bus.write32(track + 0x14, current);
                if current != 0 {
                    c.bus.write16(current + 0x2, r.next_u32() as u16);
                }
            },
        );
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
                c.bus
                    .write16(DATA + 0x100, if seed % 7 == 0 { 0 } else { 1 });
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
    fn in_seq_channel_release_matches() {
        check(
            ee::s07::f_00273a00__generated,
            in_seq_channel_release,
            SEEDS_4,
            &|c, seed| {
                let _ = base_3(c, seed, 0x273a_00);
                let s = DATA + 0x100;
                clear(c, s, 0x80);
                w(c, s + 4, if seed % 2 == 0 { 0 } else { 1 });
                // `InSeq_ChannelStop` returns at once with no bound stream.
                w(c, s + 0x28, 0);
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn replay_save_compute_padding_matches() {
        check(
            ee::s07::f_00271620__generated,
            replay_save_compute_padding,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_1620, 2);
                be_library(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x640, DATA + 0x1_0000);
                c.bus.write32(DATA + 0x638, r.next_u32());
                c.bus.write32(DATA + 0x678, r.next_u32() & 0xffff);
            },
        );
    }

    #[test]
    fn replay_save_write_padding_matches() {
        check(
            ee::s07::f_00271688__generated,
            replay_save_write_padding,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_1688, 1);
                be_library(c);
                c.g[A0][0] = DATA as u64;
                let left: i32 = match seed % 5 {
                    0 => -1,
                    1 => 0,
                    2 => 1 + (r.next_u32() % 0x3fff) as i32,
                    3 => 0x4000,
                    _ => 0x4000 + (r.next_u32() % 0x1000) as i32,
                };
                c.bus.write32(DATA + 0x67c, left as u32);
                c.bus.write32(DATA + 0x640, DATA + 0x1_0000);
                c.bus.write32(DATA + 0x638, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_instance_get_remaining_steps_matches() {
        check(
            ee::s07::f_00274d08__generated,
            in_seq_instance_get_remaining_steps,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_4d08, 1);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x24, r.f32(8.0) | 0x0080_0000);
                c.bus.write32(DATA + 0x28, r.f32(4.0));
                c.bus.write32(DATA + 0x30, 0); // length path returns at once
            },
        );
    }

    /// The shipped replay state table at [`REPLAY_TABLE`].
    pub(crate) const REPLAY_HANDLERS: [u32; 16] = [
        0x0027_02ec,
        0x0027_02b0,
        0x0027_02ec,
        0x0027_02e8,
        0x0027_02d4,
        0x0027_02e8,
        0x0027_02e8,
        0x0027_02b0,
        0x0027_02e8,
        0x0027_02e8,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
    ];

    #[test]
    fn replay_is_paused_matches() {
        strict_check(
            ssx3_ee::s07::f_00270280__generated,
            replay_is_paused,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2702_80, 0);
                c.pc = 0x0027_0280;
                let replay = DATA + 0x1000;
                c.g[A0][0] = sx(replay);
                put_words(c, REPLAY_TABLE, &REPLAY_HANDLERS);
                // States 0..15 reach every handler kind; 0x10 skips the lookup.
                c.bus.write32(replay, (seed % 17) as u32);
                c.bus.write32(replay + 0x3cc, r.next_u32() % 4);
                c.bus.write32(replay + 0x484, r.next_u32() % 4);
                c.bus
                    .write32(replay + 0x604, if seed % 3 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    pub(crate) fn in_seq_test(generated: GuestFn, hand: GuestFn, salt: u64, field: u32) {
        check(generated, hand, 20, &|c, seed| {
            fakes::machine(c, seed, salt, 0);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            if seed % 3 == 0 {
                w(c, OBJ + 0xc, 0);
            } else {
                w(c, OBJ + 0xc, OBJ2);
                w(c, OBJ2 + 8, OBJ3);
                w(c, OBJ3 + 8, OBJ4);
                w(c, OBJ4 + 8, 0);
                w(c, OBJ + field, if seed % 2 == 0 { 0 } else { OBJ4 + 0x100 });
            }
        });
    }

    #[test]
    fn in_seq_span_track_stop_matches() {
        in_seq_test(
            ee::s07::f_00272ec0__generated,
            in_seq_span_track_stop,
            0x272ec0,
            0x20,
        );
    }

    #[test]
    fn in_seq_event_track_stop_matches() {
        in_seq_test(
            ee::s07::f_002732c8__generated,
            in_seq_event_track_stop,
            0x2732c8,
            0x1c,
        );
    }

    #[test]
    fn in_seq_ramp_track_stop_matches() {
        in_seq_test(
            ee::s07::f_00273630__generated,
            in_seq_ramp_track_stop,
            0x273630,
            0x1c,
        );
    }

    #[test]
    fn player_alloc_replay_cache_matches() {
        check(
            ee::s07::f_00270070__generated,
            player_alloc_replay_cache,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x270070, 0);
                globals(c);
                pool_allocator_2(c);
                c.g[A0][0] = OBJ as u64;
                let k = seed % 3;
                w(c, OBJ + 0x48c, if k == 0 { 0 } else { OBJ2 });
                w(c, OBJ + 0x490, if k == 2 { OBJ3 } else { 0 });
            },
        );
    }

    #[test]
    fn replay_count_highlights_matches() {
        check(
            ee::s07::f_00270bd8__generated,
            replay_count_highlights,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x270bd8, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                if seed % 5 == 0 {
                    w(c, OBJ + 0x3b0, 0);
                } else {
                    w(c, OBJ + 0x3b0, OBJ2);
                    w(c, OBJ2 + 0x1e, if seed % 3 == 0 { 1 } else { 0xff });
                    w(c, OBJ2 + 0x14, if seed % 4 == 0 { 0 } else { OBJ3 });
                    w(c, OBJ3 + 0x1e, 0xff);
                    w(c, OBJ3 + 0x14, 0);
                    for i in 0..4u32 {
                        w(
                            c,
                            OBJ2 + 0x20 + 4 * i,
                            if seed % 3 == 0 { 0xffff_ffff } else { 1 },
                        );
                        w(c, OBJ3 + 0x20 + 4 * i, 0x8000_0000);
                    }
                }
            },
        );
    }

    #[test]
    fn replay_begin_save_matches() {
        check(
            ee::s07::f_00270e68__generated,
            replay_begin_save,
            4,
            &|c, seed| {
                let _ = base_4(c, seed, 0x0027_0e68, 2);
                c.g[A0][0] = OBJ_2 as u64;
            },
        );
    }

    #[test]
    fn in_seq_start_sequence_matches() {
        check(
            ssx3_ee::s07::f_00272938__generated,
            in_seq_start_sequence,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x272938, 0);
                let seq = DATA + 0x1000;
                let src = DATA + 0x2000;
                c.g[A0][0] = seq as u64;
                c.g[A1][0] = src as u64;
                // Free node at seq+0x18, used list at seq+0x1c.
                let node = DATA + 0x3000;
                let used = DATA + 0x4000;
                w32(c, seq + 0x18, node);
                w32(c, seq + 0x1c, if seed % 2 == 0 { 0 } else { used });
                w32(c, node + 0x50, if seed % 3 == 0 { 0 } else { node + 0x100 });
                w32(c, node + 0x4, r.next_u32());
                w32(c, node + 0x54, r.next_u32());
                w32(c, used + 0x54, r.next_u32());
                // InSeq_InstanceSetup needs the source header to look benign.
                w16(c, src + 2, 0);
                w32(c, src + 4, 0);
                w32(c, src + 8, 0);
                w32(c, node, 0);
                w32(c, node + 8, 0);
                w32(c, node + 0x34, 0);
            },
        );
    }

    #[test]
    fn replay_seek_to_end_matches() {
        check(
            ee::s07::f_00270378__generated,
            replay_seek_to_end,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x270378, 2);
                be_library_2(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                w_3(
                    c,
                    obj + 0x48c,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w_3(
                    c,
                    obj + 0x490,
                    if seed % 4 == 0 { 0 } else { DATA + 0x2000 },
                );
                w_3(c, obj + 0x3b0, 0);
                w_3(c, obj + 0x484, r.next_u32() % 0x1000);
            },
        );
    }

    #[test]
    fn replay_check_instant_replay_matches() {
        check(
            ee::s07::f_002705b0__generated,
            replay_check_instant_replay,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2705b0, 1);
                be_library_2(c);
                let obj = DATA;
                let alt = DATA + 0x800;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = alt as u64;
                c.g[A2][0] = (seed % 2) as u64;
                w_3(c, obj + 0x3d4, if seed % 3 == 0 { 0 } else { 1 });
                let wld = DATA + 0x1000;
                let race = DATA + 0x1200;
                let mgr = DATA + 0x1400;
                w_3(c, GAME_GP.wrapping_sub(0x848), wld);
                w_3(c, wld + 0x84, race);
                w_3(c, race + 0xc, mgr);
                w_3(c, mgr + 0x7c, if seed % 4 == 0 { 2 } else { 1 });
                w_3(c, alt + 0x9c, r.next_u32() % 0x1000);
                w_3(c, alt + 0xa4, r.next_u32() % 0x1000);
                w_3(c, obj + 0x474, r.next_u32() % 0x1000);
                w_3(c, obj + 0x47c, r.next_u32() % 0x1000);
            },
        );
    }

    // ------------------------------------------------------------------
    // In-sequence channels.

    #[test]
    fn in_seq_channel_set_listener_matches() {
        check(
            ssx3_ee::s07::f_00274240__generated,
            in_seq_channel_set_listener,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274240, 4);
                be_library_3(c);
                world_2(c);
                let ch = DATA + 0x100;
                c.g[A0][0] = ch as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    (DATA + 0x300) as u64
                };
                w_4(c, ch + 0x1c, DATA + 0x400);
                b(c, DATA + 0x400 + 3, (r.next_u32() % 4) as u8);
                if seed % 3 != 0 {
                    let old = DATA + 0x500;
                    let vt = DATA + 0x600;
                    w_4(c, ch + 0x2c, old);
                    w_4(c, old, vt);
                    slot(c, vt, 0x10, 0, FAKE);
                    slot(c, vt, 0x14, 0, FAKE + 0x10);
                } else {
                    w_4(c, ch + 0x2c, 0);
                }
                let new = DATA + 0x300;
                w_4(c, new, if seed % 4 == 0 { 0 } else { DATA + 0x700 });
                let vt = DATA + 0x700;
                slot(c, vt, 8, 0, FAKE + 0x20);
                slot(c, vt, 0xc, 0, FAKE + 0x30);
            },
        );
    }

    #[test]
    fn in_seq_curve_track_start_matches() {
        check(
            ee::s07::f_002725f0__generated,
            in_seq_curve_track_start,
            150,
            &|c, seed| {
                let mut r = base(c, seed, 0x27_25f0);
                globals_3(c);
                const DATA_PTR: u32 = DATA + 0x2000;
                w_5(c, DATA_PTR, [0, 1, 2][seed as usize % 3]);
                w_5(c, DATA_PTR + 4, [0, 0x10, 0x20][seed as usize % 3]);
                // A couple of curve keys for Math_EvalCubic.
                for i in 0..6u32 {
                    w_5(c, DATA_PTR + 8 + 4 * i, r.next_u32() & 0x3f80_3f80);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA_PTR);
                c.g[A2][0] = sx(DATA + 0x3000);
            },
        );
    }

    #[test]
    fn replay_flush_save_output_matches() {
        check(
            ee::s07::f_00270fc0__generated,
            replay_flush_save_output,
            SEEDS_8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x0fc0, 2);
                let this = DATA;
                match seed % 4 {
                    0 => w_6(c, this + 0x644, 0),
                    1 => {
                        w_6(c, this + 0x644, DATA + 0x1000);
                        w_6(c, this + 0x648, 0);
                    }
                    2 => {
                        w_6(c, this + 0x644, DATA + 0x1000);
                        w_6(c, this + 0x648, 0x100);
                        w_6(c, this + 0x64c, 0x4000);
                        w_6(c, this + 0x650, DATA + 0x8000);
                    }
                    _ => {
                        w_6(c, this + 0x644, DATA + 0x1000);
                        w_6(c, this + 0x648, 0x3000);
                        w_6(c, this + 0x64c, 0x100);
                        w_6(c, this + 0x650, DATA + 0x8000);
                    }
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn replay_save_build_header_matches() {
        check(
            ee::s07::f_00271228__generated,
            replay_save_build_header,
            SEEDS_8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x1228, 3);
                let this = DATA;
                w_6(c, this + 0x640, DATA + 0x1000);
                w_6(c, this + 0x18, 0x1234);
                w_6(c, this + 0x610, 0x5678);
                w_6(c, this + 0x3c8, 0x9abc);
                w_6(c, this + 0x3cc, 0xdef0);
                w_6(c, this + 0x3b0, 0);
                w_6(c, this + 0x3d0, 0);
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn in_seq_instance_get_length_matches() {
        check(
            ee::s07::f_00274c70__generated,
            in_seq_instance_get_length,
            SEEDS_8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x4c70, 2);
                let this = DATA;
                if seed & 1 == 0 {
                    w_6(c, this + 0x30, 0x1234);
                } else {
                    w_6(c, this + 0x30, 0xffff_ffff);
                    w_6(c, this + 0x10, DATA + 0x1000);
                    wb(c, DATA + 0x1000 + 1, (seed % 3) as u8);
                    w_6(c, this + 0x18, DATA + 0x1100);
                    for i in 0..3u32 {
                        let ch = DATA + 0x1100 + 0x3c * i;
                        w_6(c, ch + 0x1c, DATA + 0x1300 + 0x20 * i);
                        w_6(c, DATA + 0x1300 + 0x20 * i + 4, 0x100 + i);
                    }
                    w_6(c, this + 0x20, (seed & 2) as u32);
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn in_seq_channel_flush_events_matches() {
        check(
            ssx3_ee::s07::f_00273d20__generated,
            in_seq_channel_flush_events,
            30,
            &|c, seed| {
                let _r = base_6(c, seed, 0x273d20, 2);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                w(
                    c,
                    this + 0x2c,
                    if seed % 2 == 0 { 0 } else { DATA + 0x2000 },
                );
                w(c, this + 0x30, r32_seed(seed));
                w(
                    c,
                    this + 0x34,
                    if seed % 3 == 0 { 0 } else { DATA + 0x3000 },
                );
                w(c, this + 0x1c, DATA + 0x4000);
                w(c, DATA + 0x2000, DATA + 0x2100);
                w(c, DATA + 0x2100 + 0x18, 0);
                w(c, DATA + 0x2100 + 0x1c, 0);
                w(c, DATA + 0x2100 + 0x20, 0);
                w(c, DATA + 0x2100 + 0x24, 0);
            },
        );
    }

    // ------------------------------------------------------------------ InSeq

    #[test]
    fn in_seq_free_sequence_matches() {
        check(
            ee::s07::f_002729a8__generated,
            in_seq_free_sequence,
            FEW,
            &|c, seed| {
                let _ = base_7(c, seed, 0x2729_a8, 4);
                let mgr = SCRATCH_2;
                let handle = 0x58 * (seed % 4) as u32;
                c.g[A0][0] = mgr as u64;
                c.g[A1][0] = handle as u64;
                w_7(c, mgr + 0x14, mgr + 0x2000);
                let seq = mgr + 0x2000 + handle;
                // Linked-list links: next (+0x50), prev (+0x54), instance
                // (+0x58) and the manager's active/idle head pointers.
                w_7(c, seq + 0x50, if seed % 3 == 0 { 0 } else { seq + 0x100 });
                w_7(c, seq + 0x54, if seed % 3 == 0 { 0 } else { seq + 0x200 });
                w_7(c, seq + 0x58, 0);
                w_7(c, seq + 0x100 + 0x50, seq);
                w_7(c, seq + 0x100 + 0x54, 0);
                w_7(c, seq + 0x200 + 0x50, 0);
                w_7(c, seq + 0x200 + 0x54, seq);
                w_7(c, mgr + 0x1c, seq);
                w_7(c, mgr + 0x18, seq + 0x100);
            },
        );
    }

    #[test]
    fn in_seq_event_track_start_matches() {
        strict(
            ee::s07::f_00273208__generated,
            in_seq_event_track_start,
            30,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn inseq_span_track_start_matches() {
        check(
            ee::s07::f_00272df0__generated,
            inseq_span_track_start,
            60,
            &|c, seed| {
                let _r = base_8(c, seed, 0x272d);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[A3][0] = (DATA + 0x3000) as u64;
                // An empty key list: the short path (the full path allocates
                // and walks a heap that cannot be bounded on test data).
                c.bus.write16(DATA + 0x1000, 0);
                c.bus.write32(DATA + 0x3000, seed);
            },
        );
    }

    #[test]
    fn replay_on_rider_checkpoint_matches() {
        check(
            ee::s07::f_00270ab0__generated,
            replay_on_rider_checkpoint,
            16,
            &|c, seed| {
                let _ = base_9(c, seed, 0x270ab0, 2);
                let this = DATA;
                clear_2(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = if seed % 4 == 0 {
                    0xffff_fffe
                } else {
                    (seed % 4) as u64
                };
                w_8(c, this + 0x61c, if seed % 5 == 0 { 1 } else { 0 });
                w_8(c, this + 0x620, if seed % 5 == 1 { 1 } else { 0 });
                w_8(c, this + 0x610, if seed % 5 == 2 { 1 } else { 0 });
                w_8(c, this + 0x60c, if seed % 5 == 3 { 1 } else { 0 });
                w_8(c, this + 0x10, 8);
                w_8(c, GAME_GP.wrapping_sub(0x848), WORLD);
            },
        );
    }

    #[test]
    fn in_seq_curve_track_update_matches() {
        diff(
            ee::s07::f_002726a8__generated,
            in_seq_curve_track_update,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27_26a8, 0);
                globals_6(c);
                let mode = seed % 4;
                let curve = if mode == 0 { 0 } else { DATA + 0x2000 };
                c.g[A0][0] = sx(DATA);
                c.f[12] = uf(&mut r, 10.0);
                w_9(c, DATA + in_seq_curve::CURVE, curve);
                w_9(c, DATA + in_seq_curve::VALUE, r.next_u32());
                w_9(c, DATA + in_seq_curve::LENGTH, DATA + 0x2100);
                w_9(
                    c,
                    DATA + in_seq_curve::SEGMENT,
                    if mode == 3 { 0 } else { DATA + 0x2200 },
                );
                w_9(c, DATA + in_seq_curve::KEY, (seed % 3) as u32);
                if curve != 0 {
                    c.bus.write16(curve, (seed % 0x30) as u16);
                    c.bus.write16(curve + 2, if mode == 1 { 2 } else { 0x20 });
                    fill(c, &mut r, curve + 4, 4);
                }
                w_9(c, DATA + 0x2100, (seed % 3) as u32);
                c.bus.write16(DATA + 0x2200, (seed % 0x30) as u16);
                c.bus.write16(DATA + 0x2202, (seed % 0x30) as u16);
                for i in 0..4 {
                    w_9(c, DATA + 0x2204 + 4 * i, uf(&mut r, 2.0));
                }
            },
        );
    }

    #[test]
    fn in_seq_heap_free_matches() {
        diff(
            ee::s07::f_002723a8__generated,
            in_seq_heap_free,
            40,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x27_23a8);
                c.g[A0][0] = sx(DATA);
                w_9(c, DATA + 8, if seed % 2 == 0 { 0 } else { 0 });
                w_9(c, DATA, r.next_u32());
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x1008) };
                w_9(c, DATA + 0x1000, 0x20); // the block's size header (payload at +8)
                w_9(c, DATA + 0x1008, 0x30);
                w_9(c, DATA + 0x100c, 0);
            },
        );
    }
    use super::*;

    /// One `InSeq_Instance` at `obj` with a one-second channel block (the
    /// cached length avoids the channel walk) and seed-picked seek/step/play
    /// state.
    pub(crate) fn fixture(c: &mut Cpu, seed: u32) {
        fakes::machine(c, seed, 0x274a30, 0);
        c.pc = 0x0027_4a30;
        let obj = DATA + 0x1000;
        let chan = DATA + 0x2000;
        c.g[A0][0] = obj as u64;
        c.bus.write8(chan + 1, 0);
        c.bus.write32(obj + 0x10, chan);
        c.bus.write32(obj + 0x18, 0);
        c.bus.write32(obj + 0x1c, if seed % 4 == 0 { 0 } else { 1 });
        c.bus
            .write32(obj + 0x20, [0, 1, 1, 2, 4][(seed % 5) as usize]);
        c.bus.write32(
            obj + 0x24,
            if seed % 2 == 0 {
                0x3f80_0000
            } else {
                0x3f00_0000
            },
        );
        c.bus.write32(
            obj + 0x28,
            [0, 0x3f00_0000, 0x4020_0000][(seed % 3) as usize],
        );
        c.bus.write32(obj + 0x2c, if seed % 2 == 0 { 0 } else { 1 });
        c.bus.write32(obj + 0x30, 100);
        c.bus
            .write32(obj + 0x34, [0xffff_ffff, 0, 3][(seed % 3) as usize]);
        c.bus.write32(obj + 0x38, [0, 1, 3, 0][(seed % 4) as usize]);
        c.bus.write32(obj + 0x3c, 0);
        c.bus.write32(obj + 0x40, 0);
    }

    #[test]
    fn in_seq_instance_update_matches() {
        check(
            ee::s07::f_00274a30__generated,
            in_seq_instance_update,
            24,
            &|c, seed| {
                fixture(c, seed);
            },
        );
    }

    /// A playing sequence (speed 1, then 0.5) moves the same script time per
    /// second at 60, 120, 144 and 240 Hz, and keeps its own speed.
    #[test]
    fn in_seq_instance_update_rate() {
        for speed in [1.0f32, 0.5] {
            for hz in [60u32, 120, 144, 240] {
                let mut c = crate::difftest::fresh();
                fixture(&mut c, 2);
                crate::tick::set_for_test(&mut c, hz);
                let obj = DATA + 0x1000;
                c.bus.write32(obj + 0x1c, 1);
                c.bus.write32(obj + 0x20, 3);
                c.bus.write32(obj + 0x24, speed.to_bits());
                c.bus.write32(obj + 0x28, 0);
                c.bus.write32(obj + 0x2c, 0);
                c.bus.write32(obj + 0x30, 1000);
                c.bus.write32(obj + 0x34, 0xffff_ffff);
                c.bus.write32(obj + 0x38, 0);
                for _ in 0..hz {
                    c.g[A0][0] = obj as u64;
                    c.g[RA][0] = SENTINEL_RA as u64;
                    c.pc = 0x0027_4a30;
                    in_seq_instance_update(&mut c);
                    assert_eq!(c.pc, SENTINEL_RA, "{hz} Hz returned");
                }
                let pos = f32::from_bits(c.bus.read32(obj + 0x28));
                assert!(
                    (pos - 60.0 * speed).abs() < 0.01,
                    "{hz} Hz speed {speed}: pos {pos}"
                );
                assert_eq!(
                    f32::from_bits(c.bus.read32(obj + 0x24)),
                    speed,
                    "{hz} Hz keeps its speed"
                );
            }
        }
    }
}
