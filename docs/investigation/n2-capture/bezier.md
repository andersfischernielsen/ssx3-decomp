# N2 capture spec: Bézier terrain (VU1 types 1, 2 and the type-4 patch FX)

Sources: Ghidra decompile + disassembly of every function below, generated
Rust for the two non-Ghidra functions (`f_0038b178`, `f_0038b190`, s12),
`crates/ssx3-render/src/{mat1,mat2,bezier}.rs`. **V** = read in the
disassembly (or the generated Rust), **I** = inferred.

Notation: `bm` = the PS2BezierMan object (a0 of every `PS2BezierMan_*`);
`C = bm+0x10` = its BezierPatchCache; `gm` = PS2GraphicsMan (`gp+0x2a90`,
= `*0x4a5b80`, V); `cur` = `&gm+0x5a00` (VifBuff cursor; passed as the
cursor pointer to every emitter, V); `st` = `*(gm+0xe84)` (current
RenderState, 0x14 bytes, pushed/popped by `+=/-= 0x14`); `dm` =
`*(gm+0x18f0)` (DeferMan). `n = 2·lod + 4` (lod 0/1/2 → 4/6/8 grid). All
packet writes go through the uncached alias `addr | 0x30000000`; guest
addresses below are the physical ones.

## 0. Flow per frame (V)

```
GameModule_RenderBezierPatches 0x22c620
  GameModule_BeginBezierPass 0x22c1b0 → bm vt17 0x38b178: zero the 4 queue counts
  for p in list +0x50a8: GameModule_DrawBezierPatch(ctx, p, flag=0)     0x22c410
  for p in list +0x70ac: GameModule_DrawBezierPatch(ctx, p, flag=1)
     → bm vt21 0x38b338 (returns if gp+0x13e4 != 0) → vt22 0x38b190: queue
  GameModule_EndBezierPass 0x22c3b8 → bm vt18 PS2BezierMan_v18 0x38b370:
     matrices, MarkUsed/ReleaseUnused,
     lod-8 uniform queue: [Tessellate+BuildPatchPacket] EmitPatchType1(lod 2)
     lod-6 uniform queue: … EmitPatchType1(lod 1)
     lod-4 uniform queue: … EmitPatchType1(lod 0)
     mixed queue:         … EmitPatchType2
     deferred list +0x5364 flush (bucket fast path, no DrawRecords)
     RenderPatchFX 0x38d968 → DrawPatchFX 0x38d690 per lod-8 patch
```

### 0.1 GameModule_DrawBezierPatch 0x22c410 (a0 ctx, a1 patch, a2 flag) (V)

Edge LOD = byte table `0x4a2a10[i]`, `i = (int)clamp(ctx[9] + ctx[8]·(dA+dB),
ctx[10], ctx[11])` (float clamp: below min → min, ≥ max → max), `dK` =
squared distance from the camera (ctx[4..6]) to corner K (patch+0x170 +
12·K). `lods[0]` = corners 0-1, `lods[1]` = 2-3, `lods[2]` = 0-2,
`lods[3]` = 1-3 (= mat2's x = row 0, y = row 7, z = column 0, w = column 7).
Calls vt21 with a1 patch, a2 `&lods` (u8[4]), a3 `&{tex1, tex2}` (unused),
**t0 = flag** (a2 moved to t0 at 0x22c424). `flag` selects which pass-2
variant the VU uses (0 = "ST rewrite", 1 = "full re-draw with clipper");
the two source lists are presumably Nash_ClassifyPatches' "inside" /
"crosses the guard band" sets (I).

### 0.2 Queue (vt22 0x38b190; a0 bm, a1 patch, a2 lods, t0 flag) (V)

| lods | queue (count, base, max) | entry |
|---|---|---|
| all 4 | +0x458, +0x45c, 900 | 0x10 B: `+0 patch, +4 flag, +8 = 0 (lod), +0xc s16 slot = patch+0x1a6` |
| all 6 | +0x3c9c, +0x3ca0, 220 | same, `+8 = 1`, slot = patch+0x1a8 |
| all 8 | +0x4a60, +0x4a64, 50 | same, `+8 = 2`, slot = patch+0x1aa |
| mixed | +0x4d84, +0x4d88, 75 | 0x14 B: `+0 patch, +4 flag, +8 = 2 if any edge is 8 else 0, +0xc s16 slot = patch+0x1aa, +0x10 u8[4] lods` |

Full queues drop the patch silently. Queues and the deferred list stay
valid in bm until the next BeginBezierPass / v18 (v18 zeroes +0x5364 at
entry, V).

### 0.3 Patch (world data, read through a 0x1b0-byte scratchpad copy `SprStream_Upload(patch, 0x1b0, 2)` 0x38f708) (V)

+0xc flags: 0x600000 → shine ("matrix") pass, 0x400000 picks its blend
bits, 0x180000 → ctx1 blend bits (st.w0 bits 2-3: 0x100000 → 4, 0x80000 →
8, 0x180000 → 0xc), 0x800000 → glitter FX. +0x40..+0x13f: 16 power-basis
coefficient qwords. +0x158/+0x164: bbox min/max (xyz). +0x170..: 4
corners. +0x1a0 / +0x1a2: tex ids ctx1 / ctx2. +0x1a4: shine texture
(< 0 → gm+0x10a8). +0x1a6/+0x1a8/+0x1aa: s16 cache slot per lod (-1 = none).

## 1. The cache (cached across frames)

### 1.1 Layout (Init 0x38af30, BezierPatchCache_Init(C, 900, 220, 140, 0, 146, 93)) (V)

| C+ (bm+) | per lod l = 0/1/2 | stride per slot |
|---|---|---|
| +0x0 (+0x10) | slot count 900 / 220 / 140 | |
| +0x18 (+0x28) | `SlotInfo[]` {+0 patch, +4 u32 use bits, +8 s32 normals index (-1)} | 0xc |
| +0x30 / +0x3c | free list / free count | |
| +0x48 (+0x58) | **positions** V3-32 (n² × 12 B) | 0xc0 / 0x1b0 / 0x300 |
| +0x54 (+0x64) | **UV1** V2-32 (n² × 8 B) | 0x80 / 0x120 / 0x200 |
| +0x60 (+0x70) | **UV2** V2-32 | 0x80 / 0x120 / 0x200 |
| +0x1b4 (+0x1c4) | **normals** V3-32 (unit, xyz), counts 0 / 146 / 93 | 0xc0 / 0x1b0 / 0x300 |
| +0x1cc / +0x1d8 | normals free list / count | |
| bm+0x440/444/448 = bm+0x44c/450/454 | **slot packets** | 0x100 (0xc0 used) |

### 1.2 BezierPatchCache_TessellatePatch 0x3747a0 (a0 C, a1 patch, a2 lod, a3 view bit = gm+0x274, t0 wantNormals) (V)

Pops a slot (−1 if the free count is 0, and then still tessellates into
index −1, i.e. before the arrays: a game bug, unreached while the queue
caps ≤ slot counts hold, I), runs TessPositions / TessUVs / TessUV2 into
that slot, sets `SlotInfo{patch, bits |= 0xc | 1<<view}`, `patch+0x1a6+2l
= slot`. If `t0`: pops a normals index (no empty check) and runs
TessPositionsAlt (unit normals = normalised cross product of the two
partial derivatives, per grid point) into it; `SlotInfo+8 = index`.
v18 passes `t0 = (flags & 0x600000) != 0` for lod 1/2 and **0 for lod 0**
(the lod-0 normals array has 0 entries, so a lod-0 shine pass would REF
garbage; presumably never happens, I).

UV1 = bilinear from the corner UVs (patch+0x20..0x38), UV2 = the rect at
patch+0x10/0x18 (phase3-render §1.3). All three grids are pure functions
of (patch data, lod) (I: patches never animate).

### 1.3 Change detection (V)

v18, per queued entry: `if slot < 0 { slot = Tessellate(…); entry.slot =
slot; if slot >= 0 { PS2BezierMan_BuildPatchPacket(bm, lod, slot) } }`.
So **every** write to a slot's positions/UV/UV2/normals is followed, in
the same v18, by `BuildPatchPacket(a0 bm, a1 lod, a2 slot)` (a2 =
`sra a2,v1,0x10` of the returned slot at 0x38beec, V). Nothing else writes
the arrays (BezierPatchCache_Reset only clears SlotInfo and free lists).

Release (ReleaseUnused 0x374b38, only when gm+0x274 == 0): MarkUsed
(0x374a88 / MarkUsed2 0x374ae0) sets bits 0xc each frame a queued patch has
a slot; ReleaseUnused clears 4, next frame 8, the frame after frees the
slot (patch+0x1a6+2l = −1, normals index returned). A freed slot is
reused by another patch only through Tessellate → BuildPatchPacket.

=> **Generation**: a global counter bumped in a BuildPatchPacket wrapper,
stored per (lod, slot). Use a global monotonic value (not per-slot
counters) so re-Init (new level, new arrays) cannot alias.

### 1.4 PS2BezierMan_BuildPatchPacket 0x38c788 (a0 bm, a1 lod, a2 slot) (V)

Writes 12 qw at `P = *(bm+0x44c+4l) + slot·0x100`. With `N = n²`,
`pos/uv/uv2` = the slot's arrays (1.1):

| qw | DMA tag (lo 64) | VIF words |
|---|---|---|
| 0 | CNT qwc 3 (`0x10000003`) | NOP, NOP |
| 1-3 | data | STMOD 0; STROW {0, 0, 1.0, 1.0}; STMASK 0x40404040; NOP; NOP; UNPACK V3-32 masked FLG N at 0x00 (`0x78008000 \| N<<16`) |
| 4 | REF qwc N·3/4 → pos | NOP, NOP (data = N V3 vectors) |
| 5 | CNT 1 | NOP, NOP |
| 6 | data | STMASK 0x50505050; NOP; UNPACK V2-32 masked FLG N at 0x40 (`0x74008040`) |
| 7 | REF qwc N/2 → uv | NOP, NOP |
| 8 | CNT 1 | NOP, NOP |
| 9 | data | STMASK 0x50505050; NOP; UNPACK V2-32 masked FLG N at 0x80 (`0x74008080`) |
| 10 | REF qwc N/2 → uv2 | NOP, NOP |
| 11 | RET qwc 0 (`0x60000000`) | NOP, NOP |

Effect, relative to the TOP of the next MSCAL: `TOP+i = (x, y, z, 1.0)`,
`TOP+0x40+i = (u, v, 1.0, 1.0)`, `TOP+0x80+i = (u2, v2, 1.0, 1.0)`, i < N
(mask 0x40 = w from row; 0x50 = z, w from row; STCYCL is 4,4 from the
microprogram setup so all four mask bytes are equal). The packet itself
is constant per (lod, slot); only the REF'd data changes. Data is read by
the DMA at kick time, but nothing rewrites it before the next v18 (1.3), so
reading it at BuildPatchPacket time or at frame publish is the same.

## 2. Emitters

Common record write (V, identical in EmitPatchType1/Type2/MatrixPass/
ProjectorPasses/DrawPatchFX): `if dm.count < 0xa28`: `st.w2[0:4] =
dm+0x69cc4 & 0x1f`; record at `dm+0x80+count·0x80` = {st (0x14), +0x14
start, +0x18 end, +0x1c s16 dm+0x69cc0 (matrix index), +0x1e
continuation}; count++. Then `*cur += 0x10` (the link-tag gap). Packets
are written even when the record table is full (then they are never
drawn). `Recorded.end` = record +0x18.

### 2.1 v18 0x38b370 (a0 bm) (V)

- `bm+0x5364 = 0`; st.w0 = (w0 & ~0xc) | 0x30; if gm+0x6b90 == 0:
  PS2GraphicsMan_UpdateViewMatrices.
- **Shine matrix bm+0x360..0x39f** (4 qw): camera matrix (from the
  gm+0x10d8 object's vt+0x11c), sin/cos of the camera position ×
  0x4a4440, times the bias matrix 0x5049c0 (0.5 / 0.5 terms). Per frame.
- **FX matrix bm+0x3e0..0x41f**: 0x4ff1a0 rotated about three axes
  (0x4ff140/150/160) by angles from the camera position × 0x4a4344.
  bm+0x420..0x42f = 128.0 ×4; bm+0x430..0x438 = f32 consts at
  0x4a434c/50/54. Per frame.
- Pushes st; w0 type bits ← 1 (`|0x40`), w1/w2 blend/alpha fields (w1 =
  … | 0x800000, |0x14000, |4, |0x400), w3 &= 0xc0000000, w2[0:4] =
  scissor; `e0 = DeferMan_InternState(dm, st, hash)` (kept for the
  deferred flush); `e4 = dm+0x69cc0`.
- Uniform queues (lod 2, 1, 0) → EmitPatchType1(a0 bm, a1 lod, a2 s16
  slot, a3 &entry, t0 cur). Then w0 type ← 2 (`|0x80`), w3 &= 0xc0000000,
  mixed queue → EmitPatchType2(a0 bm, a1 slot, a2 &entry, a3 &entry+0x10,
  t0 cur).
- Deferred flush (2.5), pop st (st.tex = −1, type 0), RenderPatchFX(a0 bm,
  a1 cur).

Order of the VifBuff packets in a frame: lod-8 type 1 (with passes), lod
6, lod 4, type 2 (with passes), deferred groups, FX.

### 2.2 PS2BezierMan_EmitPatchType1 0x38ca70 (a0 bm, a1 lod, a2 slot, a3 entry, t0 cur) (V)

- `pkt = *(bm+0x44c+4·lod) + slot·0x100`; `tbl = *(0x44b9d0 + 4·lod)`
  (= 0x504840 / 0x5048a0 / 0x504900).
- `shine = (flags & 0x600000) && gp+0x13e8 == 0`.
- `proj = lod == 2 ? TestPatchBounds(bm, copy) : 0` (0x38ca08:
  PS2GraphicsMan_CullAgainstLights(gm, (bboxMin,1), (bboxMax,1)) → bitmask
  of lights touching the bbox); disabled if gp+0x13ec != 0.
- st.w0 bits 2-3 from flags & 0x180000 (0.3).
- `immediate = shine || proj || (flags & 0x180000)`.
- **Not immediate** → append to the deferred list (2.5), nothing written,
  no record, `*cur` unchanged.
- **Immediate**: st.tex = {patch+0x1a0, patch+0x1a2}; at `start = *cur`:

  | qw | tag | VIF |
  |---|---|---|
  | 0 | CALL qwc 0 → pkt (`pkt<<32 \| 0x50000000`) | NOP, NOP |
  | 1 | CNT 1 | NOP, NOP |
  | 2 | data | NOP, NOP, NOP, MSCAL `tbl[2 + entry.flag]` |

  `end = start+0x30`; record (continuation 0); `*cur = end+0x10`. Then
  MatrixPass if shine, then ProjectorPasses if proj.

One program start. VU input at TOP: the slot grid (1.4).

MSCAL tables (static init 0x393048, MSCAL immediates in dwords; byte
address = ×8) (V):

| index | lod 0 (0x504840) | lod 1 (0x5048a0) | lod 2 (0x504900) | type 2 (0x504960) |
|---|---|---|---|---|
| 2 / 3 (draw, flag 0/1) | 0x6b8 / 0x790 | 0x700 / 0x7d8 | 0x748 / 0x820 | 0x9a0 / 0xa38 |
| 6 / 7 (projector) | 0x8d8 / 0x910 | same | same | 0xc48 / 0xc48 |
| 8 / 9 (shine) | 0x948 / 0x980 | same | same | 0xcf0 / 0xcf0 |
| 0,1 / 4,5 / 10,11 | unused (0x610.. / relight) | | | |

### 2.3 PS2BezierMan_EmitPatchType2 0x38ce20 (a0 bm, a1 slot, a2 entry, a3 lods ptr, t0 cur) (V)

Always immediate. `pkt = *(bm+0x454) + slot·0x100` (lod-2 slot, shared
with the patch's uniform-8 draws). `shine` as type 1 (gp+0x13e8);
`proj = TestPatchBounds` (always, gp+0x13ec). st.tex = patch tex pair,
st.w0 bits 2-3 from flags. At `start = *cur`:

| qw | tag | VIF |
|---|---|---|
| 0 | CNT 2 | NOP, NOP |
| 1 | data | UNPACK V4-8 (signed) FLG 1 at 0xd0 (`0x6e0180d0`), lods (u32), NOP, NOP |
| 2 | data | NOP, NOP, NOP, UNPACK V4-32 FLG 16 at 0xc0 (`0x6c1080c0`) |
| 3 | REF qwc 16 → **patch+0x40** (the real patch, not the copy) | NOP, NOP |
| 4 | CALL → pkt | NOP, NOP |
| 5 | CNT 1 | NOP, NOP |
| 6 | data | NOP, NOP, NOP, MSCAL `0x504960[2 + entry.flag]` |

(The `a3 == 0` branch that skips qw 0-3 is never taken from v18.) `end =
start+0x70`, record, `*cur = end+0x10`; then MatrixPass(…, lod 2, tbl
0x504960) and ProjectorPasses(…, 2, 0x504960, …).

One program start. VU input at TOP: the lod-2 slot grid (n = 8),
`TOP+0xc0..0xcf` = coefficients, `TOP+0xd0` = (lods[0], lods[1], lods[2],
lods[3]) as i32. The coefficients are level data, static (I), REF'd at
kick time.

### 2.4 Extra passes (records with continuation = 1: drawn right after the patch, in its chain)

**EmitPatchMatrixPass 0x38d168** (a0 bm, a1 &SlotInfo[slot] = `*(bm+0x28+4l)
+ slot·0xc`, a2 patch copy, a3 lod, t0 tbl, t1 entry, t2 cur) (V): pushes
st; st.tex = {−1, patch+0x1a4 or gm+0x10a8}; w0 |= 0x30; w1[7:11] = 0x100
(or 0x880 if flags & 0x400000). `nrm = *(bm+0x1c4+4l) + SlotInfo.normals ·
stride(l)`. At `start = *cur`:

| qw | tag | VIF |
|---|---|---|
| 0 | CNT 1 | NOP, NOP |
| 1 | data | STMASK 0x40404040, NOP, UNPACK V3-32 masked FLG N at 0x40 (`0x78008040`) |
| 2 | REF qwc N·3/4 → nrm | NOP, NOP |
| 3 | CNT 6 | NOP, NOP |
| 4 | data | STMASK 0, NOP, UNPACK V4-32 FLG 4 at 0x80 (`0x6c048080`) |
| 5-8 | data | bm+0x360..0x39f (shine matrix, by value) |
| 9 | data | NOP, NOP, NOP, MSCAL `tbl[8 + entry.flag]` |

`end = start+0x140`, record (cont 1), `*cur = end+0x10`, pop st. No STROW:
the normals' w comes from the row left by the patch's slot packet (1.0),
which holds because this record is chained directly after it (I).

**EmitPatchProjectorPasses 0x38d448** (a0 bm, a1 lod, a2 tbl, a3 entry,
t0 light mask, t1 cur) (V): for k < gm+0x13ec (light count) with mask bit
k: st.tex2 = `gm+0x1484 + k·0xa0` (s16), w0 |= 0x30; at `start = *cur`:
CNT 6 {STMASK 0, NOP, NOP, UNPACK V4-32 FLG 4 at 0x80} + 4 qw light matrix
`gm+0x1440 + k·0xa0` (by value) + {NOP, NOP, NOP, MSCAL `tbl[6 +
entry.flag]`}; `end = start+0x70`, record (cont 1), `*cur = end+0x10`.
st is not pushed (tex2 stays changed for later patches, which set it
again).

VU semantics (mat1/mat2): the pass's own TOP gets the normals / matrix; the
positions (and for type 2 the coefficients/LODs) come from the **previous
draw call's** TOP (qw510 LAST_TOP, set only by draw entries). UV2 :=
(M·P).xy (projector) or (R·N).xy (shine), then the pass-2 variant of
`flag` (ST rewrite of the previous output, or full ctx2 re-draw).

### 2.5 Deferred list bm+0x5364 (bucket fast path, no records) (V)

Entry (0x10, at bm+0x5368 + i·0x10): `+0 u16 tex1, +2 u16 tex2, +4 lod,
+8 slot packet address, +0xc queue entry ptr` (flag = entry+4, slot =
entry+0xc). Flush in v18 after the type-2 loop, if count > 0:

- Groups = maximal runs of consecutive entries with the same u32 tex word
  (list order, no sorting; equal words further apart make separate groups
  in the same bucket).
- Per group: `bucket = DeferMan_FindBucket(dm, e0, tex, hash, e4)` or a new
  bucket {state e0, tex, matrix e4} pushed on the sort list
  (DeferMan_ComputeSortKey). **The bucket state is `e0` (type 1, blend bits
  0)**, not the current st (whose type field is 2 at that point): these are
  type-1 draws (n2-render-layer §2.3 says "type 2": wrong).
- Each entry writes 3 qw = the immediate type-1 packet of 2.2 (CALL pkt;
  CNT 1; MSCAL `0x44b9d0[lod][2 + flag]`).
- `DeferMan_AppendPacket(bucket, groupStart, groupEnd)` (0x362978: links
  with NEXT), `*cur = groupEnd + 0x10`, next group starts there.

So a group = len·0x30 bytes, then a 0x10 gap. At RenderPatchFX entry,
`*cur = flushStart + Σ(0x30·len_g + 0x10)`. In v18, AppendPacket is only
called by this flush (V: the two `jal 0x362978`; no emitter calls it).

### 2.6 RenderPatchFX 0x38d968 (a0 bm, a1 cur) / DrawPatchFX 0x38d690 (a0 bm, a1 patch copy, a2 entry, a3 cur) (V)

RenderPatchFX: returns if gp+0x13f0 != 0. Pushes st: type 4 (`|0x100`),
tex1 = `gm+0xf50 + 4·gm[+0x64]` (s16), w1/w2 particle blend fields. Calls
DrawPatchFX for every uniform lod-8 entry and every mixed entry with
`+8 > 0`. Pops st.

DrawPatchFX, if flags & 0x800000: `count = cvt.w((|bboxMin − bboxMax|²) ·
(gp+0x1258 · gp−0x26b4) · gm+0xc4)`, halved if entry+8 < 2 (never from
RenderPatchFX); if count > 0: bm+0x43c = count; built in the scratchpad
(SprStream gp+0x1384, BeginBlock(start = *cur)) and DMA'd to VifBuff by
EndBlockAligned (returns start + size):

| qw | tag | VIF |
|---|---|---|
| 0 | CNT 0x18 | NOP, NOP |
| 1 | data | NOP, NOP, NOP, UNPACK V4-32 FLG 22 at 0 (`0x6c168000`) |
| 2-17 | data | patch+0x40..0x13f (coefficients) |
| 18-21 | data | bm+0x3e0..0x41f (FX matrix) |
| 22 | data | bm+0x420..0x42f (128.0 ×4) |
| 23 | data | bm+0x430..0x43f {f32 0x4a434c, 0x4a4350, 0x4a4354, i32 count} |
| 24 | data | NOP, MSCAL 0x28c (byte 0x1460), NOP, FLUSH |

`end = start+0x190`, record (cont 0), `*cur = end+0x10`. One start of the
type-4 program f55ee634 at 0x1460, input = TOP+0..0x15 (22 qw).

## 3. What a capture differs from the packet by

- Grid data is REF'd (read at kick). Same as at call time (1.3).
- Type-2 coefficients REF'd from the patch (static, I).
- FX packet goes through the scratchpad; capture it from the arguments
  (all by value), not from VifBuff.
- `slot < 0` (cache exhausted): the game CALLs `base − 0x100` (garbage). The
  capture should record nothing (or a skip) for those (I: not seen).
- Records dropped at dm.count ≥ 0xa28: packet exists, nothing draws it.
- Extra passes depend on VU state left by the previous call (LAST_TOP and
  the output buffers for the "ST rewrite" variant). They are continuation
  records, so they directly follow their patch in the chain (V for the
  flag; the chaining is DeferMan's, structs/render.md).

Per frame vs cached:

| data | lifetime |
|---|---|
| positions / UV1 / UV2 / normals per (lod, slot), slot packets | cached; change only with BuildPatchPacket |
| patch coefficients (type 2 REF, FX) | static level data (I) |
| shine matrix bm+0x360, FX matrix bm+0x3e0, light matrices gm+0x1440+k·0xa0 | per frame |
| CALL/CNT/MSCAL wrappers, lods, deferred list, FX block | per frame |

## 4. Capture points

| wrap | records |
|---|---|
| BuildPatchPacket 0x38c788 (after the body) | `gen += 1`; `grid[lod][slot] = gen`; a `BezierGrid` fill (positions/UV/UV2 and, if SlotInfo.normals ≥ 0, normals) |
| EmitPatchType1 0x38ca70 | `start = *cur` before; if `*cur` changed: patch Recorded (`first_tag = CALL pkt`) and, from the cursor walk, the pass records it made (or wrap 0x38d168/0x38d448 directly) |
| EmitPatchType2 0x38ce20 | patch Recorded (`first_tag = 0x10000002`) |
| EmitPatchMatrixPass 0x38d168 | pass (`first_tag = 0x10000001`) |
| EmitPatchProjectorPasses 0x38d448 | one pass per set mask bit (`first_tag = 0x10000006`); starts step by 0x80 |
| RenderPatchFX 0x38d968 (at entry) | deferred groups from bm+0x5364 (2.5); flush start computed back from `*cur` |
| DrawPatchFX 0x38d690 | PatchFx (`first_tag = 0x10000018`) |

All need entries in overrides.txt (capture.rs pattern).

## 5. Proposed commands

```rust
/// A cached patch grid: BuildPatchPacket's (lod, slot) and the global
/// generation it bumped. n = 2·lod + 4.
pub struct GridRef { pub lod: u8, pub slot: u16, pub gen: u32 }

pub enum Cmd {
    …
    /// Cache fill (not a bucket packet; `start` = the slot packet
    /// address): the slot's grids as BuildPatchPacket left them.
    BezierGrid { grid: GridRef, pos: Vec<[f32; 3]>, uv: Vec<[f32; 2]>,
                 uv2: Vec<[f32; 2]>, normals: Option<Vec<[f32; 3]>> },
    /// One patch record, or one deferred group: one program start per call.
    Bezier { calls: Vec<BezierCall> },
    /// A shine / projector pass over the draw just before it.
    BezierPass { entry: u32, of: BezierCall, matrix: [u128; 4],
                 normals: Option<GridRef> },
    /// Glitter on a patch: the 22 input qwords of type 4 at 0x1460.
    PatchFx { input: [u128; 22] },
}

pub struct BezierCall {
    pub entry: u32,                    // MSCAL byte address (2.2 table)
    pub grid: GridRef,
    /// Type 2 only: edge LODs (x, y, z, w) and the 16 coefficients.
    pub mixed: Option<([u8; 4], [u128; 16])>,
}
```

What the renderer needs to plan them: the header (qw0-5 of the current
MatrixRecord) and the static VU data (type 1 qw6-16 via REF 0x44b080;
type 2 qw6-10 + the 42-qw basis block at 0x200) as for `Cmd::Strip`; the
TOPs from `VifRegs` (`calls.len()` starts; a pass is one start); a
`(lod, slot) → (gen, GPU range)` table filled by `BezierGrid`. The first
step can rebuild the VU input at TOP from the table (1.4 / 2.3 layouts)
and call `mat1::plan_gpu` / `mat2::plan_gpu` unchanged; later the grids
stay on the GPU with static index buffers per n (type 1) and per
(x, y, z, w) LOD combination (type 2, ≤ 81), and a pass is a second draw
of the same vertices with UV2 from the matrix (`of` names them).
Race frames use type-1 entries 0x6b8/0x700/0x790/0x7d8 (lod 0/1, no
passes), type-2 0x9a0/0xa38 and projector 0xc48.
