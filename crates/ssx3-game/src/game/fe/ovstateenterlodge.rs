//! `src/fe/ovstateenterlodge.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::dirtysock::tagsunk::gp_8 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::festateloadhint::accept_key_6;
pub(crate) use crate::game::fe::festatetrophyroom::handle_list_key;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001f_7198,
        ov_screen_enter_lodge_init,
        "OVScreenEnterLodge_Init",
    ),
    (
        0x001f_7210,
        ov_screen_enter_lodge_on_activate,
        "OVScreenEnterLodge_OnActivate",
    ),
    (
        0x001f_72b8,
        ov_screen_enter_lodge_on_widget_notify,
        "OVScreenEnterLodge_OnWidgetNotify",
    ),
    (0x001f_73a8, ov_screen_bc_start_init, "OVScreenBCStart_Init"),
    (
        0x001f_7418,
        ov_screen_bcstart_transition_in,
        "OVScreenBCStart_TransitionIn",
    ),
    (
        0x001f_74e0,
        ov_screen_bc_start_on_widget_created,
        "OVScreenBCStart_OnWidgetCreated",
    ),
    (
        0x001f_7548,
        ov_screen_bc_start_pop_event_arg,
        "OVScreenBCStart_PopEventArg",
    ),
    (
        0x001f_7590,
        ov_screen_bc_start_on_widget_notify,
        "OVScreenBCStart_OnWidgetNotify",
    ),
    (0x001f_76b8, ov_screen_bc_fail_init, "OVScreenBCFail_Init"),
    (
        0x001f_7738,
        ov_bcfail_pop_event_arg,
        "OVScreenBCFail_PopEventArg",
    ),
    (
        0x001f_7770,
        ovscreenbcfail_on_widget_created,
        "OVScreenBCFail_OnWidgetCreated",
    ),
    (
        0x001f_77f0,
        ov_screen_bc_fail_on_widget_notify,
        "OVScreenBCFail_OnWidgetNotify",
    ),
    (
        0x001f_7e68,
        ov_screen_top_times_on_widget_created,
        "OVScreenTopTimes_OnWidgetCreated",
    ),
    (
        0x001f_7f08,
        ov_screen_top_times_on_activate,
        "OVScreenTopTimes_OnActivate",
    ),
    (
        0x001f_80c8,
        ov_screen_top_times_on_widget_notify,
        "OVScreenTopTimes_OnWidgetNotify",
    ),
    (
        0x001f_80d8,
        ov_screen_top_times_on_widget_action,
        "OVScreenTopTimes_OnWidgetAction",
    ),
    (0x001f_8118, ov_screen_top_times_v26, "OVScreenTopTimes_v26"),
    (
        0x0020_a430,
        cov_state_overlay_shutdown,
        "cOVStateOverlay_Shutdown",
    ),
    (
        0x001f_7298,
        oc989_enter_lodge_shutdown,
        "OVScreenEnterLodge_Shutdown",
    ),
    (
        0x001f_75a0,
        oc989_bc_start_on_widget_action,
        "OVScreenBCStart_OnWidgetAction",
    ),
    (
        0x001f_7800,
        oc989_bc_fail_on_widget_action,
        "OVScreenBCFail_OnWidgetAction",
    ),
    (
        0x001f_7958,
        ov_screen_top_times_init,
        "OVScreenTopTimes_Init",
    ),
    (
        0x0022_0eb8,
        ov_screen_enter_lodge_dtor,
        "OVScreenEnterLodge_dtor",
    ),
    (
        0x0022_0ee0,
        ov_screen_enter_lodge_ctor,
        "OVScreenEnterLodge_ctor",
    ),
    (0x0022_0f48, ov_screen_bc_start_ctor, "OVScreenBCStart_ctor"),
    (
        0x0022_0fa8,
        ov_screen_bc_start_ctor_2,
        "OVScreenBCStart_ctor_2",
    ),
    (0x0022_1018, ov_screen_bc_start_dtor, "OVScreenBCStart_dtor"),
    (0x0022_1040, ov_screen_bc_fail_ctor, "OVScreenBCFail_ctor"),
    (
        0x0022_10a0,
        ov_screen_bc_fail_ctor_2,
        "OVScreenBCFail_ctor_2",
    ),
    (0x0022_1110, ov_screen_bc_fail_dtor, "OVScreenBCFail_dtor"),
    (
        0x0022_1138,
        ov_screen_top_times_dtor,
        "OVScreenTopTimes_dtor",
    ),
    (
        0x0022_1160,
        ov_screen_top_times_ctor,
        "OVScreenTopTimes_ctor",
    ),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;

    pub(crate) use crate::game::bx::bxstringctor::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::KEYBOARD_POPUP_SHOW;

    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;

    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;

    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;

    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;

    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;

    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
}

/// `void OVScreenBCFail_PopEventArg(OVScreenBCFail* self)` (0x1f7738):
/// pop one pending script event and stash its argument at +0x9c.
pub fn ov_bcfail_pop_event_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c4));
        call_ptr(c, callee::WSCRIPT_MAN_EVENT_QUEUE_POP, 0x001f_7754)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        frame.restore_ra(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x9c)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void OVScreenTopTimes_OnWidgetAction(self, widget, action)` (0x1F80D8):
/// action 5 on a widget forwards to the screen's virtual handler at vtable
/// slot `+0xD0`, with `this` adjusted by the slot's short delta.
pub fn ov_screen_top_times_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let this = c.g[A0][0];
        c.g[A3][0] = this;
        let widget = c.g[A1][0];
        frame.save_ra(c, 0);
        if widget == 0 {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let action = c.g[A2][0];
            frame.restore_ra(c, 0);
            if action == c.g[V0][0] {
                c.cycles += 5;
                let vt = c.lw_r_2(V0, r(c, A3).wrapping_add(8));
                let delta = c.lh_r(A0, (vt as u32).wrapping_add(0xd0));
                let target = c.lw_r_2(V1, (vt as u32).wrapping_add(0xd4));
                let t = target as u32;
                let a0 = sx(r(c, A3).wrapping_add(delta as u32));
                c.g[A0][0] = a0;
                call_ptr(c, t, 0x001f_8108)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBCStart_PopEventArg(self)` (0x1f7548): pop the pending
/// broadcast-start event argument (the global `gp+0xce8` holds it; `+0x1c4`
/// is the queue) into `+0x9c`, falling back to the global's `+0x2a0` when the
/// queue is empty.
pub fn ov_screen_bc_start_pop_event_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, 0xce8)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let queue = (c.g[V0][0] as u32).wrapping_add(0x1c4);
        c.g[A0][0] = sx(queue);
        call_ptr(c, 0x0030_c820, 0x001f_7564)?;
        let self_ = abi::r32(c, S0);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(gp(c, 0xce8)).lw(c);
            c.cycles += 2;
            let fallback = c.g[V0][0] as u32;
            c.g[V1][0] = Addr(fallback.wrapping_add(0x2a0)).lw(c);
            Addr(self_.wrapping_add(0x9c)).sw(c, c.g[V1][0]);
        } else {
            c.cycles += 3;
            let value = c.g[V0][0] as u32;
            c.g[V0][0] = Addr(value).lw(c);
            Addr(self_.wrapping_add(0x9c)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `D_004A26FC`: the "top times visible" flag.
pub(crate) const TOP_TIMES_VISIBLE_GP: i32 = -0x9f4;

/// `void OVScreenTopTimes_v26(s, void* widget)` (0x1f8118): once the screen's
/// +0xa0 word is clear, interpret the widget's `+0x18` selection: 0 shows the
/// top times (setting the flag at +0xa0 and `D_004A26FC`), 1 plays the
/// confirmation sound. 5 (+3, +2, +3) + 2 (+2) + 1 + 2 cycles.
pub fn ov_screen_top_times_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        let busy = c.lw(p(self_, 0xa0));
        c.g[V0][0] = busy;
        frame.restore_ra(c, 0);
        if busy == 0 {
            c.cycles += 3;
            let sel = c.lw(p(r32(c, A1), 0x18));
            c.g[A1][0] = sel;
            let none = sel == 0;
            c.g[V0][0] = 1;
            if none {
                c.cycles += 3;
                c.sw(p(self_, 0xa0), c.g[V0][0]);
                c.sw(gp_2(c, TOP_TIMES_VISIBLE_GP), c.g[V0][0]);
                c.cycles += 1;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                if sel == c.g[V0][0] {
                    c.cycles += 2;
                    c.sw(p(self_, 0xa0), c.g[A1][0]);
                    c.cycles += 2;
                    c.g[A0][0] = 0x13;
                    call_ptr(c, callee::KEYBOARD_POPUP_SHOW, 0x001f_815c)?;
                    c.cycles += 1;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int OVScreenBCStart_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f7590).
pub fn ov_screen_bc_start_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int OVScreenBCFail_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f77f0).
pub fn ov_screen_bc_fail_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int OVScreenTopTimes_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f80c8).
pub fn ov_screen_top_times_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int OVScreenEnterLodge_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f72b8).
pub fn ov_screen_enter_lodge_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `OVScreenBCStart_OnWidgetCreated(OVScreenBCStart* self, Widget* item)`
/// (0x1f74e0): compare the widget's id with the two screen-name hashes and set
/// the widget's `+0x18` to 0 or 1 when one matches.
pub fn ov_screen_bc_start_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x004a_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x004a_23e0;
        frame.save_ra(c, 0);
        let item = c.g[A1][0];
        c.g[S1][0] = item;
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw(c, p(item as u32, 0x38));
        call_ptr(c, 0x0031_7670, 0x001f_7504)?;
        let first = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x004a_0000;
        if !first {
            c.cycles += 2;
            sw(c, p(item as u32, 0x18), 0);
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw(c, p(item as u32, 0x38));
            c.g[A0][0] = 0x004a_23e8;
            call_ptr(c, 0x0031_7670, 0x001f_7520)?;
            let second = c.g[S0][0] != c.g[V0][0];
            frame.restore(c, S0, 0x20);
            if !second {
                c.cycles += 2;
                c.g[V0][0] = 1;
                sw(c, p(item as u32, 0x18), c.g[V0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBCStart_Init(OVScreenBCStart* self)` (0x1f73a8): create the
/// screen by name, start its thread and initialise the overlay state.
pub fn ov_screen_bc_start_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46f4_e0;
        c.g[S0][0] = Addr(at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_73cc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_73e0)?;
        let t = c.g[V0][0] == 0;
        Addr(at(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_73f8)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x001f_7400)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end screens.

/// `func_001F7198` (0x1f7198): `OVScreenEnterLodge::Init`. Creates the screen
/// widget named `0x46f4d0` from `this->+0x10` (`0x397cc0`), stores it at
/// `+0x40`, and if it exists shows it (`0x39c870`); finally runs the two shared
/// finish calls (`0x20d110`, `0x20a380`). Signature: `void init(this)`.
pub fn ov_screen_enter_lodge_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x47_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_sub(0xb30));
        c.g[RA][0] = sx(0x001f_71bc);
        c.g[S0][0] = lw_r(c, S1, 0x10);
        call_ptr(c, 0x0031_7670, 0x001f_71bc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001f_71d0);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_71d0)?;
        let missing = c.g[V0][0] == 0;
        sw_r(c, S1, 0x40, c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_71e8);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_71e8)?;
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x001f_71f0);
        call_ptr(c, 0x0020_d110, 0x001f_71f0)?;
        c.g[RA][0] = sx(0x001f_71f8);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x001f_71f8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBCFail_Init(self)` (0x001f76b8): create the BC-fail screen
/// and initialise the overlay.
pub fn ov_screen_bc_fail_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(0x470000);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx(0x470000u32.wrapping_sub(0xb00));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::GET_HASH_VALUE_32.call(c, 0x001f_76dc)?;
        c.g[A0][0] = c.g[S0][0];
        Addr(at_2(c, S1, 0xc)).sw(c, c.g[V0][0]);
        c.g[S0][0] = Addr(at_2(c, S1, 0x10)).lw(c);
        callee::GET_HASH_VALUE_32.call(c, 0x001f_76ec)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME.call(c, 0x001f_7700)?;
        let absent = c.g[V0][0] == 0;
        Addr(at_2(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_SCREEN_PLAY_FRAME.call(c, 0x001f_7718)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        callee::C_OV_STATE_OVERLAY_INIT.call(c, 0x001f_7720)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBCFail_OnWidgetCreated(self, widget)` (0x1f7770): pick the
/// failure screen's mode (0..2) from the created widget's name hash.
pub fn ovscreenbcfail_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x4a0000;
        frame.save(c, 17, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23e0));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 16, 0x20);
        c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x38));
        call_ptr(c, callee::STR_HASH, 0x001f_7794)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a0000;
        if !differs {
            c.cycles += 2;
            c.sw(p(r32(c, S1), 0x18), 0);
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23e8));
            call_ptr(c, callee::STR_HASH, 0x001f_77b0)?;
            if c.g[S0][0] != c.g[V0][0] {
                c.g[A0][0] = 0x4a0000;
                c.cycles += 5;
                c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x38));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23f8));
                call_ptr(c, callee::STR_HASH, 0x001f_77cc)?;
                let differs = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, 16, 0x20);
                if differs {
                    c.cycles += 4;
                    frame.restore(c, 17, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 1;
                c.g[V0][0] = 2;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
            }
            c.cycles += 1;
            c.sw(p(r32(c, S1), 0x18), c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenEnterLodge_OnActivate(OVScreenEnterLodge* self, int)` (0x1f7210):
/// clear the "disabled" flag of the lodge button, set the "enabled" flag of
/// the leave button and activate the state.
pub fn ov_screen_enter_lodge_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x4a_0000;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x001f_7234);
        c.g[A0][0] = sx(0x004a_23e0);
        call_ptr(c, 0x0031_7670, 0x001f_7234)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[RA][0] = sx(0x001f_7240);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_7240)?;
        c.g[V1][0] = c.g[V0][0];
        if c.g[V1][0] != 0 {
            Addr(p(r32(c, V1), 0x18)).sw(c, 0);
        }
        c.cycles += 9;
        c.g[A0][0] = 0x4a_0000;
        c.g[RA][0] = sx(0x001f_7258);
        c.g[A0][0] = sx(0x004a_23e8);
        call_ptr(c, 0x0031_7670, 0x001f_7258)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[RA][0] = sx(0x001f_7264);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_7264)?;
        c.g[V1][0] = c.g[V0][0];
        let null = c.g[V1][0] == 0;
        c.g[V0][0] = 0x1;
        if !null {
            c.cycles += 1;
            Addr(p(r32(c, V1), 0x18)).sw(c, c.g[V0][0]);
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001f_7280);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e4c0, 0x001f_7280)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenTopTimes_OnWidgetCreated(self, widget)` (0x1f7e68): pick the
/// top-times title string and store the "online" flag.
pub fn ov_screen_top_times_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x0046_f578;
        frame.save(c, S0, 0x20);
        let widget = abi::arg(c, 1);
        c.g[S0][0] = sx(widget);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = lw_2(c, S0, 0x38);
        call_ptr(c, table::STR_HASH, 0x001f_7e8c)?;
        c.g[A0][0] = 0x0047_0000;
        let other = c.g[S1][0] != c.g[V0][0];
        if other {
            c.cycles += 4;
            c.g[A0][0] = 0x0046_f5a8;
            call_ptr(c, table::STR_HASH, 0x001f_7ee4)?;
            if c.g[S1][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                sw_2(c, S0, 0x18, c.g[V0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp_3(c, -0x9f0)).lw(c);
            if c.g[V0][0] != 0 {
                c.g[A0][0] = 0x0046_f598;
                c.cycles += 7;
                call_ptr(c, table::STR_HASH, 0x001f_7ec8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, table::UI_TEXT_SET_TEXT_ID, 0x001f_7ed4)?;
                sw_2(c, S0, 0x18, 0);
            } else {
                c.cycles += 8;
                c.g[A0][0] = 0x0046_f588;
                call_ptr(c, table::STR_HASH, 0x001f_7eac)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, table::UI_TEXT_SET_TEXT_ID, 0x001f_7eb8)?;
                sw_2(c, S0, 0x18, 0);
            }
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
// `cOVStateOverlay_Shutdown` (0x20a430): stop the overlay's cut scene, close
// its PDA template, find and shut down its UI state, then run the base
// shutdown.
pub fn cov_state_overlay_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x50));
        let no_cutscene = c.g[V0][0] == 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if no_cutscene {
            c.cycles += 1;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                call_ptr(c, 0x002ea860, 0x0020_a464)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x50), 0u64);
                c.cycles += 1;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            }
        }
        c.cycles += 10;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20d0));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x00317670, 0x0020_a47c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039f9d8, 0x0020_a488)?;
        c.g[A3][0] = c.g[V0][0];
        let no_state = c.g[A3][0] == 0;
        c.g[A1][0] = 0x4;
        if !no_state {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A2][0] = 0u64;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_a4ac)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
        let no_pda = c.g[V0][0] == 0;
        if !no_pda {
            c.cycles += 2;
            callee::OV_SCREEN_PDA_TEMPLATE_CLOSE.call(c, 0x0020_a4c0)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039e4a0, 0x0020_a4c8)?;
        c.lq(16, frame.at(c, 0x20));
        c.lq(17, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenEnterLodge_OnWidgetAction(this, widget, msg)` (0x1f72e0): the
/// "enter lodge" prompt — quit to the load screen for widget 0, mark the
/// screen done and close it for widget 1.
pub fn ov_screen_enter_lodge_on_widget_action(c: &mut Cpu) {
    'b_1f739c: {
        'b_1f7398: {
            c.cycles += 4;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.g[A3][0] = c.g[A0][0];
            let t = c.g[A1][0] == 0u64;
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            if t {
                break 'b_1f7398;
            } else {
                'b_1f7370: {
                    'b_1f730c: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x5;
                        let t = c.g[A2][0] == c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_1f730c;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A2][0] == c.g[V0][0];
                            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                            if t {
                                break 'b_1f7370;
                            } else {
                                c.cycles += 2;
                                let t = true;

                                if t {
                                    break 'b_1f739c;
                                } else {
                                    break 'b_1f730c;
                                }
                            }
                        }
                    }
                    'b_1f7354: {
                        'b_1f733c: {
                            c.cycles += 8;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A2][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x200));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                            if c.g[V0][0] == 0u64 {
                                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                break 'b_1f733c;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == c.g[A2][0];
                                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                if t {
                                    break 'b_1f7354;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_1f739c;
                                    } else {
                                        break 'b_1f733c;
                                    }
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = 0x27;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x172c), c.g[V0][0]);
                        c.g[RA][0] = sx(0x001f734c);
                        c.g[A1][0] = 0u64;
                        if call_ptr(c, 0x00230488, 0x001f734c).is_err() {
                            return;
                        }
                        let t = true;
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        if t {
                            break 'b_1f739c;
                        } else {
                            break 'b_1f7354;
                        }
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x1;
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                    c.g[RA][0] = sx(0x001f7368);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    if call_ptr(c, 0x0039f190, 0x001f7368).is_err() {
                        return;
                    }
                    let t = true;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    if t {
                        break 'b_1f739c;
                    } else {
                        break 'b_1f7370;
                    }
                }
                c.cycles += 10;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A2][0] = 0x1;
                c.sw((c.g[A3][0] as u32).wrapping_add(0x54), c.g[A2][0]);
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x200));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = sx(0x001f7398);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                if call_ptr(c, 0x0039f190, 0x001f7398).is_err() {
                    return;
                }
                break 'b_1f7398;
            }
        }
        c.cycles += 1;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        break 'b_1f739c;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
    return;
}

/// `void OVScreenBCStart_TransitionIn(OVScreenBCStart *self, int a1)`
/// (0x1f7418): fade/slide the BC-start widgets in.
pub fn ov_screen_bcstart_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f74c8: {
            'b_1f74c4: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[A1][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_1f74c4;
                } else {
                    'b_1f748c: {
                        c.cycles += 15;
                        c.g[RA][0] = sx(0x001f743c);
                        call_ptr(c, 0x0014dc80, 0x001f743c)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001f7448);
                        c.g[A1][0] = 0xa;
                        call_ptr(c, 0x0014dd58, 0x001f7448)?;
                        c.g[A0][0] = 0x470000;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001f7458);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xb10));
                        call_ptr(c, 0x00317670, 0x001f7458)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001f7464);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001f7464)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A0][0] = 0x4a0000;
                            break 'b_1f748c;
                        } else {
                            c.cycles += 7;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                            c.g[RA][0] = sx(0x001f747c);
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x00153c88, 0x001f747c)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001f7488);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0e90, 0x001f7488)?;
                            c.g[A0][0] = 0x4a0000;
                            break 'b_1f748c;
                        }
                    }
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x001f7494);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23f0));
                    call_ptr(c, 0x00317670, 0x001f7494)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f74a0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f74a0)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_1f74c8;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                        c.g[RA][0] = sx(0x001f74b8);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x00153d28, 0x001f74b8)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001f74c4);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a0d00, 0x001f74c4)?;
                        break 'b_1f74c4;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1f74c8;
        }
        c.cycles += 6;
        c.g[V0][0] = 0u64;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// OVScreenTopTimes_OnActivate.

pub(crate) const UI_STATE_ON_ACTIVATE: u32 = 0x0039_e4c0;

pub(crate) const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

pub(crate) const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

pub(crate) const SPRINTF: u32 = 0x0041_7828;

pub(crate) const PLAY_OBJECT_ANIM: u32 = 0x0020_a6f0;

/// Format strings in `.rodata` and the global byte that selects the second
/// round of labels.
pub(crate) const LABEL_FORMATS: [u32; 4] = [0x004a_2440, 0x004a_2428, 0x004a_2430, 0x004a_2438];

pub(crate) const LABEL_MODE: u32 = 0x0053_5c11;

/// Four `sprintf(buf, fmt[i], count)` + `PlayObjectAnim(this, buf, anim)`
/// pairs, with the original's return addresses.
pub(crate) fn play_object_labels(
    c: &mut Cpu,
    this: u64,
    count: u64,
    rets: &[(u32, u32); 4],
) -> Flow {
    for i in 0..4 {
        c.g[A1][0] = LABEL_FORMATS[i] as u64;
        c.g[A0][0] = sp(c) as u64;
        c.g[A2][0] = count;
        call_ptr(c, SPRINTF, rets[i].0)?;
        c.g[S0][0] = 0x0047_0000;
        c.g[A1][0] = sp(c) as u64;
        c.g[S0][0] = sx(0x0047_0000u32.wrapping_sub(0x1b18));
        c.g[A0][0] = this;
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, PLAY_OBJECT_ANIM, rets[i].1)?;
    }
    Ok(())
}

/// `void OVScreenTopTimes_OnActivate(OVScreenTopTimes *s)` (0x1f7f08): format
/// and play the screen's label animations for the current mode.
pub fn ov_screen_top_times_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x20);
        c.g[S2][0] = abi::arg64(c, 0);
        let this = c.g[S2][0];

        call_ptr(c, UI_STATE_ON_ACTIVATE, 0x001f_7f28)?;
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x001f_7f30)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 8;
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x001f_7f3c)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x001f_7f44)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x001f_7f50)?;

        c.g[S1][0] = Addr(r32(c, S3)).off(0x18).lw(c);
        let negative = (c.g[S1][0] as i64) < 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
        if !negative {
            c.cycles += 38;
            play_object_labels(
                c,
                this,
                c.g[S1][0],
                &[
                    (0x001f_7f70, 0x001f_7f88),
                    (0x001f_7f9c, 0x001f_7fac),
                    (0x001f_7fc0, 0x001f_7fd0),
                    (0x001f_7fe4, 0x001f_7ff4),
                ],
            )?;
        }

        // The second block only runs for label mode 2.
        c.cycles += 5;
        c.g[V0][0] = 0x0053_0000;
        c.g[A0][0] = 2;
        c.g[V1][0] = Addr(LABEL_MODE).lb(c);
        if c.g[V1][0] == c.g[A0][0] {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S3)).off(0x1c).lw(c);
            let negative = (c.g[V0][0] as i64) < 0;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            if !negative {
                c.cycles += 39;
                play_object_labels(
                    c,
                    this,
                    c.g[S1][0],
                    &[
                        (0x001f_8028, 0x001f_8040),
                        (0x001f_8054, 0x001f_8064),
                        (0x001f_8078, 0x001f_8088),
                        (0x001f_809c, 0x001f_80ac),
                    ],
                )?;
            }
        }

        c.cycles += 6;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenTopTimes_Init(cOVScreenTopTimes* self)` (0x001f7958): builds the
/// "top times" pause overlay. Adds the two UI screens (title and list), runs the
/// overlay base init, then fills the header text from the current event type
/// (`0x535bc8+0x4a`: peak events use their name text, event 5 a formatted track
/// name, others the track name plus the mode name from `0x535bc8+0x48`), shows or
/// hides the full/short layout objects by mode, and writes five rows of
/// score-holder name, owner and score (min:sec format in the full layout). Ends
/// with the front-end confirmation sound. Every register, cycle and call is kept
/// as in the generated body (its straight conversion: calls through `call_ptr`,
/// the mode and event jump tables as `match`).
pub fn ov_screen_top_times_init(c: &mut Cpu) {
    if c.pc != 0x001f7958 {
        return ov_screen_top_times_init_resume(c);
    }
    guest(c, |c| 'l_1f7958: loop {
        'b_1f79c8: {
            c.cycles += 24;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x220));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1f0));
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x200));
            c.g[A0][0] = 0x470000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1e0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xaf0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1d0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1c0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x1a0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x190));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x180), c.g[RA][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0xa0), 0u64);
            c.g[RA][0] = sx(0x001f799c);
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001f799c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f79b0);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001f79b0)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1f79c8;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001f79c8);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001f79c8)?;
                break 'b_1f79c8;
            }
        }
        'b_1f7a00: {
            c.cycles += 11;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x001f79d8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
            call_ptr(c, 0x00317670, 0x001f79d8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f79ec);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001f79ec)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_1f7a00;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001f7a00);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001f7a00)?;
                break 'b_1f7a00;
            }
        }
        'b_1f7b90: {
            'b_1f7acc: {
                'b_1f7a90: {
                    c.cycles += 22;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7a10);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x170), c.g[V0][0]);
                    call_ptr(c, 0x0020a380, 0x001f7a10)?;
                    c.g[RA][0] = sx(0x001f7a18);
                    call_ptr(c, 0x0014dc80, 0x001f7a18)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7a24);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x001f7a24)?;
                    c.g[RA][0] = sx(0x001f7a2c);
                    c.g[S4][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x001f7a2c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7a38);
                    c.g[A1][0] = 0x8;
                    call_ptr(c, 0x0014dd58, 0x001f7a38)?;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[V1][0] = 0x5;
                    c.g[V0][0] = 0x530000;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    let t = c.g[A0][0] == c.g[V1][0];
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    if t {
                        break 'b_1f7a90;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A0][0] as i64) < 5i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                        if t {
                            break 'b_1f7acc;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A0][0] as i64) < 12i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                            if t {
                                break 'b_1f7acc;
                            } else {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x001f7a78);
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                                Callee {
                                    addr: 0x001fd190,
                                    f: ee::s04::FE_GetEventNameText,
                                }
                                .call(c, 0x001f7a78)?;
                                c.g[S4][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001f7a88);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002c2508, 0x001f7a88)?;
                                let t = true;
                                c.g[A0][0] = 0x470000;
                                if t {
                                    break 'b_1f7b90;
                                } else {
                                    break 'b_1f7a90;
                                }
                            }
                        }
                    }
                }
                c.cycles += 15;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xae0));
                c.g[RA][0] = sx(0x001f7aa4);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c2540, 0x001f7aa4)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x001f7ab0);
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                call_ptr(c, 0x00144bf0, 0x001f7ab0)?;
                c.g[S4][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f7ac4);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x002c26d0, 0x001f7ac4)?;
                let t = true;
                c.g[A0][0] = 0x470000;
                if t {
                    break 'b_1f7b90;
                } else {
                    break 'b_1f7acc;
                }
            }
            'b_1f7b54: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[S3][0] = 0u64;
                if t {
                    break 'b_1f7b54;
                } else {
                    'b_1f7b2c: {
                        'b_1f7b20: {
                            'b_1f7b10: {
                                'b_1f7b00: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x470000;
                                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xaa0));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x001f7958 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001f7958);
                                            }
                                            continue 'l_1f7958;
                                        }
                                        0x001f7af0 => {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x470000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                                            if t {
                                                break 'b_1f7b2c;
                                            } else {
                                                break 'b_1f7b00;
                                            }
                                        }
                                        0x001f7b00 => {
                                            break 'b_1f7b00;
                                        }
                                        0x001f7b10 => {
                                            break 'b_1f7b10;
                                        }
                                        0x001f7b20 => {
                                            break 'b_1f7b20;
                                        }
                                        0x001f7b54 => {
                                            break 'b_1f7b54;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x001f7958, 0x001f7e68);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x470000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c68));
                                if t {
                                    break 'b_1f7b2c;
                                } else {
                                    break 'b_1f7b10;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c58));
                            if t {
                                break 'b_1f7b2c;
                            } else {
                                break 'b_1f7b20;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c40));
                        break 'b_1f7b2c;
                    }
                    c.cycles += 10;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001f7b40);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001f7b40)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001f7b50);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001f7b50)?;
                    c.g[S3][0] = c.g[V0][0];
                    break 'b_1f7b54;
                }
            }
            c.cycles += 15;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2400));
            c.g[RA][0] = sx(0x001f7b68);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c2540, 0x001f7b68)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x001f7b74);
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            call_ptr(c, 0x00144bf0, 0x001f7b74)?;
            c.g[S4][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001f7b8c);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x002c26d0, 0x001f7b8c)?;
            c.g[A0][0] = 0x470000;
            break 'b_1f7b90;
        }
        'b_1f7ca4: {
            'b_1f7c38: {
                'b_1f7bc8: {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x001f7b98);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xad0));
                    call_ptr(c, 0x00317670, 0x001f7b98)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7ba4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7ba4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7bb0);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x003a0e90, 0x001f7bb0)?;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x5;
                    if t {
                        break 'b_1f7bc8;
                    } else {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1f7c38;
                        } else {
                            break 'b_1f7bc8;
                        }
                    }
                }
                c.cycles += 28;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2408));
                c.g[RA][0] = sx(0x001f7bdc);
                c.g[S0][0] = 0x470000;
                call_ptr(c, 0x0039e948, 0x001f7bdc)?;
                c.g[S1][0] = 0x470000;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2410));
                c.g[RA][0] = sx(0x001f7bf4);
                c.g[S4][0] = c.g[S0][0];
                call_ptr(c, 0x0039e8b8, 0x001f7bf4)?;
                c.g[S3][0] = c.g[S1][0];
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f7c08);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xac0));
                call_ptr(c, 0x0039e948, 0x001f7c08)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f7c18);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xab0));
                call_ptr(c, 0x0039e8b8, 0x001f7c18)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f7c24);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1608));
                call_ptr(c, 0x0039e948, 0x001f7c24)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f7c30);
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1618));
                call_ptr(c, 0x0039e8b8, 0x001f7c30)?;
                let t = true;
                c.g[A1][0] = 0x4a0000;
                if t {
                    break 'b_1f7ca4;
                } else {
                    break 'b_1f7c38;
                }
            }
            c.cycles += 27;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2408));
            c.g[RA][0] = sx(0x001f7c48);
            c.g[S0][0] = 0x470000;
            call_ptr(c, 0x0039e8b8, 0x001f7c48)?;
            c.g[S1][0] = 0x470000;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2410));
            c.g[RA][0] = sx(0x001f7c60);
            c.g[S4][0] = c.g[S0][0];
            call_ptr(c, 0x0039e948, 0x001f7c60)?;
            c.g[S3][0] = c.g[S1][0];
            c.g[A1][0] = 0x470000;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x170), 0u64);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xac0));
            c.g[RA][0] = sx(0x001f7c78);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0039e8b8, 0x001f7c78)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7c88);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xab0));
            call_ptr(c, 0x0039e948, 0x001f7c88)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7c94);
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1608));
            call_ptr(c, 0x0039e8b8, 0x001f7c94)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7ca0);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1618));
            call_ptr(c, 0x0039e948, 0x001f7ca0)?;
            c.g[A1][0] = 0x4a0000;
            break 'b_1f7ca4;
        }
        'b_1f7cf8: {
            c.cycles += 14;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7cb0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2418));
            call_ptr(c, 0x0039e948, 0x001f7cb0)?;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7cc0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2420));
            call_ptr(c, 0x0039e948, 0x001f7cc0)?;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f7cd0);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2090));
            call_ptr(c, 0x0039e948, 0x001f7cd0)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9f0));
            if c.g[V0][0] == 0u64 {
                c.g[S3][0] = 0x1;
                break 'b_1f7cf8;
            } else {
                c.cycles += 7;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1608));
                c.g[RA][0] = sx(0x001f7ce8);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001f7ce8)?;
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1618));
                c.g[RA][0] = sx(0x001f7cf4);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001f7cf4)?;
                c.g[S3][0] = 0x1;
                break 'b_1f7cf8;
            }
        }
        c.cycles += 6;
        c.g[FP][0] = 0x4a0000;
        c.g[S7][0] = 0x4a0000;
        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S6][0] = 0x4a0000;
        c.g[V1][0] = 0x4a0000;
        'l_1f7d10: loop {
            'b_1f7e0c: {
                'b_1f7df0: {
                    c.cycles += 47;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2428));
                    c.g[RA][0] = sx(0x001f7d20);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x001f7d20)?;
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    c.g[RA][0] = sx(0x001f7d2c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001f7d2c)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7d38);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7d38)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x001f7d48);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00155208, 0x001f7d48)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001f7d54);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0c50, 0x001f7d54)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001f7d64);
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2430));
                    call_ptr(c, 0x00417828, 0x001f7d64)?;
                    c.g[RA][0] = sx(0x001f7d6c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001f7d6c)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7d78);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7d78)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x001f7d88);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00155190, 0x001f7d88)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001f7d94);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0c50, 0x001f7d94)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x2438));
                    c.g[RA][0] = sx(0x001f7da4);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x001f7da4)?;
                    c.g[RA][0] = sx(0x001f7dac);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001f7dac)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7db8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7db8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        break 'b_1f7df0;
                    } else {
                        c.cycles += 9;
                        c.g[RA][0] = sx(0x001f7dd4);
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x00155130, 0x001f7dd4)?;
                        c.g[RA][0] = sx(0x001f7ddc);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0020a1a8, 0x001f7ddc)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001f7de8);
                        c.g[A0][0] = c.g[S4][0];
                        call_ptr(c, 0x00417828, 0x001f7de8)?;
                        let t = true;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_1f7e0c;
                        } else {
                            break 'b_1f7df0;
                        }
                    }
                }
                c.cycles += 7;
                c.g[RA][0] = sx(0x001f7df8);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x00155130, 0x001f7df8)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x001f7e08);
                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2138));
                call_ptr(c, 0x00417828, 0x001f7e08)?;
                c.g[A0][0] = c.g[S0][0];
                break 'b_1f7e0c;
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x001f7e14);
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x003a0c50, 0x001f7e14)?;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 6i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V1][0] = 0x4a0000;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001f7d10);
                }
                continue 'l_1f7d10;
            } else {
                c.cycles += 17;
                c.g[RA][0] = sx(0x001f7e2c);
                call_ptr(c, 0x0028b180, 0x001f7e2c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f7e38);
                c.g[A1][0] = 0xe;
                call_ptr(c, 0x00294f78, 0x001f7e38)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x180));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// Native resume of `ov_screen_top_times_init` (converted from the `entries` body): any entry
/// other than 0x001f7958, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn ov_screen_top_times_init_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x001f7958 => {
                    c.cycles += 24;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x220));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x200));
                    c.g[A0][0] = 0x470000;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xaf0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x190));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x180), c.g[RA][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xa0), 0u64);
                    c.g[RA][0] = sx(0x001f799c);
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317670, 0x001f799c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f79b0);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00397cc0, 0x001f79b0)?;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    if t {
                        pc = 0x001f79c8;
                        continue;
                    }
                    pc = 0x001f79b8;
                    continue;
                }
                0x001f79b8 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001f79c8);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039c870, 0x001f79c8)?;
                    pc = 0x001f79c8;
                    continue;
                }
                0x001f79c8 => {
                    c.cycles += 11;
                    c.g[A0][0] = 0x470000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x001f79d8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
                    call_ptr(c, 0x00317670, 0x001f79d8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f79ec);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00397cc0, 0x001f79ec)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[V0][0];
                    if t {
                        pc = 0x001f7a00;
                        continue;
                    }
                    pc = 0x001f79f4;
                    continue;
                }
                0x001f79f4 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001f7a00);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039c870, 0x001f7a00)?;
                    pc = 0x001f7a00;
                    continue;
                }
                0x001f7a00 => {
                    c.cycles += 22;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7a10);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x170), c.g[V0][0]);
                    call_ptr(c, 0x0020a380, 0x001f7a10)?;
                    c.g[RA][0] = sx(0x001f7a18);
                    call_ptr(c, 0x0014dc80, 0x001f7a18)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7a24);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x001f7a24)?;
                    c.g[RA][0] = sx(0x001f7a2c);
                    c.g[S4][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x001f7a2c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7a38);
                    c.g[A1][0] = 0x8;
                    call_ptr(c, 0x0014dd58, 0x001f7a38)?;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[V1][0] = 0x5;
                    c.g[V0][0] = 0x530000;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    let t = c.g[A0][0] == c.g[V1][0];
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    if t {
                        pc = 0x001f7a90;
                        continue;
                    }
                    pc = 0x001f7a58;
                    continue;
                }
                0x001f7a58 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                    if t {
                        pc = 0x001f7acc;
                        continue;
                    }
                    pc = 0x001f7a64;
                    continue;
                }
                0x001f7a64 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 12i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                    if t {
                        pc = 0x001f7acc;
                        continue;
                    }
                    pc = 0x001f7a70;
                    continue;
                }
                0x001f7a70 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x001f7a78);
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    Callee {
                        addr: 0x001fd190,
                        f: ee::s04::FE_GetEventNameText,
                    }
                    .call(c, 0x001f7a78)?;
                    c.g[S4][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7a88);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c2508, 0x001f7a88)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        pc = 0x001f7b90;
                        continue;
                    }
                    pc = 0x001f7a90;
                    continue;
                }
                0x001f7a90 => {
                    c.cycles += 15;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xae0));
                    c.g[RA][0] = sx(0x001f7aa4);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c2540, 0x001f7aa4)?;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001f7ab0);
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    call_ptr(c, 0x00144bf0, 0x001f7ab0)?;
                    c.g[S4][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7ac4);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x002c26d0, 0x001f7ac4)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        pc = 0x001f7b90;
                        continue;
                    }
                    pc = 0x001f7acc;
                    continue;
                }
                0x001f7acc => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x001f7b54;
                        continue;
                    }
                    pc = 0x001f7ad4;
                    continue;
                }
                0x001f7ad4 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x470000;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xaa0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001f7af0 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                    if t {
                        pc = 0x001f7b2c;
                        continue;
                    }
                    pc = 0x001f7b00;
                    continue;
                }
                0x001f7b00 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c68));
                    if t {
                        pc = 0x001f7b2c;
                        continue;
                    }
                    pc = 0x001f7b10;
                    continue;
                }
                0x001f7b10 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c58));
                    if t {
                        pc = 0x001f7b2c;
                        continue;
                    }
                    pc = 0x001f7b20;
                    continue;
                }
                0x001f7b20 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c40));
                    pc = 0x001f7b2c;
                    continue;
                }
                0x001f7b2c => {
                    c.cycles += 10;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001f7b40);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x001f7b40)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001f7b50);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001f7b50)?;
                    c.g[S3][0] = c.g[V0][0];
                    pc = 0x001f7b54;
                    continue;
                }
                0x001f7b54 => {
                    c.cycles += 15;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2400));
                    c.g[RA][0] = sx(0x001f7b68);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c2540, 0x001f7b68)?;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001f7b74);
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    call_ptr(c, 0x00144bf0, 0x001f7b74)?;
                    c.g[S4][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001f7b8c);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x002c26d0, 0x001f7b8c)?;
                    c.g[A0][0] = 0x470000;
                    pc = 0x001f7b90;
                    continue;
                }
                0x001f7b90 => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x001f7b98);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xad0));
                    call_ptr(c, 0x00317670, 0x001f7b98)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7ba4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7ba4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7bb0);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x003a0e90, 0x001f7bb0)?;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x001f7bc8;
                        continue;
                    }
                    pc = 0x001f7bc0;
                    continue;
                }
                0x001f7bc0 => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A1][0] = 0x4a0000;
                        pc = 0x001f7c38;
                        continue;
                    }
                    pc = 0x001f7bc8;
                    continue;
                }
                0x001f7bc8 => {
                    c.cycles += 28;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2408));
                    c.g[RA][0] = sx(0x001f7bdc);
                    c.g[S0][0] = 0x470000;
                    call_ptr(c, 0x0039e948, 0x001f7bdc)?;
                    c.g[S1][0] = 0x470000;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2410));
                    c.g[RA][0] = sx(0x001f7bf4);
                    c.g[S4][0] = c.g[S0][0];
                    call_ptr(c, 0x0039e8b8, 0x001f7bf4)?;
                    c.g[S3][0] = c.g[S1][0];
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7c08);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xac0));
                    call_ptr(c, 0x0039e948, 0x001f7c08)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7c18);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xab0));
                    call_ptr(c, 0x0039e8b8, 0x001f7c18)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7c24);
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1608));
                    call_ptr(c, 0x0039e948, 0x001f7c24)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7c30);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1618));
                    call_ptr(c, 0x0039e8b8, 0x001f7c30)?;
                    let t = true;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x001f7ca4;
                        continue;
                    }
                    pc = 0x001f7c38;
                    continue;
                }
                0x001f7c38 => {
                    c.cycles += 27;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2408));
                    c.g[RA][0] = sx(0x001f7c48);
                    c.g[S0][0] = 0x470000;
                    call_ptr(c, 0x0039e8b8, 0x001f7c48)?;
                    c.g[S1][0] = 0x470000;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2410));
                    c.g[RA][0] = sx(0x001f7c60);
                    c.g[S4][0] = c.g[S0][0];
                    call_ptr(c, 0x0039e948, 0x001f7c60)?;
                    c.g[S3][0] = c.g[S1][0];
                    c.g[A1][0] = 0x470000;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x170), 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xac0));
                    c.g[RA][0] = sx(0x001f7c78);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0039e8b8, 0x001f7c78)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7c88);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xab0));
                    call_ptr(c, 0x0039e948, 0x001f7c88)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7c94);
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1608));
                    call_ptr(c, 0x0039e8b8, 0x001f7c94)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7ca0);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1618));
                    call_ptr(c, 0x0039e948, 0x001f7ca0)?;
                    c.g[A1][0] = 0x4a0000;
                    pc = 0x001f7ca4;
                    continue;
                }
                0x001f7ca4 => {
                    c.cycles += 14;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7cb0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2418));
                    call_ptr(c, 0x0039e948, 0x001f7cb0)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7cc0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2420));
                    call_ptr(c, 0x0039e948, 0x001f7cc0)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f7cd0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2090));
                    call_ptr(c, 0x0039e948, 0x001f7cd0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9f0));

                    if c.g[V0][0] == 0u64 {
                        c.g[S3][0] = 0x1;
                        pc = 0x001f7cf8;
                        continue;
                    }
                    pc = 0x001f7cdc;
                    continue;
                }
                0x001f7cdc => {
                    c.cycles += 7;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1608));
                    c.g[RA][0] = sx(0x001f7ce8);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0039e8b8, 0x001f7ce8)?;
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1618));
                    c.g[RA][0] = sx(0x001f7cf4);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0039e8b8, 0x001f7cf4)?;
                    c.g[S3][0] = 0x1;
                    pc = 0x001f7cf8;
                    continue;
                }
                0x001f7cf8 => {
                    c.cycles += 6;
                    c.g[FP][0] = 0x4a0000;
                    c.g[S7][0] = 0x4a0000;
                    c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S6][0] = 0x4a0000;
                    c.g[V1][0] = 0x4a0000;
                    pc = 0x001f7d10;
                    continue;
                }
                0x001f7d10 => {
                    c.cycles += 47;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2428));
                    c.g[RA][0] = sx(0x001f7d20);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x001f7d20)?;
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    c.g[RA][0] = sx(0x001f7d2c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001f7d2c)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7d38);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7d38)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x001f7d48);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00155208, 0x001f7d48)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001f7d54);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0c50, 0x001f7d54)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001f7d64);
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2430));
                    call_ptr(c, 0x00417828, 0x001f7d64)?;
                    c.g[RA][0] = sx(0x001f7d6c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001f7d6c)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7d78);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7d78)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x001f7d88);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00155190, 0x001f7d88)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001f7d94);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0c50, 0x001f7d94)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x2438));
                    c.g[RA][0] = sx(0x001f7da4);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x001f7da4)?;
                    c.g[RA][0] = sx(0x001f7dac);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001f7dac)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f7db8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f7db8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x001f7df0;
                        continue;
                    }
                    pc = 0x001f7dcc;
                    continue;
                }
                0x001f7dcc => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001f7dd4);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00155130, 0x001f7dd4)?;
                    c.g[RA][0] = sx(0x001f7ddc);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0020a1a8, 0x001f7ddc)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7de8);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00417828, 0x001f7de8)?;
                    let t = true;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x001f7e0c;
                        continue;
                    }
                    pc = 0x001f7df0;
                    continue;
                }
                0x001f7df0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001f7df8);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00155130, 0x001f7df8)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001f7e08);
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2138));
                    call_ptr(c, 0x00417828, 0x001f7e08)?;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x001f7e0c;
                    continue;
                }
                0x001f7e0c => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001f7e14);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x003a0c50, 0x001f7e14)?;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 6i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0x4a0000;
                    if t {
                        if back_edge_unwinds(c, 0x001f7d10) {
                            return Ok(());
                        }
                        pc = 0x001f7d10;
                        continue;
                    }
                    pc = 0x001f7e24;
                    continue;
                }
                0x001f7e24 => {
                    c.cycles += 17;
                    c.g[RA][0] = sx(0x001f7e2c);
                    call_ptr(c, 0x0028b180, 0x001f7e2c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001f7e38);
                    c.g[A1][0] = 0xe;
                    call_ptr(c, 0x00294f78, 0x001f7e38)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x210));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x200));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x190));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x180));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001f7958, 0x001f7e68);
                    return Ok(());
                }
            }
        }
    });
}

/// `base + off` as a 32-bit guest address (the `off(reg)` operand).
pub(crate) fn oc989_at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `gp + off` (negative offsets wrap, as the assembler's `-0x69c(gp)` does).
pub(crate) fn oc989_gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `void cOVScreenEnterLodge_Shutdown(cOVScreenEnterLodge* self)` (0x1f7298):
/// clears the overlay's `+0x50` word and runs the overlay state shutdown.
pub fn oc989_enter_lodge_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(oc989_at(c, V0, 0x50)).sw(c, 0);
        call_ptr(c, 0x0020_a430, 0x001f_72ac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `[gp+0xce8]` buy-attrib object's table update of the start screen
/// (`0x530000` table of pointers indexed by the level `gp-0x69c`).
pub(crate) fn oc989_bc_start_table(c: &mut Cpu) {
    c.g[V0][0] = sx(0x0053_66e8);
    c.g[A2][0] = 0x0044_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = 3;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let a0 = c.g[A0][0];
    Addr(oc989_gp(c, -0x9f4)).sw(c, a0);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let a1 = c.g[A1][0];
    Addr(oc989_gp(c, -0x69c)).sw(c, a1);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let v1 = c.g[V1][0];
    Addr(oc989_gp(c, -0x6a0)).sw(c, v1);
}

/// The `I` block of the start screen's message switch: `[gp+0xce8]` gets the
/// attribute value, then one call (with a store on the zero path).
pub(crate) fn oc989_bc_start_i_part(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
    call_ptr(c, 0x0030_b8c0, 0x001f_75e4)?;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = Addr(oc989_at(c, S0, 0x9c)).lw(c);
        c.cycles += 4;
        c.g[A0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
        let value = c.g[V0][0];
        Addr(oc989_at(c, A0, 0x2a0)).sw(c, value);
        call_ptr(c, 0x0030_b540, 0x001f_7608)?;
        c.g[A1][0] = Addr(oc989_gp(c, -0x69c)).lw(c);
    } else {
        c.cycles += 4;
        c.g[A0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
        call_ptr(c, 0x0030_b6f8, 0x001f_75f4)?;
        c.g[A1][0] = Addr(oc989_gp(c, -0x69c)).lw(c);
    }
    Ok(())
}

/// The `cycles += 19` tail of the start screen's message switch.
pub(crate) fn oc989_bc_start_g_tail(c: &mut Cpu) -> Flow {
    c.cycles += 19;
    c.g[A0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
    let value = c.g[V0][0];
    Addr(oc989_at(c, A0, 0x2a0)).sw(c, value);
    call_ptr(c, 0x0030_b658, 0x001f_7668)?;
    c.g[A1][0] = Addr(oc989_gp(c, -0x69c)).lw(c);
    oc989_bc_start_table(c);
    Ok(())
}

/// The start screen's last block (the `cycles += 20` path): the table
/// update, then two calls.
pub(crate) fn oc989_bc_start_20(c: &mut Cpu) -> Flow {
    c.cycles += 20;
    oc989_bc_start_table(c);
    call_ptr(c, 0x0028_b180, 0x001f_764c)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0029_d6d0, 0x001f_7654)?;
    Ok(())
}

/// The message switch of `OVScreenBCStart::OnWidgetAction`. Returns true when
/// the function falls into its `cycles += 1` tail, false when it leaves at
/// once.
pub(crate) fn oc989_bc_start_switch(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 5;
    let is_five = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 6;
    if !is_five {
        c.cycles += 2;
        if c.g[A2][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(oc989_at(c, S0, 0x9c)).lw(c);
            oc989_bc_start_g_tail(c)?;
            return Ok(true);
        }
        c.cycles += 2;
        return Ok(false);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(oc989_at(c, A1, 0x18)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(oc989_at(c, S0, 0x9c)).lw(c);
        oc989_bc_start_g_tail(c)?;
        return Ok(true);
    }
    oc989_bc_start_i_part(c)?;
    oc989_bc_start_20(c)?;
    Ok(false)
}

/// `void OVScreenBCStart_OnWidgetAction(OVScreenBCStart* self, item, int msg)`
/// (0x1f75a0): on a widget action for the first (5) or second (6) message,
/// records the chosen value in the start screen's object at `[gp+0xce8]`.
/// For `msg` 5 the item's `+0x18` word picks which of the two paths runs.
pub fn oc989_bc_start_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let fall = if c.g[A1][0] == 0 {
            true
        } else {
            oc989_bc_start_switch(c)?
        };
        if fall {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The table update of the fail screen (the `0x530000` pointer table).
pub(crate) fn oc989_bc_fail_table(c: &mut Cpu) {
    c.g[A0][0] = Addr(oc989_gp(c, -0x69c)).lw(c);
    c.g[A1][0] = sx(0x0053_66e8);
    c.g[A2][0] = sx(0x0044_28f0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    let level = c.g[A0][0];
    Addr(oc989_gp(c, -0x69c)).sw(c, level);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let v1 = c.g[V1][0];
    Addr(oc989_gp(c, -0x6a0)).sw(c, v1);
}

/// The `cycles += 20` block of the fail screen: two calls, the table update.
pub(crate) fn oc989_bc_fail_twenty(c: &mut Cpu) -> Flow {
    c.cycles += 20;
    c.g[A0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
    call_ptr(c, 0x0030_b658, 0x001f_78fc)?;
    c.g[A0][0] = Addr(oc989_at(c, S0, 0x10)).lw(c);
    c.g[A1][0] = 1;
    c.g[A0][0] = sx(oc989_at(c, A0, 0x18));
    call_ptr(c, 0x0039_f190, 0x001f_790c)?;
    oc989_bc_fail_table(c);
    Ok(())
}

/// The `A2 == 6` block of the fail screen: a hash lookup and one object call.
pub(crate) fn oc989_bc_fail_h(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.g[A0][0] = sx(0x0046_e050);
    call_ptr(c, 0x0031_7670, 0x001f_78cc)?;
    c.g[A0][0] = Addr(oc989_at(c, S0, 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_c778, 0x001f_78d8)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = 0xffff;
    let not_found = c.g[A1][0] != c.g[V0][0];
    c.g[A1][0] &= 0xffff;
    if not_found {
        c.cycles += 3;
        c.g[A0][0] = Addr(oc989_at(c, S0, 0x40)).lw(c);
        c.g[A2][0] = 1;
        call_ptr(c, 0x0039_c870, 0x001f_78f4)?;
    }
    Ok(())
}

/// The message switch of `OVScreenBCFail::OnWidgetAction`. Returns true when
/// the function falls into its `cycles += 1` tail, false when it leaves at
/// once.
pub(crate) fn oc989_bc_fail_switch(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 5;
    let is_five = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 6;
    if !is_five {
        c.cycles += 2;
        c.g[A0][0] = 0x0047_0000;
        if c.g[A2][0] != c.g[V0][0] {
            c.cycles += 2;
            return Ok(false);
        }
        oc989_bc_fail_h(c)?;
        oc989_bc_fail_twenty(c)?;
        return Ok(true);
    }
    c.cycles += 3;
    c.g[A1][0] = Addr(oc989_at(c, A1, 0x18)).lw(c);
    c.g[V0][0] = 2;
    if c.g[A1][0] == 0 {
        c.cycles += 28;
        c.g[V0][0] = Addr(oc989_gp(c, 0xce8)).lw(c);
        c.g[V1][0] = Addr(oc989_at(c, S0, 0x9c)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        let value = c.g[V1][0];
        Addr(oc989_at(c, V0, 0x2a0)).sw(c, value);
        call_ptr(c, 0x0030_b6f8, 0x001f_7850)?;
        c.g[A0][0] = Addr(oc989_at(c, S0, 0x10)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = sx(oc989_at(c, A0, 0x18));
        call_ptr(c, 0x0039_f190, 0x001f_7860)?;
        oc989_bc_fail_table(c);
        call_ptr(c, 0x0028_b180, 0x001f_789c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0029_d6d0, 0x001f_78a4)?;
        return Ok(false);
    }
    c.cycles += 2;
    if c.g[A1][0] != c.g[V0][0] {
        oc989_bc_fail_twenty(c)?;
        return Ok(true);
    }
    c.cycles += 4;
    c.g[A0][0] = 0x1d;
    call_ptr(c, 0x0020_ab50, 0x001f_78bc)?;
    Ok(false)
}

/// `void OVScreenBCFail_OnWidgetAction(OVScreenBCFail* self, item, int msg)`
/// (0x1f7800): the fail screen's counterpart of the start screen's handler:
/// message 5 with the item's `+0x18` word picks the path, message 6 looks up
/// a widget by hash, and the rest of the work is the same table update.
pub fn oc989_bc_fail_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let fall = if c.g[A1][0] == 0 {
            true
        } else {
            oc989_bc_fail_switch(c)?
        };
        if fall {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_ENTERLODGE::~cOVState_ENTERLODGE` (0x220eb8): stores the
/// ENTERLODGE vtable (`0x470000 + 0x4e08`) at `this+8`, then calls
/// `UIState_dtor` (0x39e390) on `this`.
pub fn ov_screen_enter_lodge_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_add(0x4e08));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_0ed4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_ENTERLODGE::cOVState_ENTERLODGE(this, engine, overlayId)`
/// (0x220ee0): the base constructor `UIState_ctor` (0x39e2a0) with the engine
/// argument, then the overlay id at `+0x98`, the vtable at `+8` (stored twice:
/// the second value, `0x473760`, stays), zeroed words at `+0x48/+0x4c/+0x54`,
/// and the 16-bit word at `+0x58` read from `0x4a2088`.
pub fn ov_screen_enter_lodge_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_0efc)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3760));
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_BIGCHALLENGE_START::cOVState_BIGCHALLENGE_START(this, engine,
/// overlayId)` (0x220f48, `func_00220F48` in the reference): `UIState_ctor`
/// with a zero engine argument, then the zeroed words, the BIGCHALLENGE_START
/// vtable (`0x473688`) at `+8`, and the 16-bit word at `+0x58`.
pub fn ov_screen_bc_start_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_0f60)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3688));
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_BIGCHALLENGE_START` constructor, the variant with the overlay id
/// (0x220fa8): `UIState_ctor` with the engine argument, the overlay id at
/// `+0x98`, the vtable at `+8`, zeroed words, the 16-bit word at `+0x58`, then
/// `OVScreenBCStart_PopEventArg` (0x1f7548) on `this` (returns to 0x221000).
pub fn ov_screen_bc_start_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_0fc4)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V0][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3688));
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V0][0]);
        c.sh(this.wrapping_add(0x58), c.g[V1][0]);
        call_ptr(c, 0x001f_7548, 0x0022_1000)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_BIGCHALLENGE_START::~cOVState_BIGCHALLENGE_START` (0x221018):
/// the BIGCHALLENGE_START vtable (`0x470000 + 0x4e08`) at `this+8`, then
/// `UIState_dtor` (0x39e390) on `this`.
pub fn ov_screen_bc_start_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_add(0x4e08));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_1034)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_BIGCHALLENGE_FAIL::cOVState_BIGCHALLENGE_FAIL(this)` (0x221040):
/// `UIState_ctor` with a zero engine argument, then the zeroed words, the
/// BIGCHALLENGE_FAIL vtable (`0x4735b0`) at `+8`, and the 16-bit word at `+0x58`.
pub fn ov_screen_bc_fail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1058)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x35b0));
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_BIGCHALLENGE_FAIL` constructor, the variant with the overlay id
/// (0x2210a0): as the BIGCHALLENGE_START variant, with the FAIL vtable
/// (`0x4735b0`), then `OVScreenBCFail_PopEventArg` (0x1f7738) on `this`
/// (returns to 0x2210f8).
pub fn ov_screen_bc_fail_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_10bc)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V0][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b0));
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V0][0]);
        c.sh(this.wrapping_add(0x58), c.g[V1][0]);
        call_ptr(c, 0x001f_7738, 0x0022_10f8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUIState_cUIState` (0x39e2a0): the base constructor every overlay state runs first.
pub(crate) const OV_STATE_BASE_CTOR: u32 = 0x0039_e2a0;

/// `cUIState__cUIState` (0x39e390): the base destructor the overlay destructors run.
pub(crate) const OV_STATE_BASE_DTOR: u32 = 0x0039_e390;

/// `EventSelect_ctor` (0x2006b8): the event-select object the map overlay builds.
pub(crate) const OV_EVENT_SELECT_CTOR: u32 = 0x0020_06b8;

/// The address `lui 0x470000; addiu off` forms: a vtable or static in the
/// 0x47xxxx data of the image (the overlay state vtables).
pub(crate) fn ov_image_sym(off: u32) -> u64 {
    let base: u32 = 0x0047_0000;
    sx(base.wrapping_add(off))
}

/// Shared body of the overlay state destructors (0x221110, 0x221138, 0x221298):
/// store the overlay vtable at `this+8` (`this` = `a0`), then run the base
/// destructor with the same `a0`. `ret` is the return address of that call.
pub(crate) fn ov_state_dtor_body(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = ov_image_sym(0x4e08);
    let vtable = c.g[V0][0];
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, vtable);
    call_ptr(c, OV_STATE_BASE_DTOR, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cOVState_BIGCHALLENGE_FAIL::~` (0x221110): overlay vtable at `this+8`, then the base dtor.
pub fn ov_screen_bc_fail_dtor(c: &mut Cpu) {
    guest(c, |c| ov_state_dtor_body(c, 0x0022_112c));
}

/// `cOVState_TOPTIMES::~` (0x221138): the same destructor shape as the BC fail screen.
pub fn ov_screen_top_times_dtor(c: &mut Cpu) {
    guest(c, |c| ov_state_dtor_body(c, 0x0022_1154));
}

/// `cOVState_TOPTIMES::cOVState_TOPTIMES(this, engine, overlayId)` (0x221160):
/// the base constructor with `overlayId` (a2), then the top-times vtables and
/// fields (overlay id at +0x98, zeroed lists and counters, a halfword from the
/// game image's table at `0x4a2088` into +0x58). Returns `this` in v0.
pub fn ov_screen_top_times_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OV_STATE_BASE_CTOR, 0x0022_117c)?;
        let this = Addr(c.g[S0][0] as u32);
        let overlay_id = c.g[S1][0];
        this.off(0x98).sw(c, overlay_id);
        this.off(0x48).sw(c, 0);
        let vtable = ov_image_sym(0x4e08);
        c.g[V1][0] = vtable;
        this.off(0x8).sw(c, vtable);
        c.g[A1][0] = 0x004a_0000;
        this.off(0x4c).sw(c, 0);
        this.off(0x54).sw(c, 0);
        c.g[V1][0] = ov_image_sym(0x34d0);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        let halfword = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[A0][0] = halfword;
        let v1 = c.g[V1][0];
        this.off(0x8).sw(c, v1);
        this.off(0x58).sh(c, halfword);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;

    use crate::difftest::fill;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_6;

    pub(crate) use crate::game::bx::bxstringctor::tests::slot_2 as slot;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::put_text;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;

    pub(crate) use crate::game::fe::fememcardcallbacks::tests::hash_screen;

    pub(crate) use crate::game::fe::festateloadhint::cfe_state_progressive_verify_ps2_on_widget_notify;
    pub(crate) use crate::game::fe::festateoptions::tests::belib as belib_2;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::fe::ovtemplatedialog::ov_screen_overlay_menu_on_widget_notify;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;

    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_4;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_4;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base as alloc_base_2;

    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_3;
    use ssx3_ee as ee;

    #[test]
    fn ov_bcfail_pop_event_arg_matches() {
        check(
            ssx3_ee::s04::f_001f7738__generated,
            ov_bcfail_pop_event_arg,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf7738, 0);
                let q = DATA + 0x200;
                c.bus.write32(GAME_GP + 0xce8, q);
                let head = r.next_u32();
                c.bus.write32(q + 0x1c4, head);
                c.bus
                    .write32(q + 0x1c8, if seed % 3 == 0 { head } else { r.next_u32() });
                c.g[A0][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn ov_screen_top_times_on_widget_action_matches() {
        check(
            ssx3_ee::s04::f_001f80d8__generated,
            ov_screen_top_times_on_widget_action,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f80_d8, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                match seed % 3 {
                    0 => {
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 5;
                    }
                    1 => {
                        c.g[A1][0] = (DATA + 0x300) as u64;
                        c.g[A2][0] = 4;
                    }
                    _ => {
                        c.g[A1][0] = (DATA + 0x300) as u64;
                        c.g[A2][0] = 5;
                        let vt = vtable(c, &mut r, &[0xd0]);
                        w(c, DATA + 8, vt);
                    }
                }
            },
        );
    }

    #[test]
    fn ov_screen_bc_start_pop_event_arg_matches() {
        check(
            ee::s04::f_001f7548__generated,
            ov_screen_bc_start_pop_event_arg,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f75_48, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x2a0, r.next_u32());
                let head = r.next_u32();
                c.bus.write32(DATA + 0x11c4, head);
                c.bus.write32(
                    DATA + 0x11c8,
                    if seed % 3 == 0 { head } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn ov_screen_top_times_v26_matches() {
        check(
            ee::s04::f_001f8118__generated,
            ov_screen_top_times_v26,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1f81);
                let s = DATA;
                let widget = DATA + 0x400;
                c.bus.write32(s + 0xa0, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(
                    widget + 0x18,
                    match seed % 4 {
                        0 => 0,
                        _ => 2 + r.next_u32() % 6,
                    },
                );
                c.bus.write32(gp_at(-0x9f4), 0);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(widget);
            },
        );
    }

    #[test]
    fn accept_key_6_family_matches() {
        check(
            ssx3_ee::s04::f_001f7590__generated,
            ov_screen_bc_start_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f7590, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001f77f0__generated,
            ov_screen_bc_fail_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f77f0, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001f80c8__generated,
            ov_screen_top_times_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f80c8, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s05::f_0020d308__generated,
            ov_screen_overlay_menu_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x20d308, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s06::f_00246268__generated,
            cfe_state_progressive_verify_ps2_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x246268, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    /// Runs a generated function to completion as a setup step.
    pub(crate) fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    // ---------------------------------------------------------- misc FE widgets

    #[test]
    fn ov_screen_bc_start_on_widget_created_matches() {
        check(
            ssx3_ee::s04::f_001f74e0__generated,
            ov_screen_bc_start_on_widget_created,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1f74e0);
                let h0 = run(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x004a_23e0]);
                let h1 = run(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x004a_23e8]);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let id = match seed % 3 {
                    0 => h0,
                    1 => h1,
                    _ => r.next_u32() as u64,
                };
                c.bus.write32(DATA + 0x138, id as u32);
                c.bus.write32(DATA + 0x118, r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_bc_start_init_matches() {
        check(
            ee::s04::f_001f73a8__generated,
            ov_screen_bc_start_init,
            30,
            &|c, seed| {
                alloc_base(c, seed, 0x1f73_a8);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(W + 0x84, 0);
            },
        );
    }

    #[test]
    fn ov_screen_enter_lodge_init_matches() {
        check(
            ee::s04::f_001f7198__generated,
            ov_screen_enter_lodge_init,
            16,
            &|c, seed| {
                machine(c, seed, 0x1f7198, 8);
                be_library(c);
                world(c);
                heap_init_2(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = this as u64;
                w(c, this + 0x10, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn ov_screen_bc_fail_init_matches() {
        check(
            ee::s04::f_001f76b8__generated,
            ov_screen_bc_fail_init,
            8,
            &|c, seed| {
                let _ = alloc_base_2(c, seed, 0x1f76_b8, 4);
                let engine = DATA + 0x1000;
                w(c, DATA + 0x10, engine);
                w(c, engine, DATA + 0x1100);
                w(c, engine + 4, 0);
                w(c, DATA + 0x1100, 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// `str_hash(text)` computed on a scratch copy (the hasher overwrites the
    /// string it hashes with the result).
    pub(crate) fn hash_text(c: &mut Cpu, bytes: &[u8]) -> u32 {
        let tmp = DATA + 0x3f00;
        for (i, b) in bytes.iter().enumerate() {
            c.bus.write8(tmp + i as u32, *b);
        }
        c.bus.write8(tmp + bytes.len() as u32, 0);
        c.g[A0][0] = sx(tmp);
        c.g[RA][0] = sx(0x0012_3450);
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        c.g[V0][0] as u32
    }

    #[test]
    fn ovscreenbcfail_on_widget_created_matches() {
        check(
            ee::s04::f_001f7770__generated,
            ovscreenbcfail_on_widget_created,
            SEEDS_4,
            &|c, seed| {
                base_4(c, seed, 0x7770, 0);
                // Short strings at the three name addresses the original hashes.
                for (addr, text) in [
                    (0x004a_23e0u32, b'A'),
                    (0x004a_23e8, b'B'),
                    (0x004a_23f8, b'C'),
                ] {
                    c.bus.write8(addr, text);
                    c.bus.write8(addr + 1, 0);
                }
                let h0 = hash_text(c, b"A");
                let h1 = hash_text(c, b"B");
                let h2 = hash_text(c, b"C");
                let widget = DATA + 0x100;
                c.g[A1][0] = sx(widget);
                let h = match seed % 4 {
                    0 => h0,
                    1 => h1,
                    2 => h2,
                    _ => 0xdead_beef,
                };
                c.bus.write32(widget + 0x38, h);
            },
        );
    }

    #[test]
    fn ov_screen_enter_lodge_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001f7210__generated,
            ov_screen_enter_lodge_on_activate,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f7210, 6);
                be_library(c);
                world(c);
                let st = DATA + 0x100;
                c.g[A0][0] = st as u64;
                c.g[A1][0] = r.next_u32() as u64;
                w(c, st + 0x40, DATA + 0x300);
                if seed % 3 != 0 {
                    let objs = DATA + 0x400;
                    let obj = DATA + 0x500;
                    w(c, DATA + 0x300 + 0x38, objs);
                    w(c, DATA + 0x300 + 0x3c, 1);
                    w(c, objs, obj);
                    w(c, obj + 0x38, 0);
                }
            },
        );
    }

    /// `GetHashValue32` over the bytes at_3 `addr` (the same algorithm as
    /// `StrHashElf`, for fixtures whose strings the test writes itself).
    pub(crate) fn hash(c: &mut Cpu, addr: u32) -> u32 {
        let mut h: u32 = 0;
        let mut at_3 = addr;
        loop {
            let b = c.bus.read8(at_3);
            if b == 0 {
                return h;
            }
            h = h.wrapping_shl(4).wrapping_add(b as i32 as u32);
            let tmp = h & 0xf000_0000;
            if tmp != 0 {
                h ^= tmp >> 23;
                h ^= tmp;
            }
            at_3 += 1;
        }
    }

    #[test]
    fn ov_screen_top_times_on_widget_created_matches() {
        check(
            ssx3_ee::s04::f_001f7e68__generated,
            ov_screen_top_times_on_widget_created,
            20,
            &|c, seed| {
                let _r = base_5(c, seed, 0x1f7e68, 2);
                let widget = DATA + 0x1000;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = widget as u64;
                put_text(c, 0x0046_f578, b"aaa");
                put_text(c, 0x0046_f588, b"bbb");
                put_text(c, 0x0046_f598, b"ccc");
                put_text(c, 0x0046_f5a8, b"ddd");
                let h1 = hash(c, 0x0046_f578);
                let h4 = hash(c, 0x0046_f5a8);
                let h = match seed % 4 {
                    0 | 1 => h1,
                    2 => h4,
                    _ => 0x1111_2222,
                };
                w_2(c, widget + 0x38, h);
                w_2(c, GAME_GP.wrapping_sub(0x9f0), (seed % 3 == 0) as u32);
            },
        );
    }

    #[test]
    fn cov_state_overlay_shutdown_matches() {
        check(
            ee::s05::f_0020a430__generated,
            cov_state_overlay_shutdown,
            40,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x20a4);
                let screen = DATA + 0x1000;
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x50, (seed % 2) as u32);
                c.bus.write32(DATA + 0x54, (seed / 2 % 2) as u32);
                c.bus.write32(DATA + 0x10, screen - 0x18);
                hash_screen(c, screen, node, 0x004a_20d0);
                c.bus.write32(node + 8, DATA + 0x3000);
                slot(c, DATA + 0x3000, 0xc0, 1);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn ov_screen_bcstart_transition_in_matches() {
        strict(
            ee::s04::f_001f7418__generated,
            ov_screen_bcstart_transition_in,
            40,
            &|c, seed| {
                let _r = base_7(c, seed, 0x1f74);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(DATA + 0x40, 0);
                c.bus.write32(DATA + 0x9c, seed as u32);
            },
        );
    }

    #[test]
    fn ov_screen_top_times_on_activate_matches() {
        check(
            ee::s04::f_001f7f08__generated,
            ov_screen_top_times_on_activate,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f_7f08, 12);
                belib_2(c);
                c.bus.write8(LABEL_MODE, if seed % 2 == 0 { 0 } else { 2 });
                // No screen: PlayObjectAnim takes its not-found path.
                w(c, DATA + 0x40, 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::Rng;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::oc982_machine;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::oc982_path_count;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::oc982_world;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::OC982_SELF;

    use std::sync::atomic::AtomicUsize;

    /// The game state bytes at `0x535bc8`: `+0x48` mode, `+0x49` flag, `+0x4a` event.
    pub(crate) const OC982_STATE: u32 = 0x0053_5bc8;

    // -- 0x001f7958 OVScreenTopTimes_Init ------------------------------------

    pub(crate) fn top_times_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc982_machine(c, seed, 0x1f7958, 70);
        oc982_world(c, &mut r);
        // Event type (5, the peak events 6..11, others) and mode (0..7).
        let ev = [5u8, 6, 9, 11, 0, 3, 12, 7][seed as usize % 8];
        let mode = ((seed / 3) % 8) as u8;
        c.bus.write8(OC982_STATE + 0x4a, ev);
        c.bus.write8(OC982_STATE + 0x48, mode);
        c.bus.write32(GAME_GP.wrapping_sub(0x9f0), (seed / 5) % 2);
        c.g[A0][0] = OC982_SELF as u64;
        c.pc = 0x001f_7958;
    }

    /// OVScreenTopTimes_Init with every callee faked (some seeds unwind inside
    /// a call, some results are zero), over the event types and modes.
    #[test]
    fn ov_screen_top_times_init_matches() {
        check(
            ee::s04::f_001f7958__generated,
            ov_screen_top_times_init,
            96,
            &top_times_setup,
        );
        assert!(
            oc982_path_count(ee::s04::f_001f7958__generated, 96, &top_times_setup) >= 4,
            "fixture reaches too few paths"
        );
    }

    #[test]
    fn oc989_enter_lodge_shutdown_matches() {
        check(
            ee::s04::f_001f7298__generated,
            oc989_enter_lodge_shutdown,
            24,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_7298, 0);
                elf_image(c);
                c.bus.write32(DATA + 0x10, seed);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn oc989_bc_start_on_widget_action_matches() {
        check(
            ee::s04::f_001f75a0__generated,
            oc989_bc_start_on_widget_action,
            24,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_75a0, 0);
                elf_image(c);
                let this = DATA + 0x100;
                let item = DATA + 0x200;
                c.bus.write32(item + 0x18, seed % 2);
                c.bus.write32(this + 0x9c, seed);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x800);
                c.bus.write32(GAME_GP.wrapping_sub(0x69c), 2 + seed % 3);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(item) };
                c.g[A2][0] = [5u64, 6, 7][(seed % 3) as usize];
            },
        );
    }

    #[test]
    fn oc989_bc_fail_on_widget_action_matches() {
        check(
            ee::s04::f_001f7800__generated,
            oc989_bc_fail_on_widget_action,
            24,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_7800, 0);
                elf_image(c);
                let this = DATA + 0x100;
                let item = DATA + 0x200;
                c.bus.write32(item + 0x18, seed % 3);
                c.bus.write32(this + 0x9c, seed);
                c.bus.write32(this + 0x40, DATA + 0x500);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x800);
                c.bus.write32(GAME_GP.wrapping_sub(0x69c), 2 + seed % 3);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(item) };
                c.g[A2][0] = [5u64, 6, 7][(seed % 3) as usize];
            },
        );
    }

    /// The fixture shared by the eight functions: scrambled registers, `this`
    /// at `DATA + 0x100`, the engine (a1) and overlay id (a2) varied by seed.
    pub(crate) fn object_fixture(c: &mut Cpu, seed: u32, salt: u64) -> u32 {
        let mut r = machine(c, seed, salt, 0);
        let this = DATA + 0x100;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = (seed % 4) as u64;
        c.g[A2][0] = if seed % 3 == 0 {
            0
        } else {
            r.next_u32() as u64
        };
        this
    }

    /// The constructors' `_2` variants call a `PopEventArg` helper that pops
    /// the global queue at `gp-0xce8`: an empty queue takes its fallback.
    pub(crate) fn empty_pop_queue(c: &mut Cpu, queue: u32) {
        c.bus.write32(GAME_GP + 0xce8, queue);
        c.bus.write32(queue + 0x1c4, 0);
    }

    #[test]
    fn ov_screen_enter_lodge_dtor_matches() {
        check(
            ee::s05::f_00220eb8__generated,
            ov_screen_enter_lodge_dtor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_0eb8);
            },
        );
    }

    #[test]
    fn ov_screen_enter_lodge_ctor_matches() {
        check(
            ee::s05::f_00220ee0__generated,
            ov_screen_enter_lodge_ctor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_0ee0);
            },
        );
    }

    #[test]
    fn ov_screen_bc_start_ctor_matches() {
        check(
            ee::s05::f_00220f48__generated,
            ov_screen_bc_start_ctor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_0f48);
            },
        );
    }

    #[test]
    fn ov_screen_bc_start_ctor_2_matches() {
        check(
            ee::s05::f_00220fa8__generated,
            ov_screen_bc_start_ctor_2,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_0fa8);
                empty_pop_queue(c, DATA + 0x1000);
            },
        );
    }

    #[test]
    fn ov_screen_bc_start_dtor_matches() {
        check(
            ee::s05::f_00221018__generated,
            ov_screen_bc_start_dtor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_1018);
            },
        );
    }

    #[test]
    fn ov_screen_bc_fail_ctor_matches() {
        check(
            ee::s05::f_00221040__generated,
            ov_screen_bc_fail_ctor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_1040);
            },
        );
    }

    #[test]
    fn ov_screen_bc_fail_ctor_2_matches() {
        check(
            ee::s05::f_002210a0__generated,
            ov_screen_bc_fail_ctor_2,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_10a0);
                empty_pop_queue(c, DATA + 0x200);
            },
        );
    }

    /// Gate word in the test machine's RAM: the callee fakes answer only when it
    /// holds [`OC1004_MAGIC`] (otherwise they run the generated bodies).
    pub(crate) const OC1004_GATE: u32 = DATA + 0x000f_0000;

    pub(crate) const OC1004_MAGIC: u32 = 0x1004_c0de;

    /// The overlay object every test passes as `this`.
    pub(crate) const OC1004_THIS: u32 = DATA + 0x1000;

    pub(crate) static OC1004_PREV_BASE_CTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1004_PREV_BASE_DTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1004_PREV_EVENT_SELECT: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of the base constructor: logs the call and returns to `ra`.
    pub(crate) fn oc1004_fake_base_ctor(c: &mut Cpu) {
        if c.bus.read32(OC1004_GATE) != OC1004_MAGIC {
            return fakes::fall_back(c, &OC1004_PREV_BASE_CTOR, ee::s13::f_0039e2a0__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, OV_STATE_BASE_CTOR, ret);
    }

    /// Gated fake of the base destructor.
    pub(crate) fn oc1004_fake_base_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1004_GATE) != OC1004_MAGIC {
            return fakes::fall_back(c, &OC1004_PREV_BASE_DTOR, ee::s13::f_0039e390__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, OV_STATE_BASE_DTOR, ret);
    }

    /// Gated fake of `EventSelect_ctor`.
    pub(crate) fn oc1004_fake_event_select(c: &mut Cpu) {
        if c.bus.read32(OC1004_GATE) != OC1004_MAGIC {
            return fakes::fall_back(c, &OC1004_PREV_EVENT_SELECT, ee::s05::f_002006b8__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, OV_EVENT_SELECT_CTOR, ret);
    }

    /// Hook the three fakes into their override slots (once per process).
    pub(crate) fn oc1004_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc1004_fake_base_ctor,
            &OC1004_PREV_BASE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc1004_fake_base_dtor,
            &OC1004_PREV_BASE_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_002006B8),
            oc1004_fake_event_select,
            &OC1004_PREV_EVENT_SELECT,
        );
    }

    /// Scrambled registers, the callee fakes armed, the overlay object filled
    /// with random words and passed as `a0`. Every other seed makes an event
    /// due at the first loop back edge.
    pub(crate) fn oc1004_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1004_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC1004_GATE, OC1004_MAGIC);
        // The halfword the constructors copy from the image table at 0x4a2088.
        c.bus.write32(0x004a_2088, r.next_u32());
        fill(c, &mut r, OC1004_THIS, 0x40);
        c.g[A0][0] = sx(OC1004_THIS);
        if seed % 2 == 0 {
            c.next_event = c.cycles;
        }
        r
    }

    #[test]
    fn ov_screen_bc_fail_dtor_matches() {
        covered(
            ee::s05::f_00221110__generated,
            ov_screen_bc_fail_dtor,
            240,
            &|c, seed| {
                oc1004_setup(c, seed, 0x0022_1110, 1);
            },
        );
    }

    #[test]
    fn ov_screen_top_times_dtor_matches() {
        covered(
            ee::s05::f_00221138__generated,
            ov_screen_top_times_dtor,
            240,
            &|c, seed| {
                oc1004_setup(c, seed, 0x0022_1138, 1);
            },
        );
    }

    #[test]
    fn ov_screen_top_times_ctor_matches() {
        covered(
            ee::s05::f_00221160__generated,
            ov_screen_top_times_ctor,
            240,
            &|c, seed| {
                let mut r = oc1004_setup(c, seed, 0x0022_1160, 1);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }
}
