# Configurable tick rate (no interpolation)

Goal: the game's base update rate is a setting (60, 120, 144, 240 Hz, …).
Every tick is 1/N seconds of game time; the renderer shows each tick's frame,
so no interpolation is needed. Game speed, physics, timers, animation and
audio cues are the same at every rate. At 60 Hz the game stays exactly the
original (the race RAM gate and the differential tests keep working).

Status: investigation done (2026-10-22), worker waves not started.

## How the game keeps time

- `cAppMan_run` (0x316d60) sets the app manager's rate and step: `+0x10 = 60`,
  `+0x14 = 1/60` (`D_004A5B64` points to the app manager), and passes 60 to
  the execution manager (vtable slot 2).
- `App_VSyncTick` (0x317348) runs on every VSync: `accum += step` (`+0x24`,
  1.0) and queues one game tick per whole unit.
- `cAppMan_mainLoop` (0x316f00) runs the queued ticks (at most `+0x20` = 12
  to catch up), then renders once.

So the game is a fixed-tick loop with a rate setting, but most code ignores the
setting and assumes 60 Hz.

## Runtime knob

`SSX3_TICK_HZ=N` (`ssx3_ps2::machine::tick_hz`) fires VSync at N Hz and
overwrites `+0x10`/`+0x14` with N and 1/N after `cAppMan_run` writes them.
Unset, nothing changes (gate identical). It is an experiment switch: until
the work below is done, everything that assumes 60 Hz runs N/60 times too
fast. Headless frame numbers count VSyncs, so scale `SSX3_INPUT` frame numbers
and durations by N/60.

## Measured at 120 Hz (race replay, input scaled ×2)

`[tick]` log: one game update per VSync (11918 updates in 12000 VSyncs; the
gap is loading). Screenshots at matched real time, 60 Hz frame F vs 120 Hz
frame 2F (scratch run, not kept):

| | 60 Hz | 120 Hz | Verdict |
| --- | --- | --- | --- |
| Race clock, 44 s of real time after the start | 0:42 | 1:28 | 2× fast: the clock counts ticks × 1/60 |
| Course progress, same instant | 22% | 42% | 2× fast: rider motion is per tick (velocity per frame) |
| Speed (MPH HUD) | 41-59 | 37-70 | about the same: a per-tick quantity, not distance per second |
| Menus, title, load screens | | | run, no hang; front-end animations not yet measured |

Everything gameplay-related ran at double speed. Code that reads the app
manager's rate (world objects, see below) is the only part that adapts.

## Inventory

`python3 tools/scan_framerate.py --tsv docs/plans/tick-rate-functions.tsv`
scans the executable and lists every function with a frame-rate constant,
its source unit and whether it is already hand-written. Current result
([`tick-rate-functions.tsv`](tick-rate-functions.tsv)):

| Kind | Functions | Hand-written |
| --- | --- | --- |
| Hard-coded `1/60`, `60.0`, `3600` | 129 | 41 |
| `1/30`, `30.0` (mostly 30 fps animation data; some unrelated) | 35 | 8 |
| `1/50`, `50.0` (mostly not timing) | 15 | 6 |
| Read the app manager's rate or step (already adapt) | 29 | 14 |

Largest units: `ai/ai` (15), `seg_1218` (13, the 12 `RiderAnim_*Seq*` functions),
`camera/camera` (11), `visualfx/boardwakefx` (11), `handplantcontrol` (6),
`seg_14B10` (5), `ai/rider` (5), `fe/fepopupmisc` (5).

What the scan cannot see (found by running at 120 Hz and looking for things
that run fast):

- velocities and accelerations stored per tick (`Rider_AddVelocity` adds
  velocity straight onto position; gravity, friction);
- per-tick decay and smoothing (`v *= k`, `x += (t - x) * a`);
- frame counting: 32 calls of `AI_GetFrameCounter` (e.g. `Rider_BeginFrame`
  gathers overlaps every third frame), integer timers in frames, scripter and
  camera-script durations in frames, animation frame advance;
- random chances per frame.

## Design for the conversion

Only hand-written code is converted. A function in the list that is still
generated is first ported in the normal porting loop (exact to the generated
body), then converted.

1. **One source of truth.** `ssx3_game::tick` (to be written): `rate()` (u32)
   and `dt()` (f32), read from the app manager (`+0x10`, `+0x14`). Do not add
   a second copy of the rate.
2. **Exact at 60.** Every converted expression must give the original bits at
   60 Hz, so the RAM gate and the differential tests still hold:
   - `x * (1/60)` → `x * dt()`: exact (`dt()` is the same float).
   - frame counts: `n * (1/60)` → `n * dt()`; thresholds in frames
     `t >= F` → `t >= F * rate() / 60` (integer, exact at 60).
   - per-tick velocity/acceleration: scale by `rate60 = 60.0 / rate`
     applied as `v * dt() * 60.0`… only where that is exact at 60; otherwise
     keep the original expression on a `rate() == 60` branch.
   - per-tick decay `v *= k` → `v *= k.powf(60 / rate)` on the non-60
     branch only (`powf` is not exact).
   - "every third frame" (load spreading, not timing): keep in ticks unless
     it changes behaviour; note it.
3. **Data in frames** (animation sequences, scripter `tIn/tHold/tOut`, camera
   scripts, trick windows): convert where they are consumed, not in the data.
4. **Physics step.** If collision/landing behaves differently at small steps
   (missed rail catches, different jump heights), sub-step at the original
   60 Hz step for that code instead of changing its integration.

## Work order (by what a player notices)

1. Race clock, timers, countdowns, HUD times (exact, easy).
2. Rider motion: velocity/position integration, gravity, boost, speed caps.
3. Rider animation (`RiderAnim_*Seq*`), board lean, IK blends.
4. Camera (`camera/camera`, chase/script controllers, triggers).
5. AI riders (`ai/ai`, `ai/computer`, air predictor, hand plants).
6. Effects (board wake/trail/spark, avalanche, particles, effect links).
7. Front end, scripter, sound cues.

## Verification

- At 60 Hz: the existing differential tests (`SSX3_DIFF_STRICT=args`) and the
  race gate, unchanged.
- At N Hz, per function: a unit test that runs the hand-written function
  `N/60` times at rate N and once at 60 from the same state and compares
  within a tolerance (exact for timers and counters).
- At N Hz, end to end: the race replay with `SSX3_TICK_HZ=120` and input
  scaled ×2; the race clock and course progress must match the 60 Hz run at
  matched real time (the table above is the starting point: 2× → 1×).

## Worker brief

Writers follow `docs/investigation/phase2-worker-guide.md` and the AGENTS.md
subagent rules (one module file, no cargo, tests per function). Per wave:
up to 50 functions from `tick-rate-functions.tsv`, one unit or system at a
time (work order above). For each function: port it if it is not
hand-written, convert it as in "Design", add the 60 Hz exactness test (the
existing differential test) and the N Hz test. Report every per-tick quantity
found that the scan did not list.
