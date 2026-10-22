# Plan: review harness, module layout, modern runtime (2026-10-22)

Owner request (2026-10-22): finish reviewing the hand-written ports with a
broad scenario harness, then reorganise the `ocN.rs` modules, then move the
runtime away from the PS2 machine model. Executed in this order.

## Task 1: scenario harness and review ✅

Scripted headless scenarios that reach as much game code as possible, each
compared with and without the hand-written ports (and checked with the
runtime shadow mode, `SSX3_SHADOW`), so a port that differs from the
generated body shows up even where the race replay never goes.

Scenarios (`tools/scenarios.py`, run by `scripts/scenarios.sh`; each run
starts from its own copy of the memory card):
- Single Event: every race and freestyle (slopestyle / superpipe / big air)
  event of the three peaks, ridden with boost, grabs, spins and trick
  buttons;
- Conquer The Mountain: the Transport gate to Peak 3's freeride drop points
  (the top of the mountain down), a Peak 2 race and freestyle event and a
  Peak 1 run; the Lodge (equip another hat, bind another Uber trick, save,
  save progress on leaving); a collectible; a new career on a fresh card
  (top of Peak 1);
- random input through the menus ("monkey").

Gate per scenario: RAM at the end identical with `SSX3_SKIP_OVERRIDES` set
to the ports under review vs installed (deterministic: the generated light
query, `SSX3_DEBUG_LENSFX_VIS=1`); a difference is bisected
(`tools/bisect_overrides.py`) and the culprit shadowed (`SSX3_SHADOW=addr`) to see the
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
| `SSXAudioSystem_PASay20C5` 0x2a3708 (+5 more in oc162) | Big Air / Super Pipe events | indirect calls without `ra = return address`: the callee came back to the old `ra`, the port left without its epilogue | `call_ptr` |
| `BENewPlayerInt_IsMessageFlagSet` 0x147980 (+4 constants in oc131) | fresh career | `li -1` as `0xffff_ffff` (no sign extension): wrong message-flag word | sign-extended |
| `SplineModifier_LoadPath` 0x345570 | The Junction, Schizophrenia, Perpendiculous | `a0 = self` missing before `GetPathLength`: spline length 0 | added |
| `BEMissionInt_IsCollected` 0x1538e8 | CTM Much-2-Much | the two multiplies (profile × 0x9b50, slot × 0xc) swapped | swapped back |
| `GameCamera_ResetAll` 0x15db58 | CTM Peak 3 runs, monkey | `a0 = camera manager` missing before the first call; vtable targets not left in `v0`/`a1` | added |

Unit-test sweep with the call trace (`SSX3_DIFF_STRICT=args`: each call's
callee and arguments, generated vs hand-written; feature `calltrace` of
`ssx3-ee-rt`, on in the tests) found more wrong arguments the scenarios had
not reached:

| function | difference | fix |
| --- | --- | --- |
| `RiderScriptObj_AcceptCommand` 0x280458 | `a0 = rider` missing for `Rider_IsLoaded` / `Rider_GetCharacter` | added |
| `SSXAudioSystem_PASay20C5` 0x2a3708 | `a0` missing for `IsReplayPlaying`, `GetLanguageMask` | added |
| `cFEStateRequestLine_UpdateSongCount` 0x198118 | `a0` missing for 0x3dbca0 | added |
| `FEStateOnlineCharacterSelect_V25` | 4th string 0x4614e8 instead of 0x4638b0 | fixed |
| `LobbyMan_On*Reply` (3) / `NetEventFD/FE_Clone` | wrong allocation name pointer | 0x4804a8 / 0x480488 |
| `__do_global_dtors_aux` | 0x43cd00 instead of 0x43ce00 | fixed |
| `AnimDataBase_SampleChannel`, `WScriptMan_DeactivateAll`, `WScriptMan_CallTableFunction`, `AV_Player_SetRate` | argument registers left differently (`a1`/`a3` delay slots) | as generated |
| constants `0xffff_fffd` etc. without sign extension (oc127, oc131, oc143) | upper word | sign-extended |

Remaining `SSX3_DIFF_STRICT=args` differences: `f13` before `Math_SinCos`
in five camera functions (oc54; the callee does not read it). With
`SSX3_DIFF_STRICT=1` (all scratch registers and the call sequence) about
120 ports differ, almost all leaf functions written as native code (math,
chase, skeleton) or a callee computed natively: not observable.

The full run (2026-10-22, 33 scenarios, every hand-written function on vs
off): 23 identical, 10 different; after the fixes all 33 identical except
the new career's opening movie, where the native libmpeg (`movie.rs`)
differs by design (identical with it skipped). Most of these ports passed
their differential tests because the wrong value only reached a scratch
register or a zeroed fixture; `SSX3_DIFF_STRICT=1` makes the tests compare
the scratch registers too. Further decomp checks use the unit tests (this
harness run is expensive and was done once).

## Task 2: module layout ✅ (2026-10-22)

The 162 `ocN.rs` batch modules are gone: the 4,614 replacements they held
are in `crates/ssx3-game/src/game/<dir>/<unit>.rs` (248 modules), laid out
like the matching C++ decompilation's `src/` tree (`tools/refsrc.py`);
functions the reference has no unit for are placed by name prefix
(`game/lib/` for the EA sound library, libc, the gcc runtime, the kernel).
`tools/reorg_modules.py` did the move: each replacement and its tests to its
unit, helpers to the units that use them (one copy, in the module's main
unit, `pub(crate)`, imported by the others; name clashes renamed), the
per-batch cargo features removed. New waves are still written as staging
`ocN.rs` modules and merged in with the same tool (AGENTS.md). Gate: all
4,979 differential tests pass, 5,092 replacements installed (as before), the
race replay RAM identical.

## Task 3: modern runtime (branch `modern-runtime`)

1. **One host thread for all guest threads.** ✅ The host-thread form ran
   each guest thread on an OS thread with a baton (condvar handoffs). Guest
   threads are now stackful coroutines (`corosensei`) on the frame-driving
   host thread; a thread switch is a stack switch, the kernel's scheduling
   decisions are unchanged and the replays byte-identical (system time of a
   6000-frame race 7.1 s -> 5.7 s). Making the unwind form (one host thread
   by construction) the desktop default was tried first: it needed the
   unwind form to charge cycles exactly as the host-thread form (done:
   `recomp-ee --unwind` now charges a call's whole block before the call
   and an extra entry's block on entry, as the structured form and its
   `__entries` twin do; the browser replay stays identical), but 34 unit
   tests still differed at thread-switch points (structured-loop ports
   defer a switch the unwind code takes at once), so it stays the browser
   form.
2. **Service threads as direct calls from a native frame loop.** ✅ (as far
   as it is sound) No OS service threads remain: every guest thread is a
   coroutine on the frame thread, and the DMA kick thread is native
   (`kick.rs`). `SND_EeThread`, the exec worker and the CD threads block in
   the middle of an iteration (a lock another thread may hold, arbitrary
   job functions, CD waits), which a direct call from the signalling
   context cannot reproduce; with coroutines there is no switch cost left
   to save, so they stay guest threads.
3. **Shrink `Machine`.** ✅ The native race touches (`SSX3_MMIO_PROFILE=1`):
   the GS privileged registers and CSR (the game polls CSR ~120 times a
   frame; SIGNAL/FINISH/LABEL come from its GIF packets), the scratchpad
   DMA channels (game code moves data through SPR), the GIF/VIF1 channels
   and VIF1_STAT, timer 1 and VU0 memory. So the DMAC, the timers, VU0/VU1
   and the VIF model (the frame build replays through it) stay. The GS is
   reduced to its guest-visible part (`gs::Mtgs`: privileged registers,
   CSR events, GIF tag tracking, PATH3 arbitration): the GS core (VRAM,
   rasterizers, GPU backend, captures; `gs/core.rs` and its modules) is
   compiled only with the `emu-gs` feature (verification builds,
   `--features vu1-microcode`), and in such builds detaching it also stops
   the GS thread. The shared wgpu device moved to `ssx3_ps2::gpu`. The IPU
   model is small and runs only with `SSX3_NATIVE_MPEG=0`.

Each step gated by the race replay RAM and the scenario RAMs.

## Rules

One heavy process at a time (8 GB machine). Commit per step; push `main`
(and the reorg branch). Game data stays out of git.
