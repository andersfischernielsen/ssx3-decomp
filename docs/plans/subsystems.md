# Plan: runtime subsystems

Each subsystem is needed before the game can boot to a frame. Ordered by what
unblocks the most.

## FPU (COP1) — 🟡

- ✅ `fpr[32]`, `.s`/`.d` storage, `add/sub/mul/div/sqrt/abs/mov/neg`,
  `c.*` compares → `fcr31_cc`, `bc1f`/`bc1t`, `mtc1/mfc1/cfc1/ctc1`.
- ⬜ `cvt.*` (needs the source format from `fmt`, which the decoder currently
  drops — extend `Cop1Arith` or the mnemonic).
- ⬜ `adda`/`suba`/`mula`/`diva` exact semantics (currently approximated as the
  non-accumulating forms).
- ⬜ Full FCR31 (rounding, flags), FPU exceptions.

## MMI / COP2 — 🟡

- The decoder now names the **full** MMI set (MMI0/1/2/3 = 78 ops) and the
  non-group single-`fct` forms; `clear`/`moveq` aliases handled. See
  `docs/investigation/mmi.md`.
- Codegen lowering is the gap: MMI vector ops and COP2/VU0 macro ops need 128-bit
  GPRs, which the current `Cpu` (`gpr: [u64; 32]`) does not have.
- `recomp-stats --todo` lists what remains (~30k instructions, almost all VU0:
  `cop2` 12k, `lqc2`/`sqc2` 13k, `qmtc2`/`qmfc2`/`cfc2`/`ctc2` ~1.8k, and 2,078
  un-decoded COP1 encodings).

## Lowering coverage

`recomp-stats --todo`, after the FPU/load-store work:

| | |
| --- | --- |
| instructions | 783,966 |
| unsupported | 29,954 (3.8%) |

The remaining unsupported set is the VU0/128-bit subsystem plus a few COP1
formats — the next large piece of work.

## VU0/VU1 — ⬜

- `.vutext` holds VU1 microcode. Needs a VU model (VU mem, 4-wide lanes,
  `vcallms`/microcode upload) — a large, self-contained subsystem.

## GS / GIF — ⬜

- A GIF packet parser (tags, `A+D`, `IMAGE` transfers) that writes VRAM.
- GS rasterization: primitives, textures, blending → feeds the existing `wgpu`
  presenter (which already shows VRAM).
- Start with `IMAGE`/VRAM writes so something appears.

## DMA, timers, INTC — ⬜

- The init path spins waiting on these (`FindAddress` loops in our runs). Model
  the DMA registers, a timer tick, and INTC so `WaitSema`/sleep paths progress.

## Assets — ⬜

- `DATA/` (2.3 GB) and `NETGUI/`. Unsampled. Likely a custom archive format;
  needs its own investigation.
