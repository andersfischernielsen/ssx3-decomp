//! `src/ai/rider.cpp`: hand-written replacements (moved from the
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
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::control::handplantcontrol::gp_4;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_4;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::debugfe::at as at_3;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_6;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_6;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_3;
pub(crate) use crate::game::fe::festateloadhint::gp as gp_5;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_5;
pub(crate) use crate::game::fe::festaterewards::gp_4 as gp_7;
pub(crate) use crate::game::fe::festaterewards::lw_3;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2 as sp_3;
pub(crate) use crate::game::render::particle::at as at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at as sw_at_2;
pub(crate) use crate::game::visualfx::boardwakefx::lh;
pub(crate) use crate::game::visualfx::boardwakefx::lw_2;
pub(crate) use crate::game::visualfx::boardwakefx::r32_2 as r32_3;
pub(crate) use crate::game::visualfx::boardwakefx::sw;
pub(crate) use crate::game::visualfx::boardwakefx::sw_at;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
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
        0x0010_4e70,
        rider_find_wall_contact,
        "Rider_FindWallContact",
    ),
    (
        0x0010_5398,
        rider_handle_wall_collision,
        "Rider_HandleWallCollision",
    ),
    (
        0x0010_7888,
        rider_check_rider_collisions,
        "Rider_CheckRiderCollisions",
    ),
    (0x0011_3f38, rider_set_slew5_target, "Rider_SetSlew5Target"),
    (
        0x0011_5b58,
        rider_update_timed_event_anim,
        "Rider_UpdateTimedEventAnim",
    ),
    (
        0x0011_5d48,
        rider_update_idle_look_anim,
        "Rider_UpdateIdleLookAnim",
    ),
    (
        0x0011_6930,
        rider_empty_stance_hook,
        "Rider_EmptyStanceHook",
    ),
    (0x0011_bb40, rider_create_focus_box, "Rider_CreateFocusBox"),
    (0x0011_c0e0, rider_has_render_slot, "Rider_HasRenderSlot"),
    (0x0011_de60, rider_place_at_marker, "Rider_PlaceAtMarker"),
    (0x0011_df18, rider_reset_motion, "Rider_ResetMotion"),
    (0x0011_e150, rider_update_bounds, "Rider_UpdateBounds"),
    (0x0011_eb98, rider_update_skeleton, "Rider_UpdateSkeleton"),
    (0x0011_f3d8, rider_solve_leg_ik, "Rider_SolveLegIK"),
    (
        0x0011_fa10,
        rider_update_board_lean,
        "Rider_UpdateBoardLean",
    ),
    (
        0x0011_fea8,
        rider_get_main_state_name,
        "Rider_GetMainStateName",
    ),
    (0x0011_ff98, rider_get_mass, "cRider_getMass"),
    (
        0x0012_00d0,
        rider_update_boost_timers,
        "Rider_UpdateBoostTimers",
    ),
    (
        0x0012_0378,
        rider_update_speed_anims,
        "Rider_UpdateSpeedAnims",
    ),
    (0x0012_0d90, rider_update_leg_ik_blend, "func_00120D90"),
    (0x0012_0e50, rider_refresh_overlaps, "Rider_RefreshOverlaps"),
    (
        0x0012_11f8,
        rider_update_slewed_params,
        "Rider_UpdateSlewedParams",
    ),
    (
        0x0012_1950,
        rider_update_heading_and_audio,
        "Rider_UpdateHeadingAndAudio",
    ),
    (0x0012_1f30, rider_compute_lod, "Rider_ComputeLOD"),
    (
        0x0012_2278,
        rider_update_visibility,
        "Rider_UpdateVisibility",
    ),
    (0x0012_2448, rider_render_model, "Rider_RenderModel"),
    (
        0x0012_2c28,
        rider_get_start_marker_pos,
        "Rider_GetStartMarkerPos",
    ),
    (0x0012_2c98, rider_is_spinning_fast, "Rider_IsSpinningFast"),
    (0x0012_2cf0, rider_update_danger_state, "func_00122CF0"),
    (
        0x0012_2d78,
        rider_estimate_finish_time,
        "Rider_EstimateFinishTime",
    ),
    (
        0x0012_2e50,
        rider_estimate_final_score,
        "Rider_EstimateFinalScore",
    ),
    (0x0012_35f8, rider_force_reset, "Rider_ForceReset"),
    (
        0x0012_3da8,
        rider_update_from_script,
        "Rider_UpdateFromScript",
    ),
    (0x0012_41c0, quat_from_euler, "Quat_FromEuler"),
    (0x0012_5038, rider_stop_transition, "Rider_StopTransition"),
    (0x0012_50a8, rider_add_velocity, "Rider_AddVelocity"),
    (
        0x0012_5228,
        rider_check_race_finish,
        "Rider_CheckRaceFinish",
    ),
    (
        0x0012_53d0,
        rider_mark_quit_requested,
        "Rider_MarkQuitRequested",
    ),
    (
        0x0012_5958,
        rider_set_boost_enabled,
        "Rider_SetBoostEnabled",
    ),
    (0x0012_5970, rider_apply_wind_force, "Rider_ApplyWindForce"),
    (
        0x0012_5ad0,
        rider_empty_progress_hook,
        "Rider_EmptyProgressHook",
    ),
    (
        0x0012_5ad8,
        rider_get_sync_checksum,
        "Rider_GetSyncChecksum",
    ),
    (
        0x0012_76f0,
        player_get_trick_button,
        "Player_GetTrickButton",
    ),
    (
        0x0012_7848,
        player_get_uber_grind_button,
        "Player_GetUberGrindButton",
    ),
    (0x0012_78d0, player_pulse_rumble, "Player_PulseRumble"),
    (
        0x002a_3c00,
        ssx_audio_system_arcade_say20a8,
        "SSXAudioSystem_ArcadeSay20A8",
    ),
];

// ---------------------------------------------------------------------------
// Direct-call targets (the generated body's `c.pc = addr; Name(c)` calls).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    use super::*;
    pub(crate) use crate::game::ai::ai::callee::AI_RESET_RIDERS;
    pub(crate) use crate::game::ai::ai::callee::AI_SET_RIDER_ORDER_MODE;
    pub(crate) use crate::game::ai::ai::callee::BE_COPY_PROFILE_CHARACTERS;
    pub(crate) use crate::game::ai::ai::callee::INTC_RESTORE_REGS;
    pub(crate) use crate::game::ai::ai::callee::INTC_SAVE_REGS;
    pub(crate) use crate::game::ai::ai::callee::IRRADIANCE_FIND_INDEX;
    pub(crate) use crate::game::ai::ai::callee::IRRADIANCE_GET_ENTRY;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_FLUSH_BUFFER32;
    pub(crate) use crate::game::ai::ai::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::ai::ai::callee::RIDER_FIND_NEARBY_LIGHTS;
    pub(crate) use crate::game::ai::ai::callee::WORLD_CACHE_FREE;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE as RIDER_GET_MAIN_STATE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_NEXT_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_PREV_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_NODE_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILD_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_GET_RIDER_PREVIEW_2 as FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUTSYNC_IS_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_HEAP_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_FIND_ENABLED_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED_3 as MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_GRID_MESH_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BASE_RESET_FX;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_CATEGORY_TITLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_BUY_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_EQUIP_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_GET_PATH_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::START_SCREEN_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::BECHAR_GET_OVERALL_RATING;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_GET_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_SPEND_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_GET_FILE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_SET_BIT;
    pub(crate) use crate::game::fe::festaterewards::callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_OPPONENT_DISCONNECTED;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SERVER_TIMEOUT;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SHOW_CANCEL;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_LANGUAGE_MASK_2 as GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_ARCADE_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_DJ_SPEECH_ON_2 as IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_IS_FLAG_SET;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_POST;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_SET_FLAG;
    pub(crate) use crate::game::fe::festaterewards::callee::MOD_OBJ_RUN_SCRIPT_EVENT;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_REGISTER_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_POP_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_START_SCRIPT;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_STOP_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_COPY_TAG;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_SET_RAW as TAG_FIELD_SET_RAW_2;
    pub(crate) use crate::game::fe::festaterewards::callee::TEXFLIP_SET_FRAME;
    pub(crate) use crate::game::fe::festaterewards::callee::VFX_PLANTS_GET_SWAY_MATRIX;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::ANIM_SEQ_LIST_GET_AT;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::EMITTER_UPDATE;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::MATH_COS;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::RIDER_COMPUTE_LOD;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::RIDER_MODEL_RENDER;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::TRANSITION_EFFECT_UPDATE_SCREEN_RECT;
    pub(crate) use crate::game::visualfx::boardwakefx::callee::WORLD_PAINTER_QUERY_RESET_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE as BELIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE as BELIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR;
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
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER_2;
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
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;

    pub const RIDER_GET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fee8,
        f: ee::s00::Rider_GetSubState,
    };

    pub const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };

    pub const RIDER_UPDATE_BOARD_LEAN: Callee = Callee {
        addr: 0x0011_fa10,
        f: ee::s00::Rider_UpdateBoardLean,
    };

    pub const RIDER_SOLVE_LEG_IK: Callee = Callee {
        addr: 0x0011_f3d8,
        f: ee::s00::Rider_SolveLegIK,
    };

    pub const RIDER_UPDATE_SPHERE_TREE: Callee = Callee {
        addr: 0x0010_6828,
        f: ee::s00::Rider_UpdateSphereTree,
    };

    pub const RIDER_UPDATE_SPEED_ANIMS: Callee = Callee {
        addr: 0x0012_0378,
        f: ee::s00::Rider_UpdateSpeedAnims,
    };

    pub const AI_GET_FRAME_COUNTER: Callee = Callee {
        addr: 0x0012_98c8,
        f: ee::s00::AI_GetFrameCounter,
    };

    pub const RIDER_METRIX_BANK_COMBO: Callee = Callee {
        addr: 0x0011_7718,
        f: ee::s00::RiderMetrix_BankCombo,
    };

    pub const RIDER_METRIX_CHECK_SCORE_MILESTONE: Callee = Callee {
        addr: 0x0011_9210,
        f: ee::s00::RiderMetrix_CheckScoreMilestone,
    };

    pub const RIDER_METRIX_UPDATE_TRICK_HUD: Callee = Callee {
        addr: 0x0011_7fe0,
        f: ee::s00::RiderMetrix_UpdateTrickHUD,
    };

    pub const CHUDFX_UPDATE: Callee = Callee {
        addr: 0x0011_6fb8,
        f: ee::s00::cHUDFX_Update,
    };

    // Cross-shard direct calls the generator emitted as `call_indirect`.
    pub const RIDER_SPIN_SUB_APPLY_POSE_ROTATION: u32 = 0x0013_4dd0;

    pub const RIDER_MODEL_APPLY_POSE: u32 = 0x0031_0200;

    pub const QUAT_SLERP: u32 = 0x0031_bcb0;

    pub const MATH_SIN_COS: u32 = 0x0031_be50;

    pub const MATH_ASIN: u32 = 0x0031_c128;

    pub const ANIM_CONTROLLER_GET_LAYER_SEQUENCE: u32 = 0x0031_1b20;

    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;

    pub const ANIM_CONTROLLER_PLAY_ANIM: u32 = 0x0031_28e8;

    pub const MEMSET: u32 = 0x0041_6210;

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;

    pub const SSX_AUDIO_SYSTEM_SET_PLAYER_TRICK_SCORE: u32 = 0x0029_0f58;

    pub const GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };

    pub const SET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe78,
        f: ee::s00::Rider_SetMainState,
    };

    pub const GET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fee8,
        f: ee::s00::Rider_GetSubState,
    };

    pub const SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::Rider_SetSubState,
    };

    pub const IS_ON_GROUND: Callee = Callee {
        addr: 0x0012_31a8,
        f: ee::s00::Rider_IsOnGround,
    };

    pub const HANDLE_WALL_RAIL_CONTACT: Callee = Callee {
        addr: 0x0010_6f78,
        f: ee::s00::Rider_HandleWallRailContact,
    };

    pub const FIND_WALL_CONTACT: Callee = Callee {
        addr: 0x0010_4e70,
        f: ee::s00::Rider_FindWallContact,
    };

    pub const BOUNCE_OFF_WALL: Callee = Callee {
        addr: 0x0010_57b8,
        f: ee::s00::Rider_BounceOffWall,
    };

    pub const FIND_RAIL: Callee = Callee {
        addr: 0x0010_8a48,
        f: ee::s00::Rider_FindRail,
    };

    pub const TRANSLATE: Callee = Callee {
        addr: 0x0010_6538,
        f: ee::s00::Rider_Translate,
    };

    pub const DEFLECT_ALONG_WALL: Callee = Callee {
        addr: 0x0010_65b0,
        f: ee::s00::Rider_DeflectAlongWall,
    };

    pub const BUILD_WORLD_MATRIX: Callee = Callee {
        addr: 0x0011_e098,
        f: ee::s00::Rider_BuildWorldMatrix,
    };

    pub const HANDLE_IMPACT: Callee = Callee {
        addr: 0x0010_5d98,
        f: ee::s00::Rider_HandleImpact,
    };

    pub const BANK_TRICK_BOOST: Callee = Callee {
        addr: 0x0010_e910,
        f: ee::s00::Rider_BankTrickBoost,
    };

    pub const LAND_TRICK: Callee = Callee {
        addr: 0x0011_9d40,
        f: ee::s00::RiderMetrix_LandTrick,
    };

    pub const ADD_BOOST: Callee = Callee {
        addr: 0x0010_e098,
        f: ee::s00::Rider_AddBoost,
    };

    pub const ALIGN_STANCE_TO_RAIL: Callee = Callee {
        addr: 0x0011_5358,
        f: ee::s00::Rider_AlignStanceToRail,
    };

    pub const ROTATE_AROUND_AXIS: Callee = Callee {
        addr: 0x0011_dfe0,
        f: ee::s00::Rider_RotateAroundAxis,
    };

    pub const GET_BUMP_STRENGTH: Callee = Callee {
        addr: 0x0011_ff98,
        f: ee::s00::Rider_GetBumpStrength,
    };

    pub const AI_GET_FRAME_COUNTER_2: Callee = Callee {
        addr: 0x0012_98c8,
        f: ee::s00::AI_GetFrameCounter,
    };

    pub const RESOLVE_RIDER_COLLISION: Callee = Callee {
        addr: 0x0010_7e70,
        f: ee::s00::Rider_ResolveRiderCollision,
    };

    // Cross-shard direct calls the generator emitted as `call_indirect`.
    pub const COLL_SEGMENT_CTOR: u32 = 0x0032_e100;

    pub const COLL_HULL_CTOR: u32 = 0x0032_f650;

    pub const COLL_HULL_DTOR: u32 = 0x0032_f708;

    pub const OVERLAP_LIST_COLLIDE_VOLUME: u32 = 0x0033_3ef8;

    pub const OVERLAP_LIST_COLLIDE_OBJECTS: u32 = 0x0033_4458;

    pub const OVERLAP_LIST_FIND_NEAREST_RAIL: u32 = 0x0033_4680;

    pub const CAMERA_TRIGGER_UPDATE_FOR_RIDER: u32 = 0x0016_d320;

    pub const PLAY_WALL_HIT_SOUND: u32 = 0x0029_6088;

    pub const MATH_ATAN: u32 = 0x0031_c228;

    pub const ANIM_PLAY_ANIM: u32 = 0x0031_28e8;

    pub const ANIM_ROTATE_ROOTS: u32 = 0x0031_1b48;

    pub const ANIM_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;

    pub const ANIM_LAYER_SEQUENCE: u32 = 0x0031_1b20;

    pub const FLAGS64_TEST: u32 = 0x0014_46a0;

    pub const RAILSLIDE_PLAY_ENTER_ANIM: u32 = 0x0013_26c8;

    pub const RAILSLIDE_COMPUTE_ENTRY_ANGLE: u32 = 0x0013_adc0;

    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const BE_CHAR_ATTRIB5_FACTOR: u32 = 0x0014_9038;

    pub const SPHERE_TREE_INTERSECT: u32 = 0x0032_9f98;

    pub const SSX_AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

    pub const SSX_AUDIO_UPDATE_RIDER_WORLD_PAINT: u32 = 0x0028_98a8;

    pub const WORLD_PAINTER_WEATHER28: u32 = 0x002e_f0a0;

    pub const WORLD_PAINTER_WEATHER20: u32 = 0x002e_e570;

    pub const BELIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

    pub const BELIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const FIXUNSSFSI: u32 = 0x0041_39f8;

    pub const SSX_AUDIO_ARCADE_SAY: u32 = 0x002a_3c00;

    pub const PATH_GET_CLOSEST_ON_PATH: u32 = 0x0026_a638;

    pub const PATH_GET_CLOSEST: u32 = 0x0026_a428;

    pub const PATH_GET_POINT_AT_DISTANCE: u32 = 0x0026_ab20;

    pub const PATH_GET_LENGTH: u32 = 0x0026_ac48;

    pub const EA_MEM_COPY: u32 = 0x003e_6574;

    pub const RIDER_IS_ON_GROUND: Callee = Callee {
        addr: 0x0012_31a8,
        f: ee::s00::Rider_IsOnGround,
    };

    pub const RIDER_UPDATE_DANGER_STATE: Callee = Callee {
        addr: 0x0012_2cf0,
        f: ee::s00::Rider_UpdateDangerState,
    };

    pub const RIDER_SET_TIMED_EVENT: Callee = Callee {
        addr: 0x0010_e028,
        f: ee::s00::Rider_SetTimedEvent,
    };

    pub const RIDER_UPDATE_DIST_TO_FINISH: Callee = Callee {
        addr: 0x0011_2fb0,
        f: ee::s00::Rider_UpdateDistToFinish,
    };

    pub const RIDER_SELECT_RACE_PATH: Callee = Callee {
        addr: 0x0011_27f0,
        f: ee::s00::Rider_SelectRacePath,
    };

    pub const RIDER_HANDLE_RACE_PATH_EVENT: Callee = Callee {
        addr: 0x0010_e5d8,
        f: ee::s00::Rider_HandleRacePathEvent,
    };

    pub const RIDER_RETURN_ZERO: Callee = Callee {
        addr: 0x0011_25b8,
        f: ee::s00::Rider_ReturnZero_25b8,
    };

    pub const RIDER_AI_LOOK_AHEAD: Callee = Callee {
        addr: 0x0011_2588,
        f: ee::s00::Rider_GetAIPathLookAheadDist,
    };

    pub const RIDER_SAVE_AI_LOOK_AHEAD: Callee = Callee {
        addr: 0x0011_25a8,
        f: ee::s00::Rider_SaveAIPathLookAhead,
    };

    pub const RIDER_SELECT_AI_PATH: Callee = Callee {
        addr: 0x0011_2a50,
        f: ee::s00::Rider_SelectAIPath,
    };

    pub const RIDER_ON_FINISH_RACE: Callee = Callee {
        addr: 0x0012_5108,
        f: ee::s00::Rider_OnFinishRace,
    };

    pub const EMITTER_UPDATE_DESCRIPTOR: Callee = Callee {
        addr: 0x0036_d428,
        f: ee::s12::Emitter_UpdateDescriptor,
    };

    pub const EMITTER_ADVANCE_DESC_TIME: Callee = Callee {
        addr: 0x0036_d3e8,
        f: ee::s12::Emitter_AdvanceDescTime,
    };

    pub const PAD_QUEUE_ADVANCE_WRITE: Callee = Callee {
        addr: 0x0032_6b48,
        f: ee::s10::PadQueue_AdvanceWrite,
    };

    pub const PAD_QUEUE_GET_PAD_STATUS: Callee = Callee {
        addr: 0x0032_6ca0,
        f: ee::s10::PadQueue_GetPadStatus,
    };

    pub const PAD_QUEUE_GET_PAD_DATA: Callee = Callee {
        addr: 0x0032_6cc8,
        f: ee::s10::PadQueue_GetPadData,
    };

    pub const INPUT_PAD_UPDATE: Callee = Callee {
        addr: 0x0032_1298,
        f: ee::s10::InputPad_Update,
    };

    pub const COMM_SYSTEM_UPDATE: Callee = Callee {
        addr: 0x0025_5a20,
        f: ee::s06::cCommSystem_Update,
    };

    pub const USB_KEYBOARD_UPDATE: Callee = Callee {
        addr: 0x0026_68e8,
        f: ee::s07::cUsbKeyboard_Update,
    };

    pub const MATH_SIN_COS_2: Callee = Callee {
        addr: 0x0031_be50,
        f: ee::s10::Math_SinCos,
    };

    pub const EMITTER_RENDER: Callee = Callee {
        addr: 0x0037_1688,
        f: ee::s12::mEmitter_Render,
    };

    pub const VU0_SAVE_CONTEXT: Callee = Callee {
        addr: 0x0031_a308,
        f: ee::s10::VU0_SaveContext,
    };

    pub const VU0_RESTORE_CONTEXT: Callee = Callee {
        addr: 0x0031_a3c0,
        f: ee::s10::VU0_RestoreContext,
    };

    pub const MODIFIER_BLOCK_GET_TRANSFORM_BOUNDS: Callee = Callee {
        addr: 0x0035_2b88,
        f: ee::s11::ModifierBlock_GetTransformBounds,
    };

    pub const NET_ACCOUNT_MEM_CTOR: Callee = Callee {
        addr: 0x001b_6ed0,
        f: ee::s03::cFEStateNetAccountMem_ctor,
    };

    pub const SHOW_CHAR_STATS: Callee = Callee {
        addr: 0x001b_1b98,
        f: ee::s03::cFEStateOnlineCharacterSelect_ShowCharStats,
    };

    pub const START_CARD_SCAN: Callee = Callee {
        addr: 0x001b_5268,
        f: ee::s03::cFEStateNetConfig_StartCardScan,
    };

    pub const SHOW_CHECKING_CARD_POPUP: Callee = Callee {
        addr: 0x001b_47c0,
        f: ee::s03::cFEStateNetConfig_ShowCheckingCardPopup,
    };

    pub const CREATE_STEP1_SET_ACCOUNT_NAME: Callee = Callee {
        addr: 0x001b_95c0,
        f: ee::s03::cFEStateCreateStep1_SetAccountNameText,
    };

    pub const CREATE_STEP1_SET_PASSWORD: Callee = Callee {
        addr: 0x001b_9610,
        f: ee::s03::cFEStateCreateStep1_SetPasswordText,
    };

    pub const CREATE_STEP1_SET_CONFIRM_PASSWORD: Callee = Callee {
        addr: 0x001b_9678,
        f: ee::s03::cFEStateCreateStep1_SetConfirmPasswordText,
    };

    pub const CREATE_STEP1_SET_EMAIL: Callee = Callee {
        addr: 0x001b_96e0,
        f: ee::s03::cFEStateCreateStep1_SetEmailText,
    };

    pub const CREATE_STEP1_SET_SHARE_INFO: Callee = Callee {
        addr: 0x001b_97b0,
        f: ee::s03::cFEStateCreateStep1_SetShareInfoSelection,
    };

    pub const CREATE_STEP1_SET_INFORM: Callee = Callee {
        addr: 0x001b_9740,
        f: ee::s03::cFEStateCreateStep1_SetInformSelection,
    };

    pub const CREATE_STEP1_BACKUP_ACCOUNT: Callee = Callee {
        addr: 0x001b_a178,
        f: ee::s03::cFEStateCreateStep1_BackupAccountInfo,
    };

    pub const CREATE_STEP1_ALL_FIELDS_FILLED: Callee = Callee {
        addr: 0x001b_9558,
        f: ee::s03::cFEStateCreateStep1_AllFieldsFilled,
    };

    pub const LOGIN_ON_ACCOUNT_MEM_POPUP: Callee = Callee {
        addr: 0x001b_f438,
        f: ee::s03::cFEStateLogin_OnAccountMemPopupEvent,
    };

    pub const LOGIN_ON_DISCONNECT_POPUP: Callee = Callee {
        addr: 0x001b_f5a8,
        f: ee::s03::cFEStateLogin_OnDisconnectPopupEvent,
    };

    pub const MESSAGE_CENTER_GET_LIST_INDEX: Callee = Callee {
        addr: 0x001e_3680,
        f: ee::s04::MessageCenter_GetListIndex,
    };

    pub const MESSAGE_CENTER_OPEN_VIEW_POPUP: Callee = Callee {
        addr: 0x001e_50c8,
        f: ee::s04::OVScreenMessageCenter_OpenViewPopup,
    };

    pub const LOBBY_MAN_SEND_REQUEST: Callee = Callee {
        addr: 0x0025_b608,
        f: ee::s06::LobbyMan_SendRequest,
    };

    pub const LOBBY_MAN_GET_LANGUAGE: Callee = Callee {
        addr: 0x0025_9198,
        f: ee::s06::LobbyMan_GetLanguage,
    };

    pub const BUDDY_MAN_GET_ROSTER_COUNT: Callee = Callee {
        addr: 0x0026_2598,
        f: ee::s07::BuddyMan_GetRosterCount,
    };

    pub const BUDDY_MAN_GET_ROSTER_ENTRY: Callee = Callee {
        addr: 0x0026_25c8,
        f: ee::s07::BuddyMan_GetRosterEntry,
    };

    pub const SSX_AUDIO_GET_SURFACE_SOUND_TYPE: Callee = Callee {
        addr: 0x0029_1710,
        f: ee::s07::SSXAudioSystem_GetSurfaceSoundType,
    };

    pub const SSX_AUDIO_START_BOARD_BASE_LOOP: Callee = Callee {
        addr: 0x0029_17d0,
        f: ee::s07::SSXAudioSystem_StartBoardBaseLoop,
    };

    pub const SSX_AUDIO_START_BOARD_SURFACE_LOOP: Callee = Callee {
        addr: 0x0029_1c88,
        f: ee::s07::SSXAudioSystem_StartBoardSurfaceLoop,
    };

    pub const SSX_AUDIO_START_BOARD_EDGE_LOOP: Callee = Callee {
        addr: 0x0029_2508,
        f: ee::s07::SSXAudioSystem_StartBoardEdgeLoop,
    };

    pub const SSX_AUDIO_PLAY_FE_SOUND: Callee = Callee {
        addr: 0x0029_4f78,
        f: ee::s08::SSXAudioSystem_PlayFESound,
    };

    pub const SSX_AUDIO_STOP_WIPEOUT_LOOP: Callee = Callee {
        addr: 0x0029_6e20,
        f: ee::s08::SSXAudioSystem_StopWipeoutLoop,
    };

    pub const SSX_AUDIO_STOP_WIPEOUT_LOOP2: Callee = Callee {
        addr: 0x0029_7438,
        f: ee::s08::SSXAudioSystem_StopWipeoutLoop2,
    };

    pub const SSX_AUDIO_STOP_TRICK_METER_SOUND: Callee = Callee {
        addr: 0x0029_b3c0,
        f: ee::s08::SSXAudioSystem_StopTrickMeterSound,
    };

    pub const SCRIPT_OBJ_CTOR: Callee = Callee {
        addr: 0x0028_3200,
        f: ee::s07::ScriptObj_ctor,
    };

    pub const SCRIPT_OBJ_DEACTIVATE: Callee = Callee {
        addr: 0x0028_3440,
        f: ee::s07::ScriptObj_Deactivate,
    };

    pub const REPLAY_FREE_SNAPSHOT: Callee = Callee {
        addr: 0x0027_0238,
        f: ee::s07::Replay_FreeSnapshot,
    };

    pub const CODEC_POP_FREE_FRAME: Callee = Callee {
        addr: 0x003b_10d0,
        f: ee::s13::PS2_SONY_CODEC_INTERNAL_PopFreeFrame,
    };

    pub const MPEG2_GET_BITS: Callee = Callee {
        addr: 0x003b_1300,
        f: ee::s13::MPEG2_Get_Bits,
    };

    pub const MPEG2_MARKER_BIT: Callee = Callee {
        addr: 0x003b_2360,
        f: ee::s13::MPEG2_marker_bit,
    };

    pub const BUY_POPUP_INIT_BUY_BOLT: Callee = Callee {
        addr: 0x001c_ab10,
        f: ee::s04::cUIStateBuyPopup_SetItemParams,
    };

    pub const RIDER_GET_START_DIST_TO_FINISH: Callee = Callee {
        addr: 0x0011_3130,
        f: ee::s00::Rider_GetStartDistToFinish,
    };

    pub const BE_GET_RIDER_PROFILE_2: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::BE_GetRiderProfile,
    };

    pub const BENEW_PLAYER_INT_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::BENewPlayerInt_GetSlot,
    };

    pub const SCRIPT_ENGINE_GET_PROCESS: Callee = Callee {
        addr: 0x0028_2cb0,
        f: ee::s07::ScriptEngine_GetProcess,
    };

    pub const IN_SEQ_PLAY: Callee = Callee {
        addr: 0x0027_2ad0,
        f: ee::s07::InSeq_Play,
    };

    pub const CROWD_MAN_GROUP_FREE: Callee = Callee {
        addr: 0x0034_4348,
        f: ee::s11::CrowdMan3d_GroupFree,
    };

    pub const RAIL_MAN_ENABLE_RAIL: Callee = Callee {
        addr: 0x0035_8998,
        f: ee::s11::RailMan_EnableRail,
    };

    pub const SYNC_PROCESS_DTOR: Callee = Callee {
        addr: 0x0035_4920,
        f: ee::s11::SyncProcess_dtor,
    };

    pub const TEX_MAN_EVICT_TEXTURE: Callee = Callee {
        addr: 0x0036_9098,
        f: ee::s12::TexMan_EvictTexture,
    };

    pub const VRAM_HEAP_FREE_TEXTURE_BLOCKS: Callee = Callee {
        addr: 0x0036_6e30,
        f: ee::s12::VramHeap_FreeTextureBlocks,
    };

    pub const GAME_UI_OPEN_SCREEN: Callee = Callee {
        addr: 0x0020_ab50,
        f: ee::s05::GameUI_OpenScreen,
    };

    pub const WORLD_CACHE_RESET: Callee = Callee {
        addr: 0x003a_7f90,
        f: ee::s13::cWorldCache_Reset,
    };

    pub const WORLD_CACHE_LOAD: Callee = Callee {
        addr: 0x003a_7fc0,
        f: ee::s13::cWorldCache_Load,
    };

    pub const STREAMER_GET_SECTION_COUNT: Callee = Callee {
        addr: 0x003a_9800,
        f: ee::s13::cWorldCache_StreamerGetSectionCount,
    };

    pub const WSCRIPT_CACHE_ALLOC: Callee = Callee {
        addr: 0x003a_c800,
        f: ee::s13::cWScriptCache_Alloc,
    };
}

/// `RiderModel` (an `sAnimModel`): the two bone tables, the parts array and
/// the per-part LOD info.
#[allow(dead_code)]
pub(crate) mod model {

    pub const BONE_OFFSETS: u32 = 0x24;

    pub const BONE_ROTATIONS: u32 = 0x28;

    pub const BONES: u32 = 0x2c;

    pub const SCALE: u32 = 0x140;

    pub const NUM_PARTS: u32 = 0x8;

    pub const PARTS: u32 = 0xc;

    pub const NUM_BONES: u32 = 0x10;

    pub const HAS_PARTS: u32 = 0x18;

    pub const BONE_TABLE_A: u32 = 0x34;

    pub const BONE_TABLE_A_SHADOW: u32 = 0x30;

    pub const BONE_TABLE_B: u32 = 0x38;

    pub const LOD_INFO: u32 = 0x40;

    pub const SCALE_Y: u32 = 0x144;

    /// `sAnimModelPart` stride; `lods[lod]` is `{used, refs}` at `+0x18+8*lod`,
    /// `refIndex` at `+0x50`, and each `sAnimModelLodRef` is 16 bytes.
    pub const PART_STRIDE: u32 = 0x58;

    pub const PART_LODS: u32 = 0x18;

    pub const PART_REF_INDEX: u32 = 0x50;
}

// ---------------------------------------------------------------------------
// Steering.

/// Rider fields used by the steering computation.
pub(crate) mod rider {
    pub(crate) use crate::game::ai::computer::rider::MODEL as MODEL_2;
    pub(crate) use crate::game::ai::computer::rider::MOTION;
    pub(crate) use crate::game::visualfx::boardwakefx::rider::HIDDEN;
    pub(crate) use crate::game::visualfx::boardwakefx::rider::LOD;
    pub(crate) use crate::game::visualfx::boardwakefx::rider::MATRIX;
    pub(crate) use crate::game::visualfx::boardwakefx::rider::SUB_STATE;

    pub const MODEL: u32 = 0x780;

    pub const STATE: u32 = 0x77c;

    pub const ANIM: u32 = 0x784;

    pub const VEL: u32 = 0x1e0;

    pub const FWD_REF: u32 = 0x1b0;

    pub const POS: u32 = 0x110;

    pub const UP: u32 = 0x370;

    pub const UP_PREV: u32 = 0x380;

    pub const FRAME_FWD: u32 = 0x3a0;

    pub const FRAME_SIDE: u32 = 0x3b0;

    pub const GROUND_NORMAL2: u32 = 0x3d0;

    pub const GROUND_ID: u32 = 0x430;

    pub const GROUND_MATERIAL: u32 = 0x438;

    pub const GROUND_HEIGHT: u32 = 0x454;

    pub const GROUND_POINT: u32 = 0x460;

    pub const OVERLAP_LIST: u32 = 0x860;

    pub const OVERLAP_HINT: u32 = 0x864;

    pub const SPHERE_TREE: u32 = 0xaa0;

    /// The rider base object (state machine + embedded vtable pointer).
    pub const BASE: u32 = 0x6c0;

    /// The rider handler object, whose base holds the main/sub state words at
    /// +0xde0/+0xde4.
    pub const HANDLER: u32 = 0x77c;

    /// World position (vec4).
    pub const POSITION: u32 = 0x110;

    /// Velocity (vec4).
    pub const VELOCITY: u32 = 0x1e0;

    /// Accumulated heading (blended toward the ground normal each frame).
    pub const HEADING: u32 = 0x5b4;

    /// Ground normal (vec4).
    pub const GROUND_NORMAL: u32 = 0x370;

    // Race / AI paths.
    pub const RACE_PATH: u32 = 0xab4;

    pub const RACE_PATH_CURSOR: u32 = 0xac0;

    pub const AI_PATH: u32 = 0xab8;

    pub const AI_PATH_CURSOR: u32 = 0xabc;

    /// Race events this frame: `{i32 a; i32 b}` records, 16 bytes each.
    pub const RACE_ENTRIES: u32 = 0x4dc;

    pub const RACE_ENTRY_COUNT: u32 = 0x59c;

    /// Distance to the finish and its estimate.
    pub const DIST_TO_FINISH: u32 = 0x4d4;

    pub const DIST_ESTIMATE: u32 = 0x4d0;

    // AI path following.
    pub const AI_POINT: u32 = 0x4a0;

    pub const AI_POINT_PREV: u32 = 0x4b0;

    pub const AI_POINT_TARGET: u32 = 0x490;

    pub const AI_ARC_LEN: u32 = 0x4c0;

    pub const AI_PROGRESS: u32 = 0x4c4;

    pub const AI_SEARCH_DIST: u32 = 0x4c8;

    pub const AI_HEADING: u32 = 0x4cc;

    // Body flags used by the visibility test.
    pub const RENDER_FLAG: u32 = 0x884;

    pub const RENDER_KIND: u32 = 0x880;

    pub const VISIBILITY_BLOCK: u32 = 0xad0;

    pub const ON_SCREEN: u32 = 0xb18;

    pub const UNK_150: u32 = 0x150;

    pub const BBOX0: u32 = 0x400;

    pub const BBOX1: u32 = 0x410;

    pub const BSPHERE: u32 = 0x420;

    // Boost timers.
    pub const DELTA_TIME: u32 = 0x300;

    pub const BOOST_A: u32 = 0x2e8;

    pub const BOOST_B: u32 = 0x2ec;

    /// Boost amount and its phase id.
    pub const BOOST: u32 = 0x2f0;

    pub const BOOST_PHASE: u32 = 0x2f4;

    /// Boost effect scale.
    pub const BOOST_SCALE: u32 = 0x2f8;

    pub const BOOST_KIND: u32 = 0x304;

    // Wind force.
    pub const IS_HUMAN: u32 = 0x874;

    pub const AI_INDEX: u32 = 0x86c;

    // Race finish.
    pub const FINISH_TIME: u32 = 0x470;

    pub const DID_NOT_FINISH: u32 = 0x480;

    /// The rider's vtable pointer (`&rider->mVtbl`): slot 5 returns the
    /// rider's position (`lq` of 4 floats).
    pub const VTBL: u32 = 0x6c0;
}

// ---------------------------------------------------------------------------
// Ground probe.

/// `Contact` records (0x80 bytes) as the collision queries fill them.
pub(crate) mod contact {

    pub const SIZE: u32 = 0x80;

    pub const NORMAL: u32 = 0x10;

    pub const DEPTH: u32 = 0x40;

    pub const SURFACE: u32 = 0x50;

    pub const INSTANCE: u32 = 0x54;

    pub const MATERIAL: u32 = 0x5c;
}

/// Rider rail fields.
pub(crate) mod rail {

    pub const KIND: u32 = 0x320;

    pub const DIR_FLAG: u32 = 0x324;

    pub const STANCE: u32 = 0x328;

    pub const SIDE: u32 = 0x330;
}

/// The game's render-context singleton (`gp+0x2a90`, `D_004A5B80`): its
/// vtable (`+0x10d8`), the render-state stack pointer (`+0xe84`) and the
/// texture-id table (`+0xf50`).
#[allow(dead_code)]
pub(crate) mod ctx {
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::TEX_IDS;
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::TOP;
    pub(crate) use crate::game::visualfx::boardwakefx::ctx::VTABLE;
}

/// `sAmCtx` (`D_004A289C`) used by `Rider_RenderModel`, and the world
/// pointer (`gp-0x848`) whose `+0x84 -> +0x84 -> +0x14` is the active rider.
#[allow(dead_code)]
pub(crate) mod globals {
    pub(crate) use crate::game::visualfx::boardwakefx::globals::AM_CTX;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::BONE_MATRICES;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::BONE_MATRICES_SHADOW;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::EMITTER_BLEND;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_MANAGER;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_STATE;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::FLAG_TABLE;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::IDENTITY_MAT;
    pub(crate) use crate::game::visualfx::boardwakefx::globals::WORLD;
}

// ---------------------------------------------------------------------------
// Timed event animations.

pub(crate) mod rider_anim {

    /// Event counter (0: none) and the frame it last changed.
    pub const EVENT: u32 = 0x358;

    pub const LAST_FRAME: u32 = 0x354;

    /// Animation controller (has a vtable at `+0x58`).
    pub const ANIM: u32 = 0x784;

    /// A doubleword handed to the animation as its blend parameter.
    pub const BLEND_SOURCE: u32 = 0x8c8;

    /// The current sub-state's blend value.
    pub const SUB_BLEND: u32 = 0x1fc;

    /// `Rider+0x77c -> +0xde4`: the rider's sub-state (non-zero: busy).
    pub const STATE: u32 = 0x77c;
}

pub(crate) mod anim_callee {
    use super::Callee;
    use ssx3_ee as ee;

    pub const GET_LAYER_ENTRY: Callee = Callee {
        addr: 0x0031_1ae8,
        f: ee::s10::AnimController_GetLayerAnimEntry,
    };

    pub const PLAY_ANIM: Callee = Callee {
        addr: 0x0031_28e8,
        f: ee::s10::AnimController_PlayAnim,
    };

    pub const AUDIO_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };

    pub const AUDIO_RIDER_SAY: Callee = Callee {
        addr: 0x0029_ff80,
        f: ee::s08::SSXAudioSystem_RiderSay2082,
    };

    pub const FRAME_COUNTER: Callee = Callee {
        addr: 0x0012_98c8,
        f: ee::s00::AI_GetFrameCounter,
    };

    pub const GET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fee8,
        f: ee::s00::Rider_GetSubState,
    };

    /// The animation-type jump table the original indexes by `event - 1`.
    pub const EVENT_TABLE: u32 = 0x0045_6cc0;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
}

/// `void Rider_SetSlew5Target(Rider *rider, float target)` (0x113f38): slew the
/// "slew 5" value (`+0x22c`) toward `target` (f12), rate limited by the two gp
/// constants (a per-frame cap and a multiplier), store the unscaled target at
/// `+0x234` and the new value at `+0x230`.
pub fn rider_set_slew5_target(c: &mut Cpu) {
    let rider = Addr(abi::arg(c, 0));
    let target = abi::farg(c, 0); // f12

    c.cycles += 13;
    let cur = rider.off(0x22c).read_fpu(c);
    c.g[AT][0] = 0x40e0_0000; // 7.0
    c.f[2] = c.g[AT][0] as u32; // mtc1 f2, at
    let diff = target - cur;
    let cap = Addr((c.g[GP][0] as u32).wrapping_sub(0x7c28)).read_fpu(c); // f1
    c.f[1] = cap.0;
    let scaled = diff.abs() * FpuF32(c.f[2]);
    c.f[0] = scaled.0;
    c.g[AT][0] = 0x4100_0000; // 8.0
    c.f[3] = c.g[AT][0] as u32; // mtc1 f3, at

    // c.le.s f1, f0: cap <= scaled.
    let within = cap.le(scaled);
    abi::set_fcc(c, within);
    c.f[2] = cap.0; // mov.s f2, f1
    if within {
        c.cycles += 1;
        c.f[2] = scaled.min(FpuF32(c.f[3])).0; // min.s f2, f0, f3
    }

    c.cycles += 5;
    let mul = Addr((c.g[GP][0] as u32).wrapping_sub(0x7c24)).read_fpu(c); // f0
    c.f[0] = mul.0;
    rider.off(0x234).write_fpu(c, target);
    let new = FpuF32(c.f[2]) * mul; // mul.s f0, f2, f0
    c.f[0] = new.0;
    rider.off(0x230).write_fpu(c, new);

    abi::ret(c);
}

/// `char *Rider_GetMainStateName(Rider *rider)` (0x11fea8): tail-call
/// `RiderBase_GetMainStateName` (0x111830) on the base object at
/// `rider+0x77c`.
pub fn rider_get_main_state_name(c: &mut Cpu) {
    call::tail_wrapper(
        c,
        Some(0x77c),
        call::Target::Direct(Callee {
            addr: 0x0011_1830,
            f: ssx3_ee::s00::RiderBase_GetMainStateName,
        }),
        0x0011_feb8,
    );
}

/// Global vectors (`.bss`, set up at boot): the bone pivot subtracted
/// before rotating a bone offset, the three rotation axes, the identity
/// quaternion.
pub(crate) const PIVOT: u32 = 0x004f_f130;

pub(crate) const AXIS_X: u32 = 0x004f_f140;

pub(crate) const AXIS_Z: u32 = 0x004f_f150;

pub(crate) const AXIS_Y: u32 = 0x004f_f160;

pub(crate) const IDENTITY_QUAT: u32 = 0x004f_f220;

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lw` (sign-extended).
#[inline(always)]
pub(crate) fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// FPU register `n`.
#[inline(always)]
pub(crate) fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `fN = v`.
#[inline(always)]
pub(crate) fn setf(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}

/// `lwc1 fN, a`.
#[inline(always)]
pub(crate) fn ldf(c: &mut Cpu, n: usize, a: u32) -> FpuF32 {
    let v = Addr(a).read_fpu(c);
    setf(c, n, v)
}

/// `swc1 fN, a`.
#[inline(always)]
pub(crate) fn stf(c: &mut Cpu, n: usize, a: u32) {
    let v = fr(c, n);
    Addr(a).write_fpu(c, v);
}

/// `li at, bits; mtc1 at, fN`.
#[inline(always)]
pub(crate) fn li_f(c: &mut Cpu, n: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = sx(bits);
    setf(c, n, FpuF32(bits))
}

/// Set the FPU condition bit; returns it.
#[inline(always)]
pub(crate) fn fcc(c: &mut Cpu, cond: bool) -> bool {
    abi::set_fcc(c, cond);
    cond
}

/// `lq reg, src; sq reg, dst` (a quadword copied through a GPR).
#[inline(always)]
pub(crate) fn qcopy(c: &mut Cpu, reg: usize, src: u32, dst: u32) {
    c.lq(reg, src);
    c.sq(reg, dst);
}

// ---------------------------------------------------------------------------
// VU0 sequences (macro mode), as the original issues them.

/// `out = a + b` (vf3, vf4 → vf5).
pub(crate) fn v_add(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `out = a - b` (vf3, vf4 → vf5).
pub(crate) fn v_sub(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// [`v_add`] or [`v_sub`].
pub(crate) fn v_add_sub(c: &mut Cpu, add: bool, a: u32, b: u32, out: u32) {
    if add {
        v_add(c, a, b, out)
    } else {
        v_sub(c, a, b, out)
    }
}

/// `out = v * fF`, the scalar moved through GPR `via` into vf3
/// (`mfc1 via, fF; qmtc2 via, vf3` — vf3 keeps the GPR's upper half).
pub(crate) fn v_scale(c: &mut Cpu, via: usize, f: usize, v: u32, out: u32) {
    c.mfc1(via, f);
    c.lqc2(4, v);
    c.qmtc2(via, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}

/// The tail of the dot-product sequence (vf4 holds the products).
pub(crate) fn dot_tail(c: &mut Cpu) {
    let vu = c.vu0();
    vu.adda(0b1000, 4, Src::Bc(4, Y));
    vu.madda(0b1000, 6, Src::Bc(4, Z));
    vu.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `vf4.x = a · b` (vf3 = a, vf5 = b).
pub(crate) fn v_dot(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    dot_tail(c);
}

/// `vf4.x = a · a` (vf3 = a).
pub(crate) fn v_dot_self(c: &mut Cpu, a: u32) {
    c.lqc2(3, a);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    dot_tail(c);
}

/// `|a|` through Q (`vsqrt`, `cfc2 via, Q`, `mtc1 via, fDst`).
pub(crate) fn v_length(c: &mut Cpu, a: u32, via: usize, dst: usize) -> FpuF32 {
    v_dot_self(c, a);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(via, 22);
    c.mtc1(via, dst);
    fr(c, dst)
}

/// `out = a / |a|` (`vrsqrt`).
pub(crate) fn v_normalize(c: &mut Cpu, a: u32, out: u32) {
    v_dot_self(c, a);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, out);
}

/// `out = v / fF` (`vdiv`, the scalar moved through v0).
pub(crate) fn v_div_by(c: &mut Cpu, f: usize, v: u32, out: u32) {
    c.mfc1(V0, f);
    c.qmtc2(V0, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, v);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 4, Src::Q);
    c.sqc2(5, out);
}

/// `out = a × b` (vf4 = a, vf5 = b → vf6).
pub(crate) fn v_cross(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(4, a);
    c.lqc2(5, b);
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, out);
}

/// Quaternion product `out = a ⊗ b` (vf4 = a, vf5 = b → vf8).
pub(crate) fn q_mul(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(4, a);
    c.lqc2(5, b);
    let vu = c.vu0();
    vu.mul(0b1111, 7, 4, Src::Vec(5));
    vu.opmula(4, 5);
    vu.opmsub(6, 5, 4);
    vu.mula(0b1110, 4, Src::Bc(5, W));
    vu.madda(0b1110, 5, Src::Bc(4, W));
    vu.suba(0b0001, 7, Src::Bc(7, X));
    vu.msuba(0b0001, 0, Src::Bc(7, Y));
    vu.msub(0b0001, 8, 0, Src::Bc(7, Z));
    vu.madd(0b1110, 8, 6, Src::Bc(0, W));
    c.sqc2(8, out);
}

/// Vector `v` rotated by quaternion `q`: `v + 2w(q×v) + 2q×(q×v)` (vf4 = q,
/// vf5 = v → vf8).
pub(crate) fn q_rotate(c: &mut Cpu, q: u32, v: u32, out: u32) {
    c.lqc2(4, q);
    c.lqc2(5, v);
    let vu = c.vu0();
    vu.sub(0b0001, 8, 8, Src::Vec(8));
    vu.opmula(4, 5);
    vu.opmsub(6, 5, 4);
    vu.opmula(4, 6);
    vu.opmsub(7, 6, 4);
    vu.mula(0b1110, 5, Src::Bc(0, W));
    vu.madda(0b1110, 6, Src::Bc(4, W));
    vu.madda(0b1110, 6, Src::Bc(4, W));
    vu.madda(0b1110, 7, Src::Bc(0, W));
    vu.madd(0b1110, 8, 7, Src::Bc(0, W));
    c.sqc2(8, out);
}

/// The quaternion `{axis * sin, cos}` from `Math_SinCos`'s outputs at
/// `sincos` (sin) and `sincos + 4` (cos), written to `out`. The original
/// reads the axis from a global (z into f1, x into `x_reg`, y into f4) and
/// the cosine into `cos_reg`; f0 ends as y·sin.
pub(crate) fn axis_quat(
    c: &mut Cpu,
    axis: u32,
    sincos: u32,
    out: u32,
    x_reg: usize,
    cos_reg: usize,
) {
    let az = ldf(c, 1, p(axis, 8));
    let s = ldf(c, 0, sincos);
    let ax = ldf(c, x_reg, axis);
    let ay = ldf(c, 4, p(axis, 4));
    let qz = setf(c, 1, s * az);
    let qx = setf(c, x_reg, s * ax);
    let w = ldf(c, cos_reg, p(sincos, 4));
    let qy = setf(c, 0, s * ay);
    Addr(p(out, 0xc)).write_fpu(c, w);
    Addr(p(out, 8)).write_fpu(c, qz);
    Addr(out).write_fpu(c, qx);
    Addr(p(out, 4)).write_fpu(c, qy);
}

/// [`axis_quat`] for an axis on the stack (x into f1, y into f2, z into
/// f4, cosine into f3; f0 ends as z·sin).
pub(crate) fn axis_quat_v(c: &mut Cpu, axis: u32, sincos: u32, out: u32) {
    let s = ldf(c, 0, sincos);
    let ax = ldf(c, 1, axis);
    let ay = ldf(c, 2, p(axis, 4));
    let az = ldf(c, 4, p(axis, 8));
    let qx = setf(c, 1, s * ax);
    let qy = setf(c, 2, s * ay);
    let w = ldf(c, 3, p(sincos, 4));
    let qz = setf(c, 0, s * az);
    Addr(p(out, 0xc)).write_fpu(c, w);
    Addr(out).write_fpu(c, qx);
    Addr(p(out, 4)).write_fpu(c, qy);
    Addr(p(out, 8)).write_fpu(c, qz);
}

// ---------------------------------------------------------------------------
// Rider_UpdateBounds.

/// Corners 2..8 of the box: (forward, up, side) signs (true: +) and the
/// side extent (+250 / −50).
pub(crate) const BOUNDS_CORNERS: [([bool; 3], u32); 7] = [
    ([true, true, false], 0x4248_0000),
    ([true, false, true], 0x437a_0000),
    ([true, false, false], 0x4248_0000),
    ([false, true, true], 0x437a_0000),
    ([false, true, false], 0x4248_0000),
    ([false, false, true], 0x437a_0000),
    ([false, false, false], 0x4248_0000),
];

/// `void Rider_UpdateBounds(Rider* r, const Vec4* centre)` (0x11e150): the
/// world AABB (`+0x400` min, `+0x410` max) of the box centred on `centre`
/// (or the rider position `+0x110` when null) spanned by forward
/// (`+0x1b0`, ±150), up (`+0x1a0`, ±100) and side (`+0x1c0`, +250/−50);
/// `+0x420` = the centre with w = 250. Leaf function, 0xa0 frame.
pub fn rider_update_bounds(c: &mut Cpu) {
    c.cycles += 65;
    c.g[A2][0] = c.g[A0][0];
    let frame = Frame::open(c, 0xa0);
    let r = r32(c, A2);
    c.lq(V0, p(r, 0x1b0));
    c.g[A3][0] = sx(p(r, 0x110));
    c.lq(V1, p(r, 0x1c0));
    if c.g[A1][0] != 0 {
        c.g[A3][0] = c.g[A1][0];
    }
    c.lq(A0, p(r, 0x1a0));
    c.g[T0][0] = sx(p(r, 0x400));
    let sp = sp(c);
    c.sq(V1, sp);
    c.sq(A0, p(sp, 0x10));
    c.sq(V0, p(sp, 0x20));
    li_f(c, 3, 0x4316_0000); // 150
    let centre = r32(c, A3);
    c.lq(V0, centre);
    li_f(c, 2, 0x42c8_0000); // 100
    c.sq(V0, p(r, 0x400));
    c.sq(V0, p(r, 0x410));
    bounds_corner(c, r, centre, V1, 0x437a_0000, [true, true, true]);
    for (i, (signs, side)) in BOUNDS_CORNERS.iter().enumerate() {
        c.cycles += if i == 1 { 49 } else { 47 };
        if i == 1 {
            c.g[A0][0] = sx(p(r, 0x400));
            c.g[A1][0] = c.g[A0][0];
        }
        bounds_corner(c, r, centre, V0, *side, *signs);
    }
    c.cycles += 7;
    c.lq(V0, centre);
    li_f(c, 0, 0x437a_0000);
    c.sq(V0, p(r, 0x420));
    stf(c, 0, p(r, 0x42c));
    frame.close(c);
}

/// One corner: `centre ± fwd*f3 ± up*f2 ± side*side_len` (the original's
/// stack round trips), then the box grown to it (the first compare is
/// part of the corner's block, each further one costs 6 cycles).
pub(crate) fn bounds_corner(
    c: &mut Cpu,
    r: u32,
    centre: u32,
    via: usize,
    side: u32,
    signs: [bool; 3],
) {
    let sp = sp(c);
    v_scale(c, via, 3, p(sp, 0x20), p(sp, 0x70));
    li_f(c, 0, side);
    qcopy(c, V0, p(sp, 0x70), p(sp, 0x60));
    v_add_sub(c, signs[0], centre, p(sp, 0x60), p(sp, 0x70));
    qcopy(c, V0, p(sp, 0x70), p(sp, 0x50));
    v_scale(c, V1, 2, p(sp, 0x10), p(sp, 0x80));
    qcopy(c, V0, p(sp, 0x80), p(sp, 0x70));
    v_add_sub(c, signs[1], p(sp, 0x50), p(sp, 0x70), p(sp, 0x80));
    qcopy(c, V0, p(sp, 0x80), p(sp, 0x40));
    v_scale(c, V1, 0, sp, p(sp, 0x90));
    qcopy(c, V0, p(sp, 0x90), p(sp, 0x80));
    v_add_sub(c, signs[2], p(sp, 0x40), p(sp, 0x80), p(sp, 0x90));
    qcopy(c, V0, p(sp, 0x90), p(sp, 0x30));
    let corner = p(sp, 0x30);
    for k in 0..3 {
        if k > 0 {
            c.cycles += 6;
        }
        grow_bound(c, p(corner, 4 * k), p(r, 0x400 + 4 * k), true);
    }
    for k in 0..3 {
        c.cycles += 6;
        grow_bound(c, p(corner, 4 * k), p(r, 0x410 + 4 * k), false);
    }
}

/// `f1 = corner, f0 = bound`; store the corner when it is below (`lower`)
/// or above the bound.
pub(crate) fn grow_bound(c: &mut Cpu, corner: u32, bound: u32, lower: bool) {
    let v = ldf(c, 1, corner);
    let b = ldf(c, 0, bound);
    let out = if lower { v.lt(b) } else { b.lt(v) };
    if fcc(c, out) {
        Addr(bound).write_fpu(c, v);
    }
}

// ---------------------------------------------------------------------------
// Rider_UpdateBoardLean.

/// `void Rider_UpdateBoardLean(Rider* r, Xform* xf)` (0x11fa10): roll
/// (`+0x208`) and pitch (`+0x1f0`) beyond twice the dead zone (`+0x214`)
/// rotate `xf` (`{pos, quat}`) about the pivot bone (`+0x8a0`) by
/// quaternions about the X and Z axes; the spin input (`+0x250`) yaws it;
/// finally `xf.pos += +0x3b0 * scale.x * pitch * 45`.
pub fn rider_update_board_lean(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x120);
        c.f[2] = 0;
        frame.save(c, S4, 0xb0);
        frame.save(c, S5, 0xa0);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0xf0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S1, 0xe0);
        frame.save(c, S2, 0xd0);
        frame.save(c, S3, 0xc0);
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 22, 0x110);
        frame.save_fpr(c, 21, 0x108);
        frame.save_fpr(c, 20, 0x100);
        let r = r32(c, S5);
        let xf = r32(c, S4);
        let roll = dead_zone(c, p(r, 0x208), p(r, 0x214), 6);
        c.cycles += 10;
        let pitch = dead_zone(c, p(r, 0x1f0), p(r, 0x214), 22);
        c.cycles += 5;
        c.f[0] = 0;
        let mut lean = !fcc(c, pitch.eq_s(FpuF32::ZERO));
        if !lean {
            c.cycles += 4;
            lean = !fcc(c, roll.eq_s(FpuF32::ZERO));
            if lean {
                c.cycles += 1;
            }
        }
        if lean {
            c.g[V1][0] = lw(c, p(r, rider::MODEL));
            c.cycles += 154;
            lean_board(c, r, xf, roll, pitch)?;
        } else {
            ldf(c, 1, p(r, 0x250));
        }
        c.cycles += 4;
        let spin = fr(c, 1);
        if fcc(c, spin.eq_s(FpuF32::ZERO)) {
            ldf(c, 1, p(r, 0x1f0));
        } else {
            c.cycles += 40;
            spin_board(c, r, xf, spin)?;
        }
        // xf.pos += body offset * scale.x * pitch * 45
        c.cycles += 29;
        let k = li_f(c, 2, 0x4234_0000);
        c.g[V0][0] = lw(c, p(r, rider::MODEL));
        let model = r32(c, V0);
        let pitch_now = fr(c, 1);
        let s = setf(c, 1, pitch_now * k);
        frame.restore(c, S0, 0xf0);
        let scale = ldf(c, 0, p(model, model::SCALE));
        frame.restore(c, S1, 0xe0);
        setf(c, 0, scale * s);
        frame.restore(c, S2, 0xd0);
        let sp = sp(c);
        v_scale(c, V1, 0, p(r, 0x3b0), p(sp, 0x10));
        frame.restore(c, S3, 0xc0);
        c.lq(V0, p(sp, 0x10));
        frame.restore(c, S5, 0xa0);
        c.sq(V0, sp);
        v_add(c, xf, sp, xf);
        frame.restore_ra(c, 0x90);
        frame.restore(c, S4, 0xb0);
        frame.restore_fpr(c, 22, 0x110);
        frame.restore_fpr(c, 21, 0x108);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `fOut = sign(v) * max(|v| - 2|dead|, 0)` (f0 = 2|dead|, f1 = |v|, f3 =
/// v, f2 = 0 as the original leaves them).
pub(crate) fn dead_zone(c: &mut Cpu, value_at: u32, dead_at: u32, out: usize) -> FpuF32 {
    let dead = ldf(c, 0, dead_at);
    let dead = setf(c, 0, dead.abs());
    let v = ldf(c, 3, value_at);
    let dead2 = setf(c, 0, dead + dead);
    let mag = setf(c, 1, v.abs());
    c.f[2] = 0;
    let r = setf(c, out, mag - dead2);
    if fcc(c, r.lt(FpuF32::ZERO)) {
        c.cycles += 2;
        setf(c, out, FpuF32::ZERO)
    } else {
        c.cycles += 4;
        if fcc(c, v.lt(FpuF32::ZERO)) {
            setf(c, out, -r)
        } else {
            r
        }
    }
}

/// The lean rotation (154-cycle block; v1 = the model): move the pivot
/// bone offset into place, rotate about X by roll and Z by pitch (half
/// angles through `Math_SinCos`), move back. Ends with f1 = `+0x250`,
/// f0 = 0.
pub(crate) fn lean_board(c: &mut Cpu, r: u32, xf: u32, roll: FpuF32, pitch: FpuF32) -> Flow {
    let sp = sp(c);
    let m = r32(c, V1);
    c.g[A0][0] = sx(p(sp, 0x70));
    c.g[V0][0] = lw(c, p(r, 0x8a0));
    c.g[A1][0] = sx(p(sp, 0x74));
    c.g[A2][0] = lw(c, p(m, model::BONE_OFFSETS));
    c.g[S0][0] = 0x50_0000;
    let off = r32(c, V0) << 4;
    c.g[V0][0] = sx(off);
    let sw_ = ldf(c, 0, p(m, model::SCALE + 0xc));
    let bone = off.wrapping_add(r32(c, A2));
    c.g[V0][0] = sx(bone);
    let sx_ = ldf(c, 3, p(m, model::SCALE));
    let bw = ldf(c, 4, p(bone, 0xc));
    c.g[S1][0] = sx(AXIS_X);
    let bx = ldf(c, 1, bone);
    c.g[S3][0] = 0x50_0000;
    setf(c, 4, bw * sw_);
    let sy = ldf(c, 5, p(m, model::SCALE + 4));
    setf(c, 1, bx * sx_);
    let by = ldf(c, 2, p(bone, 4));
    let sz = ldf(c, 3, p(m, model::SCALE + 8));
    c.g[S2][0] = sx(AXIS_Z);
    let bz = ldf(c, 0, p(bone, 8));
    setf(c, 2, by * sy);
    c.g[V0][0] = 0x50_0000;
    stf(c, 1, p(sp, 0x10));
    setf(c, 0, bz * sz);
    stf(c, 4, p(sp, 0x1c));
    stf(c, 2, p(sp, 0x14));
    c.g[V0][0] = sx(PIVOT);
    let k_roll = ldf(c, 12, gp(c, -0x7970));
    stf(c, 0, p(sp, 0x18));
    // xf.pos += rotate(xf.rot, offset - pivot)
    v_sub(c, p(sp, 0x10), PIVOT, p(sp, 0x20));
    let a = setf(c, 12, roll * k_roll);
    c.lq(V0, p(sp, 0x20));
    let half = li_f(c, 21, 0x3f00_0000);
    c.sq(V0, sp);
    q_rotate(c, p(xf, 0x10), sp, p(sp, 0x20));
    setf(c, 12, a * half);
    c.lq(V0, p(sp, 0x20));
    let k_pitch = ldf(c, 20, gp(c, -0x796c));
    c.sq(V0, p(sp, 0x10));
    v_add(c, xf, p(sp, 0x10), xf);
    call_ptr(c, callee::MATH_SIN_COS, 0x0011_fbe0)?;
    // xf.rot ⊗= {X * sin, cos}
    let b = setf(c, 20, pitch * k_pitch);
    c.g[A0][0] = sx(p(sp, 0x78));
    c.g[A1][0] = sx(p(sp, 0x7c));
    axis_quat(c, AXIS_X, p(sp, 0x70), p(sp, 0x40), 3, 2);
    setf(c, 12, b * half);
    qcopy(c, V0, p(sp, 0x40), p(sp, 0x30));
    q_mul(c, p(xf, 0x10), p(sp, 0x30), p(sp, 0x60));
    c.lq(V0, p(sp, 0x60));
    c.sq(V0, p(xf, 0x10));
    c.sq(V0, p(sp, 0x50));
    call_ptr(c, callee::MATH_SIN_COS, 0x0011_fc68)?;
    // xf.rot ⊗= {Z * sin, cos}
    axis_quat(c, AXIS_Z, p(sp, 0x78), p(sp, 0x20), 2, 3);
    li_f(c, 5, 0xbf80_0000);
    qcopy(c, V0, p(sp, 0x20), p(sp, 0x10));
    q_mul(c, p(xf, 0x10), p(sp, 0x10), p(sp, 0x40));
    c.lq(V0, p(sp, 0x40));
    c.sq(V0, p(xf, 0x10));
    c.sq(V0, p(sp, 0x30));
    // xf.pos += rotate(xf.rot, -(offset - pivot))
    v_scale(c, V1, 5, sp, p(sp, 0x20));
    qcopy(c, V0, p(sp, 0x20), p(sp, 0x10));
    q_rotate(c, p(xf, 0x10), p(sp, 0x10), p(sp, 0x30));
    qcopy(c, V0, p(sp, 0x30), p(sp, 0x20));
    v_add(c, xf, p(sp, 0x20), xf);
    ldf(c, 1, p(r, 0x250));
    c.f[0] = 0;
    Ok(())
}

/// The spin yaw (40-cycle block): `xf.rot ⊗= {X * sin, cos}` of
/// `spin * k * 0.5`. Ends with f1 = `+0x1f0`.
pub(crate) fn spin_board(c: &mut Cpu, r: u32, xf: u32, spin: FpuF32) -> Flow {
    let sp = sp(c);
    let k = ldf(c, 12, gp(c, -0x7968));
    c.g[S1][0] = 0x50_0000;
    let half = li_f(c, 0, 0x3f00_0000);
    c.g[A0][0] = sx(p(sp, 0x80));
    let a = setf(c, 12, spin * k);
    c.g[A1][0] = sx(p(sp, 0x84));
    c.g[S0][0] = sx(AXIS_X);
    setf(c, 12, a * half);
    call_ptr(c, callee::MATH_SIN_COS, 0x0011_fd8c)?;
    axis_quat(c, AXIS_X, p(sp, 0x80), p(sp, 0x10), 2, 3);
    qcopy(c, V0, p(sp, 0x10), sp);
    q_mul(c, p(xf, 0x10), sp, p(sp, 0x30));
    c.lq(V0, p(sp, 0x30));
    c.sq(V0, p(xf, 0x10));
    c.sq(V0, p(sp, 0x20));
    ldf(c, 1, p(r, 0x1f0));
    Ok(())
}

// ---------------------------------------------------------------------------
// Rider_SolveLegIK.

/// `void Rider_SolveLegIK(Rider* r, const int bones[3], const Xform* target,
/// float blend)` (0x11f3d8): `bones` = {hip, knee, foot}. The foot
/// transform moves to the target (`blend == 1`) or towards it (position
/// lerp, `Quat_Slerp`); the knee angle follows from the two bone lengths
/// (the x of the scaled bone offsets) and the hip-foot distance; the knee
/// transform (written last) hangs off the hip, which is swung about
/// `knee × foot` so that the leg reaches the foot.
pub fn rider_solve_leg_ik(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x1b0);
        let one = li_f(c, 0, 0x3f80_0000);
        frame.save(c, S0, 0x180);
        let blend = fr(c, 12);
        setf(c, 1, blend);
        frame.save(c, S1, 0x170);
        let full = fcc(c, blend.eq_s(one));
        frame.save(c, S2, 0x160);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x150);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x140);
        frame.save_ra(c, 0x130);
        frame.save_fpr(c, 23, 0x1a8);
        frame.save_fpr(c, 22, 0x1a0);
        frame.save_fpr(c, 21, 0x198);
        frame.save_fpr(c, 20, 0x190);
        let r = r32(c, S2);
        let leg = r32(c, S1);
        let target = r32(c, A2);
        c.g[A0][0] = lw(c, p(r, rider::MODEL));
        let m = r32(c, A0);
        c.g[V0][0] = lw(c, p(leg, 8));
        c.g[V1][0] = lw(c, p(m, model::BONES));
        let off = r32(c, V0) << 5;
        c.g[V0][0] = sx(off);
        let foot = r32(c, V1).wrapping_add(off);
        c.g[S0][0] = sx(foot);
        if full {
            c.cycles += 5;
            qcopy(c, V0, target, foot);
            qcopy(c, V1, p(target, 0x10), p(foot, 0x10));
        } else {
            c.cycles += 27;
            let sp = sp(c);
            // foot.pos += (target.pos - foot.pos) * blend
            v_sub(c, target, foot, p(sp, 0x30));
            setf(c, 12, blend);
            c.lq(V0, p(sp, 0x30));
            c.g[A2][0] = sx(p(target, 0x10));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx(p(foot, 0x10));
            c.sq(V0, p(sp, 0x20));
            v_scale(c, V1, 1, p(sp, 0x20), p(sp, 0x30));
            qcopy(c, V0, p(sp, 0x30), p(sp, 0x10));
            v_add(c, foot, p(sp, 0x10), p(sp, 0x30));
            c.lq(V0, p(sp, 0x30));
            c.sq(V0, sp);
            c.sq(V0, foot);
            // Quat_Slerp(&tmp, foot.rot, target.rot, blend)
            call_ptr(c, callee::QUAT_SLERP, 0x0011_f4ac)?;
            qcopy(c, V0, sp, p(foot, 0x10));
        }

        // Bone lengths and the knee angle.
        c.cycles += 79;
        let sp = sp(c);
        c.g[V1][0] = lw(c, p(r, rider::MODEL));
        let m = r32(c, V1);
        c.g[A1][0] = lw(c, p(leg, 4));
        c.g[A0][0] = lw(c, p(m, model::BONE_OFFSETS));
        let knee_i = r32(c, A1);
        let off = knee_i << 4;
        c.g[V0][0] = sx(off);
        let s_x = ldf(c, 3, p(m, model::SCALE));
        let bp = off.wrapping_add(r32(c, A0));
        c.g[V0][0] = sx(bp);
        let s_y = ldf(c, 5, p(m, model::SCALE + 4));
        let bx = ldf(c, 0, bp);
        c.g[A1][0] = sx(knee_i << 5);
        let bw = ldf(c, 4, p(bp, 0xc));
        let upper = setf(c, 20, bx * s_x);
        let by = ldf(c, 1, p(bp, 4));
        let s_z = ldf(c, 0, p(m, model::SCALE + 8));
        let s_w = ldf(c, 3, p(m, model::SCALE + 0xc));
        setf(c, 1, by * s_y);
        let bz = ldf(c, 2, p(bp, 8));
        let upper2 = setf(c, 5, upper * upper);
        setf(c, 4, bw * s_w);
        c.g[A2][0] = lw(c, p(m, model::BONES));
        setf(c, 2, bz * s_z);
        stf(c, 1, p(sp, 4));
        stf(c, 20, sp);
        let knee = r32(c, A2).wrapping_add(r32(c, A1));
        c.g[S4][0] = sx(knee);
        stf(c, 4, p(sp, 0xc));
        stf(c, 2, p(sp, 8));
        let half = li_f(c, 23, 0x3f00_0000);
        c.g[V0][0] = lw(c, p(leg, 8));
        c.g[A0][0] = lw(c, p(m, model::BONE_OFFSETS));
        let off = r32(c, V0) << 4;
        c.g[V0][0] = sx(off);
        let s_w = ldf(c, 3, p(m, model::SCALE + 0xc));
        let bp = off.wrapping_add(r32(c, A0));
        c.g[V0][0] = sx(bp);
        let s_x = ldf(c, 4, p(m, model::SCALE));
        let bx = ldf(c, 0, bp);
        let bw = ldf(c, 1, p(bp, 0xc));
        let lower = setf(c, 21, bx * s_x);
        let by = ldf(c, 2, p(bp, 4));
        setf(c, 1, bw * s_w);
        let bz = ldf(c, 0, p(bp, 8));
        let s_y = ldf(c, 3, p(m, model::SCALE + 4));
        let s_z = ldf(c, 4, p(m, model::SCALE + 8));
        c.g[V0][0] = lw(c, leg);
        setf(c, 2, by * s_y);
        setf(c, 0, bz * s_z);
        stf(c, 1, p(sp, 0xc));
        let hip_off = r32(c, V0) << 5;
        c.g[V0][0] = sx(hip_off);
        stf(c, 21, sp);
        stf(c, 2, p(sp, 4));
        let hip = r32(c, A2).wrapping_add(hip_off);
        c.g[S2][0] = sx(hip);
        stf(c, 0, p(sp, 8));
        let lower2 = setf(c, 3, lower * lower);
        v_sub(c, foot, hip, p(sp, 0x10));
        let prod = setf(c, 2, upper * lower);
        c.lq(V0, p(sp, 0x10));
        let neg_one = li_f(c, 1, 0xbf80_0000);
        c.sq(V0, sp);
        v_dot(c, sp, sp);
        c.qmfc2(V1, 4);
        c.mtc1(V1, 0);
        let dist2 = fr(c, 0);
        let t = setf(c, 0, dist2 - upper2);
        let one22 = li_f(c, 22, 0x3f80_0000);
        let t = setf(c, 0, t - lower2);
        let t = setf(c, 0, t * half);
        let cos = setf(c, 0, t.div(prod));
        let in_range = fcc(c, neg_one.le(cos));
        setf(c, 12, neg_one);
        if in_range {
            c.cycles += 1;
            setf(c, 12, cos.min(one22));
        }

        // The knee: rotate about AXIS_Y by asin(cos) - pi/2.
        c.cycles += 159;
        c.g[S0][0] = 0x50_0000;
        call_ptr(c, callee::MATH_ASIN, 0x0011_f5fc)?;
        let quarter = ldf(c, 1, gp(c, -0x7978));
        c.g[A0][0] = sx(p(sp, 0x120));
        stf(c, 20, p(sp, 0x30));
        c.g[A1][0] = sx(p(sp, 0x124));
        let a = fr(c, 0);
        let a = setf(c, 0, a - quarter);
        c.sw(p(sp, 0x34), 0);
        c.sw(p(sp, 0x38), 0);
        c.g[S1][0] = sx(AXIS_Y);
        stf(c, 22, p(sp, 0x3c));
        c.g[S3][0] = 0x50_0000;
        setf(c, 12, a * half);
        qcopy(c, V0, p(sp, 0x30), p(sp, 0x10));
        call_ptr(c, callee::MATH_SIN_COS, 0x0011_f634)?;
        axis_quat(c, AXIS_Y, p(sp, 0x120), p(sp, 0x30), 2, 3);
        let eps = ldf(c, 5, gp(c, -0x7974));
        qcopy(c, V0, p(sp, 0x30), p(sp, 0x20));
        // Where the knee and foot end up from the hip.
        q_mul(c, p(hip, 0x10), p(sp, 0x20), p(sp, 0x90));
        c.lq(V0, p(sp, 0x90));
        c.sq(V0, p(sp, 0x70));
        c.sq(V0, p(sp, 0x80));
        v_sub(c, p(sp, 0x10), PIVOT, p(sp, 0xb0));
        qcopy(c, V0, p(sp, 0xb0), p(sp, 0xa0));
        q_rotate(c, p(hip, 0x10), p(sp, 0xa0), p(sp, 0xb0));
        qcopy(c, V0, p(sp, 0xb0), p(sp, 0x90));
        v_add(c, p(sp, 0x90), hip, p(sp, 0xb0));
        c.lq(V0, p(sp, 0xb0));
        c.sq(V0, p(sp, 0x80));
        c.lq(V1, p(sp, 0x70));
        c.sq(V0, p(sp, 0x60));
        c.sq(V0, p(sp, 0x40));
        c.sq(V1, p(sp, 0x50));
        stf(c, 21, p(sp, 0x70));
        stf(c, 22, p(sp, 0x7c));
        c.sw(p(sp, 0x74), 0);
        c.sw(p(sp, 0x78), 0);
        v_sub(c, p(sp, 0x70), PIVOT, p(sp, 0xa0));
        qcopy(c, V0, p(sp, 0xa0), p(sp, 0x90));
        q_rotate(c, p(sp, 0x50), p(sp, 0x90), p(sp, 0xa0));
        qcopy(c, V0, p(sp, 0xa0), p(sp, 0x80));
        v_add(c, p(sp, 0x80), p(sp, 0x40), p(sp, 0xa0));
        qcopy(c, V0, p(sp, 0xa0), p(sp, 0x60));
        v_sub(c, p(sp, 0x60), hip, p(sp, 0x80));
        qcopy(c, V0, p(sp, 0x80), p(sp, 0x30));
        // The swing axis: reached-foot direction × wanted-foot direction.
        v_normalize(c, p(sp, 0x30), p(sp, 0x80));
        qcopy(c, V0, p(sp, 0x80), p(sp, 0x70));
        v_normalize(c, sp, p(sp, 0x90));
        qcopy(c, V0, p(sp, 0x90), p(sp, 0x80));
        v_cross(c, p(sp, 0x70), p(sp, 0x80), p(sp, 0x90));
        qcopy(c, V0, p(sp, 0x90), p(sp, 0x60));
        let len = v_length(c, p(sp, 0x60), V1, 0);
        if fcc(c, eps.lt(len)) {
            c.cycles += 42;
            v_div_by(c, 0, p(sp, 0x60), p(sp, 0x60));
            setf(c, 12, len);
            call_ptr(c, callee::MATH_ASIN, 0x0011_f894)?;
            let a = fr(c, 0);
            setf(c, 12, a * half);
            c.g[A0][0] = sx(p(sp, 0x128));
            c.g[A1][0] = sx(p(sp, 0x12c));
            call_ptr(c, callee::MATH_SIN_COS, 0x0011_f8a4)?;
            axis_quat_v(c, p(sp, 0x60), p(sp, 0x128), p(sp, 0x80));
            qcopy(c, V0, p(sp, 0x80), p(sp, 0x70));
            q_mul(c, p(sp, 0x70), p(hip, 0x10), p(sp, 0x90));
            c.lq(V0, p(sp, 0x90));
            c.sq(V0, p(sp, 0x80));
            c.sq(V0, p(hip, 0x10));
        }

        // The knee transform from the (swung) hip.
        c.cycles += 61;
        q_mul(c, p(hip, 0x10), p(sp, 0x20), p(sp, 0xf0));
        frame.restore(c, S0, 0x180);
        c.lq(V0, p(sp, 0xf0));
        frame.restore(c, S1, 0x170);
        c.sq(V0, p(sp, 0xd0));
        c.sq(V0, p(sp, 0xe0));
        v_sub(c, p(sp, 0x10), PIVOT, p(sp, 0x110));
        frame.restore_ra(c, 0x130);
        c.lq(V0, p(sp, 0x110));
        frame.restore(c, S3, 0x150);
        c.sq(V0, p(sp, 0x100));
        q_rotate(c, p(hip, 0x10), p(sp, 0x100), p(sp, 0x110));
        frame.restore_fpr(c, 23, 0x1a8);
        c.lq(V0, p(sp, 0x110));
        frame.restore_fpr(c, 22, 0x1a0);
        c.sq(V0, p(sp, 0xf0));
        v_add(c, p(sp, 0xf0), hip, p(sp, 0x110));
        frame.restore_fpr(c, 21, 0x198);
        c.lq(V1, p(sp, 0x110));
        c.lq(V0, p(sp, 0xd0));
        c.sq(V1, p(sp, 0xe0));
        c.sq(V0, p(sp, 0x80));
        c.sq(V1, p(sp, 0xc0));
        c.sq(V1, p(sp, 0x70));
        c.sq(V1, knee);
        frame.restore(c, S2, 0x160);
        c.lq(V0, p(sp, 0x80));
        frame.restore_fpr(c, 20, 0x190);
        c.sq(V0, p(knee, 0x10));
        frame.restore(c, S4, 0x140);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_UpdateSkeleton.

/// `void Rider_UpdateSkeleton(Rider* r)` (0x11eb98): the per-frame pose.
/// The root transform (`+0x110` pos, `+0x120` rot) is copied to the stack,
/// spun (`RiderSpinSub_ApplyPoseRotation`, sub-state 5), leaned
/// (`Rider_UpdateBoardLean`) and offset by the spin input (`+0x2c8`); the
/// root bones are reset when `+0x150` is set; `RiderModel_ApplyPose`;
/// then the spine twist (`+0x2bc`, the bone `+0x8a0` turned towards
/// `+0x390`) and tilt (`+0x31c`), the pelvis (`+0x8a4`) re-hung, both legs
/// solved (`+0x318`, not in sub-state 13), and the world matrix (`+0x160`)
/// built from the root transform before `Rider_UpdateSphereTree`.
pub fn rider_update_skeleton(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x1b0);
        frame.save(c, S3, 0x170);
        frame.save(c, S0, 0x1a0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x190);
        frame.save(c, S2, 0x180);
        frame.save(c, S4, 0x160);
        frame.save(c, S5, 0x150);
        frame.save_ra(c, 0x140);
        let r = r32(c, S3);
        let sp = sp(c);
        c.lq(V1, p(r, 0x120));
        c.lq(V0, p(r, 0x110));
        c.sq(V1, p(sp, 0x10));
        c.sq(V0, sp);
        callee::RIDER_GET_SUB_STATE.call(c, 0x0011_ebd0)?;
        c.g[V1][0] = 5;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 5;
            c.g[A0][0] = lw(c, p(r, rider::STATE));
            c.g[A1][0] = c.g[SP][0];
            let spin = r32(c, A0).wrapping_add(0x230);
            c.g[A0][0] = sx(spin);
            call_ptr(c, callee::RIDER_SPIN_SUB_APPLY_POSE_ROTATION, 0x0011_ebec)?;
        }
        c.g[A0][0] = c.g[S3][0];
        c.cycles += 8;
        c.g[A1][0] = c.g[SP][0];
        callee::RIDER_UPDATE_BOARD_LEAN.call(c, 0x0011_ebf8)?;
        c.g[S5][0] = sx(p(r, 0x160));
        c.g[S0][0] = sx(p(sp, 0x20));
        c.g[S4][0] = sx(p(sp, 0x10));
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 1;
        // A loop the compiler left behind: two passes, nothing in it.
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = c.g[S3][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0011_ec10);
            }
        }

        // The pose root: the transform, offset along AXIS_Y by the spin.
        c.cycles += 10;
        c.lq(V1, sp);
        c.lq(V0, p(sp, 0x10));
        c.sq(V1, p(sp, 0x20));
        c.sq(V1, p(sp, 0x40));
        c.sq(V0, p(sp, 0x30));
        c.sq(V0, p(sp, 0x50));
        callee::RIDER_GET_SUB_STATE.call(c, 0x0011_ec48)?;
        c.g[V1][0] = 0xd;
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 6;
            let spin = ldf(c, 1, p(r, 0x2c8));
            c.f[0] = 0;
            if !fcc(c, spin.eq_s(FpuF32::ZERO)) {
                c.cycles += 29;
                c.g[V1][0] = sx(AXIS_Y);
                v_scale(c, V0, 1, AXIS_Y, p(sp, 0x70));
                qcopy(c, V1, p(sp, 0x70), p(sp, 0x60));
                q_rotate(c, p(sp, 0x30), p(sp, 0x60), p(sp, 0x90));
                qcopy(c, V0, p(sp, 0x90), p(sp, 0x80));
                v_add(c, p(sp, 0x20), p(sp, 0x80), p(sp, 0x20));
            }
        }
        c.g[V0][0] = lw(c, p(r, 0x150));
        c.cycles += 2;
        c.g[A2][0] = 0x50_0000;
        if c.g[V0][0] != 0 {
            c.cycles += 35;
            reset_root_bones(c, r);
        }

        // Forward kinematics.
        c.cycles += 8;
        c.g[A0][0] = lw(c, p(r, rider::MODEL));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::RIDER_MODEL_APPLY_POSE, 0x0011_ed80)?;
        c.f[10] = 0;
        let twist = ldf(c, 0, p(r, 0x2bc));
        if !fcc(c, twist.eq_s(FpuF32::ZERO)) {
            c.cycles += 51;
            spine_twist(c, r)?;
            let bone = r32(c, S0);
            c.cycles += 5;
            c.f[0] = 0;
            let tilt = fr(c, 1);
            if fcc(c, FpuF32::ZERO.lt(tilt)) {
                c.cycles += 20;
                v_scale(c, V1, 1, p(r, 0x370), p(sp, 0xa0));
                c.lq(V0, p(sp, 0xa0));
                ldf(c, 0, p(r, 0x2bc));
                c.sq(V0, p(sp, 0x90));
                v_scale(c, V1, 0, p(sp, 0x90), p(sp, 0xa0));
                qcopy(c, V0, p(sp, 0xa0), p(sp, 0x80));
                v_add(c, bone, p(sp, 0x80), bone);
            }
            c.g[A0][0] = lw(c, p(r, rider::MODEL));
            c.cycles += 80;
            rehang_pelvis(c, r, bone);
        }

        // Legs.
        c.cycles += 5;
        c.g[A0][0] = c.g[S3][0];
        callee::RIDER_GET_SUB_STATE.call(c, 0x0011_f154)?;
        c.g[V1][0] = 0xd;
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 6;
            let ik = ldf(c, 1, p(r, 0x318));
            c.f[0] = 0;
            let off = fcc(c, ik.eq_s(FpuF32::ZERO));
            c.g[S2][0] = 0x50_0000;
            if !off {
                c.cycles += 108;
                c.g[V1][0] = lw(c, p(r, rider::MODEL));
                c.g[V0][0] = lw(c, p(r, 0x8a4));
                c.g[S1][0] = sx(p(sp, 0x60));
                let m = r32(c, V1);
                c.g[S0][0] = lw(c, p(m, model::BONES));
                let o = r32(c, V0) << 5;
                c.g[V0][0] = sx(o);
                let pelvis = r32(c, S0).wrapping_add(o);
                c.g[S0][0] = sx(pelvis);
                leg(c, r, pelvis, 0x8e0, 0x0011_f25c)?;
                leg(c, r, pelvis, 0x910, 0x0011_f328)?;
            }
        }

        // World matrix from the root transform (quaternion → rows).
        c.cycles += 44;
        let rot = r32(c, S4);
        c.lqc2(4, rot);
        c.lqc2(3, sp);
        quat_to_matrix(c);
        let world = r32(c, S5);
        c.sqc2(3, p(world, 0x30));
        c.sqc2(10, world);
        c.sqc2(11, p(world, 0x10));
        c.sqc2(12, p(world, 0x20));
        c.g[A0][0] = c.g[S3][0];
        callee::RIDER_UPDATE_SPHERE_TREE.call(c, 0x0011_f3b4)?;
        frame.restore(c, S0, 0x1a0);
        frame.restore(c, S1, 0x190);
        frame.restore(c, S2, 0x180);
        frame.restore(c, S3, 0x170);
        frame.restore(c, S4, 0x160);
        frame.restore(c, S5, 0x150);
        frame.restore_ra(c, 0x140);
        frame.close(c);
        Ok(())
    })
}

/// Root bones `+0x8a0` and `+0x8a4`: offsets reset to [`PIVOT`], rotations
/// to the identity; the pose root becomes the rider's `+0x130` transform.
pub(crate) fn reset_root_bones(c: &mut Cpu, r: u32) {
    let sp = sp(c);
    c.g[V1][0] = lw(c, p(r, rider::MODEL));
    c.g[V0][0] = lw(c, p(r, 0x8a0));
    c.g[A2][0] = sx(PIVOT);
    let m = r32(c, V1);
    c.g[A0][0] = lw(c, p(m, model::BONE_OFFSETS));
    c.g[A3][0] = 0x50_0000;
    c.lq(V1, PIVOT);
    let o = r32(c, V0) << 4;
    c.g[V0][0] = sx(o);
    let dst = o.wrapping_add(r32(c, A0));
    c.g[V0][0] = sx(dst);
    c.g[A3][0] = sx(IDENTITY_QUAT);
    c.sq(V1, dst);
    for (index, table, src) in [
        (0x8a0, model::BONE_ROTATIONS, IDENTITY_QUAT),
        (0x8a4, model::BONE_OFFSETS, PIVOT),
        (0x8a4, model::BONE_ROTATIONS, IDENTITY_QUAT),
    ] {
        c.g[A0][0] = lw(c, p(r, rider::MODEL));
        c.g[V0][0] = lw(c, p(r, index));
        let m = r32(c, A0);
        c.g[A1][0] = lw(c, p(m, table));
        c.lq(V1, src);
        let o = r32(c, V0) << 4;
        c.g[V0][0] = sx(o);
        let dst = o.wrapping_add(r32(c, A1));
        c.g[V0][0] = sx(dst);
        c.sq(V1, dst);
    }
    c.lq(A0, p(r, 0x130));
    c.lq(V0, p(r, 0x140));
    c.sq(A0, p(sp, 0x40));
    c.sq(V0, p(sp, 0x50));
}

/// The spine twist (51-cycle block onwards): the bone `+0x8a0` (left in
/// s0) turns about `+0x390 × its x axis` by `+0x2bc` times the angle
/// between them (`asin` of the axis length, mirrored past 90° when the
/// dot product is negative). Ends with f1 = `+0x31c`.
pub(crate) fn spine_twist(c: &mut Cpu, r: u32) -> Flow {
    let sp = sp(c);
    c.g[V1][0] = lw(c, p(r, rider::MODEL));
    c.g[V0][0] = lw(c, p(r, 0x8a0));
    let m = r32(c, V1);
    c.g[A0][0] = lw(c, p(m, model::BONES));
    let o = r32(c, V0) << 5;
    c.g[V0][0] = sx(o);
    let one = li_f(c, 9, 0x3f80_0000);
    let bone = r32(c, A0).wrapping_add(o);
    c.g[S0][0] = sx(bone);
    let min_len = ldf(c, 8, gp(c, -0x7984));
    // The bone's x axis (first matrix column of its quaternion).
    let qy = ldf(c, 4, p(bone, 0x14));
    let qz = ldf(c, 5, p(bone, 0x18));
    let qx = ldf(c, 0, p(bone, 0x10));
    let yy = setf(c, 1, qy * qy);
    let qw = ldf(c, 3, p(bone, 0x1c));
    let zz = setf(c, 7, qz * qz);
    let xz = setf(c, 2, qx * qz);
    stf(c, 10, p(sp, 0x6c));
    let wy = setf(c, 6, qw * qy);
    let t1 = setf(c, 1, yy + zz);
    let xy = setf(c, 0, qx * qy);
    let wz = setf(c, 3, qw * qz);
    let t2 = setf(c, 2, xz - wy);
    let t1 = setf(c, 1, t1 + t1);
    let t0 = setf(c, 0, xy + wz);
    setf(c, 2, t2 + t2);
    setf(c, 1, one - t1);
    setf(c, 0, t0 + t0);
    stf(c, 2, p(sp, 0x68));
    stf(c, 1, p(sp, 0x60));
    stf(c, 0, p(sp, 0x64));
    v_cross(c, p(r, 0x390), p(sp, 0x60), p(sp, 0x80));
    qcopy(c, V0, p(sp, 0x80), p(sp, 0x70));
    let len = v_length(c, p(sp, 0x70), V1, 12);
    if !fcc(c, min_len.lt(len)) {
        ldf(c, 1, p(r, 0x31c));
        return Ok(());
    }
    c.cycles += 20;
    v_div_by(c, 12, p(sp, 0x70), p(sp, 0x70));
    v_dot(c, p(r, 0x390), p(sp, 0x60));
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    let dot = fr(c, 0);
    let zero = fr(c, 10);
    let behind = fcc(c, dot.lt(zero));
    setf(c, 12, len.min(one));
    let sincos = if behind {
        c.cycles += 20;
        call_ptr(c, callee::MATH_ASIN, 0x0011_eeb8)?;
        let k = ldf(c, 1, gp(c, -0x7980));
        c.g[A0][0] = sx(p(sp, 0x130));
        let twist = ldf(c, 2, p(r, 0x2bc));
        c.g[A1][0] = sx(p(sp, 0x134));
        let a = fr(c, 0);
        let t = setf(c, 0, a - k);
        let half = li_f(c, 12, 0x3f00_0000);
        let t = setf(c, 0, t * twist);
        setf(c, 12, t * half);
        call_ptr(c, callee::MATH_SIN_COS, 0x0011_eee0)?;
        p(sp, 0x130)
    } else {
        c.cycles += 19;
        call_ptr(c, callee::MATH_ASIN, 0x0011_ef08)?;
        let k = ldf(c, 12, gp(c, -0x797c));
        c.g[A0][0] = sx(p(sp, 0x138));
        let twist = ldf(c, 1, p(r, 0x2bc));
        c.g[A1][0] = sx(p(sp, 0x13c));
        let a = fr(c, 0);
        let t = setf(c, 12, k - a);
        let half = li_f(c, 0, 0x3f00_0000);
        let t = setf(c, 12, t * twist);
        setf(c, 12, t * half);
        call_ptr(c, callee::MATH_SIN_COS, 0x0011_ef30)?;
        p(sp, 0x138)
    };
    c.cycles += 23;
    axis_quat_v(c, p(sp, 0x70), sincos, p(sp, 0x90));
    qcopy(c, V0, p(sp, 0x90), p(sp, 0x80));
    q_mul(c, p(sp, 0x80), p(bone, 0x10), p(sp, 0xa0));
    c.lq(V0, p(sp, 0xa0));
    c.sq(V0, p(sp, 0x90));
    c.sq(V0, p(bone, 0x10));
    ldf(c, 1, p(r, 0x31c));
    Ok(())
}

/// The pelvis (`+0x8a4`) re-hung from the twisted spine bone `bone`
/// (80-cycle block; a0 = the model): rotation `bone.rot ⊗ local rot`,
/// position `bone.pos + rotate(bone.rot, scaled offset - pivot)`.
pub(crate) fn rehang_pelvis(c: &mut Cpu, r: u32, bone: u32) {
    let sp = sp(c);
    let m = r32(c, A0);
    c.g[A2][0] = lw(c, p(r, 0x8a4));
    c.g[V0][0] = lw(c, p(m, model::BONE_OFFSETS));
    let i = r32(c, A2);
    let o4 = i << 4;
    c.g[A1][0] = sx(o4);
    let s_x = ldf(c, 4, p(m, model::SCALE));
    let bp = o4.wrapping_add(r32(c, V0));
    c.g[V0][0] = sx(bp);
    let s_y = ldf(c, 6, p(m, model::SCALE + 4));
    let x = ldf(c, 0, bp);
    c.g[A2][0] = sx(i << 5);
    let w = ldf(c, 5, p(bp, 0xc));
    setf(c, 0, x * s_x);
    let y = ldf(c, 1, p(bp, 4));
    let z = ldf(c, 2, p(bp, 8));
    let s_z = ldf(c, 3, p(m, model::SCALE + 8));
    setf(c, 1, y * s_y);
    let s_w = ldf(c, 4, p(m, model::SCALE + 0xc));
    setf(c, 2, z * s_z);
    stf(c, 0, p(sp, 0xa0));
    setf(c, 5, w * s_w);
    stf(c, 1, p(sp, 0xa4));
    stf(c, 2, p(sp, 0xa8));
    stf(c, 5, p(sp, 0xac));
    qcopy(c, V0, p(sp, 0xa0), p(sp, 0x80));
    c.g[V1][0] = lw(c, p(m, model::BONE_ROTATIONS));
    let rot = r32(c, V1).wrapping_add(o4);
    c.g[V1][0] = sx(rot);
    qcopy(c, V0, rot, p(sp, 0x90));
    c.g[A1][0] = lw(c, p(m, model::BONES));
    q_mul(c, p(bone, 0x10), p(sp, 0x90), p(sp, 0x100));
    c.lq(V0, p(sp, 0x100));
    let dst = r32(c, A1).wrapping_add(r32(c, A2));
    c.g[A1][0] = sx(dst);
    c.sq(V0, p(sp, 0xe0));
    c.sq(V0, p(sp, 0xf0));
    c.g[V0][0] = sx(PIVOT);
    v_sub(c, p(sp, 0x80), PIVOT, p(sp, 0x120));
    qcopy(c, V0, p(sp, 0x120), p(sp, 0x110));
    q_rotate(c, p(bone, 0x10), p(sp, 0x110), p(sp, 0x120));
    qcopy(c, V0, p(sp, 0x120), p(sp, 0x100));
    v_add(c, p(sp, 0x100), bone, p(sp, 0x120));
    c.lq(V1, p(sp, 0x120));
    c.lq(V0, p(sp, 0xe0));
    c.sq(V1, p(sp, 0xf0));
    c.sq(V0, p(sp, 0xc0));
    c.sq(V1, p(sp, 0xd0));
    c.sq(V1, p(sp, 0xb0));
    c.sq(V1, dst);
    qcopy(c, V0, p(sp, 0xc0), p(dst, 0x10));
}

/// One leg: the hip target `pelvis ⊗ {offset (desc+0x10), rot (desc+0x20)}`
/// on the stack (s1 = sp+0x60), then `Rider_SolveLegIK(r, desc, &target,
/// +0x318)`.
pub(crate) fn leg(c: &mut Cpu, r: u32, pelvis: u32, desc: u32, ret: u32) -> Flow {
    let sp = sp(c);
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = sx(p(r, desc));
    c.g[A2][0] = c.g[S1][0];
    q_mul(c, p(pelvis, 0x10), p(r, desc + 0x20), p(sp, 0xb0));
    c.lq(V0, p(sp, 0xb0));
    c.sq(V0, p(sp, 0x90));
    c.sq(V0, p(sp, 0xa0));
    v_sub(c, p(r, desc + 0x10), PIVOT, p(sp, 0xd0));
    qcopy(c, V0, p(sp, 0xd0), p(sp, 0xc0));
    q_rotate(c, p(pelvis, 0x10), p(sp, 0xc0), p(sp, 0xd0));
    qcopy(c, V0, p(sp, 0xd0), p(sp, 0xb0));
    v_add(c, p(sp, 0xb0), pelvis, p(sp, 0xd0));
    c.lq(V0, p(sp, 0xd0));
    c.lq(V1, p(sp, 0x90));
    ldf(c, 12, p(r, 0x318));
    c.sq(V0, p(sp, 0x60));
    c.sq(V1, p(sp, 0x70));
    c.sq(V0, p(sp, 0xa0));
    c.sq(V0, p(sp, 0x80));
    callee::RIDER_SOLVE_LEG_IK.call(c, ret)
}

/// Rotation rows vf10..vf12 of the quaternion in vf4 (vf3, the position,
/// untouched), as the original's VU0 sequence computes them.
pub(crate) fn quat_to_matrix(c: &mut Cpu) {
    let vu = c.vu0();
    vu.add(0b1110, 1, 0, Src::Bc(0, W));
    vu.add(0b1110, 5, 4, Src::Vec(4));
    vu.sub(0b0001, 10, 10, Src::Vec(10));
    vu.sub(0b0001, 11, 11, Src::Vec(11));
    vu.sub(0b0001, 12, 12, Src::Vec(12));
    vu.mul(0b1110, 6, 5, Src::Vec(4));
    vu.mul(0b1110, 7, 5, Src::Bc(4, W));
    vu.opmula(5, 4);
    vu.madd(0b1110, 8, 0, Src::Vec(0));
    vu.suba(0b1000, 1, Src::Bc(6, Y));
    vu.msub(0b1000, 10, 1, Src::Bc(6, Z));
    vu.suba(0b0100, 1, Src::Bc(6, Z));
    vu.msub(0b0100, 11, 1, Src::Bc(6, X));
    vu.suba(0b0010, 1, Src::Bc(6, X));
    vu.msub(0b0010, 12, 1, Src::Bc(6, Y));
    vu.adda(0b0100, 0, Src::Bc(8, Z));
    vu.madd(0b0100, 10, 1, Src::Bc(7, Z));
    vu.adda(0b0010, 0, Src::Bc(8, X));
    vu.madd(0b0010, 11, 1, Src::Bc(7, X));
    vu.adda(0b0100, 0, Src::Bc(8, X));
    vu.msub(0b0100, 12, 1, Src::Bc(7, X));
    vu.adda(0b0010, 0, Src::Bc(8, Y));
    vu.msub(0b0010, 10, 1, Src::Bc(7, Y));
    vu.adda(0b1000, 0, Src::Bc(8, Z));
    vu.msub(0b1000, 11, 1, Src::Bc(7, Z));
    vu.adda(0b1000, 0, Src::Bc(8, Y));
    vu.madd(0b1000, 12, 1, Src::Bc(7, Y));
}

// ---------------------------------------------------------------------------
// Rider_UpdateSpeedAnims.

/// One body-part animation channel of `Rider_UpdateSpeedAnims`.
pub(crate) struct Channel {
    /// Animation layer.
    layer: u64,
    /// Seven animation ids: idle, side +/−, forward/back, still, upside down.
    first_anim: u64,
    /// The bone whose rotation orients the channel.
    bone: u32,
    /// The local rotation used when `+0x320` is set, and otherwise.
    quat_alt: u32,
    quat: u32,
    /// The next channel's enable flag (loaded in the original's delay
    /// slots), none for the last.
    next_enable: Option<u32>,
    ret_anim_id: u32,
    ret_play: u32,
}

pub(crate) const CHANNELS: [Channel; 3] = [
    Channel {
        layer: 3,
        first_anim: 0x19b,
        bone: 0x8a8,
        quat_alt: 0x960,
        quat: 0x950,
        next_enable: Some(0x970),
        ret_anim_id: 0x0012_050c,
        ret_play: 0x0012_07b4,
    },
    Channel {
        layer: 4,
        first_anim: 0x1a2,
        bone: 0x8ac,
        quat_alt: 0x990,
        quat: 0x980,
        next_enable: Some(0x9a0),
        ret_anim_id: 0x0012_07cc,
        ret_play: 0x0012_0a74,
    },
    Channel {
        layer: 5,
        first_anim: 0x1a9,
        bone: 0x89c,
        quat_alt: 0x9c0,
        quat: 0x9b0,
        next_enable: None,
        ret_anim_id: 0x0012_0a8c,
        ret_play: 0x0012_0d34,
    },
];

/// `void Rider_UpdateSpeedAnims(Rider* r)` (0x120378): the playback rate of
/// animation layers 3..5 follows the speed (`|+0x1e0 - +0x3d0| / max`,
/// clamped to [0.5, 2]); every sixth frame (phase `+0x86c`) each enabled
/// channel (`+0x940`, `+0x970`, `+0x9a0`) picks its animation from how the
/// velocity lies in the channel bone's frame and plays it if it changed.
pub fn rider_update_speed_anims(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x90);
        let max_speed = ldf(c, 1, gp(c, -0x7944));
        frame.save(c, S0, 0x60);
        frame.save_fpr(c, 23, 0x88);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0x80);
        frame.save_fpr(c, 20, 0x70);
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 21, 0x78);
        let r = r32(c, S0);
        let sp = sp(c);
        v_sub(c, p(r, 0x1e0), p(r, 0x3d0), p(sp, 0x10));
        let half = li_f(c, 0, 0x3f00_0000);
        c.lq(V0, p(sp, 0x10));
        setf(c, 20, half);
        ldf(c, 23, gp(c, -0x7940));
        c.sq(V0, sp);
        let speed = v_length(c, sp, V1, 21);
        ldf(c, 22, gp(c, -0x793c));
        let ratio = setf(c, 1, speed.div(max_speed));
        let two = li_f(c, 2, 0x4000_0000);
        if fcc(c, half.le(ratio)) {
            setf(c, 20, ratio.min(two));
        }

        // Layer playback rates.
        c.cycles += 5;
        c.g[A0][0] = lw(c, p(r, rider::ANIM));
        for (layer, ret_get, ret_set) in [
            (3u64, 0x0012_041c, 0x0012_042c),
            (4, 0x0012_043c, 0x0012_044c),
            (5, 0x0012_045c, 0x0012_0470),
        ] {
            let last = layer == 5;
            if layer != 3 {
                c.cycles += 4;
            }
            c.g[A1][0] = layer;
            call_ptr(c, callee::ANIM_CONTROLLER_GET_LAYER_SEQUENCE, ret_get)?;
            let found = c.g[V0][0] != 0;
            if !last {
                c.g[A0][0] = lw(c, p(r, rider::ANIM));
            }
            if found {
                c.cycles += 4;
                if last {
                    c.g[A0][0] = lw(c, p(r, rider::ANIM));
                }
                c.g[A1][0] = layer;
                call_ptr(c, callee::ANIM_CONTROLLER_GET_LAYER_SEQUENCE, ret_set)?;
                let seq = r32(c, V0);
                stf(c, 20, p(seq, 0x90));
                if !last {
                    c.g[A0][0] = lw(c, p(r, rider::ANIM));
                }
            }
        }

        // Every sixth frame: the channels.
        c.cycles += 6;
        callee::AI_GET_FRAME_COUNTER.call(c, 0x0012_047c)?;
        c.g[V1][0] = 6;
        c.div(0, V0, V1);
        c.cycles += 4;
        c.g[V0][0] = lw(c, p(r, 0x86c));
        c.g[A0][0] = c.hi[0];
        if c.g[A0][0] == c.g[V0][0] {
            speed_anim_channels(c, r)?;
        }
        frame.restore(c, S0, 0x60);
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 23, 0x88);
        frame.restore_fpr(c, 22, 0x80);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// The channel pass: s1 = "slow" (speed below `gp-0x7938`; never in
/// sub-state 11, which also pushes the velocity copy by 1000 along z).
pub(crate) fn speed_anim_channels(c: &mut Cpu, r: u32) -> Flow {
    c.cycles += 5;
    let slow_limit = ldf(c, 0, gp(c, -0x7938));
    let speed = fr(c, 21);
    let slow = fcc(c, speed.le(slow_limit));
    c.g[S1][0] = 1;
    if !slow {
        c.cycles += 1;
        c.g[S1][0] = 0;
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    callee::RIDER_GET_SUB_STATE.call(c, 0x0012_04bc)?;
    c.g[V1][0] = 0xb;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 12;
        li_f(c, 0, 0x447a_0000);
        c.g[S1][0] = 0;
        let sp = sp(c);
        c.sw(p(sp, 0x10), 0);
        stf(c, 0, p(sp, 0x18));
        c.sw(p(sp, 0x14), 0);
        c.sw(p(sp, 0x1c), 0);
        v_add(c, sp, p(sp, 0x10), sp);
    }
    c.g[V0][0] = lw(c, p(r, 0x940));
    for ch in &CHANNELS {
        speed_anim_channel(c, r, ch)?;
    }
    Ok(())
}

/// One channel (v0 = its enable flag): the wanted animation in a1, played
/// (`AnimController_PlayAnim(ctrl, layer, 0, -1)`) when the layer's
/// current one (a2) differs.
pub(crate) fn speed_anim_channel(c: &mut Cpu, r: u32, ch: &Channel) -> Flow {
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[A0][0] = lw(c, p(r, rider::ANIM));
        c.g[A1][0] = ch.layer;
        call_ptr(c, callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ID, ch.ret_anim_id)?;
        let slow = c.g[S1][0] != 0;
        c.g[A2][0] = c.g[V0][0];
        if slow {
            c.cycles += 2;
            c.g[A1][0] = ch.first_anim;
        } else {
            pick_speed_anim(c, r, ch);
        }
        c.cycles += 2;
        if c.g[A1][0] != c.g[A2][0] {
            c.cycles += 6;
            c.g[A0][0] = lw(c, p(r, rider::ANIM));
            li_f(c, 12, 0xbf80_0000);
            c.g[A2][0] = 0;
            call_ptr(c, callee::ANIM_CONTROLLER_PLAY_ANIM, ch.ret_play)?;
        }
    }
    if let Some(next) = ch.next_enable {
        c.g[V0][0] = lw(c, p(r, next));
    }
    Ok(())
}

/// The channel's animation (into a1) from the velocity (at sp) in the
/// frame `bone.rot ⊗ local rot`: sideways (|side| > `f22`) → +1/+2 by
/// sign, forward/back (|fwd| > `f23`) → +3/+4, otherwise +5, or +6 when
/// upside down.
pub(crate) fn pick_speed_anim(c: &mut Cpu, r: u32, ch: &Channel) {
    c.cycles += 3;
    c.g[V0][0] = lw(c, p(r, 0x320));
    let alt = c.g[V0][0] != 0;
    c.g[A0][0] = lw(c, p(r, rider::MODEL));
    let local = if alt {
        c.cycles += 18;
        ch.quat_alt
    } else {
        c.cycles += 17;
        ch.quat
    };
    let sp = sp(c);
    let m = r32(c, A0);
    c.g[V0][0] = lw(c, p(r, ch.bone));
    c.g[V1][0] = lw(c, p(m, model::BONES));
    let o = r32(c, V0) << 5;
    c.g[V0][0] = sx(o);
    let bone = r32(c, V1).wrapping_add(o);
    c.g[V1][0] = sx(bone);
    q_mul(c, p(bone, 0x10), p(r, local), p(sp, 0x30));
    c.lq(V0, p(sp, 0x30));

    // Matrix rows of the quaternion: x row (sp+0x20) and y row.
    c.cycles += 61;
    c.sq(V0, p(sp, 0x10));
    c.sq(V0, p(sp, 0x20));
    let qy = ldf(c, 3, p(sp, 0x14));
    let qz = ldf(c, 0, p(sp, 0x18));
    let qx = ldf(c, 4, p(sp, 0x10));
    let yy = setf(c, 11, qy * qy);
    let zz = setf(c, 1, qz * qz);
    let qw = ldf(c, 6, p(sp, 0x1c));
    let xx = setf(c, 7, qx * qx);
    let one = li_f(c, 9, 0x3f80_0000);
    let xz = setf(c, 2, qx * qz);
    c.sw(p(sp, 0x2c), 0);
    let wy = setf(c, 10, qw * qy);
    let wz = setf(c, 8, qw * qz);
    let xy = setf(c, 5, qx * qy);
    let t1 = setf(c, 1, yy + zz);
    let yz = setf(c, 0, qz * qy);
    let t7 = setf(c, 7, xx + yy);
    let wx = setf(c, 6, qw * qx);
    let t3 = setf(c, 3, xz - wy);
    let t5 = setf(c, 5, xy + wz);
    let t1 = setf(c, 1, t1 + t1);
    let t2 = setf(c, 2, xz + wy);
    let t0 = setf(c, 0, yz - wx);
    let t7 = setf(c, 7, t7 + t7);
    setf(c, 1, one - t1);
    setf(c, 3, t3 + t3);
    setf(c, 5, t5 + t5);
    setf(c, 2, t2 + t2);
    stf(c, 1, p(sp, 0x20));
    setf(c, 9, one - t7);
    stf(c, 3, p(sp, 0x28));
    setf(c, 0, t0 + t0);
    stf(c, 5, p(sp, 0x24));
    v_dot(c, sp, p(sp, 0x20));
    c.qmfc2(V0, 4);
    c.mtc1(V0, 3);
    let fwd = fr(c, 3);
    stf(c, 2, p(sp, 0x20));
    stf(c, 0, p(sp, 0x24));
    stf(c, 9, p(sp, 0x28));
    c.sw(p(sp, 0x2c), 0);
    v_dot(c, sp, p(sp, 0x20));
    c.qmfc2(V0, 4);
    c.mtc1(V0, 2);
    let side = fr(c, 2);
    let side_abs = setf(c, 1, side.abs());
    let side_limit = fr(c, 22);
    if fcc(c, side_limit.lt(side_abs)) {
        c.cycles += 5;
        let fwd_abs = setf(c, 0, fwd.abs());
        let mostly_side = fcc(c, fwd_abs.lt(side_abs));
        let zero = ldf(c, 0, p(sp, 0x2c));
        if mostly_side {
            c.cycles += 4;
            let positive = fcc(c, zero.lt(side));
            c.g[A1][0] = ch.first_anim + 2;
            if positive {
                c.cycles += 2;
                c.g[A1][0] = ch.first_anim + 1;
            }
            return;
        }
    }
    c.cycles += 5;
    let fwd_abs = setf(c, 0, fwd.abs());
    let fwd_limit = fr(c, 23);
    let moving = fcc(c, fwd_limit.lt(fwd_abs));
    let qx = ldf(c, 4, p(sp, 0x10));
    if moving {
        c.cycles += 5;
        c.f[0] = 0;
        let back = fcc(c, fwd.lt(FpuF32::ZERO));
        c.g[A1][0] = ch.first_anim + 3;
        if back {
            c.cycles += 2;
            c.g[A1][0] = ch.first_anim + 4;
        }
        return;
    }
    // Still: the z row decides whether the part is upside down.
    c.cycles += 37;
    c.g[A1][0] = ch.first_anim + 5;
    let qz = ldf(c, 5, p(sp, 0x18));
    let xx = setf(c, 8, qx * qx);
    let qy = ldf(c, 1, p(sp, 0x14));
    let qw = ldf(c, 3, p(sp, 0x1c));
    let zz = setf(c, 0, qz * qz);
    let yz = setf(c, 2, qy * qz);
    let one = li_f(c, 6, 0x3f80_0000);
    let wx = setf(c, 7, qw * qx);
    c.sw(p(sp, 0x2c), 0);
    let t0 = setf(c, 0, zz + xx);
    let xy = setf(c, 1, qy * qx);
    let zero = ldf(c, 8, p(sp, 0x2c));
    let wz = setf(c, 3, qw * qz);
    let t0 = setf(c, 0, t0 + t0);
    let t2 = setf(c, 2, yz + wx);
    let t1 = setf(c, 1, xy - wz);
    setf(c, 6, one - t0);
    setf(c, 2, t2 + t2);
    setf(c, 1, t1 + t1);
    stf(c, 6, p(sp, 0x24));
    stf(c, 2, p(sp, 0x28));
    stf(c, 1, p(sp, 0x20));
    v_dot(c, sp, p(sp, 0x20));
    c.qmfc2(V0, 4);
    c.mtc1(V0, 0);
    let up = fr(c, 0);
    if fcc(c, up.lt(zero)) {
        c.g[A1][0] = ch.first_anim + 6;
    }
}

// ---------------------------------------------------------------------------
// Rider_UpdateSlewedParams.

/// The slewed parameters: 19 `{value, rate, target}` triples from `+0x1f0`.
pub(crate) const SLEWED_FIRST: u32 = 0x1f0;

pub(crate) const SLEWED_STRIDE: u32 = 0xc;

pub(crate) const SLEWED_COUNT: u32 = 19;

/// `void Rider_UpdateSlewedParams(Rider* r)` (0x1211f8):
/// `Rider_UpdateSpeedAnims(r)`, then every slewed parameter moves towards
/// its target by at most its rate.
pub fn rider_update_slewed_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let r = r32(c, S0);
        callee::RIDER_UPDATE_SPEED_ANIMS.call(c, 0x0012_120c)?;
        for i in 0..SLEWED_COUNT {
            if i > 0 {
                c.cycles += 10;
                let prev = r32(c, V0);
                stf(c, 1, prev);
            }
            let param = p(r, SLEWED_FIRST + i * SLEWED_STRIDE);
            c.g[V0][0] = sx(param);
            slew(c, param);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        let last = r32(c, V0);
        stf(c, 1, last);
        frame.close(c);
        Ok(())
    })
}

/// `f1 = value moved towards target by rate` (f0 = target ± rate, f2 =
/// rate, f3 = value as the original leaves them).
pub(crate) fn slew(c: &mut Cpu, param: u32) {
    let target = ldf(c, 1, p(param, 8));
    let rate = ldf(c, 2, p(param, 4));
    let value = ldf(c, 3, param);
    let hi = setf(c, 0, target + rate);
    if fcc(c, hi.lt(value)) {
        c.cycles += 2;
        setf(c, 1, value - rate);
        return;
    }
    let lo = setf(c, 0, target - rate);
    c.cycles += 4;
    if fcc(c, value.lt(lo)) {
        setf(c, 1, value + rate);
    }
}

// ---------------------------------------------------------------------------
// Register and memory helpers (32-bit address arithmetic, sign-extended
// results, as the compiled code computes them). The pure ones take `&Cpu`;
// the ones that change state are methods of [`Ops`], so their address
// arguments can read the registers (two-phase borrows).

/// The low word of a register.
#[inline(always)]
pub(crate) fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// The address `reg + off`.
#[inline(always)]
pub(crate) fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r(c, reg).wrapping_add(off)
}

/// A `gp_2`-relative global.
#[inline(always)]
pub(crate) fn gp_2(c: &Cpu, off: i32) -> u32 {
    r(c, GP).wrapping_add(off as u32)
}

/// The current stack pointer.
#[inline(always)]
pub(crate) fn sp_2(c: &Cpu) -> u32 {
    r(c, SP)
}

/// `slt`-style signed compare of two registers (64-bit).
#[inline(always)]
pub(crate) fn slt(c: &Cpu, a: usize, b: usize) -> u64 {
    ((c.g[a][0] as i64) < (c.g[b][0] as i64)) as u64
}

/// Register, memory and VU0-idiom operations, issued op for op as the
/// compiled code does.
pub(crate) trait Ops {
    /// `reg = sx(v)`.
    fn set_r(&mut self, reg: usize, v: u32);
    /// `reg += n` (`addiu`).
    fn inc_r(&mut self, reg: usize, n: u32);
    /// `lw reg, a`.
    fn lw_r(&mut self, reg: usize, a: u32);
    /// `sw reg, a`.
    fn sw_r(&mut self, a: u32, reg: usize);
    /// `lq reg, from; sq reg, to`.
    fn qcopy(&mut self, reg: usize, from: u32, to: u32);
    /// The dot-product tail: `vf6.w = 1`, `vf4 = vf3 * vf{rhs}`, then
    /// `vf4.w = vf4.x + vf4.y + vf4.z` through ACC.
    fn vdot_regs(&mut self, rhs: u8);
    /// `dot([a], [b])` into `vf4.w` (`vf3 = [a]`, `vf5 = [b]`), then
    /// `reg = vf4` (`qmfc2`) and `f{fr} = reg` (`mtc1`).
    fn vdot(&mut self, a: u32, b: u32, reg: usize, fr: usize);
    /// `|[a]|^2` into `vf4.w` (`vf3 = [a]`).
    fn vlen2(&mut self, a: u32);
    /// `|[a]|` (VU0 `sqrt`), into `reg` (`cfc2` of Q) and `f{fr}`.
    fn vlen(&mut self, a: u32, reg: usize, fr: usize);
    /// `vf5 = vf3 / |vf3|` (after [`Ops::vlen2`]).
    fn vnorm(&mut self);
    /// `vf5 = [v] * reg.x` (`vf4 = [v]`, `vf3 = reg`).
    fn vscale(&mut self, v: u32, reg: usize);
    /// `vf5 = [a] - [b]` (`vf3 = [a]`, `vf4 = [b]`).
    fn vsub(&mut self, a: u32, b: u32);
    /// `vf5 = [a] + [b]` (`vf3 = [a]`, `vf4 = [b]`).
    fn vadd(&mut self, a: u32, b: u32);
    /// `vf4 = [v] / reg` (`Q = vf0.w / vf3.x` with `vf3 = reg`).
    fn vdiv(&mut self, v: u32, reg: usize);
    /// `sqc2 vf{vf}, tmp`, `lq reg, tmp`, then `sq reg` to each of `to`
    /// (the compiler's VU0-to-memory round trip through a GPR).
    fn vstore(&mut self, vf: usize, tmp: u32, reg: usize, to: &[u32]);
}

impl Ops for Cpu {
    #[inline(always)]
    fn set_r(&mut self, reg: usize, v: u32) {
        self.g[reg][0] = sx(v);
    }
    #[inline(always)]
    fn inc_r(&mut self, reg: usize, n: u32) {
        let v = r(self, reg).wrapping_add(n);
        self.g[reg][0] = sx(v);
    }
    #[inline(always)]
    fn lw_r(&mut self, reg: usize, a: u32) {
        let v = self.lw(a);
        self.g[reg][0] = v;
    }
    #[inline(always)]
    fn sw_r(&mut self, a: u32, reg: usize) {
        let v = self.g[reg][0];
        self.sw(a, v);
    }
    #[inline(always)]
    fn qcopy(&mut self, reg: usize, from: u32, to: u32) {
        self.lq(reg, from);
        self.sq(reg, to);
    }
    #[inline]
    fn vdot_regs(&mut self, rhs: u8) {
        let v = self.vu0();
        v.add(0b1000, 6, 0, Src::Bc(0, W));
        v.mul(0b1111, 4, 3, Src::Vec(rhs));
        v.adda(0b1000, 4, Src::Bc(4, Y));
        v.madda(0b1000, 6, Src::Bc(4, Z));
        v.madd(0b1000, 4, 6, Src::Bc(4, W));
    }
    #[inline]
    fn vdot(&mut self, a: u32, b: u32, reg: usize, fr: usize) {
        self.lqc2(3, a);
        self.lqc2(5, b);
        self.vdot_regs(5);
        self.qmfc2(reg, 4);
        self.mtc1(reg, fr);
    }
    #[inline]
    fn vlen2(&mut self, a: u32) {
        self.lqc2(3, a);
        self.vdot_regs(3);
    }
    #[inline]
    fn vlen(&mut self, a: u32, reg: usize, fr: usize) {
        self.vlen2(a);
        self.vu0().sqrt(4, 0);
        self.vu0().waitq();
        self.cfc2(reg, 22);
        self.mtc1(reg, fr);
    }
    #[inline]
    fn vnorm(&mut self) {
        self.vu0().rsqrt(0, 3, 4, 0);
        self.vu0().waitq();
        self.vu0().mul(0b1111, 5, 3, Src::Q);
    }
    #[inline]
    fn vscale(&mut self, v: u32, reg: usize) {
        self.lqc2(4, v);
        self.qmtc2(reg, 3);
        self.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    }
    #[inline]
    fn vsub(&mut self, a: u32, b: u32) {
        self.lqc2(3, a);
        self.lqc2(4, b);
        self.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    }
    #[inline]
    fn vadd(&mut self, a: u32, b: u32) {
        self.lqc2(3, a);
        self.lqc2(4, b);
        self.vu0().add(0b1111, 5, 3, Src::Vec(4));
    }
    #[inline]
    fn vdiv(&mut self, v: u32, reg: usize) {
        self.qmtc2(reg, 3);
        self.vu0().div(0, 3, 3, 0);
        self.lqc2(4, v);
        self.vu0().waitq();
        self.vu0().mul(0b1111, 4, 4, Src::Q);
    }
    #[inline]
    fn vstore(&mut self, vf: usize, tmp: u32, reg: usize, to: &[u32]) {
        self.sqc2(vf, tmp);
        self.lq(reg, tmp);
        for &a in to {
            self.sq(reg, a);
        }
    }
}

/// The epilogue of the rider collision functions: callee-saved GPRs and
/// FPRs back, `ra`, `sp_2 += size`, return.
pub(crate) fn epilogue(
    c: &mut Cpu,
    gprs: &[(usize, u32)],
    fprs: &[(usize, u32)],
    ra_at: u32,
    size: u32,
) {
    let s = sp_2(c);
    for &(reg, off) in gprs {
        c.lq(reg, p(s, off));
    }
    c.g[RA][0] = c.ld(p(s, ra_at));
    for &(f, off) in fprs {
        c.lwc1(f, p(s, off));
    }
    let ra = r(c, RA);
    c.set_r(SP, s.wrapping_add(size));
    c.pc = ra;
}

/// The prologue: `sp_2 -= size`, the callee-saved GPRs (`sq`), `ra` (`sd`)
/// and FPRs (`swc1`).
pub(crate) fn prologue(
    c: &mut Cpu,
    size: u32,
    gprs: &[(usize, u32)],
    ra_at: u32,
    fprs: &[(usize, u32)],
) {
    c.set_r(SP, sp_2(c).wrapping_sub(size));
    let s = sp_2(c);
    for &(reg, off) in gprs {
        c.sq(reg, p(s, off));
    }
    c.sd(p(s, ra_at), c.g[RA][0]);
    for &(f, off) in fprs {
        c.swc1(f, p(s, off));
    }
}

/// A gcc 2.9x virtual call on `obj` (`vt = obj->+0xc`... read by the
/// caller): `a0 = obj + lh(vt + slot)`, target `lw(vt + slot + 4)` in
/// `func_reg`, `jalr` returning to `ret`. `vt_reg` holds the vtable.
pub(crate) fn vcall(
    c: &mut Cpu,
    obj_reg: usize,
    vt_reg: usize,
    slot: u32,
    func_reg: usize,
    ret: u32,
) -> Flow {
    let vt = r(c, vt_reg);
    let delta = c.lh(p(vt, slot));
    c.g[A0][0] = delta;
    let func = c.lw(p(vt, slot + 4));
    c.g[func_reg][0] = func;
    let this = r(c, obj_reg).wrapping_add(r(c, A0));
    c.set_r(A0, this);
    call_ptr(c, func as u32, ret)
}

/// `.rodata` defaults: the up vector and the second ground normal.
pub(crate) const RODATA_UP: u32 = 0x004f_f160;

pub(crate) const RODATA_NORMAL2: u32 = 0x004f_f120;

/// Copy a 0x80-byte contact record (`v0` from, `v1` to, `a0` end) in four
/// 0x20 steps through a1/a2, with the loop's event checks at `head`.
pub(crate) fn copy_contact(c: &mut Cpu, head: u32) {
    loop {
        c.cycles += 7;
        let (from, to) = (r(c, V0), r(c, V1));
        c.lq(A1, from);
        c.lq(A2, p(from, 0x10));
        c.sq(A1, to);
        c.sq(A2, p(to, 0x10));
        c.inc_r(V0, 0x20);
        let more = c.g[V0][0] != c.g[A0][0];
        c.inc_r(V1, 0x20);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

// ---------------------------------------------------------------------------
// Walls.

pub(crate) const FW_FILL: u32 = 0x0010_4ea8;

pub(crate) const FW_PICK: u32 = 0x0010_5008;

pub(crate) const FW_SEEN: u32 = 0x0010_5040;

pub(crate) const FW_COPY: u32 = 0x0010_522c;

pub(crate) const FW_SUM: u32 = 0x0010_5270;

/// The rider's list of wall surface ids touched this frame (`-1`
/// terminated).
pub(crate) const RIDER_WALL_IDS: u32 = 0x5b8;

/// `float Rider_FindWallContact(Rider* r, CollHull* hull, Contact* out)`
/// (0x104e70): collides the hull with the rider's overlap list (up to 64
/// contacts). None: returns -1. Otherwise keeps the touched wall surfaces
/// in the rider's id list, picks the best wall contact (real walls whose
/// normal faces the sphere tree first, then the depth closest to the
/// hull's reference depth, ties by surface / instance id), copies it to
/// `out`, averages the normals of all wall contacts into `out->normal`
/// when there are several, tells the surface's owner, and returns the
/// contact depth.
pub fn rider_find_wall_contact(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        prologue(
            c,
            0x20a0,
            &[
                (S0, 0x2090),
                (S1, 0x2080),
                (S2, 0x2070),
                (S5, 0x2040),
                (S3, 0x2060),
                (S4, 0x2050),
            ],
            0x2030,
            &[],
        );
        c.g[V0][0] = 0x3f;
        c.g[V1][0] = u64::MAX;
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        c.g[S5][0] = c.g[A2][0];
        // The (empty) constructor loop of the 64 contact records.
        loop {
            c.cycles += 7;
            c.inc_r(V0, u32::MAX);
            c.g[A1][0] = c.g[S0][0];
            if c.g[V0][0] == c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, FW_FILL);
            }
        }
        // n = OverlapList_CollideObjects(r->list, hull, contacts, 64, 0)
        c.cycles += 8;
        c.lw_r(A0, at(c, S2, rider::OVERLAP_LIST));
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0x40;
        c.g[T0][0] = 0;
        call_ptr(c, callee::OVERLAP_LIST_COLLIDE_OBJECTS, 0x0010_4ed8)?;
        c.g[S4][0] = c.g[V0][0];
        let s = sp_2(c);
        c.lw_r(V1, p(s, contact::SURFACE));
        if c.g[S4][0] == 0 {
            c.cycles += 4;
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0
            c.mtc1(AT, 0);
            c.lq(S0, p(s, 0x2090));
        } else {
            find_wall_pick(c)?;
            c.cycles += 1;
            let s = sp_2(c);
            c.lq(S0, p(s, 0x2090));
        }
        c.cycles += 8;
        epilogue(
            c,
            &[
                (S1, 0x2080),
                (S2, 0x2070),
                (S3, 0x2060),
                (S4, 0x2050),
                (S5, 0x2040),
            ],
            &[],
            0x2030,
            0x20a0,
        );
        Ok(())
    })
}

/// The contacts are in (s4 of them at `sp_2`, v1 = contact 0's surface).
pub(crate) fn find_wall_pick(c: &mut Cpu) -> Flow {
    let s = sp_2(c);
    // Contact 0's surface starts the id list.
    c.cycles += 2;
    c.set_r(V0, r(c, S1) << 2);
    if c.g[V1][0] != 0 {
        c.cycles += 4;
        c.lw_r(V1, at(c, V1, 0x78));
        c.set_r(V0, r(c, V0).wrapping_add(r(c, S2)));
        c.g[S1][0] = 1;
        c.sw_r(at(c, V0, RIDER_WALL_IDS), V1);
    }
    // f0 = hull->vt[0x18](hull): the reference depth.
    c.cycles += 22;
    c.lw_r(V0, at(c, S0, 0x50));
    c.g[S3][0] = 0;
    vcall(c, S0, V0, 0x18, V1, 0x0010_4f24)?;
    c.lw_r(V1, p(s, contact::MATERIAL));
    c.g[V0][0] = 0xc;
    c.lw_r(A0, p(s, contact::SURFACE));
    c.mov_s(3, 0);
    c.mult(0, V1, V1, V0);
    c.lwc1(0, p(s, contact::DEPTH));
    c.lw_r(A1, at(c, A0, 0x88));
    c.sub_s(0, 0, 3);
    c.g[A0][0] = 0;
    c.abs_s(2, 0);
    c.inc_r(V1, 0x10);
    c.set_r(A1, r(c, A1).wrapping_add(r(c, V1)));
    c.g[V0][0] = c.lhu(at(c, A1, 0x8)) & 1;
    if c.g[V0][0] != 0 {
        // A wall material with a non-zero first field.
        c.cycles += 6;
        c.lwc1(1, r(c, A1));
        c.mtc1(0, 0);
        c.c_eq_s(1, 0);
        if !c.fcc() {
            c.g[A0][0] = 1;
        }
        c.cycles += 1;
    }
    c.lw_r(V0, at(c, S2, rider::SPHERE_TREE));
    // a0 = dot(contact0.pos - tree.center, contact0.normal) < 0
    c.cycles += 22;
    c.g[T4][0] = c.g[A0][0];
    c.vsub(s, at(c, V0, 0x10));
    c.sqc2(5, p(s, 0x2010));
    c.mtc1(0, 1);
    c.lq(V0, p(s, 0x2010));
    c.g[A0][0] = 1;
    c.sq(V0, p(s, 0x2000));
    c.vdot(p(s, 0x2000), p(s, 0x10), V1, 0);
    c.c_lt_s(0, 1);
    if !c.fcc() {
        c.g[A0][0] = 0;
    }
    // Contacts 1..n.
    c.cycles += 5;
    c.g[T1][0] = 1;
    c.g[T3][0] = c.g[A0][0];
    c.g[V0][0] = slt(c, T1, S4);
    c.g[T6][0] = ((c.g[S4][0] as i64) < 2) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.set_r(V0, r(c, S1) << 2);
        c.mov_s(4, 1);
        c.inc_r(V0, RIDER_WALL_IDS);
        c.g[T7][0] = 0xc;
        c.set_r(T2, r(c, V0).wrapping_add(r(c, S2)));
        c.set_r(V0, r(c, T1) << 7);
        loop {
            find_wall_pick_one(c);
            c.cycles += 3;
            c.g[V0][0] = slt(c, T1, S4);
            let more = c.g[V0][0] != 0;
            c.set_r(V0, r(c, T1) << 7);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, FW_PICK);
            }
        }
    }
    // *out = contacts[best]
    c.cycles += 5;
    let s = sp_2(c);
    c.set_r(V0, r(c, S3) << 7);
    c.set_r(T1, r(c, S1) << 2);
    c.set_r(V0, r(c, V0).wrapping_add(s));
    c.g[V1][0] = c.g[S5][0];
    c.set_r(A0, p(r(c, V0), contact::SIZE));
    copy_contact(c, FW_COPY);
    // Several contacts: the normalised sum of the wall normals.
    c.cycles += 2;
    let single = c.g[T6][0] != 0;
    c.g[V0][0] = 0x50_0000;
    if !single {
        c.cycles += 4;
        c.set_r(V0, RODATA_NORMAL2);
        c.qcopy(V1, RODATA_NORMAL2, p(s, 0x2000));
        if (c.g[S4][0] as i64) > 0 {
            c.cycles += 4;
            c.mtc1(0, 1);
            c.g[A2][0] = c.g[S4][0];
            c.g[T0][0] = 0xc;
            c.set_r(A1, p(s, 0x10));
            loop {
                c.cycles += 10;
                c.lw_r(V1, at(c, A1, contact::MATERIAL - contact::NORMAL));
                c.lw_r(V0, at(c, A1, contact::SURFACE - contact::NORMAL));
                c.mult(0, V1, V1, T0);
                c.lw_r(A0, at(c, V0, 0x88));
                c.inc_r(V1, 0x10);
                c.set_r(A0, r(c, A0).wrapping_add(r(c, V1)));
                c.g[V0][0] = c.lhu(at(c, A0, 0x8)) & 1;
                c.g[A3][0] = 0;
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    c.lwc1(0, r(c, A0));
                    c.c_eq_s(0, 1);
                    if !c.fcc() {
                        c.g[A3][0] = 1;
                    }
                }
                c.cycles += 2;
                if c.g[A3][0] != 0 {
                    c.cycles += 5;
                    c.vadd(p(s, 0x2000), r(c, A1));
                    c.sqc2(5, p(s, 0x2000));
                }
                c.inc_r(A2, u32::MAX);
                c.cycles += 2;
                let more = c.g[A2][0] != 0;
                c.inc_r(A1, contact::SIZE);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, FW_SUM);
                }
            }
        }
        c.cycles += 14;
        c.lwc1(0, gp_2(c, -0x7f38));
        c.vlen(p(s, 0x2000), V0, 1);
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.cycles += 10;
            c.mfc1(V1, 1);
            c.vdiv(p(s, 0x2000), V1);
            let out = r(c, S5);
            c.vstore(
                4,
                p(s, 0x2020),
                V0,
                &[p(out, contact::NORMAL), p(s, 0x2010)],
            );
        }
    }
    // Terminate the id list; tell the surface's owner (virtual 0x150).
    c.cycles += 7;
    c.g[V0][0] = u64::MAX;
    c.set_r(V1, r(c, T1).wrapping_add(r(c, S2)));
    c.sw_r(at(c, V1, RIDER_WALL_IDS), V0);
    let out = r(c, S5);
    c.lw_r(A0, p(out, contact::SURFACE));
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.lw_r(A2, at(c, A0, 0xc));
        c.g[A1][0] = c.g[S5][0];
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.lw_r(V0, at(c, A2, 0xc));
            vcall(c, A2, V0, 0x150, V1, 0x0010_536c)?;
        }
        c.cycles += 1;
    }
    let out = r(c, S5);
    c.lwc1(0, p(out, contact::DEPTH));
    Ok(())
}

/// One contact (t1, at `sp_2 + v0`) of the wall pick: records its surface id
/// if new, scores it (wall, facing, depth) against the best (s3).
pub(crate) fn find_wall_pick_one(c: &mut Cpu) {
    let s = sp_2(c);
    c.cycles += 5;
    c.g[T0][0] = c.g[V0][0];
    c.set_r(A0, s.wrapping_add(r(c, V0)));
    c.lw_r(V0, at(c, A0, contact::SURFACE));
    c.set_r(T5, r(c, T1).wrapping_add(1));
    if c.g[V0][0] != 0 {
        // Is the surface id already in the list?
        c.cycles += 2;
        let empty = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = 0;
        let mut known = false;
        if !empty {
            c.cycles += 4;
            c.lw_r(V1, at(c, V0, 0x78));
            c.lw_r(V0, at(c, S2, RIDER_WALL_IDS));
            let first = c.g[V0][0] == c.g[V1][0];
            c.set_r(A1, at(c, S2, RIDER_WALL_IDS));
            let mut found = first;
            if !first {
                c.cycles += 3;
                c.g[A3][0] = c.g[A0][0];
                c.inc_r(A2, 1);
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = slt(c, A2, S1);
                    let in_list = c.g[V0][0] != 0;
                    c.inc_r(A1, 4);
                    if !in_list {
                        break;
                    }
                    c.cycles += 5;
                    c.lw_r(V0, at(c, A3, contact::SURFACE));
                    c.lw_r(A0, r(c, A1));
                    c.lw_r(V1, at(c, V0, 0x78));
                    if c.g[A0][0] == c.g[V1][0] {
                        found = true;
                        break;
                    }
                    c.inc_r(A2, 1);
                    if event_due(c) {
                        deliver_event_here(c, FW_SEEN);
                    }
                }
            }
            if found {
                c.cycles += 3;
                c.g[V0][0] = slt(c, A2, S1);
                known = c.g[V0][0] != 0;
                c.set_r(V0, s.wrapping_add(r(c, T0)));
            }
        }
        if !known {
            c.cycles += 6;
            c.set_r(V0, s.wrapping_add(r(c, T0)));
            c.inc_r(S1, 1);
            c.lw_r(V1, at(c, V0, contact::SURFACE));
            c.lw_r(A0, at(c, V1, 0x78));
            c.sw_r(r(c, T2), A0);
            c.inc_r(T2, 4);
            c.cycles += 1;
            c.set_r(V0, s.wrapping_add(r(c, T0)));
        }
    } else {
        c.cycles += 1;
        c.set_r(V0, s.wrapping_add(r(c, T0)));
    }
    // a2 = a wall material with a non-zero first field; f1 = |depth - f3|.
    c.cycles += 14;
    c.g[A2][0] = 0;
    c.lw_r(V1, at(c, V0, contact::MATERIAL));
    c.lw_r(A0, at(c, V0, contact::SURFACE));
    c.mult(0, V1, V1, T7);
    c.lwc1(0, at(c, V0, contact::DEPTH));
    c.lw_r(A1, at(c, A0, 0x88));
    c.sub_s(0, 0, 3);
    c.abs_s(1, 0);
    c.inc_r(V1, 0x10);
    c.set_r(A1, r(c, A1).wrapping_add(r(c, V1)));
    c.g[V0][0] = c.lhu(at(c, A1, 0x8)) & 1;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.lwc1(0, r(c, A1));
        c.c_eq_s(0, 4);
        if !c.fcc() {
            c.g[A2][0] = 1;
        }
    }
    // Walls beat non-walls (t4: the best is a wall).
    c.cycles += 2;
    if c.g[A2][0] != 0 {
        c.cycles += 2;
        if c.g[T4][0] == 0 {
            c.cycles += 4;
            c.g[S3][0] = c.g[T1][0];
            c.mov_s(2, 1);
            c.g[T4][0] = 1;
            pick_next(c);
            return;
        }
    } else {
        c.cycles += 2;
        if c.g[T4][0] == 0 {
            c.lw_r(V0, at(c, S2, rider::SPHERE_TREE));
            find_wall_facing(c);
            return;
        }
    }
    c.cycles += 2;
    if c.g[A2][0] == 0 {
        c.g[T1][0] = c.g[T5][0];
        return;
    }
    c.cycles += 1;
    c.lw_r(V0, at(c, S2, rider::SPHERE_TREE));
    find_wall_facing(c);
}

/// `t1 = t5` (the next contact), through the shared block.
#[inline]
pub(crate) fn pick_next(c: &mut Cpu) {
    c.cycles += 1;
    c.g[T1][0] = c.g[T5][0];
}

/// The facing test (a1 = dot(pos - tree.center, normal) < 0) and the rest
/// of the scoring of one wall pick candidate (v0 = sphere tree).
pub(crate) fn find_wall_facing(c: &mut Cpu) {
    let s = sp_2(c);
    c.cycles += 21;
    c.set_r(V1, r(c, T0).wrapping_add(s));
    let cur = r(c, V1);
    c.vsub(cur, at(c, V0, 0x10));
    c.sqc2(5, p(s, 0x2010));
    c.g[A1][0] = 1;
    c.qcopy(V0, p(s, 0x2010), p(s, 0x2000));
    c.vdot(p(s, 0x2000), p(cur, contact::NORMAL), A0, 0);
    c.c_lt_s(0, 4);
    if !c.fcc() {
        c.g[A1][0] = 0;
    }
    // Facing beats not facing (t3: the best faces).
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        if c.g[T3][0] == 0 {
            c.cycles += 4;
            c.g[S3][0] = c.g[T1][0];
            c.mov_s(2, 1);
            c.g[T3][0] = 1;
            pick_next(c);
            return;
        }
    } else {
        c.cycles += 2;
        if c.g[T3][0] == 0 {
            find_wall_depth(c);
            return;
        }
    }
    c.cycles += 2;
    if c.g[A1][0] == 0 {
        c.g[T1][0] = c.g[T5][0];
        return;
    }
    find_wall_depth(c);
}

/// Depth closest to the reference wins (f1 against the best f2); ties by
/// surface / instance id.
pub(crate) fn find_wall_depth(c: &mut Cpu) {
    c.cycles += 3;
    c.c_lt_s(1, 2);
    if c.fcc() {
        c.cycles += 3;
        c.g[S3][0] = c.g[T1][0];
        c.mov_s(2, 1);
        pick_next(c);
        return;
    }
    c.cycles += 4;
    c.c_eq_s(1, 2);
    let tie = c.fcc();
    let s = sp_2(c);
    c.set_r(V1, r(c, T0).wrapping_add(s));
    if tie {
        // v1 = current, a0 = best.
        c.cycles += 4;
        c.set_r(V0, r(c, S3) << 7);
        c.lw_r(A1, at(c, V1, contact::SURFACE));
        c.set_r(A0, r(c, V0).wrapping_add(s));
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.lw_r(V0, at(c, A0, contact::SURFACE));
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 1;
            } else {
                c.lw_r(V1, at(c, A1, 0x78));
                c.cycles += 3;
                c.lw_r(V0, at(c, V0, 0x78));
                c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
            }
        } else {
            c.cycles += 3;
            c.lw_r(V0, at(c, A0, contact::SURFACE));
            let best_has_surface = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !best_has_surface {
                c.cycles += 5;
                c.lw_r(V1, at(c, V1, contact::INSTANCE));
                c.lw_r(A0, at(c, A0, contact::INSTANCE));
                c.lw_r(A1, at(c, V1, 0x150));
                c.lw_r(V0, at(c, A0, 0x150));
                c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            }
        }
        c.cycles += 1;
        if c.g[V0][0] != 0 {
            c.g[S3][0] = c.g[T1][0];
        }
    }
    pick_next(c);
}

pub(crate) const WC_SAVE_CONTACT: u32 = 0x0010_56e0;

/// The global (`0x4c5830`) camera trigger manager.
pub(crate) const CAMERA_TRIGGER_MAN: u32 = 0x004c_5830;

/// `void Rider_HandleWallCollision(Rider* r, const Vec4* ground_normal)`
/// (0x105398): unless the rider is in sub-state 9, handles rail contacts
/// along walls, builds the rider's collision hull and finds the wall
/// contact. With a contact (depth >= 0): the push-out normal is flattened
/// onto the ground plane (when on the ground) or taken from the board when
/// grinding, the push-out depth is measured along the velocity, the
/// contact is saved in the rider (for walls that keep it), the rider
/// bounces off a bouncy wall and a non-silent wall plays the hit sound.
/// Always updates the camera triggers for the rider and destroys the hull.
pub fn rider_handle_wall_collision(c: &mut Cpu) {
    const SAVED: &[(usize, u32)] = &[
        (S2, 0x460),
        (S1, 0x470),
        (S0, 0x480),
        (S3, 0x450),
        (S4, 0x440),
    ];
    guest(c, |c| {
        c.cycles += 15;
        prologue(c, 0x4a0, SAVED, 0x430, &[(21, 0x498), (20, 0x490)]);
        c.g[S2][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        callee::GET_SUB_STATE.call(c, 0x0010_53c8)?;
        c.g[V1][0] = 9;
        let skip = c.g[V0][0] == c.g[V1][0];
        let s = sp_2(c);
        c.lq(S0, p(s, 0x480));
        if !skip {
            wall_collision_body(c)?;
            // CameraTriggerMan_UpdateForRider(&cameraTriggers, r); CollHull_dtor(&hull, 2)
            c.cycles += 6;
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, callee::CAMERA_TRIGGER_UPDATE_FOR_RIDER, 0x0010_5784)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 2;
            call_ptr(c, callee::COLL_HULL_DTOR, 0x0010_5790)?;
            let s = sp_2(c);
            c.lq(S0, p(s, 0x480));
        }
        c.cycles += 9;
        epilogue(
            c,
            &[(S1, 0x470), (S2, 0x460), (S3, 0x450), (S4, 0x440)],
            &[(21, 0x498), (20, 0x490)],
            0x430,
            0x4a0,
        );
        Ok(())
    })
}

/// Rail contacts, the hull, the wall contact and its response; ends with
/// a0 = the camera trigger manager.
pub(crate) fn wall_collision_body(c: &mut Cpu) -> Flow {
    c.cycles += 19;
    c.mtc1(0, 20);
    c.g[A0][0] = c.g[S2][0];
    callee::HANDLE_WALL_RAIL_CONTACT.call(c, 0x0010_53e0)?;
    // CollHull_ctor(&hull, r->sphere_tree, 1)
    c.g[S3][0] = 0x4c_0000;
    let s = sp_2(c);
    c.set_r(S0, p(s, 0xc0));
    c.lw_r(A1, at(c, S2, rider::SPHERE_TREE));
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = 1;
    call_ptr(c, callee::COLL_HULL_CTOR, 0x0010_53f8)?;
    // depth = Rider_FindWallContact(r, &hull, &contact)
    c.g[S4][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[SP][0];
    callee::FIND_WALL_CONTACT.call(c, 0x0010_540c)?;
    c.mov_s(21, 0);
    c.c_le_s(20, 21);
    let hit = c.fcc();
    c.set_r(A0, CAMERA_TRIGGER_MAN);
    if !hit {
        return Ok(());
    }
    wall_push_normal(c)?;
    wall_push_depth(c);
    wall_response(c)
}

/// The push-out normal at `sp_2+0x10` (the contact normal): flattened onto
/// the ground plane on the ground, or the board normal when grinding.
pub(crate) fn wall_push_normal(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S2][0];
    callee::IS_ON_GROUND.call(c, 0x0010_5428)?;
    c.set_r(V1, at(c, S2, rider::UP));
    if c.g[V0][0] != 0 {
        c.g[S1][0] = c.g[V1][0];
    }
    let s = sp_2(c);
    if c.g[S1][0] != 0 {
        // n -= ground * dot(n, ground); renormalised when long enough.
        c.cycles += 36;
        let ground = r(c, S1);
        c.vdot(p(s, 0x10), ground, V0, 0);
        c.lwc1(1, gp_2(c, -0x7f34));
        c.mfc1(V1, 0);
        c.vscale(ground, V1);
        c.vstore(5, p(s, 0x400), V0, &[p(s, 0x3f0)]);
        c.vsub(p(s, 0x10), p(s, 0x3f0));
        c.vstore(5, p(s, 0x400), V0, &[p(s, 0x3e0)]);
        c.vlen(p(s, 0x3e0), V1, 0);
        c.c_lt_s(0, 1);
        if c.fcc() {
            // Too short: the ground normal itself.
            c.cycles += 3;
            c.qcopy(V0, ground, p(s, 0x10));
        } else {
            c.cycles += 11;
            c.mfc1(V1, 0);
            c.vdiv(p(s, 0x3e0), V1);
            c.vstore(4, p(s, 0x400), V0, &[p(s, 0x10), p(s, 0x3f0)]);
        }
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S2][0];
    callee::GET_MAIN_STATE.call(c, 0x0010_5508)?;
    c.g[V1][0] = 4;
    if c.g[V0][0] != c.g[V1][0] {
        return Ok(());
    }
    // Grinding: the board normal, facing the contact normal.
    c.cycles += 13;
    let s = sp_2(c);
    c.lw_r(V1, at(c, S2, 0x77c));
    let board = at(c, V1, 0xb0);
    c.vdot(p(s, 0x10), board, V0, 0);
    c.c_lt_s(20, 0);
    if c.fcc() {
        c.cycles += 3;
        c.qcopy(V0, board, p(s, 0x10));
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 0);
        c.mfc1(V0, 0);
        c.vscale(board, V0);
        c.vstore(5, p(s, 0x3f0), V0, &[p(s, 0x10), p(s, 0x3e0)]);
    }
    Ok(())
}

/// f0 = how far the velocity runs into the wall: `max(dot(-n, contact -
/// vel), 0)`-style compares into f0 (as the original), v0 = `[sp_2]`.
pub(crate) fn wall_push_depth(c: &mut Cpu) {
    c.cycles += 29;
    let s = sp_2(c);
    let rd = r(c, S2);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 1);
    c.mfc1(V0, 1);
    c.vscale(p(s, 0x10), V0);
    c.sqc2(5, p(s, 0x3f0));
    c.mtc1(0, 2);
    c.lq(V0, p(s, 0x3f0));
    c.sq(V0, p(s, 0x3e0));
    c.vsub(p(rd, rider::VEL), p(s, 0x20));
    c.vstore(5, p(s, 0x400), V0, &[p(s, 0x3f0)]);
    c.vdot(p(s, 0x3e0), p(s, 0x3f0), V1, 0);
    c.c_le_s(2, 0);
    let ahead = c.fcc();
    c.mov_s(0, 2);
    if ahead {
        c.cycles += 24;
        c.mfc1(V1, 1);
        c.vscale(p(s, 0x10), V1);
        c.vstore(5, p(s, 0x410), V0, &[p(s, 0x400)]);
        c.vsub(p(rd, rider::VEL), p(s, 0x20));
        c.vstore(5, p(s, 0x420), V0, &[p(s, 0x410)]);
        c.vdot(p(s, 0x400), p(s, 0x410), V1, 0);
    } else {
        c.cycles += 1;
    }
    c.lq(V0, s);
}

/// Save the contact in the rider (walls with material bit 2, unless the
/// surface has flag 0x2000), bounce (material bit 1) and the hit sound
/// (unless flag 0x2000).
pub(crate) fn wall_response(c: &mut Cpu) -> Flow {
    c.cycles += 28;
    let s = sp_2(c);
    let rd = r(c, S2);
    c.g[V1][0] = 0xc;
    c.g[S3][0] = 0x4c_0000;
    c.sq(V0, p(s, 0x80));
    c.vlen2(p(rd, rider::VEL));
    c.vnorm();
    c.sqc2(5, p(s, 0x3f0));
    c.lq(A1, p(s, 0x3f0));
    c.lq(V0, p(s, 0x10));
    c.lw_r(A0, p(s, contact::MATERIAL));
    c.sq(A1, p(s, 0x90));
    c.mult(0, A0, A0, V1);
    c.sq(V0, p(s, 0xa0));
    c.swc1(0, p(s, 0xb0));
    c.sq(A1, p(s, 0x3e0));
    c.lw_r(A2, p(s, contact::SURFACE));
    c.lw_r(V1, at(c, A2, 0x88));
    c.set_r(A0, r(c, A0).wrapping_add(r(c, V1)));
    c.g[V0][0] = c.lhu(at(c, A0, 0x18)) & 2;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.lw_r(V0, at(c, A2, 0x8));
        c.g[V0][0] &= 0x2000;
        if c.g[V0][0] == 0 {
            // rider->wall_contact = contact (0x80), then the four
            // response vectors.
            c.cycles += 3;
            c.set_r(V1, p(rd, 0x9e0));
            c.g[V0][0] = c.g[SP][0];
            c.set_r(A0, p(s, 0x80));
            copy_contact(c, WC_SAVE_CONTACT);
            c.cycles += 9;
            c.lq(V0, p(s, 0x80));
            c.lq(V1, p(s, 0x90));
            c.lq(A0, p(s, 0xa0));
            c.lq(A1, p(s, 0xb0));
            c.sq(V0, p(rd, 0xa60));
            c.sq(V1, p(rd, 0xa70));
            c.sq(A0, p(rd, 0xa80));
            c.sq(A1, p(rd, 0xa90));
        }
    }
    c.lw_r(V1, p(s, contact::MATERIAL));
    // Bouncy wall: Rider_BounceOffWall(r, ?, &response, &contact, depth)
    c.cycles += 9;
    c.g[V0][0] = 0xc;
    c.lw_r(A1, p(s, contact::SURFACE));
    c.mult(0, V1, V1, V0);
    c.lw_r(A0, at(c, A1, 0x88));
    c.set_r(V1, r(c, V1).wrapping_add(r(c, A0)));
    c.g[V0][0] = c.lhu(at(c, V1, 0x18)) & 1;
    c.mov_s(12, 21);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[A0][0] = c.g[S2][0];
        c.set_r(A2, p(s, 0x80));
        c.g[A3][0] = c.g[SP][0];
        callee::BOUNCE_OFF_WALL.call(c, 0x0010_5754)?;
    }
    // SSXAudioSystem_PlayWallHitSound(audio, r, ?, material)
    c.cycles += 5;
    let s = sp_2(c);
    c.lw_r(A2, p(s, contact::SURFACE));
    c.lw_r(V0, at(c, A2, 0x8));
    c.g[V0][0] &= 0x2000;
    let silent = c.g[V0][0] != 0;
    c.set_r(A0, CAMERA_TRIGGER_MAN);
    if !silent {
        c.cycles += 5;
        c.lw_r(A0, gp_2(c, 0x410));
        c.g[A1][0] = c.g[S2][0];
        c.lw_r(A3, p(s, contact::MATERIAL));
        call_ptr(c, callee::PLAY_WALL_HIT_SOUND, 0x0010_5778)?;
        c.set_r(A0, CAMERA_TRIGGER_MAN);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Rails.

/// The board's rail axis from the bone quaternion `q` at `s1` (`+0x10`
/// x, y, z, w): `(2(yw - zx)... )` as the compiler scheduled it, written
/// to `sp_2`, `sp_2+4`, `sp_2+8` (and `sp_2+0xc = 0`).
pub(crate) fn rail_axis_from_quat(c: &mut Cpu) {
    let (s, q) = (sp_2(c), r(c, S1));
    c.lwc1(5, p(q, 0x10));
    c.g[A1][0] = c.g[S1][0];
    c.lwc1(4, p(q, 0x18));
    c.g[A3][0] = 1;
    c.lwc1(0, p(q, 0x14));
    c.mul_s(8, 5, 5);
    c.lwc1(3, p(q, 0x1c));
    c.mul_s(1, 4, 4);
    c.mul_s(2, 0, 4);
    c.lw_r(A0, at(c, S0, rider::OVERLAP_LIST));
    c.mul_s(6, 3, 5);
    c.sw(p(s, 0xc), 0);
    c.add_s(1, 1, 8);
    c.mul_s(0, 0, 5);
    c.mul_s(3, 3, 4);
    c.add_s(2, 2, 6);
    c.add_s(1, 1, 1);
    c.sub_s(0, 0, 3);
    c.add_s(2, 2, 2);
    c.sub_s(7, 7, 1);
    c.add_s(0, 0, 0);
    c.swc1(2, p(s, 0x8));
    c.swc1(7, p(s, 0x4));
    c.swc1(0, s);
}

/// `bool Rider_HandleWallRailContact(Rider* r)` (0x106f78): unless
/// grinding already, looks for a rail along the wall the board touches
/// (`OverlapList_FindNearestRail` with the board axis); when the rider is
/// slow or moving along the rail and close enough to it, and
/// `Rider_FindRail` finds none to jump on, the rider is pushed off the
/// rail, deflected along the wall and the impact handled. Returns whether
/// it was.
pub fn rider_handle_wall_rail_contact(c: &mut Cpu) {
    const SAVED: &[(usize, u32)] = &[(S0, 0x1f0), (S1, 0x1e0), (S2, 0x1d0), (S3, 0x1c0)];
    guest(c, |c| {
        c.cycles += 12;
        prologue(c, 0x210, SAVED, 0x1b0, &[(20, 0x200)]);
        c.g[S0][0] = c.g[A0][0];
        callee::GET_MAIN_STATE.call(c, 0x0010_6f9c)?;
        c.g[V1][0] = 4;
        let grinding = c.g[V0][0] == c.g[V1][0];
        c.set_r(A2, p(sp_2(c), 0x10));
        if grinding {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            wall_rail_contact_body(c)?;
        }
        c.cycles += 8;
        epilogue(c, SAVED, &[(20, 0x200)], 0x1b0, 0x210);
        Ok(())
    })
}

/// Everything after the state check; leaves v0 = the result.
pub(crate) fn wall_rail_contact_body(c: &mut Cpu) -> Flow {
    c.cycles += 38;
    let s = sp_2(c);
    c.lw_r(V1, at(c, S0, 0x780));
    c.lw_r(V0, at(c, S0, 0x89c));
    c.g[S3][0] = c.g[A2][0];
    c.lw_r(A0, at(c, V1, 0x2c));
    c.g[S2][0] = c.g[SP][0];
    c.set_r(V0, r(c, V0) << 5);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    c.mtc1(AT, 7);
    c.set_r(S1, r(c, A0).wrapping_add(r(c, V0)));
    c.g[AT][0] = 0x4396_0000; // 300.0
    c.mtc1(AT, 12);
    rail_axis_from_quat(c);
    // OverlapList_FindNearestRail(r->list, bone, &contact, 1, axis, 300)
    call_ptr(c, callee::OVERLAP_LIST_FIND_NEAREST_RAIL, 0x0010_7038)?;
    let found = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !found {
        return Ok(());
    }
    let rd = r(c, S0);
    // Fast riders only when moving towards the rail.
    c.cycles += 14;
    c.lwc1(0, gp_2(c, -0x7ea4));
    c.vlen(p(rd, rider::VEL), V0, 2);
    c.c_lt_s(0, 2);
    if c.fcc() {
        c.cycles += 21;
        let bone = r(c, S1);
        c.vsub(p(s, 0x10), bone);
        c.sqc2(5, p(s, 0xa0));
        c.lwc1(1, gp_2(c, -0x7ea0));
        c.lq(V0, p(s, 0xa0));
        c.mul_s(1, 2, 1);
        c.sq(V0, p(s, 0x90));
        c.vdot(p(s, 0x90), p(rd, rider::VEL), V1, 0);
        c.c_lt_s(0, 1);
        let away = c.fcc();
        c.g[V0][0] = 0;
        if away {
            return Ok(());
        }
    }
    // Distance to the rail along the axis (clamped to +-50).
    c.cycles += 24;
    let bone = r(c, S1);
    let axis = r(c, S2);
    c.vsub(p(s, 0x10), bone);
    c.sqc2(5, p(s, 0xa0));
    c.g[AT][0] = 0xffff_ffff_c248_0000; // -50.0
    c.mtc1(AT, 0);
    c.lq(V0, p(s, 0xa0));
    c.mov_s(2, 0);
    c.g[AT][0] = 0x4248_0000; // 50.0
    c.mtc1(AT, 3);
    c.sq(V0, p(s, 0x90));
    c.vdot(axis, p(s, 0x90), V1, 1);
    c.c_le_s(0, 1);
    if c.fcc() {
        c.min_s(2, 1, 3);
    }
    c.cycles += 33;
    c.mfc1(V1, 2);
    c.vscale(axis, V1);
    c.vstore(5, p(s, 0xc0), V0, &[p(s, 0xb0)]);
    c.vadd(bone, p(s, 0xb0));
    c.vstore(5, p(s, 0xc0), V0, &[p(s, 0xa0)]);
    c.vsub(p(s, 0xa0), p(s, 0x10));
    c.vstore(5, p(s, 0xc0), V0, &[p(s, 0x90)]);
    c.vlen(p(s, 0x90), V1, 0);
    c.c_lt_s(3, 0);
    let far = c.fcc();
    c.g[V0][0] = 0;
    if far {
        return Ok(());
    }
    // A rail to jump on wins.
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    c.set_r(A1, p(s, 0xd0));
    callee::FIND_RAIL.call(c, 0x0010_71bc)?;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        return Ok(());
    }
    wall_rail_push_off(c)
}

/// Push the rider off the rail: the contact normal facing the velocity,
/// the owner told, the push-out distance and direction, deflection and
/// impact. Returns with v0 = 1.
pub(crate) fn wall_rail_push_off(c: &mut Cpu) -> Flow {
    let s = sp_2(c);
    let rd = r(c, S0);
    // f20 = -|dot(vel, n)|, n flipped to face against the velocity.
    c.cycles += 14;
    c.mtc1(0, 0);
    c.vdot(p(rd, rider::VEL), p(s, 0x20), V0, 20);
    c.c_lt_s(20, 0);
    let towards = c.fcc();
    c.lw_r(A0, p(s, 0x60));
    if towards {
        c.cycles += 9;
        c.neg_s(20, 20);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 0);
        c.mfc1(V0, 0);
        c.vscale(p(s, 0x20), V0);
        c.sqc2(5, p(s, 0x20));
        c.lw_r(A0, p(s, 0x60));
    }
    // contact.surface->owner->vt[0x150](owner, &contact)
    c.cycles += 2;
    c.lq(V0, p(s, 0x10));
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.lw_r(A2, at(c, A0, 0xc));
        c.g[A1][0] = c.g[S3][0];
        if c.g[A2][0] != 0 {
            c.cycles += 6;
            c.lw_r(V0, at(c, A2, 0xc));
            vcall(c, A2, V0, 0x150, V1, 0x0010_7250)?;
            c.lq(V0, p(s, 0x10));
        }
    }
    // The impact record (sp_2+0x150..): contact point, velocity direction,
    // speed into the wall; the push-out vector at sp_2+0xa0.
    c.cycles += 46;
    c.g[A0][0] = c.g[S0][0];
    c.sq(V0, p(s, 0x150));
    c.vlen2(p(rd, rider::VEL));
    c.vnorm();
    c.sqc2(5, p(s, 0xb0));
    c.lq(V0, p(s, 0xb0));
    c.swc1(20, p(s, 0x180));
    c.sq(V0, p(s, 0xa0));
    c.sq(V0, p(s, 0x160));
    let bone = r(c, S1);
    c.vsub(bone, p(s, 0x10));
    c.vstore(5, p(s, 0xb0), V0, &[p(s, 0xa0)]);
    c.vdot(p(s, 0xa0), p(s, 0x20), V1, 0);
    c.mfc1(A1, 0);
    c.vscale(p(s, 0x20), A1);
    c.vstore(5, p(s, 0xc0), V0, &[p(s, 0xb0)]);
    c.vsub(p(s, 0xa0), p(s, 0xb0));
    c.sqc2(5, p(s, 0xa0));
    callee::IS_ON_GROUND.call(c, 0x0010_7304)?;
    if c.g[V0][0] != 0 {
        // Flattened onto the ground plane.
        c.cycles += 20;
        c.vdot(p(s, 0xa0), p(rd, rider::UP), V0, 0);
        c.mfc1(V1, 0);
        c.vscale(p(rd, rider::UP), V1);
        c.vstore(5, p(s, 0xc0), V0, &[p(s, 0xb0)]);
        c.vsub(p(s, 0xa0), p(s, 0xb0));
        c.sqc2(5, p(s, 0xa0));
    }
    // Normalised (or the contact normal when too short).
    c.cycles += 15;
    c.lwc1(0, gp_2(c, -0x7e9c));
    c.vlen(p(s, 0xa0), V0, 1);
    c.c_lt_s(1, 0);
    let short = c.fcc();
    c.lq(V0, p(s, 0x20));
    if short {
        c.cycles += 2;
        c.sq(V0, p(s, 0xa0));
    } else {
        c.cycles += 7;
        c.mfc1(V0, 1);
        c.qmtc2(V0, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, p(s, 0xa0));
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, p(s, 0xa0));
    }
    // Translate out when closer than 50 along it.
    c.cycles += 17;
    c.lq(V0, p(s, 0xa0));
    c.g[AT][0] = 0x4248_0000; // 50.0
    c.mtc1(AT, 1);
    c.sq(V0, p(s, 0x170));
    c.vdot(p(s, 0x90), p(s, 0xa0), V1, 0);
    c.c_lt_s(0, 1);
    let inside = c.fcc();
    c.g[A0][0] = c.g[S0][0];
    if inside {
        c.cycles += 10;
        c.sub_s(0, 1, 0);
        c.mfc1(V0, 0);
        c.vscale(p(s, 0xa0), V0);
        c.sqc2(5, p(s, 0xc0));
        c.set_r(A1, p(s, 0xb0));
        c.lq(V0, p(s, 0xc0));
        c.sq(V0, p(s, 0xb0));
        callee::TRANSLATE.call(c, 0x0010_7428)?;
    }
    // The velocity's component into the wall removed (plus a margin), the
    // per-frame push-out for the owner's virtual 0x158.
    c.cycles += 51;
    c.lq(V0, p(rd, rider::VEL));
    c.mtc1(0, 1);
    c.sq(V0, p(s, 0xb0));
    c.vsub(p(s, 0x30), p(rd, rider::VEL));
    c.sqc2(5, p(s, 0x190));
    c.lwc1(2, gp_2(c, -0x7e98));
    c.lq(V0, p(s, 0x190));
    c.sq(V0, p(s, 0xc0));
    c.vdot(p(s, 0xa0), p(s, 0xc0), V1, 0);
    c.max_s(0, 0, 1);
    c.add_s(0, 0, 2);
    c.mfc1(V1, 0);
    c.vscale(p(s, 0xa0), V1);
    c.vstore(5, p(s, 0xc0), V0, &[p(s, 0x190)]);
    c.vadd(p(rd, rider::VEL), p(s, 0x190));
    c.sqc2(5, p(rd, rider::VEL));
    c.vsub(p(s, 0xb0), p(rd, rider::VEL));
    c.sqc2(5, p(s, 0x1a0));
    c.lq(V0, p(s, 0x1a0));
    c.lw_r(V1, gp_2(c, -0x848));
    c.sq(V0, p(s, 0xc0));
    c.lwc1(0, at(c, V1, 0x10));
    c.cvt_s_w(0, 0);
    c.mfc1(A0, 0);
    c.vscale(p(s, 0xc0), A0);
    c.vstore(5, p(s, 0x1a0), V0, &[p(s, 0x190)]);
    c.lw_r(A0, p(s, 0x60));
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.lw_r(A3, at(c, A0, 0xc));
        c.g[A1][0] = c.g[S3][0];
        if c.g[A3][0] != 0 {
            c.cycles += 6;
            c.lw_r(V0, at(c, A3, 0xc));
            c.set_r(A2, p(s, 0x190));
            vcall(c, A3, V0, 0x158, V1, 0x0010_7518)?;
        }
    }
    // Not grinding: deflect along the wall and rebuild the matrix.
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    callee::GET_MAIN_STATE.call(c, 0x0010_7520)?;
    let state = c.g[V0][0];
    c.lw_r(A2, p(s, 0x5c));
    if state == 0 {
        c.cycles += 8;
        c.lq(V0, p(s, 0xa0));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        c.sq(V0, p(s, 0x20));
        callee::DEFLECT_ALONG_WALL.call(c, 0x0010_753c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::BUILD_WORLD_MATRIX.call(c, 0x0010_7544)?;
        c.lw_r(A2, p(s, 0x5c));
    }
    // Rider_HandleImpact(r, &impact)
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    c.set_r(A1, p(s, 0x150));
    callee::HANDLE_IMPACT.call(c, 0x0010_7554)?;
    c.g[V0][0] = 1;
    Ok(())
}

/// `.rodata`: the board animation's rotation axis.
pub(crate) const RODATA_ANIM_AXIS: u32 = 0x004f_f130;

/// Two board axes from the bone quaternion at `s0` (+0x10..+0x1c), as the
/// compiler scheduled it: `sp_2+0x90..0x9c` (f22 = 0 from `sp_2+0x9c`) and
/// `sp_2+0xa0..0xac`.
pub(crate) fn railslide_board_axes(c: &mut Cpu) {
    let (s, q) = (sp_2(c), r(c, S0));
    c.lwc1(4, p(q, 0x14));
    c.lwc1(5, p(q, 0x18));
    c.lwc1(0, p(q, 0x10));
    c.mul_s(1, 4, 4);
    c.lwc1(3, p(q, 0x1c));
    c.mul_s(6, 5, 5);
    c.mul_s(2, 0, 5);
    c.sw(p(s, 0x9c), 0);
    c.mul_s(7, 3, 4);
    c.mul_s(3, 3, 5);
    c.lwc1(22, p(s, 0x9c));
    c.add_s(1, 1, 6);
    c.mul_s(0, 0, 4);
    c.sub_s(2, 2, 7);
    c.add_s(1, 1, 1);
    c.add_s(0, 0, 3);
    c.add_s(2, 2, 2);
    c.sub_s(1, 8, 1);
    c.add_s(0, 0, 0);
    c.swc1(2, p(s, 0x98));
    c.swc1(1, p(s, 0x90));
    c.swc1(0, p(s, 0x94));
    c.lwc1(3, p(q, 0x10));
    c.lwc1(4, p(q, 0x14));
    c.lwc1(0, p(q, 0x18));
    c.mul_s(5, 3, 3);
    c.lwc1(2, p(q, 0x1c));
    c.mul_s(7, 4, 4);
    c.mul_s(1, 0, 4);
    c.sw(p(s, 0xac), 0);
    c.mul_s(6, 2, 3);
    c.add_s(5, 5, 7);
    c.mul_s(0, 0, 3);
    c.mul_s(2, 2, 4);
    c.sub_s(1, 1, 6);
    c.add_s(5, 5, 5);
    c.add_s(0, 0, 2);
    c.add_s(1, 1, 1);
    c.sub_s(8, 8, 5);
    c.add_s(0, 0, 0);
    c.swc1(1, p(s, 0xa4));
    c.swc1(8, p(s, 0xa8));
    c.swc1(0, p(s, 0xa0));
}

/// `bool Rider_TryStartRailslide(Rider* r)` (0x106848): when
/// `Rider_FindRail` finds a rail, puts the rider on it: the velocity along
/// the rail (speed clamped, vertical part damped), the owner told, the
/// board's angle to the rail picks the stance (`r+0x328`: 1-4, from the
/// trick state and the angle), tricks landed or the bank boost, main state
/// 4 (grinding), the sub-state and animation for the rail kind, the board
/// turned for a sideways stance, and the look direction set. Returns
/// whether the rider got on.
pub fn rider_try_start_railslide(c: &mut Cpu) {
    const SAVED: &[(usize, u32)] = &[(S3, 0xe0), (S0, 0x110), (S1, 0x100), (S2, 0xf0)];
    const FSAVED: &[(usize, u32)] = &[(23, 0x138), (22, 0x130), (21, 0x128), (20, 0x120)];
    guest(c, |c| {
        c.cycles += 15;
        prologue(c, 0x140, SAVED, 0xd0, FSAVED);
        c.g[A1][0] = c.g[SP][0];
        c.g[S3][0] = c.g[A0][0];
        callee::FIND_RAIL.call(c, 0x0010_687c)?;
        let found = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if found {
            railslide_velocity(c)?;
            railslide_stance(c)?;
            railslide_enter(c)?;
        }
        c.cycles += 11;
        epilogue(c, SAVED, FSAVED, 0xd0, 0x140);
        Ok(())
    })
}

/// The rail direction (`sp_2+0x10`) along the velocity; the velocity along
/// it with the speed clamped (f23), the vertical part damped; the owner of
/// the rail told (virtual 0x158) with the velocity change.
pub(crate) fn railslide_velocity(c: &mut Cpu) -> Flow {
    let (s, rd) = (sp_2(c), r(c, S3));
    c.cycles += 13;
    c.mtc1(0, 0);
    c.vdot(p(rd, rider::VEL), p(s, 0x10), V0, 12);
    c.c_lt_s(12, 0);
    if c.fcc() {
        c.cycles += 8;
        c.neg_s(12, 12);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 0);
        c.mfc1(V0, 0);
        c.vscale(p(s, 0x10), V0);
        c.sqc2(5, p(s, 0x10));
    }
    c.cycles += 5;
    c.lwc1(1, gp_2(c, -0x7ed4));
    c.c_le_s(12, 1);
    let slow = c.fcc();
    c.mov_s(0, 12);
    if slow {
        c.cycles += 1;
        c.mov_s(0, 1);
    }
    c.cycles += 15;
    c.mov_s(12, 0);
    c.vlen(p(rd, rider::VEL), V0, 0);
    c.c_le_s(0, 12);
    let fast = c.fcc();
    c.mov_s(23, 12);
    if fast {
        c.cycles += 1;
        c.mov_s(23, 0);
    }
    c.cycles += 23;
    c.lq(V0, p(rd, rider::VEL));
    c.lwc1(1, gp_2(c, -0x7ed0));
    c.sq(V0, p(s, 0x80));
    c.mfc1(V1, 23);
    c.vscale(p(s, 0x10), V1);
    c.vstore(5, p(s, 0xa0), V0, &[p(rd, rider::VEL), p(s, 0x90)]);
    c.lwc1(0, p(rd, rider::VEL + 8));
    c.mul_s(0, 0, 1);
    c.swc1(0, p(rd, rider::VEL + 8));
    c.vsub(p(rd, rider::VEL), p(s, 0x80));
    c.sqc2(5, p(s, 0xa0));
    c.lq(V0, p(s, 0xa0));
    c.lw_r(A0, p(s, 0x50));
    c.sq(V0, p(s, 0x80));
    c.sq(V0, p(s, 0x90));
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.lw_r(T0, at(c, A0, 0xc));
        c.g[A1][0] = c.g[SP][0];
        if c.g[T0][0] != 0 {
            c.cycles += 15;
            c.lw_r(A3, at(c, T0, 0xc));
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 0);
            c.set_r(A2, p(s, 0x90));
            c.mfc1(V1, 0);
            c.vscale(p(s, 0x80), V1);
            c.vstore(5, p(s, 0xa0), V0, &[p(s, 0x90)]);
            vcall(c, T0, A3, 0x158, V1, 0x0010_69d4)?;
        }
    }
    Ok(())
}

/// The board's angle to the rail (f20) and the stance (`r+0x328`); leaves
/// s2 = the previous stance, v0 as the original.
pub(crate) fn railslide_stance(c: &mut Cpu) -> Flow {
    let (s, rd) = (sp_2(c), r(c, S3));
    c.cycles += 71;
    c.lw_r(V1, p(rd, 0x780));
    c.lw_r(V0, p(rd, 0x8a0));
    c.lw_r(A0, at(c, V1, 0x2c));
    c.set_r(V0, r(c, V0) << 5);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 8);
    c.set_r(S0, r(c, A0).wrapping_add(r(c, V0)));
    railslide_board_axes(c);
    c.vdot(p(s, 0x90), p(s, 0x10), V0, 20);
    c.vdot(p(s, 0xa0), p(s, 0x10), V1, 21);
    // f20 = atan2-style angle of the rail in the board frame.
    c.c_eq_s(20, 22);
    let along_zero = c.fcc();
    let mut angle_done = false;
    if along_zero {
        c.cycles += 4;
        c.c_eq_s(21, 22);
        let side_zero = c.fcc();
        c.mov_s(20, 21);
        if side_zero {
            c.cycles += 1;
            c.lw_r(S2, p(rd, rail::STANCE));
        } else {
            c.cycles += 4;
            c.c_le_s(22, 21);
            c.lwc1(20, gp_2(c, -0x7ecc));
            let positive = c.fcc();
            c.lw_r(S2, p(rd, rail::STANCE));
            if positive {
                c.cycles += 3;
                c.lwc1(20, gp_2(c, -0x7ec8));
            }
        }
        angle_done = true;
    }
    if !angle_done {
        c.cycles += 7;
        c.div_s(12, 21, 20);
        call_ptr(c, callee::MATH_ATAN, 0x0010_6b28)?;
        c.c_lt_s(20, 22);
        let behind = c.fcc();
        c.mov_s(1, 0);
        if behind {
            c.cycles += 3;
            c.c_lt_s(22, 21);
            if c.fcc() {
                c.cycles += 3;
                c.lwc1(0, gp_2(c, -0x7ec4));
                c.add_s(1, 1, 0);
            } else {
                c.cycles += 2;
                c.lwc1(0, gp_2(c, -0x7ec0));
                c.sub_s(1, 1, 0);
            }
        }
        c.cycles += 1;
        c.mov_s(20, 1);
        c.cycles += 1;
        c.lw_r(S2, p(rd, rail::STANCE));
    }
    railslide_pick_stance(c)
}

/// Where the stance pick ends.
pub(crate) enum Stance {
    /// `sw s1 (1)`, then done.
    One,
    /// `sw v0` if FCC, else `v0 = 3` and the plain store.
    IfFcc,
    /// `sw v0`.
    Store,
    /// Already stored.
    Done,
}

pub(crate) fn railslide_pick_stance(c: &mut Cpu) -> Flow {
    let rd = r(c, S3);
    c.cycles += 5;
    c.g[A0][0] = c.g[S3][0];
    callee::GET_SUB_STATE.call(c, 0x0010_6b68)?;
    c.g[S1][0] = 1;
    let end = if c.g[V0][0] == c.g[S1][0] {
        // Sub-state 1: kind 0 gives stance 1, else 2.
        c.cycles += 3;
        c.lw_r(V0, p(rd, rail::KIND));
        let kind0 = c.g[V0][0] == 0;
        c.g[V0][0] = 2;
        if kind0 {
            Stance::One
        } else {
            c.cycles += 2;
            c.sw_r(p(rd, rail::STANCE), V0);
            Stance::Done
        }
    } else {
        c.lw_r(V0, p(rd, rail::SIDE));
        let mut end = None;
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.lwc1(1, gp_2(c, -0x7ebc));
            c.abs_s(0, 20);
            c.c_lt_s(1, 0);
            let wide = c.fcc();
            c.g[V0][0] = 2;
            end = Some(if !wide {
                Stance::One
            } else {
                c.cycles += 2;
                c.sw_r(p(rd, rail::STANCE), V0);
                Stance::Done
            });
        }
        match end {
            Some(e) => e,
            None => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S3][0];
                callee::GET_SUB_STATE.call(c, 0x0010_6bb8)?;
                c.g[V1][0] = 0xc;
                let sub12 = c.g[V0][0] == c.g[V1][0];
                c.g[V0][0] = 3;
                if sub12 {
                    Stance::Store
                } else {
                    railslide_stance_from_angle(c)
                }
            }
        }
    };
    match end {
        Stance::One => {
            c.cycles += 2;
            c.sw_r(p(rd, rail::STANCE), S1);
        }
        Stance::IfFcc => {
            c.cycles += 2;
            if c.fcc() {
                c.sw_r(p(rd, rail::STANCE), V0);
            } else {
                c.cycles += 1;
                c.g[V0][0] = 3;
                c.cycles += 1;
                c.sw_r(p(rd, rail::STANCE), V0);
            }
        }
        Stance::Store => {
            c.cycles += 1;
            c.sw_r(p(rd, rail::STANCE), V0);
        }
        Stance::Done => {}
    }
    Ok(())
}

/// The stance from the board's up axis (`sp_2+0xb0`) and the angle.
pub(crate) fn railslide_stance_from_angle(c: &mut Cpu) -> Stance {
    let (s, q) = (sp_2(c), r(c, S0));
    c.cycles += 26;
    c.lwc1(5, p(q, 0x18));
    c.lwc1(3, p(q, 0x10));
    c.lwc1(1, p(q, 0x14));
    c.mul_s(2, 5, 5);
    c.lwc1(4, p(q, 0x1c));
    c.mul_s(7, 3, 3);
    c.mul_s(0, 1, 5);
    c.sw(p(s, 0xbc), 0);
    c.mul_s(8, 4, 3);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 6);
    c.add_s(2, 2, 7);
    c.mul_s(1, 1, 3);
    c.lwc1(7, p(s, 0xbc));
    c.mul_s(4, 4, 5);
    c.add_s(0, 0, 8);
    c.add_s(2, 2, 2);
    c.sub_s(1, 1, 4);
    c.add_s(0, 0, 0);
    c.sub_s(6, 6, 2);
    c.add_s(1, 1, 1);
    c.c_lt_s(0, 7);
    c.swc1(0, p(s, 0xb8));
    c.swc1(6, p(s, 0xb4));
    let down = c.fcc();
    c.swc1(1, p(s, 0xb0));
    if down {
        c.cycles += 3;
        c.c_lt_s(21, 7);
        c.g[V0][0] = 4;
        return Stance::IfFcc;
    }
    c.cycles += 6;
    c.lwc1(0, gp_2(c, -0x7eb8));
    c.abs_s(1, 20);
    c.c_lt_s(0, 1);
    let wide = c.fcc();
    c.g[V0][0] = 2;
    if wide {
        return Stance::Store;
    }
    c.cycles += 5;
    c.lwc1(0, gp_2(c, -0x7eb4));
    c.c_lt_s(1, 0);
    let narrow = c.fcc();
    c.g[V0][0] = 4;
    if narrow {
        return Stance::One;
    }
    c.cycles += 3;
    c.lwc1(0, gp_2(c, -0x7eb0));
    c.c_lt_s(20, 0);
    Stance::IfFcc
}

/// Tricks, main state 4, sub-state, animation, the board turned for a
/// sideways stance, the look direction; v0 = 1.
pub(crate) fn railslide_enter(c: &mut Cpu) -> Flow {
    let (s, rd) = (sp_2(c), r(c, S3));
    // Ground material from the rail contact; land a trick or the bank boost.
    c.cycles += 7;
    c.lw_r(V0, p(s, 0x4c));
    c.g[A0][0] = c.g[S3][0];
    c.sw_r(p(rd, rider::GROUND_MATERIAL), V0);
    callee::GET_MAIN_STATE.call(c, 0x0010_6c98)?;
    c.g[V1][0] = 1;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 9;
        c.lw_r(A2, p(rd, rail::STANCE));
        c.mov_s(12, 23);
        c.lw_r(A3, p(rd, rail::SIDE));
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0;
        c.g[T0][0] = c.g[SP][0];
        callee::BANK_TRICK_BOOST.call(c, 0x0010_6cc0)?;
        c.lw_r(V1, gp_2(c, 0x410));
    } else {
        c.lw_r(A1, p(rd, rail::KIND));
        c.cycles += 13;
        c.g[A2][0] = 0;
        c.lw_r(V0, p(rd, rail::DIR_FLAG));
        c.lw_r(A0, p(rd, 0x790));
        c.g[A1][0] = c.g[A1][0] ^ c.g[V0][0];
        c.lw_r(A3, p(rd, rail::STANCE));
        c.lw_r(T0, p(rd, rail::SIDE));
        c.g[A1][0] = (0 < c.g[A1][0]) as u64;
        callee::LAND_TRICK.call(c, 0x0010_6ce8)?;
        c.mov_s(12, 0);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 1;
        callee::ADD_BOOST.call(c, 0x0010_6cf8)?;
        c.lw_r(V1, gp_2(c, 0x410));
    }
    // audio->rail_material = material; Rider_SetMainState(r, 4)
    c.cycles += 9;
    c.g[A0][0] = c.g[S3][0];
    c.lw_r(V0, p(s, 0x4c));
    c.g[A1][0] = 4;
    c.sw_r(at(c, V1, 0x598c), V0);
    callee::SET_MAIN_STATE.call(c, 0x0010_6d10)?;
    c.g[A0][0] = c.g[S3][0];
    callee::GET_SUB_STATE.call(c, 0x0010_6d18)?;
    let mut fresh = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S3][0];
    let mut sub_4_or_5 = false;
    if !fresh {
        c.cycles += 4;
        c.g[S0][0] = 4;
        callee::GET_SUB_STATE.call(c, 0x0010_6d28)?;
        let is4 = c.g[V0][0] == c.g[S0][0];
        c.g[A0][0] = c.g[S3][0];
        if is4 {
            sub_4_or_5 = true;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S3][0];
            callee::GET_SUB_STATE.call(c, 0x0010_6d38)?;
            c.g[V1][0] = 5;
            let is5 = c.g[V0][0] == c.g[V1][0];
            c.g[A0][0] = c.g[S3][0];
            if is5 {
                sub_4_or_5 = true;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S3][0];
                callee::GET_SUB_STATE.call(c, 0x0010_6d4c)?;
                c.g[V1][0] = 0xb;
                if c.g[V0][0] != c.g[V1][0] {
                    c.lw_r(V0, p(rd, rail::STANCE));
                    railslide_turn_board(c)?;
                    let rd = r(c, S3);
                    c.lw_r(A0, p(rd, 0x77c));
                    return railslide_look(c);
                }
                fresh = true;
            }
        }
    }
    if fresh && !sub_4_or_5 {
        c.cycles += 1;
        c.g[A0][0] = c.g[S3][0];
    }
    // From a trick (sub-state 4/5) or fresh: sub-state 0xd, stance aligned.
    c.cycles += 5;
    c.g[S0][0] = 0;
    callee::GET_SUB_STATE.call(c, 0x0010_6d64)?;
    c.g[V1][0] = 5;
    let mut tricking = false;
    if c.g[V0][0] == c.g[V1][0] {
        c.g[S0][0] = 1;
        tricking = true;
    } else {
        c.cycles += 5;
        c.g[A0][0] = c.g[S3][0];
        callee::GET_SUB_STATE.call(c, 0x0010_6d78)?;
        c.g[V1][0] = 4;
        if c.g[V0][0] != c.g[V1][0] {
            c.g[A0][0] = c.g[S3][0];
        } else {
            c.cycles += 1;
            c.g[S0][0] = 1;
            tricking = true;
        }
    }
    if tricking {
        c.cycles += 1;
        c.g[A0][0] = c.g[S3][0];
    }
    c.cycles += 7;
    c.g[A1][0] = 0xd;
    callee::SET_SUB_STATE.call(c, 0x0010_6d94)?;
    c.g[A0][0] = c.g[S3][0];
    callee::ALIGN_STANCE_TO_RAIL.call(c, 0x0010_6d9c)?;
    let rd = r(c, S3);
    c.lw_r(V1, p(rd, rail::SIDE));
    let centred = c.g[V1][0] == 0;
    c.g[V0][0] = 1;
    if centred {
        // The railslide enter animation (from a trick or not); sub-state 7.
        c.cycles += 9;
        c.lw_r(A0, p(rd, 0x77c));
        c.g[A1][0] = c.g[S0][0];
        c.inc_r(A0, 0x2b0);
        call_ptr(c, callee::RAILSLIDE_PLAY_ENTER_ANIM, 0x0010_6e04)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 7;
        callee::SET_SUB_STATE.call(c, 0x0010_6e10)?;
    } else {
        // Side animation (0x1a / 0x22); sub-state 1.
        c.cycles += 2;
        let left = c.g[V1][0] == c.g[V0][0];
        c.lw_r(A0, p(rd, 0x784));
        if left {
            c.cycles += 7;
            c.g[A1][0] = 0x1a;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 12);
            c.g[A2][0] = 0;
            call_ptr(c, callee::ANIM_PLAY_ANIM, 0x0010_6dc4)?;
        } else {
            c.cycles += 6;
            c.g[A1][0] = 0x22;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 12);
            c.g[A2][0] = 0;
            call_ptr(c, callee::ANIM_PLAY_ANIM, 0x0010_6de0)?;
        }
        c.g[A0][0] = c.g[S3][0];
        c.cycles += 4;
        c.g[A1][0] = 1;
        callee::SET_SUB_STATE.call(c, 0x0010_6dec)?;
    }
    let rd = r(c, S3);
    c.lw_r(A0, p(rd, 0x77c));
    railslide_look(c)
}

/// `RiderRailslideState_ComputeEntryAngle(r->state + 0xb0, &rail_dir)`;
/// v0 = 1.
pub(crate) fn railslide_look(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.set_r(A1, p(sp_2(c), 0x10));
    c.inc_r(A0, 0xb0);
    call_ptr(c, callee::RAILSLIDE_COMPUTE_ENTRY_ANGLE, 0x0010_6f44)?;
    c.g[V0][0] = 1;
    Ok(())
}

/// A stance change to 3 or 4 (from s2, the previous one; v0 = the new
/// one) turns the board by +-90 degrees (gp_2-0x7eac / gp_2-0x7ea8) and its
/// animation; ends with the stance aligned to the rail.
pub(crate) fn railslide_turn_board(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let same = c.g[S2][0] == c.g[V0][0];
    c.g[V0][0] = 3;
    if same {
        c.cycles += 1;
        return Ok(());
    }
    c.cycles += 2;
    let to3 = c.g[S2][0] == c.g[V0][0];
    let s = sp_2(c);
    c.set_r(A1, p(s, 0xb0));
    let turn = if !to3 {
        c.cycles += 2;
        let to4 = c.g[S2][0] == c.g[S0][0];
        c.g[S0][0] = 0x50_0000;
        if !to4 {
            None
        } else {
            c.cycles += 27;
            Some((
                -0x7ea8,
                0x0010_6ec0u32,
                0x0010_6eccu32,
                0x0010_6ef4u32,
                0xc8u32,
            ))
        }
    } else {
        c.cycles += 29;
        Some((-0x7eac, 0x0010_6e48, 0x0010_6e54, 0x0010_6e7c, 0xc0))
    };
    if let Some((angle, ret_rot, ret_yaw, ret_sincos, out)) = turn {
        let rd = r(c, S3);
        c.lq(V0, p(rd, 0x1c0));
        c.lwc1(20, gp_2(c, angle));
        c.g[A0][0] = c.g[S3][0];
        c.sq(V0, p(s, 0xb0));
        c.g[S0][0] = 0x50_0000;
        c.mov_s(12, 20);
        c.set_r(S1, RODATA_UP);
        callee::ROTATE_AROUND_AXIS.call(c, ret_rot)?;
        let rd = r(c, S3);
        c.lw_r(A0, p(rd, 0x784));
        c.mov_s(12, 20);
        call_ptr(c, callee::ANIM_ROTATE_ROOTS, ret_yaw)?;
        c.mtc1(0, 12);
        c.g[V0][0] = 0x50_0000;
        c.set_r(V0, RODATA_ANIM_AXIS);
        let rd = r(c, S3);
        c.lw_r(S2, p(rd, 0x784));
        c.lq(V1, RODATA_ANIM_AXIS);
        c.neg_s(12, 12);
        c.set_r(A0, p(s, out));
        c.set_r(A1, p(s, out + 4));
        let anim = r(c, S2);
        c.sq(V1, p(anim, 0x30));
        call_ptr(c, callee::MATH_SIN_COS, ret_sincos)?;
        // The rotation quaternion (axis * sin, cos) into the animation.
        let (s1, s0) = (r(c, S1), r(c, S0));
        c.lwc1(1, p(s1, 0x8));
        c.lwc1(0, p(s, out));
        c.lwc1(2, s0.wrapping_sub(0xea0));
        c.lwc1(4, p(s1, 0x4));
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.lwc1(3, p(s, out + 4));
        c.cycles += 7;
        c.mul_s(0, 0, 4);
        c.swc1(3, p(s, 0xbc));
        c.swc1(1, p(s, 0xb8));
        c.swc1(2, p(s, 0xb0));
        c.swc1(0, p(s, 0xb4));
        c.lq(V0, p(s, 0xb0));
        let anim = r(c, S2);
        c.sq(V0, p(anim, 0x40));
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S3][0];
    callee::ALIGN_STANCE_TO_RAIL.call(c, 0x0010_6f34)?;
    c.cycles += 1;
    Ok(())
}

// ---------------------------------------------------------------------------
// Rider against rider.

pub(crate) const RC_LOOP: u32 = 0x0010_7948;

/// `FLAGS64_TEST(layer(anim, 1) + 0xb0, which)` for `anim = [base +
/// 0x784]` (two calls), the shape the original uses to test the rider's
/// animation layer.
pub(crate) fn anim_layer_is(
    c: &mut Cpu,
    rider_reg: usize,
    which: u64,
    ret_layer: u32,
    ret_is: u32,
) -> Flow {
    c.lw_r(A0, at(c, rider_reg, 0x784));
    c.g[A1][0] = 1;
    call_ptr(c, callee::ANIM_LAYER_SEQUENCE, ret_layer)?;
    c.set_r(A0, p(r(c, V0), 0xb0));
    c.g[A1][0] = which;
    call_ptr(c, callee::FLAGS64_TEST, ret_is)
}

/// `void Rider_CheckRiderCollisions(Rider* r)` (0x107888): for each of the
/// six other rider slots (`r + 0x24 * i` non-zero, the AI rider list at
/// `world+0x84`), unless that rider is out (`+0x878`): when the pair's
/// bump timer allows, their sphere trees are intersected and the riders
/// pushed apart (bump strengths, flattened on the ground) with the
/// relative velocity resolved; then, unless this rider is protected (`sp_2
/// +0x60`: its animation), when the other rider's knock-down timer allows,
/// is not tumbling and is close and in front, its attribute factor read and the
/// other rider knocked down along the push direction.
pub fn rider_check_rider_collisions(c: &mut Cpu) {
    const SAVED: &[(usize, u32)] = &[
        (S1, 0xf0),
        (S0, 0x100),
        (S2, 0xe0),
        (S3, 0xd0),
        (S4, 0xc0),
        (S5, 0xb0),
        (S6, 0xa0),
        (S7, 0x90),
        (FP, 0x80),
    ];
    const FSAVED: &[(usize, u32)] = &[
        (22, 0x120),
        (24, 0x130),
        (23, 0x128),
        (21, 0x118),
        (20, 0x110),
    ];
    guest(c, |c| {
        c.cycles += 26;
        prologue(c, 0x140, SAVED, 0x70, FSAVED);
        c.g[S1][0] = c.g[A0][0];
        let s = sp_2(c);
        c.sw(p(s, 0x60), 0);
        callee::GET_BUMP_STRENGTH.call(c, 0x0010_78d4)?;
        // protected = layer-1 anim entry != 0xd || sequence flag 1 set || flag 0 clear
        c.lw_r(A0, at(c, S1, 0x784));
        c.mov_s(22, 0);
        c.g[A1][0] = 1;
        call_ptr(c, callee::ANIM_LAYER_ANIM_ENTRY, 0x0010_78e4)?;
        c.g[V1][0] = 0xd;
        let mut store = true;
        let other = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 1;
        if !other {
            c.cycles += 8;
            anim_layer_is(c, S1, 1, 0x0010_78fc, 0x0010_7908)?;
            let set = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !set {
                c.cycles += 8;
                anim_layer_is(c, S1, 0, 0x0010_791c, 0x0010_7928)?;
                let set = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                store = !set;
            }
        }
        if store {
            c.cycles += 1;
            let s = sp_2(c);
            c.sw_r(p(s, 0x60), V0);
        }
        c.cycles += 5;
        c.lwc1(24, gp_2(c, -0x7e80));
        c.g[S7][0] = 0;
        c.lwc1(23, gp_2(c, -0x7e7c));
        c.g[FP][0] = 0x24;
        c.g[S6][0] = c.g[S1][0];
        loop {
            rider_pair(c)?;
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S7][0] as i64) < 6) as u64;
            let more = c.g[V0][0] != 0;
            c.inc_r(S6, 0x24);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, RC_LOOP);
            }
        }
        c.cycles += 17;
        epilogue(
            c,
            &[
                (S0, 0x100),
                (S1, 0xf0),
                (S2, 0xe0),
                (S3, 0xd0),
                (S4, 0xc0),
                (S5, 0xb0),
                (S6, 0xa0),
                (S7, 0x90),
                (FP, 0x80),
            ],
            &[
                (24, 0x130),
                (23, 0x128),
                (22, 0x120),
                (21, 0x118),
                (20, 0x110),
            ],
            0x70,
            0x140,
        );
        Ok(())
    })
}

/// One rider slot (s7, s6); ends with s7 advanced.
pub(crate) fn rider_pair(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.lw_r(V0, r(c, S6));
    let empty = c.g[V0][0] == 0;
    c.lw_r(A0, gp_2(c, -0x848));
    if empty {
        c.cycles += 1;
        c.inc_r(S7, 1);
        return Ok(());
    }
    // s0 = the other rider (world->ai_riders->list[s7]->rider)
    c.cycles += 8;
    c.set_r(A1, r(c, S7) << 2);
    c.lw_r(V1, at(c, A0, 0x84));
    c.lw_r(V0, at(c, V1, 0xc));
    c.set_r(V0, r(c, V0).wrapping_add(r(c, A1)));
    c.lw_r(S0, at(c, V0, 0x28));
    c.lw_r(V1, at(c, S0, 0x878));
    if c.g[V1][0] != 0 {
        c.inc_r(S7, 1);
        return Ok(());
    }
    rider_pair_bump(c)?;
    // v0 = protected
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.inc_r(S7, 1);
        return Ok(());
    }
    rider_pair_knock_down(c)
}

/// The pair's bump: timers (`+0x14` per pair entry, `r+0x86c` index * 0x24
/// into the other's table), sphere trees, push apart and resolve. Ends
/// with v0 = `[sp_2+0x60]`.
pub(crate) fn rider_pair_bump(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    c.lw_r(V0, at(c, S1, 0x86c));
    c.mult(0, V1, V0, FP);
    c.set_r(S5, r(c, V1).wrapping_add(r(c, S0)));
    callee::AI_GET_FRAME_COUNTER_2.call(c, 0x0010_7984)?;
    c.lw_r(V1, at(c, S5, 0x14));
    c.g[V1][0] = slt(c, V1, V0);
    let due = c.g[V1][0] != 0;
    let s = sp_2(c);
    c.lw_r(V0, p(s, 0x60));
    if !due {
        return Ok(());
    }
    c.cycles += 14;
    c.lw_r(V0, at(c, S0, 0x86c));
    c.mult(0, V1, V0, FP);
    c.set_r(S2, r(c, V1).wrapping_add(r(c, S1)));
    callee::AI_GET_FRAME_COUNTER_2.call(c, 0x0010_79a4)?;
    c.sw_r(at(c, S2, 0x14), V0);
    callee::AI_GET_FRAME_COUNTER_2.call(c, 0x0010_79ac)?;
    c.sw_r(at(c, S5, 0x14), V0);
    c.g[A2][0] = c.g[SP][0];
    c.set_r(A3, p(s, 0x10));
    c.lw_r(A0, at(c, S1, rider::SPHERE_TREE));
    c.lw_r(A1, at(c, S0, rider::SPHERE_TREE));
    call_ptr(c, callee::SPHERE_TREE_INTERSECT, 0x0010_79c4)?;
    let hit = c.g[V0][0] != 0;
    c.lw_r(V0, p(s, 0x60));
    if !hit {
        return Ok(());
    }
    // Push vectors scaled by the bump factors, flattened on the ground.
    c.cycles += 23;
    c.g[A0][0] = c.g[S0][0];
    callee::GET_BUMP_STRENGTH.call(c, 0x0010_79d4)?;
    c.mfc1(V1, 24);
    c.vscale(s, V1);
    c.sqc2(5, p(s, 0x30));
    c.mov_s(21, 0);
    c.lq(V0, p(s, 0x30));
    c.set_r(S3, p(s, 0x30));
    c.set_r(S4, p(s, 0x40));
    c.g[A0][0] = c.g[S1][0];
    c.sq(V0, p(s, 0x20));
    c.mfc1(V1, 23);
    c.vscale(s, V1);
    c.vstore(5, p(s, 0x40), V0, &[p(s, 0x30)]);
    callee::IS_ON_GROUND.call(c, 0x0010_7a20)?;
    if c.g[V0][0] != 0 {
        c.cycles += 20;
        let up = at(c, S1, rider::UP);
        flatten_on(c, p(s, 0x20), up);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    callee::IS_ON_GROUND.call(c, 0x0010_7a80)?;
    let grounded = c.g[V0][0] != 0;
    c.set_r(A1, p(s, 0x20));
    if grounded {
        c.cycles += 21;
        let up = at(c, S0, rider::UP);
        flatten_on(c, p(s, 0x30), up);
        c.set_r(A1, p(s, 0x20));
    }
    // Rider_Translate both; resolve when the pair's resolve timer allows.
    c.cycles += 12;
    c.g[A0][0] = c.g[S1][0];
    callee::TRANSLATE.call(c, 0x0010_7ae4)?;
    c.g[A1][0] = c.g[S3][0];
    c.g[A0][0] = c.g[S0][0];
    callee::TRANSLATE.call(c, 0x0010_7af0)?;
    callee::AI_GET_FRAME_COUNTER_2.call(c, 0x0010_7af8)?;
    c.lw_r(V1, at(c, S2, 0x10));
    // 3-frame bump cooldown is a duration: 3 ticks at 60, 6 at 120.
    let cool = crate::tick::ticks_from_frames60(c, 3);
    c.inc_r(V0, cool.wrapping_neg());
    c.g[V1][0] = slt(c, V1, V0);
    let due = c.g[V1][0] != 0;
    c.lw_r(V0, p(s, 0x60));
    if !due {
        return Ok(());
    }
    // n = normalize(push); impulse = (v0.n - v1.n) / (k0 + k1)
    c.cycles += 52;
    c.vlen2(s);
    c.vnorm();
    c.sqc2(5, p(s, 0x50));
    c.add_s(1, 22, 21);
    c.lq(V0, p(s, 0x50));
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S4][0];
    c.sq(V0, p(s, 0x40));
    c.g[A3][0] = 0;
    let (v_other, v_self) = (at(c, S0, rider::VEL), at(c, S1, rider::VEL));
    c.vdot(v_other, p(s, 0x40), V0, 20);
    c.vdot(v_self, p(s, 0x40), V1, 0);
    c.sub_s(20, 20, 0);
    c.div_s(20, 20, 1);
    c.mul_s(12, 21, 20);
    callee::RESOLVE_RIDER_COLLISION.call(c, 0x0010_7ba8)?;
    c.neg_s(12, 22);
    c.g[A2][0] = c.g[S4][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = 0;
    c.mul_s(12, 12, 20);
    callee::RESOLVE_RIDER_COLLISION.call(c, 0x0010_7bc4)?;
    callee::AI_GET_FRAME_COUNTER_2.call(c, 0x0010_7bcc)?;
    c.sw_r(at(c, S2, 0x10), V0);
    callee::AI_GET_FRAME_COUNTER_2.call(c, 0x0010_7bd4)?;
    c.sw_r(at(c, S5, 0x10), V0);
    let s = sp_2(c);
    c.lw_r(V0, p(s, 0x60));
    Ok(())
}

/// `[v] -= up * dot([v], up)` through `sp_2+0x40`/`sp_2+0x50`.
pub(crate) fn flatten_on(c: &mut Cpu, v: u32, up: u32) {
    let s = sp_2(c);
    c.vdot(v, up, V0, 0);
    c.mfc1(V1, 0);
    c.vscale(up, V1);
    c.vstore(5, p(s, 0x50), V0, &[p(s, 0x40)]);
    c.vsub(v, p(s, 0x40));
    c.sqc2(5, v);
}

/// Knock the other rider (s0) down when its timer (`s5+0x18`) allows, it
/// is not tumbling, is within range and in front.
pub(crate) fn rider_pair_knock_down(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    callee::AI_GET_FRAME_COUNTER_2.call(c, 0x0010_7bec)?;
    c.lw_r(V1, at(c, S5, 0x18));
    // 3-frame knock-down cooldown is a duration: 3 ticks at 60, 6 at 120.
    let cool = crate::tick::ticks_from_frames60(c, 3);
    c.inc_r(V0, cool.wrapping_neg());
    c.g[V1][0] = slt(c, V1, V0);
    if c.g[V1][0] == 0 {
        c.inc_r(S7, 1);
        return Ok(());
    }
    // Not while tumbling (layer-1 anim entry 3 with flag 0 set and flag 1 clear).
    c.cycles += 6;
    c.lw_r(A0, at(c, S0, 0x784));
    c.g[A1][0] = 1;
    call_ptr(c, callee::ANIM_LAYER_ANIM_ENTRY, 0x0010_7c0c)?;
    c.g[V1][0] = 3;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 8;
        anim_layer_is(c, S0, 0, 0x0010_7c24, 0x0010_7c30)?;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            anim_layer_is(c, S0, 1, 0x0010_7c44, 0x0010_7c50)?;
            if c.g[V0][0] == 0 {
                c.inc_r(S7, 1);
                return Ok(());
            }
        }
    }
    // Close (150 < d is too far, d < gp_2-0x7e78 too close) and in front.
    c.cycles += 22;
    let s = sp_2(c);
    let (p_other, p_self) = (at(c, S0, rider::POS), at(c, S1, rider::POS));
    c.vsub(p_other, p_self);
    c.sqc2(5, p(s, 0x10));
    c.g[AT][0] = 0x4316_0000; // 150.0
    c.mtc1(AT, 0);
    c.qcopy(V0, p(s, 0x10), s);
    c.vlen(s, V1, 1);
    c.c_lt_s(0, 1);
    if c.fcc() {
        c.inc_r(S7, 1);
        return Ok(());
    }
    c.cycles += 5;
    c.lwc1(20, gp_2(c, -0x7e78));
    c.c_lt_s(1, 20);
    if c.fcc() {
        c.inc_r(S7, 1);
        return Ok(());
    }
    c.cycles += 23;
    c.mfc1(V0, 1);
    c.vdiv(s, V0);
    c.sqc2(4, p(s, 0x20));
    c.lq(V0, p(s, 0x20));
    c.mtc1(0, 1);
    c.sq(V0, p(s, 0x10));
    let fwd = at(c, S1, 0x340);
    c.vdot(p(s, 0x10), fwd, V1, 0);
    c.c_lt_s(1, 0);
    if !c.fcc() {
        c.inc_r(S7, 1);
        return Ok(());
    }
    // The rider's attribute factor (behaviour library); the knock-down
    // impulse along the direction (plus the velocity when fast).
    c.cycles += 33;
    call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0010_7d28)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 3;
    call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0010_7d34)?;
    c.lw_r(A1, at(c, S1, 0x86c));
    c.g[A0][0] = c.g[V0][0];
    c.lw_r(A2, at(c, S1, 0xb34));
    call_ptr(c, callee::BE_CHAR_ATTRIB5_FACTOR, 0x0010_7d44)?;
    let rd = r(c, S1);
    c.lwc1(2, gp_2(c, -0x7e74));
    c.vlen(p(rd, rider::VEL), V0, 5);
    c.g[AT][0] = 0x3f00_0000; // 0.5
    c.mtc1(AT, 4);
    c.c_lt_s(20, 5);
    c.mul_s(0, 0, 2);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    c.mtc1(AT, 3);
    c.lwc1(1, p(rd, 0x350));
    c.lwc1(2, gp_2(c, -0x7e70));
    c.add_s(1, 1, 4);
    c.add_s(0, 0, 3);
    c.mul_s(0, 0, 1);
    let fast = c.fcc();
    c.mul_s(12, 0, 2);
    let s = sp_2(c);
    if fast {
        c.cycles += 22;
        c.mfc1(V0, 4);
        c.vscale(p(s, 0x10), V0);
        c.sqc2(5, p(s, 0x40));
        c.div_s(0, 4, 5);
        c.qcopy(V0, p(s, 0x40), p(s, 0x30));
        c.mfc1(V1, 0);
        c.vscale(p(rd, rider::VEL), V1);
        c.vstore(5, p(s, 0x50), V0, &[p(s, 0x40)]);
        c.vadd(p(s, 0x30), p(s, 0x40));
        c.vstore(5, p(s, 0x50), V0, &[p(s, 0x10), p(s, 0x20)]);
    }
    // Rider_ResolveRiderCollision(other, r, &dir, 1)
    c.cycles += 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    c.set_r(A2, p(s, 0x10));
    c.g[A3][0] = 1;
    callee::RESOLVE_RIDER_COLLISION.call(c, 0x0010_7e10)?;
    callee::AI_GET_FRAME_COUNTER_2.call(c, 0x0010_7e18)?;
    c.sw_r(at(c, S5, 0x18), V0);
    c.cycles += 1;
    c.inc_r(S7, 1);
    Ok(())
}

/// A `gp_3`-relative global.
#[inline(always)]
pub(crate) fn gp_3(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `lwc1 f, a`, returning the value.
#[inline(always)]
pub(crate) fn ldf_2(c: &mut Cpu, f: usize, a: u32) -> FpuF32 {
    c.lwc1(f, a);
    FpuF32(c.f[f])
}

/// FPU register `f`.
#[inline(always)]
pub(crate) fn fr_2(c: &Cpu, f: usize) -> FpuF32 {
    FpuF32(c.f[f])
}

/// Set FPU register `f`.
#[inline(always)]
pub(crate) fn setf_2(c: &mut Cpu, f: usize, v: FpuF32) {
    c.f[f] = v.0;
}

/// `c.lt.s a, b` leaving the condition bit.
#[inline(always)]
pub(crate) fn lt(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.lt(b);
    abi::set_fcc(c, r);
    r
}

/// `c.le.s a, b`.
#[inline(always)]
pub(crate) fn le(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.le(b);
    abi::set_fcc(c, r);
    r
}

/// `c.eq.s a, b`.
#[inline(always)]
pub(crate) fn eq(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.eq_s(b);
    abi::set_fcc(c, r);
    r
}

/// `x + y` of two sign-extended words (`addu`).
#[inline(always)]
pub(crate) fn add32(x: u64, y: u64) -> u64 {
    sx((x as u32).wrapping_add(y as u32))
}

/// `sll x, n` (32-bit, sign-extended).
#[inline(always)]
pub(crate) fn shl32(x: u64, n: u32) -> u64 {
    sx((x as u32) << n)
}

/// `sllv x, y` (32-bit, sign-extended).
#[inline(always)]
pub(crate) fn shlv32(x: u64, y: u64) -> u64 {
    sx((x as u32) << ((y as u32) & 31))
}

/// `sra x, n`.
#[inline(always)]
pub(crate) fn sra32(x: u64, n: u32) -> u64 {
    sx((((x as u32) as i32) >> n) as u32)
}

// ---------------------------------------------------------------------------
// Wind.

/// `void Rider_ApplyWindForce(Rider* r)` (0x125970): a human rider whose speed
/// (from the weather/AI speed model, capped at 1388.9) is at least 277.8 is
/// pushed along the weather direction (a 512-entry table at 0x504fb8) by
/// `(speed - v·dir) * dir * 0.015`, through vtable slot 0x20 of the rider base,
/// when that is ahead of the velocity.
pub fn rider_apply_wind_force(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        let rider = c.g[S0][0] as u32;
        let sp = sp(c);
        c.g[V0][0] = c.lw(p(rider, rider::IS_HUMAN));
        if c.g[V0][0] != 0 {
            // Speed in game units, capped at the maximum.
            c.cycles += 9;
            let max_speed = ldf_2(c, 21, gp_3(c, -0x7878));
            c.g[A0][0] = c.lw(p(rider, rider::AI_INDEX));
            call_ptr(c, callee::WORLD_PAINTER_WEATHER28, 0x001259a0)?;
            let per_ms = ldf_2(c, 1, gp_3(c, -0x7874));
            let speed = fr_2(c, 0) * per_ms;
            setf_2(c, 20, speed);
            if lt(c, max_speed, speed) {
                setf_2(c, 20, max_speed);
            }
            c.cycles += 5;
            let min_speed = ldf_2(c, 0, gp_3(c, -0x7870));
            if le(c, min_speed, fr_2(c, 20)) {
                // Wind direction for the weather's angle: (cos, sin, 0, 0).
                c.cycles += 37;
                c.g[A0][0] = c.lw(p(rider, rider::AI_INDEX));
                call_ptr(c, callee::WORLD_PAINTER_WEATHER20, 0x001259d4)?;
                let deg_to_rad = ldf_2(c, 1, gp_3(c, -0x786c));
                c.g[A0][0] = 0x0050_0000;
                let idx_scale = ldf_2(c, 2, gp_3(c, -0x7868));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4fb8));
                let rad = fr_2(c, 0) * deg_to_rad;
                setf_2(c, 0, rad);
                c.sw(p(sp, 8), 0);
                c.sw(p(sp, 0xc), 0);
                let zero = ldf_2(c, 3, p(sp, 8));
                let scaled = rad * idx_scale;
                setf_2(c, 0, scaled);
                c.cvt_w_s(1, 0);
                c.mfc1(V0, 1);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x80));
                c.g[V0][0] = shl32(c.g[V0][0], 2);
                c.g[V1][0] = shl32(c.g[V1][0], 2);
                c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = add32(c.g[V1][0], c.g[A0][0]);
                let dir_y = ldf_2(c, 0, c.g[V0][0] as u32);
                let dir_x = ldf_2(c, 1, c.g[V1][0] as u32);
                c.swc1(0, p(sp, 4));
                c.swc1(1, sp);
                c.lqc2(3, p(rider, rider::VELOCITY));
                c.lqc2(5, sp);
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(V0, 4);
                c.mtc1(V0, 0);
                let along = fr_2(c, 20) - fr_2(c, 0);
                setf_2(c, 0, along);
                if lt(c, zero, along) {
                    // impulse = dir * d * 0.015
                    c.cycles += 23;
                    c.mfc1(V0, 0);
                    c.lqc2(4, sp);
                    c.qmtc2(V0, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp, 0x30));
                    c.g[A2][0] = sx(p(rider, rider::BASE));
                    c.lq(V0, p(sp, 0x30));
                    c.g[A1][0] = sx(p(sp, 0x10));
                    ldf_2(c, 0, gp_3(c, -0x7864));
                    c.sq(V0, p(sp, 0x20));
                    c.mfc1(V1, 0);
                    c.lqc2(4, p(sp, 0x20));
                    c.qmtc2(V1, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp, 0x30));
                    c.lq(V0, p(sp, 0x30));
                    c.g[V1][0] = c.lw(p(rider, rider::BASE));
                    c.sq(V0, p(sp, 0x10));
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x20));
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x24));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = add32(c.g[A2][0], c.g[A0][0]);
                    call_ptr(c, target, 0x00125ab8)?;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Heading and audio.

/// `void Rider_UpdateHeadingAndAudio(Rider* r)` (0x121950): the audio hooks
/// (`SSXAudioSystem_UpdateRiderWorldPaint`), then, while the rider is on the
/// ground, blend the accumulated heading (+0x5b4) 15% of the way toward the
/// ground normal's polar angle (wrapped to (-pi, pi]); finally update the
/// danger state. The angle is `atan2(|n.xy|, n.z)`; with `n.z == 0` it is
/// +-pi/2 (0 for a degenerate normal).
pub fn rider_update_heading_and_audio(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        let rider = c.g[S0][0] as u32;
        let sp = sp(c);
        call_ptr(c, callee::SSX_AUDIO_GET_INSTANCE, 0x00121970)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::SSX_AUDIO_UPDATE_RIDER_WORLD_PAINT, 0x0012197c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_IS_ON_GROUND.call(c, 0x00121984)?;
        if c.g[V0][0] != 0 {
            c.cycles += 13;
            let nx = ldf_2(c, 0, p(rider, rider::GROUND_NORMAL));
            let ny = ldf_2(c, 1, p(rider, rider::GROUND_NORMAL + 4));
            let nz = ldf_2(c, 21, p(rider, rider::GROUND_NORMAL + 8));
            let nx2 = nx * nx;
            setf_2(c, 2, nx2);
            let ny2 = ny * ny;
            setf_2(c, 3, ny2);
            c.swc1(0, sp);
            c.mtc1(0, 22);
            c.swc1(1, p(sp, 4));
            let z_zero = eq(c, nz, fr_2(c, 22));
            let sum = nx2 + ny2;
            setf_2(c, 2, sum);
            let len = sum.sqrt();
            setf_2(c, 20, len);
            // Only the degenerate normal (len == 0) runs the `+=1; f0 = heading`
            // tail; both non-degenerate `n.z == 0` paths break out before it.
            let mut tail = 1u32;
            if z_zero {
                c.cycles += 4;
                if eq(c, len, fr_2(c, 22)) {
                    setf_2(c, 12, len);
                } else {
                    c.cycles += 4;
                    let pos = le(c, fr_2(c, 22), len);
                    let a = gp_3(c, -0x790c);
                    let v = ldf_2(c, 12, a);
                    setf_2(c, 12, v);
                    if pos {
                        c.cycles += 3;
                        let a = gp_3(c, -0x7908);
                        let v = ldf_2(c, 12, a);
                        setf_2(c, 12, v);
                    }
                    tail = 0;
                }
            } else {
                c.cycles += 7;
                c.div_s(12, 20, 21);
                call_ptr(c, callee::MATH_ATAN, 0x001219f8)?;
                let neg = lt(c, fr_2(c, 21), fr_2(c, 22));
                setf_2(c, 12, fr_2(c, 0));
                if neg {
                    c.cycles += 3;
                    if lt(c, fr_2(c, 22), fr_2(c, 20)) {
                        c.cycles += 3;
                        let pi = ldf_2(c, 0, gp_3(c, -0x7904));
                        setf_2(c, 12, fr_2(c, 12) + pi);
                    } else {
                        c.cycles += 2;
                        let pi = ldf_2(c, 0, gp_3(c, -0x7900));
                        setf_2(c, 12, fr_2(c, 12) - pi);
                    }
                }
            }
            c.cycles += tail as u64;
            let head = p(rider, rider::HEADING);
            let v = ldf_2(c, 0, head);
            setf_2(c, 0, v);
            // Wrap the difference to (-pi, pi] and blend 15%.
            c.cycles += 6;
            let pi = ldf_2(c, 1, gp_3(c, -0x78fc));
            let mut delta = fr_2(c, 12) - fr_2(c, 0);
            setf_2(c, 12, delta);
            let over = lt(c, pi, delta);
            setf_2(c, 2, fr_2(c, 0));
            if over {
                c.cycles += 3;
                let two_pi = ldf_2(c, 0, gp_3(c, -0x78f8));
                delta = delta - two_pi;
                setf_2(c, 12, delta);
            } else {
                c.cycles += 4;
                let neg_pi = ldf_2(c, 0, gp_3(c, -0x78f4));
                if lt(c, delta, neg_pi) {
                    c.cycles += 2;
                    let two_pi = ldf_2(c, 0, gp_3(c, -0x78f0));
                    delta = delta + two_pi;
                    setf_2(c, 12, delta);
                }
            }
            c.cycles += 4;
            let k = ldf_2(c, 0, gp_3(c, -0x78ec));
            // `heading += (target - heading) * 0.15` per tick is a smoothing
            // filter: the factor follows the rate (exact at 60).
            let k = smoothing_60(c, k);
            let step = fr_2(c, 12) * k;
            setf_2(c, 0, step);
            let heading = fr_2(c, 2) + step;
            setf_2(c, 0, heading);
            c.swc1(0, p(rider, rider::HEADING));
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_UPDATE_DANGER_STATE.call(c, 0x00121a80)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Visibility.

/// `int Rider_UpdateVisibility(Rider* r)` (0x122278): refreshes the on-screen
/// flag (+0xb18) and returns it. Riders that are not in the visible render
/// state (render flag 0, kind != 7, or the visibility-block word set) are off
/// screen. Otherwise two projection tests of the rider's bounding boxes
/// (slots 0x158/0x15c and 0x2ec of the world painter at the draw context's
/// +0x10d8) set the flag, the second against the rider handler's boxes; with no
/// handler, the frustum test at the painter's +0x2ec can clear it.
pub fn rider_update_visibility(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xa0);
        let sp = sp(c);
        frame.save(c, S4, 0x50);
        frame.save(c, S0, 0x90);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save_ra(c, 0x40);
        let rider = c.g[S4][0] as u32;
        'exit: {
            c.g[V0][0] = c.lw(p(rider, rider::RENDER_FLAG));
            let flag_off = c.g[V0][0] == 0;
            c.g[V0][0] = 0x7;
            if flag_off {
                // b_1222bc fallen through: clear and return 0.
                c.cycles += 1;
                c.sw(p(rider, rider::ON_SCREEN), 0);
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'exit;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(p(rider, rider::RENDER_KIND));
            if c.g[V1][0] != c.g[V0][0] {
                c.sw(p(rider, rider::ON_SCREEN), 0);
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'exit;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(rider, rider::VISIBILITY_BLOCK));
            let unblocked = c.g[V0][0] == 0;
            c.g[A1][0] = c.lw(gp_3(c, -0x854));
            if !unblocked {
                c.cycles += 1;
                c.sw(p(rider, rider::ON_SCREEN), 0);
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'exit;
            }
            // On-screen: projection test of the two bounding boxes.
            c.cycles += 21;
            c.g[A0][0] = c.g[SP][0];
            c.g[S2][0] = sx(p(rider, rider::BBOX0));
            c.g[S3][0] = sx(p(rider, rider::BBOX1));
            c.g[S0][0] = c.lw(p(c.g[A1][0] as u32, 0x10d8));
            c.g[S1][0] = c.lh(p(c.g[S0][0] as u32, 0x2e8));
            c.g[V0][0] = c.lh(p(c.g[S0][0] as u32, 0x158));
            c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 0x15c));
            c.g[S1][0] = add32(c.g[A1][0], c.g[S1][0]);
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = add32(c.g[A1][0], c.g[V0][0]);
            call_ptr(c, target, 0x001222f0)?;
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x2ec));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            let target = c.g[V0][0] as u32;
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, target, 0x00122308)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[V1][0] = c.lw(p(rider, rider::UNK_150));
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            let no_handler = c.g[V1][0] == 0;
            c.sw(p(rider, rider::ON_SCREEN), c.g[V0][0]);
            if !no_handler {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    // The same test against the rider handler's boxes.
                    c.cycles += 22;
                    c.g[A1][0] = c.lw(gp_3(c, -0x854));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S2][0] = c.lw(p(rider, rider::HANDLER));
                    let ctx = c.g[A1][0] as u32;
                    c.g[S1][0] = c.lw(p(ctx, 0x10d8));
                    c.g[S3][0] = add32(c.g[S2][0], 0x90);
                    c.g[S0][0] = c.lh(p(c.g[S1][0] as u32, 0x2e8));
                    c.g[S2][0] = add32(c.g[S2][0], 0x80);
                    c.g[V0][0] = c.lh(p(c.g[S1][0] as u32, 0x158));
                    c.g[S0][0] = add32(c.g[A1][0], c.g[S0][0]);
                    c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x15c));
                    let target = c.g[V1][0] as u32;
                    c.g[A1][0] = add32(c.g[A1][0], c.g[V0][0]);
                    call_ptr(c, target, 0x00122354)?;
                    c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x2ec));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S3][0];
                    let target = c.g[V0][0] as u32;
                    c.g[A3][0] = c.g[SP][0];
                    call_ptr(c, target, 0x0012236c)?;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                    c.sw(p(rider, rider::ON_SCREEN), c.g[V0][0]);
                    c.g[V0][0] = c.lw(p(rider, rider::UNK_150));
                }
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw(p(rider, rider::ON_SCREEN));
                    break 'exit;
                }
            }
            // No handler: the frustum test can clear the flag. It walks the
            // world's query list (race manager +0x8c, entries of 8 bytes,
            // count at +0x10c) skipping unset bits of the +0x110 mask.
            let mut hidden = 0u64;
            'frustum: {
                'list: {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(gp_3(c, -0x848));
                    c.g[T1][0] = sx(p(rider, rider::BSPHERE));
                    c.g[A0][0] = 0;
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x84));
                    c.g[A2][0] = c.lw(p(c.g[V1][0] as u32, 0x8c));
                    c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 0x10c));
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[T0][0] = c.lw(p(c.g[A2][0] as u32, 0x110));
                    if empty {
                        break 'list;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = 0;
                    c.g[T2][0] = 1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x828));
                    c.g[A1][0] = add32(c.g[A2][0], 0x8);
                    c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                    loop {
                        c.cycles += 4;
                        c.g[V0][0] = shlv32(c.g[T2][0], c.g[A0][0]);
                        c.g[V0][0] = c.g[T0][0] & c.g[V0][0];
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 0x10c));
                        } else {
                            c.cycles += 15;
                            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                            c.lqc2(1, p(c.g[V0][0] as u32, 0x50));
                            c.lqc2(2, p(c.g[V0][0] as u32, 0x60));
                            c.lqc2(3, p(c.g[V0][0] as u32, 0x70));
                            c.lqc2(4, p(c.g[V0][0] as u32, 0x80));
                            c.lqc2(5, p(c.g[V0][0] as u32, 0x90));
                            c.lqc2(6, c.g[T1][0] as u32);
                            c.ctc2(7, 27);
                            c.vu0().vnop();
                            c.vu0().vnop();
                            c.vcallmsr();
                            c.cfc2(3, 1);
                            c.g[V1][0] = c.g[V1][0] ^ 0x1;
                            let done = c.g[V1][0] == 0;
                            c.g[V0][0] = 1;
                            if done {
                                hidden = 1;
                                break 'frustum;
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 0x10c));
                        }
                        c.cycles += 4;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A1][0] = add32(c.g[A1][0], 8);
                        if !more {
                            // (Out of the list the way an empty one leaves:
                            // b_122418's cycle.)
                            break 'list;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x001223b8);
                        }
                    }
                }
                c.cycles += 1;
            }
            c.cycles += 2;
            if hidden != 0 {
                c.sw(p(rider, rider::ON_SCREEN), 0);
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(p(rider, rider::ON_SCREEN));
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Boost timers.

/// `void Rider_UpdateBoostTimers(Rider* r)` (0x1200d0): decays the two boost
/// timers (+0x2e8/+0x2ec) toward zero by the frame's `dt`, then drives the
/// boost chain: the rider base's vtable slot 0x48 runs first; a boost amount
/// (+0x2f0) below one frame is raised to one frame while in the air (main state
/// 1 or sub state 12) or else decayed, firing timed event 6 when it reaches 0;
/// phase 10 restarts the boost at 20 frames once the amount runs out; otherwise
/// the effect scale (+0x2f8) is reset (kind 0) or decayed (kinds 1 and 2,
/// unless the rider is in sub state 6).
pub fn rider_update_boost_timers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let sp = sp(c);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let rider = c.g[S0][0] as u32;
        // --- decay(+0x2e8, step)
        let per_frame = ldf_2(c, 1, gp_3(c, -0x7958));
        let delta = ldf_2(c, 0, p(rider, rider::DELTA_TIME));
        let a = ldf_2(c, 2, p(rider, rider::BOOST_A));
        let step = delta * per_frame;
        setf_2(c, 20, step);
        if lt(c, step, a) {
            c.cycles += 2;
            setf_2(c, 3, a - step);
        } else {
            setf_2(c, 0, -step);
            c.cycles += 5;
            setf_2(c, 3, FpuF32::ZERO);
            if lt(c, a, fr_2(c, 0)) {
                setf_2(c, 3, a + step);
            }
        }
        // --- decay(+0x2ec, step), unless the timer is already past one frame
        // and the rider is in main state 1.
        c.cycles += 6;
        let b = ldf_2(c, 1, p(rider, rider::BOOST_B));
        let one60 = ldf_2(c, 0, gp_3(c, -0x7954));
        let over = lt(c, one60, b);
        c.swc1(3, p(rider, rider::BOOST_A));
        let mut vt: u64 = 0;
        'decay_b: {
            if !over {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::RIDER_GET_MAIN_STATE.call(c, 0x00120140)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] == c.g[V1][0] {
                    vt = c.lw(p(rider, rider::BASE));
                    c.g[V1][0] = sx(vt as u32);
                    break 'decay_b;
                }
            }
            c.cycles += 5;
            let x = ldf_2(c, 2, p(rider, rider::BOOST_B));
            if lt(c, step, x) {
                c.cycles += 2;
                setf_2(c, 1, x - step);
            } else {
                c.cycles += 5;
                setf_2(c, 0, -step);
                setf_2(c, 1, FpuF32::ZERO);
                if lt(c, x, fr_2(c, 0)) {
                    setf_2(c, 1, x + step);
                }
            }
            c.cycles += 2;
            c.swc1(1, p(rider, rider::BOOST_B));
            vt = c.lw(p(rider, rider::BASE));
            c.g[V1][0] = sx(vt as u32);
        }
        // --- the rider base's per-frame hook
        c.cycles += 10;
        c.g[A1][0] = sx(p(rider, rider::BASE));
        let vt = vt as u32;
        c.g[A0][0] = c.lh(p(vt, 0x48));
        c.g[V0][0] = c.lw(p(vt, 0x4c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
        call_ptr(c, target, 0x00120198)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_GET_MAIN_STATE.call(c, 0x001201a0)?;
        c.g[V1][0] = 1;
        // 0: the clamp block's give-up / 1: the clamp succeeded
        let mut give_up = false;
        if c.g[V0][0] == c.g[V1][0] {
            ldf_2(c, 1, p(rider, rider::BOOST));
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            callee::RIDER_GET_SUB_STATE.call(c, 0x001201b4)?;
            c.g[V1][0] = 0xc;
            if c.g[V0][0] != c.g[V1][0] {
                ldf_2(c, 2, p(rider, rider::BOOST));
                give_up = true;
            } else {
                c.cycles += 1;
                ldf_2(c, 1, p(rider, rider::BOOST));
            }
        }
        // The one-frame clamp: 0 < t <= 1/60 -> t = 1/60.
        if !give_up {
            c.cycles += 5;
            setf_2(c, 0, FpuF32::ZERO);
            if lt(c, fr_2(c, 0), fr_2(c, 1)) {
                c.cycles += 5;
                let one_frame = ldf_2(c, 0, gp_3(c, -0x7950));
                if le(c, fr_2(c, 1), one_frame) {
                    c.cycles += 2;
                    c.swc1(0, p(rider, rider::BOOST));
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(p(rider, rider::BOOST_PHASE));
                } else {
                    ldf_2(c, 2, p(rider, rider::BOOST));
                    give_up = true;
                }
            } else {
                ldf_2(c, 2, p(rider, rider::BOOST));
                give_up = true;
            }
        }
        if give_up {
            // b_1201f4: decay the amount; at zero fire the timed event.
            c.cycles += 5;
            c.mtc1(0, 1);
            if lt(c, fr_2(c, 1), fr_2(c, 2)) {
                c.cycles += 4;
                if lt(c, step, fr_2(c, 2)) {
                    c.cycles += 2;
                    setf_2(c, 1, fr_2(c, 2) - step);
                } else {
                    c.cycles += 4;
                    setf_2(c, 0, -step);
                    setf_2(c, 1, FpuF32::ZERO);
                    if lt(c, fr_2(c, 2), fr_2(c, 0)) {
                        setf_2(c, 1, fr_2(c, 2) + step);
                    }
                }
                c.cycles += 5;
                c.mtc1(0, 12);
                let zero = eq(c, fr_2(c, 1), fr_2(c, 12));
                c.swc1(1, p(rider, rider::BOOST));
                if zero {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x6;
                    callee::RIDER_SET_TIMED_EVENT.call(c, 0x00120250)?;
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw(p(rider, rider::BOOST_PHASE));
            } else {
                c.g[V1][0] = c.lw(p(rider, rider::BOOST_PHASE));
            }
        }
        // --- b_120368: phase 10 restarts the boost, else the kind switch.
        c.cycles += 3;
        c.g[V0][0] = 0xa;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 8;
            let amount = ldf_2(c, 0, p(rider, rider::BOOST));
            c.mtc1(0, 1);
            setf_2(c, 2, FpuF32::ONE);
            let spent = eq(c, amount, fr_2(c, 1));
            c.swc1(2, p(rider, rider::BOOST_SCALE));
            if spent {
                c.cycles += 6;
                c.g[AT][0] = 0x41a0_0000;
                c.mtc1(1, 0);
                c.g[V0][0] = 5;
                c.sw(p(rider, rider::BOOST_PHASE), c.g[V0][0]);
                c.swc1(0, p(rider, rider::BOOST));
            }
            c.cycles += 1;
            c.lq(16, p(sp, 0x10));
        } else {
            c.g[V1][0] = sx(c.lw(p(rider, rider::BOOST_KIND)) as u32);
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            let kind = c.g[V1][0];
            c.g[V0][0] = ((kind as i64) < 2) as u64;
            if kind == 1 {
                // break 'b_1202dc: kind 1 decays the scale by step * 0.005
                // unless the rider is in sub state 6.
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::RIDER_GET_SUB_STATE.call(c, 0x001202e4)?;
                c.g[V1][0] = 6;
                if c.g[V0][0] == c.g[V1][0] {
                    c.lq(16, p(sp, 0x10));
                } else {
                    c.cycles += 7;
                    let rate = ldf_2(c, 0, gp_3(c, -0x794c));
                    let scale = ldf_2(c, 3, p(rider, rider::BOOST_SCALE));
                    let dec = step * rate;
                    setf_2(c, 2, dec);
                    if lt(c, dec, scale) {
                        setf_2(c, 1, scale - dec);
                    } else {
                        c.cycles += 2;
                        setf_2(c, 0, -dec);
                        c.cycles += 5;
                        setf_2(c, 1, FpuF32::ZERO);
                        if lt(c, scale, fr_2(c, 0)) {
                            setf_2(c, 1, scale + dec);
                        }
                    }
                    c.cycles += 1;
                    c.swc1(1, p(rider, rider::BOOST_SCALE));
                    // `break 'b_120364` lands on the outer `+=1; lq16` block.
                    c.cycles += 1;
                    c.lq(16, p(sp, 0x10));
                }
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    // kind >= 2: `v0 = 2; break 'b_1202bc`
                    c.g[V0][0] = 0x2;
                    c.cycles += 2;
                    if kind == 2 {
                        // break 'b_120314: kind 2 decays by step * 0.024.
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        callee::RIDER_GET_SUB_STATE.call(c, 0x0012031c)?;
                        c.g[V1][0] = 6;
                        if c.g[V0][0] == c.g[V1][0] {
                            c.lq(16, p(sp, 0x10));
                        } else {
                            c.cycles += 7;
                            let rate = ldf_2(c, 0, gp_3(c, -0x7948));
                            let scale = ldf_2(c, 3, p(rider, rider::BOOST_SCALE));
                            let dec = step * rate;
                            setf_2(c, 2, dec);
                            if lt(c, dec, scale) {
                                c.cycles += 2;
                                setf_2(c, 1, scale - dec);
                            } else {
                                setf_2(c, 0, -dec);
                                c.cycles += 5;
                                setf_2(c, 1, FpuF32::ZERO);
                                if lt(c, scale, fr_2(c, 0)) {
                                    setf_2(c, 1, scale + dec);
                                }
                            }
                            c.cycles += 1;
                            c.swc1(1, p(rider, rider::BOOST_SCALE));
                            c.cycles += 1;
                            c.lq(16, p(sp, 0x10));
                        }
                    } else {
                        // Unknown kind: nothing to do.
                        c.cycles += 2;
                        c.lq(16, p(sp, 0x10));
                    }
                } else {
                    // kind 0: `break 'b_1202cc` resets the scale to 1
                    // (`lui at, 0x3f80; mtc1 at, f0` — the value goes to f0).
                    c.cycles += 2;
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.swc1(0, p(rider, rider::BOOST_SCALE));
                    c.cycles += 1;
                    c.lq(16, p(sp, 0x10));
                }
            }
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(sp);
        c.lwc1(20, p(sp, 0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp.wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Race finish.

/// `int Rider_CheckRaceFinish(Rider* r)` (0x125228): returns whether the rider
/// has finished (finish time >= 0). For a human who has not finished: when the
/// BE race state is not 4 and the race timer is running and the game's DNF
/// flag is clear, a rider whose remaining time (the entity's +0x78 scaled by
/// 1/60) is less than the race clock is marked did-not-finish, told
/// `Rider_OnFinishRace` and the arcade announcer line 2 plays. A DNF rider in
/// race state 4 is also finished.
pub fn rider_check_race_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        let sp = sp(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(p(c.g[S0][0] as u32, rider::BASE));
        frame.save_fpr(c, 20, 0x30);
        let rider = c.g[S0][0] as u32;
        c.g[V1][0] = c.lw(p(rider, rider::BASE));
        c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x40));
        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x44));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
        call_ptr(c, target, 0x00125258)?;
        // 1: from b_12539c (v0 = 1), 2: from b_1253a0 (v0 = 1), 3: b_1253ac
        let mut tail = 2u32;
        let mut finish = FpuF32::ZERO;
        'body: {
            if c.g[V0][0] == 0 {
                finish = ldf_2(c, 1, p(rider, rider::FINISH_TIME));
                tail = 1;
                break 'body;
            }
            // done = finish >= 0
            c.cycles += 6;
            let h = ldf_2(c, 0, p(rider, rider::FINISH_TIME));
            c.mtc1(0, 20);
            let already = le(c, fr_2(c, 20), h);
            c.g[V0][0] = 1;
            if !already {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                finish = ldf_2(c, 1, p(rider, rider::FINISH_TIME));
                tail = 1;
                break 'body;
            }
            // Is the BE race state out of the running phase?
            c.cycles += 10;
            call_ptr(c, callee::BELIBRARY_GET_INSTANCE, 0x0012528c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, callee::BELIBRARY_GET_INTERFACE, 0x00125298)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = 4;
            c.g[V0][0] = c.lb(p(c.g[V1][0] as u32, 0x5c10));
            let state4 = c.g[V0][0] == c.g[A0][0];
            c.g[A0][0] = c.lw(gp_3(c, -0x848));
            if state4 {
                c.cycles += 4;
                let h = ldf_2(c, 0, p(rider, rider::FINISH_TIME));
                le(c, fr_2(c, 20), h);
                c.g[V0][0] = 1;
                tail = 3;
                break 'body;
            }
            // The did-not-finish check.
            c.cycles += 4;
            c.g[A1][0] = c.lw(p(c.g[A0][0] as u32, 0xc0));
            c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x88));
            let no_timer = c.g[V0][0] == 0;
            c.g[V1][0] = 0x0053_0000;
            // The bit-set path jumps straight to the shared tail (v1 already
            // loaded there); the others run the tail's prologue.
            let mut tail_charge = 1u32;
            if !no_timer {
                c.cycles += 5;
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x8d0));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 9) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                if c.g[V0][0] == 0 {
                    // Seconds left vs. the race clock.
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(p(c.g[A0][0] as u32, 0x84));
                    c.g[A0][0] = c.lw(p(c.g[A1][0] as u32, 0x78));
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0xc));
                    let negative = (c.g[A0][0] as i64) < 0;
                    c.g[S1][0] = c.lw(p(c.g[V0][0] as u32, 0xc));
                    if !negative {
                        c.cycles += 4;
                        c.mtc1(4, 0);
                        c.cvt_s_w(0, 0);
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
                        c.g[V0][0] = c.g[A0][0] & 0x1;
                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        setf_2(c, 0, fr_2(c, 0) + fr_2(c, 0));
                    }
                    c.cycles += 8;
                    let per_sec = ldf_2(c, 12, gp_3(c, -0x787c));
                    let t = fr_2(c, 0) * per_sec;
                    setf_2(c, 12, t);
                    call_ptr(c, callee::FIXUNSSFSI, 0x00125328)?;
                    c.g[V1][0] = 0x3c;
                    c.mult(0, V0, V0, V1);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let keep_going = c.g[V0][0] == 0;
                    c.g[V0][0] = 1;
                    if !keep_going {
                        c.cycles += 11;
                        c.g[A0][0] = c.g[S0][0];
                        c.sw(p(rider, rider::DID_NOT_FINISH), c.g[V0][0]);
                        callee::RIDER_ON_FINISH_RACE.call(c, 0x00125348)?;
                        call_ptr(c, callee::SSX_AUDIO_GET_INSTANCE, 0x00125350)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = 2;
                        call_ptr(c, callee::SSX_AUDIO_ARCADE_SAY, 0x00125360)?;
                        finish = ldf_2(c, 1, p(rider, rider::FINISH_TIME));
                        tail = 1;
                        break 'body;
                    }
                } else {
                    tail_charge = 0;
                }
            }
            // A DNF rider whose race state is 4 also finishes.
            c.cycles += tail_charge as u64;
            c.g[V1][0] = c.lw(p(rider, rider::DID_NOT_FINISH));
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                // `break 'b_1253a0`: skips the `+=1; v0 = 1` block.
                tail = 2;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.lw(gp_3(c, -0x848));
                c.g[A1][0] = 4;
                c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x84));
                c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x214));
                if c.g[V1][0] == c.g[A1][0] {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    callee::RIDER_ON_FINISH_RACE.call(c, 0x00125398)?;
                }
                // Both exits of the DNF block are `break 'b_12539c`.
                tail = 1;
            }
            finish = ldf_2(c, 1, p(rider, rider::FINISH_TIME));
        }
        if tail == 1 {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        if tail != 3 {
            c.cycles += 3;
            setf_2(c, 0, FpuF32::ZERO);
            le(c, FpuF32::ZERO, finish);
        }
        c.cycles += 2;
        if !c.fcc() {
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        let _ = sp;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_RenderModel (0x122448).

/// `void Rider_RenderModel(Rider* r)` (0x122448): if the rider is on screen,
/// compute its LOD, reset the render state's blend words to `0x14`, draw the
/// rider model (`RiderModel_Render(model, lod, a, b)`) between two
/// identity-matrix pushes, then (re)compute the hidden flag `r+0xb1c` from the
/// world state, the on-screen flag and the LOD.
pub fn rider_render_model(c: &mut Cpu) {
    c.cycles += 10;
    let frame = Frame::open(c, 0x50);
    frame.save(c, S0, 0x30);
    frame.save(c, S1, 0x20);
    let rider = r32(c, A0);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S2, 0x10);
    frame.save_ra(c, 0);
    frame.save_fpr(c, 20, 0x40);
    guest(c, |c| {
        let on_screen = lw(c, p(rider, rider::ON_SCREEN));
        c.g[V0][0] = on_screen;
        let world = lw(c, gp(c, globals::WORLD));
        c.g[V0][0] = world;
        if on_screen != 0 {
            // b = q+0xa0 != 0 or sub-state >= 11; a = q+0xa4 != 0 or
            // sub-state >= 11 (q is only valid when the handler is set).
            let handler = lw(c, p(rider, rider::HANDLER));
            c.g[V0][0] = handler;
            let mut q = (handler as u32).wrapping_add(0xd30);
            c.g[V1][0] = sx(q);
            if handler == 0 {
                q = 0;
                c.g[V1][0] = 0;
            }
            let first = lw(c, p(q, 0xa0));
            c.g[A0][0] = first;
            c.cycles += 6;
            c.g[S2][0] = 0;
            let mut v0 = first;
            if first != 0 {
                c.cycles += 2;
                c.g[S2][0] = 1;
                v0 = lw(c, p(q, 0xa4));
                c.g[V0][0] = v0;
            } else {
                c.cycles += 4;
                let sub = lw(c, p(rider, rider::SUB_STATE));
                c.g[V0][0] = sub;
                c.g[V0][0] = ((sub as i64) < 11) as u64;
                if c.g[V0][0] != 0 {
                    v0 = lw(c, p(q, 0xa4));
                    c.g[V0][0] = v0;
                } else {
                    c.cycles += 2;
                    c.g[S2][0] = 1;
                    v0 = lw(c, p(q, 0xa4));
                    c.g[V0][0] = v0;
                }
            }
            c.cycles += 2;
            let nonzero = v0 != 0;
            c.g[S1][0] = 0;
            if nonzero {
                c.cycles += 1;
                c.g[S1][0] = 1;
            } else {
                c.cycles += 4;
                let sub = lw(c, p(rider, rider::SUB_STATE));
                c.g[V0][0] = sub;
                c.g[V0][0] = ((sub as i64) < 11) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[S1][0] = 1;
                }
            }
            c.cycles += 40;
            c.mtc1(0, 20); // f20 = 0.0
            c.g[RA][0] = sx(0x0012_24c8);
            c.g[A0][0] = c.g[S0][0];
            callee::RIDER_COMPUTE_LOD.call(c, 0x0012_24c8)?;
            // rider->vt[69](rider->matrix, 1)
            let g = lw(c, gp(c, globals::AM_CTX));
            c.g[A3][0] = g;
            c.g[A2][0] = sx(1);
            c.g[A1][0] = sx(p(rider, rider::MATRIX));
            let vt = lw(c, p(g as u32, ctx::VTABLE));
            c.g[V0][0] = vt;
            c.g[A0][0] = lh(c, p(vt as u32, 0x228));
            let f = lw(c, p(vt as u32, 0x22c));
            c.g[V1][0] = f;
            let f = f as u32;
            c.g[RA][0] = sx(0x0012_24e8);
            c.g[A0][0] = sx((g as u32).wrapping_add(r32(c, A0)));
            call_ptr(c, f, 0x0012_24e8)?;
            // rs[1] = (rs[1] & ~0x7c) | 0x14; vt[27](0.0, &rider->pos)
            let g = lw(c, gp(c, globals::AM_CTX));
            c.g[A2][0] = g;
            c.g[V1][0] = sx(!0x7cu32);
            c.mov_s(12, 20);
            c.g[A1][0] = sx(p(rider, rider::POSITION));
            let rs = lw(c, p(g as u32, ctx::TOP));
            c.g[A0][0] = rs;
            c.g[V0][0] = lw(c, p(rs as u32, 4));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x14;
            let v = c.g[V0][0];
            Addr(p(rs as u32, 4)).sw(c, v);
            let vt = lw(c, p(g as u32, ctx::VTABLE));
            c.g[V1][0] = vt;
            c.g[A0][0] = lh(c, p(vt as u32, 0xd8));
            let f = lw(c, p(vt as u32, 0xdc));
            c.g[V0][0] = f;
            let f = f as u32;
            c.g[RA][0] = sx(0x0012_2520);
            c.g[A0][0] = sx((g as u32).wrapping_add(r32(c, A0)));
            call_ptr(c, f, 0x0012_2520)?;
            // RiderModel_Render(model, lod, a, b)
            c.g[A0][0] = lw(c, p(rider, rider::MODEL));
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = lw(c, p(rider, rider::LOD));
            c.g[RA][0] = sx(0x0012_2534);
            c.g[A3][0] = c.g[S2][0];
            call_ptr(c, callee::RIDER_MODEL_RENDER, 0x0012_2534)?;
            // vt[27](0.0, &D_004FF120)
            let g = lw(c, gp(c, globals::AM_CTX));
            c.g[A2][0] = g;
            c.g[A1][0] = sx(0x50_0000);
            c.mov_s(12, 20);
            c.g[A1][0] = sx(0x50_0000u32.wrapping_sub(0xee0));
            let vt = lw(c, p(g as u32, ctx::VTABLE));
            c.g[V0][0] = vt;
            c.g[A0][0] = lh(c, p(vt as u32, 0xd8));
            let f = lw(c, p(vt as u32, 0xdc));
            c.g[V1][0] = f;
            let f = f as u32;
            c.g[RA][0] = sx(0x0012_2558);
            c.g[A0][0] = sx((g as u32).wrapping_add(r32(c, A0)));
            call_ptr(c, f, 0x0012_2558)?;
            let world = lw(c, gp(c, globals::WORLD));
            c.g[V0][0] = world;
        }
        // Hidden flag (`r+0xb1c`): recomputed when the world has no active
        // rider or the flag is already set.
        let world = r32(c, V0);
        let wm = lw(c, p(world, 0x84));
        c.g[V1][0] = wm;
        let race = lw(c, p(wm as u32, 0x84));
        c.g[A0][0] = race;
        let rider_id = lw(c, p(race as u32, 0x14));
        c.g[V0][0] = rider_id;
        c.cycles += 5;
        let mut v0 = rider_id;
        let mut early = false;
        if rider_id == 0 {
            v0 = lw(c, p(rider, rider::ON_SCREEN));
            c.g[V0][0] = v0;
        } else {
            c.cycles += 3;
            let hidden = lw(c, p(rider, rider::HIDDEN));
            c.g[V0][0] = hidden;
            if hidden == 0 {
                early = true;
            } else {
                c.cycles += 1;
                v0 = lw(c, p(rider, rider::ON_SCREEN));
                c.g[V0][0] = v0;
            }
        }
        if !early {
            c.cycles += 3;
            c.g[A0][0] = sx(p(rider, rider::HIDDEN));
            let off_screen = v0 == 0;
            c.g[A1][0] = 0;
            if off_screen {
                c.cycles += 2;
                c.g[A1][0] = 1;
                let v = c.g[A1][0];
                Addr(p(rider, rider::HIDDEN)).sw(c, v);
            } else {
                c.cycles += 4;
                c.g[V1][0] = lw(c, p(rider, rider::LOD));
                c.g[V0][0] = sx(2);
                if c.g[V1][0] != c.g[V0][0] {
                    let v = c.g[A1][0];
                    Addr(p(rider, rider::HIDDEN)).sw(c, v);
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = 1;
                    let v = c.g[A1][0];
                    Addr(p(rider, rider::HIDDEN)).sw(c, v);
                }
            }
            c.cycles += 1;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `li at, bits; mtc1 at, fN`.
#[inline(always)]
pub(crate) fn li_f_2(c: &mut Cpu, n: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.mtc1(1, n);
}

// ---------------------------------------------------------------------------
// Rider mass and leg IK.

/// `float cRider_getMass(cRider* r)` (0x11ff98): the rider's weight from the
/// character database, scaled by the collision-toughness factor
/// (`1.5 * attrib + 1`) and by `(10 * r->massScale + 1)`.
pub fn rider_get_mass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let rider = abi::arg(c, 0);
        c.g[S0][0] = sx(rider);
        frame.save_ra(c, 0);
        // weight = getWeight(getInterface(getBE(), 2), r->aiIndex)
        call_ptr(c, 0x0014_dc80, 0x0011_ffb0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0014_dd58, 0x0011_ffbc)?;
        c.g[A1][0] = Addr(p(rider, 0x86c)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_ef30, 0x0011_ffc8)?;
        c.g[S1][0] = c.g[V0][0];
        // toughness = getCollisionAttrib(getInterface(getBE(), 3), index, r->b34)
        call_ptr(c, 0x0014_dc80, 0x0011_ffd0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x0014_dd58, 0x0011_ffdc)?;
        c.g[A1][0] = Addr(p(rider, 0x86c)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = Addr(p(rider, 0xb34)).lw(c);
        call_ptr(c, 0x0014_8f50, 0x0011_ffec)?;
        // f0 = weight * (toughness * 1.5003352 + 1) * (massScale * 10 + 1)
        c.lwc1(1, gp(c, -0x7964));
        c.mtc1(17, 3);
        c.cvt_s_w(3, 3);
        c.lwc1(2, p(rider, 0x2fc));
        c.mul_s(0, 0, 1);
        li_f_2(c, 4, 0x3f80_0000);
        li_f_2(c, 1, 0x4120_0000);
        frame.restore(c, S0, 0x20);
        c.add_s(0, 0, 4);
        frame.restore(c, S1, 0x10);
        c.mul_s(2, 2, 1);
        frame.restore_ra(c, 0);
        c.mul_s(3, 3, 0);
        c.add_s(2, 2, 4);
        c.mul_s(0, 3, 2);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00120D90(cRider* r)` (0x120d90): approach the leg-IK blend
/// `r->legIKBlend` (+0x318) towards 0 for the spin/air animation ids 20..26
/// and towards 1 otherwise, at 0.05 per frame, never overshooting.
pub fn rider_update_leg_ik_blend(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let rider = abi::arg(c, 0);
        c.g[S0][0] = sx(rider);
        c.g[A0][0] = Addr(p(rider, 0x784)).lw(c);
        call_ptr(c, 0x0031_1ae8, 0x0012_0dac)?;
        let anim = (r32(c, V0).wrapping_sub(0x14)) < 7;
        c.g[V0][0] = anim as u64;
        let in_air_anim = c.g[V0][0] != 0;
        c.lwc1(2, p(rider, 0x318));
        if in_air_anim {
            c.cycles += 5;
            c.lwc1(3, gp(c, -0x7934));
            // The blend's step is 0.05 per 60 Hz tick (also the clamp's step).
            c.f[3] = rate_step_60(c, FpuF32(c.f[3])).0;
            c.c_lt_s(3, 2);
            if c.fcc() {
                // blend = current - step, straight out (no clamp).
                c.sub_s(1, 2, 3);
                leg_ik_blend_exit(c, frame);
                return Ok(());
            }
            c.cycles += 4;
            c.lwc1(0, gp(c, -0x7930));
            // (Towards 0: f1 = 0, then the clamp.)
            c.mtc1(0, 1);
            leg_ik_blend_clamp(c, frame);
            return Ok(());
        }
        // Not an air animation: approach 1.
        c.cycles += 5;
        c.lwc1(0, gp(c, -0x792c));
        // `1 + step`: the band the blend settles inside (the step follows the
        // rate; at 60 the baked 1.05 is used).
        if crate::tick::rate(c) != 60 {
            let step = rate_step_60(c, FpuF32::from_f32(0.05f32));
            c.f[0] = (FpuF32::ONE + step).0;
        }
        c.lwc1(3, gp(c, -0x7928));
        // The blend's step is 0.05 per 60 Hz tick.
        c.f[3] = rate_step_60(c, FpuF32(c.f[3])).0;
        c.c_lt_s(0, 2);
        if !c.fcc() {
            c.cycles += 3;
            c.lwc1(0, gp(c, -0x7924));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            leg_ik_blend_clamp(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        c.sub_s(1, 2, 3);
        leg_ik_blend_exit(c, frame);
        Ok(())
    })
}

/// The clamp of [`rider_update_leg_ik_blend`] (0x120e08): never overshoot.
pub(crate) fn leg_ik_blend_clamp(c: &mut Cpu, frame: Frame) {
    c.cycles += 4;
    c.c_lt_s(2, 0);
    if c.fcc() {
        c.add_s(1, 2, 3);
    }
    leg_ik_blend_exit(c, frame);
}

/// The epilogue of [`rider_update_leg_ik_blend`] (0x120e18): store the blend.
pub(crate) fn leg_ik_blend_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 5;
    c.swc1(1, p(r32(c, S0), 0x318));
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Rider danger state.

/// `void func_00122CF0(cRider* r)` (0x122cf0): refresh `r->dangerState`
/// (+0x5b0) from the danger manager, falling back to "on the ground and
/// spinning fast" when the manager has no danger, and tell the metrics when
/// the state changed.
pub fn rider_update_danger_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        let rider = abi::arg(c, 0);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(rider);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
        c.g[A1][0] = Addr(p(rider, 0x86c)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
        c.g[S1][0] = Addr(p(rider, 0x5b0)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0xa8)).lw(c);
        call_ptr(c, 0x0010_1a10, 0x0012_2d20)?;
        let danger = c.g[V0][0] != 0;
        c.sw(p(rider, 0x5b0), c.g[V0][0]);
        if danger {
            c.cycles += 1;
            c.g[A1][0] = Addr(p(rider, 0x5b0)).lw(c);
            return rider_danger_changed(c, frame);
        }
        c.cycles += 4;
        c.g[A0][0] = sx(rider);
        call_ptr(c, 0x0012_2cd0, 0x0012_2d30)?;
        if c.g[V0][0] == 0 {
            c.g[A1][0] = Addr(p(rider, 0x5b0)).lw(c);
            return rider_danger_changed(c, frame);
        }
        c.cycles += 4;
        c.g[A0][0] = sx(rider);
        call_ptr(c, 0x0012_2c98, 0x0012_2d40)?;
        let not_spinning = c.g[V0][0] == 0;
        c.g[V0][0] = 1;
        if !not_spinning {
            c.cycles += 1;
            c.sw(p(rider, 0x5b0), c.g[V0][0]);
        }
        c.cycles += 1;
        c.g[A1][0] = Addr(p(rider, 0x5b0)).lw(c);
        rider_danger_changed(c, frame)
    })
}

/// The state-change tail of [`rider_update_danger_state`] (0x122d50).
pub(crate) fn rider_danger_changed(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    if c.g[S1][0] != c.g[A1][0] {
        c.cycles += 3;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x790)).lw(c);
        call_ptr(c, 0x0011_a0c0, 0x0012_2d60)?;
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}
// D_004FA370, sRSMEntry[0]
pub(crate) const SPIN_FAST_THRESHOLD: i32 = -0x78b4;

/// `Rider::unk_5B4`: the current spin rate, radians per update.
pub(crate) const RIDER_SPIN_RATE: u32 = 0x5b4;

// ---------------------------------------------------------------------------
// Rider helpers.

/// `int Rider_IsSpinningFast(cRider *rider)` (0x122c98): true when the spin
/// rate is at least +0.925 or below -0.925 rad/update.
pub fn rider_is_spinning_fast(c: &mut Cpu) {
    c.cycles += 6;
    let rider = Addr(abi::arg(c, 0));
    c.lwc1(1, rider.off(RIDER_SPIN_RATE).0);
    c.lwc1(0, gp_4(c, SPIN_FAST_THRESHOLD));
    c.c_le_s(0, 1); // threshold <= rate
    c.g[V0][0] = 0;
    let mut fast = 0u64;
    if c.fcc() {
        c.cycles += 1;
        fast = 1;
    } else {
        c.cycles += 4;
        c.neg_s(0, 0); // -threshold
        c.c_lt_s(1, 0); // rate < -threshold
        if c.fcc() {
            c.cycles += 1;
            fast = 1;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = fast;
    abi::ret(c);
}

/// The global object `D_004A289C` (`gp-0x854`) and the float table read from
/// it (its vtable at_2 +0x10d8: slot 35 returns the view-projection matrix,
/// slot 29 the field of view).
pub(crate) const GLOBAL: i32 = -0x854;

pub(crate) const GLOBAL_VTBL: u32 = 0x10d8;

/// The `0.01` scale factor the LOD distance is multiplied by (`gp-0x78c4`).
pub(crate) const LOD_SCALE: i32 = -0x78c4;

/// `void Rider_ComputeLOD(cRider* self)` (0x121f30): project the rider's
/// position into view space, scale z by tan/`tan(cot)` of the field of view
/// and 0.01, then pick the LOD: 0 below 6 units, 1 below 10, else 2.
///
/// State lives in the original's registers: s0 the matrix, s1 `self`, f20 the
/// view depth.
pub fn rider_compute_lod(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 53;
        let frame = Frame::open(c, 0x70);
        c.g[A1][0] = c.lw(gp(c, GLOBAL));
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save_fpr(c, 20, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[V1][0] = c.lw(at_2(c, A1, GLOBAL_VTBL));
        c.g[A0][0] = c.lh(at_2(c, V1, 0x118));
        c.g[V0][0] = c.lw(at_2(c, V1, 0x11c));
        // Register at_2 the call: a0 = the global plus its slot 35 delta,
        // v0 the slot's function.
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0012_1f60)?;
        // The rider's position through its own vtable slot 5.
        c.g[V1][0] = c.lw(at_2(c, S1, rider::VTBL));
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(rider::VTBL));
        c.g[A0][0] = c.lh(at_2(c, V1, 0x28));
        c.g[V0][0] = c.lw(at_2(c, V1, 0x2c));
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0012_1f7c)?;
        // view = matrix * position, keeping z.
        c.lqc2(8, r32(c, V0));
        c.lqc2(4, r32(c, S0));
        c.lqc2(5, at_2(c, S0, 0x10));
        c.lqc2(6, at_2(c, S0, 0x20));
        c.lqc2(7, at_2(c, S0, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        let sp = sp(c);
        c.sqc2(12, p(sp, 0x10));
        c.lq(2, p(sp, 0x10));
        c.g[A1][0] = c.lw(gp(c, GLOBAL));
        c.sq(2, sp);
        c.g[V1][0] = c.lw(at_2(c, A1, GLOBAL_VTBL));
        c.lwc1(20, p(sp, 0x8));
        c.g[A0][0] = c.lh(at_2(c, V1, 0xe8));
        c.g[V0][0] = c.lw(at_2(c, V1, 0xec));
        // The field of view.
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0012_1fc8)?;
        // (sin, cos) of the field of view into the frame's scratch area.
        c.mov_s(12, 0);
        c.g[A0][0] = sx(sp.wrapping_add(0x20));
        c.g[A1][0] = sx(sp.wrapping_add(0x24));
        callee::MATH_SIN_COS_2.call(c, 0x0012_1fd8)?;
        c.lwc1(0, p(sp, 0x20));
        c.lwc1(1, p(sp, 0x24));
        c.lwc1(3, gp(c, LOD_SCALE));
        c.div_s(0, 0, 1);
        c.g[AT][0] = 0x40c0_0000;
        c.mtc1(1, 2);
        c.mul_s(20, 20, 0);
        c.mul_s(20, 20, 3);
        c.c_lt_s(20, 2);
        let near = c.fcc();
        if near {
            // Below 6 units.
            c.cycles += 2;
            c.sw(p(r32(c, S1), rider::LOD), 0);
        } else {
            c.cycles += 6;
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 0);
            c.c_lt_s(20, 0);
            let middle = c.fcc();
            c.g[V0][0] = 0x2;
            if middle {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            }
            c.cycles += 1;
            c.sw(p(r32(c, S1), rider::LOD), c.g[V0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_SetBoostEnabled(Rider* r, Boost* boost)` (0x125958): stores the
/// boost pointer at +0xb2c; clearing it also zeroes the two boost state words
/// at +0x2f0/+0x2f4.
pub fn rider_set_boost_enabled(c: &mut Cpu) {
    c.cycles += 2;
    let r = abi::arg(c, 0);
    let boost = c.g[A1][0];
    c.sw(r.wrapping_add(0xb2c), boost);
    if boost == 0 {
        c.cycles += 2;
        c.sw(r.wrapping_add(0x2f0), 0);
        c.sw(r.wrapping_add(0x2f4), 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `OverlapList_GatherAll` (0x332db8).
pub(crate) const OVERLAP_LIST_GATHER_ALL: u32 = 0x0033_2db8;

// ---------------------------------------------------------------------------
// Rider_RefreshOverlaps / AI_ResetRidersAndOrder.

/// `void Rider_RefreshOverlaps(Rider* r)` (0x120e50): re-gather the rider's
/// overlap list and then find the lights near it.
pub fn rider_refresh_overlaps(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x400));
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x860)).lw(c);
        call_ptr(c, OVERLAP_LIST_GATHER_ALL, 0x0012_0e6c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_FIND_NEARBY_LIGHTS.call(c, 0x0012_0e74)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int Rider_GetSyncChecksum(const Rider* r)` (0x125ad8): the bit pattern
/// of `sum(position) + sum(rotation)` (the two vectors at `+0x110` and
/// `+0x120`, w included). Leaf: no frame, no calls.
pub fn rider_get_sync_checksum(c: &mut Cpu) {
    c.cycles += 16;
    let r = crate::abi::arg(c, 0);
    let f1 = Addr(p(r, 0x124)).read_fpu(c);
    c.f[1] = f1.0;
    let mut f0 = Addr(p(r, 0x120)).read_fpu(c);
    c.f[0] = f0.0;
    let mut f2 = Addr(p(r, 0x110)).read_fpu(c);
    c.f[2] = f2.0;
    f0 = f0 + f1;
    c.f[0] = f0.0;
    let f3 = Addr(p(r, 0x114)).read_fpu(c);
    c.f[3] = f3.0;
    let f4 = Addr(p(r, 0x128)).read_fpu(c);
    c.f[4] = f4.0;
    f2 = f2 + f3;
    c.f[2] = f2.0;
    let f5 = Addr(p(r, 0x12c)).read_fpu(c);
    c.f[5] = f5.0;
    f0 = f0 + f4;
    c.f[0] = f0.0;
    let f1 = Addr(p(r, 0x118)).read_fpu(c);
    c.f[1] = f1.0;
    f2 = f2 + f1;
    c.f[2] = f2.0;
    f0 = f0 + f5;
    c.f[0] = f0.0;
    let f2 = f2 + f0;
    c.f[2] = f2.0;
    c.mfc1(V0, 2);
    crate::abi::ret(c);
}

/// `void Rider_ForceReset(Rider* r)` (0x1235f8): hard-reset a rider: set the
/// handler's reset latch (`+0x350`), go to sub-state 9 / main state 3 and
/// clear the handler's `+0x290`.
pub fn rider_force_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 9;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x77c));
        c.sw(r32(c, V0).wrapping_add(0x350), c.g[V1][0]);
        callee::RIDER_SET_SUB_STATE.call(c, 0x0012_361c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 3;
        callee::RIDER_SET_MAIN_STATE.call(c, 0x0012_3628)?;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x77c));
        c.sw(r32(c, V0).wrapping_add(0x290), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Riders, camera and the front end.

/// `void Rider_CreateFocusBox(Rider* self)` (0x11bb40): allocate the rider's
/// focus box and register it with the focus manager.
pub fn rider_create_focus_box(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x45_7960; // D_00457960
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0011_bb68)?;
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        Addr(p(this, 0x78c)).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x84).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0xa4).lw(c);
        callee::FOCUS_MAN_ADD_FOCUS_BOX.call(c, 0x0011_bb84)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool Rider_HasRenderSlot(Rider* self)` (0x11c0e0): whether the rider's
/// safe render tick is older than the renderer's current tick (no tick `-1`
/// means never).
pub fn rider_has_render_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(this, 0x888)).lw(c);
        let has_tick = c.g[V0][0] != c.g[V1][0];
        c.g[A1][0] = Addr(gp(c, -0x854)).lw(c);
        let mut slot = 0u64;
        if has_tick {
            c.cycles += 7;
            c.g[V1][0] = Addr(r32(c, A1)).off(0x10d8).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x390).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x394).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0011_c11c)?;
            c.g[V1][0] = Addr(p(this, 0x888)).lw(c);
            slot = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        } else {
            c.cycles += 2;
        }
        c.g[V0][0] = slot;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Player_PulseRumble(Player* player, float amount)` (0x1278d0):
/// `player->rumble = max(amount, player->rumble)` (leave-set, no decay).
pub fn player_pulse_rumble(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.lwc1(0, this.wrapping_add(0xdfc));
    c.max_s(12, 12, 0);
    c.swc1(12, this.wrapping_add(0xdfc));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Riders.

/// `void Rider_GetStartMarkerPos(Rider* rider, Vec3* out, StartMarker* marker)`
/// (0x122c28): copy the marker's position (and a 1.0 w) into `out`.
pub fn rider_get_start_marker_pos(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let out = c.g[A0][0];
        c.g[S0][0] = out;
        c.g[A0][0] = 0x004d_33a0;
        c.g[A2][0] = Addr(at_3(c, A1, 0x86c)).lw(c);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0026_b5e0, 0x0012_2c4c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 2);
        c.lwc1(1, at_3(c, V1, 0x8));
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = Addr(r32_2(c, SP)).ld(c);
        let dst = r32_2(c, S0);
        c.swc1(1, dst);
        c.lwc1(0, at_3(c, V1, 0xc));
        c.swc1(0, dst.wrapping_add(4));
        c.lwc1(1, at_3(c, V1, 0x10));
        c.swc1(2, dst.wrapping_add(0xc));
        c.swc1(1, dst.wrapping_add(8));
        frame.restore(c, S0, 0x10);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void Rider_AddVelocity(Rider* rider, Vec4* v)` (0x1250a8): add `v` to the
/// rider's velocity with VU0, and prime the air predictor while in the air.
pub fn rider_add_velocity(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let rider = c.g[A0][0];
        let v = c.g[A1][0];
        c.g[S0][0] = rider;
        frame.save_ra(c, 0);
        c.lqc2(3, at_3(c, S0, 0x1e0));
        c.lqc2(4, v as u32);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at_3(c, S0, 0x1e0));
        c.g[RA][0] = sx(0x0012_50d4);
        c.g[S1][0] = sx(at_3(c, S0, 0x1e0));
        call_ptr(c, 0x0011_fe98, 0x0012_50d4)?;
        c.g[V1][0] = 0x1;
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 6;
            c.g[A0][0] = Addr(at_3(c, S0, 0x788)).lw(c);
            c.g[A1][0] = sx(at_3(c, S0, 0x110));
            c.f[12] = Addr(gp(c, -0x7880)).read_fpu(c).0;
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0011_35b8, 0x0012_50f4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_EmptyProgressHook(Rider* r)` (0x125ad0): per-frame progress
/// hook, not installed.
pub fn rider_empty_progress_hook(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void Rider_EmptyStanceHook(Rider* r)` (0x116930): stance hook, not
/// installed.
pub fn rider_empty_stance_hook(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `ld` (a 64-bit load).
#[inline(always)]
pub(crate) fn ld(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).ld(c)
}

/// The common epilogue: `s0` back, `ra` back, `sp += 0x20`, return.
pub(crate) fn timed_event_epilogue(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// The tail at 0x115d38: clear the event, remember the frame it ended on and
/// return (the original repeats the epilogue's 3 cycles here).
pub(crate) fn timed_event_tail(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    let last = c.g[V0][0];
    sw_at(c, S0, rider_anim::EVENT, 0);
    sw_at(c, S0, rider_anim::LAST_FRAME, last);
    timed_event_epilogue(c, frame);
}

/// The two variants of the "landing sound" arm (events 1 and 2): play the
/// landing animation on layer 0x13b, reset the blend doubleword, then tell the
/// audio system which landing it was. `rets` are the original return
/// addresses (PlayAnim, GetInstance, RiderSay).
pub(crate) fn timed_event_land_sound(
    c: &mut Cpu,
    frame: Frame,
    say_arg: u64,
    rets: [u32; 3],
) -> Flow {
    c.cycles += 19;
    c.g[V1][0] = lw_2(c, p(r32_3(c, S0), rider_anim::ANIM));
    c.g[A1][0] = 0x13b;
    c.g[V0][0] = ld(c, p(r32_3(c, S0), rider_anim::BLEND_SOURCE));
    c.g[A2][0] = 0;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 12);
    let v = c.g[V0][0];
    Addr(p(r32_3(c, V1), 0x20)).sd(c, v);
    c.g[A0][0] = lw_2(c, p(r32_3(c, S0), rider_anim::ANIM));
    anim_callee::PLAY_ANIM.call(c, rets[0])?;
    c.g[V1][0] = lw_2(c, p(r32_3(c, S0), rider_anim::ANIM));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    Addr(p(r32_3(c, V1), 0x20)).sd(c, 0xffff_ffff_ffff_ffff);
    anim_callee::AUDIO_GET_INSTANCE.call(c, rets[1])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = say_arg;
    anim_callee::AUDIO_RIDER_SAY.call(c, rets[2])?;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    timed_event_tail(c, frame);
    Ok(())
}

/// `void Rider_UpdateTimedEventAnim(Rider* rider)` (0x00115b58): when the rider
/// has a pending timed event and the animation controller is idle (no layer
/// in the middle of a transition), pick the event's landing animation from
/// the rider's event code and start it; the "landing sound" events also tell
/// the audio system which landing played. The event clears after 180 frames,
/// or after the landing animation ran.
pub fn rider_update_timed_event_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, p(r32_3(c, S0), rider_anim::EVENT));
        if c.g[V0][0] == 0 {
            timed_event_epilogue(c, frame);
            return Ok(());
        }
        // Busy checks: layer 2 must be idle (0), layer 1 idle, layer 2 not
        // in the 0xa state.
        c.cycles += 6;
        c.g[A0][0] = lw_2(c, p(r32_3(c, S0), rider_anim::ANIM));
        c.g[A1][0] = 2;
        anim_callee::GET_LAYER_ENTRY.call(c, 0x0011_5b80)?;
        c.g[V1][0] = 5;
        if c.g[V0][0] == c.g[V1][0] {
            timed_event_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = lw_2(c, p(r32_3(c, S0), rider_anim::ANIM));
        c.g[A1][0] = 1;
        anim_callee::GET_LAYER_ENTRY.call(c, 0x0011_5b98)?;
        if c.g[V0][0] != 0 {
            timed_event_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 6;
        c.g[A0][0] = lw_2(c, p(r32_3(c, S0), rider_anim::ANIM));
        c.g[A1][0] = 2;
        anim_callee::GET_LAYER_ENTRY.call(c, 0x0011_5bac)?;
        c.g[V1][0] = 0xa;
        if c.g[V0][0] == c.g[V1][0] {
            timed_event_epilogue(c, frame);
            return Ok(());
        }

        // Recent enough (under 180 frames = 3 s) and not in the future.
        c.cycles += 7;
        anim_callee::FRAME_COUNTER.call(c, 0x0011_5bc0)?;
        c.g[V1][0] = lw_2(c, p(r32_3(c, S0), rider_anim::LAST_FRAME));
        c.g[V1][0] = sx(r32_3(c, V0).wrapping_sub(r32_3(c, V1)));
        // 180 frames is a duration: 180 ticks at 60, 360 at 120.
        c.g[V0][0] = ((c.g[V1][0] as i64) < crate::tick::ticks_from_frames60(c, 180) as i64) as u64;
        let recent = c.g[V0][0] != 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !recent {
            timed_event_tail(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        if (c.g[V1][0] as i64) < 0 {
            timed_event_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 5;
        c.g[V0][0] = lw_2(c, p(r32_3(c, S0), rider_anim::EVENT));
        c.g[A0][0] = sx(r32_3(c, V0).wrapping_sub(1));
        c.g[V1][0] = (c.g[A0][0] < 6) as u64;
        let in_range = c.g[V1][0] != 0;
        c.g[V0][0] = 0x0045_0000;
        if !in_range {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            timed_event_tail(c, frame);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V1][0] = sx(r32_3(c, A0) << 2);
        c.g[V0][0] = sx(r32_3(c, V0).wrapping_add(0x6cc0));
        c.g[V1][0] = sx(r32_3(c, V1).wrapping_add(r32_3(c, V0)));
        c.g[A0][0] = lw_2(c, r32_3(c, V1));
        let target = r32_3(c, A0);
        match target {
            0x0011_5c08 => {
                timed_event_land_sound(c, frame, 0, [0x0011_5c2c, 0x0011_5c3c, 0x0011_5c4c])?;
                return Ok(());
            }
            0x0011_5c54 => {
                timed_event_land_sound(c, frame, 1, [0x0011_5c78, 0x0011_5c88, 0x0011_5c98])?;
                return Ok(());
            }
            0x0011_5ca0 => {
                c.cycles += 3;
                c.g[V1][0] = lw_2(c, p(r32_3(c, S0), rider_anim::ANIM));
                c.g[A1][0] = 0x13b;
            }
            0x0011_5cac => {
                c.cycles += 2;
                c.g[V1][0] = lw_2(c, p(r32_3(c, S0), rider_anim::ANIM));
                c.g[A1][0] = 0x13a;
            }
            0x0011_5ce0 => {
                // The landing-animation arm: only while the blend is below
                // 0.75 and the rider is not busy.
                c.cycles += 8;
                c.lwc1(0, p(r32_3(c, S0), rider_anim::SUB_BLEND));
                c.g[AT][0] = 0x3f40_0000;
                c.mtc1(1, 1);
                c.abs_s(0, 0);
                c.c_lt_s(0, 1);
                let high = !c.fcc();
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                if high {
                    timed_event_tail(c, frame);
                    return Ok(());
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                anim_callee::GET_SUB_STATE.call(c, 0x0011_5d08)?;
                let busy = c.g[V0][0] != 0;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                if busy {
                    timed_event_tail(c, frame);
                    return Ok(());
                }
                c.cycles += 6;
                c.g[A0][0] = lw_2(c, p(r32_3(c, S0), rider_anim::ANIM));
                c.g[A1][0] = 0x13e;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.mtc1(1, 12);
                c.g[A2][0] = 0;
                anim_callee::PLAY_ANIM.call(c, 0x0011_5d28)?;
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                timed_event_tail(c, frame);
                return Ok(());
            }
            _ => {
                c.jump_out(target, 0x0011_5b58, 0x0011_5d44);
                return Ok(());
            }
        }

        // Shared by the 0x13b / 0x13a landing animations: start it, reset the
        // blend, then clear the event.
        c.cycles += 11;
        c.g[V0][0] = ld(c, p(r32_3(c, S0), rider_anim::BLEND_SOURCE));
        c.g[A2][0] = 0;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12);
        let v = c.g[V0][0];
        Addr(p(r32_3(c, V1), 0x20)).sd(c, v);
        c.g[A0][0] = lw_2(c, p(r32_3(c, S0), rider_anim::ANIM));
        anim_callee::PLAY_ANIM.call(c, 0x0011_5cd0)?;
        c.g[V1][0] = lw_2(c, p(r32_3(c, S0), rider_anim::ANIM));
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        Addr(p(r32_3(c, V1), 0x20)).sd(c, 0xffff_ffff_ffff_ffff);
        c.cycles += 1;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        timed_event_tail(c, frame);
        Ok(())
    })
}

/// Where `Rider_UpdateIdleLookAnim` plays its look animation.
const IDLE_LOOK_PLAY: u32 = 0x0031_28e8;

/// `void Rider_UpdateIdleLookAnim(Rider* rider)` (0x00115d48): while layer 1
/// of the animation controller is idle, the rider's idle clock (`+0x35c`,
/// 1/60 per tick) runs; once it reaches 1 s, each update: with a 90% roll,
/// look at the first other rider (slots `rider + 0x24 * i`) within 1000
/// units and more than 60 degrees off the heading that was not looked at in
/// the last 10 s (`slot+0x20`): over the shoulder (0x13f) beyond 135
/// degrees, else left or right (0x141/0x140) by side and stance; otherwise
/// (or when none qualifies) glance back at the first rider within 1000
/// units and more than 120 degrees off (0x13d/0x13c). Busy: the clock
/// restarts.
pub fn rider_update_idle_look_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xb0);
        c.g[A1][0] = 1;
        frame.save(c, S2, 0x50);
        frame.save(c, S0, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0x20);
        for (f, off) in [(24, 0xa0), (23, 0x98), (22, 0x90), (21, 0x88), (20, 0x80)] {
            frame.save_fpr(c, f, off);
        }
        let rider = r32_3(c, S2);
        c.g[A0][0] = c.lw(p(rider, 0x784));
        call_ptr(c, 0x0031_1ae8, 0x0011_5d88)?;
        if c.g[V0][0] != 0 {
            // Layer 1 busy: restart the idle clock.
            c.cycles += 2;
            c.sw(p(rider, 0x35c), 0);
            idle_look_epilogue(c, frame, true);
            return Ok(());
        }
        // The idle clock: += 1/60 (the `.lit4` step, follows the rate); wait
        // for 1 s.
        c.lwc1(0, p(rider, 0x35c));
        c.cycles += 8;
        c.lwc1(1, gp_2(c, -0x7b54));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 2);
        c.add_s(0, 0, 1);
        c.c_lt_s(0, 2);
        let waiting = c.fcc();
        c.swc1(0, p(rider, 0x35c));
        if waiting {
            idle_look_epilogue(c, frame, true);
            return Ok(());
        }

        // f22 = heading of the velocity (atan(y/x) in -pi..pi).
        c.cycles += 9;
        c.lq(V0, p(rider, 0x1b0));
        c.mtc1(0, 22);
        let s = sp(c);
        c.sq(V0, p(s, 0x10));
        c.sq(V0, s);
        c.lwc1(20, p(s, 0x10));
        c.c_eq_s(20, 22);
        let x_nonzero = !c.fcc();
        c.lwc1(21, p(s, 0x4));
        if x_nonzero {
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, 0x0031_c228, 0x0011_5e18)?;
            c.c_lt_s(20, 22);
            let x_pos = !c.fcc();
            c.mov_s(12, 0);
            if !x_pos {
                c.cycles += 3;
                c.c_lt_s(22, 21);
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(0, gp_2(c, -0x7b48));
                    c.add_s(12, 12, 0);
                } else {
                    c.cycles += 2;
                    c.lwc1(0, gp_2(c, -0x7b44));
                    c.sub_s(12, 12, 0);
                }
            }
            c.cycles += 1;
            c.mov_s(22, 12);
        } else {
            c.cycles += 3;
            c.c_eq_s(21, 22);
            if c.fcc() {
                c.cycles += 2;
                c.mov_s(22, 21);
            } else {
                c.cycles += 4;
                c.c_le_s(22, 21);
                c.lwc1(22, gp_2(c, -0x7b50));
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(22, gp_2(c, -0x7b4c));
                }
            }
        }

        // 90%: look at a rider not looked at for 10 s.
        c.cycles += 6;
        call_ptr(c, 0x0031_7810, 0x0011_5e54)?;
        c.g[V1][0] = 0x64;
        c.divu(0, V0, V1);
        c.cycles += 4;
        c.g[A0][0] = c.hi[0];
        c.g[A0][0] = ((c.g[A0][0] as i64) < 90) as u64;
        let look = c.g[A0][0] != 0;
        c.g[V0][0] = 0x24;
        if look {
            c.cycles += 3;
            c.lwc1(23, gp_2(c, -0x7b40));
            c.g[S4][0] = 0;
            c.lwc1(24, gp_2(c, -0x7b3c));
            loop {
                if let Some(a1) = idle_look_slot(c)? {
                    // A rider to look at: play it and clear the blend source.
                    c.g[A1][0] = a1;
                    if a1 == 0x140 {
                        idle_look_play_b(c)?;
                    } else {
                        idle_look_play_a(c)?;
                    }
                    idle_look_epilogue(c, frame, true);
                    return Ok(());
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S4][0] as i64) < 6) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = 0x24;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0011_5e80);
                }
            }
        }

        // Otherwise glance back at a rider more than 120 degrees off (the
        // roll is always under 100).
        c.cycles += 6;
        call_ptr(c, 0x0031_7810, 0x0011_5fac)?;
        c.g[V1][0] = 0x64;
        c.divu(0, V0, V1);
        c.cycles += 4;
        c.g[A0][0] = c.hi[0];
        c.g[A0][0] = ((c.g[A0][0] as i64) < 100) as u64;
        let glance = c.g[A0][0] != 0;
        frame.restore(c, S0, 0x70);
        if !glance {
            idle_look_epilogue(c, frame, false);
            return Ok(());
        }
        c.cycles += 9;
        c.g[V1][0] = c.g[S2][0];
        c.lwc1(7, gp_2(c, -0x7b30));
        c.lwc1(6, gp_2(c, -0x7b2c));
        c.g[A2][0] = sx(r32_3(c, V1).wrapping_add(0xd8));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 5);
        c.g[A1][0] = sx(r32_3(c, V1).wrapping_add(0xc));
        c.g[A0][0] = 0;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32_3(c, V1));
            let empty = c.g[V0][0] == 0;
            c.g[V0][0] = sx(r32_3(c, A1).wrapping_add(r32_3(c, A0)));
            let mut skip_d8 = false;
            if !empty {
                // f1 = wrap(angle - heading) to -pi..pi.
                c.cycles += 9;
                let v0 = r32_3(c, V0);
                c.lwc1(1, v0);
                c.sub_s(4, 1, 22);
                c.mul_s(0, 4, 6);
                c.add_s(0, 0, 5);
                c.cvt_w_s(3, 0);
                c.cvt_s_w(3, 3);
                c.c_lt_s(0, 3);
                if c.fcc() {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.sub_s(3, 3, 0);
                }
                c.cycles += 10;
                c.lwc1(0, gp_2(c, -0x7b28));
                c.g[V0][0] = sx(r32_3(c, S2).wrapping_add(r32_3(c, A0)));
                c.g[AT][0] = 0x447a_0000;
                c.mtc1(1, 1);
                let v0 = r32_3(c, V0);
                c.lwc1(2, p(v0, 0x8));
                c.mul_s(0, 3, 0);
                c.c_lt_s(2, 1);
                let far = !c.fcc();
                c.sub_s(1, 4, 0);
                if !far {
                    c.cycles += 5;
                    c.abs_s(0, 1);
                    c.c_lt_s(7, 0);
                    let in_view = !c.fcc();
                    c.g[V1][0] = sx(r32_3(c, V1).wrapping_add(0x24));
                    if in_view {
                        skip_d8 = true;
                    } else {
                        c.cycles += 8;
                        c.mtc1(0, 0);
                        c.g[A0][0] = 1;
                        c.g[V1][0] = c.lw(p(rider, 0x784));
                        c.g[V0][0] = c.ld(p(rider, 0x8d0));
                        c.c_lt_s(1, 0);
                        let left = c.fcc();
                        let v1 = r32_3(c, V1);
                        c.sd(p(v1, 0x20), c.g[V0][0]);
                        if !left {
                            c.cycles += 1;
                            c.g[A0][0] = 0;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(p(rider, 0x320));
                        let other = c.g[A0][0] != c.g[V0][0];
                        c.g[A0][0] = c.lw(p(rider, 0x784));
                        if other {
                            c.cycles += 1;
                            c.g[A1][0] = 0x13c;
                            idle_look_play_b(c)?;
                        } else {
                            c.cycles += 1;
                            c.g[A1][0] = 0x13d;
                            idle_look_play_a(c)?;
                        }
                        idle_look_epilogue(c, frame, true);
                        return Ok(());
                    }
                }
            }
            if !skip_d8 {
                c.cycles += 1;
                c.g[V1][0] = sx(r32_3(c, V1).wrapping_add(0x24));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32_3(c, A0).wrapping_add(0x24));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0011_5ff0);
            }
        }
        idle_look_epilogue(c, frame, true);
        Ok(())
    })
}

/// One slot of `Rider_UpdateIdleLookAnim`'s look loop (s4 = index, s1 =
/// s4 * 0x24, f22 = heading, f23/f24 = 60/135 degrees). Returns the
/// animation (0x13f/0x141/0x140, with a0 = the controller) when this rider
/// is looked at, else advances s4.
fn idle_look_slot(c: &mut Cpu) -> Result<Option<u64>, call::Unwound> {
    c.cycles += 5;
    c.mult(0, S1, S4, V0);
    c.g[V1][0] = sx(r32_3(c, S2).wrapping_add(r32_3(c, S1)));
    c.g[V0][0] = c.lw(r32_3(c, V1));
    let empty = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[V1][0];
    if empty {
        c.cycles += 1;
        c.inc_r(S4, 1);
        return Ok(None);
    }
    c.cycles += 3;
    c.g[V1][0] = c.lw(p(r32_3(c, V0), 0x1c));
    let off = c.g[V1][0] == 0;
    c.g[V0][0] = sx(r32_3(c, S2).wrapping_add(0x20));
    if off {
        c.cycles += 1;
        c.inc_r(S4, 1);
        return Ok(None);
    }
    // Looked at less than 10 s ago (600 frames at 60 Hz): skip. The
    // cooldown is a duration: 600 ticks at 60, 1200 at 120.
    c.cycles += 7;
    c.g[S3][0] = sx(r32_3(c, V0).wrapping_add(r32_3(c, S1)));
    c.g[S0][0] = c.lw(r32_3(c, S3));
    let cooldown = crate::tick::ticks_from_frames60(c, 600);
    c.g[S0][0] = sx(r32_3(c, S0).wrapping_add(cooldown));
    anim_callee::FRAME_COUNTER.call(c, 0x0011_5eb0)?;
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.inc_r(S4, 1);
        return Ok(None);
    }
    // f21 = wrap(angle - heading) to -pi..pi; within 1000 units.
    c.cycles += 13;
    c.g[V0][0] = sx(r32_3(c, S2).wrapping_add(r32_3(c, S1)));
    c.lwc1(0, gp_2(c, -0x7b38));
    let v0 = r32_3(c, V0);
    c.lwc1(1, p(v0, 0xc));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(1, 2);
    c.sub_s(4, 1, 22);
    c.mul_s(0, 4, 0);
    c.add_s(0, 0, 2);
    c.cvt_w_s(3, 0);
    c.cvt_s_w(3, 3);
    c.c_lt_s(0, 3);
    if c.fcc() {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.sub_s(3, 3, 0);
    }
    c.cycles += 9;
    c.lwc1(0, gp_2(c, -0x7b34));
    c.lwc1(2, p(v0, 0x8));
    c.g[AT][0] = 0x447a_0000;
    c.mtc1(1, 1);
    c.mul_s(0, 3, 0);
    c.c_lt_s(2, 1);
    let far = !c.fcc();
    c.sub_s(21, 4, 0);
    if far {
        c.cycles += 1;
        c.inc_r(S4, 1);
        return Ok(None);
    }
    c.cycles += 5;
    c.abs_s(20, 21);
    c.c_lt_s(23, 20);
    if !c.fcc() {
        // Within 60 degrees of the heading (in view): no need to look.
        c.inc_r(S4, 1);
        return Ok(None);
    }
    // Look: stamp the slot, clear the blend source.
    c.cycles += 8;
    anim_callee::FRAME_COUNTER.call(c, 0x0011_5f3c)?;
    let s3 = r32_3(c, S3);
    c.sw(s3, c.g[V0][0]);
    c.c_lt_s(24, 20);
    let rider = r32_3(c, S2);
    c.g[V1][0] = c.lw(p(rider, 0x784));
    c.g[V0][0] = c.ld(p(rider, 0x8c0));
    let behind = c.fcc();
    let v1 = r32_3(c, V1);
    c.sd(p(v1, 0x20), c.g[V0][0]);
    if behind {
        c.cycles += 3;
        c.g[A0][0] = c.lw(p(rider, 0x784));
        return Ok(Some(0x13f));
    }
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(21, 0);
    let left = c.fcc();
    c.g[V1][0] = 1;
    if !left {
        c.cycles += 1;
        c.g[V1][0] = 0;
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(p(rider, 0x320));
    let other = c.g[V1][0] != c.g[V0][0];
    c.g[A0][0] = c.lw(p(rider, 0x784));
    c.cycles += 2;
    Ok(Some(if other { 0x140 } else { 0x141 }))
}

/// `Rider_UpdateIdleLookAnim` 0x11609c: play a1 (blend -1) and reset the
/// controller's blend source.
fn idle_look_play_a(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 12);
    c.g[A2][0] = 0;
    call_ptr(c, IDLE_LOOK_PLAY, 0x0011_60ac)?;
    idle_look_clear(c);
    Ok(())
}

/// `Rider_UpdateIdleLookAnim` 0x1160b8: the same call at its other site.
fn idle_look_play_b(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 12);
    c.g[A2][0] = 0;
    call_ptr(c, IDLE_LOOK_PLAY, 0x0011_60c8)?;
    idle_look_clear(c);
    Ok(())
}

/// 0x1160cc: `controller+0x20 = -1`.
fn idle_look_clear(c: &mut Cpu) {
    let rider = r32_3(c, S2);
    c.g[V1][0] = c.lw(p(rider, 0x784));
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let v1 = r32_3(c, V1);
    c.sd(p(v1, 0x20), c.g[V0][0]);
}

/// 0x1160e8 (`with_s0`: s0 not yet restored) / 0x1160ec.
fn idle_look_epilogue(c: &mut Cpu, frame: Frame, with_s0: bool) {
    if with_s0 {
        c.cycles += 1;
        frame.restore(c, S0, 0x70);
    }
    c.cycles += 12;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore_ra(c, 0x20);
    for (f, off) in [(24, 0xa0), (23, 0x98), (22, 0x90), (21, 0x88), (20, 0x80)] {
        frame.restore_fpr(c, f, off);
    }
    frame.close(c);
}

/// `void Rider_StopTransition(Rider* r)` (0x125038): when the rider is in a
/// view and its view index is valid, stop the transition on the current
/// camera.
pub fn rider_stop_transition(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(gp_5(c, -0x848)).lw(c);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p_2(c.g[V1][0] as u32, 0x84)).lw(c);
        c.g[V0][0] = Addr(p_2(c.g[A0][0] as u32, 0x84)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = Addr(p_2(r32(c, S0), 0x870)).lw(c);
            if (c.g[A1][0] as i64) >= 0 {
                c.cycles += 6;
                call::call_ptr(c, 0x0023_0698, 0x0012_5070)?;
                c.g[V1][0] = Addr(p_2(c.g[V0][0] as u32, 0x78)).lw(c);
                c.g[V0][0] = 1;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(gp_5(c, -0x848)).lw(c);
                    c.g[A1][0] = Addr(p_2(r32(c, S0), 0x870)).lw(c);
                    c.g[A0][0] = Addr(p_2(c.g[V0][0] as u32, 0x84)).lw(c);
                    call::call_ptr(c, 0x0023_0698, 0x0012_5090)?;
                    c.g[A0][0] = c.g[V0][0];
                    call::call_ptr(c, 0x002e_4578, 0x0012_5098)?;
                }
            }
        }
        lqsp(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `Rider_MarkQuitRequested` (0x1253D0): when the rider's "finished" timer is
/// negative, flag the rider as a did-not-finish and set its AI index entry.
pub fn rider_mark_quit_requested(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.mtc1(ZERO, 1);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.lwc1(0, r32(c, A0).wrapping_add(0x470));
        c.c_le_s(1, 0);
        let finished = c.fcc();
        c.g[V0][0] = 1;
        if !finished {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        let skip = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x10);
        if !skip {
            c.cycles += 14;
            c.g[S0][0] = 1;
            c.g[V1][0] = lw_at(c, A0, 0x6c0);
            sw_at_2(c, A0, 0x480, c.g[S0][0]);
            c.g[A1][0] = low(c, A0).wrapping_add(0x6c0);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x38)).lh(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3c)).lw(c);
            let index = r32(c, V0);
            c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, index, 0x0012_5420)?;
            c.g[V1][0] = 0x53_0000;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = low(c, V1).wrapping_add(0x66d0);
            c.g[V0][0] = low(c, V0).wrapping_add(low(c, V1));
            c.sw(r32(c, V0), c.g[S0][0]);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int Player_GetUberGrindButton(self)` (0x127848): which of the four uber
/// grind buttons (0x1b..0x1e) is held, or -1.
pub fn player_get_uber_grind_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x20));
        c.g[A1][0] = 0x1b;
        c.sq(16, sp_3(c).wrapping_add(0x10));
        c.sd(sp_3(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = lw_r(c, S0, 0xdf0);
        call_ptr(c, 0x0032_0c48, 0x0012_7864)?;
        let mut v0;
        if c.g[V0][0] != 0 {
            v0 = 0;
        } else {
            c.cycles += 5;
            c.g[A0][0] = lw_r(c, S0, 0xdf0);
            c.g[A1][0] = 0x1c;
            call_ptr(c, 0x0032_0c48, 0x0012_7878)?;
            if c.g[V0][0] != 0 {
                v0 = 1;
            } else {
                c.cycles += 5;
                c.g[A0][0] = lw_r(c, S0, 0xdf0);
                c.g[A1][0] = 0x1d;
                call_ptr(c, 0x0032_0c48, 0x0012_788c)?;
                if c.g[V0][0] != 0 {
                    v0 = 2;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = lw_r(c, S0, 0xdf0);
                    c.g[A1][0] = 0x1e;
                    call_ptr(c, 0x0032_0c48, 0x0012_78a0)?;
                    c.g[A0][0] = 0x3;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[A0][0];
                    }
                    v0 = low_2(c, V1);
                }
            }
        }
        c.g[V0][0] = sx(v0);
        c.cycles += 4;
        c.lq(16, sp_3(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_3(c)).ld(c);
        let t = low_2(c, RA);
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void Rider_UpdateFromScript(Rider* self, ScriptState* script,
/// float, float, int)` (0x123da8): gather the script camera state, then apply
/// the script track and its sounds.
pub fn rider_update_from_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        open(c, 0x70);
        let sp0 = spv(c);
        c.sq(16, at_4(sp0, 0x50));
        c.sq(17, at_4(sp0, 0x40));
        c.g[S0][0] = c.g[A0][0];
        c.swc1(21, at_4(sp0, 0x68));
        c.g[A0][0] = c.g[A1][0];
        c.swc1(20, at_4(sp0, 0x60));
        c.mov_s(21, 13);
        c.sd(at_4(sp0, 0x30), c.g[RA][0]);
        c.mov_s(20, 12);
        let this = c.g[S0][0];
        c.g[V0][0] = lw_at_2(c, this, 0xac4);
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A3][0];
        if !none {
            c.cycles += 14;
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, callee::RIDER_GATHER_CAM_STATE, 0x0012_3de4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            c.mov_s(12, 20);
            c.mov_s(13, 21);
            c.g[A2][0] = c.g[S1][0];
            callee::RIDER_APPLY_SCRIPT_TRACK.call(c, 0x0012_3dfc)?;
            c.g[A0][0] = c.g[S0][0];
            c.mov_s(12, 20);
            c.mov_s(13, 21);
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            callee::RIDER_UPDATE_SCRIPT_SOUNDS.call(c, 0x0012_3e14)?;
        }
        c.cycles += 7;
        c.lq(16, at_4(sp0, 0x50));
        c.lq(17, at_4(sp0, 0x40));
        c.g[RA][0] = c.ld(at_4(sp0, 0x30));
        c.lwc1(21, at_4(sp0, 0x68));
        c.lwc1(20, at_4(sp0, 0x60));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_4(sp0, 0x70));
        c.pc = ra;
        Ok(())
    })
}

/// `int Rider_EstimateFinalScore(Rider* r)` (0x122e50): extrapolate the final
/// score from the distance still to go and the time so far.
pub fn rider_estimate_final_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_3(c, S0, 0x790);
        c.g[V0][0] = lw_3(c, V1, 0x198);
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        callee::RIDER_GET_START_DIST_TO_FINISH.call(c, 0x0012_2e74)?;
        c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x4d0));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.sub_s(2, 0, 3);
        c.c_lt_s(2, 1);
        if c.fcc() {
            c.mov_s(2, 1);
        }
        c.cycles += 11;
        c.g[AT][0] = 0x447a_0000;
        c.mtc1(AT, 0);
        c.mtc1(S1, 1);
        c.cvt_s_w(1, 1);
        c.mtc1(0, 4);
        c.sub_s(0, 3, 0);
        c.div_s(1, 1, 2);
        c.c_lt_s(0, 4);
        if c.fcc() {
            c.mov_s(0, 4);
        }
        c.cycles += 9;
        c.mul_s(0, 0, 1);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int RiderModel_FindBone(model, index, name)` (0x310c48): find the named
/// bone inside the indexed model part, returning its absolute bone index.
pub fn rider_model_find_bone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S0, 0x40);
        c.g[V1][0] = 0x58;
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        let this = abi::arg(c, 0);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).off(0x1c).lw(c);
        c.g[A3][0] = Addr(this).off(0xc).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.mult(0, V0, V0, V1);
        c.g[S2][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = lw_4(c, S2, 0x44);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = lw_4(c, S2, 0x38);
        if none {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 1;
            c.g[S1][0] = c.g[A0][0];
            'bones: loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, table::STR_CASECMP, 0x0031_0ca4)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = lw_4(c, S2, 0x44);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw_4(c, S2, 0x4);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    break 'bones;
                }
                c.cycles += 4;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x50));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_0c98);
                    }
                    continue 'bones;
                }
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
                break 'bones;
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

/// `void Rider_PlaceAtMarker(rider, marker, int a2)` (0x11de60): place the
/// rider at a marker transform, reset its matrix and state.
pub fn rider_place_at_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x40);
        let arg1 = c.g[A1][0];
        c.g[V0][0] = arg1;
        frame.save(c, S0, 0x30);
        c.g[A1][0] = c.g[A2][0];
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 0x4d0000;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
        call_ptr(c, 0x0026_b5e0, 0x0011_de88)?;
        c.lwc1(2, p(c.g[V0][0] as u32, 8));
        c.g[A1][0] = 0x1;
        c.lwc1(0, p(c.g[V0][0] as u32, 0xc));
        c.lwc1(1, p(c.g[V0][0] as u32, 0x10));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 3);
        c.swc1(2, c.g[SP][0] as u32);
        c.swc1(0, p(c.g[SP][0] as u32, 4));
        c.swc1(1, p(c.g[SP][0] as u32, 8));
        c.swc1(3, p(c.g[SP][0] as u32, 0xc));
        c.lwc1(2, p(c.g[V0][0] as u32, 0x1c));
        c.lwc1(0, p(c.g[V0][0] as u32, 0x14));
        c.lwc1(1, p(c.g[V0][0] as u32, 0x18));
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x790)).lw(c);
        c.swc1(0, p(c.g[SP][0] as u32, 0x10));
        c.swc1(1, p(c.g[SP][0] as u32, 0x14));
        c.swc1(2, p(c.g[SP][0] as u32, 0x18));
        c.swc1(3, p(c.g[SP][0] as u32, 0x1c));
        call_ptr(c, 0x0011_9368, 0x0011_ded4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0011_fec8, 0x0011_dee0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0011_fe78, 0x0011_deec)?;
        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = 0x5;
        call_ptr(c, 0x0011_d660, 0x0011_df04)?;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_PlaceAtStart(this)` (0x11dd98): read the start marker's position
/// and direction, offset the position by the start pose angle and place the
/// rider with type 0x11f and the 3000-unit snap radius.
pub fn rider_place_at_start(c: &mut Cpu) {
    c.cycles += 49;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[A1][0] = 0x2;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
    c.g[S0][0] = c.g[A0][0];
    c.g[A0][0] = 0x4d0000;
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x86c));
    c.g[RA][0] = sx(0x0011ddbc);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
    if call_ptr(c, 0x0026b5e0, 0x0011ddbc).is_err() {
        return;
    }
    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
    c.g[A0][0] = c.g[S0][0];
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 2);
    c.swc1(1, (c.g[SP][0] as u32));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x18));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x1c));
    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x1c));
    c.g[RA][0] = sx(0x0011de04);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
    c.pc = 0x00115b08;
    ee::s00::Rider_GetStartPoseAngle(c);
    if c.pc != 0x0011de04 {
        return;
    }
    c.mfc1(2, 0);
    c.lqc2(4, (c.g[SP][0] as u32));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x798c));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A2][0] = c.g[SP][0];
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = 0x11f;
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = sx(0x0011de4c);
    c.pc = 0x0011d660;
    ee::s00::Rider_Place(c);
    if c.pc != 0x0011de4c {
        return;
    }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t;
    return;
}

/// The world singleton at `gp_6-0x848`.
#[inline(always)]
pub(crate) fn world(c: &mut Cpu) -> u32 {
    Addr(gp_6(c, -0x848)).lw(c) as u32
}

/// `void Rider_ResetMotion(Rider *self, int notify)` (0x11df18): zero the
/// rider's motion, scale its velocity row by 1/833.33, reset the FX state and
/// (for a notify) push the new velocity into the rider's AI object.
pub fn rider_reset_motion(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x2;
        c.lq(2, p_3(c.g[S1][0], 0x1b0));
        c.g[A0][0] = Addr(p_3(c.g[S1][0], 0x784)).lw(c);
        c.sq(2, c.g[SP][0] as u32);
        call_ptr(c, 0x0031_1b20, 0x0011_df44)?;
        Addr(p_3(c.g[V0][0], 0x90)).sw(c, 0);
        c.lwc1(0, gp_6(c, -0x7988));
        c.mfc1(3, 0);
        c.lqc2(4, c.g[SP][0] as u32);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, p_3(c.g[SP][0], 0x20));
        c.lq(2, p_3(c.g[SP][0], 0x20));
        c.g[A0][0] = Addr(p_3(c.g[S1][0], 0x77c)).lw(c);
        c.sq(2, p_3(c.g[S1][0], 0x1e0));
        Addr(p_3(c.g[S1][0], 0x1e8)).sw(c, 0);
        c.sq(2, p_3(c.g[SP][0], 0x10));
        callee::RIDER_BASE_RESET_FX.call(c, 0x0011_df78)?;
        let no_notify = c.g[S0][0] == 0;
        c.g[A0][0] = sx(world(c));
        if !no_notify {
            c.cycles += 17;
            c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c0));
            c.g[V1][0] = Addr(p_3(c.g[S1][0], 0x6c0)).lw(c);
            c.g[V0][0] = Addr(p_3(c.g[A0][0], 0x84)).lw(c);
            c.g[A0][0] = Addr(p_3(c.g[V1][0], 0x38)).lh(c);
            c.g[A1][0] = Addr(p_3(c.g[V0][0], 0x84)).lw(c);
            c.g[V0][0] = Addr(p_3(c.g[V1][0], 0x3c)).lw(c);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(p_3(c.g[A1][0], 0x4)).lw(c);
            c.g[S0][0] = Addr(p_3(c.g[V1][0], 0xa0)).lw(c);
            c.g[S1][0] = Addr(p_3(c.g[S0][0], 0x14)).lw(c);
            c.g[V1][0] = Addr(p_3(c.g[S1][0], 0x18)).lh(c);
            let target = c.g[V0][0] as u32;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            call_ptr(c, target, 0x0011_dfb4)?;
            c.g[V1][0] = Addr(p_3(c.g[S1][0], 0x1c)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, target, 0x0011_dfc4)?;
        }
        c.cycles += 7;
        c.g[A0][0] = Addr(gp_6(c, 0xce8)).lw(c);
        call_ptr(c, 0x0030_99f8, 0x0011_dfcc)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider finish time.

/// `int Rider_EstimateFinishTime(Rider* self)` (0x122d78): estimate the
/// remaining time as `n + dist / rate`, where `rate` is the recent distance
/// rate clamped to at least `30 - aiIndex`.
///
/// `n` (`+0xc` of the AI module) counts ticks, so `(startDist - dist) / n` is
/// units travelled per *tick*, and so is the `30 - aiIndex` floor. At another
/// rate the same speed is fewer units per tick: [`per_tick_60`] scales the
/// floor, the estimate (`dist / rate` ticks) then follows on its own. At 60
/// the code is the original.
pub fn rider_estimate_finish_time(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.swc1(21, frame.at(c, 0x48));
        c.swc1(20, frame.at(c, 0x40));
        c.g[S1][0] = sx(p(r32(c, S0), 0x6c0));
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[AT][0] = 0x41f00000;
        c.mtc1(1, 21);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
        c.g[S2][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
        call_ptr(c, 0x0011_3130, 0x0012_2db8)?;
        c.lwc1(1, p(r32(c, S0), 0x4d0));
        c.mtc1(18, 2);
        c.cvt_s_w(2, 2);
        c.g[V1][0] = Addr(p(r32(c, S0), 0x6c0)).lw(c);
        c.sub_s(0, 0, 1);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
        c.div_s(20, 0, 2);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, f, 0x0012_2de0)?;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.sub_s(0, 21, 0);
        // The `30 - index` floor is a speed per 60 Hz tick.
        per_tick_60(c, 0);
        c.c_lt_s(20, 0);
        if c.fcc() {
            c.cycles += 9;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x6c0)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
            let f = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, f, 0x0012_2e10)?;
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.sub_s(20, 21, 0);
            // `f20` = the floor again (the second virtual call's index).
            per_tick_60(c, 20);
            c.lwc1(0, p(r32(c, S0), 0x4d0));
        } else {
            c.lwc1(0, p(r32(c, S0), 0x4d0));
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x30);
        c.div_s(0, 0, 20);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0);
        c.lwc1(21, frame.at(c, 0x48));
        c.lwc1(20, frame.at(c, 0x40));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
        frame.restore(c, S2, 0x10);
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    })
}

/// An amount per 60 Hz tick — a speed in units/tick, a per-tick increment, a
/// slew step, a per-tick phase — as the amount per tick at the game's rate
/// (`x * 60 / rate`). At 60 the value is returned unchanged, so the
/// differential test still sees the original bits.
#[inline]
pub(crate) fn rate_step_60(c: &mut Cpu, x: FpuF32) -> FpuF32 {
    let hz = crate::tick::rate(c);
    if hz == 60 {
        x
    } else {
        x * FpuF32::from_f32(60.0f32 / hz as f32)
    }
}

/// [`rate_step_60`] for the value in `f{r}` (nothing changes at 60).
#[inline]
pub(crate) fn per_tick_60(c: &mut Cpu, r: usize) {
    let x = fr_2(c, r);
    let y = rate_step_60(c, x);
    setf_2(c, r, y);
}

/// A per-60-Hz-tick smoothing factor `a` of `x += (t - x) * a`, as the factor
/// for the game's rate: `1 - (1 - a)^(60/rate)`, so the filter keeps its time
/// constant. At 60 it is unchanged.
#[inline]
pub(crate) fn smoothing_60(c: &mut Cpu, a: FpuF32) -> FpuF32 {
    let hz = crate::tick::rate(c);
    if hz == 60 {
        a
    } else {
        let a = a.to_f32();
        FpuF32::from_f32(1.0 - (1.0 - a).powf(60.0 / hz as f32))
    }
}

/// `void SSXAudioSystem_ArcadeSay20A8(void* self, void* obj, int arg)`
/// (0x2a3c00): as 0x2a3b18 with event `0x20a8`; the `+0x470` animation test is
/// skipped when `arg == 2`.
pub fn ssx_audio_system_arcade_say20a8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        callee::IS_ARCADE_SPEECH_ON.call(c, 0x002a_3c24)?;

        'outer: {
            if r(c, V0) == 0 {
                frame.restore(c, 16, 0x30);
                break 'outer;
            }

            'checks: {
                c.cycles += 2;
                let no_object = c.g[S0][0] == 0;
                c.g[V0][0] = 2;
                if no_object {
                    break 'checks;
                }

                'idle: {
                    c.cycles += 2;
                    let shortcut = c.g[S2][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    if shortcut {
                        break 'idle;
                    }
                    // f0 = 0; fcc = (f0 <= *(*obj + 0x470)).
                    c.cycles += 6;
                    c.lwc1(1, p(r(c, S0), 0x470));
                    c.mtc1(0, 0);
                    c.c_le_s(0, 1);
                    let done = c.fcc();
                    c.g[V0][0] = 1;
                    if !done {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                    c.cycles += 2;
                    if r(c, V0) != 0 {
                        frame.restore(c, 16, 0x30);
                        break 'outer;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S1][0];
                }

                c.cycles += 4;
                c.g[A1][0] = u64::MAX;
                call_ptr(c, 0x0028_5d98, 0x002a_3c6c)?;
                if c.g[S0][0] != c.g[V0][0] {
                    frame.restore(c, 16, 0x30);
                    break 'outer;
                }

                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0028_8ae0, 0x002a_3c7c)?;
                let busy = r(c, V0) != 0;
                frame.restore(c, 16, 0x30);
                if busy {
                    break 'outer;
                }

                c.cycles += 11;
                c.mtc1(0, 12);
                c.g[A0][0] = sx(p(r(c, S1), 0x5560));
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20a8;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0xc;
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                callee::SPEECH_SET_EVENT_CONTEXT.call(c, 0x002a_3ca8)?;
                let skip = r(c, V0) == 0;
                c.g[A0][0] = 1;
                if !skip {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0x20a8;
                    call_ptr(c, 0x003d_8008, 0x002a_3cbc)?;
                    c.g[V1][0] = Addr(gp_7(c, 0x173c)).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S2][0];
                    let target = r(c, V1);
                    c.g[A1][0] = 1;
                    call_ptr(c, target, 0x002a_3cd0)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
        }

        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Quat_FromEuler(out, x, y, z)` (0x1241c0): build a quaternion from three
/// Euler angles (half-angle sines and cosines through `Math_SinCos`, then the
/// standard product/sum combinations) and store it in `out`.
pub fn quat_from_euler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        let frame = Frame::open(c, 0x70);
        let out = abi::arg(c, 1);
        c.swc1(20, p(sp(c), 0x50));
        c.g[A0][0] = sx(p(sp(c), 0x10));
        c.swc1(22, p(sp(c), 0x60));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 20); // f20 = 0.5
        c.mov_s(22, 12);
        frame.save(c, 16, 0x40);
        c.mul_s(12, 14, 20);
        c.swc1(21, p(sp(c), 0x58));
        c.g[S0][0] = c.g[A1][0];
        c.mov_s(21, 13);
        c.sd(p(sp(c), 0x30), c.g[RA][0]);
        c.g[RA][0] = sx(0x0012_41fc);
        c.g[A1][0] = sx(p(sp(c), 0x14));
        c.pc = 0x0031_be50;
        call_ptr(c, 0x0031_be50, 0x0012_41fc)?; // Math_SinCos(sin, cos, z*0.5)
        c.mul_s(12, 21, 20);
        c.g[A0][0] = sx(p(sp(c), 0x18));
        c.g[RA][0] = sx(0x0012_420c);
        c.g[A1][0] = sx(p(sp(c), 0x1c));
        c.pc = 0x0031_be50;
        call_ptr(c, 0x0031_be50, 0x0012_420c)?;
        c.mul_s(12, 22, 20);
        c.g[A0][0] = sx(p(sp(c), 0x20));
        c.g[RA][0] = sx(0x0012_421c);
        c.g[A1][0] = sx(p(sp(c), 0x24));
        c.pc = 0x0031_be50;
        call_ptr(c, 0x0031_be50, 0x0012_421c)?;
        c.lwc1(2, p(sp(c), 0x14));
        c.lwc1(5, p(sp(c), 0x10));
        c.lwc1(1, p(sp(c), 0x1c));
        c.lwc1(6, p(sp(c), 0x24));
        c.lwc1(0, p(sp(c), 0x18));
        c.mul_s(7, 2, 1);
        c.lwc1(4, p(sp(c), 0x20));
        c.mul_s(11, 1, 6);
        c.mul_s(8, 5, 0);
        c.g[RA][0] = c.ld(p(sp(c), 0x30));
        c.mul_s(10, 0, 4);
        c.lwc1(22, p(sp(c), 0x60));
        c.mul_s(0, 2, 0);
        c.lwc1(21, p(sp(c), 0x58));
        c.mul_s(1, 5, 1);
        c.lwc1(20, p(sp(c), 0x50));
        c.mul_s(3, 5, 11);
        c.mul_s(9, 2, 10);
        c.mul_s(7, 7, 4);
        c.mul_s(8, 8, 6);
        c.mul_s(0, 0, 6);
        c.mul_s(1, 1, 4);
        c.mul_s(2, 2, 11);
        c.mul_s(5, 5, 10);
        c.sub_s(3, 3, 9);
        c.add_s(0, 0, 1);
        c.add_s(2, 2, 5);
        c.sub_s(7, 7, 8);
        c.swc1(3, sp(c));
        c.swc1(0, p(sp(c), 4));
        c.swc1(2, p(sp(c), 0xc));
        c.swc1(7, p(sp(c), 8));
        c.lq(2, sp(c));
        c.sq(2, out);
        frame.restore(c, 16, 0x40);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Addresses.

/// `int InputBinding_EvalBool(InputBinding*, int value)` — evaluates the input
/// map bound to a button, non-zero when the button is held.
pub(crate) const INPUT_BINDING_EVAL_BOOL: u32 = 0x0032_0c48;

// ---------------------------------------------------------------------------
// Player / FE input queries.

/// `int Player_GetTrickButton(Player *p)` (0x1276f0): index of the first held
/// trick button (bindings 0x1f..0x2c map to 0..13), 14 for the last binding
/// (0x2d) or -1 when none is held.
pub fn player_get_trick_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);

        for i in 0..14u32 {
            if i > 0 {
                c.cycles += 5;
            }
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xdf0)).lw(c);
            c.g[A1][0] = (0x1f + i) as u64;
            call_ptr(c, INPUT_BINDING_EVAL_BOOL, 0x0012_770c + 0x14 * i)?;
            let held = c.g[V0][0] != 0;
            c.g[V0][0] = i as u64;
            if held {
                c.cycles += 4;
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }

        // Binding 0x2d: report 14 when held, -1 otherwise.
        c.cycles += 7;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xdf0)).lw(c);
        c.g[A1][0] = 0x2d;
        call_ptr(c, INPUT_BINDING_EVAL_BOOL, 0x0012_7824)?;
        c.g[A0][0] = 0xe;
        c.g[V1][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[A0][0];
        }
        c.g[V0][0] = c.g[V1][0];

        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
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
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
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
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng as rng_3;
    pub(crate) use crate::game::ai::ai::tests::strict as strict_2;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_11;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::globals_3 as globals_4;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::pool_4 as pool_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::w as w_8;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4_2 as OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_8;
    pub(crate) use crate::game::fe::festateoptions::tests::rng as rng_2;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_5;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_3;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_4;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_7;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE_3;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::W as W_2;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_7;
    pub(crate) use crate::game::main::debugmenu::tests::w as w_9;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::w as w_3;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_10;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_9;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_4;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_3;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::w as w_6;
    pub(crate) use crate::game::sound::soundsys::tests::wf as wf_2;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD as WORLD_4;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::ctx_at;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::put_ctx;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::put_world;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::BASE as BASE_2;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::CTX;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_3 as SEEDS_2;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::TEX;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::TOP;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::WORLD as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::dummy_world;
    pub(crate) use crate::game::wscript::wscriptman::tests::gp_at;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6;
    use ssx3_ee as ee;

    #[test]
    fn rider_set_slew5_target_matches() {
        check(
            ssx3_ee::s00::f_00113f38,
            rider_set_slew5_target,
            300,
            &|c, seed| {
                let mut r = Rng(0x2233_4455 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x22c, r.edge_f32());
                c.f[12] = r.edge_f32();
                // Alternate a small cap (compare true -> min.s path) and a huge one
                // (compare false -> cap path) so both branches are taken.
                let cap = if seed % 2 == 0 {
                    r.f32(1.0)
                } else {
                    0x4f00_0000 | (r.next_u32() & 0x007f_ffff)
                };
                c.bus.write32(GAME_GP - 0x7c28, cap);
                c.bus.write32(GAME_GP - 0x7c24, r.edge_f32());
                c.bus.write32(DATA + 0x230, r.next_u32());
                c.bus.write32(DATA + 0x234, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_get_main_state_name_matches() {
        check(
            ssx3_ee::s00::f_0011fea8,
            rider_get_main_state_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x2468_ace0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let base = DATA + 0x1000;
                c.bus.write32(DATA + 0x77c, base);
                // Cover both branches of the callee's range test (< 7 vs >= 7).
                let state = if seed % 2 == 0 {
                    r.next_u32() % 7
                } else {
                    7 + r.next_u32() % 8
                };
                c.bus.write32(base + 0xde0, state);
                // name tables the callee indexes.
                for i in 0..8 {
                    c.bus.write32(0x459d70 + i * 4, r.next_u32());
                    c.bus.write32(0x456c48 + i * 4, r.next_u32());
                }
            },
        );
    }

    pub(crate) const SEEDS: u32 = 250;

    pub(crate) const RIDER: u32 = DATA;

    pub(crate) const MODEL: u32 = DATA + 0x2000;

    pub(crate) const BONE_OFFSETS: u32 = DATA + 0x3000;

    pub(crate) const BONE_ROTATIONS: u32 = DATA + 0x3400;

    pub(crate) const BONES: u32 = DATA + 0x3800;

    pub(crate) const NBONES: u32 = 8;

    pub(crate) const STATE: u32 = DATA + 0x6000;

    pub(crate) fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// A float in [lo, hi).
    pub(crate) fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        f(lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo))
    }

    pub(crate) fn rand_vec(c: &mut Cpu, r: &mut Rng, a: u32, range: f32) {
        let v = [r.f32(range), r.f32(range), r.f32(range), r.f32(range)];
        put_vec(c, a, v);
    }

    pub(crate) fn rand_quat(c: &mut Cpu, r: &mut Rng, a: u32) {
        rand_vec(c, r, a, 1.0);
    }

    /// The boot-time globals: pivot (zero for `zero_pivot`), axes, identity.
    pub(crate) fn put_globals(c: &mut Cpu, r: &mut Rng, zero_pivot: bool) {
        let pivot = if zero_pivot {
            [0; 4]
        } else {
            [r.f32(0.5), r.f32(0.5), r.f32(0.5), 0]
        };
        put_vec(c, PIVOT, pivot);
        put_vec(c, AXIS_X, [f(1.0), 0, 0, 0]);
        put_vec(c, AXIS_Z, [0, 0, f(1.0), 0]);
        put_vec(c, AXIS_Y, [0, f(1.0), 0, 0]);
        put_vec(c, IDENTITY_QUAT, [0, 0, 0, f(1.0)]);
    }

    /// The rider's model with its three bone tables (zero scale for
    /// `zero_scale`).
    pub(crate) fn put_model(c: &mut Cpu, r: &mut Rng, zero_scale: bool) {
        c.bus.write32(RIDER + 0x780, MODEL);
        c.bus.write32(MODEL + 0x24, BONE_OFFSETS);
        c.bus.write32(MODEL + 0x28, BONE_ROTATIONS);
        c.bus.write32(MODEL + 0x2c, BONES);
        for k in 0..4 {
            let s = if zero_scale { 0 } else { uf(r, 0.5, 1.5) };
            c.bus.write32(MODEL + 0x140 + 4 * k, s);
        }
        for i in 0..NBONES {
            rand_vec(c, r, BONE_OFFSETS + 0x10 * i, 2.0);
            rand_quat(c, r, BONE_ROTATIONS + 0x10 * i);
            rand_vec(c, r, BONES + 0x20 * i, 3.0);
            rand_quat(c, r, BONES + 0x20 * i + 0x10);
        }
    }

    /// The rider's state object (`+0x77c`): main state `+0xde0`, sub-state
    /// `+0xde4`.
    pub(crate) fn put_state(c: &mut Cpu, main: u32, sub: u32) {
        c.bus.write32(RIDER + 0x77c, STATE);
        c.bus.write32(STATE + 0xde0, main);
        c.bus.write32(STATE + 0xde4, sub);
    }

    /// The board-lean inputs.
    pub(crate) fn put_lean(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(RIDER + 0x8a0, r.next_u32() % NBONES);
        c.bus.write32(RIDER + 0x214, r.f32(0.3));
        let roll = if seed % 3 == 0 {
            r.f32(0.2)
        } else {
            r.f32(2.0)
        };
        c.bus.write32(RIDER + 0x208, roll);
        let pitch = if seed % 4 == 1 {
            r.f32(0.2)
        } else {
            r.f32(2.0)
        };
        c.bus.write32(RIDER + 0x1f0, pitch);
        let spin = if seed % 5 == 0 { 0 } else { r.f32(1.0) };
        c.bus.write32(RIDER + 0x250, spin);
        rand_vec(c, r, RIDER + 0x3b0, 2.0);
    }

    #[test]
    fn rider_update_bounds_matches() {
        check(
            ee::s00::f_0011e150__generated,
            rider_update_bounds,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0011_e150 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                for off in [0x110, 0x1a0, 0x1b0, 0x1c0] {
                    rand_vec(c, &mut r, RIDER + off, 3.0);
                }
                for k in 0..8 {
                    c.bus.write32(RIDER + 0x400 + 4 * k, r.f32(600.0));
                }
                let centre = DATA + 0x5000;
                rand_vec(c, &mut r, centre, 100.0);
                c.g[A0][0] = sx(RIDER);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(centre) };
            },
        );
    }

    #[test]
    fn rider_update_board_lean_matches() {
        check(
            ee::s00::f_0011fa10__generated,
            rider_update_board_lean,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xfa10, 0);
                elf_image(c);
                put_globals(c, &mut r, seed % 4 == 0);
                put_model(c, &mut r, false);
                put_lean(c, &mut r, seed);
                let xf = DATA + 0x5000;
                rand_vec(c, &mut r, xf, 5.0);
                rand_quat(c, &mut r, xf + 0x10);
                c.g[A0][0] = sx(RIDER);
                c.g[A1][0] = sx(xf);
            },
        );
    }

    #[test]
    fn rider_solve_leg_ik_matches() {
        check(
            ee::s00::f_0011f3d8__generated,
            rider_solve_leg_ik,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf3d8, 0);
                elf_image(c);
                let degenerate = seed % 6 == 0;
                put_globals(c, &mut r, degenerate || seed % 4 == 0);
                put_model(c, &mut r, degenerate);
                let leg = DATA + 0x5000;
                for k in 0..3 {
                    c.bus.write32(leg + 4 * k, r.next_u32() % NBONES);
                }
                let target = DATA + 0x5100;
                rand_vec(c, &mut r, target, 4.0);
                rand_quat(c, &mut r, target + 0x10);
                c.f[12] = if seed % 3 == 0 {
                    f(1.0)
                } else {
                    uf(&mut r, 0.0, 1.0)
                };
                c.g[A0][0] = sx(RIDER);
                c.g[A1][0] = sx(leg);
                c.g[A2][0] = sx(target);
            },
        );
    }

    #[test]
    fn rider_update_skeleton_matches() {
        check(
            ee::s00::f_0011eb98__generated,
            rider_update_skeleton,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xeb98, 0);
                elf_image(c);
                put_globals(c, &mut r, seed % 4 == 0);
                put_model(c, &mut r, false);
                put_lean(c, &mut r, seed);
                let sub = [5, 0xd, 0xb, 0, 5, 1][(seed % 6) as usize];
                put_state(c, 0, sub);
                rand_vec(c, &mut r, RIDER + 0x110, 5.0);
                rand_quat(c, &mut r, RIDER + 0x120);
                rand_vec(c, &mut r, RIDER + 0x130, 5.0);
                rand_quat(c, &mut r, RIDER + 0x140);
                let spin = if seed % 7 == 0 { 0 } else { r.f32(2.0) };
                c.bus.write32(RIDER + 0x2c8, spin);
                c.bus.write32(RIDER + 0x150, (seed / 2) % 2);
                c.bus.write32(RIDER + 0x8a4, r.next_u32() % NBONES);
                let twist = if seed % 5 == 2 { 0 } else { r.f32(1.0) };
                c.bus.write32(RIDER + 0x2bc, twist);
                rand_vec(c, &mut r, RIDER + 0x390, 1.0);
                if seed % 9 == 4 {
                    // Parallel to the bone's axis: no twist axis.
                    put_vec(c, RIDER + 0x390, [0; 4]);
                }
                c.bus.write32(RIDER + 0x31c, r.f32(1.0));
                rand_vec(c, &mut r, RIDER + 0x370, 1.0);
                let ik = if seed % 8 == 3 {
                    0
                } else {
                    uf(&mut r, 0.0, 1.0)
                };
                c.bus.write32(RIDER + 0x318, ik);
                for desc in [0x8e0, 0x910] {
                    for k in 0..3 {
                        c.bus.write32(RIDER + desc + 4 * k, r.next_u32() % NBONES);
                    }
                    rand_vec(c, &mut r, RIDER + desc + 0x10, 2.0);
                    rand_quat(c, &mut r, RIDER + desc + 0x20);
                }
                c.g[A0][0] = sx(RIDER);
            },
        );
    }

    /// The animation controller of `Rider_UpdateSpeedAnims` (layer
    /// sequences, current animation ids, a handler vtable whose `PlayAnim`
    /// hook is a fake that returns 0x207 so `PlayAnim` stops there), the
    /// frame counter and the channel inputs.
    pub(crate) fn put_speed_anims(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let ctrl = DATA + 0x7000;
        let layers = DATA + 0x7100;
        c.bus.write32(RIDER + 0x784, ctrl);
        c.bus.write32(ctrl + 0x50, layers);
        for layer in 3..6u32 {
            let seq = if r.next_u32() % 3 == 0 {
                0
            } else {
                DATA + 0x7200 + 0x100 * layer
            };
            c.bus.write32(layers + 8 * layer + 4, seq);
            let first = [0x19b, 0x1a2, 0x1a9][(layer - 3) as usize];
            c.bus.write32(ctrl + 4 * layer, first + r.next_u32() % 8);
        }
        let vt = vtable(c, r, &[0x20]);
        c.bus.write32(ctrl + 0x58, vt);
        for i in 0..8 {
            c.bus.write32(LOG_RESULTS + 4 * i, 0x207);
        }
        // AI_GetFrameCounter: [[[gp-0x848]+0x84]+0xc]+8.
        let (a, b, d) = (DATA + 0x7800, DATA + 0x7900, DATA + 0x7a00);
        c.bus.write32(GAME_GP - 0x848, a);
        c.bus.write32(a + 0x84, b);
        c.bus.write32(b + 0xc, d);
        let frame = r.next_u32() % 1000;
        c.bus.write32(d + 8, frame);
        let phase = if seed % 4 == 3 {
            (frame + 1) % 6
        } else {
            frame % 6
        };
        c.bus.write32(RIDER + 0x86c, phase);
        // Velocity relative to +0x3d0; speeds up to ~2500.
        let range = [100.0, 600.0, 1500.0][(seed % 3) as usize];
        rand_vec(c, r, RIDER + 0x1e0, range);
        rand_vec(c, r, RIDER + 0x3d0, 50.0);
        put_state(
            c,
            0,
            if seed % 5 == 1 {
                0xb
            } else {
                r.next_u32() % 16
            },
        );
        for en in [0x940, 0x970, 0x9a0] {
            c.bus.write32(RIDER + en, (r.next_u32() % 4 != 0) as u32);
        }
        c.bus.write32(RIDER + 0x320, r.next_u32() % 2);
        for bone in [0x8a8, 0x8ac, 0x89c] {
            c.bus.write32(RIDER + bone, r.next_u32() % NBONES);
        }
        for q in [0x950, 0x960, 0x980, 0x990, 0x9b0, 0x9c0] {
            rand_quat(c, r, RIDER + q);
        }
    }

    #[test]
    fn rider_update_speed_anims_matches() {
        check(
            ee::s00::f_00120378__generated,
            rider_update_speed_anims,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0378, 3);
                elf_image(c);
                put_globals(c, &mut r, false);
                put_model(c, &mut r, false);
                put_speed_anims(c, &mut r, seed);
                c.g[A0][0] = sx(RIDER);
            },
        );
    }

    #[test]
    fn rider_update_slewed_params_matches() {
        check(
            ee::s00::f_001211f8__generated,
            rider_update_slewed_params,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11f8, 3);
                elf_image(c);
                put_globals(c, &mut r, false);
                put_model(c, &mut r, false);
                put_speed_anims(c, &mut r, seed);
                for i in 0..SLEWED_COUNT {
                    let a = RIDER + SLEWED_FIRST + SLEWED_STRIDE * i;
                    c.bus.write32(a, r.f32(10.0));
                    c.bus.write32(a + 4, uf(&mut r, 0.0, 3.0));
                    c.bus.write32(a + 8, r.f32(10.0));
                }
                c.g[A0][0] = sx(RIDER);
            },
        );
    }

    /// A float in [lo, hi).
    pub(crate) fn uf_2(r: &mut Rng, lo: f32, hi: f32) -> f32 {
        lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)
    }

    pub(crate) fn put_v(c: &mut Cpu, at: u32, v: [f32; 4]) {
        for (k, x) in v.iter().enumerate() {
            c.bus.write32(at + 4 * k as u32, f(*x));
        }
    }

    pub(crate) fn rand_v(r: &mut Rng, lo: f32, hi: f32) -> [f32; 4] {
        [uf_2(r, lo, hi), uf_2(r, lo, hi), uf_2(r, lo, hi), 0.0]
    }

    pub(crate) fn gp_f32(c: &mut Cpu, off: i32, x: f32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), f(x));
    }

    // -----------------------------------------------------------------
    // Shared fixtures.

    /// Vtables: the collider sub-object (0x130 delegate, 0x138 contacts
    /// added), surface owners (0x150, 0x158), cache volumes (0x20 triangle
    /// test, 0x40 UV refine), hulls (0x18 reference depth).
    pub(crate) const VT_SUB: u32 = DATA + 0x3_0000;

    pub(crate) const VT_OWNER: u32 = DATA + 0x3_0400;

    pub(crate) const VT_VOL: u32 = DATA + 0x3_0600;

    pub(crate) const VT_HULL: u32 = DATA + 0x3_0700;

    pub(crate) const OWNER: u32 = DATA + 0x3_0800;

    /// Four surfaces (0x100 apart), their material table, four instances.
    pub(crate) const SURFS: u32 = DATA + 0x3_1000;

    pub(crate) const MATS: u32 = DATA + 0x3_2000;

    pub(crate) const INSTS: u32 = DATA + 0x3_3000;

    /// The overlap list and its objects (0x100 apart).
    pub(crate) const LIST: u32 = DATA + 0x3_4000;

    pub(crate) const OBJS: u32 = DATA + 0x3_5000;

    /// The rider, its state base (`+0x77c`), params (`+0x780`), sphere tree.
    pub(crate) const RIDER_2: u32 = DATA + 0x1_0000;

    pub(crate) const BASE: u32 = DATA + 0x2_0000;

    pub(crate) const PARAMS: u32 = DATA + 0x2_8000;

    pub(crate) const TREE: u32 = DATA + 0x2_c000;

    /// `fakes::fake_call`, then a result that depends on the method.
    pub(crate) fn fake_varied(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let h = c.g[V1][0];
        match target.wrapping_sub(FAKE) {
            0x10 => c.g[V0][0] = h & 3,                      // contacts added
            0x60 => c.g[V0][0] = ((h >> 4) & 3 == 0) as u64, // triangle hit
            _ => {}
        }
    }

    pub(crate) fn put_slot(c: &mut Cpu, vt: u32, off: u32, target: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, target);
    }

    /// Scrambled registers, the fake hooks and vtables, surfaces,
    /// materials and instances.
    pub(crate) fn setup_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.hooks.miss = fake_varied;
        put_slot(c, VT_SUB, 0x130, FAKE);
        put_slot(c, VT_SUB, 0x138, FAKE + 0x10);
        put_slot(c, VT_OWNER, 0x150, FAKE + 0x40);
        put_slot(c, VT_OWNER, 0x158, FAKE + 0x50);
        put_slot(c, VT_VOL, 0x20, FAKE + 0x60);
        put_slot(c, VT_VOL, 0x40, FAKE + 0x70);
        put_slot(c, VT_HULL, 0x18, FAKE + 0x80);
        c.bus.write32(OWNER + 0xc, VT_OWNER);
        for k in 0..4 {
            let s = SURFS + 0x100 * k;
            c.bus.write32(s + 0x78, r.next_u32() % 3);
            c.bus.write32(s + 0xc, if k % 2 == 0 { OWNER } else { 0 });
            c.bus.write32(s + 0x88, MATS);
            c.bus
                .write32(s + 0x8, if r.next_u32() % 3 == 0 { 0x2000 } else { 0 });
            c.bus.write32(s + 0x8c, 0);
        }
        for i in 0..4 {
            let m = MATS + 0x10 + 0xc * i;
            c.bus
                .write32(m, if r.next_u32() % 3 == 0 { 0 } else { f(1.0) });
            c.bus.write32(m + 4, f(uf_2(&mut r, 0.0, 2.0)));
            c.bus.write16(m + 8, (r.next_u32() & 3) as u16);
        }
        for k in 0..4 {
            let i = INSTS + 0x200 * k;
            c.bus.write32(i + 0x150, r.next_u32() % 3);
            c.bus.write16(i + 0xa, r.next_u32() as u16);
        }
        // The behaviour library exists (a zeroed dummy object: no
        // construction or disc load on first use).
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        // A world whose race manager has one rider (this one): without it
        // `SSXAudioSystem_FindRider` (sound paths) walks a garbage rider
        // count read from low memory and runs away.
        let (world, ail, arr) = (DATA + 0x6_8000, DATA + 0x6_8100, DATA + 0x6_8200);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, ail);
        c.bus.write32(ail + 0xc, arr);
        c.bus.write32(arr + 8, 20);
        c.bus.write32(arr + 0x28, RIDER_2);
        c.bus.write32(arr + 0x7c, 1);
        r
    }

    /// `n` contact records at `base`: positions, normals (some repeated
    /// with their depth, for ties), depths, materials, surfaces, instances.
    pub(crate) fn put_contacts(c: &mut Cpu, r: &mut Rng, base: u32, n: u32) {
        let mut prev = ([0.0f32, 1.0, 0.0, 0.0], 0.5f32);
        for i in 0..n {
            let ct = base + 0x80 * i;
            put_v(c, ct, rand_v(r, -3.0, 3.0));
            let tie = r.next_u32() % 4 == 0;
            let normal = if tie {
                prev.0
            } else {
                [
                    uf_2(r, -1.0, 1.0),
                    uf_2(r, -0.2, 1.0),
                    uf_2(r, -1.0, 1.0),
                    0.0,
                ]
            };
            let depth = if tie {
                prev.1
            } else {
                match r.next_u32() % 3 {
                    0 => 0.5,
                    1 => 0.25,
                    _ => uf_2(r, 0.0, 1.0),
                }
            };
            prev = (normal, depth);
            put_v(c, ct + 0x10, normal);
            put_v(c, ct + 0x20, rand_v(r, -1.0, 1.0));
            c.bus.write32(ct + 0x40, f(depth));
            c.bus.write32(
                ct + 0x4c,
                if r.next_u32() % 4 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 8
                },
            );
            let surface = if r.next_u32() % 3 == 0 {
                0
            } else {
                SURFS + 0x100 * (r.next_u32() % 4)
            };
            c.bus.write32(ct + 0x50, surface);
            let inst = if r.next_u32() % 4 == 0 {
                0
            } else {
                INSTS + 0x200 * (r.next_u32() % 4)
            };
            c.bus.write32(ct + 0x54, inst);
            c.bus.write32(ct + 0x5c, r.next_u32() % 4);
            c.bus.write32(ct + 0x6c, f(uf_2(r, 0.0, 1.0)));
            c.bus.write32(ct + 0x70, f(uf_2(r, 0.0, 1.0)));
        }
    }

    /// The overlap list: no instances, no rails, `objects` static objects
    /// with huge boxes whose collider delegate adds 0-3 contacts.
    pub(crate) fn put_list(c: &mut Cpu, objects: u32) {
        c.bus.write32(LIST, LIST + 0x800);
        c.bus.write32(LIST + 0x10c, 0);
        c.bus.write32(LIST + 0x210, 0);
        c.bus.write32(LIST + 8, objects);
        for i in 0..objects {
            let o = OBJS + 0x100 * i;
            c.bus.write32(LIST + 0xc + 4 * i, o);
            c.bus.write32(o + 8, 0x20);
            let sub = o + 0x90;
            c.bus.write32(o + 0xc, sub);
            c.bus.write32(sub + 0xc, VT_SUB);
            c.bus.write32(o + 0x88, o + 0xc0);
            put_v(c, o + 0x60, [-1.0e6, -1.0e6, -1.0e6, 0.0]);
            put_v(c, o + 0x6c, [1.0e6, 1.0e6, 1.0e6, 0.0]);
        }
    }

    /// The rider: vectors, list, sphere tree (one child), states.
    pub(crate) fn put_rider(c: &mut Cpu, r: &mut Rng, main: u32, sub: u32) {
        c.bus.write32(RIDER_2 + 0x77c, BASE);
        c.bus.write32(RIDER_2 + 0x780, PARAMS);
        put_v(
            c,
            RIDER_2 + 0x370,
            [uf_2(r, -0.2, 0.2), 1.0, uf_2(r, -0.2, 0.2), 0.0],
        );
        put_v(c, RIDER_2 + 0x110, rand_v(r, -5.0, 5.0));
        put_v(c, RIDER_2 + 0x1e0, rand_v(r, -30.0, 30.0));
        put_v(c, RIDER_2 + 0x1b0, rand_v(r, -1.0, 1.0));
        put_v(c, RIDER_2 + 0x3b0, rand_v(r, -1.0, 1.0));
        put_v(c, RIDER_2 + 0x340, rand_v(r, -1.0, 1.0));
        c.bus.write32(RIDER_2 + 0x1f0, f(uf_2(r, 0.0, 20.0)));
        c.bus.write32(RIDER_2 + 0x350, f(uf_2(r, 0.0, 1.0)));
        c.bus.write32(PARAMS + 0x140, f(uf_2(r, 0.0, 1.0)));
        c.bus.write32(RIDER_2 + 0x860, LIST);
        c.bus.write32(RIDER_2 + 0x864, 0);
        c.bus.write32(RIDER_2 + 0x874, 0);
        c.bus.write32(RIDER_2 + 0xaa0, TREE);
        put_v(c, TREE + 0x10, rand_v(r, -1.0, 1.0));
        c.bus.write32(TREE + 0x20, f(uf_2(r, 0.5, 2.0)));
        c.bus.write32(TREE + 0x2c, 1);
        put_v(c, TREE + 0x30, rand_v(r, -1.0, 1.0));
        c.bus.write32(TREE + 0x40, f(0.5));
        c.bus.write32(BASE + 0xde0, main);
        c.bus.write32(BASE + 0xde4, sub);
        put_v(
            c,
            BASE + 0xb0,
            [uf_2(r, -0.3, 0.3), 1.0, uf_2(r, -0.3, 0.3), 0.0],
        );
        for k in 0..6 {
            c.bus.write32(RIDER_2 + 0x5b8 + 4 * k, r.next_u32() % 3);
        }
    }

    // -----------------------------------------------------------------
    // Sphere trees.

    pub(crate) fn put_sphere_tree(
        c: &mut Cpu,
        r: &mut Rng,
        t: u32,
        centre: [f32; 4],
        radius: f32,
        n: u32,
    ) {
        put_v(c, t + 0x10, centre);
        c.bus.write32(t + 0x20, f(radius));
        c.bus.write32(t + 0x2c, n);
        for i in 0..n {
            let ch = t + 0x20 * i;
            let off = rand_v(r, -1.5, 1.5);
            put_v(
                c,
                ch + 0x30,
                [
                    centre[0] + off[0],
                    centre[1] + off[1],
                    centre[2] + off[2],
                    0.0,
                ],
            );
            c.bus.write32(ch + 0x40, f(uf_2(r, 0.1, 0.8)));
        }
    }

    // -----------------------------------------------------------------
    // Walls.

    pub(crate) const HULL: u32 = DATA + 0x600;

    #[test]
    fn rider_find_wall_contact_matches() {
        check(
            ee::s00::f_00104e70__generated,
            rider_find_wall_contact,
            SEEDS,
            &|c, seed| {
                let mut r = setup_machine(c, seed, 0x4e70, 6);
                put_rider(c, &mut r, 0, 0);
                put_list(c, seed % 4);
                put_contacts(c, &mut r, STACK - 0x20a0, 10);
                c.bus.write32(HULL + 0x50, VT_HULL);
                gp_f32(c, -0x7f38, uf_2(&mut r, 0.0, 0.5));
                c.g[A0][0] = sx(RIDER_2);
                c.g[A1][0] = sx(HULL);
                c.g[A2][0] = sx(DATA + 0x700);
            },
        );
    }

    #[test]
    fn rider_handle_wall_collision_matches() {
        check(
            ee::s00::f_00105398__generated,
            rider_handle_wall_collision,
            SEEDS,
            &|c, seed| {
                let mut r = setup_machine(c, seed, 0x5398, 8);
                // Sub-state 9 skips; main state 4 (grinding) takes the board
                // normal and skips the rail contacts; 2 / 0 the ground tests.
                let sub = if seed % 9 == 0 { 9 } else { 0 };
                let main = [4u32, 0, 2, 1][(seed % 4) as usize];
                put_rider(c, &mut r, main, sub);
                c.bus.write32(BASE + 0x30, r.next_u32() % 2);
                put_list(c, if seed % 5 == 0 { 0 } else { 1 + seed % 3 });
                // Rider_FindWallContact's records (its frame inside ours).
                put_contacts(c, &mut r, STACK - 0x4a0 - 0x20a0, 10);
                gp_f32(c, -0x7f34, uf_2(&mut r, 0.0, 0.5));
                gp_f32(c, -0x7f38, uf_2(&mut r, 0.0, 0.5));
                let ground = DATA + 0x800;
                put_v(
                    c,
                    ground,
                    [uf_2(&mut r, -0.3, 0.3), 1.0, uf_2(&mut r, -0.3, 0.3), 0.0],
                );
                c.g[A0][0] = sx(RIDER_2);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(ground) };
            },
        );
    }

    // -----------------------------------------------------------------
    // Rails (reachable paths only: a rail contact needs the real rail
    // finders on real rail data).

    #[test]
    fn rider_handle_wall_rail_contact_matches() {
        check(
            ee::s00::f_00106f78,
            rider_handle_wall_rail_contact,
            SEEDS,
            &|c, seed| {
                let mut r = setup_machine(c, seed, 0x6f78, 2);
                put_rider(c, &mut r, if seed % 2 == 0 { 4 } else { 0 }, 0);
                put_list(c, 0);
                let bones = DATA + 0x5000;
                c.bus.write32(PARAMS + 0x2c, bones);
                c.bus.write32(RIDER_2 + 0x89c, r.next_u32() % 4);
                for i in 0..4 {
                    put_v(c, bones + 0x20 * i + 0x10, rand_v(&mut r, -1.0, 1.0));
                }
                c.g[A0][0] = sx(RIDER_2);
            },
        );
    }

    #[test]
    fn rider_try_start_railslide_matches() {
        check(
            ee::s00::f_00106848,
            rider_try_start_railslide,
            SEEDS,
            &|c, seed| {
                let mut r = setup_machine(c, seed, 0x6848, 2);
                put_rider(c, &mut r, seed % 3, 0);
                put_list(c, 0);
                let bones = DATA + 0x5000;
                c.bus.write32(PARAMS + 0x2c, bones);
                c.bus.write32(RIDER_2 + 0x8a0, r.next_u32() % 4);
                for i in 0..4 {
                    put_v(c, bones + 0x20 * i + 0x10, rand_v(&mut r, -1.0, 1.0));
                }
                c.g[A0][0] = sx(RIDER_2);
            },
        );
    }

    // -----------------------------------------------------------------
    // Rider against rider.

    pub(crate) const WORLD: u32 = DATA + 0x6_0000;

    pub(crate) const OTHERS: u32 = DATA + 0x4_0000;

    #[test]
    fn rider_check_rider_collisions_matches() {
        check(
            ee::s00::f_00107888__generated,
            rider_check_rider_collisions,
            SEEDS,
            &collisions_setup,
        );
    }

    /// `Rider_CheckRiderCollisions`' fixture: this rider (`RIDER_2`), six
    /// other riders (`OTHERS`) around it, pair timers a few frames old.
    pub(crate) fn collisions_setup(c: &mut Cpu, seed: u32) {
        {
            let mut r = setup_machine(c, seed, 0x7888, 12);
            put_rider(c, &mut r, seed % 3, 0);
            let (ail, arr) = (WORLD + 0x100, WORLD + 0x200);
            c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
            c.bus.write32(WORLD + 0x84, ail);
            c.bus.write32(ail + 0xc, arr);
            let frame = 10 + r.next_u32() % 50;
            c.bus.write32(arr + 8, frame);
            gp_f32(c, -0x7e80, uf_2(&mut r, 0.5, 2.0));
            gp_f32(c, -0x7e7c, uf_2(&mut r, 0.5, 2.0));
            gp_f32(c, -0x7e78, uf_2(&mut r, 0.0, 20.0));
            gp_f32(c, -0x7e74, uf_2(&mut r, 0.0, 1.0));
            gp_f32(c, -0x7e70, uf_2(&mut r, 0.0, 10.0));
            let me = r.next_u32() % 6;
            c.bus.write32(RIDER_2 + 0x86c, me);
            let centre = rand_v(&mut r, -3.0, 3.0);
            let (radius, n) = (uf_2(&mut r, 1.0, 3.0), 1 + r.next_u32() % 2);
            put_sphere_tree(c, &mut r, TREE, centre, radius, n);
            put_v(c, RIDER_2 + 0x110, centre);
            for i in 0..6u32 {
                // This rider's slot i: active, timers.
                let slot = RIDER_2 + 0x24 * i;
                c.bus.write32(slot, (r.next_u32() % 4 != 0) as u32);
                for t in [0x10u32, 0x14, 0x18] {
                    c.bus
                        .write32(slot + t, frame.wrapping_sub(r.next_u32() % 8));
                }
                // The other rider i.
                let o = OTHERS + 0x1000 * i;
                c.bus.write32(arr + 0x28 + 4 * i, o);
                c.bus.write32(o + 0x878, (r.next_u32() % 5 == 0) as u32);
                c.bus.write32(o + 0x86c, i);
                c.bus.write32(o + 0x77c, BASE);
                let tree = o + 0x800;
                c.bus.write32(o + 0xaa0, tree);
                let oc = rand_v(&mut r, -3.0, 3.0);
                let ocentre = [centre[0] + oc[0], centre[1] + oc[1], centre[2] + oc[2], 0.0];
                let (radius, n) = (uf_2(&mut r, 1.0, 3.0), 1 + r.next_u32() % 2);
                put_sphere_tree(c, &mut r, tree, ocentre, radius, n);
                put_v(c, o + 0x110, ocentre);
                put_v(c, o + 0x370, [0.0, 1.0, 0.0, 0.0]);
                put_v(c, o + 0x1e0, rand_v(&mut r, -30.0, 30.0));
                let pair = o + 0x24 * me;
                for t in [0x10u32, 0x14, 0x18] {
                    c.bus
                        .write32(pair + t, frame.wrapping_sub(r.next_u32() % 8));
                }
            }
            c.g[A0][0] = sx(RIDER_2);
        }
    }

    /// `Rider_CheckRiderCollisions`' bump and knock-down cooldowns are 3
    /// frames at 60 Hz (`slot+0x10`/`+0x18` < now - 3): the same 50 ms at
    /// every rate. With every pair timer `age` ticks old, the seeds that bump
    /// (re-stamp a pair timer with `now`) at 60 Hz with age 4 bump at 120 Hz
    /// only from age 7 on (6 ticks = 3 frames), and none bump at age 3 / 6.
    #[test]
    fn rider_check_rider_collisions_cooldowns_follow_the_rate() {
        use crate::tick;
        let bumps = |hz: u32, age: u32| -> Vec<u32> {
            let mut hit = Vec::new();
            for seed in 0..SEEDS {
                let mut c = crate::difftest::fresh();
                collisions_setup(&mut c, seed);
                tick::set_for_test(&mut c, hz);
                c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                let frame = c.bus.read32(WORLD + 0x200 + 8);
                let me = c.bus.read32(RIDER_2 + 0x86c);
                for i in 0..6u32 {
                    for slot in [RIDER_2 + 0x24 * i, OTHERS + 0x1000 * i + 0x24 * me] {
                        c.bus.write32(slot + 0x14, frame.wrapping_sub(1));
                        c.bus.write32(slot + 0x10, frame.wrapping_sub(age));
                        c.bus.write32(slot + 0x18, frame.wrapping_sub(age));
                    }
                }
                rider_check_rider_collisions(&mut c);
                if (0..6u32).any(|i| c.bus.read32(RIDER_2 + 0x24 * i + 0x10) == frame) {
                    hit.push(seed);
                }
            }
            hit
        };
        let at_60 = bumps(60, 4);
        assert!(!at_60.is_empty(), "no seed bumps at 60 Hz");
        assert!(
            bumps(60, 3).is_empty(),
            "60 Hz: bump inside the 3-frame cooldown"
        );
        assert!(
            bumps(120, 6).is_empty(),
            "120 Hz: bump inside the 6-tick cooldown"
        );
        assert_eq!(bumps(120, 7), at_60, "120 Hz: past the cooldown");
        assert!(
            bumps(240, 12).is_empty(),
            "240 Hz: bump inside the 12-tick cooldown"
        );
        assert_eq!(bumps(240, 13), at_60, "240 Hz: past the cooldown");
    }

    /// A scratch object after the rider fields.
    pub(crate) const OBJ: u32 = DATA + 0x4000;

    /// A vtable page.
    pub(crate) const VT: u32 = DATA + 0x8000;

    /// A float at `gp_3 + off` (the game's `.rodata` constants).
    pub(crate) fn wf(c: &mut Cpu, off: i32, v: f32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), v.to_bits());
    }

    pub(crate) fn w(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }

    /// A vtable slot `{ short delta; short; fn }` at `vt + off`.
    pub(crate) fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE);
    }

    /// The `Math_Atan` polynomial constants.
    pub(crate) fn atan_constants(c: &mut Cpu) {
        wf(c, 0xdf0, 0.167_702_27);
        wf(c, 0xdf4, 0.067_658_5);
        wf(c, 0xdf8, 0.052_005_03);
        wf(c, 0xdfc, 0.054_984_73);
        wf(c, -0x2e4c, 1.570_796_4);
        wf(c, -0x2e50, -1.570_796_4);
    }

    /// The heading-blend constants (`Rider_UpdateHeadingAndAudio`).
    pub(crate) fn heading_constants(c: &mut Cpu) {
        for (off, v) in [
            (-0x790c, -1.570_796_4f32),
            (-0x7908, 1.570_796_4),
            (-0x7904, std::f32::consts::PI),
            (-0x7900, std::f32::consts::PI),
            (-0x78fc, std::f32::consts::PI),
            (-0x78f8, std::f32::consts::TAU),
            (-0x78f4, -std::f32::consts::PI),
            (-0x78f0, std::f32::consts::TAU),
            (-0x78ec, 0.15),
        ] {
            wf(c, off, v);
        }
    }

    #[test]
    fn rider_apply_wind_force_matches() {
        check(
            ssx3_ee::s00::f_00125970__generated,
            rider_apply_wind_force,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x125970, 4);
                let rider = RIDER;
                c.g[A0][0] = rider as u64;
                // The wind direction table (512 sin/cos entries).
                let dirs = 0x0050_4fb8u32;
                for i in 0..0x300u32 {
                    let v = (i as f32 * 0.002).sin();
                    c.bus.write32(dirs + 4 * i, v.to_bits());
                }
                slot(c, VT, 0x20, 0);
                w(c, rider + rider::BASE, VT);
                let speed = match seed % 3 {
                    0 => 0.0f32,
                    1 => 1000.0,
                    _ => 1.0,
                };
                let angle = match seed % 5 {
                    0 => 0.0f32,
                    1 => 45.0,
                    2 => -170.0,
                    3 => 359.0,
                    _ => 12.5,
                };
                let fv = OBJ + 0x100;
                w(c, fv, speed.to_bits());
                w(c, fv + 4, angle.to_bits());
                w(c, fakes::LOG_RESULTS, fv);
                w(c, fakes::LOG_RESULTS + 4, fv + 4);
                w(
                    c,
                    rider + rider::IS_HUMAN,
                    if seed % 3 == 0 { 0 } else { 1 },
                );
                w(c, rider + rider::AI_INDEX, r.next_u32() % 4);
                // Some seeds have a velocity that already outruns the wind.
                let v = if seed % 4 == 3 {
                    1e30f32
                } else {
                    f32::from_bits(r.f32(50.0))
                };
                put_vec(c, rider + rider::VELOCITY, [v.to_bits(), v.to_bits(), 0, 0]);
                wf(c, -0x7878, 1388.888_9);
                wf(c, -0x7874, 27.777_779);
                wf(c, -0x7870, 277.777_77);
                wf(c, -0x786c, 0.017_453_293);
                wf(c, -0x7868, 81.487_33);
                wf(c, -0x7864, 0.015_000_001);
            },
        );
    }

    #[test]
    fn rider_update_heading_and_audio_matches() {
        check(
            ssx3_ee::s00::f_00121950__generated,
            rider_update_heading_and_audio,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x121950, 1);
                let rider = RIDER;
                c.g[A0][0] = rider as u64;
                heading_constants(c);
                atan_constants(c);
                // A created audio system (so its instance accessor is trivial).
                w(c, GAME_GP.wrapping_add(0x410), 0);
                // Skip the heavy audio-paint block of UpdateRiderWorldPaint.
                w(c, rider + 0x870, 2);
                w(c, rider + rider::HANDLER, 0);
                w(c, rider + 0x5b0, (seed % 2) as u32);
                w(c, rider + 0x86c, 0);
                // The ground normal: sometimes a zero z (the degenerate branch).
                let nx = f32::from_bits(r.f32(4.0));
                let ny = f32::from_bits(r.f32(4.0));
                let nz = if seed % 4 == 0 {
                    0.0
                } else {
                    f32::from_bits(r.f32(4.0))
                };
                put_vec(
                    c,
                    rider + rider::GROUND_NORMAL,
                    [nx.to_bits(), ny.to_bits(), nz.to_bits(), 0],
                );
                let heading = match seed % 4 {
                    0 => 0.0f32,
                    1 => 100.0,
                    2 => -100.0,
                    _ => f32::from_bits(r.f32(7.0)),
                };
                w(c, rider + rider::HEADING, heading.to_bits());
                wf(c, -0x78b4, 0.925_082);
                // A null world keeps the danger manager quiet.
                w(c, GAME_GP.wrapping_sub(0x848), 0);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn rider_update_visibility_matches() {
        check(
            ssx3_ee::s00::f_00122278__generated,
            rider_update_visibility,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x122278, 2);
                let rider = RIDER;
                c.g[A0][0] = rider as u64;
                // A draw context whose world painter is a fake vtable.
                let ctx = OBJ;
                let painter = VT;
                w(c, ctx + 0x10d8, painter);
                slot(c, painter, 0x158, 0);
                slot(c, painter, 0x2e8, 0);
                w(c, painter + 0x2ec, fakes::FAKE + 0x10);
                w(c, GAME_GP.wrapping_sub(0x854), ctx);
                // The render state: only kind 7 without a block word is tested.
                w(
                    c,
                    rider + rider::RENDER_FLAG,
                    if seed % 5 == 0 { 0 } else { 1 },
                );
                w(
                    c,
                    rider + rider::RENDER_KIND,
                    if seed % 5 == 2 { 5 } else { 7 },
                );
                w(
                    c,
                    rider + rider::VISIBILITY_BLOCK,
                    if seed % 5 == 3 { 1 } else { 0 },
                );
                w(
                    c,
                    rider + rider::ON_SCREEN,
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                w(c, rider + rider::UNK_150, if seed % 7 == 0 { 1 } else { 0 });
                w(c, rider + rider::HANDLER, OBJ + 0x2000);
                // Make the projection probes return "not visible" sometimes, so the
                // handler re-test and its branches run.
                w(c, fakes::LOG_RESULTS, if seed % 3 == 0 { 1 } else { 0 });
                w(c, fakes::LOG_RESULTS + 4, if seed % 2 == 0 { 1 } else { 0 });
                // A null world: the frustum query list is empty.
                w(c, GAME_GP.wrapping_sub(0x848), 0);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn rider_update_boost_timers_matches() {
        check(
            ssx3_ee::s00::f_001200d0__generated,
            rider_update_boost_timers,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1200d0, 1);
                let rider = RIDER;
                c.g[A0][0] = rider as u64;
                wf(c, -0x7958, 0.016_666_667);
                wf(c, -0x7954, 0.016_666_667);
                wf(c, -0x7950, 0.016_666_667);
                wf(c, -0x794c, 0.004_999_999_8);
                wf(c, -0x7948, 0.023_986_11);
                slot(c, VT, 0x48, 0);
                w(c, rider + rider::BASE, VT);
                // The rider handler holds the state words at +0xde0/+0xde4.
                w(c, rider + rider::HANDLER, OBJ + 0x1000);
                let (main, sub) = match seed % 4 {
                    0 => (1u32, 0u32),
                    1 => (0, 0xc),
                    2 => (0, (seed / 4) % 16),
                    _ => (1, 6),
                };
                w(c, OBJ + 0x1000 + 0xde0, main);
                w(c, OBJ + 0x1000 + 0xde4, sub);
                let dt = if seed % 2 == 0 { 1.0f32 } else { 0.05 };
                w(c, rider + rider::DELTA_TIME, dt.to_bits());
                w(c, rider + rider::BOOST_A, r.edge_f32());
                w(c, rider + rider::BOOST_B, r.edge_f32());
                w(
                    c,
                    rider + rider::BOOST,
                    if seed % 3 == 0 {
                        0
                    } else {
                        f32::from_bits(r.f32(30.0)).to_bits()
                    },
                );
                w(
                    c,
                    rider + rider::BOOST_PHASE,
                    match seed % 5 {
                        0 => 10,
                        1 => 5,
                        2 => 1,
                        3 => 2,
                        _ => 3,
                    },
                );
                w(c, rider + rider::BOOST_SCALE, r.edge_f32());
                w(c, rider + rider::BOOST_KIND, (seed / 5) % 5);
                w(c, GAME_GP.wrapping_sub(0x848), 0);
            },
        );
    }

    #[test]
    fn rider_check_race_finish_matches() {
        check(
            ssx3_ee::s00::f_00125228__generated,
            rider_check_race_finish,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x125228, 6);
                let rider = RIDER;
                c.g[A0][0] = rider as u64;
                // The rider base's "is human" slot 0x40.
                slot(c, VT, 0x40, 0);
                w(c, rider + rider::BASE, VT);
                wf(c, -0x787c, 0.016_666_667);
                // A created BE library object (never let it build one).
                w(c, GAME_GP.wrapping_sub(0x1f04), OBJ + 0x3000);
                w(c, GAME_GP.wrapping_sub(0x1efc), 0);
                w(c, 0x0053_08a8, 0);
                // The world / race manager chain.
                let world = OBJ + 0x2000;
                w(c, GAME_GP.wrapping_sub(0x848), world);
                w(c, world + 0x84, world + 0x100);
                w(c, world + 0xc0, OBJ + 0x2200);
                w(c, world + 0x100 + 0x214, 0);
                w(c, world + 0x100 + 0x8, r.next_u32());
                w(c, OBJ + 0x2200 + 0x88, if seed % 6 == 4 { 1 } else { 0 });
                w(c, OBJ + 0x2200 + 0x78, r.next_u32());
                let finish = match seed % 3 {
                    0 => 1.0f32,
                    1 => -1.0,
                    _ => -1000.0,
                };
                w(c, rider + rider::FINISH_TIME, finish.to_bits());
                w(
                    c,
                    rider + rider::DID_NOT_FINISH,
                    if seed % 4 == 0 { 1 } else { 0 },
                );
                w(c, rider + 0x87c, 0);
                w(c, rider + 0x150, 0);
                w(c, rider + rider::IS_HUMAN, 1);
                let _ = &mut r;
            },
        );
    }

    pub(crate) const AM_CTX: u32 = DATA + 0x25_0000;

    pub(crate) const AM_TOP: u32 = DATA + 0x26_0000;

    pub(crate) const MODEL_2: u32 = DATA + 0x40_0000;

    #[test]
    fn rider_render_model_matches() {
        check(
            ee::s00::f_00122448__generated,
            rider_render_model,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2448, 8);
                // The AM render context (`gp-0x854`) and its vtable.
                c.bus.write32(GAME_GP - 0x854, AM_CTX);
                c.bus.write32(AM_CTX + ctx::TEX_IDS, TEX);
                ctx_at(c, &mut r, AM_CTX, AM_TOP, &[0x228, 0xd8, 0x118, 0xe8]);
                // The model renderer's own context (`gp+0x2a90`).
                put_ctx(c, &mut r, &[0x378, 0x318]);
                let rider = BASE_2;
                let on_screen = (seed % 5 != 0) as u32;
                let hidden = (seed % 3 == 0) as u32;
                c.bus.write32(rider + rider::ON_SCREEN, on_screen);
                c.bus.write32(rider + rider::HIDDEN, hidden);
                c.bus.write32(
                    rider + rider::HANDLER,
                    if seed % 4 == 0 { 0 } else { rider + 0x3000 },
                );
                let q = rider + 0x3000 + 0xd30;
                c.bus.write32(q + 0xa0, (seed % 3 == 1) as u32);
                c.bus.write32(q + 0xa4, (seed % 6 == 1) as u32);
                c.bus.write32(rider + rider::SUB_STATE, (seed % 14) as u32);
                c.bus.write32(rider + rider::LOD, seed % 3);
                c.bus.write32(rider + 0x74, 7);
                c.bus.write32(rider + rider::MATRIX, rider + 0x4000);
                for i in 0..4 {
                    c.bus
                        .write32(rider + rider::POSITION + 4 * i, uf(&mut r, -100.0, 100.0));
                }
                // Rider vtable for Rider_ComputeLOD (`r+0x6c0`, slot 5).
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(rider + 0x6c0, vt);
                // A cheap model: no bones, no parts (the real RiderModel_Render
                // then only does its LOD-matrix call).
                let model = MODEL_2;
                c.bus.write32(rider + rider::MODEL, model);
                c.bus.write32(model + model::NUM_BONES, 0);
                c.bus.write32(model + model::NUM_PARTS, 0);
                c.bus.write32(model + model::HAS_PARTS, 0);
                c.bus.write32(model + model::LOD_INFO, r.next_u32());
                c.bus.write32(model + model::SCALE_Y, f(1.0));
                // The active rider handle: equal on most seeds.
                put_world(c, &mut r, if seed % 6 == 0 { 8 } else { 7 });
                c.g[A0][0] = sx(rider);
            },
        );
    }

    /// Point the world chain at dummies: `gp-0x848 -> +0x84` (race manager)
    /// `-> +0xc` (its object). Returns the world address.
    pub(crate) fn put_world_2(c: &mut Cpu, world: u32, man: u32, ctx: u32) {
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, man);
        c.bus.write32(man + 0xc, ctx);
    }

    #[test]
    fn rider_get_mass_matches() {
        check(
            ee::s00::f_0011ff98__generated,
            rider_get_mass,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11ff98, 5);
                // The back end's singleton exists (else it builds itself from
                // disc); its interface table starts empty, so the interface call
                // goes through a null vtable slot answered by the fake hook.
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x3000);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                let rider = DATA;
                c.bus.write32(rider + 0x86c, r.next_u32() % 4);
                c.bus.write32(
                    rider + 0xb34,
                    [0u32, 1, 0xffff_ffff, 5][(seed % 4) as usize],
                );
                c.bus.write32(rider + 0x2fc, r.f32(0.4));
                c.pc = 0x0011_ff98;
                c.g[A0][0] = sx(rider);
            },
        );
    }

    #[test]
    fn rider_update_leg_ik_blend_matches() {
        check(
            ee::s00::f_00120d90__generated,
            rider_update_leg_ik_blend,
            250,
            &|c, seed| {
                machine(c, seed, 0x120d90, 1);
                let rider = DATA;
                let anim = DATA + 0x200;
                let table = 0x0044_6990u32;
                let idx = (seed % 4) as u32;
                c.bus.write32(rider + 0x784, anim);
                // `AnimTable_GetEntry` indexes the table with the raw id word.
                c.bus
                    .write32(anim + 8, if seed % 8 == 7 { 0x1b6 } else { idx * 4 });
                if seed % 8 != 7 {
                    let id = if seed % 8 < 4 { 0x14 + seed % 7 } else { 0x30 };
                    c.bus.write32(table + idx * 4, id);
                }
                // +-1 makes the four approach paths (in/out of an air animation,
                // below/above the step) deterministic: both are reached.
                c.bus.write32(
                    rider + 0x318,
                    if seed % 2 == 0 {
                        0xbf80_0000
                    } else {
                        0x3f80_0000
                    },
                );
                // The five approach constants (`riderApproach`'s arguments).
                put_words(
                    c,
                    GAME_GP - 0x7934,
                    &[
                        0x3c23_d70a,
                        0x3f80_0000,
                        0x3dcc_cccd,
                        0x3c23_d70a,
                        0x3f80_0000,
                    ],
                );
                c.g[A0][0] = sx(rider);
            },
        );
    }

    #[test]
    fn rider_update_danger_state_matches() {
        check(
            ee::s00::f_00122cf0__generated,
            rider_update_danger_state,
            250,
            &|c, seed| {
                machine(c, seed, 0x122cf0, 5);
                let rider = DATA;
                let world = DATA + 0x100;
                let man = DATA + 0x200;
                let ctx = DATA + 0x300;
                let danger = DATA + 0x400;
                put_world_2(c, world, man, ctx);
                c.bus.write32(ctx + 0xa8, danger);
                let index = (seed % 4) as u32;
                c.bus.write32(rider + 0x86c, index);
                c.bus.write32(
                    danger + index + 0x710,
                    if seed % 3 == 0 { 0 } else { 1 + seed % 5 },
                );
                c.bus
                    .write32(rider + 0x5b0, [0u32, 1, 2][(seed % 3) as usize]);
                c.bus.write32(rider + 0x790, DATA + 0x800);
                let state = DATA + 0x500;
                c.bus.write32(rider + 0x77c, state);
                c.bus
                    .write32(state + 0xde0, [0u32, 1, 2][(seed % 3) as usize]);
                c.bus
                    .write32(state + 0x30, if seed % 4 == 2 { 1 } else { 0 });
                c.bus.write32(
                    rider + 0x5b4,
                    if seed % 2 == 0 {
                        0xbf80_0000
                    } else {
                        0x3f80_0000
                    },
                );
                put_words(c, GAME_GP - 0x78b4, &[0x3f80_0000]);
                c.g[A0][0] = sx(rider);
            },
        );
    }

    #[test]
    fn rider_is_spinning_fast_matches() {
        check(
            ee::s00::f_00122c98__generated,
            rider_is_spinning_fast,
            SEEDS_3,
            &|c, seed| {
                let mut r = Rng(0x0012_2c98 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let rider = DATA + 0x1000;
                c.g[A0][0] = rider as u64;
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x78b4), 0.925_081_97f32.to_bits());
                let rate: f32 = match seed % 6 {
                    0 => 2.0,  // at or past +threshold
                    1 => -2.0, // below -threshold
                    2 => 0.0,
                    3 => 0.5,
                    4 => 1.0,
                    _ => -0.5,
                };
                c.bus.write32(rider + 0x5b4, rate.to_bits());
            },
        );
    }

    #[test]
    fn rider_compute_lod_matches() {
        covered(
            ssx3_ee::s00::f_00121f30__generated,
            rider_compute_lod,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 17, 3);
                // D_004A289C and the global vtable (slots 35 and 29).
                let g = DATA + 0x200;
                c.bus.write32(GAME_GP - 0x854, g);
                let gvt = vtable(c, &mut r, &[0x118, 0xe8]);
                c.bus.write32(g + 0x10d8, gvt);
                // The rider and its own vtable (slot 5: the position).
                let rider = DATA + 0x300;
                let rvt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(rider + 0x6c0, rvt);
                c.bus.write32(rider + 0x898, r.next_u32());
                // The 0.01 scale and Math_SinCos' polynomial coefficients.
                put_words(c, GAME_GP - 0x78c4, &[0x3c23_d70a]);
                put_words(
                    c,
                    GAME_GP - 0x2ec0,
                    &[
                        0x3f22_f983,
                        0x3fc9_0fdb,
                        0x3638_ef1f,
                        0xb950_0d03,
                        0x3c08_8889,
                        0xbe2a_aaab,
                    ],
                );
                // A field of view of a plausible size half the time, random else.
                c.g[A0][0] = sx(rider);
            },
        );
    }

    #[test]
    fn rider_set_boost_enabled_matches() {
        check(
            ee::s00::f_00125958__generated,
            rider_set_boost_enabled,
            SEEDS_4,
            &|c, seed| {
                let mut r = rng(0x0012_5958, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    mixed(&mut r, seed, DATA + 0x100)
                };
                c.bus.write32(DATA + 0xb2c, r.next_u32());
                c.bus.write32(DATA + 0x2f0, r.next_u32());
                c.bus.write32(DATA + 0x2f4, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_refresh_overlaps_matches() {
        check(
            ee::s00::f_00120e50__generated,
            rider_refresh_overlaps,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x120e_50, 2);
                be_library(c);
                // No app / light manager: the light query sees an empty overlap
                // list and returns at once.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), 0);
                let rider = DATA + 0x400;
                c.g[A0][0] = sx(rider);
                c.bus.write32(rider + 0x860, 0);
            },
        );
    }

    #[test]
    fn rider_get_sync_checksum_matches() {
        check(
            ssx3_ee::s00::f_00125ad8__generated,
            rider_get_sync_checksum,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x07, 0);
                c.g[A0][0] = sx(DATA);
                let mut words = [0u32; 8];
                for (i, w) in words.iter_mut().enumerate() {
                    *w = if i % 2 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(100.0)
                    };
                }
                put_words(c, DATA + 0x110, &words);
            },
        );
    }

    #[test]
    fn rider_force_reset_matches() {
        check(
            ee::s00::f_001235f8__generated,
            rider_force_reset,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_35f8, 0);
                let rider = DATA + 0x1000;
                c.bus.write32(rider + 0x77c, DATA + 0x2000);
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Riders, camera and the front end.

    #[test]
    fn rider_create_focus_box_matches() {
        check(
            ee::s00::f_0011bb40__generated,
            rider_create_focus_box,
            SEEDS_6,
            &|c, seed| {
                let _ = machine(c, seed, 0x11bb_40, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let world = DATA + 0x2000;
                clear(c, this, 0x800);
                let module = dummy_world(c, world);
                let race = DATA + 0x3400;
                let focus = DATA + 0x4000;
                clear(c, focus, 0x100);
                c.bus.write32(module + 0xc, race);
                c.bus.write32(race + 0xa4, focus);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn rider_has_render_slot_matches() {
        check(
            ee::s00::f_0011c0e0__generated,
            rider_has_render_slot,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11c0_e0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let obj = DATA + 0x800;
                let vt = DATA + 0x1000;
                clear(c, this, 0x1000);
                clear(c, obj, 0x2000);
                clear(c, vt, 0x400);
                c.bus.write32(gp_at(-0x854), obj);
                c.bus.write32(obj + 0x10d8, vt);
                c.bus
                    .write32(vt + 0x390, (r.next_u32() as i16 as i32 & !3) as u32);
                c.bus.write32(vt + 0x394, FAKE);
                let tick = if seed % 3 == 0 {
                    u32::MAX
                } else {
                    r.next_u32()
                };
                c.bus.write32(this + 0x888, tick);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn player_pulse_rumble_matches() {
        check(
            ee::s00::f_001278d0__generated,
            player_pulse_rumble,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 21);
                c.g[A0][0] = AUX as u64;
                put(c, AUX + 0xdfc, r.f32(2.0));
                c.f[12] = if seed % 2 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(2.0)
                };
            },
        );
    }

    // -------------------------------------------------------------- riders

    #[test]
    fn rider_get_start_marker_pos_matches() {
        check(
            ssx3_ee::s00::f_00122c28__generated,
            rider_get_start_marker_pos,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x122c28, 0);
                w(c, OBJ2 + 0x86c, seed % 4);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }

    #[test]
    fn rider_add_velocity_matches() {
        check(
            ssx3_ee::s00::f_001250a8__generated,
            rider_add_velocity,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x1250a8, 0);
                let state = OBJ2 + 0x800;
                w(c, OBJ_2 + 0x77c, state);
                w(c, state + 0xde0, if seed % 2 == 0 { 1 } else { 2 });
                put_vec(c, OBJ_2 + 0x1e0, [0x3f80_0000, 0x4000_0000, 0, 0x3f80_0000]);
                put_vec(c, OBJ2, [1, 2, 3, 4]);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }

    #[test]
    fn rider_empty_progress_hook_matches() {
        leaf(
            ssx3_ee::s00::f_00125ad0__generated,
            rider_empty_progress_hook,
            2,
        );
    }

    #[test]
    fn rider_empty_stance_hook_matches() {
        leaf(
            ssx3_ee::s00::f_00116930__generated,
            rider_empty_stance_hook,
            13,
        );
    }

    #[test]
    fn rider_update_timed_event_anim_matches() {
        covered(
            ssx3_ee::s00::f_00115b58__generated,
            rider_update_timed_event_anim,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x1b58, 1);
                c.pc = 0x0011_5b58;
                let rider = DATA;
                let ctrl = DATA + 0x4000;
                let vt = fakes::vtable(c, &mut r, &[0x20]);
                c.g[A0][0] = rider as u64;
                c.bus.write32(rider + 0x784, ctrl);
                c.bus.write32(ctrl + 0x58, vt);
                c.bus.write32(ctrl + 4, 0x1b6);
                c.bus.write32(ctrl + 8, 0x1b6);
                for i in 0..8 {
                    c.bus.write32(fakes::LOG_RESULTS + 4 * i, 0x207);
                }
                // AI_GetFrameCounter: [[[gp-0x848]+0x84]+0xc]+8.
                let (a, b, d) = (DATA + 0x5000, DATA + 0x5100, DATA + 0x5200);
                c.bus.write32(GAME_GP - 0x848, a);
                c.bus.write32(a + 0x84, b);
                c.bus.write32(b + 0xc, d);
                c.bus.write32(d + 8, 100);
                c.bus.write32(rider + 0x354, 90);
                c.bus.write32(rider + 0x1fc, 0.5f32.to_bits());
                c.bus.write64(rider + 0x8c8, 0x1234_5678_9abc_def0);
                put_words(
                    c,
                    0x0045_6cc0,
                    &[
                        0x0011_5c08,
                        0x0011_5c54,
                        0x0011_5ca0,
                        0x0011_5cac,
                        0x0011_5ce0,
                        0x0011_5ce0,
                    ],
                );
                let mut event = 1;
                match seed % 13 {
                    0 => event = 0,
                    1 => {
                        // 500 frames since the event: too old.
                        c.bus.write32(d + 8, 500);
                        c.bus.write32(rider + 0x354, 0);
                    }
                    2 => {
                        // In the future: ignore.
                        c.bus.write32(d + 8, 10);
                        c.bus.write32(rider + 0x354, 100);
                    }
                    4 => event = 2,
                    5 => event = 3,
                    6 => event = 4,
                    7 => event = 5,
                    8 => {
                        // The landing arm, blend above 0.75.
                        event = 5;
                        c.bus.write32(rider + 0x1fc, 1.5f32.to_bits());
                    }
                    9 => event = 6,
                    10 => event = 7,
                    11 => event = 8,
                    12 => {
                        // The landing arm with the rider busy.
                        event = 5;
                        let state = DATA + 0x6000;
                        c.bus.write32(rider + 0x77c, state);
                        c.bus.write32(state + 0xde4, 1);
                    }
                    _ => {}
                }
                c.bus.write32(rider + 0x358, event);
            },
        );
    }

    /// `Rider_UpdateTimedEventAnim`'s "recent enough" window is 180 frames
    /// (3 s) of the AI's tick counter: the same wall-clock window at 120/240 Hz.
    #[test]
    fn rider_update_timed_event_anim_window_is_a_duration() {
        use crate::tick;
        let mut c = crate::difftest::fresh();
        for hz in [60u32, 120, 240] {
            tick::set_for_test(&mut c, hz);
            // The comparison the function makes: `now - last < 180 frames`.
            let limit = tick::ticks_from_frames60(&mut c, 180);
            assert_eq!(limit, 3 * hz, "{hz} Hz: 3 s of ticks");
            // A counter difference of 179 frames is inside the window, 180 is not.
            let inside = tick::ticks_from_frames60(&mut c, 179);
            assert!(inside < limit, "{hz} Hz");
        }
    }

    /// `Rider_UpdateIdleLookAnim`'s fixture (shared by the differential and
    /// the rate test): the rider at `DATA`, its controller at `DATA+0x4000`
    /// (layer 1 idle, `PlayAnim` reaching a fake), the AI frame counter
    /// `now`, the idle clock, the velocity and the `Rand_NextGame` state.
    pub(crate) fn idle_look_base(
        c: &mut Cpu,
        r: &mut Rng,
        now: u32,
        clock: f32,
        vel: [f32; 2],
    ) -> u32 {
        let rider = DATA;
        let ctrl = DATA + 0x4000;
        let vt = fakes::vtable(c, r, &[0x20]);
        c.pc = 0x0011_5d48;
        c.g[A0][0] = rider as u64;
        c.bus.write32(rider + 0x784, ctrl);
        c.bus.write32(ctrl + 0x58, vt);
        c.bus.write32(ctrl + 4, 0x1b6);
        c.bus.write32(ctrl + 8, 0x1b6);
        for i in 0..8 {
            c.bus.write32(fakes::LOG_RESULTS + 4 * i, 0x207);
        }
        // AI_GetFrameCounter: [[[gp-0x848]+0x84]+0xc]+8.
        let (a, b, d) = (DATA + 0x5000, DATA + 0x5100, DATA + 0x5200);
        c.bus.write32(GAME_GP - 0x848, a);
        c.bus.write32(a + 0x84, b);
        c.bus.write32(b + 0xc, d);
        c.bus.write32(d + 8, now);
        c.bus.write32(rider + 0x35c, clock.to_bits());
        c.bus.write32(rider + 0x1b0, vel[0].to_bits());
        c.bus.write32(rider + 0x1b4, vel[1].to_bits());
        c.bus.write32(rider + 0x320, r.next_u32() % 2);
        c.bus.write64(
            rider + 0x8c0,
            (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        );
        c.bus.write64(
            rider + 0x8d0,
            (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        );
        // Rand_NextGame's state (`0x4ff030`, six words).
        for i in 0..6u32 {
            c.bus.write32(0x004f_f030 + 4 * i, r.next_u32());
        }
        rider
    }

    #[test]
    fn rider_update_idle_look_anim_matches() {
        check_lohi(
            ee::s00::f_00115d48__generated,
            rider_update_idle_look_anim,
            400,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x5d48, 1);
                let now = 5000u32;
                let clock = match seed % 9 {
                    0 => 0.5f32,
                    1 => 0.99,
                    _ => 1.0 + (r.next_u32() % 100) as f32 * 0.01,
                };
                let f = |r: &mut Rng| (r.next_u32() % 2000) as f32 * 0.001 - 1.0;
                let vel = match seed % 7 {
                    0 => [0.0, 0.0],
                    1 => [0.0, 0.5],
                    2 => [0.0, -0.5],
                    3 => [-0.5, f(&mut r)],
                    _ => [f(&mut r), f(&mut r)],
                };
                let rider = idle_look_base(c, &mut r, now, clock, vel);
                if seed % 11 == 5 {
                    // Layer 1 busy (anim table entry 0x12 is non-zero).
                    c.bus.write32(DATA + 0x4000 + 4, 0x12);
                }
                for i in 0..6u32 {
                    let slot = rider + 0x24 * i;
                    c.bus.write32(slot, (r.next_u32() % 4 != 0) as u32);
                    c.bus.write32(slot + 0x1c, (r.next_u32() % 5 != 0) as u32);
                    let dist = if r.next_u32() % 4 == 0 {
                        1500.0f32
                    } else {
                        (r.next_u32() % 1000) as f32
                    };
                    c.bus.write32(slot + 0x8, dist.to_bits());
                    let angle = (r.next_u32() % 14000) as f32 * 0.001 - 7.0;
                    c.bus.write32(slot + 0xc, angle.to_bits());
                    let age = [0u32, 1, 599, 600, 601, 3000][(r.next_u32() % 6) as usize];
                    c.bus.write32(slot + 0x20, now - age);
                }
            },
        );
    }

    /// `Rider_UpdateIdleLookAnim` looks at the same rider again only after
    /// 600 frames (10 s): 600 ticks at 60 Hz, 1200 at 120, 2400 at 240.
    #[test]
    fn rider_update_idle_look_anim_cooldown_is_a_duration() {
        use crate::tick;
        let now = 5000u32;
        // True when the rider looked at rider 0 (90 degrees to the side,
        // looked at `age` ticks ago): its stamp becomes `now`.
        let looked = |k: u32, hz: u32, age: u32| -> bool {
            let mut c = crate::difftest::fresh();
            elf_image(&mut c);
            let mut r = machine(&mut c, k, 0x5d49, 0);
            tick::set_for_test(&mut c, hz);
            let rider = idle_look_base(&mut c, &mut r, now, 2.0, [1.0, 0.0]);
            for i in 0..6u32 {
                c.bus.write32(rider + 0x24 * i, 0);
            }
            c.bus.write32(rider, 1);
            c.bus.write32(rider + 0x1c, 1);
            c.bus.write32(rider + 0x8, 100.0f32.to_bits());
            c.bus
                .write32(rider + 0xc, std::f32::consts::FRAC_PI_2.to_bits());
            c.bus.write32(rider + 0x20, now - age);
            rider_update_idle_look_anim(&mut c);
            c.bus.read32(rider + 0x20) == now
        };
        // A Rand state whose roll takes the look branch (90%).
        let k = (0..64)
            .find(|&k| looked(k, 60, 600))
            .expect("no seed rolls the look branch");
        assert!(!looked(k, 60, 599), "60 Hz: inside 600 ticks");
        for hz in [120u32, 240] {
            let ticks = 10 * hz;
            assert!(!looked(k, hz, 600), "{hz} Hz: 600 ticks is not 10 s");
            assert!(!looked(k, hz, ticks - 1), "{hz} Hz: inside 10 s");
            assert!(looked(k, hz, ticks), "{hz} Hz: 10 s passed");
        }
    }

    /// `Rider_UpdateHeadingAndAudio` blends the heading 15% of the way to the
    /// ground normal per tick: a tenth of a second of ticks must cover the same
    /// fraction of the difference at every rate (the factor is
    /// `1 - 0.85^(60/rate)`), so the steering response keeps its timing.
    #[test]
    fn rider_update_heading_and_audio_follows_the_rate() {
        use crate::tick;
        let mut c = crate::difftest::fresh();
        let run = |c: &mut Cpu, hz: u32| -> f32 {
            tick::set_for_test(c, hz);
            let k = smoothing_60(c, FpuF32::from_f32(0.15));
            let mut heading = FpuF32::ZERO;
            // 1/10 s of ticks.
            for _ in 0..hz / 10 {
                heading = heading + (FpuF32::ONE - heading) * k;
            }
            heading.to_f32()
        };
        let at_60 = run(&mut c, 60);
        assert!((at_60 - 0.6228).abs() < 1e-3, "{at_60}");
        for hz in [120u32, 240] {
            let at = run(&mut c, hz);
            assert!(
                (at - at_60).abs() < 1e-4,
                "{hz} Hz: {at} after 0.1 s, 60 Hz: {at_60}"
            );
        }
    }

    /// `Rider_UpdateLegIKBlend` moves the blend 0.05 per tick: at 120/240 Hz a
    /// tick carries `60/rate` of that, so a blend takes the same wall-clock
    /// time to reach its target.
    #[test]
    fn rider_update_leg_ik_blend_follows_the_rate() {
        use crate::tick;
        let mut c = crate::difftest::fresh();
        let per_60 = |c: &mut Cpu, hz: u32| -> f32 {
            tick::set_for_test(c, hz);
            let step = rate_step_60(c, FpuF32::from_f32(0.05));
            step.to_f32() * (hz / 60) as f32
        };
        let at_60 = per_60(&mut c, 60);
        assert!((at_60 - 0.05).abs() < 1e-9, "{at_60}");
        for hz in [120u32, 240] {
            let at = per_60(&mut c, hz);
            assert!(
                (at - 0.05).abs() < 1e-9,
                "{hz} Hz: {} ticks move the blend {at}, not 0.05",
                hz / 60
            );
        }
    }

    #[test]
    fn rider_stop_transition_matches() {
        check(
            ssx3_ee::s00::f_00125038__generated,
            rider_stop_transition,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x125038, 0);
                let rider = DATA + 0x1000;
                let world = DATA + 0x3000;
                let ail = DATA + 0x4000;
                let cam = DATA + 0x5000;
                c.g[A0][0] = rider as u64;
                w32(c, GAME_GP.wrapping_sub(0x848), world);
                w32(c, world + 0x84, ail);
                w32(c, ail + 0x84, if seed % 5 == 0 { 0 } else { cam });
                let view = if seed % 3 == 0 {
                    0x8000_0000u32
                } else {
                    (seed % 4) as u32
                };
                w32(c, rider + 0x870, view);
                // func_00230698(ail, view) -> *(ail + view*4 + 0x5c)
                for i in 0..4u32 {
                    w32(c, ail + 0x5c + 4 * i, DATA + 0x6000 + i * 0x100);
                }
                for i in 0..4u32 {
                    let c2 = DATA + 0x6000 + i * 0x100;
                    w32(c, c2 + 0x78, if seed % 2 == 0 { 1 } else { 0 });
                    w32(c, c2 + 0x44, 0);
                }
                let _ = &mut r;
            },
        );
    }

    pub(crate) fn hh(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }

    pub(crate) fn fb(x: f32) -> u32 {
        x.to_bits()
    }

    /// A vtable slot_2 `{delta, fn}` at `vt + off` (the game's `lh`/`lw` pair).
    pub(crate) fn slot_2(c: &mut Cpu, vt: u32, off: u32, delta: u16, f: u32) {
        hh(c, vt + off, delta);
        w_2(c, vt + off + 4, f);
    }

    #[test]
    fn rider_mark_quit_requested_matches() {
        check(
            ee::s00::f_001253d0__generated,
            rider_mark_quit_requested,
            SEEDS_9,
            &|c, seed| {
                machine(c, seed, 0x12_53d0, 1);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                w_2(
                    c,
                    DATA + 0x2000 + 0x470,
                    if seed % 2 == 0 { fb(-1.0) } else { fb(0.5) },
                );
                w_2(c, DATA + 0x2000 + 0x6c0, DATA + 0x2400);
                slot_2(c, DATA + 0x2400, 0x38, 0, 0x0027_7f08);
                c.pc = 0x0012_53d0;
            },
        );
    }

    #[test]
    fn player_get_uber_grind_button_matches() {
        check(
            ee::s00::f_00127848__generated,
            player_get_uber_grind_button,
            SEEDS_10,
            &|c, seed| {
                let mut r = machine(c, seed, 0x127848, 4);
                be_library_2(c);
                let obj = DATA;
                let map = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                w_3(c, obj + 0xdf0, map);
                w_3(c, map, 0);
                w_3(c, map + 4, 0);
                w_3(c, map + 0x14, r.next_u32() % 4);
            },
        );
    }

    #[test]
    fn rider_update_from_script_matches() {
        check(
            ee::s00::f_00123da8__generated,
            rider_update_from_script,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x123da8, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A3][0] = (seed % 5) as u64;
                // No script state: the body returns after the prologue.
                c.bus.write32(DATA + 0xac4, 0);
            },
        );
    }

    #[test]
    fn rider_estimate_final_score_matches() {
        check(
            ee::s00::f_00122e50__generated,
            rider_estimate_final_score,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x122e50, 1);
                let rider = DATA + 0x1000;
                let metrix = DATA + 0x2000;
                c.g[A0][0] = rider as u64;
                w_4(c, rider + 0x790, metrix);
                w_4(c, metrix + 0x198, (seed % 32) as u32);
                w_4(c, rider + 0x4d8, r.edge_f32());
                w_4(c, rider + 0x4d0, r.edge_f32());
            },
        );
    }

    #[test]
    fn rider_place_at_marker_matches() {
        check(
            ee::s00::f_0011de60__generated,
            rider_place_at_marker,
            6,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x0011_de60, 4);
                c.g[A0][0] = OBJ_3 as u64;
                c.g[A1][0] = OBJ2_2 as u64;
                c.g[A2][0] = OBJ3 as u64;
                c.g[A3][0] = (seed % 4) as u64;
                w_5(c, OBJ_3 + 0x790, OBJ4);
                let t = DATA + 0x10000;
                for i in 0..8u32 {
                    w_5(c, t + 4 * i, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn rider_reset_motion_matches() {
        check(
            ee::s00::f_0011df18__generated,
            rider_reset_motion,
            60,
            &|c, seed| {
                let _r = base_3(c, seed, 0x11df);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                for i in 0..8 {
                    c.bus.write32(DATA + 0x1b0 + 4 * i, seed + i as u32);
                }
                c.bus.write32(DATA + 0x6c0, 0);
                c.bus.write32(DATA + 0x784, 0);
                c.bus.write32(DATA + 0x77c, 0);
            },
        );
    }

    pub(crate) fn clear_2(c: &mut Cpu, a: u32, n: u32) {
        for i in 0..n {
            c.bus.write32(a + 4 * i, 0);
        }
    }

    #[test]
    fn rider_estimate_finish_time_matches() {
        check(
            ee::s00::f_00122d78__generated,
            rider_estimate_finish_time,
            12,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x122d78, 2);
                let this = DATA;
                clear_2(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                let n = if seed % 3 == 0 { 0 } else { 60 };
                w_6(c, WORLD_4 + 0x84, WORLD_4 + 0x1000);
                w_6(c, WORLD_4 + 0x1000 + 0xc, WORLD_4 + 0x2000);
                w_6(c, WORLD_4 + 0x2000 + 0xc, n);
                wf_2(c, this + 0x4d0, if seed % 4 == 0 { 0.0 } else { 100.0 });
                let vt = vtable(c, &mut r, &[0x38]);
                w_6(c, this + 0x6c0, vt);
                wf_2(c, this + 0x520, 30.0);
            },
        );
    }

    /// The `30 - aiIndex` speed floor is per tick: a rider whose recent speed
    /// is under it gets the same estimated *time* to the finish at 60, 120 and
    /// 240 Hz (the 60 Hz path is the original).
    #[test]
    fn rider_estimate_finish_time_follows_the_rate() {
        use crate::tick;
        const R: u32 = 0x0080_0000;
        const WORLD: u32 = 0x0081_0000;
        const AI: u32 = 0x0081_1000;
        const RACE: u32 = 0x0081_2000;
        const VT: u32 = 0x0082_0000;
        const START: f32 = 3500.0; // start distance
        const DIST: f32 = 1000.0; // remaining
        let seconds = |hz: u32| -> (f64, f64) {
            let mut c = crate::difftest::fresh();
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = R as u64;
            // `world -> +0x84 -> +0xc`: the race clock in ticks.
            c.bus.write32(GAME_GP - 0x848, WORLD);
            c.bus.write32(WORLD + 0x84, AI);
            c.bus.write32(AI + 0xc, RACE);
            let frames = 2 * hz; // two seconds elapsed
            c.bus.write32(RACE + 0xc, frames);
            c.bus.write32(R + 0x4d8, START.to_bits());
            c.bus.write32(R + 0x4d0, DIST.to_bits());
            // The AI-index virtual at `r+0x6c0`: slot +0x3c indexed by +0x38
            // (`lh`), here zero, answered by a fake returning index 3.
            c.bus.write32(R + 0x6c0, VT);
            c.bus.write16(VT + 0x38, 0);
            c.bus.write32(VT + 0x3c, crate::difftest::fakes::FAKE);
            c.hooks.miss = crate::difftest::fakes::fake_call;
            c.bus
                .write32(crate::difftest::fakes::LOG_UNWIND_AT, u32::MAX);
            for i in 0..2u32 {
                c.bus
                    .write32(crate::difftest::fakes::LOG_RESULTS + 4 * i, 3);
            }
            tick::set_for_test(&mut c, hz);
            rider_estimate_finish_time(&mut c);
            let estimate = c.g[V0][0] as u32 as i32 as i64;
            (
                (estimate - frames as i64) as f64 / hz as f64,
                frames as f64 / hz as f64,
            )
        };
        let (at_60, t_60) = seconds(60);
        assert_eq!(t_60, 2.0);
        for hz in [120u32, 240] {
            let (at, t) = seconds(hz);
            assert_eq!(t, 2.0);
            assert!(
                (at - at_60).abs() < 1e-9,
                "{hz} Hz: {at} s left of the race, 60 Hz: {at_60} s"
            );
        }
    }

    #[test]
    fn ssx_audio_system_arcade_say20a8_matches() {
        strict(
            ee::s08::f_002a3c00__generated,
            ssx_audio_system_arcade_say20a8,
            150,
            &|c, seed| {
                let _ = base_5(c, seed, 0x2a3c, 6);
                let obj = DATA + 0x800;
                let v = seed % 6;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (if v == 1 { 0 } else { obj }) as u64;
                c.g[A2][0] = (seed % 4) as u64;
                w_7(
                    c,
                    obj + 0x470,
                    if v == 2 { 0x3f80_0000 } else { 0xbf80_0000 },
                );
                w_7(c, DATA + 0x635c, if v == 0 { 0 } else { 0x3f80_0000 });
                let busy = DATA + 0x5000;
                w_7(c, W_2 + 0x1000 + 0x28, if v == 5 { busy } else { 0 });
                w_7(c, busy, 5);
                let q = DATA + 0x5100;
                w_7(c, W_2 + 0x2000 + 0x28, if v == 3 { 0 } else { obj });
                w_7(c, W_2 + 0x2000 + 0x40, if v == 5 { q } else { 0 });
                w_7(c, q + 0x18, obj);
                w_7(c, GAME_GP + 0x173c, FAKE);
                if seed % 4 == 1 {
                    w_7(c, DATA + 0x5560 + 0x18, 1);
                    w_7(c, DATA + 0x5560 + 0x20, 0);
                    w_7(c, DATA + 0x5560 + 0x24, 0x20a8);
                }
            },
        );
    }

    #[test]
    fn quat_from_euler_matches() {
        strict_2(
            ee::s00::f_001241c0__generated,
            quat_from_euler,
            SEEDS_11,
            &|c, seed| {
                let mut r = Rng(0x0012_41c0 ^ (seed as u64) << 10);
                let _ = base_6(c, seed, 0x41c0, 0);
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
            },
        );
    }

    /// A valid, empty input map at `map` (`InputVM` stops on its first op).
    pub(crate) fn input_map(c: &mut Cpu, map: u32) {
        const INPUT_VM_TABLE: u32 = 0x0048_e330;
        const INPUT_VM_STOP: u32 = 0x0032_5ef8;
        const STOP_OPCODE: u32 = 0x20;
        w_9(c, map + 0x4, map + 0x80);
        w_9(c, map + 0x14, map + 0x100);
        for i in 0..4u32 {
            w_9(c, map + 0x80 + 4 * i, i);
            w_9(c, map + 0x100 + 4 * i, STOP_OPCODE);
        }
        w_9(c, INPUT_VM_TABLE + 4 * STOP_OPCODE, INPUT_VM_STOP);
    }

    /// An `InputBinding` at `binding` whose map is `map`.
    pub(crate) fn binding(c: &mut Cpu, binding: u32, map: u32, value: u32) {
        w_9(c, binding, value);
        w_9(c, binding + 4, map);
        input_map(c, map);
    }

    #[test]
    fn player_get_trick_button_matches() {
        check(
            ee::s00::f_001276f0__generated,
            player_get_trick_button,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x12_76f0, 15);
                c.g[A0][0] = DATA as u64;
                let b = DATA + 0x2000;
                binding(c, b, DATA + 0x3000, 0);
                w_9(c, DATA + 0xdf0, b);
                // The stopped input VM leaves the tested value in f0.
                let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
                w_9(c, SP - 0x1f8, value.to_bits());
            },
        );
    }
}
