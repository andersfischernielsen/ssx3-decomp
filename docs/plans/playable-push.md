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
