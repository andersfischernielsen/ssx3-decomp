# Modern graphics (branch `modernize-graphics`)

Goal: optional modern rendering on top of the native renderer, each a
setting in the F10 overlay (which now scrolls). The defaults are the
original: with every option at its default the image and the game are the
same as before.

| Setting (F10) | Variable | What it does | Where |
| --- | --- | --- | --- |
| Anti-aliasing | `SSX3_AA=fxaa` | FXAA 3.11 on the 3D image before the HUD | `post.wgsl` `fs_fxaa` |
| Texture mipmaps | `SSX3_MIPMAPS=1` | Mipmap chains for the game's textures (the original has none: distant textures shimmer and anisotropic filtering had nothing to work with) | `tex::mipmaps`, alpha-weighted box filter |
| Texture sharpness | `SSX3_LOD_BIAS=±n` | Texture LOD bias (with mipmaps) | `draw.wgsl` `textureSampleBias` |
| Terrain detail | `SSX3_TERRAIN=2/4` | Uniform-LOD Bezier patches tessellated that many times finer: the patch through the game's n×n grid (Lagrange interpolation of degree n − 1 reproduces the bicubic patch exactly), the border kept on the game's samples so neighbours at other LODs stay watertight | `ssx3_render::bezier::gpu::fine_grid`, `mat1` draw and pass-2 ST |
| Rider detail distance | `SSX3_DRAW_DIST=1.5/2/3` | The riders' detailed models further out (the LOD distance scale at 0x49b82c, used only by `Rider_ComputeLOD`). The terrain's LOD table (0x4a2a10) is left alone: finer grids further out broke the terrain (large untextured areas); terrain detail refines the grids instead | `ssx3_game::detail` (memory written at VSync only when not 1) |
| Ambient occlusion | `SSX3_AO=1` | SSAO from reconstructed world positions (12 samples in an 80 cm radius above the surface's tangent plane, edge-aware blur), multiplied into the 3D image before the game's post passes; without a terrain camera a depth-only variant | `post.wgsl` `fs_ao*`; camera = the first terrain strip call's matrix and guard mapping (`gs::Cam`), rebased near the camera (world coordinates are ≈ 10⁵ cm, too large for f32) |
| Bloom | `SSX3_BLOOM=modern/off` | The game's bloom pass replaced by a wide soft one (bright pass above snow white, 6-level dual-filter pyramid, added in linear light), or off | the game's bloom batches are skipped by their post-pass tag (`Built::path3_post`) |
| Snow sparkle | `SSX3_SNOW=1` | Glints on bright, unsaturated terrain texels, anchored in texture space | `draw.wgsl` flag 2048 |

New art (normal maps, materials, models) is out of scope. Higher shadow
resolution, soft shadows, screen-space sun shadows and colour grades were
built and then removed at the owner's request.

## How the passes fit in

- Batches carry a `Tag` (terrain, HUD, the post pass they belong to). The post passes' PATH3 packets now carry their kind.
- The renderer's own passes run in the item loop: AO before the first
  post-pass item (fog, bloom, colour filter), bloom/FXAA before the first
  HUD item. They write RGB only: the alpha channel is the
  GS's destination alpha, which later draws use.
- Settings apply live (`Renderer::set_options`; detail distance through
  `ssx3_game::detail::set`). Headless runs read the same variables, so each
  option can be checked with screenshots.

## Verification

- Defaults: the race replay's screenshots (2× resolution, 11 frames) match
  the `main` build's; one menu frame differs in 2 edge pixels by 2 levels.
- Each option on its own and all together, against the defaults
  (screenshots of the race replay). Found and fixed this way: the terrain
  LOD table (dropped), bloom on all snow (threshold above snow white),
  AO darkening every slope (now oriented by the surface normal), AO banding
  (f32 precision, units in cm).
- Debug: `SSX3_GFX_SHOW=ao` shows the AO mask as the image.
- Unit tests: mipmap chains, the 4×4 inverse, the settings overlay's
  scrolling, the detail module's constants.
