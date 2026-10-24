# Plans

Working plans. Each states the goal, the design and what "done" means; keep
them current.

| File | Scope |
| --- | --- |
| [`roadmap.md`](roadmap.md) | **Start here.** The phases: static recompilation → typed Rust → native port (→ optional browser) |
| [`native-rendering.md`](native-rendering.md) | Everything around the game logic native: renderer, draw functions, movies, the remaining PS2 (N1-N6) |
| [`configurable-tick-rate.md`](configurable-tick-rate.md) | Game rate as a setting (120/144/240 Hz, no interpolation): the rate switch (`ssx3_game::tick`), the physics decision, the measurements, the inventory and the worker pass (AGENTS.md) |
| [`modern-graphics.md`](modern-graphics.md) | Optional modern rendering in F10 (FXAA, mipmaps, terrain detail, rider detail distance, AO, bloom, snow sparkle): what each does, how it fits the renderer, how it was checked |
| [`shadow-maps.md`](shadow-maps.md) | Real sun shadows (shadow maps, later ray-traced) in the native renderer, tested on Snow Jam; iterative |
| [`terrain-lighting.md`](terrain-lighting.md) | Future work: a new terrain lighting model with per-course tuning: the tracks as they look now, lit live (sun, sky, shadows) instead of the baked layer |
| [`port-architecture.md`](port-architecture.md) | Current: the review harness, the module layout, the modern runtime (one host thread, a native frame loop, a smaller `Machine`) |

Development, debugging and verification switches: [`../development.md`](../development.md).

Principles:

1. **Recompiler, not emulator.** Generated Rust contains no instruction
   fetch or decode; game code is translated ahead of time, everything around
   it is native.
2. **Exactness.** Runs are deterministic. A change keeps the replay RAM and
   screenshots identical unless it is meant to change behaviour; hand-written
   code is checked against the generated code (differential tests, the
   scenario harness, `SSX3_SHADOW`).
3. **Findings are written down** in `docs/investigation/`, plans here.
