//! `src/camera/camera.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::bx::bxstring::gp as gp_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::chase_reset_for_rider;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::BackHeight;
pub(crate) use crate::game::fe::feasyncfile::gp_11 as gp_8;
pub(crate) use crate::game::fe::feasyncfile::gp_8 as gp_6;
pub(crate) use crate::game::fe::feasyncfile::gpn;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_3;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::u32_at;
pub(crate) use crate::game::fe::festateload::at as at_2;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::fe::splashmanager::stk;
pub(crate) use crate::game::render::graphicsman::at as at_3;
pub(crate) use crate::game::render::graphicsman::gp as gp_5;
pub(crate) use crate::game::render::lightman::at;
pub(crate) use crate::game::render::lightman::gp_2 as gp;
pub(crate) use crate::game::scripter::ssxscriptengine::gp_5 as gp_2;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::ui::uivector::r;
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
        0x0016_0bf8,
        relative_chase_camera_algorithm_update,
        "RelativeChaseCameraAlgorithm_Update",
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
    (
        0x0016_5938,
        chase_camera_algorithm_apply_shake,
        "ChaseCameraAlgorithm_ApplyShake",
    ),
    (
        0x0016_68b8,
        chase_camera_algorithm_rotate_by_pitch,
        "ChaseCameraAlgorithm_RotateByPitch",
    ),
    (
        0x0016_6f90,
        chase_camera_build_output_view,
        "ChaseCameraAlgorithm_BuildOutputView",
    ),
    (
        0x0016_2c78,
        chase_camera_update_chase_heading,
        "ChaseCameraAlgorithm_UpdateChaseHeading",
    ),
    (
        0x0015_ec98,
        game_camera_update_stream_view,
        "GameCamera_UpdateStreamView",
    ),
    (
        0x003a_9658,
        world_cache_streamer_set_view,
        "cWorldCache_StreamerSetView",
    ),
    (
        0x0016_6530,
        chase_camera_check_distance_reset_wrapper,
        "ChaseCameraAlgorithm_CheckDistanceReset_Wrapper",
    ),
    (
        0x0015_f780,
        rider_camera_target_v23,
        "RiderCameraTarget_v23",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_RESET_STATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;

    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_MAP_EVAL_FLOAT;

    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;

    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;

    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;

    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;

    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR_3;

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
pub(crate) mod g {}

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

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
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
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
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
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
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
                c.sq(2, c.g[SP][0] as u32);
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
                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                c.g[A0][0] = c.lh(at_3(c, V1, 0x98));
                                c.g[V0][0] = c.lw(at_3(c, V1, 0x9c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x00164900);
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
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
                                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                        c.g[A0][0] = c.lh(at_3(c, V0, 0x60));
                                        c.g[V1][0] = c.lw(at_3(c, V0, 0x64));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00164968);
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
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
                                            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                            c.g[A0][0] = c.lh(at_3(c, V0, 0x60));
                                            c.g[V1][0] = c.lw(at_3(c, V0, 0x64));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x0016499c);
                                            c.g[A0][0] =
                                                sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
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
                                        c.swc1(0, c.g[S0][0] as u32);
                                        c.swc1(20, at_3(c, S1, 0x23c));
                                        c.sw(at_3(c, S1, 0x234), 0u64);
                                        c.g[A1][0] = c.lw(at_3(c, S1, 0x30));
                                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                        c.g[A0][0] = c.lh(at_3(c, V1, 0x80));
                                        c.g[V0][0] = c.lw(at_3(c, V1, 0x84));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001649c8);
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        if call_ptr(c, t, 0x001649c8).is_err() {
                                            return;
                                        }
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A2][0] = 0x500000;
                                        c.lqc2(4, (c.g[A2][0] as u32).wrapping_sub(0xea0));
                                        c.lqc2(5, c.g[A0][0] as u32);
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
                                        c.lqc2(3, c.g[A0][0] as u32);
                                        c.lqc2(5, c.g[A0][0] as u32);
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.lqc2(3, at_3(c, S1, 0x110));
                                        c.lqc2(5, c.g[A0][0] as u32);
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 1);
                                        c.div_s(1, 1, 0);
                                        c.mfc1(3, 1);
                                        c.lqc2(4, c.g[A0][0] as u32);
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
                                        c.lqc2(3, c.g[A1][0] as u32);
                                        c.lqc2(5, c.g[A1][0] as u32);
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 0);
                                        c.lqc2(3, at_3(c, S1, 0x120));
                                        c.lqc2(5, c.g[A1][0] as u32);
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 1);
                                        c.div_s(1, 1, 0);
                                        c.mfc1(3, 1);
                                        c.lqc2(4, c.g[A1][0] as u32);
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
        c.lqc2(3, c.g[SP][0] as u32);
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
        let swinging: bool;

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
        // 50: the sideways push and the 50/s clearance, distances (not a
        // step; the tick-rate scan's 50.0 at 0x15ee9c).
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
            // 50 units along the contact normal, once on the first blocked
            // update: a distance, not a step (0x15f3d4).
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
                {
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
        {
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
                    {
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
                    {
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
        {
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
        // The eye keeps at least 50 from the target (max(f30, 50)): a
        // distance, not a step (0x164100).
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

/// `void RelativeChaseCameraAlgorithm_Update(Alg* self)` (0x160bf8):
/// build a horizontal frame from the gathered rider velocity, falling back
/// to the target's forward vector and then the global direction. Smooth
/// the focus against the previous focus (+0x3b0), place the eye on the
/// angle/radius orbit, then smooth its offset (+0x3a0). The first update
/// (+0x394 == 0) initializes both histories without blending.
pub fn relative_chase_camera_algorithm_update(c: &mut Cpu) {
    const FOCUS: u32 = 0x20;
    const EYE: u32 = 0x40;
    const ORBIT_ANGLE: u32 = 0x390;
    const HISTORY_INITIALIZED: u32 = 0x394;
    const PREVIOUS_EYE_OFFSET: u32 = 0x3a0;
    const PREVIOUS_FOCUS: u32 = 0x3b0;
    const GET_FOCUS: u32 = 8;
    const GET_FORWARD: u32 = 0x20;
    const INFO_DIRECTION: u32 = chase_info::DIR;
    const DIRECTION: u32 = 0x70;
    // The rig's fallback forward vector, then the horizontal side vector.
    const SIDE_OR_FORWARD: u32 = 0x80;
    const SAVED_RA: u32 = 0x100;
    const SAVED_WORLD_BASE: u32 = 0x110;
    const SAVED_CAMERA: u32 = 0x120;
    const SAVED_F20: u32 = 0x130;
    // Stack +0x90..+0xf0: reused VU vector-expression spills, kept in order.
    /// gp-0x6bac: 0.27136135, GatherRiderState f12 tuning argument.
    const GP_GATHER_F12: u32 = 0x6bac;
    /// gp-0x6ba8: 0.9, GatherRiderState f13 tuning argument.
    const GP_GATHER_F13: u32 = 0x6ba8;
    /// gp-0x6ba4: 0.29420722, GatherRiderState f14 tuning argument.
    const GP_GATHER_F14: u32 = 0x6ba4;
    /// gp-0x6ba0: 0.851393, GatherRiderState f15 tuning argument.
    const GP_GATHER_F15: u32 = 0x6ba0;
    /// gp-0x6b9c: 0.85, GatherRiderState f16 tuning argument.
    const GP_GATHER_F16: u32 = 0x6b9c;
    /// gp-0x6b98: 0.97, GatherRiderState f17 tuning argument.
    const GP_GATHER_F17: u32 = 0x6b98;
    /// gp-0x6b94: 0.1, GatherRiderState f18 tuning argument.
    const GP_GATHER_F18: u32 = 0x6b94;
    /// gp-0x6b90: 0.6, GatherRiderState f19 tuning argument.
    const GP_GATHER_F19: u32 = 0x6b90;
    /// gp-0x6b8c: 0.001, minimum usable horizontal direction component.
    const GP_MIN_DIRECTION_COMPONENT: u32 = 0x6b8c;
    /// gp-0x6b88: 0.64, previous focus share.
    const GP_FOCUS_RETAINED: u32 = 0x6b88;
    /// gp-0x6b84: 0.36, new focus share.
    const GP_FOCUS_INCOMING: u32 = 0x6b84;
    /// gp-0x6b80: 385.9184, eye orbit radius.
    const GP_ORBIT_RADIUS: u32 = 0x6b80;
    /// gp-0x6b7c: 53.241573, eye height above focus.
    const GP_EYE_HEIGHT: u32 = 0x6b7c;
    /// gp-0x6b78: 0.97, retained eye-offset share at 60 Hz.
    const GP_EYE_OFFSET_RETAINED: u32 = 0x6b78;
    /// gp-0x6b74: 0.029999971, incoming eye-offset share at 60 Hz.
    const GP_EYE_OFFSET_INCOMING: u32 = 0x6b74;
    c.cycles += 23;
    let frame = Frame::open(c, 0x140);
    c.lwc1(12, r32(c, GP).wrapping_sub(GP_GATHER_F12));
    frame.save(c, S0, SAVED_CAMERA);
    c.g[A1][0] = c.g[SP][0];
    c.lwc1(13, r32(c, GP).wrapping_sub(GP_GATHER_F13));
    c.lwc1(14, r32(c, GP).wrapping_sub(GP_GATHER_F14));
    c.lwc1(15, r32(c, GP).wrapping_sub(GP_GATHER_F15));
    c.lwc1(16, r32(c, GP).wrapping_sub(GP_GATHER_F16));
    c.lwc1(17, r32(c, GP).wrapping_sub(GP_GATHER_F17));
    c.lwc1(18, r32(c, GP).wrapping_sub(GP_GATHER_F18));
    c.lwc1(19, r32(c, GP).wrapping_sub(GP_GATHER_F19));
    frame.save_fpr(c, 20, SAVED_F20);
    frame.save(c, S1, SAVED_WORLD_BASE);
    frame.save_ra(c, SAVED_RA);
    c.g[RA][0] = sx(0x00160c38);
    c.g[S0][0] = c.g[A0][0];
    if call_ptr(c, 0x00162568, 0x00160c38).is_err() {
        return;
    }
    c.lwc1(20, r32(c, GP).wrapping_sub(GP_MIN_DIRECTION_COMPONENT));
    c.lwc1(0, frame.at(c, INFO_DIRECTION));
    c.abs_s(0, 0);
    c.c_lt_s(20, 0);
    let moving_x = c.fcc();
    c.lq(2, frame.at(c, INFO_DIRECTION));
    let moving = if moving_x {
        true
    } else {
        c.cycles += 6;
        c.lwc1(0, frame.at(c, INFO_DIRECTION + 4));
        c.abs_s(0, 0);
        c.c_lt_s(20, 0);
        if !c.fcc() {
            c.g[A2][0] = Addr(r32(c, S0).wrapping_add(chase::RIG)).lw(c);
        }
        c.fcc()
    };
    if moving {
        c.cycles += 2;
        c.sq(2, frame.at(c, DIRECTION));
    } else {
        c.cycles += 14;
        c.g[A0][0] = sx(frame.at(c, SIDE_OR_FORWARD));
        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
        c.g[A1][0] = Addr(r32(c, V0).wrapping_add(GET_FORWARD)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(GET_FORWARD + 4)).lw(c);
        let target_method = r32(c, V1);
        c.g[RA][0] = sx(0x00160c8c);
        c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
        if call_ptr(c, target_method, 0x00160c8c).is_err() {
            return;
        }
        c.lq(2, frame.at(c, SIDE_OR_FORWARD));
        c.sq(2, frame.at(c, DIRECTION));
        c.lwc1(0, frame.at(c, DIRECTION));
        c.abs_s(0, 0);
        c.c_lt_s(0, 20);
        let forward_x = !c.fcc();
        c.lwc1(0, frame.at(c, DIRECTION + 4));
        if !forward_x {
            c.cycles += 5;
            c.abs_s(0, 0);
            c.c_lt_s(0, 20);
            c.g[V0][0] = 0x500000;
            if c.fcc() {
                c.cycles += 3;
                // 0x4ff140: global fallback direction.
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xec0));
                c.lq(3, c.g[V0][0] as u32);
                c.sq(3, frame.at(c, DIRECTION));
            }
        }
    }
    // Normalize the horizontal direction and its perpendicular, then get the focus.
    c.cycles += 68;
    c.sw(frame.at(c, DIRECTION + 8), 0u64);
    c.g[S1][0] = 0x500000;
    // 0x4ff160: world up.
    c.lqc2(4, r32(c, S1).wrapping_sub(0xea0));
    c.lqc2(5, frame.at(c, DIRECTION));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, frame.at(c, 0x90));
    c.g[A0][0] = sx(frame.at(c, 0xa0));
    c.lq(2, frame.at(c, 0x90));
    c.sq(2, frame.at(c, SIDE_OR_FORWARD));
    c.lqc2(3, frame.at(c, DIRECTION));
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
    c.lqc2(4, frame.at(c, DIRECTION));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 4, Src::Q);
    c.sqc2(5, frame.at(c, DIRECTION));
    c.lqc2(3, frame.at(c, SIDE_OR_FORWARD));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    c.mtc1(3, 1);
    c.mfc1(2, 1);
    c.qmtc2(2, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, frame.at(c, SIDE_OR_FORWARD));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 4, Src::Q);
    c.sqc2(5, frame.at(c, SIDE_OR_FORWARD));
    c.g[A2][0] = Addr(r32(c, S0).wrapping_add(chase::RIG)).lw(c);
    c.g[V0][0] = Addr(r32(c, A2)).lw(c);
    c.g[A1][0] = Addr(r32(c, V0).wrapping_add(GET_FOCUS)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(GET_FOCUS + 4)).lw(c);
    let target_method = r32(c, V1);
    c.g[RA][0] = sx(0x00160d98);
    c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
    if call_ptr(c, target_method, 0x00160d98).is_err() {
        return;
    }
    c.mtc1(0, 0);
    c.mfc1(3, 0);
    // 0x4ff160: world up.
    c.lqc2(4, r32(c, S1).wrapping_sub(0xea0));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0xc0));
    c.lq(2, frame.at(c, 0xc0));
    c.sq(2, frame.at(c, 0xb0));
    c.lqc2(3, frame.at(c, 0xa0));
    c.lqc2(4, frame.at(c, 0xb0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0xc0));
    c.lq(2, frame.at(c, 0xc0));
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(HISTORY_INITIALIZED)).lw(c);
    c.sq(2, r32(c, S0).wrapping_add(FOCUS));
    let initialized = c.g[V1][0] != 0;
    c.sq(2, frame.at(c, 0x90));
    if initialized {
        // Focus = retained previous focus + new target focus.
        c.cycles += 23;
        c.lwc1(0, r32(c, GP).wrapping_sub(GP_FOCUS_RETAINED));
        c.mfc1(2, 0);
        c.lqc2(4, r32(c, S0).wrapping_add(PREVIOUS_FOCUS));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0xb0));
        c.lwc1(1, r32(c, GP).wrapping_sub(GP_FOCUS_INCOMING));
        c.lq(2, frame.at(c, 0xb0));
        c.sq(2, frame.at(c, 0xa0));
        c.mfc1(3, 1);
        c.lqc2(4, r32(c, S0).wrapping_add(FOCUS));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0xc0));
        c.lq(2, frame.at(c, 0xc0));
        c.sq(2, frame.at(c, 0xb0));
        c.lqc2(3, frame.at(c, 0xa0));
        c.lqc2(4, frame.at(c, 0xb0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0xc0));
        c.lq(2, frame.at(c, 0xc0));
        c.sq(2, r32(c, S0).wrapping_add(FOCUS));
        c.sq(2, frame.at(c, 0x90));
    }
    // Place the eye on the orbit in the horizontal frame.
    c.cycles += 52;
    c.lq(2, r32(c, S0).wrapping_add(FOCUS));
    c.lwc1(12, r32(c, S0).wrapping_add(ORBIT_ANGLE));
    c.lwc1(20, r32(c, GP).wrapping_sub(GP_ORBIT_RADIUS));
    c.g[RA][0] = sx(0x00160e4c);
    c.sq(2, r32(c, S0).wrapping_add(PREVIOUS_FOCUS));
    if call_ptr(c, 0x0031c040, 0x00160e4c).is_err() {
        return;
    }
    c.mul_s(0, 0, 20);
    c.mfc1(3, 0);
    c.lqc2(4, frame.at(c, DIRECTION));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0xd0));
    c.lq(2, frame.at(c, 0xd0));
    c.sq(2, frame.at(c, 0xc0));
    c.lqc2(3, r32(c, S0).wrapping_add(FOCUS));
    c.lqc2(4, frame.at(c, 0xc0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0xd0));
    c.lq(2, frame.at(c, 0xd0));
    c.lwc1(12, r32(c, S0).wrapping_add(ORBIT_ANGLE));
    c.g[RA][0] = sx(0x00160e8c);
    c.sq(2, frame.at(c, 0xb0));
    if call_ptr(c, 0x0031bf60, 0x00160e8c).is_err() {
        return;
    }
    c.mul_s(0, 0, 20);
    c.lwc1(1, r32(c, GP).wrapping_sub(GP_EYE_HEIGHT));
    c.mfc1(3, 0);
    c.lqc2(4, frame.at(c, SIDE_OR_FORWARD));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0xe0));
    c.lq(2, frame.at(c, 0xe0));
    c.sq(2, frame.at(c, 0xd0));
    c.lqc2(3, frame.at(c, 0xb0));
    c.lqc2(4, frame.at(c, 0xd0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0xe0));
    c.lq(2, frame.at(c, 0xe0));
    c.sq(2, frame.at(c, 0xa0));
    c.mfc1(3, 1);
    // 0x4ff160: world up.
    c.lqc2(4, r32(c, S1).wrapping_sub(0xea0));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame.at(c, 0xf0));
    c.lq(2, frame.at(c, 0xf0));
    c.sq(2, frame.at(c, 0xe0));
    c.lqc2(3, frame.at(c, 0xa0));
    c.lqc2(4, frame.at(c, 0xe0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0xf0));
    c.lq(2, frame.at(c, 0xf0));
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(HISTORY_INITIALIZED)).lw(c);
    c.sq(2, r32(c, S0).wrapping_add(EYE));
    let initialized = c.g[V1][0] != 0;
    c.sq(2, frame.at(c, 0x90));
    if initialized {
        // Blend the eye offset against its history, preserving the focus.
        c.cycles += 35;
        c.lqc2(3, r32(c, S0).wrapping_add(FOCUS));
        c.lqc2(4, r32(c, S0).wrapping_add(EYE));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0xa0));
        c.lwc1(0, r32(c, GP).wrapping_sub(GP_EYE_OFFSET_RETAINED));
        c.lq(2, frame.at(c, 0xa0));
        c.lwc1(1, r32(c, GP).wrapping_sub(GP_EYE_OFFSET_INCOMING));
        c.sq(2, frame.at(c, 0x90));
        c.mfc1(3, 0);
        c.lqc2(4, r32(c, S0).wrapping_add(PREVIOUS_EYE_OFFSET));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0xc0));
        c.lq(2, frame.at(c, 0xc0));
        c.sq(2, frame.at(c, 0xb0));
        c.mfc1(3, 1);
        c.lqc2(4, frame.at(c, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0xd0));
        c.lq(2, frame.at(c, 0xd0));
        c.sq(2, frame.at(c, 0xc0));
        c.lqc2(3, frame.at(c, 0xb0));
        c.lqc2(4, frame.at(c, 0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0xd0));
        c.lq(2, frame.at(c, 0xd0));
        c.sq(2, frame.at(c, 0xa0));
        c.lqc2(3, r32(c, S0).wrapping_add(FOCUS));
        c.lqc2(4, frame.at(c, 0xa0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0xc0));
        c.lq(2, frame.at(c, 0xc0));
        c.sq(2, r32(c, S0).wrapping_add(EYE));
        c.sq(2, frame.at(c, 0xb0));
    }
    // Store the eye offset and mark both histories initialized.
    c.cycles += 15;
    c.lqc2(3, r32(c, S0).wrapping_add(FOCUS));
    c.lqc2(4, r32(c, S0).wrapping_add(EYE));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0xa0));
    c.g[V1][0] = 0x1;
    c.lq(2, frame.at(c, 0xa0));
    c.sw(r32(c, S0).wrapping_add(HISTORY_INITIALIZED), c.g[V1][0]);
    c.sq(2, r32(c, S0).wrapping_add(PREVIOUS_EYE_OFFSET));
    frame.restore(c, S0, SAVED_CAMERA);
    frame.restore(c, S1, SAVED_WORLD_BASE);
    frame.restore_ra(c, SAVED_RA);
    frame.restore_fpr(c, 20, SAVED_F20);
    c.sq(2, frame.at(c, 0x90));
    frame.close(c);
}

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
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
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
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
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
    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
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
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
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

pub(crate) mod chase_view {

    /// The camera's look-at offset (a vec4 transformed by the base matrix).
    pub const OFFSET: u32 = 0x40;

    /// Set to 1.0 each build.
    pub const SCALE: u32 = 0x4c;

    /// The yaw (`+0x50`) and pitch (`+0x54`) angles, applied negated.
    pub const YAW: u32 = 0x50;

    pub const PITCH: u32 = 0x54;

    /// The output: position (`+0x60`) and orientation quaternion (`+0x70`).
    pub const OUT_POS: u32 = 0x60;

    pub const OUT_QUAT: u32 = 0x70;

    /// The base matrix and the pitch and yaw axes.
    pub const BASE: u32 = 0x004f_f1a0;

    pub const PITCH_AXIS: u32 = 0x004f_f150;

    pub const YAW_AXIS: u32 = 0x004f_f160;

    /// The `{0, 0, 0, 1}`-style row the rotations' last row is copied from.
    pub const LAST_ROW: u32 = 0x004f_f130;

    /// `void Math_SinCos(float a, float* sin, float* cos)`.
    pub const SIN_COS: u32 = 0x0031_be50;

    /// `void Mat4_ToPosQuat(Mat4* m, PosQuat* out)`.
    pub const TO_POS_QUAT: u32 = 0x0031_b748;
}

pub(crate) mod shake {

    pub const POS: u32 = 0x20;

    pub const TARGET: u32 = 0x40;

    /// Seconds of shake left.
    pub const TIME_LEFT: u32 = 0x2b4;

    /// Last frame's "shake requested" argument.
    pub const LAST_REQUEST: u32 = 0x2ec;
}

/// `lqc2 vf1..vf4, src; sqc2 vf1..vf4, dst`: a 4x4 matrix copy through VU0.
pub(crate) fn chase_copy_matrix(c: &mut Cpu, dst: u32, src: u32) {
    for i in 0..4 {
        c.lqc2(1 + i, src.wrapping_add(0x10 * i as u32));
    }
    for i in 0..4 {
        c.sqc2(1 + i, dst.wrapping_add(0x10 * i as u32));
    }
}

/// `dst = a * b` on VU0 (rows of `b` combined through the rows of `a`):
/// `vf4..7 = a`, `vf8..11 = b`, `vf12..15` the product.
pub(crate) fn chase_mul_matrix(c: &mut Cpu, dst: u32, a: u32, b: u32) {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    for i in 0..4 {
        c.lqc2(4 + i, a.wrapping_add(0x10 * i as u32));
    }
    for i in 0..4 {
        c.lqc2(8 + i, b.wrapping_add(0x10 * i as u32));
    }
    for i in 0..4 {
        c.vu0().mula(0b1111, 4, Src::Bc(8 + i, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8 + i, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8 + i, Z));
        c.vu0().madd(0b1111, 12 + i, 7, Src::Bc(8 + i, W));
    }
    for i in 0..4 {
        c.sqc2(12 + i, dst.wrapping_add(0x10 * i as u32));
    }
}

/// One axis rotation of `ChaseCameraAlgorithm_BuildOutputView`: with the
/// angle already in f12, save the current matrix (at sp) to sp+0x40, get
/// sin/cos into `sp+sincos`, build the axis-angle rotation matrix at sp
/// (its last row copied from the quadword the original parks at sp+0x80),
/// then `sp = rotation * saved`.
pub(crate) fn chase_axis_rotation(
    c: &mut Cpu,
    axis: u32,
    sincos: u32,
    ret: u32,
) -> crate::call::Flow {
    use self::chase_view::*;
    use crate::ps2f::FpuF32;
    let base = sp(c);
    c.g[S1][0] = 0x0050_0000;
    c.g[S2][0] = sx(axis);
    c.g[S5][0] = sx(base);
    c.f[12] = (-FpuF32(c.f[12])).0;
    chase_copy_matrix(c, base.wrapping_add(0x40), base);
    c.g[S3][0] = sx(LAST_ROW);
    c.lq(V1, base.wrapping_add(0x70));
    c.lq(V0, LAST_ROW);
    c.g[A0][0] = sx(base.wrapping_add(sincos));
    c.g[A1][0] = sx(base.wrapping_add(sincos + 4));
    c.sq(V1, base.wrapping_add(0x80));
    c.sq(V0, base.wrapping_add(0x70));
    call_ptr(c, SIN_COS, ret)?;

    let f = |c: &mut Cpu, n: usize| FpuF32(c.f[n]);
    let cos = Addr(base.wrapping_add(sincos + 4)).read_fpu(c);
    c.f[11] = cos.0;
    let z = Addr(axis).off(8).read_fpu(c);
    c.f[3] = z.0;
    let t = f(c, 20) - cos; // 1 - cos
    c.f[1] = t.0;
    let x = Addr(axis).read_fpu(c);
    c.f[0] = x.0;
    let y = Addr(axis).off(4).read_fpu(c);
    c.f[7] = y.0;
    let sin = Addr(base.wrapping_add(sincos)).read_fpu(c);
    c.f[8] = sin.0;
    let tz = t * z;
    c.lq(V0, base.wrapping_add(0x80));
    let tx = t * x;
    let ty = t * y;
    let sz = sin * z;
    let sx_ = sin * x;
    let tzz = tz * z;
    let txz = tx * z;
    let tzx = tz * x;
    let txx = tx * x;
    let tyz = ty * z;
    let tyx = ty * x;
    let sy = sin * y;
    let tzy = tz * y;
    let txy = tx * y;
    let tyy = ty * y;
    // Rows 0..2 of the rotation, column 3 zero; row 3 from sp+0x80.
    let m = [
        (0x00, txx + cos, 6),
        (0x04, tyx - sz, 0),
        (0x08, tzx + sy, 5),
        (0x10, txy + sz, 4),
        (0x14, tyy + cos, 1),
        (0x18, tzy - sx_, 2),
        (0x20, txz - sy, 10),
        (0x24, tyz + sx_, 3),
        (0x28, tzz + cos, 9),
    ];
    for &(off, v, reg) in &m {
        c.f[reg] = v.0;
        Addr(base.wrapping_add(off)).write_fpu(c, v);
    }
    c.f[8] = sy.0;
    c.f[12] = sx_.0;
    c.f[13] = sz.0;
    for off in [0x0c, 0x1c, 0x2c, 0x30, 0x34, 0x38] {
        Addr(base.wrapping_add(off)).sw(c, 0);
    }
    let one = f(c, 20);
    Addr(base.wrapping_add(0x3c)).write_fpu(c, one);
    c.sq(V0, base.wrapping_add(0x30));

    chase_mul_matrix(c, base, base, base.wrapping_add(0x40));
    Ok(())
}

/// `void ChaseCameraAlgorithm_BuildOutputView(ChaseCameraAlgorithm* self)`
/// (0x166f90): the chase camera's output transform. Start from the base
/// matrix with its translation replaced by the transformed look-at offset,
/// rotate by -pitch about the pitch axis and by -yaw about the yaw axis,
/// split the result into position and quaternion (`Mat4_ToPosQuat`) and
/// store them with the quaternion conjugated.
pub fn chase_camera_build_output_view(c: &mut Cpu) {
    use self::chase_view::*;
    use crate::ps2f::FpuF32;
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    guest(c, |c| {
        c.cycles += 260;
        let frame = Frame::open(c, 0x120);
        c.g[V0][0] = sx(BASE);
        frame.save_fpr(c, 20, 0x110);
        frame.save(c, S0, 0x100);
        c.g[AT][0] = 0x3f80_0000;
        c.f[20] = 0x3f80_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0xf0);
        let this = Addr(r32(c, S0));
        c.g[V1][0] = sx(this.off(OFFSET).0);
        frame.save(c, S2, 0xe0);
        frame.save(c, S3, 0xd0);
        frame.save(c, S4, 0xc0);
        frame.save(c, S5, 0xb0);
        frame.save_ra(c, 0xa0);
        let one = FpuF32(c.f[20]);
        this.off(SCALE).write_fpu(c, one);
        let base = sp(c);
        chase_copy_matrix(c, base, BASE);
        // Translation row = base matrix * offset.
        for i in 0..4 {
            c.lqc2(4 + i, base.wrapping_add(0x10 * i as u32));
        }
        c.lqc2(8, this.off(OFFSET).0);
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, base.wrapping_add(0x30));

        c.f[12] = this.off(PITCH).read_fpu(c).0;
        c.g[S4][0] = sx(base.wrapping_add(0x40));
        chase_axis_rotation(c, PITCH_AXIS, 0x90, 0x0016_7074)?;
        c.f[12] = this.off(YAW).read_fpu(c).0;
        chase_axis_rotation(c, YAW_AXIS, 0x98, 0x0016_71f4)?;

        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = sx(base);
        call_ptr(c, TO_POS_QUAT, 0x0016_7330)?;
        let base = sp(c);
        c.lq(V1, base.wrapping_add(0x50));
        c.lq(V0, base.wrapping_add(0x40));
        c.sq(V1, this.off(OUT_QUAT).0);
        c.sq(V0, this.off(OUT_POS).0);
        // Conjugate the quaternion (negate x, y, z).
        for i in 0..3u32 {
            let q = -this.off(OUT_QUAT + 4 * i).read_fpu(c);
            c.f[i as usize] = q.0;
            Addr(base.wrapping_add(0x50 + 4 * i)).write_fpu(c, q);
        }
        let w = this.off(OUT_QUAT + 0xc).read_fpu(c);
        c.f[3] = w.0;
        Addr(base.wrapping_add(0x5c)).write_fpu(c, w);
        c.lq(V0, base.wrapping_add(0x50));
        c.sq(V0, this.off(OUT_QUAT).0);
        frame.restore(c, S0, 0x100);
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.restore_fpr(c, 20, 0x110);
        c.sq(V0, base.wrapping_add(0x40));
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------- ChaseCameraAlgorithm_RotateByPitch

/// `void ChaseCameraAlgorithm_RotateByPitch(ChaseCameraAlgorithm* this,
/// Vec4* out)` (0x1668b8): the chase camera's look direction. Starting from
/// the identity (0x4ff1a0), rotates by `-this->+0x54` about the axis at
/// 0x4ff150 and then by `-this->+0x50` about the axis at 0x4ff160 (each an
/// axis-angle matrix from `Math_SinCos`, `R = (1-c) a a^T + c I + s [a]x`,
/// multiplied on with VU0), and writes the first row to `out`.
///
/// Inlined `vu0` matrix copies and multiplies: the VU0 macro ops, stack
/// layout (the matrix at sp, its copy at sp+0x40, the rotation at
/// sp+0x40..0x70) and FPU temporaries are the original's.
pub fn chase_camera_algorithm_rotate_by_pitch(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x130);
        c.cycles += 233;
        c.g[V0][0] = 0x500000; // 0x4ff1a0: the identity matrix
        frame.save(c, S5, 0xc0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
        frame.save(c, S6, 0xb0);
        c.g[S5][0] = c.g[A0][0];
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S0, 0x110);
        frame.save(c, S1, 0x100);
        frame.save(c, S2, 0xf0);
        frame.save(c, S3, 0xe0);
        frame.save(c, S4, 0xd0);
        frame.save_ra(c, 0xa0);
        frame.save_fpr(c, 20, 0x120);
        c.lqc2(1, c.g[V0][0] as u32);
        c.lqc2(2, crate::game::render::particle::at(c, V0, 0x10));
        c.lqc2(3, crate::game::render::particle::at(c, V0, 0x20));
        c.lqc2(4, crate::game::render::particle::at(c, V0, 0x30));
        c.sqc2(1, stk(c, 0));
        c.sqc2(2, stk(c, 0x10));
        c.sqc2(3, stk(c, 0x20));
        c.sqc2(4, stk(c, 0x30));
        c.lwc1(12, crate::game::render::particle::at(c, S5, 0x54));
        c.g[S0][0] = 0x500000;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xeb0)); // 0x4ff150: the first axis
        c.g[S4][0] = c.g[SP][0];
        c.neg_s(12, 12);
        c.g[S3][0] = sx(stk(c, 0x40));
        c.lqc2(1, c.g[S4][0] as u32);
        c.lqc2(2, crate::game::render::particle::at(c, S4, 0x10));
        c.lqc2(3, crate::game::render::particle::at(c, S4, 0x20));
        c.lqc2(4, crate::game::render::particle::at(c, S4, 0x30));
        c.sqc2(1, c.g[S3][0] as u32);
        c.sqc2(2, crate::game::render::particle::at(c, S3, 0x10));
        c.sqc2(3, crate::game::render::particle::at(c, S3, 0x20));
        c.sqc2(4, crate::game::render::particle::at(c, S3, 0x30));
        c.g[S2][0] = 0x500000;
        c.lq(3, stk(c, 0x70));
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xed0));
        c.g[A0][0] = sx(stk(c, 0x90));
        c.lq(2, c.g[S2][0] as u32);
        c.g[A1][0] = sx(stk(c, 0x94));
        c.sq(3, stk(c, 0x80));
        c.sq(2, stk(c, 0x70));
        call_ptr(c, 0x0031be50, 0x0016696c)?; // Math_SinCos(-yaw)
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.lwc1(11, stk(c, 0x94));
        c.lwc1(3, crate::game::render::particle::at(c, S1, 0x8));
        c.sub_s(1, 20, 11);
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_sub(0xeb0));
        c.lwc1(7, crate::game::render::particle::at(c, S1, 0x4));
        c.lwc1(8, stk(c, 0x90));
        c.mul_s(2, 1, 3);
        c.lq(2, stk(c, 0x80));
        c.mul_s(4, 1, 0);
        c.sw(stk(c, 0x30), 0u64);
        c.mul_s(1, 1, 7);
        c.sw(stk(c, 0x34), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(stk(c, 0x38), 0u64);
        c.mul_s(12, 8, 0);
        c.swc1(20, stk(c, 0x3c)); // the rotation's w = 1
        c.mul_s(9, 2, 3);
        c.sq(2, stk(c, 0x30));
        c.mul_s(10, 4, 3);
        c.sw(stk(c, 0xc), 0u64);
        c.mul_s(5, 2, 0);
        c.sw(stk(c, 0x1c), 0u64);
        c.mul_s(6, 4, 0);
        c.sw(stk(c, 0x2c), 0u64);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 7);
        c.mul_s(2, 2, 7);
        c.mul_s(4, 4, 7);
        c.mul_s(1, 1, 7);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, stk(c, 0x8));
        c.add_s(6, 6, 11);
        c.swc1(0, stk(c, 0x4));
        c.add_s(4, 4, 13);
        c.swc1(2, stk(c, 0x18));
        c.sub_s(10, 10, 8);
        c.swc1(9, stk(c, 0x28));
        c.add_s(1, 1, 11);
        c.swc1(6, stk(c, 0));
        c.add_s(3, 3, 12);
        c.swc1(4, stk(c, 0x10));
        c.swc1(10, stk(c, 0x20));
        c.swc1(1, stk(c, 0x14));
        c.swc1(3, stk(c, 0x24));
        c.lqc2(4, c.g[S4][0] as u32);
        c.lqc2(5, crate::game::render::particle::at(c, S4, 0x10));
        c.lqc2(6, crate::game::render::particle::at(c, S4, 0x20));
        c.lqc2(7, crate::game::render::particle::at(c, S4, 0x30));
        c.lqc2(8, c.g[S3][0] as u32);
        c.lqc2(9, crate::game::render::particle::at(c, S3, 0x10));
        c.lqc2(10, crate::game::render::particle::at(c, S3, 0x20));
        c.lqc2(11, crate::game::render::particle::at(c, S3, 0x30));
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
        c.sqc2(12, c.g[S4][0] as u32);
        c.sqc2(13, crate::game::render::particle::at(c, S4, 0x10));
        c.sqc2(14, crate::game::render::particle::at(c, S4, 0x20));
        c.sqc2(15, crate::game::render::particle::at(c, S4, 0x30));
        c.lwc1(12, crate::game::render::particle::at(c, S5, 0x50));
        c.g[S0][0] = 0x500000;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xea0)); // 0x4ff160: the second axis
        c.g[S4][0] = c.g[SP][0];
        c.neg_s(12, 12);
        c.lqc2(1, c.g[S4][0] as u32);
        c.lqc2(2, crate::game::render::particle::at(c, S4, 0x10));
        c.lqc2(3, crate::game::render::particle::at(c, S4, 0x20));
        c.lqc2(4, crate::game::render::particle::at(c, S4, 0x30));
        c.sqc2(1, c.g[S3][0] as u32);
        c.sqc2(2, crate::game::render::particle::at(c, S3, 0x10));
        c.sqc2(3, crate::game::render::particle::at(c, S3, 0x20));
        c.sqc2(4, crate::game::render::particle::at(c, S3, 0x30));
        c.lq(2, c.g[S2][0] as u32);
        c.g[A0][0] = sx(stk(c, 0x98));
        c.lq(3, stk(c, 0x70));
        c.g[A1][0] = sx(stk(c, 0x9c));
        c.sq(2, stk(c, 0x70));
        c.sq(3, stk(c, 0x80));
        call_ptr(c, 0x0031be50, 0x00166af4)?; // Math_SinCos(-pitch)
        c.lwc1(11, stk(c, 0x9c));
        c.lwc1(3, crate::game::render::particle::at(c, S1, 0x8));
        c.sub_s(1, 20, 11);
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_sub(0xea0));
        c.lwc1(7, crate::game::render::particle::at(c, S1, 0x4));
        c.lwc1(8, stk(c, 0x98));
        c.mul_s(2, 1, 3);
        c.lq(2, stk(c, 0x80));
        c.mul_s(4, 1, 0);
        c.swc1(20, stk(c, 0x3c)); // the rotation's w = 1
        c.mul_s(1, 1, 7);
        c.sw(stk(c, 0x30), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(stk(c, 0x34), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(stk(c, 0x38), 0u64);
        c.mul_s(9, 2, 3);
        c.sq(2, stk(c, 0x30));
        c.mul_s(10, 4, 3);
        c.sw(stk(c, 0xc), 0u64);
        c.mul_s(5, 2, 0);
        c.sw(stk(c, 0x1c), 0u64);
        c.mul_s(6, 4, 0);
        c.sw(stk(c, 0x2c), 0u64);
        c.mul_s(3, 1, 3);
        c.mul_s(0, 1, 0);
        c.mul_s(8, 8, 7);
        c.mul_s(2, 2, 7);
        c.mul_s(4, 4, 7);
        c.mul_s(1, 1, 7);
        c.add_s(5, 5, 8);
        c.sub_s(0, 0, 13);
        c.sub_s(2, 2, 12);
        c.add_s(9, 9, 11);
        c.swc1(5, stk(c, 0x8));
        c.add_s(6, 6, 11);
        c.swc1(0, stk(c, 0x4));
        c.add_s(4, 4, 13);
        c.swc1(2, stk(c, 0x18));
        c.sub_s(10, 10, 8);
        c.swc1(9, stk(c, 0x28));
        c.add_s(1, 1, 11);
        c.swc1(6, stk(c, 0));
        c.add_s(3, 3, 12);
        c.swc1(4, stk(c, 0x10));
        c.swc1(10, stk(c, 0x20));
        c.swc1(1, stk(c, 0x14));
        c.swc1(3, stk(c, 0x24));
        c.lqc2(4, c.g[S4][0] as u32);
        c.lqc2(5, crate::game::render::particle::at(c, S4, 0x10));
        c.lqc2(6, crate::game::render::particle::at(c, S4, 0x20));
        c.lqc2(7, crate::game::render::particle::at(c, S4, 0x30));
        c.lqc2(8, c.g[S3][0] as u32);
        c.lqc2(9, crate::game::render::particle::at(c, S3, 0x10));
        c.lqc2(10, crate::game::render::particle::at(c, S3, 0x20));
        c.lqc2(11, crate::game::render::particle::at(c, S3, 0x30));
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
        c.sqc2(12, c.g[S4][0] as u32);
        c.sqc2(13, crate::game::render::particle::at(c, S4, 0x10));
        c.sqc2(14, crate::game::render::particle::at(c, S4, 0x20));
        c.sqc2(15, crate::game::render::particle::at(c, S4, 0x30));
        c.lq(2, stk(c, 0));
        frame.restore(c, S0, 0x110);
        c.sq(2, c.g[S6][0] as u32);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore(c, S4, 0xd0);
        frame.restore(c, S5, 0xc0);
        frame.restore(c, S6, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.restore_fpr(c, 20, 0x120);
        c.sq(2, stk(c, 0x40));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ChaseCameraAlgorithm_ApplyShake

/// `u32 Rand_NextMain()` (0x3177f0): the main random stream.
pub(crate) const RAND_NEXT_MAIN: u32 = 0x0031_77f0;

/// One of the six shake channels (x/y/z, slow and fast): a random target
/// value is re-drawn each time its timer passes its period, and the output
/// eases from the previous to the current target over the period.
pub(crate) struct ShakeChannel {
    timer: u32,
    period: u32,
    prev: u32,
    cur: u32,
    /// FPU registers: amplitude, period range and period base.
    amp: usize,
    range: usize,
    base: usize,
    /// The block charge and the two `Rand_NextMain` return addresses.
    cycles: u64,
    rets: [u32; 2],
}

pub(crate) const SHAKE_CHANNELS: [ShakeChannel; 6] = [
    ShakeChannel {
        timer: 0x278,
        period: 0x254,
        prev: 0x26c,
        cur: 0x260,
        amp: 22,
        range: 26,
        base: 24,
        cycles: 29,
        rets: [0x0016_5a54, 0x0016_5a90],
    },
    ShakeChannel {
        timer: 0x27c,
        period: 0x258,
        prev: 0x270,
        cur: 0x264,
        amp: 22,
        range: 26,
        base: 24,
        cycles: 30,
        rets: [0x0016_5ae0, 0x0016_5b1c],
    },
    ShakeChannel {
        timer: 0x280,
        period: 0x25c,
        prev: 0x274,
        cur: 0x268,
        amp: 23,
        range: 26,
        base: 24,
        cycles: 29,
        rets: [0x0016_5b70, 0x0016_5bac],
    },
    ShakeChannel {
        timer: 0x2a8,
        period: 0x284,
        prev: 0x29c,
        cur: 0x290,
        amp: 22,
        range: 24,
        base: 25,
        cycles: 29,
        rets: [0x0016_5c0c, 0x0016_5c48],
    },
    ShakeChannel {
        timer: 0x2ac,
        period: 0x288,
        prev: 0x2a0,
        cur: 0x294,
        amp: 22,
        range: 24,
        base: 25,
        cycles: 30,
        rets: [0x0016_5c98, 0x0016_5cd4],
    },
    ShakeChannel {
        timer: 0x2b0,
        period: 0x28c,
        prev: 0x2a4,
        cur: 0x298,
        amp: 23,
        range: 24,
        base: 25,
        cycles: 29,
        rets: [0x0016_5d20, 0x0016_5d5c],
    },
];

/// The channel's timer passed its period: new period `base + range *
/// (rand - 1)`, the current target becomes the previous one, new target
/// `-amp + 2 amp (rand - 1)`; the timer keeps the overshoot.
pub(crate) fn shake_retarget(c: &mut Cpu, ch: &ShakeChannel) -> Flow {
    let cam = r32(c, S2);
    c.cycles += ch.cycles;
    c.g[S0][0] = 0x7f_0000;
    call_ptr(c, RAND_NEXT_MAIN, ch.rets[0])?;
    c.g[S0][0] |= 0xffff;
    c.g[S1][0] = 0x3f80_0000;
    c.g[V0][0] &= c.g[S0][0];
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 21);
    c.g[V0][0] |= c.g[S1][0];
    c.lwc1(1, cam.wrapping_add(ch.cur));
    c.mtc1(V0, 2);
    c.swc1(1, cam.wrapping_add(ch.prev));
    c.neg_s(20, ch.amp);
    c.sub_s(0, 2, 21);
    c.mul_s(0, ch.range, 0);
    c.add_s(0, ch.base, 0);
    c.swc1(0, cam.wrapping_add(ch.period));
    call_ptr(c, RAND_NEXT_MAIN, ch.rets[1])?;
    c.g[V0][0] &= c.g[S0][0];
    c.sub_s(2, ch.amp, 20);
    c.g[V0][0] |= c.g[S1][0];
    c.lwc1(1, cam.wrapping_add(ch.timer));
    c.mtc1(V0, 0);
    c.sub_s(21, 0, 21);
    c.lwc1(0, cam.wrapping_add(ch.period));
    c.mul_s(2, 2, 21);
    c.sub_s(1, 1, 0);
    c.add_s(20, 20, 2);
    c.swc1(1, cam.wrapping_add(ch.timer));
    c.swc1(20, cam.wrapping_add(ch.cur));
    Ok(())
}

/// The channel's ease weight into f`out`: 0 before the timer starts (f`zero`
/// = 0), 1 past the period, else smoothstep `3t^2 - 2t^3` of `t = timer /
/// period` capped at 1.
pub(crate) fn shake_ease(c: &mut Cpu, ch: &ShakeChannel, out: usize, zero: usize) {
    let cam = r32(c, S2);
    c.cycles += 6;
    c.mtc1(0, zero);
    c.lwc1(0, cam.wrapping_add(ch.timer));
    c.c_le_s(zero, 0);
    let started = c.fcc();
    c.lwc1(1, cam.wrapping_add(ch.period));
    if !started {
        c.cycles += 1;
        c.mov_s(out, zero);
        return;
    }
    c.cycles += 3;
    c.c_le_s(0, 1);
    if !c.fcc() {
        c.cycles += 4;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, out);
        return;
    }
    c.cycles += 14;
    c.div_s(0, 0, 1);
    c.g[AT][0] = 0x4040_0000;
    c.mtc1(AT, 2);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3);
    c.add_s(1, 0, 0);
    c.mul_s(0, 0, 0);
    c.mul_s(1, 1, 0);
    c.mul_s(0, 0, 2);
    c.sub_s(0, 0, 1);
    c.c_lt_s(3, 0);
    if c.fcc() {
        c.mov_s(0, 3);
    }
    c.cycles += 2;
    c.mov_s(out, 0);
}

/// The distance from the camera to its target through VU0 (`|pos - target|`
/// into f0), with the difference spilled at `sp+tmp` and copied to
/// `sp+copy` as the original does.
pub(crate) fn shake_target_distance(c: &mut Cpu, tmp: u32, copy: u32, f1_from: Option<u32>) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    let cam = r32(c, S2);
    let s = sp(c);
    c.lqc2(3, cam.wrapping_add(shake::POS));
    c.lqc2(4, cam.wrapping_add(shake::TARGET));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(tmp));
    match f1_from {
        None => {
            c.g[AT][0] = 0x4316_0000; // 150.0
            c.mtc1(AT, 1);
        }
        Some(gp_off) => c.lwc1(1, r32(c, GP).wrapping_sub(gp_off)),
    }
    c.lq(V0, s.wrapping_add(tmp));
    c.sq(V0, s.wrapping_add(copy));
    c.lqc2(3, s.wrapping_add(copy));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
    c.mtc1(V1, 0);
}

/// `void ChaseCameraAlgorithm_ApplyShake(Camera* cam, int request, int
/// scale_by_distance, float period_base f12, float period_range f13, float
/// amplitude f14)` (0x165938): camera shake. A new request (or the last one
/// ending) sets 2 s of shake; the time left runs down by the frame step.
/// Six channels (three axes at two speeds) re-draw random targets when
/// their timers pass their random periods and ease between targets; the
/// summed offset, scaled by half the time left (capped at 1), by 15 or a
/// distance-based factor (within 150 units of the target), moves the
/// camera position.
pub fn chase_camera_algorithm_apply_shake(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save_fpr(c, 26, 0xb0);
        frame.save_fpr(c, 24, 0xa0);
        frame.save_fpr(c, 22, 0x90);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 27, 0xb8);
        frame.save_fpr(c, 25, 0xa8);
        frame.save_fpr(c, 23, 0x98);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A2][0];
        c.mov_s(26, 13);
        c.mov_s(24, 12);
        c.mov_s(22, 14);
        let cam = r32(c, S2);
        let request = c.g[A1][0] != 0;

        // Time left: 0 while requested; 2 s when the request just ended.
        if request {
            c.cycles += 2;
            c.sw(cam.wrapping_add(shake::TIME_LEFT), 0);
            c.cycles += 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(cam.wrapping_add(shake::LAST_REQUEST));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[AT][0] = 0x4000_0000;
                c.mtc1(AT, 0);
                c.swc1(0, cam.wrapping_add(shake::TIME_LEFT));
                c.cycles += 1;
            }
        }
        c.lwc1(2, cam.wrapping_add(shake::TIME_LEFT));
        c.cycles += 5;
        c.mtc1(0, 1);
        c.c_lt_s(1, 2);
        let running = c.fcc();
        c.sw(cam.wrapping_add(shake::LAST_REQUEST), c.g[A1][0]);
        if running {
            c.cycles += 4;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x68d0));
            c.sub_s(0, 2, 0);
            c.max_s(0, 0, 1);
            c.swc1(0, cam.wrapping_add(shake::TIME_LEFT));
        }
        // f27: the fade factor, half the time left (1 while requested).
        c.cycles += 5;
        c.lwc1(0, cam.wrapping_add(shake::TIME_LEFT));
        c.c_lt_s(1, 0);
        let fading = c.fcc();
        c.mov_s(2, 0);
        if fading {
            c.cycles += 4;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 0);
            c.mul_s(27, 2, 0);
        } else {
            c.cycles += 2;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 27);
        }
        c.cycles += 2;
        if !request {
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_lt_s(0, 2);
            let left = c.fcc();
            frame.restore(c, S0, 0x70);
            if !left {
                return shake_epilogue(c, frame);
            }
        }

        // Advance the slow channels' timers by the frame step and re-draw
        // the channels whose timers passed their periods.
        c.cycles += 13;
        c.lwc1(1, r32(c, GP).wrapping_sub(0x68cc));
        c.lwc1(0, cam.wrapping_add(0x278));
        c.lwc1(3, cam.wrapping_add(0x27c));
        c.add_s(0, 0, 1);
        c.lwc1(2, cam.wrapping_add(0x280));
        c.lwc1(4, cam.wrapping_add(0x254));
        c.add_s(3, 3, 1);
        c.add_s(2, 2, 1);
        c.c_lt_s(4, 0);
        c.swc1(0, cam.wrapping_add(0x278));
        c.swc1(3, cam.wrapping_add(0x27c));
        let due = c.fcc();
        c.swc1(2, cam.wrapping_add(0x280));
        if due {
            shake_retarget(c, &SHAKE_CHANNELS[0])?;
        }
        c.cycles += 6;
        c.lwc1(1, cam.wrapping_add(0x27c));
        c.lwc1(0, cam.wrapping_add(0x258));
        c.c_lt_s(0, 1);
        if c.fcc() {
            shake_retarget(c, &SHAKE_CHANNELS[1])?;
        }
        c.lwc1(0, cam.wrapping_add(0x280));
        // The z channel's amplitude is scaled by the constant at gp-0x68c8 (f23).
        c.cycles += 6;
        c.lwc1(1, cam.wrapping_add(0x25c));
        c.lwc1(2, r32(c, GP).wrapping_sub(0x68c8));
        c.c_lt_s(1, 0);
        let due = c.fcc();
        c.mul_s(23, 22, 2);
        if due {
            shake_retarget(c, &SHAKE_CHANNELS[2])?;
        }
        // The fast channels: half the period base, range and amplitudes.
        c.cycles += 10;
        c.lwc1(2, cam.wrapping_add(0x2a8));
        c.lwc1(1, cam.wrapping_add(0x284));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.c_lt_s(1, 2);
        c.mul_s(25, 24, 0);
        c.mul_s(22, 22, 0);
        c.mul_s(24, 26, 0);
        let due = c.fcc();
        c.mul_s(23, 23, 0);
        if due {
            shake_retarget(c, &SHAKE_CHANNELS[3])?;
        }
        c.cycles += 6;
        c.lwc1(1, cam.wrapping_add(0x2ac));
        c.lwc1(0, cam.wrapping_add(0x288));
        c.c_lt_s(0, 1);
        if c.fcc() {
            shake_retarget(c, &SHAKE_CHANNELS[4])?;
        }
        c.lwc1(1, cam.wrapping_add(0x2b0));
        c.cycles += 4;
        c.lwc1(0, cam.wrapping_add(0x28c));
        c.c_lt_s(0, 1);
        if c.fcc() {
            shake_retarget(c, &SHAKE_CHANNELS[5])?;
        }

        // Ease weights f20..f15 (zero registers f13, f11, f6, f4, f3, f2).
        for (ch, (out, zero)) in
            SHAKE_CHANNELS
                .iter()
                .zip([(20, 13), (19, 11), (18, 6), (17, 4), (16, 3), (15, 2)])
        {
            shake_ease(c, ch, out, zero);
        }

        // Offset = sum over both speeds of prev + w (cur - prev), as
        // (1 - w) prev + w cur (f13, f11, f6 = x, y, z).
        c.cycles += 42;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.lwc1(5, cam.wrapping_add(0x26c));
        c.sub_s(10, 0, 20);
        c.lwc1(2, cam.wrapping_add(0x270));
        c.sub_s(9, 0, 19);
        c.lwc1(1, cam.wrapping_add(0x274));
        c.sub_s(8, 0, 18);
        c.lwc1(13, cam.wrapping_add(0x260));
        c.sub_s(14, 0, 15);
        c.lwc1(11, cam.wrapping_add(0x264));
        c.sub_s(7, 0, 17);
        c.lwc1(6, cam.wrapping_add(0x268));
        c.mul_s(10, 10, 5);
        c.lwc1(4, cam.wrapping_add(0x290));
        c.mul_s(9, 9, 2);
        c.lwc1(12, cam.wrapping_add(0x29c));
        c.mul_s(8, 8, 1);
        c.lwc1(3, cam.wrapping_add(0x294));
        c.sub_s(0, 0, 16);
        c.lwc1(5, cam.wrapping_add(0x2a0));
        c.lwc1(2, cam.wrapping_add(0x298));
        c.mul_s(7, 7, 12);
        c.lwc1(1, cam.wrapping_add(0x2a4));
        c.mul_s(13, 20, 13);
        c.mul_s(0, 0, 5);
        c.mul_s(14, 14, 1);
        c.mul_s(11, 19, 11);
        c.mul_s(6, 18, 6);
        c.mul_s(4, 17, 4);
        c.mul_s(3, 16, 3);
        c.mul_s(2, 15, 2);
        c.add_s(13, 13, 10);
        c.add_s(11, 11, 9);
        c.add_s(6, 6, 8);
        c.add_s(4, 4, 7);
        c.add_s(3, 3, 0);
        c.add_s(2, 2, 14);
        c.add_s(13, 13, 4);
        c.add_s(11, 11, 3);
        let by_distance = c.g[S3][0] != 0;
        c.add_s(6, 6, 2);

        // Scale (f0): 15, or distance * k within 150 units of the target.
        let mut near = false;
        if by_distance {
            c.cycles += 21;
            shake_target_distance(c, 0x10, 0, None);
            c.c_le_s(0, 1);
            near = c.fcc();
            if near {
                c.cycles += 19;
                shake_target_distance(c, 0x20, 0x10, Some(0x68c4));
                c.mul_s(0, 0, 1);
            }
        }
        if !near {
            c.cycles += 2;
            c.g[AT][0] = 0x4170_0000;
            c.mtc1(AT, 0);
        }

        // pos += offset * scale * fade.
        let s = sp(c);
        c.cycles += 17;
        c.swc1(13, s.wrapping_add(0x10));
        c.mul_s(0, 27, 0);
        c.swc1(11, s.wrapping_add(0x14));
        c.swc1(6, s.wrapping_add(0x18));
        c.sw(s.wrapping_add(0x1c), 0);
        c.mfc1(V1, 0);
        c.lqc2(4, s.wrapping_add(0x10));
        c.qmtc2(V1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s.wrapping_add(0x20));
        c.lq(V0, s.wrapping_add(0x20));
        c.sq(V0, s);
        c.lqc2(3, cam.wrapping_add(shake::POS));
        c.lqc2(4, s);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, cam.wrapping_add(shake::POS));
        frame.restore(c, S0, 0x70);
        shake_epilogue(c, frame)
    })
}

pub(crate) fn shake_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 14;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore_ra(c, 0x30);
    for (f, off) in [
        (27, 0xb8),
        (26, 0xb0),
        (25, 0xa8),
        (24, 0xa0),
        (23, 0x98),
        (22, 0x90),
        (21, 0x88),
        (20, 0x80),
    ] {
        frame.restore_fpr(c, f, off);
    }
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// ChaseCameraAlgorithm_GatherRiderState (0x162568).

/// `vf4.w = vf3 . vf(b)` (xyz), the compiler's VU0 dot-product idiom
/// (`vf6.w` as the 1.0 multiplier through ACC).
#[inline]
pub(crate) fn vu0_dot3(c: &mut Cpu, b: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `|v|` of the vector at `addr` into Q (`sqrt Q, vf4.w`), then `cfc2 rt, Q`
/// and `mtc1 rt, f`.
#[inline]
pub(crate) fn vu0_length(c: &mut Cpu, addr: u32, f: usize) {
    c.lqc2(3, addr);
    vu0_dot3(c, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, f);
}

// ---------------------------------------------------------------------------
// ChaseCameraAlgorithm_UpdateChaseHeading (0x162c78).

/// `void ChaseCameraAlgorithm_UpdateChaseHeading(ChaseCameraAlgorithm*
/// self, CamInfo* info)` (0x162c78): above 27.8 m/s, measure the angle
/// between the stored chase heading (`self + 0x80`) and the target's
/// velocity (kept from last frame when shorter than 1), pass it (less a
/// 10.9 degree dead zone) through a 5-tap filter history at `self + 0x340`
/// (input, output and error rings, gains at +0x40..+0x48), and turn the
/// filtered angle into a blend rate (square-rooted twice in rider states
/// 11..13); blend the heading towards `info->velocity / info->speed`.
/// Below that speed keep the heading. Store the normalised heading in
/// `info + 0x30`.
pub fn chase_camera_algorithm_update_chase_heading(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xb0);
        let gp = r32(c, GP);
        c.lwc1(1, gp.wrapping_sub(0x6ae0)); // 27.78 m/s
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S1][0] = c.g[A1][0];
        let info = r32(c, S1);
        c.lwc1(0, info.wrapping_add(0x44));
        c.c_lt_s(1, 0);
        c.g[S0][0] = c.g[A0][0];
        let me = r32(c, S0);
        let sp0 = sp(c);
        if c.fcc() {
            chase_heading_follow(c, me, info, sp0, gp)?;
        } else {
            c.cycles += 1;
            c.lq(2, me.wrapping_add(0x80));
        }

        // info->heading = normalize(heading).
        c.cycles += 25;
        c.sq(2, sp0);
        vu0_length(c, sp0, 0);
        frame.restore(c, S0, 0xa0);
        c.mfc1(3, 0);
        c.qmtc2(3, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, sp0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, sp0);
        frame.restore_ra(c, 0x80);
        c.lq(2, sp0);
        c.sq(2, info.wrapping_add(0x30));
        frame.restore(c, S1, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// The fast branch of `UpdateChaseHeading`: filter the heading error and
/// blend; leaves the new heading in `$2` (and at `self + 0x80`).
pub(crate) fn chase_heading_follow(c: &mut Cpu, me: u32, info: u32, sp0: u32, gp: u32) -> Flow {
    // Target velocity (virtual slot 0x18 of the target, into sp+0x10);
    // keep last frame's when |v|^2 < 1.
    c.cycles += 22;
    c.g[A2][0] = Addr(me.wrapping_add(0x30)).lw(c);
    let obj = r32(c, A2);
    c.g[A0][0] = sx(sp0 + 0x10);
    c.g[V0][0] = Addr(obj).lw(c);
    let vt = r32(c, V0);
    c.g[A1][0] = Addr(vt.wrapping_add(0x18)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(0x1c)).lw(c);
    c.g[A1][0] = sx(obj.wrapping_add(r32(c, A1)));
    let func = r32(c, V1);
    call_ptr(c, func, 0x0016_2cc0)?;
    c.g[AT][0] = 0x3f80_0000;
    c.f[4] = 0x3f80_0000;
    c.lqc2(3, sp0 + 0x10);
    c.lqc2(5, sp0 + 0x10);
    vu0_dot3(c, 5);
    c.qmfc2(2, 4);
    c.mtc1(2, 0);
    c.c_lt_s(0, 4);
    c.lq(2, sp0 + 0x10);
    if c.fcc() {
        c.cycles += 3;
        c.lq(2, me.wrapping_add(0x1c0));
        c.sq(2, sp0 + 0x10);
        c.lq(2, sp0 + 0x10);
    }

    // len = sqrt(|h|^2 |v|^2); angle = 90 deg - acos(clamp(h.v / len)).
    c.cycles += 25;
    c.lwc1(2, gp.wrapping_sub(0x6adc)); // 0.001
    c.sq(2, me.wrapping_add(0x1c0));
    c.lqc2(3, me.wrapping_add(0x80));
    c.lqc2(5, me.wrapping_add(0x80));
    vu0_dot3(c, 5);
    c.qmfc2(3, 4);
    c.mtc1(3, 0);
    c.lqc2(3, me.wrapping_add(0x1c0));
    c.lqc2(5, me.wrapping_add(0x1c0));
    vu0_dot3(c, 5);
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.mul_s(0, 0, 1);
    c.sqrt_s(3, 0);
    c.c_lt_s(3, 2);
    if c.fcc() {
        c.cycles += 3;
        c.f[7] = 0;
    } else {
        c.cycles += 16;
        c.lqc2(3, me.wrapping_add(0x80));
        c.lqc2(5, me.wrapping_add(0x1c0));
        vu0_dot3(c, 5);
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[1] = 0xbf80_0000;
        c.div_s(0, 0, 3);
        c.c_le_s(1, 0);
        c.f[12] = c.f[1];
        if c.fcc() {
            c.cycles += 1;
            c.min_s(12, 0, 4);
        }
        c.cycles += 5;
        call_ptr(c, 0x0031_c128, 0x0016_2dc4)?; // acosf
        c.lwc1(1, gp.wrapping_sub(0x6ad8)); // pi/2
        c.sub_s(7, 1, 0);
    }
    c.g[V0][0] = Addr(me.wrapping_add(0x37c)).lw(c);

    // f7 = max(angle - dead zone, 0); f3 = f7 - previous output; f2 = sum
    // of the error ring.
    c.cycles += 4;
    c.g[V1][0] = 5;
    c.lwc1(1, gp.wrapping_sub(0x6ad4)); // dead zone
    c.cycles += 16;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
    c.f[0] = 0;
    c.div(0, 2, 3);
    c.sub_s(7, 7, 1);
    c.g[V0][0] = 5;
    c.g[T0][0] = sx(me.wrapping_add(0x368));
    c.f[2] = c.f[0];
    c.max_s(7, 7, 0);
    c.g[A3][0] = sx(me.wrapping_add(0x340));
    c.g[V0][0] = c.g[T0][0];
    c.g[V1][0] = 4;
    c.g[A0][0] = c.hi[0];
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.g[A0][0] = sx(me.wrapping_add(r32(c, A0)));
    c.lwc1(4, r32(c, A0).wrapping_add(0x354));
    c.sub_s(3, 7, 4);
    loop {
        c.cycles += 7;
        c.lwc1(0, r32(c, V0));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
        let more = (c.g[V1][0] as i64) >= 0;
        c.add_s(2, 2, 0);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0016_2e20);
        }
    }

    // Filter step: out = prev + d*k0 + sum*k1 + (in[prev] - f7)*k2; store
    // in/out/error at idx, advance idx; rate from the output.
    c.cycles += 4;
    let hist = r32(c, A3);
    c.g[A0][0] = Addr(hist + 0x3c).lw(c);
    c.g[A1][0] = 5;
    c.cycles += 52;
    c.g[A2][0] = 5;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(4));
    c.lwc1(0, hist + 0x40);
    c.div(0, 2, 5);
    c.lwc1(1, hist + 0x44);
    c.mul_s(0, 3, 0);
    c.g[A0][0] = sx(r32(c, A0) << 2);
    c.mul_s(1, 2, 1);
    c.lwc1(3, hist + 0x48);
    c.g[A0][0] = sx(hist.wrapping_add(r32(c, A0)));
    c.lwc1(6, gp.wrapping_sub(0x6ad0));
    c.add_s(0, 4, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.f[8] = 0x3f80_0000;
    c.f[2] = 0;
    c.lwc1(4, gp.wrapping_sub(0x6acc));
    c.add_s(0, 0, 1);
    c.lwc1(5, gp.wrapping_sub(0x6ac8));
    c.g[V1][0] = c.hi[0];
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = sx(hist.wrapping_add(r32(c, V1)));
    c.lwc1(1, r32(c, V1));
    c.swc1(7, r32(c, A0));
    c.sub_s(1, 1, 7);
    c.g[V0][0] = Addr(hist + 0x3c).lw(c);
    c.mul_s(1, 1, 3);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(hist.wrapping_add(r32(c, V0)));
    c.add_s(0, 0, 1);
    c.swc1(0, r32(c, V0).wrapping_add(0x14));
    c.sub_s(1, 7, 0);
    c.mul_s(0, 0, 6);
    c.g[V0][0] = Addr(hist + 0x3c).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.min_s(0, 0, 8);
    c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V0)));
    c.max_s(0, 0, 2);
    c.swc1(1, r32(c, V0));
    c.sub_s(0, 8, 0);
    c.g[V0][0] = Addr(hist + 0x3c).lw(c);
    c.mul_s(0, 0, 4);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.div(0, 2, 5);
    c.g[A0][0] = c.hi[0];
    let idx = c.g[A0][0];
    Addr(hist + 0x3c).sw(c, idx);
    c.g[V1][0] = Addr(me.wrapping_add(0x30)).lw(c);
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(4)).lw(c);
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x434)).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xb));
    c.g[V0][0] = (c.g[V0][0] < 3) as u64;
    c.add_s(2, 0, 5);
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.sqrt_s(0, 2);
        c.sqrt_s(2, 0);
    }

    // heading = heading * rate + (velocity / speed) * (1 - rate).
    c.cycles += 32;
    c.lwc1(0, info.wrapping_add(0x44));
    c.sub_s(1, 8, 2);
    c.div_s(0, 8, 0);
    c.mfc1(3, 0);
    c.lqc2(4, info.wrapping_add(0x20));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp0 + 0x30);
    c.lq(2, sp0 + 0x30);
    c.sq(2, sp0 + 0x20);
    c.mfc1(3, 2);
    c.lqc2(4, me.wrapping_add(0x80));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp0 + 0x60);
    c.lq(2, sp0 + 0x60);
    c.sq(2, sp0 + 0x50);
    c.mfc1(3, 1);
    c.lqc2(4, sp0 + 0x20);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, sp0 + 0x70);
    c.lq(2, sp0 + 0x70);
    c.sq(2, sp0 + 0x60);
    c.lqc2(3, sp0 + 0x50);
    c.lqc2(4, sp0 + 0x60);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp0 + 0x70);
    c.lq(2, sp0 + 0x70);
    c.sq(2, me.wrapping_add(0x80));
    c.sq(2, sp0 + 0x40);
    Ok(())
}

pub(crate) mod chase {

    /// The rider/model object (first word the vtable; +4 the rider).
    pub const RIG: u32 = 0x30;

    /// `vec4` smoothed chase heading.
    pub const HEADING: u32 = 0x80;

    /// `vec4` last valid look direction from the rig.
    pub const LAST_DIR: u32 = 0x1c0;

    /// The heading-angle PID filter (`sHist`): `a[5]` measured angles,
    /// `b[5]` filtered values (+0x14), `c[5]` errors (+0x28), `idx` (+0x3c),
    /// gains `k0`, `k1`, `k2` (+0x40/+0x44/+0x48).
    pub const HIST: u32 = 0x340;

    pub const H_B: u32 = 0x14;

    pub const H_C: u32 = 0x28;

    pub const H_IDX: u32 = 0x3c;

    pub const H_K0: u32 = 0x40;

    pub const H_K1: u32 = 0x44;

    pub const H_K2: u32 = 0x48;

    /// Rider state word (in the rider at rig+4); states 0xb..0xd slow the
    /// heading follow (double square root of the blend).
    pub const RIDER_STATE: u32 = 0x434;
}

pub(crate) mod chase_info {

    /// `vec4` travel direction (scaled by 1/speed).
    pub const DIR: u32 = 0x20;

    /// `vec4` output: the normalised heading.
    pub const OUT: u32 = 0x30;

    /// Speed (m/s).
    pub const SPEED: u32 = 0x44;
}

pub(crate) mod chase_lit {

    /// 27.78 m/s (100 km/h): below this the heading is left alone.
    pub const MIN_SPEED: i32 = -0x6ae0;

    /// 0.001: shorter vector products give an angle of 0.
    pub const MIN_LEN: i32 = -0x6adc;

    /// pi/2 (acos = pi/2 - asin).
    pub const HALF_PI: i32 = -0x6ad8;

    /// 0.1908 rad dead zone subtracted from the heading error.
    pub const DEAD_ZONE: i32 = -0x6ad4;

    /// 2.52: scale of the filtered angle before clamping to [0, 1].
    pub const ANGLE_SCALE: i32 = -0x6ad0;

    /// Blend `s = (1 - clamp) * 0.0395 + 0.9555`.
    pub const BLEND_RANGE: i32 = -0x6acc;

    pub const BLEND_BASE: i32 = -0x6ac8;
}

/// gp-relative address `gp + off`.
#[inline(always)]
pub(crate) fn gp_at(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `vf4.x = a . b` (four lanes) as the original's VU0 macro sequence
/// leaves it: vf3 = a, vf5 = b, vf6.x = 1, ACC.x scratch.
pub(crate) fn vu_dot4(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    vu_dot4_regs(c, 5);
}

/// The dot product of vf3 with vf`b` into vf4.x.
pub(crate) fn vu_dot4_regs(c: &mut Cpu, b: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `vf5 = vf4 * s` with `s` from FPU register `f` (through v1/vf3), stored
/// to `tmp` and copied (via v0) to `dst`, as the original's inlined
/// vector-scale does.
pub(crate) fn vu_scale_to(c: &mut Cpu, f: usize, src: u32, tmp: u32, dst: u32) {
    c.mfc1(3, f);
    c.lqc2(4, src);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, tmp);
    c.lq(2, tmp);
    c.sq(2, dst);
}

/// `void ChaseCameraAlgorithm_UpdateChaseHeading(ChaseCameraAlgorithm *this,
/// ChaseInfo *info)` (0x162c78): above 100 km/h, measure the angle between
/// the smoothed heading and the rig's look direction (virtual call at
/// vtable +0x18; a near-zero direction keeps the last one), run it through
/// the heading PID filter (5-entry history at +0x340), and blend the heading
/// toward the travel direction by the filtered blend factor (slower in rider
/// states 0xb..0xd). Always writes the normalised heading to `info->out`.
pub fn chase_camera_update_chase_heading(c: &mut Cpu) {
    use chase::*;
    use chase_info::*;
    use chase_lit::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xb0);
        c.lwc1(1, gp_at(c, MIN_SPEED));
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S1][0] = c.g[A1][0];
        let info = r32(c, S1);
        c.lwc1(0, info.wrapping_add(SPEED));
        c.c_lt_s(1, 0);
        let fast = c.fcc();
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        if !fast {
            // Slow: keep the heading as it is.
            c.cycles += 1;
            c.lq(2, this.wrapping_add(HEADING));
        } else {
            update_heading(c, frame, this, info)?;
        }

        // Normalise the heading (v0) into info->out.
        c.cycles += 25;
        let tmp = frame.at(c, 0);
        c.sq(2, tmp);
        c.lqc2(3, tmp);
        vu_dot4_regs(c, 3);
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 0);
        frame.restore(c, S0, 0xa0);
        c.mfc1(3, 0);
        c.qmtc2(3, 3);
        c.vu0().div(0, 3, 3, 0);
        let tmp = frame.at(c, 0);
        c.lqc2(4, tmp);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, tmp);
        frame.restore_ra(c, 0x80);
        c.lq(2, tmp);
        c.sq(2, info.wrapping_add(OUT));
        frame.restore(c, S1, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// The fast path of [`chase_camera_update_chase_heading`]: leaves the new
/// heading in v0 (and at `this->heading`).
pub(crate) fn update_heading(c: &mut Cpu, frame: Frame, this: u32, info: u32) -> crate::call::Flow {
    use chase::*;
    use chase_info::*;
    use chase_lit::*;
    // The rig's look direction (virtual, vtable slot +0x18) into sp+0x10.
    c.cycles += 22;
    let rig = Addr(this.wrapping_add(RIG)).lw(c);
    c.g[A2][0] = rig;
    c.g[A0][0] = sx(frame.at(c, 0x10));
    c.g[V0][0] = Addr(rig as u32).lw(c);
    let vt = c.g[V0][0] as u32;
    c.g[A1][0] = Addr(vt.wrapping_add(0x18)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(0x1c)).lw(c);
    c.g[A1][0] = sx((rig as u32).wrapping_add(c.g[A1][0] as u32));
    let method = c.g[V1][0] as u32;
    call_ptr(c, method, 0x0016_2cc0)?;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 4);
    let dir = frame.at(c, 0x10);
    vu_dot4(c, dir, dir);
    c.qmfc2(2, 4);
    c.mtc1(2, 0);
    c.c_lt_s(0, 4);
    let short = c.fcc();
    c.lq(2, dir);
    if short {
        // A near-zero direction: reuse the last one.
        c.cycles += 3;
        c.lq(2, this.wrapping_add(LAST_DIR));
        c.sq(2, dir);
        c.lq(2, dir);
    }

    // The angle between the heading and the direction (acos via pi/2 - asin).
    c.cycles += 25;
    c.lwc1(2, gp_at(c, MIN_LEN));
    c.sq(2, this.wrapping_add(LAST_DIR));
    vu_dot4(c, this.wrapping_add(HEADING), this.wrapping_add(HEADING));
    c.qmfc2(3, 4);
    c.mtc1(3, 0);
    vu_dot4(c, this.wrapping_add(LAST_DIR), this.wrapping_add(LAST_DIR));
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.mul_s(0, 0, 1);
    c.sqrt_s(3, 0);
    c.c_lt_s(3, 2);
    if c.fcc() {
        c.cycles += 3;
        c.mtc1(0, 7);
    } else {
        c.cycles += 16;
        vu_dot4(c, this.wrapping_add(HEADING), this.wrapping_add(LAST_DIR));
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 1);
        c.div_s(0, 0, 3);
        c.c_le_s(1, 0);
        let in_range = c.fcc();
        c.mov_s(12, 1);
        if in_range {
            c.cycles += 1;
            c.min_s(12, 0, 4);
        }
        c.cycles += 5;
        call_ptr(c, 0x0031_c128, 0x0016_2dc4)?; // Math_Asin(clamped cos)
        c.lwc1(1, gp_at(c, HALF_PI));
        c.sub_s(7, 1, 0);
    }
    c.g[V0][0] = Addr(this.wrapping_add(HIST + H_IDX)).lw(c);

    // ang = max(ang - dead zone, 0); prev = b[(idx + 4) % 5].
    c.cycles += 4;
    c.g[V1][0] = 5;
    c.lwc1(1, gp_at(c, DEAD_ZONE));
    c.cycles += 16;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
    c.mtc1(0, 0);
    c.div(0, V0, V1);
    c.sub_s(7, 7, 1);
    c.g[V0][0] = 5;
    let errs = this.wrapping_add(HIST + H_C);
    c.g[T0][0] = sx(errs);
    c.mov_s(2, 0);
    c.max_s(7, 7, 0);
    let hist = this.wrapping_add(HIST);
    c.g[A3][0] = sx(hist);
    c.g[V0][0] = c.g[T0][0];
    c.g[V1][0] = 4;
    c.g[A0][0] = sx(this.wrapping_add((c.hi[0] as u32) << 2));
    c.lwc1(4, (c.g[A0][0] as u32).wrapping_add(HIST + H_B));
    c.sub_s(3, 7, 4);

    // sum = c[0] + ... + c[4] (f2).
    loop {
        c.cycles += 7;
        c.lwc1(0, c.g[V0][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        let more = (c.g[V1][0] as i64) >= 0;
        c.add_s(2, 2, 0);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0016_2e20);
        }
    }

    // val = prev + d*k0 + sum*k1 + (a[p2] - ang)*k2; store a/b/c[idx];
    // idx = (idx + 1) % 5; blend s from val.
    c.cycles += 4;
    c.g[A0][0] = Addr(hist.wrapping_add(H_IDX)).lw(c);
    c.g[A1][0] = 5;
    c.cycles += 52;
    c.g[A2][0] = 5;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
    c.lwc1(0, hist.wrapping_add(H_K0));
    c.div(0, V0, A1);
    c.lwc1(1, hist.wrapping_add(H_K1));
    c.mul_s(0, 3, 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    c.mul_s(1, 2, 1);
    c.lwc1(3, hist.wrapping_add(H_K2));
    c.g[A0][0] = sx(hist.wrapping_add(c.g[A0][0] as u32));
    c.lwc1(6, gp_at(c, ANGLE_SCALE));
    c.add_s(0, 4, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 8);
    c.mtc1(0, 2);
    c.lwc1(4, gp_at(c, BLEND_RANGE));
    c.add_s(0, 0, 1);
    c.lwc1(5, gp_at(c, BLEND_BASE));
    c.g[V1][0] = sx(hist.wrapping_add((c.hi[0] as u32) << 2));
    c.lwc1(1, c.g[V1][0] as u32);
    c.swc1(7, c.g[A0][0] as u32);
    c.sub_s(1, 1, 7);
    c.g[V0][0] = Addr(hist.wrapping_add(H_IDX)).lw(c);
    c.mul_s(1, 1, 3);
    c.g[V0][0] = sx(hist.wrapping_add((c.g[V0][0] as u32) << 2));
    c.add_s(0, 0, 1);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(H_B));
    c.sub_s(1, 7, 0);
    c.mul_s(0, 0, 6);
    c.g[V0][0] = Addr(hist.wrapping_add(H_IDX)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.min_s(0, 0, 8);
    c.g[V0][0] = sx(errs.wrapping_add(c.g[V0][0] as u32));
    c.max_s(0, 0, 2);
    c.swc1(1, c.g[V0][0] as u32);
    c.sub_s(0, 8, 0);
    c.g[V0][0] = Addr(hist.wrapping_add(H_IDX)).lw(c);
    c.mul_s(0, 0, 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.div(0, V0, A1);
    c.g[A0][0] = c.hi[0];
    Addr(hist.wrapping_add(H_IDX)).sw(c, c.g[A0][0]);
    c.g[V1][0] = Addr(this.wrapping_add(RIG)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(RIDER_STATE)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb));
    c.g[V0][0] = (c.g[V0][0] < 3) as u64;
    let slow_state = c.g[V0][0] != 0;
    c.add_s(2, 0, 5);
    if slow_state {
        c.cycles += 2;
        c.sqrt_s(0, 2);
        c.sqrt_s(2, 0);
    }

    // The filter and state-specific fourth root determine the final retained
    // share. Preserve the original FPU sequence at 60; at other rates keep
    // that effective retention per 1/60 second.
    let hz = crate::tick::rate(c);
    if hz != 60 {
        c.f[2] = f32::from_bits(c.f[2]).powf(60.0 / hz as f32).to_bits();
    }

    // heading = dir * (1/speed) * (1 - s) + heading * s.
    c.cycles += 32;
    c.lwc1(0, info.wrapping_add(SPEED));
    c.sub_s(1, 8, 2);
    c.div_s(0, 8, 0);
    let (t20, t30) = (frame.at(c, 0x20), frame.at(c, 0x30));
    vu_scale_to(c, 0, info.wrapping_add(DIR), t30, t20);
    let (t50, t60, t70) = (frame.at(c, 0x50), frame.at(c, 0x60), frame.at(c, 0x70));
    vu_scale_to(c, 2, this.wrapping_add(HEADING), t60, t50);
    vu_scale_to(c, 1, t20, t70, t60);
    c.lqc2(3, t50);
    c.lqc2(4, t60);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, t70);
    c.lq(2, t70);
    c.sq(2, this.wrapping_add(HEADING));
    let t40 = frame.at(c, 0x40);
    c.sq(2, t40);
    Ok(())
}

// ---------------------------------------------- GameCamera_UpdateStreamView

/// `void GameCamera_UpdateStreamView(GameCamera* cam)` (0x15ec98): point
/// the world-cache streamer slot of this camera at the camera position with
/// a radius of 1.5x the view distance (+8, clamped to [0, 15000] except in
/// mode 1 of the game module's race state), then flag the camera
/// (`+0xb4 = 1`) when the streamer reports a clearance at least as large
/// as the far limit (2/3 of the distance clamped to [15000, 30000]) or the
/// view distance itself.
pub fn game_camera_update_stream_view(c: &mut Cpu) {
    /// `GameCamera` fields.
    const VIEW_DIST: u32 = 0x8;
    const POSITION: u32 = 0x20;
    const STREAM_SLOT: u32 = 0xb0;
    const CLEARANCE_OK: u32 = 0xb4;
    /// Streamer slots: `0x50` bytes each, the active flag at +0x250 of the
    /// world cache (the streamer starts at +0x10).
    const SLOT_SIZE: u32 = 0x50;
    const SLOT_ACTIVE: u32 = 0x250;
    const STREAMER: u32 = 0x10;
    /// `cWorldCache_StreamerSetView(streamer, slot, pos, radius)`.
    const STREAMER_SET_VIEW: u32 = 0x003a_9658;
    /// `cWorldCache_StreamerGetViewClearance(streamer, slot, &pos, &t)`.
    const STREAMER_GET_VIEW_CLEARANCE: u32 = 0x003a_96e0;
    /// Globals: the app (`gp-0x848`), the world cache pointer (`gp+0x16c8`).
    const APP_GP: u32 = 0x848;
    const WORLD_CACHE_GP: u32 = 0x16c8;
    /// `.lit4`: 15000, 2/3, 15000, 30000.
    const MAX_DIST_GP: u32 = 0x6c68;
    const FAR_SCALE_GP: u32 = 0x6c64;
    const FAR_MIN_GP: u32 = 0x6c60;
    const FAR_MAX_GP: u32 = 0x6c5c;
    let lit = |c: &Cpu, off: u32| r32(c, GP).wrapping_sub(off);
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        let app = Addr(lit(c, APP_GP)).lw(c);
        c.g[A1][0] = app;
        frame.save(c, S1, 0x40);
        c.g[A2][0] = 1;
        frame.save(c, S0, 0x50);
        let cam = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        let module = Addr(app as u32).off(0x84).lw(c);
        c.g[V0][0] = module;
        let race = Addr(module as u32).off(0x84).lw(c);
        c.g[V1][0] = race;
        let mode = Addr(race as u32).off(0x10).lw(c);
        c.g[A0][0] = mode;
        // The view distance, clamped unless in mode 1.
        let dist = if mode != 1 {
            let raw = Addr(cam).off(VIEW_DIST).read_fpu(c);
            c.f[1] = raw.0;
            c.cycles += 6;
            c.f[0] = 0;
            let max = Addr(lit(c, MAX_DIST_GP)).read_fpu(c);
            c.f[2] = max.0;
            let positive = FpuF32(0).le(raw);
            abi::set_fcc(c, positive);
            let mut d = FpuF32(0);
            if positive {
                c.cycles += 1;
                d = raw.min(max);
            }
            c.f[3] = d.0;
            c.cycles += 1;
            d
        } else {
            c.cycles += 2;
            Addr(cam).off(VIEW_DIST).read_fpu(c)
        };
        c.f[20] = dist.0;

        // Radius and far limit.
        c.cycles += 11;
        let k = Addr(lit(c, FAR_SCALE_GP)).read_fpu(c);
        let far_min = Addr(lit(c, FAR_MIN_GP)).read_fpu(c);
        c.f[2] = far_min.0;
        let scaled = dist * k;
        c.f[0] = scaled.0;
        c.g[AT][0] = 0x3fc0_0000;
        let one_half = FpuF32(0x3fc0_0000);
        c.f[1] = one_half.0;
        let mut far = far_min;
        let far_max = Addr(lit(c, FAR_MAX_GP)).read_fpu(c);
        c.f[3] = far_max.0;
        let above = far_min.le(scaled);
        abi::set_fcc(c, above);
        c.f[12] = (dist * one_half).0;
        if above {
            c.cycles += 1;
            far = scaled.min(far_max);
        }
        c.f[21] = far.0;

        // Mark the slot active and set the view.
        c.cycles += 22;
        c.g[V1][0] = Addr(cam).off(STREAM_SLOT).lw(c);
        c.g[V0][0] = SLOT_SIZE as u64;
        let cache_ptr = Addr(r32(c, GP).wrapping_add(WORLD_CACHE_GP)).lw(c);
        c.g[S0][0] = cache_ptr;
        c.g[S2][0] = 1;
        c.mult(0, V1, V1, V0);
        c.g[A2][0] = sx(cam.wrapping_add(POSITION));
        let cache = Addr(cache_ptr as u32).lw(c);
        c.g[V0][0] = abi::addu(cache, c.g[V1][0]);
        Addr(r32(c, V0)).off(SLOT_ACTIVE).sw(c, 1);
        let cache = Addr(cache_ptr as u32).lw(c);
        c.g[A1][0] = Addr(cam).off(STREAM_SLOT).lw(c);
        c.g[A0][0] = sx((cache as u32).wrapping_add(STREAMER));
        call_ptr(c, STREAMER_SET_VIEW, 0x0015_ed60)?;
        Addr(cam).off(CLEARANCE_OK).sw(c, 0);
        c.g[A2][0] = c.g[SP][0];
        c.g[A1][0] = Addr(cam).off(STREAM_SLOT).lw(c);
        let clearance_at = frame.at(c, 0x10);
        c.g[A3][0] = sx(clearance_at);
        let cache = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = sx((cache as u32).wrapping_add(STREAMER));
        call_ptr(c, STREAMER_GET_VIEW_CLEARANCE, 0x0015_ed7c)?;
        let have = c.g[V0][0] != 0;
        let clearance = Addr(frame.at(c, 0x10)).read_fpu(c);
        c.f[0] = clearance.0;
        if !have {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            let far_ok = far.le(clearance);
            abi::set_fcc(c, far_ok);
            if far_ok {
                Addr(cam).off(CLEARANCE_OK).sw(c, 1);
                c.cycles += 1;
            } else {
                c.cycles += 4;
                let near_ok = dist.le(clearance);
                abi::set_fcc(c, near_ok);
                if near_ok {
                    c.cycles += 1;
                    Addr(cam).off(CLEARANCE_OK).sw(c, 1);
                    c.cycles += 1;
                }
            }
        }
        frame.restore(c, S0, 0x50);
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cWorldCache_StreamerSetView (0x3a9658)

/// A world cache streamer (0x50 bytes, from +0x244 of the cache): active
/// flag (+0x244), state (+0x248), view position (+0x250), the view box's
/// low (+0x260) and high (+0x270) corners, the clearance (+0x280) and two
/// zeroed floats (+0x284 / +0x288).
pub(crate) const STREAMER_SIZE: u32 = 0x50;

pub(crate) const STREAMER_ACTIVE: u32 = 0x244;

pub(crate) const STREAMER_STATE: u32 = 0x248;

pub(crate) const STREAMER_VIEW_POS: u32 = 0x250;

pub(crate) const STREAMER_BOX_LO: u32 = 0x260;

pub(crate) const STREAMER_BOX_HI: u32 = 0x270;

pub(crate) const STREAMER_CLEARANCE: u32 = 0x280;

pub(crate) const STREAMER_ZERO_A: u32 = 0x284;

pub(crate) const STREAMER_ZERO_B: u32 = 0x288;

/// `void cWorldCache_StreamerSetView(cWorldCache* self /*a0*/,
/// unsigned i /*a1*/, sVec4* pos /*a2*/, float clearance /*f12*/)`
/// (0x3a9658): point streamer `i` at `pos`: the view box is `pos -
/// (r, r, r, 0)` .. `pos + (r, r, r, 0)` with `r = max(clearance, 0)`
/// (the w components use 0), the streamer becomes active with state 0.
/// Leaf, frame 0x20 (the copy of `pos` and the radius vector).
pub fn world_cache_streamer_set_view(c: &mut Cpu) {
    c.cycles += 34;
    c.g[V0][0] = STREAMER_SIZE as u64;
    c.mtc1(0, 1); // f1 = 0
    c.mult(0, 3, 5, 2); // v1 = i * 0x50
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    let frame = sp(c);
    let pos = r32(c, A2);
    c.lq(2, pos);
    c.max_s(0, 12, 1); // f0 = max(clearance, 0)
    c.g[A1][0] = 1;
    c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    let streamer = r32(c, A0);
    c.sq(2, streamer.wrapping_add(STREAMER_VIEW_POS));
    c.swc1(0, streamer.wrapping_add(STREAMER_CLEARANCE));
    c.lq(2, pos);
    c.sq(2, frame);
    c.sq(2, frame.wrapping_add(0x10));
    c.sq(2, streamer.wrapping_add(STREAMER_BOX_LO));
    c.lq(3, frame.wrapping_add(0x10));
    c.sq(3, streamer.wrapping_add(STREAMER_BOX_HI));
    // The radius vector (clearance, clearance, clearance, 0).
    c.swc1(12, frame.wrapping_add(0x8));
    c.swc1(12, frame);
    c.swc1(12, frame.wrapping_add(0x4));
    c.swc1(1, frame.wrapping_add(0xc));
    c.lqc2(3, streamer.wrapping_add(STREAMER_BOX_LO));
    c.lqc2(4, frame);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, streamer.wrapping_add(STREAMER_BOX_LO));
    c.lqc2(3, streamer.wrapping_add(STREAMER_BOX_HI));
    c.lqc2(4, frame);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, streamer.wrapping_add(STREAMER_BOX_HI));
    c.sw(streamer.wrapping_add(STREAMER_ACTIVE), c.g[A1][0]);
    c.swc1(1, streamer.wrapping_add(STREAMER_ZERO_B));
    c.sw(streamer.wrapping_add(STREAMER_STATE), 0);
    c.swc1(1, streamer.wrapping_add(STREAMER_ZERO_A));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    ret(c);
}

// -- 0x00166530 ChaseCameraAlgorithm_CheckDistanceReset_Wrapper --------------

/// `void ChaseCameraAlgorithm_CheckDistanceReset_Wrapper(Chase* self)`
/// (0x166530): forwards to `ChaseCameraAlgorithm_CheckDistanceReset(self)`
/// (0x168150). Frame 0x10 (ra).
pub fn chase_camera_check_distance_reset_wrapper(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(Callee {
            addr: 0x0016_8150,
            f: ee::s02::f_00168150,
        }),
        0x0016_6540,
    )
}

// ---------------------------------------------------------------------------
// RiderCameraTarget_v23 (0x15f780)

/// `RiderCameraTarget::mRider` (the rider object).
pub(crate) const RCT_V23_RIDER: u32 = 0x4;

/// `Rider + 0x788`: the landing record (velocity at +0x10, normal at +0x20).
pub(crate) const RCT_V23_LANDING: u32 = 0x788;

pub(crate) const RCT_V23_LANDING_VELOCITY: u32 = 0x10;

pub(crate) const RCT_V23_LANDING_NORMAL: u32 = 0x20;

/// `.lit4` at gp-0x6c50: minimum |n|·|v| (0.001) for a meaningful angle.
pub(crate) const RCT_V23_MIN_LEN: u32 = 0x6c50;

/// `.lit4` at gp-0x6c4c: pi/2.
pub(crate) const RCT_V23_HALF_PI: u32 = 0x6c4c;

/// `Math_Asin` (0x31c128, another shard: through the table).
pub(crate) const RCT_V23_MATH_ASIN: u32 = 0x0031_c128;

/// gp-relative address `gp - off`.
#[inline]
pub(crate) fn rct_v23_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// The compiled inline dot product of the quadwords at `a` and `b`: leaves
/// the result in vf4 (lane w), with vf3/vf5/vf6 and ACC as the original.
#[inline]
pub(crate) fn rct_v23_dot(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `float RiderCameraTarget_v23(RiderCameraTarget* self)` (0x15f780): the
/// landing angle of the rider. Takes the rider's landing normal `n` and
/// landing velocity `v`; if both have positive squared length and
/// |n|·|v| >= 0.001, returns `pi/2 - asin(clamp(n·(-v) / (|n||v|), -1, 1))`
/// (the angle between the surface and the incoming direction), else 0.
pub fn rider_camera_target_v23(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        c.mtc1(0, 1);
        frame.save_ra(c, 0x40);
        c.g[A1][0] = 0;
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(RCT_V23_RIDER)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(RCT_V23_LANDING)).lw(c);
        let landing = c.g[A0][0] as u32;
        // sp+0x00: the normal, sp+0x10: the velocity.
        c.lq(2, landing.wrapping_add(RCT_V23_LANDING_NORMAL));
        c.sq(2, frame.at(c, 0));
        c.lq(3, landing.wrapping_add(RCT_V23_LANDING_VELOCITY));
        c.sq(3, frame.at(c, 0x10));
        let (n, v) = (frame.at(c, 0), frame.at(c, 0x10));
        rct_v23_dot(c, n, n);
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.cycles += 13;
            rct_v23_dot(c, v, v);
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.g[A1][0] = 1;
            }
        }
        c.cycles += 2;
        if c.g[A1][0] == 0 {
            // No usable vectors: 0.
            c.cycles += 1;
            c.mtc1(0, 0);
            c.cycles += 1;
            frame.restore_ra(c, 0x40);
        } else {
            c.cycles += 34;
            // sp+0x20 (via sp+0x30): -v.
            c.g[AT][0] = sx(0xbf80_0000);
            c.mtc1(1, 3);
            c.mfc1(2, 3);
            c.lqc2(4, v);
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x30));
            let lit = rct_v23_gp(c, RCT_V23_MIN_LEN);
            c.lwc1(2, lit);
            c.lq(2, frame.at(c, 0x30));
            c.sq(2, frame.at(c, 0x20));
            let neg_v = frame.at(c, 0x20);
            rct_v23_dot(c, neg_v, neg_v);
            c.qmfc2(2, 4);
            c.mtc1(2, 1);
            rct_v23_dot(c, n, n);
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.mul_s(0, 0, 1);
            c.sqrt_s(1, 0);
            c.c_lt_s(1, 2);
            c.mtc1(0, 0);
            let too_short = c.fcc();
            frame.restore_ra(c, 0x40);
            if !too_short {
                c.cycles += 14;
                rct_v23_dot(c, n, neg_v);
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.div_s(12, 0, 1);
                // clamp(cos, -1, 1) (f3 = -1)
                c.c_le_s(3, 12);
                if !c.fcc() {
                    c.mov_s(12, 3);
                } else {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.min_s(12, 12, 0);
                }
                c.cycles += 5;
                call_ptr(c, RCT_V23_MATH_ASIN, 0x0015_f8e8)?;
                let lit = rct_v23_gp(c, RCT_V23_HALF_PI);
                c.lwc1(1, lit);
                c.sub_s(0, 1, 0);
                c.cycles += 1;
                frame.restore_ra(c, 0x40);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::elf_image;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::strict as strict_2;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_4;

    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;
    pub(crate) use crate::game::bx::bxstring2::tests::game;

    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_8;

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

    pub(crate) use crate::game::fe::feasyncfile::tests::pool;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;

    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_6;

    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2 as globals;

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

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;

    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_2;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_7;

    pub(crate) use crate::game::sound::soundsys::tests::w as w_10;

    pub(crate) use crate::game::ui::uivector::tests::start;
    use ssx3_ee as ee;

    use ssx3_ee::s02;

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

    pub(crate) fn wf(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
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

    /// Real GatherRiderState, Rider_GetMainState, sin and cos; only target
    /// virtuals are fake. Ten cases cross five direction exits with first /
    /// subsequent update. Every gp pool slot read by the update and gather
    /// varies with the seed, so hard-coded tuning constants are observable.
    fn relative_update_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x160bf8, 7);
        elf_image(c);
        c.bus.write32(crate::tick::APPMAN_PTR, 0);
        let (this, target, vt, rider, state) = (
            DATA,
            DATA + 0x1000,
            DATA + 0x2000,
            DATA + 0x3000,
            DATA + 0x4000,
        );
        c.g[A0][0] = this as u64;
        c.pc = 0x0016_0bf8;
        c.bus.write32(this + 0x30, target);
        c.bus.write32(target, vt);
        c.bus.write32(target + 4, rider);
        c.bus.write32(rider + 0x77c, state);
        c.bus.write32(state + 0xde0, 0);
        for (i, slot) in [0x18u32, 0xa0, 0x88, 0x90, 8, 0x20].into_iter().enumerate() {
            c.bus.write16(vt + slot, (i as u16) * 0x10);
            c.bus.write32(vt + slot + 4, FAKE + i as u32 * 0x10);
        }
        c.bus.write32(LOG_RESULTS + 4, 4);
        // Keep the first ten cases complete, then exercise each unwind index.
        c.bus.write32(
            LOG_UNWIND_AT,
            if seed < 10 || seed % 3 != 0 {
                u32::MAX
            } else {
                (seed / 3) % 7
            },
        );
        for neg in (0x6b74..=0x6bac).step_by(4).chain([0x6af8, 0x6afc]) {
            let at = GAME_GP - neg;
            let v = f32::from_bits(c.bus.read32(at));
            let scale = [0.8f32, 1.0, 1.2][(r.next_u32() % 3) as usize];
            c.bus.write32(at, (v * scale).to_bits());
        }
        let case = seed % 5;
        let vec = |x: f32, y: f32, z: f32| [x.to_bits(), y.to_bits(), z.to_bits(), 0];
        let velocity = match case {
            0 => vec(60.0, 10.0, 0.0),
            1 => vec(0.0, 60.0, 0.0),
            _ => vec(0.0, 0.0, 60.0),
        };
        put_vec(c, SP - 0x250, velocity);
        put_vec(c, this + 0x1b0, velocity);
        put_vec(c, this + 0x100, velocity);
        let forward = match case {
            2 => vec(1.0, 0.0, 0.0),
            3 => vec(0.0, 1.0, 0.0),
            _ => vec(0.0, 0.0, 1.0),
        };
        put_vec(c, SP - 0xc0, forward);
        put_vec(c, 0x004f_f140, vec(1.0, 0.0, 0.0));
        put_vec(c, 0x004f_f160, vec(0.0, 0.0, 1.0));
        put_vec(c, 0x004f_f120, vec(0.0, 0.0, 1.0));
        // Target position outputs, plus the initial focus/eye histories.
        for at in [
            SP - 0x1d0,
            SP - 0xa0,
            this + 0x20,
            this + 0x40,
            this + 0x3a0,
            this + 0x3b0,
        ] {
            let v = [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0];
            put_vec(c, at, v);
        }
        c.bus.write32(this + 0x390, r.f32(3.0));
        c.bus.write32(this + 0x394, (seed / 5) % 2);
    }

    /// Compare the actual eye-offset blend on a piecewise-constant direction
    /// schedule. Gather's horizontal heading and target focus are held fixed
    /// to that input, isolating this recurrence from the other camera slews.
    #[test]
    fn relative_chase_camera_eye_follows_the_rate() {
        let sample = |hz: u32| {
            let mut c = crate::difftest::fresh();
            relative_update_setup(&mut c, 0);
            elf_image(&mut c);
            crate::tick::set_for_test(&mut c, hz);
            let keep = crate::tick::per_tick_at(hz, 0x0049_c578, 0x3f78_51ec);
            let add = crate::tick::per_tick_values(hz)
                .into_iter()
                .find(|x| x.0 == 0x0049_c57c)
                .map(|x| x.1)
                .unwrap_or(0x3cf5_c280);
            if hz == 60 {
                assert_eq!(keep, 0x3f78_51ec);
                assert_eq!(add, 0x3cf5_c280);
            }
            c.bus.write32(0x0049_c578, keep);
            c.bus.write32(0x0049_c57c, add);
            c.bus.write32(DATA + 0x390, 0);
            c.bus.write32(DATA + 0x394, 0);
            put_vec(&mut c, DATA + 0x3b0, [0; 4]);
            let mut out = Vec::new();
            for n in 0..=hz {
                let v: [f32; 4] = if n == 0 {
                    [40.0, 0.0, 0.0, 0.0]
                } else if n <= hz / 2 {
                    [0.0, 40.0, 0.0, 0.0]
                } else {
                    [-40.0, 0.0, 0.0, 0.0]
                };
                let bits = v.map(f32::to_bits);
                put_vec(&mut c, SP - 0x250, bits);
                put_vec(&mut c, DATA + 0x1b0, bits);
                put_vec(&mut c, DATA + 0x100, bits);
                put_vec(&mut c, SP - 0x1d0, [0; 4]);
                put_vec(&mut c, SP - 0xa0, [0; 4]);
                c.bus.write32(fakes::LOG_COUNT, 0);
                c.bus.write32(fakes::LOG_NEXT, 0);
                c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                c.g[A0][0] = DATA as u64;
                c.g[ssx3_ee_rt::cpu::reg::SP][0] = SP as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                relative_chase_camera_algorithm_update(&mut c);
                assert_eq!(c.pc, crate::difftest::RA);
                assert_eq!(c.g[ssx3_ee_rt::cpu::reg::SP][0], SP as u64);
                if n > 0 && n % (hz / 4) == 0 {
                    let eye: [f32; 3] = std::array::from_fn(|i| {
                        f32::from_bits(c.bus.read32(DATA + 0x40 + 4 * i as u32))
                    });
                    out.push(eye);
                }
            }
            out
        };
        let stock = sample(60);
        println!("RELATIVE_RATE eye hz=60 samples={stock:?}");
        for hz in [120, 240] {
            let actual = sample(hz);
            println!("RELATIVE_RATE eye hz={hz} samples={actual:?}");
            for (n, (a, b)) in actual.iter().zip(&stock).enumerate() {
                for axis in 0..3 {
                    assert!(
                        (a[axis] - b[axis]).abs() < 0.02,
                        "{hz} Hz checkpoint {n} axis {axis}: {a:?} vs {b:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn relative_chase_camera_algorithm_update_matches() {
        covered(
            ee::s01::f_00160bf8__generated,
            relative_chase_camera_algorithm_update,
            240,
            &relative_update_setup,
        );
        // The ten complete paths: five direction exits, each with and without history.
        crate::difftest::require_paths(
            ee::s01::f_00160bf8__generated,
            240,
            &[504, 517, 535, 543, 545, 562, 573, 607, 597, 603],
            &relative_update_setup,
        );
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

    // ------------------------------------------------ ChaseCameraAlgorithm_BuildOutputView

    /// `ChaseCameraAlgorithm_BuildOutputView`: random base matrix, axes,
    /// offset and angles through the real `Math_SinCos` and
    /// `Mat4_ToPosQuat`.
    #[test]
    fn chase_camera_build_output_view_matches() {
        check(
            ssx3_ee::s02::f_00166f90__generated,
            chase_camera_build_output_view,
            200,
            &|c, seed| {
                let mut r = Rng(0x0016_6f90 ^ (seed as u64) << 16);
                crate::difftest::elf_image(c);
                scramble(c, &mut r);
                c.g[GP][0] = sx(crate::difftest::GAME_GP);
                c.pc = 0x0016_6f90;
                let this = DATA;
                c.g[A0][0] = sx(this);
                for i in 0..0x20 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                for i in 0..4 {
                    c.bus.write32(this + 0x40 + 4 * i, r.f32(50.0));
                }
                c.bus.write32(this + 0x50, r.f32(3.5));
                c.bus.write32(this + 0x54, r.f32(1.5));
                // Base matrix: a rotation-ish matrix with a translation.
                for i in 0..16u32 {
                    let v = if i % 5 == 0 {
                        1.0f32.to_bits()
                    } else {
                        r.f32(0.7)
                    };
                    c.bus.write32(0x004f_f1a0 + 4 * i, v);
                }
                let axes = [[1.0f32, 0.0, 0.0], [0.0, 1.0, 0.0], [0.6, 0.0, 0.8]];
                for (k, a) in [0x004f_f150u32, 0x004f_f160].into_iter().enumerate() {
                    let axis = axes[(k + seed as usize) % 3];
                    for (j, v) in axis.iter().enumerate() {
                        c.bus.write32(a + 4 * j as u32, v.to_bits());
                    }
                }
                for (j, v) in [0.0f32, 0.0, 0.0, 1.0].iter().enumerate() {
                    c.bus.write32(0x004f_f130 + 4 * j as u32, v.to_bits());
                }
                // The stack words the original parks and copies (sp+0x70).
                for k in 0..0x48u32 {
                    c.bus
                        .write32(crate::difftest::SP - 0x120 + 4 * k, r.next_u32());
                }
            },
        );
    }

    // -- 0x001668b8 ChaseCameraAlgorithm_RotateByPitch ------------------------

    /// `ChaseCameraAlgorithm_RotateByPitch`: random and edge-case angles at
    /// +0x50/+0x54 (the game image provides the identity, the axes and
    /// `Math_SinCos`'s tables); the stack scratch starts random.
    #[test]
    fn chase_camera_algorithm_rotate_by_pitch_matches() {
        use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA, SP};
        const THIS: u32 = DATA;
        const OUT: u32 = DATA + 0x100;
        check(
            ee::s02::f_001668b8__generated,
            chase_camera_algorithm_rotate_by_pitch,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = Rng(0x0016_68b8 ^ (seed as u64) << 20 ^ 0x2545_f491);
                scramble(c, &mut r);
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                c.pc = 0x0016_68b8;
                fill(c, &mut r, SP - 0x140, 0x50);
                let angle = |r: &mut Rng| {
                    if r.next_u32() % 8 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(7.0)
                    }
                };
                let pitch = angle(&mut r);
                let yaw = angle(&mut r);
                c.bus.write32(THIS + 0x50, pitch);
                c.bus.write32(THIS + 0x54, yaw);
                fill(c, &mut r, OUT, 4);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(OUT);
            },
        );
    }

    pub(crate) fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// ChaseCameraAlgorithm_ApplyShake: shake requested or not (with the
    /// last request set or not), time left around 0..2, every channel's
    /// timer before 0, inside or past its period (so re-draws through the
    /// real `Rand_NextMain` and all ease branches run), and the distance
    /// scale off, near or far (150 units).
    #[test]
    fn chase_camera_algorithm_apply_shake_matches() {
        check(
            ee::s01::f_00165938__generated,
            chase_camera_algorithm_apply_shake,
            300,
            &|c, seed| {
                let mut r = Rng(0x1659_38 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x0016_5938;
                c.g[GP][0] = GAME_GP as u64;
                let cam = DATA;
                // The main random stream's state.
                for k in 0..6 {
                    c.bus.write32(0x004f_f018 + 4 * k, r.next_u32());
                }
                c.bus.write32(GAME_GP - 0x68d0, (1.0f32 / 60.0).to_bits());
                c.bus.write32(GAME_GP - 0x68cc, (1.0f32 / 60.0).to_bits());
                c.bus.write32(GAME_GP - 0x68c8, 0.5f32.to_bits());
                c.bus.write32(GAME_GP - 0x68c4, 0.1f32.to_bits());
                c.bus.write32(cam + 0x2ec, r.next_u32() % 2);
                let left = [0.0f32, 0.01, 0.5, 1.9, 2.5][(r.next_u32() % 5) as usize];
                c.bus.write32(cam + 0x2b4, left.to_bits());
                for i in 0..6u32 {
                    let (timer, period) = [
                        (0x278, 0x254),
                        (0x27c, 0x258),
                        (0x280, 0x25c),
                        (0x2a8, 0x284),
                        (0x2ac, 0x288),
                        (0x2b0, 0x28c),
                    ][i as usize];
                    let p = 0.2 + (r.next_u32() % 100) as f32 * 0.01;
                    c.bus.write32(cam + period, p.to_bits());
                    let t = match r.next_u32() % 4 {
                        0 => -0.1f32,
                        1 => p * 0.5,
                        2 => p + 0.3,
                        _ => p - 0.005,
                    };
                    c.bus.write32(cam + timer, t.to_bits());
                }
                for off in (0x260..0x26c).step_by(4).chain((0x26c..0x278).step_by(4)) {
                    c.bus.write32(cam + off, r.f32(3.0));
                }
                for off in (0x290..0x2a8).step_by(4) {
                    c.bus.write32(cam + off, r.f32(3.0));
                }
                put_vec(
                    c,
                    cam + 0x20,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
                );
                let far = if r.next_u32() % 2 == 0 { 50.0 } else { 400.0 };
                put_vec(
                    c,
                    cam + 0x40,
                    [r.f32(far), r.f32(far), r.f32(far), r.f32(1.0)],
                );
                c.g[A0][0] = sx(cam);
                c.g[A1][0] = if seed % 3 == 0 {
                    r.next_u32() as u64 | 1
                } else {
                    0
                };
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { 1 };
                c.f[12] = 0.5f32.to_bits();
                c.f[13] = 1.0f32.to_bits();
                c.f[14] = r.f32(2.0);
            },
        );
    }

    /// Freeze the filter output with zero gains, so this tests the retained
    /// share alone, including the fourth-root transform in states 11..13.
    /// The target turns at time 0 and 0.5 s on the same game-time schedule.
    #[test]
    fn chase_camera_heading_blend_follows_the_rate() {
        for state in [0u32, 11, 12, 13] {
            for filtered in [0.0f32, 0.2, 0.5] {
                let sample = |hz: u32| {
                    let mut c = crate::difftest::fresh();
                    machine(&mut c, 0, 0x162c78, 0);
                    elf_image(&mut c);
                    crate::tick::set_for_test(&mut c, hz);
                    let (info, rig, vt, rider) =
                        (DATA + 0x800, DATA + 0x1000, DATA + 0x1100, DATA + 0x1200);
                    c.bus.write32(DATA + chase::RIG, rig);
                    c.bus.write32(rig, vt);
                    c.bus.write32(rig + 4, rider);
                    c.bus.write16(vt + 0x18, 0);
                    c.bus.write32(vt + 0x1c, FAKE);
                    c.bus.write32(rider + chase::RIDER_STATE, state);
                    c.bus.write32(info + chase_info::SPEED, 40.0f32.to_bits());
                    put_vec(&mut c, DATA + chase::HEADING, [1.0f32.to_bits(), 0, 0, 0]);
                    for i in 0..5 {
                        c.bus
                            .write32(DATA + chase::HIST + chase::H_B + 4 * i, filtered.to_bits());
                    }
                    let mut out = Vec::new();
                    for n in 1..=hz {
                        let v: [f32; 4] = if n <= hz / 2 {
                            [0.0, 40.0, 0.0, 0.0]
                        } else {
                            [-40.0, 0.0, 0.0, 0.0]
                        };
                        put_vec(&mut c, info + chase_info::DIR, v.map(f32::to_bits));
                        put_vec(&mut c, SP - 0xa0, [1.0f32.to_bits(), 0, 0, 0]);
                        c.bus.write32(fakes::LOG_COUNT, 0);
                        c.bus.write32(fakes::LOG_NEXT, 0);
                        c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                        c.g[A0][0] = DATA as u64;
                        c.g[A1][0] = info as u64;
                        c.g[RA][0] = crate::difftest::RA as u64;
                        c.g[ssx3_ee_rt::cpu::reg::SP][0] = SP as u64;
                        chase_camera_update_chase_heading(&mut c);
                        assert_eq!(c.pc, crate::difftest::RA);
                        assert_eq!(c.g[ssx3_ee_rt::cpu::reg::SP][0], SP as u64);
                        if n % (hz / 4) == 0 {
                            let heading: [f32; 4] = std::array::from_fn(|i| {
                                f32::from_bits(c.bus.read32(DATA + chase::HEADING + 4 * i as u32))
                            });
                            out.push(heading);
                        }
                    }
                    out
                };
                let stock = sample(60);
                println!("HEADING_RATE state={state} filter={filtered} hz=60 samples={stock:?}");
                for hz in [120, 240] {
                    let actual = sample(hz);
                    println!(
                        "HEADING_RATE state={state} filter={filtered} hz={hz} samples={actual:?}"
                    );
                    for (n, (a, b)) in actual.iter().zip(&stock).enumerate() {
                        for axis in 0..4 {
                            assert!((a[axis] - b[axis]).abs() < 0.001,
                                "state {state}, filter {filtered}, {hz} Hz checkpoint {n}: {a:?} vs {b:?}");
                        }
                    }
                }
            }
        }
    }

    /// ChaseCameraAlgorithm_UpdateChaseHeading: slow (speed below 100 km/h)
    /// and fast paths; the rig direction short (falls back to the last one)
    /// or not; a zero heading (angle 0) or the asin path with the cosine
    /// clamped low/high; the history index in range; rider states in and
    /// out of 0xb..0xd. The virtual call is a fake that sometimes unwinds;
    /// Math_Asin runs for real on the game's constants (ELF image).
    #[test]
    fn chase_camera_update_chase_heading_matches() {
        covered(
            ssx3_ee::s01::f_00162c78__generated,
            chase_camera_update_chase_heading,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x162c78, 1);
                elf_image(c);
                c.pc = 0x0016_2c78;
                let this = DATA;
                let info = DATA + 0x800;
                let rig = DATA + 0x1000;
                let vt = DATA + 0x1100;
                let rider = DATA + 0x1200;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = info as u64;
                fill(c, &mut r, this, 0x100);
                fill(c, &mut r, info, 0x20);
                let v = |r: &mut Rng, big: bool| {
                    let k = if big { 3.0 } else { 0.3 };
                    [r.f32(k), r.f32(k), r.f32(k), r.f32(k * 0.5)]
                };
                c.bus.write32(this + chase::RIG, rig);
                c.bus.write32(rig, vt);
                c.bus.write32(rig + 4, rider);
                c.bus.write16(vt + 0x18, (r.next_u32() as u16) & 0x7c);
                c.bus.write32(vt + 0x1c, FAKE);
                let state = match seed % 4 {
                    0 => 0xb + r.next_u32() % 3,
                    _ => r.next_u32() % 0x20,
                };
                c.bus.write32(rider + chase::RIDER_STATE, state);
                let speed = if seed % 7 == 0 {
                    10.0f32
                } else {
                    30.0 + (r.next_u32() % 40) as f32
                };
                c.bus.write32(info + chase_info::SPEED, speed.to_bits());
                let d = v(&mut r, true);
                put_vec(c, info + chase_info::DIR, d);
                let h = if seed % 9 == 1 {
                    [0; 4]
                } else {
                    v(&mut r, true)
                };
                put_vec(c, this + chase::HEADING, h);
                let l = v(&mut r, seed % 2 == 0);
                put_vec(c, this + chase::LAST_DIR, l);
                // The rig's direction (the fake leaves sp+0x10 as it is).
                let g = v(&mut r, seed % 3 != 0);
                put_vec(c, SP - 0xb0 + 0x10, g);
                let hist = this + chase::HIST;
                for i in 0..15 {
                    c.bus.write32(hist + 4 * i, r.f32(0.5));
                }
                c.bus.write32(hist + chase::H_IDX, r.next_u32() % 5);
                put_words(c, hist + chase::H_K0, &[r.f32(1.0), r.f32(1.0), r.f32(1.0)]);
            },
        );
    }

    // ---------------------------------------------- GameCamera_UpdateStreamView

    /// `GameCamera_UpdateStreamView`: race mode 1 or not, view distances
    /// negative / small / mid / huge / edge (both clamps, both clearance
    /// tests, and the no-clearance result of a non-positive radius); the
    /// real streamer view setter and clearance query run on a world cache
    /// with no streamer nodes.
    #[test]
    fn game_camera_update_stream_view_matches() {
        check(
            ee::s01::f_0015ec98__generated,
            game_camera_update_stream_view,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_ec98, 0);
                elf_image(c);
                c.pc = 0x0015_ec98;
                // App -> game module -> race state (mode at +0x10).
                let app = DATA + 0x1000;
                let module = DATA + 0x1100;
                let race = DATA + 0x1200;
                c.bus.write32(GAME_GP - 0x848, app);
                c.bus.write32(app + 0x84, module);
                c.bus.write32(module + 0x84, race);
                c.bus.write32(
                    race + 0x10,
                    if seed % 4 == 0 { 1 } else { r.next_u32() % 3 },
                );
                // The world cache: streamer at +0x10 with no nodes
                // (`streamer+4 -> {+8: 0}`); the frame clock at gp+0x2a74.
                let cache_ptr = DATA + 0x2000;
                let cache = DATA + 0x3000;
                let nodes = DATA + 0x2100;
                let clock = DATA + 0x2200;
                c.bus.write32(GAME_GP + 0x16c8, cache_ptr);
                c.bus.write32(cache_ptr, cache);
                c.bus.write32(cache + 0x10 + 4, nodes);
                c.bus.write32(nodes + 8, 0);
                c.bus.write32(GAME_GP + 0x2a74, clock);
                c.bus.write32(clock + 0x18, r.next_u32());
                let cam = DATA + 0x8000;
                let dist: u32 = match seed % 6 {
                    0 => (-((r.next_u32() % 500) as f32)).to_bits(),
                    1 => r.f32(2.0),
                    2 => r.f32(12000.0),
                    3 => r.f32(80000.0),
                    4 => r.edge_f32(),
                    _ => ((r.next_u32() % 40000) as f32).to_bits(),
                };
                c.bus.write32(cam + 0x8, dist);
                for k in 0..4 {
                    c.bus.write32(cam + 0x20 + 4 * k, r.f32(5000.0));
                }
                c.bus.write32(cam + 0xb0, r.next_u32() % 4);
                c.bus.write32(cam + 0xb4, r.next_u32());
                c.g[A0][0] = sx(cam);
            },
        );
    }

    // --------------------------------------------- cWorldCache_StreamerSetView

    /// `cWorldCache_StreamerSetView`: streamer indices 0..7, random view
    /// positions and clearances (negative and edge values too: the `max`
    /// with 0).
    #[test]
    fn world_cache_streamer_set_view_matches() {
        use crate::difftest::{check, put_vec, scramble, Rng, DATA};
        check(
            ee::s13::f_003a9658__generated,
            world_cache_streamer_set_view,
            300,
            &|c, seed| {
                let mut r = Rng(0x003a_9658 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003a_9658;
                let cache = DATA;
                let pos = DATA + 0x2000;
                c.g[A0][0] = sx(cache);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = sx(pos);
                c.f[12] = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(300.0)
                };
                put_vec(
                    c,
                    pos,
                    [r.f32(5000.0), r.f32(5000.0), r.f32(5000.0), r.next_u32()],
                );
            },
        );
    }

    // -- 0x00166530 ChaseCameraAlgorithm_CheckDistanceReset_Wrapper ------------

    /// `ChaseCameraAlgorithm_CheckDistanceReset_Wrapper` against the
    /// generated body, with the real `CheckDistanceReset`: camera-to-target
    /// distances below 20, between 20 and the limit (gp-0x6778), and above
    /// (the reset path, whose rider virtual is a fake that sometimes
    /// unwinds).
    #[test]
    fn chase_camera_check_distance_reset_wrapper_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::{put_vec, DATA, GAME_GP};
        const CAM: u32 = DATA + 0x20_0000;
        const RIDER: u32 = DATA + 0x20_1000;
        const BODY: u32 = DATA + 0x20_2000;
        covered(
            ee::s02::f_00166530__generated,
            chase_camera_check_distance_reset_wrapper,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0016_6530, 1);
                c.pc = 0x0016_6530;
                c.bus.write32(GAME_GP - 0x6778, 100.0f32.to_bits());
                let span = [5.0f32, 40.0, 400.0][(r.next_u32() % 3) as usize];
                let p = [r.f32(span), r.f32(span), r.f32(span), r.f32(1.0)];
                put_vec(c, CAM + 0x20, p);
                put_vec(
                    c,
                    CAM + 0x40,
                    [r.f32(span), r.f32(span), r.f32(span), r.f32(1.0)],
                );
                c.bus.write32(CAM + 0x30, RIDER);
                let vt = vtable(c, &mut r, &[0x8]);
                c.bus.write32(RIDER, vt);
                c.bus.write32(RIDER + 4, BODY);
                put_vec(c, BODY + 0x1b0, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                put_vec(
                    c,
                    0x0050_0000 - 0xea0,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0],
                );
                c.g[A0][0] = sx(CAM);
            },
        );
    }

    pub(crate) use crate::game::object::railmodifier::tests::oc313_vec;

    /// Landing angle: zero vectors (both early outs), tiny vectors (below
    /// the 0.001 length), parallel/antiparallel vectors (clamp edges) and
    /// random ones; the real `Math_Asin` runs with its `.lit4` constants.
    #[test]
    fn rider_camera_target_v23_matches() {
        check(
            ee::s01::f_0015f780__generated,
            rider_camera_target_v23,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_f780, 0);
                c.pc = 0x0015_f780;
                // .lit4: the function's and Math_Asin's.
                let lit = |c: &mut Cpu, off: i32, v: f32| {
                    c.bus.write32(GAME_GP.wrapping_add(off as u32), v.to_bits())
                };
                lit(c, -0x6c50, 0.001);
                lit(c, -0x6c4c, std::f32::consts::FRAC_PI_2);
                lit(c, -0x2e60, -std::f32::consts::FRAC_1_SQRT_2);
                lit(c, -0x2e58, std::f32::consts::FRAC_1_SQRT_2);
                lit(c, -0x2e54, std::f32::consts::FRAC_PI_2);
                put_words(
                    c,
                    GAME_GP + 0xdf0,
                    &[
                        0.167_702_27f32.to_bits(),
                        0.067_658_506f32.to_bits(),
                        0.052_005_03f32.to_bits(),
                        0.054_984_73f32.to_bits(),
                    ],
                );
                let target = DATA;
                let rider = DATA + 0x100;
                let landing = DATA + 0x1000;
                c.bus.write32(target + RCT_V23_RIDER, rider);
                c.bus.write32(rider + RCT_V23_LANDING, landing);
                c.g[A0][0] = sx(target);
                let edge = seed % 7 == 6;
                let mut n = oc313_vec(&mut r, edge);
                let mut v = oc313_vec(&mut r, edge);
                match seed % 8 {
                    0 => n = [0; 4],
                    1 => v = [0; 4],
                    2 => {
                        // tiny: |n||v| < 0.001
                        for x in n.iter_mut().chain(v.iter_mut()) {
                            *x = r.f32(0.01);
                        }
                    }
                    3 => {
                        // v parallel or antiparallel to n
                        let k = if r.next_u32() & 1 == 0 { 3.0f32 } else { -2.5 };
                        for i in 0..4 {
                            v[i] = (f32::from_bits(n[i]) * k).to_bits();
                        }
                    }
                    _ => {}
                }
                put_vec(c, landing + RCT_V23_LANDING_VELOCITY, v);
                put_vec(c, landing + RCT_V23_LANDING_NORMAL, n);
            },
        );
    }
}
