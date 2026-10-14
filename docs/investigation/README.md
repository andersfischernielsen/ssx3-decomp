# Investigation notes

Findings about decompiling **SSX 3 (USA)** (`SLUS_207.72`) as they are learned.
Each file is a topic; keep them factual and dated where it matters.

| File | Covers |
| --- | --- |
| [`ghidra-mcp.md`](ghidra-mcp.md) | Ghidra + GhidraMCP backend, endpoints, OpenCode wiring |
| [`disc-and-image.md`](disc-and-image.md) | The ISO, the extracted ELF, the EE memory map |
| [`function-inventory.md`](function-inventory.md) | The 11,844-function inventory and its TSV format |
| [`r5900-decoder.md`](r5900-decoder.md) | Decoder coverage and R5900 encoding notes |
| [`boundaries.md`](boundaries.md) | Authoritative instruction boundaries + recursive descent |
| [`codegen.md`](codegen.md) | R5900 → Rust generation and the runtime/SDK boundary |
| [`playability.md`](playability.md) | Distance to playable: subsystem status and calibration |
| [`limitations.md`](limitations.md) | Known gaps in the current approach |
| [`gs.md`](gs.md), [`gs-performance.md`](gs-performance.md) | Software GS and its parallel rasterizer |
| [`gs-gpu.md`](gs-gpu.md) | GPU (wgpu compute) GS backend, `SSX3_GS=gpu` |
| [`race-textures.md`](race-textures.md) | Noisy race textures: PATH3 masking (MSKPATH3), `gs_replay --draws` |

Conventions: addresses are EE virtual addresses in hex; "the image" means
`orig/SLUS_207.72`.
