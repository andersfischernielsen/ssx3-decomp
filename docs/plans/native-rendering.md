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

**Render directly (owner decision, 2026-10-18).** The renderer must not sit
behind the PS2 hardware interface and reconstruct the frame from packets and
a RAM copy: the game's draw functions themselves become native and record
GPU draws where the game says "draw this". No render worker or other extra
data passing is added on the way (a render-worker split for the browser was
considered and dropped). The GPU runs asynchronously, so recording draws on
the game thread is cheap; the render thread and the snapshot go once every
draw family is direct.

## Steps

Each step keeps the game playable and is verified before the next.

### N1: VU1 vertex programs on the GPU ✅

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

### N2: direct rendering at the game's draw functions 🟡

Family by family (order: models slot 95/96, riders 99-101/124, Bézier
terrain, particles/trails 82-86, HUD strips 71 + 2D DIRECT slots 72-81, grid
meshes, post passes and the rest):

- **D1 (capture):** the family's draw function is wrapped: the generated
  body still runs (guest-visible state, cycles and event points identical,
  so RAM identity holds by construction), and a native draw command is
  recorded from the function's own inputs (mesh, matrices, textures,
  render state), keyed by the packet it wrote. The renderer draws the
  command (meshes cached on the GPU, the N1 vertex shaders) and skips that
  packet: no VIF decoding, no per-frame mesh upload for the family.
- **D2 (replace):** once every family is captured, the render layer stops
  building packets: the draw functions, `PS2GraphicsMan_EndFrame` /
  `RenderFrame`, the kick thread and texture uploads become native, keeping
  only what the rest of the game reads (the render-state stack, matrix
  records, buffer states, counters). The gate becomes RAM identity with the
  render-private regions masked, plus behaviour (cycle charges move).
- Then N3 is immediate: nothing reads guest RAM after the frame is
  recorded, so the snapshot and the render thread go.

Progress (2026-10-18):

- Models first, on the renderer side (no game hook yet): a model's prebuilt
  mesh chain is decoded once and replayed (`frame::ChainCache`, ~99% hits in
  races), and its vertex data stays on the GPU (`gs::MeshArena`): typed
  frame 1.76 → 1.24 ms per frame natively, VIF decoding 14% → 4% of the
  browser worker.
- Measured in the browser race: the game's own render layer (packet
  building) is only ~7% of the worker; the renderer is ~44% (typed frame
  13%, `Build::run` 12%, `draw_built` 12% with uploads 5%), the game logic
  the rest. What D2 still saves is the reconstruction (typed frame, the GS
  state machine for state packets), worth roughly another 15-20% of the
  browser frame; the snapshot is already cheap (incremental pages).

- Meshes the arena keeps are planned without their vertex data (the VU plan
  learns per call whether the arena has it).

**D1 mechanics (2026-10-18).**

- `ssx3_game::capture`: a wrapper per draw function in an override slot
  (`overrides.txt`, "draw capture"; installed at boot unless
  `SSX3_DIRECT=0`). It runs `f_X__generated`, then records a
  `ssx3_render::direct::Cmd` keyed by the packet start (`gm+0x5a00` before
  the body), the packet end (where the bucket link goes) and the packet's
  first DMA tag. Nothing is recorded when the body unwound (that packet
  stays on the decoding path) or while no renderer is installed.
- The frame sink takes the list at the kick-semaphore signal, so it travels
  with the snapshot (`DumpMem::direct`) and with frame dumps (a section
  after the VU1 memory).
- `frame::build_cached`: inside a payload, a packet with a capture is not
  decoded: `Event::Native { index, regs }`, its program starts advance the
  VIF double buffering only, and the walk continues at the packet end.
- `ssx3-gfx` `Build::run` draws `Event::Native` with
  `direct::plan(cmd, regs, header, vu)`: the draw's header from its
  MatrixRecord, the programs' static data (fan tags) from the frame-start
  VU1 image.
- Pilot: slot 71 strips (`Cmd::Strip`). The other families follow the
  same pattern; D2 then stops building their packets (the record keeps an
  empty packet, so the bucket order stays the game's).
- Captured so far: strips (slot 71), board wake and shadow ribbons
  (`EmitBoardWakeStrips` / `EmitShadowStrips`, `Cmd::Ribbon`: literal ports
  of their batching over the vertex rings). The native event carries VU1
  qw0-5 as the VIF holds them (the MatrixRecord header differs in the guard
  z lanes for some draws). Captured draws are planned with the VU calls
  (`vujob::JobItem::Native`), so the browser's helpers run them.
  `SSX3_DIRECT_CHECK=1` decodes captured packets too and compares.
- Measured (2026-10-18, race): decoded VU input 41.5k -> 26.6k qwords per
  frame; browser worker: renderer 42.7% -> 39.9%, typed frame 11.5% ->
  8.8%. Left in the decoder: rider bones and morph targets (morph chains
  write partial qwords, so they are not cached), Bézier patch packets,
  particles.
- Other renderer work on the way: replayed mesh calls share their input
  (`VuCall::shared`); PATH3 texture uploads are read without their image
  data (1.4 MiB per race frame); render-pass state set only on change; VU
  job data copied in bulk.
- Uploads per race frame 3.2 MB -> ~1.5 MB: triangle lists the shader
  expands (`GpuCall::index_mode`: Bézier strip/fan and rider strip indices
  `t << shift | corner`), compacted per-call uniforms (`uq()`), rider
  vertex records in the mesh arena (`GpuCall::split`: they hold VU table
  addresses; the call carries the bone setup's windows, shared by the
  draws after one setup).
- Typed frame: cached chains are validated by page writes instead of a
  hash of their data (the frame sink stamps each RAM page with the
  snapshot that first copied it: `Mem::snapshot` / `Mem::unchanged`);
  texture keys likewise (`tex::KeyCache`); per-draw payload copies only
  for checks (`frame::keep_payloads`). Native race: typed frame 0.84 ->
  0.49 ms per frame.
- Browser race (Chrome, after the above but the last two items): frames
  12.6-14.8 ms (were 16-21 ms at the start of N2), renderer 31% of the
  worker (was 44%), the worker idle ~21% of the time.

**The way to the end (owner, 2026-10-18: "go ahead with the
pre-requisites, then. Work autonomously").** Stages, each verified on frame
dumps (pixel identity with the decoding path) before the next:

1. **A: every packet captured.** Each producer gets a capture and its
   command a native planner (what it draws from the command alone: header,
   lights, bones, mesh, descriptor; a scratch VU image inside a planner is
   an implementation detail). Gate: on race and menu dumps no payload
   packet is decoded (`SSX3_VIF_STATS`: decoded payload input 0), images
   identical with `SSX3_DIRECT=0`.
   *Done 2026-10-18* (`ssx3_game::capture`): every producer is captured
   (strips, ribbons, particles, models and grid meshes, riders and their
   shadows, 2D/DIRECT draws, render-target switches, shadow volume, fog
   sprites, Bézier patches and passes, AppendPacket's record-less packets,
   the post passes, the rider lighting / shadow header packets). Most are
   `Cmd::Vu`: the packet's effects read back (stage C computes them from
   the inputs), CALLs as chain steps through the chain cache, rare masked
   UNPACKs as raw codes. No decoded payload in race, freestyle, ctm and
   multi runs.
2. **B: the frame from the buckets.** The renderer builds the frame from
   DeferMan's sorted buckets and the commands, without walking the chains:
   GS state from the emitter's prediction (`Draw::gs`, checked by
   `frame_verify`), textures from TexMan, the post passes (depth fog,
   bloom, colour filter) and the 2D/DIRECT draws as native passes. Gate:
   identical images with the walk disabled.
   *Done 2026-10-18* (`ssx3_render::frame::native`, the default;
   `SSX3_NATIVE_FRAME=0` walks the chains): material setups per type and
   mode, TexMan_EmitTex0's binds (an evicted texture, TBP -1 when the frame
   is built, gets a block of its own the renderer maps back to TexMan),
   the post passes with their PATH3 windows, the phase split. On 37 dumps
   (race, menu, freestyle, ctm, multi) the events equal the walked frame's
   (`SSX3_NATIVE_FRAME_CHECK=1`; texture uploads and VRAM addresses aside)
   and the images are identical; the build is 10-30% faster.
3. **C: no packets (D2).** The draw functions and `RenderFrame` /
   `EndFrame` become native: they keep what the game reads (records,
   buckets, render-state stack, counters, cursors) and build no packet; the
   kick thread and chain publication go. Gate: RAM identity with the
   render-private regions masked; behaviour.
   *In progress* (`ssx3_game::packetless`, on while the native renderer
   draws; `SSX3_PACKETS=1` keeps the packets). A native draw function
   writes the game-visible state (render state, records, buckets, hash
   chains, cursors, flags) and records its draw at an empty packet (the
   0x10 link-tag room), which the native frame build resolves through the
   capture. Guest callees run with `Cpu::no_unwind`. Cycle charges are
   estimates, so whole runs drift slightly from the generated timing
   (interrupt order); screenshots match until the drift shows.
   Check: `SSX3_DRAW_CHECK=1` (`scripts/draw-check.sh`) runs native and
   generated from the same state and compares the game-visible words and
   the draws (VU writes as maps; a packet's stale words past a batch count
   are reported as uncovered, not as differences); functions that write
   beyond their records are checked against all of memory, sampled every
   `SSX3_DRAW_CHECK_EVERY` calls (packet addresses, VifBuff, the SprStream
   ring and stack scratch masked). `SSX3_DRAW_CHECK=2` renders the native
   draws under generated behaviour.
   Native (menu, race, freestyle checked: no differences): strips (71,
   Begin / EndTriStrip), particles (82-86, PatchFX), DrawModel (95, fast
   path, alpha path, environment maps), grid meshes, riders (99, 100,
   101, 124), Bézier terrain (v18 with its emitters, passes and deferred
   groups), shadow volume and board wake (89, 90, with the ribbons as
   their packets' steps), render-target switches (53, 54, 55),
   transition (61), 2D: indexed triangles (72), line strips with their
   clipping (73), sprites (78, 79), glyphs (80, 81), fog sprites, and the
   post passes (depth fog with its CLUT, bloom, colour filter) as
   `Cmd::Post`. `RenderFrame` is native (`frame.rs`): it keeps the bucket
   build and sort, TexMan's per-frame state, the post passes and
   DeferMan's emit state, and publishes bare `END` chains; no state
   packets, texture binds or VRAM uploads, no CALL / RET (the frame build
   ends a bucket's payload at its last packet). The ctm and two-player
   modes run without frame issues and match the packet build until the
   timing drift shows. Native race, 6000 frames: 22.98 s with packets ->
   19.90 s.
   Bucket links: packetless, `DeferMan_AppendPacket` records them
   (`Cmd::Link`, at the packet end the NEXT tag would be) instead of
   writing the tag; the frame build follows them. Left in guest memory:
   the four bare `END` chain heads. Slot 75 (`DrawLineShape`, debug
   paths only) is native too, ported from the decompile; no run calls it.
   Guest callees from native draw functions run with `Cpu::no_unwind` and
   at host depth 0 (`events::deliver_event_here` now restores the outer
   `no_unwind`: a hand-written port's event check inside such a call
   cleared it, and the browser build switched threads mid-call).
4. **N3/N4/N5** as below.

Background (the earlier proposal, still the reference for the details):

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

Findings ([`../investigation/n2-render-layer.md`](../investigation/n2-render-layer.md)):
30 functions write `DeferMan` records, plus two producers that append to
buckets without a record (slot 95 opaque models, `PS2BezierMan_v18`'s
deferred type-2 patches), so reading records at `RenderFrame` would miss
draws; only `cPS2FogParticleMan_DrawParticles` sits outside the render layer;
nothing outside it reads `DeferMan`/VifBuff/GifBuff, but ~90 game functions
use the render-state stack (`gm+0xe84`) and the draw slots leave changes in
it. Approach:

- **Phase A (observe):** wrap each producer: run the generated body (RAM,
  cycles and event points stay identical) and record a typed command keyed
  by its VifBuff packet start, with its data copied now (vertices,
  descriptors, bones from the scratchpad, lighting, render state, matrix
  index). At EndFrame, join commands to the sorted bucket list; packets with
  no command take today's path. Order: slot 71 strips → 95 models + grid
  meshes → riders → Bézier → particles/trails → 2D DIRECT slots → the rest.
- **Phase B (replace):** family by family, stop building the packets; the
  gate becomes behavioural or RAM identity with the render-private regions
  (VifBuff, GifBuff, DeferMan, scratchpad, SprStream) masked.

Risks: model memory reused by streaming (key meshes by address + content
hash or hook load/unload), VifBuff double buffering (copy at record time),
scratchpad-resident bones/matrices (capture during the call), the layer
split the lens-flare queries need.

### N3: no guest-RAM snapshot 🟡

With N2 the renderer consumes the command list; the 32 MiB copy per frame
goes (the commands carry what they reference).

Step 1 (done): every store into EE RAM marks its page (`bus::mark_written`,
the bus and the DMA path), and each snapshot buffer copies only the pages
written since it was last filled. Measured on a race dump, the renderer
reads ~845 pages (3.3 MiB) per frame: VifBuff, the model chains, textures.

Step 2 (done 2026-10-18): the typed frame is built on the game's thread
at the kick signal, from live memory (`frame_sink::on_signal_sema`;
`FrameBuilder` lives in the sink). The renderer gets a `Published` frame:
the typed frame, the flip packet and PATH3 packets taken at build time,
and a `FrameMem` holding only the pages it still reads
(`frame::texture_ranges`: TexMan's table, the TexInfos, pixels and CLUTs
of the textures bound this frame or resident in VRAM). Pages are shared
between frames until written (`PAGE_STORE`, by page generation), so a
frame copies almost nothing; reads outside them are counted
(`timing_report`, `SSX3_FRAME_MEM_LOG=1`). The 32 MiB snapshot buffers
are gone except for frame dumps (`SSX3_FRAME_DUMP`,
`SSX3_FULL_SNAPSHOT=1`). Gate: the 37 dumps render identically from
`FrameMem` and from all of memory, no read misses; a live freestyle run
gives the same screenshots. Native freestyle: 0.86 ms per frame on the
game thread (typed frame 0.66 ms), the render thread no longer builds.
Left for N3: the renderer still decodes textures from guest pages (stage
C: TexMan registration hands them over).

### N4: strip the hardware models 🟡

`Renderer_DmaKickThread` (0x382760), `Renderer_SetDisplayRegs` (0x382af0),
the DMA/VIF/GS register models: reduced to the statuses and events the game
waits on (VBlank, render idle, buffer states).

State (2026-10-18), native mode: the GS model is detached (registers and
events only; it rasterizes nothing: every frame, menus, loading screens
and movies included, is drawn by `ssx3-gfx`), the kick thread's DMA
kicks of a drawn frame complete at once without VIF / VU1 / GS work
(`frame_sink::skip_chain`), and the chains it kicks are bare `END`s.
What remains is the guest kick thread itself (its semaphore handshake,
the DMA-end interrupts and the flip packet `Renderer_SetDisplayRegs`
builds, which the renderer reads for the frame blend and clear) and the
register models' code for the emulated paths (`SSX3_RENDER=emu`).

### N5: movies ✅

Done 2026-10-18: Sony libmpeg's API is native (`crates/ssx3-game/src/movie.rs`:
`sceMpegInit/Create/AddBs/AddCallback/GetPicture/IsEnd/Reset`) over a native
MPEG-2 decoder (`crates/ssx3-mpeg`), which shares the IPU model's VLC
tables, dequantisation, IDCT and CSC (moved into that crate). EA's player
is unchanged: its "no data" callback still feeds `MPCh` chunks through
`sceMpegAddBs`; neither libmpeg's EE code nor the IPU runs. The 14 movies
use only progressive frame pictures with frame prediction/DCT (512x448,
zigzag, linear quantiser), which is what is implemented. Output: RGB32
macroblocks in libmpeg's column order. Gate: per-picture hashes against
the emulated path (`SSX3_MOVIE_HASH=1`, both paths) match except +-1-2 in
a few bidirectional macroblocks with a diagonal backward vector (the
native result is the standard's); unit test `eabig_native_decoder`.
Boot with movies, 3000 frames: 13.8 s -> 8.0 s. `SSX3_NATIVE_MPEG=0`
keeps libmpeg + IPU.

### N6: the rest of the PS2 (PC port) 🟡

Owner, 2026-10-18 night: after the movies, port what is still emulated;
"we essentially want a PC port"; game logic stays recompiled for now.
Inventory (native race, 6000 frames, `SSX3_MMIO_PROFILE=1` and a sampled
profile of the game thread `ee-thread-1`, busy 81% of the time):

| what | still emulated | per frame / cost | plan |
| --- | --- | --- | --- |
| frame build | the VIF1 model replays every captured `Cmd::Vu` and the models' VIF chains in RAM into VU1 memory images for the native VU programs (`frame::native`, `play_chain`) | ~10% of the game thread | typed geometry per draw (no VU memory images); chains converted once and cached |
| VU0 macro mode | each COP2 op goes through `dyn Vu0` into the VU model (stage, commit, flush) | ~9% of the game thread | direct native vector ops, results bit-identical (unit tests against the model) |
| flip | `Renderer_SetDisplayRegs` runs as guest code: GS privileged registers, the flip GIF packet, a GIF DMA, busy-wait loops on CSR | 1 call, ~120 CSR polls | native: registers and packet written directly, no DMA, no polling |
| scratchpad DMA | `SprStream` and others kick SPR to/from DMAs (ch 8/9) | ~25 | native copies at the stream's API |
| timers, INTC, DMAC status | registers the game reads | a few | keep as plain state (no timing model) |
| EE kernel | threads (host threads with a baton), semaphores, interrupt handlers | ~4 thread switches | keep; a native scheduler later |
| sound | `ssx3-snd` drives the SPU2 voice model as its mixer | 1 tick / 512 samples | already native; the SPU2 model is the mixer |
| memory | 32 MiB guest RAM, scratchpad, PS2 float semantics | everywhere | stays while game logic is recompiled |
| movies | done (N5) | | |

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
| N1 | ✅ | all seven programs: meshes, strips/ribbons, riders, Bézier terrain, particles, snow (type 3 0x2400 PRIM-only and 0x3bf0 line shape stay on the CPU); ~73% of VU calls on the GPU, merged into ~0.85M batches per 6000 frames; race replay frames match the CPU programs to rounding; native render 6.5 → 5.3 ms per frame |
| N2 | ✅ | stages A (every draw captured), B (frames from the buckets, no chain walk) and C (draw functions and RenderFrame without packets) done: every draw producer, the bucket links and RenderFrame are native in packetless mode. Earlier: D1 capture infrastructure (`ssx3_game::capture`, `ssx3_render::direct`): strips and ribbons drawn from native commands; model and rider vertex data resident on the GPU; uploads per race frame 3.2 -> ~1.5 MB; browser race 16-21 -> ~13-15 ms per frame, renderer 44% -> 29% of the worker. Next: D1 for riders / Bézier / particles, then D2 (the draw functions native, no packets: `DrawModel` first, ~3% of the browser worker) |
| N3 | 🟡 | step 1: incremental snapshots and page write generations; step 2: the typed frame built on the game thread from live memory, the renderer gets only the texture pages it reads (no 32 MiB snapshot) |
| N4 | 🟡 | functionally reached in native mode: GS detached, kicks complete without running, chains are bare `END`s; the kick thread is still guest code |
| N5 | ✅ | movies: native libmpeg API over `ssx3-mpeg` |
| N6 | 🟡 | PC port: the inventory above |
