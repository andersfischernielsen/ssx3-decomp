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
- Not supported: functions with external mid-function entries (an
  `f_X__entries` twin); `recomp-ee` refuses them.

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
word until it did this. ACC and the MAC/status flags are not reproduced yet.

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
- Shared helpers each package wrote privately (to consolidate):
  `set_fcc`, `mult` (LO/HI), `set_gpr128`/`set_word`, `arg64`, a `scramble`
  randomiser and an edge-case float generator for `difftest`, a `.lit4`
  constant loader, `Addr` loads/stores routed through `Cpu::lw/sw` (I/O
  timing side effects), unaligned `ldl/ldr` helpers, `quat_rows` (shared by
  rider and camera).

Next: A4 render packets, A5 sound/files, then Tier B (loops and calls).
