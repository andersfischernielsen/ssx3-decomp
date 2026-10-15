# Light glows and flares show through geometry (open, parked 2026-10-15)

Status: **parked by the owner; must be fixed before the project is done**
(final task in `../plans/roadmap.md`).

Symptoms: not only the sun. Every light effect that should be hidden by
geometry is drawn at full strength on top of it:

- the sun's glare/lens flare over the rider, halfpipe walls and the stadium
  roof (`flare-occlusion-sun.png`: Snow Jam race start);
- point-light glows and star flares through solid terrain: with the camera
  pressed against a dark rock face, a pink star flare and blue glows still
  draw over the rock (`flare-occlusion-rock.png`: Peak 3 freestyle, 0:42).

Everything else renders correctly. The likely common cause is the game's
visibility test for light sources (GS readback below): if it reports every
light as fully visible, every glow is drawn unoccluded. Check also whether
the glow sprites themselves rely on Z testing against the scene (ZTST/ZBUF
state, Z buffer contents and format, draw order) in case part of the effect
is a plain depth-test problem.

Findings so far:

- The game reads data back from the GS every frame: a VIF1 DMA towards memory
  (GS download path) of 24 or 48 qwords into **0x1467200** (~6,500 times in a
  6,000-frame run; `SSX3_TRACE_DOWNLOAD=1` logs them). This is presumably the
  sun visibility test (sample pixels/Z around the sun, scale the flare).
- `hw.rs` used to complete that DMA without copying anything ("Rare; complete
  empty."). It now drains the GS readback FIFO (`Gs::read_local`) into memory.
  That changes timing slightly (scripted menu navigation can land on a
  different event than with older binaries), but did not fix the glare.
- The buffer at 0x1467200 is **all zeros** after the download, with both the
  GPU GS and the CPU GS (`SSX3_GS=cpu`), so it is not the GPU readback path.

Next steps:

1. Trace the local→host transfer that precedes the download: BITBLTBUF
   (SBP/SBW/SPSM), TRXPOS (SSAX/SSAY), TRXREG (RRW/RRH), TRXDIR=1, and
   BUSDIR/VIF1 FBRST/STAT.FDR writes; check which VRAM region (frame or Z
   buffer, PSM) is read and whether it holds data at that moment (draw
   order: the sun test may need the scene drawn first; with MTGS, the read
   must see all queued draws).
2. Find the code that consumes 0x1467200 (Ghidra xrefs) and how it turns
   pixels into a flare intensity.
3. Compare with PCSX2 (PINE: the same buffer in PCSX2 RAM during a race) to
   confirm what the data should look like.

Note (2026-10-15): in PCSX2 (paused in a Snow Jam race, pause menu) the same
address 0x1467200 holds float records (e.g. 11.0, 5.0, 142.0, 16.0, 8.0) and
pointers like 0x00d63304, not pixel data. So either the download target is a
heap buffer that sits elsewhere in PCSX2's run (heap layouts differ between
runs), or it is reused while paused. Before comparing contents, find the
destination pointer in game code (the VIF1 download's MADR source) and read
it from PCSX2 at the same game state.

## Update 2026-10-16: the visibility test works; the sun case no longer reproduces

**How light visibility works.**
- Each frame, after the first chain phase, the render kick thread calls
  `LensFX_UpdateCallback` → `LensFX_UpdateSourceVisibility` (0x2e3130). It runs a query,
  `LensFX_v04` (0x2ec478), for every light source: the sun records at LensFX+0x6070, and the
  per-emitter glows.
- The query is a GS local→host copy of a 16×16 rectangle around the light from the **Z
  buffer** (`sbp` = ZBUF base 0x1c00, PSMZ24, 3 bytes per pixel). The bytes go to
  LensFX+0x6510 through the VIF1 download. That address is the 0x146db40 / 0x1467200 seen
  above: the LensFX object lives at 0x1467630.
- It counts the pixels inside the light's screen rectangle whose depth is ≤ the light's depth
  (record float 9).
- visibility = visible / total when the rectangle is fully on screen, otherwise
  `max(0, (visible − half) / half)`. The result is stored per source (+0x6274).

**What was checked.**
- **The readback is right.** `SSX3_TRACE_DOWNLOAD=1` now also logs each transfer's surface and
  value range. The game reads the real Z buffer: during the race-start sequence, values up to
  0x1c9313, mostly non-zero. Example, frame 4400 at the start gate: the sun's query (401,154,
  16×16) reads 256/256 pixels with depth around 0x3e800, far above the sun's depth of 3099. So
  the sun is occluded, visibility is 0 (+0x6274 = 0), and no sun glare is drawn.
- **Visibility drives the glows.** `SSX3_DEBUG_LENSFX_VIS=<f>` (`crates/ssx3-game/src/debug.rs`)
  forces every query result. At 0 there are no glows; at 1 the frame looks exactly like the
  bug report: a large sun glare over the rider and the stadium roof
  (`flare-forced-visibility.png`, left 0 / right 1, frame 4400).
- **The sun glare no longer reproduces.** Race-start frames 4480-4760, including 00:00:01,
  the moment in `flare-occlusion-sun.png`, show no glare through the roof. With
  `SSX3_GS_LATENT_READBACK=1` (the old default), only a small glow at frame 4480 differs.

The earlier symptom was produced by a query that saw "fully visible", which is what an empty
(all-zero) or stale readback gives: every pixel's depth 0 is ≤ the light's depth. The VIF1
download now delivers the FIFO and readbacks are synchronous by default, so the query sees
the current Z buffer.

**Still open.** The point-light glows through rock (`flare-occlusion-rock.png`, Peak 3
freestyle, 0:42) were not re-tested: the run's inputs are not recorded. Next step: reproduce a
camera-against-terrain shot with a light behind it, with `SSX3_TRACE_DOWNLOAD=1`, and check
that light's query and depth. A rock clipped by the near plane would leave no depth there, so
the light would count as visible: compare with PCSX2 before treating that as a bug. Until this
is confirmed in the windowed game, the final task stays open.
