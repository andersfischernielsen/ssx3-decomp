//! `src/fe/festatebuyattrib.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::fepopupmisc::transition_in;
pub(crate) use crate::game::fe::festatetrophyroom::handle_key_6_8_9;
pub(crate) use crate::game::fe::festatetrophyroom::handle_list_key;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::main::ssxappdtor::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001f_4830,
        cfestatebuyattrib_init,
        "cFEStateBuyAttrib_Init",
    ),
    (
        0x001f_4910,
        fe_state_buy_attrib_on_widget_created,
        "cFEStateBuyAttrib_OnWidgetCreated",
    ),
    (
        0x001f_49d0,
        fe_state_buy_attrib_transition_in,
        "cFEStateBuyAttrib_TransitionIn",
    ),
    (
        0x001f_49f8,
        c_fe_state_buy_attrib_on_activate,
        "cFEStateBuyAttrib_OnActivate",
    ),
    (
        0x001f_4a38,
        cfe_state_buy_attrib_on_widget_notify,
        "cFEStateBuyAttrib_OnWidgetNotify",
    ),
    (
        0x001f_4a60,
        c_fe_state_buy_attrib_on_child_event,
        "cFEStateBuyAttrib_OnChildEvent",
    ),
    (
        0x001f_54b0,
        c_fe_state_career_stats_ctor,
        "cFEStateCareerStats_ctor",
    ),
    (
        0x001f_55c0,
        fe_state_career_stats_transition_in,
        "cFEStateCareerStats_TransitionIn",
    ),
    (
        0x001f_55e8,
        c_fe_state_career_stats_on_activate,
        "cFEStateCareerStats_OnActivate",
    ),
    (
        0x001f_5618,
        cfe_state_career_stats_on_widget_notify,
        "cFEStateCareerStats_OnWidgetNotify",
    ),
    (
        0x001f_5650,
        c_fe_state_career_stats_on_widget_action,
        "cFEStateCareerStats_OnWidgetAction",
    ),
    (
        0x001f_6840,
        cfe_state_career_stats_set_text,
        "cFEStateCareerStats_SetText",
    ),
    (
        0x001f_4a90,
        oc970_on_widget_action,
        "cFEStateBuyAttrib_OnWidgetAction",
    ),
    (
        0x001f_4c30,
        cfe_state_buy_attrib_post_input,
        "cFEStateBuyAttrib_PostInput",
    ),
    (
        0x001f_4ea8,
        oc970_update_level_text,
        "cFEStateBuyAttrib_UpdateLevelText",
    ),
    (
        0x001f_4fc0,
        oc970_update_cost_text,
        "cFEStateBuyAttrib_UpdateCostText",
    ),
    (
        0x001f_50b0,
        oc970_update_bars,
        "cFEStateBuyAttrib_UpdateBars",
    ),
    (
        0x001f_5240,
        oc970_update_total_cost,
        "cFEStateBuyAttrib_UpdateTotalCost",
    ),
];

// ---------------------------------------------------------------------------
// Callees (same-shard direct calls; cross-shard ones stay `call_ptr`s).

pub(crate) mod callee {
    pub(crate) use crate::game::be::beintmission::callee::BUY_ATTRIB_UPDATE_BARS;
    pub(crate) use crate::game::be::beintmission::callee::CAREER_STATS_SHOW_HIGHLIGHTS;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::BUY_ATTRIB_COMMIT_PURCHASE;
    pub(crate) use crate::game::main::ssxappdtor::callee::CAREER_STATS_SELECT_PAGE;
    pub(crate) use crate::game::main::ssxappdtor::callee::FE_STATE_ON_ACTIVATE;
}

/// `bool cFEStateBuyAttrib_TransitionIn(cFEStateBuyAttrib* s, int on)`
/// (0x1f49d0).
pub fn fe_state_buy_attrib_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        transition_in(c, 0x001f_49e4, callee::BUY_ATTRIB_UPDATE_BARS)
    })
}

/// `bool cFEStateCareerStats_TransitionIn(cFEStateCareerStats* s, int on)`
/// (0x1f55c0).
pub fn fe_state_career_stats_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        transition_in(c, 0x001f_55d4, callee::CAREER_STATS_SHOW_HIGHLIGHTS)
    })
}

/// `void cFEStateBuyAttrib_OnChildEvent(self, sender, msg, arg)` (0x1f4a60):
/// on the "confirm purchase" message (0x16), commit the purchase if the
/// sender has one pending (`+0x6c`).
pub fn c_fe_state_buy_attrib_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x16;
        let msg = c.g[A2][0];
        frame.save_ra(c, 0);
        if msg == 0x16 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(abi::arg(c, 1), 0x6c));
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                call_ptr(c, callee::BUY_ATTRIB_COMMIT_PURCHASE, 0x001f_4a84)?;
                c.cycles += 1;
            }
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCareerStats_OnActivate(self, arg)` (0x1f55e8): activate the
/// base front-end state, then select the page held in `self+0x48`.
pub fn c_fe_state_career_stats_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::FE_STATE_ON_ACTIVATE, 0x001f_55fc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.lw(p(r32(c, A0), 0x48));
        call_ptr(c, callee::CAREER_STATS_SELECT_PAGE, 0x001f_5608)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_OnActivate(State* s)` (0x1f49f8): activate the
/// state, then the selected attribute's "show" method (slot +0x38 of the
/// object at `+0xac`).
pub fn c_fe_state_buy_attrib_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::C_FE_STATE_ON_ACTIVATE.call(c, 0x001f_4a0c)?;
        let this = crate::abi::r32(c, S0);
        let obj = lw(c, this, 0xac);
        c.g[A2][0] = obj;
        c.g[A1][0] = 1;
        let vt = lw(c, obj as u32, 8);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x38);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = m.this(obj);
        crate::call::call_ptr(c, m.func as u32, 0x001f_4a28)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCareerStats_SetText(s, const char* name, const char* text)`
/// (0x1f6840): find the text object called `name` in the screen at +0x40 and
/// set it to `text`. 13 (+2) + 5 cycles plus the callees.
pub fn cfe_state_career_stats_set_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001f_6860)?;
        c.g[A0][0] = c.lw(p(self_, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME, 0x001f_686c)?;
        let text = c.g[V0][0];
        c.g[A0][0] = text;
        if text != 0 {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, callee::UI_TEXT_SET_ASCII_STRING, 0x001f_687c)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateCareerStats_ctor(self)` (0x1f54b0): run the UI state ctor,
/// install the career stats class and copy the pad mask byte.
pub fn c_fe_state_career_stats_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x001f_54c4);
        call_ptr(c, callee::UI_STATE_CTOR_2, 0x001f_54c4)?;
        c.g[V0][0] = 0x2a;
        c.g[V1][0] = 0x47_0000;
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3838));
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 0;
        Addr(p(r32(c, S0), 0x44)).sb(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x10).lw(c);
        c.g[RA][0] = sx(0x001f_54ec);
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        call_ptr(c, callee::FE_MODULE_GET_PLAYER_PAD_MASK, 0x001f_54ec)?;
        Addr(p(r32(c, S0), 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateBuyAttrib_OnWidgetNotify(State*, void*, int key)`
/// (0x001f4a38).
pub fn cfe_state_buy_attrib_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `int cFEStateCareerStats_OnWidgetNotify(State*, void*, int key)`
/// (0x001f5618).
pub fn cfe_state_career_stats_on_widget_notify(c: &mut Cpu) {
    handle_key_6_8_9(c);
}

// ---------------------------------------------------------------------------
// Front-end buy-attrib and event-select screens.

/// `void cFEStateBuyAttrib_UpdateTotalCost(cFEStateBuyAttrib* self)`
/// (0x1f5240): recompute the total cost label from the selected item.
pub fn c_fe_state_buy_attrib_update_total_cost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let self_ = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_f090;
        c.g[S1][0] = self_;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_f090);
        call_ptr(c, 0x0031_7670, 0x001f_5260)?;
        c.g[A0][0] = Addr(at(c, S1, 0x48)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_526c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(at(c, S1, 0xa0)).lw(c);
            call_ptr(c, 0x0019_8af0, 0x001f_5280)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_528c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_UpdateMoney(cFEStateBuyAttrib* self)`
/// (0x1f52a0): refresh the money label from the player's balance.
pub fn c_fe_state_buy_attrib_update_money(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let self_ = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_f0a0;
        c.g[S1][0] = self_;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_f0a0);
        call_ptr(c, 0x0031_7670, 0x001f_52c0)?;
        c.g[A0][0] = Addr(at(c, S1, 0x48)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_52cc)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(at(c, S1, 0xa4)).lw(c);
            call_ptr(c, 0x0019_8af0, 0x001f_52e0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_52ec)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI widget event handlers.

/// `func_001F5650` (0x1f5650): `cFEStateCareerStats::OnWidgetAction`. For
/// widget action 6 only (action 5 and a null widget do nothing) it asks the
/// widget (`widget+0x18` through the owner's `vtable[0x28]`) whether it has a
/// selection; if it does, it forwards the event to `this->+0x10 + 0x18`
/// (`0x39f400`). Signature: `void on_widget_action(this, widget, action)`.
pub fn c_fe_state_career_stats_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_widget = c.g[V1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        let mut tail_charge = true;
        if no_widget {
            // nothing to do
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            let five = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = 0x6;
            if five {
                // nothing to do
            } else {
                c.cycles += 2;
                if c.g[A2][0] != c.g[V0][0] {
                    tail_charge = false;
                } else {
                    c.cycles += 11;
                    c.g[V0][0] = lw_r(c, S0, 0x10);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = lw_r(c, V1, 0x18);
                    c.g[A3][0] = Addr(low(c, V0)).lw(c);
                    c.g[V1][0] = lw_r(c, A3, 0x4);
                    c.g[A0][0] = lh_r(c, V1, 0x28);
                    c.g[V0][0] = lw_r(c, V1, 0x2c);
                    let target = low(c, V0);
                    c.g[RA][0] = sx(0x001f_56a0);
                    c.g[A0][0] = sx(low(c, A3).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x001f_56a0)?;
                    let no_selection = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[V0][0];
                    if !no_selection {
                        c.cycles += 3;
                        c.g[A0][0] = lw_r(c, S0, 0x10);
                        c.g[RA][0] = sx(0x001f_56b4);
                        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x18));
                        call_ptr(c, 0x0039_f400, 0x001f_56b4)?;
                    }
                }
            }
        }
        if tail_charge {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateBuyAttrib_OnWidgetCreated` (0x001f4910).
//
// `int cFEStateBuyAttrib::OnWidgetCreated(self, UIWidget* w)`: install the
// created widget into the state's slots by its id.

pub fn fe_state_buy_attrib_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f49bc: {
            'b_1f4964: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = c.g[A0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = 0x470000;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001f493c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xff8));
                call_ptr(c, 0x00317670, 0x001f493c)?;
                if c.g[S0][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x470000;
                    break 'b_1f4964;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001f4950);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfe0));
                    call_ptr(c, 0x00317670, 0x001f4950)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001f495c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001f495c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1f49bc;
                    } else {
                        break 'b_1f4964;
                    }
                }
            }
            'b_1f49b8: {
                'b_1f4980: {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001f4970);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfc0));
                    call_ptr(c, 0x00317670, 0x001f4970)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1f4980;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0xac), c.g[S1][0]);
                        if t {
                            break 'b_1f49b8;
                        } else {
                            break 'b_1f4980;
                        }
                    }
                }
                'b_1f499c: {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001f498c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2320));
                    call_ptr(c, 0x00317670, 0x001f498c)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1f499c;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                        if t {
                            break 'b_1f49b8;
                        } else {
                            break 'b_1f499c;
                        }
                    }
                }
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001f49a8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2328));
                call_ptr(c, 0x00317670, 0x001f49a8)?;
                let t = c.g[S0][0] != c.g[V0][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1f49bc;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    break 'b_1f49b8;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1f49bc;
        }
        c.cycles += 5;
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
// Buy attributes screen.

/// `void cFEStateBuyAttrib_Init(self)` (0x1f4830): create the screen, fetch
/// the character's attributes and refresh every displayed value.
pub fn cfestatebuyattrib_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x1008));
        c.g[S0][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_4854)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_4868)?;
        let no_screen = c.g[V0][0] == 0;
        Addr(p(r32(c, S1), 0x48)).sw(c, c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_4880)?;
        }
        c.cycles += 35;
        call_ptr(c, 0x0014_dc80, 0x001f_4888)?;
        c.g[A1][0] = 0xb;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x001f_4894)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_a080, 0x001f_48a0)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0015_0928, 0x001f_48b0)?;
        Addr(p(r32(c, S1), 0xa8)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr(p(r32(c, S1), 0xa0)).sw(c, 0);
        Addr(p(r32(c, S1), 0xa4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x001f_5240, 0x001f_48c4)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_4ea8, 0x001f_48cc)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_4fc0, 0x001f_48d4)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_50b0, 0x001f_48dc)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_52a0, 0x001f_48e4)?;
        call_ptr(c, 0x0028_b180, 0x001f_48ec)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x0028_f140, 0x001f_48f8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_PostInput(self, widget)`: after the input pass, if not busy,
/// the attribute widget's decrement (vtable 0x98) refunds one bought level and
/// its increment (0xa0) buys one when the cost fits the bank and the per-level
/// limit; otherwise a "cannot" message goes to the custom interface. Updates
/// the total cost and the bars when anything changed.
/// (0x001f4c30; every register, cycle and call of the generated body is kept.)
pub fn cfe_state_buy_attrib_post_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f4e74: {
            c.cycles += 27;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.g[S1][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x001f4c5c);
            c.g[S4][0] = 0u64;
            call_ptr(c, 0x0039e508, 0x001f4c5c)?;
            c.g[RA][0] = sx(0x001f4c64);
            call_ptr(c, 0x0014dc80, 0x001f4c64)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001f4c70);
            c.g[A1][0] = 0xb;
            call_ptr(c, 0x0014dd58, 0x001f4c70)?;
            c.g[A0][0] = 0x470000;
            c.g[S3][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001f4c80);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfc0));
            call_ptr(c, 0x00317670, 0x001f4c80)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            c.g[RA][0] = sx(0x001f4c8c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001f4c8c)?;
            c.g[S2][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
            let t = c.g[V0][0] != 0u64;

            if t {
                break 'b_1f4e74;
            } else {
                'b_1f4e4c: {
                    'b_1f4e48: {
                        'b_1f4e30: {
                            'b_1f4de4: {
                                'b_1f4d28: {
                                    c.cycles += 7;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001f4cb0);
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001f4cb0)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x84));
                                    if t {
                                        break 'b_1f4d28;
                                    } else {
                                        'b_1f4d08: {
                                            c.cycles += 5;
                                            c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                                            c.g[A0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                            let t = (c.g[V0][0] as i64) <= 0;
                                            c.g[S4][0] = 0x1;
                                            if t {
                                                break 'b_1f4d08;
                                            } else {
                                                c.cycles += 15;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x68));
                                                c.g[RA][0] = sx(0x001f4ce8);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                                                call_ptr(c, 0x00150e50, 0x001f4ce8)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_sub(c.g[V0][0] as u32));
                                                c.g[RA][0] = sx(0x001f4cfc);
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xa0),
                                                    c.g[V1][0],
                                                );
                                                Callee {
                                                    addr: 0x001f5240,
                                                    f: ee::s04::cFEStateBuyAttrib_UpdateTotalCost,
                                                }
                                                .call(c, 0x001f4cfc)?;
                                                c.g[A0][0] = 0x4a0000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x2330));
                                                if t {
                                                    break 'b_1f4de4;
                                                } else {
                                                    break 'b_1f4d08;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                        c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                        if c.g[A2][0] == 0 {
                                            c.g[A2][0] = c.g[V1][0];
                                        }
                                        let t = c.g[A2][0] == 0u64;
                                        c.g[A1][0] = 0x4;
                                        if t {
                                            break 'b_1f4e74;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                            if t {
                                                break 'b_1f4e30;
                                            } else {
                                                break 'b_1f4d28;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f4d3c);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001f4d3c)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x68));
                                if t {
                                    break 'b_1f4e74;
                                } else {
                                    c.cycles += 12;
                                    c.g[S1][0] = sx((c.g[S2][0] as u32) << 2);
                                    c.g[S2][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                    c.g[S4][0] = 0x1;
                                    c.g[RA][0] = sx(0x001f4d60);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                                    call_ptr(c, 0x00150e50, 0x001f4d60)?;
                                    c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                    c.g[A0][0] = ((c.g[A2][0] as i64) < 11i64) as u64;
                                    let t = c.g[A0][0] == 0u64;
                                    c.g[V1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    if t {
                                        break 'b_1f4e48;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                                        c.g[V0][0] =
                                            ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                            break 'b_1f4e4c;
                                        } else {
                                            c.cycles += 12;
                                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x84));
                                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                            c.g[A3][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                            c.g[A1][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                            c.g[A2][0] = c.lw(c.g[A3][0] as u32);
                                            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A2][0] as u32));
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                            if t {
                                                break 'b_1f4e74;
                                            } else {
                                                c.cycles += 12;
                                                c.g[A0][0] = c.g[S3][0];
                                                c.sw(c.g[A3][0] as u32, c.g[V0][0]);
                                                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                                c.g[RA][0] = sx(0x001f4dc8);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                                                call_ptr(c, 0x00150e50, 0x001f4dc8)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[RA][0] = sx(0x001f4ddc);
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xa0),
                                                    c.g[V1][0],
                                                );
                                                Callee {
                                                    addr: 0x001f5240,
                                                    f: ee::s04::cFEStateBuyAttrib_UpdateTotalCost,
                                                }
                                                .call(c, 0x001f4ddc)?;
                                                c.g[A0][0] = 0x470000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0xf90));
                                                break 'b_1f4de4;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_1f4e18: {
                                c.cycles += 9;
                                c.g[RA][0] = sx(0x001f4dec);
                                call_ptr(c, 0x00317670, 0x001f4dec)?;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                                c.g[RA][0] = sx(0x001f4df8);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039c778, 0x001f4df8)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffff;
                                if c.g[A1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    break 'b_1f4e18;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                                    c.g[RA][0] = sx(0x001f4e14);
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x0039c870, 0x001f4e14)?;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    break 'b_1f4e18;
                                }
                            }
                            c.cycles += 5;
                            c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                            if c.g[A2][0] == 0 {
                                c.g[A2][0] = c.g[V1][0];
                            }
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_1f4e74;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                break 'b_1f4e30;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001f4e40);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001f4e40)?;
                        let t = true;

                        if t {
                            break 'b_1f4e74;
                        } else {
                            break 'b_1f4e48;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    break 'b_1f4e4c;
                }
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                if c.g[A2][0] == 0 {
                    c.g[A2][0] = c.g[V1][0];
                }
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = 0x4;
                if t {
                    break 'b_1f4e74;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001f4e74);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001f4e74)?;
                    break 'b_1f4e74;
                }
            }
        }
        'b_1f4e88: {
            c.cycles += 2;
            if c.g[S4][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_1f4e88;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001f4e84);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001f50b0,
                    f: ee::s04::cFEStateBuyAttrib_UpdateBars,
                }
                .call(c, 0x001f4e84)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_1f4e88;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `base + off` as a 32-bit guest address (the `off(reg)` operand).
pub(crate) fn oc970_at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

// Format strings and callees of the buy-attrib screens (the generated bodies'
// addresses): sprintf 0x417828, StrHash 0x317670, UIScreen_FindObject 0x39d860,
// UIText_SetAsciiString 0x3a0c50, int to string 0x198af0.

/// `void cFEStateBuyAttrib_UpdateTotalCost(cFEStateBuyAttrib* self)`
/// (0x1f5240): the total-cost text gets the item's cost (`self+0xa0`) as a
/// string, if the screen has a text object for it.
pub fn oc970_update_total_cost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_f090);
        call_ptr(c, 0x0031_7670, 0x001f_5260)?;
        c.g[A0][0] = Addr(oc970_at(c, S1, 0x48)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_526c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(oc970_at(c, S1, 0xa0)).lw(c);
            call_ptr(c, 0x0019_8af0, 0x001f_5280)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_528c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_UpdateLevelText(cFEStateBuyAttrib* self)`
/// (0x1f4ea8): for each of the seven attribute rows, the level is
/// `points / 5` (kept at its maximum so far), the rest `points - 5 * level`
/// is shown as `rem * 2` next to it, in the text object `"<row name>"`.
pub fn oc970_update_level_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S5, 0x70);
        frame.save(c, S2, 0xa0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S3, 0x90);
        c.g[S2][0] = sx(oc970_at(c, S5, 0x68));
        frame.save(c, S4, 0x80);
        c.g[S3][0] = 0;
        frame.save(c, S6, 0x60);
        c.g[S4][0] = sx(sp(c).wrapping_add(0x20));
        frame.save(c, S7, 0x50);
        c.g[S6][0] = 5;
        frame.save(c, FP, 0x40);
        c.g[S7][0] = 0x4a_0000;
        frame.save(c, S0, 0xc0);
        c.g[FP][0] = 0x4a_0000;
        frame.save(c, S1, 0xb0);
        frame.save_ra(c, 0x30);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_sub(0x1c)).lw(c);
        'lp: loop {
            c.cycles += 2;
            // Level and remainder of row i; the level only ever grows.
            c.cycles += 11;
            c.g[A2][0] = c.g[S3][0];
            c.g[A0][0] = c.g[SP][0];
            c.div(0, V0, S6);
            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2338));
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.g[S1][0] = c.lo[0];
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let t = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
            if !t {
                c.cycles += 1;
                let level = c.g[S1][0];
                Addr(c.g[S2][0] as u32).sw(c, level);
            }
            // Name the row, look its text up and set it.
            c.cycles += 15;
            c.g[S0][0] = Addr((c.g[S2][0] as u32).wrapping_sub(0x1c)).lw(c);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            call_ptr(c, 0x0041_7828, 0x001f_4f40)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001f_4f48)?;
            c.g[A0][0] = Addr(oc970_at(c, S5, 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_4f54)?;
            c.g[A3][0] = sx((c.g[S0][0] as u32) << 1);
            c.g[A2][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[S0][0] = c.g[V0][0];
            let found = c.g[S0][0] != 0;
            c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x2340));
            if found {
                c.cycles += 5;
                call_ptr(c, 0x0041_7828, 0x001f_4f74)?;
                c.g[A1][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a_0c50, 0x001f_4f80)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 7) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_sub(0x1c)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001f_4ef8);
                }
                continue 'lp;
            }
            break;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_UpdateCostText(cFEStateBuyAttrib* self)`
/// (0x1f4fc0): for each of the seven rows, the text shows the row's level
/// minus one through the BE interface, and for levels below 11 the row's
/// vtable entry is called with 0 (the level-maxed marker at 11 and up).
pub fn oc970_update_cost_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S3, 0x50);
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        c.g[S1][0] = sx(oc970_at(c, S3, 0x68));
        frame.save(c, S4, 0x40);
        c.g[S2][0] = 0;
        frame.save(c, S5, 0x30);
        frame.save(c, S0, 0x80);
        frame.save_ra(c, 0x20);
        c.g[S5][0] = 0x4a_0000;
        call_ptr(c, 0x0014_dc80, 0x001f_4ff4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x001f_5000)?;
        c.g[S4][0] = c.g[V0][0];
        'lp: loop {
            c.cycles += 13;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2348));
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            call_ptr(c, 0x0041_7828, 0x001f_501c)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001f_5024)?;
            c.g[A0][0] = Addr(oc970_at(c, S3, 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_5030)?;
            c.g[S0][0] = c.g[V0][0];
            let missing = c.g[S0][0] == 0;
            c.g[A0][0] = c.g[S4][0];
            if !missing {
                c.cycles += 12;
                c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                call_ptr(c, 0x0015_0e50, 0x001f_5048)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0019_8af0, 0x001f_5050)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a_0c50, 0x001f_505c)?;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 11) as u64;
                let below_max = c.g[V0][0] != 0;
                c.g[A1][0] = 0;
                if !below_max {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(oc970_at(c, S0, 8)).lw(c);
                    c.g[A0][0] = Addr(oc970_at(c, V0, 0x48)).lh(c);
                    c.g[V1][0] = Addr(oc970_at(c, V0, 0x4c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001f_5080)?;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 7) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001f_5008);
                }
                continue 'lp;
            }
            break;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_UpdateBars(cFEStateBuyAttrib* self)` (0x1f50b0):
/// positions the seven points bars (`pts[i] * 3.3636` wide, height 12) and
/// the add-on bars (`(add[i] + pts[i]) * 3.3636`), each a text-screen object
/// whose vtable takes a vector at +0xa8/+0x48 and a visibility flag.
pub fn oc970_update_bars(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x80);
        frame.save_fpr(c, 21, 0xb8);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xb0);
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        c.g[S1][0] = sx(oc970_at(c, S2, 0x4c));
        frame.save(c, S4, 0x60);
        c.g[S3][0] = 0;
        frame.save(c, S5, 0x50);
        c.g[S4][0] = 0;
        frame.save(c, S6, 0x40);
        c.g[S5][0] = 0x4a_0000;
        frame.save(c, S0, 0xa0);
        c.g[S6][0] = 0x47_0000;
        frame.save_ra(c, 0x30);
        c.g[AT][0] = 0x4140_0000;
        abi::set_fpr(c, 21, FpuF32(c.g[AT][0] as u32));
        let scale = Addr((c.g[GP][0] as u32).wrapping_sub(0x554c)).read_fpu(c);
        abi::set_fpr(c, 20, scale);
        'lp: loop {
            // The points bar of row i.
            c.cycles += 12;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0xf80));
            c.g[A2][0] = c.g[S3][0];
            call_ptr(c, 0x0041_7828, 0x001f_5110)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001f_5118)?;
            c.g[A0][0] = Addr(oc970_at(c, S2, 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_5124)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                c.g[A0][0] = c.g[SP][0];
            } else {
                c.cycles += 19;
                let pts = Addr(c.g[S1][0] as u32).read_fpu(c);
                abi::set_fpr(c, 0, FpuF32::from_i32(pts.0 as i32));
                c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
                frame.save_fpr(c, 21, 0x24);
                Addr(sp(c).wrapping_add(0x28)).sw(c, 0);
                let width = FpuF32(c.f[0]) * FpuF32(c.f[20]);
                abi::set_fpr(c, 0, width);
                Addr(sp(c).wrapping_add(0x20)).write_fpu(c, FpuF32(c.f[0]));
                c.g[V0][0] = Addr(oc970_at(c, S0, 8)).lw(c);
                c.g[A0][0] = Addr(oc970_at(c, V0, 0xa8)).lh(c);
                c.g[V1][0] = Addr(oc970_at(c, V0, 0xac)).lw(c);
                let set_vec = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, set_vec, 0x001f_5160)?;
                c.g[V0][0] = Addr(oc970_at(c, S0, 8)).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = Addr(oc970_at(c, V0, 0x48)).lh(c);
                c.g[V1][0] = Addr(oc970_at(c, V0, 0x4c)).lw(c);
                let set_flag = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, set_flag, 0x001f_5178)?;
                c.g[A0][0] = c.g[SP][0];
            }
            // The add-on bar of row i: (add[i] + pts[i]) wide.
            c.cycles += 11;
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2350));
            c.g[A2][0] = c.g[S3][0];
            call_ptr(c, 0x0041_7828, 0x001f_5188)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001f_5190)?;
            c.g[A0][0] = Addr(oc970_at(c, S2, 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_519c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S4][0] as u32));
            if c.g[S0][0] != 0 {
                c.cycles += 21;
                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V0][0] = Addr(oc970_at(c, V1, 0x84)).lw(c);
                c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
                frame.save_fpr(c, 21, 0x24);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                Addr(sp(c).wrapping_add(0x28)).sw(c, 0);
                abi::set_fpr(c, 0, FpuF32(c.g[V0][0] as u32));
                let as_float = FpuF32::from_i32(c.f[0] as i32);
                abi::set_fpr(c, 0, as_float);
                let width = FpuF32(c.f[0]) * FpuF32(c.f[20]);
                abi::set_fpr(c, 0, width);
                Addr(sp(c).wrapping_add(0x20)).write_fpu(c, FpuF32(c.f[0]));
                c.g[V0][0] = Addr(oc970_at(c, S0, 8)).lw(c);
                c.g[A0][0] = Addr(oc970_at(c, V0, 0xa8)).lh(c);
                c.g[V1][0] = Addr(oc970_at(c, V0, 0xac)).lw(c);
                let set_vec = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, set_vec, 0x001f_51e4)?;
                c.g[V0][0] = Addr(oc970_at(c, S0, 8)).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = Addr(oc970_at(c, V0, 0x48)).lh(c);
                c.g[V1][0] = Addr(oc970_at(c, V0, 0x4c)).lw(c);
                let set_flag = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, set_flag, 0x001f_51fc)?;
            }
            c.cycles += 5;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 7) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001f_5100);
                }
                continue 'lp;
            }
            break;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_OnWidgetAction(cFEStateBuyAttrib* self, void* obj,
/// int msg)` (0x1f4a90). A null `obj` does nothing. `msg` 5 (next) opens the
/// buy popup when the bank covers the total cost and the total is positive,
/// else plays the engine's sound. `msg` 6 (prev) clears the buy prompt and
/// hides its group, or leaves the screen for the previous state. Other
/// messages return at once.
pub fn oc970_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let obj_null = c.g[V1][0] == 0;
        frame.save_ra(c, 0);
        'c20: {
            'c1c: {
                'c18: {
                    if obj_null {
                        break 'c18;
                    }
                    'd: {
                        'e: {
                            'f: {
                                'g: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 5;
                                    let is_next = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = 6;
                                    if is_next {
                                        break 'g;
                                    }
                                    c.cycles += 2;
                                    let is_prev = c.g[A2][0] == c.g[V0][0];
                                    frame.restore(c, S0, 0x20);
                                    if is_prev {
                                        break 'f;
                                    }
                                    c.cycles += 2;
                                    frame.restore(c, S1, 0x10);
                                    break 'c20;
                                }
                                // msg 5: the bank against the total cost.
                                'h: {
                                    c.cycles += 5;
                                    c.g[V0][0] = Addr(oc970_at(c, S1, 0xa4)).lw(c);
                                    c.g[V1][0] = Addr(oc970_at(c, S1, 0xa0)).lw(c);
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    if c.g[V0][0] != 0 {
                                        c.g[V0][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                                        break 'h;
                                    }
                                    'i: {
                                        c.cycles += 2;
                                        let no_total = (c.g[V1][0] as i64) <= 0;
                                        c.g[A1][0] = 0x47_0000;
                                        if no_total {
                                            break 'i;
                                        }
                                        // Open the buy popup on the state stack.
                                        c.cycles += 22;
                                        c.g[A0][0] = 0x70;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xfb0));
                                        c.g[A2][0] = 0x100;
                                        c.g[A3][0] = 0;
                                        call_ptr(c, 0x0031_7d70, 0x001f_4af8)?;
                                        c.g[A1][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A3][0] = Addr(oc970_at(c, S1, 0x44)).lb(c);
                                        c.g[A2][0] = c.g[S1][0];
                                        call_ptr(c, 0x001c_aba8, 0x001f_4b0c)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = Addr(oc970_at(c, S1, 0xa0)).lw(c);
                                        c.g[A3][0] = Addr(oc970_at(c, S1, 0xa4)).lw(c);
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0;
                                        call_ptr(c, 0x001c_afc0, 0x001f_4b24)?;
                                        c.g[A0][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                                        c.g[A1][0] = c.g[S0][0];
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039_f290, 0x001f_4b34)?;
                                        frame.restore(c, S0, 0x20);
                                        break 'c1c;
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                                }
                                // The sound of the engine (msg 5 without the popup).
                                c.cycles += 5;
                                c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                                c.g[A2][0] = Addr(oc970_at(c, V0, 0x14)).lw(c);
                                if c.g[A2][0] == 0 {
                                    c.g[A2][0] = c.g[V1][0];
                                }
                                let no_sound = c.g[A2][0] == 0;
                                c.g[A1][0] = 4;
                                if no_sound {
                                    break 'c18;
                                }
                                c.cycles += 4;
                                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                                c.g[A0][0] = Addr(oc970_at(c, V0, 0x10)).lh(c);
                                c.g[V1][0] = Addr(oc970_at(c, V0, 0x14)).lw(c);
                                break 'e;
                            }
                            // msg 6: clear the buy prompt, or leave the screen.
                            c.cycles += 3;
                            c.g[V0][0] = Addr(oc970_at(c, S1, 0xa8)).lw(c);
                            let no_prompt = c.g[V0][0] == 0;
                            c.g[A1][0] = 0x47_0000;
                            if no_prompt {
                                break 'd;
                            }
                            c.cycles += 9;
                            Addr(oc970_at(c, S1, 0xa8)).sw(c, 0);
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xfa0));
                            call_ptr(c, 0x0039_e8b8, 0x001f_4b80)?;
                            c.g[A2][0] = Addr(oc970_at(c, S1, 0xac)).lw(c);
                            c.g[A1][0] = 1;
                            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
                            c.g[A0][0] = Addr(oc970_at(c, V0, 0x38)).lh(c);
                            c.g[V1][0] = Addr(oc970_at(c, V0, 0x3c)).lw(c);
                        }
                        // Call the sound's (or the attrib menu's) vtable entry.
                        c.cycles += 4;
                        let entry = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, entry, 0x001f_4b9c)?;
                        frame.restore(c, S0, 0x20);
                        break 'c1c;
                    }
                    // msg 6 with no buy prompt: go back to the previous state.
                    c.cycles += 12;
                    c.g[V0][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = Addr(oc970_at(c, V1, 0x18)).lw(c);
                    c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(4)).lw(c);
                    c.g[A0][0] = Addr(oc970_at(c, V1, 0x28)).lh(c);
                    c.g[V0][0] = Addr(oc970_at(c, V1, 0x2c)).lw(c);
                    let prev_entry = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, prev_entry, 0x001f_4bc8)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0 {
                        frame.restore(c, S0, 0x20);
                        break 'c1c;
                    }
                    // A frame with the transition-out label plays first.
                    'c08: {
                        c.cycles += 10;
                        c.g[A0][0] = 0x47_0000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                        call_ptr(c, 0x0031_7670, 0x001f_4be0)?;
                        c.g[A0][0] = Addr(oc970_at(c, S1, 0x48)).lw(c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x001f_4bec)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let no_frame = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[A1][0] & 0xffff;
                        if no_frame {
                            break 'c08;
                        }
                        c.cycles += 3;
                        c.g[A0][0] = Addr(oc970_at(c, S1, 0x48)).lw(c);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0039_c870, 0x001f_4c08)?;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = Addr(oc970_at(c, S1, 0x10)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, 0x001f_4c18)?;
                    break 'c18;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            c.cycles += 1;
            frame.restore(c, S1, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
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
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::bx::bxstringctor::tests::be_library;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_2 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_3;
    pub(crate) use crate::game::fe::fememcard::tests::vtable_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear as clear_2;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_2;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_2;
    use ssx3_ee as ee;

    #[test]
    fn fe_state_buy_attrib_transition_in_matches() {
        crate::difftest::check(
            ee::s04::f_001f49d0__generated,
            fe_state_buy_attrib_transition_in,
            40,
            &|c, seed| {
                machine(c, seed, 0x1f49_d0, 2);
                be_library(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn fe_state_career_stats_transition_in_matches() {
        crate::difftest::check(
            ee::s04::f_001f55c0__generated,
            fe_state_career_stats_transition_in,
            40,
            &|c, seed| {
                machine(c, seed, 0x1f55_c0, 2);
                be_library(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_buy_attrib_on_child_event_matches() {
        check(
            ee::s04::f_001f4a60__generated,
            c_fe_state_buy_attrib_on_child_event,
            60,
            &|c, seed| {
                machine(c, seed, 0x1f4a60, 6);
                globals(c);
                clear(c, DATA, 0x800);
                c.g[A0][0] = DATA as u64;
                let sender = DATA + 0x400;
                c.g[A1][0] = sender as u64;
                // The purchase confirmation message (0x16) with and without a
                // pending purchase, and another message.
                c.g[A2][0] = match seed % 3 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => 0x16,
                };
                c.bus
                    .write32(sender + 0x6c, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn c_fe_state_career_stats_on_activate_matches() {
        check(
            ee::s04::f_001f55e8__generated,
            c_fe_state_career_stats_on_activate,
            20,
            &|c, seed| {
                machine(c, seed, 0x1f55e8, 8);
                globals(c);
                clear(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed & 1) as u64;
                // The page to select (`self+0x48`); page 0 is the default and
                // keeps `cFEStateCareerStats_SelectPage` on its first path.
                c.bus.write32(DATA + 0x48, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_buy_attrib_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001f49f8__generated,
            c_fe_state_buy_attrib_on_activate,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27, 1);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0xac, DATA + 0x3000);
                vtable_at(c, &mut r, DATA + 0x3000, 0x38);
            },
        );
    }

    #[test]
    fn cfe_state_career_stats_set_text_matches() {
        check(
            ee::s04::f_001f6840__generated,
            cfe_state_career_stats_set_text,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1f68);
                let s = DATA;
                let screen = DATA + 0x400;
                let name = DATA + 0x1000;
                let n = (r.next_u32() % 12) as usize;
                for k in 0..n {
                    c.bus
                        .write8(name + k as u32, b'A' + ((seed as usize + k) % 26) as u8);
                }
                c.bus.write8(name + n as u32, 0);
                fill(c, &mut Rng(0x1f68_0000 ^ seed as u64), screen, 4);
                c.bus.write32(s + 0x40, screen);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(name);
                c.g[A2][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn c_fe_state_career_stats_ctor_matches() {
        check(
            ee::s04::f_001f54b0__generated,
            c_fe_state_career_stats_ctor,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1f54b0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_2(c, this, 0x400);
                let module = DATA + 0x2000;
                clear_2(c, module, 0x40);
                c.bus.write32(this + 0x10, module);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn cfe_state_career_stats_on_widget_notify_matches() {
        check(
            ssx3_ee::s04::f_001f5618__generated,
            cfe_state_career_stats_on_widget_notify,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f5618, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_career_stats_on_widget_action_matches() {
        check(
            ee::s04::f_001f5650__generated,
            c_fe_state_career_stats_on_widget_action,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x1f5650, 4);
                be_library_2(c);
                world(c);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
                let widget = DATA + 0x400;
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    _ => widget as u64,
                };
                c.g[A2][0] = [5, 6, 7][(seed % 3) as usize] as u64;
                w(c, widget + 0x18, 0x10);
                let list = DATA + 0x600;
                w(c, DATA + 0x10, list);
                let owner = DATA + 0x800;
                w(c, list, owner);
                let vt = DATA + 0x2000;
                w(c, owner + 4, vt);
                slot(c, vt, 0x28, 0, FAKE);
            },
        );
    }

    #[test]
    fn fe_state_buy_attrib_on_widget_created_matches() {
        strict(
            ee::s04::f_001f4910__generated,
            fe_state_buy_attrib_on_widget_created,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f49, 4);
                let this = DATA;
                let widget = DATA + 0x1000;
                c.bus
                    .write32(widget + 0x38, if seed % 4 == 0 { 0 } else { r.next_u32() });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(widget);
            },
        );
    }

    #[test]
    fn cfestatebuyattrib_init_matches() {
        check(
            ee::s04::f_001f4830__generated,
            cfestatebuyattrib_init,
            6,
            &|c, seed| {
                let _ = base_2(c, seed, 0x1f4830, 3);
                let this = DATA;
                clear_3(c, this, 0x800);
                c.g[A0][0] = this as u64;
                w(c, this + 0x10, DATA + 0x400);
                w(c, this + 0x44, (seed % 3) as u32);
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes;

    use crate::difftest::fakes::machine_rt;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::fe::festatelodge::tests::oc963_base;
    pub(crate) use crate::game::fe::festatelodge::tests::oc963_force;
    pub(crate) use crate::game::fe::festatelodge::tests::oc963_method;
    pub(crate) use crate::game::fe::festatelodge::tests::oc963_slot;
    pub(crate) use crate::game::fe::festatelodge::tests::w_2;

    pub(crate) use crate::game::fe::festatelodge::tests::OC963_ENG;

    pub(crate) use crate::game::fe::festatelodge::tests::OC963_F_COST;
    pub(crate) use crate::game::fe::festatelodge::tests::OC963_F_LOOKUP;
    pub(crate) use crate::game::fe::festatelodge::tests::OC963_F_OBJECT;

    pub(crate) use crate::game::fe::festatelodge::tests::OC963_SCREEN;
    pub(crate) use crate::game::fe::festatelodge::tests::OC963_SELF;

    pub(crate) use crate::game::fe::festatelodge::tests::OC963_WIDGET;

    pub(crate) use crate::game::fe::festatelodge::tests::OC963_WVT;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    pub(crate) const OC963_OBJ95: u32 = DATA + 0x20_d000;

    pub(crate) const OC963_MSGOBJ: u32 = DATA + 0x20_e000;

    pub(crate) const OC963_MSGVT: u32 = DATA + 0x20_f000;

    /// Buy-attribute post-input: the widget's decrement (vtable 0x98) refunds
    /// a bought level, the increment (0xa0) buys one when the cost fits the
    /// bank and the level limit; refusals send the "cannot" message; a busy
    /// state does nothing; the bars update when something changed.
    #[test]
    fn cfe_state_buy_attrib_post_input_matches() {
        check(
            ee::s04::f_001f4c30__generated,
            cfe_state_buy_attrib_post_input,
            400,
            &|c, seed| {
                let mut r = oc963_base(c, seed, 0x1f4c30);
                let idx = r.next_u32() % 4;
                c.bus.write8(OC963_OBJ95 + 0x95, idx as u8);
                oc963_force(c, OC963_F_OBJECT, OC963_OBJ95);
                oc963_force(c, OC963_F_COST, 5 + r.next_u32() % 20);
                oc963_force(
                    c,
                    OC963_F_LOOKUP,
                    if r.next_u32() % 2 == 0 { 0xffff } else { 3 },
                );
                w_2(c, OC963_SELF + 0x48, OC963_SCREEN);
                w_2(c, OC963_SELF + 0xa8, (r.next_u32() % 6 == 0) as u32);
                w_2(c, OC963_SELF + 0xa0, r.next_u32() % 100);
                w_2(c, OC963_SELF + 0xa4, r.next_u32() % 120);
                for i in 0..4 {
                    w_2(c, OC963_SELF + 0x84 + 4 * i, r.next_u32() % 4);
                    w_2(c, OC963_SELF + 0x68 + 4 * i, 1 + r.next_u32() % 12);
                    w_2(c, OC963_SELF + 0x4c + 4 * i, r.next_u32() % 30);
                }
                let dec = oc963_method(&mut r);
                let inc = oc963_method(&mut r);
                oc963_slot(c, OC963_WVT, 0x98, dec, 0);
                oc963_slot(c, OC963_WVT, 0xa0, inc, 4);
                // The message object: the engine's own, or the gp fallback.
                w_2(c, OC963_MSGOBJ, OC963_MSGVT);
                w_2(c, GAME_GP + 0x2968, OC963_MSGVT);
                w_2(
                    c,
                    OC963_ENG + 0x14,
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        OC963_MSGOBJ
                    },
                );
                oc963_slot(c, OC963_MSGVT, 0x10, FAKE + 0xc0, 0);
                c.g[A1][0] = sx(OC963_WIDGET);
            },
        );
    }

    // Gated fakes for the callees that allocate or walk game data on zeroed
    // test memory: each has its own gate word, magic and forced-result word in
    // the machine's RAM (DATA+0x270000..), past the other fixture regions. The
    // result word is read as is, so a forced 0 is possible.
    pub(crate) const OC970_FIND_CTRL: u32 = DATA + 0x27_0000;

    pub(crate) const OC970_FIND_RESULT: u32 = DATA + 0x27_0004;

    pub(crate) const OC970_FIND_MAGIC: u32 = 0x0970_0001;

    pub(crate) const OC970_LOOKUP_CTRL: u32 = DATA + 0x27_0010;

    pub(crate) const OC970_LOOKUP_RESULT: u32 = DATA + 0x27_0014;

    pub(crate) const OC970_LOOKUP_MAGIC: u32 = 0x0970_0002;

    pub(crate) const OC970_PLAY_CTRL: u32 = DATA + 0x27_0020;

    pub(crate) const OC970_PLAY_RESULT: u32 = DATA + 0x27_0024;

    pub(crate) const OC970_PLAY_MAGIC: u32 = 0x0970_0003;

    pub(crate) const OC970_BE_CTRL: u32 = DATA + 0x27_0030;

    pub(crate) const OC970_BE_RESULT: u32 = DATA + 0x27_0034;

    pub(crate) const OC970_BE_MAGIC: u32 = 0x0970_0004;

    pub(crate) const OC970_IFACE_CTRL: u32 = DATA + 0x27_0040;

    pub(crate) const OC970_IFACE_RESULT: u32 = DATA + 0x27_0044;

    pub(crate) const OC970_IFACE_MAGIC: u32 = 0x0970_0005;

    pub(crate) const OC970_LEVEL_CTRL: u32 = DATA + 0x27_0050;

    pub(crate) const OC970_LEVEL_RESULT: u32 = DATA + 0x27_0054;

    pub(crate) const OC970_LEVEL_MAGIC: u32 = 0x0970_0006;

    pub(crate) const OC970_MONEY_CTRL: u32 = DATA + 0x27_0060;

    pub(crate) const OC970_MONEY_RESULT: u32 = DATA + 0x27_0064;

    pub(crate) const OC970_MONEY_MAGIC: u32 = 0x0970_0007;

    pub(crate) const OC970_SETTEXT_CTRL: u32 = DATA + 0x27_0070;

    pub(crate) const OC970_SETTEXT_RESULT: u32 = DATA + 0x27_0074;

    pub(crate) const OC970_SETTEXT_MAGIC: u32 = 0x0970_0008;

    /// One gated fake: with its gate word set it returns at once with V0 = the
    /// forced result word (0 included) and charges a fixed cost; with the gate
    /// off it runs the body the slot held before (`fall_back`).
    macro_rules! oc970_fake {
        ($fake:ident, $prev:ident, $ctrl:expr, $magic:expr, $result:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32($ctrl) != $magic {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                c.g[V0][0] = c.bus.read32($result) as u64;
                c.cycles += 5;
                c.pc = c.g[RA][0] as u32;
            }
        };
    }

    oc970_fake!(
        oc970_fake_find,
        OC970_PREV_FIND,
        OC970_FIND_CTRL,
        OC970_FIND_MAGIC,
        OC970_FIND_RESULT,
        ee::s13::f_0039d860__generated
    );

    oc970_fake!(
        oc970_fake_lookup,
        OC970_PREV_LOOKUP,
        OC970_LOOKUP_CTRL,
        OC970_LOOKUP_MAGIC,
        OC970_LOOKUP_RESULT,
        ee::s13::f_0039c778__generated
    );

    oc970_fake!(
        oc970_fake_play,
        OC970_PREV_PLAY,
        OC970_PLAY_CTRL,
        OC970_PLAY_MAGIC,
        OC970_PLAY_RESULT,
        ee::s13::f_0039c870__generated
    );

    oc970_fake!(
        oc970_fake_be,
        OC970_PREV_BE,
        OC970_BE_CTRL,
        OC970_BE_MAGIC,
        OC970_BE_RESULT,
        ee::s01::f_0014dc80__generated
    );

    oc970_fake!(
        oc970_fake_iface,
        OC970_PREV_IFACE,
        OC970_IFACE_CTRL,
        OC970_IFACE_MAGIC,
        OC970_IFACE_RESULT,
        ee::s01::f_0014dd58__generated
    );

    oc970_fake!(
        oc970_fake_level,
        OC970_PREV_LEVEL,
        OC970_LEVEL_CTRL,
        OC970_LEVEL_MAGIC,
        OC970_LEVEL_RESULT,
        ee::s01::f_00150e50__generated
    );

    oc970_fake!(
        oc970_fake_money,
        OC970_PREV_MONEY,
        OC970_MONEY_CTRL,
        OC970_MONEY_MAGIC,
        OC970_MONEY_RESULT,
        ee::s03::f_00198af0__generated
    );

    oc970_fake!(
        oc970_fake_settext,
        OC970_PREV_SETTEXT,
        OC970_SETTEXT_CTRL,
        OC970_SETTEXT_MAGIC,
        OC970_SETTEXT_RESULT,
        ee::s13::f_003a0c50__generated
    );

    pub(crate) fn oc970_install() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc970_fake_find,
            &OC970_PREV_FIND,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C778),
            oc970_fake_lookup,
            &OC970_PREV_LOOKUP,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc970_fake_play,
            &OC970_PREV_PLAY,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc970_fake_be,
            &OC970_PREV_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc970_fake_iface,
            &OC970_PREV_IFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00150E50),
            oc970_fake_level,
            &OC970_PREV_LEVEL,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_00198AF0),
            oc970_fake_money,
            &OC970_PREV_MONEY,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc970_fake_settext,
            &OC970_PREV_SETTEXT,
        );
    }

    /// Arm the gated fakes for one fixture. `forced` is, in order: the object
    /// UIScreen_FindObject returns, the value UIScreen_LookupValue returns, the
    /// BE object (cBE_getBE), the BE interface, the attribute cost
    /// (BEEconInt_GetAttributeCost) and the money string (FE_FormatMoney).
    /// A zero result is forced as zero.
    pub(crate) fn oc970_arm(c: &mut Cpu, forced: [u32; 6]) {
        let [find, lookup, be, iface, level, money] = forced;
        c.bus.write32(OC970_FIND_CTRL, OC970_FIND_MAGIC);
        c.bus.write32(OC970_FIND_RESULT, find);
        c.bus.write32(OC970_LOOKUP_CTRL, OC970_LOOKUP_MAGIC);
        c.bus.write32(OC970_LOOKUP_RESULT, lookup);
        c.bus.write32(OC970_PLAY_CTRL, OC970_PLAY_MAGIC);
        c.bus.write32(OC970_PLAY_RESULT, 0);
        c.bus.write32(OC970_BE_CTRL, OC970_BE_MAGIC);
        c.bus.write32(OC970_BE_RESULT, be);
        c.bus.write32(OC970_IFACE_CTRL, OC970_IFACE_MAGIC);
        c.bus.write32(OC970_IFACE_RESULT, iface);
        c.bus.write32(OC970_LEVEL_CTRL, OC970_LEVEL_MAGIC);
        c.bus.write32(OC970_LEVEL_RESULT, level);
        c.bus.write32(OC970_MONEY_CTRL, OC970_MONEY_MAGIC);
        c.bus.write32(OC970_MONEY_RESULT, money);
        c.bus.write32(OC970_SETTEXT_CTRL, OC970_SETTEXT_MAGIC);
        c.bus.write32(OC970_SETTEXT_RESULT, 0);
    }

    #[test]
    fn oc970_update_total_cost_matches() {
        oc970_install();
        check(
            ee::s04::f_001f5240__generated,
            oc970_update_total_cost,
            40,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_5240, 3);
                elf_image(c);
                let this = DATA;
                c.bus.write32(this + 0x48, DATA + 0x1000);
                c.bus.write32(this + 0xa0, seed * 37);
                let text = if seed % 2 == 0 { 0 } else { DATA + 0x3000 };
                let money = if seed % 3 == 0 { 0 } else { DATA + 0x5000 };
                oc970_arm(c, [text, 0, 0, 0, 0, money]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc970_update_level_text_matches() {
        oc970_install();
        check(
            ee::s04::f_001f4ea8__generated,
            oc970_update_level_text,
            40,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_4ea8, 4);
                elf_image(c);
                let this = DATA;
                for i in 0..7u32 {
                    c.bus.write32(this + 0x4c + 4 * i, (seed + 3 * i) % 23);
                    c.bus.write32(this + 0x68 + 4 * i, (seed + i) % 4);
                }
                c.bus.write32(this + 0x48, DATA + 0x1000);
                let text = if seed % 3 == 0 { 0 } else { DATA + 0x3000 };
                oc970_arm(c, [text, 0, 0, 0, 0, 0]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc970_update_cost_text_matches() {
        oc970_install();
        check(
            ee::s04::f_001f4fc0__generated,
            oc970_update_cost_text,
            40,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_4fc0, 4);
                elf_image(c);
                let this = DATA;
                for i in 0..7u32 {
                    c.bus.write32(this + 0x68 + 4 * i, (seed + 5 * i) % 14);
                }
                c.bus.write32(this + 0x48, DATA + 0x1000);
                let text = if seed % 3 == 0 { 0 } else { DATA + 0x3000 };
                let be = if seed % 5 == 0 { 0 } else { DATA + 0x6000 };
                let level = if seed % 4 == 0 { 0 } else { seed % 20 };
                oc970_arm(c, [text, 0, be, be, level, DATA + 0x5000]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc970_update_bars_matches() {
        oc970_install();
        check(
            ee::s04::f_001f50b0__generated,
            oc970_update_bars,
            40,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_50b0, 4);
                elf_image(c);
                let this = DATA;
                for i in 0..7u32 {
                    c.bus.write32(this + 0x4c + 4 * i, (seed + 2 * i) % 30);
                    c.bus.write32(this + 0x84 + 4 * i, (seed + i) % 9);
                }
                c.bus.write32(this + 0x48, DATA + 0x1000);
                let text = if seed % 4 == 0 { 0 } else { DATA + 0x3000 };
                oc970_arm(c, [text, 0, 0, 0, 0, 0]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc970_on_widget_action_matches() {
        oc970_install();
        check(
            ee::s04::f_001f4a90__generated,
            oc970_on_widget_action,
            60,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_4a90, 4);
                elf_image(c);
                let this = DATA;
                let obj = if seed % 7 == 0 { 0 } else { DATA + 0x1000 };
                let engine = DATA + 0x2000;
                let sound = DATA + 0x2400;
                c.bus.write32(this + 0x10, engine);
                c.bus
                    .write32(engine + 0x14, if seed % 3 == 0 { 0 } else { sound });
                c.bus.write32(this + 0xa0, 10 + seed % 4);
                c.bus.write32(this + 0xa4, seed % 15);
                c.bus.write32(this + 0xa8, seed % 2);
                c.bus.write32(this + 0xac, DATA + 0x2800);
                c.bus.write32(this + 0x48, DATA + 0x1000);
                let text = if seed % 2 == 0 { 0 } else { DATA + 0x3000 };
                oc970_arm(c, [text, 0, 0, 0, 0, 0]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(obj);
                c.g[A2][0] = [5u64, 6, 7][(seed % 3) as usize];
            },
        );
    }
}
