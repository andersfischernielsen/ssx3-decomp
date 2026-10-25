//! `SSX3_EVENT=<code>[:<event>]` (and optionally `SSX3_RIDER=<name|index>`):
//! boot straight into an event. When the boot load is done
//! (`InitLoadScreen_Update` 0x232588 hands the app the start screen: memory
//! card check, profile load, title, then the front end), set the race up
//! through the game database the way the menus do (main menu "Single
//! Event", character select, event select) and hand the app the pre-game
//! load screen instead, as the rules screen's "Start" does
//! (`FEUICustom_CreateState` 0x1a0830, states 0x14/0x15). The memory card,
//! the title and the whole front end never run (the profile load's
//! "continue in the backcountry" goes the same way, `cFEStateProfileLoad_v33`
//! 0x18faf0); the race uses the default profile.
//!
//! The codes are the course codes of the event select tables (0x4781d0
//! races, 0x4786e0 freestyle): `ARA1` Snow Jam, `BRA2` Metro City, `CRA3`
//! Ruthless Ridge, `DRA4` Intimidator, `ERA5` Gravitude; `ASS1`/`DSS2`/`ESS3`
//! slopestyle, `ABA1`/`CBA2`/`EBA3` big air, `BHP1`/`CHP2`/`EHP3` superpipe;
//! `ABC1`/`DBC2`/`EBC3` backcountry (several events each: add `:<event>`,
//! e.g. `ABC1:6`; the first one by default). `SSX3_EVENT=list` prints them.
//!
//! The prompts after the setup (the intro flythrough, the race card's
//! "Continue") are pressed for you: the front ends add [`pad_buttons`] to
//! pad 1 until the race's countdown starts.
//!
//! Off (and never installed) unless `SSX3_EVENT` is set.

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::call::call_ptr;
use crate::call::Flow;

const INIT_LOAD_SCREEN_UPDATE: u32 = 0x0023_2588;
/// The race states that end the boot (no more presses): the countdown, and
/// free ride (events without one).
const RACE_STARTED: [u32; 2] = [0x0011_3d48, 0x0011_3cf0];

const FE_SET_ONLINE_MODE: u32 = 0x001a_86e8;
const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
const BE_NEW_RACE_INT_SET_GAME_MODE: u32 = 0x0014_5108;
const BE_NEW_RACE_INT_SET_EVENT_TYPE: u32 = 0x0014_4df0;
const BE_NEW_RACE_INT_SET_EVENT: u32 = 0x0014_51e8;
const BE_NEW_PLAYER_INT_SET_CHARACTER: u32 = 0x0014_7338;
const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
const EA_MEM_FREE: u32 = 0x0031_7e98;
const PRE_GAME_LOAD_SCREEN_CTOR: u32 = 0x0023_2720;
const APP_SET_NEXT_STATE: u32 = 0x0031_6ef8;

/// The event select tables: (base, peak stride, slots per peak).
const EVENT_TABLES: [(u32, u32, u32); 2] = [(0x0047_81d0, 0x1b0, 4), (0x0047_86e0, 0x21c, 5)];
const EVENT_RECORD: u32 = 0x6c;
/// Course records: 100 bytes each, the course code at +0x34.
const COURSES: u32 = 0x0043_d950;
const COURSE_COUNT: u32 = 17;

/// The riders by character index (`BENewPlayerInt` record byte +1).
pub const RIDERS: [&str; 10] = [
    "moby", "kaori", "allegra", "mac", "zoe", "griff", "elise", "nate", "psymon", "viggo",
];

/// What to boot into.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub code: String,
    pub track: u32,
    pub kind: u32,
    pub event: u32,
}

struct Config {
    want: String,
    rider: Option<u32>,
}

static CONFIG: OnceLock<Config> = OnceLock::new();
/// Where the boot is: before the setup (booting), after it (pressing
/// Cross), in the race (nothing more to do).
static PHASE: AtomicU8 = AtomicU8::new(OFF);
const OFF: u8 = 0;
const BOOT: u8 = 1;
const LOADING: u8 = 2;
const RACING: u8 = 3;

/// The installed functions the boot's wrappers forward to (the hand-written
/// one, else the generated body), by address.
static INNER: OnceLock<Vec<(u32, GuestFn)>> = OnceLock::new();

/// Buttons (libpad bits) the boot holds on pad 1, asked once per pad update
/// by the front ends: Cross pulses from the setup until the race starts,
/// nothing otherwise. Pulsed (6 updates on, 6 off) so each press is
/// a new edge.
pub fn pad_buttons() -> u16 {
    static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = N.fetch_add(1, Ordering::Relaxed);
    if (n / 6) % 2 == 1 {
        return 0;
    }
    match PHASE.load(Ordering::Relaxed) {
        LOADING => 0x4000,
        _ => 0,
    }
}

/// Install the boot hook when `SSX3_EVENT` is set. Call after
/// [`crate::install`] (it wraps the hand-written boot load update).
pub fn install() {
    let Ok(want) = std::env::var("SSX3_EVENT") else {
        return;
    };
    let rider = std::env::var("SSX3_RIDER").ok().map(|r| {
        let r = r.to_ascii_lowercase();
        RIDERS
            .iter()
            .position(|n| *n == r)
            .map(|i| i as u32)
            .or_else(|| r.parse().ok())
            .unwrap_or_else(|| panic!("SSX3_RIDER: unknown rider {r:?} (one of {RIDERS:?})"))
    });
    let _ = CONFIG.set(Config { want, rider });
    let installed = crate::overrides();
    let inner = std::iter::once(INIT_LOAD_SCREEN_UPDATE)
        .chain(RACE_STARTED)
        .map(|a| {
            let f = installed
                .iter()
                .find(|(addr, _, _)| *addr == a)
                .map(|(_, f, _)| *f)
                .or_else(|| ssx3_ee::generated_body(a))
                .expect("event boot: no function to wrap");
            (a, f)
        })
        .collect();
    let _ = INNER.set(inner);
    assert!(ssx3_ee::set_override(
        INIT_LOAD_SCREEN_UPDATE,
        init_load_screen_update
    ));
    assert!(ssx3_ee::set_override(RACE_STARTED[0], countdown_update));
    assert!(ssx3_ee::set_override(RACE_STARTED[1], freeride_update));
    PHASE.store(BOOT, Ordering::Relaxed);
}

fn inner(addr: u32) -> GuestFn {
    INNER
        .get()
        .and_then(|v| v.iter().find(|(a, _)| *a == addr))
        .map(|(_, f)| *f)
        .expect("event boot: wrapped function")
}

/// The race states' updates: the boot is over.
fn countdown_update(c: &mut Cpu) {
    race_started();
    inner(RACE_STARTED[0])(c);
}

fn freeride_update(c: &mut Cpu) {
    race_started();
    inner(RACE_STARTED[1])(c);
}

fn race_started() {
    let _ = PHASE.compare_exchange(LOADING, RACING, Ordering::Relaxed, Ordering::Relaxed);
}

/// Every event of the event select tables, read from guest memory.
pub fn events(c: &mut Cpu) -> Vec<Event> {
    let mut out = Vec::new();
    for (base, stride, slots) in EVENT_TABLES {
        for peak in 0..3 {
            for slot in 0..slots {
                let rec = base + peak * stride + slot * EVENT_RECORD;
                let track = c.bus.read32(rec);
                if track >= COURSE_COUNT {
                    continue;
                }
                let code = (0..4)
                    .map(|i| c.bus.read8(COURSES + track * 100 + 0x34 + i) as char)
                    .collect();
                out.push(Event {
                    code,
                    track,
                    kind: c.bus.read32(rec + 4),
                    event: c.bus.read32(rec + 8),
                });
            }
        }
    }
    out
}

/// Pick the event `want` names (`CODE` or `CODE:event`).
pub fn find(events: &[Event], want: &str) -> Option<Event> {
    let (code, event) = match want.split_once(':') {
        Some((c, e)) => (c, Some(e.parse::<u32>().ok()?)),
        None => (want, None),
    };
    events
        .iter()
        .find(|e| e.code.eq_ignore_ascii_case(code) && event.is_none_or(|n| e.event == n))
        .cloned()
}

/// The boot load screen's update; when it hands the app the start screen
/// (the boot load is done), the race setup and the pre-game load screen
/// instead.
fn init_load_screen_update(c: &mut Cpu) {
    inner(INIT_LOAD_SCREEN_UPDATE)(c);
    let gp = c.g[GP][0] as u32;
    let app = c.bus.read32(gp.wrapping_sub(0x848));
    // `App_SetNextState` stores the next state at app+4.
    let start_screen = c.bus.read32(app + 4);
    if start_screen == 0
        || PHASE
            .compare_exchange(BOOT, LOADING, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
    {
        return;
    }
    let cfg = CONFIG.get().expect("event boot installed without config");
    let all = events(c);
    if cfg.want.eq_ignore_ascii_case("list") {
        for e in &all {
            eprintln!(
                "[event] {}:{} (track {}, type {})",
                e.code, e.event, e.track, e.kind
            );
        }
        PHASE.store(RACING, Ordering::Relaxed);
        return;
    }
    let Some(ev) = find(&all, &cfg.want) else {
        eprintln!(
            "[event] SSX3_EVENT={:?}: no such event (SSX3_EVENT=list lists them)",
            cfg.want
        );
        PHASE.store(RACING, Ordering::Relaxed);
        return;
    };
    eprintln!(
        "[event] booting into {}:{} (track {}, type {}){}",
        ev.code,
        ev.event,
        ev.track,
        ev.kind,
        cfg.rider
            .map(|r| format!(" as {}", RIDERS.get(r as usize).copied().unwrap_or("?")))
            .unwrap_or_default()
    );
    // The caller resumes at `back` with the update's own results (it
    // returns nothing); the calls below only use caller-saved registers.
    let back = c.pc;
    let ra = c.g[RA][0];
    // The start screen is only constructed (`StartScreen_ctor` sets fields):
    // free it, the pre-game load screen replaces it.
    if call(c, EA_MEM_FREE, &[start_screen], back).is_err()
        || start(c, &ev, cfg.rider, back).is_err()
    {
        // A callee switched away (host-thread unwind): the stack is gone.
        return;
    }
    c.g[RA][0] = ra;
    c.pc = back;
}

fn call(c: &mut Cpu, target: u32, args: &[u32], back: u32) -> Flow {
    for (i, a) in args.iter().enumerate() {
        c.g[A0 + i][0] = sx(*a);
    }
    call_ptr(c, target, back)
}

/// `iface->commit()`: the `BEInterface` vtable (at +0xc) entry at +8.
fn commit(c: &mut Cpu, iface: u32, back: u32) -> Flow {
    let vt = c.bus.read32(iface + 0xc);
    let delta = c.bus.read16(vt + 8) as i16 as i32;
    let f = c.bus.read32(vt + 0xc);
    call(c, f, &[iface.wrapping_add(delta as u32)], back)
}

fn interface(c: &mut Cpu, n: u32, back: u32) -> Result<u32, crate::call::Unwound> {
    call(c, BE_LIBRARY_GET_INSTANCE, &[], back)?;
    let lib = c.g[V0][0] as u32;
    call(c, BE_LIBRARY_GET_INTERFACE, &[lib, n], back)?;
    Ok(c.g[V0][0] as u32)
}

fn start(c: &mut Cpu, ev: &Event, rider: Option<u32>, back: u32) -> Flow {
    let gp = c.g[GP][0] as u32;
    // Main menu "Single Event" (cFEStateMainMenu_OnWidgetAction, widget 9).
    call(c, FE_SET_ONLINE_MODE, &[0], back)?;
    let race = interface(c, 0, back)?;
    call(c, BE_NEW_RACE_INT_SET_GAME_MODE, &[race, 1], back)?;
    commit(c, race, back)?;
    // Character select: player 0's character.
    if let Some(ch) = rider {
        let player = interface(c, 1, back)?;
        call(c, BE_NEW_PLAYER_INT_SET_CHARACTER, &[player, 0, ch], back)?;
        commit(c, player, back)?;
    }
    // Entering event select (FEUICustom_CreateState, state 0xc).
    c.bus.write32(gp.wrapping_sub(0xb4c), 1);
    c.bus.write32(gp.wrapping_sub(0xb5c), 0);
    c.bus.write32(gp.wrapping_sub(0xb58), 0);
    // Event select confirm (EventSelect_OnWidgetAction, single event).
    c.bus.write32(0x0053_5c08, ev.track);
    c.bus.write32(gp.wrapping_sub(0x1f38), 1);
    call(c, BE_NEW_RACE_INT_SET_EVENT_TYPE, &[race, ev.kind], back)?;
    call(c, BE_NEW_RACE_INT_SET_EVENT, &[race, ev.event], back)?;
    commit(c, race, back)?;
    // Rules "Start": the pre-game load screen becomes the app's next state.
    call(c, EA_MEM_ALLOC_NAMED, &[8, 0x0045_ce60, 0x100, 0], back)?;
    let state = c.g[V0][0] as u32;
    call(c, PRE_GAME_LOAD_SCREEN_CTOR, &[state], back)?;
    let app = c.bus.read32(gp.wrapping_sub(0x848));
    call(c, APP_SET_NEXT_STATE, &[app, state], back)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(code: &str, track: u32, kind: u32, event: u32) -> Event {
        Event {
            code: code.into(),
            track,
            kind,
            event,
        }
    }

    #[test]
    fn find_picks_code_and_event() {
        let all = [
            ev("BRA2", 1, 0, 0),
            ev("ABC1", 14, 5, 4),
            ev("ABC1", 14, 5, 6),
        ];
        assert_eq!(find(&all, "bra2"), Some(all[0].clone()));
        assert_eq!(find(&all, "ABC1"), Some(all[1].clone()));
        assert_eq!(find(&all, "ABC1:6"), Some(all[2].clone()));
        assert_eq!(find(&all, "ABC1:9"), None);
        assert_eq!(find(&all, "ZZZ9"), None);
    }
}
