# 300 frames, 2180 functions called, 16853 calls/frame

## Top 60 functions by calls per frame

| calls/frame | callers | function | group |
|---:|---:|---|---|
| 1024.7 | 1 | `SND_UtkSkipBits` 0x3cd260 | sound |
| 487.8 | 5 | `ProcessList_Next` 0x354ef8 | other |
| 348.6 | 3 | `DeferMan_AppendPacket` 0x362978 | render |
| 304.5 | 3 | `DeferMan_FindBucket` 0x395000 | render |
| 234.0 | 2 | `Nash_CullNode` 0x22a830 | world |
| 232.1 | 9 | `PS2GraphicsMan_ClipTestBox` 0x37de88 | render |
| 206.5 | 2 | `DeferMan_InternState` 0x394ed0 | render |
| 176.4 | 9 | `PS2GraphicsMan_GetWorldToClipMatrix` 0x3953b8 | render |
| 176.4 | 1 | `PS2GraphicsMan_EnsureViewMatrices` 0x3956b0 | render |
| 174.7 | 1 | `TexMan_EmitTex0` 0x368970 | render |
| 173.0 | 1 | `RiderModel_BuildBoneMatrix` 0x310120 | riders |
| 168.3 | 3 | `strcmp` 0x4165a8 | libc |
| 159.0 | 32 | `Rider_GetMainState` 0x11fe98 | riders |
| 156.0 | 1 | `Nash_ClassifyPatch` 0x22a128 | world |
| 154.8 | 4 | `GetThreadId` 0x423c90 | other |
| 154.3 | 1 | `TexMan_BindTextures` 0x368660 | render |
| 153.1 | 3 | `SND_UtkReadBits` 0x3cd1f8 | sound |
| 147.1 | 1 | `PS2GraphicsMan_EmitModelHeader` 0x37e120 | render |
| 136.2 | 2 | `WorldObj_v15` 0x360910 | world |
| 126.1 | 1 | `Nash_ClassifyInstance` 0x229fc8 | world |
| 117.7 | 24 | `Math_SinCos` 0x31be50 | math |
| 110.3 | 3 | `DeferMan_ComputeSortKey` 0x364240 | render |
| 103.5 | 2 | `Dma_FromScratchpad` 0x371dd8 | render |
| 102.4 | 22 | `Math_Atan` 0x31c228 | math |
| 102.3 | 9 | `AnimSeqList_GetAt` 0x314760 | riders |
| 97.9 | 11 | `SprStream_BeginBlock` 0x38f460 | render |
| 95.0 | 1 | `SplineModifier_IsNodeVisible` 0x345638 | other |
| 92.5 | 9 | `SprStream_EndBlockAligned` 0x38f668 | render |
| 92.5 | 1 | `SprStream_EndBlock` 0x38f598 | render |
| 91.7 | 2 | `VramHeap_UnlinkSizeList` 0x366238 | other |
| 91.7 | 2 | `VramHeap_LinkFreeBlock` 0x3660a8 | other |
| 85.0 | 2 | `GsPkt_Finish` 0x3627a8 | other |
| 83.7 | 1 | `WorldObj_Update` 0x3608e8 | world |
| 83.7 | 1 | `GameProcess_PostUpdate` 0x360728 | other |
| 82.6 | 10 | `WaitSema` 0x423de0 | kernel |
| 82.2 | 3 | `BankMonitorInstance_PollFree` 0x2abb80 | sound |
| 82.0 | 1 | `DeferMan_EmitStateChange` 0x363c20 | render |
| 80.6 | 1 | `VramHeap_FindFreeBlock` 0x3667f8 | other |
| 80.1 | 14 | `AI_GetFrameCounter` 0x1298c8 | riders |
| 80.0 | 1 | `BankMonitor_TimerUpdate` 0x2ab828 | sound |
| 78.1 | 1 | `TypedProcess_Render` 0x360790 | other |
| 77.9 | 8 | `SignalSema` 0x423dc0 | kernel |
| 74.0 | 23 | `BE_GetRiderCharacter` 0x14a080 | game |
| 71.5 | 1 | `Vu0_RandNext` 0x36d400 | render |
| 71.1 | 18 | `Rider_GetSubState` 0x11fee8 | riders |
| 70.3 | 1 | `SND_MutexLock` 0x3c3380 | sound |
| 70.3 | 1 | `SND_MutexUnlock` 0x3c33e0 | sound |
| 70.3 | 2 | `SND_DefaultLock` 0x3b5860 | sound |
| 70.3 | 2 | `SND_DefaultUnlock` 0x3b5880 | sound |
| 69.7 | 2 | `ModObj_v20` 0x356bf0 | other |
| 69.7 | 1 | `ModObj_Render` 0x356298 | other |
| 69.6 | 2 | `ModObj_Update` 0x356198 | other |
| 69.6 | 1 | `ModObj_UpdateModifiers` 0x3556f8 | other |
| 69.6 | 2 | `ModObj_v50` 0x3568b0 | other |
| 69.6 | 1 | `ModObj_PostUpdateModifiers` 0x355748 | other |
| 69.6 | 1 | `ModObj_v03` 0x360da0 | other |
| 69.3 | 41 | `BELibrary_GetInterface` 0x14dd58 | game |
| 69.3 | 1 | `BERace_GetInterface` 0x1530e0 | game |
| 68.7 | 21 | `SND_Lock` 0x3b58a0 | sound |
| 68.7 | 21 | `SND_Unlock` 0x3b58d8 | sound |

## Call trees (calls per frame)

### `App_RunModuleLoop` 0x316f00 (0.00/frame)

```
     4.0  App_InvokeExecCallback
       4.0  PS2ExecMan_InvokeCallback
     3.0  SSXApp_NextTick
       3.0  PadQueue_AdvanceWrite
       2.0  InputPad_Update
       2.0  PadQueue_GetPadStatus
       2.0  PadQueue_GetPadData
       1.0  cUsbKeyboard_Update
     2.0  PS2ExecMan_v11
     2.0  PS2ExecMan_v12
     2.0  EATimer_ServiceCallbacks
       1.0  ssx_CdTrayCallback
       1.0  SND_TimerServiceCallback
         1.0  SND_RunServiceCallbacks
           1.0  SNDSTRM_Service
       1.0  EAFile_HostReadService
     1.0  GameModule_Render
       2.0  PS2GraphicsMan_SetPerspective
         6.0  PS2GraphicsMan_GetScreenWidth
         6.0  PS2GraphicsMan_GetScreenHeight
         4.0  tanf
           4.0  __kernel_tanf
         2.0  Math_SinCos
         2.0  PS2GraphicsMan_SetScissorClamped
          11.4  PS2GraphicsMan_GetScreenWidth
          11.4  PS2GraphicsMan_GetScreenHeight
       2.0  PS2GraphicsMan_AddView
       2.0  PS2GraphicsMan_LoadMatrix
       1.0  AI_Render
         6.0  Rider_UpdateVisibility
           6.0  PS2GraphicsMan_ClipTestBox
           6.0  PS2GraphicsMan_GetWorldToClipMatrix
         6.0  Rider_RenderAttachedFX
           6.0  AttachedEmitterFX_Render
         6.0  cBoardWakeFX_Render
         6.0  cPlayerPulseFX_Render
           6.0  Rider_IsHuman
           5.0  BELibrary_GetInstance
           5.0  BELibrary_GetInterface
         6.0  cSpeedBoostStreamerFX_Render
         6.0  RiderBase_RenderPowerUpTrail
         6.0  cAirStreamerFX_Render
           2.3  PS2GraphicsMan_v71
         6.0  RiderBase_RenderCheatCode
         2.9  Rider_RenderModel
           5.8  PS2GraphicsMan_v27
           2.9  Rider_ComputeLOD
           2.9  RiderModel_Render
           2.9  PS2GraphicsMan_SetLighting
         2.9  Rider_PrepareLighting
           2.9  Rider_BuildLightSet
         2.9  Rider_RenderShadow
           2.9  Rider_DrawShadow
         2.9  cAngerFX_Render
           2.9  Rider_IsHuman
         2.9  cBoardSparkFX_Render
         2.9  cDynamicSprayFX_Render
           5.3  mEmitter_Render
         2.9  RiderBase_RenderAttackFX
         1.0  cBoardTrailFX_RenderBatch
           1.0  PS2GraphicsMan_v89
       1.0  LoadIndicator_Draw
       1.0  CrowdMan2d_RenderSprites
       1.0  Nash_GatherVisible
         4.0  Nash_CullNode
         230.0  Nash_CullNode ...
         1.0  Vu0_UploadMatrixPair
         1.0  Nash_EmptyHook
         1.0  Nash_ClassifyInstances
         126.1  Nash_ClassifyInstance
         1.0  Nash_ClassifyPatches
         156.0  Nash_ClassifyPatch
         1.0  Nash_ClassifyEffects
         1.0  Nash_CollectInstances
         1.0  Nash_CollectPatches
         1.0  Nash_CollectEffects
         1.0  OcclusionMan_BuildVisible
           3.8  PS2GraphicsMan_ClipTestBox
           2.0  PS2GraphicsMan_GetMatrix
           2.0  Vu0_UploadOccluder
           2.0  OcclusionMan_BuildOccluderPlanes
           1.0  PS2GraphicsMan_GetWorldToClipMatrix
           1.0  qsort
         1.0  PS2GraphicsMan_GetWorldToClipMatrix
         176.4  PS2GraphicsMan_EnsureViewMatrices
       1.0  GameModule_RenderModels
        42.0  GraphicsMan_DrawModelSimple
          42.0  PS2GraphicsMan_v95
       1.0  GameModule_RenderBezierPatches
        42.0  GameModule_DrawBezierPatch
          42.0  PS2BezierMan_v21
         1.0  GameModule_BeginBezierPass
           2.0  PS2GraphicsMan_GetMatrix
           1.0  PS2BezierMan_BeginPass
           1.0  PS2BezierMan_GetLodNearDist
           1.0  PS2BezierMan_GetLodFarDist
         1.0  GameModule_EndBezierPass
           1.0  PS2BezierMan_v18
         1.0  BezierManBase_v05
       1.0  GameModule_RenderFogParticles
         1.0  cPS2FogParticleMan_Render
       1.0  GameModule_RenderLensFlares
         1.0  LensFX_ClearSources
         1.0  LensFX_Render
           1.0  LensFX_PrepareView
           1.0  LensFX_DrawFlares
       1.0  GameViewMan_PrefetchViews
         1.0  GameCamera_UpdateStreamView
           1.0  cWorldCache_StreamerSetView
           1.0  cWorldCache_StreamerGetViewClearance
       1.0  GameViewMan_AllViewsReady
       1.0  GameModule_RenderProcessLists
         6.0  ProcessList_RenderPriority
         163.3  ProcessList_Next
          78.1  TypedProcess_Render
          69.7  ModObj_Render
           6.0  ProcessList_First
           5.5  Flag_Render
           2.0  cTransitionMan_Render
           2.0  cSplashManager_Render
           1.0  VfxProcess_Render
           1.0  AIVfxProcess_Render
           1.0  StatsProcess_Render
           1.0  SnowFall_Render
           1.0  cFlagManager_Render
           1.0  GameProcess_Render
       1.0  AppModule_RenderFade
       1.0  GenericStreamPause_v02
       1.0  ScriptEngine_ToggleMoviePause
         1.0  ScriptMovie_TogglePause
       1.0  SSXAudioSystem_Update
         1.0  SSXAudioSystem_UpdateFade
         1.0  SSXAudioSystem_UpdateMixRamp
         1.0  SSXAudioSystem_UpdateSpeechDucking
         1.0  SSXAudioSystem_GetApp
         1.0  SSXAudioSystem_GetGameModule
         1.0  SSXAudioSystem_UpdateMusicIntensity
           1.2  SSXAudioSystem_GetChannelGainPtr
           1.0  Rider_GetMainState
           1.0  SSXAudioSystem_FindRider
           1.0  SSXAudioSystem_IsReplayPlaying
         1.0  SSXAudioSystem_UpdatePeakMusic
         1.0  SSXAudioSystem_UpdateMusicFadeIn
           1.0  SSXAudioSystem_FindRider
           1.0  SSXAudioSystem_GetRiderRatingTier
           1.0  MusicPlayer_PathCmdA
           1.0  MusicPlayer_GetSongTag
         1.0  SSXAudioSystem_UpdateAirborneRiders
         1.0  SSXAudioSystem_UpdateTrackedSounds
         1.0  SSXAudioSystem_UpdateObjectLoops
           2.8  ModObj_v24
         1.0  SSXAudioSystem_UpdateBoostMeterSounds
           2.0  SSXAudioSystem_GetRaceMan
           1.0  BELibrary_GetInstance
           1.0  BELibrary_GetInterface
           1.0  SSXAudioSystem_IsReplayPlaying
         1.0  SSXAudioSystem_UpdateUberSpeech
           1.0  SSXAudioSystem_FindRider
           1.0  SSXAudioSystem_IsReplayPlaying
         1.0  SSXAudioSystem_UpdateCountdownBeeps
           1.0  SSXAudioSystem_IsReplayPlaying
         1.0  SSXAudioSystem_UpdateRaceLoopSfx
           1.0  SSXAudioSystem_GetRaceMan
         1.0  SSXAudioSystem_UpdateLoopSfx
           1.0  SND_Lock
           1.0  SND_Unlock
           1.0  SND_IsHandleDone
         1.0  SSXAudioSystem_UpdateBanksAndStreams
           1.0  SndLoadQueue_Update
           1.0  CrowdAudio_Update
           1.0  StreamArray_UpdateAll
           1.0  SSXAudioSystem_UpdateBankMonitor
           1.0  SpeechSystem_Update
           1.0  MusicPlayer_Update
         1.0  VfxScreenTint_PollFlashSound
       1.0  WorldTriggerManager_Update
         1.0  SSXAudioSystem_GetInstance
       1.0  MenuSys_IsActive
       1.0  WScriptMan_Render
       1.0  Mat4_Inverse
         1.0  Mat4_Determinant
           4.0  Math_Det3x3
         1.0  Mat4_Adjoint
          16.0  Math_Det3x3
       1.0  CrowdMan3d_Render
       1.0  SkyBox_Render
         1.0  World_GetCameraEye
         1.0  World_GetCurrentViewIndex
         1.0  PS2GraphicsMan_v95
         147.1  PS2GraphicsMan_EmitModelHeader
         122.0  DeferMan_AppendPacket
         122.0  DeferMan_FindBucket
          47.5  PS2GraphicsMan_GetMatrix
          18.0  PS2GraphicsMan_ClipTestBox
          10.6  DeferMan_ComputeSortKey
           1.0  PS2GraphicsMan_UpdateViewMatrices
       1.0  MultiParticleMan_EmitAll
         1.0  MultiParticle_EmitAtInstances
       1.0  DeferredDelete_AdvanceFrame
       1.0  PS2GraphicsMan_SetViewport
         5.0  DeferMan_SetScissor
         5.0  PS2GraphicsMan_SetScissorClamped ...
         5.0  PS2GraphicsMan_GetScreenHeight
       1.0  PS2GraphicsMan_SetOrtho
         8.8  PS2GraphicsMan_GetScreenWidth
         8.8  PS2GraphicsMan_GetScreenHeight
         4.4  PS2GraphicsMan_SetScissorClamped ...
       1.0  PS2GraphicsMan_EndFrame
         2.0  PS2GraphicsMan_GetScreenWidth
         2.0  PS2GraphicsMan_GetScreenHeight
         2.0  PS2GraphicsMan_SetViewportRect
           3.0  PS2GraphicsMan_SetViewport ...
         2.0  SignalSema
         2.0  InvalidDCache
           6.0  _InvalidDCache
           6.0  DIntr
         2.0  EIntr
         1.0  IntcTable_EndFrameStub
         1.0  DeferMan_ResetFrame
           2.0  SprStream_Download
           2.0  InvalidDCache ...
           2.0  EIntr
           1.0  EA_MemSet
         1.0  PS2GraphicsMan_RenderFrame
         154.3  TexMan_BindTextures
          82.0  DeferMan_EmitStateChange
           3.0  DeferMan_EmitDefaultState
           2.0  SprStream_SetMode
           1.0  DeferMan_BuildBuckets
           1.0  DeferMan_SortBuckets
           1.0  VramHeap_NodesToScratchpad
           1.0  VramHeap_NodesFromScratchpad
           1.0  TexMan_ResetUsageTable
           1.0  PS2GraphicsMan_RenderDepthFog
           1.0  PS2GraphicsMan_RenderBloom
           1.0  PS2GraphicsMan_RenderColorFilter
           1.0  TexMan_InvalidateBound
         1.0  TexMan_EndFrameStub
         1.0  PS2GraphicsMan_DrawFill
         1.0  WaitSema
       1.0  PS2GraphicsMan_GetScreenWidth
       1.0  PS2GraphicsMan_GetScreenHeight
       1.0  PS2GraphicsMan_SetViewportRect ...
       1.0  PS2GraphicsMan_GetWorldToClipMatrix ...
       1.0  PS2GraphicsMan_ClearLights
       1.0  PS2GraphicsMan_TryAcquireFrame
         1.0  PS2GraphicsMan_AcquireFrameBuffer
       1.0  UIEngine_Render
         1.0  PS2GraphicsMan_PushView
           3.4  PS2GraphicsMan_CopyViewState
         1.0  PS2GraphicsMan_PopView
         1.0  PS2GraphicsMan_SetOrtho ...
         1.0  PS2GraphicsMan_PushMatrix
         1.0  PS2GraphicsMan_PopMatrix
         1.0  PS2GraphicsMan_LoadMatrix
         1.0  UIStateMgr_Render
           2.0  UIList_First
           1.0  cOVStateHUD1P_Render
           1.0  UIListNode_IsSentinel
     1.0  GameModule_Update
       6.0  ProcessList_UpdatePriority
       163.4  ProcessList_Next
        83.7  WorldObj_Update
          83.7  WorldObj_v15
        61.1  ModObj_Update
          69.6  ModObj_UpdateModifiers
          69.6  ModObj_v20
          69.6  ModObj_PostUpdateModifiers
          58.1  ModifierBlock_IsTransformFinished
          52.4  WorldObj_v15
          16.0  AnimObject_v15
           2.0  ModObj_v23
           1.0  AnimTeeter_v15
        12.0  ProcessList_First
         8.5  Particle_Update
           8.5  ModObj_Update ...
         2.0  cTransitionMan_Update
         2.0  cSplashManager_Update
           1.0  cSplashManager_EmitPending
           1.0  Rand_NextMain
         1.0  VfxProcess_Update
           1.0  PS2VisualEffectsMan_Update
         1.0  AIVfxProcess_Update
           1.0  PS2AIVisualEffectsMan_Update
         1.0  StatsProcess_Update
           1.0  StatsProcess_UpdateStub
         1.0  SnowFall_Update
           6.0  SnowBox_Update
           1.0  GameModule_GetSplashManager
           1.0  WorldPainter_GetWeather08
           1.0  WorldPainter_GetWeather10
           1.0  WorldPainter_GetWeather18
           1.0  WorldPainter_GetWeather20
           1.0  WorldPainter_GetWeather38
           1.0  WorldPainter_GetWeather48
           1.0  WorldPainter_GetWeather50
           1.0  WorldPainter_GetWeather40
           1.0  WorldPainter_GetWeather60
           1.0  WorldPainter_GetWeather68
           1.0  WorldPainter_GetWeather70
           1.0  WorldPainter_GetWeather78
           1.0  WorldPainter_GetWeather80
           1.0  WorldPainter_GetWeather88
           1.0  WorldPainter_GetWeather90
           1.0  WorldPainter_GetWeather98
           1.0  cSplashManager_SetSnowAmount
         1.0  cCutSceneTransitionMan_Update
         1.0  cFlagManager_Update
           2.0  cFlagManager_GroupUpdate
           1.0  World_GetPeakNumber
       5.0  Rider_UpdateRumble
       3.0  MenuSys_IsActive
       2.0  Replay_IsPaused
       1.0  Player_UpdateRumble
         1.0  Player_ApplyRumble
           2.0  PS2Pad_SetActuator
           1.0  PS2Pad_GetNumActuators
         1.0  Rider_GetIndex
         1.0  BENewPlayerInt_GetVibration
           1.0  BENewPlayerInt_GetSlot
         1.0  BELibrary_GetInstance
         1.0  BELibrary_GetInterface
          69.3  BERace_GetInterface
       1.0  AI_Update
         6.0  Rider_UpdateAttachedFX
           6.0  cBoltOnFXInterface_Update
           6.0  AttachedEmitterFX_Update
         6.0  Rider_BeginFrame
           6.0  Rider_UpdateTargetSpeed
           6.0  AI_GetFrameCounter
           4.0  Rider_IsControlMode3
           2.0  Rider_FindNearbyLights
           2.0  OverlapList_GatherAll
         6.0  Rider_ProcessControlInput
           6.0  RiderBase_HandleSubStateInput
           5.0  ComputerActorRider_GetControlInput
           1.0  Player_GetControlInput
         6.0  Rider_UpdateImpacts
          10.9  Rider_GetMainState
           6.0  Rider_UpdateBoostTimers
           6.0  Rider_UpdateLegIKBlend
         6.0  Rider_UpdateSlewedParams
           6.0  Rider_UpdateSpeedAnims
         6.0  Rider_UpdateMainState
           6.0  RiderBase_UpdateMainState
         6.0  Rider_UpdateAnimation
           6.0  Rider_TickAnimation
         6.0  Rider_UpdatePose
           6.0  Rider_UpdateSkeleton
         6.0  Rider_PostUpdateMainState
           6.0  RiderBase_PostUpdateMainState
         6.0  Rider_UpdateModel
           6.0  RiderModel_BuildBoneMatrices
         6.0  Rider_UpdateProgress
           6.0  Rider_UpdateRaceProgress
           6.0  Rider_UpdateAIPathProgress
           6.0  RiderMetrix_Update
           6.0  Rider_UpdateFocusBox
           6.0  Rider_CheckRaceFinish
           6.0  Rider_EmptyProgressHook
         6.0  Rider_UpdateSurfaceFX
           6.0  Rider_GetIndex
           6.0  SectionMan_FindLogicalSection
           6.0  WorldPainter_UpdateView
           6.0  FXTrail_Reset
         6.0  Rider_UpdateHeadingAndAudio
           6.0  Rider_UpdateDangerState
           6.0  Rider_IsOnGround
           6.0  SSXAudioSystem_UpdateRiderWorldPaint
           6.0  SSXAudioSystem_GetInstance
           4.8  Math_Atan
         6.0  cAngerFX_Update
          10.0  Rider_GetIndex
           6.0  Rider_IsHuman
           5.0  BEAggressionInt_GetInstance
           5.0  BEAggressionInt_GetRivalLevel
         6.0  cBoardSparkFX_Update
           6.0  Rider_GetMainState
           6.0  Emitter_UpdateDescriptor
           6.0  DynamicEmitter_SpawnAt
         6.0  cBoardWakeFX_Update
           6.2  Rider_GetMainState
         6.0  cDynamicSprayFX_Update
           6.0  Rider_GetMainState
           6.0  Rider_GetVelocity
           6.0  Rider_GetIndex
           6.0  cDynamicSprayFX_UpdateEmitter5
           5.3  Rider_GetSubState
           4.9  cDynamicSprayFX_UpdateEmitter0
           1.0  cDynamicSprayFX_UpdateEmitter3
           1.0  cDynamicSprayFX_UpdateEmitter1
           1.0  cDynamicSprayFX_UpdateEmitter4
           1.0  cDynamicSprayFX_UpdateEmitter7
           1.0  cDynamicSprayFX_UpdateEmitter8
           1.0  cDynamicSprayFX_UpdateEmitter9
         6.0  cPlayerPulseFX_Update
           6.0  Rider_IsHuman
           1.0  Math_Cos
         6.0  cSpeedBoostStreamerFX_Update
           6.0  Rider_GetMainState
           6.0  cSpeedBoostStreamerFX_UpdatePoints
         6.0  cBoardTrailFX_Update
           9.6  Rider_GetMainState
           6.0  cBoardTrailFX_IsActive
           4.8  Rider_GetIndex
           4.8  cBoardTrailFX_ShouldEmit
           4.8  Math_Atan
         6.0  RiderBase_UpdatePowerUpTrail
         6.0  cAirStreamerFX_Update
           6.0  Rider_GetMainState
           1.1  AI_GetFrameCounter
         6.0  RiderBase_UpdateAttackFX
         6.0  RiderBase_UpdateCheatCode
           6.0  Rider_IsComputer
           1.0  BELibrary_GetInterface ...
           1.0  RiderBase_MatchCheatCode
         1.0  DangerMan_Update
         1.0  FocusMan_Update
           1.1  AI_GetFrameCounter
         1.0  AI_UpdateRiderRelations
           5.0  Rider_GetPosition
           2.5  BEAggressionInt_GetInstance
           2.5  BEAggressionInt_GetRivalLevel
           2.5  Math_Atan
         1.0  AI_UpdateRaceState
           1.0  RaceStateRace_Update
         1.0  AI_UpdateRaceTimeLimit
           1.0  BELibrary_GetInterface ...
       1.0  FEAsyncManager_Update
        15.0  FEAsyncManager_GetState
         5.0  PS2GraphicsMan_GetFlipCount
       1.0  MessageCenter_Update
       1.0  LoadIndicator_Update
       1.0  GameUI_IsPauseMenuOpen
         1.0  StrHash
           1.0  StrHashElf
         1.0  UIStateMgr_FindState
           2.0  UIListNode_IsSentinel
           2.0  UIList_First
       1.0  GameUI_Update
         1.0  UIStateMgr_GetActiveState
           1.0  UIList_First
       1.0  CrowdMan2d_Update
         1.0  CrowdMan2d_SpawnSprites
           1.0  CrowdAudio_GetInstanceReaction
         1.0  cCrowdAnim2D_Update
       1.0  SectionMan_Update
         1.0  SectionMan_NextQueued
       1.0  GameViewMan_UpdateCameras
         1.0  GameCamera_Update
           1.0  GameCamera_ComputeView
           1.0  ChaseCamController_Update
       1.0  GameViewMan_AllViewsReady
       1.0  GameModule_ProcessStateChange
         1.0  BELibrary_GetInterface ...
         1.0  GameState_Update
       1.0  GameModule_CheckPauseRequest
         1.0  BENewPlayerInt_GetPadIndex
           1.0  BENewPlayerInt_GetSlot
         1.0  BELibrary_GetInstance
         1.0  BELibrary_GetInterface ...
         1.0  InputBinding_EvalBool
           4.0  InputMap_EvalBool
       1.0  AppModule_UpdateFade
       1.0  PlaybackMan_PollFileLoad
       1.0  PlaybackMan_ReadInputFrame
       1.0  Replay_Update
         1.0  Replay_SetState
       1.0  ScriptEngine_Update
         4.0  ScriptList_Update
         2.0  ScriptViewObj_Update
         1.0  ScriptAudioObj_UpdateSounds
           1.0  SSXAudioSystem_GetInstance
         1.0  ScriptEngine_UpdateProcesses
           1.0  InSeq_UpdateAll
         1.0  ScriptMovie_Update
       1.0  ScriptEngine_PollAsyncLoads
       1.0  cWorldSound_Update
         2.0  cWorldSound_ProcessEmitter
         1.0  cWorldSound_RefreshNearbyEmitters
           4.0  cWorldSound_QueryTreeNode
           1.0  SSXAudioSystem_GetGameModule
       1.0  WorldPainter_GetSkyBox08
         1.0  tWPIGD_SkyBox_Get08
       1.0  WScriptMan_Update
         1.0  WScriptMan_ProcessPendingRequest
         1.0  WScriptProcessList_PopFront
       1.0  CrowdMan3d_Update
       1.0  ProcessList_MergePending
        13.8  SyncProcess_IdLessEqual
          13.8  SyncProcess_GetId
        13.7  ProcessList_Prev
         1.0  ProcessList_PendingFirst
       1.0  ProcessList_PostUpdatePriority
       153.4  ProcessList_Next
        83.7  GameProcess_PostUpdate
        69.6  ModObj_v03
          69.6  ModObj_v50
         1.0  ProcessList_First
       1.0  MultiParticleMan_Update
         1.0  MultiParticle_Update
           1.0  Emitter_Update
       1.0  UIEngine_Update
         1.0  FEUIInput_PollUsbKeyboard
           1.0  cUsbKeyboard_GetKey
         1.0  UIStateMgr_Update
           1.0  cOVStateHUD1P_Update
           1.0  UIListNode_IsSentinel
           1.0  UIList_First
           1.0  UIState_UpdateScreens
           1.0  UIStateMgr_ProcessStates
       1.0  World_Update
         1.0  cWorldCache_Update
           1.0  cStreamMan_Update
           1.0  cWorldCache_UpdateHullLRU
           1.0  cWorldCache_StreamerUpdate
     1.0  PS2ExecMan_WaitFrame
       1.0  PS2ExecMan_v11
       1.0  PS2ExecMan_v12
       1.0  WaitSema
       1.0  PollSema
```

### `SND_EeThread` 0x3c19a8 (0.00/frame)

```
     1.6  SND_RunChangeCallbacks
     1.6  SND_Service
       1.7  SND_Tick
        17.1  SND_ChannelCalcVolume
        17.1  SND_ChannelApplyVolume
        14.3  SND_ChannelCalcPitch
          14.4  SND_CentsToPitch
        14.3  SND_ChannelApplyPitch
         1.7  BankMonitor_SndTickCallback
           1.7  SSXAudioSystem_UpdateTimers
         1.7  SNDMIDI_Service
         1.7  SND_TickStub
         1.7  PATH_UpdateTick42
           1.7  PATH_Update
     1.6  SND_DefaultLock
      70.3  SND_MutexLock
        70.3  GetThreadId
        66.9  WaitSema
     1.6  SND_DefaultUnlock
      70.3  SND_MutexUnlock
        70.3  GetThreadId
        66.9  SignalSema
     1.6  SND_SaveVu0Context
     1.6  SND_RestoreVu0Context
       1.6  SND_RestoreVu0Flags
     1.6  SND_ReleaseFinishedVoices
     1.6  SND_MemCopy
     1.6  SNDSTRM_RunCallbacks
       2.5  SNDSTRM_ConsumeCallback
     1.6  SND_EeMix
       1.6  SND_EeMixBlock
        22.9  SND_EeMixScaledVu0
        15.1  SND_DspResample_Process
           9.6  SND_DecUtk_DecodeLoop
           5.3  SND_DecEaxa_DecodeStream
         3.1  SND_DspClip16_Process
         3.1  memset
     1.6  WaitSema
     1.6  FlushCache
     1.6  sceSifSetDma
```

### `Renderer_DmaKickThread` 0x382760 (0.00/frame)

```
     6.0  WaitSema
     3.0  SignalSema
     1.0  LensFX_UpdateCallback
       1.0  LensFX_UpdateSourceVisibility
     1.0  Renderer_SetDisplayRegs
       2.0  __fixunssfdi
         4.0  __cmpdf2
           8.0  __unpack_d
           4.0  __fpcmp_parts_d
         4.0  __fixunsdfsi
           4.0  __unpack_d
         2.0  __floatdidf
           4.0  __muldf3
           4.0  __floatsidf
           2.0  __adddf3
         2.0  __subdf3
           4.0  __unpack_d
           2.0  __pack_d
           2.0  _fpadd_parts_d
         2.0  __muldf3
          12.0  __unpack_d
           8.1  __muldi3
           6.0  __pack_d
         2.0  __extendsfdf2
           2.0  __make_dp
           2.0  __unpack_f
       1.0  Dma_SendGifNormal
     1.0  FlushCache
```

### `PS2ExecMan_VSyncThread` 0x31ac08 (0.00/frame)

```
     1.0  App_VSyncCallback
       1.0  App_VSyncTick
         1.0  SSXApp_Tick
           1.0  PadQueue_Poll
         1.0  PS2ExecMan_v05
           1.0  SignalSema
     1.0  WaitSema
```

### `EAFile_CdReadThread` 0x3e3be0 (0.00/frame)

```
```

### `PS2ExecMan_WorkerThread` 0x31ac60 (0.00/frame)

```
```

## Calls per frame between subsystems (caller group -> callee group)

| from | to | calls/frame |
|---|---|---:|
| other | render | 484.2 |
| world | other | 368.3 |
| riders | other | 303.3 |
| render | other | 284.9 |
| other | world | 284.2 |
| other | libc | 200.6 |
| riders | game | 177.8 |
| game | other | 154.4 |
| sound | other | 148.5 |
| other | math | 142.4 |
| sound | kernel | 140.1 |
| riders | math | 133.2 |
| other | riders | 127.6 |
| riders | render | 112.3 |
| riders | world | 102.5 |
| ui | render | 86.1 |
| game | render | 66.7 |
| sound | libc | 53.5 |
| sound | math | 51.8 |
| world | riders | 46.3 |
| other | game | 45.5 |
| sound | riders | 28.8 |
| world | render | 26.7 |
| riders | sound | 24.6 |
| render | kernel | 17.3 |
| ui | game | 17.0 |
| ui | riders | 16.4 |
| other | kernel | 15.4 |
| libc | other | 13.6 |
| sound | game | 13.1 |
| render | math | 12.7 |
| kernel | other | 10.1 |
| game | riders | 10.1 |
| render | world | 9.4 |
| world | game | 8.4 |
| ui | other | 7.0 |
| ui | libc | 7.0 |
| game | ui | 7.0 |
| riders | libc | 6.6 |
| game | world | 6.1 |
| render | libc | 5.8 |
| sound | files | 5.4 |
| files | other | 5.0 |
| frame | kernel | 5.0 |
| frame | other | 4.0 |
| other | sound | 2.0 |
| other | frame | 2.0 |
| other | ui | 2.0 |
| game | sound | 2.0 |
| sound | world | 2.0 |
| world | sound | 2.0 |
| frame | game | 2.0 |
| sound | frame | 1.9 |
| files | libc | 1.6 |
| kernel | sound | 1.6 |
| kernel | libc | 1.3 |
| frame | sound | 1.2 |
| files | kernel | 1.1 |
| game | math | 1.0 |
| game | libc | 1.0 |
| frame | files | 1.0 |

## Entry points into each subsystem (called from outside it)

### files

- `StreamFile_GetChunk` 0x3e12e0: 4.7/frame
- `EAFile_HostReadService` 0x3e33b0: 1.0/frame
- `StreamFile_ReleaseChunk` 0x3e13e8: 0.5/frame
- `StreamFile_GetChannelBytes` 0x3e14c8: 0.2/frame
- `sceCdSync` 0x4013e8: 0.2/frame
- `sceCdSyncS` 0x401488: 0.1/frame
- `EAFile_CdSignal` 0x3e3ad8: 0.1/frame
- `StreamFile_QueueRange` 0x3e0c88: 0.0/frame

### sound

- `SSXAudioSystem_GetInstance` 0x28b180: 14.2/frame
- `SSXAudioSystem_SetPlayerTrickScore` 0x290f58: 6.0/frame
- `SSXAudioSystem_UpdateRiderWorldPaint` 0x2898a8: 6.0/frame
- `SND_SifCmdHandler` 0x3c1578: 1.6/frame
- `SND_VBlankHandler` 0x3c1980: 1.0/frame
- `CrowdAudio_GetInstanceReaction` 0x2a77c8: 1.0/frame
- `SSXAudioSystem_Update` 0x285bf8: 1.0/frame
- `SSXAudioSystem_GetGameModule` 0x28b1c8: 1.0/frame
- `SND_TimerServiceCallback` 0x3b5450: 1.0/frame
- `PATH_PlatTimerCallback` 0x2ae020: 0.2/frame
- `SSXAudioSystem_PlayWallHitSound` 0x296088: 0.2/frame
- `SSXAudioSystem_PlayBoostEmptySound` 0x299368: 0.1/frame
- `SSXAudioSystem_OnRacePositionChange` 0x299e28: 0.0/frame
- `SSXAudioSystem_ResetBoostTickPitch` 0x29ab08: 0.0/frame
- `SSXAudioSystem_PlayTakeoffSound` 0x294170: 0.0/frame
- `SSXAudioSystem_PlayWipeoutImpact` 0x296310: 0.0/frame
- `SSXAudioSystem_PlayRiderGrunt` 0x29f660: 0.0/frame
- `SSXAudioSystem_PlayRiderCollisionSound` 0x298488: 0.0/frame
- `SSXAudioSystem_SpeechRiderCollision` 0x2a0a30: 0.0/frame
- `SSXAudioSystem_PlayLandingSound` 0x2948d0: 0.0/frame
- `SSXAudioSystem_PlayWipeoutCrowdReaction` 0x2961f0: 0.0/frame
- `SSXAudioSystem_StartBoostSound` 0x298d90: 0.0/frame
- `SSXAudioSystem_StopBoostSound` 0x2992d8: 0.0/frame
- `SSXAudioSystem_ResetWipeoutSfx` 0x29e578: 0.0/frame
- `SSXAudioSystem_StopWipeoutLoop` 0x296e20: 0.0/frame
- `SSXAudioSystem_StopWipeoutLoop2` 0x297438: 0.0/frame
- `SSXAudioSystem_SpeechWipeoutRecover` 0x2a02d8: 0.0/frame
- `SSXAudioSystem_PlayGrabTrickSound` 0x29a530: 0.0/frame
- `SSXAudioSystem_PlayObjectSound` 0x2974a0: 0.0/frame
- `SSXAudioSystem_StartObjectLoop` 0x297950: 0.0/frame
- `SSXAudioSystem_StartWipeoutLoop` 0x296868: 0.0/frame
- `SSXAudioSystem_StartWipeoutLoop2` 0x296e80: 0.0/frame

### frame

- `EATimer_GetTicks` 0x3e4e98: 1.9/frame
- `PS2ExecMan_VBlankEndIntc` 0x31a490: 1.0/frame
- `EATimer_Rcnt1Handler` 0x3e4db8: 1.0/frame

### render

- `PS2GraphicsMan_ClipTestBox` 0x37de88: 177.2/frame
- `PS2GraphicsMan_GetWorldToClipMatrix` 0x3953b8: 176.4/frame
- `Vu0_RandNext` 0x36d400: 71.5/frame
- `PS2GraphicsMan_v95` 0x37e238: 47.5/frame
- `PS2BezierMan_v21` 0x38b338: 42.0/frame
- `PS2GraphicsMan_v71` 0x379860: 39.0/frame
- `PS2GraphicsMan_v99` 0x37a610: 30.9/frame
- `PS2GraphicsMan_v100` 0x37b548: 29.9/frame
- `PS2GraphicsMan_v80` 0x378808: 27.0/frame
- `PS2GraphicsMan_GetMatrix` 0x395c68: 10.8/frame
- `PS2GraphicsMan_v58` 0x3889f0: 10.6/frame
- `PS2GraphicsMan_PushMatrix` 0x3956e8: 6.9/frame
- `PS2GraphicsMan_PopMatrix` 0x395730: 6.9/frame
- `PS2GraphicsMan_GetFlipCount` 0x395510: 6.0/frame
- `PS2GraphicsMan_LoadMatrix` 0x395c38: 6.0/frame
- `PS2GraphicsMan_v27` 0x377278: 5.8/frame
- `PS2GraphicsMan_v111` 0x386bd0: 5.8/frame
- `PS2GraphicsMan_v84` 0x380ce0: 5.3/frame
- `PS2GraphicsMan_SetOrtho` 0x377458: 4.0/frame
- `PS2GraphicsMan_PushView` 0x3765b8: 3.0/frame
- `PS2GraphicsMan_PopView` 0x376768: 3.0/frame
- `PS2GraphicsMan_GetFov` 0x376c10: 2.9/frame
- `PS2GraphicsMan_SetLighting` 0x3954d0: 2.9/frame
- `PS2GraphicsMan_SetShadowHeader` 0x37ba50: 2.9/frame
- `PS2GraphicsMan_v53` 0x37d348: 2.9/frame
- `PS2GraphicsMan_DrawFill` 0x37d450: 2.9/frame
- `PS2GraphicsMan_v55` 0x37d800: 2.9/frame
- `PS2GraphicsMan_AddLight` 0x37d758: 2.9/frame
- `Renderer_Vif1DmacHandler` 0x382650: 2.8/frame
- `PS2GraphicsMan_SetViewport` 0x376a70: 2.0/frame
- `PS2GraphicsMan_GetScreenWidth` 0x395330: 2.0/frame
- `PS2GraphicsMan_GetScreenHeight` 0x395350: 2.0/frame
- `PS2GraphicsMan_SetPerspective` 0x376c58: 2.0/frame
- `PS2GraphicsMan_AddView` 0x386688: 2.0/frame
- `Vu0_UploadOccluder` 0x229f88: 2.0/frame
- `PS2GraphicsMan_v78` 0x377cf0: 1.1/frame
- `cPS2GridMesh_v03` 0x3852e8: 1.0/frame
- `Renderer_VBlankStartIntc` 0x3825c0: 1.0/frame
- `Vu0_UploadMatrixPair` 0x229f30: 1.0/frame
- `PS2GraphicsMan_EndFrame` 0x377a10: 1.0/frame

### kernel

- `WaitSema` 0x423de0: 82.4/frame
- `SignalSema` 0x423dc0: 77.9/frame
- `iSignalSema` 0x423dd0: 4.8/frame
- `FlushCache` 0x424020: 3.3/frame
- `scePadGetState` 0x3ffbc0: 2.0/frame
- `sceSifSetDma` 0x424160: 1.9/frame
- `sceSifCmdIntrHandler` 0x426230: 1.8/frame
- `sceGsSyncPath` 0x3fdb18: 1.6/frame
- `PollSema` 0x423df0: 1.3/frame
- `scePadRead` 0x3ffa58: 1.0/frame
- `sceSifWriteBackDCache` 0x426358: 0.6/frame
- `sceSifInitRpc` 0x426408: 0.3/frame
- `sceSifCallRpc` 0x426d18: 0.3/frame
- `sceSifCheckStatRpc` 0x426f08: 0.3/frame
- `scePadSetActDirect` 0x400190: 0.1/frame
- `sceSifDmaStat` 0x424140: 0.1/frame

