# Phase 2 candidates, round 4

_Selected 2026-10-15 by script from the round-3 classifier database (scratchpad `cand2/db.json`; see phase2-candidates-2.md § How the data was made), excluding the 355 replaced functions. Per package: the ~25 named functions of its classes with the highest race-profile weight (inclusive samples + 5 per race-path flag), size 9..700 bytes, no VU0 micro-program calls. Package authors research semantics themselves (generated Rust + Ghidra), as in rounds 1-3. Fields: `cyc` = block costs, `loops`/`ev` = loops/event checks, `matchpc` = re-entrable dispatcher, `entries` = has an `__entries` twin (overridable: stub handles non-start entries), `direct`/`ind`/`sysc` = direct calls / indirect calls / syscalls, `vu`/`vuacc`/`fpu`/`mmi` = instruction counts, `spst` = stack stores, `race`/`prof` = race-path flag / inclusive samples._

## E1 EA SND sound library core (module `snd_lib`, 25 functions)

```text
0x003c19a8 SND_EeThread shard=s13 size=472 race=P prof=8993 callers=0
  cyc=[16, 5, 8, 2, 5, 3, 13, 3, 6, 6, 4, 6, 3, 2, 14, 7, 7, 3, 4, 1] loops=2 ev=3 matchpc=False entries=False direct=11 ind=5 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=8
  callees=['FUN_003b54e0', 'SND_Service', 'SND_SaveVu0Context', 'SND_RestoreVu0Context', 'SND_SaveSprDma', 'SND_RestoreSprDma']
0x003c8968 SND_EeMix shard=s14 size=212 race=P prof=171 callers=1
  cyc=[10, 2, 7, 2, 5, 7, 3, 7, 3, 7] loops=3 ev=3 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['SND_EeMixBlock']
0x003cde68 SND_UtkDecode shard=s14 size=524 race=P prof=89 callers=3
  cyc=[15, 2, 7, 6, 18, 6, 6, 22, 2, 16, 1, 5, 7, 2, 4, 12] loops=2 ev=2 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=3 mmi=0 spst=12
  callees=['SND_UtkDecodeFrame', 'memcpy']
0x003c9520 SND_DecUtk_DecodeLoop shard=s14 size=208 race=P prof=81 callers=0
  cyc=[9, 3, 13, 18, 2, 7] loops=1 ev=1 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=9
  callees=['SND_UtkSetInput', 'SND_UtkDecode', 'SND_UtkSetState']
0x003cd690 SND_UtkSynthesize shard=s14 size=96 race=P prof=51 callers=4
  cyc=[24] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['SND_UtkRcToLpc', 'SND_UtkLpcFilter']
0x003ca460 SND_DecEaxa_DecodeStream shard=s14 size=392 race=P prof=22 callers=0
  cyc=[14, 4, 2, 2, 17, 9, 6, 13, 9, 2, 3, 3, 2, 1, 2, 9] loops=1 ev=1 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=6 mmi=0 spst=10
  callees=['SNDSTRM_PopBlock', 'SNDSTRM_ReleaseBytes', 'SND_EaxaSetInput', 'SND_EaxaDecode', 'SND_EaxaSetHistory', 'memset']
0x003ccf90 SND_EaxaDecode shard=s14 size=424 race=P prof=21 callers=6
  cyc=[13, 2, 5, 5, 1, 12, 8, 4, 5, 6, 2, 20, 1, 9, 1, 3, 1, 8] loops=2 ev=2 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=4 mmi=0 spst=6
  callees=['SND_EaxaDecodeBlocks']
0x003b55f0 SND_Tick shard=s13 size=624 race=P prof=17 callers=1
  cyc=[18, 2, 7, 4, 3, 7, 3, 3, 7, 1, 6, 2, 8, 1, 3, 5, 3, 2, 2, 3, 2, 1, 4, 4, 3, 8, 4, 6, 3, 8, 2, 2, 5, 3, 4, 7] loops=2 ev=2 matchpc=False entries=False direct=6 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['SND_Stop', 'SND_ChannelCalcVolume', 'SND_ChannelCalcPitch', 'FUN_003c2438', 'SND_ChannelApplyPitch', 'SND_ChannelApplyVolume']
0x003cd590 SND_UtkRcToLpc shard=s14 size=252 race=P prof=13 callers=1
  cyc=[6, 7, 5, 10, 11, 4, 5, 8, 5, 2] loops=4 ev=4 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=22 mmi=0 spst=2
  callees=[]
0x003c9360 SND_DecUtk_DecodeMem shard=s14 size=148 race=P prof=8 callers=0
  cyc=[11, 2, 14, 4, 1, 5] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['SND_UtkDecode', 'memset']
0x003ca6a8 SND_DspClip16_Process shard=s14 size=248 race=P prof=8 callers=0
  cyc=[10, 8, 9, 28, 2, 5] loops=1 ev=1 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=14 vuacc=0 fpu=0 mmi=2 spst=4
  callees=[]
0x003cd518 SND_UtkInterpolate shard=s14 size=120 race=P prof=7 callers=1
  cyc=[10, 18, 2] loops=1 ev=1 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=18 mmi=0 spst=0
  callees=[]
0x003cd2b0 SND_UtkDecodeExcitation shard=s14 size=612 race=P prof=5 callers=2
  cyc=[17, 7, 17, 8, 2, 9, 2, 4, 2, 4, 7, 2, 3, 1, 5, 4, 5, 3, 1, 1, 2, 2, 10, 4, 2, 2, 1, 5, 4, 3, 1, 13] loops=1 ev=4 matchpc=True entries=False direct=6 ind=0 sysc=0 vu=0 vuacc=0 fpu=15 mmi=0 spst=12
  callees=['SND_UtkReadBits', 'SND_UtkSkipBits']
0x003c25e0 SND_ChannelApplyPitch shard=s13 size=508 race=P prof=2 callers=7
  cyc=[17, 4, 12, 5, 8, 7, 4, 2, 2, 6, 6, 3, 12, 2, 6, 6, 4, 12, 1, 8] loops=3 ev=3 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=6
  callees=['SND_EeVoiceSetPitch']
0x003cd260 SND_UtkSkipBits shard=s14 size=80 race=P prof=2 callers=3
  cyc=[9, 9, 2] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x003cd1f8 SND_UtkReadBits shard=s14 size=104 race=P prof=1 callers=15
  cyc=[15, 9, 2] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x003c0858 SND_SaveVu0Context shard=s13 size=160 race=P prof=1 callers=1
  cyc=[40] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=1 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x003c1578 SND_SifCmdHandler shard=s13 size=96 race=P prof=1 callers=0
  cyc=[6, 2, 2, 2, 4, 6, 2] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['SND_SifDmaComplete', 'iSignalSema']
0x003b5c60 SND_MemAlloc shard=s13 size=568 race=c2 prof=0 callers=31
  cyc=[19, 10, 2, 2, 3, 5, 3, 7, 8, 5, 2, 11, 2, 5, 20, 1, 25, 1, 1, 1, 9] loops=2 ev=2 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=15
  callees=['SND_MemClampBlock']
0x003b5e98 SND_MemFree shard=s13 size=200 race=c2 prof=0 callers=27
  cyc=[8, 2, 6, 4, 2, 11, 3, 12, 2] loops=2 ev=2 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x003c51e8 SND_MemCopy shard=s13 size=272 race=c prof=0 callers=21
  cyc=[3, 2, 3, 3, 2, 4, 3, 3, 20, 3, 6, 2, 1, 3, 6, 2, 2] loops=4 ev=6 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x003b89e0 SND_Stop shard=s13 size=88 race=c prof=0 callers=11
  cyc=[8, 2, 2, 5, 5] loops=1 ev=1 matchpc=True entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=3
  callees=['SND_HandleToChannel', 'SND_NextHandleChannel', 'SND_ChannelStopVoices']
0x003b8ab8 SND_IsHandleDone shard=s13 size=32 race=c prof=0 callers=8
  cyc=[8] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['SND_HandleToChannel']
0x003ba550 SND_ChannelRelease shard=s13 size=404 race=c2 prof=0 callers=8
  cyc=[12, 6, 3, 3, 3, 3, 3, 4, 3, 7, 6, 1, 2, 2, 19, 7, 2, 3, 6, 6] loops=1 ev=1 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x003c8f48 SND_DspInsertSorted shard=s14 size=104 race=c2 prof=0 callers=8
  cyc=[3, 5, 2, 3, 4, 2, 1, 2, 2, 2] loops=1 ev=1 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
```

## E2 SSXAudioSystem and bank monitors (module `audio_system`, 25 functions)

```text
0x00285bf8 SSXAudioSystem_Update shard=s07 size=412 race=P prof=41 callers=3
  cyc=[9, 6, 6, 3, 4, 2, 4, 6, 9, 6, 3, 2, 8, 4, 2, 17, 3, 2, 7] loops=0 ev=0 matchpc=False entries=False direct=14 ind=0 sysc=0 vu=0 vuacc=0 fpu=8 mmi=0 spst=3
  callees=['SSXAudioSystem_UpdateFade', 'FUN_00287f00', 'SSXAudioSystem_UpdateMixRamp', 'FUN_00289688', 'SSXAudioSystem_ResumeGameAudio', 'SSXAudioSystem_GetInstance']
0x002ac868 BankMonitorInstance_Update3D shard=s08 size=608 race=P prof=15 callers=1
  cyc=[13, 3, 9, 4, 1, 2, 3, 10, 6, 1, 5, 3, 5, 2, 14, 2, 5, 4, 1, 6, 3, 9, 1, 7, 1, 3, 3, 6, 4, 4, 3, 1, 8] loops=0 ev=0 matchpc=False entries=False direct=7 ind=1 sysc=0 vu=0 vuacc=0 fpu=31 mmi=0 spst=7
  callees=['BankMonitorInstance_IsDone', 'BankMonitorInstance_GetFalloffVolume', 'BankMonitor_CalcDopplerPitch', 'BankMonitorInstance_SetPan', 'BankMonitorInstance_SetPitch', 'BankMonitorInstance_SetVolume']
0x00292ef8 SSXAudioSystem_UpdateBoardSurfaceLoopRemote shard=s07 size=284 race=P prof=6 callers=0
  cyc=[14, 4, 3, 7, 11, 9, 3, 6, 1, 2, 11] loops=0 ev=0 matchpc=False entries=False direct=7 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=7
  callees=['Rider_GetMainState', 'FUN_00270280', 'SSXAudioSystem_GetGameModule', 'SSXAudioSystem_GetSurfaceSoundType', 'SSXAudioSystem_StartBoardSurfaceLoop', 'SSXAudioSystem_CalcSurfaceLoopParamsRemote']
0x002aba40 BankMonitorInstance_IsDone shard=s08 size=248 race=P prof=5 callers=2
  cyc=[9, 3, 2, 2, 3, 2, 8, 8, 2, 2, 7, 6, 3, 1, 1, 3] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=3
  callees=['SND_Lock', 'SND_Unlock', 'SNDSTRM_GetRequestStatus', 'SND_IsHandleDone', 'SNDMIDI_GetStatus']
0x00285d98 SSXAudioSystem_FindRider shard=s07 size=432 race=P prof=3 callers=31
  cyc=[10, 5, 2, 2, 5, 8, 3, 2, 4, 3, 6, 5, 2, 2, 1, 8, 3, 2, 6, 2, 2, 1, 8, 6, 2, 1, 7] loops=1 ev=4 matchpc=True entries=False direct=10 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['SSXAudioSystem_IsReplayPlaying', 'SSXAudioSystem_GetRaceMan']
0x00293538 SSXAudioSystem_CalcSurfaceLoopGroup0 shard=s07 size=516 race=P prof=2 callers=2
  cyc=[22, 4, 18, 3, 5, 4, 5, 68] loops=0 ev=0 matchpc=False entries=False direct=2 ind=1 sysc=0 vu=0 vuacc=0 fpu=39 mmi=0 spst=6
  callees=['Audio_EvalCurve5']
0x002b21e0 SongInstance_Update shard=s08 size=516 race=P prof=2 callers=2
  cyc=[11, 4, 31, 16, 4, 4, 6, 3, 3, 12, 8, 7, 10, 2, 1, 7] loops=0 ev=0 matchpc=False entries=False direct=5 ind=0 sysc=0 vu=0 vuacc=0 fpu=26 mmi=0 spst=6
  callees=['SongInstance_PostEvent', 'SongInstance_SetVolume', 'SongInstance_GetStreamHandle', 'MusicPlayer_IsSongDataLoaded', 'MusicPlayer_GetSongDataBuffer', 'SND_Lock']
0x002898a8 SSXAudioSystem_UpdateRiderWorldPaint shard=s07 size=316 race=P prof=2 callers=1
  cyc=[12, 12, 29, 4, 14, 8] loops=0 ev=0 matchpc=False entries=False direct=5 ind=4 sysc=0 vu=0 vuacc=0 fpu=12 mmi=0 spst=7
  callees=['FUN_002899e8', 'FUN_002899f8', 'SSXAudioSystem_ApplyMixPaint', 'FUN_0028d960', 'FUN_0028d988']
0x00289c98 SSXAudioSystem_ApplyMixPaint shard=s07 size=112 race=P prof=2 callers=1
  cyc=[12, 2, 9, 1, 4] loops=0 ev=0 matchpc=False entries=False direct=2 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['SSXAudioSystem_FindRider', 'SSXAudioSystem_SetMixPreset']
0x00293350 SSXAudioSystem_CalcSurfaceLoopParamsRemote shard=s07 size=188 race=P prof=2 callers=1
  cyc=[17, 7, 2, 7, 2, 6, 6] loops=0 ev=0 matchpc=False entries=False direct=4 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['FUN_002934d0', 'SSXAudioSystem_CalcSurfaceLoopRemoteGroup0', 'SSXAudioSystem_CalcSurfaceLoopRemoteGroup1', 'SSXAudioSystem_CalcSurfaceLoopRemoteGroup2']
0x002afc88 SpeechInstance_Update shard=s08 size=136 race=P prof=2 callers=1
  cyc=[9, 3, 9, 2, 5, 1, 1, 4] loops=0 ev=0 matchpc=False entries=False direct=1 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=3
  callees=['FUN_002b0290', 'FUN_003d7ec8', 'FUN_003d8040']
0x002992d8 SSXAudioSystem_StopBoostSound shard=s08 size=140 race=P prof=1 callers=2
  cyc=[9, 3, 2, 2, 4, 2, 8, 1, 4] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=3
  callees=['SSXAudioSystem_IsReplayPlaying', 'BankMonitor_StopHandle']
0x002abf60 BankMonitorInstance_SetVolume shard=s08 size=544 race=P prof=1 callers=2
  cyc=[12, 3, 2, 2, 3, 2, 5, 6, 7, 5, 4, 4, 4, 6, 6, 7, 5, 4, 4, 2, 4, 5, 6, 7, 5, 2, 4, 4, 6] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=24 mmi=0 spst=5
  callees=['FUN_002a9f30', 'StreamBuffer_Stop', 'SND_Lock', 'SND_Unlock', 'SND_SetVolume', 'SND_Stop']
0x0028f000 SSXAudioSystem_UpdateMusicFadeIn shard=s07 size=260 race=P prof=1 callers=1
  cyc=[12, 4, 4, 2, 10, 2, 2, 9, 2, 1, 13, 4] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=13 mmi=0 spst=3
  callees=['SSXAudioSystem_FindRider', 'FUN_00289c18', 'FUN_002b3c28', 'FUN_002b49e0']
0x00293168 SSXAudioSystem_CalcBoardBaseLoopParams shard=s07 size=296 race=P prof=1 callers=1
  cyc=[74] loops=0 ev=0 matchpc=False entries=True direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=11 mmi=0 spst=5
  callees=['Audio_EvalCurve5']
0x00293290 SSXAudioSystem_CalcSurfaceLoopParams shard=s07 size=188 race=P prof=1 callers=1
  cyc=[17, 7, 2, 7, 2, 6, 6] loops=0 ev=0 matchpc=False entries=True direct=4 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['FUN_002934d0', 'SSXAudioSystem_CalcSurfaceLoopGroup0', 'SSXAudioSystem_CalcSurfaceLoopGroup1', 'SSXAudioSystem_CalcSurfaceLoopGroup2']
0x00293410 SSXAudioSystem_CalcEdgeLoopParams shard=s07 size=188 race=P prof=1 callers=1
  cyc=[17, 7, 2, 7, 2, 6, 6] loops=0 ev=0 matchpc=False entries=False direct=4 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['FUN_002934d0', 'SSXAudioSystem_CalcEdgeLoopGroup0', 'SSXAudioSystem_CalcEdgeLoopGroup1', 'SSXAudioSystem_CalcEdgeLoopGroup2']
0x00293bb0 SSXAudioSystem_CalcEdgeLoopGroup0 shard=s07 size=528 race=P prof=1 callers=1
  cyc=[33, 3, 5, 4, 5, 66, 8, 8] loops=0 ev=0 matchpc=False entries=False direct=2 ind=1 sysc=0 vu=0 vuacc=0 fpu=41 mmi=0 spst=7
  callees=['Audio_EvalCurve5']
0x0029cca8 SSXAudioSystem_UpdateCountdownBeeps shard=s08 size=380 race=P prof=1 callers=1
  cyc=[10, 4, 3, 5, 2, 11, 4, 5, 8, 3, 3, 2, 2, 2, 2, 2, 19, 1, 1, 6] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=13 mmi=0 spst=5
  callees=['SSXAudioSystem_GetChannelGainPtr', 'SSXAudioSystem_IsReplayPlaying', 'SSXAudioSystem_GetRaceMan', 'BankMonitor_AddTimer', '__fixunssfsi']
0x002ab828 BankMonitor_TimerUpdate shard=s08 size=192 race=P prof=1 callers=1
  cyc=[7, 5, 3, 8, 2, 2, 4, 1, 5, 4, 3, 1, 3] loops=0 ev=0 matchpc=False entries=False direct=1 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=2
  callees=['BankMonitor_TimerKill']
0x002abd90 BankMonitorInstance_SetPan shard=s08 size=228 race=P prof=1 callers=1
  cyc=[13, 4, 3, 2, 2, 3, 2, 10, 9, 3, 1, 5] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['StreamBuffer_SetPan', 'SND_Lock', 'SND_Unlock', 'SND_SetPan', 'SNDMIDI_SetPan']
0x00292cb8 SSXAudioSystem_UpdateBoardBaseLoop shard=s07 size=228 race=P prof=1 callers=0
  cyc=[15, 4, 5, 6, 1, 2, 4, 6, 1, 2, 11] loops=0 ev=0 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=3 mmi=0 spst=9
  callees=['Rider_GetMainState', 'FUN_00270280', 'SSXAudioSystem_GetGameModule', 'SSXAudioSystem_CalcBoardBaseLoopParams', 'BankMonitorInstance_SetPitch']
0x00292da0 SSXAudioSystem_UpdateBoardSurfaceLoop shard=s07 size=340 race=P prof=1 callers=0
  cyc=[15, 5, 3, 7, 2, 6, 9, 8, 5, 4, 6, 1, 2, 12] loops=0 ev=0 matchpc=False entries=False direct=7 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=9
  callees=['Rider_GetMainState', 'FUN_00270280', 'SSXAudioSystem_GetGameModule', 'SSXAudioSystem_GetSurfaceSoundType', 'SSXAudioSystem_StartBoardSurfaceLoop', 'SSXAudioSystem_CalcSurfaceLoopParams']
0x00293018 SSXAudioSystem_UpdateBoardEdgeLoop shard=s07 size=332 race=P prof=1 callers=0
  cyc=[17, 2, 7, 2, 6, 12, 8, 5, 4, 6, 1, 2, 11] loops=0 ev=0 matchpc=False entries=True direct=8 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=8
  callees=['Rider_GetMainState', 'FUN_00270280', 'SSXAudioSystem_GetGameModule', 'SSXAudioSystem_GetSurfaceSoundType', 'SSXAudioSystem_StartBoardEdgeLoop', 'SSXAudioSystem_CalcEdgeLoopParams']
0x002adca0 BankMonitor_AddTimer shard=s08 size=212 race=c prof=0 callers=37
  cyc=[27, 11, 15] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=10
  callees=['BankMonitor_TimerSet', 'BankMonitor_FindFreeTimer']
```

## E3 world painter, irradiance, lighting (module `world_painter`, 25 functions)

```text
0x0022add8 Nash_GatherVisible shard=s05 size=556 race=P prof=179 callers=1
  cyc=[67, 3, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 34] loops=0 ev=0 matchpc=False entries=False direct=16 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=9
  callees=['FUN_00229f30', 'FUN_00229f80', 'FUN_0022a368', 'FUN_0022a408', 'FUN_0022a4a8', 'Nash_CollectInstances']
0x00336850 Nash_FindClosestContact shard=s11 size=392 race=P prof=17 callers=8
  cyc=[10, 7, 7, 4, 13, 1, 6, 3, 4, 3, 3, 2, 3, 3, 5, 1, 4, 4, 7, 2, 6] loops=3 ev=3 matchpc=False entries=False direct=1 ind=1 sysc=0 vu=0 vuacc=0 fpu=12 mmi=0 spst=5
  callees=['Nash_CollideVolume']
0x002c0778 WorldPainterQuery_UpdateAtPos shard=s08 size=664 race=P prof=9 callers=0
  cyc=[19, 1, 2, 11, 5, 7, 2, 7, 6, 5, 9, 2, 2, 12, 5, 7, 6, 10, 2, 9, 9, 7, 3, 2, 2, 1, 5, 1, 7] loops=1 ev=1 matchpc=True entries=False direct=1 ind=7 sysc=0 vu=0 vuacc=0 fpu=33 mmi=0 spst=5
  callees=['WorldPainterQuery_LookupCell']
0x00389810 Irradiance_ScaleInPlace shard=s12 size=48 race=P prof=4 callers=5
  cyc=[2, 8, 2] loops=1 ev=1 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=1 vuacc=0 fpu=1 mmi=0 spst=0
  callees=[]
0x00389730 Irradiance_Add shard=s12 size=112 race=P prof=2 callers=3
  cyc=[4, 12, 2, 7, 3] loops=2 ev=2 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=1 vuacc=0 fpu=0 mmi=0 spst=2
  callees=[]
0x002ee400 WorldPainter_GetSkyBox08 shard=s09 size=72 race=P prof=2 callers=1
  cyc=[18] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=1
  callees=[]
0x0038ac50 Irradiance_FindIndex shard=s12 size=124 race=P prof=1 callers=5
  cyc=[10, 2, 6, 2, 4, 1, 6] loops=1 ev=1 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['strcmp']
0x002ed1d0 WorldPainter_BlendViewColours shard=s09 size=360 race=P prof=1 callers=2
  cyc=[3, 2, 85] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=68 mmi=0 spst=24
  callees=[]
0x002f5b68 WorldLightMan_FindStrongestLights shard=s09 size=452 race=P prof=1 callers=2
  cyc=[19, 3, 7, 4, 1, 7, 9, 3, 5, 2, 3, 5, 1, 2, 5, 4, 9, 4, 4, 3, 13] loops=4 ev=4 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=13 mmi=0 spst=11
  callees=['FUN_002f5d30']
0x002c0a10 WorldPainterQuery_LookupCell shard=s08 size=72 race=P prof=1 callers=1
  cyc=[10, 3, 1, 4] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=2
  callees=['WPData_FindCellIndex']
0x002ee340 WorldPainter_GetIrradiance1 shard=s09 size=36 race=P prof=1 callers=1
  cyc=[9] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['IrradianceRef_GetEntry', 'WorldPainter_GetIrradianceName1']
0x002eeed0 WorldPainter_GetLightGlow28 shard=s09 size=72 race=P prof=1 callers=1
  cyc=[18] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=1
  callees=[]
0x002bcea8 tWPIGD_Camera_Blend shard=s08 size=144 race=P prof=1 callers=0
  cyc=[36] loops=0 ev=0 matchpc=False entries=False direct=0 ind=2 sysc=0 vu=0 vuacc=0 fpu=14 mmi=0 spst=4
  callees=[]
0x002bcf38 tWPIGD_Fog_Blend shard=s08 size=304 race=P prof=1 callers=0
  cyc=[76] loops=0 ev=0 matchpc=False entries=False direct=0 ind=2 sysc=0 vu=0 vuacc=0 fpu=54 mmi=0 spst=4
  callees=[]
0x0038abf8 Irradiance_GetEntry shard=s12 size=36 race=c prof=0 callers=6
  cyc=[9] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x00389260 Irradiance_Clear shard=s12 size=44 race=c prof=0 callers=3
  cyc=[2, 7, 2] loops=1 ev=1 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=5 mmi=0 spst=0
  callees=[]
0x00389520 Irradiance_AddDirectional shard=s12 size=52 race=c prof=0 callers=3
  cyc=[13] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=4 mmi=0 spst=1
  callees=['Irradiance_AddDirectionalScaled']
0x002ee220 WorldPainter_GetSunTexture shard=s09 size=72 race=c2 prof=0 callers=2
  cyc=[18] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=[]
0x002ee3b8 WorldPainter_GetCamera08 shard=s09 size=72 race=c prof=0 callers=2
  cyc=[18] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=1
  callees=[]
0x002ee570 WorldPainter_GetWeather20 shard=s09 size=72 race=c prof=0 callers=2
  cyc=[18] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=1
  callees=[]
0x002ee7c8 WorldPainter_GetWeather68 shard=s09 size=72 race=c prof=0 callers=2
  cyc=[18] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=1
  callees=[]
0x002ee810 WorldPainter_GetWeather70 shard=s09 size=72 race=c prof=0 callers=2
  cyc=[18] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=1
  callees=[]
0x002ee858 WorldPainter_GetWeather78 shard=s09 size=72 race=c prof=0 callers=2
  cyc=[18] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=1
  callees=[]
0x002ee9c0 WorldPainter_GetFog08 shard=s09 size=72 race=c prof=0 callers=2
  cyc=[18] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=1
  callees=[]
0x002eea08 WorldPainter_GetFog10 shard=s09 size=72 race=c prof=0 callers=2
  cyc=[18] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=1
  callees=[]
```

## E4 rider logic & metrics (module `rider_logic`, 25 functions)

```text
0x00111408 RiderBase_UpdateMainState shard=s00 size=152 race=P prof=75 callers=1
  cyc=[7, 6, 4, 4, 4, 4, 4, 2, 1, 2] loops=0 ev=0 matchpc=False entries=True direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['RiderStaticState_Update', 'RiderWipeoutState_Update', 'RiderHandplantState_Update', 'RiderAirState_Update', 'RiderRailslideState_Update', 'RiderGroundState_Update']
0x001114a0 RiderBase_PostUpdateMainState shard=s00 size=152 race=P prof=50 callers=1
  cyc=[7, 6, 4, 4, 4, 4, 4, 2, 1, 2] loops=0 ev=0 matchpc=False entries=True direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['RiderStaticState_PostUpdate', 'RiderWipeoutState_PostUpdate', 'RiderHandplantState_PostUpdate', 'RiderAirState_PostUpdate', 'RiderRailslideState_PostUpdate', 'RiderGroundState_PostUpdate']
0x00122448 Rider_RenderModel shard=s00 size=376 race=P prof=35 callers=1
  cyc=[10, 6, 4, 2, 2, 4, 1, 40, 5, 3, 1, 3, 4, 2, 1, 6] loops=0 ev=0 matchpc=False entries=False direct=1 ind=3 sysc=0 vu=0 vuacc=0 fpu=5 mmi=0 spst=5
  callees=['Rider_ComputeLOD', 'RiderModel_Render']
0x00120f20 Rider_BeginFrame shard=s00 size=328 race=P prof=14 callers=1
  cyc=[14, 6, 7, 6, 4, 2, 1, 5, 2, 3, 5, 3, 15, 4, 5] loops=0 ev=0 matchpc=False entries=False direct=4 ind=0 sysc=0 vu=0 vuacc=0 fpu=6 mmi=0 spst=4
  callees=['Rider_UpdateTargetSpeed', 'Rider_FindNearbyLights', 'Rider_ProbeGround', 'AI_GetFrameCounter', 'FUN_001448a8', 'OverlapList_GatherAll']
0x00111728 RiderBase_HandleSubStateInput shard=s00 size=264 race=P prof=13 callers=1
  cyc=[7, 6, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 1, 2] loops=0 ev=0 matchpc=False entries=True direct=10 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['RiderGateSub_HandleInput', 'RiderFinishLineSub_HandleInput', 'RiderWipeoutSub_HandleInput', 'RiderBumpSub_HandleInput', 'RiderPrewindSub_HandleInput', 'RiderResetSub_HandleInput']
0x001125c0 Rider_UpdateAIPathProgress shard=s00 size=560 race=P prof=10 callers=1
  cyc=[14, 8, 6, 20, 7, 3, 7, 4, 10, 5, 8, 2, 1, 9, 4, 4, 3, 7, 3, 3, 2, 1, 1, 8] loops=0 ev=0 matchpc=False entries=False direct=6 ind=1 sysc=0 vu=0 vuacc=0 fpu=51 mmi=0 spst=6
  callees=['Rider_GetAIPathLookAheadDist', 'Rider_SaveAIPathLookAhead', 'FUN_001125b8', 'Rider_SelectAIPath', 'Path_GetClosestPoint', 'Path_GetClosestPointFrom']
0x001220d8 Rider_BuildLightSet shard=s00 size=412 race=P prof=8 callers=1
  cyc=[21, 7, 20, 3, 2, 3, 6, 13, 3, 16, 9] loops=3 ev=3 matchpc=False entries=False direct=1 ind=2 sysc=0 vu=0 vuacc=0 fpu=14 mmi=0 spst=13
  callees=['Rider_GetHipsPosition', 'WorldPainter_GetViewIrradiance', 'WorldPainter_GetLighting30', 'Irradiance_Clear', 'Irradiance_AddDirectional', 'Vec3_AddTo']
0x00112338 Rider_UpdateRaceProgress shard=s00 size=592 race=P prof=6 callers=1
  cyc=[30, 6, 4, 7, 4, 10, 5, 9, 2, 1, 2, 7, 3, 4, 3, 3, 4, 4, 2, 3, 11, 2, 2, 3, 4, 3, 10] loops=3 ev=3 matchpc=False entries=False direct=7 ind=1 sysc=0 vu=0 vuacc=0 fpu=12 mmi=0 spst=8
  callees=['Rider_HandleRacePathEvent', 'Rider_SelectRacePath', 'Rider_UpdateDistToFinish', 'Path_GetLength', 'EA_MemCopy']
0x00108a48 Rider_FindRail shard=s00 size=476 race=P prof=4 callers=2
  cyc=[9, 5, 1, 19, 15, 21, 1, 5, 7, 8, 2, 5, 5, 1, 7, 3, 5] loops=1 ev=1 matchpc=True entries=False direct=3 ind=0 sysc=0 vu=13 vuacc=4 fpu=8 mmi=0 spst=7
  callees=['FUN_001086b8', 'Rider_GetMainState', 'FUN_001446a0', 'AnimController_GetLayerAnimEntry', 'AnimController_GetLayerSequence', 'OverlapList_FindNearestRail']
0x00114130 Rider_HandleBoostInput shard=s00 size=360 race=P prof=2 callers=13
  cyc=[10, 4, 2, 5, 5, 6, 6, 2, 7, 5, 3, 4, 5, 2, 6, 5, 6, 1, 6] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=30 mmi=0 spst=5
  callees=['SSXAudioSystem_GetInstance', 'SSXAudioSystem_StartBoostSound', 'SSXAudioSystem_StopBoostSound', 'FUN_00299368', 'RiderBase_RecordCheatCodeInput']
0x001127f0 Rider_SelectRacePath shard=s00 size=604 race=P prof=2 callers=3
  cyc=[30, 3, 18, 9, 6, 2, 4, 1, 9, 9, 2, 1, 39, 2, 2, 14] loops=1 ev=1 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=20 vuacc=6 fpu=28 mmi=0 spst=21
  callees=['Rider_GetAIPathLookAheadDist', 'Path_GetClosestPoint', 'Path_GetPointAtDistance', 'PathData_FindNearTrackPaths']
0x0011b3f8 Rider_UpdateTargetSpeed shard=s00 size=640 race=P prof=2 callers=1
  cyc=[31, 1, 5, 1, 3, 1, 6, 5, 5, 1, 4, 21, 17, 3, 3, 14, 6, 2, 1, 3, 1, 7, 3, 6, 3, 1, 6] loops=1 ev=1 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=73 mmi=0 spst=5
  callees=['Rider_GetMainState', 'BECharInt_GetAttrib0Factor', 'BELibrary_GetInstance', 'BELibrary_GetInterface']
0x00121950 Rider_UpdateHeadingAndAudio shard=s00 size=332 race=P prof=2 callers=1
  cyc=[15, 13, 4, 4, 3, 7, 3, 3, 2, 1, 6, 3, 4, 2, 4, 9] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=46 mmi=0 spst=7
  callees=['FUN_00122cf0', 'Rider_IsOnGround', 'SSXAudioSystem_UpdateRiderWorldPaint', 'SSXAudioSystem_GetInstance', 'Math_Atan']
0x001036a0 RiderAnim_UpdateSequence shard=s00 size=628 race=P prof=2 callers=0
  cyc=[18, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 6, 2, 3, 3, 2, 2, 2, 1, 1, 6] loops=1 ev=1 matchpc=False entries=True direct=17 ind=0 sysc=0 vu=0 vuacc=0 fpu=19 mmi=0 spst=6
  callees=['RiderAnim_FireSoundEvents', 'RiderAnim_UpdateSeqType03', 'RiderAnim_UpdateSeqType04', 'RiderAnim_UpdateSeqType05', 'RiderAnim_UpdateSeqType06', 'RiderAnim_UpdateSeqType0A']
0x00114cc0 Rider_TurnAroundIfBackward shard=s00 size=244 race=P prof=1 callers=5
  cyc=[10, 15, 32, 4] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=14 vuacc=7 fpu=17 mmi=0 spst=12
  callees=['Rider_ToggleStance', 'Rider_BuildWorldMatrix']
0x002f6ac8 RiderBase_RecordCheatCodeInput shard=s09 size=92 race=P prof=1 callers=3
  cyc=[3, 4, 2, 2, 3, 2, 2, 3, 2] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x00104358 RiderAnim_UpdateSeqType07 shard=s00 size=160 race=P prof=1 callers=1
  cyc=[17, 1, 11, 2, 3, 6] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=14 mmi=0 spst=4
  callees=['AnimSequence_UpdateWeightFade', 'AnimSequence_SetTrackTime', 'AnimSeqList_Remove']
0x001049c0 RiderAnim_UpdateSeqType02 shard=s00 size=124 race=P prof=1 callers=1
  cyc=[15, 3, 2, 4, 1, 6] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=6 mmi=0 spst=4
  callees=['AnimSequence_AdvanceTrack', 'AnimSequence_UpdateWeightFade', 'AnimSeqList_Remove']
0x00125228 Rider_CheckRaceFinish shard=s00 size=420 race=P prof=1 callers=1
  cyc=[14, 6, 1, 2, 10, 4, 4, 5, 5, 4, 6, 8, 11, 1, 3, 6, 3, 1, 3, 2, 6] loops=0 ev=0 matchpc=False entries=False direct=2 ind=1 sysc=0 vu=0 vuacc=0 fpu=22 mmi=0 spst=4
  callees=['Rider_OnFinishRace', 'BELibrary_GetInstance', 'BELibrary_GetInterface', 'SSXAudioSystem_GetInstance', 'SSXAudioSystem_ArcadeSay20A8', '__fixunssfsi']
0x00310c48 RiderModel_FindBone shard=s10 size=160 race=c2 prof=0 callers=96
  cyc=[19, 1, 5, 3, 4, 1, 7] loops=1 ev=1 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['strcmp']
0x0010e098 Rider_AddBoost shard=s00 size=396 race=c prof=0 callers=37
  cyc=[12, 5, 3, 13, 15, 1, 4, 3, 3, 5, 3, 3, 4, 3, 5, 5, 4, 1, 1, 6] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=28 mmi=0 spst=5
  callees=['FUN_0010e028', 'BECharInt_GetAttrib2Factor', 'BELibrary_GetInstance', 'BELibrary_GetInterface', 'SSXAudioSystem_GetInstance', 'FUN_0029ab08']
0x00113f88 Rider_SetSlew3And4Targets shard=s00 size=424 race=c prof=0 callers=12
  cyc=[4, 4, 4, 4, 3, 4, 5, 5, 1, 5, 3, 1, 6, 5, 13, 1, 1, 6, 6, 5, 5, 1, 1, 8, 5] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=5 vuacc=2 fpu=63 mmi=0 spst=0
  callees=[]
0x00117400 RiderMetrix_IsHUDMessageActive shard=s00 size=172 race=c prof=0 callers=12
  cyc=[4, 2, 3, 5, 5, 6, 5, 1, 10, 2] loops=1 ev=1 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x00113e80 Rider_SetSlew0Target shard=s00 size=180 race=c prof=0 callers=10
  cyc=[18, 11, 1, 5, 3, 2, 5] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=6 vuacc=2 fpu=23 mmi=0 spst=0
  callees=[]
0x00117838 RiderMetrix_ResetTrick shard=s00 size=200 race=c2 prof=0 callers=8
  cyc=[50] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=9 mmi=0 spst=1
  callees=['EA_MemSet']
```

## E5 graphics manager & draw deferral (module `graphics`, 25 functions)

```text
0x00369cb0 GraphicsMan_DrawModelSimple shard=s12 size=68 race=P prof=86 callers=0
  cyc=[17] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=1
  callees=[]
0x00386bd0 PS2GraphicsMan_v111 shard=s12 size=284 race=P prof=18 callers=0
  cyc=[4, 1, 5, 2, 25, 1, 22, 8, 3] loops=2 ev=2 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=16 vuacc=4 fpu=0 mmi=0 spst=1
  callees=[]
0x002f0a98 PS2VisualEffectsMan_Update shard=s09 size=568 race=P prof=12 callers=0
  cyc=[115, 3, 18, 1, 5] loops=1 ev=1 matchpc=False entries=False direct=1 ind=18 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['PS2VisualEffectsMan_ApplyWorldPainter']
0x0037d968 PS2GraphicsMan_UpdateViewMatrices shard=s12 size=636 race=P prof=11 callers=35
  cyc=[159] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=48 vuacc=36 fpu=0 mmi=0 spst=13
  callees=['DeferMan_InternMatrixRecord']
0x0038f708 SprStream_Upload shard=s12 size=44 race=P prof=10 callers=5
  cyc=[11] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['Dma_ToScratchpad']
0x00364050 DeferMan_SortBuckets shard=s12 size=496 race=P prof=10 callers=1
  cyc=[28, 2, 28, 2, 19, 1, 7, 3, 18, 6, 10] loops=4 ev=4 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=6
  callees=['SprStream_BeginBlock', 'SprStream_NextIndex', 'EA_MemSet']
0x0037e120 PS2GraphicsMan_EmitModelHeader shard=s12 size=280 race=P prof=6 callers=2
  cyc=[48, 3, 3, 6, 10] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=1 spst=9
  callees=['FUN_0037e040', 'PS2GraphicsMan_EmitModelLighting']
0x003953b8 PS2GraphicsMan_GetWorldToClipMatrix shard=s13 size=92 race=P prof=6 callers=0
  cyc=[23] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=3
  callees=['PS2GraphicsMan_EnsureViewMatrices']
0x0038f668 SprStream_EndBlockAligned shard=s12 size=60 race=P prof=5 callers=25
  cyc=[15] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['SprStream_EndBlock']
0x00395000 DeferMan_FindBucket shard=s13 size=188 race=P prof=5 callers=5
  cyc=[9, 3, 5, 3, 3, 4, 1, 1, 2, 2, 4, 6, 2, 2] loops=1 ev=1 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x0038f598 SprStream_EndBlock shard=s12 size=208 race=P prof=5 callers=3
  cyc=[8, 3, 3, 7, 20, 5, 6] loops=0 ev=0 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['Dma_FromScratchpad', 'SprStream_NextIndex', 'Dma_WaitFromScratchpad']
0x00394ed0 DeferMan_InternState shard=s13 size=304 race=P prof=5 callers=3
  cyc=[10, 4, 4, 4, 2, 4, 1, 2, 3, 3, 2, 6, 31] loops=2 ev=2 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x0038f4f8 SprStream_BeginBlockUpload shard=s12 size=100 race=P prof=3 callers=2
  cyc=[25] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['Dma_ToScratchpad', 'SprStream_BeginBlock']
0x003883b8 PS2GraphicsMan_v90 shard=s12 size=548 race=P prof=3 callers=0
  cyc=[33, 2, 9, 2, 13, 6, 33, 39] loops=1 ev=1 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=8 mmi=0 spst=11
  callees=['PS2GraphicsMan_UpdateViewMatrices', 'PS2GraphicsMan_EmitBoardWakeStrips', 'EA_MemSet']
0x00364360 DeferMan_EmitDefaultState shard=s12 size=596 race=P prof=2 callers=3
  cyc=[16, 7, 126] loops=1 ev=1 matchpc=False entries=False direct=10 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=10
  callees=['GsPkt_AddAlpha', 'GsPkt_AddClamp', 'GsPkt_AddZbuf', 'GsPkt_AddTest', 'GsPkt_Finish', 'DeferMan_EmitMaterialSetup']
0x00362cc8 DeferMan_ResetFrame shard=s12 size=288 race=P prof=2 callers=2
  cyc=[72] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['SprStream_Download', 'EA_MemSet', 'InvalidDCache']
0x003904a0 PS2GraphicsMan_RenderColorFilter shard=s12 size=328 race=P prof=2 callers=1
  cyc=[13, 3, 3, 10, 5, 5, 6, 5, 5, 1, 1, 1, 16, 1, 7] loops=0 ev=0 matchpc=False entries=False direct=5 ind=0 sysc=0 vu=0 vuacc=0 fpu=14 mmi=0 spst=7
  callees=['TexMan_EmitPath3Window', 'SprStream_BeginBlock', 'SprStream_EndBlockAligned', 'PS2GraphicsMan_ResetColorFilter', 'PS2GraphicsMan_EmitColorFilterPacket']
0x003956b0 PS2GraphicsMan_EnsureViewMatrices shard=s13 size=40 race=P prof=2 callers=1
  cyc=[5, 3, 2] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['PS2GraphicsMan_UpdateViewMatrices']
0x003765b8 PS2GraphicsMan_PushView shard=s12 size=432 race=P prof=2 callers=0
  cyc=[108] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=6 mmi=0 spst=1
  callees=['PS2GraphicsMan_CopyViewState']
0x00376a70 PS2GraphicsMan_SetViewport shard=s12 size=288 race=P prof=2 callers=0
  cyc=[22, 2, 8, 6, 1, 33] loops=0 ev=0 matchpc=False entries=False direct=2 ind=1 sysc=0 vu=0 vuacc=0 fpu=30 mmi=0 spst=6
  callees=['DeferMan_SetScissor', 'PS2GraphicsMan_SetScissorClamped']
0x0037d348 PS2GraphicsMan_v53 shard=s12 size=260 race=P prof=2 callers=0
  cyc=[24, 31, 10] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['TexMan_BuildUploadPacket']
0x0038f460 SprStream_BeginBlock shard=s12 size=152 race=P prof=1 callers=29
  cyc=[10, 2, 2, 1, 9, 14] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['Dma_WaitFromScratchpad']
0x00364240 DeferMan_ComputeSortKey shard=s12 size=288 race=P prof=1 callers=5
  cyc=[10, 3, 2, 2, 2, 2, 2, 2, 3, 1, 5, 6, 2, 2, 2, 2, 2, 2, 1, 7, 3, 9] loops=0 ev=0 matchpc=False entries=True direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x0038f300 SprStream_SetMode shard=s12 size=352 race=P prof=1 callers=3
  cyc=[6, 5, 6, 2, 2, 2, 2, 2, 1, 1, 4, 7, 2, 2, 2, 2, 2, 1, 1, 3, 3, 12, 6, 4, 1, 4, 3] loops=1 ev=1 matchpc=True entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['Dma_WaitFromScratchpad', 'Dma_WaitToScratchpad']
0x0036ac00 PS2GraphicsMan_RenderDepthFog shard=s12 size=544 race=P prof=1 callers=1
  cyc=[17, 3, 3, 8, 3, 1, 89, 1, 11] loops=0 ev=0 matchpc=False entries=False direct=14 ind=1 sysc=0 vu=0 vuacc=0 fpu=3 mmi=0 spst=13
  callees=['VramHeap_FreeBlock', 'VramHeap_Alloc', 'TexMan_EmitPath3Window', 'PS2GraphicsMan_BuildDepthFogClut', 'PS2GraphicsMan_EmitDepthFogClutUpload', 'PS2GraphicsMan_ResetDepthFogParams']
```

## E6 file & stream I/O (module `streaming`, 25 functions)

```text
0x003e3be0 EAFile_CdReadThread shard=s14 size=408 race=P prof=8993 callers=0
  cyc=[14, 6, 3, 5, 4, 4, 3, 2, 9, 3, 10, 6, 3, 6, 8, 7, 8, 1] loops=1 ev=5 matchpc=False entries=False direct=5 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=6
  callees=['EAFile_CompleteRequest', 'EAFile_CdIssueRead', 'EAFile_CdMountDisc', 'EA_MemCopy', 'sceCdGetError', 'WaitSema']
0x003d5128 PATH_Update shard=s14 size=356 race=P prof=9 callers=2
  cyc=[15, 4, 3, 22, 4, 2, 3, 6, 2, 1, 2, 4, 1, 4, 4, 12] loops=1 ev=1 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=10
  callees=['PATH_PlatRefreshStatus', 'PATH_PlatGetTimeMs', 'FUN_003d4b00', 'PATH_UpdateTracks']
0x003e0c88 StreamFile_QueueRange shard=s14 size=248 race=P prof=6 callers=5
  cyc=[14, 5, 2, 16, 3, 4, 4, 4, 3, 7] loops=0 ev=0 matchpc=False entries=False direct=7 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['StreamFile_Validate', 'StreamFile_AllocRequest', 'StreamFile_EnqueueRequest', 'StreamFile_StartNextRequest', 'EAMutex_Lock', 'EAMutex_Unlock']
0x003e0270 StreamFile_StartNextRequest shard=s14 size=352 race=P prof=6 callers=4
  cyc=[14, 3, 3, 1, 2, 3, 4, 4, 6, 2, 8, 5, 7, 6, 4, 6, 3, 1, 6] loops=0 ev=0 matchpc=False entries=False direct=7 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['EAFile_SetRequestCallback', 'EAFile_QueueOpen', 'EAFile_QueueClose', 'StreamFile_IssueRead', 'EAMutex_Lock', 'EAMutex_Unlock']
0x003b76e8 SNDSTRM_QueueRequest shard=s13 size=304 race=P prof=6 callers=3
  cyc=[17, 3, 2, 8, 2, 6, 2, 19, 8, 9] loops=0 ev=0 matchpc=False entries=False direct=5 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=7
  callees=['SND_Lock', 'SND_Unlock', 'SNDSTRM_GetStream', 'SND_ListPushBack', 'SND_ListPopFront', 'StreamFile_QueueRange']
0x003b78d8 SNDSTRM_Queue shard=s13 size=28 race=P prof=6 callers=3
  cyc=[7] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['SNDSTRM_QueueRequest']
0x003d4f10 PATH_UpdateTracks shard=s14 size=532 race=P prof=6 callers=1
  cyc=[16, 5, 4, 2, 2, 3, 5, 3, 8, 2, 2, 3, 3, 2, 3, 2, 3, 3, 2, 3, 4, 5, 4, 6, 2, 2, 2, 3, 4, 1, 2, 1, 3, 1, 2, 4, 11] loops=2 ev=2 matchpc=False entries=False direct=7 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=10
  callees=['FUN_002ae9f8', 'FUN_002aebd0', 'PATH_PlatGetTrackVolume', 'PATH_ServiceQueues', 'FUN_003d3d20', 'PATH_TrackGetQueuedCount']
0x003dd648 EAFile_QueueRead shard=s14 size=212 race=P prof=5 callers=8
  cyc=[27, 4, 22] loops=0 ev=0 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=8
  callees=['EAFile_ServiceQueue', 'FUN_003de668', 'EAFile_AllocRequest']
0x003e3350 EAFile_PlatformRead shard=s14 size=92 race=P prof=5 callers=2
  cyc=[8, 10, 3, 2] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['EAFile_CdRead']
0x003e3b00 EAFile_CdIssueRead shard=s14 size=220 race=P prof=5 callers=2
  cyc=[14, 2, 1, 4, 1, 5, 3, 7, 4, 2, 4, 8] loops=1 ev=1 matchpc=True entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=7
  callees=['EAFile_DelayTicks', 'sceCdSync', 'sceCdRead']
0x003e4648 EAFile_CdRead shard=s14 size=656 race=P prof=5 callers=1
  cyc=[34, 8, 3, 1, 2, 3, 6, 3, 7, 9, 13, 3, 8, 9, 7, 3, 7, 9, 5, 8, 2, 7, 7] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['EAFile_CompleteRequest', 'EA_MemCopy', 'SignalSema']
0x002ae020 PATH_PlatTimerCallback shard=s08 size=40 race=P prof=5 callers=0
  cyc=[4, 2, 4] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=[]
0x002ae7a8 PATH_PlatQueue shard=s08 size=592 race=P prof=4 callers=3
  cyc=[27, 3, 3, 7, 5, 2, 26, 2, 6, 3, 9, 3, 4, 5, 1, 5, 2, 4, 1, 11, 5, 2, 12] loops=2 ev=2 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=10
  callees=['PATH_PlatFindBankSlot', 'PATH_PlatGetTimeMs', 'PATH_PlatSetVolume', 'SND_Lock', 'SND_Unlock', 'SND_Play']
0x002af370 PATH_PlatGetTimeMs shard=s08 size=68 race=P prof=2 callers=4
  cyc=[17] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=7 mmi=0 spst=1
  callees=['EATimer_GetTicks', '__fixunssfsi']
0x003e0170 StreamFile_OnRead shard=s14 size=252 race=P prof=2 callers=1
  cyc=[12, 7, 6, 12, 2, 2, 5, 1, 3, 4, 3, 6] loops=0 ev=0 matchpc=False entries=False direct=6 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['EAFile_ReleaseRequest', 'StreamFile_ParseChunks', 'StreamFile_StartNextRequest', 'StreamFile_IssueRead', 'EAMutex_Lock', 'EAMutex_Unlock']
0x003e0118 StreamFile_OnClose shard=s14 size=84 race=P prof=2 callers=0
  cyc=[13, 4, 4] loops=0 ev=0 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=2
  callees=['EAFile_SetRequestCallback', 'EAFile_ReleaseRequest', 'EAFile_QueueOpen']
0x003dd4e8 EAFile_QueueOpen shard=s14 size=184 race=P prof=1 callers=6
  cyc=[26, 4, 16] loops=0 ev=0 matchpc=False entries=False direct=4 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=6
  callees=['EAFile_ServiceQueue', 'FUN_003de668', 'EAFile_AllocRequest', 'EAFile_AllocFileHandle', 'strncpy']
0x003d3030 PATH_GetTrackStream shard=s14 size=40 race=P prof=1 callers=2
  cyc=[6, 1, 3] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['PATH_FindTrack']
0x003e00d8 StreamFile_OnOpen shard=s14 size=60 race=P prof=1 callers=0
  cyc=[8, 3, 4] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=2
  callees=['EAFile_ReleaseRequest', 'StreamFile_IssueRead']
0x003df9d8 AsyncFile_GetResult shard=s14 size=280 race=c prof=0 callers=25
  cyc=[19, 3, 1, 4, 2, 3, 6, 1, 2, 2, 2, 4, 2, 4, 2, 2, 3, 1, 7] loops=1 ev=1 matchpc=False entries=False direct=6 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=6
  callees=['EAFile_WaitRequest', 'AsyncFile_FromHandle', 'AsyncFile_Free', 'EAMutex_Lock', 'EAMutex_Unlock']
0x003dd310 EAFile_ReleaseRequest shard=s14 size=292 race=c prof=0 callers=18
  cyc=[15, 2, 3, 2, 2, 7, 6, 2, 3, 2, 2, 8, 2, 5, 2, 2, 8] loops=0 ev=0 matchpc=False entries=True direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['EAFile_FreeRequest', 'EAFile_FreeFileHandle']
0x003df980 AsyncFile_IsDone shard=s14 size=88 race=c prof=0 callers=15
  cyc=[12, 2, 8] loops=0 ev=0 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=3
  callees=['AsyncFile_FromHandle', 'EAMutex_Lock', 'EAMutex_Unlock']
0x003b7c40 SNDSTRM_GetRequestStatus shard=s13 size=320 race=c prof=0 callers=11
  cyc=[12, 5, 2, 5, 3, 2, 2, 4, 2, 29, 6, 1, 1, 6] loops=0 ev=0 matchpc=False entries=False direct=6 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=8
  callees=['SNDSTRM_GetStream', 'SNDSTRM_FindRequest', 'SND_UMul32x32', 'SND_UDiv64by32']
0x003e35b0 EAFile_DelayTicks shard=s14 size=104 race=c prof=0 callers=11
  cyc=[26] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=6
  callees=['SetAlarm', 'CreateSema', 'DeleteSema', 'WaitSema']
0x003de420 EAFile_CompleteRequest shard=s14 size=164 race=c prof=0 callers=10
  cyc=[12, 3, 3, 6, 5, 6, 3, 3] loops=0 ev=0 matchpc=False entries=False direct=1 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=2
  callees=['EAFile_ServiceQueue']
```

## E7 replay, scripts, game module (module `replay_script`, 25 functions)

```text
0x0022c410 GameModule_DrawBezierPatch shard=s05 size=528 race=P prof=11 callers=2
  cyc=[75, 1, 4, 1, 4, 1, 4, 1, 41] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=94 mmi=0 spst=11
  callees=[]
0x0026d760 Replay_RecordSnapshot shard=s07 size=112 race=P prof=4 callers=4
  cyc=[28] loops=0 ev=0 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['WorldState_Write', 'FUN_0026ed88', 'Replay_AllocSnapshot']
0x0022c790 GameModule_RenderLensFlares shard=s05 size=156 race=P prof=4 callers=1
  cyc=[17, 4, 3, 4, 11] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['LensFX_ClearSources', 'LensFX_AddSources', 'LensFX_Render']
0x00309c88 WScriptMan_CallFunc shard=s10 size=148 race=P prof=2 callers=7
  cyc=[37] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=6
  callees=['FUN_002224a8', 'LunoVM_Call', 'LunoVMRegister_dtor', 'FUN_00226600', 'World_GetSectionScriptFunc']
0x0026c5f8 PlaybackMan_PollFileLoad shard=s07 size=240 race=P prof=2 callers=3
  cyc=[9, 5, 9, 3, 27, 2, 5] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=6
  callees=['MEM_Free', 'operator_new_named', 'AsyncFile_IsDone', 'AsyncFile_GetResult', 'EA_MemCopy']
0x0030a688 WScriptMan_RunObjScript5 shard=s10 size=80 race=P prof=2 callers=1
  cyc=[12, 3, 5] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=3
  callees=['WScriptMan_CallFunc', 'FUN_003a6b78']
0x00231840 GameModule_CheckPauseRequest shard=s06 size=632 race=P prof=1 callers=3
  cyc=[12, 3, 10, 8, 2, 2, 3, 2, 4, 4, 2, 7, 10, 3, 2, 2, 2, 3, 3, 1, 5, 2, 2, 5, 3, 6, 10, 3, 2, 6, 2, 2, 9, 4, 1, 1, 10] loops=2 ev=2 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=8
  callees=['BENewPlayerInt_GetPadIndex', 'BELibrary_GetInstance', 'BELibrary_GetInterface', 'FUN_00231ab8', 'InputBinding_EvalBool']
0x00231250 GameModule_RequestState shard=s06 size=40 race=c prof=0 callers=22
  cyc=[5, 2, 3] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['GameModule_ProcessStateChange']
0x00230698 GameModule_GetTransitionMan shard=s06 size=16 race=c2 prof=0 callers=17
  cyc=[4] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x002706f0 Replay_Finish shard=s07 size=64 race=c prof=0 callers=10
  cyc=[7, 3, 6] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=2
  callees=['Replay_SetState', 'Replay_EndPlayback']
0x00230488 GameModule_QuitToLoadScreen shard=s06 size=208 race=c2 prof=0 callers=8
  cyc=[16, 3, 2, 2, 2, 12, 11, 4] loops=0 ev=0 matchpc=False entries=False direct=4 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=3
  callees=['GameModule_RequestState', 'cPreGameLoadScreen_ctor', 'cPreFELoadScreen_ctor', 'FUN_00258ae0', 'cVoipMan_Stop', 'Replay_Finish']
0x002703f0 Replay_StepFrame shard=s07 size=324 race=c2 prof=0 callers=8
  cyc=[8, 3, 3, 3, 2, 2, 5, 3, 3, 2, 5, 5, 3, 22, 5, 2, 5] loops=1 ev=1 matchpc=False entries=False direct=6 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['ChaseCamController_ClearOverride', 'GameViewMan_UpdateCameras', 'FUN_0026d4d8', 'Replay_RestoreSnapshot', 'Replay_FindSnapshot', 'Replay_SetState']
0x0026e448 Replay_FindSnapshot shard=s07 size=28 race=c2 prof=0 callers=6
  cyc=[7] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['SnapshotList_FindByFrame']
0x0026f8a0 Replay_BeginPlayback shard=s07 size=220 race=c prof=0 callers=6
  cyc=[15, 3, 3, 13, 4, 2, 2, 2, 4, 1, 1, 5] loops=0 ev=0 matchpc=False entries=False direct=8 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=5
  callees=['Replay_EnableReplayCameras', 'FUN_0026ef30', 'Replay_SetState', 'Replay_SaveResumeSnapshot', 'Replay_SeekToSnapshot', 'Replay_StepFrame']
0x0026f980 Replay_EndPlayback shard=s07 size=204 race=c2 prof=0 callers=6
  cyc=[10, 5, 3, 5, 4, 5, 4, 3, 2, 4, 2, 1, 3] loops=0 ev=0 matchpc=False entries=False direct=8 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=2
  callees=['Replay_RestoreGameCameras', 'FUN_0026ef80', 'Replay_SetState', 'Replay_RestoreResumeSnapshot', 'Replay_CancelInstantReplay', 'SSXAudioSystem_GetInstance']
0x002306a8 GameModule_GetSplashManager shard=s06 size=16 race=c prof=0 callers=5
  cyc=[4] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x0030a2e8 WScriptMan_HasObjScript3 shard=s10 size=40 race=c2 prof=0 callers=4
  cyc=[10] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['FUN_003a6b78']
0x0030a728 WScriptMan_ProcessCommand shard=s10 size=316 race=c2 prof=0 callers=4
  cyc=[8, 3, 2, 2, 2, 2, 2, 4, 4, 2, 4, 4, 4, 2, 4, 4, 4, 2, 4, 4, 4, 2, 3, 3] loops=0 ev=0 matchpc=False entries=False direct=8 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=2
  callees=['WScriptProcess_Terminate', 'WScriptProcess_NextTask', 'WScriptMan_StartProcess', 'WScriptMan_StopProcess', 'WScriptProcessList_FindById']
0x0026e800 Replay_HighlightSlotClear shard=s07 size=52 race=c2 prof=0 callers=3
  cyc=[4, 7, 2] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x00270238 Replay_FreeSnapshot shard=s07 size=72 race=c2 prof=0 callers=3
  cyc=[18] loops=0 ev=0 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=3
  callees=['WorldStateStream_Reset', 'SnapshotList_PushBack', 'SnapshotList_Remove']
0x002702f8 Replay_SeekToSnapshot shard=s07 size=128 race=c2 prof=0 callers=3
  cyc=[8, 2, 2, 6, 4, 2, 2, 6] loops=1 ev=1 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=['FUN_0026d4d8']
0x00270970 Replay_OnRiderReset shard=s07 size=104 race=c2 prof=0 callers=3
  cyc=[7, 3, 3, 4, 5, 1, 3] loops=0 ev=0 matchpc=False entries=False direct=3 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=2
  callees=['Replay_CancelInstantReplay', 'Replay_FindRiderIndex', 'Replay_CancelHighlight']
0x0030a548 WScriptMan_RunObjScript3 shard=s10 size=80 race=c2 prof=0 callers=3
  cyc=[12, 3, 5] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=3
  callees=['WScriptMan_CallFunc', 'FUN_003a6b78']
0x002312d8 GameModule_IsOverlayStateDone shard=s06 size=72 race=c2 prof=0 callers=2
  cyc=[5, 3, 7, 1, 2] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=[]
0x0026ce50 PlaybackMan_ReadInputFrame shard=s07 size=408 race=c prof=0 callers=2
  cyc=[7, 6, 4, 3, 9, 7, 4, 2, 7, 7, 6, 5, 2, 1, 15, 13, 1, 3] loops=1 ev=3 matchpc=True entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=16 mmi=0 spst=1
  callees=['InputPad_Update']
```

## E8 world cache, sections, CPU riders, anim objects (module `world_cache`, 25 functions)

```text
0x0035b418 MultiSplineModifier_v11 shard=s11 size=396 race=P prof=31 callers=0
  cyc=[14, 6, 6, 2, 23, 12, 8, 7, 1, 4, 3, 2, 11] loops=1 ev=1 matchpc=False entries=False direct=1 ind=6 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=9
  callees=['FUN_00345430']
0x0034dc90 AnimObject_v51 shard=s11 size=132 race=P prof=10 callers=0
  cyc=[33] loops=0 ev=0 matchpc=False entries=False direct=0 ind=3 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=4
  callees=[]
0x0035abf0 MultiSplineModifier_v04 shard=s11 size=48 race=P prof=8 callers=0
  cyc=[12] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=[]
0x0010a768 ComputerActorRider_GetControlInput shard=s00 size=300 race=P prof=7 callers=0
  cyc=[22, 6, 5, 5, 5, 5, 5, 5, 5, 3, 1, 2, 6] loops=1 ev=1 matchpc=False entries=True direct=11 ind=0 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=4
  callees=['ComputerActorRider_ThinkCruise', 'ComputerActorRider_ThinkPrewind', 'ComputerActorRider_ThinkBoardPress', 'ComputerActorRider_ThinkRailslide', 'ComputerActorRider_ThinkSpin', 'ComputerActorRider_ThinkNaturalAir']
0x0034e348 AnimObject_v52 shard=s11 size=140 race=P prof=5 callers=3
  cyc=[9, 3, 2, 13, 1, 7] loops=1 ev=1 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=1 mmi=0 spst=4
  callees=[]
0x003a9d60 cWorldCache_StreamerUpdateView shard=s13 size=236 race=P prof=4 callers=3
  cyc=[16, 8, 2, 5, 7, 7, 5, 9] loops=1 ev=1 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=3 mmi=0 spst=7
  callees=['cWorldCache_StreamerUpdateViewNode']
0x0035a5d8 MultiSplineModifier_v03 shard=s11 size=28 race=P prof=4 callers=0
  cyc=[7] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['MultiSplineModifier_UpdateNodes']
0x00361c20 MultiSplineModifier_v18 shard=s12 size=76 race=P prof=4 callers=0
  cyc=[7, 6, 6] loops=0 ev=0 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=2
  callees=[]
0x003aa028 cWorldCache_StreamerUpdateViewNode shard=s13 size=688 race=P prof=3 callers=3
  cyc=[25, 5, 5, 5, 5, 6, 2, 3, 14, 7, 2, 5, 8, 6, 2, 3, 8, 5, 3, 16, 3, 3, 16, 3, 1, 11] loops=0 ev=0 matchpc=False entries=False direct=6 ind=0 sysc=0 vu=10 vuacc=4 fpu=52 mmi=0 spst=10
  callees=['cWorldCache_RequestHull', 'cWorldCache_IsHullResident', 'cWorldCache_PointBoxDistance', 'cWorldCache_StreamerUpdateViewNode']
0x00341d48 AnimObject_v15 shard=s11 size=256 race=P prof=3 callers=2
  cyc=[9, 3, 3, 2, 2, 5, 3, 4, 4, 2, 4, 2, 1, 7, 8, 1, 4] loops=0 ev=0 matchpc=False entries=False direct=4 ind=1 sysc=0 vu=0 vuacc=0 fpu=7 mmi=0 spst=2
  callees=['AnimObject_StepLoop', 'AnimObject_StepPingPong', 'AnimObject_StepClamp', 'FUN_0034eba0']
0x00312490 AnimController_DispatchEndEvents shard=s10 size=260 race=P prof=2 callers=2
  cyc=[12, 9, 3, 4, 7, 2, 4, 2, 3, 10, 9] loops=3 ev=3 matchpc=False entries=False direct=1 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=7
  callees=['FUN_001446a0', 'AnimSeqList_GetAt']
0x003a96e0 cWorldCache_StreamerGetViewClearance shard=s13 size=144 race=P prof=2 callers=1
  cyc=[12, 2, 14, 1, 1, 6] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=6 mmi=0 spst=4
  callees=['cWorldCache_StreamerUpdateView']
0x0035a918 MultiSplineModifier_v24 shard=s11 size=440 race=P prof=2 callers=0
  cyc=[11, 6, 4, 1, 80, 1, 7] loops=1 ev=1 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=2 vuacc=0 fpu=16 mmi=0 spst=18
  callees=['World_GetNash', 'SpatialHash_Update']
0x0010b980 ComputerActorRider_FindJumpMarker shard=s00 size=408 race=P prof=1 callers=5
  cyc=[35, 5, 3, 24, 1, 17, 4, 1, 12] loops=1 ev=1 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=13 mmi=0 spst=11
  callees=['Path_GetPointAtDistance']
0x0010bb18 ComputerActorRider_FindSpeedMarker shard=s00 size=224 race=P prof=1 callers=2
  cyc=[25, 3, 3, 11, 1, 3, 3, 1, 6] loops=1 ev=1 matchpc=False entries=False direct=0 ind=1 sysc=0 vu=0 vuacc=0 fpu=12 mmi=0 spst=4
  callees=[]
0x0010d1a0 ComputerActorRider_ShouldResetPath shard=s00 size=624 race=P prof=1 callers=2
  cyc=[20, 4, 4, 3, 7, 3, 3, 2, 1, 12, 3, 5, 6, 4, 1, 8, 3, 5, 3, 1, 4, 8, 1, 5, 9, 2, 2, 2, 4, 1, 5, 2, 2, 2, 1, 8] loops=0 ev=0 matchpc=False entries=False direct=5 ind=0 sysc=0 vu=0 vuacc=0 fpu=46 mmi=0 spst=9
  callees=['Rider_GetMainState', 'Rider_GetSubState', 'Math_Atan']
0x00342150 AnimObject_Save shard=s11 size=80 race=P prof=1 callers=2
  cyc=[20] loops=0 ev=0 matchpc=False entries=False direct=1 ind=1 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=3
  callees=['AnimObject_BaseSave']
0x0034e3d8 AnimObject_BaseSave shard=s11 size=108 race=P prof=1 callers=2
  cyc=[27] loops=0 ev=0 matchpc=False entries=False direct=1 ind=2 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=3
  callees=['ModObj_Save']
0x00311b20 AnimController_GetLayerSequence shard=s10 size=40 race=c prof=0 callers=81
  cyc=[10] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['AnimSeqList_GetAt']
0x00311ae8 AnimController_GetLayerAnimEntry shard=s10 size=56 race=c prof=0 callers=76
  cyc=[8, 3, 3] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['AnimTable_GetEntry']
0x00311e88 AnimController_FadeOutLayer shard=s10 size=116 race=c prof=0 callers=14
  cyc=[29] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=4 mmi=0 spst=5
  callees=['AnimSeqList_FadeOutAll', 'AnimSeqList_RemoveEventTrigger']
0x00312ae8 AnimController_IsLayerAnimFinished shard=s10 size=44 race=c prof=0 callers=14
  cyc=[11] loops=0 ev=0 matchpc=False entries=False direct=1 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=1
  callees=['AnimSeqList_GetAt']
0x00311b48 AnimController_RotateRoots shard=s10 size=168 race=c prof=0 callers=11
  cyc=[42] loops=0 ev=0 matchpc=False entries=False direct=2 ind=0 sysc=0 vu=0 vuacc=0 fpu=15 mmi=0 spst=10
  callees=['AnimController_TransformRoots', 'Math_SinCos']
0x003a9800 cWorldCache_StreamerGetSectionCount shard=s13 size=28 race=c prof=0 callers=10
  cyc=[3, 2, 2] loops=0 ev=0 matchpc=False entries=False direct=0 ind=0 sysc=0 vu=0 vuacc=0 fpu=0 mmi=0 spst=0
  callees=[]
0x003135b0 AnimSequence_AdvanceTrack shard=s10 size=592 race=c prof=0 callers=8
  cyc=[24, 4, 4, 6, 4, 2, 4, 3, 2, 2, 2, 2, 4, 3, 6, 3, 7, 5, 3, 2, 3, 4, 2, 5, 2, 4, 1, 4, 5, 5, 2, 4, 1, 14] loops=0 ev=0 matchpc=False entries=False direct=9 ind=0 sysc=0 vu=0 vuacc=0 fpu=55 mmi=0 spst=8
  callees=['AnimSequence_FireTriggersInRange', 'AnimSequence_FireTriggersAtTime', 'AnimSequence_FireEndTriggers']
```
