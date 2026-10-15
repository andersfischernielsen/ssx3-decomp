# Phase 3 boundaries: files, sound, frame loop, rendering

Where the game hands work to the hardware, and where a native replacement can
be slotted in with a typed interface (roadmap § "Current order of work", step 3).
Research pass 2026-10-15 over the fully named Ghidra project (naming pass 4).



**Confidence key:** **V** = checked in decompilation or disassembly in this pass. **D** = taken from an existing doc that rates it high (decompilation or live evidence), not re-checked. **S** = from a scan of generated code (approximate). **I** = inferred from names or structure.

Calling convention: EE o32-ish, so arguments are a0..a3, then t0..t3, and the return value is in v0. `gm` = PS2GraphicsMan (0x61ba60 live), `em` = PS2ExecMan, `App` = SSXApp at 0x4c9428.

---

## 0. Summary: where to cut

| Subsystem | Narrowest clean seam | Why | Alternative (higher level) |
|---|---|---|---|
| Files | **`EAFile_Platform{Open,Read,Close,GetSize,GetLsn,Write}`** (0x3e3208..0x3e3538). Each has exactly one caller (`EAFile_ServiceQueue`, or `EAFile_ParseBigPath` for Open). Completion is reported by calling `EAFile_CompleteRequest(ok)`. | Everything above it (BIG lookup, request queue, priorities, chunking, AsyncFile, StreamFile, callbacks) stays generated and unchanged. Only one request is outstanding at a time. Below it are the CD thread, `sceCdRead` and SIF. **V** | EA left a `host0:` device path in the code: `EAFile_PlatformInit(_, 0)` selects device 2, which calls `sceOpen/sceRead/sceLseek` and completes from the `EAFile_HostReadService` timer task. Even cheaper, but it still goes through the fileio RPC. **V** |
| Sound | **Two-level.** (A) Replace SNDDRV + SPU2 below the EE SND library: the per-tick tag-buffer DMA (`sceSifSetDma(0x50c800,1)` in `SND_EeThread`), sample uploads (`SND_IopUpload`), the init/effect RPC (`SND_IopCallRpc`) and the IOP tick (`SND_SifCmdHandler`). (B) Optionally later, the SND C API (`SND_Play/Stop/SetVolume/SetPitch/SetPan/FadeVolume/BankAdd/BankRemove/IsHandleDone`, `SNDSTRM_*`). | Voice commands do **not** go over RPC. They are written into a shared command and status block that is DMA'd every tick, and the EE also mixes its own voices to PCM (`SND_EeMix`). So (A) is a small, typed surface. (B) means reimplementing EA patch, MIDI and DSP logic. **V** | Game-side `SSXAudioSystem` → `Audio_PlayPushedSound` (52 callers) is too high: it carries game policy. |
| Frame loop | **`App_VSyncTick(App*)` driven by a host 60 Hz timer**, plus overrides of `PS2ExecMan_WaitFrame` (em slot 4) and the EATimer tick counter `0x450dd0`. | The logic tick is fixed-step and pad-sampled: each VSync pushes one PadQueue entry, and the main loop runs one module Update per queued entry (at most 12), then one Render, then WaitFrame. **V** | Replacing `App_RunModuleLoop` 0x316f00 as a whole. |
| Rendering | First: **the DMA kick** (`PS2GraphicsMan_EndFrame` publishes the {VIF1 tadr, GIF tadr} × 2 phases, which `Renderer_DmaKickThread` consumes). Consume the chains natively with VIF/VU1/GS translated, or with only the GS replaced. Later: **the PS2GraphicsMan vtable** (0x493260, 125 slots) plus `PS2BezierMan` vt 18 and the post-effect emitters. | The chain hand-off is one place with a fixed format. The draw slots are the typed "draw API", but about 30 slots build VIF packets directly into the VifBuff. **V/D** | Hooking `DeferMan` records (state, packet, matrix): typed per-draw, but the payloads are still VU1 packets. |

---

## 1. Files (EAFile / BigFile / CD)

### 1.1 Layer stack (top → bottom)

```
game: SSXApp_BootLoad, GameModule_Load, BoltFile cache, ScriptEngine, SndBank_Load, FEAsyncManager, cWorldCache/cStreamMan, MusicPlayer, AV_Player ...
  | sync helpers           EAFile_LoadFile{,Ex,Into,IntoEx,GetSize,s}, EAFile_LoadDecompressed{,Into}, EAFile_PathExists/GetPathSize, BigFile_Mount
  | async helpers          AsyncFile_Load/LoadInto/IsDone/GetResult (pool, poll)      StreamFile_* (ring-buffer streaming, chunked "SCxl")
  v
EAFile sync API            EAFile_Open/Read/Write/Close/GetSize/Exists/MountBig/UnmountBig  (= Queue* + EAFile_WaitRequest)
EAFile async API           EAFile_QueueOpen/Read/Write/Close/GetSize/Exists/MountBig/UnmountBig, SetRequestCallback, Get/Release/Wait/CancelRequest
  v
EAFile_ServiceQueue 0x3ddfa8   priority-sorted request list, ONE active request (0x519ad4), BIG path resolution ('|')
  v
EAFile_Platform*           <-- NARROWEST SEAM
  v  device 1 (cd)                              device 2 (host0:)
EAFile_CdLookup/CdRead -> CD thread            sceOpen/sceLseek/sceRead (fileio RPC 0x80000001), completed by timer task
EAFile_CdReadThread 0x3e3be0 -> EAFile_CdIssueRead -> sceCdRead(lsn,nsec,buf,mode) + sceCdSync  (cdvdfsv RPC 0x80000595)
```

### 1.2 Boundary functions

| Addr | Name | Signature (best understanding) | Callers | Below | Conf |
|---|---|---|---|---|---|
| 0x3dcbd8 | EAFile_Startup | (maxReq=0x30, ?=500, maxHandles=0x20) → Init → `EAFile_CdInit` 0x3e4040 | ssx_SystemInit | CD init, own ISO9660 directory table, CD thread prio 0xc, cd callback | D |
| 0x3e3020 | EAFile_PlatformInit | (?, a1 useCd): a1==0 → device 2 (host0:), else device 1 (cd). Registers `EAFile_HostReadService` as an EATimer callback | EAFile_Init | — | V |
| 0x3dde78 | EAFile_SetRootPath | (char* root) "cd:" / "host0:" → 0x519af0 | ssx_SystemInit only | — | V |
| 0x3debf0 | EAFile_Open | (path a0, mode a1, prio a2, out u32* handle a3) → bool. Blocking (QueueOpen + WaitRequest) | 7 (LoadFile workers, Avalanche history) | ServiceQueue | V |
| 0x3dec60 | EAFile_Read | (fh a0, offset a1, buf a2, size a3, prio t0) → bytes. Sync; chunks of 0x8000 via `EAFile_TransferChunked` with `t1 = EAFile_QueueRead` | 7 (LoadFile workers, LoadBigHeaderWorker, Avalanche) | QueueRead | V |
| 0x3dec80 | EAFile_Write | same shape, with `t1 = EAFile_QueueWrite` 0x3dd720 | Avalanche_WriteHistoryFile | | V |
| 0x3deca0 / 0x3decf8 / 0x3dee18 | EAFile_Close / GetSize / Exists | sync wrappers | 9 / 3 / 2 | | V |
| 0x3ded50 | EAFile_MountBig | (path, ?, prio=100, out BigMount**) → bool. Sync | SSXApp_BootLoad, BigFile_Mount, FEAsyncManager_ctor, FEModule_LoadAnimData, AI_LoadAnimDataSources, SSXAudioSystem_SetLanguage, ssx_MountSlusOvfBig | QueueMountBig → open + read header (`EAFile_OnBigOpened`/`OnBigHeaderRead`) | V |
| 0x3dd4e8 | EAFile_QueueOpen | (path, mode, prio, userArg) → reqId | AsyncFile_Load/LoadInto, EAFile_Open, QueueMountBig, StreamFile_StartNextRequest/OnClose | ServiceQueue | V |
| 0x3dd648 | **EAFile_QueueRead** | (fh a0, offset a1, buf a2, size a3, prio t0, userArg t1) → reqId; size is clamped to `fh->size - offset` | 9: AsyncFile_On*, AsyncFile_ReadRange, EAFile_Read, EAFile_OnBig*, **StreamFile_IssueRead** | ServiceQueue | V |
| 0x3dcf70 | EAFile_SetRequestCallback | (reqId, cb). `cb(reqId, status, userArg)` runs at completion, **in the completing thread** (CD thread, prio 0xc, or the timer task) | many | | V (already overridden) |
| 0x3dd1d8 | EAFile_WaitRequest | (reqId) → status. Spins: `EAThread_Yield(1)` on other threads, `EATimer_ServiceCallbacks(0)` on the main thread | sync wrappers, AsyncFile_GetResult | | V |
| 0x3df690 / 0x3df748 | AsyncFile_Load / LoadInto | (path, userArg) / (path, buf, ...) → AsyncFile id; callback chain Open → Size → Read chunks | 9 / 7 (BoltFile, FEAsyncManager, ScriptEngine, SndLoadQueue, MusicPlayer, WPFile, Rider_LoadStep, AV_Player, PlaybackMan, DNAS) | QueueOpen/QueueRead | V |
| 0x3df980 / 0x3df9d8 | AsyncFile_IsDone / GetResult | polled each frame | 13 / 22 | | V |
| 0x3e06d8 / 0x3e0d80 / 0x3e0c88 / 0x3e12e0 | StreamFile_Create / QueueFile / QueueRange / GetChunk | ring-buffer streamer with a chunk filter (tag mask table, `StreamFile_ClassifyChunk`) | SNDSTRM_Open/QueueRequest/Service, AV_Player_*, cStreamMan_* (world streaming) | StreamFile_IssueRead → QueueRead | V |
| 0x3ddfa8 | EAFile_ServiceQueue | (req or 0): inserts by priority; if idle, dequeues and dispatches on op = `w0>>20 & 0xf`: 2/8 open (8 = exists), 3 close, 4 read, 5 write, 6 size, 7/9/10 nop/big ops | 13 Queue*/completion | Platform* | V |
| 0x3de420 | EAFile_CompleteRequest | (ok): status = 1 / −2 (fail) / −1 (cancelled); calls the callback; re-enters ServiceQueue(0) when the nesting count reaches 0 | CdRead (cached sector), CdReadThread, HostReadService, PlatformWrite, ServiceQueue | — | V |
| **0x3e3208** | **EAFile_PlatformOpen** | (resolvedName, mode, fh*) → bool; writes `fh->handle = dev<<24 \| idx`. cd: `EAFile_CdLookup` (upper-case, hash 0x3e36f8, bsearch). host: `sceOpen` | ServiceQueue, ParseBigPath | CD table / fileio | V |
| **0x3e3350** | **EAFile_PlatformRead** | (handle a0, buf a1, offset a2, size a3) → started. cd: `EAFile_CdRead` → CD thread; host: latches the request, completed later by `EAFile_HostReadService` | ServiceQueue only | CD thread / timer task | V |
| 0x3e32f8 / 0x3e34e8 / 0x3e3538 / 0x3e3478 | PlatformClose / GetSize / GetLsn / Write | (handle) / (handle) / (handle) / (handle, buf, off, size) | ServiceQueue only | CdGetEntry / sceLseek / sceWrite | V |
| 0x3e4648 | EAFile_CdRead | (idx, buf, off, size): LSN = entry.lsn + off/2048; handles partial and unaligned sectors through a 1-sector cache (0x519c7c); a hit on the cached sector completes synchronously; otherwise `SignalSema(0x519c4c)` | PlatformRead | CD thread | V |
| 0x3e3be0 / 0x3e3b00 | EAFile_CdReadThread / CdIssueRead | `sceCdRead(lsn, nsec, buf, 0x519c5c)` with 3 retries via `EAFile_DelayTicks` (SetAlarm) | — | 0x401df8 sceCdRead → SIF RPC 0x80000595 (HLE in `sif.rs` `cd_ncmd`) | V |
| 0x4011c0 | sceCdSearchFile | — | **0 callers**: EA walks the ISO directory itself (`EAFile_CdScanDir`/`CdBuildHashTable`) | | V |

### 1.3 Data structures (V)

- **File handle** (pool, 0x20 handles): +0 platform handle `dev<<24|idx` **or** `BigMount*` when the file is inside a BIG; +4 size; +8 offset inside the BIG (0 = a plain file); +0xc LSN; +0x10 `char name[0x100]`.
- **Request** (0x30 bytes, pool at `*(0x519ad8)`, id = `slot<<24 | op<<20 | serial`): +0 id; +4 cancel; +8 status (0 pending, 1 ok, −2 fail, −1 cancel); +0xc error; +0x10 priority; +0x14 userArg; +0x18 offset (or open mode); +0x1c size; +0x20 buf; +0x24 fh; +0x28 callback; +0x2c next.
- **BigMount** (linked at 0x519ae4): +0 header buffer, +4 header size, +0xc the BIG's own fh, +0x14 next. Supported types (`BigFile_GetType`): `0xC0FB` (EA compact), `"BIGF"`, `"BIG\0"` (V). Path syntax: `"a.big|name"` searches one archive, `"|name"` searches all mounted archives, `"name"` opens directly (V). On a read inside a BIG, ServiceQueue calls `PlatformRead(bigfh->handle, buf, fh->bigOffset + off)`, so the BIG layer sits **above** the seam (V).
- **Sync vs async.** Everything is asynchronous internally. The sync APIs spin in `EAFile_WaitRequest`, which advances EATimer tasks on the main thread. Callbacks run in the completing thread. Polling (`AsyncFile_IsDone`) is the game-side pattern (V).

### 1.4 Native replacement plan (proposal)

- Override the six `EAFile_Platform*` functions:
  - `open(name, mode) -> Option<u32 handle>` maps to `std::fs` on the extracted `DATA/`. Note: CD lookup is case-insensitive and drops `;1`.
  - `size(handle) -> u32`, plus `lsn(handle) -> u32` (only stored in fh+0xc; check whether anything reads it, see open questions).
  - `read(handle, buf, off, size)` copies straight into EE RAM, then calls `EAFile_CompleteRequest(1)`, either immediately (the CD cached-sector path already does this) or deferred to the next `EATimer_ServiceCallbacks` call, as the host0 path does. Deferring is safer for re-entrancy, because a synchronous completion recurses through ServiceQueue once per queued request.
- Disabling `EAFile_CdInit` then removes the CD thread, cdvd RPC and SIF from the file path.
- Typed interface: `trait EaPlatformFs { open, close, size, read_async(handle, dst:&mut [u8], off) -> Completion }`.

---

## 2. Sound (SND / SNDSTRM / PATH / SPCH / SSXAudioSystem)

### 2.1 Layer stack

```
game        SSXAudioSystem_* (~300 fns: PlayFESound 18 callers, PlayMusic 13, TrackSound 9, event handlers, mix/duck/fade)
            Audio_PlayPushedSound 0x2906b8 (52 callers) <- param stack Audio_PushParams*(vtable)
EA C++ mgr  SndBankMan (bank load/unload, SndLoadQueue async), BankMonitor(+Instance: 3D falloff, doppler, timers),
            StreamArray/StreamBuffer, MusicPlayer/SongInstance -> PATH_* (interactive music), SpeechSystem/SpeechInstance -> SPCH_*,
            CrowdAudio, MIDIBanks -> SNDMIDI_*, PS2SoundManager (SND_Init, SPU block alloc)
EA SND C    SND_Play/PlayPatch, SND_Stop/SetVolume/SetPitch/SetPan/FadeVolume/IsHandleDone/SetMasterVolume, SND_BankAdd/Remove,
            SNDSTRM_Open/Queue*/Stop/Close/Set*, decoders (EAXA ADPCM, UTK MicroTalk, PCM16), DSP chain (SND_Dsp*), SNDENC (DTS/Dolby enc)
EE driver   SND_EeThread 0x3c19a8 (prio 6): woken per IOP tick -> SND_Service(94) -> SND_Tick xN; SND_EeMix (EE voices -> PCM);
            sceSifSetDma(tag buffer 0x50c800) ; SND_IopUpload (bank samples) ; SND_IopCallRpc (init / effect only)
IOP         SNDDRV.IRX + LIBSD.IRX (recompiled, SSX3_IOP=1) -> SPU2 model -> cpal
```

### 2.2 Boundary functions

| Addr | Name | Signature | Callers | Below | Conf |
|---|---|---|---|---|---|
| 0x3b5158 | SND_Init | (cfg, ?) → err; calls `SND_IopInit` | PS2SoundManager_Init, SND_InitWithOutputMode | | V |
| 0x3c1b80 | SND_IopInit | loads LIBSD/SNDDRV if needed (`sceSifLoadModule`); creates `SND_EeThread` (prio 6, stack 0x50cb20/0x6000); main thread prio → 100; `sceSifAddCmdHandler(1, SND_SifCmdHandler)`; `sceSifBindRpc('SND' 0x534e44)`; one init RPC (fno 0, 0x14-byte block 0x50acc8: flags 0x80303, buffer, rates); mix block size 0x100/0x180/0x200 for 24/36/48 kHz; optional VBlank-start INTC 2 handler when rate == −1 | SND_Init | SIF | V |
| 0x3c0b10 | SND_IopCallRpc | (fno, ?, size): copies to 0x50ad00, spins on `sceSifCheckStatRpc`, `sceSifCallRpc(0x50b980, fno, fno==2 ? nowait : wait, ...)` | **3 only**: IopInit, IopSetEffect (reverb), IopShutdown | 'SND' RPC | V |
| 0x3c1578 | SND_SifCmdHandler | SIF user cmd 1: type 0 = IOP tick (stores status word, `iSignalSema(0x50c7a4)`), type 2 = upload DMA complete | interrupt | | V |
| 0x3c19a8 | **SND_EeThread** | loop: WaitSema(tick); lock; pre-hook; `SND_Service(0x5e)`; change callbacks; copy the 0x240-byte status (0x50b740 → 0x50b500); `SND_ReleaseFinishedVoices`; save VU0 + SPR; `SND_EeMix(0x50c7bc, blockSize)` (or the SNDENC encoder hook 0x50c848 for DTS/Pro Logic); `SNDSTRM_RunCallbacks`; `FlushCache`; **`sceSifSetDma(0x50c800, 1)`** (the per-tick tag buffer); post-hook; unlock | thread | SIF DMA → SNDDRV | V |
| 0x3c42d8 → 0x3c4788 | SND_IopUpload / SND_SifDmaUpload | (memKind {8, 0x100}, src EE, dst, size): 0x1000-byte SIF DMA pieces; `SND_IopUploadWait(id)` polls | SND_BankAdd only | SNDDRV upload (cmd 0 type 1) → SPU DMA | V |
| 0x3b6098 | **SND_BankAdd** | (out int* bankHandle, bankImage*) → 7/8 ok, −6 no memory. Image header: +6 u16 patchCount, +8 sample data offset, +0xc size of region A (kind 8), +0x10 size of region B (kind 0x100), +0x14 patch offsets[]; allocates `SND_SpuMemAlloc`, uploads, relocates patches | SndBank_Load, SndBank_LoadFromMemory, SndLoadQueue_Update | upload | V (layout) / I (meaning of kinds 8 / 0x100) |
| 0x3b6300 | SND_BankRemove | (bankHandle) | 6 | | V |
| 0x3b6820 | **SND_Play** | (bankHandle a0, soundIndex a1, SndPlayParams* a2 [0x18 bytes: +0 vol u8, +2 …]) → handle ≥ 0 or −8 | Audio_PlayPushedSound, BankMonitorInstance_Play, PATH_PlatQueue, SNDMIDI_NoteOn | SND_PlayPatch → voice alloc → `SND_ChannelStartVoices` (EE voice: `SND_EeVoiceStart/Activate`; SPU voice: writes the voice start address to the command block `*(0x50b9b0)+0x20/+0x1a0` with a sequence counter) | V |
| 0x3b89e0 / 0x3b85f0 / 0x3b8790 / 0x3b8c00 / 0x3b8530 | SND_Stop / SetVolume / SetPitch / SetPan / FadeVolume | (handle) / (handle, vol) / (handle, pitch) / (handle, pan, ?) / (handle, target, time) → err | 11 / 7 / 4 / 4 / 4 (BankMonitorInstance_*, SNDMIDI_*, SNDSTRM_*, PATH_Plat*) | `SND_Channel*` → command block / EE voice params | V (shapes) / I (units) |
| 0x3b8ab8 | SND_IsHandleDone | (handle) → bool | 8 | | V |
| 0x3b8160 | SND_SetMasterVolume | | SSXAudioSystem_SetChannelVolume | | V |
| 0x3b5910 | SND_AddTickCallback | (fn) runs on the SND thread | BankMonitor, PATH_PlatInit, SNDMIDI_Init | | V |
| 0x3b7450 / 0x3b7818 | SNDSTRM_Open / Create | (...7 args: buffers, channels, …) → stream id; uses `StreamFile_Create` | PATH_PlatAddTrack, StreamBuffer_ctor, AV_Audio_Init | StreamFile → EAFile | V |
| 0x3b76e8 | SNDSTRM_QueueRequest | (streamId, userArg, offsetOrFh, size, mode {0 range, 1 file}) → request id; chunk filter tag `0x6c454353` = `"SCEl"` | Queue / QueueFile / QueueFileOffset | `StreamFile_QueueRange/QueueFile` | V |
| 0x3b71d8 | SNDSTRM_Service | parses chunks (`SNDSTRM_ParseHeader` → `SND_ParseStreamHeader`), starts playback (`SND_ChannelStartStreamVoices`), decodes (EAXA/UTK/PCM16) | SND timer service | | V |
| 0x2ae020 / 0x2ae048 | PATH_PlatTimerCallback / PATH_PlatInit | music path tick, registered with **EATimer_AddCallback** (needs RCNT1 ticks; see iop-audio.md) | PATH_AddTrack | | V/D |
| 0x2906b8 | Audio_PlayPushedSound | (SoundRequest* {mgr, bankIdx[], soundIdx[]}) → `SND_Lock; SND_Play; SND_Unlock; BankMonitor_AddInstance` | 52 game functions | SND_Play | V |
| 0x285bf8 | SSXAudioSystem_Update | (sys, ticks) per frame | FEModule_Render, GameModule_RenderUI | everything above | V (callers) |

### 2.3 Where the "commands" travel (V unless marked)

- **Play, volume, pitch and pan do not use RPC.** They update channel state on the EE (`SND_Channel*`, `SND_EeVoice*`). For SPU voices, `SND_ChannelStartVoices` writes start addresses and parameters into the command/status block at `*(0x50b9b0)`. That block is part of the tag buffer DMA'd to SNDDRV by `SND_EeThread` every tick. The IOP returns a 0x240-byte status block with each tick (`iop-audio.md`, D).
- **EE-mixed voices** (streams, EAXA/UTK-decoded sounds, and anything with an EE DSP chain) are mixed on the EE by `SND_EeMix` 0x3c8968 into a PCM block of 0x100/0x180/0x200 frames. SNDDRV resamples it into the SPU AutoDMA input (D).
- **RPC** is used only for init, reverb/effect (`SND_IopSetEffect` → `SND_SetSpuReverbMode`) and shutdown.
- **Formats loaded from disc:**
  - `.bnk` sound banks in `data/audio/audio.big`, character grunt banks `GRNT_*.bnk`, `LoadingScreen.bnk`: header as above, patches, PS-ADPCM (SPU) and EE sample regions. V/I.
  - `.mus` interactive music in `music.big`/`music2.big` through PATH + SNDSTRM: EA `SCHl…SCEl` chunked streams. V for the `SCEl` tag, I for SCHl.
  - Speech: `speech.big` and per-language `english/french/german/spanish.big`, plus `data/config/speech.inf`; SPCH banks; UTK (MicroTalk) codec is present. I.
  - Movies: `.mpc` (MPEG2 + audio through AV_Player/AV_Audio → SNDSTRM). V for names.

### 2.4 Native replacement (proposal)

- **(A) Recommended first.** Model "SNDDRV on host":
  - `fn snd_tick(tag: &TagBuffer /*0x50c800 chain*/, ee_pcm: &[i16]) -> StatusBlock(0x240)`
  - `fn upload(kind, dst, &[u8])` plus completion notification (type 2)
  - `fn rpc(fno, &[u8]) -> Vec<u8>`
  - Drive the tick from the host audio callback, replacing the IOP SIF cmd type 0 that signals sema 0x50c7a4.
  - This is what the recompiled IOP and the SPU2 model do today, without IOP emulation, so it can be checked side by side against `SSX3_IOP=1`.
- **(B) Optional.** Replace the ~15 SND C entry points plus `SNDSTRM_*` with a native mixer. That needs native EA patch parsing (`SND_ParsePatch` 0x3baac0), EAXA/UTK decoders, voice groups and DSP. PATH, SPCH and BankMonitor stay generated above it.

---

## 3. Frame loop, timing and threads

### 3.1 What drives a frame (V)

```
VBlank-End INTC 3 -> PS2ExecMan_VBlankEndIntc 0x31a490 -> iSignalSema(em+0x4034)                        (D: boot-path)
  PS2ExecMan_VSyncThread 0x31ac08 (prio P-1) -> App_VSyncCallback 0x317500 -> App_VSyncTick(App) 0x317348:
      App+0x18++ ; acc += App+0x24 (1.0) ; for each whole tick: App vt slot 7 (+0x3c) = SSXApp_Tick 0x227f58 -> PadQueue_Poll 0x326b88
          (reads every pad into a 30-entry ring, 4 x 100 bytes per entry)
      em vt +0x2c = PS2ExecMan_v05 0x31aac8: if main is waiting (em+0x1c) SignalSema(em+0x18)
main thread  App_Run_MainLoop 0x316d60:
      em->Init(60) [PS2ExecMan_Init: worker + vsync threads, INTC 3] ; App->Init (SSXApp_Init) ; em->Post(App+4)
      while worker not done: em->Begin; EATimer_ServiceCallbacks(0); em->End; em->WaitFrame; App_InvokeExecCallback   (boot load on the worker thread)
      first module Enter (+0x2c) ; App_RunModuleLoop 0x316f00:
        loop:
          em->Begin (+0x5c) ; EATimer_ServiceCallbacks(0) ; em->End (+0x64)
          module state machine (App+0x58): 0 idle / 1 next module loading on the worker (Load slot 2 = +0x14 via em->Post) /
                                            2 TryExit(+0x24) -> Unload(+0x1c) -> dtor -> swap -> Enter(+0x2c) / 3 quit
          while App vt slot 6 (+0x34) = SSXApp_NextTick 0x227e98 returns 1 (a PadQueue entry is available; also cCommSystem/USB kbd, InputPad_Update):
                ticks++ ; if ticks > App+0x20 (12): resync (App vt +0x2c) and break
                App+0x1c++ ; module->Update (+0x34)     <- fixed 1/60 s logic step
          if ticks>0: module->Render (+0x3c)             <- one render per loop iteration
          em->WaitFrame (+0x24) = PS2ExecMan_WaitFrame 0x31aa58: em+0x1c=1, Begin, WaitSema(em+0x18), drain PollSema, End
          App_InvokeExecCallback
```

So **logic runs at a fixed 60 Hz in step with VSync-sampled input**, with up to 12 catch-up Updates per rendered frame. Render happens once per loop. Rendering is decoupled from the GS by double buffering: `TryAcquireFrame` returns 0 while both buffers are busy, and the module then skips its draw.

### 3.2 Boundary functions

| Addr | Name | Role | Conf |
|---|---|---|---|
| 0x316d60 | App_Run_MainLoop(App*, em*, argc, argv) | outer loop, boot load | V |
| 0x316f00 | App_RunModuleLoop(App*) | the per-frame loop above | V |
| 0x317348 | App_VSyncTick(App*) | the per-VSync tick; pure apart from PadQueue_Poll and the em signal | V |
| 0x227f58 / 0x227e98 | SSXApp_Tick / SSXApp_NextTick | pad sampling / tick consumption and input update | V |
| 0x31aa58 | PS2ExecMan_WaitFrame(em) | blocks on the frame sema (em slot 4, vt+0x24) | V |
| 0x31a9d8 / 0x31aa18 | PS2ExecMan_v11/v12 (Begin/End, vt +0x5c/+0x64) | forward to the em+0x14080 sub-object (not traced) | D (med) |
| 0x31ac60 | PS2ExecMan_WorkerThread | runs `module->Load` (vt+0x14) posted by `PS2ExecMan_v09`; clears em+0x14048 | V |
| 0x3e4af0 / 0x3e4db8 | EATimer_Init(60) / EATimer_Rcnt1Handler | RCNT1 target IRQ (INTC 10): increments ticks 0x450dcc/0x450dd0 and runs the 8-slot ISR table 0x450da8 | V |
| 0x3e57f8 / 0x3e5928 | EATimer_AddCallback / ServiceCallbacks | 16-slot **polled** task table 0x51ed98 {fn, period, nextTick, busy}, due when `0x450dd0 >= next`. Registered: ssx_SystemInit (0x31ad20 tray poll), EAFile_PlatformInit (HostReadService), EAFile_CdInit (CdRemount), PATH_PlatInit (music), SND_StartTimerService. Serviced by the main loop, WaitRequest, WaitForRenderIdle, PlayBlocking and others (15 callers) | V |
| 0x423b20 | SetAlarm (patched 0xFC) | EAFile_DelayTicks, THREAD_yieldticks (`EAThread_Yield`), libcdvd/libmc delays, DirtyRpc | V (callers) |
| 0x3c19a8 | SND_EeThread | paced by the IOP tick (~94 Hz), not by VSync | V |
| 0x382760 | Renderer_DmaKickThread | paced by the VIF1 DMA-end and VBlank-start IRQs | V |

**Threads** (D, `boot-path.md` §4; creators V via CreateThread xrefs): main (prio 1 → 100), PS2ExecMan worker (P+1) and VSync (P−1), Renderer DMA kick (5), SND EE (6), EAFile CD read (0xc), libcdvd callback (0xb), power-off (1), libkernl helper (0), DirtyRpc/net (10), EAThread_Create (GameLink only).

### 3.3 What a host-timer loop replaces (proposal)

1. Host 60 Hz timer → call `App_VSyncTick(App)` directly (or signal em+0x4034) and advance EATimer's `0x450dd0` (plus the 8-slot ISR table if any entry is used). This removes the VBlank-End INTC, RCNT1 and the VSync thread.
2. Override `PS2ExecMan_WaitFrame` to block on the host frame event.
3. Renderer pacing (VBlank-start, flip after N VBlanks) goes away with the native present (§4).
4. The SND tick comes from the host audio callback (§2.4 A).
5. A single-threaded `--unwind`-friendly design is possible later: worker-thread Loads can become synchronous calls; the CD thread disappears with §1.

---

## 4. Rendering

### 4.1 Per-frame flow (D from `render.md`, re-checked V where marked)

```
module Render (GameModule vt slot 7 = 0x22b008, NOT a Ghidra function; generated as f_0022b008, 0x22b008..0x22c074)
  gm slot 17 TryAcquireFrame -> PS2GraphicsMan_AcquireFrameBuffer 0x3826e0 (buffer state 0 -> 1; 0 = busy, skip)   [V]
  gm slot 116 AddView, GameViewMan_PrefetchViews, AppModule_RenderFade, DeferredDelete_AdvanceFrame,
  gm 66 ClearLights, 34 LoadMatrix, 25 SetPerspective, 43 GetWorldToClip, Mat4_Inverse, (world pass: Nash_GatherVisible, models,
  bezier patches, process lists, crowd, particles ...), gm 10/11 screen size, MenuSys_Draw, UIEngine_Render, LoadIndicator_Draw,
  gm slot 20 EndFrame, SSXAudioSystem_Update, WorldTriggerManager_Update                                           [S: call list of f_0022b008]
draw slots -> SprStream_BeginBlock / EndBlockAligned (SPR -> VifBuff DMA) + DeferMan record (0x80)
gm slot 20 PS2GraphicsMan_EndFrame 0x377a10:
  PS2GraphicsMan_RenderFrame 0x363490 (a DeferMan method): BuildBuckets, SortBuckets, EmitStateChange/EmitMaterialSetup (VU1 MPG CALL
  into .vutext + headers), TexMan_BindTextures (GIF PATH3 uploads into GifBuff), CALL/RET into packets, two phases (+ depth fog after
  layer 6, bloom/colour filter after layer 9)
  WaitSema(gm+0x5acc); gm+0x5aa4 = buildIndex; +0x5aa8/+0x5aac = VIF1 tadr ph1/ph2; +0x5ab0/+0x5ab4 = GIF tadr ph1/ph2;
  state[+0x5a90+i*4] = 2 (queued); SignalSema(+0x5acc); SignalSema(+0x5ac8); DeferMan_ResetFrame ...                [V]
Renderer_DmaKickThread 0x382760 (prio 5): waits until DMA/VIF1/GS are idle, VIF1 FIFO <- MSKPATH3 qword (0x44b9c0), GIF_MODE=4,
  GIF TADR/CHCR 0x105, VIF1 TADR/CHCR 0x185; phase 2 after the VIF1 DMA-end IRQ (runs callback gm+0x6d0c = LensFX first);
  flip: Renderer_SetDisplayRegs 0x382af0 (PMODE/SMODE2/DISPFB1/DISPLAY1 + GIF packet via Dma_SendGifNormal 0x371940)    [V]
```

### 4.2 Boundary functions

| Addr | Name | Signature / role | Conf |
|---|---|---|---|
| 0x377a10 | PS2GraphicsMan_EndFrame (slot 20) | (gm): builds the chain and **publishes the 4 chain addresses**. 12 callers (S): every module Render, VidEnginePlayer_PlayBlocking | V |
| 0x382760 | Renderer_DmaKickThread(gm) | the **only** VIF1/GIF chain kick per frame | V |
| 0x3826e0 / 0x382730 | AcquireFrameBuffer / IsRenderIdle (slots 17/18 wrappers) | already overridden (typed) | V |
| 0x369c28 | PS2GraphicsMan_WaitForRenderIdle (slot 19) | spins on slot 18 with em Begin/End + EATimer; called from GameModule_Update/Unload, FEModule_Unload, … (S) | V |
| 0x382af0 | Renderer_SetDisplayRegs(gm, flip, ?) | direct GS privileged registers + GIF packet | V (callers) / D |
| 0x37bd98 | Renderer_SetDisplayMode | VRAM FB/Z allocation, `sceGsResetGraph` | D |
| 0x375a40 | Renderer_Init (slot 2) | DMAC reset, kick thread, INTC 2 / DMAC ch1 handlers, `Vu0_UploadMicrocodeA/B` (VIF0 chain 0x43bd00, VU0 programs) | D/V |
| 0x371940 / 0x371f98 | Dma_SendGifNormal / Dma_SendGifChain | other direct GIF kicks: SetDisplayRegs, LensFX_v04; Gs_UploadImage ← PS2GraphicsMan_UploadToDrawBuffer (FMV/frame copies) | V |
| 0x38f460 / 0x38f668 | SprStream_BeginBlock / EndBlockAligned | packet builders' allocator (scratchpad → VifBuff via `Dma_FromScratchpad`). 21 / 18 callers = the complete list of packet producers (slots 61, 71, 81-86, tri strips, depth fog, bloom, colour filter, fog particles, grid mesh, Bezier FX, TexMan) | V |
| 0x368660 / 0x367db8 / 0x368170 / 0x3684f0 | TexMan_BindTextures / BuildUploadPacket / EmitImageUpload / EmitClutUpload | **texture uploads**: per-bucket VRAM alloc (VramHeap) + GIF IMAGE packets into GifBuff, TEX0/MIPTBP into the VIF chain | D (upload packet builder already overridden) |
| 0x367440 | TexMan_LoadTexture | (registers TexInfo: format, pixels, clut, mips, residency) via gm slot 46 `LoadTextureFromShape` (SSH/FSH), slot 47; freed by slot 50 | D |
| 0x364cd0 | DeferMan_EmitMaterialSetup (unnamed FUN) | **VU1 microprogram upload**: DMA `CALL` into the static `.vutext` chains (7 material types; programs f4babac5, 9790369f, d159716b, f55ee634, e72540c9, 9fa47842) + header UNPACK | D |

### 4.3 Draw entry points used by the race renderer

Caller lists come from a scan of generated code for `gm(+0x10d8)` virtual calls (**S**: misses calls where the gm pointer is cached in a saved register, e.g. Rider slot 99/100). Formats are from `render.md` (D).

| Slot | Impl | What / path | Race callers (S/D) |
|---|---|---|---|
| 17 / 20 | 0x395f60 / 0x377a10 | begin / end frame | f_0022b008 (GameModule Render), GameModule_RenderUI |
| 21/22, 31/32, 34-38, 25/26, 43, 66/67, 69 | view/matrix/projection/lights | state only (no packets) | Render, LensFX, Rider_*, HUD, OcclusionMan, Nash_GatherVisible |
| PS2BezierMan vt 18 (0x38b370) + EmitPatchType1/2 0x38ca70/0x38ce20, RenderPatchFX 0x38d968 | terrain Bezier patches, VU1 types 1/2/4 | GameModule_RenderBezierPatches (BeginBezierPass/EndBezierPass) |
| 95 / 96 | 0x37e238 / 0x369cb0 | DrawModel / DrawModelSimple (prebuilt mesh packets, VU1 type 3) | GameModule_RenderModels (Nash instance lists), Conveyor, SkyBox_Render (D), ModObj, DeadFade |
| 99 / 100 / 101 / 124 | 0x37a610 / 0x37b548 / 0x37ba50 / 0x396b40 | skinned rider + shadow (VU1 types 6/7) | Rider_RenderModel, Rider_DrawShadow (D) |
| 89 | 0x386e78 | shadow volume (multi-pass dest-alpha) | cBoardTrailFX_RenderBatch |
| 90 | 0x3883b8 | board wake (VU1 type 3, near clip entry 0x3a08) | cBoardWakeFX_Render |
| 82 / 83 / 84 | 0x380518 / 0x3807a0 / 0x380ce0 | particle emitters (VU1 type 4) | Emitter_Render, DynamicEmitter_Render, mEmitter_Render, cBoardSparkFX_Render |
| 85 / 86 | 0x381310 / 0x3816f0 | trails (VU1 type 5) | SnowBox_Render |
| 71 (+ BeginTriStrip 0x37a260 / EndTriStrip 0x37a430) | 0x379860 | tri strip (VU1 type 3), most-used (38 callers) | HUD_*, MenuSys*, GameModule_DrawFullscreenQuad, cSpeedBoostStreamerFX |
| 72 / 73 / 78 / 79 / 80 / 81 | | DIRECT (PATH2, EE-projected): indexed tris, line strip, sprites, glyphs | UIVector, UIListBox, LoadIndicator, Font_DrawText(W), cBoardSparkFX |
| 53 / 54 / 55 / 61 | | texture-upload draw, clear, textured sprite, transition | Rider_DrawShadow, EndFrame, cTransitionMan_Render |
| 76/77 + cPS2GridMesh_Draw | | grid meshes (fences, flags) | Fence_*, cFlagManager_Group* |
| non-slot | cPS2FogParticleMan_DrawParticles, PS2GraphicsMan_RenderDepthFog 0x36ac00 / RenderBloom 0x36c790 / RenderColorFilter 0x3904a0, LensFX (phase-2 callback gm+0x6d0c, slot 117) | post effects / fog | RenderFrame per-view passes, GameModule_RenderFogParticles/RenderLensFlares |

### 4.4 Native replacement (proposal)

1. **Stage 1:** override `Renderer_DmaKickThread` and `Renderer_SetDisplayRegs`. Hand the published chains `{vif1_ph1, gif_ph1, vif1_ph2, gif_ph2}` plus the phase-2 callback to a native consumer: today's VIF/VU1/GS path running in-process, or VU1 translated and only the GS replaced. Set buffer state 5 → 0 on present. The typed boundary is the chain-hand-off struct at gm+0x5a8c..+0x5ab4.
2. **Stage 2:** intercept at `DeferMan` records plus `EmitMaterialSetup` (the material type gives the shader; the packet gives the vertex data). Or override the draw slots one at a time, starting with the DIRECT 2D slots 72-81 and the slot-71 strips, then 95/99. Texture residency stays in TexMan; replace `TexMan_BindTextures` uploads with host texture creation.

---

## 5. Open questions

1. **GameModule Render (vt slot 7, 0x22b008..0x22c074) is not a Ghidra function.** Ghidra's list jumps from `Nash_GatherVisible` 0x22add8 to 0x22c078, but the recompiler emits `f_0022b008`. Create and name it (`GameModule_Render`) before typing the frame interface. xrefs for functions it calls (e.g. `GameModule_RenderUI`, `RenderModels`) show 0 callers because of this gap.
2. `PS2ExecMan` Begin/End (vt +0x5c/+0x64 → em+0x14080 sub-object) and `App_InvokeExecCallback`/em vt+0x74: what they do per frame (perf counters? DMA sync?).
3. SND bank memory kinds 8 vs 0x100 in `SND_BankAdd`/`SND_SpuMemAlloc`/`SND_IopUpload`: SPU2 RAM vs IOP RAM (the EE voice source?). Check SNDDRV's upload handler.
4. Layout of the SND tag buffer at 0x50c800 and the command block `*(0x50b9b0)` (voice address words at +0x20 and +0x1a0, sequence counter 0x50c7b8): needs a struct doc before (A) can be typed.
5. Does anything read `fh->lsn` (+0xc, set from `EAFile_PlatformGetLsn`)? If only movies or streams use it for direct `sceCdSt*` reads, the file seam may need an LSN shim.
6. Re-entrancy if `PlatformRead` completes synchronously: `CompleteRequest` → `ServiceQueue(0)` recursion depth with long queues. Probably safe (the CD cached-sector path already does it), but unverified.
7. Does the host0 path (`EAFile_PlatformInit(_,0)`) work end to end? `CdLookup` upper-cases names; host paths are case-sensitive on macOS with an extracted `DATA/`.
8. EATimer: the RCNT1 rate for `EATimer_Init(60)`, and what sits in the 8-slot ISR table 0x450da8 (SND_TimerServiceCallback?).
9. Rider draw slots 99/100 and SkyBox slot 95 do not show in the scan (the gm pointer is held in a register), so the per-slot caller list needs a decompile pass.
10. Movies: `AV_Player`/`VidEnginePlayer` (IPU decode → `PS2GraphicsMan_UploadToDrawBuffer` → `Gs_UploadImage`/`Dma_SendGifChain`) cross files, sound and rendering. They need their own small seam.
11. The `SCHl/SCCl/SCDl/SCEl` chunk set and codec IDs for `.mus` and speech streams (only `SCEl` was confirmed, in `SNDSTRM_QueueRequest`).
