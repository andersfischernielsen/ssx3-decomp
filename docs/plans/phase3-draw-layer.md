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
