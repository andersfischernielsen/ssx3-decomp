# ssx3-decomp

Native **Rust** recompilation of **SSX 3 (USA)** — serial `SLUS_207.72` — with a
`wgpu` renderer. The reference binary is the PlayStation 2 Emotion Engine
(R5900) ELF extracted from the retail disc image.

> SSX 3 is © EA. This repository contains no copyrighted game data. The disc
> image and the extracted ELF are **not** committed; see
> [`tools/extract-from-iso.sh`](tools/extract-from-iso.sh).

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

## Layout

```
crates/
  ssx3-core/   Machine model: memory map, RAM, syscalls (no dependencies)
  ssx3-gs/     Graphics Synthesizer model + wgpu presenter
  ssx3/        The `ssx3` binary
orig/          Extracted SLUS_207.72 (gitignored)
tools/         Helper scripts
```

## Build and test

```sh
cargo build
cargo test
```

Tests in `crates/ssx3-core/tests/` cover the memory model (LE accesses,
cached/uncached aliases, mirroring), the syscall table, and — when
`orig/SLUS_207.72` is present — the real ELF header and program-header load.
`crates/ssx3-gs` unit-tests the GS register decoding.

## Run

```sh
cargo run -p ssx3
```

This opens a `wgpu` window presenting GS VRAM. Set `SSX3_EXIT_AFTER_FRAMES=N`
to exit after N redraws (used for smoke tests).

## Ghidra MCP

OpenCode is configured with the `ghidra` MCP server (LaurieWired GhidraMCP).
For it to work, Ghidra must be running with the **GhidraMCPPlugin** enabled (it
then serves a loopback HTTP backend on `127.0.0.1:8080`) and the program open.

## Roadmap

- [x] Extract `SLUS_207.72`, import with the R5900 language, auto-analyze
- [x] Machine model: memory map, RAM/scratchpad/IOP RAM, syscall table
- [x] Test suite, including validation against the real ELF
- [x] `wgpu` window presenting GS VRAM
- [ ] R5900 → Rust recompiler (seed from the 11,844 Ghidra functions)
- [ ] EE runtime: DMA, timers, INTC, GS/GIF packet path
- [ ] GS rasterizer feeding `wgpu`
