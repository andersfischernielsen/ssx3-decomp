# N2 capture spec: DIRECT (GIF over VIF) and other non-mesh draws

Area: slots 72 `DrawIndexedTriangles`, 73 `DrawLineStrip` (+ `LineStrip_*`),
75 `DrawLineShape`, 78 `DrawSprite3D`, 79 `DrawRotatedSprites`, 80 / 81
`DrawGlyphs(W)`, 53 / 54 / 55 (render target), 61 `DrawTransition`, 87
`DrawCircle`, 89 `DrawShadowVolume` (its DIRECT parts), 90 `DrawBoardWake`,
and the type-3 entry 0x2400.

Marks: **V** = checked in the disassembly (or generated Rust where Ghidra has
no function); **D** = decompiler only; **I** = inferred.

## 0. Common facts

**Globals.**
- gp = 0x4a30f0.
- gp+0x2a90 = `gm` (0x4a5b80).
- gp+0x2a94 = scratchpad base pointer (0x4a5b84 → 0x70000000, I). Slot 72 keeps a 4×4 matrix there, and slots 89 / 90 keep the 0x30-byte GsPkt builder state there (`EA_MemSet(*(gp+0x2a94), 0, 0x30)`).
- gp+0x1384 = SprStream.

**Corrections to `n2-render-layer.md` / `phase3-boundaries.md`.**
- 53 is not an upload, 54 is not a clear, and 55 is not a textured sprite. Instead:
  - **53 = set the render target to a texture**;
  - **54 = restore the frame buffer target**;
  - **55 = fill the bound target with a colour, then inset the scissor**.
- 87 has no packet of its own: it calls slot 73.
- 90 has no DIRECT part.
- 0x2400 is not emitted by any slot in this area (§9).

**Record writer** (V, identical in every function here; the same as slot 71):
- `start = gm+0x5a00` before the body.
- If `*(DeferMan) < 0xa28` (2600), the body writes record `count*0x80` at `DeferMan+0x80 | 0x30000000`:
  - 0x00..0x13 = `*curState` (5 words), after w2 bits 0-4 := `*(DeferMan+0x69cc4) & 0x1f` (scissor index; V: `lui 6; ori 0x8000; lw 0x1cc4`). The decompiler shows `piVar[0x1a731]` = +0x698c4, which is wrong; `particles.md` copies that wrong value;
  - +0x14 = start;
  - +0x18 = end;
  - +0x1c = short `*(DeferMan+0x69cc0)` (matrix index);
  - +0x1e = short continuation flag (0 except 54 and 55, which write **1**).
- After the record, `gm+0x5a00 = end + 0x10` (room for RET/NEXT).
- Above the cap there is no record, but the packet and the cursor advance still happen.

**Allocation.**
- Direct (written at `start | 0x30000000`, uncached): 53, 54, 55, 72, 73, 75, 78, 79, 80, 89.
- SprStream: 61 and 81. They use `SprStream_BeginBlock(spr, start, -1, 0)` … `EndBlockAligned(spr, end, 4)`. In this runtime the fromSPR DMA completes inside `EndBlock` (see `particles.md` §0), so the packet is in RAM when the body returns.
- **For every function here, the packet can be read back from RAM `start..end` right after the generated body. This is the simplest exact capture, because these packets have fixed layouts.** The field formulas below are for a later native replacement (Phase B), and to know what to keep.

**Packet framing** (all DIRECT packets, V):
- qw0 = DMA `CNT qwc` with the upper half 0 (VIF NOP, NOP). `first_tag` (u128) = `0x1000_0000 | qwc`.
- qw1 = `{0, 0, 0, DIRECT k}` (word3 = `0x5000_0000 | k`, k = qwc-1).
- Then GIF data.
- No FLUSH / FLUSHA precedes the DIRECT, except 53/54, where qw1 = `{0, 0, FLUSH, DIRECT}` (word2 = 0x1100_0000).

**`*curState` side effects** (game-visible; a replacement must keep them):
- every record writer: w2 bits 0-4 := scissor index;
- 72, 75: w0 = (w0 & ~0x3c0) | 0xc0 (type 3), w3 &= 0xc000_0000, both before the record (V);
- 87: +0x10 (short) = 0xffff (untextured), w1 bits 2-6 := 1 (D);
- 89 and 90: see §7 and §8.

**gm fields used.**

| Field | Meaning |
|---|---|
| +0x5900 short[4] | scissor x, y, w, h |
| +0x5a30 / +0x5a34 | fbWidth / fbHeight |
| +0x5a3c | drawPsm |
| +0x5a50 | fbw (/64) |
| +0x5a54 / +0x5a58 | dispWidth / dispHeight |
| +0x5a5c | dispFbw |
| +0x5a80 | ZBP |
| +0x5a84 | drawFbp |
| +0x5a88 | dispFbp |
| +0x6b30..+0x6b6f | projection matrix (columns; `vmula.x/y/z, vmadd.w`): object → GS window coordinates, as written |
| +0x6b70 / +0x6b74 / +0x6b78 | clip minimums for x / y / z |
| +0x6b80 / +0x6b84 / +0x6b88 | clip maximums |
| +0x6af0 / +0x6b04 | screen x / y scale |
| +0x6b18 / +0x6b28 | z scale / offset (2D font path) |
| +0x6b20 / +0x6b24 | screen x / y offset (2D font path) |
| +0x6b90 | "view matrices valid" (`PS2GraphicsMan_UpdateViewMatrices` 0x37d968 when 0) |
| +0x13e4 | model matrix stack pointer (slot 31 push / 32 pop / 37 load identity; 32 clears +0x6b90) |
| +0x10dc | current view index |
| +0x6d40 + 0x230·view | int: font 2D mode |

**Proposed command** (one for this whole area; details per function below):

```rust
/// GS work done by a DIRECT packet, in order.
Cmd::Gs(Vec<GsOp>)
pub enum GsOp {
    /// A+D register write (FRAME_1, ZBUF_1, TEST_1, FBA_1, SCISSOR_n,
    /// XYOFFSET_1, ALPHA_1, TEXA, TEX0_1, TEX1_1, TEXFLUSH).
    Set { reg: u8, value: u64 },
    /// One primitive run: PRIM, then vertices as kicked. Per vertex
    /// (RGBAQ, ST or UV, XYZ2), constant values repeated.
    Prims { prim: u16, verts: Vec<[u64; 3]> },
}
```

- `Cmd::Gs` has no VU1 starts (`starts() = 0`).
- The renderer plans it the way `gs.rs` plans decoded GIF:
  - `Set` updates its GS register model;
  - `Prims` draws with the current FRAME / ZBUF / TEST / ALPHA / TEX0 (TEX0 is from the bucket's `TexMan_BindTextures`, which already precedes the CALL).
- Render-target ops (53 / 54 / 55 / 61) need a TexId to map FBP → texture. Add `tex: Option<u32>` to the Recorded, or give them dedicated variants (§5).

## 1. Slot 72 `DrawIndexedTriangles` 0x379bd0 (V)

**Arguments:**
- a0 gm;
- a1 n (triangles);
- a2 unused (overwritten);
- a3 `idx: *u8[3n]`;
- t0 `pos: *[f32;3]` (stride 0xc);
- t1 `uv: *[f32;2]` (stride 8);
- t2 `col: *[f32;4]` (stride 0x10, order **a, r, g, b**);
- t3 `offset: *[f32;3]`.

The only caller is `UIVector_Render` (menus).

**Transform.**
- M = the global identity 0x4ff1a0 (BSS, filled at static init; read by about 60 functions) with row 3 := M·(offset, 1). That is, a translation by `offset`, staged in the scratchpad 0x70000000..0x3f (the scratchpad is overwritten).
- C = (gm+0x6b30) · M.
- Per index i: p = C·(pos[i], 1); q = 1/p.w (VU DIV); p *= q (all lanes).
- XYZ2 = `int(p.x) | int(p.y)<<16 | int(p.z)<<32`. These are cvt.w truncations, OR'd as sign-extended 64-bit values.

**Packet.** The branch is on `*(short*)(curState+0x10) < 0` (untextured):

| | Untextured | Textured |
|---|---|---|
| qwc | 3n+4 | `9n/2 + (n&1) + 4` (C int division) |
| qw2 | GIFtag `0x1000000000000001`, REGS 0xe (NLOOP 1, A+D, EOP 0) | same |
| qw3 | PRIM (A+D addr 0) = **0x4b** (triangle, IIP, ABE) | **0x5b** (triangle, IIP, TME, ABE, FST=0: ST) |
| qw4 | `0x6400000000008000 \| n`, REGS 0x515151 (REGLIST, NREG 6: RGBAQ, XYZ2 ×3) | `0x9400000000008000 \| n`, REGS 0x512512512 (NREG 9: ST, RGBAQ, XYZ2 ×3) |
| per vertex | 1 qw: RGBAQ, XYZ2 | 3 dwords: ST, RGBAQ, XYZ2 |
| RGBAQ | `int(r) \| int(g)<<8 \| int(b)<<16 \| int(a·0.5)<<24 \| 1.0f<<32` | **every channel × 0.5** |
| ST | — | raw `uv[i]` words, Q = 1 |
| end | | an odd n adds one zero dword (pad) |

- 3n vertices, in index order (triangle list).
- end = start + 0x50 + data; the record follows.

**Capture:** `Cmd::Gs([Prims{prim, verts}])`. Read the packet back, or copy idx / pos / uv / col / offset and run the transform.

## 2. Slot 73 `DrawLineStrip` 0x381ad0 + `LineStrip_*` (V)

**Arguments:** a0 gm, a1 n (vertices), a2 `verts` (0x30 each):
- +0x00 S, +0x04 T (f32);
- +0x10..0x1c R, G, B, A (u32 each);
- +0x20 position xyzw (w used: `vmaddw`).

87 `DrawCircle` is the only other user (§6). `LineStrip_*` are reached only from 73 (xrefs V).

**Per vertex:**
- p = (gm+0x6b30)·pos; q = 1/p.w;
- s = (p.x·q, p.y·q, p.z (not divided), q);
- outcode as in slot 78 (§4).
- Visible segments go to the strip.
- A segment crossing a clip plane goes through `PS2GraphicsMan_ClipLineSegment` 0x383a10, which writes clipped endpoints. A new strip begins after each exit.
- **Do not port the clipper for Phase A: read the packet back.**

**LineStrip state** (on the stack):

| Offset | Field |
|---|---|
| +0 | `&gm+0x5a00` |
| +4 | write pointer (\|0x3000_0000) |
| +8 | packet start (0 = none yet) |
| +0xc | open strip header |
| +0x10 | vertex count |

- **`Begin`:**
  - if this is the first strip, packet start = ptr and ptr += 0x20 (qw0, qw1);
  - otherwise it `Close`s the open strip;
  - then header = ptr, ptr += 0x30, count = 0, add the vertex.
- **`AddVertex(v, s)`** writes 3 dwords (0x18):
  - RGBAQ = `R | G<<8 | B<<16 | A<<24` (u32 OR, no masking) `| bits(s.q)<<32`;
  - ST = `bits(S·q) | bits(T·q)<<32` (floats, no cvt);
  - XYZ2 = `int(s.x) | int(s.y)<<16 | int(s.z·q)<<32`.
- **`Close`:**
  - an odd count gets a zero dword pad;
  - header qw0 = GIFtag `0x1400000000000001`, REGS 0 (REGLIST, NREG 1, NLOOP 1: PRIM);
  - qw1 = `{PRIM 0x4a, pad 0}` (line strip, IIP, ABE; no TME);
  - qw2 = `0x3400000000008000 | count`, REGS 0x521 (REGLIST, NREG 3: RGBAQ, ST, XYZ2; **EOP=1 on every strip**).
- **`Finish(ls, 2)`:**
  - if no strip was begun: no packet, no record, cursor unchanged;
  - otherwise it closes the open strip, qwc = (ptr - start)/16, and writes qw0 = `CNT qwc-1`, qw1 = `{0, 0, 0, DIRECT qwc-2}`;
  - the record (via gp+0x2a90's gm) has start..ptr; cursor = ptr + 0x10.

**Capture:** `Cmd::Gs(strips.map(|s| Prims{prim: 0x4a, verts}))`. Several strips go in one packet and one record.

## 3. Slot 75 `DrawLineShape` 0x381f10 (V): VU1, not DIRECT

**Arguments:**
- a0 gm;
- a1 `coeffs: *[u128; 4]` (C3, C2, C1, C0: a cubic, **not** a matrix);
- a2 `colour: *[f32;4]` (a, r, g, b);
- a3 n.

**curState:** type 3, w3 &= 0xc0000000.

**Packet** (0xb0 bytes, CNT 10):
- qw1 = `{0, 0, 0, UNPACK V4-32 num 8 addr 0 FLG}` (0x6c088000).
- TOP+0..7:
  - +0: GIFtag `0x1001400000000001`, REGS 0xe (NLOOP 1, PRE, PRIM 2 line strip, A+D);
  - +1: `{RGBAQ, 1}` with RGBAQ = `int(r·128) | int(g·128)<<8 | int(b·128)<<16 | int(a·128)<<24 | 1.0f<<32`;
  - +2: GIFtag `0x1001400000008000 | n`, REGS 4 (EOP, PRE, PRIM 2, PACKED XYZF2);
  - +3..6: coeffs[0..3];
  - +7: `{n, 0, 0, 0}`.
- qw10 = `{0, 0, 0, MSCAL 0x3bf0}` (0x1400077e).

**Capture:** `Cmd::Vu(vec![Write{addr: 0, rel: true, data: 8 qw}, Start(0x3bf0)])`. `mat3::line_shape` already runs it natively with the bucket header (qw0-5).

**Callers:** statically only `MultiSplineModifier_DebugDrawPath`, but `phase3-render.md` saw 73-183 calls per race capture. So there is another (virtual) caller (I, open).

## 4. Slot 78 `DrawSprite3D` 0x377cf0 (V)

**Arguments:**
- a0 gm;
- a1 `pos: *[f32;4]` (w used);
- a2 `size: *[f32;2]`;
- a3 `st0: *[u32;2]`;
- t0 `st1: *[u32;2]`;
- t1 `colour: *[f32;4]` (a, r, g, b).

**Geometry:**
- p = (gm+0x6b30)·pos; iw = 1/p.w;
- a = (p.x·iw, p.y·iw, p.z, iw);
- d = (gm+0x6af0·size[0]·iw, gm+0x6b04·size[1]·iw, 0, 0);
- lo = a - d, hi = a + d.

**Outcode per corner:**
- x < +0x6b70 → 2; x > +0x6b80 → 1;
- y < +0x6b74 → 8; y > +0x6b84 → 4;
- if w < 0: code ^= 0x2f; else z > +0x6b78 → 0x20, z < +0x6b88 → 0x10.
- **When `code(lo) & code(hi) != 0`, nothing is written: no packet, no record.**

**Packet** (0x60 bytes, CNT 5, DIRECT 4):
- GIFtag `0x6400000000008001`, REGS 0x525120 (REGLIST, NREG 6, NLOOP 1: PRIM, ST, RGBAQ, XYZ2, ST, XYZ2);
- PRIM **0x5e** (sprite, IIP, TME, ABE; ST);
- ST0 = `st0[0] | st0[1]<<32` (raw);
- RGBAQ = colour·128 (a, r, g, b order as in 75), Q = 1.0;
- XYZ2 = `int(lo.x) | int(lo.y)<<16 | int(p.z·iw)<<32`;
- ST1 = raw st1;
- XYZ2 hi, the same way.

**Capture:** `Prims{0x5e, [[rgbaq, st0, xyz_lo], [rgbaq, st1, xyz_hi]]}`.

## 5. Slot 79 `DrawRotatedSprites` 0x3781a0 (V)

**Arguments:**
- f12 angle;
- a0 gm;
- a1 `pos: *vec4`;
- a2 `size: *[f32;2]`;
- a3 `st0`;
- t0 `st1`;
- t1 `colour` (a, r, g, b);
- t2 count.

**Geometry:**
- (s, c) = `Math_SinCos(angle)` 0x31be50.
- R = *(gm+0x13e4) (model matrix, row-major floats):
  - U = c·(m0, m4, m8, m12) + s·(m1, m5, m9, m13);
  - V = -s·(…) + c·(…).
- P1 = pos + U·size[0], P2 = pos + V·size[1] (w := 1).
- Project with slot 92 `ProjectToScreen` 0x37dd20. It returns `(int(X - (2048 - fbW/2)), int(Y - (2048 - fbH/2)), clamp(int z, 0, 0xffffff), w)` as floats.
- C = proj(pos), A = proj(P1) - C, B = proj(P2) - C.
- Corners: v0 = C-A-B, v1 = C+A-B, v2 = C-A+B, v3 = C+A+B.
- X = (0x800 - fbW/2)·16 + int(v.x·16), likewise Y; Z = int(C.z) for all four.

**Packet:** `count` identical copies of 0x80 bytes, each:
- `CNT 7`, `{0, 0, 0, DIRECT 6}`;
- GIFtag `0xa400000000008001`, REGS 0x5252525210 (REGLIST, NREG 10: PRIM, RGBAQ, (ST, XYZ2)×4);
- PRIM **0x54** (triangle strip, TME, ABE, flat);
- RGBAQ = colour·128, Q = 1.0;
- ST per corner: (s0, t0), (s1, t0), (s0, t1), (s1, t1). These are raw words, re-read from a3 / t0 for each copy.

**Edge cases:**
- There is no culling.
- `count <= 0`: an **empty packet** (start == end). A record is still written and the cursor still moves by 0x10, so the chain CALLs a bare RET/NEXT. Record nothing for it.

**Capture:** one `Prims{0x54, 4 verts}` per copy. Repeats draw the same quad several times: keep them, because they add up when blended.

## 6. Slots 80 / 81 `DrawGlyphs` 0x378808 / `DrawGlyphsW` 0x379028 (V)

**Arguments:**
- f12 x, f13 y, f14 sx, f15 sy, f16 uScale, f17 vScale, f18 z;
- a0 gm;
- a1 font;
- a2 text (80: u8, 81: u16; 0-terminated);
- a3 colour (a, r, g, b).

**Font:**
- +0 count;
- +4 glyph table;
- +8 default glyph;
- +0xc first direct code;
- +0x10 last direct code.

**Lookup:**
- code < first: skipped (no advance);
- code ≤ last: `table[code - first]`;
- otherwise a binary search over [last-first, count), with the default glyph when not found.

**Glyph** (0xc bytes):
- +0 u16 code;
- +2 u8 w, +3 u8 h;
- +4 u16 u, +6 u16 v;
- +8 s8 advance, +9 s8 xoff, +0xa s8 yoff.

**Mode:** `*(gm + 0x6d40 + 0x230·*(gm+0x10dc))`.

*2D mode (≠ 0):*
- zi = `int(+0x6b18·z + +0x6b28)`;
- x0 = (x + xoff·sx)·+0x6af0 + +0x6b20;
- y0 = (y + yoff·sy)·+0x6b04 + +0x6b24;
- x1 = x0 + w·sx·+0x6af0;
- y1 = y0 + h·sy·+0x6b04.

*3D mode (= 0):*
- centre (x + xoff·sx + w·sx/2, y + yoff·sy + h·sy/2, z, 1) through gm+0x6b30;
- half extents (+0x6af0·w·sx/2·iw, +0x6b04·h·sy/2·iw);
- Z = int(p.z·iw), as in 78;
- no culling.

**Per glyph** (both modes):
- ST0 = `bits(u·uScale) | bits(v·vScale)<<32`;
- ST1 = `bits((u+w)·uScale) | bits((v+h)·vScale)<<32`.

  These are **float bits**: the decompiler's `(int)` casts here are wrong (V: no cvt.w).
- XYZ2 = `int(x) | int(y)<<16 | zi<<32`.
- Then x += advance·sx.

**Packet** (n = glyphs found):
- `CNT 2n+4`, `{0, 0, 0, DIRECT 2n+3}`;
- GIFtag `0x2400000000000001`, REGS 0x10 (REGLIST, NREG 2: PRIM, RGBAQ; EOP 0);
- PRIM **0x56** (sprite, TME, ABE; ST);
- RGBAQ = `int(r·204) | int(g·204)<<8 | int(b·204)<<16 | int(a·128)<<24 | 1.0f<<32` (**204**, V: 0x434c0000);
- GIFtag `0x4400000000008000 | n`, REGS 0x5252 (REGLIST, NREG 4: ST, XYZ2, ST, XYZ2);
- 2 qw per glyph.

The header is 0x50 bytes; end = start + 0x50 + 0x20n.

**Always written**, even for n = 0 (NLOOP 0 with EOP).

**81** builds the same bytes in an SPR block. It reads back from RAM after the call. Its 3D/2D code is the same, except that the character is u16.

**Capture:** `Prims{0x56, 2 verts per glyph}`, the same RGBAQ for every vertex.

## 7. Slot 89 `DrawShadowVolume` 0x386e78: the DIRECT segments (V; segment D–F flag logic D)

**Arguments:** a0 gm, a1 `list: *Entry[count]`, a2 count. Entry (0x14) = {rings ptr, first, count, alpha f32, step f32}.

**curState:**
- := default (0x501420);
- w2 bits 5-9 := 3 (**layer 3**);
- type 3, w3 &= 0xc0000000;
- w1: bit 22 set, bits 23-24 := 2, bits 20-21 := 0, bits 12-19 := 0, bits 2-6 := 5;
- +0x10 := short `*(gm + 0xf50 + 4·*(gp+0x14e4))`.

**w** = slot 10 (vt+0x54) result (frame width); FBW = w/64 (signed).

**Packets.** Built through the GsPkt state in the scratchpad (gp+0x2a94). Each segment is its own `CNT` packet laid at `gm+0x5a00` and advances `gm+0x5a00` **without** a record or +0x10. Layout:
`CNT k-1 | {0,0,0,DIRECT d} | GIFtag(NLOOP m, EOP, A+D) | m A+D | [VU part]`.

The VU part is `{0, STCYCL 0x0101, 0, UNPACK V4-32 n=2 addr 4 (no FLG, absolute)}` + 2 qwords → **VU1 qw4 / qw5** (the type-3 header's guard scale / offset).

| Seg | A+D writes | VU qw4 / qw5 | Then |
|---|---|---|---|
| A (CNT 8) | FRAME_1 = drawFbp \| FBW<<16 \| **FBMSK 0xffffff** (alpha only); ZBUF_1 = ZBP \| PSMZ24 \| **ZMSK**; TEST_1 = 0x30000 (ZTE, ZTST ALWAYS) | gm+0x5840, gm+0x5850 | `DrawShadowMeshPass` (ribbons A=0, B=1, untextured, alpha 0, step 0) over all entries |
| B (CNT 4) | TEST_1 = 0x50000 (ZTE, GEQUAL); FBA_1 = 1 | — | MeshPass again |
| C (CNT 8) | ZBUF_1 = ZBP \| PSMZ24 (**z write on**); TEST_1 = 0x3c000 (DATE, DATM=1, ZTE, ALWAYS); FBA_1 = 0 | gm+0x5840 with z := 0; gm+0x5850 with z := 0 | MeshPass again |
| D (CNT 7, or 8 with FBA) | FRAME_1 = drawFbp \| FBW<<16 (normal write); TEST_1 = 0x50000; FBA_1 = 1 if gp+0x14dc ≠ 0 | gm+0x5840, gm+0x5850 | `DrawShadowEdgePass` ring pairs (2,0), (0,3), (3,4), (4,1), (1,5), textured, per-entry alpha / step |
| E (CNT 4, only if gp+0x14dc ≠ 0) | TEST_1 = 0x54000 (DATE, DATM 0, ZTE, GEQUAL); FBA_1 = 0 | — | EdgePass (0,1) |
| F (CNT 3, same condition) | TEST_1 = 0x50000 | — | — |

- gp+0x14dc is 1 in the ELF data.
- The ribbons are `EmitShadowStrips` calls (captured already, each with its own `Recorded`).
- **One record at the end:** start = the original cursor (segment A), end = the current cursor. Then +0x10, and curState type := 0 and reset to the default.

**Capture:** one `Recorded` per segment, start = segment start (the frame build looks up every DMA tag address it walks).
- Command: `Cmd::Gs(sets)`, plus for A / C / D the VU part. Either:
  - add `GsOp`-like `Vu` steps (`Step::Cycle(0x101), Step::Write{addr: 4, rel: false, data}`); or
  - a combined `Cmd::Seq(Vec<Cmd>)`.
- `first_tag = 0x1000_0000 | (k-1)`.

**State later draws depend on** (they are left this way after 89):
- ZBUF_1 has ZMSK **0** with PSMZ24 (hard-coded, not gm+0x5a40);
- TEST_1 = 0x50000;
- FBA_1 = 0;
- FRAME_1 is normal;
- VU1 qw4/5 = gm+0x5840/0x5850.

The DeferMan emitter only rewrites ZBUF / TEST when state bits differ from **the last emitted state** (89's). So the next buckets inherit ZMSK 0 (z writes on) when their bit 22 matches. The renderer's GS model must apply these `Set`s.

## 8. Slot 90 `DrawBoardWake` 0x3883b8 (V)

There is no DIRECT and no VIF of its own.

**Body:**
1. curState := type 3 (w3 &= 0xc0000000);
2. `EA_MemSet(*(gp+0x2a94), 0, 0x30)`;
3. for each i in 1..n-1: `EmitBoardWakeStrips(f12, f13, gm, a1, t0, t1, t2, i-1, i)` (already captured);
4. one record [start, cursor); +0x10;
5. curState type := 0, then reset to the default (0x501420).

**Arguments:** f12 / f13 alpha / step, a0 gm, a1 rings, a2 count (param_5), t0..t2 passed on.

## 9. Slots 53 / 54 / 55: rider-shadow render target (V)

**Sequence** (`Rider_DrawShadow` 0x122898):
1. push state, layer 1;
2. slot 31 (push matrix);
3. **53**(gm, tex = `shadow+0x170`);
4. **55**(gm, tex, &{0, 0, 0, 0});
5. slot 101 header;
6. slot 100 shadow mesh;
7. **54**(gm);
8. slot 32 (pop matrix);
9. pop state.

`EndFrame` also calls **54** (after slot 24 `SetViewportRect`, before `RenderFrame`). Its record is a continuation (+0x1e = 1) of the frame's last record.

T = `*(TexMan+8 + 4·tex)` (TexInfo): TW = TEX0 bits 26-29, TH = bits 30-33, PSM = bits 20-25, +0x28 = VRAM block address.

- **53** `SetTargetTexture` (a0 gm, a1 tex), 0x60 bytes, built by `TexMan_BuildUploadPacket` 0x367db8:
  - `CNT 5 | {0, 0, FLUSH, DIRECT 4}` | GIFtag NLOOP 3 A+D;
  - FRAME_1 = `T+0x28 >> 5 | ((1<<TW)>>6)<<16 | PSM<<24`;
  - SCISSOR_1 = (0, 2^TW - 1, 0, 2^TH - 1);
  - XYOFFSET_1 = ((0x800 - 2^TW/2)·16, (0x800 - 2^TH/2)·16).
  - Record continuation 0.
- **55** `ClearTarget` (a0 gm, a1 tex, a2 `*[f32;4]` a, r, g, b), 0x70 bytes, built by `TexMan_BuildSpritePacket` 0x367f18:
  - `CNT 6 | {0, 0, 0, DIRECT 5}`;
  - GIFtag `0x4400000000000001`, REGS 0x5510 (REGLIST: PRIM, RGBAQ, XYZ2, XYZ2);
  - PRIM 0x10e (sprite, IIP, FST; **untextured, no blend**);
  - RGBAQ = colour·128, Q = 1;
  - XYZ2 (0x800 - w/2)·16, (0x800 - h/2)·16, Z = 1;
  - XYZ2 (w/2 + 0x801)·16, (h/2 + 0x801)·16, Z = 1;
  - GIFtag NLOOP 1, A+D, EOP;
  - SCISSOR_1 = (1, w-2, 1, h-2) (an inset of 1 px, left for slot 100).
  - Record **continuation 1**.
- **54** `RestoreTarget` (a0 gm), 0x70 bytes; the function at 0x37d450..0x37d734 is not defined in Ghidra (generated Rust read):
  - `CNT 6 | {0, 0, FLUSH, DIRECT 5}` | NLOOP 4 A+D;
  - FRAME_1 = `gm+0x5a84 | gm+0x5a50<<16 | gm+0x5a3c<<24`;
  - SCISSOR_1 = SCISSOR_2 = from gm+0x5900 (x, x+w-1, y, y+h-1);
  - XYOFFSET_1 = ((0x800 - fbW/2)<<4, (0x800 - fbH/2)<<4).
  - Record **continuation 1**.

**Proposed variants:**
- `Cmd::Target { tex, frame, scissor, xyoffset }` (53);
- `Cmd::Restore { frame, scissor, xyoffset }` (54);
- `Cmd::Clear { tex, rgba, scissor_after }` (55).

Or `Cmd::Gs` with the TexId alongside.

**Native:** 53 → render pass on the TexId's GPU texture; 55 → LoadOp::Clear with that colour; 54 → back to the main target.

**Dependency:** SCISSOR after 54 comes from gm+0x5900, not from the DeferMan scissor index. The emitter's cached index does not know about it.

## 10. Slot 61 `DrawTransition` 0x37cc98 (V for layout, D for arguments)

**Arguments:** a0 gm, a1 `t` (t+4 = TexMan id).

**Caller:** `cTransitionMan_Render` 0x2e47e8, once when its capture flag (+0x64) is set. Then it draws the overlay itself (layer 8, slot 71).

**State:**
- pushes curState (copy) and clears the type;
- w3 &= 0xc0000000;
- records with that state, then pops;
- sets `gm+0xf44 = 1` (force display flip).

**Packet** (SPR, 0x140 bytes): `CNT 0x13 | {0, 0, 0, DIRECT 0x12}` | GIFtag NLOOP 17 A+D, EOP, then in order:

1. FRAME_1 = `T+0x28 / 32 | 4<<16` (FBW 4 = 256 px, CT32);
2. ZBUF_1 = ZBP \| PSMZ24 \| ZMSK;
3. SCISSOR_1 = (0, 255, 0, 255);
4. TEST_1 = 0x30000;
5. XYOFFSET_1 = 0;
6. ALPHA_1 = 0x2a (Cd = Cs, FIX 0);
7. TEXA = 0x80_00000080;
8. TEX1_1 = 0;
9. TEX0_1 = `dispFbp<<5 | dispFbw<<14 | TW=TH=10 | CT32 | MODULATE` (source = **the displayed buffer**, gm+0x5a88);
10. PRIM = 0x116 (sprite, TME, FST UV);
11. RGBAQ = 0x3f800000_80808080;
12. UV (8, 8);
13. XYZ2 (0, 0, 0xffff);
14. UV (dispW·16 - 8, dispH·16 - 8);
15. XYZ2 (0x1000, 0x1000, 0xffff);
16. TEXFLUSH;
17. SCISSOR_1 = gm+0x5900 rect.

**Native:** `Cmd::ScreenCopy { dst: tex, src_fbp: dispFbp, src_size: (dispW, dispH), dst_size: 256 }` (a downscaling copy).

**Left behind:**
- FRAME_1 = the 256² texture;
- XYOFFSET_1 = 0;
- ZBUF_1 has ZMSK on;
- TEST_1, ALPHA_1, TEX0_1, TEX1_1 and TEXA are clobbered.

**Nothing in this function restores FRAME / XYOFFSET.** Later buckets in that frame draw into the texture until a FRAME write (slot 54 in EndFrame, post passes). Whether that happens in play is **open (I)**; check on a transition frame dump. So capture the register writes (`GsOp::Set`) too, not only the copy.

## 11. Slot 87 `DrawCircle` 0x382170 (D; debug)

**Arguments:** f12 radius, a0 gm, a1 `centre: *vec4`, a2 colour, a3 step (≤ 0 → 4, 1 or 2 by view depth).

**Body:**
1. centre := slot 35 view matrix · centre;
2. if beyond gp-0x2758, nothing;
3. otherwise up to 17 vertices = centre + radius·unitCircle[k] (static table 0x4ffbc0, built once; gp+0x28b0 flag);
4. RGBA = colour·128 as u32s (a, r, g, b);
5. slot 31 push, slot 37 identity, **slot 73**(gm, k+2, verts), slot 32 pop.

It is captured by the slot 73 wrapper. It needs nothing of its own.

## 12. Type-3 entry 0x2400 (V)

- MSCAL 0x2400 is the static qword **0x509400** = `{0, 0, 0, MSCAL 0x2400}` (0x14000480), built by `World_StaticInitDestroy` 0x3ad290. Next to it: 0x509410 = MSCAL 0x2270 and 0x509420 = MSCAL 0x22c8.
- `cPS2ChunkResolve_PatchDmaChain` 0x3ac048 and `PatchInstanceDmaChain` 0x3ac508 write `REF 0x509400` (`0x0050940030000001`) into **world mesh DMA chains at load**. They do it for meshes flagged two-pass (bit 31 of the REF address), between pass 1 (MSCAL 0x2270 / 0x22c8) and pass 2.
- So it is reached only through slot 95/96 mesh CALL chains. It kicks the ctx-2 strip tag at VU qw 0x1f.
- It belongs in the mesh capture (slot 95): one `Step::Start(0x2400)` with no input.

## 13. Capture summary

| Fn | Packet(s) per call | first_tag (u128) | Record | Command |
|---|---|---|---|---|
| 72 | 1 | `0x1000_0000 \| (3n+4)` / `(9n/2+(n&1)+4)` | yes | Gs: 1 Prims |
| 73 | 0-1 | `0x1000_0000 \| (qwc-1)` | if any vertex | Gs: k line strips |
| 75 | 1 | 0x1000_000a | yes | Vu: write 8 @TOP, start 0x3bf0 |
| 78 | 0-1 (cull) | 0x1000_0005 | if drawn | Gs: sprite |
| 79 | 1 (may be empty) | 0x1000_0007 | yes | Gs: count × strip quad |
| 80 / 81 | 1 | `0x1000_0000 \| (2n+4)` | yes | Gs: n sprites |
| 53 / 54 / 55 | 1 | 0x1000_0005 / 6 / 6 | yes (54 and 55 continuation) | Target / Restore / Clear |
| 61 | 1 (SPR) | 0x1000_0013 | yes | ScreenCopy + Sets |
| 89 | 4-6 DIRECT segments + ribbons | per segment | one for all | Gs + VU qw4/5 per segment |
| 90 | ribbons only | — | one | (ribbons) |

**Common rules:**
- Key by start. Copy everything at capture (VifBuff halves are reused two frames later).
- Skip the capture if the body unwound (`c.pc != ra`, as in `capture.rs`).
- None of these functions reads game memory after it returns, and none rewrites its packet before EndFrame.
