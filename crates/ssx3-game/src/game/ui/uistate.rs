//! `src/ui/uistate.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::ovstates::at as at_3;
pub(crate) use crate::game::fe::ovstates::close;
pub(crate) use crate::game::fe::ovstates::open;
pub(crate) use crate::game::scripter::ssxscriptengine::p;
use crate::guest::Addr;
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

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_e948, ui_state_show_object, "UIState_ShowObject"),
    (0x0039_e9d8, ui_state_mgr_dtor, "UIStateMgr_dtor"),
    (
        0x0039_f190,
        ui_state_mgr_close_top_states,
        "UIStateMgr_CloseTopStates",
    ),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    use ssx3_ee as ee;
}

/// `void* UIStateMgr_CloseTopStates(UIList* list, int count)` (0x39f190):
/// close (mark hidden) the first `count` UI states.
pub fn ui_state_mgr_close_top_states(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0039_7840, 0x0039_f1a4)?;
        c.g[A0][0] = c.g[V0][0];
        let none = (c.g[S0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        // The empty-list exit skips the one-cycle charge of the count block.
        let mut empty_exit = false;
        if !none {
            c.cycles += 2;
            c.g[A3][0] = 0xffff_ffff_ffff_c0ff;
            c.g[A2][0] = 0xffff_ffff_ffff_ff7f;
            loop {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    empty_exit = true;
                    break;
                }
                c.cycles += 10;
                c.g[V0][0] = Addr(at(c, A0, 0x1c)).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[S0][0] as i64)) as u64;
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | 0x600;
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | 0x80;
                Addr(at(c, A0, 0x1c)).sw(c, c.g[V0][0]);
                let more = c.g[V1][0] != 0;
                c.g[A0][0] = Addr(at(c, A0, 4)).lw(c);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_f1b8);
                }
            }
        }
        frame.restore(c, S0, 0x10);
        if !empty_exit {
            c.cycles += 1;
        }
        c.cycles += 4;
        c.g[V0][0] = c.g[A0][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI states.

/// `void cUIState_hideObjSafe(cUIState* self, char* name)` (0x39e8b8): hide
/// the first screen object whose hash name matches.
pub fn ui_state_hide_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = sx(p(r32(c, A0), 0x24));
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::UI_LIST_FIRST.call(c, 0x0039_e8d4)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0031_7670, 0x0039_e8e8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                callee::UI_SCREEN_FIND_OBJECT.call(c, 0x0039_e8f4)?;
                c.g[A2][0] = c.g[V0][0];
                let missing = c.g[A2][0] == 0;
                c.g[A1][0] = 0;
                if !missing {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                    c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                    let target = r32(c, V1);
                    c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                    call_ptr(c, target, 0x0039_e914)?;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                c.cycles += 5;
                c.g[S0][0] = Addr(p(r32(c, S0), 4)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                callee::UI_LIST_NODE_IS_SENTINEL.call(c, 0x0039_e928)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_e8e0);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE state transition and messenger.

/// `void cUIState_showObjSafe(cUIState* self, char* name)` (0x39e948): walk the
/// object list at_2 `self + 0x24`, and for the first node whose object hash
/// name matches, call its virtual `show(1)` (slot 0x4c of the vtable at_2
/// `object + 8`).
pub fn ui_state_show_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        let name = abi::arg(c, 1);
        c.g[A0][0] = sx(p(abi::arg(c, 0), 0x24));
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(name);
        call_ptr(c, callee::UI_LIST_FIRST_2, 0x0039_e964)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, callee::GET_HASH_VALUE32, 0x0039_e978)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::UI_SCREEN_FIND_OBJECT_2, 0x0039_e984)?;
                c.g[A2][0] = c.g[V0][0];
                let found = c.g[A2][0] != 0;
                c.g[A1][0] = 1;
                if found {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(at_2(c, A2, 8));
                    c.g[A0][0] = c.lh(at_2(c, V0, 0x48));
                    c.g[V1][0] = c.lw(at_2(c, V0, 0x4c));
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0039_e9a4)?;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                c.cycles += 5;
                c.g[S0][0] = c.lw(at_2(c, S0, 4));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::UI_LIST_NODE_IS_SENTINEL_2, 0x0039_e9b8)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_e970);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI state destructor.

/// `void UIStateMgr_dtor(UIStateMgr* self, int flags)` (0x39e9d8): destroy the
/// state stack, the three lists and the four list nodes, retagging each
/// vtable, then delete `self` when `flags & 1`.
pub fn ui_state_mgr_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        open(c, 0x60);
        c.sq(20, at_3(c.g[SP][0] as u32, 0x10));
        let this = c.g[A0][0];
        c.g[S4][0] = this;
        c.sq(18, at_3(c.g[SP][0] as u32, 0x30));
        c.sq(16, at_3(c.g[SP][0] as u32, 0x50));
        c.g[S2][0] = c.g[A1][0];
        c.sq(17, at_3(c.g[SP][0] as u32, 0x40));
        c.sq(19, at_3(c.g[SP][0] as u32, 0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0039_ea04);
        c.g[S3][0] = sx((this as u32).wrapping_add(0x1c));
        call_ptr(c, 0x0039_f8c8, 0x0039_ea04)?;
        c.g[S1][0] = 0x490000;
        c.g[A0][0] = c.g[S3][0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c98));
        c.g[RA][0] = sx(0x0039_ea18);
        Addr(this as u32).off(0x34).sw(c, c.g[S1][0]);
        call_ptr(c, 0x0039_7948, 0x0039_ea18)?;
        c.g[S0][0] = 0x490000;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x28));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4cc0));
        c.g[RA][0] = sx(0x0039_ea2c);
        Addr(this as u32).off(0x30).sw(c, c.g[S0][0]);
        call_ptr(c, 0x003a_4868, 0x0039_ea2c)?;
        Addr(this as u32).off(0x24).sw(c, c.g[S0][0]);
        c.g[RA][0] = sx(0x0039_ea38);
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x003a_4868, 0x0039_ea38)?;
        Addr(this as u32).off(0x18).sw(c, c.g[S1][0]);
        c.g[RA][0] = sx(0x0039_ea44);
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x0039_7948, 0x0039_ea44)?;
        Addr(this as u32).off(0x14).sw(c, c.g[S0][0]);
        c.g[RA][0] = sx(0x0039_ea50);
        c.g[A0][0] = sx((this as u32).wrapping_add(0xc));
        call_ptr(c, 0x003a_4868, 0x0039_ea50)?;
        Addr(this as u32).off(8).sw(c, c.g[S0][0]);
        c.g[RA][0] = sx(0x0039_ea5c);
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x003a_4868, 0x0039_ea5c)?;
        c.g[S2][0] = c.g[S2][0] & 1;
        let keep = c.g[S2][0] == 0;
        c.lq(16, at_3(c.g[SP][0] as u32, 0x50));
        if !keep {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0039_ea70);
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0031_7e50, 0x0039_ea70)?;
            c.lq(16, at_3(c.g[SP][0] as u32, 0x50));
        }
        c.cycles += 7;
        c.lq(17, at_3(c.g[SP][0] as u32, 0x40));
        c.lq(18, at_3(c.g[SP][0] as u32, 0x30));
        c.lq(19, at_3(c.g[SP][0] as u32, 0x20));
        c.lq(20, at_3(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        close(c, 0x60);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::dirtysock::tags2::tests::cstr;
    pub(crate) use crate::game::dirtysock::tags2::tests::w8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w16;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn ui_state_mgr_close_top_states_matches() {
        check(
            ee::s13::f_0039f190__generated,
            ui_state_mgr_close_top_states,
            60,
            &|c, seed| {
                base(c, seed, 0x39f1_90);
                let sentinel = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(DATA + 4, sentinel);
                if seed % 3 == 0 {
                    c.bus.write32(sentinel, sentinel);
                    c.bus.write32(sentinel + 4, sentinel);
                } else {
                    let n1 = DATA + 0x1100;
                    let n2 = DATA + 0x1200;
                    c.bus.write32(sentinel, sentinel);
                    c.bus.write32(sentinel + 4, n1);
                    c.bus.write32(n1, n1);
                    c.bus.write32(n1 + 4, n2);
                    c.bus.write32(n1 + 0x1c, 0x1234_5678);
                    c.bus.write32(n2, n2);
                    c.bus.write32(n2 + 4, sentinel);
                    c.bus.write32(n2 + 0x1c, 0x9abc_def0);
                }
            },
        );
    }

    /// `RA` the setup's helper calls return to.
    pub(crate) const RET_2: u32 = 0x0012_3450;

    /// `GetHashValue32(s)`, as the game computes it (the same machine state
    /// in both runs, so it can be called from the setup).
    pub(crate) fn hash(c: &mut Cpu, s: u32) -> u32 {
        c.g[RA][0] = RET_2 as u64;
        c.g[A0][0] = sx(s);
        c.pc = 0x0031_7670;
        ssx3_ee_rt::funcs::call_indirect(c, 0x0031_7670);
        assert_eq!(c.pc, RET_2, "GetHashValue32 did not return");
        c.g[V0][0] as u32
    }

    // ---------------------------------------------------------------- FE/UI

    #[test]
    fn ui_state_show_object_matches() {
        check(
            ee::s13::f_0039e948__generated,
            ui_state_show_object,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x39e9, 0);
                env(c);
                let this = DATA + 0x400;
                let name = DATA + 0x800;
                cstr(c, name, "OBJ_HELLO");
                let list = this + 0x24;
                let sentinel = DATA + 0x1000;
                let node = DATA + 0x1100;
                w32(c, sentinel + 4, sentinel);
                match seed % 3 {
                    0 => {
                        w32(c, list + 4, sentinel);
                    }
                    1 => {
                        let obj = DATA + 0x1200;
                        let vt = DATA + 0x1300;
                        w32(c, node, sentinel);
                        w32(c, node + 4, sentinel);
                        w32(c, list + 4, node);
                        w32(c, node + 0x38, DATA + 0x1400);
                        w32(c, node + 0x3c, 1);
                        w32(c, DATA + 0x1400, obj);
                        let hv = hash(c, name);
                        w32(c, obj + 0x38, hv);
                        w32(c, obj + 8, vt);
                        slot(c, vt, 0x48, EMPTY);
                    }
                    _ => {
                        w32(c, node, sentinel);
                        w32(c, node + 4, sentinel);
                        w32(c, list + 4, node);
                        w32(c, node + 0x3c, 0);
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
            },
        );
    }

    // ------------------------------------------------------------- UI state.

    #[test]
    fn ui_state_mgr_dtor_matches() {
        check(
            ee::s13::f_0039e9d8__generated,
            ui_state_mgr_dtor,
            100,
            &|c, seed| {
                machine(c, seed, 0x39_e9d8, 0);
                c.g[A0][0] = sx(DATA);
                // Even flags: the page base must not free the test object. The
                // lists stay empty so every list destructor takes its short path.
                c.g[A1][0] = [0u64, 2][seed as usize % 2];
            },
        );
    }
}
