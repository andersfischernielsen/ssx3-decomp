//! `src/fe/ovstateenterlodge.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Flow;
use crate::call::Frame;
pub(crate) use crate::game::dirtysock::tags2::gp_8 as gp_2;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::festateloadhint::accept_key_6;
pub(crate) use crate::game::fe::festaterewards2::handle_list_key;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
use crate::guest::Addr;
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
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;

    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;

    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;

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
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_6;

    pub(crate) use crate::game::bx::bxstring2::tests::slot_2 as slot;

    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::put_text;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_7;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;

    pub(crate) use crate::game::fe::fememcard2::tests::hash_screen;

    pub(crate) use crate::game::fe::festateloadhint::cfe_state_progressive_verify_ps2_on_widget_notify;
    pub(crate) use crate::game::fe::festateoptions::tests::belib as belib_2;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_2;
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
}
