//! `src/scripter/scanimmgr.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::be::be::gp;
pub(crate) use crate::game::be::be::vt_call;
pub(crate) use crate::game::dirtysock::tags2::at;
pub(crate) use crate::game::dirtysock::tags2::p_2 as p;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_3;
pub(crate) use crate::game::sound::soundsys::p as p_4;
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
    (
        0x0027_5498,
        script_anim_bank_man_dtor,
        "ScriptAnimBankMan_dtor",
    ),
    (
        0x0027_5650,
        script_anim_bank_man_remove_bank,
        "ScriptAnimBankMan_RemoveBank",
    ),
    (
        0x0027_5698,
        script_anim_bank_man_remove_all,
        "ScriptAnimBankMan_RemoveAll",
    ),
    (
        0x0027_5718,
        script_anim_bank_man_find_bank,
        "ScriptAnimBankMan_FindBank",
    ),
    (
        0x0027_5760,
        script_anim_bank_man_free_entry,
        "ScriptAnimBankMan_FreeEntry",
    ),
    (
        0x0027_57f0,
        script_anim_bank_man_find_free_entry,
        "ScriptAnimBankMan_FindFreeEntry",
    ),
];

/// Direct and cross-shard callees, by guest address.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
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
}

// ---------------------------------------------------------------------------
// Script tables.

/// `void* ScriptAnimBankMan_FindFreeEntry(ScriptAnimBank* self)` (0x2757f0):
/// first entry (stride 0x14) whose data pointer is null, or 0.
pub fn script_anim_bank_man_find_free_entry(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    let empty = (c.g[A1][0] as i64) <= 0;
    c.g[V1][0] = 0;
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[A0][0] = Addr(abi::arg(c, 0)).off(8).lw(c);
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
        let occupied = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if !occupied {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0027_5800);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script animation banks.

/// `void ScriptAnimBankMan_RemoveBank(ScriptAnimBankMan* self, int id)`
/// (0x275650): when the bank exists, remove it through the manager's list.
pub fn script_anim_bank_man_remove_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK.call(c, 0x0027_5664)?;
        c.g[A1][0] = c.g[V0][0];
        if c.g[A1][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.lw(at(c, S0, 0x0c));
            c.g[A0][0] = c.lh(p(r32(c, V0), 0x10));
            c.g[V1][0] = c.lw(p(r32(c, V0), 0x14));
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_5684)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptAnimBank* ScriptAnimBankMan_FindBank(ScriptAnimBankMan* self, int
/// id /*a1*/)` (0x275718): walk the bank list (0x14 bytes each, head at +8),
/// returning the first live entry with `id`.
pub fn script_anim_bank_man_find_bank(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = c.lw(at(c, A0, 0x04));
    let none = (c.g[A2][0] as i64) <= 0;
    c.g[V1][0] = 0;
    if !none {
        c.cycles += 1;
        c.g[A0][0] = c.lw(at(c, A0, 0x08));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, A0, 0x08));
            let dead = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if dead {
                c.cycles += 1;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, A0));
                let mismatch = c.g[V0][0] != c.g[A1][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if !mismatch {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A0][0];
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_5728);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `ScriptAnimBankMan_dtor(ScriptAnimBankMan* self, int flags)` (0x275498):
/// install the vtable, remove all banks, free the name block and delete the
/// object when the low flag bit is set.
pub fn script_anim_bank_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = 0x0048_25d8;
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        sw(c, p_2(this as u32, 0xc), c.g[V0][0]);
        call_ptr(c, 0x0027_5698, 0x0027_54c0)?;
        c.g[A0][0] = lw(c, p_2(this as u32, 8));
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !none {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0027_54d4)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_7e50, 0x0027_54e8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptAnimBankMan_RemoveAll(ScriptAnimBankMan* m)` (0x00275698):
/// same shape as [`script_sound_bank_man_remove_all`] with stride 0x14 and
/// the destructor slot at `+0x14`.
pub fn script_anim_bank_man_remove_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(8));
                if c.g[V1][0] != 0 {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    let vt = c.g[V0][0];
                    let obj = c.g[S0][0];
                    vt_call(c, vt, 0x10, obj, V1, 0x0027_56e8)?;
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_56c0);
                }
            }
        }
        c.cycles += 6;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script animation banks.

/// `void ScriptAnimBankMan_FreeEntry(ScriptAnimBankMan* self, Entry* e)`
/// (0x275760): free the bank entry's animation and data.
pub fn script_anim_bank_man_free_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = Addr(gp(c, 0xd08)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[V1][0];
        c.g[V0][0] = Addr(p_3(r32(c, S0), 4)).lbu(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[S1][0] = Addr(r32(c, V1)).lw(c);
        Addr(r32(c, V1)).sw(c, 0);
        call_ptr(c, 0x0031_1110, 0x0027_5794)?;
        let no_anim = c.g[S1][0] == 0;
        c.g[A0][0] = c.g[S1][0];
        if !no_anim {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0031_12c8, 0x0027_57a4)?;
        }
        let mut one_cycle = true;
        c.cycles += 3;
        c.g[V0][0] = Addr(p_3(r32(c, S0), 0xc)).lw(c);
        let no_data = c.g[V0][0] == 0;
        c.g[V0][0] = u64::MAX;
        if !no_data {
            c.cycles += 3;
            c.g[A0][0] = Addr(p_3(r32(c, S0), 0x10)).lw(c);
            if c.g[A0][0] == 0 {
                Addr(p_3(r32(c, S0), 0x10)).sw(c, 0);
                one_cycle = false;
            } else {
                c.cycles += 3;
                callee::MEM_MAN_FREE_ENTRY.call(c, 0x0027_57c4)?;
                c.g[V0][0] = u64::MAX;
            }
        }
        if one_cycle {
            c.cycles += 1;
            Addr(p_3(r32(c, S0), 0x10)).sw(c, 0);
        }
        c.cycles += 8;
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        Addr(p_3(r32(c, S0), 0xc)).sw(c, 0);
        Addr(p_3(r32(c, S0), 8)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script animation bank manager.

/// `ScriptAnimBankMan* ScriptAnimBankMan_ctor(self, base, count, flags)`
/// (0x2753b8): allocate the bank array and initialise each entry.
pub fn script_anim_bank_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
        c.g[T0][0] = c.g[A2][0];
        c.sq(S1, p_4(r32(c, SP), 0x10));
        c.g[V1][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, p_4(r32(c, SP), 0x20));
        c.g[A0][0] = 0x14;
        c.g[V0][0] = 0x480000;
        c.mult(0, 0, 8, 4);
        c.sd(r32(c, SP), c.g[RA][0]);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x25d8));
        c.g[S0][0] = c.g[A3][0];
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = c.g[V1][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1a80));
        Addr(r32(c, S1)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.lo[0];
        Addr(p_4(r32(c, S1), 0xc)).sw(c, c.g[V0][0]);
        c.g[A3][0] = 0;
        Addr(p_4(r32(c, S1), 4)).sw(c, c.g[T0][0]);
        call_ptr(c, 0x0031_7e30, 0x0027_540c)?;
        c.g[V1][0] = Addr(p_4(r32(c, S1), 4)).lw(c);
        c.g[A1][0] = 0;
        let none = (c.g[V1][0] as i64) <= 0;
        Addr(p_4(r32(c, S1), 8)).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 3;
            c.g[A2][0] = 0xffff_ffff_ffff_ffff;
            c.g[A0][0] = 0;
            loop {
                c.cycles += 21;
                c.g[V1][0] = Addr(p_4(r32(c, S1), 8)).lw(c);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V1][0] = sx(p_4(r32(c, A0), r32(c, V1)));
                Addr(r32(c, V1)).sw(c, c.g[A2][0]);
                c.g[V0][0] = Addr(p_4(r32(c, S1), 8)).lw(c);
                c.g[V0][0] = sx(p_4(r32(c, A0), r32(c, V0)));
                Addr(p_4(r32(c, V0), 4)).sw(c, c.g[S0][0]);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = Addr(p_4(r32(c, S1), 8)).lw(c);
                c.g[V0][0] = sx(p_4(r32(c, A0), r32(c, V0)));
                Addr(p_4(r32(c, V0), 8)).sw(c, 0);
                c.g[V1][0] = Addr(p_4(r32(c, S1), 8)).lw(c);
                c.g[V1][0] = sx(p_4(r32(c, A0), r32(c, V1)));
                Addr(p_4(r32(c, V1), 0xc)).sw(c, 0);
                c.g[V0][0] = Addr(p_4(r32(c, S1), 8)).lw(c);
                c.g[V0][0] = sx(p_4(r32(c, A0), r32(c, V0)));
                Addr(p_4(r32(c, V0), 0x10)).sw(c, 0);
                c.g[V0][0] = Addr(p_4(r32(c, S1), 4)).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x14));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_5428);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        c.lq(S0, p_4(r32(c, SP), 0x20));
        c.lq(S1, p_4(r32(c, SP), 0x10));
        c.g[RA][0] = c.ld(r32(c, SP));
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x30));
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_method;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_2;
    use ssx3_ee as ee;

    // --------------------------------------------------------------- script

    #[test]
    fn script_anim_bank_man_find_free_entry_matches() {
        check(
            ee::s07::f_002757f0__generated,
            script_anim_bank_man_find_free_entry,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2757_f0, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x40);
                zero(c, DATA + 0x600, 0x100);
                c.g[A0][0] = sx(self_);
                let count = seed % 4;
                c.bus.write32(self_ + 4, count);
                c.bus.write32(self_ + 8, DATA + 0x600);
                for i in 0..4u32 {
                    let occupied = (i + seed) % 2 == 0;
                    c.bus.write32(DATA + 0x600 + i * 0x14 + 8, occupied as u32);
                }
            },
        );
    }

    #[test]
    fn script_anim_bank_man_remove_bank_matches() {
        check(
            ssx3_ee::s07::f_00275650__generated,
            script_anim_bank_man_remove_bank,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x275650, 0);
                be_library(c);
                let mgr = DATA + 0x100;
                let banks = DATA + 0x2000;
                let count = [0u32, 1, 2, 4][seed as usize % 4];
                c.bus.write32(mgr + 4, count);
                c.bus.write32(mgr + 8, banks);
                for i in 0..4u32 {
                    let e = banks + 0x14 * i;
                    c.bus.write32(e, i + 1);
                    c.bus.write32(e + 8, if i < count { 1 } else { 0 });
                }
                // A bank list method is only called when the bank is found.
                let vt = DATA + 0x3000;
                c.bus.write32(mgr + 0x0c, vt);
                fake_method(c, vt, 0x10);
                c.g[A0][0] = sx(mgr);
                c.g[A1][0] = 1 + (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn script_anim_bank_man_find_bank_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x275718, 0);
            be_library(c);
            let mgr = DATA + 0x100;
            let banks = DATA + 0x2000;
            let count: u32 = [0, 1, 2, 6, 0x8000_0000][seed as usize % 5];
            c.bus.write32(mgr + 4, count);
            c.bus.write32(mgr + 8, banks);
            for i in 0..6u32 {
                let e = banks + 0x14 * i;
                c.bus.write32(e, i);
                c.bus.write32(e + 8, if i == 2 || i == 4 { 0 } else { 1 });
            }
            c.g[A0][0] = sx(mgr);
            c.g[A1][0] = [0u64, 2, 4, 5, 9][seed as usize % 5];
            let _ = r.next_u32();
            let ev = schedule(c, &mut r, seed, 0x0027_5718);
            if ev != Events::Off {
                c.next_event = c.cycles + (r.next_u32() % 4) as u64;
            }
            ev
        };
        check_events(
            ssx3_ee::s07::f_00275718__generated,
            script_anim_bank_man_find_bank,
            80,
            &setup,
        );
    }

    #[test]
    fn script_anim_bank_man_dtor_matches() {
        check(
            ssx3_ee::s07::f_00275498__generated,
            script_anim_bank_man_dtor,
            80,
            &|c, seed| {
                let _r = setup(c, seed, 0x275498);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA + 0x408, 0);
            },
        );
    }

    #[test]
    fn script_anim_bank_man_remove_all_matches() {
        check(
            ee::s07::f_00275698__generated,
            script_anim_bank_man_remove_all,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x2756, 3);
                let m = DATA;
                let n = (seed % 4) as u32;
                w(c, m + 4, n);
                w(c, m + 8, DATA + 0x1000);
                w(c, m + 0xc, DATA + 0x1800);
                slot(c, DATA + 0x1800, 0x10, FAKE);
                for i in 0..4u32 {
                    w(
                        c,
                        DATA + 0x1000 + 0x14 * i + 8,
                        if (seed >> i) & 1 == 1 { 1 } else { 0 },
                    );
                }
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn script_anim_bank_man_free_entry_matches() {
        check(
            ee::s07::f_00275760__generated,
            script_anim_bank_man_free_entry,
            100,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x27_5760);
                globals(c);
                const TBL: u32 = DATA + 0x1000;
                w(c, GAME_GP.wrapping_add(0xd08), TBL);
                for i in 0..4u32 {
                    w(c, TBL + 4 * i, if i == 2 { DATA + 0x3000 } else { 0 });
                }
                w(c, DATA + 0x2000 + 4, (seed % 4) as u32);
                w(c, DATA + 0x2000 + 0xc, [0, 1, 1][seed as usize % 3]);
                w(
                    c,
                    DATA + 0x2000 + 0x10,
                    [0, DATA + 0x4000, 0][seed as usize % 3],
                );
                w(c, DATA + 0x2000, 0x1111);
                w(c, DATA + 0x2000 + 8, 0x2222);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                let _ = r.next_u32();
            },
        );
    }
}
