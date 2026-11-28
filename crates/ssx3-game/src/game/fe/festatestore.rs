//! `src/fe/festatestore.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::be::vt_call;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::dirtysock::tagsunk::at_3 as at;
pub(crate) use crate::game::dirtysock::tagsunk::gp_5 as gp;
pub(crate) use crate::game::fe::feasyncfile::p as p_2;
pub(crate) use crate::game::fe::fepopup::oc508_vtable_dtor;
pub(crate) use crate::game::fe::fepopupmisc::transition_in;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::fe::festatecredits::oc830_ea;
pub(crate) use crate::game::fe::festatecredits::oc830_sx_sub;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
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
        0x0018_4780,
        cfe_uber_trick_show_buy_buttons,
        "cFEStateUberTrick_ShowBuyButtons",
    ),
    (
        0x0018_4b70,
        fe_state_uber_trick_shutdown,
        "cFEStateUberTrick_Shutdown",
    ),
    (
        0x0018_4bb8,
        fe_state_uber_trick_transition_in,
        "cFEStateUberTrick_TransitionIn",
    ),
    (
        0x0018_4be0,
        c_fe_state_uber_trick_on_activate,
        "cFEStateUberTrick_OnActivate",
    ),
    (
        0x0018_59d8,
        fe_state_uber_trick_on_widget_notify,
        "cFEStateUberTrick_OnWidgetNotify",
    ),
    (
        0x0018_5a18,
        cfe_options_credits_ctor,
        "cFEOptionsCredits_ctor",
    ),
    (
        0x001d_d8f0,
        c_fe_state_uber_trick_dtor,
        "cFEStateUberTrick_dtor",
    ),
    (
        0x0018_4668,
        uber_trick_show_cost,
        "cFEStateUberTrick_ShowCost",
    ),
    (
        0x0018_4828,
        cfe_state_uber_trick_show_execute_info,
        "cFEStateUberTrick_ShowExecuteInfo",
    ),
    (
        0x0018_49b0,
        cfe_state_uber_trick_init,
        "cFEStateUberTrick_Init",
    ),
    (
        0x0018_4c60,
        cfe_state_uber_trick_update,
        "cFEStateUberTrick_Update",
    ),
    (
        0x0018_4f40,
        cfe_state_uber_trick_on_child_event,
        "cFEStateUberTrick_OnChildEvent",
    ),
    (
        0x0018_5080,
        cfe_state_uber_trick_on_widget_created,
        "cFEStateUberTrick_OnWidgetCreated",
    ),
    (
        0x0018_5a70,
        cfe_options_credits_dtor,
        "cFEOptionsCredits_dtor",
    ),
    (
        0x0018_5268,
        c_fe_state_uber_trick_on_widget_action,
        "cFEStateUberTrick_OnWidgetAction",
    ),
    (
        0x001d_d918,
        c_fe_state_uber_trick_v13_001dd918,
        "cFEStateUberTrick_v13",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::UBER_TRICK_REFRESH;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::FEMODULE_SET_PREVIEW_VISIBLE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_SHUTDOWN;
}

/// `bool cFEStateUberTrick_TransitionIn(cFEStateUberTrick* s, int on)`
/// (0x184bb8): refresh the trick list when entering, and report success.
pub fn fe_state_uber_trick_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        transition_in(c, 0x0018_4bcc, callee::UBER_TRICK_REFRESH)
    })
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

/// `cFEStateUberTrick::~cFEStateUberTrick(this)` (0x1dd8f0).
pub fn c_fe_state_uber_trick_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc508_vtable_dtor(c, sx(0x0047_0000 - 0x2f30), 0x0039_e390, 0x001d_d90c)
    })
}

/// `void cFEStateUberTrick_ShowCost(self, on)`: set four cost widgets of the screen (the first
/// screen of the state's list) to `on`.
/// (0x00184668; every register, cycle and call of the generated body is kept.)
pub fn uber_trick_show_cost(c: &mut Cpu) {
    guest(c, |c| {
        'b_1846c0: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00184684);
            c.g[S1][0] = c.g[A1][0];
            call_ptr(c, 0x00397840, 0x00184684)?;
            c.g[A0][0] = 0x460000;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184694);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2648));
            call_ptr(c, 0x00317670, 0x00184694)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001846a0);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x001846a0)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_1846c0;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001846c0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001846c0)?;
                break 'b_1846c0;
            }
        }
        'b_1846f8: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001846cc);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2628));
            call_ptr(c, 0x00317670, 0x001846cc)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001846d8);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x001846d8)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_1846f8;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001846f8);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001846f8)?;
                break 'b_1846f8;
            }
        }
        'b_184730: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00184704);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2638));
            call_ptr(c, 0x00317670, 0x00184704)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184710);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x00184710)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184730;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184730);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184730)?;
                break 'b_184730;
            }
        }
        'b_184768: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x0018473c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2618));
            call_ptr(c, 0x00317670, 0x0018473c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00184748);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00184748)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184768;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184768);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184768)?;
                break 'b_184768;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateUberTrick_ShowExecuteInfo(cFEStateUberTrick* self, int on)` (0x184828): show or hide the
/// six widgets of the left panel (`To_Execute_info`, `Checkboxes`, `MenuTrick`, `InstBtn0..2`), found by hash name
/// in the first screen of the state, with the widget's `setVisible(on)` virtual.
pub fn cfe_state_uber_trick_show_execute_info(c: &mut Cpu) {
    guest(c, |c| {
        'b_184880: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00184844);
            c.g[S1][0] = c.g[A1][0];
            call_ptr(c, 0x00397840, 0x00184844)?;
            c.g[A0][0] = 0x460000;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184854);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2608));
            call_ptr(c, 0x00317670, 0x00184854)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184860);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x00184860)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184880;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184880);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184880)?;
                break 'b_184880;
            }
        }
        'b_1848b8: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x0018488c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25f8));
            call_ptr(c, 0x00317670, 0x0018488c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184898);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x00184898)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_1848b8;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001848b8);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001848b8)?;
                break 'b_1848b8;
            }
        }
        'b_1848f0: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001848c4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25e8));
            call_ptr(c, 0x00317670, 0x001848c4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001848d0);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x001848d0)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_1848f0;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001848f0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001848f0)?;
                break 'b_1848f0;
            }
        }
        'b_184928: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001848fc);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26a0));
            call_ptr(c, 0x00317670, 0x001848fc)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184908);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x00184908)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184928;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184928);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184928)?;
                break 'b_184928;
            }
        }
        'b_184960: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00184934);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2690));
            call_ptr(c, 0x00317670, 0x00184934)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184940);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x00184940)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184960;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184960);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184960)?;
                break 'b_184960;
            }
        }
        'b_184998: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x0018496c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2680));
            call_ptr(c, 0x00317670, 0x0018496c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00184978);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00184978)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184998;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184998);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184998)?;
                break 'b_184998;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateUberTrick_Init(cFEStateUberTrick* self)` (0x1849b0): add the `btnmap` screen, set the rider's
/// character in the front-end rider manager and the preview camera, look up the checkbox, checkmark and dollar
/// textures, then hide cost and buy buttons.
pub fn cfe_state_uber_trick_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_184a04: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25d8));
            c.g[RA][0] = sx(0x001849d8);
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001849d8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001849ec);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001849ec)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_184a04;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x00184a04);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x00184a04)?;
                break 'b_184a04;
            }
        }
        c.cycles += 91;
        c.g[RA][0] = sx(0x00184a0c);
        call_ptr(c, 0x0014dc80, 0x00184a0c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00184a18);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x00184a18)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
        c.g[RA][0] = sx(0x00184a28);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00147488, 0x00184a28)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[S0][0] = sx((c.g[V0][0] as u32) << 24);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00184a44);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00184a44)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A3][0] = 0u64;
        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
        c.g[RA][0] = sx(0x00184a60);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x001a0508, 0x00184a60)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A2][0] = 0u64;
        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[RA][0] = sx(0x00184a78);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x001a0570, 0x00184a78)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x42a00000;
        c.mtc1(1, 2);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 1);
        c.g[AT][0] = 0x44250000;
        c.mtc1(1, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.sw(c.g[SP][0] as u32, 0u64);
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        c.g[RA][0] = sx(0x00184ac8);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
        call_ptr(c, 0x0015e050, 0x00184ac8)?;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x59f4));
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25c8));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x00184ae0);
        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x00317670, 0x00184ae0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[RA][0] = sx(0x00184aec);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x00398380, 0x00184aec)?;
        c.g[A0][0] = 0x460000;
        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x58), c.g[V0][0]);
        c.g[RA][0] = sx(0x00184b00);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25b8));
        call_ptr(c, 0x00317670, 0x00184b00)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[RA][0] = sx(0x00184b0c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x00398380, 0x00184b0c)?;
        c.g[A0][0] = 0x460000;
        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.g[RA][0] = sx(0x00184b20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25a8));
        call_ptr(c, 0x00317670, 0x00184b20)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[RA][0] = sx(0x00184b2c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x00398380, 0x00184b2c)?;
        c.g[RA][0] = sx(0x00184b34);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
        call_ptr(c, 0x0028b180, 0x00184b34)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00184b40);
        c.g[A1][0] = 0x5;
        call_ptr(c, 0x0028f140, 0x00184b40)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00184b4c);
        c.g[A1][0] = 0u64;
        Callee {
            addr: 0x00184668,
            f: ee::s02::f_00184668,
        }
        .call(c, 0x00184b4c)?; // cFEStateUberTrick_ShowCost
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00184b58);
        c.g[A1][0] = 0u64;
        Callee {
            addr: 0x00184780,
            f: ee::s02::f_00184780,
        }
        .call(c, 0x00184b58)?; // cFEStateUberTrick_ShowBuyButtons
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateUberTrick_Update(cFEStateUberTrick* self)` (0x184c60): the first update with an active rider
/// record (`+0x48` clear): place the three preview camera points (rotated by the rider's quaternion, via VU0) and
/// make the preview visible, then run the base state's update.
pub fn cfe_state_uber_trick_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_184f08: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[S7][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x48));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_184f08;
            } else {
                'b_184cd4: {
                    c.cycles += 9;
                    c.g[A1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                    c.g[RA][0] = sx(0x00184cac);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                    call_ptr(c, 0x001a0548, 0x00184cac)?;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0u64;
                    if t {
                        break 'b_184cd4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xcb8));
                        let t = c.g[V0][0] == 0u64;

                        if t {
                            break 'b_184cd4;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xcb4));
                            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                            break 'b_184cd4;
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[V1][0] == 0u64;
                c.g[FP][0] = 0x1;
                if t {
                    break 'b_184f08;
                } else {
                    'b_184d08: {
                        c.cycles += 3;
                        c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                        let t = c.g[S0][0] == 0u64;
                        c.sw((c.g[S6][0] as u32).wrapping_add(0xccc), c.g[FP][0]);
                        if t {
                            break 'b_184d08;
                        } else {
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x00184cf0);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00311a50, 0x00184cf0)?;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 12);
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0x1b4;
                            c.g[RA][0] = sx(0x00184d08);
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x003128e8, 0x00184d08)?;
                            break 'b_184d08;
                        }
                    }
                    c.cycles += 128;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[AT][0] = 0xffffffffc3640000;
                    c.mtc1(1, 0);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x74));
                    c.g[AT][0] = 0xffffffffc34d0000;
                    c.mtc1(1, 1);
                    c.g[S0][0] = 0x500000;
                    c.g[AT][0] = 0x427c0000;
                    c.mtc1(1, 2);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xec0));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = 0x500000;
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xeb0));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
                    c.g[S4][0] = 0x500000;
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
                    c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_sub(0xea0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x48), c.g[FP][0]);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x2c));
                    c.sq(2, c.g[SP][0] as u32);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                    c.g[RA][0] = sx(0x00184d80);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                    call_ptr(c, 0x0031be50, 0x00184d80)?;
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x78));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x7c));
                    c.lwc1(2, (c.g[S0][0] as u32).wrapping_sub(0xec0));
                    c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x4));
                    c.mul_s(1, 0, 1);
                    c.mul_s(2, 0, 2);
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
                    c.mul_s(0, 0, 4);
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x5c));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                    c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.mtc1(0, 12);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x00184dfc);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x0031be50, 0x00184dfc)?;
                    c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x78));
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x84));
                    c.lwc1(2, (c.g[S2][0] as u32).wrapping_sub(0xeb0));
                    c.lwc1(4, (c.g[S3][0] as u32).wrapping_add(0x4));
                    c.mul_s(1, 0, 1);
                    c.mul_s(2, 0, 2);
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x7c));
                    c.mul_s(0, 0, 4);
                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x59f0));
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4c));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                    c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x00184e78);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    call_ptr(c, 0x0031be50, 0x00184e78)?;
                    c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[S6][0];
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(2, (c.g[S4][0] as u32).wrapping_sub(0xea0));
                    c.lwc1(4, (c.g[S5][0] as u32).wrapping_add(0x4));
                    c.mul_s(1, 0, 1);
                    c.mul_s(2, 0, 2);
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.mul_s(0, 0, 4);
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4c));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                    c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x00184ef0);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    call_ptr(c, 0x0019f3e8, 0x00184ef0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A2][0] = 0x1;
                    c.g[A1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                    c.g[RA][0] = sx(0x00184f08);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                    call_ptr(c, 0x001a0570, 0x00184f08)?;
                    break 'b_184f08;
                }
            }
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x00184f10);
        c.g[A0][0] = c.g[S7][0];
        call_ptr(c, 0x0039e510, 0x00184f10)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateUberTrick_OnChildEvent(self, child, event)` (0x184f40): events 0x15 and 0x16 refresh the trick
/// list; 0x16 also re-applies the selected trick (rider preview, mute/trick screens) and the cost line.
pub fn cfe_state_uber_trick_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_185068: {
            'b_185064: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.g[V0][0] = 0x15;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S3][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[A2][0] == c.g[V0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_185064;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x16;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_185068;
                    } else {
                        'b_18505c: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_18505c;
                            } else {
                                'b_185054: {
                                    c.cycles += 48;
                                    c.g[RA][0] = sx(0x00184f88);
                                    call_ptr(c, 0x0014dc80, 0x00184f88)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00184f94);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x0014dd58, 0x00184f94)?;
                                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                                    c.g[RA][0] = sx(0x00184fa0);
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x00147488, 0x00184fa0)?;
                                    c.g[RA][0] = sx(0x00184fa8);
                                    c.g[S0][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dc80, 0x00184fa8)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00184fb4);
                                    c.g[A1][0] = 0xb;
                                    call_ptr(c, 0x0014dd58, 0x00184fb4)?;
                                    c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                    c.g[A2][0] = c.g[S0][0];
                                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                                    c.g[RA][0] = sx(0x00184fc8);
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x00150b48, 0x00184fc8)?;
                                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4c));
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x50));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                                    c.g[S1][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.g[RA][0] = sx(0x00184fe4);
                                    c.g[S2][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0014dc80, 0x00184fe4)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00184ff0);
                                    c.g[A1][0] = 0x6;
                                    call_ptr(c, 0x0014dd58, 0x00184ff0)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[A3][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x00185008);
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x0014fe08, 0x00185008)?;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x0018501c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018501c)?;
                                    c.g[A0][0] = 0x460000;
                                    c.g[RA][0] = sx(0x00185028);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26d0));
                                    call_ptr(c, 0x00317670, 0x00185028)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x00185034);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x00185034)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    let t = c.g[S0][0] == 0u64;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_185054;
                                    } else {
                                        c.cycles += 5;
                                        c.g[RA][0] = sx(0x00185048);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2590));
                                        call_ptr(c, 0x00317670, 0x00185048)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00185054);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003a1000, 0x00185054)?;
                                        break 'b_185054;
                                    }
                                }
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x0018505c);
                                c.g[A0][0] = c.g[S3][0];
                                Callee {
                                    addr: 0x00184520,
                                    f: ee::s02::f_00184520,
                                }
                                .call(c, 0x0018505c)?; // cFEStateUberTrick_UpdateCost
                                break 'b_18505c;
                            }
                        }
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x00185064);
                        c.g[A0][0] = c.g[S3][0];
                        Callee {
                            addr: 0x00183de0,
                            f: ee::s02::f_00183de0,
                        }
                        .call(c, 0x00185064)?; // cFEStateUberTrick_RefreshTrickList
                        break 'b_185064;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_185068;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateUberTrick_OnWidgetCreated(self, widget)` (0x185080): give the trick-name text widgets their
/// label and trick id (`mMiscData`), and remember the two menus.
pub fn cfe_state_uber_trick_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S2][0] = 0u64;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = c.g[A0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S4][0] = 0x4a0000;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S5][0] = 0x4a0000;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S6][0] = 0x4a0000;
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S7][0] = 0x460000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[FP][0] = 0x4a0000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V0][0] = 0x4a0000;
        'l_1850d0: loop {
            'b_185238: {
                'b_185224: {
                    'b_185208: {
                        'b_1851f0: {
                            'b_1851dc: {
                                'b_1851c4: {
                                    'b_1851ac: {
                                        'b_185194: {
                                            c.cycles += 5;
                                            c.g[S0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                            c.g[RA][0] = sx(0x001850dc);
                                            c.g[A0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x1468));
                                            call_ptr(c, 0x00317670, 0x001850dc)?;
                                            let t = c.g[S0][0] == c.g[V0][0];
                                            c.g[A1][0] = 0x4a0000;
                                            if t {
                                                break 'b_185194;
                                            } else {
                                                c.cycles += 5;
                                                c.g[S0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                c.g[RA][0] = sx(0x001850f0);
                                                c.g[A0][0] =
                                                    sx((c.g[FP][0] as u32).wrapping_add(0x1478));
                                                call_ptr(c, 0x00317670, 0x001850f0)?;
                                                let t = c.g[S0][0] == c.g[V0][0];
                                                c.g[A1][0] = 0x4a0000;
                                                if t {
                                                    break 'b_1851ac;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                    c.g[RA][0] = sx(0x00185104);
                                                    c.g[A0][0] =
                                                        sx((c.g[S7][0] as u32)
                                                            .wrapping_sub(0x2578));
                                                    call_ptr(c, 0x00317670, 0x00185104)?;
                                                    let t = c.g[S0][0] == c.g[V0][0];
                                                    c.g[A1][0] = 0x460000;
                                                    if t {
                                                        break 'b_1851c4;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[RA][0] = sx(0x00185118);
                                                        c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                            .wrapping_add(0x1488));
                                                        call_ptr(c, 0x00317670, 0x00185118)?;
                                                        let t = c.g[S0][0] == c.g[V0][0];
                                                        c.g[A1][0] = 0x4a0000;
                                                        if t {
                                                            break 'b_1851dc;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[S0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x38));
                                                            c.g[RA][0] = sx(0x0018512c);
                                                            c.g[A0][0] = sx((c.g[S5][0] as u32)
                                                                .wrapping_add(0x1498));
                                                            call_ptr(c, 0x00317670, 0x0018512c)?;
                                                            let t = c.g[S0][0] == c.g[V0][0];
                                                            c.g[A1][0] = 0x460000;
                                                            if t {
                                                                break 'b_1851f0;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[S0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x38));
                                                                c.g[RA][0] = sx(0x00185140);
                                                                c.g[A0][0] = sx((c.g[S4][0]
                                                                    as u32)
                                                                    .wrapping_add(0x14a0));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x00185140,
                                                                )?;
                                                                let t = c.g[S0][0] == c.g[V0][0];
                                                                c.g[S2][0] = sx((c.g[S2][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                if t {
                                                                    break 'b_185208;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = ((c.g[S2][0]
                                                                        as i64)
                                                                        < 6i64)
                                                                        as u64;
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[V0][0] = 0x4a0000;
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x001850d0,
                                                                            );
                                                                        }
                                                                        continue 'l_1850d0;
                                                                    } else {
                                                                        'b_185178: {
                                                                            c.cycles += 6;
                                                                            c.g[A0][0] = 0x4a0000;
                                                                            c.g[S0][0] = c.lw((c.g
                                                                                [S1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x38,
                                                                                ));
                                                                            c.g[RA][0] =
                                                                                sx(0x00185164);
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x14a8,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, 0x00317670,
                                                                                0x00185164,
                                                                            )?;
                                                                            if c.g[S0][0]
                                                                                != c.g[V0][0]
                                                                            {
                                                                                c.g[A0][0] =
                                                                                    0x460000;
                                                                                break 'b_185178;
                                                                            } else {
                                                                                c.cycles += 3;
                                                                                c.g[V0][0] = 0xffffffffffffffff;
                                                                                let t = true;
                                                                                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                                                                if t {
                                                                                    break 'b_185238;
                                                                                } else {
                                                                                    break 'b_185178;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x38));
                                                                        c.g[RA][0] = sx(0x00185184);
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x2538,
                                                                                ));
                                                                        call_ptr(
                                                                            c, 0x00317670,
                                                                            0x00185184,
                                                                        )?;
                                                                        if c.g[S0][0] != c.g[V0][0]
                                                                        {
                                                                            c.g[A0][0] = 0x460000;
                                                                            break 'b_185224;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = true;
                                                                            c.sw(
                                                                                (c.g[S3][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x4c,
                                                                                    ),
                                                                                c.g[S1][0],
                                                                            );
                                                                            if t {
                                                                                break 'b_185238;
                                                                            } else {
                                                                                break 'b_185194;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x001851a0);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1470));
                                        call_ptr(c, 0x003a0c50, 0x001851a0)?;
                                        c.g[V0][0] = 0x1;
                                        let t = true;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                        if t {
                                            break 'b_185238;
                                        } else {
                                            break 'b_1851ac;
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x001851b8);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1480));
                                    call_ptr(c, 0x003a0c50, 0x001851b8)?;
                                    c.g[V0][0] = 0x3;
                                    let t = true;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                    if t {
                                        break 'b_185238;
                                    } else {
                                        break 'b_1851c4;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001851d0);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2568));
                                call_ptr(c, 0x003a0c50, 0x001851d0)?;
                                c.g[V0][0] = 0x2;
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                if t {
                                    break 'b_185238;
                                } else {
                                    break 'b_1851dc;
                                }
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001851e8);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1490));
                            call_ptr(c, 0x003a0c50, 0x001851e8)?;
                            let t = true;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                            if t {
                                break 'b_185238;
                            } else {
                                break 'b_1851f0;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001851fc);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2558));
                        call_ptr(c, 0x003a0c50, 0x001851fc)?;
                        c.g[V0][0] = 0x4;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_185238;
                        } else {
                            break 'b_185208;
                        }
                    }
                    c.cycles += 7;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00185218);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2548));
                    call_ptr(c, 0x003a0c50, 0x00185218)?;
                    c.g[V0][0] = 0x9;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    if t {
                        break 'b_185238;
                    } else {
                        break 'b_185224;
                    }
                }
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x00185230);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25e8));
                call_ptr(c, 0x00317670, 0x00185230)?;
                if c.g[S0][0] == c.g[V0][0] {
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x50), c.g[S1][0]);
                    break 'b_185238;
                } else {
                    break 'b_185238;
                }
            }
            c.cycles += 12;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
            c.pc = t;
            return Ok(());
        }
    })
}

/// `cFEOptionsCredits::~cFEOptionsCredits` (0x185a70): restore the
/// `cFEState` vtable and run the `UIState` destructor.
pub fn cfe_options_credits_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x47_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = oc830_sx_sub(c, V0, 0x2f30);
        let a = oc830_ea(c, V1, 0x8);
        c.sw(a, c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0018_5a8c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateUberTrick_OnWidgetAction` (0x00185268): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_uber_trick_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x00185268 {
        return c_fe_state_uber_trick_on_widget_action_resume(c);
    }
    guest(c, |c| 'l_185268: loop {
        'b_1859b8: {
            'b_1859b4: {
                'b_1859b0: {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    if t {
                        break 'b_1859b0;
                    } else {
                        'b_185998: {
                            'b_1856f4: {
                                'b_185654: {
                                    'b_185618: {
                                        'b_185614: {
                                            'b_185558: {
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x460000;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x24a0));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x00185268 => {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00185268);
                                                        }
                                                        continue 'l_185268;
                                                    }
                                                    0x001852bc => {
                                                        'b_18534c: {
                                                            c.cycles += 4;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[A0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x4c));
                                                            if c.g[V1][0] != c.g[A0][0] {
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x50));
                                                                break 'b_18534c;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[A0][0] = 0x4a0000;
                                                                c.g[S0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x38));
                                                                c.g[RA][0] = sx(0x001852dc);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x14a8));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001852dc,
                                                                )?;
                                                                let t = c.g[S0][0] == c.g[V0][0];
                                                                c.g[A1][0] = 0u64;
                                                                if t {
                                                                    break 'b_185614;
                                                                } else {
                                                                    c.cycles += 26;
                                                                    c.g[A2][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x4c));
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A2][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    c.g[A0][0] = c
                                                                        .lh((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x38));
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x3c));
                                                                    let t = c.g[V1][0] as u32;
                                                                    c.g[RA][0] = sx(0x001852fc);
                                                                    c.g[A0][0] = sx((c.g[A2][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[A0][0] as u32,
                                                                        ));
                                                                    call_ptr(c, t, 0x001852fc)?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x50));
                                                                    c.g[RA][0] = sx(0x00185308);
                                                                    c.g[A1][0] = 0u64;
                                                                    call_ptr(
                                                                        c, 0x0039b960, 0x00185308,
                                                                    )?;
                                                                    c.g[A2][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x50));
                                                                    c.g[A1][0] = 0x1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A2][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    c.g[A0][0] = c
                                                                        .lh((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x38));
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x3c));
                                                                    let t = c.g[V1][0] as u32;
                                                                    c.g[RA][0] = sx(0x00185324);
                                                                    c.g[A0][0] = sx((c.g[A2][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[A0][0] as u32,
                                                                        ));
                                                                    call_ptr(c, t, 0x00185324)?;
                                                                    c.g[RA][0] = sx(0x0018532c);
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    Callee { addr: 0x00183c08, f: ee::s02::cFEStateUberTrick_SelectRightPanel }.call(c, 0x0018532c)?;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    c.g[RA][0] = sx(0x00185338);
                                                                    c.g[A1][0] = 0x1;
                                                                    Callee { addr: 0x00184668, f: ee::s02::cFEStateUberTrick_ShowCost }.call(c, 0x00185338)?;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    c.g[RA][0] = sx(0x00185344);
                                                                    c.g[A1][0] = 0x1;
                                                                    Callee { addr: 0x00184780, f: ee::s02::cFEStateUberTrick_ShowBuyButtons }.call(c, 0x00185344)?;
                                                                    let t = true;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x90),
                                                                    );
                                                                    if t {
                                                                        break 'b_1859b4;
                                                                    } else {
                                                                        break 'b_18534c;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] != c.g[V0][0];
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x90),
                                                        );
                                                        if t {
                                                            break 'b_1859b4;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A2][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xa0));
                                                            let t = c.g[A2][0] == 0u64;
                                                            c.g[V1][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xa0));
                                                            if t {
                                                                break 'b_1859b4;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = c.g[V1][0] == 0u64;
                                                                c.lq(
                                                                    17,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x80),
                                                                );
                                                                if t {
                                                                    break 'b_1859b8;
                                                                } else {
                                                                    'b_185404: {
                                                                        'b_185400: {
                                                                            c.cycles += 18;
                                                                            c.g[S4][0] = c.lb((c.g
                                                                                [A2][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x18,
                                                                                ));
                                                                            c.g[RA][0] =
                                                                                sx(0x00185374);
                                                                            c.g[S5][0] = c.lb((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x18,
                                                                                ));
                                                                            call_ptr(
                                                                                c, 0x0014dc80,
                                                                                0x00185374,
                                                                            )?;
                                                                            c.g[A0][0] = c.g[V0][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x00185380);
                                                                            c.g[A1][0] = 0x6;
                                                                            call_ptr(
                                                                                c, 0x0014dd58,
                                                                                0x00185380,
                                                                            )?;
                                                                            c.g[RA][0] =
                                                                                sx(0x00185388);
                                                                            c.g[S3][0] = c.g[V0][0];
                                                                            call_ptr(
                                                                                c, 0x0014dc80,
                                                                                0x00185388,
                                                                            )?;
                                                                            c.g[A0][0] = c.g[V0][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x00185394);
                                                                            c.g[A1][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x0014dd58,
                                                                                0x00185394,
                                                                            )?;
                                                                            c.g[A1][0] = c.lb((c.g
                                                                                [S2][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x44,
                                                                                ));
                                                                            c.g[A0][0] = c.g[S3][0];
                                                                            c.g[A2][0] = c.g[S4][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x001853a8);
                                                                            c.g[A3][0] = c.g[S5][0];
                                                                            call_ptr(
                                                                                c, 0x00150048,
                                                                                0x001853a8,
                                                                            )?;
                                                                            let t =
                                                                                c.g[V0][0] == 0u64;
                                                                            c.g[A0][0] = c.g[S3][0];
                                                                            if t {
                                                                                break 'b_185400;
                                                                            } else {
                                                                                c.cycles += 6;
                                                                                c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                                                                c.g[A2][0] =
                                                                                    c.g[S4][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x001853c0);
                                                                                c.g[A3][0] =
                                                                                    c.g[S5][0];
                                                                                call_ptr(
                                                                                    c, 0x0014ffb8,
                                                                                    0x001853c0,
                                                                                )?;
                                                                                let t = c.g[V0][0]
                                                                                    != 0u64;
                                                                                c.g[V0][0] =
                                                                                    0x530000;
                                                                                if t {
                                                                                    break 'b_185404;
                                                                                } else {
                                                                                    c.cycles += 14;
                                                                                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                                                                    c.g[A0][0] =
                                                                                        c.g[S3][0];
                                                                                    c.g[A2][0] =
                                                                                        c.g[S4][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001853dc,
                                                                                    );
                                                                                    c.g[A3][0] =
                                                                                        c.g[S5][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x0014fd80,
                                                                                        0x001853dc,
                                                                                    )?;
                                                                                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                                                                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                                                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        as u32;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001853f0,
                                                                                    );
                                                                                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                                                                    call_ptr(
                                                                                        c, t,
                                                                                        0x001853f0,
                                                                                    )?;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001853f8,
                                                                                    );
                                                                                    c.g[A0][0] =
                                                                                        c.g[S2][0];
                                                                                    Callee { addr: 0x00183de0, f: ee::s02::cFEStateUberTrick_RefreshTrickList }.call(c, 0x001853f8)?;
                                                                                    let t = true;
                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                                                    if t {
                                                                                        break 'b_1859b4;
                                                                                    } else {
                                                                                        break 'b_185400;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 1;
                                                                        c.g[V0][0] = 0x530000;
                                                                        break 'b_185404;
                                                                    }
                                                                    'b_185528: {
                                                                        c.cycles += 3;
                                                                        c.g[V1][0] = c.lb((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x5c11));
                                                                        if c.g[V1][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S2][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                            break 'b_185528;
                                                                        } else {
                                                                            c.cycles += 7;
                                                                            c.g[A1][0] = c.lb((c.g
                                                                                [S2][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x44,
                                                                                ));
                                                                            c.g[A0][0] = c.g[S3][0];
                                                                            c.g[A2][0] = c.g[S4][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x00185424);
                                                                            c.g[A3][0] = c.g[S5][0];
                                                                            call_ptr(
                                                                                c, 0x0014ffb8,
                                                                                0x00185424,
                                                                            )?;
                                                                            let t =
                                                                                c.g[V0][0] == 0u64;
                                                                            c.lq(
                                                                                16,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x90,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_1859b4;
                                                                            } else {
                                                                                c.cycles += 40;
                                                                                c.g[RA][0] =
                                                                                    sx(0x00185434);
                                                                                call_ptr(
                                                                                    c, 0x0014dc80,
                                                                                    0x00185434,
                                                                                )?;
                                                                                c.g[A0][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x00185440);
                                                                                c.g[A1][0] = 0x1;
                                                                                call_ptr(
                                                                                    c, 0x0014dd58,
                                                                                    0x00185440,
                                                                                )?;
                                                                                c.g[S0][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                                                                c.g[RA][0] =
                                                                                    sx(0x00185450);
                                                                                c.g[A0][0] =
                                                                                    c.g[S0][0];
                                                                                call_ptr(
                                                                                    c, 0x00147488,
                                                                                    0x00185450,
                                                                                )?;
                                                                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                c.g[S1][0] =
                                                                                    sx((c.g[V0][0]
                                                                                        as u32)
                                                                                        << 24);
                                                                                c.g[S1][0] = sx(
                                                                                    ((c.g[S1][0]
                                                                                        as u32)
                                                                                        as i32
                                                                                        >> 24)
                                                                                        as u32,
                                                                                );
                                                                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                                                                let t = c.g[V0][0]
                                                                                    as u32;
                                                                                c.g[RA][0] =
                                                                                    sx(0x0018546c);
                                                                                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                                                                call_ptr(
                                                                                    c, t,
                                                                                    0x0018546c,
                                                                                )?;
                                                                                c.g[RA][0] =
                                                                                    sx(0x00185474);
                                                                                call_ptr(
                                                                                    c, 0x0014dc80,
                                                                                    0x00185474,
                                                                                )?;
                                                                                c.g[A0][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x00185480);
                                                                                c.g[A1][0] = 0xb;
                                                                                call_ptr(
                                                                                    c, 0x0014dd58,
                                                                                    0x00185480,
                                                                                )?;
                                                                                c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                                                                c.g[A2][0] =
                                                                                    c.g[S1][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x00185490);
                                                                                c.g[A0][0] =
                                                                                    c.g[V0][0];
                                                                                call_ptr(
                                                                                    c, 0x00150928,
                                                                                    0x00185490,
                                                                                )?;
                                                                                c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                                                                c.g[A0][0] =
                                                                                    c.g[S3][0];
                                                                                c.g[S6][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[A2][0] =
                                                                                    c.g[S4][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x001854a8);
                                                                                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                                                                call_ptr(
                                                                                    c, 0x0014ff90,
                                                                                    0x001854a8,
                                                                                )?;
                                                                                c.g[V1][0] =
                                                                                    sx((c.g[S5][0]
                                                                                        as u32)
                                                                                        << 3);
                                                                                c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                c.g[RA][0] =
                                                                                    sx(0x001854b8);
                                                                                c.g[A0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x4));
                                                                                call_ptr(
                                                                                    c, 0x00116938,
                                                                                    0x001854b8,
                                                                                )?;
                                                                                c.g[S3][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x6));
                                                                                c.g[V0][0] = ((c.g
                                                                                    [S6][0]
                                                                                    as i64)
                                                                                    < (c.g[V0][0]
                                                                                        as i64))
                                                                                    as u64;
                                                                                if c.g[V0][0]
                                                                                    != 0u64
                                                                                {
                                                                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                                                    break 'b_185528;
                                                                                } else {
                                                                                    c.cycles += 23;
                                                                                    c.g[A1][0] =
                                                                                        0x460000;
                                                                                    c.g[A0][0] =
                                                                                        0x70;
                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2528));
                                                                                    c.g[A2][0] =
                                                                                        0x100;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001854e4,
                                                                                    );
                                                                                    c.g[A3][0] =
                                                                                        0u64;
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x00317d70,
                                                                                        0x001854e4,
                                                                                    )?;
                                                                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                                                    c.g[A0][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[A3][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001854f8,
                                                                                    );
                                                                                    c.g[A2][0] =
                                                                                        c.g[S2][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x001caba8,
                                                                                        0x001854f8,
                                                                                    )?;
                                                                                    c.g[S0][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[A2][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x6));
                                                                                    c.g[A0][0] =
                                                                                        c.g[S0][0];
                                                                                    c.g[A1][0] =
                                                                                        c.g[S3][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x00185510,
                                                                                    );
                                                                                    c.g[A3][0] =
                                                                                        c.g[S6][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x001caf58,
                                                                                        0x00185510,
                                                                                    )?;
                                                                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                                                    c.g[A1][0] =
                                                                                        c.g[S0][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x00185520,
                                                                                    );
                                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x0039f290,
                                                                                        0x00185520,
                                                                                    )?;
                                                                                    let t = true;
                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                                                    if t {
                                                                                        break 'b_1859b4;
                                                                                    } else {
                                                                                        break 'b_185528;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 5;
                                                                    c.g[V1][0] = sx((c.g[GP][0]
                                                                        as u32)
                                                                        .wrapping_add(0x2968));
                                                                    c.g[A2][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    if c.g[A2][0] == 0 {
                                                                        c.g[A2][0] = c.g[V1][0];
                                                                    }
                                                                    let t = c.g[A2][0] == 0u64;
                                                                    c.g[A1][0] = 0x4;
                                                                    if t {
                                                                        break 'b_1859b0;
                                                                    } else {
                                                                        c.cycles += 7;
                                                                        c.g[V0][0] =
                                                                            c.lw(c.g[A2][0] as u32);
                                                                        c.g[A0][0] = c.lh((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        c.g[V1][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        let t = c.g[V1][0] as u32;
                                                                        c.g[RA][0] = sx(0x00185550);
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A2][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[A0][0]
                                                                                        as u32,
                                                                                ));
                                                                        call_ptr(c, t, 0x00185550)?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x90),
                                                                        );
                                                                        if t {
                                                                            break 'b_1859b4;
                                                                        } else {
                                                                            break 'b_185558;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    0x00185558 => {
                                                        break 'b_185558;
                                                    }
                                                    0x00185654 => {
                                                        break 'b_185654;
                                                    }
                                                    0x001856f4 => {
                                                        break 'b_1856f4;
                                                    }
                                                    0x00185998 => {
                                                        break 'b_185998;
                                                    }
                                                    0x001859b0 => {
                                                        break 'b_1859b0;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x00185268, 0x001859d8);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                break 'b_185618;
                                            } else {
                                                'b_1855e4: {
                                                    c.cycles += 18;
                                                    c.g[A2][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x00185584);
                                                    c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x00185584)?;
                                                    c.g[A2][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                                    c.g[A1][0] = 0u64;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x38));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x001855a0);
                                                    c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001855a0)?;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                                                    if c.g[V0][0] == 0u64 {
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x50));
                                                        break 'b_1855e4;
                                                    } else {
                                                        c.cycles += 13;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 0);
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x58));
                                                        c.swc1(0, c.g[SP][0] as u32);
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x4),
                                                            0u64,
                                                        );
                                                        c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x8),
                                                            0u64,
                                                        );
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0xc),
                                                            0u64,
                                                        );
                                                        c.g[V0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x5c));
                                                        let t = c.g[V0][0] as u32;
                                                        c.g[RA][0] = sx(0x001855e0);
                                                        c.g[A1][0] = c.g[SP][0];
                                                        call_ptr(c, t, 0x001855e0)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x50));
                                                        break 'b_1855e4;
                                                    }
                                                }
                                                c.cycles += 12;
                                                c.g[RA][0] = sx(0x001855ec);
                                                c.g[A1][0] = 0xff;
                                                call_ptr(c, 0x0039b960, 0x001855ec)?;
                                                c.g[RA][0] = sx(0x001855f4);
                                                c.g[A0][0] = c.g[S2][0];
                                                Callee {
                                                    addr: 0x00183b08,
                                                    f: ee::s02::cFEStateUberTrick_SelectLeftPanel,
                                                }
                                                .call(c, 0x001855f4)?;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[RA][0] = sx(0x00185600);
                                                c.g[A1][0] = 0u64;
                                                Callee {
                                                    addr: 0x00184668,
                                                    f: ee::s02::cFEStateUberTrick_ShowCost,
                                                }
                                                .call(c, 0x00185600)?;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[RA][0] = sx(0x0018560c);
                                                c.g[A1][0] = 0u64;
                                                Callee {
                                                    addr: 0x00184780,
                                                    f: ee::s02::cFEStateUberTrick_ShowBuyButtons,
                                                }
                                                .call(c, 0x0018560c)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                if t {
                                                    break 'b_1859b4;
                                                } else {
                                                    break 'b_185614;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        break 'b_185618;
                                    }
                                    c.cycles += 10;
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00185638);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00185638)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_1859b0;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = sx(0x0018564c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f400, 0x0018564c)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                        if t {
                                            break 'b_1859b4;
                                        } else {
                                            break 'b_185654;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                let t = c.g[A1][0] != c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                if t {
                                    break 'b_1859b4;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                                    let t = c.g[S1][0] == 0u64;
                                    c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xa0));
                                    if t {
                                        break 'b_1859b0;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[S0][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                            break 'b_1859b4;
                                        } else {
                                            c.cycles += 16;
                                            c.g[RA][0] = sx(0x00185684);
                                            call_ptr(c, 0x0014dc80, 0x00185684)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x00185690);
                                            c.g[A1][0] = 0x6;
                                            call_ptr(c, 0x0014dd58, 0x00185690)?;
                                            c.g[S3][0] =
                                                c.lb((c.g[S0][0] as u32).wrapping_add(0x18));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[A2][0] =
                                                c.lb((c.g[S1][0] as u32).wrapping_add(0x18));
                                            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
                                            c.g[A1][0] =
                                                c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                            c.g[RA][0] = sx(0x001856ac);
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                                            call_ptr(c, 0x0014ff90, 0x001856ac)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x48));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            if t {
                                                break 'b_1859b0;
                                            } else {
                                                c.cycles += 14;
                                                c.g[A1][0] =
                                                    c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                                                c.g[RA][0] = sx(0x001856cc);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                                                call_ptr(c, 0x001a0548, 0x001856cc)?;
                                                c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                                c.g[AT][0] = 0xffffffffbf800000;
                                                c.mtc1(1, 12);
                                                c.g[A1][0] = c.lhu(c.g[V1][0] as u32);
                                                c.g[RA][0] = sx(0x001856ec);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x003128e8, 0x001856ec)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                if t {
                                                    break 'b_1859b4;
                                                } else {
                                                    break 'b_1856f4;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            'b_1857cc: {
                                c.cycles += 11;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                                c.g[RA][0] = sx(0x00185708);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                                call_ptr(c, 0x001a0548, 0x00185708)?;
                                c.g[V1][0] = 0x1;
                                c.sw((c.g[V0][0] as u32).wrapping_add(0xcd0), c.g[V1][0]);
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_1857cc;
                                } else {
                                    'b_1857b0: {
                                        'b_18577c: {
                                            'b_185778: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0xffffffffffffffff;
                                                if t {
                                                    break 'b_185778;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                    if c.g[V1][0] == c.g[V0][0] {
                                                        c.g[A1][0] = 0u64;
                                                        break 'b_18577c;
                                                    } else {
                                                        c.cycles += 14;
                                                        c.g[A1][0] = 0x1;
                                                        c.g[RA][0] = sx(0x00185740);
                                                        c.g[A0][0] = c.g[S2][0];
                                                        Callee { addr: 0x00184828, f: ee::s02::cFEStateUberTrick_ShowExecuteInfo }.call(c, 0x00185740)?;
                                                        c.g[RA][0] = sx(0x00185748);
                                                        c.g[A0][0] = c.g[S2][0];
                                                        Callee { addr: 0x00183de0, f: ee::s02::cFEStateUberTrick_RefreshTrickList }.call(c, 0x00185748)?;
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[RA][0] = sx(0x00185754);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x26e0));
                                                        call_ptr(c, 0x00317670, 0x00185754)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x00185760);
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0039d860, 0x00185760)?;
                                                        c.g[S0][0] = c.g[V0][0];
                                                        if c.g[S0][0] == 0u64 {
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x90),
                                                            );
                                                            break 'b_1859b4;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x460000;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x2518));
                                                            if t {
                                                                break 'b_1857b0;
                                                            } else {
                                                                break 'b_185778;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A1][0] = 0u64;
                                            break 'b_18577c;
                                        }
                                        c.cycles += 11;
                                        c.g[RA][0] = sx(0x00185784);
                                        c.g[A0][0] = c.g[S2][0];
                                        Callee {
                                            addr: 0x00184828,
                                            f: ee::s02::cFEStateUberTrick_ShowExecuteInfo,
                                        }
                                        .call(c, 0x00185784)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x00185790);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26e0));
                                        call_ptr(c, 0x00317670, 0x00185790)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x0018579c);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0018579c)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        if c.g[S0][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                            break 'b_1859b4;
                                        } else {
                                            c.cycles += 2;
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x2500));
                                            break 'b_1857b0;
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x001857b8);
                                    call_ptr(c, 0x00317670, 0x001857b8)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x001857c4);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x001857c4)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                    if t {
                                        break 'b_1859b4;
                                    } else {
                                        break 'b_1857cc;
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                            if t {
                                break 'b_1859b4;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                                if t {
                                    break 'b_1859b8;
                                } else {
                                    c.cycles += 12;
                                    c.g[A0][0] = 0x460000;
                                    c.g[RA][0] = sx(0x001857e8);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26d0));
                                    call_ptr(c, 0x00317670, 0x001857e8)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001857f4);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001857f4)?;
                                    c.g[S6][0] = c.g[V0][0];
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                                    let t = c.g[A2][0] == 0u64;
                                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
                                    if t {
                                        break 'b_1859b0;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V1][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                            break 'b_1859b4;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[S6][0] == 0u64 {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                break 'b_1859b4;
                                            } else {
                                                'b_185988: {
                                                    'b_185974: {
                                                        'b_185958: {
                                                            'b_18588c: {
                                                                'b_185888: {
                                                                    c.cycles += 18;
                                                                    c.g[S4][0] = c
                                                                        .lb((c.g[A2][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    c.g[RA][0] = sx(0x00185828);
                                                                    c.g[S5][0] = c
                                                                        .lb((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    call_ptr(
                                                                        c, 0x0014dc80, 0x00185828,
                                                                    )?;
                                                                    c.g[A0][0] = c.g[V0][0];
                                                                    c.g[RA][0] = sx(0x00185834);
                                                                    c.g[A1][0] = 0x6;
                                                                    call_ptr(
                                                                        c, 0x0014dd58, 0x00185834,
                                                                    )?;
                                                                    c.g[RA][0] = sx(0x0018583c);
                                                                    c.g[S3][0] = c.g[V0][0];
                                                                    call_ptr(
                                                                        c, 0x0014dc80, 0x0018583c,
                                                                    )?;
                                                                    c.g[A0][0] = c.g[V0][0];
                                                                    c.g[RA][0] = sx(0x00185848);
                                                                    c.g[A1][0] = 0u64;
                                                                    call_ptr(
                                                                        c, 0x0014dd58, 0x00185848,
                                                                    )?;
                                                                    c.g[A1][0] = c
                                                                        .lb((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x44));
                                                                    c.g[A0][0] = c.g[S3][0];
                                                                    c.g[A2][0] = c.g[S4][0];
                                                                    c.g[RA][0] = sx(0x0018585c);
                                                                    c.g[A3][0] = c.g[S5][0];
                                                                    call_ptr(
                                                                        c, 0x00150048, 0x0018585c,
                                                                    )?;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[S3][0];
                                                                    if t {
                                                                        break 'b_185888;
                                                                    } else {
                                                                        c.cycles += 6;
                                                                        c.g[A1][0] = c.lb((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x44));
                                                                        c.g[A2][0] = c.g[S4][0];
                                                                        c.g[RA][0] = sx(0x00185874);
                                                                        c.g[A3][0] = c.g[S5][0];
                                                                        call_ptr(
                                                                            c, 0x0014ffb8,
                                                                            0x00185874,
                                                                        )?;
                                                                        let t = c.g[V0][0] != 0u64;
                                                                        c.g[V0][0] = 0x530000;
                                                                        if t {
                                                                            break 'b_18588c;
                                                                        } else {
                                                                            c.cycles += 3;
                                                                            c.g[A0][0] = 0x460000;
                                                                            let t = true;
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x2590,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_185958;
                                                                            } else {
                                                                                break 'b_185888;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V0][0] = 0x530000;
                                                                break 'b_18588c;
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V1][0] = c.lb((c.g[V0][0] as u32)
                                                                .wrapping_add(0x5c11));
                                                            let t = c.g[V1][0] != 0u64;
                                                            c.g[A0][0] = 0x460000;
                                                            if t {
                                                                break 'b_185974;
                                                            } else {
                                                                c.cycles += 7;
                                                                c.g[A1][0] = c
                                                                    .lb((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x44));
                                                                c.g[A0][0] = c.g[S3][0];
                                                                c.g[A2][0] = c.g[S4][0];
                                                                c.g[RA][0] = sx(0x001858ac);
                                                                c.g[A3][0] = c.g[S5][0];
                                                                call_ptr(
                                                                    c, 0x0014ffb8, 0x001858ac,
                                                                )?;
                                                                let t = c.g[V0][0] == 0u64;

                                                                if t {
                                                                    break 'b_185988;
                                                                } else {
                                                                    'b_185954: {
                                                                        c.cycles += 37;
                                                                        c.g[RA][0] = sx(0x001858bc);
                                                                        call_ptr(
                                                                            c, 0x0014dc80,
                                                                            0x001858bc,
                                                                        )?;
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        c.g[RA][0] = sx(0x001858c8);
                                                                        c.g[A1][0] = 0x1;
                                                                        call_ptr(
                                                                            c, 0x0014dd58,
                                                                            0x001858c8,
                                                                        )?;
                                                                        c.g[S0][0] = c.g[V0][0];
                                                                        c.g[A1][0] = c.lb((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x44));
                                                                        c.g[RA][0] = sx(0x001858d8);
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        call_ptr(
                                                                            c, 0x00147488,
                                                                            0x001858d8,
                                                                        )?;
                                                                        c.g[V1][0] = c
                                                                            .lw((c.g[S0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                        c.g[S1][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 24);
                                                                        c.g[S1][0] = sx(
                                                                            ((c.g[S1][0] as u32)
                                                                                as i32
                                                                                >> 24)
                                                                                as u32,
                                                                        );
                                                                        c.g[A0][0] = c.lh((c.g[V1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        c.g[V0][0] = c.lw((c.g[V1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        let t = c.g[V0][0] as u32;
                                                                        c.g[RA][0] = sx(0x001858f4);
                                                                        c.g[A0][0] =
                                                                            sx((c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[A0][0]
                                                                                        as u32,
                                                                                ));
                                                                        call_ptr(c, t, 0x001858f4)?;
                                                                        c.g[RA][0] = sx(0x001858fc);
                                                                        call_ptr(
                                                                            c, 0x0014dc80,
                                                                            0x001858fc,
                                                                        )?;
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        c.g[RA][0] = sx(0x00185908);
                                                                        c.g[A1][0] = 0xb;
                                                                        call_ptr(
                                                                            c, 0x0014dd58,
                                                                            0x00185908,
                                                                        )?;
                                                                        c.g[A1][0] = c.lb((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x44));
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        c.g[RA][0] = sx(0x00185918);
                                                                        c.g[A2][0] = c.g[S1][0];
                                                                        call_ptr(
                                                                            c, 0x00150928,
                                                                            0x00185918,
                                                                        )?;
                                                                        c.g[A1][0] = c.lb((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x44));
                                                                        c.g[S0][0] = c.g[V0][0];
                                                                        c.g[A0][0] = c.g[S3][0];
                                                                        c.g[A2][0] = c.g[S4][0];
                                                                        c.g[RA][0] = sx(0x00185930);
                                                                        c.g[A3][0] =
                                                                            sx((c.g[SP][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x18,
                                                                                ));
                                                                        call_ptr(
                                                                            c, 0x0014ff90,
                                                                            0x00185930,
                                                                        )?;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[S5][0] as u32)
                                                                                << 3);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[V0][0] = c.lhu(
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x6),
                                                                        );
                                                                        c.g[S0][0] = ((c.g[S0][0]
                                                                            as i64)
                                                                            < (c.g[V0][0] as i64))
                                                                            as u64;
                                                                        if c.g[S0][0] != 0u64 {
                                                                            c.g[A0][0] = 0x460000;
                                                                            break 'b_185954;
                                                                        } else {
                                                                            c.cycles += 3;
                                                                            c.g[A0][0] = 0x460000;
                                                                            let t = true;
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x24e8,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_185958;
                                                                            } else {
                                                                                break 'b_185954;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x24d0));
                                                                    break 'b_185958;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 7;
                                                        c.g[RA][0] = sx(0x00185960);
                                                        call_ptr(c, 0x00317670, 0x00185960)?;
                                                        c.g[A0][0] = c.g[S6][0];
                                                        c.g[RA][0] = sx(0x0018596c);
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(c, 0x003a1000, 0x0018596c)?;
                                                        let t = true;

                                                        if t {
                                                            break 'b_185988;
                                                        } else {
                                                            break 'b_185974;
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[RA][0] = sx(0x0018597c);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x24b8));
                                                    call_ptr(c, 0x00317670, 0x0018597c)?;
                                                    c.g[A0][0] = c.g[S6][0];
                                                    c.g[RA][0] = sx(0x00185988);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, 0x003a1000, 0x00185988)?;
                                                    break 'b_185988;
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x00185990);
                                                c.g[A0][0] = c.g[S2][0];
                                                Callee {
                                                    addr: 0x00184520,
                                                    f: ee::s02::cFEStateUberTrick_UpdateCost,
                                                }
                                                .call(c, 0x00185990)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                if t {
                                                    break 'b_1859b4;
                                                } else {
                                                    break 'b_185998;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                        c.g[RA][0] = sx(0x001859ac);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                        call_ptr(c, 0x001a0548, 0x001859ac)?;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0xcd0), 0u64);
                        break 'b_1859b0;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                break 'b_1859b4;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_1859b8;
        }
        c.cycles += 8;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `c_fe_state_uber_trick_on_widget_action` (converted from the `entries` body): any entry
/// other than 0x00185268, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn c_fe_state_uber_trick_on_widget_action_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00185268 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    if t {
                        pc = 0x001859b0;
                        continue;
                    }
                    pc = 0x001852a0;
                    continue;
                }
                0x001852a0 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x24a0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001852bc => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));

                    if c.g[V1][0] != c.g[A0][0] {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                        pc = 0x0018534c;
                        continue;
                    }
                    pc = 0x001852cc;
                    continue;
                }
                0x001852cc => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x4a0000;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001852dc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14a8));
                    call_ptr(c, 0x00317670, 0x001852dc)?;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00185614;
                        continue;
                    }
                    pc = 0x001852e4;
                    continue;
                }
                0x001852e4 => {
                    c.cycles += 26;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001852fc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001852fc)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = sx(0x00185308);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0039b960, 0x00185308)?;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00185324);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00185324)?;
                    c.g[RA][0] = sx(0x0018532c);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00183c08,
                        f: ee::s02::cFEStateUberTrick_SelectRightPanel,
                    }
                    .call(c, 0x0018532c)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00185338);
                    c.g[A1][0] = 0x1;
                    Callee {
                        addr: 0x00184668,
                        f: ee::s02::cFEStateUberTrick_ShowCost,
                    }
                    .call(c, 0x00185338)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00185344);
                    c.g[A1][0] = 0x1;
                    Callee {
                        addr: 0x00184780,
                        f: ee::s02::cFEStateUberTrick_ShowBuyButtons,
                    }
                    .call(c, 0x00185344)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x0018534c;
                    continue;
                }
                0x0018534c => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x00185354;
                    continue;
                }
                0x00185354 => {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa0));
                    let t = c.g[A2][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x00185360;
                    continue;
                }
                0x00185360 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if t {
                        pc = 0x001859b8;
                        continue;
                    }
                    pc = 0x00185368;
                    continue;
                }
                0x00185368 => {
                    c.cycles += 18;
                    c.g[S4][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.g[RA][0] = sx(0x00185374);
                    c.g[S5][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0014dc80, 0x00185374)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00185380);
                    c.g[A1][0] = 0x6;
                    call_ptr(c, 0x0014dd58, 0x00185380)?;
                    c.g[RA][0] = sx(0x00185388);
                    c.g[S3][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x00185388)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00185394);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00185394)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001853a8);
                    c.g[A3][0] = c.g[S5][0];
                    call_ptr(c, 0x00150048, 0x001853a8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        pc = 0x00185400;
                        continue;
                    }
                    pc = 0x001853b0;
                    continue;
                }
                0x001853b0 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A2][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001853c0);
                    c.g[A3][0] = c.g[S5][0];
                    call_ptr(c, 0x0014ffb8, 0x001853c0)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x530000;
                    if t {
                        pc = 0x00185404;
                        continue;
                    }
                    pc = 0x001853c8;
                    continue;
                }
                0x001853c8 => {
                    c.cycles += 14;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001853dc);
                    c.g[A3][0] = c.g[S5][0];
                    call_ptr(c, 0x0014fd80, 0x001853dc)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001853f0);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001853f0)?;
                    c.g[RA][0] = sx(0x001853f8);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00183de0,
                        f: ee::s02::cFEStateUberTrick_RefreshTrickList,
                    }
                    .call(c, 0x001853f8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x00185400;
                    continue;
                }
                0x00185400 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x530000;
                    pc = 0x00185404;
                    continue;
                }
                0x00185404 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));

                    if c.g[V1][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        pc = 0x00185528;
                        continue;
                    }
                    pc = 0x00185410;
                    continue;
                }
                0x00185410 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x00185424);
                    c.g[A3][0] = c.g[S5][0];
                    call_ptr(c, 0x0014ffb8, 0x00185424)?;
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x0018542c;
                    continue;
                }
                0x0018542c => {
                    c.cycles += 40;
                    c.g[RA][0] = sx(0x00185434);
                    call_ptr(c, 0x0014dc80, 0x00185434)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00185440);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x00185440)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x00185450);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00147488, 0x00185450)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0018546c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018546c)?;
                    c.g[RA][0] = sx(0x00185474);
                    call_ptr(c, 0x0014dc80, 0x00185474)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00185480);
                    c.g[A1][0] = 0xb;
                    call_ptr(c, 0x0014dd58, 0x00185480)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00185490);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00150928, 0x00185490)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S6][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001854a8);
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x0014ff90, 0x001854a8)?;
                    c.g[V1][0] = sx((c.g[S5][0] as u32) << 3);
                    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x001854b8);
                    c.g[A0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x00116938, 0x001854b8)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x6));
                    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        pc = 0x00185528;
                        continue;
                    }
                    pc = 0x001854cc;
                    continue;
                }
                0x001854cc => {
                    c.cycles += 23;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = 0x70;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2528));
                    c.g[A2][0] = 0x100;
                    c.g[RA][0] = sx(0x001854e4);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x001854e4)?;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001854f8);
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x001caba8, 0x001854f8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x6));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00185510);
                    c.g[A3][0] = c.g[S6][0];
                    call_ptr(c, 0x001caf58, 0x00185510)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00185520);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039f290, 0x00185520)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x00185528;
                    continue;
                }
                0x00185528 => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    if c.g[A2][0] == 0 {
                        c.g[A2][0] = c.g[V1][0];
                    }
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0x4;
                    if t {
                        pc = 0x001859b0;
                        continue;
                    }
                    pc = 0x0018553c;
                    continue;
                }
                0x0018553c => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00185550);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00185550)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x00185558;
                    continue;
                }
                0x00185558 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        pc = 0x00185618;
                        continue;
                    }
                    pc = 0x00185568;
                    continue;
                }
                0x00185568 => {
                    c.cycles += 18;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00185584);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00185584)?;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001855a0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001855a0)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));

                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                        pc = 0x001855e4;
                        continue;
                    }
                    pc = 0x001855b0;
                    continue;
                }
                0x001855b0 => {
                    c.cycles += 13;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                    c.swc1(0, c.g[SP][0] as u32);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001855e0);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, t, 0x001855e0)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    pc = 0x001855e4;
                    continue;
                }
                0x001855e4 => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x001855ec);
                    c.g[A1][0] = 0xff;
                    call_ptr(c, 0x0039b960, 0x001855ec)?;
                    c.g[RA][0] = sx(0x001855f4);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00183b08,
                        f: ee::s02::cFEStateUberTrick_SelectLeftPanel,
                    }
                    .call(c, 0x001855f4)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00185600);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x00184668,
                        f: ee::s02::cFEStateUberTrick_ShowCost,
                    }
                    .call(c, 0x00185600)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0018560c);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x00184780,
                        f: ee::s02::cFEStateUberTrick_ShowBuyButtons,
                    }
                    .call(c, 0x0018560c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x00185614;
                    continue;
                }
                0x00185614 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    pc = 0x00185618;
                    continue;
                }
                0x00185618 => {
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00185638);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00185638)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x001859b0;
                        continue;
                    }
                    pc = 0x00185640;
                    continue;
                }
                0x00185640 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0018564c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039f400, 0x0018564c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x00185654;
                    continue;
                }
                0x00185654 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x00185664;
                    continue;
                }
                0x00185664 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                    let t = c.g[S1][0] == 0u64;
                    c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x001859b0;
                        continue;
                    }
                    pc = 0x00185674;
                    continue;
                }
                0x00185674 => {
                    c.cycles += 2;

                    if c.g[S0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x0018567c;
                    continue;
                }
                0x0018567c => {
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x00185684);
                    call_ptr(c, 0x0014dc80, 0x00185684)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00185690);
                    c.g[A1][0] = 0x6;
                    call_ptr(c, 0x0014dd58, 0x00185690)?;
                    c.g[S3][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001856ac);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                    call_ptr(c, 0x0014ff90, 0x001856ac)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x48));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x001859b0;
                        continue;
                    }
                    pc = 0x001856bc;
                    continue;
                }
                0x001856bc => {
                    c.cycles += 14;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                    c.g[RA][0] = sx(0x001856cc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                    call_ptr(c, 0x001a0548, 0x001856cc)?;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 12);
                    c.g[A1][0] = c.lhu(c.g[V1][0] as u32);
                    c.g[RA][0] = sx(0x001856ec);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x003128e8, 0x001856ec)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x001856f4;
                    continue;
                }
                0x001856f4 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                    c.g[RA][0] = sx(0x00185708);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                    call_ptr(c, 0x001a0548, 0x00185708)?;
                    c.g[V1][0] = 0x1;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xcd0), c.g[V1][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    if t {
                        pc = 0x001857cc;
                        continue;
                    }
                    pc = 0x00185720;
                    continue;
                }
                0x00185720 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x00185778;
                        continue;
                    }
                    pc = 0x00185728;
                    continue;
                }
                0x00185728 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[A1][0] = 0u64;
                        pc = 0x0018577c;
                        continue;
                    }
                    pc = 0x00185734;
                    continue;
                }
                0x00185734 => {
                    c.cycles += 14;
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x00185740);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00184828,
                        f: ee::s02::cFEStateUberTrick_ShowExecuteInfo,
                    }
                    .call(c, 0x00185740)?;
                    c.g[RA][0] = sx(0x00185748);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00183de0,
                        f: ee::s02::cFEStateUberTrick_RefreshTrickList,
                    }
                    .call(c, 0x00185748)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00185754);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26e0));
                    call_ptr(c, 0x00317670, 0x00185754)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00185760);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00185760)?;
                    c.g[S0][0] = c.g[V0][0];

                    if c.g[S0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x0018576c;
                    continue;
                }
                0x0018576c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2518));
                    if t {
                        pc = 0x001857b0;
                        continue;
                    }
                    pc = 0x00185778;
                    continue;
                }
                0x00185778 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0u64;
                    pc = 0x0018577c;
                    continue;
                }
                0x0018577c => {
                    c.cycles += 11;
                    c.g[RA][0] = sx(0x00185784);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00184828,
                        f: ee::s02::cFEStateUberTrick_ShowExecuteInfo,
                    }
                    .call(c, 0x00185784)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00185790);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26e0));
                    call_ptr(c, 0x00317670, 0x00185790)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0018579c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0018579c)?;
                    c.g[S0][0] = c.g[V0][0];

                    if c.g[S0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x001857a8;
                    continue;
                }
                0x001857a8 => {
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2500));
                    pc = 0x001857b0;
                    continue;
                }
                0x001857b0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001857b8);
                    call_ptr(c, 0x00317670, 0x001857b8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001857c4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001857c4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x001857cc;
                    continue;
                }
                0x001857cc => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x001857d4;
                    continue;
                }
                0x001857d4 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if t {
                        pc = 0x001859b8;
                        continue;
                    }
                    pc = 0x001857dc;
                    continue;
                }
                0x001857dc => {
                    c.cycles += 12;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001857e8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26d0));
                    call_ptr(c, 0x00317670, 0x001857e8)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001857f4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001857f4)?;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                    let t = c.g[A2][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
                    if t {
                        pc = 0x001859b0;
                        continue;
                    }
                    pc = 0x0018580c;
                    continue;
                }
                0x0018580c => {
                    c.cycles += 2;

                    if c.g[V1][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x00185814;
                    continue;
                }
                0x00185814 => {
                    c.cycles += 2;

                    if c.g[S6][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x0018581c;
                    continue;
                }
                0x0018581c => {
                    c.cycles += 18;
                    c.g[S4][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.g[RA][0] = sx(0x00185828);
                    c.g[S5][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0014dc80, 0x00185828)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00185834);
                    c.g[A1][0] = 0x6;
                    call_ptr(c, 0x0014dd58, 0x00185834)?;
                    c.g[RA][0] = sx(0x0018583c);
                    c.g[S3][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x0018583c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00185848);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00185848)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0018585c);
                    c.g[A3][0] = c.g[S5][0];
                    call_ptr(c, 0x00150048, 0x0018585c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        pc = 0x00185888;
                        continue;
                    }
                    pc = 0x00185864;
                    continue;
                }
                0x00185864 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A2][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x00185874);
                    c.g[A3][0] = c.g[S5][0];
                    call_ptr(c, 0x0014ffb8, 0x00185874)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x530000;
                    if t {
                        pc = 0x0018588c;
                        continue;
                    }
                    pc = 0x0018587c;
                    continue;
                }
                0x0018587c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2590));
                    if t {
                        pc = 0x00185958;
                        continue;
                    }
                    pc = 0x00185888;
                    continue;
                }
                0x00185888 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x530000;
                    pc = 0x0018588c;
                    continue;
                }
                0x0018588c => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x00185974;
                        continue;
                    }
                    pc = 0x00185898;
                    continue;
                }
                0x00185898 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x001858ac);
                    c.g[A3][0] = c.g[S5][0];
                    call_ptr(c, 0x0014ffb8, 0x001858ac)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x00185988;
                        continue;
                    }
                    pc = 0x001858b4;
                    continue;
                }
                0x001858b4 => {
                    c.cycles += 37;
                    c.g[RA][0] = sx(0x001858bc);
                    call_ptr(c, 0x0014dc80, 0x001858bc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001858c8);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x001858c8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[RA][0] = sx(0x001858d8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00147488, 0x001858d8)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001858f4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001858f4)?;
                    c.g[RA][0] = sx(0x001858fc);
                    call_ptr(c, 0x0014dc80, 0x001858fc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00185908);
                    c.g[A1][0] = 0xb;
                    call_ptr(c, 0x0014dd58, 0x00185908)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00185918);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x00150928, 0x00185918)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x00185930);
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0014ff90, 0x00185930)?;
                    c.g[V1][0] = sx((c.g[S5][0] as u32) << 3);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x6));
                    c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = 0x460000;
                        pc = 0x00185954;
                        continue;
                    }
                    pc = 0x00185948;
                    continue;
                }
                0x00185948 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x24e8));
                    if t {
                        pc = 0x00185958;
                        continue;
                    }
                    pc = 0x00185954;
                    continue;
                }
                0x00185954 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x24d0));
                    pc = 0x00185958;
                    continue;
                }
                0x00185958 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00185960);
                    call_ptr(c, 0x00317670, 0x00185960)?;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0018596c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x0018596c)?;
                    let t = true;

                    if t {
                        pc = 0x00185988;
                        continue;
                    }
                    pc = 0x00185974;
                    continue;
                }
                0x00185974 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0018597c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x24b8));
                    call_ptr(c, 0x00317670, 0x0018597c)?;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00185988);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00185988)?;
                    pc = 0x00185988;
                    continue;
                }
                0x00185988 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00185990);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00184520,
                        f: ee::s02::cFEStateUberTrick_UpdateCost,
                    }
                    .call(c, 0x00185990)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001859b4;
                        continue;
                    }
                    pc = 0x00185998;
                    continue;
                }
                0x00185998 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                    c.g[RA][0] = sx(0x001859ac);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                    call_ptr(c, 0x001a0548, 0x001859ac)?;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xcd0), 0u64);
                    pc = 0x001859b0;
                    continue;
                }
                0x001859b0 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    pc = 0x001859b4;
                    continue;
                }
                0x001859b4 => {
                    c.cycles += 1;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    pc = 0x001859b8;
                    continue;
                }
                0x001859b8 => {
                    c.cycles += 8;
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00185268, 0x001859d8);
                    return Ok(());
                }
            }
        }
    });
}

/// `cFEStateUberTrick_v13` (0x001d_d918): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_uber_trick_v13_001dd918(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::festatecredits::tests::oc830_install_fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS;
    pub(crate) use crate::game::bx::bxstringctor::tests::be_library;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;

    use ssx3_ee as ee;

    #[test]
    fn fe_state_uber_trick_transition_in_matches() {
        crate::difftest::check(
            ee::s02::f_00184bb8__generated,
            fe_state_uber_trick_transition_in,
            40,
            &|c, seed| {
                machine(c, seed, 0x184b_b8, 2);
                be_library(c);
                // A zeroed state object: its trick lists are empty, so the
                // refresh walks terminate at once.
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
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
                let _r = machine(c, seed, 0x184b70, 0);
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
        check(
            ee::s02::f_001859d8__generated,
            fe_state_uber_trick_on_widget_notify,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x0018_59d8, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, DATA + 0x50, (seed % 2) as u32);
                c.g[A2][0] = (seed % 12) as u64;
            },
        );
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

    pub(crate) use crate::game::fe::fepopup::tests::oc508_setup;

    #[test]
    fn c_fe_state_uber_trick_dtor_matches() {
        check(
            ee::s04::f_001dd8f0__generated,
            c_fe_state_uber_trick_dtor,
            200,
            &|c, seed| {
                oc508_setup(c, seed, 0x1dd8f0);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::Rng;
    pub(crate) use crate::game::fe::festatecredits::tests::oc830_arm;

    pub(crate) use crate::game::fe::festatecredits::tests::OC830_OBJ;

    pub(crate) use crate::game::fe::festateriderbio::tests::oc827_base;

    pub(crate) use crate::game::fe::festateriderbio::tests::OC827_SCR;

    pub(crate) use crate::game::fe::festateriderbio::tests::OC827_THIS;

    use crate::game::main::loadscreens::tests::oc736_fake_call;

    use crate::game::main::loadscreens::tests::oc736_force;
    use crate::game::main::loadscreens::tests::oc736_force_zero;

    use std::sync::atomic::AtomicUsize;

    /// `cFEStateUberTrick_ShowCost`: the screen has the four widgets or none.
    #[test]
    fn uber_trick_show_cost_matches() {
        check(
            ee::s02::f_00184668__generated,
            uber_trick_show_cost,
            200,
            &|c, seed| {
                oc827_base(c, seed, 0x184668, 16);
                if seed % 3 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC827_SCR);
                }
                c.g[A0][0] = sx(OC827_THIS);
                c.g[A1][0] = sx(seed.wrapping_mul(0x9e37_79b1) & 3);
                c.pc = 0x0018_4668;
            },
        );
    }

    /// The fakes' gate word and its magic (both machines of a diff get their own).
    pub(crate) const OC829_CTRL: u32 = DATA + 0x2f_0000;

    pub(crate) const OC829_MAGIC: u32 = 0x4f43_3832;

    /// Hash table of the `GetHashValue32` fake: the result for the string at
    /// `a0` is the word at `OC829_HASH_TAB + (a0 & 0xffffc)` (0: a pointer).
    pub(crate) const OC829_HASH_TAB: u32 = DATA + 0x50_0000;

    pub(crate) const OC829_SELF: u32 = DATA + 0x21_0000;

    pub(crate) const OC829_WORLD: u32 = DATA + 0x22_0000;

    pub(crate) const OC829_RM: u32 = DATA + 0x23_0000;

    pub(crate) const OC829_PLAYER: u32 = DATA + 0x24_0000;

    pub(crate) const OC829_ENG: u32 = DATA + 0x25_0000;

    pub(crate) const OC829_IFACE: u32 = DATA + 0x26_0000;

    pub(crate) const OC829_IFVT: u32 = DATA + 0x27_0000;

    pub(crate) const OC829_WIDGET: u32 = DATA + 0x28_0000;

    pub(crate) const OC829_WVT: u32 = DATA + 0x29_0000;

    pub(crate) const OC829_CHILD: u32 = DATA + 0x2a_0000;

    pub(crate) const OC829_PANEL_A: u32 = DATA + 0x2b_0000;

    pub(crate) const OC829_PANEL_B: u32 = DATA + 0x2c_0000;

    pub(crate) const OC829_REC_A: u32 = DATA + 0x2d_0000;

    pub(crate) const OC829_REC_B: u32 = DATA + 0x2e_0000;

    pub(crate) const OC829_SCREEN: u32 = DATA + 0x2e_8000;

    macro_rules! oc829_fakes {
        ($install:ident; $(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC829_CTRL) != OC829_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc736_fake_call(c, $addr);
                }
            )*
            fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc829_fakes!(
        oc829_install_generic;
        (oc829_fake_397840, OC829_PREV_397840, ee::s13::OVERRIDE_00397840, ee::s13::f_00397840__generated, 0x0039_7840),
        (oc829_fake_39d860, OC829_PREV_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860),
        (oc829_fake_397cc0, OC829_PREV_397CC0, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated, 0x0039_7cc0),
        (oc829_fake_39c870, OC829_PREV_39C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039_c870),
        (oc829_fake_398380, OC829_PREV_398380, ee::s13::OVERRIDE_00398380, ee::s13::f_00398380__generated, 0x0039_8380),
        (oc829_fake_39e510, OC829_PREV_39E510, ee::s13::OVERRIDE_0039E510, ee::s13::f_0039e510__generated, 0x0039_e510),
        (oc829_fake_3a1000, OC829_PREV_3A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a_1000),
        (oc829_fake_3a0c50, OC829_PREV_3A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50),
        (oc829_fake_311a50, OC829_PREV_311A50, ee::s10::OVERRIDE_00311A50, ee::s10::f_00311a50__generated, 0x0031_1a50),
        (oc829_fake_3128e8, OC829_PREV_3128E8, ee::s10::OVERRIDE_003128E8, ee::s10::f_003128e8__generated, 0x0031_28e8),
        (oc829_fake_31be50, OC829_PREV_31BE50, ee::s10::OVERRIDE_0031BE50, ee::s10::f_0031be50__generated, 0x0031_be50),
        (oc829_fake_14dc80, OC829_PREV_14DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, 0x0014_dc80),
        (oc829_fake_14dd58, OC829_PREV_14DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, 0x0014_dd58),
        (oc829_fake_147488, OC829_PREV_147488, ee::s01::OVERRIDE_00147488, ee::s01::f_00147488__generated, 0x0014_7488),
        (oc829_fake_15e050, OC829_PREV_15E050, ee::s01::OVERRIDE_0015E050, ee::s01::f_0015e050__generated, 0x0015_e050),
        (oc829_fake_150b48, OC829_PREV_150B48, ee::s01::OVERRIDE_00150B48, ee::s01::f_00150b48__generated, 0x0015_0b48),
        (oc829_fake_14fe08, OC829_PREV_14FE08, ee::s01::OVERRIDE_0014FE08, ee::s01::f_0014fe08__generated, 0x0014_fe08),
        (oc829_fake_1a0508, OC829_PREV_1A0508, ee::s03::OVERRIDE_001A0508, ee::s03::f_001a0508__generated, 0x001a_0508),
        (oc829_fake_1a0570, OC829_PREV_1A0570, ee::s03::OVERRIDE_001A0570, ee::s03::f_001a0570__generated, 0x001a_0570),
        (oc829_fake_19f3e8, OC829_PREV_19F3E8, ee::s03::OVERRIDE_0019F3E8, ee::s03::f_0019f3e8__generated, 0x0019_f3e8),
        (oc829_fake_1a0548, OC829_PREV_1A0548, ee::s03::OVERRIDE_001A0548, ee::s03::f_001a0548__generated, 0x001a_0548),
        (oc829_fake_28b180, OC829_PREV_28B180, ee::s07::OVERRIDE_0028B180, ee::s07::f_0028b180__generated, 0x0028_b180),
        (oc829_fake_28f140, OC829_PREV_28F140, ee::s07::OVERRIDE_0028F140, ee::s07::f_0028f140__generated, 0x0028_f140),
        (oc829_fake_183de0, OC829_PREV_183DE0, ee::s02::OVERRIDE_00183DE0, ee::s02::f_00183de0__generated, 0x0018_3de0),
        (oc829_fake_184520, OC829_PREV_184520, ee::s02::OVERRIDE_00184520, ee::s02::f_00184520__generated, 0x0018_4520),
    );

    pub(crate) static OC829_PREV_317670: AtomicUsize = AtomicUsize::new(0);

    /// `GetHashValue32`: the hash table's value for the string, else the fake's pointer.
    pub(crate) fn oc829_fake_317670(c: &mut Cpu) {
        if c.bus.read32(OC829_CTRL) != OC829_MAGIC {
            return fakes::fall_back(c, &OC829_PREV_317670, ee::s10::f_00317670__generated);
        }
        let a0 = c.g[A0][0] as u32;
        let ra = c.g[RA][0] as u32;
        oc736_fake_call(c, 0x0031_7670);
        if c.pc == ra {
            let h = c.bus.read32(OC829_HASH_TAB + (a0 & 0xf_fffc));
            if h != 0 {
                c.g[V0][0] = h as u64;
            }
        }
    }

    pub(crate) fn oc829_install_fakes() {
        oc829_install_generic();
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc829_fake_317670,
            &OC829_PREV_317670,
        );
    }

    /// Scrambled machine with the fakes' gate open and the world (`gp-0x848`)
    /// with its rider manager (`+0x7c`).
    pub(crate) fn oc829_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC829_CTRL, OC829_MAGIC);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC829_WORLD);
        c.bus.write32(OC829_WORLD + 0x7c, OC829_RM);
        r
    }

    /// A widget whose virtual `setVisible` slot (`+0x48` delta, `+0x4c` function) is a fake call.
    pub(crate) fn oc829_widget(c: &mut Cpu, seed: u32) {
        c.bus.write32(OC829_WIDGET + 8, OC829_WVT);
        c.bus.write16(OC829_WVT + 0x48, 0x10 * (seed % 3) as u16);
        c.bus.write32(OC829_WVT + 0x4c, FAKE);
    }

    /// The back-end interface the fakes of `0x14dd58` return: vtable at `+0xc`
    /// whose slots `+0x8/+0xc` and `+0x10/+0x14` are fake calls.
    pub(crate) fn oc829_iface(c: &mut Cpu, seed: u32) {
        oc736_force(c, 0x0014_dd58, OC829_IFACE);
        c.bus.write32(OC829_IFACE + 0xc, OC829_IFVT);
        for off in [0x8u32, 0x10] {
            c.bus.write16(OC829_IFVT + off, 0x10 * (seed % 3) as u16);
            c.bus.write32(OC829_IFVT + off + 4, FAKE + 0x10);
        }
    }

    /// `ShowExecuteInfo(self, on)`: the six widgets of the first screen, all
    /// missing (seed % 4 == 0) or present with a `setVisible` fake call.
    #[test]
    fn cfe_state_uber_trick_show_execute_info_matches() {
        oc829_install_fakes();
        check(
            ee::s02::f_00184828__generated,
            cfe_state_uber_trick_show_execute_info,
            60,
            &|c, seed| {
                let _ = oc829_fixture(c, seed, 0x0018_4828, 14);
                oc829_widget(c, seed);
                if seed % 4 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC829_WIDGET);
                }
                c.g[A0][0] = sx(OC829_SELF);
                c.g[A1][0] = (seed % 3) as u64;
                c.pc = 0x0018_4828;
            },
        );
    }

    /// `Init(self)`: the `btnmap` screen (missing or present), the rider's
    /// character, the camera set-up, the three texture lookups and the real
    /// `ShowCost`/`ShowBuyButtons` hiding their widgets.
    #[test]
    fn cfe_state_uber_trick_init_matches() {
        oc829_install_fakes();
        check(
            ee::s02::f_001849b0__generated,
            cfe_state_uber_trick_init,
            60,
            &|c, seed| {
                let _ = oc829_fixture(c, seed, 0x0018_49b0, 30);
                oc829_widget(c, seed);
                oc829_iface(c, seed);
                if seed % 3 == 0 {
                    oc736_force_zero(c, 0x0039_7cc0);
                } else {
                    oc736_force(c, 0x0039_7cc0, OC829_SCREEN);
                }
                if seed % 2 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC829_WIDGET);
                }
                c.bus.write32(OC829_SELF + 0x10, OC829_ENG);
                c.bus.write8(OC829_SELF + 0x44, (seed % 4) as u8);
                c.g[A0][0] = sx(OC829_SELF);
                c.pc = 0x0018_49b0;
            },
        );
    }

    /// `Update(self)`: the first update (`+0x48` clear) with a rider record
    /// that is, or is not, ready (`+0` below 10, `+0xcb8` set, `+0xcb4` > 0),
    /// with or without a camera object (`+0xc`); then the base update.
    #[test]
    fn cfe_state_uber_trick_update_matches() {
        oc829_install_fakes();
        check(
            ee::s02::f_00184c60__generated,
            cfe_state_uber_trick_update,
            160,
            &|c, seed| {
                let _ = oc829_fixture(c, seed, 0x0018_4c60, 16);
                oc736_force(c, 0x001a_0548, OC829_PLAYER);
                c.bus
                    .write32(OC829_SELF + 0x48, if (seed / 32) % 3 == 0 { 1 } else { 0 });
                c.bus.write8(OC829_SELF + 0x44, (seed % 4) as u8);
                c.bus
                    .write32(OC829_PLAYER, if seed % 2 == 0 { 3 } else { 20 });
                c.bus.write32(OC829_PLAYER + 0xcb8, (seed / 2) % 2);
                let cb4 = match (seed / 4) % 3 {
                    0 => 0,
                    1 => 5,
                    _ => u32::MAX,
                };
                c.bus.write32(OC829_PLAYER + 0xcb4, cb4);
                c.bus.write32(
                    OC829_PLAYER + 0xc,
                    if (seed / 16) % 2 == 0 { 0 } else { OC829_ENG },
                );
                c.g[A0][0] = sx(OC829_SELF);
                c.pc = 0x0018_4c60;
            },
        );
    }

    /// `OnChildEvent(self, child, event)`: events 0x15, 0x16 (with the child's
    /// `+0x6c` set or clear, the `btnmap` widget present or missing) and one
    /// other event.
    #[test]
    fn cfe_state_uber_trick_on_child_event_matches() {
        oc829_install_fakes();
        check(
            ee::s02::f_00184f40__generated,
            cfe_state_uber_trick_on_child_event,
            64,
            &|c, seed| {
                let _ = oc829_fixture(c, seed, 0x0018_4f40, 30);
                oc829_iface(c, seed);
                oc829_widget(c, seed);
                if (seed / 8) % 2 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC829_WIDGET);
                }
                c.bus.write32(OC829_SELF + 0x40, OC829_SCREEN);
                c.bus.write8(OC829_SELF + 0x44, (seed % 4) as u8);
                c.bus.write32(OC829_SELF + 0x4c, OC829_PANEL_A);
                c.bus.write32(OC829_SELF + 0x50, OC829_PANEL_B);
                c.bus.write32(OC829_PANEL_A + 0xa0, OC829_REC_A);
                c.bus.write32(OC829_PANEL_B + 0xa0, OC829_REC_B);
                c.bus.write8(OC829_REC_A + 0x18, (seed % 7) as u8);
                c.bus.write8(OC829_REC_B + 0x18, (seed % 5) as u8);
                c.bus.write32(OC829_CHILD + 0x48, 0x1000 + seed);
                c.bus.write32(OC829_CHILD + 0x6c, (seed / 4) % 2);
                c.g[A0][0] = sx(OC829_SELF);
                c.g[A1][0] = sx(OC829_CHILD);
                c.g[A2][0] = [0x15u64, 0x16, 0x17, 0x16][(seed % 4) as usize];
                c.pc = 0x0018_4f40;
            },
        );
    }

    /// `OnWidgetCreated(self, widget)`: the widget's hash name matches one of
    /// the nine names (six trick labels, `done`, `MenuMain`, `MenuTrick`) or
    /// none. Each name's string address hashes to its own value.
    #[test]
    fn cfe_state_uber_trick_on_widget_created_matches() {
        oc829_install_fakes();
        const NAMES: [u32; 9] = [
            0x004a_1468,
            0x004a_1478,
            0x0045_da88,
            0x004a_1488,
            0x004a_1498,
            0x004a_14a0,
            0x004a_14a8,
            0x0045_dac8,
            0x0045_da18,
        ];
        check(
            ee::s02::f_00185080__generated,
            cfe_state_uber_trick_on_widget_created,
            33,
            &|c, seed| {
                let _ = oc829_fixture(c, seed, 0x0018_5080, 20);
                for (i, s) in NAMES.into_iter().enumerate() {
                    c.bus
                        .write32(OC829_HASH_TAB + (s & 0xf_fffc), 0x1000 + 0x10 * i as u32);
                }
                let k = (seed % 11) as usize;
                let name = if k < 9 {
                    0x1000 + 0x10 * k as u32
                } else {
                    0x7777
                };
                c.bus.write32(OC829_WIDGET + 0x38, name);
                c.g[A0][0] = sx(OC829_SELF);
                c.g[A1][0] = sx(OC829_WIDGET);
                c.pc = 0x0018_5080;
            },
        );
    }

    #[test]
    fn cfe_options_credits_dtor_matches() {
        oc830_install_fakes();
        check(
            ee::s02::f_00185a70__generated,
            cfe_options_credits_dtor,
            12,
            &|c, seed| {
                machine(c, seed, 0x18_5a70, 1);
                oc830_arm(c);
                c.pc = 0x0018_5a70;
                c.g[A0][0] = sx(OC830_OBJ);
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::fakes::LOG_COUNT;
    use crate::difftest::fakes::LOG_START;
    use crate::difftest::fill;
    use crate::difftest::fresh;
    use crate::difftest::run_generated;

    pub(crate) const UBER_SELF: u32 = DATA + 0x1000;

    pub(crate) const UBER_ITEM: u32 = DATA + 0x3000;

    /// Fixture: `msg` cycles 0..=8 (0 and 8 are out of the jump table range),
    /// the shop state and the item are random words with the pointer fields
    /// (main menu, trick menu, parent) pointing at objects in the data area;
    /// even seeds make the item's parent the main menu.
    pub(crate) fn uber_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine_rt(c, seed, 0x1855_268, 12);
        elf_image(c);
        fill(c, &mut r, DATA, 0x2000);
        c.bus.write32(UBER_SELF + 0x4c, DATA + 0x5000);
        c.bus.write32(UBER_SELF + 0x50, DATA + 0x5800);
        let parent = match seed % 3 {
            0 => DATA + 0x5000,
            1 => DATA + 0x5800,
            _ => DATA + 0x6000,
        };
        c.bus.write32(UBER_ITEM + 0x40, parent);
        c.bus
            .write32(UBER_ITEM + 0x38, 0x0080_0000 + (seed % 5) * 0x10);
        c.g[A0][0] = sx(UBER_SELF);
        c.g[A1][0] = sx(UBER_ITEM);
        c.g[A2][0] = (seed % 9) as u64;
        c.pc = 0x0018_5268;
    }

    /// Port against the generated body (fake callees, unwinds included).
    #[test]
    fn c_fe_state_uber_trick_on_widget_action_matches() {
        covered(
            ee::s02::f_00185268__generated,
            c_fe_state_uber_trick_on_widget_action,
            400,
            &uber_setup,
        );
    }

    /// The fixture reaches several distinct call sequences.
    #[test]
    fn c_fe_state_uber_trick_on_widget_action_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..400u32 {
            let mut c = fresh();
            uber_setup(&mut c, seed);
            run_generated(&mut c, ee::s02::f_00185268__generated);
            let n = c.bus.read32(LOG_COUNT).min(64);
            let seq: Vec<u32> = (0..n)
                .map(|i| c.bus.read32(LOG_START + 0x220 * i))
                .collect();
            seen.insert(seq);
        }
        assert!(
            seen.len() >= 5,
            "only {} distinct call sequences",
            seen.len()
        );
    }

    /// `cFEStateUberTrick_v13` (0x001d_d918) against the generated body.
    #[test]
    fn c_fe_state_uber_trick_v13_001dd918_matches() {
        check(
            ee::s04::f_001dd918__generated,
            c_fe_state_uber_trick_v13_001dd918,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1dd918, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
