# Distance to playable

_Assessment as of 2026-10-13. This is an estimate, not a promise; revisit it as
subsystems land._

## Summary

We have a **working static-recompilation foundation, not a playable build**.
Roughly **low single-digit percent** of the total effort to "playable" is done,
and it is the *mechanical, verifiable* part. The remaining ~90% is the hard,
game-specific part (VU, GS, the IOP, assets, timing).

## Subsystem status

| Subsystem | To be playable | Status |
| --- | --- | --- |
| EE (R5900) recompilation | correct translation of 783,966 instrs | structure done; ~30k instrs unlifted; semantics approximate |
| EE runtime | threads, scheduler, exceptions | minimal (regs, memory, syscall stubs) |
| DMA / timers / INTC | actually drive them | register models only, not wired to memory |
| VU0 / VU1 | microcode execution (vertex math) | **0%** — `.vutext` untouched |
| GS | rasterize prims, textures, blend → pixels | **~5%** — GIF parsing only; zero pixels drawn |
| IOP (second CPU) | recompile R3000 + IRX modules (CD/DVD, sound) | **0%** — not even scoped |
| Assets | parse `DATA/` (2.3 GB), load levels | **0%** |
| Audio (SPU) | music / SFX | **0%** |
| Input | controller | **0%** |
| Register promotion / correctness | "reads like hand-written Rust" | **0%** |

## What "playable" requires

All of the above working **simultaneously and correctly enough** to: get past the
boot sequence, load a course from the disc, run VU1-transformed geometry through a
GS rasterizer, and not deadlock on the scheduler/interrupts.

Today we cannot get past the game's C-runtime init: it spins waiting on hardware
we do not drive (`FindAddress` loop; see `codegen.md`).

## Structural realities that make this hard

- **Two CPUs.** The IOP (R3000) runs its own programs — the filesystem driver and
  audio. Recompiling it is a separate project, not started.
- **VU1 is where the geometry happens.** ~59 KB of microcode transforms vertices;
  without a VU model there is no 3D, and it is a large subsystem.
- **GS accuracy is the long tail.** Many texture formats and blend paths; matching
  them is where projects stall.
- **Assets are opaque.** A 2.3 GB custom archive format must be reverse-engineered
  before a level can load.

## Calibration

- PCSX2 — an **emulator**, strictly easier than a decompilation — is 20+ years of
  continuous work.
- Community **matching decompilations** of much simpler games (Mario 64, PS1
  titles) are ~5–10 person-years each.
- A PS2 3D game recompiled to *correct* native Rust is at the far end of that
  scale. Realistically: **years for a focused team; effectively open-ended solo.**

## Shortest path to a first frame

1. Wire **DMA / INTC / timers** so boot proceeds past the init spin.
2. **128-bit GPRs + VU0** lowering (unlocks the ~27k VU0 instructions).
3. A **minimal GS rasterizer** (IMAGE transfers + flat primitives) feeding the
   existing wgpu presenter.
4. **See the first pixels.** Still many weeks of focused work, and well short of
   playable.

## Measured state snapshot

| | |
| --- | --- |
| functions translated | 11,844 |
| instructions | 783,966 (decode coverage 99.73%) |
| structured control flow | 10,219 (86%) |
| unsupported lowering | 29,954 (3.8%) |
| tests | 104 |
| generated Rust | ~120 MB, 51 shards (`recomp-all`) |

## What is reused / automatable

The reusable parts are the ones built here: Ghidra-driven analysis, an
authoritative decoder validated against Ghidra, a code generator, and coverage
tooling (`recomp-stats --todo`). That is why mechanical coverage moved fast — but
the distance from "translates 783,966 instructions" to "renders a snowboarder at
60 fps" is mostly VU + GS + IOP + assets.
