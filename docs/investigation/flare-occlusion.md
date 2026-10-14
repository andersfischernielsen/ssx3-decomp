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
