//! `src/fe/festateruleselect.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::festatebigradio::oc849_at;
pub(crate) use crate::game::fe::festatebigradio::OC849_STR_HASH;
pub(crate) use crate::game::main::gamestate::p_2 as p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_1c68,
        c_fe_state_rule_select_on_widget_action,
        "cFEStateRuleSelect_OnWidgetAction",
    ),
    (
        0x0019_1e08,
        c_fe_state_rule_select_v21,
        "cFEStateRuleSelect_v21",
    ),
    (
        0x0019_26f0,
        c_fe_state_bragging_rights_init_player_rows,
        "cFEStateBraggingRights_InitPlayerRows",
    ),
    (
        0x0019_2740,
        c_fe_state_bragging_rights_set_rows_visible,
        "cFEStateBraggingRights_SetRowsVisible",
    ),
    (
        0x0019_2948,
        cfe_state_bragging_rights_is_row_enabled,
        "cFEStateBraggingRights_IsRowEnabled",
    ),
    (
        0x0019_2d00,
        cfestatebraggingrights_ctor,
        "cFEStateBraggingRights_ctor",
    ),
    (
        0x0019_1938,
        cfe_state_rule_select_init,
        "cFEStateRuleSelect_Init",
    ),
    (
        0x0019_2240,
        cfe_state_rule_select_get_rule_value,
        "cFEStateRuleSelect_GetRuleValue",
    ),
    (
        0x0019_2488,
        cfe_state_rule_select_update_row_visibility,
        "cFEStateRuleSelect_UpdateRowVisibility",
    ),
    (
        0x0019_13b8,
        rule_select_fill_rule_list,
        "cFEStateRuleSelect_FillRuleList",
    ),
    (
        0x0019_1c48,
        fe_state_rule_select_on_activate,
        "cFEStateRuleSelect_OnActivate",
    ),
    (
        0x0019_1e48,
        cfe_state_rule_select_on_widget_notify,
        "cFEStateRuleSelect_OnWidgetNotify",
    ),
    (
        0x0019_2088,
        cfe_state_rule_select_set_rule_value,
        "cFEStateRuleSelect_SetRuleValue",
    ),
    (
        0x0019_2380,
        cfe_state_rule_select_apply_rule_dependencies,
        "cFEStateRuleSelect_ApplyRuleDependencies",
    ),
    (
        0x0019_27b0,
        cfe_state_bragging_rights_update_row_colors,
        "cFEStateBraggingRights_UpdateRowColors",
    ),
    (
        0x0019_2918,
        c_fe_state_bragging_rights_find_row_index,
        "cFEStateBraggingRights_FindRowIndex",
    ),
    (
        0x0019_2968,
        c_fe_state_bragging_rights_set_row_mask,
        "cFEStateBraggingRights_SetRowMask",
    ),
    (
        0x0019_2970,
        c_fe_state_bragging_rights_hide_unused_rows,
        "cFEStateBraggingRights_HideUnusedRows",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;

    /// `cFEStateRuleSelect_SetRuleValue` (0x192088).
    pub(crate) const SET_RULE_VALUE: Callee = Callee {
        addr: 0x0019_2088,
        f: ee::s02::f_00192088,
    };
    /// `cFEStateRuleSelect_ApplyRuleDependencies` (0x192380).
    pub(crate) const APPLY_RULE_DEPENDENCIES: Callee = Callee {
        addr: 0x0019_2380,
        f: ee::s02::f_00192380,
    };
    /// `cFEStateOptions_ctor` (0x1887a0).
    pub(crate) const FE_STATE_OPTIONS_CTOR: Callee = Callee {
        addr: 0x0018_87a0,
        f: ee::s02::f_001887a0,
    };
}

// ---------------------------------------------------------------------------
// Small leaves.

/// `bool cFEStateBraggingRights_IsRowEnabled(this, int idx)` (0x00192948):
/// bit `idx` of the top byte of the word at `+0x2c`.
pub fn cfe_state_bragging_rights_is_row_enabled(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = 0x0100_0000;
    let word = Addr((c.g[A0][0] as u32).wrapping_add(0x2c)).lw(c);
    c.g[V0][0] = word;
    let shift = (c.g[A1][0] as u32) & 31;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << shift);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
    let masked = word & c.g[V1][0];
    c.g[V0][0] = masked;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = (0u64 < masked) as u64;
    c.pc = ra;
}

/// `StrHash` (0x317670).
pub(crate) const STR_HASH: u32 = 0x0031_7670;

/// `int cFEStateRuleSelect_v21(this, msg)` (0x191e08): 0x101 when the message
/// id at `+0x38` is the hash of `D_004A13A0`, 0x100 otherwise.
pub fn c_fe_state_rule_select_v21(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = 0x004a_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x004a_13a0);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lw(c);
        call_ptr(c, STR_HASH, 0x0019_1e24)?;
        c.g[S0][0] = c.g[S0][0] ^ c.g[V0][0];
        c.g[V1][0] = 0x100;
        c.g[V0][0] = 0x101;
        frame.restore_ra(c, 0);
        if c.g[S0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end states.

/// `cFEStateRulesSelect* cFEStateBraggingRights_InitPlayerRows(self)`
/// (0x1926f0): install the vtable, clear the eight player rows (0x8..0x24,
/// descending) and the selection state; return `self`.
pub fn c_fe_state_bragging_rights_init_player_rows(c: &mut Cpu) {
    c.cycles += 6;
    let this = c.g[A0][0] as u32;
    c.g[V0][0] = 0x0047_0000;
    c.g[V1][0] = 0x7;
    c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x4f88)); // &D_0046B078
    Addr(this).off(0x30).sw(c, c.g[V0][0]);
    c.g[V0][0] = sx(this.wrapping_add(0x24));
    loop {
        c.cycles += 7;
        Addr(c.g[V0][0] as u32).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0019_2708);
        }
    }
    c.cycles += 6;
    c.g[V0][0] = c.g[A0][0];
    Addr(this).sw(c, 0);
    Addr(this).off(0x2c).sw(c, 0);
    Addr(this).off(0x28).sw(c, 0);
    Addr(this).off(4).sw(c, 0);
    crate::abi::ret(c);
}

/// `void cFEStateBraggingRights_SetRowsVisible(cFEState* self, int a1)`
/// (0x192740): tell up to eight child rows they are visible or not.
pub fn c_fe_state_bragging_rights_set_rows_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(at(c, A0, 8));
        frame.save_ra(c, 0);
        c.g[S1][0] = 7;
        Addr(at(c, A0, 0x28)).sw(c, c.g[S2][0]);
        c.g[A2][0] = Addr(at(c, S0, 0)).lw(c);
        loop {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[S0][0] = sx(at(c, S0, 4));
            let none = c.g[A2][0] == 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            if !none {
                c.cycles += 5;
                c.g[V0][0] = Addr(at(c, A2, 8)).lw(c);
                c.g[A0][0] = Addr(at(c, V0, 0x40)).lh(c);
                c.g[V1][0] = Addr(at(c, V0, 0x44)).lw(c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0019_278c)?;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                c.cycles += 6;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.g[A2][0] = Addr(at(c, S0, 0)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0019_2768);
            }
        }
    })
}

/// `cFEStateBraggingRights_ctor(self, ...)` (0x192d00): construct the state,
/// set its vtable and clear the two player rows (`0x34` bytes each).
pub fn cfestatebraggingrights_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0;
        frame.save(c, 18, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 19, 0x10);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6d0));
        frame.save_ra(c, 0);
        c.g[S1][0] = 1;
        call_ptr(c, 0x001a_8500, 0x0019_2d2c)?;
        c.g[S3][0] = u64::MAX;
        c.g[V0][0] = 0x470000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5098));
        c.sw(p(r32(c, S2), 8), c.g[V0][0]);
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            call_ptr(
                c,
                callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS.addr,
                0x0019_2d4c,
            )?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_2d40);
            }
        }
        c.cycles += 11;
        c.g[V1][0] = 0x14;
        c.g[V0][0] = c.g[S2][0];
        c.sw(p(r32(c, S2), 0xc), c.g[V1][0]);
        c.sw(p(r32(c, S2), 0x738), 0);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Widget actions.

/// `UIStateMgr_SwitchState` (0x39f400).
const UI_STATE_MGR_SWITCH_STATE: u32 = 0x0039_f400;
/// `UIStateMgr_SwitchStateImmediate` (0x39f320).
const UI_STATE_MGR_SWITCH_STATE_IMMEDIATE: u32 = 0x0039_f320;
/// `EA_MemSet` (0x3e6448).
const EA_MEM_SET: u32 = 0x003e_6448;
/// `UIListBox_GetSelectedValue` (0x39a738).
const UI_LIST_BOX_GET_SELECTED_VALUE: u32 = 0x0039_a738;
/// `cMemMan_alloc` (0x317d70).
const MEM_MAN_ALLOC: u32 = 0x0031_7d70;

/// The flow's "next state" method `{+0x20 delta, +0x24 fn}` (`slot` 0x20) or
/// "previous state" (`slot` 0x28) of the state manager's owner
/// (`**(self+0x10)`), called as `owner->m(self, widget->id)`; returns to
/// `ret` with the new state in v0.
fn rule_select_flow_call(c: &mut Cpu, this: u32, widget: u32, slot: u32, ret: u32) -> Flow {
    c.g[V0][0] = Addr(this.wrapping_add(0x10)).lw(c);
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = Addr(widget.wrapping_add(0x18)).lw(c);
    c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
    let owner = c.g[A3][0] as u32;
    c.g[V1][0] = Addr(owner.wrapping_add(4)).lw(c);
    let m = VSlot::read(c, c.g[V1][0], slot);
    c.g[A0][0] = m.delta;
    c.g[V0][0] = m.func;
    c.g[A0][0] = sx(owner.wrapping_add(m.delta as u32));
    call_ptr(c, m.func as u32, ret)
}

/// `void cFEStateRuleSelect_OnWidgetAction(self, UIWidget* w, int action)`
/// (0x191c68): 5 (accept, unless the widget's popup is busy) and 6 (back)
/// ask the flow for the next/previous state and switch to it (back also
/// clears the rule flags at 0x5308b8); 9 (value changed) stores the list
/// box's value for rule `w->id` and applies the rules that depend on it; 7
/// opens the options screen on top.
pub fn c_fe_state_rule_select_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S2][0] as u32;
        let widget = c.g[S1][0] as u32;
        // Epilogue depth: the original restores s0/s1/s2 along three exits
        // (+1, +1, +4 cycles); every path restores what it changed.
        let mut tail: u64 = 4;
        if c.g[S1][0] == 0 {
            tail = 6;
        } else {
            c.cycles += 3;
            let action = c.g[A2][0];
            c.g[V0][0] = (action < 7) as u64;
            if action == 6 {
                // Back: the flow's previous state, then clear the rule flags.
                c.cycles += 11;
                rule_select_flow_call(c, this, widget, 0x28, 0x0019_1d3c)?;
                c.g[A1][0] = c.g[V0][0];
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] =
                        sx((Addr(this.wrapping_add(0x10)).lw(c) as u32).wrapping_add(0x18));
                    call_ptr(c, UI_STATE_MGR_SWITCH_STATE, 0x0019_1d50)?;
                }
                c.cycles += 7;
                c.g[A0][0] = sx(0x0053_08b8);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20;
                call_ptr(c, EA_MEM_SET, 0x0019_1d64)?;
                tail = 5;
            } else if action < 7 {
                c.cycles += 2;
                c.g[V0][0] = 5;
                c.cycles += 2;
                if action == 5 {
                    // Accept, unless the widget's popup (+0x7c) is busy.
                    c.cycles += 4;
                    c.g[V0][0] = Addr(widget.wrapping_add(0x7c)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c);
                    if c.g[V1][0] == 0 {
                        c.cycles += 11;
                        rule_select_flow_call(c, this, widget, 0x20, 0x0019_1cfc)?;
                        c.g[A1][0] = c.g[V0][0];
                        if c.g[V0][0] == 0 {
                            tail = 6;
                        } else {
                            c.cycles += 5;
                            let mgr = Addr(this.wrapping_add(0x10)).lw(c) as u32;
                            c.g[A0][0] = sx(mgr.wrapping_add(0x18));
                            call_ptr(c, UI_STATE_MGR_SWITCH_STATE, 0x0019_1d10)?;
                            tail = 5;
                        }
                    }
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                c.cycles += 3;
                c.g[V0][0] = 9;
                if action == 7 {
                    // Open the options screen on top of this one.
                    c.cycles += 21;
                    c.g[A1][0] = sx(0x0045_d698);
                    c.g[A3][0] = 0;
                    c.g[A0][0] = 0x54;
                    c.g[A2][0] = 0x100;
                    call_ptr(c, MEM_MAN_ALLOC, 0x0019_1db4)?;
                    c.g[A1][0] = Addr(this.wrapping_add(0x10)).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 1;
                    callee::FE_STATE_OPTIONS_CTOR.call(c, 0x0019_1dc4)?;
                    let state = c.g[V0][0] as u32;
                    c.g[V1][0] = Addr(state.wrapping_add(0x1c)).lw(c);
                    c.g[A0][0] = 0xffff_ffff_ffff_c0ff;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0;
                    c.g[V1][0] &= c.g[A0][0];
                    c.g[A3][0] = 0;
                    c.g[V1][0] |= 0x100;
                    Addr(state.wrapping_add(0x1c)).sw(c, c.g[V1][0]);
                    c.g[A0][0] =
                        sx((Addr(this.wrapping_add(0x10)).lw(c) as u32).wrapping_add(0x18));
                    call_ptr(c, UI_STATE_MGR_SWITCH_STATE_IMMEDIATE, 0x0019_1df0)?;
                    tail = 6;
                } else {
                    c.cycles += 2;
                    if action == 9 {
                        // A rule's value changed.
                        c.cycles += 12;
                        c.g[S0][0] = Addr(widget.wrapping_add(0x18)).lw(c);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, UI_LIST_BOX_GET_SELECTED_VALUE, 0x0019_1d78)?;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[V0][0];
                        callee::SET_RULE_VALUE.call(c, 0x0019_1d88)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S1][0];
                        callee::APPLY_RULE_DEPENDENCIES.call(c, 0x0019_1d94)?;
                        tail = 5;
                    } else {
                        c.cycles += 2;
                    }
                }
            }
        }
        c.cycles += tail;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateRuleSelect::getBEData(cFEStateRuleSelect* this, int rule)` (0x192240).
///
/// Reads one rule setting of the back-end (`cBERules`) as 0/1: `rule` 1..=14
/// indexes a jump table (0x45fb90) whose cases test a bit of the rule words at
/// 0x5308b8 / 0x5308d0 (case 2 returns the whole word at 0x5308d4). Any other
/// `rule` returns -1.
pub fn cfe_state_rule_select_get_rule_value(c: &mut Cpu) {
    if c.pc != 0x0019_2240 {
        return cfe_state_rule_select_get_rule_value_resume(c);
    }
    c.cycles += 4;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
    c.g[V0][0] = (c.g[V1][0] < 0xe) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[V0][0] = 0x460000;
    if out_of_range {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x470));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    let target = c.g[A0][0] as u32;
    // (cycles, rule word, shift) of each case of the jump table.
    let (cycles, word, shift) = match target {
        0x0019_2268 => (4, RULE_WORD_D0, Shift::None),
        0x0019_2278 => (5, RULE_WORD_B8, Shift::Srl(3)),
        0x0019_228c => (5, RULE_WORD_D0, Shift::Sra(3)),
        0x0019_22a0 => (5, RULE_WORD_B8, Shift::Srl(5)),
        0x0019_22b4 => (5, RULE_WORD_B8, Shift::Srl(4)),
        0x0019_22c8 => (5, RULE_WORD_D0, Shift::Sra(1)),
        0x0019_22dc => (5, RULE_WORD_D0, Shift::Sra(2)),
        0x0019_22f0 => (3, RULE_WORD_D4, Shift::Whole),
        0x0019_22fc => (5, RULE_WORD_D0, Shift::Sra(4)),
        0x0019_2310 => (5, RULE_WORD_D0, Shift::Sra(5)),
        0x0019_2324 => (5, RULE_WORD_D0, Shift::Sra(6)),
        0x0019_2338 => (5, RULE_WORD_D0, Shift::Sra(7)),
        0x0019_234c => (5, RULE_WORD_D0, Shift::Sra(8)),
        0x0019_2360 => (5, RULE_WORD_D0, Shift::Sra(9)),
        _ => {
            c.jump_out(target, 0x0019_2240, 0x0019_237c);
            return;
        }
    };
    c.cycles += cycles;
    c.g[V1][0] = 0x530000;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(word));
    match shift {
        Shift::None => {}
        Shift::Srl(n) => c.g[V0][0] = sx((c.g[V0][0] as u32) >> n),
        Shift::Sra(n) => c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> n) as u32),
        Shift::Whole => {}
    }
    ret(c);
    if !matches!(shift, Shift::Whole) {
        c.g[V0][0] &= 1;
    }
}

/// Native resume of `cfe_state_rule_select_get_rule_value` (converted from the `entries` body): any entry
/// other than 0x00192240, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn cfe_state_rule_select_get_rule_value_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00192240 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x00192374;
                        continue;
                    }
                    pc = 0x00192250;
                    continue;
                }
                0x00192250 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x470));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00192268 => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00192278 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x0019228c => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x001922a0 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x001922b4 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x001922c8 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x001922dc => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x001922f0 => {
                    c.cycles += 3;
                    c.g[V1][0] = 0x530000;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d4));
                    c.pc = t;
                    return Ok(());
                }
                0x001922fc => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00192310 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00192324 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00192338 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x0019234c => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00192360 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 9) as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00192374 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00192240, 0x0019237c);
                    return Ok(());
                }
            }
        }
    });
}

/// The rule words below 0x530000 that `cfe_state_rule_select_get_rule_value` reads.
pub(crate) const RULE_WORD_B8: u32 = 0x8b8;

pub(crate) const RULE_WORD_D0: u32 = 0x8d0;

pub(crate) const RULE_WORD_D4: u32 = 0x8d4;

/// How a case of `cfe_state_rule_select_get_rule_value` picks its bit.
#[derive(Clone, Copy)]
pub(crate) enum Shift {
    /// Bit 0 of the word.
    None,
    /// `(word >> n) & 1`, logical shift.
    Srl(u32),
    /// `(word >> n) & 1`, arithmetic shift.
    Sra(u32),
    /// The whole word (no mask).
    Whole,
}

/// `void cFEStateRuleSelect_Init(cFEStateRuleSelect* self)` (0x191938).
///
/// Opens the "my rules" screen, picks the rule table and rule count of the
/// current event type, fills the rule list and highlights/enables the rows
/// (`FillRuleList`, per-row screen item flags and texts).
pub fn cfe_state_rule_select_init(c: &mut Cpu) {
    if c.pc != 0x00191938 {
        return cfe_state_rule_select_init_resume(c);
    }
    guest(c, |c| 'l_191938: loop {
        'b_1919a0: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = 0x460000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4f0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x00191974)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00191988)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1919a0;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001919a0)?;
                break 'b_1919a0;
            }
        }
        'b_191a38: {
            c.cycles += 11;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
            c.g[S4][0] = 0u64;
            call_ptr(c, 0x0014dc80, 0x001919ac)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x001919b8)?;
            c.g[V1][0] = 0x530000;
            c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c12));
            c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            if t {
                break 'b_191a38;
            } else {
                'b_191a34: {
                    'b_191a2c: {
                        'b_191a1c: {
                            'b_191a10: {
                                'b_191a04: {
                                    'b_1919f4: {
                                        c.cycles += 6;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4d0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x00191938 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00191938);
                                                }
                                                continue 'l_191938;
                                            }
                                            0x001919e4 => {
                                                c.cycles += 4;
                                                c.g[V0][0] = 0x440000;
                                                c.g[S4][0] = 0xb;
                                                let t = true;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1098));
                                                if t {
                                                    break 'b_191a34;
                                                } else {
                                                    break 'b_1919f4;
                                                }
                                            }
                                            0x001919f4 => {
                                                break 'b_1919f4;
                                            }
                                            0x00191a04 => {
                                                break 'b_191a04;
                                            }
                                            0x00191a10 => {
                                                break 'b_191a10;
                                            }
                                            0x00191a1c => {
                                                break 'b_191a1c;
                                            }
                                            0x00191a2c => {
                                                break 'b_191a2c;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x00191938, 0x00191c44);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = 0x440000;
                                    c.g[S4][0] = 0xa;
                                    let t = true;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10a8));
                                    if t {
                                        break 'b_191a34;
                                    } else {
                                        break 'b_191a04;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1a60));
                                let t = true;
                                c.g[S4][0] = 0x8;
                                if t {
                                    break 'b_191a34;
                                } else {
                                    break 'b_191a10;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1a58));
                            let t = true;
                            c.g[S4][0] = 0x8;
                            if t {
                                break 'b_191a34;
                            } else {
                                break 'b_191a1c;
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = 0x440000;
                        c.g[S4][0] = 0xb;
                        let t = true;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10b8));
                        if t {
                            break 'b_191a34;
                        } else {
                            break 'b_191a2c;
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1a50));
                    c.g[S4][0] = 0x8;
                    break 'b_191a34;
                }
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                break 'b_191a38;
            }
        }
        c.cycles += 4;
        c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = 0xe;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5f));
        'l_191a48: loop {
            c.cycles += 7;
            c.sb(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00191a48);
                }
                continue 'l_191a48;
            } else {
                'b_191b18: {
                    c.cycles += 18;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x50), c.g[S4][0]);
                    c.g[A0][0] = 0x530000;
                    c.g[A2][0] = 0x20;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x003e6448, 0x00191a7c)?;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x001913b8, 0x00191a88)?;
                    c.g[A0][0] = 0x4a0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
                    call_ptr(c, 0x00317670, 0x00191a94)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00191aa0)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_191b18;
                    } else {
                        c.cycles += 18;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[A1][0] = c.g[S4][0];
                        c.g[V0][0] = c.g[V0][0] | 0x80;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        call_ptr(c, 0x0039b760, 0x00191ac0)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0039b960, 0x00191acc)?;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e0));
                        call_ptr(c, 0x00317670, 0x00191ad8)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00191ae4)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                        let t = c.g[S0][0] == 0u64;
                        c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                        if t {
                            break 'b_191b18;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = 0x440000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1058));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                            call_ptr(c, 0x00317670, 0x00191b0c)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x00191b18)?;
                            break 'b_191b18;
                        }
                    }
                }
                c.cycles += 18;
                c.g[V0][0] = 0x4a0000;
                c.g[S2][0] = 0u64;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16a8));
                c.g[V1][0] = c.lb(c.g[A2][0] as u32);
                c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x2));
                c.sb(c.g[SP][0] as u32, c.g[V1][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[A1][0]);
                c.g[S6][0] = 0xfffffffffffffffe;
                c.g[V0][0] = 0x4a0000;
                c.g[S3][0] = 0x1000000;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13a0));
                c.g[V1][0] = c.lb(c.g[A2][0] as u32);
                c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[A0][0]);
                c.g[S5][0] = 0x1000000;
                'l_191b60: loop {
                    'b_191bd8: {
                        c.cycles += 17;
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00317670, 0x00191b68)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00191b74)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                        c.g[A0][0] = c.g[S7][0];
                        c.g[V1][0] = c.g[V1][0] & c.g[S6][0];
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                        call_ptr(c, 0x00317670, 0x00191b88)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00191b94)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                        if t {
                            break 'b_191bd8;
                        } else {
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00191bbc)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00191bd4)?;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                            break 'b_191bd8;
                        }
                    }
                    'b_191c04: {
                        'b_191bf8: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_191bf8;
                            } else {
                                c.cycles += 6;
                                c.g[V1][0] = c.lbu(c.g[SP][0] as u32);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                let t = true;
                                c.sb(c.g[SP][0] as u32, c.g[V1][0]);
                                if t {
                                    break 'b_191c04;
                                } else {
                                    break 'b_191bf8;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x58));
                        c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                        break 'b_191c04;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.g[S3][0];
                    c.g[S2][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 11i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00191b60);
                        }
                        continue 'l_191b60;
                    } else {
                        c.cycles += 11;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// Native resume of `cfe_state_rule_select_init` (converted from the `entries` body): any entry
/// other than 0x00191938, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn cfe_state_rule_select_init_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00191938 => {
                    c.cycles += 22;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A0][0] = 0x460000;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4f0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[RA][0] = sx(0x00191974);
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317670, 0x00191974)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00191988);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00397cc0, 0x00191988)?;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                    if t {
                        pc = 0x001919a0;
                        continue;
                    }
                    pc = 0x00191990;
                    continue;
                }
                0x00191990 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001919a0);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039c870, 0x001919a0)?;
                    pc = 0x001919a0;
                    continue;
                }
                0x001919a0 => {
                    c.cycles += 11;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
                    c.g[RA][0] = sx(0x001919ac);
                    c.g[S4][0] = 0u64;
                    call_ptr(c, 0x0014dc80, 0x001919ac)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001919b8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x001919b8)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c12));
                    c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    if t {
                        pc = 0x00191a38;
                        continue;
                    }
                    pc = 0x001919cc;
                    continue;
                }
                0x001919cc => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x460000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4d0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001919e4 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x440000;
                    c.g[S4][0] = 0xb;
                    let t = true;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1098));
                    if t {
                        pc = 0x00191a34;
                        continue;
                    }
                    pc = 0x001919f4;
                    continue;
                }
                0x001919f4 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x440000;
                    c.g[S4][0] = 0xa;
                    let t = true;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10a8));
                    if t {
                        pc = 0x00191a34;
                        continue;
                    }
                    pc = 0x00191a04;
                    continue;
                }
                0x00191a04 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1a60));
                    let t = true;
                    c.g[S4][0] = 0x8;
                    if t {
                        pc = 0x00191a34;
                        continue;
                    }
                    pc = 0x00191a10;
                    continue;
                }
                0x00191a10 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1a58));
                    let t = true;
                    c.g[S4][0] = 0x8;
                    if t {
                        pc = 0x00191a34;
                        continue;
                    }
                    pc = 0x00191a1c;
                    continue;
                }
                0x00191a1c => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x440000;
                    c.g[S4][0] = 0xb;
                    let t = true;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10b8));
                    if t {
                        pc = 0x00191a34;
                        continue;
                    }
                    pc = 0x00191a2c;
                    continue;
                }
                0x00191a2c => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1a50));
                    c.g[S4][0] = 0x8;
                    pc = 0x00191a34;
                    continue;
                }
                0x00191a34 => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                    pc = 0x00191a38;
                    continue;
                }
                0x00191a38 => {
                    c.cycles += 4;
                    c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = 0xe;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5f));
                    pc = 0x00191a48;
                    continue;
                }
                0x00191a48 => {
                    c.cycles += 7;
                    c.sb((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x00191a48) {
                            return Ok(());
                        }
                        pc = 0x00191a48;
                        continue;
                    }
                    pc = 0x00191a64;
                    continue;
                }
                0x00191a64 => {
                    c.cycles += 18;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x50), c.g[S4][0]);
                    c.g[A0][0] = 0x530000;
                    c.g[A2][0] = 0x20;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[RA][0] = sx(0x00191a7c);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x003e6448, 0x00191a7c)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00191a88);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x001913b8,
                        f: ee::s02::cFEStateRuleSelect_FillRuleList,
                    }
                    .call(c, 0x00191a88)?;
                    c.g[A0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00191a94);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
                    call_ptr(c, 0x00317670, 0x00191a94)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00191aa0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00191aa0)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00191b18;
                        continue;
                    }
                    pc = 0x00191aac;
                    continue;
                }
                0x00191aac => {
                    c.cycles += 18;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[V0][0] = c.g[V0][0] | 0x80;
                    c.g[RA][0] = sx(0x00191ac0);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    call_ptr(c, 0x0039b760, 0x00191ac0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00191acc);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0039b960, 0x00191acc)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00191ad8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e0));
                    call_ptr(c, 0x00317670, 0x00191ad8)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00191ae4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00191ae4)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                    let t = c.g[S0][0] == 0u64;
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32));
                    if t {
                        pc = 0x00191b18;
                        continue;
                    }
                    pc = 0x00191af4;
                    continue;
                }
                0x00191af4 => {
                    c.cycles += 9;
                    c.g[V0][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1058));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x00191b0c);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    call_ptr(c, 0x00317670, 0x00191b0c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00191b18);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00191b18)?;
                    pc = 0x00191b18;
                    continue;
                }
                0x00191b18 => {
                    c.cycles += 18;
                    c.g[V0][0] = 0x4a0000;
                    c.g[S2][0] = 0u64;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16a8));
                    c.g[V1][0] = c.lb((c.g[A2][0] as u32));
                    c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x2));
                    c.sb((c.g[SP][0] as u32), c.g[V1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[A1][0]);
                    c.g[S6][0] = 0xfffffffffffffffe;
                    c.g[V0][0] = 0x4a0000;
                    c.g[S3][0] = 0x1000000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13a0));
                    c.g[V1][0] = c.lb((c.g[A2][0] as u32));
                    c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[A0][0]);
                    c.g[S5][0] = 0x1000000;
                    pc = 0x00191b60;
                    continue;
                }
                0x00191b60 => {
                    c.cycles += 17;
                    c.g[RA][0] = sx(0x00191b68);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x00191b68)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00191b74);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00191b74)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = c.g[S7][0];
                    c.g[V1][0] = c.g[V1][0] & c.g[S6][0];
                    c.g[RA][0] = sx(0x00191b88);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    call_ptr(c, 0x00317670, 0x00191b88)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00191b94);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00191b94)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                    if t {
                        pc = 0x00191bd8;
                        continue;
                    }
                    pc = 0x00191ba4;
                    continue;
                }
                0x00191ba4 => {
                    c.cycles += 13;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00191bbc);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00191bbc)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00191bd4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00191bd4)?;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                    pc = 0x00191bd8;
                    continue;
                }
                0x00191bd8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x00191bf8;
                        continue;
                    }
                    pc = 0x00191be0;
                    continue;
                }
                0x00191be0 => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lbu((c.g[SP][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    let t = true;
                    c.sb((c.g[SP][0] as u32), c.g[V1][0]);
                    if t {
                        pc = 0x00191c04;
                        continue;
                    }
                    pc = 0x00191bf8;
                    continue;
                }
                0x00191bf8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x58));
                    c.sb((c.g[SP][0] as u32), c.g[V0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    pc = 0x00191c04;
                    continue;
                }
                0x00191c04 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.g[S3][0];
                    c.g[S2][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 11i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                    if t {
                        if back_edge_unwinds(c, 0x00191b60) {
                            return Ok(());
                        }
                        pc = 0x00191b60;
                        continue;
                    }
                    pc = 0x00191c18;
                    continue;
                }
                0x00191c18 => {
                    c.cycles += 11;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00191938, 0x00191c44);
                    return Ok(());
                }
            }
        }
    });
}

/// Row object (`cUIText`) virtual call: `obj->vt[off](obj + delta, a1)` with
/// `a1` already set by the caller. `obj` is a register index (s0 or s1).
pub(crate) fn rule_row_vcall(c: &mut Cpu, obj: usize, off: u32, ret: u32) -> Flow {
    c.g[V0][0] = c.lw((c.g[obj][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(off + 4));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// Copies the 16-byte static colour (0x4c66c8) into `obj + 0x88`, with the
/// original's unaligned loads in registers `lo` and `lo + 1`.
pub(crate) fn rule_row_copy_colour(c: &mut Cpu, src_reg: usize, lo: usize, obj: usize) {
    c.ldl(lo, (c.g[src_reg][0] as u32).wrapping_add(0x7));
    c.ldr(lo, c.g[src_reg][0] as u32);
    c.ldl(lo + 1, (c.g[src_reg][0] as u32).wrapping_add(0xf));
    c.ldr(lo + 1, (c.g[src_reg][0] as u32).wrapping_add(0x8));
    c.sdl(lo, (c.g[obj][0] as u32).wrapping_add(0x8f));
    c.sdr(lo, (c.g[obj][0] as u32).wrapping_add(0x88));
    c.sdl(lo + 1, (c.g[obj][0] as u32).wrapping_add(0x97));
    c.sdr(lo + 1, (c.g[obj][0] as u32).wrapping_add(0x90));
}

/// `void cFEStateRuleSelect::UpdateRowVisibility(cFEStateRuleSelect* this)` (0x192488).
///
/// For each of the first `min(rowCount (+0x50), 11)` rows it looks up the
/// screen objects named "0b".."9b", "ab".. (list box) and "0t".."9t", "at"..
/// (text box) by hash (`this->screen (+0x40)`). When both exist: if the
/// row's flag byte (`this + rowData(+0x18) + 0x51`) is positive both get the
/// static colour (0x4c66c8) and are shown, otherwise they are hidden. The
/// names advance one character per row (digits, then `a`..).
pub fn cfe_state_rule_select_update_row_visibility(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.g[V0][0] = 0x4a0000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S3][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A0][0] = 0x4a0000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[V1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(RULE_ROW_COUNT));
        // name0 = "0b" (0x4a16a8), name1 = "0t" (0x4a16b0): 3 bytes each.
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16a8));
        c.g[A1][0] = c.lb(c.g[T0][0] as u32);
        c.g[A2][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x1));
        c.g[A3][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x2));
        c.sb(c.g[SP][0] as u32, c.g[A1][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A2][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[A3][0]);
        c.g[V0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(RULE_ROW_COUNT));
        c.g[V1][0] = ((c.g[V1][0] as i64) < 12i64) as u64;
        c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x16b0));
        c.g[A1][0] = c.lb(c.g[T0][0] as u32);
        c.g[A2][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x1));
        c.g[A3][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x2));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[A2][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x12), c.g[A3][0]);
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0xb;
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        c.g[S4][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        let no_rows = (c.g[S4][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !no_rows {
            c.cycles += 3;
            c.g[S6][0] = 0x4c0000;
            c.g[S5][0] = 0x1000000;
            loop {
                c.cycles += 13;
                // Both objects by name hash.
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_7670, 0x0019_2528)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RULE_SCREEN));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0019_2534)?;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, 0x0031_7670, 0x0019_2540)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RULE_SCREEN));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0019_254c)?;
                let no_list_box = c.g[S1][0] == 0;
                c.g[S0][0] = c.g[V0][0];
                if no_list_box {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                } else {
                    c.cycles += 2;
                    let no_text_box = c.g[S0][0] == 0;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                    if !no_text_box {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(RULE_ROW_DATA_OFF));
                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x51));
                        let hidden = (c.g[V1][0] as i64) <= 0;
                        c.g[A1][0] = 0x1;
                        if !hidden {
                            // Show both rows in the static colour.
                            c.cycles += 43;
                            rule_row_vcall(c, S0, 0xc8, 0x0019_2584)?;
                            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x66c8));
                            rule_row_copy_colour(c, A1, 2, S0);
                            c.g[A1][0] = 0x1;
                            rule_row_vcall(c, S0, 0x40, 0x0019_25c0)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc0));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc4));
                            let method = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, method, 0x0019_25d8)?;
                            c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x66c8));
                            rule_row_copy_colour(c, V0, 6, S1);
                            c.g[A1][0] = 0x1;
                            rule_row_vcall(c, S1, 0x40, 0x0019_2614)?;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                        } else {
                            // Hide both rows.
                            c.cycles += 24;
                            c.g[A1][0] = 0;
                            rule_row_vcall(c, S0, 0xc8, 0x0019_2634)?;
                            c.g[A1][0] = 0;
                            rule_row_vcall(c, S0, 0x40, 0x0019_264c)?;
                            c.g[A1][0] = 0;
                            // (the original loads v0 = [s1+8] before a1 = 0: same result)
                            rule_row_vcall(c, S1, 0xc0, 0x0019_2664)?;
                            c.g[A1][0] = 0;
                            rule_row_vcall(c, S1, 0x40, 0x0019_267c)?;
                            c.cycles += 1;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                        }
                    }
                }
                // Advance the names: one character per row, then 'a'.. .
                c.cycles += 2;
                let past_digits = c.g[V0][0] == 0;
                c.g[V0][0] = c.lbu(c.g[SP][0] as u32);
                if !past_digits {
                    c.cycles += 6;
                    c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x58));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                }
                c.cycles += 6;
                c.g[V0][0] = c.g[S5][0];
                c.g[V1][0] = 0x1000000;
                c.g[S2][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_2520);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = ra;
        Ok(())
    })
}

/// `cUITemplate_MAP` fields of `event_select_update_confirm_popup`.
pub(crate) const POPUP_ACTIVE: u32 = 0x2cc;

/// `cFEStateRuleSelect` fields of `cfe_state_rule_select_update_row_visibility`.
pub(crate) const RULE_ROW_COUNT: u32 = 0x50;

pub(crate) const RULE_SCREEN: u32 = 0x40;

pub(crate) const RULE_ROW_DATA_OFF: u32 = 0x18;

/// `void cFEStateRuleSelect_FillRuleList(self, signed char first)`
/// (0x001913b8): fills the ten rule rows (listbox `0..9` and label `0t..9t`,
/// then `a`) from the game-rule mapping: label text, list entries by rule kind
/// (jump table at 0x45faf0: off/on, no/yes, percent, seconds, ... ), the
/// current value; then the arrow widgets and the menu frame (normal or
/// greyed out) and the row visibility.
pub fn rule_select_fill_rule_list(c: &mut Cpu) {
    if c.pc != 0x001913b8 {
        return rule_select_fill_rule_list_resume(c);
    }
    guest(c, |c| {
        c.cycles += 34;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
        c.g[V0][0] = 0x4a0000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[V1][0] = 0x4a0000;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S3][0] = c.g[A0][0];
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S5][0] = 0u64;
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S7][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16a8));
        c.g[A2][0] = c.lb(c.g[T1][0] as u32);
        c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x1));
        c.g[T0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x2));
        c.sb(c.g[SP][0] as u32, c.g[A2][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A3][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[T0][0]);
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x16b0));
        c.g[V0][0] = c.lb(c.g[A3][0] as u32);
        c.g[A1][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x1));
        c.g[A2][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x2));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[A1][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x12), c.g[A2][0]);
        'l_191440: loop {
            'b_1917f0: {
                'b_1917ac: {
                    'b_1917a8: {
                        c.cycles += 13;
                        c.g[RA][0] = sx(0x00191448);
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00317670, 0x00191448)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x00191454);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00191454)?;
                        c.g[A0][0] = c.g[S6][0];
                        c.g[RA][0] = sx(0x00191460);
                        c.g[S0][0] = c.g[V0][0];
                        call_ptr(c, 0x00317670, 0x00191460)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x0019146c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x0019146c)?;
                        let t = c.g[S0][0] == 0u64;
                        c.g[S2][0] = c.g[V0][0];
                        if t {
                            break 'b_1917a8;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S2][0] == 0u64;
                            c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S7][0] as u32));
                            if t {
                                break 'b_1917a8;
                            } else {
                                c.cycles += 6;
                                c.g[V1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x50));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                let t = c.g[V1][0] == 0u64;
                                c.g[A1][0] = 0x440000;
                                if t {
                                    break 'b_1917f0;
                                } else {
                                    'b_19175c: {
                                        c.cycles += 20;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x48));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1018));
                                        c.g[V1][0] = 0x440000;
                                        c.g[A0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1008));
                                        c.g[S4][0] = c.lb(c.g[A0][0] as u32);
                                        c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                                        c.g[V1][0] =
                                            sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                        c.g[S1][0] = c.lb(c.g[V1][0] as u32);
                                        c.g[RA][0] = sx(0x001914c4);
                                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                                        call_ptr(c, 0x00317670, 0x001914c4)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001914d0);
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x003a1000, 0x001914d0)?;
                                        c.g[RA][0] = sx(0x001914d8);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x0039a670, 0x001914d8)?;
                                        c.g[V0][0] = (c.g[S1][0] < 0x7u64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                                        if t {
                                            break 'b_19175c;
                                        } else {
                                            'b_191708: {
                                                'b_191688: {
                                                    'b_1915e0: {
                                                        'b_19156c: {
                                                            'b_19154c: {
                                                                'b_191528: {
                                                                    c.cycles += 7;
                                                                    c.g[V0][0] = 0x460000;
                                                                    c.g[V1][0] = sx((c.g[S1][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x510));
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V0][0] as u32,
                                                                        ));
                                                                    c.g[A0][0] =
                                                                        c.lw(c.g[V1][0] as u32);
                                                                    let t = c.g[A0][0] as u32;
                                                                    match t {
                                                                        0x00191500 => {
                                                                            c.cycles += 10;
                                                                            c.g[A0][0] = 0x460000;
                                                                            c.g[RA][0] =
                                                                                sx(0x0019150c);
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x20c0,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, 0x00317670,
                                                                                0x0019150c,
                                                                            )?;
                                                                            c.g[A1][0] = c.g[V0][0];
                                                                            c.g[A2][0] = 0u64;
                                                                            c.g[RA][0] =
                                                                                sx(0x0019151c);
                                                                            c.g[A0][0] = c.g[S0][0];
                                                                            call_ptr(
                                                                                c, 0x0039a568,
                                                                                0x0019151c,
                                                                            )?;
                                                                            c.g[A0][0] = 0x460000;
                                                                            let t = true;
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x1fd8,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_19154c;
                                                                            } else {
                                                                                break 'b_191528;
                                                                            }
                                                                        }
                                                                        0x00191528 => {
                                                                            break 'b_191528;
                                                                        }
                                                                        0x0019156c => {
                                                                            break 'b_19156c;
                                                                        }
                                                                        0x001915e0 => {
                                                                            break 'b_1915e0;
                                                                        }
                                                                        0x00191688 => {
                                                                            break 'b_191688;
                                                                        }
                                                                        0x00191708 => {
                                                                            break 'b_191708;
                                                                        }
                                                                        0x0019175c => {
                                                                            break 'b_19175c;
                                                                        }
                                                                        _ => {
                                                                            c.jump_out(
                                                                                t, 0x001913b8,
                                                                                0x00191938,
                                                                            );
                                                                            return Ok(());
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 9;
                                                                c.g[A0][0] = 0x460000;
                                                                c.g[RA][0] = sx(0x00191534);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x580));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x00191534,
                                                                )?;
                                                                c.g[A1][0] = c.g[V0][0];
                                                                c.g[A2][0] = 0u64;
                                                                c.g[RA][0] = sx(0x00191544);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                call_ptr(
                                                                    c, 0x0039a568, 0x00191544,
                                                                )?;
                                                                c.g[A0][0] = 0x460000;
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x2338));
                                                                break 'b_19154c;
                                                            }
                                                            c.cycles += 8;
                                                            c.g[RA][0] = sx(0x00191554);
                                                            call_ptr(c, 0x00317670, 0x00191554)?;
                                                            c.g[A1][0] = c.g[V0][0];
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x00191564);
                                                            c.g[A2][0] = 0x1;
                                                            call_ptr(c, 0x0039a568, 0x00191564)?;
                                                            let t = true;

                                                            if t {
                                                                break 'b_19175c;
                                                            } else {
                                                                break 'b_19156c;
                                                            }
                                                        }
                                                        c.cycles += 29;
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[RA][0] = sx(0x00191578);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x20c0));
                                                        call_ptr(c, 0x00317670, 0x00191578)?;
                                                        c.g[A1][0] = c.g[V0][0];
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x00191588);
                                                        c.g[A2][0] = 0u64;
                                                        call_ptr(c, 0x0039a568, 0x00191588)?;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16b8));
                                                        c.g[RA][0] = sx(0x0019159c);
                                                        c.g[A2][0] = 0x5a;
                                                        call_ptr(c, 0x0039a3f8, 0x0019159c)?;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16c0));
                                                        c.g[RA][0] = sx(0x001915b0);
                                                        c.g[A2][0] = 0x50;
                                                        call_ptr(c, 0x0039a3f8, 0x001915b0)?;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16c8));
                                                        c.g[RA][0] = sx(0x001915c4);
                                                        c.g[A2][0] = 0x46;
                                                        call_ptr(c, 0x0039a3f8, 0x001915c4)?;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16d0));
                                                        c.g[RA][0] = sx(0x001915d8);
                                                        c.g[A2][0] = 0x3c;
                                                        call_ptr(c, 0x0039a3f8, 0x001915d8)?;
                                                        let t = true;

                                                        if t {
                                                            break 'b_19175c;
                                                        } else {
                                                            break 'b_1915e0;
                                                        }
                                                    }
                                                    c.cycles += 42;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16d8));
                                                    c.g[RA][0] = sx(0x001915f4);
                                                    c.g[A2][0] = 0xa;
                                                    call_ptr(c, 0x0039a3f8, 0x001915f4)?;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16e0));
                                                    c.g[RA][0] = sx(0x00191608);
                                                    c.g[A2][0] = 0x5;
                                                    call_ptr(c, 0x0039a3f8, 0x00191608)?;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A2][0] = 0x3;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16e8));
                                                    c.g[RA][0] = sx(0x0019161c);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x0039a3f8, 0x0019161c)?;
                                                    c.g[A0][0] = 0x460000;
                                                    c.g[RA][0] = sx(0x00191628);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x20c0));
                                                    call_ptr(c, 0x00317670, 0x00191628)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x00191638);
                                                    c.g[A2][0] = 0u64;
                                                    call_ptr(c, 0x0039a568, 0x00191638)?;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A2][0] = 0xffffffffffff0000;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16f0));
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x00191650);
                                                    c.g[A2][0] = c.g[A2][0] | 0xfffd;
                                                    call_ptr(c, 0x0039a3f8, 0x00191650)?;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A2][0] = 0xffffffffffff0000;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16f8));
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x00191668);
                                                    c.g[A2][0] = c.g[A2][0] | 0xfffb;
                                                    call_ptr(c, 0x0039a3f8, 0x00191668)?;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A2][0] = 0xffffffffffff0000;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1700));
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x00191680);
                                                    c.g[A2][0] = c.g[A2][0] | 0xfff6;
                                                    call_ptr(c, 0x0039a3f8, 0x00191680)?;
                                                    let t = true;

                                                    if t {
                                                        break 'b_19175c;
                                                    } else {
                                                        break 'b_191688;
                                                    }
                                                }
                                                c.cycles += 32;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x13a0));
                                                c.g[RA][0] = sx(0x0019169c);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x0019169c)?;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x14f8));
                                                c.g[RA][0] = sx(0x001916b0);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x001916b0)?;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1500));
                                                c.g[RA][0] = sx(0x001916c4);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x001916c4)?;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1508));
                                                c.g[RA][0] = sx(0x001916d8);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x001916d8)?;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1510));
                                                c.g[RA][0] = sx(0x001916ec);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x001916ec)?;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1518));
                                                c.g[RA][0] = sx(0x00191700);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x00191700)?;
                                                let t = true;

                                                if t {
                                                    break 'b_19175c;
                                                } else {
                                                    break 'b_191708;
                                                }
                                            }
                                            c.cycles += 21;
                                            c.g[A0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x00191714);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x570));
                                            call_ptr(c, 0x00317670, 0x00191714)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x00191724);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x0039a568, 0x00191724)?;
                                            c.g[A0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x00191730);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x558));
                                            call_ptr(c, 0x00317670, 0x00191730)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x00191740);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x0039a568, 0x00191740)?;
                                            c.g[A0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x0019174c);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x540));
                                            call_ptr(c, 0x00317670, 0x0019174c)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0019175c);
                                            c.g[A2][0] = 0u64;
                                            call_ptr(c, 0x0039a568, 0x0019175c)?;
                                            break 'b_19175c;
                                        }
                                    }
                                    'b_191790: {
                                        c.cycles += 2;
                                        let t = c.g[S4][0] == 0u64;
                                        c.g[A1][0] = c.g[S4][0];
                                        if t {
                                            break 'b_191790;
                                        } else {
                                            c.cycles += 11;
                                            c.g[RA][0] = sx(0x0019176c);
                                            c.g[A0][0] = c.g[S3][0];
                                            Callee {
                                                addr: 0x00192240,
                                                f: ee::s02::cFEStateRuleSelect_GetRuleValue,
                                            }
                                            .call(c, 0x0019176c)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0019177c);
                                            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                            call_ptr(c, 0x0039a768, 0x0019177c)?;
                                            c.g[A1][0] =
                                                c.lbu((c.g[SP][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x00191788);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x0039a7a8, 0x00191788)?;
                                            let t = true;
                                            c.g[V0][0] = ((c.g[S5][0] as i64) < 9i64) as u64;
                                            if t {
                                                break 'b_1917ac;
                                            } else {
                                                break 'b_191790;
                                            }
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001917a8);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001917a8)?;
                                    break 'b_1917a8;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[S5][0] as i64) < 9i64) as u64;
                    break 'b_1917ac;
                }
                'b_1917d8: {
                    'b_1917cc: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lbu(c.g[SP][0] as u32);
                        if t {
                            break 'b_1917cc;
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                            let t = true;
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                            if t {
                                break 'b_1917d8;
                            } else {
                                break 'b_1917cc;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x58));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                    break 'b_1917d8;
                }
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[S5][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V1][0] = ((c.g[S5][0] as i64) < 11i64) as u64;
                let t = c.g[V1][0] != 0u64;

                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00191440);
                    }
                    continue 'l_191440;
                } else {
                    break 'b_1917f0;
                }
            }
            'b_191814: {
                c.cycles += 9;
                c.g[A0][0] = 0x4a0000;
                c.g[S0][0] = 0xff;
                c.g[RA][0] = sx(0x00191800);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
                call_ptr(c, 0x00317670, 0x00191800)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x0019180c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x0019180c)?;
                if c.g[V0][0] != 0u64 {
                    c.g[S0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                    break 'b_191814;
                } else {
                    break 'b_191814;
                }
            }
            'b_1918d0: {
                'b_1918bc: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        break 'b_1918bc;
                    } else {
                        'b_19187c: {
                            c.cycles += 15;
                            c.g[A0][0] = 0x460000;
                            c.g[RA][0] = sx(0x00191834);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d90));
                            call_ptr(c, 0x00317670, 0x00191834)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x00191840);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00191840)?;
                            c.g[A0][0] = 0x460000;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00191850);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d80));
                            call_ptr(c, 0x00317670, 0x00191850)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x0019185c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0019185c)?;
                            let t = c.g[S0][0] == 0u64;
                            c.g[S1][0] = c.g[V0][0];
                            if t {
                                break 'b_19187c;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x0019187c);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0019187c)?;
                                break 'b_19187c;
                            }
                        }
                        'b_191898: {
                            c.cycles += 2;
                            let t = c.g[S1][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_191898;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00191898);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00191898)?;
                                break 'b_191898;
                            }
                        }
                        c.cycles += 9;
                        c.g[V0][0] = 0x4a0000;
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1708));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1708));
                        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x6));
                        c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                        c.sh((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                        let t = true;
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x36), c.g[A1][0]);
                        if t {
                            break 'b_1918d0;
                        } else {
                            break 'b_1918bc;
                        }
                    }
                }
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x528));
                c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x528));
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A1][0]);
                break 'b_1918d0;
            }
            'b_191900: {
                c.cycles += 9;
                c.g[RA][0] = sx(0x001918d8);
                c.g[A0][0] = c.g[FP][0];
                call_ptr(c, 0x00317670, 0x001918d8)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001918e4);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039c778, 0x001918e4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if t {
                    break 'b_191900;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00191900);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039c870, 0x00191900)?;
                    break 'b_191900;
                }
            }
            c.cycles += 14;
            c.g[RA][0] = sx(0x00191908);
            c.g[A0][0] = c.g[S3][0];
            Callee {
                addr: 0x00192488,
                f: ee::s02::cFEStateRuleSelect_UpdateRowVisibility,
            }
            .call(c, 0x00191908)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
            c.pc = t;
            return Ok(());
        }
    });
}

/// Native resume of `rule_select_fill_rule_list` (converted from the `entries` body): any entry
/// other than 0x001913b8, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn rule_select_fill_rule_list_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x001913b8 => {
                    c.cycles += 34;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
                    c.g[V0][0] = 0x4a0000;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[V1][0] = 0x4a0000;
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S5][0] = 0u64;
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S7][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
                    c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16a8));
                    c.g[A2][0] = c.lb((c.g[T1][0] as u32));
                    c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x1));
                    c.g[T0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x2));
                    c.sb((c.g[SP][0] as u32), c.g[A2][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A3][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[T0][0]);
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x16b0));
                    c.g[V0][0] = c.lb((c.g[A3][0] as u32));
                    c.g[A1][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[A2][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x2));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[A1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x12), c.g[A2][0]);
                    pc = 0x00191440;
                    continue;
                }
                0x00191440 => {
                    c.cycles += 13;
                    c.g[RA][0] = sx(0x00191448);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x00191448)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00191454);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00191454)?;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00191460);
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, 0x00317670, 0x00191460)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0019146c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0019146c)?;
                    let t = c.g[S0][0] == 0u64;
                    c.g[S2][0] = c.g[V0][0];
                    if t {
                        pc = 0x001917a8;
                        continue;
                    }
                    pc = 0x00191474;
                    continue;
                }
                0x00191474 => {
                    c.cycles += 2;
                    let t = c.g[S2][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                    if t {
                        pc = 0x001917a8;
                        continue;
                    }
                    pc = 0x0019147c;
                    continue;
                }
                0x0019147c => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[A1][0] = 0x440000;
                    if t {
                        pc = 0x001917f0;
                        continue;
                    }
                    pc = 0x00191494;
                    continue;
                }
                0x00191494 => {
                    c.cycles += 20;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x48));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1018));
                    c.g[V1][0] = 0x440000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1008));
                    c.g[S4][0] = c.lb((c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[S1][0] = c.lb((c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x001914c4);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001914c4)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001914d0);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x003a1000, 0x001914d0)?;
                    c.g[RA][0] = sx(0x001914d8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a670, 0x001914d8)?;
                    c.g[V0][0] = (c.g[S1][0] < 0x7u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                    if t {
                        pc = 0x0019175c;
                        continue;
                    }
                    pc = 0x001914e4;
                    continue;
                }
                0x001914e4 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x510));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00191500 => {
                    c.cycles += 10;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0019150c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
                    call_ptr(c, 0x00317670, 0x0019150c)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0019151c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x0019151c)?;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fd8));
                    if t {
                        pc = 0x0019154c;
                        continue;
                    }
                    pc = 0x00191528;
                    continue;
                }
                0x00191528 => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00191534);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x580));
                    call_ptr(c, 0x00317670, 0x00191534)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00191544);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x00191544)?;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                    pc = 0x0019154c;
                    continue;
                }
                0x0019154c => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x00191554);
                    call_ptr(c, 0x00317670, 0x00191554)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00191564);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039a568, 0x00191564)?;
                    let t = true;

                    if t {
                        pc = 0x0019175c;
                        continue;
                    }
                    pc = 0x0019156c;
                    continue;
                }
                0x0019156c => {
                    c.cycles += 29;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00191578);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
                    call_ptr(c, 0x00317670, 0x00191578)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00191588);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a568, 0x00191588)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16b8));
                    c.g[RA][0] = sx(0x0019159c);
                    c.g[A2][0] = 0x5a;
                    call_ptr(c, 0x0039a3f8, 0x0019159c)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16c0));
                    c.g[RA][0] = sx(0x001915b0);
                    c.g[A2][0] = 0x50;
                    call_ptr(c, 0x0039a3f8, 0x001915b0)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16c8));
                    c.g[RA][0] = sx(0x001915c4);
                    c.g[A2][0] = 0x46;
                    call_ptr(c, 0x0039a3f8, 0x001915c4)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16d0));
                    c.g[RA][0] = sx(0x001915d8);
                    c.g[A2][0] = 0x3c;
                    call_ptr(c, 0x0039a3f8, 0x001915d8)?;
                    let t = true;

                    if t {
                        pc = 0x0019175c;
                        continue;
                    }
                    pc = 0x001915e0;
                    continue;
                }
                0x001915e0 => {
                    c.cycles += 42;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16d8));
                    c.g[RA][0] = sx(0x001915f4);
                    c.g[A2][0] = 0xa;
                    call_ptr(c, 0x0039a3f8, 0x001915f4)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16e0));
                    c.g[RA][0] = sx(0x00191608);
                    c.g[A2][0] = 0x5;
                    call_ptr(c, 0x0039a3f8, 0x00191608)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = 0x3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16e8));
                    c.g[RA][0] = sx(0x0019161c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a3f8, 0x0019161c)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00191628);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x20c0));
                    call_ptr(c, 0x00317670, 0x00191628)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00191638);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a568, 0x00191638)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = 0xffffffffffff0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16f0));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00191650);
                    c.g[A2][0] = c.g[A2][0] | 0xfffd;
                    call_ptr(c, 0x0039a3f8, 0x00191650)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = 0xffffffffffff0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x16f8));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00191668);
                    c.g[A2][0] = c.g[A2][0] | 0xfffb;
                    call_ptr(c, 0x0039a3f8, 0x00191668)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = 0xffffffffffff0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1700));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00191680);
                    c.g[A2][0] = c.g[A2][0] | 0xfff6;
                    call_ptr(c, 0x0039a3f8, 0x00191680)?;
                    let t = true;

                    if t {
                        pc = 0x0019175c;
                        continue;
                    }
                    pc = 0x00191688;
                    continue;
                }
                0x00191688 => {
                    c.cycles += 32;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x13a0));
                    c.g[RA][0] = sx(0x0019169c);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a3f8, 0x0019169c)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14f8));
                    c.g[RA][0] = sx(0x001916b0);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a3f8, 0x001916b0)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1500));
                    c.g[RA][0] = sx(0x001916c4);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a3f8, 0x001916c4)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1508));
                    c.g[RA][0] = sx(0x001916d8);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a3f8, 0x001916d8)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1510));
                    c.g[RA][0] = sx(0x001916ec);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a3f8, 0x001916ec)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1518));
                    c.g[RA][0] = sx(0x00191700);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a3f8, 0x00191700)?;
                    let t = true;

                    if t {
                        pc = 0x0019175c;
                        continue;
                    }
                    pc = 0x00191708;
                    continue;
                }
                0x00191708 => {
                    c.cycles += 21;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00191714);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x570));
                    call_ptr(c, 0x00317670, 0x00191714)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00191724);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x00191724)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00191730);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x558));
                    call_ptr(c, 0x00317670, 0x00191730)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x00191740);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a568, 0x00191740)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x0019174c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x540));
                    call_ptr(c, 0x00317670, 0x0019174c)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0019175c);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039a568, 0x0019175c)?;
                    pc = 0x0019175c;
                    continue;
                }
                0x0019175c => {
                    c.cycles += 2;
                    let t = c.g[S4][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        pc = 0x00191790;
                        continue;
                    }
                    pc = 0x00191764;
                    continue;
                }
                0x00191764 => {
                    c.cycles += 11;
                    c.g[RA][0] = sx(0x0019176c);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00192240,
                        f: ee::s02::cFEStateRuleSelect_GetRuleValue,
                    }
                    .call(c, 0x0019176c)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0019177c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    call_ptr(c, 0x0039a768, 0x0019177c)?;
                    c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x00191788);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039a7a8, 0x00191788)?;
                    let t = true;
                    c.g[V0][0] = ((c.g[S5][0] as i64) < 9i64) as u64;
                    if t {
                        pc = 0x001917ac;
                        continue;
                    }
                    pc = 0x00191790;
                    continue;
                }
                0x00191790 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001917a8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001917a8)?;
                    pc = 0x001917a8;
                    continue;
                }
                0x001917a8 => {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[S5][0] as i64) < 9i64) as u64;
                    pc = 0x001917ac;
                    continue;
                }
                0x001917ac => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lbu((c.g[SP][0] as u32));
                    if t {
                        pc = 0x001917cc;
                        continue;
                    }
                    pc = 0x001917b4;
                    continue;
                }
                0x001917b4 => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sb((c.g[SP][0] as u32), c.g[V0][0]);
                    let t = true;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    if t {
                        pc = 0x001917d8;
                        continue;
                    }
                    pc = 0x001917cc;
                    continue;
                }
                0x001917cc => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x58));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sb((c.g[SP][0] as u32), c.g[V0][0]);
                    pc = 0x001917d8;
                    continue;
                }
                0x001917d8 => {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                    c.g[S5][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[V1][0] = ((c.g[S5][0] as i64) < 11i64) as u64;
                    let t = c.g[V1][0] != 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x00191440) {
                            return Ok(());
                        }
                        pc = 0x00191440;
                        continue;
                    }
                    pc = 0x001917f0;
                    continue;
                }
                0x001917f0 => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x4a0000;
                    c.g[S0][0] = 0xff;
                    c.g[RA][0] = sx(0x00191800);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
                    call_ptr(c, 0x00317670, 0x00191800)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0019180c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0019180c)?;

                    if c.g[V0][0] != 0u64 {
                        c.g[S0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                        pc = 0x00191814;
                        continue;
                    }
                    pc = 0x00191814;
                    continue;
                }
                0x00191814 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32));
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001918bc;
                        continue;
                    }
                    pc = 0x00191828;
                    continue;
                }
                0x00191828 => {
                    c.cycles += 15;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00191834);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d90));
                    call_ptr(c, 0x00317670, 0x00191834)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00191840);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00191840)?;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00191850);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d80));
                    call_ptr(c, 0x00317670, 0x00191850)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0019185c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0019185c)?;
                    let t = c.g[S0][0] == 0u64;
                    c.g[S1][0] = c.g[V0][0];
                    if t {
                        pc = 0x0019187c;
                        continue;
                    }
                    pc = 0x00191864;
                    continue;
                }
                0x00191864 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0019187c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0019187c)?;
                    pc = 0x0019187c;
                    continue;
                }
                0x0019187c => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00191898;
                        continue;
                    }
                    pc = 0x00191884;
                    continue;
                }
                0x00191884 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00191898);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00191898)?;
                    pc = 0x00191898;
                    continue;
                }
                0x00191898 => {
                    c.cycles += 9;
                    c.g[V0][0] = 0x4a0000;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1708));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1708));
                    c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x6));
                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    let t = true;
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x36), c.g[A1][0]);
                    if t {
                        pc = 0x001918d0;
                        continue;
                    }
                    pc = 0x001918bc;
                    continue;
                }
                0x001918bc => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x528));
                    c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x528));
                    c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A1][0]);
                    pc = 0x001918d0;
                    continue;
                }
                0x001918d0 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001918d8);
                    c.g[A0][0] = c.g[FP][0];
                    call_ptr(c, 0x00317670, 0x001918d8)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001918e4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039c778, 0x001918e4)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[A1][0] = c.g[A1][0] & 0xffff;
                    if t {
                        pc = 0x00191900;
                        continue;
                    }
                    pc = 0x001918f4;
                    continue;
                }
                0x001918f4 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00191900);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039c870, 0x00191900)?;
                    pc = 0x00191900;
                    continue;
                }
                0x00191900 => {
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x00191908);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00192488,
                        f: ee::s02::cFEStateRuleSelect_UpdateRowVisibility,
                    }
                    .call(c, 0x00191908)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001913b8, 0x00191938);
                    return Ok(());
                }
            }
        }
    });
}

/// `cFEStateRuleSelect::OnActivate` (0x191c48): forwards to `UIState_OnActivate`
/// (0x39e4c0) through the function table, returning its result.
pub fn fe_state_rule_select_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0039_e4c0), 0x0019_1c58);
}

/// `int cFEStateRuleSelect_OnWidgetNotify(self, w, kind, id)` (0x00191e48): kind 4 refills the
/// rule list (`FillRuleList`, returns 0x101); kind 1 (a rule changed) greys or
/// re-enables the `b`/`t` widgets of the row `id` by its rule flag and may start
/// the frame thread (`UIScreen_LookupValue`/`StartThread`); the other kinds
/// return 0x101 without work.
pub fn cfe_state_rule_select_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        'b_192060: {
            'b_19205c: {
                'b_192048: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.g[V0][0] = 0x4;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    if t {
                        break 'b_192048;
                    } else {
                        'b_191ea8: {
                            'b_191e94: {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[A2][0] < 0x5u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_191e94;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x101;
                                    if t {
                                        break 'b_191ea8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                                        if t {
                                            break 'b_192060;
                                        } else {
                                            break 'b_191e94;
                                        }
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = 0x9;
                            let t = c.g[A2][0] != c.g[V0][0];
                            c.g[V0][0] = 0x101;
                            if t {
                                break 'b_19205c;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0x100;
                                if t {
                                    break 'b_19205c;
                                } else {
                                    break 'b_191ea8;
                                }
                            }
                        }
                        'b_191f44: {
                            c.cycles += 29;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x48));
                            c.g[A0][0] = 0x460000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d90));
                            c.g[S6][0] = 0u64;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                            c.g[RA][0] = sx(0x00191ec4);
                            c.g[S4][0] = c.lb(c.g[V0][0] as u32);
                            call_ptr(c, 0x00317670, 0x00191ec4)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x00191ed0);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00191ed0)?;
                            c.g[A0][0] = 0x460000;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00191ee0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d80));
                            call_ptr(c, 0x00317670, 0x00191ee0)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x00191eec);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00191eec)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[A0][0] = 0x460000;
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S4][0] as u32));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e0));
                            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x51));
                            c.g[RA][0] = sx(0x00191f08);
                            c.g[S5][0] = ((c.g[V1][0] as i64) < 1i64) as u64;
                            call_ptr(c, 0x00317670, 0x00191f08)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x00191f14);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00191f14)?;
                            let t = c.g[S5][0] == 0u64;
                            c.g[S1][0] = c.g[V0][0];
                            if t {
                                break 'b_191f44;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S1][0] == 0u64;
                                c.g[V0][0] = 0x440000;
                                if t {
                                    break 'b_191f44;
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] = sx((c.g[S4][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1058));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[RA][0] = sx(0x00191f38);
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    call_ptr(c, 0x00317670, 0x00191f38)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00191f44);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x00191f44)?;
                                    break 'b_191f44;
                                }
                            }
                        }
                        'b_192008: {
                            'b_191fac: {
                                c.cycles += 2;
                                let t = c.g[S4][0] == 0u64;

                                if t {
                                    break 'b_191fac;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S5][0] == 0u64;

                                    if t {
                                        break 'b_192008;
                                    } else {
                                        'b_191f70: {
                                            c.cycles += 2;
                                            let t = c.g[S0][0] == 0u64;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_191f70;
                                            } else {
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00191f70);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00191f70)?;
                                                break 'b_191f70;
                                            }
                                        }
                                        'b_191f8c: {
                                            c.cycles += 2;
                                            let t = c.g[S2][0] == 0u64;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_191f8c;
                                            } else {
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00191f8c);
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00191f8c)?;
                                                break 'b_191f8c;
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[V0][0] = 0x460000;
                                        c.g[S6][0] = 0x1;
                                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x528));
                                        c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x528));
                                        c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x8));
                                        c.sd(c.g[SP][0] as u32, c.g[A0][0]);
                                        let t = true;
                                        c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                        if t {
                                            break 'b_192008;
                                        } else {
                                            break 'b_191fac;
                                        }
                                    }
                                }
                            }
                            'b_191fc8: {
                                c.cycles += 2;
                                let t = c.g[S0][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_191fc8;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00191fc8);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00191fc8)?;
                                    break 'b_191fc8;
                                }
                            }
                            'b_191fe4: {
                                c.cycles += 2;
                                let t = c.g[S2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_191fe4;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00191fe4);
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00191fe4)?;
                                    break 'b_191fe4;
                                }
                            }
                            c.cycles += 9;
                            c.g[V0][0] = 0x4a0000;
                            c.g[S6][0] = 0x1;
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1708));
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1708));
                            c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x6));
                            c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                            c.sw(c.g[SP][0] as u32, c.g[A0][0]);
                            c.sh((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x6), c.g[A1][0]);
                            break 'b_192008;
                        }
                        c.cycles += 2;
                        let t = c.g[S6][0] == 0u64;
                        c.g[V0][0] = c.g[S5][0];
                        if t {
                            break 'b_19205c;
                        } else {
                            'b_192040: {
                                c.cycles += 9;
                                c.g[RA][0] = sx(0x00192018);
                                c.g[A0][0] = c.g[SP][0];
                                call_ptr(c, 0x00317670, 0x00192018)?;
                                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x00192024);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039c778, 0x00192024)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffff;
                                let t = c.g[A1][0] == c.g[V0][0];
                                c.g[A1][0] = c.g[A1][0] & 0xffff;
                                if t {
                                    break 'b_192040;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x00192040);
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x0039c870, 0x00192040)?;
                                    break 'b_192040;
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = c.g[S5][0];
                            if t {
                                break 'b_19205c;
                            } else {
                                break 'b_192048;
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = sx((c.g[A3][0] as u32) << 24);
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x00192058);
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                Callee {
                    addr: 0x001913b8,
                    f: ee::s02::cFEStateRuleSelect_FillRuleList,
                }
                .call(c, 0x00192058)?;
                c.g[V0][0] = 0x101;
                break 'b_19205c;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            break 'b_192060;
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
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

/// `void cFEStateRuleSelect_ApplyRuleDependencies(self, widget)` (0x00192380): after a rule list box changed, resets (or decrements the counter byte at `+0x52+i` of) every dependent rule `i < 15` selected by the dependency mask of the list box value, then refreshes the row visibility.
pub fn cfe_state_rule_select_apply_rule_dependencies(c: &mut Cpu) {
    guest(c, |c| {
        'b_19245c: {
            'b_192418: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.g[A2][0] = 0x440000;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10c8));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[RA][0] = sx(0x001923c4);
                c.g[S3][0] = sx((c.g[V1][0] as u32) << 1);
                call_ptr(c, 0x0039a738, 0x001923c4)?;
                let t = c.g[V0][0] != 0u64;
                c.g[S1][0] = 0x1;
                if t {
                    break 'b_192418;
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0x1;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x52));
                    'l_1923d8: loop {
                        'b_192404: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A2][0] as u32) << ((c.g[A1][0] as u32) & 31));
                            c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                            if c.g[V0][0] != 0u64 {
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                break 'b_192404;
                            } else {
                                'b_192400: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
                                    let t = (c.g[V1][0] as i64) > 0;
                                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                    if t {
                                        break 'b_192400;
                                    } else {
                                        c.cycles += 1;
                                        c.sb(c.g[A0][0] as u32, 0u64);
                                        break 'b_192400;
                                    }
                                }
                                c.cycles += 1;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                break 'b_192404;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 15i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001923d8);
                            }
                            continue 'l_1923d8;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_19245c;
                            } else {
                                break 'b_192418;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[S4][0] = 0x1;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x52));
            'l_192420: loop {
                'b_192450: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << ((c.g[S1][0] as u32) & 31));
                    c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        break 'b_192450;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = 0u64;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.g[RA][0] = sx(0x0019244c);
                        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                        Callee {
                            addr: 0x00192088,
                            f: ee::s02::cFEStateRuleSelect_SetRuleValue,
                        }
                        .call(c, 0x0019244c)?;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        break 'b_192450;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 15i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00192420);
                    }
                    continue 'l_192420;
                } else {
                    break 'b_19245c;
                }
            }
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x00192464);
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x00192488,
            f: ee::s02::cFEStateRuleSelect_UpdateRowVisibility,
        }
        .call(c, 0x00192464)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
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

/// `void cFEStateBraggingRights_UpdateRowColors(self)` (0x001927b0): colours the 8 player-row widgets (`+8..`): the selected row (index at `obj+0x95`) white, enabled rows by the list box (`UIListBox_GetSelectedValue`) one of two colour pairs, via the widget vtable colour method (+0x5c).
pub fn cfe_state_bragging_rights_update_row_colors(c: &mut Cpu) {
    guest(c, |c| {
        'b_1928e0: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x88));
            c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x78));
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
            let t = c.g[V0][0] == 0u64;
            c.g[S1][0] = 0u64;
            if t {
                break 'b_1928e0;
            } else {
                c.cycles += 10;
                c.g[S3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x95));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                c.lwc1(25, (c.g[GP][0] as u32).wrapping_sub(0x59cc));
                c.lwc1(24, (c.g[GP][0] as u32).wrapping_sub(0x59c8));
                c.lwc1(23, (c.g[GP][0] as u32).wrapping_sub(0x59c4));
                c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x59c0));
                c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x59bc));
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                'l_192818: loop {
                    'b_1928d4: {
                        'b_1928d0: {
                            'b_1928cc: {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1928cc;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00192828);
                                    c.g[A1][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x00192948,
                                        f: ee::s02::cFEStateBraggingRights_IsRowEnabled,
                                    }
                                    .call(c, 0x00192828)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                        break 'b_1928d0;
                                    } else {
                                        'b_1928a0: {
                                            'b_19288c: {
                                                'b_19285c: {
                                                    c.cycles += 2;
                                                    let t = c.g[S1][0] != c.g[S3][0];
                                                    c.g[A1][0] = c.g[SP][0];
                                                    if t {
                                                        break 'b_19285c;
                                                    } else {
                                                        c.cycles += 9;
                                                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x58));
                                                        c.swc1(20, c.g[SP][0] as u32);
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0x4),
                                                        );
                                                        c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0x8),
                                                        );
                                                        let t = true;
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0xc),
                                                        );
                                                        if t {
                                                            break 'b_19288c;
                                                        } else {
                                                            break 'b_19285c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x00192864);
                                                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                                call_ptr(c, 0x0039a738, 0x00192864)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = c.g[SP][0];
                                                if t {
                                                    break 'b_1928a0;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                                    c.g[V1][0] =
                                                        c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                    c.swc1(20, c.g[SP][0] as u32);
                                                    c.swc1(
                                                        25,
                                                        (c.g[SP][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.swc1(
                                                        24,
                                                        (c.g[SP][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0xc),
                                                        0u64,
                                                    );
                                                    break 'b_19288c;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x00192898);
                                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                            call_ptr(c, t, 0x00192898)?;
                                            let t = true;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                            if t {
                                                break 'b_1928d4;
                                            } else {
                                                break 'b_1928a0;
                                            }
                                        }
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                        c.swc1(20, c.g[SP][0] as u32);
                                        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.g[A0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001928cc);
                                        c.g[A1][0] = c.g[SP][0];
                                        call_ptr(c, t, 0x001928cc)?;
                                        break 'b_1928cc;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            break 'b_1928d0;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        break 'b_1928d4;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x00192818);
                        }
                        continue 'l_192818;
                    } else {
                        break 'b_1928e0;
                    }
                }
            }
        }
        c.cycles += 13;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x88));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x78));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRuleSelect_SetRuleValue(self, int rule, int value)` (0x00192088): stores
/// `value & 1` as the bit of rule `rule` (jump table at 0x45fb50, 14 cases, in
/// the rule words at 0x5308b8/0x5308d0; one case stores the whole word at
/// 0x5308d4). Out-of-range rules do nothing. No calls.
pub fn cfe_state_rule_select_set_rule_value(c: &mut Cpu) {
    if c.pc != 0x00192088 {
        return cfe_state_rule_select_set_rule_value_resume(c);
    }
    'b_192238: {
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x460000;
        if t {
            break 'b_192238;
        } else {
            'b_1920e8: {
                'b_19222c: {
                    'b_19217c: {
                        'b_192160: {
                            'b_192144: {
                                'b_19212c: {
                                    'b_192114: {
                                        'b_192214: {
                                            'b_1921f8: {
                                                'b_1921dc: {
                                                    'b_1921c0: {
                                                        'b_1921a4: {
                                                            c.cycles += 6;
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x4b0));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                            let t = c.g[A0][0] as u32;
                                                            match t {
                                                                0x001920b0 => {
                                                                    c.cycles += 9;
                                                                    c.g[V1][0] = 0x530000;
                                                                    c.g[A1][0] = 0xfffffffffffffffe;
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x8b8));
                                                                    c.g[A0][0] = c.g[A2][0] & 0x1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    c.g[V0][0] =
                                                                        c.g[V0][0] & c.g[A1][0];
                                                                    c.g[V0][0] =
                                                                        c.g[V0][0] | c.g[A0][0];
                                                                    let t = c.g[RA][0] as u32;
                                                                    c.sw(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x18),
                                                                        c.g[V0][0],
                                                                    );
                                                                    c.pc = t;
                                                                    return;
                                                                }
                                                                0x001920d4 => {
                                                                    c.cycles += 5;
                                                                    c.g[A1][0] = 0x530000;
                                                                    c.g[V1][0] = c.g[A2][0] & 0x1;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A1][0] as u32)
                                                                            .wrapping_add(0x8b8));
                                                                    c.g[A0][0] = 0xfffffffffffffff7;
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        << 3);
                                                                    break 'b_1920e8;
                                                                }
                                                                0x001920f8 => {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] = 0x530000;
                                                                    c.g[V1][0] = c.g[A2][0] & 0x1;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x8b8));
                                                                    c.g[A1][0] = 0xfffffffffffffff7;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    let t = true;
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        << 3);
                                                                    if t {
                                                                        break 'b_19222c;
                                                                    } else {
                                                                        break 'b_192114;
                                                                    }
                                                                }
                                                                0x00192114 => {
                                                                    break 'b_192114;
                                                                }
                                                                0x0019212c => {
                                                                    break 'b_19212c;
                                                                }
                                                                0x00192144 => {
                                                                    break 'b_192144;
                                                                }
                                                                0x00192160 => {
                                                                    break 'b_192160;
                                                                }
                                                                0x0019217c => {
                                                                    break 'b_19217c;
                                                                }
                                                                0x00192188 => {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] = 0x530000;
                                                                    c.g[V1][0] = c.g[A2][0] & 0x1;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x8b8));
                                                                    c.g[A1][0] = 0xffffffffffffffef;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    let t = true;
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        << 4);
                                                                    if t {
                                                                        break 'b_19222c;
                                                                    } else {
                                                                        break 'b_1921a4;
                                                                    }
                                                                }
                                                                0x001921a4 => {
                                                                    break 'b_1921a4;
                                                                }
                                                                0x001921c0 => {
                                                                    break 'b_1921c0;
                                                                }
                                                                0x001921dc => {
                                                                    break 'b_1921dc;
                                                                }
                                                                0x001921f8 => {
                                                                    break 'b_1921f8;
                                                                }
                                                                0x00192214 => {
                                                                    break 'b_192214;
                                                                }
                                                                _ => {
                                                                    c.jump_out(
                                                                        t, 0x00192088, 0x00192240,
                                                                    );
                                                                    return;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 7;
                                                        c.g[A0][0] = 0x530000;
                                                        c.g[V1][0] = c.g[A2][0] & 0x1;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x8b8));
                                                        c.g[A1][0] = 0xffffffffffffffdf;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        let t = true;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                                        if t {
                                                            break 'b_19222c;
                                                        } else {
                                                            break 'b_1921c0;
                                                        }
                                                    }
                                                    c.cycles += 7;
                                                    c.g[A0][0] = 0x530000;
                                                    c.g[V1][0] = c.g[A2][0] & 0x1;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                                                    c.g[A1][0] = 0xffffffffffffffbf;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                                    let t = true;
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                                                    if t {
                                                        break 'b_19222c;
                                                    } else {
                                                        break 'b_1921dc;
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.g[A0][0] = 0x530000;
                                                c.g[V1][0] = c.g[A2][0] & 0x1;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                                                c.g[A1][0] = 0xffffffffffffff7f;
                                                c.g[V0][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                                let t = true;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
                                                if t {
                                                    break 'b_19222c;
                                                } else {
                                                    break 'b_1921f8;
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[A0][0] = 0x530000;
                                            c.g[V1][0] = c.g[A2][0] & 0x1;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                                            c.g[A1][0] = 0xfffffffffffffeff;
                                            c.g[V0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                            let t = true;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                                            if t {
                                                break 'b_19222c;
                                            } else {
                                                break 'b_192214;
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A0][0] = 0x530000;
                                        c.g[V1][0] = c.g[A2][0] & 0x1;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                                        c.g[A1][0] = 0xfffffffffffffdff;
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 9);
                                        break 'b_19222c;
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x530000;
                                    c.g[V1][0] = c.g[A2][0] & 0x1;
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8b8));
                                    c.g[A0][0] = 0xffffffffffffffdf;
                                    let t = true;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                    if t {
                                        break 'b_1920e8;
                                    } else {
                                        break 'b_19212c;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A1][0] = 0x530000;
                                c.g[V1][0] = c.g[A2][0] & 0x1;
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8b8));
                                c.g[A0][0] = 0xffffffffffffffef;
                                let t = true;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                if t {
                                    break 'b_1920e8;
                                } else {
                                    break 'b_192144;
                                }
                            }
                            c.cycles += 7;
                            c.g[A0][0] = 0x530000;
                            c.g[V1][0] = c.g[A2][0] & 0x1;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                            c.g[A1][0] = 0xfffffffffffffffd;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                            let t = true;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                            if t {
                                break 'b_19222c;
                            } else {
                                break 'b_192160;
                            }
                        }
                        c.cycles += 7;
                        c.g[A0][0] = 0x530000;
                        c.g[V1][0] = c.g[A2][0] & 0x1;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                        c.g[A1][0] = 0xfffffffffffffffb;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                        let t = true;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        if t {
                            break 'b_19222c;
                        } else {
                            break 'b_19217c;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x530000;
                    let t = c.g[RA][0] as u32;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8d4), c.g[A2][0]);
                    c.pc = t;
                    return;
                }
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_192238;
            }
            c.cycles += 4;
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            let t = c.g[RA][0] as u32;
            c.sw((c.g[A1][0] as u32).wrapping_add(0x8b8), c.g[V0][0]);
            c.pc = t;
            return;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
    return;
}

/// Native resume of `cfe_state_rule_select_set_rule_value` (converted from the `entries` body): any entry
/// other than 0x00192088, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn cfe_state_rule_select_set_rule_value_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00192088 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x00192238;
                        continue;
                    }
                    pc = 0x00192098;
                    continue;
                }
                0x00192098 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4b0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001920b0 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0x530000;
                    c.g[A1][0] = 0xfffffffffffffffe;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[A0][0] = c.g[A2][0] & 0x1;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    let t = c.g[RA][0] as u32;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.pc = t;
                    return Ok(());
                }
                0x001920d4 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8b8));
                    c.g[A0][0] = 0xfffffffffffffff7;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                    pc = 0x001920e8;
                    continue;
                }
                0x001920e8 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    let t = c.g[RA][0] as u32;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8b8), c.g[V0][0]);
                    c.pc = t;
                    return Ok(());
                }
                0x001920f8 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0xfffffffffffffff7;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = true;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                    if t {
                        pc = 0x0019222c;
                        continue;
                    }
                    pc = 0x00192114;
                    continue;
                }
                0x00192114 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8b8));
                    c.g[A0][0] = 0xffffffffffffffdf;
                    let t = true;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                    if t {
                        if back_edge_unwinds(c, 0x001920e8) {
                            return Ok(());
                        }
                        pc = 0x001920e8;
                        continue;
                    }
                    pc = 0x0019212c;
                    continue;
                }
                0x0019212c => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8b8));
                    c.g[A0][0] = 0xffffffffffffffef;
                    let t = true;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    if t {
                        if back_edge_unwinds(c, 0x001920e8) {
                            return Ok(());
                        }
                        pc = 0x001920e8;
                        continue;
                    }
                    pc = 0x00192144;
                    continue;
                }
                0x00192144 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0xfffffffffffffffd;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = true;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                    if t {
                        pc = 0x0019222c;
                        continue;
                    }
                    pc = 0x00192160;
                    continue;
                }
                0x00192160 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0xfffffffffffffffb;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = true;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    if t {
                        pc = 0x0019222c;
                        continue;
                    }
                    pc = 0x0019217c;
                    continue;
                }
                0x0019217c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x530000;
                    let t = c.g[RA][0] as u32;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8d4), c.g[A2][0]);
                    c.pc = t;
                    return Ok(());
                }
                0x00192188 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0xffffffffffffffef;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = true;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                    if t {
                        pc = 0x0019222c;
                        continue;
                    }
                    pc = 0x001921a4;
                    continue;
                }
                0x001921a4 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0xffffffffffffffdf;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = true;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                    if t {
                        pc = 0x0019222c;
                        continue;
                    }
                    pc = 0x001921c0;
                    continue;
                }
                0x001921c0 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0xffffffffffffffbf;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = true;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                    if t {
                        pc = 0x0019222c;
                        continue;
                    }
                    pc = 0x001921dc;
                    continue;
                }
                0x001921dc => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0xffffffffffffff7f;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = true;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
                    if t {
                        pc = 0x0019222c;
                        continue;
                    }
                    pc = 0x001921f8;
                    continue;
                }
                0x001921f8 => {
                    c.cycles += 7;
                    c.g[A0][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0xfffffffffffffeff;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = true;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                    if t {
                        pc = 0x0019222c;
                        continue;
                    }
                    pc = 0x00192214;
                    continue;
                }
                0x00192214 => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x530000;
                    c.g[V1][0] = c.g[A2][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                    c.g[A1][0] = 0xfffffffffffffdff;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 9);
                    pc = 0x0019222c;
                    continue;
                }
                0x0019222c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    pc = 0x00192238;
                    continue;
                }
                0x00192238 => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;

                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00192088, 0x00192240);
                    return Ok(());
                }
            }
        }
    });
}

pub(crate) const OC849_FIND_OBJECT: u32 = 0x0039_d860;

/// `int cFEStateBraggingRights_FindRowIndex(self, int value)` (0x192918): the
/// index (0..8) of the first row slot (`self + 8 + 4*i`) holding `value`, or -1.
pub fn c_fe_state_bragging_rights_find_row_index(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx(oc849_at(c, A0, 0x8));
    c.g[V1][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        if c.g[V0][0] == c.g[A1][0] {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            ret(c);
            return;
        }
        c.g[V1][0] = sx(oc849_at(c, V1, 0x1));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 8i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(oc849_at(c, A0, 0x4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0019_2920);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        ret(c);
        return;
    }
}

/// `void cFEStateBraggingRights_SetRowMask(self, int mask)` (0x192968): store the
/// enabled-row bit mask at `self + 0x2c`.
pub fn c_fe_state_bragging_rights_set_row_mask(c: &mut Cpu) {
    c.cycles += 2;
    let at = oc849_at(c, A0, 0x2c);
    c.sw(at, c.g[A1][0]);
    ret(c);
}

/// Look up the widget named by the string at `sp + name_off` in the screen (`s1`)
/// and, if it exists, call its vtable slot 0x48/0x4c (`setVisible(0)`).
/// `ret` is the return address of the string hash call; the lookup returns to
/// `ret + 0xc` and the method to `ret + 0x2c`. `charge` is the block cost not
/// already counted by the caller.
pub(crate) fn oc849_hide_named_widget(
    c: &mut Cpu,
    frame: Frame,
    name_off: u32,
    ret: u32,
    charge: u64,
) -> Flow {
    c.cycles += charge;
    c.g[A0][0] = sx(frame.at(c, name_off));
    call_ptr(c, OC849_STR_HASH, ret)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, OC849_FIND_OBJECT, ret + 0xc)?;
    c.g[A2][0] = c.g[V0][0];
    let missing = c.g[A2][0] == 0;
    c.g[A1][0] = 0;
    if !missing {
        c.cycles += 5;
        c.g[V0][0] = c.lw(oc849_at(c, A2, 0x8));
        c.g[A0][0] = c.lh(oc849_at(c, V0, 0x48));
        c.g[V1][0] = c.lw(oc849_at(c, V0, 0x4c));
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx(oc849_at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, ret + 0x2c)?;
    }
    Ok(())
}

/// `void cFEStateBraggingRights_HideUnusedRows(self, screen)` (0x192970): for each
/// of the 8 rows whose bit in the mask (`self + 0x2c`) is clear and that has an
/// item, make the item static and hidden, then hide the screen widgets of the
/// row by name (left/right arrows, arrows 1, row label, box, w1, w2). The names
/// are copied from the string table onto the stack, with the row digit patched in.
pub fn c_fe_state_bragging_rights_hide_unused_rows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 80;
        let frame = Frame::open(c, 0xd0);
        c.g[V0][0] = 0x46_0000;
        frame.save(c, S1, 0xb0);
        c.g[V1][0] = 0x46_0000;
        frame.save(c, S2, 0xa0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x90);
        c.g[S2][0] = 0;
        frame.save(c, S0, 0xc0);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x80);
        // Copy the row-name templates onto the stack (unaligned doubleword and
        // word copies, as the compiler emitted for the struct assignments).
        let a = |c: &Cpu, off: u32| frame.at(c, off);
        c.g[T1][0] = sx(oc849_at(c, V0, 0).wrapping_sub(0x378));
        c.ldl(6, oc849_at(c, T1, 0x7));
        c.ldr(6, oc849_at(c, T1, 0));
        c.g[A3][0] = c.lb(oc849_at(c, T1, 0x8));
        c.g[T0][0] = c.lb(oc849_at(c, T1, 0x9));
        let p = a(c, 0x7);
        c.sdl(6, p);
        let p = a(c, 0);
        c.sdr(6, p);
        let p = a(c, 0x8);
        c.sb(p, c.g[A3][0]);
        let p = a(c, 0x9);
        c.sb(p, c.g[T0][0]);
        c.g[A2][0] = c.lb(oc849_at(c, T1, 0xa));
        let p = a(c, 0xa);
        c.sb(p, c.g[A2][0]);
        c.g[T1][0] = sx(oc849_at(c, V1, 0).wrapping_sub(0x368));
        c.ldl(2, oc849_at(c, T1, 0x7));
        c.ldr(2, oc849_at(c, T1, 0));
        c.lwl(7, oc849_at(c, T1, 0xb));
        c.lwr(7, oc849_at(c, T1, 0x8));
        let p = a(c, 0x17);
        c.sdl(2, p);
        let p = a(c, 0x10);
        c.sdr(2, p);
        let p = a(c, 0x1b);
        c.swl(7, p);
        let p = a(c, 0x18);
        c.swr(7, p);
        c.g[A2][0] = 0x46_0000;
        c.g[T1][0] = sx(oc849_at(c, A2, 0).wrapping_sub(0x358));
        c.ldl(3, oc849_at(c, T1, 0x7));
        c.ldr(3, oc849_at(c, T1, 0));
        c.lwl(7, oc849_at(c, T1, 0xb));
        c.lwr(7, oc849_at(c, T1, 0x8));
        let p = a(c, 0x27);
        c.sdl(3, p);
        let p = a(c, 0x20);
        c.sdr(3, p);
        let p = a(c, 0x2b);
        c.swl(7, p);
        let p = a(c, 0x28);
        c.swr(7, p);
        c.g[V0][0] = 0x46_0000;
        c.g[T1][0] = sx(oc849_at(c, V0, 0).wrapping_sub(0x348));
        c.ldl(6, oc849_at(c, T1, 0x7));
        c.ldr(6, oc849_at(c, T1, 0));
        c.lwl(7, oc849_at(c, T1, 0xb));
        c.lwr(7, oc849_at(c, T1, 0x8));
        c.g[T0][0] = c.lb(oc849_at(c, T1, 0xc));
        let p = a(c, 0x37);
        c.sdl(6, p);
        let p = a(c, 0x30);
        c.sdr(6, p);
        let p = a(c, 0x3b);
        c.swl(7, p);
        let p = a(c, 0x38);
        c.swr(7, p);
        let p = a(c, 0x3c);
        c.sb(p, c.g[T0][0]);
        c.g[V1][0] = 0x4a_0000;
        c.g[T1][0] = sx(oc849_at(c, V1, 0x1740));
        c.lwl(2, oc849_at(c, T1, 0x3));
        c.lwr(2, oc849_at(c, T1, 0));
        let p = a(c, 0x43);
        c.swl(2, p);
        let p = a(c, 0x40);
        c.swr(2, p);
        c.g[A2][0] = 0x4a_0000;
        c.g[T1][0] = sx(oc849_at(c, A2, 0x1748));
        c.lwl(3, oc849_at(c, T1, 0x3));
        c.lwr(3, oc849_at(c, T1, 0));
        let p = a(c, 0x53);
        c.swl(3, p);
        let p = a(c, 0x50);
        c.swr(3, p);
        c.g[V0][0] = 0x4a_0000;
        c.g[V1][0] = 0x4a_0000;
        c.g[T1][0] = sx(oc849_at(c, V0, 0x14f8));
        c.g[A2][0] = c.lb(oc849_at(c, T1, 0));
        c.g[A3][0] = c.lb(oc849_at(c, T1, 0x1));
        let p = a(c, 0x60);
        c.sb(p, c.g[A2][0]);
        let p = a(c, 0x61);
        c.sb(p, c.g[A3][0]);
        c.g[A3][0] = sx(oc849_at(c, V1, 0x1750));
        c.lwl(2, oc849_at(c, A3, 0x3));
        c.lwr(2, oc849_at(c, A3, 0));
        c.g[A0][0] = c.lb(oc849_at(c, A3, 0x4));
        let p = a(c, 0x73);
        c.swl(2, p);
        let p = a(c, 0x70);
        c.swr(2, p);
        let p = a(c, 0x74);
        c.sb(p, c.g[A0][0]);
        // The row loop: s2 = row, s3 = self, s1 = screen.
        c.g[V0][0] = c.lw(oc849_at(c, S3, 0x2c));
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[S2][0] as u32) & 31)) as u32);
            c.g[V0][0] &= 0x1;
            if c.g[V0][0] != 0 {
                c.g[S2][0] = sx(oc849_at(c, S2, 0x1));
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[V0][0] = sx(oc849_at(c, S3, 0x8));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                let no_item = c.g[A2][0] == 0;
                c.g[A1][0] = 0x1;
                if !no_item {
                    c.cycles += 30;
                    // item->setStatic(1)
                    c.g[V0][0] = c.lw(oc849_at(c, A2, 0x8));
                    c.g[A0][0] = c.lh(oc849_at(c, V0, 0x40));
                    c.g[V1][0] = c.lw(oc849_at(c, V0, 0x44));
                    let method = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(oc849_at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0019_2aec)?;
                    // item->setVisible(0)
                    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = 0;
                    c.g[V0][0] = c.lw(oc849_at(c, A2, 0x8));
                    c.g[A0][0] = c.lh(oc849_at(c, V0, 0x48));
                    c.g[V1][0] = c.lw(oc849_at(c, V0, 0x4c));
                    let method = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(oc849_at(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0019_2b08)?;
                    // Patch the row digit into the eight name strings.
                    c.g[V1][0] = sx(oc849_at(c, S2, 0x31));
                    c.g[V0][0] = sx(oc849_at(c, S2, 0x30));
                    c.g[A0][0] = c.g[SP][0];
                    for off in [0x30u32, 0, 0x10, 0x20] {
                        let p = a(c, off);
                        c.sb(p, c.g[V0][0]);
                    }
                    for off in [0x52u32, 0x60, 0x73, 0x42] {
                        let p = a(c, off);
                        c.sb(p, c.g[V1][0]);
                    }
                    // Hide the widgets by name, in the original order.
                    oc849_hide_named_widget(c, frame, 0x00, 0x0019_2b38, 0)?;
                    oc849_hide_named_widget(c, frame, 0x10, 0x0019_2b6c, 8)?;
                    oc849_hide_named_widget(c, frame, 0x20, 0x0019_2ba0, 8)?;
                    oc849_hide_named_widget(c, frame, 0x30, 0x0019_2bd4, 8)?;
                    oc849_hide_named_widget(c, frame, 0x60, 0x0019_2c08, 8)?;
                    oc849_hide_named_widget(c, frame, 0x70, 0x0019_2c3c, 8)?;
                    oc849_hide_named_widget(c, frame, 0x40, 0x0019_2c70, 8)?;
                    oc849_hide_named_widget(c, frame, 0x50, 0x0019_2ca4, 8)?;
                }
                c.cycles += 1;
                c.g[S2][0] = sx(oc849_at(c, S2, 0x1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 8i64) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw(oc849_at(c, S3, 0x2c));
                if event_due(c) {
                    deliver_event_here(c, 0x0019_2ab0);
                }
                continue;
            }
            break;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::sound::soundsys::tests::oc773_install_fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::*;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn cfe_state_bragging_rights_is_row_enabled_matches() {
        check(
            ee::s02::f_00192948__generated,
            cfe_state_bragging_rights_is_row_enabled,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19_2948, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 48) as u64;
                c.bus.write32(DATA + 0x2c, r.next_u32());
            },
        );
    }

    #[test]
    fn c_fe_state_rule_select_v21_matches() {
        check(
            ssx3_ee::s02::f_00191e08__generated,
            c_fe_state_rule_select_v21,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x191e_08, 0);
                c.g[GP][0] = GAME_GP as u64;
                let msg = DATA + 0x800;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = msg as u64;
                c.bus.write32(msg + 0x38, r.next_u32());
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x1926f0 cFEStateBraggingRights_InitPlayerRows.

    #[test]
    fn c_fe_state_bragging_rights_init_player_rows_matches() {
        check(
            ee::s02::f_001926f0__generated,
            c_fe_state_bragging_rights_init_player_rows,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x1926_f0, 0);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn c_fe_state_bragging_rights_set_rows_visible_matches() {
        check(
            ee::s02::f_00192740__generated,
            c_fe_state_bragging_rights_set_rows_visible,
            60,
            &|c, seed| {
                machine(c, seed, 0x1927_40, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                for i in 0..8u32 {
                    let p = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        DATA + 0x1000 + i * 0x100
                    };
                    c.bus.write32(DATA + 8 + i * 4, p);
                    if p != 0 {
                        put_vtable(c, p, &[0x40]);
                    }
                }
            },
        );
    }

    #[test]
    fn cfestatebraggingrights_ctor_matches() {
        check(
            ee::s02::f_00192d00__generated,
            cfestatebraggingrights_ctor,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x2d00, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x191c68 cFEStateRuleSelect_OnWidgetAction.

    /// `UIListNode_IsSentinel` behind a vtable slot: returns 0 here.
    const RET_ZERO: u32 = 0x001a_8828;

    #[test]
    fn c_fe_state_rule_select_on_widget_action_matches() {
        use crate::difftest::elf_image;
        use crate::game::be::bedbreward::tests::alloc_ready;
        check(
            ee::s02::f_00191c68__generated,
            c_fe_state_rule_select_on_widget_action,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0019_1c68, 1);
                // Real tables (the rule-value jump table, the dependency
                // table) and a heap for the options state.
                elf_image(c);
                alloc_ready(c);
                let this = DATA;
                let widget = DATA + 0x1000;
                let base = DATA + 0x2000; // *(self+0x10): the flow object
                let owner = DATA + 0x3000;
                let vt = DATA + 0x3100;
                let popup = DATA + 0x3200;
                let state = DATA + 0x5000;
                for a in [this, widget, base, owner, vt, popup, state] {
                    for i in 0..0x100 {
                        c.bus.write32(a + i * 4, 0);
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 23 == 0 { 0 } else { sx(widget) };
                c.g[A2][0] = match seed % 6 {
                    0 => 5,
                    1 => 6,
                    2 => 7,
                    3 => 9,
                    4 => [0u64, 1, 4, 8, 10, 0xffff_ffff_ffff_fff9][(seed / 6 % 6) as usize],
                    _ => [5u64, 6][(seed / 6 % 2) as usize],
                };
                c.bus.write32(this + 0x10, base);
                // The state manager at base+0x18: an empty list.
                c.bus.write32(base, owner);
                c.bus.write32(base + 0x18, base + 0x18);
                c.bus.write32(base + 0x1c, base + 0x18);
                c.bus.write32(owner + 4, vt);
                let null_state = (seed / 3) % 3 == 0;
                for slot in [0x20u32, 0x28] {
                    c.bus.write16(vt + slot, (r.next_u32() & 0x3c) as u16);
                    c.bus
                        .write32(vt + slot + 4, if null_state { RET_ZERO } else { FAKE });
                }
                // The fake flow method returns a state to switch to.
                c.bus.write32(LOG_RESULTS, state);
                c.bus.write32(state + 0x1c, r.next_u32());
                // Accept: the widget's popup busy or not.
                c.bus.write32(widget + 0x7c, popup);
                c.bus
                    .write32(popup + 0x18, if seed % 4 == 1 { 1 } else { 0 });
                // Value changed: rule id and the list box selection.
                c.bus.write32(widget + 0x18, r.next_u32() % 16);
                let count = (r.next_u32() % 4) as u8;
                c.bus.write8(widget + 0x318, count);
                c.bus.write8(widget + 0x319, (r.next_u32() % 4) as u8);
                for i in 0..4 {
                    c.bus.write32(widget + 0xa4 + i * 0x14, r.next_u32() % 3);
                }
                // `UpdateRowVisibility`: no rows.
                c.bus.write8(this + 0x50, 0);
                for i in 0..16 {
                    c.bus.write8(this + 0x51 + i, (r.next_u32() % 3) as u8);
                }
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fill;
    use crate::difftest::put_words;

    pub(crate) use crate::game::sound::soundsys::tests::oc773_arm;
    pub(crate) use crate::game::sound::soundsys::tests::oc773_force_zero;

    pub(crate) use crate::game::ui::uimenu::tests::OC776_CTRL;
    pub(crate) use crate::game::ui::uimenu::tests::OC776_MAGIC;
    use std::sync::atomic::AtomicUsize;

    /// `cFEStateRuleSelect_GetRuleValue`: every rule 0..=16 (out of range, the
    /// 14 cases of the jump table) over random rule words, and a corrupt table
    /// entry (jump out).
    #[test]
    fn cfe_state_rule_select_get_rule_value_matches() {
        let table: [u32; 14] = [
            0x1922dc, 0x1922f0, 0x192278, 0x192268, 0x19228c, 0x1922fc, 0x1922b4, 0x1922c8,
            0x192310, 0x192324, 0x192338, 0x19234c, 0x192360, 0x1922a0,
        ];
        check(
            ee::s02::f_00192240__generated,
            cfe_state_rule_select_get_rule_value,
            340,
            &|c, seed| {
                let mut r = machine(c, seed, 0x754d, 0);
                c.pc = 0x0019_2240;
                put_words(c, 0x0045_fb90, &table);
                for off in [0x8b8u32, 0x8d0, 0x8d4] {
                    c.bus.write32(0x0053_0000 + off, r.next_u32());
                }
                let rule = seed % 17;
                c.g[A1][0] = sx(rule);
                if seed % 23 == 0 && (1..=14).contains(&rule) {
                    c.bus.write32(0x0045_fb90 + 4 * (rule - 1), 0x0019_2250);
                }
            },
        );
    }

    /// `cFEStateRuleSelect_Init`: every event type (the jump table: 6 cases,
    /// out-of-range types and a corrupt table entry), a missing screen or
    /// rule row, and the unwinds of its many calls.
    #[test]
    fn cfe_state_rule_select_init_matches() {
        oc773_install_fakes();
        let table: [u32; 6] = [
            0x0019_19e4,
            0x0019_19f4,
            0x0019_1a10,
            0x0019_1a04,
            0x0019_1a1c,
            0x0019_1a2c,
        ];
        fakes::covered(
            ee::s02::f_00191938__generated,
            cfe_state_rule_select_init,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x773e, 40);
                oc773_arm(c);
                c.pc = 0x0019_1938;
                let this = DATA + 0x10000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x10, DATA + 0x1000);
                put_words(c, 0x0045_fb30, &table);
                let event = seed % 8;
                c.bus.write8(0x0053_5c12, event as u8);
                if seed % 29 == 0 && event < 6 {
                    c.bus.write32(0x0045_fb30 + 4 * event, 0x0019_1900);
                }
                if seed % 5 == 1 {
                    oc773_force_zero(c, 0x0039_7cc0);
                }
                if seed % 7 == 2 {
                    oc773_force_zero(c, 0x0039_d860);
                }
                // Rule strings / selection bytes the row loops read.
                for i in 0..0x40 {
                    c.bus.write8(0x004a_16a8 + i, (r.next_u32() % 12) as u8);
                    c.bus.write8(0x004a_13a0 + i, (r.next_u32() % 12) as u8);
                }
            },
        );
    }

    pub(crate) const OC776_THIS: u32 = DATA + 0x1_0000;

    pub(crate) const OC776_ROWS: u32 = DATA + 0x2_0000;

    pub(crate) const OC776_VT: u32 = DATA + 0x4_0000;

    pub(crate) static OC776_PREV_HASH: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC776_PREV_FIND: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of `StrHash` (0x317670): the hash is the first two
    /// characters of the name (logged like any fake call).
    pub(crate) fn oc776_fake_hash(c: &mut Cpu) {
        if c.bus.read32(OC776_CTRL) != OC776_MAGIC {
            return fakes::fall_back(c, &OC776_PREV_HASH, ee::s10::f_00317670__generated);
        }
        let ra = c.g[RA][0] as u32;
        let name = c.g[A0][0] as u32;
        let hash = c.bus.read8(name) as u32 | (c.bus.read8(name + 1) as u32) << 8;
        fakes::fake_call(c, 0x0031_7670);
        if c.pc == ra {
            c.g[V0][0] = hash as u64;
        }
    }

    /// Gated fake of `UIScreen_FindObject` (0x39d860): the object of a row
    /// name hash ("0b".."9b", "ab", "bb" and the "t" twins), or null when the
    /// fixture's absence mask (`CTRL + 4`) has its bit.
    pub(crate) fn oc776_fake_find(c: &mut Cpu) {
        if c.bus.read32(OC776_CTRL) != OC776_MAGIC {
            return fakes::fall_back(c, &OC776_PREV_FIND, ee::s13::f_0039d860__generated);
        }
        let ra = c.g[RA][0] as u32;
        let hash = c.g[A1][0] as u32;
        let mask = c.bus.read32(OC776_CTRL + 4);
        let closes_popup = c.bus.read32(OC776_CTRL + 8) != 0 && hash & 0xff == b'1' as u32;
        fakes::fake_call(c, 0x0039_d860);
        if c.pc == ra {
            if closes_popup {
                c.bus.write32(OC776_THIS + POPUP_ACTIVE, 0);
            }
            let (c0, c1) = (hash & 0xff, (hash >> 8) & 0xff);
            let row = if c0 < 0x40 {
                c0.wrapping_sub(0x30)
            } else {
                c0.wrapping_sub(0x61) + 9
            };
            let n = row.wrapping_mul(2).wrapping_add((c1 == b't' as u32) as u32);
            c.g[V0][0] = if n >= 32 || mask >> n & 1 != 0 {
                0
            } else {
                (OC776_ROWS + n * 0x200) as u64
            };
        }
    }

    pub(crate) fn oc776_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc776_fake_hash,
            &OC776_PREV_HASH,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc776_fake_find,
            &OC776_PREV_FIND,
        );
    }

    /// `cFEStateRuleSelect_UpdateRowVisibility`: row counts -1..=100 (none,
    /// each length, capped at 11), missing list/text boxes, shown (flag > 0)
    /// and hidden rows, and the unwinds of its calls.
    #[test]
    fn cfe_state_rule_select_update_row_visibility_matches() {
        oc776_install_fakes();
        let counts: [i32; 10] = [-1, 0, 1, 2, 5, 9, 10, 11, 12, 100];
        fakes::covered(
            ee::s02::f_00192488__generated,
            cfe_state_rule_select_update_row_visibility,
            450,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7762, 40);
                c.bus.write32(OC776_CTRL, OC776_MAGIC);
                let mask = match seed % 3 {
                    0 => 0,
                    _ => r.next_u32() & r.next_u32(),
                };
                c.bus.write32(OC776_CTRL + 4, mask);
                c.g[A0][0] = sx(OC776_THIS);
                c.bus.write8(
                    OC776_THIS + RULE_ROW_COUNT,
                    counts[(seed % 10) as usize] as u8,
                );
                c.bus.write32(OC776_THIS + RULE_SCREEN, 0x1234);
                for (i, b) in b"0b\0".iter().enumerate() {
                    c.bus.write8(0x004a_16a8 + i as u32, *b);
                }
                for (i, b) in b"0t\0".iter().enumerate() {
                    c.bus.write8(0x004a_16b0 + i as u32, *b);
                }
                fill(c, &mut r, 0x004c_66c8, 4);
                for (i, slot) in [0xc8u32, 0xc0, 0x40].into_iter().enumerate() {
                    c.bus
                        .write16(OC776_VT + slot, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus
                        .write32(OC776_VT + slot + 4, fakes::FAKE + 0x10 * i as u32);
                }
                for n in 0..32u32 {
                    let obj = OC776_ROWS + n * 0x200;
                    c.bus.write32(obj + 8, OC776_VT);
                    let data_off = 0x100 + 0x10 * (n / 2);
                    c.bus.write32(obj + RULE_ROW_DATA_OFF, data_off);
                    c.bus.write8(
                        OC776_THIS + data_off + 0x51,
                        (r.next_u32() % 5).wrapping_sub(2) as u8,
                    );
                }
            },
        );
    }

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::oc800_machine;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::oc800_mask;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::OC800_SELF;

    pub(crate) use crate::game::fe::festatecharselect::tests::OC800_ENG;

    use crate::game::main::loadscreens::tests::oc736_force;

    pub(crate) const OC800_MAP: u32 = DATA + 0x2e_1000;

    /// `cFEStateRuleSelect_FillRuleList`: `first` of 0 to 3 rows into a list
    /// of 0 to 13 rules of every kind (0 to 8), widgets present or missing
    /// (the `FindObject` fake answers by a per-seed bit mask), the frame
    /// lookup failing (0xffff) or not.
    #[test]
    fn rule_select_fill_rule_list_matches() {
        check(
            ee::s02::f_001913b8__generated,
            rule_select_fill_rule_list,
            300,
            &|c, seed| {
                let mut r = oc800_machine(c, seed, 0x0019_13b8, 200);
                // The kind jump table (kinds 0 to 6).
                for (i, t) in [
                    0x0019_175cu32,
                    0x0019_1500,
                    0x0019_1528,
                    0x0019_156c,
                    0x0019_15e0,
                    0x0019_1688,
                    0x0019_1708,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(0x0045_faf0 + 4 * i as u32, t);
                }
                // D_00441008 (kind per rule) and D_00441018 (label id per rule).
                for i in 0..0x10u32 {
                    c.bus.write8(0x0044_1008 + i, (r.next_u32() % 9) as u8);
                    c.bus.write32(0x0044_1018 + 4 * i, r.next_u32());
                }
                c.bus.write32(OC800_SELF + 0x40, OC800_ENG);
                c.bus.write32(OC800_SELF + 0x48, OC800_MAP);
                c.bus.write8(OC800_SELF + 0x50, ((seed / 3) % 14) as u8);
                for i in 0..0x20u32 {
                    let rule = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32() % 12
                    };
                    c.bus.write8(OC800_MAP + i, rule as u8);
                }
                let mask = match seed % 3 {
                    0 => 0xffff_ffff,
                    _ => r.next_u32() | r.next_u32() | 0x8000_0000,
                };
                oc800_mask(c, 0x0039_d860, mask);
                if seed % 2 == 0 {
                    oc736_force(c, 0x0039_c778, 0xffff);
                }
                c.g[A0][0] = sx(OC800_SELF);
                c.g[A1][0] = sx((seed / 42) % 4);
                c.pc = 0x0019_13b8;
            },
        );
    }

    pub(crate) use crate::game::fe::festatebigradio::tests::oc849_base;
    pub(crate) use crate::game::fe::festatebigradio::tests::oc849_clear;
    pub(crate) use crate::game::fe::festatebigradio::tests::oc849_item_vtable;

    pub(crate) use crate::game::fe::festatebigradio::tests::OC849_ITEM;
    pub(crate) use crate::game::fe::festatebigradio::tests::OC849_SCREEN;
    pub(crate) use crate::game::fe::festatebigradio::tests::OC849_THIS;
    pub(crate) use crate::game::fe::festatebigradio::tests::OC849_VT;
    pub(crate) use crate::game::fe::festatebigradio::tests::OC849_WIDGET;

    use crate::game::main::loadscreens::tests::oc736_force_echo;
    use crate::game::main::loadscreens::tests::oc736_force_zero;

    /// The wrapper calls UIState_OnActivate through the table; a0 points at
    /// zeroed data (empty list), the callee runs for real.
    #[test]
    fn fe_state_rule_select_on_activate_matches() {
        check(
            ee::s02::f_00191c48__generated,
            fe_state_rule_select_on_activate,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x191c_48, 0);
                c.g[A0][0] = (DATA + 0x1000 + (r.next_u32() & 0xff0)) as u64;
                c.pc = 0x0019_1c48;
            },
        );
    }

    /// Gate word of this module's fakes (in the machine's own RAM).
    pub(crate) const OC847_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC847_MAGIC: u32 = 0x8470_f4ce;

    /// FindObject results, 2 bits per call (0: null, 1..3: widget object).
    pub(crate) const OC847_FIND_MASK: u32 = OC847_CTRL + 4;

    /// Number of FindObject calls so far.
    pub(crate) const OC847_FIND_N: u32 = OC847_CTRL + 8;

    /// Non-zero: UIScreen_LookupValue returns 0xffff.
    pub(crate) const OC847_LOOKUP_NONE: u32 = OC847_CTRL + 0xc;

    /// UIListBox_GetSelectedValue results, one bit per call.
    pub(crate) const OC847_SEL_MASK: u32 = OC847_CTRL + 0x10;

    pub(crate) const OC847_SEL_N: u32 = OC847_CTRL + 0x14;

    /// IsRowEnabled result bits by row.
    pub(crate) const OC847_ROW_MASK: u32 = OC847_CTRL + 0x18;

    /// Widget objects returned by the FindObject fake, and their vtable.
    pub(crate) const OC847_OBJS: u32 = DATA + 0x20_1000;

    pub(crate) const OC847_VT: u32 = DATA + 0x20_8000;

    pub(crate) fn oc847_post_none(_c: &mut Cpu, _a0: u32, _a1: u32) {}

    /// FindObject: null or one of three widget objects, per call by mask.
    pub(crate) fn oc847_post_find(c: &mut Cpu, _a0: u32, _a1: u32) {
        let n = c.bus.read32(OC847_FIND_N);
        c.bus.write32(OC847_FIND_N, n + 1);
        let sel = (c.bus.read32(OC847_FIND_MASK) >> (2 * (n % 16))) & 3;
        c.g[V0][0] = if sel == 0 {
            0
        } else {
            (OC847_OBJS + sel * 0x200) as u64
        };
    }

    pub(crate) fn oc847_post_lookup(c: &mut Cpu, _a0: u32, _a1: u32) {
        if c.bus.read32(OC847_LOOKUP_NONE) != 0 {
            c.g[V0][0] = 0xffff;
        }
    }

    pub(crate) fn oc847_post_selected(c: &mut Cpu, _a0: u32, _a1: u32) {
        let n = c.bus.read32(OC847_SEL_N);
        c.bus.write32(OC847_SEL_N, n + 1);
        c.g[V0][0] = ((c.bus.read32(OC847_SEL_MASK) >> (n % 32)) & 1) as u64;
    }

    pub(crate) fn oc847_post_row_enabled(c: &mut Cpu, _a0: u32, a1: u32) {
        c.g[V0][0] = ((c.bus.read32(OC847_ROW_MASK) >> (a1 % 32)) & 1) as u64;
    }

    macro_rules! oc847_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $post:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC847_CTRL) != OC847_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    let (a0, a1) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
                    fakes::fake_call(c, $addr);
                    if c.pc == ra {
                        $post(c, a0, a1);
                    }
                }
            )*
            pub(crate) fn oc847_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc847_fakes!(
        (
            oc847_fake_sprintf,
            OC847_PREV_SPRINTF,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828,
            oc847_post_none
        ),
        (
            oc847_fake_hash,
            OC847_PREV_HASH,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            oc847_post_none
        ),
        (
            oc847_fake_find,
            OC847_PREV_FIND,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            oc847_post_find
        ),
        (
            oc847_fake_set_text,
            OC847_PREV_SET_TEXT,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000,
            oc847_post_none
        ),
        (
            oc847_fake_selected,
            OC847_PREV_SELECTED,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738,
            oc847_post_selected
        ),
        (
            oc847_fake_lookup,
            OC847_PREV_LOOKUP,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778,
            oc847_post_lookup
        ),
        (
            oc847_fake_start,
            OC847_PREV_START,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870,
            oc847_post_none
        ),
        (
            oc847_fake_row_enabled,
            OC847_PREV_ROW_ENABLED,
            ee::s02::OVERRIDE_00192948,
            ee::s02::f_00192948__generated,
            0x0019_2948,
            oc847_post_row_enabled
        ),
        (
            oc847_fake_visibility,
            OC847_PREV_VISIBILITY,
            ee::s02::OVERRIDE_00192488,
            ee::s02::f_00192488__generated,
            0x0019_2488,
            oc847_post_none
        ),
        (
            oc847_fake_fill_list,
            OC847_PREV_FILL_LIST,
            ee::s02::OVERRIDE_001913B8,
            ee::s02::f_001913b8__generated,
            0x0019_13b8,
            oc847_post_none
        ),
    );

    /// Arms the gated fakes and sets this fixture's control words.
    pub(crate) fn oc847_arm(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        oc847_install_fakes();
        c.bus.write32(OC847_CTRL, OC847_MAGIC);
        // Mostly "found", some nulls; every 5th seed finds everything.
        let find = if seed % 5 == 0 {
            0x5555_5555 | r.next_u32() & 0xaaaa_aaaa
        } else {
            r.next_u32()
        };
        c.bus.write32(OC847_FIND_MASK, find);
        c.bus.write32(OC847_LOOKUP_NONE, seed & 1);
        c.bus.write32(OC847_SEL_MASK, r.next_u32());
        c.bus.write32(OC847_ROW_MASK, r.next_u32());
        // Widget objects: vtable with the colour/text method (+0x48 delta,
        // +0x4c function) and the row colour method (+0x58, +0x5c).
        for k in 1..4u32 {
            let obj = OC847_OBJS + k * 0x200;
            c.bus.write32(obj + 8, OC847_VT);
        }
        for off in [0x48u32, 0x58] {
            let delta = r.next_u32() as i16 as i32 & !3;
            c.bus.write16(OC847_VT + off, delta as u16);
            c.bus.write32(OC847_VT + off + 4, fakes::FAKE + 0x10 + off);
        }
    }

    /// `OnWidgetNotify`: the kinds 0..=10 (4 refills the list, 1 recolours the
    /// row widgets, 9 and the rest answer directly), rule rows with and
    /// without a map entry, null or present widgets, the frame lookup failing
    /// or not, and the unwinds of the calls.
    #[test]
    fn cfe_state_rule_select_on_widget_notify_matches() {
        check(
            ee::s02::f_00191e48__generated,
            cfe_state_rule_select_on_widget_notify,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8473, 14);
                oc847_arm(c, &mut r, seed);
                let this = DATA + 0x1000;
                let map = DATA + 0x2000;
                c.bus.write32(this + 0x40, 0x1234_5678);
                c.bus.write32(this + 0x48, map);
                for i in 0..8 {
                    // Rule byte (0: no widget work) and the row flag bytes.
                    let b = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write8(map + i, b as u8);
                    c.bus
                        .write8(this + 0x51 + i, (r.next_u32() % 4).wrapping_sub(1) as u8);
                }
                fill(c, &mut r, 0x0044_1058, 8);
                const KINDS: [u32; 8] = [1, 1, 1, 4, 0, 2, 9, 10];
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = KINDS[(seed % 8) as usize] as u64;
                c.g[A3][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    /// The rule jump table of `SetRuleValue` (0x45fb50): the 14 case targets in
    /// a per-seed order, and now and then a corrupt entry.
    pub(crate) fn oc847_rule_table(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        let mut t: [u32; 14] = [
            0x0019_20b0,
            0x0019_20d4,
            0x0019_20f8,
            0x0019_2114,
            0x0019_212c,
            0x0019_2144,
            0x0019_2160,
            0x0019_217c,
            0x0019_2188,
            0x0019_21a4,
            0x0019_21c0,
            0x0019_21dc,
            0x0019_21f8,
            0x0019_2214,
        ];
        for i in (1..14).rev() {
            t.swap(i, (r.next_u32() % (i as u32 + 1)) as usize);
        }
        if seed % 31 == 7 {
            t[(seed / 31) as usize % 14] = 0x0019_2100;
        }
        crate::difftest::put_words(c, 0x0045_fb50, &t);
        for off in [0x8b8u32, 0x8d0, 0x8d4] {
            c.bus.write32(0x0053_0000 + off, r.next_u32());
        }
    }

    /// `SetRuleValue`: every rule (0..=16: out of range too), every jump
    /// table target, both values.
    #[test]
    fn cfe_state_rule_select_set_rule_value_matches() {
        check(
            ee::s02::f_00192088__generated,
            cfe_state_rule_select_set_rule_value,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8474, 0);
                c.pc = 0x0019_2088;
                oc847_rule_table(c, &mut r, seed);
                c.g[A1][0] = (seed % 17) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    /// `ApplyRuleDependencies`: every dependency mask (15 rule bits), the
    /// list box value set (resets the dependent rules through the real
    /// `SetRuleValue`) or not (decrements their counters).
    #[test]
    fn cfe_state_rule_select_apply_rule_dependencies_matches() {
        check(
            ee::s02::f_00192380__generated,
            cfe_state_rule_select_apply_rule_dependencies,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8475, 2);
                oc847_arm(c, &mut r, seed);
                oc847_rule_table(c, &mut r, 1);
                let this = DATA + 0x1000;
                let widget = DATA + 0x3000;
                let rule = r.next_u32() % 8;
                c.bus.write32(widget + 0x18, rule);
                for i in 0..8 {
                    let w = if i == rule && seed % 3 == 0 {
                        0
                    } else {
                        r.next_u32() & 0x7fff
                    };
                    c.bus.write32(0x0044_10c8 + 4 * i, w);
                }
                for i in 0..16 {
                    c.bus.write8(this + 0x52 + i, (r.next_u32() % 3) as u8);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
            },
        );
    }

    /// `UpdateRowColors`: eight rows with missing or present widgets, the
    /// selected row, enabled or disabled rows and the list box answers.
    #[test]
    fn cfe_state_bragging_rights_update_row_colors_matches() {
        check(
            ee::s02::f_001927b0__generated,
            cfe_state_bragging_rights_update_row_colors,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8476, 24);
                oc847_arm(c, &mut r, seed);
                let this = DATA + 0x1000;
                let owner = DATA + 0x2000;
                c.bus.write32(this, if seed % 17 == 3 { 0 } else { owner });
                c.bus.write8(owner + 0x95, (r.next_u32() % 9) as u8);
                for i in 0..8u32 {
                    let w = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        OC847_OBJS + (1 + i % 3) * 0x200
                    };
                    c.bus.write32(this + 8 + 4 * i, w);
                }
                for off in [0x59ccu32, 0x59c8, 0x59c4, 0x59c0, 0x59bc] {
                    c.bus.write32(GAME_GP - off, r.edge_f32());
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn find_row_index_matches() {
        check(
            ee::s02::f_00192918__generated,
            c_fe_state_bragging_rights_find_row_index,
            64,
            &|c, seed| {
                fakes::machine(c, seed, 0x19_2918, 0);
                let this = DATA + 0x100;
                for i in 0..8u32 {
                    c.bus.write32(this + 8 + 4 * i, (i * 3 + seed) % 12);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = ((seed / 2) % 13) as u64;
                c.pc = 0x0019_2918;
            },
        );
    }

    #[test]
    fn set_row_mask_matches() {
        check(
            ee::s02::f_00192968__generated,
            c_fe_state_bragging_rights_set_row_mask,
            16,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x19_2968, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = r.next_u32() as u64;
                c.pc = 0x0019_2968;
            },
        );
    }

    /// Mask bits set / clear, rows with and without an item, and the name
    /// lookups finding the widget or not.
    #[test]
    fn hide_unused_rows_matches() {
        check(
            ee::s02::f_00192970__generated,
            c_fe_state_bragging_rights_hide_unused_rows,
            120,
            &|c, seed| {
                let mut r = oc849_base(c, seed, 0x19_2970, 40);
                oc849_clear(c, OC849_THIS, 0x100);
                c.bus.write32(OC849_THIS + 0x2c, r.next_u32() & 0xff);
                for i in 0..8u32 {
                    let item = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        OC849_ITEM + i * 0x100
                    };
                    c.bus.write32(OC849_THIS + 8 + 4 * i, item);
                    if item != 0 {
                        c.bus.write32(item + 8, OC849_VT);
                    }
                }
                oc849_item_vtable(c, seed);
                c.bus.write32(OC849_WIDGET + 8, OC849_VT);
                oc736_force_echo(c, 0x0031_7670);
                if seed % 3 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC849_WIDGET);
                }
                c.g[A0][0] = sx(OC849_THIS);
                c.g[A1][0] = sx(OC849_SCREEN);
                c.pc = 0x0019_2970;
            },
        );
    }
}
