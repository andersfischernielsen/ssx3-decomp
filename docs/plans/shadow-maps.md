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

- Does the baked lighting agree with the sun's direction on each course?
- How much of the terrain's shading is baked shadow (double darkening)?
- Keep the game's projected rider shadow or replace it?

## Log

- 2026-10-24: branch and worktree (`../ssx3-shadow-maps`) created from `main`.
