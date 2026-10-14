# Plan: overnight push toward playable (started 2026-10-14)

Branch: `overnight-playable`. Goal: get SSX 3 as far toward playable as
possible in one long unattended session. Milestones, in order:

1. EE runs past C-runtime init (the `FindAddress` spin in `codegen.md`).
2. IOP modules "load", SIF RPC answered, disc reads served from the ISO.
3. GS receives packets; first non-black pixels in the window.
4. Front-end / menu renders.
5. A course loads; input drives the rider.

## Architecture (current)

Full static recompilation, no interpreter: EE code via `recomp-ee` (structured
Rust), VU microcode via `vu_recomp` (`generated/vu`), IOP modules via
`recomp-iop`. `crates/ssx3-ps2` provides only hardware and firmware (HLE kernel,
DMAC/INTC/timers, VIF/VU runtime, GS (CPU or wgpu), IPU, SIF/IOP bridge). See
`docs/architecture.md`. The original "interpreter-first" plan was abandoned
on night one (see the log below).

## Status log

(append newest last)

- 2026-10-14: crate skeleton, plan written; research agents mapping the boot path
  (`docs/investigation/boot-path.md`) and the reference design.
- 2026-10-14 (night): **architecture corrected to full recompilation** (no EE
  interpreter): `recomp-ee` lowers all EE code to Rust (`recompiler-v2.md`),
  `ssx3-ps2` is only the hardware/firmware layer, VU microcode is statically
  translated too (`vu-vif.md`). Milestones reached, in order:
  1. Boot through `_InitSys` (FindAddress answered consistently), SIF init, IOP
     reboot (LOADFILE version read from `DNAS271.IMG`), all IOP module loads.
  2. First pixels: the loading icon (`docs/investigation/first-pixels-loading-icon.png`).
  3. First UI: memory-card prompt (`first-ui-memcard-prompt.png`); pad input works
     (native padman writing `pad_data` frames each VBlank).
  4. EA SNDDRV protocol emulated (per-frame tick, upload acks) → main menu,
     character select with a VU1-rendered 3D rider (`character-select.png`).
  5. Course loads; race briefing over the 3D course (`race-briefing.png`).
- Open items: IPU (agent working; movies skipped with `SSX3_SKIP_MOVIES` until
  then), GS speed (software rasterizer: ~2 fps in 3D scenes), audio output,
  memory card (reports "no card").

- 2026-10-14 (later): **real audio path.** The disc's LIBSD.IRX and SNDDRV.IRX are
  statically recompiled (`recomp-iop`) and run on a native IOP kernel with an SPU2
  model and cpal output, behind `SSX3_IOP=1` (`docs/investigation/iop-audio.md`).
  Title/menu music and sound effects play. Music also needed EE RCNT compare
  interrupts (EA's timer task table runs the stream service), added opt-in in
  `ee_timers.rs` (on with `SSX3_IOP=1` or `SSX3_EE_TIMER_IRQ=1`).
- 2026-10-14 (morning):
  6. **In race** (`in-race-first.png`, `racing.png`): HUD, rivals, speed, position.
  7. Broken 3D geometry fixed: VU clip flags must become visible after 3 pairs, not 4
     (`race-start-fixed.png`). Found with the `SSX3_GIF_STATS` tap (far vertices
     14k → 0).
  8. FPU made bit-exact (round toward zero, integer-key compares).
  9. EE timer compare/overflow interrupts: the game's master clock (Timer 1
     handler 0x3e4db8) now runs; boot movies play through the IPU
     (`movie-eabig.png`).
  10. Static VU0 macro lowering (14 of ~10k COP2 macro ops still decode at
      run time); 1,109 functions named (`naming.md`), emitted as `pub use` aliases.
- In flight: GS performance (tile-parallel rasterizer + capture/replay),
  IOP recompilation + SPU2 audio (`SSX3_IOP=1`).
- 2026-10-14 (late morning):
  11. **GS 7–13× faster** (tile-parallel, bit-identical) + mipmapping: races run
      ~20 fps in the software rasterizer (was ~2).
  12. **Audio**: SNDDRV/LIBSD IRX statically recompiled (`crates/ssx3-iop`,
      `recomp-iop`) on a native IOP kernel + SPU2 model; music/SFX in menus and
      races; on by default (`SSX3_IOP=0` falls back to HLE stubs).
  13. VU VI-backup quirk now follows PCSX2 exactly (`_vuBackupVI`): fixes terrain
      microcode overrun that fed vertex data into CLAMP (striped terrain).
  14. **Structured decompilation output** (`recomp-ee --structured`): host-thread
      runtime + Ramsey structuring, 97% of functions as `if`/`loop` Rust; runs
      the game (verification to the race in progress).
- GPU GS backend (`SSX3_GS=gpu`, `docs/investigation/gs-gpu.md`): wgpu compute
  rasterizer, visually identical to the CPU GS on all traces and in a full
  headless run to frame 4800; races are now VU-bound (~22 fps headless).
- Fresh checkout: `tools/bootstrap.sh` (stub generated crates, then recomp).
- 2026-10-14 (afternoon):
  15. **Memory card saving + native pads**: the disc's SIO2MAN, PADMAN, MCMAN
      and MCSERV are recompiled too and drive a SIO2 model with a DualShock 2
      and an 8 MB memory card backed by `saves/ssx3-mc0.ps2`
      (`docs/investigation/sio2-memcard.md`). The boot "No memory card"
      prompt is gone; Options → Save/Load formats the card and saves
      ("Save complete."), a later run loads it ("Load complete.").
      `SSX3_IOP_SIO2=0` returns to the HLE pad/mc servers, `SSX3_MEMCARD=none`
      empties the slot. The race script still works (each press lands one
      screen earlier; see the doc for the old-pacing variant).
- 2026-10-14 (midday): **memory card saving** — SIO2MAN/PADMAN/MCMAN/MCSERV
  recompiled on the IOP runtime with a SIO2 + DualShock 2 + 8 MB card model
  (`saves/ssx3-mc0.ps2`); save and load verified (`sio2-memcard.md`).
- VU differential testing against ps2-web-recomp's interpreter on captured game
  invocations (`vu-differential.md`): fixed I-bit ordering (upper reads the old I),
  MAX/MINI raw-bit semantics (integer vertex colours were zeroed: course now
  shaded), VI-backup default; `SSX3_RTC` pins the clock for reproducible runs.
- 2026-10-14 (evening): GPU GS is the default in the game front ends
  (`SSX3_GS=cpu` for the software rasterizer). VU differential branch merged.
  In flight (agents): race texture noise (`race-textures.md`), VU1 performance
  (`vu-performance.md`). Open: walls/course still show a blocky noise texture
  after the MAX/MINI fix; race ~20 fps.
- Long scripted race (hold Cross from frame 4600): at frame 7000 the rider is
  **1st of 6, 42 s in, 22% down the course, 52 mph** with no misses or halts
  (`docs/investigation/race-42s-first-place.png`). Visible issues: salmon
  untextured slabs, a dark rectangle over the sky. The run stopped at frame
  7140 (killed externally, not a game halt).
- Window front end: game controllers via gilrs (buttons + analog sticks),
  boot movies play by default in the window.
- Build hygiene: worktree agents each compile the full generated crates; cap
  with `CARGO_BUILD_JOBS=3` + `nice`, at most two heavy builds at once.
- **Race and menu textures fixed: PATH3 masking** (`race-textures.md`). SSX 3
  queues each frame's texture uploads in one GIF DMA chain and releases one
  packet per `MSKPATH3 0`/`MSKPATH3 1` pulse in the VIF1 stream. All of them
  used to land at once, so draws sampled the last texture uploaded to a
  shared VRAM slot through their own CLUT (noise on the race walls, the
  smeared sky panel, wrong menu images). New debug tools:
  `gs_replay --draws`, `SSX3_TRACE_PATH3`.
- After the PATH3 merge: scripted race runs to frame 9000 (1:16 race time, 2nd
  place, 35% down the course) in 296 s headless with no halts or misses
  (`race-116s-after-path3.png`). Fixed-clock runs are pixel-deterministic.
  Next: stretched geometry ("girders", dark sky rectangles), agent on
  `race-geometry.md`.
- 2026-10-14 (evening): **translated VU 3.5× faster, races 23.8 → 53.7 fps**
  headless (GPU GS, fixed RTC; `vu-performance.md`). Translate-time analysis
  of stalls / ready times / Q-P-flag observers, deferred MAC/status, NEON
  four-lane FMAC (bit-identical), NEON macro-mode VU0, plus getenv-per-call,
  per-primitive allocations, VIF unpack and upload re-hashing fixes. Every
  captured VU invocation replays to an identical full-state digest; all
  screenshots of the race run are byte-identical.
- VU performance merged on top of PATH3: the scripted run to frame 9000 takes
  171 s headless (was 296 s) and all 18 screenshots are byte-identical to the
  pre-merge run. Races ~54 fps headless; remaining frame time is split
  VU ~38% / GS CPU side ~28% / EE ~14% (`vu-performance.md`).
- **A full race completes**: the scripted run (holding Cross) reaches the
  "Snow Jam - Race / Single Event Results" screen at frame 20000 (6th, 3:49)
  in 452 s headless, no halts or misses (`race-complete-results.png`). Around
  72% down the course (frame 14000) the view is mostly empty blue sky with a
  sliver of geometry (`race-72pct-blank-view.png`). Montage of frames
  10000-19500 every 500: `race-full-montage.png` (stretched polygons throughout).

## Next steps (as of 2026-10-14 evening)

1. Stretched race geometry / dark sky rectangles (agent; `race-geometry.md`).
2. 60 fps: frame time is VU ~38%, GS CPU side ~28%, EE ~14%. Biggest lever is
   running GS work on its own thread (PCSX2 MTGS style) with sync points at
   readbacks/CSR/vram reads; then texture upload (`transfer_bytes`, per-pixel
   swizzle) and `GpuGs::push`/`Pages::rect`.
3. A race finishes (done). Next: the blank view at ~72% of Snow Jam; career
   mode progression; the other courses.
4. (done) No VU0 macro op decodes at run time any more: `c.vu0_macro` appears
   nowhere in `generated/ee`.

5. **Build-time split (after the geometry and MTGS agents merge)**: every
   `ssx3-ps2` change recompiles all ~79 MB of generated EE crates (2–5 min with
   3 jobs, >10 min when agents build concurrently). Split a small, stable core
   crate (Cpu registers, FPU/MMI/COP2 semantics, memory access) that generated
   code depends on; GS/VU/VIF/DMA/kernel move behind a narrow interface it calls
   (MMIO, syscalls, COP2/VU0), so hardware-model changes only rebuild that crate
   and relink. Also consider lower opt-level for generated crates in dev and a
   shared sccache for worktree agents.

## How to resume

```sh
tools/bootstrap.sh                                   # fresh checkout: stub crates, then recomp-ee/iop
cargo run --release -p ssx3-recomp --bin recomp-ee   # regenerate generated/ee (+ IOP modules)
cargo build --profile game -p ssx3                   # ~5 min after a platform change (all EE crates)
tools/run-headless.sh 2400 --shot-every 100          # screenshots → /tmp/ssx3-shots (SSX3_SHOTS=dir)
SSX3_INPUT="1600:0008:8,2000:4000:8,2500:4000:8,2700:4000:8,2900:4000:8,3100:4000:8,3300:4000:8,3500:4000:8,3700:4000:8,4250:4000:8,4600:4000:3000" \
  ./target/game/ssx3-headless 9000 --shot-every 500  # scripted path into a race, holding Cross
# (with the memory card present there is no boot prompt; with SSX3_MEMCARD=none prepend 950:0010:8,1000:4000:8)
cargo run --profile game -p ssx3                     # windowed (keyboard or game controller)
SSX3_WAV=/tmp/a.wav ./target/game/ssx3-headless 2400 # capture audio headless
```
Input script: `frame:buttons(hex libpad bits):duration`. Runs are deterministic
(fixed RTC; `SSX3_RTC=<secs>` / `SSX3_REAL_CLOCK=1` change that).
Runtime misses are appended to `generated/entries.txt` automatically; rerun
`recomp-ee` after a miss. Debug env: `SSX3_TRACE_SIF`, `SSX3_TRACE_SYSCALLS`,
`SSX3_TRACE_THREADS`, `SSX3_TRACE_PATH3`, `SSX3_VRAM_DUMP`, `SSX3_GS_CAPTURE`
(+ `gs_replay`), `SSX3_VU_CAPTURE` (+ `vu_diff`), `SSX3_SKIP_MOVIES`, `SSX3_GS=cpu`.
Machine limits: worktree agents each build all generated crates; use
`CARGO_BUILD_JOBS=3 nice -n 10`, at most two heavy builds at once, and delete
finished worktrees' `target/`.
