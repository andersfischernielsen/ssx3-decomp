//! `src/ui/uistate.cpp`: hand-written replacements (moved from the
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
    (
        0x0039_ecb0,
        ui_state_mgr_process_states,
        "UIStateMgr_ProcessStates",
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

pub(crate) mod ui_state {

    pub const NEXT: u32 = 0x4;

    pub const VPTR: u32 = 0x8;

    pub const FLAGS: u32 = 0x1c;

    pub const OWNER: u32 = 0x20;

    /// `flags & ~(0x3f << 8)` (clear the phase).
    pub const PHASE_MASK: u64 = 0xffff_ffff_ffff_c0ff;
}

pub(crate) mod ui_callee {
    use super::Callee;
    use ssx3_ee as ee;

    pub const LIST_FIRST: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::UIList_First,
    };

    pub const NODE_IS_SENTINEL: Callee = Callee {
        addr: 0x0039_7718,
        f: ee::s13::UIListNode_IsSentinel,
    };

    pub const NODE_UNLINK: Callee = Callee {
        addr: 0x0039_7740,
        f: ee::s13::UIListNode_Unlink,
    };

    pub const UPDATE_SCREENS: Callee = Callee {
        addr: 0x0039_e868,
        f: ee::s13::UIState_UpdateScreens,
    };

    pub const SET_PARENT_CHAIN_VISIBLE: Callee = Callee {
        addr: 0x0039_f7b8,
        f: ee::s13::UIStateMgr_SetParentChainVisible,
    };

    pub const HAS_STATE_IN_PHASE: Callee = Callee {
        addr: 0x0039_eb60,
        f: ee::s13::UIStateMgr_HasStateInPhase,
    };

    pub const GET_ACTIVE_STATE: Callee = Callee {
        addr: 0x0039_fa88,
        f: ee::s13::UIStateMgr_GetActiveState,
    };

    pub const RESTART_STATE_IN_PHASE: Callee = Callee {
        addr: 0x0039_ea90,
        f: ee::s13::UIStateMgr_RestartStateInPhase,
    };
}

/// `reg + off` (a field of the object in `reg`).
#[inline(always)]
pub(crate) fn oc224_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// The loop head (resume point after an unwinding event).
pub(crate) const PROCESS_STATES_HEAD: u32 = 0x0039_ed00;

/// `int UIStateMgr_ProcessStates(UIStateMgr* mgr)` (0x39ecb0): steps every
/// active state of the manager's list through its open/close phases
/// (2 opening -> 4 -> 3 -> 5 open; 6 closing -> 7 closed -> 8), calling the
/// state's phase methods, updating its screens, showing/hiding the owner
/// chain and restarting a state when the last one closed. Returns
/// `!opened` if a state finished closing (else 0). Dispatcher-shaped: an
/// unwinding event at the loop back edge returns with `pc` at
/// [`PROCESS_STATES_HEAD`], where the trampoline re-enters it (loop state in
/// s0/s2/s6/s7 and the node's flags in v1).
pub fn ui_state_mgr_process_states(c: &mut Cpu) {
    guest(c, |c| {
        let mut run_body = match c.pc {
            0x0039_ecb0 => {
                c.cycles += 17;
                let frame = Frame::open(c, 0x90);
                frame.save(c, S2, 0x60);
                c.g[S2][0] = c.g[A0][0];
                frame.save(c, S0, 0x80);
                frame.save(c, S6, 0x20);
                frame.save(c, S7, 0x10);
                c.g[S6][0] = 0;
                frame.save(c, S1, 0x70);
                frame.save(c, S3, 0x50);
                frame.save(c, S4, 0x40);
                frame.save(c, S5, 0x30);
                frame.save_ra(c, 0);
                c.g[S7][0] = 0;
                ui_callee::LIST_FIRST.call(c, 0x0039_ece8)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    return process_states_finish(c);
                }
                c.cycles += 2;
                false
            }
            PROCESS_STATES_HEAD => true,
            pc => {
                c.jump_out(pc, 0x0039_ecb0, 0x0039_f0fc);
                return Ok(());
            }
        };
        loop {
            if run_body {
                process_states_step(c)?;
            }
            run_body = true;
            // 0x39f0b4: next node, until the sentinel.
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            ui_callee::NODE_IS_SENTINEL.call(c, 0x0039_f0bc)?;
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
            if crate::events::back_edge_unwinds(c, PROCESS_STATES_HEAD) {
                return Err(crate::call::Unwound);
            }
        }
        process_states_finish(c)
    })
}

/// 0x39f0c4: v0 = closed ? !opened : 0, restore, return.
pub(crate) fn process_states_finish(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 2;
    let closed = c.g[S6][0] != 0;
    c.g[V0][0] = 0;
    if closed {
        c.cycles += 1;
        c.g[V0][0] = c.g[S7][0] ^ 1;
    }
    c.cycles += 11;
    let frame = Frame { size: 0x90 };
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, S6, 0x20);
    frame.restore(c, S7, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// Virtual call `vt[slot]` on the state in `obj_reg` (vptr at +8):
/// `a0 = obj + delta` (the other argument registers are already set).
#[inline(always)]
pub(crate) fn ui_vcall(
    c: &mut Cpu,
    obj_reg: usize,
    vt_reg: usize,
    fn_reg: usize,
    slot: u32,
    ret: u32,
) -> crate::call::Flow {
    c.g[vt_reg][0] = c.lw(oc224_at(c, obj_reg, ui_state::VPTR));
    c.g[A0][0] = c.lh(oc224_at(c, vt_reg, slot));
    c.g[fn_reg][0] = c.lw(oc224_at(c, vt_reg, slot + 4));
    let t = r32(c, fn_reg);
    c.g[A0][0] = sx((c.g[obj_reg][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// One node (0x39ed00, s0 = node, v1 = its flags); leaves s0 = next.
pub(crate) fn process_states_step(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 5;
    c.g[S5][0] = c.lw(oc224_at(c, S0, ui_state::NEXT));
    c.g[V0][0] = sx((c.g[V1][0] as u32 as i32 >> 5) as u32) & 1;
    c.g[A1][0] = c.g[V1][0];
    if c.g[V0][0] == 0 {
        return step_next(c);
    }
    // The phase, 2..7 through the jump table at 0x493fc0.
    c.cycles += 8;
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8) & 0x3f;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(2));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
    c.g[A0][0] = sx((c.g[V0][0] as u32 as i32 >> 24) as u32);
    c.g[V1][0] = (c.g[A0][0] < 6) as u64;
    c.g[V0][0] = 0x0049_0000;
    if c.g[V1][0] == 0 {
        return step_next(c);
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3fc0));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw(r32(c, V1));
    match r32(c, A0) {
        0x0039_ed4c => phase_opening(c),
        0x0039_ee14 => phase_open_input(c),
        0x0039_edc8 => {
            c.cycles += 1;
            c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::VPTR));
            phase_open_wait(c)
        }
        0x0039_ee78 => {
            // 5: open: vt[0x60](), update screens.
            c.cycles += 9;
            ui_vcall(c, S0, V0, V1, 0x60, 0x0039_ee8c)?;
            c.g[A0][0] = c.g[S0][0];
            ui_callee::UPDATE_SCREENS.call(c, 0x0039_ee94)?;
            c.g[S0][0] = c.g[S5][0];
            Ok(())
        }
        0x0039_ee9c => {
            // 6: closing: vt[0x50]() true -> phase 7; mark shown.
            c.cycles += 7;
            ui_vcall(c, S0, V1, V0, 0x50, 0x0039_eeb0)?;
            let done = c.g[V0][0] != 0;
            c.g[V1][0] = ui_state::PHASE_MASK;
            if done {
                c.cycles += 4;
                c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
                c.g[V0][0] &= c.g[V1][0];
                c.g[V0][0] |= 0x700;
                let v = c.g[V0][0];
                c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] |= 0x10;
            store_flags_update(c)
        }
        0x0039_eee4 => phase_closed(c),
        pc => {
            c.jump_out(pc, 0x0039_ecb0, 0x0039_f0fc);
            Err(crate::call::Unwound)
        }
    }
}

/// 0x39f0b0: s0 = next.
pub(crate) fn step_next(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 1;
    c.g[S0][0] = c.g[S5][0];
    Ok(())
}

/// 0x39eed4: store v0 as the flags, update the screens, s0 = next.
pub(crate) fn store_flags_update(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 4;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
    ui_callee::UPDATE_SCREENS.call(c, 0x0039_eedc)?;
    c.g[S0][0] = c.g[S5][0];
    Ok(())
}

/// Phase 2 (0x39ed4c): opening: vt[0x20](), phase 4, update screens; a
/// top-level state shows its owner chain and notifies the owner
/// (vt[0x88](state, 0x15)); then as phase 4.
pub(crate) fn phase_opening(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 17;
    c.g[S7][0] = 1;
    ui_vcall(c, S0, V0, V1, 0x20, 0x0039_ed64)?;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[V1][0] = ui_state::PHASE_MASK;
    c.g[A0][0] = c.g[S0][0];
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= 0x400;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
    ui_callee::UPDATE_SCREENS.call(c, 0x0039_ed80)?;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS)) & 1;
    if c.g[V0][0] == 0 {
        c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::VPTR));
        return phase_open_wait(c);
    }
    c.cycles += 3;
    c.g[S1][0] = c.lw(oc224_at(c, S0, ui_state::OWNER));
    c.g[A0][0] = c.g[S2][0];
    if c.g[S1][0] != 0 {
        c.cycles += 11;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 1;
        c.g[A3][0] = 1;
        ui_callee::SET_PARENT_CHAIN_VISIBLE.call(c, 0x0039_edac)?;
        c.g[V0][0] = c.lw(oc224_at(c, S1, ui_state::VPTR));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0x15;
        c.g[A0][0] = c.lh(oc224_at(c, V0, 0x88));
        c.g[V1][0] = c.lw(oc224_at(c, V0, 0x8c));
        let t = r32(c, V1);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0039_edc8)?;
    }
    c.cycles += 1;
    c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::VPTR));
    phase_open_wait(c)
}

/// Phase 4 (0x39edcc, v1 = vptr): vt[0x38](input) true -> phase 3; clear
/// the input bit; update screens.
pub(crate) fn phase_open_wait(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 9;
    c.g[A1][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[A0][0] = c.lh(oc224_at(c, V1, 0x38));
    c.g[A1][0] = sx((c.g[A1][0] as u32 as i32 >> 4) as u32);
    c.g[V0][0] = c.lw(oc224_at(c, V1, 0x3c));
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let t = r32(c, V0);
    c.g[A1][0] &= 1;
    call_ptr(c, t, 0x0039_ede8)?;
    let done = c.g[V0][0] != 0;
    c.g[V1][0] = ui_state::PHASE_MASK;
    if done {
        c.cycles += 4;
        c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= 0x300;
        let v = c.g[V0][0];
        c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
    }
    c.cycles += 5;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[V1][0] = 0xffff_ffff_ffff_ffef;
    c.g[A0][0] = c.g[S0][0];
    c.g[V0][0] &= c.g[V1][0];
    store_flags_update(c)
}

/// Phase 3 (0x39ee14, a1 = flags): with input, vt[0x38](1) and clear the
/// input bit; vt[0x30](0); phase 5; update screens.
pub(crate) fn phase_open_input(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A1][0] as u32 as i32 >> 4) as u32) & 1;
    c.g[A1][0] = 1;
    if c.g[V0][0] != 0 {
        c.cycles += 9;
        ui_vcall(c, S0, V1, V0, 0x38, 0x0039_ee38)?;
        c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
        c.g[V0][0] = 0xffff_ffff_ffff_ffef;
        c.g[V1][0] &= c.g[V0][0];
        let v = c.g[V1][0];
        c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
    }
    c.cycles += 12;
    c.g[A1][0] = 0;
    ui_vcall(c, S0, V0, V1, 0x30, 0x0039_ee60)?;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[V1][0] = ui_state::PHASE_MASK;
    c.g[A0][0] = c.g[S0][0];
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= 0x500;
    store_flags_update(c)
}

/// Phase 7 (0x39eee4): closed: vt[0x58](0), vt[0x28](); phase 8, inactive;
/// unlink if remove-on-close; a top-level state hides/re-shows its owner
/// chain (unless another state is still opening) and notifies the owner
/// (vt[0x88](state, 0x16)); a removed state gets vt[0x8](3) (delete), else
/// the shown bit is cleared; when nothing is active any more, restart a
/// state in phase 1 (else 8).
pub(crate) fn phase_closed(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 23;
    c.g[A1][0] = 0;
    c.g[S4][0] = 0;
    c.g[S6][0] = 1;
    ui_vcall(c, S0, V0, V1, 0x58, 0x0039_ef04)?;
    ui_vcall(c, S0, V0, V1, 0x28, 0x0039_ef18)?;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[V1][0] = ui_state::PHASE_MASK;
    c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
    c.g[V0][0] &= c.g[V1][0];
    c.g[V0][0] |= 0x800;
    c.g[V0][0] &= c.g[A0][0];
    c.g[V1][0] = sx((c.g[V0][0] as u32 as i32 >> 7) as u32) & 1;
    let remove = c.g[V1][0] != 0;
    let v = c.g[V0][0];
    c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
    if remove {
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        ui_callee::NODE_UNLINK.call(c, 0x0039_ef48)?;
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
    c.g[S3][0] = c.g[V0][0] & 1;
    c.g[V0][0] = sx((c.g[V0][0] as u32 as i32 >> 14) as u32);
    if c.g[S3][0] != 0 {
        c.cycles += 4;
        c.g[S1][0] = c.lw(oc224_at(c, S0, ui_state::OWNER));
        c.g[V0][0] &= 1;
        c.g[S6][0] = 0;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 1;
            ui_callee::HAS_STATE_IN_PHASE.call(c, 0x0039_ef74)?;
            c.g[S4][0] = c.g[V0][0];
        } else {
            c.cycles += 1;
            c.g[S4][0] = 0;
        }
        c.cycles += 2;
        if c.g[S4][0] == 0 {
            // No state opening: mark hidden-owner, step the owner's count.
            c.cycles += 4;
            c.g[V0][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS)) | 0x80;
            let v = c.g[V0][0];
            c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
            if c.g[S1][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.lw(oc224_at(c, S1, ui_state::FLAGS));
                c.g[V0][0] = c.g[A0][0] & 6;
                let counted = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[A0][0] as u32) >> 1);
                if counted {
                    c.cycles += 8;
                    c.g[V1][0] = 0xffff_ffff_ffff_fff9;
                    c.g[V0][0] &= 3;
                    c.g[V1][0] = c.g[A0][0] & c.g[V1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[V0][0] &= 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V1][0] |= c.g[V0][0];
                    let v = c.g[V1][0];
                    c.sw(oc224_at(c, S1, ui_state::FLAGS), v);
                }
                c.cycles += 17;
                c.g[V1][0] = c.lw(oc224_at(c, S1, ui_state::FLAGS));
                c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
                c.g[A3][0] = c.lw(oc224_at(c, S1, ui_state::VPTR));
                c.g[A1][0] = c.g[S0][0];
                c.g[V0][0] = c.g[V1][0] & 6;
                c.g[A2][0] = 0x16;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
                c.g[V1][0] &= c.g[A0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V1][0] |= c.g[V0][0];
                let v = c.g[V1][0];
                c.sw(oc224_at(c, S1, ui_state::FLAGS), v);
                c.g[A0][0] = c.lh(oc224_at(c, A3, 0x88));
                c.g[V0][0] = c.lw(oc224_at(c, A3, 0x8c));
                let t = r32(c, V0);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0039_f004)?;
            }
        } else {
            // 0x39f00c: another state opening: re-show the owner chain.
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            if c.g[S1][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = 0;
                c.g[A3][0] = 1;
                ui_callee::SET_PARENT_CHAIN_VISIBLE.call(c, 0x0039_f024)?;
            }
        }
    }
    // 0x39f024
    c.cycles += 2;
    if c.g[S0][0] != 0 {
        c.cycles += 5;
        c.g[V1][0] = c.lw(oc224_at(c, S0, ui_state::FLAGS));
        c.g[V0][0] = sx((c.g[V1][0] as u32 as i32 >> 7) as u32) & 1;
        let remove = c.g[V0][0] != 0;
        c.g[V0][0] = 0xffff_ffff_ffff_fff7;
        if remove {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            ui_callee::NODE_UNLINK.call(c, 0x0039_f048)?;
            c.g[A1][0] = 3;
            ui_vcall(c, S0, V0, V1, 0x8, 0x0039_f060)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
            let v = c.g[V0][0];
            c.sw(oc224_at(c, S0, ui_state::FLAGS), v);
        }
    }
    // 0x39f070
    c.cycles += 2;
    if c.g[S3][0] != 0 {
        c.cycles += 2;
        c.g[S0][0] = c.g[S5][0];
        if c.g[S4][0] == 0 {
            return Ok(());
        }
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    ui_callee::GET_ACTIVE_STATE.call(c, 0x0039_f088)?;
    c.g[S0][0] = c.g[S5][0];
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 1;
    ui_callee::RESTART_STATE_IN_PHASE.call(c, 0x0039_f09c)?;
    c.g[S0][0] = c.g[S5][0];
    if c.g[V0][0] != 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 8;
    ui_callee::RESTART_STATE_IN_PHASE.call(c, 0x0039_f0b0)?;
    step_next(c)
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
    use super::*;
    use crate::difftest::fakes::covered;

    // ----------------------------------------------- UIStateMgr_ProcessStates

    /// The manager (state list at +0).
    pub(crate) const UI_MGR: u32 = DATA;

    /// A shared vtable: every slot 0..0x100 a distinct fake.
    pub(crate) const UI_VT: u32 = DATA + 0xf000;

    /// Owner states (not in the list) at DATA+0x1000.
    pub(crate) const UI_OWNERS: u32 = DATA + 0x1000;

    /// Link `states` into the list at `list` (anchor link at +4), ending at
    /// the self-linked `sentinel`. Sometimes an empty list is a null link.
    pub(crate) fn ui_list(c: &mut Cpu, r: &mut Rng, list: u32, states: &[u32], sentinel: u32) {
        if states.is_empty() && r.next_u32() % 4 == 0 {
            c.bus.write32(list + 4, 0);
            return;
        }
        let mut prev = list;
        for &s in states {
            c.bus.write32(prev + 4, s);
            c.bus.write32(s, prev);
            prev = s;
        }
        c.bus.write32(prev + 4, sentinel);
        c.bus.write32(sentinel, prev);
        c.bus.write32(sentinel + 4, sentinel);
    }

    /// A random state at `s`: vptr = [`UI_VT`], flags with a phase in 0..8
    /// and random bits, no screens, owner `owner`.
    pub(crate) fn ui_state(c: &mut Cpu, r: &mut Rng, s: u32, owner: u32) {
        c.bus.write32(s + 8, UI_VT);
        c.bus.write32(s + 0xc, r.next_u32() % 6);
        let phase = 2 + r.next_u32() % 7;
        c.bus
            .write32(s + 0x1c, (r.next_u32() & !0x3f00) | phase << 8);
        c.bus.write32(s + 0x20, owner);
        // An empty screen list (+0x24) for UIState_UpdateScreens.
        ui_list(c, r, s + 0x24, &[], s + 0x30);
    }

    /// 0..6 states in the manager's list (0x40 apart from DATA+0x100),
    /// half of them owned by a state outside the list, the shared vtable,
    /// and the phase jump table (.rodata 0x493fc0).
    pub(crate) fn process_states_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = machine(c, seed, 0x0039_ecb0, calls);
        for off in (0..0x100).step_by(8) {
            c.bus.write16(UI_VT + off, ((r.next_u32() % 16) * 4) as u16);
            c.bus.write32(UI_VT + off + 4, FAKE + 2 * off);
        }
        crate::difftest::put_words(
            c,
            0x0049_3fc0,
            &[
                0x0039_ed4c,
                0x0039_ee14,
                0x0039_edc8,
                0x0039_ee78,
                0x0039_ee9c,
                0x0039_eee4,
            ],
        );
        let n = r.next_u32() % 7;
        let states: Vec<u32> = (0..n).map(|i| DATA + 0x100 + 0x40 * i).collect();
        for (i, &s) in states.iter().enumerate() {
            let owner = if r.next_u32() % 2 == 0 {
                0
            } else {
                let o = UI_OWNERS + 0x40 * i as u32;
                ui_state(c, &mut r, o, 0);
                o
            };
            ui_state(c, &mut r, s, owner);
        }
        ui_list(c, &mut r, UI_MGR, &states, DATA + 0x6c0);
        // The manager's second list (+0x1c) is empty.
        ui_list(c, &mut r, UI_MGR + 0x1c, &[], DATA + 0xec0);
        c.g[A0][0] = UI_MGR as u64;
        c.pc = 0x0039_ecb0;
        r
    }

    #[test]
    fn ui_state_mgr_process_states_matches() {
        use crate::difftest::events::{check_events, schedule};
        let (generated, hand): (GuestFn, GuestFn) =
            (ee::s13::f_0039ecb0__generated, ui_state_mgr_process_states);
        covered(generated, hand, 300, &|c, seed| {
            process_states_setup(c, seed, 8);
        });
        check_events(generated, hand, 300, &|c, seed| {
            let mut r = process_states_setup(c, seed, 0);
            schedule(c, &mut r, seed, 0x0039_ecb0)
        });
    }
}
