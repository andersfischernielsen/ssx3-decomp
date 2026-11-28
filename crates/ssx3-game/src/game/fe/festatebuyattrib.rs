//! `src/fe/festatebuyattrib.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::fepopupmisc::transition_in;
pub(crate) use crate::game::fe::festatelodge::fe_state_ctor_2_body;
pub(crate) use crate::game::fe::festatetrophyroom::handle_key_6_8_9;
pub(crate) use crate::game::fe::festatetrophyroom::handle_list_key;
pub(crate) use crate::game::fe::ovstateenterlodge::oc989_at;
pub(crate) use crate::game::fe::ovstatehud::oc1002_vtable_dtor;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovstatepause::oc1023_at;
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
    (
        0x001f_56c8,
        cfe_state_career_stats_handle_input,
        "cFEStateCareerStats_HandleInput",
    ),
    (
        0x001f_5a38,
        oc989_career_stats_show_page,
        "cFEStateCareerStats_ShowPage",
    ),
    (
        0x001f_5ba8,
        cfe_state_career_stats_select_page,
        "cFEStateCareerStats_SelectPage",
    ),
    (
        0x001f_5da0,
        cfe_state_career_stats_show_highlights_page,
        "cFEStateCareerStats_ShowHighlightsPage",
    ),
    (
        0x001f_60e0,
        cfe_state_career_stats_show_race_page,
        "cFEStateCareerStats_ShowRacePage",
    ),
    (
        0x001f_6490,
        cfe_state_career_stats_show_freestyle_page,
        "cFEStateCareerStats_ShowFreestylePage",
    ),
    (
        0x001f_6890,
        c_fe_state_career_stats_show_records_page,
        "cFEStateCareerStats_ShowRecordsPage",
    ),
    (0x001f_6f30, count_medals, "cFEStateCareerStats_CountMedals"),
    (
        0x0022_0e30,
        fe_state_buy_attrib_dtor,
        "cFEStateBuyAttrib_dtor",
    ),
    (
        0x0022_0e90,
        cfe_state_career_stats_dtor,
        "cFEStateCareerStats_dtor",
    ),
    (
        0x001f_52a0,
        oc1023_buy_attrib_update_money,
        "cFEStateBuyAttrib_UpdateMoney",
    ),
    (
        0x001f_5300,
        oc1023_buy_attrib_commit_purchase,
        "cFEStateBuyAttrib_CommitPurchase",
    ),
    (
        0x001f_5508,
        oc1023_career_stats_init,
        "cFEStateCareerStats_Init",
    ),
    (
        0x0022_0df8,
        cfe_state_buy_attrib_ctor_2,
        "cFEStateBuyAttrib_ctor_2",
    ),
    (
        0x0022_0e58,
        cfe_state_career_stats_ctor_2,
        "cFEStateCareerStats_ctor_2",
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

// Offsets of the 0x54-byte riders-best record copied to `sp + 0x70`.
pub(crate) const SC_REC: u32 = 0x70;

/// `sprintf(buf, fmt, ...)` (0x417828) returning to `ret`.
pub(crate) fn sc_sprintf(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    call_ptr(c, 0x0041_7828, ret)
}

/// `cFEStateCareerStats_SetText(self, name, text)` (0x1f6840).
pub(crate) fn sc_set_text(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    call_ptr(c, 0x001f_6840, ret)
}

pub(crate) fn sc_add(c: &Cpu, reg: usize, off: u32) -> u64 {
    sx((c.g[reg][0] as u32).wrapping_add(off))
}

pub(crate) fn sc_sub(c: &Cpu, reg: usize, off: u32) -> u64 {
    sx((c.g[reg][0] as u32).wrapping_sub(off))
}

/// One text row, the record at `sp + 0x70 + ..` has `n > 0`: format `n`
/// with `fmt`, set the value text, look up the rider name (`s3`, id in `s1`)
/// and set the name text. `name_a`/`name_b` set the label address registers.
pub(crate) fn sc_row_positive(
    c: &mut Cpu,
    cycles: u64,
    fmt: u32,
    rets: [u32; 5],
    name_a: &dyn Fn(&mut Cpu),
    name_b: &dyn Fn(&mut Cpu),
) -> crate::call::Flow {
    c.cycles += cycles;
    c.g[A1][0] = 0x4a0000;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = sc_add(c, A1, fmt);
    sc_sprintf(c, rets[0])?;
    name_a(c);
    c.g[A2][0] = c.g[SP][0];
    c.g[A0][0] = c.g[S2][0];
    sc_set_text(c, rets[1])?;
    sc_name_and_set(c, rets[2], rets[3], rets[4], name_b)
}

/// The shared tail of a positive row: rider name lookup, format, set text.
pub(crate) fn sc_name_and_set(
    c: &mut Cpu,
    ret_name: u32,
    ret_fmt: u32,
    ret_set: u32,
    name_b: &dyn Fn(&mut Cpu),
) -> crate::call::Flow {
    c.g[S0][0] = 0x4a0000;
    c.g[A1][0] = c.g[S1][0];
    c.g[S0][0] = sc_add(c, S0, 0x23c0);
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0014_4c60, ret_name)?;
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[SP][0];
    sc_sprintf(c, ret_fmt)?;
    name_b(c);
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.g[SP][0];
    sc_set_text(c, ret_set)
}

/// One text row with `n <= 0`: "none" placeholders for both texts.
pub(crate) fn sc_row_empty(
    c: &mut Cpu,
    cycles: u64,
    rets: [u32; 2],
    name_a: &dyn Fn(&mut Cpu),
    name_b: &dyn Fn(&mut Cpu),
) -> crate::call::Flow {
    c.cycles += cycles;
    c.g[A2][0] = 0x4a0000;
    name_a(c);
    c.g[A2][0] = sc_add(c, A2, 0x2108);
    c.g[A0][0] = c.g[S2][0];
    sc_set_text(c, rets[0])?;
    c.g[A2][0] = 0x4a0000;
    name_b(c);
    c.g[A2][0] = sc_add(c, A2, 0x2168);
    c.g[A0][0] = c.g[S2][0];
    sc_set_text(c, rets[1])
}

/// Row prologue: `a2` holds `n`; load the rider id into `s1` and branch.
pub(crate) fn sc_row(
    c: &mut Cpu,
    pre_cycles: u64,
    id_off: u32,
    positive: &dyn Fn(&mut Cpu) -> crate::call::Flow,
    empty: &dyn Fn(&mut Cpu) -> crate::call::Flow,
) -> crate::call::Flow {
    c.cycles += pre_cycles;
    let none = (c.g[A2][0] as i64) <= 0;
    c.g[S1][0] = c.lw(sp(c).wrapping_add(id_off));
    if none {
        empty(c)
    } else {
        positive(c)
    }
}

/// `void cFEStateCareerStats_ShowRecordsPage(cFEStateCareerStats* self)`
/// (0x1f6890): copy the riders-best record of the current character (BE
/// interface 8) to the stack and fill the records page: for each of seven
/// records (and the last "-"/count one) the value text and the name of the
/// rider that holds it ("none" placeholders when the count is 0).
pub fn c_fe_state_career_stats_show_records_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x170);
        frame.save(c, 18, 0x140);
        frame.save(c, 16, 0x160);
        frame.save(c, 19, 0x130);
        frame.save(c, 17, 0x150);
        frame.save(c, 20, 0x120);
        frame.save(c, 21, 0x110);
        frame.save(c, 22, 0x100);
        frame.save(c, 23, 0xf0);
        frame.save(c, 30, 0xe0);
        frame.save_ra(c, 0xd0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x001f_68c4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x001f_68d0)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x001f_68d8)?;
        c.g[A1][0] = 8;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x001f_68e4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_a080, 0x001f_68f0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0015_5288, 0x001f_6900)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = c.g[V1][0] & 7;
        let aligned = c.g[V0][0] == 0;
        c.g[A0][0] = sc_add(c, SP, SC_REC);
        // Copy the first 0x40 bytes of the record.
        c.cycles += 7;
        c.g[V0][0] = sc_add(c, V1, 0x40);
        c.g[S0][0] = 0x4a0000;
        c.g[S4][0] = 0x470000;
        c.g[S5][0] = 0x470000;
        c.g[S6][0] = 0x470000;
        c.g[S7][0] = 0x4a0000;
        c.g[FP][0] = 0x470000;
        if !aligned {
            loop {
                c.cycles += 19;
                let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
                for (k, reg) in [5usize, 6, 7, 8].into_iter().enumerate() {
                    let o = 8 * k as u32;
                    c.ldl(reg, src.wrapping_add(o + 7));
                    c.ldr(reg, src.wrapping_add(o));
                }
                for (k, reg) in [5usize, 6, 7, 8].into_iter().enumerate() {
                    let o = 8 * k as u32;
                    c.sdl(reg, dst.wrapping_add(o + 7));
                    c.sdr(reg, dst.wrapping_add(o));
                }
                c.g[V1][0] = sc_add(c, V1, 0x20);
                let again = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sc_add(c, A0, 0x20);
                if !again {
                    c.cycles += 2;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001f_692c);
                }
            }
        } else {
            loop {
                c.cycles += 11;
                let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
                c.g[A1][0] = c.ld(src);
                c.g[A2][0] = c.ld(src.wrapping_add(8));
                c.g[A3][0] = c.ld(src.wrapping_add(0x10));
                c.g[T0][0] = c.ld(src.wrapping_add(0x18));
                c.sd(dst, c.g[A1][0]);
                c.sd(dst.wrapping_add(8), c.g[A2][0]);
                c.sd(dst.wrapping_add(0x10), c.g[A3][0]);
                c.sd(dst.wrapping_add(0x18), c.g[T0][0]);
                c.g[V1][0] = sc_add(c, V1, 0x20);
                let again = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sc_add(c, A0, 0x20);
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001f_699c);
                }
            }
        }
        // The last 0x14 bytes.
        c.cycles += 13;
        let (src, dst) = (c.g[V1][0] as u32, c.g[A0][0] as u32);
        c.ldl(2, src.wrapping_add(7));
        c.ldr(2, src);
        c.ldl(5, src.wrapping_add(0xf));
        c.ldr(5, src.wrapping_add(8));
        c.g[A2][0] = c.lw(src.wrapping_add(0x10));
        c.sdl(2, dst.wrapping_add(7));
        c.sdr(2, dst);
        c.sdl(5, dst.wrapping_add(0xf));
        c.sdr(5, dst.wrapping_add(8));
        c.sw(dst.wrapping_add(0x10), c.g[A2][0]);
        c.g[A2][0] = c.lw(sp(c).wrapping_add(SC_REC));

        // Row 1 (n at +0x70, id +0x74).
        sc_row(
            c,
            0,
            0x74,
            &|c| {
                sc_row_positive(
                    c,
                    23,
                    0x2138,
                    [
                        0x001f_6a0c,
                        0x001f_6a1c,
                        0x001f_6a30,
                        0x001f_6a40,
                        0x001f_6a50,
                    ],
                    &|c| c.g[A1][0] = sc_add(c, S0, 0x23b8),
                    &|c| c.g[A1][0] = sc_sub(c, S4, 0xc48),
                )
            },
            &|c| {
                sc_row_empty(
                    c,
                    11,
                    [0x001f_6a6c, 0x001f_6a80],
                    &|c| c.g[A1][0] = sc_add(c, S0, 0x23b8),
                    &|c| c.g[A1][0] = sc_sub(c, S4, 0xc48),
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x98));
        // Row 2 (n +0x98, id +0x9c).
        sc_row(
            c,
            2,
            0x9c,
            &|c| {
                sc_row_positive(
                    c,
                    23,
                    0x23c8,
                    [
                        0x001f_6a9c,
                        0x001f_6aac,
                        0x001f_6ac0,
                        0x001f_6ad0,
                        0x001f_6ae0,
                    ],
                    &|c| c.g[A1][0] = sc_sub(c, S5, 0xc38),
                    &|c| c.g[A1][0] = sc_sub(c, S6, 0xc28),
                )
            },
            &|c| {
                sc_row_empty(
                    c,
                    11,
                    [0x001f_6afc, 0x001f_6b10],
                    &|c| c.g[A1][0] = sc_sub(c, S5, 0xc38),
                    &|c| c.g[A1][0] = sc_sub(c, S6, 0xc28),
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x78));
        // Row 3 (n +0x78, id +0x7c).
        sc_row(
            c,
            2,
            0x7c,
            &|c| {
                sc_row_positive(
                    c,
                    23,
                    0x2138,
                    [
                        0x001f_6b2c,
                        0x001f_6b3c,
                        0x001f_6b50,
                        0x001f_6b60,
                        0x001f_6b70,
                    ],
                    &|c| c.g[A1][0] = sc_add(c, S7, 0x23d0),
                    &|c| c.g[A1][0] = sc_sub(c, FP, 0xc10),
                )
            },
            &|c| {
                sc_row_empty(
                    c,
                    11,
                    [0x001f_6b8c, 0x001f_6ba0],
                    &|c| c.g[A1][0] = sc_add(c, S7, 0x23d0),
                    &|c| c.g[A1][0] = sc_sub(c, FP, 0xc10),
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x80));
        // Row 4 (n +0x80, id +0x84).
        sc_row(
            c,
            2,
            0x84,
            &|c| {
                sc_row_positive(
                    c,
                    25,
                    0x2138,
                    [
                        0x001f_6bbc,
                        0x001f_6bd0,
                        0x001f_6be4,
                        0x001f_6bf4,
                        0x001f_6c08,
                    ],
                    &|c| {
                        c.g[T0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, T0, 0xc00);
                    },
                    &|c| {
                        c.g[V0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, V0, 0xbf0);
                    },
                )
            },
            &|c| {
                sc_row_empty(
                    c,
                    13,
                    [0x001f_6c28, 0x001f_6c40],
                    &|c| {
                        c.g[A3][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, A3, 0xc00);
                    },
                    &|c| {
                        c.g[T0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, T0, 0xbf0);
                    },
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x88));
        // Row 5 (n +0x88, id +0x8c).
        sc_row(
            c,
            2,
            0x8c,
            &|c| {
                sc_row_positive(
                    c,
                    25,
                    0x2138,
                    [
                        0x001f_6c5c,
                        0x001f_6c70,
                        0x001f_6c84,
                        0x001f_6c94,
                        0x001f_6ca8,
                    ],
                    &|c| {
                        c.g[V0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, V0, 0xbd8);
                    },
                    &|c| {
                        c.g[A2][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, A2, 0xbc8);
                    },
                )
            },
            &|c| {
                sc_row_empty(
                    c,
                    13,
                    [0x001f_6cc8, 0x001f_6ce0],
                    &|c| {
                        c.g[A3][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, A3, 0xbd8);
                    },
                    &|c| {
                        c.g[T0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, T0, 0xbc8);
                    },
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0xa0));
        // Row 6 (n +0xa0, id +0xa4): a percentage, n * 0.01 through 0x413af8.
        sc_row(
            c,
            2,
            0xa4,
            &|c| {
                c.cycles += 32;
                c.mtc1(6, 0);
                c.cvt_s_w(0, 0);
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5548));
                c.g[S0][0] = 0x470000;
                c.g[S0][0] = sc_sub(c, S0, 0xbb0);
                c.mul_s(12, 0, 12);
                call_ptr(c, 0x0041_3af8, 0x001f_6d08)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                sc_sprintf(c, 0x001f_6d18)?;
                c.g[V0][0] = 0x470000;
                c.g[A2][0] = c.g[SP][0];
                c.g[A1][0] = sc_sub(c, V0, 0xba0);
                c.g[A0][0] = c.g[S2][0];
                sc_set_text(c, 0x001f_6d2c)?;
                sc_name_and_set(c, 0x001f_6d40, 0x001f_6d50, 0x001f_6d64, &|c| {
                    c.g[A2][0] = 0x470000;
                    c.g[A1][0] = sc_sub(c, A2, 0xb90);
                })
            },
            &|c| {
                sc_row_empty(
                    c,
                    13,
                    [0x001f_6d84, 0x001f_6d9c],
                    &|c| {
                        c.g[A3][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, A3, 0xba0);
                    },
                    &|c| {
                        c.g[T0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, T0, 0xb90);
                    },
                )
            },
        )?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0xa8));
        // Row 7 (n +0xa8, id +0xac).
        sc_row(
            c,
            2,
            0xac,
            &|c| {
                c.cycles += 25;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sc_add(c, A1, 0x23c8);
                sc_sprintf(c, 0x001f_6db8)?;
                c.g[V0][0] = 0x4a0000;
                c.g[A2][0] = c.g[SP][0];
                c.g[A1][0] = sc_add(c, V0, 0x23d8);
                c.g[A0][0] = c.g[S2][0];
                sc_set_text(c, 0x001f_6dcc)?;
                c.g[S0][0] = 0x4a0000;
                c.g[A0][0] = c.g[S3][0];
                c.g[S0][0] = sc_add(c, S0, 0x23c0);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0014_4c60, 0x001f_6de0)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[SP][0];
                sc_sprintf(c, 0x001f_6df0)?;
                c.g[A2][0] = 0x470000;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = sc_sub(c, A2, 0xb80);
                c.g[A2][0] = c.g[SP][0];
                sc_set_text(c, 0x001f_6e04)
            },
            &|c| {
                sc_row_empty(
                    c,
                    13,
                    [0x001f_6e24, 0x001f_6e3c],
                    &|c| {
                        c.g[A3][0] = 0x4a0000;
                        c.g[A1][0] = sc_add(c, A3, 0x23d8);
                    },
                    &|c| {
                        c.g[T0][0] = 0x470000;
                        c.g[A1][0] = sc_sub(c, T0, 0xb80);
                    },
                )
            },
        )?;
        c.g[V0][0] = 0x470000;
        // Last row: set the title text, then the "n" / "-" text.
        c.cycles += 15;
        c.g[A0][0] = sc_sub(c, V0, 0xb70);
        call_ptr(c, 0x0031_7670, 0x001f_6e48)?;
        c.g[A0][0] = c.lw(p_off(c.g[S2][0], 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_6e54)?;
        c.g[A1][0] = 0x470000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sc_sub(c, A1, 0xb58);
        call_ptr(c, 0x0031_7670, 0x001f_6e64)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x001f_6e70)?;
        c.g[A2][0] = c.lw(sp(c).wrapping_add(0x90));
        let none = (c.g[A2][0] as i64) <= 0;
        c.g[A1][0] = 0x4a0000;
        if none {
            c.cycles += 4;
            c.g[V0][0] = 0x4a0000;
            c.g[V1][0] = c.lhu(sc_add(c, V0, 0x2108) as u32);
            let buf = sp(c);
            c.sh(buf, c.g[V1][0]);
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sc_add(c, A1, 0x2138);
            sc_sprintf(c, 0x001f_6e88)?;
        }
        c.g[A2][0] = 0x470000;
        c.cycles += 16;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sc_sub(c, A2, 0xb40);
        c.g[A2][0] = c.g[SP][0];
        sc_set_text(c, 0x001f_6eb0)?;
        frame.restore(c, 16, 0x160);
        frame.restore(c, 17, 0x150);
        frame.restore(c, 18, 0x140);
        frame.restore(c, 19, 0x130);
        frame.restore(c, 20, 0x120);
        frame.restore(c, 21, 0x110);
        frame.restore(c, 22, 0x100);
        frame.restore(c, 23, 0xf0);
        frame.restore(c, 30, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) fn p_off(a: u64, off: u32) -> u32 {
    (a as u32).wrapping_add(off)
}

/// `bool cFEStateCareerStats_HandleInput(self, cInputEvent* w)` (0x1f56c8): the career
/// stats screen's input. Left (19 on the widget's `want` slot at +0x98) / right (20, +0xa0)
/// step the page `self->mStatsState` (+0x48) through the four pages (wrapping 0..3), play
/// the "hll"/"hlr" frame and redraw (`SelectPage`, `ShowPage`). Up (17, +0x88) / down (18,
/// +0x90) scroll the highlights (+0x4c, 24 rows, 3 visible), race (+0x50, `count` +0x58, 5
/// visible) or freestyle (+0x54, `count` +0x68) list one row, replay the matching frame and
/// redraw that page. Finally the UI sound object (`engine+0x14`, else the null sound at
/// `gp+0x2968`) plays sound 1 (page), 2 (scrolled), 4 (at the end) or 0. Returns whether
/// anything changed.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn cfe_state_career_stats_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f57cc: {
            'b_1f5764: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S3][0] = 0u64;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001f5704);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001f5704)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    break 'b_1f5764;
                } else {
                    'b_1f5744: {
                        c.cycles += 10;
                        c.g[A0][0] = 0x4a0000;
                        c.g[RA][0] = sx(0x001f5718);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2358));
                        call_ptr(c, 0x00317670, 0x001f5718)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001f5724);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039c778, 0x001f5724)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        if c.g[A1][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                            break 'b_1f5744;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f5740);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039c870, 0x001f5740)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                            break 'b_1f5744;
                        }
                    }
                    c.cycles += 5;
                    c.g[S2][0] = 0x1;
                    c.g[S3][0] = 0x1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                    if t {
                        break 'b_1f57cc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                        if t {
                            break 'b_1f57cc;
                        } else {
                            break 'b_1f5764;
                        }
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001f5774);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001f5774)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0x4a0000;
            if t {
                break 'b_1f57cc;
            } else {
                'b_1f57b0: {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001f5784);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2360));
                    call_ptr(c, 0x00317670, 0x001f5784)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f5790);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039c778, 0x001f5790)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    if c.g[A1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                        break 'b_1f57b0;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001f57ac);
                        c.g[A2][0] = 0x1;
                        call_ptr(c, 0x0039c870, 0x001f57ac)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                        break 'b_1f57b0;
                    }
                }
                c.cycles += 6;
                c.g[S2][0] = 0x1;
                c.g[S3][0] = 0x1;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
                let t = c.g[V1][0] != 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                if t {
                    break 'b_1f57cc;
                } else {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                    break 'b_1f57cc;
                }
            }
        }
        'b_1f57ec: {
            c.cycles += 2;
            if c.g[S3][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                break 'b_1f57ec;
            } else {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                c.g[RA][0] = sx(0x001f57e0);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001f5ba8,
                    f: ee::s04::cFEStateCareerStats_SelectPage,
                }
                .call(c, 0x001f57e0)?;
                c.g[RA][0] = sx(0x001f57e8);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001f5a38,
                    f: ee::s04::cFEStateCareerStats_ShowPage,
                }
                .call(c, 0x001f57e8)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                break 'b_1f57ec;
            }
        }
        'b_1f59ec: {
            'b_1f59e8: {
                'b_1f59e4: {
                    'b_1f59ac: {
                        'b_1f5880: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_1f5880;
                            } else {
                                'b_1f5838: {
                                    c.cycles += 7;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001f5808);
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001f5808)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_1f5838;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        if t {
                                            break 'b_1f59e4;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x4c),
                                                c.g[V0][0],
                                            );
                                            c.g[RA][0] = sx(0x001f582c);
                                            c.g[S3][0] = 0x1;
                                            Callee {
                                                addr: 0x001f5da0,
                                                f: ee::s04::cFEStateCareerStats_ShowHighlightsPage,
                                            }
                                            .call(c, 0x001f582c)?;
                                            c.g[A0][0] = 0x4a0000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x2368));
                                            if t {
                                                break 'b_1f59ac;
                                            } else {
                                                break 'b_1f5838;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f5848);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001f5848)?;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    break 'b_1f59ec;
                                } else {
                                    c.cycles += 5;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3));
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 24i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'b_1f59e4;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001f5874);
                                        c.g[S3][0] = 0x1;
                                        Callee {
                                            addr: 0x001f5da0,
                                            f: ee::s04::cFEStateCareerStats_ShowHighlightsPage,
                                        }
                                        .call(c, 0x001f5874)?;
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xe88));
                                        if t {
                                            break 'b_1f59ac;
                                        } else {
                                            break 'b_1f5880;
                                        }
                                    }
                                }
                            }
                        }
                        'b_1f5918: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_1f5918;
                            } else {
                                'b_1f58cc: {
                                    c.cycles += 7;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001f589c);
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001f589c)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_1f58cc;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        if t {
                                            break 'b_1f59e4;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x50),
                                                c.g[V0][0],
                                            );
                                            c.g[RA][0] = sx(0x001f58c0);
                                            c.g[S3][0] = 0x1;
                                            Callee {
                                                addr: 0x001f60e0,
                                                f: ee::s04::cFEStateCareerStats_ShowRacePage,
                                            }
                                            .call(c, 0x001f58c0)?;
                                            c.g[A0][0] = 0x4a0000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x2370));
                                            if t {
                                                break 'b_1f59ac;
                                            } else {
                                                break 'b_1f58cc;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f58dc);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001f58dc)?;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    break 'b_1f59ec;
                                } else {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5));
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'b_1f59e4;
                                    } else {
                                        c.cycles += 7;
                                        c.g[S3][0] = 0x1;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001f590c);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                                        Callee {
                                            addr: 0x001f60e0,
                                            f: ee::s04::cFEStateCareerStats_ShowRacePage,
                                        }
                                        .call(c, 0x001f590c)?;
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xe78));
                                        if t {
                                            break 'b_1f59ac;
                                        } else {
                                            break 'b_1f5918;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            break 'b_1f59ec;
                        } else {
                            'b_1f5964: {
                                c.cycles += 7;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f5934);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001f5934)?;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_1f5964;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    if t {
                                        break 'b_1f59e4;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001f5958);
                                        c.g[S3][0] = 0x1;
                                        Callee {
                                            addr: 0x001f6490,
                                            f: ee::s04::cFEStateCareerStats_ShowFreestylePage,
                                        }
                                        .call(c, 0x001f5958)?;
                                        c.g[A0][0] = 0x4a0000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2378));
                                        if t {
                                            break 'b_1f59ac;
                                        } else {
                                            break 'b_1f5964;
                                        }
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001f5974);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001f5974)?;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                break 'b_1f59ec;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1f59e4;
                                } else {
                                    c.cycles += 6;
                                    c.g[S3][0] = 0x1;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x001f59a4);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                                    Callee {
                                        addr: 0x001f6490,
                                        f: ee::s04::cFEStateCareerStats_ShowFreestylePage,
                                    }
                                    .call(c, 0x001f59a4)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xe68));
                                    break 'b_1f59ac;
                                }
                            }
                        }
                    }
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001f59b4);
                    call_ptr(c, 0x00317670, 0x001f59b4)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f59c0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039c778, 0x001f59c0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[S2][0] = 0x2;
                    if t {
                        break 'b_1f59e8;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001f59dc);
                        c.g[A2][0] = 0x1;
                        call_ptr(c, 0x0039c870, 0x001f59dc)?;
                        let t = true;
                        c.g[S2][0] = 0x2;
                        if t {
                            break 'b_1f59e8;
                        } else {
                            break 'b_1f59e4;
                        }
                    }
                }
                c.cycles += 1;
                c.g[S2][0] = 0x4;
                break 'b_1f59e8;
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            break 'b_1f59ec;
        }
        'b_1f5a14: {
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            if c.g[A2][0] == 0 {
                c.g[A2][0] = c.g[V1][0];
            }
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S2][0];
            if t {
                break 'b_1f5a14;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5a14);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001f5a14)?;
                break 'b_1f5a14;
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCareerStats_SelectPage(self, int menu)` (0x1f5ba8, `setupMenuFocus`):
/// stores the page (`self+0x48`), looks the four tab widgets up by hash name and takes
/// the focus off each (virtual `+0x38/+0x3c` with 0), then gives the focus to the
/// selected page's tab and builds its page (highlights, race, freestyle or records).
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn cfe_state_career_stats_select_page(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f5c04: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = 0x470000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xdf0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x001f5bd8);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x48), c.g[A1][0]);
            call_ptr(c, 0x00317670, 0x001f5bd8)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001f5be4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001f5be4)?;
            c.g[S4][0] = c.g[V0][0];
            let t = c.g[S4][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1f5c04;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5c04);
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001f5c04)?;
                break 'b_1f5c04;
            }
        }
        'b_1f5c3c: {
            c.cycles += 9;
            c.g[A0][0] = 0x470000;
            c.g[RA][0] = sx(0x001f5c10);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xde0));
            call_ptr(c, 0x00317670, 0x001f5c10)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001f5c1c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001f5c1c)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1f5c3c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5c3c);
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001f5c3c)?;
                break 'b_1f5c3c;
            }
        }
        'b_1f5c74: {
            c.cycles += 9;
            c.g[A0][0] = 0x470000;
            c.g[RA][0] = sx(0x001f5c48);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xdc8));
            call_ptr(c, 0x00317670, 0x001f5c48)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001f5c54);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001f5c54)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1f5c74;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5c74);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001f5c74)?;
                break 'b_1f5c74;
            }
        }
        'b_1f5cac: {
            c.cycles += 9;
            c.g[A0][0] = 0x470000;
            c.g[RA][0] = sx(0x001f5c80);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xdb0));
            call_ptr(c, 0x00317670, 0x001f5c80)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001f5c8c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001f5c8c)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1f5cac;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5cac);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001f5cac)?;
                break 'b_1f5cac;
            }
        }
        'b_1f5d88: {
            'b_1f5d84: {
                'b_1f5d64: {
                    'b_1f5d3c: {
                        'b_1f5d14: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x48));
                            c.g[V0][0] = 0x1;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_1f5d14;
                            } else {
                                'b_1f5cec: {
                                    'b_1f5cd4: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_1f5cd4;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == 0u64;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_1f5cec;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_1f5d88;
                                                } else {
                                                    break 'b_1f5cd4;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_1f5d3c;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            break 'b_1f5d64;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_1f5d84;
                                            } else {
                                                break 'b_1f5cec;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 10;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001f5d04);
                                c.g[A0][0] =
                                    sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001f5d04)?;
                                c.g[RA][0] = sx(0x001f5d0c);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001f5da0,
                                    f: ee::s04::cFEStateCareerStats_ShowHighlightsPage,
                                }
                                .call(c, 0x001f5d0c)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_1f5d84;
                                } else {
                                    break 'b_1f5d14;
                                }
                            }
                        }
                        c.cycles += 10;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001f5d2c);
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001f5d2c)?;
                        c.g[RA][0] = sx(0x001f5d34);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001f60e0,
                            f: ee::s04::cFEStateCareerStats_ShowRacePage,
                        }
                        .call(c, 0x001f5d34)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        if t {
                            break 'b_1f5d84;
                        } else {
                            break 'b_1f5d3c;
                        }
                    }
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001f5d54);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001f5d54)?;
                    c.g[RA][0] = sx(0x001f5d5c);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001f6490,
                        f: ee::s04::cFEStateCareerStats_ShowFreestylePage,
                    }
                    .call(c, 0x001f5d5c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1f5d84;
                    } else {
                        break 'b_1f5d64;
                    }
                }
                c.cycles += 8;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5d78);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001f5d78)?;
                c.g[RA][0] = sx(0x001f5d80);
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x001f6890,
                    f: ee::s04::cFEStateCareerStats_ShowRecordsPage,
                }
                .call(c, 0x001f5d80)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_1f5d84;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1f5d88;
        }
        c.cycles += 6;
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

/// `void cFEStateCareerStats_ShowHighlightsPage(self)` (0x1f5da0, `setupHighlightsList`):
/// shows or hides the scroll arrows (`self+0x4c` > 0, `+3 < 24`) and fills the three
/// visible rows (`i + scroll`, level = row / 3, step = row % 3): the step's name, the
/// on/off marker from the score interface's current highlight level, the trick name and
/// the required-trick text (or the locked text).
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn cfe_state_career_stats_show_highlights_page(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f5e14: {
            'b_1f5e04: {
                c.cycles += 20;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x210));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x170), c.g[RA][0]);
                c.g[RA][0] = sx(0x001f5dd4);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x001f5dd4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f5de0);
                c.g[A1][0] = 0x8;
                call_ptr(c, 0x0014dd58, 0x001f5de0)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_1f5e04;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x470000;
                    c.g[RA][0] = sx(0x001f5dfc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd98));
                    call_ptr(c, 0x0039e948, 0x001f5dfc)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    if t {
                        break 'b_1f5e14;
                    } else {
                        break 'b_1f5e04;
                    }
                }
            }
            c.cycles += 4;
            c.g[A1][0] = 0x470000;
            c.g[RA][0] = sx(0x001f5e10);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd98));
            call_ptr(c, 0x0039e8b8, 0x001f5e10)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
            break 'b_1f5e14;
        }
        'b_1f5e48: {
            'b_1f5e38: {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 24i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_1f5e38;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x470000;
                    c.g[RA][0] = sx(0x001f5e30);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd88));
                    call_ptr(c, 0x0039e948, 0x001f5e30)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    if t {
                        break 'b_1f5e48;
                    } else {
                        break 'b_1f5e38;
                    }
                }
            }
            c.cycles += 4;
            c.g[A1][0] = 0x470000;
            c.g[RA][0] = sx(0x001f5e44);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd88));
            call_ptr(c, 0x0039e8b8, 0x001f5e44)?;
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_1f5e48;
        }
        c.cycles += 8;
        c.g[V1][0] = 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x168), c.g[V0][0]);
        c.g[FP][0] = 0x3;
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x16c), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
        'l_1f5e68: loop {
            'b_1f5e74: {
                c.cycles += 3;
                c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                if c.g[FP][0] == 0u64 {
                    break 'b_1f5e74;
                } else {
                    break 'b_1f5e74;
                }
            }
            'b_1f5f7c: {
                'b_1f5f78: {
                    c.cycles += 19;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S7][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S5][0] = c.g[A2][0];
                    c.div(0, 23, 30);
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2380));
                    c.g[S3][0] = c.lo[0];
                    c.g[S4][0] = c.hi[0];
                    c.g[RA][0] = sx(0x001f5e9c);
                    call_ptr(c, 0x00417828, 0x001f5e9c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001f5ea8);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x164), 0u64);
                    call_ptr(c, 0x00317670, 0x001f5ea8)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f5eb4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f5eb4)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V1][0] = 0x440000;
                    if t {
                        break 'b_1f5f78;
                    } else {
                        'b_1f5f64: {
                            c.cycles += 32;
                            c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1ba0));
                            c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[A0][0] = c.g[S6][0];
                            c.g[V1][0] = 0x4a0000;
                            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                            c.g[RA][0] = sx(0x001f5ee4);
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2388));
                            call_ptr(c, 0x00417828, 0x001f5ee4)?;
                            c.g[RA][0] = sx(0x001f5eec);
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x00317670, 0x001f5eec)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f5ef8);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x003a1000, 0x001f5ef8)?;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                            c.g[V0][0] = 0x470000;
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd78));
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001f5f10);
                            c.g[A2][0] = c.g[S5][0];
                            call_ptr(c, 0x00417828, 0x001f5f10)?;
                            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                            c.g[V1][0] = 0x470000;
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xd68));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001f5f28);
                            c.g[A2][0] = c.g[S5][0];
                            call_ptr(c, 0x00417828, 0x001f5f28)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                            c.g[RA][0] = sx(0x001f5f34);
                            c.g[A1][0] = c.g[S3][0];
                            call_ptr(c, 0x001552b0, 0x001f5f34)?;
                            c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            if t {
                                break 'b_1f5f64;
                            } else {
                                c.cycles += 9;
                                c.g[RA][0] = sx(0x001f5f48);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0039e948, 0x001f5f48)?;
                                c.g[V0][0] = 0x1;
                                c.g[A1][0] = c.g[S1][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x164), c.g[V0][0]);
                                c.g[RA][0] = sx(0x001f5f5c);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0039e8b8, 0x001f5f5c)?;
                                let t = true;
                                c.g[V1][0] = 0x4a0000;
                                if t {
                                    break 'b_1f5f7c;
                                } else {
                                    break 'b_1f5f64;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001f5f6c);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039e8b8, 0x001f5f6c)?;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001f5f78);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039e948, 0x001f5f78)?;
                        break 'b_1f5f78;
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = 0x4a0000;
                break 'b_1f5f7c;
            }
            'b_1f6020: {
                c.cycles += 12;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2390));
                c.g[RA][0] = sx(0x001f5f8c);
                c.g[A2][0] = c.g[S5][0];
                call_ptr(c, 0x00417828, 0x001f5f8c)?;
                c.g[RA][0] = sx(0x001f5f94);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001f5f94)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001f5fa0);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001f5fa0)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64;
                c.g[V1][0] = 0x470000;
                if t {
                    break 'b_1f6020;
                } else {
                    c.cycles += 29;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001f5fbc);
                    c.g[A2][0] = 0x8;
                    call_ptr(c, 0x003e6448, 0x001f5fbc)?;
                    c.g[V1][0] = 0x440000;
                    c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1b40));
                    c.g[T0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[A3][0] = c.lwu(c.g[V0][0] as u32);
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[V1][0] = c.g[V1][0] >> 5;
                    c.g[T0][0] = c.g[T0][0] & c.g[V1][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A3][0] = c.g[A3][0] << 59;
                    c.g[A2][0] = 0x64;
                    c.g[T0][0] = c.g[T0][0] | c.g[A3][0];
                    c.g[RA][0] = sx(0x001f5ffc);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T0][0]);
                    call_ptr(c, 0x00116950, 0x001f5ffc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    c.g[V0][0] = 0x470000;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[RA][0] = sx(0x001f6010);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd58));
                    call_ptr(c, 0x00417828, 0x001f6010)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    c.g[RA][0] = sx(0x001f601c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0c50, 0x001f601c)?;
                    c.g[V1][0] = 0x470000;
                    break 'b_1f6020;
                }
            }
            'b_1f60a4: {
                'b_1f60a0: {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xd40));
                    c.g[RA][0] = sx(0x001f6030);
                    c.g[A2][0] = c.g[S5][0];
                    call_ptr(c, 0x00417828, 0x001f6030)?;
                    c.g[RA][0] = sx(0x001f6038);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001f6038)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f6044);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f6044)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x164));
                    if t {
                        break 'b_1f60a0;
                    } else {
                        'b_1f6088: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0x440000;
                            if t {
                                break 'b_1f6088;
                            } else {
                                c.cycles += 12;
                                c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1b40));
                                c.g[A0][0] = c.g[S6][0];
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[A2][0] = 0x64;
                                c.g[RA][0] = sx(0x001f6074);
                                c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                call_ptr(c, 0x00116e08, 0x001f6074)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001f6080);
                                c.g[A1][0] = c.g[S6][0];
                                call_ptr(c, 0x003a0c50, 0x001f6080)?;
                                let t = true;
                                c.g[V1][0] = c.g[S5][0];
                                if t {
                                    break 'b_1f60a4;
                                } else {
                                    break 'b_1f6088;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x001f6094);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd30));
                        call_ptr(c, 0x00317670, 0x001f6094)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001f60a0);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001f60a0)?;
                        break 'b_1f60a0;
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = c.g[S5][0];
                break 'b_1f60a4;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                if event_due(c) {
                    deliver_event_here(c, 0x001f5e68);
                }
                continue 'l_1f5e68;
            } else {
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x170));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x210));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEStateCareerStats_ShowRacePage(self)` (0x1f60e0, `setupRacePlatinumMedalsList`):
/// shows or hides the scroll arrows (`self+0x50`, `+5 < count (+0x58)`), prints the medal
/// count, and fills five rows (peak name, track, time, medal marker from the done mask at
/// +0x60) for the events in range; rows past the end are hidden.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn cfe_state_career_stats_show_race_page(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f6154: {
            'b_1f6144: {
                c.cycles += 20;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[RA][0]);
                c.g[RA][0] = sx(0x001f6114);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x001f6114)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f6120);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x001f6120)?;
                c.g[FP][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_1f6144;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x470000;
                    c.g[RA][0] = sx(0x001f613c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd18));
                    call_ptr(c, 0x0039e948, 0x001f613c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1f6154;
                    } else {
                        break 'b_1f6144;
                    }
                }
            }
            c.cycles += 4;
            c.g[A1][0] = 0x470000;
            c.g[RA][0] = sx(0x001f6150);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd18));
            call_ptr(c, 0x0039e8b8, 0x001f6150)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
            break 'b_1f6154;
        }
        'b_1f618c: {
            'b_1f617c: {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_1f617c;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f6174);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd08));
                    call_ptr(c, 0x0039e948, 0x001f6174)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f618c;
                    } else {
                        break 'b_1f617c;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f6188);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd08));
            call_ptr(c, 0x0039e8b8, 0x001f6188)?;
            c.g[A0][0] = 0x470000;
            break 'b_1f618c;
        }
        c.cycles += 27;
        c.g[S5][0] = 0u64;
        c.g[RA][0] = sx(0x001f6198);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xcf8));
        call_ptr(c, 0x00317670, 0x001f6198)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x001f61a4);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x001f61a4)?;
        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = 0x4a0000;
        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
        c.g[A0][0] = c.g[SP][0];
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001f61c0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2398));
        call_ptr(c, 0x00417828, 0x001f61c0)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001f61cc);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003a0c50, 0x001f61cc)?;
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
        c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), c.g[A0][0]);
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe8), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xec), c.g[A0][0]);
        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
        'l_1f61f8: loop {
            'b_1f6450: {
                'b_1f63d0: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                    c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        break 'b_1f63d0;
                    } else {
                        'b_1f62a4: {
                            c.cycles += 38;
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xce8));
                            c.g[RA][0] = sx(0x001f621c);
                            c.g[A0][0] = c.g[S7][0];
                            call_ptr(c, 0x00417828, 0x001f621c)?;
                            c.g[S0][0] = sx((c.g[S4][0] as u32) << 2);
                            c.g[RA][0] = sx(0x001f6228);
                            c.g[A0][0] = c.g[S7][0];
                            call_ptr(c, 0x00317670, 0x001f6228)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f6234);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f6234)?;
                            c.g[A0][0] = 0x480000;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7f10));
                            c.g[V0][0] = 0x4a0000;
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2138));
                            c.g[A2][0] = c.lw(c.g[V1][0] as u32);
                            c.g[A0][0] = c.g[S6][0];
                            c.g[RA][0] = sx(0x001f625c);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            call_ptr(c, 0x00417828, 0x001f625c)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001f6268);
                            c.g[A1][0] = c.g[S6][0];
                            call_ptr(c, 0x003a0c50, 0x001f6268)?;
                            c.g[A0][0] = 0x470000;
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xcd8));
                            c.g[RA][0] = sx(0x001f627c);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                            call_ptr(c, 0x00417828, 0x001f627c)?;
                            c.g[RA][0] = sx(0x001f6284);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                            call_ptr(c, 0x00317670, 0x001f6284)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f6290);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f6290)?;
                            c.g[A0][0] = 0x3;
                            c.g[S1][0] = c.g[V0][0];
                            c.div(0, 20, 4);
                            if c.g[A0][0] == 0u64 {
                                break 'b_1f62a4;
                            } else {
                                break 'b_1f62a4;
                            }
                        }
                        'b_1f6344: {
                            'b_1f6308: {
                                c.cycles += 3;
                                c.g[V1][0] = c.hi[0];
                                if c.g[V1][0] != 0u64 {
                                    c.g[A0][0] = 0x480000;
                                    break 'b_1f6308;
                                } else {
                                    c.cycles += 22;
                                    c.g[V0][0] = 0x480000;
                                    c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f30));
                                    c.g[A0][0] = c.g[FP][0];
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[RA][0] = sx(0x001f62cc);
                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                    call_ptr(c, 0x00144c60, 0x001f62cc)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.g[V0][0] = 0x4a0000;
                                    c.g[RA][0] = sx(0x001f62e0);
                                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x23a0));
                                    call_ptr(c, 0x00417828, 0x001f62e0)?;
                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[A0][0] = c.g[FP][0];
                                    c.g[RA][0] = sx(0x001f62f0);
                                    c.g[A2][0] = 0x4;
                                    call_ptr(c, 0x001456a0, 0x001f62f0)?;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001f6300);
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x003a0c50, 0x001f6300)?;
                                    let t = true;
                                    c.g[V0][0] = 0x4a0000;
                                    if t {
                                        break 'b_1f6344;
                                    } else {
                                        break 'b_1f6308;
                                    }
                                }
                            }
                            c.cycles += 15;
                            c.g[A2][0] = 0u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7f30));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                            c.g[RA][0] = sx(0x001f6320);
                            c.g[A0][0] = c.g[FP][0];
                            call_ptr(c, 0x001456a0, 0x001f6320)?;
                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A0][0] = c.g[FP][0];
                            c.g[RA][0] = sx(0x001f6330);
                            c.g[S0][0] = c.g[V0][0];
                            call_ptr(c, 0x00144c60, 0x001f6330)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001f633c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0c50, 0x001f633c)?;
                            c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                            c.g[V0][0] = 0x4a0000;
                            break 'b_1f6344;
                        }
                        'b_1f63c0: {
                            c.cycles += 27;
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x23a8));
                            c.g[RA][0] = sx(0x001f6354);
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x00417828, 0x001f6354)?;
                            c.g[RA][0] = sx(0x001f635c);
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x00317670, 0x001f635c)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f6368);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f6368)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001f6374);
                            c.g[S1][0] = c.g[V0][0];
                            call_ptr(c, 0x0020a1a8, 0x001f6374)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f6380);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x003a0c50, 0x001f6380)?;
                            c.g[A0][0] = 0x470000;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xcc8));
                            c.g[A2][0] = c.g[S5][0];
                            c.g[RA][0] = sx(0x001f6398);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00417828, 0x001f6398)?;
                            c.g[V1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S4][0] as u32) & 31));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            if t {
                                break 'b_1f63c0;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001f63b8);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0039e948, 0x001f63b8)?;
                                let t = true;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1f6450;
                                } else {
                                    break 'b_1f63c0;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001f63c8);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039e8b8, 0x001f63c8)?;
                        let t = true;
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_1f6450;
                        } else {
                            break 'b_1f63d0;
                        }
                    }
                }
                c.cycles += 32;
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.g[S7][0];
                c.g[RA][0] = sx(0x001f63e0);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xce8));
                call_ptr(c, 0x00417828, 0x001f63e0)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f63ec);
                c.g[A1][0] = c.g[S7][0];
                call_ptr(c, 0x0039e8b8, 0x001f63ec)?;
                c.g[V0][0] = 0x470000;
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x001f6400);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xcd8));
                call_ptr(c, 0x00417828, 0x001f6400)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f640c);
                c.g[A1][0] = c.g[S6][0];
                call_ptr(c, 0x0039e8b8, 0x001f640c)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                c.g[V0][0] = 0x4a0000;
                c.g[A2][0] = c.g[S5][0];
                c.g[RA][0] = sx(0x001f6420);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x23a8));
                call_ptr(c, 0x00417828, 0x001f6420)?;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                c.g[RA][0] = sx(0x001f642c);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001f642c)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xec));
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xcc8));
                c.g[RA][0] = sx(0x001f6440);
                c.g[A2][0] = c.g[S5][0];
                call_ptr(c, 0x00417828, 0x001f6440)?;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xec));
                c.g[RA][0] = sx(0x001f644c);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001f644c)?;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                break 'b_1f6450;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S5][0] as i64) < 5i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                if event_due(c) {
                    deliver_event_here(c, 0x001f61f8);
                }
                continue 'l_1f61f8;
            } else {
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xf0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEStateCareerStats_ShowFreestylePage(self)` (0x1f6490, `setupFSPlatinumMedalsList`):
/// like the race page for the freestyle events (`self+0x54` scroll, `count` +0x68, medals
/// +0x6c, done mask +0x70).
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn cfe_state_career_stats_show_freestyle_page(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f6504: {
            'b_1f64f4: {
                c.cycles += 20;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[RA][0]);
                c.g[RA][0] = sx(0x001f64c4);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x001f64c4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f64d0);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x001f64d0)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_1f64f4;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x470000;
                    c.g[RA][0] = sx(0x001f64ec);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xcb8));
                    call_ptr(c, 0x0039e948, 0x001f64ec)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    if t {
                        break 'b_1f6504;
                    } else {
                        break 'b_1f64f4;
                    }
                }
            }
            c.cycles += 4;
            c.g[A1][0] = 0x470000;
            c.g[RA][0] = sx(0x001f6500);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xcb8));
            call_ptr(c, 0x0039e8b8, 0x001f6500)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
            break 'b_1f6504;
        }
        'b_1f653c: {
            'b_1f652c: {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_1f652c;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f6524);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xca8));
                    call_ptr(c, 0x0039e948, 0x001f6524)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f653c;
                    } else {
                        break 'b_1f652c;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f6538);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xca8));
            call_ptr(c, 0x0039e8b8, 0x001f6538)?;
            c.g[A0][0] = 0x470000;
            break 'b_1f653c;
        }
        c.cycles += 27;
        c.g[S5][0] = 0u64;
        c.g[RA][0] = sx(0x001f6548);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xc98));
        call_ptr(c, 0x00317670, 0x001f6548)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x001f6554);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x001f6554)?;
        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
        c.g[A1][0] = 0x4a0000;
        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c));
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001f6570);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2398));
        call_ptr(c, 0x00417828, 0x001f6570)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001f657c);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003a0c50, 0x001f657c)?;
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), c.g[V0][0]);
        c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe8), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xec), c.g[V0][0]);
        c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
        'l_1f65a8: loop {
            'b_1f6804: {
                'b_1f6780: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                    c.g[S3][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_1f6780;
                    } else {
                        'b_1f66ec: {
                            'b_1f66a4: {
                                c.cycles += 37;
                                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                                c.g[V1][0] = 0x470000;
                                if c.g[V0][0] != 0 {
                                    c.g[S0][0] = c.g[S3][0];
                                }
                                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xc88));
                                c.g[A2][0] = c.g[S5][0];
                                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 2) as u32);
                                c.g[RA][0] = sx(0x001f65e0);
                                c.g[A0][0] = c.g[FP][0];
                                call_ptr(c, 0x00417828, 0x001f65e0)?;
                                c.g[RA][0] = sx(0x001f65e8);
                                c.g[A0][0] = c.g[FP][0];
                                call_ptr(c, 0x00317670, 0x001f65e8)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001f65f4);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001f65f4)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = 0x4a0000;
                                c.g[A0][0] = c.g[S7][0];
                                c.g[RA][0] = sx(0x001f660c);
                                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2138));
                                call_ptr(c, 0x00417828, 0x001f660c)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001f6618);
                                c.g[A1][0] = c.g[S7][0];
                                call_ptr(c, 0x003a0c50, 0x001f6618)?;
                                c.g[V1][0] = 0x470000;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xc78));
                                c.g[RA][0] = sx(0x001f662c);
                                c.g[A2][0] = c.g[S5][0];
                                call_ptr(c, 0x00417828, 0x001f662c)?;
                                c.g[RA][0] = sx(0x001f6634);
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                call_ptr(c, 0x00317670, 0x001f6634)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001f6640);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001f6640)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[V0][0] = c.g[S3][0] & 0x3;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x480000;
                                if t {
                                    break 'b_1f66a4;
                                } else {
                                    c.cycles += 21;
                                    c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ef0));
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[S4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.g[RA][0] = sx(0x001f666c);
                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                    call_ptr(c, 0x00144c60, 0x001f666c)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.g[V1][0] = 0x4a0000;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001f6680);
                                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x23b0));
                                    call_ptr(c, 0x00417828, 0x001f6680)?;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.g[RA][0] = sx(0x001f668c);
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x003a0c50, 0x001f668c)?;
                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[A2][0] = 0x5;
                                    c.g[RA][0] = sx(0x001f669c);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                                    call_ptr(c, 0x001456a0, 0x001f669c)?;
                                    let t = true;
                                    c.g[V1][0] = 0x4a0000;
                                    if t {
                                        break 'b_1f66ec;
                                    } else {
                                        break 'b_1f66a4;
                                    }
                                }
                            }
                            c.cycles += 18;
                            c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ef0));
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[RA][0] = sx(0x001f66bc);
                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                            call_ptr(c, 0x00144c60, 0x001f66bc)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f66c8);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x003a0c50, 0x001f66c8)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                            c.g[RA][0] = sx(0x001f66d4);
                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                            call_ptr(c, 0x00145870, 0x001f66d4)?;
                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f66e4);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                            call_ptr(c, 0x001456a0, 0x001f66e4)?;
                            c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                            c.g[V1][0] = 0x4a0000;
                            break 'b_1f66ec;
                        }
                        'b_1f6770: {
                            c.cycles += 29;
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2138));
                            c.g[RA][0] = sx(0x001f66fc);
                            c.g[A0][0] = c.g[S4][0];
                            call_ptr(c, 0x00417828, 0x001f66fc)?;
                            c.g[V0][0] = 0x470000;
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc68));
                            c.g[RA][0] = sx(0x001f6710);
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x00417828, 0x001f6710)?;
                            c.g[RA][0] = sx(0x001f6718);
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x00317670, 0x001f6718)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f6724);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f6724)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f6730);
                            c.g[A1][0] = c.g[S4][0];
                            call_ptr(c, 0x003a0c50, 0x001f6730)?;
                            c.g[V1][0] = 0x470000;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xc58));
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001f6748);
                            c.g[A2][0] = c.g[S5][0];
                            call_ptr(c, 0x00417828, 0x001f6748)?;
                            c.g[V1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x70));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S3][0] as u32) & 31));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            if t {
                                break 'b_1f6770;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001f6768);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0039e948, 0x001f6768)?;
                                let t = true;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1f6804;
                                } else {
                                    break 'b_1f6770;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001f6778);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039e8b8, 0x001f6778)?;
                        let t = true;
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_1f6804;
                        } else {
                            break 'b_1f6780;
                        }
                    }
                }
                c.cycles += 33;
                c.g[V0][0] = 0x470000;
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.g[FP][0];
                c.g[RA][0] = sx(0x001f6794);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc88));
                call_ptr(c, 0x00417828, 0x001f6794)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f67a0);
                c.g[A1][0] = c.g[FP][0];
                call_ptr(c, 0x0039e8b8, 0x001f67a0)?;
                c.g[V1][0] = 0x470000;
                c.g[A2][0] = c.g[S5][0];
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xc78));
                c.g[RA][0] = sx(0x001f67b4);
                c.g[A0][0] = c.g[S7][0];
                call_ptr(c, 0x00417828, 0x001f67b4)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f67c0);
                c.g[A1][0] = c.g[S7][0];
                call_ptr(c, 0x0039e8b8, 0x001f67c0)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xec));
                c.g[V0][0] = 0x470000;
                c.g[A2][0] = c.g[S5][0];
                c.g[RA][0] = sx(0x001f67d4);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc68));
                call_ptr(c, 0x00417828, 0x001f67d4)?;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xec));
                c.g[RA][0] = sx(0x001f67e0);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001f67e0)?;
                c.g[V1][0] = 0x470000;
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xc58));
                c.g[RA][0] = sx(0x001f67f4);
                c.g[A2][0] = c.g[S5][0];
                call_ptr(c, 0x00417828, 0x001f67f4)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f6800);
                c.g[A1][0] = c.g[S6][0];
                call_ptr(c, 0x0039e8b8, 0x001f6800)?;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                break 'b_1f6804;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S5][0] as i64) < 5i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                if event_due(c) {
                    deliver_event_here(c, 0x001f65a8);
                }
                continue 'l_1f65a8;
            } else {
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xf0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEStateCareerStats::CountMedals(this)` (0x001f6f30): tallies the medals the
/// current rider still has to earn. For the three reward checks, the 8 track
/// medals (`D_004780D0`, mode 4 on every third track) and the 12 trophy
/// medals (`D_00478110`, mode 5 on every fourth) it asks the reward interface
/// whether the medal is missing and, if so, bumps the counters at
/// `this+0x58/0x5c/0x60/0x68/0x6c/0x70` (and the bit masks).
pub fn count_medals(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f6ff8: {
            c.cycles += 44;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x001f6f64)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xd;
            call_ptr(c, 0x0014dd58, 0x001f6f70)?;
            c.g[S4][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x001f6f78)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x001f6f84)?;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x001f6f8c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x001f6f98)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0u64;
            c.g[FP][0] = c.g[V0][0];
            call_ptr(c, 0x00146e98, 0x001f6fa8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x00147398, 0x001f6fb4)?;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0u64;
            c.g[S5][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            call_ptr(c, 0x00146e98, 0x001f6fc8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] = 0u64;
            call_ptr(c, 0x00157b08, 0x001f6fd8)?;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S1][0];
            if t {
                break 'b_1f6ff8;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[V1][0]);
                break 'b_1f6ff8;
            }
        }
        'b_1f7030: {
            c.cycles += 8;
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00146e98, 0x001f7000)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] = 0x1;
            call_ptr(c, 0x00157b08, 0x001f7010)?;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S1][0];
            if t {
                break 'b_1f7030;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[V1][0]);
                break 'b_1f7030;
            }
        }
        'b_1f706c: {
            c.cycles += 8;
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00146e98, 0x001f7038)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] = 0x2;
            call_ptr(c, 0x00157b08, 0x001f7048)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x480000;
            if t {
                break 'b_1f706c;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[V1][0]);
                c.g[V0][0] = 0x480000;
                break 'b_1f706c;
            }
        }
        c.cycles += 5;
        c.g[S1][0] = 0u64;
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f30));
        c.g[S2][0] = 0x3;
        c.g[S7][0] = 0x4;
        c.g[S6][0] = 0x1;
        'l_1f7080: loop {
            'b_1f708c: {
                c.cycles += 3;
                c.div(0, 17, 18);
                if c.g[S2][0] == 0u64 {
                    break 'b_1f708c;
                } else {
                    break 'b_1f708c;
                }
            }
            'b_1f70d8: {
                c.cycles += 11;
                c.g[T0][0] = c.lw(c.g[S3][0] as u32);
                c.g[A3][0] = 0u64;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S5][0];
                c.g[V0][0] = c.hi[0];
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[S7][0];
                }
                call_ptr(c, 0x00158910, 0x001f70ac)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                if c.g[V0][0] != 0u64 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    break 'b_1f70d8;
                } else {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                    c.g[A0][0] = sx((c.g[S6][0] as u32) << ((c.g[S1][0] as u32) & 31));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    break 'b_1f70d8;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001f7080);
                }
                continue 'l_1f7080;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x480000;
                c.g[S1][0] = 0u64;
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ef0));
                c.g[S3][0] = 0x1;
                'l_1f70f8: loop {
                    'b_1f7114: {
                        'b_1f710c: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[S1][0] & 0x3;
                            if c.g[V0][0] != 0u64 {
                                c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                break 'b_1f710c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0x5;
                                if t {
                                    break 'b_1f7114;
                                } else {
                                    break 'b_1f710c;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[FP][0];
                        call_ptr(c, 0x00145870, 0x001f7114)?;
                        break 'b_1f7114;
                    }
                    'b_1f7158: {
                        c.cycles += 9;
                        c.g[T0][0] = c.lw(c.g[S2][0] as u32);
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = c.g[S5][0];
                        call_ptr(c, 0x00158910, 0x001f712c)?;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        if c.g[V0][0] != 0u64 {
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            break 'b_1f7158;
                        } else {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                            c.g[A0][0] = sx((c.g[S3][0] as u32) << ((c.g[S1][0] as u32) & 31));
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                            c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            break 'b_1f7158;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 12i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001f70f8);
                        }
                        continue 'l_1f70f8;
                    } else {
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
                }
            }
        }
    });
}

/// One visibility update in the show-page tail: the object named by `name`
/// (hashed) under the screen (`self+0x40`) gets a vtable call with the show
/// flag (`S4`). The caller has set `A0` to the generated base (0x470000),
/// which the name's address replaces, as the generated `sx(A0 - off)` does.
pub(crate) fn oc989_show_page_object(
    c: &mut Cpu,
    name: u32,
    hash_ret: u32,
    finder_ret: u32,
    vt_ret: u32,
) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, 0x0031_7670, hash_ret)?;
    c.g[A0][0] = Addr(oc989_at(c, S0, 0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, finder_ret)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S4][0];
    if c.g[A2][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = Addr(oc989_at(c, A2, 0x8)).lw(c);
        c.g[A0][0] = Addr(oc989_at(c, V0, 0x48)).lh(c);
        c.g[V1][0] = Addr(oc989_at(c, V0, 0x4c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, vt_ret)?;
    }
    Ok(())
}

/// The shared tail of the show-page branches that hide or show one object:
/// `cUIState_showObjSafe(self, A1)` (0x39e948), then `A0 = 0x470000`.
pub(crate) fn oc989_show_page_tail(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_e948, 0x001f_5afc)?;
    c.g[A0][0] = 0x0047_0000;
    Ok(())
}

/// The mode-0 and mode-3 exit: the same object call, then the show flag is
/// cleared (`S4 = 0`).
pub(crate) fn oc989_show_page_l0_tail(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_e948, 0x001f_5b0c)?;
    c.g[S4][0] = 0;
    c.g[A0][0] = 0x0047_0000;
    Ok(())
}

/// `void cFEStateCareerStats_ShowPage(cFEStateCareerStats* self)` (0x1f5a38):
/// hides the four page panels, shows the one for the page (`self+0x48`: 0
/// shows panel 0 and clears the show flag, 1 and 2 show panels 1 and 2, 3
/// shows panel 3 and clears the flag), then shows or hides the two page
/// labels (hash of 0x46f1f0 and 0x46f200) with the vtable's show method.
pub fn oc989_career_stats_show_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x60);
        frame.save(c, S5, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S5][0] = sx(0x0046_f1a8);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[A1][0] = c.g[S5][0];
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0);
        c.g[S4][0] = 1;
        call_ptr(c, 0x0039_e8b8, 0x001f_5a70)?;
        c.g[V0][0] = 0x0047_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[S3][0] = sx(0x0046_f1b8);
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0039_e8b8, 0x001f_5a84)?;
        c.g[V0][0] = 0x0047_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = sx(0x0046_f1c8);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0039_e8b8, 0x001f_5a98)?;
        c.g[V0][0] = 0x0047_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = sx(0x0046_f1e0);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e8b8, 0x001f_5aac)?;
        c.g[V1][0] = Addr(oc989_at(c, S0, 0x48)).lw(c);
        let mode_is_one = c.g[V1][0] == c.g[S4][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        c.cycles += 2;
        if mode_is_one {
            c.g[A1][0] = c.g[S3][0];
            oc989_show_page_tail(c)?;
        } else if c.g[V0][0] != 0 {
            // Mode 0 or below (mode 1 was handled above).
            c.g[V0][0] = 2;
            c.cycles += 2;
            c.g[A1][0] = c.g[S5][0];
            if c.g[V1][0] == 0 {
                oc989_show_page_l0_tail(c)?;
            } else {
                c.cycles += 2;
                c.g[A0][0] = 0x0047_0000;
            }
        } else {
            // Mode 2 or above.
            c.g[V0][0] = 2;
            c.cycles += 2;
            let is_two = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if is_two {
                c.cycles += 1;
                c.g[A1][0] = c.g[S2][0];
                oc989_show_page_tail(c)?;
            } else {
                c.cycles += 2;
                let is_three = c.g[V1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                if is_three {
                    oc989_show_page_l0_tail(c)?;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = 0x0047_0000;
                }
            }
        }
        c.cycles += 8;
        oc989_show_page_object(c, 0x0046_f1f0, 0x001f_5b1c, 0x001f_5b28, 0x001f_5b48)?;
        c.cycles += 9;
        c.g[A0][0] = 0x0047_0000;
        oc989_show_page_object(c, 0x0046_f200, 0x001f_5b54, 0x001f_5b60, 0x001f_5b80)?;
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateBuyAttrib::~cFEStateBuyAttrib(this)` (0x220e30): the same shape as
/// [`fe_state_lodge_dtor`] (vtable 0x46d0d0, return 0x220e4c).
pub fn fe_state_buy_attrib_dtor(c: &mut Cpu) {
    guest(c, |c| oc1002_vtable_dtor(c, 0x0046_d0d0, 0x0022_0e4c))
}

/// `cFEStateCareerStats::~cFEStateCareerStats` (0x220e90): stores the career
/// stats vtable (the `0x470000` page less `0x2f30`) at `this+8`, then calls the
/// base destructor `UIState_dtor` (0x39e390) on `this`. Returns to the caller.
pub fn cfe_state_career_stats_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x2f30));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_0eac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod cfe_state_career_stats_field {
    pub const ENGINE: u32 = 0x10;
    pub const SCREEN: u32 = 0x40;
    pub const STATS_STATE: u32 = 0x48;
    pub const TOP_HIGHLIGHT: u32 = 0x4c;
    pub const TOP_RACE: u32 = 0x50;
    pub const TOP_FREESTYLE: u32 = 0x54;
    pub const TOTAL_RACE: u32 = 0x58;
    pub const RACE_MEDALS: u32 = 0x5c;
    pub const RACE_DONE: u32 = 0x60;
    pub const TOTAL_FREESTYLE: u32 = 0x68;
    pub const FREESTYLE_MEDALS: u32 = 0x6c;
    pub const FREESTYLE_DONE: u32 = 0x70;
}

/// `void cFEStateBuyAttrib_UpdateMoney(cFEStateBuyAttrib* self)` (0x1f52a0):
/// looks the money text object (hash of the string at 0x46f0a0) up under the
/// screen at `self+0x48`; if it exists, sets its text to the bank value
/// `self+0xa4` formatted as money.
pub fn oc1023_buy_attrib_update_money(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_f0a0);
        call_ptr(c, 0x0031_7670, 0x001f_52c0)?;
        c.g[A0][0] = Addr(oc1023_at(c, S1, 0x48)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_52cc)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(oc1023_at(c, S1, 0xa4)).lw(c);
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

/// `void cFEStateBuyAttrib_CommitPurchase(cFEStateBuyAttrib* self)`
/// (0x1f5300): spend the bank on the pending purchase. Charges the cost
/// (`self+0xa0`) to the bank (`self+0xa4`), clears the cost, refreshes the
/// total, plays a message on the screen object (`self+0xac`), then for each
/// of the seven attributes bought (`self+0x84+4i` > 0) calls the BE
/// interface once per level; finally adds the bought levels into the owned
/// levels (`self+0x4c`, 7 words) while clearing them, and refreshes the
/// level, cost, bar and money texts.
pub fn oc1023_buy_attrib_commit_purchase(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S3, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x50);
        c.g[V0][0] = sx(oc1023_at(c, S3, 0x4c));
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        c.g[S6][0] = sx(oc1023_at(c, S3, 0x84));
        frame.save(c, S0, 0xa0);
        c.g[FP][0] = c.g[S6][0];
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        frame.save_ra(c, 0x10);
        let owned = c.g[V0][0];
        let slot = frame.at(c, 0);
        c.sw(slot, owned);
        c.g[V0][0] = Addr(oc1023_at(c, S3, 0xa4)).lw(c);
        c.g[V1][0] = Addr(oc1023_at(c, S3, 0xa0)).lw(c);
        Addr(oc1023_at(c, S3, 0xa0)).sw(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        let bank = c.g[V0][0];
        Addr(oc1023_at(c, S3, 0xa4)).sw(c, bank);
        call_ptr(c, 0x001f_5240, 0x001f_5358)?;
        Addr(oc1023_at(c, S3, 0xa8)).sw(c, 0);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx(0x0046_f060);
        call_ptr(c, 0x0039_e8b8, 0x001f_536c)?;
        c.g[A2][0] = Addr(oc1023_at(c, S3, 0xac)).lw(c);
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(oc1023_at(c, A2, 0x8)).lw(c);
        c.g[A0][0] = Addr(oc1023_at(c, V0, 0x38)).lh(c);
        c.g[V1][0] = Addr(oc1023_at(c, V0, 0x3c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001f_5388)?;
        call_ptr(c, 0x0014_dc80, 0x001f_5390)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x001f_539c)?;
        c.g[S5][0] = c.g[V0][0];
        c.g[V1][0] = 0;
        c.g[V0][0] = 0x480000;
        c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f50));
        c.g[A3][0] = sx((c.g[V1][0] as u32) << 2);
        'l_1f53b0: loop {
            'b_1f53fc: {
                c.cycles += 5;
                c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S0][0] = 0;
                if none {
                    break 'b_1f53fc;
                }
                c.cycles += 3;
                c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[S1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.g[A0][0] = 0;
                'l_1f53d0: loop {
                    c.cycles += 11;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    call_ptr(c, 0x0014_a080, 0x001f_53d8)?;
                    c.g[A3][0] = Addr(oc1023_at(c, S1, 0)).lw(c);
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0015_0c20, 0x001f_53ec)?;
                    c.g[V1][0] = Addr(oc1023_at(c, S2, 0)).lw(c);
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    c.g[A0][0] = 0;
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x001f_53d0);
                        }
                        continue 'l_1f53d0;
                    }
                    break 'b_1f53fc;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[S4][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 7) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[V1][0] as u32) << 2);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001f_53b0);
                }
                continue 'l_1f53b0;
            }
            c.cycles += 9;
            c.g[V0][0] = Addr(oc1023_at(c, S5, 0xc)).lw(c);
            c.g[A0][0] = Addr(oc1023_at(c, V0, 0x8)).lh(c);
            c.g[V1][0] = Addr(oc1023_at(c, V0, 0xc)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001f_5420)?;
            c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
            c.g[A1][0] = c.g[S6][0];
            c.g[A2][0] = 6;
            'l_1f5430: loop {
                c.cycles += 9;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let sum = c.g[V1][0];
                Addr(c.g[A0][0] as u32).sw(c, sum);
                Addr(c.g[A1][0] as u32).sw(c, 0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                let more = (c.g[A2][0] as i64) >= 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001f_5430);
                    }
                    continue 'l_1f5430;
                }
                c.cycles += 20;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_4ea8, 0x001f_545c)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_4fc0, 0x001f_5464)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_50b0, 0x001f_546c)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_52a0, 0x001f_5474)?;
                frame.restore(c, S0, 0xa0);
                frame.restore(c, S1, 0x90);
                frame.restore(c, S2, 0x80);
                frame.restore(c, S3, 0x70);
                frame.restore(c, S4, 0x60);
                frame.restore(c, S5, 0x50);
                frame.restore(c, S6, 0x40);
                frame.restore(c, S7, 0x30);
                frame.restore(c, FP, 0x20);
                frame.restore_ra(c, 0x10);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void cFEStateCareerStats_Init(cFEStateCareerStats* self)` (0x1f5508):
/// creates the career-stats screen under the engine (`self+0x10`) and plays
/// its first frame when it exists (`self+0x40`), clears the medal and page
/// counters (`self+0x48..0x74`), counts the medals, shows the first page
/// and finishes the setup with two engine calls.
pub fn oc1023_career_stats_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_f168);
        c.g[S0][0] = Addr(oc1023_at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_552c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_5540)?;
        let screen = c.g[V0][0];
        Addr(oc1023_at(c, S1, 0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_5558)?;
        }
        c.cycles += 26;
        Addr(oc1023_at(c, S1, 0x4c)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        for off in [0x50, 0x54, 0x58, 0x5c, 0x64, 0x60, 0x68, 0x70, 0x6c, 0x74] {
            Addr(oc1023_at(c, S1, off)).sw(c, 0);
        }
        Addr(oc1023_at(c, S1, 0x48)).sw(c, 0);
        call_ptr(c, 0x001f_6f30, 0x001f_5590)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_5a38, 0x001f_5598)?;
        call_ptr(c, 0x0028_b180, 0x001f_55a0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 8;
        call_ptr(c, 0x0028_f140, 0x001f_55ac)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateBuyAttrib_ctor_2(self)` (0x220df8): `UIState_ctor(self, 0)`, then
/// the vtable `0x473908` at `+8`.
pub fn cfe_state_buy_attrib_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_state_ctor_2_body(c, 0x0022_0e10, 0x0047_3908));
}

/// `cFEStateCareerStats_ctor_2(self)` (0x220e58): `UIState_ctor(self, 0)`,
/// then the vtable `0x473838` at `+8`.
pub fn cfe_state_career_stats_ctor_2(c: &mut Cpu) {
    guest(c, |c| fe_state_ctor_2_body(c, 0x0022_0e70, 0x0047_3838));
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
    pub(crate) use crate::game::fe::ovstatepause::tests::oc986_install;
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
    use super::cfe_state_career_stats_field as field;

    use crate::difftest::check_lohi;
    use crate::difftest::fakes::check_reaching;
    use crate::difftest::fakes::machine_rt as machine_m2;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::RA as DT_RA;
    pub(crate) use crate::game::fe::ovstateenterlodge::tests::object_fixture;
    pub(crate) use crate::game::fe::ovstatehud::tests::oc1002_arm;

    pub(crate) use crate::game::fe::ovstatehud::tests::oc1002_install_fakes;

    pub(crate) use crate::game::fe::ovstatehud::tests::OC1002_OBJ;

    pub(crate) use crate::game::fe::ovstatepause::tests::oc986_machine;
    pub(crate) use crate::game::fe::ovstatepause::tests::oc986_results;

    pub(crate) use crate::game::fe::ovstatepause::tests::OC986_T_145870;
    pub(crate) use crate::game::fe::ovstatepause::tests::OC986_T_146E98;
    pub(crate) use crate::game::fe::ovstatepause::tests::OC986_T_147398;
    pub(crate) use crate::game::fe::ovstatepause::tests::OC986_T_14DC80;
    pub(crate) use crate::game::fe::ovstatepause::tests::OC986_T_14DD58;
    pub(crate) use crate::game::fe::ovstatepause::tests::OC986_T_157B08;
    pub(crate) use crate::game::fe::ovstatepause::tests::OC986_T_158910;

    /// `ShowRecordsPage` against the generated body: aligned and unaligned
    /// record copies, every row with n > 0, n == 0 and n < 0, and call
    /// unwinds (all callees are faked; the SetText and sprintf calls are
    /// compared through the call log).
    #[test]
    fn c_fe_state_career_stats_show_records_page_matches() {
        check(
            ee::s04::f_001f6890__generated,
            c_fe_state_career_stats_show_records_page,
            400,
            &|c, seed| {
                let mut r: Rng = machine_m2(c, seed, 0x1f6890, 40);
                let this = DATA + 0x100;
                let screen = DATA + 0x400;
                // The record the 6th call (0x155288) returns, 8-aligned or not.
                let rec = DATA + 0x2000 + if seed % 2 == 0 { 0 } else { 1 + (seed / 2) % 7 };
                c.bus.write32(LOG_RESULTS + 4 * 5, rec);
                c.bus.write32(this + 0x40, screen);
                c.g[A0][0] = sx(this);
                // Counts at the row offsets: > 0, 0 or < 0 by seed bits.
                for (k, off) in [0u32, 0x28, 8, 0x10, 0x18, 0x30, 0x38, 0x20]
                    .into_iter()
                    .enumerate()
                {
                    let kind = (seed >> k) as u64 ^ r.next_u32() as u64;
                    let n: u32 = match kind % 3 {
                        0 => 0,
                        1 => 1 + r.next_u32() % 5000,
                        _ => (seed >> 3) | 0x8000_0000,
                    };
                    let n = if seed % 5 == 0 && k == 0 { 0 } else { n };
                    c.bus.write32(rec + off, n);
                    c.bus.write32(rec + off + 4, r.next_u32() % 40);
                }
                // 0.01f for the percentage row.
                c.bus
                    .write32(crate::difftest::GAME_GP.wrapping_sub(0x5548), 0x3c23_d70a);
            },
        );
    }

    /// Gate word of this module's fakes in the machine's RAM (both sides of a
    /// diff get their own machine); the fakes fall back when it is not set.
    pub(crate) const OC985_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC985_MAGIC: u32 = 0x0985_f56c;

    /// Bit n set: the n-th `UIScreen_FindObject` call finds a widget.
    pub(crate) const OC985_FIND_MASK: u32 = OC985_CTRL + 4;

    pub(crate) const OC985_FIND_N: u32 = OC985_CTRL + 8;

    /// Forced results (result + 1; 0 = the fake's default) per mailbox.
    pub(crate) const OC985_MAIL: u32 = OC985_CTRL + 0x10;

    pub(crate) const OC985_MAIL_LOOKUP: u32 = 0;

    pub(crate) const OC985_MAIL_LEVEL: u32 = 1;

    pub(crate) const OC985_NO_MAIL: u32 = u32::MAX;

    /// Eight widgets (0x100 apart) sharing one vtable whose `+0x38/+0x3c`
    /// slot (`setFocus`) is a fake call.
    pub(crate) const OC985_OBJ: u32 = DATA + 0x20_1000;

    pub(crate) const OC985_OBJ_VT: u32 = DATA + 0x20_2000;

    pub(crate) const OC985_THIS: u32 = DATA + 0x1000;

    pub(crate) const OC985_SCREEN: u32 = DATA + 0x2000;

    pub(crate) const OC985_ENG: u32 = DATA + 0x3000;

    pub(crate) const OC985_SOUND: u32 = DATA + 0x3100;

    pub(crate) const OC985_SOUND_VT: u32 = DATA + 0x3200;

    pub(crate) const OC985_W: u32 = DATA + 0x3300;

    pub(crate) const OC985_WVT: u32 = DATA + 0x3400;

    /// A real stub that returns 0 (a vtable slot answering false).
    pub(crate) const OC985_RET_ZERO: u32 = 0x0014_06c0;

    /// The fake callee: logs the call like `fakes::fake_call`, then applies the
    /// forced result of its mailbox, or (for `UIScreen_FindObject`) finds a
    /// widget or nothing according to the mask.
    pub(crate) fn oc985_fake_call(c: &mut Cpu, addr: u32, mail: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if mail != OC985_NO_MAIL {
            let m = c.bus.read32(OC985_MAIL + 4 * mail);
            if m != 0 {
                c.g[V0][0] = (m - 1) as u64;
            }
        }
        if addr == 0x0039_d860 {
            let n = c.bus.read32(OC985_FIND_N);
            c.bus.write32(OC985_FIND_N, n.wrapping_add(1));
            let mask = c.bus.read32(OC985_FIND_MASK);
            c.g[V0][0] = if (mask >> (n % 32)) & 1 != 0 {
                sx(OC985_OBJ + 0x100 * (n % 8))
            } else {
                0
            };
        }
    }

    macro_rules! oc985_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $mail:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC985_CTRL) != OC985_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc985_fake_call(c, $addr, $mail);
                }
            )*
            fn oc985_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc985_fakes!(
        (
            oc985_fake_317670,
            OC985_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_39d860,
            OC985_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_39e8b8,
            OC985_PREV_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_39e948,
            OC985_PREV_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_3a0c50,
            OC985_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_3a1000,
            OC985_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_39c778,
            OC985_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778,
            OC985_MAIL_LOOKUP
        ),
        (
            oc985_fake_39c870,
            OC985_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_417828,
            OC985_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_3e6448,
            OC985_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_14dc80,
            OC985_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_14dd58,
            OC985_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1552b0,
            OC985_PREV_1552B0,
            ee::s01::OVERRIDE_001552B0,
            ee::s01::f_001552b0__generated,
            0x0015_52b0,
            OC985_MAIL_LEVEL
        ),
        (
            oc985_fake_144c60,
            OC985_PREV_144C60,
            ee::s01::OVERRIDE_00144C60,
            ee::s01::f_00144c60__generated,
            0x0014_4c60,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1456a0,
            OC985_PREV_1456A0,
            ee::s01::OVERRIDE_001456A0,
            ee::s01::f_001456a0__generated,
            0x0014_56a0,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_145870,
            OC985_PREV_145870,
            ee::s01::OVERRIDE_00145870,
            ee::s01::f_00145870__generated,
            0x0014_5870,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_116950,
            OC985_PREV_116950,
            ee::s00::OVERRIDE_00116950,
            ee::s00::f_00116950__generated,
            0x0011_6950,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_116e08,
            OC985_PREV_116E08,
            ee::s00::OVERRIDE_00116E08,
            ee::s00::f_00116e08__generated,
            0x0011_6e08,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_20a1a8,
            OC985_PREV_20A1A8,
            ee::s05::OVERRIDE_0020A1A8,
            ee::s05::f_0020a1a8__generated,
            0x0020_a1a8,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f5a38,
            OC985_PREV_1F5A38,
            ee::s04::OVERRIDE_001F5A38,
            ee::s04::f_001f5a38__generated,
            0x001f_5a38,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f5ba8,
            OC985_PREV_1F5BA8,
            ee::s04::OVERRIDE_001F5BA8,
            ee::s04::f_001f5ba8__generated,
            0x001f_5ba8,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f5da0,
            OC985_PREV_1F5DA0,
            ee::s04::OVERRIDE_001F5DA0,
            ee::s04::f_001f5da0__generated,
            0x001f_5da0,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f60e0,
            OC985_PREV_1F60E0,
            ee::s04::OVERRIDE_001F60E0,
            ee::s04::f_001f60e0__generated,
            0x001f_60e0,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f6490,
            OC985_PREV_1F6490,
            ee::s04::OVERRIDE_001F6490,
            ee::s04::f_001f6490__generated,
            0x001f_6490,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f6890,
            OC985_PREV_1F6890,
            ee::s04::OVERRIDE_001F6890,
            ee::s04::f_001f6890__generated,
            0x001f_6890,
            OC985_NO_MAIL
        ),
    );

    /// A scrambled machine with the fakes' gate open, widgets for the finder
    /// (found per the mask), and a career stats object with random list
    /// positions at `OC985_THIS`: the highlights scroll (0..23), the race and
    /// freestyle scrolls with a total around `scroll + 5`, medal counts and
    /// done masks.
    pub(crate) fn oc985_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc985_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC985_CTRL, OC985_MAGIC);
        c.bus.write32(OC985_FIND_N, 0);
        let mask = r.next_u32() | r.next_u32();
        c.bus.write32(OC985_FIND_MASK, mask);
        for k in 0..8 {
            c.bus.write32(OC985_OBJ + 0x100 * k + 8, OC985_OBJ_VT);
        }
        c.bus
            .write16(OC985_OBJ_VT + 0x38, (r.next_u32() & 0x30) as u16);
        c.bus.write32(OC985_OBJ_VT + 0x3c, FAKE);
        // 0x39c778 finds the frame (5) or not (0xffff); the highlight level 0..5.
        let lookup = if r.next_u32() % 2 == 0 { 0x1_0000 } else { 6 };
        c.bus.write32(OC985_MAIL + 4 * OC985_MAIL_LOOKUP, lookup);
        c.bus
            .write32(OC985_MAIL + 4 * OC985_MAIL_LEVEL, 1 + r.next_u32() % 6);

        let this = OC985_THIS;
        c.bus.write32(this + field::ENGINE, OC985_ENG);
        c.bus.write32(this + field::SCREEN, OC985_SCREEN);
        c.bus.write32(this + field::STATS_STATE, r.next_u32() % 4);
        const HIGHLIGHT_SCROLLS: [u32; 7] = [0, 0, 1, 5, 20, 21, 23];
        let hl = HIGHLIGHT_SCROLLS[(r.next_u32() % 7) as usize];
        c.bus.write32(this + field::TOP_HIGHLIGHT, hl);
        for (top, total, medals, done) in [
            (
                field::TOP_RACE,
                field::TOTAL_RACE,
                field::RACE_MEDALS,
                field::RACE_DONE,
            ),
            (
                field::TOP_FREESTYLE,
                field::TOTAL_FREESTYLE,
                field::FREESTYLE_MEDALS,
                field::FREESTYLE_DONE,
            ),
        ] {
            let scroll = if r.next_u32() % 4 == 0 {
                0
            } else {
                r.next_u32() % 12
            };
            let count = if r.next_u32() % 6 == 0 {
                r.next_u32() % 4
            } else {
                scroll + 3 + r.next_u32() % 5
            };
            c.bus.write32(this + top, scroll);
            c.bus.write32(this + total, count);
            c.bus.write32(this + medals, r.next_u32() % 20);
            c.bus.write32(this + done, r.next_u32());
        }
        r
    }

    /// `HandleInput`: left/right (neither, one) x up/down (none, up, down) in
    /// each of the four pages; the fake vtable slots answer true (a fake
    /// pointer) or false (a real leaf returning 0); the UI sound object is
    /// there or the null sound is used; fakes unwind at random calls.
    #[test]
    fn cfe_state_career_stats_handle_input_matches() {
        oc985_install_fakes();
        check_reaching(
            ee::s04::f_001f56c8__generated,
            cfe_state_career_stats_handle_input,
            320,
            &[
                0x001f_57e0, // left/right: SelectPage
                0x001f_582c, // highlights up
                0x001f_5874, // highlights down
                0x001f_58c0, // race up
                0x001f_590c, // race down
                0x001f_5958, // freestyle up
                0x001f_59a4, // freestyle down
            ],
            &|c, seed| {
                let mut r = oc985_fixture(c, seed, 0x001f_56c8, 14);
                let this = OC985_THIS;
                c.bus.write32(this + field::STATS_STATE, (seed / 20) % 4);
                c.bus.write32(OC985_W + 8, OC985_WVT);
                let lr = seed % 5; // 0: left, 1: right, else neither
                let ud = (seed / 5) % 4; // bit 0: up, bit 1: down
                let want = [
                    (0x98u32, lr == 0),
                    (0xa0, lr == 1),
                    (0x88, ud & 1 != 0),
                    (0x90, ud & 2 != 0),
                ];
                for (off, yes) in want {
                    c.bus.write16(OC985_WVT + off, 0);
                    c.bus
                        .write32(OC985_WVT + off + 4, if yes { FAKE } else { OC985_RET_ZERO });
                }
                // The UI sound object, or none (the null sound at gp+0x2968).
                if r.next_u32() % 4 != 0 {
                    c.bus.write32(OC985_ENG + 0x14, OC985_SOUND);
                    c.bus.write32(OC985_SOUND, OC985_SOUND_VT);
                    c.bus.write16(OC985_SOUND_VT + 0x10, 0);
                    c.bus.write32(OC985_SOUND_VT + 0x14, FAKE);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(OC985_W);
            },
        );
    }

    /// `SelectPage`: the page (0..5: four pages and out of range) is stored,
    /// the four tab widgets (found or not) lose the focus, the selected one
    /// gains it and its page is built.
    #[test]
    fn cfe_state_career_stats_select_page_matches() {
        oc985_install_fakes();
        check_reaching(
            ee::s04::f_001f5ba8__generated,
            cfe_state_career_stats_select_page,
            200,
            &[0x001f_5d0c, 0x001f_5d34, 0x001f_5d5c, 0x001f_5d80],
            &|c, seed| {
                oc985_fixture(c, seed, 0x001f_5ba8, 12);
                c.g[A0][0] = sx(OC985_THIS);
                c.g[A1][0] = (seed % 6) as u64;
            },
        );
    }

    /// `ShowHighlightsPage`: scroll arrows, three rows of step name, on/off
    /// marker (level 0..5 against the step), trick name and required-trick or
    /// locked text, with widgets found or missing.
    #[test]
    fn cfe_state_career_stats_show_highlights_page_matches() {
        oc985_install_fakes();
        check_reaching(
            ee::s04::f_001f5da0__generated,
            cfe_state_career_stats_show_highlights_page,
            300,
            &[
                0x001f_5f5c,
                0x001f_5f6c,
                0x001f_601c,
                0x001f_6074,
                0x001f_60a0,
            ],
            &|c, seed| {
                oc985_fixture(c, seed, 0x001f_5da0, 40);
                c.g[A0][0] = sx(OC985_THIS);
            },
        );
    }

    /// `ShowRacePage`: scroll arrows, medal count and five rows (events in
    /// range with the two track-name variants and the done marker, rows past
    /// the end hidden).
    #[test]
    fn cfe_state_career_stats_show_race_page_matches() {
        oc985_install_fakes();
        check_reaching(
            ee::s04::f_001f60e0__generated,
            cfe_state_career_stats_show_race_page,
            300,
            &[
                0x001f_62cc,
                0x001f_6320,
                0x001f_63b8,
                0x001f_63c8,
                0x001f_63e0,
            ],
            &|c, seed| {
                oc985_fixture(c, seed, 0x001f_60e0, 40);
                c.g[A0][0] = sx(OC985_THIS);
            },
        );
    }

    /// `ShowFreestylePage`: the same for the freestyle events.
    #[test]
    fn cfe_state_career_stats_show_freestyle_page_matches() {
        oc985_install_fakes();
        check_reaching(
            ee::s04::f_001f6490__generated,
            cfe_state_career_stats_show_freestyle_page,
            300,
            &[
                0x001f_666c,
                0x001f_66bc,
                0x001f_6768,
                0x001f_6778,
                0x001f_67a0,
            ],
            &|c, seed| {
                oc985_fixture(c, seed, 0x001f_6490, 40);
                c.g[A0][0] = sx(OC985_THIS);
            },
        );
    }

    /// `CountMedals`: reward medals missing for the current rider.
    #[test]
    fn oc986_count_medals_matches() {
        oc986_install();
        check_lohi(
            ee::s04::f_001f6f30__generated,
            count_medals,
            160,
            &|c, seed| {
                let mut r = oc986_machine(c, seed, 0x1f6f30, 0, 70);
                let this = DATA + 0x1000;
                for off in [0x58u32, 0x5c, 0x60, 0x68, 0x6c, 0x70] {
                    c.bus.write32(this + off, r.next_u32() % 64);
                }
                oc986_results(c, OC986_T_14DC80, &mut |k| DATA + 0x2000 + 0x10 * k);
                oc986_results(c, OC986_T_14DD58, &mut |k| DATA + 0x2400 + 0x10 * k);
                oc986_results(c, OC986_T_146E98, &mut |_| r.next_u32() % 5);
                oc986_results(c, OC986_T_147398, &mut |_| r.next_u32());
                oc986_results(c, OC986_T_145870, &mut |_| r.next_u32() % 4);
                // The three reward checks (bit k of the seed: medal missing).
                oc986_results(c, OC986_T_157B08, &mut |k| {
                    if (seed >> (k % 3)) & 1 == 0 {
                        0
                    } else {
                        1 + (k % 5)
                    }
                });
                // The medal lookup: 0 (missing), nonzero with the low byte
                // zero (still missing: the result is a char) or a real medal.
                oc986_results(c, OC986_T_158910, &mut |_| match r.next_u32() % 3 {
                    0 => 0,
                    1 => 0x100,
                    _ => 1 + r.next_u32() % 255,
                });
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `GetHashValue32` of the NUL-terminated string at `addr` in the loaded
    /// game image (the setup has to run `elf_image` first).
    pub(crate) fn hash_at(c: &mut Cpu, addr: u32) -> u32 {
        let mut s = Vec::new();
        let mut a = addr;
        loop {
            let b = Addr(a).read_u8(c);
            if b == 0 {
                break;
            }
            s.push(b);
            a += 1;
        }
        crate::game::hashvalue::str_hash(&s)
    }

    #[test]
    fn oc989_career_stats_show_page_matches() {
        check(
            ee::s04::f_001f5a38__generated,
            oc989_career_stats_show_page,
            24,
            &|c, seed| {
                machine_rt(c, seed, 0x001f_5a38, 0);
                elf_image(c);
                let this = DATA + 0x100;
                let mode = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 5,
                    _ => u32::MAX,
                };
                c.bus.write32(this + 0x48, mode);
                let screen = DATA + 0x500;
                c.bus.write32(this + 0x40, screen);
                c.bus.write32(screen + 0x38, DATA + 0x600);
                c.bus.write32(DATA + 0x600, 2);
                c.bus.write32(screen + 0x3c, DATA + 0x610);
                let (n1, n2) = (DATA + 0x620, DATA + 0x640);
                c.bus.write32(DATA + 0x610, n1);
                c.bus.write32(DATA + 0x614, n2);
                let h1 = hash_at(c, 0x0046_f1f0);
                let h2 = hash_at(c, 0x0046_f200);
                c.bus.write32(n1 + 0x38, h1);
                c.bus.write32(n1 + 0x8, DATA + 0x660);
                c.bus.write32(n2 + 0x38, h2);
                c.bus.write32(n2 + 0x8, DATA + 0x660);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `cFEStateBuyAttrib_dtor` (0x220e30): vtable store, `UIState_dtor` call.
    #[test]
    fn fe_state_buy_attrib_dtor_matches() {
        oc1002_install_fakes();
        fakes::covered(
            ee::s05::f_00220e30__generated,
            fe_state_buy_attrib_dtor,
            80,
            &|c, seed| {
                let _ = machine(c, seed, 0x1002_0e30, 1);
                oc1002_arm(c);
                c.g[A0][0] = sx(OC1002_OBJ);
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    #[test]
    fn cfe_state_career_stats_dtor_matches() {
        check(
            ee::s05::f_00220e90__generated,
            cfe_state_career_stats_dtor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_0e90);
            },
        );
    }

    pub(crate) use crate::game::fe::ovstatepause::tests::oc1023_force;

    pub(crate) use crate::game::fe::ovstatepause::tests::oc1023_machine;

    pub(crate) use crate::game::fe::ovstatepause::tests::OC1023_OBJ;

    #[test]
    fn oc1023_buy_attrib_update_money_matches() {
        fakes::covered(
            ee::s04::f_001f52a0__generated,
            oc1023_buy_attrib_update_money,
            40,
            &|c, seed| {
                oc1023_machine(c, seed, 0x001f_52a0, 4);
                let this = DATA + 0x100;
                c.bus.write32(this + 0x48, DATA + 0x1000);
                c.bus.write32(this + 0xa4, seed * 1000 + 5);
                // The money text object: found (seed % 3 != 0) or not.
                oc1023_force(c, 1, if seed % 3 == 0 { 0 } else { OC1023_OBJ });
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc1023_buy_attrib_commit_purchase_matches() {
        fakes::covered(
            ee::s04::f_001f5300__generated,
            oc1023_buy_attrib_commit_purchase,
            60,
            &|c, seed| {
                let mut r = oc1023_machine(c, seed, 0x001f_5300, 14);
                let this = DATA + 0x100;
                c.bus.write32(this + 0xa0, seed % 5 * 3);
                c.bus.write32(this + 0xa4, seed * 7);
                for i in 0..7u32 {
                    c.bus.write32(this + 0x4c + 4 * i, r.next_u32() % 20);
                    // Levels bought: none, one, two, or (rarely) negative.
                    let bought = match r.next_u32() % 6 {
                        0 | 1 => 0,
                        2 | 3 => 1,
                        4 => 2,
                        _ => u32::MAX,
                    };
                    c.bus.write32(this + 0x84 + 4 * i, bought);
                    c.bus.write32(0x0047_80b0 + 4 * i, r.next_u32() % 9);
                }
                // The message screen object (vtable 0x38 / 0x3c).
                let screen = DATA + 0x400;
                c.bus.write32(this + 0xac, screen);
                let vt = DATA + 0x500;
                c.bus.write32(screen + 8, vt);
                c.bus.write16(vt + 0x38, 0x20);
                c.bus.write32(vt + 0x3c, fakes::FAKE + 0x80);
                // The BE interface (call 4): vtable 8 / 0xc at +0xc.
                let iface = DATA + 0x600;
                let iface_vt = DATA + 0x700;
                c.bus.write32(iface + 0xc, iface_vt);
                c.bus.write16(iface_vt + 8, 0x10);
                c.bus.write32(iface_vt + 0xc, fakes::FAKE + 0xa0);
                oc1023_force(c, 4, iface);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc1023_career_stats_init_matches() {
        fakes::covered(
            ee::s04::f_001f5508__generated,
            oc1023_career_stats_init,
            40,
            &|c, seed| {
                oc1023_machine(c, seed, 0x001f_5508, 7);
                let this = DATA + 0x100;
                c.bus.write32(this + 0x10, DATA + 0x300);
                for off in (0x40..0x78).step_by(4) {
                    c.bus.write32(this + off, 0x5555_0000 + off);
                }
                // The new screen: created (non-zero) or not.
                oc1023_force(c, 1, if seed % 3 == 0 { 0 } else { OC1023_OBJ });
                c.g[A0][0] = sx(this);
            },
        );
    }

    pub(crate) use crate::game::fe::festatelodge::tests::ui_state_ctor_2_case;

    #[test]
    fn cfe_state_buy_attrib_ctor_2_matches() {
        ui_state_ctor_2_case(
            ee::s05::f_00220df8__generated,
            cfe_state_buy_attrib_ctor_2,
            0x0022_0df8,
        );
    }

    #[test]
    fn cfe_state_career_stats_ctor_2_matches() {
        ui_state_ctor_2_case(
            ee::s05::f_00220e58__generated,
            cfe_state_career_stats_ctor_2,
            0x0022_0e58,
        );
    }
}
