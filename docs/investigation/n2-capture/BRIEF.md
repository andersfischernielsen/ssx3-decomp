# Brief: N2 capture specs (read-only research)

Context: `docs/plans/native-rendering.md` (step N2, stages A-C). SSX 3 is
statically recompiled to Rust; the game's render layer builds VIF/GIF packets
in guest RAM that the renderer (`crates/ssx3-render`, `crates/ssx3-gfx`)
decodes. Stage A replaces decoding by **captures**: a wrapper around a game
draw function runs the generated body unchanged, then records a typed command
(`crates/ssx3-render/src/direct.rs`) from the function's arguments and guest
memory, keyed by the start of the packet the body wrote. The renderer plans
GPU draws from the command alone. Examples already done:
`crates/ssx3-game/src/capture.rs` (slot 71 strips; the board-wake and shadow
ribbon emitters, literal ports of their batching), `direct.rs`
(`Cmd::Strip`, `Cmd::Ribbon`: per program start, the VU1 input qwords
relative to TOP).

Your job: write a **precise spec** for one area into
`docs/investigation/n2-capture/<area>.md`, enough to implement the capture
(or native draw) without re-reading the decompiler. Do not modify any other
file, do not run cargo builds or the game.

Sources:
- Ghidra over HTTP (JSON): `curl -s "http://127.0.0.1:8080/decompile_function?address=0x37e238"`
  (`.data.code`), `.../disassemble_function?address=0x...` (`.data.items`:
  address, instruction). Use the disassembly to settle what the decompiler
  blurs (registers, gp-relative globals, delay slots, exact constants).
- Generated Rust: `generated/ee/sNN/src/lib.rs`, functions `f_XXXXXXXX`
  (`grep -l "fn f_0037e238" generated/ee/*/src/lib.rs`); names in
  `generated/functions.tsv` (start, end, name).
- Docs: `docs/investigation/n2-render-layer.md` (the map of producers),
  `phase3-render.md` (VU1 programs and their memory layouts),
  `structs/render.md`, `phase3-boundaries.md` §4.
- Renderer code: `crates/ssx3-render/src/frame.rs` (DeferMan reading,
  `Emitter` = the state-change simulation, `MatrixRecord`), `mat3.rs`,
  `mat6.rs`, `mat4.rs`, `mat5.rs`, `bezier.rs`, `mat1.rs`, `mat2.rs`
  (the native VU1 programs: what each entry reads from VU memory).

A spec must give, per function involved:
1. Address, name, calling convention: which register holds which argument
   (EE: a0-a3, t0-t3; floats f12-f15), gp-relative globals (gp+0xNNNN) read.
2. Every packet it writes, as what reaches VU1 per program start (MSCAL
   entry; the qwords unpacked, at which address relative to TOP or
   absolute; STCYCL/STMASK/STROW effects; data from REF tags), and GIF
   data (DIRECT) it sends. Where data comes from game memory (pointers,
   strides, scratchpad 0x70000000 contents filled by whom and when).
3. Packet start/end: how they relate to `gm+0x5a00` (start = value before
   the call?), whether one call writes several packets/records, the DMA
   tag at the start (to recognise it), and records/buckets it writes.
4. Things that make the capture differ from the packet (data the DMA
   reads later than the call, buffers rewritten before EndFrame).
5. A proposed `Cmd` variant and what the renderer needs to plan it.

Be exact and terse; mark what you verified in the disassembly (V) versus
inferred (I).
