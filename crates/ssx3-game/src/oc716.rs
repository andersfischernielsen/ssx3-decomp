//! Staging module oc716 (wave wave240): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc716`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `UIState_Render()`, the one call of `cFEStateCharSelect_Render`.
const CHAR_SELECT_RENDER_CALLEE: u32 = 0x0039_e6b8;
/// The return address after that call (the generated `RA` value).
const CHAR_SELECT_RENDER_RET: u32 = 0x0018_1410;

/// `EventSelect_OnWidgetCreated(this + 0x48)`, the one call of
/// `cFEStateEventSelect_OnWidgetCreated`.
const EVENT_SELECT_WIDGET_CREATED_CALLEE: u32 = 0x0020_09d0;
/// The return address after that call (the generated `RA` value).
const EVENT_SELECT_WIDGET_CREATED_RET: u32 = 0x0018_6718;
/// Offset of the event-select state inside the state object the widget
/// belongs to; the callee gets `this + STATE_OFFSET`.
const EVENT_SELECT_STATE_OFFSET: u32 = 0x48;

/// `void cFEStateCharSelect_Render(void)` (0x181400): renders the UI state,
/// `UIState_Render()`. A frame of 0x10 holding `ra`, which is what gcc
/// emits for a forwarding wrapper.
pub fn cfe_state_char_select_render(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Table(CHAR_SELECT_RENDER_CALLEE),
        CHAR_SELECT_RENDER_RET,
    )
}

/// `void cFEStateEventSelect_OnWidgetCreated(void* self)` (0x186708): hands
/// the event-select state (`self + 0x48`) to `EventSelect_OnWidgetCreated`.
/// The argument is an add, not a load, so this is not a plain wrapper.
pub fn cfe_state_event_select_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(EVENT_SELECT_STATE_OFFSET));
        call_ptr(
            c,
            EVENT_SELECT_WIDGET_CREATED_CALLEE,
            EVENT_SELECT_WIDGET_CREATED_RET,
        )?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_1400,
        cfe_state_char_select_render,
        "cFEStateCharSelect_Render",
    ),
    (
        0x0018_6708,
        cfe_state_event_select_on_widget_created,
        "cFEStateEventSelect_OnWidgetCreated",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 200;

    /// Gates of the two callee fakes: a magic word in the machine's RAM turns
    /// the fake on (both sides of a diff get their own machine).
    const CTRL_MAGIC: u32 = 0x0716_0716;
    const UI_CTRL: u32 = DATA + 0x20_0000;
    const EVT_CTRL: u32 = DATA + 0x20_0010;
    static OC716_PREV_UI_RENDER: AtomicUsize = AtomicUsize::new(0);
    static OC716_PREV_EVENT_SELECT: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of `UIState_Render` (logs the call, returns to `ra`).
    fn oc716_fake_ui_render(c: &mut Cpu) {
        if c.bus.read32(UI_CTRL) != CTRL_MAGIC {
            return fakes::fall_back(c, &OC716_PREV_UI_RENDER, ee::s13::f_0039e6b8);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, CHAR_SELECT_RENDER_CALLEE, ra);
    }

    /// Gated fake of `EventSelect_OnWidgetCreated`.
    fn oc716_fake_event_select(c: &mut Cpu) {
        if c.bus.read32(EVT_CTRL) != CTRL_MAGIC {
            return fakes::fall_back(c, &OC716_PREV_EVENT_SELECT, ee::s05::f_002009d0);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, EVENT_SELECT_WIDGET_CREATED_CALLEE, ra);
    }

    fn oc716_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E6B8),
            oc716_fake_ui_render,
            &OC716_PREV_UI_RENDER,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_002009D0),
            oc716_fake_event_select,
            &OC716_PREV_EVENT_SELECT,
        );
    }

    #[test]
    fn cfe_state_char_select_render_matches() {
        oc716_hook();
        check(
            ee::s02::f_00181400__generated,
            cfe_state_char_select_render,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 716, 1);
                c.pc = 0x0018_1400;
                c.bus.write32(UI_CTRL, CTRL_MAGIC);
            },
        );
    }

    #[test]
    fn cfe_state_event_select_on_widget_created_matches() {
        oc716_hook();
        check(
            ee::s02::f_00186708__generated,
            cfe_state_event_select_on_widget_created,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 717, 1);
                c.g[A0][0] = fakes::ptr(&mut r) as u64;
                c.pc = 0x0018_6708;
                c.bus.write32(EVT_CTRL, CTRL_MAGIC);
            },
        );
    }
}
