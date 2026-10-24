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
- Meshes without depth writes do not cast.
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
- 2026-10-24: flicker, found with dense captures (every 2 frames,
  5800-6000) and receiver debug views (`SSX3_SHADOW_SHOW=cascade|factor|ndl`).
  - Mixed-LOD terrain (mat2) went to the CPU programs in some frames (all
    5,751 such calls in the race: `fan_tags_ok`, fan slots holding another
    program's data), and CPU draws neither cast nor received. The GPU draws
    now use the program's own fan tag when the slot holds something else,
    so mat2 always plans for the GPU (race gate identical). No triangle draws
    are left on the CPU (`SSX3_SHADOW_DEBUG=gpustats`): only mat6 bone
    setup, morph, coefficient scaling and the ctx2 kick, mat3's PRIM kick
    and its cubic lines.
  - What still changes from frame to frame: casters the game stops drawing
    (a rider leaving the view, the rail below the camera).
  - Terrain layers on Snow Jam: base (Cs), bake ((Cd − Cs)·As, the terrain's
    only lighting), detail (Cs·Ad + Cd). Multiplying every layer's source by
    the shadow factor shadows the sum (the blends are linear). See
    `terrain-lighting.md`. Debug: `SSX3_SHADOW_DEBUG=no-sub|no-add|sub-only|blend`.
- 2026-10-24: casters off screen, quick pass (option 1). With shadows on,
  `Nash_CullNode` (Rust, world.rs) widens the frame's cull matrix once per
  frame, in RAM (world +0x10) and in VU0 (qwords 0x40-0x43, the patch and
  instance tests): x, y of its clip output / k (`SSX3_SHADOW_CULL=k,d`,
  default 2,0; `=0` off). The shadow pass draws terrain strip triangles and
  rider skin whole, ignoring the camera's clip test, and none of the
  clipped fan copies. Terrain sent to the GPU +40 % at k = 3.
  - A margin behind the eye (d > 0) draws garbage: geometry behind the eye
    counts as inside the widened volume and is drawn unclipped (mirrored).
  - The race plays out differently with it on: the extra geometry costs EE
    cycles. Off: race gate identical.
  - Behind the camera, and the views from big jumps, need the native caster
    pass (option 2): next.
