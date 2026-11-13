# Overrides: how hand-written functions replace generated ones

The mechanism and the contract behind every port in `crates/ssx3-game`. The workflow
(waves, writers, gate) is in `AGENTS.md` ("Porting loop"), the writer rules in
`../porting/writer-brief.md` and `phase2-worker-guide.md`, the verification switches in
`../development.md`.

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
- `ssx3_game::install()` (called by `crates/ssx3-engine/src/boot.rs` at boot) fills
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
- PS2 float results, in the original operation order: VU0 code issues the
  original's macro ops (`c.vu0()`, `lqc2`/`sqc2`; recomputing with
  `ps2f::VuF32` leaves the VU0 flags different), FPU code uses
  `ps2f::FpuF32` (MADD/MSUB are one rounding step each);
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

## Lessons that became rules

- `difftest` compares v1, the upper 64 bits of every GPR, f0/f1 and fcr31 as hard
  state, so ports reproduce incidental values (the FCC bit after compares, v1
  leftovers, 128-bit `lq` results).
- Some originals write dead slots below `sp` (VU0/FPU spills); ports write the same
  values (`CameraBase_UpdateMatrices`, `RiderHandplantState_Enter`).
- Ports issue the original's VU0 ops: a port that recomputed VU0 maths left the VU0
  status flags different, which the difftest only reports but the replay caught.
- Loops with back-edge event checks use `events::event_due`/`deliver_event_here`;
  a loop that ran `c.event()` and ignored the unwind halted the browser build.
  Single post-syscall checks keep the generated inline `c.event()` form
  (`deliver_event_here` there reschedules and diverges the browser replay).
- Structured loops make the same back-edge checks with the same `c.pc`; functions the
  generator emitted as `match pc` dispatchers must be re-enterable at their resume
  heads (the trampoline calls them again with `pc` at the loop head after an
  unwind). Tests deliver real events mid-loop and drive re-entry.
- Calls: `Flow = Result<(), Unwound>` with `?` for the unwind check; tests use fake
  callees (function-table misses) that log every register to RAM and unwind at
  chosen call indices.
- The replay gate has caught what unit tests missed: an inverted branch that
  diverged a whole race, a missing `f0` result (soft FPU state, now asserted).

## Shared helpers

Read the module docs before writing a replacement.

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
