# Phase 3, rendering: native wgpu renderer design

Research pass 2026-10-16. Working material (scratch, not committed):
- `vudis.py`: a VU disassembler that reads the `.vutext` MPG chains in the ELF.
- `<hash>.s`: full listings of all 7 programs.
- `d159/`, `bez/`, `ab67/`, `f55/`: per-program working notes from 4 analysis subagents. They include a `.vucap` parser (`d159/vc.py`) and a VIF walker over the frame-5000 RAM dump (`ab67/vif.py`).

Confidence marks: **V** = read in code, or seen in captured/live data. **I** = inferred.

**Corrections to `structs/render.md` found on the way:**
- Header qw0-3 for types 1/2/3/6 is **not** worldView. It is view(gm+0x57c0) × world, and SetPerspective builds it as a **guard-band projection**. So it is model→guard-clip space.
- Type 1/2 GIF tags use **REGS ST, RGBAQ, XYZF2 with FGE set**, not XYZ2.
- In 9fa47842, MSCAL 0x398 is the normal draw entry. Morph is MSCAL 0x10.
- e72540c9 (type 5, slots 85/86) draws **SnowBox snow sprites**, not trails.
- Slot 75 / entry 0x3bf0 *is* called in races, 73-183 times per capture.
- `FUN_0030c8d8` is `WScriptMan_StaticInit`, not a type-4 user.
- The type-4 mode-0 header really is wrong, and the program needs the mode-1 header (§1.5).

---

## 0. Summary

| Item | Decision / finding |
|---|---|
| Interface level for "one draw" | **DeferMan bucket + VU call** (§2). Each sorted bucket is one state, ≤2 textures, one MatrixRecord, and a packet range. Parsing a packet's VIF codes yields a list of `VuCall { material, entry, header, input qwords }` or a GIF stream (DIRECT). Native code per material consumes exactly what the microprogram reads. |
| Shader model | **Vertex pulling.** The raw input qwords of all VU calls in the frame go into one storage buffer. Each material/entry is one WGSL vertex shader that indexes it by `vertex_index` and runs the microprogram's per-vertex maths. Clipping and procedural generation (Bezier stitching, particles, snow) run in a CPU pass or a compute pass that writes vertices. |
| Composition with the emulated GS (verification stages) | **Surface hand-off at run boundaries** (§4.1). The sort key places all buckets of one material type in a layer and sort group next to each other, so there are only a few native "runs" per frame. Before each run, unswizzle the frame (CT32) and Z (Z24) from GS VRAM into wgpu textures. Draw natively. Swizzle back. Z24 → `Depth32Float` as z/2^24 is exact. |
| End state | The wgpu targets are the master surfaces, at N× resolution. The GS model is kept only as an oracle. Post effects (depth fog, bloom, colour filter, transition capture, flare Z-query) become native passes. They do not reproduce the VRAM aliasing tricks. |
| Biggest risks | GS Z semantics (32-bit Z kicked into Z24 and clamped, §5.2). AFAIL=FB_ONLY alpha test. Destination-alpha blends with 0x80 = 1.0 (needs a float or 2× alpha target). Screen-space-linear colour interpolation. Clip-and-ADC parity. Texture formats pre-swizzled in RAM. The depth-fog Z→alpha copy. |

---

## 1. The 7 microprograms

Common to all VU1 programs:
- **Matrix and projection.** Each program forms `c = M·p` (clip or guard-clip space), `q = 1/c.w` (DIV, often pipelined one vertex ahead), screen `xyz = FTOI4(c.xyz·q·scale + offset)` (12.4 fixed point) and `STQ = (s·q, t·q, q)`.
- **ADC.** The first 2 vertices of every strip have ADC=1, and triangles that fail the guard-band test are suppressed through ADC.
- **No backface culling anywhere (V).**
- **No GS fog except types 1/2** (FGE with a constant F, §5.2).
- **Colour convention.** RGBA ints, 0x80 = 1.0 under TFX MODULATE. Alpha 0x80 = 1.0 in blending.

### 1.1 Overview

| hash | type | slots / users | entries (MSCAL byte addr) | input (TOPS, BASE/OFFSET) | computes | GIF output |
|---|---|---|---|---|---|---|
| f4babac5 | 1 | Bezier terrain patches, uniform LOD (`PS2BezierMan_EmitPatchType1`) | tables 0x504840/8a0/900 (n=4/6/8), idx 2+flag: 0x6b8/0x700/0x748 (flag 0), 0x790/0x7d8/0x820 (flag 1); projector 0x8d8/0x910; matrix pass 0x948/0x980; relight 0x868/0x8a0 (no caller found) | 0x250 / 0xd8. CPU-tessellated n×n grid: V3 positions @0, V2 UV @0x40, V2 UV2 @0x80 | strips from grid rows, constant colour, guard clip + clipper, 2nd ctx2 pass (UV2 / projected / normal-matrix UV) | strip/fan ctx1 and ctx2, PRIM 0x5c/0x5d (IIP TME FGE?), REGS ST,RGBAQ,XYZF2 |
| 9790369f | 2 | Bezier patches with mixed edge LODs (`EmitPatchType2`, n=8 + crack stitching) | table 0x504960: 0x9a0 (flag 0), 0xa38 (flag 1), 0xc48 projector, 0xcf0 matrix, 0xb00/0xb68 relight | as type 1 + V4-32 16 coefficients @0xc0 + V4-8 edge LODs @0xd0; header 42 qw @0x200 (basis/derivative rows) | type 1 + border points re-evaluated on the VU (power-basis bicubic) + "zipper" strips | as type 1 |
| d159716b | 3 | slot 71 tri strips, BeginTriStrip, 89 shadow volume, 90 wake, 95/96 models (SkyBox, ModObj, MeshAnim, cPS2GridMesh), 75 line shape | 0x12b8 (0x12b0), 0x3a08 (0x3a00), 0x2270/0x22c8/0x2320/0x2358 (meshes), 0x2400 (PRIM-only kick), 0x3bf0 | 0x26 / 0x1b6, output at XTOP+0xdb | float strips; meshes (int16 positions, 4.12 UV, colour **or** normal → SH lighting), texgen, 2-pass ctx1/ctx2; guard cull + near clipper | input tag (strip 0x5c/0x4c ctx1, or 0x25c ctx2) + clipped **fans**, REGS ST,RGBAQ,XYZ2 |
| 9fa47842 | 6/7 | slot 99 rider (lit), 100 shadow silhouette (via 101), 124 lighting packet | 0x00/0x398 lit draw (bone per vertex), 0x50 lit (one bone per batch), 0x10 morph, 0x20 shadow, 0x30 env re-kick, 0x40 kick ctx2 tag, 0x80 bone setup, 0x70 empty (parity), 0x1150 lighting ×255 | 0xc0 / 0x169 | rigid 1-bone skinning (blending done on the EE), SH lighting ×255, env-map ctx2 pass, morph deltas | strip ctx1 0x5c + re-kick ctx2; shadow: XYZ only |
| f55ee634 | 4 | slots 82/83/84 emitters, cBoardSparkFX, Bezier patch glitter | 0x0, 0x510, 0xa00, 0xf10, 0x1460 | 7 / 0x9a, out 0x13b/0x29a | procedural particles (LFSR RNG, drag curve, trails, colour/size over life) | SPRITE 0x56 (TME ABE), REGS ST,RGBAQ,XYZ2,ST,XYZ2 |
| e72540c9 | 5 | slots 85/86 SnowBox | 0x0 (CLIP cull), 0x408 (depth fade) | 3 / 0xb, out 0x19/0x209 | random snow in a wrapped unit box | SPRITE 0x56, 5 qw per sprite |
| 27649fa5 | VU0 | EE micro-subroutine (VCALLMS): mesh bounding-box test (entry 0x0570) and physics | — | — | CLIPs 8 corners; needs clip latency 4 (race-geometry.md) | none. **Not a renderer concern**: stays translated or becomes a Rust function |

### 1.2 Type 3: d159716b, the general program (V unless marked)

**VU memory layout**

| Address | Contents |
|---|---|
| qw0-3 | M (model→guard clip). Meshes: qw6-9 = M × model, computed by 0x2470 |
| qw4 / qw5 | guardScale (gw/2, −gh/2, −8388467.5, 0) / guardOffset (cx+2048, cy+2048, 8388467.5, 0) |
| Mesh-only | qw10-13 texgen matrix ("matrix 2"); qw15-24 SH lighting coefficients ×128 (qw15.w = 128); qw25-27 lighting basis |
| GIF tags | strip ctx2 @0x1f (0x25c), fan ctx2 @0x20, fan ctx1 @0x3c8 / 0x3e4 (alternating); all REGS 0x412 |
| Clipper buffers | 0x392 / 0x3ad (9 vertices × 3 qw) |

**Inputs and computation per entry**

| Entry | Input at TOPS | Computation |
|---|---|---|
| 0x12b8 (slot 71) | qw0 GIF tag (copied out); qw1.x = 3n; per vertex {S,T,1,0} f32, {R,G,B,A} i32, {X,Y,Z,1} f32; ≤ 0x48 vertices | `c = M·p`; STQ = (S·q, T·q, q) |
| 0x3a08 (wake, shadow strips) | qw0 tag, qw1 {3m, m, Δα, α0}; edge A then edge B, m × 3 qw each | Rebuilds the ribbon A0 B0 A1 B1… in place. `alpha_k = a · clamp(α0 + kΔα, 0, 1)`. Patches the count, then jumps to 0x12b8 |
| 0x2270 family (meshes) | Header: model matrix +0..3, {0,0,0,flags} +4, optional matrix 2 +5..8, optional lighting +9..11/+15..24. Mesh at +0x1b: tag (NLOOP = n), {nLenQw, 0, n}, strip lengths, n × int16-derived xyz (ITOF15) with w=1, n × UV 4.12, n × RGBA i32 **or** normal (ITOF15) | Flags: bit0 lit; bits1-4 pass-1 texgen (2 UV matrix, 4 planar xy, 8 normal→UV, 0x10 sphere map); 0x20 clip needed; bits 6-9 the same texgen for pass 2. ITOP==0 → pass 1 (ctx1), ITOP!=0 → pass 2 (rewrites only ST, ctx2 tag, re-kick) |
| 0x3bf0 (line shape) | A+D RGBAQ, line-strip tag, cubic coefficients C3..C0, n | `P(t) = Σ C t^k`, t = i/(n−1); XYZF2 F=0xff; ADC=1 if an endpoint is outside the guard band |

**Shared rules**

- **Lighting (bit 0):** `d = B·n` (basis qw25-27). `col = K0 + Σ Ki·di² + Kxy·dx·dy + Kzx·dz·dx + Kyz·dy·dz + Σ K'i·di`, clamped to 0..255. Alpha = 128. This is a 2nd-order SH irradiance. The point-light code at 0x0008-0x08a0 is dead.
- **Texgen:**
  - UV matrix: `uv' = A·uv + b`.
  - Planar: the same with position xy as input.
  - Normal env: `uv = Mx·n + c`; live values are 0.5 scale + 0.5 offset.
  - Sphere map (0x3400): `R = I − 2(n·I)n`, `uv = 0.5·R.xy / |R + (0,0,1)| + 0.5`.
- **XYZ:** `FTOI4(c·q·gS + gO)`, Z included. So **Z = 16 × 8388467.5 × (1 − z/w)** goes into a 32-bit XYZ2 word (§5.2).
- **Cull and clip** (strip paths, and meshes with flag 0x20):
  - CLIP against |w| per vertex; FCAND over the last 3 vertices.
  - A failing triangle gets ADC on its 3rd vertex.
  - It is **dropped** if any vertex has z > |w| (far) or all 3 are outside one plane.
  - Otherwise it is Sutherland-Hodgman clipped in the order z−, x−, x+, y+, y−. ST is interpolated raw (Q forced to 1), RGBA as float.
  - The result is kicked **immediately** as a separate **ctx1 fan** (even if the strip is ctx2; pass 2 patches the fan to ctx2). Fans therefore reach the GS **before** their strip.
  - Unclipped mesh path: no culling (the EE guarantees the box is inside the guard band).
- **WGSL:** one shader with the texgen/lighting branches selected by flags (uniform per draw). Strips become an index buffer with primitive restart at the strip starts. Clipping: let the GPU clip against the real near plane. Emulate the "drop if z > |w|" far rule and the guard-band reject with clip distances, or on the CPU for exact parity (Stage 2A). Pass 2 is a second draw of the same vertices with the ST from the pass-2 texgen and ctx2 texture/state.

### 1.3 Types 1/2: Bezier terrain (f4babac5, 9790369f)

| Aspect | Detail |
|---|---|
| Patch data (V) | Bicubic in **power basis**: 16 vec4 at patch+0x40, `P(u,v) = Σ C[4k+l]·u^(3−l)·v^(3−k)`. Corner UVs at +0x20/28/30/38 (bilinear, accumulated incrementally in float). UV2 = an axis-aligned rect at +0x10/+0x18 (lightmap/detail atlas, I). Textures +0x1a0 (ctx1), +0x1a2 (ctx2). Flags +0xc: 0x800000 → glitter FX. |
| LOD (V) | `GameModule_DrawBezierPatch` 0x22c410 picks a per-edge LOD of 4/6/8 from camera distance (table 0x4a2a10). All edges equal → type 1 with n×n grid. Otherwise → type 2 (n = 8, borders stitched). |
| Tessellation | Type 1: **CPU** (`BezierPatchCache_Tess*`, cached), uploaded as positions/UV/UV2 grids. Type 2: interior from the CPU 8×8 grid. Border lines with L<8 are re-evaluated on the VU from the coefficients (basis rows at VU 0x200) and stitched with "zipper" strips (inner 6 points alternating with outer L points; the longer list goes first; leftovers appended). |
| Header | qw0-5 M/guard; qw6 colour {128}×4; qw7-10 tags strip/fan ctx1/ctx2; qw11-16 per-n strip/NLOOP tables. Persistent state across MSCALs: [39] {n, tbl, n²}, [510] XTOP, tag slots 96/289/452/480. |
| Vertex | `c = M·P`; STQ = (s·q, t·q, q); RGBA = qw6 (constant); XYZF = FTOI4(c·q·gS + gO); **F = const from gS.w + gO.w** (I: live value unknown); ADC on the first 2 vertices of each strip; same guard cull and clipper as type 3 (fans). |
| 2nd pass, flag 0 | Re-kick the same output with ST = UV2·q, ctx2 tags. **Clipped fans are not redrawn**, so ctx2 is missing on clipped triangles. |
| 2nd pass, flag 1 | Full re-transform with clipper, UV2, ctx2. |
| Matrix pass | UV = (rotM·N).xy, an animated shine (I). |
| Projector pass | UV = (lightM·P).xy, affine (shadow/light projector), one per light bit. |
| Relight (unused) | 3 point/spot lights; computes RGBA. |
| WGSL | Precomputed index buffers per n, and per (Lx,Ly,Lz,Lw) combination for type 2 (at most 3^4 = 81). Vertex buffer = the CPU grid (already in RAM). Type-2 border points: evaluate the power-basis cubic in the vertex shader from the 16 coefficients (storage) or on the CPU. Pass 2 = a second draw with a UV2/projector/matrix UV source and ctx2 state. |

### 1.4 Types 6/7: rider (9fa47842)

**Memory (V)**

| Address | Contents |
|---|---|
| qw0-3 / 4 / 5 | M (guard clip), scale, offset |
| qw7-16 | 10 SH lighting coefficients (slot 124; MSCAL 0x1150 multiplies them by 255 in place) |
| 0x11+4i | bones M·B (MSCAL 0x80 builds them from the raw matrices; n ≤ 25 at 0x3ff) |
| 0x75+3i | raw rotations |
| 0x392 | ctx2 strip tag |
| 0x393-395 | E = bias{0.5, −0.5} × M |
| 0x396+3i | E·R per bone |

**Inputs and computation**

- **Mesh batch input (TOPS):** tag (NLOOP total, strip ctx1 0x302e4000, REGS 0x412); {nSub, boneAddr, nrmAddr, total}; sub-strip counts; 3 qw per vertex (STCYCL 3,1):
  - V4-16 {u, v (4.12), boneAddr, nrmAddr}
  - V3-16 normal (1.15)
  - V3-32 position, with **w = 1.0 from STROW/STMASK**
  - Output at TOPS+0xb4 (≤ ~58 vertices).
- **Skinning:** one matrix per vertex (`c = Bone·(p,1)`). Blended palettes are made on the EE (`PS2GraphicsMan_*BlendedMatrix*`, slot 109). Morph (0x10): `pos.xyz += itof15(d)·weight·12800`, written into the output area before the draw.
- **Lighting:** `nw = R·n` (not renormalised). `col = L0 + L1x² + L2y² + L3z² + L4xy + L5xz + L6yz + L7x + L8y + L9z` with L = coefficient·255, clamped to 0..255. Alpha comes from the coefficients' .w (an env/Fresnel strength term, I). The EE folds directional + converted point/spot + ambient into these coefficients (`Irradiance_*` 0x389260..).
- **Cull:** CLIP against w=1 after the divide (guard band); ADC if any of the last 3 vertices is out. **No near clipper.**
- **Env pass (0x30):** `st = ((E·R)[bone]·n).xy + 0.5`, times the vertex's Q. RGBA and XYZ are reused. Re-kicked with ctx2. The packet sequence `MSCAL 0x50, 0x30, 0x30` depends on double-buffer parity.
- **Shadow (type 7, 0x20):**
  - Header from slot 101: an orthographic light matrix with scale (64,64) and offset (2048,2048), so Z = 0.
  - 1 qw per vertex {xyz, boneAddr}; output XYZ only; flat colour 0x80262626 from a DIRECT RGBAQ.
  - This is a **render-to-texture of the silhouette into a 128×128 area**, projected onto the ground later (`BoaderShadow_*`, I strong).
- **WGSL:** a storage buffer of bone matrices per draw (M·B) plus the SH coefficients as a uniform. The vertex shader reads the vertex qwords directly (vertex pulling also handles the absolute boneAddr). The env pass is a second draw. The shadow becomes an offscreen 128×128 target registered as the shadow texture id.

### 1.5 Type 4: particles (f55ee634) and type 5: snow (e72540c9)

| | Type 4 | Type 5 |
|---|---|---|
| Header | **Needs the mode-1 header** (worldClip, screenScale/Offset z/16, {viewProj[0].x, viewProj[1].y}). The mode-0 header (sent when the material type changes) makes the first type-4 bucket after a switch wrong and nearly invisible: a **game bug**. Decide whether to keep parity (§5) | screenScale, screenOffset (z/16), proj xy |
| Generation | Every particle is regenerated each frame from a 21-qw descriptor (D0-D20) and LFSR seeds (`r = ((r<<1) ^ b4 ^ b22) & 0x7fffff`, value in [1,2)). Life, velocity, spawn, colour (D15 + D16 r + D17 r + D18·age), size over life, drag curve `pos = spawn + G t + (G−V)(−0.73 tc + 0.113 tc²)` with tc = min(t, 2.7), trails of m samples with alpha fade. 0x510 / 0xa00 / 0xf10 interpolate ring-buffer spawn points (≤ 64 / 43 per batch; c+1 off-by-one). 0x1460 scatters glitter on a Bezier patch through an order-dependent float hash | p = LFSR^(k+2)(seed) per axis + box offset, single-step wrap into a unit box, `c = M·(p,1)`; 0x0 CLIP-cull, 0x408 alpha fade by depth |
| Output | Axis-aligned SPRITE, corners `c ± h` with `h = min(|size·P·S/w|, 64 or 128)` per axis; ST corners D19/D20 (order differs per entry); Z shared; **centre-only CLIP cull** | same sprite format; 99 sprites per kick |
| Native | **CPU or compute simulation**, bit-exact LFSR, then instanced quads. The GPU only expands the two-corner sprites. Respect the RGB clamps per entry: 0x0 alpha unclamped/wrapping, 0xa00/0xf10 clamp to [0,128] | CPU or compute; the LFSR must match or the flakes move |

---

## 2. The draw-record level

### 2.1 What exists (V, render.md + this pass)

**Per-draw objects**

| Object | Fields that matter |
|---|---|
| DrawRecord (0x80, max 2600) | RenderState (0x14), packet start/end in VifBuff, matrix index (short), continuation flag |
| RenderState w0 | CLAMP1/2 (bits 2-5), **material type** (bits 6-9), ALPHA FIX 1/2 (12-27) |
| RenderState w1 | sort mode (0-1), blend preset 1/2 (2-11), AREF (12-19), ATST (20-21), ZMSK (22), ZTST (23-24) |
| RenderState w2 | scissor index (0-4), layer (5-9), explicit sort value (10-28) |
| RenderState w3 / +0x10 | w3 = material parameter (EE address of the type-6 lighting packet or type-7 shadow header); +0x10 = tex[2] (−1 = none) |
| MatrixRecord (0x180) | world, view, worldView, guardScale/Offset, viewProj, worldClip, screenScale/Offset. Deduplicated on qw 14-23 |
| Bucket (0x20) | state, textures, matrix index, first packet start (CALL target), last end, chain (NEXT tags between consecutive packets) |

**Sort key (`DeferMan_ComputeSortKey` 0x364240, V).** Buckets are radix-sorted ascending on `~key`, where

`key = (31−layer)<<26 | sortval<<16 | matRank<<13 | texKey<<3`

- So the order is: layer ascending, then sortval descending (mode 0 = 0x3ff, mode 1 = 0x3fe, mode 2 = explicit), then matRank descending, then texture descending.
- matRank: type 5 → 6, 2 → 5, 1 → 4, 3 → 3, 6/7 → 2, 4/8 → 1, 0 → 0.
- texKey = tex0 | (tex1 & 0xf) << 6.

**Chain emission (`PS2GraphicsMan_RenderFrame` 0x363490, V).** Per bucket:
1. `EmitStateChange`: material setup (mode 0/1/2), then SCISSOR / ZBUF / TEST / CLAMP / ALPHA diffs as A+D.
2. `TexMan_BindTextures`: VRAM allocation + GIF upload + TEX0/MIPTBP.
3. `CALL` the packet, with a RET written at its end.

The layer thresholds in the table at 0x4a4058 split the sorted bucket list:
- after layer 5 → per-view `RenderDepthFog`;
- after layer 8 → END of phase 1; the phase-2 chain then starts with `RenderBloom` and `RenderColorFilter` per view;
- then layers ≥ 9 (HUD/UI, I).

`LensFX_UpdateCallback` runs in the kick thread **between phase 1 and phase 2**, so it sees Z after the 3D scene and before bloom reuses the Z memory.

**Blend presets (jump table 0x491fb0, V).** Here A, B, C, D are the GS ALPHA fields; the result is `(A−B)·C + D`, with C = 0x80 meaning 1.0.

| # | ALPHA | Formula | # | ALPHA | Formula |
|---|---|---|---|---|---|
| 0 | 0xaa | (ABE off) | 9 | 0x42 | Cd − Cs·As |
| 1 | 0x2a | Cs | 10, 21 | 0x49 | Cd·As + Cd |
| 2 | 0x68, FIX 0x80 | Cs + Cd | 11 | 0x64, FIX | lerp(Cd, Cs, FIX) |
| 3 | 0x6a | Cd (colour write off) | 12 | 0x68, FIX | Cs·FIX + Cd |
| 4 | 0x89 | Cd·As | 13, 14 | 0x88 | Cs·As |
| 5 | 0x44 | (Cs−Cd)·As + Cd | 16 | 0x98 | Cs·**Ad** |
| 6 | 0x46 | Cd·(1−As) | 17 | 0x58 | Cs·**Ad** + Cd |
| 7 | 0x48 | Cs·As + Cd | 15, 18-20 | — | no value written (unused) |
| 8 | 0x81 | (Cd−Cs)·As | | | |

**TEST (`GsPkt_AddTest`, V).** ATE is always 1:
- ATST: 2 → LESS, 3 → GREATER, otherwise ALWAYS;
- **AFAIL = FB_ONLY**: a failing pixel still writes colour but not Z;
- ZTE = 1; ZTST: 0 → GREATER, 1 → GEQUAL, otherwise ALWAYS. Larger Z is nearer.

### 2.2 Proposed typed interface

```rust
// One frame as handed to the renderer (built in an override of PS2GraphicsMan_RenderFrame,
// after BuildBuckets/SortBuckets; the generated code keeps building packets).
pub struct Frame {
    pub views: Vec<View>,                 // gm+0x5930.., numViews (split screen)
    pub matrices: Vec<MatrixRecord>,      // DeferMan +0x69cd0
    pub scissors: Vec<[u16; 4]>,          // DeferMan +0x6aed4
    pub passes: Vec<Pass>,                // in emission order
    pub display: DisplayState,            // brightness/blit, fb size, screenOffset, flip
}
pub enum Pass {
    Draws(Vec<Draw>),                      // a contiguous range of sorted buckets
    DepthFog { view: u8, p: DepthFogParams },        // gm+0x6c64[view]
    PhaseBoundary,                         // run the phase-2 callback here (LensFX queries)
    Bloom { view: u8, p: BloomParams },              // gm+0x6cd4[view] + globals 0x4a43c8..
    ColorFilter { view: u8, p: ColorFilterParams },  // gm+0x6ca4[view]
}
pub struct Draw {                          // one bucket (or one record of a chained bucket)
    pub state: RenderState,                // decoded: blend[2], fix[2], atst/aref, ztst, zmask,
                                           // clamp[2], scissor, layer, sort, material
    pub tex: [Option<TexId>; 2],           // ctx1 / ctx2
    pub matrix: u16,                       // index into Frame::matrices
    pub material_param: Option<GuestPtr>,  // type 6 lighting packet / type 7 shadow header
    pub payload: Vec<Payload>,             // decoded from the packet range(s)
}
pub enum Payload {
    Vu(VuCall),                            // material type != 0
    Gif(GifStream),                        // DIRECT / PATH2 (type 0 slots 72-81, 89 passes,
                                           // 53/61 uploads/copies, colour/state A+D)
    TexUpload { tex: TexId },              // slot 53
    ScreenCopy { dst: TexId },             // slot 61 (transition capture)
}
pub struct VuCall {
    pub material: Material,                // Bezier1, Bezier2, General, Particle, Snow, Skinned, Shadow
    pub entry: u16,                        // MSCAL address
    pub tops: u16, pub itop: u16,          // double-buffer parity matters (types 3 and 6)
    pub input: Range<u32>,                 // qwords in the frame's VU-input arena (VIF-unpacked)
    pub header_extra: Range<u32>,          // per-call uploads at fixed addresses (bones@0x11,
                                           // count@0x3ff, model header, lighting@7)
}
```

**How Payload is obtained: a VIF interpreter at the EE level, not the hardware VIF.**
- It walks CNT/REF/CALL/RET across the packet range and applies STCYCL/STMASK/STROW/UNPACK into a *virtual* VU memory image.
- At each MSCAL it snapshots the input buffer (TOPS..) and the dirtied fixed areas into the arena.
- This is the existing `vif/mod.rs` unpack logic run on the EE side. It is exactly what the microprogram sees.
- **Verification:** run the translated program on that snapshot and compare its XGKICK output with the emulated path (`vu_diff`-style). Then compare the native per-material implementation with the same output.

### 2.3 Textures (TexMan)

| Item | Detail (V unless marked) |
|---|---|
| TexInfo (0x58) | Fields: name, `format` (2 = 16-bit, 8 = 24/32, 9 = 8-bit pal stored as **T8H in the display buffer's alpha byte** (temp heap), 10 = 8-bit pal, 0xb/0xc = 4-bit pal), pixels ptr, clut ptr (0x400 or 0x40 B), mip count, TBP/CBP (−1 = not resident), TEX0, TEX1, MIPTBP1/2 |
| Pre-swizzled formats | 10 and 0xc are **pre-swizzled in RAM** (`TexMan_SwizzleT8/T4`) and uploaded as PSMCT32 at half or quarter width (`TexMan_EmitImageUpload`), then read as T8/T4. A host texture needs the GS swizzle model (`gs/swizzle.rs`) to un-swizzle once per texture |
| CLUTs | CSM1 layout: index bits 3 and 4 swapped (seen in `BuildDepthFogClut`). Alpha 0x80 = 1.0 (values up to 0xff = 2.0). TCC/TFX come from TEX0 / `TexMan_SetTexFunction` |
| Mips | Up to 4 levels (MIPTBP from TEX0 dims). Mip blocks are at least 256 B. TEX1 (LCM/MXL/K/filters) from `TexMan_SetTex1Filter` |
| Residency | `TexMan_BindTextures` allocates VRAM per bucket on demand (VramHeap, LRU by draw index), emits a GIF upload into GifBuff and TEX0 into the VIF chain |
| Mutation | `TexMan_UpdateTextureData` / `LockTexture` / slot 53 change pixel data in place. A native cache must hash or version the data. Tip: generation counter bumped by overriding those three; fall back to hashing pixels + CLUT for safety in stage 1 |

**Native texture flow:**
1. `TexCache: HashMap<TexId, (generation, wgpu::Texture)>`.
2. Decode (pixels, CLUT, format, mips) into Rgba8 (or R8Uint + palette texture, if palette animation is common: sky palettes, `race-textures-sky-palettes.png`).
3. Keep the TEX1 filter/LOD and the CLAMP mode from RenderState as the sampler.
4. Alpha: store `min(a·2, 255)` in Rgba8, or use Rgba16Float with a/128.
5. Render-target textures (slot 61 screen capture, rider shadow, bloom levels) are registered under their TexId as GPU-only entries.
6. Bypass `TexMan_BindTextures` uploads once no draw is emulated (stage 5). Until then they still go to GS VRAM.

---

## 3. 2D/UI path and post effects

### 3.1 2D

| Slot | Path | Native |
|---|---|---|
| 71 / BeginTriStrip | VU type 3 entry 0x12b8, float input already in "world" space with M = gm matrices (HUD uses ortho via slot 26) | Same shader as type 3 (pass-through transform); per-batch ≤ 0x48 vertices, split with an overlap of 2 |
| 72 indexed tris, 73 line strip (EE near/far clip), 78 sprite3D, 79 rotated sprites, 80/81 glyphs (Font_DrawText) | **DIRECT/PATH2.** The EE already projected the vertices into GS window coordinates (12.4, window offset XYOFFSET = (2048 − w/2, 2048 − h/2)·16). REGLIST/PACKED GIF | **A small Rust GIF interpreter** (PRIM, RGBAQ, ST/UV, XYZ2/XYZF2, A+D for TEX0/ALPHA/TEST/SCISSOR) produces triangles/sprites in window coordinates. Draw with an ortho shader. Scaled resolution: multiply by the scale factor around the window origin. |
| 54 clear, 53 upload, 61 transition capture | DIRECT | Clear → `LoadOp::Clear` or a quad. 53 → texture-cache update. 61 → `copy_texture_to_texture` from the display/draw target into the TexId texture |
| SetDisplayRegs blit | Per flip, copies the draw buffer into the display buffer with sprite strips × brightness (gm+0x59e8/0x59ec/0x59fc); PMODE/DISPLAY | The final present pass: brightness multiply, 4:3 or widescreen letterbox |

Fonts are textured sprites from font sheets (normal TexMan textures). No special handling.

### 3.2 Post effects (decompiled this pass, V)

**Depth fog** (after layer 5), per view: `BuildDepthFogClut` + `ZCopy` + `Blend`.
- **CLUT.** 256 entries, with index = screen Z >> 8 clamped to 0..255.
  - Near index = projected fogStart, far index = fogEnd.
  - Mode 0 linear, 1 exp, 2 exp2, using polynomial exp approximations.
  - alpha = (1 − fog)·128; entries ≥ near index = 0x80. RGB = fog colour.
- **ZCopy.** Draws the Z buffer (as **PSMZ16** texture) into the draw buffer as **CT16 with FBMSK 0x3fff**, in 16-px sprite columns, so Z bits land in the frame's alpha byte.
- **Blend.** Samples the frame as **PSMT8H** (alpha byte as palette index) with the fog CLUT, DECAL, ALPHA 1 = lerp(Cs, Cd, As) (alpha 0x80 → frame unchanged), Z test GREATER with ZMSK, 32-px columns.
- **Native.** One fullscreen pass that reads the depth texture: `idx = f(Zgs)` → CLUT lookup (1D texture, 256) → blend. The exact Z → index bit mapping of the CT16/Z16 aliasing is **I**; derive it by running the GS model on a synthetic Z ramp (§5).

**Bloom** (start of phase 2), params at gm+0x6cd4 + globals 0x4a43c8..0x4a43f8. Scratch buffers live **in the Z buffer's VRAM** (zbp·32 + 0x400).
1. `EmitBlurDownsample`: 1–4 bilinear taps with additive weight (FIX 0x80, ±offset), into a ½ buffer, FBMSK alpha.
2. `EmitBlendedFill`: bright pass, `(Cd − threshold)·FIX`, clamped.
3. Three levels of downsample (halving each time) with `EmitBloomComposite` per non-zero level weight: `Cd·FIX + tex·weight`, bilinear upsample, onto the frame.

Native: a standard mip-chain bloom with the same weights and threshold. Not bit-exact, compared with tolerance.

**Colour filter.**
1. Sprites sample the frame itself (CT32, MODULATE, RGBAQ = scale·127.5), drawn in place in 32-px columns: `frame = frame·scale` (0..2×).
2. If an offset is non-zero: an additive untextured pass, ALPHA 0x48 = Cs·As + Cd.

Native: one fullscreen pass, `clamp(frame·scale + offset)`.

**Fog particles** (`cPS2FogParticleMan_DrawParticles`): ordinary draws (type 0/3, check at implementation time).

**Lens flare occlusion** (`LensFX_v04` 0x2ec478, called from the kick thread between the phases).
- Input: query {x, y, w, h (light rect), rx, ry, rw, rh (readback rect), …, depth (+9)}.
- GS local→host copy of the Z24 rect into LensFX+0x6510, then on the CPU: `visible = count(z ≤ depth)/total`, or `max(0, (visible − half)/half)` if the rect is partly off screen. Output to `*out`.
- **Native:** override `LensFX_v04`. Record the queries; after the phase-1 draws, run one **compute pass** over the depth texture that counts `Zgs ≤ depth` per query (respecting the screen bounds). Then either:
  - (a) map the result synchronously (a few µs of GPU work, but it stalls the CPU until phase 1 is rasterized), or
  - (b) use it one frame late, as GS latent readback did. Captures showed no image difference with (b), but synchronous is the documented default since the flare fix.
- At N× resolution, scale the rect and count at the scaled resolution. The ratio is unchanged.
- This keeps the final roadmap task (flare occlusion) fixable natively.

---

## 4. Staged plan

### 4.1 Composition mechanism (stages 1-4)

- **Master surfaces stay in GS VRAM** while any draw in the frame is emulated.
- The **NativeRun executor** runs in the GS thread (MTGS) at the point in the command stream where the run's buckets would have been CALLed. The native override of `RenderFrame` inserts a marker command `Cmd::Native(run_id)` into the chain or stream instead of the CALLs.
- **Steps per run:**
  1. Flush the GS batch.
  2. Compute pass: unswizzle the frame (CT32 at FBP) → `Rgba8Unorm` and Z24 → `Depth32Float`. A fullscreen fragment pass writes `frag_depth = z/16777216`, exact for 24-bit integers.
  3. Render pass(es) with the native pipelines: same scissor, same `CompareFunction::Greater/GreaterEqual`, same blend.
  4. Compute pass: swizzle colour and depth back. Write only pixels covered by the run (a coverage mask, or write all: unchanged pixels round-trip exactly).
  5. Mark the pages `gpu_dirty`.
- **Cost:** about 4 fullscreen passes per run, ~1 ns/pixel → ~1 ms per run at 512×448.
- **Number of runs:** the sort key puts each material type's buckets contiguous within (layer, sortval), so a native material yields ≤ (#layers × #sort groups) runs. Mode-2 sorted layers (particles, layer 7) interleave by depth; in those, either make the whole layer native or accept more runs.
- **Same-frame ordering is exact by construction**: the native run executes at the same position in the stream. Fans before strips (§1.2) must be preserved *within* a run.
- **Comparison:** both renders use the same 1× grid, so `gs_replay --compare`-style metrics apply (differ / >8 / >32 / mean). The expected residual is rasterization-rule and f32 differences. Gate on the existing thresholds: >32 differences ≤ 0.01%, plus a visual diff directory.
- **Rasterization parity knobs:**
  - Positions are snapped to 1/16 px from the FTOI4 values, with the same 0.5-px guard offset (2047.5).
  - Colour uses `@interpolate(linear)` (GS Gouraud is screen-space linear); STQ is perspective-correct (ST/Q).
  - Sprites map to 2 triangles with the GS sprite texel rule (start at +0.5 with FST, I).
  - The top-left fill rule matches the GS.

### 4.2 Stages

| Stage | Native | Removes | Verify | Perf (I) | Main risks |
|---|---|---|---|---|---|
| **0. Typed frame** | Override `RenderFrame` after `SortBuckets`: build `Frame` (§2.2) incl. the EE-side VIF interpretation into `VuCall`s. Still emit the identical chain | nothing | (a) re-serialise → **byte-identical chain** vs the game's; (b) translated VU on the `VuCall` snapshots → identical XGKICK bytes vs the emulated capture | ~0.3 ms extra EE | VIF parity: STCYCL/STMASK/STROW, MSCAL double-buffer parity (types 3 pass-2 ITOP, 6 MSCAL 0x70/0x30) |
| **1. 2D DIRECT + slot 71** | DIRECT GIF slots 72-81 and type-3 entry 0x12b8 (HUD, menus, fonts, fullscreen quads) via the composition runs. Layers ≥ 9 are mostly one run at the end of phase 2 | GIF/GS for UI | Menus and HUD frames: image compare ≤ thresholds. Menus are 100% covered, so this also tests texture-cache decoding (T4/T8 swizzle, CLUT CSM1, T8H, alpha 2×) | small; the cache matters more | Texture formats; sprite texel rules; alpha-test FB_ONLY; destination alpha (shadow volume slot 89 stays emulated) |
| **2A. Type 3 meshes on the CPU reference** | Rust implementation of d159716b entries producing the *same GS primitives* (not yet wgpu). Push into the GS like XGKICK | VU1 translation for type 3 | **Bit-exact** GIF vs the translated program (vu_diff) | big VU win (type 3 = course/models) | Clipper parity (pipelined Q, \|w\| CLIP), integer colour denormals |
| **2B. Type 3 on wgpu** | Vertex-pulling WGSL (texgen, SH light) in composition runs; clip on GPU + far/guard cull | GS raster for type 3 | image compare race frames (course/sky/models) | GS CPU side shrinks (fewer prims) | Z semantics (§5.2); ADC/fan order differences at the near plane; guard-band vs viewport |
| **3. Types 1/2 terrain** | Index buffers per n / LOD combination; CPU grids are uploaded as-is; border evaluation in WGSL | VU+GS for terrain (largest prim count) | race frames; check the type-2 seams (no cracks) | largest single GS reduction | F/FGE meaning; ctx2 missing on clipped triangles (parity vs "correct"); UV2 lightmap blend state |
| **4. Types 6/7 rider, 4/5 particles, snow** | Bones storage + SH; env pass; shadow RTT (128×128) as GPU texture; CPU/compute particle sim with bit-exact LFSR | the hottest VU program (9fa47842, 31 pairs/vertex) | rider close-ups, charsel; particle frames vs emulation (LFSR exactness ⇒ near-exact) | removes most remaining VU time | MSCAL parity (env 0x30×2); morph-before-draw; particle mode-0 header bug |
| **5. Flip the master** | Post effects native (§3.2), flare query compute, display/brightness present; GS VRAM no longer master. Remaining emulated payloads (slot 89 shadow volume, rare paths) rendered by an internal **GIF→wgpu** path (the same mini-GIF interpreter as stage 1, with destination alpha) | VIF1/VU1/GS per frame; GifBuff uploads (`TexMan_BindTextures` uploads skipped) | whole-frame compare vs the emulated build (both still selectable, e.g. `SSX3_RENDER=native|emu`), plus PCSX2 screenshots for post effects | GPU-bound; EE work only. 60 fps headroom ≫ | Post-effect fidelity (bloom/fog not bit-exact; tolerance compare); depth-fog index mapping |
| **6. Resolution / widescreen** | Scale factor N for targets; widescreen projection | — | visual only; 1× must stay identical to stage 5 | GPU cost ∝ N² | see §4.3 |

Each stage is behind a switch, like `SSX3_NATIVE_FILES` / `SSX3_SND=native`: `SSX3_RENDER_NATIVE=2d,type3,...`. The scripted race replay stays deterministic because rendering does not feed back into game state. The exceptions are the **flare query**, which affects only visuals, and **frame pacing**, which `AcquireFrameBuffer` already overrides.

### 4.3 Resolution scaling and widescreen

**Scaling (from stage 5; usable earlier inside runs only if the master is native).**
- Targets become `512N × 448N`.
- 3D: change only the final viewport mapping (guardScale/Offset × N in shader uniforms). Clip-space maths is unchanged.
- 2D DIRECT: scale window coordinates around XYOFFSET.
- Post passes run in target space; bloom levels scale.
- The flare query rect scales.
- Half-pixel: the GS offset 2047.5 means a −0.5 px bias at 1×. At N×, use −0.5/N px, or re-derive it from the texel-centre rules.

**Widescreen.** The game culls with a 4:3 frustum in several places: `Nash_GatherVisible`, `OcclusionMan`, the VU0 box test using worldClip, the EE guard tests for slot 95 flag 0x20, and the Bezier LOD/clip flag.
- Widen the horizontal FOV at `SetPerspective` (gm+0x5908 aspect) *before* the game derives its cull matrices. Then the game culls and renders consistently at 16:9.
- Render to a 16:9 target (`(512·4/3·16/9)` wide internal).
- HUD/2D: keep it in a centred 4:3 box (pillarbox the 2D layer). Optionally anchor known HUD elements to the edges later.
- Menus/FMV: 4:3 pillarbox.
- Guard band: the type 3/1 clipper uses ±w at the guard band (4× half-width). With a wider viewport the native GPU clipper replaces it anyway.

---

## 5. Open questions and risks

1. **Z depth scale mismatch (important).**
   - Types 3 and 6 FTOI4 their Z (×16) into **XYZ2**, a 32-bit word: Z = 16·8388467.5·(1 − z/w) with no 1/16 pre-scale (V for type 3).
   - Types 1/2 use **XYZF2**, whose packed Z is bits 4..27, so the ×16 cancels. Types 4/5 pre-scale by 1/16.
   - The Z buffer is **Z24** and our GS model clamps Z to 0xFFFFFF (`raster.rs:1109`). So most type-3/6 geometry would saturate at "nearest" and compare equal under GEQUAL.
   - Either this is real and the game relies on draw order (bucket sort + layers) for objects, or one header z term differs from what was read.
   - **Verify** with `gs_replay --packets` on `geo/f5000.gstrace`: check the Z of type-3 vs type-1 vertices for the same depth. Also check PCSX2's Z clamp for Z24 (GSdx HW clamps; real GS behaviour should be confirmed).
   - The native depth design must reproduce `min(Zgs, 2^24−1)` exactly for parity. Fixing it to "correct" depth is a separate visual decision.
2. **Fog on terrain:** types 1/2 set FGE with F = FTOI4(gS.w + gO.w). Live gS.w/gO.w read as 0, which would mean F = 0 = full fog colour. Read FOGCOL and F from a trace to settle it.
3. **Depth fog Z → index mapping** through the Z16→CT16 FBMSK 0x3fff copy. Derive it with the GS model on a synthetic Z ramp, then hard-code it in the native pass.
4. **Type-4 mode-0 header bug:** keep parity (first particle bucket after a material switch is nearly invisible) or fix it? The recommendation is parity in stages 0-4 and a fix flag after.
5. **Type 1 flag-0 pass:** ctx2 is missing on clipped triangles, and particle entries 0x510/0xa00/0xf10 have a c+1 off-by-one that blends stale data. Same question: parity or fix.
6. What uses Bezier tables idx 4/5 (relight) and 0x0df0/0x0eb0 (integer strip variant)? If no caller is ever found, drop them.
7. Contents of the Bezier LOD table 0x4a2a10, and the RenderState bits 0x180000 for the terrain ctx2 blend (lightmap vs detail).
8. Rider shadow: confirm the 128×128 RTT and how `BoaderShadow_Submit` projects it (texture id, blend). This decides how GPU-only textures enter the cache.
9. Slot 89 shadow volume (multi-pass, FBMSK alpha-only, destination alpha, FBA): only medium-confidence decode. It stays on the GIF→wgpu fallback until decoded. A float colour target is needed for Ad presets 16/17 (alpha up to 2.0).
10. Alpha-test AFAIL = FB_ONLY needs two pipelines per alpha-tested draw: pass/fail with depth write on/off, using `discard` both ways. This doubles the draw count for those buckets.
11. Colour clamp per blend (COLCLAMP after every blend) vs a float target that clamps only at the end. This matters for additive-then-multiply sequences, e.g. bloom composite after glows. Option: Rgba8Unorm plus a separate R8 "alpha×2" target, or clamp in a resolve. To be measured in stage 1/5.
12. Texture data mutation detection: is overriding `UpdateTextureData`/`LockTexture`/slot 53 enough, or do callers write pixels directly (streamed `strm_tex`)?
13. MSCAL double-buffer parity has to be modelled in the stage-0 VIF interpreter (types 3 and 6). Prefer an exact port of the VIF TOPS/ITOPS toggling.
14. Split-screen (`numViews` > 1): post passes run per view and viewport. The native passes must take the view rect from gm+0x59b0.
15. FMV (`UploadToDrawBuffer`/`Gs_UploadImage`) and the loading icon: these sit outside DeferMan. They need a small separate seam: upload the RGBA frame as a texture and present.

---

## Progress

### Stage 0a: the frame at the EE level (2026-10-16)

`crates/ssx3-render` reads the frame the game hands to VIF1 without the hardware model:
- `chain.rs`: the source-chain DMA walk;
- `vif.rs`: the VIFcode interpreter into a virtual VU1 memory. Each MSCAL/MSCNT becomes a
  `VuCall` {entry, TOP, ITOP, input qwords since the previous call}, and DIRECT becomes a GIF
  stream.

Check (`SSX3_VERIFY_VUCALLS=1`, `crates/ssx3-ps2/src/render_verify.rs`): at every VIF1 chain
start the walker runs over EE memory. Every emulated microprogram start then compares entry,
TOP/ITOP and each input qword with VU1 data memory. Boot → menus → race to frame 4800:
**2,574,506 calls, 0 mismatches**, none missing or extra, over 9,490 chains, plus 742,123
DIRECT packets.

Next:
- **Stage 0b:** group calls by DeferMan bucket (state, textures, matrix) into the typed
  `Frame`.
- **Stage 2A:** the first material natively, starting with type 3.

### Stage 2A, type 3 (2026-10-16)

`crates/ssx3-render/src/mat3.rs` is VU1 program d159716b in readable Rust. It covers every
entry the game uses:
- 0x12b8 float strips, with the guard-band test and a clear Sutherland–Hodgman clipper (plane
  order z−, x−, x+, y+, y−; ping-pong buffers 0x392/0x3ad);
- 0x3a08 ribbons;
- the mesh family 0x2270/0x22c8/0x2320/0x2358 with SH lighting, all texgen modes and pass 2;
- 0x2400 and the 0x3bf0 cubic line shape.

Check (`SSX3_VERIFY_MAT3=1`, `crates/ssx3-ps2/src/mat3_verify.rs`): at every type-3 call the
native code runs on a copy of VU1 memory and must produce the same XGKICK packets and the
same 16 KiB of memory as the translated program. Menus plus race to frame 4800: **998,528
calls, 0 mismatches**, all entries native, about 40,000 calls with clipped fans.

Notes:
- Nothing crosses calls except VU1 memory (`VuState` is empty).
- Per-vertex Q and clip results never leak between vertices, so a vertex shader can
  reproduce the maths.
- Exactness depends on the original operation order (w, z, y, x in the strip loop; the
  accumulator round trip), so the GPU version will be close, not bit-exact. `mat3.rs` is
  the bit-exact CPU reference.
- Declined, never seen in play: the pass-1 sphere map reads a stale vi11 pointer (a game
  bug), and inputs whose original loops would never terminate.

### Stage 2A, types 6/7: rider (2026-10-16)

`crates/ssx3-render/src/mat6.rs` is VU1 program 9fa47842 in readable Rust, covering every
entry:
- 0x00/0x398 lit draw (bone per vertex), 0x50 lit draw (one bone per batch);
- 0x10 morph, 0x20 shadow silhouette, 0x30 env pass, 0x40 ctx2 tag;
- 0x70/0x80 bone setup and 0x1150 SH lighting ×255.

Check (`SSX3_VERIFY_MAT6=1`): menus, character select and race to frame 4800, **1,434,419
calls, 0 mismatches** in kicked packets and VU1 memory.

Notes:
- State between calls lives only in VU1 memory. The env pass reads each vertex's Q back from
  the draw's output at TOP+0xb4, which explains the 0x50/0x30/0x30 parity.
- Morph quirk: an entry's target is read before the previous entry's result is stored, so two
  consecutive entries with the same target replace instead of adding.
- Shader inputs:
  - per draw: guard scale and offset, 10 SH coefficients;
  - per bone: M·B (positions), R (normals), E·R (env);
  - per vertex: {u, v, boneAddr, nrmAddr}, the normal (1.15), the position.
- The ADC rule: the first two vertices of a strip, or any of the last three vertices outside
  the guard band.

### Stage 2A, types 1/2: Bezier terrain (2026-10-16)

The Bezier programs are native Rust: `crates/ssx3-render/src/mat1.rs` (f4babac5),
`mat2.rs` (9790369f), and `bezier.rs` for the shared strip/fan emission, second passes, UV
projection and clipper. The clipper is not mat3's: it uses different buffers and
interpolates from the inside end of each edge.

Check (`SSX3_VERIFY_BEZIER=1`): race to frame 7000, **350,267 calls, 0 mismatches**, about
42,000 of them with clipped fans. That covers every n = 4/6/8 strip entry, the
projector/shine passes and 80 type-2 edge-LOD combinations.

Findings:
- Terrain F is always 0 with FGE off: fog comes only from the depth-fog post pass.
- Type 2 projector/shine reuse the previous call's tags and saved return address (qw511.x).
- Index-buffer layouts per n and per edge-LOD combination are documented in the module
  headers.

### Stage 2A, types 4/5: particles and snow (2026-10-16)

`crates/ssx3-render/src/mat4.rs` (f55ee634) and `mat5.rs` (e72540c9) are native Rust, with
the exact LFSR (`Lfsr`, checked against `Vu::rinit/rnext`).

Check (`SSX3_VERIFY_PARTICLES=1`): race to frame 7000, **50,321 type-4 calls, 0 mismatches**,
about 980,000 sprites. Entry 0x0f10 and the snow program did not run on that course; their
check is a unit test that runs the translated microcode on random inputs for every entry.

Corrections and findings:
- The type-4 header carried the worldClip matrix and qw4.x = 1024 (the guard scale) on every
  call. The "mode-0 header bug" of §1.5 was not seen, and §1.5 / `render.md` give 256 for this
  field, which looks wrong.
- Snow's depth fade (0x0408) compares against a reused register (a game bug; kept for parity).
- What a compute-shader version must reproduce: LFSR lanes and seeds, operation order, the
  per-entry colour and size clamps, and the corner order. See the module docs.

**All six VU1 rendering programs now have bit-exact readable Rust implementations.** The
seventh microprogram, 27649fa5, runs on VU0 for the EE and is not part of rendering.

### Native VU1 at run time (`SSX3_VU1=native`, 2026-10-16)

`vif::run_vu` dispatches each VU1 call to `ssx3_render::run_native` by segment hash and entry.
Calls that are not covered run the translated microcode.

Check: the full race replay to frame 9000 with native VU1 against the translated microcode is
**byte-identical: 0 of 36 screenshots differ, final RAM identical**, with about 6% less CPU
(155 s vs 164 s user). VU1 rendering now runs as readable Rust end to end.

Next: the wgpu stages (2B and on) draw from the same calls instead of producing GIF packets
for the GS model.
