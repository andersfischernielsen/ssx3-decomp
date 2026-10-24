# World instances from RAM (for a native shadow-caster pass)

How to find every static world model (trees, signs, fences, buildings,
rails) in EE RAM and turn it into world-space triangles without running game
code or the VU1 program. Verified against the code (Ghidra, `generated/ee`,
the hand ports in `crates/ssx3-game/src/world.rs` and
`crates/ssx3-game/src/packetless/model.rs`) and against the race dump
`target/gate/base/ds1.bin` (Snow Jam, frame 6000): with the recipe below, all
1240 drawable instances decode to vertices that lie inside the instance's
world AABB (`+0x60..+0x78`), 3599 mesh batches and 126 656 vertices in all.

Confidence: **H** = read in code and checked in the dump, **M** = read in code
or inferred from consistent dump data, **L** = guess.

## 1. Enumerating instances

### Roots

| Where | What | Conf |
|---|---|---|
| `g_App` = `*(0x4a28a8)` (gp-0x848) | `SSXApp*` | H |
| `App+0x84` | `GameModule*` (dump 0xcbfa20) | H |
| `GameModule+0x20` | `Nash*` (0xa8 bytes, dump 0x5bc600) | H |
| `GameModule+0x10` → `World+0` | `cWorldCache*` (for the section filter, see below) | H |
| `Nash + 0x14·o`, o = 0..7 | octant record `{i32 level; i32 x, y, z; Node* root}`; o = 4·(x<0) + 2·(y<0) + (z<0); empty octants have root 0 (and stale key words) | H |

### Node (0x2c bytes, `EA_MemAllocNamed(0x2c)` in `SpatialHash_Insert` 0x328c20)

| Off | Type | Meaning | Conf |
|---|---|---|---|
| 0x00..0x1c | `Node*[8]` | children, index = 4·xbit + 2·ybit + zbit of the child cell (cell = (level−1, 2x+xbit, 2y+ybit, 2z+zbit)); null = absent | H |
| 0x20 | `Instance*` | list type 0: **instances** (models) — `Nash_CollectInstances` 0x22a5a0 | H |
| 0x24 | `*` | list type 1: Bézier patches (`Nash_CollectPatches`) | H |
| 0x28 | `*` | list type 2: effects (fog particles type 7, lens flares type 8) | H |

The cell of a node is implicit (the recursion carries it; the level goes down
by one per child, leaves at level 11 = 2048 units). For enumeration the cells
are not needed: walk the tree from the 8 roots and collect every node's +0x20
list. Lists are doubly linked through the object: `+0 next`, `+4 prev`
(H, `SpatialHash_Insert`). Dump: 2203 nodes, 3503 list entries.

The list entries **are** the instances (no wrapper). `Nash_CollectInstances`
appends the entry pointer itself to the render list, and
`GameModule_RenderModels` passes it straight to `DrawModelSimple`.

### Which entries are drawn (kind word `Instance+8`)

| `inst+8 & 3` | Dump count | What | Drawn by | Conf |
|---|---|---|---|---|
| 3 | 2920 | static model instance, `+0xc` (WorldObj) = 0 | `GameModule_RenderModels` (this doc) | H |
| 1 | 218 | owned by a `WorldObj` (`+0xc` ≠ 0): Object 0x490e80, Hide 0x491800, AnimObject 0x490b10, DeadNode 0x491b00, AnimTeeter, RestoreNode | its process' `ModObj_Render` 0x356298 (see §4) | H |
| 2 | 354 | 342 without WorldObj (mostly blended materials), 12 Particle WorldObjs | not drawn by RenderModels; bit 0 looks like "shown" (Hide objects / `World_ResetSections` toggle it) | M |
| 0 | 11 | DeadNode / RestoreNode leftovers | no | M |

`Nash_CollectInstances` (and `Nash_ClassifyInstance` 0x229fc8 for partially
visible nodes) draw an entry iff `(inst+8 & 3) == 3` **and** its section is
active:

```
cache = *(World*)(GameModule+0x10)          // cWorldCache*
s = u8(inst+0x7d)                            // world section, 0xff = shared (always on)
ok = s == 0xff
  || ( u32(cache + 0x24 + 8*s) == 6          // SectionStreamer state: resident+active
    && u32(cache + 0x3f0 + 0x18*i16(inst+0x7e)) == 3 )   // hull slot state: resident
```

(H. The original reads `**(visible+0x90)`, which is `*(GameModule+0x10)` =
`World->cache`.) Then a sphere test against the view (VU0 0xef0, sphere at
`inst+0x50`) and the occluders: a shadow pass skips those. There is **no
distance LOD** in the instance path: no LOD selection in Collect/Classify or
in `DrawModel`; the only distance gating is the hull streaming above (hull
priority comes from the views' positions, so it is distance-driven). Dump:
1240 of the 2920 kind-3 instances pass; the 1680 others belong to hull slots
in state 0 (their model data still decodes correctly, so the gate is a
draw policy, not a memory-safety one — keep it anyway so shadows match what
the game would draw when the camera turns).

## 2. The instance (0xb0 bytes, "Instance"; InstancesMan pool or section memory)

| Off | Type | Meaning | Conf |
|---|---|---|---|
| 0x00 / 0x04 | `Instance*` | Nash list next / prev | H |
| 0x08 | u32 | flags: bits 0-1 kind (above); bit 2 shown (ModObj path); 0x100 no DangerMan; **0x4000 SH-lit** (normals, chain base from the descriptor) | H (0x4000, bits 0-1), M (others) |
| 0x0c | `WorldObj*` | owner (kind 1), else 0 | H |
| 0x10..0x4f | f32[4][4] | **world matrix**, row-vector convention: rows 0-2 = model x/y/z axes in world space, row 3 = translation (w = 1). `world = x·r0 + y·r1 + z·r2 + r3`. Z is up (rotations seen are about z) | H |
| 0x50 | vec4 | bounding sphere (centre, radius) — the cull tests use it | H |
| 0x60 / 0x6c | vec3 / vec3 | world AABB min / max (`DrawModel`'s ClipTestBox) | H |
| 0x78 | u32 | resource handle | M |
| 0x7d | u8 | world section (0xff shared) | H |
| 0x7e | i16 | hull slot ("logical section") | H |
| 0x80 | `ModelDesc*` | model descriptor (below) | H |
| 0x84 | f32 | uniform instance scale (also scales node translations) | H |
| 0x92 | u16 | bit 0: plant sway (`PS2GraphicsMan_v94` 0x36a2c0) | H |
| 0x94 | `MatTable*` | material / texture table (below) | H |
| 0x98 | u32 | chain base for unlit instances (per-instance copy: model-data REFs + this instance's vertex colours) | H |

Coordinates are large (|x|, |z| ≈ 2–4·10⁵ units on Snow Jam): build
light-space matrices camera-relative (subtract the camera/shadow origin in
f64 or before converting) to keep float precision.

### ModelDesc (`inst+0x80`)

| Off | Type | Meaning | Conf |
|---|---|---|---|
| 0x04 | i32 | node count (dump: 1 for 357 of 372 descs; up to 15) | H |
| 0x08 | `Node[count]` | 0x10 each: `{i32 parent (-1 root); MeshList* ml (0 = no geometry); ?; f32[16]* local}`; a single node with `local == 0x4ff1a0` (identity) needs no node matrix | H |
| 0x10 | u32 | flags: 2 = environment texgen facing the eye (only the texgen matrices, **not** the geometry); 8 = depth-sorted/blended model; 1 = ? | H (2, 8), L (1) |
| 0x18 | vec3 | **dequantisation scale** (per axis; = the max |extent| of the mesh) | H |
| 0x24 | u32 | chain base for SH-lit instances (`inst+8 & 0x4000`) | H |

### MeshList (`node.ml`)

| Off | Type | Meaning | Conf |
|---|---|---|---|
| 0x00 / 0x0c | vec3 / vec3 | model-space AABB (used for the per-node clip with flag 0x1020) | H |
| 0x1c | u32 | group count | H |
| 0x20 | `Group*[count]` | groups | H |

Group: `{i16 material; u16 flags (bit 3: force sorted); u32 chainOffset}` (H).
The group's DMA chain is at `chainBase + chainOffset`, where chainBase =
`desc+0x24` if `inst+8 & 0x4000` else `inst+0x98`.

### MatTable (`inst+0x94`) and materials

`+0 count; +4 Material*[count]`, indexed by `group.material` (H). Material:
`+0 i16 tex0` (TexMan id, −1 none — the renderer already resolves these via
TexMan, `crates/ssx3-render/src/frame/pages.rs`), `+2 i16 tex1` (second-pass
environment texture), `+0xc u32 flags`. Flag field `& 0x66_0000`
(`packetless/model.rs` `draw_model`): `0` opaque, `0x2_0000` alpha-tested
cut-out (its own AREF state; 1059 of the dump's drawn groups — the trees),
`0x4_0000`/`0x6_0000` blended + depth sorted, `0x20_0000` adds the env pass;
bits `0x18_0000` = UV clamp. (H for the dispatch, M for the names.)

### The world matrix of a mesh (from `PS2GraphicsMan_DrawModel` 0x37e238)

`DrawModelSimple(inst)` = `DrawModel(alpha 1, inst, texTable = inst+0x94,
world = inst+0x10, nodeMatrices = 0, matrix2 = 0, flags)`. With row vectors
(`a·b` = rows of `a` through `b`, i.e. `vu0::mul_m(b, a)`):

```
sc = f32(inst+0x84);  S = diag(desc.scale.x·sc, desc.scale.y·sc, desc.scale.z·sc, 1)
W  = inst+0x10                       // (· Sway if inst+0x92 & 1, see §4)
N[i] = L[i] · N[parent]  (L[i] = *node[i].local with its row 3 xyz ·= sc; root: N = L)
mesh matrix of node i:  H[i] = S · N[i] · W        // VU1 header qw0-3 ("model matrix")
world position = (x/32768, y/32768, z/32768, 1) · H[i]
```

(H: this is exactly `S[0..3]` that `draw_model` uploads; checked on all 1240
instances incl. 486 with node hierarchies.) The VU1 program then multiplies by
its own world→screen matrix (VU1 qw `MATRIX`), so `H` is the full
model→world transform.

## 3. Mesh data (VU1 type 3, entries 0x2270 / 0x22c8)

A group chain is a list of REF tags ending in RET; each batch is two REFs:
first the **shared model data** (VIF codes + header, strip lengths, UVs,
positions; same address for every instance of a descriptor), then the
**per-instance** part (vertex colours + `MSCAL`), or model-only for lit
instances. One batch (VIF1, `STCYCL 1,1`, `FLG` = TOPS-relative):

| VIFcode | VU addr (first batch / later) | Content | Conf |
|---|---|---|---|
| `STMOD 0`, `STROW`, `ITOP 0` | | | H |
| `UNPACK V4-32 num 2` | 0x1b / 0 | qw0 GIF tag: PRIM 0x5c (tri strip, IIP, TME, ABE), NREG 3, NLOOP; qw1 `{strips, _, n, _}` | H |
| `UNPACK V2-8 num strips` | +2 | strip lengths (x lane); Σ = n (checked on all batches) | H |
| `UNPACK V2-16 num n` (masked) | +2+strips+n | UVs, **signed 4.12** (`/4096`, normalised ST) | H |
| `UNPACK V3-16 num n` (masked, w from ROW) | +2+strips | **positions, signed int16 → ITOF15 = `/32768`**, then × `desc.scale·sc` via `H` (matches mat3.wgsl `xyz / 32768`) | H |
| `UNPACK V4-5 num n` | +2+strips+2n | per-instance RGBA5551 colours (unlit); lit: a `V3-16` of normals here instead | H |
| `MSCAL 0x44e` / `0x459` | | 0x44e·8 = **0x2270** (first batch of a group: header at TOP, mesh at TOP+0x1b) / 0x459·8 = **0x22c8** (later batches: mesh at TOP, matrix kept) | H |
| `ITOP 1` | | | H |

Materials with a second (env) pass append, after the pass-1 batches,
`MSCAL 0x480` (0x2400, PRIM-only kick) and for each batch a `V3-16` of
**normals** into the colour slot + `MSCAL` with ITOP 1 (pass 2 over the same
positions). A decoder must therefore take the position `V3-16` only as the
first `V3-16` after a batch's `V4-32 num 2` header (at header addr + 2 +
strips); later `V3-16`s in the same batch are normals. 0x2320 / 0x2358 did
not occur for instances in the dump.

Triangles: each strip of length L is a plain triangle strip of L vertices
(the program sets ADC on the first two of each strip; winding alternates).
Dump sizes: groups have 1–20 batches, batches ≤ ~62 vertices.

### Relation to `VuCall::src` / `GpuCall::mesh_key`

Not a RAM address. `src = recording id << 16 | call index`, where the
recording id is a counter (`ChainCache::next_id`, `ssx3-render/src/frame.rs`
`play_chain`) for a cache entry keyed by `(chain address, VIF regs)`. The
chain address is the group chain `chainBase + chainOffset` above — the same
address the recipe computes, but **per instance** (chainBase = `inst+0x98`), so
two instances of one model get different `src`/`mesh_key`s; ids change when
the cache is cleared (> 8192 entries). For a shadow pass, key the GPU
geometry by the **first REF address of the batch** (the shared model data)
or by `(desc, group, batch)`: positions/UVs are shared across instances, and
only the colours (not needed for shadows) are per instance.

## Recipe

```
for o in 0..8: root = Nash[o].root; if root: walk(root)        // DFS over children +0..+0x1c
  for inst in list(node+0x20) (next = +0):
    if inst+8 & 3 != 3 or !section_active(inst): continue
    desc = inst+0x80; S, N[], W as in §2   (cacheable: kind-3 instances never move)
    base = (inst+8 & 0x4000) ? desc+0x24 : inst+0x98
    for node i with ml = node[i].ml != 0:
      H = S·N[i]·W
      for g in ml.groups[0..ml+0x1c]:
        mat = MatTable(inst+0x94)[i16 g+0]; tex0 = i16 mat+0; cutout = mat+0xc & 0x2_0000
        walk DMA chain at base + u32(g+4) (REF/CNT/NEXT/RET; chain.rs walk_segments)
        parse VIFcodes; per batch (starts at V4-32 num 2):
           lens = V2-8, uv = V2-16 (/4096), pos = first V3-16 (/32768)
           emit triangle strips of lens[k] vertices: world = (pos,1)·H
```

Cost (Snow Jam frame 6000): 1240 instances, 372 distinct descriptors, ~3.6k
batches, ~127k vertices; all of it static between section loads, so decode
once per (descriptor, group) and draw instanced with `H` per instance.

## 4. What makes it harder

| Issue | Detail | Conf |
|---|---|---|
| Streaming | Sections and hull pages stream in/out; gate with the section/hull states every frame (§1). Instances of an unloaded section leave the lists via the chunk resolver's Release (not traced); read RAM only at frame build on the game thread | M |
| Plant sway | `inst+0x92 & 1` (8 instances in the dump, 2-node shrubs): when `PS2GraphicsMan+0xa4` ≠ 0, `W = VfxPlants_GetSwayMatrix(W.row3) · W` (0x38f2a8, wind computed per frame by game code). Use the static `W` for shadows (tiny error) | H |
| Billboards | **None** in this path: the geometry matrix never depends on the camera. Desc flag 2 builds an eye-facing rotation for the environment texgen matrices only (`packetless/model.rs` `env`). Trees are ordinary meshes with alpha-tested materials (`0x2_0000`) — cut-outs need tex0 + UVs | H |
| WorldObj-owned (kind 1) | Drawn by `ModObj_Render` 0x356298 per process, also only when on screen (`ClipTestBox` → skip, `World_IsOccluded`). Shown iff `inst+8 & 4` and `WorldObj+0x28 == 0`; alpha `WorldObj+0x24`; same section gate. Matrix: `WorldObj` vt slot 0xc4 = the transform modifier's matrix (`ModifierBlock+0`, vt +0x94) if any, else `inst+0x10`. AnimObjects (0x490b10) pass node matrices from vt slot 0xdc (animation, computed per frame) and identity as world. Static `Object`s can use the same recipe with `inst+0x10`; moving/animated ones need the modifier/animation state (read the matrices the game last computed, or skip) | H (code), M (which are static) |
| Hidden (kind 2) | Not drawn by the game; exclude | M |
| Lit instances | 0x4000 (7 in the dump): chain from `desc+0x24`, normals instead of colours; positions identical format | H |
| Occluders | Only cull for the camera; irrelevant to shadows | H |
| Sorted/blended materials | `0x4_0000/0x6_0000` (glass, decals?) — probably exclude from casters or cast at reduced strength | L |

## Open questions

- Meaning of desc `+0x10` bit 0 (260 descs), desc `+0x00/+0x0c/+0x14`, inst `+0x88/+0x8c/+0xa0/+0xa4`, node entry word 2.
- What a "hull" slot (`inst+0x7e`) gates exactly (instances in non-resident hulls decode fine in the dump).
- Whether kind-2 entries are always "hidden by script/Hide" or include another class drawn elsewhere.
- Whether released sections unlink their instances from Nash immediately (the gate makes this moot for reading).
