# World lights from RAM (for native point / spot lights)

How to enumerate the placed world lights (Metro City street lamps and neon,
Snow Jam lodge lights, ...) in EE RAM at frame build, their record layout,
and the exact formula the game uses to turn one into light. Verified against
Ghidra, `generated/ee`, the hand ports (`visualfx/worldlightman.rs`,
`render/particle.rs` `Irradiance_*`, `world/worldview.rs` resolver slots) and
the race dump `target/gate/base/ds1.bin` (Snow Jam, frame 6000): the walk
below finds **188 lights** (100 spot, 88 point) whose fields decode cleanly.
No Metro City dump was available; the code path is world-independent.

Confidence: **H** = read in code and checked in the dump, **M** = read in
code or inferred from consistent data, **L** = guess.

## 1. Where the lights live and how to walk them

There is no global light array. Each light is a **section resource**
(world chunk resource type 6) that the chunk resolver links straight into the
Nash octree's **effects list** (node `+0x28`); the record is its own list
node (no wrapper).

| Step | Detail | Conf |
|---|---|---|
| Roots | `g_App = *(0x4a28a8)` → `+0x84` GameModule → `+0x20` Nash. Octant record `o` (0..7) at `Nash + 0x14·o` = `{i32 level; i32 x, y, z; Node* root}`, root at **record +0x10** (so octant 0's root is `Nash+0x10`, octant 1's `Nash+0x24`, ...) | H |
| Node (0x2c) | `+0x00..+0x1c` 8 children, `+0x20` instances, `+0x24` patches, `+0x28` **effects** (SpatialHash list type 2) | H |
| Effects list | singly walked through `+0 next` (`+4 prev`); entry kind at **`+8`**: 1 = world rail spline node, 2 = RailModifier, 3 = FlexRail node, 5 = ?, **6 = light**, 7 = fog-particle volume, 8 = lens flare. Dump: `{6: 188, 1: 820, 8: 133, 5: 23, 7: 7, 2: 2}` | H (6, 7, 8, 1-3), L (5) |
| Insert | `cPS2ChunkResolve_Resolve` 0x3aae68, case 6 (0x3ab094): resolver slot 21 `cPS2ChunkResolve_v21` 0x3aac50 accepts the light, then `entry+8 = 6`, `SpatialKey_FromAABB(+0x44..+0x5b)`, `SpatialHash_Insert(root, 2, light, key)` | H |
| Accept rule (v21) | only `(+0xc & 0x100) == 0` **and** type `+0x10` ∈ {1, 2}; clamps radius `+0x1c` to ≤ 5000 (gp-0x23b8). Directional (0) / ambient (3) records and flag-0x100 records never enter the Nash | H |
| Remove | `cPS2ChunkResolve_Release` 0x3ab498 → `SpatialHash_Remove` 0x3284b8 when the section is released. So the Nash holds exactly the lights of resolved sections; no extra section gate is needed (the SH code applies none) | M |

```
for o in 0..8: walk(u32(nash + 0x14*o + 0x10))
walk(n): if n == 0: return
         e = u32(n + 0x28)
         while e: if u32(e+8) == 6: emit(e); e = u32(e)
         for i in 0..8: walk(u32(n + 4*i))
```

Cost: 2203 nodes / ~1200 effect entries on Snow Jam; a full walk per frame is
cheap. The game itself only ever looks at lights near a point:
`OverlapList_GatherType2` 0x331450 collects the effects lists of cells
overlapping a box (±1000 around an instance; the rider's AABB for riders),
then keeps the strongest few (§3).

## 2. The light record (0x70 bytes, section memory)

Positions are **world space** (same frame as instance `+0x40`, Z up, values
around ±3·10⁵ on Snow Jam); nothing transforms them at resolve time.

| Off | Type | Meaning | Dump | Conf |
|---|---|---|---|---|
| 0x00 / 0x04 | ptr | Nash list next / prev | | H |
| 0x08 | u32 | entry kind = 6 (written by the resolver) | 6 | H |
| 0x0c | u32 | flags; 0x100 = not a dynamic light (rejected) | 0 | H (0x100 test), L (meaning) |
| 0x10 | i32 | type: **1 spot, 2 point** (0 directional, 3 ambient exist in the format, see `LightMan_v05` 0x38dc70, but never reach the Nash) | 1 / 2 | H |
| 0x14 | f32 | intensity (spot falloff 2: ~500..5000; spot falloff 0: 1..6; point: 40..150) | | H |
| 0x18 | f32 | luminance of the colour (0.299 r + 0.587 g + 0.114 b): used **only for ranking** (`LightContribution`), not for the light itself | e.g. rgb (1,0,0) → 0.299 | H (use), M (meaning, fits every dump record) |
| 0x1c | f32 | radius: hard cut-off (`d² > r²` → no light), ≤ 5000. Points: = `100·sqrt(I·lum/0.05)` (the falloff-2 distance where light drops to 0.05); spots: 5000 | | H |
| 0x20..0x28 | f32 ×3 | colour r, g, b, **0..1 floats** (normalised: max component 1) | (0.404, 0.396, 1.0) | H |
| 0x2c..0x34 | f32 ×3 | spot axis, unit, pointing from the light **into** the lit area (Z up: lamps ≈ (0,0,−1)). Points: (1,0,0), unused | | H |
| 0x38..0x40 | f32 ×3 | position | | H |
| 0x44..0x58 | f32 ×6 | AABB min xyz, max xyz (used only for the octree key) | | H |
| 0x5c | f32 | cos inner cone angle | 0.9659 | H |
| 0x60 | f32 | cos outer cone angle (≤ inner) | 0.8192 | H |
| 0x64 | i8 | distance falloff power: 0 = none, 1, 2, 3 (others = none) | 0 / 2 | H |
| 0x65 | i8 | spot exponent on the cone cosine (0..56 seen; negative = 1/cos^-e) | 0, 1, 5, 20 | H |
| 0x66..0x6f | | identical bytes in every record (`3940de2b540010000000`), unused by the light code | | L |

Dump mix (type, falloff, exponent): spots (1,2,0) ×62, (1,0,5) ×9, (1,0,1) ×7,
(1,2,20) ×6, (1,0,20) ×4, a few others; points all (2,2,·) ×88. One spot sits at
the origin with intensity 735 (a template?); harmless.

## 3. How the game turns a light into lighting

`Irradiance_AddDynamicLight` 0x38a6a8 (point / spot vectors 0x38a618 /
0x38a530); constants gp-0x26e4.. = 5000, 3750, 0.0008 for both types. `P` =
sample point, `L` = light:

```
v   = L.pos - P;  d = |v|                 // d == 0 -> d = 1, 1/d = 1
if d*d > L.radius^2: return 0
l   = v / d                               // direction towards the light
// spot only:
cs  = -dot(l, L.dir)                       // = cos(angle off the axis)
if cs < L.cosOuter: return 0
// both:
lin = (L.radius < 5000 || d <= 3750) ? 1 : 1 - (d - 3750) * 0.0008   // fade 3750..5000, only radius-5000 lights
x   = 100 / d
fall = x >= 1 ? 1 : {1: x, 2: x*x, 3: x*x*x, else: 1}[L.falloff]
k   = fall * L.intensity * lin
// spot only:
if cs >= L.cosInner: k *= cs^L.exponent
else:                k *= L.cosInner^L.exponent * (cs - L.cosOuter) / (L.cosInner - L.cosOuter)
k   = clamp(k, 0, 5)
add directional light (direction l, colour L.rgb * k) to the order-2 SH set
```

`Irradiance_AddDirectional` 0x389520 / `AddDirectionalScaled` 0x389308 use the
Ramamoorthi–Hanrahan irradiance constants (c1..c5 = 0.429, 0.511, 0.743,
0.886, 0.248 with the Y_lm factors), so the stored SH evaluate to roughly
`L.rgb·k·max(0, n·l)` (band-limited: a soft Lambert, peak ≈ 1.06×). A shader
match: **Lambert `max(0, n·l)` × colour × k** with the k above; no 1/π. (H for
the formula, M for the SH-to-Lambert equivalence.)

Who receives it (important for scope):

| Receiver | Selection | Conf |
|---|---|---|
| Model instances with `inst+8 & 0x4000` (SH-lit; 18 of ~3500 on Snow Jam) | `PS2GraphicsMan_DrawModel` 0x37e238 → `WorldLightMan_LightInstanceCB` 0x2f5148 → `GetInstanceLighting` 0x2f5400: one sample at the instance centre (`+0x40`, or WorldObj node 0), re-sampled after > 10 units of motion; base SH from the painter's Irradiance table entry, plus the **4 strongest** lights (`AddPointLights` 0x2f5af0 → `FindStrongestLights` 0x2f5b68) from the region's overlap list (box ±1000, 8 cached regions, 32 cached instances) | H |
| Riders | `Rider_FindNearbyLights` 0x122088: the **8 strongest** lights at the hips from the rider's OverlapList (`+0x860`) into `rider+0x794`; `Rider_BuildLightSet` 0x1220d8 adds them at the hips | H |
| Terrain (Bézier patches), static unlit instances | **none**: they use baked vertex colours; the game never lights terrain with these records | H |

Ranking (`WorldLightMan_LightContribution` 0x2f5d30, `LightDistance`
0x2f6168): the same `k` before the clamp, times `+0x18` (luminance); for a
renderer that shades every light per pixel the ranking does not matter.

So real-time terrain lighting from these records is an **enhancement**, not
a reproduction: the baked terrain colours may already contain the lamps
(unknown, L), so expect to need a strength scale or a per-world switch.

## 4. Runtime changes

| Question | Answer | Conf |
|---|---|---|
| Flicker / toggles | No code found that writes a light record after resolve (only the radius clamp in v21). Neon "flicker" is not in these records | M |
| Streaming | Lights appear / disappear with their section (Resolve / Release). `World_ResetLights` 0x3ad1a8 (from `cWorldCache_StreamerUpdate`) drops the WorldLightMan region cache and the riders' light lists on stream changes | H |
| Enabled flag | None besides `+0xc & 0x100` (filtered at insert). Every type-6 entry in the Nash is live | M |
| Script override | Only the ambient SH (`WorldPainter_SetIrradianceOverride` gp+0x24c0), not the point lights | H |

## 5. LightGlow is not lights

`tWPIGD_LightGlow` (world-painter region data, ctor 0x2bc830, getters
0x2eedb0..0x2eef60) holds 7 per-view floats that
`PS2VisualEffectsMan_ApplyWorldPainter` 0x2f00a0 copies to 0x4a43d4..0x4a43f4
and `PS2GraphicsMan_ResetBloomParams` 0x36c740 copies to
`PS2GraphicsMan+0x6cd4 + view·0x1c`: **screen bloom / glow parameters** (H for
the data flow, M for "bloom" from the consumer's name). Halo sprites at lamps
are the effects-list kind-8 records (§6). Neither carries
light positions for shading.

## 6. Light-glow flares (effects-list kind 8)

The halo sprites at lamps and neon are the kind-8 effects-list records, which
are separate from the type-6 lights. They are a world resource (chunk
resource type 7). The resolver case 0x3ab0dc calls slot 20
`cPS2ChunkResolve_v20` 0x3aabd8, sets `+8 = 8` and inserts the box
`+0x28..+0x3c` (SpatialHash list type 2). Every frame
`Nash_CollectEffects` 0x22a770 appends the kind-8 entries of visible nodes to
`gm+0x7bc4` (count `+0x7bc0`); there is no per-record cull there.
`GameModule_RenderLensFlares` 0x22c790 then calls `LensFX_AddSources`,
`LensFX_PrepareView` (project) and `LensFX_DrawFlares`.

Record (0x50 bytes):

| Off | Type | Meaning | Conf |
|---|---|---|---|
| 0x00 / 0x04 | ptr | Nash list next / prev | H |
| 0x08 | u32 | kind = 8 | H |
| 0x0c | u32 | flags. **`& 0x70` = size class**: 0x10 → 180, 0x20 → 100, 0x40 → 350 world units (`LensFX_GetFlareSizeForRes` 0x2e27e8); any other value → size 0, so the flare is drawn at zero size. Other bits unused by LensFX | H (0x70), L (other bits) |
| 0x10..0x18 | f32 ×3 | colour r, g, b. `LensFX_DrawFlareQuad` **normalises it to unit length** before drawing, so only the hue counts: the record carries no brightness | H |
| 0x1c..0x24 | f32 ×3 | world position (sprite centre) | H |
| 0x28..0x3c | f32 ×6 | AABB = centre ∓ size, written by v20 (Metro: ∓100 for class 0x20) | H |
| 0x40 / 0x44 | f32 | per-view visibility 0..1 (views 0 / 1), zeroed at resolve. `LensFX_UpdateSourceVisibility` 0x2e3130 writes the renderer's occlusion-query result here (slot `+0x24` of the object at `LensFX+0x649c`; when no result is ready it keeps the previous value) | H (writes), M (occlusion query) |
| 0x48 | u32 | e.g. 0x5553c5: never read by LensFX, Nash or the resolver code read here (an id or hash?) | L |
| 0x4c | u32 | e.g. 0x21: never read by that code either | L |

Drawing (`LensFX_DrawFlareQuad` 0x2e2868, per projected source that is on
screen):

- The sprite is a world-space billboard at `+0x1c` with width = height =
  the class size (100 / 180 / 350).
- Colour is `(alpha, normalize(+0x10))`. When the source passed projection
  (`src+0x2e`), alpha is the record's view visibility; otherwise it is 1.0.
  Alpha ≤ 0 means the flare is not drawn.
- The texture comes from the renderer (`PS2GraphicsMan+0x1028` for class
  0x20, `+0x1024` for 0x10 / 0x40), so it is per size class, not per
  record. Additive-style blend state.
- Projection (`LensFX_ProjectSource` 0x2e2b00): the flare is skipped when the
  centre fails the renderer's view test or its depth·2⁻²⁴ is ≤ 0.005. For the
  occlusion probe the point is moved 80 / 100 / 200 units towards the camera
  (classes 0x20 / 0x10 / 0x40), and the probe rect is 1..16 × 1..8 pixels,
  scaled with depth.
- (H for the data flow; M for the blend and texture names.)

| Question | Answer | Conf |
|---|---|---|
| Size / intensity | Only the size class (0x70 bits). The record and the class textures have no intensity, and the colour is normalised. For a point light, derive the strength from the class (e.g. radius ∝ 100 / 180 / 350) and take the colour hue from `+0x10` | H |
| Not lamps? | The sun flare is not in the Nash: it is `LensFX_DrawSun` 0x2f4a08 at `LensFX+0x6070`, gated by gp+0xa28. The screen-space ghost set is `LensFX_SetDefaultFlares` / `DrawFlareSet`. Every kind-8 record is a placed world glow. Whether some mark non-lamps (e.g. snow sparkle) can only be judged from placement | H (sun), L (rest) |
| Runtime toggles | Only the per-view occlusion visibility (`+0x40/+0x44`) and section streaming (Release removes them). No time-of-day input. Fog/LightGlow painter values do not touch them | M |
| Relation to type-6 lights | None in code: separate resources, and no pointer either way. Lamps that have both sit at nearby positions | M |

Generated functions studied for this (not hand-ported): 0x22c790
GameModule_RenderLensFlares, 0x22a770 Nash_CollectEffects, 0x2e2868
LensFX_DrawFlareQuad, 0x2e2b00 LensFX_ProjectSource, 0x2e2ff8
LensFX_AddSources, 0x2e30d0 LensFX_Render, 0x2e3130
LensFX_UpdateSourceVisibility, 0x2e3338 LensFX_PrepareView, 0x2e3478
LensFX_DrawFlares, 0x3ad230 World_ChunkResolveHook. Already ported: 0x2e27e8,
0x2e2fa8, 0x3aabd8, 0x22a270.

## Generated functions studied (not hand-ported yet)

| Address | Name |
|---|---|
| 0x2f5400 | WorldLightMan_GetInstanceLighting |
| 0x2f5af0 | WorldLightMan_AddPointLights |
| 0x2f5a70 | WorldLightMan_GetOverlapRegion |
| 0x2f5148 | WorldLightMan_LightInstanceCB |
| 0x331450 | OverlapList_GatherType2 |
| 0x339598 | Cell_GatherType2Recursive |
| 0x3aae68 | cPS2ChunkResolve_Resolve |
| 0x3ab498 | cPS2ChunkResolve_Release |
| 0x3aace0 | cPS2ChunkResolve_CalcPercent (light radius from intensity; no direct xrefs) |
| 0x389308 | Irradiance_AddDirectionalScaled |
| 0x1220d8 | Rider_BuildLightSet |
| 0x36c740 | PS2GraphicsMan_ResetBloomParams |
| 0x328c20 | SpatialHash_Insert |

Already hand-ported and read: 0x2f5b68, 0x2f5d30, 0x2f6168 (`worldlightman.rs`;
note its `light::TYPE` comment calls type 2 "directional-like": it is the
point light), 0x38a6a8 / 0x38a530 / 0x38a618 (`particle.rs`), 0x389520,
0x3aac50 / 0x3aaca8 (`worldview.rs`), 0x122088, 0x3ad1a8, 0x2bc830, 0x2f00a0,
0x37e238.

## Open questions

- Meaning of light flag `+0xc` 0x100 and of the constant tail `+0x66..+0x6f`.
- Effects-list kind 5 (23 on Snow Jam).
- Whether the baked terrain vertex colours already include these lamps (compare a Metro City night shot with and without native lights).
