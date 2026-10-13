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

## MMI / COP2 — ⬜

- The decoder names only a handful of MMI ops; the rest are `mmi` with structure
  preserved. Generate a complete mnemonic table **from the extension's Sleigh**
  (`eecore.sinc`) into `docs/investigation/mmi.md`, then implement.
- COP2/VU0 macro-mode arithmetic (`lqc2`/`sqc2` are done; `vadd` etc. are not).

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
