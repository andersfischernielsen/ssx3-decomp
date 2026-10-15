//! Driving loops with real events, for replacements of loops that check
//! events (`crate::events`).
//!
//! [`schedule`] enters the function on a running guest thread with an event
//! due after a few cycles: off, delivered without unwinding, or a VBlank that
//! requests an unwind ([`Events`], picked by `seed % 3`). [`drive`] runs a
//! function the way the trampoline does (re-entering it at the cleared `pc`
//! after an unwind until it returns to `RA`), and [`driven!`] wraps a
//! function in it as a plain `fn` for the comparison. [`check_events`] is
//! `check` that also requires equal temporaries and that the event seeds
//! really reached an event.

use super::{compare, fresh, Rng, RA};
use crate::events::event_due;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// The `UNWIND` bit the kernel sets in `pc`.
pub const UNWIND: u32 = 1;

thread_local! {
    /// Re-entries at a resume point made by [`drive`] (this test thread).
    static RESUMES: std::cell::RefCell<std::collections::BTreeMap<u32, u32>> =
        const { std::cell::RefCell::new(std::collections::BTreeMap::new()) };
}

/// Run `f` the way the trampoline does: after an unwind, deliver any due
/// event and re-enter the function at the (cleared) `pc` until it returns.
pub fn drive(c: &mut Cpu, f: GuestFn) {
    f(c);
    for _ in 0..10_000 {
        if c.pc == RA {
            return;
        }
        assert!(c.pc & UNWIND != 0, "stopped at {:#x} without an unwind", c.pc);
        c.pc &= !UNWIND;
        if event_due(c) && c.event() {
            continue;
        }
        RESUMES.with(|r| *r.borrow_mut().entry(c.pc).or_default() += 1);
        f(c);
    }
    panic!("no return after 10000 re-entries (pc {:#x}, cycles {}, next {})", c.pc, c.cycles, c.next_event);
}

/// Wrap a function in [`drive`] (difftest takes plain `fn`s).
macro_rules! driven {
    ($name:ident, $f:path) => {
        fn $name(c: &mut ssx3_ee_rt::Cpu) {
            $crate::difftest::events::drive(c, $f)
        }
    };
}
pub(crate) use driven;

/// How a seed exercises event delivery.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Events {
    /// No event while the function runs.
    Off,
    /// The first back edge after `k` cycles delivers a VBlank, which
    /// requests an unwind (a frame is ready); later checks unwind too.
    Unwind,
    /// Events are delivered but never unwind (no VBlank due).
    Deliver,
}

/// Enter at `entry` on a running guest thread (so a delivered VBlank has
/// a thread to come back to), with the cycle counter and event schedule
/// for `seed`. Returns the event mode.
pub fn schedule(c: &mut Cpu, r: &mut Rng, seed: u32, entry: u32) -> Events {
    let saved = (c.g[A0], c.g[A1], c.g[31]);
    ssx3_ps2::kernel::start_main_thread(c, entry, 0, 0);
    (c.g[A0], c.g[A1], c.g[31]) = saved;
    c.pc = entry;
    c.cycles = 1000 + (r.next_u32() % 5000) as u64;
    let k = (r.next_u32() % 48) as u64;
    match seed % 3 {
        0 => Events::Off,
        1 => {
            c.next_event = c.cycles + k;
            Events::Unwind
        }
        _ => {
            ssx3_ps2::platform::machine_of(&mut c.bus.plat).sched.next_vblank = 1 << 40;
            c.next_event = c.cycles + k;
            Events::Deliver
        }
    }
}

/// Like `difftest::check` (and also requiring equal temporaries), and
/// asserts that the event seeds really delivered an event inside the
/// function often enough to mean something.
pub fn check_events(
    generated: GuestFn,
    hand: GuestFn,
    seeds: u32,
    setup: &dyn Fn(&mut Cpu, u32) -> Events,
) {
    let (mut evented, mut event_seeds) = (0, 0);
    for seed in 0..seeds {
        let r = compare(generated, hand, &|c| {
            setup(c, seed);
        });
        assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
        // Temporaries too: the replacements reproduce every register. VU0
        // status/MAC/clip flags (ctrl 16-18) stay report-only, as in `check`:
        // they can't be written through the VU0 interface.
        let soft: Vec<&String> = r.soft.iter().filter(|m| !m.starts_with("vu0 ctrl 1")).collect();
        assert!(soft.is_empty(), "seed {seed} (soft): {:#?}", soft);
        let mut c = fresh();
        let mode = setup(&mut c, seed);
        let first = c.next_event;
        generated(&mut c);
        if mode != Events::Off {
            event_seeds += 1;
            // Delivering an event reschedules the next one.
            if c.next_event != first {
                evented += 1;
            }
        }
    }
    assert!(
        evented * 4 >= event_seeds,
        "only {evented} of {event_seeds} event seeds reached an event"
    );
    let resumes = RESUMES.with(|r| std::mem::take(&mut *r.borrow_mut()));
    eprintln!("{evented} of {event_seeds} event seeds delivered an event; resumes (pc: count): {resumes:x?}");
}
