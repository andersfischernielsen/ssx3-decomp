//! `src/visualfx/boardwakefx.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_fcc;
use crate::abi::set_fpr;
use crate::abi::set_reg;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ridermetrix::at_3 as at_7;
pub(crate) use crate::game::ai::ridermetrix::lw_3;
pub(crate) use crate::game::ai::ridermetrix::lwc1;
pub(crate) use crate::game::ai::ridermetrix::sw_2 as sw_3;
pub(crate) use crate::game::ai::ridermetrix::swc1;
pub(crate) use crate::game::camera::camera::gp_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_5;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at_4;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::gpr;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::fe::festateselectmp::epilogue;
pub(crate) use crate::game::fe::ovstates::at as at_6;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::intersect::riderspheretree::p_2;
pub(crate) use crate::game::misc::gp_lit;
use crate::game::misc::STATUS_EIE;
pub(crate) use crate::game::object::debouncenode::lw32;
pub(crate) use crate::game::object::debouncenode::BOARD_WAKE_WORLD;
pub(crate) use crate::game::object::splinemodifier::ldf as ldf_3;
pub(crate) use crate::game::object::splinemodifier::setf as setf_3;
pub(crate) use crate::game::object::splinemodifier::stf as stf_2;
pub(crate) use crate::game::render::particle::addi;
pub(crate) use crate::game::render::particle::at as at_2;
pub(crate) use crate::game::replay::replay2::destroy_if_flag;
use crate::game::visualfx::boardwakefx as bw;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::tick;
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
    (0x0028_32d8, script_obj_unregister, "ScriptObj_Unregister"),
    (0x002d_d0b8, c_board_wake_fx_update, "cBoardWakeFX_Update"),
    (0x002d_dab8, board_wake_fx_render, "cBoardWakeFX_Render"),
    (
        0x002d_e398,
        dynamic_spray_fx_add_random_velocity,
        "cDynamicSprayFX_AddRandomVelocity",
    ),
    (0x002d_f3b0, dynamic_spray_fx_reset, "cDynamicSprayFX_Reset"),
    (
        0x002d_f448,
        c_dynamic_spray_fx_setup_emitters,
        "cDynamicSprayFX_SetupEmitters",
    ),
    (
        0x002d_fe88,
        dynamic_spray_update_emitter3,
        "cDynamicSprayFX_UpdateEmitter3",
    ),
    (
        0x002e_0ee8,
        dynamic_spray_fx_update_emitter0,
        "cDynamicSprayFX_UpdateEmitter0",
    ),
    (
        0x002e_1120,
        dynamic_spray_update_emitter4,
        "cDynamicSprayFX_UpdateEmitter4",
    ),
    (
        0x002e_1598,
        dynamic_spray_fx_update_emitter5,
        "cDynamicSprayFX_UpdateEmitter5",
    ),
    (
        0x002e_24d0,
        dynamic_spray_fx_render,
        "cDynamicSprayFX_Render",
    ),
    (
        0x002e_27e8,
        lens_fx_get_flare_size_for_res,
        "LensFX_GetFlareSizeForRes",
    ),
    (0x002e_3060, lens_fx_base_dtor, "LensFXBase_dtor"),
    (
        0x002e_3578,
        lens_fx_base_push_render_state,
        "LensFXBase_PushRenderState",
    ),
    (
        0x002e_3668,
        lens_fx_base_pop_render_state,
        "LensFXBase_PopRenderState",
    ),
    (0x002e_3930, c_player_pulse_fx_reset, "cPlayerPulseFX_Reset"),
    (0x002e_39d8, player_pulsefx_update, "cPlayerPulseFX_Update"),
    (
        0x002e_3af8,
        c_player_pulse_fx_render,
        "cPlayerPulseFX_Render",
    ),
    (0x002e_4228, c_transition_man_ctor, "cTransitionMan_ctor"),
    (0x002e_42d0, c_transition_man_dtor, "cTransitionMan_dtor"),
    (
        0x002e_44f0,
        c_transition_man_start_out,
        "cTransitionMan_StartOut",
    ),
    (
        0x002e_4540,
        c_transition_man_start_captured,
        "cTransitionMan_StartCaptured",
    ),
    (0x002e_4578, ctransition_man_stop, "cTransitionMan_Stop"),
    (
        0x002e_4638,
        c_transition_man_set_paused,
        "cTransitionMan_SetPaused",
    ),
    (
        0x002e_4678,
        c_transition_man_get_time_remaining,
        "cTransitionMan_GetTimeRemaining",
    ),
    (
        0x002e_46c8,
        c_transition_man_update,
        "cTransitionMan_Update",
    ),
    (0x002e_47e8, transition_man_render, "cTransitionMan_Render"),
    (
        0x002e_4b98,
        c_transition_man_script_activate,
        "cTransitionMan_ScriptActivate",
    ),
    (
        0x002e_4c60,
        c_transition_man_script_deactivate,
        "cTransitionMan_ScriptDeactivate",
    ),
    (
        0x002e_4c90,
        c_transition_man_script_is_running,
        "cTransitionMan_ScriptIsRunning",
    ),
    (
        0x002e_4cb0,
        c_transition_man_reset_effect_pool,
        "cTransitionMan_ResetEffectPool",
    ),
    (
        0x002e_4ce8,
        transition_effect_alloc_data,
        "TransitionEffect_AllocData",
    ),
    (
        0x002e_4d30,
        transition_effect_free_data,
        "TransitionEffect_FreeData",
    ),
    (0x002e_4d70, transition_effect_ctor, "TransitionEffect_ctor"),
    (0x002e_4d88, snow_box_ctor, "SnowBox_ctor"),
    (0x002e_4f10, snow_box_reset, "SnowBox_Reset"),
    (0x002e_4f28, snow_box_dtor, "SnowBox_dtor"),
    (0x002e_5430, snow_box_wrap_offset, "SnowBox_WrapOffset"),
    (0x002e_55d8, snow_box_render, "SnowBox_Render"),
    (0x002e_5d18, snow_fall_dtor, "SnowFall_dtor"),
    (0x002e_62c8, snow_fall_reset_boxes, "SnowFall_ResetBoxes"),
    (
        0x002e_6640,
        c_speed_boost_streamer_fx_reset,
        "cSpeedBoostStreamerFX_Reset",
    ),
    (
        0x002e_66b8,
        c_speed_boost_streamer_fx_update,
        "cSpeedBoostStreamerFX_Update",
    ),
    (
        0x002e_6c08,
        c_speed_boost_streamer_fx_update_points,
        "cSpeedBoostStreamerFX_UpdatePoints",
    ),
    (
        0x002e_7a10,
        c_speed_boost_streamer_fx_render,
        "cSpeedBoostStreamerFX_Render",
    ),
    (
        0x002d_f920,
        c_dynamic_spray_fx_update,
        "cDynamicSprayFX_Update",
    ),
    (0x002e_4f50, snow_box_update, "SnowBox_Update"),
    (
        0x002e_1a80,
        dynamic_spray_fx_update_emitter7,
        "cDynamicSprayFX_UpdateEmitter7",
    ),
    (
        0x002d_dd30,
        board_wake_fx_build_trail,
        "cBoardWakeFX_BuildTrail",
    ),
    (
        0x002e_1f70,
        dynamic_spray_fx_update_emitter8,
        "cDynamicSprayFX_UpdateEmitter8",
    ),
    (0x002e_2868, lens_fx_draw_flare_quad, "LensFX_DrawFlareQuad"),
    (0x002e_2b00, lens_fx_project_source, "LensFX_ProjectSource"),
    (
        0x002e_3130,
        lens_fx_update_source_visibility,
        "LensFX_UpdateSourceVisibility",
    ),
    (0x002e_5da0, snowfall_update, "SnowFall_Update"),
    (0x002e_6008, snow_fall_render, "SnowFall_Render"),
    (
        0x002d_e058,
        board_wake_fx_advance_ring,
        "cBoardWakeFX_AdvanceRing",
    ),
    (
        0x002e_2260,
        dynamic_spray_fx_update_emitter9,
        "cDynamicSprayFX_UpdateEmitter9",
    ),
    (0x002e_3338, lens_fx_prepare_view, "LensFX_PrepareView"),
    (0x002e_3478, lens_fx_draw_flares, "LensFX_DrawFlares"),
    (0x002f_4690, lens_fx_draw_flare_set, "LensFX_DrawFlareSet"),
    (0x002e_23e0, dynamic_spray_queue_burst, "cDynamicSprayFX_QueueBurst"),
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

    /// The most points the streamer keeps (20, or 18 for some riders).
    pub const MAX_COUNT: u32 = 0x3a4;

    /// The rider was in the air at the last update.
    pub const IN_AIR: u32 = 0x3a8;

    pub const POINTS: u32 = 0x3ac;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::COMM_SYSTEM_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_ADVANCE_DESC_TIME;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_RENDER;
    pub(crate) use crate::game::ai::rider::callee::EMITTER_UPDATE_DESCRIPTOR;
    pub(crate) use crate::game::ai::rider::callee::INPUT_PAD_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::MATH_SIN_COS_2 as MATH_SIN_COS;
    pub(crate) use crate::game::ai::rider::callee::MODIFIER_BLOCK_GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_ADVANCE_WRITE;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_GET_PAD_DATA;
    pub(crate) use crate::game::ai::rider::callee::PAD_QUEUE_GET_PAD_STATUS;
    pub(crate) use crate::game::ai::rider::callee::USB_KEYBOARD_UPDATE;
    pub(crate) use crate::game::ai::rider::callee::VU0_RESTORE_CONTEXT;
    pub(crate) use crate::game::ai::rider::callee::VU0_SAVE_CONTEXT;
    pub(crate) use crate::game::camera::camera::callee::BEBAGT_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::camera::camera::callee::BE_LIBRARY_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BE_OPTION_GET_MAX_HANDICAP;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_FREE_TRIGGERS;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_RESET_HEADER;
    pub(crate) use crate::game::camera::camera::callee::CAM_CONTROLLER_RESET;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_PROFILE_LIST;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_SLOTS;
    pub(crate) use crate::game::camera::camera::callee::MANUAL_CAM_ALGO_BASE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::MEASURE_ROW;
    pub(crate) use crate::game::camera::camera::callee::MEMCARD_SET_STATE;
    pub(crate) use crate::game::camera::camera::callee::SPHERE_TREE_RESET_TRANSFORM;
    pub(crate) use crate::game::camera::camera::callee::TRANSITION_MAN_START;
    pub(crate) use crate::game::camera::camera::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::camera::camera::callee::WSCRIPT_DELETE_ALL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2 as CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2 as SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_CTOR_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_RECV;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_SSX_WORLD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET as MC_MANAGER_GET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_13 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3 as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2 as SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2 as SPCH_MAKE_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXTURE_SET_FIND as UI_TEXTURE_SET_FIND_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::object::debouncenode::callee::SCRIPT_VIEW_OBJ_BIND_CAM_SLOT;
    pub(crate) use crate::game::object::debouncenode::callee::SPLINE_MODIFIER_IS_NODE_VISIBLE;
    pub(crate) use crate::game::object::debouncenode::callee::TRANSITION_MAN_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;
    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_INIT;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;

    pub const ADD_RANDOM_VELOCITY: Callee = Callee {
        addr: 0x002d_e398,
        f: ee::s09::cDynamicSprayFX_AddRandomVelocity,
    };

    pub const ADD_SPLASH: Callee = Callee {
        addr: 0x002f_4118,
        f: ee::s09::cSplashManager_AddSplashAllViews,
    };

    /// Same-shard direct calls (`jal`).
    pub const ANIM_SEQ_LIST_GET_AT: Callee = Callee {
        addr: 0x0031_4760,
        f: ee::s10::AnimSeqList_GetAt,
    };

    pub const RIDER_COMPUTE_LOD: Callee = Callee {
        addr: 0x0012_1f30,
        f: ee::s00::Rider_ComputeLOD,
    };

    pub const TRANSITION_EFFECT_UPDATE_SCREEN_RECT: Callee = Callee {
        addr: 0x002e_b8f0,
        f: ee::s09::TransitionEffect_UpdateScreenRect,
    };

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
    pub(crate) use crate::game::be::bedbreward::table_callee::EA_MEMSET;
    pub(crate) use crate::game::be::bedbreward::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::be::bedbreward::table_callee::WORLD_CREATE_DEAD_NODE;
}

pub(crate) mod spray_callee {
    use super::Callee;
    use ssx3_ee as ee;

    pub const ADD_RANDOM_VELOCITY: Callee = Callee {
        addr: 0x002d_e398,
        f: ee::s09::cDynamicSprayFX_AddRandomVelocity,
    };

    pub const SPAWN_AT: Callee = Callee {
        addr: 0x0037_17c0,
        f: ee::s12::DynamicEmitter_SpawnAt,
    };

    pub const WORLD_PAINTER_GET_WEATHER: Callee = Callee {
        addr: 0x002e_e6f0,
        f: ee::s09::WorldPainter_GetWeather30,
    };

    pub const SETUP_EMITTERS: Callee = Callee {
        addr: 0x002d_f448,
        f: ee::s09::cDynamicSprayFX_SetupEmitters,
    };

    pub const FIND_BONES: Callee = Callee {
        addr: 0x002d_f4d0,
        f: ee::s09::cDynamicSprayFX_FindBones,
    };

    pub const QUEUE_BURST: Callee = Callee {
        addr: 0x002e_23e0,
        f: ee::s09::cDynamicSprayFX_QueueBurst,
    };

    pub const UPDATE_EMITTER_ENABLES: Callee = Callee {
        addr: 0x002e_2550,
        f: ee::s09::cDynamicSprayFX_UpdateEmitterEnables,
    };

    /// The per-emitter updates in call order: (enable flag offset in the
    /// emitter array, callee, return address).
    pub const EMITTERS: [(u32, Callee, u32); 8] = [
        (
            0x384,
            Callee {
                addr: 0x002e_02b8,
                f: ee::s09::cDynamicSprayFX_UpdateEmitter1,
            },
            0x002d_fdc4,
        ),
        (
            0x7a4,
            Callee {
                addr: 0x002d_fe88,
                f: ee::s09::cDynamicSprayFX_UpdateEmitter3,
            },
            0x002d_fddc,
        ),
        (
            0x174,
            Callee {
                addr: 0x002e_0ee8,
                f: ee::s09::cDynamicSprayFX_UpdateEmitter0,
            },
            0x002d_fdf4,
        ),
        (
            0x9b4,
            Callee {
                addr: 0x002e_1120,
                f: ee::s09::cDynamicSprayFX_UpdateEmitter4,
            },
            0x002d_fe0c,
        ),
        (
            0xbc4,
            Callee {
                addr: 0x002e_1598,
                f: ee::s09::cDynamicSprayFX_UpdateEmitter5,
            },
            0x002d_fe24,
        ),
        (
            0xfe4,
            Callee {
                addr: 0x002e_1a80,
                f: ee::s09::cDynamicSprayFX_UpdateEmitter7,
            },
            0x002d_fe3c,
        ),
        (
            0x1404,
            Callee {
                addr: 0x002e_2260,
                f: ee::s09::cDynamicSprayFX_UpdateEmitter9,
            },
            0x002d_fe54,
        ),
        (
            0x11f4,
            Callee {
                addr: 0x002e_1f70,
                f: ee::s09::cDynamicSprayFX_UpdateEmitter8,
            },
            0x002d_fe6c,
        ),
    ];

    /// `AnimController_GetLayerAnimId(anim, layer)`.
    pub const ANIM_LAYER_ID: u32 = 0x0031_2aa0;

    /// `Rider_GetMainState(rider)` / `Rider_GetSubState(rider)`.
    pub const RIDER_MAIN_STATE: u32 = 0x0011_fe98;

    pub const RIDER_SUB_STATE: u32 = 0x0011_fee8;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::STRLEN;
}

/// Cross-shard calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_CALL_A;
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_CALL_B;
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_CALL_C;
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_CALL_D;
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_SYSTEM;
    pub(crate) use crate::game::ai::ridermetrix::table::AUDIO_TRICK_TICK;
    pub(crate) use crate::game::ai::ridermetrix::table::BE_CALL;
    pub(crate) use crate::game::ai::ridermetrix::table::BE_GET_BE;
    pub(crate) use crate::game::ai::ridermetrix::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::ai::ridermetrix::table::BOARD_DONE;
    pub(crate) use crate::game::ai::ridermetrix::table::BOARD_WAKE_A;
    pub(crate) use crate::game::ai::ridermetrix::table::BOARD_WAKE_B;
    pub(crate) use crate::game::ai::ridermetrix::table::BOARD_WAKE_CLEAR;
    pub(crate) use crate::game::ai::ridermetrix::table::PLAY_ANIM;
    pub(crate) use crate::game::ai::ridermetrix::table::SIN;
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

/// `void cSpeedBoostStreamerFX_UpdatePoints(cSpeedBoostStreamerFX*)`
/// (0x2e6c08).
pub(crate) const UPDATE_POINTS: Callee = Callee {
    addr: 0x002e_6c08,
    f: ee::s09::cSpeedBoostStreamerFX_UpdatePoints,
};

/// `void cSpeedBoostStreamerFX_Update(cSpeedBoostStreamerFX* self)`
/// (0x2e66b8): the boost streamers behind a rider. The rider boosts when it
/// is not in the air and one of its boost timers (`+0x2fc`, `+0x2e8`) runs;
/// leaving the air clears the streamer. Picks the streamer texture (and its
/// scroll rate) from the boost phase and globals, sets length, alpha and
/// width while boosting and fades them (`*= 0.95`) otherwise. With streamers
/// enabled (`gp+0x1610`) it records one point pair per tick into the 20-slot
/// ring while boosting (the older pairs spreading apart by the rider's
/// direction * 1.4) and shrinks the live count by one per tick otherwise,
/// writes the newest pair from the board's bones, scrolls the texture by the
/// speed, then runs `UpdatePoints`.
///
/// Tick rate: the ring records, spreads and shrinks once per 60 Hz frame of
/// the streamer's own clock (`tick::frame60_due`; every tick at 60), so the
/// trail covers the same time and spreads at the same speed; the fade
/// constant becomes 0.95^(60/rate). The texture scroll's step is a `.lit4`
/// 1/60 (follows the rate). Exact at 60.
pub fn c_speed_boost_streamer_fx_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save_ra(c, 0x70);
        c.g[S0][0] = c.g[A0][0];
        let fx = r32(c, S0);
        c.g[A0][0] = c.lw(fx);
        call_ptr(c, RIDER_GET_MAIN_STATE, 0x002e_66d0)?;
        // In the air (main state 1)? Otherwise boosting while a boost timer
        // runs.
        c.g[V0][0] ^= 1;
        c.g[V1][0] = (c.g[V0][0] < 1) as u64;
        let in_air = c.g[V1][0] != 0;
        c.g[A2][0] = 0;
        'boost: {
            if !in_air {
                c.cycles += 7;
                c.g[V0][0] = c.lw(fx);
                c.mtc1(0, 1);
                c.lwc1(0, p(r32(c, V0), 0x2fc));
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    c.cycles += 5;
                    c.lwc1(0, p(r32(c, V0), 0x2e8));
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.g[V0][0] = c.lw(p(fx, boost::IN_AIR));
                        break 'boost;
                    }
                    c.cycles += 1;
                }
                c.g[A2][0] = 1;
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(p(fx, boost::IN_AIR));
        }
        // Leaving the air clears the streamer and the flames.
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                c.cycles += 4;
                c.sw(p(fx, boost::COUNT), 0);
                c.sw(p(fx, boost::FLAME_CLAMP), 0);
                c.sw(p(fx, boost::FLAMES), 0);
            }
        }
        c.sw(p(fx, boost::IN_AIR), c.g[V1][0]);
        // The texture (v0) and its scroll rate (f4).
        'tex: {
            c.cycles += 6;
            c.g[V0][0] = 0xb;
            c.g[A0][0] = c.lw(fx);
            c.lwc1(4, gp(c, -0x3a00));
            c.g[V1][0] = c.lw(p(r32(c, A0), 0x2f4));
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.lwc1(4, gp(c, -0x39fc));
                c.g[V0][0] = 0x3d;
                break 'tex;
            }
            c.lwc1(0, p(r32(c, A0), 0x2e8));
            'phase: {
                c.cycles += 5;
                c.lwc1(1, gp(c, -0x39f8));
                c.c_lt_s(1, 0);
                let long = c.fcc();
                c.g[V0][0] = 0x3c;
                if !long {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(gp(c, 0x1640));
                    let off = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x3c;
                    if off {
                        break 'phase;
                    }
                }
                c.cycles += 2;
                c.mov_s(4, 1);
                break 'tex;
            }
            'under5: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 10) as u64;
                let at_least10 = c.g[V0][0] == 0;
                c.g[V0][0] = c.lw(gp(c, 0x1644));
                if !at_least10 {
                    c.cycles += 2;
                    let off = c.g[V0][0] == 0;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 5) as u64;
                    if off {
                        break 'under5;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = 0x3b;
                c.mov_s(4, 1);
                break 'tex;
            }
            c.cycles += 2;
            let at_least5 = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(gp(c, 0x1648));
            if !at_least5 {
                c.cycles += 2;
                let off = c.g[V0][0] == 0;
                c.g[V0][0] = 0x39;
                if off {
                    break 'tex;
                }
            }
            c.cycles += 2;
            c.lwc1(4, gp(c, -0x39f4));
            c.g[V0][0] = 0x3a;
        }
        // f5 = |rider velocity|; length, alpha and width.
        'size: {
            c.cycles += 14;
            c.sw(p(fx, boost::TEXTURE), c.g[V0][0]);
            c.g[A0][0] = c.lw(fx);
            c.lqc2(3, p(r32(c, A0), 0x1e0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 5);
            if c.g[A2][0] != 0 {
                // Boosting: f3 = clamp(max(timers), 0, 1);
                // length = (f3 * 400 + 400) * (speed / k).
                c.cycles += 9;
                c.lwc1(1, p(r32(c, A0), 0x2fc));
                c.lwc1(0, p(r32(c, A0), 0x2e8));
                c.mtc1(0, 2);
                c.max_s(0, 0, 1);
                c.c_le_s(2, 0);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 1);
                let negative = !c.fcc();
                c.mov_s(3, 2);
                if !negative {
                    c.cycles += 1;
                    c.min_s(3, 0, 1);
                }
                'width: {
                    c.cycles += 12;
                    c.g[AT][0] = 0x43c8_0000;
                    c.mtc1(1, 2);
                    c.lwc1(1, gp(c, -0x39f0));
                    c.g[V1][0] = 0xb;
                    c.mul_s(0, 3, 2);
                    c.div_s(1, 5, 1);
                    c.add_s(0, 0, 2);
                    c.mul_s(0, 0, 1);
                    c.swc1(0, p(fx, boost::FADE));
                    c.g[V0][0] = c.lw(p(r32(c, A0), 0x2f4));
                    let phase_b = c.g[V0][0] == c.g[V1][0];
                    c.g[A1][0] = sx(p(fx, boost::WIDTH));
                    if !phase_b {
                        // width = f3 * 9 + 6
                        c.cycles += 7;
                        c.g[AT][0] = 0x4110_0000;
                        c.mtc1(1, 0);
                        c.g[AT][0] = 0x40c0_0000;
                        c.mtc1(1, 1);
                        c.mul_s(0, 3, 0);
                        c.add_s(2, 0, 1);
                        break 'width;
                    }
                    c.cycles += 2;
                    c.g[AT][0] = 0x4248_0000;
                    c.mtc1(1, 2); // width 50
                }
                c.cycles += 5;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.swc1(2, r32(c, A1));
                c.swc1(0, p(fx, boost::ALPHA));
                break 'size;
            }
            c.lwc1(1, p(fx, boost::FADE));
            // Not boosting: length, alpha and width fade by 0.95 per tick
            // (0.95^(60/rate) above 60).
            c.cycles += 9;
            c.lwc1(0, gp(c, -0x39ec));
            per_tick_factor(c, 0);
            c.lwc1(2, p(fx, boost::ALPHA));
            c.lwc1(3, p(fx, boost::WIDTH));
            c.mul_s(1, 1, 0);
            c.mul_s(2, 2, 0);
            c.mul_s(3, 3, 0);
            c.swc1(1, p(fx, boost::FADE));
            c.swc1(2, p(fx, boost::ALPHA));
            c.swc1(3, p(fx, boost::WIDTH));
        }
        'points: {
            c.cycles += 3;
            c.g[V0][0] = c.lw(gp(c, 0x1610));
            let disabled = c.g[V0][0] == 0;
            c.g[A3][0] = c.g[SP][0];
            if disabled {
                break 'points;
            }
            // v0 = the live point count after this tick's record or shrink.
            'count: {
                c.cycles += 4;
                c.g[V1][0] = c.lw(fx);
                c.lq(2, p(r32(c, V1), 0x1a0)); // the rider's direction
                let boosting = c.g[A2][0] != 0;
                c.sq(2, sp(c));
                // One record (or shrink step) per 60 Hz frame of this
                // streamer (every tick at 60).
                if !tick::frame60_due(c, fx, 0) {
                    c.g[V0][0] = c.lw(p(fx, boost::COUNT));
                    break 'count;
                }
                if boosting {
                    // Advance the ring head (mod 20), grow the count up to
                    // the maximum, start the new pair as a copy of the last.
                    c.cycles += 7;
                    c.g[A1][0] = c.lw(p(fx, boost::FIRST));
                    c.g[V1][0] = 0x14;
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(1));
                    c.div(0, 2, 3);
                    c.sw(p(fx, boost::FIRST), c.g[V0][0]);
                    c.cycles += 6;
                    c.g[V1][0] = c.lw(p(fx, boost::COUNT));
                    c.g[V0][0] = c.lw(p(fx, boost::MAX_COUNT));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[A0][0] = c.hi[0];
                    let full = c.g[V0][0] == 0;
                    c.sw(p(fx, boost::FIRST), c.g[A0][0]);
                    if !full {
                        c.cycles += 2;
                        c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
                        c.sw(p(fx, boost::COUNT), c.g[V0][0]);
                    }
                    c.cycles += 20;
                    c.g[V0][0] = c.lw(p(fx, boost::POINTS));
                    c.g[A1][0] = sx(r32(c, A1) << 5);
                    c.g[A0][0] = c.lw(p(fx, boost::FIRST));
                    c.g[A2][0] = 0;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                    c.lwc1(0, gp(c, -0x39e8)); // 1.4
                    c.lq(3, r32(c, A1));
                    c.g[A0][0] = sx(r32(c, A0) << 5);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                    c.sq(3, r32(c, A0));
                    c.lq(2, p(r32(c, A1), 0x10));
                    c.sq(2, p(r32(c, A0), 0x10));
                    // spread = direction * 1.4
                    c.mfc1(3, 0);
                    c.lqc2(4, sp(c));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp(c), 0x20));
                    c.lq(2, p(sp(c), 0x20));
                    c.sq(2, p(sp(c), 0x10));
                    c.g[V1][0] = c.lw(p(fx, boost::POINTS));
                    // Every pair: a -= spread, b += spread.
                    loop {
                        c.cycles += 16;
                        c.g[A0][0] = sx(r32(c, A2) << 5);
                        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                        c.g[A1][0] = ((c.g[A2][0] as i64) < 20) as u64;
                        c.lqc2(3, r32(c, V1));
                        c.lqc2(4, p(sp(c), 0x10));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, r32(c, V1));
                        c.g[V0][0] = c.lw(p(fx, boost::POINTS));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                        c.lqc2(3, p(r32(c, A0), 0x10));
                        c.lqc2(4, p(sp(c), 0x10));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, p(r32(c, A0), 0x10));
                        if c.g[A1][0] == 0 {
                            break;
                        }
                        c.g[V1][0] = c.lw(p(fx, boost::POINTS));
                        if event_due(c) {
                            deliver_event_here(c, 0x002e_6950);
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(p(fx, boost::COUNT));
                    break 'count;
                }
                // Not boosting: the count steps towards 0 by one.
                c.cycles += 5;
                c.g[A0][0] = c.lw(p(fx, boost::COUNT));
                c.g[V0][0] = 1;
                c.g[V0][0] = (1i64 < c.g[A0][0] as i64) as u64;
                let above = c.g[V0][0] != 0;
                c.g[V1][0] = sx(r32(c, A0).wrapping_sub(1));
                if !above {
                    c.cycles += 4;
                    c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(1));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = 0;
                    }
                }
                c.cycles += 2;
                c.sw(p(fx, boost::COUNT), c.g[V1][0]);
                c.g[V0][0] = c.lw(p(fx, boost::COUNT));
            }
            c.cycles += 2;
            if (c.g[V0][0] as i64) <= 0 {
                break 'points;
            }
            // The newest pair from the board bones (+0x89c, +0x8b0, +0x8b8):
            // the side facing the direction gets the -0x39e4 or -0x39e0
            // offset.
            'pair: {
                'other: {
                    c.cycles += 44;
                    c.g[A2][0] = c.lw(fx);
                    c.g[V0][0] = c.lw(p(r32(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(r32(c, A2), 0x89c));
                    c.g[A0][0] = c.lw(p(r32(c, V0), 0x30));
                    c.g[V1][0] = sx(r32(c, V1) << 6);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                    c.lq(2, p(r32(c, A0), 0x30));
                    c.sq(2, p(sp(c), 0x10));
                    c.g[A1][0] = c.lw(p(r32(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(r32(c, A2), 0x8b0));
                    c.g[A0][0] = c.lw(p(r32(c, A1), 0x30));
                    c.g[V1][0] = sx(r32(c, V1) << 6);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                    c.lq(2, p(r32(c, A0), 0x30));
                    c.sq(2, p(sp(c), 0x20));
                    c.g[A1][0] = c.lw(p(r32(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(r32(c, A2), 0x8b8));
                    c.g[A0][0] = c.lw(p(r32(c, A1), 0x30));
                    c.g[V1][0] = sx(r32(c, V1) << 6);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                    c.lq(2, p(r32(c, A0), 0x30));
                    c.sq(2, p(sp(c), 0x30));
                    c.lqc2(3, p(sp(c), 0x30));
                    c.lqc2(5, r32(c, A3));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.lqc2(3, p(sp(c), 0x20));
                    c.lqc2(5, r32(c, A3));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.c_lt_s(0, 1);
                    let behind = !c.fcc();
                    c.g[V1][0] = c.lw(p(fx, boost::POINTS));
                    if behind {
                        break 'other;
                    }
                    c.cycles += 35;
                    c.g[A0][0] = c.lw(p(fx, boost::FIRST));
                    c.lwc1(0, gp(c, -0x39e4));
                    c.mfc1(2, 0);
                    c.lqc2(4, p(sp(c), 0x10));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp(c), 0x60));
                    c.g[A0][0] = sx(r32(c, A0) << 5);
                    c.lq(2, p(sp(c), 0x60));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                    c.sq(2, p(sp(c), 0x50));
                    c.lqc2(3, p(sp(c), 0x20));
                    c.lqc2(4, p(sp(c), 0x50));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, p(sp(c), 0x60));
                    c.lq(2, p(sp(c), 0x60));
                    c.sq(2, p(sp(c), 0x40));
                    c.sq(2, r32(c, A0));
                    c.g[V1][0] = c.lw(p(fx, boost::FIRST));
                    c.g[A0][0] = c.lw(p(fx, boost::POINTS));
                    c.mfc1(2, 0);
                    c.lqc2(4, p(sp(c), 0x10));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp(c), 0x60));
                    c.g[V1][0] = sx(r32(c, V1) << 5);
                    c.lq(2, p(sp(c), 0x60));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                    c.sq(2, p(sp(c), 0x50));
                    c.lqc2(3, p(sp(c), 0x30));
                    c.lqc2(4, p(sp(c), 0x50));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, p(sp(c), 0x60));
                    c.lq(2, p(sp(c), 0x60));
                    break 'pair;
                }
                c.cycles += 34;
                c.g[A0][0] = c.lw(p(fx, boost::FIRST));
                c.lwc1(0, gp(c, -0x39e0));
                c.mfc1(2, 0);
                c.lqc2(4, p(sp(c), 0x10));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp(c), 0x60));
                c.g[A0][0] = sx(r32(c, A0) << 5);
                c.lq(2, p(sp(c), 0x60));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                c.sq(2, p(sp(c), 0x50));
                c.lqc2(3, p(sp(c), 0x30));
                c.lqc2(4, p(sp(c), 0x50));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp(c), 0x60));
                c.lq(2, p(sp(c), 0x60));
                c.sq(2, p(sp(c), 0x40));
                c.sq(2, r32(c, A0));
                c.g[V1][0] = c.lw(p(fx, boost::FIRST));
                c.g[A0][0] = c.lw(p(fx, boost::POINTS));
                c.mfc1(2, 0);
                c.lqc2(4, p(sp(c), 0x10));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp(c), 0x60));
                c.g[V1][0] = sx(r32(c, V1) << 5);
                c.lq(2, p(sp(c), 0x60));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.sq(2, p(sp(c), 0x50));
                c.lqc2(3, p(sp(c), 0x20));
                c.lqc2(4, p(sp(c), 0x50));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp(c), 0x60));
                c.lq(2, p(sp(c), 0x60));
            }
            // Scroll the texture by speed * rate * 1/60 (a `.lit4` step),
            // wrapped to [0, 1].
            c.cycles += 12;
            c.sq(2, p(sp(c), 0x40));
            c.sq(2, p(r32(c, V1), 0x10));
            c.lwc1(0, gp(c, -0x39dc));
            c.lwc1(2, p(fx, boost::V_START));
            c.mul_s(0, 5, 0);
            c.mtc1(0, 1);
            c.mul_s(0, 4, 0);
            c.sub_s(2, 2, 0);
            c.c_lt_s(2, 1);
            let below = c.fcc();
            c.swc1(2, p(fx, boost::V_START));
            if below {
                c.cycles += 4;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.add_s(0, 2, 0);
                c.swc1(0, p(fx, boost::V_START));
            }
            c.cycles += 6;
            c.lwc1(0, p(fx, boost::V_START));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.cycles += 2;
                c.sub_s(0, 0, 1);
                c.swc1(0, p(fx, boost::V_START));
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        UPDATE_POINTS.call(c, 0x002e_6bf8)?;
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
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
pub(crate) fn gfx_method(
    c: &mut Cpu,
    gfx_reg: usize,
    slot: u32,
    vt_reg: usize,
    fn_reg: usize,
    ret: u32,
) -> Flow {
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

pub(crate) const BOOST: Streamer = Streamer {
    this: S2,
    points: boost::POINTS,
    count: boost::COUNT,
    tmp: 0xfc0,
    half: 0x7e0,
};

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
                                // A per-tick rise of the level: keep the rise
                                // per unit of wall-clock time.
                                per_tick_step(c, 0);
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
                                // Per-tick level rise -> per wall-clock time.
                                per_tick_step(c, 0);
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
                            // Per-tick level rise -> per wall-clock time.
                            per_tick_step(c, 0);
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
                    if tick::rate(c) != 60 {
                        // The same first-order lag per unit of wall-clock time:
                        // keep = 0.8^(60/rate), weight = 1 - keep. At 60 the
                        // two loaded constants are used as they are.
                        let keep = 0.8f32.powf(60.0 / tick::rate(c) as f32);
                        c.f[1] = keep.to_bits();
                        c.f[0] = (1.0 - keep).to_bits();
                    }
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
                // The factor applies once per tick at 60 Hz: at another rate
                // keep the same decay per unit of wall-clock time.
                per_tick_factor(c, 1);
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
pub(crate) fn copy_render_state(
    c: &mut Cpu,
    r0: usize,
    r1: usize,
    word: usize,
    src: u32,
    dst: u32,
) {
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

/// A per-tick step or rise as it becomes at the current rate: `x * 60/rate`
/// (the same amount per unit of wall-clock time). At 60 the caller keeps the
/// original constant, so the 60 Hz path stays bit-exact.
#[inline]
fn per_tick_step(c: &mut Cpu, f: usize) {
    if tick::rate(c) != 60 {
        let k = FpuF32((60.0 / tick::rate(c) as f32).to_bits());
        c.f[f] = (fr_2(c, f) * k).0;
    }
}

/// A per-tick factor as it becomes at the current rate: `k^(60/rate)` (a
/// first-order decay applied once per tick at 60 Hz). At 60 the caller keeps
/// the original constant.
#[inline]
fn per_tick_factor(c: &mut Cpu, f: usize) {
    if tick::rate(c) != 60 {
        c.f[f] = f32::from_bits(c.f[f])
            .powf(60.0 / tick::rate(c) as f32)
            .to_bits();
    }
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
        // Blend the animation's current time towards the target. The step is
        // 1/600 per tick at 60 Hz, so at another rate it is stretched to the
        // same wall-clock slew (the value stays in f0, where the original
        // keeps it; the 60 Hz path loads and subtracts it unchanged).
        c.cycles += 4;
        c.lwc1(1, p(r32_2(c, S0), 0x1c));
        c.c_le_s(1, 20);
        let above = !c.fcc();
        if above {
            c.cycles += 3;
            c.lwc1(0, gp(c, -0x3ae4));
            per_tick_step(c, 0);
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
                    if built {
                        pc = 0x002d_d9e4
                    } else {
                        pc = 0x002d_d770
                    };
                }
                0x002d_d770 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xb0));
                    let no_prev = c.g[V0][0] == 0u64;
                    if no_prev {
                        pc = 0x002d_d798
                    } else {
                        pc = 0x002d_d77c
                    };
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
                    // Per-tick blends `k * old + (1 - k) * new`: above 60 Hz
                    // k becomes k^(60/rate) and the weight 1 - k, the same lag
                    // in wall-clock time (at 60 the loaded constants).
                    per_tick_factor(c, 5); // 0.8: +0x60/+0x70/+0x80, +0x90
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.mfc1(2, 5);
                    c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x60));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3bfc));
                    if tick::rate(c) != 60 {
                        c.f[4] = (FpuF32::from_f32(1.0) - FpuF32(c.f[5])).0; // 0.2
                    }
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3bf8));
                    per_tick_factor(c, 0); // 0.7: +0x94
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3bf4));
                    if tick::rate(c) != 60 {
                        c.f[6] = (FpuF32::from_f32(1.0) - FpuF32(c.f[0])).0; // 0.3
                    }
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
                    if short {
                        pc = 0x002d_d990
                    } else {
                        pc = 0x002d_d97c
                    };
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
                    if have_point {
                        pc = 0x002d_da8c
                    } else {
                        pc = 0x002d_d9ec
                    };
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
                    if wide {
                        pc = 0x002d_da24
                    } else {
                        pc = 0x002d_da20
                    };
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

pub(crate) mod snow_update {

    /// Weather mode: 0 snow (record `f[0..2]`, colour `c0`), 1 the second
    /// kind (record `f[2..4]`, colour `c1`), anything else keeps the
    /// current parameters.
    pub const MODE: u32 = 0x04;

    /// Number of flakes drawn (`(int)(density * k)`).
    pub const FLAKE_COUNT: u32 = 0x08;

    /// Side length of the box (each position component wraps into
    /// `(-size/2, size/2]`).
    pub const SIZE: u32 = 0x0c;

    /// Flake speed (scale of a new drift velocity).
    pub const SPEED: u32 = 0x10;

    /// Copied from the record's `f[4]` in mode 0.
    pub const PARAM_14: u32 = 0x14;

    /// First component of the colour (`c0.x`/`c1.x`).
    pub const COLOR_X: u32 = 0x18;

    /// Wind factor of this box (times the record's `f[7]`).
    pub const WIND_SCALE: u32 = 0x1c;

    /// Box offset (x, y, z, w), wrapped into the box.
    pub const POS: u32 = 0x20;

    /// Drift velocity being blended toward.
    pub const VEL: u32 = 0x30;

    /// Previous drift velocity (blended from).
    pub const PREV_VEL: u32 = 0x40;

    /// Seconds left until a new drift velocity is picked.
    pub const TIMER: u32 = 0x90;

    /// Length of the current drift period (seconds).
    pub const PERIOD: u32 = 0x94;

    /// Remaining colour components (`c.y`, `c.z`, `c.w`).
    pub const COLOR_YZW: u32 = 0xa4;

    /// Non-zero: snap: the previous velocity becomes the new one (cleared).
    pub const SNAP: u32 = 0xb0;

    /// The weather record (`sWakeRec_5DA0`): `float f[8]; Vec4 c0, c1;`.
    pub const REC_DENSITY0: u32 = 0x00;

    pub const REC_SPEED0: u32 = 0x04;

    pub const REC_DENSITY1: u32 = 0x08;

    pub const REC_SPEED1: u32 = 0x0c;

    pub const REC_F4: u32 = 0x10;

    /// Drift heading (scaled into the 512-entry sine table).
    pub const REC_HEADING: u32 = 0x14;

    /// How much of the random part goes into a new velocity (0..1).
    pub const REC_RANDOMNESS: u32 = 0x18;

    pub const REC_WIND: u32 = 0x1c;

    pub const REC_C0: u32 = 0x20;

    pub const REC_C1: u32 = 0x30;

    /// gp+0x2a74: the frame-rate object (its int rate at +0x10).
    pub const GP_RATE_OBJ: i32 = 0x2a74;

    pub const RATE: u32 = 0x10;

    /// gp+0xa0c: the snow's random seed (a float in [1, 2) as bits).
    pub const GP_SEED: i32 = 0xa0c;

    /// gp-0x3a18, gp-0x3a14: the heading-to-table-index factors.
    pub const GP_HEADING_K1: i32 = -0x3a18;

    pub const GP_HEADING_K2: i32 = -0x3a14;

    /// The global wind vector.
    pub const WIND: u32 = 0x004f_f160;

    /// The 512-entry sine table (cos at +0x80 entries).
    pub const SIN_TABLE: u32 = 0x0050_4fb8;

    /// The seed's linear congruential step.
    pub const LCG_MUL: u32 = 0x0001_8fcd;

    pub const LCG_ADD: u32 = 0x00e9_507c;

    pub const MANTISSA: u32 = 0x007f_ffff;

    pub const ONE_BITS: u32 = 0x3f80_0000;
}

pub(crate) mod spray_fx {

    /// The rider object (`cRider*`).
    pub const OBJ: u32 = 0x00;

    /// The ten particle emitters (0x210 bytes each).
    pub const EMITTERS: u32 = 0x28;

    /// Matrix rows 1, 2, 0 of the spray bone (copied each update).
    pub const ROW1: u32 = 0x30;

    pub const ROW2: u32 = 0x40;

    pub const ROW0: u32 = 0x50;

    /// Non-zero: the emitters still have to be set up.
    pub const NEEDS_SETUP: u32 = 0x60;

    /// Last update's speed.
    pub const PREV_SPEED: u32 = 0x74;

    /// Non-zero once the bones were looked up.
    pub const BONES_FOUND: u32 = 0x80;

    /// Surface colour of the snow under the board (rgb * 2, alpha 1).
    pub const COLOUR: u32 = 0x90;

    /// Spray bone position (matrix row 3).
    pub const POS: u32 = 0xa0;

    /// Whether the spray is active this update.
    pub const ACTIVE: u32 = 0xb0;

    /// Signed sideways speed.
    pub const SIDE_SPEED: u32 = 0xb4;

    /// Speed (length of the physics velocity) and its absolute value.
    pub const SPEED: u32 = 0xb8;

    pub const ABS_SPEED: u32 = 0xbc;

    /// Direction of travel (normalised when the speed is positive).
    pub const DIR: u32 = 0xc0;

    /// This rider's race data (game + 0x44, 0xb0 bytes per rider).
    pub const RACE_DATA: u32 = 0xd0;

    /// The rider's main state, as of this update.
    pub const MAIN_STATE: u32 = 0xd4;

    /// The rider's emitter-enable state the emitters were last set for.
    pub const ENABLE_STATE: u32 = 0x128;

    /// Which side the spray goes to (-1, 0 or +1).
    pub const SIDE: u32 = 0x12c;
}

pub(crate) mod spray_rider {

    pub const VELOCITY: u32 = 0x1e0;

    pub const FORWARD_SPEED: u32 = 0x1f0;

    pub const SLIP: u32 = 0x214;

    /// Non-zero: goofy stance (mirrors the spray side).
    pub const GOOFY: u32 = 0x320;

    /// Non-zero: in a trick (the side follows the trick animation).
    pub const IN_TRICK: u32 = 0x330;

    pub const BOARD_DIR: u32 = 0x370;

    pub const INDEX: u32 = 0x438;

    /// The physics sub-object (with its own vtable).
    pub const PHYSICS: u32 = 0x6c0;

    pub const MODEL: u32 = 0x780;

    pub const ANIM: u32 = 0x784;

    pub const ENABLE_STATE: u32 = 0x898;

    pub const SPRAY_BONE: u32 = 0x8a4;

    pub const GRINDING: u32 = 0xac4;

    pub const GRIND_OFF: u32 = 0xad0;

    pub const GRIND_RAIL: u32 = 0xafc;

    pub const GRIND_SPRAY: u32 = 0xb00;
}

/// `void SnowBox_Update(SnowBox* box, const sWakeRec_5DA0* rec)` (0x2e4f50):
/// the per-frame update of the weather flake box.
///
/// 1. Mode 0/1: take density (flake count), speed and colour from the
///    record (the second set for mode 1).
/// 2. Count the drift timer down by one tick (`1 / rate`); when it runs
///    out, pick a new drift velocity (a heading from the record plus a
///    random part, four steps of the seed LCG) and a new period in [1, 5).
/// 3. Move the box offset by the velocity blended from the previous one
///    (`lerp(prev, vel, 1 - timer/period)`) and the wind, both times the
///    tick, then wrap it into the box (as `SnowBox_WrapOffset`).
pub fn snow_box_update(c: &mut Cpu) {
    use crate::game::visualfx::boardwakefx::{
        copy_q, fconst, fr, frac_times, gp, ldf, p, setf, vadd_2,
    };
    use snow_update::*;

    c.cycles += 5;
    c.g[T2][0] = c.g[A0][0];
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
    let this = r32(c, T2);
    let rec = r32(c, A1);
    let mode = c.lw(p(this, MODE));
    c.g[V1][0] = mode;
    c.g[T3][0] = c.g[A1][0];

    // Step 1: the weather parameters of the mode.
    if mode == 0 {
        c.cycles += 20;
        c.lwc1(1, p(rec, REC_C0));
        let k = fconst(c, 3, 0x43fa_0000); // 500
        c.swc1(1, p(this, COLOR_X));
        for i in 0..3u32 {
            c.lwc1(i as usize, p(rec, REC_C0 + 4 + 4 * i));
            c.swc1(i as usize, p(this, COLOR_YZW + 4 * i));
        }
        let n = ldf(c, 0, p(rec, REC_DENSITY0)) * k;
        setf(c, 0, n);
        c.cvt_w_s(1, 0);
        c.swc1(1, p(this, FLAKE_COUNT));
        let s = ldf(c, 1, p(rec, REC_SPEED0)) * k;
        setf(c, 1, s);
        c.swc1(1, p(this, SPEED));
        c.lwc1(0, p(rec, REC_F4));
        c.swc1(0, p(this, PARAM_14));
        c.cycles += 1;
        c.g[V0][0] = c.lw(gp(c, GP_RATE_OBJ));
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp(c, GP_RATE_OBJ));
        if mode == 1 {
            c.cycles += 19;
            c.lwc1(0, p(rec, REC_C1));
            let k_count = fconst(c, 3, 0x40c0_0000); // 6
            c.swc1(0, p(this, COLOR_X));
            let k_speed = fconst(c, 4, 0x437a_0000); // 250
            for i in 0..3u32 {
                c.lwc1(i as usize, p(rec, REC_C1 + 4 + 4 * i));
                c.swc1(i as usize, p(this, COLOR_YZW + 4 * i));
            }
            let n = ldf(c, 0, p(rec, REC_DENSITY1)) * k_count;
            setf(c, 0, n);
            c.cvt_w_s(1, 0);
            c.swc1(1, p(this, FLAKE_COUNT));
            let s = ldf(c, 1, p(rec, REC_SPEED1)) * k_speed;
            setf(c, 1, s);
            c.swc1(1, p(this, SPEED));
            c.cycles += 1;
            c.g[V0][0] = c.lw(gp(c, GP_RATE_OBJ));
        } else {
            c.cycles += 2;
        }
    }

    // Step 2: the drift timer (f11 = 1, f12 = tick, f13 = 0 stay live).
    c.cycles += 12;
    let one = fconst(c, 11, 0x3f80_0000);
    let rate_obj = r32(c, V0);
    c.lwc1(1, p(rate_obj, RATE));
    c.cvt_s_w(1, 1);
    let timer = ldf(c, 0, p(this, TIMER));
    c.f[13] = 0;
    let dt = one.div(fr(c, 1));
    setf(c, 12, dt);
    setf(c, 0, timer - dt);
    c.c_lt_s(0, 13);
    let expired = c.fcc();
    c.swc1(0, p(this, TIMER));
    if expired {
        snow_box_new_drift(c, this, rec);
    }

    // Step 3: move by the blended velocity and the wind, then wrap.
    c.cycles += 66;
    let period = ldf(c, 2, p(this, PERIOD));
    c.g[A0][0] = sx(WIND);
    let timer = ldf(c, 1, p(this, TIMER));
    let one = fconst(c, 0, 0x3f80_0000);
    let t = timer.div(period);
    setf(c, 1, t);
    fconst(c, 3, 0x3f00_0000); // 0.5
    setf(c, 0, one - t);
    let sp = sp(c);
    snow_scale_q(c, 0, p(this, VEL), p(sp, 0x30), p(sp, 0x20)); // vel * (1 - t)
    snow_scale_q(c, 1, p(this, PREV_VEL), p(sp, 0x40), p(sp, 0x30)); // prev * t
    vadd_2(c, p(sp, 0x20), p(sp, 0x30), p(sp, 0x40));
    copy_q(c, V0, p(sp, 0x40), p(sp, 0x10));
    snow_scale_q(c, 12, p(sp, 0x10), p(sp, 0x40), sp); // * tick
    vadd_2(c, p(this, POS), sp, p(this, POS));
    let wind = ldf(c, 0, p(this, WIND_SCALE)) * ldf(c, 1, p(rec, REC_WIND));
    setf(c, 0, wind);
    let wind = wind * fr(c, 12);
    setf(c, 0, wind);
    snow_scale_q(c, 0, WIND, p(sp, 0x10), sp); // wind * scale * tick
    vadd_2(c, p(this, POS), sp, p(this, POS));

    // The wrap: x in f4, y in f3, z in f5; results in f4, f6, f7.
    c.lwc1(2, p(this, SIZE));
    c.lwc1(4, p(this, POS));
    c.mul_s(0, 2, 3); // f0 = h = size * 0.5
    c.lwc1(6, p(this, POS + 4));
    c.lwc1(7, p(this, POS + 8));
    c.mov_s(3, 6);
    c.c_lt_s(0, 4); // h < x ?
    let x_above = c.fcc();
    c.mov_s(5, 7);
    'x: {
        'x_tail: {
            if x_above {
                c.cycles += 9;
                let (v, s) = (fr(c, 4), fr(c, 2));
                let r = frac_times(c, v, s);
                setf(c, 4, r - s);
                break 'x_tail;
            }
            c.cycles += 5;
            c.neg_s(0, 0); // f0 = -h
            c.c_lt_s(4, 0); // x < -h ?
            if !c.fcc() {
                c.lwc1(0, p(this, SIZE));
                break 'x;
            }
            c.cycles += 8;
            let (v, s) = (fr(c, 4), fr(c, 2));
            let r = frac_times(c, v, s);
            setf(c, 4, r + s);
        }
        c.cycles += 1;
        c.lwc1(0, p(this, SIZE));
    }
    'y: {
        'y_below: {
            c.cycles += 8;
            fconst(c, 1, 0x3f00_0000);
            c.mov_s(2, 0); // f2 = size
            c.mul_s(0, 0, 1); // f0 = h
            c.c_lt_s(0, 3); // h < y ?
            if !c.fcc() {
                c.neg_s(0, 0);
                break 'y_below;
            }
            c.cycles += 9;
            let (v, s) = (fr(c, 3), fr(c, 2));
            let r = frac_times(c, v, s);
            setf(c, 6, r - s);
            break 'y;
        }
        c.cycles += 3;
        c.c_lt_s(3, 0); // y < -h ?
        if !c.fcc() {
            break 'y;
        }
        c.cycles += 8;
        let (v, s) = (fr(c, 3), fr(c, 2));
        let r = frac_times(c, v, s);
        setf(c, 6, r + s);
    }
    'z: {
        'z_below: {
            c.cycles += 7;
            fconst(c, 0, 0x3f00_0000);
            c.mul_s(0, 2, 0); // f0 = h
            c.c_lt_s(0, 5); // h < z ?
            if !c.fcc() {
                c.neg_s(0, 0);
                break 'z_below;
            }
            c.cycles += 9;
            let (v, s) = (fr(c, 5), fr(c, 2));
            let r = frac_times(c, v, s);
            setf(c, 7, r - s);
            break 'z;
        }
        c.cycles += 3;
        c.c_lt_s(5, 0); // z < -h ?
        if !c.fcc() {
            break 'z;
        }
        c.cycles += 8;
        let (v, s) = (fr(c, 5), fr(c, 2));
        let r = frac_times(c, v, s);
        setf(c, 7, r + s);
    }
    c.cycles += 10;
    fconst(c, 0, 0x3f80_0000); // w = 1
    c.swc1(4, sp);
    c.swc1(6, p(sp, 4));
    c.swc1(7, p(sp, 8));
    c.swc1(0, p(sp, 0xc));
    copy_q(c, V0, sp, p(this, POS));
    c.g[SP][0] = sx(sp.wrapping_add(0x50));
    ret(c);
}

/// `mfc1 v1, f; lqc2 vf4, src; qmtc2 v1, vf3; vmul.xyzw vf5, vf4, vf3x;
/// sqc2 vf5, tmp; lq v0, tmp; sq v0, out`: `out = src * f` through the
/// stack slot `tmp`.
#[inline(always)]
pub(crate) fn snow_scale_q(c: &mut Cpu, f: usize, src: u32, tmp: u32, out: u32) {
    use ssx3_ee_rt::vu0::{Src, X};
    c.mfc1(V1, f);
    c.lqc2(4, src);
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, tmp);
    c.lq(V0, tmp);
    c.sq(V0, out);
}

/// One step of the snow seed: `((s * 0x18fcd + 0xe9507c) & 0x7fffff) |
/// 0x3f800000` (a float in [1, 2)); the `mult` leaves LO/HI.
#[inline(always)]
pub(crate) fn snow_lcg(c: &mut Cpu, s: u32) -> u32 {
    use snow_update::*;
    let lo = crate::abi::mult(c, s, LCG_MUL);
    lo.wrapping_add(LCG_ADD) & MANTISSA | ONE_BITS
}

/// The expired-timer block of [`snow_box_update`]: the previous velocity
/// becomes the current one, a new velocity is picked (`speed` along the
/// record's heading, mixed with a random vector in the speed cube by the
/// record's randomness) with a new period in [1, 5), and with `SNAP` set the
/// blend starts at the new velocity.
pub(crate) fn snow_box_new_drift(c: &mut Cpu, this: u32, rec: u32) {
    use crate::game::visualfx::boardwakefx::{copy_q, fr, gp, p, setf};
    use crate::ps2f::FpuF32;
    use snow_update::*;

    c.cycles += 93;
    let seed_at = gp(c, GP_SEED);
    let seed = c.lw(seed_at) as u32;
    copy_q(c, V1, p(this, VEL), p(this, PREV_VEL));
    let speed = FpuF32(c.lw(p(this, SPEED)) as u32);
    let heading = FpuF32(c.lw(p(rec, REC_HEADING)) as u32);
    let k1 = FpuF32(c.lw(gp(c, GP_HEADING_K1)) as u32);
    let k2 = FpuF32(c.lw(gp(c, GP_HEADING_K2)) as u32);
    let mix = FpuF32(c.lw(p(rec, REC_RANDOMNESS)) as u32);
    let one = fr(c, 11);

    // Four LCG steps, each stored back as the seed.
    let r1 = snow_lcg(c, seed);
    c.sw(seed_at, r1 as u64);
    let r2 = snow_lcg(c, r1);
    c.sw(seed_at, r2 as u64);
    let r3 = snow_lcg(c, r2);
    c.sw(seed_at, r3 as u64);
    let r4 = snow_lcg(c, r3);

    // The heading's table index (cvt.w through f1 into v0).
    setf(c, 0, heading * k1 * k2);
    c.cvt_w_s(1, 0);
    c.mfc1(V0, 1);
    let idx = r32(c, V0) & 0x1ff;
    let sin = FpuF32(c.lw(SIN_TABLE + idx * 4) as u32);
    let cos = FpuF32(c.lw(SIN_TABLE + (idx + 0x80) * 4) as u32);

    let low = -speed;
    let span = speed - low;
    let u1 = FpuF32(r1) - one;
    let u2 = FpuF32(r2) - one;
    let u3 = FpuF32(r3) - one;
    let x0 = low + span * u1;
    let y0 = low + span * u2;
    let span_z = span * u3;
    let z0 = low + span_z;
    let sin_s = speed * sin;
    let cos_s = speed * cos;
    let keep = one - mix;
    let mix_x = mix * x0;
    let mix_y = mix * y0;
    let vx = cos_s * keep + mix_x;
    let vy = sin_s * keep + mix_y;
    let vz = mix * z0;

    c.g[V0][0] = c.lw(p(this, SNAP));
    let sp = sp(c);
    c.swc1(13, p(sp, 0x1c));
    c.sw(p(sp, 4), sin_s.0 as u64);
    c.sw(sp, cos_s.0 as u64);
    c.swc1(13, p(sp, 8));
    c.swc1(13, p(sp, 0xc));
    c.sw(p(sp, 0x14), vy.0 as u64);
    c.sw(p(sp, 0x18), vz.0 as u64);
    c.sw(p(sp, 0x10), vx.0 as u64);
    copy_q(c, V1, p(sp, 0x10), p(this, VEL));
    c.sw(seed_at, r4 as u64);
    let four = FpuF32(0x4080_0000);
    let period = (FpuF32(r4) - one) * four + one;
    c.sw(p(this, TIMER), period.0 as u64);
    c.sw(p(this, PERIOD), period.0 as u64);

    // The registers the original leaves.
    c.g[AT][0] = 0x4080_0000;
    c.g[A0][0] = r4 as u64;
    c.g[A1][0] = sx(SIN_TABLE);
    c.g[A2][0] = LCG_ADD as u64;
    c.g[A3][0] = LCG_MUL as u64;
    c.g[T0][0] = MANTISSA as u64;
    c.g[T1][0] = ONE_BITS as u64;
    setf(c, 0, period);
    setf(c, 1, mix_y);
    setf(c, 2, u3);
    setf(c, 3, cos);
    setf(c, 4, mix_x);
    setf(c, 5, vx);
    setf(c, 6, span_z);
    setf(c, 7, vz);
    setf(c, 8, z0);
    setf(c, 9, keep);
    setf(c, 10, four);

    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.sq(V1, p(this, PREV_VEL));
        c.sw(p(this, SNAP), 0);
    }
}

/// `s0 + off` (the spray), `base + off` for any other object.
#[inline(always)]
pub(crate) fn spray_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `void cDynamicSprayFX_Update(cDynamicSprayFX* self)` (0x2df920): the
/// per-tick update of a rider's board spray. Sets the emitters and bones up
/// on first use, picks the spray side (from the trick animation and the
/// stance), measures the rider's sideways and forward speed and direction,
/// decides whether the spray is active (not in the air states, not in a
/// sub-state 9, grinding only on a spraying rail), copies the spray bone's
/// matrix, queues a burst on a sudden slow-down (or a rail landing), re-arms
/// the emitters when the rider's enable state changed, samples the snow
/// colour under the board, and runs every enabled emitter's update.
pub fn c_dynamic_spray_fx_update(c: &mut Cpu) {
    use spray_callee as callee;
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        // Prologue: s0 = self; s0, s1, ra and f20 saved.
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 20, 0x60);
        let this = c.g[S0][0];
        c.g[V0][0] = c.lw(spray_at(this, spray_fx::NEEDS_SETUP));
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            callee::SETUP_EMITTERS.call(c, 0x002d_f94c)?;
        }
        c.g[V0][0] = c.lw(spray_at(this, spray_fx::BONES_FOUND));

        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[A0][0] = this;
            callee::FIND_BONES.call(c, 0x002d_f960)?;
        }
        c.sw(spray_at(this, spray_fx::SIDE), 0);

        // The spray side: during a trick, from the trick animation (anims
        // 23-28, 37 and 38 spray to the stance side, all others to the
        // other side); mirrored for goofy riders.
        c.cycles += 4;
        c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));
        c.g[V0][0] = c.lw(spray_at(c.g[V1][0], spray_rider::IN_TRICK));
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            c.g[A0][0] = c.lw(spray_at(c.g[V1][0], spray_rider::ANIM));
            c.g[A1][0] = 0x2;
            call_ptr(c, callee::ANIM_LAYER_ID, 0x002d_f980)?;
            let anim = c.g[V0][0];
            c.g[V1][0] = anim;
            c.g[V0][0] = (sx((anim as u32).wrapping_sub(0x17)) < 0x6) as u64;
            let stance_side = if c.g[V0][0] != 0 {
                // Anims 23-28.
                true
            } else {
                // Anims 37 and 38 (v0 is left 0x26 either way).
                c.cycles += 3;
                c.g[V0][0] = 0x26;
                let hit = if anim == 0x25 {
                    true
                } else {
                    c.cycles += 2;
                    anim == 0x26
                };
                if hit {
                    c.cycles += 1;
                }
                hit
            };
            c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));
            // +1.0 or -1.0, the other one for goofy riders.
            let (regular, goofy, goofy_cost): (u64, u64, u64) = if stance_side {
                (0x3f80_0000, sx(0xbf80_0000), 4)
            } else {
                (sx(0xbf80_0000), 0x3f80_0000, 3)
            };
            c.cycles += 5;
            c.g[AT][0] = regular;
            c.mtc1(AT, 0);
            c.g[V0][0] = c.lw(spray_at(c.g[V1][0], spray_rider::GOOFY));
            if c.g[V0][0] != 0 {
                c.cycles += goofy_cost;
                c.g[AT][0] = goofy;
                c.mtc1(AT, 0);
            }
            c.swc1(0, spray_at(this, spray_fx::SIDE));
            c.cycles += 1;
            c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));
        }

        // Sideways speed: -forward * (1 - |slip|), mirrored for goofy.
        c.cycles += 12;
        let obj = c.g[V1][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        c.lwc1(0, spray_at(obj, spray_rider::SLIP));
        c.lwc1(1, spray_at(obj, spray_rider::FORWARD_SPEED));
        c.abs_s(0, 0);
        c.sub_s(2, 2, 0);
        c.neg_s(1, 1);
        c.mul_s(0, 1, 2);
        c.swc1(0, spray_at(this, spray_fx::SIDE_SPEED));
        c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GOOFY));
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.neg_s(0, 0);
            c.swc1(0, spray_at(this, spray_fx::SIDE_SPEED));
        }
        c.g[A0][0] = c.lw(spray_at(this, spray_fx::OBJ));

        // Speed = |physics velocity| (virtual call on the physics
        // sub-object), direction = the rider's velocity, normalised.
        c.cycles += 26;
        let obj = c.g[A0][0];
        c.g[V1][0] = c.lw(spray_at(obj, spray_rider::PHYSICS));
        c.g[A0][0] = sx(spray_at(obj, spray_rider::PHYSICS));
        c.g[A1][0] = c.lh(spray_at(c.g[V1][0], 0x10));
        c.g[V0][0] = c.lw(spray_at(c.g[V1][0], 0x14));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, method, 0x002d_fa48)?;
        c.lqc2(3, c.g[V0][0] as u32);
        spray_vu0_length(c);
        c.cfc2(A0, 22);
        c.mtc1(A0, 0);
        c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));
        c.abs_s(1, 0);
        c.swc1(0, spray_at(this, spray_fx::SPEED));
        c.swc1(1, spray_at(this, spray_fx::ABS_SPEED));
        c.mtc1(0, 2);
        c.lq(V0, spray_at(c.g[V1][0], spray_rider::VELOCITY));
        c.c_lt_s(2, 0);
        let moving = c.fcc();
        c.sq(V0, spray_at(this, spray_fx::DIR));
        if moving {
            c.cycles += 17;
            c.lqc2(3, spray_at(this, spray_fx::DIR));
            spray_vu0_length(c);
            c.cfc2(V0, 22);
            c.mtc1(V0, 0);
            c.mfc1(V1, 0);
            c.qmtc2(V1, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, spray_at(this, spray_fx::DIR));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            c.sqc2(5, spray_at(this, spray_fx::DIR));
        }

        // Active unless the main state is 1, 2 or 4, the sub-state is 9, or
        // the rider grinds a rail that does not spray.
        c.cycles += 9;
        c.g[S1][0] = 0x1;
        c.g[A0][0] = c.lw(spray_at(this, spray_fx::OBJ));
        c.sw(spray_at(this, spray_fx::ACTIVE), c.g[S1][0]);
        call_ptr(c, callee::RIDER_MAIN_STATE, 0x002d_faec)?;
        let main_state = c.g[V0][0];
        c.g[V1][0] = main_state;
        c.g[V0][0] = (sx((main_state as u32).wrapping_sub(0x1)) < 0x2) as u64;
        c.sw(spray_at(this, spray_fx::MAIN_STATE), main_state);
        // None: inactive through the shared store (+1); Some(true): store
        // here; Some(false): stays active.
        let inactive: Option<bool> = if c.g[V0][0] != 0 {
            None
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            if main_state == 0x4 {
                Some(true)
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.lw(spray_at(this, spray_fx::OBJ));
                call_ptr(c, callee::RIDER_SUB_STATE, 0x002d_fb14)?;
                c.g[V1][0] = 0x9;
                if c.g[V0][0] == 0x9 {
                    Some(true)
                } else {
                    c.cycles += 4;
                    let obj = c.lw(spray_at(this, spray_fx::OBJ));
                    c.g[A0][0] = obj;
                    c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRINDING));
                    if c.g[V0][0] == 0 {
                        Some(false)
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRIND_OFF));
                        c.g[V1][0] = 0;
                        if c.g[V0][0] == 0 {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRIND_RAIL));
                            c.g[V1][0] = if c.g[V0][0] == 0 { 0 } else { c.g[S1][0] };
                        }
                        c.cycles += 2;
                        if c.g[V1][0] == 0 {
                            Some(true)
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRIND_SPRAY));
                            if c.g[V0][0] != 0 {
                                Some(false)
                            } else {
                                None
                            }
                        }
                    }
                }
            }
        };
        match inactive {
            Some(false) => {}
            Some(true) => {
                c.sw(spray_at(this, spray_fx::ACTIVE), 0);
                c.cycles += 1;
            }
            None => {
                c.cycles += 1;
                c.sw(spray_at(this, spray_fx::ACTIVE), 0);
                c.cycles += 1;
            }
        }
        c.g[T0][0] = c.lw(spray_at(this, spray_fx::OBJ));

        // Copy the spray bone's matrix rows (model matrices + bone * 64),
        // find this rider's race data, and the slow-down rate
        // (prev - speed) * 60 (f20).
        c.cycles += 49;
        let obj = c.g[T0][0];
        c.g[A3][0] = 0xb0;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[T1][0] = 0;
        c.g[A0][0] = c.lw(spray_at(obj, spray_rider::MODEL));
        c.g[V0][0] = c.lw(spray_at(obj, spray_rider::SPRAY_BONE));
        c.g[V1][0] = c.lw(spray_at(c.g[A0][0], 0x30));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
        c.lwc1(1, spray_at(this, spray_fx::PREV_SPEED));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.lwc1(2, spray_at(this, spray_fx::SPEED));
        c.lq(V0, c.g[V1][0] as u32);
        c.sub_s(1, 1, 2);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3b88));
        c.sq(V0, spray_at(this, spray_fx::ROW0));
        c.sq(V0, sp(c));
        c.mul_s(20, 1, 0);
        c.g[V0][0] = c.lw(spray_at(obj, spray_rider::MODEL));
        c.g[A0][0] = c.lw(spray_at(obj, spray_rider::SPRAY_BONE));
        c.g[V1][0] = c.lw(spray_at(c.g[V0][0], 0x30));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 6);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3b84));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.lq(V0, spray_at(c.g[V1][0], 0x10));
        c.c_lt_s(0, 20);
        c.sq(V0, spray_at(this, spray_fx::ROW1));
        c.sq(V0, sp(c));
        for (row, dst) in [(0x20u32, spray_fx::ROW2), (0x30, spray_fx::POS)] {
            c.g[A1][0] = c.lw(spray_at(obj, spray_rider::MODEL));
            c.g[V1][0] = c.lw(spray_at(obj, spray_rider::SPRAY_BONE));
            c.g[A0][0] = c.lw(spray_at(c.g[A1][0], 0x30));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.lq(V0, spray_at(c.g[A0][0], row));
            c.sq(V0, spray_at(this, dst));
            c.sq(V0, sp(c));
        }
        c.g[V1][0] = c.lw(spray_at(obj, spray_rider::INDEX));
        c.g[A0][0] = c.lw(spray_at(c.g[A2][0], 0x84));
        c.mult(0, V1, V1, A3);
        c.g[V0][0] = c.lw(spray_at(c.g[A0][0], 0x44));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let fast_slowdown = c.fcc();
        c.sw(spray_at(this, spray_fx::RACE_DATA), c.g[V0][0]);
        // burst (t1): a hard slow-down from a high speed, not in sub-state 9.
        let mut skip_reload_cost = false;
        if fast_slowdown {
            c.cycles += 4;
            c.c_lt_s(0, 2);
            if !c.fcc() {
                skip_reload_cost = true;
            } else {
                c.cycles += 4;
                c.g[A0][0] = obj;
                call_ptr(c, callee::RIDER_SUB_STATE, 0x002d_fc40)?;
                c.g[V0][0] ^= 0x9;
                c.g[T1][0] = (0 < c.g[V0][0]) as u64;
            }
        }
        if !skip_reload_cost {
            c.cycles += 1;
        }
        c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));

        // On a spraying rail the full rate counts and any slow-down of
        // 27.8 or more bursts; otherwise half the rate (f12).
        c.cycles += 6;
        let obj = c.g[V1][0];
        c.g[A0][0] = 0;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRINDING));
        c.mul_s(12, 20, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRIND_OFF));
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw(spray_at(obj, spray_rider::GRIND_RAIL));
                c.g[A0][0] = (0 < c.g[V0][0]) as u64;
            }
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3b80));
            c.c_le_s(0, 20);
            let burst = c.fcc();
            c.g[T1][0] = 0x1;
            if !burst {
                c.cycles += 1;
                c.g[T1][0] = 0;
            }
            c.cycles += 2;
            c.mov_s(12, 20);
        }
        c.g[V0][0] = c.lw(spray_at(this, spray_fx::ACTIVE));

        // Queue the burst at pos + board direction * 10.
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            let burst = c.g[T1][0] != 0;
            c.g[A0][0] = this;
            if burst {
                c.cycles += 22;
                c.g[V0][0] = c.lw(spray_at(this, spray_fx::OBJ));
                c.g[AT][0] = 0x4120_0000;
                c.mtc1(AT, 0);
                c.g[A1][0] = c.g[SP][0];
                c.mfc1(V1, 0);
                c.lqc2(4, spray_at(c.g[V0][0], spray_rider::BOARD_DIR));
                c.qmtc2(V1, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x20));
                c.g[T0][0] = 0x1;
                c.lq(V0, frame.at(c, 0x20));
                c.sq(V0, frame.at(c, 0x10));
                c.lqc2(3, spray_at(this, spray_fx::POS));
                c.lqc2(4, frame.at(c, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x20));
                c.lq(V0, frame.at(c, 0x20));
                c.g[V1][0] = c.lw(spray_at(this, spray_fx::OBJ));
                c.sq(V0, sp(c));
                c.g[A2][0] = sx(spray_at(c.g[V1][0], spray_rider::BOARD_DIR));
                c.g[A3][0] = c.lw(spray_at(c.g[V1][0], spray_rider::INDEX));
                callee::QUEUE_BURST.call(c, 0x002d_fd08)?;
            }
            c.cycles += 1;
        }
        c.lwc1(0, spray_at(this, spray_fx::SPEED));

        // Remember the speed; re-arm the emitters when the rider's enable
        // state changed.
        c.cycles += 6;
        c.g[A0][0] = c.lw(spray_at(this, spray_fx::OBJ));
        c.swc1(0, spray_at(this, spray_fx::PREV_SPEED));
        c.g[V1][0] = c.lw(spray_at(this, spray_fx::ENABLE_STATE));
        c.g[V0][0] = c.lw(spray_at(c.g[A0][0], spray_rider::ENABLE_STATE));
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 3;
            c.g[A0][0] = this;
            callee::UPDATE_EMITTER_ENABLES.call(c, 0x002d_fd2c)?;
        }
        c.g[A0][0] = c.lw(spray_at(this, spray_fx::OBJ));

        // Snow colour: the physics sub-object's surface index (virtual
        // call) selects an entry of the surface table (0xf0 bytes each);
        // colour = (g, b, a, r) * 2 with alpha 1.
        c.cycles += 35;
        let obj = c.g[A0][0];
        c.g[V1][0] = c.lw(spray_at(obj, spray_rider::PHYSICS));
        c.g[A0][0] = sx(spray_at(obj, spray_rider::PHYSICS));
        c.g[A1][0] = c.lh(spray_at(c.g[V1][0], 0x38));
        c.g[V0][0] = c.lw(spray_at(c.g[V1][0], 0x3c));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, method, 0x002d_fd48)?;
        c.g[A0][0] = 0xf0;
        c.g[V1][0] = 0x50_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5c68));
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(AT, 4);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 5);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let entry = c.g[V0][0];
        c.lwc1(3, spray_at(entry, 0x0));
        c.lwc1(0, spray_at(entry, 0x4));
        c.lwc1(1, spray_at(entry, 0x8));
        c.lwc1(2, spray_at(entry, 0xc));
        c.swc1(0, sp(c));
        c.swc1(1, frame.at(c, 0x4));
        c.swc1(2, frame.at(c, 0x8));
        c.swc1(3, frame.at(c, 0xc));
        c.lq(V0, sp(c));
        c.sq(V0, spray_at(this, spray_fx::COLOUR));
        c.mfc1(V1, 4);
        c.lqc2(4, spray_at(this, spray_fx::COLOUR));
        c.qmtc2(V1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, spray_at(this, spray_fx::COLOUR));
        c.swc1(5, spray_at(this, spray_fx::COLOUR + 0xc));

        // Run every enabled emitter (flag at emitters + 0x174 + n * 0x210).
        let [first, second, middle @ .., last] = callee::EMITTERS;
        c.g[V0][0] = c.lw(spray_at(this, spray_fx::EMITTERS));
        c.g[V1][0] = c.lw(spray_at(c.g[V0][0], first.0));
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = this;
            first.1.call(c, first.2)?;
            c.g[V0][0] = c.lw(spray_at(this, spray_fx::EMITTERS));
        }
        c.g[V1][0] = c.lw(spray_at(c.g[V0][0], second.0));
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = this;
            second.1.call(c, second.2)?;
        }
        c.g[V0][0] = c.lw(spray_at(this, spray_fx::EMITTERS));
        for (flag, update, ret) in middle {
            c.cycles += 3;
            c.g[V1][0] = c.lw(spray_at(c.g[V0][0], flag));
            if c.g[V1][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = this;
                update.call(c, ret)?;
            }
            c.g[V0][0] = c.lw(spray_at(this, spray_fx::EMITTERS));
        }
        c.cycles += 3;
        c.g[V1][0] = c.lw(spray_at(c.g[V0][0], last.0));
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = this;
            last.1.call(c, last.2)?;
        }

        // Epilogue.
        frame.restore(c, S0, 0x50);
        c.cycles += 5;
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// VU0 `|vf3.xyz|` into Q (vf4 = vf3 * vf3, vf6.w = vf0.w, ACC sum, sqrt).
#[inline(always)]
pub(crate) fn spray_vu0_length(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
}

pub(crate) mod spray7 {

    /// The rider (`+0x370` up vector, `+0x3b0` side axis, `+0x1e0` velocity).
    pub const OBJ: u32 = 0x00;

    /// Base of the dynamic emitter array (0x210 bytes per slot).
    pub const P28: u32 = 0x28;

    /// Curve table (`+0x730..0x738`: up/normal/side spawn scales).
    pub const P2C: u32 = 0x2c;

    /// The edge normal the spray is thrown along.
    pub const NORMAL: u32 = 0x50;

    /// Spawn colour.
    pub const COL: u32 = 0x90;

    /// Spray position.
    pub const POS: u32 = 0xa0;

    /// Non-zero while the board is carving.
    pub const ACTIVE: u32 = 0xb0;

    /// Side of the board (negative flips the throw direction).
    pub const SIDE: u32 = 0xb4;

    /// Ground speed.
    pub const SPEED: u32 = 0xb8;

    /// Scale of the up-vector offset.
    pub const UP_SCALE: u32 = 0xbc;

    /// Board direction (dotted with the rider's side axis).
    pub const DIR: u32 = 0xc0;

    /// Configuration (`+0x50` enabled, `+0x54` throw scale).
    pub const CFG: u32 = 0xd0;

    /// Turn rate: the spray only spawns while it is small.
    pub const TURN: u32 = 0x12c;

    /// The emitter slot this update drives (7 * 0x210).
    pub const SLOT: u32 = 0xe70;
}

/// `void cDynamicSprayFX_UpdateEmitter7(cDynamicSprayFX* fx)` (0x2e1a80): the
/// carve spray. The spawn chance grows with how sideways the board runs
/// (`2|dir . side|`, capped at 1), the rider's lean (`+0x1f0`) and the square
/// of the speed above 416.67 (over 1250). While carving, with the spray
/// enabled and a small turn rate, a random draw below that chance spawns one
/// particle in slot 7: thrown along the edge normal (flipped by the side) plus
/// the rider's velocity and up vector, at the trail point, scaled by the
/// curve table. Otherwise slot 7 is kept alive at the spray position.
pub fn dynamic_spray_fx_update_emitter7(c: &mut Cpu) {
    use spray7::*;
    guest(c, |c| {
        // chance = min(2 |dir . side|, 1).
        c.cycles += 26;
        let frame = Frame::open(c, 0x150);
        bw::fconst(c, 6, 0x3f80_0000); // 1
        frame.save(c, S1, 0x130);
        frame.save(c, S2, 0x120);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x140);
        c.mov_s(1, 6);
        frame.save_ra(c, 0x110);
        let fx = r32(c, S1);
        let sp0 = sp(c);
        c.g[T1][0] = c.lw(fx);
        c.lqc2(3, bw::p(r32(c, T1), 0x3b0));
        c.lqc2(5, bw::p(fx, DIR));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(V0, 4);
        c.mtc1(V0, 0);
        c.g[V1][0] = c.lw(bw::p(fx, P28));
        c.abs_s(0, 0);
        c.add_s(0, 0, 0);
        c.c_le_s(0, 6); // 2|d| <= 1 ?
        let small = c.fcc();
        c.g[S2][0] = sx(r32(c, V1).wrapping_add(SLOT));
        if small {
            c.cycles += 1;
            c.mov_s(1, 0);
        }
        // s = min((speed - 416.67) / 1250, 1); lean = |rider+0x1f0| + |chance| - 0.1.
        c.cycles += 15;
        c.abs_s(4, 1);
        c.lwc1(2, bw::gp(c, -0x3a90)); // 416.66669
        c.lwc1(1, bw::p(fx, SPEED));
        c.mov_s(5, 6);
        c.lwc1(3, bw::gp(c, -0x3a8c)); // 1250
        c.sub_s(1, 1, 2);
        c.lwc1(0, bw::p(r32(c, T1), 0x1f0));
        c.abs_s(0, 0);
        c.lwc1(2, bw::gp(c, -0x3a88)); // 0.1
        c.add_s(0, 0, 4);
        c.div_s(1, 1, 3);
        c.c_lt_s(1, 6);
        let under = c.fcc();
        c.sub_s(0, 0, 2);
        if under {
            c.cycles += 1;
            c.mov_s(5, 1);
        }
        'idle: {
            // A random draw in [0, 0.4) against lean * s^2.
            c.cycles += 21;
            c.g[V0][0] = c.lw(bw::gp(c, 0xa0c));
            c.g[A1][0] = 0x0001_8fcd;
            c.g[A0][0] = 0x00e9_507c;
            c.mult(0, 2, 2, 5);
            c.g[A2][0] = 0x007f_ffff;
            c.g[T2][0] = 0x3f80_0000;
            c.mul_s(5, 5, 5);
            c.g[V1][0] = c.lw(bw::p(fx, ACTIVE));
            c.g[V0][0] = bw::add32(c.g[V0][0], c.g[A0][0]);
            c.lwc1(1, bw::gp(c, -0x3a84)); // 0.4
            c.g[V0][0] &= c.g[A2][0];
            c.mul_s(2, 0, 5);
            c.g[T0][0] = c.g[V0][0] | c.g[T2][0];
            c.sw(bw::gp(c, 0xa0c), c.g[T0][0]);
            c.lwc1(0, bw::gp(c, 0xa0c));
            c.sub_s(0, 0, 6);
            let inactive = c.g[V1][0] == 0;
            c.mul_s(0, 0, 1);
            if inactive {
                break 'idle;
            }
            c.cycles += 3;
            c.c_le_s(0, 2);
            if !c.fcc() {
                break 'idle;
            }
            c.cycles += 4;
            c.g[A3][0] = c.lw(bw::p(fx, CFG));
            c.g[V0][0] = c.lw(bw::p(r32(c, A3), 0x50));
            if c.g[V0][0] == 0 {
                break 'idle;
            }
            c.cycles += 7;
            c.lwc1(0, bw::p(fx, TURN));
            c.lwc1(1, bw::gp(c, -0x3a80)); // 0.2
            c.abs_s(0, 0);
            c.c_lt_s(0, 1);
            let turning = !c.fcc();
            c.mult(0, 2, 8, 5);
            if turning {
                break 'idle;
            }
            // Spawn: k = (next random - 0.5) * cfg scale; the throw is
            // pos + up * k*3 * 1/60 ... (see the vector steps below).
            c.cycles += 69;
            bw::fconst(c, 4, 0x3f00_0000); // 0.5
            bw::fconst(c, 3, 0x4040_0000); // 3
            c.lwc1(2, bw::gp(c, -0x3a7c)); // 1/60
            c.mtc1(0, 5); // f5 = 0
            c.g[V1][0] = bw::add32(c.g[V0][0], c.g[A0][0]);
            c.lq(2, bw::p(fx, POS));
            c.g[V1][0] &= c.g[A2][0];
            c.g[V1][0] |= c.g[T2][0];
            c.sq(2, sp0);
            c.lwc1(1, bw::p(r32(c, A3), 0x54));
            c.sw(bw::gp(c, 0xa0c), c.g[V1][0]);
            c.lwc1(0, bw::gp(c, 0xa0c));
            c.sub_s(0, 0, 6);
            c.add_s(0, 0, 4);
            c.mul_s(1, 0, 1);
            // up * f1 -> sp+0x20.
            c.mfc1(3, 1);
            c.lqc2(4, bw::p(r32(c, T1), 0x370));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x30));
            c.lq(2, bw::p(sp0, 0x30));
            c.g[V1][0] = c.lw(fx);
            c.sq(2, bw::p(sp0, 0x20));
            // velocity * f3 * f2 * f0 -> sp+0x30.
            c.mfc1(4, 3);
            c.lqc2(4, bw::p(r32(c, V1), 0x1e0));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x60));
            c.lq(2, bw::p(sp0, 0x60));
            c.sq(2, bw::p(sp0, 0x50));
            c.mfc1(3, 2);
            c.lqc2(4, bw::p(sp0, 0x50));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x60));
            c.lq(2, bw::p(sp0, 0x60));
            c.sq(2, bw::p(sp0, 0x40));
            c.mfc1(3, 0);
            c.lqc2(4, bw::p(sp0, 0x40));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x60));
            c.lq(2, bw::p(sp0, 0x60));
            c.sq(2, bw::p(sp0, 0x30));
            // pos += up*f1 - that.
            c.lqc2(3, bw::p(sp0, 0x20));
            c.lqc2(4, bw::p(sp0, 0x30));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0x60));
            c.lq(2, bw::p(sp0, 0x60));
            c.sq(2, bw::p(sp0, 0x10));
            c.lqc2(3, sp0);
            c.lqc2(4, bw::p(sp0, 0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp0);
            // side = normal x up (x cleared), flipped when SIDE < 0.
            c.g[V0][0] = c.lw(fx);
            c.lqc2(4, bw::p(fx, NORMAL));
            c.lqc2(5, bw::p(r32(c, V0), 0x370));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, bw::p(sp0, 0x20));
            c.lwc1(0, bw::p(fx, SIDE));
            c.lq(2, bw::p(sp0, 0x20));
            c.c_lt_s(0, 5);
            let flip = c.fcc();
            c.sq(2, bw::p(sp0, 0x10));
            if flip {
                c.cycles += 10;
                bw::fconst(c, 0, 0xbf80_0000); // -1
                c.mfc1(3, 0);
                c.lqc2(4, bw::p(sp0, 0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, bw::p(sp0, 0x30));
                c.lq(2, bw::p(sp0, 0x30));
                c.sq(2, bw::p(sp0, 0x10));
                c.sq(2, bw::p(sp0, 0x20));
            }
            // The edge normal, negated when the rider's +0x320 is set.
            c.cycles += 4;
            c.g[V0][0] = c.lw(fx);
            c.g[V1][0] = c.lw(bw::p(r32(c, V0), 0x320));
            if c.g[V1][0] == 0 {
                c.lq(2, bw::p(fx, NORMAL));
            } else {
                c.cycles += 8;
                bw::fconst(c, 0, 0xbf80_0000); // -1
                c.mfc1(3, 0);
                c.lqc2(4, bw::p(fx, NORMAL));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, bw::p(sp0, 0x30));
                c.lq(2, bw::p(sp0, 0x30));
            }
            // vel = (-normal * speed - velocity) * 0.3; then the trail point.
            c.cycles += 134;
            c.sq(2, bw::p(sp0, 0x20));
            bw::fconst(c, 1, 0xbf80_0000); // -1
            c.g[S0][0] = sx(bw::p(sp0, 0x60));
            c.mfc1(2, 1);
            c.lqc2(4, bw::p(sp0, 0x20));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x50));
            c.lwc1(2, bw::gp(c, -0x3a78)); // 0.3
            c.lq(2, bw::p(sp0, 0x50));
            c.lwc1(0, bw::p(fx, SPEED));
            c.sq(2, bw::p(sp0, 0x40));
            c.mfc1(3, 0);
            c.lqc2(4, bw::p(sp0, 0x40));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x50));
            c.lq(2, bw::p(sp0, 0x50));
            c.g[V1][0] = c.lw(fx);
            c.sq(2, bw::p(sp0, 0x30));
            c.mfc1(4, 1);
            c.lqc2(4, bw::p(r32(c, V1), 0x1e0));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x50));
            c.lq(2, bw::p(sp0, 0x50));
            c.sq(2, bw::p(sp0, 0x40));
            c.lqc2(3, bw::p(sp0, 0x30));
            c.lqc2(4, bw::p(sp0, 0x40));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0x70));
            c.lq(2, bw::p(sp0, 0x70));
            c.sq(2, bw::p(sp0, 0x60));
            c.mfc1(3, 2);
            c.lqc2(4, bw::p(sp0, 0x60));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x70));
            c.lq(2, bw::p(sp0, 0x70));
            // The trail point through the rider's vtable (+0x6c0 slot 0x10).
            c.g[A0][0] = c.lw(fx);
            c.sq(2, bw::p(sp0, 0x50));
            c.g[A1][0] = sx(r32(c, A0).wrapping_add(0x6c0));
            c.g[V1][0] = c.lw(bw::p(r32(c, A0), 0x6c0));
            c.g[A0][0] = c.lh(bw::p(r32(c, V1), 0x10));
            c.g[V0][0] = c.lw(bw::p(r32(c, V1), 0x14));
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002e_1de4)?;
            // Spawn offset = trail * curve[0x730] + side * speed * f0
            //   + up * curve[0x734] + up * curve[0x738] * UP_SCALE + vel.
            c.g[V1][0] = c.lw(bw::p(fx, P2C));
            c.g[A0][0] = c.g[S2][0];
            c.mtc1(0, 1); // f1 = 0
            c.g[A2][0] = c.g[S0][0];
            c.lwc1(0, bw::p(r32(c, V1), 0x730));
            c.g[A3][0] = sx(bw::p(fx, COL));
            c.mfc1(3, 0);
            c.lqc2(4, r32(c, V0));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0xb0));
            c.lwc1(12, bw::gp(c, -0x3a74)); // 1/60
            c.lq(2, bw::p(sp0, 0xb0));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(0, bw::p(fx, SPEED));
            c.g[T0][0] = 1;
            c.sq(2, bw::p(sp0, 0xa0));
            c.mfc1(3, 0);
            c.lqc2(4, bw::p(sp0, 0x10));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0xd0));
            c.lq(2, bw::p(sp0, 0xd0));
            c.sq(2, bw::p(sp0, 0xc0));
            c.mfc1(3, 1);
            c.lqc2(4, bw::p(sp0, 0xc0));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0xd0));
            c.lq(2, bw::p(sp0, 0xd0));
            c.sq(2, bw::p(sp0, 0xb0));
            c.lqc2(3, bw::p(sp0, 0xa0));
            c.lqc2(4, bw::p(sp0, 0xb0));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0xd0));
            c.lq(2, bw::p(sp0, 0xd0));
            c.g[T1][0] = c.lw(bw::p(fx, P2C));
            c.sq(2, bw::p(sp0, 0x90));
            c.lwc1(0, bw::p(r32(c, T1), 0x734));
            c.g[V1][0] = c.lw(fx);
            c.mfc1(10, 0);
            c.lqc2(4, bw::p(r32(c, V1), 0x370));
            c.qmtc2(10, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0xe0));
            c.lq(2, bw::p(sp0, 0xe0));
            c.sq(2, bw::p(sp0, 0xd0));
            c.lqc2(3, bw::p(sp0, 0x90));
            c.lqc2(4, bw::p(sp0, 0xd0));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0xe0));
            c.lq(2, bw::p(sp0, 0xe0));
            c.g[T1][0] = c.lw(bw::p(fx, P2C));
            c.sq(2, bw::p(sp0, 0x80));
            c.lwc1(0, bw::p(r32(c, T1), 0x738));
            c.g[V1][0] = c.lw(fx);
            c.mfc1(10, 0);
            c.lqc2(4, bw::p(r32(c, V1), 0x370));
            c.qmtc2(10, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x100));
            c.lq(2, bw::p(sp0, 0x100));
            c.lwc1(0, bw::p(fx, UP_SCALE));
            c.sq(2, bw::p(sp0, 0xf0));
            c.mfc1(3, 0);
            c.lqc2(4, bw::p(sp0, 0xf0));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x100));
            c.lq(2, bw::p(sp0, 0x100));
            c.sq(2, bw::p(sp0, 0xe0));
            c.lqc2(3, bw::p(sp0, 0x80));
            c.lqc2(4, bw::p(sp0, 0xe0));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0x100));
            c.lq(2, bw::p(sp0, 0x100));
            c.sq(2, bw::p(sp0, 0x70));
            c.lqc2(3, bw::p(sp0, 0x70));
            c.lqc2(4, bw::p(sp0, 0x50));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0x100));
            c.lq(2, bw::p(sp0, 0x100));
            c.sq(2, bw::p(sp0, 0x60));
            // DynamicEmitter_SpawnAt(slot 7, &pos, &vel, &colour, 1/60, 1).
            call_ptr(c, 0x0037_17c0, 0x002e_1f34)?;
            frame.restore(c, S0, 0x140);
            c.cycles += 5;
            frame.restore(c, S1, 0x130);
            frame.restore(c, S2, 0x120);
            frame.restore_ra(c, 0x110);
            frame.close(c);
            return Ok(());
        }
        // Idle: keep slot 7 alive at the spray position.
        c.cycles += 8;
        c.lwc1(12, bw::gp(c, -0x3a70)); // 1/60
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(bw::p(r32(c, S1), POS));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x0037_17c0, 0x002e_1f58)?;
        frame.restore(c, S0, 0x140);
        c.cycles += 5;
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    });
}

pub(crate) mod rctx {

    /// Top of the 0x14-byte render-state stack.
    pub const STATE_TOP: u32 = 0xe84;

    /// The current texture handle (copied into the pushed state +0x10).
    pub const TEX: u32 = 0xf70;

    /// The vptr; slot +0x118 returns the view matrix.
    pub const VPTR: u32 = 0x10d8;

    pub const GET_VIEW: u32 = 0x118;
}

pub(crate) mod spray8 {

    /// The rider (its `+0x6c0` sub-object's virtual slot `+0x10` returns
    /// the velocity vector).
    pub const OBJ: u32 = 0x00;

    /// Spray level (fades in and out).
    pub const LEVEL: u32 = 0x10;

    /// Base of the dynamic emitter array (0x210 bytes per slot).
    pub const P28: u32 = 0x28;

    /// Curve table (`+0x818/0x81c/0x820`: velocity, direction and
    /// speed-scaled direction weights).
    pub const P2C: u32 = 0x2c;

    /// Throw direction.
    pub const DIR: u32 = 0x30;

    /// Spawn colour (its w becomes the alpha).
    pub const COL: u32 = 0x90;

    /// Spray position.
    pub const POS: u32 = 0xa0;

    /// Non-zero while the effect is driven.
    pub const ACTIVE: u32 = 0xb0;

    /// Speed (scales the direction term; below a limit the spray stops).
    pub const SPEED: u32 = 0xbc;

    /// Configuration (`+0x5c`: the level's target).
    pub const CFG: u32 = 0xd0;

    /// The emitter slot this update drives (8 * 0x210).
    pub const SLOT: u32 = 0x1080;
}

pub(crate) mod snowfall {
    use super::ee;
    use super::Callee;

    /// `gp-0x848`: the app (`+0x84` game module, `+0x84` its weather
    /// object, whose `+0x10` is the number of splash managers / views).
    pub const GP_APP: u32 = 0x848;

    /// gp+0x164c: non-zero when the weather comes from the override block
    /// (gp+0x1650..0x166c and the colours at 0x5059b8 / 0x5059c8) instead of
    /// the world painter.
    pub const GP_WEATHER_OVERRIDE: u32 = 0x164c;

    pub const GP_OVERRIDE_F: [u32; 8] = [
        0x1650, 0x1654, 0x1668, 0x166c, 0x1658, 0x165c, 0x1660, 0x1664,
    ];

    pub const OVERRIDE_C0: u32 = 0x0050_59b8;

    pub const OVERRIDE_C1: u32 = 0x0050_59c8;

    /// One weather record per view on the stack (0x40 bytes: f[8], c0, c1).
    pub const REC_SIZE: u32 = 0x40;

    /// `SnowFall`: twelve `SnowBox*` from +0x10 (a box's first word is its
    /// view index).
    pub const BOXES: u32 = 0x10;

    /// `WorldPainter_GetWeatherNN(kind)`, in record order.
    pub const GETTERS: [Callee; 16] = [
        Callee {
            addr: 0x002e_e448,
            f: ee::s09::WorldPainter_GetWeather08,
        },
        Callee {
            addr: 0x002e_e508,
            f: ee::s09::WorldPainter_GetWeather18,
        },
        Callee {
            addr: 0x002e_e600,
            f: ee::s09::WorldPainter_GetWeather48,
        },
        Callee {
            addr: 0x002e_e660,
            f: ee::s09::WorldPainter_GetWeather50,
        },
        Callee {
            addr: 0x002e_e4c0,
            f: ee::s09::WorldPainter_GetWeather10,
        },
        Callee {
            addr: 0x002e_e570,
            f: ee::s09::WorldPainter_GetWeather20,
        },
        Callee {
            addr: 0x002e_e5b8,
            f: ee::s09::WorldPainter_GetWeather38,
        },
        Callee {
            addr: 0x002e_e6a8,
            f: ee::s09::WorldPainter_GetWeather40,
        },
        Callee {
            addr: 0x002e_e780,
            f: ee::s09::WorldPainter_GetWeather60,
        },
        Callee {
            addr: 0x002e_e7c8,
            f: ee::s09::WorldPainter_GetWeather68,
        },
        Callee {
            addr: 0x002e_e810,
            f: ee::s09::WorldPainter_GetWeather70,
        },
        Callee {
            addr: 0x002e_e858,
            f: ee::s09::WorldPainter_GetWeather78,
        },
        Callee {
            addr: 0x002e_e8a0,
            f: ee::s09::WorldPainter_GetWeather80,
        },
        Callee {
            addr: 0x002e_e8e8,
            f: ee::s09::WorldPainter_GetWeather88,
        },
        Callee {
            addr: 0x002e_e930,
            f: ee::s09::WorldPainter_GetWeather90,
        },
        Callee {
            addr: 0x002e_e978,
            f: ee::s09::WorldPainter_GetWeather98,
        },
    ];

    /// `GameModule_GetSplashManager(gm, i)` (cross-shard).
    pub const GET_SPLASH_MANAGER: u32 = 0x0023_06a8;

    pub const SET_SNOW_AMOUNT: Callee = Callee {
        addr: 0x002f_4330,
        f: ee::s09::cSplashManager_SetSnowAmount,
    };

    pub const SNOW_BOX_UPDATE: Callee = Callee {
        addr: 0x002e_4f50,
        f: ee::s09::SnowBox_Update,
    };
}

/// Copy a 0x14-byte render state (two unaligned doublewords through GPRs
/// `ra`/`rb`, then a word through `rw`) from `src` to `dst`, as the
/// original's `ldl/ldr/sdl/sdr` sequence does.
pub(crate) fn copy_render_state_2(
    c: &mut Cpu,
    src: u32,
    dst: u32,
    ra: usize,
    rb: usize,
    rw: usize,
) {
    c.ldl(ra, src.wrapping_add(0x7));
    c.ldr(ra, src);
    c.ldl(rb, src.wrapping_add(0xf));
    c.ldr(rb, src.wrapping_add(0x8));
    c.g[rw][0] = Addr(src.wrapping_add(0x10)).lw(c);
    c.sdl(ra, dst.wrapping_add(0x7));
    c.sdr(ra, dst);
    c.sdl(rb, dst.wrapping_add(0xf));
    c.sdr(rb, dst.wrapping_add(0x8));
    Addr(dst.wrapping_add(0x10)).sw(c, c.g[rw][0]);
}

/// `void SnowFall_Render(SnowFall *this)` (0x2e6008): unless rendering is
/// off (`gp+0x1434`), take the current rider's movement since the last
/// frame (position at rider+0x20 minus `this->last[k]` at +0x40 + 16k),
/// push a render state (the template at 0x501420; alpha/blend bits, the
/// fog bit from `gp+0x1670`, the current texture), fetch the view matrix
/// (renderer vtable +0x118) with its translation replaced by 0x4ff130, and
/// for each of the 12 snow boxes (+0x10) owned by that rider wrap it by the
/// movement (`SnowBox_WrapOffset`) and draw it (`SnowBox_Render`) tinted by
/// the rider's colour * 0.6 - 0.5. Pops the render state.
pub fn snow_fall_render(c: &mut Cpu) {
    const WRAP: Callee = Callee {
        addr: 0x002e_5430,
        f: ee::s09::f_002e5430,
    };
    const RENDER: Callee = Callee {
        addr: 0x002e_55d8,
        f: ee::s09::f_002e55d8,
    };
    /// The render-state template pushed for the snow.
    const STATE_TEMPLATE: u32 = 0x0050_1420;
    /// Translation row put into the view matrix copy.
    const VIEW_ORIGIN: u32 = 0x004f_f130;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xf0);
        c.g[V0][0] = Addr(gp_at(c, 0x1434)).lw(c);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 22, 0xe0);
        frame.save_fpr(c, 21, 0xd8);
        frame.save_fpr(c, 20, 0xd0);
        let disabled = c.g[V0][0] != 0;
        if !disabled {
            let this = r32(c, S0);
            // The current rider k and its movement since the last frame.
            c.cycles += 64;
            c.g[V0][0] = Addr(gp_at(c, -0x848)).lw(c);
            c.g[A3][0] = 0x50_0000;
            c.g[A2][0] = 0xffff_ffff_ffbf_ffff;
            c.g[T0][0] = 0x40_0000;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[T1][0] = 0xffff_ffff_ffff_fc1f;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[S3][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
            let k = r32(c, S3);
            c.g[V1][0] = sx(k << 2);
            c.g[A0][0] = sx(k << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(k << 2));
            let last = this.wrapping_add(k << 4);
            c.g[A1][0] = sx(last);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = c.g[A1][0];
            let rider = r32(c, V1);
            let (t0, t10, t20) = (frame.at(c, 0), frame.at(c, 0x10), frame.at(c, 0x20));
            c.lq(2, rider.wrapping_add(0x20));
            c.sq(2, t0);
            c.lqc2(3, t0);
            c.lqc2(4, last.wrapping_add(0x40));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, t20);
            c.lq(2, t20);
            c.lq(3, t0);
            c.sq(2, t10);
            c.g[S1][0] = Addr(gp_at(c, 0x2a90)).lw(c);
            let rc = r32(c, S1);
            c.sq(3, last.wrapping_add(0x40));

            // Push the render state: duplicate the top, then load the template.
            let top = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c) as u32;
            c.g[V0][0] = sx(top);
            copy_render_state_2(c, top, top.wrapping_add(0x14), V1, A0, A1);
            c.g[V1][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let top = (c.g[V1][0] as u32).wrapping_add(0x14);
            c.g[V0][0] = sx(top);
            Addr(rc.wrapping_add(rctx::STATE_TOP)).sw(c, c.g[V0][0]);
            c.g[T2][0] = sx(STATE_TEMPLATE);
            let prev = r32(c, V1);
            copy_render_state_2(c, STATE_TEMPLATE, prev.wrapping_add(0x14), V0, A0, A1);
            // Word 1: bit 22 set; word 2: bits 5..9 = 7.
            c.g[A0][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let st = r32(c, A0);
            c.g[V0][0] = (Addr(st.wrapping_add(4)).lw(c) & c.g[A2][0]) | c.g[T0][0];
            Addr(st.wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A1][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let st = r32(c, A1);
            c.g[V0][0] = (Addr(st.wrapping_add(8)).lw(c) & c.g[T1][0]) | 0xe0;
            Addr(st.wrapping_add(8)).sw(c, c.g[V0][0]);
            // Word 1 bits 2..6: 7 with the fog flag, else 5.
            let fog = Addr(gp_at(c, 0x1670)).lw(c) != 0;
            c.g[V1][0] = 0xffff_ffff_ffff_ff83;
            c.g[A0][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let st = r32(c, A0);
            c.g[V0][0] = Addr(st.wrapping_add(4)).lw(c) & c.g[V1][0];
            if fog {
                c.cycles += 5;
                c.g[V0][0] |= 0x1c;
            } else {
                c.cycles += 4;
                c.g[V0][0] |= 0x14;
            }
            // Word 1 bits 0..1 = 2; word 2 bits 10..28 cleared; texture.
            c.cycles += 58;
            Addr(st.wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A1][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let st = r32(c, A1);
            c.g[A0][0] = 0xffff_ffff_ffff_fffc;
            c.g[V1][0] = 0xffff_ffff_e000_03ff;
            c.g[V0][0] = (Addr(st.wrapping_add(4)).lw(c) & c.g[A0][0]) | 2;
            Addr(st.wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A0][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let st = r32(c, A0);
            c.g[V0][0] = Addr(st.wrapping_add(8)).lw(c) & c.g[V1][0];
            Addr(st.wrapping_add(8)).sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            c.g[A1][0] = Addr(rc.wrapping_add(rctx::TEX)).lw(c);
            Addr((c.g[V0][0] as u32).wrapping_add(0x10)).sh(c, c.g[A1][0]);

            // The view matrix (renderer vtable +0x118) into sp+0x30.
            c.g[V1][0] = Addr(rc.wrapping_add(rctx::VPTR)).lw(c);
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt.wrapping_add(rctx::GET_VIEW)).lh(c);
            c.g[V0][0] = Addr(vt.wrapping_add(rctx::GET_VIEW + 4)).lw(c);
            c.g[A0][0] = sx(rc.wrapping_add(c.g[A0][0] as u32));
            let method = r32(c, V0);
            call_ptr(c, method, 0x002e_61b4)?;
            let m = frame.at(c, 0x30);
            c.g[V1][0] = sx(m);
            let src = r32(c, V0);
            for i in 0..4u32 {
                c.lqc2(1 + i as usize, src.wrapping_add(0x10 * i));
            }
            for i in 0..4u32 {
                c.sqc2(1 + i as usize, m.wrapping_add(0x10 * i));
            }
            c.g[V0][0] = 0x50_0000;
            c.g[S4][0] = c.g[V1][0];
            c.g[V0][0] = sx(VIEW_ORIGIN);
            c.g[A1][0] = Addr(gp_at(c, -0x848)).lw(c);
            c.lq(3, VIEW_ORIGIN);
            c.g[S0][0] = sx(this.wrapping_add(0x10));
            c.lwc1(3, gp_at(c, -0x3a04));
            c.g[S2][0] = 0xb;
            let t60 = frame.at(c, 0x60);
            c.sq(3, t60);
            c.g[AT][0] = 0xffff_ffff_bf00_0000;
            c.mtc1(1, 4);
            // The tint: rider colour (+0x80) * 0.6 - 0.5 into f22/f21/f20.
            c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
            c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x80));
            c.sq(2, t20);
            c.lwc1(0, t20);
            c.lwc1(1, t20 + 4);
            c.lwc1(2, t20 + 8);
            c.mul_s(0, 0, 3);
            c.mul_s(1, 1, 3);
            c.mul_s(2, 2, 3);
            c.add_s(22, 0, 4);
            c.add_s(21, 1, 4);
            c.add_s(20, 2, 4);

            // The 12 snow boxes.
            loop {
                c.cycles += 3;
                let slot = r32(c, S0);
                c.g[A0][0] = Addr(slot).lw(c);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                    if c.g[V0][0] == c.g[S3][0] {
                        c.cycles += 9;
                        c.g[A1][0] = sx(frame.at(c, 0x10));
                        WRAP.call(c, 0x002e_6268)?;
                        c.g[A0][0] = Addr(slot).lw(c);
                        c.g[A1][0] = c.g[S4][0];
                        c.mov_s(12, 22);
                        c.mov_s(13, 21);
                        c.mov_s(14, 20);
                        RENDER.call(c, 0x002e_6280)?;
                    }
                }
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                c.cycles += 2;
                let more = (c.g[S2][0] as i64) >= 0;
                c.g[S0][0] = sx(slot.wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_6248);
                }
            }
            // Pop the render state.
            c.cycles += 3;
            c.g[V0][0] =
                sx((Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c) as u32).wrapping_sub(0x14));
            Addr(rc.wrapping_add(rctx::STATE_TOP)).sw(c, c.g[V0][0]);
        }
        c.cycles += 11;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 22, 0xe0);
        frame.restore_fpr(c, 21, 0xd8);
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LensFX_ProjectSource (0x2e2b00)
// ---------------------------------------------------------------------------

/// The camera / view object (`*(gp + 0x2a90)`).
pub(crate) const LENS_CAMERA_GP: u32 = 0x2a90;

/// The camera's embedded projector (vtable pointer at +0x10d8).
pub(crate) const CAM_PROJECTOR_VT: u32 = 0x10d8;

/// Projector slot: `IsOccluded(point)` (non-zero hides the flare).
pub(crate) const VT_OCCLUDED: u32 = 0x2d8;

/// Projector slot: `Project(out, point)` (screen x/y at +0/+4, depth +8).
pub(crate) const VT_PROJECT: u32 = 0x2e0;

/// Lens flare: the light source it follows.
pub(crate) const LENS_SOURCE: u32 = 0x20;

/// Lens flare: visible / on-screen flags (halfwords).
pub(crate) const LENS_VISIBLE: u32 = 0x2c;

pub(crate) const LENS_ON_SCREEN: u32 = 0x2e;

/// Light source: world position (x, y, z) and its kind bits (`& 0x70`).
pub(crate) const SRC_POS: u32 = 0x1c;

pub(crate) const SRC_KIND: u32 = 0xc;

/// `void LensFX_ProjectSource(LensFX* lens, View* view)` (0x2e2b00): project
/// the flare's light source onto the screen. Hidden (occluded) sources clear
/// both flags. Visible ones in front of the camera get a sample rectangle
/// sized from their depth (clamped to 16x8 pixels, at least 1x1) centred on
/// the projected point and clipped to the view, a projected flare radius
/// (100/80/200 world units by source kind) and a horizontal screen offset
/// normalised to -1..1.
pub fn lens_fx_project_source(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue; the source position as a homogeneous point at sp+0.
        c.cycles += 27;
        let frame = Frame::open(c, 0xc0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x90);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x80);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x70);
        let lens = r32(c, S0);
        let view = r32(c, S1);
        let point = sp(c);
        let gp = c.g[GP][0] as u32;
        c.g[S2][0] = Addr(gp).off(LENS_CAMERA_GP).lw(c);
        let cam = r32(c, S2);
        c.g[V0][0] = Addr(lens).off(LENS_SOURCE).lw(c);
        let src = r32(c, V0);
        c.lwc1(0, src + SRC_POS);
        c.swc1(0, point);
        c.lwc1(1, src + SRC_POS + 4);
        c.swc1(1, point + 4);
        c.lwc1(0, src + SRC_POS + 8);
        c.swc1(2, point + 0xc);
        c.swc1(0, point + 8);

        // Occluded?
        c.g[V1][0] = Addr(cam).off(CAM_PROJECTOR_VT).lw(c);
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, VT_OCCLUDED);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(c.g[S2][0]);
        call_ptr(c, slot.func as u32, 0x002e_2b64)?;
        c.g[S3][0] = 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            Addr(lens).off(LENS_ON_SCREEN).sh(c, 0);
            Addr(lens).off(LENS_VISIBLE).sh(c, 0);
            c.cycles += 1;
            return epilogue(c, frame);
        }

        // Project it (screen point at sp+0x10).
        c.cycles += 16;
        c.g[A0][0] = sx(point.wrapping_add(0x10));
        Addr(lens).off(LENS_VISIBLE).sh(c, 1);
        c.g[A2][0] = c.g[SP][0];
        c.g[V0][0] = Addr(cam).off(CAM_PROJECTOR_VT).lw(c);
        let vt = c.g[V0][0];
        let slot = VSlot::read(c, vt, VT_PROJECT);
        c.g[A1][0] = slot.delta;
        c.g[V1][0] = slot.func;
        c.g[A1][0] = slot.this(c.g[S2][0]);
        call_ptr(c, slot.func as u32, 0x002e_2b98)?;
        let point = sp(c);
        let gp = c.g[GP][0] as u32;
        c.lwc1(2, point + 0x18);
        c.lwc1(0, gp.wrapping_sub(0x3a40));
        c.lwc1(1, gp.wrapping_sub(0x3a3c));
        c.mul_s(2, 2, 0);
        c.c_le_s(2, 1);
        if c.fcc() {
            // Behind the near plane: visible but not on screen.
            return epilogue(c, frame);
        }

        // Sample rectangle size from the depth: w = clamp(800 d, 1, 16),
        // h = clamp(400 d, 1, 8).
        c.cycles += 13;
        c.g[AT][0] = 0x4448_0000;
        c.mtc1(AT, 1);
        c.g[V1][0] = 0x10;
        c.g[AT][0] = 0x43c8_0000;
        c.mtc1(AT, 0);
        c.mul_s(1, 2, 1);
        c.mul_s(0, 2, 0);
        c.cvt_w_s(2, 1);
        c.mfc1(V0, 2);
        c.cvt_w_s(1, 0);
        c.mfc1(A1, 1);
        Addr(lens).off(LENS_ON_SCREEN).sh(c, 1);
        if (c.g[V0][0] as i64) <= 0 {
            c.cycles += 1;
            c.g[A0][0] = 1;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = c.g[V1][0];
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 8;
        if (c.g[A1][0] as i64) <= 0 {
            c.cycles += 1;
            c.g[V1][0] = 1;
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[V1][0] = c.g[A1][0];
            }
        }

        // The rectangle around the projected point, clipped to the view.
        c.cycles += 39;
        c.g[A1][0] = c.g[V1][0];
        c.g[V0][0] = sx(((c.g[A0][0] as u32) as i32 >> 1) as u32);
        c.g[V1][0] = sx(((c.g[A1][0] as u32) as i32 >> 1) as u32);
        c.lwc1(4, point + 0x14);
        c.mtc1(V1, 1);
        c.cvt_s_w(1, 1);
        c.lwc1(3, point + 0x10);
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.g[AT][0] = 0x4080_0000;
        c.mtc1(AT, 2);
        c.g[AT][0] = 0x4100_0000;
        c.mtc1(AT, 7);
        c.mtc1(A0, 5);
        c.cvt_s_w(5, 5);
        c.sub_s(1, 4, 1);
        c.g[AT][0] = 0x4180_0000;
        c.mtc1(AT, 6);
        c.sub_s(0, 3, 0);
        c.swc1(7, lens + 0x1c);
        c.sub_s(4, 4, 2);
        c.swc1(5, lens + 0x8);
        c.sub_s(3, 3, 7);
        c.swc1(1, lens + 0x4);
        c.mtc1(A1, 2);
        c.cvt_s_w(2, 2);
        c.swc1(0, lens);
        c.swc1(4, lens + 0x14);
        c.swc1(3, lens + 0x10);
        c.swc1(2, lens + 0xc);
        c.swc1(6, lens + 0x18);
        c.mtc1(0, 2);
        c.lwc1(0, view);
        c.lwc1(1, view + 0x8);
        c.c_le_s(2, 3);
        c.mov_s(5, 2);
        c.add_s(0, 0, 1);
        c.sub_s(0, 0, 6);
        if c.fcc() {
            c.cycles += 1;
            c.min_s(5, 3, 0);
        }
        c.cycles += 8;
        c.swc1(5, lens + 0x10);
        c.c_le_s(2, 4);
        c.mov_s(3, 2);
        c.lwc1(0, view + 0x4);
        c.lwc1(1, view + 0xc);
        c.add_s(0, 0, 1);
        c.sub_s(0, 0, 7);
        if c.fcc() {
            c.cycles += 1;
            c.min_s(3, 4, 0);
        }

        // Flare radius by source kind (f2; 0 for the other kinds).
        c.cycles += 7;
        c.swc1(3, lens + 0x14);
        c.g[A0][0] = 0x20;
        c.g[V0][0] = Addr(lens).off(LENS_SOURCE).lw(c);
        let src = r32(c, V0);
        c.g[V1][0] = Addr(src).off(SRC_KIND).lw(c);
        c.g[V1][0] &= 0x70;
        let kind = c.g[V1][0];
        c.g[V0][0] = (kind < 0x21) as u64;
        let radius_bits = if kind == 0x20 {
            c.cycles += 4;
            Some(0x42a0_0000)
        } else if kind < 0x21 {
            c.cycles += 2;
            c.g[V0][0] = 0x10;
            c.cycles += 2;
            if kind == 0x10 {
                c.cycles += 4;
                Some(0x42c8_0000)
            } else {
                c.cycles += 2;
                None
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x10;
            c.cycles += 3;
            c.g[V0][0] = 0x40;
            c.cycles += 2;
            if kind == 0x40 {
                Some(0x4348_0000)
            } else {
                None
            }
        };
        if let Some(bits) = radius_bits {
            c.g[AT][0] = bits;
            c.mtc1(AT, 2);
        }
        c.lwc1(0, view + 0x18);
        if radius_bits.is_some() || kind < 0x21 {
            c.cycles += 1;
        }
        c.g[A0][0] = sx(point.wrapping_add(0x40));

        // Project the point offset by the radius (out at sp+0x40).
        c.cycles += 41;
        c.lwc1(1, view + 0x28);
        c.g[A2][0] = sx(point.wrapping_add(0x30));
        c.lwc1(3, view + 0x38);
        c.mul_s(4, 0, 2);
        c.mul_s(5, 1, 2);
        c.swc1(0, point + 0x20);
        c.mul_s(2, 3, 2);
        c.swc1(1, point + 0x24);
        c.swc1(4, point + 0x40);
        c.swc1(3, point + 0x28);
        c.swc1(2, point + 0x48);
        c.swc1(4, point + 0x50);
        c.swc1(2, point + 0x58);
        c.swc1(5, point + 0x44);
        c.swc1(5, point + 0x54);
        Addr(point).off(0x4c).sw(c, 0);
        c.lqc2(3, point);
        c.lqc2(4, point + 0x40);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, point + 0x60);
        c.lq(2, point + 0x60);
        c.sq(2, point + 0x30);
        c.g[V1][0] = Addr(cam).off(CAM_PROJECTOR_VT).lw(c);
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, VT_PROJECT);
        c.g[A1][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A1][0] = slot.this(c.g[S2][0]);
        call_ptr(c, slot.func as u32, 0x002e_2dc0)?;
        let point = sp(c);
        let gp = c.g[GP][0] as u32;
        c.lwc1(0, point + 0x48);
        c.lwc1(3, point + 0x10);
        c.swc1(0, lens + 0x24);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 4);
        c.lwc1(0, view);
        c.lwc1(2, view + 0x8);
        c.sub_s(0, 3, 0);
        c.lwc1(1, gp.wrapping_sub(0x3a38));
        c.add_s(0, 0, 0);
        c.div_s(0, 0, 2);
        c.sub_s(0, 0, 4);
        c.mul_s(1, 0, 1);
        c.swc1(1, lens + 0x28);
        c.cycles += 1;
        epilogue(c, frame)
    });

    fn epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 6;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// VU0 macro sequences shared by the ports below (the original's exact ops)
// ---------------------------------------------------------------------------

/// `mfc1 gpr, f; lqc2 vf4, src; qmtc2 gpr, vf3; vmul.xyzw vf5, vf4, vf3x;
/// sqc2 vf5, dst`: `dst = src * f`.
pub(crate) fn vscale_2(c: &mut Cpu, gpr: usize, f: usize, src: u32, dst: u32) {
    c.mfc1(gpr, f);
    c.lqc2(4, src);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, dst);
}

/// The xyz length of the vector at `src` (`vf3`), left in `gpr` by
/// `cfc2 gpr, Q` after `vsqrt`.
pub(crate) fn vlen(c: &mut Cpu, gpr: usize, src: u32) {
    c.lqc2(3, src);
    vlen_vf3(c, gpr);
}

/// [`vlen`] of the vector already in vf3.
pub(crate) fn vlen_vf3(c: &mut Cpu, gpr: usize) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(gpr, 22);
}

/// `lqc2 vf3, a; lqc2 vf4, b; v{add,sub}.xyzw vf5, vf3, vf4; sqc2 vf5, dst`.
pub(crate) fn vadd_3(c: &mut Cpu, a: u32, b: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, dst);
}

pub(crate) fn vsub_2(c: &mut Cpu, a: u32, b: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, dst);
}

/// `lq gpr, src; sq gpr, dst` (the compiler's quadword copies through a GPR).
pub(crate) fn qcopy(c: &mut Cpu, gpr: usize, src: u32, dst: u32) {
    c.lq(gpr, src);
    c.sq(gpr, dst);
}

// ---------------------------------------------------------------------------
// cBoardWakeFX_BuildTrail (0x2ddd30)
// ---------------------------------------------------------------------------

/// Board wake: the rider, ring bookkeeping and the trail shape.
pub(crate) const WAKE_RIDER: u32 = 0x0;

pub(crate) const WAKE_HEAD: u32 = 0x8;

pub(crate) const WAKE_ELEMS: u32 = 0xc;

/// `rings[n]` (pointers to 0x30-byte segments, one per head slot).
pub(crate) const WAKE_RINGS: u32 = 0x10;

pub(crate) const WAKE_PHASE: u32 = 0x38;

pub(crate) const WAKE_SPREAD: u32 = 0x60;

pub(crate) const WAKE_CURVE: u32 = 0x70;

pub(crate) const WAKE_BACK: u32 = 0x80;

pub(crate) const WAKE_WIDTH: u32 = 0x90;

pub(crate) const WAKE_ALPHA: u32 = 0x94;

pub(crate) const WAKE_POINTS: u32 = 0xb4;

/// The rider's velocity (+0x1e0) and up vector (+0x370).
pub(crate) const RIDER_VEL: u32 = 0x1e0;

pub(crate) const RIDER_UP: u32 = 0x370;

/// A trail element (0x70 bytes): points from +0, the direction at +0x50,
/// the width at +0x60.
pub(crate) const ELEM_SIZE: u32 = 0x70;

pub(crate) const ELEM_DIR: u32 = 0x50;

pub(crate) const ELEM_WIDTH: u32 = 0x60;

pub(crate) const ELEM_CLEAR: u32 = 0x64;

/// Ring segment (0x30 bytes): position at +0x20, alpha at +0x1c.
pub(crate) const SEG_SIZE: u32 = 0x30;

pub(crate) const SEG_POS: u32 = 0x20;

pub(crate) const SEG_ALPHA: u32 = 0x1c;

/// `void cBoardWakeFX_BuildTrail(cBoardWakeFX* wake, const Vec4* pos, float
/// width)` (0x2ddd30): fill the head trail element. The trail width is
/// modulated by 1D fractal noise of the phase; the direction is the wake's
/// back vector scaled by the rider's speed, minus the reversed velocity,
/// times 0.85; the points follow `w * (0.75 t^2 curve + t spread)` with the
/// last one pulled in by 0.9. The head slot of every ring gets the position
/// `pos - 5 up` and the alpha `(int)(alpha * 128)`.
pub fn board_wake_fx_build_trail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 83;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xb0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x80);
        let wake = r32(c, S0);
        c.mov_s(20, 12);
        c.g[A0][0] = 3;
        c.lwc1(12, wake + WAKE_PHASE);
        Callee {
            addr: 0x002d_1928,
            f: ee::s09::f_002d1928,
        }
        .call(c, 0x002d_dd5c)?;
        let sp0 = sp(c);
        let gp = c.g[GP][0] as u32;

        // The head element; clear it and its first point.
        c.g[V0][0] = Addr(wake).off(WAKE_HEAD).lw(c);
        c.g[V1][0] = ELEM_SIZE as u64;
        c.g[A1][0] = Addr(wake).off(WAKE_ELEMS).lw(c);
        c.g[A0][0] = 0x50_0000;
        c.mult(0, V0, V0, V1);
        c.lwc1(3, wake + WAKE_WIDTH);
        c.g[AT][0] = 0x3e80_0000;
        c.mtc1(AT, 1);
        c.g[A0][0] = sx(0x004f_f120);
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(AT, 2);
        c.g[A2][0] = 0;
        c.mul_s(0, 0, 1);
        c.lwc1(4, gp.wrapping_sub(0x3be4));
        c.g[T1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        let elem = r32(c, T1);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        Addr(elem).off(ELEM_CLEAR).sw(c, 0);
        c.add_s(0, 0, 1);
        qcopy(c, V0, 0x004f_f120, elem);
        // w = width * (noise * 0.25 + 1).
        c.mul_s(3, 3, 0);

        // dir = (-back * |vel| - (-vel)) * k.
        vscale_2(c, A0, 2, wake + WAKE_BACK, sp0 + 0x20);
        c.lq(V1, sp0 + 0x20);
        c.g[V0][0] = Addr(wake).off(WAKE_RIDER).lw(c);
        let rider = r32(c, V0);
        c.sq(V1, sp0 + 0x10);
        vlen(c, A0, rider + RIDER_VEL);
        c.mtc1(A0, 0);
        vscale_2(c, V1, 0, sp0 + 0x10, sp0 + 0x20);
        c.lq(V0, sp0 + 0x20);
        c.g[V1][0] = Addr(wake).off(WAKE_RIDER).lw(c);
        c.sq(V0, sp0);
        vscale_2(c, A0, 2, rider + RIDER_VEL, sp0 + 0x20);
        qcopy(c, V0, sp0 + 0x20, sp0 + 0x10);
        vsub_2(c, sp0, sp0 + 0x10, sp0 + 0x40);
        qcopy(c, V0, sp0 + 0x40, sp0 + 0x30);
        vscale_2(c, V1, 4, sp0 + 0x30, sp0 + 0x40);
        qcopy(c, V0, sp0 + 0x40, sp0 + 0x20);
        c.sq(V0, elem + ELEM_DIR);

        // The trail points.
        c.g[V1][0] = Addr(wake).off(WAKE_POINTS).lw(c);
        c.g[A0][0] = c.g[T1][0];
        if (c.g[V1][0] as i64) > 0 {
            c.cycles += 3;
            c.lwc1(4, gp.wrapping_sub(0x3be0));
            c.g[AT][0] = 0x3f40_0000;
            c.mtc1(AT, 2);
            loop {
                // t = i / (n - 1); a = t^2; the last t *= 0.9.
                c.cycles += 8;
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(1));
                c.mtc1(A2, 1);
                c.cvt_s_w(1, 1);
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.div_s(1, 1, 0);
                let last = c.g[A2][0] == c.g[V0][0];
                c.mul_s(0, 1, 1);
                if last {
                    c.cycles += 1;
                    c.mul_s(1, 1, 4);
                }
                // point = w * (curve * a * 0.75 + spread * t).
                c.cycles += 40;
                vscale_2(c, V0, 0, wake + WAKE_CURVE, sp0 + 0x60);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                qcopy(c, V0, sp0 + 0x60, sp0 + 0x50);
                vscale_2(c, V1, 2, sp0 + 0x50, sp0 + 0x60);
                qcopy(c, V0, sp0 + 0x60, sp0 + 0x40);
                vscale_2(c, V1, 1, wake + WAKE_SPREAD, sp0 + 0x70);
                qcopy(c, V0, sp0 + 0x70, sp0 + 0x60);
                vadd_3(c, sp0 + 0x40, sp0 + 0x60, sp0 + 0x70);
                qcopy(c, V0, sp0 + 0x70, sp0 + 0x30);
                vscale_2(c, V1, 3, sp0 + 0x30, sp0 + 0x70);
                qcopy(c, V0, sp0 + 0x70, sp0 + 0x20);
                let point = r32(c, A0);
                c.sq(V0, point);
                c.g[V1][0] = Addr(wake).off(WAKE_POINTS).lw(c);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[A0][0] = sx(point.wrapping_add(0x10));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_de88);
                }
            }
        }

        // Ring 0's head segment: position pos - 5 up, alpha * 128.
        c.cycles += 37;
        c.g[V1][0] = Addr(wake).off(WAKE_RINGS).lw(c);
        c.g[A1][0] = SEG_SIZE as u64;
        c.g[A0][0] = Addr(wake).off(WAKE_HEAD).lw(c);
        c.g[A3][0] = 1;
        c.g[V0][0] = Addr(wake).off(WAKE_RIDER).lw(c);
        let rider = r32(c, V0);
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(AT, 0);
        c.mult(0, A0, A0, A1);
        vscale_2(c, A2, 0, rider + RIDER_UP, sp0 + 0x40);
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(AT, 1);
        qcopy(c, V0, sp0 + 0x40, sp0 + 0x30);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        let pos = r32(c, S1);
        vsub_2(c, pos, sp0 + 0x30, sp0 + 0x40);
        qcopy(c, V0, sp0 + 0x40, sp0 + 0x20);
        let seg0 = r32(c, A0);
        c.sq(V0, seg0 + SEG_POS);
        c.g[V1][0] = Addr(wake).off(WAKE_HEAD).lw(c);
        c.lwc1(0, wake + WAKE_ALPHA);
        c.mult(0, V1, V1, A1);
        c.g[V0][0] = Addr(wake).off(WAKE_RINGS).lw(c);
        c.mul_s(0, 0, 1);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.cvt_w_s(1, 0);
        let seg0 = r32(c, V1);
        c.swc1(1, seg0 + SEG_ALPHA);

        // Copy it into the head slot of rings 1..n.
        c.g[V0][0] = Addr(wake).off(WAKE_POINTS).lw(c);
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[T0][0] = SEG_SIZE as u64;
            c.g[A2][0] = sx(wake + WAKE_RINGS + 4);
            loop {
                c.cycles += 21;
                c.g[V1][0] = Addr(wake).off(WAKE_HEAD).lw(c);
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.g[A0][0] = Addr(wake).off(WAKE_RINGS).lw(c);
                c.mult(0, V1, V1, T0);
                c.g[A1][0] = Addr(r32(c, A2)).lw(c);
                c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
                let (from, to) = (r32(c, A0), r32(c, V1));
                qcopy(c, V0, from + SEG_POS, to + SEG_POS);
                c.g[A0][0] = Addr(wake).off(WAKE_HEAD).lw(c);
                c.g[V0][0] = Addr(wake).off(WAKE_RINGS).lw(c);
                c.mult(0, A0, A0, T0);
                c.g[A1][0] = Addr(r32(c, A2)).lw(c);
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.g[V1][0] = Addr(r32(c, V0)).off(SEG_ALPHA).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                let alpha = c.g[V1][0];
                Addr(r32(c, A0)).off(SEG_ALPHA).sw(c, alpha);
                c.g[V0][0] = Addr(wake).off(WAKE_POINTS).lw(c);
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_dfe8);
                }
            }
        }
        c.swc1(20, elem + ELEM_WIDTH);

        c.cycles += 6;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `cDynamicSprayFX_AddRandomVelocity(fx, vec)`.
pub(crate) const SPRAY_ADD_RANDOM_VELOCITY: Callee = Callee {
    addr: 0x002d_e398,
    f: ee::s09::cDynamicSprayFX_AddRandomVelocity,
};

/// `DynamicEmitter_SpawnAt(em, pos, vel, col, spawn, dt)`.
pub(crate) const DYNAMIC_EMITTER_SPAWN_AT: u32 = 0x0037_17c0;

/// The rider sub-object's velocity getter (virtual slot `+0x10` of
/// `rider+0x6c0`), registers as at 0x2e2004 / 0x2e20a4 / 0x2e20d4.
pub(crate) fn spray8_velocity(c: &mut Cpu, fx: u32, ret: u32) -> crate::call::Flow {
    c.g[A0][0] = c.lw(fx);
    c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x6c0));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
    c.g[A1][0] = c.lh(r32(c, V1).wrapping_add(0x10));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x14));
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    call_ptr(c, target, ret)
}

/// `void cDynamicSprayFX_UpdateEmitter8(cDynamicSprayFX* fx)` (0x2e1f70):
/// a spray whose level fades in toward the configured target (`+0.01` per
/// tick) while the effect is driven and the target is at least 0.01, and
/// otherwise fades out (`-1/60` per tick, not below 0). The level drops to
/// 0 at once when, undriven, the rider falls faster than 3333 (velocity z),
/// or when the speed is below a limit. While fading out with a level left,
/// one particle spawns in slot 8 at the position (plus a random jitter),
/// alpha `min(1.25 level, 1)` (scaled down while falling), thrown along
/// the rider's velocity and the direction (weighted by the curve table and
/// the speed). Otherwise slot 8 is only kept alive at the position.
pub fn dynamic_spray_fx_update_emitter8(c: &mut Cpu) {
    use spray8::*;
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xf0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0xa0);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 22, 0xe0);
        frame.save_fpr(c, 21, 0xd8);
        frame.save_fpr(c, 20, 0xd0);
        let fx = r32(c, S0);
        let gp = r32(c, GP);
        c.g[V0][0] = c.lw(fx + P28);
        c.g[V1][0] = c.lw(fx + ACTIVE);
        c.g[S2][0] = sx(r32(c, V0).wrapping_add(SLOT));
        let mut fade_in = false;
        if c.g[V1][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = c.lw(fx + CFG);
            c.lwc1(0, gp.wrapping_sub(0x3a6c)); // 0.01
            c.lwc1(2, r32(c, V0).wrapping_add(0x5c));
            c.c_lt_s(2, 0);
            fade_in = !c.fcc();
        }
        if fade_in {
            // level = min(level + 0.01, target); s1 = 1 (no spawn).
            c.lwc1(0, fx + LEVEL);
            c.cycles += 4;
            c.g[S1][0] = 1;
            c.lwc1(1, gp.wrapping_sub(0x3a64)); // 0.01
            c.add_s(0, 0, 1);
            c.min_s(0, 0, 2);
        } else {
            // level = max(level - 1/60, 0).
            c.cycles += 6;
            c.lwc1(0, fx + LEVEL);
            c.lwc1(2, gp.wrapping_sub(0x3a68)); // 1/60
            c.mtc1(0, 1);
            c.sub_s(0, 0, 2);
            c.max_s(0, 0, 1);
        }

        // Cut the level when falling fast (undriven) or too slow.
        c.cycles += 4;
        c.swc1(0, fx + LEVEL);
        c.g[V0][0] = c.lw(fx + ACTIVE);
        let mut cut = false;
        if c.g[V0][0] == 0 {
            c.cycles += 13;
            spray8_velocity(c, fx, 0x002e_2018)?;
            c.lwc1(1, r32(c, V0).wrapping_add(0x8));
            c.lwc1(0, gp.wrapping_sub(0x3a60)); // -3333.33
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.sw(fx + LEVEL, 0);
                cut = true;
            }
        }
        if !cut {
            c.cycles += 6;
            c.lwc1(1, gp.wrapping_sub(0x3a5c)); // speed limit
            c.lwc1(0, fx + SPEED);
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.cycles += 1;
                c.sw(fx + LEVEL, 0);
                cut = true;
            }
        }
        if cut {
            c.cycles += 1;
        }
        c.lq(2, fx + POS);

        // pos -> sp+0.
        c.cycles += 2;
        c.sq(2, sp(c));
        let mut spawn = false;
        if c.g[S1][0] == 0 {
            c.cycles += 5;
            c.lwc1(1, fx + LEVEL);
            c.mtc1(0, 21);
            c.c_lt_s(21, 1);
            spawn = c.fcc();
        }
        if spawn {
            // alpha = clamp(1.25 level, 0, 1).
            c.cycles += 9;
            c.g[AT][0] = 0x3fa0_0000;
            c.mtc1(AT, 0);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 22);
            c.mul_s(0, 1, 0);
            c.c_le_s(21, 0);
            let pos = c.fcc();
            c.mov_s(2, 21);
            if pos {
                c.cycles += 1;
                c.min_s(2, 0, 22);
            }
            c.cycles += 13;
            c.mov_s(20, 2);
            spray8_velocity(c, fx, 0x002e_20b4)?;
            c.lwc1(0, r32(c, V0).wrapping_add(0x8));
            c.c_lt_s(0, 21);
            if c.fcc() {
                // Falling: alpha *= clamp((z + 3333.33) / 3333.33, 0, 1).
                c.cycles += 15;
                spray8_velocity(c, fx, 0x002e_20e4)?;
                c.lwc1(1, gp.wrapping_sub(0x3a58)); // 3333.33
                c.lwc1(0, r32(c, V0).wrapping_add(0x8));
                c.add_s(0, 0, 1);
                c.div_s(0, 0, 1);
                c.c_le_s(21, 0);
                let pos = c.fcc();
                c.mov_s(2, 21);
                if pos {
                    c.cycles += 1;
                    c.min_s(2, 0, 22);
                }
                c.cycles += 2;
                c.mul_s(20, 20, 2);
            }
            c.lq(2, fx + COL);

            // col (alpha in w) -> sp+0x10; velocity terms.
            c.cycles += 68;
            c.g[A0][0] = c.lw(fx);
            let sp0 = sp(c);
            c.sq(2, sp0 + 0x10);
            c.swc1(20, sp0 + 0x1c);
            c.g[A1][0] = sx(r32(c, A0).wrapping_add(0x6c0));
            c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x6c0));
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x10));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x14));
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002e_2134)?;
            let sp0 = sp(c);
            // velocity * curve[0x818] -> sp+0x40.
            c.g[V1][0] = c.lw(fx + P2C);
            c.g[A0][0] = sx(fx);
            c.g[A1][0] = sx(sp0);
            c.lwc1(0, r32(c, V1).wrapping_add(0x818));
            c.mfc1(A2, 0);
            c.lqc2(4, r32(c, V0));
            c.qmtc2(A2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp0 + 0x50);
            c.lq(2, sp0 + 0x50);
            c.g[V1][0] = c.lw(fx + P2C);
            c.sq(2, sp0 + 0x40);
            // dir * curve[0x81c] -> sp+0x50; sum -> sp+0x30.
            c.lwc1(0, r32(c, V1).wrapping_add(0x81c));
            c.mfc1(A2, 0);
            c.lqc2(4, fx + DIR);
            c.qmtc2(A2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp0 + 0x60);
            c.lq(2, sp0 + 0x60);
            c.sq(2, sp0 + 0x50);
            c.lqc2(3, sp0 + 0x40);
            c.lqc2(4, sp0 + 0x50);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp0 + 0x60);
            c.lq(2, sp0 + 0x60);
            c.g[V1][0] = c.lw(fx + P2C);
            c.sq(2, sp0 + 0x30);
            // dir * curve[0x820] * speed -> sp+0x60; total -> sp+0x20.
            c.lwc1(0, r32(c, V1).wrapping_add(0x820));
            c.mfc1(A2, 0);
            c.lqc2(4, fx + DIR);
            c.qmtc2(A2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp0 + 0x80);
            c.lq(2, sp0 + 0x80);
            c.lwc1(0, fx + SPEED);
            c.sq(2, sp0 + 0x70);
            c.mfc1(V1, 0);
            c.lqc2(4, sp0 + 0x70);
            c.qmtc2(V1, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp0 + 0x80);
            c.lq(2, sp0 + 0x80);
            c.sq(2, sp0 + 0x60);
            c.lqc2(3, sp0 + 0x30);
            c.lqc2(4, sp0 + 0x60);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp0 + 0x80);
            c.lq(2, sp0 + 0x80);
            c.sq(2, sp0 + 0x20);
            SPRAY_ADD_RANDOM_VELOCITY.call(c, 0x002e_21fc)?;
            // Spawn one particle.
            let sp0 = sp(c);
            c.lwc1(12, r32(c, GP).wrapping_sub(0x3a54)); // 1/60
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx(sp0);
            c.g[A2][0] = sx(sp0 + 0x20);
            c.g[A3][0] = sx(sp0 + 0x10);
            c.g[T0][0] = 1;
            call_ptr(c, DYNAMIC_EMITTER_SPAWN_AT, 0x002e_2218)?;
        } else {
            // Keep slot 8 alive at the position, no spawn.
            c.cycles += 8;
            c.lwc1(12, r32(c, GP).wrapping_sub(0x3a50)); // 1/60
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx(fx + POS);
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, DYNAMIC_EMITTER_SPAWN_AT, 0x002e_223c)?;
        }
        frame.restore(c, S0, 0xc0);
        c.cycles += 8;
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 22, 0xe0);
        frame.restore_fpr(c, 21, 0xd8);
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    });
}

// ------------------------------------------- LensFX_UpdateSourceVisibility

/// `void LensFX_UpdateSourceVisibility(LensFX* self)` (0x2e3130): refresh
/// how visible each lens-flare source is. Nothing while `+0x6494` is set.
/// When the occlusion test is on (`gp+0xa28`), each of the `+0x6490`
/// sources (0x210-byte records from `+0x6070`) whose flag at `+0x2e` is set
/// asks the occlusion query (virtual `+0x20` of `+0x649c`) and stores its
/// result, or the source's fallback (`+0x6694`), at `+0x6274`. Then for
/// each source the per-element visibility of its element list (count at
/// `+0x60 + 4 i`, 0x30-byte elements at `+0x6088 + 0x3000 i`): queried
/// elements (flag at +0xe) store the query result, or else copy the value
/// from the source's own element (at `+0x6068 + 0x3000 i`), into their
/// target's `+0x40`.
pub fn lens_fx_update_source_visibility(c: &mut Cpu) {
    /// LensFX fields.
    const BUSY: u32 = 0x6494;
    const SOURCE_COUNT: u32 = 0x6490;
    const OCCLUSION: u32 = 0x649c;
    const SOURCES: u32 = 0x6070;
    const SOURCE_SIZE: u32 = 0x210;
    const SOURCE_FALLBACK: u32 = 0x6694;
    const SOURCE_QUERIED: u32 = 0x609e;
    const SOURCE_VISIBILITY: u32 = 0x6274;
    const ELEMENT_COUNTS: u32 = 0x60;
    const ELEMENT_SIZE: u32 = 0x30;
    /// `gp+0xa28`: occlusion queries enabled.
    const OCCLUSION_ON_GP: u32 = 0xa28;
    /// Loop heads, for event delivery.
    const SOURCE_HEAD: u32 = 0x002e_3190;
    const LIST_HEAD: u32 = 0x002e_3228;
    const ELEMENT_HEAD: u32 = 0x002e_3280;

    /// `occlusion->Query(self, at, &out)` (virtual +0x20) with `out` at
    /// `sp+out_off` zeroed first; `a1` is already the query position.
    fn query(c: &mut Cpu, this: u32, out_off: u32, ret: u32) -> crate::call::Flow {
        c.g[V1][0] = c.lw(this.wrapping_add(OCCLUSION));
        c.sw(sp(c) + out_off, 0);
        c.g[A2][0] = if out_off == 0 {
            c.g[SP][0]
        } else {
            sx(sp(c) + out_off)
        };
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x20));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x24));
        let target = r32(c, V0);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, target, ret)
    }

    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x90);
        frame.save(c, S0, 0xb0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save_ra(c, 0x20);
        let this = r32(c, S2);
        c.g[V0][0] = c.lw(this.wrapping_add(BUSY));
        frame.restore(c, S0, 0xb0);
        if c.g[V0][0] == 0 {
            // The sources' own visibility.
            c.cycles += 4;
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(OCCLUSION_ON_GP));
            c.g[S5][0] = 0;
            c.g[V1][0] = c.lw(this.wrapping_add(SOURCE_COUNT));
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[S4][0] = 0;
                if (c.g[V1][0] as i64) > 0 {
                    c.cycles += 3;
                    c.g[S3][0] = sx(this.wrapping_add(SOURCE_FALLBACK));
                    c.g[S1][0] = sx(this.wrapping_add(SOURCES));
                    c.g[S0][0] = c.g[S2][0];
                    loop {
                        c.cycles += 3;
                        let src = r32(c, S0);
                        c.g[V0][0] = c.lhu(src.wrapping_add(SOURCE_QUERIED));
                        c.g[A1][0] = c.g[S1][0];
                        if c.g[V0][0] != 0 {
                            c.cycles += 9;
                            query(c, this, 0, 0x002e_31b8)?;
                            c.lwc1(0, sp(c));
                            if c.g[V0][0] == 0 {
                                c.cycles += 1;
                                c.lwc1(0, r32(c, S3));
                            }
                            c.cycles += 1;
                            c.swc1(0, src.wrapping_add(SOURCE_VISIBILITY));
                        }
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(this.wrapping_add(SOURCE_COUNT));
                        c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
                        c.g[S3][0] = sx(r32(c, S3).wrapping_add(SOURCE_SIZE));
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(SOURCE_SIZE));
                        c.g[V1][0] = c.g[V0][0];
                        let more = (c.g[S4][0] as i64) < (c.g[V0][0] as i64);
                        c.g[V0][0] = more as u64;
                        c.g[S0][0] = sx(src.wrapping_add(SOURCE_SIZE));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, SOURCE_HEAD);
                        }
                    }
                }
            }

            // The element lists.
            c.cycles += 2;
            c.g[A0][0] = 0;
            if (c.g[V1][0] as i64) > 0 {
                c.cycles += 14;
                let sp0 = sp(c);
                c.g[V0][0] = 0x6000;
                c.g[V1][0] = sx(r32(c, S5) << 3);
                c.g[A1][0] = c.g[V0][0] | 0x88;
                c.sw(sp0 + 0x8, c.g[V1][0]);
                c.mult(0, V0, S5, V0);
                c.sw(sp0 + 0x10, c.g[A1][0]);
                c.g[V1][0] = sx(this.wrapping_add(ELEMENT_COUNTS));
                c.sw(sp0 + 0xc, c.g[V1][0]);
                c.g[A1][0] = sx(r32(c, V0).wrapping_add(0x88));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x68));
                c.sw(sp0 + 0x14, c.g[A1][0]);
                c.g[V0][0] = sx(this.wrapping_add(r32(c, V0)));
                c.sw(sp0 + 0x18, c.g[V0][0]);
                c.g[V0][0] = sx(r32(c, A0) << 2);
                loop {
                    // List a0: its count, the element pointers.
                    c.cycles += 9;
                    let sp0 = sp(c);
                    c.g[V1][0] = c.lw(sp0 + 0x8);
                    c.g[S6][0] = c.g[V0][0];
                    c.g[A1][0] = c.lw(sp0 + 0xc);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.g[S4][0] = 0;
                    c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    c.g[FP][0] = sx(r32(c, A0).wrapping_add(1));
                    if (c.g[V0][0] as i64) > 0 {
                        c.cycles += 13;
                        c.g[V0][0] = 0x3000;
                        c.g[S7][0] = c.g[V1][0];
                        c.mult(0, A0, A0, V0);
                        c.g[V1][0] = c.lw(sp0 + 0x10);
                        c.g[A1][0] = c.lw(sp0 + 0x14);
                        c.g[S5][0] = c.g[S6][0];
                        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                        c.g[S3][0] = sx(r32(c, V0).wrapping_add(this));
                        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                        c.g[V0][0] = c.lw(sp0 + 0x18);
                        c.g[S0][0] = sx(r32(c, V1).wrapping_add(this));
                        c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                        loop {
                            c.cycles += 3;
                            let el = r32(c, S0);
                            c.g[V0][0] = c.lhu(el.wrapping_add(0xe));
                            c.g[A1][0] = c.g[S1][0];
                            if c.g[V0][0] != 0 {
                                c.cycles += 9;
                                query(c, this, 0x4, 0x002e_32a8)?;
                                c.lwc1(0, sp(c) + 0x4);
                                if c.g[V0][0] != 0 {
                                    // The query result.
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw(el);
                                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S6)));
                                    c.swc1(0, r32(c, V0).wrapping_add(0x40));
                                } else {
                                    // The source element's value.
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lw(r32(c, S3));
                                    c.g[V1][0] = c.lw(el);
                                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S5)));
                                    c.lwc1(0, r32(c, V0).wrapping_add(0x40));
                                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S5)));
                                    c.swc1(0, r32(c, V1).wrapping_add(0x40));
                                }
                            }
                            c.cycles += 7;
                            c.g[V0][0] = c.lw(r32(c, S7));
                            c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
                            c.g[S3][0] = sx(r32(c, S3).wrapping_add(ELEMENT_SIZE));
                            c.g[S0][0] = sx(el.wrapping_add(ELEMENT_SIZE));
                            let more = (c.g[S4][0] as i64) < (c.g[V0][0] as i64);
                            c.g[V0][0] = more as u64;
                            c.g[S1][0] = sx(r32(c, S1).wrapping_add(ELEMENT_SIZE));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, ELEMENT_HEAD);
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(this.wrapping_add(SOURCE_COUNT));
                    c.g[A0][0] = c.g[FP][0];
                    let more = (c.g[A0][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = sx(r32(c, A0) << 2);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, LIST_HEAD);
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0xb0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `void SnowFall_Update(SnowFall* self)` (0x2e5da0): for each view, build
/// the weather record on the stack (from the override block when gp+0x164c
/// is set, else from the world painter's weather of kind 6 for view 0 and
/// 7 for the others) and pass its first value to that view's splash manager
/// as the snow amount; then update each of the twelve snow boxes with the
/// record of its view. Frame 0x120 (records at_2 +0, s0-s7, fp).
pub fn snowfall_update(c: &mut Cpu) {
    use snowfall::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x120);
        c.g[A1][0] = c.lw(gp_lit(c, GP_APP));
        for (reg, off) in [
            (S3, 0xe0),
            (FP, 0x90),
            (S0, 0x110),
            (S1, 0x100),
            (S2, 0xf0),
            (S4, 0xd0),
            (S5, 0xc0),
            (S6, 0xb0),
            (S7, 0xa0),
        ] {
            frame.save(c, reg, off);
        }
        c.g[FP][0] = c.g[A0][0];
        frame.save_ra(c, 0x80);
        c.g[V0][0] = c.lw(at_2(c, A1, 0x84));
        c.g[V1][0] = c.lw(at_2(c, V0, 0x84));
        c.g[A0][0] = c.lw(at_2(c, V1, 0x10));
        c.g[S3][0] = 0;
        if c.g[A0][0] != 0 {
            // s3 = view, s2 = its record, s1 = record + 0x3c.
            c.cycles += 8;
            c.g[S7][0] = 0x50_0000;
            c.g[S6][0] = 0x50_0000;
            c.g[S5][0] = sx(OVERRIDE_C0);
            c.g[S4][0] = sx(OVERRIDE_C1);
            c.g[S2][0] = c.g[SP][0];
            c.g[S1][0] = sx(sp(c).wrapping_add(0x3c));
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(GP_WEATHER_OVERRIDE));
            loop {
                // 0x2e5e08
                c.cycles += 2;
                let gp = r32(c, GP);
                c.lwc1(0, gp.wrapping_add(GP_OVERRIDE_F[0]));
                if c.g[V0][0] != 0 {
                    c.cycles += 32;
                    for (f, &off) in GP_OVERRIDE_F.iter().enumerate().skip(1) {
                        c.lwc1(f, gp.wrapping_add(off));
                    }
                    for k in 0..4 {
                        c.lwc1(8 + k, OVERRIDE_C0 + 4 * k as u32);
                    }
                    for k in 0..4 {
                        c.lwc1(12 + k, OVERRIDE_C1 + 4 * k as u32);
                    }
                    let rec = r32(c, S2);
                    for f in 0..16 {
                        c.swc1(f, rec.wrapping_add(4 * f as u32));
                    }
                } else {
                    c.cycles += 51;
                    c.g[V0][0] = 6;
                    c.g[S0][0] = if c.g[S3][0] == 0 { 6 } else { 7 };
                    for (n, getter) in GETTERS.iter().enumerate() {
                        c.g[A0][0] = c.g[S0][0];
                        getter.call(c, 0x002e_5ea4 + 0xc * n as u32)?;
                        let slot = r32(c, S2).wrapping_add(4 * n as u32);
                        c.swc1(0, slot);
                    }
                }
                // 0x2e5f5c: the view's splash manager gets f[0].
                c.cycles += 17;
                c.g[V0][0] = c.lw(gp_lit(c, GP_APP));
                c.g[A1][0] = c.g[S3][0];
                c.g[S3][0] = sx(at_2(c, S3, 1));
                c.g[S1][0] = sx(at_2(c, S1, REC_SIZE));
                c.g[A0][0] = c.lw(at_2(c, V0, 0x84));
                call_ptr(c, GET_SPLASH_MANAGER, 0x002e_5f74)?;
                c.lwc1(12, r32(c, S2));
                c.g[A0][0] = c.g[V0][0];
                c.g[S2][0] = sx(at_2(c, S2, REC_SIZE));
                SET_SNOW_AMOUNT.call(c, 0x002e_5f84)?;
                c.g[V0][0] = c.lw(gp_lit(c, GP_APP));
                c.g[V1][0] = c.lw(at_2(c, V0, 0x84));
                c.g[A0][0] = c.lw(at_2(c, V1, 0x84));
                c.g[V0][0] = c.lw(at_2(c, A0, 0x10));
                let more = c.g[S3][0] < c.g[V0][0];
                c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(GP_WEATHER_OVERRIDE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_5e08);
                }
            }
        }

        // 0x2e5fa0: the twelve boxes (s1 counts 11..0).
        c.cycles += 4;
        c.g[S0][0] = sx(at_2(c, FP, BOXES));
        c.g[S1][0] = 0xb;
        c.g[A1][0] = c.lw(r32(c, S0));
        loop {
            c.cycles += 4;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[S0][0] = sx(at_2(c, S0, 4));
            c.g[A0][0] = c.g[A1][0];
            if c.g[A1][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.lw(r32(c, A1));
                c.g[A1][0] = sx(r32(c, A1) << 6);
                c.g[A1][0] = sx(sp(c).wrapping_add(r32(c, A1)));
                SNOW_BOX_UPDATE.call(c, 0x002e_5fd0)?;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[A1][0] = c.lw(r32(c, S0));
            if event_due(c) {
                deliver_event_here(c, 0x002e_5fb0);
            }
        }
        c.cycles += 12;
        for (reg, off) in [
            (S0, 0x110),
            (S1, 0x100),
            (S2, 0xf0),
            (S3, 0xe0),
            (S4, 0xd0),
            (S5, 0xc0),
            (S6, 0xb0),
            (S7, 0xa0),
            (FP, 0x90),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- LensFX_DrawFlareQuad

/// `void LensFX_DrawFlareQuad(LensFX* fx, int idx)` (0x2e2868): draw one
/// lens-flare sprite. Skipped unless the flare is on (+0x2c). The colour's
/// direction is the particle's (+0x20) velocity (+0x10) normalised (as is
/// when zero); its alpha is the particle's per-sprite alpha `[idx]` (+0x40)
/// when the flare uses alpha (+0x2e), else 1.0, and the render state's
/// blend bits are set to match. With a positive alpha the texture (+0x1024
/// glow, +0x1028 ring) and size (180 / 100 / 350, else 0) follow the
/// particle's type bits (+0xc & 0x70), and the graphics manager's sprite
/// draw method (vtable +0x10d8, slot +0x278) is called with the position
/// (+0x1c, w 1), the size, uv (0,0)-(1,1), the colour, the rotation (+0x28)
/// and mode 2.
pub fn lens_fx_draw_flare_quad(c: &mut Cpu) {
    const ON: u32 = 0x2c;
    const USE_ALPHA: u32 = 0x2e;
    const PARTICLE: u32 = 0x20;
    const ROTATION: u32 = 0x28;
    /// Particle fields.
    const TYPE: u32 = 0xc;
    const VELOCITY: u32 = 0x10;
    const POSITION: u32 = 0x1c;
    const ALPHA: u32 = 0x40;
    /// Graphics manager fields.
    const GFX_MAN: u32 = 0x2a90;
    const STATE_TOP: u32 = 0xe84;
    const TEX_GLOW: u32 = 0x1024;
    const TEX_RING: u32 = 0x1028;
    const VTABLE: u32 = 0x10d8;
    const DRAW_SPRITE: u32 = 0x278;
    /// The render state's flags word.
    const FLAGS: u32 = 0x4;

    /// `top->flags = (top->flags & mask) | set` with the top in `base` and
    /// the word in v0 (the last store is left to the caller).
    fn flags(c: &mut Cpu, gfx: u32, base: usize, mask: u64, set: u64, store: bool) {
        c.g[base][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
        let at = (c.g[base][0] as u32).wrapping_add(FLAGS);
        c.g[V0][0] = (Addr(at).lw(c) & mask) | set;
        if store {
            c.sw(at, c.g[V0][0]);
        }
    }

    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x70);
        let fx = c.g[A0][0];
        c.g[T3][0] = fx;
        frame.save_ra(c, 0x60);
        let fx = Addr(fx as u32);
        c.g[V0][0] = fx.off(ON).lhu(c);
        frame.restore_ra(c, 0x60);
        if c.g[V0][0] != 0 {
            // Normalised velocity into sp+0..8.
            c.cycles += 15;
            let part = fx.off(PARTICLE).lw(c);
            c.g[V0][0] = part;
            let part = Addr(part as u32);
            c.f[3] = 0;
            let vx = part.off(VELOCITY).read_fpu(c);
            let vy = part.off(VELOCITY + 4).read_fpu(c);
            let vz = part.off(VELOCITY + 8).read_fpu(c);
            c.f[5] = vx.0;
            c.f[6] = vy.0;
            c.f[4] = vz.0;
            let sum = vx * vx + vy * vy;
            let zz = vz * vz;
            c.f[2] = zz.0;
            let sum = sum + zz;
            c.f[0] = sum.0;
            let len = sum.sqrt();
            c.f[1] = len.0;
            let zero = len.eq_s(FpuF32::ZERO);
            abi::set_fcc(c, zero);
            let sp = crate::call::sp(c);
            if zero {
                c.swc1(5, sp);
                c.cycles += 4;
                let y = part.off(VELOCITY + 4).read_fpu(c);
                c.f[0] = y.0;
                c.swc1(0, sp.wrapping_add(0x4));
                let z = part.off(VELOCITY + 8).read_fpu(c);
                c.f[1] = z.0;
                c.swc1(1, sp.wrapping_add(0x8));
            } else {
                c.cycles += 10;
                c.g[AT][0] = FpuF32::ONE.0 as u64;
                let inv = FpuF32::ONE.div(len);
                c.f[1] = (vz * inv).0;
                c.f[2] = (vx * inv).0;
                c.f[0] = (vy * inv).0;
                c.swc1(1, sp.wrapping_add(0x8));
                c.swc1(2, sp);
                c.swc1(0, sp.wrapping_add(0x4));
            }

            // The colour: direction at sp+0x14, alpha at sp+0x10.
            c.cycles += 9;
            c.lwc1(0, sp);
            c.lwc1(1, sp.wrapping_add(0x4));
            c.lwc1(2, sp.wrapping_add(0x8));
            c.g[V0][0] = fx.off(USE_ALPHA).lhu(c);
            c.swc1(0, sp.wrapping_add(0x14));
            c.swc1(1, sp.wrapping_add(0x18));
            c.swc1(2, sp.wrapping_add(0x1c));
            let use_alpha = c.g[V0][0] != 0;
            let gfx = Addr((c.g[GP][0] as u32).wrapping_add(GFX_MAN)).lw(c);
            c.g[T2][0] = gfx;
            let gfx = gfx as u32;
            if use_alpha {
                c.cycles += 28;
                c.g[V0][0] = fx.off(PARTICLE).lw(c);
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[A1][0] = 0xffff_ffff_fe7f_ffff;
                c.g[T1][0] = 0x100_0000;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.lwc1(0, r32(c, V0).wrapping_add(ALPHA));
                c.g[A2][0] = 0xffff_ffff_ffcf_ffff;
                c.g[T0][0] = 0xffff_ffff_fff0_0fff;
                c.swc1(0, sp.wrapping_add(0x10));
                c.g[A3][0] = 0x1_4000;
                let (a1, t1, a2, t0, a3) =
                    (c.g[A1][0], c.g[T1][0], c.g[A2][0], c.g[T0][0], c.g[A3][0]);
                flags(c, gfx, A0, a1, t1, true);
                flags(c, gfx, V1, a2, 0, true);
                flags(c, gfx, A0, t0, a3, false);
            } else {
                c.cycles += 25;
                c.g[AT][0] = FpuF32::ONE.0 as u64;
                c.f[0] = FpuF32::ONE.0;
                c.g[V1][0] = 0xffff_ffff_fe7f_ffff;
                c.g[T0][0] = 0x80_0000;
                c.swc1(0, sp.wrapping_add(0x10));
                c.g[A0][0] = 0xffff_ffff_ffcf_ffff;
                c.g[A2][0] = 0xffff_ffff_fff0_0fff;
                c.g[A1][0] = 0x1_4000;
                let (v1, t0, a0, a2, a1) =
                    (c.g[V1][0], c.g[T0][0], c.g[A0][0], c.g[A2][0], c.g[A1][0]);
                flags(c, gfx, A3, v1, t0, true);
                flags(c, gfx, V1, a0, 0, true);
                flags(c, gfx, A0, a2, a1, false);
            }
            c.cycles += 7;
            c.sw(r32(c, A0).wrapping_add(FLAGS), c.g[V0][0]);
            c.lwc1(0, sp.wrapping_add(0x10));
            c.f[1] = 0;
            let alpha = FpuF32(c.f[0]);
            let invisible = alpha.le(FpuF32::ZERO);
            abi::set_fcc(c, invisible);
            frame.restore_ra(c, 0x60);
            if !invisible {
                // Texture and size by the particle type.
                c.cycles += 6;
                c.g[V0][0] = fx.off(PARTICLE).lw(c);
                c.g[A0][0] = 0x20;
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(TYPE)).lw(c) & 0x70;
                c.f[3] = c.f[1];
                let kind = c.g[V1][0];
                let tex = match kind {
                    0x20 => {
                        c.cycles += 5;
                        c.g[V1][0] = Addr(gfx.wrapping_add(TEX_RING)).lw(c);
                        Some(0x42c8_0000) // 100.0
                    }
                    _ => {
                        c.cycles += 3;
                        c.g[V0][0] = (kind < 0x21) as u64;
                        if kind < 0x21 {
                            c.g[V0][0] = 0x10;
                            c.cycles += 2;
                            if kind == 0x10 {
                                c.g[V1][0] = Addr(gfx.wrapping_add(TEX_GLOW)).lw(c);
                                c.cycles += 4;
                                Some(0x4334_0000) // 180.0
                            } else {
                                c.cycles += 2;
                                None
                            }
                        } else {
                            c.g[V0][0] = 0x10;
                            c.cycles += 3;
                            c.g[V0][0] = 0x40;
                            if kind == 0x40 {
                                c.g[V1][0] = Addr(gfx.wrapping_add(TEX_GLOW)).lw(c);
                                c.cycles += 3;
                                Some(0x43af_0000) // 350.0
                            } else {
                                c.cycles += 2;
                                None
                            }
                        }
                    }
                };
                if let Some(size) = tex {
                    c.g[AT][0] = size as u64;
                    c.f[3] = size;
                    c.g[V0][0] = Addr(gfx.wrapping_add(STATE_TOP)).lw(c);
                    c.cycles += 2;
                    c.sh(r32(c, V0).wrapping_add(0x10), c.g[V1][0]);
                }
                c.g[V0][0] = fx.off(PARTICLE).lw(c);

                // Draw: (pos, size, uv0, uv1, colour, rot, 2).
                c.cycles += 28;
                let part = r32(c, V0);
                c.g[A1][0] = sx(sp.wrapping_add(0x20));
                let vt = Addr(gfx.wrapping_add(VTABLE)).lw(c);
                c.g[V1][0] = vt;
                c.g[A2][0] = sx(sp.wrapping_add(0x30));
                c.lwc1(0, part.wrapping_add(POSITION));
                c.g[A3][0] = sx(sp.wrapping_add(0x40));
                c.g[A0][0] = Addr((vt as u32).wrapping_add(DRAW_SPRITE)).lh(c);
                c.g[T0][0] = sx(sp.wrapping_add(0x50));
                c.swc1(0, sp.wrapping_add(0x20));
                c.g[T1][0] = sx(sp.wrapping_add(0x10));
                c.g[AT][0] = FpuF32::ONE.0 as u64;
                c.f[2] = FpuF32::ONE.0;
                c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
                c.lwc1(1, part.wrapping_add(POSITION + 4));
                c.f[12] = fx.off(ROTATION).read_fpu(c).0;
                c.swc1(1, sp.wrapping_add(0x24));
                c.lwc1(0, part.wrapping_add(POSITION + 8));
                c.swc1(3, sp.wrapping_add(0x34));
                c.swc1(0, sp.wrapping_add(0x28));
                c.swc1(2, sp.wrapping_add(0x54));
                c.swc1(2, sp.wrapping_add(0x2c));
                c.swc1(3, sp.wrapping_add(0x30));
                c.sw(sp.wrapping_add(0x40), 0);
                c.sw(sp.wrapping_add(0x44), 0);
                c.swc1(2, sp.wrapping_add(0x50));
                let method = Addr((vt as u32).wrapping_add(DRAW_SPRITE + 4)).lw(c);
                c.g[V0][0] = method;
                c.g[T2][0] = 2;
                call_ptr(c, method as u32, 0x002e_2af4)?;
                frame.restore_ra(c, 0x60);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod flare_set {
    use super::ee;
    use super::Callee;

    /// `gp-0x854`: the renderer (`+0x10d8` vtable: width at +0x50, height
    /// at +0x58, DrawSprite at +0x270; `+0xe84` the draw state, whose
    /// halfword at +0x10 takes the word at `+0x1004`).
    pub const GP_RENDERER: u32 = 0x854;

    pub const VTABLE: u32 = 0x10d8;

    pub const RENDER_STATE: u32 = 0xe84;

    pub const BLEND: u32 = 0x1004;

    /// `LensFX`: sun position at +0x1f0/+0x1f4, the flare count at +0x200,
    /// the intensity at +0x204; flares (0x1c each) from +0x30: type,
    /// distance along the sun ray (+0x34), size (+0x38), colour (+0x3c).
    pub const SUN_X: u32 = 0x1f0;

    pub const SUN_Y: u32 = 0x1f4;

    pub const COUNT: u32 = 0x200;

    pub const INTENSITY: u32 = 0x204;

    pub const FLARE_TYPE: u32 = 0x30;

    pub const FLARE_DIST: u32 = 0x34;

    pub const FLARE_SIZE: u32 = 0x38;

    pub const FLARE_COLOUR: u32 = 0x3c;

    pub const FLARE_STRIDE: u32 = 0x1c;

    /// `WorldPainter_GetSunFlareIntensity(6)`.
    pub const SUN_FLARE_INTENSITY: Callee = Callee {
        addr: 0x002e_e268,
        f: ee::s09::f_002ee268,
    };
}

pub(crate) mod spray9 {

    /// The rider object.
    pub const OBJ: u32 = 0x0;

    /// The spray level (seconds left).
    pub const LEVEL: u32 = 0x4;

    /// The emitter block (slot 9 lives at +0x1290).
    pub const P28: u32 = 0x28;

    /// The curve table (0x900: velocity scale).
    pub const P2C: u32 = 0x2c;

    /// Ring index into the bone list (0..30).
    pub const RING: u32 = 0x78;

    /// The bone list the ring walks.
    pub const BONES: u32 = 0x7c;

    /// The particle colour.
    pub const COL: u32 = 0x90;

    /// The speed compared against the limit.
    pub const SPEED: u32 = 0xb8;

    /// The spray mode (2: the level does not fade).
    pub const MODE: u32 = 0xd4;

    /// Emitter slot 9 inside the emitter block.
    pub const SLOT: u32 = 0x1290;
}

pub(crate) mod lens_prep {
    use super::*;

    /// The camera / graphics object (`*(gp + 0x2a90)`), its interface's
    /// vtable pointer at +0x10d8.
    pub const CAMERA_GP: u32 = 0x2a90;

    pub const CAM_VT: u32 = 0x10d8;

    /// Interface slots: `GetViewRect(out)` (16-byte struct returned through
    /// a hidden pointer), `GetViewMatrix()` (returns a 4x4 matrix pointer),
    /// `GetFov(&x, &y, &z)`.
    pub const VT_VIEW_RECT: u32 = 0xe0;

    pub const VT_VIEW_MATRIX: u32 = 0x118;

    pub const VT_FOV: u32 = 0xf0;

    /// LensFX: view rect copy at +0, view matrix copy at +0x10, the three
    /// FOV outputs at +0x50/+0x54/+0x58, per-view source counts at +0x60,
    /// per-view flare arrays (0x3000 bytes, 0x30 per flare) at +0x68, per-view
    /// sun state (0x210 bytes) at +0x6070.
    pub const MATRIX: u32 = 0x10;

    pub const FOV_X: u32 = 0x50;

    pub const FOV_Y: u32 = 0x54;

    pub const FOV_Z: u32 = 0x58;

    pub const COUNTS: u32 = 0x60;

    pub const FLARES: u32 = 0x68;

    pub const FLARE_VIEW_SIZE: u64 = 0x3000;

    pub const FLARE_SIZE: u32 = 0x30;

    pub const SUN: u32 = 0x6070;

    pub const SUN_SIZE: u64 = 0x210;

    pub const UPDATE_SUN_POSITION: Callee = Callee {
        addr: 0x002f_4db8,
        f: ee::s09::LensFX_UpdateSunPosition,
    };

    pub const PROJECT_SOURCE: Callee = Callee {
        addr: 0x002e_2b00,
        f: ee::s09::LensFX_ProjectSource,
    };
}

/// `(float)(unsigned)v` the way gcc converts an unsigned int: directly when
/// it is non-negative, else halved (keeping the low bit) and doubled.
/// Leaves `f1` = the result, and v0/v1 as the compiled code does (`v1` is
/// set by the caller where the original sets it in a delay slot).
pub(crate) fn flare_u32_to_f1(c: &mut Cpu, extra: u64) {
    let v = c.g[V0][0];
    if (v as i64) < 0 {
        c.cycles += 2 + extra;
        let half = sx((v as u32) >> 1);
        c.g[V1][0] = half;
        c.g[V0][0] = (v & 1) | half;
        c.mtc1(V0, 1);
        c.cvt_s_w(1, 1);
        c.add_s(1, 1, 1);
    } else {
        c.cycles += 4;
        c.mtc1(V0, 1);
        c.cvt_s_w(1, 1);
    }
}

/// An 8-byte struct copy through GPR `reg` (`ldl/ldr` + `sdl/sdr`).
pub(crate) fn flare_copy8(c: &mut Cpu, reg: usize, src: u32, dst: u32) {
    c.ldl(reg, src.wrapping_add(0x7));
    c.ldr(reg, src);
    c.sdl(reg, dst.wrapping_add(0x7));
    c.sdr(reg, dst);
}

/// `void LensFX_DrawFlareSet(LensFX* fx)` (0x2f4690): draws the lens
/// flares. The screen centre is half the renderer's width / height; each
/// flare sits at `centre + (sun - centre) * dist`, with its colour's red
/// scaled by the intensity times `WorldPainter_GetSunFlareIntensity(6)`,
/// the quarter of the flare texture its type selects (0..3, else the left
/// half), and a size of `size * 300`, drawn through the renderer's
/// DrawSprite. Frame 0xf0 (s0-s3, f20/f21; centre at +0, sun at +0x10, ray
/// at +0x20, position at +0x30, uv0 at +0x40, colour at +0x50, uv1 at
/// +0x60, scratch / size at +0x70, the sprite position at +0x80).
pub fn lens_fx_draw_flare_set(c: &mut Cpu) {
    use flare_set::*;
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xf0);
        let gp = c.g[GP][0] as u32;
        let rend = c.lw(gp.wrapping_sub(GP_RENDERER));
        c.g[A1][0] = rend;
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        let fx = c.g[A0][0];
        c.g[S0][0] = fx;
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 21, 0xe8);
        frame.save_fpr(c, 20, 0xe0);
        let fx = fx as u32;
        // Width.
        let vt = c.lw((rend as u32).wrapping_add(VTABLE));
        c.g[V1][0] = vt;
        let slot = crate::call::VSlot::read(c, vt, 0x50);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(rend);
        call_ptr(c, slot.func as u32, 0x002f_46cc)?;
        let rend = c.lw(gp.wrapping_sub(GP_RENDERER));
        c.g[A1][0] = rend;
        flare_u32_to_f1(c, 4);

        // centre.x = width * 0.5; height.
        c.cycles += 10;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        let vt = c.lw((rend as u32).wrapping_add(VTABLE));
        c.g[V1][0] = vt;
        c.mul_s(20, 1, 0);
        let slot = crate::call::VSlot::read(c, vt, 0x58);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(rend);
        call_ptr(c, slot.func as u32, 0x002f_471c)?;
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
        flare_u32_to_f1(c, 3);

        // centre, sun, ray = sun - centre; the renderer's blend word.
        c.cycles += 20;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 5);
        let s = sp(c);
        c.lwc1(0, fx.wrapping_add(SUN_X));
        c.mul_s(1, 1, 5);
        c.lwc1(3, fx.wrapping_add(SUN_Y));
        c.sub_s(4, 0, 20);
        c.swc1(20, s);
        c.swc1(0, s.wrapping_add(0x10));
        c.sub_s(2, 3, 1);
        c.swc1(1, s.wrapping_add(0x4));
        c.swc1(4, s.wrapping_add(0x20));
        c.swc1(3, s.wrapping_add(0x14));
        c.swc1(2, s.wrapping_add(0x24));
        let rend = c.lw(gp.wrapping_sub(GP_RENDERER));
        c.g[V1][0] = rend;
        let state = c.lw((rend as u32).wrapping_add(RENDER_STATE));
        c.g[A0][0] = state;
        let blend = c.lw((rend as u32).wrapping_add(BLEND));
        c.g[V0][0] = blend;
        c.sh((state as u32).wrapping_add(0x10), blend);
        let count = c.lw(fx.wrapping_add(COUNT));
        c.g[V1][0] = count;
        c.g[S3][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 21);
            c.mov_s(20, 5);
            c.g[S2][0] = sx(fx.wrapping_add(FLARE_COLOUR));
            c.g[S1][0] = 0;
            c.g[V0][0] = sx(fx);
            loop {
                // pos = centre + ray * dist; colour copy.
                c.cycles += 32;
                let s = sp(c);
                let flare = c.g[V0][0] as u32;
                c.lwc1(3, s.wrapping_add(0x20));
                c.lwc1(4, flare.wrapping_add(FLARE_DIST));
                c.lwc1(1, s.wrapping_add(0x24));
                c.mul_s(3, 3, 4);
                c.lwc1(2, s);
                c.mul_s(1, 1, 4);
                c.lwc1(0, s.wrapping_add(0x4));
                c.add_s(2, 2, 3);
                c.swc1(3, s.wrapping_add(0x40));
                c.add_s(0, 0, 1);
                c.swc1(1, s.wrapping_add(0x44));
                c.swc1(2, s.wrapping_add(0x30));
                c.swc1(0, s.wrapping_add(0x34));
                let colour = c.g[S2][0] as u32;
                crate::game::fe::ovstatehud::copy16_unaligned(
                    c,
                    V0,
                    V1,
                    colour,
                    s.wrapping_add(0x50),
                );
                c.g[A0][0] = 6;
                SUN_FLARE_INTENSITY.call(c, 0x002f_480c)?;

                // colour.r *= intensity * sun flare intensity.
                let s = sp(c);
                c.lwc1(2, fx.wrapping_add(INTENSITY));
                let flare = fx.wrapping_add(c.g[S1][0] as u32);
                c.g[V0][0] = sx(flare);
                c.lwc1(1, s.wrapping_add(0x50));
                c.g[V1][0] = 1;
                c.mul_s(2, 2, 0);
                let ty = c.lw(flare.wrapping_add(FLARE_TYPE));
                c.g[A0][0] = ty;
                c.mul_s(1, 1, 2);
                c.swc1(1, s.wrapping_add(0x50));

                // uv0 (sp+0x40) and uv1 (sp+0x60) by type, staged at
                // sp+0x70; f20 = 0.5, f21 = 1.0.
                let ty = ty as i64;
                let scratch = s.wrapping_add(0x70);
                let (uv0, uv1) = (s.wrapping_add(0x40), s.wrapping_add(0x60));
                if ty == 1 {
                    c.cycles += 14;
                    c.swc1(20, scratch);
                    c.sw(scratch.wrapping_add(4), 0);
                    flare_copy8(c, A1, scratch, uv0);
                    c.swc1(21, scratch);
                    c.swc1(20, scratch.wrapping_add(4));
                    flare_copy8(c, A0, scratch, uv1);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (ty < 2) as u64;
                    c.g[V0][0] = 2;
                    if ty < 2 {
                        c.cycles += 2;
                        c.sw(scratch, 0);
                        if ty == 0 {
                            c.cycles += 13;
                            c.sw(scratch.wrapping_add(4), 0);
                            flare_copy8(c, V0, scratch, uv0);
                            c.swc1(20, scratch);
                            c.swc1(20, scratch.wrapping_add(4));
                            flare_copy8(c, A2, scratch, uv1);
                        } else {
                            // Negative: the default.
                            c.cycles += 2;
                            c.sw(scratch.wrapping_add(4), 0);
                            flare_default_uv(c, scratch, uv0, uv1);
                        }
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 3;
                        if ty == 2 {
                            c.cycles += 14;
                            c.sw(scratch, 0);
                            c.swc1(20, scratch.wrapping_add(4));
                            flare_copy8(c, V1, scratch, uv0);
                            c.swc1(20, scratch);
                            c.swc1(21, scratch.wrapping_add(4));
                            flare_copy8(c, V0, scratch, uv1);
                        } else if ty == 3 {
                            c.cycles += 2;
                            c.swc1(20, scratch);
                            c.cycles += 13;
                            c.swc1(20, scratch.wrapping_add(4));
                            flare_copy8(c, A2, scratch, uv0);
                            c.swc1(21, scratch);
                            c.swc1(21, scratch.wrapping_add(4));
                            flare_copy8(c, A1, scratch, uv1);
                        } else {
                            // Not 3 either (+2), the zero store (+2).
                            c.cycles += 4;
                            c.sw(scratch, 0);
                            c.cycles += 1;
                            c.sw(scratch.wrapping_add(4), 0);
                            flare_default_uv(c, scratch, uv0, uv1);
                        }
                    }
                }
                let flare = fx.wrapping_add(c.g[S1][0] as u32);
                c.g[V0][0] = sx(flare);

                // DrawSprite(R, (pos, 0, 1), (size, size), uv0, uv1, colour).
                c.cycles += 30;
                c.g[AT][0] = 0x4396_0000;
                c.mtc1(AT, 1);
                c.lwc1(0, flare.wrapping_add(FLARE_SIZE));
                c.g[A1][0] = sx(s.wrapping_add(0x80));
                let rend = c.lw(gp.wrapping_sub(GP_RENDERER));
                c.g[V1][0] = rend;
                c.g[A2][0] = sx(scratch);
                c.mul_s(0, 0, 1);
                c.lwc1(2, s.wrapping_add(0x30));
                c.lwc1(1, s.wrapping_add(0x34));
                c.g[A3][0] = sx(uv0);
                c.g[T0][0] = sx(uv1);
                c.g[T1][0] = sx(s.wrapping_add(0x50));
                c.swc1(0, scratch.wrapping_add(4));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.swc1(0, scratch);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(FLARE_STRIDE));
                let vt = c.lw((rend as u32).wrapping_add(VTABLE));
                c.g[V0][0] = vt;
                let slot = crate::call::VSlot::read(c, vt, 0x270);
                c.g[A0][0] = slot.delta;
                c.swc1(2, s.wrapping_add(0x80));
                c.swc1(1, s.wrapping_add(0x84));
                c.g[A0][0] = slot.this(rend);
                c.sw(s.wrapping_add(0x88), 0);
                c.swc1(21, s.wrapping_add(0x8c));
                c.g[V1][0] = slot.func;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(FLARE_STRIDE));
                call_ptr(c, slot.func as u32, 0x002f_49d4)?;
                let count = c.lw(fx.wrapping_add(COUNT));
                let more = (c.g[S3][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                c.g[V0][0] = sx(fx.wrapping_add(c.g[S1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_47b0);
                }
            }
        }

        c.cycles += 9;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 21, 0xe8);
        frame.restore_fpr(c, 20, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// The default flare texture quarter: uv0 (0, 0) (the zeroed scratch),
/// uv1 (0.5, 1).
pub(crate) fn flare_default_uv(c: &mut Cpu, scratch: u32, uv0: u32, uv1: u32) {
    c.cycles += 11;
    flare_copy8(c, A0, scratch, uv0);
    c.swc1(20, scratch);
    c.swc1(21, scratch.wrapping_add(4));
    flare_copy8(c, V1, scratch, uv1);
}

/// `void cDynamicSprayFX_UpdateEmitter9(cDynamicSprayFX* fx)` (0x2e2260):
/// while the spray has a level and the rider is fast enough, spawn one
/// particle at the next bone of a 30-entry ring (the bone's matrix
/// translation), coloured with alpha `clamp(1.5 level, 0, 1)` and thrown
/// along the rider's velocity (`obj + 0x1e0`) times the curve's scale; then
/// fade the level by 1/60 (not below 0) unless the mode is 2. Otherwise slot
/// 9 is only kept alive at the rider's bone `obj + 0x89c`.
pub fn dynamic_spray_fx_update_emitter9(c: &mut Cpu) {
    use spray9::*;
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save_fpr(c, 20, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 21, 0x68);
        let fx = r32(c, S0);
        let gp = r32(c, GP);
        c.mtc1(0, 20); // 0.0
        c.lwc1(2, fx + LEVEL);
        c.g[V0][0] = c.lw(fx + P28);
        c.c_lt_s(20, 2);
        let has_level = c.fcc();
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(SLOT));
        let mut spawn = false;
        if has_level {
            c.cycles += 6;
            c.lwc1(1, gp.wrapping_sub(0x3a4c)); // speed limit
            c.lwc1(0, fx + SPEED);
            c.c_lt_s(1, 0);
            spawn = c.fcc();
            if !spawn {
                // (delay slot of the branch to the idle path)
                c.g[V0][0] = c.lw(fx + OBJ);
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.lw(fx + OBJ);
        }
        if spawn {
            // alpha = clamp(1.5 level, 0, 1) in f3.
            c.cycles += 9;
            c.g[AT][0] = 0x3fc0_0000;
            c.mtc1(AT, 0);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 1);
            c.mul_s(0, 2, 0);
            c.c_le_s(20, 0);
            let positive = c.fcc();
            c.mov_s(3, 20);
            if positive {
                c.cycles += 1;
                c.min_s(3, 0, 1);
            }

            // col (alpha in w) -> sp+0; bone translation -> sp+0x10;
            // velocity * scale -> sp+0x20.
            c.cycles += 36;
            let sp0 = sp(c);
            c.lq(V0, fx + COL);
            c.g[A1][0] = sx(sp0.wrapping_add(0x10));
            c.g[T1][0] = c.lw(fx + RING);
            c.g[A2][0] = sx(sp0.wrapping_add(0x20));
            c.sq(V0, sp0);
            c.g[A3][0] = c.g[SP][0];
            c.swc1(3, sp0.wrapping_add(0xc));
            c.g[T1][0] = sx(r32(c, T1) << 2);
            c.lwc1(21, gp.wrapping_sub(0x3a48)); // 1/60
            c.g[T0][0] = 1;
            c.g[V0][0] = c.lw(fx + BONES);
            c.g[T2][0] = c.lw(fx + OBJ);
            c.mov_s(12, 21);
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
            c.g[T4][0] = c.lw(fx + P2C);
            c.g[T3][0] = c.lw(r32(c, T2).wrapping_add(0x780));
            c.g[V0][0] = c.lw(r32(c, T1));
            c.g[V1][0] = c.lw(r32(c, T3).wrapping_add(0x30));
            c.g[V0][0] = sx(r32(c, V0) << 6);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.lq(T1, r32(c, V1).wrapping_add(0x30));
            c.sq(T1, sp0.wrapping_add(0x10));
            c.lwc1(0, r32(c, T4).wrapping_add(0x900));
            c.mfc1(V1, 0);
            c.lqc2(4, r32(c, T2).wrapping_add(0x1e0));
            c.qmtc2(V1, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp0.wrapping_add(0x30));
            c.lq(V0, sp0.wrapping_add(0x30));
            c.sq(V0, sp0.wrapping_add(0x20));
            call_ptr(c, 0x0037_17c0, 0x002e_2350)?;

            // Advance the ring (wrap at 30).
            c.g[V0][0] = c.lw(fx + RING);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[V1][0] = ((c.g[V0][0] as i64) < 30) as u64;
            let in_range = c.g[V1][0] != 0;
            c.sw(fx + RING, c.g[V0][0]);
            if !in_range {
                c.cycles += 1;
                c.sw(fx + RING, 0);
            }

            // Fade the level unless the mode holds it.
            c.cycles += 4;
            c.g[V1][0] = c.lw(fx + MODE);
            c.g[V0][0] = 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 5;
                c.lwc1(0, fx + LEVEL);
                c.sub_s(0, 0, 21);
                c.max_s(0, 0, 20);
                c.swc1(0, fx + LEVEL);
                c.cycles += 1;
            }
            frame.restore(c, S0, 0x50);
        } else {
            // Idle: keep slot 9 alive at the rider's bone obj+0x89c.
            c.cycles += 13;
            let sp0 = sp(c);
            let obj = r32(c, V0);
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(12, gp.wrapping_sub(0x3a44)); // 1/60
            c.g[A2][0] = 0;
            c.g[T0][0] = c.lw(obj.wrapping_add(0x780));
            c.g[A3][0] = 0;
            c.g[T1][0] = c.lw(obj.wrapping_add(0x89c));
            c.g[V1][0] = c.lw(r32(c, T0).wrapping_add(0x30));
            c.g[T1][0] = sx(r32(c, T1) << 6);
            c.g[T0][0] = 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, T1)));
            c.lq(V0, r32(c, V1).wrapping_add(0x30));
            c.sq(V0, sp0);
            call_ptr(c, 0x0037_17c0, 0x002e_23c4)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cBoardWakeFX_AdvanceRing (0x2de058)

/// cBoardWakeFX: the owner (rider) whose colour source sits at +0x6c0.
pub(crate) const WAKE_OWNER: u32 = 0x00;

/// Live segments in the ring (grows up to the capacity).
pub(crate) const WAKE_COUNT: u32 = 0x04;

/// Ring index of the newest segment.
pub(crate) const WAKE_HEAD_2: u32 = 0x08;

/// The main ring: `cap` entries of 0x30 bytes (`f32 pos; ... int r, g, b`
/// at +0x10/+0x14/+0x18).
pub(crate) const WAKE_RING: u32 = 0x10;

/// Four trail rings (pointers), mirrored from the main ring's head entry.
pub(crate) const WAKE_TRAILS: u32 = 0x14;

/// Animation phase, wrapped into [0, 4).
pub(crate) const WAKE_PHASE_2: u32 = 0x38;

/// Frame counter (an int, converted to float for the segment position).
pub(crate) const WAKE_FRAME: u32 = 0x40;

/// Current wake colour (four floats, clamped to [0, 1]).
pub(crate) const WAKE_COLOUR: u32 = 0xa0;

/// Ring capacity.
pub(crate) const WAKE_CAP: u32 = 0xb8;

/// Bytes per ring entry.
pub(crate) const RING_STRIDE: u32 = 0x30;

/// The colour source object inside the owner (a vtable at its start).
pub(crate) const OWNER_COLOUR_SOURCE: u32 = 0x6c0;

/// Its vtable slot returning the colour index.
pub(crate) const SLOT_COLOUR_INDEX: u32 = 0x38;

/// `.lit4` phase step per tick (1/30), at gp-0x3bdc.
pub(crate) const GP_WAKE_PHASE_STEP: u32 = 0x3bdc;

/// Colour table `D_004FA398`: 0xf0-byte records starting with four floats.
pub(crate) const WAKE_COLOUR_TABLE: u32 = 0x004f_a398;

pub(crate) const WAKE_COLOUR_STRIDE: u32 = 0xf0;

/// Loop head of the trail copy (event checks).
pub(crate) const WAKE_TRAIL_HEAD: u32 = 0x002d_e2c0;

/// `void cBoardWakeFX::AdvanceRing(cBoardWakeFX* fx)` (0x2de058): one tick
/// of a board's wake ribbon. Advances the phase (wrapping at 4), grows the
/// live count up to the capacity, steps the ring head back one slot, fetches
/// the owner's wake colour (a virtual call for the colour index into the
/// colour table), doubles and clamps it to [0, 1], then fills the new head
/// entry: position `frame / 2` folded into [-32, 32) per 64 and the colour
/// as 0..128 ints; copies that entry into the four trail rings and bumps the
/// frame counter.
pub fn board_wake_fx_advance_ring(c: &mut Cpu) {
    guest(c, |c| {
        // Phase += step; wrap at 4.
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let step = Addr(r32(c, GP).wrapping_sub(GP_WAKE_PHASE_STEP)).read_fpu(c);
        set_fpr(c, 1, step);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        set_reg(c, S0, reg(c, A0));
        let fx = r32(c, S0);
        let at = |o: u32| Addr(fx.wrapping_add(o));
        set_reg(c, AT, 0x4080_0000);
        let four = FpuF32(0x4080_0000);
        set_fpr(c, 2, four);
        let mut phase = at(WAKE_PHASE_2).read_fpu(c) + step;
        let wrap = four.le(phase);
        set_fcc(c, wrap);
        at(WAKE_PHASE_2).write_fpu(c, phase);
        if wrap {
            c.cycles += 2;
            phase = phase - four;
            at(WAKE_PHASE_2).write_fpu(c, phase);
        }
        set_fpr(c, 0, phase);

        // count = min(count + 1, cap).
        c.cycles += 5;
        let count = at(WAKE_COUNT).lw(c);
        let cap = at(WAKE_CAP).lw(c);
        set_reg(c, V1, count);
        set_reg(c, V0, addu(count, 1));
        if (count as i64) < (cap as i64) {
            c.cycles += 1;
            at(WAKE_COUNT).sw(c, reg(c, V0));
        }

        // head = (head + cap - 1) % cap.
        c.cycles += 4;
        let cap = at(WAKE_CAP).lw(c);
        let head = at(WAKE_HEAD_2).lw(c);
        set_reg(c, V1, cap);
        c.cycles += 43;
        let n = (head as u32).wrapping_add(cap as u32).wrapping_sub(1);
        set_reg(c, V0, sx(n));
        let owner = at(WAKE_OWNER).lw(c);
        set_reg(c, A1, owner);
        let (_, new_head) = abi::div(c, n, cap as u32);
        let source = (owner as u32).wrapping_add(OWNER_COLOUR_SOURCE);
        set_reg(c, A2, sx(source));
        set_reg(c, A0, sx(new_head));
        at(WAKE_HEAD_2).sw(c, sx(new_head));

        // The owner's colour index (virtual call).
        let vt = Addr(source).lw(c);
        set_reg(c, V1, vt);
        let slot = VSlot::read(c, vt, SLOT_COLOUR_INDEX);
        set_reg(c, V0, slot.func);
        set_reg(c, A0, slot.this(sx(source)));
        call_ptr(c, slot.func as u32, 0x002d_e0ec)?;

        // Copy the table colour to the stack and double it.
        let index = r32(c, V0);
        set_reg(c, A0, WAKE_COLOUR_STRIDE as u64);
        set_reg(c, V1, 0x50_0000);
        let off = abi::mult(c, index, WAKE_COLOUR_STRIDE);
        set_reg(c, V0, sx(off));
        set_reg(c, V1, sx(WAKE_COLOUR_TABLE));
        let zero = FpuF32::ZERO;
        let one = FpuF32::ONE;
        set_fpr(c, 4, zero);
        set_reg(c, AT, 0x3f80_0000);
        set_fpr(c, 5, one);
        set_fpr(c, 6, zero);
        let entry = off.wrapping_add(WAKE_COLOUR_TABLE);
        set_reg(c, V0, sx(entry));
        c.ldl(V1, entry.wrapping_add(7));
        c.ldr(V1, entry);
        c.ldl(A0, entry.wrapping_add(0xf));
        c.ldr(A0, entry.wrapping_add(8));
        let s = sp(c);
        c.sdl(V1, s.wrapping_add(7));
        c.sdr(V1, s);
        c.sdl(A0, s.wrapping_add(0xf));
        c.sdr(A0, s.wrapping_add(8));
        let tmp = |o: u32| Addr(s.wrapping_add(o));
        let y = tmp(4).read_fpu(c);
        let x = tmp(0).read_fpu(c);
        let y2 = y + y;
        let z = tmp(8).read_fpu(c);
        let w = tmp(0xc).read_fpu(c);
        let x2 = x + x;
        let z2 = z + z;
        let w2 = w + w;
        tmp(4).write_fpu(c, y2);
        // Clamp each lane to [0, 1] (negative -> 0).
        let pos = zero.le(y2);
        set_fcc(c, pos);
        tmp(0).write_fpu(c, x2);
        tmp(8).write_fpu(c, z2);
        tmp(0xc).write_fpu(c, w2);
        let mut cy = zero;
        if pos {
            c.cycles += 1;
            cy = y2.min(one);
        }
        c.cycles += 4;
        let pos = zero.le(z2);
        set_fcc(c, pos);
        tmp(4).write_fpu(c, cy);
        let mut cz = zero;
        if pos {
            c.cycles += 1;
            cz = z2.min(one);
        }
        c.cycles += 4;
        let pos = zero.le(w2);
        set_fcc(c, pos);
        tmp(8).write_fpu(c, cz);
        let mut cw = zero;
        if pos {
            c.cycles += 1;
            cw = w2.min(one);
        }
        c.cycles += 4;
        let pos = zero.le(x2);
        set_fcc(c, pos);
        tmp(0xc).write_fpu(c, cw);
        let mut cx = zero;
        if pos {
            c.cycles += 1;
            cx = x2.min(one);
        }

        // Store the colour in the effect.
        c.cycles += 70;
        tmp(0).write_fpu(c, cx);
        set_reg(c, A0, RING_STRIDE as u64);
        set_reg(c, AT, 0x3f00_0000);
        let half = FpuF32(0x3f00_0000);
        set_reg(c, A3, RING_STRIDE as u64);
        c.ldl(V0, s.wrapping_add(7));
        c.ldr(V0, s);
        c.ldl(V1, s.wrapping_add(0xf));
        c.ldr(V1, s.wrapping_add(8));
        let col = fx.wrapping_add(WAKE_COLOUR);
        c.sdl(V0, col.wrapping_add(7));
        c.sdr(V0, col);
        c.sdl(V1, col.wrapping_add(0xf));
        c.sdr(V1, col.wrapping_add(8));
        set_reg(c, T0, 3);
        set_reg(c, AT, 0x3c80_0000);
        let sixty_fourth = FpuF32(0x3c80_0000);
        let trails = fx.wrapping_add(WAKE_TRAILS);
        set_reg(c, A2, sx(trails));

        // Head entry position: frame / 2, folded per 64 into [-32, 32).
        let head = at(WAKE_HEAD_2).lw(c);
        set_reg(c, V0, head);
        let frame_no = FpuF32::from_i32(at(WAKE_FRAME).read_u32(c) as i32);
        let ring = at(WAKE_RING).lw(c);
        set_reg(c, V1, ring);
        let off = abi::mult(c, head as u32, RING_STRIDE);
        set_reg(c, AT, 0x4280_0000);
        let sixty_four = FpuF32(0x4280_0000);
        let pos0 = frame_no * half;
        set_reg(c, AT, 0x4200_0000);
        let thirty_two = FpuF32(0x4200_0000);
        set_reg(c, AT, 0x4300_0000);
        let one_two_eight = FpuF32(0x4300_0000);
        let e = off.wrapping_add(ring as u32);
        set_reg(c, V0, sx(e));
        Addr(e).write_fpu(c, pos0);
        let head = at(WAKE_HEAD_2).lw(c);
        let ring = at(WAKE_RING).lw(c);
        set_reg(c, V0, ring);
        let off = abi::mult(c, head as u32, RING_STRIDE);
        let e = off.wrapping_add(ring as u32);
        set_reg(c, V1, sx(e));
        let pos = Addr(e).read_fpu(c);
        let scaled = pos * sixty_fourth;
        set_fpr(c, 2, scaled);
        let whole = scaled.to_i32();
        set_reg(c, V0, sx(whole as u32));
        let fold = FpuF32::from_i32(whole) * sixty_four + thirty_two;
        let pos = pos - fold;
        Addr(e).write_fpu(c, pos);

        // Colour channels as 0..128 ints at +0x10/+0x14/+0x18 (the head and
        // ring registers alternate between v0 and v1).
        let mut last = zero;
        for (k, (src, dst)) in [(0xa4u32, 0x10u32), (0xa8, 0x14), (0xac, 0x18)]
            .into_iter()
            .enumerate()
        {
            let (hreg, rreg) = if k == 1 { (V1, V0) } else { (V0, V1) };
            let head = at(WAKE_HEAD_2).lw(c);
            let chan = at(src).read_fpu(c);
            let off = abi::mult(c, head as u32, RING_STRIDE);
            let ring = at(WAKE_RING).lw(c);
            set_reg(c, rreg, ring);
            let v = chan * one_two_eight;
            let e = off.wrapping_add(ring as u32);
            set_reg(c, hreg, sx(e));
            Addr(e.wrapping_add(dst)).write_u32(c, v.to_i32() as u32);
            last = v;
        }
        set_fpr(c, 0, last);
        set_fpr(c, 1, FpuF32(last.to_i32() as u32));
        set_fpr(c, 3, one_two_eight);
        set_fpr(c, 4, thirty_two);
        set_fpr(c, 5, sixty_four);
        set_fpr(c, 6, cy);

        // Mirror the head entry into the four trail rings.
        let mut trail_at = trails;
        let mut left: u32 = 3;
        loop {
            c.cycles += 35;
            let head = at(WAKE_HEAD_2).lw(c);
            set_reg(c, V0, head);
            left = left.wrapping_sub(1);
            set_reg(c, T0, sx(left));
            let ring = at(WAKE_RING).lw(c);
            set_reg(c, V1, ring);
            let off = abi::mult(c, head as u32, RING_STRIDE);
            set_reg(c, V0, sx(off));
            let trail = Addr(trail_at).lw(c);
            set_reg(c, A0, trail);
            let src = off.wrapping_add(ring as u32);
            set_reg(c, V1, sx(src));
            let p = Addr(src).read_fpu(c);
            set_fpr(c, 0, p);
            let dst = off.wrapping_add(trail as u32);
            set_reg(c, V0, sx(dst));
            Addr(dst).write_fpu(c, p);
            // r, g, b: registers rotate (head, ring, off, src, dst).
            for (field, h, rg, sr, ds) in [
                (0x10u32, V1, A0, A0, V1),
                (0x14, A0, V1, V1, A0),
                (0x18, V1, A0, A0, V1),
            ] {
                let head = at(WAKE_HEAD_2).lw(c);
                set_reg(c, h, head);
                let ring = at(WAKE_RING).lw(c);
                set_reg(c, rg, ring);
                let off = abi::mult(c, head as u32, RING_STRIDE);
                set_reg(c, h, sx(off));
                let trail = Addr(trail_at).lw(c);
                set_reg(c, A1, trail);
                if field == 0x18 {
                    trail_at = trail_at.wrapping_add(4);
                    set_reg(c, A2, sx(trail_at));
                }
                let src = off.wrapping_add(ring as u32);
                set_reg(c, sr, sx(src));
                let v = Addr(src.wrapping_add(field)).lw(c);
                set_reg(c, V0, v);
                let dst = off.wrapping_add(trail as u32);
                set_reg(c, ds, sx(dst));
                Addr(dst.wrapping_add(field)).sw(c, v);
            }
            if (left as i32) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, WAKE_TRAIL_HEAD);
            }
        }

        // frame++.
        c.cycles += 7;
        let n = at(WAKE_FRAME).lw(c);
        frame.restore_ra(c, 0x10);
        set_reg(c, V0, addu(n, 1));
        at(WAKE_FRAME).sw(c, reg(c, V0));
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

// -------------------------------------------------------- LensFX_DrawFlares

/// `void LensFX_DrawFlares(LensFX* fx, int view)` (0x2e3478): draw the sun
/// of this view (when the sun is enabled, `gp+0xa28`), then, between the
/// lens FX's push / pop render-state methods (vtable at +0x649c, slots
/// +0x10 and +0x18), every flare quad of the view whose "on" flag is set.
/// Per view: a flare count at `+0x60 + 4*view`, `0x3000` bytes of flares
/// (`0x30` each, the "on" halfword at flare +0x2c) from +0x68, and a sun
/// record of `0x210` bytes at +0x6070.
pub fn lens_fx_draw_flares(c: &mut Cpu) {
    const DRAW_SUN: Callee = Callee {
        addr: 0x002f_4a08,
        f: ee::s09::LensFX_DrawSun,
    };
    const DRAW_FLARE_QUAD: Callee = Callee {
        addr: 0x002e_2868,
        f: ee::s09::LensFX_DrawFlareQuad,
    };
    /// `gp+0xa28`: the sun is enabled.
    const SUN_ENABLED_GP: u32 = 0xa28;
    /// `LensFX` layout.
    const COUNTS: u32 = 0x60;
    const FLARES: u32 = 0x68;
    const FLARES_PER_VIEW: u32 = 0x3000;
    const FLARE_SIZE: u32 = 0x30;
    /// The flare's "on" halfword (`FLARES + 0x2c`), relative to the lens FX.
    const FLARE_ON: u32 = 0x94;
    const SUNS: u32 = 0x6070;
    const SUN_SIZE: u32 = 0x210;
    const VTABLE: u32 = 0x649c;
    /// Vtable slots `{delta: i16, method}`: push / pop render state.
    const PUSH_STATE: u32 = 0x10;
    const POP_STATE: u32 = 0x18;
    /// Loop head (event checks).
    const LOOP: u32 = 0x002e_3510;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let sun_on = Addr(r32(c, GP).wrapping_add(SUN_ENABLED_GP)).lw(c);
        c.g[V0][0] = sun_on;
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        let fx = r32(c, A0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        let view = r32(c, A1);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        if sun_on != 0 {
            c.cycles += 7;
            c.g[V0][0] = sx(fx.wrapping_add(SUNS));
            let off = abi::mult(c, view, SUN_SIZE);
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S4][0];
            c.g[A0][0] = sx(fx.wrapping_add(SUNS).wrapping_add(off));
            DRAW_SUN.call(c, 0x002e_34c4)?;
        }

        // Push the render state.
        c.cycles += 12;
        let vt = Addr(fx).off(VTABLE).lw(c);
        c.g[V0][0] = vt;
        c.g[S3][0] = 0;
        let delta = Addr(vt as u32).off(PUSH_STATE).lh(c);
        let method = Addr(vt as u32).off(PUSH_STATE + 4).lw(c);
        c.g[V1][0] = method;
        c.g[A0][0] = abi::addu(c.g[S2][0], delta);
        call_ptr(c, method as u32, 0x002e_34dc)?;
        c.g[A0][0] = sx(view << 2);
        c.g[V0][0] = sx(fx.wrapping_add(COUNTS));
        let count_at = fx.wrapping_add(COUNTS).wrapping_add(view << 2);
        c.g[A1][0] = sx(count_at);
        let count = Addr(count_at).lw(c);
        c.g[V1][0] = count;
        c.g[V0][0] = FLARES_PER_VIEW as u64;
        if (count as i64) > 0 {
            c.cycles += 7;
            c.g[A0][0] = sx(fx.wrapping_add(FLARES));
            let block = abi::mult(c, view, FLARES_PER_VIEW);
            c.g[V0][0] = sx(block);
            c.g[S5][0] = c.g[A1][0];
            c.g[V1][0] = sx(block.wrapping_add(FLARE_ON));
            c.g[S1][0] = sx(fx.wrapping_add(FLARES).wrapping_add(block));
            c.g[S0][0] = sx(block.wrapping_add(FLARE_ON).wrapping_add(fx));
            loop {
                c.cycles += 3;
                let on = Addr(r32(c, S0)).lhu(c);
                c.g[V0][0] = on;
                c.g[A0][0] = c.g[S1][0];
                if on != 0 {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S4][0];
                    DRAW_FLARE_QUAD.call(c, 0x002e_3524)?;
                }
                c.cycles += 6;
                let count = Addr(r32(c, S5)).lw(c);
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(FLARE_SIZE));
                let more = (c.g[S3][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(FLARE_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOOP);
                }
            }
        }

        // Pop the render state.
        c.cycles += 14;
        let vt = Addr(fx).off(VTABLE).lw(c);
        c.g[V0][0] = vt;
        let delta = Addr(vt as u32).off(POP_STATE).lh(c);
        let method = Addr(vt as u32).off(POP_STATE + 4).lw(c);
        c.g[V1][0] = method;
        c.g[A0][0] = abi::addu(c.g[S2][0], delta);
        call_ptr(c, method as u32, 0x002e_3550)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void LensFX_PrepareView(LensFX* fx, int view)` (0x2e3338): per view,
/// before the flares are drawn: copy the camera's view rect, view matrix
/// (VU0 quadword copy) and FOV values into the lens FX, update the sun
/// position for this view, then project every flare source of the view.
pub fn lens_fx_prepare_view(c: &mut Cpu) {
    use crate::call::VSlot;
    use lens_prep::*;
    guest(c, |c| {
        c.cycles += 59;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A0][0]; // fx
        frame.save(c, S2, 0x30);
        c.g[S3][0] = c.g[A1][0]; // view
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        let gp = r32(c, GP);
        c.g[S0][0] = Addr(gp.wrapping_add(CAMERA_GP)).lw(c);
        let cam = c.g[S0][0];

        // The view rect, returned into sp+0 and copied (unaligned) to fx+0.
        c.g[V0][0] = Addr((cam as u32).wrapping_add(CAM_VT)).lw(c);
        let vt = c.g[V0][0];
        let slot = VSlot::read(c, vt, VT_VIEW_RECT);
        c.g[V1][0] = slot.func;
        c.g[A1][0] = slot.this(cam);
        call_ptr(c, slot.func as u32, 0x002e_3374)?;
        let (stack, fx) = (sp(c), r32(c, S1));
        c.ldl(V0, stack.wrapping_add(0x7));
        c.ldr(V0, stack);
        c.ldl(V1, stack.wrapping_add(0xf));
        c.ldr(V1, stack.wrapping_add(0x8));
        c.sdl(V0, fx.wrapping_add(0x7));
        c.sdr(V0, fx);
        c.sdl(V1, fx.wrapping_add(0xf));
        c.sdr(V1, fx.wrapping_add(0x8));

        // The view matrix.
        c.g[V1][0] = Addr((cam as u32).wrapping_add(CAM_VT)).lw(c);
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, VT_VIEW_MATRIX);
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(cam);
        call_ptr(c, slot.func as u32, 0x002e_33a8)?;
        let src = r32(c, V0);
        let dst = fx.wrapping_add(MATRIX);
        c.g[V1][0] = sx(dst);
        for row in 0..4u32 {
            c.lqc2(1 + row as usize, src.wrapping_add(0x10 * row));
        }
        for row in 0..4u32 {
            c.sqc2(1 + row as usize, dst.wrapping_add(0x10 * row));
        }

        // The FOV values.
        c.g[V1][0] = Addr((cam as u32).wrapping_add(CAM_VT)).lw(c);
        c.g[A3][0] = sx(fx.wrapping_add(FOV_Z));
        c.g[A1][0] = sx(fx.wrapping_add(FOV_X));
        c.g[A2][0] = sx(fx.wrapping_add(FOV_Y));
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, VT_FOV);
        c.g[S2][0] = 0;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(cam);
        call_ptr(c, slot.func as u32, 0x002e_33f0)?;

        // The sun of this view.
        c.g[A0][0] = SUN_SIZE;
        c.g[V0][0] = sx(fx.wrapping_add(SUN));
        c.mult(0, A0, S3, A0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        UPDATE_SUN_POSITION.call(c, 0x002e_340c)?;

        // Project the view's flare sources.
        c.g[A0][0] = sx(r32(c, S3) << 2);
        c.g[V0][0] = sx(fx.wrapping_add(COUNTS));
        c.g[A3][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[V1][0] = Addr(r32(c, A3)).lw(c);
        c.g[V0][0] = FLARE_VIEW_SIZE;
        if (c.g[V1][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V1][0] = sx(fx.wrapping_add(FLARES));
            c.mult(0, V0, S3, V0);
            c.g[S3][0] = c.g[A3][0]; // &counts[view]
            c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                PROJECT_SOURCE.call(c, 0x002e_3444)?;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = Addr(r32(c, S3)).lw(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(FLARE_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_3438);
                }
            }
        }

        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}




// ---------------------------------------------------------------------------
// cDynamicSprayFX_QueueBurst (0x2e23e0)

/// The spray effect: its rider (+0), the queued burst's strength (+0xe0),
/// the queued position (+0xf0) and direction (+0x100), the burst's two
/// parameters (+0x110 / +0x114), the "short range" flag (+0x124).
pub(crate) const SPRAY_RIDER: u32 = 0x0;


pub(crate) const SPRAY_STRENGTH: u32 = 0xe0;


pub(crate) const SPRAY_POS: u32 = 0xf0;


pub(crate) const SPRAY_DIR: u32 = 0x100;


pub(crate) const SPRAY_PARAM_A: u32 = 0x110;


pub(crate) const SPRAY_PARAM_B: u32 = 0x114;


pub(crate) const SPRAY_SHORT_RANGE: u32 = 0x124;



/// The rider fields read for "carving": +0xac4 set, +0xad0 clear, +0xafc
/// set, then +0xb00 set.
pub(crate) const RIDER_CARVE_A: u32 = 0xac4;


pub(crate) const RIDER_CARVE_BLOCK: u32 = 0xad0;


pub(crate) const RIDER_CARVE_B: u32 = 0xafc;


pub(crate) const RIDER_CARVE_C: u32 = 0xb00;



/// `void cDynamicSprayFX_QueueBurst(cDynamicSprayFX* self /*a0*/,
/// sV4* pos /*a1*/, sV4* dir /*a2*/, int a /*a3*/, int b /*t0*/,
/// float speed /*f12*/)` (0x2e23e0): queue a snow-spray burst. A stronger
/// burst (|speed| above the queued strength) replaces the queued one when it
/// is far enough from the queued position (180, or 100 with the short-range
/// flag) or when the rider is carving; its strength is |speed| (doubled when
/// `b` is 0). Leaf, frame 0x20 (a scratch quadword for the distance).
pub fn dynamic_spray_queue_burst(c: &mut Cpu) {
    c.cycles += 24;
    let this = r32(c, A0);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    let scratch = sp(c);
    // f1 = |speed|; distance = |queued pos - pos| through VU0.
    c.abs_s(1, 12);
    c.lqc2(3, this.wrapping_add(SPRAY_POS));
    c.lqc2(4, r32(c, A1));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, scratch.wrapping_add(0x10));
    c.g[AT][0] = 0x4334_0000; // 180.0
    c.mtc1(1, 3);
    c.lq(2, scratch.wrapping_add(0x10));
    c.g[V1][0] = c.lw(this.wrapping_add(SPRAY_SHORT_RANGE));
    c.sq(2, scratch);
    c.lqc2(3, scratch);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, 0);
    c.abs_s(2, 0); // f2 = distance
    c.lwc1(0, this.wrapping_add(SPRAY_STRENGTH));
    if c.g[V1][0] != 0 {
        c.cycles += 2;
        c.g[AT][0] = 0x42c8_0000; // 100.0
        c.mtc1(1, 3);
    }
    // f3 = range
    c.cycles += 3;
    c.c_lt_s(0, 1);
    if c.fcc() {
        c.cycles += 4;
        c.c_lt_s(3, 2);
        let far = c.fcc();
        c.g[T2][0] = 0;
        let mut queue = far;
        if !far {
            // Carving test on the rider.
            c.cycles += 4;
            c.g[V1][0] = c.lw(this.wrapping_add(SPRAY_RIDER));
            let rider = r32(c, V1);
            c.g[V0][0] = c.lw(rider.wrapping_add(RIDER_CARVE_A));
            c.g[T1][0] = c.g[V1][0];
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(rider.wrapping_add(RIDER_CARVE_BLOCK));
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(rider.wrapping_add(RIDER_CARVE_B));
                    c.g[T2][0] = (0 < c.g[V0][0]) as u64;
                }
            }
            c.cycles += 2;
            if c.g[T2][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(rider.wrapping_add(RIDER_CARVE_C));
                queue = c.g[V0][0] != 0;
            }
        }
        if queue {
            c.cycles += 2;
            if c.g[T0][0] != 0 {
                c.swc1(1, this.wrapping_add(SPRAY_STRENGTH));
            } else {
                c.cycles += 2;
                c.add_s(0, 1, 1);
                c.swc1(0, this.wrapping_add(SPRAY_STRENGTH));
            }
            c.cycles += 6;
            c.lq(2, r32(c, A1));
            c.sq(2, this.wrapping_add(SPRAY_POS));
            c.lq(3, r32(c, A2));
            c.sw(this.wrapping_add(SPRAY_PARAM_B), c.g[T0][0]);
            c.sq(3, this.wrapping_add(SPRAY_DIR));
            c.sw(this.wrapping_add(SPRAY_PARAM_A), c.g[A3][0]);
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    ret(c);
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
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::ai::ridermetrix::tests::BELIB;
    pub(crate) use crate::game::ai::ridermetrix::tests::M;
    pub(crate) use crate::game::be::beintplayer::tests::fillb;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals as game_globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_4 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_7 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_4 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W_2;
    pub(crate) use crate::game::fe::festateoptions::tests::rng;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::object::effectlink::tests::alloc;
    pub(crate) use crate::game::replay::replay2::tests::dtor_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_8;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_6;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::util::menu::tests::wf_2 as wf;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn c_transition_man_set_paused_matches() {
        check(
            ssx3_ee::s09::f_002e4638,
            c_transition_man_set_paused,
            300,
            &|c, seed| {
                let mut r = Rng(0x1006 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    ((r.next_u32() as u64) << 32) | 1
                };
                c.bus
                    .write32(DATA + 0x44, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus
                    .write32(DATA + 0x40, if seed % 5 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn c_transition_man_get_time_remaining_matches() {
        check(
            ssx3_ee::s09::f_002e4678,
            c_transition_man_get_time_remaining,
            200,
            &|c, seed| {
                let mut r = Rng(0x1007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 0x44, if seed % 3 == 0 { 0 } else { r.next_u32() });
                for off in [0x48u32, 0x54, 0x58, 0x5c] {
                    c.bus.write32(DATA + off, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn transition_effect_ctor_matches() {
        check(
            ssx3_ee::s09::f_002e4d70,
            transition_effect_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x1008 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
            },
        );
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
        check(
            ssx3_ee::s09::f_002e6640,
            c_speed_boost_streamer_fx_reset,
            150,
            &|c, seed| {
                let mut r = Rng(0x100a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(
                    0x870,
                    if seed % 2 == 0 {
                        r.next_u32()
                    } else {
                        r.next_u32() | 0x8000_0000
                    },
                );
            },
        );
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
        c.bus
            .write32(gfx + GS_TOP, stack + RECORD * (r.next_u32() % 4));
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
        covered(
            ssx3_ee::s09::f_002e3af8__generated,
            c_player_pulse_fx_render,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e3a_f8, 4);
                gfx_fixture(c, &mut r);
                let (fx, rider, rider_vt) = (DATA, DATA + 0x100, DATA + 0x2400);
                let (app, module, ai, players, mode) = (
                    DATA + 0x2500,
                    DATA + 0x2600,
                    DATA + 0x2700,
                    DATA + 0x2800,
                    DATA + 0x2900,
                );
                let (iface, iface_vt, lib) = (DATA + 0x2a00, DATA + 0x2b00, DATA + 0x2c00);
                fill(c, &mut r, fx, 0x10);
                c.g[A0][0] = sx(fx);
                c.bus.write32(fx, rider);
                // The rider: "local player" query (0 or a fake), switches, bone.
                c.bus.write32(rider + 0x6c0, rider_vt);
                let query = if r.next_u32() % 2 == 0 {
                    RETURN_ZERO
                } else {
                    FAKE + 0x20
                };
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
            },
        );
    }

    /// The PS2 graphics manager `BeginTriStrip`/`EndTriStrip` run on (real
    /// callees): GS state, view matrices up to date (+0x6b90), the packet
    /// source (+0x5a00), the strip table (+0x18f0, full or nearly empty),
    /// and the scratchpad stream at gp+0x1384 (two slots, packets in RAM).
    pub(crate) fn graphics_man_fixture(c: &mut Cpu, r: &mut Rng) {
        let (gm, stack, table, stream) = (
            DATA + 0x2_0000,
            DATA + 0x7000,
            DATA + 0x4_0000,
            DATA + 0x4_8000,
        );
        c.bus.write32(GAME_GP + GRAPHICS_MAN, gm);
        c.bus.write32(gm + GS_TOP, stack);
        fill(c, r, stack, 8);
        c.bus.write32(gm + 0x6b90, 1);
        c.bus.write32(gm + 0x5a00, DATA + 0x3_0000);
        c.bus.write32(gm + 0x18f0, table);
        c.bus.write32(
            table,
            if r.next_u32() % 2 == 0 {
                3000
            } else {
                r.next_u32() % 4
            },
        );
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
        covered(
            ssx3_ee::s09::f_002e7a10__generated,
            c_speed_boost_streamer_fx_render,
            SEEDS,
            &|c, seed| {
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
                let count = if seed % 6 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 21
                };
                c.bus.write32(fx + boost::COUNT, count);
                let age = if seed % 7 == 1 {
                    1.0f32
                } else {
                    3.0 + (r.next_u32() % 8) as f32
                };
                c.bus.write32(fx + boost::FADE, age.to_bits());
                c.bus.write32(
                    fx + boost::FLAME_CLAMP,
                    (0.5 + (r.next_u32() % 8) as f32).to_bits(),
                );
                c.bus.write32(fx + boost::ALPHA, r.f32(1.0));
                c.bus.write32(GAME_GP - 0x39d8, (-0.5f32).to_bits());
                c.bus.write32(GAME_GP + 0x1634, (seed % 5 != 2) as u32);
                c.bus.write32(fx + boost::TEXTURE, r.next_u32() % 8);
                c.bus.write32(fx + boost::FIRST, r.next_u32() % 0x14);
                c.bus.write32(fx + boost::POINTS, points);
                fill(c, &mut r, points, 0x14 * 8);
                let flames = if seed % 4 == 0 {
                    r.next_u32() % 2
                } else {
                    2 + r.next_u32() % 14
                };
                c.bus.write32(fx + boost::FLAMES, flames);
                c.bus.write32(fx + boost::FLAME_HEAD, r.next_u32() % 0x1e);
                streamer_rider(c, &mut r, rider, 0x89c, 0x8a4);
                fill(c, &mut r, rider + 0x1a0, 4);
                c.bus.write32(GAME_GP - 0x39d4, r.f32(8.0));
            },
        );
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
        check(
            ee::s09::f_002e5430__generated,
            snow_box_wrap_offset,
            SEEDS_2,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn dynamic_spray_fx_add_random_velocity_matches() {
        check(
            ee::s09::f_002de398__generated,
            dynamic_spray_fx_add_random_velocity,
            SEEDS_2,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn board_wake_fx_render_matches() {
        check(
            ee::s09::f_002ddab8__generated,
            board_wake_fx_render,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2dda, 1);
                let (w, gfx, state, vt, verts) = (
                    DATA,
                    DATA + 0x1000,
                    DATA + 0x2000,
                    DATA + 0x3000,
                    DATA + 0x4000,
                );
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
                c.bus
                    .write32(w + 0xbc, f(0.5 + (r.next_u32() % 100) as f32 * 0.1));
                for k in 0..4u32 {
                    c.bus.write32(
                        verts + 0x70 * k + 0x64,
                        f((r.next_u32() % 100) as f32 * 0.01),
                    );
                }
                // Cover the early-outs too.
                if seed % 11 == 0 {
                    c.bus.write32(w + 0x4, 0);
                }
                c.bus
                    .write32(gp_at(0x1438), if seed % 13 == 0 { 1 } else { 0 });
                c.g[A0][0] = sx(w);
            },
        );
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
        check(
            ee::s09::f_002e0ee8__generated,
            dynamic_spray_fx_update_emitter0,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e0e, 1);
                let (fx, obj, vt, curves, cfg, em) = (
                    DATA,
                    DATA + 0x4000,
                    DATA + 0x4300,
                    DATA + 0x4400,
                    DATA + 0x4500,
                    DATA + 0x5000,
                );
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
                c.bus
                    .write32(fx + 0xbc, f(0.2 + (r.next_u32() % 160) as f32 * 0.01));
                c.bus.write32(fx + 0xd0, cfg);
                c.bus.write32(cfg + 0x58, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(obj + 0x88c, obj + 0x4200);
                c.bus
                    .write32(obj + 0x4200 + 0xa0, if seed % 5 == 0 { 1 } else { 0 });
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
            },
        );
    }

    #[test]
    fn snow_box_render_matches() {
        check(
            ee::s09::f_002e55d8__generated,
            snow_box_render,
            SEEDS_2,
            &|c, seed| {
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
            },
        );
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
        check(
            ee::s09::f_002e1598__generated,
            dynamic_spray_fx_update_emitter5,
            SEEDS_2,
            &|c, seed| {
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
                c.bus.write32(
                    fx + spray5::LEVEL,
                    f([0.5f32, 1.2, 2.5][(seed % 3) as usize]),
                );
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
                c.bus
                    .write32(cfg + spray_cfg::ENABLED, if seed % 9 == 0 { 0 } else { 1 });
                c.bus
                    .write32(cfg + spray_cfg::FLAG, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(
                    cfg + spray_cfg::DECAY,
                    f(0.5 + (r.next_u32() % 50) as f32 * 0.01),
                );
                c.bus
                    .write32(fx + spray5::ID4, [0u32, 2, 3][(seed % 3) as usize]);
                c.bus.write32(
                    fx + spray5::SPEED,
                    f([5.0f32, 100.0, 500.0][(seed % 3) as usize]),
                );
                c.bus.write32(fx + spray5::I114, (seed % 2) as u32);
                c.bus
                    .write32(fx + spray5::ALPHA, f([0.1f32, 0.75][(seed % 2) as usize]));
                c.bus.write32(fx + spray5::I124, ((seed / 3) % 2) as u32);
                c.bus
                    .write32(obj + 0xac4, if seed % 7 == 0 { 0 } else { 1 });
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
                    (-0x3abc, 27.777778f32),
                    (-0x3ab8, 277.77777),
                    (-0x3ab4, 3333.3335),
                    (-0x3ab0, 0.3),
                    (-0x3aac, 0.4),
                    (-0x3aa8, 0.8),
                    (-0x3aa4, 0.2),
                    (-0x3aa0, 1.0 / 60.0),
                    (-0x3a9c, 83.333336),
                    (-0x3a98, 0.4),
                ] {
                    c.bus.write32(gp_at(off), f(v));
                }
                c.g[A0][0] = sx(fx);
            },
        );
    }

    /// The spray's per-tick quantities all follow the rate: the level rise
    /// (`+0x4`), the alpha blend (`+0x120`) and the strength decay (`+0xe0`)
    /// reach the same values after the same wall-clock time, so 5 updates at
    /// 60 Hz and 10 at 120 Hz agree.
    #[test]
    fn emitter5_fades_follow_the_rate() {
        /// `(level, alpha, speed)` after `ticks` updates at rate `hz`.
        fn fades_after(hz: u32, ticks: u32, decay: f32) -> (f32, f32, f32) {
            let mut c = crate::difftest::fresh();
            elf_image(&mut c);
            let mut r = machine(&mut c, 5, 0x2e15, 0);
            crate::tick::set_for_test(&mut c, hz);
            // The `1/60` entries this update loads.
            for off in [-0x3aa0, -0x3a94] {
                c.bus.write32(gp_at(off), crate::tick::step_bits(hz));
            }
            for (off, v) in [
                (-0x3abc, 27.777778f32),
                (-0x3ab8, 277.77777),
                (-0x3ab4, 3333.3335),
                (-0x3ab0, 0.3),
                (-0x3aac, 0.4),
                (-0x3aa8, 0.8),
                (-0x3aa4, 0.2),
                (-0x3a9c, 83.333336),
                (-0x3a98, 0.4),
            ] {
                c.bus.write32(gp_at(off), f(v));
            }
            let (fx, obj, cfg, em, vt) = (
                DATA,
                DATA + 0x5000,
                DATA + 0x5200,
                DATA + 0x3000,
                DATA + 0x5400,
            );
            c.bus.write32(fx + spray5::OBJ, obj);
            c.bus.write32(fx + spray5::LEVEL, f(0.5));
            c.bus.write32(fx + spray5::P28, em);
            c.bus.write32(fx + spray5::P2C, DATA + 0x5600);
            for k in 0..3u32 {
                c.bus.write32(DATA + 0x5600 + 0xd8 + 4 * k, f(1.0));
            }
            for off in [
                spray5::DIR,
                spray5::COL,
                spray5::POS,
                spray5::FRAME_POS,
                spray5::THROW,
            ] {
                put_f32s(&mut c, fx + off, [0.0, 0.0, 0.0, 0.0]);
            }
            put_f32s(&mut c, obj + 0x370, [0.0, 0.0, 1.0, 0.0]);
            c.bus.write32(fx + spray5::CFG, cfg);
            c.bus.write32(cfg + spray_cfg::ENABLED, 1);
            c.bus.write32(cfg + spray_cfg::FLAG, 1); // short range: slot 5
            c.bus.write32(cfg + spray_cfg::DECAY, f(decay));
            c.bus.write32(fx + spray5::ID4, 2); // the 2.0 level-rate branch
            c.bus.write32(fx + spray5::SPEED, f(333.3333)); // r = 0.1, > 277.78
            c.bus.write32(fx + spray5::I114, 1); // alpha target 0.5
            c.bus.write32(fx + spray5::ALPHA, f(0.9));
            c.bus.write32(fx + spray5::I124, 1); // no extra 0.4 on the decay
            c.bus.write32(obj + 0xac4, 1); // no splash spawn
            c.bus.write32(obj + 0x6c0, vt);
            c.bus.write16(vt + 0x10, 0);
            c.bus.write32(vt + 0x14, FAKE);
            put_spray_slot(&mut c, &mut r, em + 5 * 0x210, DATA + 0x8000);
            put_spray_slot(&mut c, &mut r, em + 6 * 0x210, DATA + 0x8100);
            c.bus.write32(gp_at(0xa0c), 0x3f80_0000);
            for _ in 0..ticks {
                c.g[A0][0] = fx as u64;
                dynamic_spray_fx_update_emitter5(&mut c);
            }
            (
                f32::from_bits(c.bus.read32(fx + spray5::LEVEL)),
                f32::from_bits(c.bus.read32(fx + spray5::ALPHA)),
                f32::from_bits(c.bus.read32(fx + spray5::SPEED)),
            )
        }

        // A steady strength (decay 1.0) isolates the level rise and the alpha
        // blend: 0.5 + 5 * 0.2 = 1.5, and 0.5 + 0.4 * 0.8^5 = 0.631072.
        let (l60, a60, _) = fades_after(60, 5, 1.0);
        let (l120, a120, _) = fades_after(120, 10, 1.0);
        assert!((l60 - 1.5).abs() < 1e-4, "level at 60: {l60}");
        assert!(
            (l120 - l60).abs() < 1e-4,
            "level at 120: {l120}, 60 Hz: {l60}"
        );
        assert!((a60 - 0.631_072).abs() < 1e-4, "alpha at 60: {a60}");
        assert!(
            (a120 - a60).abs() < 1e-4,
            "alpha at 120: {a120}, 60 Hz: {a60}"
        );
        // Unconverted, the level would have reached 2.0 and the alpha 0.543.
        assert!(l120 < 1.8, "level at 120: {l120}");
        assert!(a120 > 0.6, "alpha at 120: {a120}");

        // The strength decays by `decay` once per tick at 60 Hz.
        let (_, _, s60) = fades_after(60, 5, 0.9);
        let (_, _, s120) = fades_after(120, 10, 0.9);
        let want = 333.3333 * 0.9f32.powi(5);
        assert!((s60 - want).abs() < 0.05, "speed at 60: {s60}, want {want}");
        assert!(
            (s120 - s60).abs() < 0.05,
            "speed at 120: {s120}, 60 Hz: {s60}"
        );
        // Unconverted, 10 ticks would leave 116.2.
        assert!(s120 > 150.0, "speed at 120: {s120}");
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
        check(
            ee::s09::f_002e39d8__generated,
            player_pulsefx_update,
            SEEDS_3,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn transition_man_render_matches() {
        check(
            ee::s09::f_002e47e8__generated,
            transition_man_render,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x47e8, 7);
                put_ctx(
                    c,
                    &mut r,
                    &[0x1e8, 0xa8, 0xf8, 0x128, 0xd0, 0x100, 0xb0, 0x150],
                );
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
                    0 => (0.5, 1.0, 2.0, 1.0, false, true), // o1 == 0
                    1 => (0.5, 1.0, 2.0, 1.0, true, true),  // t < fadeIn
                    2 => (1.5, 1.0, 2.0, 1.0, true, false), // o2 == 0
                    3 => (1.5, 1.0, 2.0, 1.0, true, true),  // hold
                    _ => (10.0, 1.0, 2.0, 1.0, true, true), // fade out (clamped)
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
                c.bus
                    .write32(GAME_GP - 0x3a1c, if seed % 4 == 0 { 0 } else { f(2.0) });
                c.g[A0][0] = sx(trans);
            },
        );
    }

    #[test]
    fn dynamic_spray_fx_render_matches() {
        check(
            ssx3_ee::s09::f_002e24d0__generated,
            dynamic_spray_fx_render,
            150,
            &|c, seed| {
                machine(c, seed, 53, 0);
                let spray = DATA + 0x1000;
                let owner = DATA + 0x2000;
                c.bus.write32(spray, owner);
                c.bus
                    .write32(owner + 0xb18, if seed % 4 == 0 { 0 } else { 1 });
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
            },
        );
    }

    // --------------------------------------------------- cTransitionMan_Update

    #[test]
    fn c_transition_man_update_matches() {
        fakes::covered(
            ee::s09::f_002e46c8__generated,
            c_transition_man_update,
            200,
            &|c, seed| {
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
                let start = if seed % 2 == 0 {
                    DATA + 0x8000 - 1
                } else {
                    DATA + 0x8000 - 8
                };
                c.bus.write32(this + 0x80, start);
                c.bus
                    .write32(this + 0x7c, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(this + 0x44, if seed % 5 == 0 { 0 } else { 1 });
                c.bus
                    .write32(this + 0x40, if seed % 7 == 0 { 1 } else { 0 });
                c.bus.write32(
                    this + 0x18,
                    match seed % 4 {
                        0 => 0,
                        1 => 1,
                        _ => r.next_u32() as i32 as u32,
                    },
                );
                c.bus
                    .write32(this + 0x70, if seed % 3 == 0 { 0 } else { DATA + 0x9000 });
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
            },
        );
    }

    #[test]
    fn snow_box_dtor_matches() {
        check(
            ee::s09::f_002e4f28__generated,
            snow_box_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x2e4f28),
        );
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
        check(
            ee::s09::f_002e4cb0__generated,
            c_transition_man_reset_effect_pool,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e4c_b0, 0);
                fill(c, &mut r, 0x0053_8850, 16);
            },
        );
    }

    // -----------------------------------------------------------------------
    // cTransitionMan_StartCaptured.

    #[test]
    fn c_transition_man_start_captured_matches() {
        check(
            ssx3_ee::s09::f_002e4540__generated,
            c_transition_man_start_captured,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e45_40, 0);
                // A zeroed manager: `+0x44` is clear (no Stop), `t0` is 1 and the
                // incoming `a2` is 0, so no vtable is reached.
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ptr(&mut r) as u64;
                c.g[A2][0] = 0;
                c.g[A3][0] = ptr(&mut r) as u64;
                c.f[12] = r.edge_f32();
            },
        );
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
                c.bus
                    .write32(this + 0x44, if seed % 2 == 0 { 0 } else { 1 });
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
        check(
            ee::s09::f_002e3578__generated,
            lens_fx_base_push_render_state,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 37);
                gpf(c, 0x2a90, AUX + 0x100);
                put(c, AUX + 0x100 + 0xe84, AUX + 0x400);
                fillb(c, &mut r, AUX + 0x400, 0x40);
                fillb(c, &mut r, 0x0050_1420, 0x40);
            },
        );
    }

    #[test]
    fn lens_fx_base_pop_render_state_matches() {
        check(
            ee::s09::f_002e3668__generated,
            lens_fx_base_pop_render_state,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 38);
                gpf(c, -0x854, AUX);
                put(c, AUX + 0xe84, r.next_u32());
            },
        );
    }

    #[test]
    fn c_transition_man_script_is_running_matches() {
        check(
            ee::s09::f_002e4c90__generated,
            c_transition_man_script_is_running,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 39);
                c.g[A0][0] = AUX as u64;
                put(c, AUX + 0x60, if seed % 2 == 0 { 0 } else { AUX + 0x200 });
                put(
                    c,
                    AUX + 0x64,
                    if seed % 4 == 1 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    pub(crate) fn put_f32s(c: &mut Cpu, a: u32, v: [f32; 4]) {
        crate::difftest::put_vec(
            c,
            a,
            [
                v[0].to_bits(),
                v[1].to_bits(),
                v[2].to_bits(),
                v[3].to_bits(),
            ],
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
            put_f32s(
                c,
                fx + off,
                [
                    f32::from_bits(r.f32(10.0)),
                    f32::from_bits(r.f32(10.0)),
                    f32::from_bits(r.f32(10.0)),
                    f32::from_bits(r.f32(1.0)),
                ],
            );
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
            put_f32s(
                c,
                owner + off,
                [
                    f32::from_bits(r.f32(4.0)),
                    f32::from_bits(r.f32(4.0)),
                    f32::from_bits(r.f32(4.0)),
                    0.0,
                ],
            );
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
                c.bus
                    .write32(DATA + 0x406c, (f32::from_bits(r.f32(1.0)) + 2.0).to_bits());
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
                    put_f32s(
                        c,
                        owner + off,
                        [
                            f32::from_bits(r.f32(4.0)),
                            f32::from_bits(r.f32(4.0)),
                            f32::from_bits(r.f32(4.0)),
                            0.0,
                        ],
                    );
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

    /// The animation blend `cDynamicSprayFX_UpdateEmitter4` applies to `+0x1c`
    /// is 1/600 per tick at 60 Hz: 60 updates at 60 Hz and 120 at 120 Hz drop
    /// it from 0.5 to the same 0.4.
    #[test]
    fn emitter4_anim_blend_follows_the_rate() {
        /// The blend value after `ticks` updates at rate `hz`, from 0.5.
        fn blend_after(hz: u32, ticks: u32) -> f32 {
            let mut c = crate::difftest::fresh();
            elf_image(&mut c);
            let mut r = machine(&mut c, 3, 0x2e11, 0);
            crate::tick::set_for_test(&mut c, hz);
            // The `1/60` entries this update loads (its 1/600 blend step at
            // gp-0x3ae4 is not one of them: the port stretches that one).
            for off in [-0x3ae0, -0x3ac8, -0x3ac4, -0x3ac0] {
                c.bus.write32(gp_at(off), crate::tick::step_bits(hz));
            }
            let (fx, owner, ctrl, ctrl2) = (DATA, DATA + 0x2000, DATA + 0x6000, DATA + 0x6400);
            let vt = fakes::vtable(&mut c, &mut r, &[0x10, 0x38]);
            c.bus.write32(fx, owner);
            c.bus.write32(fx + 0x28, DATA + 0x3000);
            c.bus.write32(fx + 0x2c, DATA + 0x4000);
            c.bus.write32(owner + 0x780, ctrl);
            c.bus.write32(ctrl + 0x30, DATA + 0x6200);
            c.bus.write32(owner + 0x8a8, 0);
            c.bus.write32(owner + 0x784, ctrl2);
            c.bus.write32(ctrl2 + 8, 0x1b6); // GetLayerAnimEntry -> 0 (target 0)
            c.bus.write32(owner + 0x6c0, vt);
            for off in [0x370, 0x3b0] {
                put_f32s(&mut c, owner + off, [4.0, 4.0, 4.0, 0.0]);
            }
            for off in [0x478, 0x47c, 0x480] {
                c.bus.write32(DATA + 0x4000 + off, f(5.0));
            }
            c.bus.write32(fx + 0xbc, 0); // below the 1111.11 gate: the blend runs
            c.bus.write32(fx + 0x1c, f(0.5)); // the blended animation time
            c.bus.write32(fx + 0x14, 0);
            c.bus.write32(fx + 0x18, 1);
            c.bus.write32(fx + 0x20, 0);
            c.bus.write32(fx + 0x24, f(0.4));
            for _ in 0..ticks {
                c.g[A0][0] = fx as u64;
                dynamic_spray_update_emitter4(&mut c);
            }
            f32::from_bits(c.bus.read32(fx + 0x1c))
        }

        let one_second_60 = blend_after(60, 60);
        let one_second_120 = blend_after(120, 120);
        assert!((one_second_60 - 0.4).abs() < 1e-5, "at 60: {one_second_60}");
        assert!(
            (one_second_120 - one_second_60).abs() < 1e-5,
            "at 120: {one_second_120}, at 60: {one_second_60}"
        );
        // An unconverted step would leave 0.3 after the same wall-clock time.
        assert!(one_second_120 > 0.35, "at 120: {one_second_120}");
    }

    #[test]
    fn lens_fx_base_dtor_matches() {
        check(
            ee::s09::f_002e3060__generated,
            lens_fx_base_dtor,
            40,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn lens_fx_get_flare_size_for_res_matches() {
        check(
            ee::s09::f_002e27e8__generated,
            lens_fx_get_flare_size_for_res,
            SEEDS_8,
            &|c, seed| {
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
            },
        );
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
        check(
            ee::s09::f_002e5d18__generated,
            snow_fall_dtor,
            40,
            &|c, seed| {
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
            },
        );
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
        check(
            ssx3_ee::s09::f_002e42d0__generated,
            c_transition_man_dtor,
            8,
            &|c, seed| {
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
            },
        );
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
        check(
            ee::s09::f_002e4578__generated,
            ctransition_man_stop,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e_4578, 3);
                let this = DATA;
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 0x44, if seed % 2 == 0 { 0 } else { 1 });
                let obj1 = DATA + 0x1000;
                let obj2 = DATA + 0x1200;
                c.bus
                    .write32(this + 0x4c, if seed % 3 == 0 { 0 } else { obj1 });
                c.bus
                    .write32(this + 0x50, if seed % 5 == 0 { 0 } else { obj2 });
                for (i, o) in [obj1, obj2].into_iter().enumerate() {
                    let vt = DATA + 0x2000 + 0x100 * i as u32;
                    c.bus.write32(o, vt);
                    c.bus.write16(vt + 8, 0);
                    c.bus.write32(vt + 0xc, FAKE + 0x10 * i as u32);
                }
                c.bus.write32(
                    this + 0x6c,
                    if seed % 7 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32()
                    },
                );
                let world = DATA + 0x3000;
                let vt3 = DATA + 0x3400;
                c.bus.write32(GAME_GP + 0x2a90, world);
                c.bus.write32(world + 0x10d8, vt3);
                c.bus.write16(vt3 + 0x390, 0);
                c.bus.write32(vt3 + 0x394, FAKE + 0x40);
            },
        );
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
        check(
            ssx3_ee::s09::f_002e4d88__generated,
            snow_box_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x2e4d88 ^ (seed as u64) << 6);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[ssx3_ee_rt::cpu::reg::SP][0] = crate::difftest::SP as u64;
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
            },
        );
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
            if seed % 8 == 0 {
                0x3c800000
            } else {
                r.edge_f32()
            },
        );
        w(c, SELF + wake::BUILT, (seed / 4) % 2);
        w(c, SELF + wake::HAS_PREV, (seed / 5) % 2);

        w(c, OBJ_2 + 0x438, (seed / 8) % 3);
        w(c, OBJ_2 + 0x77c, OWNER);
        w(c, OBJ_2 + 0x780, MODEL);
        w(c, OBJ_2 + 0x6c0, VT);
        w(c, OBJ_2 + 0x8a4, (seed / 10) % 3);
        w(
            c,
            OBJ_2 + 0x320,
            if seed % 2 == 0 { 0 } else { 0x10 + seed },
        );
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

    /// The wake's per-tick blends (0x2dd7c0..) after `ticks` ticks at `hz`
    /// towards fixed targets: (+0x90 smoothed speed, +0x94 intensity, +0x60
    /// point x). Entered at the blend block with the locals it reads in the
    /// frame; the intensity and speed stay above the gates, so each tick
    /// ends in `BuildTrail` and the epilogue.
    fn wake_blend_after(hz: u32, ticks: u32) -> [f32; 3] {
        let mut c = crate::difftest::fresh();
        setup_2(&mut c, 1);
        c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        crate::tick::set_for_test(&mut c, hz);
        let f = |x: f32| x.to_bits();
        let seg = [f(5.0), f(6.0), f(7.0), f(1.0)];
        put_vec(&mut c, SELF + 0x50, seg);
        for off in [0x60, 0x70, 0x80] {
            put_vec(&mut c, SELF + off, [0; 4]);
        }
        c.bus.write32(SELF + 0x90, f(100.0));
        c.bus.write32(SELF + 0x94, f(0.5));
        for _ in 0..ticks {
            let sp = STACK_TOP - 0x170;
            c.g[ssx3_ee_rt::cpu::reg::SP][0] = sx(sp);
            put_vec(&mut c, sp, [f(10.0), f(20.0), f(30.0), f(1.0)]);
            put_vec(&mut c, sp + 0x10, [f(1.0), f(2.0), f(3.0), f(0.0)]);
            put_vec(&mut c, sp + 0x20, [f(0.0), f(1.0), f(0.0), f(0.0)]);
            put_vec(&mut c, sp + 0x30, [f(200.0), f(1.0), 0, 0]);
            put_vec(&mut c, sp + 0x50, seg);
            c.g[S2][0] = sx(SELF);
            c.f[20] = f(1.0);
            c.pc = 0x002d_d7c0;
            c_board_wake_fx_update(&mut c);
        }
        [0x90, 0x94, 0x60].map(|off| f32::from_bits(c.bus.read32(SELF + off)))
    }

    /// The wake's 0.8/0.2 and 0.7/0.3 blends keep their time constants at
    /// 120 and 240 Hz.
    #[test]
    fn board_wake_fx_update_follows_the_rate() {
        for frames in [1u32, 4, 10] {
            let at60 = wake_blend_after(60, frames);
            let k = |x: f32| x.powi(frames as i32);
            let want = [
                200.0 - 100.0 * k(0.8),
                1.0 - 0.5 * k(0.7),
                10.0 * (1.0 - k(0.8)),
            ];
            for hz in [60u32, 120, 240] {
                let at = if hz == 60 {
                    at60
                } else {
                    wake_blend_after(hz, frames * hz / 60)
                };
                for i in 0..3 {
                    let tol = 1e-3 * want[i].abs().max(1.0);
                    assert!(
                        (at[i] - want[i]).abs() < tol,
                        "{hz} Hz, {frames}: [{i}] {} vs {}",
                        at[i],
                        want[i]
                    );
                }
            }
        }
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

    /// The boost streamer's rider owner (main state at `+0xde0`) and point
    /// ring (20 pairs of quads).
    pub(crate) const BOOST_OWNER: u32 = DATA + 0xc000;

    pub(crate) const BOOST_POINTS: u32 = DATA + 0xd000;

    /// The rider, its model, board vtable and animation as
    /// `UpdatePoints` reads them (see its test), at `FX_RIDER`.
    pub(crate) fn boost_rider(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), BELIB);
        c.bus.write32(FX_RIDER + 0x77c, BOOST_OWNER);
        c.bus.write32(FX_RIDER + 0x6c0, FX_VT);
        c.bus.write32(FX_RIDER + 0x780, MODEL_2);
        c.bus.write32(FX_RIDER + 0x784, ANIM);
        c.bus.write32(FX_RIDER + 0x8a4, r.next_u32() % 8);
        c.bus.write32(FX_RIDER + 0x320, (seed % 2) as u32);
        c.bus.write32(FX_RIDER + 0x330, r.f32(1.0));
        c.bus.write16(FX_VT + 0x28, 0);
        c.bus.write32(FX_VT + 0x2c, FAKE);
        c.bus.write32(MODEL_2 + 0x30, BONES);
    }

    #[test]
    fn speed_boost_streamer_fx_update_matches() {
        check_lohi(
            ee::s09::f_002e66b8__generated,
            c_speed_boost_streamer_fx_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x66b8, 1);
                elf_image(c);
                boost_rider(c, &mut r, seed);
                // In the air (state 1) or not; the two boost timers.
                c.bus.write32(
                    BOOST_OWNER + 0xde0,
                    if seed % 4 == 0 {
                        1
                    } else {
                        r.next_u32() % 3 * 2
                    },
                );
                let (t1, t2): (f32, f32) = match (seed / 4) % 4 {
                    0 => (0.5, -1.0),
                    1 => (-0.5, 0.7),
                    2 => (-1.0, 0.0),
                    _ => (3.0, 2.0),
                };
                c.bus.write32(FX_RIDER + 0x2fc, f(t1));
                c.bus.write32(FX_RIDER + 0x2e8, f(t2));
                c.bus.write32(
                    FX_RIDER + 0x2f4,
                    [0xbu32, 3, 7, 12, 0][(seed / 16 % 5) as usize],
                );
                for off in [0x1640, 0x1644, 0x1648] {
                    c.bus.write32(GAME_GP + off, r.next_u32() % 2);
                }
                c.bus.write32(GAME_GP + 0x1610, (seed % 7 != 0) as u32);
                for off in [0x1a0, 0x1e0, 0x370, 0x460] {
                    put_vec(
                        c,
                        FX_RIDER + off,
                        [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
                    );
                }
                // Board bones (+0x89c, +0x8b0, +0x8b8).
                c.bus.write32(FX_RIDER + 0x89c, r.next_u32() % 4);
                c.bus.write32(FX_RIDER + 0x8b0, r.next_u32() % 4);
                c.bus.write32(FX_RIDER + 0x8b8, r.next_u32() % 4);
                for i in 0..0x100 {
                    c.bus.write32(BONES + 4 * i, r.edge_f32());
                }
                // The streamer: fade, alpha, width, texture start, the
                // flames UpdatePoints runs, the point ring.
                c.bus.write32(M, FX_RIDER);
                c.bus
                    .write32(M + 0x4, f([0.5f32, -0.5, 30.0][(seed % 3) as usize]));
                c.bus.write32(M + 0x8, r.f32(1.0));
                c.bus.write32(M + 0xc, r.f32(50.0));
                c.bus.write32(
                    M + 0x10,
                    f([0.0f32, 0.999, 1.5, -0.4, 0.5][(seed / 3 % 5) as usize]),
                );
                c.bus
                    .write32(M + 0x18, f(if seed % 5 == 0 { -0.5 } else { 0.5 }));
                c.bus.write32(M + 0x14, r.next_u32() % 40);
                c.bus.write32(M + 0x30, r.next_u32() % 4);
                for i in 0..0x30 {
                    c.bus.write32(M + 0x50 + 4 * i, r.edge_f32());
                }
                c.bus.write32(M + 0x20, r.f32(1.0));
                c.bus.write32(M + 0x24, r.f32(1.0));
                c.bus.write32(M + boost::FIRST, r.next_u32() % 20);
                c.bus.write32(
                    M + boost::COUNT,
                    [
                        0u32,
                        1,
                        2,
                        5,
                        19,
                        20,
                        (-1i32) as u32,
                        (-2i32) as u32,
                        (-5i32) as u32,
                    ][(seed / 2 % 9) as usize],
                );
                c.bus
                    .write32(M + boost::MAX_COUNT, if seed % 3 == 0 { 18 } else { 20 });
                c.bus.write32(M + boost::IN_AIR, (seed % 3 == 1) as u32);
                c.bus.write32(M + boost::POINTS, BOOST_POINTS);
                for i in 0..20 * 8 {
                    c.bus.write32(BOOST_POINTS + 4 * i, r.f32(100.0));
                }
                c.g[A0][0] = sx(M);
                c.pc = 0x002e_66b8;
            },
        );
    }

    /// The boost streamer after `ticks` updates at `hz` with the object at
    /// `fx` (its 60 Hz clock is keyed by the address): (first, count, fade,
    /// alpha, width, texture start, first pair's `a.y`).
    fn boost_after(hz: u32, ticks: u32, fx: u32, boosting: bool) -> (u32, u32, [f32; 5]) {
        let mut c = crate::difftest::fresh();
        let mut r = machine(&mut c, 1, 0x66b9, 0);
        elf_image(&mut c);
        crate::tick::set_for_test(&mut c, hz);
        // The texture scroll's `.lit4` 1/60.
        c.bus
            .write32(GAME_GP.wrapping_sub(0x39dc), crate::tick::step_bits(hz));
        boost_rider(&mut c, &mut r, 0);
        c.bus.write32(BOOST_OWNER + 0xde0, 0); // on the ground
        c.bus
            .write32(FX_RIDER + 0x2fc, f(if boosting { 0.5 } else { -1.0 }));
        c.bus.write32(FX_RIDER + 0x2e8, f(-1.0));
        c.bus.write32(FX_RIDER + 0x2f4, 3);
        c.bus.write32(GAME_GP + 0x1610, 1);
        put_vec(&mut c, FX_RIDER + 0x1e0, [f(100.0), 0, 0, 0]);
        put_vec(&mut c, FX_RIDER + 0x1a0, [0, f(1.0), 0, 0]);
        for off in [0x370, 0x460] {
            put_vec(&mut c, FX_RIDER + off, [f(1.0), f(2.0), f(3.0), 0]);
        }
        c.bus.write32(fx, FX_RIDER);
        c.bus.write32(fx + boost::FADE, f(100.0));
        c.bus.write32(fx + boost::ALPHA, f(1.0));
        c.bus.write32(fx + boost::WIDTH, f(50.0));
        c.bus.write32(fx + boost::V_START, f(0.5));
        c.bus.write32(fx + boost::FIRST, 0);
        c.bus
            .write32(fx + boost::COUNT, if boosting { 0 } else { 10 });
        c.bus.write32(fx + boost::MAX_COUNT, 20);
        c.bus.write32(fx + boost::IN_AIR, 0);
        c.bus.write32(fx + boost::POINTS, BOOST_POINTS);
        for _ in 0..ticks {
            c.g[A0][0] = sx(fx);
            c.pc = 0x002e_66b8;
            c_speed_boost_streamer_fx_update(&mut c);
        }
        let floats = [
            fx + boost::FADE,
            fx + boost::ALPHA,
            fx + boost::WIDTH,
            fx + boost::V_START,
            BOOST_POINTS + 4,
        ]
        .map(|a| f32::from_bits(c.bus.read32(a)));
        (
            c.bus.read32(fx + boost::FIRST),
            c.bus.read32(fx + boost::COUNT),
            floats,
        )
    }

    /// Records, spread and shrink once per 60 Hz frame and the 0.95 fade per
    /// 60 Hz frame of time: the same trail at 120 and 240 Hz as at 60.
    #[test]
    fn speed_boost_streamer_fx_update_follows_the_rate() {
        // A fresh object address per run (clear of the fixture's rider,
        // model, bones and points at DATA + 0x9000..0xe000).
        let mut fx = DATA + 0x2_0000;
        for boosting in [false, true] {
            for frames in [3u32, 7] {
                let at60 = boost_after(60, frames, fx, boosting);
                fx += 0x1000;
                if boosting {
                    assert_eq!((at60.0, at60.1), (frames % 20, frames), "{at60:?}");
                    assert!((at60.2[4] + 1.4 * frames as f32).abs() < 1e-3, "{at60:?}");
                } else {
                    assert_eq!(at60.1, 10 - frames, "{at60:?}");
                }
                for hz in [120u32, 240] {
                    let at = boost_after(hz, frames * hz / 60, fx, boosting);
                    fx += 0x1000;
                    assert_eq!(
                        (at.0, at.1),
                        (at60.0, at60.1),
                        "{hz} Hz {boosting} {frames}: {at:?} vs {at60:?}"
                    );
                    for i in 0..5 {
                        let (a, b) = (at.2[i], at60.2[i]);
                        let d = (a - b).abs();
                        // The texture start wraps at 1.
                        let d = if i == 3 { d.min((1.0 - d).abs()) } else { d };
                        assert!(
                            d < 1e-3 * b.abs().max(1.0),
                            "{hz} Hz {boosting} {frames}: [{i}] {a} vs {b}"
                        );
                    }
                }
            }
        }
    }
    use super::*;
    use super::*;
    pub(crate) use crate::game::ai::ai::rider_spin_sub_handle_input;

    // -- 0x00133308 RiderSpinSub_HandleInput -------------------------------

    pub(crate) mod spin_input {
        use super::*;
        use crate::difftest::fakes::{fake_call, machine, FAKE};
        use crate::difftest::{check, elf_image, put_vec, Rng, DATA, GAME_GP};

        /// The objects (zeroed RAM except the fields written below).
        const THIS: u32 = DATA + 0x5_0000;
        const PAD: u32 = DATA + 0x5_0100;
        const RIDER: u32 = DATA + 0x5_1000;
        const STATE: u32 = DATA + 0x5_2000;
        const CTRL: u32 = DATA + 0x5_3000;
        const CTRL_VT: u32 = DATA + 0x5_3100;
        const SEQ_LISTS: u32 = DATA + 0x5_3200;
        const SEQ: u32 = DATA + 0x5_3300;
        const ANIM_STATE: u32 = DATA + 0x5_4000;
        const METRIX: u32 = DATA + 0x5_5000;
        const OVERLAPS: u32 = DATA + 0x5_6000;
        const BE_LIBRARY: u32 = DATA + 0x5_7000;
        const BE_OBJ: u32 = DATA + 0x5_7100;
        const BE_VT: u32 = DATA + 0x5_7200;
        const WORLD: u32 = DATA + 0x5_8000;
        const WORLD_84: u32 = DATA + 0x5_8100;
        const WORLD_TABLE: u32 = DATA + 0x5_8200;
        /// The static animation table (28-byte entries; `+0` the entry kind
        /// `AnimController_GetLayerAnimEntry` returns).
        const ANIM_TABLE: u32 = 0x0044_6990;
        /// The behaviour library's interface objects (`BELibrary_GetInterface`).
        const BE_INTERFACES: u32 = 0x0053_08a8;
        /// Fake methods: the behaviour interface getter, and the controller's
        /// "current animation kind" (answers 0x207: `PlayAnim` and
        /// `GetCurrentAnimLength` stop there).
        const FAKE_BE: u32 = FAKE;
        const FAKE_ANIM_KIND: u32 = FAKE + 0x10;

        fn f(x: f32) -> u32 {
            x.to_bits()
        }

        /// The fake hook: the animation-kind method answers 0x207.
        fn miss(c: &mut Cpu, target: u32) {
            fake_call(c, target);
            if target == FAKE_ANIM_KIND {
                c.g[V0][0] = 0x207;
            }
        }

        /// A 6-bit signed stick field: centred half of the time.
        fn axis(r: &mut Rng) -> u32 {
            if r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32() & 0x3f
            }
        }

        /// An angle-like float: 0, a half / whole turn, or random.
        fn angle(r: &mut Rng) -> u32 {
            match r.next_u32() % 6 {
                0 | 1 => 0,
                2 => f(std::f32::consts::PI),
                3 => f(-std::f32::consts::PI),
                4 => f(2.0 * std::f32::consts::PI),
                _ => r.f32(7.0),
            }
        }

        fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0013_3308, 6);
            let r = &mut r;
            c.hooks.miss = miss;
            elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            // Runtime-computed globals past the image (gp+0x1ba0..): half
            // turn, epsilon, lean limits, gains, full turns, clamp.
            for (off, v) in [
                (0x1ba0, std::f32::consts::PI),
                (0x1ba4, 0.01),
                (0x1ba8, 0.6),
                (0x1bac, 0.5),
                (0x1bb0, 0.3),
                (0x1bb4, 2.0 * std::f32::consts::PI),
                (0x1bb8, 2.0 * std::f32::consts::PI),
                (0x1bbc, 1.0),
            ] {
                w(c, GAME_GP + off, f(v));
            }
            // The behaviour library: the singleton exists, interface 0 is an
            // object whose getter is a fake; the character tables give a
            // divisor (character 0 of player 0).
            w(c, GAME_GP - 0x1f04, BE_LIBRARY);
            w(c, GAME_GP - 0x1efc, 0);
            w(c, BE_INTERFACES, BE_OBJ);
            w(c, BE_OBJ + 4, BE_VT);
            w(c, BE_VT + 0xc, FAKE_BE);
            c.bus.write8(0x0053_08e2, 10);
            c.bus.write8(0x0053_08e4, 10);
            // The world chain `Rider_TryAirReverse` reads (entry 0: zero).
            w(c, GAME_GP - 0x848, WORLD);
            w(c, WORLD + 0x84, WORLD_84);
            w(c, WORLD_84 + 0x44, WORLD_TABLE);

            // The pad: never the reset bit (12); the other buttons, the
            // trick byte and the axes at random, centred often. Every third
            // seed has no lean input at all (and no lean): the animation
            // choice at the end runs only then.
            let calm = seed % 3 == 0;
            let buttons = (r.next_u32() & !0x3f00_1000) | axis(r) << 24;
            let mut sticks = axis(r) | axis(r) << 6 | axis(r) << 12 | (r.next_u32() & 3) << 18;
            sticks |= r.next_u32() & 0xfff0_0000;
            if calm {
                sticks &= !0x000f_ffc0;
            }
            w(c, PAD, buttons);
            w(c, PAD + 4, sticks);

            // The spin sub-state.
            w(c, THIS + 0x0, r.next_u32() % 5); // MODE
            w(c, THIS + 0x4, 6); // trick state: `UpdateTrick` only reads the layer
            w(c, THIS + 0x8, r.next_u32());
            w(c, THIS + 0xc, r.next_u32() % 5); // PHASE
            let spin_target = angle(r);
            let flip_target = angle(r);
            w(c, THIS + 0x10, flip_target);
            w(c, THIS + 0x14, spin_target);
            w(
                c,
                THIS + 0x18,
                if r.next_u32() % 2 == 0 {
                    flip_target
                } else {
                    angle(r)
                },
            );
            w(
                c,
                THIS + 0x1c,
                if r.next_u32() % 2 == 0 {
                    spin_target
                } else {
                    angle(r)
                },
            );
            for off in [0x20, 0x24, 0x30, 0x34] {
                w(c, THIS + off, r.f32(10.0));
            }
            let (lean_x, lean_y) = if calm { (0, 0) } else { (angle(r), angle(r)) };
            w(c, THIS + 0x28, lean_x);
            w(c, THIS + 0x2c, lean_y);
            w(c, THIS + 0x38, r.f32(8.0));
            w(c, THIS + 0x3c, r.f32(8.0));
            w(c, THIS + 0x44, r.next_u32() % 2);
            for off in [0x48, 0x4c, 0x50, 0x54] {
                w(
                    c,
                    THIS + off,
                    if r.next_u32() % 3 == 0 { 0 } else { r.f32(1.0) },
                );
            }
            w(c, THIS + 0x58, RIDER);

            // The rider: in the air (main state 2: no rail or handplant
            // search), no boost (`Rider_AddBoost` returns at once).
            w(c, RIDER + 0x77c, STATE);
            w(c, STATE + 0xde0, 2);
            w(c, RIDER + 0x784, CTRL);
            w(c, RIDER + 0x788, ANIM_STATE);
            w(c, RIDER + 0x790, METRIX);
            w(c, RIDER + 0x860, OVERLAPS);
            w(c, RIDER + 0x304, 3);
            w(c, RIDER + 0x2f4, r.next_u32() % 10);
            w(c, RIDER + 0x86c, 0);
            w(c, RIDER + 0xb34, 1 + r.next_u32() % 5);
            w(c, RIDER + 0x2dc, r.f32(5.0));
            w(c, RIDER + 0x2e0, r.f32(5.0));
            w(
                c,
                RIDER + 0x2ec,
                if r.next_u32() % 2 == 0 { 0 } else { r.f32(1.0) },
            );
            w(c, RIDER + 0x320, r.next_u32() % 2);
            w(c, RIDER + 0x324, r.next_u32() % 2);
            w(
                c,
                RIDER + 0x28c,
                if r.next_u32() % 3 == 0 { 0 } else { r.f32(1.0) },
            );
            w(
                c,
                RIDER + 0x298,
                if r.next_u32() % 3 == 0 { 0 } else { r.f32(1.0) },
            );
            // `Rider_TryAirReverse` (anim kinds 1 and 3) gives up: the
            // rider's up vector points against the reference.
            put_vec(c, RIDER + 0x180, [f(1.0), 0, 0, 0]);
            put_vec(c, ANIM_STATE + 0x20, [f(-1.0), 0, 0, 0]);
            w(c, ANIM_STATE + 0xac, r.next_u32() % 4);
            w(c, ANIM_STATE + 0x98, r.f32(2.0));
            w(c, ANIM_STATE + 0xa0, r.f32(2.0));
            // The metrix: zeroed (no trick to announce); the spin flag.
            w(c, METRIX + 0x14, r.f32(1.0));

            // The animation controller: layer 2 plays `id` (spin poses
            // 0x10d..0x128, 0x11f, 0x120, 0x131, tweaks, "none"), whose
            // table kind is a trick (0x12..0x14), 2, 9 or other.
            const IDS: [u32; 6] = [0x11f, 0x120, 0x131, 0x129, 0x1b6, 0x200];
            let id = if r.next_u32() % 2 == 0 {
                0x10d + r.next_u32() % 0x1c
            } else {
                IDS[(r.next_u32() % 6) as usize]
            };
            const KINDS: [u32; 7] = [0x12, 0x13, 0x14, 2, 2, 9, 0];
            w(
                c,
                ANIM_TABLE + id * 0x1c,
                KINDS[(r.next_u32() % 7) as usize],
            );
            w(c, CTRL + 8, id);
            w(c, CTRL + 0x1c, f(1.0));
            w(c, CTRL + 0x50, SEQ_LISTS);
            w(c, CTRL + 0x58, CTRL_VT);
            w(c, CTRL_VT + 0x24, FAKE_ANIM_KIND);
            w(c, SEQ_LISTS + 0x14, SEQ);
            w(c, SEQ + 0xc0, r.next_u32() % 2);

            // Rare paths made common (on top of the random state, all with
            // no lean and no trick on the layer):
            match seed % 9 {
                // The spin settles this tick (phase 1 → 2, stick centred,
                // rotation at its targets): the landing-blend jump table on
                // a spin pose (or the default).
                3 => {
                    w(c, THIS + 0xc, 1);
                    w(c, PAD, buttons & !0x3f00_0000);
                    w(c, PAD + 4, sticks & !0x3f);
                    w(c, THIS + 0x18, flip_target);
                    w(c, THIS + 0x1c, spin_target);
                    w(c, THIS + 0x54, f(10.0));
                    let id = if seed % 2 == 0 {
                        0x10d + r.next_u32() % 0x1c
                    } else {
                        0x200
                    };
                    w(c, CTRL + 8, id);
                    w(c, ANIM_TABLE + id * 0x1c, if seed % 4 == 1 { 0 } else { 2 });
                }
                // A further half turn queued in phase 1 with the layer on a
                // kind-2 entry: the flip-pose choice.
                6 => {
                    w(c, THIS + 0xc, 1);
                    w(c, THIS + 0x44, 1);
                    let id = 0x10d + r.next_u32() % 0x1c;
                    w(c, CTRL + 8, id);
                    w(c, ANIM_TABLE + id * 0x1c, 2);
                }
                _ => {}
            }

            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(PAD);
            // The generated body (no override slot yet) dispatches on `pc`:
            // enter at the start.
            c.pc = 0x0013_3308;
        }

        /// `RiderSpinSub_HandleInput`: every hold mode and spin phase,
        /// centred and pushed stick axes, targets reached or not, with and
        /// without a trick on the layer, the animation choices (including
        /// the landing-blend jump table) and fake callees that unwind.
        #[test]
        fn rider_spin_sub_handle_input_matches() {
            check(
                ssx3_ee::s01::f_00133308__generated,
                rider_spin_sub_handle_input,
                300,
                &setup,
            );
        }
    }

    // -- 0x002e4f50 SnowBox_Update ------------------------------------------

    /// `SnowBox_Update` moves the weather flake box: every mode (0, 1,
    /// other), the drift timer running out or not, the snap flag, and each
    /// position component ending inside, above or below the box (wrap).
    #[test]
    fn snow_box_update_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        use snow_update::*;

        const BOX: u32 = DATA;
        const REC: u32 = DATA + 0x100;
        const RATE_OBJ: u32 = DATA + 0x200;
        let f = |x: f32| x.to_bits();
        let gp = |o: i32| GAME_GP.wrapping_add(o as u32);

        check(
            ssx3_ee::s09::f_002e4f50__generated,
            snow_box_update,
            300,
            &|c, seed| {
                let mut r = Rng(0x002e_4f50 ^ (seed as u64) << 20 ^ 0x9e37_79b9);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x002e_4f50;

                // Globals: the frame rate, the seed, the heading factors, the
                // wind and the sine table.
                c.bus.write32(gp(GP_RATE_OBJ), RATE_OBJ);
                c.bus
                    .write32(RATE_OBJ + RATE, if seed % 2 == 0 { 60 } else { 30 });
                c.bus.write32(gp(GP_SEED), r.next_u32());
                c.bus.write32(gp(GP_HEADING_K1), f(81.487_33));
                c.bus
                    .write32(gp(GP_HEADING_K2), f(1.0 + (r.next_u32() % 4) as f32));
                for k in 0..4 {
                    c.bus.write32(WIND + 4 * k, r.f32(2.0));
                }
                for i in 0..0x280 {
                    c.bus.write32(SIN_TABLE + 4 * i, r.f32(1.0));
                }

                // The weather record.
                c.bus
                    .write32(REC + REC_DENSITY0, f((r.next_u32() % 1000) as f32 * 0.002));
                c.bus
                    .write32(REC + REC_SPEED0, f((r.next_u32() % 100) as f32 * 0.0002));
                c.bus
                    .write32(REC + REC_DENSITY1, f((r.next_u32() % 1000) as f32 * 0.3));
                c.bus
                    .write32(REC + REC_SPEED1, f((r.next_u32() % 100) as f32 * 0.0004));
                c.bus.write32(REC + REC_F4, r.f32(10.0));
                c.bus.write32(REC + REC_HEADING, r.f32(6.0));
                c.bus
                    .write32(REC + REC_RANDOMNESS, f((r.next_u32() % 101) as f32 * 0.01));
                c.bus.write32(REC + REC_WIND, r.f32(1.0));
                for k in 0..8 {
                    c.bus.write32(REC + REC_C0 + 4 * k, r.f32(255.0));
                }

                // The box: mode by seed % 3, timer expiring on (seed / 3) % 2,
                // snap on (seed / 6) % 2.
                let mode = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => [2, 7, 0xffff_ffff][(seed / 3 % 3) as usize],
                };
                c.bus.write32(BOX + MODE, mode);
                c.bus
                    .write32(BOX + SPEED, f((r.next_u32() % 100) as f32 * 0.1));
                c.bus.write32(BOX + WIND_SCALE, r.f32(1.0));
                let timer = if seed / 3 % 2 == 0 {
                    0.001
                } else {
                    0.5 + (r.next_u32() % 4) as f32
                };
                c.bus.write32(BOX + TIMER, f(timer));
                c.bus
                    .write32(BOX + PERIOD, f(1.0 + (r.next_u32() % 4) as f32));
                c.bus.write32(
                    BOX + SNAP,
                    if seed / 6 % 2 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    },
                );
                for k in 0..4 {
                    c.bus.write32(BOX + VEL + 4 * k, r.f32(5.0));
                    c.bus.write32(BOX + PREV_VEL + 4 * k, r.f32(5.0));
                }
                // One position per component with its own wrap case (inside,
                // above +h, below -h); the move per tick stays well under 1.
                let size = 2.0 + (r.next_u32() % 200) as f32 * 0.25;
                let h = size * 0.5;
                c.bus.write32(BOX + SIZE, f(size));
                for k in 0..3u32 {
                    let v = match r.next_u32() % 3 {
                        0 => (r.next_u32() as f32 / u32::MAX as f32 - 0.5) * h,
                        1 => h + 2.0 + (r.next_u32() % 10) as f32,
                        _ => -h - 2.0 - (r.next_u32() % 10) as f32,
                    };
                    c.bus.write32(BOX + POS + 4 * k, f(v));
                }
                c.bus.write32(BOX + POS + 0xc, r.f32(2.0));

                c.g[A0][0] = sx(BOX);
                c.g[A1][0] = sx(REC);
            },
        );
    }

    // ------------------------------------------- cDynamicSprayFX_Update

    /// `cDynamicSprayFX_Update` against the generated body. Real callees run
    /// on short paths: the emitter setup calls a real `return 0` method per
    /// emitter, the bone lookup finds no bones, the rider-state and anim
    /// getters are tiny leaves, the burst and enable updates are leaves, and
    /// emitter 5 (switched on by the fixture or by the enable update) takes
    /// its spawn path on two prepared emitter slots. The two virtual calls on
    /// the physics sub-object are fakes (speed vector, surface index) and may
    /// unwind. The other seven emitter updates stay off (their flags are 0
    /// and the enable update, reached only with enable states 2/3, leaves
    /// them off).
    #[test]
    fn c_dynamic_spray_fx_update_matches() {
        use crate::difftest::fakes::{self, FAKE, LOG_RESULTS};
        use crate::difftest::{put_vec, DATA};
        use crate::game::visualfx::boardwakefx::tests::{f, gp_at, put_spray_slot, RETURN_ZERO};

        const FX: u32 = DATA;
        const OBJ: u32 = DATA + 0x1000;
        const EM: u32 = DATA + 0x2000;
        const VT_EMITTER: u32 = DATA + 0x3800;
        const CURVES: u32 = DATA + 0x3900;
        const VT_PHYSICS: u32 = DATA + 0x3a00;
        const MODEL: u32 = DATA + 0x3b00;
        const BONE_TABLE: u32 = DATA + 0x3b80;
        const BONE_GROUPS: u32 = DATA + 0x3c00;
        const MATRICES: u32 = DATA + 0x4000;
        const ANIM: u32 = DATA + 0x4200;
        const STATES: u32 = DATA + 0x4300;
        const BONES: u32 = DATA + 0x5200;
        const SPEED_VEC: u32 = DATA + 0x5300;
        const WORLD: u32 = DATA + 0x5400;
        const GAME: u32 = DATA + 0x5500;
        const RACE: u32 = DATA + 0x5600;
        const PLAYERS: u32 = DATA + 0x5900;
        /// The surface colour table (0xf0 bytes per surface).
        const SURFACES: u32 = 0x004f_a398;

        fakes::covered(
            ee::s09::f_002df920__generated,
            c_dynamic_spray_fx_update,
            250,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2df9_20, 2);
                let bit = |r: &mut crate::difftest::Rng| r.next_u32() & 1;

                // .lit4 constants: 60, the fast slow-down rate and speed
                // (555.6), the rail burst rate (27.8); emitter 5's.
                for (off, v) in [
                    (-0x3b88, 59.999996f32),
                    (-0x3b84, 555.55554),
                    (-0x3b80, 27.777779),
                    (-0x3abc, 27.777778),
                    (-0x3ab8, 277.77777),
                    (-0x3ab4, 3333.3335),
                    (-0x3ab0, 0.3),
                    (-0x3aac, 0.4),
                    (-0x3aa8, 0.8),
                    (-0x3aa4, 0.2),
                    (-0x3aa0, 1.0 / 60.0),
                    (-0x3a9c, 83.333336),
                    (-0x3a98, 0.4),
                    (-0x3a94, 0.5),
                ] {
                    c.bus.write32(gp_at(off), f(v));
                }
                c.bus.write32(gp_at(0xa0c), r.next_u32());
                c.bus.write32(gp_at(-0x848), WORLD);
                c.bus.write32(WORLD + 0x84, GAME);
                c.bus.write32(GAME + 0x44, RACE);
                c.bus.write32(GAME + 0x84, PLAYERS);
                c.bus.write32(PLAYERS + 0x10, 0);

                // The spray.
                for k in 0..0x4c {
                    c.bus.write32(FX + 4 * k, r.edge_f32());
                }
                c.bus.write32(FX + spray_fx::OBJ, OBJ);
                c.bus.write32(FX + spray_fx::EMITTERS, EM);
                c.bus.write32(FX + 0x2c, CURVES);
                c.bus
                    .write32(FX + spray_fx::NEEDS_SETUP, (seed % 5 == 0) as u32);
                c.bus.write32(FX + 0x7c, BONES);
                c.bus
                    .write32(FX + spray_fx::BONES_FOUND, (seed % 6 != 0) as u32);
                c.bus.write32(FX + 0x124, bit(&mut r));

                // The speed vector the physics sub-object returns, and the
                // previous speed (slow-down rate = (prev - speed) * 60).
                let (vec, prev) = match seed % 4 {
                    0 => ([0; 4], r.f32(1500.0)),
                    1 => {
                        let x = 600.0 + (r.next_u32() % 400) as f32;
                        let dv = [20.0f32, 5.0, 0.3, -5.0][((seed / 4) % 4) as usize];
                        ([f(x), 0, 0, r.edge_f32()], f(x + dv))
                    }
                    2 => (
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                        r.edge_f32(),
                    ),
                    _ => (
                        [r.f32(300.0), r.f32(300.0), r.f32(300.0), r.f32(1.0)],
                        f(f32::from_bits(r.f32(800.0)).abs()),
                    ),
                };
                put_vec(c, SPEED_VEC, vec);
                c.bus.write32(FX + spray_fx::PREV_SPEED, prev);
                c.bus.write32(LOG_RESULTS, SPEED_VEC);
                let surface = 1 + seed % 7;
                c.bus.write32(LOG_RESULTS + 4, surface);
                for k in 0..4 {
                    c.bus.write32(SURFACES + surface * 0xf0 + 4 * k, r.f32(1.0));
                }

                // The rider.
                for k in 0..0x2c4 {
                    c.bus.write32(OBJ + 4 * k, r.edge_f32());
                }
                c.bus
                    .write32(OBJ + spray_rider::IN_TRICK, (seed % 3 != 0) as u32);
                c.bus.write32(OBJ + spray_rider::ANIM, ANIM);
                let anims = [23u32, 25, 28, 37, 38, 22, 29, 36, 39, 0];
                c.bus.write32(ANIM + 8, anims[(r.next_u32() % 10) as usize]);
                c.bus.write32(OBJ + spray_rider::GOOFY, bit(&mut r));
                c.bus.write32(OBJ + spray_rider::SLIP, r.f32(1.5));
                c.bus
                    .write32(OBJ + spray_rider::FORWARD_SPEED, r.f32(500.0));
                for k in 0..4 {
                    c.bus
                        .write32(OBJ + spray_rider::VELOCITY + 4 * k, r.edge_f32());
                    c.bus
                        .write32(OBJ + spray_rider::BOARD_DIR + 4 * k, r.f32(1.0));
                }
                if seed % 9 == 0 {
                    put_vec(c, OBJ + spray_rider::VELOCITY, [0; 4]);
                }
                c.bus.write32(OBJ + spray_rider::INDEX, seed % 4);
                c.bus.write32(OBJ + spray_rider::PHYSICS, VT_PHYSICS);
                c.bus
                    .write16(VT_PHYSICS + 0x10, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(VT_PHYSICS + 0x14, FAKE);
                c.bus
                    .write16(VT_PHYSICS + 0x38, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(VT_PHYSICS + 0x3c, FAKE + 0x10);
                c.bus.write32(OBJ + 0x77c, STATES);
                c.bus.write32(STATES + 0xde0, seed % 7);
                c.bus
                    .write32(STATES + 0xde4, if seed % 11 == 0 { 9 } else { 3 });
                for off in [
                    spray_rider::GRINDING,
                    spray_rider::GRIND_OFF,
                    spray_rider::GRIND_RAIL,
                    spray_rider::GRIND_SPRAY,
                ] {
                    c.bus.write32(OBJ + off, bit(&mut r));
                }
                // The enable state: unchanged, or changed to 2/3 (which
                // leaves only emitter 5 on).
                let state = 2 + bit(&mut r);
                c.bus.write32(OBJ + spray_rider::ENABLE_STATE, state);
                let known = if seed % 4 == 3 { state ^ 1 } else { state };
                c.bus.write32(FX + spray_fx::ENABLE_STATE, known);

                // The model: matrices for the spray bone; no named bones.
                c.bus.write32(OBJ + spray_rider::MODEL, MODEL);
                c.bus.write32(OBJ + spray_rider::SPRAY_BONE, seed % 4);
                c.bus.write32(MODEL + 0x30, MATRICES);
                c.bus.write32(MODEL + 0x1c, BONE_TABLE);
                c.bus.write32(MODEL + 0xc, BONE_GROUPS);
                c.bus.write32(BONE_TABLE, 0);
                c.bus.write32(BONE_GROUPS + 0x44, 0);
                for k in 0..0x40 {
                    c.bus.write32(MATRICES + 4 * k, r.edge_f32());
                }

                // The ten emitters: a `return 0` setup method each; slots 5
                // and 6 ready for emitter 5's spawns; only emitter 5's flag
                // may be on.
                c.bus.write16(VT_EMITTER + 0x18, 0);
                c.bus.write32(VT_EMITTER + 0x1c, RETURN_ZERO);
                for k in 0..10 {
                    let e = EM + k * 0x210;
                    c.bus.write32(e + 0x1f8, VT_EMITTER);
                    c.bus.write32(e + 0x58, r.f32(2.0));
                }
                put_spray_slot(c, &mut r, EM + 5 * 0x210, DATA + 0x8000);
                put_spray_slot(c, &mut r, EM + 6 * 0x210, DATA + 0x8100);
                for (flag, _, _) in spray_callee::EMITTERS {
                    c.bus.write32(EM + flag, 0);
                }
                c.bus.write32(EM + 0xbc4, (seed % 5 == 1) as u32);

                c.g[A0][0] = sx(FX);
            },
        );
    }
    use super::*;

    /// Emitter 7 (the carve spray): the chance inputs (side dot, lean,
    /// speed), the carving/enabled/turn gates, the side and +0x320 flips, the
    /// trail-point vtable call (a fake) and the real `DynamicEmitter_SpawnAt`
    /// on slot 7, with the idle keep-alive when any gate fails.
    #[test]
    fn dynamic_spray_fx_update_emitter7_matches() {
        use spray7::*;
        check_lohi(
            ee::s09::f_002e1a80__generated,
            dynamic_spray_fx_update_emitter7,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e1a, 2);
                c.pc = 0x002e_1a80;
                let (fx, obj, cfg, em, vt, curve, world, g, players) = (
                    DATA,
                    DATA + 0x5000,
                    DATA + 0x5200,
                    DATA + 0x3000,
                    DATA + 0x5400,
                    DATA + 0x5600,
                    DATA + 0x6000,
                    DATA + 0x6200,
                    DATA + 0x6400,
                );
                c.bus.write32(fx + OBJ, obj);
                c.bus.write32(fx + P28, em);
                c.bus.write32(fx + P2C, curve);
                c.bus.write32(fx + CFG, cfg);
                for k in 0..4u32 {
                    c.bus.write32(obj + 0x3b0 + 4 * k, r.f32(1.0));
                    c.bus.write32(fx + DIR + 4 * k, r.f32(1.0));
                    c.bus.write32(obj + 0x370 + 4 * k, r.f32(10.0));
                    c.bus.write32(obj + 0x1e0 + 4 * k, r.f32(10.0));
                    c.bus.write32(fx + NORMAL + 4 * k, r.f32(10.0));
                    c.bus.write32(fx + POS + 4 * k, r.f32(10.0));
                    c.bus.write32(fx + COL + 4 * k, r.f32(10.0));
                }
                c.bus
                    .write32(obj + 0x1f0, f([0.0f32, 0.05, -3.0][(seed % 3) as usize]));
                c.bus.write32(
                    fx + SPEED,
                    f([300.0f32, 900.0, 3000.0][((seed / 3) % 3) as usize]),
                );
                c.bus.write32(fx + ACTIVE, (seed % 5 != 0) as u32);
                c.bus.write32(cfg + 0x50, (seed % 7 != 0) as u32);
                c.bus.write32(cfg + 0x54, r.f32(4.0));
                c.bus.write32(
                    fx + TURN,
                    f([0.05f32, 0.5, -0.1][((seed / 2) % 3) as usize]),
                );
                c.bus.write32(fx + SIDE, r.f32(1.0));
                c.bus.write32(fx + UP_SCALE, r.f32(2.0));
                c.bus.write32(obj + 0x320, seed % 2);
                c.bus.write32(obj + 0x6c0, vt);
                c.bus.write16(vt + 0x10, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                for off in [0x730u32, 0x734, 0x738] {
                    c.bus.write32(curve + off, r.f32(3.0));
                }
                put_spray_slot(c, &mut r, em + SLOT, DATA + 0x8000);
                c.bus.write32(gp_at(-0x848), world);
                c.bus.write32(world + 0x84, g);
                c.bus.write32(g + 0x84, players);
                c.bus.write32(players + 0x10, 0);
                c.bus.write32(gp_at(0xa0c), r.next_u32());
                for (off, v) in [
                    (-0x3a90, 416.66669f32),
                    (-0x3a8c, 1250.0),
                    (-0x3a88, 0.1),
                    (-0x3a84, 0.4),
                    (-0x3a80, 0.2),
                    (-0x3a7c, 1.0 / 60.0),
                    (-0x3a78, 0.3),
                    (-0x3a74, 1.0 / 60.0),
                    (-0x3a70, 1.0 / 60.0),
                ] {
                    c.bus.write32(gp_at(off), f(v));
                }
                c.g[A0][0] = sx(fx);
            },
        );
    }
    use super::*;
    use super::*;
    use crate::difftest::SP;

    use super::*;
    use super::*;
    use super::*;
    use crate::difftest::events::Events;

    /// SnowFall_Render: rendering off (`gp+0x1434`) or on; fog flag on/off;
    /// the 12 box slots empty, owned by another rider, or by the current
    /// one (WrapOffset and an inactive SnowBox_Render run for real). The
    /// view-matrix virtual is a fake that sometimes unwinds.
    #[test]
    fn snow_fall_render_matches() {
        covered(
            ee::s09::f_002e6008__generated,
            snow_fall_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e6008, 1);
                c.pc = 0x002e_6008;
                let gp = |off: i32| GAME_GP.wrapping_add(off as u32);
                let this = DATA;
                let (world, riders, list) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
                let (rc, vt, stack) = (DATA + 0x5000, DATA + 0x5f00, DATA + 0x6000);
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(gp(0x1434), if seed % 10 == 0 { 1 } else { 0 });
                c.bus.write32(gp(0x1670), r.next_u32() & 1);
                c.bus.write32(gp(-0x3a04), 0.6f32.to_bits());
                c.bus.write32(gp(-0x848), world);
                c.bus.write32(world + 0x84, riders);
                c.bus.write32(riders + 0x84, list);
                let k = r.next_u32() % 4;
                c.bus.write32(list + 0x14, k);
                for i in 0..4 {
                    let rider = DATA + 0x4400 + 0x100 * i;
                    c.bus.write32(list + 4 + 4 * i, rider);
                    fill(c, &mut r, rider, 0x40);
                }
                fill(c, &mut r, this, 0x100);
                c.bus.write32(gp(0x2a90), rc);
                c.bus
                    .write32(rc + rctx::STATE_TOP, stack + 0x14 * (r.next_u32() % 4));
                fill(c, &mut r, stack, 0x40);
                c.bus.write32(rc + rctx::TEX, r.next_u32());
                c.bus.write32(rc + rctx::VPTR, vt);
                c.bus
                    .write16(vt + rctx::GET_VIEW, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + rctx::GET_VIEW + 4, FAKE);
                fill(c, &mut r, 0x0050_1420, 8);
                fill(c, &mut r, 0x004f_f130, 4);
                for i in 0..12 {
                    let bx = DATA + 0x7000 + 0x100 * i;
                    let slot = match r.next_u32() % 3 {
                        0 => 0,
                        1 => bx,
                        _ => bx,
                    };
                    c.bus.write32(this + 0x10 + 4 * i, slot);
                    fill(c, &mut r, bx, 0x30);
                    let owner = if r.next_u32() % 3 == 0 {
                        (k + 1) % 4
                    } else {
                        k
                    };
                    c.bus.write32(bx, owner);
                    c.bus.write32(bx + 8, 0); // inactive: SnowBox_Render returns at once
                    c.bus.write32(bx + 0xc, 1.0f32.to_bits());
                }
            },
        );
    }

    /// A real leaf returning 0 (`RiderIface_HasVisited`: `v0 = 0; jr ra`),
    /// for vtable slots whose zero answer the fakes never give.
    pub(crate) const RETURN_ZERO_2: u32 = 0x0014_06c0;

    /// Write vtable slot_2 `off` (`{delta, func}`) at `vt`.
    pub(crate) fn slot_d(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, func);
    }

    /// Occluded and visible sources, behind and in front of the near plane,
    /// depths that clamp the sample rectangle both ways, every source kind,
    /// rectangles inside and outside the view; fake projector calls (some
    /// unwind).
    #[test]
    fn lens_fx_project_source_matches() {
        covered(
            ee::s09::f_002e2b00__generated,
            lens_fx_project_source,
            300,
            &|c, seed| {
                c.pc = 0x002e_2b00;
                let mut r = machine(c, seed, 0x2e2b00, 3);
                let gp = c.g[GP][0] as u32;
                let lens = DATA;
                let src = DATA + 0x200;
                let view = DATA + 0x400;
                let cam = DATA + 0x1000;
                let vt = DATA + 0x3000;
                c.bus.write32(gp + LENS_CAMERA_GP, cam);
                c.bus.write32(cam + CAM_PROJECTOR_VT, vt);
                let occluded = if seed % 5 == 0 { FAKE } else { RETURN_ZERO_2 };
                slot_d(c, vt, VT_OCCLUDED, (r.next_u32() as i16) & !3, occluded);
                slot_d(c, vt, VT_PROJECT, (r.next_u32() as i16) & !3, FAKE + 0x10);
                // Depth scale, near limit, x-offset scale.
                put_words(
                    c,
                    gp.wrapping_sub(0x3a40),
                    &[1.0f32.to_bits(), 0.001f32.to_bits(), r.f32(2.0)],
                );
                c.g[A0][0] = sx(lens);
                c.g[A1][0] = sx(view);
                c.bus.write32(lens + LENS_SOURCE, src);
                for i in 0..3 {
                    c.bus.write32(src + SRC_POS + 4 * i, r.f32(500.0));
                }
                let kind = [0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x70][(seed / 5 % 7) as usize];
                c.bus.write32(src + SRC_KIND, (r.next_u32() & !0x70) | kind);
                // The view rectangle (x, y, w, h), then +0x18/+0x28/+0x38.
                put_words(
                    c,
                    view,
                    &[r.f32(50.0), r.f32(50.0), r.f32(640.0), r.f32(480.0)],
                );
                for off in [0x18u32, 0x28, 0x38] {
                    c.bus.write32(
                        view + off,
                        if seed % 7 == 0 {
                            r.edge_f32()
                        } else {
                            r.f32(5.0)
                        },
                    );
                }
                // What the fake projector "returns" at sp+0x10: screen x, y, depth.
                let frame = STACK_TOP - 0xc0;
                let depth = match seed % 6 {
                    0 => r.f32(0.001),
                    1 => 0.0011f32.to_bits(),
                    2 => 0.05f32.to_bits(),
                    3 => 0.002f32.to_bits(),
                    4 => r.edge_f32(),
                    _ => r.f32(0.03),
                };
                put_words(c, frame + 0x10, &[r.f32(700.0), r.f32(500.0), depth]);
                c.bus.write32(frame + 0x18, depth);
                put_words(c, frame + 0x48, &[r.f32(300.0)]);
            },
        );
    }

    /// Trail lengths -1..5 (no points, one point whose t is 0/0, several),
    /// head slots 0-3, random vectors; Noise_Fractal1D runs for real.
    #[test]
    fn board_wake_fx_build_trail_matches() {
        crate::difftest::check(
            ee::s09::f_002ddd30__generated,
            board_wake_fx_build_trail,
            300,
            &|c, seed| {
                let mut r = Rng(0x2ddd_3000 ^ (seed as u64) << 16);
                crate::difftest::scramble(c, &mut r);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                let gp = crate::difftest::GAME_GP;
                let wake = DATA;
                let rider = DATA + 0x1000;
                let pos = DATA + 0x800;
                let n = [-1i32, 0, 1, 2, 3, 5][(seed % 6) as usize];
                let head = (seed / 6) % 4;
                let v = |r: &mut Rng| [r.f32(10.0), r.f32(10.0), r.f32(10.0), r.f32(2.0)];
                c.bus.write32(wake + WAKE_RIDER, rider);
                c.bus.write32(wake + WAKE_HEAD, head);
                c.bus.write32(wake + WAKE_ELEMS, DATA + 0x2000);
                for j in 0..6 {
                    c.bus
                        .write32(wake + WAKE_RINGS + 4 * j, DATA + 0x3000 + 0x400 * j);
                    let seg = DATA + 0x3000 + 0x400 * j + SEG_SIZE * head;
                    crate::difftest::fill(c, &mut r, seg, 12);
                }
                c.bus.write32(wake + WAKE_PHASE, r.f32(50.0));
                for off in [WAKE_SPREAD, WAKE_CURVE, WAKE_BACK] {
                    let q = v(&mut r);
                    crate::difftest::put_vec(c, wake + off, q);
                }
                c.bus.write32(wake + WAKE_WIDTH, r.f32(4.0));
                c.bus.write32(
                    wake + WAKE_ALPHA,
                    if seed % 9 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(2.0)
                    },
                );
                c.bus.write32(wake + WAKE_POINTS, n as u32);
                let q = v(&mut r);
                crate::difftest::put_vec(c, rider + RIDER_VEL, q);
                let q = v(&mut r);
                crate::difftest::put_vec(c, rider + RIDER_UP, q);
                let q = v(&mut r);
                crate::difftest::put_vec(c, pos, q);
                let q = v(&mut r);
                crate::difftest::put_vec(c, 0x004f_f120, q);
                put_words(c, gp - 0x3be4, &[0.85f32.to_bits(), 0.9f32.to_bits()]);
                // Noise_Sample1D's table.
                for i in 0..0x180u32 {
                    c.bus.write32(0x0044_5ab0 - 0x300 + 4 * i, r.f32(1.0));
                }
                c.g[A0][0] = sx(wake);
                c.g[A1][0] = sx(pos);
                c.f[12] = r.f32(3.0);
            },
        );
    }

    // ------------------------------------------- cDynamicSprayFX_UpdateEmitter8

    /// Emitter 8: driven or not, the target above or below 0.01, the
    /// fall-speed and speed cuts, the level's clamps and the falling alpha
    /// scale; the velocity getter is a fake returning a chosen vector, the
    /// random jitter and `DynamicEmitter_SpawnAt` on slot 8 are real.
    #[test]
    fn dynamic_spray_fx_update_emitter8_matches() {
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        use crate::difftest::{check, elf_image, DATA, GAME_GP};
        use crate::game::visualfx::boardwakefx::tests::put_spray_slot;
        use spray8::*;
        let f = |x: f32| x.to_bits();
        check(
            ee::s09::f_002e1f70__generated,
            dynamic_spray_fx_update_emitter8,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e1f, 3);
                elf_image(c);
                c.pc = 0x002e_1f70;
                let (fx, obj, cfg, em, vt, curve, vel) = (
                    DATA,
                    DATA + 0x5000,
                    DATA + 0x5200,
                    DATA + 0x3000,
                    DATA + 0x5400,
                    DATA + 0x5600,
                    DATA + 0x5800,
                );
                let (world, g, players) = (DATA + 0x6000, DATA + 0x6200, DATA + 0x6400);
                c.bus.write32(fx + OBJ, obj);
                c.bus.write32(fx + P28, em);
                c.bus.write32(fx + P2C, curve);
                c.bus.write32(fx + CFG, cfg);
                for k in 0..4u32 {
                    c.bus.write32(fx + DIR + 4 * k, r.f32(1.0));
                    c.bus.write32(fx + POS + 4 * k, r.f32(10.0));
                    c.bus.write32(fx + COL + 4 * k, r.f32(10.0));
                    c.bus.write32(fx + 0x40 + 4 * k, r.f32(1.0));
                    c.bus.write32(fx + 0x50 + 4 * k, r.f32(1.0));
                    c.bus.write32(vel + 4 * k, r.f32(100.0));
                }
                c.bus.write32(
                    vel + 8,
                    f([-5000.0f32, -1000.0, 50.0, -3333.0][(seed % 4) as usize]),
                );
                c.bus.write32(fx + ACTIVE, (seed % 3 != 0) as u32);
                c.bus.write32(
                    cfg + 0x5c,
                    f([0.005f32, 0.5, 2.0][((seed / 3) % 3) as usize]),
                );
                c.bus.write32(
                    fx + LEVEL,
                    match (seed / 9) % 5 {
                        0 => 0,
                        1 => f(0.01),
                        2 => f(0.5),
                        3 => f(1.5),
                        _ => r.edge_f32(),
                    },
                );
                c.bus.write32(
                    fx + SPEED,
                    f([100.0f32, 300.0, 2000.0][((seed / 5) % 3) as usize]),
                );
                c.bus.write32(fx + 0x12c, r.f32(1.0));
                for off in [0x818u32, 0x81c, 0x820] {
                    c.bus.write32(curve + off, r.f32(3.0));
                }
                c.bus.write32(obj + 0x6c0, vt);
                c.bus.write16(vt + 0x10, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                for n in 0..4 {
                    c.bus.write32(LOG_RESULTS + 4 * n, vel);
                }
                put_spray_slot(c, &mut r, em + SLOT, DATA + 0x8000);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, g);
                c.bus.write32(g + 0x84, players);
                c.bus.write32(players + 0x10, 0);
                c.bus.write32(GAME_GP + 0xa0c, r.next_u32());
                c.g[A0][0] = sx(fx);
            },
        );
    }

    // ------------------------------------------- LensFX_UpdateSourceVisibility

    /// `LensFX_UpdateSourceVisibility`: busy or not, occlusion on or off,
    /// 0..2 sources with queried / unqueried flags, element lists of -1..3
    /// elements; the occlusion query is a fake (a pointer, maybe unwinding)
    /// or a real `return 0` (the fallback paths).
    #[test]
    fn lens_fx_update_source_visibility_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        const RETURN_ZERO: u32 = 0x003a_5ab0;
        check(
            ee::s09::f_002e3130__generated,
            lens_fx_update_source_visibility,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e31, 8);
                c.pc = 0x002e_3130;
                let (this, vt, targets) = (DATA, DATA + 0x10000, DATA + 0x20000);
                let target = |r: &mut crate::difftest::Rng| targets + (r.next_u32() & 0xff0);
                fill(c, &mut r, targets, 0x420);
                c.bus.write32(this + 0x6494, (seed % 7 == 0) as u32);
                c.bus.write32(GAME_GP + 0xa28, (seed % 5 != 1) as u32);
                let count = match seed % 4 {
                    3 => (r.next_u32() % 3) as i32 - 1,
                    k => k as i32,
                };
                c.bus.write32(this + 0x6490, count as u32);
                c.bus.write32(this + 0x649c, vt);
                c.bus.write16(vt + 0x20, (r.next_u32() & 0x3c) as u16);
                c.bus
                    .write32(vt + 0x24, if seed % 3 == 0 { RETURN_ZERO } else { FAKE });
                for i in 0..3u32 {
                    let src = this + 0x210 * i;
                    c.bus.write16(src + 0x609e, (r.next_u32() % 3 != 0) as u16);
                    c.bus.write32(src + 0x6694, r.f32(1.0));
                    c.bus.write32(src + 0x6274, r.f32(1.0));
                    let n = (r.next_u32() % 5) as i32 - 1;
                    c.bus.write32(this + 0x60 + 4 * i, n as u32);
                    for k in 0..4u32 {
                        let el = this + 0x88 + 0x3000 * i + 0x30 * k;
                        let t = target(&mut r);
                        c.bus.write32(el, t);
                        c.bus.write16(el + 0xe, (r.next_u32() % 3 != 0) as u16);
                        let t = target(&mut r);
                        c.bus.write32(el + 0x6000, t);
                    }
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ---------------------------------------------------------- SnowFall_Update

    /// SnowFall_Update: zero to three views; the weather from the override
    /// block or from the world painter's getters (their quick path, or the
    /// slow path through a fake method); each view's splash manager (amount
    /// below or above the threshold); twelve box slots, some empty, the
    /// others real snow boxes (modes 0, 1, other; drift timer not expiring)
    /// of a view. Fakes may unwind; events are delivered at_2 both loop heads.
    #[test]
    fn snowfall_update_matches() {
        use crate::game::visualfx::boardwakefx::snow_update as snow;
        use snowfall::*;
        const SELF: u32 = 0x00a0_0000;
        const APP: u32 = 0x00a0_1000;
        const GM: u32 = 0x00a0_2000;
        const WX: u32 = 0x00a0_3000;
        const RATE_OBJ: u32 = 0x00a0_3800;
        const SPLASH: u32 = 0x00a1_0000;
        const BOX0: u32 = 0x00a2_0000;
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x2e_5da0, 8);
            let gp = |o: u32| GAME_GP.wrapping_add(o);
            let f = |x: f32| x.to_bits();
            c.bus.write32(GAME_GP - GP_APP, APP);
            c.bus.write32(APP + 0x84, GM);
            c.bus.write32(GM + 0x84, WX);
            let views = [0, 1, 2, 2, 3][(seed % 5) as usize];
            c.bus.write32(WX + 0x10, views);
            // Weather: override block or the painter (quick path or not).
            c.bus
                .write32(gp(GP_WEATHER_OVERRIDE), (seed % 3 == 0) as u32);
            for off in GP_OVERRIDE_F {
                c.bus.write32(gp(off), r.f32(2.0));
            }
            for k in 0..8 {
                c.bus.write32(OVERRIDE_C0 + 4 * k, r.f32(200.0));
            }
            c.bus.write32(gp(0xa70), (r.next_u32() % 4 != 0) as u32);
            // Splash managers: amount threshold and constants.
            c.bus.write32(gp(0x1190), r.f32(1.0));
            c.bus.write32(gp(0x1194), r.f32(2.0));
            c.bus.write32(GAME_GP - 0x38f4, r.f32(2.0));
            for i in 0..3 {
                let sm = SPLASH + 0x2000 * i;
                c.bus.write32(GM + 0x68 + 4 * i, sm);
                c.bus.write32(sm + 0x1020, r.f32(5.0));
                c.bus.write32(sm + 0x1024, r.f32(5.0));
            }
            // Snow boxes.
            c.bus.write32(gp(snow::GP_RATE_OBJ as u32), RATE_OBJ);
            c.bus.write32(RATE_OBJ + snow::RATE, 60);
            for k in 0..4 {
                c.bus.write32(snow::WIND + 4 * k, r.f32(1.0));
            }
            for i in 0..12 {
                let b = BOX0 + 0x100 * i;
                let used = r.next_u32() % 3 != 0;
                c.bus
                    .write32(SELF + BOXES + 4 * i, if used { b } else { 0 });
                c.bus.write32(b, r.next_u32() % views.max(1));
                c.bus.write32(b + snow::MODE, r.next_u32() % 3);
                c.bus.write32(b + snow::SIZE, f(100.0));
                c.bus.write32(b + snow::SPEED, r.f32(3.0));
                c.bus.write32(b + snow::WIND_SCALE, r.f32(1.0));
                c.bus.write32(b + snow::TIMER, f(1000.0));
                c.bus.write32(b + snow::PERIOD, f(2000.0));
                c.bus.write32(b + snow::SNAP, 0);
                for k in 0..4 {
                    c.bus.write32(b + snow::POS + 4 * k, r.f32(10.0));
                    c.bus.write32(b + snow::VEL + 4 * k, r.f32(2.0));
                    c.bus.write32(b + snow::PREV_VEL + 4 * k, r.f32(2.0));
                }
            }
            c.g[A0][0] = SELF as u64;
            schedule(c, &mut r, seed, 0x002e_5da0)
        };
        check_events(ee::s09::f_002e5da0__generated, snowfall_update, 200, &setup);
    }

    /// `LensFX_DrawFlareQuad`: off / on, zero or random velocity, with or
    /// without per-sprite alpha (positive, zero, negative), type bits 0x10,
    /// 0x20, 0x40 and others; the sprite draw method is a fake (sometimes
    /// unwinding) that logs the registers and the stack arguments.
    #[test]
    fn lens_fx_draw_flare_quad_matches() {
        use crate::difftest::GAME_GP;
        check(
            ee::s09::f_002e2868__generated,
            lens_fx_draw_flare_quad,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2e2868, 1);
                c.pc = 0x002e_2868;
                let fx = DATA;
                let part = DATA + 0x100;
                let gfx = DATA + 0x1000;
                let top = DATA + 0x3000;
                let vt = DATA + 0x4000;
                c.bus.write16(
                    fx + 0x2c,
                    (seed % 9 != 0) as u16 * (1 + r.next_u32() as u16 % 3),
                );
                c.bus.write16(fx + 0x2e, (seed % 2) as u16);
                c.bus.write32(fx + 0x20, part);
                c.bus.write32(fx + 0x28, r.f32(6.3));
                for k in 0..3 {
                    let v = if seed % 7 == 3 { 0 } else { r.f32(50.0) };
                    c.bus.write32(part + 0x10 + 4 * k, v);
                    c.bus.write32(part + 0x1c + 4 * k, r.f32(1000.0));
                }
                let idx = r.next_u32() % 4;
                c.g[A1][0] = idx as u64 | (r.next_u32() as u64) << 32;
                for k in 0..4 {
                    let a = match r.next_u32() % 4 {
                        0 => 0,
                        1 => r.f32(1.0) | 0x8000_0000,
                        _ => r.f32(1.0) & !0x8000_0000,
                    };
                    c.bus.write32(part + 0x40 + 4 * k, a);
                }
                let kind =
                    [0x10, 0x20, 0x40, 0x00, 0x30, 0x50, 0x60, 0x70][(seed / 2 % 8) as usize];
                c.bus.write32(part + 0xc, (r.next_u32() & !0x70) | kind);
                c.bus.write32(GAME_GP + 0x2a90, gfx);
                c.bus.write32(gfx + 0xe84, top);
                fill(c, &mut r, top, 8);
                c.bus.write32(gfx + 0x1024, r.next_u32());
                c.bus.write32(gfx + 0x1028, r.next_u32());
                c.bus.write32(gfx + 0x10d8, vt);
                c.bus.write16(vt + 0x278, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(vt + 0x27c, fakes::FAKE);
                c.g[A0][0] = fx as u64;
            },
        );
    }
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    pub(crate) use crate::game::lib::gcc::fixunssfsi;
    pub(crate) use crate::game::lib::snd::sndstrm_is_prebuffering;

    /// A uniform float in `lo..hi`.
    pub(crate) fn oc238_uf(r: &mut Rng, lo: f32, hi: f32) -> f32 {
        lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)
    }

    crate::difftest::events::driven!(oc238_gen_prebuffering, ee::s13::f_003b70f8__generated);

    crate::difftest::events::driven!(oc238_hand_prebuffering, sndstrm_is_prebuffering);

    crate::difftest::events::driven!(oc238_gen_fixunssfsi, ee::s15::f_004139f8__generated);

    crate::difftest::events::driven!(oc238_hand_fixunssfsi, fixunssfsi);

    /// LensFX_DrawFlareSet: the renderer's width / height through a real
    /// word getter (0x2743c0: positive, or with the top bit set for the
    /// unsigned-conversion path), 0..5 flares (or a negative count) of every
    /// type (-1..5), the painter's sun flare intensity and DrawSprite as
    /// fakes (some seeds unwind).
    #[test]
    fn lens_fx_draw_flare_set_matches() {
        const GET_WORD: u32 = 0x0027_43c0;
        covered(
            ee::s09::f_002f4690__generated,
            lens_fx_draw_flare_set,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_4690, 6);
                c.pc = 0x002f_4690;
                let rend = DATA + 0x1_0000;
                let rvt = DATA + 0x1_2000;
                let state = DATA + 0x1_3000;
                c.bus.write32(GAME_GP.wrapping_sub(0x854), rend);
                c.bus.write32(rend + 0x10d8, rvt);
                c.bus.write32(rend + 0xe84, state);
                c.bus.write32(rend + 0x1004, r.next_u32());
                fill(c, &mut r, state, 8);
                let dim = |r: &mut Rng, top: bool| -> u32 {
                    if top {
                        0x8000_0000 | r.next_u32()
                    } else {
                        r.next_u32() % 1024
                    }
                };
                let w = dim(&mut r, seed % 4 == 1);
                let h = dim(&mut r, seed % 5 == 2);
                c.bus.write32(rend, w);
                c.bus.write32(rend + 4, h);
                c.bus.write16(rvt + 0x50, 0);
                c.bus.write32(rvt + 0x54, GET_WORD);
                c.bus.write16(rvt + 0x58, 4);
                c.bus.write32(rvt + 0x5c, GET_WORD);
                c.bus
                    .write16(rvt + 0x270, ((r.next_u32() & 0x3f) * 4) as u16);
                c.bus.write32(rvt + 0x274, FAKE + 0x20);
                // The painter's sun flare entry (index 6).
                let p = DATA + 0x1_4000;
                let obj = DATA + 0x1_4100;
                let pvt = DATA + 0x1_4200;
                c.bus.write32(0x004f_a370 + 6 * 0xf0 + 0x14, p);
                c.bus.write32(p, obj);
                c.bus.write32(obj + 4, pvt);
                c.bus
                    .write16(pvt + 0x110, ((r.next_u32() & 0x3f) * 4) as u16);
                c.bus.write32(pvt + 0x114, FAKE + 0x10);
                // The flares.
                let fx = DATA + 0x1_8000;
                c.bus
                    .write32(fx + 0x1f0, oc238_uf(&mut r, 0.0, 640.0).to_bits());
                c.bus
                    .write32(fx + 0x1f4, oc238_uf(&mut r, 0.0, 448.0).to_bits());
                let count = match seed % 7 {
                    0 => 0,
                    1 => 0xffff_fffe,
                    _ => 1 + r.next_u32() % 5,
                };
                c.bus.write32(fx + 0x200, count);
                c.bus
                    .write32(fx + 0x204, oc238_uf(&mut r, 0.0, 2.0).to_bits());
                for k in 0..5 {
                    let f = fx + 0x1c * k;
                    let ty = (r.next_u32() % 7).wrapping_sub(1);
                    c.bus.write32(f + 0x30, ty);
                    c.bus
                        .write32(f + 0x34, oc238_uf(&mut r, -1.0, 2.0).to_bits());
                    c.bus
                        .write32(f + 0x38, oc238_uf(&mut r, 0.0, 0.5).to_bits());
                    for j in 0..4 {
                        c.bus
                            .write32(f + 0x3c + 4 * j, oc238_uf(&mut r, 0.0, 255.0).to_bits());
                    }
                }
                c.g[A0][0] = fx as u64;
            },
        );
    }

    // ------------------------------------------- cDynamicSprayFX_UpdateEmitter9

    /// `cDynamicSprayFX_UpdateEmitter9`: level zero / positive, speed below /
    /// above the limit, alpha clamped or not, the ring wrapping at 30 or not,
    /// mode 2 (no fade) or not; the real slot-9 emitter spawn runs.
    #[test]
    fn dynamic_spray_fx_update_emitter9_matches() {
        use super::spray9::*;
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, elf_image, DATA, GAME_GP};
        use crate::game::visualfx::boardwakefx::tests::put_spray_slot;
        let f = |x: f32| x.to_bits();
        check(
            ee::s09::f_002e2260__generated,
            dynamic_spray_fx_update_emitter9,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e22, 0);
                elf_image(c);
                c.pc = 0x002e_2260;
                let (fx, obj, em, curve, bones, holder, mats) = (
                    DATA,
                    DATA + 0x5000,
                    DATA + 0x2000,
                    DATA + 0x5600,
                    DATA + 0x5800,
                    DATA + 0x5900,
                    DATA + 0x9000,
                );
                let (world, g, players) = (DATA + 0x6000, DATA + 0x6200, DATA + 0x6400);
                c.bus.write32(fx + OBJ, obj);
                c.bus.write32(fx + P28, em);
                c.bus.write32(fx + P2C, curve);
                c.bus.write32(fx + BONES, bones);
                c.bus.write32(
                    fx + LEVEL,
                    match seed % 5 {
                        0 => 0,
                        1 => f(0.25),
                        2 => f(0.9),
                        3 => f(0.01),
                        _ => f(-0.5),
                    },
                );
                c.bus.write32(
                    fx + SPEED,
                    f([100.0f32, 300.0, 2000.0][((seed / 5) % 3) as usize]),
                );
                c.bus.write32(GAME_GP.wrapping_sub(0x3a4c), f(250.0));
                c.bus.write32(GAME_GP.wrapping_sub(0x3a48), f(1.0 / 60.0));
                c.bus.write32(GAME_GP.wrapping_sub(0x3a44), f(1.0 / 60.0));
                c.bus.write32(
                    fx + RING,
                    [0u32, 28, 29, r.next_u32() % 30][((seed / 15) % 4) as usize],
                );
                c.bus
                    .write32(fx + MODE, if seed % 7 == 0 { 2 } else { r.next_u32() % 4 });
                for k in 0..4u32 {
                    c.bus.write32(fx + COL + 4 * k, r.f32(10.0));
                    c.bus.write32(obj + 0x1e0 + 4 * k, r.f32(100.0));
                }
                c.bus.write32(curve + 0x900, r.f32(3.0));
                for k in 0..30u32 {
                    c.bus.write32(bones + 4 * k, r.next_u32() % 16);
                }
                c.bus.write32(obj + 0x780, holder);
                c.bus.write32(holder + 0x30, mats);
                c.bus.write32(obj + 0x89c, r.next_u32() % 16);
                for k in 0..16u32 {
                    for j in 0..4u32 {
                        c.bus.write32(mats + 0x40 * k + 0x30 + 4 * j, r.f32(50.0));
                    }
                }
                put_spray_slot(c, &mut r, em + SLOT, DATA + 0x8000);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, g);
                c.bus.write32(g + 0x84, players);
                c.bus.write32(players + 0x10, 0);
                c.bus.write32(GAME_GP + 0xa0c, r.next_u32());
                c.g[A0][0] = sx(fx);
            },
        );
    }

    /// `cBoardWakeFX_AdvanceRing`: an effect at DATA with phase 0..4.5 (wraps
    /// or not), count 0..9 against capacity 1..8 (grows or not), head below
    /// the capacity, a ring and four trail rings of random words, a random
    /// frame counter. The owner's colour source has a vtable whose colour
    /// index slot is a fake (forced index 1..12, sometimes unwinding); the
    /// colour table records hold floats in [-1.5, 1.5] or edge values, so
    /// the doubled lanes hit all three clamp cases.
    #[test]
    fn board_wake_fx_advance_ring_matches() {
        const FX: u32 = DATA;
        const OWNER: u32 = DATA + 0x1000;
        const VT: u32 = DATA + 0x3000;
        const RING: u32 = DATA + 0x4000;
        const TRAIL: u32 = DATA + 0x5000;
        covered(
            ee::s09::f_002de058__generated,
            board_wake_fx_advance_ring,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2de058, 1);
                c.pc = 0x002d_e058;
                let step = if seed % 2 == 0 {
                    0x3d08_8889
                } else {
                    r.f32(1.0)
                };
                put_words(c, GAME_GP - GP_WAKE_PHASE_STEP, &[step]);
                fill(c, &mut r, FX, 0x30);
                let cap = 1 + r.next_u32() % 8;
                c.bus.write32(FX + WAKE_OWNER, OWNER);
                c.bus.write32(FX + WAKE_COUNT, r.next_u32() % 10);
                c.bus.write32(FX + WAKE_HEAD_2, r.next_u32() % cap);
                c.bus.write32(FX + WAKE_RING, RING);
                for i in 0..4 {
                    c.bus.write32(FX + WAKE_TRAILS + 4 * i, TRAIL + 0x200 * i);
                }
                let phase = (r.next_u32() % 4500) as f32 / 1000.0;
                c.bus.write32(FX + WAKE_PHASE_2, phase.to_bits());
                c.bus.write32(
                    FX + WAKE_FRAME,
                    if seed % 5 == 0 {
                        r.next_u32()
                    } else {
                        r.next_u32() % 100_000
                    },
                );
                c.bus.write32(FX + WAKE_CAP, cap);
                fill(c, &mut r, RING, 0x60);
                fill(c, &mut r, TRAIL, 0x200);
                c.bus.write32(OWNER + OWNER_COLOUR_SOURCE, VT);
                c.bus
                    .write16(VT + SLOT_COLOUR_INDEX, (r.next_u32() as u16) & 0xfffc);
                c.bus.write32(VT + SLOT_COLOUR_INDEX + 4, FAKE);
                let index = 1 + r.next_u32() % 12;
                c.bus.write32(LOG_RESULTS, index);
                for k in 0..4 {
                    let v = if seed % 7 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(1.5)
                    };
                    c.bus
                        .write32(WAKE_COLOUR_TABLE + index * WAKE_COLOUR_STRIDE + 4 * k, v);
                }
                c.g[A0][0] = sx(FX);
            },
        );
    }

    // -------------------------------------------------------- LensFX_DrawFlares

    /// `LensFX_DrawFlares`: view 0 or 1, sun on or off, 0..5 flares (or a
    /// negative count), each off or on; the real sun draw returns at once
    /// (its "on" halfword is 0), the real flare-quad draw runs on random
    /// particles of every type (its sprite draw is a fake), and the push /
    /// pop render-state methods are fakes (some calls unwind).
    #[test]
    fn lens_fx_draw_flares_matches() {
        covered(
            ee::s09::f_002e3478__generated,
            lens_fx_draw_flares,
            300,
            &|c, seed| {
                use crate::difftest::fakes::vtable;
                let mut r = machine(c, seed, 0x002e_3478, 7);
                c.pc = 0x002e_3478;
                let fx = DATA + 0x30_0000;
                let view = seed % 2;
                c.bus.write32(
                    GAME_GP + 0xa28,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
                let vt = vtable(c, &mut r, &[0x10, 0x18]);
                c.bus.write32(fx + 0x649c, vt);
                let count = if seed % 13 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    seed / 2 % 6
                };
                c.bus.write32(fx + 0x60 + 4 * view, count);
                // The renderer the flare quads draw with (as in the
                // `LensFX_DrawFlareQuad` test).
                let gfx = DATA + 0x20_0000;
                let top = DATA + 0x20_3000;
                let gvt = DATA + 0x20_4000;
                c.bus.write32(GAME_GP + 0x2a90, gfx);
                c.bus.write32(gfx + 0xe84, top);
                fill(c, &mut r, top, 8);
                c.bus.write32(gfx + 0x1024, r.next_u32());
                c.bus.write32(gfx + 0x1028, r.next_u32());
                c.bus.write32(gfx + 0x10d8, gvt);
                c.bus
                    .write16(gvt + 0x278, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(gvt + 0x27c, FAKE + 0x20);
                for k in 0..6 {
                    let flare = fx + 0x68 + view * 0x3000 + k * 0x30;
                    let part = DATA + 0x21_0000 + k * 0x100;
                    let on = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        1 + r.next_u32() as u16 % 3
                    };
                    c.bus.write16(flare + 0x2c, on);
                    c.bus.write16(flare + 0x2e, (r.next_u32() % 2) as u16);
                    c.bus.write32(flare + 0x20, part);
                    c.bus.write32(flare + 0x28, r.f32(6.3));
                    for j in 0..3 {
                        let v = if r.next_u32() % 7 == 3 {
                            0
                        } else {
                            r.f32(50.0)
                        };
                        c.bus.write32(part + 0x10 + 4 * j, v);
                        c.bus.write32(part + 0x1c + 4 * j, r.f32(1000.0));
                    }
                    for j in 0..4 {
                        let a = match r.next_u32() % 4 {
                            0 => 0,
                            1 => r.f32(1.0) | 0x8000_0000,
                            _ => r.f32(1.0) & !0x8000_0000,
                        };
                        c.bus.write32(part + 0x40 + 4 * j, a);
                    }
                    let kind = [0x10, 0x20, 0x40, 0x00, 0x30, 0x50, 0x60, 0x70]
                        [(r.next_u32() % 8) as usize];
                    c.bus.write32(part + 0xc, (r.next_u32() & !0x70) | kind);
                }
                c.g[A0][0] = sx(fx);
                c.g[A1][0] = view as u64;
            },
        );
    }

    /// `LensFX_PrepareView` for views 0 and 1: the camera interface's view
    /// rect / matrix / FOV slots and the projector's occlusion slot are
    /// fakes (the matrix call returns a random matrix; occlusion is always
    /// reported, so the real `LensFX_ProjectSource` takes its short path);
    /// the painter's sun texture is -1 (the real `LensFX_UpdateSunPosition`
    /// hides the sun); zero, negative and 1-4 flare sources; unwinds in the
    /// first four calls.
    #[test]
    fn lens_fx_prepare_view_matches() {
        use super::lens_prep::*;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::GAME_GP;
        use crate::game::visualfx::lensfx::tests::painter_sun;
        covered(
            ee::s09::f_002e3338__generated,
            lens_fx_prepare_view,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002e_3338, 4);
                c.pc = 0x002e_3338;
                let fx = DATA + 0x4_0000;
                let cam = DATA + 0x5_0000;
                let vt = DATA + 0x5_1000;
                let no_sun = DATA + 0x5_4000;
                let src = DATA + 0x5_5000;
                let matrix = DATA + 0x5_6000;
                let view = seed % 2;
                c.g[A0][0] = sx(fx);
                c.g[A1][0] = view as u64;
                // The camera and its interface.
                c.bus.write32(GAME_GP + CAMERA_GP, cam);
                c.bus.write32(cam + CAM_VT, vt);
                for (i, off) in [VT_VIEW_RECT, VT_VIEW_MATRIX, VT_FOV, 0x2d8]
                    .into_iter()
                    .enumerate()
                {
                    let delta = (r.next_u32() as i16 as i32 % 64) & !3;
                    c.bus.write16(vt + off, delta as u16);
                    c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
                }
                fill(c, &mut r, matrix, 16);
                c.bus.write32(LOG_RESULTS + 4, matrix);
                // Painter views 6 and 7: sun texture -1.
                for v in [6, 7] {
                    painter_sun(c, &mut r, v, DATA + 0x5_2000, DATA + 0x5_3000, &[0x108]);
                }
                c.bus.write32(no_sun, u32::MAX);
                c.bus.write32(LOG_RESULTS + 12, no_sun);
                // The lens FX: header, counts, flares pointing at one source.
                fill(c, &mut r, fx, 0x18);
                fill(c, &mut r, src, 0x10);
                let count = match seed % 4 {
                    0 => 0,
                    1 => (r.next_u32() % 5 + 1).wrapping_neg(),
                    _ => 1 + r.next_u32() % 4,
                };
                c.bus.write32(fx + COUNTS + 4 * view, count);
                for k in 0..4 {
                    let flare = fx + FLARES + view * FLARE_VIEW_SIZE as u32 + k * FLARE_SIZE;
                    fill(c, &mut r, flare, 0xc);
                    c.bus.write32(flare + 0x20, src);
                }
            },
        );
    }
    use super::*;


    // ------------------------------------------------ cDynamicSprayFX_QueueBurst

    /// `cDynamicSprayFX_QueueBurst`: weaker and stronger bursts, near and
    /// far positions, the short-range flag, every step of the rider's
    /// carving test, `b` zero and non-zero.
    #[test]
    fn dynamic_spray_queue_burst_matches() {
        use crate::difftest::{check, put_vec, scramble, Rng, DATA};
        check(
            ee::s09::f_002e23e0__generated,
            dynamic_spray_queue_burst,
            300,
            &|c, seed| {
                let mut r = Rng(0x002e_23e0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x002e_23e0;
                let this = DATA;
                let rider = DATA + 0x1000;
                let pos = DATA + 0x400;
                let dir = DATA + 0x410;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(dir);
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                let speed = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(50.0)
                };
                c.f[12] = speed;
                c.bus.write32(
                    this + SPRAY_STRENGTH,
                    if seed % 5 == 0 {
                        r.f32(100.0)
                    } else {
                        r.f32(10.0)
                    },
                );
                c.bus.write32(
                    this + SPRAY_SHORT_RANGE,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(this + SPRAY_RIDER, rider);
                // Queued position near or far from the new one.
                let spread = match seed % 4 {
                    0 => 30.0,
                    1 => 120.0,
                    2 => 300.0,
                    _ => 1000.0,
                };
                put_vec(
                    c,
                    this + SPRAY_POS,
                    [r.f32(spread), r.f32(spread), r.f32(spread), r.next_u32()],
                );
                put_vec(
                    c,
                    this + SPRAY_DIR,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                put_vec(
                    c,
                    pos,
                    [r.f32(spread), r.f32(spread), r.f32(spread), r.next_u32()],
                );
                put_vec(
                    c,
                    dir,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                let k = seed / 4;
                let pick = |bit: u32, r: &mut Rng| if k & bit == 0 { 0 } else { r.next_u32() | 1 };
                let a = pick(1, &mut r);
                let block = if k & 2 == 0 { 0 } else { r.next_u32() | 1 };
                let b = pick(4, &mut r);
                let cc = pick(8, &mut r);
                c.bus.write32(rider + RIDER_CARVE_A, a);
                c.bus.write32(rider + RIDER_CARVE_BLOCK, block);
                c.bus.write32(rider + RIDER_CARVE_B, b);
                c.bus.write32(rider + RIDER_CARVE_C, cc);
            },
        );
    }
}
