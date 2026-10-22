# Investigation notes

Findings about **SSX 3 (USA)** (`SLUS_207.72`) and how this project handles
them, one topic per file. Addresses are EE virtual addresses in hex; "the
image" means `orig/SLUS_207.72`.

| File | Covers |
| --- | --- |
| [`disc-and-image.md`](disc-and-image.md) | The ISO, the extracted ELF, the EE memory map |
| [`boot-path.md`](boot-path.md) | Boot to the main loop: syscalls, IOP modules, threads, movies |
| [`function-inventory.md`](function-inventory.md), [`naming.md`](naming.md) | The 11,844-function inventory and the naming passes |
| [`r5900-decoder.md`](r5900-decoder.md), [`boundaries.md`](boundaries.md), [`codegen.md`](codegen.md), [`recompiler-v2.md`](recompiler-v2.md) | Decoding, instruction boundaries, R5900 → Rust generation |
| [`fpu.md`](fpu.md), [`mmi.md`](mmi.md) | PS2 float and MMI semantics |
| [`crate-split.md`](crate-split.md) | Crate layout for build times |
| [`phase2-overrides.md`](phase2-overrides.md), [`phase2-worker-guide.md`](phase2-worker-guide.md) | Hand-written typed replacements: contract, workflow, log |
| [`frame-callgraph.md`](frame-callgraph.md) | The race's per-frame call graph (`callprof`) |
| [`phase3-boundaries.md`](phase3-boundaries.md), [`phase3-files.md`](phase3-files.md), [`phase3-sound.md`](phase3-sound.md), [`phase3-frame-loop.md`](phase3-frame-loop.md), [`phase3-render.md`](phase3-render.md), [`n2-render-layer.md`](n2-render-layer.md) | The native subsystems: where the game meets the hardware, and how each is replaced |
| [`ipu.md`](ipu.md) | MPEG-2 movies (the IPU model and the native decoder's shared parts) |
| [`gs.md`](gs.md), [`gs-gpu.md`](gs-gpu.md), [`gs-mtgs.md`](gs-mtgs.md), [`vu-vif.md`](vu-vif.md), [`vu-performance.md`](vu-performance.md) | The emulated graphics path (`SSX3_RENDER=emu`, kept for reference) |
| [`iop-audio.md`](iop-audio.md), [`sio2-memcard.md`](sio2-memcard.md) | The recompiled IOP paths (`SSX3_SND=iop`, `SSX3_IOP_SIO2=1`) |
| [`flare-occlusion.md`](flare-occlusion.md) | Open: light glows through geometry |
| [`coverage.md`](coverage.md) | Which game paths have been exercised, with their input scripts |
| [`ghidra-mcp.md`](ghidra-mcp.md), [`pcsx2-pine.md`](pcsx2-pine.md), [`ps2-web-recomp-notes.md`](ps2-web-recomp-notes.md) | Tools and references |
| [`limitations.md`](limitations.md) | Known limitations |
| [`structs/`](structs/README.md) | Runtime data structures: rider, camera, world, render, input |
