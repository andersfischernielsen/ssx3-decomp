//! `src/fe/ovstaterewardslist.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::Flow;
use crate::events::back_edge_unwinds;
use crate::guest::Addr;
use crate::call::Unwound;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
pub(crate) use crate::game::dirtysock::tags2::at;
pub(crate) use crate::game::dirtysock::tags2::p_2 as p;
pub(crate) use crate::game::dirtysock::tags2::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::fepopup3::p as p_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0020_0218, ov_screen_rewards_list_popup_init, "OVScreenRewardsListPopup_Init"),
    (0x0020_06b8, event_select_ctor, "EventSelect_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
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


/// `EventSelect* EventSelect_ctor(EventSelect* self)` (0x2006b8): when the
/// reward screen is in play, cache its selected index for the widget layer.
pub fn event_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[V0][0] = c.lw(gp(c, -0xb5c));
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0020_06d4)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0020_06e0)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_GET_INTERFACE_RIDER, 0x0020_06e8)?;
            c.g[V1][0] = c.lw(at(c, V0, 0x54));
            sw(c, gp(c, -0xb4c), 0, c.g[V1][0]);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void OVScreenRewardsListPopup_Init(OVScreenRewardsListPopup* self)`
/// (0x200218): create the rewards-list popup screen and start its thread.
pub fn ov_screen_rewards_list_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x4704_b8;
        c.g[S0][0] = Addr(at_2(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0020_023c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_0250)?;
        let t = c.g[V0][0] == 0;
        Addr(at_2(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0020_0268)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x0020_0270)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The shared body of `OVScreenTrophy_OnWidgetAction` (0x20e9a0) and
/// `OVScreenRewardsListPopup_OnWidgetAction` (0x200288): the same screen
/// widget-action handler compiled twice. `ret0` is the return address of its
/// first nested call; the others follow at fixed offsets.
pub(crate) fn ov_screen_widget_action(c: &mut Cpu, ret0: u32) -> Flow {    c.cycles += 7;
    let frame = Frame::open(c, 0x40);
    let widget = c.g[A1][0];
    frame.save(c, 16, 0x30);
    frame.save(c, 17, 0x20);
    c.g[S0][0] = widget;
    frame.save_ra(c, 0x10);
    let no_widget = c.g[S0][0] == 0;
    c.g[S1][0] = c.g[A0][0];
    if !no_widget {
        c.cycles += 3;
        c.g[V0][0] = 0x5;
        let action5 = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = 0x6;
        if action5 {
            // a "clicked" action: tell the widget (slot 0x58) the new state
            c.cycles += 16;
            c.g[V1][0] = c.lw(p_2(c.g[S0][0] as u32, 8));
            c.g[A1][0] = c.g[SP][0];
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = c.lh(p_2(c.g[V1][0] as u32, 0x58));
            c.swc1(0, p_2(c.g[SP][0] as u32, 4));
            c.swc1(0, c.g[SP][0] as u32);
            c.sw(p_2(c.g[SP][0] as u32, 8), 0);
            c.sw(p_2(c.g[SP][0] as u32, 0xc), 0);
            c.g[V0][0] = c.lw(p_2(c.g[V1][0] as u32, 0x5c));
            let func = c.g[V0][0] as u32;
            c.g[RA][0] = sx(ret0);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, ret0)?;
            c.g[V1][0] = c.lw(p_2(c.g[S0][0] as u32, 0x18));
            c.g[V0][0] = 0x1;
            let not_first = c.g[V1][0] != c.g[V0][0];
            c.g[A1][0] = 0;
            if !not_first {
                c.cycles += 10;
                c.g[V0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0x1;
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw(p_2(c.g[A3][0] as u32, 4));
                c.g[A0][0] = c.lh(p_2(c.g[V1][0] as u32, 0x20));
                c.g[V0][0] = c.lw(p_2(c.g[V1][0] as u32, 0x24));
                let func = c.g[V0][0] as u32;
                c.g[RA][0] = sx(ret0 + 0x34);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, ret0 + 0x34)?;
                c.g[A1][0] = c.g[V0][0];
            }
            c.cycles += 5;
            c.g[A0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
            c.g[RA][0] = sx(ret0 + 0x44);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f400, ret0 + 0x44)?;
            c.lq(16, p_2(c.g[SP][0] as u32, 0x30));
        } else {
            c.cycles += 2;
            if c.g[A2][0] == c.g[V0][0] {
                // the "second" action: hand the widget's state to the screen
                c.g[V0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
                c.cycles += 10;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.lw(p_2(c.g[S0][0] as u32, 0x18));
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw(p_2(c.g[A3][0] as u32, 4));
                c.g[A0][0] = c.lh(p_2(c.g[V1][0] as u32, 0x28));
                c.g[V0][0] = c.lw(p_2(c.g[V1][0] as u32, 0x2c));
                let func = c.g[V0][0] as u32;
                c.g[RA][0] = sx(ret0 + 0x6c);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, ret0 + 0x6c)?;
                let unchanged = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if !unchanged {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
                    c.g[RA][0] = sx(ret0 + 0x80);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, ret0 + 0x80)?;
                }
            } else {
                c.cycles += 2;
                c.lq(16, p_2(c.g[SP][0] as u32, 0x30));
            }
        }
    }
    c.cycles += 1;
    c.lq(16, p_2(c.g[SP][0] as u32, 0x30));
    c.cycles += 4;
    c.lq(17, p_2(c.g[SP][0] as u32, 0x20));
    c.g[RA][0] = c.ld(p_2(c.g[SP][0] as u32, 0x10));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    Ok(())
}


/// `void func_00200288(OVScreen*, Widget* w, int action)` (0x200288): the
/// rewards-list popup's widget-action handler (the same code).
pub fn ovscreen_rewards_list_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| ov_screen_widget_action(c, 0x0020_02f0))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_4 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_4 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_7 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;


    #[test]
    fn event_select_ctor_matches() {
        check(
            ssx3_ee::s05::f_002006b8__generated,
            event_select_ctor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2006b8, 0);
                be_library(c);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0xb5c),
                    if seed % 2 == 0 { 0 } else { 2 },
                );
            },
        );
    }


    #[test]
    fn ov_screen_rewards_list_popup_init_matches() {
        check(
            ee::s05::f_00200218__generated,
            ov_screen_rewards_list_popup_init,
            30,
            &|c, seed| {
                alloc_base(c, seed, 0x2002_18);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(W + 0x84, 0);
            },
        );
    }
}
