//! `src/fe/festatestore.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::sp;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::fe::fepopup3::transition_in;
pub(crate) use crate::game::dirtysock::tags2::at;
pub(crate) use crate::game::dirtysock::tags2::gp_5 as gp;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::feasyncfile::lw;
pub(crate) use crate::game::be::be::vt_call;
pub(crate) use crate::game::fe::feasyncfile::p as p_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_4780, cfe_uber_trick_show_buy_buttons, "cFEStateUberTrick_ShowBuyButtons"),
    (0x0018_4b70, fe_state_uber_trick_shutdown, "cFEStateUberTrick_Shutdown"),
    (0x0018_4bb8, fe_state_uber_trick_transition_in, "cFEStateUberTrick_TransitionIn"),
    (0x0018_4be0, c_fe_state_uber_trick_on_activate, "cFEStateUberTrick_OnActivate"),
    (0x0018_59d8, fe_state_uber_trick_on_widget_notify, "cFEStateUberTrick_OnWidgetNotify"),
    (0x0018_5a18, cfe_options_credits_ctor, "cFEOptionsCredits_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
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


/// `bool cFEStateUberTrick_TransitionIn(cFEStateUberTrick* s, int on)`
/// (0x184bb8): refresh the trick list when entering, and report success.
pub fn fe_state_uber_trick_transition_in(c: &mut Cpu) {
    guest(c, |c| transition_in(c, 0x0018_4bcc, callee::UBER_TRICK_REFRESH))
}


// ---------------------------------------------------------------------------
// Front-end states.

/// `void cFEStateUberTrick_Shutdown(cFEStateUberTrick* self)` (0x184b70):
/// hide the rider preview when there is a rider manager, then shut the state
/// down.
pub fn fe_state_uber_trick_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.lw(gp(c, -0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(at(c, V0, 0x7c));
        let none = c.g[A0][0] == 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        if !none {
            c.cycles += 3;
            c.g[A1][0] = c.lb(at(c, S0, 0x44));
            c.g[A2][0] = 0;
            call_ptr(c, callee::FEMODULE_SET_PREVIEW_VISIBLE, 0x0018_4b9c)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::UI_STATE_SHUTDOWN, 0x0018_4ba4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Front end.

/// `cFEOptionsCredits* cFEOptionsCredits_ctor(void* self)` (0x185a18): the
/// credits screen: `UIObject_ctor`, vtable, and the default field values.
pub fn cfe_options_credits_ctor(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    if call_ptr(c, 0x0039_e2a0, 0x0018_5a2c).is_err() {
        return;
    }
    c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
    c.g[V1][0] = sx(0x0046_bdf8);
    c.sw(p(c, S0, 0x60), c.g[V0][0]);
    c.g[A0][0] = 1;
    c.g[A1][0] = 0x24;
    c.sw(p(c, S0, 8), c.g[V1][0]);
    c.g[V0][0] = c.g[S0][0];
    c.sw(p(c, S0, 0x64), c.g[A0][0]);
    c.sw(p(c, S0, 0xc), c.g[A1][0]);
    c.sw(p(c, S0, 0x58), 0);
    c.sw(p(c, S0, 0x5c), 0);
    c.sw(p(c, S0, 0x48), 0);
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `int cFEStateUberTrick_OnWidgetNotify(...)` (0x1859D8): `Ok` (9) answers
/// 0x100 once the widget belongs to the current page (`+0x50`), `Next`/`Prev`
/// (8, 9) on the overview page; everything else 0x101.
pub fn fe_state_uber_trick_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x50));
    let other_page = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 0xa) as u64;
    if other_page {
        c.cycles += 2;
        let below_ten = c.g[V0][0] != 0;
        c.g[V1][0] = (c.g[A2][0] < 8) as u64;
        if !below_ten {
            c.cycles += 2;
            let below_eight = c.g[V1][0] != 0;
            c.g[V0][0] = WIDGET_ACTIVATED;
            if !below_eight {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = WIDGET_IGNORED;
    } else {
        c.cycles += 3;
        c.g[V0][0] = 9;
        let not_nine = c.g[A2][0] != 9;
        c.g[V0][0] = WIDGET_IGNORED;
        if !not_nine {
            c.cycles += 2;
            c.g[V0][0] = WIDGET_ACTIVATED;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void cFEStateUberTrick_OnActivate(cFEState*, void* arg)` (0x00184be0):
/// select the left panel, notify both optional panels (`+0x4c`, `+0x50`) with
/// their own event id, then activate the base state.
pub fn c_fe_state_uber_trick_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0018_3b08, 0x0018_4bfc)?; // cFEStateUberTrick_SelectLeftPanel
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
        c.g[A1][0] = 1;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(8));
            let vt = c.g[V0][0];
            let obj = c.g[A2][0];
            vt_call(c, vt, 0x38, obj, V1, 0x0018_4c1c)?;
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[A1][0] = 0;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(8));
            let vt = c.g[V0][0];
            let obj = c.g[A2][0];
            vt_call(c, vt, 0x38, obj, V1, 0x0018_4c3c)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e4c0, 0x0018_4c48)?; // UIState_OnActivate
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}


/// `void cFEStateUberTrick_ShowBuyButtons(self, int on)` (0x184780): show or
/// hide the two buy buttons of the uber-trick screen.
pub fn cfe_uber_trick_show_buy_buttons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = sx(p_2(r32(c, A0), 0x24));
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0039_7840, 0x0018479c)?;
        c.g[A0][0] = sx(0x004a_1458);
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0031_7670, 0x001847ac)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_d860, 0x001847b8)?;
        c.g[A2][0] = c.g[V0][0];
        let missing = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !missing {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_2(r32(c, A2), 8)).lw(c);
            c.g[A0][0] = Addr(p_2(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p_2(r32(c, V0), 0x4c)).lw(c);
            let show = r32(c, V1);
            c.g[A0][0] = sx(p_2(r32(c, A2), r32(c, A0)));
            call_ptr(c, show, 0x001847d8)?;
        }
        c.cycles += 9;
        c.g[A0][0] = sx(0x004a_1460);
        call_ptr(c, 0x0031_7670, 0x001847e4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001847f0)?;
        c.g[A2][0] = c.g[V0][0];
        let missing = c.g[A2][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !missing {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_2(r32(c, A2), 8)).lw(c);
            c.g[A0][0] = Addr(p_2(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p_2(r32(c, V0), 0x4c)).lw(c);
            let show = r32(c, V1);
            c.g[A0][0] = sx(p_2(r32(c, A2), r32(c, A0)));
            call_ptr(c, show, 0x00184810)?;
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
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fill;
    use ssx3_ee as ee;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_2;


    #[test]
    fn fe_state_uber_trick_transition_in_matches() {
        crate::difftest::check(ee::s02::f_00184bb8__generated, fe_state_uber_trick_transition_in, 40, &|c, seed| {
            machine(c, seed, 0x184b_b8, 2);
            be_library(c);
            // A zeroed state object: its trick lists are empty, so the
            // refresh walks terminate at once.
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    /// A dummy world (`gp-0x848`) and race manager: rider manager at +0x7c,
    /// movie player at +0x94, overlay screen slot at +0x200.
    pub(crate) fn world(c: &mut Cpu) {
        let w = DATA + 0x4000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x7c, DATA + 0x5000);
        c.bus.write32(w + 0x84, DATA + 0x6000);
        c.bus.write32(DATA + 0x6000 + 0x94, DATA + 0x7000);
    }


    #[test]
    fn fe_state_uber_trick_shutdown_matches() {
        check(
            ssx3_ee::s02::f_00184b70__generated,
            fe_state_uber_trick_shutdown,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x184b70, 0);
                be_library_2(c);
                if seed % 3 != 0 {
                    world(c);
                }
                let this_ = DATA + 0x100;
                // The rider index selects the preview object (0xce0 bytes each).
                c.bus.write8(this_ + 0x44, (seed % 4) as u8);
                c.g[A0][0] = sx(this_);
            },
        );
    }


    // ---------------------------------------------------------- front end

    #[test]
    fn cfe_options_credits_ctor_matches() {
        check(
            ssx3_ee::s02::f_00185a18__generated,
            cfe_options_credits_ctor,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x185a18, 1);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }


    #[test]
    fn fe_state_uber_trick_on_widget_notify_matches() {
        check(ee::s02::f_001859d8__generated, fe_state_uber_trick_on_widget_notify, SEEDS, &|c, seed| {
            machine(c, seed, 0x0018_59d8, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            w(c, DATA + 0x50, (seed % 2) as u32);
            c.g[A2][0] = (seed % 12) as u64;
        });
    }


    #[test]
    fn c_fe_state_uber_trick_on_activate_matches() {
        check(
            ee::s02::f_00184be0__generated,
            c_fe_state_uber_trick_on_activate,
            60,
            &|c, seed| {
                let _ = base(c, seed, 0x184b, 3);
                let st = DATA;
                let panel_a = if seed % 3 == 0 { 0 } else { DATA + 0x600 };
                let panel_b = if seed % 4 == 0 { 0 } else { DATA + 0x700 };
                w(c, st + 0x4c, panel_a);
                w(c, st + 0x50, panel_b);
                for (p, vt) in [(panel_a, DATA + 0x680), (panel_b, DATA + 0x780)] {
                    if p != 0 {
                        w(c, p + 8, vt);
                        slot(c, vt, 0x38, FAKE);
                    }
                }
                c.g[A0][0] = sx(st);
                c.g[A1][0] = sx(DATA + 0x300);
            },
        );
    }

    pub(crate) const ENT: u32 = DATA + 0x7300;


        #[test]
    fn cfe_uber_trick_show_buy_buttons_matches() {
        check(
            ee::s02::f_00184780__generated,
            cfe_uber_trick_show_buy_buttons,
            8,
            &|c, seed| {
                let _ = base_2(c, seed, 0x184780, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, OBJ + 0x28, ENT);
                w(c, ENT + 4, if seed % 3 == 0 { ENT } else { 0 });
                w(c, 0x004a_1458, 0);
                w(c, 0x004a_1460, 0);
            },
        );
    }
}
