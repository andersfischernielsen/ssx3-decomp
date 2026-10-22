//! `src/camera/trigger/cameratriggerman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::call_ptr;
use crate::call::sp;
use crate::call::Frame;
use crate::ps2f::FpuF32;
use crate::abi::r32;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::funcs::call_indirect;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::copy_matrix;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::mul_matrix;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::mul_matrix_vec;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::rot_axis;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::r;
pub(crate) use crate::game::camera::script::scriptcontroller::IDENTITY_MAT;
pub(crate) use crate::game::camera::script::scriptcontroller::AXIS_X;
pub(crate) use crate::game::camera::script::scriptcontroller::AXIS_Z;
pub(crate) use crate::game::camera::script::scriptcontroller::AXIS_Y;
pub(crate) use crate::game::camera::script::scriptcontroller::quad_copy;
pub(crate) use crate::game::camera::script::scriptcontroller::copy_matrix as copy_matrix_2;
pub(crate) use crate::game::camera::script::scriptcontroller::mat_vec;
pub(crate) use crate::game::camera::script::scriptcontroller::mat_mul;
pub(crate) use crate::game::camera::script::scriptcontroller::rot_axis as rot_axis_2;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw;
pub(crate) use crate::game::animation::rideranimbase::sw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_c9b0, camera_trigger_man_unload, "CameraTriggerMan_Unload"),
    (0x0016_cf18, camera_trigger_man_empty_global_call, "CameraTriggerMan_EmptyGlobalCall"),
    (0x0016_d210, camera_trigger_stack_ctor, "CameraTriggerStack_ctor"),
    (0x0016_d238, camera_trigger_man_push_trigger_stack, "CameraTriggerMan_PushTriggerStack"),
    (0x0016_d260, camera_trigger_man_pop_trigger_stack, "CameraTriggerMan_PopTriggerStack"),
    (0x0016_d320, camera_trigger_man_update_for_rider, "CameraTriggerMan_UpdateForRider"),
    (0x0017_0240, camera_line_bound_world_start, "CameraLineBoundObj_GetWorldStart"),
    (0x0017_07b0, camera_line_bound_world_end, "CameraLineBoundObj_GetWorldEnd"),
    (0x0017_0d20, camera_bound_obj_get_world_point, "CameraBoundObj_GetWorldPoint"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MATH_SIN_COS;
    pub(crate) use crate::game::camera::script::scriptcontroller::callee::MATH_SIN_COS as MATH_SIN_COS_2;
    pub(crate) use crate::game::camera::script::scriptcontroller::callee::MAT4_TO_POS_QUAT;
    pub(crate) use crate::game::camera::script::scriptcontroller::callee::CAMERA_WOBBLE_GET_OFFSET;
    pub(crate) use crate::game::camera::script::scriptcontroller::callee::CAMERA_WOBBLE_PHASE;
    pub(crate) use crate::game::be::beintmission::callee::WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::be::beintmission::callee::UITEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::be::beintmission::callee::MPEG2_ERROR;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_COUNT;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_NTH;
    pub(crate) use crate::game::be::beintmission::callee::REWARDS_FILE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_EMPTY_STUB;
    pub(crate) use crate::game::be::beintmission::callee::UBER_TRICK_REFRESH;
    pub(crate) use crate::game::be::beintmission::callee::MEMCARD_SCAN_STEP;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_ON_WIDGET_ACTION;
    pub(crate) use crate::game::be::beintmission::callee::VIEW_POPUP_SHOW_MESSAGE;
    pub(crate) use crate::game::be::beintmission::callee::BUY_ATTRIB_UPDATE_BARS;
    pub(crate) use crate::game::be::beintmission::callee::CAREER_STATS_SHOW_HIGHLIGHTS;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2 as SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID as SCRIPT_ENGINE_GET_SEQ_PROCESS_ID_2;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR as MULTI_PARTICLE_DTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;


    pub const CLEAR_FLAGS: Callee = Callee {
        addr: 0x0016_c678,
        f: ee::s02::f_0016c678,
    };

    pub const FIND: Callee = Callee {
        addr: 0x0016_c6b0,
        f: ee::s02::f_0016c6b0,
    };

    pub const ADD: Callee = Callee {
        addr: 0x0016_c718,
        f: ee::s02::f_0016c718,
    };

    pub const FIND_BY_ID: Callee = Callee {
        addr: 0x0016_c6e0,
        f: ee::s02::f_0016c6e0,
    };

    pub const REMOVE: Callee = Callee {
        addr: 0x0016_c778,
        f: ee::s02::f_0016c778,
    };

    pub const POP_TRIGGER: Callee = Callee {
        addr: 0x0016_d260,
        f: ee::s02::f_0016d260,
    };

    pub const PUSH_TRIGGER: Callee = Callee {
        addr: 0x0016_d238,
        f: ee::s02::f_0016d238,
    };

    pub const APPLY_ACTION: Callee = Callee {
        addr: 0x0016_e1d8,
        f: ee::s02::f_0016e1d8,
    };

    pub const COLLECT_NODE: Callee = Callee {
        addr: 0x0017_6890,
        f: ee::s02::f_00176890,
    };

    pub const COLLECT_BOX: Callee = Callee {
        addr: 0x0017_4848,
        f: ee::s02::f_00174848,
    };

    pub const CLASSIFY_AABB: u32 = 0x0032_8360;
}

/// Return addresses of the four `GetView` bodies: the addresses their calls
/// come back to, relative to the function's entry.
pub(crate) mod ret {
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::WOBBLE;
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::PHASE;
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::ROT_X;
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::ROT_Z;
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::ROT_Y;
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::TO_POS_QUAT;
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::LINE_ROT_X;
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::LINE_ROT_Z;
    pub(crate) use crate::game::camera::script::scriptcontroller::ret::LINE_ROT_Y;
}

/// Fields of a line-bound object (the matrix source).
pub(crate) mod line_bound {

    /// Translation (x, y, z) at +0, the three rotation angles and the scale
    /// (x, y, z) at +0x18, then the local start point at +0x2c and the end
    /// point at +0x38.
    pub const TRANSLATION: u32 = 0x0;

    pub const ROT_Z: u32 = 0xc;

    pub const ROT_X: u32 = 0x10;

    pub const ROT_Y: u32 = 0x14;

    pub const SCALE: u32 = 0x18;
}


// ---------------------------------------------------------------------------
// Camera triggers.

/// `void CameraTriggerMan_PushTriggerStack(CameraTriggerMan *man, uint value)`
/// (0x16d238): write `value` into `man->stack[man->count]` and at +0x4, then
/// bump the count.
pub fn camera_trigger_man_push_trigger_stack(c: &mut Cpu) {
    let man = Addr(abi::arg(c, 0));
    let value = c.g[A1][0];
    c.cycles += 9;
    let count = man.lw(c) as u32;
    let slot = man.0.wrapping_add(count << 2);
    c.g[V0][0] = sx(slot);
    Addr(slot).off(0x8).sw(c, value);
    man.off(0x4).sw(c, value);
    let count = man.lw(c) as u32;
    c.g[V0][0] = sx(count.wrapping_add(1));
    man.sw(c, c.g[V0][0]);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Camera bound object.

/// `void CameraBoundObj_GetWorldPoint(Vec4* out, void* src, int corner)`
/// (0x170d20): writes the world-space position of one of the object's eight
/// corners (`src + 0x28 + corner*12`) through the matrix built from `src`'s
/// translation, the three angles at `+0x10`, `+0x14`, `+0xc` and the scale at
/// `+0x18`.
pub fn camera_bound_obj_get_world_point(c: &mut Cpu) {
    guest(c, camera_bound_obj_get_world_point_body)
}


pub(crate) fn camera_bound_obj_get_world_point_body(c: &mut Cpu) -> Flow {
    c.cycles += 355;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
    let sp = c.g[SP][0] as u32;
    let out = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    let corner_index = abi::arg(c, 2);

    c.g[V0][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    let identity = c.g[V0][0] as u32;
    c.sq(16, sp.wrapping_add(0x140));
    c.sq(18, sp.wrapping_add(0x120));
    c.g[S2][0] = sx(corner_index);
    c.sq(20, sp.wrapping_add(0x100));
    c.sq(17, sp.wrapping_add(0x130));
    c.sq(19, sp.wrapping_add(0x110));
    c.sd(sp.wrapping_add(0xf0), c.g[RA][0]);
    c.swc1(20, sp.wrapping_add(0x150));

    // m = identity, then m = m * translate(src->pos).
    c.lqc2(1, identity);
    c.lqc2(2, identity.wrapping_add(0x10));
    c.lqc2(3, identity.wrapping_add(0x20));
    c.lqc2(4, identity.wrapping_add(0x30));
    c.sqc2(1, sp);
    c.sqc2(2, sp.wrapping_add(0x10));
    c.sqc2(3, sp.wrapping_add(0x20));
    c.sqc2(4, sp.wrapping_add(0x30));
    c.lwc1(0, src);
    c.lwc1(1, src.wrapping_add(0x4));
    c.lwc1(2, src.wrapping_add(0x8));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 20);
    c.swc1(0, sp.wrapping_add(0x40));
    c.swc1(1, sp.wrapping_add(0x44));
    c.swc1(2, sp.wrapping_add(0x48));
    c.swc1(20, sp.wrapping_add(0x4c));
    mul_matrix_vec(c, sp.wrapping_add(0x30), sp, sp.wrapping_add(0x40));

    // Three axis rotations: x, then y, then z (angles not negated here).
    rot_axis(c, false, sp.wrapping_add(0x50), src, 0x10, 0x004f_f140, sp.wrapping_add(0xd0), sp.wrapping_add(0xd4), 0x0017_0dd4)?;
    rot_axis(c, false, sp.wrapping_add(0x40), src, 0x14, 0x004f_f150, sp.wrapping_add(0xd8), sp.wrapping_add(0xdc), 0x0017_0f1c)?;
    rot_axis(c, false, sp.wrapping_add(0x40), src, 0xc, 0x004f_f160, sp.wrapping_add(0xe0), sp.wrapping_add(0xe4), 0x0017_1064)?;

    // m = m * scale(src->scale).
    let work = sp.wrapping_add(0x90);
    c.lwc1(2, src.wrapping_add(0x18));
    c.lwc1(1, src.wrapping_add(0x1c));
    c.lwc1(0, src.wrapping_add(0x20));
    c.swc1(2, sp.wrapping_add(0x80));
    c.swc1(1, sp.wrapping_add(0x84));
    c.swc1(0, sp.wrapping_add(0x88));
    c.swc1(20, sp.wrapping_add(0x8c));
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

    // out = m * (src + 0x28 + corner*12), a 128-bit store.
    c.g[V1][0] = 0xc;
    c.g[V0][0] = sx(out);
    c.mult(0, 4, 18, 3);
    c.lq(17, sp.wrapping_add(0x130));
    c.lq(19, sp.wrapping_add(0x110));
    c.g[RA][0] = c.ld(sp.wrapping_add(0xf0));
    c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(src));
    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
    c.lq(16, sp.wrapping_add(0x140));
    let corner = c.g[S2][0] as u32;
    c.lwc1(0, corner);
    c.swc1(0, sp.wrapping_add(0x40));
    c.lwc1(1, corner.wrapping_add(0x4));
    c.swc1(1, sp.wrapping_add(0x44));
    c.lwc1(0, corner.wrapping_add(0x8));
    c.swc1(20, sp.wrapping_add(0x4c));
    c.swc1(0, sp.wrapping_add(0x48));
    mul_matrix_vec(c, sp.wrapping_add(0x50), sp, sp.wrapping_add(0x40));
    c.lq(18, sp.wrapping_add(0x120));
    c.lq(3, sp.wrapping_add(0x50));
    c.lwc1(20, sp.wrapping_add(0x150));
    c.sq(3, out);
    c.lq(20, sp.wrapping_add(0x100));
    c.g[SP][0] = sx(sp.wrapping_add(0x160));
    abi::ret(c);
    Ok(())
}


/// `m = m * diag(scale)`: each column of `m` scaled by one component of the
/// scale vector, through the stack scratch at `tmp`, then copied to `via`
/// and back into `m` (the original's double copy).
pub(crate) fn scale_columns(c: &mut Cpu, m: u32, scale: u32, tmp: u32, via: u32) {
    c.lqc2(1, scale);
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, m + 0x10 * i);
    }
    c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
    c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
    c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
    c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
    for i in 0..4u32 {
        c.sqc2(8 + i as usize, tmp + 0x10 * i);
    }
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, tmp + 0x10 * i);
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, via + 0x10 * i);
    }
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, via + 0x10 * i);
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, m + 0x10 * i);
    }
}


// ---------------------------------------------------------------------------
// Line-bound endpoints.

/// Shared body of `CameraLineBoundObj_GetWorldStart/End`: build the bound's
/// transform (translation, then rotations about X, Z and Y, then a
/// non-uniform scale) and transform `point` into world space at `out`.
pub(crate) fn camera_line_bound_world(c: &mut Cpu, out: u64, bound: u32, point: u32, base: u32) -> Flow {
    c.cycles += 348;
    let frame = Frame::open(c, 0x150);
    frame.save(c, S0, 0x130);
    frame.save(c, S1, 0x120);
    frame.save(c, S2, 0x110);
    frame.save(c, S3, 0x100);
    frame.save_ra(c, 0xf0);
    frame.save_fpr(c, 20, 0x140);

    let m = sp(c);
    copy_matrix_2(c, m, IDENTITY_MAT);

    // Translation: (x, y, z, 1) from the bound's first three words.
    c.f[0] = Addr(bound + line_bound::TRANSLATION).read_fpu(c).0;
    c.f[1] = Addr(bound + line_bound::TRANSLATION + 4).read_fpu(c).0;
    c.f[2] = Addr(bound + line_bound::TRANSLATION + 8).read_fpu(c).0;
    c.g[AT][0] = FpuF32::ONE.0 as u64;
    c.mtc1(1, 20);
    Addr(m + 0x40).write_fpu(c, FpuF32(c.f[0]));
    Addr(m + 0x44).write_fpu(c, FpuF32(c.f[1]));
    Addr(m + 0x48).write_fpu(c, FpuF32(c.f[2]));
    Addr(m + 0x4c).write_fpu(c, FpuF32::ONE);
    mat_vec(c, m, m + 0x40, m + 0x30);

    let a = Addr(bound + line_bound::ROT_X).read_fpu(c);
    rot_axis_2(c, m, m + 0x50, AXIS_X, a, m + 0xd0, m + 0xd4, base + ret::LINE_ROT_X)?;
    let b = Addr(bound + line_bound::ROT_Y).read_fpu(c);
    rot_axis_2(c, m, m + 0x40, AXIS_Z, b, m + 0xd8, m + 0xdc, base + ret::LINE_ROT_Z)?;
    let cc = Addr(bound + line_bound::ROT_Z).read_fpu(c);
    rot_axis_2(c, m, m + 0x40, AXIS_Y, cc, m + 0xe0, m + 0xe4, base + ret::LINE_ROT_Y)?;

    // Scale: (sx, sy, sz, 1) multiplies the matrix columns.
    c.f[2] = Addr(bound + line_bound::SCALE).read_fpu(c).0;
    c.f[1] = Addr(bound + line_bound::SCALE + 4).read_fpu(c).0;
    c.f[0] = Addr(bound + line_bound::SCALE + 8).read_fpu(c).0;
    Addr(m + 0x80).write_fpu(c, FpuF32(c.f[2]));
    Addr(m + 0x84).write_fpu(c, FpuF32(c.f[1]));
    Addr(m + 0x88).write_fpu(c, FpuF32(c.f[0]));
    Addr(m + 0x8c).write_fpu(c, FpuF32::ONE);
    scale_columns(c, m, m + 0x80, m + 0x90, m + 0x40);

    // Transform the point (x, y, z, 1) and copy the result to `out`.
    c.f[2] = Addr(bound + point + 8).read_fpu(c).0;
    c.g[V0][0] = out;
    c.f[0] = Addr(bound + point).read_fpu(c).0;
    c.f[1] = Addr(bound + point + 4).read_fpu(c).0;
    Addr(m + 0x4c).write_fpu(c, FpuF32::ONE);
    Addr(m + 0x40).write_fpu(c, FpuF32(c.f[0]));
    Addr(m + 0x44).write_fpu(c, FpuF32(c.f[1]));
    Addr(m + 0x48).write_fpu(c, FpuF32(c.f[2]));
    mat_vec(c, m, m + 0x40, m + 0x50);
    frame.restore(c, S0, 0x130);
    quad_copy(c, m + 0x50, out as u32);
    frame.restore(c, S1, 0x120);
    frame.restore(c, S3, 0x100);
    frame.restore(c, S2, 0x110);
    frame.restore_ra(c, 0xf0);
    frame.restore_fpr(c, 20, 0x140);
    frame.close(c);
    Ok(())
}


/// `void CameraLineBoundObj_GetWorldStart(vec4 *out, LineBound *b)` (0x170240).
pub fn camera_line_bound_world_start(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let bound = abi::arg(c, 1);
        camera_line_bound_world(c, out, bound, 0x2c, 0x0017_0240)
    })
}


/// `void CameraLineBoundObj_GetWorldEnd(vec4 *out, LineBound *b)` (0x1707b0).
pub fn camera_line_bound_world_end(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let bound = abi::arg(c, 1);
        camera_line_bound_world(c, out, bound, 0x38, 0x0017_07b0)
    })
}


// ---------------------------------------------------------------------------
// CameraTriggerMan_EmptyGlobalCall.

/// `void CameraTriggerMan_EmptyGlobalCall(CameraTriggerMan* m)` (0x16cf18):
/// `CameraTriggerMan_EmptyStub(D_004C5830, m)`.
pub fn camera_trigger_man_empty_global_call(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004c_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5830));
        callee::CAMERA_TRIGGER_EMPTY_STUB.call(c, 0x0016_cf30)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Camera trigger stack and array element destructors.

/// `cCameraTriggerStack* CameraTriggerStack_ctor(cCameraTriggerStack* self)`
/// (0x16d210): clear the stack and return it.
pub fn camera_trigger_stack_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CAMERA_TRIGGER_STACK_CLEAR.call(c, 0x0016_d224)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void CameraTriggerMan_Unload(CameraTriggerMan* m)` (0x16c9b0): purge both
/// active lists and both stacks, remove every trigger, clear the data and
/// reset the chase controllers.
pub fn camera_trigger_man_unload(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
    if callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR
        .call(c, 0x0016_c9c8)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
    if callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR
        .call(c, 0x0016_c9d0)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
    if callee::CAMERA_TRIGGER_STACK_CLEAR_2
        .call(c, 0x0016_c9d8)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c));
    if callee::CAMERA_TRIGGER_STACK_CLEAR_2
        .call(c, 0x0016_c9e0)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    if callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS
        .call(c, 0x0016_c9e8)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    if callee::CAMERA_TRIGGER_MAN_CLEAR_DATA
        .call(c, 0x0016_c9f0)
        .is_err()
    {
        return;
    }
    if call_ptr(c, 0x0015_dc18, 0x0016_c9f8).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void CameraTriggerMan_streamIn(self, stream)` (0x16ce30): clean up, parse
/// the trigger list from the stream, clear each trigger's fired flag and
/// refresh the in-game mask.
pub fn camera_trigger_man_reload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        callee::CAMERA_TRIGGER_MAN_UNLOAD.call(c, 0x0016_ce4c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::CAMERA_TRIGGER_MAN_PARSE.call(c, 0x0016_ce58)?;
        c.g[V1][0] = lw_r(c, S1, 0x4);
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[A0][0] = Addr(low(c, S1)).lw(c);
            loop {
                c.cycles += 9;
                c.g[V0][0] = sx(low(c, A1) << 2);
                c.g[A1][0] = sx(low(c, A1).wrapping_add(1));
                c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
                c.g[V1][0] = Addr(low(c, V0)).lw(c);
                sw_r(c, V1, 0x20, 0);
                c.g[V0][0] = lw_r(c, S1, 0x4);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(low(c, S1)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0016_ce68);
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2.call(c, 0x0016_ce94)?;
        c.lq(16, sp_2(c).wrapping_add(0x20));
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `CameraTriggerMan_PopTriggerStack` (0x0016d260).
//
// `int CameraTriggerMan::PopTriggerStack(TriggerStack* s, int key, int* changed,
//  int* out)`: remove `key` from the stack; report whether the top changed.

pub fn camera_trigger_man_pop_trigger_stack(c: &mut Cpu) {
    c.cycles += 3;
    let stack = c.g[A0][0] as u32;
    let key = c.g[A1][0];
    let changed = c.g[A2][0] as u32;
    let out = c.g[A3][0] as u32;
    let mut t1 = lw(c, stack, 0);
    c.g[T1][0] = t1;
    c.g[T0][0] = 0;
    if t1 != 0 {
        c.cycles += 3;
        let mut i: u32 = 0;
        let mut v1 = stack.wrapping_add(8);
        let t2 = v1;
        c.g[V1][0] = sx(v1);
        c.g[T2][0] = sx(t2);
        loop {
            c.cycles += 3;
            let v0 = lw(c, v1, 0);
            if v0 != key {
                t1 = lw(c, stack, 0);
                c.g[T1][0] = t1;
            } else {
                c.cycles += 4;
                let mut j = i.wrapping_add(1);
                c.g[A1][0] = sx(j);
                let in_range = (j as u64) < t1;
                c.g[V0][0] = in_range as u64;
                c.g[V0][0] = sx(j << 2);
                if in_range {
                    c.cycles += 1;
                    v1 = (j << 2).wrapping_add(t2);
                    c.g[V1][0] = sx(v1);
                    loop {
                        c.cycles += 7;
                        let v = lw(c, v1, 0);
                        j = j.wrapping_add(1);
                        c.g[A1][0] = sx(j);
                        sw(c, v1.wrapping_sub(4), 0, v);
                        c.g[V0][0] = lw(c, stack, 0);
                        c.g[V0][0] = ((j as u64) < c.g[V0][0]) as u64;
                        let more = c.g[V0][0] != 0;
                        v1 = v1.wrapping_add(4);
                        c.g[V1][0] = sx(v1);
                        if !more {
                            break;
                        }
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0016_d298);
                        }
                    }
                }
                c.g[A1][0] = sx(i.wrapping_add(1));
                c.cycles += 4;
                c.g[V0][0] = lw(c, stack, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                sw(c, stack, 0, c.g[V0][0]);
                break;
            }
            c.cycles += 4;
            c.g[T0][0] = sx(i.wrapping_add(1));
            i = i.wrapping_add(1);
            c.g[V0][0] = ((i as u64) < c.g[T1][0]) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(v1.wrapping_add(4));
            v1 = v1.wrapping_add(4);
            if !more {
                break;
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0016_d278);
            }
        }
    }
    c.cycles += 3;
    let count = lw(c, stack, 0);
    let v0 = sx((count as u32).wrapping_sub(1));
    if count == 0 {
        c.cycles += 3;
        sw(c, changed, 0, 0);
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    let top = lw(c, stack, 4);
    c.cycles += 6;
    let slot = (v0 as u32) << 2;
    let addr = (stack as u64).wrapping_add(slot as u64);
    let item = lw(c, addr as u32, 8);
    c.g[A0][0] = item;
    c.g[V1][0] = top;
    c.g[V0][0] = 1;
    if item != top {
        c.cycles += 2;
        sw(c, changed, 0, 1);
        sw(c, out, 0, item);
    } else {
        c.cycles += 2;
        sw(c, changed, 0, 0);
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Small helpers.

#[inline]
pub(crate) fn add32(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}


/// Where a value comes from at an instruction of the original: a register,
/// a word on the frame (`lw(off(sp))`), or a constant.
#[derive(Clone, Copy)]
pub(crate) enum Src_2 {
    Reg(usize),
    Stack(u32),
    Const(u64),
}


impl Src_2 {
    fn get(self, c: &mut Cpu) -> u64 {
        match self {
            Src_2::Reg(r) => c.g[r][0],
            Src_2::Stack(off) => Addr((c.g[SP][0] as u32).wrapping_add(off)).lw(c),
            Src_2::Const(v) => v,
        }
    }
}


/// What a pass leaves behind when it does not process its list.
#[derive(Clone, Copy)]
pub(crate) enum End {
    /// `a0 = *(sp + 0x44)` (trigger count).
    Count,
    /// `v0 = *(s6 + off)` (next category's flag).
    Field(u32),
    /// `v1 = *(sp + 0x48)` (count * 4).
    Stack48,
}


pub(crate) fn apply_end(c: &mut Cpu, end: End) {
    match end {
        End::Count => c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c),
        End::Field(off) => c.g[V0][0] = Addr(r32(c, S6).wrapping_add(off)).lw(c),
        End::Stack48 => c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x48)).lw(c),
    }
}


/// One category pass of `CameraTriggerMan_UpdateForRider`: test the
/// category's spatial key against the rider box, walk the trigger list
/// marking matching nodes (`*(node + 0xc) = marker`), then collect the
/// octree node's eight child trigger lists.
///
/// The original keeps its state in s0..s7/a0..a3/v0/v1; this helper does the
/// same (it is passed the descriptor and reads/writes those registers) so
/// the callees see exactly the registers the generated body leaves for
/// them, and so the event back edge can be delivered at `resume_pc`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn category_pass(
    c: &mut Cpu,
    presence_gate: bool,
    desc: Src_2,
    s0: Option<u32>,
    s1: Option<u32>,
    gate_cycles: u64,
    gate_ret: u32,
    box_a0: Src_2,
    box_t1: Src_2,
    child_rets: &[u32; 8],
    marker_reg: usize,
    type_reg: usize,
    loop_cycles: u64,
    resume_pc: u32,
    end: End,
    box_ret: u32,
    vtable_rets: [u32; 4],
    setup: Option<(usize, u64)>,
    setup_cycles: u64,
) -> Flow {
    // The first category tests the list head itself rather than a flag.
    if presence_gate {
        let d = desc.get(c);
        c.g[S5][0] = Addr((d as u32).wrapping_add(0x10)).lw(c);
        if c.g[S5][0] == 0 {
            c.cycles += 1;
            apply_end(c, end);
            return Ok(());
        }
    }

    c.cycles += gate_cycles;
    if let Some(off) = s0 {
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(off));
    }
    if let Some(off) = s1 {
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(off));
    }
    let d = desc.get(c);
    if !presence_gate {
        c.g[S5][0] = Addr((d as u32).wrapping_add(0x10)).lw(c);
    }
    c.g[A0][0] = d;
    c.g[RA][0] = sx(gate_ret);
    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    call_ptr(c, callee::CLASSIFY_AABB, gate_ret)?;

    // 1 = outside, 2 = overlapping/partial, 0 = inside.
    c.g[V1][0] = c.g[V0][0];
    let nonzero = c.g[V1][0] != 0;
    c.g[V0][0] = 2;
    if nonzero {
        c.cycles += 2;
        let differs = c.g[V1][0] != c.g[V0][0];
        apply_end(c, end);
        if !differs {
            c.cycles += 7;
            let a0 = box_a0.get(c);
            let t1 = box_t1.get(c);
            c.g[A0][0] = a0;
            c.g[T0][0] = c.g[S5][0];
            c.g[T1][0] = t1;
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S4][0];
            c.g[RA][0] = sx(box_ret);
            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            callee::COLLECT_BOX.call(c, box_ret)?;
            // `break` to the block after the box call: one cycle and the
            // category's end value (A0/V0/V1).
            c.cycles += 1;
            apply_end(c, end);
        }
        return Ok(());
    }

    // The list of trigger objects attached to this octree node.
    c.cycles += 3;
    c.g[S2][0] = Addr(r32(c, S5).wrapping_add(0x28)).lw(c);
    if c.g[S2][0] == 0 {
        c.g[A0][0] = Addr(r32(c, S5)).lw(c);
    } else {
        c.cycles += setup_cycles;
        if let Some((reg, val)) = setup {
            c.g[reg][0] = val;
        }
        if marker_reg == S6 {
            // The last category compares against 4 in s7 and marks with s6.
            c.g[S7][0] = 4;
        }
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
        loop {
            c.cycles += loop_cycles;
            c.g[type_reg][0] = 4;
            if c.g[V0][0] != c.g[type_reg][0] {
                c.g[S2][0] = Addr(r32(c, S2)).lw(c);
            } else {
                c.cycles += 18;
                c.g[S3][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr(r32(c, S4).wrapping_add(4)).lw(c);
                c.g[S0][0] = Addr(r32(c, S3).wrapping_add(8)).lw(c);
                c.g[A2][0] = Addr(r32(c, A0).wrapping_add(0x6c0)).lw(c);
                c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x24)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
                c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x28)).lh(c);
                c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x38)).lh(c);
                c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x2c)).lw(c);
                c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
                let target = r32(c, V0);
                c.g[RA][0] = sx(vtable_rets[0]);
                c.g[S0][0] = add32(c.g[S0][0], c.g[A1][0]);
                call_ptr(c, target, vtable_rets[0])?;
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x3c)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                let target = r32(c, V1);
                c.g[RA][0] = sx(vtable_rets[1]);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, target, vtable_rets[1])?;
                if c.g[V0][0] == 0 {
                    c.g[S2][0] = Addr(r32(c, S2)).lw(c);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = Addr(r32(c, S4)).lw(c);
                    c.g[RA][0] = sx(vtable_rets[2]);
                    c.g[A1][0] = c.g[S3][0];
                    callee::FIND.call(c, vtable_rets[2])?;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = Addr(r32(c, S4)).lw(c);
                        c.cycles += 2;
                        c.g[RA][0] = sx(vtable_rets[3]);
                        c.g[A1][0] = c.g[S3][0];
                        callee::ADD.call(c, vtable_rets[3])?;
                    } else {
                        c.cycles += 2;
                        c.sw(r32(c, V0).wrapping_add(0xc), c.g[marker_reg][0]);
                    }
                    c.cycles += 1;
                    c.g[S2][0] = Addr(r32(c, S2)).lw(c);
                }
            }
            c.cycles += 2;
            if c.g[S2][0] != 0 {
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, resume_pc);
                }
                continue;
            }
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S5)).lw(c);
            break;
        }
    }

    // The node's eight child octree entries.
    for (i, ret) in child_rets.iter().enumerate() {
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += if i == 7 { 4 } else { 3 };
            c.g[RA][0] = sx(*ret);
            c.g[A1][0] = c.g[S4][0];
            callee::COLLECT_NODE.call(c, *ret)?;
        }
        if i < 7 {
            c.g[A0][0] = Addr(r32(c, S5).wrapping_add(4 * (i as u32 + 1))).lw(c);
        }
    }
    apply_end(c, end);
    Ok(())
}


// ---------------------------------------------------------------------------
// CameraTriggerMan_UpdateForRider.

/// `void CameraTriggerMan_UpdateForRider(CameraTriggerMan*, Rider*)`
/// (0x16d320): clear the trigger flags for this rider's slot, classify the
/// rider's bounding box against each of the eight camera-trigger categories,
/// mark the triggers that contain it, collect the octree child lists, then
/// pop/apply the triggers the rider just left and push/apply the ones it
/// entered.
pub fn camera_trigger_man_update_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x100);
        frame.save(c, FP, 0x70);
        frame.save(c, S0, 0xf0);
        c.g[FP][0] = c.g[A1][0];
        frame.save(c, S1, 0xe0);
        frame.save(c, S2, 0xd0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S5, 0xa0);
        frame.save(c, S6, 0x90);
        frame.save(c, S7, 0x80);
        frame.save_ra(c, 0x60);
        c.sw(frame.at(c, 0x40), c.g[A0][0]);

        c.g[V0][0] = Addr(r32(c, FP).wrapping_add(0x874)).lw(c);
        let skip = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !skip {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, FP).wrapping_add(0x87c)).lw(c);
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }

        c.cycles += 2;
        let inactive = c.g[V1][0] == 0;
        c.g[S1][0] = sx(r32(c, FP).wrapping_add(0x6c0));
        if inactive {
            c.cycles += 12;
            frame.restore(c, S0, 0xf0);
            frame.restore(c, S1, 0xe0);
            frame.restore(c, S2, 0xd0);
            frame.restore(c, S3, 0xc0);
            frame.restore(c, S4, 0xb0);
            frame.restore(c, S5, 0xa0);
            frame.restore(c, S6, 0x90);
            frame.restore(c, S7, 0x80);
            frame.restore(c, FP, 0x70);
            frame.restore_ra(c, 0x60);
            frame.close(c);
            return Ok(());
        }

        // Clear this rider's flag words: this + 0x14 + count * 4.
        c.cycles += 33;
        c.g[V0][0] = Addr(r32(c, FP).wrapping_add(0x870)).lw(c);
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[S0][0] = sx((r32(c, V0)) << 2);
        c.sw(frame.at(c, 0x44), c.g[V0][0]);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x14));
        c.g[RA][0] = sx(0x0016_d38c);
        c.g[A0][0] = add32(c.g[V1][0], c.g[A0][0]);
        callee::CLEAR_FLAGS.call(c, 0x0016_d38c)?;

        // CameraTriggerMan from world->game->trigger manager.
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
        c.g[S0][0] = add32(c.g[S0][0], c.g[A0][0]);
        c.g[A1][0] = Addr(r32(c, FP).wrapping_add(0x6c0)).lw(c);
        c.g[S6][0] = Addr(r32(c, V0).wrapping_add(0x20)).lw(c);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x14));
        c.sw(frame.at(c, 0), c.g[S0][0]);
        c.sw(frame.at(c, 4), c.g[FP][0]);

        // Two virtual calls on the rider's pose object (the two quadwords
        // they return are the rider's bounding box, copied to the frame).
        c.g[A0][0] = Addr(r32(c, A1).wrapping_add(0x28)).lh(c);
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(0x2c)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0016_d3c0);
        c.g[A0][0] = add32(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0016_d3c0)?;
        c.g[V1][0] = Addr(r32(c, FP).wrapping_add(0x6c0)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0016_d3d8);
        c.g[A0][0] = add32(c.g[S1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0016_d3d8)?;
        c.lq(3, r32(c, S0));
        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(4, r32(c, V0));
        c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S5][0] = Addr(r32(c, S6).wrapping_add(0x10)).lw(c);
        let no_head = c.g[S5][0] == 0;
        c.g[S4][0] = c.g[SP][0];

        // Category 0: the manager's first list (no flag; the head is the
        // test), its box collected with the manager as the second key.
        if !no_head {
            category_pass(
                c,
                true,
                Src_2::Reg(S6),
                Some(0x30),
                None,
                7,
                0x0016_d404,
                Src_2::Reg(S0),
                Src_2::Reg(S6),
                &[
                    0x0016_d4c0,
                    0x0016_d4d4,
                    0x0016_d4e8,
                    0x0016_d4fc,
                    0x0016_d510,
                    0x0016_d524,
                    0x0016_d538,
                    0x0016_d54c,
                ],
                S7,
                V1,
                3,
                0x0016_d428,
                End::Count,
                0x0016_d578,
                [0x0016_d464, 0x0016_d474, 0x0016_d488, 0x0016_d4a0],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c);
        }

        // Cache the remaining category descriptors and flag fields.
        c.cycles += 16;
        c.g[V1][0] = sx(r32(c, S6).wrapping_add(0x3c));
        c.g[V0][0] = Addr(r32(c, S6).wrapping_add(0x24)).lw(c);
        c.g[S0][0] = sx(r32(c, S6).wrapping_add(0x14));
        c.g[A0][0] = sx(r32(c, A0) << 2);
        c.sw(frame.at(c, 0x4c), c.g[V1][0]);
        c.sw(frame.at(c, 0x48), c.g[A0][0]);
        c.g[V1][0] = sx(r32(c, S6).wrapping_add(0x64));
        c.g[A0][0] = sx(r32(c, S6).wrapping_add(0x50));
        c.sw(frame.at(c, 0x54), c.g[V1][0]);
        c.sw(frame.at(c, 0x50), c.g[A0][0]);
        c.g[V1][0] = sx(r32(c, S6).wrapping_add(0x8c));
        c.g[A0][0] = sx(r32(c, S6).wrapping_add(0x78));
        c.g[S7][0] = sx(r32(c, S6).wrapping_add(0x28));
        c.sw(frame.at(c, 0x58), c.g[A0][0]);
        let skip = c.g[V0][0] == 0;
        c.sw(frame.at(c, 0x5c), c.g[V1][0]);
        if !skip {
            category_pass(
                c,
                false,
                Src_2::Reg(S0),
                None,
                Some(0x31),
                8,
                0x0016_d5d0,
                Src_2::Reg(S1),
                Src_2::Reg(S0),
                &[
                    0x0016_d68c,
                    0x0016_d6a0,
                    0x0016_d6b4,
                    0x0016_d6c8,
                    0x0016_d6dc,
                    0x0016_d6f0,
                    0x0016_d704,
                    0x0016_d718,
                ],
                V1,
                A0,
                3,
                0x0016_d5f0,
                End::Field(0x38),
                0x0016_d744,
                [0x0016_d62c, 0x0016_d63c, 0x0016_d650, 0x0016_d66c],
                None,
                2,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x38));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x32));
        if !skip {
            category_pass(
                c,
                false,
                Src_2::Reg(S7),
                None,
                None,
                7,
                0x0016_d760,
                Src_2::Reg(S0),
                Src_2::Reg(S7),
                &[
                    0x0016_d81c,
                    0x0016_d830,
                    0x0016_d844,
                    0x0016_d858,
                    0x0016_d86c,
                    0x0016_d880,
                    0x0016_d894,
                    0x0016_d8a4,
                ],
                S7,
                A0,
                3,
                0x0016_d780,
                End::Field(0x4c),
                0x0016_d8d0,
                [0x0016_d7bc, 0x0016_d7cc, 0x0016_d7e0, 0x0016_d7f8],
                Some((S7, 1)),
                2,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x4c));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x4c)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src_2::Reg(V0),
                Some(0x33),
                None,
                8,
                0x0016_d8f0,
                Src_2::Reg(S0),
                Src_2::Stack(0x4c),
                &[
                    0x0016_d9a8,
                    0x0016_d9bc,
                    0x0016_d9d0,
                    0x0016_d9e4,
                    0x0016_d9f8,
                    0x0016_da0c,
                    0x0016_da20,
                    0x0016_da34,
                ],
                S7,
                V1,
                3,
                0x0016_d910,
                End::Field(0x60),
                0x0016_da60,
                [0x0016_d94c, 0x0016_d95c, 0x0016_d970, 0x0016_d988],
                Some((S7, 1)),
                2,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x60));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x50)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src_2::Reg(A0),
                Some(0x34),
                None,
                7,
                0x0016_da7c,
                Src_2::Reg(S0),
                Src_2::Stack(0x50),
                &[
                    0x0016_db38,
                    0x0016_db4c,
                    0x0016_db60,
                    0x0016_db74,
                    0x0016_db88,
                    0x0016_db9c,
                    0x0016_dbb0,
                    0x0016_dbc4,
                ],
                S7,
                V1,
                3,
                0x0016_daa0,
                End::Field(0x74),
                0x0016_dbf0,
                [0x0016_dadc, 0x0016_daec, 0x0016_db00, 0x0016_db18],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x74));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x54)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src_2::Reg(A0),
                Some(0x35),
                None,
                7,
                0x0016_dc0c,
                Src_2::Reg(S0),
                Src_2::Stack(0x54),
                &[
                    0x0016_dcc8,
                    0x0016_dcdc,
                    0x0016_dcf0,
                    0x0016_dd04,
                    0x0016_dd18,
                    0x0016_dd2c,
                    0x0016_dd40,
                    0x0016_dd54,
                ],
                S7,
                V1,
                3,
                0x0016_dc30,
                End::Field(0x88),
                0x0016_dd80,
                [0x0016_dc6c, 0x0016_dc7c, 0x0016_dc90, 0x0016_dca8],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x88));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x58)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src_2::Reg(A0),
                Some(0x36),
                None,
                7,
                0x0016_dd9c,
                Src_2::Reg(S0),
                Src_2::Stack(0x58),
                &[
                    0x0016_de58,
                    0x0016_de6c,
                    0x0016_de80,
                    0x0016_de94,
                    0x0016_dea8,
                    0x0016_debc,
                    0x0016_ded0,
                    0x0016_dee4,
                ],
                S7,
                V1,
                3,
                0x0016_ddc0,
                End::Field(0x9c),
                0x0016_df10,
                [0x0016_ddfc, 0x0016_de0c, 0x0016_de20, 0x0016_de38],
                Some((S7, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Field(0x9c));
        }

        c.cycles += 2;
        let skip = c.g[V0][0] == 0;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x5c)).lw(c);
        if !skip {
            category_pass(
                c,
                false,
                Src_2::Reg(A0),
                Some(0x37),
                None,
                7,
                0x0016_df2c,
                Src_2::Reg(S0),
                Src_2::Stack(0x5c),
                &[
                    0x0016_dfe8,
                    0x0016_dff8,
                    0x0016_e00c,
                    0x0016_e020,
                    0x0016_e034,
                    0x0016_e048,
                    0x0016_e05c,
                    0x0016_e070,
                ],
                S6,
                S7,
                2,
                0x0016_df50,
                End::Stack48,
                0x0016_e09c,
                [0x0016_df88, 0x0016_df98, 0x0016_dfac, 0x0016_dfc4],
                Some((S6, 1)),
                3,
            )?;
        } else {
            c.cycles += 1;
            apply_end(c, End::Stack48);
        }

        // Triggers that just became inactive: pop from the stack box, apply
        // the action, or apply the matching trigger's action, then unlink.
        c.cycles += 5;
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[V0][0] = add32(c.g[V1][0], c.g[A0][0]);
        c.g[S0][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let empty = c.g[S0][0] == 0;
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c);
        if !empty {
            c.cycles += 5;
            c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x48)).lw(c);
            c.g[V0][0] = sx(r32(c, V1) << 5);
            c.g[S2][0] = sx(r32(c, A0).wrapping_add(0x14));
            c.g[S3][0] = sx(r32(c, V0).wrapping_add(0x1c));
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
            loop {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
                } else {
                    c.cycles += 9;
                    c.g[S1][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3c));
                    c.g[A1][0] = Addr(r32(c, S1)).lw(c);
                    c.g[RA][0] = sx(0x0016_e0ec);
                    c.g[A0][0] = add32(c.g[V0][0], c.g[S3][0]);
                    callee::POP_TRIGGER.call(c, 0x0016_e0ec)?;
                    let popped = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x38)).lw(c);
                    if !popped {
                        c.cycles += 5;
                        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
                        c.g[RA][0] = sx(0x0016_e100);
                        c.g[A1][0] = c.g[FP][0];
                        callee::APPLY_ACTION.call(c, 0x0016_e100)?;
                        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                    } else {
                        c.cycles += 2;
                        let no_entry = c.g[V0][0] == 0;
                        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                        if !no_entry {
                            c.cycles += 5;
                            c.g[A1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x3c)).lw(c);
                            c.g[RA][0] = sx(0x0016_e11c);
                            c.g[A0][0] = add32(c.g[V1][0], c.g[S2][0]);
                            callee::FIND_BY_ID.call(c, 0x0016_e11c)?;
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                                c.g[RA][0] = sx(0x0016_e130);
                                c.g[A1][0] = c.g[FP][0];
                                callee::APPLY_ACTION.call(c, 0x0016_e130)?;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0016_e140);
                    c.g[A0][0] = add32(c.g[V0][0], c.g[S2][0]);
                    callee::REMOVE.call(c, 0x0016_e140)?;
                    c.g[S0][0] = c.g[V0][0];
                }
                c.cycles += 2;
                if c.g[S0][0] != 0 {
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_e0c8);
                    }
                    continue;
                }
                break;
            }
        }

        // Triggers that just became active: push the stack box and apply the
        // trigger's action.
        c.cycles += 6;
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x48)).lw(c);
        c.g[A0][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[V0][0] = add32(c.g[V1][0], c.g[A0][0]);
        c.g[S1][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let empty = c.g[S1][0] == 0;
        c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x44)).lw(c);
        if !empty {
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V1) << 5);
            c.g[S0][0] = sx(r32(c, V0).wrapping_add(0x1c));
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
            loop {
                c.cycles += 2;
                let no_entry = c.g[V0][0] == 0;
                c.g[V1][0] = Addr((c.g[SP][0] as u32).wrapping_add(0x40)).lw(c);
                if !no_entry {
                    c.cycles += 8;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[RA][0] = sx(0x0016_e188);
                    c.g[A0][0] = add32(c.g[V1][0], c.g[S0][0]);
                    callee::PUSH_TRIGGER.call(c, 0x0016_e188)?;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    c.g[A1][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x0016_e198);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                    callee::APPLY_ACTION.call(c, 0x0016_e198)?;
                }
                c.cycles += 3;
                c.g[S1][0] = Addr(r32(c, S1).wrapping_add(4)).lw(c);
                if c.g[S1][0] != 0 {
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_e170);
                    }
                    continue;
                }
                break;
            }
        }

        c.cycles += 12;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::put_vec;
    use ssx3_ee as ee;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::compare;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::ONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::IDENTITY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::AXIS_X;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::AXIS_Y;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::AXIS_Z;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::axes;
    pub(crate) use crate::game::camera::script::scriptcontroller::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::camera::script::scriptcontroller::tests::OUT;
    pub(crate) use crate::game::camera::script::scriptcontroller::tests::WOBBLE_SCALE;
    pub(crate) use crate::game::camera::script::scriptcontroller::tests::NO_WOBBLE;
    pub(crate) use crate::game::camera::script::scriptcontroller::tests::f;
    pub(crate) use crate::game::camera::script::scriptcontroller::tests::put_globals;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2 as rng;
    pub(crate) use crate::game::camera::camera::tests::camera_chain;


    #[test]
    fn camera_trigger_man_push_trigger_stack_matches() {
        check(ssx3_ee::s02::f_0016d238__generated, camera_trigger_man_push_trigger_stack, 200, &|c, seed| {
            let mut r = Rng(0xa1a1_a1a1 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let n0 = r.next_u32() % 8;
            // Odd seeds set high bits that vanish in the 32-bit `<< 2`.
            let count = if seed % 2 == 0 { n0 } else { 0x4000_0000 | n0 };
            c.bus.write32(DATA, count);
        });
    }


    #[test]
    fn camera_bound_obj_get_world_point_matches() {
        check(
            ssx3_ee::s02::f_00170d20__generated,
            camera_bound_obj_get_world_point,
            200,
            &|c, seed| {
                let mut r = Rng(0x170d_2001 ^ seed as u64);
                scramble(c, &mut r);
                axes(c);
                let out = DATA + 0x200;
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(DATA);
                c.g[A2][0] = sx((seed % 8) as u32);
                // Position, three angles, scale, then the eight corners.
                for i in 0..8 {
                    c.bus.write32(DATA + 4 * i, r.edge_f32());
                }
                c.bus.write32(DATA + 0x18, r.f32(2.0));
                c.bus.write32(DATA + 0x1c, r.f32(2.0));
                c.bus.write32(DATA + 0x20, r.f32(2.0));
                for i in 0..8 * 3 {
                    c.bus.write32(DATA + 0x28 + 4 * i, r.edge_f32());
                }
                c.pc = 0x0017_0d20;
            },
        );
    }

    pub(crate) const BOUND: u32 = DATA + 0x1000;


    /// A line bound: translation, three angles, scale, start and end points.
    pub(crate) fn put_line_bound(c: &mut Cpu, r: &mut Rng, seed: u32) {
        for k in 0..0x12u32 {
            c.bus.write32(BOUND + 4 * k, r.f32(3.0));
        }
        for k in 0..3u32 {
            let s = if seed % 5 == 0 && k == 1 { 0 } else { r.f32(2.0) };
            c.bus.write32(BOUND + line_bound::SCALE + 4 * k, s);
        }
    }


    #[test]
    fn camera_line_bound_world_start_matches() {
        check(ee::s02::f_00170240__generated, camera_line_bound_world_start, SEEDS, &|c, seed| {
            let mut r = Rng(0x0017_0240 ^ (seed as u64) << 7);
            scramble(c, &mut r);
            put_globals(c, &mut r);
            put_line_bound(c, &mut r, seed);
            c.g[A0][0] = sx(OUT);
            c.g[A1][0] = sx(BOUND);
        });
    }


    #[test]
    fn camera_line_bound_world_end_matches() {
        check(ee::s02::f_001707b0__generated, camera_line_bound_world_end, SEEDS, &|c, seed| {
            let mut r = Rng(0x0017_07b0 ^ (seed as u64) << 7);
            scramble(c, &mut r);
            put_globals(c, &mut r);
            put_line_bound(c, &mut r, seed);
            c.g[A0][0] = sx(OUT);
            c.g[A1][0] = sx(BOUND);
        });
    }


    #[test]
    fn camera_trigger_man_empty_global_call_matches() {
        crate::difftest::check(ee::s02::f_0016cf18__generated, camera_trigger_man_empty_global_call, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x16cf_18, 1);
            c.g[A0][0] = if seed % 3 == 0 { 0 } else { sx(DATA + (r.next_u32() & 0xfff0)) };
        });
    }


    #[test]
    fn camera_trigger_stack_ctor_matches() {
        check(ssx3_ee::s02::f_0016d210__generated, camera_trigger_stack_ctor, 200, &|c, seed| {
            let mut r = rng(seed, 0x14);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x2000);
            for i in 0..4u32 {
                c.bus.write32(DATA + 0x2000 + 4 * i, r.next_u32());
            }
        });
    }


    #[test]
    fn camera_trigger_man_unload_matches() {
        check(
            ssx3_ee::s02::f_0016c9b0__generated,
            camera_trigger_man_unload,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x16c9b0, 6);
                // The camera chain is set up so the chase reset really runs;
                // an empty trigger list makes the purge loops skip.
                camera_chain(c, seed);
                let m = DATA;
                c.bus.write32(m + 4, 0);
                c.g[A0][0] = m as u64;
            },
        );
    }


        #[test]
    fn camera_trigger_man_pop_trigger_stack_matches() {
        check(ee::s02::f_0016d260__generated, camera_trigger_man_pop_trigger_stack, 150, &|c, seed| {
            let mut r = Rng(0x0016_d260 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let stack = DATA;
            let count = (seed % 4) as u32;
            c.bus.write32(stack, count);
            c.bus.write32(stack + 4, r.next_u32());
            for i in 0..4 {
                c.bus.write32(stack + 8 + 4 * i, (r.next_u32() % 4) as u32);
            }
            c.g[A0][0] = sx(stack);
            c.g[A1][0] = (r.next_u32() % 5) as u64;
            c.g[A2][0] = sx(DATA + 0x100);
            c.g[A3][0] = sx(DATA + 0x200);
        });
    }


    pub(crate) const SEEDS_2: u32 = 200;


    /// Hard, caller-saved (`soft`) and LO/HI must all match.
    pub(crate) fn strict(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }


    pub(crate) fn w(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }


    pub(crate) fn w16(c: &mut Cpu, at: u32, v: u16) {
        c.bus.write16(at, v);
    }


    /// Rider at DATA+0x14000, its pose vtable object at +0x15c0, camera
    /// trigger manager at DATA+0x12000 (two spatial-key categories), the
    /// rider's trigger list at DATA+0x13014, and the two virtual pose calls
    /// forced to return the rider box at DATA+0x16000 / +0x17000.
    ///
    /// The box is min=max=(0, 2048, 0), so a category whose cell is
    /// [-2048, 2048]^3 classifies as 0 (inside; exercises the list walk) and
    /// a far-away cell as 1 (outside; exercises the skip/tail paths). The
    /// `v0 == 2` (box collection) path is not exercised: it would run the
    /// real octree box walker, whose fixture is much larger than this test's
    /// budget.
    pub(crate) fn setup(c: &mut Cpu, seed: u32) {
        let _ = machine(c, seed, 0x0016_d320, 0);
        let world = DATA + 0x1_0000;
        let game = world + 0x1000;
        let mgr = DATA + 0x12_000;
        let this = DATA + 0x13_000;
        let rider = DATA + 0x14_000;
        let vt = DATA + 0x15_000;
        let amin = DATA + 0x16_000;
        let amax = DATA + 0x17_000;

        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x11_000); // BELibrary dummy
        w(c, GAME_GP.wrapping_sub(0x848), world);
        w(c, world + 0x84, game);
        w(c, game + 0x20, mgr);

        // Rider slot count and the trigger list inside `this`.
        let count = seed % 3 % 2;
        w(c, this + 0x870, count);
        w(c, this + 0x14, 0);
        w(c, this + 0x18, 0);

        // Entry gate: 0x874 != 0 and 0x87c > 0 to do any work.
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(rider);
        w(c, rider + 0x874, if seed % 8 == 0 { 0 } else { 1 });
        w(
            c,
            rider + 0x87c,
            match seed % 8 {
                1 => 0,
                2 => 0xffff_ffff,
                _ => 1,
            },
        );

        // The rider's pose object; both virtual calls go to the fake hook.
        w(c, rider + 0x6c0, vt);
        w16(c, vt + 0x28, 0);
        w(c, vt + 0x2c, 0x5000);
        w(c, LOG_RESULTS, amin);
        w(c, LOG_RESULTS + 4, amax);
        for a in [amin, amax] {
            w(c, a, 0);
            w(c, a + 4, 0x4500_0000); // 2048.0
            w(c, a + 8, 0);
            w(c, a + 12, 0);
        }

        // Category 0 is the manager itself; its key is either inside or far
        // outside the rider box depending on the seed.
        let outside = !matches!(seed % 8, 3 | 4 | 5 | 6);
        let idx = if outside { -100i32 } else { -1i32 };
        w(c, mgr, 11);
        for off in [4u32, 8, 12] {
            w(c, mgr + off, idx as u32);
        }

        // The other seven categories (spatial key + list head at +0x10).
        for off in [0x14u32, 0x28, 0x3c, 0x50, 0x64, 0x78, 0x8c] {
            w(c, mgr + off, 11);
            for k in [4u32, 8, 12] {
                w(c, mgr + off + k, (-100i32) as u32);
            }
            w(c, mgr + off + 0x10, 0);
        }

        // A small valid list node so category 0's head and the child slots
        // are real (all child pointers null, so no octree walk).
        let node = DATA + 0x18_000;
        let walk = DATA + 0x19_000;
        w(c, node + 0x28, 0);
        for off in [0u32, 4, 8, 12, 0x10, 0x14, 0x18, 0x1c] {
            w(c, node + off, 0);
        }

        match seed % 8 {
            3 => {
                w(c, mgr + 0x10, node);
            }
            4 => {
                // One non-matching list node: type != 4, end of list.
                w(c, walk, 0);
                w(c, walk + 8, 0);
                w(c, node + 0x28, walk);
                w(c, mgr + 0x10, node);
            }
            5 => {
                // One matching list node (type 4): two virtual calls; the
                // second is pointed at ActiveCameraTriggerList_Find with an
                // empty list, so it returns 0 and no list is modified.
                let trigger = DATA + 0x1a_000;
                let obj1 = DATA + 0x1b_000;
                let obj2 = DATA + 0x1c_000;
                w(c, walk, 0);
                w(c, walk + 8, 4);
                w(c, walk + 0xc, trigger);
                w(c, node + 0x28, walk);
                w(c, mgr + 0x10, node);
                w(c, trigger + 8, obj2);
                w(c, vt + 0x6c0, obj1);
                w16(c, obj1 + 0x28, 0);
                w(c, obj1 + 0x2c, 0x5000);
                w(c, obj2 + 0x24, obj2);
                w(c, obj2, 0);
                w16(c, obj2 + 0x38, 0);
                w(c, obj2 + 0x3c, 0x0016_c6b0);
            }
            6 => {}
            7 => {
                // Every category has a list head: all eight category tests
                // run (but their keys are outside the box, so none walks a
                // list or collects a box).
                let dummy = DATA + 0x1d_000;
                for off in [0x10u32, 0x24, 0x38, 0x4c, 0x60, 0x74, 0x88, 0x9c] {
                    w(c, mgr + off, dummy);
                }
            }
            _ => {}
        }
    }


    #[test]
    fn camera_trigger_man_update_for_rider_matches() {
        strict(
            ee::s02::f_0016d320__generated,
            camera_trigger_man_update_for_rider,
            SEEDS_2,
            &setup,
        );
    }
}
