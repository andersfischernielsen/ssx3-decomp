//! `src/ai/control/handplantcontrol.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::rider_save_state_raw;
pub(crate) use crate::game::ai::ai::rider_state_save_forward;
pub(crate) use crate::game::ai::ai::rider_state_save_self_bytes;
pub(crate) use crate::game::ai::ai::virtual_call_slot;
pub(crate) use crate::game::ai::ai::SAVE_STATE_STREAM_WRITE;
pub(crate) use crate::game::ai::computer::fr_2 as fr;
pub(crate) use crate::game::ai::computer::gp_2 as gp_3;
pub(crate) use crate::game::ai::computer::ldf_2 as ldf;
pub(crate) use crate::game::ai::computer::li_f;
pub(crate) use crate::game::ai::computer::lt_2 as lt;
pub(crate) use crate::game::ai::computer::lw;
pub(crate) use crate::game::ai::computer::setf_2 as setf;
pub(crate) use crate::game::ai::rider::at;
pub(crate) use crate::game::ai::rider::copy_contact;
pub(crate) use crate::game::ai::rider::epilogue;
pub(crate) use crate::game::ai::rider::gp_2 as gp;
pub(crate) use crate::game::ai::rider::p;
pub(crate) use crate::game::ai::rider::prologue;
pub(crate) use crate::game::ai::rider::r;
pub(crate) use crate::game::ai::rider::rate_step_60;
pub(crate) use crate::game::ai::rider::slt;
pub(crate) use crate::game::ai::rider::sp_2;
pub(crate) use crate::game::ai::rider::vcall;
pub(crate) use crate::game::ai::rider::Ops;
pub(crate) use crate::game::ai::rider::RODATA_NORMAL2;
pub(crate) use crate::game::ai::rider::RODATA_UP;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_8;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw;
pub(crate) use crate::game::fe::festateload::at as at_2;
pub(crate) use crate::game::fe::festateloadhint::gp as gp_7;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::splashmanager::gpo;
pub(crate) use crate::game::intersect::worldsphtree::sp_2 as sp_3;
pub(crate) use crate::game::movie::movieplayer::gp as gp_5;
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
        0x0013_99a8,
        rider_handplant_state_load_state,
        "RiderHandplantState_LoadState",
    ),
    (0x0013_9a18, rider_air_state_exit, "RiderAirState_Exit"),
    (0x0013_9a20, rider_air_state_update, "RiderAirState_Update"),
    (
        0x0013_a7b0,
        rider_air_state_cast_board_ray,
        "RiderAirState_CastBoardRay",
    ),
    (
        0x0013_a8f8,
        rider_air_state_play_land_anim,
        "RiderAirState_PlayLandAnim",
    ),
    (
        0x0013_ace8,
        rider_air_state_load_state,
        "RiderAirState_LoadState",
    ),
    (
        0x0013_ad20,
        rider_railslide_state_enter,
        "RiderRailslideState_Enter",
    ),
    (
        0x0013_c618,
        rider_railslide_state_load_state,
        "RiderRailslideState_LoadState",
    ),
    (
        0x0013_c7a8,
        rider_ground_state_enter,
        "RiderGroundState_Enter",
    ),
    (
        0x0013_c878,
        rider_ground_state_compute_friction_accel,
        "RiderGroundState_ComputeFrictionAccel",
    ),
    (
        0x0013_d028,
        rider_ground_state_compute_carve_accel,
        "RiderGroundState_ComputeCarveAccel",
    ),
    (
        0x0013_d1b8,
        rider_ground_state_probe_ground,
        "RiderGroundState_ProbeGround",
    ),
    (
        0x0013_d818,
        rider_ground_state_update,
        "RiderGroundState_Update",
    ),
    (
        0x0013_f410,
        rider_ground_state_exit,
        "RiderGroundState_Exit",
    ),
    (
        0x0013_f880,
        rider_ground_state_load_state,
        "RiderGroundState_LoadState",
    ),
    (0x0014_0738, rider_init_data, "Rider_InitData"),
    (0x0014_0bc8, rider_is_computer, "Rider_IsComputer"),
    (0x0014_4670, flags64_move_to_second, "Flags64_MoveToSecond"),
    (0x0014_46a0, flags64_test, "Flags64_Test"),
    (0x0014_46b8, flags64_test_second, "Flags64_TestSecond"),
    (0x0014_48a8, rider_is_control_mode3, "Rider_IsControlMode3"),
    (
        0x0013_c948,
        rider_ground_state_compute_boost_accel,
        "RiderGroundState_ComputeBoostAccel",
    ),
    (
        0x0013_ccf0,
        rider_ground_state_compute_drag,
        "RiderGroundState_ComputeDrag",
    ),
    (
        0x0013_f178,
        rider_ground_state_post_update,
        "RiderGroundState_PostUpdate",
    ),
    (
        0x0013_f488,
        rider_ground_state_collide_walls,
        "RiderGroundState_CollideWalls",
    ),
    (
        0x0013_f8f8,
        obj_ptr_array_merge_sort,
        "ObjPtrArray_MergeSort",
    ),
    (
        0x0013_9970,
        rider_handplant_state_save_state,
        "RiderHandplantState_SaveState",
    ),
    (
        0x0013_acb0,
        rider_air_state_save_state,
        "RiderAirState_SaveState",
    ),
    (
        0x0013_c5e0,
        rider_railslide_state_save_state,
        "RiderRailslideState_SaveState",
    ),
    (
        0x0013_f848,
        rider_ground_state_save_state,
        "RiderGroundState_SaveState",
    ),
    (
        0x0013_a968,
        rider_air_state_play_land_anim2,
        "RiderAirState_PlayLandAnim2",
    ),
];

/// Direct call targets, as the generated body calls them (the shard alias,
/// which dispatches through its override slot).
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::computer::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::ai::computer::callee::GET_MAIN_STATE as GET_MAIN_STATE_2;
    pub(crate) use crate::game::ai::computer::callee::GET_SUB_STATE as GET_SUB_STATE_2;
    pub(crate) use crate::game::ai::rider::callee::ADD_BOOST;
    pub(crate) use crate::game::ai::rider::callee::AI_GET_FRAME_COUNTER_2 as AI_GET_FRAME_COUNTER;
    pub(crate) use crate::game::ai::rider::callee::ALIGN_STANCE_TO_RAIL;
    pub(crate) use crate::game::ai::rider::callee::ANIM_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::ai::rider::callee::ANIM_LAYER_SEQUENCE;
    pub(crate) use crate::game::ai::rider::callee::ANIM_PLAY_ANIM;
    pub(crate) use crate::game::ai::rider::callee::ANIM_ROTATE_ROOTS;
    pub(crate) use crate::game::ai::rider::callee::BANK_TRICK_BOOST;
    pub(crate) use crate::game::ai::rider::callee::BE_CHAR_ATTRIB5_FACTOR;
    pub(crate) use crate::game::ai::rider::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::ai::rider::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::ai::rider::callee::BOUNCE_OFF_WALL;
    pub(crate) use crate::game::ai::rider::callee::BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::ai::rider::callee::CAMERA_TRIGGER_UPDATE_FOR_RIDER;
    pub(crate) use crate::game::ai::rider::callee::COLL_HULL_CTOR;
    pub(crate) use crate::game::ai::rider::callee::COLL_HULL_DTOR;
    pub(crate) use crate::game::ai::rider::callee::COLL_SEGMENT_CTOR;
    pub(crate) use crate::game::ai::rider::callee::DEFLECT_ALONG_WALL;
    pub(crate) use crate::game::ai::rider::callee::FIND_RAIL;
    pub(crate) use crate::game::ai::rider::callee::FIND_WALL_CONTACT;
    pub(crate) use crate::game::ai::rider::callee::FLAGS64_TEST;
    pub(crate) use crate::game::ai::rider::callee::GET_BUMP_STRENGTH;
    pub(crate) use crate::game::ai::rider::callee::GET_MAIN_STATE;
    pub(crate) use crate::game::ai::rider::callee::GET_SUB_STATE;
    pub(crate) use crate::game::ai::rider::callee::HANDLE_IMPACT;
    pub(crate) use crate::game::ai::rider::callee::HANDLE_WALL_RAIL_CONTACT;
    pub(crate) use crate::game::ai::rider::callee::IS_ON_GROUND;
    pub(crate) use crate::game::ai::rider::callee::LAND_TRICK;
    pub(crate) use crate::game::ai::rider::callee::MATH_ATAN;
    pub(crate) use crate::game::ai::rider::callee::MATH_SIN_COS;
    pub(crate) use crate::game::ai::rider::callee::OVERLAP_LIST_COLLIDE_OBJECTS;
    pub(crate) use crate::game::ai::rider::callee::OVERLAP_LIST_COLLIDE_VOLUME;
    pub(crate) use crate::game::ai::rider::callee::OVERLAP_LIST_FIND_NEAREST_RAIL;
    pub(crate) use crate::game::ai::rider::callee::PLAY_WALL_HIT_SOUND;
    pub(crate) use crate::game::ai::rider::callee::RAILSLIDE_COMPUTE_ENTRY_ANGLE;
    pub(crate) use crate::game::ai::rider::callee::RAILSLIDE_PLAY_ENTER_ANIM;
    pub(crate) use crate::game::ai::rider::callee::RESOLVE_RIDER_COLLISION;
    pub(crate) use crate::game::ai::rider::callee::ROTATE_AROUND_AXIS;
    pub(crate) use crate::game::ai::rider::callee::SET_MAIN_STATE;
    pub(crate) use crate::game::ai::rider::callee::SET_SUB_STATE;
    pub(crate) use crate::game::ai::rider::callee::SPHERE_TREE_INTERSECT;
    pub(crate) use crate::game::ai::rider::callee::TRANSLATE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX;
    pub(crate) use crate::game::intersect::worldsphtree::callee::COLLIDE_VOLUME_OBJECT_2 as COLLIDE_VOLUME_OBJECT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::FLEX_RAIL_FIND_CLOSEST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::FOCUS_MAN_COLLECT_OCTREE_NODE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_BUILTIN;
    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_PAD_CHANNEL;
    pub(crate) use crate::game::intersect::worldsphtree::callee::RAIL_FIND_NEAREST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::RAIL_MODIFIER_FIND_CLOSEST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WP_DATA_FIND_CELL_INDEX;
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_APPLY_WIND;
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_PREDICTOR_UPDATE;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_RESET_BOOST_PITCH;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_CHAR_ATTRIB_FACTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILD_OCCLUDER_PLANES;
    pub(crate) use crate::game::movie::movieplayer::callee::MATH_DET3X3;
    pub(crate) use crate::game::movie::movieplayer::callee::OVERLAP_LIST_FIND_CLOSEST;
    pub(crate) use crate::game::movie::movieplayer::callee::PACK_POSITION;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL128_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL16_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL256_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL32_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL512_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL64_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL8_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::QSORT;
    pub(crate) use crate::game::movie::movieplayer::callee::REMOVE_HUD_MESSAGE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_GET_SUBSTATE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_ORIENTATION_IMPLICIT;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_UPDATE_ORIENTATION;
    pub(crate) use crate::game::movie::movieplayer::callee::SET_TIMED_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::SHOW_HUD_NUMBER;
    pub(crate) use crate::game::movie::movieplayer::callee::VU0_UPLOAD_OCCLUDER;

    pub const CAST_BOARD_RAY: Callee = Callee {
        addr: 0x0013_a7b0,
        f: ee::s01::RiderAirState_CastBoardRay,
    };

    pub const PLAY_LAND_ANIM: Callee = Callee {
        addr: 0x0013_a8f8,
        f: ee::s01::RiderAirState_PlayLandAnim,
    };

    pub const PLAY_LAND_ANIM2: Callee = Callee {
        addr: 0x0013_a968,
        f: ee::s01::RiderAirState_PlayLandAnim2,
    };

    pub const COLLIDE_WALLS: Callee = Callee {
        addr: 0x0013_aa48,
        f: ee::s01::RiderAirState_CollideWalls,
    };

    pub const FLAGS64_TEST_2: Callee = Callee {
        addr: 0x0014_46a0,
        f: ee::s01::Flags64_Test,
    };

    pub const RIDER_UPDATE_BOUNDS: Callee = Callee {
        addr: 0x0011_e150,
        f: ee::s00::Rider_UpdateBounds,
    };

    pub const BE_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    pub const BE_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };

    pub const ATTRIB6_FACTOR: Callee = Callee {
        addr: 0x0014_9120,
        f: ee::s01::BECharInt_GetAttrib6Factor,
    };

    pub const RIDER_GROUND_STATE_COMPUTE_FRICTION_ACCEL: Callee = Callee {
        addr: 0x0013_c878,
        f: ee::s01::RiderGroundState_ComputeFrictionAccel,
    };

    pub const RIDER_GROUND_STATE_COMPUTE_BOOST_ACCEL: Callee = Callee {
        addr: 0x0013_c948,
        f: ee::s01::RiderGroundState_ComputeBoostAccel,
    };

    pub const RIDER_GROUND_STATE_COMPUTE_DRAG: Callee = Callee {
        addr: 0x0013_ccf0,
        f: ee::s01::RiderGroundState_ComputeDrag,
    };

    pub const RIDER_GROUND_STATE_COMPUTE_CARVE_ACCEL: Callee = Callee {
        addr: 0x0013_d028,
        f: ee::s01::RiderGroundState_ComputeCarveAccel,
    };

    pub const BELIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    pub const BELIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };

    pub const BECHARINT_GET_ATTRIB3_FACTOR_2: Callee = Callee {
        addr: 0x0014_8e68,
        f: ee::s01::BECharInt_GetAttrib3Factor_2,
    };

    pub const RIDER_GROUND_STATE_PROBE_GROUND: Callee = Callee {
        addr: 0x0013_d1b8,
        f: ee::s01::RiderGroundState_ProbeGround,
    };
}

// ---------------------------------------------------------------------------
// Ground probe.

/// `Contact` records (0x80 bytes) as the collision queries fill them.
pub(crate) mod contact {
    pub(crate) use crate::game::ai::rider::contact::DEPTH;
    pub(crate) use crate::game::ai::rider::contact::INSTANCE;
    pub(crate) use crate::game::ai::rider::contact::MATERIAL;
    pub(crate) use crate::game::ai::rider::contact::NORMAL;
    pub(crate) use crate::game::ai::rider::contact::SIZE;
    pub(crate) use crate::game::ai::rider::contact::SURFACE;
}

/// Rider fields this function reads or writes.
pub(crate) mod rider {
    pub(crate) use crate::game::ai::computer::rider::AIR_PREDICTOR;
    pub(crate) use crate::game::ai::computer::rider::ATTRIB_VALUE;
    pub(crate) use crate::game::ai::computer::rider::BASE_2 as BASE;
    pub(crate) use crate::game::ai::computer::rider::BOOST_2 as BOOST;
    pub(crate) use crate::game::ai::computer::rider::BOOST_COUNT;
    pub(crate) use crate::game::ai::computer::rider::BOOST_FLAG;
    pub(crate) use crate::game::ai::computer::rider::BOOST_MASK;
    pub(crate) use crate::game::ai::computer::rider::BOOST_STATE;
    pub(crate) use crate::game::ai::computer::rider::BOOST_TIMER;
    pub(crate) use crate::game::ai::computer::rider::CHAR_COMP;
    pub(crate) use crate::game::ai::computer::rider::CHAR_ID;
    pub(crate) use crate::game::ai::computer::rider::HEADING_2 as HEADING;
    pub(crate) use crate::game::ai::computer::rider::METRIC_SCALE;
    pub(crate) use crate::game::ai::computer::rider::PLAYER;
    pub(crate) use crate::game::ai::computer::rider::SPEED;
    pub(crate) use crate::game::ai::computer::rider::SPEED_FACTOR;
    pub(crate) use crate::game::ai::computer::rider::VELOCITY_2 as VELOCITY;
    pub(crate) use crate::game::ai::rider::rider::FRAME_FWD;
    pub(crate) use crate::game::ai::rider::rider::FRAME_SIDE;
    pub(crate) use crate::game::ai::rider::rider::FWD_REF;
    pub(crate) use crate::game::ai::rider::rider::GROUND_HEIGHT;
    pub(crate) use crate::game::ai::rider::rider::GROUND_ID;
    pub(crate) use crate::game::ai::rider::rider::GROUND_MATERIAL;
    pub(crate) use crate::game::ai::rider::rider::GROUND_NORMAL2;
    pub(crate) use crate::game::ai::rider::rider::GROUND_POINT;
    pub(crate) use crate::game::ai::rider::rider::OVERLAP_HINT;
    pub(crate) use crate::game::ai::rider::rider::OVERLAP_LIST;
    pub(crate) use crate::game::ai::rider::rider::POS;
    pub(crate) use crate::game::ai::rider::rider::SPHERE_TREE;
    pub(crate) use crate::game::ai::rider::rider::UP;
    pub(crate) use crate::game::ai::rider::rider::UP_PREV;
    pub(crate) use crate::game::ai::rider::rider::VEL;

    pub const ANIM: u32 = 0x784;

    pub const TRANSFORM: u32 = 0x370;

    pub const UP_2: u32 = 0x3a0;

    pub const SIDE: u32 = 0x3b0;

    pub const LAND_ANIM: u32 = 0x3d0;

    pub const TRIGGER: u32 = 0x430;

    pub const CONTACT: u32 = 0x438;

    pub const NORMAL: u32 = 0x454;

    pub const XFORM: u32 = 0x460;

    pub const WALLS: u32 = 0xaa0;

    pub const BLEND_A: u32 = 0xaac;

    pub const BLEND_B: u32 = 0xab0;
}

/// gp offsets of the globals this module reads.
pub(crate) mod global {

    /// The `BELibrary` singleton.
    pub const BE_LIBRARY: u32 = 0x1f04;

    /// The race-phase index `BELibrary_GetInterface` selects.
    pub const BE_LIBRARY_INDEX: u32 = 0x1efc;
}

/// The computer rider's AI bookkeeping.
pub(crate) mod computer {
    pub(crate) use crate::game::ai::computer::computer::DIR;
    pub(crate) use crate::game::ai::computer::computer::RIDER;
    pub(crate) use crate::game::ai::computer::computer::STUCK_TIME;
    pub(crate) use crate::game::ai::computer::computer::WRONG_TIME;
}

// ---------------------------------------------------------------------------
// Speed curve and the carve response.

/// `sCurvePt_13D028` pairs: four `{x, y}` points, x ascending.
pub(crate) mod speed_curve {

    pub const P0_X: u32 = 0x00;

    pub const P0_Y: u32 = 0x04;

    pub const P1_X: u32 = 0x08;

    pub const P1_Y: u32 = 0x0c;

    pub const P2_X: u32 = 0x10;

    pub const P2_Y: u32 = 0x14;

    pub const P3_X: u32 = 0x18;

    pub const P3_Y: u32 = 0x1c;
}

/// `RiderGroundState` fields used by the friction model, and the rider field
/// the model publishes its normalised slip to.
pub(crate) mod ground_state {

    /// `m4`: the start of the slip curve.
    pub const M4: u32 = 0x4;

    /// `m8`: the top of the slip curve.
    pub const M8: u32 = 0x8;

    /// `mRider`.
    pub const RIDER: u32 = 0x18;

    /// `p[7]`, the second friction coefficient.
    pub const COEFF_B: u32 = 0x1c;

    /// `Rider::unk_0x758`: the normalised slip published every update.
    pub const SLIP: u32 = 0x758;
}

// ---------------------------------------------------------------------------
// Rider air state.

/// Rider fields used by the air-state update.
pub(crate) mod air {

    /// The rider object behind the state.
    pub const RIDER: u32 = 0x04;

    /// The rider's animation / air-predictor state.
    pub const ANIM: u32 = 0x788;

    /// The rider's sub-state object.
    pub const SUB: u32 = 0x77c;

    /// The rider's time scale (normally 1.0): the tick's dt is this times the
    /// step.
    pub const DT: u32 = 0x300;

    /// Predicted position / velocity buffers handed to the predictor.
    pub const PRED_A: u32 = 0x110;

    pub const PRED_B: u32 = 0x1e0;

    /// Up vector and the copy the physics step reads.
    pub const UP: u32 = 0x180;

    pub const UP_COPY: u32 = 0x370;

    pub const GRAVITY: u32 = 0x3d0;

    /// The orientation the air state drives.
    pub const ORIENT: u32 = 0x1b0;

    /// Set while the wind force applies.
    pub const WIND: u32 = 0x874;
}

/// Air-predictor fields.
pub(crate) mod anim {

    /// Current state of the animation (1 and 3 are air states).
    pub const STATE: u32 = 0xac;

    /// The orientation target and its "thin" threshold.
    pub const ORIENT: u32 = 0x10;

    pub const THIN: u32 = 0x28;

    /// Index into the race manager's section records.
    pub const SECTION: u32 = 0x90;

    /// Section flags (bit 3 marks a boost pad).
    pub const FLAGS: u32 = 0x94;

    /// Time window of the air phase.
    pub const T0: u32 = 0x98;

    pub const T1: u32 = 0xa0;

    pub const ANGLE: u32 = 0x258;
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

/// Calls the generator emitted as `call_indirect` (cross-shard `jal`).
pub(crate) mod target {

    /// `Rider_GetSubState(rider)`.
    pub const GET_SUB_STATE: u32 = 0x0011_fee8;

    /// `Rider_SetSubState(rider, state)`.
    pub const SET_SUB_STATE: u32 = 0x0011_fec8;

    /// `Rider_SetMainState(rider, state)`.
    pub const SET_MAIN_STATE: u32 = 0x0011_fe78;

    /// `Rider_CheckResetPath(rider, out, flag)`.
    pub const CHECK_RESET_PATH: u32 = 0x0011_6120;

    /// `Rider_ResetGrindOrientation(rider)`.
    pub const RESET_GRIND: u32 = 0x0011_5640;

    /// `Rider_TurnAroundIfBackward(rider)`.
    pub const TURN_AROUND: u32 = 0x0011_4cc0;

    /// `Rider_Translate(rider, ...)`.
    pub const TRANSLATE: u32 = 0x0010_6538;

    /// `Rider_BankTrickBoost(rider, ...)`.
    pub const BANK_TRICK_BOOST: u32 = 0x0010_e910;

    /// `Rider_StartWipeout(rider, a1, a2, a3)`.
    pub const START_WIPEOUT: u32 = 0x0010_eb30;

    /// `AirPredictor_...` (src/ai/airpredictor.cpp).
    pub const AIR_PREDICTOR: u32 = 0x0011_3998;

    /// `RiderAnimBase` layer queries.
    pub const ANIM_LAYER: u32 = 0x0031_1ae8;

    pub const ANIM_LAYER_SEQUENCE: u32 = 0x0031_1b20;

    /// `AnimController_PlayAnim(controller, anim, blend, flags)`.
    pub const PLAY_ANIM: u32 = 0x0031_28e8;

    /// `Rand_NextGame()`.
    pub const RAND_NEXT_GAME: u32 = 0x0031_7810;

    /// `WScriptMan` query.
    pub const WSCRIPT: u32 = 0x0030_98b0;

    /// The air state's wall and camera tail calls.
    pub const AIR_WALLS: u32 = 0x0010_5398;

    pub const AIR_CAMERA: u32 = 0x0010_7888;
}

pub(crate) const PG_FILL: u32 = 0x0013_d2f0;

pub(crate) const PG_PICK: u32 = 0x0013_d4a0;

pub(crate) const PG_COPY: u32 = 0x0013_d5a8;

/// `rider = state->rider` (`lw reg, 0x18(s1)`).
#[inline(always)]
pub(crate) fn ground_rider(c: &mut Cpu, reg: usize) -> u32 {
    c.lw_r(reg, at(c, S1, 0x18));
    r(c, reg)
}

/// Re-orthonormalise the rider frame on its up vector (`a1 = rider`):
/// `fwd = normalize(ref - up * dot(ref, up))`, `side = up x fwd`.
pub(crate) fn rebuild_rider_frame(c: &mut Cpu) {
    let s = sp_2(c);
    let rd = ground_rider(c, A1);
    c.qcopy(V1, p(rd, rider::FWD_REF), p(s, 0x2100));
    c.qcopy(A0, p(rd, rider::FWD_REF), p(s, 0x2120));
    c.vdot(p(s, 0x2120), p(rd, rider::UP), V1, 0);
    c.mfc1(A0, 0);
    c.vscale(p(rd, rider::UP), A0);
    c.vstore(5, p(s, 0x2130), V1, &[p(s, 0x2110)]);
    c.vsub(p(s, 0x2100), p(s, 0x2110));
    c.vstore(5, p(s, 0x2130), V1, &[p(s, 0x40)]);
    c.vlen2(p(s, 0x40));
    c.vnorm();
    c.vstore(5, p(s, 0x2130), V1, &[p(s, 0x30), p(rd, rider::FRAME_FWD)]);
    let rd = ground_rider(c, A0);
    c.lqc2(4, p(rd, rider::UP));
    c.lqc2(5, p(rd, rider::FRAME_FWD));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.vstore(6, p(s, 0x40), V1, &[p(s, 0x30), p(rd, rider::FRAME_SIDE)]);
}

/// `bool RiderGroundState_ProbeGround(RiderGroundState* g, Contact* out,
/// float* height)` (0x13d1b8): casts a segment along the rider's up axis
/// (through the position shifted sideways by `speed * 45 * k`, from 100
/// below to 200 above, radius 0.5) against the rider's overlap list. No contact: resets the up vector and ground
/// normal to the defaults, rebuilds the frame and returns false with
/// `*height = 0`. Otherwise picks the contact whose normal is closest to
/// the floor (steep enough, depth nearest 0.5, ties by surface material /
/// instance id), copies it to `out`, records ground id, UV and flags,
/// takes its normal as the new up vector and returns true with the height
/// of the rider above it.
pub fn rider_ground_state_probe_ground(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 78;
        prologue(
            c,
            0x2190,
            &[(S0, 0x2180), (S1, 0x2170), (S2, 0x2160), (S3, 0x2150)],
            0x2140,
            &[],
        );
        let s = sp_2(c);
        c.g[AT][0] = 0x4234_0000; // 45.0
        c.mtc1(AT, 3);
        c.g[A3][0] = 2;
        c.set_r(S0, p(s, 0x50));
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A2][0];
        c.g[AT][0] = 0xffff_ffff_c2c8_0000; // -100.0
        c.mtc1(AT, 2);
        c.set_r(A1, p(s, 0x10));
        let rd = ground_rider(c, V1);
        c.set_r(A2, p(s, 0x20));
        c.g[AT][0] = 0x4348_0000; // 200.0
        c.mtc1(AT, 4);
        c.g[A0][0] = c.g[S0][0];
        c.lq(V0, p(rd, rider::UP));
        c.g[AT][0] = 0x3f00_0000; // 0.5
        c.mtc1(AT, 12);
        c.sq(V0, p(rd, rider::UP_PREV));
        // start = pos + side * (speed * 45 * k)
        let rd = ground_rider(c, V1);
        c.lwc1(1, p(rd, 0x1f0));
        c.lw_r(V0, p(rd, 0x780));
        c.mul_s(1, 1, 3);
        c.lwc1(0, at(c, V0, 0x140));
        c.mul_s(0, 0, 1);
        c.mfc1(T0, 0);
        c.vscale(p(rd, rider::FRAME_SIDE), T0);
        c.vstore(5, p(s, 0x20), V0, &[]);
        let rd = ground_rider(c, V1);
        c.sq(V0, p(s, 0x10));
        c.vadd(p(rd, rider::POS), p(s, 0x10));
        c.vstore(5, p(s, 0x20), V0, &[]);
        let rd = ground_rider(c, V1);
        c.sq(V0, s);
        // a = start + up * -100
        c.mfc1(T0, 2);
        c.vscale(p(rd, rider::UP), T0);
        c.vstore(5, p(s, 0x30), V0, &[p(s, 0x20)]);
        c.vadd(s, p(s, 0x20));
        c.vstore(5, p(s, 0x30), V0, &[]);
        let rd = ground_rider(c, V1);
        c.sq(V0, p(s, 0x10));
        // b = start + up * 200
        c.mfc1(T0, 4);
        c.vscale(p(rd, rider::UP), T0);
        c.vstore(5, p(s, 0x40), V0, &[p(s, 0x30)]);
        c.vadd(s, p(s, 0x30));
        c.vstore(5, p(s, 0x40), V0, &[]);
        c.g[RA][0] = sx(0x0013_d2e0);
        c.sq(V0, p(s, 0x20));
        // CollSegment_ctor(&seg, &a, &b, 2, 0.5)
        call_ptr(c, callee::COLL_SEGMENT_CTOR, 0x0013_d2e0)?;
        c.g[A1][0] = c.g[S0][0];
        c.set_r(A2, p(sp_2(c), 0x100));
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 0x3f;
        // The (empty) constructor loop of the 64 contact records.
        loop {
            c.cycles += 7;
            c.inc_r(V0, u32::MAX);
            c.g[A3][0] = 0x40;
            if c.g[V0][0] == c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, PG_FILL);
            }
        }
        // n = OverlapList_CollideVolume(rider->list, &seg, contacts, 64, rider->hint)
        c.cycles += 7;
        let rd = ground_rider(c, V0);
        c.lw_r(T0, p(rd, rider::OVERLAP_HINT));
        c.lw_r(A0, p(rd, rider::OVERLAP_LIST));
        call_ptr(c, callee::OVERLAP_LIST_COLLIDE_VOLUME, 0x0013_d31c)?;
        c.g[T3][0] = c.g[V0][0];
        if c.g[T3][0] == 0 {
            probe_ground_miss(c);
        } else {
            ground_rider(c, V1);
            probe_ground_pick(c);
            probe_ground_hit(c)?;
        }
        c.cycles += 7;
        epilogue(
            c,
            &[(S0, 0x2180), (S1, 0x2170), (S2, 0x2160), (S3, 0x2150)],
            &[],
            0x2140,
            0x2190,
        );
        Ok(())
    })
}

/// No ground: default up and ground normal, frame rebuilt, `*height = 0`.
pub(crate) fn probe_ground_miss(c: &mut Cpu) {
    c.cycles += 65;
    c.g[V0][0] = 0x50_0000;
    let rd = ground_rider(c, A2);
    c.set_r(V0, RODATA_UP);
    c.g[A1][0] = 0x50_0000;
    c.lq(A0, RODATA_UP);
    c.set_r(A1, RODATA_NORMAL2);
    c.g[V0][0] = 0;
    c.sq(A0, p(rd, rider::UP));
    c.lq(V1, RODATA_NORMAL2);
    let rd = ground_rider(c, A0);
    c.sq(V1, p(rd, rider::GROUND_NORMAL2));
    rebuild_rider_frame(c);
    let rd = ground_rider(c, A1);
    c.sw(p(rd, rider::GROUND_MATERIAL), 0);
    let out = r(c, S3);
    c.sw(out, 0);
}

/// Pick the best contact into t1 (t3 contacts at `sp_2+0x100`, v1 = rider).
pub(crate) fn probe_ground_pick(c: &mut Cpu) {
    let s = sp_2(c);
    // Contact 0: f1 = n.up, f2 = |depth - 0.5|, t4 = steep enough (f4 <= f1).
    c.cycles += 20;
    c.g[T1][0] = 0;
    c.lwc1(4, gp(c, -0x7068));
    c.vdot(p(s, 0x110), at(c, V1, rider::UP), V0, 1);
    c.lwc1(0, p(s, 0x140));
    c.g[T4][0] = 1;
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 3);
    c.c_le_s(4, 1);
    c.sub_s(0, 0, 3);
    c.abs_s(2, 0);
    if !c.fcc() {
        c.g[T4][0] = 0;
    }
    c.cycles += 4;
    c.g[A3][0] = 1;
    c.g[V0][0] = slt(c, A3, T3);
    c.set_r(A2, p(s, 0x180));
    if c.g[V0][0] == 0 {
        return;
    }
    c.cycles += 5;
    c.mov_s(5, 3);
    c.g[T2][0] = c.g[V1][0];
    c.mov_s(3, 4);
    c.set_r(T0, p(s, 0x1c0));
    loop {
        // f1 = n.up, f0 = |depth - 0.5|
        c.cycles += 14;
        c.lwc1(0, r(c, T0));
        c.vdot(at(c, A2, contact::NORMAL), at(c, T2, rider::UP), V0, 1);
        c.sub_s(0, 0, 5);
        c.abs_s(0, 0);
        let mut advanced = false;
        let mut decided = false;
        if c.g[T4][0] != 0 {
            // A steep-enough best: skip contacts that are not.
            c.cycles += 4;
            c.c_lt_s(1, 3);
            if c.fcc() {
                c.inc_r(A3, 1);
                advanced = true;
                decided = true;
            }
        } else {
            c.cycles += 3;
            c.c_le_s(3, 1);
            if c.fcc() {
                // The first steep-enough contact replaces a flat best.
                c.cycles += 4;
                c.mov_s(2, 0);
                c.g[T1][0] = c.g[A3][0];
                c.g[T4][0] = 1;
                decided = true;
            }
        }
        if !decided {
            c.cycles += 3;
            c.c_lt_s(0, 2);
            if c.fcc() {
                c.cycles += 3;
                c.mov_s(2, 0);
                c.g[T1][0] = c.g[A3][0];
            } else {
                c.cycles += 4;
                c.c_eq_s(0, 2);
                let tie = c.fcc();
                c.set_r(V0, r(c, T1) << 7);
                if tie {
                    contact_tie_break(c, A2, 0x100);
                    c.cycles += 1;
                    if c.g[V0][0] != 0 {
                        c.g[T1][0] = c.g[A3][0];
                    }
                }
            }
        }
        if !advanced {
            c.cycles += 1;
            c.inc_r(A3, 1);
        }
        c.cycles += 4;
        c.inc_r(A2, contact::SIZE);
        c.g[V0][0] = slt(c, A3, T3);
        c.inc_r(T0, contact::SIZE);
        if c.g[V0][0] == 0 {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, PG_PICK);
        }
    }
}

/// Equal-score tie break between the current contact (`cur_reg`) and the
/// best one (`v0 = best << 7`, records at `sp_2 + base`): leaves v0 = 1 when
/// the current one wins (it has a surface and the best has none, or a
/// lower surface id; without surfaces, a lower instance id).
pub(crate) fn contact_tie_break(c: &mut Cpu, cur_reg: usize, base: u32) {
    c.cycles += 4;
    c.lw_r(A0, at(c, cur_reg, contact::SURFACE));
    let best = r(c, V0).wrapping_add(sp_2(c));
    c.set_r(V1, best);
    c.set_r(V1, best.wrapping_add(base));
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.lw_r(V0, at(c, V1, contact::SURFACE));
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 1;
            return;
        }
        c.lw_r(V1, at(c, A0, 0x78));
        c.cycles += 3;
        c.lw_r(V0, at(c, V0, 0x78));
        c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        return;
    }
    c.cycles += 3;
    c.lw_r(V0, at(c, V1, contact::SURFACE));
    let best_has_surface = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if best_has_surface {
        return;
    }
    c.cycles += 5;
    c.lw_r(V1, at(c, V1, contact::INSTANCE));
    c.lw_r(A0, at(c, cur_reg, contact::INSTANCE));
    c.lw_r(A1, at(c, V1, 0x150));
    c.lw_r(V0, at(c, A0, 0x150));
    c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
}

/// The best contact (t1) to `out`, ground id / UV / flags / material,
/// owner notified, up vector and frame from it, height above it.
pub(crate) fn probe_ground_hit(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    let s = sp_2(c);
    let best = p(s, r(c, T1) << 7);
    c.set_r(V0, r(c, T1) << 7);
    c.g[V1][0] = c.g[S2][0];
    c.set_r(V0, best);
    c.set_r(A0, p(best, 0x180));
    c.set_r(V0, p(best, 0x100));
    copy_contact(c, PG_COPY);
    let out = r(c, S2);
    // Ground id and UV from the instance, or no id.
    c.cycles += 3;
    c.lw_r(V0, p(out, contact::INSTANCE));
    if c.g[V0][0] == 0 {
        let rd = ground_rider(c, V1);
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        c.sw_r(p(rd, rider::GROUND_ID), V0);
    } else {
        c.cycles += 14;
        c.lw_r(V1, at(c, V0, 0x150));
        let rd = ground_rider(c, V0);
        c.sw_r(p(rd, rider::GROUND_ID), V1);
        let rd = ground_rider(c, A0);
        c.lwc1(0, p(out, 0x6c));
        c.swc1(0, p(rd, 0xaac));
        let rd = ground_rider(c, V0);
        c.lwc1(0, p(out, 0x70));
        c.swc1(0, p(rd, 0xab0));
        c.lw_r(V1, p(out, contact::INSTANCE));
        let rd = ground_rider(c, A0);
        c.g[V0][0] = c.lhu(at(c, V1, 0xa));
        let flags = c.g[V0][0];
        c.sh(p(rd, 0x2d4), flags);
    }
    // Tell the surface's owner (virtual 0x150).
    c.cycles += 3;
    c.lw_r(A0, p(out, contact::SURFACE));
    if c.g[A0][0] == 0 {
        ground_rider(c, V1);
    } else {
        c.cycles += 3;
        c.lw_r(A2, at(c, A0, 0xc));
        c.g[A1][0] = c.g[S2][0];
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.lw_r(V0, at(c, A2, 0xc));
            vcall(c, A2, V0, 0x150, V1, 0x0013_d640)?;
        }
        c.cycles += 1;
        ground_rider(c, V1);
    }
    // Material (-1: none).
    c.cycles += 7;
    c.g[A0][0] = u64::MAX;
    c.lw_r(V0, p(out, 0x4c));
    c.sw_r(at(c, V1, rider::GROUND_MATERIAL), V0);
    let rd = ground_rider(c, V1);
    c.lw_r(V0, p(rd, rider::GROUND_MATERIAL));
    if c.g[V0][0] == c.g[A0][0] {
        c.sw(p(rd, rider::GROUND_MATERIAL), 0);
    }
    // Up = contact normal, ground point and second normal; frame; height.
    c.cycles += 103;
    let s = sp_2(c);
    let rd = ground_rider(c, A1);
    c.g[V0][0] = 1;
    c.qcopy(V1, p(out, 0x10), p(rd, rider::UP));
    let rd = ground_rider(c, A0);
    c.qcopy(V1, out, p(rd, rider::GROUND_POINT));
    c.lq(V1, p(out, 0x20));
    let rd = ground_rider(c, A2);
    c.sq(V1, p(rd, rider::GROUND_NORMAL2));
    rebuild_rider_frame(c);
    c.vsub(s, out);
    c.sqc2(5, p(s, 0x40));
    c.lq(V1, p(s, 0x40));
    let rd = ground_rider(c, A0);
    c.sq(V1, p(s, 0x30));
    c.vdot(p(s, 0x30), p(rd, rider::UP), A1, 0);
    c.swc1(0, p(rd, rider::GROUND_HEIGHT));
    let rd = ground_rider(c, V1);
    c.lwc1(0, p(rd, rider::GROUND_HEIGHT));
    c.mfc1(A1, 0);
    c.vscale(p(rd, rider::UP), A1);
    c.vstore(5, p(s, 0x2110), A0, &[p(s, 0x2100)]);
    c.vsub(p(s, 0x30), p(s, 0x2100));
    c.vstore(5, p(s, 0x2110), V1, &[p(s, 0x40)]);
    c.vlen(p(s, 0x40), A0, 0);
    let height = r(c, S3);
    c.swc1(0, height);
    Ok(())
}

/// A `gp_2`-relative global address.
#[inline(always)]
pub(crate) fn gp_2(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Focus manager octree.

/// `void FocusMan_CollectOctreeNode(sFocusMan* self, sOctNode* node,
/// sOctKey* key)` (0x00144368): walk the octree for the cell described by
/// `key` (level, x, y, z). The first of the manager's boxes that overlaps the
/// cell's volume (scaled by `2^level`, 0.2 margin) picks the node: the node
/// is reported to the focus manager and, unless it is a leaf or a level-11
/// node, all eight children are visited with the doubled, halved-level key.
pub fn focus_man_collect_octree_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0]; // key
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A1][0]; // node
        frame.save_ra(c, 0x10);
        c.g[T1][0] = Addr(p(c.g[S2][0] as u32, 8)).lw(c); // box count
                                                          // Box loop: pick the first box overlapping the cell volume.
        let skip_cycle = loop {
            let i = c.g[A3][0] as u32;
            c.cycles += 2;
            let done = c.g[A3][0] == c.g[T1][0];
            c.g[A0][0] = sx(i << 5);
            if done {
                break false;
            }
            c.cycles += 18;
            c.g[A2][0] = Addr(p(c.g[S1][0] as u32, 4)).lw(c); // key->x
            c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0)).lw(c); // key->level
            c.mtc1(6, 1);
            c.cvt_s_w(1, 1); // f1 = (float)level ...
            c.lwc1(3, gp_2(c, -0x6e1c)); // 0.2 margin
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = Addr(p(c.g[S2][0] as u32, 4)).lw(c); // boxes
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7f));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
            c.sub_s(1, 1, 3);
            c.mtc1(2, 2); // f2 = 2^(level-127) from the exponent bits
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.lwc1(0, p(c.g[A0][0] as u32, 0x10));
            c.mul_s(1, 1, 2);
            c.c_le_s(1, 0);
            c.g[T0][0] = 0;
            if c.fcc() {
                c.cycles += 9;
                c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 8)).lw(c); // key->y
                c.lwc1(1, p(c.g[A0][0] as u32, 0x14));
                c.mtc1(3, 0);
                c.cvt_s_w(0, 0);
                c.sub_s(0, 0, 3);
                c.mul_s(0, 0, 2);
                c.c_le_s(0, 1);
                if c.fcc() {
                    c.cycles += 10;
                    c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0xc)).lw(c); // key->z
                    c.lwc1(1, p(c.g[A0][0] as u32, 0x18));
                    c.mtc1(5, 0);
                    c.cvt_s_w(0, 0);
                    c.sub_s(0, 0, 3);
                    c.mul_s(0, 0, 2);
                    c.c_le_s(0, 1);
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    if c.fcc() {
                        c.cycles += 9;
                        c.lwc1(1, c.g[A0][0] as u32);
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.add_s(0, 0, 3);
                        c.mul_s(0, 0, 2);
                        c.c_le_s(1, 0);
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        if c.fcc() {
                            c.cycles += 9;
                            c.lwc1(1, p(c.g[A0][0] as u32, 4));
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            c.add_s(0, 0, 3);
                            c.mul_s(0, 0, 2);
                            c.c_le_s(1, 0);
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                            if c.fcc() {
                                c.cycles += 9;
                                c.lwc1(1, p(c.g[A0][0] as u32, 8));
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 3);
                                c.mul_s(0, 0, 2);
                                c.c_le_s(1, 0);
                                if c.fcc() {
                                    c.g[T0][0] = 1;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            let no_match = c.g[T0][0] == 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            if no_match {
                if events::event_due(c) {
                    events::deliver_event_here(c, 0x0014_4398);
                }
                continue;
            }
            // The node overlaps: report it, and descend unless it is a leaf.
            c.cycles += 7;
            c.g[S0][0] = ((c.g[S0][0] as i64) < 14i64) as u64;
            c.g[A0][0] = Addr(p(c.g[S2][0] as u32, 0)).lw(c);
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S0][0];
            crate::call::call_ptr(c, 0x0010_1a28, 0x0014_44bc)?;
            if c.g[S0][0] != 0 {
                break true;
            }
            c.cycles += 4;
            c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0)).lw(c);
            c.g[V0][0] = 0xb;
            let level_11 = c.g[A1][0] == c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            if level_11 {
                break false;
            }
            // Child keys: level-1, doubled x/y/z; each child shifts one
            // coordinate by one after its visit.
            c.cycles += 12;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 4)).lw(c);
            c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 8)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[A2][0] = Addr(p(c.g[S3][0] as u32, 0)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
            c.sw(sp_3(c), c.g[A1][0]);
            c.sw(p(sp_3(c), 4), c.g[V0][0]);
            c.sw(p(sp_3(c), 8), c.g[V1][0]);
            c.sw(p(sp_3(c), 0xc), c.g[A0][0]);
            if c.g[A2][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.g[A2][0];
                descend(c, 0x0014_4514)?;
            }
            // child[4] (+0x10), key.x + 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp_3(c), 4)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0x10)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp_3(c), 4), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_4534)?;
            }
            // child[6] (+0x18), key.y + 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp_3(c), 8)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0x18)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp_3(c), 8), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_4554)?;
            }
            // child[2] (+0x8), key.x - 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp_3(c), 4)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0x8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp_3(c), 4), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_4574)?;
            }
            // child[3] (+0xc), key.z + 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp_3(c), 0xc)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0xc)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp_3(c), 0xc), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_4594)?;
            }
            // child[7] (+0x1c), key.x + 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp_3(c), 4)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0x1c)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp_3(c), 4), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_45b4)?;
            }
            // child[5] (+0x14), key.y - 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp_3(c), 8)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 0x14)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp_3(c), 8), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_45d4)?;
            }
            // child[1] (+0x4), key.x - 1
            c.cycles += 5;
            c.g[V0][0] = Addr(p(sp_3(c), 4)).lw(c);
            c.g[A1][0] = Addr(p(c.g[S3][0] as u32, 4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let skip = c.g[A1][0] == 0;
            c.sw(p(sp_3(c), 4), c.g[V0][0]);
            if !skip {
                c.cycles += 3;
                descend(c, 0x0014_45f4)?;
            }
            break false;
        };
        if !skip_cycle {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x50);
        c.cycles += 6;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The recursive child visit of [`focus_man_collect_octree_node`]:
/// `FocusMan_CollectOctreeNode(self, child, key)`, returning to `ret`.
#[inline(always)]
pub(crate) fn descend(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = sx(sp_3(c));
    callee::FOCUS_MAN_COLLECT_OCTREE_NODE.call(c, ret)
}

/// `BELibrary_GetInterface`, through the function table.
pub(crate) const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

/// `BECharInt_GetAttrib3Factor_2`, through the function table.
pub(crate) const BE_CHAR_ATTRIB3_FACTOR: u32 = 0x0014_8e68;

/// `D_004A1140`: pointer to the four-point speed curve.
pub(crate) const SPEED_CURVE: u32 = 0x004a_1140;

/// The back-end's phase-interface table (`BELibrary_GetInterface`).
pub(crate) const BE_PHASE_INTERFACES: u32 = 0x0053_08a8;

/// The carve curve's `x = |b| * 0.036` scale, the edging stat's reciprocal
/// scale, and the pointer variable holding the curve.
pub(crate) const CURVE_SPEED_SCALE: i32 = -0x7070;
// 0.036
pub(crate) const STAT_SPEED_SCALE: i32 = -0x706c;
// 4.0086374
pub(crate) const SPEED_CURVE_PTR: i32 = -0x1fb0;
// `D_004A1140`

/// `float RiderGroundState_ComputeCarveAccel(ground, const Vec4* v, float a,
/// float b, float c)` (0x13d028): when the lateral speed `a` and the carve
/// input `c` point the same way, `a * c` (or `a * -c` for the other sign) is
/// scaled by the speed curve at `|b| * 0.036`, by `-v[2]`, and — when the
/// ground's `+0x2fc` factor is positive — by `1 / (1 + 4.0086 * factor)`.
/// The edging stat comes from `BECharInt_GetAttrib3Factor_2`. The curve
/// search is the inlined `hpCurve`: outside `p0..p3` the end point's y is
/// used without the lerp, inside a band the lerp uses that band's endpoints.
pub fn rider_ground_state_compute_carve_accel(c: &mut Cpu) {
    use speed_curve::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.mtc1(0, 0); // f0 = 0
        frame.save_fpr(c, 20, 0x20);
        c.mov_s(20, 12); // f20 = a
        frame.save(c, S0, 0x10);
        let a_negative = lt(c, 20, 0);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        // `a` and `c` must point the same way: `a * c`, `a * -c`, else 0.
        let mut same_way = false;
        let mut second_test = true;
        if a_negative {
            c.cycles += 3;
            if lt(c, 0, 14) {
                c.cycles += 2;
                setf(c, 20, fr(c, 20) * fr(c, 14));
                same_way = true;
                second_test = false;
            }
        }
        if second_test {
            c.cycles += 3;
            if lt(c, 0, 20) {
                c.cycles += 3;
                if lt(c, 14, 0) {
                    c.cycles += 3;
                    let c_neg = -fr(c, 14);
                    setf(c, 0, c_neg);
                    setf(c, 20, fr(c, 20) * c_neg);
                    same_way = true;
                }
            }
        }
        if !same_way {
            // No carve: return +0.0.
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore(c, S0, 0x10);
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.restore_fpr(c, 20, 0x20);
            frame.close(c);
            return Ok(());
        }
        // The curve at |b| * 0.036: f3 = x, f1 receives the value.
        c.cycles += 9;
        let scale = ldf(c, 1, gp_3(c, CURVE_SPEED_SCALE));
        let abs_b = setf(c, 0, fr(c, 13).abs());
        let curve = lw(c, gp_3(c, SPEED_CURVE_PTR)) as u32;
        setf(c, 3, abs_b * scale);
        ldf(c, 4, p(curve, P1_X));
        let mut lerp = false;
        if !lt(c, 4, 3) {
            // x <= p1.x: the p0..p1 band, or the clamp at p0.
            ldf(c, 1, p(curve, P0_X));
            c.cycles += 4;
            if lt(c, 3, 1) {
                c.cycles += 2;
                ldf(c, 1, p(curve, P0_Y));
            } else {
                ldf(c, 2, p(curve, P0_Y));
                c.cycles += 2;
                setf(c, 3, fr(c, 3) - fr(c, 1));
                ldf(c, 0, p(curve, P1_Y));
                c.cycles += 1;
                setf(c, 1, fr(c, 4) - fr(c, 1));
                lerp = true;
            }
        } else {
            c.cycles += 5;
            ldf(c, 1, p(curve, P2_X));
            if !lt(c, 1, 3) {
                ldf(c, 2, p(curve, P1_Y));
                c.cycles += 4;
                setf(c, 3, fr(c, 3) - fr(c, 4));
                ldf(c, 0, p(curve, P2_Y));
                setf(c, 1, fr(c, 1) - fr(c, 4));
                lerp = true;
            } else {
                c.cycles += 5;
                ldf(c, 4, p(curve, P3_X));
                if !lt(c, 4, 3) {
                    ldf(c, 2, p(curve, P2_Y));
                    c.cycles += 3;
                    setf(c, 3, fr(c, 3) - fr(c, 1));
                    ldf(c, 0, p(curve, P3_Y));
                    c.cycles += 1;
                    setf(c, 1, fr(c, 4) - fr(c, 1));
                    lerp = true;
                } else {
                    c.cycles += 2;
                    ldf(c, 1, p(curve, P3_Y));
                }
            }
        }
        if lerp {
            c.cycles += 4;
            let f0 = setf(c, 0, fr(c, 0) - fr(c, 2));
            let f0 = setf(c, 0, f0 * fr(c, 3));
            let f0 = setf(c, 0, f0.div(fr(c, 1)));
            setf(c, 1, fr(c, 2) + f0);
        }
        // a = a * curve * -v[2]
        c.cycles += 22;
        let v = r32(c, A1);
        ldf(c, 0, p(v, 0x8));
        setf(c, 20, fr(c, 20) * fr(c, 1));
        let v2 = -fr(c, 0);
        setf(c, 0, v2);
        setf(c, 20, fr(c, 20) * v2);
        callee::BE_LIBRARY_GET_INSTANCE_2.call(c, 0x0013_d148)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x0013_d154)?;
        c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, computer::RIDER));
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = lw(c, p(c.g[V1][0] as u32, rider::ATTRIB_VALUE));
        c.g[A1][0] = lw(c, p(c.g[V1][0] as u32, rider::PLAYER));
        call_ptr(c, BE_CHAR_ATTRIB3_FACTOR, 0x0013_d168)?;
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, computer::RIDER));
        c.mtc1(0, 0);
        ldf(c, 1, p(c.g[V0][0] as u32, rider::SPEED_FACTOR));
        li_f(c, 2, 0x3f80_0000); // 1.0
        let positive = lt(c, 0, 1);
        c.mov_s(0, 20); // f0 = the acceleration so far
        if positive {
            c.cycles += 6;
            let k = ldf(c, 0, gp_3(c, STAT_SPEED_SCALE));
            let denom = fr(c, 1) * k + fr(c, 2);
            let ratio = fr(c, 2).div(denom);
            setf(c, 20, fr(c, 20) * ratio);
            c.mov_s(0, 20);
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Flags64_Test.

/// `bool Flags64_Test(Flags64* f, int bit)` (0x1446a0): whether bit `bit` of
/// the first 64-bit word is set (`bit` masked to 0..63).
pub fn flags64_test(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = c.ld(abi::arg(c, 0));
    c.g[V1][0] = 0x1;
    c.g[V1][0] = c.g[V1][0] << (c.g[A1][0] & 63);
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    abi::ret(c);
}
// 0.9250819683074951f
pub(crate) const FRICTION_SLOPE: i32 = -0x70e8;

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// A `gp_4`-relative global.
#[inline(always)]
pub(crate) fn gp_4(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// On the ground the board turns towards the velocity by a share of its
/// heading error every tick (0x13e6cc), stock once per 1/60 s. Keep the share
/// left per 1/60 s at finer ticks; a share outside (0, 1) stays as it is.
fn yaw_align_gain(hz: u32, gain: f32) -> f32 {
    if hz == 60 || !(gain > 0.0 && gain < 1.0) {
        return gain;
    }
    1.0 - (1.0 - gain).powf(60.0 / hz as f32)
}

/// `float RiderGroundState_ComputeFrictionAccel(RiderGroundState *gs,
/// const float *p, float a, float b)` (0x13c878): the hand-plant ground
/// friction model. Returns the friction acceleration and publishes the
/// normalised slip (`max(-a / m8, 0)`) to `rider + 0x758`.
pub fn rider_ground_state_compute_friction_accel(c: &mut Cpu) {
    const M4: u32 = ground_state::M4;
    const M8: u32 = ground_state::M8;

    let gs = Addr(abi::arg(c, 0));
    let p = Addr(abi::arg(c, 1));
    let a = abi::farg(c, 0); // f12
    let b = abi::farg(c, 1); // f13

    c.cycles += 5;
    c.mtc1(ZERO, 1); // f1 = 0.0
    let zero = fr(c, 1);
    let m8;
    let r;
    if abi::c_lt_s(c, zero, a) {
        // a > 0: sliding at constant speed, `r = p0 * a * -1/30`.
        c.cycles += 7;
        let slope = Addr(gp_4(c, FRICTION_SLOPE)).read_fpu(c);
        setf(c, 0, slope);
        let b_positive = abi::c_lt_s(c, zero, b);
        let p0 = p.read_fpu(c);
        setf(c, 1, p0);
        let scaled = a * slope;
        setf(c, 0, scaled);
        m8 = gs.off(M8).read_fpu(c);
        setf(c, 7, m8);
        let product = p0 * scaled;
        setf(c, 5, product);
        if b_positive {
            c.cycles += 4;
            let p7 = p.off(ground_state::COEFF_B).read_fpu(c);
            setf(c, 0, p7);
            let drag = p7 * b;
            setf(c, 0, drag);
            setf(c, 5, product - drag);
        }
        r = fr(c, 5);
    } else {
        let m4 = gs.off(M4).read_fpu(c);
        setf(c, 6, m4);
        c.cycles += 5;
        let neg_m4 = -m4;
        setf(c, 0, neg_m4);
        if abi::c_lt_s(c, neg_m4, a) {
            // -m4 < a <= 0: `r = -p0 * a / m4`.
            c.cycles += 8;
            let p0 = p.read_fpu(c);
            setf(c, 0, p0);
            let p7 = p.off(ground_state::COEFF_B).read_fpu(c);
            setf(c, 1, p7);
            let neg_p0 = -p0;
            setf(c, 0, neg_p0);
            m8 = gs.off(M8).read_fpu(c);
            setf(c, 7, m8);
            let scaled = neg_p0 * a;
            setf(c, 0, scaled);
            let quotient = scaled.div(m4);
            setf(c, 5, quotient);
            let drag = p7 * b;
            setf(c, 1, drag);
            c.cycles += 1;
            setf(c, 5, quotient - drag);
            r = fr(c, 5);
        } else {
            // a <= -m4: the deep-slip curve from m4 to m8. `f0` (m8) is
            // loaded before the block, as the original does, and both the
            // load and the clamp read it.
            m8 = gs.off(M8).read_fpu(c);
            setf(c, 0, m8);
            c.cycles += 14;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2); // f2 = 1.0
            setf(c, 3, -m8);
            setf(c, 7, m8);
            setf(c, 4, m8 - m4);
            let p7 = p.off(ground_state::COEFF_B).read_fpu(c);
            setf(c, 1, p7);
            let p0 = p.read_fpu(c);
            setf(c, 5, p0);
            let clamped = fr(c, 3).max(fr(c, 12));
            setf(c, 12, clamped);
            let drag = p7 * b;
            setf(c, 1, drag);
            let sum = clamped + m4;
            setf(c, 0, sum);
            let doubled = sum + sum;
            setf(c, 0, doubled);
            let span = fr(c, 4);
            let ratio = doubled.div(span);
            setf(c, 0, ratio);
            let one = fr(c, 2);
            let remaining = one - ratio;
            setf(c, 2, remaining);
            let curve = p0 * remaining;
            setf(c, 5, curve);
            c.cycles += 1;
            setf(c, 5, curve - drag);
            r = fr(c, 5);
        }
    }

    c.cycles += 8;
    let a_clamped = fr(c, 12);
    setf(c, 1, -a_clamped);
    let rider = gs.off(ground_state::RIDER).lw(c);
    c.g[V0][0] = rider;
    c.mtc1(ZERO, 2); // f2 = 0.0
    setf(c, 0, r);
    let neg_a = fr(c, 1);
    let slip_raw = neg_a.div(m8);
    setf(c, 1, slip_raw);
    let floor = fr(c, 2);
    setf(c, 1, slip_raw.max(floor));
    let slip = (rider as u32).wrapping_add(ground_state::SLIP);
    c.swc1(1, slip);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Riders.

/// `bool Rider_IsControlMode3(Rider* rider)` (0x1448a8): the control mode
/// word at +0x70 is 3 (the comparison is on the sign-extended 64-bit value,
/// as the original's is).
pub fn rider_is_control_mode3(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw(p(abi::arg(c, 0), 0x70));
    c.g[V0][0] ^= 0x3;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

/// World / race manager offsets used to find the rider's section record.
pub(crate) const WORLD: i32 = -0x848;

pub(crate) const SECTIONS: u32 = 0x44;

/// The turn gain `RiderAirState_Update` (0x139a20) hands
/// `Rider_UpdateOrientation` (0x121aa0) when less than one tick of the air
/// window is left. The gain is per second: the other branch passes `time scale
/// / window left`, and the consumer turns by `min(angle * gain, cap) * step`.
/// Here the window counts as one tick: the original `time scale * 59.999996`
/// (the reciprocal of `1/60`), `time scale / step` at any other rate.
#[inline]
pub(crate) fn air_last_turn_fraction(c: &mut Cpu, time_scale: FpuF32, fallback: FpuF32) -> FpuF32 {
    if crate::tick::rate(c) == 60 {
        time_scale * fallback
    } else {
        time_scale * FpuF32::ONE.div(FpuF32::from_f32(crate::tick::dt(c)))
    }
}

/// `void RiderAirState_Update(RiderAirState* self)` (0x139a20): the rider's
/// air-state tick: wind force, one air-prediction step, then (in the air)
/// choose the target orientation and the turn rate and hand them to
/// `Rider_UpdateOrientation`, finally syncing the implicit orientation.
pub fn rider_air_state_update(c: &mut Cpu) {
    c.cycles += 33;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x40);
    frame.save(c, S1, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);
    frame.save_fpr(c, 21, 0x58);
    frame.save_fpr(c, 20, 0x50);
    let this = r32(c, S0);
    // Wind force, then one step of the air prediction.
    c.g[A0][0] = c.lw(p(this, air::RIDER));
    if call_ptr(c, callee::AIR_APPLY_WIND, 0x0013_9a44).is_err() {
        return;
    }
    c.g[A1][0] = c.lw(p(this, air::RIDER));
    c.lwc1(0, gp_5(c, -0x71d4));
    c.lwc1(12, p(r32(c, A1), air::DT));
    c.g[A2][0] = sx(p(r32(c, A1), air::PRED_B));
    c.g[A0][0] = c.lw(p(r32(c, A1), air::ANIM));
    c.mul_s(12, 12, 0);
    c.g[A1][0] = sx(p(r32(c, A1), air::PRED_A));
    if call_ptr(c, callee::AIR_PREDICTOR_UPDATE, 0x0013_9a64).is_err() {
        return;
    }
    // Copy the up vector, then the constant gravity step.
    c.g[A1][0] = c.lw(p(this, air::RIDER));
    c.g[A0][0] = 0x50_0000;
    c.g[S1][0] = c.g[A0][0];
    c.lq(3, p(r32(c, A1), air::UP));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
    c.g[A2][0] = 1;
    c.sq(3, sp(c));
    c.sq(3, p(r32(c, A1), air::UP_COPY));
    c.lq(2, r32(c, A0));
    c.g[V1][0] = c.lw(p(this, air::RIDER));
    c.sq(2, p(r32(c, V1), air::GRAVITY));
    c.g[A0][0] = c.lw(p(this, air::RIDER));
    c.g[V0][0] = c.lw(p(r32(c, A0), air::ANIM));
    c.g[V1][0] = c.lw(p(r32(c, V0), anim::STATE));
    c.g[A3][0] = 0;
    let in_air = if c.g[V1][0] == c.g[A2][0] {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        c.g[V1][0] == c.g[V0][0]
    };
    if in_air {
        c.cycles += 1;
        c.g[A3][0] = 1;
    }
    c.cycles += 2;
    c.g[V1][0] = 0xb0;
    if c.g[A3][0] != 0 {
        'body: {
            // The rider's section record, and whether it is a boost section.
            c.cycles += 12;
            c.g[T1][0] = c.lw(p(this, air::RIDER));
            c.g[V0][0] = c.lw(gp_5(c, WORLD));
            c.g[T2][0] = 0;
            c.g[A2][0] = c.lw(p(r32(c, T1), air::ANIM));
            c.g[A1][0] = 0xffff_ffff_ffff_ffff;
            c.g[A0][0] = c.lw(p(r32(c, V0), 0x84));
            c.g[T0][0] = c.lw(p(r32(c, A2), anim::SECTION));
            c.g[V0][0] = c.lw(p(r32(c, A0), SECTIONS));
            c.mult(0, 3, 8, 3);
            c.g[A3][0] = c.lh(p(r32(c, A2), anim::FLAGS));
            let no_flags = c.g[A3][0] == c.g[A1][0];
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            if !no_flags {
                c.cycles += 2;
                c.g[V0][0] = c.g[A3][0] & 0x8;
                c.g[T2][0] = (0 < c.g[V0][0]) as u64;
            }
            c.cycles += 3;
            c.g[V0][0] = 0x12;
            if c.g[T0][0] != c.g[V0][0] {
                c.cycles += 3;
                c.g[V0][0] = c.lw(p(r32(c, V1), SECTIONS));
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    if c.g[T2][0] != 0 {
                        // Turn rate: dt over the air window, or the fixed step.
                        'rate: {
                            c.cycles += 8;
                            c.lwc1(0, p(r32(c, A2), anim::T1));
                            c.lwc1(1, p(r32(c, A2), anim::T0));
                            c.lwc1(2, gp_5(c, -0x71d0));
                            c.sub_s(1, 1, 0);
                            c.c_le_s(2, 1);
                            let small = !c.fcc();
                            c.lwc1(12, p(r32(c, T1), air::DT));
                            if !small {
                                c.cycles += 3;
                                c.div_s(20, 12, 1);
                                c.g[A0][0] = c.lw(p(this, air::RIDER));
                                break 'rate;
                            }
                            c.cycles += 3;
                            c.lwc1(0, gp_5(c, -0x71cc));
                            // Less than a tick of the window left: it counts
                            // as one tick.
                            let fraction =
                                air_last_turn_fraction(c, FpuF32(c.f[12]), FpuF32(c.f[0]));
                            c.f[20] = fraction.0;
                            c.g[A0][0] = c.lw(p(this, air::RIDER));
                        }
                        // Turn amount for the frame.
                        c.cycles += 7;
                        c.lwc1(1, gp_5(c, -0x71c8));
                        c.lwc1(0, p(r32(c, A0), air::DT));
                        c.mul_s(21, 0, 1);
                        if call_ptr(c, callee::RIDER_GET_SUBSTATE, 0x0013_9b5c).is_err() {
                            return;
                        }
                        c.g[V1][0] = 5;
                        let not_grind = c.g[V0][0] != c.g[V1][0];
                        c.g[A0][0] = c.lw(p(this, air::RIDER));
                        if !not_grind {
                            // On a rail: keep the up vector unless it has
                            // tilted too far.
                            c.cycles += 9;
                            c.g[V1][0] = 1;
                            c.lwc1(1, gp_5(c, -0x71c4));
                            c.g[V0][0] = c.lw(p(r32(c, A0), air::SUB));
                            c.lwc1(0, p(r32(c, V0), anim::ANGLE));
                            c.abs_s(0, 0);
                            c.c_lt_s(1, 0);
                            if !c.fcc() {
                                c.g[V1][0] = 0;
                            }
                            c.cycles += 2;
                            c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xee0));
                            if c.g[V1][0] != 0 {
                                c.cycles += 4;
                                c.g[A1][0] = c.lw(p(r32(c, A0), air::ANIM));
                                c.mov_s(12, 20);
                                c.mov_s(13, 21);
                                c.cycles += 4;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                if call_ptr(c, callee::RIDER_UPDATE_ORIENTATION, 0x0013_9c50)
                                    .is_err()
                                {
                                    return;
                                }
                                break 'body;
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.lw(p(this, air::RIDER));
                        }
                        // Steep or flipped: steer towards the target.
                        c.cycles += 7;
                        c.lwc1(1, gp_5(c, -0x71c0));
                        c.g[V1][0] = c.lw(p(r32(c, A0), air::ANIM));
                        c.lwc1(0, p(r32(c, V1), anim::THIN));
                        c.c_lt_s(0, 1);
                        let thin = c.fcc();
                        c.g[A1][0] = sx(p(r32(c, V1), 0x20));
                        if !thin {
                            c.cycles += 3;
                            let flat = c.lw(this) == 0;
                            c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xee0));
                            if flat {
                                c.cycles += 3;
                                c.mov_s(12, 20);
                                c.mov_s(13, 21);
                                if call_ptr(c, callee::RIDER_UPDATE_ORIENTATION, 0x0013_9c64)
                                    .is_err()
                                {
                                    return;
                                }
                                break 'body;
                            }
                        }
                        // Build the up-vector target, keep its facing.
                        c.cycles += 18;
                        c.lq(3, p(r32(c, V1), anim::ORIENT));
                        c.mtc1(0, 1);
                        c.sq(3, sp(c));
                        c.lq(2, p(r32(c, A0), air::ORIENT));
                        c.sq(2, p(sp(c), 0x10));
                        c.lqc2(3, sp(c));
                        c.lqc2(5, p(sp(c), 0x10));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(3, 4);
                        c.mtc1(3, 0);
                        c.c_lt_s(0, 1);
                        if !c.fcc() {
                            c.g[A0][0] = c.lw(p(this, air::RIDER));
                        } else {
                            c.cycles += 8;
                            c.g[AT][0] = 0xffff_ffff_bf80_0000;
                            c.mtc1(1, 0);
                            c.mfc1(2, 0);
                            c.lqc2(4, sp(c));
                            c.qmtc2(2, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, sp(c));
                            c.g[A0][0] = c.lw(p(this, air::RIDER));
                        }
                        c.cycles += 4;
                        c.mov_s(12, 20);
                        c.mov_s(13, 21);
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A1][0] = c.lw(p(r32(c, A0), air::ANIM));
                        c.cycles += 4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if call_ptr(c, callee::RIDER_UPDATE_ORIENTATION, 0x0013_9c50).is_err() {
                            return;
                        }
                    }
                }
            }
        }
    }
    c.cycles += 9;
    c.g[A0][0] = c.lw(p(this, air::RIDER));
    if call_ptr(c, callee::RIDER_ORIENTATION_IMPLICIT, 0x0013_9c6c).is_err() {
        return;
    }
    frame.restore(c, S0, 0x40);
    frame.restore(c, S1, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 21, 0x58);
    frame.restore_fpr(c, 20, 0x50);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Board ray.

/// `float RiderAirState_CastBoardRay(RiderAirState* self, Hit* hit)` (0x13a7b0):
/// shoot a 200-unit segment along the board's forward direction through the
/// rider's overlap list and return the distance of the closest contact (or
/// the negative result of the query). A hit with an object dispatches to that
/// object's contact handler.
pub fn rider_air_state_cast_board_ray(c: &mut Cpu) {
    c.cycles += 63;
    let frame = Frame::open(c, 0x160);
    c.g[AT][0] = 0x4348_0000;
    c.mtc1(1, 0);
    frame.save(c, S0, 0x140);
    c.g[A2][0] = sx(p(sp(c), 0xe0));
    frame.save(c, S1, 0x130);
    c.g[S0][0] = sx(p(sp(c), 0x10));
    frame.save(c, S2, 0x120);
    c.g[S1][0] = c.g[A0][0];
    frame.save_fpr(c, 20, 0x150);
    c.g[S2][0] = c.g[A1][0];
    frame.save_ra(c, 0x110);
    c.g[A1][0] = sx(p(sp(c), 0xc0));
    c.lwc1(12, gp_5(c, -0x71a4));
    c.g[A0][0] = c.g[S0][0];
    c.g[V0][0] = c.lw(p(r32(c, S1), 0x04));
    c.g[A3][0] = 2;
    c.g[V1][0] = c.lw(p(r32(c, V0), 0x780));
    c.lq(8, p(r32(c, V0), 0x180));
    c.g[T1][0] = c.lw(p(r32(c, V1), 0x2c));
    c.g[V1][0] = c.lw(p(r32(c, V0), 0x8a0));
    c.sq(8, sp(c));
    c.mfc1(2, 0);
    c.lqc2(4, sp(c));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0xe0));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
    c.lq(2, p(sp(c), 0xe0));
    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sq(2, p(sp(c), 0xd0));
    c.lqc2(3, r32(c, T1));
    c.lqc2(4, p(sp(c), 0xd0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp(c), 0xe0));
    c.lq(2, p(sp(c), 0xe0));
    c.sq(2, p(sp(c), 0xc0));
    c.mfc1(3, 0);
    c.lqc2(4, sp(c));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x100));
    c.lq(2, p(sp(c), 0x100));
    c.sq(2, p(sp(c), 0xf0));
    c.lqc2(3, r32(c, T1));
    c.lqc2(4, p(sp(c), 0xf0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp(c), 0x100));
    c.lq(2, p(sp(c), 0x100));
    c.sq(2, p(sp(c), 0xe0));
    if call_ptr(c, callee::COLL_SEGMENT_CTOR, 0x0013_a87c).is_err() {
        return;
    }
    c.g[V1][0] = c.lw(p(r32(c, S1), 0x04));
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.lw(p(r32(c, V1), 0x864));
    c.g[A0][0] = c.lw(p(r32(c, V1), 0x860));
    if call_ptr(c, callee::OVERLAP_LIST_FIND_CLOSEST, 0x0013_a894).is_err() {
        return;
    }
    c.mov_s(20, 0);
    c.mtc1(0, 0);
    c.c_le_s(0, 20);
    let behind = !c.fcc();
    c.mov_s(0, 20);
    let mut extra = behind;
    if !behind {
        c.cycles += 3;
        c.g[A0][0] = c.lw(p(r32(c, S2), 0x50));
        let no_hit = c.g[A0][0] == 0;
        c.lq(16, p(sp(c), 0x140));
        if !no_hit {
            c.cycles += 3;
            c.g[A2][0] = c.lw(p(r32(c, A0), 0x0c));
            let no_object = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S2][0];
            if !no_object {
                c.cycles += 6;
                c.g[V0][0] = c.lw(p(r32(c, A2), 0x0c));
                c.g[A0][0] = c.lh(p(r32(c, V0), 0x150));
                c.g[V1][0] = c.lw(p(r32(c, V0), 0x154));
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(r32(c, A0)));
                if call_ptr(c, target, 0x0013_a8d8).is_err() {
                    return;
                }
                c.mov_s(0, 20);
                extra = true;
            }
        }
    }
    if extra {
        c.cycles += 1;
        c.lq(16, p(sp(c), 0x140));
    }
    c.cycles += 6;
    frame.restore(c, S1, 0x130);
    frame.restore(c, S2, 0x120);
    frame.restore_ra(c, 0x110);
    frame.restore_fpr(c, 20, 0x150);
    frame.close(c);
}

/// `bool Flags64_TestSecond(u64* flags, int bit)` (0x1446b8): bit `bit` of the
/// 64-bit flag word at +8, non-zero as a 0/1 (and `v1` keeps the mask).
pub fn flags64_test_second(c: &mut Cpu) {
    c.cycles += 6;
    let selfp = abi::arg(c, 0);
    c.g[V0][0] = c.ld(selfp.wrapping_add(8));
    c.g[V1][0] = 0x1;
    c.g[V1][0] = c.g[V1][0] << (c.g[A1][0] & 63);
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `void Flags64_MoveToSecond(Flags64* f, int bit)` (0x144670): clear `bit` in
/// the first word (moving it into the second word's same bit).
pub fn flags64_move_to_second(c: &mut Cpu) {
    c.cycles += 11;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 1;
    c.g[V1][0] = Addr(self_).ld(c);
    c.g[V0][0] = c.g[V0][0] << (c.g[A1][0] & 63);
    c.g[A1][0] = Addr(at_2(self_, 8)).ld(c);
    c.g[A2][0] = !c.g[V0][0];
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
    c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
    let first = c.g[V1][0];
    Addr(self_).sd(c, first);
    let second = c.g[A1][0];
    Addr(at_2(self_, 8)).sd(c, second);
    abi::ret(c);
}

/// `RiderHandplantState_LoadState(self, iface)` (0x1399a8): `0xa0`.
pub fn rider_handplant_state_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_99d0, Some(0xa0)))
}

/// `RiderAirState_LoadState(self, iface)` (0x13ace8): `0x4`.
pub fn rider_air_state_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_ad10, Some(0x4)))
}

/// `RiderRailslideState_LoadState(self, iface)` (0x13c618): `0x50`.
pub fn rider_railslide_state_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_c640, Some(0x50)))
}

/// `RiderGroundState_LoadState(self, iface)` (0x13f880): `0x18`.
pub fn rider_ground_state_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_f8a8, Some(0x18)))
}

// ---------------------------------------------------------------------------
// Rider and game-state predicates.

/// `int Rider_IsComputer(Rider* rider)` (0x140bc8): `rider->+0x874 ^ 1`
/// (1 for the AI flag).
pub fn rider_is_computer(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x874)).lw(c);
    c.g[V0][0] ^= 0x1;
    abi::ret(c);
}

/// `void RiderAirState_Exit(RiderAirState* s)` (0x139a18): the air state has
/// nothing to tear down.
pub fn rider_air_state_exit(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// The rider pointer inside the air state.
pub(crate) const RIDER: u32 = 0x4;

/// A `gp_6`-relative global.
#[inline(always)]
pub(crate) fn gp_6(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// The low 32 bits of a GPR (what the generated `as u32` reads).
#[inline(always)]
pub(crate) fn g(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `*motion->rider`, read from `s0` exactly as the original does (the
/// pointer is not cached in a local: a callee that perturbs `s0` must move
/// the port the same way it moves the generated body).
#[inline(always)]
pub(crate) fn rider(c: &mut Cpu) -> u64 {
    let motion = r32(c, S0);
    Addr(p(motion, RIDER)).lw(c)
}

/// `lq r, src; sq r, dst` (a quadword moved through a GPR).
#[inline(always)]
pub(crate) fn qcopy(c: &mut Cpu, r: usize, src: u32, dst: u32) {
    c.lq(r, src);
    c.sq(r, dst);
}

/// The `sw a, src` of the original.
#[inline(always)]
pub(crate) fn st(c: &mut Cpu, a: u32, src: usize) {
    let v = c.g[src][0];
    c.sw(a, v);
}

// ---------------------------------------------------------------------------
// VU0 sequences (macro mode), exactly as the original issues them.

/// The tail of the dot-product sequence: `vf6` is `(1, 1, 1, 1)` and `vf4`
/// holds the products, so `vf4.x` ends as the dot product.
#[inline(always)]
pub(crate) fn dot_tail(c: &mut Cpu) {
    let vu = c.vu0();
    vu.adda(0b1000, 4, Src::Bc(4, Y));
    vu.madda(0b1000, 6, Src::Bc(4, Z));
    vu.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `vf4.x = v · v` (`vf3 = v`).
#[inline(always)]
pub(crate) fn dot_self(c: &mut Cpu, v: u32) {
    c.lqc2(3, v);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    dot_tail(c);
}

/// `vf4.x = a · b` (`vf3 = a`, `vf5 = b`).
#[inline(always)]
pub(crate) fn dot_ab(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    dot_tail(c);
}

/// `f[dst] = |v|` through Q (`vsqrt`, `cfc2`, `mtc1`).
#[inline(always)]
pub(crate) fn v_length(c: &mut Cpu, v: u32, via: usize, dst: usize) {
    dot_self(c, v);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(via, 22);
    c.mtc1(via, dst);
}

/// `out = v / |v|` (`vrsqrt`).
#[inline(always)]
pub(crate) fn v_normalize(c: &mut Cpu, v: u32, out: u32) {
    dot_self(c, v);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, out);
}

/// `out = a - b` (`vf3`, `vf4` → `vf5`).
#[inline(always)]
pub(crate) fn v_sub(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `out = a + b` (`vf3`, `vf4` → `vf5`).
#[inline(always)]
pub(crate) fn v_add(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

// ---------------------------------------------------------------------------
// RiderAirState_PostUpdate.

/// `void RiderAirState_PostUpdate(AirState* motion)` (0x139c88): board
/// contact, animation choice, bounds/wall update and camera notification.
pub fn rider_air_state_post_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x150);
        frame.save(c, S0, 0x120);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x138);
        frame.save(c, S1, 0x110);
        frame.save(c, S2, 0x100);
        frame.save_ra(c, 0xf0);
        frame.save_fpr(c, 22, 0x140);
        frame.save_fpr(c, 20, 0x130);
        callee::CAST_BOARD_RAY.call(c, 0x0013_9cb4)?;
        // The frame is fixed from here on: keep its addresses as plain locals
        // so they can be used next to `&mut Cpu` arguments.
        let sp = frame.at(c, 0);
        // `f21 = 0` (`mtc1 $zero, f21`), `f1 = 0.5`: the ray must reach half a
        // unit for the board contact to count.
        c.mtc1(0, 21);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 1);
        c.c_le_s(1, 0);
        'func: {
            'body: {
                if !c.fcc() {
                    c.g[A0][0] = rider(c);
                    break 'body;
                }
                c.cycles += 20;
                // `d = rider.pos - ray hit`; `f21 = d · hit.velocity`, negative
                // while the rider still moves onto the board.
                c.g[V0][0] = rider(c);
                c.lqc2(3, p(g(c, V0), rider::VEL));
                c.lqc2(4, p(sp, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp, 0x90));
                qcopy(c, 3, p(sp, 0x90), p(sp, 0x80));
                dot_ab(c, p(sp, 0x80), p(sp, 0x10));
                c.qmfc2(2, 4);
                c.mtc1(2, 21);
                c.c_le_s(1, 0);
                let t = !c.fcc();
                c.g[A0][0] = rider(c);
                if t {
                    break 'body;
                }
                c.cycles += 4;
                c.mtc1(0, 0);
                c.c_lt_s(21, 0);
                if !c.fcc() {
                    break 'body;
                }

                // ---- 0x139d7c: touching the board ---------------------------
                c.cycles += 15;
                let vel = p(g(c, A0), rider::VEL);
                v_length(c, vel, 2, 22);
                call_ptr(c, target::GET_SUB_STATE, 0x0013_9d60)?;
                c.g[V1][0] = 0x5;
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[A1][0] = rider(c);
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = rider(c);
                    c.g[A1][0] = 0xd;
                    call_ptr(c, target::SET_SUB_STATE, 0x0013_9d78)?;
                    c.g[A1][0] = rider(c);
                }

                // ---- 0x139de8: the trigger object under the board -----------
                'object: {
                    'join: {
                        c.cycles += 14;
                        // AirPredictor(*(rider + 0x788), rider + 0x110, rider +
                        // 0x1e0).
                        c.g[A0][0] = Addr(p(g(c, A1), 0x788)).lw(c);
                        c.g[A2][0] = sx(p(g(c, A1), rider::VEL));
                        c.g[A1][0] = sx(p(g(c, A1), rider::POS));
                        call_ptr(c, target::AIR_PREDICTOR, 0x0013_9d8c)?;
                        // Trigger entry `module->+0x44 + frame->+0x4c`, 0xb0
                        // bytes per entry.
                        c.g[V1][0] = Addr(gp_6(c, -0x848)).lw(c);
                        c.g[A1][0] = 0xb0;
                        c.g[V0][0] = Addr(p(sp, 0x4c)).lw(c);
                        c.g[A0][0] = Addr(p(g(c, V1), 0x84)).lw(c);
                        c.mult(0, 2, 2, 5);
                        c.g[V1][0] = Addr(p(g(c, A0), 0x44)).lw(c);
                        c.g[S1][0] = sx(p(g(c, V1), g(c, V0)));
                        c.g[A0][0] = Addr(p(g(c, S1), 0x44)).lw(c);
                        if c.g[A0][0] != 0 {
                            c.g[A0][0] = rider(c);
                            break 'join;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p(sp, 0x54)).lw(c);
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = rider(c);
                            break 'object;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = Addr(p(g(c, V0), 0xa)).lh(c);
                        c.g[V0][0] = c.g[V0][0] & 0x2;
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = rider(c);
                            break 'object;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = rider(c);
                        break 'join;
                    }
                    // The trigger was taken: reset the path and leave.
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 1;
                    call_ptr(c, target::CHECK_RESET_PATH, 0x0013_9de0)?;
                    break 'func;
                }

                // ---- 0x139e6c: blend the contact point ----------------------
                c.cycles += 22;
                c.lqc2(3, p(sp, 0));
                c.lqc2(4, p(g(c, V0), rider::POS));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp, 0x90));
                c.mtc1(0, 2);
                qcopy(c, 2, p(sp, 0x90), p(sp, 0x80));
                dot_ab(c, p(sp, 0x80), p(sp, 0x10));
                c.qmfc2(3, 4);
                c.mtc1(3, 1);
                c.lwc1(0, p(g(c, S1), 0x18));
                c.sub_s(1, 1, 0);
                c.c_lt_s(2, 1);
                if !c.fcc() {
                    c.g[V1][0] = rider(c);
                } else {
                    c.cycles += 11;
                    // `frame+0x10 *= f1` (the blend distance), then move.
                    c.mfc1(2, 1);
                    c.lqc2(4, p(sp, 0x10));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp, 0x90));
                    c.g[A1][0] = sx(p(sp, 0x80));
                    c.lq(2, p(sp, 0x90));
                    c.g[A0][0] = rider(c);
                    c.sq(2, p(sp, 0x80));
                    call_ptr(c, target::TRANSLATE, 0x0013_9e68)?;
                    c.g[V1][0] = rider(c);
                }

                // ---- 0x139edc: landing on the board -------------------------
                'bounce: {
                    'bounce_join: {
                        c.cycles += 14;
                        c.neg_s(0, 21);
                        c.lq(2, p(sp, 0x10));
                        c.sq(2, p(g(c, V1), rider::TRANSFORM));
                        c.g[A0][0] = rider(c);
                        c.lq(2, p(sp, 0x20));
                        c.sq(2, p(g(c, A0), rider::LAND_ANIM));
                        c.g[V1][0] = rider(c);
                        c.swc1(0, p(g(c, V1), 0x770));
                        c.g[A0][0] = rider(c);
                        c.lq(2, p(g(c, A0), rider::TRANSFORM));
                        c.sq(2, p(g(c, A0), 0x380));
                        c.g[V0][0] = Addr(p(sp, 0x54)).lw(c);
                        let t = c.g[V0][0] == 0;
                        c.mov_s(1, 0);
                        if t {
                            break 'bounce_join;
                        }
                        c.cycles += 10;
                        c.g[V1][0] = Addr(p(g(c, V0), 0x150)).lw(c);
                        c.g[V0][0] = rider(c);
                        let at = p(g(c, V0), rider::TRIGGER);
                        st(c, at, V1);
                        c.lwc1(0, p(sp, 0x6c));
                        c.g[A0][0] = rider(c);
                        c.swc1(0, p(g(c, A0), rider::BLEND_A));
                        c.g[V0][0] = rider(c);
                        c.lwc1(0, p(sp, 0x70));
                        c.swc1(0, p(g(c, V0), rider::BLEND_B));
                        break 'bounce;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = rider(c);
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    let at = p(g(c, V1), rider::TRIGGER);
                    st(c, at, V0);
                }

                // ---- 0x139fa0: bounce the velocity off the board ------------
                c.cycles += 35;
                c.g[V1][0] = rider(c);
                c.mfc1(4, 1);
                c.lqc2(4, p(g(c, V1), rider::TRANSFORM));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp, 0x90));
                c.lq(2, p(sp, 0x90));
                c.sq(2, p(sp, 0x80));
                c.lwc1(0, p(g(c, S1), 0x3c));
                c.mfc1(3, 0);
                c.lqc2(4, p(sp, 0x80));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp, 0x80));
                c.g[V0][0] = rider(c);
                let vel = p(g(c, V0), rider::VEL);
                v_add(c, vel, p(sp, 0x80), vel);
                c.g[V1][0] = rider(c);
                let vel1 = p(g(c, V1), rider::VEL);
                let xf = p(g(c, V1), rider::TRANSFORM);
                dot_ab(c, vel1, xf);
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.lwc1(1, p(g(c, S1), 0x40));
                c.neg_s(0, 0);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[A0][0] = Addr(p(sp, 0x50)).lw(c);
                if !t {
                    c.cycles += 14;
                    c.sub_s(0, 0, 1);
                    c.mfc1(2, 0);
                    c.lqc2(4, p(g(c, V1), rider::TRANSFORM));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp, 0xa0));
                    c.lq(2, p(sp, 0xa0));
                    c.g[V1][0] = rider(c);
                    c.sq(2, p(sp, 0x90));
                    let vel2 = p(g(c, V1), rider::VEL);
                    v_add(c, vel2, p(sp, 0x90), vel2);
                    c.g[A0][0] = Addr(p(sp, 0x50)).lw(c);
                }

                // ---- 0x13a020: drive the trigger animation ------------------
                'trigger: {
                    c.cycles += 2;
                    if c.g[A0][0] != 0 {
                        c.cycles += 3;
                        c.g[T0][0] = Addr(p(g(c, A0), 0xc)).lw(c);
                        let t = c.g[T0][0] == 0;
                        c.g[A1][0] = c.g[SP][0];
                        if !t {
                            c.cycles += 26;
                            // `frame+0x80` scaled by the trigger's `+0x158`
                            // and the world rate, then the object's callback.
                            c.g[A3][0] = Addr(p(g(c, T0), 0xc)).lw(c);
                            c.g[AT][0] = 0xffff_ffff_bf80_0000;
                            c.mtc1(1, 0);
                            c.g[A2][0] = sx(p(sp, 0x90));
                            c.g[A0][0] = Addr(p(g(c, A3), 0x158)).lh(c);
                            c.mfc1(3, 0);
                            c.lqc2(4, p(sp, 0x80));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, p(sp, 0xb0));
                            c.lq(2, p(sp, 0xb0));
                            let a = p(g(c, T0), g(c, A0));
                            c.g[A0][0] = sx(a);
                            c.g[V1][0] = Addr(gp_6(c, -0x848)).lw(c);
                            c.sq(2, p(sp, 0xa0));
                            c.lwc1(0, p(g(c, V1), 0x10));
                            c.cvt_s_w(0, 0);
                            c.mfc1(8, 0);
                            c.lqc2(4, p(sp, 0xa0));
                            c.qmtc2(8, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, p(sp, 0xb0));
                            c.lq(2, p(sp, 0xb0));
                            c.sq(2, p(sp, 0x90));
                            c.g[V1][0] = Addr(p(g(c, A3), 0x15c)).lw(c);
                            let target = g(c, V1);
                            call_ptr(c, target, 0x0013_a01c)?;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = rider(c);
                        break 'trigger;
                    }
                    c.g[A0][0] = rider(c);
                    break 'trigger;
                }

                // ---- 0x13a4cc: the airborne animation -----------------------
                'case: {
                    'state: {
                        // 0x13a1bc: the animation state decides the pose.
                        'anim: {
                            // 0x13a1a8
                            'seq: {
                                // 0x13a17c: board normal, up and side axes.
                                c.cycles += 81;
                                c.g[A1][0] = 0x2;
                                c.g[S1][0] = 0x1b6;
                                c.lq(2, p(g(c, A0), 0x1b0));
                                c.sq(2, p(sp, 0xb0));
                                c.lq(3, p(g(c, A0), 0x1b0));
                                c.sq(3, p(sp, 0xd0));
                                let xf = p(g(c, A0), rider::TRANSFORM);
                                dot_ab(c, p(sp, 0xd0), xf);
                                c.qmfc2(2, 4);
                                c.mtc1(2, 0);
                                c.mfc1(3, 0);
                                c.lqc2(4, p(g(c, A0), rider::TRANSFORM));
                                c.qmtc2(3, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, p(sp, 0xe0));
                                c.lq(2, p(sp, 0xe0));
                                c.sq(2, p(sp, 0xc0));
                                v_sub(c, p(sp, 0xb0), p(sp, 0xc0), p(sp, 0xe0));
                                c.lq(2, p(sp, 0xe0));
                                c.sq(2, p(sp, 0xa0));
                                v_normalize(c, p(sp, 0xa0), p(sp, 0xe0));
                                c.lq(2, p(sp, 0xe0));
                                c.sq(2, p(sp, 0x90));
                                c.sq(2, p(g(c, A0), rider::UP_2));
                                c.g[V1][0] = rider(c);
                                c.lqc2(4, p(g(c, V1), rider::TRANSFORM));
                                c.lqc2(5, p(g(c, V1), rider::UP_2));
                                c.vu0().opmula(4, 5);
                                c.vu0().opmsub(6, 5, 4);
                                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                                c.sqc2(6, p(sp, 0xa0));
                                c.lq(2, p(sp, 0xa0));
                                c.sq(2, p(sp, 0x90));
                                c.sq(2, p(g(c, V1), rider::SIDE));
                                c.g[A0][0] = rider(c);
                                c.g[V0][0] = Addr(p(sp, 0x4c)).lw(c);
                                let at = p(g(c, A0), rider::CONTACT);
                                st(c, at, V0);
                                c.g[A2][0] = rider(c);
                                c.lq(2, p(sp, 0));
                                c.sq(2, p(g(c, A2), rider::XFORM));
                                c.g[V1][0] = rider(c);
                                c.lqc2(3, p(g(c, V1), rider::POS));
                                c.lqc2(4, p(sp, 0));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, p(sp, 0xa0));
                                c.lq(4, p(sp, 0xa0));
                                c.g[V0][0] = rider(c);
                                c.sq(4, p(sp, 0x90));
                                let xf = p(g(c, V0), rider::TRANSFORM);
                                dot_ab(c, p(sp, 0x90), xf);
                                c.qmfc2(3, 4);
                                c.mtc1(3, 0);
                                c.swc1(0, p(g(c, V0), rider::NORMAL));
                                c.g[V1][0] = rider(c);
                                c.g[A0][0] = Addr(p(g(c, V1), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER, 0x0013_a158)?;
                                c.g[V1][0] = 0x14;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[V0][0] = rider(c);
                                if !t {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x2;
                                    c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                    call_ptr(c, target::ANIM_LAYER_SEQUENCE, 0x0013_a170)?;
                                    c.g[A0][0] = sx(p(g(c, V0), 0xb0));
                                    c.g[A1][0] = 0x2;
                                    break 'seq;
                                }
                                // 0x13a188: the air (not board) animation.
                                c.cycles += 6;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER, 0x0013_a188)?;
                                c.g[V1][0] = 0x15;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[V0][0] = rider(c);
                                if t {
                                    break 'anim;
                                }
                                c.cycles += 5;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER_SEQUENCE, 0x0013_a1a0)?;
                                c.g[A0][0] = sx(p(g(c, V0), 0xb0));
                                c.g[A1][0] = 0x3;
                                break 'seq;
                            }
                            // 0x13a1b0: test the animation layer's flags.
                            c.cycles += 5;
                            callee::FLAGS64_TEST_2.call(c, 0x0013_a1b0)?;
                            c.g[V1][0] = 0x169;
                            if c.g[V0][0] == 0 {
                                c.g[S1][0] = c.g[V1][0];
                            }
                            break 'state;
                        }

                        // ---- 0x13a258: wall/rail animation selection ---------
                        'wall: {
                            'kind: {
                                // 0x13a1f8
                                'first: {
                                    c.cycles += 7;
                                    c.g[A1][0] = 0x2;
                                    c.g[S2][0] = 0;
                                    c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                    call_ptr(c, target::ANIM_LAYER, 0x0013_a1cc)?;
                                    c.g[V1][0] = 0x13;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[V0][0] = rider(c);
                                    if t {
                                        break 'first;
                                    }
                                    c.cycles += 8;
                                    c.g[A1][0] = 0x2;
                                    c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                    call_ptr(c, target::ANIM_LAYER_SEQUENCE, 0x0013_a1e4)?;
                                    c.g[A0][0] = sx(p(g(c, V0), 0xb0));
                                    c.g[A1][0] = 0x2;
                                    callee::FLAGS64_TEST_2.call(c, 0x0013_a1f0)?;
                                    c.g[S2][0] = (c.g[V0][0] < 1) as u64;
                                    break 'kind;
                                }
                                // 0x13a204
                                c.cycles += 6;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER, 0x0013_a204)?;
                                c.g[V1][0] = 0x12;
                                if c.g[V0][0] != c.g[V1][0] {
                                    c.g[A0][0] = rider(c);
                                    break 'wall;
                                }
                                c.cycles += 9;
                                c.g[V0][0] = rider(c);
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER_SEQUENCE, 0x0013_a220)?;
                                c.g[A0][0] = sx(p(g(c, V0), 0xb0));
                                c.g[A1][0] = 0x2;
                                callee::FLAGS64_TEST_2.call(c, 0x0013_a22c)?;
                                if c.g[V0][0] != 0 {
                                    c.g[A0][0] = rider(c);
                                    break 'wall;
                                }
                                c.cycles += 8;
                                c.g[V0][0] = rider(c);
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = Addr(p(g(c, V0), rider::ANIM)).lw(c);
                                call_ptr(c, target::ANIM_LAYER_SEQUENCE, 0x0013_a244)?;
                                c.g[A0][0] = sx(p(g(c, V0), 0xb0));
                                c.g[A1][0] = 0;
                                callee::FLAGS64_TEST_2.call(c, 0x0013_a250)?;
                                c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
                                break 'kind;
                            }
                            // 0x13a258
                            c.cycles += 1;
                            c.g[A0][0] = rider(c);
                            break 'wall;
                        }

                        // ---- 0x13a298: board side check ----------------------
                        c.cycles += 16;
                        c.mtc1(0, 5);
                        c.lq(2, p(g(c, A0), 0x1c0));
                        c.sq(2, p(sp, 0x90));
                        let xf = p(g(c, A0), rider::TRANSFORM);
                        dot_ab(c, p(sp, 0x90), xf);
                        c.qmfc2(3, 4);
                        c.mtc1(3, 0);
                        c.c_lt_s(0, 5);
                        if c.fcc() {
                            c.g[S2][0] = 0x1;
                        }

                        // ---- 0x13a424: pick the pose from the board angle ----
                        {
                            c.cycles += 2;
                            if c.g[S2][0] == 0 {
                                c.g[V0][0] = Addr(p(g(c, A0), 0x330)).lw(c);
                            } else {
                                // Up/down/side components of the board normal.
                                'angle: {
                                    c.cycles += 38;
                                    let xf = p(g(c, A0), rider::TRANSFORM);
                                    c.lq(2, p(g(c, A0), 0x1a0));
                                    c.sq(2, p(sp, 0x90));
                                    dot_ab(c, p(sp, 0x90), xf);
                                    c.qmfc2(3, 4);
                                    c.mtc1(3, 4);
                                    c.lq(2, p(g(c, A0), 0x1b0));
                                    c.abs_s(0, 4);
                                    c.sq(2, p(sp, 0x90));
                                    dot_ab(c, p(sp, 0x90), xf);
                                    c.qmfc2(3, 4);
                                    c.mtc1(3, 2);
                                    c.abs_s(3, 2);
                                    c.lq(2, p(g(c, A0), 0x1c0));
                                    c.c_lt_s(0, 3);
                                    c.sq(2, p(sp, 0x90));
                                    dot_ab(c, p(sp, 0x90), xf);
                                    c.qmfc2(3, 4);
                                    c.mtc1(3, 1);
                                    if !c.fcc() {
                                        break 'angle;
                                    }
                                    c.cycles += 4;
                                    c.abs_s(0, 1);
                                    c.c_lt_s(0, 3);
                                    if !c.fcc() {
                                        break 'angle;
                                    }
                                    c.cycles += 4;
                                    c.c_lt_s(2, 5);
                                    let t = !c.fcc();
                                    c.g[S1][0] = 0x163;
                                    if !t {
                                        c.cycles += 2;
                                        c.g[S1][0] = 0x162;
                                    }
                                    break 'state;
                                }
                                // 0x13a408: no usable board angle: use the state.
                                'back: {
                                    c.cycles += 5;
                                    c.abs_s(3, 1);
                                    c.abs_s(0, 4);
                                    c.c_lt_s(0, 3);
                                    if !c.fcc() {
                                        break 'back;
                                    }
                                    c.cycles += 4;
                                    c.abs_s(0, 2);
                                    c.c_lt_s(0, 3);
                                    if !c.fcc() {
                                        break 'back;
                                    }
                                    c.cycles += 5;
                                    c.mtc1(0, 0);
                                    c.c_lt_s(1, 0);
                                    let t = c.fcc();
                                    c.g[S1][0] = 0x15e;
                                    if t {
                                        break 'state;
                                    }
                                    // 0x13a3f0: a random 0x15f/0x160/0x161.
                                    {
                                        // 0x13a3e4
                                        'rand_odd: {
                                            c.cycles += 15;
                                            c.g[V0][0] = rider(c);
                                            c.lwc1(1, gp_6(c, -0x71bc));
                                            let vel = p(g(c, V0), rider::VEL);
                                            v_length(c, vel, 3, 0);
                                            c.c_lt_s(1, 0);
                                            if !c.fcc() {
                                                break 'rand_odd;
                                            }
                                            c.cycles += 4;
                                            c.g[S1][0] = 0x160;
                                            call_ptr(c, target::RAND_NEXT_GAME, 0x0013_a3dc)?;
                                            c.g[V0][0] = c.g[V0][0] & 0x3;
                                        }
                                        // 0x13a3ec
                                        c.cycles += 3;
                                        c.g[S1][0] = 0x160;
                                        call_ptr(c, target::RAND_NEXT_GAME, 0x0013_a3ec)?;
                                        c.g[V0][0] = c.g[V0][0] & 0x1;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0;
                                    c.g[V1][0] = c.g[V0][0] ^ 0x1;
                                    if t {
                                        break 'state;
                                    }
                                    c.cycles += 4;
                                    c.g[S1][0] = 0x15f;
                                    c.g[V0][0] = 0x161;
                                    if c.g[V1][0] == 0 {
                                        c.g[S1][0] = c.g[V0][0];
                                    }
                                    break 'state;
                                }
                                // 0x13a414: last resort, the animation state.
                                c.cycles += 5;
                                c.mtc1(0, 0);
                                c.c_lt_s(4, 0);
                                let t = !c.fcc();
                                c.g[S1][0] = 0x164;
                                if !t {
                                    c.cycles += 2;
                                    c.g[S1][0] = 0x165;
                                }
                                break 'state;
                            }
                        }
                        // 0x13a4b0
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0;
                        c.g[V0][0] = 0x1b6;
                        if t {
                            break 'case;
                        }
                        'boost: {
                            c.cycles += 31;
                            c.lq(2, p(g(c, A0), 0x1c0));
                            c.sq(2, p(sp, 0x90));
                            let up = p(g(c, A0), rider::UP_2);
                            dot_ab(c, p(sp, 0x90), up);
                            c.qmfc2(3, 4);
                            c.mtc1(3, 20);
                            callee::BE_INSTANCE.call(c, 0x0013_a460)?;
                            c.g[A0][0] = sx(g(c, V0));
                            c.g[A1][0] = 0x3;
                            callee::BE_INTERFACE.call(c, 0x0013_a46c)?;
                            c.g[V1][0] = rider(c);
                            c.g[A0][0] = sx(g(c, V0));
                            c.g[A2][0] = Addr(p(g(c, V1), 0xb34)).lw(c);
                            c.g[A1][0] = Addr(p(g(c, V1), 0x86c)).lw(c);
                            callee::ATTRIB6_FACTOR.call(c, 0x0013_a480)?;
                            c.lwc1(2, gp_6(c, -0x71b8));
                            c.g[AT][0] = 0x3f80_0000;
                            c.mtc1(1, 3);
                            c.mul_s(0, 0, 2);
                            c.lwc1(1, gp_6(c, -0x71b4));
                            c.add_s(2, 0, 3);
                            c.mul_s(1, 2, 1);
                            c.c_lt_s(1, 20);
                            if !c.fcc() {
                                break 'boost;
                            }
                            c.cycles += 2;
                            c.g[S1][0] = 0x15c;
                            break 'state;
                        }
                        // 0x13a4da
                        c.cycles += 6;
                        c.lwc1(0, gp_6(c, -0x71b0));
                        c.mul_s(0, 2, 0);
                        c.c_lt_s(20, 0);
                        if c.fcc() {
                            c.g[S1][0] = 0x15d;
                        }
                        break 'state;
                    }
                    // 0x13a4c8 exit: the default animation.
                    c.cycles += 1;
                    c.g[V0][0] = 0x1b6;
                }

                // ---- 0x13a540: apply the chosen animation -------------------
                c.cycles += 2;
                let t = c.g[S1][0] == c.g[V0][0];
                c.lq(2, p(sp, 0));
                if !t {
                    c.cycles += 27;
                    c.abs_s(0, 21);
                    c.g[V1][0] = rider(c);
                    c.g[A1][0] = c.g[S1][0];
                    c.sq(2, p(sp, 0x90));
                    c.g[A2][0] = 0;
                    let vel = p(g(c, V1), rider::VEL);
                    v_normalize(c, vel, p(sp, 0xe0));
                    c.g[T0][0] = sx(p(sp, 0x90));
                    c.lq(3, p(sp, 0xe0));
                    c.lq(2, p(sp, 0x10));
                    c.g[A0][0] = rider(c);
                    c.sq(3, p(sp, 0xa0));
                    c.sq(2, p(sp, 0xb0));
                    c.swc1(0, p(sp, 0xc0));
                    c.sq(3, p(sp, 0xd0));
                    c.g[A3][0] = Addr(p(g(c, A0), rider::CONTACT)).lw(c);
                    call_ptr(c, target::START_WIPEOUT, 0x0013_a538)?;
                    c.g[A0][0] = rider(c);
                    break 'body;
                }

                // ---- 0x13a560: the "on the ground" flag --------------------
                c.cycles += 7;
                c.g[A0][0] = rider(c);
                c.lwc1(1, gp_6(c, -0x71ac));
                c.lwc1(0, p(g(c, A0), 0x378));
                c.c_lt_s(0, 1);
                let t = c.fcc();
                c.g[A1][0] = 0x1;
                if !t {
                    c.cycles += 1;
                    c.g[A1][0] = 0;
                }

                // ---- 0x13a684: trick / landing animation -------------------
                'land_anim: {
                    c.cycles += 13;
                    c.g[A3][0] = Addr(p(g(c, A0), 0x330)).lw(c);
                    c.mov_s(12, 22);
                    c.g[A2][0] = 0;
                    c.g[T0][0] = c.g[SP][0];
                    call_ptr(c, target::BANK_TRICK_BOOST, 0x0013_a574)?;
                    c.g[A0][0] = rider(c);
                    c.g[A1][0] = 0;
                    call_ptr(c, target::SET_MAIN_STATE, 0x0013_a580)?;
                    c.g[A0][0] = rider(c);
                    call_ptr(c, target::GET_SUB_STATE, 0x0013_a588)?;
                    c.g[V1][0] = 0xc;
                    if c.g[V0][0] == c.g[V1][0] {
                        c.g[V1][0] = rider(c);
                        break 'land_anim;
                    }
                    'land2: {
                        'sub: {
                            c.cycles += 5;
                            c.g[A0][0] = rider(c);
                            call_ptr(c, target::GET_SUB_STATE, 0x0013_a59c)?;
                            c.g[V1][0] = 0x2;
                            if c.g[V0][0] != c.g[V1][0] {
                                c.g[A0][0] = rider(c);
                                break 'sub;
                            }
                            c.cycles += 7;
                            c.g[A0][0] = rider(c);
                            call_ptr(c, target::RESET_GRIND, 0x0013_a5b0)?;
                            c.lwc1(0, gp_6(c, -0x71a8));
                            c.c_lt_s(21, 0);
                            let t = !c.fcc();
                            c.mov_s(12, 21);
                            if t {
                                break 'land2;
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            callee::PLAY_LAND_ANIM2.call(c, 0x0013_a5cc)?;
                            c.g[V1][0] = rider(c);
                            break 'land_anim;
                        }
                        // 0x13a644
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p(g(c, A0), 0x330)).lw(c);
                        let t = c.g[V0][0] == 0;
                        if !t {
                            c.cycles += 5;
                            call_ptr(c, target::GET_SUB_STATE, 0x0013_a5e8)?;
                            c.g[A0][0] = 0x1;
                            let t = c.g[V0][0] == c.g[A0][0];
                            c.g[V1][0] = rider(c);
                            if t {
                                break 'land_anim;
                            }
                            'land: {
                                'land_state: {
                                    c.cycles += 3;
                                    c.g[V0][0] = Addr(p(g(c, V1), 0x330)).lw(c);
                                    let t = c.g[V0][0] != c.g[A0][0];
                                    c.g[A0][0] = Addr(p(g(c, V1), rider::ANIM)).lw(c);
                                    if t {
                                        break 'land_state;
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = 0x1a;
                                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                                    c.mtc1(1, 12);
                                    c.g[A2][0] = 0;
                                    call_ptr(c, target::PLAY_ANIM, 0x0013_a614)?;
                                    c.g[A0][0] = rider(c);
                                    break 'land;
                                }
                                // 0x13a624
                                c.cycles += 6;
                                c.g[A1][0] = 0x22;
                                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                                c.mtc1(1, 12);
                                c.g[A2][0] = 0;
                                call_ptr(c, target::PLAY_ANIM, 0x0013_a630)?;
                                c.g[A0][0] = rider(c);
                            }
                            // 0x13a63c
                            c.cycles += 4;
                            c.g[A1][0] = 0x1;
                            call_ptr(c, target::SET_SUB_STATE, 0x0013_a63c)?;
                            c.g[V1][0] = rider(c);
                            break 'land_anim;
                        }
                        // 0x13a678
                        c.cycles += 9;
                        call_ptr(c, target::RESET_GRIND, 0x0013_a64c)?;
                        c.g[A0][0] = rider(c);
                        c.g[A1][0] = 0;
                        call_ptr(c, target::SET_SUB_STATE, 0x0013_a658)?;
                        c.g[A0][0] = rider(c);
                        call_ptr(c, target::TURN_AROUND, 0x0013_a660)?;
                        let t = c.g[V0][0] == 0;
                        c.mov_s(12, 21);
                        if !t {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            callee::PLAY_LAND_ANIM.call(c, 0x0013_a670)?;
                            c.g[V1][0] = rider(c);
                            break 'land_anim;
                        }
                        // 0x13a680
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        callee::PLAY_LAND_ANIM2.call(c, 0x0013_a680)?;
                        break 'land2;
                    }
                    // 0x13a680 exit
                    c.cycles += 1;
                    c.g[V1][0] = rider(c);
                }

                // ---- 0x13a700: cap the downward speed -----------------------
                c.cycles += 31;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 1);
                c.lq(2, p(g(c, V1), 0x1b0));
                c.mov_s(2, 1);
                c.sq(2, p(sp, 0x90));
                let vel = p(g(c, V1), rider::VEL);
                v_normalize(c, vel, p(sp, 0xb0));
                c.lq(2, p(sp, 0xb0));
                c.sq(2, p(sp, 0xa0));
                dot_ab(c, p(sp, 0x90), p(sp, 0xa0));
                c.qmfc2(3, 4);
                c.mtc1(3, 0);
                c.abs_s(0, 0);
                c.c_le_s(1, 0);
                if c.fcc() {
                    c.min_s(2, 0, 2);
                }
                // 0x13a710
                c.cycles += 9;
                c.g[V0][0] = rider(c);
                c.mfc1(3, 2);
                c.lqc2(4, p(g(c, V0), rider::VEL));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(g(c, V0), rider::VEL));
                c.g[A0][0] = Addr(gp_6(c, 0xce8)).lw(c);
                call_ptr(c, target::WSCRIPT, 0x0013_a720)?;
                c.g[A0][0] = rider(c);
                break 'body;
            }

            // ---- 0x13a784: bounds, walls and camera -------------------------
            'walls: {
                'collide: {
                    c.cycles += 16;
                    c.g[A1][0] = 0;
                    callee::RIDER_UPDATE_BOUNDS.call(c, 0x0013_a72c)?;
                    c.g[V1][0] = rider(c);
                    c.g[S1][0] = sx(p(sp, 0x80));
                    c.g[A2][0] = Addr(gp_6(c, -0x1fd0)).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = Addr(p(g(c, V1), rider::WALLS)).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    let at = p(g(c, V0), 0x28);
                    st(c, at, A2);
                    callee::COLLIDE_WALLS.call(c, 0x0013_a74c)?;
                    c.g[A1][0] = rider(c);
                    c.g[V1][0] = 0xffff_ffff_ffff_0000;
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.g[A0][0] = Addr(p(g(c, A1), rider::WALLS)).lw(c);
                    let t = c.g[V0][0] == 0;
                    let at = p(g(c, A0), 0x28);
                    st(c, at, V1);
                    if t {
                        break 'collide;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = rider(c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, target::AIR_WALLS, 0x0013_a770)?;
                    break 'walls;
                }
                // 0x13a784
                c.cycles += 3;
                c.g[A0][0] = rider(c);
                c.g[A1][0] = 0;
                call_ptr(c, target::AIR_WALLS, 0x0013_a784)?;
            }
            // 0x13a78c: the camera trigger manager sees the final position.
            c.cycles += 3;
            c.g[A0][0] = rider(c);
            call_ptr(c, target::AIR_CAMERA, 0x0013_a78c)?;
            break 'func;
        }

        // ---- epilogue -------------------------------------------------------
        c.cycles += 8;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore(c, S2, 0x100);
        frame.restore_ra(c, 0xf0);
        frame.restore_fpr(c, 22, 0x140);
        frame.restore_fpr(c, 21, 0x138);
        frame.restore_fpr(c, 20, 0x130);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderAirState_PlayLandAnim(Rider* r, float v)` (0x13a8f8): play the
/// hard-landing animation (0x41) and clear the landing flag when `v` is below
/// −2222.22, otherwise the soft landing (0x40).
pub fn rider_air_state_play_land_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.lwc1(0, gp_7(c, -0x71a0));
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        c.c_lt_s(12, 0);
        frame.save_ra(c, 0);
        let hard = !c.fcc();
        c.g[S0][0] = c.g[A0][0];
        if !hard {
            c.cycles += 10;
            c.g[V0][0] = Addr(p_2(r32(c, S0), 4)).lw(c);
            c.g[A1][0] = 0x41;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr(p_2(c.g[V0][0] as u32, 0x784)).lw(c);
            call::call_ptr(c, 0x0031_28e8, 0x0013_a930)?;
            c.g[V1][0] = Addr(p_2(r32(c, S0), 4)).lw(c);
            Addr(p_2(c.g[V1][0] as u32, 0x2dc)).sw(c, 0);
        } else {
            c.cycles += 7;
            c.g[V0][0] = Addr(p_2(r32(c, S0), 4)).lw(c);
            c.g[A1][0] = 0x40;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr(p_2(c.g[V0][0] as u32, 0x784)).lw(c);
            call::call_ptr(c, 0x0031_28e8, 0x0013_a958)?;
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

/// `void RiderRailslideState_Enter(self)` (0x13ad20): lean pose, orientation,
/// clear the railslide's balance/impulse accumulators and seed the state.
pub fn rider_railslide_state_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = lw_2(c, S0, 0x50);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x110));
        call_ptr(c, 0x0011_fa10, 0x0013_ad3c)?;
        c.g[A0][0] = lw_2(c, S0, 0x50);
        call_ptr(c, 0x0011_e098, 0x0013_ad44)?;
        c.g[V1][0] = lw_2(c, S0, 0x50);
        c.lwc1(0, gp_8(c, -0x7184));
        // The rail-value slew's step is 0.05 per 60 Hz tick (`+0x260`).
        c.f[0] = rate_step_60(c, FpuF32(c.f[0])).0;
        sw(c, V1, 0x1f4, 0);
        sw(c, V1, 0x1f8, 0);
        sw(c, V1, 0x1f0, 0);
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x0));
        sw(c, V0, 0x20c, 0);
        sw(c, V0, 0x210, 0);
        sw(c, V0, 0x208, 0);
        c.g[V1][0] = lw_2(c, S0, 0x50);
        sw(c, V1, 0x254, 0);
        sw(c, V1, 0x258, 0);
        sw(c, V1, 0x250, 0);
        sw(c, S0, 0x10, 0);
        c.g[V1][0] = lw_2(c, S0, 0x50);
        sw(c, S0, 0x14, 0);
        sw(c, S0, 0x18, 0);
        sw(c, V1, 0x2dc, 0);
        c.g[V0][0] = lw_2(c, S0, 0x50);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x260));
        sw(c, V0, 0x264, 0);
        sw(c, S0, 0x20, 0);
        sw(c, S0, 0x1c, 0);
        c.g[V0][0] = lw_2(c, S0, 0x50);
        c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x110));
        sw(c, S0, 0x40, 0);
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x30));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderGroundState_Enter / _Exit.

/// The AI frame counter (`AI_GetFrameCounter`, AI +0x8, one per tick).
const AI_GET_FRAME_COUNTER: u32 = 0x0012_98c8;

/// `void RiderGroundState_Enter(RiderGroundState* self)` (0x13c7a8): reset the
/// state, take the section's ground friction pair scaled by the model's
/// factor (`+0x4`/`+0x8`), copy the rider's position to `+0x390`, stamp the
/// landing time (`+0x10`) and scale the rider's velocity (`+0x1e0`) by the
/// landing speed factor: 0.7 after up to 40 frames in the air, then +0.01 per
/// further frame, clamped to 1.
///
/// The air time is `now - self+0x14` in ticks of the AI frame counter; off
/// 60 Hz it is converted to 60 Hz frames (`ticks * 60 / rate`, in float)
/// before the formula, so a hop of the same real length lands with the same
/// factor at any rate. At 60 the original integer path runs unchanged.
pub fn rider_ground_state_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0xb0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let gp = c.g[GP][0] as u32;
        c.g[A0][0] = c.lw(gp.wrapping_sub(0x848));
        let this = c.g[S0][0] as u32;
        c.sw(this, 0u64);
        c.g[A1][0] = c.lw(this.wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x438));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        // v1 = ground index * 0xb0 (the section record stride).
        c.mult(0, 3, 3, 6);
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x140));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x14));
        c.mul_s(0, 2, 0);
        c.swc1(0, this.wrapping_add(0x4));
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x18));
        c.mul_s(2, 2, 1);
        c.swc1(2, this.wrapping_add(0x8));
        c.lq(2, (c.g[A1][0] as u32).wrapping_add(0x370));
        c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x390));
        c.sw(this.wrapping_add(0xc), 0u64);
        call_ptr(c, AI_GET_FRAME_COUNTER, 0x0013_c80c)?;
        c.g[V1][0] = c.lw(this.wrapping_add(0x14));
        c.sw(this.wrapping_add(0x10), c.g[V0][0]);
        // Air time in ticks.
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 41i64) as u64;
        let short = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x28));
        let hz = crate::tick::rate(c);
        if hz == 60 {
            if short {
                c.cycles += 1;
                c.lwc1(1, gp.wrapping_sub(0x70ec));
            } else {
                c.cycles += 7;
                c.lwc1(2, gp.wrapping_sub(0x70f4));
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.lwc1(1, gp.wrapping_sub(0x70f0));
                c.mul_s(0, 0, 2);
                c.add_s(1, 0, 1);
            }
        } else {
            // The air time in 60 Hz frames.
            let frames = (c.g[V1][0] as u32 as i32) as f32 * 60.0 / hz as f32;
            if frames > 40.0 {
                c.cycles += 7;
                c.lwc1(2, gp.wrapping_sub(0x70f4));
                c.f[0] = (frames - 40.0).to_bits();
                c.lwc1(1, gp.wrapping_sub(0x70f0));
                c.mul_s(0, 0, 2);
                c.add_s(1, 0, 1);
            } else {
                c.cycles += 1;
                c.lwc1(1, gp.wrapping_sub(0x70ec));
            }
        }
        c.cycles += 13;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[V0][0] = c.lw(this.wrapping_add(0x18));
        c.min_s(0, 1, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.mfc1(3, 0);
        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x1e0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[V0][0] as u32).wrapping_add(0x1e0));
        frame.close(c);
        Ok(())
    })
}

/// `void RiderGroundState_Exit(RiderGroundState* self)` (0x13f410): reset
/// three of the rider's slews (`+0x2c0`, `+0x20c`, `+0x2cc`) and stamp the
/// take-off time (`+0x14`) from the AI frame counter. The first exit (stamp
/// still `-1`) stores `now - 500` frames ("left the ground long ago", read by
/// `RiderGroundState_Enter`); the 500 is converted to ticks at the current
/// rate (exact at 60).
pub fn rider_ground_state_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        let gp = c.g[GP][0] as u32;
        c.lwc1(0, gp.wrapping_sub(0x6fa8));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.lwc1(1, gp.wrapping_sub(0x6fa4));
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = c.lw(this.wrapping_add(0x18));
        c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x2c0));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x2c4), 0u64);
        c.g[V0][0] = c.lw(this.wrapping_add(0x18));
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20c));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x210), 0u64);
        c.g[V1][0] = c.lw(this.wrapping_add(0x18));
        c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x2cc));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x2d0), 0u64);
        c.g[V0][0] = c.lw(this.wrapping_add(0x14));
        if c.g[V0][0] != c.g[A0][0] {
            c.cycles += 2;
            call_ptr(c, AI_GET_FRAME_COUNTER, 0x0013_f474)?;
        } else {
            c.cycles += 4;
            call_ptr(c, AI_GET_FRAME_COUNTER, 0x0013_f464)?;
            let long_ago = crate::tick::ticks_from_frames60(c, 500);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(long_ago));
        }
        c.cycles += 5;
        c.sw(this.wrapping_add(0x14), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_InitData.

/// `void Rider_InitData(Rider *rider)` (0x140738): clear the rider's control /
/// trick state words at `+0x1f0..+0x2d0`, set the `-1` sentinel at `+0x430` and
/// fill the 64-word input history at `+0x5b8` with `-1`.
pub fn rider_init_data(c: &mut Cpu) {
    c.cycles += 66;
    let rider = abi::arg(c, 0);
    // The original writes the 57 state words in a shuffled order; memory is
    // the same either way.
    let mut off = 0x1f0u32;
    while off <= 0x2d0 {
        Addr(rider).off(off).sw(c, 0);
        off += 4;
    }
    Addr(rider).off(0x430).sw(c, u64::MAX);

    // 64 words of -1 at +0x5b8. The loop's back edge can deliver an event, so
    // the address/counter live in their original registers.
    c.g[V0][0] = u64::MAX;
    c.g[A1][0] = u64::MAX;
    c.g[A2][0] = 0x3f;
    c.g[A3][0] = u64::MAX;
    c.g[V1][0] = sx(rider.wrapping_add(0x5b8));
    loop {
        c.cycles += 7;
        Addr(r32(c, V1)).sw(c, c.g[A1][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        let more = c.g[A2][0] != c.g[A3][0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_0840);
        }
    }
}

// Rider / ground-state layout used below (byte offsets):
//
//   GroundState: +0 steering angle, +4/+8 ground speeds, +0xc grounded flag,
//                +0x18 Rider*.
//   Rider:       +0x110/+0x120 position/quaternion, +0x1e0 direction
//                quaternion, +0x1f0 pitch, +0x208 roll, +0x214 dead zone,
//                +0x220 grip, +0x2c0..+0x2dc lean/speed outputs,
//                +0x2e4 max tilt, +0x300 steering, +0x308 airborne flag,
//                +0x31c lean output, +0x370 contact normal,
//                +0x378 vertical speed, +0x390/+0x3a0/+0x3b0/+0x3d0 contact
//                frame, +0x3f8 drag output, +0x438 ground index,
//                +0x454 ground contact parameter, +0x758 friction output,
//                +0x77c state object, +0x780 model, +0x784 animation,
//                +0x860 overlap list, +0x864 collision group, +0x86c/+0xb34
//                character record ids, +0x874 wind enabled.
//
// The frame at sp holds the accumulated force and collision output.

/// `void RiderGroundState_Update(GroundState* gs)` (0x13d818).
pub fn rider_ground_state_update(c: &mut Cpu) {
    // --------------------------------------------------------------------------
    // Prologue: save the callee-saved state, apply wind, and clamp the
    // direction quaternion (+0x1e0) to the maximum tilt (+0x2e4) about up.
    // --------------------------------------------------------------------------
    'b_13d8cc: {
        c.cycles += 38;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x210));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x150), c.g[RA][0]);
        c.swc1(31, (c.g[SP][0] as u32).wrapping_add(0x208));
        c.swc1(30, (c.g[SP][0] as u32).wrapping_add(0x200));
        c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x1f8));
        c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x1e8));
        c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x1d8));
        c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        if call_ptr(c, 0x00125970, 0x0013d870).is_err() {
            return;
        }
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x1e0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 23);
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x2e4));
        c.c_lt_s(0, 23);
        if !c.fcc() {
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13d8cc;
        } else {
            c.cycles += 7;
            c.div_s(0, 0, 23);
            c.mfc1(2, 0);
            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x1e0));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x1e0));
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13d8cc;
        }
    }
    // --------------------------------------------------------------------------
    // Integrate the forward ground speed (gs+0x4) towards the track's
    // reference speed (ground entry +0x14), limited by gs+0x8 * 100.
    // --------------------------------------------------------------------------
    'b_13d944: {
        'b_13d930: {
            c.cycles += 23;
            c.g[A2][0] = 0xb0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x438));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.mult(0, 3, 3, 6);
            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x300));
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7064));
            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x780));
            c.mul_s(25, 1, 0);
            c.g[S2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[AT][0] = 0x42c80000;
            c.mtc1(1, 1);
            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
            c.lwc1(28, (c.g[V0][0] as u32).wrapping_add(0x140));
            c.mul_s(3, 25, 1);
            c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x4));
            c.mul_s(2, 28, 0);
            c.add_s(0, 2, 3);
            c.c_lt_s(0, 4);
            if !c.fcc() {
                c.sub_s(0, 2, 3);
                break 'b_13d930;
            } else {
                c.cycles += 2;
                let t = true;
                c.sub_s(1, 4, 3);
                if t {
                    break 'b_13d944;
                } else {
                    break 'b_13d930;
                }
            }
        }
        c.cycles += 4;
        c.c_lt_s(4, 0);
        let t = !c.fcc();
        c.mov_s(1, 2);
        if t {
            break 'b_13d944;
        } else {
            c.cycles += 1;
            c.add_s(1, 4, 3);
            break 'b_13d944;
        }
    }
    // --------------------------------------------------------------------------
    // Integrate the lateral ground speed (gs+0x8) the same way
    // (ground entry +0x18).
    // --------------------------------------------------------------------------
    'b_13d98c: {
        'b_13d97c: {
            c.cycles += 12;
            c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[AT][0] = 0x42c80000;
            c.mtc1(1, 0);
            c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x18));
            c.mul_s(3, 25, 0);
            c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x8));
            c.mul_s(1, 28, 1);
            c.add_s(0, 1, 3);
            c.c_lt_s(0, 2);
            if !c.fcc() {
                c.sub_s(0, 1, 3);
                break 'b_13d97c;
            } else {
                c.cycles += 2;
                let t = true;
                c.sub_s(1, 2, 3);
                if t {
                    break 'b_13d98c;
                } else {
                    break 'b_13d97c;
                }
            }
        }
        c.cycles += 4;
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.add_s(1, 2, 3);
            break 'b_13d98c;
        } else {
            break 'b_13d98c;
        }
    }
    // --------------------------------------------------------------------------
    // Ground frame: the rider's normal and the three ground axes projected
    // through the contact quaternion (+0x370/+0x3a0/+0x3b0), then the friction,
    // boost, drag and carve accelerations.
    // --------------------------------------------------------------------------
    'b_13dab8: {
        c.cycles += 70;
        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.mov_s(14, 25);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.g[S1][0];
        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x1e0));
        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x3d0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[S2][0];
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(3, (c.g[SP][0] as u32));
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x370));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(6, 4);
        c.mtc1(6, 31);
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x3a0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 20);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1f0));
        c.mov_s(24, 31);
        c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(0x454));
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x3b0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 21);
        c.mov_s(13, 24);
        c.neg_s(27, 0);
        if callee::RIDER_GROUND_STATE_COMPUTE_FRICTION_ACCEL
            .call(c, 0x0013da40)
            .is_err()
        {
            return;
        }
        c.mov_s(22, 0);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        if callee::RIDER_GROUND_STATE_COMPUTE_BOOST_ACCEL
            .call(c, 0x0013da50)
            .is_err()
        {
            return;
        }
        c.lwc1(13, (c.g[S2][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.mov_s(12, 20);
        c.div_s(13, 22, 13);
        c.mov_s(30, 0);
        if callee::RIDER_GROUND_STATE_COMPUTE_DRAG
            .call(c, 0x0013da6c)
            .is_err()
        {
            return;
        }
        c.mov_s(26, 0);
        c.mov_s(12, 21);
        c.mov_s(13, 20);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.mov_s(14, 27);
        if callee::RIDER_GROUND_STATE_COMPUTE_CARVE_ACCEL
            .call(c, 0x0013da88)
            .is_err()
        {
            return;
        }
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.mov_s(29, 0);
        c.mtc1(0, 1);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x378));
        c.c_lt_s(0, 1);
        let t = !c.fcc();

        if t {
            break 'b_13dab8;
        } else {
            c.cycles += 4;
            c.c_lt_s(22, 1);
            if c.fcc() {
                c.mov_s(22, 1);
                break 'b_13dab8;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13dab8;
            }
        }
    }
    // --------------------------------------------------------------------------
    // Lean-curve lookup 1: interpolate the global curve (gp-0x1fa8) at
    // |velocity| * gp-0x7060.
    // --------------------------------------------------------------------------
    'b_13db60: {
        'b_13db5c: {
            'b_13db4c: {
                'b_13db48: {
                    'b_13db28: {
                        c.cycles += 9;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7060));
                        c.swc1(26, (c.g[V0][0] as u32).wrapping_add(0x3f8));
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fa8));
                        c.mul_s(3, 23, 0);
                        c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                        c.c_lt_s(4, 3);
                        if !c.fcc() {
                            c.lwc1(1, (c.g[V0][0] as u32));
                            break 'b_13db28;
                        } else {
                            'b_13db18: {
                                c.cycles += 5;
                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                c.c_lt_s(1, 3);
                                if !c.fcc() {
                                    c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                    break 'b_13db18;
                                } else {
                                    'b_13db0c: {
                                        c.cycles += 5;
                                        c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x18));
                                        c.c_lt_s(4, 3);
                                        if !c.fcc() {
                                            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                            break 'b_13db0c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lwc1(21, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                            if t {
                                                break 'b_13db60;
                                            } else {
                                                break 'b_13db0c;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.sub_s(3, 3, 1);
                                    let t = true;
                                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_13db48;
                                    } else {
                                        break 'b_13db18;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.sub_s(3, 3, 4);
                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                            let t = true;
                            c.sub_s(1, 1, 4);
                            if t {
                                break 'b_13db4c;
                            } else {
                                break 'b_13db28;
                            }
                        }
                    }
                    'b_13db40: {
                        c.cycles += 4;
                        c.c_lt_s(3, 1);
                        if !c.fcc() {
                            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                            break 'b_13db40;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.lwc1(21, (c.g[V0][0] as u32).wrapping_add(0x4));
                            if t {
                                break 'b_13db5c;
                            } else {
                                break 'b_13db40;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.sub_s(3, 3, 1);
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                    break 'b_13db48;
                }
                c.cycles += 1;
                c.sub_s(1, 4, 1);
                break 'b_13db4c;
            }
            c.cycles += 4;
            c.sub_s(0, 0, 2);
            c.mul_s(0, 0, 3);
            c.div_s(0, 0, 1);
            c.add_s(21, 2, 0);
            break 'b_13db5c;
        }
        c.cycles += 1;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x705c));
        break 'b_13db60;
    }
    // --------------------------------------------------------------------------
    // Lean-curve lookup 2: interpolate the global curve (gp-0x1fa0) at
    // |velocity| * gp-0x705c.
    // --------------------------------------------------------------------------
    'b_13dbfc: {
        'b_13dbec: {
            'b_13dbe8: {
                'b_13dbc8: {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fa0));
                    c.mul_s(3, 23, 0);
                    c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.c_lt_s(4, 3);
                    if !c.fcc() {
                        c.lwc1(1, (c.g[V0][0] as u32));
                        break 'b_13dbc8;
                    } else {
                        'b_13dbb8: {
                            c.cycles += 5;
                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                            c.c_lt_s(1, 3);
                            if !c.fcc() {
                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                break 'b_13dbb8;
                            } else {
                                'b_13dbac: {
                                    c.cycles += 5;
                                    c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x18));
                                    c.c_lt_s(4, 3);
                                    if !c.fcc() {
                                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                        break 'b_13dbac;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lwc1(20, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                        if t {
                                            break 'b_13dbfc;
                                        } else {
                                            break 'b_13dbac;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.sub_s(3, 3, 1);
                                let t = true;
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                if t {
                                    break 'b_13dbe8;
                                } else {
                                    break 'b_13dbb8;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.sub_s(3, 3, 4);
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                        let t = true;
                        c.sub_s(1, 1, 4);
                        if t {
                            break 'b_13dbec;
                        } else {
                            break 'b_13dbc8;
                        }
                    }
                }
                'b_13dbe0: {
                    c.cycles += 4;
                    c.c_lt_s(3, 1);
                    if !c.fcc() {
                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                        break 'b_13dbe0;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.lwc1(20, (c.g[V0][0] as u32).wrapping_add(0x4));
                        if t {
                            break 'b_13dbfc;
                        } else {
                            break 'b_13dbe0;
                        }
                    }
                }
                c.cycles += 2;
                c.sub_s(3, 3, 1);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                break 'b_13dbe8;
            }
            c.cycles += 1;
            c.sub_s(1, 4, 1);
            break 'b_13dbec;
        }
        c.cycles += 4;
        c.sub_s(0, 0, 2);
        c.mul_s(0, 0, 3);
        c.div_s(0, 0, 1);
        c.add_s(20, 2, 0);
        break 'b_13dbfc;
    }
    // --------------------------------------------------------------------------
    // Character attribute factor (BE library), the lean sine/cosine through
    // Math_SinCos, and the accumulated force vector kept in the frame.
    // --------------------------------------------------------------------------
    'b_13de08: {
        c.cycles += 117;
        if callee::BELIBRARY_GET_INSTANCE.call(c, 0x0013dc04).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x3;
        if callee::BELIBRARY_GET_INTERFACE.call(c, 0x0013dc10).is_err() {
            return;
        }
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x86c));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xb34));
        if callee::BECHARINT_GET_ATTRIB3_FACTOR_2
            .call(c, 0x0013dc24)
            .is_err()
        {
            return;
        }
        c.sub_s(1, 20, 21);
        c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0xc));
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7058));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x144));
        c.mul_s(0, 0, 1);
        c.add_s(0, 0, 1);
        c.add_s(0, 21, 0);
        c.mul_s(12, 12, 0);
        c.mul_s(12, 12, 2);
        c.mul_s(12, 12, 27);
        if call_ptr(c, 0x0031be50, 0x0013dc54).is_err() {
            return;
        }
        c.lwc1(1, (c.g[S2][0] as u32));
        c.add_s(3, 26, 30);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x144));
        c.add_s(1, 1, 1);
        c.mfc1(4, 0);
        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x370));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.min_s(1, 22, 1);
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.div_s(1, 1, 0);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.mfc1(4, 2);
        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x3b0));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sub_s(22, 22, 1);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(0, (c.g[S2][0] as u32));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
        c.neg_s(0, 0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.mfc1(4, 22);
        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.mfc1(3, 1);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.mfc1(4, 3);
        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x3a0));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.mfc1(4, 29);
        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x3b0));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x308));
        if c.g[V1][0] != 0u64 {
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13de08;
        } else {
            c.cycles += 14;
            c.g[AT][0] = 0xffffffffc47a0000;
            c.mtc1(1, 0);
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x370));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13de08;
        }
    }
    // --------------------------------------------------------------------------
    // Vertical speed / penetration correction: move the rider out of the
    // ground along the contact normal and damp the vertical speed.
    // --------------------------------------------------------------------------
    'b_13df34: {
        'b_13df30: {
            c.cycles += 10;
            c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x454));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7054));
            c.add_s(1, 0, 1);
            c.mtc1(0, 3);
            c.max_s(1, 1, 2);
            c.c_lt_s(1, 3);
            if !c.fcc() {
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13df30;
            } else {
                'b_13dec8: {
                    c.cycles += 18;
                    c.mfc1(3, 1);
                    c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x370));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.c_lt_s(31, 3);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x110));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x110));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x454));
                    c.sub_s(0, 0, 1);
                    let t = !c.fcc();
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x454));
                    if t {
                        break 'b_13dec8;
                    } else {
                        c.cycles += 20;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.mfc1(4, 24);
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.qmtc2(4, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lqc2(3, (c.g[SP][0] as u32));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.lqc2(3, (c.g[SP][0] as u32));
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x3d0));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x1e0));
                        break 'b_13dec8;
                    }
                }
                c.cycles += 14;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x370));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.c_lt_s(0, 3);
                if !c.fcc() {
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x308));
                    break 'b_13df34;
                } else {
                    c.cycles += 12;
                    c.mfc1(2, 0);
                    c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break 'b_13df30;
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x308));
        break 'b_13df34;
    }
    // --------------------------------------------------------------------------
    // Landing and bounce handling for the corrected vertical speed.
    // --------------------------------------------------------------------------
    'b_13df90: {
        c.cycles += 2;
        if c.g[V0][0] != 0u64 {
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13df90;
        } else {
            c.cycles += 6;
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x454));
            c.mtc1(0, 1);
            c.c_le_s(1, 0);
            if !c.fcc() {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13df90;
            } else {
                c.cycles += 15;
                c.mfc1(2, 0);
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x110));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x110));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x454));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13df90;
            }
        }
    }
    'b_13e080: {
        c.cycles += 25;
        c.lqc2(3, (c.g[SP][0] as u32));
        c.lqc2(5, (c.g[SP][0] as u32));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 2);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 1);
        c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x110));
        c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x390));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 3);
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x214));
        c.abs_s(0, 0);
        c.c_lt_s(1, 0);
        let t = !c.fcc();

        if t {
            break 'b_13e080;
        } else {
            c.cycles += 4;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7050));
            c.c_lt_s(2, 0);
            let t = !c.fcc();

            if t {
                break 'b_13e080;
            } else {
                c.cycles += 7;
                c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x370));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x704c));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.c_lt_s(1, 0);
                let t = !c.fcc();

                if t {
                    break 'b_13e080;
                } else {
                    'b_13e03c: {
                        c.cycles += 6;
                        c.g[AT][0] = 0x40000000;
                        c.mtc1(1, 0);
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7048));
                        c.c_lt_s(2, 0);
                        let t = !c.fcc();

                        if t {
                            break 'b_13e03c;
                        } else {
                            c.cycles += 1;
                            c.mtc1(0, 1);
                            break 'b_13e03c;
                        }
                    }
                    c.cycles += 17;
                    c.mfc1(2, 1);
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32));
                    c.g[A0][0] = 0x500000;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
                    c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x3d0));
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(3, (c.g[A1][0] as u32).wrapping_add(0x1e0));
                    c.lq(2, (c.g[A0][0] as u32));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    break 'b_13e080;
                }
            }
        }
    }
    // --------------------------------------------------------------------------
    // Animation checks: layer entry 10/4/5 or anim id 0x16 zero the lean.
    // --------------------------------------------------------------------------
    'b_13e1a4: {
        c.cycles += 72;
        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[AT][0] = 0x41a00000;
        c.mtc1(1, 4);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.mul_s(4, 25, 4);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.mfc1(4, 25);
        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x1e0));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 5);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x110));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x110));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 1);
        c.mfc1(2, 25);
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[AT][0] = 0xffffffffc1a00000;
        c.mtc1(1, 6);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sub_s(4, 1, 4);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x1e0));
        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x1e0));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x1e0));
        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x3d0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.sq(3, (c.g[SP][0] as u32));
        c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x110));
        c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x390));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.sub_s(0, 3, 0);
        c.lwc1(2, (c.g[A0][0] as u32).wrapping_add(0x220));
        c.lwc1(3, (c.g[A0][0] as u32).wrapping_add(0x2c8));
        c.sub_s(1, 1, 2);
        c.mul_s(0, 0, 5);
        c.mul_s(7, 2, 5);
        c.mul_s(1, 1, 6);
        c.add_s(20, 3, 0);
        c.mul_s(20, 20, 4);
        c.c_le_s(1, 20);
        let t = !c.fcc();
        c.mov_s(0, 1);
        if t {
            break 'b_13e1a4;
        } else {
            c.cycles += 1;
            c.min_s(0, 20, 7);
            break 'b_13e1a4;
        }
    }
    'b_13e20c: {
        'b_13e208: {
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x784));
            c.mov_s(20, 0);
            c.g[A1][0] = 0x2;
            if call_ptr(c, 0x00311ae8, 0x0013e1b4).is_err() {
                return;
            }
            c.g[V1][0] = 0xa;
            let t = c.g[V0][0] == c.g[V1][0];
            c.g[A1][0] = 0x2;
            if t {
                break 'b_13e208;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
                if call_ptr(c, 0x00311ae8, 0x0013e1cc).is_err() {
                    return;
                }
                c.g[V1][0] = 0x4;
                let t = c.g[V0][0] == c.g[V1][0];
                c.g[A1][0] = 0x2;
                if t {
                    break 'b_13e208;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
                    if call_ptr(c, 0x00311ae8, 0x0013e1e4).is_err() {
                        return;
                    }
                    c.g[V1][0] = 0x5;
                    let t = c.g[V0][0] == c.g[V1][0];
                    c.g[A1][0] = 0x2;
                    if t {
                        break 'b_13e208;
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
                        if call_ptr(c, 0x00312aa0, 0x0013e1fc).is_err() {
                            return;
                        }
                        c.g[V1][0] = 0x16;
                        let t = c.g[V0][0] != c.g[V1][0];

                        if t {
                            break 'b_13e20c;
                        } else {
                            break 'b_13e208;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.mtc1(0, 20);
        break 'b_13e20c;
    }
    // --------------------------------------------------------------------------
    // Sub-state / speed gate for the bank-angle (Math_Asin) block.
    // --------------------------------------------------------------------------
    'b_13e714: {
        'b_13e298: {
            c.cycles += 24;
            c.g[AT][0] = 0x42c80000;
            c.mtc1(1, 0);
            c.mul_s(1, 20, 28);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.mul_s(0, 25, 0);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2d0));
            c.mtc1(0, 23);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2cc));
            c.lqc2(3, (c.g[SP][0] as u32));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 24);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            if call_ptr(c, 0x0011fee8, 0x0013e25c).is_err() {
                return;
            }
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 0x2;
            let t = c.g[S0][0] != c.g[V0][0];

            if t {
                break 'b_13e298;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                if call_ptr(c, 0x0011fee8, 0x0013e274).is_err() {
                    return;
                }
                if c.g[V0][0] != c.g[S0][0] {
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break 'b_13e714;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7044));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x378));
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        break 'b_13e714;
                    } else {
                        break 'b_13e298;
                    }
                }
            }
        }
        c.cycles += 5;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7040));
        c.c_lt_s(0, 24);
        if !c.fcc() {
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13e714;
        } else {
            'b_13e340: {
                c.cycles += 37;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.lqc2(4, (c.g[SP][0] as u32));
                c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x3a0));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 0);
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.mov_s(12, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.mfc1(3, 24);
                c.qmtc2(3, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x370));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(4, 4);
                c.mtc1(4, 1);
                c.c_le_s(0, 1);
                if c.fcc() {
                    c.min_s(12, 1, 2);
                    break 'b_13e340;
                } else {
                    break 'b_13e340;
                }
            }
            // --------------------------------------------------------------------------
            // Bank angle from the lateral/forward ratio (Math_Asin) and its two
            // curve lookups.
            // --------------------------------------------------------------------------
            'b_13e3f0: {
                'b_13e3e0: {
                    'b_13e3dc: {
                        'b_13e3b8: {
                            c.cycles += 11;
                            if call_ptr(c, 0x0031c128, 0x0013e348).is_err() {
                                return;
                            }
                            c.neg_s(8, 0);
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fc0));
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x703c));
                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.mul_s(3, 24, 0);
                            c.c_lt_s(4, 3);
                            let t = !c.fcc();
                            c.lwc1(9, (c.g[S2][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_13e3b8;
                            } else {
                                'b_13e3a8: {
                                    c.cycles += 5;
                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(1, 3);
                                    if !c.fcc() {
                                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                        break 'b_13e3a8;
                                    } else {
                                        'b_13e39c: {
                                            c.cycles += 5;
                                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x18));
                                            c.c_lt_s(4, 3);
                                            if !c.fcc() {
                                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                                break 'b_13e39c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lwc1(7, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                                if t {
                                                    break 'b_13e3f0;
                                                } else {
                                                    break 'b_13e39c;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.sub_s(3, 3, 1);
                                        let t = true;
                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                        if t {
                                            break 'b_13e3dc;
                                        } else {
                                            break 'b_13e3a8;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.sub_s(3, 3, 4);
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                                let t = true;
                                c.sub_s(1, 1, 4);
                                if t {
                                    break 'b_13e3e0;
                                } else {
                                    break 'b_13e3b8;
                                }
                            }
                        }
                        'b_13e3d4: {
                            c.cycles += 5;
                            c.lwc1(1, (c.g[V0][0] as u32));
                            c.c_lt_s(3, 1);
                            if !c.fcc() {
                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                                break 'b_13e3d4;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lwc1(7, (c.g[V0][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_13e3f0;
                                } else {
                                    break 'b_13e3d4;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.sub_s(3, 3, 1);
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                        break 'b_13e3dc;
                    }
                    c.cycles += 1;
                    c.sub_s(1, 4, 1);
                    break 'b_13e3e0;
                }
                c.cycles += 4;
                c.sub_s(0, 0, 2);
                c.mul_s(0, 0, 3);
                c.div_s(0, 0, 1);
                c.add_s(7, 2, 0);
                break 'b_13e3f0;
            }
            'b_13e4d8: {
                'b_13e4c8: {
                    'b_13e4c4: {
                        'b_13e4a4: {
                            c.cycles += 26;
                            c.mul_s(0, 27, 27);
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 5);
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x7038));
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            c.sub_s(0, 5, 0);
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x7034));
                            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x220));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7030));
                            c.mul_s(0, 0, 4);
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x702c));
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x7028));
                            c.mul_s(1, 1, 2);
                            c.mul_s(3, 7, 3);
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fb8));
                            c.mul_s(0, 9, 0);
                            c.mul_s(2, 24, 4);
                            c.add_s(1, 1, 5);
                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.add_s(0, 0, 6);
                            c.c_lt_s(4, 2);
                            c.mul_s(0, 0, 27);
                            c.mul_s(5, 0, 3);
                            c.div_s(5, 5, 1);
                            if !c.fcc() {
                                c.lwc1(1, (c.g[V0][0] as u32));
                                break 'b_13e4a4;
                            } else {
                                'b_13e494: {
                                    c.cycles += 5;
                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.c_lt_s(1, 2);
                                    if !c.fcc() {
                                        c.sub_s(3, 2, 4);
                                        break 'b_13e494;
                                    } else {
                                        'b_13e488: {
                                            c.cycles += 5;
                                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x18));
                                            c.c_lt_s(4, 2);
                                            if !c.fcc() {
                                                c.sub_s(3, 2, 1);
                                                break 'b_13e488;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                                if t {
                                                    break 'b_13e4d8;
                                                } else {
                                                    break 'b_13e488;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                        let t = true;
                                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                        if t {
                                            break 'b_13e4c4;
                                        } else {
                                            break 'b_13e494;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                let t = true;
                                c.sub_s(1, 1, 4);
                                if t {
                                    break 'b_13e4c8;
                                } else {
                                    break 'b_13e4a4;
                                }
                            }
                        }
                        'b_13e4bc: {
                            c.cycles += 4;
                            c.c_lt_s(2, 1);
                            if !c.fcc() {
                                c.sub_s(3, 2, 1);
                                break 'b_13e4bc;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_13e4d8;
                                } else {
                                    break 'b_13e4bc;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                        break 'b_13e4c4;
                    }
                    c.cycles += 1;
                    c.sub_s(1, 4, 1);
                    break 'b_13e4c8;
                }
                c.cycles += 4;
                c.sub_s(0, 0, 2);
                c.mul_s(0, 0, 3);
                c.div_s(0, 0, 1);
                c.add_s(1, 2, 0);
                break 'b_13e4d8;
            }
            'b_13e51c: {
                c.cycles += 16;
                c.mul_s(5, 5, 1);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.mtc1(0, 0);
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x3a0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 1);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.add_s(8, 8, 5);
                if t {
                    break 'b_13e51c;
                } else {
                    c.cycles += 1;
                    c.neg_s(8, 8);
                    break 'b_13e51c;
                }
            }
            'b_13e5e4: {
                'b_13e5dc: {
                    c.cycles += 32;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7024));
                    c.g[V0][0] = 0x500000;
                    c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x34));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
                    c.mul_s(0, 24, 0);
                    c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                    c.lqc2(5, (c.g[V0][0] as u32));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7020));
                    c.mul_s(1, 1, 0);
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
                    c.mtc1(3, 3);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.c_lt_s(2, 3);
                    c.mul_s(1, 1, 24);
                    c.mul_s(7, 1, 25);
                    // The yaw alignment's speed term, over the stock step (see
                    // `yaw_align_gain`).
                    let hz = crate::tick::rate(c);
                    if hz != 60 {
                        c.f[7] = (f32::from_bits(c.f[7]) * (hz as f32 / 60.0)).to_bits();
                    }
                    let t = !c.fcc();
                    c.min_s(7, 7, 0);
                    if t {
                        break 'b_13e5dc;
                    } else {
                        c.cycles += 16;
                        c.mfc1(2, 3);
                        c.qmtc2(2, 3);
                        c.vu0().div(0, 3, 3, 0);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.vu0().waitq();
                        c.vu0().mul(0b1111, 5, 4, Src::Q);
                        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                        c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x90));
                        let t = true;
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        if t {
                            break 'b_13e5e4;
                        } else {
                            break 'b_13e5dc;
                        }
                    }
                }
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x1b0));
                break 'b_13e5e4;
            }
            'b_13e6cc: {
                'b_13e69c: {
                    'b_13e684: {
                        c.cycles += 32;
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lqc2(3, (c.g[SP][0] as u32));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(2, 4);
                        c.mtc1(2, 3);
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x701c));
                        c.abs_s(3, 3);
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 5);
                        c.div_s(3, 3, 24);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 4);
                        c.div_s(0, 24, 0);
                        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x28));
                        c.mtc1(0, 2);
                        c.add_s(1, 1, 1);
                        c.lwc1(6, (c.g[S2][0] as u32).wrapping_add(0x2c));
                        c.c_le_s(8, 2);
                        c.sub_s(5, 5, 3);
                        c.min_s(0, 0, 4);
                        c.mul_s(0, 0, 5);
                        c.mul_s(0, 0, 1);
                        c.max_s(0, 0, 2);
                        c.add_s(6, 6, 0);
                        let t = !c.fcc();
                        c.min_s(6, 6, 4);
                        if t {
                            break 'b_13e684;
                        } else {
                            c.cycles += 6;
                            c.neg_s(0, 27);
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7018));
                            c.mul_s(0, 0, 1);
                            c.c_le_s(6, 0);
                            let t = c.fcc();

                            if t {
                                break 'b_13e69c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.mul_s(7, 7, 6);
                                if t {
                                    break 'b_13e6cc;
                                } else {
                                    break 'b_13e684;
                                }
                            }
                        }
                    }
                    c.cycles += 6;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7014));
                    c.mul_s(0, 27, 0);
                    c.c_le_s(6, 0);
                    if !c.fcc() {
                        c.mul_s(7, 7, 6);
                        break 'b_13e6cc;
                    } else {
                        break 'b_13e69c;
                    }
                }
                'b_13e6c0: {
                    c.cycles += 4;
                    c.mtc1(0, 0);
                    c.c_le_s(8, 0);
                    let t = !c.fcc();

                    if t {
                        break 'b_13e6c0;
                    } else {
                        c.cycles += 5;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7010));
                        c.neg_s(1, 27);
                        c.mul_s(0, 7, 0);
                        let t = true;
                        c.mul_s(7, 0, 1);
                        if t {
                            break 'b_13e6cc;
                        } else {
                            break 'b_13e6c0;
                        }
                    }
                }
                c.cycles += 3;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x700c));
                c.mul_s(0, 7, 0);
                c.mul_s(7, 0, 27);
                break 'b_13e6cc;
            }
            // 0x13e6cc: f7 is the share of the board's heading error (f8)
            // turned away this tick, a stock 60 Hz amount.
            let hz = crate::tick::rate(c);
            if hz != 60 {
                c.f[7] = yaw_align_gain(hz, f32::from_bits(c.f[7])).to_bits();
            }
            'b_13e6e8: {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.mtc1(0, 1);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x2dc));
                c.c_eq_s(0, 1);
                if !c.fcc() {
                    c.mov_s(7, 1);
                    break 'b_13e6e8;
                } else {
                    break 'b_13e6e8;
                }
            }
            'b_13e70c: {
                c.cycles += 9;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7008));
                c.mul_s(1, 8, 7);
                c.mul_s(2, 25, 0);
                c.neg_s(0, 0);
                c.mul_s(0, 25, 0);
                c.c_le_s(0, 1);
                if c.fcc() {
                    c.min_s(0, 1, 2);
                    break 'b_13e70c;
                } else {
                    break 'b_13e70c;
                }
            }
            c.cycles += 2;
            c.mov_s(23, 0);
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            break 'b_13e714;
        }
    }
    // --------------------------------------------------------------------------
    // Carve-angle sign handling and the clamping helpers.
    // --------------------------------------------------------------------------
    'b_13ea68: {
        c.cycles += 5;
        c.mtc1(0, 22);
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x2dc));
        c.c_eq_s(0, 22);
        let t = c.fcc();

        if t {
            break 'b_13ea68;
        } else {
            'b_13e8fc: {
                'b_13e8f8: {
                    'b_13e8f4: {
                        'b_13e8ec: {
                            'b_13e8e0: {
                                'b_13e8b8: {
                                    'b_13e89c: {
                                        'b_13e838: {
                                            c.cycles += 27;
                                            c.lqc2(3, (c.g[SP][0] as u32));
                                            c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x370));
                                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                            c.qmfc2(2, 4);
                                            c.mtc1(2, 0);
                                            c.lwc1(26, (c.g[GP][0] as u32).wrapping_sub(0x7004));
                                            c.mfc1(2, 0);
                                            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x370));
                                            c.qmtc2(2, 3);
                                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            c.lqc2(3, (c.g[SP][0] as u32));
                                            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x320));
                                            let t = c.g[V1][0] == 0u64;

                                            if t {
                                                break 'b_13e838;
                                            } else {
                                                'b_13e808: {
                                                    c.cycles += 23;
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    c.lqc2(
                                                        5,
                                                        (c.g[A0][0] as u32).wrapping_add(0x3a0),
                                                    );
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.qmfc2(2, 4);
                                                    c.mtc1(2, 0);
                                                    c.lqc2(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    c.lqc2(
                                                        5,
                                                        (c.g[A0][0] as u32).wrapping_add(0x3b0),
                                                    );
                                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                                    c.qmfc2(3, 4);
                                                    c.mtc1(3, 1);
                                                    c.neg_s(20, 0);
                                                    c.c_eq_s(20, 22);
                                                    let t = !c.fcc();
                                                    c.neg_s(21, 1);
                                                    if t {
                                                        break 'b_13e808;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.c_eq_s(21, 22);
                                                        let t = c.fcc();
                                                        c.mov_s(12, 21);
                                                        if t {
                                                            break 'b_13e8f8;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;

                                                            if t {
                                                                break 'b_13e89c;
                                                            } else {
                                                                break 'b_13e808;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.div_s(12, 21, 20);
                                                if call_ptr(c, 0x0031c228, 0x0013e814).is_err() {
                                                    return;
                                                }
                                                c.c_lt_s(20, 22);
                                                let t = !c.fcc();
                                                c.mov_s(1, 0);
                                                if t {
                                                    break 'b_13e8f4;
                                                } else {
                                                    c.cycles += 3;
                                                    c.c_lt_s(22, 21);
                                                    let t = c.fcc();

                                                    if t {
                                                        break 'b_13e8e0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;

                                                        if t {
                                                            break 'b_13e8ec;
                                                        } else {
                                                            break 'b_13e838;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 21;
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x3a0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(2, 4);
                                        c.mtc1(2, 20);
                                        c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x3b0));
                                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                        c.qmfc2(3, 4);
                                        c.mtc1(3, 21);
                                        c.c_eq_s(20, 22);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_13e8b8;
                                        } else {
                                            c.cycles += 4;
                                            c.c_eq_s(21, 22);
                                            let t = c.fcc();
                                            c.mov_s(12, 21);
                                            if t {
                                                break 'b_13e8f8;
                                            } else {
                                                break 'b_13e89c;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.c_le_s(22, 21);
                                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x7000));
                                    let t = !c.fcc();
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    if t {
                                        break 'b_13e8fc;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x6ffc));
                                        let t = true;

                                        if t {
                                            break 'b_13e8fc;
                                        } else {
                                            break 'b_13e8b8;
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.div_s(12, 21, 20);
                                if call_ptr(c, 0x0031c228, 0x0013e8c4).is_err() {
                                    return;
                                }
                                c.c_lt_s(20, 22);
                                let t = !c.fcc();
                                c.mov_s(1, 0);
                                if t {
                                    break 'b_13e8f4;
                                } else {
                                    c.cycles += 3;
                                    c.c_lt_s(22, 21);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_13e8ec;
                                    } else {
                                        break 'b_13e8e0;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6ff8));
                            let t = true;
                            c.add_s(1, 1, 0);
                            if t {
                                break 'b_13e8f4;
                            } else {
                                break 'b_13e8ec;
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6ff4));
                        c.sub_s(1, 1, 0);
                        break 'b_13e8f4;
                    }
                    c.cycles += 1;
                    c.mov_s(12, 1);
                    break 'b_13e8f8;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13e8fc;
            }
            'b_13e960: {
                c.cycles += 7;
                c.mtc1(0, 1);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x2dc));
                c.mul_s(0, 12, 0);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    c.neg_s(12, 12);
                    break 'b_13e960;
                } else {
                    'b_13e954: {
                        c.cycles += 12;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6ff0));
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fec));
                        c.sub_s(12, 0, 12);
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 2);
                        c.mul_s(1, 12, 1);
                        c.add_s(1, 1, 2);
                        c.cvt_w_s(2, 1);
                        c.cvt_s_w(2, 2);
                        c.c_lt_s(1, 2);
                        let t = !c.fcc();

                        if t {
                            break 'b_13e954;
                        } else {
                            c.cycles += 3;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.sub_s(2, 2, 0);
                            break 'b_13e954;
                        }
                    }
                    c.cycles += 3;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fe8));
                    c.mul_s(0, 2, 0);
                    c.sub_s(12, 12, 0);
                    break 'b_13e960;
                }
            }
            'b_13e9d4: {
                c.cycles += 6;
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x2dc));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fe4));
                c.abs_s(0, 0);
                c.c_lt_s(1, 0);
                let t = !c.fcc();

                if t {
                    break 'b_13e9d4;
                } else {
                    'b_13e994: {
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_lt_s(12, 0);
                        if !c.fcc() {
                            c.add_s(0, 12, 1);
                            break 'b_13e994;
                        } else {
                            c.cycles += 2;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fe0));
                            c.add_s(0, 12, 0);
                            break 'b_13e994;
                        }
                    }
                    'b_13e9cc: {
                        'b_13e9b4: {
                            c.cycles += 6;
                            c.mul_s(3, 26, 25);
                            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x2dc));
                            c.c_lt_s(3, 2);
                            let t = !c.fcc();
                            c.mov_s(12, 0);
                            if t {
                                break 'b_13e9b4;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sub_s(1, 2, 3);
                                if t {
                                    break 'b_13e9cc;
                                } else {
                                    break 'b_13e9b4;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.neg_s(0, 3);
                        c.mtc1(0, 1);
                        c.c_lt_s(2, 0);
                        if c.fcc() {
                            c.add_s(1, 2, 3);
                            break 'b_13e9cc;
                        } else {
                            break 'b_13e9cc;
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2dc));
                    break 'b_13e9d4;
                }
            }
            'b_13e9f8: {
                c.cycles += 6;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fdc));
                c.abs_s(1, 12);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    c.neg_s(1, 26);
                    break 'b_13e9f8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    let t = true;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x2dc), 0u64);
                    if t {
                        break 'b_13ea68;
                    } else {
                        break 'b_13e9f8;
                    }
                }
            }
            'b_13ea18: {
                c.cycles += 7;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fd8));
                c.div_s(0, 26, 0);
                c.mul_s(0, 12, 0);
                c.c_le_s(1, 0);
                let t = !c.fcc();
                c.mov_s(2, 1);
                if t {
                    break 'b_13ea18;
                } else {
                    c.cycles += 1;
                    c.min_s(2, 0, 26);
                    break 'b_13ea18;
                }
            }
            'b_13ea44: {
                c.cycles += 6;
                c.neg_s(0, 25);
                c.mtc1(0, 1);
                c.mul_s(0, 0, 2);
                c.c_lt_s(0, 1);
                let t = !c.fcc();

                if t {
                    break 'b_13ea44;
                } else {
                    c.cycles += 3;
                    c.c_lt_s(23, 1);
                    let t = !c.fcc();

                    if t {
                        break 'b_13ea44;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.min_s(23, 23, 0);
                        if t {
                            break 'b_13ea68;
                        } else {
                            break 'b_13ea44;
                        }
                    }
                }
            }
            c.cycles += 4;
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.add_s(23, 23, 0);
                break 'b_13ea68;
            } else {
                c.cycles += 4;
                c.c_lt_s(1, 23);
                if !c.fcc() {
                    c.add_s(23, 23, 0);
                    break 'b_13ea68;
                } else {
                    c.cycles += 1;
                    c.max_s(23, 23, 0);
                    break 'b_13ea68;
                }
            }
        }
    }
    'b_13eab0: {
        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_lt_s(0, 24);
        if !c.fcc() {
            c.mov_s(5, 0);
            break 'b_13eab0;
        } else {
            c.cycles += 13;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x1b0));
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lqc2(3, (c.g[SP][0] as u32));
            c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.div_s(5, 0, 24);
            break 'b_13eab0;
        }
    }
    'b_13eb64: {
        'b_13eb60: {
            'b_13eb50: {
                'b_13eb4c: {
                    'b_13eb28: {
                        c.cycles += 11;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fd4));
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f98));
                        c.mul_s(3, 24, 0);
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fd0));
                        c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                        c.sub_s(5, 5, 1);
                        c.mtc1(0, 0);
                        c.c_lt_s(4, 3);
                        let t = !c.fcc();
                        c.max_s(5, 5, 0);
                        if t {
                            break 'b_13eb28;
                        } else {
                            'b_13eb18: {
                                c.cycles += 5;
                                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                                c.c_lt_s(1, 3);
                                if !c.fcc() {
                                    c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                    break 'b_13eb18;
                                } else {
                                    'b_13eb0c: {
                                        c.cycles += 5;
                                        c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x18));
                                        c.c_lt_s(4, 3);
                                        if !c.fcc() {
                                            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                            break 'b_13eb0c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                            if t {
                                                break 'b_13eb60;
                                            } else {
                                                break 'b_13eb0c;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.sub_s(3, 3, 1);
                                    let t = true;
                                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_13eb4c;
                                    } else {
                                        break 'b_13eb18;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.sub_s(3, 3, 4);
                            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                            let t = true;
                            c.sub_s(1, 1, 4);
                            if t {
                                break 'b_13eb50;
                            } else {
                                break 'b_13eb28;
                            }
                        }
                    }
                    'b_13eb44: {
                        c.cycles += 5;
                        c.lwc1(1, (c.g[V0][0] as u32));
                        c.c_lt_s(3, 1);
                        let t = !c.fcc();
                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                        if t {
                            break 'b_13eb44;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            if t {
                                break 'b_13eb64;
                            } else {
                                break 'b_13eb44;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.sub_s(3, 3, 1);
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                    break 'b_13eb4c;
                }
                c.cycles += 1;
                c.sub_s(1, 4, 1);
                break 'b_13eb50;
            }
            c.cycles += 4;
            c.sub_s(0, 0, 2);
            c.mul_s(0, 0, 3);
            c.div_s(0, 0, 1);
            c.add_s(2, 2, 0);
            break 'b_13eb60;
        }
        c.cycles += 1;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        break 'b_13eb64;
    }
    // --------------------------------------------------------------------------
    // Probe the ground with a collision volume at the rider's feet; on a hit
    // resolve the contact and record the surface.
    // --------------------------------------------------------------------------
    'b_13eb98: {
        c.cycles += 9;
        c.mtc1(0, 1);
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x1f0));
        c.mul_s(0, 5, 0);
        c.mul_s(0, 0, 2);
        c.sub_s(23, 23, 0);
        c.c_eq_s(23, 1);
        let t = c.fcc();
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        if t {
            break 'b_13eb98;
        } else {
            c.cycles += 4;
            c.mov_s(12, 23);
            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x370));
            if call_ptr(c, 0x0011dfe0, 0x0013eb94).is_err() {
                return;
            }
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
            break 'b_13eb98;
        }
    }
    'b_13ebf8: {
        'b_13ebf4: {
            'b_13ebc4: {
                c.cycles += 7;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x148));
                if callee::RIDER_GROUND_STATE_PROBE_GROUND
                    .call(c, 0x0013eba8)
                    .is_err()
                {
                    return;
                }
                c.g[S3][0] = c.g[V0][0];
                let t = c.g[S3][0] != 0u64;
                c.g[S4][0] = 0u64;
                if t {
                    break 'b_13ebc4;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x308));
                    let t = true;
                    c.g[S4][0] = (c.g[V1][0] < 0x1u64) as u64;
                    if t {
                        break 'b_13ebf4;
                    } else {
                        break 'b_13ebc4;
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x110));
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13ebf8;
            } else {
                c.cycles += 3;
                c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                let t = c.g[A3][0] == 0u64;
                c.g[A1][0] = c.g[S0][0];
                if t {
                    break 'b_13ebf4;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x158));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x15c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0013ebf4);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.pc = t;
                    call_indirect(c, t);
                    if c.pc != 0x0013ebf4 {
                        return;
                    }
                    break 'b_13ebf4;
                }
            }
        }
        c.cycles += 1;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        break 'b_13ebf8;
    }
    // --------------------------------------------------------------------------
    // Ground normal and velocity correction after the probe.
    // --------------------------------------------------------------------------
    'b_13ed2c: {
        c.cycles += 5;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x438));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        if c.g[V0][0] != 0u64 {
            c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x10));
            break 'b_13ed2c;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0xd;
            if c.g[V1][0] == c.g[V0][0] {
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x10));
                break 'b_13ed2c;
            } else {
                'b_13eca4: {
                    c.cycles += 35;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fcc));
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x370));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 3);
                    c.g[AT][0] = 0xffffffffc2200000;
                    c.mtc1(1, 2);
                    c.mul_s(0, 0, 1);
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x370));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x58));
                    c.c_lt_s(0, 2);
                    if c.fcc() {
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x58));
                        break 'b_13eca4;
                    } else {
                        break 'b_13eca4;
                    }
                }
                'b_13ed0c: {
                    c.cycles += 19;
                    c.mtc1(0, 0);
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32));
                    c.lqc2(3, (c.g[SP][0] as u32));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(2, 22);
                    c.mtc1(2, 1);
                    c.c_eq_s(1, 0);
                    if c.fcc() {
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        break 'b_13ed0c;
                    } else {
                        c.cycles += 7;
                        c.div_s(0, 3, 1);
                        c.mfc1(2, 0);
                        c.lqc2(4, (c.g[SP][0] as u32));
                        c.qmtc2(2, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, (c.g[SP][0] as u32));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        break 'b_13ed0c;
                    }
                }
                c.cycles += 8;
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x3d0));
                c.lqc2(4, (c.g[SP][0] as u32));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x1e0));
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x10));
                break 'b_13ed2c;
            }
        }
    }
    // --------------------------------------------------------------------------
    // Set the grounded flag (gs+0xc) and skip the rest of the frame.
    // --------------------------------------------------------------------------
    'b_13f11c: {
        // --------------------------------------------------------------------------
        // Banking about the contact normal (Math_Atan / Math_Asin) and the
        // ground-normal torque.
        // --------------------------------------------------------------------------
        'b_13ed68: {
            'b_13ed4c: {
                c.cycles += 2;
                let t = c.g[S3][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                if t {
                    break 'b_13ed4c;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = c.g[V0][0];
                    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x454));
                    c.c_lt_s(1, 0);
                    let t = !c.fcc();
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x148));
                    if t {
                        break 'b_13ed68;
                    } else {
                        break 'b_13ed4c;
                    }
                }
            }
            'b_13ed60: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x308));
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_13ed60;
                } else {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;

                    if t {
                        break 'b_13f11c;
                    } else {
                        break 'b_13ed60;
                    }
                }
            }
            c.cycles += 2;
            let t = true;
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            if t {
                break 'b_13f11c;
            } else {
                break 'b_13ed68;
            }
        }
        'b_13eea0: {
            c.cycles += 6;
            c.g[AT][0] = 0x40a00000;
            c.mtc1(1, 0);
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                break 'b_13eea0;
            } else {
                c.cycles += 52;
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x378));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 1);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x70), 0u64);
                c.mul_s(0, 0, 1);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x74), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x7c), 0u64);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fc8));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x78));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x370));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x1c0));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
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
                if !c.fcc() {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break 'b_13eea0;
                } else {
                    c.cycles += 20;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    c.mfc1(2, 0);
                    c.qmtc2(2, 3);
                    c.vu0().div(0, 3, 3, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 4, Src::Q);
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.min_s(12, 0, 12);
                    if call_ptr(c, 0x0031c128, 0x0013ee7c).is_err() {
                        return;
                    }
                    c.neg_s(0, 0);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6fc4));
                    c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x38));
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.mul_s(0, 0, 1);
                    c.mul_s(12, 0, 12);
                    if call_ptr(c, 0x0011dfe0, 0x0013ee9c).is_err() {
                        return;
                    }
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break 'b_13eea0;
                }
            }
        }
        'b_13eef4: {
            c.cycles += 15;
            c.g[A1][0] = 0x2;
            c.g[S0][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
            if call_ptr(c, 0x00311ae8, 0x0013eeb0).is_err() {
                return;
            }
            c.g[S3][0] = c.g[V0][0];
            c.g[A1][0] = 0x2;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
            if call_ptr(c, 0x00312aa0, 0x0013eec4).is_err() {
                return;
            }
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.g[S2][0] = c.g[V0][0];
            if call_ptr(c, 0x0011fee8, 0x0013eed0).is_err() {
                return;
            }
            c.g[A0][0] = 0x1;
            let t = c.g[V0][0] == c.g[A0][0];

            if t {
                break 'b_13eef4;
            } else {
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[S0][0] = c.g[A0][0];
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x438));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
                c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = 0u64;
                }
                break 'b_13eef4;
            }
        }
        'b_13f028: {
            'b_13f018: {
                'b_13ef28: {
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[V1][0] = 0u64;
                    if t {
                        break 'b_13ef28;
                    } else {
                        'b_13ef20: {
                            c.cycles += 3;
                            c.g[V0][0] = 0xa;
                            let t = c.g[S3][0] == c.g[V0][0];
                            c.g[V0][0] = 0x5;
                            if t {
                                break 'b_13ef20;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S3][0] == c.g[V0][0];
                                c.g[V0][0] = 0x16;
                                if t {
                                    break 'b_13ef20;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S2][0] == c.g[V0][0];
                                    c.g[V0][0] = c.g[S2][0] ^ 0x2;
                                    if t {
                                        break 'b_13ef20;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V1][0] = c.g[A0][0];
                                        if c.g[V0][0] == 0 {
                                            c.g[V1][0] = 0u64;
                                        }
                                        break 'b_13ef20;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[S0][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                            break 'b_13f018;
                        } else {
                            break 'b_13ef28;
                        }
                    }
                }
                'b_13eff4: {
                    'b_13eff0: {
                        'b_13efe0: {
                            'b_13efdc: {
                                'b_13efbc: {
                                    c.cycles += 18;
                                    c.lqc2(3, (c.g[SP][0] as u32));
                                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                    c.vu0().sqrt(4, 0);
                                    c.vu0().waitq();
                                    c.cfc2(2, 22);
                                    c.mtc1(2, 1);
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fc0));
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1fc8));
                                    c.mul_s(1, 1, 0);
                                    c.lwc1(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.c_lt_s(5, 1);
                                    if !c.fcc() {
                                        c.lwc1(4, (c.g[V0][0] as u32));
                                        break 'b_13efbc;
                                    } else {
                                        'b_13efac: {
                                            c.cycles += 5;
                                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x10));
                                            c.c_lt_s(4, 1);
                                            if !c.fcc() {
                                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                                                break 'b_13efac;
                                            } else {
                                                'b_13efa0: {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        5,
                                                        (c.g[V0][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.c_lt_s(5, 1);
                                                    if !c.fcc() {
                                                        c.lwc1(
                                                            2,
                                                            (c.g[V0][0] as u32).wrapping_add(0x14),
                                                        );
                                                        break 'b_13efa0;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lwc1(
                                                            2,
                                                            (c.g[V0][0] as u32).wrapping_add(0x1c),
                                                        );
                                                        if t {
                                                            break 'b_13eff0;
                                                        } else {
                                                            break 'b_13efa0;
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.sub_s(3, 1, 4);
                                                let t = true;
                                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                                if t {
                                                    break 'b_13efdc;
                                                } else {
                                                    break 'b_13efac;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.sub_s(3, 1, 5);
                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                                        let t = true;
                                        c.sub_s(1, 4, 5);
                                        if t {
                                            break 'b_13efe0;
                                        } else {
                                            break 'b_13efbc;
                                        }
                                    }
                                }
                                'b_13efd4: {
                                    c.cycles += 4;
                                    c.c_lt_s(1, 4);
                                    let t = !c.fcc();
                                    c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
                                    if t {
                                        break 'b_13efd4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                        if t {
                                            break 'b_13eff4;
                                        } else {
                                            break 'b_13efd4;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.sub_s(3, 1, 4);
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                                break 'b_13efdc;
                            }
                            c.cycles += 1;
                            c.sub_s(1, 5, 4);
                            break 'b_13efe0;
                        }
                        c.cycles += 4;
                        c.sub_s(0, 0, 2);
                        c.mul_s(0, 0, 3);
                        c.div_s(0, 0, 1);
                        c.add_s(2, 2, 0);
                        break 'b_13eff0;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break 'b_13eff4;
                }
                c.cycles += 9;
                c.g[AT][0] = 0x40400000;
                c.mtc1(1, 1);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x208));
                c.mul_s(1, 25, 1);
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x210));
                c.sub_s(0, 0, 2);
                c.abs_s(0, 0);
                let t = true;
                c.mul_s(0, 0, 1);
                if t {
                    break 'b_13f028;
                } else {
                    break 'b_13f018;
                }
            }
            c.cycles += 4;
            c.g[AT][0] = 0x40400000;
            c.mtc1(1, 0);
            c.mul_s(0, 25, 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x210), 0u64);
            break 'b_13f028;
        }
        // --------------------------------------------------------------------------
        // Lean, pitch and spin outputs plus the speed curves.
        // --------------------------------------------------------------------------
        'b_13f064: {
            'b_13f050: {
                c.cycles += 2;
                let t = c.g[V1][0] == 0u64;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20c));
                if t {
                    break 'b_13f050;
                } else {
                    c.cycles += 8;
                    c.g[AT][0] = 0x40400000;
                    c.mtc1(1, 0);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.mul_s(0, 25, 0);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 1);
                    let t = true;
                    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2c4));
                    if t {
                        break 'b_13f064;
                    } else {
                        break 'b_13f050;
                    }
                }
            }
            c.cycles += 5;
            c.g[AT][0] = 0x40400000;
            c.mtc1(1, 0);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.mul_s(0, 25, 0);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x2c4), 0u64);
            break 'b_13f064;
        }
        // --------------------------------------------------------------------------
        // Advance the steering angle (gs+0x0, wrapped at 360 degrees) and
        // rebuild the rider's world matrix.
        // --------------------------------------------------------------------------
        'b_13f0cc: {
            c.cycles += 25;
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2c0));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6fbc));
            c.lqc2(3, (c.g[SP][0] as u32));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 3);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.abs_s(1, 27);
            c.mul_s(3, 3, 2);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 4);
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x214));
            c.abs_s(0, 0);
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6fb8));
            c.add_s(1, 1, 0);
            c.c_lt_s(4, 3);
            let t = !c.fcc();
            c.add_s(0, 1, 2);
            if t {
                break 'b_13f0cc;
            } else {
                c.cycles += 1;
                c.mov_s(3, 4);
                break 'b_13f0cc;
            }
        }
        'b_13f0fc: {
            c.cycles += 10;
            c.mul_s(20, 3, 0);
            c.lwc1(1, (c.g[S1][0] as u32));
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6fb4));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6fb0));
            c.mul_s(0, 20, 0);
            c.add_s(0, 1, 0);
            c.c_le_s(2, 0);
            let t = !c.fcc();
            c.swc1(0, (c.g[S1][0] as u32));
            if t {
                break 'b_13f0fc;
            } else {
                c.cycles += 2;
                c.sub_s(0, 0, 2);
                c.swc1(0, (c.g[S1][0] as u32));
                break 'b_13f0fc;
            }
        }
        c.cycles += 8;
        c.lwc1(12, (c.g[S1][0] as u32));
        if call_ptr(c, 0x0031c040, 0x0013f104).is_err() {
            return;
        }
        c.g[AT][0] = 0x40f00000;
        c.mtc1(1, 1);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.mul_s(0, 0, 1);
        c.mul_s(0, 20, 0);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x31c));
        break 'b_13f11c;
    }
    c.cycles += 22;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
    if call_ptr(c, 0x0011e098, 0x0013f124).is_err() {
        return;
    }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x150));
    c.lwc1(31, (c.g[SP][0] as u32).wrapping_add(0x208));
    c.lwc1(30, (c.g[SP][0] as u32).wrapping_add(0x200));
    c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x1f8));
    c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x1f0));
    c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x1e8));
    c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x1e0));
    c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x1d8));
    c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1d0));
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x210));
    c.pc = t;
    return;
}

pub(crate) mod air_walls {

    /// `RiderAirState`: the rider.
    pub const RIDER: u32 = 0x4;

    /// `Rider`: velocity, the controller (vtable slot `+0x40`: "use the
    /// small hull"), the hull's sphere tree, the overlap list and its flags.
    pub const VEL: u32 = 0x1e0;

    pub const POSITION_ROW: u32 = 0x180;

    pub const CONTROLLER: u32 = 0x6c0;

    pub const SPHERE_TREE: u32 = 0xaa0;

    pub const OVERLAP_LIST: u32 = 0x860;

    pub const OVERLAP_FLAGS: u32 = 0x868;

    pub const GET_SUB_STATE: u32 = 0x0011_fee8;

    pub const HULL_CTOR: u32 = 0x0032_f650;

    pub const HULL_DTOR: u32 = 0x0032_f708;

    pub const FIND_CLOSEST_CONTACT: u32 = 0x0033_42d0;

    pub const TRANSLATE: u32 = 0x0010_6538;

    pub const BUILD_WORLD_MATRIX: u32 = 0x0011_e098;

    pub const HANDLE_IMPACT: u32 = 0x0010_5d98;

    /// Sub state 9: no wall collision.
    pub const SUB_STATE_SKIP: u64 = 9;
}

pub(crate) mod boost_accel {
    use super::*;

    /// The ground state's rider.
    pub const RIDER: u32 = 0x18;

    /// Rider fields: the ground material (>= 17: the slow-snow scale), the
    /// finish time (< 0 while racing), the AI (player) index, the character
    /// slot, the animation controller.
    pub const GROUND_MATERIAL: u32 = 0x434;

    pub const FINISH_TIME: u32 = 0x470;

    pub const AI_INDEX: u32 = 0x86c;

    pub const CHARACTER_SLOT: u32 = 0xb34;

    pub const ANIM_CTRL: u32 = 0x784;

    /// The tuning parameters: base acceleration and boost factor.
    pub const PARAM_ACCEL: u32 = 0x20;

    pub const PARAM_BOOST: u32 = 0x24;

    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;

    pub const BE_CHAR_INT_GET_ATTRIB1_FACTOR: Callee = Callee {
        addr: 0x0014_93d8,
        f: ee::s01::f_001493d8,
    };

    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };

    pub const MATH_ATAN: u32 = 0x0031_c228;
}

/// `vf5 = vf4(at) * x` with `x` moved through a GPR (`mfc1; qmtc2`), stored
/// at `sp+out` and reloaded into v0 (`lq`): the original's vector scale.
pub(crate) fn air_walls_scale(c: &mut Cpu, gpr: usize, fpr: usize, at: u32, out: u32) {
    use ssx3_ee_rt::vu0::{Src, X};
    let base = sp(c);
    c.mfc1(gpr, fpr);
    c.lqc2(4, base.wrapping_add(at));
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, base.wrapping_add(out));
    c.lq(V0, base.wrapping_add(out));
}

/// `vf4 = vf3 * vf5; vf4.w = vf4.x + vf4.y + vf4.z` (the original's dot
/// product macro, `vf6.w = 1`).
pub(crate) fn air_walls_dot(c: &mut Cpu, b: u8) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `int RiderAirState_CollideWalls(RiderAirState* self, vec* normal_out)`
/// (0x13aa48): build the rider's collision hull (the small one when the
/// controller says so), find the closest wall contact ahead of it, push the
/// rider out along the contact normal, and when the rider is moving into
/// the wall bounce its velocity off the normal (scaled by speed), rebuild
/// its world matrix and report the impact. Returns 1 (and the normal) on
/// an impact, else 0; sub state 9 skips everything.
///
/// Parked (not installed): its differential test could not be bounded (the
/// real callees under `OverlapList_FindClosestContact` / `Rider_HandleImpact`
/// run away on fixture data, pc 0x312138 in `AnimController_PlayAnimImpl`).
/// The port is a straight transliteration of the generated body.
#[allow(dead_code)]
pub fn rider_air_state_collide_walls(c: &mut Cpu) {
    use self::air_walls::*;
    use crate::ps2f::FpuF32;
    use ssx3_ee_rt::vu0::Src;
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x490);
        frame.save(c, S1, 0x460);
        frame.save(c, S4, 0x430);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x470);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S2, 0x450);
        frame.save(c, S3, 0x440);
        frame.save_ra(c, 0x420);
        frame.save_fpr(c, 21, 0x488);
        frame.save_fpr(c, 20, 0x480);
        let this = Addr(r32(c, S1));
        c.g[A0][0] = this.off(RIDER).lw(c);
        call_ptr(c, GET_SUB_STATE, 0x0013_aa7c)?;
        c.g[V1][0] = SUB_STATE_SKIP;
        let mut impact = false;
        if c.g[V0][0] == SUB_STATE_SKIP {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            let rider = this.off(RIDER).lw(c);
            c.g[S0][0] = rider;
            let rider = Addr(rider as u32);
            c.cycles += 29;
            let base = sp(c);
            let hull = base.wrapping_add(0x80);
            c.g[S2][0] = sx(hull);
            c.f[21] = 0;
            c.g[S3][0] = sx(hull);
            // controller->useSmallHull()
            let vtable = rider.off(CONTROLLER).lw(c);
            c.g[V1][0] = vtable;
            c.g[A1][0] = sx(rider.off(CONTROLLER).0);
            let slot = crate::call::VSlot::read(c, vtable, 0x40);
            c.g[V0][0] = slot.func;
            c.g[A0][0] = slot.this(c.g[A1][0]);
            call_ptr(c, slot.func as u32, 0x0013_aab4)?;
            // CollHull hull(rider->sphereTree, !small)
            c.g[A1][0] = rider.off(SPHERE_TREE).lw(c);
            c.g[A0][0] = sx(hull);
            c.g[A2][0] = c.g[V0][0] ^ 1;
            call_ptr(c, HULL_CTOR, 0x0013_aac4)?;
            // OverlapList_FindClosestContact(list, &hull, &contact, flags)
            let rider_now = this.off(RIDER).lw(c);
            c.g[V1][0] = rider_now;
            let rider_now = Addr(rider_now as u32);
            c.g[A0][0] = 1;
            c.g[A1][0] = sx(hull);
            c.g[A2][0] = sx(base);
            c.lq(V0, rider_now.off(POSITION_ROW).0);
            Addr(base.wrapping_add(0x88)).sw(c, 1);
            c.sq(V0, base.wrapping_add(0x90));
            c.sq(V0, base.wrapping_add(0x3a0));
            c.g[A3][0] = rider_now.off(OVERLAP_FLAGS).lw(c);
            c.g[A0][0] = rider_now.off(OVERLAP_LIST).lw(c);
            call_ptr(c, FIND_CLOSEST_CONTACT, 0x0013_aaf0)?;
            c.f[1] = c.f[0];
            let depth = FpuF32(c.f[1]);
            let missed = depth.lt(FpuF32(c.f[21]));
            abi::set_fcc(c, missed);
            c.g[A0][0] = c.g[S3][0];
            if !missed {
                // Push out: translate by normal * (k * depth).
                c.cycles += 40;
                c.f[0] = Addr(r32(c, GP).wrapping_sub(0x7190)).read_fpu(c).0;
                c.g[A1][0] = sx(base.wrapping_add(0x3a0));
                air_walls_scale(c, V1, 0, 0x10, 0x3c0);
                c.sq(V0, base.wrapping_add(0x3b0));
                air_walls_scale(c, V1, 1, 0x3b0, 0x3c0);
                c.g[A0][0] = this.off(RIDER).lw(c);
                c.sq(V0, base.wrapping_add(0x3a0));
                call_ptr(c, TRANSLATE, 0x0013_ab4c)?;
                // Into the wall? (-normal).x * vel.x < 0 means moving in.
                c.g[AT][0] = sx(0xbf80_0000);
                c.f[0] = 0xbf80_0000;
                air_walls_scale(c, V0, 0, 0x10, 0x3c0);
                let rider_now = this.off(RIDER).lw(c);
                c.g[A0][0] = rider_now;
                c.sq(V0, base.wrapping_add(0x3b0));
                c.lqc2(3, base.wrapping_add(0x3b0));
                c.lqc2(5, (rider_now as u32).wrapping_add(VEL));
                air_walls_dot(c, 5);
                c.qmfc2(V1, 4);
                c.f[20] = c.g[V1][0] as u32;
                let receding = FpuF32(c.f[20]).lt(FpuF32(c.f[21]));
                abi::set_fcc(c, receding);
                if receding {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                } else {
                    impact = true;
                }
            }
            if !impact {
                // 0x0013aba8: ~CollHull, no impact.
                c.cycles += 4;
                c.g[A1][0] = 2;
                call_ptr(c, HULL_DTOR, 0x0013_abb0)?;
                c.g[V0][0] = 0;
            } else {
                // 0x0013abb8: bounce, scaled by speed (with a floor).
                c.cycles += 18;
                let rider_now = r32(c, A0);
                c.f[0] = Addr(r32(c, GP).wrapping_sub(0x718c)).read_fpu(c).0;
                c.lqc2(3, rider_now.wrapping_add(VEL));
                air_walls_dot(c, 3);
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, base.wrapping_add(0x3c0));
                let cap = Addr(r32(c, GP).wrapping_sub(0x7188)).read_fpu(c);
                c.f[1] = cap.0;
                let k = FpuF32(c.f[20]) * FpuF32(c.f[0]);
                c.f[0] = k.0;
                c.lq(V0, base.wrapping_add(0x3c0));
                let floored = k.lt(cap);
                abi::set_fcc(c, floored);
                c.sq(V0, base.wrapping_add(0x3b0));
                if floored {
                    c.cycles += 1;
                    c.f[0] = c.f[1];
                }
                c.cycles += 32;
                c.f[0] = (FpuF32(c.f[20]) + FpuF32(c.f[0])).0;
                air_walls_scale(c, V1, 0, 0x10, 0x3d0);
                let rider_now = this.off(RIDER).lw(c);
                c.g[V1][0] = rider_now;
                c.sq(V0, base.wrapping_add(0x3c0));
                let vel = (rider_now as u32).wrapping_add(VEL);
                c.lqc2(3, vel);
                c.lqc2(4, base.wrapping_add(0x3c0));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, vel);
                c.g[A0][0] = this.off(RIDER).lw(c);
                call_ptr(c, BUILD_WORLD_MATRIX, 0x0013_ac40)?;
                // Rider_HandleImpact(rider, &{contact.pos, dir, normal, speed}, contact.surface)
                c.lq(V0, base.wrapping_add(0x3b0));
                c.g[A1][0] = sx(base.wrapping_add(0x3e0));
                c.lq(A3, base);
                c.lq(V1, base.wrapping_add(0x10));
                c.g[A0][0] = this.off(RIDER).lw(c);
                c.g[A2][0] = Addr(base.wrapping_add(0x4c)).lw(c);
                c.sq(V0, base.wrapping_add(0x3f0));
                c.sq(A3, base.wrapping_add(0x3e0));
                c.sq(V1, base.wrapping_add(0x400));
                let speed = FpuF32(c.f[20]);
                Addr(base.wrapping_add(0x410)).write_fpu(c, speed);
                call_ptr(c, HANDLE_IMPACT, 0x0013_ac6c)?;
                c.lq(V0, base.wrapping_add(0x10));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = 2;
                c.sq(V0, r32(c, S4));
                call_ptr(c, HULL_DTOR, 0x0013_ac80)?;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x470);
        frame.restore(c, S1, 0x460);
        frame.restore(c, S2, 0x450);
        frame.restore(c, S3, 0x440);
        frame.restore(c, S4, 0x430);
        frame.restore_ra(c, 0x420);
        frame.restore_fpr(c, 21, 0x488);
        frame.restore_fpr(c, 20, 0x480);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// RiderGroundState_CollideWalls (0x13f488, handplantcontrol.cpp)

/// Called functions (all through the function table, as the original).
pub(crate) const RIDER_GET_SUB_STATE: u32 = 0x0011_fee8;

pub(crate) const COLL_HULL_CTOR: u32 = 0x0032_f650;

pub(crate) const COLL_HULL_DTOR: u32 = 0x0032_f708;

pub(crate) const OVERLAP_LIST_FIND_CLOSEST_CONTACT: u32 = 0x0033_42d0;

pub(crate) const RIDER_TRANSLATE: u32 = 0x0010_6538;

pub(crate) const RIDER_DEFLECT_ALONG_WALL: u32 = 0x0010_65b0;

pub(crate) const RIDER_BUILD_WORLD_MATRIX: u32 = 0x0011_e098;

pub(crate) const RIDER_HANDLE_IMPACT: u32 = 0x0010_5d98;

pub(crate) const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;

pub(crate) const SSX_AUDIO_PLAY_GROUND_WALL_HIT_SOUND: u32 = 0x0029_89a8;

/// The ground state's rider.
pub(crate) const GS_RIDER: u32 = 0x18;

/// Rider fields.
pub(crate) const R_POS_Y: u32 = 0x118;

pub(crate) const R_VEL: u32 = 0x1e0;

pub(crate) const R_FORWARD: u32 = 0x370;

pub(crate) const R_GROUND_Y: u32 = 0x498;

pub(crate) const R_SPEED: u32 = 0x4c8;

/// The rider's controller object (vtable slot 0x40: "is in contact").
pub(crate) const R_CONTROLLER: u32 = 0x6c0;

pub(crate) const R_OVERLAP_LIST: u32 = 0x860;

pub(crate) const R_OVERLAP_HINT: u32 = 0x868;

pub(crate) const R_HULL: u32 = 0xaa0;

/// `.lit4` constants (gp-relative, subtracted): -0.9999 (head-on
/// threshold), 1.1 (push-out factor), 0.05 (bounce factor), 27.777779 (the
/// minimal bounce speed, 100 km/h in m/s).
pub(crate) const GP_HEAD_ON: u32 = 0x6fa0;

pub(crate) const GP_PUSH_OUT: u32 = 0x6f9c;

pub(crate) const GP_BOUNCE_SCALE: u32 = 0x6f98;

pub(crate) const GP_MIN_BOUNCE: u32 = 0x6f94;

/// Frame layout: the contact (0x00: pos, 0x10: normal, 0x20: the hit
/// surface's velocity,
/// 0x4c: material, 0x50: object), the hull volume (0x80, +0x10 its
/// direction), the impact info (0x3f0) and VU spill slots.
pub(crate) const F_HIT: u32 = 0x00;

pub(crate) const F_HIT_NORMAL: u32 = 0x10;

pub(crate) const F_HIT_VELOCITY: u32 = 0x20;

pub(crate) const F_HIT_MATERIAL: u32 = 0x4c;

pub(crate) const F_HIT_OBJECT: u32 = 0x50;

pub(crate) const F_HULL: u32 = 0x80;

pub(crate) const F_SPILL_A: u32 = 0x3a0;

pub(crate) const F_SPILL_B: u32 = 0x3b0;

pub(crate) const F_SPILL_C: u32 = 0x3c0;

pub(crate) const F_SPILL_D: u32 = 0x3d0;

pub(crate) const F_SPILL_E: u32 = 0x3e0;

pub(crate) const F_IMPACT: u32 = 0x3f0;

/// `vf4.x = vf3 . vf<b>` (xyz, through the `vf6.x = 1 + vf0.w` trick the
/// compiler's inline dot product uses).
#[inline]
pub(crate) fn vu_dot3(c: &mut Cpu, b: u8) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `vf5 = vf3 * rsqrt(vf3 . vf3)` (the inline normalise).
#[inline]
pub(crate) fn vu_normalize3(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::Src;
    vu_dot3(c, 3);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
}

/// `vf5 = q(at) * x` with `x` passed through gpr `gpr` (`mfc1`/`qmtc2`).
#[inline]
pub(crate) fn vu_scale(c: &mut Cpu, gpr: usize, at: u32) {
    use ssx3_ee_rt::vu0::{Src, X};
    c.lqc2(4, at);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
}

/// `vf5 = q(a) - q(b)`.
#[inline]
pub(crate) fn vu_sub(c: &mut Cpu, a: u32, b: u32) {
    use ssx3_ee_rt::vu0::Src;
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
}

/// `sqc2 vf5, tmp; lq v0, tmp; sq v0, dst...` (the compiler's VU spill).
#[inline]
pub(crate) fn vu_spill(c: &mut Cpu, frame: Frame, tmp: u32, dst: &[u32]) {
    let t = frame.at(c, tmp);
    c.sqc2(5, t);
    c.lq(2, t);
    for &d in dst {
        let a = frame.at(c, d);
        c.sq(2, a);
    }
}

/// `void RiderGroundState_CollideWalls(RiderGroundState* self)` (0x13f488):
/// the ground state's wall collision. Skipped in sub-state 9, and for a slow
/// rider (< 50) close to the ground that is not in contact. Otherwise the
/// rider's hull is swept along its forward direction against its overlap
/// list; on a hit (depth >= 0) the hit object is notified (vtable 0x150),
/// and unless the rider runs head-on into the wall (forward . normal <
/// -0.9999) the rider is pushed out by 1.1 * depth along the normal
/// (flattened against the forward direction when it faces the wall). If the
/// rider then still moves into the wall it is bounced off (by the into-wall
/// speed plus 5% of it, at least 27.78), deflected along the wall, its
/// matrix rebuilt, the impact handled and the wall-hit sound played. The
/// hull is destroyed on every path after its construction.
pub fn rider_ground_state_collide_walls(c: &mut Cpu) {
    use crate::abi::{c_lt_s, set_fpr};
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use ssx3_ee_rt::vu0::Src;

    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x490);
        frame.save(c, S2, 0x450);
        frame.save(c, S0, 0x470);
        frame.save(c, S1, 0x460);
        frame.save(c, S3, 0x440);
        frame.save_ra(c, 0x430);
        frame.save_fpr(c, 21, 0x488);
        frame.save_fpr(c, 20, 0x480);
        c.g[S2][0] = c.g[A0][0]; // self
        let this = Addr(r32(c, S2));

        // Rider_GetSubState(rider) == 9: nothing to do.
        c.g[A0][0] = this.off(GS_RIDER).lw(c);
        call_ptr(c, RIDER_GET_SUB_STATE, 0x0013_f4b4)?;
        c.g[V1][0] = 9;
        let skip = if c.g[V0][0] == c.g[V1][0] {
            true
        } else {
            // In contact? Else only when fast or off the ground.
            c.cycles += 9;
            let rider = this.off(GS_RIDER).lw(c) as u32;
            controller_in_contact(c, rider, true, 0x0013_f4dc)?;
            if c.g[V0][0] != 0 {
                false
            } else {
                c.cycles += 8;
                let rider = this.off(GS_RIDER).lw(c);
                c.g[V0][0] = rider;
                let rider = Addr(rider as u32);
                c.g[AT][0] = 0x4248_0000;
                let fifty = FpuF32(0x4248_0000);
                set_fpr(c, 2, fifty);
                let speed = rider.off(R_SPEED).read_fpu(c);
                set_fpr(c, 0, speed);
                if !c_lt_s(c, speed, fifty) {
                    false
                } else {
                    c.cycles += 8;
                    let y = rider.off(R_POS_Y).read_fpu(c);
                    let ground = rider.off(R_GROUND_Y).read_fpu(c);
                    set_fpr(c, 1, y);
                    let height = (ground - y).abs();
                    set_fpr(c, 0, height);
                    if c_lt_s(c, height, fifty) {
                        true
                    } else {
                        c.cycles += 1;
                        false
                    }
                }
            }
        };
        if !skip {
            c.g[S0][0] = this.off(GS_RIDER).lw(c);
            collide_walls_hit(c, frame)?;
        }
        frame.restore(c, S0, 0x470);

        c.cycles += 8;
        frame.restore(c, S1, 0x460);
        frame.restore(c, S2, 0x450);
        frame.restore(c, S3, 0x440);
        frame.restore_ra(c, 0x430);
        frame.restore_fpr(c, 21, 0x488);
        frame.restore_fpr(c, 20, 0x480);
        frame.close(c);
        Ok(())
    });

    /// `rider->controller->vt[0x40]()` ("in contact"), returning to `ret`.
    /// The two call sites leave different registers: the first keeps the
    /// `this` delta in a1, the second the controller address.
    fn controller_in_contact(c: &mut Cpu, rider: u32, first: bool, ret: u32) -> crate::call::Flow {
        let ctl = rider.wrapping_add(R_CONTROLLER);
        let vt = Addr(ctl).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr(vt as u32).off(0x40).lh(c);
        c.g[A1][0] = if first { delta } else { sx(ctl) };
        let method = Addr(vt as u32).off(0x44).lw(c);
        c.g[V0][0] = method;
        c.g[A0][0] = sx(ctl.wrapping_add(delta as u32));
        call_ptr(c, method as u32, ret)
    }

    /// The hull sweep and the response (rider in s0, self in s2).
    fn collide_walls_hit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
        c.cycles += 28;
        c.g[S1][0] = sx(frame.at(c, F_HULL));
        c.g[S3][0] = c.g[S1][0];
        let rider = r32(c, S0);
        // CollHull_ctor(&hull, rider->hull, !in_contact)
        controller_in_contact(c, rider, false, 0x0013_f548)?;
        c.g[A1][0] = Addr(rider).off(R_HULL).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0] ^ 1;
        call_ptr(c, COLL_HULL_CTOR, 0x0013_f558)?;
        // hull.f8 = 1; hull.dir = rider->forward;
        // d = OverlapList_FindClosestContact(rider->list, &hull, &hit, hint)
        let rider = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[V1][0] = rider;
        let rider = Addr(rider as u32);
        c.g[A0][0] = 1;
        Addr(frame.at(c, F_HULL + 8)).sw(c, 1);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        c.lq(2, rider.off(R_FORWARD).0);
        let a = frame.at(c, F_HULL + 0x10);
        c.sq(2, a);
        c.g[A3][0] = rider.off(R_OVERLAP_HINT).lw(c);
        c.g[A0][0] = rider.off(R_OVERLAP_LIST).lw(c);
        call_ptr(c, OVERLAP_LIST_FIND_CLOSEST_CONTACT, 0x0013_f580)?;
        let depth = FpuF32(c.f[0]);
        set_fpr(c, 20, depth);
        set_fpr(c, 0, FpuF32::ZERO);
        let missed = c_lt_s(c, depth, FpuF32::ZERO);
        c.g[A0][0] = c.g[S3][0];
        if missed {
            return destroy_hull(c);
        }

        // Tell the hit object (vtable 0x150 of its owner).
        c.cycles += 3;
        let obj = Addr(frame.at(c, F_HIT_OBJECT)).lw(c);
        c.g[A0][0] = obj;
        if obj != 0 {
            c.cycles += 3;
            let owner = Addr(obj as u32).off(0xc).lw(c);
            c.g[A2][0] = owner;
            c.g[A1][0] = c.g[SP][0];
            if owner != 0 {
                c.cycles += 5;
                let vt = Addr(owner as u32).off(0xc).lw(c);
                c.g[V0][0] = vt;
                let delta = Addr(vt as u32).off(0x150).lh(c);
                let method = Addr(vt as u32).off(0x154).lw(c);
                c.g[V1][0] = method;
                c.g[A0][0] = sx((owner as u32).wrapping_add(delta as u32));
                call_ptr(c, method as u32, 0x0013_f5c4)?;
            }
            c.cycles += 1;
        }
        let rider = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[V1][0] = rider;
        let rider = rider as u32;

        // dot = forward . normal; head-on: give up.
        c.cycles += 14;
        c.lwc1(0, r32(c, GP).wrapping_sub(GP_HEAD_ON));
        c.lqc2(3, rider.wrapping_add(R_FORWARD));
        c.lqc2(5, frame.at(c, F_HIT_NORMAL));
        vu_dot3(c, 5);
        c.qmfc2(2, 4);
        c.mtc1(2, 1);
        c.c_lt_s(1, 0);
        let head_on = c.fcc();
        c.g[A0][0] = c.g[S3][0];
        if head_on {
            return destroy_hull(c);
        }
        c.cycles += 4;
        c.mtc1(0, 21); // f21 = 0
        let facing = c_lt_s(c, FpuF32(c.f[1]), FpuF32::ZERO);
        if facing {
            // normal = normalize(normal - forward * dot)
            c.cycles += 26;
            c.mfc1(2, 1);
            vu_scale(c, 2, rider.wrapping_add(R_FORWARD));
            vu_spill(c, frame, F_SPILL_D, &[F_SPILL_C]);
            vu_sub(c, frame.at(c, F_HIT_NORMAL), frame.at(c, F_SPILL_C));
            vu_spill(c, frame, F_SPILL_D, &[F_SPILL_B]);
            c.lqc2(3, frame.at(c, F_SPILL_B));
            vu_normalize3(c);
            vu_spill(c, frame, F_SPILL_D, &[F_HIT_NORMAL, F_SPILL_A]);
        }

        // Rider_Translate(rider, normal * 1.1 * depth)
        c.cycles += 47;
        c.lwc1(0, r32(c, GP).wrapping_sub(GP_PUSH_OUT));
        c.g[A1][0] = sx(frame.at(c, F_SPILL_A));
        c.mfc1(3, 0);
        vu_scale(c, 3, frame.at(c, F_HIT_NORMAL));
        vu_spill(c, frame, F_SPILL_C, &[F_SPILL_B]);
        c.mfc1(3, 20);
        vu_scale(c, 3, frame.at(c, F_SPILL_B));
        let t = frame.at(c, F_SPILL_C);
        c.sqc2(5, t);
        c.lq(2, t);
        c.g[A0][0] = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        let a = frame.at(c, F_SPILL_A);
        c.sq(2, a);
        call_ptr(c, RIDER_TRANSLATE, 0x0013_f6c0)?;

        // rel = rider->vel - hit.velocity; into = -normal . rel
        let rider = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[V0][0] = rider;
        let rider = rider as u32;
        vu_sub(c, rider.wrapping_add(R_VEL), frame.at(c, F_HIT_VELOCITY));
        let t = frame.at(c, F_SPILL_C);
        c.sqc2(5, t);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0); // f0 = -1.0
        c.lq(2, t);
        let a = frame.at(c, F_SPILL_B);
        c.sq(2, a);
        c.mfc1(3, 0);
        vu_scale(c, 3, frame.at(c, F_HIT_NORMAL));
        vu_spill(c, frame, F_SPILL_D, &[F_SPILL_C]);
        c.lqc2(3, frame.at(c, F_SPILL_C));
        c.lqc2(5, frame.at(c, F_SPILL_B));
        vu_dot3(c, 5);
        c.qmfc2(3, 4);
        c.mtc1(3, 20);
        let into = FpuF32(c.f[20]);
        let receding = c_lt_s(c, into, FpuF32(c.f[21]));
        c.g[A0][0] = c.g[S3][0];
        if receding {
            return destroy_hull(c);
        }

        // dir = normalize(rel); push = max(into * 0.05, 27.78)
        c.cycles += 18;
        c.lwc1(0, r32(c, GP).wrapping_sub(GP_BOUNCE_SCALE));
        c.lqc2(3, frame.at(c, F_SPILL_B));
        vu_normalize3(c);
        let t = frame.at(c, F_SPILL_D);
        c.sqc2(5, t);
        let min_push = Addr(r32(c, GP).wrapping_sub(GP_MIN_BOUNCE)).read_fpu(c);
        set_fpr(c, 1, min_push);
        let mut push = into * FpuF32(c.f[0]);
        set_fpr(c, 0, push);
        c.lq(2, t);
        let small = c_lt_s(c, push, min_push);
        let a = frame.at(c, F_SPILL_C);
        c.sq(2, a);
        if small {
            c.cycles += 1;
            push = min_push;
            set_fpr(c, 0, push);
        }

        // rider->vel += normal * (into + push)
        c.cycles += 38;
        set_fpr(c, 0, into + push);
        c.g[A1][0] = c.g[SP][0];
        c.mfc1(3, 0);
        vu_scale(c, 3, frame.at(c, F_HIT_NORMAL));
        let t = frame.at(c, F_SPILL_E);
        c.sqc2(5, t);
        c.lq(2, t);
        let rider = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[V1][0] = rider;
        let rider = rider as u32;
        let a = frame.at(c, F_SPILL_D);
        c.sq(2, a);
        c.lqc2(3, rider.wrapping_add(R_VEL));
        c.lqc2(4, frame.at(c, F_SPILL_D));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, rider.wrapping_add(R_VEL));
        // Rider_DeflectAlongWall(rider, &hit); Rider_BuildWorldMatrix(rider)
        c.g[A0][0] = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        call_ptr(c, RIDER_DEFLECT_ALONG_WALL, 0x0013_f7d0)?;
        c.g[A0][0] = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        call_ptr(c, RIDER_BUILD_WORLD_MATRIX, 0x0013_f7d8)?;
        // Rider_HandleImpact(rider, &{ hit.pos, dir, hit.normal, into },
        // hit.material)
        c.lq(7, frame.at(c, F_HIT));
        c.g[A1][0] = sx(frame.at(c, F_IMPACT));
        c.lq(2, frame.at(c, F_SPILL_C));
        c.lq(3, frame.at(c, F_HIT_NORMAL));
        c.g[A0][0] = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[A2][0] = Addr(frame.at(c, F_HIT_MATERIAL)).lw(c);
        let a = frame.at(c, F_IMPACT);
        c.sq(7, a);
        c.sq(2, a + 0x10);
        c.sq(3, a + 0x20);
        frame_swc1(c, frame, 20, F_IMPACT + 0x30);
        call_ptr(c, RIDER_HANDLE_IMPACT, 0x0013_f804)?;
        // SSXAudioSystem_PlayGroundWallHitSound(GetInstance(), rider)
        call_ptr(c, SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0013_f80c)?;
        c.g[A1][0] = Addr(r32(c, S2)).off(GS_RIDER).lw(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, SSX_AUDIO_PLAY_GROUND_WALL_HIT_SOUND, 0x0013_f818)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 2;
        call_ptr(c, COLL_HULL_DTOR, 0x0013_f824)
    }

    /// `swc1 f, off(sp)`.
    fn frame_swc1(c: &mut Cpu, frame: Frame, f: usize, off: u32) {
        let a = frame.at(c, off);
        c.swc1(f, a);
    }

    /// `CollHull_dtor(&hull, 2)` (a0 already holds the hull).
    fn destroy_hull(c: &mut Cpu) -> crate::call::Flow {
        c.cycles += 4;
        c.g[A1][0] = 2;
        call_ptr(c, COLL_HULL_DTOR, 0x0013_f73c)
    }
}

// ---------------------------------------------------------------------------
// RiderGroundState_PostUpdate (0x13f178, handplantcontrol.cpp)

pub(crate) const RIDER_LAUNCH_INTO_AIR: u32 = 0x0011_4298;

pub(crate) const RIDER_START_WIPEOUT: u32 = 0x0010_eb30;

pub(crate) const RIDER_CHECK_RESET_PATH: u32 = 0x0011_6120;

pub(crate) const RIDER_UPDATE_BOUNDS: u32 = 0x0011_e150;

pub(crate) const RIDER_HANDLE_WALL_COLLISION: u32 = 0x0010_5398;

pub(crate) const RIDER_CHECK_RIDER_COLLISIONS: u32 = 0x0010_7888;

pub(crate) const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;

pub(crate) const RIDER_SET_MAIN_STATE: u32 = 0x0011_fe78;

pub(crate) const SSX_AUDIO_PLAY_TAKEOFF_SOUND: u32 = 0x0029_4170;

pub(crate) const RIDER_GROUND_STATE_COLLIDE_WALLS: Callee = Callee {
    addr: 0x0013_f488,
    f: ee::s01::RiderGroundState_CollideWalls,
};

/// The ground state: "leaving the ground" flag.
pub(crate) const GS_LEAVING: u32 = 0x0c;

/// Rider fields.
pub(crate) const R_POS: u32 = 0x110;

pub(crate) const R_SURFACE: u32 = 0x438;

pub(crate) const R_CONTACT_FLAGS: u32 = 0x2d4;

pub(crate) const R_MAX_SPEED: u32 = 0x2e4;

pub(crate) const R_SMOOTH_UP: u32 = 0x390;

pub(crate) const R_SPEED_FRACTION: u32 = 0x75c;

pub(crate) const R_ABS_VEL_Z: u32 = 0x754;

pub(crate) const R_ABS_SPIN: u32 = 0x750;

/// The surface type that wipes the rider out, and the surface table's
/// entry size and "resets the path" word.
pub(crate) const SURFACE_WIPEOUT: u64 = 0x12;

pub(crate) const SURFACE_ENTRY: u64 = 0xb0;

pub(crate) const SURFACE_RESETS: u32 = 0x44;

/// gp-relative globals: the world, the hull's ground sphere mask, the audio
/// system; `.lit4` 3333.3335 (the speed of a full speed fraction).
pub(crate) const GP_WORLD: u32 = 0x848;

pub(crate) const GP_HULL_GROUND_MASK: u32 = 0x1f94;

pub(crate) const GP_AUDIO: u32 = 0x410;

pub(crate) const GP_FULL_SPEED: u32 = 0x6fac;

/// `void RiderGroundState_PostUpdate(RiderGroundState* self)` (0x13f178):
/// after the ground physics step. A rider leaving the ground is launched
/// (`Rider_LaunchIntoAir(-1)`); otherwise surface type 0x12 starts a
/// wipeout (position, normalised velocity, forward), and a surface that
/// resets the path (or contact flag 2) checks the reset path. Then the
/// bounds are updated and the walls collided with the hull's ground spheres
/// masked in; wall and rider collisions are handled; a rider leaving the
/// ground while in main state 0 gets main state 1 and the take-off sound. Finally the smoothed up vector follows forward (normalise(up +
/// forward / 2)), the velocity is clamped to the maximal speed, and the
/// speed fraction (|v| / 3333.33, 0..1) and the absolute z velocity and spin
/// are stored.
pub fn rider_ground_state_post_update(c: &mut Cpu) {
    use crate::guest::Addr;
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save_ra(c, 0x60);
        c.g[S0][0] = c.g[A0][0];
        let this = Addr(r32(c, S0));
        let rider = |c: &mut Cpu| this.off(GS_RIDER).lw(c);

        c.g[V0][0] = this.off(GS_LEAVING).lw(c);
        if c.g[V0][0] != 0 {
            // Rider_LaunchIntoAir(rider, -1.0)
            c.cycles += 6;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 12);
            c.g[A0][0] = rider(c);
            call_ptr(c, RIDER_LAUNCH_INTO_AIR, 0x0013_f1a4)?;
        } else {
            c.g[A3][0] = rider(c);
            let r = r32(c, A3);
            // entry = &world->surfaces->table[rider->surface]
            c.cycles += 9;
            c.g[V1][0] = SURFACE_ENTRY;
            let world = Addr(r32(c, GP).wrapping_sub(GP_WORLD)).lw(c);
            c.g[V0][0] = world;
            c.g[A2][0] = SURFACE_WIPEOUT;
            c.g[A1][0] = Addr(r).off(R_SURFACE).lw(c);
            c.g[A0][0] = Addr(world as u32).off(0x84).lw(c);
            c.mult(0, V1, A1, V1);
            c.g[V0][0] = Addr(r32(c, A0)).off(0x44).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            if c.g[A1][0] == c.g[A2][0] {
                // Rider_StartWipeout(rider, 0x168, 0, surface, &info)
                c.cycles += 27;
                c.lq(2, r.wrapping_add(R_POS));
                c.g[A1][0] = 0x168;
                c.g[A2][0] = 0;
                c.g[T0][0] = c.g[SP][0];
                let a = frame.at(c, 0);
                c.sq(2, a);
                c.lqc2(3, r.wrapping_add(R_VEL));
                vu_normalize3(c);
                vu_spill(c, frame, 0x50, &[0x10, 0x40]);
                let r = rider(c);
                c.g[V1][0] = r;
                c.g[A0][0] = r;
                c.lq(2, (r as u32).wrapping_add(R_FORWARD));
                Addr(frame.at(c, 0x30)).sw(c, 0);
                let a = frame.at(c, 0x20);
                c.sq(2, a);
                c.g[A3][0] = Addr(r as u32).off(R_SURFACE).lw(c);
                call_ptr(c, RIDER_START_WIPEOUT, 0x0013_f234)?;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V0)).off(SURFACE_RESETS).lw(c);
                c.g[A0][0] = c.g[A3][0];
                let mut reset = c.g[V0][0] != 0;
                if !reset {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r).off(R_CONTACT_FLAGS).lh(c) & 2;
                    reset = c.g[V0][0] != 0;
                }
                if reset {
                    // Rider_CheckResetPath(rider, 0, 1)
                    c.cycles += 4;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 1;
                    call_ptr(c, RIDER_CHECK_RESET_PATH, 0x0013_f264)?;
                }
            }
        }
        c.g[A0][0] = rider(c);

        // Rider_UpdateBounds(rider, 0); the walls with the ground spheres.
        c.cycles += 21;
        c.g[A1][0] = 0;
        call_ptr(c, RIDER_UPDATE_BOUNDS, 0x0013_f270)?;
        c.g[A1][0] = rider(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(GP_HULL_GROUND_MASK)).lw(c);
        c.g[V0][0] = Addr(r32(c, A1)).off(R_HULL).lw(c);
        let mask = c.g[V1][0];
        Addr(r32(c, V0)).off(0x28).sw(c, mask);
        RIDER_GROUND_STATE_COLLIDE_WALLS.call(c, 0x0013_f288)?;
        c.g[A0][0] = rider(c);
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = 0;
        c.g[V1][0] = Addr(r32(c, A0)).off(R_HULL).lw(c);
        let all = c.g[V0][0];
        Addr(r32(c, V1)).off(0x28).sw(c, all);
        // Rider_HandleWallCollision(rider, 0); Rider_CheckRiderCollisions(rider)
        c.g[A0][0] = rider(c);
        call_ptr(c, RIDER_HANDLE_WALL_COLLISION, 0x0013_f2a8)?;
        c.g[A0][0] = rider(c);
        call_ptr(c, RIDER_CHECK_RIDER_COLLISIONS, 0x0013_f2b0)?;
        c.g[V0][0] = this.off(GS_LEAVING).lw(c);
        if c.g[V0][0] != 0 {
            // Main state 0: set 1 and play the take-off sound.
            c.cycles += 4;
            c.g[A0][0] = rider(c);
            call_ptr(c, RIDER_GET_MAIN_STATE, 0x0013_f2c4)?;
            let state = c.g[V0][0];
            c.g[A0][0] = rider(c);
            if state == 0 {
                c.cycles += 6;
                c.g[A1][0] = 1;
                call_ptr(c, RIDER_SET_MAIN_STATE, 0x0013_f2d4)?;
                c.g[A0][0] = Addr(r32(c, GP).wrapping_add(GP_AUDIO)).lw(c);
                c.g[A1][0] = rider(c);
                call_ptr(c, SSX_AUDIO_PLAY_TAKEOFF_SOUND, 0x0013_f2e0)?;
            }
        }
        c.g[A0][0] = rider(c);

        // up = normalize(up + forward * 0.5)
        c.cycles += 44;
        let r = r32(c, A0);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.mfc1(V1, 0);
        vu_scale(c, V1, r.wrapping_add(R_FORWARD));
        vu_spill(c, frame, 0x30, &[0x20]);
        c.g[V1][0] = rider(c);
        c.lqc2(3, r32(c, V1).wrapping_add(R_SMOOTH_UP));
        c.lqc2(4, frame.at(c, 0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        vu_spill(c, frame, 0x30, &[0x10]);
        c.lqc2(3, frame.at(c, 0x10));
        vu_normalize3(c);
        vu_spill(c, frame, 0x30, &[0]);
        c.sq(2, r.wrapping_add(R_SMOOTH_UP));
        // |v|, clamped to the maximal speed.
        let r = rider(c);
        c.g[V1][0] = r;
        let r = r as u32;
        c.lqc2(3, r.wrapping_add(R_VEL));
        vu_dot3(c, 3);
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V0, 22);
        c.mtc1(V0, 1); // f1 = |v|
        c.lwc1(0, r.wrapping_add(R_MAX_SPEED));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.cycles += 6;
            c.div_s(0, 0, 1);
            c.mfc1(V0, 0);
            c.lqc2(4, r.wrapping_add(R_VEL));
            c.qmtc2(V0, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, r.wrapping_add(R_VEL));
        }
        // fraction = 0 below 0, |v| / full below full, else 1.
        c.cycles += 6;
        c.mtc1(0, 0);
        c.lwc1(2, r32(c, GP).wrapping_sub(GP_FULL_SPEED));
        c.c_le_s(0, 1);
        c.g[V0][0] = rider(c);
        if c.fcc() {
            c.cycles += 5;
            c.c_le_s(1, 2);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            if c.fcc() {
                c.cycles += 1;
                c.div_s(0, 1, 2);
            }
        }
        c.cycles += 13;
        frame.restore_ra(c, 0x60);
        c.swc1(0, r32(c, V0).wrapping_add(R_SPEED_FRACTION));
        let r = rider(c);
        c.g[V1][0] = r;
        c.lwc1(0, (r as u32).wrapping_add(0x1f0));
        c.abs_s(0, 0);
        c.swc1(0, (r as u32).wrapping_add(R_ABS_VEL_Z));
        let r = rider(c);
        c.g[V0][0] = r;
        frame.restore(c, S0, 0x70);
        c.lwc1(0, (r as u32).wrapping_add(0x214));
        c.abs_s(0, 0);
        c.swc1(0, (r as u32).wrapping_add(R_ABS_SPIN));
        frame.close(c);
        Ok(())
    })
}

/// `float RiderGroundState_ComputeBoostAccel(RiderGroundState* this,
/// const Tuning* p)` (0x13c948): the forward acceleration of a riding
/// rider this tick (f0); 0 when `+0x214` is non-zero (a brake / air
/// state) or the rider has finished.
///
/// - A ground-material scale (1, or a `.lit4` factor on materials >= 17).
/// - The slope factor: 1 when the rider is flagged (`+0x2d4` bit 4), else
///   from the heading's angle (`Math_Atan` of the `+0x1b0` direction,
///   folded into a half turn) relative to the fall line (`+0x4cc`), faded
///   out past a `.lit4` angle; on materials >= 17 the factor is ignored
///   when negative.
/// - Speed-dependent drag (`+0x2e8`, `+0x2fc`, `+0x1f0`, `+0x3a8`) and,
///   while the speed (VU0 length of `+0x1e0`) is under the boost cap, the
///   character's acceleration attribute (`BECharInt_GetAttrib1Factor`), a
///   boost-pose bonus (layer 2 anim id 0x16) and the carve factor on
///   materials 11..13 (`+0x378`).
/// - Result: `slope * (p.accel * accel * material + rider[0x220] * k)`.
#[allow(unused_labels)]
pub fn rider_ground_state_compute_boost_accel(c: &mut Cpu) {
    use boost_accel::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0xa0);
        'b_13c9a0: {
            c.cycles += 21;
            frame.save(c, S0, 0x50);
            frame.save(c, S1, 0x40);
            frame.save(c, S2, 0x30);
            c.g[S1][0] = c.g[A0][0];
            frame.save_fpr(c, 26, 0x90);
            frame.save_ra(c, 0x20);
            frame.save_fpr(c, 25, 0x88);
            frame.save_fpr(c, 24, 0x80);
            frame.save_fpr(c, 23, 0x78);
            frame.save_fpr(c, 22, 0x70);
            frame.save_fpr(c, 21, 0x68);
            frame.save_fpr(c, 20, 0x60);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 26);
            c.g[V1][0] = c.lw(at(c, S1, RIDER));
            c.g[V0][0] = c.lw(at(c, V1, GROUND_MATERIAL));
            c.g[S0][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
            c.g[S0][0] = c.g[S0][0] ^ 0x1;
            let t = c.g[S0][0] == 0u64;
            c.g[S2][0] = c.g[A1][0];
            if t {
                break 'b_13c9a0;
            } else {
                c.cycles += 1;
                c.lwc1(26, gp(c, -0x70e4));
                break 'b_13c9a0;
            }
        }
        'b_13ccb8: {
            c.cycles += 6;
            c.lwc1(0, at(c, V1, 0x214));
            c.mtc1(0, 23);
            c.c_eq_s(0, 23);
            let t = !c.fcc();
            c.mov_s(0, 23);
            if t {
                break 'b_13ccb8;
            } else {
                c.cycles += 5;
                c.lwc1(0, at(c, V1, FINISH_TIME));
                c.c_le_s(23, 0);
                let t = c.fcc();
                c.mov_s(0, 23);
                if t {
                    break 'b_13ccb8;
                } else {
                    'b_13caf4: {
                        'b_13c9f0: {
                            c.cycles += 5;
                            c.g[V0][0] = c.lh(at(c, V1, 0x2d4));
                            c.mov_s(22, 23);
                            c.g[V0][0] = c.g[V0][0] & 0x10;
                            let t = c.g[V0][0] == 0u64;
                            c.mov_s(24, 22);
                            if t {
                                break 'b_13c9f0;
                            } else {
                                c.cycles += 4;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 24);

                                break 'b_13caf4;
                            }
                        }
                        'b_13ca84: {
                            'b_13ca80: {
                                'b_13ca44: {
                                    c.cycles += 10;
                                    c.lq(2, at(c, V1, 0x1b0));
                                    c.lwc1(25, gp(c, -0x70e0));
                                    c.sq(2, frame.at(c, 0));
                                    c.lq(2, at(c, V1, 0x1b0));
                                    c.lwc1(21, frame.at(c, 0x4));
                                    c.sq(2, frame.at(c, 0x10));
                                    c.lwc1(20, frame.at(c, 0x10));
                                    c.c_eq_s(20, 22);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_13ca44;
                                    } else {
                                        c.cycles += 4;
                                        c.c_eq_s(21, 22);
                                        let t = c.fcc();
                                        c.mov_s(12, 21);
                                        if t {
                                            break 'b_13ca80;
                                        } else {
                                            c.cycles += 4;
                                            c.c_le_s(22, 21);
                                            c.lwc1(12, gp(c, -0x70dc));
                                            let t = !c.fcc();
                                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                            if t {
                                                break 'b_13ca84;
                                            } else {
                                                c.cycles += 3;
                                                c.lwc1(12, gp(c, -0x70d8));

                                                break 'b_13ca84;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.div_s(12, 21, 20);
                                call_ptr(c, MATH_ATAN, 0x0013ca50)?;
                                c.c_lt_s(20, 22);
                                let t = !c.fcc();
                                c.mov_s(12, 0);
                                if t {
                                    break 'b_13ca80;
                                } else {
                                    'b_13ca78: {
                                        c.cycles += 3;
                                        c.c_lt_s(22, 21);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_13ca78;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, gp(c, -0x70d4));
                                            c.add_s(12, 12, 0);
                                            break 'b_13ca80;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.lwc1(0, gp(c, -0x70d0));
                                    c.sub_s(12, 12, 0);
                                    break 'b_13ca80;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                            break 'b_13ca84;
                        }
                        'b_13cac4: {
                            c.cycles += 13;
                            c.lwc1(0, gp(c, -0x70cc));
                            c.lwc1(1, at(c, V0, 0x4cc));
                            c.g[V1][0] = c.g[V0][0];
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 2);
                            c.sub_s(12, 12, 1);
                            c.mul_s(0, 12, 0);
                            c.add_s(0, 0, 2);
                            c.cvt_w_s(1, 0);
                            c.cvt_s_w(1, 1);
                            c.c_lt_s(0, 1);
                            let t = !c.fcc();

                            if t {
                                break 'b_13cac4;
                            } else {
                                c.cycles += 3;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 0);
                                c.sub_s(1, 1, 0);
                                break 'b_13cac4;
                            }
                        }
                        c.cycles += 7;
                        c.lwc1(0, gp(c, -0x70c8));
                        c.mul_s(0, 1, 0);
                        c.sub_s(0, 12, 0);
                        c.abs_s(12, 0);
                        c.c_lt_s(12, 25);
                        let t = !c.fcc();

                        if t {
                            break 'b_13caf4;
                        } else {
                            c.cycles += 5;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 1);
                            c.div_s(0, 1, 25);
                            c.mul_s(0, 12, 0);
                            c.sub_s(24, 1, 0);
                            break 'b_13caf4;
                        }
                    }
                    'b_13cb1c: {
                        c.cycles += 7;
                        c.mtc1(0, 0);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 25);
                        c.c_le_s(24, 0);
                        if !c.fcc() {
                            c.lwc1(0, at(c, V1, 0x2e8));
                            break 'b_13cb1c;
                        } else {
                            'b_13cb18: {
                                c.cycles += 2;
                                if c.g[S0][0] != 0u64 {
                                    c.mov_s(25, 0);
                                    break 'b_13cb18;
                                } else {
                                    break 'b_13cb18;
                                }
                            }
                            c.cycles += 1;
                            c.lwc1(0, at(c, V1, 0x2e8));
                            break 'b_13cb1c;
                        }
                    }
                    'b_13cb48: {
                        c.cycles += 7;
                        c.mtc1(0, 4);
                        c.lwc1(2, at(c, V1, 0x2fc));
                        c.lwc1(1, gp(c, -0x70c4));
                        c.c_lt_s(4, 0);
                        let t = !c.fcc();
                        c.mul_s(2, 2, 1);
                        if t {
                            break 'b_13cb48;
                        } else {
                            c.cycles += 4;
                            c.g[AT][0] = 0x40b00000;
                            c.mtc1(1, 0);
                            c.mul_s(0, 24, 0);
                            c.max_s(2, 0, 2);
                            break 'b_13cb48;
                        }
                    }
                    'b_13cbb4: {
                        c.cycles += 4;
                        c.c_lt_s(4, 2);
                        if !c.fcc() {
                            c.lwc1(0, at(c, S2, PARAM_ACCEL));
                            break 'b_13cbb4;
                        } else {
                            c.cycles += 8;
                            c.lwc1(0, at(c, V1, 0x1f0));
                            c.lwc1(1, gp(c, -0x70c0));
                            c.abs_s(0, 0);
                            c.sub_s(0, 1, 0);
                            c.c_lt_s(4, 0);
                            if !c.fcc() {
                                c.lwc1(0, at(c, S2, PARAM_ACCEL));
                                break 'b_13cbb4;
                            } else {
                                'b_13cba4: {
                                    'b_13cba0: {
                                        c.cycles += 5;
                                        c.lwc1(3, at(c, V1, 0x3a8));
                                        c.c_le_s(4, 3);
                                        let t = !c.fcc();
                                        c.mul_s(2, 2, 0);
                                        if t {
                                            break 'b_13cba0;
                                        } else {
                                            c.cycles += 5;
                                            c.lwc1(0, gp(c, -0x70bc));
                                            c.lwc1(1, gp(c, -0x70b8));
                                            c.mul_s(0, 3, 0);
                                            c.add_s(0, 0, 1);
                                            break 'b_13cba4;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.lwc1(0, gp(c, -0x70b4));
                                    break 'b_13cba4;
                                }
                                c.cycles += 4;
                                c.mul_s(0, 2, 0);
                                c.add_s(0, 23, 0);
                                c.mov_s(22, 0);
                                c.lwc1(0, at(c, S2, PARAM_ACCEL));
                                break 'b_13cbb4;
                            }
                        }
                    }
                    'b_13cc04: {
                        c.cycles += 19;
                        c.lqc2(3, at(c, V1, 0x1e0));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.vu0().sqrt(4, 0);
                        c.vu0().waitq();
                        c.cfc2(2, 22);
                        c.mtc1(2, 3);
                        c.lwc1(2, gp(c, -0x70b0));
                        c.mul_s(0, 0, 26);
                        c.lwc1(1, gp(c, -0x70ac));
                        c.mul_s(0, 0, 2);
                        c.sub_s(0, 0, 3);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();
                        c.mov_s(20, 1);
                        if t {
                            break 'b_13cc04;
                        } else {
                            c.cycles += 1;
                            c.mov_s(20, 0);
                            break 'b_13cc04;
                        }
                    }
                    'b_13cc9c: {
                        c.cycles += 5;
                        c.mtc1(0, 21);
                        c.c_lt_s(21, 20);
                        if !c.fcc() {
                            c.lwc1(0, at(c, S2, PARAM_BOOST));
                            break 'b_13cc9c;
                        } else {
                            'b_13cc60: {
                                c.cycles += 17;
                                BE_LIBRARY_GET_INSTANCE.call(c, 0x0013cc20)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x3;
                                BE_LIBRARY_GET_INTERFACE.call(c, 0x0013cc2c)?;
                                c.g[V1][0] = c.lw(at(c, S1, RIDER));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lw(at(c, V1, AI_INDEX));
                                c.g[A2][0] = c.lw(at(c, V1, CHARACTER_SLOT));
                                BE_CHAR_INT_GET_ATTRIB1_FACTOR.call(c, 0x0013cc40)?;
                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ID, 0x0013cc50)?;
                                c.g[V1][0] = 0x16;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.mov_s(1, 21);
                                if t {
                                    break 'b_13cc60;
                                } else {
                                    c.cycles += 1;
                                    c.lwc1(1, gp(c, -0x70a8));
                                    break 'b_13cc60;
                                }
                            }
                            'b_13cc84: {
                                c.cycles += 6;
                                c.g[V1][0] = c.lw(at(c, S1, RIDER));
                                c.g[V0][0] = c.lw(at(c, V1, GROUND_MATERIAL));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb));
                                c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_13cc84;
                                } else {
                                    c.cycles += 3;
                                    c.lwc1(0, at(c, V1, 0x378));
                                    c.abs_s(0, 0);
                                    c.mul_s(20, 20, 0);
                                    break 'b_13cc84;
                                }
                            }
                            c.cycles += 6;
                            c.lwc1(0, gp(c, -0x70a4));
                            c.mul_s(0, 1, 0);
                            c.max_s(0, 24, 0);
                            c.mul_s(0, 0, 20);
                            c.add_s(22, 22, 0);
                            c.lwc1(0, at(c, S2, PARAM_BOOST));
                            break 'b_13cc9c;
                        }
                    }
                    c.cycles += 7;
                    c.lwc1(2, at(c, V1, 0x220));
                    c.mul_s(0, 0, 22);
                    c.lwc1(1, gp(c, -0x70a0));
                    c.mul_s(2, 2, 1);
                    c.mul_s(0, 0, 26);
                    c.add_s(0, 0, 2);
                    c.mul_s(0, 25, 0);
                    break 'b_13ccb8;
                }
            }
        }
        c.cycles += 13;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 26, 0x90);
        frame.restore_fpr(c, 25, 0x88);
        frame.restore_fpr(c, 24, 0x80);
        frame.restore_fpr(c, 23, 0x78);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// --------------------------------------------- RiderGroundState_ComputeDrag

/// `float RiderGroundState_ComputeDrag(RiderGroundState* this, DragCfg* cfg,
/// float speed /*f12*/)` (0x13ccf0): the ground state's along-track drag.
///
/// - Three character factors from the behaviour library (interface 3:
///   attribute factors 0, 1 and 3 of the rider's player/value): `k1 = 1 /
///   (f1 * a + 1)`, `k3 = f3 * b + 1` (`.lit4` gains).
/// - The drag curve: `|speed| * k` looked up in the four points of `cfg`
///   (`+0x90..+0xac`: x/y pairs, linear between them, clamped at the ends),
///   blended toward `k1` by `|speed| / max`.
/// - A ramp on the rider's `+0x378` (0..1 between two thresholds) scales a
///   turn-dependent lift (`|turn|^2 * k3`) and the turn drag.
/// - Surfaces 2, 3 and 13 scale by `0.5 - depth / this->depth`; climbing
///   (`+0x1e8 > +0x3d8`), a stance mismatch without boost, and climbing
///   forward on surface 13 scale by `.lit4` factors.
/// - Result in f0: `((-speed * surface) * (curve + lift) * climb * stance -
///   turnDrag) * f5`.
///
/// The FPU sequence is the original's, register for register; the blocks
/// follow its labels.
pub fn rider_ground_state_compute_drag(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        'b_13ce64: {
            'b_13ce54: {
                'b_13ce50: {
                    'b_13ce2c: {
                        c.cycles += 60;
                        frame.save(c, S0, 0x20);
                        frame.save(c, S1, 0x10);
                        c.g[S0][0] = c.g[A1][0];
                        frame.save_fpr(c, 23, 0x48);
                        c.g[S1][0] = c.g[A0][0];
                        frame.save_fpr(c, 22, 0x40);
                        frame.save_fpr(c, 21, 0x38);
                        frame.save_fpr(c, 20, 0x30);
                        c.mov_s(21, 12);
                        frame.save_ra(c, 0);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 22);
                        c.g[V0][0] = c.lw(at(c, S1, 0x18));
                        c.lwc1(0, at(c, V0, 0x214));
                        c.abs_s(20, 0);
                        call_ptr(c, 0x0014dc80, 0x0013cd38)?; // BELibrary_GetInstance
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x3;
                        call_ptr(c, 0x0014dd58, 0x0013cd44)?; // BELibrary_GetInterface
                        c.g[V1][0] = c.lw(at(c, S1, 0x18));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.lw(at(c, V1, 0xb34));
                        c.g[A1][0] = c.lw(at(c, V1, 0x86c));
                        call_ptr(c, 0x001494c0, 0x0013cd58)?; // BECharInt_GetAttrib0Factor
                        call_ptr(c, 0x0014dc80, 0x0013cd60)?; // BELibrary_GetInstance
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x3;
                        call_ptr(c, 0x0014dd58, 0x0013cd6c)?; // BELibrary_GetInterface
                        c.g[V1][0] = c.lw(at(c, S1, 0x18));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.lw(at(c, V1, 0xb34));
                        c.g[A1][0] = c.lw(at(c, V1, 0x86c));
                        call_ptr(c, 0x001493d8, 0x0013cd80)?; // BECharInt_GetAttrib1Factor
                        c.lwc1(1, gpo(c, -0x709c));
                        c.mul_s(0, 0, 1);
                        c.add_s(0, 0, 22);
                        c.div_s(23, 22, 0);
                        call_ptr(c, 0x0014dc80, 0x0013cd98)?; // BELibrary_GetInstance
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x3;
                        call_ptr(c, 0x0014dd58, 0x0013cda4)?; // BELibrary_GetInterface
                        c.g[V1][0] = c.lw(at(c, S1, 0x18));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.lw(at(c, V1, 0xb34));
                        c.g[A1][0] = c.lw(at(c, V1, 0x86c));
                        call_ptr(c, 0x00148d80, 0x0013cdb8)?; // BECharInt_GetAttrib3Factor
                        c.lwc1(1, gpo(c, -0x7098));
                        c.abs_s(3, 21);
                        c.lwc1(2, gpo(c, -0x7094));
                        c.mul_s(3, 3, 1);
                        c.lwc1(4, at(c, S0, 0x98));
                        c.mul_s(0, 0, 2);
                        c.c_lt_s(4, 3);
                        let t = !c.fcc();
                        c.add_s(9, 0, 22);
                        if t {
                            break 'b_13ce2c;
                        } else {
                            'b_13ce1c: {
                                c.cycles += 5;
                                c.lwc1(1, at(c, S0, 0xa0));
                                c.c_lt_s(1, 3);
                                if !c.fcc() {
                                    c.lwc1(2, at(c, S0, 0x9c));
                                    break 'b_13ce1c;
                                } else {
                                    'b_13ce10: {
                                        c.cycles += 5;
                                        c.lwc1(4, at(c, S0, 0xa8));
                                        c.c_lt_s(4, 3);
                                        if !c.fcc() {
                                            c.lwc1(2, at(c, S0, 0xa4));
                                            break 'b_13ce10;
                                        } else {
                                            c.cycles += 2;
                                            c.lwc1(3, at(c, S0, 0xac));
                                            break 'b_13ce64;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.sub_s(3, 3, 1);
                                    c.lwc1(0, at(c, S0, 0xac));
                                    break 'b_13ce50;
                                }
                            }
                            c.cycles += 4;
                            c.sub_s(3, 3, 4);
                            c.lwc1(0, at(c, S0, 0xa4));
                            c.sub_s(1, 1, 4);
                            break 'b_13ce54;
                        }
                    }
                    'b_13ce48: {
                        c.cycles += 5;
                        c.lwc1(1, at(c, S0, 0x90));
                        c.c_lt_s(3, 1);
                        if !c.fcc() {
                            c.lwc1(2, at(c, S0, 0x94));
                            break 'b_13ce48;
                        } else {
                            c.cycles += 2;
                            c.lwc1(3, at(c, S0, 0x94));
                            break 'b_13ce64;
                        }
                    }
                    c.cycles += 2;
                    c.sub_s(3, 3, 1);
                    c.lwc1(0, at(c, S0, 0x9c));
                    break 'b_13ce50;
                }
                c.cycles += 1;
                c.sub_s(1, 4, 1);
                break 'b_13ce54;
            }
            c.cycles += 4;
            c.sub_s(0, 0, 2);
            c.mul_s(0, 0, 3);
            c.div_s(0, 0, 1);
            c.add_s(3, 2, 0);
            break 'b_13ce64;
        }
        'b_13ce90: {
            c.cycles += 10;
            c.lwc1(1, gpo(c, -0x7090));
            c.abs_s(0, 21);
            c.mtc1(0, 6);
            c.div_s(0, 0, 1);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 5);
            c.c_le_s(6, 0);
            let t = !c.fcc();
            c.mov_s(1, 6);
            if t {
                break 'b_13ce90;
            } else {
                c.cycles += 1;
                c.min_s(1, 0, 5);
                break 'b_13ce90;
            }
        }
        'b_13cee8: {
            'b_13cee4: {
                c.cycles += 12;
                c.sub_s(0, 5, 1);
                c.g[V0][0] = c.lw(at(c, S1, 0x18));
                c.mul_s(1, 22, 1);
                c.lwc1(4, gpo(c, -0x708c));
                c.lwc1(2, at(c, V0, 0x378));
                c.g[A0][0] = c.g[V0][0];
                c.mul_s(0, 23, 0);
                c.c_le_s(4, 2);
                c.add_s(0, 0, 1);
                c.mul_s(0, 22, 0);
                let t = !c.fcc();
                c.mul_s(3, 3, 0);
                if t {
                    break 'b_13cee4;
                } else {
                    c.cycles += 5;
                    c.lwc1(0, gpo(c, -0x7088));
                    c.c_le_s(2, 0);
                    if !c.fcc() {
                        c.mov_s(4, 5);
                        break 'b_13cee8;
                    } else {
                        c.cycles += 4;
                        c.sub_s(0, 2, 4);
                        c.div_s(4, 0, 4);
                        break 'b_13cee8;
                    }
                }
            }
            c.cycles += 1;
            c.mov_s(4, 6);
            break 'b_13cee8;
        }
        'b_13cf48: {
            'b_13cf2c: {
                c.cycles += 14;
                c.lwc1(0, gpo(c, -0x7084));
                c.lwc1(1, gpo(c, -0x7080));
                c.mul_s(0, 20, 0);
                c.g[V1][0] = c.lw(at(c, A0, 0x438));
                c.mul_s(1, 20, 1);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 8);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
                c.mul_s(0, 4, 0);
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                c.mul_s(7, 4, 1);
                c.mul_s(0, 0, 20);
                let t = c.g[V0][0] != 0u64;
                c.mul_s(6, 0, 9);
                if t {
                    break 'b_13cf2c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0xd;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.lwc1(0, at(c, A0, 0x1e8));
                        break 'b_13cf48;
                    } else {
                        break 'b_13cf2c;
                    }
                }
            }
            c.cycles += 7;
            c.lwc1(0, at(c, S1, 0x8));
            c.lwc1(1, at(c, A0, 0x454));
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 2);
            c.div_s(1, 1, 0);
            c.sub_s(8, 2, 1);
            c.lwc1(0, at(c, A0, 0x1e8));
            break 'b_13cf48;
        }
        'b_13cf70: {
            c.cycles += 9;
            c.lwc1(1, at(c, A0, 0x3d8));
            c.mtc1(0, 4);
            c.sub_s(0, 0, 1);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 12);
            c.c_lt_s(4, 0);
            let t = !c.fcc();
            c.g[V1][0] = c.lw(at(c, A0, 0x320));
            if t {
                break 'b_13cf70;
            } else {
                c.cycles += 1;
                c.lwc1(12, gpo(c, -0x707c));
                break 'b_13cf70;
            }
        }
        'b_13cf98: {
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(c, A0, 0x324));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 2);
            let t = c.g[V1][0] == c.g[V0][0];
            if t {
                break 'b_13cf98;
            } else {
                c.cycles += 4;
                c.lwc1(0, at(c, A0, 0x2fc));
                c.c_eq_s(0, 4);
                let t = !c.fcc();
                if t {
                    break 'b_13cf98;
                } else {
                    c.cycles += 1;
                    c.lwc1(2, gpo(c, -0x7078));
                    break 'b_13cf98;
                }
            }
        }
        'b_13cfe4: {
            c.cycles += 7;
            c.mtc1(0, 4);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 5);
            c.c_lt_s(4, 21);
            if !c.fcc() {
                c.neg_s(0, 21);
                break 'b_13cfe4;
            } else {
                'b_13cfe0: {
                    c.cycles += 7;
                    c.lwc1(0, at(c, A0, 0x1e8));
                    c.lwc1(1, at(c, A0, 0x3d8));
                    c.sub_s(0, 0, 1);
                    c.c_lt_s(4, 0);
                    let t = !c.fcc();
                    c.g[V0][0] = 0xd;
                    if t {
                        break 'b_13cfe0;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw(at(c, A0, 0x438));
                        if c.g[V1][0] != c.g[V0][0] {
                            c.neg_s(0, 21);
                            break 'b_13cfe4;
                        } else {
                            c.cycles += 1;
                            c.lwc1(5, gpo(c, -0x7074));
                            break 'b_13cfe0;
                        }
                    }
                }
                c.cycles += 1;
                c.neg_s(0, 21);
                break 'b_13cfe4;
            }
        }
        c.cycles += 16;
        frame.restore(c, S0, 0x20);
        c.add_s(1, 3, 6);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.mul_s(0, 0, 8);
        frame.restore_fpr(c, 23, 0x48);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        c.mul_s(0, 0, 1);
        frame.restore_fpr(c, 20, 0x30);
        c.mul_s(0, 0, 12);
        c.mul_s(0, 0, 2);
        c.sub_s(0, 0, 7);
        c.mul_s(0, 0, 5);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x13f8f8 ObjPtrArray_MergeSort

/// The sort key of an object in [`obj_ptr_array_merge_sort`] (unsigned).
pub(crate) const MERGE_SORT_KEY: u32 = 0x78;

/// `ObjPtrArray_MergeSort` itself (the recursion).
pub(crate) const MERGE_SORT_SELF: Callee = Callee {
    addr: 0x0013_f8f8,
    f: ee::s01::ObjPtrArray_MergeSort,
};

/// `void ObjPtrArray_MergeSort(Obj** first, Obj** last, Obj** buf, cmp)`
/// (0x13f8f8): stable merge sort of an object-pointer array by the
/// unsigned key at +0x78, ascending, into `buf` (which must hold `n`
/// pointers). Two elements are ordered directly; otherwise the first half
/// (`(n + 1) / 2`) is sorted into the upper half of `buf`, the second half
/// into the start of the source, and both are merged into `buf`. `cmp` is
/// only passed along.
pub fn obj_ptr_array_merge_sort(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 2;
        frame.save(c, S1, 0x80);
        frame.save(c, S7, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[S7][0] = c.g[A1][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S3, 0x60);
        c.g[V1][0] = sx(r32(c, S7).wrapping_sub(r32(c, S1)));
        frame.save(c, S6, 0x30);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x70);
        c.g[S0][0] = sx((r32(c, V1) as i32 >> 2) as u32);
        frame.save(c, S4, 0x50);
        c.g[S6][0] = c.g[A3][0];
        frame.save(c, S5, 0x40);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        let first = r32(c, S1);
        let buf = r32(c, S3);
        if c.g[S0][0] == c.g[V0][0] {
            // Two elements: order them directly into buf.
            c.cycles += 7;
            c.g[A1][0] = c.lw(first);
            c.g[A0][0] = c.lw(first.wrapping_add(4));
            c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(MERGE_SORT_KEY));
            c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(MERGE_SORT_KEY));
            c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
            if c.g[V0][0] == 0 {
                c.sw(buf, c.g[A0][0]);
                c.cycles += 3;
                c.g[V0][0] = c.lw(first);
                c.sw(buf.wrapping_add(4), c.g[V0][0]);
            } else {
                c.cycles += 4;
                c.sw(buf, c.g[A1][0]);
                c.g[V0][0] = c.lw(first.wrapping_add(4));
                c.sw(buf.wrapping_add(4), c.g[V0][0]);
            }
        } else {
            // s2 = mid = first + (n + 1) / 2, s4 = buf + n / 2, s5 = buf + n.
            c.cycles += 16;
            let n = r32(c, S0);
            c.g[V0][0] = sx(n.wrapping_add(1));
            c.g[V1][0] = sx(r32(c, V1) >> 31);
            c.g[FP][0] = c.g[V0][0];
            c.g[A0][0] = sx(r32(c, V0) >> 31);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = sx(n.wrapping_add(r32(c, V1)));
            c.g[V0][0] = sx((r32(c, V0) as i32 >> 1) as u32);
            c.g[V1][0] = sx((r32(c, V1) as i32 >> 1) as u32);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[S2][0] = sx(first.wrapping_add(r32(c, V0)));
            c.g[S4][0] = sx(buf.wrapping_add(r32(c, V1)));
            c.g[V0][0] = sx(n << 2);
            let small = (c.g[S0][0] as i64) < 3;
            c.g[V1][0] = small as u64;
            c.g[S5][0] = sx(buf.wrapping_add(r32(c, V0)));
            if small {
                c.cycles += 3;
                c.g[V0][0] = c.lw(first);
                c.sw(r32(c, S4), c.g[V0][0]);
            } else {
                // First half into buf's upper half.
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S4][0];
                c.g[A3][0] = c.g[S6][0];
                MERGE_SORT_SELF.call(c, 0x0013_f9dc)?;
            }
            c.cycles += 3;
            let small = (c.g[S0][0] as i64) < 4;
            c.g[V0][0] = small as u64;
            c.g[A0][0] = c.g[S2][0];
            if small {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, S2));
                c.sw(first, c.g[V0][0]);
            } else {
                // Second half into the start of the source.
                c.cycles += 4;
                c.g[A1][0] = c.g[S7][0];
                c.g[A3][0] = c.g[S6][0];
                c.g[A2][0] = c.g[S1][0];
                MERGE_SORT_SELF.call(c, 0x0013_fa04)?;
            }
            // Merge: a0..t1 (the source's n / 2), a3..t2 (buf's
            // (n + 1) / 2), out a2..s5.
            c.cycles += 15;
            c.g[V0][0] = sx(r32(c, FP) >> 31);
            c.g[V1][0] = sx(r32(c, S0) >> 31);
            c.g[V0][0] = sx(r32(c, FP).wrapping_add(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
            c.g[V0][0] = sx((r32(c, V0) as i32 >> 1) as u32);
            c.g[V1][0] = sx((r32(c, V1) as i32 >> 1) as u32);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[A3][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[A2][0] = c.g[S3][0];
            c.g[T2][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            let any = c.g[A2][0] < c.g[S5][0];
            c.g[V0][0] = any as u64;
            c.g[T1][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            if any {
                c.cycles += 2;
                c.g[V0][0] = (c.g[A0][0] < c.g[T1][0]) as u64;
                loop {
                    // 0x13fa48
                    c.cycles += 2;
                    let a_left = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[A3][0] < c.g[T2][0]) as u64;
                    if a_left {
                        c.cycles += 2;
                        let b_left = c.g[V0][0] != 0;
                        c.g[T0][0] = c.lw(r32(c, A0));
                        let mut took_b = false;
                        if b_left {
                            c.cycles += 6;
                            c.g[A1][0] = c.lw(r32(c, A3));
                            c.g[V1][0] = c.lw(r32(c, T0).wrapping_add(MERGE_SORT_KEY));
                            c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(MERGE_SORT_KEY));
                            c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                            if c.g[V0][0] == 0 {
                                c.sw(r32(c, A2), c.g[T0][0]);
                            } else {
                                c.cycles += 3;
                                c.sw(r32(c, A2), c.g[A1][0]);
                                c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
                                took_b = true;
                            }
                        } else {
                            c.cycles += 1;
                            c.sw(r32(c, A2), c.g[T0][0]);
                        }
                        if !took_b {
                            c.cycles += 2;
                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                        }
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, A3));
                        c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
                        c.sw(r32(c, A2), c.g[V0][0]);
                    }
                    c.cycles += 4;
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
                    let more = c.g[A2][0] < c.g[S5][0];
                    c.g[V0][0] = (c.g[A0][0] < c.g[T1][0]) as u64;
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0013_fa48);
                    }
                }
            }
        }
        c.cycles += 12;
        for (reg, off) in [
            (S0, 0x90),
            (S1, 0x80),
            (S2, 0x70),
            (S3, 0x60),
            (S4, 0x50),
            (S5, 0x40),
            (S6, 0x30),
            (S7, 0x20),
            (FP, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderGroundState::SaveState(RiderGroundState* this, Stream* s)`
/// (0x13f848): writes the state's 0x18-byte block to the replay stream.
pub fn rider_ground_state_save_state(c: &mut Cpu) {
    rider_save_state_raw(c, 0x18, 0x0013_f870);
}

/// `void RiderRailslideState_SaveState(RiderRailslideState* self, cStream*
/// stream)` (0x13c5e0): write the state's first 0x50 bytes to the stream.
pub fn rider_railslide_state_save_state(c: &mut Cpu) {
    rider_state_save_self_bytes(c, 0x50, 0x0013_c608);
}

/// `void RiderAirState_SaveState(RiderAirState* self, cPbStream* stream)`
/// (0x13acb0): write the state's 4 bytes.
pub fn rider_air_state_save_state(c: &mut Cpu) {
    guest(c, |c| rider_state_save_forward(c, 0x0013_acd8, 0x4))
}

/// Saved sizes of the handplant state and the reset sub-state.
pub(crate) const HANDPLANT_STATE_SIZE: u64 = 0xa0;

/// `void RiderHandplantState_SaveState(RiderHandplantState* self, Stream* s)` (0x139970): writes
/// the handplant state's 0xa0 bytes to the save stream.
pub fn rider_handplant_state_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = HANDPLANT_STATE_SIZE;
        // s->Write(self, 0xa0): vtable slot +0x8 `{delta, fn}`.
        let stream = r32(c, A3);
        c.g[V1][0] = Addr(stream).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(0x8).lh(c);
        c.g[V0][0] = Addr(vt).off(0xc).lw(c);
        let write = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, write, 0x0013_9998)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderAirState_PlayLandAnim2 (0x13a968)

/// gp-relative literals: the hard-landing speed (-2222.22), pi and the
/// medium-landing speed (-1250.0).
pub(crate) const LAND2_HARD_SPEED_GP: u32 = 0x719c;

pub(crate) const LAND2_PI_GP: u32 = 0x7198;

pub(crate) const LAND2_MEDIUM_SPEED_GP: u32 = 0x7194;

/// Air state -> its rider.
pub(crate) const LAND2_RIDER: u32 = 0x4;

/// Rider -> its anim controller.
pub(crate) const LAND2_RIDER_ANIM: u32 = 0x784;

/// Rider -> the landing spin angle (radians).
pub(crate) const LAND2_RIDER_SPIN: u32 = 0x2dc;

/// `AnimController_PlayAnim` (through the function table).
pub(crate) const LAND2_PLAY_ANIM: u32 = 0x0031_28e8;

/// `void RiderAirState::PlayLandAnim2(float vy)` (0x13a968): a very hard
/// landing (`vy < -2222.2`) plays anim 0x3f and clears the spin angle;
/// otherwise an over-rotated spin plays 0x42 (< -pi) or 0x43 (> pi), and
/// a straight landing 0x3e (`vy < -1250`) or 0x3d. Each through
/// `AnimController_PlayAnim(anim, id, 0, -1.0)`. Frame 0x20 (s0).
pub fn rider_air_state_play_land_anim2(c: &mut Cpu) {
    use crate::abi::set_fcc;
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 7;
        c.f[0] = land2_gp_lit(c, LAND2_HARD_SPEED_GP);
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let hard = FpuF32(c.f[12]).lt(FpuF32(c.f[0]));
        set_fcc(c, hard);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if hard {
            c.cycles += 10;
            let state = c.g[S0][0] as u32;
            c.g[V0][0] = Addr(state.wrapping_add(LAND2_RIDER)).lw(c);
            c.g[A1][0] = 0x3f;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.f[12] = 0xbf80_0000;
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(LAND2_RIDER_ANIM)).lw(c);
            call_ptr(c, LAND2_PLAY_ANIM, 0x0013_a9a0)?;
            let state = c.g[S0][0] as u32;
            c.g[V1][0] = Addr(state.wrapping_add(LAND2_RIDER)).lw(c);
            Addr((c.g[V1][0] as u32).wrapping_add(LAND2_RIDER_SPIN)).sw(c, 0);
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            // Pick the anim id; `None` is the plain landing 0x3d, which
            // loads the controller after the decision (0x13aa1c).
            c.cycles += 8;
            c.f[2] = land2_gp_lit(c, LAND2_PI_GP);
            let state = c.g[S0][0] as u32;
            c.g[A0][0] = Addr(state.wrapping_add(LAND2_RIDER)).lw(c);
            let pi = FpuF32(c.f[2]);
            c.f[0] = pi.0 ^ 0x8000_0000;
            let rider = c.g[A0][0] as u32;
            c.f[1] = Addr(rider.wrapping_add(LAND2_RIDER_SPIN)).read_u32(c);
            let spin = FpuF32(c.f[1]);
            let under = spin.lt(FpuF32(c.f[0]));
            set_fcc(c, under);
            c.g[A1][0] = 0x42;
            let chosen = if under {
                c.cycles += 2;
                true
            } else {
                c.cycles += 4;
                let over = pi.lt(spin);
                set_fcc(c, over);
                c.g[A1][0] = 0x43;
                if over {
                    c.cycles += 2;
                    true
                } else {
                    c.cycles += 5;
                    c.f[0] = land2_gp_lit(c, LAND2_MEDIUM_SPEED_GP);
                    let medium = FpuF32(c.f[12]).lt(FpuF32(c.f[0]));
                    set_fcc(c, medium);
                    c.g[A1][0] = 0x3e;
                    if medium {
                        c.cycles += 1;
                    }
                    medium
                }
            };
            if chosen {
                let rider = c.g[A0][0] as u32;
                c.g[A0][0] = Addr(rider.wrapping_add(LAND2_RIDER_ANIM)).lw(c);
                c.cycles += 6;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.f[12] = 0xbf80_0000;
                c.g[A2][0] = 0;
                call_ptr(c, LAND2_PLAY_ANIM, 0x0013_aa14)?;
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 6;
                let rider = c.g[A0][0] as u32;
                c.g[A0][0] = Addr(rider.wrapping_add(LAND2_RIDER_ANIM)).lw(c);
                c.g[A1][0] = 0x3d;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.f[12] = 0xbf80_0000;
                c.g[A2][0] = 0;
                call_ptr(c, LAND2_PLAY_ANIM, 0x0013_aa34)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lwc1` of the gp-relative literal at `gp - off`.
#[inline(always)]
pub(crate) fn land2_gp_lit(c: &mut Cpu, off: u32) -> u32 {
    let gp = c.g[GP][0] as u32;
    Addr(gp.wrapping_sub(off)).read_u32(c)
}

#[cfg(test)]
pub(crate) mod tests {
    #[test]
    fn yaw_align_gain_keeps_the_stock_share_per_sixtieth() {
        for hz in [60, 120, 240] {
            for gain in [0.05f32, 0.1, 0.5, 0.9] {
                let mut error = 1.0f32;
                for _ in 0..hz / 60 {
                    error -= super::yaw_align_gain(hz, gain) * error;
                }
                assert!((error - (1.0 - gain)).abs() < 1e-6, "{hz} {gain}: {error}");
            }
            for gain in [0.0f32, 1.0, -0.5, 9.7] {
                assert_eq!(super::yaw_align_gain(hz, gain).to_bits(), gain.to_bits());
            }
        }
    }

    use super::*;
    use crate::abi;
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
    use crate::difftest::SP as STACK;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng as rng_3;
    pub(crate) use crate::game::ai::ai::tests::slot;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::ai::tests::LOAD_B0;
    pub(crate) use crate::game::ai::computer::tests::literals;
    pub(crate) use crate::game::ai::rider::tests::f;
    pub(crate) use crate::game::ai::rider::tests::fake_varied;
    pub(crate) use crate::game::ai::rider::tests::gp_f32;
    pub(crate) use crate::game::ai::rider::tests::put_contacts;
    pub(crate) use crate::game::ai::rider::tests::put_list;
    pub(crate) use crate::game::ai::rider::tests::put_rider;
    pub(crate) use crate::game::ai::rider::tests::put_slot;
    pub(crate) use crate::game::ai::rider::tests::put_v;
    pub(crate) use crate::game::ai::rider::tests::rand_v;
    pub(crate) use crate::game::ai::rider::tests::setup_machine;
    pub(crate) use crate::game::ai::rider::tests::uf_2 as uf;
    pub(crate) use crate::game::ai::rider::tests::BASE;
    pub(crate) use crate::game::ai::rider::tests::INSTS;
    pub(crate) use crate::game::ai::rider::tests::LIST;
    pub(crate) use crate::game::ai::rider::tests::MATS;
    pub(crate) use crate::game::ai::rider::tests::OBJS;
    pub(crate) use crate::game::ai::rider::tests::OWNER;
    pub(crate) use crate::game::ai::rider::tests::PARAMS;
    pub(crate) use crate::game::ai::rider::tests::RIDER_2 as RIDER;
    pub(crate) use crate::game::ai::rider::tests::SEEDS;
    pub(crate) use crate::game::ai::rider::tests::SURFS;
    pub(crate) use crate::game::ai::rider::tests::TREE;
    pub(crate) use crate::game::ai::rider::tests::VT_HULL;
    pub(crate) use crate::game::ai::rider::tests::VT_OWNER;
    pub(crate) use crate::game::ai::rider::tests::VT_SUB;
    pub(crate) use crate::game::ai::rider::tests::VT_VOL;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::fe::festateoptions::tests::rng as rng_2;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_3;
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

    // -----------------------------------------------------------------
    // Ground probe.

    #[test]
    fn rider_ground_state_probe_ground_matches() {
        check(
            ee::s01::f_0013d1b8__generated,
            rider_ground_state_probe_ground,
            SEEDS,
            &|c, seed| {
                let mut r = setup_machine(c, seed, 0xd1b8, 6);
                put_rider(c, &mut r, 0, 0);
                put_list(c, seed % 4);
                // The probe's contact records (its frame at sp_2 - 0x2190).
                put_contacts(c, &mut r, STACK - 0x2190 + 0x100, 10);
                gp_f32(c, -0x7068, uf(&mut r, 0.3, 0.9));
                let state = DATA + 0x200;
                c.bus.write32(state + 0x18, RIDER);
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x500);
            },
        );
    }

    #[test]
    fn rider_ground_state_compute_carve_accel_matches() {
        check(
            ssx3_ee::s01::f_0013d028__generated,
            rider_ground_state_compute_carve_accel,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0013_d028, 0);
                literals(c);
                let ground = DATA;
                let rider = DATA + 0x200;
                let v = DATA + 0x300;
                c.g[A0][0] = ground as u64;
                c.g[A1][0] = v as u64;
                c.bus.write32(ground + computer::RIDER, rider);
                // The edging stat chain (`BECharInt_GetAttrib3Factor_2`): the
                // player's record has a profile, character 0, no fixed flag.
                c.bus.write32(rider + rider::PLAYER, 0);
                c.bus
                    .write32(rider + rider::ATTRIB_VALUE, r.next_u32() % 400);
                c.bus.write32(
                    rider + rider::SPEED_FACTOR,
                    if seed % 3 == 0 { 0 } else { r.edge_f32() },
                );
                c.bus.write8(0x0053_5538 + 3, (r.next_u32() % 250) as u8);
                c.bus
                    .write8(0x0053_08d8 + 8 + 3, 1 + (r.next_u32() % 250) as u8);
                put_vec(c, v, [r.f32(10.0), r.f32(10.0), r.f32(10.0), r.f32(1.0)]);
                // The speed curve: four ascending points.
                let curve = DATA + 0x800;
                c.bus.write32(SPEED_CURVE, curve);
                let mut x = 0i32;
                for i in 0..4u32 {
                    x += 1 + (r.next_u32() % 20) as i32;
                    c.bus.write32(curve + 8 * i, x as u32);
                    c.bus.write32(curve + 8 * i + 4, r.f32(1000.0));
                }
                // The back-end singleton and the interface's vtable (a fake: the
                // interface value is not used by the attrib chain).
                c.bus.write32(GAME_GP - global::BE_LIBRARY, DATA + 0x600);
                c.bus.write32(GAME_GP - global::BE_LIBRARY_INDEX, 0);
                c.bus.write32(BE_PHASE_INTERFACES, DATA + 0x700);
                c.bus.write16(DATA + 0x908, 0); // vtable slot 1's delta (at vt+8)
                c.bus.write32(DATA + 0x90c, fakes::FAKE); // ... and its function
                c.bus.write32(DATA + 0x704, DATA + 0x900); // lib->vtable
                                                           // Arguments: a, b, c in f12-f14.
                c.f[12] = if seed % 4 == 0 {
                    r.edge_f32() | 0x8000_0000
                } else {
                    r.edge_f32()
                };
                c.f[13] = r.edge_f32();
                c.f[14] = match seed % 4 {
                    0 => r.edge_f32() | 0x8000_0000,
                    1 => 0,
                    _ => r.edge_f32(),
                };
            },
        );
    }

    #[test]
    fn flags64_test_matches() {
        check(
            ssx3_ee::s01::f_001446a0__generated,
            flags64_test,
            200,
            &|c, seed| {
                let mut r = Rng(0x51ed_2701 ^ seed as u64);
                scramble(c, &mut r);
                let at = DATA + (seed % 8) * 0x10;
                c.g[A0][0] = sx(at);
                c.bus
                    .write64(at, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                // 0..63, plus out-of-range shifts (masked to 0..63).
                c.g[A1][0] = if seed % 5 == 0 {
                    64 + (seed % 9) as u64
                } else {
                    (r.next_u32() % 64) as u64
                };
            },
        );
    }

    pub(crate) const SEEDS_2: u32 = 200;

    /// `RiderGroundState_ComputeFrictionAccel` publishes a slip value and
    /// leaves caller-saved FPU registers that the race's caller may spill, so
    /// compare the soft registers as well, over varied `m4`/`m8`/`p` and every
    /// branch — including both sides of the deep-slip `max(-m8, a)` clamp.
    #[test]
    fn rider_ground_state_compute_friction_accel_matches() {
        for seed in 0..crate::difftest::seed_count(SEEDS_2) {
            let r = crate::difftest::compare(
                ee::s01::f_0013c878__generated,
                rider_ground_state_compute_friction_accel,
                &|c| friction_setup(c, seed),
            );
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} (soft): {:#?}", r.soft);
        }
    }

    pub(crate) fn friction_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0xc878, 0);
        let gs = DATA + 0x1000;
        let p = DATA + 0x1100;
        c.g[A0][0] = gs as u64;
        c.g[A1][0] = p as u64;
        let m4 = [0.25f32, 0.5, 1.0][(seed % 3) as usize];
        let m8 = m4 + [1.0f32, 2.5, 4.0][((seed / 3) % 3) as usize];
        let p0 = [0.25f32, -0.5, 1.5][(seed % 3) as usize];
        let p7 = [0.75f32, 0.125, -0.5][((seed / 2) % 3) as usize];
        c.bus.write32(gs + 0x4, m4.to_bits()); // m4
        c.bus.write32(gs + 0x8, m8.to_bits()); // m8
        c.bus.write32(gs + 0x18, DATA + 0x1200); // mRider
        c.bus.write32(p, p0.to_bits()); // p[0]
        c.bus.write32(p + 0x1c, p7.to_bits()); // p[7]
        c.bus
            .write32(GAME_GP.wrapping_sub(0x70e8), (-1.0f32 / 30.0).to_bits());
        let (a, b): (f32, f32) = match seed % 5 {
            0 => (2.0 * m4, 0.5),   // a > 0, b > 0
            1 => (2.0 * m4, -0.5),  // a > 0, b <= 0
            2 => (-0.5 * m4, 0.5),  // -m4 < a <= 0
            3 => (-1.5 * m4, -0.5), // a <= -m4, clamp inactive
            _ => (-3.0 * m8, 1.0),  // a <= -m8, clamp to -m8
        };
        c.f[12] = a.to_bits();
        c.f[13] = b.to_bits();
    }

    #[test]
    fn rider_is_control_mode3_matches() {
        check(
            ssx3_ee::s01::f_001448a8__generated,
            rider_is_control_mode3,
            300,
            &|c, seed| {
                let mut r = Rng(0x1234_5678 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                // 3 and not-3, plus values whose sign extension matters.
                let mode = match seed % 4 {
                    0 => 3,
                    1 => 0,
                    2 => r.next_u32(),
                    _ => 0x8000_0003,
                };
                c.bus.write32(DATA + 0x70, mode);
            },
        );
    }

    #[test]
    fn rider_air_state_update_matches() {
        check(
            ee::s01::f_00139a20__generated,
            rider_air_state_update,
            160,
            &|c, seed| {
                let mut r = Rng(0x139a_20 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA;
                let rider = DATA + 0x100;
                let anim = DATA + 0x8000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + air::RIDER, rider);
                c.bus.write32(this, 0);
                for k in 0..4 {
                    c.bus.write32(rider + air::UP + k * 4, r.f32(1.0));
                    c.bus.write32(rider + air::ORIENT + k * 4, r.f32(1.0));
                }
                c.bus.write32(rider + air::DT, 0.016f32.to_bits());
                c.bus.write32(rider + air::ANIM, anim);
                c.bus.write32(rider + air::SUB, DATA + 0x500);
                c.bus.write32(rider + air::WIND, 0);
                let angle = if seed % 4 == 2 { 0.5f32 } else { 0.001f32 };
                c.bus.write32(DATA + 0x500 + anim::ANGLE, angle.to_bits());
                // world -> +0x84 -> +0x44: the race manager's section records.
                let world = DATA + 0x600;
                let mgr = DATA + 0x620;
                c.bus.write32((GAME_GP as i32 + super::WORLD) as u32, world);
                c.bus.write32(world + 0x84, mgr);
                c.bus.write32(mgr + SECTIONS, DATA + 0x640);
                c.bus.write32(DATA + 0x640 + 0xb0 + SECTIONS, 0);
                // Animation / predictor state.
                let state = match seed % 4 {
                    0 => 2u32,
                    1 => 3,
                    2 => 1,
                    _ => 3,
                };
                c.bus.write32(anim + anim::STATE, state);
                c.bus.write32(anim + anim::SECTION, 1);
                c.bus.write16(anim + anim::FLAGS, 8);
                c.bus.write32(anim + anim::T0, 2.0f32.to_bits());
                c.bus.write32(anim + anim::T1, 0.0f32.to_bits());
                c.bus.write32(anim + 0x58, 0.0f32.to_bits());
                c.bus.write32(anim + 0xa4, 0.0f32.to_bits());
                c.bus.write32(anim + 0xa8, 1.0f32.to_bits());
                let thin = if seed % 8 == 3 { 0.5f32 } else { 0.1f32 };
                c.bus.write32(anim + anim::THIN, thin.to_bits());
                for k in 0..4 {
                    c.bus.write32(anim + anim::ORIENT + k * 4, r.f32(1.0));
                }
                let sub = if seed % 4 == 2 { 5u32 } else { 0 };
                c.bus.write32(DATA + 0x500 + 0xde4, sub);
                // The float constants the state reads through gp_5.
                c.bus
                    .write32((GAME_GP as i32 - 0x71d4) as u32, (1.0f32 / 60.0).to_bits());
                c.bus
                    .write32((GAME_GP as i32 - 0x71d0) as u32, (1.0f32 / 60.0).to_bits());
                c.bus
                    .write32((GAME_GP as i32 - 0x71cc) as u32, 60.0f32.to_bits());
                c.bus
                    .write32((GAME_GP as i32 - 0x71c8) as u32, 1.396f32.to_bits());
                c.bus
                    .write32((GAME_GP as i32 - 0x71c4) as u32, 0.0872f32.to_bits());
                c.bus
                    .write32((GAME_GP as i32 - 0x71c0) as u32, 0.3f32.to_bits());
                // The predictor's internal step budget and rates.
                c.bus
                    .write32((GAME_GP as i32 - 0x7c5c) as u32, (1.0f32 / 60.0).to_bits());
                c.bus
                    .write32((GAME_GP as i32 - 0x7c74) as u32, 2.0f32.to_bits());
                c.bus
                    .write32((GAME_GP as i32 - 0x7c70) as u32, 1.0f32.to_bits());
                c.bus
                    .write32((GAME_GP as i32 - 0x7c6c) as u32, 1.0f32.to_bits());
                c.bus
                    .write32((GAME_GP as i32 - 0x7c60) as u32, 0.0f32.to_bits());
            },
        );
    }

    /// `RiderAirState_Update`'s last-partial-tick turn through the real
    /// `Rider_UpdateOrientation` (0x121aa0), which turns by `min(angle * gain,
    /// cap) * step`: the whole remaining turn at 60/120/240 when the cap does
    /// not bind, the same turn per second when it does, and the original
    /// multiply at 60. A gain of 1.0 would turn 1/rate of the angle per tick.
    #[test]
    fn rider_air_state_last_turn_follows_the_rate() {
        use crate::tick;
        const R: u32 = 0x0080_0000;
        const TARGET: u32 = R + 0x1000;
        // The game's value at gp-0x71cc: 59.999996 (0x426fffff), the
        // reciprocal of the 1/60 bits.
        let fallback = FpuF32(0x426f_ffff);
        assert!((fallback.to_f32() - 59.999_996f32).abs() < 1e-4);
        // The heading after `ticks` last-partial turns towards a target 0.05
        // rad away, at time scale 1.0 and angular cap `cap` (rad/s).
        let turn = |hz: u32, cap: f32, ticks: u32| {
            let mut c = crate::difftest::fresh();
            elf_image(&mut c);
            c.g[GP][0] = GAME_GP as u64;
            tick::set_for_test(&mut c, hz);
            // The consumer's `.lit4` step.
            c.bus.write32(0x0049_b828, tick::step_bits(hz));
            // Identity orientation (quaternion at +0x120).
            c.bus.write32(R + 0x12c, 1.0f32.to_bits());
            for (i, v) in [0.05f32.sin(), 0.0, 0.05f32.cos(), 0.0]
                .into_iter()
                .enumerate()
            {
                c.bus.write32(TARGET + i as u32 * 4, v.to_bits());
            }
            for _ in 0..ticks {
                c.g[A0][0] = R as u64;
                c.g[A1][0] = TARGET as u64;
                // The caller's default up vector (0x4ff120).
                c.g[A2][0] = 0x004f_f120;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.f[12] = air_last_turn_fraction(&mut c, FpuF32::ONE, fallback).0;
                c.f[13] = cap.to_bits();
                s00::f_00121aa0(&mut c);
                assert_eq!(c.pc, crate::difftest::RA);
            }
            let x = f32::from_bits(c.bus.read32(R + 0x1c0));
            let z = f32::from_bits(c.bus.read32(R + 0x1c8));
            x.atan2(z).abs()
        };
        // The cap does not bind: one tick lands the whole turn.
        for hz in [60, 120, 240] {
            let heading = turn(hz, 100.0, 1);
            assert!((heading - 0.05).abs() < 0.0002, "{hz} Hz: {heading}");
        }
        // The game's cap (gp-0x71c8, 80°/s) binds: two ticks at 120 turn as
        // far as one at 60, short of the target.
        let at_60 = turn(60, 1.3962634, 1);
        let at_120 = turn(120, 1.3962634, 2);
        assert!(
            (at_120 - at_60).abs() < 0.0002,
            "capped: {at_120} at 120 Hz, {at_60} at 60"
        );
        assert!(at_120 < 0.03);
        // At 60 the original multiply, at any time scale.
        let mut c = crate::difftest::fresh();
        tick::set_for_test(&mut c, 60);
        for s in [0.5f32, 1.0, 1.15] {
            let s = FpuF32::from_f32(s);
            assert_eq!(air_last_turn_fraction(&mut c, s, fallback), s * fallback);
        }
    }

    #[test]
    fn rider_air_state_cast_board_ray_matches() {
        check(
            ee::s01::f_0013a7b0__generated,
            rider_air_state_cast_board_ray,
            120,
            &|c, seed| {
                let mut r = Rng(0x13a7_b0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA;
                let rider = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x600);
                c.bus.write32(this + 0x4, rider);
                c.bus.write32(rider + 0x780, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x2c, DATA + 0x300);
                c.bus.write32(rider + 0x8a0, 0);
                c.bus.write32(rider + 0x860, 0);
                c.bus.write32(rider + 0x864, 0x40);
                for k in 0..4 {
                    c.bus.write32(rider + 0x180 + k * 4, r.f32(1.0));
                    c.bus.write32(DATA + 0x300 + k * 4, r.f32(10.0));
                }
                c.bus
                    .write32((GAME_GP as i32 - 0x71a4) as u32, 0.575f32.to_bits());
            },
        );
    }

    #[test]
    fn flags64_test_second_matches() {
        check(
            ee::s01::f_001446b8__generated,
            flags64_test_second,
            SEEDS_3,
            &|c, seed| {
                let mut r = rng(0x0014_46b8, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let flags = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.bus.write64(DATA + 8, flags);
                c.g[A1][0] = sx((seed % 70) as u32);
            },
        );
    }

    #[test]
    fn flags64_move_to_second_matches() {
        check(
            s01::f_00144670__generated,
            flags64_move_to_second,
            200,
            &|c, seed| {
                let mut r = Rng(0x1446_70aa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(seed);
                c.bus
                    .write64(OBJ, r.next_u32() as u64 | (r.next_u32() as u64) << 32);
                c.bus
                    .write64(OBJ + 8, r.next_u32() as u64 | (r.next_u32() as u64) << 32);
            },
        );
    }

    pub(crate) fn put(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    // ------------------------------------------------------------------
    // State predicates.

    #[test]
    fn rider_is_computer_matches() {
        check(
            ee::s01::f_00140bc8__generated,
            rider_is_computer,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 18);
                c.g[A0][0] = AUX as u64;
                put(
                    c,
                    AUX + 0x874,
                    if seed % 3 == 0 {
                        0
                    } else if seed % 3 == 1 {
                        1
                    } else {
                        r.next_u32()
                    },
                );
            },
        );
    }

    #[test]
    fn rider_air_state_exit_matches() {
        leaf(
            ssx3_ee::s01::f_00139a18__generated,
            rider_air_state_exit,
            11,
        );
    }

    /// The rider.
    pub(crate) const RIDER_2: u32 = DATA + 0x1000;

    /// The air state: `rider + 0x20`, the way `RiderBase_PostUpdateMainState`
    /// passes it (`off: 0x20`, see `rider_logic.rs`), with the rider back at
    /// `+0x4`.
    pub(crate) const MOTION: u32 = RIDER_2 + 0x20;

    /// The rider's model (`CastBoardRay` reads its transform base).
    pub(crate) const MODEL: u32 = DATA + 0x2000;

    /// The board transform the ray starts from.
    pub(crate) const BOARD: u32 = DATA + 0x4000;

    /// The per-seed control word the fake board ray reads.
    pub(crate) const SEEDWORD: u32 = DATA + 0x5000;

    /// The rider's wall-collision object.
    pub(crate) const WALLS: u32 = DATA + 0x6000;

    /// The rider state object (`rider->+0x77c`).
    pub(crate) const STATE: u32 = DATA + 0x7000;

    /// The trigger-entry array (`module->+0x44`, 0xb0 bytes per entry).
    pub(crate) const TRIGGERS: u32 = DATA + 0x9000;

    /// The animation controller dummy (`rider->+0x784`).
    pub(crate) const ANIM_OBJ: u32 = DATA + 0xa000;

    /// A zeroed collision object (`rider + 0x860`): no level geometry, so the
    /// real board ray misses and returns -1.
    pub(crate) const COLL: u32 = DATA + 0xb000;

    /// The trigger object the ray reports at `frame + 0x50`, its descriptor
    /// (`+0xc`) and the callback object (`descriptor + 0xc`).
    pub(crate) const TRIG: u32 = DATA + 0xc000;

    pub(crate) const TRIG_DESC: u32 = DATA + 0xc100;

    pub(crate) const TRIG_SUB: u32 = DATA + 0xc200;

    /// The animation object the ray reports at `frame + 0x54`.
    pub(crate) const ANIM: u32 = DATA + 0xd000;

    /// The zeroed BELibrary dummy.
    pub(crate) const BE: u32 = DATA + 0x3_0000;

    /// The dummy world (no geometry, no riders).
    pub(crate) const WORLD: u32 = DATA + 0x8_0000;

    pub(crate) fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// `difftest::check`, but requiring the caller-saved scratch state and
    /// LO/HI to match as well (`check` only fails on the ABI-preserved half).
    /// The game's hand-written callees may read registers the ABI lets them
    /// ignore, and a fake callee logs every register.
    pub(crate) fn check_strict(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..seed_count(seeds) {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(
                r.soft.is_empty(),
                "seed {seed} (scratch state): {:#?}",
                r.soft
            );
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }

    /// The BE singleton, the world with its module and race manager (no
    /// riders), and the wall flag `RiderAirState_CollideWalls` writes.
    pub(crate) fn globals(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        w(c, WORLD + 0x84, WORLD + 0x1000);
        w(c, WORLD + 0x1000 + 0xc, WORLD + 0x2000);
        w(c, WORLD + 0x2000 + 0x7c, 0);
        w(c, GAME_GP.wrapping_sub(0x1fd0), 0x10);
    }

    /// The rider with its air state, model and board, and the objects the arm
    /// dereferences (all zeroed dummies, never raw addresses).
    pub(crate) fn rider_fixture(c: &mut Cpu, r: &mut Rng) {
        c.g[A0][0] = MOTION as u64;
        w(c, MOTION + 4, RIDER_2);
        w(c, RIDER_2 + 0x780, MODEL);
        w(c, MODEL + 0x2c, BOARD);
        w(c, RIDER_2 + 0x8a0, 0);
        w(c, RIDER_2 + 0x784, ANIM_OBJ);
        w(c, RIDER_2 + 0x788, 0);
        w(c, RIDER_2 + 0xaa0, WALLS);
        w(c, RIDER_2 + 0x77c, STATE);
        w(c, RIDER_2 + 0x330, (r.next_u32() % 3) as u32);
        w(c, RIDER_2 + 0x378, r.f32(100.0));
        w(c, RIDER_2 + 0x86c, r.next_u32());
        w(c, RIDER_2 + 0xb34, r.next_u32());
        // The board contact's trigger lookup: `module->+0x44` is the array and
        // `frame->+0x4c` the index the fake writes.
        w(c, WORLD + 0x1000 + 0x44, TRIGGERS);
        w(c, WORLD + 0x10, r.edge_f32());
        // The trigger object chain (only used when the fake reports one).
        w(c, TRIG + 0xc, TRIG_DESC);
        w(c, TRIG_DESC + 0xc, TRIG_SUB);
        c.bus.write16(TRIG_SUB + 0x158, 0x20);
        w(c, TRIG_SUB + 0x15c, FAKE);
        // The animation object's flag word (`lh(+0xa)`).
        c.bus.write16(ANIM + 0xa, (r.next_u32() % 2) as u16 * 2);
        w(c, TRIGGERS + 0x18, r.edge_f32());
        w(c, TRIGGERS + 0x3c, r.edge_f32());
        w(c, TRIGGERS + 0x40, r.edge_f32());
        let q = |r: &mut Rng| [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0x3f80_0000];
        put_vec(c, RIDER_2 + 0x110, q(r));
        put_vec(c, RIDER_2 + 0x1a0, q(r));
        put_vec(c, RIDER_2 + 0x1b0, q(r));
        put_vec(c, RIDER_2 + 0x1c0, q(r));
        put_vec(c, RIDER_2 + 0x1e0, q(r));
        put_vec(c, RIDER_2 + 0x370, q(r));
        put_vec(c, BOARD, q(r));
    }

    /// The air path: `RiderAirState_CastBoardRay` runs for real against a
    /// zeroed collision object, finds no geometry, returns -1, and the body
    /// goes straight to the bounds/wall/camera tail.
    pub(crate) fn setup_air(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x139c_88, 6);
        globals(c);
        rider_fixture(c, &mut r);
        // The real ray: `model->+0x2c + (rider->+0x8a0 << 5)` is the board
        // transform, `+0x860` the collision object, `+0x864` the flags.
        w(c, RIDER_2 + 0x860, COLL);
        w(c, RIDER_2 + 0x864, 0);
        // Sub-state 9 makes `RiderAirState_CollideWalls` return at once (its
        // real body needs level geometry).
        w(c, STATE + 0xde4, 9);
        // No trigger/animation object in the ray's out struct.
        w(c, SEEDWORD, 0x1);
    }

    /// A stand-in for `RiderAirState_CastBoardRay` (0x13a7b0), driven by the
    /// fixture's [`SEEDWORD`]: bit 0 picks a miss (`f0 = -1`, the real
    /// function's answer without geometry) or a hit at 10 units, bit 1 the
    /// side of the ray the hit lies on (so the arm's `(velocity - hit) .
    /// direction` test takes both sides), bit 2 the trigger object at
    /// `frame+0x50` and bit 3 the animation object at `frame+0x54`; bits 8-9
    /// are the trigger-entry index.
    pub(crate) fn fake_cast_board_ray(c: &mut Cpu) {
        let v = Addr(SEEDWORD).lw(c) as u32;
        let out = abi::arg(c, 1);
        put_vec(c, out, [0, 0, 0, 0x3f80_0000]);
        put_vec(
            c,
            p(out, 0x10),
            [0, 0, if v & 2 == 0 { 0xbf80_0000 } else { 0x3f80_0000 }, 0],
        );
        put_vec(c, p(out, 0x20), [0, 0, 0xc2c8_0000, 0]);
        w(c, p(out, 0x4c), (v >> 8) & 3);
        w(c, p(out, 0x50), if v & 4 == 0 { 0 } else { TRIG });
        w(c, p(out, 0x54), if v & 8 == 0 { 0 } else { ANIM });
        // The distance: 10.0 for a hit, -1.0 for a miss.
        c.f[0] = if v & 1 == 0 { 0x4120_0000 } else { 0xbf80_0000 };
        c.g[V0][0] = 0;
        abi::ret(c);
    }

    /// Puts the `CastBoardRay` override slot back (the generated body is its
    /// default: the game's own replacement is installed by
    /// `ssx3_game::install`, which the tests do not run).
    pub(crate) struct RaySlot;

    impl Drop for RaySlot {
        fn drop(&mut self) {
            unsafe {
                ssx3_ee::s01::OVERRIDE_0013A7B0 = ssx3_ee::s01::f_0013a7b0__generated;
            }
        }
    }

    /// The board-contact arm, which the real function cannot reach in a test
    /// machine (no level geometry): the fake reports a hit and the objects the
    /// arm reads. The sub-state picks the landing chain's branch, and the
    /// control word the arm's side, trigger and animation objects.
    pub(crate) fn setup_board_contact(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x139c_88, 12);
        globals(c);
        rider_fixture(c, &mut r);
        // Bit 0 clear: a hit. Bit 1 varies the arm's side test, bits 2/3 the
        // trigger and animation objects, bits 8-9 the trigger entry.
        w(c, SEEDWORD, (seed % 8) * 2 | ((seed % 4) << 8));
        // The air sub-state: 0xc skips the landing chain, 2 takes the
        // `Rider_ResetGrindOrientation` (0x115640) branch, 9 the 0x13a644 one.
        w(c, STATE + 0xde4, [9, 9, 9, 2][(seed % 4) as usize]);
        // A non-zero `entry->+0x44` takes the `Rider_CheckResetPath` early
        // return (the arm's only `break 'func`).
        w(
            c,
            TRIGGERS + 0xb0 * (seed % 4) + 0x44,
            if seed % 7 == 0 { 1 } else { 0 },
        );
        w(c, RIDER_2 + 0x864, 0);
    }

    /// The same arm with every field it reads randomized (pointers stay on
    /// zeroed dummies), and the fake's hit/miss and object selection random:
    /// a sweep of the data-dependent branches.
    pub(crate) fn setup_fuzz(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x139c_f8, 10);
        globals(c);
        rider_fixture(c, &mut r);
        w(c, SEEDWORD, r.next_u32());
        w(
            c,
            STATE + 0xde4,
            [0, 1, 2, 5, 9, 0xc][(r.next_u32() % 6) as usize],
        );
        w(c, RIDER_2 + 0x8a0, r.next_u32() % 4);
        w(c, RIDER_2 + 0x860, COLL);
        w(c, RIDER_2 + 0x864, r.next_u32() & 3);
        for k in 0..4u32 {
            w(c, TRIGGERS + 0xb0 * k + 0x44, r.next_u32() % 2);
        }
        // The quadwords the arm consumes (random edge-case floats).
        let q = |r: &mut Rng| [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
        put_vec(c, RIDER_2 + 0x110, q(&mut r));
        put_vec(c, RIDER_2 + 0x1a0, q(&mut r));
        put_vec(c, RIDER_2 + 0x1b0, q(&mut r));
        put_vec(c, RIDER_2 + 0x1c0, q(&mut r));
        put_vec(c, RIDER_2 + 0x1e0, q(&mut r));
        put_vec(c, RIDER_2 + 0x370, q(&mut r));
        put_vec(c, RIDER_2 + 0x400, q(&mut r));
        put_vec(c, BOARD, q(&mut r));
        w(c, RIDER_2 + 0x378, r.edge_f32());
    }

    #[test]
    fn rider_air_state_post_update_matches() {
        check_strict(
            ssx3_ee::s01::f_00139c88__generated,
            rider_air_state_post_update,
            24,
            &|c, seed| setup_air(c, seed),
        );
    }

    /// The board-contact arm: the ray hit, the trigger callback, the
    /// animation-object and landing-chain branches, all driven by the fake.
    #[test]
    fn rider_air_state_post_update_board_contact_matches() {
        let _slot = RaySlot;
        unsafe {
            ssx3_ee::s01::OVERRIDE_0013A7B0 = fake_cast_board_ray;
        }
        check_strict(
            ssx3_ee::s01::f_00139c88__generated,
            rider_air_state_post_update,
            32,
            &|c, seed| setup_board_contact(c, seed),
        );
    }

    /// A randomized sweep over everything the function reads.
    #[test]
    fn rider_air_state_post_update_fuzz_matches() {
        let _slot = RaySlot;
        unsafe {
            ssx3_ee::s01::OVERRIDE_0013A7B0 = fake_cast_board_ray;
        }
        check_strict(
            ssx3_ee::s01::f_00139c88__generated,
            rider_air_state_post_update,
            40,
            &|c, seed| setup_fuzz(c, seed),
        );
    }

    #[test]
    fn rider_air_state_play_land_anim_matches() {
        check(
            ssx3_ee::s01::f_0013a8f8__generated,
            rider_air_state_play_land_anim,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x13a8f8, 0);
                let rider = DATA + 0x1000;
                let model = DATA + 0x3000;
                let anim = DATA + 0x4000;
                c.g[A0][0] = rider as u64;
                w32(c, GAME_GP.wrapping_sub(0x71a0), (-2222.222_2f32).to_bits());
                c.f[12] = if seed % 2 == 0 {
                    (-2222.3f32 + (seed % 5) as f32).to_bits()
                } else {
                    r.f32(3000.0)
                };
                w32(c, rider + 4, model);
                w32(c, model + 0x784, anim);
                let vt = DATA + 0x5000;
                w32(c, anim + 0x58, vt);
                slot(c, vt, 0x20, 0, LOAD_B0);
                w32(c, anim + 0xb0, 0x207);
                w32(c, model + 0x2dc, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_railslide_state_enter_matches() {
        check(
            ssx3_ee::s01::f_0013ad20__generated,
            rider_railslide_state_enter,
            8,
            &|c, seed| {
                let _r = base(c, seed, 0x13ad20, 2);
                let this = DATA + 0x1000;
                let rider = DATA + 0x4000;
                c.g[A0][0] = this as u64;
                w_2(c, this + 0x50, rider);
                put_vec(
                    c,
                    rider + 0x110,
                    [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 0],
                );
                w_2(c, rider + 0x10, 0);
                w_2(c, rider + 0x770, 0);
                w_2(c, GAME_GP.wrapping_sub(0x7184), 0.05f32.to_bits());
            },
        );
    }

    /// `RiderRailslideState_Enter` seeds the rail-value slew's step (`+0x260`)
    /// with 0.05 per tick: at 120/240 Hz the step is halved/quartered, so the
    /// rail value reaches its target in the same real time.
    #[test]
    fn rider_railslide_state_enter_follows_the_rate() {
        use crate::tick;
        // The game's 0.05 at gp-0x7184.
        const STEP_60: u32 = 0x3d4c_ccce;
        let step = |hz: u32| -> u32 {
            let mut c = crate::difftest::fresh();
            let _r = base(&mut c, 0, 0x13ad20, 0);
            let this = DATA + 0x1000;
            let rider = DATA + 0x4000;
            c.g[A0][0] = this as u64;
            c.bus.write32(this + 0x50, rider);
            for (i, v) in [1.0f32, 2.0, 3.0, 0.0].into_iter().enumerate() {
                c.bus.write32(rider + 0x110 + 4 * i as u32, v.to_bits());
            }
            c.bus.write32(rider + 0x10, 0);
            c.bus.write32(rider + 0x770, 0);
            c.bus.write32(GAME_GP.wrapping_sub(0x7184), STEP_60);
            tick::set_for_test(&mut c, hz);
            rider_railslide_state_enter(&mut c);
            c.bus.read32(rider + 0x260) // slew #9's maxStep
        };
        assert_eq!(step(60), STEP_60, "60 Hz step");
        assert_eq!(step(120), 0x3ccc_ccce, "120 Hz step");
        assert_eq!(step(240), 0x3c4c_ccce, "240 Hz step");
    }

    // -----------------------------------------------------------------
    // RiderGroundState_Enter / _Exit.

    const GSE_THIS: u32 = DATA + 0xa_0000;
    const GSE_RIDER: u32 = DATA + 0xa_1000;
    const GSE_MODEL: u32 = DATA + 0xa_2000;
    const GSE_WORLD: u32 = DATA + 0xa_3000;
    const GSE_AI: u32 = DATA + 0xa_3100;
    const GSE_SECT: u32 = DATA + 0xa_4000;

    /// The ground state, its rider, the world's AI record (sections at +0x44,
    /// the frame counter's owner at +0xc, counter at owner+0x8 = `now`) and
    /// the gp literals both functions load. The real `AI_GetFrameCounter`
    /// runs.
    fn ground_enter_exit_fixture(c: &mut Cpu, seed: u32, now: u32, stamp: u32) -> Rng {
        let mut r = machine(c, seed, 0x13_c7a8, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), GSE_WORLD);
        c.bus.write32(GSE_WORLD + 0x84, GSE_AI);
        c.bus.write32(GSE_AI + 0x44, GSE_SECT);
        c.bus.write32(GSE_AI + 0xc, GSE_AI + 0x80);
        c.bus.write32(GSE_AI + 0x88, now);
        c.bus.write32(GSE_THIS + 0x18, GSE_RIDER);
        c.bus.write32(GSE_THIS + 0x14, stamp);
        for off in [0x0u32, 0x4, 0x8, 0xc, 0x10] {
            c.bus.write32(GSE_THIS + off, r.next_u32());
        }
        let idx = r.next_u32() % 8;
        c.bus.write32(GSE_RIDER + 0x438, idx);
        c.bus.write32(GSE_SECT + idx * 0xb0 + 0x14, r.f32(2.0));
        c.bus.write32(GSE_SECT + idx * 0xb0 + 0x18, r.f32(2.0));
        c.bus.write32(GSE_RIDER + 0x780, GSE_MODEL);
        c.bus.write32(GSE_MODEL + 0x140, r.f32(2.0));
        put_vec(
            c,
            GSE_RIDER + 0x370,
            [r.f32(100.0), r.f32(100.0), r.f32(100.0), r.f32(1.0)],
        );
        put_vec(
            c,
            GSE_RIDER + 0x1e0,
            [r.f32(50.0), r.f32(50.0), r.f32(50.0), r.f32(1.0)],
        );
        c.bus
            .write32(GAME_GP.wrapping_sub(0x70f4), 0.01f32.to_bits());
        c.bus
            .write32(GAME_GP.wrapping_sub(0x70f0), 0.7f32.to_bits());
        c.bus
            .write32(GAME_GP.wrapping_sub(0x70ec), 0.7f32.to_bits());
        c.bus.write32(GAME_GP.wrapping_sub(0x6fa8), r.f32(1.0));
        c.bus.write32(GAME_GP.wrapping_sub(0x6fa4), r.f32(1.0));
        c.g[A0][0] = sx(GSE_THIS);
        r
    }

    #[test]
    fn rider_ground_state_enter_matches() {
        check_lohi(
            ee::s01::f_0013c7a8__generated,
            rider_ground_state_enter,
            200,
            &|c, seed| {
                let mut r = Rng(0x13c7_a8 ^ seed as u64);
                let now = 1000 + r.next_u32() % 100_000;
                // Air time: short, the ramp, clamped to 1, negative, the 40/41 edge.
                let air: i32 = match seed % 5 {
                    0 => (r.next_u32() % 41) as i32,
                    1 => 41 + (r.next_u32() % 30) as i32,
                    2 => 71 + (r.next_u32() % 2000) as i32,
                    3 => -1 - (r.next_u32() % 500) as i32,
                    _ => 40 + (seed / 5 % 2) as i32,
                };
                let _ = ground_enter_exit_fixture(c, seed, now, now.wrapping_sub(air as u32));
            },
        );
    }

    #[test]
    fn rider_ground_state_exit_matches() {
        check(
            ee::s01::f_0013f410__generated,
            rider_ground_state_exit,
            100,
            &|c, seed| {
                let mut r = Rng(0x13f4_10 ^ seed as u64);
                let now = r.next_u32() % 100_000;
                let stamp = match seed % 3 {
                    0 | 1 => u32::MAX,
                    _ => r.next_u32(),
                };
                let _ = ground_enter_exit_fixture(c, seed, now, stamp);
            },
        );
    }

    /// `RiderGroundState_Enter`'s landing speed factor follows the air time in
    /// real time: `n` 60 Hz frames in the air give the same factor (and the
    /// same scaled velocity, bit for bit) as `2n` ticks at 120 Hz and `4n`
    /// ticks at 240 Hz, over the 0.7 floor, the ramp and the clamp to 1.
    #[test]
    fn rider_ground_state_enter_follows_the_rate() {
        use crate::tick;
        let vel = |hz: u32, ticks: u32| -> [u32; 4] {
            let mut c = crate::difftest::fresh();
            let now = 50_000u32;
            let _ = ground_enter_exit_fixture(&mut c, 7, now, now - ticks);
            put_vec(
                &mut c,
                GSE_RIDER + 0x1e0,
                [
                    1.0f32.to_bits(),
                    2.0f32.to_bits(),
                    (-3.0f32).to_bits(),
                    1.0f32.to_bits(),
                ],
            );
            tick::set_for_test(&mut c, hz);
            rider_ground_state_enter(&mut c);
            assert_eq!(c.bus.read32(GSE_THIS + 0x10), now, "{hz} Hz: landing stamp");
            [0u32, 4, 8, 12].map(|k| c.bus.read32(GSE_RIDER + 0x1e0 + k))
        };
        for n in [0u32, 10, 39, 40, 41, 55, 69, 70, 71, 100, 600] {
            let at60 = vel(60, n);
            assert_eq!(vel(120, 2 * n), at60, "{n} frames at 120 Hz");
            assert_eq!(vel(240, 4 * n), at60, "{n} frames at 240 Hz");
        }
        // 100 ticks at 240 Hz are 25 frames: the short-hop 0.7, not the
        // clamped 1 that 100 frames at 60 Hz give.
        assert_eq!(f32::from_bits(vel(240, 100)[0]), 0.7f32);
        assert_eq!(f32::from_bits(vel(60, 100)[0]), 1.0f32);
    }

    /// `RiderGroundState_Exit`'s first-exit stamp is `now - 500` 60 Hz frames:
    /// 500, 1000 and 2000 ticks back at 60, 120 and 240 Hz. A real take-off
    /// stamp is `now` at any rate.
    #[test]
    fn rider_ground_state_exit_follows_the_rate() {
        use crate::tick;
        let stamp = |hz: u32, old: u32| -> u32 {
            let mut c = crate::difftest::fresh();
            let _ = ground_enter_exit_fixture(&mut c, 3, 10_000, old);
            tick::set_for_test(&mut c, hz);
            rider_ground_state_exit(&mut c);
            c.bus.read32(GSE_THIS + 0x14)
        };
        assert_eq!(stamp(60, u32::MAX), 9_500);
        assert_eq!(stamp(120, u32::MAX), 9_000);
        assert_eq!(stamp(240, u32::MAX), 8_000);
        for hz in [60u32, 120, 240] {
            assert_eq!(stamp(hz, 1234), 10_000, "{hz} Hz: take-off stamp");
        }
    }

    #[test]
    fn rider_init_data_matches() {
        check(
            ee::s01::f_00140738__generated,
            rider_init_data,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x14_0738, 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    pub(crate) const SEEDS_6: u32 = 200;

    /// The function's argument (`GroundState*`).
    pub(crate) const GS: u32 = DATA + 0x1000;

    pub(crate) const RIDER_3: u32 = DATA + 0x2000;

    pub(crate) const MODEL_2: u32 = DATA + 0x3400;

    pub(crate) const ANIM_2: u32 = DATA + 0x3600;

    pub(crate) const STATE_2: u32 = DATA + 0x4000;

    pub(crate) const WORLD_2: u32 = DATA + 0x5000;

    pub(crate) const RACE: u32 = DATA + 0x5100;

    pub(crate) const GROUND: u32 = DATA + 0x6000;

    pub(crate) const OVERLAP: u32 = DATA + 0x7000;

    pub(crate) const CURVES: u32 = DATA + 0x8000;

    pub(crate) const BELIB: u32 = DATA + 0x9000;

    pub(crate) const IFACE: u32 = DATA + 0x9100;

    pub(crate) const VT: u32 = DATA + 0x9200;

    /// `(&DAT_005308a8)[i]`, the BE interface array indexed by `gp-0x1efc`.
    pub(crate) const IFACE_ARRAY: u32 = 0x0053_08a8;

    /// The per-frame ground-state update, with the world/race/ground tables,
    /// the rider's model/animation/state objects, the BE library interface
    /// (whose one vtable method is a fake) and the global speed-curve
    /// pointers the real callees read. The collision volume misses (empty
    /// overlap list), so the ground probe takes its short, no-contact path.
    ///
    /// The contact branches (a ProbeGround hit, and the `rider+0x438` values
    /// 2/3/0xd it leaves) need real collision geometry and a populated
    /// overlap list, so the fixture cannot reach them; everything before the
    /// probe is covered. The port keeps those branches verbatim.
    pub(crate) fn setup_2(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0013_d818, 0);
        elf_image(c);
        // The generated frame: when the probe's volume misses it leaves the
        // slot at +0x110 alone, so start from a zeroed frame instead of the
        // random bytes `machine` puts there (it would be followed as a
        // pointer by the contact callback path).
        for i in 0..0x210 / 4 {
            c.bus.write32(SP - 0x210 + 4 * i, 0);
        }
        // Ground state.
        c.bus.write32(GS, r.f32(3.0)); // steering angle
        c.bus.write32(GS + 4, r.f32(30.0)); // forward speed
        c.bus.write32(GS + 8, r.f32(30.0)); // lateral speed
        c.bus.write32(GS + 0xc, 0xdead_beef); // grounded flag, set by the tail
        c.bus.write32(GS + 0x18, RIDER_3);
        c.g[A0][0] = sx(GS);
        // Rider: small floats everywhere, then the fields that are pointers,
        // flags or indices.
        fill(c, &mut r, RIDER_3, 0xc00 / 4);
        c.bus
            .write32(RIDER_3 + 0x214, if seed % 3 == 0 { 0 } else { r.f32(1.0) }); // lean dead zone
        c.bus.write32(RIDER_3 + 0x2e4, r.f32(0.9)); // maximum tilt
        c.bus.write32(RIDER_3 + 0x378, r.f32(20.0)); // vertical speed
        c.bus.write32(RIDER_3 + 0x454, r.f32(10.0)); // ground contact parameter
        c.bus.write32(RIDER_3 + 0x780, MODEL_2);
        c.bus.write32(RIDER_3 + 0x784, ANIM_2);
        c.bus.write32(RIDER_3 + 0x77c, STATE_2);
        c.bus.write32(RIDER_3 + 0x860, OVERLAP);
        c.bus.write32(RIDER_3 + 0x864, 0);
        c.bus.write32(RIDER_3 + 0x874, 0); // wind disabled
        c.bus.write32(RIDER_3 + 0x86c, 0);
        c.bus.write32(RIDER_3 + 0xb34, 0);
        c.bus.write32(RIDER_3 + 0x438, seed % 4); // ground segment index
        fill(c, &mut r, MODEL_2, 0x200 / 4);
        fill(c, &mut r, ANIM_2, 0x40 / 4);
        c.bus.write32(ANIM_2 + 8, 0x1b6); // "no entry" -> the anim-table call is skipped
        fill(c, &mut r, STATE_2, 0xe00 / 4);
        c.bus.write32(STATE_2 + 0xde4, seed % 5); // sub-state
                                                  // World -> race manager -> the ground segments.
        c.bus.write32(GAME_GP - 0x848, WORLD_2);
        c.bus.write32(WORLD_2 + 0x84, RACE);
        c.bus.write32(RACE + 0x44, GROUND);
        fill(c, &mut r, GROUND, 4 * 0xb0 / 4);
        // An empty overlap list: CollideVolume finds nothing.
        for i in 0..0x200 / 4 {
            c.bus.write32(OVERLAP + 4 * i, 0);
        }
        // The six animation speed/lean curve structs.
        for (k, off) in [0x1fa8u32, 0x1fa0, 0x1fb8, 0x1fc0, 0x1f98, 0x1fc8]
            .into_iter()
            .enumerate()
        {
            let a = CURVES + 0x40 * k as u32;
            fill(c, &mut r, a, 8);
            c.bus.write32(GAME_GP - off, a);
        }
        // The BE library singleton and interface slot 0 (the generated
        // GetInterface indexes it by `gp-0x1efc`), whose vtable method is
        // unmapped so the fake call intercepts it.
        c.bus.write32(GAME_GP - 0x1f04, BELIB);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        // A non-zero character attribute divisor (the table is in .bss).
        c.bus.write8(0x0053_08e3, 10);
        c.bus.write32(IFACE_ARRAY, IFACE);
        c.bus.write32(IFACE + 4, VT);
        c.bus.write16(VT + 8, 0);
        c.bus.write32(VT + 0xc, 0x7000_0000);
    }

    /// Exercise the actual ground update at both sides of the speed clamp.
    /// The low coefficient keeps the random test geometry below the clamp;
    /// the high coefficient saturates it. Both still reach the yaw gain.
    #[test]
    fn ground_yaw_runtime_gain_follows_the_rate() {
        for (hz, seed, speed_coefficient, expected_gain) in [
            (60, 0, 1e-30f32, 0.0019627037f32),
            (120, 0, 1e-30f32, 0.00024539232f32),
            (60, 1, 1.0f32, 0.1f32),
            (120, 1, 1.0f32, 0.051316738f32),
        ] {
            let mut c = crate::difftest::fresh();
            setup_2(&mut c, seed);
            crate::tick::set_for_test(&mut c, hz);
            for (a, bits) in crate::tick::per_tick_values(hz) {
                c.bus.write32(a, bits);
            }
            c.bus.write32(GAME_GP - 0x7064, crate::tick::step_bits(hz));
            c.bus.write32(GS + 4, 600.0f32.to_bits());
            c.bus.write32(GS + 8, 0.0f32.to_bits());
            c.bus.write32(GAME_GP - 0x7024, speed_coefficient.to_bits());
            let material = GROUND + (seed % 4) * 0xb0;
            for (off, value) in [
                (0x14, 600.0f32),
                (0x18, 0.0),
                (0x28, 0.0),
                (0x2c, 0.1),
                (0x34, 1.0),
            ] {
                c.bus.write32(material + off, value.to_bits());
            }
            c.bus.write32(RIDER_3 + 0x300, 1.0f32.to_bits());
            c.bus.write32(RIDER_3 + 0x1f0, 0.0f32.to_bits());
            c.bus.write32(RIDER_3 + 0x2dc, 0.0f32.to_bits());
            c.bus.write32(MODEL_2 + 0x140, 0.0f32.to_bits());
            for reg in 1..32 {
                c.bus.plat.set_vf(reg, [0; 4]);
            }
            c.bus.plat.set_acc([0; 4]);
            for (off, v) in [
                (0x1b0, [1.0f32, 0.0, 0.0, 0.0]),
                (0x1e0, [0.0, 1.0, 0.0, 0.0]),
                (0x370, [1.0, 0.0, 0.0, 0.0]),
                (0x3a0, [0.0, 1.0, 0.0, 0.0]),
                (0x3d0, [100.0, 1.0, 0.0, 0.0]),
            ] {
                for (i, value) in v.iter().enumerate() {
                    c.bus.write32(RIDER_3 + off + 4 * i as u32, value.to_bits());
                }
            }
            rider_ground_state_update(&mut c);
            assert_eq!(c.pc, crate::difftest::RA, "{hz} Hz, seed {seed}");
            let actual = f32::from_bits(c.f[7]);
            assert!(
                (actual - expected_gain).abs() < 1e-6,
                "{hz} Hz, seed {seed}: yaw gain {actual} != {expected_gain}"
            );
        }
    }

    #[test]
    fn rider_ground_state_update_matches() {
        for seed in 0..SEEDS {
            let r = compare(
                ee::s01::f_0013d818__generated,
                rider_ground_state_update,
                &|c| setup_2(c, seed),
            );
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lo/hi: {:#?}", r.lohi);
        }
    }
    use super::*;

    use super::*;
    use super::*;
    pub(crate) use crate::game::ai::computer::tests::fake_closest_point;
    pub(crate) use crate::game::ai::computer::tests::fake_collide_volume;
    pub(crate) use crate::game::ai::computer::tests::install_fake_slots;
    pub(crate) use crate::game::ai::computer::tests::FCC_CTRL;
    pub(crate) use crate::game::ai::computer::tests::FCC_MAGIC;
    pub(crate) use crate::game::ai::computer::tests::UDF_CTRL;
    pub(crate) use crate::game::ai::computer::tests::UDF_MAGIC;

    // ------------------------------------------------ RiderAirState_CollideWalls

    /// `RiderAirState_CollideWalls`: sub state 9 (skipped), no contact, and
    /// a contact the rider recedes from (push-out through the real
    /// `Rider_Translate`). The overlap query is the gated fake
    /// `OverlapList_CollideVolume` under the real
    /// `OverlapList_FindClosestContact`; the controller's virtual call lands
    /// in the fakes, which sometimes unwind. The impact branch (bounce,
    /// world matrix, `Rider_HandleImpact`) is not reached: the real
    /// `Rider_HandleImpact` cannot be bounded on a fixture rider.
    #[test]
    #[ignore = "fixture runs away (pc 0x312138); the port is parked, not installed"]
    fn rider_air_state_collide_walls_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::put_vec;
        install_fake_slots();
        check(
            ssx3_ee::s01::f_0013aa48,
            rider_air_state_collide_walls,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0013_aa48, 4);
                crate::difftest::elf_image(c);
                c.pc = 0x0013_aa48;
                let f = |x: f32| x.to_bits();
                let this = DATA;
                let rider = DATA + 0x1000;
                let sub = DATA + 0x3000;
                let ctl_vt = DATA + 0x3100;
                let tree = DATA + 0x3200;
                let list = DATA + 0x3400;
                let out = DATA + 0x3500;
                let template = DATA + 0x1_0000;
                let hit_obj = DATA + 0x3a00;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(out);
                c.bus.write32(this + 0x4, rider);
                c.bus.write32(rider + 0x77c, sub);
                let sub_state = if seed % 11 == 0 { 9 } else { r.next_u32() % 8 };
                c.bus.write32(sub + 0xde4, sub_state);
                // Controller: vt[0x40] = { delta, fake } ("use the small hull").
                c.bus.write32(rider + 0x6c0, ctl_vt);
                c.bus.write16(ctl_vt + 0x40, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(ctl_vt + 0x44, FAKE);
                // Sphere tree: a single sphere (no children).
                c.bus.write32(rider + 0xaa0, tree);
                c.bus.write32(tree + 0x28, 0);
                put_vec(
                    c,
                    tree + 0x10,
                    [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)],
                );
                c.bus
                    .write32(tree + 0x20, f(0.5 + (r.next_u32() % 4) as f32));
                c.bus.write32(rider + 0x860, list);
                c.bus.write32(rider + 0x868, r.next_u32() % 0x40);
                put_vec(
                    c,
                    rider + 0x110,
                    [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)],
                );
                put_vec(
                    c,
                    rider + 0x180,
                    [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)],
                );
                // Velocity x shares the sign of every contact normal's x, so the rider
                // always recedes ((-n.x) * vel.x < 0): the impact path's
                // Rider_HandleImpact runs the rider's animation/reset machinery (a
                // runaway on a fixture rider, pc 0x312138 in
                // AnimController_PlayAnimImpl), so it is left untested.
                let side = if r.next_u32() & 1 == 0 { 1.0f32 } else { -1.0 };
                let vx = side * (1.0 + (r.next_u32() % 30) as f32);
                put_vec(c, rider + 0x1e0, [f(vx), r.f32(30.0), r.f32(30.0), 0]);
                let unit = |r: &mut Rng| {
                    let v = [
                        (r.next_u32() % 200) as f32 - 100.0,
                        (r.next_u32() % 200) as f32 - 100.0,
                        (r.next_u32() % 200) as f32 - 100.0 + 0.5,
                    ];
                    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                    [v[0] / n, v[1] / n, v[2] / n]
                };
                let fwd = unit(&mut r);
                put_vec(c, rider + 0x370, [f(fwd[0]), f(fwd[1]), f(fwd[2]), 0]);

                // The contacts the fake overlap query reports.
                let count = if seed % 5 == 1 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                c.bus.write32(FCC_CTRL, FCC_MAGIC);
                c.bus.write32(FCC_CTRL + 4, count);
                c.bus.write32(FCC_CTRL + 8, template);
                for i in 0..count {
                    let hit = template + 0x80 * i;
                    for k in 0..0x20 {
                        c.bus.write32(hit + 4 * k, r.next_u32());
                    }
                    put_vec(c, hit, [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)]);
                    let mut n = unit(&mut r);
                    n[0] = side * (n[0].abs() + 0.25);
                    put_vec(c, hit + 0x10, [f(n[0]), f(n[1]), f(n[2]), 0]);
                    put_vec(c, hit + 0x20, [r.f32(50.0), r.f32(50.0), r.f32(50.0), 0]);
                    let depth = if seed % 6 == 2 {
                        -((r.next_u32() % 10) as f32) - 0.5
                    } else {
                        (r.next_u32() % 10) as f32 * 0.25
                    };
                    c.bus.write32(hit + 0x40, f(depth));
                    c.bus.write32(hit + 0x4c, r.next_u32() % 16);
                    c.bus
                        .write32(hit + 0x50, if seed % 3 == 0 { 0 } else { hit_obj });
                    c.bus.write32(hit + 0x54, 0);
                }
                c.bus.write32(hit_obj + 0xc, 0);
            },
        );
    }

    /// RiderGroundState_CollideWalls fixture. `impact`: the rider moves into
    /// the wall after the push-out (reaches the bounce, the deflection and
    /// the impact/sound callees); otherwise it recedes from it.
    ///
    /// The sub-state is 9 or not; the controller's "in contact" method is the
    /// real getter 0x2d1bd8 (returns the word at gp+0x16c8, set to 0 or 1);
    /// speed and height pick the slow-on-the-ground early exit. The overlap
    /// list holds no object (no contact: depth -1) or one static object with
    /// a huge box whose sub-object's collide method (a fake) reports one
    /// contact; the contact record then comes from the closest-contact
    /// query's frame, which the fixture fills (depth, normal, surface velocity,
    /// hit object with or without an owner to notify).
    pub(crate) fn collide_walls_setup(c: &mut Cpu, seed: u32, impact: bool) {
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        use crate::difftest::{put_vec, DATA, GAME_GP, SP};
        let f = |x: f32| x.to_bits();
        let mut r = machine(c, seed, 0x13f4_88 ^ impact as u64, 4);
        c.pc = 0x0013_f488;
        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const SUB: u32 = DATA + 0x3000;
        const CTL_VT: u32 = DATA + 0x3100;
        const HULL: u32 = DATA + 0x3200;
        const LIST: u32 = DATA + 0x3400;
        const OBJ: u32 = DATA + 0x3600;
        const SUBOBJ: u32 = DATA + 0x3700;
        const SUB_VT: u32 = DATA + 0x3800;
        const HIT_OBJ: u32 = DATA + 0x3a00;
        const OWNER: u32 = DATA + 0x3b00;
        const OWNER_VT: u32 = DATA + 0x3c00;
        // The closest-contact query's frame (0x2050 below ours, 0x490).
        const RECORD: u32 = SP - 0x490 - 0x2050;

        // .lit4 constants.
        c.bus.write32(GAME_GP - 0x6fa0, f(-0.999_9));
        c.bus.write32(GAME_GP - 0x6f9c, f(1.1));
        c.bus.write32(GAME_GP - 0x6f98, f(0.05));
        c.bus.write32(GAME_GP - 0x6f94, f(27.777_779));

        c.g[A0][0] = sx(SELF);
        c.bus.write32(SELF + 0x18, RIDER);
        c.bus.write32(RIDER + 0x77c, SUB);
        let sub_state = if !impact && seed % 9 == 0 {
            9
        } else {
            r.next_u32() % 8
        };
        c.bus.write32(SUB + 0xde4, sub_state);
        // Controller: vt[0x40] = { delta, 0x2d1bd8 }.
        c.bus.write32(RIDER + 0x6c0, CTL_VT);
        c.bus.write16(CTL_VT + 0x40, (r.next_u32() % 4 * 4) as u16);
        c.bus.write32(CTL_VT + 0x44, 0x002d_1bd8);
        let in_contact = impact || r.next_u32() % 2 == 0;
        c.bus.write32(GAME_GP + 0x16c8, in_contact as u32);
        c.bus.write32(RIDER + 0x4c8, f((r.next_u32() % 100) as f32));
        let y = (r.next_u32() % 200) as f32 - 100.0;
        c.bus.write32(RIDER + 0x118, f(y));
        c.bus
            .write32(RIDER + 0x498, f(y + (r.next_u32() % 100) as f32 - 50.0));
        c.bus.write32(RIDER + 0xaa0, HULL);
        c.bus.write32(RIDER + 0x860, LIST);
        c.bus.write32(RIDER + 0x868, r.next_u32() % 0x40);
        // Hull: no spheres (+0x28 mask, +0x2c count), radius at +0x20.
        c.bus.write32(HULL + 0x20, f(0.5));

        // Overlap list: no instances, zero or one object.
        let contact = impact || seed % 5 != 1;
        c.bus.write32(LIST + 0x8, contact as u32);
        c.bus.write32(LIST + 0xc, LIST + 0x40);
        c.bus.write32(LIST + 0x40, OBJ);
        c.bus.write32(OBJ + 0x8, 0x20);
        for k in 0..3 {
            c.bus.write32(OBJ + 0x60 + 4 * k, f(-1.0e30));
            c.bus.write32(OBJ + 0x6c + 4 * k, f(1.0e30));
        }
        c.bus.write32(OBJ + 0x88, 1);
        c.bus.write32(OBJ + 0xc, SUBOBJ);
        c.bus.write32(SUBOBJ + 0xc, SUB_VT);
        c.bus.write32(SUB_VT + 0x134, FAKE);
        c.bus.write32(SUB_VT + 0x13c, FAKE + 0x10);
        // Fake call 1 (vt 0x13c) reports one contact.
        c.bus.write32(LOG_RESULTS + 4, 1);

        // The contact record.
        let unit = |r: &mut crate::difftest::Rng| {
            let v = [
                (r.next_u32() % 200) as f32 - 100.0,
                (r.next_u32() % 200) as f32 - 100.0,
                (r.next_u32() % 200) as f32 - 100.0 + 0.5,
            ];
            let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            [v[0] / n, v[1] / n, v[2] / n]
        };
        let fwd = unit(&mut r);
        let normal = if !impact && seed % 7 == 3 {
            [-fwd[0], -fwd[1], -fwd[2]] // head-on
        } else {
            unit(&mut r)
        };
        put_vec(c, RIDER + 0x370, [f(fwd[0]), f(fwd[1]), f(fwd[2]), 0]);
        put_vec(
            c,
            RECORD + 0x10,
            [f(normal[0]), f(normal[1]), f(normal[2]), 0],
        );
        let point = [r.f32(50.0), r.f32(50.0), r.f32(50.0), 0];
        put_vec(c, RECORD, [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)]);
        put_vec(c, RECORD + 0x20, point);
        for k in 0x30..0x40 {
            c.bus.write8(RECORD + k, r.next_u32() as u8);
        }
        let depth = if !impact && seed % 6 == 2 {
            -((r.next_u32() % 10) as f32) - 0.5
        } else {
            (r.next_u32() % 10) as f32 * 0.25
        };
        c.bus.write32(RECORD + 0x40, f(depth));
        c.bus.write32(RECORD + 0x44, r.next_u32());
        c.bus.write32(RECORD + 0x48, r.next_u32());
        c.bus.write32(RECORD + 0x4c, r.next_u32() % 16);
        let hit_obj = match seed % 3 {
            0 => 0,
            _ => HIT_OBJ,
        };
        c.bus.write32(RECORD + 0x50, hit_obj);
        for k in 0x54..0x80 {
            c.bus.write8(RECORD + k, r.next_u32() as u8);
        }
        c.bus
            .write32(HIT_OBJ + 0xc, if seed % 3 == 1 { 0 } else { OWNER });
        c.bus.write32(OWNER + 0xc, OWNER_VT);
        c.bus.write16(OWNER_VT + 0x150, 8);
        c.bus.write32(OWNER_VT + 0x154, FAKE + 0x20);

        // The rider's velocity relative to the hit surface's: along the
        // normal (receding: no bounce) or against it (impact).
        let k = (1 + r.next_u32() % 20) as f32 * if impact { -1.0 } else { 1.0 };
        let p = |i: usize| f32::from_bits(point[i]);
        put_vec(
            c,
            RIDER + 0x1e0,
            [
                f(p(0) + normal[0] * k),
                f(p(1) + normal[1] * k),
                f(p(2) + normal[2] * k),
                0,
            ],
        );
        for off in [0x110, 0x400, 0x410, 0x9d0] {
            put_vec(
                c,
                RIDER + off,
                [r.f32(100.0), r.f32(100.0), r.f32(100.0), f(1.0)],
            );
        }
    }

    /// RiderGroundState_CollideWalls up to the bounce decision: skipped
    /// sub-state, slow-on-the-ground exit, no contact, head-on, facing and
    /// not facing the wall (normal flattening), the hit-object notification,
    /// and a rider receding from the wall after the push-out. Fakes unwind
    /// at chosen calls.
    #[test]
    fn rider_ground_state_collide_walls_matches() {
        crate::difftest::check(
            ee::s01::f_0013f488__generated,
            rider_ground_state_collide_walls,
            300,
            &|c, seed| collide_walls_setup(c, seed, false),
        );
    }

    /// RiderGroundState_CollideWalls through the bounce: velocity update,
    /// Rider_DeflectAlongWall, Rider_BuildWorldMatrix, Rider_HandleImpact
    /// and the wall-hit sound (real callees on a mostly zeroed rider; their
    /// virtual calls through empty memory land in the fakes).
    #[test]
    fn rider_ground_state_collide_walls_impact_matches() {
        crate::difftest::check(
            ee::s01::f_0013f488__generated,
            rider_ground_state_collide_walls,
            64,
            &|c, seed| collide_walls_setup(c, seed, true),
        );
    }

    /// RiderGroundState_PostUpdate fixture. `branches == false`: the rider
    /// stays on the ground on an ordinary surface (no wipeout, no path
    /// reset); `true`: leaving the ground (`Rider_LaunchIntoAir`) in a main
    /// state other than 0. Sub-state 9 keeps the wall collisions
    /// (`RiderGroundState_CollideWalls`, `Rider_HandleWallCollision`) on
    /// their early exits; the rider's animation table answers "no bump"
    /// (entry 0x1b6); the zeroed BELibrary dummy at gp-0x1f04 keeps
    /// `BELibrary_GetInstance` (from `Rider_GetBumpStrength` /
    /// `Rider_CheckRiderCollisions`) from building the library, which loads
    /// files from disc (the runaway at 0x3de768). Velocities below and above
    /// the maximal speed and the full-speed constant.
    ///
    /// Not reached (unbounded on a test machine): the wipeout surface
    /// (`Rider_StartWipeout`), path resets (`Rider_CheckResetPath`) and
    /// main state 0 on leaving (`Rider_SetMainState` + take-off sound):
    /// their audio callees reach `BankMonitor_StopHandle` / sound playback,
    /// which reach the same disc-loading request loop.
    pub(crate) fn post_update_setup(c: &mut Cpu, seed: u32, branches: bool) {
        use crate::difftest::fakes::machine;
        use crate::difftest::{put_vec, DATA, GAME_GP};
        let f = |x: f32| x.to_bits();
        let mut r = machine(
            c,
            seed,
            0x13f1_78 ^ branches as u64,
            if branches { 3 } else { 0 },
        );
        c.pc = 0x0013_f178;
        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const SUB: u32 = DATA + 0x3000;
        const HULL: u32 = DATA + 0x4000;
        const WORLD: u32 = DATA + 0x4100;
        const W84: u32 = DATA + 0x4200;
        const SURF: u32 = DATA + 0x5000;
        const ANIM: u32 = DATA + 0x6000;
        const BE_LIBRARY: u32 = DATA + 0x8000;
        for k in 0..0x400 {
            c.bus.write32(BE_LIBRARY + 4 * k, 0);
        }
        c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
        c.bus.write32(GAME_GP - 0x6fac, f(3333.3335));
        c.bus.write32(GAME_GP - 0x1f94, r.next_u32());
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, W84);
        c.bus.write32(W84 + 0x44, SURF);
        c.g[A0][0] = sx(SELF);
        c.bus.write32(SELF + 0x18, RIDER);
        let leaving = branches;
        c.bus.write32(SELF + 0xc, leaving as u32);
        c.bus.write32(RIDER + 0x77c, SUB);
        c.bus.write32(SUB + 0xde4, 9);
        c.bus.write32(
            SUB + 0xde0,
            if seed % 2 == 0 && !leaving {
                0
            } else {
                1 + r.next_u32() % 5
            },
        );
        c.bus.write32(RIDER + 0xaa0, HULL);
        c.bus.write32(RIDER + 0x784, ANIM);
        c.bus.write32(ANIM + 4, 0x1b6);
        let surface = r.next_u32() % 0x12;
        c.bus.write32(RIDER + 0x438, surface);
        let reset = false;
        let by_table = reset && seed % 2 == 0;
        c.bus.write32(SURF + surface * 0xb0 + 0x44, by_table as u32);
        let flags = (r.next_u32() & 0xfffd) as u16 | if reset && !by_table { 2 } else { 0 };
        c.bus.write16(RIDER + 0x2d4, flags);
        for off in [0x110, 0x370, 0x390] {
            put_vec(c, RIDER + off, [r.f32(1.0), r.f32(1.0), r.f32(1.0), f(1.0)]);
        }
        let speed = [10.0f32, 5000.0][(seed / 2 % 2) as usize];
        put_vec(
            c,
            RIDER + 0x1e0,
            [r.f32(speed), r.f32(speed), r.f32(speed), 0],
        );
        c.bus.write32(
            RIDER + 0x2e4,
            f([100.0f32, 8000.0][(seed / 4 % 2) as usize]),
        );
        c.bus.write32(RIDER + 0x1f0, r.f32(50.0));
        c.bus.write32(RIDER + 0x214, r.f32(5.0));
    }

    /// RiderGroundState_PostUpdate on the ground: no launch, wipeout or
    /// path reset; bounds, walls (masked hull spheres), rider collisions,
    /// up smoothing, speed clamp and fraction.
    #[test]
    fn rider_ground_state_post_update_matches() {
        crate::difftest::check(
            ee::s01::f_0013f178__generated,
            rider_ground_state_post_update,
            160,
            &|c, seed| post_update_setup(c, seed, false),
        );
    }

    /// RiderGroundState_PostUpdate leaving the ground: `Rider_LaunchIntoAir`
    /// (real, on a mostly empty rider; virtual calls through empty memory
    /// land in the fakes, some of which unwind), main state already set.
    #[test]
    fn rider_ground_state_post_update_launch_matches() {
        crate::difftest::check(
            ee::s01::f_0013f178__generated,
            rider_ground_state_post_update,
            96,
            &|c, seed| post_update_setup(c, seed, true),
        );
    }

    // -- 0x0013ccf0 RiderGroundState_ComputeDrag ------------------------------

    /// `RiderGroundState_ComputeDrag`: speeds below, between and beyond the
    /// four curve points, every ramp band, the surface classes (2, 3, 13,
    /// other), climbing or not, stance mismatches with and without boost;
    /// the behaviour library's interface getter is a fake (some calls
    /// unwind), the attribute-factor callees are real on the game image's
    /// tables (values > 0, or <= 0 for the `attrib / 5` default).
    #[test]
    fn rider_ground_state_compute_drag_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::{elf_image, put_words, DATA, GAME_GP};
        fn f(x: f32) -> u32 {
            x.to_bits()
        }
        fn uf(r: &mut crate::difftest::Rng, lo: f32, hi: f32) -> u32 {
            f(lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo))
        }
        const THIS: u32 = DATA;
        const CFG: u32 = DATA + 0x400;
        const RIDER: u32 = DATA + 0x1000;
        const BE_LIBRARY: u32 = DATA + 0x3000;
        const BE_OBJ: u32 = DATA + 0x3100;
        const BE_VT: u32 = DATA + 0x3200;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        const CHARACTER_MAX: u32 = 0x0053_08d8;
        covered(
            ee::s01::f_0013ccf0__generated,
            rider_ground_state_compute_drag,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x13_ccf0, 3);
                let r = &mut r;
                c.pc = 0x0013_ccf0;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // The behaviour library: interface getter (vtable slot +8:
                // delta 0, method a fake).
                w(c, GAME_GP - 0x1f04, BE_LIBRARY);
                w(c, GAME_GP - 0x1efc, 0);
                w(c, BE_INTERFACES, BE_OBJ);
                w(c, BE_OBJ + 4, BE_VT);
                c.bus.write16(BE_VT + 8, 0);
                w(c, BE_VT + 0xc, FAKE);
                // Character maxima (non-zero divisors).
                for ch in 0..16 {
                    for n in 0..7 {
                        c.bus.write8(
                            CHARACTER_MAX + ch * 15 + 8 + n,
                            1 + (r.next_u32() % 20) as u8,
                        );
                    }
                }

                // The state: its rider and the surface depth divisor.
                w(c, THIS + 0x18, RIDER);
                w(c, THIS + 0x8, uf(r, 1.0, 50.0));
                // The curve: four increasing x with their y.
                let mut x = 0.0f32;
                for i in 0..4 {
                    x += 5.0 + (r.next_u32() % 20) as f32;
                    put_words(c, CFG + 0x90 + 8 * i, &[f(x), uf(r, 0.0, 30.0)]);
                }
                // The rider.
                w(
                    c,
                    RIDER + 0x214,
                    if seed % 5 == 0 { 0 } else { uf(r, -1.5, 1.5) },
                );
                w(c, RIDER + 0x86c, r.next_u32() % 6);
                w(
                    c,
                    RIDER + 0xb34,
                    if seed % 4 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 5
                    },
                );
                w(c, RIDER + 0x378, uf(r, 0.0, 0.3));
                w(
                    c,
                    RIDER + 0x438,
                    [2, 3, 13, 13, 0, 7][(r.next_u32() % 6) as usize],
                );
                w(c, RIDER + 0x454, uf(r, -5.0, 5.0));
                w(c, RIDER + 0x1e8, uf(r, -10.0, 10.0));
                w(c, RIDER + 0x3d8, uf(r, -10.0, 10.0));
                w(c, RIDER + 0x320, r.next_u32() % 2);
                w(c, RIDER + 0x324, r.next_u32() % 2);
                w(
                    c,
                    RIDER + 0x2fc,
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        uf(r, 0.0, 1.0)
                    },
                );

                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(CFG);
                // Speeds across the curve (|speed| * 0.036 vs the x points).
                c.f[12] = if seed % 9 == 0 {
                    r.edge_f32()
                } else {
                    uf(r, -2500.0, 2500.0)
                };
            },
        );
    }
    use super::*;

    /// ObjPtrArray_MergeSort on -3..=24 pointers (two elements, the small
    /// halves, deep recursion, empty/negative counts), objects with keys
    /// from a small range (ties keep their order) or full 32-bit values
    /// (unsigned compare); the buffer after the array. Events are delivered
    /// in the merge loops (at any recursion depth).
    #[test]
    fn obj_ptr_array_merge_sort_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        use crate::difftest::fakes::machine;
        use crate::difftest::DATA;
        const ARRAY: u32 = DATA + 0x1_0000;
        const BUF: u32 = DATA + 0x1_1000;
        const OBJS: u32 = DATA + 0x2_0000;
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x13f8_f800, 0);
            let n: i32 = match seed % 10 {
                0 => 2,
                1 => (r.next_u32() % 4) as i32 - 3,
                _ => (r.next_u32() % 25) as i32,
            };
            let small_keys = seed % 3 != 0;
            for i in 0..32u32 {
                let obj = OBJS + 0x100 * i;
                c.bus
                    .write32(ARRAY - 0x40 + 4 * i, OBJS + 0x100 * (r.next_u32() % 32));
                c.bus.write32(ARRAY + 4 * i, obj);
                c.bus.write32(BUF - 0x40 + 4 * i, r.next_u32());
                c.bus
                    .write32(BUF + 4 * i, OBJS + 0x100 * (r.next_u32() % 32));
                let key = if small_keys {
                    r.next_u32() % 6
                } else {
                    r.next_u32()
                };
                c.bus.write32(obj + MERGE_SORT_KEY, key);
            }
            c.g[A0][0] = ARRAY as u64;
            c.g[A1][0] = sx(ARRAY.wrapping_add((4 * n) as u32));
            c.g[A2][0] = BUF as u64;
            schedule(c, &mut r, seed, 0x0013_f8f8)
        };
        check_events(
            ee::s01::f_0013f8f8__generated,
            obj_ptr_array_merge_sort,
            300,
            &setup,
        );
    }

    #[test]
    fn grounded_lean_brake_follows_the_rate() {
        use crate::tick;
        // Freeze the other forces with zero elapsed dt and use a flat
        // ground packet. The actual ground update still applies its brake.
        let run = |hz: u32, lean: f32, speed: f32, normal_z: f32| {
            let mut velocity = speed;
            for _ in 0..hz / 60 {
                let mut c = crate::difftest::fresh();
                setup_2(&mut c, 0);
                tick::set_for_test(&mut c, hz);
                for (a, bits) in tick::per_tick_values(hz) {
                    c.bus.write32(a, bits);
                }
                c.bus.write32(RIDER_3 + 0x300, 0); // isolate non-dt brake
                c.bus.write32(RIDER_3 + 0x2e4, 10000.0f32.to_bits());
                c.bus.write32(RIDER_3 + 0x214, lean.to_bits());
                for (off, values) in [
                    (0x1e0, [velocity, 0.0, 0.0, 0.0]),
                    (0x370, [0.0, 0.0, normal_z, 0.0]),
                    (0x3d0, [0.0; 4]),
                ] {
                    for (i, value) in values.into_iter().enumerate() {
                        c.bus.write32(RIDER_3 + off + 4 * i as u32, value.to_bits());
                    }
                }
                c.g[A0][0] = GS as u64;
                rider_ground_state_update(&mut c);
                assert_eq!(c.pc, crate::difftest::RA);
                velocity = f32::from_bits(c.bus.read32(RIDER_3 + 0x1e0));
            }
            velocity
        };
        for (lean, speed, normal_z) in [
            (0.8, 400.0, 1.0),
            (-0.8, 400.0, 1.0),
            (0.0, 400.0, 1.0),
            (0.8, 600.0, 1.0),
            (0.8, 400.0, 0.5),
            (0.8, 1.0, 1.0),
        ] {
            let stock = run(60, lean, speed, normal_z);
            if lean.abs() > 0.5 && speed == 400.0 && normal_z == 1.0 {
                assert!(
                    (stock - 360.0).abs() < 0.001,
                    "brake branch not reached: {stock}"
                );
            }
            if speed == 1.0 {
                assert_eq!(stock, 0.0, "near-zero brake clamp");
            }
            for hz in [120, 240] {
                let value = run(hz, lean, speed, normal_z);
                assert!((value - stock).abs() < 0.001, "{hz} Hz: {value} vs {stock}");
            }
        }
    }

    use super::*;
    use super::*;
    use super::*;
    use super::*;
    pub(crate) use crate::game::ai::ai::tests::oc286_save_block_setup;
    pub(crate) use crate::game::ai::ai::tests::rider_save_state_setup;
    pub(crate) use crate::game::ai::ai::tests::rider_state_save_setup;
    pub(crate) use crate::game::ai::ai::tests::rider_state_save_setup_2;

    /// `RiderGroundState_SaveState`: one stream write of 0x18 bytes.
    #[test]
    fn rider_ground_state_save_state_matches() {
        covered(
            ee::s01::f_0013f848__generated,
            rider_ground_state_save_state,
            200,
            &|c, seed| rider_save_state_setup(c, seed, 0x0013_f848),
        );
    }

    /// RiderRailslideState_SaveState: write(self, 0x50) through a fake.
    #[test]
    fn rider_railslide_state_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s01::f_0013c5e0__generated,
            rider_railslide_state_save_state,
            64,
            &|c, seed| rider_state_save_setup(c, seed, 0x0013_c5e0),
        );
    }

    #[test]
    fn rider_air_state_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s01::f_0013acb0__generated,
            rider_air_state_save_state,
            60,
            &|c, seed| rider_state_save_setup_2(c, seed, 0x13_acb0, 0x0013_acb0),
        );
    }

    #[test]
    fn rider_handplant_state_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s01::f_00139970__generated,
            rider_handplant_state_save_state,
            200,
            &|c, seed| oc286_save_block_setup(c, seed, 0x13_9970, 0x0013_9970),
        );
    }
    use super::*;

    /// RiderAirState_PlayLandAnim2: vertical speeds around the hard and
    /// medium thresholds and random, spin angles below -pi, above pi and in
    /// between; the real AnimController_PlayAnim on the controller fixture
    /// of the RiderAirState_PlayLandAnim test.
    #[test]
    fn rider_air_state_play_land_anim2_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::GAME_GP;
        use crate::game::ai::ai::tests::{slot, LOAD_B0};
        check(
            ee::s01::f_0013a968__generated,
            rider_air_state_play_land_anim2,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x13a968, 0);
                let state = DATA;
                let rider = DATA + 0x1000;
                let anim = DATA + 0x4000;
                let vt = DATA + 0x5000;
                c.bus
                    .write32(GAME_GP - LAND2_HARD_SPEED_GP, (-2222.2222f32).to_bits());
                c.bus
                    .write32(GAME_GP - LAND2_PI_GP, std::f32::consts::PI.to_bits());
                c.bus
                    .write32(GAME_GP - LAND2_MEDIUM_SPEED_GP, (-1250.0f32).to_bits());
                c.f[12] = match seed % 4 {
                    0 => (-2222.3f32 + (seed % 5) as f32).to_bits(),
                    1 => (-1251.0f32 + (seed % 3) as f32).to_bits(),
                    2 => r.f32(3000.0),
                    _ => r.edge_f32(),
                };
                c.bus.write32(state + LAND2_RIDER, rider);
                c.bus.write32(rider + LAND2_RIDER_ANIM, anim);
                c.bus.write32(anim + 0x58, vt);
                slot(c, vt, 0x20, 0, LOAD_B0);
                c.bus.write32(anim + 0xb0, 0x207);
                let spin = match (seed / 4) % 4 {
                    0 => -3.5f32 - (seed % 7) as f32,
                    1 => 3.5 + (seed % 7) as f32,
                    2 => ((seed % 61) as f32 - 30.0) / 10.0,
                    _ => f32::from_bits(r.edge_f32()),
                };
                c.bus.write32(rider + LAND2_RIDER_SPIN, spin.to_bits());
                c.g[A0][0] = sx(state);
                c.pc = 0x0013_a968;
            },
        );
    }
}
