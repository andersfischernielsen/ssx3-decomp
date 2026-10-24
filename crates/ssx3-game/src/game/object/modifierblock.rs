//! `src/object/modifierblock.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ridermetrix::swc1_r;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::movie::movieplayer::add32;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::wscript::wscriptman::p;
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
    (0x002d_1c20, world_get_camera_eye, "World_GetCameraEye"),
    (
        0x002d_1c58,
        world_get_current_view_index,
        "World_GetCurrentViewIndex",
    ),
    (0x0035_2aa8, modifier_block_ctor, "ModifierBlock_ctor"),
    (
        0x0035_2bf8,
        modifier_block_get_transform_radius,
        "ModifierBlock_GetTransformRadius",
    ),
    (
        0x0035_2e50,
        modifier_block_has_transform_changed,
        "ModifierBlock_HasTransformChanged",
    ),
    (
        0x0035_2ed0,
        modifier_block_is_transform_finished,
        "ModifierBlock_IsTransformFinished",
    ),
    (
        0x0035_2f40,
        modifier_block_handle_event,
        "ModifierBlock_HandleEvent",
    ),
    (
        0x0035_3020,
        modifier_block_forward_v43,
        "ModifierBlock_ForwardV43",
    ),
    (
        0x0035_3098,
        modifier_block_forward_v42,
        "ModifierBlock_ForwardV42",
    ),
    (
        0x0035_3228,
        modifier_block_reset_particles,
        "ModifierBlock_ResetParticles",
    ),
    (
        0x0035_3278,
        modifier_block_remove_dead_particles,
        "ModifierBlock_RemoveDeadParticles",
    ),
    (
        0x0035_3418,
        modifier_block_has_rail_modifier,
        "ModifierBlock_HasRailModifier",
    ),
    (0x0035_3ac0, sky_box_ctor, "SkyBox_ctor"),
    (
        0x0035_3da8,
        flex_bridge_springs_ctor,
        "FlexBridge_SpringsCtor",
    ),
    (
        0x0035_3db8,
        flex_bridge_springs_dtor,
        "FlexBridge_SpringsDtor",
    ),
    (
        0x0035_3e08,
        flex_bridge_springs_init,
        "FlexBridge_SpringsInit",
    ),
    (
        0x0035_3e80,
        flex_bridge_springs_load,
        "FlexBridge_SpringsLoad",
    ),
];

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
}

// ---------------------------------------------------------------------------
// Callees (generated shard functions, reached through their override slots).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE as CBE_GET_BE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES as FLEX_BRIDGE_ALLOC_SPRING_NODES_2;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS as RIDER_METRIX_RESET_STATS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
}

// ---------------------------------------------------------------------------
// World.

/// `int World_GetCurrentViewIndex(void)` (0x2d1c58): follows three pointers out
/// of the world singleton (at `gp - 0x848`, then +0x84, +0x84) and returns the
/// view index at +0x14.
pub fn world_get_current_view_index(c: &mut Cpu) {
    c.cycles += 5;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let views = Addr(world as u32).off(0x84).lw(c);
    c.g[V1][0] = views;
    let view = Addr(views as u32).off(0x84).lw(c);
    c.g[A0][0] = view;
    c.g[V0][0] = Addr(view as u32).off(0x14).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World camera

/// `Vec3* World_GetCameraEye(World* w, int view)` (0x2d1c20): return
/// `view[view].eye + 0x20`.
pub fn world_get_camera_eye(c: &mut Cpu) {
    c.cycles += 8;
    let view = abi::arg(c, 0) as u32;
    let world = Addr(c.g[GP][0] as u32 - 0x848).lw(c);
    let viewer = Addr(world as u32).off(0x84).lw(c);
    let table = Addr(viewer as u32).off(0x84).lw(c);
    let entry = (table as u32).wrapping_add(view.wrapping_shl(2));
    c.g[A1][0] = world;
    c.g[A0][0] = sx(view.wrapping_shl(2));
    c.g[V1][0] = sx(entry);
    let eye = Addr(entry).off(0x4).lw(c);
    c.g[V0][0] = sx((eye as u32).wrapping_add(0x20));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Assorted constructors.

/// `ModifierBlock* ModifierBlock_ctor(ModifierBlock *b)` (0x352aa8): install
/// the vtable pointers and clear the block.
pub fn modifier_block_ctor(c: &mut Cpu) {
    c.cycles += 16;
    let b = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x490000;
    c.g[V1][0] = 0x490000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1340));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1200));
    b.off(0x18).sw(c, c.g[V0][0]);
    b.off(0x24).sw(c, c.g[V1][0]);
    c.g[V0][0] = c.g[A0][0];
    b.off(0x10).sw(c, 0);
    b.off(0x14).sw(c, 0);
    b.off(0x1c).sw(c, 0);
    b.off(0x20).sw(c, 0);
    b.sw(c, 0);
    b.off(4).sw(c, 0);
    b.off(8).sw(c, 0);
    b.off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `SkyBox* SkyBox_ctor(SkyBox *s)` (0x353ac0): install the vtable pointer and
/// clear the sky box.
pub fn sky_box_ctor(c: &mut Cpu) {
    c.cycles += 10;
    let s = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x490000;
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xff8));
    s.off(0x10).sw(c, c.g[V1][0]);
    s.off(0xc).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    s.off(4).sw(c, 0);
    s.off(8).sw(c, 0);
    s.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `FlexBridge_Springs* FlexBridge_SpringsCtor(FlexBridge_Springs *s)`
/// (0x353da8): return the object with its spring count cleared.
pub fn flex_bridge_springs_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).off(0x24).sw(c, 0);
    abi::ret(c);
}

/// `bool ModifierBlock_HasTransformChanged(ModifierBlock* b)` (0x352e50):
/// false when the block has no bound-box node, else slot 0x88's answer.
pub fn modifier_block_has_transform_changed(c: &mut Cpu) {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = Addr(abi::arg(c, 0)).lw(c);
    let node = c.g[A1][0];
    if node == 0 {
        c.g[V0][0] = 0;
    } else {
        c.cycles += 8;
        c.g[V1][0] = Addr(node as u32).lw(c);
        let vt = c.g[V1][0];
        c.g[A0][0] = Addr(vt as u32).off(0x88).lh(c);
        c.g[V0][0] = Addr(vt as u32).off(0x8c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0035_2e78);
        c.g[A0][0] = abi::addu(node, c.g[A0][0]);
        if call_ptr(c, target, 0x0035_2e78).is_err() {
            return;
        }
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 3;
    c.g[RA][0] = Addr(sp(c)).ld(c);
    frame.close(c);
}

/// `bool ModifierBlock_IsTransformFinished(ModifierBlock* b)` (0x352ed0):
/// false without a node, else the node's slot 0x30 result.
pub fn modifier_block_is_transform_finished(c: &mut Cpu) {
    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = Addr(abi::arg(c, 0)).lw(c);
    let node = c.g[A1][0];
    if node == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.g[V1][0] = Addr(node as u32).lw(c);
        let vt = c.g[V1][0];
        c.cycles += 4;
        c.g[A0][0] = Addr(vt as u32).off(0x30).lh(c);
        c.g[V0][0] = Addr(vt as u32).off(0x34).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0035_2efc);
        c.g[A0][0] = abi::addu(node, c.g[A0][0]);
        if call_ptr(c, target, 0x0035_2efc).is_err() {
            return;
        }
    }
    c.cycles += 3;
    c.g[RA][0] = Addr(sp(c)).ld(c);
    frame.close(c);
}

/// `float ModifierBlock_GetTransformRadius(ModifierBlock* mb)` (0x352bf8):
/// virtual slot 14 of the block's node, 0 when there is none.
pub fn modifier_block_get_transform_radius(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(r32(c, A0));
        if c.g[A1][0] != 0 {
            c.g[V0][0] = c.lw(r32(c, A1));
        } else {
            c.cycles += 3;
            c.mtc1(0, 0);
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.lh(at(c, V0, 0x70));
        c.g[V1][0] = c.lw(at(c, V0, 0x74));
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0035_2c28)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `ModifierBlock_HasRailModifier(block, id)` (0x353418): walk the modifier
/// list at `+0x1c` for a node whose id (`+0x30`) matches.
pub fn modifier_block_has_rail_modifier(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lw(c);
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A1][0] {
            c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x30).lw(c);
                if events::event_due(c) {
                    events::deliver_event_here(c, 0x0035_3428);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void ModifierBlock_ForwardV42(ModifierBlock* block, int arg)`
/// (0x353098): call the wrapped node's `v02(arg)`, if there is a node.
pub fn modifier_block_forward_v42(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(abi::arg(c, 0)).lw(c);
        let node = c.g[A2][0];
        frame.restore_ra(c, 0);
        if node != 0 {
            c.cycles += 6;
            let vt = Addr(node as u32).lw(c);
            c.g[V0][0] = vt;
            let m = VSlot::read(c, vt, 0xb0);
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(node);
            call_ptr(c, m.func as u32, 0x0035_30c0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void ModifierBlock_ResetParticles(ModifierBlock* self)` (0x353228):
/// call the third virtual method of every modifier link.
pub fn modifier_block_reset_particles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = Addr(r32(c, A1)).off(8).lw(c);
            loop {
                c.cycles += 8;
                c.g[S0][0] = Addr(r32(c, A1)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).off(0x18).lh(c);
                c.g[V1][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0035_325c)?;
                c.g[A1][0] = c.g[S0][0];
                if c.g[A1][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V0][0] = Addr(r32(c, A1)).off(8).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_3248);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexBridge_SpringsDtor(FlexBridge* self, int flags)` (0x353db8):
/// free the spring array and, when `flags & 1`, the object.
pub fn flex_bridge_springs_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(p(this, 0x24)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x0035_3de0)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0035_3df4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_00353020` (0x353020): `ModifierBlock_ForwardV43`. Forwards a modifier
/// call to the block's first child (`this->+0` through `vtable[0xc8]`) and then
/// to the block itself (`child->+0xc` through `vtable[0x158]`, with the two
/// saved arguments). Signature: `void forward(this, a1, a2)`.
pub fn modifier_block_forward_v43(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = lw_r(c, A0, 0);
        let none = c.g[A3][0] == 0;
        c.g[S1][0] = c.g[A2][0];
        if none {
            c.cycles += 1;
            c.g[A3][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr(low(c, A3)).lw(c);
            c.g[A0][0] = lh_r(c, V1, 0xc8);
            c.g[V0][0] = lw_r(c, V1, 0xcc);
            let target = low(c, V0);
            c.g[RA][0] = sx(0x0035_3054);
            c.g[A0][0] = sx(low(c, A3).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0035_3054)?;
            c.g[A3][0] = c.g[V0][0];
        }
        c.cycles += 2;
        let skip = c.g[A3][0] == 0;
        c.g[A1][0] = c.g[S0][0];
        if !skip {
            c.cycles += 6;
            c.g[V0][0] = lw_r(c, A3, 0xc);
            c.g[A2][0] = c.g[S1][0];
            c.g[A0][0] = lh_r(c, V0, 0x158);
            c.g[V1][0] = lw_r(c, V0, 0x15c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0035_3080);
            c.g[A0][0] = sx(low(c, A3).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0035_3080)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexBridge_SpringsInit(self, desc)` (0x353e08): copy the five spring
/// parameter floats, scale the sixth, compute the segment length and build the
/// spring nodes.
pub fn flex_bridge_springs_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[V1][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.lwc1(2, gpo(c, -0x2a88));
        c.g[V0][0] = lw_r(c, A1, 0x1c);
        sw_r(c, V1, 0, c.g[V0][0]);
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(1));
        c.lwc1(0, low(c, A1));
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        swc1_r(c, 0, V1, 0x4);
        c.div_s(12, 12, 1);
        c.lwc1(0, low(c, A1).wrapping_add(0x4));
        swc1_r(c, 0, V1, 0x8);
        c.lwc1(1, low(c, A1).wrapping_add(0x8));
        swc1_r(c, 1, V1, 0xc);
        c.lwc1(0, low(c, A1).wrapping_add(0xc));
        swc1_r(c, 0, V1, 0x10);
        c.lwc1(1, low(c, A1).wrapping_add(0x10));
        swc1_r(c, 1, V1, 0x14);
        c.lwc1(0, low(c, A1).wrapping_add(0x14));
        c.mul_s(0, 0, 2);
        swc1_r(c, 0, V1, 0x18);
        c.lwc1(1, low(c, A1).wrapping_add(0x18));
        swc1_r(c, 12, V1, 0x20);
        swc1_r(c, 1, V1, 0x1c);
        callee::FLEX_BRIDGE_ALLOC_SPRING_NODES.call(c, 0x0035_3e74)?;
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void* FlexBridge_SpringsLoad(self, stream)` (0x00353e80): load the spring
/// nodes and tell the owner.
pub fn flex_bridge_springs_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x24;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(at(c, S1, 0)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x10)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0x14)).lw(c);
        c.g[A0][0] = add32(word(c, S1), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_3eb4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::FLEX_BRIDGE_ALLOC_SPRING_NODES_2.call(c, 0x0035_3ebc)?;
        c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
        c.g[A2][0] = 0x50;
        c.g[V1][0] = Addr(at(c, S1, 0)).lw(c);
        c.mult(0, V0, V0, A2);
        c.g[A1][0] = Addr(at(c, S0, 0x24)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x10)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0x14)).lw(c);
        c.g[A2][0] = c.lo[0];
        c.g[A0][0] = add32(word(c, S1), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_3ee4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Modifier blocks.

/// `void ModifierBlock_RemoveDeadParticles(ModifierBlock* self)` (0x353278):
/// walk the block's particle links; a live one (its virtual `state()` is 3)
/// is deleted and unlinked.
pub fn modifier_block_remove_dead_particles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(r32(c, A0), 0x10)).lw(c);
        let null = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !null {
            c.cycles += 2;
            c.g[S2][0] = 0x3;
            c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
            loop {
                c.cycles += 7;
                c.g[S1][0] = Addr(r32(c, S0)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
                let target = r32(c, V0);
                c.g[RA][0] = sx(0x0035_32b4);
                c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                call_ptr(c, target, 0x0035_32b4)?;
                if c.g[V0][0] != c.g[S2][0] {
                    c.g[S0][0] = c.g[S1][0];
                } else {
                    c.cycles += 2;
                    let dead = c.g[S0][0] == 0;
                    c.g[A1][0] = 0x3;
                    if !dead {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                        c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
                        c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
                        let target = r32(c, V1);
                        c.g[RA][0] = sx(0x0035_32d8);
                        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
                        call_ptr(c, target, 0x0035_32d8)?;
                    }
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S1][0];
                }
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_32a0);
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

/// `void SkyBox_SetSection(self, int id)` (0x353cf0): set the section id and,
/// when it is valid, look up the model reference's index.
pub fn sky_box_set_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let valid = (c.g[S0][0] as i64) >= 0;
        Addr(p(r32(c, S1), 0x10)).sw(c, c.g[S0][0]);
        let mut one = false;
        if !valid {
            c.cycles += 4;
            call_ptr(c, 0x0035_3d98, 0x00353d18)?;
        } else {
            one = true;
            c.cycles += 11;
            call_ptr(c, 0x002d_1bd8, 0x00353d28)?;
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = 0x58;
            c.g[V1][0] = Addr(r32(c, A2)).lw(c);
            c.mult(0, 2, 16, 2);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
            c.g[V0][0] = sx(p(r32(c, V0), r32(c, A0)));
            c.g[V1][0] = Addr(p(r32(c, V0), 0x26)).lh(c);
            let none = (c.g[V1][0] as i64) <= 0;
            c.g[A1][0] = c.g[S0][0] & 0xff;
            if !none {
                c.cycles += 6;
                c.g[V1][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[V0][0] = sx(r32(c, A1) << 2);
                c.g[V0][0] = sx(p(r32(c, V0), r32(c, V1)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x1c)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0) >> 8);
                    let nonzero = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if !nonzero {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                }
                c.cycles += 1;
                Addr(p(r32(c, S1), 0x14)).sw(c, c.g[V0][0]);
            }
        }
        if one {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ModifierBlock_HandleEvent (0x0000352f40).

/// `ModifierBlock_HandleEvent`: dispatch an event to the modifier block's handlers.
pub fn modifier_block_handle_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_352f7c: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A2][0] = c.lw((c.g[S0][0] as u32));
            let t = c.g[A2][0] == 0u64;
            c.mov_s(20, 12);
            if t {
                break 'b_352f7c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[RA][0] = sx(0x00352f7c);
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x00352f7c {
                    return Err(crate::call::Unwound);
                }
                break 'b_352f7c;
            }
        }
        'b_352f90: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_352f90;
            } else {
                c.cycles += 2;
                c.mov_s(12, 20);
                call_ptr(c, 0x0035fb30, 0x00352f90)?;
                break 'b_352f90;
            }
        }
        'b_352fa4: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_352fa4;
            } else {
                c.cycles += 2;
                c.mov_s(12, 20);
                call_ptr(c, 0x0035f598, 0x00352fa4)?;
                break 'b_352fa4;
            }
        }
        'b_352fc8: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_352fc8;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.mov_s(12, 20);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[RA][0] = sx(0x00352fc8);
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x00352fc8 {
                    return Err(crate::call::Unwound);
                }
                break 'b_352fc8;
            }
        }
        'b_35300c: {
            c.cycles += 3;
            c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            if c.g[S0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_35300c;
            } else {
                c.cycles += 3;
                c.g[A2][0] = c.g[S0][0];
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                'l_352fe0: loop {
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = c.lw((c.g[A2][0] as u32));
                    c.mov_s(12, 20);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[RA][0] = sx(0x00352ffc);
                    c.pc = t;
                    ssx3_ee_rt::funcs::call_indirect(c, t);
                    if c.pc != 0x00352ffc {
                        return Err(crate::call::Unwound);
                    }
                    c.g[A2][0] = c.g[S0][0];
                    if c.g[A2][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        if event_due(c) {
                            deliver_event_here(c, 0x00352fe0);
                        }
                        continue 'l_352fe0;
                    } else {
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_35300c;
                    }
                }
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::ai::ai::tests::uf;
    pub(crate) use crate::game::bx::bxstring::tests::heap_init;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::bx::memblockman::tests::fake_slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_8 as WORLD;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator;
    pub(crate) use crate::game::movie::movieplayer::tests::ret_slot;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2 as run;
    pub(crate) use crate::game::movie::movieplayer::tests::wh;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::MEM_HEAP;
    pub(crate) use crate::game::movie::movieplayer::tests::RET_ZERO;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::object::animnode::tests::heap_block;
    pub(crate) use crate::game::render::particle::tests::slot;
    pub(crate) use crate::game::render::particle::tests::vt;
    pub(crate) use crate::game::render::particle::tests::SEEDS;
    pub(crate) use crate::game::worldpainter::quadtree::tests::put_vtable;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn world_get_current_view_index_matches() {
        check(
            ssx3_ee::s09::f_002d1c58__generated,
            world_get_current_view_index,
            300,
            &|c, seed| {
                let mut r = Rng(0x1234_5678 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let base = GAME_GP.wrapping_sub(0x848);
                let p1 = DATA + 0x400;
                let p2 = DATA + 0x500;
                let p3 = DATA + 0x600;
                c.bus.write32(base, p1);
                c.bus.write32(p1 + 0x84, p2);
                c.bus.write32(p2 + 0x84, p3);
                c.bus.write32(p3 + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn world_get_camera_eye_matches() {
        check(
            ssx3_ee::s09::f_002d1c20__generated,
            world_get_camera_eye,
            200,
            &|c, seed| {
                let mut r = Rng(0x8888 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x100;
                let viewer = DATA + 0x200;
                let table = DATA + 0x400;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, viewer);
                c.bus.write32(viewer + 0x84, table);
                for k in 0..16u32 {
                    c.bus.write32(table + 4 * k, r.next_u32());
                }
                c.g[A0][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn modifier_block_ctor_matches() {
        check(
            ssx3_ee::s11::f_00352aa8,
            modifier_block_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x101c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..8u32 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn sky_box_ctor_matches() {
        check(ssx3_ee::s11::f_00353ac0, sky_box_ctor, 200, &|c, seed| {
            let mut r = Rng(0x101d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..8u32 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn flex_bridge_springs_ctor_matches() {
        check(
            ssx3_ee::s11::f_00353da8,
            flex_bridge_springs_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x101e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x24, r.next_u32());
            },
        );
    }

    /// A generated function that returns 0 (a `bool`-returning callee).
    pub(crate) const RETURNS_ZERO: u32 = 0x0032_e688;

    #[test]
    fn modifier_block_has_transform_changed_matches() {
        covered(
            ee::s11::f_00352e50__generated,
            modifier_block_has_transform_changed,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x35_2e50, 1);
                let block = DATA;
                let node = DATA + 0x3100;
                slot(c, 0x88, if seed % 3 == 0 { RETURNS_ZERO } else { FAKE });
                c.bus.write32(node, vt());
                if seed % 5 == 0 {
                    c.bus.write32(block, 0);
                } else {
                    c.bus.write32(block, node);
                }
                c.g[A0][0] = sx(block);
            },
        );
    }

    #[test]
    fn modifier_block_is_transform_finished_matches() {
        covered(
            ee::s11::f_00352ed0__generated,
            modifier_block_is_transform_finished,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x35_2ed0, 1);
                let block = DATA;
                let node = DATA + 0x3100;
                slot(c, 0x30, if seed % 4 == 0 { RETURNS_ZERO } else { FAKE });
                c.bus.write32(node, vt());
                if seed % 5 == 0 {
                    c.bus.write32(block, 0);
                } else {
                    c.bus.write32(block, node);
                }
                c.g[A0][0] = sx(block);
            },
        );
    }

    #[test]
    fn modifier_block_get_transform_radius_matches() {
        covered(
            ssx3_ee::s11::f_00352bf8__generated,
            modifier_block_get_transform_radius,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 71, 1);
                let mb = DATA + 0x100;
                if seed % 2 == 0 {
                    c.bus.write32(mb, 0);
                } else {
                    let node = DATA + 0x400;
                    let vt = vtable(c, &mut r, &[0x70]);
                    c.bus.write32(node, vt);
                    c.bus.write32(mb, node);
                }
                c.g[A0][0] = sx(mb);
            },
        );
    }

    #[test]
    fn modifier_block_has_rail_modifier_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x353418, 0);
            c.g[A0][0] = DATA as u64;
            if seed % 5 == 0 {
                c.bus.write32(DATA + 0x1c, 0);
                return;
            }
            let n0 = DATA + 0x100;
            let n1 = DATA + 0x140;
            c.bus.write32(DATA + 0x1c, n0);
            c.bus.write32(n0, n1);
            c.bus.write32(n1, 0);
            c.bus.write32(n0 + 0x30, 0x21);
            c.bus.write32(n1 + 0x30, 0x22);
            c.g[A1][0] = (0x20 + seed % 4) as u64;
        };
        check(
            ee::s11::f_00353418__generated,
            modifier_block_has_rail_modifier,
            SEEDS_2,
            &setup,
        );
    }

    #[test]
    fn modifier_block_forward_v42_matches() {
        check(
            ee::s11::f_00353098__generated,
            modifier_block_forward_v42,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3530_98, 0);
                let block = DATA + 0x100;
                let node = DATA + 0x300;
                let vt = put_vtable(c, &mut r, DATA + 0x800, &[0xb0]);
                c.bus.write32(block, if seed % 4 == 0 { 0 } else { node });
                c.bus.write32(node, vt);
                c.g[A0][0] = sx(block);
                c.g[A1][0] = sx(r.next_u32());
                fill(c, &mut r, node, 8);
            },
        );
    }

    #[test]
    fn modifier_block_reset_particles_matches() {
        check(
            ee::s11::f_00353228__generated,
            modifier_block_reset_particles,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3532_28, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                let nodes = (seed % 3) as u32;
                let mut next = 0u32;
                for i in (0..nodes).rev() {
                    let node = DATA + 0x800 + 0x100 * i;
                    clear(c, node, 0x80);
                    c.bus.write32(node, next);
                    let vt = vtable(c, &mut r, &[0x18]);
                    c.bus.write32(node + 8, vt);
                    next = node;
                }
                c.bus.write32(this + 0x10, next);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn flex_bridge_springs_dtor_matches() {
        check(
            ee::s11::f_00353db8__generated,
            flex_bridge_springs_dtor,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x353d_b8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x60);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this + 0x24, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    #[test]
    fn modifier_block_forward_v43_matches() {
        check(
            ee::s11::f_00353020__generated,
            modifier_block_forward_v43,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x353020, 4);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = (DATA + 0x900) as u64;
                if seed % 2 == 0 {
                    w(c, DATA, 0);
                } else {
                    let child = DATA + 0x400;
                    w(c, DATA, child);
                    fake_slot(c, child, DATA + 0x2000, 0xc8);
                }
            },
        );
    }

    #[test]
    fn flex_bridge_springs_init_matches() {
        check(
            ee::s11::f_00353e08__generated,
            flex_bridge_springs_init,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x353e08, 1);
                be_library(c);
                heap_init(c);
                let desc = DATA + 0x800;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = desc as u64;
                c.f[12] = r.edge_f32();
                for i in 0..7u32 {
                    w(c, desc + 4 * i, r.edge_f32());
                }
                w(c, desc + 0x1c, (2 + seed % 4) as u32);
            },
        );
    }

    #[test]
    fn flex_bridge_springs_load_matches() {
        check(
            ee::s11::f_00353e80__generated,
            flex_bridge_springs_load,
            8,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x353e_80, 6);
                let self_ = DATA;
                let stream = DATA + 0x1000;
                let vtable = DATA + 0x1200;
                w(c, self_, 1 + r.next_u32() % 4);
                w(c, self_ + 0x24, DATA + 0x2000);
                w(c, stream, vtable);
                ret_slot(c, vtable, 0x10, RET_ZERO);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = stream as u64;
            },
        );
    }

    #[test]
    fn modifier_block_remove_dead_particles_matches() {
        check(
            ssx3_ee::s11::f_00353278__generated,
            modifier_block_remove_dead_particles,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x353278, 6);
                be_library(c);
                world(c);
                let blk = DATA + 0x100;
                c.g[A0][0] = blk as u64;
                c.g[A1][0] = r.next_u32() as u64;
                if seed % 4 == 0 {
                    w(c, blk + 0x10, 0);
                } else {
                    let n1 = DATA + 0x300;
                    let n2 = DATA + 0x380;
                    let vt1 = DATA + 0x500;
                    let vt2 = DATA + 0x600;
                    w(c, blk + 0x10, n1);
                    w(c, n1, n2);
                    w(c, n1 + 8, vt1);
                    w(c, n2, 0);
                    w(c, n2 + 8, vt2);
                    slot_3(c, vt1, 0x10, 0, FAKE);
                    slot_3(c, vt1, 0x14, 0, FAKE + 0x10);
                    slot_3(c, vt2, 0x10, 0, FAKE + 0x20);
                    slot_3(c, vt2, 0x14, 0, FAKE + 0x30);
                    // The state getter returns 3 (dead) on some seeds.
                    if seed % 2 == 0 {
                        w(c, 0x0090_0010, 3);
                    }
                }
            },
        );
    }

    #[test]
    fn modifier_block_handle_event_matches() {
        diff(
            ee::s11::f_00352f40__generated,
            modifier_block_handle_event,
            30,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x35_2f40);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.f[12] = uf(&mut r, 10.0);
                w(c, DATA, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
                w(c, DATA + 0x1000, 0); // the handler's vtable is null
                w(c, DATA + 4, if seed % 3 == 0 { 0 } else { DATA + 0x1100 });
                w(c, DATA + 8, if seed % 3 == 0 { DATA + 0x1200 } else { 0 });
                w(c, DATA + 0xc, if seed % 5 == 0 { DATA + 0x1300 } else { 0 });
                w(c, DATA + 0x1100, r.next_u32());
            },
        );
    }
}
