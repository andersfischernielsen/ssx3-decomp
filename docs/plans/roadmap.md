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
| Game logic (EE) | Statically compiled to Rust: about half of it hand-written typed Rust (the README's progress chart, `tools/progress.py`), the rest generated. Still PS2-shaped: register file, guest RAM, PS2 float rules. |
| Kernel | Native HLE: guest threads as coroutines on one host thread, semaphores, alarms, interrupts. The final state has no guest threads (below). |
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
   the register file at call boundaries.
5. ⬜ Register promotion: guest registers → typed locals, so functions stop
   reading/writing `c.g[...]` internally.
6. ⬜ Replace PS2 idioms with plain Rust where semantics allow (memcpy/memset,
   strings, fixed-point/float helpers), keeping bit-exactness where it
   matters.

Done when the subsystems in phase 3 have clear, typed interfaces.

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
