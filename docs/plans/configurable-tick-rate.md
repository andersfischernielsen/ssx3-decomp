# Configurable tick rate (no interpolation)

Goal: the game's base update rate is a setting (60, 120, 144, 240 Hz).
Every tick is 1/N seconds of game time; the renderer shows each tick's frame,
so no interpolation is needed. Game speed, physics, timers, animation and
audio cues are the same at every rate. At 60 Hz the game stays exactly the
original (the race RAM gate and the differential tests keep working).

Status (2026-10-22):

- ✅ Rate switch: `ssx3_game::tick`, `SSX3_TICK_HZ`, the settings
  overlay's "Game rate" (F10, applies after a restart).
- ✅ Every `1/60` the game loads, and the app manager's rate and step, follow
  the rate: at 120 Hz the race clock and course progress match 60 Hz.
- ✅ A slow host keeps game time real time (catch-up, below); interpolation
  is removed (the game rate replaces it).
- ✅ T1: the 193 `1/60` entries classified (187 STEP, 6 NOT_A_STEP, 2 UNSURE);
  the script/animation frame-time entries no longer follow the rate.
- ⏳ T2: the other rate constants (`60.0`, `3600`, `1/30`, `30.0`, `1/50`,
  `50.0`).
- ⏳ T3: the per-tick code the scan cannot see (front end first, see "Fast at
  240 Hz").

## How the game keeps time

- `cAppMan_run` (0x316d60) sets the app manager's rate and step: `+0x10 = 60`,
  `+0x14 = 1/60` (`D_004A5B64` points to the app manager), and passes 60 to
  the execution manager (vtable slot 2; what it does with it is not yet
  known, nothing visible depends on it).
- `App_VSyncTick` (0x317348) runs on every VSync: `accum += step` (`+0x24`,
  1.0) and queues one game tick per whole unit.
- `cAppMan_mainLoop` (0x316f00) runs the queued ticks (at most `+0x20` = 12
  to catch up), then renders once.

So the game is a fixed-tick loop with a rate setting, but most code ignores the
setting and loads `1/60` itself.

## The rate switch

`ssx3_game::tick` (`crates/ssx3-game/src/tick.rs`) is the one place the rate
lives:

- `tick::install()` reads `SSX3_TICK_HZ` (the settings file puts the
  overlay's choice there). The front end (`crates/ssx3/src/game.rs`) passes
  the rate to `ssx3_ps2::machine::set_vsync_hz` (VSync at N × 1000/1001 Hz;
  60 is the original 59.94 Hz) and installs `tick::on_vblank` as the
  machine's VSync hook.
- `tick::on_vblank`, at each VSync, once the app manager exists: writes N
  and 1/N into `+0x10`/`+0x14`, and 1/N into every `.lit4` entry holding
  `1/60` (193 entries; [`tick::NOT_A_STEP`] lists entries to leave alone).
  At 60 it writes nothing.
- Hand-written code reads `tick::rate(c)` / `tick::dt(c)` (from the app
  manager; 60 and 1/60 in unit tests) and `tick::ticks_from_frames60(c, n)`.
  Unit tests at another rate call `tick::set_for_test(c, hz)`.
- The window paces one tick per `1.001 / N` s and draws in-between images
  only when the display is faster than the game rate.

Why the `.lit4` patch works: the compiler gives every `1/60` use its own
gp-relative `.lit4` entry (193 entries, one or two loads each, no `1/60` built
in a register anywhere). Generated code and the hand-written ports both load
the entry from memory (ports keep the `lwc1` from `gp-N`), so rewriting the
entries changes every such use at once. One port had inlined the constant
(`math.rs`, the loading indicator); it now reads its entry.

## Frame pacing: the game runs at its rate, whatever the display or host

The window is a fixed-timestep loop (`ssx3_game::tick::Pacer`), as modern
games are:

- The game advances one VSync per update period of wall time (1.001 / rate
  s). Each VSync runs the game's update and render, so game time and the
  emulated clock (loading, streaming, timers) stay real time.
- A display slower than the game rate gets several VSyncs per image (at most
  game rate / display rate, rounded up); a faster display waits for the next
  update and shows each game image until then. The display rate never
  changes the game's speed. Measured on a 60 Hz display at a 120 Hz game
  rate: 119 updates, 118 VSyncs and 290 M EE cycles (294.9 M nominal) per
  second.
- A host too slow to run that many VSyncs: with catch-up (`SSX3_CATCHUP`,
  settings "Keep speed when slow", on by default) the extra updates are
  queued per VSync instead, through the game's own frame-drop path
  (`App_VSyncTick` queues `+0x24` updates per VSync, 1.0; the main loop runs
  them, then renders once), at most 4 per VSync; beyond that, or with
  catch-up off, the game slows down rather than spiralling. A debt or lead
  over 0.25 s (pause, overlay, window drag) is forgotten.
- There are no in-between images: the game rate is the frame rate.
  Rendering faster than the game rate would only repeat images.
- Audio (`snd_native::service`, `machine::set_realtime`): the mixer ticks on
  the EE clock as before, and when the device's queue is under the lead it
  runs an extra tick and moves the EE-clock schedule along. As on the
  console, where the IOP keeps its own time, sound no longer depends on how
  fast the guest runs. Two details matter:
  - An extra tick waits for the EE's next tag buffer (`note_dma` marks it
    fresh). Each buffer carries the next 10.7 ms of the music the EE mixes;
    ticking twice on one buffer replays it (measured: 67% of 10.7 ms blocks
    repeated their predecessor, heard as a metallic sound; with the check
    0.2%, as at normal speed).
  - The lead adapts (`CatchUp::audio_lead`: 2.5 × the longest recent frame,
    60-180 ms), because the guest, which mixes, does not run while the host
    draws or presents a frame. With a fixed 60 ms a slow frame (2×
    resolution) emptied the device: choppy sound. The window logs the gaps
    (`audio gaps ... lead ... ms`); `SSX3_AUDIO=null` is a silent device
    drained in real time for headless tests.
- Replays, headless runs and the gate never use it (`SSX3_CATCHUP=1` turns
  catch-up on in `ssx3-headless` for tests, one VSync per frame; with `SSX3_SLOW_FRAME_MS` it simulates a
  slow host). This replaces the old `SSX3_HOST_CLOCK`, which skipped EE
  cycles and starved loading.

Measured, race replay with 20 ms added per frame (a ~35 fps host), 60 Hz:

| | catch-up off | catch-up on |
| --- | --- | --- |
| Game updates after 169 s of wall time | (one per VSync: ~5900) | 10084 (real time: 10148) |
| Race clock over 2000 frames (~60 s of wall time) | 0:01 → 0:34 (slow motion) | 0:04 → 1:05 (real time) |
| Audio, a 19 fps host (50 ms added per frame), 178 s | (follows the game: slow) | 177 s of audio, no gaps, lead ~150 ms |

Not covered: the browser (its page drains audio every frame; its frame loop
is the worker's `requestAnimationFrame`, catch-up to be wired there).

Interpolation (in-between images from blended matrices, `SSX3_FPS`) is
removed: the game rate gives one real image per update, and a display faster
than the game rate shows each image until the next. The 60 Hz screenshots
are byte-identical to before.

## Rider physics: scale, no sub-stepping

Rider velocity is stored in units per second (`Rider +0x1e0`, see
`rider_logic.rs`) and integrated with the loaded `1/60` step (ground state,
air state, wipeout, rail, hand-plant updates all load it from `.lit4`; the
air predictor also steps at the `1/60` entry). With the entries at 1/N the
integration runs at the smaller step: no sub-stepping at 60 Hz is needed.

Measured, the race replay at 120 Hz against 60 Hz at matched real time
(`tools/tick_ab.py`):

| | 60 Hz | 120 Hz, before | 120 Hz, entries patched |
| --- | --- | --- | --- |
| Race clock | 0:01 0:05 0:09 0:17 0:25 0:34 0:42 | 2× fast (0:42 vs 1:28) | 0:02 0:06 0:10 0:18 0:27 0:35 0:44 |
| Course progress | 0 2 4 8 13 18 22 % | 2× fast (22 vs 42 %) | 1 3 6 10 15 20 23 % |
| Speed (MPH) | 55 59 58 55 48 7 | similar | 55 55 57 59 50 63 |
| Rail tricks, landings | BS Rail, 50/50 Rail | | BS Rail landed |

The 120 Hz run starts its race about a second earlier (menu timing), hence
the constant offset. The paths differ (a different step is a different
simulation), the rates agree. Drag and friction look step-correct (the
speeds match); a per-tick factor there would show as a lower top speed.

Keep watching for: rail catches and landings decided by a per-tick distance
threshold, jump heights (semi-implicit Euler at a smaller step gives slightly
different arcs), wall collisions. If one of them changes behaviour at N Hz,
the fix is local to that check (scale the threshold), not a sub-stepped
physics loop.

## Fast at 240 Hz (baseline, before T1/T2/T3)

`python3 tools/tick_ab.py 240` on the 2026-10-23 baseline (two headless runs
of the same race input, 60 Hz left and 240 Hz right at matched real time;
sheets in `/tmp/ssx3-tick-ab-240/sheet*.png`). The 240 Hz run is still in the
front end when the 60 Hz run reaches the race, so each row also tells where
the two runs are in the sequence.

| System | What is wrong at 240 Hz | Evidence (sheet row) |
| --- | --- | --- |
| Front end, attract loop | ~4× fast: the title/attract cycle reaches the attract-mode demo (rider, LIVE overlay) at 16.7 s of real time while 60 Hz is still on the first "Press START" image | `sheet0` rows 1-3 (60 Hz frames 1000/1250/1500 vs 240 Hz 4000/5000/6000) |
| Front end, screen transitions | ~4× slow: title→main menu and every menu step take 4× the real time, so at the same instant the 240 Hz run is one screen behind; the race starts ~2 s later and that offset then stays constant | `sheet1` rows 2-4, `sheet2`, `sheet3` row 3 (clock 00:00:03 vs 00:00:01) |
| Race clock, course % | right: agree with 60 Hz up to the start offset (22 % both at the last row, 55 MPH both) | `sheet3`-`sheet6` |
| Rider physics, speed | right: speeds and progress match (55 MPH both; drag/friction follow the step) | `sheet4`-`sheet5` |
| HUD and effects | not separable yet: the front-end offset puts the two runs at different points of the course; re-check after the front end is fixed | — |

So the first T3 target is the front end (menus, HUD, attract loop): its
timers are counted in updates or frames, and its transitions/fades are
per-update increments. The race screen proper already follows the rate.

## Inventory

`python3 tools/scan_framerate.py --tsv docs/plans/tick-rate-functions.tsv`
lists every function with a rate constant: unit, whether hand-written, the
instruction addresses (`sites`) and, for loads, the `.lit4` entry each loads
(`pool`: `site=entry`). Current result:

| Kind | Functions | Hand-written | State |
| --- | --- | --- | --- |
| `1/60` loads (193 entries) | 129 (with `60.0`/`3600`) | 41 | follow the rate; classify (T1) |
| `60.0` (27 sites, 24 built in registers), `3600` (17) | in the above | | classify, convert (T2) |
| `1/30`, `30.0` (mostly 30 fps animation data) | 35 | 8 | classify (T2) |
| `1/50`, `50.0` (mostly not timing) | 15 | 6 | classify (T2) |
| App manager rate or step reads | 29 | 14 | follow the rate |

Not found by the scan (T3):

- frame counting: 32 calls of `AI_GetFrameCounter` (e.g. `Rider_BeginFrame`
  gathers overlaps every third frame), integer timers counted in ticks,
  thresholds in frames, scripter and camera-script durations in frames;
- per-tick factors without the step: `v *= k`, `x += (t - x) * a`, `+= c`;
- random chances per tick.

### T1: the 193 `1/60` entries, classified

Four workers read every load site (hand-written port or `generated/ee`, plus
the C++ reference) and wrote one row per entry to `docs/plans/tick/t1-{a,b,c,d}.tsv`
(`entry site function class reason`). Result: **187 STEP, 6 NOT_A_STEP, 2
UNSURE**. Every entry is loaded by exactly one function, so the split is a
per-site decision.

The six `NOT_A_STEP` entries are script/animation data converted with the
fixed 60 Hz authoring unit, now listed in `tick::NOT_A_STEP`:

| entries | use |
| --- | --- |
| `0x49b860`, `0x49b868` | `Rider_ApplyScriptTrack`: script track time/start frame → seconds for `AnimController_SampleAnimPose` / `PlayAnimDirect` |
| `0x49e8ec`, `0x49e8f0` | `ScriptList_Start`: keyframe counts → the transition duration |
| `0x49e8f4`, `0x49e8f8` | `ScriptList_StartTransition`: script shorts → transition start/duration |

Why they stay `1/60`: the transition manager advances its fade by
`1.0 / rate` per tick (`cTransitionMan_Update`, 0x2e46c8) and compares it with
the duration, so the duration is a wall-clock time; the script timeline is
data authored at a fixed 60 frames/s (T3 makes the script engine advance
`N/60` frames per tick, which keeps the authoring unit fixed).

Two entries are `UNSURE` and stay STEP (they behave the same either way): a
DSock channel virtual getter that returns `1/60` with no resolvable caller
(`0x49e694`), and a `DebugMenu_ctor` store to a write-only global
(`0x49d5d0`).

T1 also found one constant the scan misses: `0x49bf24` holds `59.999996`
(the reciprocal of the `1/60` bits) in `RiderAirState_Update`, alongside the
`1/60` guard at `0x49bf20`; it is a T2 item (the air-turn fraction's
fallback), not a `.lit4` step.


## Conversion rules (exact at 60)

1. Read the rate from `crate::tick` only (`rate`, `dt`,
   `ticks_from_frames60`). No second copy, no `1.0 / 60.0` literal.
2. At 60 the converted code must give the original bits, so the existing
   differential test against the generated body still passes unchanged:
   - a step already loaded from `.lit4`: nothing to do (it follows);
   - `x * 60.0` meaning "per second → per tick": `x * dt * 60` is not exact;
     write `if tick::rate(c) == 60 { original } else { converted }`;
   - frames: thresholds `t >= F` → `t >= tick::ticks_from_frames60(c, F)`
     (exact at 60); `n % 3` load spreading stays in ticks (not timing);
   - per-tick decay `v *= k` → on the non-60 branch `v *= k.powf(60.0 / r)`;
     smoothing `x += (t - x) * a` → `a' = 1 - (1 - a).powf(60.0 / r)`;
   - per-tick increments without a step (`+= c`) → `+= c * 60 / r` on the
     non-60 branch.
3. A function that is still generated is first ported (the normal porting
   loop, exact to the generated body), then converted.
4. Each converted function gets a rate test next to its differential test:
   from the same state, run it `r/60` times at rate r (`set_for_test`, its
   `.lit4` entries at `step_bits(r)`) and once at 60, and compare the
   timing-relevant outputs (exact for counters, a tolerance for floats).

## Verification

- At 60 Hz: the differential tests (`SSX3_DIFF_STRICT=args`) and the race
  gate, unchanged.
- At N Hz: the rate tests per function, and `python3 tools/tick_ab.py 120`
  (two headless runs, side-by-side sheets): race clock and course % agree,
  nothing visibly fast or slow.

## Work order (by what a player notices)

1. T1: classify the 193 `1/60` entries (no code; `NOT_A_STEP` entries).
2. T2: `60.0`, `3600`, `1/30`, `30.0`, `1/50`, `50.0`: classify, convert the
   rate-related ones.
3. T3, by system: race and HUD timers, rider (states, animation, boost),
   camera, AI riders (`ai/ai`, `ai/computer`, air predictor), effects (board
   wake/trail/spark, particles, effect links), front end and scripter, sound
   cues.
