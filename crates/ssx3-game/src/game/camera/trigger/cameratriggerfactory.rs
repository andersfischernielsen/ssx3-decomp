//! `src/camera/trigger/cameratriggerfactory.cpp`: hand-written replacements (moved from the
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
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::bedbreward::BUILTIN_DELETE_ADDR;
pub(crate) use crate::game::bx::bxstring::gp;
pub(crate) use crate::game::camera::camera::camera_algorithm_ctor;
pub(crate) use crate::game::camera::camera::gp_4 as gp_2;
pub(crate) use crate::game::camera::camera::write_tuning;
pub(crate) use crate::game::dirtysock::tagsunk::arg;
pub(crate) use crate::game::dirtysock::tagsunk::const_reg;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p as p_7;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_2;
use crate::game::misc::vu0_length_vf3;
pub(crate) use crate::game::render::particle::p as p_2;
pub(crate) use crate::game::ui::uivector::p as p_3;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::wscript::wscriptman::p as p_5;
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
    (
        0x0016_2318,
        chase_camera_algorithm_ctor,
        "ChaseCameraAlgorithm_ctor",
    ),
    (
        0x0017_14e8,
        camera_trigger_read_action_type1,
        "CameraTrigger_ReadActionType1",
    ),
    (
        0x0017_15a0,
        camera_trigger_read_action_type0,
        "CameraTrigger_ReadActionType0",
    ),
    (
        0x0017_15f8,
        camera_trigger_read_action_type2,
        "CameraTrigger_ReadActionType2",
    ),
    (
        0x0017_1680,
        camera_trigger_read_action_type3,
        "CameraTrigger_ReadActionType3",
    ),
    (
        0x0017_21c0,
        camera_sphere_trigger_get_bounds,
        "CameraSphereTrigger_GetBounds",
    ),
    (
        0x0017_2278,
        camera_sphere_trigger_contains_point,
        "CameraSphereTrigger_ContainsPoint",
    ),
    (
        0x0017_2840,
        camera_box_trigger_get_bounds,
        "CameraBoxTrigger_GetBounds",
    ),
    (0x0017_31c0, vec3_max_component, "Vec3_MaxComponent"),
    (
        0x0017_3208,
        bounded_camera_algorithm_base_ctor,
        "BoundedCameraAlgorithmBase_ctor",
    ),
    (
        0x0017_3678,
        bounded_camera_algorithm_base_reset,
        "BoundedCameraAlgorithmBase_Reset",
    ),
    (
        0x0017_4190,
        bounded_camera_algorithm_ctor,
        "BoundedCameraAlgorithm_ctor",
    ),
    (
        0x0017_47a0,
        bounded_camera_algorithm_reset,
        "BoundedCameraAlgorithm_Reset",
    ),
    (
        0x0017_4848,
        camera_trigger_man_collect_octree_triggers_in_box,
        "CameraTriggerMan_CollectOctreeTriggersInBox",
    ),
    (
        0x0017_6358,
        script_camera_algo_get_look_at,
        "ScriptCameraAlgo_GetLookAt",
    ),
    (
        0x0017_63f8,
        algorithmic_script_camera_algo_dtor,
        "AlgorithmicScriptCameraAlgo_dtor",
    ),
    (
        0x0017_6808,
        camera_point_bound_obj_clamp_point,
        "CameraPointBoundObj_ClampPoint",
    ),
    (
        0x0017_6840,
        camera_trigger_array_45c1d0_v01,
        "CameraTriggerArray_45c1d0_v01",
    ),
    (
        0x0017_6868,
        camera_trigger_array_45c1b8_v01,
        "CameraTriggerArray_45c1b8_v01",
    ),
    (
        0x0017_6890,
        camera_trigger_man_collect_octree_node_triggers,
        "CameraTriggerMan_CollectOctreeNodeTriggers",
    ),
    (
        0x0017_6a68,
        default_2_chase_camera_algorithm_ctor,
        "Default_2ChaseCameraAlgorithm_ctor",
    ),
    (
        0x0017_6ad0,
        default_2_chase_camera_algorithm_get_eye_drop,
        "Default_2ChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0017_6ae0,
        default_2_chase_camera_algorithm_get_tuning,
        "Default_2ChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0017_6ce0,
        default_2_chase_camera_reset_for_rider,
        "Default_2ChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_6d68,
        default_3_chase_camera_algorithm_ctor,
        "Default_3ChaseCameraAlgorithm_ctor",
    ),
    (
        0x0017_6dd0,
        default_3_chase_camera_algorithm_get_eye_drop,
        "Default_3ChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0017_6de0,
        default_3_chase_camera_algorithm_get_tuning,
        "Default_3ChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0017_6fe0,
        default_3_chase_camera_reset_for_rider,
        "Default_3ChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_7068,
        default_4_chase_camera_algorithm_ctor,
        "Default_4ChaseCameraAlgorithm_ctor",
    ),
    (
        0x0017_70d0,
        default_4_chase_camera_algorithm_get_eye_drop,
        "Default_4ChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0017_70e0,
        default_4_chase_camera_algorithm_get_tuning,
        "Default_4ChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0017_72e0,
        default_4_chase_camera_reset_for_rider,
        "Default_4ChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_74c0,
        direction_ne_chase_camera_algorithm_ctor,
        "DirectionNeChaseCameraAlgorithm_ctor",
    ),
    (
        0x0017_7558,
        direction_ne_chase_camera_algorithm_get_tuning,
        "DirectionNeChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0017_7588,
        direction_ne_chase_camera_reset_for_rider,
        "DirectionNeChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_7e50,
        spoke_chase_camera_algorithm_ctor,
        "SpokeChaseCameraAlgorithm_ctor",
    ),
    (
        0x0017_7eb8,
        spoke_chase_camera_algorithm_get_eye_drop,
        "SpokeChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0017_7ec8,
        spoke_chase_camera_get_tuning,
        "SpokeChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0017_7ef8,
        spoke_chase_camera_algorithm_get_focus_point,
        "SpokeChaseCameraAlgorithm_GetFocusPoint",
    ),
    (
        0x0017_7fc0,
        spoke_chase_camera_algorithm_compute_desired_eye,
        "SpokeChaseCameraAlgorithm_ComputeDesiredEye",
    ),
    (
        0x0017_8208,
        spoke_chase_camera_algorithm_update,
        "SpokeChaseCameraAlgorithm_Update",
    ),
    (
        0x0017_8520,
        superpipe_chase_camera_algorithm_ctor,
        "SuperpipeChaseCameraAlgorithm_ctor",
    ),
    (
        0x0017_85b0,
        superpipe_chase_camera_get_eye_drop,
        "SuperpipeChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0017_85c0,
        superpipe_chase_camera_get_tuning,
        "SuperpipeChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0017_8938,
        superpipe_chase_camera_algorithm_reset_for_rider,
        "SuperpipeChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_8bb0,
        post_race_1_chase_camera_update,
        "Post_Race_1ChaseCameraAlgorithm_Update",
    ),
    (
        0x0017_8e90,
        post_race_1_chase_camera_reset_for_rider,
        "Post_Race_1ChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_9fc8,
        debug_menu_is_track_selectable,
        "DebugMenu_IsTrackSelectable",
    ),
    (
        0x0017_a028,
        debug_menu_next_enabled_item,
        "DebugMenu_NextEnabledItem",
    ),
    (
        0x0017_a0b8,
        debug_menu_prev_enabled_item,
        "DebugMenu_PrevEnabledItem",
    ),
    (
        0x0017_a220,
        debug_menu_find_next_item,
        "DebugMenu_FindNextItem",
    ),
    (
        0x0017_a2c8,
        debug_menu_find_prev_item,
        "DebugMenu_FindPrevItem",
    ),
    (
        0x0017_abc8,
        debug_menu_init_background_list,
        "DebugMenu_InitBackgroundList",
    ),
    (
        0x0017_abe0,
        debug_menu_free_backgrounds,
        "DebugMenu_FreeBackgrounds",
    ),
    (
        0x0017_ac98,
        debug_menu_select_background,
        "DebugMenu_SelectBackground",
    ),
    (
        0x0016_2568,
        chase_camera_algorithm_gather_rider_state,
        "ChaseCameraAlgorithm_GatherRiderState",
    ),
    (
        0x0017_6e10,
        default_3_chase_camera_algorithm_update,
        "Default_3ChaseCameraAlgorithm_Update",
    ),
    (
        0x0017_12b0,
        camera_trigger_read_action_2,
        "CameraTrigger_ReadAction",
    ),
    (
        0x0017_1690,
        camera_trigger_read_bound_obj,
        "CameraTrigger_ReadBoundObj",
    ),
    (
        0x0017_2e18,
        camera_trigger_read_sphere_trigger,
        "CameraTrigger_ReadSphereTrigger",
    ),
    (
        0x0017_2fe0,
        camera_trigger_read_sphere_trigger_data,
        "CameraTrigger_ReadSphereTriggerData",
    ),
    (
        0x0017_1d10,
        camera_trigger_read_point_bound_obj,
        "CameraTrigger_ReadPointBoundObj",
    ),
    (
        0x0017_30d0,
        camera_trigger_read_box_trigger_data_2,
        "CameraTrigger_ReadBoxTriggerData",
    ),
    (
        0x0017_6440,
        camera_bound_obj_get_position,
        "CameraBoundObj_GetPosition",
    ),
    (
        0x0017_6458,
        camera_bound_obj_get_field14,
        "CameraBoundObj_GetField14",
    ),
    (
        0x0017_6460,
        camera_bound_obj_get_scale,
        "CameraBoundObj_GetScale",
    ),
    (
        0x0017_6448,
        camera_bound_obj_get_rot_x,
        "CameraBoundObj_GetRotX",
    ),
    (
        0x0017_6450,
        camera_bound_obj_get_rot_y,
        "CameraBoundObj_GetRotY",
    ),
    (
        0x0017_6da8,
        default_3_chase_camera_algorithm_dtor,
        "Default_3ChaseCameraAlgorithm_dtor",
    ),
    (
        0x0017_6368,
        manual_script_camera_algo_dtor,
        "ManualScriptCameraAlgo_dtor",
    ),
    (
        0x0017_6398,
        target_script_camera_algo_dtor,
        "TargetScriptCameraAlgo_dtor",
    ),
    (
        0x0017_28f8,
        camera_box_trigger_contains_point,
        "CameraBoxTrigger_ContainsPoint",
    ),
    (
        0x0017_32b8,
        bounded_camera_algorithm_base_dtor,
        "BoundedCameraAlgorithmBase_dtor",
    ),
    (
        0x0017_36e0,
        bounded_camera_algorithm_base_update,
        "BoundedCameraAlgorithmBase_Update",
    ),
    (
        0x0017_3e40,
        bounded_camera_algorithm_base_get_view,
        "BoundedCameraAlgorithmBase_GetView",
    ),
    (
        0x0017_41d8,
        bounded_camera_algorithm_dtor,
        "BoundedCameraAlgorithm_dtor",
    ),
    (
        0x0017_4200,
        bounded_camera_algorithm_update,
        "BoundedCameraAlgorithm_Update",
    ),
    (
        0x0017_44b0,
        bounded_camera_algorithm_reset_for_rider,
        "BoundedCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_5a20,
        camera_trigger_man_static_init_destroy,
        "CameraTriggerMan_StaticInitDestroy",
    ),
    (
        0x0017_6270,
        camera_trigger_man_member45bde0_dtor,
        "CameraTriggerMan_Member45bde0_dtor",
    ),
    (
        0x0017_62f0,
        camera_trigger_man_member10_call_v05,
        "CameraTriggerMan_Member10_CallV05",
    ),
    (
        0x0017_6328,
        script_camera_algo_dtor,
        "ScriptCameraAlgo_dtor",
    ),
    (
        0x0017_63c8,
        subject_script_camera_algo_dtor,
        "SubjectScriptCameraAlgo_dtor",
    ),
    (
        0x0017_6a28,
        camera_trigger_man_global_ctors,
        "CameraTriggerMan_GlobalCtors",
    ),
    (
        0x0017_6a48,
        camera_trigger_man_global_dtors,
        "CameraTriggerMan_GlobalDtors",
    ),
    (
        0x0017_6aa8,
        default_2_chase_camera_algorithm_dtor,
        "Default_2ChaseCameraAlgorithm_dtor",
    ),
    (
        0x0017_6b10,
        default_2_chase_camera_algorithm_update,
        "Default_2ChaseCameraAlgorithm_Update",
    ),
    (
        0x0017_70a8,
        default_4_chase_camera_algorithm_dtor,
        "Default_4ChaseCameraAlgorithm_dtor",
    ),
    (
        0x0017_7110,
        default_4_chase_camera_algorithm_update,
        "Default_4ChaseCameraAlgorithm_Update",
    ),
    (
        0x0017_7368,
        direction_ne_chase_camera_algorithm_update,
        "DirectionNeChaseCameraAlgorithm_Update",
    ),
    (
        0x0017_7520,
        direction_ne_chase_camera_algorithm_dtor,
        "DirectionNeChaseCameraAlgorithm_dtor",
    ),
    (
        0x0017_7650,
        direction_ne_chase_static_init,
        "DirectionNeChaseCameraAlgorithm_StaticInit",
    ),
    (
        0x0017_7e30,
        direction_ne_chase_camera_algorithm_global_ctor,
        "DirectionNeChaseCameraAlgorithm_GlobalCtor",
    ),
    (
        0x0017_7e90,
        spoke_chase_camera_algorithm_dtor,
        "SpokeChaseCameraAlgorithm_dtor",
    ),
    (
        0x0017_83e0,
        spoke_chase_camera_algorithm_reset_for_rider,
        "SpokeChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_8588,
        superpipe_chase_camera_algorithm_dtor,
        "SuperpipeChaseCameraAlgorithm_dtor",
    ),
    (
        0x0017_85f0,
        superpipe_chase_camera_compute_desired_eye,
        "SuperpipeChaseCameraAlgorithm_ComputeDesiredEye",
    ),
    (
        0x0017_8758,
        superpipe_chase_camera_compute_look_at,
        "SuperpipeChaseCameraAlgorithm_ComputeLookAt",
    ),
    (
        0x0017_87e0,
        superpipe_chase_camera_update,
        "SuperpipeChaseCameraAlgorithm_Update",
    ),
    (
        0x0017_89e8,
        post_race_1_chase_camera_ctor,
        "Post_Race_1ChaseCameraAlgorithm_ctor",
    ),
    (
        0x0017_8b70,
        post_race_1_chase_camera_dtor,
        "Post_Race_1ChaseCameraAlgorithm_dtor",
    ),
    (
        0x0017_8f58,
        post_race_1chase_camera_algorithm_static_init,
        "Post_Race_1ChaseCameraAlgorithm_StaticInit",
    ),
    (
        0x0017_9738,
        post_race_1_chase_camera_global_ctor,
        "Post_Race_1ChaseCameraAlgorithm_GlobalCtor",
    ),
    (
        0x0017_9798,
        camera_consts_static_init_9798,
        "CameraConsts_StaticInit_9798",
    ),
    (
        0x0017_9fa8,
        camera_consts_global_ctor_9fa8,
        "CameraConsts_GlobalCtor_9fa8",
    ),
    (0x001d_cd00, debug_menu_v01, "DebugMenu_v01"),
    (
        0x0017_1a20,
        camera_ellipse_bound_obj_read,
        "CameraEllipseBoundObj_Read",
    ),
    (
        0x0017_1b10,
        camera_box_bound_obj_read_2,
        "CameraBoxBoundObj_Read",
    ),
    (
        0x0017_1c00,
        camera_line_bound_obj_read,
        "CameraLineBoundObj_Read",
    ),
    (
        0x0017_1d58,
        camera_trigger_read_spline,
        "CameraTrigger_ReadSpline",
    ),
    (
        0x0017_a830,
        debug_menu_add_world_if_missing,
        "DebugMenu_AddWorldIfMissing",
    ),
    (
        0x0017_1fa8,
        oc680_camera_trigger_find_boundary_point,
        "CameraTrigger_FindBoundaryPoint",
    ),
    (
        0x0017_32e8,
        oc680_bounded_camera_algorithm_base_reset_for_rider,
        "BoundedCameraAlgorithmBase_ResetForRider",
    ),
    (
        0x0017_a158,
        debug_menu_dispatch_item_op,
        "DebugMenu_DispatchItemOp",
    ),
    (
        0x0017_a638,
        debug_menu_init_world_list,
        "DebugMenu_InitWorldList",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::bx::bxstringctor::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstringctor::callee::CHASE_CAMERA_RESET_FOR_RIDER;

    pub(crate) use crate::game::bx::bxstringctor::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstringctor::callee::SUPERPIPE_COMPUTE_LOOK_AT;

    pub(crate) use crate::game::camera::camera::callee::CHASE_CAMERA_ALGORITHM_CTOR;

    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::OPERATOR_DELETE;

    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;

    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;

    pub(crate) use crate::game::fe::fepopup::callee::C0016C6B0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016C718;

    pub(crate) use crate::game::fe::fepopup::callee::C00176890;

    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE;

    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;

    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;

    pub(crate) use crate::game::render::particle::callee::STRLEN;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_CAMERA_ALGORITHM_CTOR as RIDER_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE0;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE1;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TYPE3;

    pub const MATH_SIN_COS: Callee = Callee {
        addr: 0x0031_be50,
        f: ee::s10::Math_SinCos,
    };

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

    pub const SPOKE_CHASE_GET_FOCUS_POINT: Callee = Callee {
        addr: 0x0017_7ef8,
        f: ee::s02::f_00177ef8,
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

    pub const BE_MISSION_SET_COLLECTED: u32 = 0x0015_3b00;

    pub const MSG_PRINTF: u32 = 0x0041_7828;

    pub const QSORT: u32 = 0x0041_8ef8;

    pub const EA_TIMER_SERVICE: u32 = 0x003e_5928;

    pub const EA_THREAD_YIELD: u32 = 0x003e_5398;

    pub const GAME_MODULE_REQUEST_STATE_2: u32 = 0x0023_1250;

    pub const LOBBY_API_SEND: u32 = 0x003e_8b10;

    pub const SSX_AUDIO_RESUME_SFX: u32 = 0x0029_ce70;

    pub const SSX_AUDIO_IS_TRACK_GROUP5: u32 = 0x002a_3f90;

    pub const RIDER_GATHER_CAM_STATE: u32 = 0x0027_c9b0;

    pub const MEM_REPORT_STATS_LOCKED: u32 = 0x0031_9cc8;

    pub const OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;

    pub const EA_MEM_ALLOC_NAMED_2: u32 = 0x0031_7d70;

    pub const UI_LIST_FIRST_WRAPPER: u32 = 0x0039_fae8;

    pub const SERIALIZE_READ_U32: u32 = 0x002d_2510;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {

    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
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
    rot_axis(
        c,
        true,
        sp.wrapping_add(0x40),
        trigger,
        0xc,
        0x004f_f160,
        sp.wrapping_add(0xd0),
        sp.wrapping_add(0xd4),
        0x0017_2398,
    )?;
    rot_axis(
        c,
        true,
        sp.wrapping_add(0x40),
        trigger,
        0x14,
        0x004f_f150,
        sp.wrapping_add(0xd8),
        sp.wrapping_add(0xdc),
        0x0017_24e0,
    )?;
    rot_axis(
        c,
        true,
        sp.wrapping_add(0x40),
        trigger,
        0x10,
        0x004f_f140,
        sp.wrapping_add(0xe0),
        sp.wrapping_add(0xe4),
        0x0017_2628,
    )?;

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
    camera_algorithm_ctor(
        c,
        callee::CHASE_CAMERA_ALGORITHM_CTOR,
        0x3c,
        0x0045_cab0,
        0x0017_6a7c,
    );
}

/// `Default_3ChaseCameraAlgorithm* ctor(this)` (0x176d68): tag 0x3d, vtable
/// `D_0045CA38`.
pub fn default_3_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(
        c,
        callee::CHASE_CAMERA_ALGORITHM_CTOR,
        0x3d,
        0x0045_ca38,
        0x0017_6d7c,
    );
}

/// `Default_4ChaseCameraAlgorithm* ctor(this)` (0x177068): tag 0x3e, vtable
/// `D_0045C9C0`.
pub fn default_4_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(
        c,
        callee::CHASE_CAMERA_ALGORITHM_CTOR,
        0x3e,
        0x0045_c9c0,
        0x0017_707c,
    );
}

/// `SpokeChaseCameraAlgorithm* ctor(this)` (0x177e50): tag 0x42, vtable
/// `D_0045CCC8`.
pub fn spoke_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(
        c,
        callee::CHASE_CAMERA_ALGORITHM_CTOR,
        0x42,
        0x0045_ccc8,
        0x0017_7e64,
    );
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
        call_ptr(
            c,
            callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX,
            0x0017_9ff4,
        )?;
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
        for (f, off) in [
            (13, 0x5d78u32),
            (14, 0x5d74),
            (15, 0x5d70),
            (16, 0x5d6c),
            (17, 0x5d68),
            (18, 0x5d64),
            (19, 0x5d60),
        ] {
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
            let share =
                FpuF32(c.f[0]) * FpuF32((POST_RACE_ORBIT_FRAMES as f32 / n as f32).to_bits());
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
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lq(3, c.g[A0][0] as u32);
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
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
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
                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
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
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0017_8970)?;
        if c.g[S1][0] != c.g[V0][0] {
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.cycles += 24;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            callee::SUPERPIPE_COMPUTE_DESIRED_EYE.call(c, 0x0017_8984)?;
            c.lq(2, c.g[SP][0] as u32);
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x40));
            callee::SUPERPIPE_COMPUTE_LOOK_AT.call(c, 0x0017_8998)?;
            c.lq(2, c.g[SP][0] as u32);
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
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
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
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
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
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    if call_ptr(c, t, 0x00177f30).is_err() {
        return;
    }
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5e0c));
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.mfc1(3, 0);
    c.lqc2(4, c.g[V0][0] as u32);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
    c.sq(2, c.g[SP][0] as u32);
    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    let t = c.g[V0][0] as u32;
    c.g[RA][0] = sx(0x00177f6c);
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
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
    c.lqc2(3, c.g[SP][0] as u32);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(3, c.g[S1][0] as u32);
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    c.pc = t;
    return;
}

/// `Vec4* SpokeChaseCameraAlgorithm_ComputeDesiredEye(out, this, dir)`
/// (0x177fc0): the spoke camera's eye. The direction is the target's
/// `+0x50` vector while the rider handplants (main state 5), else `*dir`;
/// it is normalized, or, shorter than 0.01, replaced as it is by the
/// target's `+0x20` vector. The heading (`this + 0x390`) becomes the
/// normalized `0.96 * heading + 0.04 * direction` (the direction itself when
/// that is shorter than 0.02), and the eye sits 30 above the focus point
/// (the up vector at 0x4ff160) and 250 behind it along the heading. The
/// blend's two shares are a [`crate::tick::PER_TICK`] entry.
pub fn spoke_chase_camera_algorithm_compute_desired_eye(c: &mut Cpu) {
    use crate::game::camera::camera::chase;
    const RIG_RIDER: u32 = 4;
    const SPOKE_HEADING: u32 = 0x390;
    const GET_HANDPLANT_DIRECTION: u32 = 0x50;
    const GET_FORWARD: u32 = 0x20;
    const DIRECTION: u32 = 0;
    const HEADING: u32 = 0x10;
    const SAVED_RA: u32 = 0x80;
    const SAVED_OUTPUT: u32 = 0x90;
    const SAVED_DIRECTION: u32 = 0xa0;
    const SAVED_CAMERA: u32 = 0xb0;
    /// gp-0x5e04: 0.01, direction length below which the rig supplies a fallback.
    const GP_MIN_DIRECTION: u32 = 0x5e04;
    /// gp-0x5e00: 0.96, retained heading share at 60 Hz.
    const GP_HEADING_RETAINED: u32 = 0x5e00;
    /// gp-0x5dfc: 0.04000002, incoming direction share at 60 Hz.
    const GP_HEADING_INCOMING: u32 = 0x5dfc;
    /// gp-0x5df8: 0.02, blended heading length below which direction wins.
    const GP_MIN_HEADING: u32 = 0x5df8;
    // Stack +0x20..+0x70: reused VU vector-expression spills; preserve every store.
    c.cycles += 14;
    let frame = Frame::open(c, 0xc0);
    frame.save(c, S0, SAVED_CAMERA);
    frame.save(c, S1, SAVED_DIRECTION);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S2, SAVED_OUTPUT);
    c.g[S1][0] = c.g[A2][0];
    frame.save_ra(c, SAVED_RA);
    c.g[S2][0] = c.g[A0][0];
    // The rider's main state.
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(chase::RIG)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(RIG_RIDER)).lw(c);
    if call_ptr(c, callee::RIDER_GET_MAIN_STATE.addr, 0x0017_7fec).is_err() {
        return;
    }
    c.g[V1][0] = 0x5;
    if c.g[V0][0] != c.g[V1][0] {
        c.lq(2, c.g[S1][0] as u32);
        c.cycles += 1;
        c.sq(2, frame.at(c, DIRECTION));
    } else {
        // Handplant: the target's +0x50 vector.
        c.cycles += 9;
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(chase::RIG)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(GET_HANDPLANT_DIRECTION)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(GET_HANDPLANT_DIRECTION + 4)).lw(c);
        let target_method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        if call_ptr(c, target_method, 0x0017_8010).is_err() {
            return;
        }
        c.lq(3, c.g[V0][0] as u32);
        c.sq(3, frame.at(c, DIRECTION));
    }
    // Normalize the direction, or take the target's +0x20 vector.
    c.cycles += 15;
    c.lwc1(0, r32(c, GP).wrapping_sub(GP_MIN_DIRECTION));
    c.lqc2(3, frame.at(c, DIRECTION));
    vu0_length_vf3(c);
    c.cfc2(2, 22);
    c.mtc1(2, 1);
    c.c_lt_s(1, 0);
    let direction_has_length = !c.fcc();
    c.g[A0][0] = sx(frame.at(c, HEADING));
    if direction_has_length {
        c.cycles += 7;
        c.mfc1(2, 1);
        c.qmtc2(2, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, frame.at(c, DIRECTION));
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, frame.at(c, DIRECTION));
    } else {
        c.cycles += 9;
        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(chase::RIG)).lw(c);
        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
        c.g[A1][0] = Addr(r32(c, V0).wrapping_add(GET_FORWARD)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(GET_FORWARD + 4)).lw(c);
        let target_method = r32(c, V1);
        c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
        if call_ptr(c, target_method, 0x0017_8074).is_err() {
            return;
        }
        c.lq(2, frame.at(c, HEADING));
        c.sq(2, frame.at(c, DIRECTION));
    }
    // heading = 0.96 * heading + 0.04 * direction, normalized.
    c.cycles += 37;
    c.lwc1(0, r32(c, GP).wrapping_sub(GP_HEADING_RETAINED));
    c.mfc1(2, 0);
    c.lqc2(4, r32(c, S0).wrapping_add(SPOKE_HEADING));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x30));
    c.lwc1(1, r32(c, GP).wrapping_sub(GP_HEADING_INCOMING));
    c.lq(2, frame.at(c, 0x30));
    c.lwc1(0, r32(c, GP).wrapping_sub(GP_MIN_HEADING));
    c.sq(2, frame.at(c, 0x20));
    c.mfc1(3, 1);
    c.lqc2(4, frame.at(c, DIRECTION));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x40));
    c.lq(2, frame.at(c, 0x40));
    c.sq(2, frame.at(c, 0x30));
    c.lqc2(3, frame.at(c, 0x20));
    c.lqc2(4, frame.at(c, 0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x40));
    c.lq(2, frame.at(c, 0x40));
    c.sq(2, frame.at(c, HEADING));
    c.lqc2(3, frame.at(c, HEADING));
    vu0_length_vf3(c);
    c.cfc2(3, 22);
    c.mtc1(3, 1);
    c.c_lt_s(1, 0);
    let heading_has_length = !c.fcc();
    c.lq(2, frame.at(c, DIRECTION));
    if heading_has_length {
        c.cycles += 7;
        c.mfc1(2, 1);
        c.qmtc2(2, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, frame.at(c, HEADING));
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, frame.at(c, HEADING));
    } else {
        c.cycles += 2;
        c.sq(2, frame.at(c, HEADING));
    }
    // eye = focus + 30 * up - 250 * heading.
    c.cycles += 45;
    c.lq(2, frame.at(c, HEADING));
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = sx(frame.at(c, 0x40));
    c.sq(2, r32(c, S0).wrapping_add(SPOKE_HEADING));
    if callee::SPOKE_CHASE_GET_FOCUS_POINT
        .call(c, 0x0017_8168)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = 0x50_0000;
    c.g[AT][0] = 0x41f0_0000;
    c.mtc1(1, 0);
    // 0x4ff160: world up; preserve the original split address construction.
    c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0xea0));
    c.g[AT][0] = 0x437a_0000;
    c.mtc1(1, 1);
    c.mfc1(3, 0);
    c.lqc2(4, c.g[A0][0] as u32);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x60));
    c.g[V0][0] = c.g[S2][0];
    c.lq(3, frame.at(c, 0x60));
    frame.restore(c, S0, SAVED_CAMERA);
    c.sq(3, frame.at(c, 0x50));
    c.lqc2(3, frame.at(c, 0x40));
    c.lqc2(4, frame.at(c, 0x50));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x60));
    frame.restore(c, S1, SAVED_DIRECTION);
    c.lq(3, frame.at(c, 0x60));
    frame.restore_ra(c, SAVED_RA);
    c.sq(3, frame.at(c, 0x30));
    c.mfc1(4, 1);
    c.lqc2(4, frame.at(c, HEADING));
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x70));
    c.lq(3, frame.at(c, 0x70));
    c.sq(3, frame.at(c, 0x60));
    c.lqc2(3, frame.at(c, 0x30));
    c.lqc2(4, frame.at(c, 0x60));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x70));
    c.lq(3, frame.at(c, 0x70));
    c.sq(3, c.g[S2][0] as u32);
    frame.restore(c, S2, SAVED_OUTPUT);
    c.sq(3, frame.at(c, 0x20));
    frame.close(c);
}

/// `void SpokeChaseCameraAlgorithm_Update(Alg* self)` (0x178208):
/// gather the rider state and blend the eye toward the spoke camera's desired
/// eye. Retention falls with the focus-to-eye distance and is square-rooted
/// in handplant state 5. The focus is the target's focus point plus 37 units
/// along the world up vector. At other rates the retained share is raised to
/// the power 60/hz, so the eye keeps as much per 1/60 s as at 60 Hz.
pub fn spoke_chase_camera_algorithm_update(c: &mut Cpu) {
    use crate::game::camera::camera::{chase, chase_info};
    const RIG_RIDER: u32 = 4;
    const FOCUS: u32 = 0x20;
    const EYE: u32 = 0x40;
    const INFO_DIRECTION: u32 = chase_info::DIR;
    const SAVED_RA: u32 = 0xc0;
    const SAVED_CAMERA: u32 = 0xd0;
    const SAVED_RETENTION: u32 = 0xe0;
    const SAVED_ONE: u32 = 0xe8;
    // Stack +0x70..+0xb0: reused vector-expression and call-result spills.
    /// gp-0x5df4: 0.27136135, GatherRiderState f12 tuning argument.
    const GP_GATHER_F12: u32 = 0x5df4;
    /// gp-0x5df0: 0.9, GatherRiderState f13 tuning argument.
    const GP_GATHER_F13: u32 = 0x5df0;
    /// gp-0x5dec: 0.29420722, GatherRiderState f14 tuning argument.
    const GP_GATHER_F14: u32 = 0x5dec;
    /// gp-0x5de8: 0.851393, GatherRiderState f15 tuning argument.
    const GP_GATHER_F15: u32 = 0x5de8;
    /// gp-0x5de4: 0.85, GatherRiderState f16 tuning argument.
    const GP_GATHER_F16: u32 = 0x5de4;
    /// gp-0x5de0: 0.97, GatherRiderState f17 tuning argument.
    const GP_GATHER_F17: u32 = 0x5de0;
    /// gp-0x5ddc: 0.1, GatherRiderState f18 tuning argument.
    const GP_GATHER_F18: u32 = 0x5ddc;
    /// gp-0x5dd8: 0.6, GatherRiderState f19 tuning argument.
    const GP_GATHER_F19: u32 = 0x5dd8;
    /// gp-0x5dd4: pi/4, camera field of view.
    const GP_FOV: u32 = 0x5dd4;
    /// gp-0x5dd0: 1500, focus-to-eye distance where retention reaches zero.
    const GP_FAR_DISTANCE: u32 = 0x5dd0;
    /// gp-0x5dcc: 1350, distance interval above the 150-unit near threshold.
    const GP_DISTANCE_SPAN: u32 = 0x5dcc;
    /// gp-0x5dc8: 0.988, maximum retained eye share at 60 Hz.
    const GP_EYE_RETENTION: u32 = 0x5dc8;
    c.cycles += 39;
    let frame = Frame::open(c, 0xf0);
    c.lwc1(12, r32(c, GP).wrapping_sub(GP_GATHER_F12));
    frame.save(c, S0, SAVED_CAMERA);
    c.g[A1][0] = c.g[SP][0];
    c.g[S0][0] = c.g[A0][0];
    c.lwc1(13, r32(c, GP).wrapping_sub(GP_GATHER_F13));
    c.lwc1(14, r32(c, GP).wrapping_sub(GP_GATHER_F14));
    c.lwc1(15, r32(c, GP).wrapping_sub(GP_GATHER_F15));
    c.lwc1(16, r32(c, GP).wrapping_sub(GP_GATHER_F16));
    c.lwc1(17, r32(c, GP).wrapping_sub(GP_GATHER_F17));
    c.lwc1(18, r32(c, GP).wrapping_sub(GP_GATHER_F18));
    c.lwc1(19, r32(c, GP).wrapping_sub(GP_GATHER_F19));
    frame.save_ra(c, SAVED_RA);
    frame.save_fpr(c, 21, SAVED_ONE);
    c.g[RA][0] = sx(0x0017_8248);
    frame.save_fpr(c, 20, SAVED_RETENTION);
    if call_ptr(c, 0x0016_2568, 0x0017_8248).is_err() {
        return;
    }
    c.lwc1(0, r32(c, GP).wrapping_sub(GP_FOV));
    c.g[AT][0] = 0x4316_0000;
    c.mtc1(1, 2);
    c.swc1(0, c.g[S0][0] as u32);
    c.lqc2(3, r32(c, S0).wrapping_add(FOCUS));
    c.lqc2(4, r32(c, S0).wrapping_add(EYE));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x80));
    c.lq(2, frame.at(c, 0x80));
    c.sq(2, frame.at(c, 0x70));
    c.lqc2(3, frame.at(c, 0x70));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    c.mtc1(3, 1);
    c.c_le_s(2, 1);
    // Distance below 150 retains the full tuned share. Between 150 and
    // the far threshold it falls linearly; beyond the threshold it is zero.
    if !c.fcc() {
        c.cycles += 1;
        c.mtc1(0, 1);
    } else {
        c.cycles += 4;
        c.lwc1(0, r32(c, GP).wrapping_sub(GP_FAR_DISTANCE));
        c.c_le_s(1, 0);
        if c.fcc() {
            c.cycles += 5;
            c.sub_s(1, 1, 2);
            c.lwc1(0, r32(c, GP).wrapping_sub(GP_DISTANCE_SPAN));
            c.div_s(1, 1, 0);
        } else {
            c.cycles += 4;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
        }
    }
    // Handplant (state 5) retains the square root of the effective share.
    c.cycles += 11;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 21);
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(chase::RIG)).lw(c);
    c.sub_s(1, 21, 1);
    c.lwc1(0, r32(c, GP).wrapping_sub(GP_EYE_RETENTION));
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(RIG_RIDER)).lw(c);
    c.g[RA][0] = sx(0x0017_82fc);
    c.mul_s(20, 1, 0);
    if call_ptr(c, callee::RIDER_GET_MAIN_STATE.addr, 0x0017_82fc).is_err() {
        return;
    }
    c.g[V1][0] = 0x5;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 1;
        c.sqrt_s(20, 20);
    }
    // Distance and handplant state shape the effective retained share first.
    // Keep the original FPU operations at 60 Hz, and preserve that share per
    // 1/60 second at other rates.
    let hz = crate::tick::rate(c);
    if hz != 60 {
        c.f[20] = f32::from_bits(c.f[20]).powf(60.0 / hz as f32).to_bits();
    }
    // Blend the old eye with ComputeDesiredEye, then raise the focus 37 units.
    c.cycles += 53;
    c.mfc1(2, 20);
    c.lqc2(4, r32(c, S0).wrapping_add(EYE));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0x90));
    c.g[A0][0] = sx(frame.at(c, 0xa0));
    c.lq(2, frame.at(c, 0x90));
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = sx(frame.at(c, INFO_DIRECTION));
    c.sub_s(20, 21, 20);
    c.g[RA][0] = sx(0x0017_833c);
    c.sq(2, frame.at(c, 0x80));
    if call_ptr(c, 0x0017_7fc0, 0x0017_833c).is_err() {
        return;
    }
    c.mfc1(2, 20);
    c.lqc2(4, frame.at(c, 0xa0));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0xb0));
    c.g[A1][0] = c.g[S0][0];
    c.lq(2, frame.at(c, 0xb0));
    c.g[A0][0] = sx(frame.at(c, 0x80));
    c.sq(2, frame.at(c, 0x90));
    c.lqc2(3, frame.at(c, 0x80));
    c.lqc2(4, frame.at(c, 0x90));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0xb0));
    c.lq(2, frame.at(c, 0xb0));
    c.sq(2, r32(c, S0).wrapping_add(EYE));
    c.g[RA][0] = sx(0x0017_8380);
    c.sq(2, frame.at(c, 0x70));
    if call_ptr(c, callee::SPOKE_CHASE_GET_FOCUS_POINT.addr, 0x0017_8380).is_err() {
        return;
    }
    c.g[V0][0] = 0x0050_0000;
    c.g[AT][0] = 0x4214_0000;
    c.mtc1(1, 0);
    // 0x4ff160: world up.
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xea0));
    frame.restore_ra(c, SAVED_RA);
    c.mfc1(3, 0);
    c.lqc2(4, c.g[V0][0] as u32);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0xa0));
    frame.restore_fpr(c, 21, SAVED_ONE);
    c.lq(2, frame.at(c, 0xa0));
    frame.restore_fpr(c, 20, SAVED_RETENTION);
    c.sq(2, frame.at(c, 0x90));
    c.lqc2(3, frame.at(c, 0x80));
    c.lqc2(4, frame.at(c, 0x90));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0xa0));
    c.lq(2, frame.at(c, 0xa0));
    c.sq(2, r32(c, S0).wrapping_add(FOCUS));
    frame.restore(c, S0, SAVED_CAMERA);
    c.sq(2, frame.at(c, 0x70));
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
pub(crate) fn p_8(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
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
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
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
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
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
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    }
    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
                0x0017_1b48,
                0x0017_1b54,
                0x0017_1b6c,
                0x0017_1b78,
                0x0017_1b90,
                0x0017_1b9c,
                0x0017_1bb4,
                0x0017_1bc0,
                0x0017_1bd8,
                0x0017_1be4,
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
                0x0017_3108,
                0x0017_3114,
                0x0017_312c,
                0x0017_3138,
                0x0017_3150,
                0x0017_315c,
                0x0017_3174,
                0x0017_3180,
                0x0017_3198,
                0x0017_31a4,
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

/// `sw` of a value into a frame slot.
macro_rules! store_slot {
    ($c:ident, $f:ident, $off:expr, $v:expr) => {{
        let a = $f.at($c, $off);
        let v: u64 = $v;
        $c.sw(a, v);
    }};
}

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
use crate::abi::ret;
use crate::events::back_edge_unwinds;
pub(crate) use crate::game::camera::camera::vu0_dot3;
pub(crate) use crate::game::camera::camera::vu0_length;
pub(crate) use crate::game::fe::debugfe::menu_sx_add;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_add;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_lh;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_lw;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_sw0;
pub(crate) use crate::game::sound::bankmanager::fpr;

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
        0x0017_4da0,
        0x0017_4db4,
        0x0017_4dc8,
        0x0017_4ddc,
        0x0017_4df0,
        0x0017_4e04,
        0x0017_4e18,
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
        0x0017_4f48,
        0x0017_4f5c,
        0x0017_4f70,
        0x0017_4f84,
        0x0017_4f98,
        0x0017_4fac,
        0x0017_4fc0,
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
        0x0017_50f0,
        0x0017_5104,
        0x0017_5118,
        0x0017_512c,
        0x0017_5140,
        0x0017_5154,
        0x0017_5168,
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
        0x0017_5298,
        0x0017_52ac,
        0x0017_52c0,
        0x0017_52d4,
        0x0017_52e8,
        0x0017_52fc,
        0x0017_5310,
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
        0x0017_5440,
        0x0017_5454,
        0x0017_5468,
        0x0017_547c,
        0x0017_5490,
        0x0017_54a4,
        0x0017_54b8,
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
        0x0017_55e8,
        0x0017_55fc,
        0x0017_5610,
        0x0017_5624,
        0x0017_5638,
        0x0017_564c,
        0x0017_5660,
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
        0x0017_5790,
        0x0017_57a4,
        0x0017_57b8,
        0x0017_57cc,
        0x0017_57e0,
        0x0017_57f4,
        0x0017_5808,
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
        0x0017_593c,
        0x0017_5950,
        0x0017_5964,
        0x0017_5978,
        0x0017_598c,
        0x0017_59a0,
        0x0017_59b4,
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

            {
                let mut skip = false;
                child_case!(
                    c,
                    frame,
                    skip,
                    CELL_W,
                    1,
                    0x04,
                    U_LO,
                    V_LO,
                    W_HI,
                    U_LO_IN,
                    V_LO_IN,
                    W_HI_IN,
                    SITE_B,
                    2,
                    false,
                    FP,
                    Finish::LoadV0(CELL_V)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_V);
                }
            }

            {
                let mut skip = false;
                child_case!(
                    c,
                    frame,
                    skip,
                    CELL_V,
                    1,
                    0x0c,
                    U_LO,
                    V_HI,
                    W_HI,
                    U_LO_IN,
                    V_HI_IN,
                    W_HI_IN,
                    SITE_C,
                    2,
                    false,
                    FP,
                    Finish::LoadV0(CELL_W)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_W);
                }
            }

            {
                let mut skip = false;
                child_case!(
                    c,
                    frame,
                    skip,
                    CELL_W,
                    0xffff_ffff,
                    0x08,
                    U_LO,
                    V_HI,
                    W_LO,
                    U_LO_IN,
                    V_HI_IN,
                    W_LO_IN,
                    SITE_D,
                    2,
                    false,
                    FP,
                    Finish::LoadV0(CELL_U)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_U);
                }
            }

            {
                let mut skip = false;
                child_case!(
                    c,
                    frame,
                    skip,
                    CELL_U,
                    1,
                    0x18,
                    U_HI,
                    V_HI,
                    W_LO,
                    U_HI_IN,
                    V_HI_IN,
                    W_LO_IN,
                    SITE_E,
                    2,
                    false,
                    FP,
                    Finish::LoadV0(CELL_W)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_W);
                }
            }

            {
                let mut skip = false;
                child_case!(
                    c,
                    frame,
                    skip,
                    CELL_W,
                    1,
                    0x1c,
                    U_HI,
                    V_HI,
                    W_HI,
                    U_HI_IN,
                    V_HI_IN,
                    W_HI_IN,
                    SITE_F,
                    2,
                    false,
                    FP,
                    Finish::LoadV0(CELL_V)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_V);
                }
            }

            {
                let mut skip = false;
                child_case!(
                    c,
                    frame,
                    skip,
                    CELL_V,
                    0xffff_ffff,
                    0x14,
                    U_HI,
                    V_LO,
                    W_HI,
                    U_HI_IN,
                    V_LO_IN,
                    W_HI_IN,
                    SITE_G,
                    2,
                    false,
                    FP,
                    Finish::LoadV0(CELL_W)
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
                c,
                frame,
                skip,
                CELL_W,
                0xffff_ffff,
                0x10,
                U_HI,
                V_LO,
                W_LO,
                U_HI_IN,
                V_LO_IN,
                W_LO_IN,
                SITE_H,
                4,
                true,
                S6,
                Finish::RestoreS0
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
                                    if callee::C0016C6B0.call(c, 0x00176934).is_err() {
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
                                if callee::C0016C718.call(c, 0x0017694c).is_err() {
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

/// Call virtual slot `slot` of the camera target (`self + 0x30`) with
/// `this` in `this_reg` (the other argument register keeps what the caller
/// set): `v0`/`v1` hold the vtable and function as the original loads them
/// (`vt_reg`, `fn_reg`).
#[inline]
pub(crate) fn target_vcall(
    c: &mut Cpu,
    self_: u32,
    obj_reg: usize,
    vt_reg: usize,
    this_reg: usize,
    fn_reg: usize,
    slot: u32,
    ret: u32,
) -> Flow {
    c.g[obj_reg][0] = Addr(self_.wrapping_add(0x30)).lw(c);
    let obj = r32(c, obj_reg);
    c.g[vt_reg][0] = Addr(obj).lw(c);
    let vt = r32(c, vt_reg);
    c.g[this_reg][0] = Addr(vt.wrapping_add(slot)).lh(c);
    c.g[fn_reg][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
    let func = r32(c, fn_reg);
    c.g[this_reg][0] = sx(obj.wrapping_add(r32(c, this_reg)));
    call_ptr(c, func, ret)
}

/// `void ChaseCameraAlgorithm_GatherRiderState(ChaseCameraAlgorithm* self,
/// CamInfo* out, float a..h)` (0x162568): read the target's velocity (kept
/// from last frame below 27.8 m/s), split it into its horizontal part and
/// the vertical rest, blend the stored horizontal heading towards it by a
/// factor from the slope of the velocity (`|n.z|` mapped between two
/// thresholds onto [lo, hi]; the second set of parameters in trick/rail
/// states 2, 3 and 13), and fill the camera info: rider mode flags, two
/// target parameters, the frame (position, matrix, velocity), its speed and
/// a 0..1 boost amount; mirror three of them into the algorithm.
pub fn chase_camera_algorithm_gather_rider_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        // Keep the eight float parameters in callee-saved registers:
        // f25..f27 = a, b, c; f24 = d; f22, f23 = e, f; f21 = g; f20 = h.
        for (f, src, off) in [
            (27, 14, 0x108),
            (26, 13, 0x100),
            (25, 12, 0xf8),
            (24, 15, 0xf0),
            (23, 17, 0xe8),
            (22, 16, 0xe0),
            (21, 18, 0xd8),
        ] {
            frame.save_fpr(c, f, off);
            c.f[f] = c.f[src];
        }
        frame.save_fpr(c, 20, 0xd0);
        frame.save_ra(c, 0x90);
        c.f[20] = c.f[19];
        let me = r32(c, S1);
        let out = r32(c, S0);
        let sp0 = sp(c);

        // Velocity (returned in sp+0) and the rider's mode.
        c.g[A0][0] = Addr(me.wrapping_add(0x30)).lw(c);
        let obj = r32(c, A0);
        c.g[V0][0] = Addr(obj).lw(c);
        let vt = r32(c, V0);
        c.g[A1][0] = Addr(vt.wrapping_add(0x18)).lh(c);
        c.g[V1][0] = Addr(vt.wrapping_add(0x1c)).lw(c);
        c.g[A1][0] = sx(obj.wrapping_add(r32(c, A1)));
        c.g[A0][0] = sx(sp0);
        let func = r32(c, V1);
        call_ptr(c, func, 0x0016_25e0)?;
        c.g[V0][0] = Addr(me.wrapping_add(0x30)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        call_ptr(c, 0x0011_fe98, 0x0016_25ec)?;
        c.g[S2][0] = c.g[V0][0];
        c.lwc1(1, r32(c, GP).wrapping_sub(0x6afc)); // 27.78 m/s
        vu0_length(c, sp0, 0);
        c.c_lt_s(0, 1);
        c.lq(2, sp0);
        if c.fcc() {
            // Too slow: keep last frame's velocity.
            c.cycles += 3;
            c.lq(2, me.wrapping_add(0x1b0));
            c.sq(2, sp0);
            c.lq(2, sp0);
        }

        // Horizontal part (sp+0x10), vertical rest (sp+0x20), direction
        // (sp+0x30).
        c.cycles += 23;
        c.mtc1(0, 1);
        c.sq(2, sp0 + 0x10);
        c.swc1(1, sp0 + 0x18);
        c.sq(2, me.wrapping_add(0x1b0));
        c.lqc2(3, sp0);
        c.lqc2(4, sp0 + 0x10);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp0 + 0x30);
        c.lq(2, sp0 + 0x30);
        c.sq(2, sp0 + 0x20);
        c.lqc2(3, sp0);
        c.lqc2(5, sp0);
        vu0_dot3(c, 5);
        c.qmfc2(3, 4);
        c.mtc1(3, 0);
        c.c_lt_s(1, 0);
        c.g[V0][0] = 0x50_0000;
        if c.fcc() {
            c.cycles += 13;
            c.lqc2(3, sp0);
            vu0_dot3(c, 3);
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, sp0 + 0x40);
            c.lq(2, sp0 + 0x40);
            c.sq(2, sp0 + 0x30);
        } else {
            // Zero velocity: the default direction (0x4ff120).
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xee0));
            c.lq(3, r32(c, V0));
            c.sq(3, sp0 + 0x30);
        }

        // Pick the parameter set by the target's state.
        c.cycles += 11;
        target_vcall(c, me, A1, V1, A0, V0, 0xa0, 0x0016_26ec)?;
        c.g[V1][0] = c.g[V0][0];
        let state = c.g[V1][0];
        c.g[V0][0] = (sx((state as u32).wrapping_sub(2)) < 2) as u64;
        c.f[12] = c.f[22];
        let second = if c.g[V0][0] != 0 {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0xd;
            c.f[12] = c.f[25];
            if state == 0xd {
                c.cycles += 1;
                c.f[12] = c.f[22];
                true
            } else {
                false
            }
        };
        // f12 = lo, f13 = hi, f14 = t0, f15 = t1.
        if second {
            c.cycles += 4;
            c.f[13] = c.f[23];
            c.f[14] = c.f[21];
            c.f[15] = c.f[20];
        } else {
            c.cycles += 3;
            c.f[13] = c.f[26];
            c.f[14] = c.f[27];
            c.f[15] = c.f[24];
        }

        // frac = clamp((|dir.z| - t0) / (t1 - t0), 0, 1).
        c.cycles += 6;
        let x = Addr(sp0 + 0x38).read_fpu(c).abs();
        c.f[0] = x.0;
        let (lo, hi, t0, t1) = (fpr(c, 12), fpr(c, 13), fpr(c, 14), fpr(c, 15));
        let frac = if !abi_le(c, t0, x) {
            c.cycles += 1;
            FpuF32::ZERO
        } else {
            c.cycles += 3;
            if abi_le(c, x, t1) {
                c.cycles += 5;
                let span = t1 - t0;
                let d = x - t0;
                c.f[0] = d.0;
                d.div(span)
            } else {
                c.cycles += 4;
                c.g[AT][0] = 0x3f80_0000;
                FpuF32::ONE
            }
        };
        c.f[1] = frac.0;
        let range = hi - lo;

        // r = lo + (hi - lo) * frac; while racing (mode 1) a slow
        // horizontal velocity keeps the stored heading.
        c.cycles += 4;
        c.g[V0][0] = 1;
        let step = range * frac;
        c.f[0] = step.0;
        c.f[12] = (lo + step).0;
        if c.g[S2][0] == 1 {
            c.cycles += 14;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x6af8)); // 27.78 m/s
            vu0_length(c, sp0 + 0x10, 1);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.cycles += 2;
                c.lq(2, me.wrapping_add(0x100));
                c.sq(2, sp0 + 0x10);
            }
        }

        // heading = heading * r + horizontal * (1 - r); position = heading
        // + vertical rest. Mode flags.
        c.cycles += 89;
        c.mfc1(2, 12);
        c.lqc2(4, me.wrapping_add(0x100));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp0 + 0x70);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
        c.lq(2, sp0 + 0x70);
        let mode = c.g[S2][0];
        c.g[A2][0] = mode ^ 4;
        c.sub_s(0, 20, 12);
        c.g[A1][0] = mode ^ 2;
        c.sq(2, sp0 + 0x60);
        c.g[A0][0] = mode ^ 5;
        c.mfc1(2, 0);
        c.lqc2(4, sp0 + 0x10);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp0 + 0x80);
        c.g[V1][0] = mode ^ 1;
        c.lq(2, sp0 + 0x80);
        c.g[A2][0] = (c.g[A2][0] == 0) as u64;
        c.g[A1][0] = (c.g[A1][0] == 0) as u64;
        c.g[A0][0] = (c.g[A0][0] == 0) as u64;
        c.sq(2, sp0 + 0x70);
        c.g[V1][0] = (c.g[V1][0] == 0) as u64;
        c.lqc2(3, sp0 + 0x60);
        c.lqc2(4, sp0 + 0x70);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp0 + 0x80);
        c.lq(2, sp0 + 0x80);
        c.sq(2, me.wrapping_add(0x100));
        c.sq(2, sp0 + 0x50);
        c.sq(2, sp0 + 0x40);
        c.lqc2(3, sp0 + 0x40);
        c.lqc2(4, sp0 + 0x20);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp0 + 0x60);
        let (is4, is2, is5, is1) = (c.g[A2][0], c.g[A1][0], c.g[A0][0], c.g[V1][0]);
        Addr(out + 0x64).sw(c, is4);
        Addr(out + 0x58).sw(c, is2);
        Addr(out + 0x5c).sw(c, is5);
        Addr(out + 0x60).sw(c, is1);
        Addr(out + 0x40).sw(c, mode);
        c.lq(2, sp0 + 0x60);
        c.sq(2, sp0 + 0x50);
        // Target parameters (slots 0x88, 0x90) and frame (slot 0x8).
        target_vcall(c, me, A1, V1, A0, V0, 0x88, 0x0016_2890)?;
        c.swc1(0, out + 0x4c);
        target_vcall(c, me, A1, V0, A0, V1, 0x90, 0x0016_28ac)?;
        c.swc1(0, out + 0x54);
        c.g[A0][0] = sx(sp0 + 0x80);
        c.lq(2, sp0 + 0x50);
        c.g[A2][0] = Addr(me.wrapping_add(0x30)).lw(c);
        c.sq(2, sp0 + 0x60);
        let obj = r32(c, A2);
        c.g[V1][0] = Addr(obj).lw(c);
        let vt = r32(c, V1);
        c.g[A1][0] = Addr(vt.wrapping_add(0x8)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(0xc)).lw(c);
        c.g[A1][0] = sx(obj.wrapping_add(r32(c, A1)));
        let func = r32(c, V0);
        call_ptr(c, func, 0x0016_28d4)?;
        c.lq(3, me.wrapping_add(0x40));
        c.lq(4, sp0 + 0x80);
        c.lq(2, sp0 + 0x60);
        c.lwc1(1, out + 0x4c);
        c.mtc1(0, 2);
        c.sq(3, out);
        c.sq(4, out + 0x10);
        c.c_lt_s(2, 1);
        c.sq(2, out + 0x20);
        c.lqc2(3, out + 0x20);
        vu0_dot3(c, 3);
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 0);
        c.sq(4, sp0 + 0x70);
        c.swc1(0, out + 0x44);
        // Boost amount: (p + 1) / 2 for a positive parameter, else 0.
        if c.fcc() {
            c.cycles += 5;
            c.add_s(1, 1, 20);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 0);
            c.mul_s(1, 1, 0);
        } else {
            c.cycles += 1;
            c.f[1] = c.f[2];
        }

        c.cycles += 21;
        c.lwc1(0, out + 0x4c);
        c.swc1(1, out + 0x50);
        c.swc1(0, me.wrapping_add(0x2f4));
        c.lwc1(1, out + 0x50);
        c.swc1(1, me.wrapping_add(0x2f8));
        c.g[V0][0] = Addr(out + 0x5c).lw(c);
        let v0 = c.g[V0][0];
        Addr(me.wrapping_add(0x2fc)).sw(c, v0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore_ra(c, 0x90);
        for (f, off) in [
            (27, 0x108),
            (26, 0x100),
            (25, 0xf8),
            (24, 0xf0),
            (23, 0xe8),
            (22, 0xe0),
            (21, 0xd8),
            (20, 0xd0),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// `c.le.s a, b` (latched in FCC).
#[inline]
pub(crate) fn abi_le(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let le = a.le(b);
    abi::set_fcc(c, le);
    le
}

// ---------------------------------------------------------------------------
// Default_3ChaseCameraAlgorithm_Update (0x176e10)

/// One stage of a chase camera update: `fn(algo, &info, f12..)` at `addr`
/// (other shard), returning to `ret`; `gp` lists the `.lit4` tuning floats
/// loaded (`fN = *(gp - off)`) before the call.
pub(crate) struct ChaseStage {
    addr: u32,
    ret: u32,
    gp: &'static [(usize, u32)],
}

/// The stages of [`default_3_chase_camera_algorithm_update`] after the
/// first (GatherRiderState), up to the constant-argument ones.
pub(crate) const DEFAULT_3_CHASE_STAGES: [ChaseStage; 6] = [
    // ChaseCamera_*(algo, &info): the derived state.
    ChaseStage {
        addr: 0x0016_2998,
        ret: 0x0017_6e60,
        gp: &[],
    },
    ChaseStage {
        addr: 0x0016_2a20,
        ret: 0x0017_6e70,
        gp: &[(21, 0x6100)],
    },
    ChaseStage {
        addr: 0x0016_2b80,
        ret: 0x0017_6e7c,
        gp: &[],
    },
    ChaseStage {
        addr: 0x0016_2c78,
        ret: 0x0017_6e88,
        gp: &[],
    },
    ChaseStage {
        addr: 0x0016_3010,
        ret: 0x0017_6eb0,
        gp: &[
            (17, 0x60fc),
            (18, 0x60f8),
            (12, 0x60f4),
            (13, 0x60f0),
            (14, 0x60ec),
            (15, 0x60e8),
            (16, 0x60e4),
        ],
    },
    ChaseStage {
        addr: 0x0016_3270,
        ret: 0x0017_6ed0,
        gp: &[
            (16, 0x60e0),
            (12, 0x60dc),
            (13, 0x60d8),
            (14, 0x60d4),
            (15, 0x60d0),
        ],
    },
];

/// Load a stage's tuning floats and call it with `(algo, sp)`.
pub(crate) fn chase_stage(c: &mut Cpu, algo: u64, st: &ChaseStage) -> crate::call::Flow {
    let gp = c.g[GP][0] as u32;
    for &(f, off) in st.gp {
        c.lwc1(f, gp.wrapping_sub(off));
    }
    chase_call(c, algo, st.addr, st.ret)
}

/// `addr(algo, &info)` (info at `sp`) returning to `ret`.
pub(crate) fn chase_call(c: &mut Cpu, algo: u64, addr: u32, ret: u32) -> crate::call::Flow {
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = algo;
    call_ptr(c, addr, ret)
}

/// `void Default_3ChaseCameraAlgorithm::Update()` (0x176e10): the default
/// chase camera (tuning set 3): gather the rider's state into a local info
/// block, then run the shared chase-camera stages over it with this
/// camera's tuning (distances, heights, damping, look-ahead, FOV, the
/// collision push and the final smoothing), all from `.lit4` constants and
/// a few literal ones (0, 1, 15, 1.5, 100, 300, 600).
pub fn default_3_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 115;
        let frame = Frame::open(c, 0xa0);
        let gp = c.g[GP][0] as u32;
        c.lwc1(19, gp.wrapping_sub(0x6120));
        frame.save(c, S0, 0x80);
        c.g[A1][0] = c.g[SP][0];
        let algo = c.g[A0][0];
        c.g[S0][0] = algo;
        for (f, off) in [
            (12, 0x611cu32),
            (13, 0x6118),
            (14, 0x6114),
            (15, 0x6110),
            (16, 0x610c),
            (17, 0x6108),
            (18, 0x6104),
        ] {
            c.lwc1(f, gp.wrapping_sub(off));
        }
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        // GatherRiderState(algo, &info, tuning f12..f19).
        call_ptr(c, 0x0016_2568, 0x0017_6e50)?;
        // f20 = 0 and f21 (a .lit4 tuning value) are reused below.
        c.mtc1(0, 20);
        for st in DEFAULT_3_CHASE_STAGES.iter() {
            chase_stage(c, algo, st)?;
        }

        let gp = c.g[GP][0] as u32;
        c.lwc1(13, gp.wrapping_sub(0x60cc));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 14);
        c.lwc1(15, gp.wrapping_sub(0x60c8));
        c.mov_s(12, 20);
        chase_call(c, algo, 0x0016_33b0, 0x0017_6ef0)?;

        let gp = c.g[GP][0] as u32;
        c.lwc1(18, gp.wrapping_sub(0x60c4));
        c.lwc1(19, gp.wrapping_sub(0x60c0));
        c.mov_s(17, 21);
        c.lwc1(12, gp.wrapping_sub(0x60bc));
        c.mov_s(16, 20);
        for (f, off) in [(13, 0x60b8u32), (14, 0x60b4), (15, 0x60b0)] {
            c.lwc1(f, gp.wrapping_sub(off));
        }
        chase_call(c, algo, 0x0016_3450, 0x0017_6f1c)?;

        for st in [
            ChaseStage {
                addr: 0x0016_2b90,
                ret: 0x0017_6f3c,
                gp: &[
                    (16, 0x60ac),
                    (12, 0x60a8),
                    (13, 0x60a4),
                    (14, 0x60a0),
                    (15, 0x609c),
                ],
            },
            ChaseStage {
                addr: 0x0016_35f8,
                ret: 0x0017_6f58,
                gp: &[(14, 0x6098), (15, 0x6094), (13, 0x6090), (12, 0x608c)],
            },
            ChaseStage {
                addr: 0x0016_4878,
                ret: 0x0017_6f68,
                gp: &[(12, 0x6088)],
            },
            ChaseStage {
                addr: 0x0016_46a0,
                ret: 0x0017_6f7c,
                gp: &[(12, 0x6084), (13, 0x6080)],
            },
        ] {
            chase_stage(c, algo, &st)?;
        }

        // (algo, &info, 600, 300, 100, f21).
        c.g[AT][0] = 0x4416_0000;
        c.mtc1(AT, 12);
        c.mov_s(15, 21);
        c.g[AT][0] = 0x4396_0000;
        c.mtc1(AT, 13);
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 14);
        chase_call(c, algo, 0x0016_41c0, 0x0017_6fa4)?;

        // (algo, &info, 15, 1.5, 1.5).
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 13);
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(AT, 12);
        c.mov_s(14, 13);
        chase_call(c, algo, 0x0016_43a8, 0x0017_6fc4)?;

        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CameraTrigger_ReadAction (0x001712b0)

/// `CameraTrigger_ReadActionType0` (0x001715a0).
pub(crate) const CAMERA_TRIGGER_READ_ACTION_TYPE0_CALL: Callee = Callee {
    addr: 0x0017_15a0,
    f: ee::s02::CameraTrigger_ReadActionType0,
};

/// `CameraTrigger_ReadActionType1` (0x001714e8).
pub(crate) const CAMERA_TRIGGER_READ_ACTION_TYPE1_CALL: Callee = Callee {
    addr: 0x0017_14e8,
    f: ee::s02::CameraTrigger_ReadActionType1,
};

/// `CameraTrigger_ReadActionType2` (0x001715f8).
pub(crate) const CAMERA_TRIGGER_READ_ACTION_TYPE2_CALL: Callee = Callee {
    addr: 0x0017_15f8,
    f: ee::s02::CameraTrigger_ReadActionType2,
};

/// `CameraTrigger_ReadActionType3` (0x00171680).
pub(crate) const CAMERA_TRIGGER_READ_ACTION_TYPE3_CALL: Callee = Callee {
    addr: 0x0017_1680,
    f: ee::s02::CameraTrigger_ReadActionType3,
};

/// `int CameraTrigger::ReadAction(reader, out**)` (0x1712b0): read the action type (`get_uint`), allocate the matching action object (switch 0x10 bytes, bounded 0x34, spline 0x20, none 8) with its default fields, let the type-specific reader fill it, store it in `*out` and return the bytes consumed. Unknown types read nothing.
pub fn camera_trigger_read_action_2(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        'b_1714c8: {
            'b_1714bc: {
                'b_171480: {
                    'b_171414: {
                        'b_171370: {
                            c.cycles += 16;
                            Frame::open(c, 0x70);
                            frame.save(c, S3, 0x30);
                            frame.save(c, S4, 0x20);
                            c.g[S3][0] = c.g[A0][0];
                            c.g[S4][0] = c.g[A1][0];
                            frame.save(c, S1, 0x50);
                            frame.save(c, S2, 0x40);
                            frame.save(c, S0, 0x60);
                            frame.save_ra(c, 0x10);
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, 0x002d_2558, 0x0017_12dc)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[S1][0] = c.lw(r32(c, SP));
                            c.g[V0][0] = 0x1;
                            let t = c.g[S1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_171370;
                            } else {
                                'b_171320: {
                                    'b_171308: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_171308;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S1][0] == 0u64;
                                            c.g[V0][0] = c.g[S2][0];
                                            if t {
                                                break 'b_171320;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                frame.restore(c, S0, 0x60);
                                                if t {
                                                    break 'b_1714c8;
                                                } else {
                                                    break 'b_171308;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_171414;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];
                                        c.g[V0][0] = c.g[S2][0];
                                        if t {
                                            break 'b_171480;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            frame.restore(c, S0, 0x60);
                                            if t {
                                                break 'b_1714c8;
                                            } else {
                                                break 'b_171320;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 20;
                                c.g[A1][0] = 0x460000;
                                c.g[A0][0] = 0x10;
                                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x3f50));
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x0031_7d70, 0x0017_1338)?;
                                c.g[AT][0] = 0x3f80_0000;
                                c.mtc1(1, 0);
                                c.g[V1][0] = 0x460000;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3d18));
                                c.sw(r32(c, S0).wrapping_add(0x4), c.g[V1][0]);
                                c.g[A0][0] = c.g[S3][0];
                                c.swc1(0, r32(c, S0).wrapping_add(0xc));
                                c.g[A1][0] = c.g[S0][0];
                                c.sw(r32(c, S0), 0u64);
                                c.sw(r32(c, S0).wrapping_add(0x8), 0u64);
                                CAMERA_TRIGGER_READ_ACTION_TYPE0_CALL.call(c, 0x0017_1368)?;
                                let t = true;
                                c.sw(r32(c, S4), c.g[S0][0]);
                                if t {
                                    break 'b_1714bc;
                                } else {
                                    break 'b_171370;
                                }
                            }
                        }
                        c.cycles += 41;
                        c.g[A1][0] = 0x460000;
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = 0x34;
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x3f38));
                        call_ptr(c, 0x0031_7d70, 0x0017_1388)?;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 3);
                        c.g[V1][0] = 0x460000;
                        c.lwc1(4, r32(c, GP).wrapping_sub(0x643c));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[AT][0] = 0x43c8_0000;
                        c.mtc1(1, 2);
                        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3e30));
                        c.g[AT][0] = 0x41c8_0000;
                        c.mtc1(1, 1);
                        c.g[V0][0] = 0x500000;
                        c.g[AT][0] = 0x428c_0000;
                        c.mtc1(1, 0);
                        c.g[A0][0] = c.g[S3][0];
                        c.sw(r32(c, S0).wrapping_add(0x4), c.g[V1][0]);
                        c.g[A1][0] = c.g[S0][0];
                        c.sw(r32(c, S0), c.g[S1][0]);
                        c.swc1(3, r32(c, S0).wrapping_add(0xc));
                        c.swc1(4, r32(c, S0).wrapping_add(0x14));
                        c.swc1(2, r32(c, S0).wrapping_add(0x10));
                        c.swc1(1, r32(c, S0).wrapping_add(0x1c));
                        c.swc1(0, r32(c, S0).wrapping_add(0x18));
                        c.sw(r32(c, S0).wrapping_add(0x8), 0u64);
                        c.sw(r32(c, S0).wrapping_add(0x24), 0u64);
                        c.g[T0][0] = sx(r32(c, V0).wrapping_sub(0xf28));
                        c.ldl(3, r32(c, T0).wrapping_add(0x7));
                        c.ldr(3, r32(c, T0));
                        c.g[A2][0] = c.lw(r32(c, T0).wrapping_add(0x8));
                        c.sdl(3, r32(c, S0).wrapping_add(0x2f));
                        c.sdr(3, r32(c, S0).wrapping_add(0x28));
                        c.sw(r32(c, S0).wrapping_add(0x30), c.g[A2][0]);
                        c.sw(r32(c, S0).wrapping_add(0x20), 0u64);
                        CAMERA_TRIGGER_READ_ACTION_TYPE1_CALL.call(c, 0x0017_140c)?;
                        let t = true;
                        c.sw(r32(c, S4), c.g[S0][0]);
                        if t {
                            break 'b_1714bc;
                        } else {
                            break 'b_171414;
                        }
                    }
                    c.cycles += 27;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x20;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x3f20));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x0031_7d70, 0x0017_142c)?;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 1);
                    c.g[V1][0] = 0x460000;
                    c.lwc1(2, r32(c, GP).wrapping_sub(0x6438));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[AT][0] = 0x4080_0000;
                    c.mtc1(1, 0);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3e48));
                    c.sw(r32(c, S0).wrapping_add(0x4), c.g[V1][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.sw(r32(c, S0), c.g[S1][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.swc1(1, r32(c, S0).wrapping_add(0xc));
                    c.swc1(2, r32(c, S0).wrapping_add(0x10));
                    c.swc1(0, r32(c, S0).wrapping_add(0x18));
                    c.sw(r32(c, S0).wrapping_add(0x8), 0u64);
                    c.sw(r32(c, S0).wrapping_add(0x14), 0u64);
                    c.sw(r32(c, S0).wrapping_add(0x1c), 0u64);
                    CAMERA_TRIGGER_READ_ACTION_TYPE2_CALL.call(c, 0x0017_1478)?;
                    let t = true;
                    c.sw(r32(c, S4), c.g[S0][0]);
                    if t {
                        break 'b_1714bc;
                    } else {
                        break 'b_171480;
                    }
                }
                c.cycles += 15;
                c.g[A1][0] = 0x460000;
                c.g[A0][0] = 0x8;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x3f08));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x0031_7d70, 0x0017_1498)?;
                c.g[V1][0] = 0x460000;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x3d18));
                c.sw(r32(c, S0), c.g[S1][0]);
                c.sw(r32(c, S0).wrapping_add(0x4), c.g[V1][0]);
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                CAMERA_TRIGGER_READ_ACTION_TYPE3_CALL.call(c, 0x0017_14b8)?;
                c.sw(r32(c, S4), c.g[S0][0]);
                break 'b_1714bc;
            }
            c.cycles += 3;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
            c.g[V0][0] = c.g[S2][0];
            frame.restore(c, S0, 0x60);
            break 'b_1714c8;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int CameraTrigger_ReadBoundObj(Reader* reader, BoundObj** out)` (0x171690): read a
/// bound-object type word (0 ellipse, 1 box, 2 line, 3 point), allocate the
/// matching object (0x2c bytes; the line 0x44), let its own reader fill it,
/// store it in `*out` and return the bytes consumed. An unknown type leaves
/// `*out` alone and returns just the type word's size.
pub fn camera_trigger_read_bound_obj(c: &mut Cpu) {
    const CAMERA_BOX_BOUND_OBJ_READ: Callee = Callee {
        addr: 0x00171b10,
        f: ee::s02::f_00171b10,
    };
    const CAMERA_ELLIPSE_BOUND_OBJ_READ: Callee = Callee {
        addr: 0x00171a20,
        f: ee::s02::f_00171a20,
    };
    const CAMERA_LINE_BOUND_OBJ_READ: Callee = Callee {
        addr: 0x00171c00,
        f: ee::s02::f_00171c00,
    };
    const CAMERA_TRIGGER_READ_POINT_BOUND_OBJ: Callee = Callee {
        addr: 0x00171d10,
        f: ee::s02::f_00171d10,
    };
    guest(c, |c| {
        'b_171a00: {
            'b_1719f4: {
                'b_171954: {
                    'b_171848: {
                        'b_1717a4: {
                            c.cycles += 16;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[S3][0] = c.g[A0][0];
                            c.g[S4][0] = c.g[A1][0];
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            call_ptr(c, 0x002d2558, 0x001716bc)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = 0x1;
                            let t = c.g[S1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_1717a4;
                            } else {
                                'b_171700: {
                                    'b_1716e8: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_1716e8;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S1][0] == 0u64;
                                            c.g[V0][0] = c.g[S2][0];
                                            if t {
                                                break 'b_171700;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                                if t {
                                                    break 'b_171a00;
                                                } else {
                                                    break 'b_1716e8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_171848;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];
                                        c.g[V0][0] = c.g[S2][0];
                                        if t {
                                            break 'b_171954;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            if t {
                                                break 'b_171a00;
                                            } else {
                                                break 'b_171700;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 41;
                                c.g[A1][0] = 0x460000;
                                c.g[A2][0] = 0u64;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3ef0));
                                c.g[A3][0] = 0u64;
                                c.g[A0][0] = 0x2c;
                                call_ptr(c, 0x00317d70, 0x00171718)?;
                                c.g[V1][0] = 0x460000;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3c10));
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 0);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                                c.g[A1][0] = 0x500000;
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = 0x460000;
                                c.swc1(0, c.g[SP][0] as u32);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3d58));
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.g[S3][0];
                                c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
                                c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                                c.ldr(3, c.g[T0][0] as u32);
                                c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                                c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
                                c.sdr(3, c.g[S0][0] as u32);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                                c.g[A1][0] = c.g[S0][0];
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                                c.ldl(9, (c.g[SP][0] as u32).wrapping_add(0x7));
                                c.ldr(9, c.g[SP][0] as u32);
                                c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                c.sdl(9, (c.g[S0][0] as u32).wrapping_add(0x1f));
                                c.sdr(9, (c.g[S0][0] as u32).wrapping_add(0x18));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[T2][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
                                CAMERA_ELLIPSE_BOUND_OBJ_READ.call(c, 0x0017179c)?;
                                let t = true;
                                c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                                if t {
                                    break 'b_1719f4;
                                } else {
                                    break 'b_1717a4;
                                }
                            }
                        }
                        c.cycles += 41;
                        c.g[A1][0] = 0x460000;
                        c.g[A3][0] = 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3ed8));
                        c.g[A2][0] = 0u64;
                        c.g[A0][0] = 0x2c;
                        call_ptr(c, 0x00317d70, 0x001717bc)?;
                        c.g[V1][0] = 0x460000;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3c10));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                        c.g[A1][0] = 0x500000;
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = 0x460000;
                        c.swc1(0, c.g[SP][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3d98));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
                        c.ldl(7, (c.g[T2][0] as u32).wrapping_add(0x7));
                        c.ldr(7, c.g[T2][0] as u32);
                        c.g[T0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                        c.sdl(7, (c.g[S0][0] as u32).wrapping_add(0x7));
                        c.sdr(7, c.g[S0][0] as u32);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                        c.g[A1][0] = c.g[S0][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[SP][0] as u32);
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x1f));
                        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x18));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A2][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S1][0]);
                        CAMERA_BOX_BOUND_OBJ_READ.call(c, 0x00171840)?;
                        let t = true;
                        c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                        if t {
                            break 'b_1719f4;
                        } else {
                            break 'b_171848;
                        }
                    }
                    c.cycles += 40;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x44;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3ec0));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x00171860)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = 0x500000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3c10));
                    c.g[A3][0] = c.g[V1][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0x460000;
                    c.swc1(0, c.g[SP][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3dd8));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xf28));
                    c.ldl(9, (c.g[T0][0] as u32).wrapping_add(0x7));
                    c.ldr(9, c.g[T0][0] as u32);
                    c.g[T2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[S0][0] as u32).wrapping_add(0x7));
                    c.sdr(9, c.g[S0][0] as u32);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[V1][0] = 0x1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                    c.ldl(9, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.ldr(9, c.g[SP][0] as u32);
                    c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[S0][0] as u32).wrapping_add(0x1f));
                    c.sdr(9, (c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[T2][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    'l_1718e8: loop {
                        c.cycles += 7;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        let t = c.g[V1][0] != c.g[A0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001718e8);
                            }
                            continue 'l_1718e8;
                        } else {
                            c.cycles += 20;
                            c.g[A0][0] = c.g[S3][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
                            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x7));
                            c.ldr(9, c.g[V1][0] as u32);
                            c.g[T2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0x33));
                            c.sdr(9, (c.g[A2][0] as u32).wrapping_add(0x2c));
                            c.sw((c.g[A2][0] as u32).wrapping_add(0x34), c.g[T2][0]);
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
                            c.ldl(8, (c.g[V0][0] as u32).wrapping_add(0x7));
                            c.ldr(8, c.g[V0][0] as u32);
                            c.g[T1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x3f));
                            c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x38));
                            c.sw((c.g[A2][0] as u32).wrapping_add(0x40), c.g[T1][0]);
                            c.g[A1][0] = c.g[S0][0];
                            CAMERA_LINE_BOUND_OBJ_READ.call(c, 0x0017194c)?;
                            let t = true;
                            c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                            if t {
                                break 'b_1719f4;
                            } else {
                                break 'b_171954;
                            }
                        }
                    }
                }
                c.cycles += 40;
                c.g[A1][0] = 0x460000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3ea8));
                c.g[A3][0] = 0u64;
                c.g[A0][0] = 0x2c;
                call_ptr(c, 0x00317d70, 0x0017196c)?;
                c.g[V1][0] = 0x460000;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3c10));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                c.g[A1][0] = 0x500000;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x460000;
                c.swc1(0, c.g[SP][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3e18));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[S3][0];
                c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
                c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[T0][0] as u32);
                c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
                c.sdr(3, c.g[S0][0] as u32);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A1][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                c.ldl(9, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(9, c.g[SP][0] as u32);
                c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(9, (c.g[S0][0] as u32).wrapping_add(0x1f));
                c.sdr(9, (c.g[S0][0] as u32).wrapping_add(0x18));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[T2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S1][0]);
                CAMERA_TRIGGER_READ_POINT_BOUND_OBJ.call(c, 0x001719f0)?;
                c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                break 'b_1719f4;
            }
            c.cycles += 3;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = c.g[S2][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            break 'b_171a00;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        Ok(())
    });
}

/// `int CameraTrigger_ReadSphereTrigger(Reader* reader, Volume** out)`
/// (0x172e18): read a volume type word (0 ellipse, 1 box), allocate and read
/// the matching volume and return the bytes consumed. Any other type pushes
/// the stream back by the type word and returns 0.
pub fn camera_trigger_read_sphere_trigger(c: &mut Cpu) {
    const CAMERA_TRIGGER_READ_BOX_TRIGGER_DATA: Callee = Callee {
        addr: 0x001730d0,
        f: ee::s02::f_001730d0,
    };
    const CAMERA_TRIGGER_READ_SPHERE_TRIGGER_DATA: Callee = Callee {
        addr: 0x00172fe0,
        f: ee::s02::f_00172fe0,
    };
    guest(c, |c| {
        'b_172fc0: {
            'b_172fbc: {
                'b_172fac: {
                    'b_172fa4: {
                        'b_172f08: {
                            'b_172e64: {
                                c.cycles += 14;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[S2][0] = c.g[A0][0];
                                c.g[S4][0] = c.g[A1][0];
                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                call_ptr(c, 0x002d2558, 0x00172e44)?;
                                c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                let t = c.g[S1][0] == 0u64;
                                c.g[S3][0] = c.g[V0][0];
                                if t {
                                    break 'b_172e64;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[S1][0] == c.g[V0][0];
                                    c.g[A1][0] = 0x460000;
                                    if t {
                                        break 'b_172f08;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                                        if t {
                                            break 'b_172fac;
                                        } else {
                                            break 'b_172e64;
                                        }
                                    }
                                }
                            }
                            c.cycles += 41;
                            c.g[A1][0] = 0x460000;
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3e78));
                            c.g[A0][0] = 0x2c;
                            call_ptr(c, 0x00317d70, 0x00172e7c)?;
                            c.g[V1][0] = 0x460000;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3c10));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                            c.g[A1][0] = 0x500000;
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = 0x460000;
                            c.swc1(0, c.g[SP][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3cb0));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.g[S2][0];
                            c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
                            c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                            c.ldr(3, c.g[T0][0] as u32);
                            c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                            c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
                            c.sdr(3, c.g[S0][0] as u32);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A1][0] = c.g[S0][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                            c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                            c.ldr(3, c.g[SP][0] as u32);
                            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                            c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x1f));
                            c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x18));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A2][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
                            CAMERA_TRIGGER_READ_SPHERE_TRIGGER_DATA.call(c, 0x00172f00)?;
                            let t = true;
                            c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                            if t {
                                break 'b_172fa4;
                            } else {
                                break 'b_172f08;
                            }
                        }
                        c.cycles += 39;
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3e60));
                        c.g[A0][0] = 0x2c;
                        call_ptr(c, 0x00317d70, 0x00172f1c)?;
                        c.g[V1][0] = 0x460000;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3c10));
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                        c.g[A1][0] = 0x500000;
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = 0x460000;
                        c.swc1(0, c.g[SP][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3d00));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf28));
                        c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[T0][0] as u32);
                        c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
                        c.sdr(3, c.g[S0][0] as u32);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                        c.g[A1][0] = c.g[S0][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[SP][0] as u32);
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x1f));
                        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x18));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A2][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S1][0]);
                        CAMERA_TRIGGER_READ_BOX_TRIGGER_DATA.call(c, 0x00172fa0)?;
                        c.sw(c.g[S4][0] as u32, c.g[S0][0]);
                        break 'b_172fa4;
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        break 'b_172fbc;
                    } else {
                        break 'b_172fac;
                    }
                }
                c.cycles += 4;
                c.g[V0][0] = 0u64;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                let t = true;
                c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                if t {
                    break 'b_172fc0;
                } else {
                    break 'b_172fbc;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
            break 'b_172fc0;
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// CameraTrigger_ReadSphereTriggerData (0x172fe0)

/// The serializer `cCTVolumeEllipse` reads: the five fields' accessor offsets
/// in the trigger's descriptor (`+0x24`): (accessor offset, reader function,
/// return address of the accessor call, return address of the reader call).
pub(crate) const SPHERE_TRIGGER_FIELDS: [(u32, u32, u32, u32); 5] = [
    (0x08, 0x002d_25a0, 0x0017_3014, 0x0017_3020),
    (0x28, 0x002d_25a0, 0x0017_3038, 0x0017_3044),
    (0x10, 0x002d_2510, 0x0017_305c, 0x0017_3068),
    (0x18, 0x002d_2510, 0x0017_3080, 0x0017_308c),
    (0x20, 0x002d_2510, 0x0017_30a4, 0x0017_30b0),
];

/// `int get_cCTVolumeEllipse(reader, cCTVolumeEllK2* obj)` (0x172fe0): read
/// the camera-trigger ellipse volume from a serializer: clears `obj+0x28`,
/// then for each of its five fields (center, axis, size x/y/z) calls the
/// field's accessor through the descriptor at `obj+0x24` (vtable-style
/// `{delta, function}` pairs) and feeds the returned pointer to the reader
/// (a vector reader for the first two, a float reader for the others).
/// Returns the sum of the readers' results.
pub fn camera_trigger_read_sphere_trigger_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 59;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        oc417_sw0(c, S0, 0x28);
        for (i, &(field, reader, ret_accessor, ret_reader)) in
            SPHERE_TRIGGER_FIELDS.iter().enumerate()
        {
            c.g[V1][0] = oc417_lw(c, S0, 0x24);
            c.g[A0][0] = oc417_lh(c, V1, field);
            c.g[V0][0] = oc417_lw(c, V1, field + 4);
            let accessor = c.g[V0][0] as u32;
            c.g[A0][0] = oc417_add(c.g[S0][0], c.g[A0][0]);
            call_ptr(c, accessor, ret_accessor)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, reader, ret_reader)?;
            if i == 0 {
                c.g[S1][0] = c.g[V0][0];
            } else {
                c.g[S1][0] = oc417_add(c.g[S1][0], c.g[V0][0]);
            }
        }
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CameraTrigger_ReadPointBoundObj(Serializer *s, BoundObjPoint *obj)`
/// (0x171d10): marks the bound object as a point (`obj->+0x28 = 3`), asks its
/// owner (`obj->+0x24`, vtable slot +8/+0xc) for the vector to fill, and reads
/// three floats into it (`Serialize_ReadVec3(s, vec)`).
pub fn camera_trigger_read_point_bound_obj(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 3;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw((c.g[A1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0017_1d3c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d_25a0, 0x0017_1d48)?; // Serialize_ReadVec3
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int CameraTrigger_ReadBoxTriggerData(Reader* reader, CameraTrigger* obj)` (0x1730d0): read a box trigger from a stream and return the number of bytes read.
/// Mark the trigger loaded (`obj+0x28 = 1`), then read its five fields through the object's accessor virtuals (vtable at `obj+0x24`, slots +8, +0x28, +0x10, +0x18, +0x20): the center and axis vectors (`t3Vector` reads, 0x2d25a0) and the three float sizes (`Serialize_ReadU32`). Returns the summed sizes.
pub fn camera_trigger_read_box_trigger_data_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0x1;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00173108)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00173114)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0017312c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00173138)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00173150)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x0017315c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00173174)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00173180)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00173198)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x001731a4)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `t3Vector& CameraBoundObj_GetPosition(CameraBoundObj* self)` (0x176440,
/// `cCTObject::pos`): the position vector is the object's first member, so
/// it returns `self`.
pub fn camera_bound_obj_get_position(c: &mut Cpu) {
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A0][0];
    c.pc = ra;
}

/// `void* CameraBoundObj_GetScale(CameraBoundObj* self)` (0x00176460):
/// the address of the scale field (`self + 0x18`).
pub fn camera_bound_obj_get_scale(c: &mut Cpu) {
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    c.pc = ra;
}

/// `void* CameraBoundObj_GetField14(CameraBoundObj* self)` (0x00176458):
/// the address of the field at `self + 0x14`.
pub fn camera_bound_obj_get_field14(c: &mut Cpu) {
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
    c.pc = ra;
}

/// `float& cCTObject::heading()` / `CameraBoundObj_GetRotX` (0x176448):
/// returns `this + 0xc`.
pub fn camera_bound_obj_get_rot_x(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0xc));
    ret(c);
}

/// `float& cCTObject::pitch()` / `CameraBoundObj_GetRotY` (0x176450):
/// returns `this + 0x10`.
pub fn camera_bound_obj_get_rot_y(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x10));
    ret(c);
}

/// `void Default_3ChaseCameraAlgorithm_dtor(self)` (0x176da8): sets the
/// vtable (0x45ca38) at `+0x10`, then runs the base destructor
/// (`ChaseCameraAlgorithm_dtor`, 0x162458).
pub fn default_3_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x46_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x35c8));
        Addr(r32(c, V1).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0016_2458, 0x0017_6dc4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The delete helper called by scalar-deleting destructors (0x317e50).
pub(crate) const OC437_OPERATOR_DELETE: u32 = 0x0031_7e50;

/// A scalar-deleting destructor: sets the vtable pointer
/// (`vtable_hi - vtable_lo`) at `this + vtable_field`, and when `flags & 1`
/// calls the delete helper (0x317e50) on `this`.
pub(crate) fn oc437_dtor_flag_delete(
    c: &mut Cpu,
    vtable_hi: u32,
    vtable_lo: u32,
    vtable_field: u32,
    ret_to: u32,
) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = vtable_hi as u64;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(vtable_hi.wrapping_sub(vtable_lo));
        c.g[A1][0] &= 1;
        let skip = c.g[A1][0] == 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(vtable_field), c.g[V0][0]);
        if !skip {
            c.cycles += 2;
            call_ptr(c, OC437_OPERATOR_DELETE, ret_to)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `TargetScriptCameraAlgo_dtor(this, flags)` (0x176398): vtable at +0x10, delete if `flags & 1`.
pub fn target_script_camera_algo_dtor(c: &mut Cpu) {
    oc437_dtor_flag_delete(c, 0x46_0000, 0x4220, 0x10, 0x0017_63bc)
}

/// `ManualScriptCameraAlgo_dtor(this, flags)` (0x176368): vtable at +0x10, delete if `flags & 1`.
pub fn manual_script_camera_algo_dtor(c: &mut Cpu) {
    oc437_dtor_flag_delete(c, 0x46_0000, 0x4220, 0x10, 0x0017_638c)
}

/// `base + off` with `base` a register's low word (the generated
/// `(c.g[R][0] as u32).wrapping_add(off)`).
#[inline(always)]
pub(crate) fn cvb_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `base - back` with `base` a register's low word (the generated
/// `wrapping_sub`).
#[inline(always)]
pub(crate) fn cvb_back(c: &Cpu, reg: usize, back: u32) -> u32 {
    r32(c, reg).wrapping_sub(back)
}

/// `bool cCTVolumeBox_containsPoint(cCTVolumeBox* box, t4Vector* pt)`
/// (0x1728f8), the `CameraBoxTrigger` containment test. The point is mapped
/// into the box's local frame: the identity matrix is turned by `-rot[1]`
/// about x and `-rot[0]` about z (`Math_SinCos` for each angle, the matrix
/// products as VU0 operations on the stack copies) and translated by `-pos`.
/// The box extends `size` either way along each axis; the point is inside
/// when it is on the non-positive side of all six slab planes. The three axis
/// vectors are copied into the static table at `0x4c5480` on the first call
/// (the flag is `gp + 0x1cfc`). Returns 1 inside, 0 outside.
///
/// Transcribed op for op from the generated body: the VU0 and FPU steps and
/// the stack spills stay as the original issues them.
pub fn camera_box_trigger_contains_point(c: &mut Cpu) {
    guest(c, |c| {
        'b_172d38: {
            c.cycles += 260;
            c.g[SP][0] = sx(cvb_back(c, SP, 0x130));
            c.g[V0][0] = 0x500000;
            c.sq(17, cvb_at(c, SP, 0x100));
            c.g[V0][0] = sx(cvb_back(c, V0, 0xe60));
            c.sq(20, cvb_at(c, SP, 0xd0));
            c.g[S1][0] = c.g[A0][0];
            c.g[S4][0] = c.g[A1][0];
            c.sq(16, cvb_at(c, SP, 0x110));
            c.sq(18, cvb_at(c, SP, 0xf0));
            c.sq(19, cvb_at(c, SP, 0xe0));
            c.sq(21, cvb_at(c, SP, 0xc0));
            c.sq(22, cvb_at(c, SP, 0xb0));
            c.sd(cvb_at(c, SP, 0xa0), c.g[RA][0]);
            c.swc1(20, cvb_at(c, SP, 0x120));
            c.lqc2(1, r32(c, V0));
            c.lqc2(2, cvb_at(c, V0, 0x10));
            c.lqc2(3, cvb_at(c, V0, 0x20));
            c.lqc2(4, cvb_at(c, V0, 0x30));
            c.sqc2(1, r32(c, SP));
            c.sqc2(2, cvb_at(c, SP, 0x10));
            c.sqc2(3, cvb_at(c, SP, 0x20));
            c.sqc2(4, cvb_at(c, SP, 0x30));
            c.lwc1(12, cvb_at(c, S1, 0x10));
            c.g[S2][0] = c.g[SP][0];
            c.g[A0][0] = sx(cvb_at(c, SP, 0x90));
            c.g[A1][0] = sx(cvb_at(c, SP, 0x94));
            c.neg_s(12, 12);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 20);
            c.g[S0][0] = 0x500000;
            c.g[S6][0] = sx(cvb_back(c, S0, 0xec0));
            call_ptr(c, 0x0031be50, 0x00172978)?;
            c.lwc1(11, cvb_at(c, SP, 0x94));
            c.g[S3][0] = sx(cvb_at(c, SP, 0x40));
            c.lwc1(3, cvb_at(c, S6, 0x8));
            c.sub_s(1, 20, 11);
            c.lwc1(0, cvb_back(c, S0, 0xec0));
            c.lwc1(6, cvb_at(c, S6, 0x4));
            c.lwc1(8, cvb_at(c, SP, 0x90));
            c.mul_s(2, 1, 3);
            c.sw(cvb_at(c, SP, 0x70), 0u64);
            c.mul_s(4, 1, 0);
            c.sw(cvb_at(c, SP, 0x74), 0u64);
            c.mul_s(1, 1, 6);
            c.sw(cvb_at(c, SP, 0x78), 0u64);
            c.mul_s(13, 8, 3);
            c.sw(cvb_at(c, SP, 0x4c), 0u64);
            c.mul_s(12, 8, 0);
            c.sw(cvb_at(c, SP, 0x5c), 0u64);
            c.mul_s(9, 2, 3);
            c.sw(cvb_at(c, SP, 0x6c), 0u64);
            c.mul_s(10, 4, 3);
            c.swc1(20, cvb_at(c, SP, 0x7c));
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
            c.swc1(5, cvb_at(c, SP, 0x48));
            c.add_s(7, 7, 11);
            c.swc1(0, cvb_at(c, SP, 0x44));
            c.add_s(4, 4, 13);
            c.swc1(2, cvb_at(c, SP, 0x58));
            c.sub_s(10, 10, 8);
            c.swc1(9, cvb_at(c, SP, 0x68));
            c.add_s(1, 1, 11);
            c.swc1(7, cvb_at(c, SP, 0x40));
            c.add_s(3, 3, 12);
            c.swc1(4, cvb_at(c, SP, 0x50));
            c.swc1(10, cvb_at(c, SP, 0x60));
            c.swc1(1, cvb_at(c, SP, 0x54));
            c.swc1(3, cvb_at(c, SP, 0x64));
            c.lqc2(4, r32(c, S2));
            c.lqc2(5, cvb_at(c, S2, 0x10));
            c.lqc2(6, cvb_at(c, S2, 0x20));
            c.lqc2(7, cvb_at(c, S2, 0x30));
            c.lqc2(8, r32(c, S3));
            c.lqc2(9, cvb_at(c, S3, 0x10));
            c.lqc2(10, cvb_at(c, S3, 0x20));
            c.lqc2(11, cvb_at(c, S3, 0x30));
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
            c.sqc2(12, r32(c, S2));
            c.sqc2(13, cvb_at(c, S2, 0x10));
            c.sqc2(14, cvb_at(c, S2, 0x20));
            c.sqc2(15, cvb_at(c, S2, 0x30));
            c.lwc1(12, cvb_at(c, S1, 0xc));
            c.g[S2][0] = c.g[SP][0];
            c.g[A0][0] = sx(cvb_at(c, SP, 0x98));
            c.g[A1][0] = sx(cvb_at(c, SP, 0x9c));
            c.neg_s(12, 12);
            c.g[S0][0] = 0x500000;
            c.g[S5][0] = sx(cvb_back(c, S0, 0xea0));
            call_ptr(c, 0x0031be50, 0x00172ac4)?;
            c.lwc1(11, cvb_at(c, SP, 0x9c));
            c.lwc1(3, cvb_at(c, S5, 0x8));
            c.sub_s(1, 20, 11);
            c.lwc1(0, cvb_back(c, S0, 0xea0));
            c.lwc1(6, cvb_at(c, S5, 0x4));
            c.lwc1(8, cvb_at(c, SP, 0x98));
            c.mul_s(2, 1, 3);
            c.sw(cvb_at(c, SP, 0x70), 0u64);
            c.mul_s(4, 1, 0);
            c.sw(cvb_at(c, SP, 0x74), 0u64);
            c.mul_s(1, 1, 6);
            c.sw(cvb_at(c, SP, 0x78), 0u64);
            c.mul_s(13, 8, 3);
            c.sw(cvb_at(c, SP, 0x4c), 0u64);
            c.mul_s(12, 8, 0);
            c.sw(cvb_at(c, SP, 0x5c), 0u64);
            c.mul_s(9, 2, 3);
            c.sw(cvb_at(c, SP, 0x6c), 0u64);
            c.mul_s(10, 4, 3);
            c.swc1(20, cvb_at(c, SP, 0x7c));
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
            c.swc1(5, cvb_at(c, SP, 0x48));
            c.add_s(7, 7, 11);
            c.swc1(0, cvb_at(c, SP, 0x44));
            c.add_s(4, 4, 13);
            c.swc1(2, cvb_at(c, SP, 0x58));
            c.sub_s(10, 10, 8);
            c.swc1(9, cvb_at(c, SP, 0x68));
            c.add_s(1, 1, 11);
            c.swc1(7, cvb_at(c, SP, 0x40));
            c.add_s(3, 3, 12);
            c.swc1(4, cvb_at(c, SP, 0x50));
            c.swc1(10, cvb_at(c, SP, 0x60));
            c.swc1(1, cvb_at(c, SP, 0x54));
            c.swc1(3, cvb_at(c, SP, 0x64));
            c.lqc2(4, r32(c, S2));
            c.lqc2(5, cvb_at(c, S2, 0x10));
            c.lqc2(6, cvb_at(c, S2, 0x20));
            c.lqc2(7, cvb_at(c, S2, 0x30));
            c.lqc2(8, r32(c, S3));
            c.lqc2(9, cvb_at(c, S3, 0x10));
            c.lqc2(10, cvb_at(c, S3, 0x20));
            c.lqc2(11, cvb_at(c, S3, 0x30));
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
            c.sqc2(12, r32(c, S2));
            c.sqc2(13, cvb_at(c, S2, 0x10));
            c.sqc2(14, cvb_at(c, S2, 0x20));
            c.sqc2(15, cvb_at(c, S2, 0x30));
            c.lwc1(0, r32(c, S1));
            c.lwc1(1, cvb_at(c, S1, 0x4));
            c.lwc1(2, cvb_at(c, S1, 0x8));
            c.neg_s(0, 0);
            c.neg_s(1, 1);
            c.swc1(20, cvb_at(c, SP, 0x4c));
            c.neg_s(2, 2);
            c.swc1(0, cvb_at(c, SP, 0x40));
            c.swc1(1, cvb_at(c, SP, 0x44));
            c.swc1(2, cvb_at(c, SP, 0x48));
            c.swc1(0, cvb_at(c, SP, 0x50));
            c.swc1(1, cvb_at(c, SP, 0x54));
            c.swc1(2, cvb_at(c, SP, 0x58));
            c.lqc2(4, r32(c, SP));
            c.lqc2(5, cvb_at(c, SP, 0x10));
            c.lqc2(6, cvb_at(c, SP, 0x20));
            c.lqc2(7, cvb_at(c, SP, 0x30));
            c.lqc2(8, cvb_at(c, SP, 0x40));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, cvb_at(c, SP, 0x30));
            c.lqc2(8, r32(c, S4));
            c.lqc2(4, r32(c, SP));
            c.lqc2(5, cvb_at(c, SP, 0x10));
            c.lqc2(6, cvb_at(c, SP, 0x20));
            c.lqc2(7, cvb_at(c, SP, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, cvb_at(c, SP, 0x50));
            c.lwc1(0, cvb_at(c, S1, 0x18));
            c.lwc1(1, cvb_at(c, S1, 0x1c));
            c.lwc1(2, cvb_at(c, S1, 0x20));
            c.neg_s(0, 0);
            c.lq(2, cvb_at(c, SP, 0x50));
            c.neg_s(1, 1);
            c.neg_s(2, 2);
            c.swc1(20, cvb_at(c, SP, 0x6c));
            c.sq(2, cvb_at(c, SP, 0x40));
            c.swc1(0, cvb_at(c, SP, 0x70));
            c.swc1(1, cvb_at(c, SP, 0x74));
            c.swc1(2, cvb_at(c, SP, 0x78));
            c.swc1(0, cvb_at(c, SP, 0x60));
            c.swc1(1, cvb_at(c, SP, 0x64));
            c.swc1(2, cvb_at(c, SP, 0x68));
            c.swc1(20, cvb_at(c, SP, 0x4c));
            c.lqc2(3, cvb_at(c, SP, 0x40));
            c.lqc2(4, cvb_at(c, SP, 0x60));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, cvb_at(c, SP, 0x80));
            c.lwc1(2, cvb_at(c, S1, 0x20));
            c.lwc1(0, cvb_at(c, S1, 0x18));
            c.lwc1(1, cvb_at(c, S1, 0x1c));
            c.lq(2, cvb_at(c, SP, 0x80));
            c.swc1(0, cvb_at(c, SP, 0x70));
            c.sq(2, cvb_at(c, SP, 0x50));
            c.swc1(1, cvb_at(c, SP, 0x74));
            c.swc1(2, cvb_at(c, SP, 0x78));
            c.swc1(20, cvb_at(c, SP, 0x7c));
            c.lqc2(3, cvb_at(c, SP, 0x40));
            c.lqc2(4, cvb_at(c, SP, 0x70));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, cvb_at(c, SP, 0x80));
            c.lq(2, cvb_at(c, SP, 0x80));
            c.sq(2, cvb_at(c, SP, 0x60));
            c.g[V1][0] = c.lw(cvb_at(c, GP, 0x1cfc));
            let t = c.g[V1][0] != 0u64;
            c.g[V0][0] = 0x4c0000;
            if t {
                break 'b_172d38;
            } else {
                c.cycles += 12;
                c.g[V1][0] = 0x500000;
                c.lq(6, r32(c, S6));
                c.g[V1][0] = sx(cvb_back(c, V1, 0xeb0));
                c.lq(4, r32(c, S5));
                c.lq(5, r32(c, V1));
                c.g[V0][0] = sx(cvb_at(c, V0, 0x5480));
                c.g[V1][0] = 0x1;
                c.sq(4, cvb_at(c, V0, 0x20));
                c.sq(6, r32(c, V0));
                c.sq(5, cvb_at(c, V0, 0x10));
                c.sw(cvb_at(c, GP, 0x1cfc), c.g[V1][0]);
                c.g[V0][0] = 0x4c0000;
                break 'b_172d38;
            }
        }
        c.cycles += 6;
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 2);
        c.mtc1(0, 1);
        c.g[A0][0] = sx(cvb_at(c, V0, 0x5480));
        c.g[A1][0] = 0u64;
        'l_172d50: loop {
            'b_172de8: {
                c.cycles += 20;
                c.mfc1(2, 2);
                c.lqc2(4, r32(c, A0));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, cvb_at(c, SP, 0x80));
                c.lq(3, cvb_at(c, SP, 0x80));
                c.sq(3, cvb_at(c, SP, 0x70));
                c.lqc2(3, cvb_at(c, SP, 0x70));
                c.lqc2(5, cvb_at(c, SP, 0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.c_lt_s(1, 0);
                let t = c.fcc();
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_172de8;
                } else {
                    c.cycles += 13;
                    c.lqc2(3, r32(c, A0));
                    c.lqc2(5, cvb_at(c, SP, 0x60));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.c_lt_s(1, 0);
                    let t = c.fcc();
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_172de8;
                    } else {
                        c.cycles += 4;
                        c.g[A1][0] = sx(cvb_at(c, A1, 0x1));
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 3i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = sx(cvb_at(c, A0, 0x10));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0017_2d50);
                            }
                            continue 'l_172d50;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0x1;
                            break 'b_172de8;
                        }
                    }
                }
            }
            c.cycles += 11;
            c.lq(16, cvb_at(c, SP, 0x110));
            c.lq(17, cvb_at(c, SP, 0x100));
            c.lq(18, cvb_at(c, SP, 0xf0));
            c.lq(19, cvb_at(c, SP, 0xe0));
            c.lq(20, cvb_at(c, SP, 0xd0));
            c.lq(21, cvb_at(c, SP, 0xc0));
            c.lq(22, cvb_at(c, SP, 0xb0));
            c.g[RA][0] = c.ld(cvb_at(c, SP, 0xa0));
            c.lwc1(20, cvb_at(c, SP, 0x120));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx(cvb_at(c, SP, 0x130));
            c.pc = t;
            return Ok(());
        }
    });
}

/// `base + off` with `base` a register's low word (the generated
/// `(c.g[R][0] as u32).wrapping_add(off)`).
#[inline(always)]
pub(crate) fn bcu_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `base - back` with `base` a register's low word (the generated
/// `wrapping_sub`), used for the gp-relative constants.
#[inline(always)]
pub(crate) fn bcu_back(c: &Cpu, reg: usize, back: u32) -> u32 {
    r32(c, reg).wrapping_sub(back)
}

/// `void BoundedCameraAlgorithmBase_Update(self)` (0x1736e0; the reference
/// names it `cTriggerAlgorithm_update`): the trigger camera's per-frame update.
///
/// The focus point is the rider's position (vtable slot 1) raised by
/// `HeightOffset` (`+0x6c`) along z and pushed by `LeadDistance` (`+0x74`)
/// along its front (slot 4); it becomes `CamTargetPos` (`+0x20`). Yaw and
/// pitch of camera-to-focus turn towards the front's yaw and pitch (wrapped),
/// the distance moves towards `IdealDist` (`+0x70`): at once when more than
/// 5000 away, else by a third of the error, or by error * k / 60 with
/// k = 0.0072 * |velocity| + 8 when k >= 20. `Position` (`+0x40`) is the focus
/// pulled back along the new direction (two `Math_SinCos`). `Heading` (`+0x50`)
/// and `Pitch` (`+0x54`) then turn towards that direction by
/// `HeadingLookRate` (`+0x64`) and `PitchLookRate` (`+0x68`) per 1/60 s.
///
/// The arithmetic is the generated body's VU0 and FPU sequence, with its
/// stack spills and cycle charges per block, so the port is exact at every
/// branch.
pub fn bounded_camera_algorithm_base_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 71;
        c.g[SP][0] = sx(bcu_back(c, SP, 0xe0));
        c.sq(16, bcu_at(c, SP, 0xa0));
        c.swc1(22, bcu_at(c, SP, 0xc0));
        c.g[S0][0] = c.g[A0][0];
        c.swc1(21, bcu_at(c, SP, 0xb8));
        c.g[A0][0] = c.g[SP][0];
        c.swc1(20, bcu_at(c, SP, 0xb0));
        c.sd(bcu_at(c, SP, 0x90), c.g[RA][0]);
        c.swc1(25, bcu_at(c, SP, 0xd8));
        c.swc1(24, bcu_at(c, SP, 0xd0));
        c.swc1(23, bcu_at(c, SP, 0xc8));
        c.mtc1(0, 22);

        // The rider's position (slot 1, +8), velocity (slot 3, +0x18) and
        // front (slot 4, +0x20) into the stack vectors at sp+0, +0x10, +0x20.
        c.g[A2][0] = c.lw(bcu_at(c, S0, 0x30));
        c.g[V0][0] = c.lw(r32(c, A2));
        c.g[A1][0] = c.lh(bcu_at(c, V0, 0x8));
        c.g[V1][0] = c.lw(bcu_at(c, V0, 0xc));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
        call_ptr(c, t, 0x0017_3728)?;

        c.g[A2][0] = c.lw(bcu_at(c, S0, 0x30));
        c.g[A0][0] = sx(bcu_at(c, SP, 0x10));
        c.g[V0][0] = c.lw(r32(c, A2));
        c.g[A1][0] = c.lh(bcu_at(c, V0, 0x18));
        c.g[V1][0] = c.lw(bcu_at(c, V0, 0x1c));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
        call_ptr(c, t, 0x0017_3744)?;

        c.g[A2][0] = c.lw(bcu_at(c, S0, 0x30));
        c.g[A0][0] = sx(bcu_at(c, SP, 0x20));
        c.g[V0][0] = c.lw(r32(c, A2));
        c.g[A1][0] = c.lh(bcu_at(c, V0, 0x20));
        c.g[V1][0] = c.lw(bcu_at(c, V0, 0x24));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
        call_ptr(c, t, 0x0017_3760)?;

        // focus = position + HeightOffset * z (the t4vect_ZAxis constant at
        // 0x4ff160), then focus += front * LeadDistance.
        c.g[V0][0] = 0x500000;
        c.lwc1(0, bcu_at(c, S0, 0x6c));
        c.g[V0][0] = sx(bcu_back(c, V0, 0xea0));
        c.mfc1(3, 0);
        c.lqc2(4, r32(c, V0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, bcu_at(c, SP, 0x40));
        c.lq(2, bcu_at(c, SP, 0x40));
        c.sq(2, bcu_at(c, SP, 0x30));
        c.lqc2(3, r32(c, SP));
        c.lqc2(4, bcu_at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, r32(c, SP));
        c.lwc1(0, bcu_at(c, S0, 0x74));
        c.mfc1(3, 0);
        c.lqc2(4, bcu_at(c, SP, 0x20));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, bcu_at(c, SP, 0x40));
        c.lq(2, bcu_at(c, SP, 0x40));
        c.sq(2, bcu_at(c, SP, 0x30));
        c.lqc2(3, r32(c, SP));
        c.lqc2(4, bcu_at(c, SP, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, r32(c, SP));
        // self->CamTargetPos = focus; toFocus = focus - Position (sp+0x30).
        c.lq(2, r32(c, SP));
        c.sq(2, bcu_at(c, S0, 0x20));
        c.lqc2(3, r32(c, SP));
        c.lqc2(4, bcu_at(c, S0, 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, bcu_at(c, SP, 0x40));
        c.lq(2, bcu_at(c, SP, 0x40));
        c.sq(2, bcu_at(c, SP, 0x30));

        // yawD = Atan2(toFocus.y, toFocus.x) (f23), with the x == 0 cases
        // (+-pi/2 or 0) handled before the atan call.
        'b_17386c: {
            'b_173868: {
                'b_173828: {
                    c.lwc1(20, bcu_at(c, SP, 0x30));
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    c.lwc1(21, bcu_at(c, SP, 0x34));
                    if t {
                        break 'b_173828;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(23, 21);
                        if t {
                            break 'b_173868;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(23, bcu_back(c, GP, 0x63f4));
                            let t = !c.fcc();
                            c.lwc1(20, bcu_at(c, SP, 0x20));
                            if t {
                                break 'b_173868;
                            } else {
                                c.cycles += 3;
                                c.lwc1(23, bcu_back(c, GP, 0x63f0));
                                break 'b_17386c;
                            }
                        }
                    }
                }
                'b_173864: {
                    c.cycles += 7;
                    c.div_s(12, 21, 20);
                    call_ptr(c, 0x0031_c228, 0x0017_3834)?;
                    c.c_lt_s(20, 22);
                    let t = !c.fcc();
                    c.mov_s(12, 0);
                    if t {
                        break 'b_173864;
                    } else {
                        'b_17385c: {
                            c.cycles += 3;
                            c.c_lt_s(22, 21);
                            let t = !c.fcc();
                            if t {
                                break 'b_17385c;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, bcu_back(c, GP, 0x63ec));
                                c.add_s(12, 12, 0);
                                break 'b_173864;
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, bcu_back(c, GP, 0x63e8));
                        c.sub_s(12, 12, 0);
                        break 'b_173864;
                    }
                }
                c.cycles += 1;
                c.mov_s(23, 12);
                break 'b_173868;
            }
            c.cycles += 1;
            c.lwc1(20, bcu_at(c, SP, 0x20));
            break 'b_17386c;
        }

        // yawC = Atan2(riderFront.y, riderFront.x) (f12).
        'b_1738e8: {
            'b_1738ac: {
                c.cycles += 5;
                c.mtc1(0, 22);
                c.c_eq_s(20, 22);
                let t = !c.fcc();
                c.lwc1(21, bcu_at(c, SP, 0x24));
                if t {
                    break 'b_1738ac;
                } else {
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let t = c.fcc();
                    c.mov_s(12, 21);
                    if t {
                        break 'b_1738e8;
                    } else {
                        c.cycles += 4;
                        c.c_le_s(22, 21);
                        c.lwc1(12, bcu_back(c, GP, 0x63e4));
                        let t = !c.fcc();
                        if t {
                            break 'b_1738e8;
                        } else {
                            c.cycles += 3;
                            c.lwc1(12, bcu_back(c, GP, 0x63e0));
                            break 'b_1738e8;
                        }
                    }
                }
            }
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, 0x0031_c228, 0x0017_38b8)?;
            c.c_lt_s(20, 22);
            let t = !c.fcc();
            c.mov_s(12, 0);
            if t {
                break 'b_1738e8;
            } else {
                'b_1738e0: {
                    c.cycles += 3;
                    c.c_lt_s(22, 21);
                    let t = !c.fcc();
                    if t {
                        break 'b_1738e0;
                    } else {
                        c.cycles += 3;
                        c.lwc1(0, bcu_back(c, GP, 0x63dc));
                        c.add_s(12, 12, 0);
                        break 'b_1738e8;
                    }
                }
                c.cycles += 2;
                c.lwc1(0, bcu_back(c, GP, 0x63d8));
                c.sub_s(12, 12, 0);
                break 'b_1738e8;
            }
        }

        // f12 = diff = Wrap(yawC) - yawD (f23), with Wrap(v) = v - floor(v / 2pi
        // + 1/2) * 2pi (the floor by cvt.w.s).
        'b_17391c: {
            c.cycles += 10;
            c.lwc1(2, bcu_back(c, GP, 0x63d4));
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 4);
            c.mul_s(0, 12, 2);
            c.add_s(0, 0, 4);
            c.cvt_w_s(1, 0);
            c.cvt_s_w(1, 1);
            c.c_lt_s(0, 1);
            let t = !c.fcc();
            if t {
                break 'b_17391c;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.sub_s(1, 1, 0);
                break 'b_17391c;
            }
        }
        'b_17395c: {
            c.cycles += 12;
            c.lwc1(3, bcu_back(c, GP, 0x63d0));
            c.mul_s(0, 1, 3);
            c.sub_s(0, 12, 0);
            c.sub_s(12, 0, 23);
            c.mul_s(1, 12, 2);
            c.add_s(1, 1, 4);
            c.cvt_w_s(2, 1);
            c.cvt_s_w(2, 2);
            c.c_lt_s(1, 2);
            if !c.fcc() {
                c.mul_s(0, 2, 3);
                break 'b_17395c;
            } else {
                c.cycles += 4;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.sub_s(2, 2, 0);
                c.mul_s(0, 2, 3);
                break 'b_17395c;
            }
        }

        // f25 = yawN = yawD + Wrap(diff) * HeadingAdjustRate / 60. The distance
        // f24 = len3 + dd when |dd| > 5000 (dd = IdealDist - len3); otherwise
        // see the k branch below.
        'b_173a24: {
            'b_1739bc: {
                c.cycles += 22;
                c.lwc1(4, bcu_at(c, S0, 0x5c));
                c.lwc1(3, bcu_at(c, S0, 0x70));
                c.lqc2(3, bcu_at(c, SP, 0x30));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(2, 22);
                c.mtc1(2, 6);
                c.lwc1(7, bcu_back(c, GP, 0x63cc));
                c.sub_s(0, 12, 0);
                c.lwc1(2, bcu_back(c, GP, 0x63c8));
                c.sub_s(5, 3, 6);
                c.abs_s(1, 5);
                c.mul_s(0, 0, 4);
                c.c_lt_s(2, 1);
                c.mul_s(0, 0, 7);
                let t = !c.fcc();
                c.add_s(25, 23, 0);
                if t {
                    break 'b_1739bc;
                } else {
                    c.cycles += 2;
                    c.add_s(24, 6, 5);
                    break 'b_173a24;
                }
            }
            // The k branch of the distance: k = |riderVel| * 0.0072 + 8, and
            // x = dd * k / 60 when k >= 20, else dd / 3; f24 = len3 + x.
            'b_173a20: {
                'b_173a18: {
                    c.cycles += 20;
                    c.lwc1(2, bcu_back(c, GP, 0x63c4));
                    c.lqc2(3, bcu_at(c, SP, 0x10));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(2, 22);
                    c.mtc1(2, 0);
                    c.g[AT][0] = 0x4100_0000;
                    c.mtc1(1, 3);
                    c.mul_s(0, 0, 2);
                    c.g[AT][0] = 0x41a0_0000;
                    c.mtc1(1, 1);
                    c.add_s(0, 0, 3);
                    c.c_le_s(1, 0);
                    let t = !c.fcc();
                    if t {
                        break 'b_173a18;
                    } else {
                        c.cycles += 3;
                        c.mul_s(0, 5, 0);
                        c.mul_s(0, 0, 7);
                        break 'b_173a20;
                    }
                }
                c.cycles += 2;
                c.lwc1(0, bcu_back(c, GP, 0x63c0));
                c.mul_s(0, 5, 0);
                break 'b_173a20;
            }
            c.cycles += 1;
            c.add_s(24, 6, 0);
            break 'b_173a24;
        }

        // pitchD = Atan2(-toFocus.z, Len2(toFocus.x, toFocus.y)) (f23).
        'b_173ad0: {
            'b_173ac8: {
                'b_173a88: {
                    c.cycles += 14;
                    c.lwc1(0, bcu_at(c, SP, 0x30));
                    c.lwc1(1, bcu_at(c, SP, 0x34));
                    c.mul_s(2, 0, 0);
                    c.lwc1(3, bcu_at(c, SP, 0x38));
                    c.mul_s(4, 1, 1);
                    c.swc1(0, bcu_at(c, SP, 0x40));
                    c.mtc1(0, 22);
                    c.neg_s(21, 3);
                    c.swc1(1, bcu_at(c, SP, 0x44));
                    c.add_s(2, 2, 4);
                    c.sqrt_s(20, 2);
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    if t {
                        break 'b_173a88;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(23, 21);
                        if t {
                            break 'b_173ac8;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(23, bcu_back(c, GP, 0x63bc));
                            let t = !c.fcc();
                            c.lwc1(0, bcu_at(c, SP, 0x20));
                            if t {
                                break 'b_173ac8;
                            } else {
                                c.cycles += 3;
                                c.lwc1(23, bcu_back(c, GP, 0x63b8));
                                c.lwc1(1, bcu_at(c, SP, 0x24));
                                break 'b_173ad0;
                            }
                        }
                    }
                }
                'b_173ac4: {
                    c.cycles += 7;
                    c.div_s(12, 21, 20);
                    call_ptr(c, 0x0031_c228, 0x0017_3a94)?;
                    c.c_lt_s(20, 22);
                    let t = !c.fcc();
                    c.mov_s(3, 0);
                    if t {
                        break 'b_173ac4;
                    } else {
                        'b_173abc: {
                            c.cycles += 3;
                            c.c_lt_s(22, 21);
                            let t = !c.fcc();
                            if t {
                                break 'b_173abc;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, bcu_back(c, GP, 0x63b4));
                                c.add_s(3, 3, 0);
                                break 'b_173ac4;
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, bcu_back(c, GP, 0x63b0));
                        c.sub_s(3, 3, 0);
                        break 'b_173ac4;
                    }
                }
                c.cycles += 1;
                c.mov_s(23, 3);
                break 'b_173ac8;
            }
            c.cycles += 2;
            c.lwc1(0, bcu_at(c, SP, 0x20));
            c.lwc1(1, bcu_at(c, SP, 0x24));
            break 'b_173ad0;
        }

        // pitchC = Atan2(riderFront.z, Len2(riderFront.x, riderFront.y)) (f12).
        'b_173b6c: {
            'b_173b68: {
                'b_173b28: {
                    c.cycles += 11;
                    c.mul_s(2, 0, 0);
                    c.swc1(0, bcu_at(c, SP, 0x40));
                    c.mul_s(3, 1, 1);
                    c.swc1(1, bcu_at(c, SP, 0x44));
                    c.mtc1(0, 22);
                    c.lwc1(21, bcu_at(c, SP, 0x28));
                    c.add_s(2, 2, 3);
                    c.sqrt_s(20, 2);
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    if t {
                        break 'b_173b28;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(12, 21);
                        if t {
                            break 'b_173b68;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(12, bcu_back(c, GP, 0x63ac));
                            let t = !c.fcc();
                            c.lwc1(1, bcu_at(c, S0, 0x58));
                            if t {
                                break 'b_173b6c;
                            } else {
                                c.cycles += 3;
                                c.lwc1(12, bcu_back(c, GP, 0x63a8));
                                break 'b_173b6c;
                            }
                        }
                    }
                }
                'b_173b64: {
                    c.cycles += 7;
                    c.div_s(12, 21, 20);
                    call_ptr(c, 0x0031_c228, 0x0017_3b34)?;
                    c.c_lt_s(20, 22);
                    let t = !c.fcc();
                    c.mov_s(1, 0);
                    if t {
                        break 'b_173b64;
                    } else {
                        'b_173b5c: {
                            c.cycles += 3;
                            c.c_lt_s(22, 21);
                            let t = !c.fcc();
                            if t {
                                break 'b_173b5c;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, bcu_back(c, GP, 0x63a4));
                                c.add_s(1, 1, 0);
                                break 'b_173b64;
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, bcu_back(c, GP, 0x63a0));
                        c.sub_s(1, 1, 0);
                        break 'b_173b64;
                    }
                }
                c.cycles += 1;
                c.mov_s(12, 1);
                break 'b_173b68;
            }
            c.cycles += 1;
            c.lwc1(1, bcu_at(c, S0, 0x58));
            break 'b_173b6c;
        }

        // tp = pitchC * -PitchMultiplier + PitchOffset / 60 (f2), wrapped
        // (floor of the scaled value, as above).
        'b_173bb8: {
            c.cycles += 16;
            c.lwc1(3, bcu_back(c, GP, 0x639c));
            c.neg_s(1, 1);
            c.lwc1(2, bcu_at(c, S0, 0x78));
            c.lwc1(0, bcu_back(c, GP, 0x6398));
            c.mul_s(2, 2, 3);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 4);
            c.mul_s(1, 12, 1);
            c.add_s(3, 1, 2);
            c.mul_s(0, 3, 0);
            c.add_s(0, 0, 4);
            c.cvt_w_s(2, 0);
            c.cvt_s_w(2, 2);
            c.c_lt_s(0, 2);
            let t = !c.fcc();
            if t {
                break 'b_173bb8;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.sub_s(2, 2, 0);
                break 'b_173bb8;
            }
        }

        // pitchN from pitchC and pitchD; two Math_SinCos calls (yawN in f25,
        // pitchN in f20) into the stack, then dir = (sc3 * sc1, sc3 * sc0,
        // -sc2, 0), n = focus - dir * dist (f24), and Position (+0x40) = n.
        'b_173cfc: {
            'b_173cf8: {
                'b_173cbc: {
                    c.cycles += 54;
                    c.lwc1(20, bcu_back(c, GP, 0x6394));
                    c.g[A0][0] = sx(bcu_at(c, SP, 0x80));
                    c.lwc1(1, bcu_at(c, S0, 0x60));
                    c.g[A1][0] = sx(bcu_at(c, SP, 0x84));
                    c.mul_s(20, 2, 20);
                    c.lwc1(0, bcu_back(c, GP, 0x6390));
                    c.mov_s(12, 25);
                    c.mtc1(0, 22);
                    c.sub_s(20, 3, 20);
                    c.sub_s(20, 20, 23);
                    c.mul_s(20, 20, 1);
                    c.mul_s(20, 20, 0);
                    c.add_s(20, 23, 20);
                    call_ptr(c, 0x0031_be50, 0x0017_3bf0)?;
                    c.g[A0][0] = sx(bcu_at(c, SP, 0x88));
                    c.g[A1][0] = sx(bcu_at(c, SP, 0x8c));
                    c.mov_s(12, 20);
                    call_ptr(c, 0x0031_be50, 0x0017_3c00)?;
                    c.lwc1(2, bcu_at(c, SP, 0x8c));
                    c.lwc1(0, bcu_at(c, SP, 0x84));
                    c.lwc1(3, bcu_at(c, SP, 0x80));
                    c.lwc1(1, bcu_at(c, SP, 0x88));
                    c.mul_s(0, 2, 0);
                    c.mul_s(2, 2, 3);
                    c.swc1(22, bcu_at(c, SP, 0x4c));
                    c.neg_s(1, 1);
                    c.swc1(0, bcu_at(c, SP, 0x40));
                    c.swc1(2, bcu_at(c, SP, 0x44));
                    c.swc1(1, bcu_at(c, SP, 0x48));
                    c.mfc1(3, 24);
                    c.lqc2(4, bcu_at(c, SP, 0x40));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, bcu_at(c, SP, 0x70));
                    c.lq(2, bcu_at(c, SP, 0x70));
                    c.sq(2, bcu_at(c, SP, 0x60));
                    c.lqc2(3, r32(c, SP));
                    c.lqc2(4, bcu_at(c, SP, 0x60));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, bcu_at(c, SP, 0x70));
                    c.lq(2, bcu_at(c, SP, 0x70));
                    c.sq(2, bcu_at(c, S0, 0x40));
                    c.sq(2, bcu_at(c, SP, 0x50));
                    // n = focus - Position (sp+0x60), then n at sp+0x50.
                    c.lqc2(3, r32(c, SP));
                    c.lqc2(4, bcu_at(c, S0, 0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, bcu_at(c, SP, 0x60));
                    c.lq(2, bcu_at(c, SP, 0x60));
                    c.sq(2, bcu_at(c, SP, 0x50));
                    // yawE = Atan2(n.y, n.x) (f12).
                    c.lwc1(20, bcu_at(c, SP, 0x50));
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    c.lwc1(21, bcu_at(c, SP, 0x54));
                    if t {
                        break 'b_173cbc;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(12, 21);
                        if t {
                            break 'b_173cf8;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(12, bcu_back(c, GP, 0x638c));
                            let t = !c.fcc();
                            c.lwc1(7, bcu_at(c, S0, 0x50));
                            if t {
                                break 'b_173cfc;
                            } else {
                                c.cycles += 3;
                                c.lwc1(12, bcu_back(c, GP, 0x6388));
                                break 'b_173cfc;
                            }
                        }
                    }
                }
                c.cycles += 7;
                c.div_s(12, 21, 20);
                call_ptr(c, 0x0031_c228, 0x0017_3cc8)?;
                c.c_lt_s(20, 22);
                let t = !c.fcc();
                c.mov_s(12, 0);
                if t {
                    break 'b_173cf8;
                } else {
                    'b_173cf0: {
                        c.cycles += 3;
                        c.c_lt_s(22, 21);
                        let t = !c.fcc();
                        if t {
                            break 'b_173cf0;
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, bcu_back(c, GP, 0x6384));
                            c.add_s(12, 12, 0);
                            break 'b_173cf8;
                        }
                    }
                    c.cycles += 2;
                    c.lwc1(0, bcu_back(c, GP, 0x6380));
                    c.sub_s(12, 12, 0);
                    break 'b_173cf8;
                }
            }
            c.cycles += 1;
            c.lwc1(7, bcu_at(c, S0, 0x50));
            break 'b_173cfc;
        }

        // Heading wrap: Wrap(yawE - Heading) (f1).
        'b_173d34: {
            c.cycles += 11;
            c.lwc1(0, bcu_back(c, GP, 0x637c));
            c.sub_s(12, 12, 7);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 1);
            c.mul_s(0, 12, 0);
            c.add_s(0, 0, 1);
            c.cvt_w_s(1, 0);
            c.cvt_s_w(1, 1);
            c.c_lt_s(0, 1);
            let t = !c.fcc();
            if t {
                break 'b_173d34;
            } else {
                c.cycles += 3;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.sub_s(1, 1, 0);
                break 'b_173d34;
            }
        }

        // Heading (+0x50) += Wrap(yawE - Heading) * HeadingLookRate / 60, and
        // pitchE = Atan2(n.z, Len2(n.x, n.y)) turns Pitch in the tail below.
        'b_173dfc: {
            'b_173df8: {
                'b_173df4: {
                    'b_173db4: {
                        c.cycles += 21;
                        c.lwc1(0, bcu_back(c, GP, 0x6378));
                        c.lwc1(5, bcu_at(c, S0, 0x64));
                        c.mul_s(0, 1, 0);
                        c.lwc1(2, bcu_at(c, SP, 0x54));
                        c.lwc1(1, bcu_at(c, SP, 0x50));
                        c.lwc1(6, bcu_back(c, GP, 0x6374));
                        c.mul_s(4, 2, 2);
                        c.sub_s(0, 12, 0);
                        c.swc1(1, bcu_at(c, SP, 0x60));
                        c.mul_s(3, 1, 1);
                        c.swc1(2, bcu_at(c, SP, 0x64));
                        c.mtc1(0, 22);
                        c.mul_s(0, 0, 5);
                        c.lwc1(21, bcu_at(c, SP, 0x58));
                        c.add_s(3, 3, 4);
                        c.sqrt_s(20, 3);
                        c.mul_s(0, 0, 6);
                        c.c_eq_s(20, 22);
                        c.add_s(0, 7, 0);
                        let t = !c.fcc();
                        c.swc1(0, bcu_at(c, S0, 0x50));
                        if t {
                            break 'b_173db4;
                        } else {
                            c.cycles += 4;
                            c.c_eq_s(21, 22);
                            let t = c.fcc();
                            c.mov_s(0, 21);
                            if t {
                                break 'b_173df4;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(22, 21);
                                c.lwc1(0, bcu_back(c, GP, 0x6370));
                                let t = !c.fcc();
                                c.lwc1(1, bcu_at(c, S0, 0x54));
                                if t {
                                    break 'b_173df8;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, bcu_back(c, GP, 0x636c));
                                    c.lwc1(3, bcu_at(c, S0, 0x68));
                                    break 'b_173dfc;
                                }
                            }
                        }
                    }
                    'b_173df0: {
                        c.cycles += 7;
                        c.div_s(12, 21, 20);
                        call_ptr(c, 0x0031_c228, 0x0017_3dc0)?;
                        c.c_lt_s(20, 22);
                        let t = !c.fcc();
                        c.mov_s(12, 0);
                        if t {
                            break 'b_173df0;
                        } else {
                            'b_173de8: {
                                c.cycles += 3;
                                c.c_lt_s(22, 21);
                                let t = !c.fcc();
                                if t {
                                    break 'b_173de8;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, bcu_back(c, GP, 0x6368));
                                    c.add_s(12, 12, 0);
                                    break 'b_173df0;
                                }
                            }
                            c.cycles += 2;
                            c.lwc1(0, bcu_back(c, GP, 0x6364));
                            c.sub_s(12, 12, 0);
                            break 'b_173df0;
                        }
                    }
                    c.cycles += 1;
                    c.mov_s(0, 12);
                    break 'b_173df4;
                }
                c.cycles += 1;
                c.lwc1(1, bcu_at(c, S0, 0x54));
                break 'b_173df8;
            }
            c.cycles += 1;
            c.lwc1(3, bcu_at(c, S0, 0x68));
            break 'b_173dfc;
        }

        // Pitch (+0x54) += (pitchE - Pitch) * PitchLookRate / 60.
        c.cycles += 16;
        c.sub_s(0, 0, 1);
        c.lwc1(2, bcu_back(c, GP, 0x6360));
        c.g[RA][0] = c.ld(bcu_at(c, SP, 0x90));
        c.lwc1(25, bcu_at(c, SP, 0xd8));
        c.mul_s(0, 0, 3);
        c.lwc1(24, bcu_at(c, SP, 0xd0));
        c.lwc1(23, bcu_at(c, SP, 0xc8));
        c.lwc1(22, bcu_at(c, SP, 0xc0));
        c.mul_s(0, 0, 2);
        c.lwc1(21, bcu_at(c, SP, 0xb8));
        c.lwc1(20, bcu_at(c, SP, 0xb0));
        c.add_s(1, 1, 0);
        c.swc1(1, bcu_at(c, S0, 0x54));
        c.lq(16, bcu_at(c, SP, 0xa0));
        c.g[SP][0] = sx(bcu_at(c, SP, 0xe0));
        ret(c);
        Ok(())
    });
}

/// `BoundedCameraAlgorithmBase::~BoundedCameraAlgorithmBase(this, flags)`
/// (0x1732b8): stores the base vtable word 0x45bde0 at `this+0x10`, then when
/// bit 0 of `flags` is set frees `this` through `operator delete`
/// (0x317e50, with `this` still in `a0`). Leaves `v0` = 0x45bde0 and
/// `a1` = `flags & 1`, as the generated body does.
pub fn bounded_camera_algorithm_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0045_bde0);
        c.g[A1][0] &= 0x1;
        let this = Addr(c.g[A0][0] as u32);
        let base = c.g[V0][0];
        this.off(0x10).sw(c, base);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            Callee {
                addr: 0x0031_7e50,
                f: ee::s10::f_00317e50,
            }
            .call(c, 0x0017_32dc)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraTriggerMan_ctor` (0x16c800): the manager's constructor, returns to
/// the static-init code at 0x176228.
pub(crate) const CTM_CTOR: Callee = Callee {
    addr: 0x0016_c800,
    f: ee::s02::f_0016c800,
};

/// `CameraTriggerMan_dtor` (0x16c888): the manager's destructor, returns to
/// the static-init code at 0x176240.
pub(crate) const CTM_DTOR: Callee = Callee {
    addr: 0x0016_c888,
    f: ee::s02::f_0016c888,
};

/// Effective address `rs + off` as the generated code forms it
/// (`(c.g[rs][0] as u32).wrapping_add(off)`).
#[inline(always)]
pub(crate) fn cts_ea(c: &Cpu, rs: usize, off: i32) -> u32 {
    (c.g[rs][0] as u32).wrapping_add(off as u32)
}

/// `void CameraTriggerMan_StaticInitDestroy(int init, uint prio)` (0x175a20),
/// the static constructor and destructor of the camera trigger manager. Only
/// `prio == 0xffff` does anything: `init != 0` writes the initial values of
/// the trigger-manager globals (the float tables at 0x4c54b0..0x4c5820, read
/// from the constant pool at gp-0x6304..gp-0x61f4) and then constructs the
/// manager at `gCamTriggerMan`; `init == 0` destroys it. Other priorities
/// return at once.
pub fn camera_trigger_man_static_init_destroy(c: &mut Cpu) {
    guest(c, |c: &mut Cpu| -> Flow {
        'b_176244: {
            'b_176240: {
                c.cycles += 13;
                c.g[SP][0] = sx(cts_ea(c, SP, -0xa0));
                c.g[V0][0] = 0xffff;
                c.sq(16, cts_ea(c, SP, 0x90));
                c.sq(17, cts_ea(c, SP, 0x80));
                c.sq(18, cts_ea(c, SP, 0x70));
                c.sq(19, cts_ea(c, SP, 0x60));
                c.sq(20, cts_ea(c, SP, 0x50));
                c.sq(21, cts_ea(c, SP, 0x40));
                c.sq(22, cts_ea(c, SP, 0x30));
                c.sq(23, cts_ea(c, SP, 0x20));
                c.sq(30, cts_ea(c, SP, 0x10));
                let t = c.g[A1][0] != c.g[V0][0];
                c.sd(cts_ea(c, SP, 0), c.g[RA][0]);
                if t {
                    break 'b_176240;
                } else {
                    'b_176230: {
                        c.cycles += 2;
                        let t = c.g[A0][0] == 0u64;
                        c.g[T5][0] = 0x4c0000;
                        if t {
                            break 'b_176230;
                        } else {
                            c.cycles += 501;
                            c.lwc1(9, cts_ea(c, GP, -0x6304));
                            c.lwc1(10, cts_ea(c, GP, -0x6300));
                            c.g[T6][0] = 0x4c0000;
                            c.lwc1(11, cts_ea(c, GP, -0x62fc));
                            c.g[T7][0] = 0x4c0000;
                            c.lwc1(7, cts_ea(c, GP, -0x62f8));
                            c.g[V0][0] = sx(cts_ea(c, T7, 0x54e0));
                            c.swc1(9, cts_ea(c, T5, 0x54c0));
                            c.g[S1][0] = 0x4c0000;
                            c.swc1(10, cts_ea(c, T6, 0x54d0));
                            c.g[S2][0] = 0x4c0000;
                            c.swc1(11, cts_ea(c, T7, 0x54e0));
                            c.g[S3][0] = 0x4c0000;
                            c.lwc1(8, cts_ea(c, GP, -0x62f4));
                            c.g[A0][0] = sx(cts_ea(c, S3, 0x5510));
                            c.lwc1(4, cts_ea(c, GP, -0x62f0));
                            c.g[S5][0] = 0x4c0000;
                            c.lwc1(5, cts_ea(c, GP, -0x62ec));
                            c.g[S4][0] = 0x4c0000;
                            c.swc1(7, cts_ea(c, V0, 0xc));
                            c.g[T3][0] = 0x4c0000;
                            c.lwc1(2, cts_ea(c, GP, -0x62e8));
                            c.g[A1][0] = sx(cts_ea(c, S5, 0x54b0));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.g[A3][0] = sx(cts_ea(c, T5, 0x54c0));
                            c.swc1(8, cts_ea(c, S1, 0x54f0));
                            c.g[T1][0] = sx(cts_ea(c, T6, 0x54d0));
                            c.swc1(4, cts_ea(c, S2, 0x5500));
                            c.g[T0][0] = sx(cts_ea(c, S2, 0x5500));
                            c.swc1(5, cts_ea(c, S3, 0x5510));
                            c.g[V1][0] = sx(cts_ea(c, S1, 0x54f0));
                            c.lwc1(3, cts_ea(c, GP, -0x62e4));
                            c.g[S2][0] = 0x4c0000;
                            c.lwc1(1, cts_ea(c, GP, -0x62e0));
                            c.g[S1][0] = sx(cts_ea(c, S2, 0x5590));
                            c.swc1(2, cts_ea(c, A0, 0xc));
                            c.g[S3][0] = 0x4c0000;
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 6);
                            c.g[S2][0] = sx(cts_ea(c, S3, 0x55a0));
                            c.swc1(3, cts_ea(c, S4, 0x5520));
                            c.g[FP][0] = 0x4c0000;
                            c.swc1(1, cts_ea(c, T3, 0x5530));
                            c.g[A2][0] = sx(cts_ea(c, S4, 0x5520));
                            c.swc1(0, cts_ea(c, S5, 0x54b0));
                            c.g[S3][0] = sx(cts_ea(c, FP, 0x55b0));
                            c.swc1(6, cts_ea(c, A1, 0xc));
                            c.g[FP][0] = 0x4c0000;
                            c.sw(cts_ea(c, A3, 0xc), 0u64);
                            c.g[T4][0] = sx(cts_ea(c, T3, 0x5530));
                            c.lwc1(17, cts_ea(c, GP, -0x62dc));
                            c.g[T5][0] = 0x4c0000;
                            c.lwc1(18, cts_ea(c, GP, -0x62d8));
                            c.g[T2][0] = sx(cts_ea(c, T5, 0x5540));
                            c.sw(cts_ea(c, T1, 0xc), 0u64);
                            c.g[S7][0] = 0x4c0000;
                            c.lwc1(13, cts_ea(c, GP, -0x62d4));
                            c.g[T7][0] = sx(cts_ea(c, S7, 0x5570));
                            c.lwc1(14, cts_ea(c, GP, -0x62d0));
                            c.g[T9][0] = 0x4c0000;
                            c.swc1(17, cts_ea(c, V0, 0x4));
                            c.g[T5][0] = sx(cts_ea(c, T9, 0x5550));
                            c.swc1(18, cts_ea(c, V0, 0x8));
                            c.g[T8][0] = 0x4c0000;
                            c.swc1(13, cts_ea(c, V1, 0x4));
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(14, cts_ea(c, V1, 0x8));
                            c.g[S5][0] = sx(cts_ea(c, V0, 0x55c0));
                            c.sw(cts_ea(c, V1, 0xc), 0u64);
                            c.g[T6][0] = sx(cts_ea(c, T8, 0x5560));
                            c.lwc1(15, cts_ea(c, GP, -0x62cc));
                            c.g[V1][0] = 0x4c0000;
                            c.lwc1(16, cts_ea(c, GP, -0x62c8));
                            c.g[V0][0] = sx(cts_ea(c, V1, 0x5600));
                            c.sw(cts_ea(c, T0, 0xc), 0u64);
                            c.g[V1][0] = sx(cts_ea(c, FP, 0x5620));
                            c.lwc1(12, cts_ea(c, GP, -0x62c4));
                            c.g[FP][0] = 0x4c0000;
                            c.swc1(15, cts_ea(c, A0, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(16, cts_ea(c, A0, 0x8));
                            c.g[S0][0] = sx(cts_ea(c, S6, 0x5580));
                            c.swc1(12, cts_ea(c, A2, 0x4));
                            c.g[A0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, A2, 0xc));
                            c.g[S4][0] = sx(cts_ea(c, A0, 0x5610));
                            c.swc1(0, cts_ea(c, A1, 0x4));
                            c.g[A0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, A1, 0x8));
                            c.g[T3][0] = sx(cts_ea(c, A0, 0x5630));
                            c.swc1(0, cts_ea(c, A3, 0x4));
                            c.g[A1][0] = sx(cts_ea(c, FP, 0x5650));
                            c.swc1(0, cts_ea(c, A3, 0x8));
                            c.g[FP][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, T1, 0x4));
                            c.g[A0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, T1, 0x8));
                            c.swc1(0, cts_ea(c, T0, 0x4));
                            c.g[T1][0] = sx(cts_ea(c, FP, 0x5680));
                            c.swc1(0, cts_ea(c, T0, 0x8));
                            c.g[FP][0] = 0x4c0000;
                            c.lwc1(19, cts_ea(c, GP, -0x62c0));
                            c.swc1(0, cts_ea(c, A2, 0x8));
                            c.lwc1(2, cts_ea(c, GP, -0x62bc));
                            c.g[A2][0] = sx(cts_ea(c, A0, 0x5660));
                            c.swc1(19, cts_ea(c, T4, 0x4));
                            c.g[A0][0] = 0x4c0000;
                            c.lwc1(1, cts_ea(c, GP, -0x62b8));
                            c.g[A3][0] = sx(cts_ea(c, A0, 0x56b0));
                            c.swc1(2, cts_ea(c, FP, 0x5540));
                            c.g[A0][0] = 0x4c0000;
                            c.lwc1(8, cts_ea(c, GP, -0x62b4));
                            c.g[FP][0] = 0x4c0000;
                            c.swc1(1, cts_ea(c, T2, 0xc));
                            c.g[T0][0] = sx(cts_ea(c, A0, 0x5710));
                            c.lwc1(3, cts_ea(c, GP, -0x62b0));
                            c.g[A0][0] = 0x4c0000;
                            c.swc1(8, cts_ea(c, T7, 0xc));
                            c.lwc1(4, cts_ea(c, GP, -0x62ac));
                            c.lwc1(5, cts_ea(c, GP, -0x62a8));
                            c.swc1(3, cts_ea(c, T4, 0x8));
                            c.sw(cts_ea(c, T4, 0xc), 0u64);
                            c.swc1(4, cts_ea(c, T2, 0x4));
                            c.g[T4][0] = sx(cts_ea(c, FP, 0x5730));
                            c.swc1(5, cts_ea(c, T2, 0x8));
                            c.g[FP][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, T9, 0x5550));
                            c.g[T2][0] = sx(cts_ea(c, A0, 0x5740));
                            c.swc1(0, cts_ea(c, T5, 0xc));
                            c.g[A0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, T8, 0x5560));
                            c.sw(cts_ea(c, T6, 0xc), 0u64);
                            c.lwc1(7, cts_ea(c, GP, -0x62a4));
                            c.swc1(0, cts_ea(c, S7, 0x5570));
                            c.swc1(7, cts_ea(c, T7, 0x8));
                            c.swc1(0, cts_ea(c, S6, 0x5580));
                            c.swc1(0, cts_ea(c, S0, 0xc));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S6, 0x5590));
                            c.swc1(0, cts_ea(c, S1, 0xc));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S6, 0x55a0));
                            c.sw(cts_ea(c, S2, 0xc), 0u64);
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, FP, 0x55b0));
                            c.swc1(0, cts_ea(c, T5, 0x4));
                            c.g[FP][0] = 0xffffffffffff0000;
                            c.swc1(0, cts_ea(c, T5, 0x8));
                            c.g[FP][0] = c.g[FP][0] | 0xff00;
                            c.sw(cts_ea(c, T6, 0x4), 0u64);
                            c.g[T5][0] = 0x4c0000;
                            c.sw(cts_ea(c, T6, 0x8), 0u64);
                            c.swc1(8, cts_ea(c, T7, 0x4));
                            c.g[T6][0] = sx(cts_ea(c, S6, 0x57d0));
                            c.swc1(0, cts_ea(c, S0, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S0, 0x8));
                            c.swc1(0, cts_ea(c, S1, 0x4));
                            c.g[S0][0] = sx(cts_ea(c, T5, 0x57c0));
                            c.swc1(0, cts_ea(c, S1, 0x8));
                            c.g[T5][0] = 0x4c0000;
                            c.sw(cts_ea(c, S2, 0x4), 0u64);
                            c.sw(cts_ea(c, S2, 0x8), 0u64);
                            c.g[S2][0] = sx(cts_ea(c, A0, 0x57b0));
                            c.sw(cts_ea(c, S3, 0x4), 0u64);
                            c.sw(cts_ea(c, S3, 0x8), 0u64);
                            c.g[A0][0] = 0x4c0000;
                            c.sw(cts_ea(c, S3, 0xc), 0u64);
                            c.g[A0][0] = sx(cts_ea(c, A0, 0x55f0));
                            c.lwc1(2, cts_ea(c, GP, -0x62a0));
                            c.g[S3][0] = 0x4c0000;
                            c.swc1(6, cts_ea(c, A0, 0xc));
                            c.lwc1(1, cts_ea(c, GP, -0x629c));
                            c.g[A0][0] = 0x4c0000;
                            c.swc1(2, cts_ea(c, V0, 0xc));
                            c.g[T7][0] = sx(cts_ea(c, A0, 0x57f0));
                            c.swc1(1, cts_ea(c, T5, 0x5620));
                            c.swc1(0, cts_ea(c, S6, 0x55c0));
                            c.g[T5][0] = sx(cts_ea(c, S3, 0x57e0));
                            c.swc1(0, cts_ea(c, S5, 0xc));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S6, 0x55d0));
                            c.g[S3][0] = 0x4c0000;
                            c.g[A0][0] = c.g[S6][0];
                            c.g[S1][0] = sx(cts_ea(c, S3, 0x5800));
                            c.g[A0][0] = sx(cts_ea(c, A0, 0x55d0));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(6, cts_ea(c, A0, 0xc));
                            c.g[S3][0] = sx(cts_ea(c, S6, 0x5820));
                            c.g[A0][0] = 0x4c0000;
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, A0, 0x55e0));
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x55e0));
                            c.swc1(6, cts_ea(c, S6, 0xc));
                            c.g[A0][0] = 0x4c0000;
                            c.g[S6][0] = 0x4c0000;
                            c.lwc1(8, cts_ea(c, GP, -0x6298));
                            c.swc1(0, cts_ea(c, S6, 0x55f0));
                            c.g[A0][0] = sx(cts_ea(c, A0, 0x5830));
                            c.lwc1(7, cts_ea(c, GP, -0x6294));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S6, 0x5600));
                            c.swc1(8, cts_ea(c, V0, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(7, cts_ea(c, V0, 0x8));
                            c.lwc1(5, cts_ea(c, GP, -0x6290));
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V0, 0x5610));
                            c.lwc1(4, cts_ea(c, GP, -0x628c));
                            c.g[V0][0] = 0x4c0000;
                            c.lwc1(3, cts_ea(c, GP, -0x6288));
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x55d0));
                            c.swc1(5, cts_ea(c, S4, 0x8));
                            c.sw(cts_ea(c, S4, 0xc), 0u64);
                            c.swc1(4, cts_ea(c, V1, 0x4));
                            c.swc1(3, cts_ea(c, V1, 0x8));
                            c.sw(cts_ea(c, V1, 0xc), 0u64);
                            c.g[V1][0] = 0x4c0000;
                            c.lwc1(2, cts_ea(c, GP, -0x6284));
                            c.swc1(0, cts_ea(c, V1, 0x5630));
                            c.swc1(0, cts_ea(c, T3, 0xc));
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S6, 0x5640));
                            c.g[V1][0] = sx(cts_ea(c, V1, 0x55e0));
                            c.swc1(0, cts_ea(c, S5, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S5, 0x8));
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x55f0));
                            c.swc1(6, cts_ea(c, V0, 0x4));
                            c.swc1(0, cts_ea(c, V0, 0x8));
                            c.swc1(6, cts_ea(c, V1, 0x4));
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V1, 0x8));
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x5640));
                            c.swc1(6, cts_ea(c, S6, 0x4));
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S6, 0x8));
                            c.swc1(0, cts_ea(c, S4, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, T3, 0x4));
                            c.swc1(0, cts_ea(c, T3, 0x8));
                            c.swc1(0, cts_ea(c, V0, 0x4));
                            c.lwc1(1, cts_ea(c, GP, -0x6280));
                            c.swc1(2, cts_ea(c, A2, 0xc));
                            c.lwc1(3, cts_ea(c, GP, -0x627c));
                            c.swc1(1, cts_ea(c, T1, 0xc));
                            c.swc1(3, cts_ea(c, V0, 0x8));
                            c.sw(cts_ea(c, V0, 0xc), 0u64);
                            c.lwc1(4, cts_ea(c, GP, -0x6278));
                            c.g[V0][0] = 0x4c0000;
                            c.lwc1(5, cts_ea(c, GP, -0x6274));
                            c.swc1(0, cts_ea(c, V1, 0x5650));
                            c.swc1(4, cts_ea(c, A1, 0x4));
                            c.g[V1][0] = c.g[V0][0];
                            c.swc1(5, cts_ea(c, A1, 0x8));
                            c.g[V1][0] = sx(cts_ea(c, V1, 0x5670));
                            c.sw(cts_ea(c, A1, 0xc), 0u64);
                            c.lwc1(6, cts_ea(c, GP, -0x6270));
                            c.lwc1(7, cts_ea(c, GP, -0x626c));
                            c.swc1(0, cts_ea(c, S6, 0x5660));
                            c.swc1(6, cts_ea(c, A2, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(7, cts_ea(c, A2, 0x8));
                            c.lwc1(8, cts_ea(c, GP, -0x6268));
                            c.swc1(0, cts_ea(c, V0, 0x5670));
                            c.swc1(8, cts_ea(c, V1, 0x8));
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V1, 0xc));
                            c.lwc1(9, cts_ea(c, GP, -0x6264));
                            c.g[V1][0] = c.g[V0][0];
                            c.swc1(0, cts_ea(c, S6, 0x5680));
                            c.g[V1][0] = sx(cts_ea(c, V1, 0x5690));
                            c.swc1(9, cts_ea(c, T1, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V0, 0x5690));
                            c.swc1(0, cts_ea(c, V1, 0xc));
                            c.g[V0][0] = c.g[S6][0];
                            c.swc1(0, cts_ea(c, S6, 0x56a0));
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x56a0));
                            c.swc1(0, cts_ea(c, V0, 0xc));
                            c.g[V1][0] = 0x4c0000;
                            c.lwc1(10, cts_ea(c, GP, -0x6260));
                            c.g[S6][0] = 0x4c0000;
                            c.lwc1(11, cts_ea(c, GP, -0x625c));
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V1, 0x56b0));
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x5670));
                            c.swc1(10, cts_ea(c, A3, 0x4));
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(11, cts_ea(c, A3, 0x8));
                            c.g[V1][0] = sx(cts_ea(c, V1, 0x5690));
                            c.sw(cts_ea(c, A3, 0xc), 0u64);
                            c.lwc1(12, cts_ea(c, GP, -0x6258));
                            c.swc1(0, cts_ea(c, S6, 0x56c0));
                            c.swc1(12, cts_ea(c, V0, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, T1, 0x8));
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x56a0));
                            c.swc1(0, cts_ea(c, V1, 0x4));
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V1, 0x8));
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x56c0));
                            c.swc1(0, cts_ea(c, S6, 0x4));
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S6, 0x8));
                            c.swc1(0, cts_ea(c, V0, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V0, 0x8));
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x5720));
                            c.lwc1(4, cts_ea(c, GP, -0x6254));
                            c.swc1(0, cts_ea(c, V0, 0xc));
                            c.lwc1(3, cts_ea(c, GP, -0x6250));
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(4, cts_ea(c, V1, 0x5700));
                            c.lwc1(2, cts_ea(c, GP, -0x624c));
                            c.g[V1][0] = c.g[V0][0];
                            c.swc1(3, cts_ea(c, T0, 0xc));
                            c.g[V1][0] = sx(cts_ea(c, V1, 0x56d0));
                            c.lwc1(1, cts_ea(c, GP, -0x6248));
                            c.swc1(2, cts_ea(c, S6, 0xc));
                            c.swc1(1, cts_ea(c, T2, 0xc));
                            c.g[S6][0] = 0x4c0000;
                            c.lwc1(5, cts_ea(c, GP, -0x6244));
                            c.swc1(0, cts_ea(c, V0, 0x56d0));
                            c.swc1(5, cts_ea(c, V1, 0x4));
                            c.g[V0][0] = c.g[S6][0];
                            c.sw(cts_ea(c, V1, 0xc), 0u64);
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x56e0));
                            c.swc1(0, cts_ea(c, S6, 0x56e0));
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V0, 0xc));
                            c.g[S6][0] = c.g[V1][0];
                            c.swc1(0, cts_ea(c, V1, 0x56f0));
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x56f0));
                            c.g[V0][0] = 0x4c0000;
                            c.sw(cts_ea(c, S6, 0xc), 0u64);
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x5700));
                            c.lwc1(6, cts_ea(c, GP, -0x6240));
                            c.sw(cts_ea(c, V0, 0xc), 0u64);
                            c.g[V1][0] = 0x4c0000;
                            c.lwc1(7, cts_ea(c, GP, -0x623c));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V1, 0x5710));
                            c.g[V0][0] = c.g[S6][0];
                            c.swc1(6, cts_ea(c, T0, 0x4));
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x5720));
                            c.swc1(7, cts_ea(c, T0, 0x8));
                            c.g[V1][0] = 0x4c0000;
                            c.lwc1(8, cts_ea(c, GP, -0x6238));
                            c.swc1(0, cts_ea(c, S6, 0x5720));
                            c.swc1(8, cts_ea(c, V0, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.lwc1(9, cts_ea(c, GP, -0x6234));
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V1, 0x5730));
                            c.swc1(9, cts_ea(c, T4, 0x8));
                            c.g[V1][0] = 0x4c0000;
                            c.sw(cts_ea(c, T4, 0xc), 0u64);
                            c.g[V1][0] = sx(cts_ea(c, V1, 0x56d0));
                            c.lwc1(10, cts_ea(c, GP, -0x6230));
                            c.swc1(0, cts_ea(c, S6, 0x5740));
                            c.swc1(10, cts_ea(c, T2, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V0, 0x5750));
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x56e0));
                            c.swc1(0, cts_ea(c, V1, 0x8));
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S6, 0x8));
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x56f0));
                            c.swc1(0, cts_ea(c, S6, 0x4));
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V0, 0x4));
                            c.g[V1][0] = sx(cts_ea(c, V1, 0x5700));
                            c.sw(cts_ea(c, V0, 0x8), 0u64);
                            c.g[S6][0] = 0x4c0000;
                            c.sw(cts_ea(c, V1, 0x4), 0u64);
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x5720));
                            c.sw(cts_ea(c, V1, 0x8), 0u64);
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S6, 0x8));
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x5750));
                            c.swc1(0, cts_ea(c, T4, 0x4));
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, T2, 0x8));
                            c.g[V1][0] = sx(cts_ea(c, V1, 0x5760));
                            c.swc1(0, cts_ea(c, V0, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.lwc1(2, cts_ea(c, GP, -0x622c));
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x57a0));
                            c.swc1(12, cts_ea(c, V1, 0xc));
                            c.lwc1(1, cts_ea(c, GP, -0x6228));
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(2, cts_ea(c, S6, 0xc));
                            c.swc1(1, cts_ea(c, S2, 0xc));
                            c.g[S6][0] = c.g[V1][0];
                            c.swc1(0, cts_ea(c, V0, 0xc));
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x5770));
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V1, 0x5770));
                            c.swc1(0, cts_ea(c, V0, 0x5760));
                            c.g[V0][0] = 0x4c0000;
                            c.sw(cts_ea(c, S6, 0xc), 0u64);
                            c.g[V1][0] = c.g[V0][0];
                            c.swc1(0, cts_ea(c, V0, 0x5780));
                            c.g[S6][0] = 0x4c0000;
                            c.g[V1][0] = sx(cts_ea(c, V1, 0x5780));
                            c.sw(cts_ea(c, V1, 0xc), 0u64);
                            c.g[V0][0] = c.g[S6][0];
                            c.swc1(0, cts_ea(c, S6, 0x5790));
                            c.g[V1][0] = 0x4c0000;
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x5790));
                            c.lwc1(3, cts_ea(c, GP, -0x6224));
                            c.sw(cts_ea(c, V0, 0xc), 0u64);
                            c.g[S6][0] = c.g[V1][0];
                            c.swc1(0, cts_ea(c, V1, 0x57a0));
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x57a0));
                            c.swc1(3, cts_ea(c, S6, 0x4));
                            c.g[V0][0] = 0x4c0000;
                            c.lwc1(4, cts_ea(c, GP, -0x6220));
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V0, 0x57b0));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(4, cts_ea(c, S2, 0x8));
                            c.g[V0][0] = 0x4c0000;
                            c.lwc1(5, cts_ea(c, GP, -0x621c));
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x5750));
                            c.swc1(0, cts_ea(c, V1, 0x57c0));
                            c.swc1(5, cts_ea(c, S0, 0x8));
                            c.g[V1][0] = 0x4c0000;
                            c.sw(cts_ea(c, S0, 0xc), 0u64);
                            c.g[V1][0] = sx(cts_ea(c, V1, 0x5760));
                            c.lwc1(6, cts_ea(c, GP, -0x6218));
                            c.lwc1(7, cts_ea(c, GP, -0x6214));
                            c.swc1(0, cts_ea(c, S6, 0x57d0));
                            c.swc1(6, cts_ea(c, T6, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(7, cts_ea(c, T6, 0x8));
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x5770));
                            c.swc1(0, cts_ea(c, V0, 0x8));
                            c.swc1(12, cts_ea(c, V1, 0x4));
                            c.g[V0][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, V1, 0x8));
                            c.g[V0][0] = sx(cts_ea(c, V0, 0x5780));
                            c.sw(cts_ea(c, S6, 0x8), 0u64);
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S6, 0x4));
                            c.g[V1][0] = sx(cts_ea(c, V1, 0x5790));
                            c.swc1(0, cts_ea(c, V0, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.sw(cts_ea(c, V0, 0x8), 0u64);
                            c.g[S6][0] = sx(cts_ea(c, S6, 0x57a0));
                            c.swc1(0, cts_ea(c, V1, 0x4));
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.sw(cts_ea(c, V1, 0x8), 0u64);
                            c.swc1(0, cts_ea(c, S6, 0x8));
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S2, 0x4));
                            c.g[S6][0] = 0x4c0000;
                            c.swc1(0, cts_ea(c, S0, 0x4));
                            c.g[S2][0] = 0x4c0000;
                            c.sw(cts_ea(c, T6, 0xc), 0u64);
                            c.lwc1(2, cts_ea(c, GP, -0x6210));
                            c.sb(cts_ea(c, GP, 0x1d00), c.g[V0][0]);
                            c.lwc1(1, cts_ea(c, GP, -0x620c));
                            c.swc1(2, cts_ea(c, T7, 0xc));
                            c.g[V0][0] = c.g[V0][0] & 0xff;
                            c.swc1(1, cts_ea(c, S1, 0xc));
                            c.g[V0][0] = c.g[V0][0] | c.g[FP][0];
                            c.lwc1(4, cts_ea(c, GP, -0x6208));
                            c.g[FP][0] = c.g[S6][0];
                            c.lwc1(3, cts_ea(c, GP, -0x6204));
                            c.g[FP][0] = sx(cts_ea(c, FP, 0x5810));
                            c.swc1(0, cts_ea(c, V1, 0x57e0));
                            c.swc1(4, cts_ea(c, T5, 0x4));
                            c.g[V1][0] = 0x4c0000;
                            c.swc1(3, cts_ea(c, T5, 0x8));
                            c.sw(cts_ea(c, T5, 0xc), 0u64);
                            c.lwc1(8, cts_ea(c, GP, -0x6200));
                            c.g[T5][0] = 0x4c0000;
                            c.lwc1(5, cts_ea(c, GP, -0x61fc));
                            c.swc1(0, cts_ea(c, T5, 0x57f0));
                            c.swc1(8, cts_ea(c, T7, 0x4));
                            c.swc1(5, cts_ea(c, T7, 0x8));
                            c.lwc1(6, cts_ea(c, GP, -0x61f8));
                            c.lwc1(7, cts_ea(c, GP, -0x61f4));
                            c.swc1(0, cts_ea(c, S2, 0x5800));
                            c.swc1(6, cts_ea(c, S1, 0x4));
                            c.swc1(7, cts_ea(c, S1, 0x8));
                            c.swc1(0, cts_ea(c, S6, 0x5810));
                            c.sw(cts_ea(c, FP, 0xc), 0u64);
                            c.swc1(0, cts_ea(c, V1, 0x5820));
                            c.swc1(0, cts_ea(c, S3, 0x4));
                            c.sw(cts_ea(c, S3, 0xc), 0u64);
                            c.sw(cts_ea(c, GP, 0x1d00), c.g[V0][0]);
                            c.sw(cts_ea(c, FP, 0x4), 0u64);
                            c.swc1(0, cts_ea(c, FP, 0x8));
                            c.sw(cts_ea(c, S3, 0x8), 0u64);
                            CTM_CTOR.call(c, 0x00176228)?;
                            let t = true;
                            c.lq(16, cts_ea(c, SP, 0x90));
                            if t {
                                break 'b_176244;
                            } else {
                                break 'b_176230;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = 0x4c0000;
                    c.g[A1][0] = 0x2;
                    c.g[A0][0] = sx(cts_ea(c, A0, 0x5830));
                    CTM_DTOR.call(c, 0x00176240)?;
                    break 'b_176240;
                }
            }
            c.cycles += 1;
            c.lq(16, cts_ea(c, SP, 0x90));
            break 'b_176244;
        }
        c.cycles += 11;
        c.lq(17, cts_ea(c, SP, 0x80));
        c.lq(18, cts_ea(c, SP, 0x70));
        c.lq(19, cts_ea(c, SP, 0x60));
        c.lq(20, cts_ea(c, SP, 0x50));
        c.lq(21, cts_ea(c, SP, 0x40));
        c.lq(22, cts_ea(c, SP, 0x30));
        c.lq(23, cts_ea(c, SP, 0x20));
        c.lq(30, cts_ea(c, SP, 0x10));
        c.g[RA][0] = c.ld(cts_ea(c, SP, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(cts_ea(c, SP, 0xa0));
        c.pc = t;
        return Ok(());
    })
}

/// Base of the colour records the static init fills (`D_004C5890`).
pub(crate) const DIRECTION_NE_COLOUR_BASE: u32 = 0x004c_5890;

/// The 56 colour records, 16 bytes each, at `DIRECTION_NE_COLOUR_BASE + 16 * i`:
/// four words (RGBA floats as bits; a zero word where the original stores zero).
/// Taken from the original's stores (checked against the game's RAM image).
pub(crate) const DIRECTION_NE_COLOURS: [[u32; 4]; 56] = [
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f000000], // 0x004c5890
    [0x3effeeb3, 0x3f800000, 0x3f800000, 0x00000000], // 0x004c58a0
    [0x3e04a01b, 0x3f800000, 0x3f800000, 0x00000000], // 0x004c58b0
    [0x3e65a6c6, 0x3f70b2fb, 0x3f0ae1ac, 0x39d349bf], // 0x004c58c0
    [0x3eee2fbe, 0x3ea4b685, 0x3f7e8d54, 0x00000000], // 0x004c58d0
    [0x3e36323c, 0x3f800000, 0x3f800000, 0x00000000], // 0x004c58e0
    [0x3ee1e18f, 0x3ed3e8de, 0x3ea46aa1, 0x3ee1309c], // 0x004c58f0
    [0x3f004157, 0x3e7e75bc, 0x3f800000, 0x3f800000], // 0x004c5900
    [0x3e8dc24c, 0x3f23ba56, 0x3e0975f3, 0x00000000], // 0x004c5910
    [0x3f64c5da, 0x3f06bead, 0x3f01512f, 0x3f5e6517], // 0x004c5920
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5930
    [0x3f800000, 0x00000000, 0x00000000, 0x00000000], // 0x004c5940
    [0x3f800000, 0x3f0ccccd, 0x3f08f5c3, 0x3f0ccccd], // 0x004c5950
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5960
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5970
    [0x3f800000, 0x00000000, 0x00000000, 0x00000000], // 0x004c5980
    [0x3f800000, 0x00000000, 0x00000000, 0x00000000], // 0x004c5990
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c59a0
    [0x3f800000, 0x3f000000, 0x3f800000, 0x3f000000], // 0x004c59b0
    [0x3f800000, 0x3f000000, 0x3f800000, 0x3f000000], // 0x004c59c0
    [0x3f800000, 0x3f000000, 0x3f800000, 0x3f000000], // 0x004c59d0
    [0x3f800000, 0x3f48665e, 0x3d7ccd10, 0x3d233376], // 0x004c59e0
    [0x3f800000, 0x3f800000, 0x3f4c7fdc, 0x00000000], // 0x004c59f0
    [0x3f7d198b, 0x3f6c4d01, 0x3eafad08, 0x00000000], // 0x004c5a00
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5a10
    [0x3f800000, 0x3f800000, 0x3f4c3255, 0x00000000], // 0x004c5a20
    [0x3f800000, 0x3f6c19b1, 0x3eaf0004, 0x00000000], // 0x004c5a30
    [0x3f800000, 0x3f48e64b, 0x3d780022, 0x3d266666], // 0x004c5a40
    [0x3f800000, 0x3f4ccccd, 0x3ee66666, 0x3f800000], // 0x004c5a50
    [0x3f800000, 0x3ea80022, 0x3f800000, 0x3eb5332b], // 0x004c5a60
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5a70
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5a80
    [0x3f800000, 0x3f5c800f, 0x3ec332f0, 0x00000000], // 0x004c5a90
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5aa0
    [0x3f800000, 0x3ea61cbd, 0x3f800000, 0x00000000], // 0x004c5ab0
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5ac0
    [0x3f800000, 0x3f800000, 0x00000000, 0x00000000], // 0x004c5ad0
    [0x3f336178, 0x00000000, 0x00000000, 0x00000000], // 0x004c5ae0
    [0x3f800000, 0x3f48690e, 0x3d7b84dc, 0x3d29d51b], // 0x004c5af0
    [0x3f800000, 0x3ea65626, 0x3f800000, 0x3eb4a255], // 0x004c5b00
    [0x3f800000, 0x3f800000, 0x3f4c386d, 0x00000000], // 0x004c5b10
    [0x3f800000, 0x3efcccab, 0x3f800000, 0x3efbccd1], // 0x004c5b20
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5b30
    [0x3f800000, 0x3f4ccccd, 0x3f800000, 0x3f4ccccd], // 0x004c5b40
    [0x3f800000, 0x3f800000, 0x00000000, 0x00000000], // 0x004c5b50
    [0x3f800000, 0x3f800000, 0x00000000, 0x00000000], // 0x004c5b60
    [0x3f800000, 0x3f800000, 0x00000000, 0x00000000], // 0x004c5b70
    [0x3f800000, 0x3ea7995f, 0x3f800000, 0x3eb53309], // 0x004c5b80
    [0x3f800000, 0x3f800000, 0x3d79db23, 0x3d1fbe77], // 0x004c5b90
    [0x3f800000, 0x3f800000, 0x3f4cccab, 0x00000000], // 0x004c5ba0
    [0x3f800000, 0x3f757ffa, 0x3f1d47f1, 0x00000000], // 0x004c5bb0
    [0x3f800000, 0x3f6db329, 0x3eae1262, 0x00000000], // 0x004c5bc0
    [0x3f800000, 0x3f5fb2ff, 0x3e7f4cf5, 0x3c8fffbd], // 0x004c5bd0
    [0x3f800000, 0x3f48ccbc, 0x3d7b3312, 0x3d219978], // 0x004c5be0
    [0x3f800000, 0x00000000, 0x3f800000, 0x00000000], // 0x004c5bf0
    [0x3f800000, 0x3f800000, 0x00000000, 0x00000000], // 0x004c5c00
];

/// Temporaries the original leaves behind after the fill (its schedule of
/// constant loads): the register, then its final value. The saved s0..s7 and
/// fp are restored by the epilogue and are not listed.
pub(crate) const DIRECTION_NE_LEFT_GPR: [(usize, u64); 17] = [
    (AT, 0x3f000000),
    (V0, 0x4c5b50),
    (V1, 0x4c5b60),
    (A0, 0x4c5b80),
    (A1, 0x4c0000),
    (A2, 0x4c0000),
    (A3, 0x4c0000),
    (T0, 0x4c5a60),
    (T1, 0x4c5a70),
    (T2, 0x4c0000),
    (T3, 0x4c5a10),
    (T4, 0x4c5b10),
    (T5, 0x4c5bf0),
    (T6, 0x4c5b90),
    (T7, 0x4c0000),
    (T8, 0x4c0000),
    (T9, 0x4c0000),
];

/// Float registers the original leaves holding literal-pool values (f0..f19).
pub(crate) const DIRECTION_NE_LEFT_FPR: [(usize, u32); 20] = [
    (0, 0x3f800000),
    (1, 0x3d219978),
    (2, 0x3c8fffbd),
    (3, 0x3f6db329),
    (4, 0x3eae1262),
    (5, 0x3f5fb2ff),
    (6, 0x3e7f4cf5),
    (7, 0x3f48ccbc),
    (8, 0x3d7b3312),
    (9, 0x3f4c386d),
    (10, 0x3efcccab),
    (11, 0x3ec332f0),
    (12, 0x3f4ccccd),
    (13, 0x3ea4b685),
    (14, 0x3f7e8d54),
    (15, 0x3ed3e8de),
    (16, 0x3ea46aa1),
    (17, 0x3f70b2fb),
    (18, 0x3f0ae1ac),
    (19, 0x3f23ba56),
];

/// `void DirectionNeChaseCameraAlgorithm_StaticInit(int initialize, int priority)`
/// (0x00177650): the one-time static init of the direction-NE chase camera's
/// colour table. Works only for `priority == 0xffff` and a non-zero
/// `initialize`: writes the 56 colour records at `D_004C5890`, leaves the
/// temporaries of its schedule in the registers, and returns.
/// Cycles: 12 on entry, 2 more once the priority matches, 477 for the fill,
/// 11 on exit (the priority-mismatch path: 23; the initialize-zero path: 25).
pub fn direction_ne_chase_static_init(c: &mut Cpu) {
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
    c.g[V0][0] = 0xffff;
    let frame = c.g[SP][0] as u32;
    c.sq(S0, frame.wrapping_add(0x80));
    c.sq(S1, frame.wrapping_add(0x70));
    c.sq(S2, frame.wrapping_add(0x60));
    c.sq(S3, frame.wrapping_add(0x50));
    c.sq(S4, frame.wrapping_add(0x40));
    c.sq(S5, frame.wrapping_add(0x30));
    c.sq(S6, frame.wrapping_add(0x20));
    c.sq(S7, frame.wrapping_add(0x10));
    c.sq(FP, frame);
    // `priority` (a1) must be 0xffff, and `initialize` (a0) non-zero.
    if c.g[A1][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[T5][0] = 0x4c0000;
        if c.g[A0][0] != 0 {
            c.cycles += 477;
            direction_ne_fill_colours(c);
            direction_ne_leave_temporaries(c);
        }
    }
    c.cycles += 11;
    c.lq(S0, frame.wrapping_add(0x80));
    c.lq(S1, frame.wrapping_add(0x70));
    c.lq(S2, frame.wrapping_add(0x60));
    c.lq(S3, frame.wrapping_add(0x50));
    c.lq(S4, frame.wrapping_add(0x40));
    c.lq(S5, frame.wrapping_add(0x30));
    c.lq(S6, frame.wrapping_add(0x20));
    c.lq(S7, frame.wrapping_add(0x10));
    c.lq(FP, frame);
    c.g[SP][0] = sx(frame.wrapping_add(0x90));
    ret(c);
}

/// Stores the colour records (four words each, in order).
pub(crate) fn direction_ne_fill_colours(c: &mut Cpu) {
    for (i, record) in DIRECTION_NE_COLOURS.iter().enumerate() {
        let base = DIRECTION_NE_COLOUR_BASE + 16 * i as u32;
        for (k, word) in record.iter().enumerate() {
            Addr(base + 4 * k as u32).sw(c, *word as u64);
        }
    }
}

/// Sets the temporaries the original leaves behind.
pub(crate) fn direction_ne_leave_temporaries(c: &mut Cpu) {
    for &(reg, value) in DIRECTION_NE_LEFT_GPR.iter() {
        c.g[reg][0] = value;
    }
    for &(reg, bits) in DIRECTION_NE_LEFT_FPR.iter() {
        c.f[reg] = bits;
    }
}

/// The vtable word the camera-algorithm destructors store at `this+0x10`
/// for `0x460000 - 0x4220` (the `ScriptCameraAlgo`-family vtable).
pub(crate) const ALGO_DTOR_VTABLE_45DDE0: u32 = 0x0046_0000 - 0x4220;

/// `operator delete(void*)` (0x317e50), called by the deleting destructors.
pub(crate) const OPERATOR_DELETE_317E50: u32 = 0x0031_7e50;

/// `CameraTriggerMan_StaticInitDestroy` (0x175a20).
pub(crate) const CAMERA_TRIGGER_MAN_STATIC_INIT_DESTROY: u32 = 0x0017_5a20;

/// `DirectionNeChaseCameraAlgorithm_StaticInit` (0x177650).
pub(crate) const DIRECTION_NE_CHASE_STATIC_INIT: u32 = 0x0017_7650;

/// The chase camera base destructor (0x162458), tail-called by the chase
/// algorithms' destructors.
pub(crate) const CHASE_CAMERA_BASE_DTOR_162458: u32 = 0x0016_2458;

/// Destructor shape shared by the bounded and chase camera algorithms:
/// `this->vptr = vtable` (`this+0x10`), then the base destructor is
/// tail-called with the same `this` and returns to `ret`. Charges 10 cycles.
pub(crate) fn forward_to_base_dtor(c: &mut Cpu, vtable: u32, base: u32, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx(vtable);
    let (this, word) = (c.g[V1][0] as u32, c.g[V0][0]);
    Addr(this.wrapping_add(0x10)).sw(c, word);
    call_ptr(c, base, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// Deleting destructor shape (`ScriptCameraAlgo`, `SubjectScriptCameraAlgo`,
/// `CameraTriggerMan_Member45bde0`): `this->vptr = ALGO_DTOR_VTABLE_45DDE0`,
/// then `operator delete(this)` when bit 0 of `a1` is set. `delete_ret` is
/// the return address of that call. Charges 7 cycles, plus 2 on the delete
/// path, and 3 on the way out.
pub(crate) fn vptr_delete_dtor(c: &mut Cpu, delete_ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = sx(ALGO_DTOR_VTABLE_45DDE0);
    frame.save_ra(c, 0);
    c.g[A1][0] &= 1;
    let (this, word) = (c.g[A0][0] as u32, c.g[V0][0]);
    Addr(this.wrapping_add(0x10)).sw(c, word);
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        call_ptr(c, OPERATOR_DELETE_317E50, delete_ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// A global constructor/destructor call (`CameraTriggerMan_GlobalCtors` and
/// friends): `callee(a0, a1)` with the arguments set here, returning to
/// `ret`. Charges 8 cycles.
pub(crate) fn static_init_call(c: &mut Cpu, a0: u64, a1: u64, callee: u32, ret: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = a0;
    c.g[A1][0] = a1;
    call_ptr(c, callee, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void BoundedCameraAlgorithm_dtor(void* this)` (0x1741d8): set the bounded
/// algorithm's vtable and tail-call the base destructor. Not deleting.
pub fn bounded_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(c, 0x0046_0000 - 0x3ba8, 0x0017_32b8, 0x0017_41f4)
    })
}

/// `void CameraTriggerMan_Member45bde0_dtor(void* this, int flags)` (0x176270).
pub fn camera_trigger_man_member45bde0_dtor(c: &mut Cpu) {
    guest(c, |c| vptr_delete_dtor(c, 0x0017_6294))
}

/// `void CameraTriggerMan_Member10_CallV05(void* this)` (0x1762f0): call the
/// virtual at `vtable+0x2c` (`this` adjusted by the `short` delta at `+0x28`).
pub fn camera_trigger_man_member10_call_v05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(this.wrapping_add(0x10)).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(vt.wrapping_add(0x28)).lh(c);
        c.g[A1][0] = Addr(vt.wrapping_add(0x2c)).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0017_630c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptCameraAlgo_dtor(void* this, int flags)` (0x176328).
pub fn script_camera_algo_dtor(c: &mut Cpu) {
    guest(c, |c| vptr_delete_dtor(c, 0x0017_634c))
}

/// `void SubjectScriptCameraAlgo_dtor(void* this, int flags)` (0x1763c8).
pub fn subject_script_camera_algo_dtor(c: &mut Cpu) {
    guest(c, |c| vptr_delete_dtor(c, 0x0017_63ec))
}

/// `CameraTriggerMan_GlobalCtors` (0x176a28): static init with `(1, 0xffff)`.
pub fn camera_trigger_man_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        static_init_call(
            c,
            1,
            0xffff,
            CAMERA_TRIGGER_MAN_STATIC_INIT_DESTROY,
            0x0017_6a3c,
        )
    })
}

/// `CameraTriggerMan_GlobalDtors` (0x176a48): static destroy with `(0, 0xffff)`.
pub fn camera_trigger_man_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        static_init_call(
            c,
            0,
            0xffff,
            CAMERA_TRIGGER_MAN_STATIC_INIT_DESTROY,
            0x0017_6a5c,
        )
    })
}

/// `void Default_2ChaseCameraAlgorithm_dtor(void* this)` (0x176aa8).
pub fn default_2_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(
            c,
            0x0046_0000 - 0x3550,
            CHASE_CAMERA_BASE_DTOR_162458,
            0x0017_6ac4,
        )
    })
}

/// `void Default_4ChaseCameraAlgorithm_dtor(void* this)` (0x1770a8).
pub fn default_4_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(
            c,
            0x0046_0000 - 0x3640,
            CHASE_CAMERA_BASE_DTOR_162458,
            0x0017_70c4,
        )
    })
}

/// `void DirectionNeChaseCameraAlgorithm_dtor(void* this)` (0x177520).
pub fn direction_ne_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(
            c,
            0x0046_0000 - 0x34d8,
            CHASE_CAMERA_BASE_DTOR_162458,
            0x0017_753c,
        )
    })
}

/// `DirectionNeChaseCameraAlgorithm_GlobalCtor` (0x177e30): static init with `(1, 0xffff)`.
pub fn direction_ne_chase_camera_algorithm_global_ctor(c: &mut Cpu) {
    guest(c, |c| {
        static_init_call(c, 1, 0xffff, DIRECTION_NE_CHASE_STATIC_INIT, 0x0017_7e44)
    })
}

/// `void SpokeChaseCameraAlgorithm_dtor(void* this)` (0x177e90).
pub fn spoke_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(
            c,
            0x0046_0000 - 0x3338,
            CHASE_CAMERA_BASE_DTOR_162458,
            0x0017_7eac,
        )
    })
}

/// `void SuperpipeChaseCameraAlgorithm_dtor(void* this)` (0x178588).
pub fn superpipe_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        forward_to_base_dtor(
            c,
            0x0046_0000 - 0x33b0,
            CHASE_CAMERA_BASE_DTOR_162458,
            0x0017_85a4,
        )
    })
}

/// `$gp - k`: the address of a `$gp`-relative float constant or global.
pub(crate) fn camera_gp_slot(c: &Cpu, k: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(k)
}

/// `VF[dst] = M * VF[k]` for the 4x4 matrix in VF4..VF7: the four VU0 ops
/// (`mula`, `madda`, `madda`, `madd`) with one broadcast component each.
pub(crate) fn camera_mat4_times_vec(c: &mut Cpu, k: u8, dst: u8) {
    c.vu0().mula(0b1111, 4, Src::Bc(k, X));
    c.vu0().madda(0b1111, 5, Src::Bc(k, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(k, Z));
    c.vu0().madd(0b1111, dst, 7, Src::Bc(k, W));
}

/// `void BoundedCameraAlgorithmBase_GetView(void* this, void* out)` (0x173e40):
/// the view pose of the bounded camera. The camera matrix at `this+0x40` is
/// copied to the stack, transformed twice by the VU0 matrix product (once per
/// `.lit4` basis at `gp-0xeb0`/`0x500000-0xea0`), the results stored as the
/// eight-float pose at `out`, and `tPose_tPose` (0x31b748) finishes it.
/// Block structure and the scratch registers mirror the generated body.
pub fn bounded_camera_algorithm_base_get_view(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 211;
        let frame = Frame::open(c, 0x110);
        c.g[V0][0] = 0x0050_0000;
        frame.save(c, S2, 0xd0);
        c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0xe60));
        frame.save(c, S5, 0xa0);
        c.g[S2][0] = c.g[A1][0];
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S0, 0xf0);
        frame.save(c, S1, 0xe0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 20, 0x100);
        let table = c.g[V0][0] as u32;
        c.lqc2(1, table);
        c.lqc2(2, table.wrapping_add(0x10));
        c.lqc2(3, table.wrapping_add(0x20));
        c.lqc2(4, table.wrapping_add(0x30));
        c.sqc2(1, frame.at(c, 0));
        c.sqc2(2, frame.at(c, 0x10));
        c.sqc2(3, frame.at(c, 0x20));
        c.sqc2(4, frame.at(c, 0x30));
        c.lqc2(4, frame.at(c, 0));
        c.lqc2(5, frame.at(c, 0x10));
        c.lqc2(6, frame.at(c, 0x20));
        c.lqc2(7, frame.at(c, 0x30));
        c.lqc2(8, (c.g[S2][0] as u32).wrapping_add(0x40));
        camera_mat4_times_vec(c, 8, 12);
        c.sqc2(12, frame.at(c, 0x30));
        c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x54));
        c.g[S3][0] = c.g[SP][0];
        c.g[A0][0] = sx(frame.at(c, 0x80));
        c.g[A1][0] = sx(frame.at(c, 0x84));
        c.neg_s(12, 12);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 20);
        c.g[S0][0] = 0x0050_0000;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xeb0));
        call_ptr(c, 0x0031_be50, 0x0017_3ee4)?;
        c.lwc1(11, frame.at(c, 0x84));
        c.g[S4][0] = sx(frame.at(c, 0x40));
        c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.sub_s(1, 20, 11);
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_sub(0xeb0));
        c.lwc1(6, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.lwc1(8, frame.at(c, 0x80));
        c.mul_s(2, 1, 3);
        c.sw(frame.at(c, 0x70), 0);
        c.mul_s(4, 1, 0);
        c.sw(frame.at(c, 0x74), 0);
        c.mul_s(1, 1, 6);
        c.sw(frame.at(c, 0x78), 0);
        c.mul_s(13, 8, 3);
        c.sw(frame.at(c, 0x4c), 0);
        c.mul_s(12, 8, 0);
        c.sw(frame.at(c, 0x5c), 0);
        c.mul_s(9, 2, 3);
        c.sw(frame.at(c, 0x6c), 0);
        c.mul_s(10, 4, 3);
        c.swc1(20, frame.at(c, 0x7c));
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
        c.swc1(5, frame.at(c, 0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, frame.at(c, 0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, frame.at(c, 0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, frame.at(c, 0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, frame.at(c, 0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, frame.at(c, 0x50));
        c.swc1(10, frame.at(c, 0x60));
        c.swc1(1, frame.at(c, 0x54));
        c.swc1(3, frame.at(c, 0x64));
        let s3 = c.g[S3][0] as u32;
        let s4 = c.g[S4][0] as u32;
        c.lqc2(4, s3);
        c.lqc2(5, s3.wrapping_add(0x10));
        c.lqc2(6, s3.wrapping_add(0x20));
        c.lqc2(7, s3.wrapping_add(0x30));
        c.lqc2(8, s4);
        c.lqc2(9, s4.wrapping_add(0x10));
        c.lqc2(10, s4.wrapping_add(0x20));
        c.lqc2(11, s4.wrapping_add(0x30));
        camera_mat4_times_vec(c, 8, 12);
        camera_mat4_times_vec(c, 9, 13);
        camera_mat4_times_vec(c, 10, 14);
        camera_mat4_times_vec(c, 11, 15);
        c.sqc2(12, s3);
        c.sqc2(13, s3.wrapping_add(0x10));
        c.sqc2(14, s3.wrapping_add(0x20));
        c.sqc2(15, s3.wrapping_add(0x30));
        c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x50));
        c.g[S0][0] = c.g[SP][0];
        c.g[A0][0] = sx(frame.at(c, 0x88));
        c.g[A1][0] = sx(frame.at(c, 0x8c));
        call_ptr(c, 0x0031_be50, 0x0017_4024)?;
        c.lwc1(11, frame.at(c, 0x8c));
        c.g[V0][0] = 0x0050_0000;
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_sub(0xea0));
        c.sub_s(1, 20, 11);
        c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.lwc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.lwc1(8, frame.at(c, 0x88));
        c.mul_s(2, 1, 3);
        c.swc1(20, frame.at(c, 0x7c));
        c.mul_s(4, 1, 0);
        c.sw(frame.at(c, 0x70), 0);
        c.mul_s(1, 1, 6);
        c.sw(frame.at(c, 0x74), 0);
        c.mul_s(13, 8, 3);
        c.sw(frame.at(c, 0x78), 0);
        c.mul_s(12, 8, 0);
        c.sw(frame.at(c, 0x4c), 0);
        c.mul_s(9, 2, 3);
        c.sw(frame.at(c, 0x5c), 0);
        c.mul_s(10, 4, 3);
        c.sw(frame.at(c, 0x6c), 0);
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
        c.swc1(5, frame.at(c, 0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, frame.at(c, 0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, frame.at(c, 0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, frame.at(c, 0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, frame.at(c, 0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, frame.at(c, 0x50));
        c.swc1(10, frame.at(c, 0x60));
        c.swc1(1, frame.at(c, 0x54));
        c.swc1(3, frame.at(c, 0x64));
        let s0 = c.g[S0][0] as u32;
        let s4 = c.g[S4][0] as u32;
        c.lqc2(4, s0);
        c.lqc2(5, s0.wrapping_add(0x10));
        c.lqc2(6, s0.wrapping_add(0x20));
        c.lqc2(7, s0.wrapping_add(0x30));
        c.lqc2(8, s4);
        c.lqc2(9, s4.wrapping_add(0x10));
        c.lqc2(10, s4.wrapping_add(0x20));
        c.lqc2(11, s4.wrapping_add(0x30));
        camera_mat4_times_vec(c, 8, 12);
        camera_mat4_times_vec(c, 9, 13);
        camera_mat4_times_vec(c, 10, 14);
        camera_mat4_times_vec(c, 11, 15);
        c.sqc2(12, s0);
        c.sqc2(13, s0.wrapping_add(0x10));
        c.sqc2(14, s0.wrapping_add(0x20));
        c.sqc2(15, s0.wrapping_add(0x30));
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0031_b748, 0x0017_4160)?;
        c.g[V0][0] = c.g[S5][0];
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void Default_2ChaseCameraAlgorithm_Update(void* this)` (0x176b10): a
/// straight run of the chase camera's per-frame steps (`0x162568` ..
/// `0x1643a8`), each with its `.lit4` float operands loaded from `$gp`.
/// The stack holds the step outputs (`sp+0..0x80`); `a1` is `sp` for every
/// step, as in the generated body.
pub fn default_2_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 116;
        let frame = Frame::open(c, 0xa0);
        c.lwc1(19, camera_gp_slot(c, 0x61d8));
        frame.save(c, S0, 0x80);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(12, camera_gp_slot(c, 0x61d4));
        c.lwc1(13, camera_gp_slot(c, 0x61d0));
        c.lwc1(14, camera_gp_slot(c, 0x61cc));
        c.lwc1(15, camera_gp_slot(c, 0x61c8));
        c.lwc1(16, camera_gp_slot(c, 0x61c4));
        c.lwc1(17, camera_gp_slot(c, 0x61c0));
        c.lwc1(18, camera_gp_slot(c, 0x61bc));
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        call_ptr(c, 0x0016_2568, 0x0017_6b50)?;
        c.mtc1(0, 20);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2998, 0x0017_6b60)?;
        c.lwc1(21, camera_gp_slot(c, 0x61b8));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2a20, 0x0017_6b70)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b80, 0x0017_6b7c)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2c78, 0x0017_6b88)?;
        c.lwc1(17, camera_gp_slot(c, 0x61b4));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(18, camera_gp_slot(c, 0x61b0));
        c.lwc1(12, camera_gp_slot(c, 0x61ac));
        c.lwc1(13, camera_gp_slot(c, 0x61a8));
        c.lwc1(14, camera_gp_slot(c, 0x61a4));
        c.g[AT][0] = 0x40a8_0000;
        c.mtc1(AT, 15);
        c.lwc1(16, camera_gp_slot(c, 0x61a0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3010, 0x0017_6bb4)?;
        c.lwc1(16, camera_gp_slot(c, 0x619c));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, camera_gp_slot(c, 0x6198));
        c.lwc1(13, camera_gp_slot(c, 0x6194));
        c.lwc1(14, camera_gp_slot(c, 0x6190));
        c.lwc1(15, camera_gp_slot(c, 0x618c));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3270, 0x0017_6bd4)?;
        c.lwc1(13, camera_gp_slot(c, 0x6188));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 14);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(15, camera_gp_slot(c, 0x6184));
        c.mov_s(12, 20);
        call_ptr(c, 0x0016_33b0, 0x0017_6bf4)?;
        c.lwc1(18, camera_gp_slot(c, 0x6180));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(19, camera_gp_slot(c, 0x617c));
        c.mov_s(17, 21);
        c.lwc1(12, camera_gp_slot(c, 0x6178));
        c.mov_s(16, 20);
        c.lwc1(13, camera_gp_slot(c, 0x6174));
        c.lwc1(14, camera_gp_slot(c, 0x6170));
        c.lwc1(15, camera_gp_slot(c, 0x616c));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3450, 0x0017_6c20)?;
        c.lwc1(16, camera_gp_slot(c, 0x6168));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, camera_gp_slot(c, 0x6164));
        c.lwc1(13, camera_gp_slot(c, 0x6160));
        c.lwc1(14, camera_gp_slot(c, 0x615c));
        c.lwc1(15, camera_gp_slot(c, 0x6158));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b90, 0x0017_6c40)?;
        c.lwc1(14, camera_gp_slot(c, 0x6154));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(15, camera_gp_slot(c, 0x6150));
        c.lwc1(13, camera_gp_slot(c, 0x614c));
        c.lwc1(12, camera_gp_slot(c, 0x6148));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_35f8, 0x0017_6c5c)?;
        c.lwc1(12, camera_gp_slot(c, 0x6144));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_4878, 0x0017_6c6c)?;
        c.lwc1(12, camera_gp_slot(c, 0x6140));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(13, camera_gp_slot(c, 0x613c));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_46a0, 0x0017_6c80)?;
        c.g[AT][0] = 0x4416_0000;
        c.mtc1(AT, 12);
        c.mov_s(15, 21);
        c.g[AT][0] = 0x4396_0000;
        c.mtc1(AT, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 14);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0016_41c0, 0x0017_6ca8)?;
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(AT, 12);
        c.g[A1][0] = c.g[SP][0];
        c.mov_s(14, 13);
        call_ptr(c, 0x0016_43a8, 0x0017_6cc8)?;
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// `void Default_4ChaseCameraAlgorithm_Update(void* this)` (0x177110): the
/// same per-frame step list as the Default_2 chase camera, with its own
/// `$gp` constants and callee return addresses.
pub fn default_4_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 115;
        let frame = Frame::open(c, 0xa0);
        c.lwc1(19, camera_gp_slot(c, 0x6064));
        frame.save(c, S0, 0x80);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(12, camera_gp_slot(c, 0x6060));
        c.lwc1(13, camera_gp_slot(c, 0x605c));
        c.lwc1(14, camera_gp_slot(c, 0x6058));
        c.lwc1(15, camera_gp_slot(c, 0x6054));
        c.lwc1(16, camera_gp_slot(c, 0x6050));
        c.lwc1(17, camera_gp_slot(c, 0x604c));
        c.lwc1(18, camera_gp_slot(c, 0x6048));
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        call_ptr(c, 0x0016_2568, 0x0017_7150)?;
        c.mtc1(0, 20);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2998, 0x0017_7160)?;
        c.lwc1(21, camera_gp_slot(c, 0x6044));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2a20, 0x0017_7170)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b80, 0x0017_717c)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2c78, 0x0017_7188)?;
        c.lwc1(17, camera_gp_slot(c, 0x6040));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(18, camera_gp_slot(c, 0x603c));
        c.lwc1(12, camera_gp_slot(c, 0x6038));
        c.lwc1(13, camera_gp_slot(c, 0x6034));
        c.lwc1(14, camera_gp_slot(c, 0x6030));
        c.lwc1(15, camera_gp_slot(c, 0x602c));
        c.lwc1(16, camera_gp_slot(c, 0x6028));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3010, 0x0017_71b0)?;
        c.lwc1(16, camera_gp_slot(c, 0x6024));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, camera_gp_slot(c, 0x6020));
        c.lwc1(13, camera_gp_slot(c, 0x601c));
        c.lwc1(14, camera_gp_slot(c, 0x6018));
        c.lwc1(15, camera_gp_slot(c, 0x6014));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3270, 0x0017_71d0)?;
        c.lwc1(13, camera_gp_slot(c, 0x6010));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 14);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(15, camera_gp_slot(c, 0x600c));
        c.mov_s(12, 20);
        call_ptr(c, 0x0016_33b0, 0x0017_71f0)?;
        c.lwc1(18, camera_gp_slot(c, 0x6008));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(19, camera_gp_slot(c, 0x6004));
        c.mov_s(17, 21);
        c.lwc1(12, camera_gp_slot(c, 0x6000));
        c.mov_s(16, 20);
        c.lwc1(13, camera_gp_slot(c, 0x5ffc));
        c.lwc1(14, camera_gp_slot(c, 0x5ff8));
        c.lwc1(15, camera_gp_slot(c, 0x5ff4));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_3450, 0x0017_721c)?;
        c.lwc1(16, camera_gp_slot(c, 0x5ff0));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, camera_gp_slot(c, 0x5fec));
        c.lwc1(13, camera_gp_slot(c, 0x5fe8));
        c.lwc1(14, camera_gp_slot(c, 0x5fe4));
        c.lwc1(15, camera_gp_slot(c, 0x5fe0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b90, 0x0017_723c)?;
        c.lwc1(14, camera_gp_slot(c, 0x5fdc));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(15, camera_gp_slot(c, 0x5fd8));
        c.lwc1(13, camera_gp_slot(c, 0x5fd4));
        c.lwc1(12, camera_gp_slot(c, 0x5fd0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_35f8, 0x0017_7258)?;
        c.lwc1(12, camera_gp_slot(c, 0x5fcc));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_4878, 0x0017_7268)?;
        c.lwc1(12, camera_gp_slot(c, 0x5fc8));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(13, camera_gp_slot(c, 0x5fc4));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_46a0, 0x0017_727c)?;
        c.g[AT][0] = 0x4416_0000;
        c.mtc1(AT, 12);
        c.mov_s(15, 21);
        c.g[AT][0] = 0x4396_0000;
        c.mtc1(AT, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 14);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0016_41c0, 0x0017_72a4)?;
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 13);
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(AT, 12);
        c.g[A1][0] = c.g[SP][0];
        c.mov_s(14, 13);
        call_ptr(c, 0x0016_43a8, 0x0017_72c4)?;
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// `void DirectionNeChaseCameraAlgorithm_Update(void* this)` (0x177368): the
/// chase camera steps again, with the second `.lit4` set and the `0x165540`
/// read of `this+0x390` at the end. Saves only `s0` and `f20`.
pub fn direction_ne_chase_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 86;
        let frame = Frame::open(c, 0xa0);
        c.lwc1(19, camera_gp_slot(c, 0x5fbc));
        frame.save(c, S0, 0x80);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(12, camera_gp_slot(c, 0x5fb8));
        c.lwc1(13, camera_gp_slot(c, 0x5fb4));
        c.lwc1(14, camera_gp_slot(c, 0x5fb0));
        c.lwc1(15, camera_gp_slot(c, 0x5fac));
        c.lwc1(16, camera_gp_slot(c, 0x5fa8));
        c.lwc1(17, camera_gp_slot(c, 0x5fa4));
        c.lwc1(18, camera_gp_slot(c, 0x5fa0));
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 20, 0x90);
        call_ptr(c, 0x0016_2568, 0x0017_73a4)?;
        c.mtc1(0, 20);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2998, 0x0017_73b4)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2a20, 0x0017_73c0)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b80, 0x0017_73cc)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2c78, 0x0017_73d8)?;
        c.lwc1(17, camera_gp_slot(c, 0x5f9c));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(18, camera_gp_slot(c, 0x5f98));
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(12, camera_gp_slot(c, 0x5f94));
        c.mov_s(13, 20);
        c.lwc1(14, camera_gp_slot(c, 0x5f90));
        c.lwc1(16, camera_gp_slot(c, 0x5f8c));
        c.mov_s(15, 20);
        call_ptr(c, 0x0016_3010, 0x0017_7400)?;
        c.lwc1(16, camera_gp_slot(c, 0x5f88));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(13, camera_gp_slot(c, 0x5f84));
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 20);
        c.mov_s(14, 20);
        c.mov_s(15, 20);
        call_ptr(c, 0x0016_3270, 0x0017_7420)?;
        c.lwc1(13, camera_gp_slot(c, 0x5f80));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 14);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(15, camera_gp_slot(c, 0x5f7c));
        c.mov_s(12, 20);
        call_ptr(c, 0x0016_33b0, 0x0017_7440)?;
        c.lwc1(16, camera_gp_slot(c, 0x5f78));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(12, camera_gp_slot(c, 0x5f74));
        c.lwc1(13, camera_gp_slot(c, 0x5f70));
        c.lwc1(14, camera_gp_slot(c, 0x5f6c));
        c.lwc1(15, camera_gp_slot(c, 0x5f68));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_2b90, 0x0017_7460)?;
        c.lwc1(14, camera_gp_slot(c, 0x5f64));
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(15, camera_gp_slot(c, 0x5f60));
        c.lwc1(13, camera_gp_slot(c, 0x5f5c));
        c.lwc1(12, camera_gp_slot(c, 0x5f58));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_35f8, 0x0017_747c)?;
        c.lwc1(12, camera_gp_slot(c, 0x5f54));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_4878, 0x0017_748c)?;
        c.lwc1(13, camera_gp_slot(c, 0x5f50));
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0016_46a0, 0x0017_74a0)?;
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x390));
        call_ptr(c, 0x0016_5540, 0x0017_74ac)?;
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// `void BoundedCameraAlgorithm_Update(void* this)` (0x174200): the bounded
/// camera's per-frame update. Three regions with the generated block
/// structure: the first picks the target from `this+0x80` (with a shortcut
/// when its `+0x24` flag is 1), then the VU0 offset of the rider's pose; the
/// second computes the blend from the `+0x74`/`+0x20` fields with a
/// division by `.lit4` constants; the third clamps `this+0x54` against the
/// `.lit4` limits, with a square root for the step length.
pub fn bounded_camera_algorithm_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xb0);
        'b_17432c: {
            'b_174280: {
                c.g[V1][0] = 0x1;
                frame.save(c, S0, 0x80);
                frame.save_ra(c, 0x70);
                c.g[S0][0] = c.g[A0][0];
                frame.save_fpr(c, 22, 0xa0);
                frame.save_fpr(c, 21, 0x98);
                frame.save_fpr(c, 20, 0x90);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x70));
                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x18));
                c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x6c));
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x1c));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x78));
                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x14));
                c.swc1(1, c.g[S0][0] as u32);
                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x74));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                    break 'b_174280;
                } else {
                    c.cycles += 10;
                    c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x28));
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 2);
                    c.swc1(1, frame.at(c, 0));
                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x2c));
                    c.swc1(0, frame.at(c, 4));
                    c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x30));
                    c.swc1(2, frame.at(c, 0xc));
                    c.swc1(1, frame.at(c, 8));
                    break 'b_17432c;
                }
            }
            c.cycles += 43;
            c.g[A0][0] = sx(frame.at(c, 0x20));
            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0017_4298)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx(frame.at(c, 0x40));
            c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x74));
            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0017_42b8)?;
            c.mfc1(4, 20);
            c.lqc2(4, frame.at(c, 0x40));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x50));
            c.lq(2, frame.at(c, 0x50));
            c.sq(2, frame.at(c, 0x30));
            c.lqc2(3, frame.at(c, 0x20));
            c.lqc2(4, frame.at(c, 0x30));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x50));
            c.lq(2, frame.at(c, 0x50));
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6c));
            c.sq(2, frame.at(c, 0x10));
            c.g[V0][0] = 0x0050_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
            c.mfc1(3, 0);
            c.lqc2(4, c.g[V0][0] as u32);
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x60));
            c.lq(2, frame.at(c, 0x60));
            c.sq(2, frame.at(c, 0x50));
            c.lqc2(3, frame.at(c, 0x10));
            c.lqc2(4, frame.at(c, 0x50));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x60));
            c.lq(2, frame.at(c, 0x60));
            c.sq(2, frame.at(c, 0));
            break 'b_17432c;
        }
        'b_1743f4: {
            'b_1743ec: {
                'b_1743b0: {
                    c.cycles += 22;
                    c.lq(2, frame.at(c, 0));
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
                    c.mtc1(0, 22);
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017_4358)?;
                    c.lqc2(3, frame.at(c, 0));
                    c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x20));
                    c.lq(2, frame.at(c, 0x20));
                    c.sq(2, frame.at(c, 0x10));
                    c.lwc1(20, frame.at(c, 0x10));
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    c.lwc1(21, frame.at(c, 0x14));
                    if t {
                        break 'b_1743b0;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(12, 21);
                        if t {
                            break 'b_1743ec;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(12, camera_gp_slot(c, 0x635c));
                            let t = !c.fcc();
                            c.lwc1(0, frame.at(c, 0x10));
                            if t {
                                break 'b_1743ec;
                            } else {
                                c.cycles += 3;
                                c.lwc1(12, camera_gp_slot(c, 0x6358));
                                c.lwc1(1, frame.at(c, 0x14));
                                break 'b_1743f4;
                            }
                        }
                    }
                }
                c.cycles += 7;
                c.div_s(12, 21, 20);
                call_ptr(c, 0x0031_c228, 0x0017_43bc)?;
                c.c_lt_s(20, 22);
                let t = !c.fcc();
                c.mov_s(12, 0);
                if t {
                    break 'b_1743ec;
                } else {
                    'b_1743e4: {
                        c.cycles += 3;
                        c.c_lt_s(22, 21);
                        let t = !c.fcc();
                        if t {
                            break 'b_1743e4;
                        } else {
                            c.cycles += 3;
                            c.lwc1(0, camera_gp_slot(c, 0x6354));
                            c.add_s(12, 12, 0);
                            break 'b_1743ec;
                        }
                    }
                    c.cycles += 2;
                    c.lwc1(0, camera_gp_slot(c, 0x6350));
                    c.sub_s(12, 12, 0);
                    break 'b_1743ec;
                }
            }
            c.cycles += 2;
            c.lwc1(0, frame.at(c, 0x10));
            c.lwc1(1, frame.at(c, 0x14));
            break 'b_1743f4;
        }
        'b_174494: {
            'b_174490: {
                'b_174450: {
                    c.cycles += 12;
                    c.mul_s(2, 0, 0);
                    c.swc1(12, (c.g[S0][0] as u32).wrapping_add(0x50));
                    c.mul_s(3, 1, 1);
                    c.swc1(0, frame.at(c, 0x20));
                    c.mtc1(0, 22);
                    c.swc1(1, frame.at(c, 0x24));
                    c.add_s(2, 2, 3);
                    c.lwc1(21, frame.at(c, 0x18));
                    c.sqrt_s(20, 2);
                    c.c_eq_s(20, 22);
                    let t = !c.fcc();
                    if t {
                        break 'b_174450;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 22);
                        let t = c.fcc();
                        c.mov_s(0, 21);
                        if t {
                            break 'b_174490;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(22, 21);
                            c.lwc1(0, camera_gp_slot(c, 0x634c));
                            if !c.fcc() {
                                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
                                break 'b_174494;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, camera_gp_slot(c, 0x6348));
                                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
                                break 'b_174494;
                            }
                        }
                    }
                }
                'b_17448c: {
                    c.cycles += 7;
                    c.div_s(12, 21, 20);
                    call_ptr(c, 0x0031_c228, 0x0017_445c)?;
                    c.c_lt_s(20, 22);
                    let t = !c.fcc();
                    c.mov_s(12, 0);
                    if t {
                        break 'b_17448c;
                    } else {
                        'b_174484: {
                            c.cycles += 3;
                            c.c_lt_s(22, 21);
                            let t = !c.fcc();
                            if t {
                                break 'b_174484;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, camera_gp_slot(c, 0x6344));
                                c.add_s(12, 12, 0);
                                break 'b_17448c;
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, camera_gp_slot(c, 0x6340));
                        c.sub_s(12, 12, 0);
                        break 'b_17448c;
                    }
                }
                c.cycles += 1;
                c.mov_s(0, 12);
                break 'b_174490;
            }
            c.cycles += 1;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
            break 'b_174494;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 22, 0xa0);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// `void BoundedCameraAlgorithmBase_ResetForRider(void* this, void* rider)`
/// (0x1744b0): reset the bounded camera for a rider. Calls the rider's
/// pose query (`this+0x30` -> `+0x4` -> `+0x6c0` vtable `+0x38/+0x3c`), then
/// either finishes early (the rider's object is not the one `this` tracks),
/// or copies the rider's `.lit4` offsets into the camera and runs the
/// base reset (`0x1732e8`) and the same clamp logic as the update.
pub fn bounded_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xc0);
        'b_174780: {
            frame.save(c, S0, 0x90);
            frame.save(c, S1, 0x80);
            c.g[S0][0] = c.g[A0][0];
            frame.save_ra(c, 0x70);
            c.g[S1][0] = c.g[A1][0];
            frame.save_fpr(c, 22, 0xb0);
            frame.save_fpr(c, 21, 0xa8);
            frame.save_fpr(c, 20, 0xa0);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0017_44f4)?;
            if c.g[V0][0] != c.g[S1][0] {
                frame.restore(c, S0, 0x90);
                break 'b_174780;
            } else {
                'b_1745e4: {
                    'b_174538: {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                        c.g[V1][0] = 0x1;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
                        if c.g[V0][0] != c.g[V1][0] {
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                            break 'b_174538;
                        } else {
                            c.cycles += 10;
                            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x28));
                            c.g[AT][0] = 0x3f80_0000;
                            c.mtc1(AT, 2);
                            c.swc1(1, frame.at(c, 0));
                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x2c));
                            c.swc1(0, frame.at(c, 4));
                            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x30));
                            c.swc1(2, frame.at(c, 0xc));
                            c.swc1(1, frame.at(c, 8));
                            break 'b_1745e4;
                        }
                    }
                    c.cycles += 43;
                    c.g[A0][0] = sx(frame.at(c, 0x20));
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x0017_4550)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = sx(frame.at(c, 0x40));
                    c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x74));
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                    let t = c.g[V1][0] as u32;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x0017_4570)?;
                    c.mfc1(4, 20);
                    c.lqc2(4, frame.at(c, 0x40));
                    c.qmtc2(4, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x50));
                    c.lq(2, frame.at(c, 0x50));
                    c.sq(2, frame.at(c, 0x30));
                    c.lqc2(3, frame.at(c, 0x20));
                    c.lqc2(4, frame.at(c, 0x30));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x50));
                    c.lq(2, frame.at(c, 0x50));
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6c));
                    c.sq(2, frame.at(c, 0x10));
                    c.g[V0][0] = 0x0050_0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
                    c.mfc1(3, 0);
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x60));
                    c.lq(2, frame.at(c, 0x60));
                    c.sq(2, frame.at(c, 0x50));
                    c.lqc2(3, frame.at(c, 0x10));
                    c.lqc2(4, frame.at(c, 0x50));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x60));
                    c.lq(2, frame.at(c, 0x60));
                    c.sq(2, frame.at(c, 0));
                    break 'b_1745e4;
                }
                'b_1746dc: {
                    'b_1746d4: {
                        'b_174698: {
                            c.cycles += 34;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.mtc1(0, 22);
                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x70));
                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x6c));
                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x78));
                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                            c.swc1(1, c.g[S0][0] as u32);
                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x74));
                            call_ptr(c, 0x0017_32e8, 0x0017_4620)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x24));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0017_4640)?;
                            c.lqc2(3, frame.at(c, 0));
                            c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x40));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x20));
                            c.lq(2, frame.at(c, 0x20));
                            c.sq(2, frame.at(c, 0x10));
                            c.lwc1(20, frame.at(c, 0x10));
                            c.c_eq_s(20, 22);
                            let t = !c.fcc();
                            c.lwc1(21, frame.at(c, 0x14));
                            if t {
                                break 'b_174698;
                            } else {
                                c.cycles += 4;
                                c.c_eq_s(21, 22);
                                let t = c.fcc();
                                c.mov_s(12, 21);
                                if t {
                                    break 'b_1746d4;
                                } else {
                                    c.cycles += 4;
                                    c.c_le_s(22, 21);
                                    c.lwc1(12, camera_gp_slot(c, 0x633c));
                                    let t = !c.fcc();
                                    c.lwc1(0, frame.at(c, 0x10));
                                    if t {
                                        break 'b_1746d4;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(12, camera_gp_slot(c, 0x6338));
                                        c.lwc1(1, frame.at(c, 0x14));
                                        break 'b_1746dc;
                                    }
                                }
                            }
                        }
                        c.cycles += 7;
                        c.div_s(12, 21, 20);
                        call_ptr(c, 0x0031_c228, 0x0017_46a4)?;
                        c.c_lt_s(20, 22);
                        let t = !c.fcc();
                        c.mov_s(12, 0);
                        if t {
                            break 'b_1746d4;
                        } else {
                            'b_1746cc: {
                                c.cycles += 3;
                                c.c_lt_s(22, 21);
                                let t = !c.fcc();
                                if t {
                                    break 'b_1746cc;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, camera_gp_slot(c, 0x6334));
                                    c.add_s(12, 12, 0);
                                    break 'b_1746d4;
                                }
                            }
                            c.cycles += 2;
                            c.lwc1(0, camera_gp_slot(c, 0x6330));
                            c.sub_s(12, 12, 0);
                            break 'b_1746d4;
                        }
                    }
                    c.cycles += 2;
                    c.lwc1(0, frame.at(c, 0x10));
                    c.lwc1(1, frame.at(c, 0x14));
                    break 'b_1746dc;
                }
                'b_17477c: {
                    'b_174778: {
                        'b_174738: {
                            c.cycles += 12;
                            c.mul_s(2, 0, 0);
                            c.swc1(12, (c.g[S0][0] as u32).wrapping_add(0x50));
                            c.mul_s(3, 1, 1);
                            c.swc1(0, frame.at(c, 0x20));
                            c.mtc1(0, 22);
                            c.swc1(1, frame.at(c, 0x24));
                            c.add_s(2, 2, 3);
                            c.lwc1(21, frame.at(c, 0x18));
                            c.sqrt_s(20, 2);
                            c.c_eq_s(20, 22);
                            let t = !c.fcc();
                            if t {
                                break 'b_174738;
                            } else {
                                c.cycles += 4;
                                c.c_eq_s(21, 22);
                                let t = c.fcc();
                                c.mov_s(0, 21);
                                if t {
                                    break 'b_174778;
                                } else {
                                    c.cycles += 4;
                                    c.c_le_s(22, 21);
                                    c.lwc1(0, camera_gp_slot(c, 0x632c));
                                    if !c.fcc() {
                                        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
                                        break 'b_17477c;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(0, camera_gp_slot(c, 0x6328));
                                        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
                                        break 'b_17477c;
                                    }
                                }
                            }
                        }
                        'b_174774: {
                            c.cycles += 7;
                            c.div_s(12, 21, 20);
                            call_ptr(c, 0x0031_c228, 0x0017_4744)?;
                            c.c_lt_s(20, 22);
                            let t = !c.fcc();
                            c.mov_s(12, 0);
                            if t {
                                break 'b_174774;
                            } else {
                                'b_17476c: {
                                    c.cycles += 3;
                                    c.c_lt_s(22, 21);
                                    let t = !c.fcc();
                                    if t {
                                        break 'b_17476c;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(0, camera_gp_slot(c, 0x6324));
                                        c.add_s(12, 12, 0);
                                        break 'b_174774;
                                    }
                                }
                                c.cycles += 2;
                                c.lwc1(0, camera_gp_slot(c, 0x6320));
                                c.sub_s(12, 12, 0);
                                break 'b_174774;
                            }
                        }
                        c.cycles += 1;
                        c.mov_s(0, 12);
                        break 'b_174778;
                    }
                    c.cycles += 1;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x54));
                    break 'b_17477c;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x90);
                break 'b_174780;
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 22, 0xb0);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// `void SpokeChaseCameraAlgorithm_ResetForRider(void* this, void* rider)`
/// (0x1783e0): the spoke chase camera's rider reset. Reads the rider's pose
/// as the bounded reset does; when it matches, it computes the desired eye
/// and focus point, scales the rider's offset through VU0, and runs the chase
/// base reset (`0x166c60`) and init (`0x166550`) before copying the offset
/// back into `this+0x20`.
pub fn spoke_chase_camera_algorithm_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x40);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, t, 0x0017_841c)?;
        let rider_differs = c.g[S2][0] != c.g[V0][0];
        frame.restore(c, S0, 0x70);
        if !rider_differs {
            c.cycles += 58;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.g[SP][0];
            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0017_8440)?;
            c.lq(5, frame.at(c, 0));
            c.g[S0][0] = sx(frame.at(c, 0x10));
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.g[S0][0];
            c.sq(5, (c.g[S1][0] as u32).wrapping_add(0x390));
            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0017_8468)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0017_7fc0, 0x0017_8478)?;
            c.lq(2, frame.at(c, 0));
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x40));
            call_ptr(c, 0x0017_7ef8, 0x0017_848c)?;
            c.g[V1][0] = 0x0050_0000;
            c.g[AT][0] = 0x4214_0000;
            c.mtc1(AT, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
            c.g[A1][0] = c.g[S2][0];
            c.mfc1(2, 0);
            c.lqc2(4, c.g[V1][0] as u32);
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x30));
            c.g[A0][0] = c.g[S1][0];
            c.lq(2, frame.at(c, 0x30));
            c.sq(2, frame.at(c, 0x20));
            c.lqc2(3, frame.at(c, 0x10));
            c.lqc2(4, frame.at(c, 0x20));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x30));
            c.lq(2, frame.at(c, 0x30));
            c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x20));
            c.sq(2, frame.at(c, 0));
            call_ptr(c, 0x0016_6c60, 0x0017_84e0)?;
            c.lwc1(12, camera_gp_slot(c, 0x5dc4));
            c.g[AT][0] = 0x4396_0000;
            c.mtc1(AT, 13);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0016_6550, 0x0017_84f4)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0017_8508)?;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `Post_Race_1ChaseCameraAlgorithm_StaticInit` (0x178f58). When `a1` is
/// `0xffff` and `a0` is non-zero (the C++ `priority == 0xFFFF && initialize != 0`),
/// it fills the post-race chase camera's static colour table: 58 `sColor`
/// entries, four floats each, 0x10 bytes apart from `0x4c5c10` to `0x4c5f80`.
/// The float values come from GP-relative literals; the zero components are
/// stored as zero words. The body is the static-init data as laid out in the
/// generated code (store order and register results kept), since it is a
/// flat table of about 500 straight-line instructions.
pub fn post_race_1chase_camera_algorithm_static_init(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x00178f58);
    'b_179704: {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[V0][0] = 0xffff;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[A1][0] != c.g[V0][0];
        c.sq(30, c.g[SP][0] as u32);
        if t {
            break 'b_179704;
        } else {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[T5][0] = 0x4c0000;
            if t {
                break 'b_179704;
            } else {
                c.cycles += 477;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x5d48));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5d44));
                c.g[T6][0] = 0x4c0000;
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x5d40));
                c.g[T7][0] = 0x4c0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5d3c));
                c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5c40));
                c.swc1(9, (c.g[T5][0] as u32).wrapping_add(0x5c20));
                c.g[S1][0] = 0x4c0000;
                c.swc1(10, (c.g[T6][0] as u32).wrapping_add(0x5c30));
                c.g[S2][0] = 0x4c0000;
                c.swc1(11, (c.g[T7][0] as u32).wrapping_add(0x5c40));
                c.g[S3][0] = 0x4c0000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5d38));
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5c70));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5d34));
                c.g[S5][0] = 0x4c0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5d30));
                c.g[S4][0] = 0x4c0000;
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[T3][0] = 0x4c0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5d2c));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5c10));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_add(0x5c20));
                c.swc1(8, (c.g[S1][0] as u32).wrapping_add(0x5c50));
                c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_add(0x5c30));
                c.swc1(4, (c.g[S2][0] as u32).wrapping_add(0x5c60));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5c50));
                c.swc1(5, (c.g[S3][0] as u32).wrapping_add(0x5c70));
                c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5c60));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5d28));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5c80));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5d24));
                c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_add(0x5c90));
                c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[T9][0] = 0x4c0000;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 6);
                c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_add(0x5ca0));
                c.swc1(3, (c.g[S4][0] as u32).wrapping_add(0x5c80));
                c.g[S6][0] = 0x4c0000;
                c.swc1(1, (c.g[T3][0] as u32).wrapping_add(0x5c90));
                c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5cd0));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x5c10));
                c.g[S2][0] = 0x4c0000;
                c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5cf0));
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S3][0] = 0x4c0000;
                c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x5d20));
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5d00));
                c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x5d1c));
                c.g[T8][0] = 0x4c0000;
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x4c0000;
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x5d18));
                c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5d10));
                c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x5d14));
                c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_add(0x5cb0));
                c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S7][0] = 0x4c0000;
                c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4c0000;
                c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4c0000;
                c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d20));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_add(0x5cc0));
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x5d10));
                c.g[V1][0] = 0x4c0000;
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x5d0c));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5d60));
                c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5d80));
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5d08));
                c.g[S0][0] = 0x4c0000;
                c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4c0000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d70));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4c0000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d90));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4c0000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5da0));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x4c0000;
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5db0));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x4c0000;
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x5d04));
                c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5de0));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0x4c0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5d00));
                c.g[A2][0] = 0x4c0000;
                c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5dc0));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5cfc));
                c.g[A2][0] = 0x4c0000;
                c.swc1(2, (c.g[T9][0] as u32).wrapping_add(0x5ca0));
                c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5df0));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5cf8));
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5e10));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5cf4));
                c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5cf0));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5cec));
                c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x5cb0));
                c.g[T2][0] = 0x4c0000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
                c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5e70));
                c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x5cc0));
                c.g[T2][0] = 0x4c0000;
                c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5ce8));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x5cd0));
                c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5ce0));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5ce0));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5ce4));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x4c0000;
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x5cf0));
                c.g[T4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5e90));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x4c0000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5ce0));
                c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5ea0));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5cdc));
                c.g[S0][0] = 0x4c0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5cd8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5d00));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S0][0] = 0x4c0000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5d10));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x4c0000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
                c.g[T5][0] = 0x4c0000;
                c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x5ce0));
                c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.g[T6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5f10));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[T7][0] = 0x4c0000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = 0x4c0000;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.g[S2][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5ef0));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                c.g[T7][0] = 0x4c0000;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T5][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5f20));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5f40));
                c.g[S3][0] = 0x4c0000;
                c.g[S0][0] = 0x4c0000;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5d50));
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5f50));
                c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x4c0000;
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5f60));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5cd4));
                c.g[S0][0] = 0x4c0000;
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5cd0));
                c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x5d80));
                c.g[S0][0] = 0x4c0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5ccc));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5d20));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x4c0000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5d30));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5d30));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5cc8));
                c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x4c0000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5cc4));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5d40));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5d40));
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x5cc0));
                c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x4c0000;
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5cbc));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5d50));
                c.g[S0][0] = 0x4c0000;
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x5cb8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5d60));
                c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x4c0000;
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x4c0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5cb4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5d70));
                c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x4c0000;
                c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d30));
                c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = 0x4c0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5cb0));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x5d90));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x4c0000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5da0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5d40));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x4c0000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5d50));
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4c0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4c0000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x4c0000;
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
                c.swc1(3, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5cac));
                c.g[FP][0] = 0x4c0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5ca8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x5db0));
                c.swc1(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4c0000;
                c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5ca4));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5dc0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5dd0));
                c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[FP][0];
                c.swc1(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5e00));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5ca0));
                c.g[A1][0] = 0x4c0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x5dd0));
                c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x4c0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5de0));
                c.g[A0][0] = 0x4c0000;
                c.swc1(9, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x4c0000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5df0));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5dd0));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x4c0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x5e00));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x4c0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x5e10));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5e80));
                c.swc1(10, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.swc1(11, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5c9c));
                c.g[A2][0] = c.g[A0][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x5e20));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5e20));
                c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4c0000;
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A0][0];
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5e40));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4c0000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.g[V0][0];
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5c98));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5e30));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5c94));
                c.g[A2][0] = 0x4c0000;
                c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x5e60));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5c90));
                c.g[S0][0] = c.g[A2][0];
                c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5e50));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5c8c));
                c.swc1(2, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x4c0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5c88));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5e60));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5e30));
                c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4c0000;
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x5e40));
                c.g[V1][0] = 0x4c0000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x5e50));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5e80));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A1][0] = 0x4c0000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A2][0] = 0x4c0000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5c84));
                c.g[S0][0] = 0x4c0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5c80));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5e30));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5e70));
                c.g[FP][0] = 0x4c0000;
                c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5e40));
                c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x4c0000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5c7c));
                c.g[A3][0] = 0x4c0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x5e80));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5e50));
                c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4c0000;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x5c78));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5e60));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5e90));
                c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x4c0000;
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5ec0));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5c74));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x5ea0));
                c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = 0x4c0000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x5eb0));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5f00));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x4c0000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V0][0] = c.g[FP][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5ee0));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x4c0000;
                c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[A3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5eb0));
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[T2][0] = 0x4c0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5c70));
                c.g[A3][0] = 0x4c0000;
                c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = c.g[T2][0];
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5c6c));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5ed0));
                c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = 0x4c0000;
                c.swc1(1, (c.g[T6][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x5ec0));
                c.g[A0][0] = 0x4c0000;
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x5ed0));
                c.g[A3][0] = 0x4c0000;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T2][0] = c.g[A3][0];
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x5ee0));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5f30));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S0][0] = 0x4c0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x5ef0));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5eb0));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x4c0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5c68));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5ec0));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x5f00));
                c.g[V0][0] = 0x4c0000;
                c.swc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5ed0));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5c64));
                c.g[A2][0] = 0x4c0000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5f10));
                c.g[V1][0] = 0x4c0000;
                c.swc1(4, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5ee0));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5c60));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5f00));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x5f20));
                c.g[A1][0] = 0x4c0000;
                c.swc1(5, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = 0x4c0000;
                c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5c5c));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5c58));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x5f30));
                c.swc1(6, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x4c0000;
                c.swc1(7, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(12, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5c54));
                c.sw((c.g[T2][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5c50));
                c.g[T2][0] = 0x4c0000;
                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[T5][0] = c.g[T2][0];
                c.swc1(1, (c.g[S3][0] as u32).wrapping_add(0xc));
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x5f70));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5c4c));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5c48));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5f40));
                c.swc1(3, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.swc1(4, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5c44));
                c.g[T7][0] = 0x4c0000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5c40));
                c.g[S0][0] = c.g[T7][0];
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x5f50));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5f80));
                c.swc1(5, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.swc1(6, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5c3c));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5c38));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x5f60));
                c.swc1(7, (c.g[S3][0] as u32).wrapping_add(0x4));
                c.swc1(8, (c.g[S3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x5f70));
                c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5f80));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.sw((c.g[T5][0] as u32).wrapping_add(0x4), 0u64);
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                break 'b_179704;
            }
        }
    }
    c.cycles += 11;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(30, c.g[SP][0] as u32);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    ret(c);
}

/// `CameraConsts_StaticInit_9798` (0x00179798): takes `initialize` in a0 and
/// `priority` in a1. When `priority == 0xFFFF` and `initialize != 0`, it fills the camera trigger
/// colour tables (sColor entries, 16 bytes each, D_004C5F90..D_004C6300) and
/// the world painter region type at D_004A4DF8 (0xFF, 0xFFFFFF). Otherwise
/// it does nothing. A flat static initialiser: the stores are transcribed in
/// the generated body's order, so the cycle cost, the scratch registers left
/// behind and the stack saves all match it.
pub fn camera_consts_static_init_9798(c: &mut Cpu) {
    'b_179f7c: {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[V0][0] = 0xffff;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[A1][0] != c.g[V0][0];
        c.sq(30, c.g[SP][0] as u32);
        if t {
            break 'b_179f7c;
        } else {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[T5][0] = 0x4c0000;
            if t {
                break 'b_179f7c;
            } else {
                c.cycles += 491;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x5c34));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5c30));
                c.g[T6][0] = 0x4c0000;
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x5c2c));
                c.g[T7][0] = 0x4c0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5c28));
                c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5fc0));
                c.swc1(9, (c.g[T5][0] as u32).wrapping_add(0x5fa0));
                c.g[S1][0] = 0x4c0000;
                c.swc1(10, (c.g[T6][0] as u32).wrapping_add(0x5fb0));
                c.g[S2][0] = 0x4c0000;
                c.swc1(11, (c.g[T7][0] as u32).wrapping_add(0x5fc0));
                c.g[S3][0] = 0x4c0000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5c24));
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5ff0));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5c20));
                c.g[S5][0] = 0x4c0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5c1c));
                c.g[S4][0] = 0x4c0000;
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[T3][0] = 0x4c0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5c18));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5f90));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_add(0x5fa0));
                c.swc1(8, (c.g[S1][0] as u32).wrapping_add(0x5fd0));
                c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_add(0x5fb0));
                c.swc1(4, (c.g[S2][0] as u32).wrapping_add(0x5fe0));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5fd0));
                c.swc1(5, (c.g[S3][0] as u32).wrapping_add(0x5ff0));
                c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5fe0));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5c14));
                c.g[S2][0] = 0x4c0000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5c10));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6000));
                c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6070));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 6);
                c.g[S3][0] = 0x4c0000;
                c.swc1(3, (c.g[S4][0] as u32).wrapping_add(0x6000));
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6080));
                c.swc1(1, (c.g[T3][0] as u32).wrapping_add(0x6010));
                c.g[FP][0] = 0x4c0000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x5f90));
                c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6090));
                c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x4c0000;
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_add(0x6010));
                c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x5c0c));
                c.g[T5][0] = 0x4c0000;
                c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x5c08));
                c.g[T2][0] = sx((c.g[T5][0] as u32).wrapping_add(0x6020));
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S7][0] = 0x4c0000;
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x5c04));
                c.g[T7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x6050));
                c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x5c00));
                c.g[T9][0] = 0x4c0000;
                c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[T5][0] = sx((c.g[T9][0] as u32).wrapping_add(0x6030));
                c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[T8][0] = 0x4c0000;
                c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4c0000;
                c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x60a0));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T6][0] = sx((c.g[T8][0] as u32).wrapping_add(0x6040));
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x5bfc));
                c.g[V1][0] = 0x4c0000;
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x5bf8));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60e0));
                c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6100));
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5bf4));
                c.g[FP][0] = 0x4c0000;
                c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = 0x4c0000;
                c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6060));
                c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4c0000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60f0));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4c0000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6110));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x4c0000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6130));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x4c0000;
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[T1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6170));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4c0000;
                c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x5bf0));
                c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6160));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0x4c0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5bec));
                c.g[A2][0] = 0x4c0000;
                c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6140));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5be8));
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6190));
                c.swc1(2, (c.g[FP][0] as u32).wrapping_add(0x6020));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5be4));
                c.g[FP][0] = 0x4c0000;
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5be0));
                c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5bdc));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5bd8));
                c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[T4][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6210));
                c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4c0000;
                c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x6030));
                c.g[T2][0] = 0x4c0000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
                c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(0x61f0));
                c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x6040));
                c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5bd4));
                c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x6050));
                c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x6060));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S6][0] = 0x4c0000;
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x6070));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[S6][0] = 0x4c0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x6080));
                c.g[T2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6220));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S6][0] = 0x4c0000;
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x6090));
                c.g[FP][0] = 0x4c0000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = 0x4c0000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
                c.g[T5][0] = 0x4c0000;
                c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[T7][0] = 0x4c0000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[T6][0] = sx((c.g[T7][0] as u32).wrapping_add(0x62b0));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[T7][0] = 0x4c0000;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = sx((c.g[T5][0] as u32).wrapping_add(0x62a0));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                c.g[S1][0] = 0x4c0000;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60d0));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                c.g[T5][0] = sx((c.g[S6][0] as u32).wrapping_add(0x62c0));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                c.g[S6][0] = 0x4c0000;
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S2][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6290));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5bd0));
                c.g[S3][0] = 0x4c0000;
                c.swc1(6, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0xffffffffffff0000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5bcc));
                c.g[S1][0] = 0x4c0000;
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = c.g[FP][0] | 0xff00;
                c.swc1(1, (c.g[S3][0] as u32).wrapping_add(0x6100));
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x60a0));
                c.g[S3][0] = 0x4c0000;
                c.g[T7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x62d0));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x62e0));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x60b0));
                c.g[S3][0] = c.g[S6][0];
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5bc8));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x60b0));
                c.g[S6][0] = 0x4c0000;
                c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6300));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5bc4));
                c.g[S6][0] = 0x4c0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5bc0));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x60c0));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x60c0));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5bbc));
                c.swc1(6, (c.g[S6][0] as u32).wrapping_add(0xc));
                c.g[S6][0] = 0x4c0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5bb8));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x60d0));
                c.g[S6][0] = 0x4c0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5bb4));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x60e0));
                c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = 0x4c0000;
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x4c0000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5bb0));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x60f0));
                c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x4c0000;
                c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x60b0));
                c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = 0x4c0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5bac));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x6110));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x4c0000;
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x6120));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60c0));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = 0x4c0000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x60d0));
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4c0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6120));
                c.swc1(6, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4c0000;
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5ba8));
                c.g[V0][0] = 0x4c0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5ba4));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x6130));
                c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4c0000;
                c.swc1(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5ba0));
                c.g[A0][0] = 0x4c0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5b9c));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x6140));
                c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x4c0000;
                c.swc1(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x4c0000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5b98));
                c.g[S6][0] = c.g[A1][0];
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x6150));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6150));
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x5b94));
                c.swc1(8, (c.g[S6][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A0][0];
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6180));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x6160));
                c.g[S6][0] = 0x4c0000;
                c.swc1(9, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4c0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x6170));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x4c0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x6180));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6150));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[V0][0];
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5b90));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x61a0));
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x5b8c));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x6190));
                c.swc1(10, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = 0x4c0000;
                c.swc1(11, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5b88));
                c.g[A2][0] = 0x4c0000;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x61a0));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6200));
                c.swc1(12, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[S6][0];
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x61b0));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4c0000;
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x4c0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5b84));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5b80));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(4, (c.g[A1][0] as u32).wrapping_add(0x61e0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x61c0));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5b7c));
                c.g[A1][0] = 0x4c0000;
                c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5b78));
                c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.g[A2][0] = c.g[A1][0];
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5b74));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x61d0));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x61b0));
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = 0x4c0000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x61e0));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x61c0));
                c.g[V0][0] = 0x4c0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x4c0000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x61d0));
                c.g[A0][0] = c.g[V1][0];
                c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6200));
                c.sw((c.g[S6][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A1][0] = 0x4c0000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5b70));
                c.g[A2][0] = 0x4c0000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5b6c));
                c.g[S6][0] = 0x4c0000;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x61f0));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x61b0));
                c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4c0000;
                c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x61c0));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5b68));
                c.g[A3][0] = 0x4c0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x6200));
                c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4c0000;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x5b64));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x61d0));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x6210));
                c.g[A0][0] = 0x4c0000;
                c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x61e0));
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A1][0] = 0x4c0000;
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x5b60));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6200));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x6220));
                c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = c.g[A3][0];
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x6230));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6230));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0x4c0000;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6240));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[S6][0] = 0x4c0000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x4c0000;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V1][0] = c.g[V0][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6250));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = 0x4c0000;
                c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = c.g[A0][0];
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6260));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[T2][0] = 0x4c0000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5b5c));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x6280));
                c.swc1(12, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5b58));
                c.swc1(2, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[T2][0] = 0x4c0000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x6240));
                c.g[A2][0] = 0x4c0000;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x6250));
                c.g[A3][0] = c.g[A2][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6270));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x6260));
                c.g[S6][0] = c.g[T2][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6280));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x6270));
                c.g[V0][0] = 0x4c0000;
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = 0x4c0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5b54));
                c.g[A0][0] = 0x4c0000;
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x6280));
                c.g[A1][0] = 0x4c0000;
                c.swc1(3, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6230));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5b50));
                c.g[A2][0] = 0x4c0000;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x6290));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6240));
                c.swc1(4, (c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0x4c0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5b4c));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6250));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x62a0));
                c.g[T2][0] = 0x4c0000;
                c.swc1(5, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x6260));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S6][0] = 0x4c0000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5b48));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x6270));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5b44));
                c.g[V0][0] = 0x4c0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x62b0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6280));
                c.swc1(6, (c.g[T6][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0xffffffffffffffff;
                c.swc1(7, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = 0x4c0000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x4c0000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = 0x4c0000;
                c.sw((c.g[A3][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x4c0000;
                c.sw((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[T2][0] = c.g[A3][0];
                c.sw((c.g[S6][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x62f0));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5b40));
                c.sb((c.g[GP][0] as u32).wrapping_add(0x1d08), c.g[V1][0]);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5b3c));
                c.swc1(2, (c.g[T7][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.g[V1][0] & 0xff;
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.g[V0][0] | c.g[FP][0];
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x5b38));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5b34));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x62c0));
                c.swc1(4, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.swc1(3, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x5b30));
                c.g[T5][0] = 0x4c0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5b2c));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x62d0));
                c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.swc1(5, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x5b28));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x5b24));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x62e0));
                c.swc1(6, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.swc1(7, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x62f0));
                c.sw((c.g[T2][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x6300));
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x1d08), c.g[V0][0]);
                c.sw((c.g[T2][0] as u32).wrapping_add(0x4), 0u64);
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                break 'b_179f7c;
            }
        }
    }
    c.cycles += 11;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(30, c.g[SP][0] as u32);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    ret(c);
}

/// The address `gp - off` (the generated code's `lwc1 f, -off(gp)`).
pub(crate) fn gp_minus(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `SuperpipeChaseCameraAlgorithm_ComputeDesiredEye` (0x1785f0): the ideal eye
/// point. The chase line runs from `Point1` (+0x390) to `Point2` (+0x3a0); the
/// eye sits on it at the projection of the target's position (`GetPos`, the
/// vtable slot at self+0x30), pulled back 1500 units along the line. The
/// `t4Vector` result goes to the hidden return buffer in a0; self is in a1.
pub fn superpipe_chase_camera_compute_desired_eye(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 90;
        let frame = Frame::open(c, 0xa0);
        let at = sp(c);
        frame.save(c, 16, 0x90);
        frame.save(c, 17, 0x80);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x70);
        c.g[S1][0] = c.g[A0][0];
        let this = r32(c, S0);
        // dir = normalize(Point1 - Point2)
        c.lqc2(3, this.wrapping_add(0x390));
        c.lqc2(4, this.wrapping_add(0x3a0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x20));
        c.g[A0][0] = sx(at.wrapping_add(0x20));
        c.lq(2, at.wrapping_add(0x20));
        c.sq(2, at.wrapping_add(0x10));
        c.lqc2(3, at.wrapping_add(0x10));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, at.wrapping_add(0x20));
        c.lq(2, at.wrapping_add(0x20));
        // GetPos(self): the target's position, through the vtable at self+0x30.
        let obj = c.lw(this.wrapping_add(0x30));
        c.g[A2][0] = obj;
        c.sq(2, at);
        c.g[V1][0] = c.lw(obj as u32);
        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sx((obj as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0017_866c)?;
        // rel = GetPos - Point1
        c.lqc2(3, at.wrapping_add(0x20));
        c.lqc2(4, this.wrapping_add(0x390));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x30));
        c.lwc1(2, gp_minus(c, 0x5dac));
        c.lq(3, at.wrapping_add(0x30));
        c.g[V0][0] = c.g[S1][0];
        c.sq(3, at.wrapping_add(0x10));
        // t = dot(rel, dir) / dot(dir, dir)
        c.lqc2(3, at);
        c.lqc2(5, at);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 1);
        c.lqc2(3, at.wrapping_add(0x10));
        c.lqc2(5, at);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(4, 4);
        c.mtc1(4, 0);
        c.div_s(0, 0, 1);
        c.mfc1(4, 0);
        c.lqc2(4, at);
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at.wrapping_add(0x30));
        c.lq(3, at.wrapping_add(0x30));
        c.sq(3, at.wrapping_add(0x20));
        // off = Point1 + dir * t
        c.lqc2(3, this.wrapping_add(0x390));
        c.lqc2(4, at.wrapping_add(0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x50));
        c.lq(3, at.wrapping_add(0x50));
        c.sq(3, at.wrapping_add(0x40));
        // eye = off - dir * 1500
        c.mfc1(4, 2);
        c.lqc2(4, at);
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at.wrapping_add(0x60));
        c.lq(3, at.wrapping_add(0x60));
        c.sq(3, at.wrapping_add(0x50));
        c.lqc2(3, at.wrapping_add(0x40));
        c.lqc2(4, at.wrapping_add(0x50));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x60));
        c.lq(3, at.wrapping_add(0x60));
        c.lq(16, at.wrapping_add(0x90));
        let s1 = r32(c, S1);
        c.sq(3, s1);
        c.lq(17, at.wrapping_add(0x80));
        c.g[RA][0] = c.ld(at.wrapping_add(0x70));
        c.sq(3, at.wrapping_add(0x30));
        frame.close(c);
        Ok(())
    })
}

/// `SuperpipeChaseCameraAlgorithm_ComputeLookAt` (0x178758): the look-at point,
/// 100 units above the target's position (`GetPos`, vtable slot at self+0x30),
/// with the hidden return buffer in a0 and self in a1.
pub fn superpipe_chase_camera_compute_look_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x50);
        let at = sp(c);
        frame.save(c, 16, 0x40);
        frame.save_ra(c, 0x30);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        let obj = c.lw(r32(c, A1).wrapping_add(0x30));
        c.g[A2][0] = obj;
        c.g[V0][0] = c.lw(obj as u32);
        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = sx((obj as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0017_8784)?;
        // The up-vector scaled by -100.0 (the constant at 0x4ff160), plus the position.
        c.g[V1][0] = 0x0050_0000;
        c.g[AT][0] = 0xffff_ffff_c2c8_0000;
        c.mtc1(1, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
        c.g[V0][0] = c.g[S0][0];
        c.mfc1(4, 0);
        c.lqc2(4, c.g[V1][0] as u32);
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at.wrapping_add(0x20));
        frame.restore_ra(c, 0x30);
        c.lq(3, at.wrapping_add(0x20));
        c.sq(3, at.wrapping_add(0x10));
        c.lqc2(3, at);
        c.lqc2(4, at.wrapping_add(0x10));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x20));
        c.lq(3, at.wrapping_add(0x20));
        let s0 = r32(c, S0);
        c.sq(3, s0);
        frame.restore(c, 16, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `SuperpipeChaseCameraAlgorithm_Update` (0x1787e0): one frame of the
/// superpipe chase camera. Runs the base chase update, then computes the eye
/// (`ComputeDesiredEye`) and the look-at point (`ComputeLookAt`), stores both,
/// and pulls the eye toward the target when the eye-to-look distance is out of
/// range (above 600 units it is clamped; below 1 unit it is pinned).
pub fn superpipe_chase_camera_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0xd0);
        let at = sp(c);
        c.lwc1(12, gp_minus(c, 0x5da8));
        frame.save(c, 17, 0xb0);
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(13, gp_minus(c, 0x5da4));
        c.lwc1(14, gp_minus(c, 0x5da0));
        c.lwc1(15, gp_minus(c, 0x5d9c));
        c.lwc1(16, gp_minus(c, 0x5d98));
        c.lwc1(17, gp_minus(c, 0x5d94));
        c.lwc1(18, gp_minus(c, 0x5d90));
        c.lwc1(19, gp_minus(c, 0x5d8c));
        frame.save(c, 16, 0xc0);
        frame.save_ra(c, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0016_2568, 0x0017_881c)?;
        c.lwc1(0, gp_minus(c, 0x5d88));
        c.g[S0][0] = sx(at.wrapping_add(0x70));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.swc1(0, r32(c, S1));
        Callee {
            addr: 0x0017_85f0,
            f: ee::s02::f_001785f0,
        }
        .call(c, 0x0017_8834)?;
        c.lq(2, at.wrapping_add(0x70));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.sq(2, r32(c, S1).wrapping_add(0x40));
        Callee {
            addr: 0x0017_8758,
            f: ee::s02::f_00178758,
        }
        .call(c, 0x0017_8848)?;
        c.lq(2, at.wrapping_add(0x70));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.sq(2, r32(c, S1).wrapping_add(0x20));
        c.lqc2(3, r32(c, S1).wrapping_add(0x20));
        c.lqc2(4, r32(c, S1).wrapping_add(0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x80));
        c.lq(2, at.wrapping_add(0x80));
        c.sq(2, at.wrapping_add(0x70));
        c.lqc2(3, at.wrapping_add(0x70));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 1);
        // The distance |eye - look| against 1.0.
        'b_178924: {
            'b_17891c: {
                'b_1788d4: {
                    c.c_lt_s(1, 0);
                    let t = !c.fcc();
                    c.g[V0][0] = 0x0050_0000;
                    if t {
                        break 'b_1788d4;
                    }
                    // Distance below 1: pin the eye to a scaled up-vector.
                    c.cycles += 11;
                    c.g[AT][0] = 0x4416_0000;
                    c.mtc1(1, 0);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
                    c.mfc1(3, 0);
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at.wrapping_add(0x90));
                    c.lq(2, at.wrapping_add(0x90));
                    c.sq(2, at.wrapping_add(0x70));
                    break 'b_17891c;
                }
                // Distance at least 1: clamp the pull to 600 units.
                c.cycles += 6;
                c.g[AT][0] = 0x4416_0000;
                c.mtc1(1, 0);
                c.c_lt_s(0, 1);
                let t = !c.fcc();
                c.lq(16, at.wrapping_add(0xc0));
                if t {
                    break 'b_178924;
                }
                c.cycles += 12;
                c.div_s(0, 0, 1);
                c.mfc1(2, 0);
                c.lqc2(4, at.wrapping_add(0x70));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at.wrapping_add(0x70));
                c.lqc2(3, r32(c, S1).wrapping_add(0x20));
                c.lqc2(4, at.wrapping_add(0x70));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at.wrapping_add(0x90));
                c.lq(2, at.wrapping_add(0x90));
                c.sq(2, r32(c, S1).wrapping_add(0x40));
                break 'b_17891c;
            }
            c.cycles += 2;
            c.sq(2, at.wrapping_add(0x80));
            c.lq(16, at.wrapping_add(0xc0));
            break 'b_178924;
        }
        c.cycles += 4;
        frame.restore(c, 17, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// `Post_Race_1ChaseCameraAlgorithm_ctor` (0x1789e8): the post-race chase
/// camera's constructor. Runs the base chase constructor, sets the vtable and
/// the initial fields, then normalises the rider's direction into `+0x390`
/// (or, when the vector is degenerate, the fallback at `+0x398`).
pub fn post_race_1_chase_camera_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x80);
        let at = sp(c);
        frame.save(c, 16, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x60);
        frame.save(c, 18, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0016_2318, 0x0017_8a08)?;
        let this = r32(c, S0);
        c.g[S2][0] = sx(this.wrapping_add(0x390));
        c.g[V1][0] = 0x0046_0000;
        c.g[V0][0] = 0x44;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3428));
        c.sw(this.wrapping_add(0xc), c.g[V0][0]);
        c.sw(this.wrapping_add(0x10), c.g[V1][0]);
        c.g[A0][0] = sx(at.wrapping_add(0x10));
        c.sw(this.wrapping_add(0x3a0), 0);
        // First virtual call on the rider object (slot +0x18): the direction.
        c.g[V0][0] = c.lw(r32(c, S1));
        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0017_8a3c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.lqc2(3, at.wrapping_add(0x10));
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
        let t = !c.fcc();
        c.g[A0][0] = sx(at.wrapping_add(0x20));
        'b_178adc: {
            'b_178ac4: {
                if t {
                    break 'b_178ac4;
                }
                c.cycles += 18;
                c.g[V0][0] = c.lw(r32(c, S1));
                c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                let target = c.g[V1][0] as u32;
                c.g[A1][0] = sx(r32(c, S1).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, target, 0x0017_8a90)?;
                c.lqc2(3, at.wrapping_add(0x20));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, at.wrapping_add(0x30));
                c.lq(2, at.wrapping_add(0x30));
                c.sq(2, at);
                break 'b_178adc;
            }
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, S1));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, target, 0x0017_8adc)?;
        }
        'b_178b58: {
            'b_178b48: {
                c.cycles += 18;
                c.lq(2, at);
                c.lwc1(0, gp_minus(c, 0x5d80));
                c.sq(2, r32(c, S2));
                c.sw(this.wrapping_add(0x398), 0);
                c.lqc2(3, this.wrapping_add(0x390));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(2, 22);
                c.mtc1(2, 1);
                c.c_lt_s(0, 1);
                let t = !c.fcc();
                c.g[V0][0] = 0x0050_0000;
                if t {
                    break 'b_178b48;
                }
                c.cycles += 9;
                c.mfc1(2, 1);
                c.qmtc2(2, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, this.wrapping_add(0x390));
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 4, Src::Q);
                c.sqc2(5, this.wrapping_add(0x390));
                c.g[V0][0] = c.g[S0][0];
                break 'b_178b58;
            }
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
            c.lq(3, c.g[V0][0] as u32);
            c.sq(3, this.wrapping_add(0x390));
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x60);
        frame.restore(c, 16, 0x70);
        frame.restore(c, 18, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `Post_Race_1ChaseCameraAlgorithm_dtor` (0x178b70): stores the post-race
/// vtable (0x45cbd8) at +0x10 and runs the base chase destructor.
pub fn post_race_1_chase_camera_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0046_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3428));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, 0x0016_2458, 0x0017_8b8c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Post_Race_1ChaseCameraAlgorithm_GlobalCtor` (0x179738): the static
/// initialiser of the post-race camera tables (`StaticInit(1, 0xffff)`).
pub fn post_race_1_chase_camera_global_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x0017_8f58,
            f: ee::s02::f_00178f58,
        }
        .call(c, 0x0017_974c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraConsts_GlobalCtor_9fa8` (0x179fa8): the camera constants' static
/// initialiser (`CameraConsts_StaticInit(1, 0xffff)`).
pub fn camera_consts_global_ctor_9fa8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x0017_9798,
            f: ee::s02::f_00179798,
        }
        .call(c, 0x0017_9fbc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cDebugMenuScreen::~cDebugMenuScreen(this, delete_flag)` (0x1dcd00): frees the
/// background set at `this + 0x8c10`, stores the screen vtable word (0x46d948)
/// at `+0`, and with bit 0 of the flag set deletes `this`.
pub fn debug_menu_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x2;
        c.g[A0][0] = menu_sx_add(c.g[S1][0], 0x8c10);
        call_ptr(c, 0x0017_abe0, 0x001d_cd28)?;
        c.g[S0][0] &= 0x1;
        c.g[V0][0] = 0x0047_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26b8));
        let delete = c.g[S0][0] != 0;
        let this = c.g[S1][0] as u32;
        let value = c.g[V0][0];
        Addr(this).sw(c, value);
        if delete {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x001d_cd44)?;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[allow(unused_labels, unreachable_code, unused_assignments)]
/// `int get_cCTBoundObjEllipse(reader, obj)` (0x171a20): reads an ellipse trigger
/// bound (position, scale, heading, pitch, roll) through the object's field
/// accessors and returns the sum of the reads. Sets `obj->Type = 0`.
pub fn camera_ellipse_bound_obj_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 59;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171a54);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171a54)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171a60);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171a60)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171a78);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171a78)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171a84);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171a84)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171a9c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171a9c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171aa8);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171aa8)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171ac0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171ac0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171acc);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171acc)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171ae4);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171ae4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171af0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171af0)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

#[allow(unused_labels, unreachable_code, unused_assignments)]
/// `int get_cCTBoundObjBox(reader, obj)` (0x171b10): reads a box trigger bound
/// (the same five fields as the ellipse). Sets `obj->Type = 1`.
pub fn camera_box_bound_obj_read_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0x1;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171b48);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171b48)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171b54);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171b54)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171b6c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171b6c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171b78);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171b78)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171b90);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171b90)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171b9c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171b9c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171bb4);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171bb4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171bc0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171bc0)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171bd8);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171bd8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171be4);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171be4)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

#[allow(unused_labels, unreachable_code, unused_assignments)]
/// `int get_cCTBoundObjLine(reader, obj)` (0x171c00): reads a line trigger bound
/// (the five fields, then the two end points). Sets `obj->Type = 2`.
pub fn camera_line_bound_obj_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 68;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.g[V0][0] = 0x2;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171c38);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171c38)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171c44);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171c44)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171c5c);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171c5c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171c68);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d25a0, 0x00171c68)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171c80);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171c80)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171c8c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171c8c)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171ca4);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171ca4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171cb0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171cb0)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171cc8);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171cc8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171cd4);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002d2510, 0x00171cd4)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c));
        c.g[RA][0] = sx(0x00171ce4);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x002d25a0, 0x00171ce4)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171cf4);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x002d25a0, 0x00171cf4)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

#[allow(unused_labels, unreachable_code, unused_assignments)]
/// `void get_camspline(reader, out)` (0x171d58): allocates a spline trigger
/// (`str_Camera_Trigger_Spline`, 0x58 bytes), copies its template, stores it in
/// `*out` and reads the five object fields plus the four control points.
pub fn camera_trigger_read_spline(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 148;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[A2][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A3][0] = 0u64;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = 0x58;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x00171d8c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3e90));
        call_ptr(c, 0x00317d70, 0x00171d8c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V0][0] = 0x460000;
        c.g[V1][0] = 0x500000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3c10));
        c.g[A3][0] = c.g[V1][0];
        c.sw((c.g[A1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
        c.g[T0][0] = 0xffffffffffffffff;
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = c.g[A1][0];
        c.swc1(0, c.g[SP][0] as u32);
        c.g[A0][0] = 0x3;
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xf28));
        c.ldl(9, (c.g[T4][0] as u32).wrapping_add(0x7));
        c.ldr(9, c.g[T4][0] as u32);
        c.g[T2][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x8));
        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.sdr(9, c.g[A1][0] as u32);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T2][0]);
        c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x14), 0u64);
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[SP][0] as u32);
        c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[A1][0] as u32).wrapping_add(0x1f));
        c.sdr(3, (c.g[A1][0] as u32).wrapping_add(0x18));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x20), c.g[T1][0]);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
        c.ldl(12, (c.g[A0][0] as u32).wrapping_add(0x7));
        c.ldr(12, c.g[A0][0] as u32);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.sdl(12, (c.g[A2][0] as u32).wrapping_add(0x2f));
        c.sdr(12, (c.g[A2][0] as u32).wrapping_add(0x28));
        c.sw((c.g[A2][0] as u32).wrapping_add(0x30), c.g[V0][0]);
        c.g[T3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
        c.ldl(8, (c.g[T3][0] as u32).wrapping_add(0x7));
        c.ldr(8, c.g[T3][0] as u32);
        c.g[T1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
        c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x3b));
        c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x34));
        c.sw((c.g[A2][0] as u32).wrapping_add(0x3c), c.g[T1][0]);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
        c.ldl(12, (c.g[A0][0] as u32).wrapping_add(0x7));
        c.ldr(12, c.g[A0][0] as u32);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.sdl(12, (c.g[A2][0] as u32).wrapping_add(0x47));
        c.sdr(12, (c.g[A2][0] as u32).wrapping_add(0x40));
        c.sw((c.g[A2][0] as u32).wrapping_add(0x48), c.g[V0][0]);
        c.g[T3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xf28));
        c.ldl(8, (c.g[T3][0] as u32).wrapping_add(0x7));
        c.ldr(8, c.g[T3][0] as u32);
        c.g[T1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
        c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x53));
        c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x4c));
        c.sw((c.g[A2][0] as u32).wrapping_add(0x54), c.g[T1][0]);
        c.sw(c.g[S1][0] as u32, c.g[A1][0]);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171e90);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171e90)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00171e9c);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002d25a0, 0x00171e9c)?;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171eb8);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171eb8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00171ec4);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002d25a0, 0x00171ec4)?;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171ee0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171ee0)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00171eec);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002d2510, 0x00171eec)?;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171f08);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171f08)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00171f14);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002d2510, 0x00171f14)?;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00171f30);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00171f30)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00171f3c);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x002d2510, 0x00171f3c)?;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[RA][0] = sx(0x00171f50);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x002d25a0, 0x00171f50)?;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[RA][0] = sx(0x00171f64);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x34));
        call_ptr(c, 0x002d25a0, 0x00171f64)?;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[RA][0] = sx(0x00171f78);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x002d25a0, 0x00171f78)?;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00171f8c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c));
        call_ptr(c, 0x002d25a0, 0x00171f8c)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `DebugMenu_AddWorldIfMissing` (0x17a830): `int f(self, name, tag)`. The
/// reference calls it `cTrackFileMan_checkLocation`. Returns 0 when a row of
/// the location table already has file `name` and big file `tag` (rows are
/// 0x8c bytes: `File` at +4, `BigFile` at +0x44). Otherwise it opens the
/// world's big file (the BigFile ctor on `worlds/<tag>`), asks
/// `World_SdbHasSections` whether a section `name` exists and returns the
/// `bFound` flag it sets (0 when that gives no map).
pub fn debug_menu_add_world_if_missing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x40);
        c.g[S2][0] = 0;
        c.g[S3][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A2][0];
        let this = c.g[S3][0] as u32;
        c.g[V0][0] = Addr(this).lw(c);
        let no_rows = (c.g[V0][0] as i64) <= 0;
        c.g[S5][0] = c.g[A1][0];

        let mut listed = false;
        if !no_rows {
            c.cycles += 2;
            c.g[S1][0] = sx(this.wrapping_add(0x44));
            c.g[S0][0] = sx(this.wrapping_add(0x4));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0041_aa88, 0x0017_a87c)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(this).lw(c);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041_aa88, 0x0017_a890)?;
                    if c.g[V0][0] == 0 {
                        // Both names match: the location is listed already.
                        c.cycles += 2;
                        listed = true;
                        break;
                    }
                    c.g[V0][0] = Addr(this).lw(c);
                }
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8c));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0017_a870);
                }
            }
        }

        if listed {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 40;
            c.g[S1][0] = sx(frame.at(c, 0x10));
            c.g[S0][0] = sx(0x0046_0000u32.wrapping_sub(0x31e0));
            // workStr = BXString(tag) ; base = "data/worlds" + workStr
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_82d8, 0x0017_a8cc)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0031_86d0, 0x0017_a8dc)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_8260, 0x0017_a8e8)?;
            // workStr = base + "big" ; BigFile(pbf) opened on workStr.
            c.g[S2][0] = sx(frame.at(c, 0x20));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = sx(0x004a_1298);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_8630, 0x0017_a900)?;
            let work_str = Addr(frame.at(c, 0x10)).lw(c);
            c.g[A1][0] = work_str;
            c.g[A2][0] = 0x100;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_68b0, 0x0017_a910)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_8260, 0x0017_a91c)?;
            // nm.sectionName = name; bFound = isValidMap(base, 1, &nm)
            let base_str = Addr(sp(c)).lw(c);
            c.g[A0][0] = base_str;
            c.g[A2][0] = sx(frame.at(c, 0x30));
            let name = c.g[S5][0];
            Addr(frame.at(c, 0x30)).sw(c, name);
            c.g[A1][0] = 1;
            call_ptr(c, 0x003a_6948, 0x0017_a930)?;
            let found = Addr(frame.at(c, 0x34)).lw(c);
            c.g[S0][0] = found;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 2;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = 0;
            }
            call_ptr(c, 0x0031_68e8, 0x0017_a944)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_8260, 0x0017_a950)?;
            c.g[V0][0] = c.g[S0][0];
        }

        c.cycles += 9;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `t4Vector* CameraTrigger_FindBoundaryPoint(out, vol, pos, step)`
/// (0x171fa8; `cCTVolume_getBoundaryPointClosestTo`): finds the point on the
/// trigger volume's boundary along the ray from the volume's centre through
/// `pos`. The centre is the midpoint of the volume's bounding box (vtable slot
/// 6, `boundBox`); starting at `pos`, the probe doubles its distance from the
/// centre until the volume's `containsPoint` (slot 7) says outside, then
/// `ceil(log(len / step)) + 1` bisection steps between the last inside and the
/// first outside point narrow it down. Returns `out` (the outside point).
///
/// The body is resumable in the original (a due event at a loop back edge
/// unwinds and re-enters at the loop head), so a `pc` other than the entry
/// runs the generated body; at install, name the generated twin
/// `f_00171fa8__generated` there.
pub fn oc680_camera_trigger_find_boundary_point(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{W, Y, Z};
    if c.pc != 0x0017_1fa8 {
        return ee::s02::f_00171fa8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x80);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0xb8);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xb0);
        c.g[A0][0] = sx(frame.at(c, 0x10));
        frame.save_ra(c, 0x70);
        c.mov_s(21, 12);
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 20);
        // volume->boundBox(out = sp+0x10 (min), sp+0x20 (max))
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0017_1ff4)?;
        // centre = (min + max) * 0.5, at sp+0
        c.lqc2(3, frame.at(c, 0x10));
        c.lqc2(4, frame.at(c, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x40));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 0);
        c.lq(2, frame.at(c, 0x40));
        c.sq(2, frame.at(c, 0x30));
        c.mfc1(3, 0);
        c.lqc2(4, frame.at(c, 0x30));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x40));
        c.lq(2, frame.at(c, 0x40));
        c.sq(2, frame.at(c, 0));
        // dir = pos - centre (sp+0x10), outside = pos (sp+0x20), inside = centre (sp+0x30)
        c.lqc2(3, c.g[S0][0] as u32);
        c.lqc2(4, frame.at(c, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x20));
        c.lq(4, c.g[S0][0] as u32);
        c.lq(2, frame.at(c, 0x20));
        c.lq(3, frame.at(c, 0));
        c.sq(2, frame.at(c, 0x10));
        c.sq(4, frame.at(c, 0x20));
        c.sq(3, frame.at(c, 0x30));
        c.g[S0][0] = sx(frame.at(c, 0x20));
        // Grow: while volume->containsPoint(&outside), inside = outside,
        // outside = centre + dir * k, k *= 2.
        loop {
            c.cycles += 8;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0017_20b8)?;
            let inside = c.g[V0][0] != 0;
            c.lq(2, frame.at(c, 0x20));
            if !inside {
                break;
            }
            if back_edge_unwinds(c, 0x0017_2060) {
                return Ok(());
            }
            c.cycles += 16;
            c.sq(2, frame.at(c, 0x30));
            c.mfc1(3, 20);
            c.lqc2(4, frame.at(c, 0x10));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x50));
            c.lq(2, frame.at(c, 0x50));
            c.add_s(20, 20, 20);
            c.sq(2, frame.at(c, 0x40));
            c.lqc2(3, frame.at(c, 0));
            c.lqc2(4, frame.at(c, 0x40));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x60));
            c.lq(2, frame.at(c, 0x60));
            c.sq(2, frame.at(c, 0x20));
            c.sq(2, frame.at(c, 0x50));
        }
        // steps = (int)logf(|outside - inside| / step) + 1
        c.cycles += 24;
        c.lqc2(3, frame.at(c, 0x20));
        c.lqc2(4, frame.at(c, 0x30));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x50));
        c.lq(2, frame.at(c, 0x50));
        c.sq(2, frame.at(c, 0x40));
        c.lqc2(3, frame.at(c, 0x40));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 12);
        c.div_s(12, 12, 21);
        call_ptr(c, 0x0040_da10, 0x0017_210c)?;
        c.cvt_w_s(1, 0);
        c.mfc1(3, 1);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        let no_steps = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = c.g[V0][0];
        if !no_steps {
            c.cycles += 2;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 20);
            // Bisect: mid = (outside + inside) * 0.5; inside or outside = mid.
            loop {
                c.cycles += 21;
                c.lqc2(3, frame.at(c, 0x20));
                c.lqc2(4, frame.at(c, 0x30));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x60));
                c.g[A1][0] = sx(frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x60));
                c.sq(2, frame.at(c, 0x50));
                c.mfc1(3, 20);
                c.lqc2(4, frame.at(c, 0x50));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x60));
                c.lq(2, frame.at(c, 0x60));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.sq(2, frame.at(c, 0x40));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0017_2174)?;
                let outside = c.g[V0][0] == 0;
                c.lq(2, frame.at(c, 0x40));
                if outside {
                    c.cycles += 1;
                    c.sq(2, frame.at(c, 0x20));
                } else {
                    c.cycles += 2;
                    c.sq(2, frame.at(c, 0x30));
                }
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                if c.g[S0][0] == 0 {
                    break;
                }
                if back_edge_unwinds(c, 0x0017_2128) {
                    return Ok(());
                }
            }
        }
        // *out = outside
        c.cycles += 11;
        c.lq(3, frame.at(c, 0x20));
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0xa0);
        c.sq(3, c.g[S2][0] as u32);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `void BoundedCameraAlgorithmBase_ResetForRider(self, key)` (0x1732e8;
/// reference `cTriggerAlgorithm::snap`): re-aims the trigger camera for the
/// rider with AI index `key` (nothing happens for another rider). Reads the
/// target's position and front vector through its vtable, offsets the focus
/// by `HeightOffset` (+0x6c) along the world Z axis and by `LeadDistance`
/// (+0x74) along the front, derives `Heading` (+0x50) and `Pitch` (+0x54)
/// from the front (two inlined `atan2`s with their quadrant fixes and the
/// angle wrap), places the camera at `focus - dir * IdealDist` (+0x40, from
/// the heading and pitch sin/cos) and calls `update()` (vtable slot at +0x28).
/// The body is the generated labelled-block structure: every register, cycle
/// and call is kept.
pub fn oc680_bounded_camera_algorithm_base_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        'b_17365c: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S0][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xa8));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0017332c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0017332c)?;
            let t = c.g[S0][0] != c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            if t {
                break 'b_17365c;
            } else {
                'b_17346c: {
                    'b_17342c: {
                        c.cycles += 51;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.g[S0][0];
                        c.mtc1(0, 22);
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let t = c.g[A2][0] as u32;
                        c.g[RA][0] = sx(0x00173358);
                        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        call_ptr(c, t, 0x00173358)?;
                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                        c.sq(3, c.g[SP][0] as u32);
                        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0017337c);
                        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        call_ptr(c, t, 0x0017337c)?;
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x6c));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = 0x500000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
                        c.mfc1(3, 0);
                        c.lqc2(4, c.g[V0][0] as u32);
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lqc2(3, c.g[SP][0] as u32);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, c.g[SP][0] as u32);
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x74));
                        c.mfc1(3, 0);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lqc2(3, c.g[SP][0] as u32);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, c.g[SP][0] as u32);
                        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.c_eq_s(20, 22);
                        let t = !c.fcc();
                        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_17342c;
                        } else {
                            c.cycles += 4;
                            c.c_eq_s(21, 22);
                            let t = c.fcc();
                            c.mov_s(12, 21);
                            if t {
                                break 'b_17346c;
                            } else {
                                c.cycles += 4;
                                c.c_le_s(22, 21);
                                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6428));
                                let t = !c.fcc();

                                if t {
                                    break 'b_17346c;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6424));
                                    let t = true;

                                    if t {
                                        break 'b_17346c;
                                    } else {
                                        break 'b_17342c;
                                    }
                                }
                            }
                        }
                    }
                    'b_173468: {
                        c.cycles += 7;
                        c.div_s(12, 21, 20);
                        c.g[RA][0] = sx(0x00173438);
                        call_ptr(c, 0x0031c228, 0x00173438)?;
                        c.c_lt_s(20, 22);
                        let t = !c.fcc();
                        c.mov_s(2, 0);
                        if t {
                            break 'b_173468;
                        } else {
                            'b_173460: {
                                c.cycles += 3;
                                c.c_lt_s(22, 21);
                                let t = !c.fcc();

                                if t {
                                    break 'b_173460;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6420));
                                    let t = true;
                                    c.add_s(2, 2, 0);
                                    if t {
                                        break 'b_173468;
                                    } else {
                                        break 'b_173460;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x641c));
                            c.sub_s(2, 2, 0);
                            break 'b_173468;
                        }
                    }
                    c.cycles += 1;
                    c.mov_s(12, 2);
                    break 'b_17346c;
                }
                'b_1734a8: {
                    c.cycles += 11;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6418));
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 1);
                    c.mul_s(0, 12, 0);
                    c.add_s(0, 0, 1);
                    c.cvt_w_s(5, 0);
                    c.cvt_s_w(5, 5);
                    c.c_lt_s(0, 5);
                    let t = !c.fcc();
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_1734a8;
                    } else {
                        c.cycles += 4;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.sub_s(5, 5, 0);
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'b_1734a8;
                    }
                }
                'b_17355c: {
                    'b_173558: {
                        'b_173554: {
                            'b_173514: {
                                c.cycles += 16;
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6414));
                                c.mul_s(3, 0, 0);
                                c.mul_s(4, 1, 1);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.mul_s(2, 5, 2);
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                                c.mtc1(0, 22);
                                c.add_s(3, 3, 4);
                                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x18));
                                c.sqrt_s(20, 3);
                                c.sub_s(2, 12, 2);
                                c.c_eq_s(20, 22);
                                let t = !c.fcc();
                                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_173514;
                                } else {
                                    c.cycles += 4;
                                    c.c_eq_s(21, 22);
                                    let t = c.fcc();
                                    c.mov_s(12, 21);
                                    if t {
                                        break 'b_173554;
                                    } else {
                                        c.cycles += 4;
                                        c.c_le_s(22, 21);
                                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6410));
                                        let t = !c.fcc();
                                        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x78));
                                        if t {
                                            break 'b_173558;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x640c));
                                            let t = true;
                                            c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x58));
                                            if t {
                                                break 'b_17355c;
                                            } else {
                                                break 'b_173514;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_173550: {
                                c.cycles += 7;
                                c.div_s(12, 21, 20);
                                c.g[RA][0] = sx(0x00173520);
                                call_ptr(c, 0x0031c228, 0x00173520)?;
                                c.c_lt_s(20, 22);
                                let t = !c.fcc();
                                c.mov_s(1, 0);
                                if t {
                                    break 'b_173550;
                                } else {
                                    'b_173548: {
                                        c.cycles += 3;
                                        c.c_lt_s(22, 21);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_173548;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6408));
                                            let t = true;
                                            c.add_s(1, 1, 0);
                                            if t {
                                                break 'b_173550;
                                            } else {
                                                break 'b_173548;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6404));
                                    c.sub_s(1, 1, 0);
                                    break 'b_173550;
                                }
                            }
                            c.cycles += 1;
                            c.mov_s(12, 1);
                            break 'b_173554;
                        }
                        c.cycles += 1;
                        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x78));
                        break 'b_173558;
                    }
                    c.cycles += 1;
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x58));
                    break 'b_17355c;
                }
                'b_1735a8: {
                    c.cycles += 16;
                    c.neg_s(2, 2);
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x6400));
                    c.neg_s(1, 1);
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x63fc));
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 4);
                    c.mul_s(2, 2, 3);
                    c.mul_s(1, 12, 1);
                    c.add_s(1, 1, 2);
                    c.mul_s(0, 1, 0);
                    c.add_s(0, 0, 4);
                    c.cvt_w_s(2, 0);
                    c.cvt_s_w(2, 2);
                    c.c_lt_s(0, 2);
                    let t = !c.fcc();

                    if t {
                        break 'b_1735a8;
                    } else {
                        c.cycles += 3;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.sub_s(2, 2, 0);
                        break 'b_1735a8;
                    }
                }
                c.cycles += 45;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x63f8));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x64));
                c.mul_s(0, 2, 0);
                c.sub_s(0, 1, 0);
                c.g[RA][0] = sx(0x001735c8);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x54));
                call_ptr(c, 0x0031be50, 0x001735c8)?;
                c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x54));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x68));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x6c));
                c.g[RA][0] = sx(0x001735dc);
                c.neg_s(12, 12);
                call_ptr(c, 0x0031be50, 0x001735dc)?;
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x6c));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.mul_s(1, 0, 1);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                c.mul_s(0, 0, 3);
                c.neg_s(2, 2);
                c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x70));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.mfc1(3, 3);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lqc2(3, c.g[SP][0] as u32);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x40));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00173658);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00173658)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                break 'b_17365c;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xa8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

/// `DebugMenu_DispatchItemOp` (0x0017a158): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn debug_menu_dispatch_item_op(c: &mut Cpu) {
    if c.pc != 0x0017a158 {
        return ee::s02::f_0017a158__generated(c);
    }
    guest(c, |c| {
        'b_17a214: {
            'b_17a210: {
                'b_17a20c: {
                    c.cycles += 4;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = (c.g[A1][0] as i64) < 0;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_17a20c;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x8c;
                        if t {
                            break 'b_17a20c;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = (c.g[A2][0] < 0x7u64) as u64;
                            c.mult(0, 7, 5, 2);
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            let t = c.g[V1][0] == 0u64;
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            if t {
                                break 'b_17a20c;
                            } else {
                                'b_17a1e0: {
                                    'b_17a1d0: {
                                        'b_17a1cc: {
                                            'b_17a1c8: {
                                                'b_17a1c0: {
                                                    'b_17a1b8: {
                                                        'b_17a1b0: {
                                                            c.cycles += 7;
                                                            c.g[V0][0] = 0x460000;
                                                            c.g[V1][0] =
                                                                sx((c.g[A2][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x3200));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                            let t = c.g[A0][0] as u32;
                                                            match t {
                                                                0x0017a1a8 => {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[V0][0] = (c.g[S0][0]
                                                                        < 0x5u64)
                                                                        as u64;
                                                                    if t {
                                                                        break 'b_17a1d0;
                                                                    } else {
                                                                        break 'b_17a1b0;
                                                                    }
                                                                }
                                                                0x0017a1b0 => {
                                                                    break 'b_17a1b0;
                                                                }
                                                                0x0017a1b8 => {
                                                                    break 'b_17a1b8;
                                                                }
                                                                0x0017a1c0 => {
                                                                    break 'b_17a1c0;
                                                                }
                                                                0x0017a1c8 => {
                                                                    break 'b_17a1c8;
                                                                }
                                                                0x0017a1e0 => {
                                                                    break 'b_17a1e0;
                                                                }
                                                                _ => {
                                                                    c.jump_out(
                                                                        t, 0x0017a158, 0x0017a220,
                                                                    );
                                                                    return Ok(());
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[V0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_sub(0x5));
                                                        if t {
                                                            break 'b_17a1cc;
                                                        } else {
                                                            break 'b_17a1b8;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_sub(0x8));
                                                    if t {
                                                        break 'b_17a1cc;
                                                    } else {
                                                        break 'b_17a1c0;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_sub(0xb));
                                                if t {
                                                    break 'b_17a1cc;
                                                } else {
                                                    break 'b_17a1c8;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xe));
                                            break 'b_17a1cc;
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                                        break 'b_17a1d0;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_17a20c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        if t {
                                            break 'b_17a214;
                                        } else {
                                            break 'b_17a1e0;
                                        }
                                    }
                                }
                                c.cycles += 11;
                                c.g[RA][0] = sx(0x0017a1e8);
                                call_ptr(c, 0x0014dc80, 0x0017a1e8)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x0017a1f4);
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x0014dd58, 0x0017a1f4)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x0017a200);
                                c.g[A1][0] = c.g[S0][0];
                                call_ptr(c, 0x00144cc0, 0x0017a200)?;
                                c.g[V1][0] = c.g[V0][0];
                                let t = c.g[V1][0] == 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_17a210;
                                } else {
                                    break 'b_17a20c;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_17a210;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_17a214;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `void DebugMenu_InitWorldList(TrackFileMan* self)` (0x17a638, the C++
/// `cTrackFileMan_scanLocations`): clears the track list (count, BAM and
/// WC-BAM start counters, the 256 location records), asks the new-race
/// interface for the 22 location file names, lets `DebugMenu_CheckWorldFiles`
/// verify them and then adds one record per name that passed: file name copy,
/// the big-file pointer (`0x4a1290`), location id, BAM flag, and the WC-BAM
/// start flag when `DebugMenu_IsTrackSelectable` agrees.
pub fn debug_menu_init_world_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
        c.g[V0][0] = 0x8000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.g[A1][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[A2][0] = 0x8c00;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[S2][0] = 0u64;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
        c.sw(c.g[S1][0] as u32, 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc08), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc04), 0u64);
        call_ptr(c, 0x003e6448, 0x0017a690)?;
        call_ptr(c, 0x0014dc80, 0x0017a698)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014dd58, 0x0017a6a4)?;
        c.g[FP][0] = c.g[V0][0];
        c.g[S0][0] = c.g[SP][0];
        'l_17a6b0: loop {
            c.cycles += 8;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[FP][0];
            call_ptr(c, 0x00144c48, 0x0017a6bc)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[V0][0] = ((c.g[S2][0] as i64) < 22i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0017a6b0);
                }
                continue 'l_17a6b0;
            } else {
                'b_17a800: {
                    c.cycles += 8;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0x16;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
                    call_ptr(c, 0x0017a978, 0x0017a6e8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
                    if t {
                        break 'b_17a800;
                    } else {
                        'b_17a7fc: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw(c.g[S1][0] as u32);
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 256i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[S4][0] = 0u64;
                            if t {
                                break 'b_17a7fc;
                            } else {
                                c.cycles += 6;
                                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x88));
                                c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x84));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lw(c.g[S5][0] as u32);
                                'l_17a718: loop {
                                    'b_17a7dc: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S3][0] = 0x8c;
                                        if t {
                                            break 'b_17a7dc;
                                        } else {
                                            'b_17a7d4: {
                                                c.cycles += 38;
                                                c.g[V0][0] = 0x8000;
                                                c.mult(0, 16, 16, 19);
                                                c.g[S2][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[A0][0] = c.g[FP][0];
                                                c.g[A1][0] = c.g[S4][0];
                                                c.g[S6][0] = 0x1;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[S0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                                call_ptr(c, 0x00144c48, 0x0017a744)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x004166f4, 0x0017a750)?;
                                                c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                                c.g[V0][0] = 0x4a0000;
                                                c.g[A1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x1290));
                                                c.g[A0][0] = c.g[S1][0];
                                                c.mult(0, 2, 3, 19);
                                                c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x44),
                                                    c.g[A1][0],
                                                );
                                                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                c.mult(0, 3, 2, 19);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                                c.sw(c.g[V0][0] as u32, c.g[S4][0]);
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
                                                c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                                c.lo[0] = c.g[V0][0];
                                                c.madd(0, 3, 3, 19);
                                                c.sw(c.g[V1][0] as u32, c.g[S6][0]);
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0xc04));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0xc04),
                                                    c.g[V0][0],
                                                );
                                                c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                                c.mult(0, 2, 3, 19);
                                                c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                                                call_ptr(c, 0x00179fc8, 0x0017a7b0)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                if t {
                                                    break 'b_17a7d4;
                                                } else {
                                                    c.cycles += 7;
                                                    c.mult(0, 3, 2, 19);
                                                    c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[S1][0] as u32));
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x8c),
                                                        c.g[S6][0],
                                                    );
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0xc08)
                                                        );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                    c.sw(
                                                        (c.g[S2][0] as u32).wrapping_add(0xc08),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                    break 'b_17a7d4;
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                            break 'b_17a7dc;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[S4][0] as i64) < 22i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x8));
                                    if t {
                                        break 'b_17a7fc;
                                    } else {
                                        c.cycles += 4;
                                        c.g[S0][0] = c.lw(c.g[S1][0] as u32);
                                        c.g[V0][0] = ((c.g[S0][0] as i64) < 256i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] = c.lw(c.g[S5][0] as u32);
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0017a718);
                                            }
                                            continue 'l_17a718;
                                        } else {
                                            break 'b_17a7fc;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
                        break 'b_17a800;
                    }
                }
                c.cycles += 11;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;
    use crate::difftest::check;
    use crate::game::ai::control::handplantcontrol::tests::oc425_install;
    pub(crate) use crate::game::main::loadscreens::tests::oc736_install_fakes;

    use crate::difftest::compare;
    use crate::difftest::elf_image;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::machine;

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
    use crate::difftest::RA as TEST_RA;

    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;

    pub(crate) use crate::game::bx::bxstringctor::tests::base_3;
    pub(crate) use crate::game::bx::bxstringctor::tests::slot_2;
    pub(crate) use crate::game::render::particle::tests::f;

    pub(crate) use crate::game::camera::camera::mid_left_chase_camera_reset_for_rider;
    pub(crate) use crate::game::camera::camera::tests::algo_setup;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::heap_block;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_5;

    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool;

    pub(crate) use crate::game::fe::feasyncfile::tests::w32;

    pub(crate) use crate::game::fe::feasyncfile::tests::world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4_2 as OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_8;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all_2;

    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::main::loadscreens_prestart::c_pre_fe_load_screen_render;
    pub(crate) use crate::game::main::loadscreens_prestart::c_pre_game_load_screen_render;
    pub(crate) use crate::game::main::loadscreens_prestart::init_load_screen_render;
    pub(crate) use crate::game::main::loadscreens_prestart::pre_start_screen_render;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base_14 as base_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::check3;

    pub(crate) use crate::game::ui::uivector::tests::start;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::w;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_2;

    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn camera_trigger_read_action_type3_matches() {
        check(
            ssx3_ee::s02::f_00171680,
            camera_trigger_read_action_type3,
            64,
            &|c, seed| {
                let mut r = Rng(0x1716 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = (DATA + 0x200) as u64;
            },
        );
    }

    #[test]
    fn vec3_max_component_matches() {
        check(
            ssx3_ee::s02::f_001731c0,
            vec3_max_component,
            300,
            &|c, seed| {
                let mut r = Rng(0x1731 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for k in 0..3 {
                    c.bus.write32(DATA + k * 4, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn debug_menu_init_background_list_matches() {
        check(
            ssx3_ee::s02::f_0017abc8,
            debug_menu_init_background_list,
            64,
            &|c, seed| {
                let mut r = Rng(0x17ab ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn debug_menu_select_background_matches() {
        check(
            ssx3_ee::s02::f_0017ac98,
            debug_menu_select_background,
            300,
            &|c, seed| {
                let mut r = Rng(0x17ac ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 3 {
                    0 => 0xffff_ffff_ffff_ffff,
                    _ => (r.next_u32() % 30) as u64,
                };
                c.bus.write32(DATA + 4, r.next_u32() % 30);
            },
        );
    }

    #[test]
    fn algorithmic_script_camera_algo_dtor_matches() {
        check(
            ee::s02::f_001763f8,
            algorithmic_script_camera_algo_dtor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x63f8, 0);
                alloc_setup(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
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
                c.bus
                    .write32(DATA + 0x18, (1.0f32 + (seed % 4) as f32).to_bits());
                c.bus
                    .write32(DATA + 0x1c, (1.0f32 + (seed % 3) as f32).to_bits());
                c.bus
                    .write32(DATA + 0x20, (1.0f32 + (seed % 5) as f32).to_bits());
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
        let member = if seed % 2 == 0 {
            0
        } else {
            heap_block(c, 0x40)
        };
        c.bus.write32(obj + 8, member);
    }

    #[test]
    fn camera_trigger_array_45c1d0_v01_matches() {
        check(
            ssx3_ee::s02::f_00176840__generated,
            camera_trigger_array_45c1d0_v01,
            80,
            &|c, seed| trigger_array_setup(c, seed, 0x15),
        );
    }

    #[test]
    fn camera_trigger_array_45c1b8_v01_matches() {
        check(
            ssx3_ee::s02::f_00176868__generated,
            camera_trigger_array_45c1b8_v01,
            80,
            &|c, seed| trigger_array_setup(c, seed, 0x16),
        );
    }

    #[test]
    fn default_2_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s02::f_00176a68__generated,
            default_2_chase_camera_algorithm_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x176a_68);
            },
        );
    }

    #[test]
    fn default_3_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s02::f_00176d68__generated,
            default_3_chase_camera_algorithm_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x176d_68);
            },
        );
    }

    #[test]
    fn default_4_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s02::f_00177068__generated,
            default_4_chase_camera_algorithm_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x1770_68);
            },
        );
    }

    #[test]
    fn spoke_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s02::f_00177e50__generated,
            spoke_chase_camera_algorithm_ctor,
            32,
            &|c, seed| {
                algo_setup(c, seed, 0x177e_50);
            },
        );
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
        check(
            ee::s02::f_00179fc8__generated,
            debug_menu_is_track_selectable,
            SEEDS_3,
            &|c, seed| {
                let mut r = common(c, seed, 0x179fc8, 3);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    (r.next_u32() % 24) as u64
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    /// One shared test body for the `GetTuning` writers.
    pub(crate) fn check_tuning(hand: GuestFn, generated: GuestFn, base: i32) {
        check(generated, hand, 100, &|c, seed| {
            let mut r = start(c, seed, base as u64);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..4 {
                c.bus
                    .write32((GAME_GP as i32 + base + 4 * i) as u32, r.edge_f32());
            }
            for i in 0..5 {
                c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn default_2_chase_camera_algorithm_get_tuning_matches() {
        check_tuning(
            default_2_chase_camera_algorithm_get_tuning,
            ssx3_ee::s02::f_00176ae0__generated,
            -0x61e8,
        );
    }

    #[test]
    fn default_3_chase_camera_algorithm_get_tuning_matches() {
        check_tuning(
            default_3_chase_camera_algorithm_get_tuning,
            ssx3_ee::s02::f_00176de0__generated,
            -0x6130,
        );
    }

    #[test]
    fn default_4_chase_camera_algorithm_get_tuning_matches() {
        check_tuning(
            default_4_chase_camera_algorithm_get_tuning,
            ssx3_ee::s02::f_001770e0__generated,
            -0x6074,
        );
    }

    #[test]
    fn direction_ne_chase_camera_algorithm_get_tuning_matches() {
        check_tuning(
            direction_ne_chase_camera_algorithm_get_tuning,
            ssx3_ee::s02::f_00177558__generated,
            -0x5f48,
        );
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
        check_eye_drop(
            default_2_chase_camera_algorithm_get_eye_drop,
            ssx3_ee::s02::f_00176ad0__generated,
            -0x61ec,
        );
    }

    #[test]
    fn default_3_chase_camera_algorithm_get_eye_drop_matches() {
        check_eye_drop(
            default_3_chase_camera_algorithm_get_eye_drop,
            ssx3_ee::s02::f_00176dd0__generated,
            -0x6134,
        );
    }

    #[test]
    fn default_4_chase_camera_algorithm_get_eye_drop_matches() {
        check_eye_drop(
            default_4_chase_camera_algorithm_get_eye_drop,
            ssx3_ee::s02::f_001770d0__generated,
            -0x6078,
        );
    }

    #[test]
    fn spoke_chase_camera_algorithm_get_eye_drop_matches() {
        check_eye_drop(
            spoke_chase_camera_algorithm_get_eye_drop,
            ssx3_ee::s02::f_00177eb8__generated,
            -0x5e20,
        );
    }

    // ------------------------------------------------------- camera objects

    #[test]
    fn script_camera_algo_get_look_at_matches() {
        check(
            ssx3_ee::s02::f_00176358__generated,
            script_camera_algo_get_look_at,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x20);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                for i in 0..8 {
                    c.bus.write32(DATA + 0x120 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn camera_point_bound_obj_clamp_point_matches() {
        check(
            ssx3_ee::s02::f_00176808__generated,
            camera_point_bound_obj_clamp_point,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x10);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.edge_f32());
                }
                for i in 0..4 {
                    c.bus.write32(DATA + 0x3fc + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// A `gp`-relative global word.
    pub(crate) fn gpf(c: &mut Cpu, off: i32, v: u32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), v);
    }

    // ------------------------------------------------------------------
    // Chase cameras.

    #[test]
    fn spoke_chase_camera_get_tuning_matches() {
        check(
            ee::s02::f_00177ec8__generated,
            spoke_chase_camera_get_tuning,
            200,
            &|c, seed| {
                setup(c, seed, 1);
                c.g[A1][0] = AUX as u64;
                gpf(c, -0x5e1c, 0x3f80_0000u32.wrapping_add(seed));
                gpf(c, -0x5e18, 0x4000_0000u32.wrapping_add(seed));
                gpf(c, -0x5e14, 0x4040_0000u32.wrapping_add(seed));
                gpf(c, -0x5e10, 0x4080_0000u32.wrapping_add(seed));
            },
        );
    }

    #[test]
    fn superpipe_chase_camera_get_eye_drop_matches() {
        check(
            ee::s02::f_001785b0__generated,
            superpipe_chase_camera_get_eye_drop,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 2);
                gpf(c, -0x5dc0, r.f32(50.0));
                c.f[0] = r.f32(5.0);
            },
        );
    }

    #[test]
    fn superpipe_chase_camera_get_tuning_matches() {
        check(
            ee::s02::f_001785c0__generated,
            superpipe_chase_camera_get_tuning,
            200,
            &|c, seed| {
                setup(c, seed, 3);
                c.g[A1][0] = AUX as u64;
                gpf(c, -0x5dbc, 0x3f80_0000u32.wrapping_add(seed));
                gpf(c, -0x5db8, 0x4000_0000u32.wrapping_add(seed));
                gpf(c, -0x5db4, 0x4040_0000u32.wrapping_add(seed));
                gpf(c, -0x5db0, 0x4080_0000u32.wrapping_add(seed));
            },
        );
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
            let sp = c.g[ssx3_ee_rt::cpu::reg::SP][0];
            for _ in 0..ticks {
                c.g[A0][0] = sx(DATA);
                c.g[ssx3_ee_rt::cpu::reg::SP][0] = sp;
                c.g[RA][0] = sx(TEST_RA);
                post_race_1_chase_camera_update(c);
                assert_eq!(c.pc, TEST_RA, "returned");
            }
            let rd = |c: &mut Cpu, a: u32| f32::from_bits(c.bus.read32(a));
            let rel: Vec<f32> = (0..3)
                .map(|i| rd(c, DATA + 0x40 + 4 * i) - rd(c, DATA + 0x20 + 4 * i))
                .collect();
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
                    assert!(
                        (eye[i] - eye60[i]).abs() < 0.05,
                        "eye at {hz} from {start}: {eye:?}, at 60: {eye60:?}"
                    );
                }
            }
        }
        // The wrap: one orbit is 1100 frames at every rate.
        assert_eq!(run(60, 1, 1099).0, 0);
        assert_eq!(run(120, 1, 2199).0, 0);
        assert_eq!(run(120, 1, 1099).0, 1100);
        assert_eq!(run(240, 1, 4399).0, 0);
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
        check(
            ssx3_ee::s02::f_0017a0b8__generated,
            debug_menu_prev_enabled_item,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x0017_a0b8);
                let menu = DATA + 0x100;
                let n = 1 + (seed % 6);
                clear_2(c, menu, 0x130);
                w(c, menu, n);
                w(
                    c,
                    menu + 0x8808,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.g[A0][0] = sx(menu);
                c.g[A1][0] = (seed % n) as u64;
                // The first candidate: sometimes enabled (returns at once), else
                // leave the wrap-around scan one enabled entry so it terminates.
                w(c, menu + 0x118, if seed % 2 == 0 { 1 } else { 0 });
                let hit = (seed % n) as u32;
                for i in 0..n {
                    w(c, menu + 0x8c + 4 * i, (i == hit) as u32);
                }
            },
        );
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
                    w(
                        c,
                        OBJ2 + 4 * i,
                        if seed % 3 == 0 && i == 0 {
                            -1i32 as u32
                        } else {
                            1
                        },
                    );
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
        check(
            ee::s02::f_001714e8__generated,
            camera_trigger_read_action_type1,
            30,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x1714, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                for i in 0..0x30u32 {
                    w(c, DATA + 0x800 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn camera_sphere_trigger_get_bounds_matches() {
        check(
            ee::s02::f_001721c0__generated,
            camera_sphere_trigger_get_bounds,
            SEEDS_7,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x1721, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                for i in 0..8u32 {
                    w(c, DATA + 4 * i, r.f32(100.0));
                }
                w(c, DATA + 0x18, r.f32(100.0));
                w(c, DATA + 0x1c, r.f32(100.0));
                w(c, DATA + 0x20, r.f32(100.0));
            },
        );
    }

    #[test]
    fn camera_box_trigger_get_bounds_matches() {
        check(
            ee::s02::f_00172840__generated,
            camera_box_trigger_get_bounds,
            SEEDS_7,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x1728, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                for i in 0..8u32 {
                    w(c, DATA + 4 * i, r.f32(100.0));
                }
                w(c, DATA + 0x18, r.f32(100.0));
                w(c, DATA + 0x1c, r.f32(100.0));
                w(c, DATA + 0x20, r.f32(100.0));
            },
        );
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

    /// The spoke camera (`this`), its target (`+0x30`), the target's rider
    /// (`+4`) and that rider's owner. Fixed addresses and vtable deltas keep
    /// each path's cost (fake calls included) the same over seeds.
    const SPOKE: u32 = DATA + 0x4_0000;

    const SPOKE_TARGET: u32 = SPOKE + 0x400;

    const SPOKE_RIDER: u32 = SPOKE + 0x1000;

    const SPOKE_OWNER: u32 = SPOKE + 0x2000;

    /// `dir`, the vector the handplant virtual returns, and `out`.
    const SPOKE_DIR: u32 = SPOKE + 0x700;

    const SPOKE_HAND: u32 = SPOKE + 0x710;

    const SPOKE_OUT: u32 = SPOKE + 0x720;

    /// Where the target's `+0x20` virtual writes its vector (`sp + 0x10`).
    const SPOKE_FALLBACK: u32 = crate::difftest::SP - 0xb0;

    /// A vector of four-lane length `len`.
    fn spoke_vec(r: &mut Rng, len: f64) -> [f64; 4] {
        let v = [0; 4].map(|_| f32::from_bits(r.f32(1.0)) as f64 + 0.01);
        let n = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        v.map(|x| x * len / n)
    }

    /// Writes `v` at `at` as four f32s.
    fn spoke_put(c: &mut Cpu, at: u32, v: [f64; 4]) {
        put_vec(c, at, v.map(|x| (x as f32).to_bits()));
    }

    /// Real `Rider_GetMainState` and `SpokeChaseCameraAlgorithm_GetFocusPoint`;
    /// the target's and the rider's virtual methods are fakes. The camera's
    /// up vector (0x4ff160) is set; every pool constant is the game's.
    fn spoke_eye_objects(c: &mut Cpu, main_state: u32) {
        let w16 = |c: &mut Cpu, at: u32, v: u16| c.bus.write16(at, v);
        c.bus.write32(SPOKE + 0x30, SPOKE_TARGET);
        let vt = SPOKE + 0x500;
        c.bus.write32(SPOKE_TARGET, vt);
        c.bus.write32(SPOKE_TARGET + 4, SPOKE_RIDER);
        for (i, (slot, delta)) in [(8u32, 0x18u16), (0x20, 0xffc0), (0x50, 0x30)]
            .into_iter()
            .enumerate()
        {
            w16(c, vt + slot, delta);
            c.bus.write32(vt + slot + 4, FAKE + 0x10 * i as u32);
        }
        let vt_rider = SPOKE + 0x600;
        c.bus.write32(SPOKE_RIDER + 0x6c0, vt_rider);
        w16(c, vt_rider + 0x28, 8);
        c.bus.write32(vt_rider + 0x2c, FAKE + 0x30);
        c.bus.write32(SPOKE_RIDER + 0x77c, SPOKE_OWNER);
        c.bus.write32(SPOKE_OWNER + 0xde0, main_state);
        put_vec(c, 0x004f_f160, [0, 1.0f32.to_bits(), 0, 0]);
        for (neg, bits) in [
            (0x5e0c, 0x3e4c_cccc),
            (0x5e08, 0x3f4c_cccd),
            (0x5e04, 0x3c23_d70a),
            (0x5e00, 0x3f75_c28f),
            (0x5dfc, 0x3d23_d710),
            (0x5df8, 0x3ca3_d70a),
        ] {
            c.bus.write32(GAME_GP - neg, bits);
        }
        c.g[A0][0] = SPOKE_OUT as u64;
        c.g[A1][0] = SPOKE as u64;
        c.g[A2][0] = SPOKE_DIR as u64;
    }

    /// Seed bits 0 to 2: handplant (main state 5), a direction shorter than
    /// the 0.01 floor (the `+0x20` fallback), and a blend shorter than 0.02.
    /// Every pool slot this function and its focus point read takes one of
    /// three values.
    fn spoke_eye_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x177fc0, 4);
        let (hand, short, degenerate) = (seed & 1 != 0, seed & 2 != 0, seed & 4 != 0);
        let states = [0, 1, 2, 3, 4, 6];
        let state = if hand {
            5
        } else {
            states[(r.next_u32() % 6) as usize]
        };
        spoke_eye_objects(c, state);
        let pick = |r: &mut Rng, v: [f32; 3]| v[(r.next_u32() % 3) as usize];
        let floor = pick(&mut r, [0.01, 0.005, 0.03]);
        let keep = pick(&mut r, [0.96, 0.9, 0.98]);
        let add = pick(&mut r, [0.04, 0.1, 0.02]);
        for (neg, v) in [
            (0x5e0c, pick(&mut r, [0.2, 0.3, 0.6])),
            (0x5e08, pick(&mut r, [0.8, 0.7, 0.4])),
            (0x5e04, floor),
            (0x5e00, keep),
            (0x5dfc, add),
            (0x5df8, pick(&mut r, [0.02, 0.015, 0.05])),
        ] {
            c.bus.write32(GAME_GP - neg, v.to_bits());
        }
        let up = spoke_vec(&mut r, 1.0);
        spoke_put(c, 0x004f_f160, up);
        // Long directions are 0.5 to 50; short ones under 0.004, below
        // every floor.
        let len = if short {
            0.004 * (0.05 + 0.9 * (r.next_u32() % 1000) as f64 / 1000.0)
        } else {
            0.5 + (r.next_u32() % 1000) as f64 / 20.0
        };
        let d = spoke_vec(&mut r, len);
        let other = spoke_vec(&mut r, 3.0);
        let (at_dir, at_hand) = if hand { (other, d) } else { (d, other) };
        spoke_put(c, SPOKE_DIR, at_dir);
        spoke_put(c, SPOKE_HAND, at_hand);
        if hand {
            c.bus.write32(LOG_RESULTS, SPOKE_HAND);
        }
        let fallback = spoke_vec(&mut r, if degenerate { 0.003 } else { 1.5 });
        spoke_put(c, SPOKE_FALLBACK, fallback);
        // A blend shorter than every 0.02 variant: the heading cancels the
        // normalized direction (or both are tiny); otherwise a unit heading.
        let heading = if !degenerate {
            spoke_vec(&mut r, 1.0)
        } else if short {
            spoke_vec(&mut r, 0.003)
        } else {
            let (k, b) = (keep as f64, add as f64);
            let noise = spoke_vec(&mut r, 0.002);
            [0, 1, 2, 3].map(|i| -b / k * d[i] / len + noise[i])
        };
        spoke_put(c, SPOKE + 0x390, heading);
    }

    /// Three distance branches crossed with state 5 / other, retaining real
    /// GatherRiderState, GetMainState, ComputeDesiredEye and GetFocusPoint.
    /// Only the target and rider virtual methods use the logging fakes.
    fn spoke_update_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x178208, 10);
        elf_image(c);
        c.bus.write32(crate::tick::APPMAN_PTR, 0);
        spoke_eye_objects(c, if seed % 2 == 0 { 0 } else { 5 });
        c.g[A0][0] = SPOKE as u64;
        c.pc = 0x0017_8208;
        let vt = SPOKE + 0x500;
        for (i, slot) in [0x18u32, 0xa0, 0x88, 0x90].into_iter().enumerate() {
            c.bus.write16(vt + slot, i as u16 * 0x10);
            c.bus.write32(vt + slot + 4, FAKE + 0x40 + i as u32 * 0x10);
        }
        c.bus.write32(LOG_RESULTS + 4, 4);
        c.bus.write32(LOG_RESULTS + 5 * 4, SPOKE_HAND);
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed < 6 || seed % 3 != 0 {
                u32::MAX
            } else {
                (seed / 3) % 10
            },
        );
        for neg in (0x5dc8..=0x5e0c).step_by(4).chain([0x6af8, 0x6afc]) {
            let at = GAME_GP - neg;
            let v = f32::from_bits(c.bus.read32(at));
            let scale = [0.8f32, 1.0, 1.2][(r.next_u32() % 3) as usize];
            c.bus.write32(at, (v * scale).to_bits());
        }
        let vector = |x: f32, y: f32, z: f32| [x.to_bits(), y.to_bits(), z.to_bits(), 0];
        let velocity = vector(60.0, 0.0, 0.0);
        put_vec(c, crate::difftest::SP - 0x200, velocity);
        put_vec(c, SPOKE + 0x1b0, velocity);
        put_vec(c, SPOKE + 0x100, velocity);
        put_vec(c, SPOKE_HAND, velocity);
        put_vec(c, SPOKE + 0x390, vector(1.0, 0.0, 0.0));
        put_vec(c, SPOKE + 0x20, [0; 4]);
        let distance = [100.0, 500.0, 2000.0][(seed / 2 % 3) as usize];
        put_vec(c, SPOKE + 0x40, vector(distance, 0.0, 0.0));
        put_vec(c, 0x004f_f160, vector(0.0, 0.0, 1.0));
        put_vec(c, 0x004f_f120, vector(0.0, 0.0, 1.0));
    }

    /// Stable virtual outputs for the rate test; the real gather, main-state,
    /// desired-eye and focus-point callees execute without replacing any slot.
    fn spoke_rate_target(c: &mut Cpu, target: u32) {
        let out = c.g[A0][0] as u32;
        match target {
            FAKE => put_vec(c, out, [0; 4]),
            x if x == FAKE + 0x10 || x == FAKE + 0x40 => {
                put_vec(c, out, [60.0f32.to_bits(), 0, 0, 0]);
            }
            x if x == FAKE + 0x20 => c.g[V0][0] = SPOKE_HAND as u64,
            x if x == FAKE + 0x30 => c.g[V0][0] = SPOKE_OUT as u64,
            x if x == FAKE + 0x50 => c.g[V0][0] = 4,
            x if x == FAKE + 0x60 || x == FAKE + 0x70 => c.f[0] = 0,
            _ => panic!("unexpected spoke target {target:#x}"),
        }
        c.pc = c.g[RA][0] as u32;
    }

    #[test]
    fn spoke_chase_camera_eye_follows_the_rate() {
        for state in [0u32, 5] {
            // The distance-dependent coefficient is recomputed each tick.
            // Crossing the ramp remains approximate (1 unit); a start in the
            // ramp is closer (0.3); beyond the far limit every rate snaps.
            for (start, tolerance) in [(-80.0f32, 1.0f32), (-200.0, 0.3), (-2000.0, 0.001)] {
                let sample = |hz: u32| {
                    let mut c = crate::difftest::fresh();
                    spoke_update_setup(&mut c, u32::from(state == 5));
                    elf_image(&mut c);
                    crate::tick::set_for_test(&mut c, hz);
                    for (at, bits) in crate::tick::per_tick_values(hz) {
                        c.bus.write32(at, bits);
                    }
                    c.hooks.miss = spoke_rate_target;
                    put_vec(&mut c, SPOKE_OUT, [0; 4]);
                    put_vec(&mut c, SPOKE + 0x20, [0, 0, 37.0f32.to_bits(), 0]);
                    put_vec(
                        &mut c,
                        SPOKE + 0x40,
                        [start.to_bits(), 0, 30.0f32.to_bits(), 0],
                    );
                    let mut out = Vec::new();
                    for n in 1..=hz {
                        c.g[A0][0] = SPOKE as u64;
                        c.g[RA][0] = crate::difftest::RA as u64;
                        c.g[ssx3_ee_rt::cpu::reg::SP][0] = crate::difftest::SP as u64;
                        spoke_chase_camera_algorithm_update(&mut c);
                        assert_eq!(c.pc, crate::difftest::RA);
                        assert_eq!(c.g[ssx3_ee_rt::cpu::reg::SP][0], crate::difftest::SP as u64);
                        assert_eq!(c.bus.read32(SPOKE_OWNER + 0xde0), state);
                        let heading = f32::from_bits(c.bus.read32(SPOKE + 0x390));
                        assert!((heading - 1.0).abs() < 0.00001);
                        if n % (hz / 4) == 0 {
                            let eye: [f32; 3] = std::array::from_fn(|i| {
                                f32::from_bits(c.bus.read32(SPOKE + 0x40 + i as u32 * 4))
                            });
                            out.push(eye);
                        }
                    }
                    out
                };
                let stock = sample(60);
                for hz in [120, 240] {
                    let actual = sample(hz);
                    for (n, (a, b)) in actual.iter().zip(&stock).enumerate() {
                        for axis in 0..3 {
                            assert!((a[axis] - b[axis]).abs() < tolerance,
                                "state {state}, start {start}, {hz} Hz checkpoint {n}: {a:?} vs {b:?}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn spoke_chase_camera_algorithm_update_matches() {
        covered(
            ee::s02::f_00178208__generated,
            spoke_chase_camera_algorithm_update,
            240,
            &spoke_update_setup,
        );
        // Exact thresholds and their adjacent representable distances, crossed
        // with ordinary/state-5 behavior. The far threshold is the loaded,
        // seed-varied pool value, not an assumed stock constant.
        crate::difftest::check(
            ee::s02::f_00178208__generated,
            spoke_chase_camera_algorithm_update,
            12,
            &|c, seed| {
                spoke_update_setup(c, seed);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                let boundary = if seed / 6 == 0 {
                    150.0f32.to_bits()
                } else {
                    c.bus.read32(GAME_GP - 0x5dd0)
                };
                let distance = match seed / 2 % 3 {
                    0 => boundary - 1,
                    1 => boundary,
                    _ => boundary + 1,
                };
                put_vec(c, SPOKE + 0x40, [distance, 0, 0, 0]);
            },
        );
        // Three distance branches crossed with normal/handplant state. Some
        // paths share a cost because ComputeDesiredEye also branches.
        crate::difftest::require_paths(
            ee::s02::f_00178208__generated,
            240,
            &[629, 639, 637, 650],
            &spoke_update_setup,
        );
        for seed in 0..6 {
            let mut c = crate::difftest::fresh();
            spoke_update_setup(&mut c, seed);
            let distance = f32::from_bits(c.bus.read32(SPOKE + 0x40));
            let far = f32::from_bits(c.bus.read32(GAME_GP - 0x5dd0));
            let branch = if distance < 150.0 {
                0
            } else if distance <= far {
                1
            } else {
                2
            };
            assert_eq!(branch, seed / 2, "seed {seed}");
            assert_eq!(
                c.bus.read32(SPOKE_OWNER + 0xde0),
                if seed % 2 == 0 { 0 } else { 5 }
            );
            crate::difftest::run_generated(&mut c, ee::s02::f_00178208__generated);
            assert_eq!(c.pc, TEST_RA);
            let calls = c.bus.read32(fakes::LOG_COUNT);
            assert_eq!(calls, 9 + seed % 2, "seed {seed}");
            let hand =
                (0..calls).any(|n| c.bus.read32(fakes::LOG_START + 0x220 * n) == FAKE + 0x20);
            assert_eq!(hand, seed % 2 != 0, "seed {seed}");
        }
    }

    #[test]
    fn spoke_chase_camera_algorithm_compute_desired_eye_matches() {
        covered(
            ee::s02::f_00177fc0__generated,
            spoke_chase_camera_algorithm_compute_desired_eye,
            192,
            &spoke_eye_setup,
        );
        // Seeds 0 to 7 take the eight branch combinations without an unwind;
        // seeds 0 and 4, which differ only in the blend floor, share a cost.
        const PATHS: &[u64] = &[186, 208, 200, 218, 207, 196, 209];
        crate::difftest::require_paths(
            ee::s02::f_00177fc0__generated,
            192,
            PATHS,
            &spoke_eye_setup,
        );
        // So the branches are also read from the generated body's outputs: the
        // fakes called (the target's +0x50 and +0x20 virtuals) and whether the
        // heading is the direction itself (the frame's `sp + 0`).
        let mut seen = [false; 8];
        for seed in 0..192 {
            let mut c = crate::difftest::fresh();
            spoke_eye_setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, ee::s02::f_00177fc0__generated);
            if c.pc != TEST_RA {
                continue;
            }
            let calls = c.bus.read32(fakes::LOG_COUNT);
            let called = |c: &mut Cpu, f: u32| {
                (0..calls).any(|n| c.bus.read32(fakes::LOG_START + 0x220 * n) == f)
            };
            let hand = called(&mut c, FAKE + 0x20);
            let short = called(&mut c, FAKE + 0x10);
            let dir = c.bus.read128(crate::difftest::SP - 0xc0);
            let degenerate = c.bus.read128(SPOKE + 0x390) == dir;
            let taken = hand as usize | (short as usize) << 1 | (degenerate as usize) << 2;
            assert_eq!(taken, (seed & 7) as usize, "seed {seed}");
            seen[taken] = true;
        }
        assert!(seen.iter().all(|&s| s), "{seen:?}");
    }

    /// The heading turns toward the direction the same way per game time at
    /// any rate. The per-update normalize makes it approximate: within 0.5
    /// degrees over the first second from 10, 45 and 90 degrees off.
    #[test]
    fn spoke_chase_camera_heading_follows_the_rate() {
        const KEEP: u32 = 0x0049_d2f0;
        const ADD: u32 = 0x0049_d2f4;
        /// The executable's shares: 0.96 kept, 0.04 added.
        const KEEP_STOCK: u32 = 0x3f75_c28f;
        const ADD_STOCK: u32 = 0x3d23_d710;
        for start in [10.0f64, 45.0, 90.0] {
            let angles = [60u32, 120, 240].map(|hz| {
                // An entry the rewrite leaves alone keeps its executable bits.
                let add = crate::tick::per_tick_values(hz)
                    .into_iter()
                    .find(|e| e.0 == ADD)
                    .map_or(ADD_STOCK, |e| e.1);
                let mut c = crate::difftest::fresh();
                let _ = machine(&mut c, 0, 0x177fc0, 0);
                spoke_eye_objects(&mut c, 0);
                c.bus
                    .write32(KEEP, crate::tick::per_tick_at(hz, KEEP, KEEP_STOCK));
                c.bus.write32(ADD, add);
                put_vec(&mut c, SPOKE_DIR, [1.0f32.to_bits(), 0, 0, 0]);
                let (s, co) = start.to_radians().sin_cos();
                put_vec(
                    &mut c,
                    SPOKE + 0x390,
                    [(co as f32).to_bits(), (s as f32).to_bits(), 0, 0],
                );
                let mut out = Vec::new();
                for n in 1..=hz {
                    c.g[A0][0] = SPOKE_OUT as u64;
                    c.g[A1][0] = SPOKE as u64;
                    c.g[A2][0] = SPOKE_DIR as u64;
                    spoke_chase_camera_algorithm_compute_desired_eye(&mut c);
                    assert_eq!(c.pc, TEST_RA);
                    if n % (hz / 4) == 0 {
                        let x = f32::from_bits(c.bus.read32(SPOKE + 0x390)) as f64;
                        let y = f32::from_bits(c.bus.read32(SPOKE + 0x394)) as f64;
                        out.push(y.atan2(x).to_degrees());
                    }
                }
                out
            });
            for (hz, got) in [(120, &angles[1]), (240, &angles[2])] {
                for (i, (a, b)) in got.iter().zip(&angles[0]).enumerate() {
                    assert!(
                        (a - b).abs() < 0.5,
                        "from {start} deg, {hz} Hz at {} s: {a} vs {b} at 60",
                        0.25 * (i + 1) as f64
                    );
                }
            }
        }
        // At 60 both entries keep the executable's bits.
        let at60 = crate::tick::per_tick_values(60);
        assert!(at60.contains(&(KEEP, KEEP_STOCK)), "keep share");
        assert!(at60.contains(&(ADD, ADD_STOCK)), "added share");
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
        let r = machine(c, seed, salt, calls);
        globals_3(c);
        pool_4(c);
        r
    }

    #[test]
    fn chase_camera_algorithm_ctor_matches() {
        check3(
            ee::s01::f_00162318__generated,
            chase_camera_algorithm_ctor,
            60,
            &|c, seed| {
                let _ = base_7(c, seed, 0x162318);
                c.g[A0][0] = sx(DATA);
            },
        );
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
        w_2(
            c,
            VT2 + 0x3c,
            if seed % 2 == 0 {
                ACTIVE_FIND
            } else {
                fakes::FAKE
            },
        );
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
        assert!(
            BASE > fakes::LOG + 0x1_0000,
            "fixture overlaps the fake call log"
        );
        for seed in 0..200 {
            let report = compare(
                ee::s02::f_00174848__generated,
                camera_trigger_man_collect_octree_triggers_in_box,
                &|c| setup_2(c, seed),
            );
            assert!(report.hard.is_empty(), "seed {seed}: {:#?}", report.hard);
            assert!(
                report.soft.is_empty(),
                "seed {seed} (soft): {:#?}",
                report.soft
            );
            assert!(
                report.lohi.is_empty(),
                "seed {seed} (LO/HI): {:#?}",
                report.lohi
            );
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

    // --- ChaseCameraAlgorithm_GatherRiderState ----------------------------

    pub(crate) const CAM: u32 = DATA;

    pub(crate) const CAM_OUT: u32 = DATA + 0x1000;

    pub(crate) const CAM_TARGET: u32 = DATA + 0x2000;

    pub(crate) const CAM_TARGET_VT: u32 = DATA + 0x2100;

    pub(crate) const CAM_RIDER: u32 = DATA + 0x3000;

    pub(crate) const CAM_RIDER_STATE: u32 = DATA + 0x4000;

    /// A chase camera whose target (a fake vtable: velocity, state,
    /// parameters, frame) is a rider in one of the modes; velocities around
    /// the 27.8 m/s threshold, thresholds that put |dir.z| below, between
    /// and above them.
    pub(crate) fn gather_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::SP;
        let mut r = machine(c, seed, 0x162568, 4);
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        let vec = |c: &mut Cpu, r: &mut Rng, a: u32, len: f32| {
            for i in 0..4 {
                c.bus.write32(a + 4 * i, r.f32(len));
            }
        };
        w(c, CAM + 0x30, CAM_TARGET);
        w(c, CAM_TARGET, CAM_TARGET_VT);
        for (i, slot) in [0x18u32, 0xa0, 0x88, 0x90, 0x8].into_iter().enumerate() {
            c.bus.write16(CAM_TARGET_VT + slot, (i as u16) * 0x10);
            w(c, CAM_TARGET_VT + slot + 4, FAKE + 0x10 * i as u32);
        }
        // The velocity the first call "returns" in its result slot (sp+0).
        let len = if seed % 3 == 0 {
            0.0
        } else if seed % 2 == 0 {
            20.0
        } else {
            60.0
        };
        vec(c, &mut r, SP - 0x110, len);
        vec(c, &mut r, CAM + 0x1b0, 40.0);
        vec(c, &mut r, CAM + 0x100, 40.0);
        vec(c, &mut r, CAM + 0x40, 40.0);
        crate::difftest::put_words(c, 0x004f_f120, &[0, 0, 0x3f80_0000, 0]);
        w(c, GAME_GP - 0x6afc, 27.777_779_f32.to_bits());
        w(c, GAME_GP - 0x6af8, 27.777_779_f32.to_bits());
        // Rider mode (0x11fe98) and target state (fake call 1).
        w(c, CAM_TARGET + 4, CAM_RIDER);
        w(c, CAM_RIDER + 0x77c, CAM_RIDER_STATE);
        w(
            c,
            CAM_RIDER_STATE + 0xde0,
            [1, 2, 4, 5, 0, 1][(seed / 2 % 6) as usize],
        );
        w(
            c,
            LOG_RESULTS + 4,
            [2, 3, 13, 5, 1][(seed / 5 % 5) as usize],
        );
        // a..h: lo/hi pairs and |z| thresholds.
        for (i, (base, span)) in [
            (-5.0f32, 10.0f32),
            (-5.0, 10.0),
            (0.0, 0.6),
            (0.5, 0.6),
            (-5.0, 10.0),
            (-5.0, 10.0),
            (0.0, 0.6),
            (0.5, 0.6),
        ]
        .into_iter()
        .enumerate()
        {
            let u = (r.next_u32() as f32 / u32::MAX as f32) * span + base;
            c.f[12 + i] = u.to_bits();
        }
        c.g[A0][0] = CAM as u64;
        c.g[A1][0] = CAM_OUT as u64;
        c.pc = 0x0016_2568;
    }

    #[test]
    fn gather_rider_state_matches() {
        covered(
            ee::s01::f_00162568__generated,
            chase_camera_algorithm_gather_rider_state,
            300,
            &|c, seed| gather_fixture(c, seed),
        );
    }

    use ssx3_ee_rt::Cpu;

    /// Default_3ChaseCameraAlgorithm_Update: a chase camera at `DATA` with
    /// random tuning state, its target (`+0x30`) whose v-table slots are
    /// fakes, the rider behind it and the world's player count (as the
    /// post-race chase camera's test builds them); the game's `.lit4`
    /// constants from the ELF image. The real stages run on it.
    #[test]
    fn default_3_chase_camera_algorithm_update_matches() {
        use crate::difftest::fakes::vtable;
        use crate::difftest::put_vec;
        check(
            ee::s02::f_00176e10__generated,
            default_3_chase_camera_algorithm_update,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x176e10, 8);
                c.pc = 0x0017_6e10;
                let (algo, t, rider, ms) = (DATA, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                c.g[A0][0] = sx(algo);
                for i in 0..0x100 {
                    c.bus.write32(algo + 4 * i, r.f32(400.0));
                }
                put_vec(c, algo + 0x390, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                // The five tuning splines (`cCSICubicSpline` at +0x304..+0x334,
                // 12 bytes: word 0 count << 11 | current << 19, +4 end
                // time, +8 segments of 0x14 bytes with the start time at
                // +0x10): one segment spanning every float, so
                // cCSICubicSpline_Evaluate always finds it as the current
                // one (random headers sent its binary search into a loop).
                for (k, off) in [0x304u32, 0x310, 0x31c, 0x328, 0x334]
                    .into_iter()
                    .enumerate()
                {
                    let segs = DATA + 0x5000 + 0x40 * k as u32;
                    for i in 0..4 {
                        c.bus.write32(segs + 4 * i, r.f32(4.0));
                    }
                    c.bus.write32(segs + 0x10, 0xff7f_ffff);
                    c.bus.write32(segs + 0x14 + 0x10, 0x7f7f_ffff);
                    c.bus.write32(algo + off, 1 << 11);
                    c.bus.write32(algo + off + 4, 0x7f7f_ffff);
                    c.bus.write32(algo + off + 8, segs);
                }
                // +0x2d0 set: stage 0x1635f8 skips rebuilding the +0x31c
                // spline from state-dependent key times (unsorted on
                // random state).
                c.bus.write32(algo + 0x2d0, 1);
                c.bus.write32(algo + 0x2c4, 0);
                c.bus.write32(algo + 0x30, t);
                let slots: Vec<u32> = (1..0x20).map(|k| 8 * k).collect();
                let vt = vtable(c, &mut r, &slots);
                c.bus.write32(t, vt);
                c.bus.write32(t + 4, rider);
                c.bus.write32(rider + 0x77c, ms);
                c.bus.write32(ms + 0xde0, r.next_u32() % 8);
                let (w, w84, w0c) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
                c.bus.write32(w + 0x84, w84);
                c.bus.write32(w84 + 0xc, w0c);
                c.bus.write32(w0c + 0x78, r.next_u32() % 4);
                c.bus.write32(crate::tick::APPMAN_PTR, 0);
            },
        );
    }

    pub(crate) use crate::game::ai::rider::tests::w32_2;
    pub(crate) use crate::game::world::worldview::tests::oc410_machine;
    pub(crate) use crate::game::world::worldview::tests::oc410_out;
    pub(crate) use crate::game::world::worldview::tests::oc410_result;

    pub(crate) use crate::game::seg::seg_1fbe38::tests::FK_READ_U32;

    pub(crate) use crate::game::ai::rider::tests::oc417_force;
    pub(crate) use crate::game::seg::seg_1fbe38::tests::oc417_machine;

    /// `CameraTrigger_ReadAction`: the four action kinds and an unknown one.
    #[test]
    fn camera_trigger_read_action_matches() {
        check(
            ee::s02::f_001712b0__generated,
            camera_trigger_read_action_2,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0017_12b0, 3);
                c.pc = 0x0017_12b0;
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = (DATA + 0x500) as u64;
                let kind = [0, 1, 2, 3, 4, 0xffff_ffff][(seed % 6) as usize];
                w32_2(c, oc410_out(11), kind);
                w32_2(c, oc410_result(11), 1 + r.next_u32() % 100);
                w32_2(c, oc410_result(12), DATA + 0x3000);
                for id in 13..17 {
                    w32_2(c, oc410_result(id), r.next_u32() % 50);
                }
                w32_2(c, GAME_GP_REL_643C, r.edge_f32());
                w32_2(c, GAME_GP_REL_6438, r.edge_f32());
                for k in 0..3 {
                    w32_2(c, 0x004f_f0d8 + 4 * k, r.next_u32());
                }
            },
        );
    }

    pub(crate) const GAME_GP_REL_643C: u32 = crate::difftest::GAME_GP.wrapping_sub(0x643c);

    pub(crate) const GAME_GP_REL_6438: u32 = crate::difftest::GAME_GP.wrapping_sub(0x6438);

    /// `CameraTrigger_ReadBoundObj`: every type word (0..3 allocate and read
    /// an ellipse/box/line/point, others fall out), random payload.
    #[test]
    fn camera_trigger_read_bound_obj_matches() {
        check(
            ee::s02::f_00171690__generated,
            camera_trigger_read_bound_obj,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x171690, 0);
                elf_image(c);
                alloc_setup(c, &mut r);
                c.pc = 0x0017_1690;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA, DATA + 0x100);
                let ty = [0u32, 1, 2, 3, 4, 0x1_0000][(seed % 6) as usize];
                for (k, b) in ty.to_be_bytes().iter().enumerate() {
                    c.bus.write8(DATA + 0x100 + k as u32, *b);
                }
                for k in 4..0x80u32 {
                    c.bus.write8(DATA + 0x100 + k, r.next_u32() as u8);
                }
            },
        );
    }

    /// `CameraTrigger_ReadSphereTrigger`: ellipse and box volumes, and
    /// other type words (the stream is pushed back).
    #[test]
    fn camera_trigger_read_sphere_trigger_matches() {
        check(
            ee::s02::f_00172e18__generated,
            camera_trigger_read_sphere_trigger,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x172e18, 0);
                elf_image(c);
                alloc_setup(c, &mut r);
                c.pc = 0x0017_2e18;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA, DATA + 0x100);
                let ty = [0u32, 1, 2, 3, 0x1_0000][(seed % 5) as usize];
                for (k, b) in ty.to_be_bytes().iter().enumerate() {
                    c.bus.write8(DATA + 0x100 + k as u32, *b);
                }
                for k in 4..0x80u32 {
                    c.bus.write8(DATA + 0x100 + k, r.next_u32() as u8);
                }
            },
        );
    }

    // ----------------------------------------------------------------------
    // CameraTrigger_ReadSphereTriggerData

    /// The trigger object with its descriptor: five `{delta, function}`
    /// accessors (generic fakes: they return pointers) and the readers
    /// (`Serialize_ReadU32` faked with forced results; `0x2d25a0` runs for
    /// real on top of it).
    #[test]
    fn camera_trigger_read_sphere_trigger_data_matches() {
        const OBJ: u32 = DATA + 0x400;
        const DESC: u32 = DATA + 0x800;
        check(
            ee::s02::f_00172fe0__generated,
            camera_trigger_read_sphere_trigger_data,
            200,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0003, 10);
                c.pc = 0x0017_2fe0;
                oc417_force(c, FK_READ_U32, r.next_u32() % 5 + 1);
                c.bus.write32(OBJ + 0x24, DESC);
                for (k, off) in [0x8u32, 0x28, 0x10, 0x18, 0x20].into_iter().enumerate() {
                    let delta = (r.next_u32() % 8) * 4;
                    c.bus.write16(DESC + off, delta as u16);
                    c.bus.write32(DESC + off + 4, fakes::FAKE + 0x10 * k as u32);
                }
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = OBJ as u64;
                c.bus.write32(OBJ + 0x28, r.next_u32());
            },
        );
    }

    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc423_fake;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc423_fake_call;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w_5 as w_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC423_CTRL;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC423_MAGIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC423_RET;

    pub(crate) use crate::game::ai::ai::tests::oc425_machine;

    oc423_fake!(
        oc423_fake_serialize_read_u32,
        OC423_PREV_SERIALIZE_READ_U32,
        0x002d_2510,
        ee::s09::f_002d2510__generated
    );

    pub(crate) fn hook_serialize_read_u32() {
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D2510),
            oc423_fake_serialize_read_u32,
            &OC423_PREV_SERIALIZE_READ_U32,
        );
    }

    /// The owner's vector method is a plain fake; the real `Serialize_ReadVec3`
    /// reads three words through the faked `Serialize_ReadU32`.
    #[test]
    fn camera_trigger_read_point_bound_obj_matches() {
        hook_serialize_read_u32();
        fakes::covered(
            ee::s02::f_00171d10__generated,
            camera_trigger_read_point_bound_obj,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0017_1d10, 5);
                w_3(c, OC423_CTRL, OC423_MAGIC);
                for i in 1..4 {
                    w_3(c, OC423_RET + 4 * i, r.next_u32() % 0x1000);
                }
                let obj = DATA + 0x5_0000;
                let vt = DATA + 0x5_1000;
                w_3(c, obj + 0x24, vt);
                c.bus.write16(vt + 8, 0x10);
                w_3(c, vt + 0xc, FAKE);
                w_3(c, obj + 0x28, r.next_u32());
                c.g[A0][0] = (DATA + 0x5_2000) as u64;
                c.g[A1][0] = obj as u64;
            },
        );
    }

    /// `CameraTrigger_ReadBoxTriggerData`: the object's five accessor virtuals
    /// (fake methods), the vector readers (real, over faked `ReadU32`) and
    /// `ReadU32` itself are called in order (some unwinding).
    #[test]
    fn camera_trigger_read_box_trigger_data_matches() {
        oc425_install();
        covered(
            ee::s02::f_001730d0__generated,
            camera_trigger_read_box_trigger_data_2,
            120,
            &|c, seed| {
                let mut r = oc425_machine(c, seed, 0x0017_30d0, 14);
                c.pc = 0x0017_30d0;
                let reader = DATA + 0x1000;
                let obj = DATA + 0x2000;
                let vt = crate::difftest::fakes::vtable(c, &mut r, &[0x8, 0x28, 0x10, 0x18, 0x20]);
                c.bus.write32(obj + 0x24, vt);
                c.bus.write32(obj + 0x28, r.next_u32());
                c.g[A0][0] = sx(reader);
                c.g[A1][0] = sx(obj);
            },
        );
    }

    /// `CameraBoundObj_GetPosition` returns its argument.
    #[test]
    fn camera_bound_obj_get_position_matches() {
        check(
            ee::s02::f_00176440__generated,
            camera_bound_obj_get_position,
            20,
            &|c, seed| {
                let mut r = Rng(0x17_6440 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0017_6440;
            },
        );
    }

    #[test]
    fn camera_bound_obj_get_scale_matches() {
        check(
            ee::s02::f_00176460__generated,
            camera_bound_obj_get_scale,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x176460, 0);
                c.pc = 0x0017_6460;
            },
        );
    }

    #[test]
    fn camera_bound_obj_get_field14_matches() {
        check(
            ee::s02::f_00176458__generated,
            camera_bound_obj_get_field14,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x176458, 0);
                c.pc = 0x0017_6458;
            },
        );
    }

    #[test]
    fn camera_bound_obj_get_rot_matches() {
        check(
            ee::s02::f_00176448__generated,
            camera_bound_obj_get_rot_x,
            100,
            &|c, seed| {
                let mut r = Rng(0x176448 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
        check(
            ee::s02::f_00176450__generated,
            camera_bound_obj_get_rot_y,
            100,
            &|c, seed| {
                let mut r = Rng(0x176450 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    use crate::difftest::fakes::ptr;

    /// Gate for this module's callee fakes (outside the pool and the log).
    pub(crate) const OC437_GATE: u32 = DATA + 0x20_0000;

    pub(crate) const OC437_MAGIC: u32 = 0x0c43_7001;

    pub(crate) fn oc437_noop(_c: &mut Cpu) {}

    /// A gated fake for the callee behind override slot `$slot`: with the gate
    /// set it logs the call (`fakes::fake_call`, which may unwind); otherwise it
    /// runs what the slot held.
    macro_rules! oc437_fake {
        ($name:ident) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC437_GATE) != OC437_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), oc437_noop);
                }
                let target = c.pc;
                fakes::fake_call(c, target)
            }
        };
    }

    oc437_fake!(oc437_fake_a);

    oc437_fake!(oc437_fake_b);

    oc437_fake!(oc437_fake_c);

    oc437_fake!(oc437_fake_d);

    oc437_fake!(oc437_fake_e);

    oc437_fake!(oc437_fake_f);

    oc437_fake!(oc437_fake_g);

    oc437_fake!(oc437_fake_h);

    oc437_fake!(oc437_fake_i);

    oc437_fake!(oc437_fake_j);

    pub(crate) fn oc437_install_fakes() {
        use std::ptr::addr_of_mut;
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 10] = [
            (addr_of_mut!(ee::s10::OVERRIDE_00317D70), oc437_fake_a),
            (addr_of_mut!(ee::s07::OVERRIDE_00282DD0), oc437_fake_b),
            (addr_of_mut!(ee::s07::OVERRIDE_00272CC0), oc437_fake_c),
            (addr_of_mut!(ee::s07::OVERRIDE_002724E8), oc437_fake_d),
            (addr_of_mut!(ee::s01::OVERRIDE_0015CB08), oc437_fake_e),
            (addr_of_mut!(ee::s07::OVERRIDE_002776E0), oc437_fake_f),
            (addr_of_mut!(ee::s07::OVERRIDE_00277838), oc437_fake_g),
            (addr_of_mut!(ee::s13::OVERRIDE_003A9188), oc437_fake_h),
            (addr_of_mut!(ee::s13::OVERRIDE_0039E390), oc437_fake_i),
            (addr_of_mut!(ee::s10::OVERRIDE_00317E50), oc437_fake_j),
        ];
        for (slot, fake) in slots {
            fakes::hook(slot, fake, fakes::prev_cell(fake));
        }
    }

    /// Fixture start: fakes hooked and gated on, `c.pc` at the entry.
    pub(crate) fn oc437_setup(
        c: &mut Cpu,
        seed: u32,
        entry: u32,
        calls: u32,
    ) -> crate::difftest::Rng {
        oc437_install_fakes();
        let r = machine(c, seed, entry as u64, calls);
        c.pc = entry;
        c.bus.write32(OC437_GATE, OC437_MAGIC);
        r
    }

    /// Runs `check`/`covered` for a call-making function.
    pub(crate) fn oc437_covered(
        generated: fn(&mut Cpu),
        hand: fn(&mut Cpu),
        entry: u32,
        calls: u32,
        init: &dyn Fn(&mut Cpu, &mut crate::difftest::Rng, u32),
    ) {
        covered(generated, hand, 300, &|c, seed| {
            let mut r = oc437_setup(c, seed, entry, calls);
            let obj = ptr(&mut r);
            c.g[A0][0] = obj as u64;
            crate::difftest::fill(c, &mut r, obj, 0x300 / 4);
            init(c, &mut r, seed);
        });
    }

    /// `flags & 1` clear (no delete) and set (delete call, may unwind).
    pub(crate) fn oc437_dtor_flags(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        let hi = (r.next_u32() as u64) << 32;
        let low = (seed % 2) as u64;
        c.g[A1][0] = hi | low | (r.next_u32() & 0xe) as u64;
    }

    #[test]
    fn target_script_camera_algo_dtor_matches() {
        oc437_covered(
            ee::s02::f_00176398__generated,
            target_script_camera_algo_dtor,
            0x0017_6398,
            1,
            &oc437_dtor_flags,
        );
    }

    #[test]
    fn manual_script_camera_algo_dtor_matches() {
        oc437_covered(
            ee::s02::f_00176368__generated,
            manual_script_camera_algo_dtor,
            0x0017_6368,
            1,
            &oc437_dtor_flags,
        );
    }

    use crate::difftest::fakes::fake;

    /// The box the function reads (position, rot[0..3], size).
    pub(crate) const CVB_TEST_BOX: u32 = DATA;

    /// The `Vec4` point.
    pub(crate) const CVB_TEST_PT: u32 = DATA + 0x100;

    #[test]
    fn camera_box_trigger_contains_point_matches() {
        check(
            ee::s02::f_001728f8__generated,
            camera_box_trigger_contains_point,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = Rng(0x0017_28f8 ^ (seed as u64) << 20 ^ 0x9e37_79b9);
                scramble(c, &mut r);
                c.pc = 0x0017_28f8;
                fill(c, &mut r, crate::difftest::SP - 0x200, 0x80);
                // A value in -m..m, as float bits.
                let sym = |r: &mut Rng, m: f32| {
                    (((r.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0) * m).to_bits()
                };
                // Position in +-50, angles in +-7 rad, size in 0.5..3.5.
                let pos: [f32; 3] = std::array::from_fn(|_| f32::from_bits(sym(&mut r, 50.0)));
                for (k, p) in pos.iter().enumerate() {
                    c.bus.write32(CVB_TEST_BOX + 4 * k as u32, p.to_bits());
                }
                for k in 0..3u32 {
                    c.bus.write32(CVB_TEST_BOX + 0xc + 4 * k, sym(&mut r, 7.0));
                }
                for k in 0..3u32 {
                    let s = 0.5 + (r.next_u32() as f32 / u32::MAX as f32) * 3.0;
                    c.bus.write32(CVB_TEST_BOX + 0x18 + 4 * k, s.to_bits());
                }
                // Even seeds put the point near the box (usually inside), odd
                // seeds far away (outside).
                let spread = if seed % 2 == 0 { 0.3 } else { 20.0 };
                let mut v = [0u32; 4];
                for k in 0..3 {
                    v[k] = (pos[k] + f32::from_bits(sym(&mut r, spread))).to_bits();
                }
                v[3] = r.next_u32();
                put_vec(c, CVB_TEST_PT, v);
                // The axis table's first-call flag: both states.
                c.bus.write32(
                    GAME_GP.wrapping_add(0x1cfc),
                    if seed % 4 == 0 { 1 } else { 0 },
                );
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(CVB_TEST_BOX);
                c.g[A1][0] = sx(CVB_TEST_PT);
            },
        );
    }

    /// The trigger camera (`self`), its rider target object, and the target's
    /// vtable (rider methods in slots 1, 3 and 4, at +8, +0x18 and +0x20).
    pub(crate) const BCU_SELF: u32 = DATA + 0x1000;

    pub(crate) const BCU_TARGET: u32 = DATA + 0x2000;

    pub(crate) const BCU_VT: u32 = DATA + 0x3000;

    /// A uniform value in -m..m, as float bits.
    pub(crate) fn bcu_sym(r: &mut Rng, m: f32) -> u32 {
        (((r.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0) * m).to_bits()
    }

    #[test]
    fn bounded_camera_algorithm_base_update_matches() {
        fakes::covered(
            ee::s02::f_001736e0__generated,
            bounded_camera_algorithm_base_update,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = fakes::machine(c, seed, 0x0017_36e0, 3);
                c.pc = 0x0017_36e0;
                let unit = |r: &mut Rng| r.next_u32() as f32 / u32::MAX as f32;
                // The rider's vtable: its three method slots call fakes.
                c.bus.write32(BCU_TARGET, BCU_VT);
                for (k, off) in [0x8u32, 0x18, 0x20].into_iter().enumerate() {
                    c.bus.write16(BCU_VT + off, 0x10 * k as u16);
                    c.bus
                        .write32(BCU_VT + off + 4, fakes::FAKE + 0x10 * k as u32);
                }
                // The rider's position, velocity and front land in the
                // stack vectors (the fake methods write nothing): focus at
                // sp+0, velocity at +0x10, front at +0x20 of the new frame.
                let frame = crate::difftest::SP - 0xe0;
                let pos = [
                    bcu_sym(&mut r, 50.0),
                    bcu_sym(&mut r, 50.0),
                    bcu_sym(&mut r, 50.0),
                    0,
                ];
                let speed = if seed % 4 == 0 { 4000.0 } else { 30.0 };
                let vel = [
                    bcu_sym(&mut r, speed),
                    bcu_sym(&mut r, speed),
                    bcu_sym(&mut r, speed),
                    0,
                ];
                let mut front = [
                    bcu_sym(&mut r, 1.0),
                    bcu_sym(&mut r, 1.0),
                    bcu_sym(&mut r, 1.0),
                    0,
                ];
                // Zero front components reach the x == 0 and y == 0 atan cases.
                if seed % 6 == 0 {
                    front[0] = 0;
                }
                if seed % 12 == 0 {
                    front[1] = 0;
                }
                put_vec(c, frame, pos);
                put_vec(c, frame + 0x10, vel);
                put_vec(c, frame + 0x20, front);
                // The camera: with front.x = 0, the focus x is the position's
                // x, so Position.x equal to it makes toFocus.x zero.
                let me = BCU_SELF;
                c.bus.write32(me + 0x30, BCU_TARGET);
                let position = if seed % 6 == 0 {
                    [pos[0], bcu_sym(&mut r, 50.0), bcu_sym(&mut r, 50.0), 0]
                } else {
                    [
                        bcu_sym(&mut r, 50.0),
                        bcu_sym(&mut r, 50.0),
                        bcu_sym(&mut r, 50.0),
                        0,
                    ]
                };
                put_vec(c, me + 0x40, position);
                c.bus.write32(me + 0x50, bcu_sym(&mut r, 3.0)); // Heading
                c.bus.write32(me + 0x54, bcu_sym(&mut r, 1.0)); // Pitch
                c.bus.write32(me + 0x58, (unit(&mut r) * 2.0).to_bits()); // PitchMultiplier
                c.bus.write32(me + 0x5c, (unit(&mut r) * 120.0).to_bits()); // HeadingAdjustRate
                c.bus.write32(me + 0x60, (unit(&mut r) * 120.0).to_bits()); // PitchAdjustRate
                c.bus.write32(me + 0x64, (unit(&mut r) * 120.0).to_bits()); // HeadingLookRate
                c.bus.write32(me + 0x68, (unit(&mut r) * 120.0).to_bits()); // PitchLookRate
                c.bus.write32(me + 0x6c, bcu_sym(&mut r, 5.0)); // HeightOffset
                                                                // Distances far apart on a third of the seeds (the |dd| > 5000
                                                                // branch), near otherwise (the k-scaled step).
                let ideal = if seed % 3 == 0 {
                    10000.0 * unit(&mut r)
                } else {
                    30.0 * unit(&mut r)
                };
                c.bus.write32(me + 0x70, ideal.to_bits()); // IdealDist
                c.bus.write32(me + 0x74, bcu_sym(&mut r, 20.0)); // LeadDistance
                c.bus.write32(me + 0x78, bcu_sym(&mut r, 30.0)); // PitchOffset
                c.g[A0][0] = sx(me);
            },
        );
    }

    /// Gate word of this module's `operator delete` fake (in the machine's RAM).
    pub(crate) const OC497_CTRL: u32 = DATA + 0xf_fd00;

    pub(crate) const OC497_MAGIC: u32 = 0x4f43_4497;

    pub(crate) static OC497_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);

    /// `operator delete` (0x317e50): logged, not run, while the gate is set.
    pub(crate) fn oc497_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC497_CTRL) == OC497_MAGIC {
            let ra = c.g[RA][0] as u32;
            return fake(c, 0x0031_7e50, ra);
        }
        fall_back(c, &OC497_PREV_DELETE, ee::s10::f_00317e50__generated)
    }

    /// `BoundedCameraAlgorithmBase_dtor` with both flag values; the delete
    /// call goes to the gated fake above.
    #[test]
    fn bounded_camera_algorithm_base_dtor_matches() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc497_fake_delete,
            &OC497_PREV_DELETE,
        );
        check(
            ee::s02::f_001732b8__generated,
            bounded_camera_algorithm_base_dtor,
            200,
            &|c, seed| {
                let mut r = Rng(0x497_1732 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.bus.write32(OC497_CTRL, OC497_MAGIC);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = r.next_u32() as u64;
                c.pc = 0x0017_32b8;
            },
        );
    }

    use crate::difftest::SP;

    pub(crate) use crate::game::fe::debugfe::tests::OC506_THIS;

    pub(crate) use crate::game::fe::debugfe::tests::oc506_arm;
    pub(crate) use crate::game::fe::debugfe::tests::oc506_hook_all;

    /// Gate word for the constructor and destructor fakes.
    pub(crate) const OC498_CTRL: u32 = DATA + 0x200000;

    pub(crate) const OC498_MAGIC: u32 = 0x498c_0de5;

    pub(crate) static OC498_PREV_CTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC498_PREV_DTOR: AtomicUsize = AtomicUsize::new(0);

    /// The constructor, gated: with the gate on it only returns to `ra`.
    pub(crate) fn oc498_fake_ctor(c: &mut Cpu) {
        if c.bus.read32(OC498_CTRL) != OC498_MAGIC {
            return fakes::fall_back(c, &OC498_PREV_CTOR, ee::s02::f_0016c800__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// The destructor, gated: with the gate on it only returns to `ra`.
    pub(crate) fn oc498_fake_dtor(c: &mut Cpu) {
        if c.bus.read32(OC498_CTRL) != OC498_MAGIC {
            return fakes::fall_back(c, &OC498_PREV_DTOR, ee::s02::f_0016c888__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn oc498_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C800),
            oc498_fake_ctor,
            &OC498_PREV_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C888),
            oc498_fake_dtor,
            &OC498_PREV_DTOR,
        );
    }

    #[test]
    fn camera_trigger_man_static_init_destroy_matches() {
        oc498_install_fakes();
        check(
            ee::s02::f_00175a20__generated,
            camera_trigger_man_static_init_destroy,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x498, 0);
                c.bus.write32(OC498_CTRL, OC498_MAGIC);
                c.pc = 0x0017_5a20;
                // init: 0 (destroy), or a non-zero value (initialise).
                c.g[A0][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                // prio: 0xffff for most seeds, anything else returns at once.
                c.g[A1][0] = if seed % 4 == 0 {
                    (r.next_u32() as u64) & 0xfffe
                } else {
                    0xffff
                };
            },
        );
    }

    /// The literal-pool words the generated body loads from `gp - K`, as
    /// (address, bits): the fixture writes them so both sides load the same
    /// constants.
    pub(crate) const DIRECTION_NE_POOL: &[(u32, u32)] = &[
        (0x0049d1bc, 0x3effeeb3),
        (0x0049d1c0, 0x3e04a01b),
        (0x0049d1c4, 0x3e65a6c6),
        (0x0049d1c8, 0x39d349bf),
        (0x0049d1cc, 0x3eee2fbe),
        (0x0049d1d0, 0x3e36323c),
        (0x0049d1d4, 0x3ee1e18f),
        (0x0049d1d8, 0x3ee1309c),
        (0x0049d1dc, 0x3f004157),
        (0x0049d1e0, 0x3e8dc24c),
        (0x0049d1e4, 0x3f70b2fb),
        (0x0049d1e8, 0x3f0ae1ac),
        (0x0049d1ec, 0x3ea4b685),
        (0x0049d1f0, 0x3f7e8d54),
        (0x0049d1f4, 0x3ed3e8de),
        (0x0049d1f8, 0x3ea46aa1),
        (0x0049d1fc, 0x3e7e75bc),
        (0x0049d200, 0x3f23ba56),
        (0x0049d204, 0x3f64c5da),
        (0x0049d208, 0x3f5e6517),
        (0x0049d20c, 0x3f0ccccd),
        (0x0049d210, 0x3e0975f3),
        (0x0049d214, 0x3f06bead),
        (0x0049d218, 0x3f01512f),
        (0x0049d21c, 0x3f08f5c3),
        (0x0049d220, 0x3d233376),
        (0x0049d224, 0x3f7d198b),
        (0x0049d228, 0x3d7ccd10),
        (0x0049d22c, 0x3f4c7fdc),
        (0x0049d230, 0x3f48665e),
        (0x0049d234, 0x3f6c4d01),
        (0x0049d238, 0x3eafad08),
        (0x0049d23c, 0x3d266666),
        (0x0049d240, 0x3eb5332b),
        (0x0049d244, 0x3ea80022),
        (0x0049d248, 0x3f5c800f),
        (0x0049d24c, 0x3ec332f0),
        (0x0049d250, 0x3d780022),
        (0x0049d254, 0x3f4c3255),
        (0x0049d258, 0x3f6c19b1),
        (0x0049d25c, 0x3eaf0004),
        (0x0049d260, 0x3f48e64b),
        (0x0049d264, 0x3ee66666),
        (0x0049d268, 0x3f4ccccd),
        (0x0049d26c, 0x3f336178),
        (0x0049d270, 0x3d29d51b),
        (0x0049d274, 0x3eb4a255),
        (0x0049d278, 0x3efbccd1),
        (0x0049d27c, 0x3ea61cbd),
        (0x0049d280, 0x3f48690e),
        (0x0049d284, 0x3d7b84dc),
        (0x0049d288, 0x3ea65626),
        (0x0049d28c, 0x3f4c386d),
        (0x0049d290, 0x3efcccab),
        (0x0049d294, 0x3eb53309),
        (0x0049d298, 0x3d1fbe77),
        (0x0049d29c, 0x3ea7995f),
        (0x0049d2a0, 0x3d79db23),
        (0x0049d2a4, 0x3f4cccab),
        (0x0049d2a8, 0x3f757ffa),
        (0x0049d2ac, 0x3f1d47f1),
        (0x0049d2b0, 0x3c8fffbd),
        (0x0049d2b4, 0x3d219978),
        (0x0049d2b8, 0x3f6db329),
        (0x0049d2bc, 0x3eae1262),
        (0x0049d2c0, 0x3f5fb2ff),
        (0x0049d2c4, 0x3e7f4cf5),
        (0x0049d2c8, 0x3f48ccbc),
        (0x0049d2cc, 0x3d7b3312),
    ];

    #[test]
    fn direction_ne_chase_static_init_matches() {
        check(
            ssx3_ee::s02::f_00177650__generated,
            direction_ne_chase_static_init,
            200,
            &|c, seed| {
                let mut r = Rng(0x1776_5000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = if seed % 4 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[A1][0] = if seed % 3 == 0 {
                    r.next_u32() as u64
                } else {
                    0xffff
                };
                c.pc = 0x0017_7650;
                for &(addr, bits) in DIRECTION_NE_POOL {
                    c.bus.write32(addr, bits);
                }
            },
        );
    }

    /// The object the tests point `a0` at, clear of the random vtables
    /// `vtable` draws from (`DATA..DATA+0xfffff`).
    pub(crate) const OBJ_2: u32 = DATA + 0x20_0000;

    /// Zero the first `words` words of the test object.
    pub(crate) fn zero_object(c: &mut Cpu, words: u32) {
        for i in 0..words {
            c.bus.write32(OBJ_2 + 4 * i, 0);
        }
    }

    /// `words` random floats (within +-4) at `base`.
    pub(crate) fn floats(c: &mut Cpu, r: &mut Rng, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, r.f32(4.0));
        }
    }

    /// The update fixture: `this` zeroed, `this+0x80` points at a record of
    /// random floats whose `+0x24` flag is `seed & 1` (both branches).
    pub(crate) fn update_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        zero_object(c, 0x40);
        let rec = OBJ_2 + 0x300;
        floats(c, r, rec, 0x14);
        c.bus.write32(rec + 0x24, seed & 1);
        c.bus.write32(OBJ_2 + 0x80, rec);
        c.g[A0][0] = OBJ_2 as u64;
    }

    /// The reset fixture: `a1` is the rider. The first fake call (the rider's
    /// pose query, `call 0`) returns the rider on even seeds (the equal
    /// branch) and something else on odd ones.
    pub(crate) fn reset_fixture(c: &mut Cpu, seed: u32) {
        zero_object(c, 0x40);
        let rider = OBJ_2 + 0x400;
        c.g[A0][0] = OBJ_2 as u64;
        c.g[A1][0] = rider as u64;
        let forced = if seed % 2 == 0 { rider } else { rider + 0x10 };
        c.bus.write32(crate::difftest::fakes::LOG_RESULTS, forced);
    }

    #[test]
    fn bounded_camera_algorithm_base_get_view_matches() {
        check(
            ee::s02::f_00173e40__generated,
            bounded_camera_algorithm_base_get_view,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x173e40, 3);
                zero_object(c, 0x40);
                let camera = OBJ_2 + 0x100;
                floats(c, &mut r, camera, 0x20);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = camera as u64;
            },
        );
    }

    #[test]
    fn bounded_camera_algorithm_update_matches() {
        check(
            ee::s02::f_00174200__generated,
            bounded_camera_algorithm_update,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x174200, 3);
                update_fixture(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn bounded_camera_algorithm_reset_for_rider_matches() {
        check(
            ee::s02::f_001744b0__generated,
            bounded_camera_algorithm_reset_for_rider,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x1744b0, 3);
                reset_fixture(c, seed);
            },
        );
    }

    #[test]
    fn default_2_chase_camera_algorithm_update_matches() {
        check(
            ee::s02::f_00176b10__generated,
            default_2_chase_camera_algorithm_update,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x176b10, 3);
                update_fixture(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn default_4_chase_camera_algorithm_update_matches() {
        check(
            ee::s02::f_00177110__generated,
            default_4_chase_camera_algorithm_update,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x177110, 3);
                update_fixture(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn direction_ne_chase_camera_algorithm_update_matches() {
        check(
            ee::s02::f_00177368__generated,
            direction_ne_chase_camera_algorithm_update,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x177368, 3);
                update_fixture(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn spoke_chase_camera_algorithm_reset_for_rider_matches() {
        check(
            ee::s02::f_001783e0__generated,
            spoke_chase_camera_algorithm_reset_for_rider,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x1783e0, 3);
                reset_fixture(c, seed);
            },
        );
    }

    #[test]
    fn bounded_camera_algorithm_dtor_matches() {
        check(
            ee::s02::f_001741d8__generated,
            bounded_camera_algorithm_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1741d8, 1);
                zero_object(c, 0x40);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn camera_trigger_man_member45bde0_dtor_matches() {
        check(
            ee::s02::f_00176270__generated,
            camera_trigger_man_member45bde0_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x176270, 1);
                c.g[A0][0] = OBJ_2 as u64;
                // Even flags: no `operator delete` call (the delete path
                // would free into the empty test heap).
                c.g[A1][0] = (r.next_u32() & !1) as u64;
            },
        );
    }

    #[test]
    fn camera_trigger_man_member10_call_v05_matches() {
        check(
            ee::s02::f_001762f0__generated,
            camera_trigger_man_member10_call_v05,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1762f0, 1);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(OBJ_2 + 0x10, vt);
                c.g[A0][0] = OBJ_2 as u64;
            },
        );
    }

    #[test]
    fn script_camera_algo_dtor_matches() {
        check(
            ee::s02::f_00176328__generated,
            script_camera_algo_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x176328, 1);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = (r.next_u32() & !1) as u64;
            },
        );
    }

    #[test]
    fn subject_script_camera_algo_dtor_matches() {
        check(
            ee::s02::f_001763c8__generated,
            subject_script_camera_algo_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1763c8, 1);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = (r.next_u32() & !1) as u64;
            },
        );
    }

    #[test]
    fn camera_trigger_man_global_ctors_matches() {
        check(
            ee::s02::f_00176a28__generated,
            camera_trigger_man_global_ctors,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x176a28, 1);
            },
        );
    }

    #[test]
    fn camera_trigger_man_global_dtors_matches() {
        check(
            ee::s02::f_00176a48__generated,
            camera_trigger_man_global_dtors,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x176a48, 1);
            },
        );
    }

    #[test]
    fn default_2_chase_camera_algorithm_dtor_matches() {
        check(
            ee::s02::f_00176aa8__generated,
            default_2_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x176aa8, 1);
                zero_object(c, 0x40);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn default_4_chase_camera_algorithm_dtor_matches() {
        check(
            ee::s02::f_001770a8__generated,
            default_4_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1770a8, 1);
                zero_object(c, 0x40);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn direction_ne_chase_camera_algorithm_dtor_matches() {
        check(
            ee::s02::f_00177520__generated,
            direction_ne_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x177520, 1);
                zero_object(c, 0x40);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn direction_ne_chase_camera_algorithm_global_ctor_matches() {
        check(
            ee::s02::f_00177e30__generated,
            direction_ne_chase_camera_algorithm_global_ctor,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x177e30, 1);
            },
        );
    }

    #[test]
    fn spoke_chase_camera_algorithm_dtor_matches() {
        check(
            ee::s02::f_00177e90__generated,
            spoke_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x177e90, 1);
                zero_object(c, 0x40);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn superpipe_chase_camera_algorithm_dtor_matches() {
        check(
            ee::s02::f_00178588__generated,
            superpipe_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x178588, 1);
                zero_object(c, 0x40);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn post_race_1chase_camera_algorithm_static_init_matches() {
        check(
            ee::s02::f_00178f58__generated,
            post_race_1chase_camera_algorithm_static_init,
            200,
            &|c, seed| {
                let mut r = Rng(0x0178_f58f ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0017_8f58;
                // a1 == 0xffff and a0 != 0 run the table; the other seeds take
                // each early exit.
                c.g[A1][0] = if seed % 3 == 0 { 0x1234 } else { 0xffff };
                c.g[A0][0] = if seed % 4 == 0 { 0 } else { 1 };
            },
        );
    }

    /// The gp-relative float pool the body reads (GP-0x5c34 ..GP-0x5b24).
    pub(crate) const POOL_LO: u32 = GAME_GP - 0x5c34;

    pub(crate) const POOL_BYTES: u32 = 0x114;

    #[test]
    fn camera_consts_static_init_9798_matches() {
        check(
            ee::s02::f_00179798__generated,
            camera_consts_static_init_9798,
            200,
            &|c, seed| {
                let mut r = Rng(0x9798_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0017_9798;
                c.g[GP][0] = GAME_GP as u64;
                fill(c, &mut r, POOL_LO, POOL_BYTES / 4);
                match seed % 4 {
                    // the full path: priority 0xFFFF, initialize set
                    0 => {
                        c.g[A1][0] = 0xffff;
                        c.g[A0][0] = 1;
                    }
                    // initialize == 0: nothing is written
                    1 => {
                        c.g[A1][0] = 0xffff;
                        c.g[A0][0] = 0;
                    }
                    // another priority: nothing is written
                    2 => {
                        c.g[A1][0] = r.next_u32() as u64 | 1 << 40;
                        c.g[A0][0] = r.next_u32() as u64;
                    }
                    _ => {
                        c.g[A1][0] = 0xffff;
                        c.g[A0][0] = (r.next_u32() as u64) << 32 | 1;
                    }
                }
            },
        );
    }

    /// The gate word the fakes below check in the machine's RAM.
    pub(crate) const OC504_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC504_MAGIC: u32 = 0x0504_0504;

    pub(crate) static OC504_PREV_CHASE_CTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC504_PREV_CHASE_DTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC504_PREV_CHASE_GATHER: AtomicUsize = AtomicUsize::new(0);

    /// The base chase constructor, gated: with the gate on it only returns.
    pub(crate) fn oc504_fake_chase_ctor(c: &mut Cpu) {
        if c.bus.read32(OC504_CTRL) != OC504_MAGIC {
            return fakes::fall_back(c, &OC504_PREV_CHASE_CTOR, ee::s01::f_00162318__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// The base chase destructor, gated: with the gate on it only returns.
    pub(crate) fn oc504_fake_chase_dtor(c: &mut Cpu) {
        if c.bus.read32(OC504_CTRL) != OC504_MAGIC {
            return fakes::fall_back(c, &OC504_PREV_CHASE_DTOR, ee::s01::f_00162458__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// `ChaseCameraAlgorithm_GatherRiderState`, gated: with the gate on it only returns.
    pub(crate) fn oc504_fake_gather(c: &mut Cpu) {
        if c.bus.read32(OC504_CTRL) != OC504_MAGIC {
            return fakes::fall_back(c, &OC504_PREV_CHASE_GATHER, ee::s01::f_00162568__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn oc504_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00162318),
            oc504_fake_chase_ctor,
            &OC504_PREV_CHASE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00162458),
            oc504_fake_chase_dtor,
            &OC504_PREV_CHASE_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00162568),
            oc504_fake_gather,
            &OC504_PREV_CHASE_GATHER,
        );
    }

    /// A chase object at `this`: Point1 and Point2 random, the target object
    /// at +0x30 with a vtable whose slots (+0x18, +0x20, +0x8) call FAKE.
    pub(crate) fn chase_setup(c: &mut Cpu, r: &mut Rng, this: u32, obj: u32, vt: u32) {
        put_vec(
            c,
            this + 0x390,
            [r.f32(900.0), r.f32(900.0), r.f32(900.0), 0x3f80_0000],
        );
        put_vec(
            c,
            this + 0x3a0,
            [r.f32(900.0), r.f32(900.0), r.f32(900.0), 0x3f80_0000],
        );
        c.bus.write32(this + 0x30, obj);
        c.bus.write32(obj, vt);
        for off in [0x8u32, 0x18, 0x20] {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, FAKE);
        }
    }

    #[test]
    fn superpipe_compute_desired_eye_matches() {
        fakes::covered(
            ee::s02::f_001785f0__generated,
            superpipe_chase_camera_compute_desired_eye,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x504_1, 1);
                c.pc = 0x0017_85f0;
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x5dac), 1500.0f32.to_bits());
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                chase_setup(c, &mut r, DATA + 0x2000, DATA + 0x3000, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn superpipe_compute_look_at_matches() {
        fakes::covered(
            ee::s02::f_00178758__generated,
            superpipe_chase_camera_compute_look_at,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x504_2, 1);
                c.pc = 0x0017_8758;
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                chase_setup(c, &mut r, DATA + 0x2000, DATA + 0x3000, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn superpipe_chase_camera_update_matches() {
        oc504_install_fakes();
        check(
            ee::s02::f_001787e0__generated,
            superpipe_chase_camera_update,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x504_3, 3);
                c.bus.write32(OC504_CTRL, OC504_MAGIC);
                c.pc = 0x0017_87e0;
                c.g[A0][0] = (DATA + 0x2000) as u64;
                for off in [
                    0x5da8u32, 0x5da4, 0x5da0, 0x5d9c, 0x5d98, 0x5d94, 0x5d90, 0x5d8c, 0x5d88,
                ] {
                    c.bus.write32(GAME_GP.wrapping_sub(off), r.f32(2000.0));
                }
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x5dac), 1500.0f32.to_bits());
                chase_setup(c, &mut r, DATA + 0x2000, DATA + 0x3000, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn post_race_1_chase_camera_ctor_matches() {
        oc504_install_fakes();
        check(
            ee::s02::f_001789e8__generated,
            post_race_1_chase_camera_ctor,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x504_4, 2);
                c.bus.write32(OC504_CTRL, OC504_MAGIC);
                c.pc = 0x0017_89e8;
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                let this = DATA + 0x1000;
                put_vec(
                    c,
                    this + 0x390,
                    [r.f32(4.0), r.f32(4.0), r.f32(4.0), 0x3f80_0000],
                );
                // The temporary the first call's result lands in (the fake leaves it alone).
                // (The frame is 0x80 below SP, so this is its +0x10 slot.)
                put_vec(c, SP - 0x70, [r.f32(4.0), r.f32(4.0), r.f32(4.0), 0]);
                // Rider object at A1 with the vtable slots +0x18 and +0x20.
                let vt = DATA + 0x4000;
                c.bus.write32(DATA + 0x2000, vt);
                for off in [0x18u32, 0x20] {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(vt + off, delta as u16);
                    c.bus.write32(vt + off + 4, FAKE);
                }
            },
        );
    }

    #[test]
    fn post_race_1_chase_camera_dtor_matches() {
        oc504_install_fakes();
        check(
            ee::s02::f_00178b70__generated,
            post_race_1_chase_camera_dtor,
            200,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x504_5, 1);
                c.bus.write32(OC504_CTRL, OC504_MAGIC);
                c.pc = 0x0017_8b70;
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn post_race_1_chase_camera_global_ctor_matches() {
        check(
            ee::s02::f_00179738__generated,
            post_race_1_chase_camera_global_ctor,
            4,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x504_6, 0);
                c.pc = 0x0017_9738;
            },
        );
    }

    #[test]
    fn camera_consts_global_ctor_9fa8_matches() {
        check(
            ee::s02::f_00179fa8__generated,
            camera_consts_global_ctor_9fa8,
            4,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x504_7, 0);
                c.pc = 0x0017_9fa8;
            },
        );
    }

    #[test]
    fn debug_menu_v01_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcd00__generated,
            debug_menu_v01,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dcd00, 3);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    /// The bound readers' object (the reader's stream cursor at `READER`).
    pub(crate) const OC637_OBJ: u32 = DATA + 0x6000;

    pub(crate) const OC637_READER: u32 = DATA + 0x6100;

    pub(crate) const OC637_STREAM: u32 = DATA + 0x6200;

    pub(crate) const OC637_VT: u32 = DATA + 0x6400;

    /// The spline's `*out` cell.
    pub(crate) const OC637_OUT: u32 = DATA + 0x6800;

    /// The block `EA_MemAllocNamed` returns to the spline (gate on).
    pub(crate) const OC637_NEW_BLOCK: u32 = DATA + 0x21_0000;

    /// The fake allocator runs only when `OC637_GATE` holds `OC637_GATE_MAGIC`.
    pub(crate) const OC637_GATE: u32 = DATA + 0x20_5000;

    pub(crate) const OC637_GATE_MAGIC: u32 = 0x0637_c0de;

    /// The vtable the spline stores into its object (`0x45c3f0`, in the game image).
    pub(crate) const OC637_SPLINE_VT: u32 = 0x0045_c3f0;

    pub(crate) static OC637_NEW_PREV: AtomicUsize = AtomicUsize::new(0);

    /// Five accessor slots (the offsets the readers and the spline call), each a
    /// fake callee: the `miss` hook logs the call and returns a pointer.
    pub(crate) fn oc637_vtable(c: &mut Cpu, r: &mut crate::difftest::Rng, vt: u32) {
        for (i, off) in [0x08u32, 0x28, 0x10, 0x18, 0x20].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
    }

    /// The readers' fixture: the object with its accessor vtable, the reader whose
    /// first word is the stream cursor (the float and vec3 reads are real guest
    /// code), and random words in the stream.
    pub(crate) fn oc637_reader_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = fakes::machine(c, seed, 0x637_0001, 5);
        c.pc = entry;
        c.g[A0][0] = OC637_READER as u64;
        c.g[A1][0] = OC637_OBJ as u64;
        fill(c, &mut r, OC637_OBJ, 0x40);
        oc637_vtable(c, &mut r, OC637_VT);
        c.bus.write32(OC637_OBJ + 0x24, OC637_VT);
        c.bus.write32(OC637_READER, OC637_STREAM);
        fill(c, &mut r, OC637_STREAM, 0x40);
    }

    #[test]
    fn camera_ellipse_bound_obj_read_matches() {
        fakes::covered(
            ssx3_ee::s02::f_00171a20__generated,
            camera_ellipse_bound_obj_read,
            200,
            &|c, seed| oc637_reader_fixture(c, seed, 0x0017_1a20),
        );
    }

    #[test]
    fn camera_box_bound_obj_read_matches() {
        fakes::covered(
            ssx3_ee::s02::f_00171b10__generated,
            camera_box_bound_obj_read_2,
            200,
            &|c, seed| oc637_reader_fixture(c, seed, 0x0017_1b10),
        );
    }

    #[test]
    fn camera_line_bound_obj_read_matches() {
        fakes::covered(
            ssx3_ee::s02::f_00171c00__generated,
            camera_line_bound_obj_read,
            200,
            &|c, seed| oc637_reader_fixture(c, seed, 0x0017_1c00),
        );
    }

    /// The fake `EA_MemAllocNamed` for the spline's `new` (gate on: the block).
    pub(crate) fn oc637_fake_new(c: &mut Cpu) {
        if c.bus.read32(OC637_GATE) != OC637_GATE_MAGIC {
            return fakes::fall_back(c, &OC637_NEW_PREV, ssx3_ee::s10::f_00317d70__generated);
        }
        c.g[V0][0] = OC637_NEW_BLOCK as u64;
        ret(c);
    }

    pub(crate) fn oc637_install_new() {
        fakes::hook(
            std::ptr::addr_of_mut!(ssx3_ee::s10::OVERRIDE_00317D70),
            oc637_fake_new,
            &OC637_NEW_PREV,
        );
    }

    /// The spline fixture: the reader stream, the `*out` cell, and the vtable at
    /// the address the body stores (its slots are the fake accessors).
    pub(crate) fn oc637_spline_fixture(c: &mut Cpu, seed: u32) {
        oc637_install_new();
        let mut r = fakes::machine(c, seed, 0x637_0002, 5);
        c.pc = 0x0017_1d58;
        c.bus.write32(OC637_GATE, OC637_GATE_MAGIC);
        c.g[A0][0] = OC637_READER as u64;
        c.g[A1][0] = OC637_OUT as u64;
        c.bus.write32(OC637_OUT, r.next_u32());
        c.bus.write32(OC637_READER, OC637_STREAM);
        fill(c, &mut r, OC637_STREAM, 0x40);
        fill(c, &mut r, OC637_NEW_BLOCK, 0x20);
        oc637_vtable(c, &mut r, OC637_SPLINE_VT);
    }

    #[test]
    fn camera_trigger_read_spline_matches() {
        fakes::covered(
            ssx3_ee::s02::f_00171d58__generated,
            camera_trigger_read_spline,
            200,
            &|c, seed| oc637_spline_fixture(c, seed),
        );
    }

    /// Gate word of the fakes (`DATA+0x1f000`, clear of the fixture): the
    /// magic enables them, +4 is the `bFound` the map reports, +8 makes the
    /// map come back missing (`isValidMap` returns 0).
    pub(crate) const OC639_CTRL: u32 = DATA + 0x1_f000;

    pub(crate) const OC639_MAGIC: u32 = 0x6f63_3633;

    /// A gated fake for a callee the body calls through the table: with the
    /// gate on it logs the call and returns (`fakes::fake`), with it off it
    /// runs what the slot held before the hook.
    macro_rules! oc639_gated_fake {
        ($fake:ident, $prev:ident, $target:literal, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC639_CTRL) != OC639_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    oc639_gated_fake!(
        oc639_fake_3182d8,
        OC639_PREV_3182D8,
        0x0031_82d8,
        ee::s10::f_003182d8__generated
    );

    oc639_gated_fake!(
        oc639_fake_3186d0,
        OC639_PREV_3186D0,
        0x0031_86d0,
        ee::s10::f_003186d0__generated
    );

    oc639_gated_fake!(
        oc639_fake_318260,
        OC639_PREV_318260,
        0x0031_8260,
        ee::s10::f_00318260__generated
    );

    oc639_gated_fake!(
        oc639_fake_318630,
        OC639_PREV_318630,
        0x0031_8630,
        ee::s10::f_00318630__generated
    );

    oc639_gated_fake!(
        oc639_fake_3168b0,
        OC639_PREV_3168B0,
        0x0031_68b0,
        ee::s10::f_003168b0__generated
    );

    oc639_gated_fake!(
        oc639_fake_3168e8,
        OC639_PREV_3168E8,
        0x0031_68e8,
        ee::s10::f_003168e8__generated
    );

    pub(crate) static OC639_PREV_3A6948: AtomicUsize = AtomicUsize::new(0);

    /// `World_SdbHasSections(name, flags, nm)`: with the gate on, it sets the
    /// `bFound` word of `nm` (a2+4) from the fixture and may come back 0.
    pub(crate) fn oc639_fake_3a6948(c: &mut Cpu) {
        if c.bus.read32(OC639_CTRL) != OC639_MAGIC {
            return fakes::fall_back(c, &OC639_PREV_3A6948, ee::s13::f_003a6948__generated);
        }
        let nm = c.g[A2][0] as u32;
        let found = c.bus.read32(OC639_CTRL + 4);
        c.bus.write32(nm + 4, found);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003a_6948, ra);
        if c.pc == ra && c.bus.read32(OC639_CTRL + 8) != 0 {
            c.g[V0][0] = 0;
        }
    }

    pub(crate) fn oc639_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003182D8),
            oc639_fake_3182d8,
            &OC639_PREV_3182D8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003186D0),
            oc639_fake_3186d0,
            &OC639_PREV_3186D0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            oc639_fake_318260,
            &OC639_PREV_318260,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318630),
            oc639_fake_318630,
            &OC639_PREV_318630,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003168B0),
            oc639_fake_3168b0,
            &OC639_PREV_3168B0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003168E8),
            oc639_fake_3168e8,
            &OC639_PREV_3168E8,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A6948),
            oc639_fake_3a6948,
            &OC639_PREV_3A6948,
        );
    }

    /// A NUL-terminated string in RAM.
    pub(crate) fn put_cstr(c: &mut Cpu, at: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// `DebugMenu_AddWorldIfMissing` against the generated body: zero to
    /// three location rows whose names match the wanted file and big file
    /// in different ways (case-insensitively for some), so the scan lists
    /// the location on some seeds and runs off the end on the others. The
    /// big file, the section lookup (bFound 0/1, and no map on every fifth
    /// seed) and every callee run as fakes; strcasecmp runs for real.
    #[test]
    fn debug_menu_add_world_if_missing_matches() {
        const SELF: u32 = DATA;
        const NAME: u32 = DATA + 0x2000;
        const TAG: u32 = DATA + 0x2100;
        oc639_install_fakes();
        covered(
            ee::s02::f_0017a830__generated,
            debug_menu_add_world_if_missing,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17a830, 9);
                c.bus.write32(OC639_CTRL, OC639_MAGIC);
                c.bus.write32(OC639_CTRL + 4, r.next_u32() & 1);
                c.bus.write32(OC639_CTRL + 8, (seed % 5 == 0) as u32);
                let rows = seed % 4;
                c.bus.write32(SELF, rows);
                put_cstr(c, NAME, "snow");
                put_cstr(c, TAG, "big");
                for i in 0..rows {
                    let (file, big) = match (seed / 3 + i) % 4 {
                        0 => ("SNOW", "big"),
                        1 => ("snow", "Other"),
                        2 => ("other", "BIG"),
                        _ => ("other", "other"),
                    };
                    put_cstr(c, SELF + 4 + 0x8c * i, file);
                    put_cstr(c, SELF + 0x44 + 0x8c * i, big);
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(NAME);
                c.g[A2][0] = sx(TAG);
                c.pc = 0x0017_a830;
            },
        );
    }

    /// Runs `body` in a child process whose function table is empty, so every
    /// call the function makes through the table lands in the fake hook
    /// (callees with no override slot cannot be faked in-process). The parent
    /// re-runs the named test in the child and checks that it passed.
    pub(crate) fn oc680_in_child(test: &str, body: &dyn Fn()) {
        const ENV: &str = "SSX3_OC680_CHILD";
        if std::env::var_os(ENV).is_some() {
            ssx3_ee_rt::funcs::register_functions(&[]);
            return body();
        }
        // The running test's full path: the helper may live in another unit
        // than the test that calls it.
        let _ = test;
        let name = std::thread::current().name().unwrap_or(test).to_string();
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
    }

    /// Control words of the boundary-point fixture: +0 the number of leading
    /// `containsPoint` calls that say inside, +4 the salt of the later answers,
    /// +8 the bits of the `logf` result.
    pub(crate) const OC680_FIND_CTRL: u32 = DATA + 0x20_0000;

    /// The miss hook of the boundary-point test: the volume's `boundBox`
    /// writes a fixed box, `containsPoint` follows the script, `logf` returns
    /// the fixture's value (the plain fake would return garbage that makes the
    /// bisection run for ever); every call is logged as usual.
    pub(crate) fn oc680_find_hook(c: &mut Cpu, target: u32) {
        let out = c.g[A0][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        let (inside_calls, salt, logf) = (
            c.bus.read32(OC680_FIND_CTRL),
            c.bus.read32(OC680_FIND_CTRL + 4),
            c.bus.read32(OC680_FIND_CTRL + 8),
        );
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        if target == fakes::FAKE {
            let b = |v: f32| v.to_bits();
            for (i, v) in [-8.0, -6.0, -4.0, 1.0, 8.0, 6.0, 4.0, 1.0]
                .into_iter()
                .enumerate()
            {
                c.bus.write32(out + 4 * i as u32, b(v));
            }
        } else if target == fakes::FAKE + 0x10 {
            c.g[V0][0] = if n < inside_calls {
                1
            } else {
                fakes::mix(salt as u64 + n as u64) & 1
            };
        } else if target == 0x0040_da10 {
            c.f[0] = logf;
        }
    }

    /// `CameraTrigger_FindBoundaryPoint(out, volume, pos, step)`: the volume's
    /// `boundBox`, `containsPoint` and `logf` are logged fakes with a scripted
    /// answer. Fixture: 0-3 leading "inside" answers (the doubling loop runs
    /// 0-3 times), then random answers for the bisection; `logf` values that
    /// give no steps, one step and several; unwinds at random calls.
    #[test]
    fn oc680_camera_trigger_find_boundary_point_matches() {
        oc680_in_child("oc680_camera_trigger_find_boundary_point_matches", &|| {
            covered(
                ee::s02::f_00171fa8__generated,
                oc680_camera_trigger_find_boundary_point,
                200,
                &|c, seed| {
                    let mut r = machine(c, seed, 0x0680_0002, 24);
                    c.hooks.miss = oc680_find_hook;
                    c.pc = 0x0017_1fa8;
                    c.g[RA][0] = crate::difftest::RA as u64;
                    let (out, vol, pos, vt) =
                        (DATA + 0x100, DATA + 0x200, DATA + 0x300, DATA + 0x400);
                    c.g[A0][0] = sx(out);
                    c.g[A1][0] = sx(vol);
                    c.g[A2][0] = sx(pos);
                    c.f[12] = [0.5f32, 1.0, 0.25, 2.0][(seed % 4) as usize].to_bits();
                    c.bus.write32(vol + 0x24, vt);
                    for (off, i) in [(0x30u32, 0u32), (0x38, 1)] {
                        c.bus.write16(vt + off, ((r.next_u32() & 0x1c) as u16) & !3);
                        c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i);
                    }
                    for i in 0..4u32 {
                        let v = (r.next_u32() % 41) as f32 - 20.0;
                        c.bus
                            .write32(pos + 4 * i, if i == 3 { 1.0f32 } else { v }.to_bits());
                    }
                    c.bus.write32(OC680_FIND_CTRL, seed / 4 % 4);
                    c.bus.write32(OC680_FIND_CTRL + 4, r.next_u32());
                    let logf = [-3.0f32, 0.0, 0.4, 2.5, 5.0][(seed / 3 % 5) as usize];
                    c.bus.write32(OC680_FIND_CTRL + 8, logf.to_bits());
                },
            );
        });
    }

    /// Control block of the debug-menu / reset fixtures (clear of the data
    /// the tests build): +0 returned key, +4 salt for scripted answers, +8
    /// "the file loads", +0xc file length, +0x10.. front/position vectors.
    pub(crate) const OC680_CTRL: u32 = DATA + 0x21_0000;

    pub(crate) const OC680_BUF: u32 = DATA + 0x22_0000;

    pub(crate) const OC680_STR: u32 = DATA + 0x23_0000;

    /// The miss hook of the debug-menu and trigger-reset fixtures: logs every
    /// call like the plain fake and gives the callees whose results steer the
    /// control flow a scripted answer (the plain fake returns a non-zero
    /// pointer for everything, which never takes the zero branches).
    pub(crate) fn oc680_hook(c: &mut Cpu, target: u32) {
        let (a0, a1, a2) = (c.g[A0][0] as u32, c.g[A1][0] as u32, c.g[A2][0] as u32);
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ctrl = OC680_CTRL;
        let salt = c.bus.read32(ctrl + 4) as u64;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        let bit = |k: u64| fakes::mix(salt.wrapping_add(k)) >> 7 & 1;
        match target {
            // The trigger's rider: AI index, position, front vector.
            t if t == fakes::FAKE => c.g[V0][0] = c.bus.read32(ctrl) as u64,
            t if t == fakes::FAKE + 0x10 || t == fakes::FAKE + 0x20 => {
                let src = ctrl + (t - fakes::FAKE);
                for i in 0..4 {
                    let w = c.bus.read32(src + 4 * i);
                    c.bus.write32(a0 + 4 * i, w);
                }
            }
            // atan(y/x): a fixed angle; fSinCos(&sin, &cos, angle).
            0x0031_c228 => c.f[0] = 0.5f32.to_bits(),
            0x0031_be50 => {
                c.bus.write32(a0, 0.25f32.to_bits());
                c.bus.write32(a1, 0.75f32.to_bits());
            }
            // strcasecmp: equal (0) or not.
            0x0041_aa88 => c.g[V0][0] = bit(n as u64),
            // isValidMap(base, n, names): marks entries found, returns any.
            0x003a_6948 => {
                for k in 0..a1.min(32) {
                    let found = bit(0x100 + n as u64 * 8 + k as u64);
                    c.bus.write32(a2 + 8 * k + 4, found as u32);
                }
                c.g[V0][0] = bit(0x200 + n as u64);
            }
            // FILE_loadsizez(path, &size, flags) / FILE_loadz(path, flags).
            0x003e_1a70 => {
                let loads = c.bus.read32(ctrl + 8) != 0;
                c.g[V0][0] = if loads { OC680_BUF as u64 } else { 0 };
                let len = c.bus.read32(ctrl + 0xc);
                c.bus.write32(a1, len);
            }
            0x003e_1908 => {
                let loads = c.bus.read32(ctrl + 8) != 0;
                c.g[V0][0] = if loads { OC680_BUF as u64 } else { 0 };
            }
            // cBXString(&str, start, len): the string pointer is the first word.
            0x0031_8350 => c.bus.write32(a0, OC680_STR),
            // SHAPE_unpack(shape): the same pointer, nothing or another.
            0x003b_3578 => {
                c.g[V0][0] = match bit(0x300 + n as u64) + bit(0x400 + n as u64) {
                    0 => a0 as u64,
                    1 => 0,
                    _ => (a0 + 4) as u64,
                }
            }
            // The uber-trick menu: some screen objects are missing.
            0x0039_d860 => {
                if bit(0x500 + n as u64) == 0 {
                    c.g[V0][0] = 0;
                }
            }
            _ => {}
        }
    }

    /// Common set-up of the hook fixtures: scrambled machine, our hook, entry
    /// `pc`, the return address and the control block.
    pub(crate) fn oc680_start(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        entry: u32,
    ) -> crate::difftest::Rng {
        let r = machine(c, seed, salt, calls);
        c.hooks.miss = oc680_hook;
        c.pc = entry;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.bus
            .write32(OC680_CTRL + 4, seed.wrapping_mul(0x9e37_79b1));
        r
    }

    /// `BoundedCameraAlgorithmBase_ResetForRider(self, key)`: the rider's AI
    /// index (a scripted vtable call) matches `key` or not; the front vector
    /// takes all 27 sign combinations of (x, y, z) in {0, +, -}, which reach
    /// every quadrant and zero case of the two inlined `atan2`s and the
    /// pitch wrap; the callees are logged fakes; unwinds at random calls.
    #[test]
    fn oc680_bounded_camera_algorithm_base_reset_for_rider_matches() {
        oc680_in_child(
            "oc680_bounded_camera_algorithm_base_reset_for_rider_matches",
            &|| {
                covered(
                    ee::s02::f_001732e8__generated,
                    oc680_bounded_camera_algorithm_base_reset_for_rider,
                    270,
                    &|c, seed| {
                        let mut r = oc680_start(c, seed, 0x0680_0003, 8, 0x0017_32e8);
                        let (this, target, rider, vt_r, vt_t, vt_s) = (
                            DATA + 0x100,
                            DATA + 0x200,
                            DATA + 0x300,
                            DATA + 0x400,
                            DATA + 0x500,
                            DATA + 0x600,
                        );
                        let key = r.next_u32() & 0xff;
                        c.g[A0][0] = sx(this);
                        c.g[A1][0] = key as u64;
                        c.bus
                            .write32(OC680_CTRL, if seed % 5 == 0 { key ^ 1 } else { key });
                        c.bus.write32(this + 0x30, target);
                        c.bus.write32(this + 0x10, vt_s);
                        c.bus.write32(target, vt_t);
                        c.bus.write32(target + 4, rider);
                        c.bus.write32(rider + 0x6c0, vt_r);
                        for (vt, off, i) in [
                            (vt_r, 0x38u32, 0u32),
                            (vt_t, 0x8, 1),
                            (vt_t, 0x20, 2),
                            (vt_s, 0x28, 3),
                        ] {
                            c.bus.write16(vt + off, 0);
                            c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i);
                        }
                        for (off, v) in [
                            (0x6cu32, 1.5f32),
                            (0x70, 2.0),
                            (0x74, 3.0),
                            (0x78, 0.5),
                            (0x58, 0.25),
                        ] {
                            c.bus
                                .write32(this + off, (v * (1.0 + (seed % 7) as f32)).to_bits());
                        }
                        // World Z axis (gp - 0xea0) and the angle constants of the gp pool.
                        c.bus.write32(0x004f_f160 + 8, 1.0f32.to_bits());
                        let pi = std::f32::consts::PI;
                        for (off, v) in [
                            (0x6428u32, -pi / 2.0),
                            (0x6424, pi / 2.0),
                            (0x6420, pi),
                            (0x641c, pi),
                            (0x6418, 0.159_154_94),
                            (0x6414, 2.0 * pi),
                            (0x6410, -pi / 2.0),
                            (0x640c, pi / 2.0),
                            (0x6408, pi),
                            (0x6404, pi),
                            (0x6400, 0.017_453_292),
                            (0x63fc, 0.159_154_94),
                            (0x63f8, 2.0 * pi),
                        ] {
                            c.bus.write32(GAME_GP - off, v.to_bits());
                        }
                        let pick = |k: u32, v: f32| [0.0, v, -v][(k % 3) as usize].to_bits();
                        let combo = seed % 27;
                        let front = [
                            pick(combo, 2.0),
                            pick(combo / 3, 3.0),
                            pick(combo / 9, 1.5),
                            0,
                        ];
                        for i in 0..4 {
                            c.bus.write32(OC680_CTRL + 0x20 + 4 * i, front[i as usize]);
                            c.bus.write32(
                                OC680_CTRL + 0x10 + 4 * i,
                                ((r.next_u32() % 200) as f32 - 100.0).to_bits(),
                            );
                        }
                    },
                );
            },
        );
    }

    #[test]
    fn debug_menu_dispatch_item_op_matches() {
        check(
            ee::s02::f_0017a158__generated,
            debug_menu_dispatch_item_op,
            200,
            &|c, seed| {
                machine(c, seed, 0xa158, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0017a158;
            },
        );
    }

    pub(crate) use crate::game::main::loadscreens::tests::oc736_fixture;

    pub(crate) use crate::game::main::loadscreens::tests::oc736_world_scan;

    pub(crate) use crate::game::main::loadscreens::tests::FX_SELF;

    /// `DebugMenu_InitWorldList`: failing and passing file checks, records
    /// accepted by 50 % of the junk `ok` words, selectable and not.
    #[test]
    fn debug_menu_init_world_list_matches() {
        oc736_install_fakes();
        fakes::covered(
            ee::s02::f_0017a638__generated,
            debug_menu_init_world_list,
            250,
            &|c, seed| {
                let mut r = oc736_fixture(c, seed, 0x0017_a638, 120);
                oc736_world_scan(c, seed);
                fill(c, &mut r, FX_SELF, 0x40);
                c.g[A0][0] = sx(FX_SELF);
                c.pc = 0x0017_a638;
            },
        );
    }
}
