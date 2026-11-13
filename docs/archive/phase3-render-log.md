# Phase 3 rendering: progress log (October 2026)

_Archived from `investigation/phase3-render.md`, which keeps the design (sections 0-5)._


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

### Stage 0b: the typed frame (2026-10-16)

`crates/ssx3-render/src/frame.rs` builds the typed `Frame` (§2.2) from EE memory at the moment
`PS2GraphicsMan_EndFrame` signals the kick thread (syscall 0x42 on gm+0x5ac8; hook behind
`SSX3_VERIFY_FRAME=1`). The frame holds:
- views, matrices and scissors;
- passes: draws, default state, depth fog, phase boundary, bloom, colour filter, post chain;
- per draw:
  - the decoded `RenderState`: blend preset and FIX, alpha/depth test, clamp, scissor,
    layer, sort, material;
  - its textures and the predicted GS registers;
  - the prelude (state change, material setup, texture bind) and the payload: VU calls and
    GIF streams from the stage-0a interpreter.

`crates/ssx3-render/src/gif.rs` is a streaming GIF reader (PACKED, REGLIST, IMAGE) that
reports the A+D writes.

Check, menus and race to frame 4800 (4,745 frames):
- **(a)** all 2,574,506 VU calls are attributed to exactly one draw, in order, with no
  leftovers;
- **(b)** 3,375,730 predicted SCISSOR/ZBUF/TEST/CLAMP/ALPHA values equal the chain's A+D
  writes, 0 bad;
- **(c)** TEX0 matches the draw's texture (for streamed textures, apart from VRAM
  placement), 0 bad;
- the VU1 header qw0-3 equals the draw's MatrixRecord on 2,363,299 calls.

Corrections found while doing this (they supersede the earlier text and `structs/render.md`):
1. **Matrix header re-upload is keyed on the state pointer.** A bucket that shares its
   predecessor's state keeps the old header. This never happened in the run, but
   `Draw::header_matrix` models it.
2. **`EmitDefaultState` encodes context 2 differently.** ZBUF_2's ZMSK comes from w1 bit 23,
   and TEST_2 takes ZTST from bits 25-26 and ATST from bits 22-23.
3. **ALPHA presets.** Presets 15 and 18-31 emit an A+D with stale data. Presets 11-14 OR in
   FIX; preset 2 uses FIX 0x80.
4. **The layer split is a binary search that is `< 6` / `< 9`.** It never tests the first
   bucket of a range. Continuation buckets follow their head regardless of their layer.
5. **Menus have numViews = 0**, so they emit no fog, bloom or colour-filter passes.
6. **Post passes write GS packets through PATH3 (GifBuff).** No VU calls happen outside draws.
7. **Render targets are switched by A+D writes inside type-0 payloads.** Example: the
   once-per-race-frame 128×128 render-to-texture with texture 0x60d; about 37 draws per frame
   run under it. The native renderer must apply A+D writes made inside payloads.
8. **Streamed textures get VRAM per draw.** Identify textures by id, and take TBP/CBP from
   the TEX0 in the prelude.

### Composition round trip (2026-10-16)

`crates/ssx3-ps2/src/gs/gpu/native.rs` implements a native run at an exact point in the GS
command stream (`Cmd::Native` through MTGS):
1. Unswizzle the current frame (CT32/24/16/16S) into `Rgba8Unorm` and Z (Z24/16/16S) into
   `Depth32Float` (Z/2^24, exact for 24-bit integers; Z32 is refused).
2. Run the recorded native passes (`NativeRecord` closures with access to the targets and the
   encoder).
3. Swizzle both back into VRAM.

API: `Gs::native_run`, `Mtgs::native_run(NativeJob)`, `GpuGs::begin_native_run`.

Check:
- Unit tests compare round-trip identity on random VRAM across formats and widths, the
  targets against the CPU swizzle model bit for bit, write-back of a pass, and ordering in the
  stream.
- `SSX3_RENDER_ROUNDTRIP=N` inserts an empty run before every N-th GIF transfer. With N = 1,
  the full race to frame 9000 did 11.4M round trips and stayed byte-identical (36 shots plus
  RAM). A check with N = 7 to frame 4700 (495k round trips) matched 47 of 47 shots.

Cost: about 150 µs per round trip at 640x448, so about 1 ms per frame at the planned few runs
per frame.

What stage 2B needs from it:
- depth `Greater`/`GreaterEqual` with `z = min(Zgs, 2^24−1)/2^24`;
- source alpha doubled for the 0x80 = 1.0 convention;
- destination-alpha presets need a float target;
- AFAIL = FB_ONLY as two pipelines;
- per-run buffers;
- textures from a decode cache, or by binding the VRAM buffer.

### Stage 2B: type 3 drawn by wgpu (2026-10-16, `SSX3_RENDER_NATIVE=type3`)

`crates/ssx3-ps2/src/gs/gpu/native3.rs`, `native3.wgsl` and `native3_dual.wgsl` draw the
type-3 material with wgpu's hardware rasterizer inside composition runs.

**How it works.**
- **Stream order.** Type-3 VU1 packets go through `Mtgs::gif_write_type3`. On the GS thread
  they are parsed by the GS model as usual: tags, PRIM, ADC, A+D. Each triangle then carries
  the full register state of its context and is drawn natively instead of by the compute GS.
- **Batching.** Triangles collect into one run until emulated GS work, a transfer touching
  the run's pages, or a surface change intervenes. That is about 16 runs per frame, each
  round-tripping only the bounding rectangle.
- **Positions and depth.** Vertices are pulled in window pixels. The GS's own per-triangle Z
  plane makes depth tests and Z writes bit-exact.
- **Fragment shader.** It reuses the GS WGSL texturing (all PSMs, CLUT, wrap, filtering, mips,
  read from VRAM), TFX, fog, the alpha test and FBA.
- **Blending.** Premultiplied source with a rounding bias, plus a fixed-function destination
  factor (dual-source As/128 or FIX/128).
- **AFAIL = FB_ONLY.** The alpha range is bounded from vertex and texture alpha. Mixed
  outcomes draw twice, over coverage-disjoint groups.

**Results** (race and menus to frame 9000, against the emulated GS):
- Final RAM is identical, so the game state is unaffected, including the flare Z readback.
- On average 0.67% of pixels differ (worst shot 1.91%, a menu). No difference exceeds 8; in
  practice almost all are ±1 from interpolation and blend rounding.
- 35.8M triangles native, 3.3% fall back to the emulated GS: As > 0x80 factors,
  destination-alpha/DATE presets, 0x3bf0 lines, and textures that overlap the target.
- Wall time +4%: wgpu encoding overhead while both paths coexist.
- The default path is unchanged: the A/B is byte-identical.

**Next:**
- make more materials native (2D/DIRECT, terrain types 1/2) so runs merge;
- move vertex maths into WGSL;
- use a wider alpha target for destination-alpha presets;
- then flip the master surfaces to wgpu, which allows resolution scaling.

### All materials through the native path (`SSX3_RENDER_NATIVE=all`, 2026-10-16)

The routing takes material names: `type3`, `terrain` (types 1/2), `rider` (types 6/7),
`particles` (types 4/5), or `all`.

With `all`, through the full race to frame 9000:
- **Game state:** final RAM identical.
- **Images:** 0.68% of pixels differ on average (worst 1.92%), none by more than 8.
- **Triangles:** 75.8M native, 22.3M (22.7%) fall back to the emulated GS:
  - As > 0x80 blend factors: 12.2M (needs a wider alpha target);
  - unsupported frame format: 7.1M (likely the rider shadow's 16-bit render target);
  - destination alpha: 1.6M;
  - non-triangles (sprites, lines): 1.3M.
- **Runs:** 589k (about 65 per frame), mostly cut by interleaved fallbacks.
- **Wall time:** 156 s against about 120 s emulated. The overhead is run splitting and the
  FB_ONLY texel scans (10.2G texels): bound alpha per texture once and cache it.

Next:
- a wider alpha target (e.g. Rgba16Float or a 2× alpha encoding) for As > 0x80 and
  destination alpha;
- CT16 frames;
- sprites;
- a per-texture alpha-range cache;
- then fewer runs.

### Fallbacks nearly gone (2026-10-16)

`gs/gpu/native3.rs` and `native3.wgsl` cover the cases the blend unit cannot express.
- **Destination reads in the shader:** the full GS integer blend for As > 0x80, destination
  alpha, DATE, PABE, no COLCLAMP, partial FBMSK, dithering and CT16 blends. The shader reads
  VRAM, then a copy of the target taken at segment breaks. Deferred triangles are kept in up
  to 8 overlap layers, so few copies are needed.
- **CT16 frames:** native, with GS 5:5:5:1 rounding.
- **Sprites:** native, from the GS sprite record, so the GS sprite rules hold exactly.
- **Alpha bounds:** cached per texture and invalidated through VRAM page stamps.

Full race with `SSX3_RENDER_NATIVE=all`, against the emulated reference:

| | Before | Now |
|---|---|---|
| Fallback triangles | 22.7% | **0.40%** |
| Runs | 589k | 297k |
| Wall time | 156 s | 142 s |

- **What still falls back:** line strips, and sprites/triangles that sample their own frame.
- **Game state and images:** final RAM identical. On average 0.88% of pixels differ (worst
  3.4%), none by more than 8.
- **Default path:** byte-identical.

Open: 4-13 px per scene differ by more than 8 in the PABE and CT16-blend unit-test scenes
(none in game frames).

### 2D and post effects through the native path (2026-10-16)

Two new `SSX3_RENDER_NATIVE` tokens, both included in `all`:
- **`2d`:** VIF1 DIRECT/DIRECTHL packets (PATH2: UI, HUD, fonts, menus).
- **`post`:** GIF DMA packets (PATH3: depth fog, bloom, colour filter).

Both go through `Gs::gif_write_native(path, ..)`, which generalises `gif_write_type3`. In
`Mtgs`, the native flag is `0x80 | path` in `Cmd::Gif`. The GS model still parses the packet,
handles IMAGE transfers and state as before, and hands set-up primitives to `n3_push`.

- **Exact blending for PATH2:** with the blend unit, 6-8% of menu pixels were off by 1-2 on
  UI and font edges because its rounding differs from the GS. PATH2 primitives now use the
  exact destination-read blend (`N3::exact`, the `K_DR` path). `SSX3_N3_EXACT_2D=0` turns
  this off. After the change, menu shots match the earlier `all` run digit for digit
  (1.45-1.92% differ, max 2, all from the 3D rider). 2D adds no differences of its own.

Full race with `SSX3_RENDER_NATIVE=all`, against the emulated reference:

| | Before | Now |
|---|---|---|
| Native triangles | — | 99.34M in 288k runs |
| Fallback triangles | 0.40% | 0.84% |
| Mean pixels differing | 0.875% | **0.824%** |
| Worst shot | 3.4% | 3.4% |
| Pixels off by more than 8 | 0 | 0 |
| Wall time | 142 s | 152 s |

- **Fallbacks:** 0.84% overall (377k line strips, 468k texture-feedback sprites).
- **Game state:** final RAM identical.
- **Wall time:** 152 s on this run. The default path took 122-126 s on the same machine, so
  timings are noisy.
- **Default path:** byte-identical.

What falls back now is mostly post-effect and 2D sprites that sample their own frame (bloom
and colour-filter passes, which copy the frame to itself), plus line strips. Drawing these
natively needs a copy of the target as a texture.

### Lines and texture feedback through the native path (2026-10-16)

With `SSX3_RENDER_NATIVE=all`, nothing falls back to the emulated GS any more (points would,
but the race has none).

- **Lines (prim 1/2):** each line is drawn as a quad reaching 1.5 px around it. The fragment
  shader keeps only the pixels that the compute GS's line walk hits. It uses the
  `line_hit`/`line_step_t` functions, now shared with `gs.wgsl`, on the compute GS's own
  line record (kind 2). Attributes and Z come from that record at the same step, so
  coverage, colour and Z match bit for bit. Coverage uses the record's endpoint bounding box.
  A line outside the scissor draws nothing.
- **Texture feedback** (a primitive sampling its own frame or Z: frame copies, PSMT8H colour
  filter, depth fog reading Z as 8H, bloom). During a run, VRAM still holds the run's
  surfaces as they were before the run, and textures are read from VRAM. That matches the
  compute GS, whose texture reads come from a snapshot taken before the batch: a primitive
  sees what earlier primitives drew, and none of its own pixels. So:
  - If the texels a primitive can reach (`prim_tex_rect`, in blocks) overlap the blocks of
    the run's bounding box (frame and Z), the run ends first and its pixels go back to VRAM.
    This is `runs ended by: texture feedback`.
  - Otherwise the primitive joins the run and reads VRAM directly.

  Z read as a texture needs no conversion, because it is read from VRAM in GS format.
- **Exact blends and planes for these filters:** blended feedback primitives use the
  destination-read GS blend. The blend unit's rounding of the Cd term left 0.23% of a menu
  screen off by 1. Feedback triangles take their attributes from the compute GS's planes,
  truncated without the epsilon. Both together make them pixel-identical to the compute GS.
- **Tests:** `n3_lines_vs_compute` covers 7 scenes: flat, Gouraud, FB_ONLY, bilinear
  textured, blended, and As > 0x80, with strips, axis-aligned and diagonal lines.
  `n3_texture_feedback_vs_compute` covers 5 scenes: frame copy, bilinear blended copy,
  frame alpha as 8H, Z24 as texture, and Z as 8H, with own-texel and shifted reads
  interleaved with drawing. Every scene matches the compute GS exactly, with 0 pixels and
  0 Z words differing. With the run-end check disabled, the frame-copy scene breaks
  (100k px differ).

Full race with `SSX3_RENDER_NATIVE=all`, against the emulated reference:

| | Before | Now |
|---|---|---|
| Fallback prims | 844,513 (0.84%) | **0** |
| Native prims | 99.34M | 100.17M (371,626 lines, 467,922 feedback) |
| Runs | 288k | 301k (9,851 ended for feedback) |
| Mean pixels differing | 0.824% | 0.824% (identical per shot) |
| Worst shot | 3.36% | 3.36% |
| Max difference / pixels > 8 | 8 / 0 | 8 / 0 |
| Wall time | 146-152 s | 149 s |

- **Game state:** final RAM identical.
- **Default path:** byte-identical (36 shots and RAM). The `gs.wgsl` line code was only
  factored into functions.

Remaining differences come from hardware interpolation and blend-unit residuals of ordinary
native triangles, the same as before. Using the compute GS's planes, or the exact blend,
more widely would reduce them, at some cost.

### Persistent native targets (2026-10-16)

The wgpu colour and depth targets now persist across runs (`native.rs` `Held`). A run on the
same frame (and Z) buffer reuses what the targets hold, loading instead of clearing and
unswizzling, as long as both of these hold:
- the run's extent lies inside the held rectangle;
- no page of those buffers was written outside native runs since. Uploads, compute raster
  batches and GPU transfers stamp pages (`note_vram_write`).

When reuse fails on the same buffers, the whole buffer width is unswizzled once, so later
runs with other scissors reuse it. VRAM is still written back at every run's end, so
everything that reads VRAM stays exact. `SSX3_NATIVE_NO_REUSE=1` turns reuse off for A/B.

Full race: **252,016 of 300,797 runs reuse the targets** (misses: 44k other buffers, 4.7k
outside the rectangle, 3 written pages). Output is byte-identical to the previous native run
(36 shots) and the RAM is identical to the emulated reference. Wall time is unchanged at
about 145 s; the cost is CPU-side command encoding, not the unswizzle.

This is the prerequisite for rendering at a higher resolution, where the targets hold more
than VRAM can.

### Resolution scaling, step 1: N× copies (`SSX3_RENDER_SCALE=N`, 2026-10-16)

`gs/gpu/hires.rs` draws every native run a second time into N× textures (kept per buffer:
frame buffers as colour, Z buffers as depth, all N × the 1× target size). Only the 1× targets
are written back to VRAM, so everything that reads VRAM, and the game state, stays exact.

**Validity.** Each copy is current while nothing else writes its pages. Every VRAM write
stamps its pages: uploads, compute raster batches, transfers, and other native runs, which
catches aliasing such as bloom scratch in Z memory. A stale copy is refreshed from VRAM by an
N× unswizzle (one pipeline variant each for colour and depth).

**The N× pass** per run:
1. Refresh stale copies from VRAM.
2. Per segment, copy colour into an N× `prev`.
3. Replay the run's draw list with the viewport and scissors ×N and the scale in the
   parameter slot (`d.y`).

**Shader changes** (`n3_pixel`): integer GS-pixel tests (lines, sprites, dither) use
`floor(pos / N)`. The Z and attribute planes are evaluated at the fragment's own position, so
depth and attributes interpolate at the finer grid. Destination reads come from the N× `prev`.

**Checks:**
- With the scale unset, the native and default paths are byte-identical to before (36 shots
  plus RAM).
- At 2× the race runs without errors, and `SSX3_HIRES_SHOTS=dir` writes the N× copies at each
  screenshot. The main frame buffer shows the same image with visibly finer edges.

**Next:**
- render-target textures from the N× copies, so the display blit, colour filter, bloom and
  frame copies keep the resolution instead of re-sampling 1× VRAM;
- presenting the N× display buffer.

### Resolution scaling, step 2: frames as textures keep the resolution (2026-10-16)

The display blit, the colour filter, bloom and frame copies sample frame buffers as textures.
With step 1 alone they re-sampled 1× VRAM, so the displayed image was only an upscale:
0.0% of 2×2 blocks in the display buffer's N× copy differed. Now:
- **Tagging.** At record time, a primitive whose texture is a buffer with a current N× copy
  is tagged. It is not mipmapped and is CT32, CT24 or CT16. For currency, only the texture
  pages inside that buffer are checked; a power-of-two texture extent reaches past it.
- **Sampling.** The N× pass samples the run's own frame from the N× `prev` snapshot and one
  other buffer per list through a new `n3_src` binding (`n3_hires_tex`).
  - The stored frame value is turned back into a raw texel and goes through the texture's
    own conversion (TEXA/AEM).
  - Nearest uses `floor(u N + N/2)`, because a frame used as a texture has texel *i* at GS
    position *i*, so a 1:1 copy reproduces the N× frame exactly. Bilinear uses the GS
    weights at N× coordinates.
- **Sprites at N×.** Texture coordinates are interpolated from the GS sprite record at the
  neighbouring GS pixels, by the fragment's sub-pixel position. Coverage stays per GS pixel.
- **Local→local transfers** are mirrored between N× copies after their upload. The race does
  none, but the path is there.

Result at 2×: the display buffer's N× copy has 85-89% non-uniform 2×2 blocks (was 0%), and
the image is visibly smoother.

**Fast tests** (`crates/ssx3-ps2/src/gs/gpu/tests.rs`, about 1 s, no game run needed):
- `hires_keeps_vram_exact_and_renders_finer`: N× never changes VRAM; the 2×2 average of the
  copy matches 1× on a smooth texture; the copy has real detail.
- `hires_frame_as_texture_copies_exactly`: a 1:1 sprite copy of a frame used as a texture
  reproduces the N× frame bit for bit, and the 1× copy too.
- `hires_refreshes_after_vram_upload`: a host upload into a buffer shows up in its N× copy.

**Regression the suite caught.** Step 1 had restructured the 1× Z-plane expression
(`let` → `var` plus a branch). That changed the GPU compiler's FMA contraction, so Z bits
differed from the compute GS in `n3_dst_read_ct16_sprites_vs_compute`. The race replay never
hit it. The 1× expressions are now kept verbatim, with the N× evaluation in its own branch.
Rule for this shader: never restructure the 1× arithmetic.

### Resolution scaling, step 3: presenting at N× (2026-10-16)

`display_gpu` (the windowed game's path) now evaluates the display circuits at N× output
size with a `display_hires` entry point in `gs.wgsl`. It uses the same merge, alpha and
field logic as `display`, with one difference per circuit:
- if the circuit's buffer has an N× copy that covers the whole displayed rectangle and is
  current there, the circuit reads that copy (16- and 24-bit formats are reduced the way
  VRAM would hold them);
- otherwise it reads VRAM at the GS pixel each N×N block covers. A host upload into the
  shown frame after the last native run is one such case.

The output size stays N× once native runs have made copies, so the window never switches
resolution between frames. Screenshots (`Gs::display`) stay 1× and exact, so the A/B gates
are unaffected.

The presenter's buffer shader box-filters when the frame is larger than the window (an N×
frame in a small window). Otherwise it samples nearest as before.

`SSX3_HIRES_SHOTS=dir` also writes the presented frame (`frameNNNNN-display.png`).

**Fast tests:**
- `hires_display_shows_the_n_copy`: the 2× display equals the N× copy pixel for pixel, and
  the 1× display still equals VRAM.
- `hires_display_falls_back_to_vram_when_stale`: after a host upload, the display shows
  VRAM at nearest.

### Widescreen (2026-10-16)

SSX 3 has its own widescreen option (Options → Game). `cFEStateOptionsGame_Init` reads it
from the options word at `0x535610`, bits 20-21:
- 0 = off;
- 1 = 16:9, letterboxed inside a 4:3 image;
- 2 = anamorphic: a full-height image squeezed horizontally, meant to be stretched to 16:9.

The windowed presenter now fits the image to a display aspect ratio, letterboxing or
pillarboxing it inside the window instead of stretching it over the window. The ratio is
16:9 when the game is set to anamorphic and 4:3 otherwise. `SSX3_ASPECT=4:3|16:9|stretch`
overrides it. The game itself is untouched: no projection patches, so every replay and gate
is unaffected. Wider-than-16:9 field of view (a projection patch) is not done.

### Native performance (2026-10-16)

**Starting point.** Race replay frames 6000-6900 (headless, uncapped): 20.7 s emulated,
29.3 s with `SSX3_RENDER_NATIVE=all`.

**Measurement tools** (`SSX3_GS_WAITS=1`, printed at exit):
- latent-readback waits;
- GPU busy time, from submission/completion timestamps of every command buffer (a poller
  thread keeps completion callbacks prompt).

`SSX3_MTGS_STATS=1` shows how long the EE waits for the GS thread.

**What it showed:**
- The EE waited 55 s for the GS in native mode (22 s emulated), almost all of it at the
  flare queries' local→host reads, about 4 per race frame.
- Readbacks are latent: they return the previous frame's copy. But waiting for that copy
  used `poll(wait_indefinitely)`, which also waits for all of the current frame's drawing
  queued since. In native mode that was 24 ms per wait.
- The GPU is busy 99% of the time in native mode (89% emulated): both modes are GPU-bound
  at roughly 43 fps in the race. Throwaway experiments that skip parts:
  - the n3 draws cost about as much GPU as the compute rasterizer they replace;
  - the swizzle-back costs about 5 s per 1,100 race frames;
  - the unswizzle costs next to nothing.

**Changes:**
- **Submit thread** (`gs/gpu/submit.rs`).
  - `CommandEncoder::finish` (wgpu's validation and Metal encoding) and `submit` run off
    the GS thread. They were about a third of the GS thread in native mode.
  - Jobs carry their own buffer uploads, so queue order is preserved.
  - Everything that maps, polls or hands a buffer to the presenter drains it first.
  - `SSX3_GS_SUBMIT_THREAD=0` turns it off.
- **Targeted latent wait.** The latent copy's map is issued right after its submission
  (a post-submit hook on the job), and its `SubmissionIndex` is recorded. `latent_read`
  waits for that submission only. The data is the same, so the result stays
  deterministic. The EE wait dropped from 55 s to 40 s.
- **Swizzle back only what a run drew.** The run's rectangle is clipped to the union of its
  primitives' bounding boxes. Undrawn target pixels already equal VRAM.
- **Lazy coverage.** The "drawn since the last copy" bitmap is only consulted by later
  destination-read primitives. Shapes are queued and rasterized only when a test's
  bounding box overlaps them.

**Result.** Frames 6000-6900 with native rendering take 24.8 s (was 29.3 s). Emulated is
unchanged at about 20.7 s.

**Not done, and why:**
- **The n3 fragment cost is structural.** The shader writes `frag_depth` (GS-exact Z from
  the compute GS's planes), so the GPU cannot reject occluded fragments early. Every
  covered fragment runs the full GS texture/fog/alpha/blend emulation.
- **Lowering it needs a design change.** One option is a non-exact fast path for the
  native port. Another is hardware-interpolated Z where it provably matches the GS
  (`docs/plans/roadmap.md` phase 4 territory).
- **Most native runs end at texture uploads.** 44k of 47k runs end because the game
  streams textures through a small VRAM region it reuses within a frame. Continuing a run
  across such an upload needs versioned texture pages in the shader.

## Frame-phase measurements (2026-10-21)

_Moved from `plans/native-rendering.md` ("Using the matching C++ decompilation")._

Race call profile at 3,414 installed (frames 5002-5300, `callprof`): 70.2% of
the weighted work (`calls/frame * bytes`) is already hand-written. Of the
remaining 29.8%, 20.7% is asm-only, 6.1% no-symbol (libc/SDK) and 3.1% C++-backed,
so the hot queue is now mostly functions with no readable C++. Waves are
selected hot-first with `tools/pick_wave.py`; the remaining hot asm bodies are
ported directly from `generated/ee` (a literal translation).

Frame-phase timing, 2026-10-21 (6000-frame race replay, no screenshots):

| build | wall / frame | EE thread incl. frame sink | frame sink (game thread) | gfx thread (build/prepare/submit/readback) |
|---|---|---|---|---|
| desktop (`ee-threads`, 3,911 installed) | 4.18 ms | ~3.5 ms | 0.65 ms | 1.79 ms (0.79/0.25/0.59/0.16) |
| browser config (`ee-unwind`, 3,705 installed) | 2.88 ms | ~2.2 ms | 0.66 ms | 1.79 ms |

The gfx thread is not the critical path on desktop (2.4 ms of slack); the
frame is EE-thread-bound. But the EE *function bodies* are cheap to improve:
`SSX3_NO_OVERRIDES=1` A/B (3 runs each) gives 25.14 vs 25.65 s on desktop
(2.0%) and 17.28 vs 18.61 s on the browser config (7.7%) for the 82% of
weighted EE work already hand-written. Extrapolated, replacing *all* EE bodies
would save at most ~0.27 ms/frame (~9%) on the browser build, and the remaining
18% is worth ~0.05 ms/frame (~1.7%) at the same rate. The rest of the EE phase
is per-instruction overhead that both forms pay (bus accesses, cycle and event
bookkeeping), plus the frame sink (0.65 ms) and the renderer (1.8 ms).

A 6-second process sample mid-replay (16 host threads, 1,604 samples each)
shows the pipeline is latency-bound, not CPU-bound: ~87% of all thread-samples
are blocked in `__psynch_cvwait`/semaphore/workqueue waits. The only busy
threads are `ee-thread-1` (the game thread, 76% busy) and `ssx3-gfx` (55%);
the main thread is 93% waiting and the gs/submit threads are idle. Of the
actual non-wait work (3,423 samples):

| area | share of work |
|---|---|
| renderer (VIF replay/chain, VU-call planning, GPU draw) | 29% |
| malloc/free | 12% |
| libc/kernel misc | 10% |
| VU core | 9% |
| audio (SPU2 render/decode, snd_utk) | 6% |
| EE runtime (call_indirect, bus) | 6% |
| hash/HashMap | 5% |
| guest generated code | 4.5% |
| guest hand-written code | 2.7% |
| other (Metal/AGX, mmap, dispatch) | 16% |

So on the critical thread the biggest single item is the frame sink's VIF
chain replay + hashing (part of the 0.65 ms/frame sink), then the renderer's
VU planning on the gfx thread and per-frame allocation/hashing. Guest code in
both forms is ~7% of the real work.

So further hand-porting is completeness work, not a frame-time lever. If frame
time is the goal: the renderer (1.8 ms/frame, 62% of the browser-config frame)
and, on desktop, the emulated-thread synchronization (4.18 vs 2.88 ms/frame
between the two builds) are the levers; a faster memory/bus path or eliding
cycle/event bookkeeping in the transliteration would be the interpreter-side
one. The weighted-work figure at 4,007 installed is 82.0% (frames 5001-5300). A
re-profile at 19:35 produced an *identical* call graph (2,115 functions, 3,709
edges) because the replay is deterministic and the replacements are
behaviour-identical, so re-profiling is not needed to refresh the hot list; a
profile is only stale with respect to the *installed set*. Of the remaining
18.0%: 9.8% asm-only, 5.2% no-symbol, 3.0% C++-backed; the top 10 functions are
7.2% and the top 50 are 11.9% of race work.

Two exact ports are kept in their modules but not installed, because their
fixtures cannot reproduce real data: `RiderAirState_PostUpdate` (oc108; both
differential tests pass, but the race replay diverges on the board-contact arm)
and `ChaseCameraAlgorithm_ApplyAirSwing` (oc109; the spline evaluate spins
without a live spline). Re-audit those before re-adding their `OVERRIDES`.
As of 2026-10-22: 5,092 of the 11,844 functions are installed as typed Rust
(1,500 at the start of the session); 5,052 remaining functions are <=800 bytes
with a readable C++ body and no `__entries` dispatcher fragment, and 4,876 of
those are <=600 bytes. The small-function waves (50 per writer) are the best
yield; the 400-function dispatcher-heavy waves at the start of the session
yielded only 5-7 per writer.
