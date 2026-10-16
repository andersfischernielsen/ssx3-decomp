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

**Still open:** the HUD and a few highlights differ by a few levels; CT16 targets keep only
the 1-bit alpha (RGB is not quantised to 5 bits).
