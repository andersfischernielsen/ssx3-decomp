//! `src/util/list.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::sp;
use crate::events::back_edge_unwinds;
pub(crate) use crate::game::ai::ai::virtual_call_slot;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::main::gamemode::sp_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_7788, ui_list_node_accept, "UIListNode_Accept"),
    (0x0039_77b8, ui_list_node_v03, "UIListNode_v03"),
    (0x0039_77e8, ui_list_ctor, "UIList_ctor"),
    (0x0039_7858, uilist_last, "UIList_Last"),
    (0x0039_7870, ui_list_get_at, "UIList_GetAt"),
    (0x0039_78f0, uilist_push_front, "UIList_PushFront"),
    (0x0039_7910, uilist_push_back, "UIList_PushBack"),
    (0x0039_7930, uilist_reset, "UIList_Reset"),
    (0x0039_7948, ui_list_delete_all, "UIList_DeleteAll"),
    (0x0039_79c0, ui_list_count, "UIList_Count"),
    (0x0039_7a68, ui_list_v03, "UIList_v03"),
];

// ---------------------------------------------------------------------------
// Intra-shard callees the generator called by name (`jal`), plus the
// cross-shard ones it wrote as `call_indirect` (called with `call_ptr`).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
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
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
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
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET as UI_LIST_RESET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT;
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
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE as CPS2_MEM_CARD_SET_STATE_2;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR;
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
    pub(crate) use crate::game::main::gamemode::callee::STRCPY;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
}


// ---------------------------------------------------------------------------
// UI list

/// `UIObject *UIList_Last(UIList *list)` (0x397858): the last node, or null
/// when the list is empty (its head points at itself).
pub fn uilist_last(c: &mut Cpu) {
    c.cycles += 5;
    let list = Addr(abi::arg(c, 0));
    c.g[V0][0] = list.off(0xc).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V1][0] = c.g[V1][0] ^ c.g[V0][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `void UIList_PushFront(UIList *list, UIObject *item)` (0x3978f0): insert
/// `item` at the front of the list.
pub fn uilist_push_front(c: &mut Cpu) {
    c.cycles += 7;
    let list = Addr(abi::arg(c, 0));
    let item = abi::arg(c, 1);
    c.g[V0][0] = list.off(0x4).lw(c);
    Addr(item).off(0x4).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    Addr(item).sw(c, c.g[V1][0]);
    Addr(c.g[V0][0] as u32).sw(c, c.g[A1][0]);
    list.off(0x4).sw(c, c.g[A1][0]);
    abi::ret(c);
}


/// `void UIList_PushBack(UIList *list, UIObject *item)` (0x397910): insert
/// `item` at the back of the list.
pub fn uilist_push_back(c: &mut Cpu) {
    c.cycles += 7;
    let list = Addr(abi::arg(c, 0));
    let item = abi::arg(c, 1);
    c.g[V0][0] = list.off(0xc).lw(c);
    Addr(item).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4).lw(c);
    Addr(item).off(0x4).sw(c, c.g[V1][0]);
    Addr(c.g[V0][0] as u32).off(0x4).sw(c, c.g[A1][0]);
    list.off(0xc).sw(c, c.g[A1][0]);
    abi::ret(c);
}


/// `void UIList_Reset(UIList *list)` (0x397930): make the list empty (head and
/// tail point at the embedded sentinel at +0xc).
pub fn uilist_reset(c: &mut Cpu) {
    c.cycles += 6;
    let list = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
    list.sw(c, c.g[A0][0]);
    list.off(0x10).sw(c, c.g[V0][0]);
    list.off(0x4).sw(c, c.g[V0][0]);
    list.off(0xc).sw(c, c.g[A0][0]);
    abi::ret(c);
}


/// `UIListNode::Accept(self, node)` (0x397788): `node->v02(self)`.
pub fn ui_list_node_accept(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0039_77ac, None))
}


/// `UIListNode` destroying/v-table variant (0x3977b8).
pub fn ui_list_node_v03(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0039_77dc, None))
}


/// `int UIList_Count(UIList* list)` (0x3979c0): walk `node->prev` from the
/// head until a node points at itself.
pub fn ui_list_count(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
    c.g[A0][0] = 0;
    if c.g[V0][0] != c.g[V1][0] {
        loop {
            c.cycles += 7;
            c.g[V1][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_79d0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// UI (s13).

/// `UIList* UIList_ctor(UIList* self)` (0x3977e8): empty circular list with
/// its two vtables.
pub fn ui_list_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 0x0049_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(0xc));
        c.g[V1][0] = sx(0x0049_4cc0);
        c.g[V0][0] = sx(0x0049_4c98);
        let s = r32(c, S0);
        Addr(s).off(0x4).sw(c, c.g[S0][0]);
        Addr(s).sw(c, c.g[S0][0]);
        Addr(s).off(0x18).sw(c, c.g[V0][0]);
        Addr(s).off(0x14).sw(c, c.g[V1][0]);
        Addr(s).off(0xc).sw(c, c.g[A1][0]);
        Addr(s).off(0x8).sw(c, c.g[V1][0]);
        c.g[RA][0] = sx(0x0039_782c);
        Addr(s).off(0x10).sw(c, c.g[A1][0]);
        callee::UI_LIST_RESET.call(c, 0x0039_782c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `UIList_v03` (0x397A68): the same backwards walk, using each node's
/// second visitor (`node+0x18` delta, `node+0x1c` function).
pub fn ui_list_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(r32(c, A0).wrapping_add(0x4)).lw(c);
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
        let at_head = c.g[V0][0] == c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        let mut visited = false;
        if !at_head {
            c.cycles += 2;
            c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x8)).lw(c);
            loop {
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                c.g[S0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x18)).lh(c);
                let visitor = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
                c.g[V0][0] = visitor;
                c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
                call_ptr(c, visitor as u32, 0x0039_7aa8)?;
                if c.g[V0][0] != 0 {
                    visited = true;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x4)).lw(c);
                if c.g[V0][0] == c.g[A2][0] {
                    break;
                }
                c.g[V1][0] = Addr(r32(c, A2).wrapping_add(0x8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7a90);
                }
            }
        }
        if !visited {
            c.cycles += 2;
            c.g[V0][0] = 0;
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
// UI lists and rails.

/// `func_00397948` (0x397948): `UIList_DeleteAll`. Walks the list from
/// `this->+4` (the sentinel links back to itself), destroying every node
/// (`node+8` through `vtable[8]`, flag 3), then resets the list
/// (`UIList_Reset`). The loop's back edge carries an event check.
pub fn ui_list_delete_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = lw_r(c, S1, 0x4);
        c.g[V0][0] = lw_r(c, A2, 0x4);
        let single = c.g[V0][0] == c.g[A2][0];
        if !single {
            c.cycles += 1;
            loop {
                c.cycles += 2;
                let sentinel = c.g[A2][0] == 0;
                c.g[S0][0] = lw_r(c, A2, 0x4);
                if !sentinel {
                    c.cycles += 6;
                    c.g[V0][0] = lw_r(c, A2, 0x8);
                    c.g[A1][0] = 0x3;
                    c.g[A0][0] = lh_r(c, V0, 0x8);
                    c.g[V1][0] = lw_r(c, V0, 0xc);
                    let target = low(c, V1);
                    c.g[RA][0] = sx(0x0039_7990);
                    c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x0039_7990)?;
                }
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                c.g[V0][0] = lw_r(c, A2, 0x4);
                let more = c.g[V0][0] != c.g[A2][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7970);
                }
            }
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x0039_79a8);
        c.g[A0][0] = c.g[S1][0];
        callee::UI_LIST_RESET_2.call(c, 0x0039_79a8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* UIList_GetAt(list, index)` (0x397870): the element `index` steps from
/// the head (0 = first), or null past the end/sentinel.
pub fn ui_list_get_at(c: &mut Cpu) {
    guest(c, |c| {
        loop {
            match c.pc {
                0x0039_7870 => {
                    c.cycles += 8;
                    c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
                    c.sq(17, sp_2(c).wrapping_add(0x10));
                    c.sq(16, sp_2(c).wrapping_add(0x20));
                    c.sd(sp_2(c), c.g[RA][0]);
                    c.g[S1][0] = c.g[A1][0];
                    callee::UI_LIST_FIRST.call(c, 0x0039_7888)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.pc = 0x0039_7894;
                }
                0x0039_7890 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx(low(c, S1).wrapping_sub(1));
                    c.pc = 0x0039_7894;
                }
                0x0039_7894 => {
                    c.cycles += 2;
                    c.pc = if (c.g[S1][0] as i64) <= 0 {
                        0x0039_78b4
                    } else {
                        0x0039_789c
                    };
                }
                0x0039_789c => {
                    c.cycles += 2;
                    let empty = c.g[S0][0] == 0;
                    c.g[V0][0] = 0;
                    c.pc = if empty { 0x0039_78d4 } else { 0x0039_78a4 };
                }
                0x0039_78a4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_78ac)?;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = lw_r(c, S0, 0x4);
                        if back_edge_unwinds(c, 0x0039_7890) {
                            return Ok(());
                        }
                        c.pc = 0x0039_7890;
                    } else {
                        c.pc = 0x0039_78b4;
                    }
                }
                0x0039_78b4 => {
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        c.g[V0][0] = 0;
                        c.pc = 0x0039_78d4;
                    } else {
                        c.pc = 0x0039_78bc;
                    }
                }
                0x0039_78bc => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_78c4)?;
                    c.g[V1][0] = c.g[V0][0];
                    let sentinel = c.g[V1][0] == 0;
                    c.g[V0][0] = c.g[S0][0];
                    if sentinel {
                        c.pc = 0x0039_78d4;
                    } else {
                        c.pc = 0x0039_78d0;
                    }
                }
                0x0039_78d0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    c.pc = 0x0039_78d4;
                }
                0x0039_78d4 => {
                    c.cycles += 5;
                    c.lq(16, sp_2(c).wrapping_add(0x20));
                    c.lq(17, sp_2(c).wrapping_add(0x10));
                    c.g[RA][0] = Addr(sp_2(c)).ld(c);
                    let t = low(c, RA);
                    c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, 0x0039_7870, 0x0039_78e8);
                    return Ok(());
                }
            }
        }
    })
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
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    pub(crate) use crate::game::ai::ai::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::ai::ai::tests::be_library;
    pub(crate) use crate::game::ai::ai::tests::virtual_setup;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::ai::rider::tests::hh;
    pub(crate) use crate::game::dirtysock::tags2::tests::bb;
    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;
    pub(crate) use crate::game::dirtysock::tags2::tests::string;
    pub(crate) use crate::game::ui::uiengine::tests::list;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_3;


    #[test]
    fn uilist_last_matches() {
        check(ssx3_ee::s13::f_00397858, uilist_last, 200, &|c, seed| {
            let mut r = Rng(0x3978_5800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let node = DATA + 0x100;
            c.bus.write32(DATA + 0xc, if seed % 2 == 0 { node } else { 0 });
            c.bus.write32(node, if seed % 3 == 0 { node } else { r.next_u32() });
        });
    }


    #[test]
    fn uilist_push_front_matches() {
        check(ssx3_ee::s13::f_003978f0, uilist_push_front, 200, &|c, seed| {
            let mut r = Rng(0x3978_f000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let node = DATA + 0x200;
            c.bus.write32(DATA + 4, node);
            c.bus.write32(node, r.next_u32());
            c.bus.write32(DATA + 0x100, r.next_u32());
            c.bus.write32(DATA + 0x104, r.next_u32());
        });
    }


    #[test]
    fn uilist_push_back_matches() {
        check(ssx3_ee::s13::f_00397910, uilist_push_back, 200, &|c, seed| {
            let mut r = Rng(0x3979_1000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let node = DATA + 0x200;
            c.bus.write32(DATA + 0xc, node);
            c.bus.write32(node + 4, r.next_u32());
            c.bus.write32(DATA + 0x100, r.next_u32());
            c.bus.write32(DATA + 0x104, r.next_u32());
        });
    }


    #[test]
    fn uilist_reset_matches() {
        check(ssx3_ee::s13::f_00397930, uilist_reset, 200, &|c, seed| {
            let mut r = Rng(0x3979_3000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA as u64) | ((r.next_u32() as u64) << 32);
            for k in 0..5 {
                c.bus.write32(DATA + k * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn ui_list_node_accept_matches() {
        check(ee::s13::f_00397788__generated, ui_list_node_accept, SEEDS, &|c, seed| {
            virtual_setup(c, seed, 0x3977_88, 0x10);
        });
    }


    #[test]
    fn ui_list_node_v03_matches() {
        check(ee::s13::f_003977b8__generated, ui_list_node_v03, SEEDS, &|c, seed| {
            virtual_setup(c, seed, 0x3977_b8, 0x10);
        });
    }


    #[test]
    fn ui_list_count_matches() {
        check(ee::s13::f_003979c0__generated, ui_list_count, 80, &|c, seed| {
            let _ = machine(c, seed, 0x3979_c0, 0);
            let list = DATA + 0x100;
            let head = DATA + 0x200;
            c.g[A0][0] = sx(list);
            c.bus.write32(list + 4, head);
            let n = seed % 5;
            for i in 0..5u32 {
                let node = head + 0x20 * i;
                let next = if i + 1 < n { node + 0x20 } else { node };
                c.bus.write32(node + 4, next);
            }
        });
    }


    // ------------------------------------------------------------------
    // UI.

    #[test]
    fn ui_list_ctor_matches() {
        check(ee::s13::f_003977e8__generated, ui_list_ctor, SEEDS_2, &|c, seed| {
            let _ = base(c, seed, 0x3977e8);
            let this = DATA + 0x100;
            clear(c, this, 0x40);
            c.g[A0][0] = sx(this);
        });
    }


    #[test]
    fn ui_list_v03_matches() {
        check(ee::s13::f_00397a68__generated, ui_list_v03, SEEDS_3, &|c, seed| {
            machine(c, seed, 0x39_7a68, 4);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x500) as u64;
            string(c, DATA + 0x500, "");
            list(c, DATA + 0x100, 3, DATA + 4, DATA + 0x600);
            let real = seed % 2 == 0;
            slot(
                c,
                DATA + 0x600,
                0x18,
                if real { 0x20 } else { 0 },
                if real { 0x0041_aa88 } else { FAKE },
            );
            c.pc = 0x0039_7a68;
        });
    }


    #[test]
    fn ui_list_delete_all_matches() {
        check(
            ee::s13::f_00397948__generated,
            ui_list_delete_all,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x397948, 4);
                be_library_2(c);
                world(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                if seed % 3 == 0 {
                    // empty list: the head links back to itself
                    w(c, this + 4, this + 4);
                    w(c, this + 8, this + 4);
                } else {
                    let n = 1 + seed % 3;
                    w(c, this + 4, this + 4);
                    w(c, this + 8, DATA + 0x400);
                    for i in 0..n {
                        let node = DATA + 0x400 + i * 0x100;
                        let next = if i + 1 == n { node } else { node + 0x100 };
                        w(c, node + 4, next);
                        w(c, node + 8, DATA + 0x2000);
                        slot_2(c, DATA + 0x2000, 8, 0, FAKE);
                    }
                }
            },
        );
    }


    #[test]
    fn ui_list_get_at_matches() {
        check(ee::s13::f_00397870__generated, ui_list_get_at, SEEDS_5, &|c, seed| {
            let _r = machine(c, seed, 0x397870, 2);
            be_library_3(c);
            c.pc = 0x0039_7870;
            let list = DATA;
            c.g[A0][0] = list as u64;
            c.g[A1][0] = (seed % 5) as u64;
            let s = DATA + 0x2000;
            if seed % 4 == 0 {
                w(c, list + 4, s);
                w(c, s + 4, s);
                w(c, s, s);
            } else {
                // head -> a -> b -> terminator (self-linked at +4)
                let a = DATA + 0x2100;
                let bb = DATA + 0x2200;
                w(c, list + 4, a);
                w(c, a + 4, bb);
                w(c, a, 0);
                w(c, bb + 4, s);
                w(c, bb, 0);
                w(c, s + 4, s);
                w(c, s, 0);
            }
        });
    }
}
