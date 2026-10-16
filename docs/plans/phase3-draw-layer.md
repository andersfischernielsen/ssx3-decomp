# Phase 3: rendering at the game's draw layer

Branch `phase3-draw-layer`. Goal: frames go straight from the game's renderer
(`DeferMan` buckets, the typed `Frame` of `ssx3-render`) to wgpu. The PS2 DMA,
VIF, VU1 and GS models leave the frame path, which fixes the performance
problem: the GS-primitive path in `main` reproduces GS rasterization exactly,
so it costs as much GPU as emulation.

The design is `../investigation/phase3-render.md` §2-5 ("stage 5: flip the
master"). This file is the plan and progress log for carrying it out.

## What changes in kind

- **Exactness moves from pixels to game state.** Rendering does not feed
  back into the game, apart from two exceptions:
  - the flare occlusion query, which becomes native;
  - frame pacing, which is already overridden.

  So the hard gate is **RAM identity** in the race replay against the emulated
  build. Images are compared with tolerances (per-frame differing-pixel and
  mean-error metrics), as the design specifies, plus visual review. Bit-exact
  GS behaviour stays available as the reference path (`SSX3_RENDER=emu`, the
  default until the native one is accepted).
- **Hardware does the rasterizing.**
  - Depth comes from interpolated vertex Z, so there is no `frag_depth` and
    early depth rejection works.
  - Textures are decoded once into wgpu textures and sampled by the hardware.
  - Blending uses wgpu blend states.

## Architecture

- **`crates/ssx3-gfx`** (new) is the host renderer. It depends on `ssx3-render`
  (`Frame`, materials, GIF reader) and wgpu, not on `ssx3-ps2`'s GS. Its parts:
  - **`targets`:** the master colour and depth targets, at N× and any aspect.
  - **`texcache`:** TexMan textures, decoded from EE memory by id and
    generation:
    - formats 2/8/9/10/0xb/0xc, including the pre-swizzled T8/T4 and T8H in
      the alpha byte;
    - CSM1 CLUTs;
    - alpha 0x80 = 1.0;
    - mips.
  - **`geometry`:** per draw, payload → vertices.
    - Step 1 runs the existing native VU programs (`run_native`) on the CPU
      over each `VuCall` snapshot and converts their GIF output, together
      with the GIF streams of type-0 payloads, into vertices: window
      coordinates, Z, colour, STQ, fog.
    - Step 2 moves the per-material transforms into vertex shaders (vertex
      pulling, design §0).
  - **`pipelines`:**
    - GS blend presets → blend states;
    - alpha test with AFAIL = FB_ONLY as two pipelines;
    - depth `Greater`/`GreaterEqual`;
    - CLAMP → sampler address modes;
    - TEX1 → filters;
    - fog.
  - **`post`:** depth fog, bloom, colour filter, the display blit and
    brightness.
  - **`query`:** the LensFX occlusion query as a compute pass over the depth
    target (`LensFX_v04` override).
  - **Render targets inside payloads:** A+D writes that switch FRAME (for
    example the 128×128 rider-shadow texture), and slot-61 screen captures,
    become GPU-only cache entries.
- **Seams in the game** (overrides in `ssx3-game`):
  - `PS2GraphicsMan_RenderFrame` / the kick thread: hand the `Frame` to
    `ssx3-gfx` and complete the VIF1/GIF DMA without running it;
  - `LensFX_v04`;
  - FMV frame upload and the loading icon (outside DeferMan).

## Stages

Each stage leaves the default build untouched and is gated before it is
committed.

1. **Shadow renderer: 2D and menus.**
   - `SSX3_RENDER=shadow` builds `ssx3-gfx` frames next to the emulated path.
     The emulation stays master; the native image is dumped by the headless
     runner and can be shown in the window.
   - Scope: targets, the texture cache, GIF streams (type-0 payloads, the
     HUD, fonts, menus) and the display blit.
   - Gate: menu frames against the emulated screenshots, within tolerance.
2. **Shadow renderer: 3D materials** through the CPU native VU programs: the
   course (type 3), terrain (1/2), the rider (6/7), particles and snow (4/5),
   and render-to-texture.
   - Gate: race frames within tolerance; review the differences.
3. **Post effects and queries:**
   - depth fog, bloom and the colour filter;
   - the LensFX query against native depth.

   Gate: race frames within tolerance; flare glows match.
4. **Flip the master.** `SSX3_RENDER=native`: the frame chains are not run
   through DMA/VIF/VU/GS, and the window shows the native target. FMV and the
   loading icon go through their seams.
   - Gate: RAM identical to the emulated build over the full race replay.
   - Gate: frame time measured against the emulated build.
5. **Performance:**
   - vertex transforms on the GPU;
   - batching across buckets with equal state;
   - resolution scaling at the draw layer;
   - widescreen by widening the projection before the game derives its cull
     matrices (design §4.3).
6. **Frame loop:** with nothing on the EE timeline but game logic, pace
   `Render` by the host display while `Update` stays on the 60 Hz tick
   (`../investigation/phase3-frame-loop.md`).

Out of scope for this branch: removing the remaining hardware models outside
the frame path (pads and memory card via SIO2, IPU movies, the IOP kernel).
Those are later seams.

## Progress

(Entries are added as stages land.)

### 2026-10-16: stage 1 and part of stage 2 (`SSX3_RENDER=shadow`)

**Working:**
- Menus match the emulated GS within a few levels: mean difference 3-6, >32 on 1-3% of
  pixels.
- In the race:
  - geometry from all 4.9M VU calls, run through the native programs;
  - textures, including the pre-swizzled formats and CSM1 CLUTs;
  - terrain lightmaps (context 2, (Cd − Cs)·As);
  - render-to-texture: the six rider-shadow CT16 targets;
  - frame self-sampling (the colour filter / speed blur, from a snapshot of the colour
    target).

**Debug tools:**
- `SSX3_GFX_TRACE=<frame>`: every primitive with its state and resolved texture. The
  renderer counts frames about 55 behind the runner's VBlanks.
- `SSX3_GFX_TEXDUMP=dir`: decoded textures, as raw RGBA.
- `SSX3_GFX_RTDUMP=1`: render targets, written as PNG at each screenshot.
- `SSX3_GFX_PATH3=0..3`: which PATH3 chains are replayed.
- Headless `SSX3_DUMP_TEX=tbp:tbw:psm:w:h:path`: a VRAM texture from the emulated GS.
- `gs_replay --draws` now also writes each decoded texture's alpha channel.

**Resolved since (same day):**
- **Black snow under the projected shadows.** `Renderer_SetDisplayRegs` (0x382af0) builds
  a GS state packet at 0x4ffcc0 on every flip and sends it by normal GIF DMA, outside the
  chains: FRAME/ZBUF, XYOFFSET_1/2, FBA, TEXA (TA0 = 0x80), PRMODECONT, COLCLAMP, DTHE,
  DIMX, TEST, the frame clear. It is replayed at the start of each frame. The GIF phase-1
  chain (colour filter and the depth-fog setup, which set TEXA TA0 = 0) runs between the
  VIF phases, phase 2 at the end; `Built::phase2` marks where VIF phase 2 starts.
- **Black rider trails.** The trail passes run with FBA_1 = 1. FBA only sets bit 7 of the
  *stored* alpha; the blend still uses the true As. Batches that blend with As and store
  alpha are drawn as a colour draw and an alpha-only draw. The trail also uses the
  destination alpha test (DATE/DATM), done from a snapshot of the frame.
- **Coloured flares.** Their PSMT8H texture lives in the alpha bits of the display
  buffer, so a texture at a render target's address is that target only for colour PSMs.
- **Depth fog.** The game copies Z bits 8..15 into the frame's alpha (a CT16 view of the
  frame drawn from a Z16 view of Z24; measured with `fogprobe` on the software GS), then
  draws the frame's alpha as a PSMT8H texture through a 256-entry CLUT uploaded each frame
  (fog colour and amount per depth), ZTST GREATER. Natively the copy is dropped and the
  CLUT pass reads a snapshot of the depth buffer.
- **Lightmap light pools.** Blends without a D term whose As can exceed 1.0 (lightmap
  alpha up to 212) are drawn at half strength and doubled: the shader output is clamped
  to 1.0 before fixed-function blending.
- **Frame blend / display buffer.** When gm+0x59e8 is set, the flip packet starts with a
  block that copies (FIX 0x80) the finished draw buffer into the display buffer; that
  block runs at the end of the frame and `read_image` returns the display buffer.

**Comparing with the emulator:** the emulated GS shows the frame published one step
earlier, so the native image of dump `frame-1` is the one to compare with the emulator's
screenshot. Measured that way (mean difference, % of pixels off by more than 32):

| Frame | Mean | >32 |
| --- | --- | --- |
| 1500 (menu) | 0.76 | 0.0% |
| 3000 (menu) | 3.32 | 0.0% |
| 4500 (race start) | 3.25 | 0.0% |
| 6000 (race) | 1.03 | 0.0% |
| 7500 (race, flares) | 1.28 | 0.3% |

**Offline replay.** `SSX3_FRAME_DUMP=3` keeps the last three frames' inputs (EE RAM,
scratchpad, the carried chain state and the VU1 data memory); the headless runner writes
them next to each screenshot (`frameNNNNN-dump/frame-K.bin`, K = 0 newest). `gfx_replay
OUT.png frame-2.bin frame-1.bin` renders them in well under a second, identical to the live
render. The dumps hold game data and stay out of the repository.

More debug switches: `SSX3_GFX_SKIP=n,a-b` (drop draws), `SSX3_GFX_SKIPTEST=<test>[a|c|z]`
(drop batches by TEST, alpha-only/colour only, or keep without depth writes),
`SSX3_GFX_PROBE=x,y` (a pixel after every item that changes it), `SSX3_GFX_PLANLOG=<test>`,
`SSX3_GFX_TEXLOG=<frame>`.

**Still open (for later; none blocks the render cut):**
- **A few highlights** differ by a few levels (sun glare edges, some additive sprites).
- **CT16 render targets** keep only the 1-bit alpha; RGB is not quantised to 5 bits.
- **Host clock mode stalls** (`SSX3_HOST_CLOCK=1`): with a host that stays slower than
  60 Hz (headless `SSX3_SLOW_FRAME_MS=20`), the game makes no progress at all, even at the
  title (every thread waits), on every rendering path. Briefly the window default
  (2026-10-16), reverted. In the window it showed as a race that never started: a
  priority-6 thread spinning in `THREAD_yieldticks(0)` while the main thread starved.
- **Depth precision.** Depth comes from hardware interpolation, so it differs from the GS's
  by a few units at some pixels. Visually invisible, but anything that reads Z back (the
  light visibility queries) sees slightly different values, so RAM identity with the
  emulated build is not reachable on this path. The gate is behavioural: same inputs, same
  race (positions, timer, score).

### 2026-10-16 (later): native by default, the render cut

- **Native is the default** (`SSX3_RENDER=native`; `emu` and `shadow` remain). In gameplay
  the frame's VIF1/GIF chains complete at the DMA kick without being walked
  (`frame_sink::skip_chain`): the kick thread's state machine (DMA end interrupts,
  semaphores, VBlank) is unchanged, but no VIF, VU1 or GS work happens for the frame. While
  frames are published the emulated GS does not rasterize; movies and loading screens,
  which publish none, still draw there.
- **Light visibility** (`LensFX_v04`) is a native override counting the light's pixels in
  `ssx3-gfx`'s depth after the 3D phase, one frame late (the readback waits only on that
  frame's submission). Unit tests cover the count and the game's off-screen formula.
- **PATH3 ordering** follows the GS: each phase's GIF chain is queued masked, each
  MSKPATH3 0 releases one packet, and an unmasked stream end drains the rest. This put
  the colour filter before the HUD (the HUD tint) and brought the race frames to a mean
  difference of about 1 against the emulator.
- **Depth-fog CLUT** is taken when the fog pass is drawn (the same VRAM is reused later in
  the frame: black sky at frame 5500).
- **Performance**:
  - VU1 calls run in parallel chunks, which start only at draw boundaries between
    different programs (programs pass state within a draw, and terrain across draws;
    `SSX3_GFX_VUCHECK` compares every call with a sequential run, `SSX3_GFX_SEQ_VU=1`
    forces it);
  - the vertex upload uses raw bytes;
  - the debug switches are read once.

  The race replay runs at about 8.7 ms per frame, against 15 ms emulated.
- **Native sound, native files and native VU1 are the defaults too.** VU1 native is
  identical to the microcode. Native sound and the native light query shift how the
  game's threads interleave, so a scripted race differs from the emulated one while each
  path stays deterministic.
- **Threads.** At each publish the game thread only copies EE memory (32 MiB in four
  parallel parts, about 1 ms) into a pooled snapshot and reads the four chain heads it
  needs for the DMA cut. A frame thread builds the typed frame from the snapshot (carrying
  the VIF state in order), and the render thread renders it (one frame queued per stage;
  the game waits only if the renderer falls further behind). The light queries read the
  latest depth from a shared slot, and the window presents a shared texture, so neither
  waits on a render in progress. Race replay to frame 7500: 45 s (emulated path: about
  110 s); the render stage, about 5 ms per frame, sets the unpaced pace.
- **Resolution.** `SSX3_RENDER_SCALE=N` renders at N× (targets, render targets and
  scissors scaled; the light-query depth is sampled back at GS resolution).

### 2026-10-17: no GS emulation, any size and aspect

- **The GS is gone from the runtime path in native mode.** Everything the game shows comes
  through published frames: menus, the memory-card check, the EA BIG / THX / intro movies
  (drawn as textured sprites by the game's renderer) and loading screens. Outside them the
  GS only ever received the flip packet, which `ssx3-gfx` replays from RAM. So the GS core
  is detached (`Mtgs::detach`): commands are dropped, readbacks return nothing. The
  guest-visible GS stays on the guest side of `Mtgs`: CSR and IMR, the privileged
  registers, SIGNAL/FINISH/LABEL, PATH3 arbitration and GIF_STAT. Measured over boot
  (frames 1-1800 with movies) and loading screens with the emulated GS still attached:
  no GIF traffic other than the flip packet once every published frame's chains are
  skipped. (Chains are skipped whenever their heads belong to a frame the sink drew, not
  only while frames come every VBlank: loading screens publish now and then.)
- **Any output size and aspect, and field of view.** `ssx3-gfx` renders to any size
  (the window's by default, `SSX3_RENDER_SIZE=WxH`). GS pixels map to it with separate x
  and y factors (render targets keep an integer scale; the display buffer follows the main
  target). The game's view is widened through its own widescreen support:
  - `PS2GraphicsMan_SetPerspective` scales its projection by gm+0x6ba0 (x) and gm+0x6ba4
    (y) when gm+0x6b94 is 0..2;
  - 0x377950 writes them when the option changes (mode 2, anamorphic: x 0.75);
  - the frame sink writes mode 2, x = (4/3)/aspect/fov and y = 1/fov at every publish, so
    the next frame's projection and its cull matrices match;
  - `SSX3_FOV` is the field-of-view multiplier.

  The HUD (draws in layers 9 and up) is squeezed horizontally around the centre so its 4:3
  art keeps its shape; menus come out pillarboxed.
- **Not done: frame rate independent of the 60 Hz logic.** The game already decouples
  logic from rendering (several fixed updates per render when behind). Rendering faster
  than 60 Hz needs interpolation between consecutive typed frames:
  - match draws across frames (static geometry: by packet range, which is stable);
  - interpolate their `MatrixRecord::world_view` (the VU1 header the programs read) and the
    skinned meshes' bone palettes;
  - re-run the VU programs with the interpolated inputs.

  Interpolating the camera alone makes the riders judder at speed. This is the next design
  task for "detach frame rate from logic"; the host clock mode (real-time catch-up) still
  stalls with a host that stays behind and stays off.

### 2026-10-17 (night): frame interpolation

- **Rendering faster than the 60 Hz logic.** `ssx3-gfx` draws `n - 1` in-between images
  before each game frame (`set_subframes`; the window picks `n` from the monitor's
  refresh rate, `SSX3_FPS` overrides). An in-between image is frame N drawn again with its
  objects' VU headers (worldView, VU qw0-3) replaced by a blend of frame N-1's and N's
  (`interp.rs`):
  - each MatrixRecord of N is paired with one of N-1: the same world matrix (static
    objects; the closest view for split screen), else the nearest world position among
    records of draws with the same state and textures (riders);
  - the VU programs re-run with the substituted header, so culling and clipping are
    consistent within each image (the first attempt, blending GS vertex kicks per draw,
    cracked: terrain draws change their kick counts frame to frame);
  - the GS register state, VU memory and render targets are saved before the in-between
    images and restored after, so the frame's own image is unchanged (checked: identical
    bytes with and without interpolation, `gfx_replay --interp K`).
  - Screen-space draws (HUD, flares, particles built on the EE) stay at frame N.
- The window steps the game on its own 59.94 Hz clock and presents every vsync, showing
  the in-between images in turn (`Renderer::present_view_at`). One frame of extra latency.
