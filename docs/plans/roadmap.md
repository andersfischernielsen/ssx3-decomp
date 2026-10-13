# Roadmap

Goal: decompile the whole of `SLUS_207.72` (11,844 functions, 783,966
instructions) to portable Rust, plus the runtime subsystems the game needs.

> For the honest distance from here to *playable*, see
> [`../investigation/playability.md`](../investigation/playability.md).

Status legend: ✅ done · 🟡 in progress · ⬜ not started

| Phase | Status | Notes |
| --- | --- | --- |
| Extract ELF, import to Ghidra, analyse | ✅ | `docs/investigation/disc-and-image.md` |
| Function inventory with extents | ✅ | 11,844 functions, `generated/functions.tsv` |
| R5900 decoder | ✅ | 99.74% of instructions, `r5900-decoder.md` |
| Authoritative instruction boundaries | ✅ | 783,966 instructions, `boundaries.md` |
| Recursive-descent cross-check | ✅ | 70.8% exact vs Ghidra |
| ELF loader | ✅ | `ssx3-core::elf` |
| Codegen + native SDK boundary | 🟡 | syscalls/direct calls done; **control flow is scaffolding** |
| Structured control flow | 🟡 | 73% of functions structured; rest fall back |
| FPU (COP1) | 🟡 | arithmetic/compares/moves done; `cvt`, accumulator forms approximate |
| MMI / COP2 breadth | 🟡 | tables complete in the decoder; codegen lowering mostly pending |
| Register promotion (SSA) | ⬜ | registers → typed locals |
| Scale codegen to all functions | ✅ | `recomp-all` emits all 11,844 (128 MB, 51 shards) |
| GS/GIF → wgpu | ⬜ | VRAM is presented, nothing writes it |
| VU0/VU1 | ⬜ | `.vutext` exists; no VU model |
| DMA, timers, INTC | ⬜ | game spins waiting on these |
| Assets (`DATA/`, 2.3 GB) | ⬜ | not examined |

## Near-term order

1. Structured control flow (replaces the `match pc` dispatcher).
2. FPU completion (`cvt`, `adda`/`mula` semantics) — [`subsystems.md`](subsystems.md).
3. MMI breadth, driven by a from-sleigh mnemonic table.
4. Register promotion, then scale-out.
5. Runtime subsystems to get a first frame.
