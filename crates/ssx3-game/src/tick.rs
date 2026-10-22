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

/// At every VSync: bring the guest's step values to the configured rate.
pub fn on_vblank(c: &mut Cpu) {
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
    fn test_rate_round_trip() {
        let mut c = ssx3_ps2::new_cpu();
        assert_eq!((rate(&mut c), dt(&mut c)), (60, 1.0 / 60.0));
        set_for_test(&mut c, 120);
        assert_eq!((rate(&mut c), dt(&mut c)), (120, 1.0 / 120.0));
        assert_eq!(ticks_from_frames60(&mut c, 30), 60);
    }
}
