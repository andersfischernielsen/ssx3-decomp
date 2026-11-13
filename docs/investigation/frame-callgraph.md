# Per-frame call graph (race)

The shape of a race frame as seen by the `callprof` feature: the main loop, the
threads and the hot paths. Recorded in October 2026 over Snow Jam frames 5001-5300;
the counts are from then, the structure is the game's.

## Recording

`recomp-ee` puts `ssx3_ee_rt::prof::enter(c, start)` at the top of every function.
Without the `callprof` feature it compiles to nothing (the normal build and the race
replay are unchanged). With it, every call between two frames counts as an edge
caller → callee (the caller is the function containing `ra - 8`). Build and record
as in `AGENTS.md` ("Porting loop", step 1: `--features callprof`,
`SSX3_CALLPROF=<out>:<from>:<to>`); `tools/pick_wave.py` ranks the edges and
`tools/callgraph.py edges.tsv --root App_RunModuleLoop --root SND_EeThread ...` prints
a report.

Limits: a tail call (`j`) is attributed to the tail-caller's caller; calls from
hand-written ports count like any other call (the stub `f_X` carries the hook);
`SSX3_NO_OVERRIDES=1` makes every generated call count.

## Race frames 5001-5300 (Snow Jam, mid-race)

- **Volume:** 2,186 distinct functions run, at about 16,850 calls per frame.
- **The 60 Hz loop** (`App_RunModuleLoop`, one iteration per frame):
  1. `SSXApp_NextTick` takes the pad entry: `PadQueue_*`, `InputPad_Update`.
  2. `GameModule_Update` runs the game logic. `ProcessList_Next` alone is 163 calls per frame,
     with `WorldObj_Update`, `ModObj_Update`, particles and the riders below it.
  3. `GameModule_Render` (0x22b008; not a Ghidra function, named in pass 5) does the following:
     - views and matrices (`PS2GraphicsMan_SetPerspective`, `LoadMatrix`, `AddView`);
     - `Nash_GatherVisible`, culling with `OcclusionMan_BuildVisible`;
     - `AI_Render`, for the riders and their effects;
     - `GameModule_RenderModels` (42 `DrawModelSimple` per frame);
     - `GameModule_RenderBezierPatches` (42 patches per frame);
     - fog particles, lens flares, process lists, the crowd, the sky box;
     - `UIEngine_Render`;
     - `PS2GraphicsMan_EndFrame`, which runs `PS2GraphicsMan_RenderFrame` (DeferMan buckets,
       about 350 `DeferMan_AppendPacket` per frame) and publishes the chains;
     - finally `SSXAudioSystem_Update`, which carries the game-side sound policy.
  4. `EATimer_ServiceCallbacks` runs the polled timer tasks: `SND_TimerServiceCallback` →
     `SNDSTRM_Service` (music and speech streaming through `StreamFile_GetChunk`), the CD tray
     poll and `EAFile_HostReadService`.
  5. `App_InvokeExecCallback`, then `PS2ExecMan_WaitFrame`.
- **Threads:**
  - `SND_EeThread` runs about 1.6 ticks per frame, i.e. the IOP tick at ~94 Hz.
    `SND_Service` → `SND_Tick` → per-channel volume and pitch.
  - `Renderer_DmaKickThread` runs `Renderer_SetDisplayRegs` once per frame, and
    `LensFX_UpdateCallback` → `LensFX_UpdateSourceVisibility`. Flare visibility is decided here,
    in the DMA kick thread after the frame's chains are sent, which is relevant to the flare
    occlusion task (`flare-occlusion.md`).
- **The hottest leaf** is `SND_UtkSkipBits`, about 1,000 calls per frame: the UTK (MicroTalk)
  speech decoder. A native sound path would remove it.
- **File traffic in a race is streaming only.** `StreamFile_GetChunk` runs about 4.7 times per
  frame for music and speech, plus an occasional `sceCdSync`. No `EAFile_Open` happens
  mid-race.
