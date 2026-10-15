# Roadmap

Goal: a complete decompilation of SSX 3 (`SLUS_207.72`) to Rust that runs
natively, then becomes readable, typed Rust, and finally a native **port**
(the PS2 hardware model replaced by host APIs at the game's own interfaces).

Agreed with the project owner on 2026-10-14. Order matters: each phase is
the safety net for the next one (see "Verification" below).

Status legend: ✅ done · 🟡 in progress · ⬜ not started

## Where we are (2026-10-14)

The game code runs as native code, but against a model of the PS2:

| Layer | Today |
| --- | --- |
| Game logic (EE, VU microcode, IOP drivers) | Statically compiled to Rust, no interpretation. Still PS2-shaped: register file (`c.g[...]`), 32 MiB guest RAM behind address checks, bit-exact PS2 float rules. |
| PS2 kernel/BIOS | Re-implemented natively (HLE): guest threads on `std::thread`, semaphores on mutex/condvar, alarms, interrupts. No BIOS code runs. |
| Hardware (DMA, VIF, VU state/pipeline, GS, SPU2, IPU, SIO2, timers, memory card) | **Emulated** models; the game programs them exactly as on a PS2. |
| Host edges | `std::fs` (ISO, memory card), wgpu (GS output), cpal (audio), winit (window), gilrs (controllers). |

Example paths: file load = game CD library → SIF RPC → IOP-side stand-in →
ISO sectors via `std::fs`. Frame = EE DMA packets → VIF model → VU1 memory →
translated microcode → GIF → GS model → wgpu.

## Phase 1: full static recompilation, playable natively 🟡

Everything the game executes is statically translated; the game is playable on
the owner's machine at full speed. Progress log and resume instructions:
[`playable-push.md`](playable-push.md).

- ✅ EE: 13,306 functions translated, ~97% structured Rust; no runtime decode
  (VU0 macro ops included). Runtime misses are recorded in
  `generated/entries.txt` and translated on the next `recomp-ee`.
- ✅ VU: all 7 microprograms translated (`generated/vu`).
- ✅ IOP: SNDDRV, LIBSD, SIO2MAN, PADMAN, MCMAN, MCSERV recompiled; CD/file
  servers are native stand-ins (firmware); network/USB modules stubbed.
- ✅ Boot, movies, menus, memory-card save/load, a full Snow Jam race to the
  results screen.
- 🟡 Rendering correctness (stretched near-plane geometry: VU WAITQ stall fix
  in progress, `race-geometry.md`).
- 🟡 60 fps (GS on its own thread, `gs-mtgs.md`; VU done, `vu-performance.md`).
- ⬜ Coverage: play every course, mode (career/progression, freestyle,
  replays, other events), menu and option once, so every reachable function
  has run at least once; fix whatever breaks.
- ✅ Build times (`../investigation/crate-split.md`): generated code depends only
  on `ssx3-ee-rt`; GS/kernel changes rebuild in seconds, VU in ~2.5 min.

Done when: all courses and modes play correctly at 60 fps natively, with no
misses, and visuals match PCSX2.

Known issues parked by the owner (2026-10-15) so phase 2 can start: light
glows and flares (sun, point lights) show through geometry
(`../investigation/flare-occlusion.md`); (Multi Play done 2026-10-15:
`SSX3_PAD2=1`, split-screen race works); race ~41 fps headless after the geometry fixes.

## Phase 2: "Rust-ify": readable, typed source 🟡

Progress (2026-10-15): override mechanism + differential tests
(`../investigation/phase2-overrides.md`); 1,368 functions replaced by typed Rust,
byte-identical; 9,715 of 11,844 functions named (pass 3); race-path structs
mapped (`../investigation/structs/`).

Turn machine-shaped Rust into source a person can read and change.

1. Names: from ~4,300 of 13,306 named functions to all of them (Ghidra
   passes, `naming.md`).
2. Types for the core data structures, starting with those the race uses:
   rider, camera, course/world objects, render packets/display lists, sound
   events. Struct definitions in Rust; guest-memory accessors typed.
3. Function signatures (arguments/returns from the MIPS ABI) instead of the
   register file at call boundaries.
4. Register promotion: guest registers → typed locals (SSA), so functions
   stop reading/writing `c.g[...]` internally.
5. Replace PS2 idioms with plain Rust where semantics allow (memcpy/memset,
   string handling, fixed-point/float helpers), keeping bit-exactness where it
   matters.

Done when: the subsystems in phase 3 have clear, typed interfaces.

## Current order of work (owner decision, 2026-10-15)

Phase 2 is not finished function by function before phase 3 starts. Phase 3
only needs the game/hardware boundary understood and typed; inner code can stay
generated. Order:

1. **Finish the running OpenCode wave** (oc23-oc28), gate it (`cargo nextest`,
   5 s per test), integrate what passes, replay, commit. No further bulk
   OpenCode waves for now (OpenCode tokens are running low).
2. **Name every remaining function** (~2,100 still `FUN_*` in Ghidra, 82%
   named after pass 3): naming pass 4 with Ghidra; re-export
   `generated/functions.tsv`; owner saves Ghidra.
3. **Phase 3 groundwork:** a per-frame call graph and subsystem map
   (rendering, files, sound, frame loop / threads), and **typed interfaces
   for the boundary functions only** (where the game hands work to the
   hardware: PS2GraphicsMan draw slots / packet builders / DMA kick, EAFile /
   BigFile / CD, the SND API, GameModule / App frame loop). Inner code stays
   generated.
4. **Phase 3 subsystems**, one at a time, each checked against the emulated
   path running side by side: files → `std::fs`, sound → native mixer, frame
   loop → host timer, rendering → native draws (the 7 VU1 microprograms as
   shaders or kept translated with only the GS replaced).

Bulk phase 2 rewriting (typed function bodies) continues later as capacity
allows (OpenCode waves, `phase2-worker-guide.md`).

## Phase 3: native port: move the boundary upward ⬜

Replace hardware models with host implementations at the game's own
interfaces, one subsystem at a time:

- Rendering: hook the game's draw layer (meshes, textures, matrices) and issue
  wgpu draws directly, skipping VIF/VU/GS emulation (also removes the main
  performance cost). Allows higher resolutions, widescreen, frame rate.
- Files: the game's file layer → `std::fs` on extracted `DATA/`, skipping
  CD/SIF/IOP.
- Audio: the game's sound API → a native mixer, skipping SPU2 emulation.
- Timing: the frame loop driven by a host timer instead of emulated VBlank and
  timer interrupts.
- Kernel/threads: guest threads → plain Rust concurrency (or a single loop)
  where the game's use allows.

Done when: the game runs without the PS2 hardware models (they stay available
for verification).

## Optional: browser (WebAssembly) ⬜

Not a goal now. wgpu targets WebGPU; winit/gilrs/cpal support the web.
Blockers: host-thread runtime (use the single-threaded `--unwind` mode, as
ps2-web-recomp does), synchronous GPU readbacks (WebGPU is async-only; extend
latent readback / CPU fallback), wasm size and speed, NEON → simd128 or scalar,
user-supplied ISO and browser-storage memory card. Phase 3 removes most of
these. Cheap insurance meanwhile: keep `--unwind` mode working and avoid new
blocking GPU readbacks.

## Final task (required before the project is called done) ⬜

- **Light glows/flares must be occluded by geometry** (sun glare, point-light
  glows and star flares currently draw through the rider, walls, roofs and
  terrain): `../investigation/flare-occlusion.md`. Parked by the owner on
  2026-10-15 so the Rust-ification could start, but explicitly required.
  Fix it in whichever phase owns rendering at that point (in the emulated GS
  path if still present; otherwise in the native renderer) and confirm
  against PCSX2.

## Verification (applies to every phase)

Runs are deterministic (fixed RTC), so any change can be checked by replaying
the scripted race and comparing screenshots byte for byte against the
previous build (`playable-push.md` → How to resume). Further oracles:
`vu_diff` captures, `gs_replay` traces, and PCSX2 via PINE
(`tools/pine.py`, `../investigation/pcsx2-pine.md`). Phase 2 rewrites must
keep phase 1 output identical; phase 3 replacements are compared against the
emulated path running side by side.
