//! The game's tick rate (`docs/plans/configurable-tick-rate.md`).
//!
//! The game updates once per VSync. `cAppMan_run` sets the app manager's
//! rate and step (`+0x10 = 60`, `+0x14 = 1/60`); some code reads those, most
//! code loads `1/60` from its own entry in the `.lit4` constant pool (one
//! entry per use site: 190 entries, no inline `1/60` anywhere). A configured
//! rate N (`SSX3_TICK_HZ`, the settings overlay's "Game rate") therefore:
//!
//! - fires VSync at N Hz (the front end passes [`configured`] to
//!   `ssx3_ps2::machine::set_vsync_hz` and installs [`on_vblank`] as the
//!   machine's VSync hook);
//! - writes N and 1/N into the app manager once `cAppMan_run` has set it up;
//! - writes 1/N into every `1/60` entry of `.lit4` (except [`NOT_A_STEP`]).
//!
//! At 60 nothing is written: the game is the original, bit for bit.
//! Hand-written code that needs the step or rate reads [`dt`] / [`rate`],
//! never a constant of its own.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

use ssx3_ee_rt::Cpu;

/// `D_004A5B64`: the `cAppMan*`.
pub const APPMAN_PTR: u32 = 0x004A_5B64;
/// `cAppMan` fields: tick rate (int, Hz), step (f32, s).
pub const APPMAN_RATE: u32 = 0x10;
pub const APPMAN_STEP: u32 = 0x14;
/// `cAppMan +0x24`: game updates queued per VSync (f32, 1.0). `App_VSyncTick`
/// adds it to an accumulator and queues one update per whole unit; the main
/// loop runs the queued updates (at most `+0x20` = 12), then renders once.
pub const APPMAN_UPDATES_PER_VSYNC: u32 = 0x24;
/// The `.lit4` section (gp-relative float constants).
pub const LIT4: (u32, u32) = (0x0049_b100, 0x0049_b100 + 0x5d58);
/// `1/60` as the game stores it.
pub const STEP_60: u32 = 0x3c88_8889;

/// `.lit4` entries holding `1/60` that are not the tick step (left at 1/60).
/// Add an entry with the reason when a worker finds one.
pub const NOT_A_STEP: &[u32] = &[];

/// The configured rate (Hz).
static RATE: AtomicU32 = AtomicU32::new(60);
/// The rate last written into guest memory (0: none yet).
static APPLIED: AtomicU32 = AtomicU32::new(0);
/// Updates to queue at the next VSync (catch-up; 1 = the original).
static UPDATES: AtomicU32 = AtomicU32::new(1);
/// The `.lit4` entries that held `1/60` in the executable.
static POOL: Mutex<Vec<u32>> = Mutex::new(Vec::new());

/// The step for `hz` as an f32 (`1/60` is the game's own bits).
pub fn step_bits(hz: u32) -> u32 {
    if hz == 60 {
        STEP_60
    } else {
        (1.0f32 / hz as f32).to_bits()
    }
}

/// Read `SSX3_TICK_HZ` at boot; returns the configured rate.
pub fn install() -> u32 {
    if let Some(hz) = std::env::var("SSX3_TICK_HZ").ok().and_then(|v| v.parse::<u32>().ok()) {
        set(hz);
    }
    configured()
}

/// Configure the tick rate (Hz, 30..=480); the guest's step values follow at
/// the next [`on_vblank`]. (The caller sets the VSync rate.)
pub fn set(hz: u32) {
    RATE.store(hz.clamp(30, 480), Ordering::Relaxed);
}

/// The configured tick rate (Hz).
pub fn configured() -> u32 {
    RATE.load(Ordering::Relaxed)
}

/// The game's tick rate as the app manager holds it (60 before it exists,
/// e.g. in unit tests).
pub fn rate(c: &mut Cpu) -> u32 {
    match c.bus.read32(APPMAN_PTR) {
        0 => 60,
        app => match c.bus.read32(app + APPMAN_RATE) {
            0 => 60,
            r => r,
        },
    }
}

/// The game's step (s) as the app manager holds it (1/60 before it exists).
pub fn dt(c: &mut Cpu) -> f32 {
    match c.bus.read32(APPMAN_PTR) {
        0 => f32::from_bits(STEP_60),
        app => match c.bus.read32(app + APPMAN_STEP) {
            0 => f32::from_bits(STEP_60),
            b => f32::from_bits(b),
        },
    }
}

/// Frames at 60 Hz as ticks at the current rate (rounded; exact at 60).
pub fn ticks_from_frames60(c: &mut Cpu, frames: u32) -> u32 {
    let r = rate(c);
    if r == 60 {
        frames
    } else {
        ((frames as u64 * r as u64 + 30) / 60) as u32
    }
}

/// Queue `n` game updates at the next VSync instead of one (the front end's
/// catch-up when the host falls behind real time, see [`CatchUp`]).
pub fn set_updates_per_vsync(n: u32) {
    UPDATES.store(n.clamp(1, 12), Ordering::Relaxed);
}

/// At every VSync: bring the guest's step values to the configured rate.
pub fn on_vblank(c: &mut Cpu) {
    sync_updates(c);
    let want = configured();
    if APPLIED.load(Ordering::Relaxed) == 0 && want == 60 {
        // The original rate and nothing written yet: leave memory alone.
        return;
    }
    let app = c.bus.read32(APPMAN_PTR);
    if app != 0 {
        let r = c.bus.read32(app + APPMAN_RATE);
        // `cAppMan_run` has run (rate set) and the rate differs.
        if r != 0 && r != want {
            c.bus.write32(app + APPMAN_RATE, want);
            c.bus.write32(app + APPMAN_STEP, step_bits(want));
        }
    }
    if APPLIED.load(Ordering::Relaxed) != want {
        patch_pool(c, want);
        APPLIED.store(want, Ordering::Relaxed);
    }
}

/// Write the catch-up count into `cAppMan +0x24` when it changes (1.0, the
/// game's own value, unless the host fell behind).
fn sync_updates(c: &mut Cpu) {
    let app = c.bus.read32(APPMAN_PTR);
    if app == 0 {
        return;
    }
    let want = (UPDATES.load(Ordering::Relaxed) as f32).to_bits();
    if c.bus.read32(app + APPMAN_UPDATES_PER_VSYNC) != want {
        c.bus.write32(app + APPMAN_UPDATES_PER_VSYNC, want);
    }
}

/// Real-time catch-up for a front end that runs one VSync per presented
/// frame: game time follows the wall clock when the host is slower than the
/// game rate, by queueing extra updates (the game's own frame-drop path:
/// several updates, one render). Bounded, so a host that is far too slow
/// slows the game down instead of spiralling.
#[derive(Debug, Default)]
pub struct CatchUp {
    /// Wall time (s) the count started at, and updates queued since.
    base: Option<f64>,
    queued: f64,
}

impl CatchUp {
    /// At most this many updates per VSync (4: a 15 fps host keeps a 60 Hz
    /// game at full speed).
    pub const MAX_UPDATES: u32 = 4;
    /// A debt or lead of more than this (s) is forgotten, not repaid: a
    /// pause, the settings overlay, a window drag, a long load.
    pub const FORGET: f64 = 0.25;

    /// Updates to queue at the VSync about to run, at wall time `now` (s)
    /// with one update per `period` (s).
    pub fn next(&mut self, now: f64, period: f64) -> u32 {
        let base = *self.base.get_or_insert(now);
        // Updates real time asks for by the end of this VSync.
        let due = (now - base) / period + 1.0;
        let owed = due - self.queued;
        if !(-Self::FORGET / period..=Self::FORGET / period).contains(&(owed - 1.0)) {
            self.base = Some(now);
            self.queued = 1.0;
            return 1;
        }
        // Half an update of hysteresis: a late frame within jitter does not
        // double the next one.
        let max = Self::MAX_UPDATES as f64;
        let n = (owed - 0.5).floor().clamp(1.0, max);
        if owed - 0.5 > max + 1.0 {
            // More than the cap can repay: let the game slow down instead of
            // carrying the debt.
            self.queued = due - n;
        }
        self.queued += n;
        n as u32
    }

    /// Start over (e.g. after a pause).
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Write the step for `hz` into the `.lit4` `1/60` entries.
fn patch_pool(c: &mut Cpu, hz: u32) {
    let mut pool = POOL.lock().unwrap();
    if pool.is_empty() {
        // First use: the executable's values are still in place.
        let mut a = LIT4.0;
        while a < LIT4.1 {
            if c.bus.read32(a) == STEP_60 && !NOT_A_STEP.contains(&a) {
                pool.push(a);
            }
            a += 4;
        }
    }
    let bits = step_bits(hz);
    for &a in pool.iter() {
        c.bus.write32(a, bits);
    }
    eprintln!("[tick] {hz} Hz: step 1/{hz} in the app manager and {} .lit4 entries", pool.len());
}

/// Unit tests at rate `hz`: an app manager at a scratch address with the
/// rate and step (the test machine's RAM starts empty, where [`rate`] and
/// [`dt`] give 60). A function under test that loads `1/60` from its `.lit4`
/// entry needs that entry too: `c.bus.write32(entry, step_bits(hz))`.
pub fn set_for_test(c: &mut Cpu, hz: u32) {
    const APP: u32 = 0x01ff_f000;
    c.bus.write32(APPMAN_PTR, APP);
    c.bus.write32(APP + APPMAN_RATE, hz);
    c.bus.write32(APP + APPMAN_STEP, step_bits(hz));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_60_is_the_games_constant() {
        assert_eq!(step_bits(60), (1.0f32 / 60.0).to_bits());
        assert_eq!(f32::from_bits(step_bits(120)), 1.0 / 120.0);
    }

    #[test]
    fn catch_up_follows_the_wall_clock() {
        let p = 1.0 / 60.0;
        // A host that keeps up: one update per VSync, also with jitter.
        let mut k = CatchUp::default();
        let mut t = 0.0;
        for i in 0..600 {
            t += p * if i % 2 == 0 { 0.9 } else { 1.1 };
            assert_eq!(k.next(t, p), 1, "vsync {i}");
        }
        // A host at 40 fps: 1.5 updates per VSync on average, game time real.
        let mut k = CatchUp::default();
        let (mut t, mut sum) = (0.0, 0u32);
        for _ in 0..400 {
            t += 1.0 / 40.0;
            sum += k.next(t, p);
        }
        assert!((sum as f64 - t / p).abs() <= 2.0, "{sum} updates in {t} s");
        // A host at 10 fps: capped at 4 per VSync (the game slows down).
        let mut k = CatchUp::default();
        let mut t = 0.0;
        for _ in 0..100 {
            t += 0.1;
            assert!(k.next(t, p) <= CatchUp::MAX_UPDATES);
        }
        // A one-second pause is forgotten, not repaid.
        let mut k = CatchUp::default();
        k.next(0.0, p);
        assert_eq!(k.next(1.0, p), 1);
        assert_eq!(k.next(1.0 + p, p), 1);
    }

    #[test]
    fn test_rate_round_trip() {
        let mut c = ssx3_ps2::new_cpu();
        assert_eq!((rate(&mut c), dt(&mut c)), (60, 1.0 / 60.0));
        set_for_test(&mut c, 120);
        assert_eq!((rate(&mut c), dt(&mut c)), (120, 1.0 / 120.0));
        assert_eq!(ticks_from_frames60(&mut c, 30), 60);
    }
}
