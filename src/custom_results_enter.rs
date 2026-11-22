//! Native transition from the finish presentation to the results UI.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use std::sync::Mutex;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0023_6da0, enter, "GameStateResults_Enter"),
    (0x0020_a8f8, open_screen_for_state, "GameUI_OpenScreenForState_ResultsPending"),
];

const CURRENT_RACE: u32 = 0x0053_5bc8;
const RIDER_COUNT: u32 = 0x30;
const SINGLE_EVENT_FLAG: u32 = 0x49;
const SCRIPT_ENGINE_GP: u32 = 0x84c;
const FINISHED: u32 = 8;

struct PendingScreen {
    course: &'static str,
    module: u32,
    state: u32,
}

static PENDING_SCREEN: Mutex<Option<PendingScreen>> = Mutex::new(None);

pub fn restart() {
    *PENDING_SCREEN.lock().unwrap() = None;
}

fn valid(at: u32, bytes: u32) -> bool {
    at >= 0x10000 && at.checked_add(bytes).is_some_and(|end| end <= 0x0200_0000)
}

/// The exact pre-call gate in GameUI_OpenScreenForState20a8f8. A rejected
/// request creates no UI state and leaves no native pending request behind.
fn module_gate(c: &mut Cpu) -> Option<(u32, bool)> {
    let app = c.bus.read32(r(c, GP).wrapping_sub(0x848));
    if !valid(app, 0x88) { return None }
    let module = c.bus.read32(app + 0x84);
    if !valid(module, 0x2c) { return None }
    let gate = c.bus.read32(module + 0x28);
    if !valid(gate, 4) { return None }
    let state = c.bus.read32(gate) as i32;
    Some((module, state != 0 && state < 10))
}

fn remember_rejected_screen(c: &mut Cpu, state: u32) {
    let pending = crate::custom_tracks::active().and_then(|track| {
        let (module, gated) = module_gate(c)?;
        gated.then_some(PendingScreen { course: track.id, module, state })
    });
    let rejected = pending.is_some();
    *PENDING_SCREEN.lock().unwrap() = pending;
    if rejected {
        crate::custom_finish_trace::observe(c, "results-screen-gated", 0, state);
    }
}

/// Observe the actual screen-request entry, including calls reached through
/// a generated continuation of Results_Enter after another callee yielded.
/// The unchanged native body still owns screen selection and construction.
fn open_screen_for_state(c: &mut Cpu) {
    if r(c, A0) == 7 && r(c, RA) == 0x0023_6e40 {
        remember_rejected_screen(c, r(c, S3));
    }
    ssx3_ee::s05::f_0020a8f8__generated(c);
}

/// Results_Update consumes only a request rejected before screen creation.
/// Clear it before calling UI code so a suspended constructor cannot cause
/// another results dialog on the next update. No result-ready flag changes.
pub fn retry_pending_screen(c: &mut Cpu, state: u32) -> bool {
    let Some(track) = crate::custom_tracks::active() else { return false };
    let mut pending = PENDING_SCREEN.lock().unwrap();
    let Some(request) = pending.as_ref() else { return false };
    if request.course != track.id || request.state != state {
        *pending = None;
        return false;
    }
    let Some((module, gated)) = module_gate(c) else { return false };
    if request.module != module || c.bus.read32(state + FINISHED) != 0 {
        *pending = None;
        return false;
    }
    if gated { return false }
    *pending = None;
    true
}

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn script_engine(c: &mut Cpu) -> u64 {
    c.lw(r(c, GP).wrapping_sub(SCRIPT_ENGINE_GP))
}

/// `void GameStateResults_Enter(GameStateResults* self)` (0x236da0).
/// Clear the player's first-time-WC flag outside the flagged event flow,
/// stop presentation list zero and open the native results screen (state 7).
/// This does not clear the race-result snapshot or its time/place fields.
pub fn enter(c: &mut Cpu) {
    restart();
    crate::custom_finish_trace::observe(c, "results-enter", 0, r(c, A0));
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_6dc0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_6dcc)?;

        c.g[V1][0] = 0x0053_0000;
        c.g[S1][0] = sx(CURRENT_RACE);
        c.g[V0][0] = c.lb(r(c, S1).wrapping_add(SINGLE_EVENT_FLAG));
        let flagged_event = c.g[V0][0] != 0;
        c.g[A0][0] = script_engine(c);
        if !flagged_event && crate::custom_tracks::active().is_none() {
            c.cycles += 9;
            c.g[S0][0] = 0;
            call_ptr(c, 0x0014_dc80, 0x0023_6de8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0014_dd58, 0x0023_6df4)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(r(c, S1).wrapping_add(RIDER_COUNT));
            let no_riders = (c.g[V0][0] as i64) <= 0;
            c.g[A0][0] = script_engine(c);
            if !no_riders {
                c.cycles += 1;
                c.g[A1][0] = c.g[S0][0];
                loop {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = 0;
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                    call_ptr(c, 0x0014_5cb0, 0x0023_6e18)?;
                    c.g[V0][0] = c.lw(r(c, S1).wrapping_add(RIDER_COUNT));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[S0][0];
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = script_engine(c);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_6e08);
                    }
                }
            }
        }

        c.cycles += 12;
        c.g[A1][0] = 0;
        call_ptr(c, 0x0027_90a0, 0x0023_6e34)?;
        c.sw(r(c, S3).wrapping_add(FINISHED), 0);
        c.g[A0][0] = 7;
        crate::custom_finish_trace::observe(c, "results-screen-request", 0, 7);
        call_ptr(c, 0x0020_a8f8, 0x0023_6e40)?;
        crate::custom_finish_trace::observe(c, "results-screen-returned", 0, 7);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
