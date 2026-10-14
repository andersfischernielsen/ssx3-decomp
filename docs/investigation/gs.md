# GS (Graphics Synthesizer) software implementation

Module: `crates/ssx3-ps2/src/gs/`. It is a CPU software GS. The public API is the
contract in `gs/mod.rs`: `new`, `write_priv`/`read_priv`, `gif_write`, `write_reg`,
`irq_pending`, `vblank`, `display`, `read_local`, and the public `vram: Vec<u32>`.
Additions: `gif_path_busy`, `local_pending`, `raise_event`, `prims_drawn`, the
`CSR_*` constants, and `gs::swizzle` (formats, `Surface`, `read_pixel`/`write_pixel`).
Also added: `sync`, `set_deferred`, `vram_written`, `batch_stats`, and trace
capture (`start_capture`/`finish_capture`, `gs::capture`). See
`gs-performance.md`.

A GPU rasterization backend (wgpu compute, `SSX3_GS=gpu`) shares
everything except the pixel pipeline; see `gs-gpu.md`.

The swizzle tables and several behaviours are ported from
[ps2-web-recomp](https://github.com/snuri00/ps2-web-recomp) (MIT;
`runtime/src/gs/{gs,gif,raster}.cpp`). Those in turn match PCSX2 GSdx.

## Layout

| File | Contents |
|---|---|
| `mod.rs` | `Gs` state, privileged registers (CSR/IMR/SIGLBLID), the GS register file, vertex queue/kick, CLUT loading |
| `swizzle.rs` | PSM constants, page/block/column tables, `Surface` addressing, pixel read/write |
| `gif.rs` | GIFtag state machine per path (PACKED / REGLIST / IMAGE), PACKED register decoding |
| `transfer.rs` | Host→local (IMAGE/HWREG), local→host (queued qwords), local→local |
| `raster.rs` | Per-draw setup (`Draw`), primitive setup (`Prim`), texturing, the per-pixel pipeline, specialized span loops |
| `batch.rs` | Primitive batching, dependency tracking, multithreaded and background drawing |
| `pool.rs` | Worker pool (`std::thread`) |
| `region.rs` | VRAM block sets used for dependency tracking |
| `texcache.rs` | Cache of decoded (RGBA8) textures, invalidated by page write generations |
| `capture.rs` | GS trace capture/replay (the `gs_replay` binary) |
| `ref_raster.rs`, `diff_tests.rs` | Test-only: the original rasterizer and a differential test against it |
| `display.rs` | PCRTC: PMODE/DISPFB/DISPLAY/SMODE2 → RGBA8 |
| `tests.rs` | Unit tests, plus an ignored perf test |

## VRAM and formats

- 4 MiB is stored as 1M little-endian `u32` words. Each format addresses elements
  (32/16/8/4-bit) inside an 8 KiB page through a precomputed `u16` table indexed
  by `(y mod ph) * pw + (x mod pw)`. The tables are built once in a `OnceLock`.
  - CT32/CT24/Z32/Z24/T8H/T4HL/T4HH: 64×32 page (CT and Z have their own tables)
  - CT16/CT16S/Z16/Z16S: 64×64 page, four tables
  - T8: 128×64 page
  - T4: 128×128 page
- Address: `elem = ((bp*64 + page*2048) << es) + table[...]`, where
  `page = x/pw + (y/ph) * max(1, bw*64/pw)`. Coordinates are masked to 11 bits
  and the result wraps at 4 MiB. Base pointers in blocks are supported, so
  unaligned TBP/CBP/DBP work. FBP/ZBP are multiplied by 32.
- CT24/Z24 writes keep the top byte. T8H/T4HL/T4HH read and write bits 31:24,
  27:24 and 31:28 of the CT32 word.
- Tests check that every table is a bijection, that each format round-trips a
  full page and touches only that page, and that addresses match the PCSX2
  `columnTable8`/`columnTable4` rows and the Z block orders.

## GIF

- Each path (1..=3, clamped) has its own tag state, so packets can be split
  across `gif_write` calls. NLOOP=0 tags are consumed with no data. Each tag
  resets internal Q to 1.0. PRE writes PRIM in PACKED mode.
- PACKED handles PRIM, RGBAQ (with internal Q), ST (latches Q), UV, XYZF2/XYZ2
  (ADC bit 111 turns them into XYZF3/XYZ3), TEX0/CLAMP, FOG, XYZF3, XYZ3, A+D
  and NOP. Register 0xB is ignored.
- REGLIST writes 64-bit halves directly. With an odd NREG×NLOOP the trailing
  half is dropped. Ids 0xE and 0xF are ignored.
- IMAGE sends raw bytes to the active host→local transfer. HWREG (0x54) does
  the same with 8 bytes.

## Registers

All registers in the brief are decoded. Changing any non-vertex register drops
the cached `Draw` setup, which is rebuilt on the next kick. PRIM resets the
vertex queue.

TEX0 and TEX2 trigger a CLUT load:
- CLD 0–5 are supported, with CBP0/CBP1 compare-and-skip for 4 and 5. A load
  only happens when the texture PSM is indexed.
- CSM1 8-bit uses a 16×16 layout with index bits 3 and 4 swapped. CSM1 4-bit
  uses 8×2. CSM2 uses TEXCLUT (CBW/COU/COV) and reads entries linearly.
- The CLUT is 512×16-bit, like the hardware buffer. CT32 entries are split into
  low and high halves (slot and slot+256, CSA&15). CT16/CT16S use CSA×16 over
  512 entries.

SIGNAL updates SIGLBLID.SIGID under its mask and sets CSR.SIGNAL. FINISH sets
CSR.FINISH straight away, because drawing is synchronous. LABEL updates
SIGLBLID.LBLID.

## Privileged registers

- PMODE…BGCOLOR (0x00–0xE0) are stored and read back.
- CSR read gives the event bits plus FIELD(13), FIFO-empty (bits 15:14 = 01),
  REV 0x1B and ID 0x55: `0x551B4000 | events`.
- Writing 1 to a CSR event bit clears it. RESET (bit 9) clears GIF, vertex,
  transfer and readback state but keeps VRAM.
- IMR keeps bits 14:8. `irq_pending = csr & 0x1F & !(imr >> 8)`.
- `vblank(field)` sets VSINT and sets FIELD to `field & 1`.
- HSINT/EDWINT are never generated internally. `raise_event` lets the machine
  raise them.

## Transfers

- Host→local works for every PSM: 32, 24 (3-byte packed, carried across writes),
  16, 8 and 4 bits (low nibble first). Extra data after RRW×RRH is ignored.
- Local→host runs when TRXDIR=1 is written. The whole rectangle is encoded into
  a qword FIFO, padded to 16 bytes, which `read_local(max)` drains.
- Local→local reads the whole rectangle first, then writes it. The TRXPOS DIR
  bits are ignored.

## Rasterizer

Primitives:
- **Vertex queue.** It follows the PS2 kick rules: point 1, line/strip 2, tri,
  strip and fan 3, sprite 2. Strips and fans keep the right vertices. XYZ3/XYZF3
  advance the queue without drawing.
- **Attribute source.** PRMODECONT.AC chooses PRIM or PRMODE attributes, and
  CTXT selects context 1 or 2.
- **Coordinates.** Positions are 12.4 fixed point minus XYOFFSET. Pixels are
  sampled at integer coordinates.
- **Triangles.** Integer edge functions in i64 with an exact top-left fill rule;
  a test checks that a strip's shared edges are drawn exactly once. Attributes
  come from f64 plane equations. Flat shading uses the last vertex's colour.
- **Sprites.** A pixel is covered when `x0 <= p*16 < x1`. Colour, Z and F come
  from the second vertex; S/T or U/V interpolate.
- **Lines.** A DDA that excludes the last pixel.
- **Points.** Rounded to the nearest pixel.

Texturing:
- STQ uses `s/q * 2^TW`, with S/T/Q interpolated linearly in screen space. UV
  uses `u/16`.
- TW and TH are capped at 10. All four wrap modes are supported: REPEAT, CLAMP,
  REGION_CLAMP, and REGION_REPEAT as `(u & MINU) | MAXU`.
- **Filtering:**
  - Nearest filtering is always available.
  - Without mipmapping, bilinear filtering (8-bit weights) is chosen per
    primitive. The LOD comes from TEX1 (LCM/K, or `log2(1/q)<<L + K` using the
    average vertex Q). LOD ≤ 0 uses MMAG; otherwise MMIN, where 1, 4 and 5 count
    as linear.
- **Mipmapping** applies when MXL > 0 and MMIN is 2–5. The LOD is computed per
  pixel: K when LCM=1, otherwise `log2(1/|Q|) * 2^L + K`.
  - LOD ≤ 0 samples level 0 with MMAG.
  - Otherwise LOD is clamped to MXL. MMIN 2/4 (`*_MIPMAP_NEAREST`) picks the
    nearest level, `round(LOD)`.
  - MMIN 3/5 (`*_MIPMAP_LINEAR`) blends `floor(LOD)` and the next level by the
    8-bit fraction.
  - Inside a level, MMIN 2/3 use nearest sampling and 4/5 use bilinear.
  - Level `l` has base pointer and width from MIPTBP1 (levels 1–3) and MIPTBP2
    (4–6), or from MTBA for levels 1–3, where each level follows the previous
    one at half the width.
  - The level is `2^TW >> l` by `2^TH >> l` texels, sampled at `(u, v) / 2^l`.
    Region clamp/repeat bounds shift right by `l`.
- **Formats:**
  - CT32/24/16/16S, Z formats read as colour, and T8/T4/T8H/T4HL/T4HH through a
    palette resolved per draw (256 or 16 `u32` entries).
  - TEXA expansion: 16-bit alpha is TA1 if bit 15 is set, otherwise TA0, or 0
    when AEM is set and RGB=0. 24-bit alpha is TA0, or 0 when AEM is set and
    RGB=0. CT16 colours expand by `<<3`.
- TFX supports MODULATE, DECAL, HIGHLIGHT and HIGHLIGHT2 (`(T*C)>>7`), with TCC.

Per-pixel pipeline, in order:
1. Scissor and SCANMSK.
2. Texture/TFX.
3. Fog: `(F*C + (255-F)*FOGCOL) >> 8`.
4. Alpha test: all ATST modes; AFAIL KEEP, FB_ONLY, ZB_ONLY and RGB_ONLY.
5. DATE/DATM. Skipped for CT24.
6. Z test: ZTE/ZTST on Z32/24/16/16S, with Z clamped to the format maximum.
   ZMSK is honoured, and nothing is written when ZTE=0.
7. Blending: `((A-B)*C>>7)+D`. PABE skips blending when As < 0x80.
8. DTHE/DIMX dithering, only for 16-bit frames.
9. COLCLAMP clamps or wraps the colour.
10. FBA, then the FBMSK-masked write to CT32, CT24 or CT16/CT16S.

When the frame is read as a blend destination:
- CT24 reads with alpha 0x80.
- CT16 alpha is 0x80 when bit 15 is set, otherwise 0.

Performance: primitives are batched and drawn by several threads, with exactly
the serial per-pixel order (see `gs-performance.md`). Heavy in-game frames went
from 250–530 ms to 35–42 ms on an M1. Two tools measure it:
- `gs_replay`, which replays captured traces.
- The ignored perf test: `cargo test -p ssx3-ps2 --release -- --ignored perf --nocapture`.

## Display

- The circuit shown is circuit 1 if EN1 is set, otherwise circuit 2. Its DISPFB
  (FBP/FBW/PSM/DBX/DBY) gives the buffer.
- Size is `(DW+1)/(MAGH+1)` × `(DH+1)/(MAGV+1)`. DX/DY are ignored.
- When SMODE2 has INT=1 and FFMD=1 (field mode), framebuffer rows are read at
  `y/2`, matching the PCRTC reading one buffer line per field line. A 640×448
  NTSC mode (DW=2559, MAGH=3, DH=447) therefore gives 640×448.
- When both circuits are enabled they are alpha-merged:
  - The alpha is ALP when MMOD=1. Otherwise it is circuit 1's pixel alpha × 2,
    capped at 255.
  - Circuit 1 is merged over circuit 2, or over BGCOLOR when SLBG=1.
- Output is `0xAABBGGRR` with A forced to 0xFF. With no circuit enabled, the
  output is black.

## Known gaps

- Mipmapping: MTBA address computation is a best guess (levels packed one after
  another, width halved). Without a mipmap MMIN, the MMAG/MMIN choice is still
  made per primitive.
- No antialiasing (AA1), and no PRIM.FIX fragment control.
- Rasterization is approximate rather than bit-exact:
  - Colours interpolate in f64 rather than the GS's fixed-point DDA, so they can
    differ by ±1 LSB.
  - Lines use a simple DDA.
  - The point rounding convention is approximate.
  - Sprite STQ uses the second vertex's Q.
- TRXPOS DIR is ignored, and so are the hardware's overlapping-copy quirks.
- Local→host is generated in one go when TRXDIR=1 is written. There is no FINISH
  or BUSDIR handshake beyond storing BUSDIR.
- GIF path arbitration (PATH3 masking, IMT, path priority) is left to the
  machine. Paths are handled independently and immediately.
- No HSYNC/EDW event generation, and no NFIELD.
- The display ignores DX/DY offsets, PMODE.CRTMD, EXTBUF/EXTWRITE feedback, and
  circuits with different geometry when merging.
- The CLUT CBP0/CBP1 cache compares only CBP, as the hardware does, so games
  must use CLD=1 to force a reload. TEX0 writes for non-indexed PSMs never load
  the CLUT.
