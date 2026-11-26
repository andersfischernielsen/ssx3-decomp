//! `src/fe/festateruleselect.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
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
        return ee::s02::f_00192240__entries(c);
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
        return ee::s02::f_00191938__entries(c);
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
}
