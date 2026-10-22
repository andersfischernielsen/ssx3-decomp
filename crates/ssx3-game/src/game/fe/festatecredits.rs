//! `src/fe/festatecredits.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::sp;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::dirtysock::tags2::p_2 as p;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::dirtysock::tags2::leave;
pub(crate) use crate::game::dirtysock::tags2::const_reg;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::fereal::gp;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_63b8, cfe_options_credits_update, "cFEOptionsCredits_Update"),
    (0x0018_63e8, c_fe_options_credits_handle_input, "cFEOptionsCredits_HandleInput"),
    (0x0018_6478, fe_options_credits_on_widget_notify, "cFEOptionsCredits_OnWidgetNotify"),
    (0x0018_64b0, cfe_options_credits_on_widget_action, "cFEOptionsCredits_OnWidgetAction"),
    (0x0018_65a8, cfe_state_on_deactivate, "cFEState_OnDeactivate"),
    (0x0018_6610, fe_state_event_select_ctor, "cFEStateEventSelect_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V43;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V42;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_HANDLE_EVENT;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;
    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_UPDATE;
    pub(crate) use crate::game::fe::festateload::callee::MEM_CARD_BASE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
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


/// `void cFEOptionsCredits_Update(cFEOptionsCredits* self)` (0x1863b8):
/// advance the credits scroll (`+0x60 += +0x64`), then the base update.
pub fn cfe_options_credits_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let self_ = abi::arg(c, 0);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(self_, 0x60)).lw(c);
        c.g[A1][0] = Addr(at(self_, 0x64)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        let y = c.g[V1][0];
        Addr(at(self_, 0x60)).sw(c, y);
        call_ptr(c, callee::UI_STATE_V13, 0x0018_63d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `cFEStateEventSelect* cFEStateEventSelect_ctor(self)` (0x186610): UI state
/// constructor, then install the vtable/state and construct the embedded
/// event-select sub-object at +0x48.
pub fn fe_state_event_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_CTOR, 0x0018_6624)?;
        let this = r32(c, S0);
        const_reg(c, V1, 0x47_0000, -0x42d8);
        c.g[V0][0] = 0x13;
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[A0][0] = sx(this.wrapping_add(0x48));
        call_ptr(c, 0x0020_06b8, 0x0018_6640)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    });
}


/// `int cFEOptionsCredits_OnWidgetNotify(...)` (0x186478): `Ok`/`Next`/`Prev`
/// (6, 8, 9) answer 0x100, everything else 0x101.
pub fn fe_options_credits_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is_six = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 6) as u64;
    if !is_six {
        c.cycles += 2;
        let below_six = c.g[V0][0] != 0;
        c.g[V0][0] = (c.g[A2][0] < 0xa) as u64;
        if !below_six {
            c.cycles += 2;
            let at_least_ten = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[A2][0] < 8) as u64;
            if at_least_ten {
                c.cycles += 2;
                c.g[V0][0] = WIDGET_IGNORED;
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            let below_eight = c.g[V0][0] != 0;
            if below_eight {
                c.cycles += 2;
                c.g[V0][0] = WIDGET_IGNORED;
                abi::ret(c);
                return;
            }
            // 8 <= id < 10
            c.cycles += 2;
            c.g[V0][0] = WIDGET_ACTIVATED;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = WIDGET_IGNORED;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = WIDGET_ACTIVATED;
    abi::ret(c);
}


/// `void cFEOptionsCredits_OnWidgetAction(self, item, int msg)` (0x1864b0):
/// for message 6, ask the engine object's slot 5 for the credits screen
/// (hash name 0x24) and push it if found.
pub fn cfe_options_credits_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        open(c, 0x20);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        let no_item = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if no_item {
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x6;
            if c.g[A2][0] != c.g[V0][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            } else {
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x24;
                c.g[A3][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0018_64f4);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, target, 0x0018_64f4)?;
                let none = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if none {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0018_6508);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x0018_6508)?;
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                }
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}


/// `void cFEState_OnDeactivate(cFEState* self)` (0x1865a8): find the current
/// screen's credits object and tell the FE module it was left.
pub fn cfe_state_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x30);
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0018_65c4);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
        call_ptr(c, 0x0039_7840, 0x0018_65c4)?;
        c.g[S0][0] = c.g[V0][0];
        let no_screen = c.g[S0][0] == 0;
        c.g[A0][0] = 0x4a0000;
        if no_screen {
            // The empty-screen path runs the shared tail (its +1 and lq16).
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        } else {
            c.cycles += 7;
            c.g[RA][0] = sx(0x0018_65d8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
            call_ptr(c, 0x0031_7670, 0x0018_65d8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0018_65e4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_65e4)?;
            let not_found = c.g[V0][0] == 0;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            if !not_found {
                c.cycles += 4;
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = sx(gp(c, 0x1d90));
                c.g[RA][0] = sx(0x0018_65fc);
                c.g[A2][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x95));
                call_ptr(c, 0x001a_0708, 0x0018_65fc)?;
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}


/// `int cFEOptionsCredits_HandleInput(self, Pad* pad)` (0x1863e8): map the
/// pad's left/right presses to a credits scroll step (or a page turn).
pub fn c_fe_options_credits_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let pad = abi::arg64(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = pad;
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(self_);
        c.g[V1][0] = c.lw(at_2(c, S0, 8));
        c.g[A0][0] = c.lh(at_2(c, V1, 0x88));
        c.g[V0][0] = c.lw(at_2(c, V1, 0x8c));
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        let target = r32(c, V0);
        call_ptr(c, target, 0x0018_6414)?;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V1][0] = c.lw(at_2(c, S1, 0x64));
            c.g[V0][0] = ((c.g[V1][0] as i64) < -7) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(1));
                c.cycles += 1;
                c.sw(at_2(c, S1, 0x64), c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
        } else {
            c.g[V1][0] = c.lw(at_2(c, S0, 8));
            c.cycles += 6;
            c.g[A0][0] = c.lh(at_2(c, V1, 0x90));
            c.g[V0][0] = c.lw(at_2(c, V1, 0x94));
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            let target = r32(c, V0);
            call_ptr(c, target, 0x0018_6444)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 4;
                c.g[V1][0] = c.lw(at_2(c, S1, 0x64));
                c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
                let too_far = c.g[V0][0] == 0;
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
                if too_far {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                } else {
                    c.cycles += 1;
                    c.sw(at_2(c, S1, 0x64), c.g[V0][0]);
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
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
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::vtable;
    use crate::difftest::RA as RET;
    use ssx3_ee as ee;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::fereal::tests::str_hash;
    pub(crate) use crate::game::fe::fereal::tests::fake_slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::w16;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;


    #[test]
    fn cfe_options_credits_update_matches() {
        check(s02::f_001863b8__generated, cfe_options_credits_update, 120, &|c, seed| {
            machine(c, seed, 0x1863b8, 1);
            globals(c);
            clear(c, OBJ, 0x100);
            c.g[A0][0] = sx(OBJ);
            c.bus.write32(OBJ + 0x60, 0x1000 + seed as u32);
            c.bus.write32(OBJ + 0x64, seed as u32 & 0x3f);
        });
    }


    #[test]
    fn fe_state_event_select_ctor_matches() {
        check(
            ssx3_ee::s02::f_00186610__generated,
            fe_state_event_select_ctor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x186610, 0);
                be_library(c);
                // Below 2 the event-select constructor skips the BE interface.
                c.bus.write32(
                    GAME_GP.wrapping_sub(0xb5c),
                    if seed % 2 == 0 { 0 } else { 5 },
                );
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }


    #[test]
    fn fe_options_credits_on_widget_notify_matches() {
        check(ee::s02::f_00186478__generated, fe_options_credits_on_widget_notify, SEEDS, &|c, seed| {
            machine(c, seed, 0x0018_6478, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = (seed % 12) as u64;
        });
    }


    #[test]
    fn cfe_options_credits_on_widget_action_matches() {
        check(ee::s02::f_001864b0__generated, cfe_options_credits_on_widget_action, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x18_64b0, 1);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x100) };
            c.g[A2][0] = if seed % 2 == 0 { 6 } else { (seed % 9) as u64 };
            // self+0x10 -> engine; engine+0 -> object; object+4 -> vtable.
            c.bus.write32(DATA + 0x10, DATA + 0x200);
            c.bus.write32(DATA + 0x200, DATA + 0x210);
            fake_slot(c, &mut r, DATA + 0x210, 0x4, 0x28);
        });
    }


    #[test]
    fn cfe_state_on_deactivate_matches() {
        check(ee::s02::f_001865a8__generated, cfe_state_on_deactivate, 40, &|c, seed| {
            base(c, seed, 0x18_65a8);
            let hash = str_hash(c, 0x004a_1398);
            c.g[A0][0] = sx(DATA);
            // The list at self+0x24: head at +0x28, sentinel check on +4.
            if seed % 3 == 0 {
                c.bus.write32(DATA + 0x28, 0);
            } else {
                let screen = DATA + 0x300;
                c.bus.write32(DATA + 0x28, screen);
                c.bus.write32(screen + 0x4, DATA + 0x310);
                if seed % 3 == 1 {
                    // FindObject walks a null array: returns null.
                    c.bus.write32(screen + 0x3c, 0);
                } else {
                    let count = DATA + 0x600;
                    let array = DATA + 0x610;
                    let obj = DATA + 0x700;
                    c.bus.write32(screen + 0x38, count);
                    c.bus.write32(count, 4);
                    c.bus.write32(screen + 0x3c, array);
                    c.bus.write32(array, obj);
                    c.bus.write32(obj + 0x38, hash as u32);
                    c.bus.write8(obj + 0x95, 0x2a);
                }
            }
            c.bus.write8(DATA + 0xc, 0x11);
        });
    }

    /// `SSXAudioSystem_GetInstance`: returns `*(gp+0x410)`, 0 in tests.
    pub(crate) const RET_ZERO: u32 = 0x0028_b180;


    #[test]
    fn c_fe_options_credits_handle_input_matches() {
        check(
            ee::s02::f_001863e8__generated,
            c_fe_options_credits_handle_input,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x1863, 0);
                env(c);
                let this = DATA + 0x400;
                let pad = DATA + 0x800;
                let vt = DATA + 0x1000;
                slot(c, vt, 0x88, if seed % 2 == 0 { RET_ZERO } else { EMPTY });
                slot(c, vt, 0x90, if seed % 3 == 0 { RET_ZERO } else { EMPTY });
                w32(c, pad + 8, vt);
                w32(
                    c,
                    this + 0x64,
                    [0i32, 1, -7, -8, 7, 8, 9][(seed % 7) as usize] as u32,
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pad);
            },
        );
    }
}
