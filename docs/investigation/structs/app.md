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
| 0x08 | ptr | platform/timing object driven by the main loop (vt at its +8: slots for begin frame, end frame, quit test ...) | `App_Run_MainLoop` (0x604890) | med |
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
| 0xb8 / 0xbc | ptr | | (0x542560 / 0x542580) | low |
| 0xc0 | `GameModeMan*` | game-mode manager (`GameModeMan_GetInstance`) | (0x59fe00) | high |
| 0xc4 / 0xc8 / 0xcc | `cBXString` | strings (initialised to the shared empty string) | ctor | med |
| 0xd4..0xe8 | i32 | 0 | ctor | low |
| 0xec..0x114 | ptr[11] | | live 0x5e09b0.. (0x110 apart) | low |
| 0x118 | i32 | 0 | ctor | low |
| 0x11c | `FEAsyncManager*` | (0x590) | `SSXApp_Init` | high |

Module switching: `SSXApp_StartGameModule` allocates "GameModule" and calls
`App_SetNextState`; `SSXApp_StartFEModule` does the same with "FEModule"; the first
module at boot is "PreStartScreen" (0xc). Modules follow the AppModule slots listed
in naming.md (Load, Unload, TryExit, Enter, Update, Render).

## Open questions

- The platform object at App+0x08 (its vtable drives frame begin/end and the quit test).
- App+0xb8/+0xbc and the 11-pointer block at +0xec.
