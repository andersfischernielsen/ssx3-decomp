# N2 capture spec: particles and trails

Area: slots 82 `DrawEmitter`, 83 `DrawDynamicEmitter`, 84 `DrawMEmitter`,
85 / 86 `DrawTrail(B)` (SnowBox), `PS2BezierMan_DrawPatchFX` (via
`RenderPatchFX`), and `cPS2FogParticleMan_DrawParticles`.
V = checked in the disassembly, D = read from the decompiler only,
I = inferred.

## 0. Common facts (all functions here)

- **Globals** (gp = 0x4a30f0): gp+0x1384 = the SprStream object (0x4a4474),
  gp+0x2a90 = the PS2GraphicsMan `gm` (0x4a5b80). Debug kill flags: gp+0x1408
  (slot 82), gp+0x140c (83 and 84), gp+0x13f0 (`RenderPatchFX`); when one is
  non-zero the function returns at once, so there is no packet and no record (V).
  85 / 86 and the fog have no such flag (V).
- **SPR stream** (V): `SprStream_BeginBlock(stream, dest = *cursor, -1, 0)`
  returns a scratchpad block. The body fills it.
  `SprStream_EndBlockAligned(stream, p, 4)` rounds the size up to 16 and DMAs
  the block from SPR to `dest`. It returns `end = dest + size`. In this runtime
  the fromSPR DMA is **synchronous**: the CHCR store inside
  `Dma_FromScratchpad` runs `Hw::dma_start` to completion
  (`crates/ssx3-ps2/src/hw.rs` `FROM_SPR`, `crates/ssx3-game/src/util.rs`
  `spr_transfer`) (V in code). So once the body has returned, the packet is in
  RAM at `start..end`, and the capture can read it back.
- **Cursor and record** (V, the same code in every function): `start = *cursor`
  before `BeginBlock`, and `*cursor = end` after it. If DeferMan count
  `*(gm+0x18f0) < 0xa28`, it writes record {curState (0x14 B, w2 low 5 bits :=
  scissor `dm+0x698c4 & 0x1f`), +0x14 start, +0x18 end, +0x1c short matrix index
  `dm+0x698c0`, +0x1e 0}. Then `*cursor = end + 0x10` (the slot for the bucket
  link tag). With 2600 records there is no record, but the packet and the
  cursor advance still happen, and the packet is never CALLed. **One record per
  batch:** `Recorded { start, end, first_tag, cmd }` per batch, where `end` is
  the record's end (= `*cursor - 0x10` after that batch). The next batch starts
  at `end + 0x10`.
- **Packet shape** (VU1 kinds, V): `qw0 = CNT qwc` (VIF NOP, NOP) → the u128
  `first_tag = 0x1000_0000 | qwc`; `qw1 = {NOP, NOP, NOP, UNPACK V4-32 n at 0,
  FLG}` (`0x6c..8000 | n<<16`: address += TOPS, no mask, USN 0); qw2.. = n data
  qwords → VU1 TOP+0..n-1; ...; last qw `{NOP, MSCAL entry/8, NOP, FLUSH}`.
  The packets contain no STCYCL, STMOD or STMASK. They rely on CL = WL and
  STMOD 0 left by the type's state change (the static `.vutext` chain sets
  STCYCL 4,4 and STMOD 0, `structs/render.md`) (I). No REF tags: all data is
  copied into the packet at call time, and nothing is read by DMA later.
- **Every VU1 kind makes one program start per record.** TOP comes from the
  VIF double buffer (type 4: BASE 7 / OFFSET 0x9a; type 5: BASE 3 / OFFSET
  0xb, `phase3-render.md` §1).
- None of these addresses is in `crates/ssx3-game/overrides.txt` yet. Each
  wrapper needs its slot listed (as for 0x379860).

## 1. Slot 82 `PS2GraphicsMan_v82` DrawEmitter 0x380518 (V)

- Args: a0 = gm, a1 = desc (21 qw, 16-aligned: `lq`). Callers:
  `Emitter_Render`, `cBoardSparkFX_Render`.
- State: it **pushes** curState: it copies 0x14 B to +0x14 and does
  gm+0xe84 += 0x14. On the new state it sets: type 4 (w0 bits 6-9 = 4), w3 &=
  0xc0000000, sort mode 2 (w1 low 2 bits), sort value 0 (w2 &= 0xe00003ff),
  and layer 7 (w2 bits 5-9). After the record it pops (e84 -= 0x14).
  `UpdateViewMatrices(gm)` runs if gm+0x6b90 == 0, after EndBlock and before the
  record (it does not touch 0x5a00).
- Packet (cursor gm+0x5a00), 0x180 B, `first_tag 0x10000017`:
  `UNPACK V4-32 21 at 0` (0x6c158000) + desc[0..21] verbatim → **TOP+0..20 =
  D0..D20**; `MSCAL 0x0000`.
- Input source: `desc + 16·k`, k < 21, read at call time (the body does not
  change it).

## 2. Slot 83 `PS2GraphicsMan_v83` DrawDynamicEmitter 0x3807a0 (V)

- Args: a0 = gm, a1 = e (DynamicEmitter). Caller `DynamicEmitter_Render`.
- Emitter fields: e+0x20..0x16f descriptor (21 qw, D0..D20; D0.x = int total
  particle count, D0.w = f32 age step); e+0x178 `n` = ring points; e+0x17c
  `head`; e+0x1a0 → `pos[n]` (16 B each); e+0x1a4 → `dir[n]` (16 B, .w = the
  segment's LFSR seed).
- State: type 4, w3 &= 0xc0000000 on the **current** state (no push, no layer
  or sort change: the layer is the caller's). At the end it clears the type
  bits and w3.
- `per = D0.x / n` (int div; n == 0 → `break 7`). `perf = (f32)per`.
  `batches = n/64 + 1` (rounding toward 0 as `(n<0 ? n+63 : n) >> 6`, + 1).
  **When n % 64 == 0 the last batch has c = 0 and is still emitted.**
- Per batch b (done = particles before it, starting at 0):
  - `c = min(n - done, 64)`; `h1 = head + 1`; `i1 = (done + h1) % n`;
    `done' = done + c`
  - `first = min(c, n - i1)`; `wrap = max(0, i1 + c - n)`
  - `next = (done' < n ? done' + h1 : done' - 1 + h1) % n`
  - D21 = `{c (i32), per (i32), perf, (perf · (f32)(b·64)) · D0.w}`, a PS2 mul
    in this order (V: `mul.S f0,f8,f2` then `·f1`).
- Packet, 0x9b0 B, `first_tag 0x1000009a`: `UNPACK V4-32 152 at 0`
  (0x6c988000), `MSCAL 0x510`. VU1 input (offset from TOP, 152 qw):

  | TOP+ | contents |
  |---|---|
  | 0..20 | e+0x20.. (D0..D20) |
  | 21 | D21 above (stack qword) |
  | 22..22+c-1 | `pos[i1..]` for `first` points, then `pos[0..]` for `wrap` points |
  | 22+c | `pos[next]` |
  | 23+c..86 | **not written**: stale SPR block contents |
  | 87..87+c-1 | `dir[i1..]` for `first` points, then `dir[0..]` for `wrap` points |
  | 87+c | `dir[next]` |
  | 88+c..151 | **not written**: stale SPR |

## 3. Slot 84 `PS2GraphicsMan_v84` DrawMEmitter 0x380ce0 (V; ring indices D, the same code as 83)

- Args: a0 = gm, a1 = e (mEmitter). Caller `mEmitter_Render`. The fields are
  those of 83, plus e+0x200 (0 → entry **0xa00**, otherwise **0xf10**) and
  e+0x204 → `col[n]` (u32 RGBA bytes).
- State: as 83.
- `per = D0.x / n`, `batches = n/43 + 1` (signed div by 0x2b). Batches are 43
  points; the same `c / i1 / first / wrap / next` with 43 in place of 64. D21.w
  = `(perf · (f32)(b·43)) · D0.w`.
- Packet, 0x7d0 B, `first_tag 0x1000007c`:
  1. `UNPACK V4-32 110 at 0` (0x6c6e8000), then:

     | TOP+ | contents |
     |---|---|
     | 0..21 | D0..D20, D21 (as 83) |
     | 22..22+c | `pos[i1..]`, `pos[0..]` (wrap), `pos[next]` |
     | 23+c..65 | stale SPR |
     | 66..66+c | `dir[...]`, the same way, ending with `dir[next]` |
     | 67+c..109 | stale SPR |

  2. qword `{NOP, NOP, NOP, UNPACK V4-8 44 at 110, FLG, USN}` (0x6e2cc06e), then
     11 qw = 44 u32 colours. Each byte is zero-extended to an int lane →
     **TOP+110..153**: `col[i1..]` for `first`, `col[0..]` for `wrap`,
     `col[next]` at 110+c. Slots 111+c..153 are **stale stack** (a
     44-word stack array that the body never initialises; the 44-iteration loop
     before the copy is an empty constructor loop, V).
  3. `MSCAL 0xa00 | 0xf10`.

## 4. The ring's off-by-one: what exactness needs (83 / 84)

`mat4::ring_particles` runs `D21.x + 1 = c + 1` segments, k = 0..c, and
segment k reads P[k], P[k+1], E[k], E[k+1] (and col[k], col[k+1] for
0xa00 / 0xf10). Segment c is drawn whenever `E[c].w ≥ 1`, which holds for
real points. It reads **P[c+1], E[c+1], col[c+1]**:
- for c below the maximum these are the stale SPR / stack slots above. They are
  in the packet, so **read them back from RAM** (packet qword index =
  2 + TOP offset; for 84's colours, byte `start + 16·113 + 4·k`);
- for a full batch (83 c = 64: P[65] = TOP+87 = E[0], **E[65] = TOP+152**;
  84 c = 43: P[44] = TOP+66, E[44] = TOP+110 (= col[0] as ints),
  **col[44] = TOP+154**), the bold qwords lie outside the upload. They are
  whatever VU1 memory held there from earlier draws: type 4 TOP is 7 or 0xa1,
  so TOP+152 is the input buffer's last two slots (last written by an 84 colour
  upload), TOP+154 is 0xa1 (the other buffer's D0) or 0x13b (output packet A's
  tag). No capture can know them. Exactness there needs the renderer's
  VU1 memory image at the start (see §8).
- A c = 0 batch (n a multiple of 64 or 43) still draws one segment:
  P[0] = `pos[next]` = the last point again, with P[1] / E[1] stale.

So the capture records the **whole VU input image** per batch, gaps included.

## 5. Slots 85 / 86 `PS2GraphicsMan_v85 / v86` DrawTrail / DrawTrailB 0x381310 / 0x3816f0 (V)

- The two bodies are byte-identical except for the MSCAL (V by diff): 85 →
  **0x0000** (CLIP cull), 86 → **0x0408** (depth fade). Args: a0 = gm, a1 = d
  (SnowBox descriptor). Caller `SnowBox_Render`. No kill flag.
- State (current state, no push): type 5, w3 &= 0xc0000000, sort mode 2, sort
  value 0. Everything is cleared again at the end (sort mode, sort value, type,
  w3).
- `UpdateViewMatrices(gm)` runs first if gm+0x6b90 == 0. Then on VU0 (macro
  mode, V): `T_j = G·D_j` for j = 0..3, where G = the columns at gm+0x5860..
  (viewProj), D_j = d+0x50+16j, computed as `ACC = G0·Dj.x; ACC += G1·Dj.y;
  ACC += G2·Dj.z; T = ACC + G3·Dj.w` (PS2 FMAC).
- Packet, 0xe0 B, `first_tag 0x1000000d`: `UNPACK V4-32 11 at 0` (0x6c0b8000),
  `MSCAL 0 | 0x408`. VU1 input:

  | TOP+ | contents (`mat5` names) |
  |---|---|
  | 0..3 | T0..T3 = viewProj × box matrix (above) |
  | 4 | {0, 0, 0, 0} (STQ corner A) |
  | 5 | {1.0, 1.0, 0, 0} (STQ corner B) |
  | 6..10 | d+0x00..0x4f verbatim (box offset, colour, {count, seeds}, min / size, max / depth scale) |

- Capture: d+0x00..0x8f and gm+0x5860..0x589f **after** the body (it may
  refresh viewProj). Either recompute T0..T3 with the PS2 FMAC helpers
  (`mat4::mul / madd / bc`), or read packet qw2..5 back.

## 6. `PS2BezierMan_DrawPatchFX` 0x38d690 (V), called from `RenderPatchFX` 0x38d968 (D)

- `RenderPatchFX(bm a0, cursor a1)`: the caller is `PS2BezierMan_v18` with
  cursor = **&gm+0x5a00** (V: v18 `sp+0x230 = gm+0x5a00`). It skips everything
  when gp+0x13f0 != 0. It **pushes** curState and sets on the new one: type 4,
  sort mode 2, sort value 0, **layer 4** (w2 bits 5-9 = 4), blend/test bits
  (w1 |= 0x54, 0x800000, 0x300000, 0x5c000, 0x400000), and tex (+0x10 short) =
  `gm+0xf50[gm+0x64]`. It pops at the end. Two lists:
  - bm+0x4a60 count, entries bm+0x4a64 stride 0x10 (uniform LOD 8);
  - bm+0x4d84 count, entries bm+0x4d88 stride 0x14 (mixed), only those with
    entry[2] > 0.

  Per entry: `p = SprStream_Upload(entry[0] /*patch*/, 0x1b0, 2)` (the patch
  copied into SPR; this uses a stream slot), then `DrawPatchFX(bm, p, entry,
  cursor)`.
- `DrawPatchFX(a0 bm, a1 p (SPR copy of the patch), a2 entry, a3 cursor)`:
  - nothing unless `p+0xc & 0x800000`;
  - `count = cvt.w( (((dx·dx + dy·dy) + dz·dz) · (gp[0x1258]·gp[-0x26b4])) ·
    gm[0xc4] )` (floats), with d = p+0x158.. − p+0x164.. (xyz); if
    `entry[2] < 2`, count /= 2 (rounding toward 0). Nothing if count ≤ 0.
  - it writes `bm+0x43c = count`.
- Packet, 0x190 B, `first_tag 0x10000018`: `UNPACK V4-32 22 at 0` (0x6c168000),
  `MSCAL 0x1460`. VU1 input:

  | TOP+ | source |
  |---|---|
  | 0..15 | p+0x40..0x13f = patch+0x40 (power-basis coefficients; RAM `entry[0]+0x40` is the same) |
  | 16..19 | bm+0x3e0..0x41f (built by v18 earlier in the same call, e.g. copied from 0x4ff1a0 at 0x38b838; I: constant during RenderPatchFX) |
  | 20..21 | bm+0x420..0x43f; 21.w = count |

- Capture: wrap `DrawPatchFX` (a jal target, not a vtable slot). Read the 22
  qw **after** the body, because bm+0x43c is written by it. One record per drawn
  patch.

## 7. `cPS2FogParticleMan_DrawParticles` 0x2dc7b0 (V): GIF, no VU program

- Arg: a0 = fog manager `f`. `f+0` = count, particles at `f+0x10 + 0x30·idx`
  (12 f32: [0..2] x0, y0, z0; [3] w; [4..6] x1, y1, z1; [8] A, [9] R, [10] G,
  [11] B), draw order list at **0x537ac0** (8 B entries, word 0 = idx; I:
  sorted by the manager's update). gm = `*(gp+0x2a90)`, cursor gm+0x5a00. It
  does not touch curState: the record uses the caller's state
  (`GameModule_RenderFogParticles`).
- Batch size: `B = (((S<<4)>>4) − 4)·2/5 − 1`, where S = stream+0x10 (the SPR
  block size in qw). Batches of `k = min(left, B)`, consuming the order list in
  sequence. Each batch makes one block and one record.
- Packet (qD = (5k>>1) + 3 + (k odd)), `first_tag = 0x1000_0000 | (qD+1)`:
  qw1 `{NOP, NOP, NOP, DIRECT qD}`, then the GIF data:
  - `GIFtag NLOOP 1 EOP REGLIST NREG 1 (PRIM)` (0x14000000_00008001), `{PRIM
    0x56 (SPRITE TME ABE), pad}`;
  - `GIFtag NLOOP k EOP REGLIST NREG 5`, REGS 0x52512 = ST, RGBAQ, XYZ2, ST,
    XYZ2;
  - per sprite, 5 dwords (w = p[3]); `cvt` = `cvt.w.s`, and every 32-bit value
    is **sign-extended to 64 bits before the ORs** (mfc1 / lw):
    1. ST = `bits(0.0·w) | bits(0.0·w)<<32`
    2. RGBAQ = `cvt(R) | cvt(G)<<8 | cvt(B)<<16 | cvt(A)<<24 | bits(w)<<32`
    3. XYZ2 = `cvt(x0) | cvt(y0)<<16 | cvt(z0·w)<<32`
    4. ST = `bits(1.0·w) | bits(1.0·w)<<32`
    5. XYZ2 = `cvt(x1) | cvt(y1)<<16 | cvt(z1·w)<<32`

    (With unmasked ORs, an out-of-range or negative lane bleeds into the next
    one, and w < 0 makes ST's low word −0.0, which sets T to all ones. A
    literal port must reproduce that.)
  - an odd k adds one zero pad dword.
- Capture: compute the sprites from `f` and the list at call time (nothing
  changes them during the body), or read the GIF data back from the packet.

## 8. Proposed commands

```rust
/// Types 4 / 5 (slots 82-86, DrawPatchFX): one program start.
Particles {
    program: u8,      // 4 (mat4) or 5 (mat5)
    entry: u32,       // 0, 0x510, 0xa00, 0xf10, 0x1460 | 0, 0x408
    /// VU1 input from TOP+0, as VU memory holds it after the UNPACKs
    /// (84's V4-8 colours already widened to one u32 lane per byte).
    input: Vec<u128>, // 21 | 152 | 154 | 22 | 11 qw
},
/// cPS2FogParticleMan: a GIF DIRECT sprite list (PRIM 0x56), no VU.
FogSprites { sprites: Vec<[u64; 5]> }, // ST, RGBAQ, XYZ2, ST, XYZ2 as sent
```

- `starts()`: 1 for `Particles`, 0 for `FogSprites`.
- **Recommended fill for stage A:** run the body, then read the input back from
  the packet in RAM (`start + 0x20 + 16·i`; 84: the first 110 qw from there,
  the 44 colours from byte `start + 0x710`, widened). This is exact by
  construction (gaps, PS2 float math of 83/84 D21 and 85 T0-3) and cheap (≤ 2.4
  KB per batch). The source maps in §1-§7 are what a native producer (and
  `check_capture`) computes. The source-computed qwords must equal the
  readback everywhere except the stale slots.
- Wrapper pattern as `capture.rs`: before the body, check the kill flag and
  read `start = *cursor`. After it, require `c.pc == ret`. Then walk the
  batches: `end_i = start_i + 16·(qwc_i + 1)`, `start_{i+1} = end_i + 0x10`;
  the final `*cursor` must equal `end_last + 0x10`. Check `RAM[start_i]` ==
  `first_tag_i` (or derive qwc from it: 83 / 84 / fog batch counts come from
  the replicated loop).
- **Renderer plan:** copy the VU1 image, write the header and `input` at TOP
  (from `direct::tops(regs, 1)`), run `mat4::run` / `mat5::run` with `entry`,
  and expand the XGKICKed SPRITE packets to quads as now. Needed:
  - **header qw0-6 for type 4** (qw6 = {viewProj[0].x, viewProj[1].y}, `PROJ`);
    `Event::Native` currently carries `[u128; 6]`, which is one short.
    Type 5 reads qw0-2.
  - for exact full ring batches, the VU1 qwords at TOP+152 (83) and TOP+154
    (84) as left by earlier draws (§4). Also, `Event::Native` does not apply the
    captured upload to `w.vif.mem`. Draws that are still decoded and read stale
    type-4 memory (the same two qwords) would see different data unless the
    frame build applies `input` to its VU image when it skips the packet.
  - `mat4::run` declines `0x1460` with count 0 (DrawPatchFX never sends it),
    and `mat5` declines a flake count of 0: fall back to the translated program
    on the image.
  - `FogSprites`: draw directly as textured sprites (STQ = (0,0,w)/(w,w,w) →
    perspective UV 0..1). The texture, blend and ZTEST come from the bucket
    state (the caller's curState).

## 9. Race-frame cross-check

Decoded race payloads: type 4 entry 0 at layer 7 = slot 82 (it forces layer
7); 0xa00 at layer 7, about 2 KB of input = slot 84 (124-qw packet, the caller's
state at layer 7); 0x1460 at layer 4 = DrawPatchFX (RenderPatchFX forces layer
4). These agree with the above.
