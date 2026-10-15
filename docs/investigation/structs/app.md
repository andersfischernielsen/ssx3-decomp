# Application glue (App / SSXApp, module switching, globals)

_Mapped 2026-10-15 (Ghidra + paused PCSX2 race over PINE). Short pass; the
`GameModule` itself is in [world.md](world.md)._

## Globals (gp = 0x4a30f0)

| address | Ghidra name | value (race) | meaning | conf |
|---|---|---|---|---|
| 0x4a28a8 | `iGpfffff7b8` | 0x4c9428 | `SSXApp*` (set in `SSXApp_Init`) | high |
| 0x4a2c68 | `iGpfffffb78` | 0xcbfa20 | `GameModule*` (set at the end of `GameModule_Load`) | high |
| 0x4a289c | `iGpfffff7ac` | 0x61ba60 | `PS2GraphicsMan*` (`PS2GraphicsMan_Create`; vtable at +0x10d8) — see render.md | high |
| 0x4a28a0 | `uGpfffff7b0` | 0x618b60 | pad queue ("PadCache", 0x2efc; `PadQueue_Init`) | high |
| 0x4a28ac | `uGpfffff7bc` | | "2DRender" (1 byte) | med |
| gp+0x2a74 | `puGp00002a74` | 0x4c9428 | `App*` (set in `App_ctor`) | high |

## App (base) and SSXApp

`SSXApp` is a static object at 0x4c9428 (constructed by the static-init function
0x242eb8 with `SSXApp_ctor` 0x226830 → `App_ctor` 0x316cd0). The main loop is
`App_Run_MainLoop` 0x316d60; `App_VSyncTick` 0x317348 runs on every VSync and
calls the app's tick slot once per elapsed 1/60 s.

App vt (at **+0x5c**): App 0x48dbe8, SSXApp 0x47d9c8 — 3 Init, 4 Shutdown, 5 VSync
resync, 6 consume tick, 7 VSync tick (from naming.md pass 2).

| off | type | meaning | evidence (live) | conf |
|---|---|---|---|---|
| 0x00 | `AppModule*` | current module | `App_Run_MainLoop` (0xcbfa20) | high |
| 0x04 | `AppModule*` | next module (`App_SetNextState` 0x316ef8) | | high |
| 0x08 | `PS2ExecMan*` | execution manager: `EA_MemAllocNamed(0x142c0, "PS2ExecutionMan")` in `ssx_main` 0x31af80 (ctor 0x317458, vt 0x48db68 at its **+8**), passed to `App_Run_MainLoop`. Slots (vt+8k+4): 2 `PS2ExecMan_Init(60)` 0x31a6d8 (creates the worker thread `PS2ExecMan_WorkerThread`, the VSync thread `PS2ExecMan_VSyncThread`, three semaphores, INTC 3 VBlank handler), 3 shutdown 0x31a920, 4 `PS2ExecMan_WaitFrame` 0x31aa58, 9 0x31acd8 stores its argument (App+0x04 at startup) at +0x14048 and signals the worker semaphore +0x14044, 10 0x31ad00 returns +0x14048 (main loop runs while it is 0; handshake not traced), 11/12 0x31a9d8/0x31aa18 forward to the +0x14080 sub-object. Live words: +0x0c = 0xf, +0x10 = 1, +0x14 = 100 (main-thread priority) | `ssx_main`, `App_Run_MainLoop` (0x604890) | high (identity) / med (slots 9-12) |
| 0x10 | i32 | ticks per second, 60 | `App_Run_MainLoop` sets 0x3c (0x3c) | high |
| 0x14 | f32 | seconds per tick, 1/60 | live 0.01667; used to convert frames to time (InputSync, AI) | high |
| 0x18 | i32 | VSync counter | `App_VSyncTick` increments (0x1017) | high |
| 0x1c | i32 | | 0x1004 | low |
| 0x20 | i32 | 0xc | `App_ctor` | low |
| 0x24 | f32 | ticks per VSync (1.0) | `App_VSyncTick` | high |
| 0x28 | f32 | tick fraction accumulator | `App_VSyncTick` | high |
| 0x2c | f32 | 60.0 | | low |
| 0x5c | vptr | | 0x47d9c8 | high |
| 0x60 | i32 | 1 = start the FE module, 0 = start the debug-menu module | `SSXApp_StartFEModule` | high |
| 0x64 | i32 | 1 | ctor | low |
| 0x78 | `BELibrary*` | back-end library (all BE interfaces) | 30 users (`BELibrary_GetInterface(app+0x78, id)`) (0x53fd08) | high |
| 0x7c | `FEModule*` | front-end module (0xb5ae0) | `SSXApp_StartFEModule` | high |
| 0x80 | `DebugMenuModule*` | (0x238) | `SSXApp_StartFEModule` | high |
| 0x84 | `GameModule*` | in-race module (0x22c) | `SSXApp_StartGameModule` 0x2281d8 (0xcbfa20) | high |
| 0x88 / 0x8c | ptr | UI resources handed to every `UIEngine` (+8 / +0x10): font set and UI sound/locale (0x57b100 / 0x5a0300) | `GameModule_Load` | med |
| 0x9c | ptr | result of 0x2c7440 at init | | low |
| 0xa0 | handle | "loadicon" texture | `SSXApp_Init` | high |
| 0xa4 | ptr | "LoadIndicator" (0x1c: 3 words + two 1.0,1.0 pairs) | `SSXApp_Init`, `LoadIndicator_*` | high |
| 0xa8 / 0xac | ptr | per-pad "InputPad" objects (0x2a4 each), fed from the pad queue in `SSXApp_NextTick` 0x227e98 | (0x9cf3e0 / 0x9cf690) | high |
| 0xb0 / 0xb4 | ptr | per-player input bindings (`Player+0xdf0`, `CamController+0x0c`) | (0x53fcf8 / 0x53fd00) | high |
| 0xb8 / 0xbc | `InputMap*` | `data/config/input.map` / `input2.map`, loaded by `SSXApp_BootLoad` (FUN_00226b60); +0xb8 is the default map of every InputBinding (see input.md) | (0x542560 / 0x542580) | high |
| 0xc0 | `GameModeMan*` | game-mode manager (`GameModeMan_GetInstance`) | (0x59fe00) | high |
| 0xc4 / 0xc8 / 0xcc | `cBXString` | strings (initialised to the shared empty string) | ctor | med |
| 0xd4..0xe8 | `BigMount*`[6] | `.big` archives mounted in `SSXApp_BootLoad` with `EAFile_MountBig(path, 0, 100, &slot)` 0x3ded50: +0xd4 `data/audio/audio.big`, +0xd8 `music.big`, +0xdc `music2.big`, +0xe0 `speech.big`, +0xe4 `data/scripts/scdat.big`, +0xe8 `data/char/mdlps2.big` (0 in the ctor) | live, BootLoad 0x228324..0x2283dc | high |
| 0xec..0x114 | `BigMount*`[11] | the 11 character texture packs, mounted the same way from the path table 0x442100: moby, kaori, alleg, mac, zoe, griff, elise, nate, psymo, viggo, other (`data/char/<name>txp.big`). Each mount record is 0x110 bytes: +0x00 0x010000nn (flags / entry count, low), +0x04 hash or offset (low), +0x0c size (low), **+0x10 path (char[0x100])** | live 0x5e0f00.. (0x110 apart) | high (identity) / low (record words) |
| 0x118 | i32 | 0 | ctor | low |
| 0x11c | `FEAsyncManager*` | (0x590) | `SSXApp_Init` | high |

Module switching: `SSXApp_StartGameModule` allocates "GameModule" and calls
`App_SetNextState`; `SSXApp_StartFEModule` does the same with "FEModule"; the first
module at boot is "PreStartScreen" (0xc). Modules follow the AppModule slots listed
in naming.md (Load, Unload, TryExit, Enter, Update, Render).

## Open questions

- ~~The platform object at App+0x08.~~ Round 2: `PS2ExecMan` ("PS2ExecutionMan", 0x142c0).
  Still open: the exact quit/handshake protocol between slots 9/10 and the worker thread,
  and the +0x14080 sub-object.
- ~~App+0xb8/+0xbc and the 11-pointer block at +0xec.~~ Round 2: input maps and mounted
  `.big` archives (+0xd4..+0x114). Still open: the first words of the 0x110 mount record.

## Proposed names (round 2)

| Address | Current | Proposed | Conf | Evidence |
|---|---|---|---|---|
| 0x317458 | FUN_00317458 | PS2ExecMan_ctor | med | called on the 0x142c0 "PS2ExecutionMan" block in `ssx_main` before the vptr store |
| 0x31acd8 | PS2ExecMan_v09 | PS2ExecMan_PostToWorker(value) | low | stores the arg at +0x14048, `SignalSema(+0x14044)`; called from `App_Run_MainLoop` with App+0x04 |
| 0x31ad00 | (no function) | PS2ExecMan_GetQuitState | low | returns +0x14048, the same word slot 9 writes; `App_Run_MainLoop` loops while it is 0, so the worker presumably clears it after taking the module (not traced) |

