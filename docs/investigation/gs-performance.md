# GS rasterizer performance

The software GS (`crates/ssx3-ps2/src/gs/`) was the bottleneck of the recompiled
game. A macOS `sample` profile of a heavy in-game frame (about 30,000 primitives,
roughly 2 fps) spent more than 95% of its time in `raster::pixel`,
`raster::texel`, `swizzle::Surface::read/write` and `raster::draw`. This note
covers the capture/replay benchmark, the changes, and the results. Rendering
output is bit-identical to the original rasterizer.

## Benchmark: capture and replay

- **Recording.** `Gs::start_capture()` snapshots the whole GS: VRAM, privileged
  and general registers, GIF path state, CLUT, transfer state and the readback
  FIFO. It then logs every externally driven call (`gif_write`, `write_reg`,
  `write_priv`, `vblank`, `read_local`, `raise_event`). `Gs::finish_capture()`
  returns the trace as a small hand-rolled binary format, described in
  `gs/capture.rs`. `capture::Trace::parse` restores the GS and the event list.
- **Headless runner.** `SSX3_GS_CAPTURE=start:n:path[,start:n:path...]` records
  frames `start..start+n-1`. For example:

  ```sh
  SSX3_GS_CAPTURE="2380:4:/tmp/charsel.gstrace,4050:4:/tmp/course.gstrace" \
  SSX3_INPUT="950:0010:8,1000:4000:8,1600:0008:8,2000:4000:8,2500:4000:8,2700:4000:8,2900:4000:8,3100:4000:8,3300:4000:8,3500:4000:8,3700:4000:8" \
    ./target/game/ssx3-headless 4100
  ```

  Adding `4250:4000:8` to `SSX3_INPUT` starts the race, so frames 4500 and 4800
  are in-race.
- **Replay.** `cargo run --release -p ssx3-ps2 --bin gs_replay -- TRACE [--iters N] [--png out.png] [--write-golden F] [--golden F] [--sync]`
  replays a trace in deferred mode (`--sync` turns that off). It reports ms per
  frame (VBlank to VBlank) and the batching statistics. After every VBlank it
  hashes all 4 MiB of VRAM, then writes or compares those golden hashes. Thread
  count comes from `SSX3_GS_THREADS` (default: all cores).

Traces are 7–24 MB and stay out of git.

## Results

Apple M1 (4 performance and 4 efficiency cores). Each figure is the best of 5
replays. Another emulator process held one core at 100% throughout, so the
8-thread numbers are slightly pessimistic.

| Trace (4 frames) | Prims/frame | Before | After, 1 thread | After, 8 threads | Speedup |
|---|---:|---:|---:|---:|---:|
| `charsel` (character select, 2380) | 5.1k | 47.1 ms | 24.8 ms | 8.6 ms | 5.5× |
| `course` (race briefing over the 3D course, 4050) | 29.4k | 534 ms | 131 ms | 41.8 ms | 12.8× |
| `race4500` (in race) | 54.2k | 429 ms | 117 ms | 39.8 ms | 10.8× |
| `race4800` (in race) | 43.5k | 251 ms | 93.6 ms | 34.6 ms | 7.2× |

"Before" is the original rasterizer. The binary was built from the commit that
added capture/replay, before any optimization.

**Correctness.**
- All four traces replay to bit-identical VRAM after every frame, against
  golden hashes from the original rasterizer.
- This holds with 1 and 8 threads and in synchronous mode.
- `cargo test -p ssx3-ps2` passes, including the original 28 GS tests.

**End to end.** `ssx3-headless` with the race input script reached frame 4020
after 61 s instead of 526 s. That run was made before the last few
optimizations.

## What changed

### Single-thread pipeline (`raster.rs`)

- **Setup and drawing are split.** `setup` turns a kicked primitive into a
  `Prim`: clipped bounding box, edge functions and attribute planes. `raster`
  draws any subset of its rows.
  - Every per-pixel value is computed from the row and column directly, never
    accumulated, so the f64 attribute math is the same as before.
  - Triangle spans per row come from exact integer edge intersections, instead
    of testing every pixel of the bounding box.
- **Specialized span loops.** These are monomorphized on the texturing mode:
  none, nearest or bilinear, each from VRAM or from decoded texels.
  - Per-draw state is classified once, for example the blend classes
    `Src` = (X−X)·C+Cs and `Lerp` = (Cs−Cd)·C+Cd.
  - ATST=ALWAYS is dropped.
  - Z is skipped when ZTST=ALWAYS and ZMSK is set.
- **Early depth and destination-alpha tests.** Nothing is written before these
  tests, and a pixel that fails either has no effect at all. So they run before
  texturing, and the fragment's colour, texture coordinates and fog are only
  computed for pixels that pass. In the course frame, 53% of pixels fail the
  depth test.
- **Addressing.**
  - `SurfRow` precomputes everything that depends only on y for frame, Z and
    texture rows.
  - Frame and Z words are located once per pixel, for both the read and the
    write.
- **Texture sampling.**
  - Bilinear filtering does two channels per u64.
  - Texels with zero weight are not fetched. The result is unchanged, and the
    set of texels a primitive can read is exact.
- **Decoded texture cache (`texcache.rs`).**
  - A texture's whole reachable rectangle is decoded to RGBA8 once (CLUT and
    TEXA applied) and sampled with plain array loads.
  - Each 8 KiB page has a write generation, bumped by drawing, host-to-local and
    local-to-local transfers. A cache entry is used only while none of its
    pages has changed. Pages of an unfinished upload are never cached.
  - A texture is decoded only if it will be sampled at least as many times as
    it has texels. Large decodes run on the worker pool.
  - Code that writes `Gs::vram` directly must call `Gs::vram_written()`.
- **Host-to-local uploads** write through cached row addressing.

### Parallel rasterization (`batch.rs`, `pool.rs`, `region.rs`)

**Batching.**
- Kicked primitives are queued in a batch instead of being drawn immediately.
- A batch is drawn by 64 units. Unit `u` owns the 8-row bands `b` with
  `b % 64 == u`.
- Each primitive is binned into the units its rows touch. A unit draws its
  primitives in submission order, so every pixel sees exactly the original
  order.
- Units are handed out dynamically by a persistent pool (`std::thread`,
  condvar plus a short spin), which balances the fast and slow cores.

**Dependency tracking.** The batch tracks the 256-byte VRAM blocks that
primitives write (bounding boxes in the frame and Z buffers) and read
(textures):
- Write sets are kept as page sets plus rectangles. The block-level set is only
  built when a page-level test hits.
- For a primitive whose texture overlaps written blocks, the texels it can reach
  are computed exactly from its UV or STQ range and the wrap mode. This matters
  for SSX 3's REGION_REPEAT textures, whose reach wraps around the end of VRAM.

**Flush rules.** A batch is flushed (drawn) before:
- **Primitive dependencies.**
  - A primitive that reads blocks the batch writes (read after write).
  - A primitive that writes blocks the batch reads (write after read).
  - A primitive whose frame or Z buffer overlaps pages written through another
    mapping (base, width or format).
- **Memory traffic outside the rasterizer.**
  - A CLUT load from blocks the batch writes.
  - Host-to-local data for blocks the batch touches.
  - Every local-to-host or local-to-local transfer.
- **Sync points.** CSR reset, `vblank()`, `sync()`, and, unless deferred mode
  is on, the end of every `gif_write`/`write_reg` call.

**Primitives that cannot be split by rows.**
- These are buffers in formats whose words span rows, pixels beyond the buffer
  width (they wrap into other rows), and textures that alias their own output.
  They are drawn alone.
- Two refinements avoid that:
  - **Row-local reads.** A sprite whose texture maps texel (x, y) to the word of
    pixel (x, y) only reads from its own 8-row band, so it stays parallel. This
    covers full-screen copies and colour-lookup passes.
  - **Splitting.** A primitive that reads blocks only its first rows write is
    split. Those rows are drawn first and the rest is queued.

**Background drawing.**
- A flushed batch is drawn in the background while the GS keeps decoding GIF
  data into the next batch. The next flush waits for it, and the main thread
  helps.
- VRAM accesses from the main thread (uploads, CLUT loads, decoding) wait only
  if they touch the in-flight batch.
- Primitives that sample decoded copies no longer read their textures from
  VRAM. Texture uploads therefore do not wait for them, which is common
  because SSX 3 streams textures into reused slots.

**Deferred mode.**
- `Gs::set_deferred(true)` lets batches span `gif_write` calls. The machine
  enables it, and `machine::run_frame` calls `gs.sync()` before returning, so
  `display()` and `vram` are current for the front ends.
- With deferred mode off (the default), every public call returns with `vram`
  complete, as before.

### Verification tooling

- `gs/diff_tests.rs` runs a differential test against the original rasterizer.
  That rasterizer is kept as `gs/ref_raster.rs`, test-only.
  - It uses random register states: all frame, Z and texture formats, blend,
    test and wrap modes, and textures placed on the frame buffer.
  - It draws random points, lines, triangles and sprites, including copy-style
    sprites, and interleaves split host-to-local uploads.
  - It compares VRAM word for word.
  - Breaking row ownership or upload dependency checks makes it fail.
- `gs/region.rs` has a test that checks block sets against per-pixel addresses
  for every format, including unaligned base pointers.

## Where the time goes now

Single-threaded, in the race frames, most time is in the textured triangle span
loop: bilinear sampling of decoded texels, the f64 plane equations and the
per-pixel blend/Z path.

With 8 threads, the main thread is the limit:
- It decodes GIF data, sets up primitives and tracks dependencies serially.
- Texture uploads into slots that queued primitives sample force flushes.
- There are about 6–9 flushes per frame in races.
- Workers are idle about a third of the time.

Further gains would need:
- SIMD span processing, for example 4 pixels at a time with fixed-point
  attributes. That changes rounding, so it would need a new baseline.
- Moving setup off the main thread.
