# ssx3-decomp

Native **Rust** static recompilation of **SSX 3 (USA)** (serial `SLUS_207.72`).
Every instruction of the game's Emotion Engine code and its VU microcode is
translated to Rust at build time and runs on a native PS2 platform layer:

- kernel
- DMA, VIF and GIF
- a software GS
- the IPU
- the IOP services

**Status (2026-10-14):** the game boots, plays its intro movies, runs the menus,
loads a course and **races** with music and sound effects: HUD, rivals, and
position and speed tracking all work. It also **saves and loads** to a
file-backed memory card, and 4,325 of 11,844 functions carry recovered names. It saves to and loads from an emulated memory card (`saves/ssx3-mc0.ps2`). Gaps remain: speed (software GS), audio, and some rendering
issues. Screenshots and the full log are in
[`docs/plans/playable-push.md`](docs/plans/playable-push.md).

> SSX 3 is © Electronic Arts; "SSX" is an EA trademark. This project is not
> affiliated with or endorsed by EA. The repository contains no game data and
> no game code: the disc image, the extracted executable and everything
> translated from them (`generated/ee`, `generated/iop`, `generated/vu`) are
> produced locally from **your own copy** of the game and are never committed
> (see [`tools/extract-from-iso.sh`](tools/extract-from-iso.sh) and
> [`tools/bootstrap.sh`](tools/bootstrap.sh)).

## Quick start

```sh
tools/extract-from-iso.sh "SSX 3 (USA).iso"            # -> orig/SLUS_207.72 (keep the ISO in the repo root)
tools/bootstrap.sh                                     # translate EE + IOP code -> generated/ee/ (~79 MB of Rust)
cargo run --profile game -p ssx3                       # play (window; ~2 min first build)
```

Keys: arrows = d-pad, X = Cross, Z = Square, S = Circle, A = Triangle,
Enter = Start, Right Shift = Select, Q/W = L1/R1, 1/2 = L2/R2, F12 = screenshot.

Useful switches: `SSX3_SKIP_MOVIES=1` skips the boot movies (the headless runner
skips them by default), `SSX3_GS=cpu` uses the software rasterizer instead of
the wgpu one, `SSX3_MEMCARD=<path>` picks the memory card file (default
`saves/ssx3-mc0.ps2`), `SSX3_REAL_CLOCK=1` uses the host clock.

Headless runs, scripted input and the debug switches are documented in
[`docs/plans/playable-push.md`](docs/plans/playable-push.md#how-to-resume).

## Architecture

| crate | role |
| --- | --- |
| `ssx3-recomp` | Function inventory and boundaries (from Ghidra), the R5900 decoder, and **`recomp-ee`**. The generator lowers all ~808k instructions in 13.3k functions to **structured Rust**: 97% of functions become `if`/`loop` code with no `pc` variable, and the rest use block dispatch. Output is sharded into 16 crates under `generated/ee/`; see [`docs/investigation/recompiler-v2.md`](docs/investigation/recompiler-v2.md). `recomp-iop` does the same for the game's IOP modules (sound, controller, memory card; see [`docs/investigation/sio2-memcard.md`](docs/investigation/sio2-memcard.md)). |
| `ssx3-ee-rt` | The EE core that generated code is compiled against: `Cpu` (128-bit GPRs, HI/LO, FPU, COP0), instruction semantics (MMI, bit-exact FPU, COP2 moves, PS2 float), guest RAM/scratchpad, the function table, and the `Platform` trait behind which the hardware sits. Kept small and stable so hardware changes never recompile generated code ([`docs/investigation/crate-split.md`](docs/investigation/crate-split.md)). |
| `ssx3-vu` | VU0/VU1: register file, pipeline/timing model, the translated-code fast path, the microcode translator, and the statically translated microcode (`generated/vu/`). Used as `ssx3_ps2::vu`. |
| `ssx3-ps2` | The platform layer (`Machine`): MMIO, DMAC, timers and INTC, the HLE EE kernel (threads, semaphores, handlers, alarms), SIF with the IOP services (loadfile, fileio, cdvdfsv, padman, EA SNDDRV protocol), VIF, GS (GPU or software rasterizer, on its own thread), and IPU (MPEG). |
| `ssx3-iop` | The IOP runtime for the recompiled IRX modules: IOP kernel, SPU2 audio, SIO2 with pad and memory card. |
| `ssx3` | Front ends: the windowed `ssx3` (winit + wgpu) and `ssx3-headless` (scripted input, screenshots, diagnostics). |
| `ssx3-core`, `ssx3-gs`, `ssx3-runtime` | Earlier scaffolding (memory model, first codegen); superseded by the above. |

Each guest thread runs on its own host thread, and a baton ensures only one runs
at a time. A guest call is a plain Rust call, and a thread switch blocks inside
the syscall. `recomp-ee --unwind` generates the original resumable form, which
uses the unwind/trampoline model from
[ps2-web-recomp](https://github.com/snuri00/ps2-web-recomp) (MIT).

## What the disc actually contains

An `.iso` is a UDF disc image, not a program — it cannot be decompiled directly.
The disc root holds:

| File | Purpose |
| --- | --- |
| `SYSTEM.CNF` | Boot config: `BOOT2=cdrom0:\SLUS_207.72;1` |
| `SLUS_207.72` | The game executable: ELF32, little-endian, **MIPS (R5900)**, stripped |
| `DATA/` | ~2.3 GB of level/asset archives |
| `NETGUI/` | ~8 MB network UI |

The ELF is the decompilation target. Known facts (from Ghidra):

- image base `0x00100000`, entry `0x00100008`
- SHA-256 `1b49d05ca2793922180851b9e1ce9ae2291d61a7863565ac4e71f12e967af7bc`
- 93 memory segments; 11,844 functions after auto-analysis
- `.vutext` (`0x0042e590`) holds VU1 microcode

## Extracting the executable

```sh
tools/extract-from-iso.sh "/path/to/SSX 3 (USA).iso"   # -> orig/SLUS_207.72
```

## Analysis in Ghidra

1. Import `orig/SLUS_207.72` as an ELF.
2. Set the language to **`r5900:LE:32:default`** ("Emotion Engine (Play
   Station 2) R5900 MIPS Core") from
   [`chaoticgd/ghidra-emotionengine-reloaded`](https://github.com/chaoticgd/ghidra-emotionengine-reloaded).
   The plain MIPS language will not decode the EE instructions.
3. Run **Analysis → Auto Analyze**.
4. Query it through the Ghidra MCP server (see below).

The ELF is stripped, but because it was built with `-ffunction-sections`, its
`.gnu.linkonce.t.*` **section names still carry mangled C++ symbol names**
(e.g. `RCMP5CODEC`, `RCMP7DECODER`), which is a free naming source.

## Function inventory

`tools/export-ghidra-functions.py` pulls every function from the GhidraMCP
backend and writes a canonical, sorted table:

```sh
python3 tools/export-ghidra-functions.py            # -> generated/functions.{json,tsv}
```

`generated/functions.tsv` (`<hex-address>\t<name>`) is embedded into
`ssx3-recomp` at compile time. As of the current export it holds **11,844**
functions, of which **75** have recovered names (`entry`, `SetGsCrt`,
`AddIntcHandler`, `_Exit`, ...); the rest are Ghidra placeholders.

Query it with the bundled CLI:

```sh
cargo run -p ssx3-recomp --bin recomp-stats            # summary + named list
cargo run -p ssx3-recomp --bin recomp-stats -- 0x0012ab34
cargo run -p ssx3-recomp --bin recomp-stats -- SetGsCrt
```

## R5900 decoder

`ssx3-recomp::r5900` decodes the raw `.text` bytes from the ELF into structured
instructions. Across the whole image it decodes **99.74%** of 801,242
instructions; the rest are preserved as raw words. See
[`docs/investigation/r5900-decoder.md`](docs/investigation/r5900-decoder.md).

## Ghidra MCP

OpenCode is configured with the `ghidra` MCP server (LaurieWired GhidraMCP).
For it to work, Ghidra must be running with the **GhidraMCPPlugin** enabled (it
then serves a loopback HTTP backend on `127.0.0.1:8080`) and the program open.

## Investigation notes

Findings are recorded under [`docs/investigation/`](docs/investigation/): the boot path, naming, the recompiler, GS, VU/VIF, IPU, the ps2-web-recomp notes, and earlier decoder and boundary work. Plans live in [`docs/plans/`](docs/plans/); the long-term phases (full static recompilation → typed Rust → native port) are in [`docs/plans/roadmap.md`](docs/plans/roadmap.md).

## Tests

```sh
cargo test -p ssx3-ee-rt -p ssx3-vu -p ssx3-ps2 -p ssx3-recomp -p ssx3-iop -p ssx3-core
```
