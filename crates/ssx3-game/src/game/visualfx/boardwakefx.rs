//! `src/visualfx/boardwakefx.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::sp;
use crate::ps2f::FpuF32;
use crate::events;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::funcs::call_indirect;
use crate::events::deliver_event;
pub(crate) use crate::game::render::particle::at as at_2;
pub(crate) use crate::game::render::particle::addi;
pub(crate) use crate::game::object::debouncenode::BOARD_WAKE_WORLD;
pub(crate) use crate::game::object::debouncenode::lw32;
pub(crate) use crate::game::object::splinemodifier::setf as setf_3;
pub(crate) use crate::game::object::splinemodifier::ldf as ldf_3;
pub(crate) use crate::game::object::splinemodifier::stf as stf_2;
pub(crate) use crate::game::replay::replay2::destroy_if_flag;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp_2;
pub(crate) use crate::game::intersect::riderspheretree::p_2;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at_4;
pub(crate) use crate::game::fe::feasyncfile::gpr;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_5;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
pub(crate) use crate::game::fe::ovstates::at as at_6;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::fe::festateselectmp::epilogue;
pub(crate) use crate::game::ai::ridermetrix::at_3 as at_7;
pub(crate) use crate::game::ai::ridermetrix::lw_3;
pub(crate) use crate::game::ai::ridermetrix::sw_2 as sw_3;
pub(crate) use crate::game::ai::ridermetrix::swc1;
pub(crate) use crate::game::ai::ridermetrix::lwc1;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0028_32d8, script_obj_unregister, "ScriptObj_Unregister"),
    (0x002d_d0b8, c_board_wake_fx_update, "cBoardWakeFX_Update"),
    (0x002d_dab8, board_wake_fx_render, "cBoardWakeFX_Render"),
    (0x002d_e398, dynamic_spray_fx_add_random_velocity, "cDynamicSprayFX_AddRandomVelocity"),
    (0x002d_f3b0, dynamic_spray_fx_reset, "cDynamicSprayFX_Reset"),
    (0x002d_f448, c_dynamic_spray_fx_setup_emitters, "cDynamicSprayFX_SetupEmitters"),
    (0x002d_fe88, dynamic_spray_update_emitter3, "cDynamicSprayFX_UpdateEmitter3"),
    (0x002e_0ee8, dynamic_spray_fx_update_emitter0, "cDynamicSprayFX_UpdateEmitter0"),
    (0x002e_1120, dynamic_spray_update_emitter4, "cDynamicSprayFX_UpdateEmitter4"),
    (0x002e_1598, dynamic_spray_fx_update_emitter5, "cDynamicSprayFX_UpdateEmitter5"),
    (0x002e_24d0, dynamic_spray_fx_render, "cDynamicSprayFX_Render"),
    (0x002e_27e8, lens_fx_get_flare_size_for_res, "LensFX_GetFlareSizeForRes"),
    (0x002e_3060, lens_fx_base_dtor, "LensFXBase_dtor"),
    (0x002e_3578, lens_fx_base_push_render_state, "LensFXBase_PushRenderState"),
    (0x002e_3668, lens_fx_base_pop_render_state, "LensFXBase_PopRenderState"),
    (0x002e_3930, c_player_pulse_fx_reset, "cPlayerPulseFX_Reset"),
    (0x002e_39d8, player_pulsefx_update, "cPlayerPulseFX_Update"),
    (0x002e_3af8, c_player_pulse_fx_render, "cPlayerPulseFX_Render"),
    (0x002e_4228, c_transition_man_ctor, "cTransitionMan_ctor"),
    (0x002e_42d0, c_transition_man_dtor, "cTransitionMan_dtor"),
    (0x002e_44f0, c_transition_man_start_out, "cTransitionMan_StartOut"),
    (0x002e_4540, c_transition_man_start_captured, "cTransitionMan_StartCaptured"),
    (0x002e_4578, ctransition_man_stop, "cTransitionMan_Stop"),
    (0x002e_4638, c_transition_man_set_paused, "cTransitionMan_SetPaused"),
    (0x002e_4678, c_transition_man_get_time_remaining, "cTransitionMan_GetTimeRemaining"),
    (0x002e_46c8, c_transition_man_update, "cTransitionMan_Update"),
    (0x002e_47e8, transition_man_render, "cTransitionMan_Render"),
    (0x002e_4b98, c_transition_man_script_activate, "cTransitionMan_ScriptActivate"),
    (0x002e_4c60, c_transition_man_script_deactivate, "cTransitionMan_ScriptDeactivate"),
    (0x002e_4c90, c_transition_man_script_is_running, "cTransitionMan_ScriptIsRunning"),
    (0x002e_4cb0, c_transition_man_reset_effect_pool, "cTransitionMan_ResetEffectPool"),
    (0x002e_4ce8, transition_effect_alloc_data, "TransitionEffect_AllocData"),
    (0x002e_4d30, transition_effect_free_data, "TransitionEffect_FreeData"),
    (0x002e_4d70, transition_effect_ctor, "TransitionEffect_ctor"),
    (0x002e_4d88, snow_box_ctor, "SnowBox_ctor"),
    (0x002e_4f10, snow_box_reset, "SnowBox_Reset"),
    (0x002e_4f28, snow_box_dtor, "SnowBox_dtor"),
    (0x002e_5430, snow_box_wrap_offset, "SnowBox_WrapOffset"),
    (0x002e_55d8, snow_box_render, "SnowBox_Render"),
    (0x002e_5d18, snow_fall_dtor, "SnowFall_dtor"),
    (0x002e_62c8, snow_fall_reset_boxes, "SnowFall_ResetBoxes"),
    (0x002e_6640, c_speed_boost_streamer_fx_reset, "cSpeedBoostStreamerFX_Reset"),
    (0x002e_6c08, c_speed_boost_streamer_fx_update_points, "cSpeedBoostStreamerFX_UpdatePoints"),
    (0x002e_7a10, c_speed_boost_streamer_fx_render, "cSpeedBoostStreamerFX_Render"),
];

// ---------------------------------------------------------------------------
// cSpeedBoostStreamerFX_Render.

/// `cSpeedBoostStreamerFX` fields.
pub(crate) mod boost {

    /// The rider (`+0x780` model, `+0x89c` bone index, `+0x1a0` direction).
    pub const RIDER: u32 = 0;

    /// Age: drawn from 3.0 on; also the flames' fade length.
    pub const FADE: u32 = 0x4;

    /// Alpha scale.
    pub const ALPHA: u32 = 0x8;

    /// Width scale and v start of the streamers.
    pub const WIDTH: u32 = 0xc;

    pub const V_START: u32 = 0x10;

    /// Number of flame segments (the tri strip), its alpha clamp.
    pub const FLAMES: u32 = 0x14;

    pub const FLAME_CLAMP: u32 = 0x18;

    /// Flame ring buffer (0x1e entries of 0x1c bytes from +0x50) and its
    /// head.
    pub const FLAME_HEAD: u32 = 0x30;

    pub const FLAME_RING: u32 = 0x50;

    /// Texture index (`gfx + 0xf50 + 4*i`).
    pub const TEXTURE: u32 = 0x398;

    /// Newest point index, point count, point pairs.
    pub const FIRST: u32 = 0x39c;

    pub const COUNT: u32 = 0x3a0;

    pub const POINTS: u32 = 0x3ac;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_UPDATE_DESCRIPTOR;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_ADVANCE_DESC_TIME;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_ADVANCE_WRITE;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_GET_PAD_STATUS;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_GET_PAD_DATA;
    pub(crate) use crate::game::ai::rider::callee::INPUT_PAD_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::COMM_SYSTEM_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::USB_KEYBOARD_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::MATH_SIN_COS_2 as MATH_SIN_COS;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_RENDER;
    pub(crate) use crate::game::ai::rider::callee::VU0_SAVE_CONTEXT;
    pub(crate) use crate::game::ai::rider::callee::VU0_RESTORE_CONTEXT;
    pub(crate) use crate::game::ai::rider::callee::MODIFIER_BLOCK_GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::object::debouncenode::callee::TRANSITION_MAN_STOP;
    pub(crate) use crate::game::object::debouncenode::callee::SPLINE_MODIFIER_IS_NODE_VISIBLE;
    pub(crate) use crate::game::object::debouncenode::callee::SCRIPT_VIEW_OBJ_BIND_CAM_SLOT;
    pub(crate) use crate::game::camera::camera::callee::MEASURE_ROW;
    pub(crate) use crate::game::camera::camera::callee::MEMCARD_SET_STATE;
    pub(crate) use crate::game::camera::camera::callee::TRANSITION_MAN_START;
    pub(crate) use crate::game::camera::camera::callee::WSCRIPT_DELETE_ALL;
    pub(crate) use crate::game::camera::camera::callee::SPHERE_TREE_RESET_TRANSFORM;
    pub(crate) use crate::game::camera::camera::callee::CAM_CONTROLLER_RESET;
    pub(crate) use crate::game::camera::camera::callee::MANUAL_CAM_ALGO_BASE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_FREE_TRIGGERS;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_RESET_HEADER;
    pub(crate) use crate::game::camera::camera::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::camera::camera::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::camera::camera::callee::BE_LIBRARY_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BEBAGT_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BE_OPTION_GET_MAX_HANDICAP;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_PROFILE_LIST;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_SLOTS;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_INIT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2 as CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2 as SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_13 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3 as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2 as SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXTURE_SET_FIND as UI_TEXTURE_SET_FIND_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET as MC_MANAGER_GET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_RECV;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_CTOR_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_SSX_WORLD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2 as SPCH_MAKE_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;


    pub const ADD_RANDOM_VELOCITY: Callee = Callee {
        addr: 0x002d_e398,
        f: ee::s09::cDynamicSprayFX_AddRandomVelocity,
    };

    pub const ADD_SPLASH: Callee = Callee {
        addr: 0x002f_4118,
        f: ee::s09::cSplashManager_AddSplashAllViews,
    };


    /// Same-shard direct calls (`jal`).
    pub const ANIM_SEQ_LIST_GET_AT: Callee =
        Callee { addr: 0x0031_4760, f: ee::s10::AnimSeqList_GetAt };

    pub const RIDER_COMPUTE_LOD: Callee =
        Callee { addr: 0x0012_1f30, f: ee::s00::Rider_ComputeLOD };

    pub const TRANSITION_EFFECT_UPDATE_SCREEN_RECT: Callee =
        Callee { addr: 0x002e_b8f0, f: ee::s09::TransitionEffect_UpdateScreenRect };


    // Cross-shard direct calls the generator emitted as `call_indirect`.
    pub const RIDER_MODEL_RENDER: u32 = 0x0031_0640;

    pub const FLAGS64_TEST: u32 = 0x0014_46a0;

    pub const MATH_COS: u32 = 0x0031_c040;

    pub const EMITTER_UPDATE: u32 = 0x0037_0788;

    pub const WORLD_PAINTER_QUERY_RESET_ALL: u32 = 0x002c_03e8;
}

// ---------------------------------------------------------------------------
// Snow box wrapping.

/// `SnowBox` fields used by the wrap.
pub(crate) mod snow_box {

    /// Side length of the snow grid cell.
    pub const SIZE: u32 = 0x0c;

    /// Position (x, y, z, w), already offset by the camera-relative shift.
    pub const POS: u32 = 0x20;
}

// ---------------------------------------------------------------------------
// Board wake strip rendering.

/// The renderer object (`cGfx`) at gp-0x854.
pub(crate) mod gfx {

    /// Top of the 20-byte state stack.
    pub const STATE_TOP: u32 = 0xe84;

    /// The renderer's frame counter (`Get1030`).
    pub const F1030: u32 = 0x1030;

    /// The renderer's virtual-method table.
    pub const VTABLE: u32 = 0x10d8;
}

/// `sWake_DAB8`: the board wake trail.
pub(crate) mod wake_dab8 {

    pub const COUNT: u32 = 0x04;

    pub const FIRST: u32 = 0x08;

    /// Pointers to the 0x70-byte trail records (age at +0x64).
    pub const VERTS: u32 = 0x0c;

    /// The strip vertex array drawn by the renderer.
    pub const STRIP: u32 = 0x10;

    pub const TEX: u32 = 0xb4;

    pub const MAX: u32 = 0xb8;

    pub const LIFE: u32 = 0xbc;

    pub const VERTEX_STRIDE: u32 = 0x70;

    pub const VERTEX_AGE: u32 = 0x64;
}

// ---------------------------------------------------------------------------
// Spray emitters.

pub(crate) mod spray {

    /// The rider object (velocity source, up vector, trail point).
    pub const OBJ: u32 = 0x00;

    /// The dynamic emitter the spray feeds (`self->em`).
    pub const EMITTER: u32 = 0xb0;

    /// The emitter object pointer used for the position (`self->p28`).
    pub const EMITTER_PTR: u32 = 0x28;

    /// Curve table, +0xd8/+0xdc/+0xe0 are velocity scales.
    pub const CURVES: u32 = 0x2c;

    /// Spray direction.
    pub const DIR: u32 = 0x30;

    /// Colour (RGBA).
    pub const COL: u32 = 0x90;

    /// Position.
    pub const POS: u32 = 0xa0;

    /// Normalised speed (0..~2): gates and scales the spray.
    pub const SPEED: u32 = 0xbc;

    /// Configuration object, +0x58 enables the spray.
    pub const CFG: u32 = 0xd0;

    /// `cDynamicSprayFX+0x28`: the emitter (its particle block starts at
    /// `+0x630`), `+0xb0`: the spawn-interval timer.
    pub const EMITTER_2: u32 = 0x28;

    pub const INTERVAL: u32 = 0xb0;

    /// The random-number state (`gp+0xa0c`).
    pub const RANDOM_GP: i32 = 0xa0c;
}

// ---------------------------------------------------------------------------
// Snow box cell rendering.

/// `SnowBox` cell fields read by the renderer.
pub(crate) mod snow_cell {

    /// 0: an ordinary cell, 1: the special cell, other: not drawn.
    pub const KIND: u32 = 0x04;

    /// Non-zero when the cell is on screen.
    pub const ACTIVE: u32 = 0x08;

    /// Cell size (used to scale the cell-local position).
    pub const SIZE: u32 = 0x0c;

    /// Extra alpha/height value, scaled by 128.
    pub const HEIGHT: u32 = 0x14;

    /// Colour scale, scaled by 128.
    pub const EXTRA: u32 = 0x18;

    /// Cell centre (divided by `size` into the cell grid).
    pub const POS: u32 = 0x20;

    /// The cell's model matrix.
    pub const MAT: u32 = 0x50;

    pub const CELL_ID: u32 = 0x98;

    pub const I9C: u32 = 0x9c;

    pub const IA0: u32 = 0xa0;

    /// RGB colour (0..1, scaled by 128 here).
    pub const COL_R: u32 = 0xa4;

    pub const COL_G: u32 = 0xa8;

    pub const COL_B: u32 = 0xac;
}

// ---------------------------------------------------------------------------
// Spray FX emitter update (the main spray, emitter slot 5).

/// `cDynamicSprayFX` fields used by the main emitter update.
pub(crate) mod spray5 {

    pub const OBJ: u32 = 0x00;

    /// Spray level (fades 0..2).
    pub const LEVEL: u32 = 0x04;

    /// Base of the dynamic emitter array.
    pub const P28: u32 = 0x28;

    /// Curve table used for the velocity scales.
    pub const P2C: u32 = 0x2c;

    pub const DIR: u32 = 0x30;

    pub const COL: u32 = 0x90;

    pub const POS: u32 = 0xa0;

    /// Configuration object.
    pub const CFG: u32 = 0xd0;

    /// Mode flag (2: double rate).
    pub const ID4: u32 = 0xd4;

    /// Spray strength (0..~3333), the fade gate.
    pub const SPEED: u32 = 0xe0;

    /// Position the splash/trail follows.
    pub const FRAME_POS: u32 = 0xf0;

    /// Direction the spray is thrown along.
    pub const THROW: u32 = 0x100;

    /// Non-zero switches to the slower/other rate.
    pub const I114: u32 = 0x114;

    /// Smoothed alpha.
    pub const ALPHA: u32 = 0x120;

    /// Non-zero disables the decay multiplier.
    pub const I124: u32 = 0x124;
}

/// The spray's configuration object.
pub(crate) mod spray_cfg {

    pub const ENABLED: u32 = 0x60;

    /// Enables the long-range mode.
    pub const FLAG: u32 = 0x64;

    /// Decay multiplier applied to the spray strength.
    pub const DECAY: u32 = 0x68;
}

/// The game's render-context singleton (`gp+0x2a90`, `D_004A5B80`): its
/// vtable (`+0x10d8`), the render-state stack pointer (`+0xe84`) and the
/// texture-id table (`+0xf50`).
#[allow(dead_code)]
pub(crate) mod ctx {

    pub const VTABLE: u32 = 0x10d8;

    pub const TOP: u32 = 0xe84;

    pub const TEX_IDS: u32 = 0xf50;
}

/// `sAmCtx` (`D_004A289C`) used by `Rider_RenderModel`, and the world
/// pointer (`gp-0x848`) whose `+0x84 -> +0x84 -> +0x14` is the active rider.
#[allow(dead_code)]
pub(crate) mod globals {

    pub const AM_CTX: i32 = -0x854;

    pub const WORLD: i32 = -0x848;

    /// `D_0044B420`: billboard blend mode by emitter kind.
    pub const EMITTER_BLEND: u32 = 0x0044_b420;

    /// `D_004FF1A0`: the identity matrix flag groups start from.
    pub const IDENTITY_MAT: u32 = 0x004f_f1a0;

    /// `D_00501420`: the base flag render state.
    pub const FLAG_STATE: u32 = 0x0050_1420;

    /// The two global bone-matrix arrays the renderers build.
    pub const BONE_MATRICES: u32 = 0x004f_c420;

    pub const BONE_MATRICES_SHADOW: u32 = 0x004f_d420;

    /// `D_004A4244` (flag manager) and the flag/model table behind it.
    pub const FLAG_MANAGER: i32 = 0x1154;

    pub const FLAG_TABLE: i32 = 0x16c8;
}

/// `Rider` fields the renderer wrapper reads.
#[allow(dead_code)]
pub(crate) mod rider {

    pub const MODEL: u32 = 0x780;

    pub const ANIM: u32 = 0x784;

    pub const LOD: u32 = 0x898;

    pub const ON_SCREEN: u32 = 0xb18;

    pub const HIDDEN: u32 = 0xb1c;

    pub const HANDLER: u32 = 0x77c;

    pub const SUB_STATE: u32 = 0x2f4;

    pub const MATRIX: u32 = 0x7c0;

    pub const POSITION: u32 = 0x110;
}

/// The wake-trail object's fields (reference `sWakeDCF28` in
/// `src/visualfx/boardwakefx.cpp`).
#[allow(dead_code)]
pub(crate) mod wake {

    pub const EMITTER: u32 = 0x174;

    pub const LIVE: u32 = 0x1e0;

    /// The rider this trail belongs to.
    pub const OBJ: u32 = 0x00;

    /// Number of live sections (the outer loop bound).
    pub const LIVE_2: u32 = 0x04;

    /// Section cursor: `(first + i) % max` picks the section slot.
    pub const CURSOR: u32 = 0x08;

    /// `sWakeStrip[5]`: the cross-sections, 0x70 bytes each.
    pub const STRIPS: u32 = 0x0c;

    /// Five element rings, one per point of a section (`sWakeElemBW`).
    pub const ROWS: u32 = 0x10;

    /// Five per-ring weights (the points fade out down the section).
    pub const VALS: u32 = 0x24;

    /// Ring time (advanced by `cBoardWakeFX_AdvanceRing`).
    pub const RING: u32 = 0x38;

    /// Set once the current trail mesh has been built.
    pub const BUILT: u32 = 0x3c;

    /// Scratch float.
    pub const F40: u32 = 0x40;

    /// A 0x40-byte sub-object.
    pub const SUB: u32 = 0x60;

    /// Trail colour (four floats).
    pub const COL: u32 = 0xa0;

    /// Non-zero while a previous point exists.
    pub const HAS_PREV: u32 = 0xb0;

    /// Points per section (`count` in the reference struct).
    pub const SPAN: u32 = 0xb4;

    /// Slots in a cross-section ring (`fB8`).
    pub const MAX: u32 = 0xb8;

    /// Section lifetime in seconds (`fBC`).
    pub const LIFE: u32 = 0xbc;
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {
    pub(crate) use crate::game::be::bedbreward::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::be::bedbreward::table_callee::EA_MEMSET;
    pub(crate) use crate::game::be::bedbreward::table_callee::WORLD_CREATE_DEAD_NODE;
}

pub(crate) mod spray_callee {
    use super::Callee;
    use ssx3_ee as ee;


    pub const ADD_RANDOM_VELOCITY: Callee = Callee {
        addr: 0x002d_e398,
        f: ee::s09::cDynamicSprayFX_AddRandomVelocity,
    };

    pub const SPAWN_AT: Callee = Callee { addr: 0x0037_17c0, f: ee::s12::DynamicEmitter_SpawnAt };

    pub const WORLD_PAINTER_GET_WEATHER: Callee = Callee {
        addr: 0x002e_e6f0,
        f: ee::s09::WorldPainter_GetWeather30,
    };
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::STRLEN;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
}

/// Cross-shard calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_SYSTEM;
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_TRICK_TICK;
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_CALL_A;
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_CALL_B;
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_CALL_C;
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_CALL_D;
    pub(crate) use crate::game::ai::ridermetrix::table::BE_GET_BE;
    pub(crate) use crate::game::ai::ridermetrix::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::ai::ridermetrix::table::BE_CALL;
    pub(crate) use crate::game::ai::ridermetrix::table::BOARD_WAKE_CLEAR;
    pub(crate) use crate::game::ai::ridermetrix::table::BOARD_DONE;
    pub(crate) use crate::game::ai::ridermetrix::table::BOARD_WAKE_A;
    pub(crate) use crate::game::ai::ridermetrix::table::BOARD_WAKE_B;
    pub(crate) use crate::game::ai::ridermetrix::table::SIN;
    pub(crate) use crate::game::ai::ridermetrix::table::PLAY_ANIM;
}


// ---------------------------------------------------------------------------
// Transition managers.

/// `void cTransitionMan_SetPaused(cTransitionMan *this, int paused)`
/// (0x2e4638): while active, bump or drop the pause hold counter at +0x40.
pub fn c_transition_man_set_paused(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x44).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        let paused = c.g[A1][0];
        c.g[V0][0] = Addr(this).off(0x40).lw(c);
        if paused == 0 {
            c.cycles += 2;
            let was_zero = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            if !was_zero {
                c.cycles += 1;
                Addr(this).off(0x40).sw(c, c.g[V0][0]);
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(this).off(0x40).sw(c, c.g[V0][0]);
            abi::ret(c);
            return;
        }
    } else {
        c.cycles += 2;
        Addr(this).off(0x40).sw(c, 0);
        abi::ret(c);
        return;
    }
}


/// `float cTransitionMan_GetTimeRemaining(cTransitionMan *this)` (0x2e4678):
/// remaining transition time, clamped at zero.
pub fn c_transition_man_get_time_remaining(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(0x44).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.mtc1(ZERO, 0); // f0 = 0
    } else {
        c.cycles += 12;
        c.lwc1(0, this.wrapping_add(0x58));
        c.lwc1(1, this.wrapping_add(0x54));
        c.lwc1(3, this.wrapping_add(0x5c));
        c.add_s(1, 1, 0);
        c.lwc1(4, this.wrapping_add(0x48));
        c.mtc1(ZERO, 2); // f2 = 0
        c.add_s(1, 1, 3);
        c.sub_s(1, 1, 4);
        c.c_lt_s(1, 2);
        let below = c.fcc();
        c.mov_s(0, 2);
        if !below {
            c.cycles += 2;
            c.mov_s(0, 1);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void TransitionEffect_ctor(TransitionEffect *this)` (0x2e4d70): install
/// the vtable at +0.
pub fn transition_effect_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let this = abi::arg(c, 0);
    c.g[V1][0] = 0x49_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7f20));
    Addr(this).sw(c, c.g[V1][0]);
    abi::ret(c);
}


/// `void SnowBox_Reset(SnowBox *this)` (0x2e4f10): clear the particle ranges
/// and arm the box.
pub fn snow_box_reset(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x1;
    let this = abi::arg(c, 0);
    Addr(this).off(0x90).sw(c, 0);
    Addr(this).off(0xb0).sw(c, c.g[V0][0]);
    Addr(this).off(0x94).sw(c, 0);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Speed-boost streamer.

/// `void cSpeedBoostStreamerFX_Reset(cSpeedBoostStreamerFX *this)` (0x2e6640):
/// build the default emit descriptor in the local frame, splat it into the
/// object and set the streamer id from the owning rider's id.
pub fn c_speed_boost_streamer_fx_reset(c: &mut Cpu) {
    c.cycles += 29;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0); // f0 = 1.0
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    let sp = c.g[SP][0] as u32;
    let this = abi::arg(c, 0);
    Addr(sp).off(0x4).sw(c, 0);
    c.g[V1][0] = 0x39;
    c.swc1(0, sp.wrapping_add(0xc));
    c.g[A2][0] = 0x14;
    c.swc1(0, sp);
    c.g[A1][0] = 0x12;
    Addr(sp).off(0x8).sw(c, 0);
    Addr(this).off(0x398).sw(c, c.g[V1][0]);
    c.g[V1][0] = Addr(this).lw(c);
    c.lq(2, sp);
    Addr(this).off(0x8).sw(c, 0);
    c.sq(2, this.wrapping_add(0x40));
    Addr(this).off(0xc).sw(c, 0);
    Addr(this).off(0x10).sw(c, 0);
    Addr(this).off(0x18).sw(c, 0);
    Addr(this).off(0x14).sw(c, 0);
    Addr(this).off(0x30).sw(c, 0);
    Addr(this).off(0x3a0).sw(c, 0);
    Addr(this).off(0x39c).sw(c, 0);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x870)).lw(c);
    Addr(this).off(0x3a8).sw(c, 0);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 0i64) as u64;
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[A2][0];
    }
    Addr(this).off(0x3a4).sw(c, c.g[A1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// The graphics manager and its GS state stack.

/// `*(gp - GFX_MAN)`: the graphics manager.
pub(crate) const GFX_MAN: u32 = 0x854;

/// `*(gp - APP)`: the application (`+0x84` the game module).
pub(crate) const APP: u32 = 0x848;

/// `gfx + GS_TOP`: pointer to the top GS state record.
pub(crate) const GS_TOP: u32 = 0xe84;

/// Size of a GS state record.
pub(crate) const RECORD: u32 = 0x14;

/// The default GS state record (`0x500000 + 0x1420`).
pub(crate) const GS_DEFAULT: u32 = 0x0050_1420;

/// `gfx + GFX_VTABLE`: the manager's vtable.
pub(crate) const GFX_VTABLE: u32 = 0x10d8;

/// Vtable slot of `Draw(gfx, count, verts, flags)`.
pub(crate) const DRAW: u32 = 0x238;


/// `base + off` as a guest address (`addiu`/`lw` offsets).
#[inline(always)]
pub(crate) fn at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}


/// The graphics manager pointer (`lw gp-0x854`).
#[inline]
pub(crate) fn gfx(c: &mut Cpu) -> u64 {
    let a = r32(c, GP).wrapping_sub(GFX_MAN);
    c.lw(a)
}


/// `rd = sx(rd + k)` on a 32-bit register value.
#[inline(always)]
pub(crate) fn addiu(c: &mut Cpu, rd: usize, rs: usize, k: u32) {
    c.g[rd][0] = sx(r32(c, rs).wrapping_add(k));
}


/// Copy a GS state record from `src` to `dst` as gcc does: two unaligned
/// doublewords through `lo` and `hi` (`ldl`/`ldr`, `sdl`/`sdr`) and the last
/// word through `word`.
pub(crate) fn copy_record(c: &mut Cpu, src: u32, dst: u32, lo: usize, hi: usize, word: usize) {
    c.ldl(lo, src.wrapping_add(7));
    c.ldr(lo, src);
    c.ldl(hi, src.wrapping_add(0xf));
    c.ldr(hi, src.wrapping_add(8));
    c.g[word][0] = c.lw(src.wrapping_add(0x10));
    c.sdl(lo, dst.wrapping_add(7));
    c.sdr(lo, dst);
    c.sdl(hi, dst.wrapping_add(0xf));
    c.sdr(hi, dst.wrapping_add(8));
    let w = c.g[word][0];
    c.sw(dst.wrapping_add(0x10), w);
}


/// One bit-field edit of the top GS record: `top = gfx->gs_top` (into
/// `top_reg`), `top[off] = (top[off] & keep) | set` (through v0).
pub(crate) fn gs_field(c: &mut Cpu, gfx: u64, top_reg: usize, off: u32, keep: u64, set: u64) {
    let top = c.lw(at(gfx, GS_TOP));
    c.g[top_reg][0] = top;
    let v = (c.lw(at(top, off)) & keep) | set;
    c.g[V0][0] = v;
    c.sw(at(top, off), v);
}


/// Call method `slot` of the graphics manager whose pointer is in
/// `gfx_reg`: the vtable goes to `vt_reg`, the method to `fn_reg`, the
/// adjusted `this` to a0 (the caller has set a1-a3).
pub(crate) fn gfx_method(c: &mut Cpu, gfx_reg: usize, slot: u32, vt_reg: usize, fn_reg: usize, ret: u32) -> Flow {
    let gfx = c.g[gfx_reg][0];
    let vt = c.lw(at(gfx, GFX_VTABLE));
    c.g[vt_reg][0] = vt;
    let delta = c.lh(at(vt, slot));
    c.g[A0][0] = delta;
    let f = c.lw(at(vt, slot + 4));
    c.g[fn_reg][0] = f;
    c.g[A0][0] = sx((gfx as u32).wrapping_add(delta as u32));
    call_ptr(c, f as u32, ret)
}


/// `Draw(gfx, count, verts, flags)` (method 0x238), see [`gfx_method`].
pub(crate) fn draw(c: &mut Cpu, gfx_reg: usize, vt_reg: usize, fn_reg: usize, ret: u32) -> Flow {
    gfx_method(c, gfx_reg, DRAW, vt_reg, fn_reg, ret)
}


/// Pop the GS state record pushed at the start (`gfx->gs_top -= 0x14`), the
/// manager in `gfx_reg`, the new top in v0.
pub(crate) fn pop_record(c: &mut Cpu, gfx_reg: usize) {
    let gfx = gfx(c);
    c.g[gfx_reg][0] = gfx;
    let top = c.lw(at(gfx, GS_TOP));
    c.g[V0][0] = sx((top as u32).wrapping_sub(RECORD));
    let v = c.g[V0][0];
    c.sw(at(gfx, GS_TOP), v);
}


/// An event check at a loop back edge (structured loop).
#[inline]
pub(crate) fn back_edge(c: &mut Cpu, head: u32) {
    if event_due(c) {
        deliver_event_here(c, head);
    }
}


/// `vf5 = vf4 * vf3.x` with `vf4 = *src` and `vf3 = gpr` (`qmtc2`, all 128
/// bits: the float was moved in with `mfc1` into the low word), stored to
/// `dst`. The scale `f` goes through `gpr` as the original does.
pub(crate) fn vmul_bc(c: &mut Cpu, gpr: usize, f: usize, src: u32, dst: u32) {
    c.mfc1(gpr, f);
    c.lqc2(4, src);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, dst);
}


/// `vf5 = *a + *b` (or `- *b`) through vf3/vf4, stored to `dst`.
pub(crate) fn vadd(c: &mut Cpu, a: u32, b: u32, dst: u32, sub: bool) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    if sub {
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    } else {
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    }
    c.sqc2(5, dst);
}


/// `lq v0, src; sq v0, dst` (a quadword copied through v0).
pub(crate) fn copy_quad(c: &mut Cpu, src: u32, dst: u32) {
    c.lq(V0, src);
    c.sq(V0, dst);
}


// ---------------------------------------------------------------------------
// Streamers (cAirStreamerFX, cSpeedBoostStreamerFX).

/// Where a streamer keeps its data: the effect pointer's register, its
/// point list and point count fields, and the stack layout (a temporary
/// block at `sp+tmp`: +0 the offset added to the points, +0x10 the colour
/// (alpha, r, g, b), +0x20/+0x30 the two points of a segment, +0x50 the
/// half width, +0x60/+0x70 scratch; two vertex arrays `half` bytes apart
/// from sp+0).
pub(crate) struct Streamer {
    pub(crate) this: usize,
    pub(crate) points: u32,
    pub(crate) count: u32,
    pub(crate) tmp: u32,
    pub(crate) half: u32,
}


/// One streamer vertex at `v`: position `*p ± *(t+0x50)` (through
/// vf3-vf5, t+0x70, v0 and t+0x60), colour from t+0x10.. (rgb * f2 = 255,
/// alpha * f6 = 128), f3 at +8, f4 at +4 and f`first` at +0.
pub(crate) fn streamer_vertex(c: &mut Cpu, t: u32, p: u32, sub: bool, v: u32, first: usize) {
    vadd(c, p, t.wrapping_add(0x50), t.wrapping_add(0x70), sub);
    copy_quad(c, t.wrapping_add(0x70), t.wrapping_add(0x60));
    c.lwc1(1, t.wrapping_add(0x14));
    c.mul_s(1, 1, 2);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x10));
    c.lwc1(0, t.wrapping_add(0x18));
    c.mul_s(0, 0, 2);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x14));
    c.lwc1(1, t.wrapping_add(0x1c));
    c.mul_s(1, 1, 2);
    c.cvt_w_s(0, 1);
    c.swc1(0, v.wrapping_add(0x18));
    c.lwc1(0, t.wrapping_add(0x10));
    c.swc1(3, v.wrapping_add(8));
    c.mul_s(0, 0, 6);
    c.cvt_w_s(1, 0);
    c.swc1(1, v.wrapping_add(0x1c));
    c.lq(V0, t.wrapping_add(0x60));
    c.sq(V0, v.wrapping_add(0x20));
    c.swc1(first, v);
    c.swc1(4, v.wrapping_add(4));
}


/// One streamer segment (the 137-cycle loop body both streamers share):
/// point `a2` (index into the 32-byte point pairs, then `a2 -= 1`) plus
/// the offset gives the segment's two points; each array gets the pair
/// `p - w`, `p + w` (v from f4, advanced by f8; f7 += 1). a3 counts the
/// segments, s0 the vertices per array, a1 walks the second array. Returns
/// whether `a3 < count`.
pub(crate) fn streamer_segment(c: &mut Cpu, s: &Streamer) -> bool {
    let t = r32(c, SP).wrapping_add(s.tmp);
    c.cycles += 137;
    c.g[V0][0] = c.lw(at(c.g[s.this][0], s.points));
    c.g[A0][0] = sx(r32(c, A2) << 5);
    addiu(c, A3, A3, 1);
    addiu(c, S0, S0, 2);
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
    addiu(c, A2, A2, u32::MAX);
    let p0 = r32(c, V0);
    vadd(c, p0, t, t.wrapping_add(0x70), false);
    c.add_s(7, 7, 3);
    c.g[V1][0] = c.lw(at(c.g[s.this][0], s.points));
    c.lq(V0, t.wrapping_add(0x70));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    c.sq(V0, t.wrapping_add(0x60));
    c.sq(V0, t.wrapping_add(0x20));
    let p1 = r32(c, A0).wrapping_add(0x10);
    vadd(c, p1, t, t.wrapping_add(0x70), false);
    copy_quad(c, t.wrapping_add(0x70), t.wrapping_add(0x60));
    c.sq(V0, t.wrapping_add(0x30));
    let a = r32(c, A1);
    let (pa, pb) = (t.wrapping_add(0x20), t.wrapping_add(0x30));
    streamer_vertex(c, t, pa, true, a.wrapping_sub(s.half), 5);
    streamer_vertex(c, t, pb, true, a, 5);
    let a = a.wrapping_add(0x30);
    c.g[A1][0] = sx(a);
    streamer_vertex(c, t, pa, false, a.wrapping_sub(s.half), 3);
    streamer_vertex(c, t, pb, false, a, 3);
    let n = c.lw(at(c.g[s.this][0], s.count));
    c.g[V1][0] = ((c.g[A3][0] as i64) < (n as i64)) as u64;
    c.add_s(4, 4, 8);
    let more = c.g[V1][0] != 0;
    c.g[A1][0] = sx(a.wrapping_add(0x30));
    more
}


// ---------------------------------------------------------------------------
// cPlayerPulseFX_Render.

/// `BELibrary_GetInstance` and `BELibrary_IsMode`-style query (0x14dd58:
/// `phase interface->method 8(a1)`), reached through the function table.
pub(crate) const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

pub(crate) const BE_LIBRARY_QUERY: u32 = 0x0014_dd58;

/// Game globals the pulse reads: a "split screen" switch and the game mode
/// byte (modes 4 and 5 are the event modes).
pub(crate) const SPLIT_SWITCH: u32 = 0x0053_4b30;

pub(crate) const GAME_MODE: u32 = 0x0053_5c12;


/// The rider's "is the local player" query: method 0x40 of the rider's
/// interface at `+0x6c0` (`this` = `*(s0) + 0x6c0`, adjusted).
pub(crate) fn pulse_rider_query(c: &mut Cpu, ret: u32) -> Flow {
    let rider = c.lw(at(c.g[S0][0], 0));
    c.g[A0][0] = rider;
    let vt = c.lw(at(rider, 0x6c0));
    c.g[V1][0] = vt;
    c.g[A0][0] = sx(at(rider, 0x6c0));
    c.g[A1][0] = c.lh(at(vt, 0x40));
    let f = c.lw(at(vt, 0x44));
    c.g[V0][0] = f;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    call_ptr(c, f as u32, ret)
}


/// `void cPlayerPulseFX_Render(cPlayerPulseFX *fx)` (0x2e3af8): decide
/// whether the pulse around the player shows and draw it. With one player
/// it shows when the rider's query (method 0x40 of `rider+0x6c0`) answers
/// 0, the game mode byte is 4 or 5 and the mode object's id is 14..16
/// (after `BELibrary_GetInstance` and a back-end query); in split screen
/// when the query answers non-zero, there are at least two players and
/// the player id differs from `rider+0x870`, or, when the split switch is
/// set, when `rider+0x87c` is 0. Never while the rider is hidden
/// (`+0xac4`). The pulse is one 8-vertex strip: four points on the
/// camera's y axis (scales gp-0x3a24, gp-0x3a20, 130 and 0) around the
/// rider's bone, each split in two along the camera's x axis (* 85),
/// coloured from `fx+0x28..0x34` (rgb * 255, alpha * 128).
///
/// Frame 0x250: s0 +0x240, s1 +0x230, ra +0x220.
pub fn c_player_pulse_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 12;
        let frame = Frame::open(c, 0x250);
        let app = c.lw(gp.wrapping_sub(APP));
        c.g[V0][0] = app;
        frame.save(c, S0, 0x240);
        frame.save(c, S1, 0x230);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x220);
        c.g[V1][0] = c.lw(at(app, 0x84));
        c.g[A0][0] = c.lw(at(c.g[V1][0], 0xc));
        let players = c.lw(at(c.g[A0][0], 0x7c));
        c.g[V0][0] = ((players as i64) < 2) as u64;
        let single = c.g[V0][0] != 0;
        c.g[S1][0] = 0;
        if !single {
            c.cycles += 9;
            pulse_rider_query(c, 0x002e_3b44)?;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(gp.wrapping_sub(APP));
            let mut direct = false;
            if !none {
                c.cycles += 6;
                let module = c.lw(at(c.g[V0][0], 0x84));
                c.g[V1][0] = module;
                c.g[V1][0] = c.lw(at(module, 0x84));
                let n = c.lw(at(c.g[V1][0], 0x10));
                c.g[V0][0] = (n < 2) as u64;
                direct = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(gp.wrapping_sub(APP));
                if !direct {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(at(c.g[S0][0], 0));
                    c.g[V1][0] = c.lw(at(c.g[V1][0], 0x14));
                    c.g[A0][0] = c.lw(at(c.g[V0][0], 0x870));
                    c.g[V1][0] ^= c.g[A0][0];
                    c.g[S1][0] = (0 < c.g[V1][0]) as u64;
                }
            }
            if !direct {
                c.cycles += 1;
                c.g[V0][0] = c.lw(gp.wrapping_sub(APP));
            }
            c.cycles += 7;
            c.g[A1][0] = 7;
            c.g[A0][0] = c.lw(at(c.g[V0][0], 0x78));
            call_ptr(c, BE_LIBRARY_QUERY, 0x002e_3b88)?;
            c.g[V1][0] = 0x53_0000;
            let split = c.lw(SPLIT_SWITCH);
            c.g[V0][0] = split;
            c.g[V0][0] = c.lw(at(c.g[S0][0], 0));
            if split != 0 {
                c.cycles += 3;
                c.g[V1][0] = c.lw(at(c.g[V0][0], 0x87c));
                c.g[S1][0] = (c.g[V1][0] < 1) as u64;
            }
        } else {
            c.cycles += 9;
            pulse_rider_query(c, 0x002e_3bc0)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw(at(c.g[S0][0], 0));
            } else {
                c.cycles += 11;
                call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x002e_3bd0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, BE_LIBRARY_QUERY, 0x002e_3bdc)?;
                c.g[V1][0] = 0x53_0000;
                let mode = c.lb(GAME_MODE);
                c.g[V0][0] = sx((mode as u32).wrapping_sub(4));
                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                let event = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(gp.wrapping_sub(APP));
                if event {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(at(c.g[V0][0], 0x84));
                    c.g[A0][0] = c.lw(at(c.g[V1][0], 0x78));
                    let id = c.lw(at(c.g[A0][0], 0x1bc));
                    c.g[V0][0] = sx((id as u32).wrapping_sub(0xe));
                    c.g[S1][0] = (c.g[V0][0] < 3) as u64;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c.g[S0][0], 0));
            }
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw(at(c.g[V0][0], 0xac4));
        if c.g[V1][0] != 0 {
            c.g[S1][0] = 0;
        }
        let show = c.g[S1][0] != 0;
        c.g[A1][0] = gfx(c);
        if show {
            pulse_draw(c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x240);
        frame.restore(c, S1, 0x230);
        frame.restore_ra(c, 0x220);
        frame.close(c);
        Ok(())
    })
}


/// The pulse strip (the 381-cycle block): GS state push, the camera matrix
/// (method 0x118), eight vertices at sp+0x80, draw, pop.
pub(crate) fn pulse_draw(c: &mut Cpu) -> Flow {
    let sp = r32(c, SP);
    let gp = r32(c, GP);
    let s = |off: u32| sp.wrapping_add(off);
    c.cycles += 381;
    let g = c.g[A1][0];
    c.g[T0][0] = 0x50_0000;
    c.g[A2][0] = 0xffff_ffff_ffbf_ffff;
    c.g[T1][0] = 0x40_0000;
    let top = c.lw(at(g, GS_TOP));
    c.g[V0][0] = top;
    c.g[T2][0] = 0xffff_ffff_ffff_ff83;
    c.g[T3][0] = 0xffff_ffff_ffff_fffc;
    copy_record(c, top as u32, at(top, RECORD), V1, A0, A3);
    c.g[A3][0] = 0xffff_ffff_e000_03ff;
    c.g[T4][0] = 0xffff_ffff_ffff_fc1f;
    let top = c.lw(at(g, GS_TOP));
    c.g[A0][0] = gfx(c);
    c.g[V0][0] = sx(at(top, RECORD));
    let new_top = c.g[V0][0];
    c.sw(at(g, GS_TOP), new_top);
    let top = c.lw(at(c.g[A0][0], GS_TOP));
    c.g[V1][0] = top;
    c.g[A1][0] = sx(GS_DEFAULT);
    copy_record(c, GS_DEFAULT, top as u32, T5, V0, A0);
    let g = gfx(c);
    c.g[A0][0] = g;
    gs_field(c, g, V1, 4, 0xffff_ffff_ffbf_ffff, 0x40_0000);
    gs_field(c, g, A1, 4, 0xffff_ffff_ffff_ff83, 0x1c);
    gs_field(c, g, A2, 4, 0xffff_ffff_ffff_fffc, 2);
    gs_field(c, g, V1, 8, 0xffff_ffff_e000_03ff, 0);
    gs_field(c, g, A1, 8, 0xffff_ffff_ffff_fc1f, 0xe0);
    let top = c.lw(at(g, GS_TOP));
    c.g[V1][0] = top;
    let tex = c.lw(at(g, 0xffc));
    c.g[V0][0] = tex;
    c.sh(at(top, 0x10), tex);
    // The camera matrix.
    c.g[A1][0] = gfx(c);
    gfx_method(c, A1, 0x118, V1, V0, 0x002e_3d34)?;
    let m = r32(c, V0);
    // sp+0: the camera's x axis, sp+0x10 its y axis (w = 0).
    c.lwc1(2, m);
    c.g[T0][0] = sx(s(0x20));
    c.lwc1(0, m.wrapping_add(0x10));
    c.g[A2][0] = sx(s(0x80));
    c.lwc1(1, m.wrapping_add(0x20));
    c.g[T1][0] = u64::MAX;
    c.swc1(2, s(0));
    c.g[A3][0] = 7;
    c.swc1(0, s(4));
    c.swc1(1, s(8));
    c.sw(s(0xc), 0);
    c.g[AT][0] = 0x42aa_0000;
    c.mtc1(AT, 3);
    c.lwc1(2, m.wrapping_add(0x24));
    c.lwc1(0, m.wrapping_add(4));
    c.lwc1(1, m.wrapping_add(0x14));
    let rider = c.lw(at(c.g[S0][0], 0));
    c.g[A1][0] = rider;
    c.swc1(0, s(0x10));
    c.swc1(1, s(0x14));
    c.swc1(2, s(0x18));
    c.sw(s(0x1c), 0);
    // The centre: the rider's bone (+0x8a8) position at sp+0x20.
    c.lwc1(0, gp.wrapping_sub(0x3a24));
    c.g[A0][0] = c.lw(at(rider, 0x780));
    c.g[V0][0] = c.lw(at(rider, 0x8a8));
    c.g[V1][0] = c.lw(at(c.g[A0][0], 0x30));
    c.g[V0][0] = sx(r32(c, V0) << 6);
    c.lwc1(1, gp.wrapping_sub(0x3a20));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[AT][0] = 0x4302_0000;
    c.mtc1(AT, 2);
    let bone = r32(c, V1);
    copy_quad(c, bone.wrapping_add(0x30), s(0x20));
    // Ring points: centre + y * f0 / f1 / f2 (sp+0x30/0x40/0x50), the
    // split offset x * 85 at sp+0x60.
    vmul_bc(c, V1, 0, s(0x10), s(0x50));
    copy_quad(c, s(0x50), s(0x40));
    vadd(c, s(0x20), s(0x40), s(0x50), false);
    copy_quad(c, s(0x50), s(0x30));
    vmul_bc(c, V1, 1, s(0x10), s(0x60));
    copy_quad(c, s(0x60), s(0x50));
    vadd(c, s(0x20), s(0x50), s(0x60), false);
    copy_quad(c, s(0x60), s(0x40));
    vmul_bc(c, V1, 2, s(0x10), s(0x70));
    copy_quad(c, s(0x70), s(0x60));
    vadd(c, s(0x20), s(0x60), s(0x70), false);
    copy_quad(c, s(0x70), s(0x50));
    vmul_bc(c, V1, 3, s(0), s(0x70));
    c.lq(V0, s(0x70));
    // The rider's colour (fx+0x2c..0x34) with alpha 0 at sp+0x70.
    let fx = c.g[S0][0];
    c.lwc1(0, at(fx, 0x2c));
    c.lwc1(1, at(fx, 0x30));
    c.lwc1(2, at(fx, 0x34));
    c.sq(V0, s(0x60));
    c.swc1(0, s(0x74));
    c.swc1(1, s(0x78));
    c.sw(s(0x70), 0);
    c.swc1(2, s(0x7c));
    // Vertex 0 (sp+0x80): centre - offset, colour sp+0x70.
    vadd(c, s(0x20), s(0x60), s(0x210), true);
    c.g[AT][0] = 0x437f_0000;
    c.mtc1(AT, 4);
    c.lwc1(0, s(0x74));
    c.g[A1][0] = 8;
    c.g[AT][0] = 0x4300_0000;
    c.mtc1(AT, 6);
    c.g[A3][0] = 0;
    c.lwc1(1, s(0x78));
    c.mul_s(0, 0, 4);
    c.lwc1(2, s(0x7c));
    c.lwc1(3, s(0x70));
    c.mul_s(1, 1, 4);
    c.lq(V0, s(0x210));
    c.mul_s(2, 2, 4);
    c.mul_s(3, 3, 6);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 5);
    c.cvt_w_s(7, 0);
    c.swc1(7, s(0x90));
    c.sq(V0, s(0x200));
    c.sq(V0, s(0xa0));
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x94));
    c.cvt_w_s(0, 2);
    c.swc1(0, s(0x98));
    c.swc1(5, s(0x88));
    c.cvt_w_s(0, 3);
    c.swc1(0, s(0x9c));
    c.sw(s(0x80), 0);
    c.swc1(5, s(0x84));
    // Vertex 1 (sp+0xb0): centre + offset.
    vadd(c, s(0x20), s(0x60), s(0x210), false);
    c.lwc1(1, s(0x74));
    c.lwc1(2, s(0x78));
    c.lwc1(3, s(0x7c));
    c.mul_s(1, 1, 4);
    c.lwc1(0, s(0x70));
    c.mul_s(2, 2, 4);
    c.mul_s(3, 3, 4);
    c.lq(V0, s(0x210));
    c.mul_s(0, 0, 6);
    c.swc1(5, s(0xb8));
    c.cvt_w_s(7, 1);
    c.swc1(7, s(0xc0));
    c.sq(V0, s(0x200));
    c.cvt_w_s(1, 2);
    c.swc1(1, s(0xc4));
    c.cvt_w_s(1, 3);
    c.swc1(1, s(0xc8));
    c.cvt_w_s(1, 0);
    c.swc1(1, s(0xcc));
    c.sq(V0, s(0xd0));
    c.swc1(5, s(0xb0));
    c.swc1(5, s(0xb4));
    // Vertex 2 (sp+0xe0): ring point 3 - offset, colour fx+0x28.
    vadd(c, s(0x50), s(0x60), s(0x210), true);
    c.lwc1(0, at(fx, 0x2c));
    c.lwc1(2, at(fx, 0x34));
    c.lwc1(3, at(fx, 0x28));
    c.mul_s(0, 0, 4);
    c.lwc1(1, at(fx, 0x30));
    c.mul_s(2, 2, 4);
    c.lq(V0, s(0x210));
    c.mul_s(3, 3, 6);
    c.mul_s(1, 1, 4);
    c.swc1(5, s(0xe8));
    c.sq(V0, s(0x200));
    c.cvt_w_s(7, 0);
    c.swc1(7, s(0xf0));
    c.sq(V0, s(0x100));
    c.cvt_w_s(0, 2);
    c.swc1(0, s(0xf8));
    c.sw(s(0xe0), 0);
    c.cvt_w_s(0, 3);
    c.swc1(0, s(0xfc));
    c.sw(s(0xe4), 0);
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0xf4));
    // Vertex 3 (sp+0x110): ring point 3 + offset.
    vadd(c, s(0x50), s(0x60), s(0x210), false);
    c.lwc1(1, at(fx, 0x2c));
    c.lwc1(2, at(fx, 0x30));
    c.lwc1(3, at(fx, 0x34));
    c.mul_s(1, 1, 4);
    c.lwc1(0, at(fx, 0x28));
    c.mul_s(2, 2, 4);
    c.mul_s(3, 3, 4);
    c.lq(V0, s(0x210));
    c.mul_s(0, 0, 6);
    c.cvt_w_s(7, 1);
    c.swc1(7, s(0x120));
    c.sq(V0, s(0x200));
    c.cvt_w_s(1, 2);
    c.swc1(1, s(0x124));
    c.cvt_w_s(1, 3);
    c.swc1(1, s(0x128));
    c.cvt_w_s(1, 0);
    c.swc1(1, s(0x12c));
    c.sq(V0, s(0x130));
    c.swc1(5, s(0x118));
    c.swc1(5, s(0x110));
    c.sw(s(0x114), 0);
    // Vertex 4 (sp+0x140): ring point 2 - offset.
    vadd(c, s(0x40), s(0x60), s(0x210), true);
    c.lwc1(0, at(fx, 0x2c));
    c.lwc1(1, at(fx, 0x30));
    c.lwc1(2, at(fx, 0x34));
    c.mul_s(0, 0, 4);
    c.lwc1(3, at(fx, 0x28));
    c.mul_s(1, 1, 4);
    c.lq(V0, s(0x210));
    c.mul_s(2, 2, 4);
    c.mul_s(3, 3, 6);
    c.swc1(5, s(0x148));
    c.sq(V0, s(0x200));
    c.cvt_w_s(7, 0);
    c.swc1(7, s(0x150));
    c.sq(V0, s(0x160));
    c.cvt_w_s(0, 3);
    c.swc1(0, s(0x15c));
    c.sw(s(0x140), 0);
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x154));
    c.swc1(5, s(0x144));
    c.cvt_w_s(0, 2);
    c.swc1(0, s(0x158));
    // Vertex 5 (sp+0x170): ring point 2 + offset.
    vadd(c, s(0x40), s(0x60), s(0x210), false);
    c.lwc1(0, at(fx, 0x2c));
    c.lwc1(1, at(fx, 0x30));
    c.lwc1(2, at(fx, 0x34));
    c.mul_s(0, 0, 4);
    c.mul_s(1, 1, 4);
    c.lq(V0, s(0x210));
    c.mul_s(2, 2, 4);
    c.cvt_w_s(3, 0);
    c.swc1(3, s(0x180));
    c.sq(V0, s(0x200));
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x184));
    c.cvt_w_s(0, 2);
    c.swc1(0, s(0x188));
    c.lwc1(0, at(fx, 0x28));
    c.sq(V0, s(0x190));
    c.mul_s(0, 0, 6);
    c.swc1(5, s(0x178));
    c.swc1(5, s(0x170));
    c.swc1(5, s(0x174));
    c.cvt_w_s(1, 0);
    c.swc1(1, s(0x18c));
    // Vertex 6 (sp+0x1a0): ring point 1 - offset, colour sp+0x70.
    vadd(c, s(0x30), s(0x60), s(0x210), true);
    c.lwc1(0, s(0x74));
    c.lwc1(1, s(0x78));
    c.lwc1(2, s(0x7c));
    c.mul_s(0, 0, 4);
    c.lwc1(3, s(0x70));
    c.mul_s(1, 1, 4);
    c.lq(V0, s(0x210));
    c.mul_s(2, 2, 4);
    c.mul_s(3, 3, 6);
    c.swc1(5, s(0x1a8));
    c.cvt_w_s(7, 0);
    c.swc1(7, s(0x1b0));
    c.sq(V0, s(0x200));
    c.sq(V0, s(0x1c0));
    c.cvt_w_s(0, 2);
    c.swc1(0, s(0x1b8));
    c.sw(s(0x1a0), 0);
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x1b4));
    c.sw(s(0x1a4), 0);
    c.cvt_w_s(0, 3);
    c.swc1(0, s(0x1bc));
    // Vertex 7 (sp+0x1d0): ring point 1 + offset.
    vadd(c, s(0x30), s(0x60), s(0x210), false);
    c.lwc1(0, s(0x74));
    c.lwc1(1, s(0x78));
    c.mul_s(0, 0, 4);
    c.lq(V0, s(0x210));
    c.mul_s(1, 1, 4);
    c.sq(V0, s(0x200));
    c.cvt_w_s(2, 0);
    c.swc1(2, s(0x1e0));
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x1e4));
    c.lwc1(0, s(0x7c));
    c.lwc1(1, s(0x70));
    c.mul_s(0, 0, 4);
    c.sq(V0, s(0x1f0));
    c.mul_s(1, 1, 6);
    c.swc1(5, s(0x1d0));
    c.swc1(5, s(0x1d8));
    c.cvt_w_s(2, 0);
    c.swc1(2, s(0x1e8));
    c.sw(s(0x1d4), 0);
    c.cvt_w_s(0, 1);
    c.swc1(0, s(0x1ec));
    // Draw the strip (a1 = 8, a2 = sp+0x80, a3 = 0), pop.
    c.g[T0][0] = gfx(c);
    draw(c, T0, V0, V1, 0x002e_4200)?;
    pop_record(c, A0);
    Ok(())
}


pub(crate) const BOOST: Streamer = Streamer { this: S2, points: boost::POINTS, count: boost::COUNT, tmp: 0xfc0, half: 0x7e0 };


/// `*(gp + GRAPHICS_MAN)`: the PS2 graphics manager (`BeginTriStrip`).
pub(crate) const GRAPHICS_MAN: u32 = 0x2a90;

pub(crate) const BEGIN_TRI_STRIP: u32 = 0x0037_a260;

pub(crate) const END_TRI_STRIP: u32 = 0x0037_a430;


/// `void cSpeedBoostStreamerFX_Render(cSpeedBoostStreamerFX *fx)`
/// (0x2e7a10): when the effect has points, is at least 3.0 old,
/// its alpha is above the global minimum (gp-0x39d8) and boosts are shown
/// (`gp+0x1634`), push a GS state (texture `gfx+0xf50+4*fx->texture`),
/// build the two streamer strips (point pairs offset along the rider's
/// bone, widened along its direction, faded by `1 - (i/count)²`; drawn when
/// they have at least 4 vertices) and, with at least two flame segments,
/// one flame strip written straight into a `BeginTriStrip` packet (ring
/// entries `(head + i) % 0x1e`, alpha clamped to [0, 1]); pop the state.
///
/// Frame 0x1090: s0 +0x1080, s1 +0x1070, s2 +0x1060, s3 +0x1050, ra
/// +0x1040.
pub fn c_speed_boost_streamer_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 10;
        let frame = Frame::open(c, 0x1090);
        frame.save(c, S2, 0x1060);
        frame.save(c, S0, 0x1080);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x1070);
        frame.save(c, S3, 0x1050);
        frame.save_ra(c, 0x1040);
        let fx = c.g[S2][0];
        c.g[V0][0] = c.lw(at(fx, boost::COUNT));
        let empty = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x1080);
        // How far the epilogue's restores start: 0 = s1, s2, s3; 1 = s2,
        // s3; 2 = s3.
        let restored = if empty {
            0
        } else {
            c.cycles += 7;
            c.lwc1(1, at(fx, boost::FADE));
            c.g[AT][0] = 0x4040_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(1, 0);
            let fading = c.fcc();
            frame.restore(c, S1, 0x1070);
            if fading {
                1
            } else {
                c.cycles += 6;
                c.lwc1(1, at(fx, boost::ALPHA));
                c.lwc1(0, gp.wrapping_sub(0x39d8));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    frame.restore(c, S2, 0x1060);
                    2
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(gp.wrapping_add(0x1634));
                    let hidden = c.g[V0][0] == 0;
                    c.g[V1][0] = gfx(c);
                    if hidden {
                        1
                    } else {
                        boost_draw(c, frame)?;
                        0
                    }
                }
            }
        };
        if restored == 0 {
            c.cycles += 1;
            frame.restore(c, S1, 0x1070);
        }
        if restored <= 1 {
            c.cycles += 1;
            frame.restore(c, S2, 0x1060);
        }
        c.cycles += 4;
        frame.restore(c, S3, 0x1050);
        frame.restore_ra(c, 0x1040);
        frame.close(c);
        Ok(())
    })
}


pub(crate) fn boost_draw(c: &mut Cpu, frame: Frame) -> Flow {
    let sp = r32(c, SP);
    let gp = r32(c, GP);
    c.cycles += 96;
    // Push the GS state.
    let g = c.g[V1][0];
    c.g[A1][0] = 0x50_0000;
    c.g[T5][0] = 0xffff_ffff_ffff_fff3;
    c.g[A3][0] = 0xffff_ffff_ffbf_ffff;
    let top = c.lw(at(g, GS_TOP));
    c.g[V0][0] = top;
    c.g[T6][0] = 0x40_0000;
    c.g[T1][0] = 0xffff_ffff_fe7f_ffff;
    copy_record(c, top as u32, at(top, RECORD), A0, A2, T0);
    c.g[T2][0] = 0xffff_ffff_ffcf_ffff;
    c.g[T3][0] = 0xffff_ffff_fff0_0fff;
    let top = c.lw(at(g, GS_TOP));
    c.g[A0][0] = gfx(c);
    c.g[V0][0] = sx(at(top, RECORD));
    c.g[T0][0] = 0x1_4000;
    let new_top = c.g[V0][0];
    c.sw(at(g, GS_TOP), new_top);
    c.g[T7][0] = 0xffff_ffff_ffff_fffc;
    c.g[T4][0] = 0xffff_ffff_e000_03ff;
    let top = c.lw(at(c.g[A0][0], GS_TOP));
    c.g[V0][0] = top;
    c.g[S0][0] = 0xffff_ffff_ffff_fc1f;
    c.g[S1][0] = 0xffff_ffff_ffff_ff83;
    c.g[A2][0] = sx(GS_DEFAULT);
    copy_record(c, GS_DEFAULT, top as u32, S3, V1, A0);
    c.g[T8][0] = 0x29;
    c.g[T9][0] = u64::MAX;
    // Texture `gfx + 0xf50 + 4 * fx->texture`.
    let index = c.lw(at(c.g[S2][0], boost::TEXTURE));
    let g = gfx(c);
    c.g[A0][0] = g;
    c.g[V0][0] = sx((g as u32).wrapping_add((index as u32) << 2));
    let top = c.lw(at(g, GS_TOP));
    c.g[A1][0] = top;
    let tex = c.lw(at(c.g[V0][0], 0xf50));
    c.g[V1][0] = tex;
    c.sh(at(top, 0x10), tex);
    let g = gfx(c);
    c.g[A2][0] = g;
    gs_field(c, g, V1, 0, 0xffff_ffff_ffff_fff3, 0);
    gs_field(c, g, A0, 4, 0xffff_ffff_ffbf_ffff, 0x40_0000);
    gs_field(c, g, V1, 4, 0xffff_ffff_fe7f_ffff, 0);
    gs_field(c, g, A0, 4, 0xffff_ffff_ffcf_ffff, 0);
    gs_field(c, g, V1, 4, 0xffff_ffff_fff0_0fff, 0x1_4000);
    gs_field(c, g, A0, 4, 0xffff_ffff_ffff_fffc, 2);
    gs_field(c, g, V1, 8, 0xffff_ffff_e000_03ff, 0);
    gs_field(c, g, A0, 8, 0xffff_ffff_ffff_fc1f, 0xe0);
    gs_field(c, g, V1, 4, 0xffff_ffff_ffff_ff83, 0x1c);
    // gcc's empty constructor loops over the 2 x 42 vertices.
    loop {
        c.cycles += 7;
        addiu(c, T8, T8, u32::MAX);
        let more = c.g[T8][0] != c.g[T9][0];
        c.g[V0][0] = 0x29;
        if !more {
            break;
        }
        back_edge(c, 0x002e_7bf8);
    }
    c.cycles += 3;
    c.g[A0][0] = c.lw(at(c.g[S2][0], boost::COUNT));
    c.g[A2][0] = c.lw(at(c.g[S2][0], boost::RIDER));
    c.g[V1][0] = u64::MAX;
    loop {
        c.cycles += 7;
        addiu(c, V0, V0, u32::MAX);
        let more = c.g[V0][0] != c.g[V1][0];
        c.g[A3][0] = 0;
        if !more {
            break;
        }
        back_edge(c, 0x002e_7c20);
    }
    // Colour 1.0, the offset along the rider's bone, 1/(count-1)² and
    // 1/count; the half width along the rider's direction.
    c.cycles += 49;
    let t = sp.wrapping_add(BOOST.tmp);
    let rider = c.g[A2][0];
    c.g[A1][0] = c.lw(at(rider, 0x780));
    addiu(c, A0, A0, u32::MAX);
    c.g[V0][0] = c.lw(at(rider, 0x89c));
    c.mult(0, A0, A0, A0);
    c.g[V1][0] = c.lw(at(c.g[A1][0], 0x30));
    c.g[V0][0] = sx(r32(c, V0) << 6);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.lwc1(1, gp.wrapping_sub(0x39d4));
    let bone = r32(c, V1);
    c.lq(V0, bone.wrapping_add(0x30));
    c.mtc1(A0, 0);
    c.cvt_s_w(0, 0);
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 2);
    c.g[S0][0] = 0;
    c.sq(V0, t.wrapping_add(0x10));
    vmul_bc(c, V1, 1, t.wrapping_add(0x10), t.wrapping_add(0x20));
    c.div_s(9, 3, 0);
    c.lq(V0, t.wrapping_add(0x20));
    c.lwc1(0, at(c.g[S2][0], boost::COUNT));
    c.cvt_s_w(0, 0);
    c.swc1(3, t.wrapping_add(0x10));
    c.sq(V0, t);
    c.swc1(3, t.wrapping_add(0x14));
    c.div_s(8, 3, 0);
    c.swc1(3, t.wrapping_add(0x18));
    c.swc1(3, t.wrapping_add(0x1c));
    c.mtc1(ZERO, 7);
    let rider = c.lw(at(c.g[S2][0], boost::RIDER));
    c.g[V1][0] = rider;
    c.lwc1(0, at(c.g[S2][0], boost::WIDTH));
    c.lq(V0, at(rider, 0x1a0));
    c.lwc1(4, at(c.g[S2][0], boost::V_START));
    c.mul_s(0, 0, 2);
    c.sq(V0, t.wrapping_add(0x40));
    vmul_bc(c, V1, 0, t.wrapping_add(0x40), t.wrapping_add(0x60));
    c.lq(V0, t.wrapping_add(0x60));
    c.g[V1][0] = c.lw(at(c.g[S2][0], boost::COUNT));
    c.sq(V0, t.wrapping_add(0x50));
    let none = (c.g[V1][0] as i64) <= 0;
    c.g[A2][0] = c.lw(at(c.g[S2][0], boost::FIRST));
    if !none {
        c.cycles += 6;
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(AT, 2);
        c.g[T0][0] = 0x13;
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(AT, 6);
        c.g[A1][0] = sx(sp.wrapping_add(BOOST.half));
        loop {
            // alpha = fx->alpha * (1 - f7² / (count-1)²), clamped at 0.
            c.cycles += 13;
            c.mul_s(0, 7, 7);
            c.lwc1(1, at(c.g[S2][0], boost::ALPHA));
            c.mtc1(ZERO, 5);
            c.g[V0][0] = (-1i64 < (c.g[A2][0] as i64)) as u64;
            c.mul_s(0, 0, 9);
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.g[T0][0];
            }
            c.sub_s(0, 3, 0);
            c.mul_s(1, 1, 0);
            c.c_lt_s(1, 5);
            let clamp = c.fcc();
            c.swc1(1, t.wrapping_add(0x10));
            if clamp {
                c.cycles += 1;
                c.swc1(5, t.wrapping_add(0x10));
            }
            if !streamer_segment(c, &BOOST) {
                break;
            }
            back_edge(c, 0x002e_7d18);
        }
    }
    // The two streamer strips, when they have at least 4 vertices.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S0][0] as i64) < 4) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 19;
        c.g[T0][0] = gfx(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0;
        draw(c, T0, V0, V1, 0x002e_7fa4)?;
        c.g[T0][0] = gfx(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx(sp.wrapping_add(BOOST.half));
        c.g[A3][0] = 0;
        draw(c, T0, V0, V1, 0x002e_7fc8)?;
    }
    c.g[A1][0] = c.lw(at(c.g[S2][0], boost::FLAMES));
    // The flame strip.
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
    let few = c.g[V0][0] != 0;
    c.g[V1][0] = gfx(c);
    if !few {
        c.cycles += 12;
        let gm = c.lw(gp.wrapping_add(GRAPHICS_MAN));
        c.g[S0][0] = gm;
        c.g[A1][0] = sx(r32(c, A1) << 1);
        c.g[A0][0] = gm;
        call_ptr(c, BEGIN_TRI_STRIP, 0x002e_7fe8)?;
        boost_flames(c, sp);
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, END_TRI_STRIP, 0x002e_81bc)?;
        c.g[V1][0] = gfx(c);
    }
    c.cycles += 4;
    let g = c.g[V1][0];
    let top = c.lw(at(g, GS_TOP));
    c.g[V0][0] = sx((top as u32).wrapping_sub(RECORD));
    let v0 = c.g[V0][0];
    c.sw(at(g, GS_TOP), v0);
    frame.restore(c, S0, 0x1080);
    Ok(())
}


/// The flame strip's vertices, two per ring entry, written from the packet
/// pointer `BeginTriStrip` returned (v0): colour 255 with alpha
/// `clamp(fx->alpha * (1 - (f8 / min(fade, clamp))), 0, 1) * 128`, v from
/// `fx+0x10` plus the entry's v offset, positions from the entry's two
/// points (+4, +0x10).
pub(crate) fn boost_flames(c: &mut Cpu, sp: u32) {
    let fx = c.g[S2][0];
    c.g[V1][0] = c.g[V0][0];
    c.lwc1(0, at(fx, boost::FADE));
    c.lwc1(1, at(fx, boost::FLAME_CLAMP));
    c.g[A1][0] = 0;
    c.g[V0][0] = c.lw(at(fx, boost::FLAMES));
    c.mtc1(ZERO, 8);
    let none = (c.g[V0][0] as i64) <= 0;
    c.min_s(10, 0, 1);
    if none {
        return;
    }
    c.cycles += 14;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 4);
    c.g[A0][0] = sx(r32(c, V1).wrapping_sub(0x30));
    c.g[AT][0] = 0x437f_0000;
    c.mtc1(AT, 6);
    c.g[T0][0] = 0x1c;
    c.g[AT][0] = 0x4300_0000;
    c.mtc1(AT, 9);
    c.mov_s(5, 8);
    c.g[A3][0] = 0x1e;
    c.g[T2][0] = 0;
    c.g[A2][0] = sx(at(fx, boost::FLAME_RING));
    c.g[T1][0] = 0xff;
    c.lwc1(0, r32(c, A2));
    loop {
        c.cycles += 11;
        c.lwc1(1, at(c.g[S2][0], boost::V_START));
        c.add_s(8, 8, 0);
        c.lwc1(2, at(c.g[S2][0], boost::ALPHA));
        c.div_s(0, 8, 10);
        c.add_s(7, 1, 0);
        c.sub_s(0, 4, 0);
        c.mul_s(2, 2, 0);
        c.c_le_s(5, 2);
        let above = c.fcc();
        c.mov_s(3, 5);
        if above {
            c.cycles += 1;
            c.min_s(3, 2, 4);
        }
        c.cycles += 4;
        c.c_le_s(5, 3);
        let above = c.fcc();
        c.mov_s(0, 5);
        if above {
            c.cycles += 1;
            c.min_s(0, 3, 4);
        }
        // Ring entry (head + i) % 0x1e.
        c.cycles += 12;
        c.g[V0][0] = c.lw(at(c.g[S2][0], boost::FLAME_HEAD));
        addiu(c, A0, A0, 0x30);
        c.swc1(0, sp);
        addiu(c, A2, A2, 0x1c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
        c.swc1(4, sp.wrapping_add(4));
        c.div(0, V0, A3);
        c.swc1(4, sp.wrapping_add(8));
        c.swc1(4, sp.wrapping_add(0xc));
        addiu(c, A1, A1, 1);
        c.cycles += 64;
        c.g[V1][0] = c.hi[0];
        c.mult(0, T3, V1, T0);
        c.g[V0][0] = sx(r32(c, T3).wrapping_add(r32(c, S2)));
        c.g[V1][0] = c.g[V0][0];
        addiu(c, V0, V0, 0x54);
        addiu(c, V1, V1, 0x60);
        let (pa, pb) = (r32(c, V0), r32(c, V1));
        // First vertex: point +0x54.
        c.lwc1(0, pa);
        c.swc1(0, sp.wrapping_add(0x10));
        c.lwc1(1, pa.wrapping_add(4));
        c.swc1(1, sp.wrapping_add(0x14));
        c.lwc1(0, pa.wrapping_add(8));
        c.swc1(4, sp.wrapping_add(0x1c));
        c.swc1(0, sp.wrapping_add(0x18));
        let v = r32(c, A0);
        let t1 = c.g[T1][0];
        c.sw(v.wrapping_add(0x10), t1);
        c.lwc1(0, sp.wrapping_add(8));
        c.mul_s(0, 0, 6);
        c.cvt_w_s(1, 0);
        c.swc1(1, v.wrapping_add(0x14));
        c.lwc1(1, sp.wrapping_add(0xc));
        c.mul_s(1, 1, 6);
        c.cvt_w_s(0, 1);
        c.swc1(0, v.wrapping_add(0x18));
        c.lwc1(0, sp);
        c.swc1(4, v.wrapping_add(8));
        c.mul_s(0, 0, 9);
        c.cvt_w_s(1, 0);
        c.swc1(1, v.wrapping_add(0x1c));
        c.lq(V0, sp.wrapping_add(0x10));
        c.swc1(5, v);
        c.sq(V0, v.wrapping_add(0x20));
        c.swc1(7, v.wrapping_add(4));
        // Second vertex: point +0x60.
        addiu(c, A0, A0, 0x30);
        let v = r32(c, A0);
        c.lwc1(0, pb);
        c.lwc1(2, sp.wrapping_add(4));
        c.swc1(0, sp.wrapping_add(0x10));
        c.mul_s(2, 2, 6);
        c.lwc1(0, pb.wrapping_add(4));
        c.swc1(0, sp.wrapping_add(0x14));
        c.lwc1(1, pb.wrapping_add(8));
        c.swc1(4, sp.wrapping_add(0x1c));
        c.swc1(1, sp.wrapping_add(0x18));
        c.cvt_w_s(0, 2);
        c.swc1(0, v.wrapping_add(0x10));
        c.lwc1(0, sp.wrapping_add(8));
        c.mul_s(0, 0, 6);
        c.cvt_w_s(1, 0);
        c.swc1(1, v.wrapping_add(0x14));
        c.lwc1(1, sp.wrapping_add(0xc));
        c.mul_s(1, 1, 6);
        c.cvt_w_s(0, 1);
        c.swc1(0, v.wrapping_add(0x18));
        c.lwc1(0, sp);
        c.mul_s(0, 0, 9);
        c.cvt_w_s(1, 0);
        c.swc1(1, v.wrapping_add(0x1c));
        c.swc1(4, v.wrapping_add(8));
        c.lq(V0, sp.wrapping_add(0x10));
        c.swc1(7, v.wrapping_add(4));
        c.sq(V0, v.wrapping_add(0x20));
        c.swc1(4, v);
        let n = c.lw(at(c.g[S2][0], boost::FLAMES));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (n as i64)) as u64;
        if c.g[V0][0] == 0 {
            return;
        }
        c.lwc1(0, r32(c, A2));
        back_edge(c, 0x002e_8040);
    }
}


// ---------------------------------------------------------------------------
// Small helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}


/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}


/// `lwc1 f, a`, returning the value.
#[inline(always)]
pub(crate) fn ldf(c: &mut Cpu, f: usize, a: u32) -> FpuF32 {
    c.lwc1(f, a);
    FpuF32(c.f[f])
}


/// FPU register `f`.
#[inline(always)]
pub(crate) fn fr(c: &Cpu, f: usize) -> FpuF32 {
    FpuF32(c.f[f])
}


/// Set FPU register `f`.
#[inline(always)]
pub(crate) fn setf(c: &mut Cpu, f: usize, v: FpuF32) {
    c.f[f] = v.0;
}


/// `lui at, hi; mtc1 at, f`: a float constant through `at`.
#[inline(always)]
pub(crate) fn fconst(c: &mut Cpu, f: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = sx(bits);
    c.f[f] = bits;
    FpuF32(bits)
}


/// `x + y` of two words (`addu`).
#[inline(always)]
pub(crate) fn add32(x: u64, y: u64) -> u64 {
    sx((x as u32).wrapping_add(y as u32))
}


/// `lq gpr, from; sq gpr, to` (a quadword moved through a GPR).
#[inline(always)]
pub(crate) fn copy_q(c: &mut Cpu, gpr: usize, from: u32, to: u32) {
    c.lq(gpr, from);
    c.sq(gpr, to);
}


/// `lqc2 vf3, a; lqc2 vf4, b; vadd_2.xyzw vf5, vf3, vf4; sqc2 vf5, out`.
#[inline(always)]
pub(crate) fn vadd_2(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}


/// `lqc2 vf3, a; lqc2 vf4, b; vsub.xyzw vf5, vf3, vf4; sqc2 vf5, out`.
#[inline(always)]
pub(crate) fn vsub(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}


/// `lqc2 vf4, v; qmtc2 rt, vf3; vmulx.xyzw vf5, vf4, vf3x; sqc2 vf5, out`:
/// a vector scaled by the float bits held in GPR `rt`.
#[inline(always)]
pub(crate) fn vscale(c: &mut Cpu, v: u32, rt: usize, out: u32) {
    c.lqc2(4, v);
    c.qmtc2(rt, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}


/// The common epilogue tail of a leaf: `addiu sp, sp, size; jr ra`.
#[inline(always)]
pub(crate) fn leave(c: &mut Cpu, size: u32) {
    let ra = r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(size));
    c.pc = ra;
}


/// The `fmodf`-style remainder the original writes: `q = v / s` truncated
/// toward zero, result `(q - trunc(q)) * s`.
#[inline(always)]
pub(crate) fn frac_times(c: &mut Cpu, v: FpuF32, s: FpuF32) -> FpuF32 {
    let q = v.div(s);
    setf(c, 0, q);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    c.mtc1(2, 1);
    c.cvt_s_w(1, 1);
    let trunc = fr(c, 1);
    let f = q - trunc;
    setf(c, 0, f);
    let r = f * s;
    setf(c, 0, r);
    r
}


/// `void SnowBox_WrapOffset(SnowBox* box, const Vec4* d)` (0x2e5430):
/// `box->pos -= *d`, then wrap each component into `(-size/2, size/2]` by the
/// fractional part of the division (and store `w = 1`).
///
/// The three components are wrapped by the same inlined code; the original
/// lays them out as three sibling blocks, each entering the next one.
pub fn snow_box_wrap_offset(c: &mut Cpu) {
    let this = abi::arg(c, 0);
    let d = abi::arg(c, 1);
    c.cycles += 17;
    c.lqc2(3, p(this, snow_box::POS));
    c.lqc2(4, d);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(this, snow_box::POS));
    fconst(c, 0, 0x3f00_0000); // f0 = 0.5
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    ldf(c, 2, p(this, snow_box::SIZE));
    ldf(c, 3, p(this, snow_box::POS));
    c.mul_s(0, 2, 0); // f0 = h = size * 0.5
    ldf(c, 6, p(this, snow_box::POS + 4));
    ldf(c, 7, p(this, snow_box::POS + 8));
    c.mov_s(4, 6); // f4 = original y
    c.c_lt_s(0, 3); // h < x ?
    let x_above = c.fcc();
    c.mov_s(5, 7); // f5 = original z
    // (x, y, z results are left in f3, f6, f7 as the original does.)
    'x: {
        'x_tail: {
            'x_inner: {
                if x_above {
                    c.cycles += 9;
                    let v = fr(c, 3);
                    let s = fr(c, 2);
                    let r = frac_times(c, v, s);
                    setf(c, 3, r - s);
                    break 'x_tail;
                }
            }
            c.cycles += 5;
            c.neg_s(0, 0); // f0 = -h
            c.c_lt_s(3, 0); // x < -h ?
            if !c.fcc() {
                c.lwc1(0, p(this, snow_box::SIZE));
                break 'x;
            }
            c.cycles += 8;
            let v = fr(c, 3);
            let s = fr(c, 2);
            let r = frac_times(c, v, s);
            setf(c, 3, r + s);
            break 'x_tail;
        }
        c.cycles += 1;
        c.lwc1(0, p(this, snow_box::SIZE));
    }
    'y: {
        'y_inner: {
            c.cycles += 8;
            fconst(c, 1, 0x3f00_0000);
            c.mov_s(2, 0); // f2 = size
            c.mul_s(0, 0, 1); // f0 = h
            c.c_lt_s(0, 4); // h < y ?
            if !c.fcc() {
                c.neg_s(0, 0);
                break 'y_inner;
            }
            c.cycles += 9;
            let v = fr(c, 4);
            let s = fr(c, 2);
            let r = frac_times(c, v, s);
            setf(c, 6, r - s);
            break 'y;
        }
        c.cycles += 3;
        c.c_lt_s(4, 0); // y < -h ?
        let t = !c.fcc();
        if t {
            break 'y;
        }
        c.cycles += 8;
        let v = fr(c, 4);
        let s = fr(c, 2);
        let r = frac_times(c, v, s);
        setf(c, 6, r + s);
        break 'y;
    }
    'z: {
        'z_inner: {
            c.cycles += 7;
            fconst(c, 0, 0x3f00_0000);
            c.mul_s(0, 2, 0); // f0 = h
            c.c_lt_s(0, 5); // h < z ?
            if !c.fcc() {
                c.neg_s(0, 0);
                break 'z_inner;
            }
            c.cycles += 9;
            let v = fr(c, 5);
            let s = fr(c, 2);
            let r = frac_times(c, v, s);
            setf(c, 7, r - s);
            break 'z;
        }
        c.cycles += 3;
        c.c_lt_s(5, 0); // z < -h ?
        let t = !c.fcc();
        if t {
            break 'z;
        }
        c.cycles += 8;
        let v = fr(c, 5);
        let s = fr(c, 2);
        let r = frac_times(c, v, s);
        setf(c, 7, r + s);
        break 'z;
    }
    c.cycles += 10;
    fconst(c, 0, 0x3f80_0000); // w = 1
    let sp = sp(c);
    c.swc1(3, sp);
    c.swc1(6, p(sp, 4));
    c.swc1(7, p(sp, 8));
    c.swc1(0, p(sp, 0xc));
    copy_q(c, 2, sp, p(this, snow_box::POS));
    leave(c, 0x10);
}


// ---------------------------------------------------------------------------
// Spray FX random velocity.

/// `void cDynamicSprayFX_AddRandomVelocity(SprayFX* fx, Vec4* out)`
/// (0x2de398): add two random vectors to `*out` — one along `fx->dir0`
/// (+0x50) scaled by `phase * 55 + 55 * rand +-`, one along `fx->dir1`
/// (+0x40) with `rand * 20 - 10`. The random source is the LCG word at
/// gp+0xa0c (`state = state * 0x18fcd + 0xe9507c`, mantissa bits give a
/// float in [1, 2)).
pub fn dynamic_spray_fx_add_random_velocity(c: &mut Cpu) {
    let this = abi::arg(c, 0);
    let out = abi::arg(c, 1);
    let state_at = gp(c, 0xa0c);
    c.cycles += 68;
    c.g[V0][0] = c.lw(state_at);
    c.g[A3][0] = 0x0001_8fcd;
    ldf(c, 3, p(this, 0x12c)); // phase (-1..1)
    fconst(c, 4, 0x3f80_0000);
    c.mult(0, 2, 2, 7); // state *= 0x18fcd
    c.abs_s(1, 3);
    c.g[A2][0] = 0x00e9_507c;
    c.sub_s(1, 4, 1); // 1 - |phase|
    fconst(c, 0, 0x425c_0000); // 55
    c.g[V1][0] = 0x007f_ffff;
    c.g[V0][0] = add32(c.g[V0][0], c.g[A2][0]);
    c.g[T0][0] = 0x3f80_0000;
    c.mul_s(1, 1, 0); // a = (1 - |phase|) * 55
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.mul_s(3, 3, 0); // phase * 55
    c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    c.sw(state_at, c.g[V0][0]);
    c.neg_s(2, 1); // lo = -a
    fconst(c, 6, 0x41a0_0000); // 20
    c.lwc1(0, state_at); // f0 in [1, 2)
    c.sub_s(1, 1, 2); // a - lo
    fconst(c, 5, 0xc120_0000); // -10
    c.sub_s(0, 0, 4); // rand - 1
    c.mul_s(1, 1, 0);
    c.add_s(2, 2, 1); // lo + (a - lo) * rand
    c.add_s(3, 3, 2); // phase * 55 + ...
    c.mfc1(9, 3);
    c.lqc2(4, p(this, 0x50));
    c.qmtc2(9, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x10));
    let stack = sp(c);
    copy_q(c, 2, p(stack, 0x10), stack);
    vadd_2(c, out, stack, out);
    c.g[V0][0] = c.lw(state_at);
    c.mult(0, 9, 2, 7);
    c.g[V0][0] = add32(c.g[T1][0], c.g[A2][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
    c.sw(state_at, c.g[V0][0]);
    c.lwc1(0, state_at);
    c.sub_s(0, 0, 4); // rand - 1
    c.mul_s(0, 0, 6); // * 20
    c.add_s(0, 0, 5); // - 10
    c.mfc1(3, 0);
    c.lqc2(4, p(this, 0x40));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x10));
    let stack = sp(c);
    copy_q(c, 2, p(stack, 0x10), stack);
    vadd_2(c, out, stack, out);
    leave(c, 0x20);
}


/// The renderer's default state record copied by the original (`D_00501420`).
pub(crate) const WAKE_STATE_RESET: u32 = 0x0050_1420;


/// `void cBoardWakeFX_Render(Wake* self)` (0x2ddab8): push a renderer state,
/// copy the default wake state into it, fill in the vertex format, then draw
/// the trail strip (alpha from the age of the newest point) through the
/// renderer's virtual `DrawStrip`.
pub fn board_wake_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(gp(c, 0x1438));
        frame.save(c, S0, 0x30);
        c.g[T8][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let self_at = r32(c, T8);
        frame.save_ra(c, 0);
        'out: {
            if c.g[V0][0] != 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(self_at, wake_dab8::COUNT));
            c.g[V1][0] = c.lw(gp(c, -0x854));
            if c.g[V0][0] == 0 {
                break 'out;
            }
            // PushState: copy the current 20-byte state to the new top and
            // advance it.
            c.cycles += 106;
            c.g[T3][0] = 0x0050_0000;
            c.g[T4][0] = sx(0xffff_fff3); // ~0xc
            c.g[A1][0] = 0xffff_ffff_ffbf_ffff; // clear bit 22
            c.g[V0][0] = c.lw(p(r32(c, V1), gfx::STATE_TOP));
            c.g[T5][0] = 0x0040_0000;
            c.g[A3][0] = 0xffff_ffff_fe7f_ffff; // clear bit 23
            c.ldl(4, p(r32(c, V0), 7));
            c.ldr(4, p(r32(c, V0), 0));
            c.ldl(6, p(r32(c, V0), 0xf));
            c.ldr(6, p(r32(c, V0), 8));
            c.g[T0][0] = c.lw(p(r32(c, V0), 0x10));
            c.sdl(4, p(r32(c, V0), 0x1b));
            c.sdr(4, p(r32(c, V0), 0x14));
            c.sdl(6, p(r32(c, V0), 0x23));
            c.sdr(6, p(r32(c, V0), 0x1c));
            c.sw(p(r32(c, V0), 0x24), c.g[T0][0]);
            c.g[T0][0] = 0xffff_ffff_ffcf_ffff; // clear bits 20, 21
            c.g[T1][0] = 0xffff_ffff_fff0_0fff; // clear bits 12..19
            c.g[V0][0] = c.lw(p(r32(c, V1), gfx::STATE_TOP));
            c.g[A0][0] = c.lw(gp(c, -0x854));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x14));
            c.g[A2][0] = 0x0001_4000; // 0x14 in the f4_12 field
            c.sw(p(r32(c, V1), gfx::STATE_TOP), c.g[V0][0]);
            c.g[T6][0] = sx(0xffff_fffc); // clear bits 0, 1
            c.g[T2][0] = sx(0xe000_03ff); // clear bits 10..28
            c.g[V0][0] = c.lw(p(r32(c, A0), gfx::STATE_TOP));
            c.g[T7][0] = sx(0xffff_fc1f); // clear bits 5..9
            c.g[S0][0] = sx(0xffff_ff83); // clear bits 2..6
            c.g[A0][0] = sx(r32(c, T3).wrapping_add(0x1420));
            c.ldl(17, p(r32(c, A0), 7));
            c.ldr(17, p(r32(c, A0), 0));
            c.ldl(18, p(r32(c, A0), 0xf));
            c.ldr(18, p(r32(c, A0), 8));
            c.g[V1][0] = c.lw(p(r32(c, A0), 0x10));
            c.sdl(17, p(r32(c, V0), 7));
            c.sdr(17, p(r32(c, V0), 0));
            c.sdl(18, p(r32(c, V0), 0xf));
            c.sdr(18, p(r32(c, V0), 8));
            c.sw(p(r32(c, V0), 0x10), c.g[V1][0]);
            c.g[S1][0] = 0x70; // trail record stride
            ldf(c, 2, gp(c, -0x3be8)); // 0.8: the fade threshold
            c.g[V0][0] = c.lw(gp(c, -0x854));
            fconst(c, 12, 0x3fa0_0000); // k = 1.25
            c.g[A0][0] = c.lw(p(r32(c, V0), gfx::STATE_TOP));
            c.g[V1][0] = c.lw(p(r32(c, V0), gfx::F1030));
            fconst(c, 3, 0x3f80_0000); // a = 1.0
            c.sh(p(r32(c, A0), 0x10), c.g[V1][0]);
            // f0_2 = 0 (clear bits 2, 3)
            c.g[T3][0] = c.lw(gp(c, -0x854));
            c.g[V1][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
            c.sw(r32(c, V1), c.g[V0][0]);
            // f4_22 = 1
            c.g[A0][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, A0), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
            c.sw(p(r32(c, A0), 4), c.g[V0][0]);
            // f4_23 = 0
            c.g[V1][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, V1), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
            c.sw(p(r32(c, V1), 4), c.g[V0][0]);
            // f4_20 = 0
            c.g[A0][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, A0), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
            c.sw(p(r32(c, A0), 4), c.g[V0][0]);
            // f4_12 = 0x14
            c.g[V1][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, V1), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
            c.sw(p(r32(c, V1), 4), c.g[V0][0]);
            // f4_0 = 2
            c.g[A0][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, A0), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
            c.g[V0][0] = c.g[V0][0] | 2;
            c.sw(p(r32(c, A0), 4), c.g[V0][0]);
            // f8_10 = 0
            c.g[V1][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, V1), 8));
            c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
            c.sw(p(r32(c, V1), 8), c.g[V0][0]);
            // f8_5 = 7
            c.g[A0][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, A0), 8));
            c.g[V0][0] = c.g[V0][0] & c.g[T7][0];
            c.g[V0][0] = c.g[V0][0] | 0xe0;
            c.sw(p(r32(c, A0), 8), c.g[V0][0]);
            // f4_2 = 5
            c.g[V1][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, V1), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
            c.g[V0][0] = c.g[V0][0] | 0x14;
            c.sw(p(r32(c, V1), 4), c.g[V0][0]);
            // draw = count, over = count + 2 - max, draw -= over when over >= 1
            c.g[T1][0] = c.lw(p(self_at, wake_dab8::COUNT));
            c.g[A3][0] = c.lw(p(self_at, wake_dab8::FIRST));
            c.g[T2][0] = c.g[T1][0];
            c.g[T0][0] = c.lw(p(self_at, wake_dab8::MAX));
            c.g[V0][0] = add32(c.g[A3][0], c.g[T2][0]); // first + n
            c.g[V1][0] = sx(r32(c, T1).wrapping_add(2)); // over = n + 2
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1)); // first + n - 1
            // last = (max + n - 1) % max; draw = n - over (if over >= 1)
            c.cycles += 16;
            c.div(0, 2, 8);
            c.g[A2][0] = c.lw(p(self_at, wake_dab8::VERTS));
            ldf(c, 1, p(self_at, wake_dab8::LIFE));
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, T0)));
            c.g[A1][0] = ((c.g[V1][0] as i64) < 1) as u64;
            c.g[V1][0] = sx(r32(c, T1).wrapping_sub(r32(c, V1)));
            c.g[A0][0] = c.hi[0];
            c.mult(0, 4, 4, 17);
            c.g[A0][0] = add32(c.g[A0][0], c.g[A2][0]);
            ldf(c, 0, p(r32(c, A0), wake_dab8::VERTEX_AGE));
            c.sub_s(0, 1, 0); // life - age
            c.div_s(0, 0, 1); // t
            c.c_le_s(0, 2); // t <= 0.8
            let faded = !c.fcc();
            if c.g[A1][0] == 0 {
                c.g[T1][0] = c.g[V1][0];
            }
            if faded {
                // a stays 1.0 (f3)
            } else {
                c.cycles += 1;
                c.mul_s(3, 0, 12); // a = t * 1.25
            }
            // da = (a - k) / (float)count
            c.cycles += 7;
            c.sub_s(0, 3, 12);
            c.g[V0][0] = ((c.g[T1][0] as i64) < 2) as u64;
            c.mtc1(10, 1); // count (t2)
            c.cvt_s_w(1, 1);
            c.div_s(13, 0, 1);
            let few = c.g[V0][0] != 0;
            c.g[V1][0] = c.lw(gp(c, -0x854));
            if !few {
                c.cycles += 8;
                c.g[V0][0] = c.lw(p(r32(c, T3), gfx::VTABLE));
                c.g[A1][0] = sx(r32(c, T8).wrapping_add(wake_dab8::STRIP));
                c.g[A2][0] = c.lw(p(self_at, wake_dab8::TEX));
                c.g[A0][0] = c.lh(p(r32(c, V0), 0x2d0));
                c.g[V1][0] = c.lw(p(r32(c, V0), 0x2d4));
                let target = r32(c, V1);
                c.g[A0][0] = sx(r32(c, T3).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x002d_dd08)?;
                c.g[V1][0] = c.lw(gp(c, -0x854));
            }
            // stateTop--
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(r32(c, V1), gfx::STATE_TOP));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            c.sw(p(r32(c, V1), gfx::STATE_TOP), c.g[V0][0]);
            break 'out;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cDynamicSprayFX_UpdateEmitter0(SprayFX* fx)` (0x2e0ee8): when the
/// spray is enabled, fast enough and the rider is not in some special mode,
/// spawn a particle 15 units along the spray direction, with the rider's
/// trail point and up-vector velocity; otherwise just advance the emitter at
/// the spray position.
pub fn dynamic_spray_fx_update_emitter0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x80);
        let self_at = r32(c, S0);
        let sp0 = sp(c);
        c.g[V0][0] = c.lw(self_at);
        c.g[A0][0] = c.lw(p(self_at, spray::EMITTER));
        c.g[V1][0] = c.lw(p(r32(c, V0), 0x88c));
        c.g[S1][0] = c.lw(p(self_at, spray::EMITTER_PTR));
        let no_emitter = c.g[A0][0] == 0;
        c.g[A1][0] = c.lw(p(r32(c, V1), 0xa0));
        'out: {
            'on: {
                if no_emitter {
                    break 'on;
                }
                c.cycles += 5;
                c.lwc1(1, gp(c, -0x3afc)); // speed limit
                c.lwc1(0, p(self_at, spray::SPEED));
                c.c_lt_s(1, 0); // limit < speed ?
                let slow = !c.fcc();
                if slow {
                    break 'on;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw(p(self_at, spray::CFG));
                c.g[V1][0] = c.lw(p(r32(c, V0), 0x58));
                if c.g[V1][0] == 0 {
                    break 'on;
                }
                c.cycles += 2;
                let special = c.g[A1][0] != 0;
                c.g[A1][0] = sx(sp0);
                if special {
                    break 'on;
                }
                // pos = fx->pos + fx->dir * 15; then randomise the velocity.
                c.cycles += 82;
                fconst(c, 0, 0x4170_0000); // 15
                c.mfc1(2, 0);
                c.lqc2(4, p(self_at, spray::DIR));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x20));
                c.g[A0][0] = c.g[S0][0];
                c.lq(2, p(sp0, 0x20));
                c.sq(2, p(sp0, 0x10));
                c.lqc2(3, p(self_at, spray::POS));
                c.lqc2(4, p(sp0, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp0, 0x20));
                c.lq(2, p(sp0, 0x20));
                c.sq(2, sp0);
                callee::ADD_RANDOM_VELOCITY.call(c, 0x002e_0f84)?;
                // vt[2] of fx->obj + 0x6c0: the trail point the spray follows.
                c.g[A0][0] = c.lw(self_at);
                c.g[V1][0] = c.lw(p(r32(c, A0), 0x6c0));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh(p(r32(c, V1), 0x10));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x14));
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                call_ptr(c, target, 0x002e_0fa0)?;
                // vel = *trail * curve(d8) + up * curve(dc)
                //     + (up * curve(e0)) * speed.
                c.g[V1][0] = c.lw(p(self_at, spray::CURVES));
                c.g[A0][0] = c.g[S1][0];
                c.lwc1(12, gp(c, -0x3af8)); // 1/60
                c.g[A3][0] = sx(r32(c, S0).wrapping_add(spray::COL));
                c.lwc1(0, p(r32(c, V1), 0xd8));
                c.g[A1][0] = sx(sp0);
                c.mfc1(3, 0);
                c.lqc2(4, r32(c, V0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x40));
                c.g[A2][0] = sx(p(sp0, 0x10));
                c.lq(2, p(sp0, 0x40));
                c.g[T0][0] = 1;
                c.g[T1][0] = c.lw(p(self_at, spray::CURVES));
                c.sq(2, p(sp0, 0x30));
                c.lwc1(0, p(r32(c, T1), 0xdc));
                c.g[V1][0] = c.lw(self_at);
                c.mfc1(10, 0);
                c.lqc2(4, p(r32(c, V1), 0x370));
                c.qmtc2(10, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x50));
                c.lq(2, p(sp0, 0x50));
                c.sq(2, p(sp0, 0x40));
                c.lqc2(3, p(sp0, 0x30));
                c.lqc2(4, p(sp0, 0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp0, 0x50));
                c.lq(2, p(sp0, 0x50));
                c.g[T1][0] = c.lw(p(self_at, spray::CURVES));
                c.sq(2, p(sp0, 0x20));
                c.lwc1(0, p(r32(c, T1), 0xe0));
                c.g[V1][0] = c.lw(self_at);
                c.mfc1(10, 0);
                c.lqc2(4, p(r32(c, V1), 0x370));
                c.qmtc2(10, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x70));
                c.lq(2, p(sp0, 0x70));
                c.lwc1(0, p(self_at, spray::SPEED));
                c.sq(2, p(sp0, 0x60));
                c.mfc1(3, 0);
                c.lqc2(4, p(sp0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x70));
                c.lq(2, p(sp0, 0x70));
                c.sq(2, p(sp0, 0x50));
                c.lqc2(3, p(sp0, 0x20));
                c.lqc2(4, p(sp0, 0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp0, 0x70));
                c.lq(2, p(sp0, 0x70));
                c.sq(2, p(sp0, 0x10));
                call_ptr(c, 0x0037_17c0, 0x002e_1080)?;
                frame.restore(c, S0, 0xa0);
                break 'out;
            }
            // Spray off: keep the emitter alive at the spray position.
            c.cycles += 8;
            c.lwc1(12, gp(c, -0x3af4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx(p(self_at, spray::POS));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, 0x0037_17c0, 0x002e_10a4)?;
            frame.restore(c, S0, 0xa0);
            break 'out;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    });
}


/// The global offset vectors used to keep the cell position in its cube.
pub(crate) const CELL_OFFSET_X: u32 = 0x004f_f140;

pub(crate) const CELL_OFFSET_Y: u32 = 0x004f_f150;

pub(crate) const CELL_OFFSET_Z: u32 = 0x004f_f160;


/// `lqc2 vf1..vf4, src; sqc2 vf1..vf4, dst` (a 64-byte matrix copy).
#[inline(always)]
pub(crate) fn copy_matrix(c: &mut Cpu, src: u32, dst: u32) {
    for k in 0..4u32 {
        c.lqc2(1 + k as usize, p(src, 0x10 * k));
        c.sqc2(1 + k as usize, p(dst, 0x10 * k));
    }
}


/// `void SnowBox_Render(Cell* cell, const Mat44* m, float x /*f12*/,
/// float y /*f13*/, float z /*f14*/)` (0x2e55d8): build the cell's render info
/// — model matrix `m * cell->mat`, cell-local position, colours and bounds —
/// nudge the position back into the cube the cell covers, then call the
/// renderer's cell draw (vtable slot 85) or the special cell draw (slot 86).
pub fn snow_box_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x120);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0x110);
        let self_at = r32(c, A3);
        let m = abi::arg(c, 1);
        let sp0 = sp(c);
        c.g[V0][0] = c.lw(p(self_at, snow_cell::ACTIVE));
        let inactive = c.g[V0][0] == 0;
        c.g[V1][0] = sx(p(sp0, 0xd0));
        'outer: {
            'done: {
                if inactive {
                    break 'done;
                }
                // info.mat = m * cell->mat (and two stack copies of it).
                c.cycles += 104;
                fconst(c, 0, 0x3f80_0000); // 1
                c.g[A0][0] = sx(p(sp0, 0x90));
                c.g[V0][0] = sx(p(self_at, snow_cell::MAT));
                c.add_s(10, 14, 0); // z1
                c.add_s(8, 12, 0); // x1
                c.add_s(9, 13, 0); // y1
                c.lqc2(4, m);
                c.lqc2(5, p(m, 0x10));
                c.lqc2(6, p(m, 0x20));
                c.lqc2(7, p(m, 0x30));
                let b = r32(c, V0);
                c.lqc2(8, b);
                c.lqc2(9, p(b, 0x10));
                c.lqc2(10, p(b, 0x20));
                c.lqc2(11, p(b, 0x30));
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
                c.sqc2(12, r32(c, V1));
                c.sqc2(13, p(r32(c, V1), 0x10));
                c.sqc2(14, p(r32(c, V1), 0x20));
                c.sqc2(15, p(r32(c, V1), 0x30));
                copy_matrix(c, p(sp0, 0xd0), p(sp0, 0x90));
                c.g[V0][0] = sx(p(sp0, 0x50));
                copy_matrix(c, p(sp0, 0x90), p(sp0, 0x50));
                // info.pos = cell->pos / cell->size.
                c.lwc1(0, p(self_at, snow_cell::SIZE));
                c.mfc1(2, 0);
                c.qmtc2(2, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, p(self_at, snow_cell::POS));
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, p(sp0, 0xa0));
                // The colours, scaled into 0..128.
                fconst(c, 1, 0x4300_0000); // 128
                c.lwc1(3, p(self_at, snow_cell::COL_R));
                c.lwc1(0, p(self_at, snow_cell::COL_G));
                c.lwc1(2, p(self_at, snow_cell::COL_B));
                c.mul_s(3, 3, 1);
                c.lwc1(4, p(self_at, snow_cell::EXTRA));
                c.mul_s(0, 0, 1);
                c.lq(2, p(sp0, 0xa0));
                c.mul_s(2, 2, 1);
                c.mul_s(4, 4, 1);
                c.g[V1][0] = c.lw(p(self_at, snow_cell::ACTIVE));
                c.g[A0][0] = c.lw(p(self_at, snow_cell::CELL_ID));
                c.g[A1][0] = c.lw(p(self_at, snow_cell::I9C));
                c.g[A2][0] = c.lw(p(self_at, snow_cell::IA0));
                c.lwc1(1, p(self_at, snow_cell::HEIGHT));
                c.lwc1(5, p(self_at, snow_cell::SIZE));
                c.sq(2, p(sp0, 0x90));
                fconst(c, 7, 0xbf00_0000); // -0.5
                c.sq(2, sp0);
                c.swc1(0, p(sp0, 0x94));
                c.add_s(6, 12, 7); // x - 0.5
                c.sw(p(sp0, 0x20), c.g[V1][0]);
                c.sw(p(sp0, 0x24), c.g[A0][0]);
                c.sw(p(sp0, 0x28), c.g[A1][0]);
                c.sw(p(sp0, 0x2c), c.g[A2][0]);
                c.swc1(1, p(sp0, 0x3c));
                c.swc1(5, p(sp0, 0x4c));
                c.swc1(3, p(sp0, 0x90));
                c.swc1(2, p(sp0, 0x98));
                c.swc1(12, p(sp0, 0x30));
                c.swc1(13, p(sp0, 0x34));
                c.swc1(14, p(sp0, 0x38));
                c.swc1(8, p(sp0, 0x40));
                c.swc1(9, p(sp0, 0x44));
                c.swc1(10, p(sp0, 0x48));
                c.swc1(4, p(sp0, 0x9c));
                c.lwc1(0, sp0);
                c.lq(2, p(sp0, 0x90));
                c.c_lt_s(0, 6); // pos.x < x - 0.5 ?
                let inside_low_x = !c.fcc();
                c.sq(2, p(sp0, 0x10));
                if !inside_low_x {
                    c.cycles += 6;
                    c.g[V0][0] = sx(CELL_OFFSET_X);
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                // Nudge the position back into the cube.
                c.cycles += 8;
                fconst(c, 2, 0x3f00_0000); // 0.5
                c.lwc1(1, sp0);
                c.add_s(0, 8, 2);
                c.c_lt_s(0, 1); // x1 + 0.5 < pos.x ?
                let t = !c.fcc();
                c.g[V0][0] = sx(CELL_OFFSET_X);
                if !t {
                    c.cycles += 5;
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                c.cycles += 6;
                c.add_s(1, 13, 7); // y - 0.5
                c.lwc1(0, p(sp0, 4));
                c.c_lt_s(0, 1); // pos.y < y - 0.5 ?
                let t = !c.fcc();
                c.g[V0][0] = sx(CELL_OFFSET_Y);
                if !t {
                    c.cycles += 5;
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                c.cycles += 6;
                c.add_s(1, 9, 2); // y1 + 0.5
                c.lwc1(0, p(sp0, 4));
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = sx(CELL_OFFSET_Y);
                if !t {
                    c.cycles += 5;
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                c.cycles += 6;
                c.add_s(1, 14, 7); // z - 0.5
                c.lwc1(0, p(sp0, 8));
                c.c_lt_s(0, 1);
                let t = !c.fcc();
                c.g[V0][0] = sx(CELL_OFFSET_Z);
                if !t {
                    c.cycles += 5;
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                c.cycles += 6;
                c.add_s(1, 10, 2); // z1 + 0.5
                c.lwc1(0, p(sp0, 8));
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = sx(CELL_OFFSET_Z);
                if !t {
                    c.cycles += 5;
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                // pos -= (1.5, 1.5, 1.5, 0)
                c.cycles += 13;
                fconst(c, 0, 0x3fc0_0000);
                c.sw(p(sp0, 0x9c), 0);
                c.swc1(0, p(sp0, 0x98));
                c.swc1(0, p(sp0, 0x90));
                c.swc1(0, p(sp0, 0x94));
                c.lqc2(3, sp0);
                c.lqc2(4, p(sp0, 0x90));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp0);
                'kind: {
                    'maybe0: {
                        c.g[A0][0] = c.lw(p(self_at, snow_cell::KIND));
                        let ordinary = c.g[A0][0] == 0;
                        c.g[V0][0] = 1;
                        if ordinary {
                            break 'maybe0;
                        }
                        c.cycles += 2;
                        let special = c.g[A0][0] == c.g[V0][0];
                        frame.restore_ra(c, 0x110);
                        if special {
                            break 'kind;
                        }
                        c.cycles += 2;
                        break 'outer;
                    }
                    // The ordinary cell: draw through vtable slot 85.
                    c.cycles += 9;
                    c.g[A2][0] = c.lw(gp(c, -0x854));
                    c.g[A1][0] = sx(sp0);
                    c.g[V0][0] = c.lw(p(r32(c, A2), gfx::VTABLE));
                    c.g[A0][0] = c.lh(p(r32(c, V0), 0x2a8));
                    c.g[V1][0] = c.lw(p(r32(c, V0), 0x2ac));
                    let target = r32(c, V1);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    call_ptr(c, target, 0x002e_58ec)?;
                    frame.restore_ra(c, 0x110);
                    break 'outer;
                }
                // The special cell: draw through vtable slot 86.
                c.cycles += 7;
                c.g[A2][0] = c.lw(gp(c, -0x854));
                c.g[A1][0] = sx(sp0);
                c.g[V0][0] = c.lw(p(r32(c, A2), gfx::VTABLE));
                c.g[A0][0] = c.lh(p(r32(c, V0), 0x2b0));
                c.g[V1][0] = c.lw(p(r32(c, V0), 0x2b4));
                let target = r32(c, V1);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x002e_5910)?;
                break 'done;
            }
            c.cycles += 1;
            frame.restore_ra(c, 0x110);
            break 'outer;
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `void cDynamicSprayFX_UpdateEmitter5(SprayFX* fx)` (0x2e1598): the main
/// board spray. At speed it spawns a particle stream from one of two emitter
/// slots (switching slot above 277 units/s), with random jitter, the rider's
/// up-vector and trail point, a smoothed alpha and the colour; otherwise it
/// keeps two emitter slots alive at the spray position.
pub fn dynamic_spray_fx_update_emitter5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xc0);
        c.lwc1(0, gp(c, -0x3abc)); // 27.7778
        frame.save(c, S1, 0x90);
        frame.save(c, S0, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 20, 0xb0);
        let self_at = r32(c, S1);
        let sp0 = sp(c);
        c.lwc1(1, p(self_at, spray5::SPEED));
        c.c_lt_s(0, 1); // 27.7778 < speed ?
        let slow = !c.fcc();
        c.g[S2][0] = c.lw(p(self_at, spray5::CFG));
        'out: {
            'on: {
                if slow {
                    break 'on;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::ENABLED));
                let disabled = c.g[V0][0] == 0;
                c.g[S3][0] = 6;
                if disabled {
                    break 'on;
                }
                // Slots: a = 6, b = 5, swapping above 277.7778.
                'slots: {
                    'flag: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                        let no_flag = c.g[V0][0] == 0;
                        c.g[S5][0] = 5;
                        if no_flag {
                            break 'flag;
                        }
                        c.cycles += 5;
                        c.lwc1(0, gp(c, -0x3ab8)); // 277.7778
                        c.c_lt_s(0, 1);
                        let under = !c.fcc();
                        c.g[V0][0] = 5;
                        if under {
                            break 'slots;
                        }
                        c.cycles += 2;
                        c.g[S3][0] = 5;
                        c.g[S5][0] = 6;
                        break 'flag;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 5;
                    break 'slots;
                }
                // v = fx->frame_pos; if a == 5, splash it.
                'v: {
                    'splash: {
                        c.cycles += 2;
                        let not_five = c.g[S3][0] != c.g[V0][0];
                        c.g[S4][0] = sx(p(self_at, spray5::FRAME_POS));
                        if !not_five {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(self_at);
                            c.g[V1][0] = c.lw(p(r32(c, V0), 0xac4));
                            if c.g[V1][0] == 0 {
                                c.cycles += 3;
                                c.lwc1(12, p(self_at, spray5::SPEED));
                                c.g[A0][0] = c.g[S4][0];
                                callee::ADD_SPLASH.call(c, 0x002e_1634)?;
                                break 'splash;
                            }
                            c.lq(2, r32(c, S4));
                            break 'v;
                        }
                    }
                    c.cycles += 1;
                    c.lq(2, r32(c, S4));
                }
                // k = 0.5 and the throw vector scaled by 20 in long-range
                // mode, else k = 1 and the throw vector scaled by 45.
                'scale: {
                    'short: {
                        c.cycles += 5;
                        c.g[V1][0] = 6;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 5);
                        let not_long = c.g[S3][0] != c.g[V1][0];
                        c.sq(2, sp0);
                        if not_long {
                            break 'short;
                        }
                        c.cycles += 11;
                        fconst(c, 0, 0x41a0_0000); // 20
                        c.mfc1(2, 0);
                        c.lqc2(4, p(self_at, spray5::THROW));
                        c.qmtc2(2, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, p(sp0, 0x20));
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(1, 5);
                        c.lq(2, p(sp0, 0x20));
                        break 'scale;
                    }
                    c.cycles += 8;
                    fconst(c, 0, 0x4234_0000); // 45
                    c.mfc1(3, 0);
                    c.lqc2(4, p(self_at, spray5::THROW));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp0, 0x20));
                    c.lq(2, p(sp0, 0x20));
                }
                // pos += throw * k; r = clamp(speed / 3333.3335, 0, 1).
                c.cycles += 15;
                c.sq(2, p(sp0, 0x10));
                c.lqc2(3, sp0);
                c.lqc2(4, p(sp0, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp0);
                c.lwc1(0, gp(c, -0x3ab4)); // 3333.3335
                c.lwc1(1, p(self_at, spray5::SPEED));
                c.mtc1(0, 2); // f2 = 0
                c.div_s(1, 1, 0);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0); // f0 = 1
                c.c_le_s(2, 1);
                let negative = !c.fcc();
                c.mov_s(4, 2);
                if negative {
                    // f4 = 0
                } else {
                    c.cycles += 1;
                    c.min_s(4, 1, 0);
                }
                // Level: fade towards 2 while a == 5 or the level is low.
                'level: {
                    'choose: {
                        c.cycles += 5;
                        c.lwc1(3, p(self_at, spray5::LEVEL));
                        c.c_lt_s(3, 0); // level < 1 ?
                        if c.fcc() {
                            c.g[V1][0] = c.lw(p(self_at, spray5::ID4));
                            break 'choose;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 5;
                        if c.g[S3][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                            break 'level;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(p(self_at, spray5::ID4));
                        break 'choose;
                    }
                    'update: {
                        'rate: {
                            c.cycles += 3;
                            c.g[V0][0] = 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V0][0] = c.lw(p(self_at, spray5::I114));
                                break 'rate;
                            }
                            c.cycles += 6;
                            fconst(c, 1, 0x4000_0000); // 2
                            c.c_lt_s(3, 1); // level < 2 ?
                            if !c.fcc() {
                                c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                                break 'level;
                            }
                            'clamp2: {
                                c.cycles += 7;
                                c.add_s(0, 4, 4);
                                c.mul_s(0, 0, 5);
                                c.add_s(0, 3, 0);
                                c.c_le_s(0, 1);
                                let t = !c.fcc();
                                c.mov_s(2, 1);
                                if t {
                                    // f2 = 2
                                } else {
                                    c.cycles += 1;
                                    c.mov_s(2, 0);
                                }
                            }
                            c.cycles += 2;
                            c.swc1(2, p(self_at, spray5::LEVEL));
                            break 'update;
                        }
                        'i114: {
                            'check114: {
                                c.cycles += 2;
                                let has114 = c.g[V0][0] != 0;
                                if has114 {
                                    break 'check114;
                                }
                                c.cycles += 6;
                                fconst(c, 0, 0x3fc0_0000); // 1.5
                                c.c_lt_s(3, 0); // level < 1.5 ?
                                if !c.fcc() {
                                    c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                                    break 'level;
                                }
                                c.cycles += 5;
                                c.mul_s(0, 5, 0);
                                c.g[AT][0] = 0x4000_0000;
                                c.mtc1(1, 1);
                                c.mul_s(0, 4, 0);
                                break 'i114;
                            }
                            c.cycles += 4;
                            c.c_lt_s(3, 0); // level < 1.5 ?
                            if !c.fcc() {
                                c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                                break 'level;
                            }
                            c.cycles += 3;
                            c.mul_s(0, 4, 5);
                            c.g[AT][0] = 0x4000_0000;
                            c.mtc1(1, 1);
                            break 'i114;
                        }
                        c.cycles += 3;
                        c.add_s(0, 3, 0);
                        c.min_s(0, 0, 1);
                        c.swc1(0, p(self_at, spray5::LEVEL));
                        break 'update;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                    break 'level;
                }
                // Jitter amplitude: 60 in long-range mode with i124 set,
                // else 25.
                'jitter: {
                    c.cycles += 4;
                    fconst(c, 2, 0x41c8_0000); // 25
                    let no_flag = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(gp(c, 0xa0c));
                    if no_flag {
                        break 'jitter;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(p(self_at, spray5::I124));
                    let no_124 = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(gp(c, 0xa0c));
                    if no_124 {
                        break 'jitter;
                    }
                    c.cycles += 2;
                    fconst(c, 2, 0x4270_0000); // 60
                }
                // Random jitter, two emitter calls, and the alpha/decay.
                c.cycles += 81;
                c.g[V1][0] = 0x0001_8fcd;
                c.g[A0][0] = 0x00e9_507c;
                c.mult(0, 2, 2, 3);
                c.g[A1][0] = 0x007f_ffff;
                c.g[A2][0] = 0x3f80_0000;
                c.neg_s(3, 2); // -j
                fconst(c, 20, 0x3f80_0000); // 1
                c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
                c.sw(p(sp0, 0x18), 0);
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.sub_s(2, 2, 3); // 2j
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                c.sw(p(sp0, 0x1c), 0);
                c.mult(0, 7, 2, 3);
                c.sw(gp(c, 0xa0c), c.g[V0][0]);
                c.lwc1(1, gp(c, 0xa0c));
                c.g[V1][0] = add32(c.g[A3][0], c.g[A0][0]);
                c.sub_s(1, 1, 20);
                c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
                c.sw(gp(c, 0xa0c), c.g[V1][0]);
                c.mul_s(1, 2, 1);
                c.lwc1(0, gp(c, 0xa0c));
                c.add_s(1, 3, 1);
                c.sub_s(0, 0, 20);
                c.swc1(1, p(sp0, 0x10));
                c.mul_s(2, 2, 0);
                c.add_s(3, 3, 2);
                c.swc1(3, p(sp0, 0x14));
                c.lqc2(3, sp0);
                c.lqc2(4, p(sp0, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp0);
                // The trail point (twice through the rider's vtable).
                c.g[A0][0] = c.lw(self_at);
                c.g[V1][0] = c.lw(p(r32(c, A0), 0x6c0));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh(p(r32(c, V1), 0x10));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x14));
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                call_ptr(c, target, 0x002e_187c)?;
                c.g[A0][0] = c.lw(self_at);
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(p(r32(c, A0), 0x6c0));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh(p(r32(c, V1), 0x10));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x14));
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                call_ptr(c, target, 0x002e_189c)?;
                // vel = trail - up * dot(trail, up) * 0.75.
                c.g[V1][0] = c.lw(self_at);
                fconst(c, 1, 0x3f40_0000); // 0.75
                c.lqc2(3, r32(c, V0));
                c.lqc2(5, p(r32(c, V1), 0x370));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(4, 4);
                c.mtc1(4, 0);
                c.lwc1(2, gp(c, -0x3ab0)); // 0.3
                c.mul_s(0, 0, 1);
                c.mfc1(4, 0);
                c.lqc2(4, p(r32(c, V1), 0x370));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x30));
                c.lq(2, p(sp0, 0x30));
                c.sq(2, p(sp0, 0x20));
                c.lqc2(3, r32(c, S0));
                c.lqc2(4, p(sp0, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp0, 0x30));
                c.lq(2, p(sp0, 0x30));
                c.g[V1][0] = c.lw(p(self_at, spray5::I114));
                let has114 = c.g[V1][0] != 0;
                c.sq(2, p(sp0, 0x10));
                if !has114 {
                    c.cycles += 1;
                    c.lwc1(2, gp(c, -0x3aac)); // 0.4
                }
                c.cycles += 11;
                c.mfc1(2, 2);
                c.lqc2(4, p(sp0, 0x10));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x30));
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 3);
                c.lq(2, p(sp0, 0x30));
                c.g[V1][0] = c.lw(p(self_at, spray5::I114));
                let has114 = c.g[V1][0] != 0;
                c.sq(2, p(sp0, 0x20));
                if !has114 {
                    c.cycles += 1;
                    c.mov_s(3, 20);
                }
                // Alpha: snap up, otherwise blend 0.8/0.2.
                'alpha: {
                    'snap: {
                        c.cycles += 4;
                        c.lwc1(2, p(self_at, spray5::ALPHA));
                        c.c_lt_s(2, 3);
                        let t = !c.fcc();
                        if t {
                            break 'snap;
                        }
                        c.cycles += 2;
                        c.swc1(3, p(self_at, spray5::ALPHA));
                        break 'alpha;
                    }
                    c.cycles += 6;
                    c.lwc1(1, gp(c, -0x3aa8)); // 0.8
                    c.lwc1(0, gp(c, -0x3aa4)); // 0.2
                    c.mul_s(1, 2, 1);
                    c.mul_s(0, 3, 0);
                    c.add_s(1, 1, 0);
                    c.swc1(1, p(self_at, spray5::ALPHA));
                    break 'alpha;
                }
                // Spawn in slot a with the jittered velocity, then in slot b
                // at the frame position.
                c.cycles += 30;
                c.g[S0][0] = 0x210;
                c.lq(2, p(self_at, spray5::COL));
                c.mult(0, 3, 19, 16); // a * 0x210
                c.lwc1(0, p(self_at, spray5::ALPHA));
                c.g[A0][0] = c.lw(p(self_at, spray5::P28));
                c.g[A1][0] = sx(sp0);
                c.lwc1(20, gp(c, -0x3aa0)); // 1/60
                c.g[A2][0] = sx(p(sp0, 0x20));
                c.sq(2, p(sp0, 0x30));
                c.g[A3][0] = sx(p(sp0, 0x30));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                c.swc1(0, p(sp0, 0x3c));
                c.g[T0][0] = 1;
                c.mov_s(12, 20);
                call_ptr(c, 0x0037_17c0, 0x002e_19b0)?;
                c.mult(0, 16, 21, 16); // b * 0x210
                c.g[A0][0] = c.lw(p(self_at, spray5::P28));
                c.g[A1][0] = c.g[S4][0];
                c.mov_s(12, 20);
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
                call_ptr(c, 0x0037_17c0, 0x002e_19d4)?;
                // Decay the strength.
                c.lwc1(2, p(self_at, spray5::SPEED));
                c.lwc1(0, gp(c, -0x3a9c)); // 83.3333
                c.c_lt_s(2, 0);
                if c.fcc() {
                    c.sw(p(self_at, spray5::SPEED), 0);
                    break 'out;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(p(self_at, spray5::I124));
                let has124 = c.g[V0][0] != 0;
                c.lwc1(1, p(r32(c, S2), spray_cfg::DECAY));
                if !has124 {
                    c.cycles += 2;
                    c.lwc1(0, gp(c, -0x3a98)); // 0.4
                    c.mul_s(1, 1, 0);
                }
                c.cycles += 3;
                c.mul_s(0, 2, 1);
                c.swc1(0, p(self_at, spray5::SPEED));
                break 'out;
            }
            // Spray off: keep the two emitter slots alive at the position.
            c.cycles += 19;
            c.g[A0][0] = c.lw(p(self_at, spray5::P28));
            c.g[S0][0] = sx(p(self_at, spray5::POS));
            c.lwc1(20, gp(c, -0x3a94)); // 1/60
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xc60));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            c.mov_s(12, 20);
            call_ptr(c, 0x0037_17c0, 0x002e_1a34)?;
            c.g[A0][0] = c.lw(p(self_at, spray5::P28));
            c.g[A1][0] = c.g[S0][0];
            c.mov_s(12, 20);
            c.g[A2][0] = 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xa50));
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, 0x0037_17c0, 0x002e_1a54)?;
            c.sw(p(self_at, spray5::SPEED), 0);
            break 'out;
        }
        c.cycles += 10;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}


/// `lwc1 fN, a`.
#[inline(always)]
pub(crate) fn ldf_2(c: &mut Cpu, n: usize, a: u32) -> FpuF32 {
    let v = Addr(a).read_fpu(c);
    c.f[n] = v.0;
    v
}


/// FPU register `n`.
#[inline(always)]
pub(crate) fn fr_2(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}


/// `fN = v`.
#[inline(always)]
pub(crate) fn setf_2(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}


/// `swc1 fN, a`.
#[inline(always)]
pub(crate) fn stf(c: &mut Cpu, n: usize, a: u32) {
    let v = fr_2(c, n);
    Addr(a).write_fpu(c, v);
}


/// `li at, bits; mtc1 at, fN`.
#[inline(always)]
pub(crate) fn li_f(c: &mut Cpu, n: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = sx(bits);
    setf_2(c, n, FpuF32(bits))
}


/// Set the FPU condition bit; returns the condition.
#[inline(always)]
pub(crate) fn fcc(c: &mut Cpu, cond: bool) -> bool {
    abi::set_fcc(c, cond);
    cond
}


/// `lw`.
#[inline(always)]
pub(crate) fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}


/// `lh`.
#[inline(always)]
pub(crate) fn lh(c: &mut Cpu, a: u32) -> u64 {
    c.lh(a)
}


/// The 20-byte render-state copy the compiler emits as two unaligned 8-byte
/// loads/stores plus a word, leaving the loaded values in the given GPRs.
#[inline(always)]
pub(crate) fn copy_render_state(c: &mut Cpu, r0: usize, r1: usize, word: usize, src: u32, dst: u32) {
    c.ldl(r0, p(src, 7));
    c.ldr(r0, src);
    c.ldl(r1, p(src, 0xf));
    c.ldr(r1, p(src, 8));
    c.g[word][0] = lw(c, p(src, 0x10));
    c.sdl(r0, p(dst, 0x1b));
    c.sdr(r0, p(dst, 0x14));
    c.sdl(r1, p(dst, 0x23));
    c.sdr(r1, p(dst, 0x1c));
    let w = c.g[word][0];
    c.sw(p(dst, 0x24), w);
}


// ---------------------------------------------------------------------------
// cPlayerPulseFX_Update (0x2e39d8).

/// `void cPlayerPulseFX_Update(cPlayerPulseFX* p)` (0x2e39d8): if the player's
/// FX object is active, advance the pulse phase by
/// `rate * (1/60)` (`rate` = `D_004A55AC`), wrap it into `[0, 2π)` with the
/// FPU truncation the original uses, store it, and blend the four output
/// values between `a` (`+0x8`) and `b` (`+0x18`) with
/// `s = 0.5 + 0.5*cos(phase)`.
pub fn player_pulsefx_update(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let pfx = r32(c, A0);
    c.g[S0][0] = c.g[A0][0];
    frame.save_fpr(c, 20, 0x20);
    guest(c, |c| {
        'body: {
        let rider = lw(c, pfx) as u32;
        c.g[A0][0] = sx(rider);
        let vt = lw(c, p(rider, 0x6c0)) as u32;
        c.g[V1][0] = sx(vt);
        c.g[A0][0] = sx(p(rider, 0x6c0));
        c.g[A1][0] = lh(c, p(vt, 0x40));
        let f = lw(c, p(vt, 0x44));
        c.g[V0][0] = f;
        let f = f as u32;
        c.g[A0][0] = sx(p(rider, 0x6c0).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, f, 0x002e_3a08)?;
        let t = c.g[V0][0] == 0;
        // Read before the branch: the original loads `rate` first.
        ldf_2(c, 1, gp(c, 0x24bc));
        if t {
            break 'body;
        }
        c.cycles += 14;
        let rate = ldf_2(c, 2, gp(c, -0x3a30));
        let phase = ldf_2(c, 3, p(pfx, 4));
        let half_rate = setf_2(c, 1, rate * fr_2(c, 1));
        let w = ldf_2(c, 0, gp(c, -0x3a2c));
        li_f(c, 20, 0x3f00_0000); // 0.5
        let p_ = setf_2(c, 3, phase + half_rate);
        let prod = setf_2(c, 0, p_ * w);
        let t_ = setf_2(c, 0, prod + fr_2(c, 20));
        c.cvt_w_s(1, 0);
        c.cvt_s_w(1, 1);
        let floor = fcc(c, t_.lt(fr_2(c, 1)));
        if floor {
            c.cycles += 3;
            let one = li_f(c, 0, 0x3f80_0000);
            setf_2(c, 1, fr_2(c, 1) - one);
        }
        c.cycles += 35;
        let two_pi = ldf_2(c, 0, gp(c, -0x3a28));
        let prod = setf_2(c, 0, fr_2(c, 1) * two_pi);
        let q = setf_2(c, 0, fr_2(c, 3) - prod);
        setf_2(c, 12, q);
        stf(c, 0, p(pfx, 4));
        call_ptr(c, callee::MATH_COS, 0x002e_3a6c)?;
        let half = setf_2(c, 0, fr_2(c, 0) * fr_2(c, 20));
        let one = li_f(c, 1, 0x3f80_0000);
        let a0 = ldf_2(c, 5, p(pfx, 0x8));
        let b0 = ldf_2(c, 7, p(pfx, 0x18));
        let s = setf_2(c, 0, half + fr_2(c, 20));
        let a1 = ldf_2(c, 4, p(pfx, 0xc));
        let b1 = ldf_2(c, 6, p(pfx, 0x1c));
        let a3 = ldf_2(c, 3, p(pfx, 0x14));
        let u = setf_2(c, 1, one - s);
        let b3 = ldf_2(c, 2, p(pfx, 0x24));
        let a2 = ldf_2(c, 9, p(pfx, 0x10));
        let sa0 = setf_2(c, 5, s * a0);
        let b2 = ldf_2(c, 8, p(pfx, 0x20));
        let sa1 = setf_2(c, 4, s * a1);
        let sa3 = setf_2(c, 3, s * a3);
        let ub3 = setf_2(c, 2, u * b3);
        let ub0 = setf_2(c, 7, u * b0);
        let ub1 = setf_2(c, 6, u * b1);
        let sa2 = setf_2(c, 0, s * a2);
        let ub2 = setf_2(c, 1, u * b2);
        let out0 = setf_2(c, 5, sa0 + ub0);
        let out1 = setf_2(c, 4, sa1 + ub1);
        let out2 = setf_2(c, 0, sa2 + ub2);
        let out3 = setf_2(c, 3, sa3 + ub3);
        stf(c, 5, p(pfx, 0x28));
        stf(c, 4, p(pfx, 0x2c));
        stf(c, 0, p(pfx, 0x30));
        stf(c, 3, p(pfx, 0x34));
        let _ = (out0, out1, out3);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 20, 0x20);
        frame.restore_ra(c, 0);
    frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// cTransitionMan_Render (0x2e47e8).

/// `void cTransitionMan_Render(cTransitionMan* t)` (0x2e47e8): only for the
/// transition of the active rider. Update the screen rectangle, retire a
/// pending fade reference, compute the fade amount (fade-in, hold, fade-out
/// between the two fade objects), clamp it to `[0, 1]`, reset the world
/// painter when it is nearly opaque, push a screen fade render state, set its
/// blend/texture words, call the fade object (`vt[2]`) and pop the state.
pub fn transition_man_render(c: &mut Cpu) {
    c.cycles += 16;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S1, 0x20);
    frame.save(c, S0, 0x30);
    let trans = r32(c, A0);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S2, 0x10);
    frame.save_ra(c, 0);
    frame.save_fpr(c, 22, 0x50);
    frame.save_fpr(c, 21, 0x48);
    frame.save_fpr(c, 20, 0x40);
    guest(c, |c| {
        'body: {
        let world = lw(c, gp(c, globals::WORLD));
        c.g[A1][0] = world;
        let wm = lw(c, p(world as u32, 0x84));
        c.g[V0][0] = wm;
        let rider = lw(c, p(trans, 0x74));
        c.g[A2][0] = rider;
        let race = lw(c, p(wm as u32, 0x84));
        c.g[V1][0] = race;
        let active = lw(c, p(race as u32, 0x14));
        c.g[A0][0] = active;
        let t = c.g[A2][0] != c.g[A0][0];
        frame.restore(c, S0, 0x30);
        if t {
            break 'body;
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x002e_4830);
        c.pc = 0x002e_b8f0;
        callee::TRANSITION_EFFECT_UPDATE_SCREEN_RECT.call(c, 0x002e_4830)?;
        let on = lw(c, p(trans, 0x44));
        c.g[V0][0] = on;
        let t = on == 0;
        let context = lw(c, gp(c, 0x2a90));
        c.g[S0][0] = context;
        if !t {
            // Pending fade reference.
            c.cycles += 3;
            let pending = lw(c, p(trans, 0x64));
            c.g[V0][0] = pending;
            let t = pending == 0;
            ldf_2(c, 20, p(trans, 0x48)); // f20 = time
            if t {
                // straight to the fade computation
            } else {
                c.cycles += 7;
                let fade_in = ldf_2(c, 0, p(trans, 0x54));
                let hold = ldf_2(c, 1, p(trans, 0x58));
                let sum = setf_2(c, 0, fade_in + hold);
                let reached = fcc(c, sum.le(fr_2(c, 20)));
                c.g[A1][0] = sx(p(trans, 0x68));
                if reached {
                    c.cycles += 6;
                    let vt = lw(c, p(context as u32, ctx::VTABLE));
                    c.g[V0][0] = vt;
                    c.g[A0][0] = lh(c, p(vt as u32, 0x1e8));
                    let f = lw(c, p(vt as u32, 0x1ec));
                    c.g[V1][0] = f;
                    let f = f as u32;
                    c.g[RA][0] = sx(0x002e_4878);
                    c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                    call_ptr(c, f, 0x002e_4878)?;
                    Addr(p(trans, 0x64)).sw(c, 0);
                }
            }
            // Fade amount and target object.
            c.cycles += 3;
            let mut o = 0u32;
            let mut amount;
            let o1 = lw(c, p(trans, 0x4c));
            c.g[V0][0] = o1;
            if o1 == 0 {
                c.cycles += 9;
                let hold = ldf_2(c, 0, p(trans, 0x58));
                let fade_out = ldf_2(c, 2, p(trans, 0x5c));
                let diff = setf_2(c, 0, fr_2(c, 20) - hold);
                let one = li_f(c, 1, 0x3f80_0000);
                o = lw(c, p(trans, 0x50)) as u32;
                c.g[S2][0] = sx(o);
                let q = setf_2(c, 0, diff.div(fade_out));
                amount = setf_2(c, 0, one - q);
            } else {
                // f1 = fadeIn
                c.cycles += 3;
                let fade_in = ldf_2(c, 1, p(trans, 0x54));
                let in_ = fcc(c, fr_2(c, 20).lt(fade_in));
                if in_ {
                    c.cycles += 3;
                    let q = setf_2(c, 0, fr_2(c, 20).div(fr_2(c, 1)));
                    o = o1 as u32;
                    c.g[S2][0] = sx(o);
                    amount = q;
                } else {
                    c.cycles += 3;
                    let o2 = lw(c, p(trans, 0x50));
                    c.g[A0][0] = o2;
                    if o2 != 0 {
                        let hold = ldf_2(c, 0, p(trans, 0x58));
                        c.cycles += 5;
                        let sum = setf_2(c, 0, fr_2(c, 1) + hold);
                        let holding = fcc(c, fr_2(c, 20).lt(sum));
                        c.g[S2][0] = o1;
                        if holding {
                            c.cycles += 4;
                            amount = li_f(c, 0, 0x3f80_0000);
                            o = o1 as u32;
                            c.g[S2][0] = sx(o);
                        } else {
                            c.cycles += 7;
                            let diff = setf_2(c, 0, fr_2(c, 20) - sum);
                            let fade_out = ldf_2(c, 1, p(trans, 0x5c));
                            let one = li_f(c, 2, 0x3f80_0000);
                            c.g[S2][0] = o2;
                            o = o2 as u32;
                            let q = setf_2(c, 0, diff.div(fade_out));
                            amount = setf_2(c, 0, one - q);
                        }
                    } else {
                        c.cycles += 3;
                        let q = setf_2(c, 0, fr_2(c, 20).div(fr_2(c, 1)));
                        o = o1 as u32;
                        c.g[S2][0] = sx(o);
                        amount = q;
                    }
                }
            }
            // Clamp to [0, 1] into f22.
            c.cycles += 7;
            c.mtc1(0, 20); // f20 = 0.0
            c.g[AT][0] = sx(0x3f80_0000);
            c.mtc1(1, 21); // f21 = 1.0
            let nonneg = fcc(c, fr_2(c, 20).le(amount));
            c.mov_s(22, 20);
            if nonneg {
                c.cycles += 1;
                c.min_s(22, 0, 21);
            }
            // Nearly opaque: reset the world painter.
            c.cycles += 5;
            let threshold = ldf_2(c, 0, gp(c, -0x3a1c));
            let opaque = fcc(c, threshold.le(fr_2(c, 22)));
            if opaque {
                c.cycles += 3;
                call_ptr(c, callee::WORLD_PAINTER_QUERY_RESET_ALL, 0x002e_494c)?;
            }
            let top = lw(c, p(context as u32, ctx::TOP));
            c.g[V0][0] = top;
            // Push a copy of the render state and run the screen setup.
            'push: {
                c.cycles += 105;
                copy_render_state(c, V1, A0, A1, top as u32, top as u32);
                let nt = lw(c, p(context as u32, ctx::TOP));
                c.g[V1][0] = nt;
                c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[V1][0] = sx((nt as u32).wrapping_add(0x14));
                Addr(p(context as u32, ctx::TOP)).sw(c, c.g[V1][0]);
                let vt = r32(c, V0);
                c.g[A0][0] = lh(c, p(vt, 0xa8));
                let f = lw(c, p(vt, 0xac));
                c.g[V1][0] = f;
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_4998);
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, f, 0x002e_4998)?;
                c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[A0][0] = lh(c, p(r32(c, V0), 0xf8));
                let f = lw(c, p(r32(c, V0), 0xfc));
                c.g[V1][0] = f;
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_49ac);
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, f, 0x002e_49ac)?;
                c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[A0][0] = lh(c, p(r32(c, V0), 0x128));
                let f = lw(c, p(r32(c, V0), 0x12c));
                c.g[V1][0] = f;
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_49c0);
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, f, 0x002e_49c0)?;
                c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.mov_s(12, 20);
                c.g[AT][0] = sx(0x4420_0000);
                c.mtc1(1, 14); // f14 = 640.0
                c.g[A1][0] = 0;
                c.g[A0][0] = lh(c, p(r32(c, V0), 0xd0));
                c.g[A2][0] = 0;
                let f = lw(c, p(r32(c, V0), 0xd4));
                c.g[V1][0] = f;
                c.mov_s(17, 21);
                c.g[AT][0] = sx(0x43f0_0000);
                c.mtc1(1, 15); // f15 = 480.0
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                c.mov_s(13, 12);
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_49fc);
                c.mov_s(16, 12);
                call_ptr(c, f, 0x002e_49fc)?;
                // Blend words of the pushed state.
                let vt = lw(c, p(context as u32, ctx::VTABLE));
                let top = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A3][0] = sx(top);
                c.g[A0][0] = 0xffff_ffff_ffff_fc1f;
                c.g[V1][0] = 0xffff_ffff_ffbf_ffff;
                c.g[T3][0] = 0x40_0000;
                c.g[V0][0] = lw(c, p(top, 8));
                c.g[A1][0] = 0xffff_ffff_fe7f_ffff;
                c.g[T4][0] = 0x100_0000;
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.g[V0][0] = c.g[V0][0] | 0x100;
                c.g[A2][0] = 0xffff_ffff_ffcf_ffff;
                Addr(p(top, 8)).sw(c, c.g[V0][0]);
                c.g[A3][0] = 0xffff_ffff_fff0_0fff;
                c.g[T0][0] = 0x1_4000;
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[T1][0] = sx(a);
                c.g[T5][0] = 0xffff_ffff_ffff_ff83;
                c.g[T6][0] = 0xffff_ffff_ffff_fffc;
                c.g[T2][0] = 0xffff_ffff_e000_03ff;
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A0][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[V1][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A0][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A1][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[V1][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 0));
                c.g[V0][0] = c.g[V0][0] | 0xc;
                Addr(p(a, 0)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A1][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 4));
                c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
                c.g[V0][0] = c.g[V0][0] | 0x2;
                Addr(p(a, 4)).sw(c, c.g[V0][0]);
                let a = lw(c, p(context as u32, ctx::TOP)) as u32;
                c.g[A0][0] = sx(a);
                c.g[V0][0] = lw(c, p(a, 8));
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                Addr(p(a, 8)).sw(c, c.g[V0][0]);
                let _ = vt;
                // Texture id.
                let use_tex = lw(c, p(trans, 0x60));
                c.g[V1][0] = use_tex;
                if use_tex == 0 {
                    let top = lw(c, p(context as u32, ctx::TOP));
                    c.g[V1][0] = top;
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    let top = c.g[V1][0];
                    c.sh(p(top as u32, 0x10), c.g[V0][0]);
                } else {
                    c.cycles += 4;
                    let tex = lw(c, p(trans, 0x6c));
                    c.g[V1][0] = tex;
                    let top = lw(c, p(context as u32, ctx::TOP));
                    c.g[V0][0] = top;
                    c.sh(p(top as u32, 0x10), c.g[V1][0]);
                }
            }
            // Fade object: vt[2](o, amount) unless a pending tex fade owns it.
            'fade: {
                c.cycles += 3;
                let use_tex = lw(c, p(trans, 0x60));
                c.g[V0][0] = use_tex;
                let (vt, is_ctx) = if use_tex == 0 {
                    let vt = lw(c, o) as u32;
                    c.g[V0][0] = sx(vt);
                    (vt, false)
                } else {
                    c.cycles += 3;
                    let pending = lw(c, p(trans, 0x64));
                    c.g[V0][0] = pending;
                    if pending != 0 {
                        let vt = lw(c, p(context as u32, ctx::VTABLE)) as u32;
                        c.g[V0][0] = sx(vt);
                        (vt, true)
                    } else {
                        c.cycles += 1;
                        let vt = lw(c, o) as u32;
                        c.g[V0][0] = sx(vt);
                        (vt, false)
                    }
                };
                if !is_ctx {
                    // vt[2] of the fade object, amount in f12.
                    c.cycles += 6;
                    c.mov_s(12, 22);
                    c.g[A0][0] = lh(c, p(vt, 0x10));
                    let f = lw(c, p(vt, 0x14));
                    c.g[V1][0] = f;
                    let f = f as u32;
                    c.g[RA][0] = sx(0x002e_4b3c);
                    c.g[A0][0] = sx(o.wrapping_add(r32(c, A0)));
                    call_ptr(c, f, 0x002e_4b3c)?;
                    c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                }
                // vt[32](); vt[22](); top--.
                c.cycles += 12;
                c.g[A0][0] = lh(c, p(r32(c, V0), 0x100));
                let f = lw(c, p(r32(c, V0), 0x104));
                c.g[V1][0] = f;
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_4b50);
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, f, 0x002e_4b50)?;
                c.g[V0][0] = lw(c, p(context as u32, ctx::VTABLE));
                c.g[A0][0] = lh(c, p(r32(c, V0), 0xb0));
                let f = lw(c, p(r32(c, V0), 0xb4));
                c.g[V1][0] = f;
                let f = f as u32;
                c.g[RA][0] = sx(0x002e_4b64);
                c.g[A0][0] = sx((context as u32).wrapping_add(r32(c, A0)));
                call_ptr(c, f, 0x002e_4b64)?;
                let top = lw(c, p(context as u32, ctx::TOP));
                c.g[V0][0] = top;
                c.g[V0][0] = sx((top as u32).wrapping_sub(0x14));
                let top = c.g[V0][0];
                Addr(p(context as u32, ctx::TOP)).sw(c, top);
            }
            let _ = amount;
        }
        c.cycles += 1;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}


/// `void cDynamicSprayFX_Render(Spray* self)` (0x2e24d0): if the owner is
/// live, render each of the ten wakes that has an emitter and live particles.
pub fn dynamic_spray_fx_render(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r32(c, S2));
        c.g[V1][0] = c.lw(at_2(c, V0, 0xb18));
        let idle = c.g[V1][0] == 0;
        c.g[S1][0] = 0;
        if !idle {
            c.cycles += 2;
            c.g[S0][0] = 0x9;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lw(at_2(c, S2, 0x28));
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
                c.g[V1][0] = c.lw(at_2(c, A0, wake::EMITTER));
                if c.g[V1][0] == 0 {
                    addi(c, S0, 0xffff_ffff);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at_2(c, A0, wake::LIVE));
                    if (c.g[V0][0] as i64) <= 0 {
                        addi(c, S0, 0xffff_ffff);
                    } else {
                        c.cycles += 3;
                        c.g[A1][0] = 0x7;
                        callee::EMITTER_RENDER.call(c, 0x002e_2528)?;
                        addi(c, S0, 0xffff_ffff);
                    }
                }
                c.cycles += 2;
                let more = (c.g[S0][0] as i64) >= 0;
                addi(c, S1, 0x210);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_2500);
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

/// The frame-rate state (`D_004A5B64`, `gp+0x2a74`).
pub(crate) const APP_STATE: i32 = 0x2a74;


// ---------------------------------------------------------------------------
// cTransitionMan_Update.

/// `void cTransitionMan_Update(cTransitionMan* self)` (0x2e46c8): the
/// board-wake fade. `+0x7c` counts fade ticks while `+0x44` is set: the
/// effect clock's frame count (`world->+0x10d8` vtable slot 114) minus
/// `+0x80` reaching 3 ends it (vtable slot 60 gets `self+0x68`) and clears
/// `+0x7c`. Then, unless `+0x44` (fading) or `+0x40` (paused) is zero, the
/// fade position `+0x48` advances: `+0x18 > 0` takes the explicit value
/// `+0x3c`, a non-null `+0x70` scales the surface colour by the three
/// weights, otherwise it adds one frame time (`D_004A5B64->+0x10`); when
/// the weights' sum is at most the new position the transition stops.
pub fn c_transition_man_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).off(0x7c).lw(c);
        let fading = c.g[V0][0] != 0;
        // ctx = D_004A5B80 (loaded in the original's delay slot).
        let ctx = lw32(c, gp(c, BOARD_WAKE_WORLD));
        c.g[S1][0] = sx(ctx);
        if fading {
            c.cycles += 10;
            let vt = lw32(c, p(ctx, 0x10d8));
            c.g[V1][0] = sx(vt);
            c.g[A0][0] = Addr(vt).off(0x390).lh(c);
            c.g[V0][0] = Addr(vt).off(0x394).lw(c);
            let target = r32(c, V0);
            c.g[RA][0] = sx(0x002e_46fc);
            c.g[A0][0] = sx(ctx.wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002e_46fc)?;
            c.g[V1][0] = Addr(this).off(0x80).lw(c);
            let elapsed = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
            c.g[V0][0] = elapsed;
            let expired = (elapsed as i64) < 3;
            c.g[V0][0] = expired as u64;
            if expired {
                // The ticks path rejoins the fade code without the extra
                // cycle the other two paths pay.
                c.g[V0][0] = Addr(this).off(0x44).lw(c);
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(ctx).off(0x10d8).lw(c);
                c.g[A1][0] = sx(p(this, 0x68));
                c.g[A0][0] = Addr(r32(c, V0)).off(0x1e0).lh(c);
                c.g[V1][0] = Addr(r32(c, V0)).off(0x1e4).lw(c);
                let target = r32(c, V1);
                let delta = r32(c, A0);
                c.g[RA][0] = sx(0x002e_4728);
                c.g[A0][0] = sx(ctx.wrapping_add(delta));
                call_ptr(c, target, 0x002e_4728)?;
                let v = 0u64;
                c.sw(p(this, 0x7c), v);
                c.cycles += 1;
                c.g[V0][0] = Addr(this).off(0x44).lw(c);
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(this).off(0x44).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            restore_s0(c, frame);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x40).lw(c);
            if c.g[V0][0] != 0 {
                restore_s0(c, frame);
            } else {
                fade_screen(c, this, frame)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `lq s0, 0x20(sp)`: the paths that leave the fade early restore it before
/// the shared epilogue.
#[inline]
pub(crate) fn restore_s0(c: &mut Cpu, frame: Frame) {
    let a = frame.at(c, 0x20);
    c.lq(S0, a);
}


/// The fade-position update and the transition stop (0x2e479c onward).
pub(crate) fn fade_screen(c: &mut Cpu, this: u32, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(this).off(0x18).lw(c);
    if (c.g[V0][0] as i64) <= 0 {
        c.g[V0][0] = Addr(this).off(0x70).lw(c);
        // The surface colour or the frame time?
        let ptr = r32(c, V0);
        c.cycles += 2;
        if ptr == 0 {
            c.g[V0][0] = Addr(gp(c, APP_STATE)).lw(c);
            c.cycles += 7;
            // pos += 1.0 / (float)frame_time
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.lwc1(2, p(r32(c, V0), 0x10));
            c.cvt_s_w(2, 2);
            c.lwc1(0, p(this, 0x48));
            c.div_s(1, 1, 2);
            c.add_s(0, 0, 1);
        } else {
            c.cycles += 8;
            // pos = *colour * (w0 + w1 + w2)
            let w0 = ldf_3(c, 1, p(this, 0x54));
            let w1 = ldf_3(c, 2, p(this, 0x58));
            let w2 = ldf_3(c, 3, p(this, 0x5c));
            setf_3(c, 1, w0 + w1);
            let v = ldf_3(c, 0, ptr);
            setf_3(c, 0, v);
            setf_3(c, 1, FpuF32(c.f[1]) + w2);
            setf_3(c, 0, FpuF32(c.f[0]) * FpuF32(c.f[1]));
        }
    } else {
        c.cycles += 2;
        c.lwc1(0, p(this, 0x3c));
    }

    // Store the new position and stop the transition once it reaches the
    // total (`w0 + w1 + w2 <= pos`).
    c.cycles += 11;
    stf_2(c, 0, p(this, 0x48));
    let w0 = ldf_3(c, 0, p(this, 0x54));
    let w1 = ldf_3(c, 1, p(this, 0x58));
    let w2 = ldf_3(c, 3, p(this, 0x5c));
    setf_3(c, 0, w0 + w1);
    let pos = ldf_3(c, 2, p(this, 0x48));
    let total = setf_3(c, 0, FpuF32(c.f[0]) + w2);
    if fcc(c, total.le(pos)) {
        c.cycles += 3;
        c.g[RA][0] = sx(0x002e_47d0);
        c.g[A0][0] = c.g[S0][0];
        callee::TRANSITION_MAN_STOP.call(c, 0x002e_47d0)?;
    }
    restore_s0(c, frame);
    Ok(())
}


/// `void SnowBox_dtor(SnowBox *this, int flags)` (0x2e4f28).
pub fn snow_box_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002e_4f40))
}


/// `cTransitionMan_ScriptDeactivate(cTransitionMan* m)` (0x2e4c60):
/// deactivate the script object at `m+0x10`, then stop the transition.
pub fn c_transition_man_script_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0028_3440, 0x002e_4c78)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002e_4578, 0x002e_4c80)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cTransitionMan_ResetEffectPool()` (0x2e4cb0): clear the four global
/// transition effect entries.
pub fn c_transition_man_reset_effect_pool(c: &mut Cpu) {
    const HEAD: u32 = 0x002e_4cc0;
    c.cycles += 4;
    c.g[V0][0] = 0x0054_0000;
    c.g[V1][0] = 3;
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x77b0));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0xc));
    loop {
        c.cycles += 7;
        Addr(r32(c, V0)).sw(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// cTransitionMan_StartCaptured.

/// `void cTransitionMan_StartCaptured(this, a1, a2, a3, float duration)`
/// (0x2e4540): forward to `cTransitionMan_Start` with a zeroed destination
/// mode, the zero/fraction pair `(0, 0)`, `t0 = 1` and the caller's `a3` in
/// `t1`; the capture duration moves from `f12` to `f14`.
pub fn c_transition_man_start_captured(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.mtc1(0, 0);
        c.mov_s(14, 12);
        c.g[T1][0] = c.g[A3][0];
        let frame = Frame::open(c, 0x10);
        c.mov_s(12, 0);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.mov_s(13, 12);
        c.g[T0][0] = 1;
        callee::TRANSITION_MAN_START.call(c, 0x002e_456c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void TransitionEffect_FreeData(void* p)` (0x2e4d30): when `p` is one of
/// the four transition-effect buffers (`D_005382C0`, stride 0x64), clear its
/// data slot.
pub fn transition_effect_free_data(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V1][0] = 0x0054_0000;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = 0x0054_0000;
    c.g[A2][0] = sx(0x0054_0000u32.wrapping_sub(0x77b0));
    c.g[V1][0] = sx(0x0054_0000u32.wrapping_sub(0x7d40));
    c.g[A1][0] = 0;
    loop {
        c.cycles += 2;
        let found = c.g[V1][0] == c.g[A0][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        if found {
            c.cycles += 2;
            let ra = r32(c, RA);
            c.sw(r32(c, A2), 0);
            c.pc = ra;
            return;
        }
        c.cycles += 4;
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
        c.g[V0][0] = ((r32(c, A1) as i64) < 4i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x64));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002e_4d48);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Script engine: object teardown and the script sound-bank list.

/// `void ScriptObj_Unregister(ScriptObj* obj)` (0x2832d8): remove `obj` from
/// the script engine's object list (`engine+0x2a8` holds a vtable-like table
/// whose slot 3 is `remove(this, obj->id)`), then mark the object dead with
/// `obj+4 = -1`.
pub fn script_obj_unregister(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_INSTANCE.call(c, 0x0028_32ec)?;
        let engine = r32(c, V0);
        c.g[V1][0] = c.lw(engine.wrapping_add(0x2a8));
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(4));
        let table = r32(c, V1);
        c.g[A0][0] = c.lh(table.wrapping_add(0x18));
        c.g[A2][0] = c.lw(table.wrapping_add(0x1c));
        let method = r32(c, A2);
        c.g[A0][0] = sx(engine.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0028_3304)?;
        c.g[V1][0] = u64::MAX;
        c.sw(r32(c, S0).wrapping_add(4), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* TransitionEffect_AllocData()` (0x2e4ce8): take the first free slot
/// of the four transition-effect wake pools (0x538850), marking it used; when
/// all four are busy return the fallback pool (`gp_2+0xa2c`).
pub fn transition_effect_alloc_data(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = 0x54_0000;
    c.g[V0][0] = 0x54_0000;
    c.g[A1][0] = sx(0x54_0000u32.wrapping_sub(0x7d40));
    c.g[V1][0] = sx(0x54_0000u32.wrapping_sub(0x77b0));
    c.g[A0][0] = 0;
    c.g[A2][0] = 1;
    let mut slot = c.g[V1][0] as u32;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(slot).lw(c);
        let busy = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        if busy {
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x64));
            c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(slot.wrapping_add(4));
            slot = c.g[V1][0] as u32;
            if !more {
                c.cycles += 2;
                c.g[V0][0] = Addr(gp_2(c, 0xa2c)).lw(c);
                abi::ret(c);
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_4d00);
            }
            continue;
        }
        c.cycles += 3;
        Addr(slot).sw(c, c.g[A2][0]);
        c.g[V0][0] = c.g[A1][0];
        abi::ret(c);
        return;
    }
}


// ---------------------------------------------------------------------------
// Transitions and lights.

/// `int cTransitionMan_StartOut(cTransitionMan* self, void* a1, int a2,
/// float f)` (0x2e44f0): while idle, start the "out" transition (0.0, 0.0
/// time); otherwise record the duration and target and report success.
pub fn c_transition_man_start_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V0][0] = Addr(this).off(0x44).lw(c);
        let idle = c.g[V0][0] == 0;
        c.mov_s(14, 12);
        if idle {
            c.cycles += 7;
            c.mtc1(0, 12);
            c.g[A3][0] = c.g[A2][0];
            c.g[A2][0] = 0;
            c.g[T0][0] = 0;
            c.mov_s(13, 12);
            c.g[T1][0] = 0;
            callee::C_TRANSITION_MAN_START.call(c, 0x002e_4530)?;
        } else {
            c.cycles += 4;
            c.swc1(14, p_2(this, 0x5c));
            c.g[V0][0] = 1;
            Addr(this).off(0x50).sw(c, c.g[A2][0]);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void SnowFall_ResetBoxes(self)` (0x2e62c8): reset the twelve snow boxes.
pub fn snow_fall_reset_boxes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(0x10));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xb;
        c.g[V0][0] = c.lw(r32(c, S0));
        loop {
            c.cycles += 4;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !empty {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002e_6300);
                callee::SNOW_BOX_RESET.call(c, 0x002e_6300)?;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[V0][0] = c.lw(r32(c, S0));
            if event_due(c) {
                deliver_event_here(c, 0x002e_62e8);
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


// ---------------------------------------------------------------------------
// Lens FX (graphics/lensfx.cpp).

/// `void LensFXBase_PushRenderState(LensFXBase* self)` (0x2e3578): push a
/// 0x14-byte copy of the render state (`+0x0..+0x13` of the block at
/// `gp+0x2a90 -> +0xe84`) onto the same stack, copy the default block from
/// `0x500000+0x1420`, then set the render-control bits (GS blend/alpha/test
/// fields at `+0x4`/`+0x8`) as the fixed point pipeline expects.
pub fn lens_fx_base_push_render_state(c: &mut Cpu) {
    c.cycles += 60;
    c.g[A0][0] = Addr(gpa(c, 0x2a90)).lw(c);
    c.g[T0][0] = 0x500000;
    c.g[A2][0] = 0xffff_ffff_ffbf_0000;
    c.g[T1][0] = 0x400000;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[A2][0] |= 0xffff;
    c.g[T2][0] = 0xffff_ffff_ffff_fffc;
    c.g[A3][0] = 0xffff_ffff_e000_0000;
    c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
    c.ldr(3, c.g[V0][0] as u32);
    c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
    c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[T3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
    c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x1b));
    c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x14));
    c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x23));
    c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x1c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[T3][0]);
    c.g[A3][0] |= 0x3ff;
    c.g[T3][0] = 0xffff_ffff_ffff_ff83;
    c.g[T4][0] = 0xffff_ffff_ffff_fc1f;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
    c.sw((c.g[A0][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1420));
    c.ldl(13, (c.g[A1][0] as u32).wrapping_add(0x7));
    c.ldr(13, c.g[A1][0] as u32);
    c.ldl(14, (c.g[A1][0] as u32).wrapping_add(0xf));
    c.ldr(14, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
    c.sdl(13, (c.g[V1][0] as u32).wrapping_add(0x1b));
    c.sdr(13, (c.g[V1][0] as u32).wrapping_add(0x14));
    c.sdl(14, (c.g[V1][0] as u32).wrapping_add(0x23));
    c.sdr(14, (c.g[V1][0] as u32).wrapping_add(0x1c));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] &= c.g[A2][0];
    c.g[V0][0] |= c.g[T1][0];
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] &= c.g[T2][0];
    c.g[V0][0] |= 0x2;
    c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] &= c.g[A3][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] &= c.g[T3][0];
    c.g[V0][0] |= 0x1c;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] &= c.g[T4][0];
    c.g[V0][0] |= 0xe0;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    abi::ret(c);
}


/// `void LensFXBase_PopRenderState(LensFXBase* self)` (0x2e3668): pop the
/// saved render-state block (`gp-0x854 -> +0xe84`) by 0x14 bytes.
pub fn lens_fx_base_pop_render_state(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = Addr(gpa(c, -0x854)).lw(c);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
    c.sw((c.g[V1][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
    abi::ret(c);
}


/// `int cTransitionMan_ScriptIsRunning(cTransitionMan* self)` (0x2e4c90):
/// a script is running when the current script pointer (`+0x60`) is set and
/// its step counter (`+0x64`) is non-zero.
pub fn c_transition_man_script_is_running(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x60)).lw(c);
    c.g[V1][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x64)).lw(c);
        c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}


/// `lw_2` (sign-extended).
#[inline(always)]
pub(crate) fn lw_2(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}


/// `lh_2` (sign-extended).
#[inline(always)]
pub(crate) fn lh_2(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lh(c)
}


/// The low 32 bits of a GPR.
#[inline(always)]
pub(crate) fn r32_2(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}


/// `sw`.
#[inline(always)]
pub(crate) fn sw(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sw(c, v);
}


/// `sw (reg + off), v`.
#[inline(always)]
pub(crate) fn sw_at(c: &mut Cpu, base: usize, off: u32, v: u64) {
    let a = p(r32_2(c, base), off);
    Addr(a).sw(c, v);
}


/// `sw (gp + off), v`.
#[inline(always)]
pub(crate) fn sw_gp(c: &mut Cpu, off: i32, v: u64) {
    let a = gp(c, off);
    Addr(a).sw(c, v);
}


/// `sp + off`.
#[inline(always)]
pub(crate) fn at_3(c: &Cpu, off: u32) -> u32 {
    p(sp(c), off)
}


/// A `gp`-relative `.lit4` float moved into `fN` (`lwc1`).
#[inline(always)]
pub(crate) fn ldf_4(c: &mut Cpu, n: usize, off: i32) {
    let a = gp(c, off);
    c.lwc1(n, a);
}


/// The emitter epilogue: `s0` is already back; `s1`, `s2`, `ra`, `f20` and the
/// frame follow.
pub(crate) fn emitter_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0xe0);
    frame.restore(c, S2, 0xd0);
    frame.restore_ra(c, 0xc0);
    frame.restore_fpr(c, 20, 0x100);
    frame.close(c);
}


/// `void cDynamicSprayFX_UpdateEmitter3(cDynamicSprayFX* fx)` (0x002dfe88):
/// advance the spawn timer; when it expires, scatter the spawn velocity,
/// shorten the interval (clamped), build the spawn volume and hand it to the
/// emitter's spawn call. Without a live emitter (or while the interval has not
/// expired) the emitter is told to spawn with the default volume instead.
pub fn dynamic_spray_update_emitter3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0xf0);
        frame.save(c, S2, 0xd0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0xe0);
        frame.save_ra(c, 0xc0);
        frame.save_fpr(c, 20, 0x100);
        c.g[V0][0] = lw_2(c, p(r32_2(c, S0), spray::EMITTER_2));
        c.g[V1][0] = lw_2(c, p(r32_2(c, S0), spray::INTERVAL));
        let idle = c.g[V1][0] == 0;
        c.g[S2][0] = sx(p(r32_2(c, V0), 0x630));
        if !idle {
            // Random state advance and the shortened spawn interval.
            c.cycles += 23;
            c.g[V1][0] = lw_2(c, gp(c, spray::RANDOM_GP));
            c.g[V0][0] = 0x0001_0000;
            c.g[V0][0] = sx(r32_2(c, V0) | 0x8fcd);
            c.g[A0][0] = 0x00e9_0000;
            c.mult(0, 3, 3, 2);
            c.g[A0][0] = sx(r32_2(c, A0) | 0x507c);
            c.g[V0][0] = 0x007f_0000;
            c.g[A1][0] = lw_2(c, p(r32_2(c, S0), 0xd0));
            c.g[V0][0] = sx(r32_2(c, V0) | 0xffff);
            c.g[A2][0] = 0x3f80_0000;
            c.lwc1(1, p(r32_2(c, A1), 0x6c));
            {
                let (a, b) = (r32_2(c, V1), r32_2(c, A0));
                c.g[V1][0] = sx(a.wrapping_add(b));
            }
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 20);
            {
                let (a, b) = (c.g[V1][0], c.g[V0][0]);
                c.g[V1][0] = a & b;
            }
            {
                let (a, b) = (c.g[V1][0], c.g[A2][0]);
                c.g[V1][0] = a | b;
            }
            let seed = c.g[V1][0];
            sw_gp(c, spray::RANDOM_GP, seed);
            c.lwc1(0, gp(c, spray::RANDOM_GP));
            c.sub_s(0, 0, 20);
            c.c_lt_s(0, 1);
            let wait = !c.fcc();
            c.g[A0][0] = c.g[S0][0];
            if !wait {
                // Scatter the new particle's velocity and build its volume.
                c.cycles += 36;
                c.g[AT][0] = 0x4170_0000;
                c.mtc1(1, 0);
                c.mfc1(2, 0);
                c.lqc2(4, p(r32_2(c, S0), 0x30));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_3(c, 0x20));
                c.g[A1][0] = c.g[SP][0];
                c.lq(2, at_3(c, 0x20));
                c.sq(2, at_3(c, 0x10));
                c.lqc2(3, p(r32_2(c, S0), 0xa0));
                c.lqc2(4, at_3(c, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, 0x20));
                c.lq(2, at_3(c, 0x20));
                c.sq(2, at_3(c, 0));
                spray_callee::ADD_RANDOM_VELOCITY.call(c, 0x002d_ff54)?;
                // How far the particle is from the emitter centre.
                c.g[V1][0] = lw_2(c, r32_2(c, S0));
                c.mov_s(6, 20);
                c.lqc2(3, p(r32_2(c, V1), 0x3b0));
                c.lqc2(5, p(r32_2(c, S0), 0xc0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.g[AT][0] = 0x3fc0_0000;
                c.mtc1(1, 1);
                c.abs_s(0, 0);
                c.mul_s(0, 0, 1);
                c.c_lt_s(0, 20);
                if c.fcc() {
                    c.mov_s(6, 0);
                }

                // Spawn interval: blend towards the minimum while close.
                c.cycles += 19;
                ldf_4(c, 2, -0x3b7c);
                c.mov_s(5, 20);
                c.lwc1(1, p(r32_2(c, V1), 0x214));
                c.abs_s(1, 1);
                ldf_4(c, 3, -0x3b78);
                c.sub_s(1, 1, 2);
                c.lwc1(0, p(r32_2(c, S0), 0xb4));
                c.g[AT][0] = 0x4040_0000;
                c.mtc1(1, 2);
                c.abs_s(0, 0);
                c.sub_s(0, 0, 3);
                ldf_4(c, 4, -0x3b74);
                c.mul_s(1, 1, 2);
                c.mtc1(0, 3);
                c.mul_s(0, 0, 4);
                c.c_le_s(3, 1);
                c.mov_s(4, 3);
                let t = !c.fcc();
                c.add_s(2, 0, 6);
                if !t {
                    c.cycles += 1;
                    c.min_s(4, 1, 5);
                }
                c.cycles += 5;
                ldf_4(c, 0, -0x3b70);
                c.c_le_s(3, 2);
                c.mov_s(1, 3);
                let t = !c.fcc();
                c.mul_s(0, 4, 0);
                if !t {
                    c.cycles += 1;
                    c.min_s(1, 2, 5);
                }
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if !c.fcc() {
                    c.mov_s(3, 0);
                } else {
                    c.cycles += 4;
                    c.c_le_s(3, 2);
                    let t = !c.fcc();
                    c.mov_s(0, 3);
                    if !t {
                        c.cycles += 1;
                        c.min_s(0, 2, 20);
                    }
                    c.cycles += 1;
                    c.mov_s(3, 0);
                }
                c.cycles += 15;
                ldf_4(c, 1, -0x3b6c);
                c.lwc1(0, p(r32_2(c, S0), 0xb8));
                c.g[AT][0] = 0x41a0_0000;
                c.mtc1(1, 2);
                c.mul_s(0, 0, 1);
                ldf_4(c, 20, -0x3b68);
                c.mtc1(0, 4);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 1);
                c.sub_s(0, 0, 2);
                c.mul_s(0, 0, 20);
                c.c_le_s(4, 0);
                let t = !c.fcc();
                c.mov_s(2, 4);
                if !t {
                    c.cycles += 1;
                    c.min_s(2, 0, 1);
                }
                // The emitter's colour ramp random state.
                c.cycles += 20;
                c.g[V1][0] = lw_2(c, gp(c, spray::RANDOM_GP));
                c.g[V0][0] = 0x0001_0000;
                c.g[V0][0] = sx(r32_2(c, V0) | 0x8fcd);
                c.g[A0][0] = 0x00e9_0000;
                c.mult(0, 3, 3, 2);
                c.g[A0][0] = sx(r32_2(c, A0) | 0x507c);
                c.g[V0][0] = 0x007f_0000;
                c.g[A1][0] = 0x3f80_0000;
                c.g[V0][0] = sx(r32_2(c, V0) | 0xffff);
                c.mul_s(3, 3, 2);
                {
                    let (a, b) = (r32_2(c, V1), r32_2(c, A0));
                    c.g[V1][0] = sx(a.wrapping_add(b));
                }
                {
                    let (a, b) = (c.g[V1][0], c.g[V0][0]);
                    c.g[V1][0] = a & b;
                }
                {
                    let (a, b) = (c.g[V1][0], c.g[A1][0]);
                    c.g[V1][0] = a | b;
                }
                let seed = c.g[V1][0];
            sw_gp(c, spray::RANDOM_GP, seed);
                c.lwc1(0, gp(c, spray::RANDOM_GP));
                c.sub_s(0, 0, 1);
                c.c_lt_s(0, 3);
                let t = c.fcc();
                c.g[S1][0] = 1;
                if !t {
                    c.cycles += 1;
                    c.g[S1][0] = 0;
                }

                // Spawn volume: the emitter's axis and the size fields.
                c.cycles += 13;
                c.g[V0][0] = lw_2(c, r32_2(c, S0));
                c.lqc2(4, p(r32_2(c, S0), 0x50));
                c.lqc2(5, p(r32_2(c, V0), 0x370));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, at_3(c, 0x20));
                c.lwc1(0, p(r32_2(c, S0), 0xb4));
                c.lq(2, at_3(c, 0x20));
                c.c_lt_s(0, 4);
                let t = !c.fcc();
                c.sq(2, at_3(c, 0x10));
                if !t {
                    c.cycles += 10;
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(1, 0);
                    c.mfc1(3, 0);
                    c.lqc2(4, at_3(c, 0x10));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at_3(c, 0x30));
                    c.lq(2, at_3(c, 0x30));
                    c.sq(2, at_3(c, 0x10));
                    c.sq(2, at_3(c, 0x20));
                }

                // Hand the volume to the emitter.
                c.cycles += 88;
                c.g[A0][0] = lw_2(c, r32_2(c, S0));
                c.g[V1][0] = lw_2(c, p(r32_2(c, A0), 0x6c0));
                c.g[A0][0] = sx(p(r32_2(c, A0), 0x6c0));
                c.g[A1][0] = lh_2(c, p(r32_2(c, V1), 0x10));
                c.g[V0][0] = lw_2(c, p(r32_2(c, V1), 0x14));
                let target = r32_2(c, V0);
                c.g[A0][0] = sx(p(r32_2(c, A0), r32_2(c, A1)));
                call_ptr(c, target, 0x002e_013c)?;
                c.g[V1][0] = lw_2(c, p(r32_2(c, S0), 0x2c));
                c.g[A0][0] = c.g[S2][0];
                c.g[A3][0] = sx(p(r32_2(c, S0), 0x90));
                c.g[T0][0] = c.g[S1][0];
                c.lwc1(0, p(r32_2(c, V1), 0x390));
                c.mov_s(12, 20);
                c.mfc1(3, 0);
                c.lqc2(4, r32_2(c, V0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_3(c, 0x60));
                c.g[A1][0] = c.g[SP][0];
                c.lq(2, at_3(c, 0x60));
                c.g[A2][0] = sx(p(r32_2(c, SP), 0x20));
                c.g[T1][0] = lw_2(c, p(r32_2(c, S0), 0x2c));
                c.sq(2, at_3(c, 0x50));
                c.lwc1(0, p(r32_2(c, T1), 0x394));
                c.g[V1][0] = lw_2(c, r32_2(c, S0));
                c.mfc1(10, 0);
                c.lqc2(4, p(r32_2(c, V1), 0x370));
                c.qmtc2(10, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_3(c, 0x70));
                c.lq(2, at_3(c, 0x70));
                c.sq(2, at_3(c, 0x60));
                c.lqc2(3, at_3(c, 0x50));
                c.lqc2(4, at_3(c, 0x60));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, 0x70));
                c.lq(2, at_3(c, 0x70));
                c.g[T1][0] = lw_2(c, p(r32_2(c, S0), 0x2c));
                c.sq(2, at_3(c, 0x40));
                c.lwc1(0, p(r32_2(c, T1), 0x398));
                c.g[V1][0] = lw_2(c, r32_2(c, S0));
                c.mfc1(10, 0);
                c.lqc2(4, p(r32_2(c, V1), 0x370));
                c.qmtc2(10, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_3(c, 0x90));
                c.lq(2, at_3(c, 0x90));
                c.lwc1(0, p(r32_2(c, S0), 0xbc));
                c.sq(2, at_3(c, 0x80));
                c.mfc1(3, 0);
                c.lqc2(4, at_3(c, 0x80));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_3(c, 0x90));
                c.lq(2, at_3(c, 0x90));
                c.sq(2, at_3(c, 0x70));
                c.lqc2(3, at_3(c, 0x40));
                c.lqc2(4, at_3(c, 0x70));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, 0x90));
                c.lq(2, at_3(c, 0x90));
                c.g[V1][0] = lw_2(c, p(r32_2(c, S0), 0x2c));
                c.sq(2, at_3(c, 0x30));
                c.lwc1(0, p(r32_2(c, V1), 0x39c));
                c.mfc1(9, 0);
                c.lqc2(4, at_3(c, 0x10));
                c.qmtc2(9, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_3(c, 0xb0));
                c.lq(2, at_3(c, 0xb0));
                c.lwc1(0, p(r32_2(c, S0), 0xbc));
                c.sq(2, at_3(c, 0xa0));
                c.mfc1(3, 0);
                c.lqc2(4, at_3(c, 0xa0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_3(c, 0xb0));
                c.lq(2, at_3(c, 0xb0));
                c.sq(2, at_3(c, 0x90));
                c.lqc2(3, at_3(c, 0x30));
                c.lqc2(4, at_3(c, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, 0xb0));
                c.lq(2, at_3(c, 0xb0));
                c.sq(2, at_3(c, 0x20));
                spray_callee::SPAWN_AT.call(c, 0x002e_0278)?;
                frame.restore(c, S0, 0xf0);
                emitter_epilogue(c, frame);
                return Ok(());
            }
        }

        // No new interval: spawn with the default volume.
        c.cycles += 8;
        ldf_4(c, 12, -0x3b64);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p(r32_2(c, S0), 0xa0));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        spray_callee::SPAWN_AT.call(c, 0x002e_029c)?;
        frame.restore(c, S0, 0xf0);
        emitter_epilogue(c, frame);
        Ok(())
    })
}


/// `cDynamicSprayFX_UpdateEmitter4`'s epilogue.
pub(crate) fn emitter4_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 8;
    frame.restore(c, S1, 0xf0);
    frame.restore(c, S2, 0xe0);
    frame.restore_ra(c, 0xd0);
    frame.restore_fpr(c, 22, 0x120);
    frame.restore_fpr(c, 21, 0x118);
    frame.restore_fpr(c, 20, 0x110);
    frame.close(c);
}


/// The default spawn from the "wobble timer not reached" path (0x2e1530).
pub(crate) fn emitter4_spawn_mid(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 9;
    c.g[A0][0] = c.g[S1][0];
    ldf_4(c, 12, -0x3ac4);
    c.g[A1][0] = sx(p(r32_2(c, SP), 0x40));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    spray_callee::SPAWN_AT.call(c, 0x002e_154c)?;
    frame.restore(c, S0, 0x100);
    emitter4_epilogue(c, frame);
    Ok(())
}


/// The default spawn once the wobble timer is spent (0x2e1554): also stop the
/// emitter's rate timer.
pub(crate) fn emitter4_spawn_late(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 9;
    sw_at(c, S0, 0x14, 0);
    c.g[A0][0] = c.g[S1][0];
    ldf_4(c, 12, -0x3ac0);
    c.g[A1][0] = sx(p(r32_2(c, SP), 0x40));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    spray_callee::SPAWN_AT.call(c, 0x002e_1574)?;
    frame.restore(c, S0, 0x100);
    emitter4_epilogue(c, frame);
    Ok(())
}


/// `void cDynamicSprayFX_UpdateEmitter4(cDynamicSprayFX* fx)` (0x002e1120):
/// the board-wake / weather-driven emitter. Copies the emitter's placement
/// matrix, advances the wobble and rate timers, gets the current weather and
/// derives the particle scale from it, then hands a spawn volume to the
/// emitter (or tells it to spawn with the default volume while the timers
/// have not expired).
pub fn dynamic_spray_update_emitter4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 52;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x100);
        frame.save(c, S1, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0xe0);
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 22, 0x120);
        frame.save_fpr(c, 21, 0x118);
        frame.save_fpr(c, 20, 0x110);
        c.g[V1][0] = lw_2(c, r32_2(c, S0));
        c.g[A0][0] = lw_2(c, p(r32_2(c, S0), spray::EMITTER_2));
        c.g[A1][0] = lw_2(c, p(r32_2(c, V1), 0x780));
        c.g[V0][0] = lw_2(c, p(r32_2(c, V1), 0x8a8));
        c.g[S1][0] = sx(p(r32_2(c, A0), 0x840));
        c.g[V1][0] = lw_2(c, p(r32_2(c, A1), 0x30));
        c.g[V0][0] = sx(r32_2(c, V0) << 6);
        c.g[V0][0] = sx(p(r32_2(c, V0), r32_2(c, V1)));
        c.lqc2(1, r32_2(c, V0));
        c.lqc2(2, p(r32_2(c, V0), 0x10));
        c.lqc2(3, p(r32_2(c, V0), 0x20));
        c.lqc2(4, p(r32_2(c, V0), 0x30));
        c.sqc2(1, sp(c));
        c.sqc2(2, at_3(c, 0x10));
        c.sqc2(3, at_3(c, 0x20));
        c.sqc2(4, at_3(c, 0x30));
        c.g[AT][0] = 0xffff_ffff_c040_0000;
        c.mtc1(1, 0);
        c.g[AT][0] = 0x4080_0000;
        c.mtc1(1, 1);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21);
        c.swc1(1, at_3(c, 0x58));
        c.swc1(0, at_3(c, 0x50));
        let v = p(sp(c), 0x54);
        sw(c, v, 0);
        c.swc1(21, at_3(c, 0x5c));
        c.lqc2(8, at_3(c, 0x50));
        c.lqc2(4, sp(c));
        c.lqc2(5, at_3(c, 0x10));
        c.lqc2(6, at_3(c, 0x20));
        c.lqc2(7, at_3(c, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, at_3(c, 0x60));
        c.lwc1(1, gp(c, -0x3ae8));
        c.lwc1(0, p(r32_2(c, S0), 0xbc));
        c.lq(2, at_3(c, 0x60));
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.sq(2, at_3(c, 0x40));
        if !t {
            c.cycles += 2;
            sw_at(c, S0, 0x14, 0);
            return emitter4_spawn_mid(c, frame);
        }

        // Layer-2 animation state: 0x1e keeps the scale at_3 one.
        c.cycles += 8;
        c.g[V0][0] = lw_2(c, r32_2(c, S0));
        c.g[A1][0] = 2;
        c.mtc1(0, 20);
        c.g[A0][0] = lw_2(c, p(r32_2(c, V0), 0x784));
        call_ptr(c, 0x0031_1ae8, 0x002e_120c)?;
        c.g[V1][0] = 0x1e;
        if c.g[V0][0] == c.g[V1][0] {
            c.mov_s(20, 21);
        }
        // Blend the animation's current time towards the target.
        c.cycles += 4;
        c.lwc1(1, p(r32_2(c, S0), 0x1c));
        c.c_le_s(1, 20);
        let above = !c.fcc();
        if above {
            c.cycles += 3;
            c.lwc1(0, gp(c, -0x3ae4));
            c.sub_s(0, 1, 0);
            c.swc1(0, p(r32_2(c, S0), 0x1c));
        } else {
            c.cycles += 2;
            c.swc1(20, p(r32_2(c, S0), 0x1c));
        }
        // Wobble timer: when it expires, toggle the wobble direction and
        // re-arm (or take the 0.4 s hold).
        c.cycles += 8;
        c.lwc1(0, p(r32_2(c, S0), 0x20));
        c.lwc1(1, gp(c, -0x3ae0));
        c.lwc1(2, p(r32_2(c, S0), 0x24));
        c.add_s(0, 0, 1);
        c.c_le_s(2, 0);
        let t = !c.fcc();
        c.swc1(0, p(r32_2(c, S0), 0x20));
        let mut wobble = false;
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw_2(c, p(r32_2(c, S0), 0x18));
            sw_at(c, S0, 0x20, 0);
            {
                let v = c.g[V0][0] ^ 1;
                c.g[V0][0] = v;
            }
            let turned_off = c.g[V0][0] == 0;
            let v = c.g[V0][0];
            sw_at(c, S0, 0x18, v);
            if turned_off {
                wobble = true;
            } else {
                c.cycles += 3;
                c.lwc1(0, gp(c, -0x3adc));
                c.swc1(0, p(r32_2(c, S0), 0x24));
            }
        }
        if wobble {
            c.cycles += 9;
            c.lwc1(2, p(r32_2(c, S0), 0x1c));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            ldf_4(c, 3, -0x3ad8);
            c.sub_s(0, 0, 2);
            ldf_4(c, 1, -0x3ad4);
            c.mul_s(0, 0, 3);
            c.add_s(0, 0, 1);
            c.swc1(0, p(r32_2(c, S0), 0x24));
        }

        // Weather-driven colour/scale.
        c.cycles += 29;
        c.g[A0][0] = lw_2(c, r32_2(c, S0));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
        c.g[A1][0] = lw_2(c, p(r32_2(c, A0), 0x6c0));
        c.g[A0][0] = sx(p(r32_2(c, A0), 0x6c0));
        c.mtc1(0, 22);
        c.g[V1][0] = lh_2(c, p(r32_2(c, A1), 0x38));
        c.g[V0][0] = lw_2(c, p(r32_2(c, A1), 0x3c));
        let target = r32_2(c, V0);
        c.g[A0][0] = sx(p(r32_2(c, A0), r32_2(c, V1)));
        call_ptr(c, target, 0x002e_12c8)?;
        c.g[A0][0] = c.g[V0][0];
        spray_callee::WORLD_PAINTER_GET_WEATHER.call(c, 0x002e_12d0)?;
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(1, 2);
        ldf_4(c, 3, -0x3ad0);
        c.add_s(0, 0, 2);
        c.lwc1(1, p(r32_2(c, S0), 0x1c));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 2);
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 5);
        c.mul_s(0, 0, 3);
        c.add_s(1, 1, 2);
        c.sub_s(0, 20, 0);
        c.mul_s(1, 1, 0);
        c.c_le_s(22, 1);
        let t = !c.fcc();
        c.mov_s(4, 22);
        if !t {
            c.cycles += 1;
            c.min_s(4, 1, 5);
        }

        // Spawn rate gate.
        c.cycles += 7;
        c.g[V0][0] = lw_2(c, p(r32_2(c, S0), 0x18));
        c.g[S2][0] = sx(p(r32_2(c, SP), 0x50));
        c.swc1(20, at_3(c, 0x50));
        c.swc1(20, at_3(c, 0x54));
        c.swc1(20, at_3(c, 0x58));
        let t = c.g[V0][0] == 0;
        c.swc1(4, at_3(c, 0x5c));
        if t {
            return emitter4_spawn_late(c, frame);
        }
        c.cycles += 5;
        c.lwc1(0, gp(c, -0x3acc));
        c.c_lt_s(0, 4);
        let t = !c.fcc();
        c.g[V1][0] = lw_2(c, gp(c, spray::RANDOM_GP));
        if t {
            return emitter4_spawn_late(c, frame);
        }

        // Spawn with the derived volume and scale.
        c.cycles += 28;
        c.g[V0][0] = 0x0001_0000;
        c.g[V0][0] = sx(r32_2(c, V0) | 0x8fcd);
        c.g[A0][0] = 0x00e9_0000;
        c.mult(0, 3, 3, 2);
        c.g[A0][0] = sx(r32_2(c, A0) | 0x507c);
        c.g[V0][0] = 0x007f_0000;
        c.g[A1][0] = 0x3f80_0000;
        c.g[V0][0] = sx(r32_2(c, V0) | 0xffff);
        c.lwc1(1, p(r32_2(c, S0), 0x14));
        c.g[AT][0] = 0x40c0_0000;
        c.mtc1(1, 2);
        {
            let (a, b) = (r32_2(c, V1), r32_2(c, A0));
            c.g[V1][0] = sx(a.wrapping_add(b));
        }
        c.g[AT][0] = 0x4190_0000;
        c.mtc1(1, 3);
        {
            let (a, b) = (c.g[V1][0], c.g[V0][0]);
            c.g[V1][0] = a & b;
        }
        ldf_4(c, 21, -0x3ac8);
        {
            let (a, b) = (c.g[V1][0], c.g[A1][0]);
            c.g[V1][0] = a | b;
        }
        let seed = c.g[V1][0];
            sw_gp(c, spray::RANDOM_GP, seed);
        c.lwc1(0, gp(c, spray::RANDOM_GP));
        c.sub_s(0, 0, 20);
        c.mul_s(0, 0, 2);
        c.add_s(0, 0, 3);
        c.mul_s(0, 0, 21);
        c.add_s(0, 1, 0);
        c.c_lt_s(20, 0);
        let t = !c.fcc();
        c.swc1(0, p(r32_2(c, S0), 0x14));
        if t {
            return emitter4_spawn_mid(c, frame);
        }

        c.cycles += 94;
        c.sub_s(0, 0, 20);
        c.g[V1][0] = 0x0050_0000;
        c.g[V1][0] = sx(r32_2(c, V1).wrapping_sub(0xed0));
        c.g[A0][0] = lw_2(c, r32_2(c, S0));
        c.swc1(0, p(r32_2(c, S0), 0x14));
        c.g[A1][0] = sx(p(r32_2(c, A0), 0x6c0));
        c.lq(2, r32_2(c, V1));
        c.sq(2, at_3(c, 0x30));
        c.g[V1][0] = lw_2(c, p(r32_2(c, A0), 0x6c0));
        c.g[A0][0] = lh_2(c, p(r32_2(c, V1), 0x10));
        c.g[V0][0] = lw_2(c, p(r32_2(c, V1), 0x14));
        let target = r32_2(c, V0);
        c.g[A0][0] = sx(p(r32_2(c, A1), r32_2(c, A0)));
        call_ptr(c, target, 0x002e_13ec)?;
        c.g[V1][0] = lw_2(c, p(r32_2(c, S0), 0x2c));
        c.g[A0][0] = c.g[S1][0];
        c.g[AT][0] = 0x4316_0000;
        c.mtc1(1, 1);
        c.g[A3][0] = c.g[S2][0];
        c.lwc1(0, p(r32_2(c, V1), 0x478));
        c.mov_s(12, 21);
        c.mfc1(3, 0);
        c.lqc2(4, r32_2(c, V0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at_3(c, 0x90));
        c.g[A1][0] = sx(p(r32_2(c, SP), 0x40));
        c.lq(2, at_3(c, 0x90));
        c.g[A2][0] = sx(p(r32_2(c, SP), 0x60));
        c.g[T1][0] = lw_2(c, p(r32_2(c, S0), 0x2c));
        c.g[T0][0] = 1;
        c.sq(2, at_3(c, 0x80));
        c.lwc1(0, p(r32_2(c, T1), 0x47c));
        c.g[V1][0] = lw_2(c, r32_2(c, S0));
        c.mfc1(10, 0);
        c.lqc2(4, p(r32_2(c, V1), 0x370));
        c.qmtc2(10, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at_3(c, 0xa0));
        c.lq(2, at_3(c, 0xa0));
        c.sq(2, at_3(c, 0x90));
        c.lqc2(3, at_3(c, 0x80));
        c.lqc2(4, at_3(c, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at_3(c, 0xa0));
        c.lq(2, at_3(c, 0xa0));
        c.g[T1][0] = lw_2(c, p(r32_2(c, S0), 0x2c));
        c.sq(2, at_3(c, 0x70));
        c.lwc1(0, p(r32_2(c, T1), 0x480));
        c.g[V1][0] = lw_2(c, r32_2(c, S0));
        c.mfc1(10, 0);
        c.lqc2(4, p(r32_2(c, V1), 0x370));
        c.qmtc2(10, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at_3(c, 0xc0));
        c.lq(2, at_3(c, 0xc0));
        c.lwc1(0, p(r32_2(c, S0), 0xbc));
        c.sq(2, at_3(c, 0xb0));
        c.mfc1(3, 0);
        c.lqc2(4, at_3(c, 0xb0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at_3(c, 0xc0));
        c.lq(2, at_3(c, 0xc0));
        c.sq(2, at_3(c, 0xa0));
        c.lqc2(3, at_3(c, 0x70));
        c.lqc2(4, at_3(c, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at_3(c, 0xc0));
        c.lq(2, at_3(c, 0xc0));
        c.swc1(22, at_3(c, 0x84));
        c.sq(2, at_3(c, 0x60));
        c.swc1(1, at_3(c, 0x88));
        c.swc1(20, at_3(c, 0x8c));
        c.swc1(22, at_3(c, 0x80));
        c.lqc2(8, at_3(c, 0x80));
        c.lqc2(4, sp(c));
        c.lqc2(5, at_3(c, 0x10));
        c.lqc2(6, at_3(c, 0x20));
        c.lqc2(7, at_3(c, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, at_3(c, 0x90));
        c.lq(2, at_3(c, 0x90));
        c.sq(2, at_3(c, 0x70));
        c.lqc2(3, at_3(c, 0x60));
        c.lqc2(4, at_3(c, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at_3(c, 0x60));
        spray_callee::SPAWN_AT.call(c, 0x002e_1528)?;
        frame.restore(c, S0, 0x100);
        emitter4_epilogue(c, frame);
        Ok(())
    })
}


/// `void LensFXBase_dtor(LensFXBase* self, int flags)` (0x2e3060): unhook the
/// lens FX from the global render list, clear the pending-effect word and
/// optionally free the object.
pub fn lens_fx_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x4886_b0;
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A3][0] = Addr(gpr(c, 0x2a90)).lw(c);
        c.g[A1][0] = 0;
        Addr(at_4(c, S1, 0x649c)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0;
        c.g[V0][0] = Addr(at_4(c, A3, 0x10d8)).lw(c);
        c.g[A0][0] = Addr(at_4(c, V0, 0x3a8)).lh(c);
        c.g[V1][0] = Addr(at_4(c, V0, 0x3ac)).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at_4(c, A3, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x002e_30a4)?;

        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        Addr(gpr(c, 0xa28)).sw(c, 0);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002e_30b8)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `LensFX_GetFlareSizeForRes` (0x2E27E8): the lens flare size for a render
/// resolution type (100/350/180, 0 for unknown).
pub fn lens_fx_get_flare_size_for_res(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x20;
    let is_32 = c.g[A0][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[A0][0] as i64) < 33) as u64;
    if is_32 {
        c.cycles += 4;
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 0);
        abi::ret(c);
        return;
    }
    // 0: the >= 33 arm (0x40 test), 1: 180 (A0 == 0x10), 2: zero.
    let mut arm = 2;
    c.cycles += 2;
    let ge33 = c.g[V0][0] == 0;
    c.g[V0][0] = 0x10;
    if ge33 {
        arm = 0;
    } else {
        c.cycles += 2;
        let is_16 = c.g[A0][0] == c.g[V0][0];
        if is_16 {
            arm = 1;
        } else {
            c.cycles += 2;
        }
    }
    if arm == 1 {
        c.cycles += 4;
        c.g[AT][0] = 0x4334_0000;
        c.mtc1(AT, 0);
        abi::ret(c);
        return;
    }
    if arm == 0 {
        c.cycles += 3;
        c.g[V0][0] = 0x40;
        let is_64 = c.g[A0][0] == c.g[V0][0];
        if is_64 {
            c.cycles += 4;
            c.g[AT][0] = 0x43af_0000;
            c.mtc1(AT, 0);
            abi::ret(c);
            return;
        }
        c.cycles += 2;
    }
    c.cycles += 3;
    c.mtc1(ZERO, 0);
    abi::ret(c);
}


/// `void cDynamicSprayFX_SetupEmitters(cDynamicSprayFX* self)` (0x2df448):
/// initialise nine spray emitters from the effect's emitter list.
pub fn c_dynamic_spray_fx_setup_emitters(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        open(c, 0x50);
        let sp0 = spv(c);
        c.sq(16, at_5(sp0, 0x40));
        c.sq(17, at_5(sp0, 0x30));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, at_5(sp0, 0x20));
        c.g[S1][0] = 9;
        c.sq(19, at_5(sp0, 0x10));
        c.g[S2][0] = 0;
        c.sd(sp0, c.g[RA][0]);
        c.g[S3][0] = 0;
        let this = c.g[S0][0];
        c.g[A0][0] = lw_at(c, this, 0x28);
        loop {
            c.cycles += 14;
            c.g[S1][0] = sx(at_5(c.g[S1][0] as u32, 0xffff_ffff));
            c.g[A1][0] = lw_at(c, this, 0x2c);
            c.g[A0][0] = sx(at_5(c.g[S2][0] as u32, c.g[A0][0] as u32));
            let rec = c.g[A0][0];
            c.g[A2][0] = lw_at(c, rec, 0x1f8);
            c.g[A1][0] = sx(at_5(c.g[A1][0] as u32, c.g[S3][0] as u32));
            c.lwc1(12, at_5(rec as u32, 0x58));
            c.g[S3][0] = sx(at_5(c.g[S3][0] as u32, 0xe8));
            c.g[V1][0] = lh_at(c, c.g[A2][0], 0x18);
            c.g[S2][0] = sx(at_5(c.g[S2][0] as u32, 0x210));
            c.g[V0][0] = lw_at(c, c.g[A2][0], 0x1c);
            let func = c.g[V0][0] as u32;
            c.g[A0][0] = sx(at_5(rec as u32, c.g[V1][0] as u32));
            call_ptr(c, func, 0x002d_f4a8)?;
            if (c.g[S1][0] as i64) < 0 {
                c.cycles += 8;
                c.sw(at_5(this as u32, 0x60), 0);
                c.lq(16, at_5(sp0, 0x40));
                c.lq(17, at_5(sp0, 0x30));
                c.lq(18, at_5(sp0, 0x20));
                c.lq(19, at_5(sp0, 0x10));
                c.g[RA][0] = c.ld(sp0);
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at_5(sp0, 0x50));
                c.pc = ra;
                return Ok(());
            }
            c.g[A0][0] = lw_at(c, this, 0x28);
            if event_due(c) {
                deliver_event_here(c, 0x002d_f478);
            }
        }
    })
}


/// `void SnowFall_dtor(SnowFall* self, int flags)` (0x2e5d18): destroy the
/// eleven snow boxes and the base process.
pub fn snow_fall_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        open(c, 0x50);
        let sp0 = spv(c);
        c.g[V0][0] = 0x480000;
        c.sq(18, at_5(sp0, 0x20));
        c.g[V0][0] = sx(at_5(c.g[V0][0] as u32, 0x7fd8));
        c.sq(16, at_5(sp0, 0x40));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, at_5(sp0, 0x30));
        c.g[S0][0] = sx(at_5(c.g[S2][0] as u32, 0x10));
        c.sq(19, at_5(sp0, 0x10));
        c.g[S1][0] = 0xb;
        c.sd(sp0, c.g[RA][0]);
        c.g[S3][0] = c.g[A1][0];
        c.sw(at_5(c.g[S2][0] as u32, 0xc), c.g[V0][0]);
        call_ptr(c, callee::MEM_REPORT_STATS_LOCKED, 0x002e_5d50)?;
        loop {
            c.cycles += 3;
            c.g[A0][0] = lw_at(c, c.g[S0][0], 0);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = 3;
                callee::SNOW_BOX_DTOR.call(c, 0x002e_5d64)?;
            }
            c.g[S1][0] = sx(at_5(c.g[S1][0] as u32, 0xffff_ffff));
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(at_5(c.g[S0][0] as u32, 4));
            if !more {
                c.cycles += 12;
                call_ptr(c, callee::MEM_REPORT_STATS_LOCKED, 0x002e_5d78)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                callee::GAME_PROCESS_DTOR.call(c, 0x002e_5d84)?;
                c.lq(16, at_5(sp0, 0x40));
                c.lq(17, at_5(sp0, 0x30));
                c.lq(18, at_5(sp0, 0x20));
                c.lq(19, at_5(sp0, 0x10));
                c.g[RA][0] = c.ld(sp0);
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at_5(sp0, 0x50));
                c.pc = ra;
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_5d50);
            }
        }
    })
}


/// `cDynamicSprayFX_Reset` (0x2df3b0): clear the spray burst state, reset the
/// emitter colour/scale, and reconfigure for the current rider if needed.
pub fn dynamic_spray_fx_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x30);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        frame.save(c, S0, 0x20);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 1);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.swc1(0, p(r(c, SP), 0xc));
        c.swc1(0, r(c, SP));
        c.swc1(0, p(r(c, SP), 4));
        c.swc1(0, p(r(c, SP), 8));
        c.sd(p(r(c, SP), 0x10), c.g[RA][0]);
        c.swc1(1, p(this, 0x120));
        c.g[V1][0] = Addr(this).lw(c);
        c.lq(2, r(c, SP));
        Addr(p(this, 0x10)).sw(c, 0);
        c.sq(2, p(this, 0x90));
        Addr(p(this, 0x74)).sw(c, 0);
        Addr(p(this, 4)).sw(c, 0);
        Addr(p(this, 0x1c)).sw(c, 0);
        Addr(p(this, 0x80)).sw(c, 0);
        Addr(p(this, 0x78)).sw(c, 0);
        Addr(p(this, 0xe0)).sw(c, 0);
        Addr(p(this, 0x12c)).sw(c, 0);
        c.g[A0][0] = Addr(p(this, 0x124)).lw(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0x870)).lw(c);
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        c.g[V0][0] = sx(r(c, V0) >> 31);
        let same = c.g[A0][0] == c.g[V0][0];
        if !same {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002d_f430);
            c.g[A0][0] = c.g[S0][0];
            callee::SPRAY_CONFIGURE_FOR_RIDER.call(c, 0x002d_f430)?;
        }
        c.cycles += 6;
        c.g[RA][0] = sx(0x002d_f438);
        c.g[A0][0] = c.g[S0][0];
        callee::SPRAY_UPDATE_EMITTER_ENABLES.call(c, 0x002d_f438)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Transitions.

/// `void cTransitionMan_dtor(TransitionMan* self, int flags)` (0x2e42d0): stop
/// the transition, release the pending effect through the global vtable, then
/// unregister and destroy the embedded script object and the game process.
pub fn c_transition_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x30);
        c.g[V1][0] = 0x0049_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f00));
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7ea8));
        sw_2(c, p(c.g[S0][0] as u32, 0x1c), c.g[V0][0]);
        c.g[S2][0] = c.g[A1][0];
        c.sw(p(c.g[S0][0] as u32, 0xc), c.g[V1][0]);
        call_ptr(c, 0x002e_4578, 0x002e_4308)?; // cTransitionMan_Stop
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x7c));
        let no_effect = c.g[V0][0] == 0;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        if !no_effect {
            c.cycles += 8;
            c.g[A2][0] = lw(c, gp(c, 0x2a90));
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x68));
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x1e0)).lh(c);
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x1e4));
            c.g[V1][0] = sx(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x002e_4330)?;
            sw_2(c, p(c.g[S0][0] as u32, 0x7c), 0);
        }
        c.cycles += 14;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0028_32d8, 0x002e_433c)?; // ScriptObj_Unregister
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_3228, 0x002e_4348)?; // ScriptObj_dtor
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0035_46c8, 0x002e_4354)?; // GameProcess_dtor
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cPlayerPulseFX_Reset(self)` (0x2e3930): reset the pulse effect's
/// four vectors (wake vector 0.5/0/0/0, tint 0.7 x3, then the two copies).
pub fn c_player_pulse_fx_reset(c: &mut Cpu) {
    c.cycles += 41;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(1, 0);
    c.swc1(1, p(sp(c), 4));
    c.swc1(0, sp(c));
    Addr(p(sp(c), 8)).sw(c, 0);
    Addr(p(sp(c), 0xc)).sw(c, 0);
    c.lwc1(0, gp(c, -0x3a34));
    c.ldl(2, p(sp(c), 7));
    c.ldr(2, sp(c));
    c.ldl(3, p(sp(c), 0xf));
    c.ldr(3, p(sp(c), 8));
    let obj = r32(c, A0);
    c.sdl(2, p(obj, 0xf));
    c.sdr(2, p(obj, 8));
    c.sdl(3, p(obj, 0x17));
    c.sdr(3, p(obj, 0x10));
    c.swc1(1, p(sp(c), 4));
    c.swc1(0, p(sp(c), 0xc));
    c.swc1(0, sp(c));
    c.swc1(0, p(sp(c), 8));
    c.ldl(2, p(sp(c), 7));
    c.ldr(2, sp(c));
    c.ldl(3, p(sp(c), 0xf));
    c.ldr(3, p(sp(c), 8));
    c.sdl(2, p(obj, 0x1f));
    c.sdr(2, p(obj, 0x18));
    c.sdl(3, p(obj, 0x27));
    c.sdr(3, p(obj, 0x20));
    c.ldl(2, p(obj, 0xf));
    c.ldr(2, p(obj, 8));
    c.ldl(3, p(obj, 0x17));
    c.ldr(3, p(obj, 0x10));
    c.sdl(2, p(obj, 0x2f));
    c.sdr(2, p(obj, 0x28));
    c.sdl(3, p(obj, 0x37));
    c.sdr(3, p(obj, 0x30));
    Addr(p(obj, 4)).sw(c, 0);
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    abi::ret(c);
}


/// `cTransitionMan* cTransitionMan_ctor(self, a1, a2)` (0x2e4228): build the
/// transition manager (effect pool, effect list, default fields).
pub fn c_transition_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0035_4648, 0x002e4248)?;
        c.g[S2][0] = sx(p(r32(c, S0), 0x10));
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0028_2cd0, 0x002e4254)?;
        c.g[V1][0] = sx(0x0049_0000);
        c.g[V0][0] = sx(0x0049_0000);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x7f00));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x7ea8));
        Addr(p(r32(c, S0), 0x74)).sw(c, c.g[S1][0]);
        c.g[A0][0] = sx(0xffff_ffff);
        Addr(p(r32(c, S0), 0x40)).sw(c, 0);
        Addr(p(r32(c, S0), 0x44)).sw(c, 0);
        Addr(p(r32(c, S0), 0x4c)).sw(c, 0);
        Addr(p(r32(c, S0), 0x50)).sw(c, 0);
        Addr(p(r32(c, S0), 0x60)).sw(c, 0);
        Addr(p(r32(c, S0), 0x64)).sw(c, 0);
        Addr(p(r32(c, S0), 0x68)).sw(c, 0);
        Addr(p(r32(c, S0), 0x70)).sw(c, 0);
        Addr(p(r32(c, S0), 0x78)).sw(c, 0);
        Addr(p(r32(c, S0), 0x7c)).sw(c, 0);
        Addr(p(r32(c, S0), 0x80)).sw(c, 0);
        Addr(p(r32(c, S0), 0x1c)).sw(c, c.g[V1][0]);
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0x6c)).sw(c, c.g[A0][0]);
        call_ptr(c, 0x002e_4cb0, 0x002e42a8)?;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0028_3298, 0x002e42b0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Visual FX.

/// `void cTransitionMan_Stop(cTransitionMan* self)` (0x2e4578): if a
/// transition is running, release both effect objects (virtual `Stop(3)`),
/// reset the state words and re-fetch the frame counter from the world's vfx
/// interface.
pub fn ctransition_man_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        open(c, 0x20);
        c.sq(16, at_6(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr(this as u32).off(0x44).lw(c);
        if c.g[V0][0] == 0 {
            c.lq(16, at_6(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 4;
        c.g[A2][0] = Addr(this as u32).off(0x4c).lw(c);
        Addr(this as u32).off(0x44).sw(c, 0);
        let none = c.g[A2][0] == 0;
        Addr(this as u32).off(0x40).sw(c, 0);
        if !none {
            c.cycles += 6;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A1][0] = 3;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002e_45bc)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(this as u32).off(0x50).lw(c);
        let none = c.g[A2][0] == 0;
        Addr(this as u32).off(0x4c).sw(c, 0);
        if !none {
            c.cycles += 6;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A1][0] = 3;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002e_45e0)?;
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(this as u32).off(0x6c).lw(c);
        Addr(this as u32).off(0x50).sw(c, 0);
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        let skip = c.g[V0][0] == 0;
        Addr(this as u32).off(0x70).sw(c, 0);
        if !skip {
            c.cycles += 11;
            c.g[A1][0] = Addr(c.g[GP][0] as u32).off(0x2a90).lw(c);
            c.g[V0][0] = 1;
            Addr(this as u32).off(0x7c).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x10d8).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x390).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x394).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002e_4614)?;
            Addr(this as u32).off(0x80).sw(c, c.g[V0][0]);
            Addr(this as u32).off(0x64).sw(c, 0);
            Addr(this as u32).off(0x60).sw(c, 0);
        }
        c.cycles += 2;
        Addr(this as u32).off(0x78).sw(c, 0);
        c.lq(16, at_6(c.g[SP][0] as u32, 0x10));
        c.cycles += 3;
        epilogue(c, 0, 0x20);
        Ok(())
    })
}


/// `cTransitionMan_ScriptActivate(this)` (0x2e4b98): activate the script's
/// transition — a captured start for type 7, a full start otherwise — and
/// record its level.
pub fn c_transition_man_script_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[A0][0] = sx(this.wrapping_add(0x10));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x28));
        call_ptr(c, 0x0028_3430, 0x002e_4bbc)?;

        c.g[A1][0] = Addr(this).off(0x28).lw(c);
        c.g[V0][0] = 7;
        let captured = c.g[A1][0] == c.g[V0][0];
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        if !captured {
            c.cycles += 19;
            c.g[A2][0] = Addr(this).off(0x38).lw(c);
            callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET.call(c, 0x002e_4bd4)?;
            c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = Addr(this).off(0x28).lw(c);
            c.g[A2][0] = Addr(this).off(0x38).lw(c);
            callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET.call(c, 0x002e_4be8)?;
            c.lwc1(12, this.wrapping_add(0x34));
            c.g[A2][0] = c.g[S0][0];
            c.lwc1(13, this.wrapping_add(0x30));
            c.g[A3][0] = c.g[V0][0];
            c.lwc1(14, this.wrapping_add(0x2c));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            callee::C_TRANSITION_MAN_START.call(c, 0x002e_4c10)?;
            c.lwc1(0, abi::r32(c, S2).wrapping_add(0x14));
        } else {
            c.cycles += 10;
            c.g[A1][0] = 7;
            c.g[A2][0] = Addr(this).off(0x38).lw(c);
            callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET.call(c, 0x002e_4c24)?;
            c.lwc1(12, this.wrapping_add(0x34));
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            c.g[A3][0] = 0;
            callee::C_TRANSITION_MAN_START_CAPTURED.call(c, 0x002e_4c3c)?;
            c.lwc1(0, abi::r32(c, S2).wrapping_add(0x14));
        }

        c.cycles += 7;
        frame.restore(c, 16, 0x30);
        c.swc1(0, this.wrapping_add(0x48));
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// 0x0034d778 Fence_Render
// ---------------------------------------------------------------------------


// ---------------------------------------------------------------------------
// 0x002e4d88 SnowBox_ctor
// ---------------------------------------------------------------------------

/// `SnowBox* SnowBox_ctor(SnowBox* self, const Vec4* a, const Vec4* b)`
/// (0x2e4d88).
///
/// Initialises the snow-box: clears its flag block, copies the two vectors into
/// `+0xc..+0x1c`, seeds three random values, builds the box orientation from
/// the identity matrix at `0x4ff1a0` (scaled by `+0xc`) and stores the unit
/// quadword at `+0x20`. `v0` is the object.
pub fn snow_box_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 98;
        let frame = Frame::open(c, 0xc0);
        c.g[V0][0] = 1;
        frame.save_fpr(c, 20, 0xb0);
        frame.save(c, 16, 0xa0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20); // f20 = 1.0
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x90);
        let obj = c.g[S0][0] as u32;
        c.sw(p(obj, 8), 0);
        c.sw(p(obj, 0x10), 0);
        c.sw(p(obj, 0x20), 0);
        c.sw(p(obj, 0x24), 0);
        c.sw(p(obj, 0x28), 0);
        c.swc1(20, p(obj, 0x2c));
        c.sw(p(obj, 0x30), 0);
        c.sw(p(obj, 0x34), 0);
        c.sw(p(obj, 0x38), 0);
        c.sw(p(obj, 0x3c), 0);
        c.sw(p(obj, 0x40), 0);
        c.sw(p(obj, 0x44), 0);
        c.sw(p(obj, 0x48), 0);
        c.sw(p(obj, 0x4c), 0);
        c.sw(p(obj, 0x90), 0);
        c.sw(p(obj, 0x94), 0);
        c.swc1(20, p(obj, 0xa4));
        c.swc1(20, p(obj, 0xa8));
        c.swc1(20, p(obj, 0xac));
        c.sw(obj, c.g[A1][0]);
        c.sw(p(obj, 4), c.g[A2][0]);
        c.swc1(12, p(obj, 0xc));
        c.swc1(13, p(obj, 0x14));
        c.swc1(14, p(obj, 0x18));
        c.swc1(15, p(obj, 0x1c));
        c.g[RA][0] = sx(0x002e_4e14);
        c.sw(p(obj, 0xb0), c.g[V0][0]);
        call_ptr(c, 0x0031_77f0, 0x002e_4e14)?; // BXrand
        c.g[RA][0] = sx(0x002e_4e1c);
        c.sw(p(obj, 0x98), c.g[V0][0]);
        call_ptr(c, 0x0031_77f0, 0x002e_4e1c)?; // BXrand
        c.g[RA][0] = sx(0x002e_4e24);
        c.sw(p(obj, 0x9c), c.g[V0][0]);
        call_ptr(c, 0x0031_77f0, 0x002e_4e24)?; // BXrand
        c.g[V1][0] = 0x0050_0000;
        c.sw(p(obj, 0xa0), c.g[V0][0]);
        c.g[V1][0] = sx(0x0050_0000u32.wrapping_sub(0xe60)); // 0x4ff1a0
        c.g[V0][0] = sx(p(obj, 0x50));

        // The base matrix, copied quadruple by quadruple.
        c.lqc2(1, c.g[V1][0] as u32);
        c.lqc2(2, p(c.g[V1][0] as u32, 0x10));
        c.lqc2(3, p(c.g[V1][0] as u32, 0x20));
        c.lqc2(4, p(c.g[V1][0] as u32, 0x30));
        c.sqc2(1, c.g[V0][0] as u32);
        c.sqc2(2, p(c.g[V0][0] as u32, 0x10));
        c.sqc2(3, p(c.g[V0][0] as u32, 0x20));
        c.sqc2(4, p(c.g[V0][0] as u32, 0x30));
        c.lwc1(0, p(obj, 0xc));
        c.g[V1][0] = sx(p(c.g[SP][0] as u32, 0x50));
        c.swc1(20, p(c.g[SP][0] as u32, 0x4c));
        c.swc1(0, p(c.g[SP][0] as u32, 0x48));
        c.swc1(0, p(c.g[SP][0] as u32, 0x40));
        c.swc1(0, p(c.g[SP][0] as u32, 0x44));
        c.lqc2(1, p(c.g[SP][0] as u32, 0x40));
        c.lqc2(4, c.g[V0][0] as u32);
        c.lqc2(5, p(c.g[V0][0] as u32, 0x10));
        c.lqc2(6, p(c.g[V0][0] as u32, 0x20));
        c.lqc2(7, p(c.g[V0][0] as u32, 0x30));
        c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
        c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
        c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
        c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
        c.sqc2(8, c.g[V1][0] as u32);
        c.sqc2(9, p(c.g[V1][0] as u32, 0x10));
        c.sqc2(10, p(c.g[V1][0] as u32, 0x20));
        c.sqc2(11, p(c.g[V1][0] as u32, 0x30));
        c.lqc2(1, c.g[V1][0] as u32);
        c.lqc2(2, p(c.g[V1][0] as u32, 0x10));
        c.lqc2(3, p(c.g[V1][0] as u32, 0x20));
        c.lqc2(4, p(c.g[V1][0] as u32, 0x30));
        c.sqc2(1, c.g[SP][0] as u32);
        c.sqc2(2, p(c.g[SP][0] as u32, 0x10));
        c.sqc2(3, p(c.g[SP][0] as u32, 0x20));
        c.sqc2(4, p(c.g[SP][0] as u32, 0x30));
        c.lqc2(1, c.g[SP][0] as u32);
        c.lqc2(2, p(c.g[SP][0] as u32, 0x10));
        c.lqc2(3, p(c.g[SP][0] as u32, 0x20));
        c.lqc2(4, p(c.g[SP][0] as u32, 0x30));
        c.sqc2(1, c.g[V0][0] as u32);
        c.sqc2(2, p(c.g[V0][0] as u32, 0x10));
        c.sqc2(3, p(c.g[V0][0] as u32, 0x20));
        c.sqc2(4, p(c.g[V0][0] as u32, 0x30));
        c.swc1(20, p(c.g[SP][0] as u32, 0xc));
        c.g[V0][0] = c.g[S0][0];
        c.sw(c.g[SP][0] as u32, 0);
        c.sw(p(c.g[SP][0] as u32, 4), 0);
        c.sw(p(c.g[SP][0] as u32, 8), 0);
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x90));
        c.lq(3, c.g[SP][0] as u32);
        c.lwc1(20, p(c.g[SP][0] as u32, 0xb0));
        c.sq(3, p(obj, 0x20));
        c.lq(16, p(c.g[SP][0] as u32, 0xa0));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = ra;
        Ok(())
    })
}


/// `int Rider_GetMainState(Rider*)` (0x11fe98), already hand-written.
pub(crate) const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;

/// `void cBoardWakeFX_AdvanceRing(cBoardWakeFX*)` (0x2de058).
pub(crate) const ADVANCE_RING: Callee = Callee {
    addr: 0x002d_e058,
    f: ee::s09::cBoardWakeFX_AdvanceRing,
};

/// `void cBoardWakeFX_BuildTrail(cBoardWakeFX*, Vec4*, Vec4*, float)`
/// (0x2ddd30).
pub(crate) const BUILD_TRAIL: Callee = Callee {
    addr: 0x002d_dd30,
    f: ee::s09::cBoardWakeFX_BuildTrail,
};


/// `void cBoardWakeFX_Update(cBoardWakeFX* self)` (0x002dd0b8).
///
/// `self+0` is the rider, `self+0x38` the ring time, `self+0x3c` the
/// "current trail built" flag. The section array is `self+0x0c`; each slot is
/// 0x70 bytes: a 0x20-byte header, five 0x10-byte points (age at +0x64 of the
/// first) and the row of 0x10-byte element rings at `self+0x10` indexed by
/// section slot.
pub fn c_board_wake_fx_update(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                // Entry: frame, world/game/rider indices, then the age loop.
                0x002d_d0b8 => {
                    c.cycles += 21;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[A2][0] = 0xb0;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[A3][0] = 0u64;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x158));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
                    // a1 = self->obj; world->+0x84 = game; game->+0x44 + obj
                    // index*0xb0 = this rider's race data (s3).
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x438));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                    c.mult(0, 2, 2, 6);
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    let no_live = (c.g[V1][0] as i64) <= 0;
                    c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    pc = if no_live { 0x002d_d2a8 } else { 0x002d_d10c };
                }
                // Constant setup for the age loop (dt, 20, 1.0, 0, 0x70, 0x30).
                0x002d_d10c => {
                    c.cycles += 13;
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3c34));
                    c.g[T4][0] = 0u64;
                    c.g[AT][0] = 0x41a00000;
                    c.mtc1(1, 7);
                    c.g[T3][0] = 0x70;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.g[T2][0] = 0x30;
                    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3c30));
                    c.g[T1][0] = 0x30;
                    c.mtc1(0, 5);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    pc = 0x002d_d140;
                }
                // Section slot = (cursor + i) % max; age += dt.
                0x002d_d140 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xb8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.div(0, 2, 3);
                    // The original compares the divisor with t4 (0) and both
                    // arms continue here.
                    pc = 0x002d_d154;
                }
                // Strip address = strips + slot*0x70; compare age with life.
                0x002d_d154 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A2][0] = c.hi[0];
                    c.mult(0, 2, 6, 11);
                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x64));
                    c.add_s(1, 0, 4);
                    c.swc1(1, (c.g[A1][0] as u32).wrapping_add(0x64));
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0xbc));
                    c.c_lt_s(0, 1);
                    let expired = c.fcc();
                    c.mult(0, 4, 6, 10);
                    pc = if expired { 0x002d_d378 } else { 0x002d_d184 };
                }
                // The section is alive: fade the point and write the first
                // element ring of the section.
                0x002d_d184 => {
                    c.cycles += 36;
                    c.mul_s(0, 1, 6);
                    c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x60));
                    c.mul_s(1, 1, 7);
                    c.g[T0][0] = 0x1;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.sub_s(0, 3, 0);
                    c.min_s(1, 1, 3);
                    c.sub_s(1, 3, 1);
                    c.max_s(0, 0, 5);
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[A1][0] as u32).wrapping_add(0x50));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.mul_s(1, 1, 2);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.add_s(1, 1, 3);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.mul_s(2, 1, 4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xb4));
                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let one_point = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                    pc = if one_point { 0x002d_d294 } else { 0x002d_d214 };
                }
                // Element stride (0x30) and the per-point ring array.
                0x002d_d214 => {
                    c.cycles += 3;
                    c.mult(0, 6, 6, 9);
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
                    pc = 0x002d_d220;
                }
                // Inner loop: add the offset of every further point of the
                // section to its element ring (0x10 bytes per element).
                0x002d_d220 => {
                    c.cycles += 29;
                    c.mfc1(2, 2);
                    c.lqc2(4, c.g[A0][0] as u32);
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x14));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x20));
                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.lwc1(1, (c.g[A1][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.add_s(0, 0, 1);
                    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xb4));
                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more_points = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    if more_points {
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_d220);
                        }
                        pc = 0x002d_d220;
                    } else {
                        pc = 0x002d_d294;
                    }
                }
                // Next live section?
                0x002d_d294 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_d140);
                        }
                        pc = 0x002d_d140;
                    } else {
                        pc = 0x002d_d2a4;
                    }
                }
                0x002d_d2a4 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    pc = 0x002d_d2a8;
                }
                // Per-frame trail state: read the rider's transform, blend the
                // two trail vectors of the newest point.
                0x002d_d2a8 => {
                    c.cycles += 21;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 2);
                    c.lq(2, (c.g[A1][0] as u32).wrapping_add(0x370));
                    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x1f0));
                    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x214));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.abs_s(0, 0);
                    c.sub_s(2, 2, 0);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                    c.neg_s(21, 1);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                    c.abs_s(1, 21);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.lq(2, c.g[A0][0] as u32);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x320));
                    let no_grind = c.g[V1][0] == 0u64;
                    c.mul_s(22, 1, 2);
                    pc = if no_grind { 0x002d_d324 } else { 0x002d_d2fc };
                }
                0x002d_d2fc => {
                    c.cycles += 10;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 0);
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    pc = 0x002d_d324;
                }
                0x002d_d324 => {
                    c.cycles += 12;
                    c.mtc1(0, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(5, (c.g[SP][0] as u32));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.c_lt_s(21, 0);
                    let positive = !c.fcc();
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    pc = if positive { 0x002d_d380 } else { 0x002d_d354 };
                }
                0x002d_d354 => {
                    c.cycles += 9;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 0);
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    pc = 0x002d_d380;
                }
                // The trail section expired: remember how many are live and
                // back to the per-frame state.
                0x002d_d378 => {
                    c.cycles += 2;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_d2a4);
                    }
                    pc = 0x002d_d2a4;
                }
                // Length of the newest point's velocity in the XZ plane.
                0x002d_d380 => {
                    c.cycles += 18;
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c2c));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x1e0));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 1);
                    c.c_lt_s(0, 1);
                    let fast = c.fcc();
                    pc = if fast { 0x002d_d3dc } else { 0x002d_d3c8 };
                }
                0x002d_d3c8 => {
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_le_s(0, 1);
                    let slow = !c.fcc();
                    c.mov_s(6, 0);
                    pc = if slow { 0x002d_d420 } else { 0x002d_d3dc };
                }
                0x002d_d3dc => {
                    c.cycles += 16;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3c28));
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x1e0));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(1, 0);
                    let clamp = !c.fcc();
                    c.mov_s(6, 0);
                    pc = if clamp { 0x002d_d420 } else { 0x002d_d41c };
                }
                0x002d_d41c => {
                    c.cycles += 1;
                    c.mov_s(6, 1);
                    pc = 0x002d_d420;
                }
                // Trail intensity: distance to the game camera's focus.
                0x002d_d420 => {
                    c.cycles += 8;
                    c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x48));
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 4);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x320));
                    let no_grind = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                    pc = if no_grind { 0x002d_d448 } else { 0x002d_d440 };
                }
                0x002d_d440 => {
                    c.cycles += 2;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 4);
                    pc = 0x002d_d448;
                }
                // Blur/fade amount: the second trail vector scaled by a speed
                // factor and the board's matrix.
                0x002d_d448 => {
                    c.cycles += 63;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x208));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[AT][0] = 0x42ec0000;
                    c.mtc1(1, 1);
                    c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x30));
                    c.abs_s(0, 0);
                    c.mul_s(0, 0, 1);
                    c.g[AT][0] = 0xffffffffc2200000;
                    c.mtc1(1, 2);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[AT][0] = 0x41700000;
                    c.mtc1(1, 1);
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                    c.add_s(0, 0, 2);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                    c.mtc1(0, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 2);
                    c.lq(2, c.g[V1][0] as u32);
                    c.c_lt_s(21, 5);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.mfc1(3, 1);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    let keep = !c.fcc();
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    pc = if keep { 0x002d_d54c } else { 0x002d_d544 };
                }
                0x002d_d544 => {
                    c.cycles += 2;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 2);
                    pc = 0x002d_d54c;
                }
                // Combine the two vectors, the two trail colours and the
                // section's fade into the final colour and width.
                0x002d_d54c => {
                    c.cycles += 61;
                    c.mfc1(2, 2);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3c24));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.mov_s(7, 5);
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 1);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.mul_s(1, 22, 1);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 2);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[AT][0] = 0x3e800000;
                    c.mtc1(1, 4);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x1e0));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().rsqrt(0, 3, 4, 0);
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 3, Src::Q);
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x3b0));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(4, 4);
                    c.mtc1(4, 0);
                    c.abs_s(0, 0);
                    c.mul_s(0, 0, 3);
                    c.add_s(3, 0, 1);
                    c.min_s(3, 3, 2);
                    c.add_s(0, 3, 3);
                    c.c_le_s(5, 0);
                    if c.fcc() {
                        c.min_s(7, 0, 4);
                    }
                    pc = 0x002d_d640;
                }
                // Fade the colour with the trail brightness.
                0x002d_d640 => {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c20));
                    c.c_lt_s(0, 6);
                    if !c.fcc() {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x84));
                        pc = 0x002d_d680;
                    } else {
                        pc = 0x002d_d654;
                    }
                }
                0x002d_d654 => {
                    c.cycles += 8;
                    c.sub_s(1, 3, 4);
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c1c));
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3c18));
                    c.mul_s(1, 1, 0);
                    c.c_le_s(5, 1);
                    let below = !c.fcc();
                    c.mov_s(4, 5);
                    pc = if below { 0x002d_d678 } else { 0x002d_d674 };
                }
                0x002d_d674 => {
                    c.cycles += 1;
                    c.min_s(4, 1, 2);
                    pc = 0x002d_d678;
                }
                0x002d_d678 => {
                    c.cycles += 2;
                    c.mov_s(5, 4);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x84));
                    pc = 0x002d_d680;
                }
                0x002d_d680 => {
                    c.cycles += 3;
                    c.mtc1(0, 20);
                    if c.g[V0][0] == 0u64 {
                        c.add_s(0, 7, 5);
                        pc = 0x002d_d6d8;
                    } else {
                        pc = 0x002d_d68c;
                    }
                }
                0x002d_d68c => {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c14));
                    c.c_lt_s(0, 6);
                    if !c.fcc() {
                        c.add_s(0, 7, 5);
                        pc = 0x002d_d6d8;
                    } else {
                        pc = 0x002d_d6a0;
                    }
                }
                0x002d_d6a0 => {
                    c.cycles += 11;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c10));
                    c.g[AT][0] = 0x420c0000;
                    c.mtc1(1, 1);
                    c.sub_s(0, 3, 0);
                    c.g[AT][0] = 0x40900000;
                    c.mtc1(1, 2);
                    c.mul_s(0, 0, 1);
                    c.c_le_s(20, 0);
                    let below = !c.fcc();
                    c.mov_s(4, 20);
                    pc = if below { 0x002d_d6d0 } else { 0x002d_d6cc };
                }
                0x002d_d6cc => {
                    c.cycles += 1;
                    c.min_s(4, 0, 2);
                    pc = 0x002d_d6d0;
                }
                0x002d_d6d0 => {
                    c.cycles += 2;
                    c.mov_s(20, 4);
                    c.add_s(0, 7, 5);
                    pc = 0x002d_d6d8;
                }
                // Final width, then the "trail is dead" test.
                0x002d_d6d8 => {
                    c.cycles += 9;
                    c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x4c));
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[S0][0] = 0x1;
                    c.mul_s(0, 6, 0);
                    c.mul_s(0, 0, 1);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                    call_ptr(c, RIDER_GET_MAIN_STATE, 0x002d_d6f4)?;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        pc = 0x002d_d760;
                    } else {
                        pc = 0x002d_d6fc;
                    }
                }
                // Airborne check and speed gates: kill a stale trail.
                0x002d_d6fc => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3c0c));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x274));
                    c.abs_s(0, 0);
                    c.c_lt_s(2, 0);
                    if c.fcc() {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        pc = 0x002d_d760;
                    } else {
                        pc = 0x002d_d71c;
                    }
                }
                0x002d_d71c => {
                    c.cycles += 6;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3c08));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.c_lt_s(0, 1);
                    if c.fcc() {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        pc = 0x002d_d760;
                    } else {
                        pc = 0x002d_d734;
                    }
                }
                0x002d_d734 => {
                    c.cycles += 4;
                    c.c_lt_s(22, 2);
                    if c.fcc() {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        pc = 0x002d_d760;
                    } else {
                        pc = 0x002d_d744;
                    }
                }
                0x002d_d744 => {
                    c.cycles += 6;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c04));
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3c));
                        pc = 0x002d_d768;
                    } else {
                        pc = 0x002d_d75c;
                    }
                }
                0x002d_d75c => {
                    c.cycles += 1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                    pc = 0x002d_d760;
                }
                0x002d_d760 => {
                    c.cycles += 2;
                    c.g[S0][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3c));
                    pc = 0x002d_d768;
                }
                // Already built?
                0x002d_d768 => {
                    c.cycles += 2;
                    let built = c.g[V0][0] == 0u64;
                    if built { pc = 0x002d_d9e4 } else { pc = 0x002d_d770 };
                }
                0x002d_d770 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xb0));
                    let no_prev = c.g[V0][0] == 0u64;
                    if no_prev { pc = 0x002d_d798 } else { pc = 0x002d_d77c };
                }
                0x002d_d77c => {
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_lt_s(21, 0);
                    if c.fcc() {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        pc = 0x002d_d7ac;
                    } else {
                        pc = 0x002d_d790;
                    }
                }
                0x002d_d790 => {
                    c.cycles += 2;
                    pc = 0x002d_d7b0;
                }
                0x002d_d798 => {
                    c.cycles += 4;
                    c.mtc1(0, 0);
                    c.c_lt_s(0, 21);
                    let not_positive = !c.fcc();
                    if not_positive {
                        pc = 0x002d_d7b0;
                    } else {
                        pc = 0x002d_d7a8;
                    }
                }
                0x002d_d7a8 => {
                    c.cycles += 1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                    pc = 0x002d_d7ac;
                }
                0x002d_d7ac => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x90), 0u64);
                    pc = 0x002d_d7b0;
                }
                // Second airborne check before building the strip.
                0x002d_d7b0 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    call_ptr(c, RIDER_GET_MAIN_STATE, 0x002d_d7b8)?;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x94), 0u64);
                        pc = 0x002d_d9c4;
                    } else {
                        pc = 0x002d_d7c0;
                    }
                }
                // Blend trail colour 1 with the section fade and advance the
                // ring; store the resulting position/width when it moved.
                0x002d_d7c0 => {
                    c.cycles += 83;
                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3c00));
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.mfc1(2, 5);
                    c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x60));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3bfc));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3bf8));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3bf4));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3bf0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x60));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.mfc1(3, 5);
                    c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x70));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x70));
                    c.mfc1(3, 5);
                    c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x80));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x94));
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.mul_s(1, 1, 0);
                    c.lwc1(3, (c.g[S2][0] as u32).wrapping_add(0x90));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.mul_s(2, 2, 6);
                    c.mul_s(3, 3, 5);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.mul_s(0, 0, 4);
                    c.add_s(1, 1, 2);
                    c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.add_s(3, 3, 0);
                    c.c_lt_s(7, 1);
                    c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0x94));
                    let small = !c.fcc();
                    c.swc1(3, (c.g[S2][0] as u32).wrapping_add(0x90));
                    pc = if small { 0x002d_d9ac } else { 0x002d_d90c };
                }
                0x002d_d90c => {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3bec));
                    c.c_lt_s(0, 3);
                    if !c.fcc() {
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x94), 0u64);
                        pc = 0x002d_d9b0;
                    } else {
                        pc = 0x002d_d920;
                    }
                }
                // The new segment is long enough: advance the ring and build.
                0x002d_d920 => {
                    c.cycles += 23;
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x50));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[AT][0] = 0x42480000;
                    c.mtc1(1, 1);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(1, 0);
                    let short = !c.fcc();
                    c.g[A1][0] = c.g[S3][0];
                    if short { pc = 0x002d_d990 } else { pc = 0x002d_d97c };
                }
                0x002d_d97c => {
                    c.cycles += 5;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002d_d988);
                    c.g[A0][0] = c.g[S2][0];
                    c.pc = 0x002d_e058;
                    ADVANCE_RING.call(c, 0x002d_d988)?;
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x50));
                    pc = 0x002d_d990;
                }
                0x002d_d990 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002d_d9a4);
                    c.mov_s(12, 20);
                    c.pc = 0x002d_dd30;
                    BUILD_TRAIL.call(c, 0x002d_d9a4)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    pc = 0x002d_da90;
                }
                0x002d_d9ac => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x94), 0u64);
                    pc = 0x002d_d9b0;
                }
                0x002d_d9b0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3c), 0u64);
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    pc = 0x002d_d9d4;
                }
                0x002d_d9c4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3c), 0u64);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    pc = 0x002d_d9d4;
                }
                0x002d_d9d4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002d_d9dc);
                    c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x94));
                    c.pc = 0x002d_dd30;
                    BUILD_TRAIL.call(c, 0x002d_d9dc)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    pc = 0x002d_da90;
                }
                // Nothing built yet: start the trail from the stored point.
                0x002d_d9e4 => {
                    c.cycles += 2;
                    let have_point = c.g[S0][0] == 0u64;
                    c.g[A2][0] = 0x1;
                    if have_point { pc = 0x002d_da8c } else { pc = 0x002d_d9ec };
                }
                0x002d_d9ec => {
                    c.cycles += 13;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x90), 0u64);
                    c.lwc1(20, (c.g[S2][0] as u32).wrapping_add(0x90));
                    c.lq(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.c_lt_s(20, 21);
                    c.lq(2, c.g[SP][0] as u32);
                    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(5, (c.g[S2][0] as u32).wrapping_add(0x80));
                    c.sq(4, (c.g[S2][0] as u32).wrapping_add(0x70));
                    c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x60));
                    c.sq(3, (c.g[S2][0] as u32).wrapping_add(0x50));
                    let wide = c.fcc();
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x40), 0u64);
                    if wide { pc = 0x002d_da24 } else { pc = 0x002d_da20 };
                }
                0x002d_da20 => {
                    c.cycles += 1;
                    c.g[A2][0] = 0u64;
                    pc = 0x002d_da24;
                }
                // Two ring advances and two strip builds, then mark built.
                0x002d_da24 => {
                    c.cycles += 26;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xb0), c.g[A2][0]);
                    c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x94));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002d_da40);
                    c.g[A2][0] = c.g[S0][0];
                    c.pc = 0x002d_e058;
                    ADVANCE_RING.call(c, 0x002d_da40)?;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.mov_s(12, 20);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002d_da58);
                    c.g[A2][0] = c.g[S1][0];
                    c.pc = 0x002d_dd30;
                    BUILD_TRAIL.call(c, 0x002d_da58)?;
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002d_da70);
                    c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x94));
                    c.pc = 0x002d_e058;
                    ADVANCE_RING.call(c, 0x002d_da70)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.mov_s(12, 20);
                    c.g[RA][0] = sx(0x002d_da84);
                    c.g[A0][0] = c.g[S2][0];
                    c.pc = 0x002d_dd30;
                    BUILD_TRAIL.call(c, 0x002d_da84)?;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                    pc = 0x002d_da8c;
                }
                0x002d_da8c => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    pc = 0x002d_da90;
                }
                // Epilogue: the caller's callee-saved registers and FPU
                // temporaries, then return.
                0x002d_da90 => {
                    c.cycles += 9;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x158));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_d0b8, 0x002d_dab4);
                    return Ok(());
                }
            }
        }
    })
}


// ---------------------------------------------------------------------------
// cSpeedBoostStreamerFX_UpdatePoints.
//
// The original is a dispatcher (it has an `__entries` twin): most of its blocks
// are separate resume points. Per docs/investigation/phase2-overrides.md the
// override stub sends every entry other than the function start to the
// generated twin, so this replacement handles the start only (`pc` is left at_7
// the loop head when an event asks to unwind, and the stub resumes there).
// The VU0 work is issued as the original's own COP2 macro ops.

/// `void cSpeedBoostStreamerFX_UpdatePoints(cSpeedBoostStreamerFX* self)`
/// (0x2e6c08): one step of the speed-boost wake particle streamer. It
/// integrates the point buffers from the rider's velocity and board, advances
/// the two wake shapes, and fades the timer at_7 `+0x18`.
pub fn c_speed_boost_streamer_fx_update_points(c: &mut Cpu) {
    guest(c, |c| {
        'b_2e7950: {
            'b_2e6c4c: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
                c.mtc1(0, 1);
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.g[S0][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x120), c.g[RA][0]);
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.c_lt_s(1, 0);
                if c.fcc() {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    break 'b_2e6c4c;
                } else {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x18));
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x4));
                        break 'b_2e7950;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                        break 'b_2e6c4c;
                    }
                }
            }
            'b_2e794c: {
                c.cycles += 14;
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x1e0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = 0x1e;
                if t {
                    break 'b_2e794c;
                } else {
                    'b_2e6cdc: {
                        'b_2e6cd4: {
                            c.cycles += 3;
                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            let t = c.g[A3][0] != c.g[V0][0];
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b_2e6cd4;
                            } else {
                                'b_2e6cb4: {
                                    c.cycles += 9;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                                    c.g[A2][0] = 0x1c;
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                                    c.g[A1][0] = 0x1e;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d));
                                    c.g[S1][0] = c.g[A0][0];
                                    c.div(0, 3, 7);
                                    if c.g[A1][0] == 0u64 {
                                        break 'b_2e6cb4;
                                    } else {
                                        break 'b_2e6cb4;
                                    }
                                }
                                c.cycles += 8;
                                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[V0][0] = c.hi[0];
                                c.mult(0, 2, 2, 6);
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.lwc1(0, (c.g[A0][0] as u32));
                                c.sub_s(1, 1, 0);
                                let t = true;
                                c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
                                if t {
                                    break 'b_2e6cdc;
                                } else {
                                    break 'b_2e6cd4;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        break 'b_2e6cdc;
                    }
                    'b_2e6cec: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = 0x1e;
                        if c.g[V1][0] == 0u64 {
                            break 'b_2e6cec;
                        } else {
                            break 'b_2e6cec;
                        }
                    }
                    'b_2e6db0: {
                        c.cycles += 38;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1d));
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                        c.div(0, 2, 3);
                        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
                        c.g[A0][0] = c.hi[0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6c0));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002e_6d18)?;
                        c.lq(3, (c.g[V0][0] as u32));
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                        c.sq(3, (c.g[SP][0] as u32));
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.lq(2, (c.g[A0][0] as u32));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x320));
                        if c.g[V1][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            break 'b_2e6db0;
                        } else {
                            c.cycles += 11;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 0);
                            c.mfc1(3, 0);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            break 'b_2e6db0;
                        }
                    }
                    'b_2e6e08: {
                        'b_2e6e04: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                                break 'b_2e6e04;
                            } else {
                                c.cycles += 18;
                                c.lqc2(3, (c.g[SP][0] as u32));
                                c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x20));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                c.vu0().sqrt(4, 0);
                                c.vu0().waitq();
                                c.cfc2(3, 22);
                                c.mtc1(3, 3);
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_2e6e08;
                                } else {
                                    break 'b_2e6e04;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.mtc1(0, 3);
                        break 'b_2e6e08;
                    }
                    'b_2e6e40: {
                        c.cycles += 14;
                        c.g[A1][0] = 0x1c;
                        c.lq(2, (c.g[SP][0] as u32));
                        c.g[A0][0] = 0x1e;
                        c.mult(0, 6, 3, 5);
                        c.mtc1(0, 4);
                        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
                        c.g[AT][0] = 0x42960000;
                        c.mtc1(1, 1);
                        c.g[AT][0] = 0x418c0000;
                        c.mtc1(1, 2);
                        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        c.swc1(4, (c.g[V1][0] as u32));
                        if c.g[A0][0] == 0u64 {
                            break 'b_2e6e40;
                        } else {
                            break 'b_2e6e40;
                        }
                    }
                    'b_2e7060: {
                        c.cycles += 118;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.div(0, 2, 4);
                        c.g[V1][0] = c.hi[0];
                        c.mult(0, 2, 3, 5);
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        c.swc1(3, (c.g[A1][0] as u32));
                        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x18));
                        c.add_s(0, 0, 3);
                        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x18));
                        c.mfc1(3, 1);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.mfc1(3, 2);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.mfc1(3, 1);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.mfc1(3, 2);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.mfc1(3, 1);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.mfc1(3, 2);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.mfc1(3, 1);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.mfc1(3, 2);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x330));
                        let t = c.g[V1][0] == 0u64;

                        if t {
                            break 'b_2e7060;
                        } else {
                            'b_2e7058: {
                                'b_2e7048: {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x784));
                                    c.g[A1][0] = 0x2;
                                    call_ptr(c, table::PLAY_ANIM, 0x002e_7024)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x17));
                                    c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0x25;
                                    if t {
                                        break 'b_2e7048;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x26;
                                        if t {
                                            break 'b_2e7048;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];

                                            if t {
                                                break 'b_2e7058;
                                            } else {
                                                break 'b_2e7048;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 4);
                                let t = true;

                                if t {
                                    break 'b_2e7060;
                                } else {
                                    break 'b_2e7058;
                                }
                            }
                            c.cycles += 2;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 4);
                            break 'b_2e7060;
                        }
                    }
                    'b_2e71bc: {
                        'b_2e7110: {
                            c.cycles += 4;
                            c.mtc1(0, 0);
                            c.c_lt_s(0, 4);
                            let t = !c.fcc();

                            if t {
                                break 'b_2e7110;
                            } else {
                                c.cycles += 40;
                                c.g[AT][0] = 0x43160000;
                                c.mtc1(1, 0);
                                c.mfc1(3, 0);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.mfc1(3, 4);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.mfc1(3, 0);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.mfc1(3, 4);
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                if t {
                                    break 'b_2e71bc;
                                } else {
                                    break 'b_2e7110;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.c_lt_s(4, 0);
                        if !c.fcc() {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                            break 'b_2e71bc;
                        } else {
                            c.cycles += 39;
                            c.g[AT][0] = 0x43160000;
                            c.mtc1(1, 0);
                            c.mfc1(3, 0);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.mfc1(3, 4);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.mfc1(3, 0);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.mfc1(3, 4);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                            break 'b_2e71bc;
                        }
                    }
                    'b_2e7240: {
                        c.cycles += 22;
                        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x1e0));
                        c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x370));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                        c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.mtc1(0, 1);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x40));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(3, 4);
                        c.mtc1(3, 0);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        if t {
                            break 'b_2e7240;
                        } else {
                            c.cycles += 11;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 0);
                            c.mfc1(3, 0);
                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                            break 'b_2e7240;
                        }
                    }
                    'b_2e7464: {
                        'b_2e73f0: {
                            'b_2e73d8: {
                                'b_2e73a0: {
                                    'b_2e7380: {
                                        'b_2e7374: {
                                            c.cycles += 72;
                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                            c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x40));
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x460));
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x460));
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x460));
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x460));
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x90));
                                            c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(3, 4);
                                            c.mtc1(3, 8);
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                            c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 6);
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                            c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(3, 4);
                                            c.mtc1(3, 7);
                                            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                            c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 5);
                                            c.abs_s(2, 7);
                                            c.abs_s(1, 5);
                                            c.c_lt_s(1, 2);
                                            c.abs_s(4, 8);
                                            c.abs_s(0, 6);
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_2e7374;
                                            } else {
                                                c.cycles += 3;
                                                c.c_lt_s(2, 0);
                                                let t = c.fcc();

                                                if t {
                                                    break 'b_2e7380;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_2e73a0;
                                                    } else {
                                                        break 'b_2e7374;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.c_lt_s(1, 0);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_2e73a0;
                                        } else {
                                            break 'b_2e7380;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.abs_s(0, 6);
                                    c.c_lt_s(0, 4);
                                    let t = c.fcc();

                                    if t {
                                        break 'b_2e73d8;
                                    } else {
                                        c.cycles += 4;
                                        c.abs_s(3, 7);
                                        c.abs_s(2, 5);
                                        let t = true;

                                        if t {
                                            break 'b_2e73f0;
                                        } else {
                                            break 'b_2e73a0;
                                        }
                                    }
                                }
                                'b_2e73cc: {
                                    c.cycles += 6;
                                    c.abs_s(0, 7);
                                    c.abs_s(1, 5);
                                    c.c_lt_s(1, 0);
                                    c.mov_s(3, 0);
                                    let t = !c.fcc();
                                    c.mov_s(2, 1);
                                    if t {
                                        break 'b_2e73cc;
                                    } else {
                                        c.cycles += 3;
                                        c.c_lt_s(3, 4);
                                        let t = c.fcc();

                                        if t {
                                            break 'b_2e73d8;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_2e73f0;
                                            } else {
                                                break 'b_2e73cc;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.c_lt_s(2, 4);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e73f0;
                                } else {
                                    break 'b_2e73d8;
                                }
                            }
                            c.cycles += 6;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 1);
                            c.abs_s(0, 8);
                            c.div_s(0, 1, 0);
                            let t = true;
                            c.mul_s(2, 8, 0);
                            if t {
                                break 'b_2e7464;
                            } else {
                                break 'b_2e73f0;
                            }
                        }
                        'b_2e7434: {
                            'b_2e7420: {
                                'b_2e7414: {
                                    c.cycles += 4;
                                    c.c_lt_s(2, 3);
                                    c.abs_s(1, 6);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e7414;
                                    } else {
                                        c.cycles += 3;
                                        c.c_lt_s(3, 1);
                                        let t = c.fcc();

                                        if t {
                                            break 'b_2e7420;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_2e7434;
                                            } else {
                                                break 'b_2e7414;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.c_lt_s(2, 1);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e7434;
                                } else {
                                    break 'b_2e7420;
                                }
                            }
                            c.cycles += 5;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.div_s(0, 0, 1);
                            let t = true;
                            c.mul_s(2, 8, 0);
                            if t {
                                break 'b_2e7464;
                            } else {
                                break 'b_2e7434;
                            }
                        }
                        'b_2e7454: {
                            c.cycles += 3;
                            c.c_lt_s(2, 3);
                            let t = !c.fcc();

                            if t {
                                break 'b_2e7454;
                            } else {
                                c.cycles += 5;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 0);
                                c.div_s(0, 0, 3);
                                let t = true;
                                c.mul_s(2, 8, 0);
                                if t {
                                    break 'b_2e7464;
                                } else {
                                    break 'b_2e7454;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.div_s(0, 0, 2);
                        c.mul_s(2, 8, 0);
                        break 'b_2e7464;
                    }
                    'b_2e7698: {
                        'b_2e750c: {
                            c.cycles += 6;
                            c.mul_s(3, 6, 0);
                            c.mul_s(5, 5, 0);
                            c.c_lt_s(2, 3);
                            let t = !c.fcc();
                            c.mul_s(4, 7, 0);
                            if t {
                                break 'b_2e750c;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 1);
                                c.add_s(0, 4, 1);
                                c.c_lt_s(2, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e750c;
                                } else {
                                    c.cycles += 4;
                                    c.add_s(0, 5, 1);
                                    c.c_lt_s(2, 0);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e750c;
                                    } else {
                                        c.cycles += 26;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e7698;
                                        } else {
                                            break 'b_2e750c;
                                        }
                                    }
                                }
                            }
                        }
                        'b_2e75a8: {
                            c.cycles += 3;
                            c.c_lt_s(3, 2);
                            let t = !c.fcc();

                            if t {
                                break 'b_2e75a8;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 1);
                                c.add_s(0, 4, 1);
                                c.c_lt_s(3, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e75a8;
                                } else {
                                    c.cycles += 4;
                                    c.add_s(0, 5, 1);
                                    c.c_lt_s(3, 0);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e75a8;
                                    } else {
                                        c.cycles += 26;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e7698;
                                        } else {
                                            break 'b_2e75a8;
                                        }
                                    }
                                }
                            }
                        }
                        'b_2e7638: {
                            c.cycles += 4;
                            c.c_lt_s(4, 2);
                            if !c.fcc() {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                break 'b_2e7638;
                            } else {
                                c.cycles += 4;
                                c.c_lt_s(4, 3);
                                let t = !c.fcc();
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                if t {
                                    break 'b_2e7638;
                                } else {
                                    c.cycles += 3;
                                    c.c_lt_s(4, 5);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e7638;
                                    } else {
                                        c.cycles += 25;
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e7698;
                                        } else {
                                            break 'b_2e7638;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 24;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 1);
                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(2, 4);
                        c.mtc1(2, 0);
                        c.sub_s(0, 0, 1);
                        c.mfc1(4, 0);
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.qmtc2(4, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                        break 'b_2e7698;
                    }
                    'b_2e78c8: {
                        'b_2e773c: {
                            c.cycles += 5;
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                            c.c_le_s(3, 2);
                            let t = !c.fcc();
                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                            if t {
                                break 'b_2e773c;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 1);
                                c.sub_s(0, 4, 1);
                                c.c_le_s(0, 2);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e773c;
                                } else {
                                    c.cycles += 4;
                                    c.sub_s(0, 5, 1);
                                    c.c_le_s(0, 2);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e773c;
                                    } else {
                                        c.cycles += 26;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e78c8;
                                        } else {
                                            break 'b_2e773c;
                                        }
                                    }
                                }
                            }
                        }
                        'b_2e77d8: {
                            c.cycles += 3;
                            c.c_le_s(2, 3);
                            let t = !c.fcc();

                            if t {
                                break 'b_2e77d8;
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 1);
                                c.sub_s(0, 4, 1);
                                c.c_le_s(0, 3);
                                let t = !c.fcc();

                                if t {
                                    break 'b_2e77d8;
                                } else {
                                    c.cycles += 4;
                                    c.sub_s(0, 5, 1);
                                    c.c_le_s(0, 3);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e77d8;
                                    } else {
                                        c.cycles += 26;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e78c8;
                                        } else {
                                            break 'b_2e77d8;
                                        }
                                    }
                                }
                            }
                        }
                        'b_2e7868: {
                            c.cycles += 4;
                            c.c_le_s(2, 4);
                            if !c.fcc() {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                break 'b_2e7868;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(3, 4);
                                let t = !c.fcc();
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                                if t {
                                    break 'b_2e7868;
                                } else {
                                    c.cycles += 3;
                                    c.c_le_s(5, 4);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_2e7868;
                                    } else {
                                        c.cycles += 25;
                                        c.g[AT][0] = 0x41200000;
                                        c.mtc1(1, 1);
                                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.sub_s(0, 0, 1);
                                        c.mfc1(4, 0);
                                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                        c.qmtc2(4, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        let t = true;
                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                                        if t {
                                            break 'b_2e78c8;
                                        } else {
                                            break 'b_2e7868;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 24;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 1);
                        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(2, 4);
                        c.mtc1(2, 0);
                        c.sub_s(0, 0, 1);
                        c.mfc1(4, 0);
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.qmtc2(4, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x110));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
                        break 'b_2e78c8;
                    }
                    c.cycles += 33;
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = 0x1c;
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.mult(0, 2, 3, 4);
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xd4));
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xd8));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0xf7));
                    c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x5b));
                    c.sdr(5, (c.g[V1][0] as u32).wrapping_add(0x54));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x5c), c.g[A2][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.mult(0, 3, 2, 4);
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xe4));
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf7));
                    c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x67));
                    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x60));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x68), c.g[A1][0]);
                    break 'b_2e794c;
                }
            }
            c.cycles += 1;
            c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x4));
            break 'b_2e7950;
        }
        'b_2e796c: {
            c.cycles += 6;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.c_lt_s(0, 2);
            let t = !c.fcc();
            c.g[T1][0] = 0u64;
            if t {
                break 'b_2e796c;
            } else {
                c.cycles += 1;
                c.g[T1][0] = 0x3;
                break 'b_2e796c;
            }
        }
        'b_2e79fc: {
            c.cycles += 4;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                break 'b_2e79fc;
            } else {
                c.cycles += 5;
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
                c.c_lt_s(2, 1);
                if !c.fcc() {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    break 'b_2e79fc;
                } else {
                    c.cycles += 6;
                    c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                    c.g[A3][0] = 0x1e;
                    c.g[T4][0] = 0u64;
                    c.g[T3][0] = 0x1c;
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    'l_2e79a8: loop {
                        'b_2e79b0: {
                            c.cycles += 2;
                            if c.g[A3][0] == c.g[T4][0] {
                                break 'b_2e79b0;
                            } else {
                                break 'b_2e79b0;
                            }
                        }
                        'b_2e79f8: {
                            c.cycles += 13;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                            c.div(0, 2, 7);
                            c.g[A2][0] = c.g[A0][0];
                            c.g[A1][0] = ((c.g[T1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                            c.g[V1][0] = c.hi[0];
                            c.mult(0, 13, 3, 11);
                            c.g[V0][0] = sx((c.g[T5][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                            c.lwc1(0, (c.g[V0][0] as u32));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                            c.sub_s(1, 1, 0);
                            let t = c.g[A1][0] == 0u64;
                            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
                            if t {
                                break 'b_2e79f8;
                            } else {
                                c.cycles += 5;
                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                                c.c_lt_s(0, 1);
                                let t = c.fcc();
                                c.g[V0][0] =
                                    sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002e_79a8);
                                    }
                                    continue 'l_2e79a8;
                                } else {
                                    break 'b_2e79f8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                        break 'b_2e79fc;
                    }
                }
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x120));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
        abi::ret(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::fakes;
    use crate::difftest::fakes::ptr;
    use ssx3_ee as ee;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::elf_image;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::RA as RET;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::compare;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::replay::replay2::tests::dtor_setup;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festateoptions::tests::rng;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::be::beintplayer::tests::fillb;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_4 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_4 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_7 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::object::effectlink::tests::alloc;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals as game_globals_2;
    pub(crate) use crate::game::util::menu::tests::wf_2 as wf;
    pub(crate) use crate::game::ai::ridermetrix::tests::M;
    pub(crate) use crate::game::ai::ridermetrix::tests::BELIB;


    #[test]
    fn c_transition_man_set_paused_matches() {
        check(ssx3_ee::s09::f_002e4638, c_transition_man_set_paused, 300, &|c, seed| {
            let mut r = Rng(0x1006 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { ((r.next_u32() as u64) << 32) | 1 };
            c.bus.write32(DATA + 0x44, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(DATA + 0x40, if seed % 5 == 0 { 0 } else { r.next_u32() });
        });
    }


    #[test]
    fn c_transition_man_get_time_remaining_matches() {
        check(ssx3_ee::s09::f_002e4678, c_transition_man_get_time_remaining, 200, &|c, seed| {
            let mut r = Rng(0x1007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x44, if seed % 3 == 0 { 0 } else { r.next_u32() });
            for off in [0x48u32, 0x54, 0x58, 0x5c] {
                c.bus.write32(DATA + off, r.edge_f32());
            }
        });
    }


    #[test]
    fn transition_effect_ctor_matches() {
        check(ssx3_ee::s09::f_002e4d70, transition_effect_ctor, 100, &|c, seed| {
            let mut r = Rng(0x1008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }


    #[test]
    fn snow_box_reset_matches() {
        check(ssx3_ee::s09::f_002e4f10, snow_box_reset, 100, &|c, seed| {
            let mut r = Rng(0x1009 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x90, r.next_u32());
            c.bus.write32(DATA + 0xb0, r.next_u32());
        });
    }


    #[test]
    fn c_speed_boost_streamer_fx_reset_matches() {
        check(ssx3_ee::s09::f_002e6640, c_speed_boost_streamer_fx_reset, 150, &|c, seed| {
            let mut r = Rng(0x100a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(
                0x870,
                if seed % 2 == 0 { r.next_u32() } else { r.next_u32() | 0x8000_0000 },
            );
        });
    }


    pub(crate) const SEEDS: u32 = 240;


    /// A real guest function that returns 0 (`UIState_TransitionIn`), for
    /// vtable slots whose 0 answer a fake cannot give.
    pub(crate) const RETURN_ZERO: u32 = 0x003a_5ab0;


    /// Vtable slot `off` = `{delta, func}`.
    pub(crate) fn slot(c: &mut Cpu, vt: u32, off: u32, delta: u32, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, func);
    }


    /// A random `this` adjustment.
    pub(crate) fn delta(r: &mut Rng) -> u32 {
        (r.next_u32() as i16 as i32 & !3) as u32
    }


    /// The graphics manager at DATA+0x4000: texture handles and state
    /// words around +0xf00..+0x1100, a GS state stack with a random top,
    /// the default record, and its vtable with `Draw` (FAKE) and the camera
    /// matrix query 0x118 (FAKE+0x10).
    pub(crate) fn gfx_fixture(c: &mut Cpu, r: &mut Rng) {
        let (gfx, vt, stack) = (DATA + 0x4000, DATA + 0x6000, DATA + 0x6800);
        c.bus.write32(GAME_GP - GFX_MAN, gfx);
        fill(c, r, gfx + 0xf00, 0x80);
        c.bus.write32(gfx + GS_TOP, stack + RECORD * (r.next_u32() % 4));
        fill(c, r, stack, 0x30);
        fill(c, r, GS_DEFAULT, 5);
        c.bus.write32(gfx + GFX_VTABLE, vt);
        slot(c, vt, DRAW, delta(r), FAKE);
        slot(c, vt, 0x118, delta(r), FAKE + 0x10);
    }


    /// A streamer's rider (+0x780 model with bone tables at +0x30/+0x34,
    /// bone indices at `bone_a`/`bone_b`).
    pub(crate) fn streamer_rider(c: &mut Cpu, r: &mut Rng, rider: u32, bone_a: u32, bone_b: u32) {
        let (model, bones) = (rider + 0x1000, rider + 0x1100);
        c.bus.write32(rider + 0x780, model);
        c.bus.write32(rider + bone_a, r.next_u32() % 4);
        c.bus.write32(rider + bone_b, r.next_u32() % 4);
        c.bus.write32(model + 0x30, bones);
        c.bus.write32(model + 0x34, bones + 0x100);
        fill(c, r, bones, 0x80);
    }


    #[test]
    fn c_player_pulse_fx_render_matches() {
        covered(ssx3_ee::s09::f_002e3af8__generated, c_player_pulse_fx_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2e3a_f8, 4);
            gfx_fixture(c, &mut r);
            let (fx, rider, rider_vt) = (DATA, DATA + 0x100, DATA + 0x2400);
            let (app, module, ai, players, mode) =
                (DATA + 0x2500, DATA + 0x2600, DATA + 0x2700, DATA + 0x2800, DATA + 0x2900);
            let (iface, iface_vt, lib) = (DATA + 0x2a00, DATA + 0x2b00, DATA + 0x2c00);
            fill(c, &mut r, fx, 0x10);
            c.g[A0][0] = sx(fx);
            c.bus.write32(fx, rider);
            // The rider: "local player" query (0 or a fake), switches, bone.
            c.bus.write32(rider + 0x6c0, rider_vt);
            let query = if r.next_u32() % 2 == 0 { RETURN_ZERO } else { FAKE + 0x20 };
            slot(c, rider_vt, 0x40, delta(&mut r), query);
            c.bus.write32(rider + 0x870, r.next_u32() % 2);
            c.bus.write32(rider + 0x87c, r.next_u32() % 2);
            c.bus.write32(rider + 0xac4, (r.next_u32() % 4 == 0) as u32);
            c.bus.write32(rider + 0x8a8, r.next_u32() % 4);
            streamer_rider(c, &mut r, rider, 0x89c, 0x8a4);
            // The app: game module, players, mode object.
            c.bus.write32(GAME_GP - APP, app);
            c.bus.write32(app + 0x84, module);
            c.bus.write32(app + 0x78, r.next_u32());
            c.bus.write32(module + 0xc, ai);
            c.bus.write32(ai + 0x7c, 1 + seed % 2);
            c.bus.write32(module + 0x84, players);
            c.bus.write32(module + 0x78, mode);
            c.bus.write32(players + 0x10, r.next_u32() % 3);
            c.bus.write32(players + 0x14, r.next_u32() % 2);
            c.bus.write32(mode + 0x1bc, 0xd + r.next_u32() % 5);
            c.bus.write32(SPLIT_SWITCH, r.next_u32() % 2);
            c.bus.write8(GAME_MODE, (3 + r.next_u32() % 4) as u8);
            // BELibrary: built; the phase interface's method 8 is a fake.
            c.bus.write32(GAME_GP - 0x1f04, lib);
            c.bus.write32(GAME_GP - 0x1efc, 0);
            c.bus.write32(0x0053_08a8, iface);
            c.bus.write32(iface + 4, iface_vt);
            slot(c, iface_vt, 8, delta(&mut r), FAKE + 0x30);
            fill(c, &mut r, GAME_GP - 0x3a30, 8);
        });
    }


    /// The PS2 graphics manager `BeginTriStrip`/`EndTriStrip` run on (real
    /// callees): GS state, view matrices up to date (+0x6b90), the packet
    /// source (+0x5a00), the strip table (+0x18f0, full or nearly empty),
    /// and the scratchpad stream at gp+0x1384 (two slots, packets in RAM).
    pub(crate) fn graphics_man_fixture(c: &mut Cpu, r: &mut Rng) {
        let (gm, stack, table, stream) = (DATA + 0x2_0000, DATA + 0x7000, DATA + 0x4_0000, DATA + 0x4_8000);
        c.bus.write32(GAME_GP + GRAPHICS_MAN, gm);
        c.bus.write32(gm + GS_TOP, stack);
        fill(c, r, stack, 8);
        c.bus.write32(gm + 0x6b90, 1);
        c.bus.write32(gm + 0x5a00, DATA + 0x3_0000);
        c.bus.write32(gm + 0x18f0, table);
        c.bus.write32(table, if r.next_u32() % 2 == 0 { 3000 } else { r.next_u32() % 4 });
        c.bus.write32(GAME_GP + 0x1290, r.next_u32() % 2);
        c.bus.write32(GAME_GP + 0x1384, stream);
        let slot = r.next_u32() % 2;
        c.bus.write32(stream + 4, 2);
        c.bus.write32(stream + 8, slot);
        c.bus.write32(stream + 0xc, slot);
        c.bus.write32(stream + 0x10, r.next_u32() % 64);
        c.bus.write32(stream + 0x14, DATA + 0x5_0000);
        c.bus.write32(stream + 0x18, DATA + 0x5_1000);
    }


    #[test]
    fn c_speed_boost_streamer_fx_render_matches() {
        covered(ssx3_ee::s09::f_002e7a10__generated, c_speed_boost_streamer_fx_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2e7a_10, 2);
            gfx_fixture(c, &mut r);
            graphics_man_fixture(c, &mut r);
            let (fx, rider, points) = (DATA, DATA + 0x800, DATA + 0x2000);
            c.g[A0][0] = sx(fx);
            // Floats everywhere (the flame ring at +0x50..+0x398), then the
            // fields.
            for i in 0..0xec {
                c.bus.write32(fx + 4 * i, r.f32(4.0));
            }
            c.bus.write32(fx + boost::RIDER, rider);
            let count = if seed % 6 == 0 { 0 } else { 1 + r.next_u32() % 21 };
            c.bus.write32(fx + boost::COUNT, count);
            let age = if seed % 7 == 1 { 1.0f32 } else { 3.0 + (r.next_u32() % 8) as f32 };
            c.bus.write32(fx + boost::FADE, age.to_bits());
            c.bus.write32(fx + boost::FLAME_CLAMP, (0.5 + (r.next_u32() % 8) as f32).to_bits());
            c.bus.write32(fx + boost::ALPHA, r.f32(1.0));
            c.bus.write32(GAME_GP - 0x39d8, (-0.5f32).to_bits());
            c.bus.write32(GAME_GP + 0x1634, (seed % 5 != 2) as u32);
            c.bus.write32(fx + boost::TEXTURE, r.next_u32() % 8);
            c.bus.write32(fx + boost::FIRST, r.next_u32() % 0x14);
            c.bus.write32(fx + boost::POINTS, points);
            fill(c, &mut r, points, 0x14 * 8);
            let flames = if seed % 4 == 0 { r.next_u32() % 2 } else { 2 + r.next_u32() % 14 };
            c.bus.write32(fx + boost::FLAMES, flames);
            c.bus.write32(fx + boost::FLAME_HEAD, r.next_u32() % 0x1e);
            streamer_rider(c, &mut r, rider, 0x89c, 0x8a4);
            fill(c, &mut r, rider + 0x1a0, 4);
            c.bus.write32(GAME_GP - 0x39d4, r.f32(8.0));
        });
    }


    pub(crate) const SEEDS_2: u32 = 250;


    pub(crate) fn f(x: f32) -> u32 {
        x.to_bits()
    }


    /// Scrambled registers and the game's gp.
    pub(crate) fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }


    pub(crate) fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }


    #[test]
    fn snow_box_wrap_offset_matches() {
        check(ee::s09::f_002e5430__generated, snow_box_wrap_offset, SEEDS_2, &|c, seed| {
            let mut r = base(c, seed, 0x2e54);
            let (b, d) = (DATA, DATA + 0x40);
            let size = 1.0 + (r.next_u32() % 200) as f32 * 0.25;
            let h = size * 0.5;
            c.bus.write32(b + 0xc, f(size));
            // One position per component with its own wrap case (in range,
            // above +h, below -h), so the seeds cover every branch triple.
            for k in 0..3u32 {
                let case = (seed / (3u32.pow(k))) % 3;
                let v = match case {
                    0 => (r.next_u32() as f32 / u32::MAX as f32 - 0.5) * h * 1.5,
                    1 => h + 1.0 + (r.next_u32() % 10) as f32,
                    _ => -h - 1.0 - (r.next_u32() % 10) as f32,
                };
                c.bus.write32(b + 0x20 + 4 * k, f(v));
                c.bus.write32(d + 4 * k, 0);
            }
            c.bus.write32(d + 0xc, f(0.0));
            c.g[A0][0] = sx(b);
            c.g[A1][0] = sx(d);
        });
    }


    #[test]
    fn dynamic_spray_fx_add_random_velocity_matches() {
        check(ee::s09::f_002de398__generated, dynamic_spray_fx_add_random_velocity, SEEDS_2, &|c, seed| {
            let mut r = base(c, seed, 0x2de3);
            let (fx, out) = (DATA, DATA + 0x100);
            for k in 0..4u32 {
                c.bus.write32(fx + 0x40 + 4 * k, r.edge_f32());
                c.bus.write32(fx + 0x50 + 4 * k, r.edge_f32());
                c.bus.write32(out + 4 * k, r.edge_f32());
            }
            c.bus.write32(fx + 0x12c, r.edge_f32());
            c.bus.write32(gp_at(0xa0c), r.next_u32());
            c.g[A0][0] = sx(fx);
            c.g[A1][0] = sx(out);
        });
    }


    #[test]
    fn board_wake_fx_render_matches() {
        check(ee::s09::f_002ddab8__generated, board_wake_fx_render, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x2dda, 1);
            let (w, gfx, state, vt, verts) =
                (DATA, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000, DATA + 0x4000);
            // The renderer and its state stack.
            c.bus.write32(gp_at(-0x854), gfx);
            c.bus.write32(gfx + 0xe84, state);
            c.bus.write32(gfx + 0x1030, r.next_u32());
            c.bus.write32(gfx + 0x10d8, vt);
            c.bus.write16(vt + 0x2d0, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0x2d4, FAKE);
            // The default state the renderer copies.
            for k in 0..5u32 {
                c.bus.write32(0x0050_1420 + 4 * k, r.next_u32());
            }
            // The wake trail.
            c.bus.write32(w + 0x4, 1 + r.next_u32() % 4);
            c.bus.write32(w + 0x8, r.next_u32() % 4);
            c.bus.write32(w + 0xc, verts);
            c.bus.write32(w + 0xb4, r.next_u32());
            c.bus.write32(w + 0xb8, 2 + r.next_u32() % 3);
            c.bus.write32(w + 0xbc, f(0.5 + (r.next_u32() % 100) as f32 * 0.1));
            for k in 0..4u32 {
                c.bus.write32(verts + 0x70 * k + 0x64, f((r.next_u32() % 100) as f32 * 0.01));
            }
            // Cover the early-outs too.
            if seed % 11 == 0 {
                c.bus.write32(w + 0x4, 0);
            }
            c.bus.write32(gp_at(0x1438), if seed % 13 == 0 { 1 } else { 0 });
            c.g[A0][0] = sx(w);
        });
    }


    /// A dynamic emitter with four-slot rings and the timing fields.
    pub(crate) fn put_spray_emitter(c: &mut Cpu, r: &mut Rng, e: u32) {
        c.bus.write32(e + 0x174, 0);
        c.bus.write32(e + 0x178, 4);
        c.bus.write32(e + 0x17c, r.next_u32() % 4);
        c.bus.write32(e + 0x1a0, DATA + 0x6000);
        c.bus.write32(e + 0x1a4, DATA + 0x7000);
        c.bus.write32(e + 0x204, DATA + 0x7800);
        c.bus.write32(e + 0x0c, 1);
        c.bus.write32(e + 0x10, f(0.0));
        c.bus.write32(e + 0x14, f(1.0));
        c.bus.write32(e + 0x1e0, 0);
        for k in 0..4u32 {
            c.bus.write32(e + 0x1b0 + 4 * k, r.edge_f32());
            c.bus.write32(e + 0x1c0 + 4 * k, r.edge_f32());
            c.bus.write32(e + 0x1d0 + 4 * k, r.edge_f32());
        }
    }


    #[test]
    fn dynamic_spray_fx_update_emitter0_matches() {
        check(ee::s09::f_002e0ee8__generated, dynamic_spray_fx_update_emitter0, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x2e0e, 1);
            let (fx, obj, vt, curves, cfg, em) =
                (DATA, DATA + 0x4000, DATA + 0x4300, DATA + 0x4400, DATA + 0x4500, DATA + 0x5000);
            put_spray_emitter(c, &mut r, em);
            c.bus.write32(fx + 0x00, obj);
            c.bus.write32(fx + 0x28, em);
            c.bus.write32(fx + 0x2c, curves);
            for k in 0..4u32 {
                c.bus.write32(fx + 0x30 + 4 * k, r.edge_f32());
                c.bus.write32(fx + 0x90 + 4 * k, r.edge_f32());
                c.bus.write32(fx + 0xa0 + 4 * k, r.edge_f32());
                c.bus.write32(obj + 0x370 + 4 * k, r.edge_f32());
            }
            for k in 0..3u32 {
                c.bus.write32(curves + 0xd8 + 4 * k, r.edge_f32());
            }
            c.bus.write32(fx + 0xbc, f(0.2 + (r.next_u32() % 160) as f32 * 0.01));
            c.bus.write32(fx + 0xd0, cfg);
            c.bus.write32(cfg + 0x58, if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(obj + 0x88c, obj + 0x4200);
            c.bus.write32(obj + 0x4200 + 0xa0, if seed % 5 == 0 { 1 } else { 0 });
            c.bus.write32(obj + 0x6c0, vt);
            c.bus.write16(vt + 0x10, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0x14, FAKE);
            c.bus.write32(gp_at(-0x3afc), f(1.0));
            c.bus.write32(gp_at(-0x3af8), f(1.0 / 60.0));
            c.bus.write32(gp_at(-0x3af4), f(1.0 / 60.0));
            if seed % 7 == 0 {
                c.bus.write32(fx + 0xb0, 0);
            }
            c.g[A0][0] = sx(fx);
        });
    }


    #[test]
    fn snow_box_render_matches() {
        check(ee::s09::f_002e55d8__generated, snow_box_render, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x2e55, 2);
            let (cell, mat, gfx, vt) = (DATA, DATA + 0x2000, DATA + 0x1000, DATA + 0x3000);
            let size = 0.5 + (seed % 3) as f32 * 0.25;
            c.bus.write32(cell + 0x4, (seed / 3) % 4);
            c.bus.write32(cell + 0x8, if seed % 9 == 0 { 0 } else { 1 });
            c.bus.write32(cell + 0xc, f(size));
            // The cell-local position, one of in-range / below / above per axis.
            let mut xyz = [0.0f32; 3];
            for k in 0..3u32 {
                let local = (r.next_u32() as f32 / u32::MAX as f32 - 0.5) * 8.0;
                let case = (seed / (4u32.pow(k))) % 3;
                xyz[k as usize] = match case {
                    0 => local,
                    1 => local + 1.0,
                    _ => local - 2.0,
                };
                c.bus.write32(cell + 0x20 + 4 * k, f(local * size));
            }
            c.f[12] = f(xyz[0]);
            c.f[13] = f(xyz[1]);
            c.f[14] = f(xyz[2]);
            for k in 0..16u32 {
                c.bus.write32(mat + 4 * k, r.edge_f32());
                c.bus.write32(cell + 0x50 + 4 * k, r.edge_f32());
            }
            c.bus.write32(cell + 0x14, r.edge_f32());
            c.bus.write32(cell + 0x18, r.edge_f32());
            for off in [0x98u32, 0x9c, 0xa0, 0xa4, 0xa8, 0xac] {
                c.bus.write32(cell + off, r.edge_f32());
            }
            c.bus.write32(gp_at(-0x854), gfx);
            c.bus.write32(gfx + 0x10d8, vt);
            c.bus.write16(vt + 0x2a8, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0x2ac, FAKE);
            c.bus.write16(vt + 0x2b0, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0x2b4, FAKE);
            c.g[A0][0] = sx(cell);
            c.g[A1][0] = sx(mat);
        });
    }


    /// A spray emitter slot at `e` (the fields `DynamicEmitter_SpawnAt`
    /// reads), with the particle ring disabled so it only writes the colour.
    pub(crate) fn put_spray_slot(c: &mut Cpu, r: &mut Rng, e: u32, colours: u32) {
        c.bus.write32(e + 0x174, 0);
        c.bus.write32(e + 0x178, 4);
        c.bus.write32(e + 0x17c, r.next_u32() % 4);
        c.bus.write32(e + 0x1a0, DATA + 0x7000);
        c.bus.write32(e + 0x1a4, DATA + 0x7400);
        c.bus.write32(e + 0x204, colours);
        c.bus.write32(e + 0x0c, 1);
        c.bus.write32(e + 0x10, f(0.0));
        c.bus.write32(e + 0x14, f(1.0));
        c.bus.write32(e + 0x1e0, 0);
        for k in 0..4u32 {
            c.bus.write32(e + 0x1b0 + 4 * k, r.edge_f32());
            c.bus.write32(e + 0x1c0 + 4 * k, r.edge_f32());
            c.bus.write32(e + 0x1d0 + 4 * k, r.edge_f32());
        }
    }


    #[test]
    fn dynamic_spray_fx_update_emitter5_matches() {
        check(ee::s09::f_002e1598__generated, dynamic_spray_fx_update_emitter5, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x2e15, 2);
            let (fx, obj, cfg, em, vt, world, g, players) = (
                DATA,
                DATA + 0x5000,
                DATA + 0x5200,
                DATA + 0x3000,
                DATA + 0x5400,
                DATA + 0x6000,
                DATA + 0x6200,
                DATA + 0x6400,
            );
            c.bus.write32(fx + spray5::OBJ, obj);
            c.bus.write32(fx + spray5::LEVEL, f([0.5f32, 1.2, 2.5][(seed % 3) as usize]));
            c.bus.write32(fx + spray5::P28, em);
            c.bus.write32(fx + spray5::P2C, DATA + 0x5600);
            for k in 0..3u32 {
                c.bus.write32(DATA + 0x5600 + 0xd8 + 4 * k, r.edge_f32());
            }
            for k in 0..4u32 {
                c.bus.write32(fx + spray5::DIR + 4 * k, r.edge_f32());
                c.bus.write32(fx + spray5::COL + 4 * k, r.edge_f32());
                c.bus.write32(fx + spray5::POS + 4 * k, r.edge_f32());
                c.bus.write32(fx + spray5::FRAME_POS + 4 * k, r.edge_f32());
                c.bus.write32(fx + spray5::THROW + 4 * k, r.edge_f32());
                c.bus.write32(obj + 0x370 + 4 * k, r.edge_f32());
            }
            c.bus.write32(fx + spray5::CFG, cfg);
            c.bus.write32(cfg + spray_cfg::ENABLED, if seed % 9 == 0 { 0 } else { 1 });
            c.bus.write32(cfg + spray_cfg::FLAG, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(cfg + spray_cfg::DECAY, f(0.5 + (r.next_u32() % 50) as f32 * 0.01));
            c.bus.write32(fx + spray5::ID4, [0u32, 2, 3][(seed % 3) as usize]);
            c.bus.write32(fx + spray5::SPEED, f([5.0f32, 100.0, 500.0][(seed % 3) as usize]));
            c.bus.write32(fx + spray5::I114, (seed % 2) as u32);
            c.bus.write32(fx + spray5::ALPHA, f([0.1f32, 0.75][(seed % 2) as usize]));
            c.bus.write32(fx + spray5::I124, ((seed / 3) % 2) as u32);
            c.bus.write32(obj + 0xac4, if seed % 7 == 0 { 0 } else { 1 });
            c.bus.write32(obj + 0x6c0, vt);
            c.bus.write16(vt + 0x10, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0x14, FAKE);
            put_spray_slot(c, &mut r, em + 5 * 0x210, DATA + 0x8000);
            put_spray_slot(c, &mut r, em + 6 * 0x210, DATA + 0x8100);
            c.bus.write32(gp_at(-0x848), world);
            c.bus.write32(world + 0x84, g);
            c.bus.write32(g + 0x84, players);
            c.bus.write32(players + 0x10, 0);
            c.bus.write32(gp_at(0xa0c), r.next_u32());
            for (off, v) in [
                (-0x3abc, 27.777778f32), (-0x3ab8, 277.77777), (-0x3ab4, 3333.3335),
                (-0x3ab0, 0.3), (-0x3aac, 0.4), (-0x3aa8, 0.8), (-0x3aa4, 0.2),
                (-0x3aa0, 1.0 / 60.0), (-0x3a9c, 83.333336), (-0x3a98, 0.4),
            ] {
                c.bus.write32(gp_at(off), f(v));
            }
            c.g[A0][0] = sx(fx);
        });
    }


    pub(crate) const SEEDS_3: u32 = 160;


    /// Fixed fixture areas, above the range `fakes::ptr` hands out (which is
    /// `DATA..DATA+0x100000`) and clear of the fake call log at 0x900000.
    pub(crate) const CTX: u32 = DATA + 0x20_0000;

    pub(crate) const TOP: u32 = DATA + 0x22_0000;

    pub(crate) const TEX: u32 = DATA + 0x23_0000;

    pub(crate) const WORLD: u32 = DATA + 0x24_0000;

    pub(crate) const BASE: u32 = DATA + 0x30_0000;

    pub(crate) const EXTRA: u32 = DATA + 0x50_0000;


    pub(crate) fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        f(lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo))
    }


    /// A render context object (vtable, render-state stack and a few random
    /// state words).
    pub(crate) fn ctx_at(c: &mut Cpu, r: &mut Rng, obj: u32, top: u32, slots: &[u32]) -> u32 {
        let vt = vtable(c, r, slots);
        c.bus.write32(obj + ctx::VTABLE, vt);
        c.bus.write32(obj + ctx::TOP, top);
        for i in 0..8 {
            c.bus.write32(top + 4 * i, r.next_u32());
        }
        vt
    }


    /// `gp+0x2a90` (the game's render context) with a texture table.
    pub(crate) fn put_ctx(c: &mut Cpu, r: &mut Rng, slots: &[u32]) -> u32 {
        c.bus.write32(GAME_GP + 0x2a90, CTX);
        c.bus.write32(CTX + ctx::TEX_IDS, TEX);
        ctx_at(c, r, CTX, TOP, slots)
    }


    /// The world chain `gp-0x848 -> +0x84 -> +0x84 -> +0x14`.
    pub(crate) fn put_world(c: &mut Cpu, r: &mut Rng, rider: u32) -> u32 {
        let (wm, race) = (WORLD, WORLD + 0x1000);
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, wm + 0x100);
        c.bus.write32(wm + 0x100 + 0x84, race);
        c.bus.write32(race + 0x14, rider);
        let _ = r;
        rider
    }


    #[test]
    fn player_pulsefx_update_matches() {
        check(ee::s09::f_002e39d8__generated, player_pulsefx_update, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x39d8, 0);
            let pfx = BASE;
            let rider = BASE + 0x1_0000;
            let vt = BASE + 0x2_0000;
            c.bus.write32(pfx, rider);
            c.bus.write32(rider + 0x6c0, vt);
            if seed % 4 == 0 {
                // A real function that returns 0 (its argument word is zero).
                c.bus.write16(vt + 0x40, 0x100);
                c.bus.write32(vt + 0x44, 0x002c_c098);
                c.bus.write32(rider + 0x7c0, 0);
            } else {
                c.bus.write16(vt + 0x40, 0);
                c.bus.write32(vt + 0x44, FAKE);
            }
            c.bus.write32(pfx + 4, uf(&mut r, -10.0, 10.0));
            for i in 0..8 {
                c.bus.write32(pfx + 8 + 4 * i, uf(&mut r, -2.0, 2.0));
            }
            // The pulse rate and the angle constants.
            c.bus.write32(GAME_GP + 0x24bc, uf(&mut r, 0.0, 20.0));
            c.bus.write32(GAME_GP - 0x3a30, f(1.0 / 60.0));
            c.bus.write32(GAME_GP - 0x3a2c, f(0.159_154_94));
            c.bus.write32(GAME_GP - 0x3a28, f(6.283_185_5));
            c.g[A0][0] = sx(pfx);
        });
    }


    #[test]
    fn transition_man_render_matches() {
        check(ee::s09::f_002e47e8__generated, transition_man_render, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x47e8, 7);
            put_ctx(c, &mut r, &[0x1e8, 0xa8, 0xf8, 0x128, 0xd0, 0x100, 0xb0, 0x150]);
            let trans = BASE;
            let rider = 7;
            let mismatch = seed % 8 == 0;
            put_world(c, &mut r, if mismatch { rider + 1 } else { rider });
            c.bus.write32(trans + 0x74, rider);
            c.bus.write32(trans + 0x44, (seed % 8 != 1) as u32);
            // Fade objects with a `vt[2]` entry.
            let o1 = BASE + 0x1_0000;
            let o2 = BASE + 0x2_0000;
            let vt = vtable(c, &mut r, &[0x10]);
            c.bus.write32(o1, vt);
            let vt = vtable(c, &mut r, &[0x10]);
            c.bus.write32(o2, vt);
            // time / fadeIn / hold / fadeOut chosen to take every branch.
            let (time, fade_in, hold, fade_out, use_o1, use_o2) = match seed % 5 {
                0 => (0.5, 1.0, 2.0, 1.0, false, true),  // o1 == 0
                1 => (0.5, 1.0, 2.0, 1.0, true, true),   // t < fadeIn
                2 => (1.5, 1.0, 2.0, 1.0, true, false),  // o2 == 0
                3 => (1.5, 1.0, 2.0, 1.0, true, true),   // hold
                _ => (10.0, 1.0, 2.0, 1.0, true, true),  // fade out (clamped)
            };
            c.bus.write32(trans + 0x48, f(time));
            c.bus.write32(trans + 0x4c, if use_o1 { o1 } else { 0 });
            c.bus.write32(trans + 0x50, if use_o2 { o2 } else { 0 });
            c.bus.write32(trans + 0x54, f(fade_in));
            c.bus.write32(trans + 0x58, f(hold));
            c.bus.write32(trans + 0x5c, f(fade_out));
            c.bus.write32(trans + 0x64, (seed % 3 == 0) as u32);
            c.bus.write32(trans + 0x68, EXTRA);
            c.bus.write32(trans + 0x60, (seed % 2) as u32);
            c.bus.write32(trans + 0x6c, r.next_u32() % 0x1000);
            for off in [0x70u32, 0x78, 0x7c, 0x80] {
                c.bus.write32(trans + off, r.next_u32());
            }
            // The near-opaque threshold (0 or 2.0).
            c.bus.write32(
                GAME_GP - 0x3a1c,
                if seed % 4 == 0 { 0 } else { f(2.0) },
            );
            c.g[A0][0] = sx(trans);
        });
    }


    #[test]
    fn dynamic_spray_fx_render_matches() {
        check(ssx3_ee::s09::f_002e24d0__generated, dynamic_spray_fx_render, 150, &|c, seed| {
            machine(c, seed, 53, 0);
            let spray = DATA + 0x1000;
            let owner = DATA + 0x2000;
            c.bus.write32(spray, owner);
            c.bus.write32(owner + 0xb18, if seed % 4 == 0 { 0 } else { 1 });
            let wakes = DATA + 0x3000;
            c.bus.write32(spray + 0x28, wakes);
            for i in 0..10u32 {
                let w = wakes + i * 0x210;
                match (seed + i) % 4 {
                    0 => {}
                    1 => {
                        c.bus.write32(w + 0x174, 1);
                    }
                    2 => {
                        c.bus.write32(w + 0x174, 1);
                        c.bus.write32(w + 0x1e0, 0xffff_ffff);
                    }
                    _ => {
                        c.bus.write32(w + 0x174, 1);
                        c.bus.write32(w + 0x1e0, 1);
                    }
                }
            }
            // mEmitter_Render leaves at_2 its negative table entry.
            let g = DATA + 0x6000;
            c.bus.write32(GAME_GP + 0x2a90, g);
            for i in 0..0x20u32 {
                c.bus.write32(g + 0xf50 + i * 4, 0xffff_ffff);
            }
            c.g[A0][0] = sx(spray);
        });
    }


    // --------------------------------------------------- cTransitionMan_Update

    #[test]
    fn c_transition_man_update_matches() {
        fakes::covered(ee::s09::f_002e46c8__generated, c_transition_man_update, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x0e50_0107, 2);
            let this = DATA + 0x1000;
            let ctx = DATA + 0x2000;
            let vt = DATA + 0x3000;
            c.g[A0][0] = this as u64;
            c.bus.write32(GAME_GP.wrapping_add(0x2a90), ctx);
            c.bus.write32(ctx + 0x10d8, vt);
            c.bus.write16(vt + 0x390, 0x10);
            c.bus.write32(vt + 0x394, FAKE);
            c.bus.write16(vt + 0x1e0, 0x18);
            c.bus.write32(vt + 0x1e4, FAKE + 0x10);
            // The effect clock returns DATA+0x8000; +0x80 is within three of it
            // on even seeds (fade over) and further away on odd ones.
            c.bus.write32(LOG_RESULTS, DATA + 0x8000);
            let start = if seed % 2 == 0 { DATA + 0x8000 - 1 } else { DATA + 0x8000 - 8 };
            c.bus.write32(this + 0x80, start);
            c.bus.write32(this + 0x7c, if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(this + 0x44, if seed % 5 == 0 { 0 } else { 1 });
            c.bus.write32(this + 0x40, if seed % 7 == 0 { 1 } else { 0 });
            c.bus.write32(this + 0x18, match seed % 4 { 0 => 0, 1 => 1, _ => r.next_u32() as i32 as u32 });
            c.bus.write32(this + 0x70, if seed % 3 == 0 { 0 } else { DATA + 0x9000 });
            c.bus.write32(DATA + 0x9000, r.edge_f32());
            for off in [0x3c, 0x48, 0x54, 0x58, 0x5c] {
                c.bus.write32(this + off, r.edge_f32());
            }
            // The frame-rate state (D_004A5B64 -> +0x10 = the frame time).
            let app = DATA + 0xa000;
            c.bus.write32(GAME_GP.wrapping_add(0x2a74), app);
            c.bus.write32(app + 0x10, (seed % 61 + 1) as u32);
            // Keep cTransitionMan_Stop on its short path (no further calls).
            c.bus.write32(this + 0x4c, 0);
            c.bus.write32(this + 0x50, 0);
            c.bus.write32(this + 0x6c, 0xffff_ffff);
        });
    }


    #[test]
    fn snow_box_dtor_matches() {
        check(ee::s09::f_002e4f28__generated, snow_box_dtor, 60, &|c, seed| dtor_setup(c, seed, 0x2e4f28));
    }


    #[test]
    fn c_transition_man_script_deactivate_matches() {
        check(
            ee::s09::f_002e4c60__generated,
            c_transition_man_script_deactivate,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e4c60, 1);
                c.g[A0][0] = DATA as u64;
                // No transition in flight: cTransitionMan_Stop returns at once.
                c.bus.write32(DATA + 0x44, 0);
                c.bus.write32(DATA + 0x10 + 8, r.next_u32());
            },
        );
    }


    #[test]
    fn c_transition_man_reset_effect_pool_matches() {
        check(ee::s09::f_002e4cb0__generated, c_transition_man_reset_effect_pool, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2e4c_b0, 0);
            fill(c, &mut r, 0x0053_8850, 16);
        });
    }


    // -----------------------------------------------------------------------
    // cTransitionMan_StartCaptured.

    #[test]
    fn c_transition_man_start_captured_matches() {
        check(ssx3_ee::s09::f_002e4540__generated, c_transition_man_start_captured, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x2e45_40, 0);
            // A zeroed manager: `+0x44` is clear (no Stop), `t0` is 1 and the
            // incoming `a2` is 0, so no vtable is reached.
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ptr(&mut r) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = ptr(&mut r) as u64;
            c.f[12] = r.edge_f32();
        });
    }


    #[test]
    fn transition_effect_free_data_matches() {
        check(
            ee::s09::f_002e4d30__generated,
            transition_effect_free_data,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x2e4d30, 0);
                const BASE: u32 = 0x0053_82c0;
                let choices = [0u32, BASE, BASE + 0x64, BASE + 0x64 * 3, 0x1234_5678];
                c.g[A0][0] = choices[(seed as usize) % choices.len()] as u64;
            },
        );
    }


    #[test]
    fn script_obj_unregister_matches() {
        covered(
            ee::s07::f_002832d8__generated,
            script_obj_unregister,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_32d8, 1);
                let obj = DATA + 0x1000;
                let engine = DATA + 0x2000;
                // `ScriptEngine_GetInstance` reads the singleton at gp+0x3ec.
                c.bus.write32(GAME_GP + 0x3ec, engine);
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(engine + 0x2a8, vt);
                c.bus.write32(obj + 4, r.next_u32());
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (r.next_u32() % 8) as u64;
            },
        );
    }


    #[test]
    fn transition_effect_alloc_data_matches() {
        check(
            ee::s09::f_002e4ce8__generated,
            transition_effect_alloc_data,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e4c_e8, 0);
                let busy = seed % 5;
                for i in 0..4u32 {
                    let value = if busy == 1 {
                        1
                    } else if busy >= 2 && i < busy {
                        1
                    } else {
                        0
                    };
                    c.bus.write32(0x0053_8850 + 4 * i, value);
                }
                c.bus.write32(GAME_GP + 0xa2c, r.next_u32());
            },
        );
    }


    // ------------------------------------------------------------------
    // Transitions and lights.

    #[test]
    fn c_transition_man_start_out_matches() {
        check(
            ee::s09::f_002e44f0__generated,
            c_transition_man_start_out,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e44_f0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x300);
                c.g[A2][0] = (seed % 4) as u64;
                c.f[12] = r.edge_f32();
                c.bus.write32(this + 0x44, if seed % 2 == 0 { 0 } else { 1 });
                clear(c, DATA + 0x300, 0x40);
            },
        );
    }


    #[test]
    fn snow_fall_reset_boxes_matches() {
        check(
            ee::s09::f_002e62c8__generated,
            snow_fall_reset_boxes,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x2e62c8, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x100);
                for i in 0..12u32 {
                    let p = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        DATA + 0x2000 + 0x100 * i
                    };
                    c.bus.write32(this + 0x10 + 4 * i, p);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }


    // ------------------------------------------------------------------
    // Lens FX.

    #[test]
    fn lens_fx_base_push_render_state_matches() {
        check(ee::s09::f_002e3578__generated, lens_fx_base_push_render_state, 200, &|c, seed| {
            let mut r = setup(c, seed, 37);
            gpf(c, 0x2a90, AUX + 0x100);
            put(c, AUX + 0x100 + 0xe84, AUX + 0x400);
            fillb(c, &mut r, AUX + 0x400, 0x40);
            fillb(c, &mut r, 0x0050_1420, 0x40);
        });
    }


    #[test]
    fn lens_fx_base_pop_render_state_matches() {
        check(ee::s09::f_002e3668__generated, lens_fx_base_pop_render_state, 200, &|c, seed| {
            let mut r = setup(c, seed, 38);
            gpf(c, -0x854, AUX);
            put(c, AUX + 0xe84, r.next_u32());
        });
    }


    #[test]
    fn c_transition_man_script_is_running_matches() {
        check(ee::s09::f_002e4c90__generated, c_transition_man_script_is_running, 200, &|c, seed| {
            let mut r = setup(c, seed, 39);
            c.g[A0][0] = AUX as u64;
            put(c, AUX + 0x60, if seed % 2 == 0 { 0 } else { AUX + 0x200 });
            put(c, AUX + 0x64, if seed % 4 == 1 { 0 } else { r.next_u32() | 1 });
        });
    }


    pub(crate) fn put_f32s(c: &mut Cpu, a: u32, v: [f32; 4]) {
        crate::difftest::put_vec(
            c,
            a,
            [v[0].to_bits(), v[1].to_bits(), v[2].to_bits(), v[3].to_bits()],
        );
    }


    pub(crate) fn put_spray_fx(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        let fx = DATA;
        let owner = DATA + 0x2000;
        let vt = fakes::vtable(c, r, &[0x10]);
        c.bus.write32(fx, owner);
        c.bus.write32(fx + 0x28, DATA + 0x3000);
        c.bus.write32(owner + 0x6c0, vt);
        c.bus.write32(fx + 0xd0, DATA + 0x4000);
        c.bus.write32(fx + 0x2c, DATA + 0x5000);
        // Random geometry and scales (both emitters read the same fields).
        for off in [0x30, 0xa0, 0xc0] {
            put_f32s(c, fx + off, [f32::from_bits(r.f32(10.0)), f32::from_bits(r.f32(10.0)), f32::from_bits(r.f32(10.0)), f32::from_bits(r.f32(1.0))]);
        }
        for off in [0xb4, 0xb8, 0xbc] {
            c.bus.write32(fx + off, r.f32(2.0));
        }
        for off in [0x390, 0x394, 0x398, 0x39c] {
            c.bus.write32(DATA + 0x5000 + off, r.f32(5.0));
        }
        c.bus.write32(DATA + 0x5000 + 0x214, r.f32(5.0));
        c.bus.write32(owner + 0x214, r.f32(5.0));
        for off in [0x370, 0x3b0] {
            put_f32s(c, owner + off, [f32::from_bits(r.f32(4.0)), f32::from_bits(r.f32(4.0)), f32::from_bits(r.f32(4.0)), 0.0]);
        }
        match seed % 6 {
            // No timer: the default spawn only.
            0 => c.bus.write32(fx + 0xb0, 0),
            // Timer still running: the default spawn only.
            1 => {
                c.bus.write32(fx + 0xb0, r.f32(1.0));
                c.bus.write32(DATA + 0x406c, (-2.0f32).to_bits());
            }
            // The elaborate spawn.
            2 => {
                c.bus.write32(fx + 0xb0, r.f32(1.0));
                c.bus.write32(DATA + 0x406c, 2.0f32.to_bits());
            }
            _ => {
                c.bus.write32(fx + 0xb0, r.f32(1.0));
                c.bus.write32(DATA + 0x406c, (f32::from_bits(r.f32(1.0)) + 2.0).to_bits());
            }
        }
        fx
    }


    #[test]
    fn dynamic_spray_update_emitter3_matches() {
        covered(
            ssx3_ee::s09::f_002dfe88__generated,
            dynamic_spray_update_emitter3,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x2dfe, 1);
                c.pc = 0x002d_fe88;
                let fx = put_spray_fx(c, &mut r, seed);
                c.g[A0][0] = fx as u64;
            },
        );
    }


    #[test]
    fn dynamic_spray_update_emitter4_matches() {
        covered(
            ssx3_ee::s09::f_002e1120__generated,
            dynamic_spray_update_emitter4,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x2e11, 2);
                c.pc = 0x002e_1120;
                let fx = DATA;
                let owner = DATA + 0x2000;
                let ctrl = DATA + 0x6000;
                let ctrl2 = DATA + 0x6400;
                let vt = fakes::vtable(c, &mut r, &[0x10, 0x38]);
                c.g[A0][0] = fx as u64;
                c.bus.write32(fx, owner);
                c.bus.write32(fx + 0x28, DATA + 0x3000);
                c.bus.write32(fx + 0x2c, DATA + 0x4000);
                c.bus.write32(owner + 0x780, ctrl);
                c.bus.write32(ctrl + 0x30, DATA + 0x6200);
                c.bus.write32(owner + 0x8a8, r.next_u32() % 4);
                c.bus.write32(owner + 0x784, ctrl2);
                c.bus.write32(ctrl2 + 8, 0x1b6);
                c.bus.write32(owner + 0x6c0, vt);
                for off in [0x370, 0x3b0] {
                    put_f32s(c, owner + off, [f32::from_bits(r.f32(4.0)), f32::from_bits(r.f32(4.0)), f32::from_bits(r.f32(4.0)), 0.0]);
                }
                for off in [0x478, 0x47c, 0x480] {
                    c.bus.write32(DATA + 0x4000 + off, r.f32(5.0));
                }
                c.bus.write32(fx + 0xbc, 500.0f32.to_bits());
                c.bus.write32(fx + 0x1c, 0.1f32.to_bits());
                c.bus.write32(fx + 0x20, 0);
                c.bus.write32(fx + 0x24, 0);
                match seed % 6 {
                    // Too far: the entry picks the default spawn.
                    0 => c.bus.write32(fx + 0xbc, 2000.0f32.to_bits()),
                    // Wobble timer spent: default spawn after the timers.
                    1 => {
                        c.bus.write32(fx + 0x18, 1);
                        c.bus.write32(fx + 0x14, 1.0f32.to_bits());
                    }
                    // Rate below one: default spawn from the rate gate.
                    2 => c.bus.write32(fx + 0x14, 0.0f32.to_bits()),
                    // The full spawn.
                    3 => {
                        c.bus.write32(fx + 0x14, 1.0f32.to_bits());
                        c.bus.write32(fx + 0x1c, r.f32(0.5));
                    }
                    _ => {
                        c.bus.write32(fx + 0x14, 1.0f32.to_bits());
                        c.bus.write32(fx + 0x18, r.next_u32() % 2);
                        c.bus.write32(fx + 0x1c, r.f32(0.5));
                    }
                }
            },
        );
    }


    #[test]
    fn lens_fx_base_dtor_matches() {
        check(ee::s09::f_002e3060__generated, lens_fx_base_dtor, 40, &|c, seed| {
            alloc_base(c, seed, 0x2e30_60);
            let obj = alloc(c, 0x6500);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = [0u64, 2, 1][seed as usize % 3];
            let man = DATA + 0x1000;
            let vt = DATA + 0x1200;
            c.bus.write32(GAME_GP + 0x2a90, man);
            c.bus.write32(man + 0x10d8, vt);
            put_vtable(c, vt, &[0x3a8]);
            c.bus.write32(GAME_GP + 0xa28, 0x1122_3344);
        });
    }


    #[test]
    fn lens_fx_get_flare_size_for_res_matches() {
        check(ee::s09::f_002e27e8__generated, lens_fx_get_flare_size_for_res, SEEDS_8, &|c, seed| {
            machine(c, seed, 0x2e_27e8, 0);
            c.g[A0][0] = match seed % 7 {
                0 => 0x20,
                1 => 0x10,
                2 => 0x40,
                3 => 0x30,
                4 => 33,
                5 => 0,
                _ => 0xffff_ffff_ffff_ffff,
            };
            c.pc = 0x002e_27e8;
        });
    }


    #[test]
    fn c_dynamic_spray_fx_setup_emitters_matches() {
        check(
            ee::s09::f_002df448__generated,
            c_dynamic_spray_fx_setup_emitters,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x2df448, 2);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x28, DATA + 0x1000);
                c.bus.write32(DATA + 0x2c, DATA + 0x2000);
                // Each of the nine records has an emitter object with a vtable slot
                // at_5 `+0x1f8`; point them all at_5 one fake-slot table.
                for i in 0..9u32 {
                    let rec = DATA + 0x1000 + 0x210 * i;
                    c.bus.write32(rec + 0x1f8, DATA + 0x3000);
                }
                c.bus.write16(DATA + 0x3000 + 0x18, 0);
                c.bus.write32(DATA + 0x3000 + 0x1c, FAKE);
            },
        );
    }


    #[test]
    fn snow_fall_dtor_matches() {
        check(ee::s09::f_002e5d18__generated, snow_fall_dtor, 40, &|c, seed| {
            let _r = machine(c, seed, 0x2e5d18, 4);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = 0;
            for i in 0..11u32 {
                c.bus.write32(
                    DATA + 0x10 + 4 * i,
                    if seed % 3 == 0 && i == 2 {
                        DATA + 0x2000 + 0x100 * i
                    } else {
                        0
                    },
                );
            }
        });
    }


        #[test]
    fn dynamic_spray_fx_reset_matches() {
        check(
            ee::s09::f_002df3b0__generated,
            dynamic_spray_fx_reset,
            SEEDS_9,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this, DATA + 0x800);
                w(c, DATA + 0x800 + 0x870, (seed & 1) as u32);
                w(c, this + 0x124, (seed & 1) as u32);
                c.g[A0][0] = this as u64;
            }),
        );
    }


    // ------------------------------------------------------------ transitions

    #[test]
    fn c_transition_man_dtor_matches() {
        check(ssx3_ee::s09::f_002e42d0__generated, c_transition_man_dtor, 8, &|c, seed| {
            let _ = base_4(c, seed, 0x002e_42d0);
            let man = DATA + 0x100;
            let mgr = DATA + 0x5000;
            let vtable = DATA + 0x6000;
            clear_2(c, man, 0x100);
            clear_2(c, mgr, 0x1200);
            clear_2(c, vtable, 0x200);
            w(c, man + 0x7c, if seed % 3 == 0 { 0 } else { DATA + 0x7000 });
            w(c, GAME_GP.wrapping_add(0x2a90), mgr);
            w(c, mgr + 0x10d8, vtable);
            c.bus.write16(vtable + 0x1e0, 0);
            w(c, vtable + 0x1e4, crate::difftest::fakes::FAKE);
            w(c, crate::difftest::fakes::LOG_RESULTS, 1);
            c.g[A0][0] = sx(man);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    #[test]
    fn c_player_pulse_fx_reset_matches() {
        check(
            ee::s09::f_002e3930__generated,
            c_player_pulse_fx_reset,
            24,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x2e3930, 0);
                c.g[A0][0] = OBJ as u64;
                fill(c, &mut r, OBJ, 0x14);
                w(c, GAME_GP.wrapping_sub(0x3a34), r.next_u32());
            },
        );
    }


            // ---------------------------------------------------------------- effects

    #[test]
    fn c_transition_man_ctor_matches() {
        check(
            ee::s09::f_002e4228__generated,
            c_transition_man_ctor,
            6,
            &|c, seed| {
                let _ = base_5(c, seed, 0x2e4228, 4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed * 7 % 11) as u64;
                fill(c, &mut Rng(seed as u64 ^ 0x1234), OBJ, 0x90);
            },
        );
    }


    // ------------------------------------------------------------- VisualFX.

    #[test]
    fn ctransition_man_stop_matches() {
        check(ee::s09::f_002e4578__generated, ctransition_man_stop, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2e_4578, 3);
            let this = DATA;
            c.g[A0][0] = sx(this);
            c.bus.write32(this + 0x44, if seed % 2 == 0 { 0 } else { 1 });
            let obj1 = DATA + 0x1000;
            let obj2 = DATA + 0x1200;
            c.bus.write32(this + 0x4c, if seed % 3 == 0 { 0 } else { obj1 });
            c.bus.write32(this + 0x50, if seed % 5 == 0 { 0 } else { obj2 });
            for (i, o) in [obj1, obj2].into_iter().enumerate() {
                let vt = DATA + 0x2000 + 0x100 * i as u32;
                c.bus.write32(o, vt);
                c.bus.write16(vt + 8, 0);
                c.bus.write32(vt + 0xc, FAKE + 0x10 * i as u32);
            }
            c.bus.write32(this + 0x6c, if seed % 7 == 0 { u32::MAX } else { r.next_u32() });
            let world = DATA + 0x3000;
            let vt3 = DATA + 0x3400;
            c.bus.write32(GAME_GP + 0x2a90, world);
            c.bus.write32(world + 0x10d8, vt3);
            c.bus.write16(vt3 + 0x390, 0);
            c.bus.write32(vt3 + 0x394, FAKE + 0x40);
        });
    }


    #[test]
    fn c_transition_man_script_activate_matches() {
        check(
            ee::s09::f_002e4b98__generated,
            c_transition_man_script_activate,
            SEEDS_10,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e4b, 3);
                game_globals_2(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(
                    this + 0x28,
                    if seed % 2 == 0 { 7 } else { (seed % 8) as u32 },
                );
                c.bus.write32(this + 0x38, seed % 4);
                c.bus.write32(this + 0x34, r.f32(1.0));
                c.bus.write32(this + 0x30, r.f32(1.0));
                c.bus.write32(this + 0x2c, r.f32(1.0));
                c.bus.write32(GAME_GP - 0x84c, DATA + 0x2000);
            },
        );
    }


    // -- 0x002e4d88 SnowBox_ctor ----------------------------------------

    #[test]
    fn snow_box_ctor_matches() {
        check(ssx3_ee::s09::f_002e4d88__generated, snow_box_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2e4d88 ^ (seed as u64) << 6);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[SP][0] = crate::difftest::SP as u64;
            c.g[RA][0] = crate::difftest::RA as u64;
            let this = DATA + 0x1000;
            let a = DATA + 0x2000;
            let b = DATA + 0x2100;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(a);
            c.g[A2][0] = sx(b);
            for i in 0..4u32 {
                wf(c, a + 4 * i, r.f32(2.0));
                wf(c, b + 4 * i, r.f32(2.0));
            }
            for i in 0..0x40u32 {
                w(c, this + 4 * i, r.next_u32());
            }
            // the base matrix the constructor scales
            for i in 0..0x40u32 {
                wf(c, 0x004f_f1a0 + 4 * i, r.f32(1.0));
            }
        });
    }


    pub(crate) const SEEDS_11: u32 = 200;


    /// Every compared state must match: hard, caller-saved (`soft`) and LO/HI.
    pub(crate) fn strict(g: GuestFn, h: GuestFn, seeds: u32, setup_2: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(g, h, &|c| setup_2(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }


    // Fixture layout (all inside RAM, away from the stack).
    pub(crate) const SELF: u32 = DATA;

    pub(crate) const OBJ_2: u32 = DATA + 0x1000;

    pub(crate) const OWNER: u32 = DATA + 0x2000;

    pub(crate) const MODEL: u32 = DATA + 0x3000;

    pub(crate) const MATS: u32 = DATA + 0x4000;

    pub(crate) const VT: u32 = DATA + 0x5000;

    pub(crate) const GAME_2: u32 = DATA + 0x6000;

    pub(crate) const S3: u32 = DATA + 0x7000;

    pub(crate) const STRIPS: u32 = DATA + 0x8000;

    pub(crate) const ROWS: u32 = DATA + 0x9000;

    pub(crate) const WORLD_5: u32 = DATA + 0xe000;

    pub(crate) const BE_6: u32 = DATA + 0xf000;


    /// The game's float constants at `gp-0x3c34..gp-0x3bd8` (dt, 20.0, 0.8,
    /// 1.0, the camera and speed thresholds, ...): the test machine's RAM
    /// starts empty, so restore them from the executable's image.
    pub(crate) const GP_CONSTS: [u32; 25] = [
        0x3c888889, 0x40555555, 0x44d05556, 0x44d05556, 0x3f4ccccd, 0x440ae38e, 0x402ccccd,
        0x3eb33333, 0x448ae38e, 0x3f0a3d71, 0x3e19999a, 0x43d05556, 0x3d4ccccd, 0x3f4ccccd,
        0x3e4ccccc, 0x3f333333, 0x3e99999a, 0x3d4ccccd, 0x425e38e4, 0x3f4ccccd, 0x3f59999a,
        0x3f666666, 0x3d088889, 0x3f19999a, 0x3ecccccd,
    ];


    /// A wake-trail object whose rider, model, matrix row, element rings and
    /// cross-section array all live in the fixture's scratch area. Counts and
    /// cursors stay small so the real `BuildTrail`/`AdvanceRing` callees finish
    /// in a bounded number of steps; the rider's `+0x6c0` vtable slot points at
    /// an unmapped address so `AdvanceRing`'s virtual call goes through the
    /// test machine's fake-call hook.
    pub(crate) fn setup_2(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x151_0000, 8);

        // Game-wide singletons and the float constants this function reads.
        w(c, GAME_GP.wrapping_sub(0x848), WORLD_5);
        w(c, GAME_GP.wrapping_sub(0x1f04), BE_6);
        put_words(c, GAME_GP.wrapping_sub(0x3c34), &GP_CONSTS);

        // world -> game object -> this rider's race data (s3).
        w(c, WORLD_5 + 0x84, GAME_2);
        w(c, GAME_2 + 0x44, S3);

        // Random contents everywhere the function reads floats from...
        fill(c, &mut r, SELF, 0x40);
        fill(c, &mut r, OBJ_2, 0x400);
        fill(c, &mut r, MATS, 0x200);
        fill(c, &mut r, S3, 0x100);
        fill(c, &mut r, STRIPS, 0x100);
        for i in 0u32..5 {
            fill(c, &mut r, ROWS + 0x1000 * i, 0x100);
        }

        // ...then the pointers and counters that bound the loops.
        w(c, SELF + wake::OBJ, OBJ_2);
        w(c, SELF + wake::LIVE_2, seed % 3);
        w(c, SELF + wake::CURSOR, (seed / 3) % 3);
        w(c, SELF + wake::SPAN, 1 + (seed / 9) % 3);
        w(c, SELF + wake::MAX, 1 + (seed / 27) % 4);
        for i in 0u32..5 {
            w(c, SELF + wake::ROWS + 4 * i, ROWS + 0x1000 * i);
        }
        // Some seeds force the section to expire (life is tiny) and cover the
        // "no previous point"/"built" flags both ways.
        w(
            c,
            SELF + wake::LIFE,
            if seed % 8 == 0 { 0x3c800000 } else { r.edge_f32() },
        );
        w(c, SELF + wake::BUILT, (seed / 4) % 2);
        w(c, SELF + wake::HAS_PREV, (seed / 5) % 2);

        w(c, OBJ_2 + 0x438, (seed / 8) % 3);
        w(c, OBJ_2 + 0x77c, OWNER);
        w(c, OBJ_2 + 0x780, MODEL);
        w(c, OBJ_2 + 0x6c0, VT);
        w(c, OBJ_2 + 0x8a4, (seed / 10) % 3);
        w(c, OBJ_2 + 0x320, if seed % 2 == 0 { 0 } else { 0x10 + seed });
        w(c, OWNER + 0xde0, (seed / 6) % 2);
        w(c, MODEL + 0x30, MATS);
        w(c, MODEL + 0x34, MATS + 0x100);
        w(c, S3 + 0x84, (seed / 7) % 2);
        // AdvanceRing's virtual call: {short delta, void (*fn)()} at +0x38.
        c.bus.write16(VT + 0x38, 0);
        w(c, VT + 0x3c, 0x0000_4000);

        c.g[A0][0] = SELF as u64;
        c.pc = 0x002d_d0b8;
    }


    #[test]
    fn board_wake_fx_update_matches() {
        strict(
            ssx3_ee::s09::f_002dd0b8__generated,
            c_board_wake_fx_update,
            SEEDS_11,
            &setup_2,
        );
    }


    /// The speed-boost streamer's own object, its rider and model.
    pub(crate) const FX_RIDER: u32 = DATA + 0x9000;

    pub(crate) const MODEL_2: u32 = DATA + 0xa000;

    pub(crate) const BONES: u32 = DATA + 0xb000;

    pub(crate) const FX_VT: u32 = DATA + 0xb800;

    pub(crate) const ANIM: u32 = DATA + 0xbc00;


    #[test]
    fn speed_boost_streamer_fx_update_points_matches() {
        check_lohi(
            ee::s09::f_002e6c08__generated,
            c_speed_boost_streamer_fx_update_points,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x6c08, 1);
                elf_image(c);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), BELIB);
                // The timer/fade decide whether the update runs at_7 all.
                let (timer, fade): (f32, f32) = match seed % 4 {
                    0 => (0.5, 0.5),
                    1 => (-0.5, 0.5),
                    2 => (-0.5, -0.5),
                    _ => (0.0, 0.1),
                };
                c.bus.write32(M, FX_RIDER);
                c.bus.write32(M + 0x4, f(timer));
                c.bus.write32(M + 0x18, f(fade));
                c.bus.write32(M + 0x14, r.next_u32() % 40);
                c.bus.write32(M + 0x30, r.next_u32() % 4);
                for i in 0..0x30 {
                    c.bus.write32(M + 0x50 + 4 * i, r.edge_f32());
                }
                // The rider, its board (a vtable whose slot is the fake
                // method), its animation and its model with the bone array
                // the streamer indexes.
                c.bus.write32(FX_RIDER + 0x6c0, FX_VT);
                c.bus.write32(FX_RIDER + 0x780, MODEL_2);
                c.bus.write32(FX_RIDER + 0x784, ANIM);
                c.bus.write32(FX_RIDER + 0x8a4, r.next_u32() % 8);
                c.bus.write32(FX_RIDER + 0x320, (seed % 2) as u32);
                c.bus.write32(FX_RIDER + 0x330, r.f32(1.0));
                c.bus.write16(FX_VT + 0x28, 0);
                c.bus.write32(FX_VT + 0x2c, FAKE);
                c.bus.write32(MODEL_2 + 0x30, BONES);
                for i in 0..0x100 {
                    c.bus.write32(BONES + 4 * i, r.edge_f32());
                }
                for off in [0x1e0, 0x370, 0x460] {
                    put_vec(
                        c,
                        FX_RIDER + off,
                        [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
                    );
                }
                c.bus.write32(M + 0x20, r.f32(1.0));
                c.bus.write32(M + 0x24, r.f32(1.0));
                c.g[A0][0] = sx(M);
                c.pc = 0x002e_6c08;
            },
        );
    }
}
