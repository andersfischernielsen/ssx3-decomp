# Architecture overview

_As of 2026-10-18._ SSX 3 (`SLUS_207.72`) runs as a **static recompilation**
with a native runtime around it. The game's own code (EE) is translated to
Rust at build time; everything around it (rendering, movies, sound, files,
pads, memory card) is native Rust. The PS2 hardware models are still in the
tree, for verification and the emulated paths (`SSX3_RENDER=emu`), but the
default native mode does not use them on the frame path.

```
 build time                                   run time (native mode, the default)
 ┌──────────────────────────────────────┐     ┌──────────────────────────────────────────────┐
 │ Ghidra: functions, names, boundaries │     │ crates/ssx3: window (winit + wgpu), headless │
 │ ssx3-recomp: recomp-ee ─► generated/ │     └───────────────┬──────────────────────────────┘
 │   ee (host threads), ee-unwind       │                     │ one VBlank per frame
 │   (single thread, WebAssembly)       │                     ▼
 └──────────────────────────────────────┘     ┌──────────────────────────────────────────────┐
                                              │ EE game code (generated Rust, ssx3-ee-rt):   │
                                              │   guest RAM, PS2 float/VU0 semantics         │
                                              │ + ~1,400 typed Rust replacements (ssx3-game) │
                                              └──┬───────────┬───────────┬───────────┬───────┘
                                                 │ draw slots│ libmpeg   │ SND API   │ files, pads,
                                                 ▼           ▼           ▼           ▼ memory card
 ┌──────────────────────────────────────┐  ┌──────────────┐ ┌─────────┐ ┌──────────────────────┐
 │ ssx3-game::packetless: native draw   │  │ ssx3-mpeg:   │ │ssx3-snd │ │ native HLE in        │
 │ functions + RenderFrame: game-visible│  │ MPEG-2 movies│ │ driver  │ │ ssx3-ps2 (sif, files,│
 │ state + typed draw commands          │  │ (N5)         │ │ + SPU2  │ │ pad, ssx3-mc)        │
 │ (ssx3_render::direct), no packets    │  └──────────────┘ │ mixer   │ └──────────────────────┘
 └──────────────┬───────────────────────┘                   └─────────┘
                │ at the kick signal (frame_sink, native kick)
                ▼
 ┌──────────────────────────────────────┐   ┌──────────────────────────────────────────────┐
 │ ssx3-render::frame::native: the typed│──►│ ssx3-gfx (render thread): native VU programs │
 │ frame from DeferMan's buckets + the  │   │ (CPU or GPU), wgpu draws, post passes,       │
 │ captures; texture pages (FrameMem)   │   │ any resolution / aspect                      │
 └──────────────────────────────────────┘   └──────────────────────────────────────────────┘

 Still modelled for the game: the EE kernel (threads as coroutines on one host thread,
 semaphores, interrupt handlers), DMAC/INTC/timer registers, the GS register state
 (detached: it draws nothing), VU0 (macro ops; its microprogram is hand-written Rust).
 What remains and the plan: plans/native-rendering.md (N6).
```

## Crates

| crate | role |
| --- | --- |
| `ssx3-ee-rt` | EE core: `Cpu`, instruction semantics, guest RAM, function table, the `Platform`/`Vu0`/`Hooks` interfaces. The only dependency of generated EE code. |
| `ssx3-vu` | VU0/VU1: state, timing model, fast path, microcode translator, translated microcode (`generated/vu`). Re-exported as `ssx3_ps2::vu`. |
| `ssx3-ps2` | The machine (`platform::Machine`): DMAC/INTC/timers, VIF, GS + MTGS, IPU, kernel HLE, threads, SIF/IOP bridge, the frame sink and the native kick (`kick.rs`). Implements `Platform`. |
| `ssx3-game` | Typed Rust replacements of game functions (phase 2), the native draw functions and RenderFrame (`packetless`), native libmpeg (`movie.rs`). |
| `ssx3-render` | The typed frame: captures (`direct`), the frame build, the native VU1 programs. |
| `ssx3-gfx` | The wgpu renderer (window, headless, browser). |
| `ssx3-mpeg` | MPEG-2 video: the native movie decoder and the pieces it shares with the IPU model. |
| `ssx3-snd` | The native sound driver (EA's IOP SNDDRV in Rust). |
| `ssx3-mc` | The native memory card. |
| `ssx3-web` | The browser build (unwind form in a Web Worker, WebGPU). |
| `ssx3-iop` | IOP runtime for the recompiled IRX modules, SPU2, SIO2/pad/memory card. |
| `ssx3-recomp` | `recomp-ee`, `recomp-iop` (generators). |
| `ssx3` | Front ends (window, headless). |

A hardware change rebuilds only its crate and relinks (GS/kernel ~3 s, VU
~2.5 min); only `ssx3-ee-rt` changes recompile the generated EE crates
(`investigation/crate-split.md`).

## Key documents

| topic | doc |
| --- | --- |
| long-term phases (recomp → typed Rust → native port) | `plans/roadmap.md` |
| building, headless runs, verification, debug switches | `development.md` |
| boot path, syscalls, IOP modules, threads | `investigation/boot-path.md` |
| EE generator, threading modes, structured output | `investigation/recompiler-v2.md` |
| function names (Ghidra passes) | `investigation/naming.md` |
| GS, performance, capture/replay | `investigation/gs.md`, `investigation/gs-performance.md` |
| texture streaming, PATH3 masking | `investigation/race-textures.md` |
| GS on its own host thread (MTGS), sync points | `investigation/gs-mtgs.md` |
| VU/VIF, microcode translation | `investigation/vu-vif.md` |
| VU translation performance (fast path, NEON, checks) | `investigation/vu-performance.md` |
| IPU (movies) | `investigation/ipu.md` |
| IOP recompilation, SPU2 audio | `investigation/iop-audio.md` |
| reference design notes | `investigation/ps2-web-recomp-notes.md` |
| crate layout for build times (core / VU / machine) | `investigation/crate-split.md` |

## Invariants worth knowing

- **Nothing decodes game instructions at run time.** EE, VU and IOP code is
  translated ahead of time. A jump to untranslated code is a fatal "miss" that
  `ssx3-headless` records in `generated/entries.txt` for the next generation.
- **One guest thread runs at a time.** Guest threads are coroutines on the
  frame-driving host thread (`threads.rs`), so the shared `Cpu` is never
  touched concurrently by guest code. The GS
  runs on its own host thread (`gs::Mtgs`, `SSX3_MTGS=0` turns it off) and,
  like the rasterizer's worker threads, only touches GS-owned state; every
  GS value the guest can observe is kept on the guest side or read after a
  sync.
- **Generated output is reproducible from `orig/SLUS_207.72` plus the ISO plus
  `generated/*.tsv`/`entries.txt`.** It is gitignored except `generated/vu/`.
- **Runs are deterministic by default** (fixed RTC). Wall-clock time only
  paces the windowed front end.
