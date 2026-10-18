# N2 capture spec: models (type-3 mesh program)

Area: model draws through VU1 program `d159716bb1281fbe` (`crates/ssx3-render/src/mat3.rs`), mesh entries
0x2270 / 0x22c8 (pass 1 with / without setup, pass 2 when ITOP != 0), 0x2320 / 0x2358 (setup / no setup, ITOP ignored), 0x2400 (PRIM-only kick).

Producers (V: Ghidra xrefs to `EmitModelHeader` 0x37e120 are exactly these two):
- slot 95 `PS2GraphicsMan_v95` "DrawModel" 0x37e238 (reached directly as vt+0x2fc, and through slot 96);
- `cPS2GridMesh_Draw` 0x385c10 (grid-mesh vt 0x493208 slot 8).

Nothing else builds a packet that CALLs a model mesh chain with this program. Riders (slot 99) CALL their own prebuilt chains but use program 6 (out of scope).

Marks: **V** = verified in the disassembly, generated Rust or the RAM dump `ram_gen.bin` (scratch); **D** = decompiler reading, not re-checked instruction by instruction; **I** = inferred.

Conventions:
- `gm` = PS2GraphicsMan; `cur` = `*(gm+0xe84)`, the current RenderState (5 words w0..w4, w4 = tex0 | tex1 << 16);
- `defer` = `*(gm+0x18f0)`;
- `S` = `*(gp+0x2a94)` = **0x70003800** (scratchpad, V from the dump; gp = 0x4a30f0);
- matrices are 4 qw of rows, row-vector convention: `A × B` = `out_r = A_r.x·B0 + A_r.y·B1 + A_r.z·B2 + A_r.w·B3` (the `vmulax/vmadday/vmaddaz/vmaddw` pattern, V).

---

## 1. Slot 96 DrawModelSimple 0x369cb0 (V, overridden in `crates/ssx3-game/src/graphics.rs`)

`(a0 gm, a1 inst, a2 x, a3 y, t0 flags)` → slot 95 with:
- `a2 = *(inst+0x94)`, `a3 = inst+0x10`;
- `t0 = x`, `t1 = y`, `t2 = flags`;
- `f12 = 1.0`.

The callers pass x = y = 0. `GameModule_RenderModels` passes flags 0xc00 (one view) or 0x400 (split screen), with `| 0x20` for its second list (`crates/ssx3-game/src/world.rs`).

Wrapping the slot-95 implementation address catches slot 96 too: its override calls through `call_ptr`.

## 2. Slot 95 DrawModel 0x37e238

### 2.1 Arguments (V, prologue 0x37e238..0x37e290)

| reg | meaning |
|---|---|
| f12 | alpha. 0 → return; == 1.0 → the opaque paths; otherwise the 2-record path |
| a0 | gm |
| a1 | instance |
| a2 | texture table. Entry k = `*(a2 + 4 + 4k)` |
| a3 | world matrix (4 qw) |
| t0 | per-node matrices (4 qw per node), or NULL = build the hierarchy |
| t1 | "matrix 2" (4 qw), or NULL |
| t2 | flags (below) |

Globals read:
- gp+0x1400 (≠ 0 → return);
- gp+0x1404 (≠ 0 → material flags `&= ~0x600000`);
- gp+0x2a94 (S).

Both flags are 0 in the dump (V).

**Flags t2.** All the bits reach VU1 in the header flags word (§2.4). mat3 reads bits 0-9 only (I, `mat3::flag`).
- 0x1: lit. Also forced on when `inst+8 & 0x4000` (V).
- 0x2-0x1e: pass-1 texgen.
- 0x20: clip. Set by the cull when the box straddles (V).
- 0x40-0x3c0: pass-2 texgen. 0x100 is or'd in for some blend modes (§2.5).
- 0x400: skip the instance cull (V).
- 0x800: "base-state fast path allowed" (`(t2 >> 11) & 1`, V).
- 0x1000: per-node cull. Only together with 0x20 and when the node count > 1 (D).

### 2.2 Prologue (V unless noted)

1. If `gm+0x6b90 == 0`: `PS2GraphicsMan_UpdateViewMatrices` (0x37d968). This interns a MatrixRecord, so the matrix index `defer+0x69cc0` can change.
2. Unless t2 & 0x400: `ClipTestBox` (slot 93, vt+0x2ec → 0x37de88) on the box `inst+0x60..+0x68` / `+0x6c..+0x74` (w = 1).
   - The test runs against `gm+0x58a0` (worldClip). Result 1 → return. Result 2 → test again against `gm+0x5800` (worldView).
   - A non-zero final result sets t2 |= 0x20.
3. If `inst+8 & 0x4000`: t2 |= 1, `light = WorldLightMan_LightInstanceCB(inst)` (0x2f5148). Otherwise `light = 0`.
   - The callback returns a pointer to 10 qw SH coefficients in a WorldLightMan per-instance cache entry (0x180 B, `WorldLightMan_GetInstanceLighting` 0x2f5400, D).
   - **It does not touch any chain data.**
4. `desc = *(inst+0x80)`. The chain base is `base = lit ? *(desc+0x24) : *(inst+0x98)`.
5. `cur.w0 = cur.w0 & ~0x3c0 | 0xc0` (type 3). `cur.w3 &= 0xc0000000`.
6. Scratch (S = 0x70003800, qw indices):
   - `S[4] = (desc+0x18, +0x1c, +0x20) · s, w = 1`, where s = `*(float*)(inst+0x84)`. This is the "scale" vector.
   - `S[0x6c..0x6f] = W` (copy of a3).
   - If `inst+0x92 & 1`: slot 94 (vt+0x2f4 → 0x36a2c0) rewrites `S[0x6c..]` in place.
   - `S[0x70..0x73] = S[0x6c..]`.
7. If t0 == NULL:
   - `S[0..3] = rows of S[0x70]` each multiplied lane-wise by `S[4]`: `S[r] = S[0x70+r] * S[4].{x,y,z,w}[r]` (`vmulx/y/z/w`, V). That is diag(scale) × W.
   - **Single-node shortcut:** if node count < 2 and `nodes[0].matrix == 0x4ff1a0` (an identity in RAM): no per-node matrices. If lit, `S[0x68..0x6b] = S[0x6c..]`.
   - Otherwise, build `S[8+4i..]` per node i. `nodes = *(desc+8)`, 0x10 each: +0 parent (−1 = root), +4 mesh list, +0xc local matrix ptr.
     - Let L = local with translation row xyz × s (w unchanged).
     - root: `S[8+4i] = L`;
     - child: `S[8+4i] = L × S[8+4·parent]` (D: loop at 0x37e5..0x37e82x).
     - Parents must precede children.
   - The per-node pointer then becomes `S+0x80`.
8. `S[0x74..0x77] = *slot35()` (vt+0x11c → 0x395c68 = `GetMatrix`: `*(gm+0x13e4)`, the current matrix-stack top; V generated code).
9. **Environment matrix** (only if `desc+0x10 & 2`; D, 0x37e884..0x37ec9c):
   1. c = box centre × S[0x74].
   2. Build a look-at style rotation R from (0,0,0,1) − c (two normalisations, RSQRT).
   3. `S[0x74] = R × S[0x74]`, then `S[0x78] = S[0x74] × S[0x70]` with the translation row set to (0,0,0,1).
   4. `S[0x7c..0x7f] = K × S[0x78]`, where `K` = the matrix at 0x504760 = rows (0.5,0,0,0), (0,−0.5,0,0), (0,0,0,0), (0.5,0.5,1,1) (V dump: normal → UV bias).
   5. `extra = 0x100` (pass-2 texgen "normal → UV"). Otherwise extra = 0.

   The per-node loop recomputes `S[0x78]` / `S[0x7c]` the same way for each node (D). **The capture should read matrix 2 from the packet, not recompute it.**
10. If `desc+0x10 & 8` (depth-sorted translucent model): **push** the state (copy 0x14 B to cur+0x14, gm+0xe84 += 0x14). On the new state: w2 layer = 7 (`w2 & ~0x3e0 | 0xe0`), w1 |= 0x400000 (ZMSK). Popped at the very end (V 0x37eca0..).

### 2.3 Per node, per strip group (V offsets 0x37f240..0x37f2a0)

Node i is drawn when `ml = node.+4` is non-NULL and `*(ml+0x1c) != 0`.

**Mesh list** (V):
- +0x00..0x0b: box min;
- +0x0c..0x17: box max;
- +0x1c: group count;
- +0x20: → array of group pointers.

(`n2-render-layer.md` §2.1 has "+8 → {+4,+8}": the real offsets are +0x1c / +0x20.)

Per node, when a node matrix exists (t0 or S+0x80):
- `S[0x70] = N_i × S[0x6c]` (node × world);
- if lit, `S[0x68..0x6b] = S[0x70..]`;
- `S[0..3] = diag(S[4]) × S[0x70]` (as above).

Optional per-node cull: if node count > 1 and `(t2 & 0x1020) == 0x1020`, ClipTestBox of `ml` box under `S[0x70] × gm+0x58a0`. Result 1 → skip the node (D).

**Group** g = `groups[j]`:
- +0 short material index;
- +2 short flags (bit 3 → material flags |= 0x40000);
- +4 int chain offset.

**Texture entry** e = `*(a2 + 4 + 4·material)`:
- +0 short tex0, +2 short tex1;
- +0xc material flags m.

1. `cur+0x10 = e.tex0`, `cur+0x12 = e.tex1`.

   **Not restored** unless flag 8 pushed a state, in which case the pop discards them.
2. Clamp `m & 0x180000`:
   - 0x80000 → w0 bits 2-3 = 2;
   - 0x100000 → 1;
   - 0x180000 → 3;
   - else 0.

   Any non-zero clamp clears "fast" (fast = t2 bit 11).
3. Blend and state (below). The result is `(base, mat2, flags)`:
   - `base` = a base-state pointer for the fast path, or 0;
   - `mat2` = the pointer passed as matrix 2: t1, or `S+0x7c0`;
   - `flags` = t2 or t2 | extra.

**Blend table.** `b = m & 0x660000`. The checks run in order; the first match wins.

All of them write cur w1 fields: blend1 (bits 2-6), blend2 (7-11), sort (0-1), aref (12-19), atst (20-21), ztst (23-24), and clear w2's sort value (bits 10-28) unless sort = 2.

| condition | fast & base | state written (when not fast) | mat2 / flags |
|---|---|---|---|
| desc+0x10 & 8 | — | blend 7/3, sort 2 + value, ztst 1, atst 3, aref 0x14 | t1 / t2 |
| b == 0 | `*(defer+0x57488)` | blend 1/3, sort 0, ztst 1, atst 0, aref 0x14 | t1 / t2 |
| b == 0x20000 | `*(defer+0x57490)` | blend 5/3, sort 1, ztst 1, atst 3, aref 0x5c | t1 / t2 |
| b == 0x200000 | `*(defer+0x5748c)` | blend 1/2, sort 0, ztst 1, atst 3, aref 0x14 | S[0x7c] / t2\|extra |
| b == 0x600000 | `*(defer+0x57488)` | blend 1/17, sort 0, ztst 1, atst 0, aref 0x14 | S[0x7c] / t2\|extra |
| b ∈ {0x40000, 0x60000} | — | blend 5/3, sort 2 + value, ztst 1, atst 3, aref 0x14 | **t1 / t2** (V: no `lw s2,0x184(sp)`) |
| b ∈ {0x220000, 0x260000} | — | blend 5/2, sort 2 + value, … | S[0x7c] / t2\|extra |
| b ∈ {0x620000, 0x660000} | — | blend 5/17, sort 2 + value, … | S[0x7c] / t2\|extra |
| other | — | blend 5/3, sort 2 + value, … | S[0x7c] / t2\|extra |

Notes on the table:
- **Fast rows.** On the fast path (`fast` && clamp == 0) the cur fields are **not** written, and `base` is the word at the given defer offset.
- **The 5 `S[0x7c]` sites (V):** 0x37f8a4 / 0x37f970 / 0x37fa20 / 0x37fb5c / 0x37fc94. Matrix 2 is then **always emitted** (S+0x7c0 is never NULL). If `desc+0x10 & 2` is clear, `S[0x7c..]` holds stale scratch, and the flags carry no 0x100 to use it.

**Sort value** (V 0x37f3c0..0x37f400):
- `v = S[3] × gm+0x58a0`, where S[3] is the translation row of the header matrix;
- `k = gm+0x5920` (1023, 0, 0, 0.0256);
- `z = ftoi0(min(max(v.w · k.w, 0), k.x))` (the w lane: `vmtir` fsf = w, V);
- then `w2 = w2 & 0xe00003ff | (z << 10 & 0x1ffffc00)`, `w1 = w1 & ~3 | 2`.

### 2.4 Packet (one per drawn group; V `EmitModelHeader` / `EmitModelMatrix2` / `EmitModelLighting` and 0x37fd30..0x37fdb0)

`start = *(gm+0x5a00)`, a physical address. It is written through the uncached alias `start | 0x30000000`.

`EmitModelHeader(a0 gm, a1 = S+0x000, a2 = S+0x680, a3 = S+0x040 (unused), t0 = mat2, t1 = light, t2 = flags, t3 = &cursor)`:

| off | qw | VIF / content |
|---|---|---|
| +0x00 | DMA `CNT 6` (0x10000006; upper 64 bits 0, `por` clear, V) | — |
| +0x10 | `STMASK 0` (0x20000000, 0), `STCYCL 1,1` (0x01000101), `UNPACK V4-32 ×5 → 0 FLG` (0x6c058000) | VU TOP+0..3 ← S[0..3] (header matrix), TOP+4 ← {0, 0, 0, flags} |
| +0x20..0x6f | 5 data qw | |
| if mat2 ≠ 0: +0x70 | `CNT 5` (0x10000005), then qw NOP, NOP, NOP, `UNPACK V4-32 ×4 → 5 FLG` (0x6c048005) + 4 qw = *mat2 | TOP+5..8 |
| if flags & 1: next | `CNT 15` (0x1000000f); NOP…, `UNPACK V4-32 ×3 → 9 FLG` (0x6c038009) + 3 qw = S[0x68..0x6a]; NOP, `UNPACK V4-32 ×10 → 0xf FLG` (0x6c0a800f) + 10 qw = *light | TOP+9..0xb (lighting basis), TOP+0xf..0x18 (SH K0..K9) |
| then | `CALL` qwc 0 → `base + g.offset` (`0x50000000 \| addr << 32`), upper 64 bits 0 | mesh chain (§3) |

Sizes: 0x70 + (0x60) + (0x100) + 0x10.

`end = (CALL tag addr + 0x10) & 0x0fffffff`. After every packet, `gm+0x5a00 = end + 0x10` (V 0x37ffd8 / 0x380190 / 0x3802e8). The 0x10 is room for the NEXT / RET that `DeferMan_AppendPacket` / `RenderFrame` write at `end`. So one DrawModel call writes **several back-to-back packets**: `next.start = this CALL tag + 0x20`.

TOP+0xc..0xe and +0x19..0x1a are not written. When mat2 = 0, TOP+5..8 keep stale VU memory: the setup copies them to the texgen matrix anyway, but no texgen bit should be set then (I).

### 2.5 Records and buckets (V 0x37fdac.., D for the hash)

Record writes are capped at `*defer < 0xa28` (2600). Past the cap the packet is still written and the cursor still advances (an orphan packet).

A record (0x80, at `defer + 0x80 + 0x80·count`, uncached) holds:
- cur (0x14 B) at +0;
- start at +0x14;
- end at +0x18;
- short matrix index `defer+0x69cc0` at +0x1c;
- short continuation flag at +0x1e.

Before the copy, `cur.w2 = cur.w2 & ~0x1f | (defer+0x69cc4 & 0x1f)` (scissor).

1. **alpha == 1.0, base == 0:** one record, continuation 0.
2. **alpha == 1.0, base ≠ 0 (fast path, no record):**
   1. `base.w2` scissor bits are overwritten in the **shared base state** (`*(base+8)`).
   2. Key: textures = cur+0x10 / +0x12, matrix = `defer+0x69cc0`, and an 8-bit hash of (base, tex).
   3. `DeferMan_FindBucket`; if it misses, create bucket `defer+0x57498 + 0x20·numBuckets++` (+0 state, +4 tex pair, +8 matrix, +0xc/+0x14/+0x18 = 0), link it into the hash `defer+0x678a0`, and append {`DeferMan_ComputeSortKey`, bucket} to the sort list `defer+0x67ca8 + 8·n`, n = `defer+0x67ca4`++.
   4. `DeferMan_AppendPacket(bucket, start, end)`: the first packet sets +0xc / +0x10; later ones write `NEXT start` at the previous end.
3. **alpha ≠ 1.0: two records for the same packet** (D, constants V at 0x37f1d4..):
   1. Push the state. Apply sort 2 + value, blend1 = 3, blend2 = 3. Preset 3 = 0x6a = (0 − 0)·FIX + Cd: **depth-only pre-pass**. Record, continuation 0.
   2. Pop, then push again from the **original** state: ztst sel 1 (GEqual), atst 0, aref 0x14, `w0` FIX1 (bits 12-19) = `(int)(alpha·128) & 0xff`, blend1 = 11 (FIX blend `(Cs − Cd)·FIX + Cd`). Record, continuation 1. Pop.

   (`n2-render-layer.md` says "ZMSK" for the second record: it is ZTST GEqual, not ZMSK.)

**Guest-visible leftovers:**
- cur w0 type 3 / w3;
- per group, cur tex ids, clamp bits and (non-fast) w1 / w2 fields;
- with flag 8, the push/pop restores them;
- scratchpad 0x70003800..0x70003fff (S[0..0x7f]);
- base-state scissor bits.

## 3. Mesh chain format (prebuilt, in the model file; V on `ram_gen.bin`)

A list of DMA `REF` tags (0x3000qwc, addr; upper 64 bits 0) ending in `RET` (0x60000000). The CALL target is `base + group.offset`:
- `base` = `inst+0x98` (unlit; colours);
- or `desc+0x24` (lit; normals).

Batch pattern. The first batch of a group writes at TOP+0x1b, after the header; later batches at TOP+0 of the other buffer.

```
REF geometry   [first only: STCYCL 1,1; STMOD 0; STROW 1.0,1.0,1.0,1.0]
               ITOP 0
               UNPACK V4-32 ×2 → B+0  FLG        GIF tag, mesh header
               UNPACK V2-8  ×s → B+2  FLG        strip lengths
               STMASK 0x50505050; UNPACK V2-16 MASK ×n → B+2+s+n FLG   UV (zw = row = 1.0)
               STMASK 0x40404040; UNPACK V3-16 MASK ×n → B+2+s   FLG   xyz (w = row = 1.0)
REF attributes unlit: STCYCL 1,1; UNPACK V4-5 ×n → B+2+s+2n FLG      colours
               lit:   STMASK 0x40404040; UNPACK V3-16 MASK ×n → B+2+s+2n FLG   normals (w = 1.0)
               STMASK 0; MSCAL e; ITOP 1
[REF 1 qw → static MSCAL qword]    pass-2 / pad calls (below)
...
RET
```

B = 0x1b for the first batch of the group, else 0. e = 0x44e (byte 0x2270) for the first batch, 0x459 (0x22c8) for the rest. All unpacks are signed (USN 0).

**Mesh header** at B+0..1, written by `UNPACK V4-32`:
- qw0 = GIF tag: NLOOP = n, EOP, PRE, PRIM 0x5c (tri-strip, IIP, TME, ABE), PACKED, NREG 3, REGS 0x412 (ST, RGBAQ, XYZ2). Example: `0000802c 302e4000 00000412 0`.
- qw1 = {s, 0, n, 0}.

**Field encodings:**
- **Strip lengths:** V2-8 x = length (signed byte), y = 0. Σ = n. Example: s = 2, (48, 7), n = 55.
- **Positions:** int16 x, y, z; the VU converts with ITOF15 (÷ 32768). Object space before the header matrix, which carries desc scale × inst scale.
- **UV:** int16 4.12 (÷ 4096).
- **Colours:** RGBA5551 16-bit. VIF expands it to r<<3, g<<3, b<<3, a<<7 (example 0xc210 → 128, 128, 128, 0x80).
- **Normals:** int16 ITOF15.
- **Batch size:** n ≤ ~60 (seen 30..55).

**Pass 2 (two-texture models).** Extra 1-qw REFs into static qwords:
- 0x509400 = `MSCAL 0x480` (0x2400);
- 0x509410 = `MSCAL 0x44e` (0x2270);
- 0x509420 = `MSCAL 0x459` (0x22c8).

`World_StaticInitDestroy` writes these qwords at boot (V xrefs). They run with ITOP still 1, so 0x2270 / 0x22c8 take the pass-2 path (ST rewrite of the earlier pass-1 output, ctx2 tag, re-kick). The VIF double buffer toggles TOP on every MSCAL, so **a pass-2 call re-draws the batch two MSCALs earlier**. 0x2400 is a pad that toggles the parity: a PRIM-only kick of the tag at VU qw 0x1f.

Seen pattern (lit two-pass chain 0xd79570):

```
B1 0x2270, B2 0x22c8, P2 0x2270 (=B1), P2 0x22c8 (=B2), B3 0x22c8, pad 0x2400, P2 0x22c8 (=B3), RET
```

Single-texture chains (0xbb61d0) have no pass-2 REFs.

**Decoding once per chain address:**
1. Walk the REFs and interpret the VIF (STCYCL / STMASK / STROW / ITOP / UNPACK as listed) into a per-call buffer.
2. At each MSCAL emit `Call::Pass1 { batch }`, `Call::Pass2 { batch: index of the call two MSCALs earlier }` or `Call::Pad` (by entry and ITOP).
3. Each batch: `{ setup: bool (B == 0x1b), strips: Vec<u8>, xyz: Vec<[i16; 3]>, uv: Vec<[i16; 2]>, attr: Rgba5551(Vec<u16>) | Normal(Vec<[i16; 3]>), tag: u128 }`.

**When the chain changes.** Model chains are static file data: nothing in DrawModel or WorldLightMan writes them (lighting is per draw, in the packet). The cache key is the chain address. Invalidate when the model's memory is freed or reloaded (world streaming / model unload, I); a cheap guard is to compare the first tag qwords. The DMA reads the chain at EndFrame, so a model freed between draw and EndFrame would differ (I; not observed).

## 4. cPS2GridMesh_Draw 0x385c10 (D; header call V by xref)

`(a0 this, a1 matrix, a2 short texId, a3 mat2 or 0)`. `gm = *(this+0x1c4)`.

1. If `this+0x1a4`: clear it and toggle `this+0x1a0` (buffer index).
2. UpdateViewMatrices if needed. `cur.w0` type 3, `cur.w3 &= 0xc0000000`, `cur+0x10 = texId` (tex1 untouched).
3. Header matrix = rows of `matrix` × (s, s, s, 1), lane-wise per row as in §2.2, with s = `*(float*)(this+0x194)`.
4. The packet is built in **SPR**: `SprStream_BeginBlock(gp+0x1384, gm+0x5a00)`, then `EmitModelHeader(gm, M, matrix, &scale, t0 = a3, t1 = 0, t2 = a3 ? 0x22 : 0x20)`:
   - texgen1 = 2 (UV matrix through matrix 2) when a3 is set;
   - clip is always on;
   - never lit.
5. Then `CALL *(this + 0x1ac + 4·this.0x1a0)` and `EndBlockAligned(…, 4)` → end.
6. One record (continuation 0, the same layout as §2.5); `gm+0x5a00 = end + 0x10`.

No fast path, no alpha path.

**Chain** (`cPS2GridMesh_BuildDmaPacket` 0x386128, D): one `CNT` (qwc = size − 3) holding everything inline, then `RET` (+ STCYCL). Per row i (count `this+0x18c`; per row {+0x10 strips?, +0x14 n}, 0xc stride):
- the same VU layout as §3: B = 0x1b for the first row, else 0;
- STCYCL / STMOD 0 / STROW 1.0 / ITOP 0;
- V4-32 ×2 (tag 0x302e4000_00008000 | n, REGS 0x412; {s, 0, n, 0});
- V2-8 lengths;
- masked V2-16 UV;
- masked V3-16 xyz;
- V4-5 colours;
- `MSCAL 0x464` (0x2320) for the first row, `0x46b` (0x2358) for later rows. These are the ITOP-independent entries: **no pass 2**.

**Dynamic data.** Positions, UVs and colours sit inline in the two chain copies (`this+0x1ac` / `+0x1b0`, position buffers `+0x1b4` / `+0x1b8`):
- `cPS2GridMesh_v03` (positions, `PackPosition`: int16 = pos × `this+0x198`) writes the **inactive** copy and sets `+0x1a4`;
- `v04` / `v05` / `v06` write UVs (`PackUV`: ×4096) and colours (`PackColor`: RGBA5551, a = 0x8000 if a > 0).

So the chain content changes every frame, and the next frame's writes go to the other copy. **Decode at capture time.** Do not cache by address, or key by address plus a content hash.

## 5. Capture

Wrap 0x37e238 and 0x385c10. Let `p0 = *(gm+0x5a00)` before and `p1` after.

Run the body, then parse the packets in `[p0, p1)`:
1. at p: expect `CNT 6` (0x10000006);
2. read 4 qw matrix + the flags qw;
3. optional `CNT 5` (matrix 2), optional `CNT 15` (basis 3 qw + SH 10 qw);
4. `CALL` → chain;
5. next p = CALL + 0x20.

GridMesh's packet came through SPR, but it is in main RAM after `EndBlockAligned`, so it parses the same way.

Read everything from the packet: it is an exact copy of what VU1 receives at TOP+0..0x18. Nothing in the packet is read later than the call except the chain (static for models; double-buffered for grid meshes).

Proposed command:

```rust
Cmd::Mesh {
    // VU1 TOP+0..4: header matrix rows (raw bits) and flags (qw4.w)
    model: [u128; 4],
    flags: u32,
    // TOP+5..8 when emitted (texgen matrix); None = stale VU memory (texgen bits should be 0)
    matrix2: Option<[u128; 4]>,
    // TOP+9..0xb basis, TOP+0xf..0x18 SH K0..K9 (flags & 1)
    light: Option<([u128; 3], [u128; 10])>,
    // CALL target, and for grid meshes the decoded batches (dynamic)
    chain: u32,
    batches: Option<std::sync::Arc<MeshChain>>, // None → decode/cache by `chain`
}
```

**What the renderer needs to plan it:**
- The bucket's state, tex[0..1] and `header_matrix` (`frame.rs` `Draw`): VU qw0-3 = MatrixRecord.world_view, qw4/5 guard. Those are what the setup multiplies with `model`: qw6-9 = `xform(M, model rows)`.
- The decoded chain: per call, pass 1 (ctx1, tex0) or pass 2 (ctx2, tex1, texgen2 from flags bits 6-9 with matrix 2 / normals) of a batch.
- Per pass-1 batch:
  - positions ITOF15 → × qw6-9 → guard (flags & 0x20: guard-band cull and near clip);
  - UV 4.12 → texgen1 per flags bits 1-4: 2 = UV matrix with matrix 2 rows, 4 = planar, 8 = normal → UV; 0x10 sphere map is unimplemented in mat3 too;
  - colour = RGBA5551 expanded, or, lit, SH from basis + K (`mat3::light`).
- Each strip of the batch is a separate tri-strip.

The VU semantics are in `mat3.rs` (`mesh_pass1`, `mesh_pass2`, `Mesh::read`).

**Alpha < 1:** the same `Cmd` serves both records (depth pre-pass, then the FIX blend); the records differ only in state.

**Fast-path buckets** carry the base state, not cur, but join by packet start like any other.
