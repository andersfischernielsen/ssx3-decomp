//! `src/camera/camera.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
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
pub(crate) use crate::game::animation::rideranimbase::sw;
pub(crate) use crate::game::bx::bxstring::gp as gp_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_4;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::chase_reset_for_rider;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::ldf;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::ofs;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::BackHeight;
pub(crate) use crate::game::fe::feasyncfile::gp_11 as gp_8;
pub(crate) use crate::game::fe::feasyncfile::gp_8 as gp_6;
pub(crate) use crate::game::fe::feasyncfile::gpn;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_3;
pub(crate) use crate::game::fe::feasyncfile::p_7 as p_4;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::u32_at;
pub(crate) use crate::game::fe::festateload::at as at_2;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::render::graphicsman::at as at_3;
pub(crate) use crate::game::render::graphicsman::gp as gp_5;
pub(crate) use crate::game::render::lightman::at;
pub(crate) use crate::game::render::lightman::gp_2 as gp;
pub(crate) use crate::game::scripter::ssxscriptengine::gp_5 as gp_2;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::ui::uivector::r;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
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
    (
        0x0015_cf80,
        manual_cam_controller_select_algorithm,
        "ManualCamController_SelectAlgorithm",
    ),
    (
        0x0015_dc18,
        game_camera_reset_all_chase_controllers,
        "GameCamera_ResetAllChaseControllers",
    ),
    (0x0015_dd88, game_camera_dtor, "GameCamera_dtor"),
    (
        0x0015_ee00,
        game_camera_avoid_collision,
        "GameCamera_AvoidCollision",
    ),
    (
        0x0016_35f8,
        chase_camera_algorithm_apply_jump_curves,
        "ChaseCameraAlgorithm_ApplyJumpCurves",
    ),
    (
        0x0015_f598,
        rider_camera_target_get_orientation,
        "RiderCameraTarget_GetOrientation",
    ),
    (
        0x0015_f5f8,
        rider_camera_target_get_forward,
        "RiderCameraTarget_GetForward",
    ),
    (
        0x0015_f610,
        rider_camera_target_get_side,
        "RiderCameraTarget_GetSide",
    ),
    (
        0x0015_f628,
        rider_camera_target_get_up,
        "RiderCameraTarget_GetUp",
    ),
    (
        0x0015_f650,
        rider_camera_target_get_ground_normal,
        "RiderCameraTarget_GetGroundNormal",
    ),
    (
        0x0015_f668,
        rider_camera_target_get_prev_ground_normal,
        "RiderCameraTarget_GetPrevGroundNormal",
    ),
    (
        0x0015_f6b0,
        rider_camera_target_get_air_time_remaining,
        "RiderCameraTarget_GetAirTimeRemaining",
    ),
    (
        0x0015_f6c8,
        rider_camera_target_get_air_time_9c_remaining,
        "RiderCameraTarget_GetAirTime9cRemaining",
    ),
    (
        0x0015_f720,
        rider_camera_target_is_landing_predicted,
        "RiderCameraTarget_IsLandingPredicted",
    ),
    (
        0x0015_f938,
        manual_cam_algo_base_ctor,
        "ManualCamAlgoBase_ctor",
    ),
    (0x0015_fd08, manual_cam_algo_ctor, "ManualCamAlgo_ctor"),
    (0x0015_ffb0, editor_cam_algo_ctor, "EditorCamAlgo_ctor"),
    (
        0x0016_0130,
        camera_wobble_update_roll,
        "CameraWobble_UpdateRoll",
    ),
    (
        0x0016_0228,
        camera_wobble_get_offset,
        "CameraWobble_GetOffset",
    ),
    (
        0x0016_0438,
        manual_chase_camera_algorithm_ctor,
        "ManualChaseCameraAlgorithm_ctor",
    ),
    (
        0x0016_0760,
        matrix_chase_camera_algorithm_ctor,
        "MatrixChaseCameraAlgorithm_ctor",
    ),
    (
        0x0016_0ae8,
        matrix_chase_camera_algorithm_reset_for_rider,
        "MatrixChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0016_0b70,
        relative_chase_camera_algorithm_ctor,
        "RelativeChaseCameraAlgorithm_ctor",
    ),
    (
        0x0016_0fd0,
        relative_chase_camera_algorithm_reset_for_rider,
        "RelativeChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0016_1370,
        spline_camera_algorithm_dtor,
        "SplineCameraAlgorithm_dtor",
    ),
    (
        0x0016_1630,
        spline_camera_algorithm_reset_for_rider,
        "SplineCameraAlgorithm_ResetForRider",
    ),
    (
        0x0016_16a8,
        idle_chase_camera_algorithm_ctor,
        "IdleChaseCameraAlgorithm_ctor",
    ),
    (
        0x0016_1950,
        chase_cam_controller_init_state,
        "ChaseCamController_InitState",
    ),
    (
        0x0016_19a8,
        chase_cam_controller_ctor,
        "ChaseCamController_ctor",
    ),
    (
        0x0016_1a10,
        chase_cam_controller_dtor,
        "ChaseCamController_dtor",
    ),
    (
        0x0016_1a60,
        chase_cam_controller_set_target,
        "ChaseCamController_SetTarget",
    ),
    (
        0x0016_1e58,
        chase_cam_controller_push_algorithm,
        "ChaseCamController_PushAlgorithm",
    ),
    (
        0x0016_1ef0,
        chase_cam_controller_set_default_algorithm,
        "ChaseCamController_SetDefaultAlgorithm",
    ),
    (
        0x0016_1fa0,
        chase_cam_controller_begin_script_camera,
        "ChaseCamController_BeginScriptCamera",
    ),
    (
        0x0016_20d0,
        chase_cam_controller_clear_override,
        "ChaseCamController_ClearOverride",
    ),
    (
        0x0016_2170,
        chase_cam_reset_to_default,
        "ChaseCamController_ResetToDefault",
    ),
    (
        0x0016_21a8,
        chase_cam_controller_enter_spoke_mode,
        "ChaseCamController_EnterSpokeMode",
    ),
    (
        0x0016_2218,
        chase_cam_controller_exit_spoke_mode,
        "ChaseCamController_ExitSpokeMode",
    ),
    (
        0x0016_2258,
        chase_cam_start_post_race_camera,
        "ChaseCamController_StartPostRaceCamera",
    ),
    (
        0x0016_22b0,
        rider_camera_algorithm_ctor,
        "RiderCameraAlgorithm_ctor",
    ),
    (
        0x0016_56b0,
        chase_camera_start_shake,
        "ChaseCameraAlgorithm_StartShake",
    ),
    (
        0x0016_62a0,
        chase_camera_algorithm_smooth_pitch,
        "ChaseCameraAlgorithm_SmoothPitch",
    ),
    (
        0x0016_7d88,
        chase_camera_algorithm_default_heading_p,
        "ChaseCameraAlgorithm_DefaultHeadingP",
    ),
    (
        0x0016_7d98,
        chase_camera_algorithm_default_heading_i,
        "ChaseCameraAlgorithm_DefaultHeadingI",
    ),
    (
        0x0016_7da8,
        chase_camera_algorithm_default_heading_d,
        "ChaseCameraAlgorithm_DefaultHeadingD",
    ),
    (
        0x0016_7db8,
        chase_camera_algorithm_clear_heading_ring,
        "ChaseCameraAlgorithm_ClearHeadingRing",
    ),
    (
        0x0016_7e30,
        camera_algorithm_default_fov,
        "CameraAlgorithm_DefaultFov",
    ),
    (
        0x0016_7e40,
        camera_algorithm_get_look_at,
        "CameraAlgorithm_GetLookAt",
    ),
    (0x0016_7e68, camera_algorithm_tick, "CameraAlgorithm_Tick"),
    (
        0x0016_7fa0,
        rider_camera_algorithm_get_look_at,
        "RiderCameraAlgorithm_GetLookAt",
    ),
    (
        0x0016_82b8,
        lazy_chase_camera_algorithm_ctor,
        "LazyChaseCameraAlgorithm_ctor",
    ),
    (
        0x0016_8650,
        look_back_chase_camera_algorithm_ctor,
        "LookBackChaseCameraAlgorithm_ctor",
    ),
    (
        0x0016_8940,
        look_back_chase_camera_algorithm_reset_for_rider,
        "LookBackChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0016_89c8,
        mid_right_chase_camera_algorithm_ctor,
        "MidRightChaseCameraAlgorithm_ctor",
    ),
    (
        0x0016_8a30,
        mid_right_chase_camera_algorithm_get_eye_drop,
        "MidRightChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0016_8a40,
        mid_right_chase_camera_algorithm_get_tuning,
        "MidRightChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0016_8c50,
        mid_right_chase_camera_algorithm_reset_for_rider,
        "MidRightChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0016_8cd8,
        mid_left_chase_camera_algorithm_ctor,
        "MidLeftChaseCameraAlgorithm_ctor",
    ),
    (
        0x0016_8d40,
        mid_left_chase_camera_algorithm_get_eye_drop,
        "MidLeftChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0016_8d50,
        mid_left_chase_camera_algorithm_get_tuning,
        "MidLeftChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0016_8f60,
        mid_left_chase_camera_reset_for_rider,
        "MidLeftChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0032_0bf0,
        input_binding_eval_float,
        "InputBinding_EvalFloat",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
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
    pub(crate) use crate::game::be::beintnetwork::callee::C_BOLTON_FX_REFRESH_EMITTERS;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_MAP_EVAL_FLOAT;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_VM_EXECUTE;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_VM_INIT;
    pub(crate) use crate::game::be::beintnetwork::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::be::beintnetwork::callee::MEM_MAN_ALLOC;
    pub(crate) use crate::game::be::beintnetwork::callee::PATH_QUERY_EVENTS;
    pub(crate) use crate::game::be::beintnetwork::callee::SSX_AUDIO_SYSTEM_GET_APP;
    pub(crate) use crate::game::be::beintnetwork::callee::SSX_AUDIO_SYSTEM_GET_VIEW_COUNT;
    pub(crate) use crate::game::be::beintnetwork::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME0;
    pub(crate) use crate::game::be::beintnetwork::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME1;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX as INSTANCE_GET_NODE_MATRIX_2;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST_3;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN as STRLEN_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM as UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2 as UI_ANIM_SET_CREATE_ANIM_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM as UI_OBJECT_SET_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3 as EA_MEM_FREE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2 as FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2 as FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED as KEYBOARD_POPUP_IS_KEY_ENABLED_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR as CAM_CONTROLLER_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;
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
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR as UI_STATE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND as UI_TEXTURE_SET_FIND_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE as BELIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE as BELIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_4;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR as ACTIVE_CAMERA_TRIGGER_LIST_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2 as SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR_3;
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
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_2;
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

    /// `MenuSysItem_MeasureRow` (0x2ca4c8).
    pub const MEASURE_ROW: Callee = Callee {
        addr: 0x002c_a4c8,
        f: ee::s09::f_002ca4c8,
    };

    /// `cPS2MemCard_SetState` (0x2c48c0): the card state machine.
    pub const MEMCARD_SET_STATE: Callee = Callee {
        addr: 0x002c_48c0,
        f: ee::s08::f_002c48c0,
    };

    /// `cTransitionMan_Start` (0x2e4370).
    pub const TRANSITION_MAN_START: Callee = Callee {
        addr: 0x002e_4370,
        f: ee::s09::f_002e4370,
    };

    /// `WScriptProcessList_DeleteAll` (0x30b1b8).
    pub const WSCRIPT_DELETE_ALL: Callee = Callee {
        addr: 0x0030_b1b8,
        f: ee::s10::f_0030b1b8,
    };

    /// `SphereTree_ResetTransform` (0x32c540).
    pub const SPHERE_TREE_RESET_TRANSFORM: Callee = Callee {
        addr: 0x0032_c540,
        f: ee::s10::f_0032c540,
    };

    /// `CamController_Reset` (0x15ccf0).
    pub const CAM_CONTROLLER_RESET: Callee = Callee {
        addr: 0x0015_ccf0,
        f: ee::s01::f_0015ccf0,
    };

    /// `ManualCamAlgoBase_ctor` (0x15f938).
    pub const MANUAL_CAM_ALGO_BASE_CTOR: Callee = Callee {
        addr: 0x0015_f938,
        f: ee::s01::f_0015f938,
    };

    /// `ChaseCameraAlgorithm_ctor` (0x162318).
    pub const CHASE_CAMERA_ALGORITHM_CTOR: Callee = Callee {
        addr: 0x0016_2318,
        f: ee::s01::f_00162318,
    };

    /// `ChaseCamController_SetAlgorithm` (0x15d078).
    pub const CHASE_CAM_SET_ALGORITHM_2: Callee = Callee {
        addr: 0x0015_d078,
        f: ee::s01::f_0015d078,
    };

    /// `CameraTriggerMan_FreeTriggers` (0x16bf40).
    pub const CAMERA_TRIGGER_FREE_TRIGGERS: Callee = Callee {
        addr: 0x0016_bf40,
        f: ee::s02::f_0016bf40,
    };

    /// `CameraTriggerMan_ResetHeader` (0x16c390).
    pub const CAMERA_TRIGGER_RESET_HEADER: Callee = Callee {
        addr: 0x0016_c390,
        f: ee::s02::f_0016c390,
    };

    /// `UITextureSet_Find` (0x398380).
    pub const UI_TEXTURE_SET_FIND: Callee = Callee {
        addr: 0x0039_8380,
        f: ee::s13::f_00398380,
    };

    /// `UIState_ctor` (0x39e2a0).
    pub const UI_STATE_CTOR: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::f_0039e2a0,
    };

    /// `BENewPlayerInt_GetSlot` (0x147798; dispatches through its override slot).
    pub const BENEWPLAYER_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::f_00147798,
    };

    /// `BELibrary_ResetPlayerProfile` (0x149a88).
    pub const BE_LIBRARY_RESET_PROFILE: Callee = Callee {
        addr: 0x0014_9a88,
        f: ee::s01::f_00149a88,
    };

    /// `BEBAGTInt_ResetProfile` (0x14fc40).
    pub const BEBAGT_RESET_PROFILE: Callee = Callee {
        addr: 0x0014_fc40,
        f: ee::s01::f_0014fc40,
    };

    /// `BEOptionInt_GetMaxHandicap` (0x14f870).
    pub const BE_OPTION_GET_MAX_HANDICAP: Callee = Callee {
        addr: 0x0014_f870,
        f: ee::s01::f_0014f870,
    };

    /// `cFEStateProfileLoad_RefreshProfileList` (0x18f2b8).
    pub const FE_REFRESH_PROFILE_LIST: Callee = Callee {
        addr: 0x0018_f2b8,
        f: ee::s02::f_0018f2b8,
    };

    /// `cFEStateProfileLoad_RefreshSlots` (0x18f9d8).
    pub const FE_REFRESH_SLOTS: Callee = Callee {
        addr: 0x0018_f9d8,
        f: ee::s02::f_0018f9d8,
    };

    pub const CAM_CONTROLLER_DTOR: Callee = Callee {
        addr: 0x0015_cc10,
        f: ee::s01::f_0015cc10,
    };
}

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
// The chase-camera state (`ChaseCameraAlgorithm`, a0) and its info block (a1).

pub(crate) mod st {

    /// `vec4` swing position offset.
    pub const POS: u32 = 0x20;

    /// `vec4` swing direction (velocity) offset.
    pub const VEL: u32 = 0x40;

    /// The rider/model object (its first word is the vtable).
    pub const RIG: u32 = 0x30;

    /// Swing phase accumulator (radians).
    pub const PHASE: u32 = 0x22c;

    /// Swing frequency (phase advanced per second).
    pub const FREQ: u32 = 0x230;

    /// Swing amplitude (0 = no swing, 1 = full).
    pub const AMPLITUDE: u32 = 0x234;

    /// Swing gain, stored for the animation blend.
    pub const GAIN: u32 = 0x238;

    /// Time spent in the current swing (seconds).
    pub const TIME: u32 = 0x23c;

    /// Time at which the swing reached full amplitude.
    pub const IN_TIME: u32 = 0x240;

    /// Time at which the swing starts to decay.
    pub const OUT_TIME: u32 = 0x244;

    /// Non-zero while the camera is airborne (decides the air-swing path).
    pub const AIRBORNE: u32 = 0x2d0;

    /// Non-zero while the air swing is being applied.
    pub const SWINGING: u32 = 0x2d8;

    /// Cubic-spline state object used to shape the swing.
    pub const SPLINE: u32 = 0x334;

    /// `vec4` swing axis (unit).
    pub const AXIS: u32 = 0x110;

    /// `vec4` scaled axis copy.
    pub const AXIS_SCALED: u32 = 0x120;

    /// `vec3` scale (x, y, z).
    pub const SCALE: u32 = 0x130;

    /// `vec4` basis vector A.
    pub const BASIS_A: u32 = 0x140;

    /// `vec4` basis vector B.
    pub const BASIS_B: u32 = 0x150;

    /// `vec4` basis vector C.
    pub const BASIS_C: u32 = 0x160;

    /// `vec4` previous frame's `POS - VEL`.
    pub const PREV_DELTA: u32 = 0x170;

    /// `vec4` previous frame's `POS - POS0`.
    pub const PREV_MOVE: u32 = 0x180;

    /// Landing timer: counts down each frame; `14` rebuilds the jump splines.
    pub const TIMER: u32 = 0x2c4;

    /// Non-zero while the jump curve is being applied.
    pub const APPLYING: u32 = 0x2e0;

    /// Non-zero while the landing splines are in use.
    pub const LANDING: u32 = 0x2e4;

    /// Non-zero once the jump splines were rebuilt.
    pub const JUMP_READY: u32 = 0x2e8;

    /// Jump blend, 0..1.
    pub const JUMP_BLEND: u32 = 0x1f4;

    /// Landing blend, 0..1.
    pub const LAND_BLEND: u32 = 0x1f8;

    /// Scale applied to the jump curve.
    pub const JUMP_SCALE: u32 = 0x1fc;

    /// Jump curve amplitude.
    pub const AMPLITUDE_2: u32 = 0x200;

    /// Velocity carried from the jump curve.
    pub const JUMP_VEL_X: u32 = 0x20c;

    pub const JUMP_VEL_Y: u32 = 0x210;

    /// Velocity carried from the landing curve.
    pub const LAND_VEL_X: u32 = 0x21c;

    pub const LAND_VEL_Y: u32 = 0x220;

    /// Smoothed vertical velocity.
    pub const VERTICAL_VEL: u32 = 0x224;

    /// Time spent in the current jump.
    pub const TIME_2: u32 = 0x228;

    /// Spline `t` for the jump curves.
    pub const CURVE_T: u32 = 0x218;

    /// The two jump splines (up and down).
    pub const SPLINE_A: u32 = 0x31c;

    pub const SPLINE_B: u32 = 0x328;

    /// The two landing splines.
    pub const LAND_SPLINE_A: u32 = 0x304;

    pub const LAND_SPLINE_B: u32 = 0x310;
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

/// `void ChaseCamController_InitState(ChaseCamController *self, int index)`
/// (0x161950): mask the flags at +0x30, set the counts at +0x1c/+0x24/+0x28/
/// +0x2c, and link in an algorithm from `gp + 0x1ce0`.
pub fn chase_cam_controller_init_state(c: &mut Cpu) {
    c.cycles += 22;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x30).lw(c);
    c.g[V1][0] = 0xffff_ffff_ffff_fffd;
    c.g[A3][0] = 0xffff_ffff_ffff_fffe;
    c.g[T0][0] = 0xffff_ffff_ffff_fffb;
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[A2][0] = 0x3d;
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
    this.off(0x1c).sw(c, 0x3d);
    this.off(0x38).sw(c, 0);
    c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1ce0));
    this.off(0x30).sw(c, c.g[V0][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    this.off(0x3c).sw(c, 0);
    this.off(0x40).sw(c, 0);
    this.off(0x2c).sw(c, 0x3d);
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
    this.off(0x28).sw(c, 0x3d);
    this.off(0x24).sw(c, c.g[V0][0]);
    this.off(0x34).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Camera algorithms and triggers.

/// `float ChaseCameraAlgorithm_DefaultHeadingP(void)` (0x167d88): the pitch
/// heading constant.
pub fn chase_camera_algorithm_default_heading_p(c: &mut Cpu) {
    c.cycles += 3;
    c.f[0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x6788)).read_fpu(c).0;
    abi::ret(c);
}

/// `float ChaseCameraAlgorithm_DefaultHeadingI(void)` (0x167d98): the yaw
/// heading constant.
pub fn chase_camera_algorithm_default_heading_i(c: &mut Cpu) {
    c.cycles += 3;
    c.f[0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x6784)).read_fpu(c).0;
    abi::ret(c);
}

/// `float ChaseCameraAlgorithm_DefaultHeadingD(void)` (0x167da8): the roll
/// heading constant.
pub fn chase_camera_algorithm_default_heading_d(c: &mut Cpu) {
    c.cycles += 3;
    c.f[0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x6780)).read_fpu(c).0;
    abi::ret(c);
}

/// `float InputBinding_EvalFloat(InputBinding *binding, float value)`
/// (0x320bf0): evaluate the binding's map with the bound value.
pub fn input_binding_eval_float(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
        callee::INPUT_MAP_EVAL_FLOAT.call(c, 0x0032_0c08)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ChaseCameraAlgorithm_SmoothPitch (0x1662a0).
//
// (The VU0 pipeline uses `Src`/`X`/`Y`/`Z`/`W`.)

/// One dot product through the VU0 macro pipeline: `vf4 = dot(*a, *b)`, the
/// result read back into `gpr` (`lqc2 vf3/vf5`, the four-op multiply-add
/// chain, `qmfc2`). Leaves vf3-vf6 and acc as the original does.
pub(crate) fn vu0_dot(c: &mut Cpu, a: u32, b: u32, gpr: usize) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(gpr, 4);
}

/// A per-update kept share `k` (`v = k * v + (1 - k) * t`, or `v *= k`) at
/// the game's rate: `k^(60/rate)`, the same share per unit of wall-clock
/// time. Unchanged at 60.
fn keep_per_tick(c: &mut Cpu, k: FpuF32) -> FpuF32 {
    let hz = crate::tick::rate(c);
    if hz == 60 {
        k
    } else {
        FpuF32(f32::from_bits(k.0).powf(60.0 / hz as f32).to_bits())
    }
}

/// `void ChaseCameraAlgorithm_SmoothPitch(self, float x, float y, float z,
/// float w)` (0x1662a0): blend the camera's pitch (`+0x54`) and its smoothed
/// pitch (`+0x1d8`) towards the target angles, limited by the angle between
/// the camera-to-target direction and the camera's forward vector, then
/// re-aim the camera (`RotateByPitch`), update the look-at offset and let
/// `CheckDistanceReset` correct the distance.
///
/// Frame 0xb0: s0 at +0x80, ra at +0x70, f20 at +0x90, f21 at +0x98, f22 at
/// +0xa0, f23 at +0xa8. Scratch vectors at +0x0..+0x60.
pub fn chase_camera_algorithm_smooth_pitch(c: &mut Cpu) {
    guest(c, |c| {
        use ssx3_ee::s02 as cam;
        c.cycles += 14;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0x80);
        frame.save_ra(c, 0x70);
        c.g[S0][0] = c.g[A0][0];
        c.swc1(23, frame.at(c, 0xa8));
        c.swc1(22, frame.at(c, 0xa0));
        c.swc1(21, frame.at(c, 0x98));
        c.swc1(20, frame.at(c, 0x90));
        c.lwc1(1, at(c.g[S0][0], 0x1d8));
        c.lwc1(0, at(c.g[S0][0], 0x54));
        c.c_lt_s(1, 0);
        c.g[V0][0] = 1;
        if !c.fcc() {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        // Up or down: pick the blend factor and target angle from the
        // argument pair (w,y) or (z,x).
        c.cycles += 2;
        let up = c.g[V0][0] != 0;
        c.mov_s(20, 15);
        if up {
            c.cycles += 2;
            c.mov_s(22, 13);
        } else {
            c.cycles += 2;
            c.mov_s(20, 14);
            c.mov_s(22, 12);
        }
        // The blend factor is the kept share per update: at another rate the
        // same share per unit of wall-clock time (`k^(60/rate)`), before the
        // angle limit below divides it. f20 is callee-saved (restored on
        // return); unchanged at 60.
        if crate::tick::rate(c) != 60 {
            let k = keep_per_tick(c, FpuF32(c.f[20]));
            c.f[20] = k.0;
        }

        // Blend the current pitch and re-aim the camera to get the actual
        // pitch offset.
        c.cycles += 42;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(23, at(c.g[S0][0], 0x54));
        c.g[A1][0] = sx(frame.at(c, 0x10));
        c.sub_s(1, 21, 20);
        c.lwc1(0, at(c.g[S0][0], 0x1d8));
        c.mul_s(0, 20, 0);
        c.mul_s(1, 1, 23);
        c.add_s(0, 0, 1);
        c.swc1(0, at(c.g[S0][0], 0x54));
        c.lqc2(3, at(c.g[S0][0], 0x20));
        c.lqc2(4, at(c.g[S0][0], 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x10));
        c.lq(2, frame.at(c, 0x10));
        c.g[RA][0] = sx(0x0016_633c);
        c.sq(2, frame.at(c, 0));
        Callee {
            addr: 0x0016_68b8,
            f: cam::ChaseCameraAlgorithm_RotateByPitch,
        }
        .call(c, 0x0016_633c)?;
        vu0_dot(c, frame.at(c, 0), frame.at(c, 0), V0);
        c.mtc1(2, 0);
        vu0_dot(c, frame.at(c, 0x10), frame.at(c, 0x10), V1);
        c.mtc1(3, 1);
        c.mul_s(0, 0, 1);
        c.lwc1(2, gp(c, -0x68c0));
        c.sqrt_s(3, 0);
        c.c_lt_s(3, 2);
        let beyond = !c.fcc();
        if beyond {
            // The aimed direction is long enough: the pitch is the angle
            // between it and the target offset.
            c.cycles += 16;
            vu0_dot(c, frame.at(c, 0), frame.at(c, 0x10), V0);
            c.mtc1(2, 0);
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 1);
            c.div_s(0, 0, 3);
            c.c_le_s(1, 0);
            let clamp = !c.fcc();
            c.mov_s(12, 1);
            if !clamp {
                c.cycles += 1;
                c.min_s(12, 0, 21);
            }
            c.cycles += 4;
            call_ptr(c, 0x0031_c128, 0x0016_63f4)?;
            c.lwc1(1, gp(c, -0x68bc));
            c.sub_s(1, 1, 0);
        } else {
            c.cycles += 3;
            c.mtc1(0, 1);
        }

        // Turn rate: `ang * k` (or a linear step when the angle is tiny).
        c.cycles += 6;
        c.lwc1(0, gp(c, -0x68b8));
        c.mul_s(0, 22, 0);
        c.c_lt_s(0, 1);
        let small = !c.fcc();
        c.mov_s(14, 20);
        if !small {
            c.cycles += 2;
            c.div_s(0, 1, 0);
            c.div_s(14, 20, 0);
        }

        // Blend the smoothed pitch and re-aim, then re-aim the look-at
        // offset and let the distance reset check run.
        c.cycles += 68;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.g[A1][0] = sx(frame.at(c, 0x20));
        c.lwc1(0, at(c.g[S0][0], 0x1d8));
        c.g[A0][0] = c.g[S0][0];
        c.sub_s(1, 1, 14);
        c.mul_s(0, 14, 0);
        c.mul_s(1, 1, 23);
        c.add_s(0, 0, 1);
        c.swc1(0, at(c.g[S0][0], 0x1d8));
        c.g[RA][0] = sx(0x0016_644c);
        c.swc1(0, at(c.g[S0][0], 0x54));
        Callee {
            addr: 0x0016_68b8,
            f: cam::ChaseCameraAlgorithm_RotateByPitch,
        }
        .call(c, 0x0016_644c)?;
        c.g[A2][0] = c.lw(at(c.g[S0][0], 0x30));
        c.g[A0][0] = sx(frame.at(c, 0x40));
        c.g[V0][0] = c.lw(at(c.g[A2][0], 0));
        c.g[A1][0] = c.lh(at(c.g[V0][0], 8));
        c.g[V1][0] = c.lw(at(c.g[V0][0], 0xc));
        c.g[RA][0] = sx(0x0016_6468);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        let target = c.g[V1][0] as u32;
        call_ptr(c, target, 0x0016_6468)?;
        c.lqc2(3, frame.at(c, 0x40));
        c.lqc2(4, at(c.g[S0][0], 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x50));
        c.g[A0][0] = c.g[S0][0];
        c.lq(2, frame.at(c, 0x50));
        c.sq(2, frame.at(c, 0x30));
        vu0_dot(c, frame.at(c, 0x20), frame.at(c, 0x20), V0);
        c.mtc1(2, 1);
        vu0_dot(c, frame.at(c, 0x30), frame.at(c, 0x20), V1);
        c.mtc1(3, 0);
        c.div_s(0, 0, 1);
        c.mfc1(3, 0);
        c.lqc2(4, frame.at(c, 0x20));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x50));
        c.lq(2, frame.at(c, 0x50));
        c.sq(2, frame.at(c, 0x40));
        c.lqc2(3, at(c.g[S0][0], 0x40));
        c.lqc2(4, frame.at(c, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x60));
        c.lq(2, frame.at(c, 0x60));
        c.sq(2, at(c.g[S0][0], 0x20));
        c.g[RA][0] = sx(0x0016_650c);
        c.sq(2, frame.at(c, 0x50));
        Callee {
            addr: 0x0016_8150,
            f: cam::ChaseCameraAlgorithm_CheckDistanceReset,
        }
        .call(c, 0x0016_650c)?;
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 23, 0xa8);
        frame.restore_fpr(c, 22, 0xa0);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// `float CameraAlgorithm_DefaultFov(void)` (0x167e30): the default field of
/// view, π/4 radians, from the `.rodata` literal at `gp-0x677c`.
pub fn camera_algorithm_default_fov(c: &mut Cpu) {
    c.cycles += 3;
    let literal = gp(c, -0x677c);
    c.lwc1(0, literal);
    abi::ret(c);
}

/// `void CameraAlgorithm_Tick(cCameraAlgorithm *this)` (0x167e68): call the
/// algorithm's virtual `v05` at `vt + 0x2c` with `this` shifted by the `+0x28`
/// delta.
pub fn camera_algorithm_tick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let obj = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[V0][0] = obj;
        c.g[V1][0] = Addr((obj as u32).wrapping_add(0x28)).lh(c);
        let method = Addr((obj as u32).wrapping_add(0x2c)).lw(c);
        c.g[A1][0] = method;
        c.g[RA][0] = sx(0x0016_7e84);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        call_ptr(c, method as u32, 0x0016_7e84)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera.

/// `void ChaseCameraAlgorithm_ClearHeadingRing(CameraAlgo* self)` (0x167db8):
/// zero the five heading samples at_2 `+0x00`, `+0x14` (as floats) and the
/// five `+0x28` words, then the cursor at_2 `+0x3c`.
pub fn chase_camera_algorithm_clear_heading_ring(c: &mut Cpu) {
    c.cycles += 2;
    Addr(at_2(abi::arg(c, 0), 0x3c)).sw(c, 0);
    c.g[V0][0] = 4;
    loop {
        c.cycles += 7;
        let a = c.g[A0][0] as u32;
        c.swc1(12, a);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.swc1(12, at_2(a, 0x14));
        Addr(at_2(a, 0x28)).sw(c, 0);
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[A0][0] = sx(a.wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0016_7dc0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Camera.

/// `void ChaseCamController_ResetToDefault(ChaseCamController* cam)`
/// (0x162170): reset the state and select the default camera algorithm.
pub fn chase_cam_reset_to_default(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CHASE_CAM_RESET_STATE.call(c, 0x0016_2184)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x24)).lw(c);
        callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE.call(c, 0x0016_2194)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ChaseCamController_StartPostRaceCamera(ChaseCamController* cam)`
/// (0x162258): switch to the post-race algorithm (id 0x44), unless the
/// camera is already in replay mode (bit 0 of +0x30).
pub fn chase_cam_start_post_race_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let flags = Addr((c.g[A0][0] as u32).wrapping_add(0x30)).lw(c);
        c.g[V0][0] = flags & 1;
        let replaying = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !replaying {
            c.cycles += 5;
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6b10));
            // ~1/60 per tick (the game's 0.0167934): a one-second blend.
            c.f[12] = blend_rate_60(c, FpuF32(c.f[12])).0;
            c.g[A1][0] = 0x44;
            c.g[A2][0] = 0;
            callee::CHASE_CAM_SET_ALGORITHM.call(c, 0x0016_2280)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// The shared shape of the camera-algorithm constructors: 15 cycles, a `0x20`
/// frame saving `s0` and `ra`, the base constructor, then the algorithm tag at
/// `+0xc` and the class vtable at `+0x10`; returns `self`.
pub(crate) fn camera_algorithm_ctor(c: &mut Cpu, base: Callee, tag: u64, vtable: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        base.call(c, ret)?;
        c.g[V0][0] = tag;
        c.g[V1][0] = 0x0046_0000;
        c.sw((this as u32).wrapping_add(0xc), tag);
        c.g[V1][0] = sx(vtable);
        let vt = c.g[V1][0];
        c.sw((this as u32).wrapping_add(0x10), vt);
        c.g[V0][0] = this;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ManualCamAlgo* ManualCamAlgo_ctor(this)` (0x15fd08): tag 0x51, vtable
/// `D_0045BD10`.
pub fn manual_cam_algo_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(
        c,
        callee::MANUAL_CAM_ALGO_BASE_CTOR,
        0x51,
        0x0045_bd10,
        0x0015_fd1c,
    );
}

/// `EditorCamAlgo* EditorCamAlgo_ctor(this)` (0x15ffb0): tag 0x50, vtable
/// `D_0045BCA8`.
pub fn editor_cam_algo_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(
        c,
        callee::MANUAL_CAM_ALGO_BASE_CTOR,
        0x50,
        0x0045_bca8,
        0x0015_ffc4,
    );
}

/// `ManualChaseCameraAlgorithm* ctor(this)` (0x160438): tag 0xb, vtable
/// `D_0045BAD0`.
pub fn manual_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(
        c,
        callee::CHASE_CAMERA_ALGORITHM_CTOR,
        0xb,
        0x0045_bad0,
        0x0016_044c,
    );
}

/// `IdleChaseCameraAlgorithm* ctor(this)` (0x1616a8): tag 7, vtable
/// `D_0045BB48`.
pub fn idle_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(
        c,
        callee::CHASE_CAMERA_ALGORITHM_CTOR,
        0x7,
        0x0045_bb48,
        0x0016_16bc,
    );
}

/// `LazyChaseCameraAlgorithm* ctor(this)` (0x1682b8): tag 0xd, vtable
/// `D_0045C628`.
pub fn lazy_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(
        c,
        callee::CHASE_CAMERA_ALGORITHM_CTOR,
        0xd,
        0x0045_c628,
        0x0016_82cc,
    );
}

/// `LookBackChaseCameraAlgorithm* ctor(this)` (0x168650): tag 0x20, vtable
/// `D_0045C6A0`.
pub fn look_back_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(
        c,
        callee::CHASE_CAMERA_ALGORITHM_CTOR,
        0x20,
        0x0045_c6a0,
        0x0016_8664,
    );
}

/// `MidRightChaseCameraAlgorithm* ctor(this)` (0x1689c8): tag 9, vtable
/// `D_0045C538`.
pub fn mid_right_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(
        c,
        callee::CHASE_CAMERA_ALGORITHM_CTOR,
        0x9,
        0x0045_c538,
        0x0016_89dc,
    );
}

/// `MidLeftChaseCameraAlgorithm* ctor(this)` (0x168cd8): tag 9, vtable
/// `D_0045C5B0`.
pub fn mid_left_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(
        c,
        callee::CHASE_CAMERA_ALGORITHM_CTOR,
        0x9,
        0x0045_c5b0,
        0x0016_8cec,
    );
}

// ---------------------------------------------------------------------------
// ChaseCamController_ExitSpokeMode.

/// `void ChaseCamController_ExitSpokeMode(this)` (0x162218): clear the
/// spoke-mode bits 0 and 1 of the flags at `+0x30`; when neither was set,
/// blend back to the stored algorithm `+0x24` over one 150th of a second.
/// A camera blend rate authored as a per-60-Hz-tick step, as the step per tick
/// at the game's rate: `60 / rate` (exactly the authored value at 60, where
/// nothing changes). A rate of 1.0 ("instant") is left alone by the callers.
#[inline]
pub(crate) fn blend_rate_60(c: &mut Cpu, raw: FpuF32) -> FpuF32 {
    let r = crate::tick::rate(c);
    if r == 60 || raw == FpuF32::ONE {
        raw
    } else {
        raw * FpuF32::from_f32(60.0f32 / r as f32)
    }
}

pub fn chase_cam_controller_exit_spoke_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0xffff_ffff_ffff_fffb;
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[V0][0] = Addr((this as u32).wrapping_add(0x30)).lw(c);
        c.g[V1][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] & 3;
        let in_spoke_mode = c.g[V0][0] != 0;
        let cleared = c.g[V1][0];
        c.sw((this as u32).wrapping_add(0x30), cleared);
        if !in_spoke_mode {
            c.cycles += 4;
            let rate = gp(c, -0x6b14);
            c.lwc1(12, rate);
            // 1/150 per 60 Hz tick: the blend takes 2.5 s at any rate.
            c.f[12] = blend_rate_60(c, FpuF32(c.f[12])).0;
            c.g[A2][0] = 0;
            c.g[A1][0] = Addr((this as u32).wrapping_add(0x24)).lw(c);
            callee::CHASE_CAM_SET_ALGORITHM_2.call(c, 0x0016_2248)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* MatrixChaseCameraAlgorithm_ctor(void* self)` (0x160760): build
/// the base chase-camera algorithm, then the Matrix variant's class id,
/// vtable and reset flag.
pub fn matrix_chase_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CHASE_CAMERA_ALGORITHM_CTOR_2.call(c, 0x0016_0774)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = 0x29;
        c.g[V1][0] = 0x46_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_sub(0x45a8));
        Addr(p(this, 0x10)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(this, 0x390)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ChaseCamController_SetTarget(Ctrl* c, void* p)` (0x161a60): delete
/// the previous target and install `p`.
pub fn chase_cam_controller_set_target(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = crate::abi::r32(c, S0);
        c.g[A0][0] = lw(c, this, 0x20);
        let old = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !old {
            c.cycles += 2;
            callee::BUILTIN_DELETE.call(c, 0x0016_1a88)?;
        }
        c.cycles += 6;
        Addr(p(crate::abi::r32(c, S0), 0x20)).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Chunk (camera/misc) updates.

/// `void* RelativeChaseCameraAlgorithm_ctor(Alg* self, float f)` (0x160b70):
/// the chase-camera algorithm with the relative chase variant: base
/// constructor, its vtable, `+0x390 = f` and `+0x394 = 0`. Returns `self`.
pub fn relative_chase_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x20);
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        callee::CHASE_CAMERA_ALGORITHM_CTOR_3.call(c, 0x0016_0b8c)?;
        c.g[V1][0] = 0x460000;
        c.swc1(20, r32(c, S0).wrapping_add(0x390));
        c.g[V1][0] = sx(0x0046_0000u32.wrapping_sub(0x4620));
        c.g[V0][0] = c.g[S0][0];
        c.sw(r32(c, S0).wrapping_add(0x10), c.g[V1][0]);
        c.sw(r32(c, S0).wrapping_add(0x394), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void ChaseCamController_dtor(this, flags)` (0x161a10): install the chase
/// camera vtable, release the camera object at `+0x20`, then run the
/// CamController destructor.
pub fn chase_cam_controller_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0046_0000u32.wrapping_sub(0x46f8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(c.g[S0][0] as u32).off(0x14).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x20)).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0016_1a3c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        CAM_CONTROLLER_DTOR.call(c, 0x0016_1a48)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera controller.

/// `void ManualCamController_SelectAlgorithm(CamController* c, int msg)`
/// (0x15cf80): a camera message 0x50/0x51 switches the controller to the
/// manual algorithm's field at +0x1c/+0x18. 4 (+5, +4 or +2) + 2 cycles plus
/// the callee.
pub fn manual_cam_controller_select_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x50;
        let msg = c.g[A1][0];
        frame.save_ra(c, 0);
        if msg == c.g[V0][0] {
            c.cycles += 5;
            c.g[A1][0] = c.lw(p(r32(c, A0), 0x1c));
            c.g[A2][0] = 0;
            callee::CAM_CONTROLLER_SET_ALGORITHM.call(c, 0x0015_cfb0)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x51;
            if msg == c.g[V0][0] {
                c.cycles += 4;
                c.g[A1][0] = c.lw(p(r32(c, A0), 0x18));
                c.g[A2][0] = 0;
                callee::CAM_CONTROLLER_SET_ALGORITHM.call(c, 0x0015_cfc4)?;
            } else {
                c.cycles += 2;
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera.

/// `void GameCamera_ResetAllChaseControllers(void)` (0x15dc18): reset the
/// primary chase controller and, when the camera has two, the secondary one.
pub fn game_camera_reset_all_chase_controllers(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    c.g[V0][0] = Addr(gp_2(c, -0x848)).lw(c);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(p_2(c, V0, 0x84)).lw(c);
    c.g[S0][0] = Addr(p_2(c, V1, 0x84)).lw(c);
    if c.g[S0][0] != 0 {
        c.cycles += 7;
        c.g[V0][0] = Addr(p_2(c, S0, 4)).lw(c);
        c.g[A0][0] = Addr(p_2(c, V0, 0xa8)).lw(c);
        if callee::CHASE_CAM_CONTROLLER_RESET_STATE
            .call(c, 0x0015_dc44)
            .is_err()
        {
            return;
        }
        c.g[V0][0] = Addr(p_2(c, S0, 0)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(p_2(c, S0, 8)).lw(c);
            c.g[A0][0] = Addr(p_2(c, V0, 0xa8)).lw(c);
            if callee::CHASE_CAM_CONTROLLER_RESET_STATE
                .call(c, 0x0015_dc60)
                .is_err()
            {
                return;
            }
        }
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void ChaseCamController_SetDefaultAlgorithm(ChaseCamController* self,
/// void* algorithm)` (0x161ef0): publish the default algorithm and apply it
/// immediately when no blend is in progress.
pub fn chase_cam_controller_set_default_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[A1][0];
        Addr(r32(c, A2)).off(0x24).sw(c, c.g[A3][0]);
        c.g[A1][0] = sx(gp_3(c, 0x1ce0));
        c.g[V1][0] = Addr(r32(c, A2)).off(0x20).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x4).lw(c);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x870).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
        c.sw(r32(c, V0), c.g[A3][0]);
        c.g[V1][0] = Addr(r32(c, A2)).off(0x30).lw(c);
        c.g[V1][0] &= 0x7;
        let blending = c.g[V1][0] != 0;
        frame.restore_ra(c, 0);
        if !blending {
            c.cycles += 5;
            c.g[A0][0] = c.g[A2][0];
            c.g[A1][0] = c.g[A3][0];
            c.g[A2][0] = 0;
            c.g[RA][0] = sx(0x0016_1f40);
            callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE.call(c, 0x0016_1f40)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera getters.

/// Copy the quadword at `*(self + 4) + off` to `out` and return `out`.
pub(crate) fn camera_copy_quad(c: &mut Cpu, off: u32) {
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

/// A `gp_4`-relative global.
#[inline(always)]
pub(crate) fn gp_4(c: &Cpu, off: i32) -> u32 {
    r(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Camera tuning blocks.

/// The four tuning floats and the `1` flag block at `gp_4 + base`:
/// `y, x, w, z` in address order, written to `out + 4, 0, 0xc, 8`, plus the
/// `1` at `out + 0x10`.
pub(crate) fn write_tuning(c: &mut Cpu, out: u32, base: i32) {
    c.cycles += 11;
    c.lwc1(3, gp_4(c, base));
    c.g[V0][0] = 1;
    c.lwc1(2, gp_4(c, base + 4));
    c.lwc1(0, gp_4(c, base + 8));
    c.lwc1(1, gp_4(c, base + 0xc));
    c.sw(p(out, 0x10), 1);
    c.swc1(3, p(out, 4));
    c.swc1(2, out);
    c.swc1(0, p(out, 0xc));
    c.swc1(1, p(out, 8));
    abi::ret(c);
}

/// `MidLeftChaseCameraAlgorithm_GetTuning` (0x00168d50).
pub fn mid_left_chase_camera_algorithm_get_tuning(c: &mut Cpu) {
    write_tuning(c, abi::arg(c, 1), -0x6608);
}

// ---------------------------------------------------------------------------
// Camera.

/// `RiderCameraAlgorithm* RiderCameraAlgorithm_ctor(RiderCameraAlgorithm* self)`
/// (0x1622b0): camera base ctor, then the algorithm's constants and vtable.
pub fn rider_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4220));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p_3(c.g[S0][0], 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0016_7e30, 0x0016_22d0)?;
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 1);
        c.g[V1][0] = 0x0046_0000;
        c.lwc1(2, p_3(c.g[GP][0], -0x6b0c_i32 as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x43c8));
        c.g[A0][0] = 0x53;
        c.swc1(0, c.g[S0][0] as u32);
        c.swc1(1, p_3(c.g[S0][0], 4));
        c.g[V0][0] = c.g[S0][0];
        c.swc1(2, p_3(c.g[S0][0], 8));
        Addr(p_3(c.g[S0][0], 0x10)).sw(c, c.g[V1][0]);
        Addr(p_3(c.g[S0][0], 0xc)).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void* ChaseCamController_ctor(void* self, void* a1, void* a2)` (0x1619a8).
pub fn chase_cam_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        call_ptr(c, 0x0015_cba0, 0x0016_19c8)?;
        c.g[V1][0] = 0x0046_0000;
        c.g[A2][0] = 0x2;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x46f8));
        c.g[V0][0] = 0x1;
        Addr(p(c.g[S0][0] as u32, 0x20)).sw(c, 0);
        c.g[A1][0] = c.g[S1][0];
        Addr(p(c.g[S0][0] as u32, 0x14)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.g[S0][0];
        Addr(c.g[S0][0] as u32).sw(c, c.g[A2][0]);
        Addr(p(c.g[S0][0] as u32, 4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0016_1950, 0x0016_19f4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SplineCameraAlgorithm_dtor(this, flags)` 0x161370: restore the base
/// vtable, free the three spline segments and run the camera dtor.
pub fn spline_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0045_b968);
        c.g[S1][0] = abi::arg64(c, 1);
        Addr(p(this, 0x10)).sw(c, c.g[V0][0]);
        for (off, ret) in [
            (0x3a8u32, 0x0016_13a0u32),
            (0x39c, 0x0016_13ac),
            (0x390, 0x0016_13b8),
        ] {
            c.g[A0][0] = sx(p(this, off));
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_d618, ret)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0016_2458, 0x0016_13c4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ChaseCamController_ClearOverride(cam)` 0x1620d0: clear the override flag
/// (bit 1 of `+0x30`), pick the algorithm mode (from `+0x24` when the current
/// one is 0x5d), reset the collision flag and blend to the mode.
pub fn chase_cam_controller_clear_override(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[V0][0] = sx(0xffff_fffe);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x5d;
        let mut mode = u32_at(c, this, 0x2c);
        c.g[S1][0] = sx(mode);
        let flags = u32_at(c, this, 0x30) & (c.g[V0][0] as u32);
        c.g[V1][0] = sx(flags);
        c.g[S1][0] = sx(mode);
        Addr(p(this, 0x30)).sw(c, c.g[V1][0]);
        if mode == 0x5d {
            c.cycles += 1;
            mode = u32_at(c, this, 0x24);
            c.g[S1][0] = sx(mode);
        }
        c.cycles += 13;
        call_ptr(c, 0x0015_dac0, 0x0016_210c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0015_d078, 0x0016_2124)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ChaseCameraAlgorithm_ApplyAirSwing(ChaseCameraAlgorithm* self,
/// Info* info, float amount /*f12*/)` (0x164878): the air-swing camera
/// impulse. Sets the swing flag (`gp_5-0x1e64`), advances the swing phase
/// (`+0x22c`/`+0x234`), drives the spline at_3 `+0x334` (init, nine points,
/// evaluate) and rotates/offsets the camera's target vectors (`+0x110`,
/// `+0x120`), the smoothing state (`+0x130`..) and the orbit vectors.
/// Virtual calls go through the view object at_3 `+0x30`.
pub fn chase_camera_algorithm_apply_air_swing(c: &mut Cpu) {
    'b_16531c: {
        'b_165318: {
            'b_16518c: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
                c.sw(gp_5(c, -0x1e64), 0u64);
                c.sq(17, at_3(c, SP, 0xc0));
                c.sq(18, at_3(c, SP, 0xb0));
                c.g[S1][0] = c.g[A0][0];
                c.swc1(20, at_3(c, SP, 0xe0));
                c.g[S2][0] = c.g[A1][0];
                c.sq(16, at_3(c, SP, 0xd0));
                c.mov_s(20, 12);
                c.sd(at_3(c, SP, 0xa0), c.g[RA][0]);
                c.swc1(21, at_3(c, SP, 0xe8));
                c.lqc2(3, at_3(c, S1, 0x20));
                c.lqc2(4, at_3(c, S1, 0x40));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, SP, 0x10));
                c.lq(2, at_3(c, SP, 0x10));
                c.lq(3, at_3(c, S1, 0x40));
                c.g[A0][0] = c.lw(at_3(c, S1, 0x2d0));
                c.sq(2, (c.g[SP][0] as u32));
                let t = c.g[A0][0] == 0u64;
                c.sq(3, at_3(c, SP, 0x10));
                if t {
                    break 'b_16518c;
                } else {
                    'b_164fb0: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at_3(c, S1, 0x2d8));
                        if c.g[V0][0] != 0u64 {
                            c.lwc1(1, at_3(c, S1, 0x234));
                            break 'b_164fb0;
                        } else {
                            'b_164fa4: {
                                c.cycles += 12;
                                c.lwc1(0, at_3(c, S1, 0x22c));
                                c.lwc1(1, gp_5(c, -0x6920));
                                c.g[A1][0] = c.lw(at_3(c, S1, 0x30));
                                c.add_s(0, 0, 1);
                                c.swc1(0, at_3(c, S1, 0x22c));
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                c.g[A0][0] = c.lh(at_3(c, V1, 0x98));
                                c.g[V0][0] = c.lw(at_3(c, V1, 0x9c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x00164900);
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                if call_ptr(c, t, 0x00164900).is_err() {
                                    return;
                                }
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw(at_3(c, S1, 0x2d8));
                                    break 'b_164fa4;
                                } else {
                                    'b_1649a4: {
                                        c.cycles += 31;
                                        c.lqc2(3, at_3(c, S1, 0x20));
                                        c.lqc2(4, at_3(c, S1, 0x40));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, at_3(c, SP, 0x30));
                                        c.g[S0][0] = sx(at_3(c, S1, 0x230));
                                        c.lq(2, at_3(c, SP, 0x30));
                                        c.g[A1][0] = c.lw(at_3(c, S1, 0x30));
                                        c.sq(2, at_3(c, S1, 0x110));
                                        c.sq(2, at_3(c, SP, 0x20));
                                        c.lqc2(3, at_3(c, S1, 0x110));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.vu0().sqrt(4, 0);
                                        c.vu0().waitq();
                                        c.cfc2(3, 22);
                                        c.mtc1(3, 21);
                                        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                        c.g[A0][0] = c.lh(at_3(c, V0, 0x60));
                                        c.g[V1][0] = c.lw(at_3(c, V0, 0x64));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00164968);
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        if call_ptr(c, t, 0x00164968).is_err() {
                                            return;
                                        }
                                        c.lwc1(1, at_3(c, S1, 0x22c));
                                        c.lwc1(2, gp_5(c, -0x691c));
                                        c.sub_s(0, 0, 1);
                                        c.c_le_s(2, 0);
                                        if !c.fcc() {
                                            c.mov_s(0, 2);
                                            break 'b_1649a4;
                                        } else {
                                            c.cycles += 8;
                                            c.g[A1][0] = c.lw(at_3(c, S1, 0x30));
                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                            c.g[A0][0] = c.lh(at_3(c, V0, 0x60));
                                            c.g[V1][0] = c.lw(at_3(c, V0, 0x64));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x0016499c);
                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            if call_ptr(c, t, 0x0016499c).is_err() {
                                                return;
                                            }
                                            c.lwc1(1, at_3(c, S1, 0x22c));
                                            c.sub_s(0, 0, 1);
                                            break 'b_1649a4;
                                        }
                                    }
                                    'b_164a88: {
                                        c.cycles += 50;
                                        c.swc1(0, (c.g[S0][0] as u32));
                                        c.swc1(20, at_3(c, S1, 0x23c));
                                        c.sw(at_3(c, S1, 0x234), 0u64);
                                        c.g[A1][0] = c.lw(at_3(c, S1, 0x30));
                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                        c.g[A0][0] = c.lh(at_3(c, V1, 0x80));
                                        c.g[V0][0] = c.lw(at_3(c, V1, 0x84));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001649c8);
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        if call_ptr(c, t, 0x001649c8).is_err() {
                                            return;
                                        }
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A2][0] = 0x500000;
                                        c.lqc2(4, (c.g[A2][0] as u32).wrapping_sub(0xea0));
                                        c.lqc2(5, (c.g[A0][0] as u32));
                                        c.vu0().opmula(4, 5);
                                        c.vu0().opmsub(6, 5, 4);
                                        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                                        c.sqc2(6, at_3(c, SP, 0x30));
                                        c.mtc1(0, 3);
                                        c.lq(2, at_3(c, SP, 0x30));
                                        c.g[A1][0] = c.g[A0][0];
                                        c.sq(2, at_3(c, SP, 0x20));
                                        c.lqc2(3, at_3(c, SP, 0x20));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.vu0().sqrt(4, 0);
                                        c.vu0().waitq();
                                        c.cfc2(3, 22);
                                        c.mtc1(3, 0);
                                        c.mfc1(2, 0);
                                        c.qmtc2(2, 3);
                                        c.vu0().div(0, 3, 3, 0);
                                        c.lqc2(4, at_3(c, SP, 0x20));
                                        c.vu0().waitq();
                                        c.vu0().mul(0b1111, 5, 4, Src::Q);
                                        c.sqc2(5, at_3(c, SP, 0x20));
                                        c.lqc2(3, at_3(c, SP, 0x20));
                                        c.lqc2(5, at_3(c, S2, 0x20));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 1);
                                        c.c_lt_s(1, 3);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_164a88;
                                        } else {
                                            c.cycles += 7;
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 0);
                                            c.mfc1(2, 0);
                                            c.lqc2(4, at_3(c, SP, 0x20));
                                            c.qmtc2(2, 3);
                                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                            c.sqc2(5, at_3(c, SP, 0x20));
                                            break 'b_164a88;
                                        }
                                    }
                                    'b_164bb4: {
                                        c.cycles += 74;
                                        c.lqc2(3, at_3(c, SP, 0x20));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.vu0().sqrt(4, 0);
                                        c.vu0().waitq();
                                        c.cfc2(2, 22);
                                        c.mtc1(2, 0);
                                        c.g[AT][0] = 0x40000000;
                                        c.mtc1(1, 2);
                                        c.div_s(0, 21, 0);
                                        c.mfc1(2, 0);
                                        c.lqc2(4, at_3(c, SP, 0x20));
                                        c.qmtc2(2, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at_3(c, SP, 0x20));
                                        c.lqc2(3, (c.g[A0][0] as u32));
                                        c.lqc2(5, (c.g[A0][0] as u32));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.lqc2(3, at_3(c, S1, 0x110));
                                        c.lqc2(5, (c.g[A0][0] as u32));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 1);
                                        c.div_s(1, 1, 0);
                                        c.mfc1(3, 1);
                                        c.lqc2(4, (c.g[A0][0] as u32));
                                        c.qmtc2(3, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at_3(c, SP, 0x60));
                                        c.lq(2, at_3(c, SP, 0x60));
                                        c.sq(2, at_3(c, SP, 0x50));
                                        c.mfc1(3, 2);
                                        c.lqc2(4, at_3(c, SP, 0x50));
                                        c.qmtc2(3, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at_3(c, SP, 0x60));
                                        c.lq(2, at_3(c, SP, 0x60));
                                        c.sq(2, at_3(c, SP, 0x40));
                                        c.lqc2(3, at_3(c, S1, 0x110));
                                        c.lqc2(4, at_3(c, SP, 0x40));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, at_3(c, SP, 0x60));
                                        c.lq(2, at_3(c, SP, 0x60));
                                        c.sq(2, at_3(c, S1, 0x120));
                                        c.sq(2, at_3(c, SP, 0x30));
                                        c.lwc1(0, at_3(c, S1, 0x128));
                                        c.neg_s(0, 0);
                                        c.swc1(0, at_3(c, S1, 0x128));
                                        c.lqc2(3, at_3(c, S1, 0x120));
                                        c.lqc2(5, at_3(c, S1, 0x120));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 1);
                                        c.c_le_s(1, 3);
                                        let t = !c.fcc();
                                        c.lq(2, at_3(c, SP, 0x20));
                                        if t {
                                            break 'b_164bb4;
                                        } else {
                                            c.cycles += 1;
                                            c.sq(2, at_3(c, S1, 0x120));
                                            break 'b_164bb4;
                                        }
                                    }
                                    'b_164d98: {
                                        c.cycles += 80;
                                        c.lwc1(0, gp_5(c, -0x6918));
                                        c.mfc1(2, 0);
                                        c.lqc2(4, at_3(c, S1, 0x120));
                                        c.qmtc2(2, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at_3(c, S1, 0x120));
                                        c.lwc1(1, gp_5(c, -0x6914));
                                        c.mfc1(2, 1);
                                        c.lqc2(4, at_3(c, SP, 0x20));
                                        c.qmtc2(2, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at_3(c, SP, 0x40));
                                        c.lwc1(2, gp_5(c, -0x6910));
                                        c.lq(2, at_3(c, SP, 0x40));
                                        c.sq(2, at_3(c, SP, 0x30));
                                        c.lqc2(3, at_3(c, S1, 0x120));
                                        c.lqc2(4, at_3(c, SP, 0x30));
                                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, at_3(c, S1, 0x120));
                                        c.lqc2(3, at_3(c, S1, 0x120));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.vu0().sqrt(4, 0);
                                        c.vu0().waitq();
                                        c.cfc2(2, 22);
                                        c.mtc1(2, 0);
                                        c.div_s(0, 21, 0);
                                        c.mfc1(2, 0);
                                        c.lqc2(4, at_3(c, S1, 0x120));
                                        c.qmtc2(2, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at_3(c, S1, 0x120));
                                        c.lqc2(3, (c.g[A1][0] as u32));
                                        c.lqc2(5, (c.g[A1][0] as u32));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.lqc2(3, at_3(c, S1, 0x120));
                                        c.lqc2(5, (c.g[A1][0] as u32));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 1);
                                        c.div_s(1, 1, 0);
                                        c.mfc1(3, 1);
                                        c.lqc2(4, (c.g[A1][0] as u32));
                                        c.qmtc2(3, 3);
                                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                        c.sqc2(5, at_3(c, SP, 0x40));
                                        c.lq(2, at_3(c, SP, 0x40));
                                        c.sq(2, at_3(c, SP, 0x30));
                                        c.sq(2, at_3(c, S1, 0x150));
                                        c.lqc2(3, at_3(c, S1, 0x120));
                                        c.lqc2(4, at_3(c, S1, 0x150));
                                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                        c.sqc2(5, at_3(c, SP, 0x40));
                                        c.lq(2, at_3(c, SP, 0x40));
                                        c.sq(2, at_3(c, SP, 0x30));
                                        c.lqc2(3, at_3(c, SP, 0x30));
                                        c.lqc2(5, at_3(c, SP, 0x30));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 0);
                                        c.c_lt_s(0, 2);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_164d98;
                                        } else {
                                            c.cycles += 16;
                                            c.swc1(3, at_3(c, SP, 0x38));
                                            c.g[AT][0] = 0x3f800000;
                                            c.mtc1(1, 0);
                                            c.lqc2(3, at_3(c, SP, 0x30));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.vu0().sqrt(4, 0);
                                            c.vu0().waitq();
                                            c.cfc2(2, 22);
                                            c.mtc1(2, 2);
                                            c.c_lt_s(0, 2);
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_164d98;
                                            } else {
                                                c.cycles += 25;
                                                c.g[AT][0] = 0x43480000;
                                                c.mtc1(1, 0);
                                                c.g[AT][0] = 0x42c80000;
                                                c.mtc1(1, 1);
                                                c.div_s(0, 0, 2);
                                                c.mfc1(2, 0);
                                                c.lqc2(4, at_3(c, SP, 0x30));
                                                c.qmtc2(2, 3);
                                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                c.sqc2(5, at_3(c, SP, 0x30));
                                                c.lqc2(3, at_3(c, S1, 0x120));
                                                c.lqc2(4, at_3(c, SP, 0x30));
                                                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                c.sqc2(5, at_3(c, S1, 0x120));
                                                c.mfc1(3, 1);
                                                c.lqc2(4, (c.g[A2][0] as u32).wrapping_sub(0xea0));
                                                c.qmtc2(3, 3);
                                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                c.sqc2(5, at_3(c, SP, 0x50));
                                                c.lq(2, at_3(c, SP, 0x50));
                                                c.sq(2, at_3(c, SP, 0x40));
                                                c.lqc2(3, at_3(c, S1, 0x120));
                                                c.lqc2(4, at_3(c, SP, 0x40));
                                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                                c.sqc2(5, at_3(c, S1, 0x120));
                                                break 'b_164d98;
                                            }
                                        }
                                    }
                                    'b_164ec8: {
                                        'b_164e6c: {
                                            c.cycles += 50;
                                            c.lqc2(4, at_3(c, S1, 0x110));
                                            c.lqc2(5, at_3(c, S1, 0x120));
                                            c.vu0().opmula(4, 5);
                                            c.vu0().opmsub(6, 5, 4);
                                            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                                            c.sqc2(6, at_3(c, SP, 0x50));
                                            c.lwc1(2, gp_5(c, -0x690c));
                                            c.lq(2, at_3(c, SP, 0x50));
                                            c.sq(2, at_3(c, S1, 0x130));
                                            c.sq(2, at_3(c, SP, 0x40));
                                            c.lqc2(3, at_3(c, S1, 0x130));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.vu0().sqrt(4, 0);
                                            c.vu0().waitq();
                                            c.cfc2(3, 22);
                                            c.mtc1(3, 0);
                                            c.mfc1(2, 0);
                                            c.qmtc2(2, 3);
                                            c.vu0().div(0, 3, 3, 0);
                                            c.lqc2(4, at_3(c, S1, 0x130));
                                            c.vu0().waitq();
                                            c.vu0().mul(0b1111, 5, 4, Src::Q);
                                            c.sqc2(5, at_3(c, S1, 0x130));
                                            c.lqc2(3, at_3(c, S1, 0x110));
                                            c.lqc2(5, at_3(c, S1, 0x110));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(3, 4);
                                            c.mtc1(3, 1);
                                            c.lqc2(3, at_3(c, S1, 0x120));
                                            c.lqc2(5, at_3(c, S1, 0x120));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 0);
                                            c.mul_s(1, 1, 0);
                                            c.sqrt_s(1, 1);
                                            c.c_lt_s(1, 2);
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_164e6c;
                                            } else {
                                                c.cycles += 3;
                                                c.mtc1(0, 1);
                                                let t = true;

                                                if t {
                                                    break 'b_164ec8;
                                                } else {
                                                    break 'b_164e6c;
                                                }
                                            }
                                        }
                                        'b_164eb8: {
                                            c.cycles += 16;
                                            c.lqc2(3, at_3(c, S1, 0x110));
                                            c.lqc2(5, at_3(c, S1, 0x120));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 0);
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 2);
                                            c.div_s(12, 0, 1);
                                            c.c_le_s(2, 12);
                                            if !c.fcc() {
                                                c.mov_s(12, 2);
                                                break 'b_164eb8;
                                            } else {
                                                c.cycles += 3;
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 0);
                                                c.min_s(12, 12, 0);
                                                break 'b_164eb8;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x00164ec0);
                                        if call_ptr(c, 0x0031c128, 0x00164ec0).is_err() {
                                            return;
                                        }
                                        c.lwc1(1, gp_5(c, -0x6908));
                                        c.sub_s(1, 1, 0);
                                        break 'b_164ec8;
                                    }
                                    c.cycles += 55;
                                    c.mtc1(0, 20);
                                    c.g[S0][0] = sx(at_3(c, S1, 0x334));
                                    c.swc1(1, at_3(c, S1, 0x238));
                                    c.g[A1][0] = 0x9;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.mov_s(12, 20);
                                    c.g[RA][0] = sx(0x00164ee8);
                                    c.mov_s(13, 20);
                                    if call_ptr(c, 0x0031d738, 0x00164ee8).is_err() {
                                        return;
                                    }
                                    c.mov_s(12, 20);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00164ef8);
                                    c.mov_s(13, 12);
                                    if call_ptr(c, 0x0031d7e0, 0x00164ef8).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3e000000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp_5(c, -0x6904));
                                    c.g[RA][0] = sx(0x00164f0c);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f0c).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3e800000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp_5(c, -0x6900));
                                    c.g[RA][0] = sx(0x00164f20);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f20).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3ec00000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp_5(c, -0x68fc));
                                    c.g[RA][0] = sx(0x00164f34);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f34).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3f000000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp_5(c, -0x68f8));
                                    c.g[RA][0] = sx(0x00164f48);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f48).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3f200000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp_5(c, -0x68f4));
                                    c.g[RA][0] = sx(0x00164f5c);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f5c).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3f400000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp_5(c, -0x68f0));
                                    c.g[RA][0] = sx(0x00164f70);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f70).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3f600000;
                                    c.mtc1(1, 12);
                                    c.lwc1(13, gp_5(c, -0x68ec));
                                    c.g[RA][0] = sx(0x00164f84);
                                    c.g[A0][0] = c.g[S0][0];
                                    if call_ptr(c, 0x0031d7e0, 0x00164f84).is_err() {
                                        return;
                                    }
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 12);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00164f98);
                                    c.mov_s(13, 12);
                                    if call_ptr(c, 0x0031d7e0, 0x00164f98).is_err() {
                                        return;
                                    }
                                    c.g[V0][0] = 0x1;
                                    c.sw(at_3(c, S1, 0x2d8), c.g[V0][0]);
                                    c.g[V0][0] = c.lw(at_3(c, S1, 0x2d8));
                                    break 'b_164fa4;
                                }
                            }
                            c.cycles += 2;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw(at_3(c, S1, 0x2d0));
                                break 'b_16531c;
                            } else {
                                c.cycles += 1;
                                c.lwc1(1, at_3(c, S1, 0x234));
                                break 'b_164fb0;
                            }
                        }
                    }
                    'b_1650a4: {
                        c.cycles += 7;
                        c.lwc1(0, gp_5(c, -0x68e8));
                        c.lwc1(2, at_3(c, S1, 0x230));
                        c.add_s(1, 1, 0);
                        c.c_lt_s(1, 2);
                        let t = !c.fcc();
                        c.swc1(1, at_3(c, S1, 0x234));
                        if t {
                            break 'b_1650a4;
                        } else {
                            c.cycles += 54;
                            c.div_s(12, 1, 2);
                            c.g[V1][0] = sx(at_3(c, S1, 0x334));
                            c.g[V0][0] = 0x1;
                            c.g[A0][0] = c.g[V1][0];
                            c.sw(gp_5(c, -0x1e64), c.g[V0][0]);
                            c.g[RA][0] = sx(0x00164fe8);
                            c.sw(at_3(c, GP, 0x1cf0), c.g[V1][0]);
                            if call_ptr(c, 0x0031dee0, 0x00164fe8).is_err() {
                                return;
                            }
                            c.lwc1(12, at_3(c, S1, 0x238));
                            c.mov_s(20, 0);
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 0);
                            c.g[A0][0] = sx(at_3(c, SP, 0x90));
                            c.mul_s(12, 20, 12);
                            c.g[A1][0] = sx(at_3(c, SP, 0x94));
                            c.g[RA][0] = sx(0x0016500c);
                            c.mul_s(12, 12, 0);
                            if call_ptr(c, 0x0031be50, 0x0016500c).is_err() {
                                return;
                            }
                            c.lwc1(1, at_3(c, S1, 0x130));
                            c.lwc1(2, at_3(c, S1, 0x134));
                            c.lwc1(0, at_3(c, SP, 0x90));
                            c.lwc1(4, at_3(c, S1, 0x138));
                            c.mul_s(1, 0, 1);
                            c.lwc1(3, at_3(c, SP, 0x94));
                            c.mul_s(2, 0, 2);
                            c.mul_s(0, 0, 4);
                            c.swc1(3, at_3(c, SP, 0x3c));
                            c.swc1(1, at_3(c, SP, 0x30));
                            c.swc1(2, at_3(c, SP, 0x34));
                            c.swc1(0, at_3(c, SP, 0x38));
                            c.lq(2, at_3(c, SP, 0x30));
                            c.sq(2, at_3(c, SP, 0x20));
                            c.lqc2(4, at_3(c, SP, 0x20));
                            c.lqc2(5, at_3(c, S1, 0x110));
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
                            c.sqc2(8, at_3(c, SP, 0x40));
                            c.lq(2, at_3(c, SP, 0x40));
                            c.sq(2, at_3(c, S1, 0x140));
                            c.sq(2, at_3(c, SP, 0x30));
                            c.mfc1(3, 20);
                            c.lqc2(4, at_3(c, S1, 0x150));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, at_3(c, SP, 0x40));
                            c.lq(2, at_3(c, SP, 0x40));
                            c.sq(2, at_3(c, S1, 0x160));
                            c.sq(2, at_3(c, SP, 0x30));
                            break 'b_1650a4;
                        }
                    }
                    'b_1650f8: {
                        c.cycles += 21;
                        c.lwc1(1, gp_5(c, -0x68e4));
                        c.lwc1(0, at_3(c, S1, 0x230));
                        c.lwc1(2, at_3(c, S1, 0x234));
                        c.mul_s(0, 0, 1);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 4);
                        c.mtc1(0, 1);
                        c.g[AT][0] = 0x40400000;
                        c.mtc1(1, 3);
                        c.div_s(2, 2, 0);
                        c.min_s(2, 2, 4);
                        c.max_s(2, 2, 1);
                        c.mul_s(0, 2, 2);
                        c.add_s(1, 0, 0);
                        c.mul_s(0, 0, 3);
                        c.mul_s(1, 1, 2);
                        c.sub_s(0, 0, 1);
                        c.c_lt_s(4, 0);
                        if c.fcc() {
                            c.mov_s(0, 4);
                            break 'b_1650f8;
                        } else {
                            break 'b_1650f8;
                        }
                    }
                    c.cycles += 37;
                    c.mov_s(2, 0);
                    c.sub_s(0, 4, 2);
                    c.mfc1(3, 0);
                    c.lqc2(4, at_3(c, S1, 0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at_3(c, SP, 0x40));
                    c.lq(2, at_3(c, SP, 0x40));
                    c.sq(2, at_3(c, SP, 0x30));
                    c.mfc1(3, 2);
                    c.lqc2(4, at_3(c, S2, 0x10));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at_3(c, SP, 0x50));
                    c.lq(2, at_3(c, SP, 0x50));
                    c.sq(2, at_3(c, SP, 0x40));
                    c.lqc2(3, at_3(c, SP, 0x30));
                    c.lqc2(4, at_3(c, SP, 0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at_3(c, SP, 0x50));
                    c.lq(2, at_3(c, SP, 0x50));
                    c.sq(2, at_3(c, S1, 0x20));
                    c.sq(2, at_3(c, SP, 0x20));
                    c.lqc2(3, at_3(c, S1, 0x20));
                    c.lqc2(4, at_3(c, S1, 0x140));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at_3(c, SP, 0x40));
                    c.lq(2, at_3(c, SP, 0x40));
                    c.sq(2, at_3(c, SP, 0x30));
                    c.lqc2(3, at_3(c, SP, 0x30));
                    c.lqc2(4, at_3(c, S1, 0x160));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at_3(c, SP, 0x40));
                    c.lq(2, at_3(c, SP, 0x40));
                    c.sq(2, at_3(c, S1, 0x40));
                    let t = true;
                    c.sq(2, at_3(c, SP, 0x20));
                    if t {
                        break 'b_165318;
                    } else {
                        break 'b_16518c;
                    }
                }
            }
            c.cycles += 6;
            c.lwc1(3, at_3(c, S1, 0x23c));
            c.mtc1(0, 1);
            c.c_lt_s(1, 3);
            if !c.fcc() {
                c.g[V0][0] = c.lw(at_3(c, S1, 0x2d0));
                break 'b_16531c;
            } else {
                'b_1651ec: {
                    c.cycles += 18;
                    c.lwc1(0, gp_5(c, -0x68e0));
                    c.g[AT][0] = 0x40400000;
                    c.mtc1(1, 2);
                    c.sub_s(0, 3, 0);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.max_s(0, 0, 1);
                    c.div_s(12, 0, 20);
                    c.swc1(0, at_3(c, S1, 0x23c));
                    c.mul_s(1, 12, 12);
                    c.add_s(0, 1, 1);
                    c.mul_s(1, 1, 2);
                    c.mul_s(0, 0, 12);
                    c.sub_s(0, 1, 0);
                    c.c_lt_s(3, 0);
                    if c.fcc() {
                        c.mov_s(0, 3);
                        break 'b_1651ec;
                    } else {
                        break 'b_1651ec;
                    }
                }
                c.cycles += 75;
                c.lq(2, at_3(c, S1, 0x20));
                c.mov_s(12, 0);
                c.sub_s(0, 3, 12);
                c.sq(2, at_3(c, SP, 0x20));
                c.mfc1(3, 0);
                c.lqc2(4, at_3(c, S1, 0x20));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_3(c, SP, 0x50));
                c.lq(2, at_3(c, SP, 0x50));
                c.sq(2, at_3(c, SP, 0x40));
                c.mfc1(3, 12);
                c.lqc2(4, at_3(c, S2, 0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_3(c, SP, 0x60));
                c.lq(2, at_3(c, SP, 0x60));
                c.sq(2, at_3(c, SP, 0x50));
                c.lqc2(3, at_3(c, SP, 0x40));
                c.lqc2(4, at_3(c, SP, 0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, SP, 0x60));
                c.lq(2, at_3(c, SP, 0x60));
                c.sq(2, at_3(c, S1, 0x20));
                c.sq(2, at_3(c, SP, 0x30));
                c.mfc1(3, 0);
                c.lqc2(4, at_3(c, S1, 0x40));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_3(c, SP, 0x50));
                c.lq(2, at_3(c, SP, 0x50));
                c.sq(2, at_3(c, SP, 0x40));
                c.lqc2(3, at_3(c, S2, 0x10));
                c.lqc2(4, at_3(c, S1, 0x140));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, SP, 0x80));
                c.lq(2, at_3(c, SP, 0x80));
                c.sq(2, at_3(c, SP, 0x70));
                c.lqc2(3, at_3(c, SP, 0x70));
                c.lqc2(4, at_3(c, S1, 0x160));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, SP, 0x80));
                c.lq(2, at_3(c, SP, 0x80));
                c.sq(2, at_3(c, SP, 0x60));
                c.mfc1(3, 12);
                c.lqc2(4, at_3(c, SP, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at_3(c, SP, 0x80));
                c.lq(2, at_3(c, SP, 0x80));
                c.sq(2, at_3(c, SP, 0x50));
                c.lqc2(3, at_3(c, SP, 0x40));
                c.lqc2(4, at_3(c, SP, 0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, SP, 0x80));
                c.lq(2, at_3(c, SP, 0x80));
                c.sq(2, at_3(c, S1, 0x40));
                c.sq(2, at_3(c, SP, 0x30));
                c.lqc2(3, at_3(c, S1, 0x20));
                c.lqc2(4, at_3(c, S1, 0x40));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, SP, 0x40));
                c.lq(2, at_3(c, SP, 0x40));
                c.sq(2, at_3(c, S1, 0x170));
                c.sq(2, at_3(c, SP, 0x30));
                c.lqc2(3, at_3(c, S1, 0x20));
                c.lqc2(4, at_3(c, SP, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at_3(c, SP, 0x40));
                c.lq(2, at_3(c, SP, 0x40));
                c.lwc1(0, at_3(c, S1, 0x23c));
                c.sq(2, at_3(c, S1, 0x180));
                c.sq(2, at_3(c, SP, 0x30));
                c.swc1(0, at_3(c, S1, 0x240));
                c.swc1(0, at_3(c, S1, 0x244));
                break 'b_165318;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw(at_3(c, S1, 0x2d0));
        break 'b_16531c;
    }
    'b_165448: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;

        if t {
            break 'b_165448;
        } else {
            c.cycles += 5;
            c.lwc1(4, at_3(c, S1, 0x240));
            c.mtc1(0, 3);
            c.c_lt_s(3, 4);
            let t = !c.fcc();

            if t {
                break 'b_165448;
            } else {
                c.cycles += 4;
                c.lwc1(1, at_3(c, S1, 0x244));
                c.c_lt_s(3, 1);
                let t = !c.fcc();

                if t {
                    break 'b_165448;
                } else {
                    'b_165390: {
                        c.cycles += 18;
                        c.lwc1(0, gp_5(c, -0x68dc));
                        c.g[AT][0] = 0x40400000;
                        c.mtc1(1, 2);
                        c.sub_s(0, 4, 0);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 4);
                        c.max_s(0, 0, 3);
                        c.div_s(3, 0, 1);
                        c.swc1(0, at_3(c, S1, 0x240));
                        c.mul_s(1, 3, 3);
                        c.add_s(0, 1, 1);
                        c.mul_s(1, 1, 2);
                        c.mul_s(0, 0, 3);
                        c.sub_s(0, 1, 0);
                        c.c_lt_s(4, 0);
                        if c.fcc() {
                            c.mov_s(0, 4);
                            break 'b_165390;
                        } else {
                            break 'b_165390;
                        }
                    }
                    c.cycles += 46;
                    c.lqc2(3, at_3(c, S1, 0x20));
                    c.lqc2(4, at_3(c, S1, 0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at_3(c, SP, 0x30));
                    c.mov_s(3, 0);
                    c.lq(2, at_3(c, SP, 0x30));
                    c.sub_s(0, 4, 3);
                    c.sq(2, at_3(c, SP, 0x20));
                    c.mfc1(3, 3);
                    c.lqc2(4, at_3(c, S1, 0x170));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at_3(c, SP, 0x50));
                    c.lq(2, at_3(c, SP, 0x50));
                    c.sq(2, at_3(c, SP, 0x40));
                    c.mfc1(3, 0);
                    c.lqc2(4, at_3(c, SP, 0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at_3(c, SP, 0x60));
                    c.lq(2, at_3(c, SP, 0x60));
                    c.sq(2, at_3(c, SP, 0x50));
                    c.lqc2(3, at_3(c, SP, 0x40));
                    c.lqc2(4, at_3(c, SP, 0x50));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at_3(c, SP, 0x60));
                    c.lq(2, at_3(c, SP, 0x60));
                    c.sq(2, at_3(c, SP, 0x30));
                    c.mfc1(3, 3);
                    c.lqc2(4, at_3(c, S1, 0x180));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at_3(c, SP, 0x50));
                    c.lq(2, at_3(c, SP, 0x50));
                    c.sq(2, at_3(c, SP, 0x40));
                    c.lqc2(3, at_3(c, S1, 0x20));
                    c.lqc2(4, at_3(c, SP, 0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at_3(c, S1, 0x20));
                    c.lqc2(3, at_3(c, S1, 0x20));
                    c.lqc2(4, at_3(c, SP, 0x30));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at_3(c, SP, 0x50));
                    c.lq(2, at_3(c, SP, 0x50));
                    c.sq(2, at_3(c, S1, 0x40));
                    c.sq(2, at_3(c, SP, 0x40));
                    break 'b_165448;
                }
            }
        }
    }
    'b_165518: {
        c.cycles += 31;
        c.lqc2(3, at_3(c, S1, 0x20));
        c.lqc2(4, at_3(c, S1, 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at_3(c, SP, 0x30));
        c.lwc1(1, gp_5(c, -0x68d8));
        c.lq(2, at_3(c, SP, 0x30));
        c.sq(2, at_3(c, SP, 0x20));
        c.lqc2(3, (c.g[SP][0] as u32));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 0);
        c.lqc2(3, at_3(c, SP, 0x20));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 2);
        c.mul_s(1, 0, 1);
        c.c_lt_s(2, 1);
        let t = !c.fcc();

        if t {
            break 'b_165518;
        } else {
            'b_165514: {
                c.cycles += 6;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.c_lt_s(0, 2);
                let t = !c.fcc();
                c.lq(2, at_3(c, SP, 0x10));
                if t {
                    break 'b_165514;
                } else {
                    c.cycles += 14;
                    c.div_s(0, 1, 2);
                    c.mfc1(2, 0);
                    c.lqc2(4, at_3(c, SP, 0x20));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at_3(c, SP, 0x20));
                    c.lqc2(3, at_3(c, S1, 0x20));
                    c.lqc2(4, at_3(c, SP, 0x20));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at_3(c, SP, 0x40));
                    c.lq(2, at_3(c, SP, 0x40));
                    c.sq(2, at_3(c, S1, 0x40));
                    let t = true;
                    c.sq(2, at_3(c, SP, 0x30));
                    if t {
                        break 'b_165518;
                    } else {
                        break 'b_165514;
                    }
                }
            }
            c.cycles += 1;
            c.sq(2, at_3(c, S1, 0x40));
            break 'b_165518;
        }
    }
    c.cycles += 10;
    c.g[RA][0] = sx(0x00165520);
    c.g[A0][0] = c.g[S1][0];
    if call_ptr(c, 0x00168150, 0x00165520).is_err() {
        return;
    }
    c.lq(16, at_3(c, SP, 0xd0));
    c.lq(17, at_3(c, SP, 0xc0));
    c.lq(18, at_3(c, SP, 0xb0));
    c.g[RA][0] = c.ld(at_3(c, SP, 0xa0));
    c.lwc1(21, at_3(c, SP, 0xe8));
    c.lwc1(20, at_3(c, SP, 0xe0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0xf0));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// Camera and memory card.

/// `ChaseCamController::EnterSpokeMode(self)` (0x1621a8): set the spoke-mode
/// flag and blend to the spoke camera algorithm at 1/60 s per frame (1.2 for
/// the "stuck" rider state, 0.6 otherwise).
pub fn chase_cam_controller_enter_spoke_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x30)).lw(c);
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.g[V1][0] = c.g[V0][0] & 0x3;
        let already = c.g[V1][0] != 0;
        Addr(p(c.g[S0][0] as u32, 0x30)).sw(c, c.g[V0][0]);
        if !already {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x20)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 4)).lw(c);
            call_ptr(c, 0x0011_fe98, 0x0016_21d8)?;
            c.g[V1][0] = 0x5;
            c.lwc1(0, gp_6(c, -0x6b20));
            let stuck = c.g[V0][0] == c.g[V1][0];
            if stuck {
                c.cycles += 1;
                c.lwc1(0, gp_6(c, -0x6b1c));
            }
            c.cycles += 6;
            c.lwc1(12, gp_6(c, -0x6b18));
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x42;
            c.g[A2][0] = 0;
            c.mul_s(12, 0, 12);
            call_ptr(c, 0x0015_d078, 0x0016_2204)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `0x0016_8150`: the chase camera's placement update, called at the end.
pub(crate) const CAMERA_UPDATE: u32 = 0x0016_8150;

/// `vec3`/`vec4` dot product of `src` with itself, left in VF4 (x) and Q, as
/// the original's `lqc2; add; mul; adda; madda; madd` sequence does.
#[inline(always)]
pub(crate) fn v_dot(c: &mut Cpu, src: u32) {
    c.lqc2(3, src);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// Dot product of two vectors: VF4 (x) and Q hold `a · b`.
#[inline(always)]
pub(crate) fn v_dot2(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// [`v_dot`], then `sqrt`, `waitq`, `cfc2 rt, 22` and `mtc1 rt, fs`: the
/// length of `src` in FPU register `fs`.
#[inline(always)]
pub(crate) fn v_length(c: &mut Cpu, src: u32, rt: usize, fs: usize) {
    v_dot(c, src);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(rt, 22);
    c.mtc1(rt, fs);
}

/// The original's `opmula/opmsub/sub` cross product: `dst = a × b`, then its
/// `x` lane is zeroed by the `sub(mask 1)`.
#[inline(always)]
pub(crate) fn v_cross(c: &mut Cpu, a: u32, b: u32, dst: u32) {
    c.lqc2(4, a);
    c.lqc2(5, b);
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, dst);
}

/// `lq 2, src; sq 2, dst`.
#[inline(always)]
pub(crate) fn qcopy(c: &mut Cpu, src: u32, dst: u32) {
    c.lq(2, src);
    c.sq(2, dst);
}

/// `ChaseCameraAlgorithm_ApplyAirSwing` (`0x164878`).
///
/// `void apply_air_swing(ChaseCameraAlgorithm* this, AirSwingInfo* info, float dt)`
///
/// `info+0x10` is a `vec4` offset, `info+0x20` a `vec4` direction.
pub fn chase_camera_apply_air_swing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xf0);
        // "air swing applied this frame" flag.
        Addr(gp(c, -0x1e64)).sw(c, 0);
        frame.save(c, 17, 0xc0);
        frame.save(c, 18, 0xb0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xe0);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 16, 0xd0);
        c.mov_s(20, 12);
        frame.save_ra(c, 0xa0);
        frame.save_fpr(c, 21, 0xe8);
        let me = c.g[S1][0] as u32;
        let info = c.g[S2][0] as u32;

        // The frame's travel direction, kept for the tail.
        c.lqc2(3, p(me, st::POS));
        c.lqc2(4, p(me, st::VEL));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x10));
        c.lq(2, frame.at(c, 0x10));
        c.lq(3, p(me, st::VEL));
        c.g[A0][0] = c.lw(p(me, st::AIRBORNE));
        c.sq(2, frame.at(c, 0));
        let airborne = c.g[A0][0] != 0;
        c.sq(3, frame.at(c, 0x10));
        // Set by the two `break 'b_16531c` paths: they jump past the tail's
        // `cycles += 1` and `v0 = lw(this+0x2d0)`.
        let mut skip_tail = false;
        // The second `break 'b_16531c` (from 0x100485) also jumps past the
        // whole `0x1650a4`/`0x1650f8`/`0x100596` region.
        let mut skip_496 = false;
        // `this->swinging` at entry: its path breaks 'b_164fb0 straight to
        // `0x1650a4`, skipping the `0x100485` block.
        let mut swinging = false;

        if airborne {
            // Airborne: keep the swing, or advance it while it is running.
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(me, st::SWINGING));
            swinging = c.g[V0][0] != 0;
            if swinging {
                c.lwc1(1, p(me, st::AMPLITUDE));
            } else {
                // Not swinging yet: advance the phase and ask the rig for its
                // swing amount (vtable slot +0x9c).
                c.cycles += 12;
                c.lwc1(0, p(me, st::PHASE));
                c.lwc1(1, gp(c, -0x6920));
                c.g[A1][0] = c.lw(p(me, st::RIG));
                c.add_s(0, 0, 1);
                c.swc1(0, p(me, st::PHASE));
                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x98));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x9c));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0016_4900)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw(p(me, st::SWINGING));
                } else {
                    // 0x1649a4: normalize the travel direction, project the
                    // rig's lean onto it and clamp the swing amount.
                    c.cycles += 31;
                    c.lqc2(3, p(me, st::POS));
                    c.lqc2(4, p(me, st::VEL));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x30));
                    c.g[S0][0] = sx(p(me, st::FREQ));
                    c.lq(2, frame.at(c, 0x30));
                    c.g[A1][0] = c.lw(p(me, st::RIG));
                    c.sq(2, p(me, st::AXIS));
                    c.sq(2, frame.at(c, 0x20));
                    v_length(c, p(me, st::AXIS), 3, 21);
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x60));
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x64));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0016_4968)?;
                    c.lwc1(1, p(me, st::PHASE));
                    c.lwc1(2, gp(c, -0x691c));
                    c.sub_s(0, 0, 1);
                    c.c_le_s(2, 0);
                    if !c.fcc() {
                        c.mov_s(0, 2);
                    } else {
                        // Past the limit: ask the rig for the other limit.
                        c.cycles += 8;
                        c.g[A1][0] = c.lw(p(me, st::RIG));
                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                        c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x60));
                        c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x64));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0016_499c)?;
                        c.lwc1(1, p(me, st::PHASE));
                        c.sub_s(0, 0, 1);
                    }

                    // 0x164a88: restart the swing from the lean direction:
                    // cross(travel_dir, one), normalized, dotted with the
                    // rig's forward direction.
                    c.cycles += 50;
                    c.swc1(0, c.g[S0][0] as u32);
                    c.swc1(20, p(me, st::TIME));
                    c.sw(p(me, st::AMPLITUDE), 0);
                    c.g[A1][0] = c.lw(p(me, st::RIG));
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x80));
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x84));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0016_49c8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0x500000;
                    v_cross(
                        c,
                        p(c.g[A2][0] as u32, 0) - 0xea0,
                        c.g[A0][0] as u32,
                        frame.at(c, 0x30),
                    );
                    c.mtc1(0, 3);
                    qcopy(c, frame.at(c, 0x30), frame.at(c, 0x20));
                    c.g[A1][0] = c.g[A0][0];
                    v_length(c, frame.at(c, 0x20), 3, 0);
                    c.mfc1(2, 0);
                    c.qmtc2(2, 3);
                    c.vu0().div(0, 3, 3, 0);
                    c.lqc2(4, frame.at(c, 0x20));
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 4, Src::Q);
                    c.sqc2(5, frame.at(c, 0x20));
                    // How far the swing axis has to lean towards the info
                    // direction.
                    v_dot2(c, frame.at(c, 0x20), p(info, 0x20));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.c_lt_s(1, 3);
                    if c.fcc() {
                        c.cycles += 7;
                        c.g[AT][0] = 0xffff_ffff_bf80_0000;
                        c.mtc1(1, 0);
                        c.mfc1(2, 0);
                        c.lqc2(4, frame.at(c, 0x20));
                        c.qmtc2(2, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, frame.at(c, 0x20));
                    }

                    // 0x164bb4: build the swing basis (axis scaled by the
                    // swing amount, rotated by 90 degrees, and the travel
                    // direction projected onto it).
                    c.cycles += 74;
                    v_length(c, frame.at(c, 0x20), 2, 0);
                    c.g[AT][0] = 0x40000000;
                    c.mtc1(1, 2);
                    c.div_s(0, 21, 0);
                    c.mfc1(2, 0);
                    c.lqc2(4, frame.at(c, 0x20));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x20));
                    let a0 = c.g[A0][0] as u32;
                    c.lqc2(3, a0);
                    c.lqc2(5, a0);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.lqc2(3, p(me, st::AXIS));
                    c.lqc2(5, a0);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.div_s(1, 1, 0);
                    c.mfc1(3, 1);
                    c.lqc2(4, a0);
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x60));
                    qcopy(c, frame.at(c, 0x60), frame.at(c, 0x50));
                    c.mfc1(3, 2);
                    c.lqc2(4, frame.at(c, 0x50));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x60));
                    qcopy(c, frame.at(c, 0x60), frame.at(c, 0x40));
                    c.lqc2(3, p(me, st::AXIS));
                    c.lqc2(4, frame.at(c, 0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x60));
                    c.lq(2, frame.at(c, 0x60));
                    c.sq(2, p(me, st::AXIS_SCALED));
                    c.sq(2, frame.at(c, 0x30));
                    c.lwc1(0, p(me, st::AXIS_SCALED + 8));
                    c.neg_s(0, 0);
                    c.swc1(0, p(me, st::AXIS_SCALED + 8));
                    c.lqc2(3, p(me, st::AXIS_SCALED));
                    c.lqc2(5, p(me, st::AXIS_SCALED));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 1);
                    c.c_le_s(1, 3);
                    let skip = !c.fcc();
                    c.lq(2, frame.at(c, 0x20));
                    if !skip {
                        c.cycles += 1;
                        c.sq(2, p(me, st::AXIS_SCALED));
                    }

                    // 0x164d98: second-order damping of the swing basis:
                    // blend the axis towards the scaled axis by the elapsed
                    // time, then clamp its length to [0, 200].
                    c.cycles += 80;
                    c.lwc1(0, gp(c, -0x6918));
                    c.mfc1(2, 0);
                    c.lqc2(4, p(me, st::AXIS_SCALED));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(me, st::AXIS_SCALED));
                    c.lwc1(1, gp(c, -0x6914));
                    c.mfc1(2, 1);
                    c.lqc2(4, frame.at(c, 0x20));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x40));
                    c.lwc1(2, gp(c, -0x6910));
                    qcopy(c, frame.at(c, 0x40), frame.at(c, 0x30));
                    c.lqc2(3, p(me, st::AXIS_SCALED));
                    c.lqc2(4, frame.at(c, 0x30));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, p(me, st::AXIS_SCALED));
                    v_length(c, p(me, st::AXIS_SCALED), 2, 0);
                    c.div_s(0, 21, 0);
                    c.mfc1(2, 0);
                    c.lqc2(4, p(me, st::AXIS_SCALED));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(me, st::AXIS_SCALED));
                    let a1 = c.g[A1][0] as u32;
                    c.lqc2(3, a1);
                    c.lqc2(5, a1);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.lqc2(3, p(me, st::AXIS_SCALED));
                    c.lqc2(5, a1);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.div_s(1, 1, 0);
                    c.mfc1(3, 1);
                    c.lqc2(4, a1);
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x40));
                    qcopy(c, frame.at(c, 0x40), frame.at(c, 0x30));
                    c.sq(2, p(me, st::BASIS_B));
                    c.lqc2(3, p(me, st::AXIS_SCALED));
                    c.lqc2(4, p(me, st::BASIS_B));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x40));
                    qcopy(c, frame.at(c, 0x40), frame.at(c, 0x30));
                    c.lqc2(3, frame.at(c, 0x30));
                    c.lqc2(5, frame.at(c, 0x30));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 2);
                    if c.fcc() {
                        c.cycles += 16;
                        c.swc1(3, frame.at(c, 0x38));
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        v_length(c, frame.at(c, 0x30), 2, 2);
                        c.c_lt_s(0, 2);
                        if c.fcc() {
                            c.cycles += 25;
                            c.g[AT][0] = 0x43480000;
                            c.mtc1(1, 0);
                            c.g[AT][0] = 0x42c80000;
                            c.mtc1(1, 1);
                            c.div_s(0, 0, 2);
                            c.mfc1(2, 0);
                            c.lqc2(4, frame.at(c, 0x30));
                            c.qmtc2(2, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x30));
                            c.lqc2(3, p(me, st::AXIS_SCALED));
                            c.lqc2(4, frame.at(c, 0x30));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, p(me, st::AXIS_SCALED));
                            c.mfc1(3, 1);
                            c.lqc2(4, p(c.g[A2][0] as u32, 0) - 0xea0);
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x50));
                            qcopy(c, frame.at(c, 0x50), frame.at(c, 0x40));
                            c.lqc2(3, p(me, st::AXIS_SCALED));
                            c.lqc2(4, frame.at(c, 0x40));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, p(me, st::AXIS_SCALED));
                        }
                    }

                    // 0x164ec8: the spline path — scale the axis to the
                    // swing's unit length and, if it is degenerate, fall back
                    // through asin() of the sine of the swing angle.
                    c.cycles += 50;
                    v_cross(
                        c,
                        p(me, st::AXIS),
                        p(me, st::AXIS_SCALED),
                        frame.at(c, 0x50),
                    );
                    c.lwc1(2, gp(c, -0x690c));
                    qcopy(c, frame.at(c, 0x50), p(me, st::SCALE));
                    c.sq(2, frame.at(c, 0x40));
                    v_length(c, p(me, st::SCALE), 3, 0);
                    c.mfc1(2, 0);
                    c.qmtc2(2, 3);
                    c.vu0().div(0, 3, 3, 0);
                    c.lqc2(4, p(me, st::SCALE));
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 4, Src::Q);
                    c.sqc2(5, p(me, st::SCALE));
                    c.lqc2(3, p(me, st::AXIS));
                    c.lqc2(5, p(me, st::AXIS));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.lqc2(3, p(me, st::AXIS_SCALED));
                    c.lqc2(5, p(me, st::AXIS_SCALED));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.mul_s(1, 1, 0);
                    c.sqrt_s(1, 1);
                    c.c_lt_s(1, 2);
                    if c.fcc() {
                        c.cycles += 3;
                        c.mtc1(0, 1);
                    } else {
                        // 0x164eb8: asin of the swing angle, stored as the
                        // swing gain.
                        c.cycles += 16;
                        c.lqc2(3, p(me, st::AXIS));
                        c.lqc2(5, p(me, st::AXIS_SCALED));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(2, 4);
                        c.mtc1(2, 0);
                        c.g[AT][0] = 0xffff_ffff_bf80_0000;
                        c.mtc1(1, 2);
                        c.div_s(12, 0, 1);
                        c.c_le_s(2, 12);
                        if !c.fcc() {
                            c.mov_s(12, 2);
                        } else {
                            c.cycles += 3;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.min_s(12, 12, 0);
                        }
                        c.cycles += 4;
                        call_ptr(c, 0x0031_c128, 0x0016_4ec0)?;
                        c.lwc1(1, gp(c, -0x6908));
                        c.sub_s(1, 1, 0);
                    }

                    // 0x164ee8: store the swing gain and push the spline
                    // through its eight keys.
                    c.cycles += 55;
                    c.mtc1(0, 20);
                    c.g[S0][0] = sx(p(me, st::SPLINE));
                    c.swc1(1, p(me, st::GAIN));
                    c.g[A1][0] = 0x9;
                    c.g[A0][0] = c.g[S0][0];
                    c.mov_s(12, 20);
                    c.mov_s(13, 20);
                    call_ptr(c, 0x0031_d738, 0x0016_4ee8)?;
                    c.mov_s(12, 20);
                    c.g[A0][0] = c.g[S0][0];
                    c.mov_s(13, 12);
                    call_ptr(c, 0x0031_d7e0, 0x0016_4ef8)?;
                    c.g[AT][0] = 0x3e000000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x6904));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f0c)?;
                    c.g[AT][0] = 0x3e800000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x6900));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f20)?;
                    c.g[AT][0] = 0x3ec00000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x68fc));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f34)?;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x68f8));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f48)?;
                    c.g[AT][0] = 0x3f200000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x68f4));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f5c)?;
                    c.g[AT][0] = 0x3f400000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x68f0));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f70)?;
                    c.g[AT][0] = 0x3f600000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x68ec));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f84)?;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    c.g[A0][0] = c.g[S0][0];
                    c.mov_s(13, 12);
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f98)?;
                    c.g[V0][0] = 0x1;
                    c.sw(p(me, st::SWINGING), c.g[V0][0]);
                    c.g[V0][0] = c.lw(p(me, st::SWINGING));
                }
            }

            // 100485: the swing path is done (V0 from the spline calls). The
            // `SWINGING != 0` path broke 'b_164fb0 and skips this block.
            if !swinging {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    // `break 'b_16531c`: past the tail's charge, the reload and
                    // the whole 0x1650a4 region.
                    c.g[V0][0] = c.lw(p(me, st::AIRBORNE));
                    skip_tail = true;
                    skip_496 = true;
                } else {
                    c.cycles += 1;
                    c.lwc1(1, p(me, st::AMPLITUDE));
                }
            }

            // 100496: ramp the amplitude up to the frequency; once it is
            // there, re-seed the spline and blend the frame.
            if !skip_496 {
                c.cycles += 7;
                c.lwc1(0, gp(c, -0x68e8));
                c.lwc1(2, p(me, st::FREQ));
                c.add_s(1, 1, 0);
                c.c_lt_s(1, 2);
                let ramping = !c.fcc();
                c.swc1(1, p(me, st::AMPLITUDE));
                if !ramping {
                    c.cycles += 54;
                    c.div_s(12, 1, 2);
                    c.g[V1][0] = sx(p(me, st::SPLINE));
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = c.g[V1][0];
                    c.sw(gp(c, -0x1e64), c.g[V0][0]);
                    c.sw(gp(c, 0x1cf0), c.g[V1][0]);
                    call_ptr(c, 0x0031_dee0, 0x0016_4fe8)?;
                    c.lwc1(12, p(me, st::GAIN));
                    c.mov_s(20, 0);
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 0);
                    c.g[A0][0] = sx(p(sp(c), 0x90));
                    c.mul_s(12, 20, 12);
                    c.g[A1][0] = sx(p(sp(c), 0x94));
                    c.mul_s(12, 12, 0);
                    call_ptr(c, 0x0031_be50, 0x0016_500c)?;
                    // Scale the swing offset by the per-axis model scale.
                    c.lwc1(1, p(me, st::SCALE));
                    c.lwc1(2, p(me, st::SCALE + 4));
                    c.lwc1(0, frame.at(c, 0x90));
                    c.lwc1(4, p(me, st::SCALE + 8));
                    c.mul_s(1, 0, 1);
                    c.lwc1(3, frame.at(c, 0x94));
                    c.mul_s(2, 0, 2);
                    c.mul_s(0, 0, 4);
                    c.swc1(3, frame.at(c, 0x3c));
                    c.swc1(1, frame.at(c, 0x30));
                    c.swc1(2, frame.at(c, 0x34));
                    c.swc1(0, frame.at(c, 0x38));
                    qcopy(c, frame.at(c, 0x30), frame.at(c, 0x20));
                    // Rotate the offset about the swing axis (a quaternion
                    // product with the axis and its square).
                    c.lqc2(4, frame.at(c, 0x20));
                    c.lqc2(5, p(me, st::AXIS));
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
                    c.sqc2(8, frame.at(c, 0x40));
                    qcopy(c, frame.at(c, 0x40), p(me, st::BASIS_A));
                    c.sq(2, frame.at(c, 0x30));
                    c.mfc1(3, 20);
                    c.lqc2(4, p(me, st::BASIS_B));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x40));
                    qcopy(c, frame.at(c, 0x40), p(me, st::BASIS_C));
                    c.sq(2, frame.at(c, 0x30));
                }

                // 100569: smooth-step the amplitude and blend the frame towards
                // the info position/direction.
                c.cycles += 21;
                c.lwc1(1, gp(c, -0x68e4));
                c.lwc1(0, p(me, st::FREQ));
                c.lwc1(2, p(me, st::AMPLITUDE));
                c.mul_s(0, 0, 1);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 4);
                c.mtc1(0, 1);
                c.g[AT][0] = 0x40400000;
                c.mtc1(1, 3);
                c.div_s(2, 2, 0);
                c.min_s(2, 2, 4);
                c.max_s(2, 2, 1);
                c.mul_s(0, 2, 2);
                c.add_s(1, 0, 0);
                c.mul_s(0, 0, 3);
                c.mul_s(1, 1, 2);
                c.sub_s(0, 0, 1);
                c.c_lt_s(4, 0);
                if c.fcc() {
                    c.mov_s(0, 4);
                }

                // 100596: frame = frame * (1 - t) + info * t, then rebuild the
                // travel direction and the deltas.
                c.cycles += 37;
                c.mov_s(2, 0);
                c.sub_s(0, 4, 2);
                c.mfc1(3, 0);
                c.lqc2(4, p(me, st::POS));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.sq(2, frame.at(c, 0x30));
                c.mfc1(3, 2);
                c.lqc2(4, p(info, 0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x50));
                c.lq(2, frame.at(c, 0x50));
                c.sq(2, frame.at(c, 0x40));
                c.lqc2(3, frame.at(c, 0x30));
                c.lqc2(4, frame.at(c, 0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x50));
                c.lq(2, frame.at(c, 0x50));
                c.sq(2, p(me, st::POS));
                c.sq(2, frame.at(c, 0x20));
                c.lqc2(3, p(me, st::POS));
                c.lqc2(4, p(me, st::BASIS_A));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.sq(2, frame.at(c, 0x30));
                c.lqc2(3, frame.at(c, 0x30));
                c.lqc2(4, p(me, st::BASIS_C));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.sq(2, p(me, st::VEL));
                c.sq(2, frame.at(c, 0x20));
            }
        } // if airborne

        // 100641: not airborne — run the swing out (or in) on the elapsed
        // time and blend the frame with the spline keys.
        if !airborne {
            c.cycles += 6;
            c.lwc1(3, p(me, st::TIME));
            c.mtc1(0, 1);
            c.c_lt_s(1, 3);
            if c.fcc() {
                c.cycles += 18;
                c.lwc1(0, gp(c, -0x68e0));
                c.g[AT][0] = 0x40400000;
                c.mtc1(1, 2);
                c.sub_s(0, 3, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 3);
                c.max_s(0, 0, 1);
                c.div_s(12, 0, 20);
                c.swc1(0, p(me, st::TIME));
                c.mul_s(1, 12, 12);
                c.add_s(0, 1, 1);
                c.mul_s(1, 1, 2);
                c.mul_s(0, 0, 12);
                c.sub_s(0, 1, 0);
                c.c_lt_s(3, 0);
                if c.fcc() {
                    c.mov_s(0, 3);
                }

                // 100673: blend towards the stored swing offset.
                c.cycles += 75;
                c.lq(2, p(me, st::POS));
                c.mov_s(12, 0);
                c.sub_s(0, 3, 12);
                c.sq(2, frame.at(c, 0x20));
                c.mfc1(3, 0);
                c.lqc2(4, p(me, st::POS));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x50));
                c.lq(2, frame.at(c, 0x50));
                c.sq(2, frame.at(c, 0x40));
                c.mfc1(3, 12);
                c.lqc2(4, p(info, 0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x60));
                c.lq(2, frame.at(c, 0x60));
                c.sq(2, frame.at(c, 0x50));
                c.lqc2(3, frame.at(c, 0x40));
                c.lqc2(4, frame.at(c, 0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x60));
                c.lq(2, frame.at(c, 0x60));
                c.sq(2, p(me, st::POS));
                c.sq(2, frame.at(c, 0x30));
                c.mfc1(3, 0);
                c.lqc2(4, p(me, st::VEL));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x50));
                c.lq(2, frame.at(c, 0x50));
                c.sq(2, frame.at(c, 0x40));
                c.lqc2(3, p(info, 0x10));
                c.lqc2(4, p(me, st::BASIS_A));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x80));
                c.lq(2, frame.at(c, 0x80));
                c.sq(2, frame.at(c, 0x70));
                c.lqc2(3, frame.at(c, 0x70));
                c.lqc2(4, p(me, st::BASIS_C));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x80));
                c.lq(2, frame.at(c, 0x80));
                c.sq(2, frame.at(c, 0x60));
                c.mfc1(3, 12);
                c.lqc2(4, frame.at(c, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x80));
                c.lq(2, frame.at(c, 0x80));
                c.sq(2, frame.at(c, 0x50));
                c.lqc2(3, frame.at(c, 0x40));
                c.lqc2(4, frame.at(c, 0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x80));
                c.lq(2, frame.at(c, 0x80));
                c.sq(2, p(me, st::VEL));
                c.sq(2, frame.at(c, 0x30));
                c.lqc2(3, p(me, st::POS));
                c.lqc2(4, p(me, st::VEL));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.sq(2, p(me, st::PREV_DELTA));
                c.sq(2, frame.at(c, 0x30));
                c.lqc2(3, p(me, st::POS));
                c.lqc2(4, frame.at(c, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.lwc1(0, p(me, st::TIME));
                c.sq(2, p(me, st::PREV_MOVE));
                c.sq(2, frame.at(c, 0x30));
                c.swc1(0, p(me, st::IN_TIME));
                c.swc1(0, p(me, st::OUT_TIME));
            } else {
                // `break 'b_16531c`: past the tail's charge and reload.
                c.g[V0][0] = c.lw(p(me, st::AIRBORNE));
                skip_tail = true;
            }
        }

        // 100752: the tail shared by every path (the `break 'b_16531c` paths
        // skip the charge and the V0 reload).
        if !skip_tail {
            c.cycles += 1;
            c.g[V0][0] = c.lw(p(me, st::AIRBORNE));
        }

        // 0x165448: while the swing is inside its in/out window, settle the
        // frame back onto the stored offsets.
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.lwc1(4, p(me, st::IN_TIME));
            c.mtc1(0, 3);
            c.c_lt_s(3, 4);
            if c.fcc() {
                c.cycles += 4;
                c.lwc1(1, p(me, st::OUT_TIME));
                c.c_lt_s(3, 1);
                if c.fcc() {
                    c.cycles += 18;
                    c.lwc1(0, gp(c, -0x68dc));
                    c.g[AT][0] = 0x40400000;
                    c.mtc1(1, 2);
                    c.sub_s(0, 4, 0);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 4);
                    c.max_s(0, 0, 3);
                    c.div_s(3, 0, 1);
                    c.swc1(0, p(me, st::IN_TIME));
                    c.mul_s(1, 3, 3);
                    c.add_s(0, 1, 1);
                    c.mul_s(1, 1, 2);
                    c.mul_s(0, 0, 3);
                    c.sub_s(0, 1, 0);
                    c.c_lt_s(4, 0);
                    if c.fcc() {
                        c.mov_s(0, 4);
                    }

                    // 100804: settle towards the stored deltas.
                    c.cycles += 46;
                    c.lqc2(3, p(me, st::POS));
                    c.lqc2(4, p(me, st::VEL));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x30));
                    c.mov_s(3, 0);
                    c.lq(2, frame.at(c, 0x30));
                    c.sub_s(0, 4, 3);
                    c.sq(2, frame.at(c, 0x20));
                    c.mfc1(3, 3);
                    c.lqc2(4, p(me, st::PREV_DELTA));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x50));
                    c.lq(2, frame.at(c, 0x50));
                    c.sq(2, frame.at(c, 0x40));
                    c.mfc1(3, 0);
                    c.lqc2(4, frame.at(c, 0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x60));
                    c.lq(2, frame.at(c, 0x60));
                    c.sq(2, frame.at(c, 0x50));
                    c.lqc2(3, frame.at(c, 0x40));
                    c.lqc2(4, frame.at(c, 0x50));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x60));
                    c.lq(2, frame.at(c, 0x60));
                    c.sq(2, frame.at(c, 0x30));
                    c.mfc1(3, 3);
                    c.lqc2(4, p(me, st::PREV_MOVE));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x50));
                    c.lq(2, frame.at(c, 0x50));
                    c.sq(2, frame.at(c, 0x40));
                    c.lqc2(3, p(me, st::POS));
                    c.lqc2(4, frame.at(c, 0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, p(me, st::POS));
                    c.lqc2(3, p(me, st::POS));
                    c.lqc2(4, frame.at(c, 0x30));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x50));
                    c.lq(2, frame.at(c, 0x50));
                    c.sq(2, p(me, st::VEL));
                    c.sq(2, frame.at(c, 0x40));
                }
            }
        }

        // 0x165518: damp the velocity towards `POS - POS0` so the camera
        // does not overshoot when the swing unwinds.
        c.cycles += 31;
        c.lqc2(3, p(me, st::POS));
        c.lqc2(4, p(me, st::VEL));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x30));
        c.lwc1(1, gp(c, -0x68d8));
        qcopy(c, frame.at(c, 0x30), frame.at(c, 0x20));
        v_length(c, frame.at(c, 0), 2, 0);
        v_length(c, frame.at(c, 0x20), 3, 2);
        c.mul_s(1, 0, 1);
        c.c_lt_s(2, 1);
        if c.fcc() {
            c.cycles += 6;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.c_lt_s(0, 2);
            let skip = !c.fcc();
            c.lq(2, frame.at(c, 0x10));
            if skip {
                c.cycles += 1;
                c.sq(2, p(me, st::VEL));
            } else {
                c.cycles += 14;
                c.div_s(0, 1, 2);
                c.mfc1(2, 0);
                c.lqc2(4, frame.at(c, 0x20));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x20));
                c.lqc2(3, p(me, st::POS));
                c.lqc2(4, frame.at(c, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.sq(2, p(me, st::VEL));
                c.sq(2, frame.at(c, 0x30));
            }
        }

        // 100929: let the camera place itself, then return.
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, CAMERA_UPDATE, 0x0016_5520)?;
        frame.restore(c, 16, 0xd0);
        frame.restore(c, 17, 0xc0);
        frame.restore(c, 18, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.restore_fpr(c, 21, 0xe8);
        frame.restore_fpr(c, 20, 0xe0);
        frame.close(c);
        Ok(())
    });
}

/// `SplineCameraAlgorithm_ResetForRider` (0x161630): if the rider id matches the
/// camera's current spline entry, reset the camera to it.
pub fn spline_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = lw_at_2(c, S0, 0x30);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x4)).lw(c);
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x6c0)).lw(c);
        c.g[A0][0] = low(c, A0).wrapping_add(0x6c0);
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x38)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3c)).lw(c);
        let get = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[A1][0]);
        call_ptr(c, get, 0x0016_1668)?;
        if c.g[S1][0] == c.g[V0][0] {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, addr::CHASE_CAMERA_RESET_FOR_RIDER, 0x0016_167c)?;
            c.g[V0][0] = lw_at_2(c, S0, 0x10);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x28)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x2c)).lw(c);
            let reset = r32(c, V1);
            c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, reset, 0x0016_1690)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The current stack pointer plus `off`.
#[inline(always)]
pub(crate) fn sp_at(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// A `gp_7`-relative global or `.lit4` constant.
#[inline(always)]
pub(crate) fn gp_7(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lui at, bits; mtc1 at, f`: a float constant through `at`.
#[inline(always)]
pub(crate) fn fconst(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.f[f] = bits;
}

/// `|v|` of the quadword at `src`, as the original's
/// `lqc2 vf3; vaddx.xyzw vf6, vf0, vf0w; vmul.xyzw vf4, vf3, vf3;
/// vaddax vf4; vmaddax vf6; vmaddx vf4; vsqrt Q, vf4x; vwaitq; cfc2 rt, 22`.
#[inline(always)]
pub(crate) fn v_length_q(c: &mut Cpu, src: u32, rt: usize) {
    c.lqc2(3, src);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(rt, 22);
}

/// `mfc1 gpr, f; lqc2 vf4, v; qmtc2 gpr, vf3; vmul.xyzw vf5, vf4, vf3x;
/// sqc2 vf5, out`: the quadword at `v` scaled by the float in `f`.
#[inline(always)]
pub(crate) fn vscale_q(c: &mut Cpu, gpr: usize, f: usize, v: u32, out: u32) {
    c.mfc1(gpr, f);
    c.lqc2(4, v);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}

// ---------------------------------------------------------------------------
// Callees (real game code, so both machines take the same path and the same
// cycle cost through them).

/// `cCSICubicSpline_Init(spline, count)` (0x31d700).
pub(crate) const SPLINE_INIT: u32 = 0x0031_d700;

/// `cCSICubicSpline_AddPoint(spline, value, tangent)` (0x31d7e0).
pub(crate) const SPLINE_ADD_POINT: u32 = 0x0031_d7e0;

/// `cCSICubicSpline_Evaluate(spline, t)` (0x31dee0).
pub(crate) const SPLINE_EVALUATE: u32 = 0x0031_dee0;

/// `a1 = count; a0 = <spline register>; jal Init`.
#[inline(always)]
pub(crate) fn spline_init(c: &mut Cpu, spline: usize, count: u64, ret: u32) -> Flow {
    c.g[A1][0] = count;
    c.g[A0][0] = c.g[spline][0];
    call_ptr(c, SPLINE_INIT, ret)
}

/// `a0 = <spline register>; jal AddPoint` (the value and tangent are already
/// in `f12`/`f13`).
#[inline(always)]
pub(crate) fn add_point(c: &mut Cpu, spline: usize, ret: u32) -> Flow {
    c.g[A0][0] = c.g[spline][0];
    call_ptr(c, SPLINE_ADD_POINT, ret)
}

/// `lqc2 vf3, a; lqc2 vf4, b; vadd.xyzw vf5, vf3, vf4; sqc2 vf5, out`.
#[inline(always)]
fn v_add_to(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `lqc2 vf3, a; lqc2 vf4, b; vsub.xyzw vf5, vf3, vf4; sqc2 vf5, out`.
#[inline(always)]
fn v_sub_to(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// World up (`0x4ff160`) and the side axis next to it (`0x4ff140`), the
/// statics the camera code addresses as `0x500000 - 0xea0 / - 0xec0`.
const AVOID_UP: u32 = 0x004f_f160;
const AVOID_SIDE: u32 = 0x004f_f140;
/// `CollSegment_ctor(seg, start, end, kind)` (0x32e100) and
/// `Nash_FindClosestContact(world, seg, out, ignore)` (0x336850).
const COLL_SEGMENT_CTOR: u32 = 0x0032_e100;
const NASH_FIND_CLOSEST_CONTACT: u32 = 0x0033_6850;
/// `GameCamera` fields of the collision avoidance.
const CAM_LOOK_AT: u32 = 0xe0;
const CAM_EYE: u32 = 0x100;
const CAM_CONTROLLER: u32 = 0xa8;
/// The collision lift along world up (f32), decayed per update.
const CAM_LIFT: u32 = 0x460;
/// The last contact's normal (vec4).
const CAM_CONTACT_NORMAL: u32 = 0x470;
const CAM_IGNORE: u32 = 0x480;
/// The script camera's last clear eye (vec4) and its two flags.
const CAM_SCRIPT_EYE: u32 = 0x490;
const CAM_SCRIPT_BLOCKED: u32 = 0x4a0;
const CAM_SCRIPT_RESET: u32 = 0x4a4;

/// `void GameCamera_AvoidCollision(GameCamera* cam)` (0x15ee00): keep the
/// eye out of the world.
///
/// The eye lifted by the collision lift (+0x460) is swept by a vertical
/// segment `±300 / s` along world up (s: the sine of the eye's elevation
/// seen from the look-at point, at least `gp-0x6c58`), pushed 50 to the
/// side of the view (the side opposite the last contact normal +0x470). On a
/// contact at fraction x (side d: the record's +0x18) the lift is corrected
/// with `e = 300/s · |2x − 1|`: by `max(0, 50/s − e)` when the half of the
/// segment and the side agree, else by `e + 50/s` (0 beyond 150); added
/// for d > 0, subtracted otherwise. The eye is then lifted, put back at
/// its distance from the look-at point, and the lift decays by
/// `gp-0x6c54` (0.97) per update.
///
/// With a script camera active (the controller at +0xa8), a segment from
/// the eye to 100 above the look-at point is tested too: on a hit the eye
/// goes back to the last clear one (+0x490) or, when +0x4a4 is set, to
/// 180 above and beside the look-at point; on the first blocked update it
/// moves 50 along the contact normal; when it ends up within 2 of the
/// look-at point or (nearly) straight above it, it is reset to 150 up and
/// 100 to the side. The eye is saved at +0x490 and +0x4a4 cleared.
///
/// Frame 0x400: the vector scratch at +0x00..+0x360 (the contact records
/// at +0x00 and +0x2a0, the segments at +0x110 and +0x1f0), `ra` at +0x370,
/// s0..s4 at +0x3c0, +0x3b0, +0x3a0, +0x390, +0x380, f20..f25 at +0x3d0..
pub fn game_camera_avoid_collision(c: &mut Cpu) {
    guest(c, |c| {
        // Block 0x15ee00: the sweep segment's geometry.
        c.cycles += 148;
        let frame = Frame::open(c, 0x400);
        c.g[A1][0] = c.lw(gp_7(c, -0x848));
        frame.save(c, S1, 0x3b0);
        c.g[V1][0] = 0x50_0000;
        frame.save(c, S2, 0x3a0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 25, 0x3f8);
        c.g[S2][0] = c.g[V1][0];
        frame.save_fpr(c, 23, 0x3e8);
        frame.save_fpr(c, 22, 0x3e0);
        frame.save_fpr(c, 21, 0x3d8);
        frame.save_fpr(c, 20, 0x3d0);
        frame.save(c, S0, 0x3c0);
        frame.save(c, S3, 0x390);
        frame.save(c, S4, 0x380);
        frame.save_ra(c, 0x370);
        frame.save_fpr(c, 24, 0x3f0);
        c.mtc1(0, 22); // f22 = 0
        let cam = r32(c, S1);
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x84));
        c.lwc1(0, cam + CAM_LIFT);
        // s3: the collision world.
        c.g[S3][0] = c.lw(r32(c, V0).wrapping_add(0x20));
        vscale_q(c, A0, 0, AVOID_UP, sp_at(c, 0xa0));
        c.lwc1(3, gp_7(c, -0x6c58)); // the least elevation sine
        qcopy(c, sp_at(c, 0xa0), sp_at(c, 0x90));
        fconst(c, 23, 0x3f80_0000); // f23 = 1
                                    // The lifted eye, and the look-at point seen from it (+0x90).
        v_add_to(c, cam + CAM_EYE, sp_at(c, 0x90), sp_at(c, 0xa0));
        fconst(c, 4, 0x4396_0000); // 300
        qcopy(c, sp_at(c, 0xa0), sp_at(c, 0x80));
        c.mov_s(6, 23);
        fconst(c, 5, 0x4248_0000); // 50
        v_sub_to(c, cam + CAM_LOOK_AT, sp_at(c, 0x80), sp_at(c, 0xa0));
        qcopy(c, sp_at(c, 0xa0), sp_at(c, 0x90));
        // Its length, and its horizontal length (z zeroed): the sine.
        v_length(c, sp_at(c, 0x90), A0, 0);
        c.swc1(22, sp_at(c, 0xa8));
        c.mul_s(0, 0, 0);
        v_length(c, sp_at(c, 0xa0), V0, 1);
        c.mul_s(2, 1, 1);
        v_sub_to(c, cam + CAM_LOOK_AT, cam + CAM_EYE, sp_at(c, 0xc0));
        c.lq(2, sp_at(c, 0xc0));
        c.add_s(2, 2, 0);
        c.sq(2, sp_at(c, 0xb0));
        c.sqrt_s(2, 2);
        // f25: the eye's distance from the look-at point.
        v_length(c, sp_at(c, 0xb0), V0, 25);
        c.div_s(1, 1, 2);
        c.lwc1(0, cam + CAM_LIFT);
        vscale_q(c, A0, 0, AVOID_UP, sp_at(c, 0xd0));
        qcopy(c, sp_at(c, 0xd0), sp_at(c, 0xc0));
        v_add_to(c, cam + CAM_EYE, sp_at(c, 0xc0), sp_at(c, 0xd0));
        c.max_s(1, 1, 3);
        qcopy(c, sp_at(c, 0xd0), sp_at(c, 0xb0));
        c.div_s(1, 23, 1);
        // f21 = 300 / s (half the segment), f20 = 50 / s.
        c.mul_s(21, 1, 4);
        vscale_q(c, A0, 21, AVOID_UP, sp_at(c, 0xd0));
        c.mul_s(20, 1, 5);
        qcopy(c, sp_at(c, 0xd0), sp_at(c, 0xc0));
        v_sub_to(c, sp_at(c, 0xb0), sp_at(c, 0xc0), sp_at(c, 0xe0));
        qcopy(c, sp_at(c, 0xe0), sp_at(c, 0xd0));
        v_add_to(c, sp_at(c, 0xb0), sp_at(c, 0xc0), sp_at(c, 0xf0));
        qcopy(c, sp_at(c, 0xf0), sp_at(c, 0xe0));
        // The side of the view, towards the last contact normal.
        v_cross(c, sp_at(c, 0x90), AVOID_UP, sp_at(c, 0x100));
        qcopy(c, sp_at(c, 0x100), sp_at(c, 0xf0));
        v_dot2(c, cam + CAM_CONTACT_NORMAL, sp_at(c, 0xf0));
        c.qmfc2(3, 4);
        c.mtc1(3, 0);
        c.c_lt_s(22, 0);
        if c.fcc() {
            c.cycles += 3;
            fconst(c, 6, 0xbf80_0000); // f6 = -1
        }
        c.mul_s(0, 6, 5);

        // Block 0x15f05c: build the segment and sweep it.
        c.cycles += 47;
        v_length(c, sp_at(c, 0xf0), V0, 1);
        fconst(c, 24, 0x3f00_0000); // 0.5
        c.g[S0][0] = sx(sp_at(c, 0x110));
        c.g[A2][0] = sx(sp_at(c, 0x1c0));
        c.g[A1][0] = sx(sp_at(c, 0x100));
        c.div_s(0, 0, 1);
        c.g[S4][0] = c.g[A2][0];
        vscale_q(c, V0, 0, sp_at(c, 0xf0), sp_at(c, 0xf0));
        c.g[A0][0] = c.g[S0][0];
        v_add_to(c, sp_at(c, 0xe0), sp_at(c, 0xf0), sp_at(c, 0x1c0));
        c.mov_s(12, 24);
        c.lq(2, sp_at(c, 0x1c0));
        c.g[A3][0] = 2;
        c.sq(2, sp_at(c, 0x100));
        v_add_to(c, sp_at(c, 0xd0), sp_at(c, 0xf0), sp_at(c, 0x1d0));
        qcopy(c, sp_at(c, 0x1d0), sp_at(c, 0x1c0));
        call_ptr(c, COLL_SEGMENT_CTOR, 0x0015_f0f4)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = sx(cam + CAM_IGNORE);
        call_ptr(c, NASH_FIND_CLOSEST_CONTACT, 0x0015_f108)?;
        // f0: the contact's fraction along the segment (-1: none).
        c.c_lt_s(22, 0);
        if !c.fcc() {
            c.lwc1(0, cam + CAM_LIFT);
        } else {
            avoid_collision_correct_lift(c, cam);
        }

        // Block 0x15f1e8: lift the eye, keep its distance, decay the lift.
        c.cycles += 48;
        vscale_q(c, V0, 0, AVOID_UP, sp_at(c, 0x1c0));
        c.lwc1(1, gp_7(c, -0x6c54)); // 0.97
                                     // The lift decays by this factor per update: at another rate the
                                     // same share per unit of wall-clock time (`k^(60/rate)`); unchanged
                                     // at 60.
        if crate::tick::rate(c) != 60 {
            let k = keep_per_tick(c, FpuF32(c.f[1]));
            c.f[1] = k.0;
        }
        qcopy(c, sp_at(c, 0x1c0), sp_at(c, 0x100));
        v_add_to(c, cam + CAM_EYE, sp_at(c, 0x100), cam + CAM_EYE);
        v_sub_to(c, cam + CAM_LOOK_AT, cam + CAM_EYE, sp_at(c, 0x1c0));
        qcopy(c, sp_at(c, 0x1c0), sp_at(c, 0x100));
        v_length(c, sp_at(c, 0x100), V1, 0);
        c.div_s(0, 25, 0);
        vscale_q(c, V0, 0, sp_at(c, 0x100), sp_at(c, 0x100));
        v_sub_to(c, cam + CAM_LOOK_AT, sp_at(c, 0x100), sp_at(c, 0x1d0));
        c.lwc1(0, cam + CAM_LIFT);
        c.lq(2, sp_at(c, 0x1d0));
        c.mul_s(0, 0, 1);
        c.g[A0][0] = c.lw(cam + CAM_CONTROLLER);
        c.sq(2, cam + CAM_EYE);
        c.sq(2, sp_at(c, 0x1c0));
        c.swc1(0, cam + CAM_LIFT);
        Callee {
            addr: 0x0015_d020,
            f: ee::s01::ChaseCamController_IsScriptCameraActive,
        }
        .call(c, 0x0015_f2a0)?;
        c.g[S0][0] = sx(sp_at(c, 0x1f0));
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.sw(cam + CAM_SCRIPT_BLOCKED, 0);
        } else {
            avoid_collision_script(c, cam)?;
        }

        // Block 0x15f52c.
        c.cycles += 15;
        c.sw(cam + CAM_SCRIPT_RESET, 0);
        frame.restore(c, S0, 0x3c0);
        frame.restore(c, S1, 0x3b0);
        frame.restore(c, S2, 0x3a0);
        frame.restore(c, S3, 0x390);
        frame.restore(c, S4, 0x380);
        frame.restore_ra(c, 0x370);
        frame.restore_fpr(c, 25, 0x3f8);
        frame.restore_fpr(c, 24, 0x3f0);
        frame.restore_fpr(c, 23, 0x3e8);
        frame.restore_fpr(c, 22, 0x3e0);
        frame.restore_fpr(c, 21, 0x3d8);
        frame.restore_fpr(c, 20, 0x3d0);
        frame.close(c);
        Ok(())
    })
}

/// [`game_camera_avoid_collision`]'s lift correction on a contact (blocks
/// 0x15f114..0x15f1d8): f0 the fraction x, f1 = the contact record's +0x18
/// (which side), f21 = 300/s, f20 = 50/s, f22 = 0, f23 = 1, f24 = 0.5.
/// Stores the lift and the contact normal; leaves the lift in f0.
fn avoid_collision_correct_lift(c: &mut Cpu, cam: u32) {
    c.cycles += 4;
    c.c_lt_s(24, 0);
    let far_half = c.fcc(); // 0.5 < x
    c.lwc1(1, sp_at(c, 0x18));
    c.cycles += 5;
    c.add_s(0, 0, 0);
    c.c_lt_s(22, 1);
    if far_half {
        c.sub_s(0, 0, 23); // 2x - 1
    } else {
        c.sub_s(0, 23, 0); // 1 - 2x
    }
    let positive = c.fcc(); // 0 < side
    c.mul_s(0, 21, 0); // e
                       // The amount: max(0, 50/s - e) when the half and the side agree (far
                       // half and positive side, or near half and the other), else e + 50/s
                       // (0 beyond 150). Added on the positive side, subtracted on the other.
    let raise_rest = far_half == positive;
    let raise = positive;
    if raise_rest {
        c.cycles += 4;
        c.c_lt_s(0, 20);
        let short = c.fcc();
        c.mov_s(1, 22);
        if short {
            c.cycles += 1;
            c.sub_s(1, 20, 0);
        }
    } else {
        c.cycles += 7;
        c.add_s(1, 0, 20);
        fconst(c, 0, 0x4316_0000); // 150
        c.c_lt_s(0, 1);
        let beyond = c.fcc();
        if beyond {
            c.mov_s(1, 22);
        }
        if !raise && !beyond {
            // (the far-half drop loads the lift in its own block)
            c.cycles += 1;
        }
    }
    if raise {
        c.cycles += 3;
        c.lwc1(0, cam + CAM_LIFT);
        c.add_s(0, 0, 1);
    } else {
        c.cycles += 1;
        c.lwc1(0, cam + CAM_LIFT);
        c.cycles += 1;
        c.sub_s(0, 0, 1);
    }
    c.cycles += 4;
    c.swc1(0, cam + CAM_LIFT);
    c.lq(2, sp_at(c, 0x10));
    c.sq(2, cam + CAM_CONTACT_NORMAL);
    c.lwc1(0, cam + CAM_LIFT);
}

/// [`game_camera_avoid_collision`] with a script camera active (blocks
/// 0x15f2b0..0x15f528): keep the eye clear of the line to the look-at point.
fn avoid_collision_script(c: &mut Cpu, cam: u32) -> Flow {
    c.cycles += 36;
    fconst(c, 0, 0x42c8_0000); // 100
    vscale_q(c, V0, 0, AVOID_UP, sp_at(c, 0x1e0));
    fconst(c, 12, 0x3f00_0000);
    c.lq(2, sp_at(c, 0x1e0));
    c.g[A2][0] = sx(sp_at(c, 0x1d0));
    c.g[A1][0] = c.g[S4][0];
    c.g[A0][0] = c.g[S0][0];
    c.sq(2, sp_at(c, 0x1d0));
    c.g[A3][0] = 2;
    v_add_to(c, cam + CAM_LOOK_AT, sp_at(c, 0x1d0), sp_at(c, 0x1e0));
    c.lq(V1, cam + CAM_EYE);
    c.lq(2, sp_at(c, 0x1e0));
    c.sq(V1, sp_at(c, 0x1d0));
    c.sq(2, sp_at(c, 0x1c0));
    call_ptr(c, COLL_SEGMENT_CTOR, 0x0015_f308)?;
    c.g[V1][0] = c.lw(gp_7(c, -0x848));
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = sx(sp_at(c, 0x2a0));
    c.g[A3][0] = 0;
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x84));
    c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x20));
    call_ptr(c, NASH_FIND_CLOSEST_CONTACT, 0x0015_f324)?;
    c.mtc1(0, 1);
    c.c_lt_s(1, 0);
    // Whether the closing `lq v0, eye` is still to come (+1 cycle).
    let mut load_eye = true;
    if !c.fcc() {
        c.sw(cam + CAM_SCRIPT_BLOCKED, 0);
    } else {
        // Blocked: back to the last clear eye, or above and beside the
        // look-at point.
        c.cycles += 3;
        c.g[V0][0] = c.lw(cam + CAM_SCRIPT_RESET);
        if c.g[V0][0] == 0 {
            c.lq(2, cam + CAM_SCRIPT_EYE);
            c.cycles += 1;
            c.sq(2, cam + CAM_EYE);
        } else {
            c.cycles += 32;
            fconst(c, 0, 0x4334_0000); // 180
            vscale_q(c, V1, 0, AVOID_UP, sp_at(c, 0x340));
            qcopy(c, sp_at(c, 0x340), sp_at(c, 0x330));
            v_add_to(c, cam + CAM_LOOK_AT, sp_at(c, 0x330), sp_at(c, 0x340));
            qcopy(c, sp_at(c, 0x340), sp_at(c, 0x320));
            c.g[V0][0] = sx(AVOID_SIDE);
            vscale_q(c, V1, 0, AVOID_SIDE, sp_at(c, 0x350));
            qcopy(c, sp_at(c, 0x350), sp_at(c, 0x340));
            v_add_to(c, sp_at(c, 0x320), sp_at(c, 0x340), sp_at(c, 0x350));
            c.lq(2, sp_at(c, 0x350));
            c.sq(2, cam + CAM_EYE);
            c.sq(2, sp_at(c, 0x1e0));
        }
        // The first blocked update moves 50 along the contact normal.
        c.cycles += 3;
        let was_blocked = c.lw(cam + CAM_SCRIPT_BLOCKED);
        c.g[V0][0] = 1;
        if was_blocked == 0 {
            c.cycles += 14;
            fconst(c, 0, 0x4248_0000); // 50
            vscale_q(c, V1, 0, sp_at(c, 0x2b0), sp_at(c, 0x320));
            qcopy(c, sp_at(c, 0x320), sp_at(c, 0x1e0));
            v_add_to(c, cam + CAM_EYE, sp_at(c, 0x1e0), cam + CAM_EYE);
            c.g[V0][0] = 1;
        }
        c.cycles += 23;
        fconst(c, 1, 0x4000_0000); // 2
        c.sw(cam + CAM_SCRIPT_BLOCKED, c.g[V0][0]);
        v_sub_to(c, cam + CAM_LOOK_AT, cam + CAM_EYE, sp_at(c, 0x320));
        qcopy(c, sp_at(c, 0x320), sp_at(c, 0x1e0));
        v_length(c, sp_at(c, 0x1e0), V1, 0);
        c.c_lt_s(0, 1);
        let too_close = c.fcc();
        c.lwc1(0, sp_at(c, 0x1e0));
        let reset = too_close || {
            c.cycles += 7;
            fconst(c, 1, 0x3f80_0000);
            c.abs_s(0, 0);
            c.c_lt_s(0, 1);
            let x_small = c.fcc();
            c.lwc1(0, sp_at(c, 0x1e4));
            x_small && {
                c.cycles += 5;
                c.abs_s(0, 0);
                c.c_lt_s(0, 1);
                if !c.fcc() {
                    c.lq(2, cam + CAM_EYE);
                    load_eye = false;
                }
                c.fcc()
            }
        };
        if reset {
            // 150 above and 100 beside the look-at point.
            c.cycles += 33;
            fconst(c, 0, 0x4316_0000); // 150
            vscale_q(c, V0, 0, AVOID_UP, sp_at(c, 0x350));
            fconst(c, 1, 0x42c8_0000); // 100
            qcopy(c, sp_at(c, 0x350), sp_at(c, 0x340));
            v_add_to(c, cam + CAM_LOOK_AT, sp_at(c, 0x340), sp_at(c, 0x350));
            qcopy(c, sp_at(c, 0x350), sp_at(c, 0x330));
            c.g[V0][0] = sx(AVOID_SIDE);
            vscale_q(c, V1, 1, AVOID_SIDE, sp_at(c, 0x360));
            qcopy(c, sp_at(c, 0x360), sp_at(c, 0x350));
            v_add_to(c, sp_at(c, 0x330), sp_at(c, 0x350), sp_at(c, 0x360));
            c.lq(2, sp_at(c, 0x360));
            c.sq(2, cam + CAM_EYE);
            c.sq(2, sp_at(c, 0x320));
        }
    }
    if load_eye {
        c.cycles += 1;
        c.lq(2, cam + CAM_EYE);
    }
    c.cycles += 2;
    c.sq(2, cam + CAM_SCRIPT_EYE);
    Ok(())
}

/// `void ChaseCameraAlgorithm_ApplyJumpCurves(algo, info, dt, f13, f14, f15)`
/// (0x1635f8): one frame of the chase camera's jump/landing curve blend.
///
/// Frame 0xf0: `ra` at +0x40, the saved callee-saved registers at +0x50..+0x90
/// and +0x80, the saved `f20..f31` at +0x90..+0xe8, vector scratch at
/// +0x00/+0x10/+0x20 and the incoming `f13` at +0x30.
pub fn chase_camera_algorithm_apply_jump_curves(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0xf0);
        c.sq(18, sp_at(c, 0x60));
        c.sq(19, sp_at(c, 0x50));
        c.g[S2][0] = c.g[A0][0];
        c.swc1(31, sp_at(c, 0xe8));
        c.g[S3][0] = c.g[A1][0];
        c.swc1(28, sp_at(c, 0xd0));
        c.mov_s(31, 15);
        c.swc1(20, sp_at(c, 0x90));
        c.mov_s(28, 14);
        c.sq(16, sp_at(c, 0x80));
        c.mov_s(20, 12);
        c.sq(17, sp_at(c, 0x70));
        frame.save_ra(c, 0x40);
        c.swc1(30, sp_at(c, 0xe0));
        c.swc1(29, sp_at(c, 0xd8));
        c.swc1(27, sp_at(c, 0xc8));
        c.swc1(26, sp_at(c, 0xc0));
        c.swc1(25, sp_at(c, 0xb8));
        c.swc1(24, sp_at(c, 0xb0));
        c.swc1(23, sp_at(c, 0xa8));
        c.swc1(22, sp_at(c, 0xa0));
        c.swc1(21, sp_at(c, 0x98));
        let s2 = r32(c, S2);
        let s3 = r32(c, S3);
        // `f30` = |pos - vel|, with the difference left at sp+0x10.
        c.lqc2(3, p(s2, st::POS));
        c.lqc2(4, p(s2, st::VEL));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_at(c, 0x10));
        c.swc1(13, sp_at(c, 0x30));
        c.lq(2, sp_at(c, 0x10));
        c.g[V1][0] = c.lw(p(s3, 0x60));
        c.sq(2, sp_at(c, 0));
        let scratch = sp_at(c, 0);
        v_length_q(c, scratch, 2);
        c.mtc1(2, 30);

        'out: {
            if c.g[V1][0] == 0 {
                c.g[V0][0] = c.lw(p(s2, st::JUMP_READY));
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(s2, st::AIRBORNE));
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw(p(s2, st::JUMP_READY));
                break 'out;
            }
            // The takeoff countdown: while it still counts down, ask the rig
            // whether the rider already landed (the countdown then ends).
            // It counts 60 Hz frames (15 at takeoff, see
            // `ChaseCameraAlgorithm_UpdateAirTransitions`): the takeoff curves
            // are built at 14, the air arc at 0 (0.25 s of air), and an early
            // landing measures the air time as `(16 - n) / 60` s. At another
            // rate it is taken down only when the algorithm's own 60 Hz clock
            // has a frame due (`tick::frame60_due`), and the reactions to 14
            // and 0 run once, on the tick the value was reached. At 60 every
            // tick takes it down.
            let timer = p(s2, st::TIMER);
            let mut timer_moved: bool;
            'timer: {
                c.cycles += 5;
                c.g[V0][0] = c.lw(timer);
                c.mtc1(0, 29);
                let at_60 = crate::tick::rate(c) == 60;
                timer_moved = at_60 || crate::tick::frame60_due(c, timer, r32(c, V0));
                if timer_moved {
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                    crate::tick::frame60_ran(c, timer, r32(c, V0));
                }
                let expired = (c.g[V0][0] as i64) <= 0;
                c.sw(timer, c.g[V0][0]);
                if expired {
                    break 'timer;
                }
                c.cycles += 8;
                c.g[A1][0] = c.lw(p(s2, st::RIG));
                c.g[V1][0] = c.lw(r32(c, A1));
                c.g[A0][0] = c.lh(p(r32(c, V1), 0x98));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x9c));
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0016_36dc)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0x10;
                    break 'timer;
                }
                c.cycles += 7;
                c.g[V1][0] = c.lw(p(s2, st::TIMER));
                c.lwc1(1, gp_7(c, -0x6aa0));
                if crate::tick::rate(c) != 60 {
                    // The countdown is in 60 Hz frames: a frame is 1/60 s
                    // whatever the step this `.lit4` entry holds.
                    c.f[1] = crate::tick::STEP_60;
                }
                c.g[V0][0] = sx(0x10u32.wrapping_sub(r32(c, V1)));
                c.sw(p(s2, st::TIMER), 0);
                timer_moved = true;
                crate::tick::frame60_ran(c, timer, 0);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(29, 0, 1);
            }
            // The jump splines are rebuilt when the timer reaches 14.
            c.cycles += 4;
            c.g[V1][0] = c.lw(p(s2, st::TIMER));
            c.g[V0][0] = 0xe;
            if c.g[V1][0] != c.g[V0][0] || !timer_moved {
                c.g[V0][0] = c.lw(p(s2, st::TIMER));
            } else {
                // `f1` = the rig's jump progress in 0..1.
                'curve: {
                    'ease: {
                        c.cycles += 15;
                        c.lwc1(0, gp_7(c, -0x6a9c));
                        c.g[A1][0] = c.lw(p(s2, st::RIG));
                        c.swc1(0, p(s2, st::CURVE_T));
                        c.g[V0][0] = c.lw(r32(c, A1));
                        c.g[A0][0] = c.lh(p(r32(c, V0), 0xa8));
                        c.g[V1][0] = c.lw(p(r32(c, V0), 0xac));
                        let target = r32(c, V1);
                        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                        call_ptr(c, target, 0x0016_3730)?;
                        fconst(c, 2, 0x4396_0000); // 300
                        c.mtc1(1, 2);
                        c.mov_s(1, 0);
                        c.c_le_s(2, 1);
                        if !c.fcc() {
                            c.mul_s(0, 28, 20);
                            break 'ease;
                        }
                        'ramp: {
                            c.cycles += 5;
                            fconst(c, 0, 0x43fa_0000); // 500
                            c.mtc1(1, 0);
                            c.c_le_s(1, 0);
                            if !c.fcc() {
                                break 'ramp;
                            }
                            c.cycles += 6;
                            c.sub_s(1, 1, 2);
                            fconst(c, 0, 0x4348_0000); // 200
                            c.mtc1(1, 0);
                            c.div_s(1, 1, 0);
                            c.mul_s(0, 28, 20);
                            break 'curve;
                        }
                        c.cycles += 4;
                        fconst(c, 1, 0x3f80_0000);
                        c.mul_s(0, 28, 20);
                        break 'curve;
                    }
                    c.cycles += 1;
                    c.mtc1(0, 1);
                }
                // `f26` = f1 * f13, the curve's time scale (its sign decides
                // whether the splines are rebuilt).
                'jump: {
                    c.cycles += 6;
                    c.mtc1(0, 27);
                    c.mul_s(26, 0, 1);
                    c.c_lt_s(27, 26);
                    let skip = !c.fcc();
                    c.swc1(1, p(s2, st::TIME_2));
                    if skip {
                        break 'jump;
                    }
                    c.cycles += 152;
                    c.g[S0][0] = sx(p(s2, st::SPLINE_A));
                    c.lwc1(22, gp_7(c, -0x6a98));
                    spline_init(c, S0, 0xd, 0x0016_37b8)?;
                    c.lwc1(23, gp_7(c, -0x6a94));
                    c.mov_s(13, 27);
                    fconst(c, 24, 0x4040_0000); // 3
                    c.mov_s(12, 27);
                    add_point(c, S0, 0x0016_37d4)?;
                    c.lwc1(25, gp_7(c, -0x6a90));
                    c.lwc1(13, gp_7(c, -0x6a8c));
                    c.mov_s(12, 22);
                    fconst(c, 20, 0x4080_0000); // 4
                    c.mul_s(13, 26, 13);
                    fconst(c, 21, 0x3f80_0000); // 1
                    c.g[S1][0] = sx(p(s2, st::SPLINE_B));
                    add_point(c, S0, 0x0016_3800)?;
                    c.lwc1(13, gp_7(c, -0x6a88));
                    c.mov_s(12, 23);
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_3814)?;
                    c.lwc1(13, gp_7(c, -0x6a84));
                    c.lwc1(12, gp_7(c, -0x6a80));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_3828)?;
                    c.lwc1(13, gp_7(c, -0x6a7c));
                    c.lwc1(12, gp_7(c, -0x6a78));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_383c)?;
                    c.lwc1(13, gp_7(c, -0x6a74));
                    c.lwc1(12, gp_7(c, -0x6a70));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_3850)?;
                    c.lwc1(13, gp_7(c, -0x6a6c));
                    fconst(c, 12, 0x4000_0000); // 2
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_3868)?;
                    c.lwc1(13, gp_7(c, -0x6a68));
                    c.lwc1(12, gp_7(c, -0x6a64));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_387c)?;
                    c.lwc1(13, gp_7(c, -0x6a60));
                    c.lwc1(12, gp_7(c, -0x6a5c));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_3890)?;
                    c.lwc1(13, gp_7(c, -0x6a58));
                    c.mov_s(12, 24);
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_38a4)?;
                    c.lwc1(13, gp_7(c, -0x6a54));
                    c.mov_s(12, 25);
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_38b8)?;
                    c.lwc1(13, gp_7(c, -0x6a50));
                    c.lwc1(12, gp_7(c, -0x6a4c));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_38cc)?;
                    c.lwc1(13, gp_7(c, -0x6a48));
                    c.mov_s(12, 20);
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_38e0)?;
                    // The second (downward) jump spline.
                    spline_init(c, S1, 0xd, 0x0016_38ec)?;
                    c.mov_s(12, 27);
                    c.mov_s(13, 12);
                    add_point(c, S1, 0x0016_38fc)?;
                    c.lwc1(13, gp_7(c, -0x6a44));
                    c.mov_s(12, 22);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3910)?;
                    c.lwc1(13, gp_7(c, -0x6a40));
                    c.mov_s(12, 23);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3924)?;
                    c.lwc1(13, gp_7(c, -0x6a3c));
                    c.mov_s(12, 21);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3938)?;
                    c.lwc1(13, gp_7(c, -0x6a38));
                    c.lwc1(12, gp_7(c, -0x6a34));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_394c)?;
                    c.lwc1(13, gp_7(c, -0x6a30));
                    c.lwc1(12, gp_7(c, -0x6a2c));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3960)?;
                    c.lwc1(13, gp_7(c, -0x6a28));
                    c.lwc1(12, gp_7(c, -0x6a24));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3974)?;
                    c.lwc1(13, gp_7(c, -0x6a20));
                    c.lwc1(12, gp_7(c, -0x6a1c));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3988)?;
                    c.lwc1(13, gp_7(c, -0x6a18));
                    c.lwc1(12, gp_7(c, -0x6a14));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_399c)?;
                    c.lwc1(13, gp_7(c, -0x6a10));
                    c.mov_s(12, 24);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_39b0)?;
                    c.lwc1(13, gp_7(c, -0x6a0c));
                    c.mov_s(12, 25);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_39c4)?;
                    c.lwc1(13, gp_7(c, -0x6a08));
                    c.lwc1(12, gp_7(c, -0x6a04));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_39d8)?;
                    c.lwc1(13, gp_7(c, -0x6a00));
                    c.mov_s(12, 20);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_39ec)?;
                    c.g[V0][0] = 0x1;
                    c.swc1(20, p(s2, st::AMPLITUDE_2));
                    c.sw(p(s2, st::JUMP_READY), c.g[V0][0]);
                    c.swc1(21, p(s2, st::JUMP_SCALE));
                    c.swc1(21, p(s2, st::LAND_BLEND));
                    c.sw(p(s2, st::LANDING), 0);
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(p(s2, st::TIMER));
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 || !timer_moved {
                c.g[V0][0] = c.lw(p(s2, st::JUMP_READY));
                break 'out;
            }
            // The timer expired: blend the jump and landing curves.
            {
                // `f1` = the landing blend (blocks 0x163a10..0x163ab4).
                'blended: {
                    'blend: {
                        c.cycles += 15;
                        c.swc1(29, p(s2, st::JUMP_BLEND));
                        c.g[A1][0] = c.lw(p(s2, st::RIG));
                        c.g[V0][0] = c.lw(r32(c, A1));
                        c.g[A0][0] = c.lh(p(r32(c, V0), 0x60));
                        c.g[V1][0] = c.lw(p(r32(c, V0), 0x64));
                        let target = r32(c, V1);
                        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                        call_ptr(c, target, 0x0016_3a2c)?;
                        c.lwc1(1, gp_7(c, -0x69fc));
                        c.mov_s(27, 0);
                        fconst(c, 0, 0x3e80_0000); // 0.25
                        c.c_le_s(1, 27);
                        let skip = !c.fcc();
                        c.mul_s(26, 27, 0);
                        if skip {
                            break 'blend;
                        }
                        {
                            c.cycles += 4;
                            c.lwc1(0, gp_7(c, -0x69f8));
                            c.c_le_s(27, 0);
                            if c.fcc() {
                                c.cycles += 16;
                                c.sub_s(0, 27, 1);
                                c.lwc1(2, gp_7(c, -0x69f4));
                                fconst(c, 3, 0x4040_0000); // 3
                                fconst(c, 4, 0x3f80_0000); // 1
                                c.div_s(0, 0, 2);
                                c.add_s(1, 0, 0);
                                c.mul_s(0, 0, 0);
                                c.mul_s(1, 1, 0);
                                c.mul_s(0, 0, 3);
                                c.sub_s(0, 0, 1);
                                c.c_lt_s(4, 0);
                                if !c.fcc() {
                                    c.mul_s(1, 0, 28);
                                    break 'blended;
                                }
                            }
                        }
                        c.cycles += 4;
                        fconst(c, 0, 0x3f80_0000);
                        c.mul_s(1, 0, 28);
                        // The original's `break 'b_163ab4`: `f1` keeps this value
                        // and the `f1 = 0` pair below is skipped.
                        break 'blended;
                    }
                    c.cycles += 2;
                    c.mtc1(0, 0);
                    c.mul_s(1, 0, 28);
                }
                // The landing blend and the two landing splines (0x163ab4).
                c.cycles += 7;
                c.lwc1(0, p(s2, st::TIME_2));
                c.mtc1(0, 28);
                c.mul_s(25, 1, 0);
                c.c_lt_s(28, 25);
                let skip = !c.fcc();
                c.g[S0][0] = sx(p(s2, st::LAND_SPLINE_A));
                if !skip {
                    c.cycles += 167;
                    c.g[S1][0] = sx(p(s2, st::LAND_SPLINE_B));
                    c.g[A1][0] = 0xd;
                    spline_init(c, S0, 0xd, 0x0016_3ae0)?;
                    c.mov_s(13, 28);
                    c.mov_s(12, 28);
                    add_point(c, S0, 0x0016_3af0)?;
                    c.lwc1(24, gp_7(c, -0x69f0));
                    c.lwc1(13, gp_7(c, -0x69ec));
                    c.mul_s(24, 26, 24);
                    c.mul_s(13, 25, 13);
                    c.mov_s(12, 24);
                    add_point(c, S0, 0x0016_3b0c)?;
                    c.lwc1(23, gp_7(c, -0x69e8));
                    c.lwc1(13, gp_7(c, -0x69e4));
                    c.mul_s(23, 26, 23);
                    c.mul_s(13, 25, 13);
                    c.mov_s(12, 23);
                    add_point(c, S0, 0x0016_3b28)?;
                    c.lwc1(12, gp_7(c, -0x69e0));
                    c.lwc1(13, gp_7(c, -0x69dc));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3b40)?;
                    c.lwc1(12, gp_7(c, -0x69d8));
                    c.lwc1(13, gp_7(c, -0x69d4));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3b58)?;
                    c.lwc1(12, gp_7(c, -0x69d0));
                    c.lwc1(13, gp_7(c, -0x69cc));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3b70)?;
                    c.lwc1(13, gp_7(c, -0x69c8));
                    c.add_s(12, 26, 26);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3b84)?;
                    c.lwc1(12, gp_7(c, -0x69c4));
                    c.lwc1(13, gp_7(c, -0x69c0));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3b9c)?;
                    c.lwc1(12, gp_7(c, -0x69bc));
                    c.lwc1(13, gp_7(c, -0x69b8));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3bb4)?;
                    fconst(c, 22, 0x4040_0000);
                    c.lwc1(13, gp_7(c, -0x69b4));
                    c.mul_s(22, 26, 22);
                    c.mul_s(13, 25, 13);
                    c.mov_s(12, 22);
                    add_point(c, S0, 0x0016_3bd4)?;
                    c.lwc1(21, gp_7(c, -0x69b0));
                    c.lwc1(13, gp_7(c, -0x69ac));
                    c.mul_s(21, 26, 21);
                    c.mul_s(13, 25, 13);
                    c.mov_s(12, 21);
                    add_point(c, S0, 0x0016_3bf0)?;
                    c.lwc1(12, gp_7(c, -0x69a8));
                    c.lwc1(13, gp_7(c, -0x69a4));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3c08)?;
                    fconst(c, 20, 0x4080_0000);
                    c.lwc1(13, gp_7(c, -0x69a0));
                    c.mul_s(20, 26, 20);
                    c.mul_s(13, 25, 13);
                    c.mov_s(12, 20);
                    add_point(c, S0, 0x0016_3c28)?;
                    // The second landing spline.
                    spline_init(c, S1, 0xd, 0x0016_3c34)?;
                    c.mov_s(12, 28);
                    c.mov_s(13, 12);
                    add_point(c, S1, 0x0016_3c44)?;
                    c.lwc1(13, gp_7(c, -0x699c));
                    c.mov_s(12, 24);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3c58)?;
                    c.lwc1(13, gp_7(c, -0x6998));
                    c.mov_s(12, 23);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3c6c)?;
                    c.lwc1(13, gp_7(c, -0x6994));
                    c.mov_s(12, 26);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3c80)?;
                    c.lwc1(12, gp_7(c, -0x6990));
                    c.lwc1(13, gp_7(c, -0x698c));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3c98)?;
                    c.lwc1(12, gp_7(c, -0x6988));
                    c.lwc1(13, gp_7(c, -0x6984));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3cb0)?;
                    c.lwc1(12, gp_7(c, -0x6980));
                    c.lwc1(13, gp_7(c, -0x697c));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3cc8)?;
                    c.lwc1(12, gp_7(c, -0x6978));
                    c.lwc1(13, gp_7(c, -0x6974));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3ce0)?;
                    c.lwc1(12, gp_7(c, -0x6970));
                    c.lwc1(13, gp_7(c, -0x696c));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3cf8)?;
                    c.lwc1(13, gp_7(c, -0x6968));
                    c.mov_s(12, 22);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3d0c)?;
                    c.lwc1(13, gp_7(c, -0x6964));
                    c.mov_s(12, 21);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3d20)?;
                    c.lwc1(12, gp_7(c, -0x6960));
                    c.lwc1(13, gp_7(c, -0x695c));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3d38)?;
                    c.lwc1(13, gp_7(c, -0x6958));
                    c.mov_s(12, 20);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3d4c)?;
                    fconst(c, 0, 0x3f80_0000);
                    c.g[V0][0] = 0x1;
                    c.sw(p(s2, st::APPLYING), c.g[V0][0]);
                    c.swc1(0, p(s2, st::JUMP_SCALE));
                    c.swc1(27, p(s2, st::AMPLITUDE_2));
                    c.swc1(0, p(s2, st::LAND_BLEND));
                    c.sw(p(s2, st::LANDING), 0);
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(p(s2, st::JUMP_READY));
                break 'out;
            }
        }

        // Evaluate the jump curves: the up curve, then the down curve scaled
        // by the landing blend.
        'eval: {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lw(p(s2, st::APPLYING));
                break 'eval;
            }
            'curve_t: {
                c.cycles += 7;
                c.lwc1(2, p(s2, st::CURVE_T));
                c.lwc1(0, gp_7(c, -0x6954));
                c.c_le_s(0, 2);
                fconst(c, 0, 0x4080_0000); // 4
                if c.fcc() {
                    c.swc1(0, p(s2, st::CURVE_T));
                    break 'curve_t;
                }
                c.cycles += 6;
                c.lwc1(0, gp_7(c, -0x6950));
                fconst(c, 1, 0x4080_0000);
                c.add_s(0, 2, 0);
                c.min_s(0, 0, 1);
                c.swc1(0, p(s2, st::CURVE_T));
            }
            c.cycles += 17;
            c.lwc1(0, p(s2, st::JUMP_SCALE));
            c.g[A0][0] = sx(p(s2, st::SPLINE_B));
            c.lwc1(12, p(s2, st::CURVE_T));
            c.mul_s(12, 0, 12);
            call_ptr(c, SPLINE_EVALUATE, 0x0016_3dc0)?;
            c.lwc1(1, p(s2, st::JUMP_SCALE));
            c.g[A0][0] = sx(p(s2, st::SPLINE_A));
            c.lwc1(12, p(s2, st::CURVE_T));
            c.lwc1(20, p(s2, st::LAND_BLEND));
            c.mul_s(12, 1, 12);
            c.mul_s(20, 20, 0);
            call_ptr(c, SPLINE_EVALUATE, 0x0016_3ddc)?;
            c.lwc1(1, p(s2, st::LAND_BLEND));
            c.swc1(20, p(s2, st::LAND_VEL_Y));
            c.mul_s(1, 1, 0);
            c.swc1(1, p(s2, st::LAND_VEL_X));
            c.g[V0][0] = c.lw(p(s2, st::APPLYING));
        }

        // The landing dip: blend the landing splines toward the target.
        'land: {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.sw(gp_7(c, -0x1e68), 0);
            } else {
                'approach: {
                    c.cycles += 8;
                    c.lwc1(1, p(s2, st::AMPLITUDE_2));
                    c.lwc1(3, gp_7(c, -0x694c));
                    c.lwc1(2, p(s2, st::JUMP_BLEND));
                    c.sub_s(0, 1, 3);
                    c.c_le_s(0, 2);
                    if !c.fcc() {
                        c.add_s(0, 2, 3);
                        c.cycles += 2;
                        c.min_s(0, 0, 1);
                        c.swc1(0, p(s2, st::JUMP_BLEND));
                    } else {
                        c.cycles += 2;
                        c.swc1(1, p(s2, st::JUMP_BLEND));
                    }
                }
                c.cycles += 23;
                c.lwc1(0, p(s2, st::JUMP_SCALE));
                c.g[S1][0] = sx(p(s2, st::LAND_SPLINE_B));
                c.lwc1(12, p(s2, st::JUMP_BLEND));
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = sx(p(s2, st::LAND_SPLINE_A));
                c.mul_s(12, 0, 12);
                call_ptr(c, SPLINE_EVALUATE, 0x0016_3e44)?;
                c.lwc1(1, p(s2, st::JUMP_SCALE));
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(12, p(s2, st::JUMP_BLEND));
                c.lwc1(20, p(s2, st::LAND_BLEND));
                c.mul_s(12, 1, 12);
                c.mul_s(20, 20, 0);
                call_ptr(c, SPLINE_EVALUATE, 0x0016_3e60)?;
                c.lwc1(1, p(s2, st::LAND_BLEND));
                c.g[V0][0] = 0x1;
                c.swc1(20, p(s2, st::JUMP_VEL_Y));
                c.mul_s(1, 1, 0);
                c.swc1(1, p(s2, st::JUMP_VEL_X));
                c.sw(gp_7(c, -0x1e68), c.g[V0][0]);
                c.sw(gp_7(c, 0x1ce8), c.g[S1][0]);
                c.sw(gp_7(c, 0x1cec), c.g[S0][0]);
                // The original's `if (true) break;`: the rebuilt spline handles
                // are left in the globals and the trailing stores are skipped.
                break 'land;
            }
            c.cycles += 2;
            c.sw(gp_7(c, 0x1ce8), 0);
            c.sw(gp_7(c, 0x1cec), 0);
        }

        // Fade the jump/landing state out.
        'fade: {
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(s3, 0x60));
            let info_set = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(p(s2, st::LANDING));
            if !info_set {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = 0x1;
                } else {
                    // `f0` = the normalised time into the current phase.
                    'ratio: {
                        c.cycles += 5;
                        c.lwc1(2, gp_7(c, -0x6948));
                        c.lwc1(1, p(s2, st::CURVE_T));
                        c.c_le_s(2, 1);
                        if !c.fcc() {
                            c.cycles += 1;
                            c.mtc1(0, 0);
                        } else {
                            c.cycles += 6;
                            c.lwc1(0, gp_7(c, -0x6944));
                            c.c_le_s(1, 0);
                            fconst(c, 0, 0x3f80_0000);
                            if c.fcc() {
                                c.sub_s(0, 1, 2);
                            } else {
                                c.cycles += 2;
                            }
                        }
                    }
                    // Smooth the vertical velocity towards the curve.
                    'smooth: {
                        c.cycles += 24;
                        fconst(c, 20, 0x3f80_0000);
                        c.mul_s(1, 0, 31);
                        c.lwc1(2, gp_7(c, -0x6940));
                        c.sub_s(0, 20, 0);
                        c.g[A1][0] = c.lw(p(s2, st::RIG));
                        c.mul_s(0, 0, 2);
                        c.add_s(1, 1, 0);
                        c.swc1(1, p(s2, st::VERTICAL_VEL));
                        c.g[V0][0] = c.lw(r32(c, A1));
                        c.g[A0][0] = c.lh(p(r32(c, V0), 0xb8));
                        c.g[V1][0] = c.lw(p(r32(c, V0), 0xbc));
                        let target = r32(c, V1);
                        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                        call_ptr(c, target, 0x0016_3f10)?;
                        c.lwc1(2, p(s2, st::VERTICAL_VEL));
                        c.mov_s(3, 0);
                        fconst(c, 1, 0x3f00_0000); // 0.5
                        c.sub_s(0, 20, 2);
                        c.lwc1(4, gp_7(c, -0x693c));
                        c.c_le_s(4, 3);
                        c.mul_s(0, 0, 1);
                        let t = !c.fcc();
                        c.add_s(2, 2, 0);
                        if t {
                            c.cycles += 1;
                            c.mtc1(0, 3);
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, gp_7(c, -0x6938));
                            c.c_le_s(3, 0);
                            if !c.fcc() {
                                c.mov_s(3, 20);
                            } else {
                                c.cycles += 5;
                                c.sub_s(1, 3, 4);
                                c.lwc1(0, gp_7(c, -0x6934));
                                c.div_s(3, 1, 0);
                            }
                        }
                        c.cycles += 9;
                        fconst(c, 0, 0x3f80_0000);
                        c.mul_s(1, 3, 2);
                        c.lwc1(2, p(s2, st::VERTICAL_VEL));
                        c.sub_s(0, 0, 3);
                        c.mul_s(0, 0, 2);
                        c.add_s(1, 1, 0);
                        c.swc1(1, p(s2, st::VERTICAL_VEL));
                        c.g[V0][0] = 0x1;
                    }
                }
                c.cycles += 4;
                c.sw(p(s2, st::APPLYING), 0);
                c.sw(p(s2, st::LANDING), c.g[V0][0]);
                c.sw(p(s2, st::JUMP_READY), 0);
                c.g[V0][0] = c.lw(p(s2, st::LANDING));
            }
        }

        // Scale the carried velocities by the vertical blend.
        'vel: {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lw(p(s2, st::APPLYING));
                break 'vel;
            }
            c.cycles += 14;
            c.lwc1(0, p(s2, st::VERTICAL_VEL));
            // After landing the four curve offsets shrink by this factor per
            // update: at another rate by `k^(60/rate)` (the same share per
            // unit of wall-clock time; a factor of 0 or below is kept).
            // Unchanged at 60.
            if crate::tick::rate(c) != 60 && FpuF32(c.f[0]).to_f32() > 0.0 {
                let k = keep_per_tick(c, FpuF32(c.f[0]));
                c.f[0] = k.0;
            }
            c.lwc1(2, p(s2, st::JUMP_VEL_X));
            c.lwc1(3, p(s2, st::JUMP_VEL_Y));
            c.lwc1(4, p(s2, st::LAND_VEL_X));
            c.mul_s(2, 2, 0);
            c.lwc1(1, p(s2, st::LAND_VEL_Y));
            c.mul_s(3, 3, 0);
            c.mul_s(4, 4, 0);
            c.mul_s(1, 1, 0);
            c.swc1(2, p(s2, st::JUMP_VEL_X));
            c.swc1(3, p(s2, st::JUMP_VEL_Y));
            c.swc1(4, p(s2, st::LAND_VEL_X));
            c.swc1(1, p(s2, st::LAND_VEL_Y));
            c.g[V0][0] = c.lw(p(s2, st::APPLYING));
        }

        // `f13` = min(blend / |pos - vel|, 1).
        'normalise: {
            c.cycles += 2;
            let idle = c.g[V0][0] == 0;
            c.lwc1(2, sp_at(c, 0x30));
            if idle {
                c.cycles += 1;
                c.mtc1(0, 13);
            } else {
                c.cycles += 6;
                c.lwc1(1, p(s2, st::JUMP_BLEND));
                fconst(c, 0, 0x3f80_0000);
                c.div_s(13, 1, 2);
                c.min_s(13, 13, 0);
            }
        }

        // Move the eye and look-at points along the curve, then update the
        // camera's placement.
        c.cycles += 112;
        c.lqc2(3, p(s2, st::POS));
        c.lqc2(4, p(s2, st::VEL));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_at(c, 0x10));
        c.lwc1(1, gp_7(c, -0x6930));
        c.lq(2, sp_at(c, 0x10));
        c.g[V1][0] = 0x0050_0000;
        c.lwc1(4, p(s2, st::LAND_VEL_X));
        c.g[A0][0] = c.g[S2][0];
        c.sq(2, sp_at(c, 0));
        let scratch = sp_at(c, 0);
        v_length_q(c, scratch, 5);
        c.mtc1(5, 2);
        fconst(c, 3, 0x3f80_0000);
        c.mul_s(2, 2, 1);
        c.lwc1(0, p(s2, st::JUMP_VEL_X));
        c.sub_s(3, 3, 13);
        c.lwc1(1, p(s2, st::JUMP_VEL_Y));
        c.lwc1(5, p(s2, st::LAND_VEL_Y));
        c.mul_s(4, 2, 4);
        c.mul_s(0, 2, 0);
        c.mul_s(1, 2, 1);
        c.mul_s(4, 3, 4);
        c.mul_s(0, 13, 0);
        c.mul_s(2, 2, 5);
        c.mul_s(1, 13, 1);
        c.add_s(0, 0, 4);
        let up = r32(c, V1).wrapping_sub(0xea0);
        let dif = sp_at(c, 0x10);
        vscale_q(c, 2, 0, up, dif);
        c.mul_s(3, 3, 2);
        c.lq(2, sp_at(c, 0x10));
        c.sq(2, sp_at(c, 0));
        c.add_s(1, 1, 3);
        c.lqc2(3, p(s2, st::POS));
        c.lqc2(4, sp_at(c, 0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(s2, st::POS));
        let up = r32(c, V1).wrapping_sub(0xea0);
        let dif = sp_at(c, 0x10);
        vscale_q(c, 5, 1, up, dif);
        c.lq(2, sp_at(c, 0x10));
        c.sq(2, sp_at(c, 0));
        c.lqc2(3, p(s2, st::VEL));
        c.lqc2(4, sp_at(c, 0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(s2, st::VEL));
        call_ptr(c, CAMERA_UPDATE, 0x0016_40f0)?;
        c.lqc2(3, p(s2, st::POS));
        c.lqc2(4, p(s2, st::VEL));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_at(c, 0x10));
        fconst(c, 0, 0x4248_0000); // 50
        c.lq(2, sp_at(c, 0x10));
        c.max_s(0, 30, 0);
        frame.restore(c, 16, 0x80);
        c.sq(2, sp_at(c, 0));
        let scratch = sp_at(c, 0);
        v_length_q(c, scratch, 3);
        c.mtc1(3, 1);
        frame.restore(c, 17, 0x70);
        c.div_s(0, 0, 1);
        frame.restore(c, 19, 0x50);
        c.mfc1(2, 0);
        c.lqc2(4, sp_at(c, 0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp_at(c, 0));
        frame.restore_ra(c, 0x40);
        c.lqc2(3, p(s2, st::POS));
        c.lqc2(4, sp_at(c, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_at(c, 0x20));
        c.lwc1(31, sp_at(c, 0xe8));
        c.lq(2, sp_at(c, 0x20));
        c.lwc1(30, sp_at(c, 0xe0));
        c.sq(2, p(s2, st::VEL));
        frame.restore(c, 18, 0x60);
        c.lwc1(29, sp_at(c, 0xd8));
        c.lwc1(28, sp_at(c, 0xd0));
        c.lwc1(27, sp_at(c, 0xc8));
        c.lwc1(26, sp_at(c, 0xc0));
        c.lwc1(25, sp_at(c, 0xb8));
        c.lwc1(24, sp_at(c, 0xb0));
        c.lwc1(23, sp_at(c, 0xa8));
        c.lwc1(22, sp_at(c, 0xa0));
        c.lwc1(21, sp_at(c, 0x98));
        c.lwc1(20, sp_at(c, 0x90));
        c.sq(2, sp_at(c, 0x10));
        frame.close(c);
        Ok(())
    });
}

/// `void ManualCamAlgoBase_ctor(ManualCamAlgoBase* a)` (0x0015f938): install
/// the class vtable, default pitch/yaw/fov, the axis quad and the zeroed
/// state words.
pub fn manual_cam_algo_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[V0][0] = sx(0x0045_bde0);
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, 0x0016_7e30, 0x0015_f958)?; // CameraAlgorithm_DefaultFov
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 1);
        c.g[V1][0] = sx(0x0045_bd78);
        c.lwc1(2, gpn(c, 0x6c48));
        c.g[V0][0] = 0x4e;
        c.swc1(0, c.g[S0][0] as u32);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[A0][0] = sx(0x004f_f130);
        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(4));
        c.swc1(2, (c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 0x4f;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x30), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x34), 0);
        c.g[RA][0] = c.ld(sp(c));
        c.lq(3, c.g[A0][0] as u32);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Chase-camera algorithms.

/// The `initFromRider` arguments of the three camera reset wrappers.
#[derive(Clone, Copy)]
pub(crate) enum InitArgs {
    /// `f12 = 100.0f`, `f13 = f12`.
    Hundred,
    /// `lwc1 f12, gp_8+off`, `f13 = 300.0f`.
    GpAnd300(i32),
}

/// Shared body of the three `*_ResetForRider` wrappers: if the rider's camera
/// target id matches, reset the chase algorithm for the rider and initialise
/// it, then notify the camera controller.
pub(crate) fn chase_camera_reset_for_rider(c: &mut Cpu, rets: [u32; 4], args: InitArgs) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x30)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 4)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, A0), 0x6c0)).lw(c);
        c.g[A0][0] = sx(p(r32(c, A0), 0x6c0));
        c.g[A1][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(rets[0]);
        c.g[A0][0] = sx(p(r32(c, A0), r32(c, A1)));
        call_ptr(c, target, rets[0])?;
        if c.g[S1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 14;
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(rets[1]);
            c.g[A0][0] = c.g[S0][0];
            callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER.call(c, rets[1])?;
            match args {
                InitArgs::Hundred => {
                    c.g[AT][0] = 0x42c8_0000;
                    c.mtc1(1, 12);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(rets[2]);
                    c.mov_s(13, 12);
                }
                InitArgs::GpAnd300(off) => {
                    c.lwc1(12, gp_8(c, off));
                    c.g[AT][0] = 0x4396_0000;
                    c.mtc1(1, 13);
                    c.g[RA][0] = sx(rets[2]);
                    c.g[A0][0] = c.g[S0][0];
                }
            }
            callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER.call(c, rets[2])?;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(rets[3]);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, target, rets[3])?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MatrixChaseCameraAlgorithm_ResetForRider(MatrixChaseCameraAlgorithm*
/// self, int id)` (0x160ae8).
pub fn matrix_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    chase_camera_reset_for_rider(
        c,
        [0x0016_0b20, 0x0016_0b34, 0x0016_0b48, 0x0016_0b5c],
        InitArgs::Hundred,
    );
}

/// `void LookBackChaseCameraAlgorithm_ResetForRider(LookBackChaseCameraAlgorithm*
/// self, int id)` (0x168940).
pub fn look_back_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    chase_camera_reset_for_rider(
        c,
        [0x0016_8978, 0x0016_898c, 0x0016_89a0, 0x0016_89b4],
        InitArgs::GpAnd300(-0x66d4),
    );
}

/// `void MidRightChaseCameraAlgorithm_ResetForRider(MidRightChaseCameraAlgorithm*
/// self, int id)` (0x168c50).
pub fn mid_right_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    chase_camera_reset_for_rider(
        c,
        [0x0016_8c88, 0x0016_8c9c, 0x0016_8cb0, 0x0016_8cc4],
        InitArgs::GpAnd300(-0x6610),
    );
}

/// `void MidLeftChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x168f60).
pub fn mid_left_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0016_8f60, BackHeight::Gp(-0x654c))
    })
}

// ---------------------------------------------------------------------------
// Chase camera.

/// `void RelativeChaseCamerAlgorithm_ResetForRider(Cam* self, int id)`
/// (0x160fd0): if the camera's rider is `id`, reset the relative chase camera
/// (drop its blend state and refresh the camera behind the rider).
pub fn relative_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x30)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 4)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, A0), 0x6c0)).lw(c);
        c.g[A0][0] = sx(p(r32(c, A0), 0x6c0));
        c.g[A1][0] = Addr(p(r32(c, V1), 0x38)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x3c)).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A0), r32(c, A1)));
        call_ptr(c, target, 0x0016_1008)?;
        if c.g[S1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 15;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0016_6c60, 0x0016_101c)?;
            c.g[AT][0] = 0x42c8_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S0][0];
            c.mov_s(13, 12);
            call_ptr(c, 0x0016_6550, 0x0016_1030)?;
            Addr(p(r32(c, S0), 0x394)).sw(c, 0);
            c.g[V0][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x2c)).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, target, 0x0016_1048)?;
            frame.restore(c, S0, 0x20);
        }
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.cycles += 4;
        frame.close(c);
        Ok(())
    })
}

/// `ChaseCamController_PushAlgorithm` (0x161e58): store the algorithm's
/// blend weight, then either push it directly (list empty) or clear the
/// list first when the weight is zero.
pub fn chase_cam_controller_push_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r(c, S1), 0xc)).lw(c);
        c.g[A0][0] = Addr(p(this, 8)).lw(c);
        Addr(p(this, 0x18)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(p(r(c, A0), 4)).lw(c);
        let has_algo = c.g[V0][0] != 0;
        c.mov_s(20, 12);
        if has_algo {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.c_eq_s(20, 0);
            let non_zero = !c.fcc();
            if non_zero {
                c.g[A0][0] = Addr(p(r(c, S0), 8)).lw(c);
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0016_1ec4);
                callee::ALGORITHM_LIST_CLEAR.call(c, 0x0016_1ec4)?;
                c.g[A0][0] = Addr(p(r(c, S0), 8)).lw(c);
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            c.mov_s(12, 20);
            c.g[RA][0] = sx(0x0016_1ed8);
            c.g[A2][0] = 1;
            callee::ALGORITHM_LIST_PUSH.call(c, 0x0016_1ed8)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[RA][0] = sx(0x0016_1e9c);
            c.g[A2][0] = 1;
            callee::ALGORITHM_LIST_PUSH.call(c, 0x0016_1e9c)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
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
        c.pc = t;
        return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera.

/// `GameCamera::~GameCamera(this, int flags)` (0x15dd88): clear the camera
/// algorithms, destroy the three camera parts, tear down the animation
/// controller and either free the object or leave the base vtable.
pub fn game_camera_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x460000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4778));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p(r32(c, S0), 0x90)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p(r32(c, S0), 0xa4)).lw(c);
        call_ptr(c, 0x0015_cd60, 0x0015_ddb4)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0xa8)).lw(c);
        call_ptr(c, 0x0015_cd60, 0x0015_ddbc)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0xac)).lw(c);
        call_ptr(c, 0x0015_cd60, 0x0015_ddc4)?;
        c.g[A2][0] = Addr(p(r32(c, S0), 0xa4)).lw(c);
        c.g[A1][0] = 3;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 0x14)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let destroy = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, destroy, 0x0015_dde4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0xa8)).lw(c);
        c.g[A1][0] = 3;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 0x14)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let destroy = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, destroy, 0x0015_de04)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p(r32(c, S0), 0xac)).lw(c);
        c.g[A1][0] = 3;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, A2), 0x14)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let destroy = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, destroy, 0x0015_de24)?;
        }
        c.cycles += 8;
        c.g[A0][0] = sx(p(r32(c, S0), 0xc0));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0017_6da8, 0x0015_de30)?;
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = c.g[S1][0] & 1;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4750));
        let free = c.g[V1][0] == 0;
        Addr(p(r32(c, S0), 0x90)).sw(c, c.g[V0][0]);
        if !free {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0015_de4c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraWobble_UpdateRoll(self, roll)` (0x160130): shake the camera roll
/// towards a random target while the wobble is active. A zero input clears
/// the wobble; otherwise it steps the roll towards the target and picks a new
/// target once close enough. Dispatcher-shaped (the generated body re-enters
/// at its loop blocks), so the replacement keeps the same block addresses.
pub fn camera_wobble_update_roll(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0016_0130 => {
                    c.cycles += 12;
                    c.mtc1(0, 0);
                    let frame = Frame::open(c, 0x50);
                    frame.save(c, 16, 0x30);
                    c.c_eq_s(12, 0);
                    frame.save(c, 17, 0x20);
                    frame.save(c, 18, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.swc1(21, p(sp(c), 0x48));
                    c.swc1(20, p(sp(c), 0x40));
                    let t = c.fcc();
                    c.swc1(12, p(c.g[S0][0] as u32, 0x1c));
                    pc = if t { 0x0016_01f8 } else { 0x0016_0160 };
                }
                0x0016_0160 => {
                    c.cycles += 6;
                    c.lwc1(1, p(c.g[S0][0] as u32, 0x24));
                    c.lwc1(0, p(c.g[S0][0] as u32, 0x2c));
                    c.c_lt_s(0, 1);
                    let t = !c.fcc();
                    c.g[S1][0] = 0x7f_0000;
                    pc = if t { 0x0016_01e8 } else { 0x0016_0178 };
                }
                0x0016_0178 => {
                    c.cycles += 9;
                    c.lwc1(0, p(c.g[S0][0] as u32, 0x14));
                    c.g[AT][0] = 0x3f00_0000;
                    c.mtc1(1, 20);
                    c.g[S1][0] |= 0xffff;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 21);
                    c.g[S2][0] = 0x3f80_0000;
                    c.swc1(0, p(c.g[S0][0] as u32, 8));
                    pc = 0x0016_01c4;
                }
                0x0016_019c => {
                    c.cycles += 1;
                    pc = 0x0016_01a0;
                }
                0x0016_01a0 => {
                    c.cycles += 9;
                    call_ptr(c, 0x0031_77f0, 0x0016_01a8)?; // BXrand()
                    c.g[V0][0] &= c.g[S1][0];
                    c.g[V0][0] |= c.g[S2][0];
                    c.mtc1(2, 1);
                    c.sub_s(0, 1, 21);
                    c.sub_s(0, 0, 20);
                    c.add_s(0, 0, 0);
                    c.swc1(0, p(c.g[S0][0] as u32, 0x14));
                    pc = 0x0016_01c4;
                }
                0x0016_01c4 => {
                    c.cycles += 7;
                    c.lwc1(0, p(c.g[S0][0] as u32, 8));
                    c.lwc1(1, p(c.g[S0][0] as u32, 0x14));
                    c.sub_s(0, 0, 1);
                    c.abs_s(0, 0);
                    c.c_lt_s(0, 20);
                    let t = c.fcc();
                    if t {
                        if back_edge_unwinds(c, 0x0016_01a0) {
                            return Ok(());
                        }
                        pc = 0x0016_01a0;
                    } else {
                        pc = 0x0016_01e0;
                    }
                }
                0x0016_01e0 => {
                    c.cycles += 2;
                    c.sw(p(c.g[S0][0] as u32, 0x24), 0);
                    pc = 0x0016_0204;
                }
                0x0016_01e8 => {
                    c.cycles += 4;
                    c.lwc1(0, gp(c, -0x6c20));
                    c.add_s(0, 1, 0);
                    c.swc1(0, p(c.g[S0][0] as u32, 0x24));
                    pc = 0x0016_0204;
                }
                0x0016_01f8 => {
                    c.cycles += 3;
                    c.swc1(0, p(c.g[S0][0] as u32, 8));
                    c.swc1(0, p(c.g[S0][0] as u32, 0x24));
                    c.swc1(0, p(c.g[S0][0] as u32, 0x14));
                    pc = 0x0016_0204;
                }
                0x0016_0204 => {
                    c.cycles += 8;
                    let frame = Frame { size: 0x50 };
                    frame.restore(c, 16, 0x30);
                    frame.restore(c, 17, 0x20);
                    frame.restore(c, 18, 0x10);
                    frame.restore_ra(c, 0);
                    frame.restore_fpr(c, 21, 0x48);
                    frame.restore_fpr(c, 20, 0x40);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0016_0130, 0x0016_0224);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Cameras, codec and audio.

/// `GameCamera_ctor(self, owner)` (0x0015dc70): build the in-race camera: a
/// camera base, the default 3-chase algorithm, a manual camera controller, a
/// chase-cam controller and a script-cam controller, and reset the state.
pub fn game_camera_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 69;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S0][0];
        c.g[S1][0] = c.g[A1][0];

        c.g[A1][0] = c.g[S1][0];
        Callee {
            addr: 0x0015_d698,
            f: ee::s01::CameraBase_ctor,
        }
        .call(c, 0x0015_dc8c)?;

        // The camera's default algorithm.
        c.g[A0][0] = sx((this as u32).wrapping_add(0xc0));
        c.g[V1][0] = sx(0x0045_b888);
        sw_2(c, this, 0x90, c.g[V1][0]);
        call_ptr(c, 0x0017_6d68, 0x0015_dca0)?;

        // A manual controller.
        c.g[A0][0] = 0x20;
        c.g[A1][0] = sx(0x0045_b748);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0015_dcb8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = this;
        Callee {
            addr: 0x0015_cd80,
            f: ee::s01::ManualCamController_ctor,
        }
        .call(c, 0x0015_dcc4)?;
        c.g[A0][0] = 0x44;
        c.g[A1][0] = sx(0x0045_b760);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        sw_2(c, this, 0xa4, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0015_dce0)?;

        // A chase-cam controller.
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = this;
        Callee {
            addr: 0x0016_19a8,
            f: ee::s01::ChaseCamController_ctor,
        }
        .call(c, 0x0015_dcf0)?;
        c.g[A0][0] = 0x20;
        c.g[A1][0] = sx(0x0045_b778);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        sw_2(c, this, 0xa8, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0015_dd0c)?;

        // A script-cam controller.
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = this;
        call_ptr(c, 0x0016_8fe8, 0x0015_dd1c)?;

        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        sw_2(c, this, 0xac, c.g[V0][0]);
        sw_2(c, this, 0xb0, c.g[V1][0]);
        sw_2(c, this, 0xb4, 0);
        sw_2(c, this, 0x454, 0);
        sw_2(c, this, 0x458, 0);
        sw_2(c, this, 0x45c, 0);
        sw_2(c, this, 0x450, 0);
        c.g[V0][0] = this;
        sw_2(c, this, 0x460, 0);
        c.g[A0][0] = sx(0x004f_f160);
        c.g[A1][0] = sx(0x004f_f130);
        c.g[A2][0] = 1;
        frame.restore(c, S1, 0x10);
        c.lq(3, c.g[A0][0] as u32);
        frame.restore_ra(c, 0);
        c.sq(3, (this as u32).wrapping_add(0x470));
        c.lq(4, c.g[A1][0] as u32);
        sw_2(c, this, 0x4a4, c.g[A2][0]);
        c.sq(4, (this as u32).wrapping_add(0x490));
        sw_2(c, this, 0x4a0, 0);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `float* CameraWobble_GetOffset(CameraWobble* self, float* out)` (0x160228):
/// the camera-wobble offset for the current time: a rotated frame from the
/// wobble's amplitude/phase fields, copied out; the identity offset when the
/// wobble is inactive.
pub fn camera_wobble_get_offset(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0016_028c;
        const SIN: u32 = 0x0031_c040;
        const COS: u32 = 0x0031_bf60;
        c.cycles += 18;
        let f = Frame::open(c, 0xe0);
        f.save(c, S0, 0xa0);
        f.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A1][0];
        f.save_fpr(c, 24, 0xd0);
        c.g[S1][0] = c.g[A0][0];
        f.save_fpr(c, 23, 0xc8);
        f.save_fpr(c, 22, 0xc0);
        c.mov_s(23, 12);
        f.save_ra(c, 0x80);
        f.save_fpr(c, 21, 0xb8);
        f.save_fpr(c, 20, 0xb0);
        c.mtc1(0, 24);
        let obj = c.g[S0][0] as u32;
        c.lwc1(0, obj.wrapping_add(0x18));
        c.c_eq_s(0, 24);
        let fcc = c.fcc();
        c.mov_s(22, 13);
        c.cycles += if !fcc { 80 } else { 5 };
        if fcc {
            // Inactive: copy the identity offset.
            c.g[V0][0] = 0x0050_0000;
            c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0xee0));
            c.lq(3, c.g[V0][0] as u32);
            c.sq(3, c.g[S1][0] as u32);
        } else {
            let out = c.g[S1][0] as u32;
            c.mov_s(12, 22);
            call_ptr(c, SIN, RET)?;
            c.mov_s(21, 0);
            c.mov_s(12, 23);
            call_ptr(c, SIN, RET)?;
            c.mul_s(21, 21, 0);
            c.mov_s(12, 22);
            call_ptr(c, SIN, RET)?;
            c.mov_s(20, 0);
            c.mov_s(12, 23);
            call_ptr(c, COS, RET)?;
            c.mul_s(20, 20, 0);
            c.mov_s(12, 22);
            call_ptr(c, COS, RET)?;
            c.neg_s(0, 0);
            c.swc1(21, f.at(c, 0));
            c.swc1(20, f.at(c, 4));
            c.mov_s(12, 23);
            c.swc1(24, f.at(c, 0xc));
            c.g[RA][0] = sx(RET);
            c.swc1(0, f.at(c, 8));
            c.pc = COS;
            ssx3_ee_rt::funcs::call_indirect(c, COS);
            if c.pc != RET {
                return Err(crate::call::Unwound);
            }
            c.mov_s(20, 0);
            c.mov_s(12, 23);
            call_ptr(c, SIN, RET)?;
            c.neg_s(0, 0);
            c.swc1(20, f.at(c, 0x10));
            c.swc1(24, f.at(c, 0x1c));
            c.swc1(24, f.at(c, 0x18));
            c.swc1(0, f.at(c, 0x14));
            // Cross product of the two axes, the rotation, then the offset.
            c.lqc2(4, f.at(c, 0));
            c.lqc2(5, f.at(c, 0x10));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, ssx3_ee_rt::vu0::Src::Vec(6));
            c.sqc2(6, f.at(c, 0x30));
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 5);
            c.lwc1(2, obj.wrapping_add(0x28));
            c.lwc1(0, obj.wrapping_add(0x20));
            c.lwc1(3, obj);
            c.div_s(0, 0, 2);
            c.lwc1(1, obj.wrapping_add(0xc));
            c.lq(2, f.at(c, 0x30));
            c.sub_s(1, 1, 3);
            c.lwc1(2, obj.wrapping_add(0x10));
            c.lwc1(3, obj.wrapping_add(4));
            c.lwc1(4, obj.wrapping_add(0x18));
            c.sq(2, f.at(c, 0x20));
            c.sub_s(2, 2, 3);
            c.mul_s(1, 0, 1);
            c.mfc1(2, 1);
            c.lqc2(4, f.at(c, 0x10));
            c.qmtc2(2, 3);
            c.vu0().mul(
                0b1111,
                5,
                4,
                ssx3_ee_rt::vu0::Src::Bc(3, ssx3_ee_rt::vu0::X),
            );
            c.sqc2(5, f.at(c, 0x60));
            c.mul_s(4, 4, 5);
            c.lq(2, f.at(c, 0x60));
            c.mul_s(0, 0, 2);
            c.sq(2, f.at(c, 0x50));
            c.mfc1(3, 0);
            c.lqc2(4, f.at(c, 0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(
                0b1111,
                5,
                4,
                ssx3_ee_rt::vu0::Src::Bc(3, ssx3_ee_rt::vu0::X),
            );
            c.sqc2(5, f.at(c, 0x70));
            c.lq(2, f.at(c, 0x70));
            c.sq(2, f.at(c, 0x60));
            c.lqc2(3, f.at(c, 0x50));
            c.lqc2(4, f.at(c, 0x60));
            c.vu0().add(0b1111, 5, 3, ssx3_ee_rt::vu0::Src::Vec(4));
            c.sqc2(5, f.at(c, 0x70));
            c.lq(2, f.at(c, 0x70));
            c.sq(2, f.at(c, 0x40));
            c.mfc1(3, 4);
            c.lqc2(4, f.at(c, 0x40));
            c.qmtc2(3, 3);
            c.vu0().mul(
                0b1111,
                5,
                4,
                ssx3_ee_rt::vu0::Src::Bc(3, ssx3_ee_rt::vu0::X),
            );
            c.sqc2(5, f.at(c, 0x70));
            c.lq(2, f.at(c, 0x70));
            c.sq(2, out);
            c.sq(2, f.at(c, 0x30));
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S1][0];
        f.restore(c, S0, 0xa0);
        f.restore(c, S1, 0x90);
        f.restore_ra(c, 0x80);
        f.restore_fpr(c, 24, 0xd0);
        f.restore_fpr(c, 23, 0xc8);
        f.restore_fpr(c, 22, 0xc0);
        f.restore_fpr(c, 21, 0xb8);
        f.restore_fpr(c, 20, 0xb0);
        f.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera shake.

/// `ChaseCameraAlgorithm_StartShake(self, float ampX, float ampY, float
/// ampZ)` (0x1656b0): arm a shake on a chase camera.  The amplitudes are
/// randomised around the requested values and written to the shake state at
/// `self+0x254..0x2ec`; the slide values restart at zero.
pub fn chase_camera_start_shake(c: &mut Cpu) {
    c.cycles += 162;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = c.g[A0][0];
    c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[S1][0] = 0x7f0000;
    c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x58));
    c.mov_s(24, 12);
    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.mov_s(23, 13);
    c.mov_s(22, 14);
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S1][0] = c.g[S1][0] | 0xffff;
    c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x78));
    c.g[S2][0] = 0x3f800000;
    c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x68));
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.neg_s(25, 22);
    c.g[RA][0] = sx(0x00165708);
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x00165708 {
        return;
    }
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 20);
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.sub_s(26, 22, 25);
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.sub_s(0, 1, 20);
    c.mul_s(0, 23, 0);
    c.add_s(0, 24, 0);
    c.g[RA][0] = sx(0x00165734);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x254));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x00165734 {
        return;
    }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 3);
    c.sub_s(0, 3, 20);
    c.mul_s(0, 23, 0);
    c.add_s(0, 24, 0);
    c.g[RA][0] = sx(0x00165754);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x258));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x00165754 {
        return;
    }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x68d4));
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.mul_s(21, 22, 21);
    c.sub_s(0, 1, 20);
    c.neg_s(27, 21);
    c.mul_s(0, 23, 0);
    c.add_s(0, 24, 0);
    c.g[RA][0] = sx(0x00165780);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x25c));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x00165780 {
        return;
    }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 3);
    c.sub_s(0, 3, 20);
    c.mul_s(0, 26, 0);
    c.add_s(0, 25, 0);
    c.g[RA][0] = sx(0x001657a0);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x260));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x001657a0 {
        return;
    }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.sub_s(0, 1, 20);
    c.mul_s(26, 26, 0);
    c.add_s(25, 25, 26);
    c.g[RA][0] = sx(0x001657c0);
    c.swc1(25, (c.g[S0][0] as u32).wrapping_add(0x264));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x001657c0 {
        return;
    }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.sub_s(1, 21, 27);
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 0);
    c.mtc1(2, 3);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x26c), 0u64);
    c.mul_s(24, 24, 0);
    c.sub_s(2, 3, 20);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x270), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x274), 0u64);
    c.mul_s(23, 23, 0);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x278), 0u64);
    c.mul_s(21, 21, 0);
    c.mul_s(1, 1, 2);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x27c), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x280), 0u64);
    c.mul_s(22, 22, 0);
    c.neg_s(26, 21);
    c.add_s(27, 27, 1);
    c.neg_s(25, 22);
    c.g[RA][0] = sx(0x0016581c);
    c.swc1(27, (c.g[S0][0] as u32).wrapping_add(0x268));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x0016581c {
        return;
    }
    c.sub_s(22, 22, 25);
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.sub_s(0, 1, 20);
    c.mul_s(0, 23, 0);
    c.add_s(0, 24, 0);
    c.g[RA][0] = sx(0x00165840);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x284));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x00165840 {
        return;
    }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 3);
    c.sub_s(0, 3, 20);
    c.mul_s(0, 23, 0);
    c.add_s(0, 24, 0);
    c.g[RA][0] = sx(0x00165860);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x288));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x00165860 {
        return;
    }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.sub_s(0, 1, 20);
    c.mul_s(23, 23, 0);
    c.add_s(24, 24, 23);
    c.g[RA][0] = sx(0x00165880);
    c.swc1(24, (c.g[S0][0] as u32).wrapping_add(0x28c));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x00165880 {
        return;
    }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 3);
    c.sub_s(0, 3, 20);
    c.mul_s(0, 22, 0);
    c.add_s(0, 25, 0);
    c.g[RA][0] = sx(0x001658a0);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x290));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x001658a0 {
        return;
    }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.sub_s(0, 1, 20);
    c.mul_s(22, 22, 0);
    c.add_s(25, 25, 22);
    c.g[RA][0] = sx(0x001658c0);
    c.swc1(25, (c.g[S0][0] as u32).wrapping_add(0x294));
    c.pc = 0x003177f0;
    call_indirect(c, 0x003177f0);
    if c.pc != 0x001658c0 {
        return;
    }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.sub_s(21, 21, 26);
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2b4), 0u64);
    c.mtc1(2, 3);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x29c), 0u64);
    c.sub_s(20, 3, 20);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2a0), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2a4), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2a8), 0u64);
    c.mul_s(21, 21, 20);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2ac), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2b0), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2ec), 0u64);
    c.add_s(26, 26, 21);
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.swc1(26, (c.g[S0][0] as u32).wrapping_add(0x298));
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x78));
    c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x68));
    c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x58));
    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    abi::ret(c);
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
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::strict as strict_2;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_4;
    pub(crate) use crate::game::ai::rider::tests::hh;
    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_8;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::globals_3 as globals_9;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::pool_4 as pool_7;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::w as w_11;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all_2 as check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2 as globals;
    pub(crate) use crate::game::fe::ovstatepause::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::ovstatepause::tests::WORLD;
    pub(crate) use crate::game::render::font::tests::rand_vec;
    pub(crate) use crate::game::render::font::tests::words;
    pub(crate) use crate::game::render::particle::tests::clear as clear_2;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base_14 as base_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2 as belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::check3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_8;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE_DUMMY;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD as WORLD_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD_4 as WORLD_8;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_7;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_8;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_6;
    pub(crate) use crate::game::sound::soundsys::tests::w as w_10;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD as WORLD_7;
    pub(crate) use crate::game::ui::uivector::tests::start;
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
    fn chase_cam_controller_init_state_matches() {
        check(
            ssx3_ee::s01::f_00161950,
            chase_cam_controller_init_state,
            200,
            &|c, seed| {
                let mut r = Rng(0x1619 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 8) as u64;
                c.bus.write32(DATA + 0x30, r.next_u32());
                for k in 0..8u32 {
                    c.bus
                        .write32(GAME_GP.wrapping_add(0x1ce0 + k * 4), r.next_u32());
                }
            },
        );
    }

    #[test]
    fn chase_camera_algorithm_default_heading_p_matches() {
        check(
            ssx3_ee::s02::f_00167d88,
            chase_camera_algorithm_default_heading_p,
            100,
            &|c, seed| {
                let mut r = Rng(0x167d ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x6788), r.edge_f32());
            },
        );
    }

    #[test]
    fn chase_camera_algorithm_default_heading_i_matches() {
        check(
            ssx3_ee::s02::f_00167d98,
            chase_camera_algorithm_default_heading_i,
            100,
            &|c, seed| {
                let mut r = Rng(0x167e ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x6784), r.edge_f32());
            },
        );
    }

    #[test]
    fn chase_camera_algorithm_default_heading_d_matches() {
        check(
            ssx3_ee::s02::f_00167da8,
            chase_camera_algorithm_default_heading_d,
            100,
            &|c, seed| {
                let mut r = Rng(0x167f ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x6780), r.edge_f32());
            },
        );
    }

    // --------------------------------------------------------------- input VM

    /// The input VM's opcode dispatch table and the interpreter's `stop`
    /// opcode handler. The handler is a label inside the interpreter itself,
    /// so the dispatcher continues there instead of tail-calling a separate
    /// function: one operand word ends the run.
    pub(crate) const INPUT_VM_TABLE: u32 = 0x0048_e330;

    pub(crate) const INPUT_VM_STOP: u32 = 0x0032_5ef8;

    pub(crate) const STOP_OPCODE: u32 = 0x20;

    /// `InputMap`: `+4` is the per-binding int array (indices into `+0x14`,
    /// the operand array). Every operand word stops the VM at once.
    pub(crate) fn input_map(c: &mut Cpu, map: u32) {
        c.bus.write32(map + 0x4, map + 0x80);
        c.bus.write32(map + 0x14, map + 0x100);
        for i in 0..4 {
            c.bus.write32(map + 0x80 + 4 * i, i); // operand offset in words
            c.bus.write32(map + 0x100 + 4 * i, STOP_OPCODE);
        }
        c.bus
            .write32(INPUT_VM_TABLE + 4 * STOP_OPCODE, INPUT_VM_STOP);
    }

    #[test]
    fn input_binding_eval_float_matches() {
        check(
            ee::s10::f_00320bf0__generated,
            input_binding_eval_float,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0032_0bf0 ^ seed as u64);
                scramble(c, &mut r);
                let binding = DATA + 0x1000;
                let map = DATA + 0x2000;
                // The binding's second argument becomes the map's binding index.
                c.g[A0][0] = binding as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(binding, 0); // the map's binding index
                c.bus.write32(binding + 4, map);
                input_map(c, map);
            },
        );
    }

    #[test]
    fn smooth_pitch_matches() {
        check(
            ssx3_ee::s02::f_001662a0__generated,
            chase_camera_algorithm_smooth_pitch,
            200,
            &|c, seed| {
                let mut r = Rng(0x1662_a0f3 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let s = DATA;
                c.g[A0][0] = s as u64;
                c.f[12] = r.edge_f32();
                c.f[13] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[15] = r.edge_f32();
                c.bus.write32(s + 0x54, r.edge_f32());
                c.bus.write32(s + 0x1d8, r.edge_f32());
                put_vec(
                    c,
                    s + 0x20,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
                );
                put_vec(
                    c,
                    s + 0x40,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
                );
                // `self+0x30` is the object whose method updates the look-at
                // target; its vtable slot 8/0xc is MenuSys_IsActive here.
                c.bus.write32(s + 0x30, s + 0x100);
                c.bus.write32(s + 0x100, s + 0x200);
                c.bus.write16(s + 0x208, 0);
                c.bus.write32(s + 0x20c, 0x002c_c098);
                c.bus.write32(s + 0x104, s + 0x120);
                c.bus.write32(s + 0x120, 0);
                for off in [
                    0x200u32, 0x204, 0x210, 0x214, 0x218, 0x21c, 0x220, 0x224, 0x228, 0x22c, 0x230,
                    0x234, 0x238, 0x23c, 0x2c0, 0x2c4, 0x2c8, 0x2cc, 0x2d0, 0x2d4, 0x2d8, 0x2e0,
                    0x2e4, 0x2e8, 0x2ec, 0x1b0, 0x1b4, 0x1b8, 0x1bc, 0x1c0,
                ] {
                    c.bus.write32(s + off, r.edge_f32());
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x68c0), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x68bc), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x68b8), r.edge_f32());
            },
        );
    }

    /// The pitch smoothing factors (f14 down, f15 up) are per update: one
    /// 60 Hz update and `hz/60` updates at `hz` leave the smoothed pitch
    /// (+0x1d8) at the same value (old 2, the camera pitch +0x54 held at 1 by
    /// the angle step: `1 + k`).
    #[test]
    fn smooth_pitch_follows_the_rate() {
        fn pitch_after(hz: u32, ticks: u32) -> f32 {
            let mut boxed = crate::difftest::fresh();
            let c: &mut Cpu = &mut boxed;
            let mut r = Rng(0x1662_a0f4);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            crate::tick::set_for_test(c, hz);
            let s = DATA;
            for off in (0..0x300u32).step_by(4) {
                c.bus.write32(s + off, 0);
            }
            put_vec(c, s + 0x20, [0, 0, 0, 0]); // look-at
            put_vec(c, s + 0x40, [0, 0x4316_0000, 0, 0]); // eye
                                                          // The target (`self+0x30`): slot 1 is MenuSys_IsActive, the rider
                                                          // a zeroed block.
            c.bus.write32(s + 0x30, s + 0x100);
            c.bus.write32(s + 0x100, s + 0x200);
            c.bus.write16(s + 0x208, 0);
            c.bus.write32(s + 0x20c, 0x002c_c098);
            c.bus.write32(s + 0x104, s + 0x400);
            c.bus.write32(s + 0x1d8, 2.0f32.to_bits()); // the old smoothed pitch
                                                        // No angle limit (the aimed direction never "long enough"), a
                                                        // linear turn rate of 1, no distance reset limit.
            c.bus
                .write32(GAME_GP.wrapping_sub(0x68c0), 1e30f32.to_bits());
            c.bus.write32(GAME_GP.wrapping_sub(0x68bc), 0);
            c.bus
                .write32(GAME_GP.wrapping_sub(0x68b8), 1.0f32.to_bits());
            c.bus
                .write32(GAME_GP.wrapping_sub(0x6778), 1e30f32.to_bits());
            for _ in 0..ticks {
                c.g[A0][0] = s as u64;
                c.f[12] = 0.5f32.to_bits(); // target angles
                c.f[13] = 0.5f32.to_bits();
                c.f[14] = 0.9f32.to_bits(); // down
                c.f[15] = 0.9f32.to_bits(); // up
                c.bus.write32(s + 0x54, 1.0f32.to_bits()); // this update's pitch
                chase_camera_algorithm_smooth_pitch(c);
            }
            f32::from_bits(c.bus.read32(s + 0x1d8))
        }

        let at_60 = pitch_after(60, 1);
        assert!((at_60 - 1.9).abs() < 1e-5, "at 60: {at_60}");
        for hz in [120u32, 240] {
            let fast = pitch_after(hz, hz / 60);
            assert!(
                (fast - at_60).abs() < 1e-4,
                "at {hz}: {fast}, at 60: {at_60}"
            );
        }
        // Unconverted, two 120 Hz updates would leave 1 + 0.9² = 1.81.
        assert!(pitch_after(120, 2) > 1.85);
    }

    #[test]
    fn camera_algorithm_default_fov_matches() {
        check(
            ee::s02::f_00167e30__generated,
            camera_algorithm_default_fov,
            SEEDS_2,
            &|c, seed| {
                let mut r = rng(0x0016_7e30, seed);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                let fov = if seed % 2 == 0 {
                    0x3f49_0fdb
                } else {
                    r.edge_f32()
                };
                c.bus.write32(0x0049_c974, fov);
            },
        );
    }

    // ---------------------------------------------------------- camera vtable

    #[test]
    fn camera_algorithm_tick_matches() {
        check(
            ee::s02::f_00167e68__generated,
            camera_algorithm_tick,
            100,
            &|c, seed| {
                let mut r = game(c, seed, 0x167e68, 1);
                let this = SCRATCH + 0x3000;
                c.g[A0][0] = this as u64;
                // `*(this + 0x10)` is the algorithm object; `+0x28` holds the
                // `this` delta the call adds and `+0x2c` the method pointer (a fake).
                let obj = fakes::ptr(&mut r);
                c.bus.write32(this + 0x10, obj);
                c.bus.write16(obj + 0x28, (r.next_u32() & 0xff8) as u16);
                c.bus.write32(obj + 0x2c, fakes::FAKE);
            },
        );
    }

    // -----------------------------------------------------------------
    // Camera.

    #[test]
    fn chase_camera_algorithm_clear_heading_ring_matches() {
        check(
            s02::f_00167db8__generated,
            chase_camera_algorithm_clear_heading_ring,
            200,
            &|c, seed| {
                let mut r = Rng(0x167d_b8aa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ);
                clear(c, OBJ, 0x60);
                // The five heading samples and their duplicates.
                for i in 0..5u32 {
                    c.bus.write32(OBJ + 4 * i, r.next_u32());
                    c.bus.write32(OBJ + 0x14 + 4 * i, r.next_u32());
                    c.bus.write32(OBJ + 0x28 + 4 * i, r.next_u32());
                }
                c.bus.write32(OBJ + 0x3c, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------------
    // Camera.

    #[test]
    fn chase_cam_reset_to_default_matches() {
        check(
            ssx3_ee::s01::f_00162170__generated,
            chase_cam_reset_to_default,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x62170, 0);
                let cam = DATA + 0x100;
                // A small algorithm id takes SetAlgorithm's jump-table path: the
                // table is empty in RAM, so the call goes out through the hook.
                c.bus.write32(cam + 0x24, 1 + r.next_u32() % 6);
                c.bus.write32(cam + 0x30, r.next_u32());
                c.g[A0][0] = sx(cam);
            },
        );
    }

    #[test]
    fn chase_cam_start_post_race_camera_matches() {
        check(
            ssx3_ee::s01::f_00162258__generated,
            chase_cam_start_post_race_camera,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x62258, 0);
                let cam = DATA + 0x100;
                // bit 0 of +0x30: already replaying, skip the algorithm change.
                c.bus.write32(cam + 0x30, (seed % 2) as u32);
                c.bus.write32(cam + 0x24, r.next_u32());
                c.g[A0][0] = sx(cam);
            },
        );
    }

    /// A camera blend rate authored per 60 Hz tick is `60/rate` of it per
    /// tick at another rate, so the blend takes the same time; a rate of 1.0
    /// ("instant") is never scaled.
    ///
    /// The two converted callers pass this value to
    /// `ChaseCamController_SetAlgorithm`, which is still generated and whose
    /// body writes the node rate, so the end-to-end observation would need
    /// that port first (the fixture cannot reach its push on test data).
    #[test]
    fn blend_rate_60_scales_per_tick_steps() {
        let mut c = crate::difftest::fresh();
        let raw = FpuF32::from_f32(0.016_793_4);
        crate::tick::set_for_test(&mut c, 60);
        assert_eq!(blend_rate_60(&mut c, raw), raw, "60 Hz unchanged");
        for hz in [120u32, 240] {
            crate::tick::set_for_test(&mut c, hz);
            let r = blend_rate_60(&mut c, raw).to_f32();
            assert!(
                (r * hz as f32 - raw.to_f32() * 60.0).abs() < 1e-7,
                "{hz} Hz: {r} per tick"
            );
            assert_eq!(blend_rate_60(&mut c, FpuF32::ONE), FpuF32::ONE, "instant");
        }
    }

    /// The engine's size-class pool allocator (`MEM_PoolAlloc`): the pools must
    /// be marked ready and have free lists, or allocations grow the (zeroed)
    /// `DLHeap` and the frees walk a corrupt chain forever.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        let gp = GAME_GP;
        c.bus.write32(gp + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = 0x008c_0000 + 0x1000 * i;
            let base = 0x008b_0000 + 0x100 * i;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(gp + slot, pool);
        }
    }

    /// A zeroed algorithm object at `DATA`; all the vtable slots the base
    /// constructors reach are real functions.
    pub(crate) fn algo_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        for i in 0..0x100u32 {
            c.bus.write32(DATA + i * 4, r.next_u32());
        }
        for i in 0..0x100u32 {
            c.bus.write32(DATA + 0x400 + i * 4, r.next_u32());
        }
    }

    #[test]
    fn manual_cam_algo_ctor_matches() {
        check(
            ssx3_ee::s01::f_0015fd08__generated,
            manual_cam_algo_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x15fd_08);
            },
        );
    }

    #[test]
    fn editor_cam_algo_ctor_matches() {
        check(
            ssx3_ee::s01::f_0015ffb0__generated,
            editor_cam_algo_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x15ff_b0);
            },
        );
    }

    #[test]
    fn manual_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s01::f_00160438__generated,
            manual_chase_camera_algorithm_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x1604_38);
            },
        );
    }

    #[test]
    fn idle_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s01::f_001616a8__generated,
            idle_chase_camera_algorithm_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x1616_a8);
            },
        );
    }

    #[test]
    fn lazy_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s02::f_001682b8__generated,
            lazy_chase_camera_algorithm_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x1682_b8);
            },
        );
    }

    #[test]
    fn look_back_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s02::f_00168650__generated,
            look_back_chase_camera_algorithm_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x1686_50);
            },
        );
    }

    #[test]
    fn mid_right_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s02::f_001689c8__generated,
            mid_right_chase_camera_algorithm_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x1689_c8);
            },
        );
    }

    #[test]
    fn mid_left_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s02::f_00168cd8__generated,
            mid_left_chase_camera_algorithm_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x168c_d8);
            },
        );
    }

    // -----------------------------------------------------------------------
    // ChaseCamController_ExitSpokeMode.

    #[test]
    fn chase_cam_controller_exit_spoke_mode_matches() {
        check(
            ssx3_ee::s01::f_00162218__generated,
            chase_cam_controller_exit_spoke_mode,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1622_18, 0);
                pool_allocator(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[GP][0] = GAME_GP as u64;
                let flags = match seed % 3 {
                    0 => 0,
                    1 => 1 + r.next_u32() % 3,
                    _ => r.next_u32(),
                };
                c.bus.write32(this + 0x30, flags);
                // `SetAlgorithm` with an out-of-range algorithm id takes its
                // "no change" path; the algorithm object itself stays zeroed.
                c.bus.write32(this + 0x24, 0x100);
                c.bus.write32(this + 0x20, 0);
                c.bus.write32(this + 0x8, 0);
                c.bus.write32(this + 0x18, 0);
            },
        );
    }

    #[test]
    fn matrix_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s01::f_00160760__generated,
            matrix_chase_camera_algorithm_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x0e, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn chase_cam_controller_set_target_matches() {
        check(
            ssx3_ee::s01::f_00161a60__generated,
            chase_cam_controller_set_target,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x0f, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
                // A previous target is deleted; the null case covers the skip.
                // The allocator's mutex looks held by this thread, so `delete`
                // takes its fast path (the `dispatch.rs` tests' recipe).
                c.bus.write32(0x0053_8b00 + 4, 0x42);
                c.bus.write32(0x0090_0010, 0x42);
                c.bus.write32(0x0053_8b00 + 8, seed % 4);
                c.bus
                    .write32(DATA + 0x20, if seed % 2 == 0 { 0 } else { DATA + 0x4000 });
            },
        );
    }

    #[test]
    fn relative_chase_camera_algorithm_ctor_matches() {
        check(
            ee::s01::f_00160b70__generated,
            relative_chase_camera_algorithm_ctor,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16_0b70, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.f[12] = if seed % 5 == 0 {
                    0xbf80_0000
                } else {
                    r.edge_f32()
                };
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x161a10 ChaseCamController_dtor.

    #[test]
    fn chase_cam_controller_dtor_matches() {
        check(
            ee::s01::f_00161a10__generated,
            chase_cam_controller_dtor,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x161a_10, 0);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = ((seed as u64) & 0xfe) << 1; // flags bit 0 clear: no delete
                c.bus.write32(this + 0x20, 0); // null camera: free is a no-op
                c.bus.write32(this + 8, 0); // no algorithm list
            },
        );
    }

    // ------------------------------------------------------ camera controller

    #[test]
    fn manual_cam_controller_select_algorithm_matches() {
        check(
            ee::s01::f_0015cf80__generated,
            manual_cam_controller_select_algorithm,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x15cf);
                let cam = DATA;
                c.bus.write32(cam + 0x18, r.next_u32() % 4);
                c.bus.write32(cam + 0x1c, r.next_u32() % 4);
                fill(c, &mut r, cam, 4);
                let msg = match seed % 3 {
                    0 => 0x50u64,
                    1 => 0x51,
                    _ => r.next_u32() as u64,
                };
                c.g[A0][0] = sx(cam);
                c.g[A1][0] = msg;
            },
        );
    }

    // ------------------------------------------------------------- camera

    pub(crate) fn camera_chain(c: &mut Cpu, seed: u32) {
        let world = DATA + 0x100;
        let cam = DATA + 0x200;
        let chase = DATA + 0x300;
        let (o1, ctrl1) = (DATA + 0x400, DATA + 0x500);
        let (o2, ctrl2) = (DATA + 0x600, DATA + 0x700);
        let have = seed % 4 != 0;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, cam);
        c.bus.write32(cam + 0x84, if have { chase } else { 0 });
        c.bus.write32(chase + 4, o1);
        c.bus.write32(o1 + 0xa8, ctrl1);
        c.bus.write32(chase + 8, o2);
        c.bus.write32(o2 + 0xa8, ctrl2);
        c.bus.write32(
            chase,
            match seed % 3 {
                0 => 1,
                1 => 2,
                _ => 0,
            },
        );
    }

    #[test]
    fn game_camera_reset_all_chase_controllers_matches() {
        check(
            ssx3_ee::s01::f_0015dc18__generated,
            game_camera_reset_all_chase_controllers,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x15dc18, 2);
                camera_chain(c, seed);
            },
        );
    }

    #[test]
    fn chase_cam_controller_set_default_algorithm_matches() {
        check(
            ee::s01::f_00161ef0__generated,
            chase_cam_controller_set_default_algorithm,
            SEEDS_3,
            &|c, seed| {
                let mut r = common(c, seed, 0x161ef0, 1);
                let this = DATA + 0x100;
                let cam = DATA + 0x200;
                let owner = DATA + 0x300;
                clear_2(c, this, 0x100);
                clear_2(c, cam, 0x100);
                clear_2(c, owner, 0x900 / 4);
                c.bus.write32(this + 0x20, cam);
                c.bus.write32(cam + 4, owner);
                c.bus.write32(this + 0x30, (seed % 9) as u32);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64 & !3;
            },
        );
    }

    pub(crate) fn camera_target_fixture(c: &mut Cpu, r: &mut Rng, off: u32) {
        scramble(c, r);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (DATA + 0x100) as u64;
        c.bus.write32(DATA + 0x104, DATA + 0x400);
        rand_vec(c, r, DATA + 0x400 + off);
    }

    #[test]
    fn rider_camera_target_get_orientation_matches() {
        check(
            ssx3_ee::s01::f_0015f598__generated,
            rider_camera_target_get_orientation,
            120,
            &|c, seed| {
                let mut r = Rng(0x15f5_9800 ^ seed as u64);
                camera_target_fixture(c, &mut r, 0x120);
            },
        );
    }

    #[test]
    fn rider_camera_target_get_forward_matches() {
        check(
            ssx3_ee::s01::f_0015f5f8__generated,
            rider_camera_target_get_forward,
            120,
            &|c, seed| {
                let mut r = Rng(0x15f5_f800 ^ seed as u64);
                camera_target_fixture(c, &mut r, 0x1b0);
            },
        );
    }

    #[test]
    fn rider_camera_target_get_side_matches() {
        check(
            ssx3_ee::s01::f_0015f610__generated,
            rider_camera_target_get_side,
            120,
            &|c, seed| {
                let mut r = Rng(0x15f6_1000 ^ seed as u64);
                camera_target_fixture(c, &mut r, 0x1a0);
            },
        );
    }

    #[test]
    fn rider_camera_target_get_up_matches() {
        check(
            ssx3_ee::s01::f_0015f628__generated,
            rider_camera_target_get_up,
            120,
            &|c, seed| {
                let mut r = Rng(0x15f6_2800 ^ seed as u64);
                camera_target_fixture(c, &mut r, 0x1c0);
            },
        );
    }

    #[test]
    fn rider_camera_target_get_ground_normal_matches() {
        check(
            ssx3_ee::s01::f_0015f650__generated,
            rider_camera_target_get_ground_normal,
            120,
            &|c, seed| {
                let mut r = Rng(0x15f6_5000 ^ seed as u64);
                camera_target_fixture(c, &mut r, 0x370);
            },
        );
    }

    #[test]
    fn rider_camera_target_get_prev_ground_normal_matches() {
        check(
            ssx3_ee::s01::f_0015f668__generated,
            rider_camera_target_get_prev_ground_normal,
            120,
            &|c, seed| {
                let mut r = Rng(0x15f6_6800 ^ seed as u64);
                camera_target_fixture(c, &mut r, 0x380);
            },
        );
    }

    pub(crate) fn air_time_fixture(c: &mut Cpu, r: &mut Rng, low: u32) {
        scramble(c, r);
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 4, DATA + 0x400);
        c.bus.write32(DATA + 0x400 + 0x788, DATA + 0x800);
        c.bus.write32(DATA + 0x800 + low, r.edge_f32());
        c.bus.write32(DATA + 0x800 + 0xa0, r.edge_f32());
    }

    #[test]
    fn rider_camera_target_get_air_time_remaining_matches() {
        check(
            ssx3_ee::s01::f_0015f6b0__generated,
            rider_camera_target_get_air_time_remaining,
            200,
            &|c, seed| {
                let mut r = Rng(0x15f6_b000 ^ seed as u64);
                air_time_fixture(c, &mut r, 0x98);
            },
        );
    }

    #[test]
    fn rider_camera_target_get_air_time_9c_remaining_matches() {
        check(
            ssx3_ee::s01::f_0015f6c8__generated,
            rider_camera_target_get_air_time_9c_remaining,
            200,
            &|c, seed| {
                let mut r = Rng(0x15f6_c800 ^ seed as u64);
                air_time_fixture(c, &mut r, 0x9c);
            },
        );
    }

    #[test]
    fn rider_camera_target_is_landing_predicted_matches() {
        check(
            ssx3_ee::s01::f_0015f720__generated,
            rider_camera_target_is_landing_predicted,
            160,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn camera_algorithm_get_look_at_matches() {
        check(
            ssx3_ee::s02::f_00167e40__generated,
            camera_algorithm_get_look_at,
            120,
            &|c, seed| {
                let mut r = Rng(0x167e_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                put_vec(
                    c,
                    0x004f_f120,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
                rand_vec(c, &mut r, DATA);
            },
        );
    }

    #[test]
    fn rider_camera_algorithm_get_look_at_matches() {
        check(
            ssx3_ee::s02::f_00167fa0__generated,
            rider_camera_algorithm_get_look_at,
            120,
            &|c, seed| {
                let mut r = Rng(0x167f_a000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                rand_vec(c, &mut r, DATA + 0x120);
                rand_vec(c, &mut r, DATA);
            },
        );
    }

    #[test]
    fn mid_right_chase_camera_algorithm_get_eye_drop_matches() {
        check(
            ssx3_ee::s02::f_00168a30__generated,
            mid_right_chase_camera_algorithm_get_eye_drop,
            120,
            &|c, seed| {
                let mut r = Rng(0x168a_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x66d0, r.edge_f32());
            },
        );
    }

    #[test]
    fn mid_right_chase_camera_algorithm_get_tuning_matches() {
        check(
            ssx3_ee::s02::f_00168a40__generated,
            mid_right_chase_camera_algorithm_get_tuning,
            160,
            &|c, seed| {
                let mut r = Rng(0x168a_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A1][0] = DATA as u64;
                c.bus.write32(GAME_GP - 0x66cc, 6.6548877f32.to_bits());
                c.bus.write32(GAME_GP - 0x66c8, 16.140755f32.to_bits());
                c.bus.write32(GAME_GP - 0x66c4, 0.84256876f32.to_bits());
                c.bus.write32(GAME_GP - 0x66c0, 0.92642629f32.to_bits());
                words(
                    c,
                    DATA,
                    &[
                        r.next_u32(),
                        r.next_u32(),
                        r.next_u32(),
                        r.next_u32(),
                        r.next_u32(),
                    ],
                );
            },
        );
    }

    #[test]
    fn mid_left_chase_camera_algorithm_get_eye_drop_matches() {
        check(
            ssx3_ee::s02::f_00168d40__generated,
            mid_left_chase_camera_algorithm_get_eye_drop,
            120,
            &|c, seed| {
                let mut r = Rng(0x168d_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x660c, r.edge_f32());
            },
        );
    }

    // ------------------------------------------------------ camera tuning

    #[test]
    fn mid_left_chase_camera_algorithm_get_tuning_matches() {
        check(
            ssx3_ee::s02::f_00168d50__generated,
            mid_left_chase_camera_algorithm_get_tuning,
            100,
            &|c, seed| {
                let mut r = start(c, seed, 0x6608);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                for i in 0..4 {
                    c.bus
                        .write32((GAME_GP as i32 - 0x6608 + 4 * i) as u32, r.edge_f32());
                }
                for i in 0..5 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn rider_camera_algorithm_ctor_matches() {
        check(
            ee::s01::f_001622b0__generated,
            rider_camera_algorithm_ctor,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1622_b0);
                let s = DATA + 0x100;
                clear_3(c, s, 0x40);
                w(c, gp_at(-0x6b0c), r.next_u32());
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn chase_cam_controller_ctor_matches() {
        check(
            ee::s01::f_001619a8__generated,
            chase_cam_controller_ctor,
            8,
            &|c, seed| {
                machine(c, seed, 0x0016_19a8, 2);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
            },
        );
    }

    #[test]
    fn spline_camera_algorithm_dtor_matches() {
        check(
            ee::s01::f_00161370__generated,
            spline_camera_algorithm_dtor,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x161370, 0);
                globals_2(c);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
            },
        );
    }

    #[test]
    fn chase_cam_controller_clear_override_matches() {
        check(
            ee::s01::f_001620d0__generated,
            chase_cam_controller_clear_override,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x1620d0, 0);
                globals_2(c);
                c.g[A0][0] = OBJ_2 as u64;
                // 0x62+ makes `ChaseCamController_SetAlgorithm` return at once.
                if seed % 3 == 0 {
                    w(c, OBJ_2 + 0x2c, 0x5d);
                    w(c, OBJ_2 + 0x24, 0x62 + (seed & 1));
                } else {
                    w(c, OBJ_2 + 0x2c, 0x62 + (seed & 1));
                }
                w(c, OBJ_2 + 0x30, if seed % 2 == 0 { 0 } else { 3 });
            },
        );
    }

    // -------------------------------------------------- camera and memcard

    #[test]
    fn chase_cam_controller_enter_spoke_mode_matches() {
        check(
            ee::s01::f_001621a8__generated,
            chase_cam_controller_enter_spoke_mode,
            6,
            &|c, seed| {
                let _ = base_3(c, seed, 0x0016_21a8, 2);
                c.g[A0][0] = OBJ as u64;
                let holder = DATA + 0x2000;
                let rider = DATA + 0x2200;
                w_2(c, OBJ + 0x20, holder);
                w_2(c, holder + 4, rider);
                w_2(c, rider + 0x880, if seed % 3 == 0 { 5 } else { 0 });
                w_2(c, OBJ + 0x30, if seed % 4 == 0 { 1 } else { 0 });
            },
        );
    }

    // The chase-camera state, its info block and the rider/model object (whose
    // first word is the vtable) live high enough that `fakes::ptr` (below
    // `DATA + 0x10_0000`) and the fake results can never overwrite them.
    pub(crate) const ME: u32 = DATA + 0x0020_0000;

    pub(crate) const INFO: u32 = DATA + 0x0020_1000;

    pub(crate) const RIG: u32 = DATA + 0x0020_2000;

    pub(crate) const BE_2: u32 = DATA + 0x0020_3000;

    pub(crate) const WORLD_3: u32 = DATA + 0x0020_4000;

    pub(crate) fn w_3(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    pub(crate) fn wf(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
    }

    /// The BELibrary singleton (a zeroed dummy, so nothing reads the disc) and
    /// the world (`gp - 0x848`, with no riders).
    pub(crate) fn globals_4(c: &mut Cpu) {
        w_3(c, GAME_GP.wrapping_sub(0x1f04), BE_2);
        w_3(c, GAME_GP.wrapping_sub(0x848), WORLD_3);
        let game = WORLD_3 + 0x1000;
        w_3(c, WORLD_3 + 0x84, game);
        let race = WORLD_3 + 0x2000;
        w_3(c, game + 0xc, race);
        w_3(c, race + 0x7c, 0);
    }

    /// The engine's size-class pool_3 allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class (the spline's `Alloc` takes a 160-byte block
    /// from the 128-byte class, `gp + 0x2a84`).
    pub(crate) fn pool_3(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w_3(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool_3 = DATA + 0x0040_0000 + 0x0004_0000 * i;
            let base = DATA + 0x0080_0000 + 0x0000_4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w_3(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w_3(c, pool_3 + head, base);
            w_3(c, GAME_GP + slot, pool_3);
        }
    }

    /// The chase-camera state with random transforms, a fake rig vtable and a
    /// zeroed spline object.
    pub(crate) fn fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 3);
        globals_4(c);
        pool_3(c);
        for i in 0..0x360u32 / 4 {
            w_3(c, ME + 4 * i, 0);
        }
        for i in 0..0x40u32 / 4 {
            w_3(c, INFO + 4 * i, 0);
        }
        // Rig + vtable: the slots the algorithm queries (+0x60, +0x80, +0x98).
        let vt = vtable(c, &mut r, &[0x60, 0x80, 0x98]);
        w_3(c, RIG, vt);
        w_3(c, ME + super::st::RIG, RIG);
        // Random transforms.
        put_vec(
            c,
            ME + super::st::POS,
            [r.f32(1e4), r.f32(1e4), r.f32(1e4), r.f32(1.0)],
        );
        put_vec(
            c,
            ME + super::st::VEL,
            [r.f32(1e3), r.f32(1e3), r.f32(1e3), r.f32(1.0)],
        );
        put_vec(
            c,
            ME + super::st::AXIS,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::AXIS_SCALED,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::BASIS_A,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::BASIS_B,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::BASIS_C,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::PREV_DELTA,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::PREV_MOVE,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        wf(c, ME + super::st::SCALE, r.edge_f32());
        wf(c, ME + super::st::SCALE + 4, r.edge_f32());
        wf(c, ME + super::st::SCALE + 8, r.edge_f32());
        put_vec(
            c,
            INFO + 0x10,
            [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(0.0)],
        );
        put_vec(
            c,
            INFO + 0x20,
            [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(0.0)],
        );
        // Swing state: cover airborne / not, swinging / not, and both clamps.
        w_3(c, ME + super::st::AIRBORNE, (seed % 3 != 0) as u32);
        w_3(c, ME + super::st::SWINGING, (seed % 4 == 0) as u32);
        wf(c, ME + super::st::PHASE, r.f32(3.0));
        wf(
            c,
            ME + super::st::FREQ,
            if seed % 5 == 0 {
                0.0f32.to_bits()
            } else {
                r.f32(2.0)
            },
        );
        wf(
            c,
            ME + super::st::AMPLITUDE,
            if seed % 4 == 0 {
                0.0f32.to_bits()
            } else {
                r.f32(1.0)
            },
        );
        wf(c, ME + super::st::GAIN, r.f32(1.0));
        wf(
            c,
            ME + super::st::TIME,
            if seed % 5 == 1 {
                0.0f32.to_bits()
            } else {
                r.f32(2.0)
            },
        );
        wf(
            c,
            ME + super::st::IN_TIME,
            if seed % 6 == 0 {
                0.0f32.to_bits()
            } else {
                r.f32(2.0)
            },
        );
        wf(
            c,
            ME + super::st::OUT_TIME,
            if seed % 6 == 3 {
                0.0f32.to_bits()
            } else {
                r.f32(-2.0)
            },
        );
        // The gp-relative `.lit4` constants the code scales by.
        wf(c, GAME_GP.wrapping_sub(0x6920), 0.5f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x691c), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x6918), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x6914), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x6910), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x690c), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x6908), 1.5707964f32.to_bits());
        for off in [0x6904u32, 0x6900, 0x68fc, 0x68f8, 0x68f4, 0x68f0, 0x68ec] {
            wf(c, GAME_GP.wrapping_sub(off), r.edge_f32());
        }
        wf(c, GAME_GP.wrapping_sub(0x68e8), 0.5f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x68e4), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x68e0), 0.5f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x68dc), 0.5f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x68d8), 0.5f32.to_bits());
        // `float dt`.
        c.f[12] = (((r.next_u32() % 500) + 1) as f32 / 1000.0).to_bits();
        c.g[A0][0] = ME as u64;
        c.g[A1][0] = INFO as u64;
        r
    }

    #[test]
    fn spline_camera_algorithm_reset_for_rider_matches() {
        check(
            ee::s01::f_00161630__generated,
            spline_camera_algorithm_reset_for_rider,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x16_1630, 4);
                belib(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                // Half the seeds match the camera's current entry (the BELibrary
                // singleton is a deterministic return value), the rest get a
                // fake visitor that returns an unrelated pointer.
                let same = seed % 2 == 0;
                c.g[A1][0] = if same {
                    (DATA + 0x8000) as u64
                } else {
                    DATA as u64
                };
                w_4(c, DATA + 0x2000 + 0x30, DATA + 0x2200);
                w_4(c, DATA + 0x2200 + 4, DATA + 0x2400);
                w_4(c, DATA + 0x2400 + 0x6c0, DATA + 0x2600);
                let get = if seed % 2 == 0 { 0x0014_dc80 } else { FAKE };
                slot(c, DATA + 0x2600, 0x38, 0, get);
                w_4(c, DATA + 0x2000 + 0x10, DATA + 0x2800);
                slot(c, DATA + 0x2800, 0x28, 0, FAKE);
                c.pc = 0x0016_1630;
            },
        );
    }

    pub(crate) const SEEDS_5: u32 = 120;

    pub(crate) const VT: u32 = DATA + 0x0020_3000;

    pub(crate) const BE_3: u32 = DATA + 0x0020_4000;

    pub(crate) const WORLD_4: u32 = DATA + 0x0020_5000;

    /// `ScriptList_GetFieldB0`: `v0 = *(a0 + 0xb0)`, used to hand the jump
    /// timer's vtable slot_2 a controlled zero.
    pub(crate) const LOAD_B0: u32 = 0x0027_72b8;

    pub(crate) fn w_5(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// `{ short delta; short; void (*fn)(); }` at `vt + off`.
    pub(crate) fn slot_2(c: &mut Cpu, vt: u32, off: u32, func: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, func);
    }

    /// The BELibrary singleton (a zeroed dummy, so nothing reads the disc) and
    /// the world (`gp_7 - 0x848`, with no riders).
    pub(crate) fn globals_5(c: &mut Cpu) {
        w_5(c, GAME_GP.wrapping_sub(0x1f04), BE_3);
        w_5(c, GAME_GP.wrapping_sub(0x848), WORLD_4);
        let game = WORLD_4 + 0x1000;
        w_5(c, WORLD_4 + 0x84, game);
        let race = WORLD_4 + 0x2000;
        w_5(c, game + 0xc, race);
        w_5(c, race + 0x7c, 0);
    }

    /// The engine's size-class pool_4 allocator: `gp_7 + 0xdd8` ready plus eight
    /// free nodes per size class (the spline's `Alloc` takes its 0x104-byte
    /// point array from the first class).
    pub(crate) fn pool_4(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w_5(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot_2, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool_4 = DATA + 0x0040_0000 + 0x0004_0000 * i;
            let base = DATA + 0x0080_0000 + 0x0000_4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w_5(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w_5(c, pool_4 + head, base);
            w_5(c, GAME_GP + slot_2, pool_4);
        }
    }

    /// The chase-camera state with random transforms, a fake rig vtable and
    /// zeroed spline objects. The timer selects the function's three paths:
    /// 1 -> the active (curve-evaluating) path, 15 -> the rebuild path, else
    /// the early exit.
    pub(crate) fn fixture_2(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1635f8, 3);
        globals_5(c);
        pool_4(c);
        for i in 0..0x3c0u32 / 4 {
            w_5(c, ME + 4 * i, 0);
        }
        for i in 0..0x80u32 / 4 {
            w_5(c, INFO + 4 * i, 0);
        }
        for i in 0..0x400u32 / 4 {
            w_5(c, RIG + 4 * i, 0);
        }
        for i in 0..0x400u32 / 4 {
            w_5(c, VT + 4 * i, 0);
        }
        // The rig and its vtable (slots +0x08, +0x60, +0x64, +0x98, +0x9c,
        // +0xa8, +0xac, +0xb8, +0xbc).
        w_5(c, RIG, VT);
        w_5(c, RIG + 4, RIG + 0x200); // the rider
        w_5(c, ME + st::RIG, RIG);
        slot_2(c, VT, 0x8, FAKE);
        // Slot +0x60 (the air arc's scale, f27): `Rider_GetDistToFinish`, a
        // real leaf returning the float at +0x4d0, kept positive so the
        // landing splines' knot times (scaled by it) ascend.
        slot_2(c, VT, 0x60, 0x0011_3128);
        wf(
            c,
            RIG + 0x4d0,
            (0.05f32 + (r.next_u32() % 100) as f32 / 50.0).to_bits(),
        );
        slot_2(c, VT, 0xa8, FAKE);
        slot_2(c, VT, 0xb8, FAKE);
        if seed % 2 == 0 {
            // `v0 == 0`: resume the landing timer.
            slot_2(c, VT, 0x98, LOAD_B0);
            w_5(c, RIG + 0xb0, 0);
        } else {
            slot_2(c, VT, 0x98, FAKE);
        }
        // Camera position and look-at point (far enough apart that the
        // placement update never resets the camera).
        put_vec(c, ME + st::POS, [0x447a_0000, 0, 0, 0x3f80_0000]);
        put_vec(c, ME + st::VEL, [0, 0, 0, 0x3f80_0000]);
        // Timer and state flags, chosen so every branch is reached.
        w_5(
            c,
            ME + st::TIMER,
            match seed % 3 {
                0 => 1,  // active path
                1 => 15, // jump spline rebuild
                _ => 3 + seed % 5,
            },
        );
        w_5(c, ME + st::AIRBORNE, ((seed / 3) % 3 == 0) as u32);
        w_5(c, INFO + 0x60, ((seed / 9) % 3 == 0) as u32);
        // The splines are zeroed (never built), so the flags that evaluate
        // them start clear: `cCSICubicSpline_Evaluate` on an empty spline
        // searches forever (0x31e060). They are set, and the curves
        // evaluated, by the rebuild and air-arc paths of the same call.
        w_5(c, ME + st::APPLYING, 0);
        w_5(c, ME + st::LANDING, ((seed / 11) % 2) as u32);
        w_5(c, ME + st::JUMP_READY, 0);
        // The float state.
        for off in [
            0x1f4u32, 0x1f8, 0x1fc, 0x200, 0x20c, 0x210, 0x21c, 0x220, 0x224, 0x228, 0x218,
        ] {
            wf(c, ME + off, r.f32(2.0));
        }
        // The gp_7-relative `.lit4` constants the blend scales by.
        let mut off = 0x6930u32;
        while off <= 0x6aa0 {
            wf(c, GAME_GP.wrapping_sub(off), r.f32(3.0));
            off += 4;
        }
        // The splines' knot times (`AddPoint`'s f12) must ascend (the
        // evaluation's binary search loops otherwise): jump A/B 0, .25, .5,
        // .75|1, 1|1.25, 1.5, 2|1.75, 2.25|2, 2.5, 3, 3.25, 3.5, 4; the
        // landing splines the same scaled by the air arc's f26 > 0.
        for (off, v) in [
            (0x6a98u32, 0.25f32),
            (0x6a94, 0.5),
            (0x6a80, 0.75),
            (0x6a78, 1.0),
            (0x6a70, 1.5),
            (0x6a64, 2.25),
            (0x6a5c, 2.5),
            (0x6a90, 3.25),
            (0x6a4c, 3.5),
            (0x6a34, 1.25),
            (0x6a2c, 1.5),
            (0x6a24, 1.75),
            (0x6a1c, 2.0),
            (0x6a14, 2.5),
            (0x6a04, 3.5),
            (0x69f0, 0.25),
            (0x69e8, 0.5),
            (0x69e0, 0.75),
            (0x69d8, 1.0),
            (0x69d0, 1.5),
            (0x69c4, 2.25),
            (0x69bc, 2.5),
            (0x69b0, 3.25),
            (0x69a8, 3.5),
            (0x6990, 1.25),
            (0x6988, 1.5),
            (0x6980, 1.75),
            (0x6978, 2.0),
            (0x6970, 2.5),
            (0x6960, 3.5),
        ] {
            wf(c, GAME_GP.wrapping_sub(off), v.to_bits());
        }
        // `CheckDistanceReset`'s "too far" limit: never.
        wf(c, GAME_GP.wrapping_sub(0x6778), 0x7f7f_ffff);
        // The four incoming tuning floats.
        c.f[12] = r.f32(1.0);
        c.f[13] = r.f32(1.0);
        c.f[14] = r.f32(1.0);
        c.f[15] = r.f32(1.0);
        c.g[A0][0] = ME as u64;
        c.g[A1][0] = INFO as u64;
    }

    #[test]
    fn apply_jump_curves_matches() {
        check(
            ee::s01::f_001635f8__generated,
            chase_camera_algorithm_apply_jump_curves,
            300,
            &fixture_2,
        );
    }

    /// The takeoff countdown (+0x2c4, 15 at takeoff) counts 60 Hz frames: the
    /// takeoff curves are built 1 frame and the air arc 15 frames (0.25 s)
    /// after takeoff at every rate, each once.
    #[test]
    fn apply_jump_curves_countdown_follows_the_rate() {
        /// Rig slots +0xa8 (the jump progress, asked when the takeoff curves
        /// are built) and +0x60 (asked when the air arc is built).
        const KICK: u32 = FAKE + 0xa0;
        const ARC: u32 = FAKE + 0x60;
        /// The countdown after each tick, and how often each was built.
        fn run(hz: u32) -> (Vec<i32>, u32, u32) {
            let mut boxed = crate::difftest::fresh();
            let c: &mut Cpu = &mut boxed;
            fixture_2(c, 1);
            crate::tick::set_for_test(c, hz);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            slot_2(c, VT, 0x98, LOAD_B0); // no early landing
            w_5(c, RIG + 0xb0, 0);
            slot_2(c, VT, 0xa8, KICK);
            slot_2(c, VT, 0x60, ARC);
            w_5(c, ME + st::TIMER, 15);
            w_5(c, ME + st::AIRBORNE, 0);
            w_5(c, INFO + 0x60, 1);
            let mut timeline = Vec::new();
            for _ in 0..hz / 4 + 8 {
                c.g[A0][0] = ME as u64;
                c.g[A1][0] = INFO as u64;
                c.f[12] = 1.0f32.to_bits();
                c.f[13] = 0.5f32.to_bits();
                // f14 = 0 scales both curve sets to 0: they are not built (the
                // fake rig's progress values would give them arbitrary knots),
                // the rig is still asked on the tick each would be built.
                c.f[14] = 0;
                c.f[15] = 0.5f32.to_bits();
                chase_camera_algorithm_apply_jump_curves(c);
                timeline.push(c.bus.read32(ME + st::TIMER) as i32);
            }
            let calls = c.bus.read32(crate::difftest::fakes::LOG_COUNT);
            let target =
                |c: &mut Cpu, i: u32| c.bus.read32(crate::difftest::fakes::LOG_START + 0x220 * i);
            let kicks = (0..calls).filter(|&i| target(c, i) == KICK).count() as u32;
            let arcs = (0..calls).filter(|&i| target(c, i) == ARC).count() as u32;
            (timeline, kicks, arcs)
        }
        for hz in [60u32, 120, 240] {
            let q = (hz / 60) as usize;
            let (timeline, kicks, arcs) = run(hz);
            assert_eq!(
                timeline.iter().position(|&n| n == 14),
                Some(0),
                "{hz} Hz: {timeline:?}"
            );
            assert_eq!(
                timeline.iter().filter(|&&n| n == 14).count(),
                q,
                "{hz} Hz: {timeline:?}"
            );
            assert_eq!(
                timeline.iter().position(|&n| n == 0),
                Some(14 * q),
                "{hz} Hz: {timeline:?}"
            );
            assert_eq!(
                (kicks, arcs),
                (1, 1),
                "{hz} Hz: takeoff curves / air arc built"
            );
        }
    }

    /// After landing the curve offsets (+0x20c, +0x210, +0x21c, +0x220)
    /// shrink by the vertical blend (+0x224) per update: one 60 Hz update and
    /// `hz/60` updates at `hz` leave the same offsets.
    #[test]
    fn apply_jump_curves_landing_decay_follows_the_rate() {
        fn offset_after(hz: u32, ticks: u32) -> f32 {
            let mut boxed = crate::difftest::fresh();
            let c: &mut Cpu = &mut boxed;
            fixture_2(c, 2);
            crate::tick::set_for_test(c, hz);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            // Straight to the landed state: no curves to build or evaluate.
            w_5(c, INFO + 0x60, 1);
            w_5(c, ME + st::AIRBORNE, 1);
            w_5(c, ME + st::JUMP_READY, 0);
            w_5(c, ME + st::APPLYING, 0);
            w_5(c, ME + st::LANDING, 1);
            wf(c, ME + st::VERTICAL_VEL, 0.9f32.to_bits());
            wf(c, ME + st::JUMP_VEL_X, 1.0f32.to_bits());
            for _ in 0..ticks {
                c.g[A0][0] = ME as u64;
                c.g[A1][0] = INFO as u64;
                chase_camera_algorithm_apply_jump_curves(c);
            }
            f32::from_bits(c.bus.read32(ME + st::JUMP_VEL_X))
        }
        let at_60 = offset_after(60, 1);
        assert!((at_60 - 0.9).abs() < 1e-6, "at 60: {at_60}");
        for hz in [120u32, 240] {
            let fast = offset_after(hz, hz / 60);
            assert!(
                (fast - at_60).abs() < 1e-5,
                "at {hz}: {fast}, at 60: {at_60}"
            );
        }
    }

    #[test]
    fn manual_cam_algo_base_ctor_matches() {
        check(
            ee::s01::f_0015f938__generated,
            manual_cam_algo_base_ctor,
            80,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x15f9, 1);
                fill(c, &mut r, DATA, 0x40);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    // ------------------------------------------------------------------
    // Chase-camera algorithms.

    #[test]
    fn matrix_chase_camera_algorithm_reset_for_rider_matches() {
        check(
            ssx3_ee::s01::f_00160ae8__generated,
            matrix_chase_camera_algorithm_reset_for_rider,
            SEEDS_6,
            &|c, seed| camera_reset_setup(c, seed, 0x160ae8),
        );
    }

    #[test]
    fn look_back_chase_camera_algorithm_reset_for_rider_matches() {
        check(
            ssx3_ee::s02::f_00168940__generated,
            look_back_chase_camera_algorithm_reset_for_rider,
            SEEDS_6,
            &|c, seed| camera_reset_setup(c, seed, 0x168940),
        );
    }

    #[test]
    fn mid_right_chase_camera_algorithm_reset_for_rider_matches() {
        check(
            ssx3_ee::s02::f_00168c50__generated,
            mid_right_chase_camera_algorithm_reset_for_rider,
            SEEDS_6,
            &|c, seed| camera_reset_setup(c, seed, 0x168c50),
        );
    }

    /// Shared fixture of the three camera resets: the algorithm's target
    /// (self+0x30 -> +4 -> +0x6c0) with a matching id, and the controller's
    /// vtable.
    pub(crate) fn camera_reset_setup(c: &mut Cpu, seed: u32, salt: u32) {
        let mut r = machine(c, seed, salt as u64, 4);
        be_library_2(c);
        world_2(c);
        let algo = DATA + 0x100;
        let target = DATA + 0x200;
        let vt = DATA + 0x600;
        let id = r.next_u32();
        c.g[A0][0] = algo as u64;
        c.g[A1][0] = if seed % 3 == 0 {
            id.wrapping_add(1) as u64
        } else {
            id as u64
        };
        w_7(c, algo + 0x30, DATA + 0x300);
        w_7(c, DATA + 0x300 + 4, target);
        w_7(c, target + 0x6c0, vt);
        slot_4(c, vt, 0x38, 0, FAKE);
        slot_4(c, vt, 0x3c, 0, FAKE + 0x10);
        // gp_8-0x66d4 / gp_8-0x6610: the pitch constants the look-back and
        // mid-right variants load.
        w_7(c, GAME_GP.wrapping_sub(0x66d4), 0x3f80_0000);
        w_7(c, GAME_GP.wrapping_sub(0x6610), 0x3f80_0000);
        // The controller's virtual +0x28/+0x2c call.
        let ctl = DATA + 0x700;
        w_7(c, algo + 0x10, ctl);
        slot_4(c, ctl, 0x28, 0, FAKE + 0x20);
        slot_4(c, ctl, 0x2c, 0, FAKE + 0x30);
    }

    #[test]
    fn relative_chase_camera_algorithm_reset_for_rider_matches() {
        // The camera's rider vtable: the original compares the id returned by
        // the first virtual call; force a match on half the seeds.
        check(
            ee::s01::f_00160fd0__generated,
            relative_chase_camera_algorithm_reset_for_rider,
            120,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x16_0fd0);
                globals_7(c);
                const HOLDER: u32 = DATA + 0x1000;
                const BASE: u32 = DATA + 0x2000;
                const VT: u32 = DATA + 0x3000;
                const VT2: u32 = DATA + 0x4000;
                w_8(c, DATA + 0x30, HOLDER);
                w_8(c, HOLDER + 4, BASE);
                w_8(c, BASE + 0x6c0, VT);
                w_8(c, VT + 0x38, 0); // delta
                w_8(c, VT + 0x3c, 0x0000_4000); // fake method
                w_8(c, DATA + 0x10, VT2);
                w_8(c, VT2 + 0x28, 0);
                w_8(c, VT2 + 0x2c, 0x0000_4000);
                let id = (r.next_u32() & 0xffff) as u32;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(id);
                if seed % 2 == 0 {
                    // Make the virtual "current rider" call return this id.
                    w_8(c, crate::difftest::fakes::LOG_RESULTS, id);
                } else {
                    w_8(c, crate::difftest::fakes::LOG_RESULTS, id ^ 0x5a5a);
                }
            },
        );
    }

    #[test]
    fn chase_cam_controller_push_algorithm_matches() {
        check(
            ee::s01::f_00161e58__generated,
            chase_cam_controller_push_algorithm,
            SEEDS,
            &|c, seed| {
                let _ = base_6(c, seed, 0x1e58, 2);
                let this = DATA;
                let list = DATA + 0x1000;
                w_9(c, this + 8, list);
                w_9(c, list + 4, (seed & 1) as u32);
                w_9(c, this + 0x18, 0);
                w_9(c, DATA + 0x1100 + 0xc, seed);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x1100) as u64;
                c.f[12] = if seed & 2 == 0 { 0 } else { 0x3f80_0000 };
            },
        );
    }

    #[test]
    fn chase_cam_controller_begin_script_camera_matches() {
        strict(
            ee::s01::f_00161fa0__generated,
            chase_cam_controller_begin_script_camera,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x161f, 4);
                let this = DATA;
                c.bus.write32(this + 0x30, r.next_u32() & 7);
                c.bus.write32(this + 0x24, r.next_u32());
                c.bus.write32(this + 0x28, r.next_u32());
                c.bus.write32(this + 0x34, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = [0x4cu64, 0x5d, 0x42, 1][(seed % 4) as usize];
            },
        );
    }

    #[test]
    fn game_camera_dtor_matches() {
        check(
            ee::s01::f_0015dd88__generated,
            game_camera_dtor,
            12,
            &|c, seed| {
                let _ = base_7(c, seed, 0x15dd88, 2);
                let this = DATA;
                clear_4(c, this, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                w_10(c, this + 0xa4, if seed % 2 == 0 { 0 } else { DATA + 0x200 });
                w_10(c, this + 0xa8, 0);
                w_10(c, this + 0xac, 0);
                clear_4(c, DATA + 0x200, 0x100);
                w_10(c, DATA + 0x200 + 0x14, 0);
            },
        );
    }

    #[test]
    fn camera_wobble_update_roll_matches() {
        strict_2(
            ee::s01::f_00160130__generated,
            camera_wobble_update_roll,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0016_0130 ^ (seed as u64) << 9);
                let _ = base_8(c, seed, 0x0130, 0);
                let this = DATA + 0x100;
                // a zero input clears the wobble; otherwise the roll chases a
                // random target
                c.f[12] = if seed % 4 == 0 { 0 } else { r.edge_f32() };
                w_11(c, this + 0x24, r.f32(0.5));
                w_11(c, this + 0x2c, r.f32(0.5));
                w_11(c, this + 0x14, r.f32(0.5));
                w_11(c, this + 8, r.f32(0.5));
                c.g[A0][0] = sx(this);
                c.pc = 0x0016_0130;
            },
        );
    }

    /// The test world / race manager / BE_5 singleton live high in `DATA`
    /// (below `DATA + 0x40000`, where the allocator arena starts).
    pub(crate) const W_2: u32 = DATA + 0x10_0000;

    pub(crate) const RM: u32 = DATA + 0x10_0100;

    pub(crate) const BE_5: u32 = DATA + 0x10_2000;

    /// The main heap's dlmalloc arena.
    pub(crate) const MEM_HEAP: u32 = 0x004f_f250;

    pub(crate) fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    pub(crate) fn heap_init(c: &mut Cpu) {
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    /// The engine's size-class pool allocator, enabled through `gp+0xdd8`.
    pub(crate) fn pool_allocator_3(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes;
            for n in 0..16u32 {
                let node = base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// A machine with a plausible game `gp`, the world singleton and a
    /// working pool + dlmalloc allocator.
    pub(crate) fn alloc_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP - 0x1f04, BE_5);
        c.bus.write32(GAME_GP - 0x848, W_2);
        c.bus.write32(W_2 + 0x84, RM);
        c.bus.write32(RM + 0x7c, 0);
        c.bus.write32(RM + 0x64, 0);
        c.bus.write32(RM + 0x84, 0);
        pool_allocator_3(c);
        heap_init(c);
        r
    }

    #[test]
    fn game_camera_ctor_matches() {
        check_all(ee::s01::f_0015dc70, game_camera_ctor, 40, &|c, seed| {
            let _r = alloc_base(c, seed, 0x15dc);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = BE_5 as u64;
        });
    }

    #[test]
    fn camera_wobble_get_offset_matches() {
        check3(
            ee::s01::f_00160228__generated,
            camera_wobble_get_offset,
            60,
            &|c, seed| {
                let mut r = base_9(c, seed, 0x160228);
                let obj = DATA + 0x100;
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(obj);
                let inactive = seed % 3 == 0;
                c.bus
                    .write32(obj + 0x18, if inactive { 0 } else { r.edge_f32() });
                for off in [0u32, 4, 0xc, 0x10, 0x20, 0x28] {
                    c.bus.write32(obj + off, r.f32(10.0));
                }
                c.f[12] = r.f32(3.0);
                c.f[13] = r.f32(3.0);
            },
        );
    }

    #[test]
    fn start_shake_matches() {
        check(
            ssx3_ee::s01::f_001656b0__generated,
            chase_camera_start_shake,
            200,
            &|c, seed| {
                let mut r = Rng(0x0016_56b0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.f32(4.0);
                c.f[13] = r.f32(4.0);
                c.f[14] = r.f32(4.0);
                // The per-frame random amount the shake scales by.
                c.bus
                    .write32((GAME_GP as u32).wrapping_sub(0x68d4), r.f32(1.0));
            },
        );
    }

    /// `GameCamera_AvoidCollision` (0x15ee00) against its generated body.
    mod avoid_collision {
        use super::*;

        const CAM: u32 = 0x0085_0000;
        const CTL: u32 = CAM + 0x1000;
        const WORLD_OBJ: u32 = CAM + 0x2000;
        const GAME: u32 = CAM + 0x2100;
        /// The collision world (`world+0x84 → +0x20`): a spatial key and
        /// one cell with one object.
        const COLL: u32 = CAM + 0x2200;
        const CELL: u32 = CAM + 0x2300;
        const OBJ: u32 = CAM + 0x2400;
        const KIND: u32 = CAM + 0x2500;
        const KIND_VT: u32 = CAM + 0x2600;
        const HIT_BOX: u32 = CAM + 0x2800;
        const MISS_BOX: u32 = CAM + 0x2840;
        /// The vtable `CollSegment_ctor` stores (slot +0x18 is called by
        /// `Nash_FindClosestContact` on a contact): MenuSys_IsActive here.
        const SEG_VT: u32 = 0x0048_e6a8;
        /// `Nash_FindClosestContact`'s contact records (its frame below ours).
        const RECORDS: u32 = SP - 0x400 - 0x2050;
        /// 2^30: on this x/y every offset below 32 rounds away, so the sweep
        /// segments are vertical lines exactly on the cell's upper x/y bound
        /// (`SpatialKey_ClassifyAABB` then reports "inside" and the object
        /// list is tested).
        const BIG: u32 = 0x4e80_0000;

        fn fb(v: f32) -> u32 {
            v.to_bits()
        }

        /// Seeds `% 4 < 2` place the camera on the collision cell (contacts
        /// through the fake object's methods: fake 0/2 its kind tests, 1 its
        /// bounds, 3 the contact count; 4..7 the script camera's query), the
        /// others at random (no contact). The contact record's fraction and
        /// side select the lift correction; the flags and the side axis the
        /// script camera's exits.
        pub(crate) fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x15_ee00, 8);
            for a in (CAM..CAM + 0x2900).step_by(4) {
                c.bus.write32(a, 0);
            }
            let on_cell = seed % 4 < 2;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD_OBJ);
            c.bus.write32(WORLD_OBJ + 0x84, GAME);
            c.bus.write32(GAME + 0x20, COLL);
            // A level-10 key (cells of 1024) whose x/y upper bound is 2^30.
            c.bus.write32(COLL, 10);
            c.bus.write32(COLL + 4, (1 << 20) - 1);
            c.bus.write32(COLL + 8, (1 << 20) - 1);
            c.bus.write32(COLL + 0xc, 0);
            c.bus.write32(COLL + 0x10, CELL);
            c.bus.write32(GAME_GP.wrapping_sub(0x2dfc), 0); // the key's margin
            c.bus.write32(CELL + 0x20, OBJ);
            c.bus.write32(OBJ + 0x8, 0x40);
            c.bus.write32(OBJ + 0xc, KIND);
            c.bus.write32(OBJ + 0x88, 1);
            c.bus.write32(KIND + 0xc, KIND_VT);
            for (i, off) in [0x164u32, 0x16c, 0x134, 0x13c].into_iter().enumerate() {
                c.bus.write16(KIND_VT + off - 4, 0);
                c.bus.write32(KIND_VT + off, FAKE + 0x10 * i as u32);
            }
            c.bus.write16(SEG_VT + 0x18, 0);
            c.bus.write32(SEG_VT + 0x1c, 0x002c_c098);
            let (lo, hi) = (fb(-3e38), fb(3e38));
            put_vec(c, HIT_BOX, [lo, lo, lo, 0]);
            put_vec(c, HIT_BOX + 0x10, [hi, hi, hi, 0]);
            put_vec(c, MISS_BOX, [hi, hi, hi, 0]);
            put_vec(c, MISS_BOX + 0x10, [lo, lo, lo, 0]);
            let script_box = if seed / 16 % 4 == 0 {
                MISS_BOX
            } else {
                HIT_BOX
            };
            for (n, v) in [1, HIT_BOX, 1, 1, 1, script_box, 1, 1]
                .into_iter()
                .enumerate()
            {
                c.bus.write32(LOG_RESULTS + 4 * n as u32, v);
            }

            // The script camera: inactive, its excluded state, or active.
            c.bus.write32(CAM + 0xa8, CTL);
            let (flags, state) = match seed / 2 % 4 {
                0 => (0, 0),
                1 => (2, 0x5d),
                _ => (2, r.next_u32() % 0x80),
            };
            c.bus.write32(CTL + 0x30, flags);
            c.bus.write32(CTL + 0x2c, state);
            c.bus.write32(CAM + 0x4a0, seed / 64 % 2);
            c.bus.write32(CAM + 0x4a4, seed / 128 % 2);
            c.bus.write32(CAM + 0x460, r.f32(50.0)); // the lift
            put_vec(c, CAM + 0x470, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            put_vec(c, CAM + 0x480, [r.next_u32(), 0, 0, 0]);

            // The contact record (fraction +0x40, side +0x18, normal +0x10).
            for i in 0..0x20 {
                c.bus.write32(RECORDS + 4 * i, r.f32(2.0));
            }
            let fraction = match seed / 8 % 5 {
                0 => 0.55f32,
                1 => 0.9,
                2 => 0.45,
                3 => 0.1,
                _ => (r.next_u32() % 1000) as f32 / 1000.0,
            };
            c.bus.write32(RECORDS + 0x40, fraction.to_bits());
            let side = if seed / 40 % 2 == 0 { 0.5f32 } else { -0.5 };
            c.bus.write32(RECORDS + 0x18, side.to_bits());

            if on_cell {
                c.bus.write32(GAME_GP.wrapping_sub(0x6c58), fb(1.0));
                put_vec(c, 0x004f_f160, [0, 0, fb(1.0), 0]); // world up
                let side_axis = match seed / 32 % 3 {
                    0 => [fb(1.0), 0, 0, 0],
                    1 => [0, fb(1.0), 0, 0],
                    _ => [0, 0, 0, 0],
                };
                put_vec(c, 0x004f_f140, side_axis);
                let look = [BIG, BIG, fb(700.0), fb(1.0)];
                put_vec(c, CAM + 0xe0, look);
                put_vec(c, CAM + 0x100, [BIG, BIG, fb(450.0), fb(1.0)]);
                // The last clear eye: on the look-at point, or 250 below it.
                let z = if seed / 24 % 2 == 0 { 700.0 } else { 450.0 };
                put_vec(c, CAM + 0x490, [BIG, BIG, fb(z), fb(1.0)]);
                // A normal with x/y large enough to survive the rounding.
                let n = |r: &mut Rng| if r.next_u32() % 2 == 0 { 0 } else { fb(8.0) };
                let (nx, ny) = (n(&mut r), n(&mut r));
                c.bus.write32(RECORDS + 0x10, nx);
                c.bus.write32(RECORDS + 0x14, ny);
            } else {
                c.bus.write32(GAME_GP.wrapping_sub(0x6c58), fb(0.05));
                put_vec(c, 0x004f_f160, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                put_vec(c, 0x004f_f140, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                put_vec(
                    c,
                    CAM + 0xe0,
                    [r.f32(500.0), r.f32(500.0), r.f32(500.0), fb(1.0)],
                );
                put_vec(
                    c,
                    CAM + 0x100,
                    [r.f32(500.0), r.f32(500.0), r.f32(500.0), fb(1.0)],
                );
                put_vec(
                    c,
                    CAM + 0x490,
                    [r.f32(500.0), r.f32(500.0), r.f32(500.0), fb(1.0)],
                );
            }
            c.bus.write32(GAME_GP.wrapping_sub(0x6c54), fb(0.97));
            c.g[A0][0] = CAM as u64;
        }

        #[test]
        fn avoid_collision_matches() {
            check(
                ee::s01::f_0015ee00__generated,
                game_camera_avoid_collision,
                400,
                &setup,
            );
        }

        /// The lift (+0x460) decays by 0.97 per update: one 60 Hz update and
        /// `hz/60` updates at `hz` leave the same lift (no contact, no
        /// script camera).
        #[test]
        fn avoid_collision_lift_decay_follows_the_rate() {
            fn lift_after(hz: u32, ticks: u32) -> f32 {
                let mut boxed = crate::difftest::fresh();
                let c: &mut Cpu = &mut boxed;
                setup(c, 2); // random geometry (no contact), script inactive
                c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                crate::tick::set_for_test(c, hz);
                c.bus.write32(CAM + 0x460, fb(40.0));
                for _ in 0..ticks {
                    c.g[A0][0] = CAM as u64;
                    game_camera_avoid_collision(c);
                }
                f32::from_bits(c.bus.read32(CAM + 0x460))
            }
            let at_60 = lift_after(60, 1);
            assert!((at_60 - 40.0 * 0.97).abs() < 1e-3, "at 60: {at_60}");
            for hz in [120u32, 240] {
                let fast = lift_after(hz, hz / 60);
                assert!(
                    (fast - at_60).abs() < 1e-3,
                    "at {hz}: {fast}, at 60: {at_60}"
                );
            }
            // Unconverted, two 120 Hz updates would leave 40 · 0.97².
            assert!(lift_after(120, 2) > 40.0 * 0.97 * 0.97 + 0.5);
        }
    }
}
