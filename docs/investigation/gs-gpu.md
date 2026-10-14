# GPU GS backend (wgpu compute)

Module: `crates/ssx3-ps2/src/gs/gpu/` (`mod.rs`, `gs.wgsl`, `tests.rs`).
Select it at run time with `SSX3_GS=gpu` (default `cpu`). The CPU
rasterizer (`gs.md`, `gs-performance.md`) stays the reference.

The design is a port of ps2-web-recomp's WebGPU GS
(`runtime/src/gs/gpu/gpu.cpp` and `gs.wgsl`, MIT, Copyright (c) 2026 Samir
Nuri). The attribution is repeated in both source files.

## Design

**What runs where.**
- The CPU keeps everything except pixels: GIF parsing, registers, the vertex
  queue, CLUT loads, transfers and primitive setup. Setup is the CPU
  rasterizer's own `raster::setup`, so coverage and attribute planes come
  from the same code.
- Each set-up primitive is encoded into a record. `Draw` state becomes a
  "setup" record with the palette already resolved.
- The GPU runs the whole per-pixel pipeline of `raster.rs` in a compute
  shader:
  - scissor and SCANMSK
  - texturing: all PSMs, CLUT/TEXA, every wrap mode, nearest/bilinear and
    mipmapping (MIPTBP/MTBA levels, LOD from Q or K)
  - TFX, fog, alpha test with every AFAIL mode, DATE
  - Z test and write for Z32/24/16/16S
  - blending with PABE, dithering, COLCLAMP, FBA, FBMSK
  - frame formats CT32/24/16/16S
- VRAM is a 4 MiB GPU storage buffer in the native swizzled layout. The
  swizzle tables are uploaded once.

**Batches and tiles.**
- Primitives queue into a batch, and each batch is one compute dispatch.
- On the CPU, primitives are binned into 8x8 pixel tiles. Triangles are
  narrowed per tile row with their edge functions, and lines are bounded by
  their endpoints.
- One 64-thread workgroup runs per tile, one thread per pixel. It walks the
  tile's list in submission order, so every pixel sees primitives in the
  original order. Bounding boxes are staged in workgroup memory.
- Sub-word writes (16-bit frames, CT24, T8H and similar) use `atomicAnd` and
  `atomicOr`. 8x8 tiles measured about 15% faster than the reference's
  16x16.

**When a batch is closed early.**
- **Read after write.** A primitive samples blocks the batch has written.
  This is block-exact, using `raster::prim_tex_rect`.
- **Aliasing.** A primitive draws to pages that the batch draws through a
  different buffer mapping.
- **Self-aliasing primitives** get a batch of their own. These are pixels
  beyond the buffer width, frame/Z overlap, or pages wrapping around VRAM.
- **Transfers** that touch the batch's blocks.

**Texture reads.**
- Before each batch, the pages the batch samples are copied into a snapshot
  buffer. Textures are read from that snapshot with plain loads, never from
  the VRAM being drawn. As a result, a primitive that samples its own
  target, or overwrites what an earlier primitive sampled, reads
  pre-batch data.
- Textures sampled about as many times as they have texels are decoded on
  the GPU to linear RGBA8 first, as the CPU texture cache does. The decode
  covers each mip level's whole wrap range. Decoded copies are reused
  within a submission until their pages are written.

**VRAM ownership.** `Gs::vram` becomes a mirror, tracked with two page sets:
- `cpu_dirty`: pages written by host->local transfers. These are uploaded as
  whole pages in front of the next batch.
- `gpu_dirty`: pages drawn on the GPU.

The common path is draw → vblank → `display_gpu`. It never reads VRAM
back. Readbacks happen only when the CPU must see GPU-drawn data:
- CLUT loads from drawn pages.
- local->local transfers.
- `sync_vram()`.
- Uploads that only partly cover a drawn page.

A readback fetches every GPU-drawn page at once.

Two common cases avoid even that:
- **Uploads into drawn pages.** If an upload covers whole pages with
  whole-word formats, no readback is needed. Partial-word formats (CT24,
  T8H, T4HL/HH) are written by a GPU compute pass instead (`pixel_write`).
  SSX 3 uploads T8H alpha maps into drawn buffers on the character-select
  screen.
- **Local->host (latent readback).** SSX 3 reads small rectangles of the Z
  buffer around each light every race frame, for flare occlusion. A
  synchronous readback would make the CPU wait for the whole frame's
  drawing. Instead the read is served from a copy of those pages taken at
  the end of the previous frame, so it is at most one frame stale. The
  first frame gets whatever the mirror held. `SSX3_GS_SYNC_READBACK=1` (or
  `Gs::set_latent_readback(false)`) restores exact readbacks. In the
  captured traces this produced no image difference.

**Display.**
- The PCRTC (both circuits, merge, SLBG/MMOD, field mode) is evaluated by a
  compute pass into an RGBA8 buffer.
- `Gs::display_gpu()` returns that buffer (`GpuFrame`). The windowed front
  end presents it with the GS's own wgpu device (`present.rs`,
  `render_gpu`), so frames never leave the GPU.
- `Gs::display()` still works on the GPU backend: it runs the same pass and
  reads back only the image. Screenshots (F12) and headless shots use this.

**Submission.**
- `sync()` and `vblank()` submit without waiting.
- At most three submissions are in flight.
- The machine's per-frame `gs.sync()` therefore costs only encoding.

**API additions.**
- `Backend`, `Gs::set_backend`, `backend`, `new_cpu`
- `gpu_stats`, `gpu_context`, `sync_vram`, `gpu_wait_idle`, `display_gpu`,
  `set_latent_readback`
- `gpu::context()`, which gives the shared device
- `display()` now takes `&mut self`
- Code that reads `gs.vram` directly must call `sync_vram()` first. The
  headless `SSX3_VRAM_DUMP` and the capture start do. Code that writes it
  must call `vram_written()`, which on the GPU makes the mirror replace the
  GPU copy.

**Debug knobs.**

| Variable | Effect |
|---|---|
| `SSX3_GS_PROFILE=1` | GPU timestamp per raster pass (`gpu_raster_ns`) |
| `SSX3_GS_PROFILE=2` | Also prints each pass |
| `SSX3_GS_TRACE_READBACK` | Logs readbacks and their cause |
| `SSX3_GS_CHECKED=1` | Keeps wgpu's shader bounds checks |
| `SSX3_GS_NO_DECODE` | Disables decoded textures |
| `SSX3_GS_NO_DIRECT` | Disables GPU-side partial-word uploads |
| `SSX3_GS_WGSL=path` | Loads the shader from a file |

## Validation

`gs_replay TRACE --compare [--diff-dir DIR]` replays a trace on both
backends side by side. It compares the displayed image after every VBlank
and reports four figures:
- pixels that differ at all
- pixels whose largest channel difference is >8
- pixels whose largest channel difference is >32
- the mean absolute channel difference

At the end it also reports the fraction of VRAM words that differ.

Results for the scratchpad traces, all frames (5 images of 512x448 each):

| Trace | differ | >8 | >32 | mean abs diff | final VRAM words differing |
|---|---:|---:|---:|---:|---:|
| `charsel` | 0.005% | 0.000% | 0.000% | 0.0000 | 0.003% |
| `course` | 0.009% | 0.001% | 0.000% | 0.0004 | 0.14% |
| `race4500` | 0.199% | 0.025% | 0.006% | 0.0067 | 1.04% |
| `race4800` | 0.429% | 0.045% | 0.010% | 0.0096 | 0.96% |

The worst single frame is `race4500` frame 2, with 0.66% of pixels
differing. Those pixels are on thin, heavily minified rails and track edges,
where texel selection sits on a boundary. The images are visually
identical; see `--diff-dir`.

**End to end.** `ssx3-headless` was run to frame 4800 with the race input
script, on each backend, with a screenshot every 300 frames. Emulation is
deterministic: both runs end at the same cycle count. All 16 screenshot
pairs match:
- at most 0.27% of pixels differ (frame 3600)
- typically 0.001–0.09% differ
- no pixel differs by more than 8 in any channel

**Unit tests** (`gs/gpu/tests.rs`) skip when no adapter exists. The first
three must match exactly:
- flat and Gouraud primitives
- a textured 1:1 copy plus a T8/CLUT scaled sprite, including the
  displayed image
- a local->host readback after GPU drawing

A fourth test runs the CPU differential test's random scene generator. It
needs fewer than 0.5% of VRAM words to differ; the measured figure is
0.19%.

### Why results are not bit-exact

- **f32 instead of f64.** WGSL on Metal has no f64. Colour, STQ, fog and Z
  planes are interpolated in f32, relative to the bounding-box origin. Z is
  stored as an integer base plus an f32 delta to keep its precision.
  Truncation can therefore differ by one LSB, and floor() of a texture
  coordinate can flip at exact texel boundaries.
- **Sprite texture steps** are also in f32.
- **Primitives that read their own output** read pre-batch data (the
  snapshot). The CPU draws such primitives serially row by row, so later
  rows see earlier rows' results.
- **Lines** find their pixel by searching for the DDA step (±1 around a
  guess), not by walking the DDA. A pixel the DDA visits twice is blended
  only once.
- **Flare Z reads** are one frame late (see above).

## Performance

Apple M1 (8 GPU cores), on a quiet machine. Figures are the best of 3
replays, as the mean over the full frames of each trace.

| Trace | CPU backend (8 threads) | GPU backend: GS CPU side | GPU backend: GPU raster time | GPU backend, serialized |
|---|---:|---:|---:|---:|
| `charsel` | 7.0 | 6.5 | ~8 | 11.2 |
| `course` | 35.6 | 13.9 | ~16 | 34.8 |
| `race4500` | 36.8 | 19.1 | ~13 | 38.1 |
| `race4800` | 31.3 | 17.1 | ~10 | 33.4 |

All figures are ms/frame.
- **GS CPU side** is GIF, setup, binning, encoding and submission
  (`gs_replay --gpu --no-gpu-wait`).
- **GPU raster time** is the timestamp-query total divided by the frame
  count (`SSX3_GS_PROFILE=1`). It includes GPU clock ramp-up, because
  profiling waits after every submission.
- **Serialized** is `gs_replay --gpu`, which waits for the GPU at every
  VBlank, so it measures the sum of CPU side and GPU time.

In the emulator, CPU and GPU work overlap across frames. The GS then costs
about max(CPU side, GPU) on the emulation thread, roughly half of the CPU
backend, and it leaves the other cores free. The CPU backend instead keeps
all 8 cores busy rasterizing.

**Whole game**, `ssx3-headless` with the race input script, screenshot
every 300 frames, quiet machine:

| | CPU GS | GPU GS |
|---|---:|---:|
| Whole run to frame 4800 | 117.0 s | 105.3 s |
| Course briefing, frames 3900–4200 | 29.1 s | 27.2 s |
| In race, frames 4500–4800 | 15.3 s (19.6 fps) | 13.5 s (22.2 fps) |

**The emulator is no longer GS-bound.** In a `sample` profile of the
in-race section with the GPU backend:
- VU1 microcode emulation dominates: `vu::float::add`/`madd`,
  `commit_stage`, `stage_fmac` and the generated VU program.
- All GS functions together (push, setup, page/region bookkeeping, close)
  are roughly 15% of busy samples on the emulation thread.

60 fields/s in races therefore needs the VU side to get faster. The GS on
the GPU has headroom: about 10–16 ms of GPU time, overlapped with the CPU.

The windowed front end with `SSX3_GS=gpu` runs menus at about 55–60 fps
(paced) and presents from the GPU buffer.

**What made the shader fast**, in raster-pass time on `race4800`, 4
frames:

| Change | ms |
|---|---:|
| First working port | 177 |
| Lines binned by their endpoints, not the whole scissor (2M → 265k tile entries) | 142 |
| No wgpu bounds checks or loop bounding (`create_shader_module_trusted`) | ~20% less |
| Sampling as loops with one texel fetch site; exact f32 bilinear weights | 82 → 73 |
| 8x8 tiles | 73 → 66 (`race4500`) |

The `create_shader_module_trusted` change is safe because every index is
masked or CPU-encoded.

Things that did not help, or made it worse:
- Caching the draw state in a private struct. Large private structs spill.
- Keeping the frame/Z element per thread across a batch.
- Removing atomics: tested in isolation, with no gain.

The kernel is latency-bound. Full-screen passes cost about 1 ns/pixel.

## Gaps and follow-ups

- **GPU throughput** of about 14–16 ms per heavy frame is close to the
  16.7 ms budget.
  - Specialized pipelines (WGSL `override` per batch feature set) showed
    little gain once the sampling code was restructured.
  - Batches that mix many tiny triangles leave most of each workgroup idle.
- **Uploads interleaved with drawing.** A primitive drawn into part of a
  page while a whole-word upload to that page is in progress can be
  overwritten when the upload's page is re-sent. An upload abandoned
  midway can also upload stale mirror words for the rest of its pages.
  Neither occurs in the traces.
- **Unordered aliasing inside one primitive.** Pixels beyond the buffer
  width, frame/Z overlap with different mappings, and buffers wrapping VRAM
  get their own batch but are unordered inside it.
- **Small transfer chunks.** GIF delivers images in small chunks.
  Consecutive partial-word writes are coalesced into one pass. Thousands of
  separate tiny passes once lost the device, before coalescing was added.
- **CPU side.**
  - Fixed-size 41-word primitive records give about 27 MB per 4 frames of
    `queue.write_buffer`.
  - `Pages::rect` and `Region::add_rect` per primitive for dependency
    tracking.
  - Both could be trimmed further.
- **Untested environments.**
  - The front end's sRGB handling of the buffer path follows the texture
    path's (decode to linear for sRGB surfaces). It was not checked against
    a screenshot of the window.
  - No other GPU vendor was tested.
