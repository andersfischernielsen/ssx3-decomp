# Plan: review harness, module layout, modern runtime (2026-10-22)

Owner request (2026-10-22): finish reviewing the hand-written ports with a
broad scenario harness, then reorganise the `ocN.rs` modules, then move the
runtime away from the PS2 machine model. Executed in this order.

## Task 1: scenario harness and review ✅/🟡

Scripted headless scenarios that reach as much game code as possible, each
compared with and without the hand-written ports (and checked with the
runtime shadow mode, `SSX3_SHADOW`), so a port that differs from the
generated body shows up even where the race replay never goes.

Scenarios (`scripts/scenarios/*.txt`, run by `scripts/scenarios.sh`):
- every race course, every freestyle (slopestyle / superpipe / big air)
  course, freeride, and the whole mountain (all three peaks);
- boost during races, tricks (grabs, spins, flips, Ubers);
- the lodge: change clothes, buy tricks;
- random input through the menus ("monkey").

Gate per scenario: RAM at the end identical with `SSX3_SKIP_OVERRIDES` set
to the ports under review vs installed (deterministic: the generated light
query, `SSX3_DEBUG_LENSFX_VIS=1`); a difference is bisected
(`bisect.py`) and the culprit shadowed (`SSX3_SHADOW=addr`) to see the
first differing call. Findings and fixes are logged below.

Tools: `SSX3_SHADOW=addr,...|all` (+ `SSX3_SHADOW_FIRST`,
`SSX3_SHADOW_EVERY`), `SSX3_SKIP_OVERRIDES=addr,...`.

### Findings (ports that differed from the generated body)

| function | found by | difference | fix |
| --- | --- | --- | --- |
| `RiderMetrix_UpdateTrickHUD` 0x117fe0 | freestyle | `PlayTrickPointsTick` got a clobbered `a2` (points) | reload `a2 = s0` before the call |
| `Rider_UpdateVisibility` 0x122278 | monkey | frustum list end skipped one cycle | leave through the list block |
| `Rider_UpdateLegIKBlend` 0x120d90 | monkey | the "towards 0" path also set `f1 = 1.0` / `at` | removed |
| `FXTrail_Reset` 0x392d18 | monkey | missed the store to `+0xc` | added |

## Task 2: module layout (branch `reorg-modules`)

The ~130 `ocN.rs` batch modules become subsystem modules
(`crates/ssx3-game/src/<subsystem>/<unit>.rs`), keyed by the matching C++
decompilation's source path (`tools/refsrc.py`), else by name prefix.
Functions move with their tests and `OVERRIDES` entries; shared helpers go to
a common module; the per-batch cargo features collapse. Gate: all
differential tests pass, the race replay RAM and the scenario RAMs are
unchanged. Merged into `main` when green.

## Task 3: modern runtime

1. **One host thread for all guest threads.** The host-thread form runs each
   guest thread on an OS thread with a baton (condvar handoffs, ~1.3 ms per
   frame on desktop against the single-thread form). Replace with
   cooperative switching on one host thread (stackful contexts, or the
   unwind form's mechanism made cheap), keeping the kernel's scheduling
   decisions identical so replays stay byte-identical.
2. **Service threads as direct calls from a native frame loop.** The DMA
   kick thread is already native. Next: `PS2ExecMan_VSyncThread`
   (`App_VSyncTick` at VBlank), the exec worker, `SND_EeThread` (the sound
   tick), each run by the frame loop at the point the scheduler ran it.
3. **Shrink `Machine`.** Keep what generated code touches (guest RAM, the
   kernel calls, the few status registers); the GS, DMAC, VIF and IPU models
   move behind a verification feature or go.

Each step gated by the race replay RAM and the scenario RAMs.

## Rules

One heavy process at a time (8 GB machine). Commit per step; push `main`
(and the reorg branch). Game data stays out of git.
