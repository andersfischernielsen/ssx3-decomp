# Native rendering: everything around the game logic native

Owner decision (2026-10-18): the game **logic** stays the decompiled code;
everything around it becomes native: no PS2 graphics data formats (DMA
chains, VIF packets, VU1 programs, GIF packets, GS state) between the game
and the GPU. Work runs autonomously, with subagents for parallel parts.
This plan is the reference for that work; update its status section as
steps land.

Related: [`roadmap.md`](roadmap.md) (phase 3, the native port),
[`../investigation/phase3-render.md`](../investigation/phase3-render.md)
(the seven VU1 programs, the draw-record level, the typed frame),
[`../investigation/phase3-boundaries.md`](../investigation/phase3-boundaries.md)
§4 (the game's render layer: draw slots, `DeferMan`, the kick thread).

## Where we are (2026-10-18)

A frame today (native build; browser numbers in `crates/ssx3-web/README.md`):

1. The game's render code (decompiled game code) records draws
   (`DeferMan`, 0x80-byte records) and builds VIF/GIF packets in guest RAM;
   `PS2GraphicsMan_EndFrame` (0x377a10) sorts them into DMA chains and
   publishes the chain heads.
2. The frame sink copies all 32 MiB of guest RAM (`frame_sink.rs`): 2.75 ms
   per frame on the game thread.
3. `ssx3-render` walks the chains, unpacks VIF into VU1 memory images and
   runs the VU1 programs natively on the CPU (`mat1`..`mat6`, bit-exact):
   transform, lighting, clipping, GIF packet output.
4. `ssx3-gfx` decodes the GIF packets into GS primitives, keeps a GS state
   model, batches and draws with wgpu.

Per frame (6000-frame menu + race run, native headless): typed frame
1.55 ms, build (VU1 + GIF decode) 3.37 ms, prepare 0.72 ms, encode+submit
0.56 ms on the render thread. Natively this fits in a frame; in the browser
everything shares one thread and it is the bottleneck.

## Target

The game's draw calls produce **typed draw commands** (mesh, matrix,
material state, textures); meshes and textures live on the GPU; vertex work
(transform, skinning, lighting, particles, Bézier patches) runs in shaders.
No guest-RAM snapshot, no DMA/VIF/GIF parsing, no VU1 programs on the CPU.

## Steps

Each step keeps the game playable and is verified before the next.

### N1: VU1 vertex programs on the GPU ⬜

Replace "run the VU1 program on the CPU, decode its GIF output" with GPU
vertex generation per material type, fed by the VU input the typed frame
already has (`VuCall` inputs, unpacked VIF data):

- framework in `ssx3-gfx`: per frame, upload the calls' VU input images to a
  storage buffer; a compute (or vertex) shader per program writes vertices
  in the renderer's vertex format into the frame's vertex buffer; the CPU
  keeps only what decides batching (GIF tag / register setup the program
  emits, primitive type, vertex counts);
- programs, by weight: type 3 general (`mat3`, world meshes, models, HUD
  strips), types 6/7 rider + shadow (`mat6`, skinning), types 1/2 Bézier
  terrain (`mat1`/`mat2`/`bezier`), type 4 particles (`mat4`), type 5 snow
  (`mat5`);
- the CPU programs stay as the reference (`SSX3_GPU_VU=0`) and for tests.

Verification: GPU floats are IEEE, not PS2, so exactness is to a tolerance:
per-program tests compare the shader's vertices with the CPU program's on
captured frames (`SSX3_FRAME_DUMP` + `gfx_replay`), then screenshot diffs
against the CPU path on menu and race frames.

**Design (the contract each program follows).**

- A call's CPU side (`ssx3-gfx/src/gpuvu/<program>.rs`): `plan(...)` gets
  the VU1 memory image (inputs applied, as today), entry, TOP, ITOP. It
  - does the program's cheap, order-dependent memory side effects on the
    image so later calls see the same memory (setup matrices, lighting
    coefficients, in-place texgen on UV arrays, flags, toggles);
  - copies the vertex inputs the shader reads into the frame's data buffer
    and writes a per-call uniform block (matrices, guard scale/offset,
    lighting, texgen, flags, data offsets);
  - builds the index list from what the CPU knows (strip lengths: ADC at
    strip starts; data-dependent culling is left to the GPU's clipping);
  - hands the GS side the PRIM / GIF tag the program would kick
    (`Gs::gpu_draw`), so the draw gets the current GS state and keeps its
    place in the batch order (a GPU batch is never merged);
  - returns `false` for anything not ported: the CPU program runs instead.
- The GPU side (`ssx3-gfx/src/gpuvu/<program>.wgsl`): a vertex entry
  `vs_<program>` with vertex pulling (bind group 1: data words, call
  uniforms; `instance_index` = call), producing the shared `VOut` of
  `draw.wgsl` (clip-space position `(x·w, y·w, z·w, w)` from the program's
  screen position, screen-linear colour / STQ / fog), so the GS fragment
  stage, blending and tests are unchanged.
- Hardware clipping replaces the VU guard-band cull and the
  Sutherland–Hodgman clipper.
- `SSX3_GPU_VU=0` keeps the CPU programs (reference, A/B).

### N2: typed draws at the game's render layer ⬜

Override the game's draw entry points (boundaries §4.3: draw slots 95/96
models, 99-101/124 riders, 71 strips, 72-81 2D, 82-86 particles/trails,
Bézier patches, grid meshes, post passes) and/or `PS2GraphicsMan_RenderFrame`
so that draws become native commands instead of packets:

- meshes (prebuilt model packets, rider skins, patches) are uploaded to the
  GPU once and cached by guest address and generation;
- per-draw data (matrices, lights, bones, colours) is copied at record time;
- textures: `TexMan` registration → host textures (already cached by
  `ssx3-gfx`), no GIF uploads;
- the replacements keep every guest-visible side effect (buffer pointers,
  counters, `DeferMan` state the game reads), so RAM-identity A/B runs
  (`SSX3_NO_OVERRIDES` / `SSX3_OVERRIDES`) keep proving the game logic.

### N3: no guest-RAM snapshot ⬜

With N2 the renderer consumes the command list; the 32 MiB copy per frame
goes (the commands carry what they reference).

### N4: strip the hardware models ⬜

`Renderer_DmaKickThread` (0x382760), `Renderer_SetDisplayRegs` (0x382af0),
the DMA/VIF/GS register models: reduced to the statuses and events the game
waits on (VBlank, render idle, buffer states).

### N5: movies (optional) ⬜

IPU emulation → a native MPEG-2 decoder feeding a texture.

## Rules

- `AGENTS.md`: native, fast, unit tests over heavy runs.
- Builds: at most **two** cargo builds at a time across all agents (the
  machine nearly crashes otherwise); `-j 4`.
- Subagents work in worktrees on self-contained pieces (one program, one
  draw slot family) with a precise spec, and run that piece's unit tests;
  integration, A/B runs and merges are done by the lead.
- Game data (frame dumps, screenshots, RAM dumps) stays out of git.
- Commit to `main`; push `main` only.

## Status

| Step | State | Notes |
| --- | --- | --- |
| N1 | ⬜ | |
| N2 | ⬜ | |
| N3 | ⬜ | |
| N4 | ⬜ | |
| N5 | ⬜ | |
