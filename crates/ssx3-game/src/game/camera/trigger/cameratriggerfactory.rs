//! `src/camera/trigger/cameratriggerfactory.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::guest;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::abi::r32;
use crate::call::sp;
use crate::call::Unwound;
use crate::events::deliver_event;
use ssx3_ee_rt::funcs::call_indirect;
pub(crate) use crate::game::be::bedbreward::BUILTIN_DELETE_ADDR;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::camera::camera::camera_algorithm_ctor;
pub(crate) use crate::game::dirtysock::tags2::p_2 as p;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::dirtysock::tags2::const_reg;
pub(crate) use crate::game::render::particle::p as p_2;
pub(crate) use crate::game::bx::bxstring::gp;
pub(crate) use crate::game::ui::uivector::p as p_3;
pub(crate) use crate::game::camera::camera::gp_4 as gp_2;
pub(crate) use crate::game::camera::camera::write_tuning;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::wscript::wscriptman::p as p_5;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p as p_7;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_2;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::ai::computer::sd;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_2318, chase_camera_algorithm_ctor, "ChaseCameraAlgorithm_ctor"),
    (0x0017_14e8, camera_trigger_read_action_type1, "CameraTrigger_ReadActionType1"),
    (0x0017_15a0, camera_trigger_read_action_type0, "CameraTrigger_ReadActionType0"),
    (0x0017_15f8, camera_trigger_read_action_type2, "CameraTrigger_ReadActionType2"),
    (0x0017_1680, camera_trigger_read_action_type3, "CameraTrigger_ReadActionType3"),
    (0x0017_21c0, camera_sphere_trigger_get_bounds, "CameraSphereTrigger_GetBounds"),
    (0x0017_2278, camera_sphere_trigger_contains_point, "CameraSphereTrigger_ContainsPoint"),
    (0x0017_2840, camera_box_trigger_get_bounds, "CameraBoxTrigger_GetBounds"),
    (0x0017_31c0, vec3_max_component, "Vec3_MaxComponent"),
    (0x0017_3208, bounded_camera_algorithm_base_ctor, "BoundedCameraAlgorithmBase_ctor"),
    (0x0017_3678, bounded_camera_algorithm_base_reset, "BoundedCameraAlgorithmBase_Reset"),
    (0x0017_4190, bounded_camera_algorithm_ctor, "BoundedCameraAlgorithm_ctor"),
    (0x0017_47a0, bounded_camera_algorithm_reset, "BoundedCameraAlgorithm_Reset"),
    (0x0017_4848, camera_trigger_man_collect_octree_triggers_in_box, "CameraTriggerMan_CollectOctreeTriggersInBox"),
    (0x0017_6358, script_camera_algo_get_look_at, "ScriptCameraAlgo_GetLookAt"),
    (0x0017_63f8, algorithmic_script_camera_algo_dtor, "AlgorithmicScriptCameraAlgo_dtor"),
    (0x0017_6808, camera_point_bound_obj_clamp_point, "CameraPointBoundObj_ClampPoint"),
    (0x0017_6840, camera_trigger_array_45c1d0_v01, "CameraTriggerArray_45c1d0_v01"),
    (0x0017_6868, camera_trigger_array_45c1b8_v01, "CameraTriggerArray_45c1b8_v01"),
    (0x0017_6890, camera_trigger_man_collect_octree_node_triggers, "CameraTriggerMan_CollectOctreeNodeTriggers"),
    (0x0017_6a68, default_2_chase_camera_algorithm_ctor, "Default_2ChaseCameraAlgorithm_ctor"),
    (0x0017_6ad0, default_2_chase_camera_algorithm_get_eye_drop, "Default_2ChaseCameraAlgorithm_GetEyeDrop"),
    (0x0017_6ae0, default_2_chase_camera_algorithm_get_tuning, "Default_2ChaseCameraAlgorithm_GetTuning"),
    (0x0017_6ce0, default_2_chase_camera_reset_for_rider, "Default_2ChaseCameraAlgorithm_ResetForRider"),
    (0x0017_6d68, default_3_chase_camera_algorithm_ctor, "Default_3ChaseCameraAlgorithm_ctor"),
    (0x0017_6dd0, default_3_chase_camera_algorithm_get_eye_drop, "Default_3ChaseCameraAlgorithm_GetEyeDrop"),
    (0x0017_6de0, default_3_chase_camera_algorithm_get_tuning, "Default_3ChaseCameraAlgorithm_GetTuning"),
    (0x0017_6fe0, default_3_chase_camera_reset_for_rider, "Default_3ChaseCameraAlgorithm_ResetForRider"),
    (0x0017_7068, default_4_chase_camera_algorithm_ctor, "Default_4ChaseCameraAlgorithm_ctor"),
    (0x0017_70d0, default_4_chase_camera_algorithm_get_eye_drop, "Default_4ChaseCameraAlgorithm_GetEyeDrop"),
    (0x0017_70e0, default_4_chase_camera_algorithm_get_tuning, "Default_4ChaseCameraAlgorithm_GetTuning"),
    (0x0017_72e0, default_4_chase_camera_reset_for_rider, "Default_4ChaseCameraAlgorithm_ResetForRider"),
    (0x0017_74c0, direction_ne_chase_camera_algorithm_ctor, "DirectionNeChaseCameraAlgorithm_ctor"),
    (0x0017_7558, direction_ne_chase_camera_algorithm_get_tuning, "DirectionNeChaseCameraAlgorithm_GetTuning"),
    (0x0017_7588, direction_ne_chase_camera_reset_for_rider, "DirectionNeChaseCameraAlgorithm_ResetForRider"),
    (0x0017_7e50, spoke_chase_camera_algorithm_ctor, "SpokeChaseCameraAlgorithm_ctor"),
    (0x0017_7eb8, spoke_chase_camera_algorithm_get_eye_drop, "SpokeChaseCameraAlgorithm_GetEyeDrop"),
    (0x0017_7ec8, spoke_chase_camera_get_tuning, "SpokeChaseCameraAlgorithm_GetTuning"),
    (0x0017_7ef8, spoke_chase_camera_algorithm_get_focus_point, "SpokeChaseCameraAlgorithm_GetFocusPoint"),
    (0x0017_8520, superpipe_chase_camera_algorithm_ctor, "SuperpipeChaseCameraAlgorithm_ctor"),
    (0x0017_85b0, superpipe_chase_camera_get_eye_drop, "SuperpipeChaseCameraAlgorithm_GetEyeDrop"),
    (0x0017_85c0, superpipe_chase_camera_get_tuning, "SuperpipeChaseCameraAlgorithm_GetTuning"),
    (0x0017_8938, superpipe_chase_camera_algorithm_reset_for_rider, "SuperpipeChaseCameraAlgorithm_ResetForRider"),
    (0x0017_8bb0, post_race_1_chase_camera_update, "Post_Race_1ChaseCameraAlgorithm_Update"),
    (0x0017_8e90, post_race_1_chase_camera_reset_for_rider, "Post_Race_1ChaseCameraAlgorithm_ResetForRider"),
    (0x0017_9fc8, debug_menu_is_track_selectable, "DebugMenu_IsTrackSelectable"),
    (0x0017_a028, debug_menu_next_enabled_item, "DebugMenu_NextEnabledItem"),
    (0x0017_a0b8, debug_menu_prev_enabled_item, "DebugMenu_PrevEnabledItem"),
    (0x0017_a220, debug_menu_find_next_item, "DebugMenu_FindNextItem"),
    (0x0017_a2c8, debug_menu_find_prev_item, "DebugMenu_FindPrevItem"),
    (0x0017_abc8, debug_menu_init_background_list, "DebugMenu_InitBackgroundList"),
    (0x0017_abe0, debug_menu_free_backgrounds, "DebugMenu_FreeBackgrounds"),
    (0x0017_ac98, debug_menu_select_background, "DebugMenu_SelectBackground"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_MANAGER_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EVENT_SELECT_ON_WIDGET_NOTIFY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_ALLOC_NAMED_ALIGNED;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_SEND_LOCK_CHAL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SHARED_LOGIN_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_VOIP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOAD_INDICATOR_CLEAR_BUSY;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_SPAWN_MISSIONS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_READ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_GET_WORLD_SECTION_INDEX;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::STRLEN;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_START_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2 as BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_7 as SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_GET_ACTIVE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2 as MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT as SCRIPT_ENGINE_CHOOSE_SCRIPT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2 as SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5 as EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_5 as EA_MEM_FREE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_BACKUP_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_FRAME_DUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_DECODE_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_AUDIO_DONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4 as UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3 as UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2 as UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_ALL_TIMERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2 as MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2 as APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_4 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_LIST_GET_AT_2 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED_4;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH as STR_HASH_3;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_CTOR_COPY;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_DTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::CHUNK_RESOLVE_RELOCATE_DMA;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_FLOAT_3 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT as QSORT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN as UI_STATE_SHUTDOWN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_FREE as EA_MEM_FREE_3;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN as USTR_LEN_3;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY as USTR_CPY_3;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2 as PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2 as PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE as BE_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE0;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE1;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL as UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_CAMERA_ALGORITHM_CTOR as RIDER_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;


    pub const MATH_SIN_COS: Callee = Callee { addr: 0x0031_be50, f: ee::s10::Math_SinCos };


    pub const RESET_FOR_RIDER: Callee = Callee {
        addr: 0x0016_6c60,
        f: ee::s02::f_00166c60,
    };

    pub const INIT_FROM_RIDER: Callee = Callee {
        addr: 0x0016_6550,
        f: ee::s02::f_00166550,
    };

    pub const DYNAMIC_EMITTER_RESET: Callee = Callee {
        addr: 0x0037_0d60,
        f: ee::s12::f_00370d60,
    };

    pub const MC_END_OPERATION: Callee = Callee {
        addr: 0x0023_ea30,
        f: ee::s06::f_0023ea30,
    };

    pub const MC_ON_OPERATION_DONE: Callee = Callee {
        addr: 0x0023_ea90,
        f: ee::s06::f_0023ea90,
    };

    pub const BUY_POPUP_RESET: Callee = Callee {
        addr: 0x001c_a8a0,
        f: ee::s04::f_001ca8a0,
    };

    pub const STOP_LOADING_SCREEN_LOOP: Callee = Callee {
        addr: 0x0028_fa98,
        f: ee::s07::f_0028fa98,
    };

    pub const CLEAR_NEW_REWARDS: Callee = Callee {
        addr: 0x0015_a510,
        f: ee::s01::f_0015a510,
    };

    pub const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::f_0011fe98,
    };

    pub const RIDER_RECORD_BEST_JUMP: Callee = Callee {
        addr: 0x0011_9448,
        f: ee::s00::f_00119448,
    };

    pub const RIDER_ADD_BOOST_2: Callee = Callee {
        addr: 0x0010_e098,
        f: ee::s00::f_0010e098,
    };

    pub const RIDER_APPLY_SCRIPT_TRACK: Callee = Callee {
        addr: 0x0012_4788,
        f: ee::s00::f_00124788,
    };

    pub const RIDER_UPDATE_SCRIPT_SOUNDS: Callee = Callee {
        addr: 0x0012_5448,
        f: ee::s00::f_00125448,
    };

    pub const SNOW_BOX_DTOR: Callee = Callee {
        addr: 0x002e_4f28,
        f: ee::s09::f_002e4f28,
    };

    pub const UI_ANIM_CTOR: Callee = Callee {
        addr: 0x0039_72a0,
        f: ee::s13::f_003972a0,
    };

    pub const VRAM_HEAP_INIT: Callee = Callee {
        addr: 0x0036_5e40,
        f: ee::s12::f_00365e40,
    };

    pub const TEX_MAN_INIT_TABLE: Callee = Callee {
        addr: 0x0036_70e0,
        f: ee::s12::f_003670e0,
    };

    pub const TAG_FIELD_COPY: Callee = Callee {
        addr: 0x003e_b198,
        f: ee::s14::f_003eb198,
    };

    pub const TAG_FIELD_SET_RAW: Callee = Callee {
        addr: 0x003e_b1e0,
        f: ee::s14::f_003eb1e0,
    };

    pub const GROW_LINES: Callee = Callee {
        addr: 0x003a_1958,
        f: ee::s13::f_003a1958,
    };

    pub const UI_TEXT_SET_EVENT_STAT: Callee = Callee {
        addr: 0x001c_e9e0,
        f: ee::s04::f_001ce9e0,
    };

    pub const UI_STATE_CTOR_2_2: Callee = Callee {
        addr: 0x0039_e318,
        f: ee::s13::f_0039e318,
    };

    pub const LOAD_INDICATOR_RENDER: Callee = Callee {
        addr: 0x0023_1fc0,
        f: ee::s06::f_00231fc0,
    };

    pub const UI_OBJECT_FIND_BY_ID: Callee = Callee {
        addr: 0x0039_ff60,
        f: ee::s13::f_0039ff60,
    };

    pub const UI_LIST_FIRST: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::f_00397840,
    };

    pub const UI_LIST_NODE_SENTINEL: Callee = Callee {
        addr: 0x0039_7718,
        f: ee::s13::f_00397718,
    };

    pub const SECTION_SELECT_PEAK_LIGHTING: Callee = Callee {
        addr: 0x0022_e180,
        f: ee::s05::f_0022e180,
    };

    pub const INSTANCE_GET_NODE_MATRIX: Callee = Callee {
        addr: 0x0034_fed8,
        f: ee::s11::f_0034fed8,
    };

    pub const AVALANCHE_GET_OBJECT_MATRIX: Callee = Callee {
        addr: 0x002d_9c00,
        f: ee::s09::f_002d9c00,
    };

    pub const GAME_PROCESS_DTOR: Callee = Callee {
        addr: 0x0035_46c8,
        f: ee::s11::f_003546c8,
    };

    pub const WORLD_GET_INSTANCE: Callee = Callee {
        addr: 0x002d_1bd8,
        f: ee::s09::f_002d1bd8,
    };

    pub const BE_GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::f_0014a0e0,
    };

    pub const BE_GET_RIDER_CHARACTER: Callee = Callee {
        addr: 0x0014_a080,
        f: ee::s01::f_0014a080,
    };

    pub const BE_GET_TRACK_INFO: Callee = Callee {
        addr: 0x0014_4bc0,
        f: ee::s01::f_00144bc0,
    };

    pub const BE_SET_CHARACTER_BY_SLOT: Callee = Callee {
        addr: 0x0014_7448,
        f: ee::s01::f_00147448,
    };

    pub const LOBBY_GET_ROOM_NAME: Callee = Callee {
        addr: 0x0025_f150,
        f: ee::s06::f_0025f150,
    };

    pub const BUDDY_GET_ROSTER_COUNT: Callee = Callee {
        addr: 0x0026_2598,
        f: ee::s07::f_00262598,
    };

    pub const BUDDY_GET_ROSTER_ENTRY: Callee = Callee {
        addr: 0x0026_25c8,
        f: ee::s07::f_002625c8,
    };

    pub const FE_ASYNC_SET_STATE: Callee = Callee {
        addr: 0x001a_39d8,
        f: ee::s03::f_001a39d8,
    };

    pub const POPUP_CONFIRM_REBUILD: Callee = Callee {
        addr: 0x001c_5dd8,
        f: ee::s03::f_001c5dd8,
    };

    pub const SSX_AUDIO_PLAY_MUSIC: Callee = Callee {
        addr: 0x0028_cf98,
        f: ee::s07::f_0028cf98,
    };


    // Cross-shard calls the generator emitted through the function table.
    pub const STR_HASH: u32 = 0x0031_7670;

    pub const STREAM_FILE_QUEUE_RANGE: u32 = 0x003e_0c88;

    pub const USTR_LEN: u32 = 0x003d_bca0;

    pub const USTR_CPY: u32 = 0x003d_be20;

    pub const UI_TEXT_SET_TEXT_ID: u32 = 0x003a_1000;

    pub const UI_TEXT_SET_STRING: u32 = 0x003a_0c50;

    pub const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;

    pub const UI_ANIM_SET_CREATE_ANIM: u32 = 0x0039_7140;

    pub const UI_OBJECT_SET_ANIM: u32 = 0x0039_fcc8;

    pub const UI_STATE_MGR_PUSH_POPUP: u32 = 0x0039_f290;

    pub const UI_LISTBOX_SET_ITEM_TEXT: u32 = 0x0039_a7f0;

    pub const SET_ITEM_YEAR: u32 = 0x0039_a7a8;

    pub const UI_TEXT_SET_ASCII: u32 = 0x003a_0c70;

    pub const UI_ANIM_SET_CREATE_ANIM_2: u32 = 0x0039_7168;

    pub const BE_MISSION_SET_COLLECTED: u32 = 0x0015_3b00;

    pub const MSG_PRINTF: u32 = 0x0041_7828;

    pub const STR_LEN: u32 = 0x0041_6810;

    pub const QSORT: u32 = 0x0041_8ef8;

    pub const EA_TIMER_SERVICE: u32 = 0x003e_5928;

    pub const EA_THREAD_YIELD: u32 = 0x003e_5398;

    pub const GAME_MODULE_REQUEST_STATE_2: u32 = 0x0023_1250;

    pub const LOBBY_API_SEND: u32 = 0x003e_8b10;

    pub const TAG_FIELD_SET_STRING: u32 = 0x003e_c180;

    pub const SSX_AUDIO_RESUME_SFX: u32 = 0x0029_ce70;

    pub const SSX_AUDIO_IS_TRACK_GROUP5: u32 = 0x002a_3f90;

    pub const RIDER_GATHER_CAM_STATE: u32 = 0x0027_c9b0;

    pub const SSX_AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

    pub const MEM_REPORT_STATS_LOCKED: u32 = 0x0031_9cc8;

    pub const OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;

    pub const EA_MEM_ALLOC_NAMED_2: u32 = 0x0031_7d70;

    pub const UI_LIST_FIRST_WRAPPER: u32 = 0x0039_fae8;

    pub const SERIALIZE_READ_U32: u32 = 0x002d_2510;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
}


/// `int CameraTrigger_ReadActionType3(CameraTrigger *self, int *out)`
/// (0x171680): store the action type 3 and return 0.
pub fn camera_trigger_read_action_type3(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x3;
    c.g[V0][0] = 0;
    Addr(abi::arg(c, 1)).sw(c, 0x3);
    abi::ret(c);
}


/// `float Vec3_MaxComponent(const Vec3 *v)` (0x1731c0): the largest of the
/// three components.
pub fn vec3_max_component(c: &mut Cpu) {
    let v = Addr(abi::arg(c, 0));
    c.cycles += 6;
    let f2 = v.read_fpu(c);
    c.f[2] = f2.0;
    let f1 = v.off(0x4).read_fpu(c);
    c.f[1] = f1.0;
    let y_lt_x = f1.lt(f2);
    abi::set_fcc(c, y_lt_x);
    let f0 = v.off(0x8).read_fpu(c);
    c.f[0] = f0.0;
    if !y_lt_x {
        c.cycles += 4;
        let z_lt_y = f0.lt(f1);
        abi::set_fcc(c, z_lt_y);
        if z_lt_y {
            c.f[0] = f1.0;
        }
    } else {
        c.cycles += 3;
        let z_lt_x = f0.lt(f2);
        abi::set_fcc(c, z_lt_x);
        if z_lt_x {
            c.cycles += 2;
            c.f[0] = f2.0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `DebugMenuBackgroundList* DebugMenu_InitBackgroundList(DebugMenuBackgroundList *self)`
/// (0x17abc8): clear the list and set its cursor to -1.
pub fn debug_menu_init_background_list(c: &mut Cpu) {
    c.cycles += 6;
    let a0 = abi::arg64(c, 0);
    let this = Addr(a0 as u32);
    c.g[V0][0] = u64::MAX;
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, u64::MAX);
    c.g[V0][0] = a0;
    this.sw(c, 0);
    abi::ret(c);
}


/// `int DebugMenu_SelectBackground(DebugMenuBackgroundList *self, int index)`
/// (0x17ac98): store `index` at +0x8 when it is in `[0, count)`; otherwise
/// store -1. Returns 1.
pub fn debug_menu_select_background(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    let index = c.g[A1][0];
    c.cycles += 2;
    let mut select = true;
    let mut stored = u64::MAX;
    'outer: {
        if (index as i64) < 0 {
            stored = u64::MAX;
            break 'outer;
        }
        c.cycles += 4;
        let limit = this.off(0x4).lw(c);
        c.g[V0][0] = limit;
        let in_range = (index as i64) < (limit as i64);
        c.g[V0][0] = in_range as u64;
        if !in_range {
            stored = u64::MAX;
            break 'outer;
        }
        c.cycles += 2;
        this.off(0x8).sw(c, index);
        select = false;
    }
    if select {
        c.cycles += 1;
        c.g[V0][0] = stored;
        this.off(0x8).sw(c, stored);
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}


/// `void AlgorithmicScriptCameraAlgo_dtor(AlgorithmicScriptCameraAlgo *this, uint flags)`
/// (0x1763f8): set the vtable at +0x10 to 0x45bde0 and, when `flags & 1`,
/// delete `this`.
pub fn algorithmic_script_camera_algo_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x46_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x46_0000u32.wrapping_sub(0x4220));
        c.g[A1][0] &= 1;
        let delete_this = c.g[A1][0] != 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        if delete_this {
            c.cycles += 2;
            call_ptr(c, BUILTIN_DELETE_ADDR, 0x0017_641c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// VU0 matrix helpers, inlined in the two camera functions below. Every one of
// them issues the original's COP2 macro operations (same destination and
// source registers, same order), because the VU0 register file is still
// observable when the function returns.

/// `vu0IdentityMtx` / `sqc2` ×4: the quadword at `src` copied to `dst`.
pub(crate) fn copy_matrix(c: &mut Cpu, dst: u32, src: u32) {
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
}


/// `vu0MulMtx` (`dst = m * n`, four rows at `dst + 0,0x10,0x20,0x30`). `dst`
/// may alias `m` or `n`: every row is loaded first.
pub(crate) fn mul_matrix(c: &mut Cpu, dst: u32, m: u32, n: u32) {
    c.lqc2(4, m);
    c.lqc2(5, m.wrapping_add(0x10));
    c.lqc2(6, m.wrapping_add(0x20));
    c.lqc2(7, m.wrapping_add(0x30));
    c.lqc2(8, n);
    c.lqc2(9, n.wrapping_add(0x10));
    c.lqc2(10, n.wrapping_add(0x20));
    c.lqc2(11, n.wrapping_add(0x30));
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
}


/// `vu0MulVec` (`dst = m * v`, one lane per row): the four dot products land
/// in vf12's four lanes.
pub(crate) fn mul_matrix_vec(c: &mut Cpu, dst: u32, m: u32, v: u32) {
    c.lqc2(4, m);
    c.lqc2(5, m.wrapping_add(0x10));
    c.lqc2(6, m.wrapping_add(0x20));
    c.lqc2(7, m.wrapping_add(0x30));
    c.lqc2(8, v);
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, dst);
}


/// `vu0RotAxis(&m, axis, angle)`: `Math_SinCos(angle, &sin, &cos)` (the two
/// result words at `out_a` and `out_b`, in that argument order; `negate` is
/// set where the original passes the negated angle), then the Rodrigues
/// rotation matrix in the four rows at `work` (with the `1.0` in f20),
/// multiplied into `m` at `sp`.
///
/// `work` is `sp + 0x40` or `sp + 0x50` depending on the call site, exactly as
/// the original lays the temporary matrix out.
#[allow(clippy::too_many_arguments)]
pub(crate) fn rot_axis(
    c: &mut Cpu,
    negate: bool,
    work: u32,
    model: u32,
    angle_field: u32,
    axis: u32,
    out_a: u32,
    out_b: u32,
    ret: u32,
) -> Flow {
    let sp = c.g[SP][0] as u32;
    c.lwc1(12, model.wrapping_add(angle_field));
    c.g[S4][0] = sx(sp);
    c.g[A0][0] = sx(out_a);
    c.g[A1][0] = sx(out_b);
    if negate {
        c.neg_s(12, 12);
    }
    c.g[S0][0] = 0x500000;
    c.g[S3][0] = sx(work);
    c.g[S1][0] = sx(axis);
    callee::MATH_SIN_COS.call(c, ret)?;

    // Rodrigues from (cos, sin) and the axis `(x, y, z)`.
    c.lwc1(11, out_b);
    c.lwc1(3, axis.wrapping_add(0x8));
    c.sub_s(1, 20, 11);
    c.lwc1(0, axis);
    c.lwc1(6, axis.wrapping_add(0x4));
    c.lwc1(8, out_a);
    c.mul_s(2, 1, 3);
    c.sw(work.wrapping_add(0x30), 0);
    c.mul_s(4, 1, 0);
    c.sw(work.wrapping_add(0x34), 0);
    c.mul_s(1, 1, 6);
    c.sw(work.wrapping_add(0x38), 0);
    c.mul_s(13, 8, 3);
    c.sw(work.wrapping_add(0xc), 0);
    c.mul_s(12, 8, 0);
    c.sw(work.wrapping_add(0x1c), 0);
    c.mul_s(9, 2, 3);
    c.sw(work.wrapping_add(0x2c), 0);
    c.mul_s(10, 4, 3);
    c.swc1(20, work.wrapping_add(0x3c));
    c.mul_s(5, 2, 0);
    c.mul_s(7, 4, 0);
    c.mul_s(3, 1, 3);
    c.mul_s(0, 1, 0);
    c.mul_s(8, 8, 6);
    c.mul_s(2, 2, 6);
    c.mul_s(4, 4, 6);
    c.mul_s(1, 1, 6);
    c.add_s(5, 5, 8);
    c.sub_s(0, 0, 13);
    c.sub_s(2, 2, 12);
    c.add_s(9, 9, 11);
    c.swc1(5, work.wrapping_add(0x8));
    c.add_s(7, 7, 11);
    c.swc1(0, work.wrapping_add(0x4));
    c.add_s(4, 4, 13);
    c.swc1(2, work.wrapping_add(0x18));
    c.sub_s(10, 10, 8);
    c.swc1(9, work.wrapping_add(0x28));
    c.add_s(1, 1, 11);
    c.swc1(7, work);
    c.add_s(3, 3, 12);
    c.swc1(4, work.wrapping_add(0x10));
    c.swc1(10, work.wrapping_add(0x20));
    c.swc1(1, work.wrapping_add(0x14));
    c.swc1(3, work.wrapping_add(0x24));
    mul_matrix(c, sp, sp, work);
    Ok(())
}


// ---------------------------------------------------------------------------
// Camera sphere trigger.

/// `bool CameraSphereTrigger_ContainsPoint(CameraSphereTrigger* self, Vec4*
/// p)` (0x172278): un-scales and un-rotates `p - self->pos` by the trigger's
/// radii and angles and returns whether the result lies in the unit sphere
/// (`x^2 + y^2 + z^2 <= 1`).
pub fn camera_sphere_trigger_contains_point(c: &mut Cpu) {
    guest(c, camera_sphere_trigger_contains_point_body)
}


pub(crate) fn camera_sphere_trigger_contains_point_body(c: &mut Cpu) -> Flow {
    c.cycles += 357;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
    let sp = c.g[SP][0] as u32;
    let trigger = abi::arg(c, 0);
    let point = abi::arg(c, 1);

    c.g[V0][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    let identity = c.g[V0][0] as u32;
    c.sq(18, sp.wrapping_add(0x130));
    c.sq(21, sp.wrapping_add(0x100));
    c.sq(16, sp.wrapping_add(0x150));
    c.sq(17, sp.wrapping_add(0x140));
    c.sq(19, sp.wrapping_add(0x120));
    c.sq(20, sp.wrapping_add(0x110));
    c.sd(sp.wrapping_add(0xf0), c.g[RA][0]);
    c.swc1(20, sp.wrapping_add(0x160));

    // m = identity.
    c.lqc2(1, identity);
    c.lqc2(2, identity.wrapping_add(0x10));
    c.lqc2(3, identity.wrapping_add(0x20));
    c.lqc2(4, identity.wrapping_add(0x30));
    c.sqc2(1, sp);
    c.sqc2(2, sp.wrapping_add(0x10));
    c.sqc2(3, sp.wrapping_add(0x20));
    c.sqc2(4, sp.wrapping_add(0x30));

    // m = m * scale(1/rx, 1/ry, 1/rz, 1), the scale quad written at sp+0x80.
    let work = sp.wrapping_add(0x90);
    c.lwc1(2, trigger.wrapping_add(0x18));
    c.lwc1(0, trigger.wrapping_add(0x1c));
    c.lwc1(1, trigger.wrapping_add(0x20));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 20);
    c.div_s(2, 20, 2);
    c.swc1(20, sp.wrapping_add(0x8c));
    c.div_s(0, 20, 0);
    c.div_s(1, 20, 1);
    c.swc1(2, sp.wrapping_add(0x80));
    c.swc1(0, sp.wrapping_add(0x84));
    c.swc1(1, sp.wrapping_add(0x88));
    c.lqc2(1, sp.wrapping_add(0x80));
    c.lqc2(4, sp);
    c.lqc2(5, sp.wrapping_add(0x10));
    c.lqc2(6, sp.wrapping_add(0x20));
    c.lqc2(7, sp.wrapping_add(0x30));
    c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
    c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
    c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
    c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
    c.sqc2(8, work);
    c.sqc2(9, work.wrapping_add(0x10));
    c.sqc2(10, work.wrapping_add(0x20));
    c.sqc2(11, work.wrapping_add(0x30));
    copy_matrix(c, sp.wrapping_add(0x40), work);
    copy_matrix(c, sp, sp.wrapping_add(0x40));

    // Three axis rotations, angles negated on the way in.
    rot_axis(c, true, sp.wrapping_add(0x40), trigger, 0xc, 0x004f_f160, sp.wrapping_add(0xd0), sp.wrapping_add(0xd4), 0x0017_2398)?;
    rot_axis(c, true, sp.wrapping_add(0x40), trigger, 0x14, 0x004f_f150, sp.wrapping_add(0xd8), sp.wrapping_add(0xdc), 0x0017_24e0)?;
    rot_axis(c, true, sp.wrapping_add(0x40), trigger, 0x10, 0x004f_f140, sp.wrapping_add(0xe0), sp.wrapping_add(0xe4), 0x0017_2628)?;

    // m = m * translate(-self->pos).
    c.lwc1(2, trigger.wrapping_add(0x8));
    c.lwc1(0, trigger);
    c.lwc1(1, trigger.wrapping_add(0x4));
    c.neg_s(2, 2);
    c.neg_s(0, 0);
    c.swc1(20, sp.wrapping_add(0x4c));
    c.neg_s(1, 1);
    c.swc1(2, sp.wrapping_add(0x48));
    c.swc1(0, sp.wrapping_add(0x40));
    c.swc1(1, sp.wrapping_add(0x44));
    c.swc1(0, sp.wrapping_add(0x50));
    c.swc1(1, sp.wrapping_add(0x54));
    c.swc1(2, sp.wrapping_add(0x58));
    mul_matrix_vec(c, sp.wrapping_add(0x30), sp, sp.wrapping_add(0x40));

    // q = m * *p, then |q.xyz|^2 <= 1.
    mul_matrix_vec(c, sp.wrapping_add(0x50), sp, point);
    c.lq(2, sp.wrapping_add(0x50));
    c.sq(2, sp.wrapping_add(0x40));
    c.lwc1(0, sp.wrapping_add(0x40));
    c.lwc1(2, sp.wrapping_add(0x44));
    c.mul_s(0, 0, 0);
    c.lwc1(1, sp.wrapping_add(0x48));
    c.mul_s(2, 2, 2);
    c.mul_s(1, 1, 1);
    c.add_s(0, 0, 2);
    c.add_s(0, 0, 1);
    c.c_le_s(0, 20);
    let inside = c.fcc();
    c.g[V1][0] = 1;
    if !inside {
        c.cycles += 1;
        c.g[V1][0] = 0;
    }

    c.cycles += 11;
    c.lq(16, sp.wrapping_add(0x150));
    c.g[V0][0] = c.g[V1][0];
    c.lq(17, sp.wrapping_add(0x140));
    c.lq(18, sp.wrapping_add(0x130));
    c.lq(19, sp.wrapping_add(0x120));
    c.lq(20, sp.wrapping_add(0x110));
    c.lq(21, sp.wrapping_add(0x100));
    c.g[RA][0] = c.ld(sp.wrapping_add(0xf0));
    c.lwc1(20, sp.wrapping_add(0x160));
    c.g[SP][0] = sx(sp.wrapping_add(0x170));
    abi::ret(c);
    Ok(())
}


/// The low 32 bits of a GPR.
#[inline(always)]
pub(crate) fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}


/// `void CameraTriggerArray_45c1d0_v01(void* self)` (0x176840): delete the
/// heap array hanging off `+0x8` if there is one.
pub fn camera_trigger_array_45c1d0_v01(c: &mut Cpu) {
    delete_array_member(c, 0x0017_685c)
}


/// `void CameraTriggerArray_45c1b8_v01(void* self)` (0x176868): same
/// destructor for the second camera-trigger array.
pub fn camera_trigger_array_45c1b8_v01(c: &mut Cpu) {
    delete_array_member(c, 0x0017_6884)
}


/// `int* p = *(int**)((char*)self + 8); if (p) operator_delete(p);`
pub(crate) fn delete_array_member(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(arg(c, 0).wrapping_add(0x8)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            call_ptr(c, callee::OPERATOR_DELETE, ret)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `Default_2ChaseCameraAlgorithm* ctor(this)` (0x176a68): tag 0x3c, vtable
/// `D_0045CAB0`.
pub fn default_2_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x3c, 0x0045_cab0, 0x0017_6a7c);
}


/// `Default_3ChaseCameraAlgorithm* ctor(this)` (0x176d68): tag 0x3d, vtable
/// `D_0045CA38`.
pub fn default_3_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x3d, 0x0045_ca38, 0x0017_6d7c);
}


/// `Default_4ChaseCameraAlgorithm* ctor(this)` (0x177068): tag 0x3e, vtable
/// `D_0045C9C0`.
pub fn default_4_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x3e, 0x0045_c9c0, 0x0017_707c);
}


/// `SpokeChaseCameraAlgorithm* ctor(this)` (0x177e50): tag 0x42, vtable
/// `D_0045CCC8`.
pub fn spoke_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x42, 0x0045_ccc8, 0x0017_7e64);
}


// ---------------------------------------------------------------------------
// Camera algorithm.

/// `BoundedCameraAlgorithm* BoundedCameraAlgorithm_ctor(BoundedCameraAlgorithm*
/// self)` (0x174190): run the base constructor and install the vtable, state
/// id 0x5b and the two cleared link fields.
pub fn bounded_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR.call(c, 0x0017_41a4)?;
        let this = r32(c, S0);
        c.g[V0][0] = 0x5b;
        c.g[V1][0] = 0x46_0000;
        sw(c, this, 0x0c, c.g[V0][0]);
        const_reg(c, V1, 0x46_0000, -0x3ba8);
        sw(c, this, 0x10, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        sw(c, this, 0x80, 0);
        sw(c, this, 0x74, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `int CameraTrigger_ReadActionType0(void* reader, void* dst)` (0x1715a0):
/// read the action's float and uint payloads and return the bytes consumed.
pub fn camera_trigger_read_action_type0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0]; // the action record
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0]; // the reader
        frame.save_ra(c, 0);
        Addr(r32(c, S0)).sw(c, 0);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0xc));
        call_ptr(c, table::SERIALIZE_READ_U32, 0x0017_15c8)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(8));
        call_ptr(c, table::SERIALIZE_READ_U32_2, 0x0017_15d8)?;
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `DirectionNeChaseCameraAlgorithm* DirectionNeChaseCameraAlgorithm_ctor(
/// self, int n)` (0x1774c0): chase-camera algorithm scaled by `n`.
pub fn direction_ne_chase_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0017_74dc);
        c.g[S0][0] = c.g[A1][0];
        callee::CHASE_CAMERA_ALGORITHM_CTOR_2.call(c, 0x0017_74dc)?;
        c.mtc1(16, 0);
        c.cvt_s_w(0, 0);
        c.lwc1(1, gp(c, -0x5f4c));
        c.g[V0][0] = 0x0046_0000;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x12));
        c.g[V0][0] = sx(0x0045_cb28);
        Addr(r32(c, S1)).off(0xc).sw(c, c.g[S0][0]);
        c.mul_s(0, 0, 1);
        Addr(r32(c, S1)).off(0x10).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        c.swc1(0, p_2(r32(c, S1), 0x390));
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `int DebugMenu_IsTrackSelectable(int a0, int track_id)` (0x179fc8): 1 when
/// the debug track is the current station track or one of the three special
/// ids.
pub fn debug_menu_is_track_selectable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0017_9fdc);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, callee::BELIBRARY_GET_INSTANCE, 0x0017_9fdc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0017_9fe8);
        c.g[A1][0] = 0;
        call_ptr(c, callee::BELIBRARY_GET_INTERFACE, 0x0017_9fe8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0017_9ff4);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX, 0x0017_9ff4)?;
        c.g[A0][0] = 1;
        let match_current = c.g[V0][0] == c.g[A0][0];
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(0xe));
        if match_current {
            c.cycles += 1;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0;
            c.g[V1][0] = (c.g[V1][0] < 3) as u64;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A0][0];
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The single float at `gp_2 + base` returned in `f0` (the camera's eye drop).
pub(crate) fn return_eye_drop(c: &mut Cpu, base: i32) {
    c.cycles += 3;
    c.lwc1(0, gp_2(c, base));
    abi::ret(c);
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
    c.lq(3, p_3(src, 0x20));
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
    c.lwc1(2, p_3(v, 8));
    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
    c.lwc1(1, p_3(v, 4));
    c.lwc1(0, v);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3); // f3 = 1.0
    c.swc1(0, r(c, SP));
    c.swc1(1, p_3(r(c, SP), 4));
    c.swc1(2, p_3(r(c, SP), 8));
    c.swc1(3, p_3(r(c, SP), 0xc));
    c.lq(2, r(c, SP));
    c.sq(2, out);
    c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Chase-camera algorithms (camera/trigger/cameratriggerfactory.cpp).

/// `void SpokeChaseCameraAlgorithm_GetTuning(SpokeChaseCameraAlgorithm*,
/// CameraTuning* out)` (0x177ec8): copy the four tuning constants of the
/// spoke chase camera (`gp-0x5e1c..-0x5e10`) into `out` (x, y, z, w at
/// `+0x0/+0x4/+0x8/+0xc`), set `out->kind = 1` at `+0x10`, return 1 in `v0`.
pub fn spoke_chase_camera_get_tuning(c: &mut Cpu) {
    c.cycles += 11;
    let gp = r32(c, GP);
    c.lwc1(3, gp.wrapping_sub(0x5e1c));
    c.g[V0][0] = 0x1;
    c.lwc1(2, gp.wrapping_sub(0x5e18));
    c.lwc1(0, gp.wrapping_sub(0x5e14));
    c.lwc1(1, gp.wrapping_sub(0x5e10));
    let out = r32(c, A1);
    c.sw(out.wrapping_add(0x10), c.g[V0][0]);
    c.swc1(3, out.wrapping_add(0x4));
    c.swc1(2, out);
    c.swc1(0, out.wrapping_add(0xc));
    c.swc1(1, out.wrapping_add(0x8));
    abi::ret(c);
}


/// `float SuperpipeChaseCameraAlgorithm_GetEyeDrop(void*)` (0x1785b0):
/// the superpipe camera's eye-drop offset (`gp-0x5dc0`) in `f0`.
pub fn superpipe_chase_camera_get_eye_drop(c: &mut Cpu) {
    c.cycles += 3;
    let gp = r32(c, GP);
    c.lwc1(0, gp.wrapping_sub(0x5dc0));
    abi::ret(c);
}


/// `void SuperpipeChaseCameraAlgorithm_GetTuning(SuperpipeChaseCameraAlgorithm*,
/// CameraTuning* out)` (0x1785c0): the superpipe counterpart of
/// [`spoke_chase_camera_get_tuning`], reading `gp-0x5dbc..-0x5db0`.
pub fn superpipe_chase_camera_get_tuning(c: &mut Cpu) {
    c.cycles += 11;
    let gp = r32(c, GP);
    c.lwc1(3, gp.wrapping_sub(0x5dbc));
    c.g[V0][0] = 0x1;
    c.lwc1(2, gp.wrapping_sub(0x5db8));
    c.lwc1(0, gp.wrapping_sub(0x5db4));
    c.lwc1(1, gp.wrapping_sub(0x5db0));
    let out = r32(c, A1);
    c.sw(out.wrapping_add(0x10), c.g[V0][0]);
    c.swc1(3, out.wrapping_add(0x4));
    c.swc1(2, out);
    c.swc1(0, out.wrapping_add(0xc));
    c.swc1(1, out.wrapping_add(0x8));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Camera algorithms.

pub(crate) fn camera_reset(c: &mut Cpu, base: u32) -> Flow {
    c.cycles += 25;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x10);
    c.g[S1][0] = Addr(p_4(c.g[S0][0] as u32, 0x10)).lw(c);
    c.g[V0][0] = Addr(p_4(c.g[S0][0] as u32, 0x30)).lw(c);
    c.g[A2][0] = Addr(p_4(c.g[S1][0] as u32, 0x18)).lh(c);
    c.g[A0][0] = Addr(p_4(c.g[V0][0] as u32, 4)).lw(c);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A1][0] = Addr(p_4(c.g[A0][0] as u32, 0x6c0)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[V1][0] = Addr(p_4(c.g[A1][0] as u32, 0x38)).lh(c);
    c.g[V0][0] = Addr(p_4(c.g[A1][0] as u32, 0x3c)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    call_ptr(c, target, base + 0x40)?;
    c.g[V1][0] = Addr(p_4(c.g[S1][0] as u32, 0x1c)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    let target2 = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, target2, base + 0x50)?;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// `void BoundedCameraAlgorithmBase_Reset(BoundedCameraAlgorithmBase* self)`
/// (0x173678).
pub fn bounded_camera_algorithm_base_reset(c: &mut Cpu) {
    guest(c, |c| camera_reset(c, 0x0017_3678))
}


/// `void BoundedCameraAlgorithm_Reset(BoundedCameraAlgorithm* self)`
/// (0x1747a0).
pub fn bounded_camera_algorithm_reset(c: &mut Cpu) {
    guest(c, |c| camera_reset(c, 0x0017_47a0))
}


/// `void* SuperpipeChaseCameraAlgorithm_ctor(void* self, Vec4* a, Vec4* b)`
/// (0x178520).
pub fn superpipe_chase_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x0016_2318, 0x0017_8544)?;
        c.g[V1][0] = 0x0046_0000;
        c.g[V0][0] = 0x43;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x33b0));
        Addr(p_4(c.g[S0][0] as u32, 0xc)).sw(c, c.g[V0][0]);
        Addr(p_4(c.g[S0][0] as u32, 0x10)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lq(3, c.g[S1][0] as u32);
        frame.restore(c, S1, 0x20);
        c.sq(3, p_4(c.g[S0][0] as u32, 0x390));
        c.lq(4, c.g[S2][0] as u32);
        frame.restore(c, S2, 0x10);
        c.sq(4, p_4(c.g[S0][0] as u32, 0x3a0));
        frame.restore(c, S0, 0x30);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Address helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}


/// `a + off` sign-extended (`addiu`).
#[inline(always)]
pub(crate) fn ofs(a: u32, off: i32) -> u32 {
    a.wrapping_add(off as u32)
}


/// A `gp`-relative global address.
#[inline(always)]
pub(crate) fn gpa(c: &Cpu, off: i32) -> u32 {
    ofs(c.g[GP][0] as u32, off)
}


/// The current stack pointer.
#[inline(always)]
pub(crate) fn spv(c: &Cpu) -> u32 {
    c.g[SP][0] as u32
}


/// `addiu sp, sp, -n`.
#[inline(always)]
pub(crate) fn open(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(n));
}


/// `lw` of `base + off`.
#[inline(always)]
pub(crate) fn lw_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(at(base as u32, off)).lw(c)
}


/// `lh` of `base + off`.
#[inline(always)]
pub(crate) fn lh_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(at(base as u32, off)).lh(c)
}


/// `lhu` of `base + off`.
#[inline(always)]
pub(crate) fn lhu_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(at(base as u32, off)).lhu(c)
}


/// `sw` at `base + off`.
#[inline(always)]
pub(crate) fn sw_at(c: &mut Cpu, base: u64, off: u32, v: u64) {
    Addr(at(base as u32, off)).sw(c, v);
}


/// `sh` at `base + off`.
#[inline(always)]
pub(crate) fn sh_at(c: &mut Cpu, base: u64, off: u32, v: u64) {
    Addr(at(base as u32, off)).sh(c, v);
}


/// `sb` at `base + off`.
#[inline(always)]
pub(crate) fn sb_at(c: &mut Cpu, base: u64, off: u32, v: u64) {
    Addr(at(base as u32, off)).sb(c, v);
}


/// `lwc1 f, addr`.
#[inline(always)]
pub(crate) fn ldf(c: &mut Cpu, f: usize, addr: u32) -> FpuF32 {
    c.lwc1(f, addr);
    FpuF32(c.f[f])
}


// ---------------------------------------------------------------------------
// Chase-camera algorithm resets.

/// Where a `<X>ChaseCameraAlgorithm_ResetForRider` gets its two distance
/// constants.
#[derive(Clone, Copy)]
pub(crate) enum BackHeight {
    /// `back = *(gp + off)`, `down = 150.0`.
    Gp(i32),
    /// Post-race: `back = down = 100.0`.
    Hundred,
}


/// Shared body of the seven `<X>ChaseCameraAlgorithm_ResetForRider` leaves
/// (0x168f60, 0x176ce0, 0x176fe0, 0x1772e0, 0x177588, 0x178e90): the camera
/// controller at `*(this+0x30)` owns the rider at `+4`; the rider's current
/// algorithm id is asked through the `+0x6c0` vtable slot, and when it is
/// still `rider`, the algorithm is reset for that rider and re-initialised
/// from it (back distance / height), then the target's current-algorithm
/// slot is notified.
pub(crate) fn chase_reset_for_rider(c: &mut Cpu, entry: u32, variant: BackHeight) -> Flow {
    c.cycles += 16;
    open(c, 0x30);
    let sp0 = spv(c);
    c.sq(16, at(sp0, 0x20));
    c.sq(17, at(sp0, 0x10));
    let this = c.g[A0][0];
    c.g[S0][0] = this;
    let rider = c.g[A1][0];
    c.g[S1][0] = rider;
    c.sd(sp0, c.g[RA][0]);

    let target = lw_at(c, this, 0x30);
    c.g[V0][0] = target;
    let obj = lw_at(c, target, 0x4);
    c.g[A0][0] = obj;
    let vt = lw_at(c, obj, 0x6c0);
    c.g[V1][0] = vt;
    c.g[A0][0] = sx(at(obj as u32, 0x6c0));
    c.g[A1][0] = lh_at(c, vt, 0x38);
    c.g[V0][0] = lw_at(c, vt, 0x3c);
    let func = c.g[V0][0] as u32;
    c.g[A0][0] = sx(at(c.g[A0][0] as u32, c.g[A1][0] as u32));
    call_ptr(c, func, entry + 0x38)?;
    if c.g[S1][0] != c.g[V0][0] {
        c.lq(16, at(sp0, 0x20));
    } else {
        c.cycles += 14;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        callee::RESET_FOR_RIDER.call(c, entry + 0x4c)?;
        match variant {
            BackHeight::Gp(off) => {
                let a = gpa(c, off);
                ldf(c, 12, a);
                c.g[AT][0] = 0x4396_0000;
                c.mtc1(1, 13);
            }
            BackHeight::Hundred => {
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 12);
                c.mov_s(13, 12);
            }
        }
        c.g[A0][0] = c.g[S0][0];
        callee::INIT_FROM_RIDER.call(c, entry + 0x60)?;
        let a0 = lw_at(c, c.g[S0][0], 0x10);
        c.g[V0][0] = a0;
        c.g[A0][0] = lh_at(c, a0, 0x28);
        c.g[V1][0] = lw_at(c, a0, 0x2c);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(c.g[S0][0] as u32, c.g[A0][0] as u32));
        call_ptr(c, func, entry + 0x74)?;
        c.lq(16, at(sp0, 0x20));
    }
    c.cycles += 4;
    c.lq(17, at(sp0, 0x10));
    c.g[RA][0] = c.ld(sp0);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(sp0, 0x30));
    c.pc = ra;
    Ok(())
}


/// `void Default_2ChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x176ce0).
pub fn default_2_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0017_6ce0, BackHeight::Gp(-0x6138))
    })
}


/// `void Default_3ChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x176fe0).
pub fn default_3_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0017_6fe0, BackHeight::Gp(-0x607c))
    })
}


/// `void Default_4ChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x1772e0).
pub fn default_4_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0017_72e0, BackHeight::Gp(-0x5fc0))
    })
}


/// `void DirectionNeChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x177588).
pub fn direction_ne_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0017_7588, BackHeight::Gp(-0x5f38))
    })
}


/// `void Post_Race_1ChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x178e90).
pub fn post_race_1_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0017_8e90, BackHeight::Hundred)
    })
}


/// The post-race orbit: one turn every 1100 updates (gp-0x5d58 = 1/1100).
const POST_RACE_ORBIT_FRAMES: u32 = 1100;

/// `void Post_Race_1ChaseCameraAlgorithm_Update(algo)` (0x178bb0): the
/// results-screen camera. Gathers the rider state with this camera's
/// tuning, steps the orbit counter `+0x3a0` (`(n + 1) % 1100`), and turns
/// the chase direction `+0x390` about the world up by a swing angle
/// `a = (1 − cos(2π n/1100)) × 0.748 − 0.748` (both angles wrapped to
/// ±π); the look-at `+0x20` is the target's position (slot 1) 3 below, the
/// eye `+0x40` sits `126 (+200)` back along the turned direction and
/// `−27 (+150)` up (the `+` terms with two or more players).
///
/// The counter counts updates and the 1100 is 60 Hz frames: at another rate
/// it wraps at `ticks_from_frames60(1100)` and the per-update angle share
/// is `(1/1100) × 1100/that`, so one orbit takes the same wall time and the
/// angle still advances every update.
pub fn post_race_1_chase_camera_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
        let sp = c.g[SP][0] as u32;
        let gp = c.g[GP][0] as u32;
        c.lwc1(12, gp.wrapping_sub(0x5d7c));
        c.sq(17, sp.wrapping_add(0x110));
        c.g[A1][0] = c.g[SP][0];
        for (f, off) in [(13, 0x5d78u32), (14, 0x5d74), (15, 0x5d70), (16, 0x5d6c), (17, 0x5d68), (18, 0x5d64), (19, 0x5d60)] {
            c.lwc1(f, gp.wrapping_sub(off));
        }
        c.swc1(22, sp.wrapping_add(0x140));
        c.swc1(21, sp.wrapping_add(0x138));
        c.swc1(20, sp.wrapping_add(0x130));
        c.sq(16, sp.wrapping_add(0x120));
        c.sq(18, sp.wrapping_add(0x100));
        c.sd(sp.wrapping_add(0xf0), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        let algo = c.g[S1][0] as u32;
        // ChaseCameraAlgorithm_GatherRiderState(algo, &state, tuning f12..f19).
        call_ptr(c, 0x0016_2568, 0x0017_8bfc)?;

        // The orbit counter.
        c.lwc1(21, gp.wrapping_sub(0x5d5c));
        c.g[V0][0] = c.lw(algo.wrapping_add(0x3a0));
        c.g[V1][0] = POST_RACE_ORBIT_FRAMES as u64;
        c.lwc1(0, gp.wrapping_sub(0x5d58));
        if crate::tick::rate(c) != 60 {
            let n = crate::tick::ticks_from_frames60(c, POST_RACE_ORBIT_FRAMES);
            c.g[V1][0] = n as u64;
            let share = FpuF32(c.f[0]) * FpuF32((POST_RACE_ORBIT_FRAMES as f32 / n as f32).to_bits());
            c.f[0] = share.0;
        }
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.lwc1(22, gp.wrapping_sub(0x5d54));
        c.div(0, V0, V1);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 20);
        c.sw(algo.wrapping_add(0x3a0), c.g[V0][0]);

        // angle = wrap(n × share × 2π).
        c.cycles += 14;
        c.g[A0][0] = c.hi[0];
        c.mtc1(A0, 1);
        c.cvt_s_w(1, 1);
        c.sw(algo.wrapping_add(0x3a0), c.g[A0][0]);
        c.mul_s(1, 1, 0);
        c.mul_s(1, 1, 21);
        c.mul_s(0, 1, 22);
        c.add_s(0, 0, 20);
        c.cvt_w_s(12, 0);
        c.cvt_s_w(12, 12);
        c.c_lt_s(0, 12);
        if c.fcc() {
            c.cycles += 4;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.sub_s(12, 12, 0);
        }
        c.mul_s(12, 12, 21);

        // Math_Cos, then the swing angle, wrapped the same way.
        c.cycles += 17;
        c.sub_s(12, 1, 12);
        call_ptr(c, 0x0031_c040, 0x0017_8c7c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 3);
        c.lwc1(2, gp.wrapping_sub(0x5d50));
        c.sub_s(0, 3, 0);
        c.lwc1(1, gp.wrapping_sub(0x5d4c));
        c.mul_s(0, 0, 2);
        c.add_s(12, 0, 1);
        c.mul_s(0, 12, 22);
        c.add_s(0, 0, 20);
        c.cvt_w_s(1, 0);
        c.cvt_s_w(1, 1);
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.sub_s(1, 1, 3);
        }

        // Math_SinCos(a / 2, &s, &c): the rotation about the world up.
        c.cycles += 69;
        c.mul_s(0, 1, 21);
        c.g[A1][0] = sx(sp.wrapping_add(0xe4));
        c.g[S2][0] = 0x50_0000;
        c.g[A0][0] = sx(sp.wrapping_add(0xe0));
        c.g[S0][0] = sx(0x004f_f160);
        c.sub_s(12, 12, 0);
        c.mul_s(12, 12, 20);
        call_ptr(c, 0x0031_be50, 0x0017_8cd8)?;
        let up = 0x004f_f160u32;
        c.lwc1(3, up + 0x8);
        c.g[A0][0] = sx(sp.wrapping_add(0xa0));
        c.lwc1(0, sp.wrapping_add(0xe0));
        c.lwc1(1, up);
        c.lwc1(4, up + 0x4);
        c.mul_s(3, 0, 3);
        c.mul_s(1, 0, 1);
        c.lwc1(2, sp.wrapping_add(0xe4));
        c.mul_s(0, 0, 4);
        c.swc1(3, sp.wrapping_add(0x88));
        c.swc1(2, sp.wrapping_add(0x8c));
        c.swc1(0, sp.wrapping_add(0x84));
        c.swc1(1, sp.wrapping_add(0x80));
        c.lq(2, sp.wrapping_add(0x80));
        c.sq(2, sp.wrapping_add(0x70));
        // v = q · dir · q* (VU0 outer products and broadcasts).
        c.lqc2(4, sp.wrapping_add(0x70));
        c.lqc2(5, algo.wrapping_add(0x390));
        c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().opmula(4, 6);
        c.vu0().opmsub(7, 6, 4);
        c.vu0().mula(0b1110, 5, Src::Bc(0, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 7, Src::Bc(0, W));
        c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
        c.sqc2(8, sp.wrapping_add(0x90));
        c.lq(2, sp.wrapping_add(0x90));
        // target->GetPosition(&pos) (slot 1).
        c.g[A2][0] = c.lw(algo.wrapping_add(0x30));
        c.sq(2, sp.wrapping_add(0x80));
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, t, 0x0017_8d68)?;

        // look_at = pos + up × −3.
        c.g[AT][0] = 0xffff_ffff_c040_0000;
        c.mtc1(AT, 0);
        c.mfc1(V0, 0);
        c.lqc2(4, up);
        c.qmtc2(V0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp.wrapping_add(0xc0));
        c.mtc1(0, 3);
        c.lq(2, sp.wrapping_add(0xc0));
        c.sq(2, sp.wrapping_add(0xb0));
        c.lqc2(3, sp.wrapping_add(0xa0));
        c.lqc2(4, sp.wrapping_add(0xb0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp.wrapping_add(0xc0));
        c.lq(2, sp.wrapping_add(0xc0));
        c.g[A1][0] = c.lw(gp.wrapping_sub(0x848));
        c.sq(2, algo.wrapping_add(0x20));
        c.sq(2, sp.wrapping_add(0x90));
        // Two or more players: the eye sits farther back and higher.
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x78));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64 ^ 1;
        let multi = c.g[V0][0] != 0;
        if multi {
            c.cycles += 2;
            c.g[AT][0] = 0x4316_0000;
            c.mtc1(AT, 3);
        }
        c.cycles += 3;
        c.mtc1(0, 2);
        if multi {
            c.cycles += 2;
            c.g[AT][0] = 0x4348_0000;
            c.mtc1(AT, 2);
        }

        // eye = look_at − v × (d + 126) + up × (h − 27).
        c.cycles += 42;
        c.g[AT][0] = 0x42fc_0000;
        c.mtc1(AT, 0);
        c.g[AT][0] = 0xffff_ffff_c1d8_0000;
        c.mtc1(AT, 1);
        c.add_s(0, 2, 0);
        c.lq(16, sp.wrapping_add(0x120));
        c.mfc1(V0, 0);
        c.lqc2(4, sp.wrapping_add(0x80));
        c.qmtc2(V0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp.wrapping_add(0xc0));
        c.add_s(1, 3, 1);
        c.lq(2, sp.wrapping_add(0xc0));
        c.g[RA][0] = c.ld(sp.wrapping_add(0xf0));
        c.sq(2, sp.wrapping_add(0xb0));
        c.lqc2(3, algo.wrapping_add(0x20));
        c.lqc2(4, sp.wrapping_add(0xb0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp.wrapping_add(0xc0));
        c.lwc1(22, sp.wrapping_add(0x140));
        c.lq(2, sp.wrapping_add(0xc0));
        c.lwc1(21, sp.wrapping_add(0x138));
        c.sq(2, sp.wrapping_add(0xa0));
        c.mfc1(V1, 1);
        c.lqc2(4, up);
        c.qmtc2(V1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp.wrapping_add(0xd0));
        c.lwc1(20, sp.wrapping_add(0x130));
        c.lq(2, sp.wrapping_add(0xd0));
        c.lq(18, sp.wrapping_add(0x100));
        c.sq(2, sp.wrapping_add(0xc0));
        c.lqc2(3, sp.wrapping_add(0xa0));
        c.lqc2(4, sp.wrapping_add(0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp.wrapping_add(0xd0));
        c.lq(2, sp.wrapping_add(0xd0));
        c.sq(2, algo.wrapping_add(0x40));
        c.lq(17, sp.wrapping_add(0x110));
        c.sq(2, sp.wrapping_add(0x90));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp.wrapping_add(0x150));
        c.pc = t;
        Ok(())
    })
}


/// `int CameraTrigger_ReadActionType2(CameraTrigger* self, u32* out)`
/// (0x1715f8): read five serialized action-type words into `out[3..7]`
/// (`out[0] = 2`) and return their sum.
pub fn camera_trigger_read_action_type2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        open(c, 0x40);
        let sp0 = spv(c);
        c.g[V0][0] = 2;
        c.sq(17, at(sp0, 0x20));
        c.sq(18, at(sp0, 0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sq(16, at(sp0, 0x30));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        let out = c.g[S1][0] as u32;
        let reader = c.g[S2][0];
        c.g[A1][0] = sx(at(out, 0xc));
        c.sw(out, c.g[V0][0]);
        call_ptr(c, callee::SERIALIZE_READ_U32, 0x0017_1624)?;
        c.g[A0][0] = reader;
        c.g[A1][0] = sx(at(out, 0x10));
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, callee::SERIALIZE_READ_U32, 0x0017_1634)?;
        c.g[A0][0] = reader;
        c.g[A1][0] = sx(at(out, 0x14));
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V0][0] as u32));
        call_ptr(c, callee::SERIALIZE_READ_U32, 0x0017_1644)?;
        c.g[A0][0] = reader;
        c.g[A1][0] = sx(at(out, 0x18));
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V0][0] as u32));
        call_ptr(c, callee::SERIALIZE_READ_U32, 0x0017_1654)?;
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V0][0] as u32));
        c.g[A0][0] = reader;
        c.g[A1][0] = sx(at(out, 0x1c));
        call_ptr(c, callee::SERIALIZE_READ_U32, 0x0017_1664)?;
        c.g[V0][0] = sx(at(c.g[S0][0] as u32, c.g[V0][0] as u32));
        c.lq(17, at(sp0, 0x20));
        c.lq(16, at(sp0, 0x30));
        c.lq(18, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}


/// `int DebugMenu_NextEnabledItem(DebugMenu* self, int i)` (0x17a028): the
/// next entry index after `i` that is enabled, -1 when the menu is off.
pub fn debug_menu_next_enabled_item(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = 0x8000;
    c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(at_2(c, V0, 0xc08));
    let present = c.g[V1][0] != 0;
    c.g[A0][0] = c.g[A1][0];
    if !present {
        c.cycles += 2;
        let ra = r32(c, RA);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.pc = ra;
        return;
    }
    c.cycles += 7;
    c.g[A2][0] = c.lw(r32(c, A3));
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
    c.g[V1][0] = 0x8c;
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x8c));
    c.div(0, 2, 6);
    c.cycles += 6;
    c.g[A0][0] = c.hi[0];
    c.mult(0, 3, 4, 3);
    c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
    c.g[V0][0] = c.lw(r32(c, V1));
    let enabled = c.g[V0][0] != 0;
    c.g[A1][0] = c.g[A2][0];
    if !enabled {
        c.cycles += 3;
        c.g[T0][0] = 0;
        c.g[A2][0] = 0x8c;
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(1));
        loop {
            c.cycles += 2;
            c.cycles += 8;
            c.div(0, 3, 5);
            c.g[V0][0] = c.hi[0];
            c.g[A0][0] = c.g[V0][0];
            c.mult(0, 2, 4, 6);
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
            c.g[V0][0] = c.lw(r32(c, V1));
            let empty = c.g[V0][0] == 0;
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(1));
            if !empty {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0017_a088);
            }
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[V0][0] = c.g[A0][0];
    c.pc = ra;
}


// ---------------------------------------------------------------------------
// Debug menu.

/// `int DebugMenu_PrevEnabledItem(DebugMenu* menu, int start)` (0x17a0b8):
/// walk backwards from `start` over the menu's item table, wrapping with the
/// item count at `menu+0`, and return the first item whose `+0x118` word is
/// non-zero; -1 when the menu is disabled (`menu+0x8808` is zero) or none is
/// enabled.
pub fn debug_menu_prev_enabled_item(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = 0x8000;
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = lw(c, p_5(c.g[V0][0] as u32, 0xc08));
    if c.g[V1][0] != 0 {
        c.g[A2][0] = lw(c, c.g[A3][0] as u32);
        c.cycles += 5;
        c.g[A0][0] = 0x8c;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8c));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.cycles += 9;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.div(0, 2, 6);
        c.g[V1][0] = c.hi[0];
        c.g[A1][0] = c.g[V1][0];
        c.mult(0, 4, 5, 4);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = lw(c, c.g[A0][0] as u32);
        let found = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[A2][0];
        if !found {
            c.cycles += 4;
            c.g[T0][0] = 0;
            c.g[A2][0] = 0x8c;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            loop {
                c.cycles += 2;
                // `if (a0 == t0) break; else break;` — both branches leave the block.
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.div(0, 2, 4);
                c.g[V1][0] = c.hi[0];
                c.g[A1][0] = c.g[V1][0];
                c.mult(0, 3, 5, 6);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V1][0] = lw(c, c.g[V0][0] as u32);
                let stop = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if stop {
                    if event_due(c) {
                        deliver_event_here(c, 0x0017_a120);
                    }
                    continue;
                }
                break;
            }
        }
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    abi::ret(c);
}


/// `void DebugMenu_FindNextItem(menu, index, op)` (0x17a220): starting at_3
/// `(index + 1) % count`, dispatch the op on each item until one accepts it.
pub fn debug_menu_find_next_item(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        'end: {
            'found: {
                c.cycles += 14;
                c.g[A1][0] = sx(at_3(c.g[A1][0], 1));
                frame.save(c, S0, 0x40);
                frame.save(c, S1, 0x30);
                frame.save(c, S2, 0x20);
                frame.save(c, S3, 0x10);
                c.g[S1][0] = 0;
                c.g[S2][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[S3][0] = c.g[A2][0];
                c.g[V0][0] = lw_2(c, c.g[S2][0] as u32);
                c.div(0, 5, 2);
                c.cycles += 3;
                c.g[V1][0] = c.hi[0];
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S0][0] = c.g[V1][0];
                if t {
                    break 'found;
                }
                c.cycles += 1;
                loop {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0017_a278);
                    c.g[A2][0] = c.g[S3][0];
                    callee::DEBUG_MENU_DISPATCH_ITEM_OP.call(c, 0x0017_a278)?;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S0][0];
                    if t {
                        break 'end;
                    }
                    {
                        c.cycles += 6;
                        c.g[V0][0] = lw_2(c, c.g[S2][0] as u32);
                        c.g[V1][0] = sx(at_3(c.g[S0][0], 1));
                        c.g[S1][0] = sx(at_3(c.g[S1][0], 1));
                        c.div(0, 3, 2);
                    }
                    c.cycles += 4;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[A0][0] = c.hi[0];
                    let t = c.g[V0][0] != 0;
                    c.g[S0][0] = c.g[A0][0];
                    if !t {
                        break 'found;
                    }
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x0017_a268);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx(0xffff_ffff);
            break 'end;
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


// ---------------------------------------------------------------------------
// `BoundedCameraAlgorithmBase_ctor` (0x173208): run the rider-camera base
// constructor, install the vtable and the bounded-camera defaults, and return
// the object.
pub fn bounded_camera_algorithm_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001622b0, 0x0017_321c)?;
        c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x6434));
        c.g[V1][0] = 0x460000;
        c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x6430));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3b38));
        c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x642c));
        c.g[A0][0] = 0x500000;
        c.g[AT][0] = 0x40a00000;
        c.mtc1(1, 4);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xed0));
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 3);
        c.g[A1][0] = 0x5a;
        c.g[AT][0] = 0x428c0000;
        c.mtc1(1, 2);
        c.g[V0][0] = c.g[S0][0];
        c.g[AT][0] = 0x43c80000;
        c.mtc1(1, 0);
        c.g[AT][0] = 0x41a00000;
        c.mtc1(1, 1);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0x58));
        c.swc1(7, (c.g[S0][0] as u32).wrapping_add(0x5c));
        c.swc1(5, (c.g[S0][0] as u32).wrapping_add(0x60));
        c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x64));
        c.swc1(3, (c.g[S0][0] as u32).wrapping_add(0x68));
        c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x6c));
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x70));
        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x78));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x74), 0u64);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lq(3, (c.g[A0][0] as u32));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x40));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `DebugMenu_FindPrevItem` (0x17a2c8): walk the menu ring backwards from
// `start + count - 1`, returning the first item the dispatch callback accepts
// (or -1).
pub fn debug_menu_find_prev_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0u64;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A2][0];
        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.div(0, 5, 3);
        c.g[V0][0] = c.hi[0];
        let nonempty = (c.g[V1][0] as i64) > 0;
        c.g[S0][0] = c.g[V0][0];
        let mut found = false;
        if nonempty {
            'walk: loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S3][0];
                callee::DEBUG_MENU_DISPATCH_ITEM_OP.call(c, 0x0017_a320)?;
                let hit = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[S0][0];
                if hit {
                    found = true;
                    break 'walk;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.g[A1][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.div(0, 3, 4);
                c.g[V0][0] = c.hi[0];
                let again = c.g[A1][0] != 0;
                c.g[S0][0] = c.g[V0][0];
                if !again {
                    break 'walk;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_a310);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
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


// ---------------------------------------------------------------------------
// `SuperpipeChaseCameraAlgorithm_ResetForRider` (0x178938): if the rider's
// camera target id matches, recompute the desired eye and look-at, reset the
// chase camera for the rider and initialise it with the superpipe distances.
pub fn superpipe_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        call_ptr(c, target, 0x0017_8970)?;
        if c.g[S1][0] != c.g[V0][0] {
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.cycles += 24;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            callee::SUPERPIPE_COMPUTE_DESIRED_EYE.call(c, 0x0017_8984)?;
            c.lq(2, (c.g[SP][0] as u32));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x40));
            callee::SUPERPIPE_COMPUTE_LOOK_AT.call(c, 0x0017_8998)?;
            c.lq(2, (c.g[SP][0] as u32));
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
            callee::CHASE_CAMERA_RESET_FOR_RIDER.call(c, 0x0017_89ac)?;
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5d84));
            c.g[AT][0] = 0x43960000;
            c.mtc1(1, 13);
            c.g[A0][0] = c.g[S0][0];
            callee::CHASE_CAMERA_INIT_FROM_RIDER.call(c, 0x0017_89c0)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0017_89d4)?;
            c.lq(16, frame.at(c, 0x30));
        }
        c.cycles += 4;
        c.lq(17, frame.at(c, 0x20));
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void DebugMenu_FreeBackgrounds(self, int flags)` (0x17abe0): release the
/// debug menu's background objects, then the menu itself when `flags & 1`.
pub fn debug_menu_free_backgrounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[V0][0] = Addr(this as u32).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !empty {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_6(this as u32, 4)).lw(c);
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0;
            if !none {
                c.cycles += 2;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    let removed = (c.g[A1][0] as i64) < 0;
                    c.g[A2][0] = Addr(p_6(r32(c, GP), 0xffff_f7ac)).lw(c);
                    if !removed {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(p_6(c.g[A2][0] as u32, 0x10d8)).lw(c);
                        c.g[A0][0] = Addr(p_6(c.g[V0][0] as u32, 0x190)).lh(c);
                        c.g[V1][0] = Addr(p_6(c.g[V0][0] as u32, 0x194)).lw(c);
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0017_ac40)?;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p_6(c.g[S1][0] as u32, 4)).lw(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0017_ac18);
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
            let no_array = c.g[A0][0] == 0;
            c.g[V0][0] = c.g[S2][0] & 0x1;
            if !no_array {
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x0017_ac68)?;
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0] & 0x1;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0] & 0x1;
        }
        c.cycles += 2;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0017_ac7c)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Camera trigger bounds.

/// `void CameraSphereTrigger_GetBounds(CameraTrigger*, Bounds*)` (0x1721c0):
/// the trigger's local box (`+0` min, `+8` max, `+0x18` centre) grown by the
/// sphere radius; `out+0` receives the min corner, `out+0x10` the max corner
/// and `out+0x30`/`+0x40` their padding words.
pub(crate) fn camera_trigger_bounds(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 46;
    let frame = Frame::open(c, 0x80);
    frame.save(c, S0, 0x70);
    frame.save(c, S1, 0x60);
    c.g[S0][0] = c.g[A1][0];
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0x50);
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = sx(r(c, S0).wrapping_add(0x18));
    callee::VEC3_MAX_COMPONENT.call(c, ret)?;

    // radius f0 -> the bounds.
    c.lwc1(3, p_7(r(c, S0), 0x8));
    c.g[V0][0] = c.g[S1][0];
    c.lwc1(2, r(c, S0));
    c.lwc1(1, p_7(r(c, S0), 0x4));
    c.add_s(7, 3, 0);
    c.add_s(6, 2, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 4);
    c.add_s(5, 1, 0);
    c.swc1(0, r(c, SP));
    c.sub_s(2, 2, 0);
    c.swc1(7, p_7(r(c, SP), 0x38));
    c.sub_s(1, 1, 0);
    c.swc1(6, p_7(r(c, SP), 0x30));
    c.sub_s(3, 3, 0);
    c.swc1(5, p_7(r(c, SP), 0x34));
    c.swc1(2, p_7(r(c, SP), 0x10));
    c.swc1(1, p_7(r(c, SP), 0x14));
    c.swc1(3, p_7(r(c, SP), 0x18));
    c.swc1(4, p_7(r(c, SP), 0x3c));
    c.swc1(4, p_7(r(c, SP), 0x1c));
    c.lq(3, p_7(r(c, SP), 0x30));
    c.lq(4, p_7(r(c, SP), 0x10));
    c.sq(3, p_7(r(c, S1), 0x10));
    c.sq(4, r(c, S1));
    frame.restore(c, 16, 0x70);
    frame.restore(c, 17, 0x60);
    frame.restore_ra(c, 0x50);
    c.swc1(0, p_7(r(c, SP), 0x4));
    c.swc1(0, p_7(r(c, SP), 0x8));
    c.swc1(2, p_7(r(c, SP), 0x20));
    c.swc1(1, p_7(r(c, SP), 0x24));
    c.swc1(3, p_7(r(c, SP), 0x28));
    c.swc1(6, p_7(r(c, SP), 0x40));
    c.swc1(5, p_7(r(c, SP), 0x44));
    c.swc1(7, p_7(r(c, SP), 0x48));
    frame.close(c);
    Ok(())
}


/// `void CameraSphereTrigger_GetBounds(CameraTrigger*, Bounds*)` (0x1721c0).
pub fn camera_sphere_trigger_get_bounds(c: &mut Cpu) {
    guest(c, |c| camera_trigger_bounds(c, 0x0017_21e0))
}


/// `void CameraBoxTrigger_GetBounds(CameraTrigger*, Bounds*)` (0x172840).
pub fn camera_box_trigger_get_bounds(c: &mut Cpu) {
    guest(c, |c| camera_trigger_bounds(c, 0x0017_2860))
}


// ---------------------------------------------------------------------------
// Camera trigger action reader.

/// `int CameraTrigger_ReadActionType1(CameraTrigger*, ActionBoundedCam*)`
/// (0x1714e8): read the six serialized fields of a bounded-camera action and
/// return their sum.
pub fn camera_trigger_read_action_type1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let out = r(c, A1);
        c.g[S1][0] = sx(out);
        frame.save(c, S0, 0x30);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save_ra(c, 0);
        Addr(out).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(p_7(out, 0xc));
        c.g[RA][0] = sx(0x0017_1514);
        call_ptr(c, 0x002d_2510, 0x0017_1514)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p_7(r(c, S1), 0x10));
        c.g[RA][0] = sx(0x0017_1524);
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x002d_2510, 0x0017_1524)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p_7(r(c, S1), 0x14));
        c.g[RA][0] = sx(0x0017_1534);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        call_ptr(c, 0x002d_2510, 0x0017_1534)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p_7(r(c, S1), 0x18));
        c.g[RA][0] = sx(0x0017_1544);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        call_ptr(c, 0x002d_2510, 0x0017_1544)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p_7(r(c, S1), 0x1c));
        c.g[RA][0] = sx(0x0017_1554);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        call_ptr(c, 0x002d_2510, 0x0017_1554)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p_7(r(c, S1), 0x20));
        c.g[RA][0] = sx(0x0017_1564);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        call_ptr(c, 0x002d_2510, 0x0017_1564)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p_7(r(c, S1), 0x24));
        c.g[RA][0] = sx(0x0017_1574);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        call_ptr(c, 0x002d_2558, 0x0017_1574)?;
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0017_1584);
        c.g[A1][0] = sx(p_7(r(c, S1), 0x28));
        call_ptr(c, 0x002d_25a0, 0x0017_1584)?;
        c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        frame.restore(c, 17, 0x20);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `SpokeChaseCameraAlgorithm_GetFocusPoint(this, out)` (0x177ef8): the
/// focus point is 20% of the spoke's point plus 80% of the camera position,
/// both read from scaled quads through the object's vtable.
pub fn spoke_chase_camera_algorithm_get_focus_point(c: &mut Cpu) {
    c.cycles += 49;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S0][0] = c.g[A1][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
    c.g[S1][0] = c.g[A0][0];
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
    let t = c.g[V0][0] as u32;
    c.g[RA][0] = sx(0x00177f30);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    if call_ptr(c, t, 0x00177f30).is_err() {
        return;
    }
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5e0c));
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.mfc1(3, 0);
    c.lqc2(4, (c.g[V0][0] as u32));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    let t = c.g[V0][0] as u32;
    c.g[RA][0] = sx(0x00177f6c);
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    if call_ptr(c, t, 0x00177f6c).is_err() {
        return;
    }
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5e08));
    c.g[V0][0] = c.g[S1][0];
    c.mfc1(4, 0);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(3, (c.g[SP][0] as u32));
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(3, (c.g[S1][0] as u32));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    c.pc = t;
    return;
}


// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
pub(crate) fn p_8(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}


/// `lq reg, src; sq reg, dst` (a quadword copied through a GPR).
#[inline(always)]
pub(crate) fn qcopy(c: &mut Cpu, reg: usize, src: u32, dst: u32) {
    c.lq(reg, src);
    c.sq(reg, dst);
}


/// `get_cCTBoundObjBox(reader, obj)` / `get_cCTVolumeBox(reader, obj)`
/// (0x171b10 / 0x1730d0): read a camera trigger box from the reader: the
/// box object's centre and axis vectors and its three size floats, through
/// the object's vtable, summing the consumed floats.
pub(crate) fn read_camera_box(c: &mut Cpu, ret: &[u32; 10]) -> crate::call::Flow {
    c.cycles += 60;
    let frame = Frame::open(c, 0x40);
    c.g[V0][0] = 1;
    frame.save(c, 16, 0x30);
    frame.save(c, 17, 0x20);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, 18, 0x10);
    frame.save_ra(c, 0);
    c.g[S2][0] = c.g[A0][0];
    c.sw(p_8(c.g[S0][0] as u32, 0x28), c.g[V0][0]);
    // slot 1 (centre vector), then slot 5 (axis vector)
    for (i, off) in [(0usize, 8u32), (2, 0x28)] {
        let vt = lw(c, p_8(c.g[S0][0] as u32, 0x24));
        c.g[V1][0] = vt;
        c.g[A0][0] = c.lh(p_8(c.g[V1][0] as u32, off));
        c.g[V0][0] = lw(c, p_8(c.g[V1][0] as u32, off + 4));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(ret[i]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        ssx3_ee_rt::funcs::call_indirect(c, t);
        if c.pc != ret[i] {
            return Err(Unwound);
        }
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(ret[i + 1]);
        c.g[A1][0] = c.g[V0][0];
        c.pc = 0x002d_25a0;
        call_ptr(c, 0x002d_25a0, ret[i + 1])?;
        if i == 0 {
            c.g[S1][0] = c.g[V0][0];
        } else {
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        }
    }
    // slots 2, 3, 4: the three size floats
    for (i, off) in [(4usize, 0x10u32), (6, 0x18), (8, 0x20)] {
        let vt = lw(c, p_8(c.g[S0][0] as u32, 0x24));
        c.g[V1][0] = vt;
        c.g[A0][0] = c.lh(p_8(c.g[V1][0] as u32, off));
        c.g[V0][0] = lw(c, p_8(c.g[V1][0] as u32, off + 4));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(ret[i]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        ssx3_ee_rt::funcs::call_indirect(c, t);
        if c.pc != ret[i] {
            return Err(Unwound);
        }
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(ret[i + 1]);
        c.g[A1][0] = c.g[V0][0];
        c.pc = 0x002d_2510;
        call_ptr(c, 0x002d_2510, ret[i + 1])?;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    }
    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    frame.restore(c, 16, 0x30);
    frame.restore(c, 17, 0x20);
    frame.restore(c, 18, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// `get_cCTBoundObjBox(reader, obj)` (0x171b10).
pub fn camera_box_bound_obj_read(c: &mut Cpu) {
    guest(c, |c| {
        read_camera_box(
            c,
            &[
                0x0017_1b48, 0x0017_1b54, 0x0017_1b6c, 0x0017_1b78, 0x0017_1b90, 0x0017_1b9c,
                0x0017_1bb4, 0x0017_1bc0, 0x0017_1bd8, 0x0017_1be4,
            ],
        )
    })
}


/// `get_cCTVolumeBox(reader, obj)` (0x1730d0).
pub fn camera_trigger_read_box_trigger_data(c: &mut Cpu) {
    guest(c, |c| {
        read_camera_box(
            c,
            &[
                0x0017_3108, 0x0017_3114, 0x0017_312c, 0x0017_3138, 0x0017_3150, 0x0017_315c,
                0x0017_3174, 0x0017_3180, 0x0017_3198, 0x0017_31a4,
            ],
        )
    })
}


/// `ChaseCameraAlgorithm* ChaseCameraAlgorithm_ctor(ChaseCameraAlgorithm*)`
/// (0x162318): the rider camera base class plus the chase-specific fields
/// (distance, speed windows, damping constants and the two curves).
pub fn chase_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0016_2330;
        /// `void func_0031D5E8(vec*)` (a three-float vector constructor).
        const VEC_CTOR: u32 = 0x0031_d5e8;
        c.cycles += 79;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        f.save_ra(c, 0);
        c.g[RA][0] = sx(RET);
        c.g[S0][0] = c.g[A0][0];
        callee::RIDER_CAMERA_ALGORITHM_CTOR_2.call(c, RET)?;
        let obj = c.g[S0][0] as u32;
        c.g[S1][0] = sx(obj.wrapping_add(0x340));
        c.g[V1][0] = 0x0046_0000;
        c.g[A0][0] = sx(obj.wrapping_add(0x304));
        c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x4440));
        c.g[RA][0] = sx(RET);
        c.sw(obj.wrapping_add(0x10), c.g[V1][0]);
        for off in [0x304u32, 0x310, 0x31c, 0x328, 0x334] {
            c.g[RA][0] = sx(RET);
            c.g[A0][0] = sx(obj.wrapping_add(off));
            call_ptr(c, VEC_CTOR, RET)?;
        }
        call_ptr(c, 0x0016_7d88, RET)?;
        c.swc1(0, obj.wrapping_add(0x380));
        call_ptr(c, 0x0016_7d98, RET)?;
        c.swc1(0, obj.wrapping_add(0x384));
        call_ptr(c, 0x0016_7da8, RET)?;
        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(RET);
        c.swc1(0, obj.wrapping_add(0x388));
        call_ptr(c, 0x0016_7db8, RET)?;
        c.g[V0][0] = 0x0050_0000;
        c.g[A0][0] = 0x54;
        c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0xed0));
        c.g[A1][0] = 0x0050_0000;
        c.lq(3, c.g[V0][0] as u32);
        c.g[A1][0] = sx(0x0050_0000u32.wrapping_sub(0xee0));
        c.sw(obj.wrapping_add(0x54), 0);
        c.g[A2][0] = 0x1388;
        c.sw(obj.wrapping_add(0xc), c.g[A0][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sq(3, obj.wrapping_add(0x40));
        c.sw(obj.wrapping_add(0x50), 0);
        c.sw(obj.wrapping_add(0x58), 0);
        c.lwc1(3, obj.wrapping_add(0x54));
        c.lq(3, c.g[A1][0] as u32);
        c.g[AT][0] = 0x4348_0000;
        c.mtc1(1, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 2);
        c.swc1(0, obj.wrapping_add(0x1d0));
        c.swc1(1, obj.wrapping_add(0x1dc));
        c.swc1(2, obj.wrapping_add(0x1e0));
        c.sq(3, obj.wrapping_add(0x90));
        c.sw(obj.wrapping_add(0x2c0), c.g[A2][0]);
        c.sw(obj.wrapping_add(0x1d4), 0);
        c.swc1(3, obj.wrapping_add(0x1d8));
        c.swc1(3, obj.wrapping_add(0x1e4));
        c.sw(obj.wrapping_add(0x2d0), 0);
        c.sw(obj.wrapping_add(0x2d8), 0);
        c.lwc1(0, gp(c, -0x6b08));
        c.lq(3, c.g[A1][0] as u32);
        c.lwc1(1, gp(c, -0x6b04));
        c.sq(3, obj.wrapping_add(0xc0));
        c.lwc1(2, gp(c, -0x6b00));
        c.lq(3, c.g[A1][0] as u32);
        f.restore(c, S1, 0x10);
        c.sq(3, obj.wrapping_add(0xd0));
        f.restore_ra(c, 0);
        c.lq(4, c.g[A1][0] as u32);
        c.swc1(0, obj);
        c.sq(4, obj.wrapping_add(0xe0));
        c.swc1(1, obj.wrapping_add(0x388));
        c.swc1(3, obj.wrapping_add(0x384));
        c.swc1(2, obj.wrapping_add(0x380));
        f.restore(c, S0, 0x20);
        f.close(c);
        Ok(())
    })
}


/// `int CameraTrigger_ReadAction(CameraTrigger* self, Action** out)`
/// (0x1712b0): read an action type word, allocate the matching action and
/// let it read its own fields; `*out` is the new action and the result is
/// the number of bytes consumed.
pub fn camera_trigger_read_action(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0017_12dc;
        const ALLOC: u32 = 0x0031_7d70;
        c.cycles += 16;
        let f = Frame::open(c, 0x70);
        f.save(c, S3, 0x30);
        f.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        f.save(c, S1, 0x50);
        f.save(c, S2, 0x40);
        f.save(c, S0, 0x60);
        f.save_ra(c, 0x10);
        c.g[RA][0] = sx(RET);
        c.g[A1][0] = sx(f.at(c, 0));
        call_ptr(c, 0x002d_2558, RET)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[S1][0] = Addr(f.at(c, 0)).lw(c);
        c.g[V0][0] = 1;
        let t1 = c.g[S1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
        if t1 {
            // Type 1: the long action (0x34 bytes).
            c.cycles += 41;
            c.g[A1][0] = 0x0046_0000;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x34;
            c.g[RA][0] = sx(RET);
            c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3f38));
            call_ptr(c, ALLOC, 0x0017_1388)?;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 3);
            c.g[V1][0] = 0x0046_0000;
            c.lwc1(4, gp(c, -0x643c));
            c.g[S0][0] = c.g[V0][0];
            c.g[AT][0] = 0x43c8_0000;
            c.mtc1(1, 2);
            c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x3e30));
            c.g[AT][0] = 0x41c8_0000;
            c.mtc1(1, 1);
            c.g[V0][0] = 0x0050_0000;
            c.g[AT][0] = 0x428c_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = c.g[S3][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V1][0]);
            c.g[A1][0] = c.g[S0][0];
            c.sw(c.g[S0][0] as u32, c.g[S1][0]);
            c.swc1(3, (c.g[S0][0] as u32).wrapping_add(0xc));
            c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x14));
            c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x10));
            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x1c));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x18));
            c.sw((c.g[S0][0] as u32).wrapping_add(8), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0);
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
            c.ldl(3, (c.g[T0][0] as u32).wrapping_add(7));
            c.ldr(3, c.g[T0][0] as u32);
            c.g[A2][0] = Addr((c.g[T0][0] as u32).wrapping_add(8)).lw(c);
            c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x2f));
            c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x28));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[A2][0]);
            c.g[RA][0] = sx(RET);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0);
            callee::TYPE1.call(c, 0x0017_140c)?;
            c.sw(c.g[S4][0] as u32, c.g[S0][0]);
        } else {
            c.cycles += 2;
            let ge2 = c.g[V0][0] == 0;
            c.g[V0][0] = 2;
            if ge2 {
                c.cycles += 2;
                let is2 = c.g[S1][0] == c.g[V0][0];
                c.g[V0][0] = 3;
                if is2 {
                    // Type 2: 0x20 bytes.
                    c.cycles += 27;
                    c.g[A1][0] = 0x0046_0000;
                    c.g[A0][0] = 0x20;
                    c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3f20));
                    c.g[A2][0] = 0;
                    c.g[RA][0] = sx(RET);
                    c.g[A3][0] = 0;
                    call_ptr(c, ALLOC, 0x0017_142c)?;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 1);
                    c.g[V1][0] = 0x0046_0000;
                    c.lwc1(2, gp(c, -0x6438));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[AT][0] = 0x4080_0000;
                    c.mtc1(1, 0);
                    c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x3e48));
                    c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V1][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                    c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x10));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[S0][0] as u32).wrapping_add(8), 0);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
                    c.g[RA][0] = sx(RET);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
                    callee::TYPE2.call(c, 0x0017_1478)?;
                    c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                } else {
                    c.cycles += 2;
                    let is3 = c.g[S1][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[S2][0];
                    if is3 {
                        // Type 3: the 8-byte action.
                        c.cycles += 15;
                        c.g[A1][0] = 0x0046_0000;
                        c.g[A0][0] = 8;
                        c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3f08));
                        c.g[A2][0] = 0;
                        c.g[RA][0] = sx(RET);
                        c.g[A3][0] = 0;
                        call_ptr(c, ALLOC, 0x0017_1498)?;
                        c.g[V1][0] = 0x0046_0000;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x3d18));
                        c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V1][0]);
                        c.g[A0][0] = c.g[S3][0];
                        c.g[RA][0] = sx(RET);
                        c.g[A1][0] = c.g[S0][0];
                        callee::TYPE3.call(c, 0x0017_14b8)?;
                        c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[S2][0];
                    }
                }
            } else {
                c.cycles += 2;
                let is0 = c.g[S1][0] == 0;
                c.g[V0][0] = c.g[S2][0];
                if is0 {
                    // Type 0: the 0x10-byte action.
                    c.cycles += 20;
                    c.g[A1][0] = 0x0046_0000;
                    c.g[A0][0] = 0x10;
                    c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x3f50));
                    c.g[A2][0] = 0;
                    c.g[RA][0] = sx(RET);
                    c.g[A3][0] = 0;
                    call_ptr(c, ALLOC, 0x0017_1338)?;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.g[V1][0] = 0x0046_0000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x3d18));
                    c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V1][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[A1][0] = c.g[S0][0];
                    c.sw(c.g[S0][0] as u32, 0);
                    c.g[RA][0] = sx(RET);
                    c.sw((c.g[S0][0] as u32).wrapping_add(8), 0);
                    callee::TYPE0.call(c, 0x0017_1368)?;
                    c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 7;
        f.restore(c, S1, 0x50);
        f.restore(c, S2, 0x40);
        f.restore(c, S3, 0x30);
        f.restore(c, S4, 0x20);
        f.restore_ra(c, 0x10);
        f.close(c);
        Ok(())
    })
}


/// `ActiveCameraTriggerList_Find` (first hit or 0).
pub(crate) const ACTIVE_FIND: u32 = 0x0016_c6b0;

/// `ActiveCameraTriggerList_Add`.
pub(crate) const ACTIVE_ADD: u32 = 0x0016_c718;

/// `CameraTriggerMan_CollectOctreeNodeTriggers` (the inlined subtree walk).
pub(crate) const COLLECT_NODE: u32 = 0x0017_6890;

/// The function itself (the recursion is a direct call of address 0x174848).
pub(crate) const IN_BOX: u32 = 0x0017_4848;


// ---------------------------------------------------------------------------
// Frame slots. The first four words double as the child descriptor the
// recursion receives in `t1`, so they are written by the box tests.

/// `t1`'s level - 1 (the child cell's level).
pub(crate) const CELL_TYPE: u32 = 0x00;

/// `2 * t1.u`.
pub(crate) const CELL_U: u32 = 0x04;

/// `2 * t1.v`.
pub(crate) const CELL_V: u32 = 0x08;

/// `2 * t1.w`.
pub(crate) const CELL_W: u32 = 0x0c;

/// The original `a0`.
pub(crate) const ARG0: u32 = 0x10;

/// The original `t0` (the node whose list and children are walked).
pub(crate) const NODE: u32 = 0x14;


// The twelve box/cell tests, in the order the original computes them:
// "the low/high half of the cell on this axis overlaps the box", then
// "the low/high half lies wholly inside the box".

pub(crate) const U_LO: u32 = 0x18;

pub(crate) const U_HI: u32 = 0x1c;

pub(crate) const V_LO: u32 = 0x20;

pub(crate) const V_HI: u32 = 0x24;

pub(crate) const W_LO: u32 = 0x28;

pub(crate) const W_HI: u32 = 0x2c;

pub(crate) const U_LO_IN: u32 = 0x30;

pub(crate) const U_HI_IN: u32 = 0x34;

pub(crate) const V_LO_IN: u32 = 0x38;

pub(crate) const V_HI_IN: u32 = 0x3c;

pub(crate) const W_LO_IN: u32 = 0x40;

pub(crate) const W_HI_IN: u32 = 0x44;


/// `lw` of a frame slot.
macro_rules! load_slot {
    ($c:ident, $f:ident, $off:expr) => {{
        let a = $f.at($c, $off);
        $c.lw(a)
    }};
}
pub(crate) use load_slot;


/// `sw` of a value into a frame slot.
macro_rules! store_slot {
    ($c:ident, $f:ident, $off:expr, $v:expr) => {{
        let a = $f.at($c, $off);
        let v: u64 = $v;
        $c.sw(a, v);
    }};
}
pub(crate) use store_slot;


/// One of the six "the half cell lies wholly inside the box" flags: the
/// original's `c_lt_s` pair with the box minimum, then with the box maximum
/// (4 cycles, +5/+1 when the flag is set).
macro_rules! inside_test {
    ($c:ident, $f:ident, $slot:ident, $box:ident, $lo_f:expr, $hi_f:expr, $max_off:expr,
     $cell_end_f:expr, $v:ident) => {{
        $c.cycles += 4;
        $c.c_lt_s($lo_f, $hi_f);
        let t = !$c.fcc();
        store_slot!($c, $f, $slot, 0);
        if !t {
            $c.cycles += 5;
            $c.lwc1(0, ($c.g[$box][0] as u32).wrapping_add($max_off));
            $c.c_lt_s($cell_end_f, 0);
            let t = !$c.fcc();
            $c.g[$v][0] = 1;
            if !t {
                $c.cycles += 1;
                store_slot!($c, $f, $slot, $c.g[$v][0]);
            }
        }
    }};
}
pub(crate) use inside_test;


/// One of the child cases 1..7: the null and overlap tests, then either the
/// "collect the whole subtree" path or a recursive descent.
///
/// The expansion ends after the recursive call, so the caller's block provides
/// the case's tail (the `+1` cycle and the result slot). The collect path sets
/// `$skip` and leaves, skipping that tail, as the original does.
macro_rules! child_case {
    (
        $c:ident, $f:ident, $skip:ident, $ctr:ident, $up:expr, $child:expr,
        $f0:ident, $f1:ident, $f2:ident, $i0:ident, $i1:ident, $i2:ident,
        $site:ident, $init:expr, $s6s7:expr, $flag:ident, $finish:expr
    ) => {{
        'done: {
            $c.cycles += 5;
            $c.g[V1][0] = load_slot!($c, $f, NODE);
            $c.g[V0][0] = sx(($c.g[V0][0] as u32).wrapping_add($up));
            let node = $c.g[V1][0] as u32;
            $c.g[S4][0] = Addr(node.wrapping_add($child)).lw($c);
            let t = $c.g[S4][0] == 0;
            let ctr = $c.g[V0][0];
            let a = $f.at($c, $ctr);
            $c.sw(a, ctr);
            if t {
                break 'done;
            }
            $c.cycles += 3;
            $c.g[V0][0] = load_slot!($c, $f, $f0);
            let t = $c.g[V0][0] == 0;
            $c.g[V1][0] = load_slot!($c, $f, $f1);
            if t {
                break 'done;
            }
            $c.cycles += 2;
            let t = $c.g[V1][0] == 0;
            $c.g[V0][0] = load_slot!($c, $f, $f2);
            if t {
                break 'done;
            }
            $c.cycles += 2;
            let t = $c.g[V0][0] == 0;
            $c.g[V1][0] = load_slot!($c, $f, $i0);
            if t {
                break 'done;
            }
            'outer: {
                'inner: {
                    $c.cycles += 2;
                    let t = $c.g[V1][0] == 0;
                    $c.g[V0][0] = load_slot!($c, $f, $i1);
                    if t {
                        break 'inner;
                    }
                    $c.cycles += 2;
                    let t = $c.g[V0][0] == 0;
                    $c.g[V1][0] = load_slot!($c, $f, $i2);
                    if t {
                        break 'inner;
                    }
                    $c.cycles += 2;
                    let t = $c.g[V1][0] == 0;
                    $c.g[A0][0] = load_slot!($c, $f, ARG0);
                    if t {
                        break 'outer;
                    }
                    collect_subtree($c, $f, S5, $flag, $s6s7, $init, &$site, $finish)?;
                    $skip = true;
                    break 'done;
                }
                $c.cycles += 1;
                $c.g[A0][0] = load_slot!($c, $f, ARG0);
            }
            descend($c, &$site)?;
        }
    }};
}
pub(crate) use child_case;


/// What the last child-collect block leaves behind (the original loads the
/// case's result slot in every child case, but restores `s0` in the last).
#[derive(Clone, Copy)]
pub(crate) enum Finish {
    /// `v0 = lw(sp + slot)`.
    LoadV0(u32),
    /// `lq 16, sp + 0xe0` (the epilogue's `s0` restore, hoisted into the case).
    RestoreS0,
}


/// One child site: the return address of every call the original makes there,
/// and the loop head events are delivered at.
pub(crate) struct Site {
    /// The trigger-list loop's head (`c.pc` for an event delivery).
    head: u32,
    /// Return addresses of the two virtual trigger tests.
    test1: u32,
    test2: u32,
    /// Return addresses of `ActiveCameraTriggerList_Find` / `_Add`.
    find: u32,
    add: u32,
    /// Return addresses of the eight `CollectOctreeNodeTriggers` calls.
    children: [u32; 8],
    /// Return address of the recursive `CollectOctreeTriggersInBox` call.
    next: u32,
}


/// The root node's own list (0x1748a0).
pub(crate) const SITE_TOP: Site = Site {
    head: 0x0017_48a0,
    test1: 0x0017_48dc,
    test2: 0x0017_48ec,
    find: 0x0017_4900,
    add: 0x0017_4918,
    children: [0; 8],
    next: 0,
};


/// Child +0x00 (u lo, v lo, w lo).
pub(crate) const SITE_A: Site = Site {
    head: 0x0017_4d08,
    test1: 0x0017_4d44,
    test2: 0x0017_4d54,
    find: 0x0017_4d68,
    add: 0x0017_4d80,
    children: [
        0x0017_4da0, 0x0017_4db4, 0x0017_4dc8, 0x0017_4ddc, 0x0017_4df0, 0x0017_4e04, 0x0017_4e18,
        0x0017_4e2c,
    ],
    next: 0x0017_4e50,
};


/// Child +0x04 (u lo, v lo, w hi).
pub(crate) const SITE_B: Site = Site {
    head: 0x0017_4eb0,
    test1: 0x0017_4eec,
    test2: 0x0017_4efc,
    find: 0x0017_4f10,
    add: 0x0017_4f28,
    children: [
        0x0017_4f48, 0x0017_4f5c, 0x0017_4f70, 0x0017_4f84, 0x0017_4f98, 0x0017_4fac, 0x0017_4fc0,
        0x0017_4fd4,
    ],
    next: 0x0017_4ff8,
};


/// Child +0x0c (u lo, v hi, w hi).
pub(crate) const SITE_C: Site = Site {
    head: 0x0017_5058,
    test1: 0x0017_5094,
    test2: 0x0017_50a4,
    find: 0x0017_50b8,
    add: 0x0017_50d0,
    children: [
        0x0017_50f0, 0x0017_5104, 0x0017_5118, 0x0017_512c, 0x0017_5140, 0x0017_5154, 0x0017_5168,
        0x0017_517c,
    ],
    next: 0x0017_51a0,
};


/// Child +0x08 (u lo, v hi, w lo).
pub(crate) const SITE_D: Site = Site {
    head: 0x0017_5200,
    test1: 0x0017_523c,
    test2: 0x0017_524c,
    find: 0x0017_5260,
    add: 0x0017_5278,
    children: [
        0x0017_5298, 0x0017_52ac, 0x0017_52c0, 0x0017_52d4, 0x0017_52e8, 0x0017_52fc, 0x0017_5310,
        0x0017_5324,
    ],
    next: 0x0017_5348,
};


/// Child +0x18 (u hi, v hi, w lo).
pub(crate) const SITE_E: Site = Site {
    head: 0x0017_53a8,
    test1: 0x0017_53e4,
    test2: 0x0017_53f4,
    find: 0x0017_5408,
    add: 0x0017_5420,
    children: [
        0x0017_5440, 0x0017_5454, 0x0017_5468, 0x0017_547c, 0x0017_5490, 0x0017_54a4, 0x0017_54b8,
        0x0017_54cc,
    ],
    next: 0x0017_54f0,
};


/// Child +0x1c (u hi, v hi, w hi).
pub(crate) const SITE_F: Site = Site {
    head: 0x0017_5550,
    test1: 0x0017_558c,
    test2: 0x0017_559c,
    find: 0x0017_55b0,
    add: 0x0017_55c8,
    children: [
        0x0017_55e8, 0x0017_55fc, 0x0017_5610, 0x0017_5624, 0x0017_5638, 0x0017_564c, 0x0017_5660,
        0x0017_5674,
    ],
    next: 0x0017_5698,
};


/// Child +0x14 (u hi, v lo, w hi).
pub(crate) const SITE_G: Site = Site {
    head: 0x0017_56f8,
    test1: 0x0017_5734,
    test2: 0x0017_5744,
    find: 0x0017_5758,
    add: 0x0017_5770,
    children: [
        0x0017_5790, 0x0017_57a4, 0x0017_57b8, 0x0017_57cc, 0x0017_57e0, 0x0017_57f4, 0x0017_5808,
        0x0017_581c,
    ],
    next: 0x0017_5840,
};


/// Child +0x10 (u hi, v lo, w lo): the last case, which leaves through the
/// epilogue instead of passing a result slot to a following case.
pub(crate) const SITE_H: Site = Site {
    head: 0x0017_58a8,
    test1: 0x0017_58e0,
    test2: 0x0017_58f0,
    find: 0x0017_5904,
    add: 0x0017_591c,
    children: [
        0x0017_593c, 0x0017_5950, 0x0017_5964, 0x0017_5978, 0x0017_598c, 0x0017_59a0, 0x0017_59b4,
        0x0017_59c8,
    ],
    next: 0x0017_59ec,
};


// ---------------------------------------------------------------------------
// The function.

/// `void CameraTriggerMan_CollectOctreeTriggersInBox(...)` (0x00174848).
pub fn camera_trigger_man_collect_octree_triggers_in_box(c: &mut Cpu) {
    guest(c, collect_in_box);
}


/// The body of [`camera_trigger_man_collect_octree_triggers_in_box`].
pub(crate) fn collect_in_box(c: &mut Cpu) -> Flow {
    c.cycles += 19;
    let frame = Frame::open(c, 0xf0);
    frame.save(c, S4, 0xa0);
    frame.save(c, S5, 0x90);
    c.g[S4][0] = c.g[T1][0];
    frame.save(c, S6, 0x80);
    c.g[S5][0] = c.g[A2][0];
    frame.save(c, S7, 0x70);
    c.g[S6][0] = c.g[A3][0];
    frame.save(c, S0, 0xe0);
    frame.save(c, S1, 0xd0);
    frame.save(c, S2, 0xc0);
    frame.save(c, FP, 0x60);
    frame.save_ra(c, 0x50);
    let node = c.g[T0][0];
    store_slot!(c, frame, NODE, node);
    let arg0 = c.g[A0][0];
    store_slot!(c, frame, ARG0, arg0);
    frame.save(c, S3, 0xb0);
    c.g[S3][0] = Addr(node as u32).off(0x28).lw(c);
    let empty = c.g[S3][0] == 0;
    // `s7` keeps the `a1` manager: the root walk uses it, the child walks use
    // the `a2` one (`s5`).
    c.g[S7][0] = c.g[A1][0];
    if !empty {
        walk_trigger_list(c, S3, S2, S7, FP, false, 3, &SITE_TOP, false)?;
    }

    'epilogue: {
        'boxed: {
            c.cycles += 4;
            c.g[A1][0] = Addr(c.g[S4][0] as u32).lw(c);
            c.g[V0][0] = 0xb;
            let t = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e));
            if t {
                break 'boxed;
            }

            // ---- the cell's size and its doubled coordinates (0x1749bc):
            //      size = 2^(level-1), and the frame's descriptor words for
            //      the child cell.
            c.cycles += 28;
            c.g[A0][0] = Addr(c.g[S4][0] as u32).off(4).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x631c));
            c.mtc1(2, 4); // f4 = the cell size
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.mtc1(4, 0); // f0 = 2u
            c.cvt_s_w(0, 0);
            c.mtc1(2, 1); // f1 = 2u + 1
            c.cvt_s_w(1, 1);
            c.lwc1(3, c.g[S6][0] as u32);
            c.g[V1][0] = Addr(c.g[S4][0] as u32).off(0xc).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = Addr(c.g[S4][0] as u32).off(8).lw(c);
            c.sub_s(0, 0, 2); // f0 = 2u - e
            c.add_s(1, 1, 2); // f1 = 2u + 1 + e
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            store_slot!(c, frame, CELL_TYPE, c.g[A1][0]);
            store_slot!(c, frame, U_LO, 0);
            c.mul_s(15, 0, 4); // f15 = low edge of the low half
            c.mul_s(16, 1, 4); // f16 = high edge of the low half
            store_slot!(c, frame, CELL_V, c.g[V0][0]);
            store_slot!(c, frame, CELL_W, c.g[V1][0]);
            c.c_lt_s(3, 16);
            let t = !c.fcc();
            store_slot!(c, frame, CELL_U, c.g[A0][0]);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x10));
                c.c_lt_s(15, 0);
                let t = !c.fcc();
                c.g[V0][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, U_LO, c.g[V0][0]);
                }
            }

            // ---- the high half of u (0x174a1c)
            c.cycles += 18;
            c.g[V0][0] = load_slot!(c, frame, CELL_U);
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x6318));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
            store_slot!(c, frame, U_HI, 0);
            c.mtc1(3, 1); // f1 = 2u + 2
            c.cvt_s_w(1, 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.mtc1(2, 0); // f0 = 2u + 1
            c.cvt_s_w(0, 0);
            c.lwc1(2, c.g[S6][0] as u32);
            c.add_s(1, 1, 3);
            c.mov_s(13, 2); // f13 = box.min.u
            c.sub_s(0, 0, 3);
            c.mul_s(14, 1, 4);
            c.c_lt_s(2, 14);
            let t = !c.fcc();
            c.mul_s(12, 0, 4);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x10));
                c.c_lt_s(12, 0);
                let t = !c.fcc();
                c.g[V1][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, U_HI, c.g[V1][0]);
                }
            }

            // ---- neither half of u overlaps the box: nothing to do (0x174a34)
            c.cycles += 3;
            c.g[V0][0] = load_slot!(c, frame, U_LO);
            let t = c.g[V0][0] != 0;
            c.g[V1][0] = load_slot!(c, frame, CELL_V);
            if !t {
                c.cycles += 3;
                c.g[V1][0] = load_slot!(c, frame, U_HI);
                let t = c.g[V1][0] == 0;
                c.g[V1][0] = load_slot!(c, frame, CELL_V);
                if t {
                    break 'boxed;
                }
            }

            // ---- the low half of v (0x174a90)
            c.cycles += 17;
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x6314));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            store_slot!(c, frame, V_LO, 0);
            c.mtc1(2, 1); // f1 = 2v + 1
            c.cvt_s_w(1, 1);
            c.g[A0][0] = c.g[V1][0];
            c.mtc1(3, 0); // f0 = 2v
            c.cvt_s_w(0, 0);
            c.lwc1(2, (c.g[S6][0] as u32).wrapping_add(0x4));
            c.add_s(1, 1, 3);
            c.mov_s(6, 2); // f6 = box.min.v
            c.sub_s(0, 0, 3);
            c.mul_s(11, 1, 4);
            c.c_lt_s(2, 11);
            let t = !c.fcc();
            c.mul_s(10, 0, 4);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x14));
                c.c_lt_s(10, 0);
                let t = !c.fcc();
                c.g[V0][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, V_LO, c.g[V0][0]);
                }
            }

            // ---- the high half of v (0x174ae4)
            c.cycles += 15;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6310));
            c.mtc1(3, 1); // f1 = 2v + 2
            c.cvt_s_w(1, 1);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.mtc1(2, 0); // f0 = 2v + 1
            c.cvt_s_w(0, 0);
            store_slot!(c, frame, V_HI, 0);
            c.add_s(1, 1, 2);
            c.sub_s(0, 0, 2);
            c.mul_s(9, 1, 4);
            c.c_lt_s(6, 9);
            let t = !c.fcc();
            c.mul_s(8, 0, 4);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x14));
                c.c_lt_s(8, 0);
                let t = !c.fcc();
                c.g[V1][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, V_HI, c.g[V1][0]);
                }
            }

            // ---- neither half of v overlaps the box (0x174afc)
            c.cycles += 3;
            c.g[V0][0] = load_slot!(c, frame, V_LO);
            let t = c.g[V0][0] != 0;
            c.g[V1][0] = load_slot!(c, frame, CELL_W);
            if !t {
                c.cycles += 3;
                c.g[V1][0] = load_slot!(c, frame, V_HI);
                let t = c.g[V1][0] == 0;
                c.g[V1][0] = load_slot!(c, frame, CELL_W);
                if t {
                    break 'boxed;
                }
            }

            // ---- the low half of w (0x174b58)
            c.cycles += 17;
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x630c));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            store_slot!(c, frame, W_LO, 0);
            c.mtc1(2, 1); // f1 = 2w + 1
            c.cvt_s_w(1, 1);
            c.g[A0][0] = c.g[V1][0];
            c.mtc1(3, 0); // f0 = 2w
            c.cvt_s_w(0, 0);
            c.lwc1(2, (c.g[S6][0] as u32).wrapping_add(0x8));
            c.add_s(1, 1, 3);
            c.mov_s(5, 2); // f5 = box.min.w
            c.sub_s(0, 0, 3);
            c.mul_s(7, 1, 4);
            c.c_lt_s(2, 7);
            let t = !c.fcc();
            c.mul_s(3, 0, 4);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x18));
                c.c_lt_s(3, 0);
                let t = !c.fcc();
                c.g[V0][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, W_LO, c.g[V0][0]);
                }
            }

            // ---- the high half of w (0x174bac)
            c.cycles += 15;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6308));
            c.mtc1(3, 1); // f1 = 2w + 2
            c.cvt_s_w(1, 1);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.mtc1(2, 0); // f0 = 2w + 1
            c.cvt_s_w(0, 0);
            store_slot!(c, frame, W_HI, 0);
            c.add_s(1, 1, 2);
            c.sub_s(0, 0, 2);
            c.mul_s(1, 1, 4);
            c.c_lt_s(5, 1);
            let t = !c.fcc();
            c.mul_s(2, 0, 4);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x18));
                c.c_lt_s(2, 0);
                let t = !c.fcc();
                c.g[V1][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, W_HI, c.g[V1][0]);
                }
            }

            // ---- neither half of w overlaps the box: the query is empty
            //      (0x174bc0; jumps straight to the epilogue).
            c.cycles += 3;
            c.g[V0][0] = load_slot!(c, frame, W_LO);
            let t = c.g[V0][0] != 0;
            c.g[V1][0] = load_slot!(c, frame, W_HI);
            if !t {
                c.cycles += 2;
                let t = c.g[V1][0] == 0;
                frame.restore(c, S0, 0xe0);
                if t {
                    break 'epilogue;
                }
            }

            // ---- the six "half cell wholly inside the box" tests
            //      (0x174be8 .. 0x174cb0).
            inside_test!(c, frame, U_LO_IN, S6, 13, 15, 0x10, 16, V0);
            inside_test!(c, frame, U_HI_IN, S6, 13, 12, 0x10, 14, V1);
            inside_test!(c, frame, V_LO_IN, S6, 6, 10, 0x14, 11, V0);
            inside_test!(c, frame, V_HI_IN, S6, 6, 8, 0x14, 9, V1);
            inside_test!(c, frame, W_LO_IN, S6, 5, 3, 0x18, 7, V0);
            inside_test!(c, frame, W_HI_IN, S6, 5, 2, 0x18, 1, V1);

            // ---- the eight children, in the original's visiting order.
            //      Child 0 (0x174e54) loads its flags in a slightly different
            //      order and keeps no counter, so it is written out.
            'case_a: {
                'done: {
                    c.cycles += 4;
                    c.g[V0][0] = load_slot!(c, frame, NODE);
                    let n = c.g[V0][0] as u32;
                    c.g[S4][0] = Addr(n).lw(c);
                    let t = c.g[S4][0] == 0;
                    c.g[V1][0] = load_slot!(c, frame, U_LO);
                    if t {
                        break 'done;
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0;
                    c.g[V0][0] = load_slot!(c, frame, V_LO);
                    if t {
                        break 'done;
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0;
                    c.g[V1][0] = load_slot!(c, frame, W_LO);
                    if t {
                        break 'done;
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0;
                    c.g[V0][0] = load_slot!(c, frame, U_LO_IN);
                    if t {
                        break 'done;
                    }
                    'outer: {
                        'inner: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0;
                            c.g[V1][0] = load_slot!(c, frame, V_LO_IN);
                            if t {
                                break 'inner;
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] == 0;
                            c.g[V0][0] = load_slot!(c, frame, W_LO_IN);
                            if t {
                                break 'inner;
                            }
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0;
                            c.g[A0][0] = load_slot!(c, frame, ARG0);
                            if t {
                                break 'outer;
                            }
                            collect_subtree(
                                c,
                                frame,
                                S5,
                                FP,
                                false,
                                3,
                                &SITE_A,
                                Finish::LoadV0(CELL_W),
                            )?;
                            break 'case_a;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = load_slot!(c, frame, ARG0);
                    }
                    descend(c, &SITE_A)?;
                }
                c.cycles += 1;
                c.g[V0][0] = load_slot!(c, frame, CELL_W);
            }

            'case_b: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_W, 1, 0x04, U_LO, V_LO, W_HI, U_LO_IN, V_LO_IN,
                    W_HI_IN, SITE_B, 2, false, FP, Finish::LoadV0(CELL_V)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_V);
                }
            }

            'case_c: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_V, 1, 0x0c, U_LO, V_HI, W_HI, U_LO_IN, V_HI_IN,
                    W_HI_IN, SITE_C, 2, false, FP, Finish::LoadV0(CELL_W)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_W);
                }
            }

            'case_d: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_W, 0xffff_ffff, 0x08, U_LO, V_HI, W_LO, U_LO_IN,
                    V_HI_IN, W_LO_IN, SITE_D, 2, false, FP, Finish::LoadV0(CELL_U)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_U);
                }
            }

            'case_e: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_U, 1, 0x18, U_HI, V_HI, W_LO, U_HI_IN, V_HI_IN,
                    W_LO_IN, SITE_E, 2, false, FP, Finish::LoadV0(CELL_W)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_W);
                }
            }

            'case_f: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_W, 1, 0x1c, U_HI, V_HI, W_HI, U_HI_IN, V_HI_IN,
                    W_HI_IN, SITE_F, 2, false, FP, Finish::LoadV0(CELL_V)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_V);
                }
            }

            'case_g: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_V, 0xffff_ffff, 0x14, U_HI, V_LO, W_HI, U_HI_IN,
                    V_LO_IN, W_HI_IN, SITE_G, 2, false, FP, Finish::LoadV0(CELL_W)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_W);
                }
            }

            // The last case has no result slot of its own. Its early exits and
            // its descend end in the epilogue's `+1; lq 16` funnel, but its
            // collect path jumps *past* that funnel: the final child check
            // restores `s0` itself (the original's `break 'b_1759f0`).
            let mut skip = false;
            child_case!(
                c, frame, skip, CELL_W, 0xffff_ffff, 0x10, U_HI, V_LO, W_LO, U_HI_IN, V_LO_IN,
                W_LO_IN, SITE_H, 4, true, S6, Finish::RestoreS0
            );
            if skip {
                break 'epilogue;
            }
            break 'boxed;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0xe0);
    }

    c.cycles += 11;
    frame.restore(c, S1, 0xd0);
    frame.restore(c, S2, 0xc0);
    frame.restore(c, S3, 0xb0);
    frame.restore(c, S4, 0xa0);
    frame.restore(c, S5, 0x90);
    frame.restore(c, S6, 0x80);
    frame.restore(c, S7, 0x70);
    frame.restore(c, FP, 0x60);
    frame.restore_ra(c, 0x50);
    frame.close(c);
    Ok(())
}


/// The recursive descent into a child cell (0x174e50 and its siblings).
pub(crate) fn descend(c: &mut Cpu, site: &Site) -> Flow {
    c.cycles += 6;
    c.g[T0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S7][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = c.g[S6][0];
    c.g[RA][0] = sx(site.next);
    c.g[T1][0] = c.g[SP][0];
    c.pc = IN_BOX;
    ee::s02::CameraTriggerMan_CollectOctreeTriggersInBox(c);
    if c.pc != site.next {
        return Err(Unwound);
    }
    Ok(())
}


/// The "child cell wholly inside the box" path (0x174d90 and siblings):
/// collect the node's own triggers, then everything under its eight children
/// (the compiler inlined `CollectOctreeNodeTriggers` here).
pub(crate) fn collect_subtree(
    c: &mut Cpu,
    frame: Frame,
    mgr: usize,
    flag: usize,
    s6s7: bool,
    init_cycles: u64,
    site: &Site,
    finish: Finish,
) -> Flow {
    c.cycles += 3;
    let node = c.g[S4][0] as u32;
    c.g[S2][0] = Addr(node).off(0x28).lw(c);
    if c.g[S2][0] == 0 {
        c.g[A0][0] = Addr(node).lw(c);
    } else {
        walk_trigger_list(c, S2, S3, mgr, flag, s6s7, init_cycles, site, true)?;
    }
    collect_child_subtrees(c, frame, mgr, &site.children, finish)
}


/// `CollectOctreeNodeTriggers`'s tail: call it for each non-null child of the
/// node in `s4` (the first entry is already in `a0`), each with its own return
/// address.
pub(crate) fn collect_child_subtrees(
    c: &mut Cpu,
    frame: Frame,
    mgr: usize,
    rets: &[u32; 8],
    finish: Finish,
) -> Flow {
    let node = c.g[S4][0] as u32;
    for (i, ret) in rets.iter().enumerate().take(7) {
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(*ret);
            c.g[A1][0] = c.g[mgr][0];
            c.pc = COLLECT_NODE;
            ee::s02::f_00176890(c);
            if c.pc != *ret {
                return Err(Unwound);
            }
        }
        let off = 4 * (i as u32 + 1);
        c.g[A0][0] = Addr(node.wrapping_add(off)).lw(c);
    }
    c.cycles += 2;
    let t = c.g[A0][0] == 0;
    match finish {
        Finish::LoadV0(off) => {
            c.g[V0][0] = load_slot!(c, frame, off);
        }
        Finish::RestoreS0 => frame.restore(c, S0, 0xe0),
    }
    if !t {
        c.cycles += 4;
        c.g[RA][0] = sx(rets[7]);
        c.g[A1][0] = c.g[mgr][0];
        c.pc = COLLECT_NODE;
        ee::s02::f_00176890(c);
        if c.pc != rets[7] {
            return Err(Unwound);
        }
        match finish {
            Finish::LoadV0(off) => {
                c.g[V0][0] = load_slot!(c, frame, off);
            }
            Finish::RestoreS0 => frame.restore(c, S0, 0xe0),
        }
    }
    Ok(())
}


/// One walk of a node's trigger list (the loop at 0x1748a0 / 0x174d08 and its
/// siblings): for every entry of type 4, run the object's two virtual tests
/// and, if both pass, mark the trigger active in the manager's list (or add it
/// there when it is not listed yet).
///
/// `list` is the register holding the entry cursor and `obj` the one the
/// trigger object is loaded into — the root walk keeps the cursor in `s3` and
/// the object in `s2`, the child walks the other way round. `mgr` is the
/// manager's register (`s7` at the root, `s5` below) and `flag` the register
/// holding 1 (the "active" marker); `s6s7` is the last site's variant, which
/// compares against `s7` and keeps `v1` untouched.
pub(crate) fn walk_trigger_list(
    c: &mut Cpu,
    list: usize,
    obj: usize,
    mgr: usize,
    flag: usize,
    s6s7: bool,
    init_cycles: u64,
    site: &Site,
    exit_loads_a0: bool,
) -> Flow {
    c.cycles += init_cycles;
    if s6s7 {
        c.g[S7][0] = 4;
        c.g[S6][0] = 1;
    } else {
        c.g[FP][0] = 1;
    }
    let p = c.g[list][0] as u32;
    c.g[V0][0] = Addr(p).off(8).lw(c);
    loop {
        let trigger = if s6s7 {
            c.cycles += 2;
            c.g[V0][0] == c.g[S7][0]
        } else {
            c.cycles += 3;
            c.g[V1][0] = 4;
            c.g[V0][0] == c.g[V1][0]
        };
        if trigger {
            c.cycles += 18;
            let entry = c.g[list][0] as u32;
            c.g[obj][0] = Addr(entry).off(0xc).lw(c);
            let m = c.g[mgr][0] as u32;
            c.g[A0][0] = Addr(m).off(4).lw(c);
            c.g[S0][0] = Addr(c.g[obj][0] as u32).off(8).lw(c);
            c.g[A2][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
            c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x28).lh(c);
            c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x38).lh(c);
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x2c).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(site.test1);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.pc = target;
            call_indirect(c, target);
            if c.pc != site.test1 {
                return Err(Unwound);
            }
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x3c).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(site.test2);
            c.g[A1][0] = c.g[V0][0];
            c.pc = target;
            call_indirect(c, target);
            if c.pc != site.test2 {
                return Err(Unwound);
            }
            if c.g[V0][0] == 0 {
                let q = c.g[list][0] as u32;
                c.g[list][0] = Addr(q).lw(c);
            } else {
                'added: {
                    'found: {
                        c.cycles += 5;
                        let m = c.g[mgr][0] as u32;
                        c.g[A0][0] = Addr(m).lw(c);
                        c.g[RA][0] = sx(site.find);
                        c.g[A1][0] = c.g[obj][0];
                        c.pc = ACTIVE_FIND;
                        ee::s02::f_0016c6b0(c);
                        if c.pc != site.find {
                            return Err(Unwound);
                        }
                        if c.g[V0][0] == 0 {
                            let m = c.g[mgr][0] as u32;
                            c.g[A0][0] = Addr(m).lw(c);
                            break 'found;
                        }
                        c.cycles += 2;
                        let p = c.g[V0][0] as u32;
                        let v = c.g[flag][0];
                        Addr(p).off(0xc).sw(c, v);
                        break 'added;
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(site.add);
                    c.g[A1][0] = c.g[obj][0];
                    c.pc = ACTIVE_ADD;
                    ee::s02::f_0016c718(c);
                    if c.pc != site.add {
                        return Err(Unwound);
                    }
                }
                c.cycles += 1;
                let q = c.g[list][0] as u32;
                c.g[list][0] = Addr(q).lw(c);
            }
        } else {
            let q = c.g[list][0] as u32;
            c.g[list][0] = Addr(q).lw(c);
        }
        c.cycles += 2;
        if c.g[list][0] == 0 {
            // The child sites charge one more cycle and reload `a0` from the
            // node they walked; the root one just leaves the loop.
            if exit_loads_a0 {
                c.cycles += 1;
                let n = c.g[S4][0] as u32;
                c.g[A0][0] = Addr(n).lw(c);
            }
            return Ok(());
        }
        let q = c.g[list][0] as u32;
        c.g[V0][0] = Addr(q).off(8).lw(c);
        if event_due(c) {
            deliver_event_here(c, site.head);
        }
    }
}


/// `void func_00176890(void* node, void* ctx)` (0x176890): walk an octree node
/// (children at_4 `+0..+0x1c`, triggers at_4 `+0x28`), activating every camera
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
        c.g[S2][0] = lw(c, at_4(c, S5, 0x28));
        let t = c.g[S2][0] == 0u64;
        c.g[S4][0] = c.g[A1][0];
        if t {
            break 'b_176958;
        } else {
            c.cycles += 4;
            c.g[S7][0] = 0x4;
            c.g[S6][0] = 0x1;
            c.g[V0][0] = lw(c, at_4(c, S2, 0x8));
            'l_1768d8: loop {
                'b_176950: {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S7][0] {
                        c.g[S2][0] = lw(c, r32(c, S2));
                        break 'b_176950;
                    } else {
                        c.cycles += 18;
                        c.g[S3][0] = lw(c, at_4(c, S2, 0xc));
                        c.g[A0][0] = lw(c, at_4(c, S4, 0x4));
                        c.g[S0][0] = lw(c, at_4(c, S3, 0x8));
                        c.g[A2][0] = lw(c, at_4(c, A0, 0x6c0));
                        c.g[S1][0] = lw(c, at_4(c, S0, 0x24));
                        c.g[A0][0] = sx(at_4(c, A0, 0x6c0));
                        c.g[V1][0] = lh(c, at_4(c, A2, 0x28));
                        c.g[A1][0] = lh(c, at_4(c, S1, 0x38));
                        c.g[V0][0] = lw(c, at_4(c, A2, 0x2c));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x00176910);
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, A1)));
                        if call_ptr(c, t, 0x00176910).is_err() {
                            return;
                        }
                        c.g[V1][0] = lw(c, at_4(c, S1, 0x3c));
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
                                        sw_2(c, at_4(c, V0, 0xc), c.g[S6][0]);
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
                    c.g[V0][0] = lw(c, at_4(c, S2, 0x8));
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
            c.g[A0][0] = lw(c, at_4(c, S5, 0x4));
            break 'b_176970;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0017696c);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x0017696c).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at_4(c, S5, 0x4));
            break 'b_176970;
        }
    }
    'b_176984: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at_4(c, S5, 0x8));
            break 'b_176984;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x00176980);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x00176980).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at_4(c, S5, 0x8));
            break 'b_176984;
        }
    }
    'b_176998: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at_4(c, S5, 0xc));
            break 'b_176998;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x00176994);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x00176994).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at_4(c, S5, 0xc));
            break 'b_176998;
        }
    }
    'b_1769ac: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at_4(c, S5, 0x10));
            break 'b_1769ac;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001769a8);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x001769a8).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at_4(c, S5, 0x10));
            break 'b_1769ac;
        }
    }
    'b_1769c0: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at_4(c, S5, 0x14));
            break 'b_1769c0;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001769bc);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x001769bc).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at_4(c, S5, 0x14));
            break 'b_1769c0;
        }
    }
    'b_1769d4: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at_4(c, S5, 0x18));
            break 'b_1769d4;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001769d0);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x001769d0).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at_4(c, S5, 0x18));
            break 'b_1769d4;
        }
    }
    'b_1769e8: {
        c.cycles += 2;
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at_4(c, S5, 0x1c));
            break 'b_1769e8;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001769e4);
            c.g[A1][0] = c.g[S4][0];
            if callee::C00176890.call(c, 0x001769e4).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at_4(c, S5, 0x1c));
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
    c.g[SP][0] = sx(at_4(c, SP, 0x90));
    c.pc = t;
    return;
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
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::put_vec;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::events::Events;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use ssx3_ee as ee;
    use crate::difftest::RA as RET;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::compare;
    use ssx3_ee_rt::funcs::GuestFn;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::ai::aianimeventmap::tests::MEM_MUTEX;
    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;
    pub(crate) use crate::game::dirtysock::tags2::tests::MEM_HEAP;
    pub(crate) use crate::game::dirtysock::tags2::tests::run;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_init;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_block;
    pub(crate) use crate::game::camera::camera::tests::algo_setup;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::ui::uivector::tests::start;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festateoptions::tests::rng;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::wscript::wscriptman::tests::w;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_3;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4_2 as OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE_DUMMY;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base_14 as base_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::check3;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::AUX as AUX_2;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;
    pub(crate) use crate::game::camera::camera::mid_left_chase_camera_reset_for_rider;
    pub(crate) use crate::game::fe::fepopup::c_ui_state_buy_popup_ctor;
    pub(crate) use crate::game::fe::festaterewards2::c_fe_state_trophy_room_show_trophy_info;
    pub(crate) use crate::game::main::loadscreens_prestart::pre_start_screen_render;
    pub(crate) use crate::game::main::loadscreens_prestart::init_load_screen_render;
    pub(crate) use crate::game::main::loadscreens_prestart::c_pre_game_load_screen_render;
    pub(crate) use crate::game::main::loadscreens_prestart::c_pre_fe_load_screen_render;


    #[test]
    fn camera_trigger_read_action_type3_matches() {
        check(ssx3_ee::s02::f_00171680, camera_trigger_read_action_type3, 64, &|c, seed| {
            let mut r = Rng(0x1716 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (DATA + 0x200) as u64;
        });
    }


    #[test]
    fn vec3_max_component_matches() {
        check(ssx3_ee::s02::f_001731c0, vec3_max_component, 300, &|c, seed| {
            let mut r = Rng(0x1731 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for k in 0..3 {
                c.bus.write32(DATA + k * 4, r.edge_f32());
            }
        });
    }


    #[test]
    fn debug_menu_init_background_list_matches() {
        check(ssx3_ee::s02::f_0017abc8, debug_menu_init_background_list, 64, &|c, seed| {
            let mut r = Rng(0x17ab ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
        });
    }


    #[test]
    fn debug_menu_select_background_matches() {
        check(ssx3_ee::s02::f_0017ac98, debug_menu_select_background, 300, &|c, seed| {
            let mut r = Rng(0x17ac ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = match seed % 3 {
                0 => 0xffff_ffff_ffff_ffff,
                _ => (r.next_u32() % 30) as u64,
            };
            c.bus.write32(DATA + 4, r.next_u32() % 30);
        });
    }


    #[test]
    fn algorithmic_script_camera_algo_dtor_matches() {
        check(ee::s02::f_001763f8, algorithmic_script_camera_algo_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x63f8, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    pub(crate) const ONE: u32 = 0x3f80_0000;

    /// The 4x4 identity matrix the camera code copies before building.
    pub(crate) const IDENTITY: u32 = 0x004f_f1a0;

    pub(crate) const AXIS_X: u32 = 0x004f_f140;

    pub(crate) const AXIS_Y: u32 = 0x004f_f150;

    pub(crate) const AXIS_Z: u32 = 0x004f_f160;


    pub(crate) fn axes(c: &mut Cpu) {
        put_vec(c, IDENTITY, [ONE, 0, 0, 0]);
        put_vec(c, IDENTITY + 0x10, [0, ONE, 0, 0]);
        put_vec(c, IDENTITY + 0x20, [0, 0, ONE, 0]);
        put_vec(c, IDENTITY + 0x30, [0, 0, 0, ONE]);
        put_vec(c, AXIS_X, [ONE, 0, 0, 0]);
        put_vec(c, AXIS_Y, [0, ONE, 0, 0]);
        put_vec(c, AXIS_Z, [0, 0, ONE, 0]);
    }


    #[test]
    fn camera_sphere_trigger_contains_point_matches() {
        check(
            ssx3_ee::s02::f_00172278__generated,
            camera_sphere_trigger_contains_point,
            200,
            &|c, seed| {
                let mut r = Rng(0x1722_7801 ^ seed as u64);
                scramble(c, &mut r);
                axes(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x40);
                // Position, angles and radii of the trigger.
                for i in 0..6 {
                    c.bus.write32(DATA + 4 * i, r.edge_f32());
                }
                for i in 6..8 {
                    c.bus.write32(DATA + 4 * i, r.f32(2.0));
                }
                c.bus.write32(DATA + 0x18, (1.0f32 + (seed % 4) as f32).to_bits());
                c.bus.write32(DATA + 0x1c, (1.0f32 + (seed % 3) as f32).to_bits());
                c.bus.write32(DATA + 0x20, (1.0f32 + (seed % 5) as f32).to_bits());
                // The probe either sits on the trigger or far outside it, so
                // both results of the final compare are covered.
                let s: f32 = if seed % 2 == 0 { 0.25 } else { 50.0 };
                put_vec(c, DATA + 0x40, [s.to_bits(), 0, 0, ONE]);
                c.pc = 0x0017_2278;
            },
        );
    }


    /// The two camera-trigger array element destructors: the member pointer is
    /// either null or a block from the test heap (which `operator_delete`
    /// legitimately releases).
    pub(crate) fn trigger_array_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = machine(c, seed, salt, 0);
        let obj = DATA + 0x3000;
        c.g[A0][0] = sx(obj);
        let member = if seed % 2 == 0 { 0 } else { heap_block(c, 0x40) };
        c.bus.write32(obj + 8, member);
    }


    #[test]
    fn camera_trigger_array_45c1d0_v01_matches() {
        check(ssx3_ee::s02::f_00176840__generated, camera_trigger_array_45c1d0_v01, 80, &|c, seed| {
            trigger_array_setup(c, seed, 0x15)
        });
    }


    #[test]
    fn camera_trigger_array_45c1b8_v01_matches() {
        check(ssx3_ee::s02::f_00176868__generated, camera_trigger_array_45c1b8_v01, 80, &|c, seed| {
            trigger_array_setup(c, seed, 0x16)
        });
    }


    #[test]
    fn default_2_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_00176a68__generated, default_2_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x176a_68);
        });
    }


    #[test]
    fn default_3_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_00176d68__generated, default_3_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x176d_68);
        });
    }


    #[test]
    fn default_4_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_00177068__generated, default_4_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x1770_68);
        });
    }


    #[test]
    fn spoke_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_00177e50__generated, spoke_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x177e_50);
        });
    }


    #[test]
    fn bounded_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s02::f_00174190__generated,
            bounded_camera_algorithm_ctor,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x174190, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }


    #[test]
    fn camera_trigger_read_action_type0_matches() {
        check(
            ee::s02::f_001715a0__generated,
            camera_trigger_read_action_type0,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1715_a0, 0);
                alloc_ready(c);
                let reader = DATA + 0x800;
                let dst = DATA + 0x100;
                let bytes = DATA + 0x900;
                clear(c, dst, 0x40);
                clear(c, reader, 0x20);
                for i in 0..0x40 {
                    c.bus.write32(bytes + 4 * i, r.next_u32());
                }
                c.bus.write32(reader, bytes);
                c.g[A0][0] = sx(reader);
                c.g[A1][0] = sx(dst);
            },
        );
    }


    #[test]
    fn direction_ne_chase_camera_algorithm_ctor_matches() {
        check(
            ee::s02::f_001774c0__generated,
            direction_ne_chase_camera_algorithm_ctor,
            SEEDS_3,
            &|c, seed| {
                let mut r = common(c, seed, 0x1774c0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (r.next_u32() % 64) as u64;
            },
        );
    }


    #[test]
    fn debug_menu_is_track_selectable_matches() {
        check(ee::s02::f_00179fc8__generated, debug_menu_is_track_selectable, SEEDS_3, &|c, seed| {
            let mut r = common(c, seed, 0x179fc8, 3);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = if seed % 2 == 0 {
                (r.next_u32() % 24) as u64
            } else {
                r.next_u32() as u64
            };
        });
    }


    /// One shared test body for the `GetTuning` writers.
    pub(crate) fn check_tuning(hand: GuestFn, generated: GuestFn, base: i32) {
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
    pub(crate) fn check_eye_drop(hand: GuestFn, generated: GuestFn, base: i32) {
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


    /// A `gp`-relative global word.
    pub(crate) fn gpf(c: &mut Cpu, off: i32, v: u32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), v);
    }


    // ------------------------------------------------------------------
    // Chase cameras.

    #[test]
    fn spoke_chase_camera_get_tuning_matches() {
        check(ee::s02::f_00177ec8__generated, spoke_chase_camera_get_tuning, 200, &|c, seed| {
            setup(c, seed, 1);
            c.g[A1][0] = AUX as u64;
            gpf(c, -0x5e1c, 0x3f80_0000u32.wrapping_add(seed));
            gpf(c, -0x5e18, 0x4000_0000u32.wrapping_add(seed));
            gpf(c, -0x5e14, 0x4040_0000u32.wrapping_add(seed));
            gpf(c, -0x5e10, 0x4080_0000u32.wrapping_add(seed));
        });
    }


    #[test]
    fn superpipe_chase_camera_get_eye_drop_matches() {
        check(ee::s02::f_001785b0__generated, superpipe_chase_camera_get_eye_drop, 200, &|c, seed| {
            let mut r = setup(c, seed, 2);
            gpf(c, -0x5dc0, r.f32(50.0));
            c.f[0] = r.f32(5.0);
        });
    }


    #[test]
    fn superpipe_chase_camera_get_tuning_matches() {
        check(ee::s02::f_001785c0__generated, superpipe_chase_camera_get_tuning, 200, &|c, seed| {
            setup(c, seed, 3);
            c.g[A1][0] = AUX as u64;
            gpf(c, -0x5dbc, 0x3f80_0000u32.wrapping_add(seed));
            gpf(c, -0x5db8, 0x4000_0000u32.wrapping_add(seed));
            gpf(c, -0x5db4, 0x4040_0000u32.wrapping_add(seed));
            gpf(c, -0x5db0, 0x4080_0000u32.wrapping_add(seed));
        });
    }


    pub(crate) fn camera_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 2);
        be_library(c);
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 0x10, DATA + 0x2000);
        c.bus.write16(DATA + 0x2000 + 0x18, 0);
        c.bus.write32(DATA + 0x2000 + 0x1c, FAKE);
        c.bus.write32(DATA + 0x30, DATA + 0x1000);
        c.bus.write32(DATA + 0x1004, DATA + 0x1100);
        c.bus.write32(DATA + 0x1100 + 0x6c0, DATA + 0x1200);
        c.bus.write16(DATA + 0x1200 + 0x38, 0);
        c.bus.write32(DATA + 0x1200 + 0x3c, FAKE);
    }


    #[test]
    fn bounded_camera_algorithm_base_reset_matches() {
        check(
            ee::s02::f_00173678__generated,
            bounded_camera_algorithm_base_reset,
            8,
            &|c, seed| camera_setup(c, seed, 0x0017_3678),
        );
    }


    #[test]
    fn bounded_camera_algorithm_reset_matches() {
        check(
            ee::s02::f_001747a0__generated,
            bounded_camera_algorithm_reset,
            8,
            &|c, seed| camera_setup(c, seed, 0x0017_47a0),
        );
    }


    #[test]
    fn superpipe_chase_camera_algorithm_ctor_matches() {
        check(
            ee::s02::f_00178520__generated,
            superpipe_chase_camera_algorithm_ctor,
            8,
            &|c, seed| {
                machine(c, seed, 0x0017_8520, 1);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x1010) as u64;
            },
        );
    }


    /// `gp-0x848` -> a dummy world_2.
    pub(crate) fn world_2(c: &mut Cpu, w: u32) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
    }


    /// The shared camera-algorithm fixture: `this` at DATA, the rider target at
    /// DATA+0x100, the rider object at DATA+0x200 and the notify object at
    /// DATA+0x300, all slots pointing at the fake. `equal` picks the branch
    /// where the rider's current algorithm id still answers `rider`.
    pub(crate) fn camera_case(c: &mut Cpu, r: &mut Rng, equal: bool, rider: u64) {
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = rider;
        let vt_target = vtable(c, r, &[0x20]);
        let vt_rider = vtable(c, r, &[0x38]);
        c.bus.write32(DATA + 0x30, DATA + 0x100);
        c.bus.write32(DATA + 0x100, vt_target);
        c.bus.write32(DATA + 0x104, DATA + 0x200);
        c.bus.write32(DATA + 0x200 + 0x6c0, vt_rider);
        c.bus.write32(DATA + 0x10, DATA + 0x300);
        c.bus.write16(DATA + 0x300 + 0x28, 0);
        c.bus.write32(DATA + 0x300 + 0x2c, FAKE);
        c.bus.write32(
            LOG_RESULTS,
            if equal {
                rider as u32
            } else {
                rider as u32 ^ 1
            },
        );
        // ResetForRider's own current-id check answers a different id, so it
        // bails out before touching the rider.
        c.bus.write32(LOG_RESULTS + 4, 0x55);
    }


    macro_rules! camera_test {
        ($name:ident, $generated:path, $hand:path, $salt:expr) => {
            #[test]
            fn $name() {
                check($generated, $hand, 40, &|c, seed| {
                    let mut r = machine(c, seed, $salt, 4);
                    camera_case(c, &mut r, seed % 2 == 0, 3);
                });
            }
        };
    }
pub(crate) use camera_test;


    camera_test!(
        mid_left_chase_camera_reset_for_rider_matches,
        ee::s02::f_00168f60__generated,
        mid_left_chase_camera_reset_for_rider,
        0x168f60
    );

    camera_test!(
        default_2_chase_camera_reset_for_rider_matches,
        ee::s02::f_00176ce0__generated,
        default_2_chase_camera_reset_for_rider,
        0x176ce0
    );

    camera_test!(
        default_3_chase_camera_reset_for_rider_matches,
        ee::s02::f_00176fe0__generated,
        default_3_chase_camera_reset_for_rider,
        0x176fe0
    );

    camera_test!(
        default_4_chase_camera_reset_for_rider_matches,
        ee::s02::f_001772e0__generated,
        default_4_chase_camera_reset_for_rider,
        0x1772e0
    );

    camera_test!(
        direction_ne_chase_camera_reset_for_rider_matches,
        ee::s02::f_00177588__generated,
        direction_ne_chase_camera_reset_for_rider,
        0x177588
    );

    camera_test!(
        post_race_1_chase_camera_reset_for_rider_matches,
        ee::s02::f_00178e90__generated,
        post_race_1_chase_camera_reset_for_rider,
        0x178e90
    );

    /// A post-race camera at `DATA` with its target (`+0x30`, the vtable
    /// slots GatherRiderState and the update call: 1, 3, 17, 18, 20), the
    /// rider behind it (`Rider_GetMainState` reads `rider+0x77c → +0xde0`),
    /// the world's player count (`gp-0x848 → +0x84 → +0xc → +0x78`) and the
    /// game's constants (`.lit4`, the world up at 0x4ff160).
    fn post_race_update_case(c: &mut Cpu, r: &mut Rng, counter: i32, players: u32) {
        let (algo, t, rider, ms) = (DATA, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
        c.g[A0][0] = sx(algo);
        for i in 0..0x100 {
            c.bus.write32(algo + 4 * i, r.f32(400.0));
        }
        put_vec(c, algo + 0x390, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
        c.bus.write32(algo + 0x3a0, counter as u32);
        c.bus.write32(algo + 0x30, t);
        let vt = vtable(c, r, &[0x8, 0x18, 0x88, 0x90, 0xa0]);
        c.bus.write32(t, vt);
        c.bus.write32(t + 4, rider);
        c.bus.write32(rider + 0x77c, ms);
        c.bus.write32(ms + 0xde0, r.next_u32() % 8);
        let (w, w84, w0c) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x84, w84);
        c.bus.write32(w84 + 0xc, w0c);
        c.bus.write32(w0c + 0x78, players);
        c.bus.write32(crate::tick::APPMAN_PTR, 0);
    }

    #[test]
    fn post_race_1_chase_camera_update_matches() {
        covered(
            ee::s02::f_00178bb0__generated,
            post_race_1_chase_camera_update,
            SEEDS,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x178bb0, 6);
                // Negative counters reach the first floor's correction.
                let counter = match seed % 4 {
                    0 => 1099,
                    1 => -((r.next_u32() % 3000) as i32),
                    _ => (r.next_u32() % 1100) as i32,
                };
                let players = r.next_u32() % 4;
                post_race_update_case(c, &mut r, counter, players);
                if seed % 3 == 0 {
                    // Other angle constants: the swing angle's floor goes
                    // below zero too.
                    for off in (0x5d4c..=0x5d5c).step_by(4) {
                        c.bus.write32(GAME_GP.wrapping_sub(off), r.f32(8.0));
                    }
                }
            },
        );
    }

    /// The orbit counter counts updates: at 120 it wraps at 2200 and each
    /// update turns the orbit half as far, so two 120 Hz updates leave the
    /// eye where one 60 Hz update does.
    #[test]
    fn post_race_1_chase_camera_update_follows_the_rate() {
        // (counter, eye − look_at) after `ticks` updates at `hz` from `start`.
        let run = |hz: u32, ticks: u32, start: i32| {
            let mut boxed = crate::difftest::fresh();
            let c: &mut Cpu = &mut boxed;
            elf_image(c);
            let mut r = machine(c, 7, 0x178bb1, 0);
            post_race_update_case(c, &mut r, start, 1);
            put_vec(c, DATA + 0x390, [0x3f80_0000, 0, 0, 0]);
            crate::tick::set_for_test(c, hz);
            let sp = c.g[SP][0];
            for _ in 0..ticks {
                c.g[A0][0] = sx(DATA);
                c.g[SP][0] = sp;
                c.g[RA][0] = sx(TEST_RA);
                post_race_1_chase_camera_update(c);
                assert_eq!(c.pc, TEST_RA, "returned");
            }
            let rd = |c: &mut Cpu, a: u32| f32::from_bits(c.bus.read32(a));
            let rel: Vec<f32> = (0..3).map(|i| rd(c, DATA + 0x40 + 4 * i) - rd(c, DATA + 0x20 + 4 * i)).collect();
            (c.bus.read32(DATA + 0x3a0) as i32, rel)
        };
        for start in [0, 100, 400, 1098] {
            let (n60, eye60) = run(60, 1, start);
            assert_eq!(n60, start + 1);
            for hz in [120u32, 240] {
                let k = (hz / 60) as i32;
                let (n, eye) = run(hz, k as u32, start * k);
                assert_eq!(n, (start + 1) * k % (1100 * k), "counter at {hz}");
                for i in 0..3 {
                    assert!((eye[i] - eye60[i]).abs() < 0.05, "eye at {hz} from {start}: {eye:?}, at 60: {eye60:?}");
                }
            }
        }
        // The wrap: one orbit is 1100 frames at every rate.
        assert_eq!(run(60, 1, 1099).0, 0);
        assert_eq!(run(120, 1, 2199).0, 0);
        assert_eq!(run(120, 1, 1099).0, 1100);
        assert_eq!(run(240, 1, 4399).0, 0);
    }


    pub(crate) fn c_ui_state_buy_popup_ctor_matches() {
        check(
            ee::s04::f_001caba8__generated,
            c_ui_state_buy_popup_ctor,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x1caba8, 2);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x200);
                // The owner's first word is the FE module (or absent).
                c.bus
                    .write32(DATA + 0x100, if seed % 3 == 0 { 0 } else { DATA + 0x1800 });
            },
        );
    }


    #[test]
    fn camera_trigger_read_action_type2_matches() {
        check(
            ee::s02::f_001715f8__generated,
            camera_trigger_read_action_type2,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1715f8, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA, DATA + 0x100);
                for k in 0..20u32 {
                    c.bus.write8(DATA + 0x100 + k, r.next_u32() as u8);
                }
            },
        );
    }


    /// `gp-0x854` -> a dummy FE manager and `gp-0x848` -> a dummy world_2, with
    /// the slots the render helpers read pointed at the fake.
    pub(crate) fn fe_render_fixture(c: &mut Cpu, r: &mut Rng) -> u32 {
        let m = DATA + 0x1000;
        let o = DATA + 0x1100;
        c.bus.write32(GAME_GP.wrapping_sub(0x854), m);
        c.bus.write32(m + 0x10d8, o);
        for off in [0x78u32, 0x88, 0xa0, 0xa8, 0xf8] {
            c.bus.write16(o + off, 0);
            c.bus.write32(o + off + 4, FAKE);
        }
        c.bus.write32(m + 0xe84, DATA + 0x1200);
        let w = DATA + 0x1300;
        world_2(c, w);
        c.bus.write32(w + 0xa4, DATA + 0x1400);
        let _ = r;
        o
    }


    pub(crate) fn c_fe_state_trophy_room_show_trophy_info_matches() {
        check(
            ee::s04::f_001d4890__generated,
            c_fe_state_trophy_room_show_trophy_info,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d4890, 3);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 3) as u64;
                let list = DATA + 0x1000;
                let item = DATA + 0x1100;
                let vt = DATA + 0x1200;
                c.bus
                    .write32(DATA + 0x4c, if seed % 4 == 0 { 0 } else { list });
                c.bus.write32(list + 8, item);
                c.bus.write16(vt + 0x48, 0);
                c.bus.write32(vt + 0x4c, FAKE);
                c.bus.write32(DATA + 0x58, 0x1111);
                c.bus.write8(DATA + 0x44, 5);
                c.bus.write32(DATA + 0x50, 0x2222);
                c.bus.write32(DATA + 0x54, 0x3333);
            },
        );
    }


    macro_rules! render_test {
        ($name:ident, $generated:path, $hand:path, $salt:expr) => {
            #[test]
            fn $name() {
                check($generated, $hand, 40, &|c, seed| {
                    let mut r = machine(c, seed, $salt, 4);
                    fe_render_fixture(c, &mut r);
                    // Active screen only every fourth seed.
                    c.bus
                        .write32(LOG_RESULTS, if seed % 4 == 0 { 0x11 } else { 0 });
                });
            }
        };
    }
pub(crate) use render_test;


    render_test!(
        pre_start_screen_render_matches,
        ee::s06::f_00232488__generated,
        pre_start_screen_render,
        0x232488
    );

    render_test!(
        init_load_screen_render_matches,
        ee::s06::f_00232650__generated,
        init_load_screen_render,
        0x232650
    );

    render_test!(
        c_pre_game_load_screen_render_matches,
        ee::s06::f_00232820__generated,
        c_pre_game_load_screen_render,
        0x232820
    );

    render_test!(
        c_pre_fe_load_screen_render_matches,
        ee::s06::f_002328a8__generated,
        c_pre_fe_load_screen_render,
        0x2328a8
    );


    #[test]
    fn debug_menu_next_enabled_item_matches() {
        check(
            ee::s02::f_0017a028__generated,
            debug_menu_next_enabled_item,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x17a0, 0);
                env(c);
                let this = DATA + 0x400;
                let n = 2 + (seed % 4); // 2..5 entries
                w32(c, this, n);
                let entries = this + 0x8c;
                // Every entry enabled, except one seed group that must scan a
                // little further.
                let dead = if seed % 3 == 0 { 0 } else { u32::MAX };
                for i in 0..n {
                    w32(c, entries + 0x8c * i, if i == dead { 0 } else { 1 });
                }
                w32(c, this + 0x8c08, (seed % 5 != 0) as u32);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % (n + 1)) as u64;
            },
        );
    }


    // ------------------------------------------------------------ debug menu

    #[test]
    fn debug_menu_prev_enabled_item_matches() {
        check(ssx3_ee::s02::f_0017a0b8__generated, debug_menu_prev_enabled_item, 24, &|c, seed| {
            let mut r = base(c, seed, 0x0017_a0b8);
            let menu = DATA + 0x100;
            let n = 1 + (seed % 6);
            clear_2(c, menu, 0x130);
            w(c, menu, n);
            w(c, menu + 0x8808, if seed % 5 == 0 { 0 } else { r.next_u32() | 1 });
            c.g[A0][0] = sx(menu);
            c.g[A1][0] = (seed % n) as u64;
            // The first candidate: sometimes enabled (returns at once), else
            // leave the wrap-around scan one enabled entry so it terminates.
            w(c, menu + 0x118, if seed % 2 == 0 { 1 } else { 0 });
            let hit = (seed % n) as u32;
            for i in 0..n {
                w(c, menu + 0x8c + 4 * i, (i == hit) as u32);
            }
        });
    }


    // ----------------------------------------------------------- debug menu

    #[test]
    fn debug_menu_find_next_item_matches() {
        check(
            ee::s02::f_0017a220__generated,
            debug_menu_find_next_item,
            SEEDS_5,
            &|c, seed| {
                let _ = base_2(c, seed, 0x17a2_20, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed ^ 0xa5) as u64;
                w(c, obj, (seed % 6) as u32);
            },
        );
    }


    pub(crate) const SEEDS_6: u32 = 120;


    #[test]
    fn bounded_camera_algorithm_base_ctor_matches() {
        check(
            ee::s02::f_00173208__generated,
            bounded_camera_algorithm_base_ctor,
            SEEDS_6,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1732);
                c.g[A0][0] = sx(DATA);
                put_words(c, gp_at(-0x6434), &[f(0.8), f(10.3), f(1.1)]);
                fill(c, &mut r, DATA, 0x144);
            },
        );
    }


    #[test]
    fn debug_menu_find_prev_item_matches() {
        check(
            ee::s02::f_0017a2c8__generated,
            debug_menu_find_prev_item,
            60,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x17a2);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = 0x41;
                c.bus.write32(DATA, (seed % 4) as u32);
                for i in 0..4 {
                    c.bus.write32(DATA + 8 * i + 4, 0);
                }
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }


    #[test]
    fn superpipe_chase_camera_reset_for_rider_matches() {
        check(
            ee::s02::f_00178938__generated,
            superpipe_chase_camera_algorithm_reset_for_rider,
            40,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1789);
                let rider = DATA + 0x1000;
                let id = 0x1234u32;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 { id as u64 } else { 0x77 };
                c.bus.write32(DATA + 0x30, DATA + 0x900);
                c.bus.write32(DATA + 0x900 + 4, rider);
                c.bus.write32(rider + 0x6c0, DATA + 0x2000);
                slot_2(c, DATA + 0x2000, 0x38, 1);
                c.bus.write32(crate::difftest::fakes::LOG_RESULTS, id);
                c.bus.write32(DATA + 0x10, DATA + 0x3100);
                fill(c, &mut r, DATA + 0x20, 0x80);
                fill(c, &mut r, DATA + 0x3000, 0x200);
            },
        );
    }


    #[test]
    fn debug_menu_free_backgrounds_matches() {
        check(
            ee::s02::f_0017abe0__generated,
            debug_menu_free_backgrounds,
            8,
            &|c, seed| {
                let _ = base_4(c, seed, 0x0017_abe0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed & 1) as u64;
                let count = if seed % 4 == 0 { 0 } else { 1 + (seed % 3) };
                w(c, OBJ, OBJ2);
                w(c, OBJ + 4, count);
                for i in 0..count {
                    w(c, OBJ2 + 4 * i, if seed % 3 == 0 && i == 0 { -1i32 as u32 } else { 1 });
                }
                w(c, GAME_GP.wrapping_sub(0x854), OBJ3);
                w(c, OBJ3 + 0x10d8, OBJ4);
                c.bus.write16(OBJ4 + 0x190, (seed as u16) & !3);
                w(c, OBJ4 + 0x194, FAKE);
            },
        );
    }


    #[test]
    fn camera_trigger_read_action_type1_matches() {
        check(ee::s02::f_001714e8__generated, camera_trigger_read_action_type1, 30, &|c, seed| {
            let mut r = base_5(c, seed, 0x1714, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            for i in 0..0x30u32 {
                w(c, DATA + 0x800 + 4 * i, r.next_u32());
            }
        });
    }


    #[test]
    fn camera_sphere_trigger_get_bounds_matches() {
        check(ee::s02::f_001721c0__generated, camera_sphere_trigger_get_bounds, SEEDS_7, &|c, seed| {
            let mut r = base_5(c, seed, 0x1721, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            for i in 0..8u32 {
                w(c, DATA + 4 * i, r.f32(100.0));
            }
            w(c, DATA + 0x18, r.f32(100.0));
            w(c, DATA + 0x1c, r.f32(100.0));
            w(c, DATA + 0x20, r.f32(100.0));
        });
    }


    #[test]
    fn camera_box_trigger_get_bounds_matches() {
        check(ee::s02::f_00172840__generated, camera_box_trigger_get_bounds, SEEDS_7, &|c, seed| {
            let mut r = base_5(c, seed, 0x1728, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            for i in 0..8u32 {
                w(c, DATA + 4 * i, r.f32(100.0));
            }
            w(c, DATA + 0x18, r.f32(100.0));
            w(c, DATA + 0x1c, r.f32(100.0));
            w(c, DATA + 0x20, r.f32(100.0));
        });
    }


    #[test]
    fn spoke_chase_camera_algorithm_get_focus_point_matches() {
        check_all(
            ee::s02::f_00177ef8__generated,
            spoke_chase_camera_algorithm_get_focus_point,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x177e, 4);
                game_globals(c);
                let out = DATA + 0x2000;
                let this = DATA;
                c.g[A0][0] = out as u64;
                c.g[A1][0] = this as u64;
                c.bus.write32(GAME_GP - 0x5e0c, 0.2f32.to_bits());
                c.bus.write32(GAME_GP - 0x5e08, 0.8f32.to_bits());
                let o = DATA + 0x2400;
                c.bus.write32(this + 0x30, o);
                let o2 = DATA + 0x2800;
                c.bus.write32(o + 4, o2);
                let vt1 = vtable(c, &mut r, &[0x28]);
                c.bus.write32(o2 + 0x6c0, vt1);
                let vt2 = vtable(c, &mut r, &[8]);
                c.bus.write32(o, vt2);
            },
        );
    }


    /// The `BELibrary` singleton (`gp - 0x1f04`) and the world (`gp - 0x848`).
    pub(crate) fn globals_3(c: &mut Cpu) {
        let be = DATA + 0x3_0000;
        let world = DATA + 0x8_0000;
        w(c, GAME_GP.wrapping_sub(0x1f04), be);
        w(c, GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        w(c, world + 0x84, game);
        let race = world + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }


    /// The engine's size-class pool_4 allocator behind `gp + 0xdd8`.
    pub(crate) fn pool_4(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool_4 = DATA + 0x20_000 + 0x100 * i as u32;
            let base_6 = nodes + 0x400 * i as u32;
            for n in 0..8u32 {
                let node = base_6 + 0x40 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x40 });
            }
            w(c, pool_4 + head, base_6);
            w(c, GAME_GP + slot, pool_4);
        }
    }


    /// Scrambled state, the fake call hooks, `gp`, the globals_3 and the pools.
    pub(crate) fn base_6(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        globals_3(c);
        pool_4(c);
        r
    }



    /// A camera trigger box whose vtable slots are fake calls, read from a
    /// dummy byte stream.
    pub(crate) fn camera_box_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = base_6(c, seed, salt, 0);
        let reader = DATA + 0x100;
        let obj = DATA + 0x2000;
        let vt = DATA + 0x3000;
        w(c, reader, DATA + 0x4000);
        w(c, obj + 0x24, vt);
        for (i, off) in [8u32, 0x10, 0x18, 0x20, 0x28].into_iter().enumerate() {
            c.bus.write16(vt + off, (r.next_u32() as i16 & !3) as u16);
            w(c, vt + off + 4, crate::difftest::fakes::FAKE + 0x10 * i as u32);
        }
        for k in 0..8u32 {
            w(c, DATA + 0x4000 + 4 * k, r.next_u32());
        }
        c.g[A0][0] = sx(reader);
        c.g[A1][0] = sx(obj);
    }


        #[test]
    fn chase_camera_algorithm_ctor_matches() {
        check3(ee::s01::f_00162318__generated, chase_camera_algorithm_ctor, 60, &|c, seed| {
            let _ = base_7(c, seed, 0x162318);
            c.g[A0][0] = sx(DATA);
        });
    }


    /// The fixture objects. They must stay clear of `fakes::LOG`
    /// (0x0090_0000, one 0x220-byte record per fake call) and of the pool_5
    /// regions below: `DATA + 0x60_0000` is above all of them.
    pub(crate) const BASE: u32 = DATA + 0x60_0000;

    /// The cell descriptor (`t1`).
    pub(crate) const DESC: u32 = BASE;

    /// The octree node (`t0`), its child node, and their lists.
    pub(crate) const NODE: u32 = BASE + 0x040;

    pub(crate) const NODE2: u32 = BASE + 0x080;

    pub(crate) const LIST: u32 = BASE + 0x0c0;

    /// A trigger: its object, the object it hangs off, and that object's
    /// second-predicate slot.
    pub(crate) const TRIG: u32 = BASE + 0x100;

    pub(crate) const SUB: u32 = BASE + 0x140;

    pub(crate) const VT2: u32 = BASE + 0x180;

    /// The manager (`a1`/`a2`), its object, and that object's first-predicate
    /// slot at `WOBJ + 0x6c0`.
    pub(crate) const MGR: u32 = BASE + 0x1c0;

    pub(crate) const WOBJ: u32 = BASE + 0x200;

    pub(crate) const VT1: u32 = WOBJ + 0x6c0;

    /// The box (`a3`).
    pub(crate) const BOX: u32 = BASE + 0x900;

    /// The active-list head slot and one list node.
    pub(crate) const HEAD: u32 = BASE + 0x940;

    pub(crate) const ACTIVE: u32 = BASE + 0x980;


    pub(crate) fn w_2(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }


    /// The BELibrary singleton (a zeroed dummy so nothing reads the disc) and
    /// the world (no riders).
    pub(crate) fn globals_4(c: &mut Cpu) {
        const BE: u32 = DATA + 0x20_0000;
        const WORLD: u32 = DATA + 0x21_0000;
        w_2(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w_2(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = WORLD + 0x1000;
        w_2(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w_2(c, game + 0xc, race);
        w_2(c, race + 0x7c, 0);
    }


    /// The engine's size-class pool_5 allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class (a zeroed heap walks garbage). The trigger
    /// list's `Add` allocates one 0x10-byte node.
    pub(crate) fn pool_5(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w_2(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool_5 = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w_2(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w_2(c, pool_5 + head, base);
            w_2(c, GAME_GP + slot, pool_5);
        }
    }


    /// A bounded octree (one child level), a trigger list of at most one entry,
    /// predicate slots that are either fakes or `ActiveCameraTriggerList_Find`
    /// (to get a 0 result), and a box whose overlap with the cell varies with
    /// the seed so every case and every descend/collect branch is taken.
    pub(crate) fn setup_2(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x1748_4848, 8);
        globals_4(c);
        pool_5(c);
        // The four `.lit4` floats the box tests read (all 0.2f).
        put_words(c, GAME_GP.wrapping_sub(0x631c), &[0x3e4c_cccd; 4]);

        // Levels 0..2 (cell size 0.5/1/2) keep the cell intervals inside the
        // box's range; level 11 short-circuits the whole query.
        let level = match seed % 3 {
            0 => 11,
            _ => (seed / 3) % 3,
        };
        w_2(c, DESC, level);
        for i in 1..4 {
            w_2(c, DESC + 4 * i, r.next_u32() % 4);
        }

        // The box. Every fifth seed asks for everything (so the "wholly
        // inside" collect path is reached).
        let huge = seed % 5 == 0;
        for i in 0..3 {
            let (lo, hi) = if huge {
                (-1000.0f32, 1000.0f32)
            } else {
                let lo = f32::from_bits(r.f32(8.0));
                (lo, lo + (r.next_u32() % 4096) as f32 * 0.01)
            };
            w_2(c, BOX + 4 * i, lo.to_bits());
            w_2(c, BOX + 0x10 + 4 * i, hi.to_bits());
        }

        // The node's eight children (bit `i` of the seed) and its list.
        for i in 0..8u32 {
            let child = if (seed >> i) & 1 != 0 { NODE2 } else { 0 };
            w_2(c, NODE + 4 * i, child);
        }
        w_2(c, NODE + 0x28, if seed % 4 == 0 { 0 } else { LIST });
        for i in 0..8u32 {
            w_2(c, NODE2 + 4 * i, 0);
        }
        w_2(c, NODE2 + 0x28, 0);

        // One list entry: a trigger (type 4) or something the walk skips.
        w_2(c, LIST, 0);
        w_2(c, LIST + 4, 0);
        w_2(c, LIST + 8, if seed % 8 < 2 { 3 } else { 4 });
        w_2(c, LIST + 0xc, TRIG);

        // trigger -> sub -> the second predicate; the manager's object holds
        // the first one.
        w_2(c, TRIG + 8, SUB);
        w_2(c, SUB + 0x24, VT2);
        w_2(c, VT2 + 0x38, 0);
        // The second predicate returning 0 (a real empty-list `Find`) makes the
        // walk skip the trigger; a fake returns a pointer (Find/Add path).
        w_2(c, VT2 + 0x3c, if seed % 2 == 0 { ACTIVE_FIND } else { fakes::FAKE });
        w_2(c, MGR, HEAD);
        w_2(c, MGR + 4, WOBJ);
        w_2(c, WOBJ + 0x6c0, VT1);
        w_2(c, VT1 + 0x28, 0);
        w_2(c, VT1 + 0x2c, fakes::FAKE);

        // The manager's active list: empty / a hit / a miss.
        let head = match seed % 4 {
            1 => {
                w_2(c, ACTIVE, TRIG);
                ACTIVE
            }
            2 => {
                w_2(c, ACTIVE, 0x1234);
                ACTIVE
            }
            _ => 0,
        };
        w_2(c, HEAD, head);
        if seed % 4 >= 1 {
            w_2(c, ACTIVE + 4, 0);
        }

        c.g[A0][0] = 0;
        c.g[A1][0] = MGR as u64;
        c.g[A2][0] = MGR as u64;
        c.g[A3][0] = BOX as u64;
        c.g[T0][0] = NODE as u64;
        c.g[T1][0] = DESC as u64;
    }


    #[test]
    fn collect_octree_triggers_in_box_matches() {
        // The fixture must not sit in the fake callees' log area: every fake
        // call writes a 0x220-byte record there and would eat the node, its
        // list and the predicate slots mid-run.
        assert!(BASE > fakes::LOG + 0x1_0000, "fixture overlaps the fake call log");
        for seed in 0..200 {
            let report = compare(
                ee::s02::f_00174848__generated,
                camera_trigger_man_collect_octree_triggers_in_box,
                &|c| setup_2(c, seed),
            );
            assert!(report.hard.is_empty(), "seed {seed}: {:#?}", report.hard);
            assert!(report.soft.is_empty(), "seed {seed} (soft): {:#?}", report.soft);
            assert!(report.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", report.lohi);
        }
    }


    pub(crate) const OCT_NODE: u32 = DATA + 0x1_0000;

    pub(crate) const OCT_CTX: u32 = DATA + 0x1_1000;

    pub(crate) const OCT_ENTRY: u32 = DATA + 0x1_2000;


    #[test]
    fn camera_trigger_man_collect_octree_node_triggers_matches() {
        check_all_2(
            ee::s02::f_00176890__generated,
            camera_trigger_man_collect_octree_node_triggers,
            20,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x1768_90, 10);
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
}
