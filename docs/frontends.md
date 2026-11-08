# Front ends

The game is a library, `crates/ssx3-engine`. A front end is a small
program around it: it creates a surface, feeds input, steps the game on its
clock, presents the image, and tells the engine about its lifecycle. This
repository has three:

| Front end | Where | What it adds |
| --- | --- | --- |
| Window | `crates/ssx3/src/main.rs` (`ssx3`) | winit window, `wgpu` presenter, keyboard and gilrs controllers, the settings file and the F10 overlay, F11 fullscreen, F12 screenshots. |
| Headless | `crates/ssx3/src/bin/ssx3_headless.rs` (`ssx3-headless`) | No window: N frames, scripted input (`SSX3_INPUT`), screenshots, RAM dumps, the verification and debug reports. The RAM gate runs it. |
| Browser | `crates/ssx3-web` | WebAssembly in a Web Worker: the disc from a `File`, an `OffscreenCanvas`, VU helper workers, audio pulled for the page. |

A front end for another platform (an Android or iOS app, a console
homebrew shell, a frame server) lives in its own repository and uses this
one as a git submodule (below). Nothing platform-specific beyond the desktop
and the browser lives here.

## The engine API

```rust
use ssx3_engine::{Audio, Engine, EngineConfig, EnvSettings, Pace, Settings};

// Optional, first: data and save roots, a standalone pack. Call it before
// reading a settings file kept with the saves; `Engine::boot` calls it
// otherwise (later calls do nothing).
pub fn init_paths(data_root: Option<PathBuf>, save_root: Option<PathBuf>, find_pack: bool) -> bool;

pub trait Settings: Send + Sync { fn get(&self, key: &str) -> Option<String>; }
pub struct EnvSettings;                                             // the process environment
impl<F: Fn(&str) -> Option<String> + Send + Sync> Settings for F {} // a closure or fn

pub struct EngineConfig {
    pub data_root: Option<PathBuf>, // disc image, orig/, texture packs
    pub save_root: Option<PathBuf>, // memory card, settings, dumps
    pub find_pack: bool,            // standalone pack (tools/bundle.py)
    pub skip_movies: bool,          // default; SSX3_SKIP_MOVIES overrides
    pub audio: Audio,               // Device (default) | Off | Pull; SSX3_AUDIO overrides
    pub settings: Box<dyn Settings>,
}
impl EngineConfig { pub fn new(settings: Box<dyn Settings>) -> Self; }

impl Engine {
    pub fn boot(config: EngineConfig) -> Engine;

    // input (applied at the next step)
    pub fn set_pad(&mut self, port: usize, buttons: u16);
    pub fn set_sticks(&mut self, port: usize, sticks: [u8; 4]);

    // stepping and pacing
    pub fn step(&mut self) -> bool;                // one VSync; false: halted
    pub fn pace(&mut self, now: f64) -> Pace;      // Run(n) | Repeat | Wait(s) | Paused
    pub fn advance(&mut self, now: f64) -> Advance; // pace + the steps it asks for
    pub fn set_display_hz(&mut self, hz: f64) -> bool;
    pub fn set_catch_up(&mut self, on: bool);
    pub fn vsync_hz(&self) -> u32;
    pub fn period(&self) -> f64;
    pub fn pacer(&self) -> &ssx3_game::tick::Pacer;

    // lifecycle
    pub fn pause(&mut self);
    pub fn resume(&mut self);
    pub fn paused(&self) -> bool;
    pub fn audio_suspend(&mut self);
    pub fn audio_resume(&mut self);
    pub fn audio_pull(&mut self, out: &mut Vec<i16>) -> usize; // Audio::Pull
    pub fn shutdown(&mut self);

    // image
    pub fn renderer(&self) -> Option<&ssx3_gfx::Renderer>;
    pub fn native_image(&self) -> bool;
    pub fn read_image(&self) -> Option<(u32, u32, Vec<u32>)>;
    pub fn set_surface_size(&mut self, w: u32, h: u32);
    pub fn setting_changed(&mut self, key: &str);
    pub fn follows_surface(&self) -> bool;
    pub fn aspect_box(&self) -> Option<f32>;

    // diagnostics
    pub fn frames(&self) -> u64;
    pub fn status(&self) -> String;
    pub fn dump(&self) -> String;
    pub fn cpu(&self) -> &ssx3_ps2::Cpu;
    pub fn cpu_mut(&mut self) -> &mut ssx3_ps2::Cpu;
}
```

The engine re-exports the crates a front end touches (`ssx3_ps2`,
`ssx3_gfx`, `ssx3_iop`, `ssx3_game`, `ssx3_core`), so a front end depends on
`ssx3-engine` alone and gets the same versions.

### Features

| Feature | Meaning |
| --- | --- |
| `ee-threads` (default) | The generated EE code with host coroutines (`generated/ee`). |
| `ee-unwind` | The single-threaded unwind form (`generated/ee-unwind`; the browser needs it). Build with `default-features = false`. |
| `emu-gs`, `vu1-microcode` | The emulated GS and VU1 (verification renderers, `SSX3_RENDER=emu/shadow`). |
| `callprof` | The call-graph profiler (`SSX3_CALLPROF`). |

## What a front end does

1. **Paths and settings.** Call `init_paths` if the game data or the saves
   are not in the checkout (an app's storage). Give the engine a `Settings`:
   the window reads its settings file with the environment over it, the
   headless runner reads the environment (`EnvSettings`), the browser the
   page's values (`SSX3_*` keys of the URL query). Keys are the `SSX3_*`
   names in `options.md`, every one of them: paths (`SSX3_ISO`,
   `SSX3_MEMCARD`, `SSX3_TEX_REPLACE`, ...), sound, the game rate, the
   renderer and the debug switches. `Engine::boot` installs the `Settings`
   as the provider of `ssx3_core::config`, which every crate reads its
   options through: the process environment first, then the front end's
   settings, then process defaults (`config::set_default`; a standalone
   build's save folder). Empty values from the settings count as unset. The
   image settings are read through it whenever they apply; most others
   once, at boot or when first needed.
2. **Boot.** `Engine::boot(config)` creates the machine, loads the disc and
   the executable, installs the hand-written functions and the renderer
   (`SSX3_RENDER`). The GPU context is created on demand; a front end that
   must create it asynchronously (the browser) calls
   `ssx3_ps2::gpu::init_context().await` first and creates its surface on
   that instance.
3. **Input.** `set_pad(port, buttons)`: libpad bits, pressed = 1 (bit 0
   Select, 1 L3, 2 R3, 3 Start, 4 up, 5 right, 6 down, 7 left, 8 L2, 9 R2,
   10 L1, 11 R1, 12 Triangle, 13 Circle, 14 Cross, 15 Square).
   `set_sticks(port, [rx, ry, lx, ly])`: 0 left/up, 0x80 centre, 0xFF
   right/down. Port 1 is the second player (`SSX3_PAD2=1` plugs it in).
4. **Step.** Every display refresh, `advance(now)` (or `pace(now)`, then
   `step()` that many times, when the front end polls input or logs
   between VSyncs, as the window does). `now` is wall time in seconds from
   any fixed origin; the engine has no clock of its own (it must also run
   where `std::time::Instant` does not). The pacer runs the game on the wall
   clock at the game rate whatever the display does; call
   `set_display_hz` when the refresh rate is known so it can lock to the
   display's cadence. `set_catch_up(true)` (the window's default) lets a
   slow host queue extra game updates per VSync and keeps the audio device
   fed. A replay or test runs `step()` alone: one VSync per call,
   deterministic.
5. **Present.** The renderer draws offscreen. `renderer().present_view()`
   gives the latest image as a texture view on the GPU context's device
   (no copy); `read_image()` gives it as pixels. Blit it to the front end's
   own surface (the window's presenter, the browser's canvas blit). Call
   `set_surface_size(w, h)` at start and on every resize: the engine renders
   at that size (the `SSX3_ASPECT` box in it, at `SSX3_RESOLUTION`) and
   widens the game's view to its aspect.
6. **Lifecycle.**
   - `pause()` / `resume()`: hidden, backgrounded, a menu over the game.
     Paused, `pace` runs nothing and the audio output is silent; `resume`
     starts the pacer over, so there is no burst of catch-up VSyncs.
     `step()` still runs a VSync when asked (the window shows a settings
     change behind its overlay that way). The browser pauses while the page
     is hidden (`visibilitychange`, `pagehide`/`pageshow`): the page tells
     the worker, which pauses the engine and stops its frame loop, and
     suspends its `AudioContext`.
   - `audio_suspend()` / `audio_resume()`: the platform took the audio away
     while the game may keep running. The output device stays open, plays
     silence and drops what was queued.
   - Surface lost and recreated (a mobile app in the background): the
     engine holds nothing tied to the surface. Recreate the surface on the
     same GPU context and call `set_surface_size` with its size.
   - `setting_changed(key)` after the front end changes a setting that
     applies at once (catch-up, the renderer's options and debug knobs, the
     view); the others apply at the next start.
   - `shutdown()` at exit (flushes a `SSX3_WAV` capture).

### Audio

`EngineConfig::audio` chooses who plays the sound (48 kHz stereo i16):

| `Audio` | Output |
| --- | --- |
| `Device` (default) | The engine opens the default output device itself (cpal). |
| `Off` | No output (the headless runner). |
| `Pull` | The front end owns the device: `engine.audio_pull(&mut Vec<i16>)` appends the samples mixed since the last call (interleaved stereo; returns the frames). From an audio callback on another thread, `ssx3_iop::audio::pull` / `pull_into(&mut [i16])` do the same without the engine. The browser uses it: the worker pulls once per presented frame and the page plays the samples with an AudioWorklet. |

The game is not in lock step with the sound card: at most ~200 ms is
queued, newer samples are dropped while it is full, and the front end
plays silence when it runs dry. Paused or `audio_suspend`ed, the queue is
dropped and nothing is pulled. The `SSX3_AUDIO` setting overrides the
choice (`0` off, `pull`, `null` a silent device drained in real time for
tests, any other value the device); `SSX3_WAV=file.wav` records the sound
whatever the output.

## Using this repository from another one

The generated code is made locally from your own disc (`tools/setup.py`)
and is never committed, so a front end in another repository cannot depend
on a published crate. Use this repository as a git submodule:

```sh
git submodule add https://github.com/andersfischernielsen/ssx3-decomp ssx3
cd ssx3
cp /path/to/your/image.iso "SSX 3 (USA).iso"
python3 tools/setup.py          # orig/, generated/ (ee, iop, vu)
# the unwind form, only for single-threaded targets:
# cargo run --release -p ssx3-recomp --bin recomp-ee -- --unwind \
#     --out generated/ee-unwind --package-prefix ssx3-ee-unwind
```

Then depend on the engine by path from the front end's own workspace:

```toml
[dependencies]
ssx3-engine = { path = "ssx3/crates/ssx3-engine" }

# Cargo takes profiles from the root workspace only: the generated crates
# need an optimised build without LTO (as this repository's `game` profile).
[profile.game]
inherits = "release"
lto = false
codegen-units = 64
```

and build with `--profile game`. Point the engine at where the app keeps
the game data and the saves with `EngineConfig::data_root` / `save_root`
(or `init_paths`); without them it uses the checkout. Update the submodule
and run `tools/setup.py` again when the recompiler changes.

Keep the front end on the public API above. A change that needs more from
the engine (a hook, a setting, an image format) belongs in `ssx3-engine`
here, so every front end gets it and the replay gate covers it.
