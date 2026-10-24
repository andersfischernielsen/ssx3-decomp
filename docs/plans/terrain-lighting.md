# Terrain lighting

In progress (2026-10-24, branch `shadow-maps`): the F10 setting "Native shadows"
(`SSX3_SHADOWS`) replaces the bake. Follows from the shadow-map work
(`shadow-maps.md`).

Goal: a new terrain lighting model, tuned per course, that makes every
existing track look the way it does now, but is computed live with modern
native graphics (real sun and sky light, real shadows) instead of the
game's baked layer.

## What it replaces

Snow Jam's terrain (Bezier patches) is drawn in up to three layers:

| Layer | GS blend (ALPHA) | What it holds |
| --- | --- | --- |
| base | `0x2a`: Cs | the snow texture, near white |
| bake | `0x81`: (Cd − Cs)·As | a smooth darkening map: slope shading, tree and object shadows, the blue tint in shade (it subtracts yellow-orange from white) |
| detail | `0x58`: Cs·Ad + Cd | additive sparkle / shine |

Every vertex of a patch gets the same colour, so the bake layer is the
terrain's only lighting. Without it the snow is flat white
(`SSX3_SHADOW_DEBUG=no-sub`; `sub-only` shows the layer alone).

## The model

1. **Smooth terrain normals**, from the patch grid in the Bezier vertex
   shader (neighbouring grid points, or the patch's derivatives); the
   receivers use flat per-triangle normals today.
2. **Shading:** base texture × (sun colour · N·L · sun shadow + sky fill),
   the fill tinted cool in shade, ambient occlusion for contact and creases,
   the detail layer kept as the game draws it.
3. **Meshes and riders** keep their SH lighting (per object, already
   dynamic); their key light is the sun the terrain uses.

## Per-course tuning

Each course gets a small parameter set: sun direction and colour, sky fill
colour and strength, shade tint, shadow softness and strength, AO strength,
exposure. The target is the course as it looks now:

- **Start from the bake.** The bake layer is a picture of the intended
  lighting. Fit the parameters to it: render the course with the bake alone
  and with the model alone, in the same views, and solve for the parameters
  that match it best (least squares over many frames). The key light from
  the meshes' SH lighting is the starting sun.
- **What the model cannot do** (hand-painted tints such as ice regions,
  darkening the art relies on): keep that part of the bake as a tint map,
  its shadow part removed (where our shadow map says lit, the bake's
  darkening is art, not shadow).
- **Check against the original:** replay frames from each course, ours next
  to the original look, compared with a tolerance per region (snow, shade,
  ice). One tuning file per course, next to the renderer.

Courses in order of play: Snow Jam first (the test course), then the rest of
Peak 1, then Peaks 2 and 3 and the backcountry.

## Until then

Keep the bake and weaken our shadows where the bake already darkens, so
they add only what it lacks (moving riders, objects and angles it did not
bake).

## Status

- Smooth terrain normals: `bezier::gpu::with_normals` (faces summed per grid
  point, keyed by position), a third qword per vertex (`DATA_STRIDE`),
  passed to the fragment shader.
- The model: `gpuvu.wgsl` `terrain_light` on the base layer (flag 16384);
  the bake layer (ALPHA 0x81) is not drawn with "Native shadows" on
  (`SSX3_SHADOW_DEBUG=baked` keeps it).
- Per-course colours: `shadow.rs` `COURSES` by world file name (cWorldCache
  +0x304, `|data/worlds/BAM.ssb` = Snow Jam), `SSX3_LIGHT` overrides.
- Fitting: reference shots of all 20 race/freestyle courses taken once
  (scenario inputs, every 150 frames of play, the default look), then
  `tools/fit_lighting.py REF NOSUB NORMALS SUNLIT` per course.
