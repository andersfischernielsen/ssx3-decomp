# Roadmap

Goal: a complete decompilation of SSX 3 (`SLUS_207.72`) to Rust that runs natively, then becomes readable, typed Rust, and finally a native port with no PS2 machine emulation (`AGENTS.md`, "Goals").

Order matters: each phase is the safety net for the next one (see "Verification" below).

Legend: 
- ✅ done
- 🟡 in progress
- ⬜ not started

## Where we are

| Layer | Today |
| --- | --- |
| Game logic (EE) | Statically compiled to Rust. Hand-written (2026-11-26, `tools/progress.py`): 9,776 of 11,844 functions (82.5%, 79.3% of the bytes); all game logic (3,958), 72% of front end, graphics and sound (1,276 left), 76% of engine, libraries and online (792 left, not started). The rest is generated. The hand-written code is still PS2-shaped: register file, guest RAM, PS2 float rules, and 914 of the ports keep the generated body's goto-style control flow ("Port quality" below). |
| Kernel | Native HLE: guest threads run nested on the driver's host stack (no coroutines, no JSPI); the SND and worker threads are frame steps. Main, VSync and four park-once threads are still guest threads: `single-loop.md` steps 6-7. |
| Graphics | Native: the draw functions record typed draws, `ssx3-gfx` (wgpu) renders them. The emulated DMA/VIF/VU1/GS remain only for the reference path (`--features vu1-microcode`). |
| Movies, sound, files, pads, memory card | Native (`ssx3-mpeg`, `ssx3-snd`, `std::fs`, native libpad/libmc). The IOP does not run. |
| Game rate | Configurable (`SSX3_TICK_HZ`), bit-exact at 60: `configurable-tick-rate.md`. |

Detailed status of the native work: `native-rendering.md` (N1-N6). 
Runtime next steps: `port-architecture.md`.
Open issues: `../investigation/limitations.md`.

## Phase 1: full static recompilation, playable natively ✅

Everything the game executes is statically translated (EE: 13,306 functions, mostly structured Rust, no runtime decode; VU microprograms; the IOP sound, pad and memory-card modules) and the game is playable at full speed: boot, movies, menus, memory-card save/load, every course and mode scripted in `tools/scenarios.py`. 

Runtime misses are recorded in `generated/entries.txt` and translated on the next `recomp-ee`. History: `../archive/` (recompiler, GS/VU/IOP models, build-time split, coverage).

## Phase 2: "Rust-ify": readable, typed source 🟡

Turn machine-shaped Rust into source a person can read and change.

1. ✅ Names: every function is named (Ghidra passes 1-5,
   `../investigation/naming.md`).
2. 🟡 Types for the core data structures (rider, camera, world, render,
   input, app: `../investigation/structs/`); typed views in `ssx3-game`.
3. 🟡 Hand-written replacements, one verified function at a time (the
   porting loop in `AGENTS.md`; mechanism `../investigation/phase2-overrides.md`).
4. ⬜ Function signatures (arguments/returns from the MIPS ABI) instead of
   the register file at call boundaries (port quality L3).
5. ⬜ Register promotion: guest registers → typed locals, so functions stop
   reading/writing `c.g[...]` internally (L3).
6. ⬜ Replace PS2 idioms with plain Rust where semantics allow (memcpy/memset,
   strings, fixed-point/float helpers), keeping bit-exactness where it
   matters (L4).

Done when the subsystems in phase 3 have clear, typed interfaces.

### Port quality

"Hand-written" is a ladder, and the porting waves climb only its first rungs:

| Level | What the function is | Today (hand-written, 2026-11-26) |
| --- | --- | --- |
| L0 | Generated (machine-shaped Rust) | 2,068 functions left to port |
| L1 | Goto-style transcription: the generated body's labelled loops, labelled blocks or `match pc` state machine, register file, cycle charges, `call_ptr` calls | 914 functions (about 29,000 lines in the 34 largest dispatchers alone) |
| L2 | Structured control flow (`if`/`loop`/`while`/early return) over the register file, every register, cycle and call still reproduced | about 9,100 functions |
| L3 | Typed signature and typed locals: arguments and results as Rust values, no `c.g[...]` inside | none counted yet |
| L4 | Idiomatic Rust: PS2 idioms replaced (memcpy, strings, float helpers), bit-exact only where it matters | none |

`python3 tools/progress.py` prints the L1/L2 split (the counter classifies the
functions of the `OVERRIDES` tables by control flow; it cannot see L3/L4 yet:
extend it when L3 starts). The differential tests against the generated body
keep every level honest, so a conversion is verified exactly like a port.

Order of work: finish the L0→L1/L2 porting waves first (they shrink the
oracle's surface and unblock the thread removal); then lift L1→L2 (structure
the goto-style bodies, starting with the small ones: 17 were done in the
2026-11-26 cleanup, the 34 dispatchers above 250 lines are left); only then L3/L4,
which also change the call boundary (the `call_ptr`/slot indirection goes away
with the signatures). No effort estimate for L3/L4 yet: time a sample
(one unit of about 20 functions) before planning it.

## Phase 3: native port: move the boundary upward 🟡

Replace hardware models with host implementations at the game's own interfaces, one subsystem at a time:

- ✅ Files → `std::fs` (`../investigation/phase3-files.md`).
- ✅ Sound → native driver and mixer (`../investigation/phase3-sound.md`,
  `ssx3-snd`).
- ✅ Movies → native MPEG-2 decoder (`ssx3-mpeg`).
- 🟡 Rendering → the game's draw functions record typed draws, drawn with
  wgpu; higher resolutions, widescreen (`native-rendering.md`: N1, N2 done,
  N3/N4 in progress; design `../investigation/phase3-render.md`,
  `../investigation/n2-render-layer.md`).
- ✅ VU0 microcode → hand-written Rust (`crates/ssx3-vu/src/vu/vu0lib.rs`);
  the translated microcode (`generated/vu`) is only in the
  `vu1-microcode` reference build.
- 🟡 The rest of the PS2 (kernel, DMAC/INTC/timer state, guest RAM):
  `native-rendering.md` N4/N6, `port-architecture.md` task 3.

Done when the game runs without the PS2 hardware models (they stay available for verification until then).

## Delete the generated code ⬜

Goal (`AGENTS.md`): no PS2 machine emulation in the codebase. Today `generated/ee`
(13,306 functions) is both the fallback for the 2,068 functions not yet ported
and the oracle of every differential test; the override machinery
(`overrides.txt`, `install()`, `f_X` slots, `f_X__generated`) routes between the
two. Deleting it is not a step we can take at the end of porting, it needs its
own design:

1. Port every function (counts under "Where we are"). Until then the generated
   code is the runtime.
2. Decide the oracle's future (owner decision, not made yet). Options: keep
   `generated/ee` as a dev/CI-only reference crate (never linked into a release
   build) and keep the differential tests; or freeze each test's expected
   results as recorded vectors and delete the crate (loses the ability to test
   new fixtures). The recommendation is the first, until L3 (typed signatures)
   is done: the tests are what makes the L1→L3 conversions safe.
3. Remove the slot machinery from the release build: ports call each other
   directly (needs the typed signatures of L3), `call_ptr`/`Callee` and the
   function table go, `overrides.txt` and the `ocN` staging flow end.
4. Delete the PS2 machine layer the generated code needs (`Cpu` register file,
   guest RAM as the game's heap, the VU0/GS/DMA reference models) once nothing
   executes generated code; it stays in the reference build until then (the
   hardware-model items of phase 3).

Done when a release build contains no generated EE code and no register-file
CPU, and the reference job (if kept) is the only user of `generated/`.

## Optional: browser (WebAssembly) 🟡

The browser front end (its own repository, [`../frontends.md`](../frontends.md)): the game in a Web Worker (guest threads run nested on one stack: no JSPI needed), the disc image read with `FileReaderSync`, `ssx3-gfx` drawing with WebGPU. 

Lowest-priority target: avoid new blocking GPU readbacks.

## Final task ⬜

- Light glows/flares must be occluded by geometry: `../investigation/flare-occlusion.md`. Parked on 2026-10-15, but explicitly required. Confirm against PCSX2.

## Final state: no guest threads ⬜

The decompilation ends with the game as one loop that the port drives, without the EE kernel's threads.

The EE has one core, but the game is written as kernel threads (8 in a race: 
- `CreateThread`, semaphores, `SleepThread`/`iWakeupThread`, alarms), one running at a time. 
- Recompiled, a guest call is a host call, so a thread that blocks deep in a call chain holds host stack frames. 
- That used to cost a host stack per guest thread (coroutines, JSPI stacks in the browser). Done: threads now run nested on one stack, and `threads.rs`, `corosensei` and the `StackSwitcher` are deleted (`docs/plans/single-loop.md`).

The solution is to remove the threads from the game itself: once everything on each thread's paths is hand-written, what a thread does becomes a call from the frame loop (a VBlank wait becomes the end of a frame, a semaphore a direct call or a flag, a streaming thread a step per frame), still deterministic and RAM-identical at each step under the gate.

Done when:
- no guest thread is created: the kernel's thread, semaphore and alarm services are unused by the game.
- the coroutine layer (`threads.rs`, `corosensei`, the `StackSwitcher`) and the trampoline and dispatch-form code that only the thread-free game needs are deleted.
- every front end, the browser included, runs the same code with no stack switching.

The browser frontend needs JSPI until then.

## Verification (applies to every phase)

Runs are deterministic (fixed RTC), so any change can be checked by replaying the scripted race and comparing RAM (the gate) or screenshots byte for byte against the previous build (`../development.md`). 
Hand-written code has differential tests against its generated body. 
Other oracles: PCSX2 via PINE (`tools/pine.py`, `../investigation/tools.md`) and, in the reference build, `vu_diff` captures and `gs_replay` traces.
