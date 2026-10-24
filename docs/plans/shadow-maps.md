# Shadow maps (branch `shadow-maps`)

Goal: real sun shadows in the native renderer, as an F10 option that is off
by default; later, ray-traced shadows on the same groundwork. Snow Jam is the
test course (the reference race replay). The plan is loose on purpose: we
iterate, and this file records what we learn.

Rules: only `ssx3-gfx` and the frame data it reads change; game logic stays
as it is; the race gate stays identical; with the option off the screenshots
match `main`.

## Direction

1. **World-space scene.** Each GPU call carries a model→clip matrix; the
   terrain calls give the camera's view-projection. Recover each call's world
   matrix and check it on Snow Jam (debug view: the scene redrawn from a free
   camera).
2. **The sun.** Find the course's light direction (world data, the riders'
   lighting or the lens flare) and check it against the baked shading.
3. **First shadows.** One shadow map from the sun over what the camera
   draws; multiply it into the 3D image with a strength setting. See how it
   looks next to the baked lighting and the game's rider shadow.
4. **Then, as needed:** cascades, casters from off screen (a native pass over
   the world data), rebasing near the camera for precision, filtering, the
   rider shadow.
5. **Later:** ray-traced shadows/AO (a BVH traced in a compute shader).

## Open questions

- Casters the game culls (off screen, behind the camera) are missing.
- CPU-path draws (35 batches with depth writes at frame 5100) and meshes
  without depth writes do not cast.
- The game's own rider shadow is still drawn as well.

- Does the baked lighting agree with the sun's direction on each course?
- How much of the terrain's shading is baked shadow (double darkening)?
- Keep the game's projected rider shadow or replace it?

## Log

- 2026-10-24: branch and worktree (`../ssx3-shadow-maps`) created from `main`.
- 2026-10-24: first working version (`SSX3_SHADOWS=1`, F10 "Sun shadows").
  - World positions: every GPU program ends in `gs_vertex` with a
    homogeneous GS screen position; the terrain camera's inverse turns it
    into a world position (relative to a point near the camera). One
    matrix for all programs; checked with debug views (`SSX3_GFX_SHOW=
    world|normal|light-uv|light-depth|shadow-map`). The world's z is up.
  - Shadow pass: meshes, riders and terrain drawn again from the sun into
    an 8192² atlas, 4 cascades (half widths 24000/3ᵏ, texel-snapped), alpha
    cut-outs discarded.
  - Receivers look the map up in their own fragment shader (forward), not in
    a full-screen pass: the depth buffer also holds translucent things
    (snow spray, trails), which hid or misplaced shadows ("cut-off"
    shadows). Surfaces facing away from the sun are left to the baked
    lighting.
  - The sun: the painter's lens-flare sun (Snow Jam: elevation 9.15°,
    azimuth 160°, `WorldPainter` view 6 +0x08/+0x10) is not the light the
    course is shaded with: nearly everything facing the camera was
    back-lit. The lit meshes' SH lighting (linear terms) gives the key
    light, ≈ (0.07, 0.59, 0.80) on Snow Jam (elevation ≈ 53°); used by
    default, eased per frame; `SSX3_SUN=az,el` overrides.
  - Board trails and wakes (mat3 strips) neither cast nor receive: they are
    blended decals.
  - Not shadows: a triangle sawtooth on the terrain near the camera is in
    the original native render too (renderer artefact, open).
