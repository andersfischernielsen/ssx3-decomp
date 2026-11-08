//! SSX 3 (USA), native recompilation, as a library: what every front end
//! (the desktop window, the headless runner, the browser, or a front end in
//! another repository) drives. See `docs/frontends.md`.
//!
//! A front end:
//! 1. optionally [`init_paths`] first (data and save roots, a standalone
//!    pack), when it reads its own settings from the save folder;
//! 2. [`Engine::boot`] with an [`EngineConfig`] (its [`Settings`]);
//! 3. every display refresh: feeds input ([`Engine::set_pad`],
//!    [`Engine::set_sticks`]), runs the VSyncs that are due
//!    ([`Engine::advance`], or [`Engine::pace`] + [`Engine::step`]), and
//!    presents the renderer's latest image ([`Engine::renderer`]) on its own
//!    surface;
//! 4. tells the engine about its lifecycle: [`Engine::pause`] /
//!    [`Engine::resume`] (hidden, backgrounded, a menu over the game),
//!    [`Engine::audio_suspend`] / [`Engine::audio_resume`] (audio focus
//!    lost), [`Engine::set_surface_size`] (resized).
//!
//! The generated EE code comes in one of two forms (features `ee-threads`,
//! the default, or `ee-unwind`, single-threaded, which the browser needs).

// The generated EE code in the chosen form (features `ee-threads` / `ee-unwind`).
#[cfg(all(feature = "ee-threads", not(feature = "ee-unwind")))]
extern crate ssx3_ee_threads as ssx3_ee;
#[cfg(feature = "ee-unwind")]
extern crate ssx3_ee_unwind as ssx3_ee;

mod boot;
mod debug;
mod render;

use std::path::PathBuf;

pub use boot::{init_paths, repo_root, BOOT_ENV_KEYS};
pub use debug::{dump, status};
pub use ssx3_gfx::Renderer;
pub use ssx3_ps2::Cpu;
/// The crates a front end works with, at the versions the engine uses.
pub use {ssx3_core, ssx3_game, ssx3_gfx, ssx3_iop, ssx3_ps2};

use ssx3_game::tick::{self, Pacer};

/// Where a front end's settings come from (a settings file, the environment,
/// fixed values, a platform's preferences). Keys are the `SSX3_*` names of
/// `docs/options.md`; `None` (or an empty value) is the default.
///
/// The engine reads the image settings through this whenever they apply
/// (at boot and on [`Engine::setting_changed`]), so a front end that edits
/// settings while running sees them take effect. Settings the game's crates
/// read while booting ([`BOOT_ENV_KEYS`]) are copied into the process
/// environment by [`Engine::boot`] unless the environment sets them; debug
/// switches stay environment variables.
pub trait Settings {
    fn get(&self, key: &str) -> Option<String>;
}

/// The process environment (empty values count as unset).
pub struct EnvSettings;

impl Settings for EnvSettings {
    fn get(&self, key: &str) -> Option<String> {
        std::env::var(key).ok().filter(|v| !v.is_empty())
    }
}

impl<F: Fn(&str) -> Option<String>> Settings for F {
    fn get(&self, key: &str) -> Option<String> {
        self(key).filter(|v| !v.is_empty())
    }
}

/// How to boot.
pub struct EngineConfig {
    /// Game data root (disc image, `orig/`, texture packs); see [`init_paths`].
    pub data_root: Option<PathBuf>,
    /// Save root (memory card, settings, dumps); see [`init_paths`].
    pub save_root: Option<PathBuf>,
    /// Look for a standalone pack (`tools/bundle.py`); see [`init_paths`].
    pub find_pack: bool,
    /// Skip the boot movies unless the `SSX3_SKIP_MOVIES` setting says
    /// otherwise (scripted input timings assume they are skipped).
    pub skip_movies: bool,
    pub settings: Box<dyn Settings>,
}

impl EngineConfig {
    /// Checkout-relative paths, no pack, movies played.
    pub fn new(settings: Box<dyn Settings>) -> Self {
        Self {
            data_root: None,
            save_root: None,
            find_pack: false,
            skip_movies: false,
            settings,
        }
    }
}

/// What [`Engine::pace`] asks the front end to do now.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pace {
    /// Run this many VSyncs ([`Engine::step`]), then present.
    Run(u32),
    /// Nothing is due: present the last image again (locked to the display,
    /// this waits for its next refresh).
    Repeat,
    /// Nothing is due for this many seconds: wait (or present and wait).
    Wait(f64),
    /// [`Engine::pause`]d: run nothing.
    Paused,
}

/// What [`Engine::advance`] did.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Advance {
    /// VSyncs run.
    pub vsyncs: u32,
    /// False when the game halted.
    pub alive: bool,
    /// Present an image now (a new one, or the last one again).
    pub present: bool,
    /// Seconds until the next VSync is due (when nothing ran).
    pub wait: f64,
}

/// The game: the machine, the renderer, input and pacing state.
pub struct Engine {
    cpu: Box<Cpu>,
    gfx: Option<Renderer>,
    settings: Box<dyn Settings>,
    pads: [u16; 2],
    sticks: [[u8; 4]; 2],
    pacer: Pacer,
    catch_up: bool,
    display_hz: f64,
    paused: bool,
    audio_interrupted: bool,
    surface: Option<(u32, u32)>,
}

/// Renderer options among the settings ([`Engine::setting_changed`]).
const GFX_KEYS: &[&str] = &[
    "SSX3_AA",
    "SSX3_LOD_BIAS",
    "SSX3_AO",
    "SSX3_SHADOWS",
    "SSX3_BLOOM",
    "SSX3_SNOW",
];

impl Engine {
    /// Create the machine, load the disc and the executable, start the game
    /// and install the renderer (`SSX3_RENDER`; none without a GPU). The
    /// GPU context (`ssx3_ps2::gpu`) is created here unless the front end
    /// created it first (the browser must: `gpu::init_context().await`).
    pub fn boot(config: EngineConfig) -> Engine {
        init_paths(config.data_root, config.save_root, config.find_pack);
        let settings = config.settings;
        boot::export_boot_settings(&*settings);
        let skip = settings
            .get("SSX3_SKIP_MOVIES")
            .map(|v| v != "0")
            .unwrap_or(config.skip_movies);
        let mut cpu = boot::boot(skip);
        let gfx = render::install_renderer(&mut cpu, &*settings);
        Engine {
            cpu,
            gfx,
            settings,
            pads: [0; 2],
            sticks: [[0x80; 4]; 2],
            pacer: Pacer::default(),
            catch_up: false,
            display_hz: 60.0,
            paused: false,
            audio_interrupted: false,
            surface: None,
        }
    }

    // ------------------------------------------------------------ state

    pub fn settings(&self) -> &dyn Settings {
        &*self.settings
    }

    /// The machine (debugging, verification, front-end diagnostics).
    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn cpu_mut(&mut self) -> &mut Cpu {
        &mut self.cpu
    }

    /// The draw-layer renderer, when installed.
    pub fn renderer(&self) -> Option<&Renderer> {
        self.gfx.as_ref()
    }

    /// Whether the renderer's image is the one to show (otherwise the
    /// emulated GS's, `emu-gs` builds only).
    pub fn native_image(&self) -> bool {
        self.gfx.is_some() && ssx3_ps2::frame_sink::native()
    }

    /// The image to show as opaque `0xAABBGGRR` words, when the renderer
    /// draws it (screenshots, CPU-side presenters).
    pub fn read_image(&self) -> Option<(u32, u32, Vec<u32>)> {
        let (w, h, px) = self
            .gfx
            .as_ref()
            .filter(|_| ssx3_ps2::frame_sink::native())?
            .read_image()?;
        Some((w, h, px.iter().map(|p| p | 0xFF00_0000).collect()))
    }

    /// VSyncs run so far.
    pub fn frames(&self) -> u64 {
        ssx3_ps2::mr!(self.cpu).sched.frames
    }

    /// One-line status ([`status`]).
    pub fn status(&self) -> String {
        status(&self.cpu)
    }

    /// Every thread and semaphore ([`dump`]).
    pub fn dump(&self) -> String {
        dump(&self.cpu)
    }

    // ------------------------------------------------------------ input

    /// Pad `port` (0 or 1)'s buttons: libpad bits, pressed = 1 (`docs/frontends.md`).
    /// Applied at the next [`Engine::step`].
    pub fn set_pad(&mut self, port: usize, buttons: u16) {
        self.pads[port & 1] = buttons;
    }

    /// Pad `port`'s analog sticks: right x, right y, left x, left y; 0 =
    /// left/up, 0x80 = centre, 0xFF = right/down.
    pub fn set_sticks(&mut self, port: usize, sticks: [u8; 4]) {
        self.sticks[port & 1] = sticks;
    }

    // ------------------------------------------------------------ stepping

    /// Run one VSync (one game frame at the game rate) with the current
    /// input. False when the game halted.
    pub fn step(&mut self) -> bool {
        // SSX3_EVENT: the boot presses through the title and the prompts.
        let boot = ssx3_game::event_boot::pad_buttons();
        let sif = &mut ssx3_ps2::m!(self.cpu).sif;
        sif.pad_buttons = [self.pads[0] | boot, self.pads[1]];
        sif.pad_sticks = self.sticks;
        ssx3_ps2::machine::run_frame(&mut self.cpu)
    }

    /// The game rate (VSyncs per second, `SSX3_TICK_HZ`: 60 originally).
    pub fn vsync_hz(&self) -> u32 {
        ssx3_ps2::machine::vsync_hz()
    }

    /// Wall time of one VSync (s): 1.001 / rate (59.94 Hz originally).
    pub fn period(&self) -> f64 {
        match self.vsync_hz() {
            60 => 0.016_683,
            hz => 1.001 / hz as f64,
        }
    }

    /// The display's refresh rate (Hz, 60 by default). The pacer follows the
    /// display's cadence when the game rate is a whole multiple or fraction
    /// of it ([`Pacer::lock_to_display`]); returns whether it does.
    pub fn set_display_hz(&mut self, hz: f64) -> bool {
        self.display_hz = hz;
        self.pacer
            .lock_to_display(self.vsync_hz() as f64 / 1.001, hz)
    }

    /// Catch-up (`SSX3_CATCHUP`, off until set): when the host is slower
    /// than the game rate, extra game updates per VSync keep game time and
    /// audio on the wall clock. Turns the machine's real-time mode on with it.
    pub fn set_catch_up(&mut self, on: bool) {
        self.catch_up = on;
        self.pacer.reset();
        tick::set_updates_per_vsync(1);
        ssx3_ps2::machine::set_realtime(on);
    }

    /// The fixed-timestep plan at wall time `now` (s, any fixed origin):
    /// the VSyncs (game updates) the wall clock asks for. A display slower
    /// than the game rate gets several VSyncs per image, a faster one waits
    /// for the next update. Run [`Engine::step`] that many times, then
    /// present.
    pub fn pace(&mut self, now: f64) -> Pace {
        if self.paused {
            return Pace::Paused;
        }
        let period = self.period();
        let game_hz = self.vsync_hz() as f64 / 1.001;
        let max_vsyncs = (game_hz / self.display_hz - 0.01).ceil().max(1.0) as u32;
        let plan = self.pacer.plan(now, period, max_vsyncs, self.catch_up);
        if plan.vsyncs == 0 {
            if self.pacer.repeat_image() {
                return Pace::Repeat;
            }
            return Pace::Wait(self.pacer.wait(now, period));
        }
        ssx3_ps2::machine::set_audio_lead_frames((self.pacer.audio_lead() * 48000.0) as u32);
        tick::set_updates_per_vsync(plan.updates_per_vsync);
        Pace::Run(plan.vsyncs)
    }

    /// [`Engine::pace`] and the VSyncs it asks for, in one call (for a front
    /// end with nothing to do between VSyncs).
    pub fn advance(&mut self, now: f64) -> Advance {
        let (present, wait) = match self.pace(now) {
            Pace::Run(n) => {
                for i in 0..n {
                    if !self.step() {
                        return Advance {
                            vsyncs: i + 1,
                            alive: false,
                            present: true,
                            wait: 0.0,
                        };
                    }
                }
                return Advance {
                    vsyncs: n,
                    alive: true,
                    present: true,
                    wait: 0.0,
                };
            }
            Pace::Repeat => (true, 0.0),
            Pace::Wait(w) => (false, w),
            Pace::Paused => (false, 0.0),
        };
        Advance {
            vsyncs: 0,
            alive: true,
            present,
            wait,
        }
    }

    /// The pacer (its counters, the audio lead it keeps).
    pub fn pacer(&self) -> &Pacer {
        &self.pacer
    }

    // ------------------------------------------------------------ lifecycle

    /// Stop game time: [`Engine::pace`] runs nothing and the audio output
    /// is silenced until [`Engine::resume`]. [`Engine::step`] still runs a
    /// VSync when asked (e.g. to show a settings change behind a menu).
    pub fn pause(&mut self) {
        if !self.paused {
            self.paused = true;
            self.restart_clock();
            self.sync_audio();
        }
    }

    /// Continue after [`Engine::pause`]: the pacer starts over from now (no
    /// burst of VSyncs to catch up the time spent paused).
    pub fn resume(&mut self) {
        if self.paused {
            self.paused = false;
            self.restart_clock();
            self.sync_audio();
        }
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    /// The platform took the audio away (a call, another app's audio focus):
    /// silence the output while the game may keep running.
    pub fn audio_suspend(&mut self) {
        self.audio_interrupted = true;
        self.sync_audio();
    }

    /// The audio is back ([`Engine::audio_suspend`]).
    pub fn audio_resume(&mut self) {
        self.audio_interrupted = false;
        self.sync_audio();
    }

    fn sync_audio(&self) {
        if self.paused || self.audio_interrupted {
            ssx3_iop::audio::suspend();
        } else {
            ssx3_iop::audio::resume();
        }
    }

    fn restart_clock(&mut self) {
        self.pacer.reset();
        tick::set_updates_per_vsync(1);
    }

    /// Flush the sound capture and stop the IOP side (at exit).
    pub fn shutdown(&mut self) {
        ssx3_ps2::iop_bridge::shutdown(&mut self.cpu);
    }

    // ------------------------------------------------------------ image

    /// The front end's surface is `w`×`h` pixels: render at that size (the
    /// aspect box in it, `SSX3_ASPECT`, at `SSX3_RESOLUTION`) and widen the
    /// game's view to its aspect (and `SSX3_FOV`), unless a fixed size
    /// (`SSX3_RENDER_SIZE`) is set. The renderer draws offscreen: a lost
    /// and recreated surface needs nothing more than this call with its size.
    pub fn set_surface_size(&mut self, w: u32, h: u32) {
        self.surface = Some((w, h));
        self.follow_surface();
    }

    fn follow_surface(&self) {
        let Some((w, h)) = self.surface else { return };
        if w == 0 || h == 0 || !self.follows_surface() || !ssx3_ps2::frame_sink::native() {
            return;
        }
        if let Some(g) = self.gfx.as_ref() {
            let (bw, bh) = match self.aspect_box() {
                Some(a) if w as f32 / h as f32 > a => ((h as f32 * a).round() as u32, h),
                Some(a) => (w, (w as f32 / a).round() as u32),
                None => (w, h),
            };
            let k = self.resolution();
            g.set_output_size(Some((
                ((bw as f32 * k).round() as u32).max(64),
                ((bh as f32 * k).round() as u32).max(64),
            )));
            ssx3_ps2::frame_sink::set_view(bw as f32 / bh as f32, self.fov());
        }
    }

    /// A setting changed while running: apply it if it applies at once
    /// (catch-up, the renderer's options and debug knobs, the view).
    /// Settings that apply after a restart are left alone.
    pub fn setting_changed(&mut self, key: &str) {
        match key {
            "SSX3_CATCHUP" => {
                let on = self.settings.get("SSX3_CATCHUP").is_none_or(|v| v != "0");
                self.set_catch_up(on);
            }
            "SSX3_ANISO" => {
                if let Some(g) = self.gfx.as_ref() {
                    g.set_anisotropy(self.anisotropy());
                }
            }
            k if GFX_KEYS.contains(&k) => {
                if let Some(g) = self.gfx.as_ref() {
                    g.set_options(self.gfx_options());
                }
            }
            // The renderer's debug knobs.
            k if ssx3_gfx::DEBUG_KNOBS.contains(&k) => {
                ssx3_gfx::set_debug_knob(k, &self.settings.get(k).unwrap_or_default());
            }
            _ => self.follow_surface(),
        }
    }

    /// Whether the image follows the surface's size ([`Engine::set_surface_size`]).
    pub fn follows_surface(&self) -> bool {
        render::follows_surface(&*self.settings)
    }

    /// `SSX3_ASPECT=4:3|16:9`: the image keeps that aspect inside the
    /// surface (boxed) instead of filling it.
    pub fn aspect_box(&self) -> Option<f32> {
        render::aspect_box(&*self.settings)
    }

    /// `SSX3_FOV`: field of view relative to the game's.
    pub fn fov(&self) -> f32 {
        render::fov(&*self.settings)
    }

    /// `SSX3_RESOLUTION`: the fraction of the surface's pixels rendered.
    pub fn resolution(&self) -> f32 {
        render::resolution(&*self.settings)
    }

    /// `SSX3_ANISO`: anisotropic filtering.
    pub fn anisotropy(&self) -> u16 {
        render::anisotropy(&*self.settings)
    }

    /// The modern graphics options (`SSX3_AA`, `SSX3_LOD_BIAS`, `SSX3_AO`,
    /// `SSX3_SHADOWS`, `SSX3_BLOOM`, `SSX3_SNOW`).
    pub fn gfx_options(&self) -> ssx3_gfx::Options {
        render::gfx_options(&*self.settings)
    }
}
