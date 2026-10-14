# Plan: overnight push toward playable (started 2026-10-14)

Branch: `overnight-playable`. Goal: get SSX 3 as far toward playable as
possible in one long unattended session. Milestones, in order:

1. EE runs past C-runtime init (the `FindAddress` spin in `codegen.md`).
2. IOP modules "load", SIF RPC answered, disc reads served from the ISO.
3. GS receives packets; first non-black pixels in the window.
4. Front-end / menu renders.
5. A course loads; input drives the rider.

## Architecture decision: interpreter-first machine, recompiler plugs in

The new crate **`crates/ssx3-ps2`** is a full PS2 machine model:

| module | owns |
| --- | --- |
| `bus` | 32 MiB RAM, scratchpad, MMIO dispatch (`0x1000_0000` I/O, GS priv regs, VU mem) |
| `ee` | R5900 interpreter: 128-bit GPRs, MMI, FPU (PS2 float rules), COP0, COP2 macro → `vu` |
| `kernel` | HLE of the EE kernel: syscalls, threads, semaphores, INTC/DMAC handlers, alarms |
| `dmac`, `hw` | DMA channels (chain tags), INTC, timers, VBlank |
| `vu`, `vif` | VU0/VU1 micro + macro semantics, VIF0/1 command stream and UNPACK |
| `gs` | GIF paths, GS registers, swizzled VRAM, software rasterizer, display readout |
| `sif`, `cdvd` | IOP HLE: SIF RPC servers (fileio, cdvd, pad, sound stubs), ISO9660 reads |

**Why an interpreter now.** Bring-up needs fast iteration (the 114 MB of generated
Rust has never been compiled) and the EE kernel needs cheap context switching
between EE threads. Recompiled functions use the native stack, so they can't switch
threads without host threads or fibers. The interpreter is also the
**correctness oracle** for the recompiler: the same `Cpu` state can run either path,
and traces can be diffed. Recompiled functions plug in later through an
address → `fn` table, with the interpreter as the fallback for indirect jumps
and gaps.

**IOP strategy.** HLE at the SIF RPC level (the `sceSifCallRpc` choke point) plus
function-level overrides for library calls where that's simpler (`sceCdRead`
served from the ISO). An R3000 interpreter running the game's IRX modules
(ps2-web-recomp's approach) is the fallback if HLE of EA's custom
`SNDDRV.IRX` / streaming proves necessary.

Reference design: `snuri00/ps2-web-recomp` (MIT); notes in
`docs/investigation/ps2-web-recomp-notes.md`.

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

## How to resume

```sh
cargo run --release -p ssx3-recomp --bin recomp-ee   # regenerate generated/ee
tools/run-headless.sh 2400 --shot-every 100          # screenshots → /tmp/ssx3-shots
SSX3_INPUT="1600:0008:8,2000:4000:8,2500:4000:8,2700:4000:8,2900:4000:8,3100:4000:8,3300:4000:8,3500:4000:8,3700:4000:8,4250:4000:8" \
  ./target/game/ssx3-headless 4700 --shot-every 100  # scripted path into a race
# (with the memory card present there is no boot prompt; with SSX3_MEMCARD=none prepend 950:0010:8,1000:4000:8)
cargo run --profile game -p ssx3                     # windowed (keyboard pad)
SSX3_IOP=1 cargo run --profile game -p ssx3          # ... with the recompiled sound driver
SSX3_IOP=1 SSX3_WAV=/tmp/a.wav ./target/game/ssx3-headless 2400   # capture audio headless
```
Runtime misses are appended to `generated/entries.txt` automatically; rerun
`recomp-ee` after a miss. Debug env: `SSX3_TRACE_SIF`, `SSX3_TRACE_SYSCALLS`,
`SSX3_TRACE_THREADS`, `SSX3_VRAM_DUMP`, `SSX3_INPUT`, `SSX3_SKIP_MOVIES=0`.
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
