# N2 map: typed draws at the game's render layer

Research for plan step N2 ([`../plans/native-rendering.md`](../plans/native-rendering.md)), 2026-10-18.

**Evidence marks:**
- **V**: verified in code (Ghidra decompile read in full for this purpose, generated Rust, or ELF/RAM-dump bytes).
- **D**: from decompiler reading of a part of a function, or from `docs/investigation/structs/render.md` without re-checking.
- **S**: from a scan of the generated code. The scan scripts were throwaway (session scratch):
  - `scan.py`: virtual calls through `gm+0x10d8` with a fresh vptr load. It misses calls where the vptr was cached across a call.
  - `callers.py`: direct `jal` calls.
  - `cls.py`: record classification.
  - Raw outputs: `slotcalls.txt` and `vt.txt` (the vtable 0x493260 read from the ELF).

## 0. Summary

1. There are exactly **30 functions that write DeferMan records** (V: scan for the `< 2600` record-cap check). Together with two bucket fast paths, they are the complete set of draw producers. Only **one producer sits outside the render layer**: `cPS2FogParticleMan_DrawParticles` 0x2dc7b0.
2. **Reading records at `RenderFrame` is not enough** (V). Two producers insert straight into buckets at draw time with `DeferMan_FindBucket` + `DeferMan_AppendPacket`, without writing a record:
   - **slot 95** (opaque models in the base states, `f12 == 1.0`);
   - **`PS2BezierMan_v18`** (the deferred type-2 patch list at BezierMan+0x5364).

   The complete draw list is the **bucket list after `BuildBuckets`**. That is what `ssx3-render::frame` already reads.
3. Nothing outside the render layer reads DeferMan, VifBuff/GifBuff, `gm+0x5a00` or `gm+0x5a8c..0x5ab4` (S).
   - Game code does read and write **`gm+0xe84` (the RenderState stack)** inline, in about 90 functions (S).
   - The draw slots **leave changes in `*curState`**. These are a guest-visible side effect that any replacement must keep.
4. **None of the packet-writing draw slots is overridden yet** (V, `overrides.txt`). Only the thin wrapper slot 96 and helpers (SprStream, GsPkt, AppendPacket, the view-matrix getters, TexMan small functions) are overridden.
5. **Recommended cut: two phases** (§5).
   - **Phase A, observe:** wrap each producer. Run the generated body, so RAM, cycles and event points stay identical, then capture a typed command keyed by its **VifBuff packet start**. At EndFrame, join the commands to the sorted bucket list.
   - **Phase B, replace:** family by family, with a behavioural gate or a masked RAM-identity gate.

## 1. Draw entry points (complete list of packet producers)

Calls go through vtable 0x493260 (vptr at gm+0x10d8, slot k at vt+8k+4). Overriding the implementation address catches every virtual call (V: `call_indirect` → `f_X` → OVERRIDE slot).

**Allocation:**
- **SPR**: `SprStream_BeginBlock` 0x38f460 / `EndBlockAligned` 0x38f668. The packet is built in scratchpad and DMA'd to `gm+0x5a00`.
- **direct**: written straight at `gm+0x5a00 | 0x30000000` (uncached).
- Every record writer then:
  - copies `*curState` (0x14 B) into record `count*0x80`;
  - writes start / end / matrix index (DeferMan+0x69cc0) / continuation flag;
  - sets curState w2[0:4] = DeferMan+0x69cc4 (scissor);
  - advances `gm+0x5a00` by 0x10 past the packet end, as room for RET/NEXT.

  (V for slots 71, 82, 95, 99 and EmitPatchType1; S for the rest.)

**Material types:** type = RenderState w0 bits 6-9. "(state)" means the slot keeps whatever type the caller set.

| Slot / fn | Addr | Signature (D unless V) | Type / path | Alloc | Callers (S; static call sites) |
|---|---|---|---|---|---|
| 71 DrawTriStrip | 0x379860 | (gm, n, verts[n] × 3 qw {STQ f32, RGBA i32, XYZW f32}) V | 3, VU1 MSCAL 0x12b8 (0x12b0 once if gp+0x1290) | SPR, split per 0x48 verts | 40 fns. HUD_* (race), MenuSys*, Render2D_DrawQuad* (8), UIIcon/UIListBox (menu), GameModule_DrawFullscreenQuad, cSpeedBoostStreamer/cAirStreamer/cAngerFX/cPlayerPulse/RailTube/PowerUpTrail (race FX), DebugMenu |
| BeginTriStrip / EndTriStrip | 0x37a260 / 0x37a430 | (gm, n) → vertex ptr in gm+0x5a2c | 3, VU1 | SPR | cSpeedBoostStreamerFX_Render |
| 72 DrawIndexedTriangles | 0x379bd0 | (gm, n, idx, pos, uv, col) | DIRECT (EE-projected) | direct | UIVector_Render |
| 73 DrawLineStrip | 0x381ad0 | (gm, n, verts) + LineStrip_Init/Finish 0x384d98/0x384e50 | DIRECT, EE near/far clip | direct | 11 fns: Avalanche_RenderActive, RailTube_Render, Render2D_DrawLine, UIListBox/UIVector, debug |
| 75 DrawLineShape | 0x381f10 | (gm, matrix, colour, n) | 3, VU1 0x3bf0 | direct | MultiSplineModifier_DebugDrawPath only (debug) |
| 78 DrawSprite3D | 0x377cf0 | (pos, size, uv0, uv1, colour) | DIRECT | direct | LoadIndicator_*, CrowdMan2d_RenderSprites, HaloModifier, cBoardSparkFX, RiderBase_RenderAttackFX, LensFX_DrawFlareSet/Sun |
| 79 DrawRotatedSprites | 0x3781a0 | (count, …) | DIRECT | direct | HaloModifier_DrawQuad, LensFX_DrawFlareQuad, cSplashManager_RenderDrop/Crystal |
| 80 / 81 DrawGlyphs(W) | 0x378808 / 0x379028 | font glyph quads | DIRECT | 80 direct, 81 SPR | Font_DrawText / Font_DrawTextW (both overridden callers) |
| 82 DrawEmitter | 0x380518 | (gm, desc: 21 qw) V | 4, VU1 MSCAL 0; pushes state, layer 7, sort mode 2 | SPR | Emitter_Render, cBoardSparkFX_Render |
| 83 DrawDynamicEmitter | 0x3807a0 | (gm, e): batches ≤ 64 particles | 4, VU1 0x510 | SPR | DynamicEmitter_Render |
| 84 DrawMEmitter | 0x380ce0 | (gm, e): batches ≤ 43, V4-8 colours | 4, VU1 0xa00/0xf10 | SPR | mEmitter_Render |
| 85 / 86 DrawTrail(B) | 0x381310 / 0x3816f0 | (gm, desc) | 5, VU1 0 / 0x408 | SPR | SnowBox_Render |
| 87 DrawCircle | 0x382170 | via slot 73 | DIRECT | — | SphereTree_WalkLevelsRecursive (debug) |
| 89 DrawShadowVolume | 0x386e78 | multi-pass | 3 + DIRECT | direct (+EmitShadowStrips 0x387ec0) | cBoardTrailFX_RenderBatch |
| 90 DrawBoardWake | 0x3883b8 | (+EmitBoardWakeStrips 0x3885e0) | 3, VU1 0x3a00/0x3a08 | direct | cBoardWakeFX_Render |
| 95 DrawModel | 0x37e238 | §2.1 V | 3, VU1 (mesh MSCAL 0x2270) | direct + **bucket fast path** | CrowdMan3d_GroupRender, DeadFade, MeshAnim ×2, SkyBox_Render, ModObj_Render/v05, slot 96 |
| 96 DrawModelSimple | 0x369cb0 | → 95(gm, inst, *(inst+0x94), inst+0x10, a2, a3, t0, f12 = 1.0) V | — | — | GameModule_RenderModels ×2 (Nash world instances), Conveyor, FlexBridge, cFlagManager, Fence. **Overridden** (graphics.rs) |
| 99 DrawSkinnedModel | 0x37a610 | §2.2 V | 6 (calls 124 first) | direct | RiderModel_Render ×2 (race riders, FE rider preview) |
| 100 DrawSkinnedModelShadow | 0x37b548 | like 99 | 7 | direct | RiderModel_RenderShadow ×2 |
| 101 SetShadowHeader | 0x37ba50 | (gm, qw[6]) | sets type 7 + w3 = packet | direct, no record | Rider_DrawShadow |
| 124 EmitLightingPacket | 0x396b40 | (gm), f12 V | sets type 6, w3 = vifCursor; packet `RET 0xc`: STCYCL, UNPACK 10 qw at 7 from gm+0x6bb0, MSCAL 0x1150; cursor += 0xb0 | direct, no record | slot 99 only (V) |
| 53 / 54 / 55 | 0x37d348 / 0x37d450 / 0x37d800 | render to texture / clear / textured sprite | DIRECT (GS FRAME switch) | direct | Rider_DrawShadow (53, 54, 55); EndFrame (54) |
| 61 DrawTransition | 0x37cc98 | screen capture into a texture | DIRECT | SPR | cTransitionMan_Render |
| GridMesh_Draw | 0x385c10 | (mesh, matrix, texId, flag); grid-mesh vt slot 8 | 3, VU1 | SPR + EmitModelHeader | Fence/Flag/FlexBridge meshes (via vtable 0x493208) |
| PS2BezierMan_v18 (end pass) + EmitPatchType1 0x38ca70 / EmitPatchType2 0x38ce20 / EmitPatchMatrixPass 0x38d168 / EmitPatchProjectorPasses 0x38d448 / RenderPatchFX 0x38d968 → DrawPatchFX 0x38d690 | 0x38b370 | §2.3 V | types 1, 2 (patches), 4 (PatchFX) | direct; PatchFX uses SPR; **bucket fast path** for the +0x5364 list | GameModule_RenderBezierPatches → EndBezierPass (both overridden) |
| cPS2FogParticleMan_DrawParticles | 0x2dc7b0 | virtual | (check) | SPR | GameModule_RenderFogParticles (via vtable) |
| RenderFrame post passes: RenderDepthFog 0x36ac00, BuildDepthFogClut 0x36a428, RenderBloom 0x36c790 (+EmitBlurDownsample/BlendedFill/BloomComposite), RenderColorFilter 0x3904a0 | | (gm, view) | DIRECT | SPR | RenderFrame only |
| TexMan_BindTextures | 0x368660 | GIF uploads into GifBuff + TEX0 | PATH3 | SPR | RenderFrame only |

**Per-frame record mix** (S): one race frame, `ram_gen.bin`, stale records of the last frame, 223 records. Fast-path buckets are not included, so models are under-counted.

| Type / layer | Records | Type / layer | Records |
|---|---|---|---|
| 3 L4 (slot 95, translucent) | 44 | 6 L4 (riders) | 11 |
| 1 L2 / 2 L2 (Bezier) | 15 / 15 | 4 L7 / L4 (particles) | 9 / 7 |
| 3 L11-14 (HUD strips) | 34 | 0 L1/L9-12/L18-24 (DIRECT, 2D) | about 45 |
| 3 L0 (slot 95: sky etc.) | 11 | 7 L1 (shadow) | 1 |

**Debug and disable globals** read by the slots (read-only): gp+0x1400 (95), gp+0x1404 (95 blend), gp+0x1420 / gp+0x1440 (99), gp+0x13e4 / gp+0x13e8 / gp+0x13ec (Bezier), 0x4a44f8 (82). All are 0 in the dump (V).

## 2. Data consumed by the top five

### 2.1 Slot 95 DrawModel (0x37e238, about 1480 decompiled lines; key parts V)

**Arguments** (Ghidra names): f12 = alpha (0 → return; 1.0 → opaque path), a0 = gm, a1 = instance, a2 = texture table, a3 = world matrix (4 qw), t0 = per-node matrices or NULL, t1 / t2 = extra matrix / flags.

**Flags (t2):**
- 0x400: skip the cull (slot 93 sphere/box test on inst+0x60/+0x6c against gm+0x58a0, then gm+0x5800);
- 0x20: needs clipping (set by the cull);
- 1: lit;
- 0x800: (use not established).

**Instance:**
- +0x08 flags (0x4000 → `WorldLightMan_LightInstanceCB`, lit);
- +0x10 matrix (passed by slot 96);
- +0x60 / +0x6c bounding-box min / max;
- +0x80 model descriptor;
- +0x84 scale;
- +0x92 flags (bit 0 → slot 94 matrix hook);
- +0x94 texture table;
- +0x98 mesh packet base (unlit).

**Model descriptor:**
- +0x04 node count;
- +0x08 nodes, 0x10 each: +0 parent / −1, +4 mesh list, +0xc matrix ptr;
- +0x10 flags (8 → depth-sorted translucent: push the state, sort mode 2, sort value from projected Z × gm+0x5920);
- +0x18..0x20 scale;
- +0x24 lit packet base (used instead of +0x98 when lit).

**Mesh list:**
- +8 → {+4 count, +8 → strip-group ptrs};
- strip group: short material index, short flags (bit 3 → 0x40000), int packet offset.

**Texture table entry** (param+4 + idx×4 → ptr): short tex0, short tex1, …, +0xc material flags:
- 0x180000: clamp mode → w0 bits 2-3;
- 0x660000: blend mode → w1 preset / ATST / ZTST / ZMSK, or a base state at DeferMan+0x57488 / 0x5748c / 0x57490.

**Per strip group:**
1. Writes the texture ids into `curState+0x10/+0x12` and clamp / blend bits into curState. **Not restored** unless descriptor flag 8 pushed a state (D).
2. Writes at the cursor:
   - `EmitModelHeader` 0x37e120 (V): `CNT 6 | STMASK 0, STCYCL 1,1, UNPACK V4-32 5 at 0 FLG`: node matrix (4 qw, node × scale, built in the scratchpad at 0x70003800+) + {0, 0, 0, flags};
   - optionally `EmitModelMatrix2` (an extra matrix);
   - optionally `EmitModelLighting` (lights for lit instances);
   - then `CALL (packetBase + offset)`.
3. Then one of three paths:
   - opaque in a base state: `FindBucket` / new bucket / `AppendPacket` (**no record**);
   - otherwise a record;
   - alpha < 1: **two records**. The second is a continuation (`+0x1e = 1`) with ZMSK and ALPHA FIX = alpha × 128.

**Mesh data** (V, RAM dump, CALL target 0xbb61d0): a **prebuilt DMA chain in the model file**, made of `REF` pairs and ending in `RET`.
- REF 1 (geometry, e.g. 0x25 qw):
  - `STCYCL 1,1; STMOD 0; STROW (position offset); ITOP`;
  - `UNPACK V4-32 2 at 0x1b` (GIF tag / header);
  - `V2-8 1 at 0x1d`;
  - masked `V2-16 ×n at 0x4a` (UV);
  - masked `V3-16 ×n at 0x1e` (positions, 16-bit + STROW offset).
- REF 2 (colours, 8 qw): `V4-5 ×n at 0x76`, `MSCAL 0x44e` (= 0x2270), `ITOP 1`.
- n ≤ 44 vertices per batch.

Lit instances CALL the same offsets on another base (+0x24), so their colour REFs point at per-instance copies (D).

**Guest-visible changes:**
- `gm+0x5a00`;
- VifBuff bytes;
- DeferMan: count, records, buckets, numBuckets +0x57494, bucket hash +0x678a0, sort list +0x67ca4 / 0x67ca8;
- `*curState` (texture ids, clamp, blend bits);
- `gm+0xe84` push/pop when flag 8;
- scratchpad 0x70003800.. (matrix scratch);
- `UpdateViewMatrices` when gm+0x6b90 == 0 (this also interns a MatrixRecord);
- the cycle charge.

### 2.2 Riders: slots 99 / 124 / 101 / 100 (0x37a610 V)

**Slot 99 arguments:** (gm, model, base, outlineFlag).
- Pushes the state, then calls slot 124 (f12 = 1.0), which sets type 6 and w3 = the lighting packet.
- Sets w0 bits 4-5 (clamp), tex1 = `gm+0x10b0[group.short@8]` (env texture). Sort mode 2 with sort value = projected Z of the root (spad+0x30) + 0x10, capped at 0x400.

**Model:**
- +0x20 offset → mesh groups, 0x14 each: +0 flags (1 = no morph, 0x100 = morph, 0x8000 = translucent + alpha FIX from byte +0xa), +4 texture id, +8 short env index, +0xc batch count, +0x10 → {packet ptr, morph ptr} pairs;
- +0x3c bone palettes {n, int* boneIdx};
- +0x40 float* morph weights;
- +0x48 short group count;
- +0x50 short morph count.

**Per group:**
1. `PS2GraphicsMan_SetTexFunction(texId, 3)`: **mutates TexInfo TEX0** (TFX / TCC).
2. Per bone batch:
   - `CNT 4n+2: STCYCL 1,1; UNPACK V4-32 4n at 0x11` + n bone matrices, copied from **scratchpad 0x70000000 + idx×0x40** (`gm+0x13e8`; written earlier by `RiderModel_BuildBoneMatrices`);
   - `UNPACK S-32 1 at 0x3ff = n`;
   - `MSCAL 0x80`, `MSCAL 0x70`;
   - then `CALL` the mesh packets (prebuilt, in the rider model file);
   - morph: `CALL a`, `CALL b` + `UNPACK V2-32 at 0xb4 {12800.0, weight}` + `MSCAL 0x398`, per weight > threshold.
3. One record per group.

At the end: pop the state; clear the type and w3 bits.

**Slot 124** (V): writes `RET 0xc` + 10 qw of gm+0x6bb0 + `MSCAL 0x1150`; cursor += 0xb0; sets curState type 6 and w3 = start.

**Slot 101** (D): 6-qword shadow header; type 7, w3. **Slot 100** (D): `FLUSH; DIRECT A+D RGBAQ` (shadow colour), then the bone batches as 99.

**What to capture:** model ptr; per group the texture ids, flags and blend state; the bone matrices (n × 64 B from the scratchpad, **capture at call time**); morph weights; the 10 lighting qw (gm+0x6bb0); the env index; the sort value.

### 2.3 Bézier terrain (V)

**Flow:**
1. `GameModule_BeginBezierPass` → BezierMan vt slot 17 (0x38b178) resets the queues.
2. `GameModule_DrawBezierPatch` 0x22c410 computes 4 corner LODs (distance-based, table 0x4a2a10) and calls BezierMan slot 21 → slot 22 (0x38b190) **queue**:
   - uniform LOD 4: +0x458 count, max 900, 0x10 entries;
   - uniform LOD 6: +0x3c9c, max 220;
   - uniform LOD 8: +0x4a60, max 50;
   - mixed: +0x4d84, max 75, 0x14 entries.

   An entry is {patch, lodSet, variant, short cacheSlot = patch+0x1a6/0x1a8/0x1aa}.
3. `EndBezierPass` → slot 18 `v18` 0x38b370:
   - builds the env / projector matrices into BezierMan+0x360.. / +0x3e0..;
   - `BezierPatchCache_MarkUsed`/`ReleaseUnused`;
   - per queued patch: if the cache slot is < 0, run `BezierPatchCache_TessellatePatch` (**EE tessellation**) then `PS2BezierMan_BuildPatchPacket(lod, slot)`;
   - `EmitPatchType1` / `EmitPatchType2`;
   - then `RenderPatchFX` (type 4, patch flag 0x800000).

**Patch** (world data): +0xc flags (0x600000 → matrix pass, 0x180000 → clamp), +0x170..+0x19c 4 corner positions, +0x1a0 / +0x1a2 tex ids, +0x1a6.. cache slot per LOD.

**Cache** (per LOD l = 0, 1, 2; grid (2l+4)², i.e. 16, 36 or 64 vertices):
- positions V3-32 at BezierMan+0x58/0x5c/0x60 + slot × 0xc0/0x1b0/0x300;
- UV1 V2-32 at +0x64/0x68/0x6c (×0x80/0x120/0x200);
- UV2 V2-32 at +0x70/0x74/0x78;
- per-slot packet at +0x44c[l] + slot × 0x100: REF-based, STROW/STMASK, three UNPACKs, RET.

**EmitPatchType1:** `CALL slotPacket; CNT 1; MSCAL table[0x44b9d0][lod].entry`, plus a record. If the patch is not drawable now, it goes to the deferred **+0x5364 list**, emitted later as type 2 through the bucket fast path (no records).

**Native:** one GPU vertex buffer per (lod, slot), keyed by a **generation bumped in `BuildPatchPacket`**. That function is called exactly when a slot is (re)tessellated (V). Per-draw data: tex ids, state, lodSet / variant (MSCAL entry = stitching variant), and the matrices of the extra passes.

### 2.4 Slot 71 strips (V)

Per draw: n vertices × 48 B copied verbatim into the packet (STQ, RGBA i32, XYZW). Transform = the current MatrixRecord (worldView + guard), through type 3.
- The state forces type 3 at the start and clears type and w3 at the end (`curState` w0 &= ~0x3c0, w3 &= 0xc0000000).
- `gp+0x1290` is cleared.
- PRIM 0x5c textured or 0x4c untextured (tex[0] < 0).

What to capture: the vertices, `*curState`, matrix index, entry 0x12b8 / 0x12b0.

### 2.5 Particles 82-84 (82 V, 83 / 84 D)

- **82:** pushes the state: type 4, sort mode 2 with value 0, layer 7. `UNPACK V4-32 21 qw at 0` (descriptor copied verbatim), `MSCAL 0`, `FLUSH`. One record; pops the state. The particles are generated by the VU1 program from the descriptor.
- **83 / 84:** descriptor at e+0x20 (21 qw) plus per-batch count / time and particle positions, and V4-8 colours for 84.

What to capture: the descriptor qwords and particle arrays (copied at call time; emitter memory is game-owned and mutates every frame).

## 3. DeferMan

**Object:** `EA_MemAllocNamed(0x6b100)`, at gm+0x18f0 (live 0x8095f0). Layout in `structs/render.md`.

**Record** (0x80; only 0x20 used) (V):

| Offset | Content |
|---|---|
| +0x00 | RenderState (0x14) |
| +0x14 | packet start |
| +0x18 | packet end |
| +0x1c | short matrix index |
| +0x1e | short continuation |

Records are written uncached at `dm + 0x80 + i×0x80`; cap 2600. Over the cap, the record is silently dropped, but the packet and the cursor advance still happen.

**Consumption** (`RenderFrame` 0x363490, called only from EndFrame; V):
1. `DeferMan_BuildBuckets` 0x362de8 (V): per record:
   - `InternState` (hash of w0-3);
   - if not a continuation, `FindBucket(state, tex, matrix)`, else a new bucket chained to the previous one (prev+0x18 = new, prev+0x14 = 1);
   - new non-continuation buckets get `ComputeSortKey` → sort list;
   - `AppendPacket` writes `NEXT` at the previous end.
2. `SortBuckets` (radix).
3. Per bucket: `EmitStateChange` (+ `EmitMaterialSetup` per type / mode), `TexMan_BindTextures`, `CALL` / `RET`.
4. `RenderDepthFog` after layer 5; END after layer 8; then `RenderBloom` / `RenderColorFilter` and the phase-2 layers.
5. VramHeap scratchpad round trip; `TexMan_ResetUsageTable`.

**EndFrame** 0x377a10 (V):
1. slot 24 viewport, slot 54 clear;
2. RenderFrame;
3. publish gm+0x5aa4..0x5ab4, bufState = 2, signal semaphores;
4. flip halves (+0x5a10 / +0x5a24, cursors, InvalidDCache);
5. reset `*curState` to the default 0x501420;
6. `DeferMan_ResetFrame`: count 0, numBuckets 0, numStates = base, hash tables cleared via scratchpad 0x70003800 memset + Download, numSorted 0, emitState default, scissor index −1, numScissors 0, matrix index −1, numMatrices 0;
7. `TexMan_EndFrameStub`; viewport again.

**Readers outside the render layer** (S): **none**. Offsets +0x18f0 / +0x5a00..+0x5a24 / +0x5a8c..+0x5ab4 / +0x6d0c in functions outside 0x362000-0x39a000 hit only cPS2FogParticleMan_DrawParticles (a producer) and unrelated objects (SSXAudioSystem object loops, using their own +0x5a00 fields).

Game code reads and writes `gm+0xe84` / `*curState` directly (about 90 functions: HUD_*, MenuSys*, Rider_*, FX renders). This is the input interface, and draw slots leave changes in it (§2.1, §2.4).

## 4. TexMan

**Registration:** `TexMan_LoadTexture` 0x367440 (V): `(tm, pixels, name, w, h, maxMip, format, clutFmt (1 = 32-bit, 2 = 16-bit), clut, residency, ownsCopy, id (−1 = dynamic))` → id.
- Callers: only `PS2GraphicsMan_LoadTexture` (slot 47) and `TexMan_LockTexture`. Slot 47 is reached from slot 46 `LoadTextureFromShape` (19 callers: FE / game load, UIEngine, Font_UploadTexture, VD_TextureSink, chunk resolve).
- Fills TexInfo (0x58, in `tm+8 + id×4`; layout in `structs/render.md` §TexMan, V here):
  - +0x0c format;
  - +0x1c pixels (copied if ownsCopy);
  - +0x20 clut;
  - +0x24 mip count;
  - +0x38 TEX0 (PSM from format: 2→CT16, 8→CT24, 9/10→T8 (0x13), 0xb/0xc→T4 (0x14); TW / TH; CLD; TCC when bpp < 9);
  - +0x40 TEX1;
  - +0x48 / +0x50 MIPTBP;
  - residency 1 / 3 → permanent VramHeap block; format 9 → T8H in the temp heap via `CopyToTempTex` (`SwizzleT8`).

**Mutators** (a native cache must observe them):

| Slot / function | Callers | Effect |
|---|---|---|
| slot 49 → `TexMan_UpdateTextureData` 0x367d20 | VD_TextureSink_UploadFrame (movies), slot 48 | new pixel data |
| slot 52 → `TexMan_LockTexture` 0x367cd0 | BoaderShadow_AllocTexture, slot 59 | render-target textures |
| slots 53 / 61 | | GS renders into a texture's VRAM |
| `PS2GraphicsMan_SetTexFunction` 0x37cbc8 → `TexMan_SetTexFunction` (overridden) | slot 99, per group | mutates TEX0 every draw |
| `SetTextureFilter` → `TexMan_SetTex1Filter` (overridden) | | mutates TEX1 |
| slot 50 FreeTexture | | frees the id |

- **CLUT animation** (sky palettes): no TexMan call was found. It is presumably written in place (**I**).

**Key for a native cache:**
- texture: (id, pixels ptr, clut ptr, format, w / h / mips from TEX0) + generation;
- generation bumped by overrides of LoadTexture / FreeTexture / UpdateTextureData / LockTexture and by slots 53 / 61;
- plus a CLUT hash, or a hook on whoever writes palettes;
- **sampler and TFX state per draw**: TEX0 TFX / TCC and TEX1 are taken from TexInfo at bind time (RenderFrame), so snapshot them per bucket at EndFrame, not at registration.

## 5. Recommendation: where to cut

**Reject records-only at RenderFrame:** it misses the slot 95 and Bezier fast paths. Use the **bucket list** (as `ssx3-render::frame` already does) or per-slot capture.

### Phase A (RAM identity holds by construction)

1. **Producer wrappers.** For each producer, an override that:
   - remembers `gm+0x5a00` (packet start) and the inputs;
   - calls `f_X__generated`;
   - pushes a typed command into a per-frame `HashMap<packet_start, Cmd>`, with data copied now: vertices, descriptors, bones from the scratchpad, lighting, `*curState`, matrix index.

   Meshes are referenced by guest address plus a cache generation: slot 95 CALL target → REF chain; rider mesh packets; Bezier (lod, slot) bumped in `BuildPatchPacket`.
2. **EndFrame wrapper** (or a hook between `RenderFrame` and `DeferMan_ResetFrame`; RenderFrame is called only from EndFrame):
   - copy the sorted bucket list, buckets, interned states, MatrixRecords, scissors, per-view post parameters and the TexInfo snapshot for the used ids;
   - join each bucket's packet starts (first start, then continuation chain) to commands.

   This is a few tens of KiB instead of 32 MiB, and enables N3.
3. **Fallback:** a packet range with no command → current `VuCall` / GIF path (`ssx3-render::frame`). This lets the families move one at a time.
4. **Order** (most frame weight first; each one is self-contained):
   1. 71 + Begin/EndTriStrip;
   2. 95 (with the fast path) + GridMesh (shares EmitModelHeader);
   3. 99 / 100 / 101 / 124;
   4. Bezier v18 / EmitPatchType1 / EmitPatchType2 / MatrixPass / ProjectorPasses / DrawPatchFX;
   5. 82 / 83 / 84, 85 / 86;
   6. DIRECT 2D: 72, 73 (LineStrip_Finish), 78, 79, 80, 81;
   7. 89, 90, FogParticles, 53 / 54 / 55 / 61, post passes (already typed in `frame.rs`).

### Phase B (optional, after acceptance)

Skip packet building inside the wrappers. A replacement must keep:
- `*curState` and `gm+0xe84` effects;
- DeferMan buckets / records, or replace RenderFrame as well;
- MatrixRecord interning and the `gm+0x6b90` valid flag (`UpdateViewMatrices`);
- TexInfo TEX0 mutation (slot 99);
- gp+0x1290 clear;
- the record-cap behaviour;
- the cycle charge and event points. Generated loops check `next_event`; interrupt timing depends on cycles.

With those differences, RAM identity holds only with VifBuff, GifBuff, DeferMan, scratchpad and SprStream masked **and** cycles kept equal. Otherwise the gate is behavioural (as already for depth readback, `plans/phase3-draw-layer.md`).

### Risks

- **Mesh memory reuse:** world streaming (`cWorldCache`) frees and reloads model files. Key meshes by address + content hash of the REF chain (cheap: hash the REF tags plus the first qwords), or hook load / unload. Lit-instance colour copies may be rewritten (**I**).
- **VifBuff double buffering:** packets of frame N are overwritten two frames later. Commands must copy at record time and never point into VifBuff.
- **Bones and model matrices live in the scratchpad:** capture during the call.
- **`DmaChain_PatchRefTags` 0x372198** (no direct callers found; it is entered as a fall-through or computed jump) probably relocates model REF tags at load (**I**). Hooking it would give a mesh-load event.
- **Order and layers:** LensFX occlusion needs depth after layers ≤ 8. Keep the bucket order and the layer split.
- **Bucket-only draws** have no record; anything counting records under-counts them.
