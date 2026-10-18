# N2 capture spec: the per-draw prelude and the post passes

Area `prelude` of [BRIEF.md](BRIEF.md). What `PS2GraphicsMan_RenderFrame`
(0x363490) writes into the VIF1 chain (and the GIF/PATH3 chain) **between**
the draws' packets, so a renderer can reproduce VU1 and GS state natively
from DeferMan data, without walking the chain.

Marks: **V** = read in the disassembly / generated Rust for this spec
(llvm-objdump `--triple=mips64el` on `orig/SLUS_207.72`, EE-only opcodes
`lq/sq/lqc2/sqc2/VU0 macro` read in `generated/ee/s12`), **D** = from the
Ghidra decompile only, **I** = inferred. gp = 0x4a30f0.

## 0. Answers first

1. **Which header a draw gets.** VU1 qw0-5 are written only by
   `DeferMan_EmitMaterialSetup` (0x364cd0) from the MatrixRecord whose index
   is DeferMan+0x69cc0 at that moment (= the bucket matrix passed to
   `EmitStateChange`), **verbatim for types 1, 2, 3, 6** (record +0x80..+0xdf,
   no lane is changed), with z/16 for types 4 and 5, from the w3 packet for
   type 7 (table §2). The setup never zeroes a z lane. (V)
2. **When it is (re)uploaded.** Only inside `DeferMan_EmitStateChange`,
   which RenderFrame calls only when the bucket's *interned state pointer*
   differs from the previous bucket's (the "previous" is reset to 0 after
   each default-state block). Then: type differs from the last emitted
   state's type (DeferMan+0x69ca8, type 0 included) → mode 0 (program +
   full header); same type and bucket matrix ≠ +0x69cc0 → mode 1 (header);
   then, if w3 ≠ +0x69cbc → mode 2 (types 6/7 only: `CALL w3`). Same state
   pointer, or same type + same matrix → **no upload**: the VU keeps whatever
   qw0-5 hold. `frame.rs` `Emitter` / `Draw::header_matrix` model this
   correctly. (V)
3. **The guard z = 0 seen on type-3 ribbons is written by a draw packet,
   not by the prelude.** Slot 89 `DrawShadowVolume` (0x386e78, caller
   `cBoardTrailFX_RenderBatch`) puts *absolute* `UNPACK V4-32 2 at 4` (no FLG)
   qwords into its own packet: live gm+0x5840/0x5850 before mesh pass 1,
   **the same with the z lanes (+0x8 of each qw) = 0 before mesh pass 3**,
   and live values again before its 5 edge passes. The ribbons of pass 3 are
   `EmitShadowStrips` (0x387ec0), MSCAL 0x3a00/0x3a08 like the board wake
   (`EmitBoardWakeStrips` 0x3885e0, slot 90), and both are `Cmd::Ribbon`, so
   a "board-wake" ribbon with z = 0 is almost certainly a slot-89 pass-3
   shadow strip (I). Slot 90 itself writes only TOP-relative UNPACKs (V).
   Of the other producers checked (slots 71, 75, 82-86, 95/EmitModelHeader,
   99/100, Bezier type 1/2/matrix/projector/PatchFX), none writes VU1 qw0-6
   absolutely (V: their UNPACKs carry FLG or target 0x11+/0xb4/0x3ff).
   **Capture rule:** a type-3 draw's qw4/qw5 = the MatrixRecord's, except
   inside slot 89's packet: pass 1/4 = gm+0x5840/0x5850 at the slot-89
   call, pass 3 = the same with z = 0. Capture those two qwords in the
   slot-89 wrapper.
4. **Records and live guard can differ.** `DeferMan_InternMatrixRecord`
   (0x3645b8) compares only record +0xe0..+0x17f (gm+0x5860..0x58ff) when
   looking for an existing record; +0x00..+0xdf (world, guard-projection,
   worldView, **guardScale/Offset**) of a reused record are the first
   interner's (V). Slot 89 reads gm+0x5840 live, the setup reads the record.
   Normally equal (the same viewport writes both halves) (I).

## 1. RenderFrame chain layout (0x363490, D + V)

`RenderFrame(dm = a0, &vifCursor = a1, &vifHead2 = a2, &gifCursor = a3,
&gifHead2 = t0)` (D; called only from EndFrame). The VIF chain is built at
`*vifCursor | 0x30000000` (VifBuff, uncached). Before emitting:
`BuildBuckets`, `SortBuckets`, `TexMan_ResetUsageTable`, SprStream mode 5,
`TexMan_InvalidateBound` (tm+0x1f4c/+0x1f50/+0x1f54 = −1),
`VramHeap_NodesToScratchpad`, VRAM LRU table 0x53ab80 (per tex id {last
bucket ordinal, count}). dm+0x69ca8 = default state (0x501420), dm+0x69cc4 = −1.

```
VIF phase 1:  DEFAULT
              per bucket b in ranges [0,split0) then [split0,split1):
                  [STATECHANGE if b.state != prev]   prev := b.state
                  BIND(b.tex)                        (only if a tex id >= 0)
                  CALL b.start  (if b+0xc != 0; RET {0x60000000,0} written at b+0x10 | 0x30000000)
                  … then the continuation chain b+0x18, same three steps each
              after range 0 (threshold 6): per view RenderDepthFog; DEFAULT; prev := 0;
                   tm+0x1f4c,+0x1f50,+0x1f54 := −1; dm+0x69cc4 := −1
              after range 1 (threshold 9): END {0x70000000,0}
GIF:          END at gifCursor; gifCursor += 0x10; *gifHead2 = gifCursor; *vifHead2 = vif cursor
VIF phase 2:  per view RenderBloom; per view RenderColorFilter; DEFAULT; resets as above
              range 2 buckets as above
end:          GIF END; VIF END; *vifCursor = end
```
(numSorted = 0: no DEFAULT, no post passes; END/heads only.) The layer
thresholds {6, 9} are at 0x4a4058; the split search is already in
`frame.rs::plan`.

**DEFAULT** = `DeferMan_EmitDefaultState` (0x364360, V): FLUSHE block,
`EmitMaterialSetup(type 0)` = nothing, then one A+D block of 8:
CLAMP_1, ALPHA_1, ZBUF_1, TEST_1, CLAMP_2, ALPHA_2, ZBUF_2, TEST_2 from
0x501420 {0, 0xb00294, 0x80, 0, −1} (context-2 ZBUF/TEST selector quirk as
in `Emitter::default_state`). It leaves +0x69cc0/+0x69cbc untouched, so the
next typed bucket always gets mode 0. Consequence (V): **the first typed
bucket of each of the three groups re-uploads program and constants**, so no
VU1 constant state carries across groups or frames; `Carry::matrix` /
`material_param` cannot change the output (the first setup is always mode 0).

**FLUSHE block** (both emitters, V): `CNT 1 {0,0}` + `qw {0, 0, 0,
0x10000000}` = NOP NOP NOP FLUSHE.

**A+D block** (`GsPkt_Finish(.., 1)` 0x3627a8, V): nothing if n = 0, else
`CNT n+2 {0,0}`, `qw {0, 0, 0, 0x50000000|(n+1)}` (DIRECT n+1), GIF tag
`NLOOP n, EOP, NREG 1, REGS 0xe`, n × {value, reg}.

### 1.1 STATECHANGE (`DeferMan_EmitStateChange` 0x363c20, V)

`(dm, state* = a1, matrix = a2, &cursor = a3)`. In order:
1. FLUSHE block.
2. If type = w0[6:9] ≠ 0: material setup (§2), modes as in §0.2;
   +0x69cbc := w3 & 0x3fffffff, +0x69cc0 := matrix whenever a setup is emitted.
3. One A+D block, registers in this order, each only on change against
   +0x69ca8.. (the last emitted state): SCISSOR_1, SCISSOR_2 (w2[0:4] ≠
   +0x69cc4; table at dm+0x6aed4 {x, y, w, h}); ZBUF_1, ZBUF_2 (w1 bit 22);
   TEST_1, TEST_2 (w1 & 0x1bff000); CLAMP_1 (w0 & 0xc); ALPHA_1 (w1 & 0x7c or
   FIX w0[12:19]); CLAMP_2 (w0 & 0x30); ALPHA_2 (w1 & 0xf80 or FIX
   w0[20:27]). ALPHA presets 15 and ≥ 18 write the reg id but **leave the
   value qword stale** (`blend_preset` → None). Then +0x69ca8..+0x69cb8 :=
   state.

All of this is in `Emitter::state_change` already, except the order of the
writes (not needed natively).

### 1.2 BIND (`TexMan_BindTextures` 0x368660, V for structure, D for VRAM math)

`(tm, ctx1 id = bucket.short@4, ctx2 id = short@6, bucket ordinal, &vifCursor,
&gifCursor)`. Nothing at all if both ids < 0. Else:
1. Per context with id ≥ 0: if TexInfo+0x28 (TBP) < 0, `VramHeap_Alloc`
   (format 9 → temp heap tm+0x4350, TBP << 2) and `TexMan_EmitImageUpload`;
   if TexInfo+0x20 (CLUT ptr) ≠ 0 and +0x2c (CBP) < 0: allocate and
   `TexMan_EmitClutUpload`. Uploads go to the GIF chain via SprStream (per
   mip: `CNT 6` {GIF tag A+D ×4: BITBLTBUF, TRXPOS 0, TRXREG w|h<<32, TRXDIR 0;
   IMAGE tag}, `REF qwc @pixels`, `CNT 2` {A+D TEXFLUSH, EOP on the last
   mip}; CLUT: 16×16 or 8×2 CT32, same shape), and each upload puts a
   **PATH3 window** into the VIF chain (below). Pixel/CLUT memory is read by
   DMA at kick time.
2. If nothing was uploaded: `CNT 1 {0,0}` + `qw {0,0,0,0x11000000}` (FLUSH).
3. `TexMan_EmitTex0(tm, id, ctx, &cursor)` (0x368970) for ctx 0 then 1:
   - id == tm+0x1f4c[ctx] (bound): only if the texture has a CLUT
     (+0x2c ≥ 0) and tm+0x1f54 ≠ id: `CNT 3`, DIRECT 2, tag NLOOP 1 EOP A+D,
     **TEX2_1/2** (0x16/0x17) = TexInfo+0x38 | CBP << 37; tm+0x1f54 := id.
   - else tm+0x1f4c[ctx] := id; n = 2 (mips = TexInfo+0x24 = 0), 3 (1..3),
     4 (≥ 4); `CNT n+2`, DIRECT n+1, tag NLOOP n EOP A+D:
     [MIPTBP1_1/2 (0x34/0x35) = +0x48 | TBP1 | TBP2<<20 | TBP3<<40] (mips ≥ 1),
     [MIPTBP2 (0x36/0x37) = +0x50 | TBP4..6] (mips ≥ 4),
     TEX1_1/2 (0x14/0x15) = TexInfo+0x40,
     TEX0_1/2 (6/7) = TexInfo+0x38 | TBP0 (+0x28) | CBP (+0x2c, or 0) << 37;
     if CLUT: tm+0x1f54 := id.

**PATH3 window** (`TexMan_EmitPath3Window` 0x368138, V): one VIF tag `REF
0x22 @0x44b200`, whose data is FLUSH; DIRECT 1 {empty GIF tag, EOP};
FLUSHA; **MSKPATH3 0**; ~100 NOPs; **MSKPATH3 1**; FLUSH; DIRECT 1 {empty};
FLUSHA. Everything queued in the GIF chain up to the next EOP runs in this
window, i.e. in VIF order between the surrounding draws.

**Native:** the only per-draw texture facts are, per context: id, TEX0
fields other than TBP/CBP/TBW (PSM, TW, TH, TCC, TFX, CPSM, CSM, CSA, CLD),
TEX1 (LCM, MXL, MMAG, MMIN, MTBA, L, K) and the mip count. **They are read
from TexInfo at RenderFrame time**, so per-draw mutations made at draw time
(slot 99 `SetTexFunction(id, 3)` rewrites TFX/TCC of +0x38) reach the GS
with the value TexInfo holds at EndFrame, for every draw of that id in the
frame (V: EmitTex0 reads +0x38/+0x40 when it emits). Snapshot TexInfo
+0x24/+0x38/+0x40 (+ pixels/CLUT pointers, format +0xc) per used id at
EndFrame. VRAM residency/TBP and the windows need no native counterpart
(ordering only matters for render-target textures, slots 53/61).

## 2. Material setup (`DeferMan_EmitMaterialSetup` 0x364cd0, V)

`(dm = a0, type = a1, mode = a2, w3 = a3, &cursor = t0)`; jump table
0x492040 (types 0 and 8 return). `M` = MatrixRecord at dm+0x69cd0 +
0x180·(dm+0x69cc0) (read when the chain is built; the CNT data is a copy).
Record fields (`frame.rs` names): +0x00 world (gm+0x5780 = `*(gm+0x13e4)`),
+0x40 `view` (gm+0x57c0: the guard-band projection), +0x80 `world_view`
(world × guard proj), +0xc0/+0xd0 `guard_scale/offset` (gm+0x5840/0x5850),
+0xe0 `view_proj` (gm+0x5860: viewport projection), +0x120 `world_clip`
(world × viewport proj), +0x160/+0x170 `screen_scale/offset`
(gm+0x58e0/0x58f0). "Set A" = {+0x80, +0xc0, +0xd0}, "set B" = {+0x120,
+0x160, +0x170}.

**MPG chains** (mode 0 `CALL addr`, a `RET` tag whose data is): STCYCL 4,4;
STMASK 0; STMOD 0; BASE; OFFSET; MPG pages to VU1 code address 0.

| type | CALL | BASE / OFFSET | MPG pairs | extra in the chain |
|---|---|---|---|---|
| 1 | 0x42e590 | 0x250 / 0xd8 | 1172 | — |
| 2 | 0x430a80 | 0x250 / 0xd8 | 2005 | — |
| 3 | 0x435bd0 | 0x26 / 0x1b6 | 1985 | — |
| 4 | 0x439a40 | 0x7 / 0x9a | 804 | FLUSH; UNPACK V4-32 1 @0x13b and 1 @0x29a = sprite GIF tag `0x502b4000_00008000`, REGS 0x52512 (PRIM 0x56 SPRITE TME ABE; ST RGBAQ XYZF2 ST XYZF2) |
| 5 | 0x43b3f0 | 0x3 / 0xb | 275 | same tag @0x19 and @0x209 |
| 6, 7 | 0x434990 | 0xc0 / 0x169 | 576 | — |

GIF tags below, all `NLOOP 0, EOP, PRE, NREG 3, REGS 0x412` (ST, RGBAQ,
XYZ2): `S1 = 0x302e4000_00008000` (PRIM 0x5c strip ctx1), `S2 =
0x312e4000_..` (0x25c strip ctx2), `F1 = 0x302ec000_..` (0x5d fan ctx1),
`F2 = 0x312ec000_..` (0x25d fan ctx2).

**What reaches VU1 (absolute addresses, no FLG), in emission order:**

| type | mode 0 | mode 1 | mode 2 |
|---|---|---|---|
| 1 | CALL MPG; `REF 12 @0x44b080` = UNPACK 11 @6: qw6 {0x80 ×4 int}, qw7-10 S1 S2 F1 F2, qw11-16 int {2,1,8,8} {16,8,48,24} {3,2,12,18} {36,24,108,72} {4,3,16,32} {64,48,192,144}; `CNT 7` UNPACK 6 @0 = M set A (qw0-3 world_view, qw4 guard_scale, qw5 guard_offset) | `CNT 7` UNPACK 6 @0 set A | — |
| 2 | CALL MPG; `CNT 13`: UNPACK 11 @0 = set A, {0x80 ×4}, S1 S2 F1 F2; then UNPACK 0x2a @0x200 fed by `REF 0x12 @*(X+0x1a0)`, `REF 0x12 @*(X+0x320)` (X = `*(gp+0x1158)`, BezierMan; **read by DMA at kick time**), `CNT 6` = {0x200,0x212,0,0} {1/3 ×4} {0x204,0x216,0,0} {1/5 ×4} {0x20a,0x21c,0,0} {1/7 ×4} (floats gp-0x28a8..) at 0x224-0x229 | as type 1 | — |
| 3 | CALL MPG; `CNT 8` UNPACK 7 @0 = set A + qw6 {0x10c, 0x2c2, 0, 0} int; `REF 7 @0x44b140` = UNPACK 2 @0x1f {S2, F2}, UNPACK 1 @0x3c8 {F1}, UNPACK 1 @0x3e4 {F1} | `CNT 7` UNPACK 6 @0 set A (qw6 and the tags stay) | — |
| 4 | CALL MPG; `CNT 8` UNPACK 7 @0 = M+0x80 (4 qw), M+0xc0 with z×0.0625, M+0xd0 with z×0.0625, {M+0x40, M+0x54, 0, 0} (view[0].x, view[1].y) | `CNT 8` UNPACK 7 @0 = **set B**: M+0x120 (4), M+0x160 z×0.0625, M+0x170 z×0.0625, {M+0xe0, M+0xf4, 0, 0} | — |
| 5 | CALL MPG; `CNT 4` UNPACK 3 @0 = M+0x160 z/16, M+0x170 z/16, {M+0xe0, M+0xf4, 0, 0} | same | — |
| 6 | CALL MPG; `CNT 7` UNPACK 6 @0 set A; `CNT 5` UNPACK 4 @0x392 = S2, then 3 rows E0..E2 (below); `CALL w3` if w3 ≠ 0 | `CNT 7` set A; `CNT 5` @0x392 as mode 0; `CALL w3` if ≠ 0 | `CALL w3` if ≠ 0 |
| 7 | CALL MPG; `CALL w3` if ≠ 0 | `CALL w3` if ≠ 0 | `CALL w3` if ≠ 0 |

`CNT n` = `{0x10000000|n, 0}` then a qw `{0, 0, 0, UNPACK}` (STCYCL is the
4,4 the MPG chain set). Types 1-5 do nothing for mode 2.

- **Type 4 mode 0 vs mode 1 really differ** (V): the first type-4 draw after
  a program load runs with the guard-projection set, later matrix changes
  upload the viewport-projection set. Model both.
- **Type 6 env rows** (V, both modes): C at 0x4ffb80 (written once, flag
  gp+0x28a0) = rows {0.5,0,0,0}, {0,−0.5,0,0}, {0,0,0,0}, {0.5,0.5,1,1};
  `E_i = C0·W_i.x + C1·W_i.y + C2·W_i.z + C3·W_i.w` with `W` = **M+0x00
  (world)**, i = 0..2, i.e. `E_i = (0.5·W_i.x + 0.5·W_i.w, −0.5·W_i.y +
  0.5·W_i.w, W_i.w, W_i.w)`. Clobbers VU0 vf1-vf15/ACC (guest-visible, I:
  harmless).
- **w3 packets** (built at draw time in VifBuff, run by the `CALL`):
  - slot 124 `EmitLightingPacket` 0x396b40 (a0 = gm, V): sets *curState type
    6, w3 = gm+0x5a00 & 0x3fffffff; packet `RET 12`: {NOP, FLUSHE, STCYCL
    1,1, UNPACK V4-32 10 @7}, **10 qw from gm+0x6bb0** (copied at the call),
    {0, 0, 0, MSCAL 0x22a (byte 0x1150)}; gm+0x5a00 += 0xd0. Every type-6
    mode 0/1/2 setup re-runs this lighting program.
  - slot 101 `SetShadowHeader` 0x37ba50 (a0 = gm, a1 = matrix (4 qw), a2 →
    qw, a3 → qw, V): `RET 7`: {NOP, FLUSHE, NOP, UNPACK V4-32 6 @0}, the 4
    matrix rows, *a2, *a3; gm+0x5a00 += 0x80; *curState type 7, w3 =
    packet. This is the whole type-7 header (qw0-5); the bucket matrix is
    ignored for type 7.

### 2.1 Proposed native form

No chain walking needed: a pure function of (type, mode, M, w3 capture).

```rust
/// What a material setup leaves in VU1 data memory (absolute qw address,
/// value), plus the program to bind (by type) and its BASE/OFFSET.
pub struct VuSetup { pub program: Option<Material>, pub writes: Vec<(u16, u128)>, pub runs: Option<u16> /* type 6: MSCAL 0x22a */ }
pub fn material_setup(ty: u32, mode: u8, m: &MatrixRecord, w3: Option<&W3Capture>) -> VuSetup;
pub enum W3Capture { Lighting([u128; 10]) /* slot 124 */, ShadowHeader([u128; 6]) /* slot 101 */ }
```
- Keep a per-frame `VuState` (qw0..0x3ff of the data memory a renderer
  cares about) updated by `material_setup` at each `Draw::setup` and by
  captured packets' absolute writes (slot 89 guard, rider bones/morph,
  lighting). A draw's header is `VuState[0..6]` at its CALL; this replaces
  "frame-start VU1 image" for the static data.
- Captures to add: slot 124 (10 qw of gm+0x6bb0, keyed by its packet start =
  w3), slot 101 (6 qw, keyed by w3), slot 89 (the 2 guard qwords per pass),
  type 2's two 18-qw blocks (`*(X+0x1a0)`, `*(X+0x320)`, read at EndFrame).

## 3. Post passes (GIF/PATH3; V unless marked)

All three write GS packets into the **GIF chain** (through SprStream, i.e.
`*gifCursor`, except Bloom's final restore which is written uncached
directly) and put **one PATH3 window per view** into the VIF chain after
them; each pass is one GIF packet ending in an EOP tag, so it executes in
that window. Common values: `W, H` = vt slot 10/11 (vt+0x54/+0x5c, screen
size), `FBW = W/64`, `fbp` = gm+0x5a84, `zbp` = gm+0x5a80, view rect
`(vx, vy, vw, vh)` = u16 at gm+0x59b0 + 8·view, XYOFFSET default
`((0x1000−W)/2·16, (0x1000−H)/2·16)`, global scissor u16 {x,y,w,h} at
gm+0x5900. All sprites are drawn in **32-px columns, n = vw >> 5**: a
remainder `vw % 32` is left untouched (quirk). Every pass ends with an EOP
A+D block restoring FRAME_1 (fbp, FBW, CT32, no mask) and XYOFFSET_1 (fog
and colour filter also SCISSOR_1 := gm+0x5900; bloom does not).

### 3.1 Depth fog (`RenderDepthFog` 0x36ac00, after range 0, per view)

Gate: gp+0x1234 (0x4a4324) ≠ 0; gp+0x1238 → `ResetDepthFogParams` (params :=
gp+0x123c..0x1248 and 0x504720..0x50472c); params `p` = gm+0x6c64 + 0x20·v:
p0 start, p1 end (view-space distances), p2 mode (int 0 lin, 1 exp, 2 exp2),
p3 density, p4 unused, p5-p7 colour r, g, b (0..1). Drawn iff `p3 ≠ 0.0 ||
p2 == 0`.

**CLUT** (`BuildDepthFogClut` 0x36a428, 256 CT32 entries at 0x507b80 +
0x400·v, rebuilt only when (near, far, rgb, mode, density) differ from the
cache gp+0x2938../+0x10b8; native: just recompute):
- `near = clamp((int)(z/w) >> 8, 0, 255)` of `(0, 0, p0, 1)·S_v`, `far` the
  same for p1, `S_v` = gm+0x5930 + 0x40·v (row-vector, `View::screen`);
  `span = max(1, near − far + 1)`; `rgb = (int)(p5·255) | (int)(p6·255)<<8 |
  (int)(p7·255)<<16`.
- For i < near, `j = max(i, far)`: mode 0 `a = clamp((j − far)·128 / span, 0,
  128)` (int division); mode 1 `x = −p3·0.1·(near − j + 1)/span`; mode 2 `x =
  −(p3·0.1·(near − j + 1)/span)²`; modes 1/2 `a = (int)(clamp(e(x), 0, 1)·128)`
  with `e(x) = 1 + x + x²/2 + … + x⁷/5040` (Horner, constants gp−0x2898..).
  For i ≥ near: a = 0x80. Entry = `rgb | a << 24`, stored at the CSM1 index
  `(i & ~0x18) | (i & 8) << 1 | (i & 0x10) >> 1`.

**GIF packets** (`EmitDepthFogClutUpload` 0x36aa60, `EmitDepthFogZCopy`
0x36ae20, `EmitDepthFogBlend` 0x36b158):
1. CLUT upload to a fresh 0x400-block VRAM allocation (freed right after).
2. Z copy, full framebuffer: FRAME_1 = fbp, FBW, **PSMCT16, FBMSK 0x3fff**;
   ZBUF_1 zbp Z24 ZMSK; TEST_1 0x30000; XYOFFSET_1 default; SCISSOR_1 x
   0..gm+0x5a30−1, y 0..2·gm+0x5a34−1; TEXFLUSH; **TEXA = 0x80_0000_0000
   (TA0 0, TA1 0x80; never restored)**; TEX1_1 0; TEX0_1 = zbp·32, TBW FBW,
   **PSMZ16**, 1024², TCC, DECAL; PRIM 0x116; RGBAQ 0x80 ×4 Q 1. Sprites k <
   gm+0x5a30/16: UV (16k, 0)+½ .. (16k+8, 2H)+½ → XY (16k+8, 0) .. (16k+16, 2H).
   Effect: GS-memory aliasing moves Z bits into the colour buffer's alpha
   byte. The exact bit mapping is **I** (`phase3-render.md`: index = Z >> 8,
   consistent with the blend's Z = 0xffff test below); derive it with the GS
   model on a Z ramp before relying on it.
3. Blend, view rect: FRAME_1 fbp CT32; ZBUF Z24 ZMSK; TEST_1 0x70000 (ZTST
   GREATER); XYOFFSET_1 = default + (vx, vy)·16 (cancels in the vertices);
   ALPHA_1 = 1 (`(Cd − Cs)·As + Cs`); TEXFLUSH; SCISSOR_1 = view rect; TEX1 0;
   TEX0_1 = fbp·32, FBW, **PSMT8H**, 1024², TCC, DECAL, CBP = CLUT, CLD 1;
   PRIM 0x156; RGBAQ 0x80. Sprites: window (vx+32k, vy)..(vx+32k+32, vy+vh),
   **Z = 0xffff**, UV = window + ½.
   **Native:** in the view rect, for pixels with stored Z < 0xffff:
   `idx = alphaByte(Z)`, `c = clut[idx]`, `frame.rgb = lerp(c.rgb,
   frame.rgb, c.a/128)`, `frame.a = c.a`.

(`EmitDepthFogFlatFill` 0x36b5e8 exists but RenderDepthFog does not call it.)

### 3.2 Bloom (`RenderBloom` 0x36c790, start of phase 2, per view)

Globals (V from the disassembly; gp-relative): enable 0x4a43c8, reset
0x4a43cc (→ `ResetBloomParams`: p0-p4 := 0x4a43d4..0x4a43e4, p5/p6 :=
0x4a43f0/0x4a43f4), size shift `L` 0x4a43d0, bright FIX 0x4a43d8, level
weights G0 0x4a43e8, G1 0x4a43ec, tap offset 0x4a43f8. Per view `p` =
gm+0x6cd4 + 0x1c·v: p0 threshold, p1 **unused**, p2 downsample weight, p3
first-tap weight, p4 frame multiplier, p5 / p6 level-2/3 weights.
`k(x) = (int)(x·127.5)`. Weights `Wt = [k(G0), k(G1), k(p5), k(p6)]`; all 0 →
nothing. Buffers in the Z buffer's VRAM: `A = zbp·32`, `B = A + 0x400`
(TBP). `S = 1 << L`, `off = (int)(0x4a43f8·16)` (1/16 texel).

Sequence (the Z buffer is overwritten, so this is after LensFX):
1. `Blur(src = frame (fbp·32, TBW W/64), rect (vx,vy,vw,vh) → A, S×S, taps 1,
   off 0, weight k(p3))`.
2. `Fill(A, S×S, T = k(p0), F = k(0x4a43d8))`: `A = clamp((A − T)·F/128)`.
3. For i = 0, 1, 2: if `Wt[i]`: `Composite(level i → frame)`; then
   `Blur(level i (S_i × S_i at 0,0) → next buffer (B, A, B), S_i/2, taps 4,
   off, weight k(p2))`.
4. If `Wt[3]`: `Composite(level 3 → frame)`.
5. Restore FRAME_1 / XYOFFSET_1 (EOP); PATH3 window.

`Blur` (`EmitBlurDownsample` 0x36b9d8; f12 weight, a0 gm, a1 srcTBP, a2 src
width, a3 x, t0 y, t1 w, t2 h, t3 dstTBP, stack dstW, dstH, off, taps,
&gif): FRAME_1 dst, FBW max(1, dstW/64), CT32, FBMSK 0xff000000; ZBUF ZMSK;
TEST 0x30000; XYOFFSET 0; ALPHA_1 0x68 FIX 0x80 (`Cs + Cd`); TEXFLUSH; SCISSOR
0..dstW−1 × 0..dstH−1; TEX1 0x60 (bilinear); TEX0 src, TBW max(1, srcW/64),
**PSMCT24**, 1024², TCC, MODULATE; CLAMP_1 REGION_CLAMP [x, x+w−1] × [y,
y+h−1]; PRIM 0x116 (**no ABE: tap 0 replaces**); RGBAQ = `(int)(weight /
taps)` ×3, A 0x80; tap 0 sprites; PRIM 0x156 (ABE); taps 1..3 sprites (add).
Column k (dst x 32k .. min(32k+32, dstW), y 0..dstH), in 1/16 units:
`U0 = 16x + 8 + off`, `span = 16w − 2off`, `ua = U0 + (512k·span)/(16·dstW)`,
`ub = U0 + (min(512(k+1), 16dstW)·span)/(16·dstW)` (integer divisions),
`V0 = 16y + 8 + off`, `V1 = 16(y+h) + 8 − off`; tap t adds (su, sv)·off with
(su, sv) = (−,−), (+,+), (+,−), (−,+) to both corners.

`Fill` (`EmitBlendedFill` 0x36c188): FRAME_1 A, FBMSK 0xff000000; XYOFFSET 0;
ALPHA_1 0xa1 FIX F (`(Cd − Cs)·F`); TEXFLUSH; PRIM 0x146 (untextured, ABE);
RGBAQ T ×3; sprites 32 px wide, full height (TEST/ZBUF/SCISSOR left from the
blur).

`Composite` (`EmitBloomComposite` 0x36c398; a0 gm, a1 srcTBP, a2 srcW, a3
srcH, t0 frame TBP, t1 vx, t2 vy, t3 vw, stack vh, weight, fix = k(p4),
&gif): FRAME_1 frame, FBW, FBMSK 0xff000000; ZBUF ZMSK; TEST 0x30000;
XYOFFSET 0; ALPHA_1 0x29 FIX fix (`Cd·fix + Cs`); TEXFLUSH; SCISSOR view
rect; TEX1 0x60; TEX0 src CT24 MODULATE; CLAMP REGION [0, srcW−1] × [0,
srcH−1]; PRIM 0x156; RGBAQ = weight ×4. Column k: XY (vx+32k, vy) ..
(vx+32k+32, vy+vh); `du = (srcW / n)·16` (**integer division**: with vw
= 640 and srcW = 128 the columns cover 120 of 128 texels); U 8+k·du ..
8+(k+1)·du, **V 0 .. 16·srcH** (no half texel).
Native: `frame.rgb = clamp(frame·fix/128 + bilinear(level)·weight/128)`.

### 3.3 Colour filter (`RenderColorFilter` 0x3904a0, phase 2, per view)

Gate: 0x4a45d8 ≠ 0; 0x4a45dc → `ResetColorFilter` (from 0x4a45e4..0x4a45f8).
`p` = gm+0x6ca4 + 0x18·v: scale r, g, b; offset r, g, b. Skipped if scale =
(1,1,1) and offset = (0,0,0). Packet (`EmitColorFilterPacket` 0x3905e8):
FRAME_1 fbp CT32; ZBUF ZMSK; TEST 0x30000; XYOFFSET view-based; SCISSOR
view rect; CLAMP_1 5 (CLAMP); TEX1 0; TEX0 fbp·32, CT32, TCC, MODULATE; PRIM
0x116; RGBAQ = `k(scale)` RGB, A 0x80. Sprites: window (vx+32k, vy) ..
(+32, vy+vh), Z 0xffff, UV = window + ½ → `frame.rgb = min(255,
frame·k(s)/128)`, alpha unchanged. If any `k(offset) > 0`: ALPHA_1 0x48
(`Cs·As + Cd`), PRIM 0x46 (flat, ABE), RGBAQ `k(offset)` A 0x80, same
sprites → `frame.rgb += k(offset)` (clamped). A negative offset channel
OR-ed into RGBAQ corrupts the colour word (quirk; `(int)` of a negative
value).

## 4. What `frame.rs` has and what is missing

Has (V by reading): bucket order and layer split, `Emitter` (modes 0/1/2,
SCISSOR/ZBUF/TEST/CLAMP/ALPHA prediction incl. the default-state quirk and
stale ALPHA presets), `Draw::header_matrix` (correct timing),
`MatrixRecord` with all nine fields, `Pass::{DepthFog, Bloom, ColorFilter,
DefaultState, PhaseBoundary, PostChain}` with the per-view params,
`View::screen`.

Missing for chain-free rendering:
1. `MatrixRecord::header_qwords()` is right only for types 1, 2, 3, 6
   (set A, verbatim). Missing: type 4 mode 0 (set A, z/16, qw6 from +0x40/
   +0x54) vs mode 1 (set B, z/16, qw6 from +0xe0/+0xf4); type 5 (3 qw set B);
   type 7 (from the slot-101 capture); the type-6 env rows at 0x392-0x395
   (from **world**, M+0x00) and lighting at 7-0x10 (slot-124 capture).
2. The static VU1 data per type (§2 table: qw6-16, 0x1f/0x20, 0x3c8/0x3e4,
   0x13b/0x29a, 0x19/0x209, type 2's 0x200 block) and BASE/OFFSET per type,
   which today come from the frame-start VU1 image.
3. VU1 writes made by draw packets that later draws inherit: slot 89 guard
   qw4/5 (z = 0 in pass 3), rider bones/morph (0x11+, 0xb4, 0x3ff).
   `VuState` of §2.1 covers them.
4. Texture binding: only ids; TEX0/TEX1 per context (from a TexInfo
   snapshot at EndFrame, after draw-time `SetTexFunction`), mip count, CLUT
   reload (TEX2) are not modelled; the TEXA left by depth fog (TA0 0, TA1
   0x80) is not tracked.
5. Post passes: the enable globals (0x4a4324, 0x4a43c8, 0x4a45d8) and skip
   conditions (§3), the bloom globals (0x4a43d0/d8/e8/ec/f8; p1 unused), the
   view rects (gm+0x59b0), the global scissor (gm+0x5900), the fog CLUT
   (recompute from params), the 32-px column remainders and integer UV
   truncations listed above. The Z-to-alpha mapping of the Z copy remains I.
6. Guest-visible side effects of RenderFrame a replacement must keep (if it
   stops running the generated body): dm+0x69ca8..+0x69cc4, tm+0x1f4c/
   +0x1f50/+0x1f54, VRAM heap state, 0x53ab80, the depth-fog CLUT cache
   (gp+0x10b8, +0x2938..), 0x4ffb80 + gp+0x28a0, SprStream mode, VU0 regs.
