# Architecture overview

_As of 2026-10-14._ SSX 3 (`SLUS_207.72`) runs as a **static recompilation**.
Every instruction of game code (EE, VU microcode, and the IOP sound modules) is
translated to Rust at build time. Only hardware and firmware are provided
natively.

```
                     build time                                      run time
 ┌───────────────────────────────────────────┐   ┌──────────────────────────────────────────────┐
 │ Ghidra (functions, boundaries, names)     │   │ crates/ssx3  (window: winit+wgpu; headless)  │
 │   generated/functions.tsv, boundaries.tsv │   │   game.rs: boot, register function tables    │
 │                                           │   └──────────────┬───────────────────────────────┘
 │ ssx3-recomp                               │                  │ run_frame (one field)
 │   recomp-ee  ──► generated/ee/sNN (Rust)  │──────────┐       ▼
 │   recomp-iop ──► generated/iop/modules    │──────┐   │  ┌──────────────────────────────────────┐
 │ ssx3-ps2 vu_recomp ─► generated/vu/*.rs   │──┐   │   └─►│ EE game code (structured Rust)       │
 └───────────────────────────────────────────┘  │   │      │   calls semantics library (cpu, mmi, │
                                                │   │      │   fpu, cop2) via `&mut Cpu`          │
                                                │   │      └──────┬───────────────────────────────┘
                                                │   │             │ loads/stores, syscalls
                                                │   │             ▼
                                                │   │      ┌──────────────────────────────────────┐
                                                │   │      │ ssx3-ps2 (platform layer)            │
                                                │   │      │  kernel: threads (host threads +     │
                                                │   │      │          baton), semas, handlers,    │
                                                │   │      │          alarms, syscalls            │
                                                │   │      │  hw: DMAC, INTC, timers, SIF regs    │
                                                │   │      │  vif ─► vu (translated microcode) ───┼─► XGKICK
                                                │   └──────┼► sif: SIF protocol; IOP services     │
                                                │          │   (fileio, cdvdfsv←ISO, loadfile,    │
                                                │          │    padman, mcserv HLE)               │
                                                │          │  iop_bridge ─► ssx3-iop (recompiled  │
                                                │          │   SNDDRV/LIBSD on native IOP kernel, │
                                                │          │   SPU2 → cpal audio)                 │
                                                │          │  ipu: MPEG-2 macroblocks (movies)    │
                                                └──────────┼► gs: GIF paths, swizzled VRAM,       │
                                                           │   tile-parallel rasterizer, display  │
                                                           └──────────────────────────────────────┘
```

## Key documents

| topic | doc |
| --- | --- |
| progress log, how to resume, debug switches | `plans/playable-push.md` |
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

## Invariants worth knowing

- **Nothing decodes game instructions at run time.** EE, VU and IOP code is
  translated ahead of time. A jump to untranslated code is a fatal "miss" that
  `ssx3-headless` records in `generated/entries.txt` for the next generation.
- **One guest thread runs at a time.** The baton serialises the host threads,
  so the shared `Cpu` is never touched concurrently by guest code. The GS
  runs on its own host thread (`gs::Mtgs`, `SSX3_MTGS=0` turns it off) and,
  like the rasterizer's worker threads, only touches GS-owned state; every
  GS value the guest can observe is kept on the guest side or read after a
  sync.
- **Generated output is reproducible from `orig/SLUS_207.72` plus the ISO plus
  `generated/*.tsv`/`entries.txt`.** It is gitignored except `generated/vu/`.
- **Runs are deterministic by default** (fixed RTC). Wall-clock time only
  paces the windowed front end.
