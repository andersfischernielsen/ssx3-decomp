//! `src/object/instanceman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
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
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::intersect::worldsphtree::save_field_block;
pub(crate) use crate::game::render::lightman::gp;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::scripter::ssxscriptengine::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::float;
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
        0x0035_12c0,
        instances_man_resolve_handle,
        "InstancesMan_ResolveHandle",
    ),
    (
        0x0035_1398,
        intersect_info_set_source,
        "IntersectInfo_SetSource",
    ),
    (
        0x0035_1508,
        intersect_info_get_duration,
        "IntersectInfo_GetDuration",
    ),
    (
        0x0035_1660,
        intersect_info_eval_derivative,
        "IntersectInfo_EvalDerivative",
    ),
    (
        0x0035_1800,
        intersect_info_evaluate_matrix,
        "IntersectInfo_EvaluateMatrix",
    ),
    (
        0x0035_1948,
        intersect_info_mul_matrix,
        "IntersectInfo_MulMatrix",
    ),
    (
        0x0035_2708,
        mesh_anim_kill_restorable,
        "MeshAnim_KillRestorable",
    ),
    (0x0035_2a58, mesh_anim_save, "MeshAnim_Save"),
    (
        0x0035_1538,
        intersect_info_evaluate,
        "IntersectInfo_Evaluate",
    ),
    (0x0035_2810, mesh_anim_render, "MeshAnim_Render"),
    (0x0035_2500, mesh_anim_update, "MeshAnim_Update"),
];

/// Direct and cross-shard callees, by guest address.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_FROM_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_TO_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::EA_MEMSET;
    pub(crate) use crate::game::render::lightman::callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS;
    pub(crate) use crate::game::render::lightman::callee::INTERSECT_INFO_EVALUATE;
    pub(crate) use crate::game::render::lightman::callee::LIGHT_CONTRIBUTION;
    pub(crate) use crate::game::render::lightman::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::render::lightman::callee::MATH_SIN_COS;
    pub(crate) use crate::game::render::lightman::callee::MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH;
    pub(crate) use crate::game::render::lightman::callee::SPATIAL_HASH_UPDATE;
    pub(crate) use crate::game::render::lightman::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::TYPED_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_AI_FRAME_COUNT;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_FLAG_MANAGER;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_NASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY as CBX_STRING_CTOR_COPY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR as CBX_STRING_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE as WORLD_OBJ_REPLACE_WITH_DEAD_NODE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
}

/// `void* InstancesMan_ResolveHandle(InstancesMan *man, uint handle)`
/// (0x3512c0): resolve a packed index/handle to an instance.
pub fn instances_man_resolve_handle(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = c.g[A1][0] & 0xff;
    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
    let matches = c.g[V0][0] == c.g[V1][0];
    c.g[V1][0] = sx((c.g[A1][0] as u32) >> 8);
    if !matches {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 21;
    c.g[V0][0] = 0xb0;
    c.g[V1][0] = sx(abi::mult(c, c.g[V1][0] as u32, 0xb0));
    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lbu(c);
    c.g[A3][0] = 0xffff_ffff_ba2e_0000;
    c.g[A3][0] |= 0x8ba3;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x78).sb(c, c.g[V0][0]);
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
    c.g[A2][0] = Addr(c.g[A0][0] as u32).off(0x78).lbu(c);
    c.g[A1][0] |= 0x2000;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x8).sw(c, c.g[A1][0]);
    c.g[V1][0] = sx(abi::mult(c, c.g[V1][0] as u32, c.g[A3][0] as u32));
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
    c.g[A2][0] |= c.g[V1][0];
    Addr(c.g[A0][0] as u32).off(0x78).sw(c, c.g[A2][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Intersect info.

/// `void IntersectInfo_SetSource(IntersectInfo *info, const void *src)`
/// (0x351398): copy six floats from the source vector into `info`.
pub fn intersect_info_set_source(c: &mut Cpu) {
    c.cycles += 14;
    let info = Addr(abi::arg(c, 0));
    let src = Addr(abi::arg(c, 1));
    info.off(0x40).sw(c, c.g[A1][0]);
    c.f[0] = src.read_fpu(c).0;
    info.off(0x44).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = src.off(4).read_fpu(c).0;
    info.off(0x48).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = src.off(8).read_fpu(c).0;
    info.off(0x4c).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = src.off(0xc).read_fpu(c).0;
    info.off(0x50).write_fpu(c, FpuF32(c.f[1]));
    c.f[0] = src.off(0x10).read_fpu(c).0;
    info.off(0x54).write_fpu(c, FpuF32(c.f[0]));
    c.f[1] = src.off(0x14).read_fpu(c).0;
    info.off(0x58).write_fpu(c, FpuF32(c.f[1]));
    abi::ret(c);
}

/// `float IntersectInfo_GetDuration(IntersectInfo *info)` (0x351508): the
/// duration from the current action, or 0.0.
pub fn intersect_info_get_duration(c: &mut Cpu) {
    c.cycles += 5;
    let info = Addr(abi::arg(c, 0));
    let action = info.off(0x40).lw(c);
    c.f[0] = 0;
    let v0 = Addr(action as u32).off(0x20).lw(c);
    c.g[V0][0] = v0;
    let v1 = Addr(v0 as u32).lw(c);
    c.g[V1][0] = v1;
    if v0 != 0 && v1 != 0 {
        c.cycles += 5;
        let ptr = Addr(v1 as u32).off(4).lw(c);
        c.g[V0][0] = ptr;
        if ptr != 0 {
            c.f[0] = Addr(ptr as u32).off(0x10).read_fpu(c).0;
        }
    } else if v0 != 0 {
        c.cycles += 2;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void IntersectInfo_EvaluateMatrix(IntersectInfo *self)` (0x00351800):
/// run the evaluation pass, then build the rotation matrix from
/// `-rot * pi/180` (three `Math_SinCos` calls) and the translation.
pub fn intersect_info_evaluate_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 81;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x30);
        c.g[S0][0] = c.g[A0][0];
        callee::INTERSECT_INFO_EVALUATE.call(c, 0x0035_1814)?;
        // r[k] = -self->rot[k] * deg2rad, at sp+0..8.
        let selfp = c.g[S0][0] as u32;
        c.lwc1(2, off(selfp, 0x50));
        c.g[A0][0] = sx(off(c.g[SP][0] as u32, 0x10));
        c.lwc1(3, off(selfp, 0x54));
        c.g[A1][0] = sx(off(c.g[SP][0] as u32, 0x14));
        c.lwc1(1, off(selfp, 0x58));
        c.neg_s(2, 2);
        c.neg_s(3, 3);
        c.lwc1(0, gp(c, -0x2a90));
        c.neg_s(1, 1);
        c.mul_s(2, 2, 0);
        c.mul_s(3, 3, 0);
        c.mul_s(1, 1, 0);
        c.mov_s(12, 2);
        c.swc1(2, c.g[SP][0] as u32);
        c.swc1(3, off(c.g[SP][0] as u32, 4));
        c.swc1(1, off(c.g[SP][0] as u32, 8));
        let sp = c.g[SP][0] as u32;
        for k in 0..3u32 {
            if k > 0 {
                c.lwc1(12, off(sp, 4 * k));
                c.g[A0][0] = sx(off(sp, 0x10 + 8 * k));
                c.g[A1][0] = sx(off(sp, 0x14 + 8 * k));
            }
            let ret = 0x0035_1858 + 0x10 * k;
            callee::MATH_SIN_COS.call(c, ret)?;
        }
        // The matrix, from sc[0] = (sx, cx), sc[1] = (sy, cy), sc[2] = (sz, cz).
        let s = c.g[SP][0] as u32;
        let selfp = c.g[S0][0] as u32;
        c.lwc1(0, off(s, 0x10)); // sx
        c.lwc1(4, off(s, 0x18)); // sy
        c.lwc1(7, off(s, 0x14)); // cx
        c.neg_s(12, 0);
        c.lwc1(10, off(s, 0x20)); // sz
        c.mul_s(8, 0, 4); // sxsy
        c.lwc1(9, off(s, 0x24)); // cz
        c.mul_s(2, 7, 4); // cxsy
        c.mul_s(17, 7, 10); // cxsz
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.lwc1(3, off(s, 0x1c)); // cy
        c.mul_s(15, 0, 9); // sx*cz
        c.mul_s(16, 8, 10); // sxsy*sz
        c.swc1(1, off(selfp, 0xcc));
        c.mul_s(1, 7, 9); // cx*cz
        c.lwc1(11, off(selfp, 0x44));
        c.mul_s(5, 17, 4); // cxsz*sy
        c.lwc1(13, off(selfp, 0x48));
        c.mul_s(0, 0, 10); // sx*sz
        c.lwc1(14, off(selfp, 0x4c));
        c.mul_s(2, 2, 9); // cxsy*cz
        c.swc1(11, off(selfp, 0xc0));
        c.neg_s(6, 3);
        c.swc1(13, off(selfp, 0xc4));
        c.mul_s(8, 8, 9); // sxsy*cz
        c.swc1(14, off(selfp, 0xc8));
        c.mul_s(7, 7, 3); // cx*cy
        c.swc1(4, off(selfp, 0x98));
        c.mul_s(12, 12, 3); // -sx*cy
        c.stw(off(selfp, 0x9c), 0);
        c.sub_s(0, 0, 2);
        c.stw(off(selfp, 0xac), 0);
        c.add_s(5, 5, 15);
        c.swc1(7, off(selfp, 0xb8));
        c.sub_s(1, 1, 16);
        c.swc1(12, off(selfp, 0xa8));
        c.mul_s(6, 6, 10); // -cy*sz
        c.swc1(0, off(selfp, 0xb0));
        c.add_s(8, 8, 17);
        c.swc1(5, off(selfp, 0xb4));
        c.mul_s(3, 3, 9); // cy*cz
        c.swc1(1, off(selfp, 0xa4));
        c.swc1(6, off(selfp, 0x94));
        c.swc1(8, off(selfp, 0xa0));
        c.swc1(3, off(selfp, 0x90));
        c.stw(off(selfp, 0xbc), 0);
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void MeshAnim_Save(MeshAnim* self, Stream* stream)` (0x352a58): 0x0c
/// bytes at `+0x1c`.
pub fn mesh_anim_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(
            c,
            callee::WORLD_OBJ_SAVE,
            0x1c,
            0x0c,
            0x0035_2a74,
            0x0035_2a90,
        )
    })
}

/// `func_00352708` (0x352708): `MeshAnim_KillRestorable`. If asked to run the
/// kill script (`a1 != 0`) and `WorldObj_RunKillScript` accepts, return;
/// otherwise if the node at `+0x6b0` is still the restorable one replace it
/// with a dead node (`WorldObj_ReplaceWithDeadNode`) and, when it became a
/// dead node (kind 2), destroy it (`vtable[8]`, flag 3).
/// Signature: `void kill_restorable(this, run_script)`.
pub fn mesh_anim_kill_restorable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_script = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !no_script {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0035_2724);
            callee::WORLD_OBJ_RUN_KILL_SCRIPT.call(c, 0x0035_2724)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 3;
        c.g[V0][0] = lw_r(c, S0, 0x6b0);
        if c.g[V0][0] != 0 {
            c.g[V1][0] = lw_r(c, S0, 0x6b0);
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0035_2740);
            c.g[A0][0] = c.g[S0][0];
            callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE.call(c, 0x0035_2740)?;
            c.g[V1][0] = lw_r(c, S0, 0x6b0);
        }
        c.cycles += 3;
        c.g[V0][0] = 0x2;
        if c.g[V1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let null = c.g[S0][0] == 0;
        c.g[A1][0] = 0x3;
        if !null {
            c.cycles += 5;
            c.g[V0][0] = lw_r(c, S0, 0xc);
            c.g[A0][0] = lh_r(c, V0, 0x8);
            c.g[V1][0] = lw_r(c, V0, 0xc);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0035_276c);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0035_276c)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Instance manager.

/// `void MeshAnim_Kill(void* self)` (0x352780): mark the instance's animation
/// dead (or replace the world object when it has none).
pub fn mesh_anim_kill(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, A2), 0x6b0)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 16;
            c.g[A1][0] = Addr(p(r32(c, A2), 0x18)).lw(c);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[V1][0] = c.g[V1][0] | 0x300;
            c.g[V0][0] = Addr(p(r32(c, A1), 8)).lw(c);
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            Addr(p(r32(c, A1), 8)).sw(c, c.g[V0][0]);
            c.g[A0][0] = Addr(p(r32(c, A2), 0x18)).lw(c);
            c.g[V0][0] = Addr(p(r32(c, A0), 8)).lw(c);
            c.g[V1][0] = sx(((r32(c, V0) as i32) >> 16) as u32);
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            Addr(p(r32(c, A0), 8)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(p(r32(c, A2), 0x18)).lw(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 8)).lw(c);
            c.g[V0][0] = c.g[V0][0] | 2;
            let no_self = c.g[A2][0] == 0;
            Addr(p(r32(c, V1), 8)).sw(c, c.g[V0][0]);
            if !no_self {
                c.cycles += 8;
                c.g[V0][0] = Addr(p(r32(c, A2), 0xc)).lw(c);
                c.g[A1][0] = 3;
                c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, target, 0x0035_27f0)?;
            }
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[A2][0];
            callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE_2.call(c, 0x0035_2800)?;
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `IntersectInfo_MulMatrix(self, m)` (0x351948): `dst = m * dst` through the
/// VU0 matrix-multiply routine, then copy the result back.
pub fn intersect_info_mul_matrix(c: &mut Cpu) {
    c.cycles += 49;
    let m = abi::arg(c, 1);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x80));
    let dst = abi::arg(c, 0).wrapping_add(0x90);
    c.g[A0][0] = sx(dst);
    c.g[V0][0] = sx(sp(c).wrapping_add(0x40));

    let tmp = sp(c).wrapping_add(0x40);
    c.lqc2(4, m);
    c.lqc2(5, m.wrapping_add(0x10));
    c.lqc2(6, m.wrapping_add(0x20));
    c.lqc2(7, m.wrapping_add(0x30));
    c.lqc2(8, dst);
    c.lqc2(9, dst.wrapping_add(0x10));
    c.lqc2(10, dst.wrapping_add(0x20));
    c.lqc2(11, dst.wrapping_add(0x30));
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
    c.sqc2(12, tmp);
    c.sqc2(13, tmp.wrapping_add(0x10));
    c.sqc2(14, tmp.wrapping_add(0x20));
    c.sqc2(15, tmp.wrapping_add(0x30));
    // Copy the result through a second temporary and store it into `dst`.
    c.lqc2(1, tmp);
    c.lqc2(2, tmp.wrapping_add(0x10));
    c.lqc2(3, tmp.wrapping_add(0x20));
    c.lqc2(4, tmp.wrapping_add(0x30));
    c.sqc2(1, sp(c));
    c.sqc2(2, sp(c).wrapping_add(0x10));
    c.sqc2(3, sp(c).wrapping_add(0x20));
    c.sqc2(4, sp(c).wrapping_add(0x30));
    c.lqc2(1, sp(c));
    c.lqc2(2, sp(c).wrapping_add(0x10));
    c.lqc2(3, sp(c).wrapping_add(0x20));
    c.lqc2(4, sp(c).wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));

    let ra = abi::r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(0x80));
    c.pc = ra;
}

/// `IntersectInfo_EvalDerivative` (0x00351660): evaluate the 16 parametric
/// curve segments of the intersect info at `t` (f12) and store the velocity
/// in `*(a1)` and the (negated) rotational velocity in `*(a2)`.  For every
/// bit set in the curve mask (`*(*(self+0x40)+0x18)`) the segment selected by
/// the current key (`*(self+i*4)`) is used, re-found with
/// `IntersectInfo_FindKeySegment` when `t` left it; the rest stay 0.
pub fn intersect_info_eval_derivative(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xe0);
        frame.save(c, S1, 0xd0);
        c.g[S0][0] = c.g[SP][0];
        frame.save(c, S3, 0xb0);
        frame.save(c, S4, 0xa0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x90);
        c.g[S1][0] = c.g[S3][0];
        frame.save(c, S6, 0x80);
        c.g[S5][0] = 0x1; // curve mask
        frame.save(c, S7, 0x70);
        c.g[S6][0] = 0x18;
        frame.save(c, FP, 0x60);
        c.g[S7][0] = c.g[A1][0]; // vel out
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf8));
        c.g[FP][0] = c.g[A2][0]; // rotVel out
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S4][0] = 0xf;
        frame.save(c, S2, 0xc0);
        c.mov_s(20, 12); // f20 = t
        frame.save_ra(c, 0x50);
        c.g[AT][0] = 0x4040_0000;
        c.mtc1(1, 21); // f21 = 3.0
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20)); // curves
        loop {
            c.cycles += 5;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.g[V0][0] & c.g[S5][0];
            let off = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw(c.g[S2][0] as u32); // curve
            if !off {
                c.cycles += 9;
                c.g[V0][0] = c.lw(c.g[S1][0] as u32); // segment index
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.mult(0, V0, V0, S6);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.c_le_s(0, 20);
                let outside = !c.fcc();
                c.g[A1][0] = 0u64;
                if !outside {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                    c.c_lt_s(20, 0);
                    if c.fcc() {
                        c.g[A1][0] = 0x1;
                    }
                }
                c.cycles += 2;
                if c.g[A1][0] != 0 {
                    c.mul_s(2, 20, 21);
                } else {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0035_1728);
                    c.mov_s(12, 20);
                    call_ptr(c, 0x0035_1a80, 0x0035_1728)?;
                    c.mul_s(2, 20, 21);
                }
                c.cycles += 13;
                c.lwc1(0, c.g[V0][0] as u32);
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                c.mul_s(0, 0, 2);
                c.add_s(1, 1, 1);
                c.mul_s(0, 0, 20);
                c.mul_s(1, 1, 20);
                c.add_s(0, 0, 1);
                c.add_s(0, 0, 3);
                c.swc1(0, c.g[S0][0] as u32);
            }
            if off {
                c.cycles += 1;
                c.sw(c.g[S0][0] as u32, 0u64);
            }
            c.cycles += 4;
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(1));
            c.g[S5][0] = sx((c.g[S5][0] as u32) << 1);
            let more = (c.g[S4][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_16c8);
            }
        }
        c.cycles += 35;
        c.lwc1(3, c.g[SP][0] as u32);
        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.neg_s(0, 0);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.neg_s(1, 1);
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x44));
        c.neg_s(2, 2);
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
        frame.restore(c, S0, 0xe0);
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x44));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
        c.sq(3, c.g[S7][0] as u32);
        frame.restore(c, S1, 0xd0);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        frame.restore(c, S2, 0xc0);
        c.sq(2, c.g[FP][0] as u32);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// IntersectInfo_Evaluate (0x351538)

/// IntersectInfo: +0 current segment index per animated channel, +0x40 the
/// curve set, +0x44 16 output floats.
pub(crate) const II_CURVE_SET: u32 = 0x40;

pub(crate) const II_OUT: u32 = 0x44;

/// Curve set: +0x18 mask of animated channels, +0x20 curve pointers (one per
/// animated channel).
pub(crate) const CS_MASK: u32 = 0x18;

pub(crate) const CS_CURVES: u32 = 0x20;

/// Curve: +0 segment count, +4 segments (0x18 bytes: cubic a b c d, then
/// the time range [start, end)).
pub(crate) const CURVE_SEGS: u32 = 0x4;

pub(crate) const SEG_SIZE: u64 = 0x18;

pub(crate) const SEG_START: u32 = 0x10;

pub(crate) const SEG_END: u32 = 0x14;

pub(crate) const INTERSECT_INFO_FIND_KEY_SEGMENT: Callee = Callee {
    addr: 0x0035_1a80,
    f: ee::s11::IntersectInfo_FindKeySegment,
};

/// `void IntersectInfo_Evaluate(IntersectInfo* self, float t)` (0x351538):
/// evaluate the animated channels of 16 at time `t`. For each channel whose
/// mask bit is set, the current cubic segment is used when `t` is in its
/// range, else `IntersectInfo_FindKeySegment` finds (and stores) the right
/// one; the output is `((a t + b) t + c) t + d`.
pub fn intersect_info_evaluate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x20);
        c.g[S0][0] = c.g[S3][0]; // current segment index of the channel
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 1; // channel mask bit
        frame.save_fpr(c, 20, 0x80);
        c.g[S2][0] = addu(c.g[S0][0], II_OUT as u64);
        frame.save(c, S1, 0x60);
        let t = abi::farg(c, 0);
        abi::set_fpr(c, 20, t);
        frame.save_ra(c, 0);
        c.g[S6][0] = SEG_SIZE;
        c.g[S4][0] = 0xf;
        let this = Addr(c.g[S3][0] as u32);
        c.g[V0][0] = this.off(II_CURVE_SET).lw(c);
        c.g[S1][0] = Addr(c.g[V0][0] as u32).off(CS_CURVES).lw(c);
        loop {
            c.cycles += 8;
            c.g[V1][0] = this.off(II_CURVE_SET).lw(c);
            c.g[S4][0] = addu(c.g[S4][0], (-1i64) as u64);
            c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[A2][0] = 0;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(CS_MASK).lw(c);
            c.g[V0][0] &= c.g[S5][0];
            let animated = c.g[V0][0] != 0;
            c.g[S5][0] = sx((c.g[S5][0] as u32) << 1);
            if animated {
                c.cycles += 13;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A0][0] = c.g[A1][0];
                c.g[V1][0] = Addr(c.g[A1][0] as u32).off(CURVE_SEGS).lw(c);
                abi::set_fpr(c, 12, t);
                c.mult(0, V0, V0, S6);
                c.g[A1][0] = c.g[S0][0];
                c.g[S1][0] = addu(c.g[S1][0], 4);
                c.g[V0][0] = addu(c.g[V1][0], c.g[V0][0]);
                let seg = Addr(c.g[V0][0] as u32);
                let start = seg.off(SEG_START).read_fpu(c);
                abi::set_fpr(c, 0, start);
                let after_start = start.le(t);
                abi::set_fcc(c, after_start);
                c.g[S0][0] = addu(c.g[S0][0], 4);
                if after_start {
                    c.cycles += 5;
                    let end = seg.off(SEG_END).read_fpu(c);
                    abi::set_fpr(c, 0, end);
                    let before_end = t.lt(end);
                    abi::set_fcc(c, before_end);
                    if before_end {
                        c.g[A2][0] = 1;
                    }
                }
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    c.cycles += 3;
                    INTERSECT_INFO_FIND_KEY_SEGMENT.call(c, 0x0035_1600)?;
                }
                let seg = Addr(c.g[V0][0] as u32);
                let a = seg.read_fpu(c);
                c.cycles += 10;
                let b = seg.off(4).read_fpu(c);
                let cc = seg.off(8).read_fpu(c);
                let d = seg.off(0xc).read_fpu(c);
                let v = ((a * t + b) * t + cc) * t + d;
                abi::set_fpr(c, 1, b);
                abi::set_fpr(c, 2, cc);
                abi::set_fpr(c, 3, d);
                abi::set_fpr(c, 0, v);
                Addr(c.g[S2][0] as u32).write_fpu(c, v);
            }
            c.cycles += 2;
            let more = (c.g[S4][0] as i64) >= 0;
            c.g[S2][0] = addu(c.g[S2][0], 4);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_1588);
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}




// ---------------------------------------------------------------------------
// MeshAnim_Render (0x352810)

/// MeshAnim: the model.
pub(crate) const MESH_ANIM_MODEL: u32 = 0x18;


/// MeshAnim: the fade (nothing is drawn at or below 0).
pub(crate) const MESH_ANIM_FADE: u32 = 0x1c;


/// MeshAnim: the draw alpha limit (the alpha passed is `min(fade, this)`
/// when this is >= 0 and >= fade, else 1.0).
pub(crate) const MESH_ANIM_ALPHA: u32 = 0x24;


/// MeshAnim: the bone rotations (quaternions, 16 bytes each).
pub(crate) const MESH_ANIM_ROTATIONS: u32 = 0x30;


/// MeshAnim: the bone translations (16 bytes each).
pub(crate) const MESH_ANIM_TRANSLATIONS: u32 = 0x3b0;


/// Model: the skeleton (`+4`: bone count, `+0xc`: the part count list).
pub(crate) const MESH_ANIM_SKELETON: u32 = 0x80;


/// Model: the part list (`+4`: part pointers; part `+0`: i16 object id).
pub(crate) const MESH_ANIM_PARTS: u32 = 0x94;


/// `gp+0x16c8`: the object visibility table holder.
pub(crate) const MESH_ANIM_VIS_GP: u32 = 0x16c8;


/// `gp+0x2a90`: the graphics manager.
pub(crate) const MESH_ANIM_GFX_GP: u32 = 0x2a90;


/// Graphics manager: render-state vtable; slot `+0x2f8`: draw skinned mesh.
pub(crate) const MESH_ANIM_GFX_VTABLE: u32 = 0x10d8;


pub(crate) const MESH_ANIM_DRAW_SLOT: u32 = 0x2f8;



/// The visibility test of one part (registers as the original: a2 the
/// packed id, a1 the id, t0 the visibility tables, t1 = !0xff, t8 = 0xff).
/// Leaves v0 = 1 when the part's object is visible.
pub(crate) fn mesh_anim_part_visible(c: &mut Cpu) {
    c.cycles += 11;
    let part = Addr(r32(c, A3)).lw(c);
    c.g[V0][0] = part;
    c.g[A2][0] &= c.g[T6][0];
    c.g[A2][0] |= c.g[T7][0];
    c.g[V1][0] = Addr(part as u32).lh(c);
    c.g[A0][0] = c.g[A2][0] & 0xff;
    c.g[V1][0] = sx(r32(c, V1) << 8);
    c.g[A0][0] |= c.g[V1][0];
    c.g[A2][0] = c.g[A0][0];
    c.g[A0][0] = c.g[A2][0] & 0xff;
    let other_table = c.g[A0][0] != c.g[T8][0];
    c.g[A1][0] = sx(r32(c, A2) >> 8);
    let t0 = r32(c, T0);
    // The object record (v1), or invisible.
    let have = if other_table {
        // (A table index other than 0xff: never taken, the low byte is
        // always 0xff here.)
        c.cycles += 5;
        c.g[V1][0] = Addr(t0).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(8).lw(c);
        let inside = c.g[A0][0] < c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A0) << 2);
        if !inside {
            c.cycles += 1;
            c.g[V0][0] = 0;
            false
        } else {
            c.cycles += 5;
            c.g[V1][0] = Addr(t0).off(8).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            let some = c.g[V1][0] != 0;
            c.g[V0][0] = 0;
            some
        }
    } else {
        c.cycles += 2;
        c.g[V1][0] = Addr(t0).off(4).lw(c);
        true
    };
    if have {
        c.cycles += 4;
        let rec = r32(c, V1);
        c.g[V0][0] = Addr(rec).off(0x50).lw(c);
        if c.g[V0][0] < c.g[A1][0] {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V0][0] = Addr(rec).off(0x4c).lw(c);
            c.g[V1][0] = sx(r32(c, A1) << 2);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            c.g[A0][0] &= c.g[T1][0];
            c.g[V0][0] = (c.g[A0][0] != 0) as u64;
        }
    }
}



/// One bone of the matrix loop: the 3x3 rotation of the quaternion at
/// `a0 - 0x380` (VU0 macro code, kept op for op) with the translation at
/// `a0` as the fourth row, written to the stack at v1 (0x40 bytes).
pub(crate) fn mesh_anim_bone_matrix(c: &mut Cpu) {
    let a0 = r32(c, A0);
    let out = r32(c, V1);
    c.lqc2(4, a0.wrapping_sub(MESH_ANIM_TRANSLATIONS - MESH_ANIM_ROTATIONS));
    let v = c.vu0();
    v.add(0b1110, 1, 0, Src::Bc(0, W));
    v.add(0b1110, 5, 4, Src::Vec(4));
    v.sub(0b0001, 10, 10, Src::Vec(10));
    v.sub(0b0001, 11, 11, Src::Vec(11));
    v.sub(0b0001, 12, 12, Src::Vec(12));
    v.mul(0b1110, 6, 5, Src::Vec(4));
    v.mul(0b1110, 7, 5, Src::Bc(4, W));
    v.opmula(5, 4);
    v.madd(0b1110, 8, 0, Src::Vec(0));
    v.suba(0b1000, 1, Src::Bc(6, Y));
    v.msub(0b1000, 10, 1, Src::Bc(6, Z));
    v.suba(0b0100, 1, Src::Bc(6, Z));
    v.msub(0b0100, 11, 1, Src::Bc(6, X));
    v.suba(0b0010, 1, Src::Bc(6, X));
    v.msub(0b0010, 12, 1, Src::Bc(6, Y));
    v.adda(0b0100, 0, Src::Bc(8, Z));
    v.madd(0b0100, 10, 1, Src::Bc(7, Z));
    v.adda(0b0010, 0, Src::Bc(8, X));
    v.madd(0b0010, 11, 1, Src::Bc(7, X));
    v.adda(0b0100, 0, Src::Bc(8, X));
    v.msub(0b0100, 12, 1, Src::Bc(7, X));
    v.adda(0b0010, 0, Src::Bc(8, Y));
    v.msub(0b0010, 10, 1, Src::Bc(7, Y));
    v.adda(0b1000, 0, Src::Bc(8, Z));
    v.msub(0b1000, 11, 1, Src::Bc(7, Z));
    v.adda(0b1000, 0, Src::Bc(8, Y));
    v.madd(0b1000, 12, 1, Src::Bc(7, Y));
    c.sqc2(0, out + 0x30);
    c.sqc2(10, out);
    c.sqc2(11, out + 0x10);
    c.sqc2(12, out + 0x20);
    c.lq(2, a0);
    c.sq(2, out + 0x30);
}



/// `void MeshAnim_Render(MeshAnim* self)` (0x352810): draw an animated mesh
/// instance. Nothing when its fade is <= 0 or when any of the model's parts
/// belongs to an object that is not visible this frame. Otherwise builds
/// one 4x4 matrix per bone on the stack (rotation from the bone quaternion,
/// translation row) and calls the graphics manager's skinned-mesh draw
/// (slot `+0x2f8`) with the model, its part list, the matrices, the alpha
/// (f12) and the fixed draw parameters (a3 = 0x4ff1a0, t1 = 0, t2 = 0x420).
pub fn mesh_anim_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x610);
        c.g[T4][0] = c.g[A0][0];
        let me = r32(c, T4);
        frame.save_ra(c, 0x600);
        c.mtc1(0, 1);
        c.lwc1(0, me + MESH_ANIM_FADE);
        c.c_le_s(0, 1);
        let faded = c.fcc();
        frame.restore_ra(c, 0x600);
        if faded {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }

        // Every part's object must be visible.
        c.cycles += 7;
        c.g[V0][0] = Addr(me).off(MESH_ANIM_MODEL).lw(c);
        c.g[T2][0] = 0;
        let model = r32(c, V0);
        c.g[V1][0] = Addr(model).off(MESH_ANIM_SKELETON).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
        c.g[T3][0] = Addr(r32(c, A0)).lw(c);
        c.g[T5][0] = c.g[V0][0];
        if (c.g[T3][0] as i64) > 0 {
            c.cycles += 10;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_add(MESH_ANIM_VIS_GP)).lw(c);
            c.g[T1][0] = 0xffff_ffff_ffff_0000;
            c.g[V1][0] = Addr(r32(c, T5)).off(MESH_ANIM_PARTS).lw(c);
            c.g[T8][0] = 0xff;
            c.g[T0][0] = Addr(r32(c, V0)).lw(c);
            c.g[T7][0] = 0xff;
            c.g[A3][0] = sx(r32(c, V1).wrapping_add(4));
            c.g[T6][0] = 0xffff_ffff_ffff_ff00;
            c.g[T1][0] |= 0xff00;
            loop {
                mesh_anim_part_visible(c);
                c.cycles += 3;
                c.g[V0][0] ^= 1;
                let hidden = c.g[V0][0] != 0;
                c.g[T2][0] = sx(r32(c, T2).wrapping_add(1));
                if hidden {
                    c.cycles += 1;
                    frame.restore_ra(c, 0x600);
                    c.cycles += 2;
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 3;
                let more = (c.g[T2][0] as i64) < (c.g[T3][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_2878);
                }
            }
        }

        // The bone matrices.
        c.cycles += 7;
        c.g[V0][0] = 0x17;
        c.g[V1][0] = u64::MAX;
        c.g[A2][0] = Addr(r32(c, T5)).off(MESH_ANIM_SKELETON).lw(c);
        c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
        c.g[A1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[A0][0] = sx(me.wrapping_add(MESH_ANIM_TRANSLATIONS));
            c.g[V1][0] = c.g[SP][0];
            loop {
                c.cycles += 40;
                mesh_anim_bone_matrix(c);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                let more = (c.g[A1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x40));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_2940);
                }
            }
        }

        // The alpha (f12): 1.0 unless 0 <= limit, then the fade if it is
        // within the limit.
        c.cycles += 8;
        c.lwc1(1, me + MESH_ANIM_ALPHA);
        c.mtc1(0, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        c.c_le_s(0, 1);
        let limited = c.fcc();
        c.g[T3][0] = Addr(r32(c, GP).wrapping_add(MESH_ANIM_GFX_GP)).lw(c);
        if limited {
            c.cycles += 5;
            c.lwc1(0, me + MESH_ANIM_FADE);
            c.c_le_s(0, 1);
            if c.fcc() {
                c.mov_s(12, 0);
            } else {
                c.cycles += 1;
                c.g[T3][0] = Addr(r32(c, GP).wrapping_add(MESH_ANIM_GFX_GP)).lw(c);
            }
        }

        c.cycles += 12;
        c.g[A3][0] = sx(0x0050_0000u32.wrapping_sub(0xe60));
        c.g[A1][0] = Addr(me).off(MESH_ANIM_MODEL).lw(c);
        let gfx = r32(c, T3);
        c.g[V1][0] = Addr(gfx).off(MESH_ANIM_GFX_VTABLE).lw(c);
        c.g[T0][0] = c.g[SP][0];
        c.g[A2][0] = Addr(r32(c, A1)).off(MESH_ANIM_PARTS).lw(c);
        c.g[T1][0] = 0;
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(MESH_ANIM_DRAW_SLOT).lh(c);
        c.g[T2][0] = 0x420;
        c.g[V0][0] = Addr(vt).off(MESH_ANIM_DRAW_SLOT + 4).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(gfx.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0035_2a48)?;
        c.cycles += 1;
        frame.restore_ra(c, 0x600);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}




// ---------------------------------------------------------------------------
// MeshAnim_Update (0x352500)

/// Debris/mesh-anim instance fields (`sDebris` in instanceman.cpp).
pub(crate) const MESH_ANIM_UPD_VTABLE: u32 = 0xc;


pub(crate) const MESH_ANIM_UPD_MODEL: u32 = 0x18;


/// Remaining life in seconds; the instance kills itself when it runs out.
pub(crate) const MESH_ANIM_UPD_TIMER: u32 = 0x1c;


/// Per-part rotation quaternions (0x10 each).
pub(crate) const MESH_ANIM_UPD_ROT: u32 = 0x30;


/// Per-part spin angles (4 each).
pub(crate) const MESH_ANIM_UPD_SPIN: u32 = 0x330;


/// Per-part velocities (0x10 each); positions are 0x180 below.
pub(crate) const MESH_ANIM_UPD_VEL: u32 = 0x530;


pub(crate) const MESH_ANIM_UPD_GRAVITY: u32 = 0x6b4;


/// Model -> skeleton (count of parts at +4).
pub(crate) const MESH_ANIM_UPD_SKELETON: u32 = 0x80;


/// gp-relative pointer to the game timer (frame time at +0x14).
pub(crate) const MESH_ANIM_UPD_TIMER_GP: u32 = 0x2a74;


/// Vtable slot of the kill method (`vt[34]`).
pub(crate) const MESH_ANIM_UPD_KILL_SLOT: u32 = 0x110;


pub(crate) const MESH_ANIM_UPD_SIN_COS: u32 = 0x0031_be50;



/// `void MeshAnim_Update(sDebris* self)` (0x352500): count the debris life
/// timer down by the frame time and kill the instance (`vt[34](self, 1)`)
/// when it runs out; while alive, spin every part (rotation quaternion
/// times the axis-angle quaternion of half its spin, renormalised) and move
/// it (position += velocity, velocity.z -= gravity * 980 * dt).
pub fn mesh_anim_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xb0);
        c.mtc1(0, 2);
        frame.save(c, S2, 0x70);
        frame.save(c, S0, 0x90);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 20, 0xa0);
        let me = r32(c, S2);
        c.lwc1(1, me + MESH_ANIM_UPD_TIMER);
        c.c_lt_s(2, 1);
        let alive = c.fcc();
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(MESH_ANIM_UPD_TIMER_GP)).lw(c);
        if alive {
            // timer -= dt
            c.cycles += 6;
            c.lwc1(0, r32(c, V0).wrapping_add(0x14));
            c.sub_s(0, 1, 0);
            c.c_le_s(0, 2);
            let expired = c.fcc();
            c.swc1(0, me + MESH_ANIM_UPD_TIMER);
            if expired {
                c.cycles += 8;
                c.g[V0][0] = Addr(me + MESH_ANIM_UPD_VTABLE).lw(c);
                c.g[A1][0] = 0x1;
                let vt = r32(c, V0);
                c.g[A0][0] = Addr(vt + MESH_ANIM_UPD_KILL_SLOT).lh(c);
                c.g[V1][0] = Addr(vt + MESH_ANIM_UPD_KILL_SLOT + 4).lw(c);
                let kill = r32(c, V1);
                c.g[A0][0] = sx(me.wrapping_add(r32(c, A0)));
                call_ptr(c, kill, 0x0035_256c)?;
                frame.restore(c, S0, 0x90);
                mesh_anim_update_epilogue(c, frame);
                return Ok(());
            }
            c.cycles += 1;
            c.lwc1(1, me + MESH_ANIM_UPD_TIMER);
        }

        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_lt_s(0, 1);
        let alive = c.fcc();
        frame.restore(c, S0, 0x90);
        if !alive {
            mesh_anim_update_epilogue(c, frame);
            return Ok(());
        }

        // Spin the parts.
        c.cycles += 5;
        c.g[V1][0] = Addr(me + MESH_ANIM_UPD_MODEL).lw(c);
        c.g[S4][0] = Addr(r32(c, V1) + MESH_ANIM_UPD_SKELETON).lw(c);
        c.g[V0][0] = Addr(r32(c, S4) + 4).lw(c);
        c.g[S3][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 20); // f20 = 0.5
            c.g[S0][0] = sx(me.wrapping_add(MESH_ANIM_UPD_ROT));
            c.g[S1][0] = sx(me.wrapping_add(MESH_ANIM_UPD_SPIN));
            loop {
                c.cycles += 48;
                c.lwc1(12, r32(c, S1));
                c.g[A0][0] = sx(sp(c).wrapping_add(0x30)); // &sin
                c.g[A1][0] = sx(sp(c).wrapping_add(0x34)); // &cos
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.mul_s(12, 12, 20);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                call_ptr(c, MESH_ANIM_UPD_SIN_COS, 0x0035_25cc)?;
                // The axis-angle quaternion (s*axis, c) at sp+0x10.
                let rot = r32(c, S0);
                let s = sp(c);
                c.lwc1(2, rot + 0x180);
                c.lwc1(3, rot + 0x184);
                c.lwc1(1, s + 0x30);
                c.lwc1(4, rot + 0x188);
                c.mul_s(2, 1, 2);
                c.lwc1(0, s + 0x34);
                c.mul_s(3, 1, 3);
                c.mul_s(1, 1, 4);
                c.swc1(0, s + 0x1c);
                c.swc1(2, s + 0x10);
                c.swc1(3, s + 0x14);
                c.swc1(1, s + 0x18);
                // rot = rot * q
                c.lqc2(4, rot);
                c.lqc2(5, s + 0x10);
                c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                c.sqc2(8, s + 0x20);
                c.lq(2, s + 0x20);
                c.sq(2, rot);
                c.sq(2, s);
                // Normalise.
                c.lqc2(3, rot);
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, rot);
                c.g[V0][0] = Addr(r32(c, S4) + 4).lw(c);
                let more = (c.g[S3][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = sx(rot.wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_25b0);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S4) + 4).lw(c);
        }

        // Move the parts.
        c.cycles += 2;
        c.g[A0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[AT][0] = 0xffff_ffff_c475_0000;
            c.mtc1(1, 2); // f2 = -980.0
            c.g[V1][0] = sx(me.wrapping_add(MESH_ANIM_UPD_VEL));
            loop {
                c.cycles += 22;
                let vel = r32(c, V1);
                let pos = vel.wrapping_sub(0x180);
                c.lqc2(3, pos);
                c.lqc2(4, vel);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, pos);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                c.lwc1(0, r32(c, S2) + MESH_ANIM_UPD_GRAVITY);
                c.g[V0][0] = Addr(r32(c, GP).wrapping_add(MESH_ANIM_UPD_TIMER_GP)).lw(c);
                c.mul_s(0, 0, 2);
                c.lwc1(1, r32(c, V0).wrapping_add(0x14));
                let s = sp(c);
                Addr(s).sw(c, 0);
                c.mul_s(0, 0, 1);
                Addr(s + 4).sw(c, 0);
                Addr(s + 0xc).sw(c, 0);
                c.swc1(0, s + 8);
                c.lqc2(3, vel);
                c.lqc2(4, s);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, vel);
                c.g[V0][0] = Addr(r32(c, S4) + 4).lw(c);
                let more = (c.g[A0][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[V1][0] = sx(vel.wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_2688);
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x90);
        mesh_anim_update_epilogue(c, frame);
        Ok(())
    })
}



/// MeshAnim_Update's shared epilogue (0x3526e4).
pub(crate) fn mesh_anim_update_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 8;
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore_ra(c, 0x40);
    frame.restore_fpr(c, 20, 0xa0);
    frame.close(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD;
    pub(crate) use crate::game::fe::fepopup::tests::f_2 as f;
    pub(crate) use crate::game::input::inputmap::tests::put_f32;
    pub(crate) use crate::game::intersect::worldsphtree::tests::fake_stream;
    pub(crate) use crate::game::intersect::worldsphtree::tests::save_test;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS;
    pub(crate) use crate::game::world::worldcache::tests::check_all;

    #[test]
    fn instances_man_resolve_handle_matches() {
        check_lohi(
            ssx3_ee::s11::f_003512c0,
            instances_man_resolve_handle,
            200,
            &|c, seed| {
                let mut r = Rng(0x1019 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = (seed % 4) as u32;
                let handle = r.next_u32() & 0xff;
                let stored = if seed % 3 == 0 {
                    handle
                } else {
                    (handle + 1) & 0xff
                };
                c.g[A1][0] = ((index << 8) | handle) as u64;
                c.bus.write32(DATA, stored);
                c.bus.write32(DATA + 0xc, DATA + 0x200);
                let p = DATA + 0x200 + index * 0xb0;
                c.bus.write32(p + 8, r.next_u32());
                c.bus.write8(p + 0x78, (r.next_u32() & 0xff) as u8);
            },
        );
    }

    #[test]
    fn intersect_info_set_source_matches() {
        check(
            ssx3_ee::s11::f_00351398,
            intersect_info_set_source,
            200,
            &|c, seed| {
                let mut r = Rng(0x101a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA as u64 + 0x100) | ((r.next_u32() as u64) << 32);
                for i in 0..6u32 {
                    c.bus.write32(DATA + 0x100 + i * 4, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn intersect_info_get_duration_matches() {
        check(
            ssx3_ee::s11::f_00351508,
            intersect_info_get_duration,
            200,
            &|c, seed| {
                let mut r = Rng(0x101b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let action = DATA + 0x100;
                c.bus.write32(DATA + 0x40, action);
                let (cur, target) = match seed % 4 {
                    0 => (0, 0),
                    1 => (DATA + 0x200, 0),
                    2 => (DATA + 0x200, DATA + 0x300),
                    _ => (DATA + 0x200, DATA + 0x300),
                };
                c.bus.write32(action + 0x20, cur);
                c.bus.write32(cur, target);
                let value = if seed % 4 == 2 { 0 } else { DATA + 0x400 };
                c.bus.write32(target + 4, value);
                c.bus.write32(DATA + 0x400 + 0x10, r.edge_f32());
            },
        );
    }

    #[test]
    fn intersect_info_evaluate_matrix_matches() {
        check(
            ssx3_ee::s11::f_00351800__generated,
            intersect_info_evaluate_matrix,
            200,
            &|c, seed| {
                let mut r = Rng(0x3518_00 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let selfp = DATA;
                c.g[A0][0] = selfp as u64;
                c.bus.write32(selfp + 0x40, 0);
                for k in 0..3 {
                    put_f32(c, selfp + 0x50 + 4 * k, r.f32(360.0));
                    put_f32(c, selfp + 0x44 + 4 * k, r.f32(1000.0));
                }
                put_f32(c, GAME_GP.wrapping_sub(0x2a90), 0x3c8e_fa36);
            },
        );
    }

    #[test]
    fn mesh_anim_save_matches() {
        save_test(
            ee::s11::f_00352a58__generated,
            mesh_anim_save,
            0x352a_58,
            0x1c,
        );
    }

    #[test]
    fn mesh_anim_kill_restorable_matches() {
        check(
            ee::s11::f_00352708__generated,
            mesh_anim_kill_restorable,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x352708, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let dead = match seed % 3 {
                    0 => 0,
                    1 => 2,
                    _ => 7,
                };
                w(c, this + 0x6b0, dead);
            },
        );
    }

    #[test]
    fn intersect_info_mul_matrix_matches() {
        check(
            ee::s11::f_00351948__generated,
            intersect_info_mul_matrix,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3519, 0);
                game_globals(c);
                let this = DATA;
                let m = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = m as u64;
                for i in 0..32 {
                    c.bus.write32(m + 4 * i, r.edge_f32());
                    c.bus.write32(this + 0x90 + 4 * i, r.edge_f32());
                }
                let _ = seed;
            },
        );
    }

    #[test]
    fn intersect_info_eval_derivative_matches() {
        const S: u64 = 0x0035_1660;
        check_all(
            ee::s11::f_00351660__generated,
            intersect_info_eval_derivative,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, S, 0);
                let this = DATA + 0x1000;
                let curveset = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x6000); // velocity out
                c.g[A2][0] = sx(DATA + 0x6100); // rotational velocity out
                c.f[12] = r.edge_f32();
                w(c, this + 0x40, curveset);
                let mask = [0xffffu32, 0x5555, 0x00ff, 0x0001][seed as usize % 4];
                w(c, curveset + 0x18, mask);
                w(c, curveset + 0x20, DATA + 0x3000);
                for i in 0..16u32 {
                    w(c, this + i * 4, r.next_u32() % 3); // current segment index
                }
                for j in 0..16u32 {
                    let curve = DATA + 0x4000 + j * 0x100;
                    w(c, DATA + 0x3000 + j * 4, curve);
                    w(c, curve, 3); // segment count
                    w(c, curve + 4, DATA + 0x5000); // segments
                }
                for k in 0..8u32 {
                    let seg = DATA + 0x5000 + k * 0x18;
                    w(c, seg, r.edge_f32()); // a
                    w(c, seg + 4, r.edge_f32()); // b
                    w(c, seg + 8, r.edge_f32()); // c
                                                 // The segment's range sometimes contains t, sometimes not.
                    if seed % 2 == 0 {
                        w(c, seg + 0x10, 0u32); // 0.0
                        w(c, seg + 0x14, 0x40a0_0000); // 5.0
                    } else {
                        w(c, seg + 0x10, 0x40a0_0000);
                        w(c, seg + 0x14, 0x4120_0000); // 10.0
                    }
                }
            },
        );
    }
    use super::*;
    use crate::difftest::{check_paths, fill};

    /// IntersectInfo_Evaluate: random channel masks, `t` inside the current
    /// segment, in the next one, elsewhere and outside all of them (the real
    /// FindKeySegment searches).
    #[test]
    fn intersect_info_evaluate_matches() {
        check(
            ee::s11::f_00351538__generated,
            intersect_info_evaluate,
            240,
            &|c, seed| {
                let mut r = Rng(0x3515_3800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                let set = DATA + 0x200;
                let curves = DATA + 0x300;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + II_CURVE_SET, set);
                let mask = match seed % 4 {
                    0 => 0xffff,
                    1 => 0,
                    _ => r.next_u32() & 0xffff,
                };
                c.bus.write32(set + CS_MASK, mask);
                c.bus.write32(set + CS_CURVES, curves);
                for i in 0..16 {
                    let curve = DATA + 0x400 + 0x10 * i;
                    let segs = DATA + 0x1000 + 0x200 * i;
                    let count = 1 + r.next_u32() % 5;
                    c.bus.write32(curves + 4 * i, curve);
                    c.bus.write32(curve, count);
                    c.bus.write32(curve + CURVE_SEGS, segs);
                    for j in 0..count {
                        let seg = segs + 0x18 * j;
                        for k in 0..4 {
                            c.bus.write32(seg + 4 * k, r.f32(3.0));
                        }
                        c.bus.write32(seg + SEG_START, (j as f32).to_bits());
                        c.bus.write32(seg + SEG_END, (j as f32 + 1.0).to_bits());
                    }
                    c.bus.write32(this + 4 * i, r.next_u32() % count);
                    c.bus.write32(this + II_OUT + 4 * i, r.next_u32());
                }
                let t = match seed % 5 {
                    0 => (r.next_u32() % 6) as f32,
                    1 => -0.5,
                    _ => (r.next_u32() % 600) as f32 / 100.0,
                };
                c.f[12] = t.to_bits();
            },
        );
    }
    use super::*;
    use crate::difftest::fakes;
    use crate::game::visualfx::renderstateman::tests::table_getter_setup;


    /// A mesh instance with fade <= 0 (nothing drawn) or > 0; 0..4 parts
    /// whose objects are visible (flag word with bits above the low byte),
    /// hidden or out of the table's range; 0..4 bones with random
    /// quaternions and translations; the alpha limit negative, below or
    /// above the fade. The draw method is a fake (unwinding on some seeds).
    #[test]
    fn mesh_anim_render_matches() {
        use crate::difftest::{put_vec, GAME_GP};
        fakes::covered(ee::s11::f_00352810__generated, mesh_anim_render, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x35_2810, 1);
            let me = DATA + 0x1000;
            let model = DATA + 0x2000;
            let skeleton = DATA + 0x2100;
            let counts = DATA + 0x2200;
            let parts = DATA + 0x2300;
            let vis_holder = DATA + 0x2500;
            let vis = DATA + 0x2600;
            let rec = DATA + 0x2700;
            let flags = DATA + 0x2800;
            let fade = match seed % 9 {
                0 => 0.0f32,
                1 => -0.5,
                _ => f32::from_bits(r.f32(1.0)).abs() + 0.01,
            };
            c.bus.write32(me + MESH_ANIM_FADE, fade.to_bits());
            let alpha = match seed / 9 % 3 {
                0 => -1.0f32,
                1 => fade * 0.5,
                _ => fade + 1.0,
            };
            c.bus.write32(me + MESH_ANIM_ALPHA, alpha.to_bits());
            c.bus.write32(me + MESH_ANIM_MODEL, model);
            c.bus.write32(model + MESH_ANIM_SKELETON, skeleton);
            c.bus.write32(skeleton + 0xc, counts);
            let n_parts = r.next_u32() % 5;
            c.bus.write32(counts, n_parts);
            c.bus.write32(model + MESH_ANIM_PARTS, parts);
            // Visibility: object ids 0..7 against a table of 6.
            c.bus.write32(GAME_GP + MESH_ANIM_VIS_GP, vis_holder);
            c.bus.write32(vis_holder, vis);
            c.bus.write32(vis + 4, rec);
            c.bus.write32(rec + 0x50, 6);
            c.bus.write32(rec + 0x4c, flags);
            let all_visible = seed % 3 != 0;
            for i in 0..8 {
                let w = if all_visible || r.next_u32() % 4 != 0 {
                    (r.next_u32() | 0x100) & !0xff | (r.next_u32() & 0xff)
                } else {
                    r.next_u32() & 0xff
                };
                c.bus.write32(flags + 4 * i, w);
            }
            for i in 0..n_parts {
                let part = DATA + 0x2400 + 0x10 * i;
                c.bus.write32(parts + 4 + 4 * i, part);
                let id = if all_visible { r.next_u32() % 7 } else { r.next_u32() % 9 };
                c.bus.write16(part, id as u16);
            }
            // Bones.
            let n_bones = r.next_u32() % 5;
            c.bus.write32(skeleton + 4, n_bones);
            for i in 0..4 {
                put_vec(
                    c,
                    me + MESH_ANIM_ROTATIONS + 0x10 * i,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)],
                );
                put_vec(
                    c,
                    me + MESH_ANIM_TRANSLATIONS + 0x10 * i,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), 1.0f32.to_bits()],
                );
            }
            // The graphics manager with a fake draw method.
            let gfx = DATA + 0x8000;
            let vt = DATA + 0x9000;
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + MESH_ANIM_DRAW_SLOT, delta as u16);
            c.bus.write32(vt + MESH_ANIM_DRAW_SLOT + 4, fakes::FAKE);
            c.bus.write32(GAME_GP + MESH_ANIM_GFX_GP, gfx);
            c.bus.write32(gfx + MESH_ANIM_GFX_VTABLE, vt);
            c.g[A0][0] = sx(me);
            c.pc = 0x0035_2810;
        });
    }
    use super::*;
    use crate::difftest::{elf_image, put_vec};


    /// MeshAnim_Update: a debris instance whose timer is <= 0 (idle), runs
    /// out this frame (kill method, a fake that unwinds on some seeds) or
    /// stays alive; 0..5 parts (or a negative count) spun through the real
    /// Math_SinCos and moved under gravity.
    #[test]
    fn mesh_anim_update_matches() {
        covered(ee::s11::f_00352500__generated, mesh_anim_update, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x35_2500, 1);
            elf_image(c);
            let me = DATA + 0x1000;
            let model = DATA + 0x3000;
            let skeleton = DATA + 0x3100;
            let timer_obj = DATA + 0x3200;
            let vt = DATA + 0x3400;
            let dt = 1.0f32 / 60.0;
            c.bus.write32(GAME_GP + MESH_ANIM_UPD_TIMER_GP, timer_obj);
            c.bus.write32(timer_obj + 0x14, dt.to_bits());
            let timer = match seed % 4 {
                0 => -((r.next_u32() % 3) as f32) * 0.5,
                1 => dt * 0.5,
                _ => 1.0 + (r.next_u32() % 100) as f32 * 0.01,
            };
            c.bus.write32(me + MESH_ANIM_UPD_TIMER, timer.to_bits());
            c.bus.write32(me + MESH_ANIM_UPD_VTABLE, vt);
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + MESH_ANIM_UPD_KILL_SLOT, delta as u16);
            c.bus.write32(vt + MESH_ANIM_UPD_KILL_SLOT + 4, FAKE);
            c.bus.write32(me + MESH_ANIM_UPD_MODEL, model);
            c.bus.write32(model + MESH_ANIM_UPD_SKELETON, skeleton);
            let parts: i32 = match seed % 7 {
                0 => 0,
                1 => -1,
                _ => (r.next_u32() % 5 + 1) as i32,
            };
            c.bus.write32(skeleton + 4, parts as u32);
            c.bus.write32(me + MESH_ANIM_UPD_GRAVITY, r.f32(2.0));
            for i in 0..6 {
                put_vec(c, me + MESH_ANIM_UPD_ROT + 0x10 * i,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0.5f32.to_bits()]);
                put_vec(c, me + MESH_ANIM_UPD_ROT + 0x180 + 0x10 * i,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.next_u32()]);
                c.bus.write32(me + MESH_ANIM_UPD_SPIN + 4 * i, r.f32(6.0));
                put_vec(c, me + MESH_ANIM_UPD_VEL - 0x180 + 0x10 * i,
                    [r.f32(500.0), r.f32(500.0), r.f32(500.0), r.next_u32()]);
                put_vec(c, me + MESH_ANIM_UPD_VEL + 0x10 * i,
                    [r.f32(50.0), r.f32(50.0), r.f32(50.0), r.next_u32()]);
            }
            c.g[A0][0] = sx(me);
            c.pc = 0x0035_2500;
        });
    }
}
