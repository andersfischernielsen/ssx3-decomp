# Terrain lighting (future work)

Not started. Follows from the shadow-map work (`shadow-maps.md`).

## What the game does

Snow Jam's terrain (Bezier patches) is drawn in up to three layers:

| Layer | GS blend (ALPHA) | What it holds |
| --- | --- | --- |
| base | `0x2a`: Cs | the snow texture, near white |
| bake | `0x81`: (Cd − Cs)·As | a smooth darkening map: slope shading, tree and object shadows, the blue tint in shade (it subtracts yellow-orange from white) |
| detail | `0x58`: Cs·Ad + Cd | additive sparkle / shine |

Every vertex of a patch gets the same colour, so the bake layer is the
terrain's only lighting. Without it the snow is flat white
(`SSX3_SHADOW_DEBUG=no-sub`; `sub-only` shows the layer alone).

## Replacing the bake with our own lighting

1. **Smooth terrain normals.** The receivers now use flat per-triangle
   normals (screen derivatives). Derive normals from the patch grid in the
   Bezier vertex shader (neighbouring grid points, or the patch's
   derivatives).
2. **A shading model.** Sun N·L with the key light (`shadow.rs`), a sky
   fill with the cool tint in shade, the sun shadows, ambient occlusion.
   The code is small; matching the art is the work, per course: the bake
   also holds hand-placed tints (ice regions) a physical model will not
   reproduce.
3. **Meshes and riders** keep their SH lighting (per object, already
   dynamic).

Middle ground until then: keep the bake and weaken our shadows where the
bake already darkens, so they add only what it lacks (moving riders, objects
and angles it did not bake).
