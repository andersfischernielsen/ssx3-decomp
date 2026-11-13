# Archive

Historical notes, moved here on 2026-11-13. They describe the PS2 hardware
models (still in the tree behind `--features vu1-microcode` / `emu-gs` and the
`SSX3_*=iop/emu` switches) and how the project got where it is. Status lines,
measurements and "next steps" in them are from October 2026 and are not
maintained. Current docs: [`../README.md`](../README.md).

| doc | what |
| --- | --- |
| [`recompiler.md`](recompiler.md) | `recomp-ee` design (block dispatch, unwind/trampoline, structured output), the function inventory, instruction boundaries, the R5900 decoder, FPU lowering, the first code generator |
| [`mmi.md`](mmi.md) | R5900 MMI opcode table and COP encodings |
| [`crate-split.md`](crate-split.md) | Why the crates are split (EE core / VU / machine) for build times |
| [`gs.md`](gs.md), [`gs-gpu.md`](gs-gpu.md), [`gs-mtgs.md`](gs-mtgs.md) | The emulated GS: software rasterizer, wgpu compute backend, its own host thread |
| [`vu-vif.md`](vu-vif.md), [`vu-performance.md`](vu-performance.md) | The VU/VIF model, microcode translation and its performance |
| [`ipu.md`](ipu.md) | The IPU model (MPEG-2) and the MPC movie container |
| [`iop-audio.md`](iop-audio.md), [`sio2-memcard.md`](sio2-memcard.md) | The recompiled IOP: sound modules and SPU2; SIO2/pad/memory-card modules |
| [`phase3-boundaries.md`](phase3-boundaries.md) | Where the game meets the hardware (the research behind the native subsystems) |
| [`phase3-frame-loop.md`](phase3-frame-loop.md) | Frame pacing before the native renderer; `SSX3_HOST_CLOCK` (superseded by the tick-rate work) |
| [`phase3-render-log.md`](phase3-render-log.md) | Stage-by-stage log of the first native renderer |
| [`coverage.md`](coverage.md) | Phase-1 coverage: which paths ran, with their input scripts |
| [`ps2-web-recomp-notes.md`](ps2-web-recomp-notes.md) | Digest of ps2-web-recomp, the reference design |
| [`ps2-bible.md`](ps2-bible.md) | External PS2 hardware/ISA compilation (EE registers, timers, DMA, MMI); cited by `ssx3-core` and `ssx3-ps2` comments |
