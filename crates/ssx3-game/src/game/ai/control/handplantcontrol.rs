//! `src/ai/control/handplantcontrol.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
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
use crate::call::Frame;
use crate::events;
use crate::guest::Addr;
use crate::abi;
use crate::abi::r32;
use crate::call::sp;
use crate::ps2f::FpuF32;
use crate::call::VSlot;
use crate::call;
use ssx3_ee_rt::funcs::call_indirect;
pub(crate) use crate::game::ai::rider::r;
pub(crate) use crate::game::ai::rider::p;
pub(crate) use crate::game::ai::rider::at;
pub(crate) use crate::game::ai::rider::gp_2 as gp;
pub(crate) use crate::game::ai::rider::sp_2;
pub(crate) use crate::game::ai::rider::slt;
pub(crate) use crate::game::ai::rider::Ops;
pub(crate) use crate::game::ai::rider::epilogue;
pub(crate) use crate::game::ai::rider::prologue;
pub(crate) use crate::game::ai::rider::vcall;
pub(crate) use crate::game::ai::rider::RODATA_UP;
pub(crate) use crate::game::ai::rider::RODATA_NORMAL2;
pub(crate) use crate::game::ai::rider::copy_contact;
pub(crate) use crate::game::intersect::worldsphtree::sp_2 as sp_3;
pub(crate) use crate::game::ai::computer::gp_2 as gp_3;
pub(crate) use crate::game::ai::computer::lw;
pub(crate) use crate::game::ai::computer::fr_2 as fr;
pub(crate) use crate::game::ai::computer::setf_2 as setf;
pub(crate) use crate::game::ai::computer::ldf_2 as ldf;
pub(crate) use crate::game::ai::computer::li_f;
pub(crate) use crate::game::ai::computer::lt_2 as lt;
pub(crate) use crate::game::movie::movieplayer::gp as gp_5;
pub(crate) use crate::game::fe::festateload::at as at_2;
pub(crate) use crate::game::ai::ai::virtual_call_slot;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::gp as gp_7;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_8;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0013_99a8, rider_handplant_state_load_state, "RiderHandplantState_LoadState"),
    (0x0013_9a18, rider_air_state_exit, "RiderAirState_Exit"),
    (0x0013_9a20, rider_air_state_update, "RiderAirState_Update"),
    (0x0013_a7b0, rider_air_state_cast_board_ray, "RiderAirState_CastBoardRay"),
    (0x0013_a8f8, rider_air_state_play_land_anim, "RiderAirState_PlayLandAnim"),
    (0x0013_ace8, rider_air_state_load_state, "RiderAirState_LoadState"),
    (0x0013_ad20, rider_railslide_state_enter, "RiderRailslideState_Enter"),
    (0x0013_c618, rider_railslide_state_load_state, "RiderRailslideState_LoadState"),
    (0x0013_c878, rider_ground_state_compute_friction_accel, "RiderGroundState_ComputeFrictionAccel"),
    (0x0013_d028, rider_ground_state_compute_carve_accel, "RiderGroundState_ComputeCarveAccel"),
    (0x0013_d1b8, rider_ground_state_probe_ground, "RiderGroundState_ProbeGround"),
    (0x0013_d818, rider_ground_state_update, "RiderGroundState_Update"),
    (0x0013_f880, rider_ground_state_load_state, "RiderGroundState_LoadState"),
    (0x0014_0738, rider_init_data, "Rider_InitData"),
    (0x0014_0bc8, rider_is_computer, "Rider_IsComputer"),
    (0x0014_4670, flags64_move_to_second, "Flags64_MoveToSecond"),
    (0x0014_46a0, flags64_test, "Flags64_Test"),
    (0x0014_46b8, flags64_test_second, "Flags64_TestSecond"),
    (0x0014_48a8, rider_is_control_mode3, "Rider_IsControlMode3"),
];

/// Direct call targets, as the generated body calls them (the shard alias,
/// which dispatches through its override slot).
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::GET_MAIN_STATE;
    pub(crate) use crate::game::ai::rider::callee::SET_MAIN_STATE;
    pub(crate) use crate::game::ai::rider::callee::GET_SUB_STATE;
    pub(crate) use crate::game::ai::rider::callee::SET_SUB_STATE;
    pub(crate) use crate::game::ai::rider::callee::IS_ON_GROUND;
    pub(crate) use crate::game::ai::rider::callee::HANDLE_WALL_RAIL_CONTACT;
    pub(crate) use crate::game::ai::rider::callee::FIND_WALL_CONTACT;
    pub(crate) use crate::game::ai::rider::callee::BOUNCE_OFF_WALL;
    pub(crate) use crate::game::ai::rider::callee::FIND_RAIL;
    pub(crate) use crate::game::ai::rider::callee::TRANSLATE;
    pub(crate) use crate::game::ai::rider::callee::DEFLECT_ALONG_WALL;
    pub(crate) use crate::game::ai::rider::callee::BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::ai::rider::callee::HANDLE_IMPACT;
    pub(crate) use crate::game::ai::rider::callee::BANK_TRICK_BOOST;
    pub(crate) use crate::game::ai::rider::callee::LAND_TRICK;
    pub(crate) use crate::game::ai::rider::callee::ADD_BOOST;
    pub(crate) use crate::game::ai::rider::callee::ALIGN_STANCE_TO_RAIL;
    pub(crate) use crate::game::ai::rider::callee::ROTATE_AROUND_AXIS;
    pub(crate) use crate::game::ai::rider::callee::GET_BUMP_STRENGTH;
    pub(crate) use crate::game::ai::rider::callee::AI_GET_FRAME_COUNTER_2 as AI_GET_FRAME_COUNTER;
    pub(crate) use crate::game::ai::rider::callee::RESOLVE_RIDER_COLLISION;
    pub(crate) use crate::game::ai::rider::callee::COLL_SEGMENT_CTOR;
    pub(crate) use crate::game::ai::rider::callee::COLL_HULL_CTOR;
    pub(crate) use crate::game::ai::rider::callee::COLL_HULL_DTOR;
    pub(crate) use crate::game::ai::rider::callee::OVERLAP_LIST_COLLIDE_VOLUME;
    pub(crate) use crate::game::ai::rider::callee::OVERLAP_LIST_COLLIDE_OBJECTS;
    pub(crate) use crate::game::ai::rider::callee::OVERLAP_LIST_FIND_NEAREST_RAIL;
    pub(crate) use crate::game::ai::rider::callee::CAMERA_TRIGGER_UPDATE_FOR_RIDER;
    pub(crate) use crate::game::ai::rider::callee::PLAY_WALL_HIT_SOUND;
    pub(crate) use crate::game::ai::rider::callee::MATH_ATAN;
    pub(crate) use crate::game::ai::rider::callee::MATH_SIN_COS;
    pub(crate) use crate::game::ai::rider::callee::ANIM_PLAY_ANIM;
    pub(crate) use crate::game::ai::rider::callee::ANIM_ROTATE_ROOTS;
    pub(crate) use crate::game::ai::rider::callee::ANIM_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::ai::rider::callee::ANIM_LAYER_SEQUENCE;
    pub(crate) use crate::game::ai::rider::callee::FLAGS64_TEST;
    pub(crate) use crate::game::ai::rider::callee::RAILSLIDE_PLAY_ENTER_ANIM;
    pub(crate) use crate::game::ai::rider::callee::RAILSLIDE_COMPUTE_ENTRY_ANGLE;
    pub(crate) use crate::game::ai::rider::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::ai::rider::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::ai::rider::callee::BE_CHAR_ATTRIB5_FACTOR;
    pub(crate) use crate::game::ai::rider::callee::SPHERE_TREE_INTERSECT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WP_DATA_FIND_CELL_INDEX;
    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_BUILTIN;
    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_PAD_CHANNEL;
    pub(crate) use crate::game::intersect::worldsphtree::callee::FOCUS_MAN_COLLECT_OCTREE_NODE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX;
    pub(crate) use crate::game::intersect::worldsphtree::callee::COLLIDE_VOLUME_OBJECT_2 as COLLIDE_VOLUME_OBJECT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::RAIL_MODIFIER_FIND_CLOSEST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::FLEX_RAIL_FIND_CLOSEST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::RAIL_FIND_NEAREST_POINT;
    pub(crate) use crate::game::ai::computer::callee::GET_MAIN_STATE as GET_MAIN_STATE_2;
    pub(crate) use crate::game::ai::computer::callee::GET_SUB_STATE as GET_SUB_STATE_2;
    pub(crate) use crate::game::ai::computer::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL8_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL16_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL32_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL64_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL128_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL256_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::POOL512_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::REMOVE_HUD_MESSAGE;
    pub(crate) use crate::game::movie::movieplayer::callee::SET_TIMED_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::PACK_POSITION;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILD_OCCLUDER_PLANES;
    pub(crate) use crate::game::movie::movieplayer::callee::MATH_DET3X3;
    pub(crate) use crate::game::movie::movieplayer::callee::SHOW_HUD_NUMBER;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_CHAR_ATTRIB_FACTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::movie::movieplayer::callee::AUDIO_RESET_BOOST_PITCH;
    pub(crate) use crate::game::movie::movieplayer::callee::QSORT;
    pub(crate) use crate::game::movie::movieplayer::callee::VU0_UPLOAD_OCCLUDER;
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_APPLY_WIND;
    pub(crate) use crate::game::movie::movieplayer::callee::AIR_PREDICTOR_UPDATE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_GET_SUBSTATE;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_UPDATE_ORIENTATION;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_ORIENTATION_IMPLICIT;
    pub(crate) use crate::game::movie::movieplayer::callee::OVERLAP_LIST_FIND_CLOSEST;


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
    pub(crate) use crate::game::ai::rider::contact::SIZE;
    pub(crate) use crate::game::ai::rider::contact::NORMAL;
    pub(crate) use crate::game::ai::rider::contact::DEPTH;
    pub(crate) use crate::game::ai::rider::contact::SURFACE;
    pub(crate) use crate::game::ai::rider::contact::INSTANCE;
    pub(crate) use crate::game::ai::rider::contact::MATERIAL;
}

/// Rider fields this function reads or writes.
pub(crate) mod rider {
    pub(crate) use crate::game::ai::rider::rider::VEL;
    pub(crate) use crate::game::ai::rider::rider::FWD_REF;
    pub(crate) use crate::game::ai::rider::rider::POS;
    pub(crate) use crate::game::ai::rider::rider::UP;
    pub(crate) use crate::game::ai::rider::rider::UP_PREV;
    pub(crate) use crate::game::ai::rider::rider::FRAME_FWD;
    pub(crate) use crate::game::ai::rider::rider::FRAME_SIDE;
    pub(crate) use crate::game::ai::rider::rider::GROUND_NORMAL2;
    pub(crate) use crate::game::ai::rider::rider::GROUND_ID;
    pub(crate) use crate::game::ai::rider::rider::GROUND_MATERIAL;
    pub(crate) use crate::game::ai::rider::rider::GROUND_HEIGHT;
    pub(crate) use crate::game::ai::rider::rider::GROUND_POINT;
    pub(crate) use crate::game::ai::rider::rider::OVERLAP_LIST;
    pub(crate) use crate::game::ai::rider::rider::OVERLAP_HINT;
    pub(crate) use crate::game::ai::rider::rider::SPHERE_TREE;
    pub(crate) use crate::game::ai::computer::rider::VELOCITY_2 as VELOCITY;
    pub(crate) use crate::game::ai::computer::rider::SPEED;
    pub(crate) use crate::game::ai::computer::rider::HEADING_2 as HEADING;
    pub(crate) use crate::game::ai::computer::rider::BASE_2 as BASE;
    pub(crate) use crate::game::ai::computer::rider::AIR_PREDICTOR;
    pub(crate) use crate::game::ai::computer::rider::PLAYER;
    pub(crate) use crate::game::ai::computer::rider::ATTRIB_VALUE;
    pub(crate) use crate::game::ai::computer::rider::SPEED_FACTOR;
    pub(crate) use crate::game::ai::computer::rider::BOOST_2 as BOOST;
    pub(crate) use crate::game::ai::computer::rider::BOOST_TIMER;
    pub(crate) use crate::game::ai::computer::rider::BOOST_COUNT;
    pub(crate) use crate::game::ai::computer::rider::BOOST_STATE;
    pub(crate) use crate::game::ai::computer::rider::BOOST_MASK;
    pub(crate) use crate::game::ai::computer::rider::BOOST_FLAG;
    pub(crate) use crate::game::ai::computer::rider::CHAR_ID;
    pub(crate) use crate::game::ai::computer::rider::CHAR_COMP;
    pub(crate) use crate::game::ai::computer::rider::METRIC_SCALE;

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
    pub(crate) use crate::game::ai::computer::computer::RIDER;
    pub(crate) use crate::game::ai::computer::computer::DIR;
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

    /// Delta time of the frame.
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
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
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
                            c.mul_s(20, 12, 0);
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
                                if call_ptr(c, callee::RIDER_UPDATE_ORIENTATION, 0x0013_9c50).is_err() {
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
                                if call_ptr(c, callee::RIDER_UPDATE_ORIENTATION, 0x0013_9c64).is_err() {
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP as STACK;
    use crate::difftest::fakes;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::SP;
    use crate::difftest::fakes::vtable;
    use crate::difftest::check_lohi;
    use ssx3_ee as ee;
    use ssx3_ee::s01;
    use ssx3_ee::s03;
    use ssx3_ee::s00;
    use ssx3_ee::s02;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::elf_image;
    use crate::abi;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    use crate::difftest::fakes::ptr;
    pub(crate) use crate::game::ai::rider::tests::SEEDS;
    pub(crate) use crate::game::ai::rider::tests::f;
    pub(crate) use crate::game::ai::rider::tests::uf_2 as uf;
    pub(crate) use crate::game::ai::rider::tests::put_v;
    pub(crate) use crate::game::ai::rider::tests::rand_v;
    pub(crate) use crate::game::ai::rider::tests::gp_f32;
    pub(crate) use crate::game::ai::rider::tests::VT_SUB;
    pub(crate) use crate::game::ai::rider::tests::VT_OWNER;
    pub(crate) use crate::game::ai::rider::tests::VT_VOL;
    pub(crate) use crate::game::ai::rider::tests::VT_HULL;
    pub(crate) use crate::game::ai::rider::tests::OWNER;
    pub(crate) use crate::game::ai::rider::tests::SURFS;
    pub(crate) use crate::game::ai::rider::tests::MATS;
    pub(crate) use crate::game::ai::rider::tests::INSTS;
    pub(crate) use crate::game::ai::rider::tests::LIST;
    pub(crate) use crate::game::ai::rider::tests::OBJS;
    pub(crate) use crate::game::ai::rider::tests::RIDER_2 as RIDER;
    pub(crate) use crate::game::ai::rider::tests::BASE;
    pub(crate) use crate::game::ai::rider::tests::PARAMS;
    pub(crate) use crate::game::ai::rider::tests::TREE;
    pub(crate) use crate::game::ai::rider::tests::fake_varied;
    pub(crate) use crate::game::ai::rider::tests::put_slot;
    pub(crate) use crate::game::ai::rider::tests::setup_machine;
    pub(crate) use crate::game::ai::rider::tests::put_contacts;
    pub(crate) use crate::game::ai::rider::tests::put_list;
    pub(crate) use crate::game::ai::rider::tests::put_rider;
    pub(crate) use crate::game::ai::computer::tests::literals;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festateoptions::tests::rng as rng_2;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::ai::ai::tests::rng as rng_3;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::LOAD_B0;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::ai::tests::slot;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base;


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
        check(ssx3_ee::s01::f_0013d028__generated, rider_ground_state_compute_carve_accel, 300, &|c, seed| {
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
            c.bus.write32(rider + rider::ATTRIB_VALUE, r.next_u32() % 400);
            c.bus.write32(rider + rider::SPEED_FACTOR, if seed % 3 == 0 { 0 } else { r.edge_f32() });
            c.bus.write8(0x0053_5538 + 3, (r.next_u32() % 250) as u8);
            c.bus.write8(0x0053_08d8 + 8 + 3, 1 + (r.next_u32() % 250) as u8);
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
            c.f[12] = if seed % 4 == 0 { r.edge_f32() | 0x8000_0000 } else { r.edge_f32() };
            c.f[13] = r.edge_f32();
            c.f[14] = match seed % 4 {
                0 => r.edge_f32() | 0x8000_0000,
                1 => 0,
                _ => r.edge_f32(),
            };
        });
    }


    #[test]
    fn flags64_test_matches() {
        check(ssx3_ee::s01::f_001446a0__generated, flags64_test, 200, &|c, seed| {
            let mut r = Rng(0x51ed_2701 ^ seed as u64);
            scramble(c, &mut r);
            let at = DATA + (seed % 8) * 0x10;
            c.g[A0][0] = sx(at);
            c.bus.write64(at, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
            // 0..63, plus out-of-range shifts (masked to 0..63).
            c.g[A1][0] = if seed % 5 == 0 { 64 + (seed % 9) as u64 } else { (r.next_u32() % 64) as u64 };
        });
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
        c.bus.write32(GAME_GP.wrapping_sub(0x70e8), (-1.0f32 / 30.0).to_bits());
        let (a, b): (f32, f32) = match seed % 5 {
            0 => (2.0 * m4, 0.5), // a > 0, b > 0
            1 => (2.0 * m4, -0.5), // a > 0, b <= 0
            2 => (-0.5 * m4, 0.5), // -m4 < a <= 0
            3 => (-1.5 * m4, -0.5), // a <= -m4, clamp inactive
            _ => (-3.0 * m8, 1.0), // a <= -m8, clamp to -m8
        };
        c.f[12] = a.to_bits();
        c.f[13] = b.to_bits();
    }


    #[test]
    fn rider_is_control_mode3_matches() {
        check(ssx3_ee::s01::f_001448a8__generated, rider_is_control_mode3, 300, &|c, seed| {
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
        });
    }

    #[test]
    fn rider_air_state_update_matches() {
        check(ee::s01::f_00139a20__generated, rider_air_state_update, 160, &|c, seed| {
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
            let state = match seed % 4 { 0 => 2u32, 1 => 3, 2 => 1, _ => 3 };
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
            c.bus.write32((GAME_GP as i32 - 0x71d4) as u32, (1.0f32 / 60.0).to_bits());
            c.bus.write32((GAME_GP as i32 - 0x71d0) as u32, (1.0f32 / 60.0).to_bits());
            c.bus.write32((GAME_GP as i32 - 0x71cc) as u32, 60.0f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x71c8) as u32, 1.396f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x71c4) as u32, 0.0872f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x71c0) as u32, 0.3f32.to_bits());
            // The predictor's internal step budget and rates.
            c.bus.write32((GAME_GP as i32 - 0x7c5c) as u32, (1.0f32 / 60.0).to_bits());
            c.bus.write32((GAME_GP as i32 - 0x7c74) as u32, 2.0f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x7c70) as u32, 1.0f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x7c6c) as u32, 1.0f32.to_bits());
            c.bus.write32((GAME_GP as i32 - 0x7c60) as u32, 0.0f32.to_bits());
        });
    }

    #[test]
    fn rider_air_state_cast_board_ray_matches() {
        check(ee::s01::f_0013a7b0__generated, rider_air_state_cast_board_ray, 120, &|c, seed| {
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
            c.bus.write32((GAME_GP as i32 - 0x71a4) as u32, 0.575f32.to_bits());
        });
    }


    #[test]
    fn flags64_test_second_matches() {
        check(ee::s01::f_001446b8__generated, flags64_test_second, SEEDS_3, &|c, seed| {
            let mut r = rng(0x0014_46b8, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let flags = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.bus.write64(DATA + 8, flags);
            c.g[A1][0] = sx((seed % 70) as u32);
        });
    }


    #[test]
    fn flags64_move_to_second_matches() {
        check(s01::f_00144670__generated, flags64_move_to_second, 200, &|c, seed| {
            let mut r = Rng(0x1446_70aa ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(seed);
            c.bus.write64(OBJ, r.next_u32() as u64 | (r.next_u32() as u64) << 32);
            c.bus.write64(OBJ + 8, r.next_u32() as u64 | (r.next_u32() as u64) << 32);
        });
    }


    pub(crate) fn put(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }


    // ------------------------------------------------------------------
    // State predicates.

    #[test]
    fn rider_is_computer_matches() {
        check(ee::s01::f_00140bc8__generated, rider_is_computer, 200, &|c, seed| {
            let mut r = setup(c, seed, 18);
            c.g[A0][0] = AUX as u64;
            put(c, AUX + 0x874, if seed % 3 == 0 { 0 } else if seed % 3 == 1 { 1 } else { r.next_u32() });
        });
    }


    #[test]
    fn rider_air_state_exit_matches() {
        leaf(ssx3_ee::s01::f_00139a18__generated, rider_air_state_exit, 11);
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
    pub(crate) fn check_strict(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
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
        check(ssx3_ee::s01::f_0013a8f8__generated, rider_air_state_play_land_anim, SEEDS_5, &|c, seed| {
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
        });
    }


    #[test]
    fn rider_railslide_state_enter_matches() {
        check(ssx3_ee::s01::f_0013ad20__generated, rider_railslide_state_enter, 8, &|c, seed| {
            let _r = base(c, seed, 0x13ad20, 2);
            let this = DATA + 0x1000;
            let rider = DATA + 0x4000;
            c.g[A0][0] = this as u64;
            w_2(c, this + 0x50, rider);
            put_vec(c, rider + 0x110, [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 0]);
            w_2(c, rider + 0x10, 0);
            w_2(c, rider + 0x770, 0);
            w_2(c, GAME_GP.wrapping_sub(0x7184), 0.05f32.to_bits());
        });
    }


    #[test]
    fn rider_init_data_matches() {
        check(ee::s01::f_00140738__generated, rider_init_data, 60, &|c, seed| {
            let _ = machine(c, seed, 0x14_0738, 0);
            c.g[A0][0] = DATA as u64;
        });
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


    #[test]
    fn rider_ground_state_update_matches() {
        for seed in 0..SEEDS {
            let r = compare(ee::s01::f_0013d818__generated, rider_ground_state_update, &|c| {
                setup_2(c, seed)
            });
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lo/hi: {:#?}", r.lohi);
        }
    }
}
