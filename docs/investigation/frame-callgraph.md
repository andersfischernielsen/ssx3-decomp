# Per-frame call graph (race)

The dynamic call graph of the EE code during a race, recorded with the `callprof` feature.
It is the phase 3 groundwork's view of the core flow. Read it together with
`phase3-boundaries.md`, which explains where each subsystem hands work to the hardware. The
full report is `callgraph-race.md`.

## Recording

```sh
CARGO_TARGET_DIR=target-prof cargo build --profile game -p ssx3 --bin ssx3-headless --features callprof
SSX3_CALLPROF=edges.tsv:5000:5300 SSX3_INPUT=<race script> target-prof/game/ssx3-headless 5300
tools/callgraph.py edges.tsv --root App_RunModuleLoop --root SND_EeThread --root Renderer_DmaKickThread ...
```

`recomp-ee` puts `ssx3_ee_rt::prof::enter(c, start)` at the top of every function. Without the
feature it compiles to nothing; the normal build is unchanged and the race replay stays
byte-identical. With the feature, every call between the two frames is counted as an edge
caller → callee, where the caller is the function containing `ra - 8`. A separate target
directory keeps the normal build cache.

Limits:
- a tail call (`j`) is attributed to the tail-caller's caller;
- calls from the hand-written replacements count like any other call, because the stub `f_X`
  carries the hook.

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

## What this means for the boundary order

The measured traffic confirms the cuts proposed in `phase3-boundaries.md`.

**Files.** During play only streaming touches the file layer, so replacing the
`EAFile_Platform*` seam can be verified at boot and load time first, with no effect on the
race path.

**Sound.** The per-frame work is all above the SND C API (`SSXAudioSystem_Update`) or
inside the SND thread.
- The thread-level cut (A) keeps all of that generated.
- The SND C API cut (B) would also take the UTK decoding and streaming off the EE.

**Frame loop.** One logic tick and one render per loop. A host timer must deliver exactly
one pad entry per 1/60 s to stay deterministic.

**Rendering.** The 42 + 42 models and patches and the ~100 `SprStream` blocks per frame all
end in `PS2GraphicsMan_EndFrame` → `Renderer_DmaKickThread`. That hand-off is the stage-1
cut point.
