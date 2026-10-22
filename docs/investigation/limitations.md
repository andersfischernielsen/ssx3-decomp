# Known limitations (2026-10-22)

## Game

- **Light glows and flares draw through geometry** in places (sun glare,
  point-light glows): [`flare-occlusion.md`](flare-occlusion.md). Required
  before the project is called done.
- **Online** reaches the network configuration screen; there is no network.
- **`SSX3_HOST_CLOCK=1`** (game time follows the host when it falls behind)
  can stall the game on a host that stays behind; off by default and
  superseded by the window's catch-up (`SSX3_CATCHUP`,
  `docs/plans/configurable-tick-rate.md`).
- Paths never scripted are covered only by the hand-written functions'
  differential tests and random menu input
  ([`coverage.md`](coverage.md), `tools/scenarios.py`).

## Runtime

- **Still a PS2-shaped runtime around the game code:** guest RAM, the EE
  kernel (guest threads as coroutines on one host thread), DMAC/INTC/timer and
  GS register state, VU0 macro ops, PS2 float semantics. Plan to reduce it:
  [`../plans/port-architecture.md`](../plans/port-architecture.md).
- **The frame build replays the captured draws through a VIF model** into
  VU memory images for the native VU programs (~10% of the game thread).
- **Movies** match the emulated path byte for byte except +-1-2 in a few
  bidirectional macroblocks with a diagonal backward vector (the native
  decoder follows the MPEG-2 standard there).
- **The browser build** reads the latest depth for the light queries (no
  render thread to wait for), so it is not replay-deterministic there.

## Hand-written code

- 5,092 of 11,844 functions are installed as typed Rust; about 200 more are
  written but not installed (fixtures could not reach their real paths, or
  a replay showed a difference). Installed ports keep the generated code's
  registers and cycles exactly; most are not yet "modern" Rust (they still
  operate on guest memory and registers).
