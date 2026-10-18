# N2 capture spec: riders (skinned models, VU1 types 6/7)

Program `9fa478425b3700e5` (`crates/ssx3-render/src/mat6.rs`). MSCAL addresses
below are byte addresses (`imm × 8`), as `mat6` names them: 0x00 / 0x398 lit
draw (bone per vertex), 0x50 lit draw (bone per batch), 0x10 morph, 0x20 shadow,
0x30 env pass, 0x40 kick ctx2 tag, 0x80 bone setup (slot 0x60's target),
0x70 empty, 0x1150 lights × 255.

Legend: **V** = checked in the disassembly / generated code, **D** = checked on
race frame dumps (frames 4200, 4500, 7500: every type-6 group packet and every
slot-100 packet parsed against this spec without a mismatch), **I** = inferred.

gp = 0x4a30f0 (gm pointer at gp+0x2a90 = 0x4a5b80).

## 0. Who draws riders (V)

```
Rider_RenderModel 0x122448
  └ RiderModel_Render 0x310640 (rider model, lod, flagA, flagB)
      per part (or once for all parts): slot 111 BuildBlendedMatrices → gm+0x13e8 (scratchpad)
      per part: slot 99 DrawSkinnedModel 0x37a610
          └ slot 124 EmitLightingPacket 0x396b40 (once per call)
Rider_DrawShadow 0x122898
  push state; BoaderShadow_ComputeProjection; state bits
  slot 31; slot 53 (render target = shadow texture, 128×128 CT16); slot 55 (clear sprite)
  slot 101 SetShadowHeader 0x37ba50 (once)
  RiderModel_RenderShadow 0x310948
      slot 111 → gm+0x13e8; per part: slot 100 DrawSkinnedModelShadow 0x37b548
  slot 54 (render target back); slot 32; pop state
  BoaderShadow_Submit (projects the texture onto the ground; not in this spec)
```

- Only slot 124 sets type 6 and only slot 101 sets type 7 (V: a scan of the
  generated code for `w0 & ~0x3c0 | 0x180/0x1c0`). Slot 99's only caller is
  `RiderModel_Render` (two call sites, 0x310844 / 0x3108fc), slot 100's only
  `RiderModel_RenderShadow` (V for the gm vtable offsets 0x31c / 0x324;
  callers that keep the vtable in a register can hide from such a scan, I).
- **The scratchpad bones are not from `RiderModel_BuildBoneMatrices`.** That
  function fills `model+0x2c`. The bones slot 99/100 read
  (`gm+0x13e8` = 0x70000000) are written by **slot 111 (0x386bd0,
  `PS2GraphicsMan_v111`)** right before each slot-99/100 call (V). Slot 111
  takes `(gm, src, 0, palettes, count, dst = 0 → gm+0x13e8)` and builds blended
  bones `Σ src[byte idx] × (short w × 0.01)` (I = 0x3c23d70a). `src` is
  0x4fc420 (Render: `model+0x34[i] × model+0x38[i]`) or 0x4fd420 (Shadow:
  `+0x30[i] × +0x38[i]`), 64 bytes per bone.
- **The "None"-material layer-1 draws with 0x20/0x70/0x80 and GIF data are the
  2nd..Nth slot-100 calls of one `Rider_DrawShadow`** (one per rider part).
  Slot 100's epilogue clears the type (`w0 &= ~0x3c0`) and `w3`. Only the first
  part's record carries type 7 (from slot 101). The later parts' records are
  type 0, and they rely on the program and header that the type-7 head left in
  VU1. They stay in order because every record of the sequence except slot
  53's has `+0x1e = 1`: `DeferMan_BuildBuckets` (0x362de8, V) then makes it a
  continuation bucket of the previous record (bucket +0x18 link, +0x14 = 1;
  never sorted on its own). Dump 4500 has 6 riders: records 179 (slot 53, head),
  180 (55), 181 (100, type 7), 182-191 (100, type 0), 192 (54) (D).

## 1. Slot 124 `EmitLightingPacket` 0x396b40 (V, generated code)

- Arguments: a0 = gm. **f12 is not read** (slot 99 sets f12 = 1.0, but that
  goes nowhere).
- State: `w0 = w0 & ~0x3c0 | 0x180` (type 6); `w3 = w3 & 0xc0000000 | cursor & 0x3fffffff`.
- Packet at `cursor` (uncached `|0x30000000`). It has no record. DeferMan CALLs it
  (`w3`) as the type-6 material parameter, so it is a separate packet from the draw:

| qw | contents |
|---|---|
| 0 | DMA `RET qwc 12` (0x6000000c), VIF NOP NOP |
| 1 | NOP, FLUSHE, STCYCL 1,1, `UNPACK V4-32 num 10 addr 7` (0x6c0a0007, absolute) |
| 2-11 | 10 qw copied from `gm+0x6bb0..` (SH coefficients) → VU qw 7-16 |
| 12 | NOP, NOP, NOP, `MSCAL 0x1150` (×255 in place) |

- `cursor += 0xd0` (13 qw; **not** 0xb0 as `n2-render-layer.md` §2.2 says). D:
  every slot-99 record has `start = w3 + 0xd0`.
- Program starts: one, MSCAL 0x1150 (input VU 7-16 absolute).

## 2. Slot 99 `DrawSkinnedModel` 0x37a610 (V)

### 2.1 Arguments, globals
- a0 = gm, a1 = model header, a2 = base (file base for the `+0x20` group
  offset), a3 = `whole` flag (RiderModel_Render's flagA for parts of type 2,
  else flagB). t0 = 0 (unused). The caller loads f12 = model+0x144, and it is
  overwritten.
- `gp+0x1420` (0x4a4510) ≠ 0 → returns at once (no push, no packet).
- `gp+0x1440` (0x4a4530): ctx2 blend select (below).
- `gp-0x276c`, `gp-0x2768`, `gp-0x2764` (0x4a0984/88/8c): morph thresholds, all
  0.01 (D).
- gm+0xe84 (state stack), +0x10d8 (vtable), +0x10b0[] (env texture ids),
  +0x13e8 (bone base), +0x58a0..0x58d0 + 0x5920 (sort projection), +0x6b90
  (calls `UpdateViewMatrices` when 0: up to three times), +0x5a00 (cursor),
  +0x18f0 (DeferMan).

### 2.2 Model layout (after slot 109 `PrepareSkinnedModel` 0x3866f0 relocated it; V)
- `+0x20` group array offset (from `base`), 0x14 per group:
  `+0` flags (1 = rigid chain, 0x100 = morphable, 0x8000 = translucent; set by
  109 from the texture name), `+4` texture id, `+8` short env index, `+0xa`
  byte alpha (name float × 128), `+0xc` batch count, `+0x10` → batches
  `{u32 elems*, u32 elemCount}`, each element `{u32 chain, u32 morph}`
  (both absolute after 109).
- `+0x3c` → palettes `{int n, int *boneIdx}`, indexed by the **cumulative**
  batch index over all groups (the running sum of `group+0xc`).
- `+0x40` float *weights (one per morph target), `+0x48` short group count,
  `+0x50` short morph target count, `+0x46` short palette count.

### 2.3 State (the record carries it; the capture does not need it)
Push (copy 0x14 bytes up, gm+0xe84 += 0x14). Slot 124. `tex1` (+0x12) =
`gm+0x10b0[group0.env]` (from the **first group only**). `w0 |= 0x30`. a3 ≠ 0:
w0 alpha2 = 0, blend1 7, blend2 0x11 (or 3 when gp+0x1440 ≠ 0), sort mode 2
with `w2[10:28] = sort`, w1 bit 23 = 1, bits 20-21 = 0, bits 12-19 = 0x14.
Per group: `SetTexFunction(gm, tex, 3)` (changes TexInfo TEX0); `tex0` (+0x10) =
tex. If a3 = 0: opaque (no 0x8000) gives blend1 1, blend2 0x11/3, sort 0;
translucent gives blend1 5, `w0[20:27]` = alpha byte, blend2 0xc, sort mode 2
+ sort, bits 20-21 = 3, bits 12-19 = 0x5c. The sort value is
`clamp(ftoi0((gm+0x58a0 · spad bone0.col3).w × gm+0x5920.w), 0, gm+0x5920.x) + 0x10`,
or 0x400 when that is above 0x400. Epilogue: pop; then on the **restored**
state, `w0 &= ~0x3c0` and `w3 &= 0xc0000000`.

### 2.4 Packets: one per group
For each group, with `start = gm+0x5a00` before it (the first group starts
right after the 0xd0 lighting packet):
- flags & 1 → **rigid** path; else flags & 0x100 → **morphable** path; else
  nothing (no packet, no record, no pad).
- After the group: `gm+0x5a00 = end` (the cursor after the last qword, `& 0x0fffffff`).
  When `DeferMan.count < 0xa28`, one **record** is written with state words,
  `+0x08[0:4]` = scissor (DeferMan+0x69cc4), `+0x14 start`, `+0x18 end`,
  `+0x1c` = DeferMan+0x69cc0, `+0x1e = 0`. Then `gm+0x5a00 += 0x10`: a pad
  qword that `DeferMan_AppendPacket` later fills with a `NEXT` to the bucket's
  next packet (V). The next group starts after the pad.
- First DMA tag: `CNT qwc 4n+2` = `0x1000_0000 | (4n+2)`, where n is the first
  batch's bone count (VIF words 0).

**Bone batch** (every batch, every path):

| qw | contents |
|---|---|
| 0 | `CNT qwc 4n+2`, NOP NOP |
| 1 | NOP, FLUSHE, STCYCL 1,1, `UNPACK V4-32 num 4n addr 0x11` (absolute) |
| 2.. | n × 4 qw: bone `palette[b].boneIdx[i]`, copied from `*(gm+0x13e8) + idx×0x40` **at the call** (lqc2/sqc2) |
| last | `UNPACK S-32 num 1 addr 0x3ff` + n (all four lanes = n), `MSCAL 0x80`, `MSCAL 0x70` |

n ≤ 25 (D: max 25). The bone setup also reads header qw0-3 (M) and 0x393-0x395
(E) from the type-6 material setup (DeferMan_EmitMaterialSetup uploads
`UNPACK V4-32 4 at 0x392`: ctx2 tag `{0x312e4000_00008000, 0x412}` + E, V).

**Rigid path** (flags & 1), per batch: bone batch, then one qword:
`CALL qwc 0 addr = elems[0].chain` (0x50000000 | addr<<32). **Only element 0;
elemCount and morph are ignored.** The chain contains its own program starts
(§2.5).

**Morphable path, no weight above the threshold** (no `weights[j] > 0.01`
for any j < `+0x50`; weights[0] is compared with gp-0x276c, the rest with
gp-0x2768). Per batch: bone batch, then
- `CALL qwc0 elems[0].chain`;
- for k = 1..count-1: `CALL qwc1 elems[k].chain` + data qw `{NOP, FLUSH, NOP, MSCAL 0x398}`.
  The MSCAL runs before the jump: it draws element k-1;
- `CNT qwc1` + `{NOP, FLUSH, NOP, MSCAL 0x398}` (draws the last element).

**Morphable path with morphs** (some weight > 0.01). Per batch: bone batch,
then elements in pairs (e0, e1). When the count is odd, the last pair's e1 is
**the same element as e0**, so it is uploaded, morphed and drawn twice (D: 7
elements gave 4 pairs). Per pair:

| qw | contents |
|---|---|
| 0 | `CALL qwc0 e0.chain` |
| 1, 2 | `CALL qwc1 e0.morph` + `{NOP, UNPACK V2-32 num 1 addr 0xb4\|FLG (0x640180b4), w0, 12800.0}` |
| 3 | `CALL qwc0 e1.chain` |
| 4, 5 | `CALL qwc1 e1.morph` + `{FLUSH, UNPACK V2-32 0xb4\|FLG, w0, 12800.0}` |
| per j ≥ 1 with `weights[j] > 0.01` (gp-0x2764) | 4 qw: `CALL qwc1 e0.morph_j` + `{NOP, UNPACK…, wj, 12800.0}`, `CALL qwc1 e1.morph_j` + `{NOP, UNPACK…, wj, 12800.0}` |
| end | `CNT qwc1` + `{FLUSH, MSCAL 0x398, FLUSH, MSCAL 0x398}` |

- `w0` = raw bits of weights[0], or 0 when weights[0] < 0.0 (it has no
  threshold). `wj` = raw bits (V: `lw`, not cvt). This sets VU TOPS+0xb4 = {weight, 12800.0}.
- `morph_j`: block j of the element's morph list. Block 0 = `elem.morph`;
  block j+1 = block j + `(u16 qwc of block j's DMA tag + 1) × 16`. The walk
  runs over every j < `+0x50` whether the target is emitted or not (V, D: face
  models have about 10 targets).

### 2.5 Prebuilt chains in the model file (D; static data)
All `CNT`/`RET` tags with the data inline. Vertex data lives in the chain
itself.

**Lit sub-packet** (repeated; a rigid chain is a sequence of them ended by
`RET qwc1 {NOP, STCYCL 1,1, NOP, NOP}`. An element chain of the morphable
path is exactly one sub-packet without the MSCAL qword, then that RET):
1. `CNT qwc h+1`: `{NOP, STCYCL 1,1, NOP, UNPACK V4-32 num h addr 0|FLG}` +
   h qw → TOPS+0..h-1:
   - qw0 = GIF tag `{NLOOP total | EOP, 0x302e4000 (PRE, PRIM 0x5c, NREG 3), 0x412, 0}`;
   - qw1 = `{nSub, boneAddr, nrmAddr, total}` (absolute VU addresses: 0x11+4i, 0x75+3i);
   - qw2.. = nSub strip counts (.x).
   - So h = 2 + nSub.
2. `CNT qwc k`: VIF stream (walk the codes; the order is fixed):
   `STROW {0,0,0x1000,0x1000}; STMASK 0x50505050; STCYCL 3,1;
   UNPACK V4-16 (signed, no MASK bit, so that row/mask is dead) num n addr h|FLG`
   = per vertex {u, v (4.12), boneAddr, nrmAddr} at TOPS+h+3i;
   `STROW {0,0,0,0x8000}; STMASK 0x40404040; STCYCL 3,1; UNPACK V3-16 MASK num n addr h+1|FLG`
   = normal (1.15), w = 0x8000;
   `STROW {0,0,0,1.0}; STMASK 0x40404040; STCYCL 3,1; UNPACK V3-32 MASK num n addr h+2|FLG`
   = position, w = 1.0.
   n = total ≤ 59 (D).
3. `CNT qwc1 {STCYCL 1,1, MSCAL d, …}` with d = 0x00 (bone per vertex) or 0x50
   (one bone per batch, header .y/.z), optionally followed in the same qword by
   env MSCALs: `0x30, 0x30` or `0x40, 0x30`.

**Env passes (D, 197 rigid chains):** VIF DBF flips on every MSCAL. A `0x30` at
parity p re-kicks the last draw made at parity p (mat6 `env_pass` reads the
output at TOP+0xb4). `0x40` is a parity filler: it kicks the ctx2 tag at 0x392
(NLOOP 0, so it writes only PRIM, I). In every chain seen, a chain either
env-passes **every** draw exactly once, or none. The capture can reduce this to
`env: bool` per chain, or per draw by simulating the parity from the chain start.

**Morph block** (V on dump bytes):
- `RET qwc Q`;
- `{NOP, STCYCL 1,1, NOP, UNPACK V4-8 (signed) num m+1 addr 0xb5|FLG}`;
- m+1 words: word0 = m (→ TOPS+0xb5.x), then m entry words, bytes
  `[dx, dy, dz, target]` (s8; target & 0xff = qword offset from the first
  vertex's position, a multiple of 3);
- pad;
- last qw `{NOP, NOP, NOP, MSCAL 0x10}`.

An empty target is `RET qwc1 {NOP, NOP, NOP, MSCAL 0x40}` (a parity filler).
Effect (`mat6::morph`): `pos.xyz += itof15(d) × (weight × 12800)`, in entry
order, applied after the earlier targets (keep the replace-not-add quirk for
consecutive equal targets).

**Shadow sub-packet** (slot-100 chains):
1. `CNT qwc4 {NOP, STCYCL 1,1, NOP, UNPACK V4-32 3 addr 0|FLG}` + h qw:
   - GIF tag `{NLOOP | EOP, 0x10224000 (PRE, PRIM 0x44, NREG 1), REGS 4}`;
   - `{nSub, …}`;
   - counts.
2. `CNT {NOP, STROW {0,0,0,1.0}, STMASK 0x40404040, STCYCL 1,1, UNPACK V4-32 (no MASK) num n addr h|FLG}`
   = {x, y, z (f32), boneAddr}.
3. `CNT qwc1 {NOP, STCYCL 1,1, NOP, MSCAL 0x20}`.
4. Then the next sub-packet, or `RET qwc1 {STCYCL 1,1}`.

### 2.6 Program starts of one group, in order
Per batch: `0x80` (bone setup; input VU 0x11..0x11+4n-1, 0x3ff = n, plus the
header M/E), `0x70` (empty). Then:
- rigid: the chain's starts (draw 0x00/0x50 with input at TOPS from the
  sub-packet, env 0x30/0x40);
- plain: `0x398` × elemCount (input = that element's sub-packet at TOPS);
- morph: per pair `0x10` (A), `0x10` (B), then `0x10`/`0x40` × 2 per extra
  target, then `0x398` (A = e0), `0x398` (B = e1).

## 3. Slot 101 `SetShadowHeader` 0x37ba50 (V, generated code)
- a0 = gm, a1 → 4 qw matrix, a2 → qw scale, a3 → qw offset. Rider_DrawShadow
  passes the BoaderShadow object's +0x110..0x140, +0x150, +0x160: an
  orthographic light matrix, (64, 64, …), (2048, 2048, …) (D).
- Packet at the cursor (no record):

| qw | contents |
|---|---|
| 0 | `RET qwc 7` (0x60000007) |
| 1 | NOP, FLUSHE, NOP, `UNPACK V4-32 num 6 addr 0` (0x6c060000, absolute, **no STCYCL**) |
| 2-5 | matrix → VU 0-3 |
| 6 | scale → VU 4 |
| 7 | offset → VU 5 |

- `cursor += 0x80`; `w0 = w0 & ~0x3c0 | 0x1c0` (type 7); `w3 = w3 & 0xc0000000 | cursor_before & 0x3fffffff`.
  It is a type-7 material parameter (CALLed by DeferMan before the type-7 head).
- All values are copied at the call.

## 4. Slot 100 `DrawSkinnedModelShadow` 0x37b548 (V)
- a0 = gm, a1 = shadow model, a2 = base, a3 → float[4] colour {a, r, g, b}.
  No gp+0x1420 check. No push/pop.
- State: `w0 |= 0x30`, blend1 1, blend2 0xb, `w0[20:27] = 0x64`, w1 bits 23-24
  = 2, bits 20-21 = 3, bits 12-19 = 0x14, tex0 = tex1 = 0xffff. Epilogue: type
  and w3 cleared (on the current state).
- **One packet and one record for the whole call** (`start` = cursor at entry,
  `+0x1e = 1`), then pad +0x10. Only groups with flags & 1 emit. Per such group:

| qw | contents |
|---|---|
| 0 | `CNT qwc3`, NOP NOP |
| 1 | NOP, NOP, FLUSH, `DIRECT 2` |
| 2 | GIF tag `NLOOP 1, EOP, PACKED, NREG 1` (0x8001 \| 1<<60), REGS 0xe (A+D) |
| 3 | RGBAQ: `R=(int)(c[1]·128) \| G<<8 \| B<<16 \| A=(int)(c[0]·128)<<24 \| Q=1.0<<32`, addr 1 (no clamp; cvt.w.s truncation) |

  then per batch the **bone batch** (as §2.4) + `CALL qwc0 *elems` (element
  0's word 0 = the shadow chain, §2.5).
- First DMA tag `0x1000_0003`. When no group has flag 1, the record has
  start = end (an empty packet).
- D: colours 0x80262626 and 0x80000000; one group and one batch per part.
- Program starts: per batch `0x80`, `0x70`, then the chain's `0x20` (× sub-packets).
  The input header VU 0-5 comes from slot 101 (the type-7 head's material
  parameter), and later parts reuse it (§0).

## 5. Capture vs packet: deferred reads
- **Bones:** read from the scratchpad and copied into the packet during the call
  (V). The scratchpad is valid on return (slot 99/100/124 do not write it), but
  the next slot-111 call (next part or rider) overwrites it. Capture them at
  the call, or read them back from the packet. Nothing is deferred.
- **Lights** (gm+0x6bb0), **weights**, **shadow colour**, **slot-101 header**:
  copied at the call.
- **Deferred, read by the DMA at the EndFrame kick:** the prebuilt mesh chains,
  shadow chains and morph blocks in the model file. They are static, but a
  model unload/reload between the call and EndFrame (rider swap, FE) would
  change what the DMA sees. Key a parsed-chain cache by (guest address,
  generation), and bump the generation in slot 109 (0x3866f0, model
  relocation), which runs on every model load (I).
- VU state outside the packet: header qw0-5 (MatrixRecord world_view / guard;
  for type 7 the slot-101 packet), 0x392 ctx2 tag and 0x393-0x395 E (type-6
  material setup), lights VU 7-16 (slot-124 packet, via w3).
- The draw order inside a group is fixed by the packet. Type-0 shadow parts
  depend on the bucket continuation order (§0).

## 6. Proposed `Cmd`
```rust
/// Slot 99 (one per group record) / slot 100 (one per call).
Cmd::Skin {
    /// Slot 99: gm+0x6bb0 × 10 at the call (VU 7-16 before ×255). Slot 100: None.
    lights: Option<[u128; 10]>,
    /// Slot 100: the slot-101 header (VU 0-5) last captured (the capture module keeps it).
    header: Option<[u128; 6]>,
    batches: Vec<SkinBatch>,
}
pub struct SkinBatch {
    /// Slot 100: the group's RGBAQ (first batch of each group).
    rgbaq: Option<u64>,
    /// VU 0x11+4i, 4 raw qwords (columns) per bone, n = 1..=25.
    bones: Vec<[u128; 4]>,
    draws: SkinDraws,
}
pub enum SkinDraws {
    /// Rigid (slot 99 flag 1, slot 100): a prebuilt chain by guest address; its own starts.
    Chain(u32),
    /// Morphable, no morph: element chains, each drawn by 0x398, in order.
    Plain(Vec<u32>),
    /// Morphable with morphs: elements in draw order (pairs; odd count repeats the last),
    /// each with its morph passes (block address, weight bits) in order.
    Morph(Vec<(u32, Vec<(u32, u32)>)>),
}
```
- Wrapper (slot 99): record `n0 = DeferMan.count` and the cursor; run the body
  (`c.pc == ra` check as in `capture.rs`). Build the batches from the model +
  palettes + `*(gm+0x13e8)` (same order as §2.4). Pair them with the records
  `n0..count` in group order (cross-check `record.start` against the running
  cursor: lighting 0xd0, then per group its size + 0x10). No record (DeferMan
  full) means the group is skipped (it is never drawn).
  `first_tag = 0x1000_0000 | (4·n₀+2)`.
- The renderer plans per batch: bone setup natively (M·B, raw R, E·R with the
  header M and E). Per draw: parse the chain sub-packet from guest RAM (cached,
  vertex pulling of 3 qw/vertex), apply the morph blocks on the CPU copy, and
  make a GPU draw like `mat6::plan_draw` with the lights ×255. An env-pass
  chain gets a second (ctx2, sphere-map ST) draw per draw. Shadow: as
  `mat6::plan_shadow`, colour = rgbaq, into the 128×128 CT16 target that slots
  53/55/54 frame.
- A size sketch per 6-rider race frame (D, 4500): about 60 type-6 group records
  (86 rigid batches, 5 plain, 3 morph), 6 heads + 54 continuation shadow records.
