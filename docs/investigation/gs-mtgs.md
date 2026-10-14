# MTGS: the GS on its own host thread (2026-10-14)

Before this change, the guest render thread (`ee-thread-6`, which runs VU1
and kicks GIF data) did all GS CPU work itself. That covered GIF parsing,
register writes, primitive setup, `GpuGs::push`, page and region
bookkeeping, uploads and the GPU submission at VBlank. In a race frame this
was about 2400 of the thread's ~5750 busy samples. The GS now runs on a
dedicated host thread named `gs`, in the style of PCSX2's MTGS. The guest
only queues work and moves on.

**Result:** the scripted 9000-frame run takes **88 s instead of 171 s**.
Race frames 4320–4680 run at **80 fps instead of 40**. All 18 screenshots
are byte-identical to the baseline, with MTGS on and with `SSX3_MTGS=0`.

## Design

Module: `crates/ssx3-ps2/src/gs/mtgs.rs`. `Hw::gs` is now an `Mtgs`, which
wraps the real `Gs`.

- **Queue.** Each guest call becomes a `Cmd`: `Gif{path,len}`, `Reg`,
  `Priv`, `Vblank`, `Raise`, `Sync`, `Path3Reset` or `Display`. A GIF
  payload is copied into the current packet's data buffer. A packet is
  handed to the GS thread when it holds 2048 qwords or 512 commands, or at
  `vblank`, `sync`, a display request, or before any sync point. The
  hand-off is a mutex-protected `VecDeque` plus a condvar. Used packets are
  recycled through a free list, so steady state does not allocate.
- **GS thread.** It pops packets and replays them into the `Gs`, which sits
  in `Mutex<Gs>`. The `Gs` receives exactly the calls it received before,
  in the same order and with the same data chunks. Rendering, the GPU
  backend's batching and submission, and captures are therefore unchanged.
  The GPU backend now submits from the GS thread, using the same wgpu
  device.
- **Back-pressure.** At most 256 packets may be in flight. Past that, the
  guest waits until half of them are done. In the race this costs nothing:
  the guest is the bottleneck. It does trigger in some loading and menu
  phases, about 11 s of waiting over the whole run.
- **Waiting.** The guest spins briefly (4000 iterations) and then sleeps on
  a condvar. The GS thread signals that condvar only when a waiter is
  registered.
- **`SSX3_MTGS=0`.** This runs every command directly on the calling thread
  (`Mode::Direct`), which gives the old synchronous behaviour.
- **`SSX3_MTGS_STATS=1`.** This prints packet, sync and wait counts at
  exit.

## What the guest can observe, and how each case is answered

The rule is that nothing the guest sees may depend on how far the GS
thread has got. Each guest-visible value is therefore either kept on the
guest side, or read after a full drain (a sync).

| Guest observation | Answered from |
| --- | --- |
| GIF_STAT, PATH3 mask/queue (M3R, M3P, P3Q, FQC) | Guest side. The PATH3 arbitration (`path3_dma`, `set_path3_mask`, `path3_flow`, `set_path3_mask_reg`, `path3_reset`) moved into `Mtgs`. It hands the GS the same `gif_write(3, chunk)` calls that `Gs::path3_dma` made. |
| PMODE…BGCOLOR, IMR, BUSDIR reads, and 32-bit read-modify-write stores | A guest-side shadow (`Mtgs::pr`). `write_priv32` does the bus merge. |
| CSR event bits and FIELD, SIGLBLID, `irq_pending` (the GS interrupt) | Guest side. VBlank, `raise_event`, CSR acks and CSR.RESET are applied at queue time. SIGNAL, FINISH and LABEL depend only on the data, so they are applied as GIF data is queued. See below. |
| Local→host data (`read_local`, the GS FIFO at 0x10006000) | Sync |
| `display()`, `vram`, captures, `sync_vram`, backend switch, statistics | Sync, through `Mtgs::core()`, which drains and then locks the `Gs` |
| `display_gpu` (windowed front end) | Queued `Cmd::Display`. The front end takes the previous request's frame (see below). |
| `prims_drawn` (status line every 60 frames) | Waits for the packets already sent. Does not force a new one. |

**SIGNAL, FINISH and LABEL.** These are the only GS register writes that
change guest-visible state. They can arrive as A+D in PACKED mode on any
path, or through `write_reg`. REGLIST descriptors cannot address them. The
guest side keeps a copy of every path's GIF tag state, and
`GifPath::scan` advances it over each queued chunk:
- Runs without A+D descriptors, REGLIST data and IMAGE data are skipped in
  bulk.
- A+D qwords are inspected one at a time.

This gives exactly the effect `Gs::write_state_reg` has, at exactly the
point in the guest's execution where the synchronous GS applied it. The
interrupt therefore fires at the same EE cycle. The same tag mirror also
gives the PATH3 packet boundaries for the mask logic. CSR.RESET clears the
mirrored tags, as `Gs::reset` does.

**Consistency check.** `SSX3_MTGS_CHECK=1`, which is always on in tests,
asserts at every sync that the mirrored tags, CSR bits and SIGLBLID equal
the `Gs`'s own.

**Captures.** `SSX3_GS_CAPTURE` calls `core().start_capture()`. It
snapshots the `Gs` after a drain, and the `Gs` then records the calls it
receives. These are the same calls as before, including the post-arbitration
PATH3 chunks, so the trace format and `gs_replay` are unchanged. The PATH3
queue was never part of a snapshot.

**Local→host in practice.** In SSX 3, `read_local` is never called
(`sync_read_local: 0`). VIF1 to-memory downloads complete empty in this
emulator, and the game's per-frame Z-buffer reads for the light flares only
fill the GS readback FIFO. In the GPU backend, those reads are served from
the previous frame's latent copy, and whether that copy has arrived depends
on GPU timing, exactly as before. The data is not observable by the guest
today. The WIP version of this change waited for the copy instead. That
made the GS CPU side 30–60% slower in `gs_replay --gpu --no-gpu-wait` and
was reverted.

## Windowed front end

`main.rs` used to call `display_gpu()` after each `run_frame`, which would
now be a full sync every frame. It now works like this:
- `request_display()` queues a `Cmd::Display` behind the frame's drawing.
- `take_display(prev_ticket)` waits only for the previous frame's request
  and presents the newest frame that has run.

The guest therefore runs at most one frame ahead of the GS, and the window
shows frame N-1, or N if it is already done. `None` means nothing new
arrived, and the window keeps its image. `Some(None)` means nothing is
displayed or the CPU backend is in use, and the front end falls back to
`core().display()`. The CPU-backend path and F12 screenshots use
`core().display()`, which is a sync. A 900-frame windowed smoke run is
paced at ~55 fps and exits cleanly.

## Single-thread wins, both modes

These apply with and without the GS thread:
- **IMAGE data in bulk.** `Gs::gif_write` hands a whole run of IMAGE qwords
  to `transfer_bytes` at once. It no longer goes qword by qword.
- **Uploads.** `transfer_bytes` decodes pixel values into a reused buffer.
  It then writes them per row with `SurfRow::write_run`, which dispatches
  on the format once per run instead of through a per-pixel closure.
- **Triangles.** `Shape::Tri` holds its `TriSetup` inline. Previously one
  `Box` was allocated and freed per triangle.

`gs_replay` CPU side (`--gpu --no-gpu-wait`, ms/frame, mean of the full
frames, best of 2×5 iterations; the machine was busy):

| trace | base | after |
| --- | ---: | ---: |
| `charsel` | 6.8 | 1.9 |
| `course` | 13.1 | 10.3 |
| `race4500` | 18.0 | 13.0 |
| `race4800` | 16.3 | 11.3 |

**Rendering is unchanged:**
- VRAM golden hashes match the base binary after every frame of all four
  traces, on the CPU backend and on the GPU backend.
- `gs_replay --compare` output (CPU vs GPU difference figures) is identical
  for all four traces.

## Numbers (whole game)

Command:
```sh
SSX3_INPUT="1600:0008:8,2000:4000:8,2500:4000:8,2700:4000:8,2900:4000:8,3100:4000:8,3300:4000:8,3500:4000:8,3700:4000:8,4250:4000:8,4600:4000:3000,7700:4000:3000" \
SSX3_SHOTS=dir SSX3_TIMEOUT=1500 ./target/game/ssx3-headless 9000 --shot-every 500
```

This is the GPU GS (the game default). Runs were made back to back on an
M1 with other jobs running.

| build | whole run | frames 4320–4680 | race fps | screenshots vs baseline |
| --- | ---: | ---: | ---: | --- |
| base (4f57718) | 171 s | 9.1 s | 39.6 | (baseline) |
| this change, `SSX3_MTGS=0` | 138 s | 8.8 s | 40.9 | 18/18 identical |
| this change, MTGS | **88 s** | **4.5 s** | **80.0** | 18/18 identical |

All three runs end at the same cycle count (44281M) with the same number
of primitives (108291638).

**Profile.** A `sample` of the race section with MTGS, 924 samples per
thread, gave these busy samples:

| thread | busy samples |
| --- | ---: |
| `ee-thread-6` (VU1 and GIF kicks) | 652 |
| `ee-thread-1` | 185 |
| `ee-thread-8` | 49 |
| `gs` | 414 |
| main | 22 |

The guest threads, serialised by the baton, are busy about 99% of the
time. The GS thread is busy about 45%, so in races the GS is off the
critical path. Guest-side MTGS overhead (tag scan, copying into packets,
kicks) is under 1% of samples. The next lever for frame rate is the guest
itself: VU1 microcode and the EE.

## Known limits

- **Duplicated PATH3 logic.** `Gs` keeps its own PATH3 arbitration
  (`Gs::path3_dma` and the related methods) for direct users: `gs_replay`
  and the unit tests. The two copies of the logic must stay in step;
  `mtgs_tests.rs` runs random sessions through `Gs`, `Mtgs` direct and
  `Mtgs` threaded, and compares every observation and the final VRAM.
- **Latency.** In headless runs, back-pressure lets the GS lag up to 256
  packets, which is several frames. Anything that reads GS state goes
  through a sync, so this is invisible to the guest and to the output.
- **HSYNC/EDW.** `raise_event` is applied on the guest side. If a future
  GS-internal event needs to raise an interrupt asynchronously, it must be
  modelled on the guest side as well, or become a sync point.
