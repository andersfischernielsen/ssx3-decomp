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
/// `cAppMan +0x18`: VSyncs seen (int, incremented by `App_VSyncTick`). The UI
/// and script timelines advance in 60 Hz frames against it.
pub const APPMAN_VSYNC: u32 = 0x18;
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
///
/// The script and animation data are authored at a fixed 60 Hz. A script
/// frame count is not a tick count: the script timeline runs in real time
/// (`ScriptList_Start` adds keyframe counts to a frame counter, and
/// `cTransitionMan_Update` advances its fade by `1.0 / rate` per tick), so a
/// duration of `frames * (1/60)` is a wall-clock time at every rate. Patch
/// these and the transition would be 4x shorter at 240 Hz.
pub const NOT_A_STEP: &[u32] = &[
    // Rider_ApplyScriptTrack (0x124788): script track time in frames ->
    // seconds for AnimController_SampleAnimPose / AnimSequence_SetTrackTime.
    0x0049_b860,
    // Rider_ApplyScriptTrack (0x124788): track start frame -> seconds for
    // AnimController_PlayAnimDirect.
    0x0049_b868,
    // ScriptList_Start (0x276388): three keyframe counts -> the transition
    // duration passed to cTransitionMan_Start.
    0x0049_e8ec,
    // ScriptList_Start (0x276388): keyframe count -> the captured
    // transition's duration.
    0x0049_e8f0,
    // ScriptList_StartTransition (0x277980): script shorts -> transition
    // start/duration in seconds.
    0x0049_e8f4,
    // ScriptList_StartTransition (0x277980): script short -> captured
    // transition's duration.
    0x0049_e8f8,
];

/// 60 Hz frame counts in the game's *data* (not `.lit4`), rewritten to the
/// same wall-clock duration in ticks once, when the rate is applied. They may
/// be live countdowns afterwards, so they are written only on a rate change.
pub const FRAME_COUNTS: &[(u32, u32)] = &[
    // `D_004A17C0`: the title screen's pre-fade wait, 300 frames (5 s). Read
    // and decremented only by cFEStateTitle_TransitionIn (0x1947f8), so the
    // countdown runs 300 ticks at 60 Hz, 600 at 120, 1200 at 240.
    (0x004a_17c0, 300),
];

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
/// catch-up when the host falls behind real time, see [`Pacer`]).
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
        patch_frame_counts(c, want);
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

/// What a front end runs before it presents its next image ([`Pacer`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plan {
    /// VSyncs to run (0: nothing is due yet, wait).
    pub vsyncs: u32,
    /// Game updates each of them queues ([`set_updates_per_vsync`]).
    pub updates_per_vsync: u32,
}

/// The frame pacer: game time runs on the wall clock at the game rate,
/// whatever the display or the host can do (a fixed-timestep loop).
///
/// - The game advances one VSync per update period of wall time, so the
///   emulated clock (loading, streaming, timers, audio) stays real time.
/// - A display slower than the game rate gets several VSyncs per image
///   (`max_vsyncs` = game rate / display rate, rounded up); a faster one waits
///   ([`Pacer::wait`]) and shows each game frame until the next.
/// - A host too slow to run that many VSyncs (catch-up on) queues extra game
///   updates per VSync instead: the game's own frame-drop path (several
///   updates, one render), at most [`Pacer::MAX_UPDATES_PER_VSYNC`]. Beyond
///   that the game slows down rather than spiralling.
#[derive(Debug, Default)]
pub struct Pacer {
    /// Wall time (s) of update 0 and updates delivered since.
    base: Option<f64>,
    done: f64,
    /// The previous run's time and the longest recent frame (s, decaying).
    last: Option<f64>,
    worst: f64,
}

impl Pacer {
    pub const MAX_UPDATES_PER_VSYNC: u32 = 4;
    /// A debt or lead of more than this (s) is forgotten, not repaid: a
    /// pause, the settings overlay, a window drag, a long stall.
    pub const FORGET: f64 = 0.25;

    /// Updates the wall clock asks for that are not delivered yet.
    fn owed(&mut self, now: f64, period: f64) -> f64 {
        let base = *self.base.get_or_insert(now - period);
        let owed = (now - base) / period - self.done;
        if owed.abs() > Self::FORGET / period {
            self.base = Some(now - period);
            self.done = 0.0;
            return 1.0;
        }
        owed
    }

    /// The plan at wall time `now` (s), one update per `period` (s).
    pub fn plan(&mut self, now: f64, period: f64, max_vsyncs: u32, catch_up: bool) -> Plan {
        let owed = self.owed(now, period);
        // Nearest whole update: half an update of slack either way absorbs
        // jitter without dropping or doubling frames.
        let n = (owed + 0.5).floor().max(0.0) as u32;
        if n == 0 {
            return Plan { vsyncs: 0, updates_per_vsync: 0 };
        }
        self.note_frame(now);
        let vsyncs = n.min(max_vsyncs.max(1));
        let per = if catch_up { n.div_ceil(vsyncs).min(Self::MAX_UPDATES_PER_VSYNC) } else { 1 };
        let delivered = (vsyncs * per) as f64;
        self.done += delivered;
        if owed - delivered > 1.5 {
            // More than the caps can repay: slow down instead of carrying it.
            self.done += owed - delivered;
        }
        Plan { vsyncs, updates_per_vsync: per }
    }

    /// For a front end that runs exactly one VSync per frame (headless):
    /// the updates that VSync queues (1 when ahead or on time).
    pub fn per_vsync(&mut self, now: f64, period: f64) -> u32 {
        let owed = self.owed(now, period);
        self.note_frame(now);
        let n = ((owed + 0.5).floor().max(1.0) as u32).min(Self::MAX_UPDATES_PER_VSYNC);
        self.done += n as f64;
        if owed - n as f64 > 1.5 {
            self.done += owed - n as f64;
        }
        n
    }

    /// Seconds until the next update is due (after a plan with no VSyncs).
    pub fn wait(&self, now: f64, period: f64) -> f64 {
        match self.base {
            Some(base) => (base + (self.done + 0.5) * period - now).max(0.0),
            None => 0.0,
        }
    }

    fn note_frame(&mut self, now: f64) {
        if let Some(l) = self.last {
            let dt = (now - l).clamp(0.0, 1.0);
            self.worst = dt.max(self.worst * 0.99);
        }
        self.last = Some(now);
    }

    /// Audio to keep queued (s): the device must not run dry while the host
    /// is busy with one frame. 2.5 × the longest recent frame, 60..180 ms.
    pub fn audio_lead(&self) -> f64 {
        (self.worst * 2.5).clamp(0.06, 0.18)
    }

    /// Start over (e.g. after a pause).
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Per-object clocks of [`frame60_due`]: the object's address → (credit,
/// the frame it was left at).
struct Clock {
    credit: u32,
    frame: u32,
}

thread_local! {
    static CLOCKS: std::cell::RefCell<std::collections::HashMap<u32, Clock>> = Default::default();
}

/// Whether an object's timeline authored at 60 fps (a UI animation, a screen
/// script thread) takes its next frame this tick: always at 60; above 60,
/// at once when the timeline is new or was moved since its last step (`frame`
/// differs from what [`frame60_ran`] recorded, e.g. a newly started thread
/// or a restarted animation), then once per `rate / 60` ticks of its own.
///
/// Each object keeps its own phase, so a timeline's first frame runs on the
/// tick it starts, in the same order with the code around it as at 60 (a
/// shared 60 Hz boundary delays it by up to `rate / 60 - 1` ticks; measured:
/// a menu's description text is then wrapped before its box is sized).
/// Call once per tick per object, and [`frame60_ran`] after the step.
pub fn frame60_due(c: &mut Cpu, key: u32, frame: u32) -> bool {
    let r = rate(c);
    if r == 60 {
        return true;
    }
    CLOCKS.with(|m| {
        let mut m = m.borrow_mut();
        match m.get_mut(&key) {
            Some(k) if k.frame == frame => {
                k.credit += 60;
                if k.credit >= r {
                    k.credit -= r;
                    true
                } else {
                    false
                }
            }
            _ => {
                if m.len() > 4096 {
                    m.clear();
                }
                m.insert(key, Clock { credit: 0, frame });
                true
            }
        }
    })
}

/// After a [`frame60_due`] step: the frame the object was left at.
pub fn frame60_ran(c: &mut Cpu, key: u32, frame: u32) {
    if rate(c) == 60 {
        return;
    }
    CLOCKS.with(|m| {
        if let Some(k) = m.borrow_mut().get_mut(&key) {
            k.frame = frame;
        }
    });
}

/// A 60 Hz frame boundary for UI/script timelines authored at 60 fps: true
/// once per 60 Hz frame at any rate (always at 60).
///
/// [`APPMAN_VSYNC`] is the VSync counter `App_VSyncTick` bumps once per VSync;
/// it advances 60 times per second at any rate.
///
/// Use it for a quantity that is authored per 60 Hz frame and cannot be
/// scaled (an integer step, a scroll in pixels per frame). Do **not** use it
/// to skip a whole UI screen update: the screens also lay out their widgets,
/// which must happen every tick. For a timeline that belongs to one object,
/// use [`frame60_due`] (its own phase).
pub fn ui_frame_due(c: &mut Cpu) -> bool {
    let r = rate(c);
    if r == 60 {
        return true;
    }
    let t = vsync_count(c);
    ((t as u64 * 60) % r as u64) < 60
}

/// The VSync counter `App_VSyncTick` keeps at `cAppMan +0x18` (0 before the
/// app manager exists).
pub fn vsync_count(c: &mut Cpu) -> u32 {
    match c.bus.read32(APPMAN_PTR) {
        0 => 0,
        app => c.bus.read32(app + APPMAN_VSYNC),
    }
}

/// Write the step for `hz` into the `.lit4` `1/60` entries.
fn patch_pool(c: &mut Cpu, hz: u32) {
    let mut pool = POOL.lock().unwrap();
    if pool.is_empty() {
        // First use: the executable's values are still in place.
        *pool = collect_pool(c);
    }
    let bits = step_bits(hz);
    for &a in pool.iter() {
        c.bus.write32(a, bits);
    }
    eprintln!("[tick] {hz} Hz: step 1/{hz} in the app manager and {} .lit4 entries", pool.len());
}

/// Write the tick count for each 60 Hz frame count in [`FRAME_COUNTS`].
fn patch_frame_counts(c: &mut Cpu, hz: u32) {
    for &(a, frames) in FRAME_COUNTS {
        let n = if hz == 60 {
            frames
        } else {
            ((frames as u64 * hz as u64 + 30) / 60) as u32
        };
        c.bus.write32(a, n);
    }
}

/// The `.lit4` entries holding `1/60` that follow the rate.
fn collect_pool(c: &mut Cpu) -> Vec<u32> {
    let mut out = Vec::new();
    let mut a = LIT4.0;
    while a < LIT4.1 {
        if c.bus.read32(a) == STEP_60 && !NOT_A_STEP.contains(&a) {
            out.push(a);
        }
        a += 4;
    }
    out
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

    /// Runs a front end for `secs`: images every `image` s (the display or
    /// the host), returns (updates delivered, VSyncs run).
    fn simulate(game_hz: f64, image: f64, max_vsyncs: u32, catch_up: bool, secs: f64) -> (u32, u32) {
        let p = 1.0 / game_hz;
        let mut k = Pacer::default();
        let (mut t, mut updates, mut vsyncs) = (0.0, 0, 0);
        while t < secs {
            let plan = k.plan(t, p, max_vsyncs, catch_up);
            if plan.vsyncs == 0 {
                t += k.wait(t, p).max(1e-4);
                continue;
            }
            updates += plan.vsyncs * plan.updates_per_vsync;
            vsyncs += plan.vsyncs;
            t += image;
        }
        (updates, vsyncs)
    }

    #[test]
    fn pacer_runs_the_game_at_its_rate_whatever_the_display() {
        // 60 Hz game, 60 Hz display; 120 Hz game on 60 Hz (2 VSyncs per
        // image); 60 Hz game on 144 Hz (waits); 240 Hz game on 60 Hz.
        for (game, display) in [(60.0, 60.0), (120.0, 60.0), (60.0, 144.0), (240.0, 60.0), (144.0, 60.0)] {
            let maxv = (game / display as f64).ceil() as u32;
            let (u, v) = simulate(game, 1.0 / display, maxv, true, 10.0);
            assert!((u as f64 - 10.0 * game).abs() <= 3.0, "{game} on {display}: {u} updates");
            // The emulated clock is real time too: one VSync per update.
            assert_eq!(u, v, "{game} on {display}");
        }
    }

    #[test]
    fn pacer_catches_up_on_a_slow_host() {
        // A 40 fps host at 60 Hz: extra updates, game time real.
        let (u, _) = simulate(60.0, 1.0 / 40.0, 1, true, 10.0);
        assert!((u as f64 - 600.0).abs() <= 3.0, "{u}");
        // Without catch-up it slows down (one update per image).
        let (u, _) = simulate(60.0, 1.0 / 40.0, 1, false, 10.0);
        assert!((u as f64 - 400.0).abs() <= 3.0, "{u}");
        // A 10 fps host: capped (the game slows down, no spiral).
        let (u, _) = simulate(60.0, 0.1, 1, true, 10.0);
        assert!(u <= 101 * Pacer::MAX_UPDATES_PER_VSYNC && u < 600, "{u}");
    }

    #[test]
    fn pacer_jitter_and_pauses() {
        // A host that keeps up with jitter: one update per image.
        let p = 1.0 / 60.0;
        let mut k = Pacer::default();
        let mut t = 0.0;
        for i in 0..600 {
            let plan = k.plan(t, p, 1, true);
            assert_eq!(plan, Plan { vsyncs: 1, updates_per_vsync: 1 }, "image {i}");
            t += p * if i % 2 == 0 { 0.9 } else { 1.1 };
        }
        // A one-second pause is forgotten, not repaid.
        let plan = k.plan(t + 1.0, p, 1, true);
        assert_eq!(plan.updates_per_vsync, 1);
        // One VSync per frame (headless): extra updates when behind.
        let mut k = Pacer::default();
        let mut sum = 0;
        for i in 0..400 {
            sum += k.per_vsync(i as f64 / 40.0, p);
        }
        assert!((sum as f64 - 600.0).abs() <= 3.0, "{sum}");
    }

    #[test]
    fn test_rate_round_trip() {
        let mut c = ssx3_ps2::new_cpu();
        assert_eq!((rate(&mut c), dt(&mut c)), (60, 1.0 / 60.0));
        set_for_test(&mut c, 120);
        assert_eq!((rate(&mut c), dt(&mut c)), (120, 1.0 / 120.0));
        assert_eq!(ticks_from_frames60(&mut c, 30), 60);
    }

    /// The script/animation frame-time entries stay at `1/60`; every other
    /// `1/60` entry of `.lit4` follows the rate.
    #[test]
    fn script_frame_entries_are_not_the_tick_step() {
        let mut c = ssx3_ps2::new_cpu();
        for (i, &e) in NOT_A_STEP.iter().enumerate() {
            assert!((LIT4.0..LIT4.1).contains(&e), "{e:#x} is not in .lit4");
            assert!(!NOT_A_STEP[..i].contains(&e), "{e:#x} is listed twice");
            c.bus.write32(e, STEP_60);
        }
        c.bus.write32(0x0049_b118, STEP_60);
        let pool = collect_pool(&mut c);
        assert!(pool.contains(&0x0049_b118));
        for &e in NOT_A_STEP {
            assert!(!pool.contains(&e), "{e:#x}");
        }
    }

    /// The UI/screen timeline advances one frame per VSync at 60 Hz and one
    /// frame per 60 Hz boundary above it: 60 frames per second at 60, 120,
    /// 144 and 240 Hz, and every VSync at 60.
    #[test]
    fn ui_frames_stay_at_60_per_second() {
        let mut c = ssx3_ps2::new_cpu();
        const APP: u32 = 0x01ff_f000;
        set_for_test(&mut c, 60);
        for t in 1..=60u32 {
            c.bus.write32(APP + APPMAN_VSYNC, t);
            assert!(ui_frame_due(&mut c), "60 Hz VSync {t}");
        }
        for hz in [120u32, 144, 240] {
            set_for_test(&mut c, hz);
            let mut n = 0;
            for t in 1..=hz {
                c.bus.write32(APP + APPMAN_VSYNC, t);
                if ui_frame_due(&mut c) {
                    assert_eq!(vsync_count(&mut c), t);
                    n += 1;
                }
            }
            assert_eq!(n, 60, "{hz} Hz: {n} frames per second");
        }
    }

    /// A 60 Hz frame count in the game's data becomes the same wall-clock
    /// duration in ticks: 300 frames stay 300 at 60 Hz, 600 at 120, 1200 at
    /// 240. It is written only on a rate change, so a live countdown is not
    /// restarted.
    #[test]
    fn frame_count_data_follows_the_rate() {
        let mut c = ssx3_ps2::new_cpu();
        assert_eq!(FRAME_COUNTS, &[(0x004a_17c0, 300)]);
        patch_frame_counts(&mut c, 60);
        assert_eq!(c.bus.read32(0x004a_17c0), 300);
        patch_frame_counts(&mut c, 120);
        assert_eq!(c.bus.read32(0x004a_17c0), 600);
        patch_frame_counts(&mut c, 240);
        assert_eq!(c.bus.read32(0x004a_17c0), 1200);
    }
}
