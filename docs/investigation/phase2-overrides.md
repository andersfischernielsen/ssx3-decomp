# Phase 2: replacing generated functions with hand-written Rust

_Started 2026-10-15._ Phase 2 of `../plans/roadmap.md` turns the recompiled
EE code into readable, typed Rust one function at a time, while the game keeps
running and every step is checked by byte-identical replays.

## Mechanism

- `crates/ssx3-game` holds the hand-written code. `overrides.txt` lists the
  replaced functions (address + name); `src/lib.rs` has the same list as
  `OVERRIDES` (a unit test keeps them in sync).
- `recomp-ee` reads `overrides.txt`. For each listed function it emits the
  generated body as `f_X__generated`, a slot
  `pub static mut OVERRIDE_X: fn(&mut Cpu) = f_X__generated;` and a stub
  `f_X` that calls through the slot. Callers, the function table and the
  readable `pub use` alias keep pointing at `f_X`, so direct calls inside a
  shard, cross-shard calls and indirect calls all reach the replacement. The
  table crate (`generated/ee/game`) gets `set_override(addr, f)`.
- `ssx3_game::install()` (called by `crates/ssx3/src/game.rs` at boot) fills
  the slots. `SSX3_NO_OVERRIDES=1` keeps the generated bodies (A/B checks);
  `SSX3_OVERRIDES=addr,addr` installs only some.
- Dependencies point one way: `ssx3-game` → generated crates → `ssx3-ee-rt`.
  Editing hand-written code rebuilds `ssx3-game` and relinks; adding an entry
  to `overrides.txt` regenerates one shard plus the table crate (~20 s
  rebuild).
- Functions with external mid-function entries (an `f_X__entries` twin:
  resume points, computed jumps into the body) can be replaced too: the stub
  sends any entry other than the start (`c.pc != start`) to the generated
  twin, so the replacement only has to handle calls to the function start.

## Contract for a replacement

To keep runs byte-identical, a replacement must reproduce everything a later
observer can see:

- memory writes (same addresses, same bits), the return value (`v0`,
  sign-extended as compiled code does) and `pc = ra` on return;
- PS2 float results: use `ps2f::VuF32` (VU0 FMAC semantics via
  `ssx3_ee_rt::float`) in the original operation order (MADD/MSUB are one
  rounding step each); FPU code needs FPU semantics (to be added);
- the EE cycle cost: `c.cycles += N` exactly as the generated code would add,
  including per-iteration costs of loops, and the `c.event()` checks at the
  same points (they deliver interrupts/thread switches);
- calls to other guest functions in the same order (call them through the
  generated functions or `call_indirect`).

Scratch state counts too when it can reach memory: a later function may
spill a VU0 register to its stack frame, so a replacement of VU0 code writes
the final lanes of the VF registers (and Q) the original used as scratch
(`ps2f::vu0_scratch`). The first replacement differed in exactly one stack
word until it did this. ACC is reproduced too (`vu0_scratch::set_acc`;
`Vu0::acc/set_acc`): `VU0_SaveContext` and the sound thread's
`SND_SaveVu0Context` copy ACC (and Q, MAC, status) to memory, so a thread
switch at an event point could expose it. `difftest` scrambles and compares
ACC as hard state. The MAC/status flags are still soft (they can't be written
through the interface).

Helpers: `abi` (arguments, return), `guest::Addr` (typed loads/stores through
the bus), `ps2f` (`VuF32`, `Vec4`), struct views such as `rider::Rider`
(offsets from `structs/`).

## Workflow for a work package

Packages (`phase2-candidates.md`: A1 math … A6 state) are modules in
`crates/ssx3-game/src/` behind cargo features (`math`, `anim`, `camera`,
`render`, `sound`, `state`), so several people can work in one checkout
without breaking each other's builds:

1. Write the replacement in the package's module and add it to that
   module's `OVERRIDES`.
2. Add a differential test (`src/difftest.rs`: `check(generated, hand,
   seeds, setup)`), comparing against `ssx3_ee::sNN::f_X` (the shard is in
   `generated/ee/game/src/lib.rs`'s table). The test builds two fresh
   machines, applies the same setup, runs both and compares registers that
   matter (v0/v1, callee-saved, sp/gp/fp/ra, f0/f1/f20-f31, VU0 VF/VI/Q),
   pc, cycles, RAM and scratchpad. Caller-saved temporaries and VU0 flags
   are reported but don't fail.
3. `cargo test --profile game -p ssx3-game --features <pkg> <pkg>::`
   (seconds; reuses the game build of the generated crates).
4. Integration (one person): add the addresses to `overrides.txt`, rerun
   `recomp-ee`, enable the feature in `default`, build, and run the replay.

## Verification

1. `cargo test --profile game -p ssx3-game --all-features` (sync test, float
   unit tests, differential tests).
2. Scripted race replay with and without overrides: screenshots and a full
   RAM dump at the end must be identical (`SSX3_NO_OVERRIDES=1`,
   `SSX3_RAM_DUMP=path`). This needs deterministic runs: GS readbacks are
   synchronous by default for that reason (`SSX3_GS_LATENT_READBACK=1` made
   two identical runs differ).

## Log

| date | function | notes |
| --- | --- | --- |
| 2026-10-15 | `Rider_BuildWorldMatrix` 0x11e098 | VU0 macro code → `rider::build_world_matrix` (quaternion normalise + matrix), 46 cycles; race to frame 9000 byte-identical (36 shots + full RAM) |
| 2026-10-15 | A1 math (18), A2 rider & animation (20), A3 camera (13), A6 state/lists/input/replay (21) | written by four parallel agents in feature-gated modules, each with differential tests (200-2000 seeds; planted mistakes caught); integrated together: 73 replacements installed, race to frame 9000 byte-identical (36 shots + full RAM), all 69 tests pass |

Lessons from the first packages:
- `difftest` compares v1, the upper 64 bits of every GPR, f0/f1 and fcr31 as
  hard state, so replacements reproduce incidental values (the FCC bit after
  compares, v1 leftovers, 128-bit `lq` results). Each package does this in
  marked lines at the end of a function.
- Some originals write dead slots below `sp` (VU0/FPU spills); replacements
  write the same values (`CameraBase_UpdateMatrices`,
  `RiderHandplantState_Enter`).
- Helpers each package first wrote privately were consolidated after round
  2 (see "Shared helpers" below).
| 2026-10-15 | A4 render packets (16), A5 sound/files (20), Tier B utilities with loops (12: memset/memcpy/str*/EA_Mem*/DMA), Tier B functions with calls (26: wrappers, virtual dispatch, rider/camera updates, allocators) | 147 replacements installed; race to frame 9000 byte-identical (36 shots + full RAM); 140 tests pass |

| 2026-10-15 | round 3: D1 rider frame doors & state handlers (25), D2 skeleton/animation/VU0 math (23, incl. hot RiderModel_BuildBoneMatrix), D5 world/sections/process lists (25) | 220 replacements installed; race to frame 9000 byte-identical (36 shots + full RAM); 213 tests pass |

| 2026-10-15 | round 3: D4 AI/BE/race logic (39), D6 frontend UI/fonts/HUD (25), D7 audio system (25), D8 memory/containers/replay/script (22) | 331 replacements installed (first override of a function with mid-function entries: GetAttrib5Factor); race to frame 9000 byte-identical (36 shots + full RAM); 302 tests pass |

| 2026-10-15 | round 3: D3 chase camera (24, incl. the Apply* family; VU0 `Acc` + Q read-back helpers) | 355 replacements installed; race to frame 9000 byte-identical (36 shots + full RAM) |

| 2026-10-15 | OpenCode waves (oc1-oc16) + round-4 packages E3-E8 (partly OpenCode, partly Claude): 323 accepted by the fast gate (`cargo nextest`, 5 s per test; 23 rejected), 3 duplicates removed | 655 replacements installed; race to frame 9000 byte-identical (36 shots + full RAM). Gate made fast: test machines without GS threads, dirty-page RAM comparison (`ssx3-ee-rt` feature `dirty-pages`, test builds only), cycle budget against runaways |

| 2026-10-15 | OpenCode wave 4 (oc17-oc22, 6 x 50) + repaired oc14/snd_lib/audio_system | 359/359 passed the gate; 1,015 replacements installed; race replay byte-identical |
| 2026-10-15 | OpenCode wave 5 (oc23-oc28, last bulk wave) | 353/353 passed the gate (10 tests timed out at 8 threads but passed in ~1.5 s each when rerun at 2 threads); 1,368 replacements installed; race replay byte-identical |

Patterns established in round 2 (now shared, see below):
- **Loops with event checks**: structured loops make the same back-edge
  `c.event()` checks with the same `c.pc`; functions the generator emitted
  as `match pc` dispatchers must be re-enterable at their resume heads (the
  trampoline calls them again with `pc` at the loop head after an unwind).
  Tests deliver real events mid-loop (VBlank on a guest thread) and drive
  re-entry.
- **Calls**: `Flow = Result<(), Unwound>` with `?` for the host-thread
  unwind check; tests use fake callees (function-table misses) that log
  every register to RAM and unwind at chosen call indices.

## Shared helpers

For package authors; read the module docs before writing a replacement.

| module | contents |
| --- | --- |
| `abi` | `arg`/`arg64`/`farg`, `set_ret`/`set_fret`, `ret`; whole registers `reg`/`set_reg` (64-bit), `r32`, `set_word` (sign-extended 32-bit), `gpr128`/`set_gpr128`; `addu`; FPU `set_fpr`, `mfc1`, `set_fcc`, `c_lt_s` (compare + FCC); `mult`/`multu`, `div`/`divu` (LO/HI exactly as `Cpu::div`/`divu`, incl. zero divisor and MIN/-1) |
| `guest` | `Addr`: bus `read_*`/`write_*`, CPU-path `lb`..`sd` (I/O timing, sign-extension), `read_u64_unaligned`/`write_u64_unaligned` (`ldl/ldr`, `sdl/sdr` pairs) |
| `ps2f` | `VuF32`, `FpuF32`, `Vec4`, `vu0_scratch` (final VF/Q values) |
| `mmi` | 128-bit register images `Gpr = [u64; 2]`: `psubb`, `psubw`, `pand`, `pnot`, `pcpyld`, `pcpyud`, `lq` |
| `call` | the call protocol (module docs: frame, registers at the call, host-thread unwind): `Flow`/`Unwound`/`guest`, `Callee{addr,f}.call(c, ret)`, `call_ptr` (`call_indirect`), `returned`, `Frame` (open/save/restore/close), `sp`, `VSlot` (gcc 2.9x vtable entry), `Target` + `tail_wrapper` (7-cycle forwarding wrappers) |
| `events` | loop event checks (module docs: structured vs dispatcher loops): `event_due`, `deliver_event`, `back_edge_unwinds`, `Resume<S>` (`Go`/`Done`/`Unwind`) for resumable stage functions |
| `difftest` (tests) | `check` (hard state), `check_lohi` (+ LO/HI; LO/HI is otherwise reported only, because e.g. `__muldi3` legitimately differs), `check_paths`/`check_paths_lohi`/`require_paths` (every listed total-cycle path taken), `compare`/`Report`; inputs `Rng` (+`edge_f32`), `scramble`, `fill`, `put_vec`, `put_words` (`.lit4`/ELF constants), `GAME_GP`, `DATA`/`SP`/`RA` |
| `difftest::fakes` | fake callees: `machine` (scramble, hooks, gp, unwind index), `fake`/`fake_call`/`fake_syscall`/`fake_jump_out`, RAM call log (`LOG*`, forced results at `LOG_RESULTS`, unwind at `LOG_UNWIND_AT`), `FAKE`, `ptr`, `vtable`, `covered` (check + fakes reached + some unwinds) |
| `difftest::events` | `schedule` (guest thread, `Events::{Off,Unwind,Deliver}` by seed), `drive` / `driven!` (trampoline-style re-entry), `check_events` (also temporaries; requires events reached) |

Still module-private: `quat_rows` (camera; rider inlines the same order).

Next: Tier B leftovers (VU0 ACC / `cfc2 vi22` camera
functions, the ChaseCameraAlgorithm_Apply* family, loops with calls); a new
candidate catalogue from the pass-3 names.
| 2026-10-18 | Night waves 1-2 (oc29-oc36): candidates from a fresh race call graph in native mode (`callprof`, frames 5001-5300; 2,115 functions run, 39.6% of calls x size already hand-written) | written by Claude subagents (code only, no cargo), tested and integrated by the lead; 58 replacements, 1,434 installed. Gate: module tests, `recomp-ee`, then the race replay RAM equal to the pre-wave build in native mode (deterministic since the light queries read a fixed depth lag). The gate caught a port that left VU0 status flags different (the difftest only reports VU0 flags): ports now issue the original's VU0 ops. `SSX3_SKIP_OVERRIDES` bisects a wave |
