# Symbol recovery (naming passes 1-5)

Every function in the image is named (Ghidra project `SLUS_207.72`; exported to
`generated/functions.tsv` and, for entries Ghidra has no function for,
`generated/discovered-names.tsv`). **Ghidra and those TSVs are the source of truth for
names**; the per-rename tables of the passes were removed from this file (git history has
them). What stays: the class map, vtable slot meanings, naming conventions and invented
prefixes, corrections, and the guesses that were *not* applied.

Pass 1 renamed 935 functions on top of the ~175 from the boot-path pass (`boot-path.md`),
conservatively: a guess went to "Uncertain candidates" instead of into Ghidra.

## Notable classes found (258 classes with constructors)

Front end: about 70 `cFEState*` screens (MainMenu, CharSelect, EventSelect, Options*,
RewardGallery*, Lodge*, Net*/Online* and others), `cFEPopupConfirm`, `cKeyboardPopup`,
`cUIStateBuyPopup`, `FELocale`, `FEUISound`/`OVUISound`, `FEAsyncManager`, and
`cOVStateHUD1P`/`cOVState_ViewPopup` (in-race overlay).
Camera: `GameCamera`, `ChaseCamController`, `ScriptCamController`, and about 20
`*ChaseCameraAlgorithm`/`*ScriptCameraAlgo` classes.
Gameplay: `Player`, `ComputerActorRider`, `AirPredictor`, `RiderMetrix`, `DangerMan`,
`BENewPlayerInt`/`BEScoreInt`/`BEOptionInt`/`BELibrary`, `InputSync`, `PS2Pad`,
`InputParser`.
World and FX: `World`, `cWorldCache`, `cHullPage`, `SectionMan`, `WorldLightMan`,
`cWorldPainterMan` plus `tWPIGD_*` trigger data, `WorldTriggerManager`, `SkyBox`,
`SnowFall`, `LensFX`, `cSplashManager`, `MultiParticleMan`, and the modifier family
(`SplineModifier`, `RailModifier`, `RollerModifier`, `UVScrollModifier`, `TexFlipModifier`,
`MagnetModifier`, `Boost`, `Conveyor`, `FlexBridge`, `FloatRail`, `SpringRail`, `Fence`,
`Floating` and others).
Audio: `SSXAudioSystem`, `PS2SoundManager`, `SpeechBank`, `SpeechInstance`, `SongInstance`,
`cSongConfiguration`, `StreamBuffer`, `cWorldSound`.
Script and animation: `ScriptEngine`, `WScriptMan`/`WScriptMission`/`WScriptProcess`,
`AnimDataBase`, `AnimSequence`, `GameAnimMap`, `cAdfDataSource`.
System: `BigFile`, `MCManager`, `cPS2MemCard`, `cPS2ConsoleConfig`, `PS2GraphicsMan`,
`cPS2GridMesh`, `PS2BezierMan`, `UIEngine` UI widgets (`UIListBox`, `UIText`,
`cUITextScroll`, `UIVector`, `UIAnim`), `MoviePlayer`/`VidEnginePlayer`, and net
(`cCommSystem`, `cGameComm`, `cDSockChannel`, `LobbyMan`, `cVoipMan`, `cUsbKeyboard`).

Full list of classes with a named ctor:

`AdfDataSource` `AIAnimEventMap` `AIFwdDiffCache` `AirPredictor` `AlgorithmicScriptCameraAlgo` `AlgorithmList` `AnimCombo` `AnimDataBase` `AnimDelta` `AnimObject` `AnimSequence` `AnimTeeter` `AttachedEmitterFX` `AvaSplineModifier` `BELibrary` `BENewPlayerInt` `BEOptionInt` `BEScoreInt` `BigFile` `BoaderShadow` `BoardSparkEmitter` `BoltFileIndexManager` `Boost` `BoostModifier` `cAdfDataSource` `cAttachedDynamicEmitter` `cBoltOnFXInterface` `cCommSystem` `cCrowdAnim2D` `cCrowdRender2D` `cCutSceneTransitionMan` `cDSockChannel` `cFEOptionsCredits` `cFEPopupConfirm` `cFEStateAudioOptions` `cFEStateAuthDNASCreate` `cFEStateAuthDNASError` `cFEStateBonusMaterial` `cFEStateBraggingRights` `cFEStateBuyAttrib` `cFEStateCareerStats` `cFEStateChallengePopup` `cFEStateChallengeReceivedPopup` `cFEStateCharEquip` `cFEStateCharSelect` `cFEStateCharSetup` `cFEStateChatRoom` `cFEStateCheatCharSelect` `cFEStateCreateDuplicate` `cFEStateCreateStep1` `cFEStateCreateStep2` `cFEStateEventSelect` `cFEStateLobbyMenu` `cFEStateLodge` `cFEStateLodgeRiderDetail` `cFEStateMainMenu` `cFEStateMemCard` `cFEStateMountainRoom` `cFEStateMPCircuit` `cFEStateNetAccountMem` `cFEStateNetConfig` `cFEStateNetCreateRoomPopup` `cFEStateNetHelpPopup` `cFEStateNetMessenger` `cFEStateNetNews` `cFEStateNetQuickMessages` `cFEStateNetQuickPlayPopup` `cFEStateNetSSXWorld` `cFEStateOnlineAgreement` `cFEStateOnlineCharacterSelect` `cFEStateOnlineMainMenu` `cFEStateOptions` `cFEStateOptionsController` `cFEStateOptionsDeviceSelect` `cFEStateOptionsGame` `cFEStateOptionsHUD` `cFEStateOptionsSaveLoad` `cFEStateOptionsSound` `cFEStatePeakRoom` `cFEStatePreviewReward` `cFEStateProfileLoad` `cFEStateReadMessagePopup` `cFEStateRelogin` `cFEStateRequestLine` `cFEStateRewardGalleryCheatChars` `cFEStateRewardGalleryConceptArts` `cFEStateRewardGalleryPosters` `cFEStateRewardGalleryToys` `cFEStateRewardGalleryTradingCards` `cFEStateRewardGalleryVideos` `cFEStateRewardsRoom` `cFEStateRiderBio` `cFEStateRiderDetail` `cFEStateRuleSelect` `cFEStateSelectPersona` `cFEStateStats` `cFEStateTitle` `cFEStateTrophyRoom` `cFEStateUberTrick` `cFEStateVideoPlayer` `cFEUseExistingAccount` `cFlagManager` `cGameComm` `ChaseCamController` `cHullPage` `cKeyboardPopup` `cLunoTable` `ComputerActorRider` `Conveyor` `Counter` `cOVState_ViewPopup` `cOVStateHUD1P` `cPreFELoadScreen` `cPreGameLoadScreen` `cPS2ConsoleConfig` `cPS2GridMesh` `cPS2MemCard` `CrowdMan2d` `CrowdMan3d` `cSetupMan` `cSharedLoginMan` `cSongConfiguration` `cSplashManager` `cTransitionMan` `cUIStateBuyPopup` `cUITextScroll` `cUsbKeyboard` `cVoipMan` `cWorldCache` `cWorldPainterMan` `cWorldSound` `cWScriptCache` `DangerMan` `DeadFade` `DeadNode` `Debounce` `DebugMenu` `Default_2ChaseCameraAlgorithm` `Default_3ChaseCameraAlgorithm` `Default_4ChaseCameraAlgorithm` `DeferMan` `DetailSystem` `DirectionNeChaseCameraAlgorithm` `DynamicParticleModifier` `EditorCamAlgo` `Emitter` `FEAsyncManager` `FELocale` `FEModule` `Fence` `FEUICustom` `FEUISound` `Floating` `FloatRail` `FXLoader` `GameAnimMap` `GameCamera` `GameModule` `GameViewMan` `HaloModifier` `IdleChaseCameraAlgorithm` `InputParser` `InputSync` `InSeq` `InstancesMan` `LazyChaseCameraAlgorithm` `LensFX` `LoadIndicator` `LobbyMan` `LobbyMenu` `LookBackChaseCameraAlgorithm` `MagnetModifier` `ManualCamAlgo` `ManualCamController` `ManualChaseCameraAlgorithm` `ManualScriptCameraAlgo` `MatrixChaseCameraAlgorithm` `MCManager` `mEmitter` `MenuSys` `MeshAnim` `MidLeftChaseCameraAlgorithm` `MidRightChaseCameraAlgorithm` `ModifierBlock` `MultiParticleMan` `MultiSplineModifier` `NetConfigAuthPopup` `Object` `OcclusionMan` `OVUISound` `ParentModifier` `Particle` `PathArrow` `PlaybackMan` `Player` `PositionModifier` `Post_Race_1ChaseCameraAlgorithm` `PS2AIVisualEffectsMan` `PS2BezierMan` `PS2GraphicsMan` `PS2Pad` `PS2SoundManager` `PS2VisualEffectsMan` `RailMan` `RailModifier` `RelativeChaseCameraAlgorithm` `Replay` `ReplayCache` `RiderMetrix` `RiderModel` `RollerModifier` `ScriptCamController` `ScriptEngine` `ScriptObjCam` `SectionAllocator` `SectionMan` `SectionTable` `SkyBox` `SnowFall` `SongInstance` `SpeechBank` `SpeechInstance` `SplineModifier` `SpokeChaseCameraAlgorithm` `SpringRail` `SSXAudioSystem` `SSXStatsMan` `SubjectScriptCameraAlgo` `TargetScriptCameraAlgo` `TexMan` `tWPIGD_Ambience` `tWPIGD_Camera` `tWPIGD_Danger` `tWPIGD_Fog` `tWPIGD_LightGlow` `tWPIGD_Lighting` `tWPIGD_Mix` `tWPIGD_MusicTrigger` `tWPIGD_ScreenTint` `tWPIGD_SkyBox` `tWPIGD_Speech` `tWPIGD_Sun` `tWPIGD_Surface` `tWPIGD_Weather` `UIAnim` `UIEngine` `UIListBox` `UIText` `UIVector` `VidEnginePlayer` `World` `WorldLightMan` `WorldMemman` `WorldPainterQuery` `WorldTriggerManager` `WScriptMan` `WScriptMission` `WScriptProcess`

## Library and helper names applied

- **libc**: `memcpy` `memmove` `memset` `strcat` `strchr` `strcmp` `strcpy` `strlen` `strncat` `strncmp` `strncpy` `_printf_r` `printf` `_sprintf_r` `sprintf` `vsprintf` `vsnprintf` `vfprintf_dispatch` `_vfprintf_r` `_vfiprintf_r` `strcasecmp` `strcspn` `strncasecmp` `strpbrk` `strrchr` `strspn` `strstr` `tolower` `malloc` `free` `__malloc_lock` `__malloc_unlock` `_malloc_r` `_free_r` `bsearch` `atoi` `strtol` `_strtol_r` `__getreent` `memcmp` `__extendsfdf2` `__muldi3` `memchr` `_sbrk_r` `qsort` `__tf13bad_exception` `__tf9exception` `__tf17__class_type_info` `__tf14__si_type_info` `__tf16__user_type_info` `__tf10bad_typeid` `__tf8bad_cast` `__tf9type_info` `__tf17__array_type_info` `__tf16__ptmd_type_info` `__tf16__ptmf_type_info` `__tf16__func_type_info` `__tf19__builtin_type_info` `__tf16__attr_type_info` `__tf19__pointer_type_info` `atof` `strtod` `_strtod_r` `strtof` `_realloc_r` `__truncdfsf2` `_dtoa_r` `_Balloc` `_Bfree` `__multadd` `__s2b` `__hi0bits` `__lo0bits` `__i2b` `__multiply` `__pow5mult` `__lshift` `__mcmp` `__mdiff` `__ulp` `__b2d` `__d2b` `__ratio` `_calloc_r`
- **libm**: `tanf` `__kernel_tanf` `__ieee754_rem_pio2f` `__kernel_rem_pio2f` `expf` `__ieee754_expf` `finitef` `logf` `__ieee754_logf` `isnanf` `fmodf` `__ieee754_fmodf` `log` `__ieee754_log` `isnan` `matherr` `fabsf` `floorf` `scalbnf` `copysignf`
- **libgcc**: `__pack_d` `__unpack_d` `_fpadd_parts_d` `__adddf3` `__subdf3` `__muldf3` `__divdf3` `__fpcmp_parts_d` `__cmpdf2` `__floatsidf` `__fixdfsi` `__fixunsdfsi` `__negdf2` `__make_dp` `__pack_f` `__unpack_f` `_fpadd_parts_f` `__addsf3` `__subsf3` `__mulsf3` `__divsf3` `__fpcmp_parts_f` `__cmpsf2` `__floatsisf` `__fixsfsi` `__fixunssfsi` `__negsf2` `__make_fp` `__fixunssfdi` `__floatdidf`
- **libkernl**: `sceSifWriteBackDCache` `sceSifCheckStatRpc` `isceSifSendCmd` `_sceSifSendCmd` `sceSifExitCmd` `SyncDCache` `_SyncDCache` `InvalidDCache` `_InvalidDCache` `sceSifGetSreg` `sceSifSetCmdBuffer` `sceSifRemoveCmdHandler` `scePrintf` `iEnableIntc` `iDisableIntc` `iEnableDmac` `iDisableDmac`
- **libdma**: `sceGsResetPath` `sceGsSyncPath` `sceDmaSync` `sceDmaWatch` `sceDmaPause` `sceDmaGetChan` `sceDmaReset` `sceDmaSend` `sceDmaSendN` `sceDmaSendI` `sceDmaRecv` `sceDmaRecvN` `sceDmaRecvI`
- **libdev**: `sceDevVu0Reset` `sceDevVu1Reset` `sceDevVif0Reset`
- **libpad**: `scePadEnd` `scePadPortClose` `scePadGetState` `scePadSetReqState` `scePadGetReqState` `scePadInfoAct` `scePadInfoMode` `scePadSetMainMode` `scePadSetActDirect` `scePadSetActAlign` `scePadGetButtonMask` `scePadSetButtonInfo` `scePadInfoPressMode` `scePadEnterPressMode` `scePadSetVrefParam` `scePadGetPortMax` `scePadGetSlotMax` `scePadGetModVersion`
- **libcdvd**: `sceCdGetError` `sceCdPowerOff` `sceCdTrayReq` `sceCdGetToc` `sceCdDiskReady`
- **libmc**: `sceMcGetInfo` `sceMcOpen` `sceMcClose` `sceMcSeek` `sceMcRead` `sceMcWrite` `sceMcFlush` `sceMcChdir` `sceMcGetDir` `sceMcSetFileInfo` `sceMcRename` `sceMcDelete` `sceMcFormat` `sceMcUnformat` `sceMcGetEntSpace` `sceMcMkdir` `sceMcSync` `sceMcEnd` `sceMcChangeThreadPriority` `sceMcUdCheckNewCard` `sceSifMCallRpc`
- **libmpeg**: `sceMpegInit` `sceMpegCreate` `sceMpegAddBs` `sceMpegGetPicture` `sceMpegGetPictureRAW8` `sceMpegGetPictureRAW8xy` `sceMpegIsEnd` `sceMpegReset` `sceMpegAddCallback` `sceIpuInit` `sceIpuStopDMA` `sceIpuRestartDMA` `sceIpuSync`
- **libusbkb**: `sceSifMInitRpc` `sceUsbKbInit` `sceUsbKbEnd` `sceUsbKbGetInfo` `sceUsbKbRead` `sceUsbKbGetLocation` `sceUsbKbSetLEDStatus` `sceUsbKbSetLEDMode`
- **ea_core**: `EA_MemSet` `EA_MemCopy` `StrHashElf` `StrHash` `MEM_AllocInternal` `MEM_FreeInternal` `operator_new_named` `__builtin_delete` `EAMutex_Lock` `EAMutex_Unlock` `cBXString_ctor_copy` `cBXString_AllocBuffer` `cBXString_Release` `cBXString_dtor` `cBXString_ctor_cstr` `cBXString_assign_cstr` `cBXString_assign` `cBXString_AssignCopy` `cBXString_AllocBeforeWrite` `EA_MemFree`
- **strings**: `THREAD_yieldticks` `EA_FatalError` `SPCH_Init` `MPEG2_Get_Hdr` `MPEG2_sequence_header` `MPEG2_group_of_pictures_header` `MPEG2_picture_header` `MPEG2_extension_and_user_data` `MPEG2_sequence_extension` `MPEG2_sequence_display_extension` `MPEG2_quant_matrix_extension` `MPEG2_sequence_scalable_extension` `MPEG2_picture_display_extension` `MPEG2_picture_coding_extension` `MPEG2_picture_spatial_scalable_extension` `MPEG2_picture_temporal_scalable_extension` `MPEG2_copyright_extension` `MPEG2_Update_Temporal_Reference_Tacking_Data` `MPEG2_Get_Bits` `MPEG2_marker_bit` `PATH_status` `PATHI_copyevent`

## Uncertain candidates (not applied)

- `0x00317e98`: identical to `__builtin_delete` (0x317e50), used to free raw buffers such as
  cBXString data. Named `EA_MemFree`; it may really be `operator delete[]`.
- `0x0040b400`, `0x0040b5b0` (libmrpc): probably `sceSifMBindRpc`/`sceSifMTermRpc`.
- `0x004007a8` (libpad RPC 0x14): probably `scePadSetWarningLevel`. `0x003ff5e8`: internal pad-init RPC.
- `0x00409c48` (libmc RPC 0x15). `0x00409b60`: returns libmc RPC internals.
- libdev `sceDevVu0*`/`sceDevVif0*`/`sceDevVu1*` (0x3fe678 to 0x3ff1b0): register-poking
  helpers whose individual names weren't pinned down (only the three `*Reset` calls used by
  `Renderer_Init` were named).
- `0x00411538` / `0x0042dad8`: 64-bit divide helpers (`__udivdi3`/`__umoddi3`/`__divdi3`, order unknown).
- sceFs (eeio) functions 0x4288c0 to 0x42a950 (ioctl/dopen/getstat...), loadfile
  0x42b230 to 0x42bce8 (`sceSifLoadElf*`, `sceSifLoadModuleBuffer`, ...).
- `cBXString` members at 0x318120 (AllocCopy?), 0x3181e8 (ConcatCopy?), 0x318540 (ConcatInPlace?).
- The MPEG2 bitstream helpers `0x003b1258` (Show_Bits?), `0x003b1290` (Flush_Buffer?),
  `0x003b1498` (next_start_code?).
- `0x003cfc30`: the PATH (EA interactive music) branch-to action handler.
- Ambiguous ctor allocations not applied: `cOVStateOverlay` (5 subclasses), `cBXString`,
  `DeadNode`, `RestoreNode`, `Debounce`/`cDebounce`, `cParticleNode`, `cMeshAnim`, `cDeadFade`,
  `UIEngine` multi-site, `cFEStateLogin`/`cFEStateNetLogin`, `RiderSphereTree`/`BoardSphereTree`.
- EA DirtySock network SDK (0x3e6600 to 0x3f6c00; strings like `PRES: buddy=`, `connecting to %08x:%d`)
  and the EA SND library (0x3b4000 to 0x3c9000): no reliable self-naming strings. These are left for a
  pass with DirtySDK/EAsnd reference material.

# Pass 2

Status: pass 2 renamed **3,512 functions** in the open Ghidra project. 3,215 of them were
`FUN_` and 297 replaced earlier names (mostly pass-1 `Class_vNN` slots that now carry a
real method name, plus a few corrections). After the pass, named functions went from
about 1,110 to **4,325 of 11,844**. `generated/functions.{tsv,json}` was re-exported.

Confidence is recorded for every rename: 1,350 are "high" (anchored by a string, an
allocation tag, an RPC/command number, an SDK structure, or an unambiguous vtable slot
whose base meaning was established) and 2,162 are "med" (behaviour read from the
decompilation; the name describes what the function does, but it is not a recovered
symbol). Med names never invent behaviour; a few use invented *class* names, listed below.

## Class hierarchy and slot meanings

Slot numbers are vtable indices (slot k's pointer is at vt+8k+4). "vptr +N" is the offset
of the vptr in the object. Names marked (inv.) are invented class names for bases that
have no allocation tag or string; they were applied with med confidence.

**UI / front end**
- `UIObject` (inv., vt 0x494bd8, ctor 0x39fb30) is the widget base. Widgets come from the
  factory 0x39d078: `cUIGroup` 0x4949e8 (-> `UIMenu` 0x494928), `UIPair`, `UIIcon`,
  `UIListBox`, `UIText`, `cUITextScroll`, `UIVector`, `cUISlider`, `cUIProgress`, `cUIScroll`.
  Widget slots: 1 dtor, 7 SetFocus, 9 SetVisible, 10 Load, 12 SetAnimParam, 14 FindById,
  15 HandleInput, 16 RefreshText, 17 Render, 19 UpdateAnim.
- `UIScreen` 0x494670, `UIThread` 0x4946a0 (animation playback). 0x494cc0 is the intrusive
  list node, 0x494c98 the list.
- `UIState` (inv., vt 0x4946c8, ctors 0x39e2a0/0x39e318, vptr +8). Driven by
  `UIStateMgr_ProcessStates` (0x39ecb0), phase in bits 8-13 of flags +0x1c:
  Init -> TransitionIn -> OnActivate -> active (Update each frame) -> TransitionOut ->
  OnDeactivate -> Shutdown -> delete. Slots: 1 dtor, 4 Init (creates the screen),
  5 Shutdown, 6 OnActivate, 7 TransitionIn, 8 HandleInput, 9 PostInput, 10 TransitionOut,
  11 OnDeactivate, 12 Update, 14 OnRender, 15 Render, 16 OnAnimEvent, 17 OnChildEvent
  (0x15 popup opened / 0x16 closed), 18 OnWidgetCreated, 19 OnWidgetAction (5 select,
  6 back), 20 OnWidgetNotify.
  - `cFEState` (vt 0x46d0d0): MainMenu, Title, Options*, CharSelect, Lodge*, ... ;
    `cFEStateRewardGallery` 0x46a3c8 -> the six RewardGallery* screens;
    an unnamed net-FE base 0x46b808 (32 slots) -> all online screens, `cFEStateLogin`
    0x468388 -> `cFEStateRelogin`; popups `cFEPopupSelectMultiplayerMode`,
    `cFEPopupScreenPos`, `cFEPopupVideoCalibration`, `cFEPopupOptionReset`,
    `cFEStateBackground` (named from inline allocation tags).
  - `cOVStateOverlay` (vt 0x474e08): every in-race overlay, and the memory-card base
    0x46c980 (MemCard, ProfileLoad, OptionsDeviceSelect). Overlay/FE classes without a known
    class name use a synthetic prefix from the screen they load: `OVScreen<Screen>_*`,
    `FEScreen<Screen>_*` (136 functions, med).

**Application**
- `App` (vt 0x48dbe8) -> `SSXApp` (vptr +0x5c): 3 Init, 4 Shutdown, 5 VSync ring resync,
  6 consume tick, 7 VSync tick.
- `AppModule` (inv., vt 0x47d980, ctor 0x231cd0): 1 dtor, 2 Load (allocates the
  module's systems, e.g. `GameModule_Load`), 3 Unload, 4 TryExit (polled), 5 Enter,
  6 Update, 7 Render. Subclasses: GameModule, FEModule, StartScreen, PreStartScreen,
  InitLoadScreen, cPreGameLoadScreen, cPreFELoadScreen, cGameLoadScreen, cFELoadScreen.

**Gameplay**
- "AI" race manager (alloc tag "AI", ctor 0x1286a0, vt 0x458488 at +0xcc): 1 dtor, 2 Init,
  3 Update, 4 Render, 5 Restart, 6 CreatePlayers, 7 SaveState, 8 LoadState. Race state
  machine states 1..7 = GameInit, Freeride, PreRace, Countdown, Race, EndRace, Shutdown,
  each an embedded object with slots Enter/Exit/Update.
- `Player` / `ComputerActorRider` derive from `RiderBase` (inv., vt 0x458710 at +0xde8:
  1 GetControlInput, 2 SaveState, 3 LoadState) and share the virtual base `Rider` (inv.,
  ctor 0x11b698; position +0x110, velocity +0x1e0, RiderModel +0x780, AirPredictor +0x788,
  RiderMetrix +0x790). Rider FX mixins (cAngerFX, cBoardWakeFX, cSpeedBoostStreamerFX,
  cAirStreamerFX, ...) also hang off this virtual base.
- `BELibrary` singleton (485 callers) with phase objects whose slot 1 is GetInterface(id):
  ids 0..13 = BENewRaceInt, BENewPlayerInt, (2 unnamed), BECharInt, BEOptionInt, BESaveInt,
  BEBAGTInt, BENetworkInt, BEScoreInt, BEBoltInt, BEMissionInt, BEEconInt,
  BEAggressionInt, BERewardInt. Interface slots 2 Reset, 3 SaveState, 4 LoadState.

**Cameras**
- `CameraBase` (vt 0x45b8b0) -> `GameCamera`: 1 dtor, 2 Init, 3 Update. GameCamera owns the
  manual/chase/script controllers at +0xa4/+0xa8/+0xac (active at +0xa0).
- `CamController` (vt 0x45b938): 1 dtor, 2 Update, 3 ResetForRider, 4 Reset.
- `CameraAlgorithm` (vt 0x45bde0): 1 dtor, 2 GetView (pure), 3 ResetForRider, 4 Reset,
  5 Update, 8 PostUpdate, 9 Tick, 12 SetTarget. Under it `RiderCameraAlgorithm` (inv.)
  -> `ChaseCameraAlgorithm` (14 chase algorithms + Spline, Superpipe) and
  `BoundedCameraAlgorithmBase` (inv.); `ManualCamAlgoBase` -> ManualCamAlgo/EditorCamAlgo;
  `ScriptCameraAlgo` -> Manual/Target/Subject/Algorithmic. `ChaseCamController_SetAlgorithm`
  (0x15d078) is the camera-id factory.

**World**
- `GameProcess` (inv., vt 0x491f00, priority list at 0x4a5988): 1 dtor, 2 Update,
  3 PostUpdate, 4 Render, 12 Save. Subclasses cTransitionMan, SnowFall, cSplashManager,
  cFlagManager, SkyBox, cCutSceneTransitionMan, and `TypedProcess` (inv.) ->
  `SyncProcess` (inv.) -> RailMan and `WorldObj` (inv., 46 slots: 2 Update, 4 Render,
  12 Save, 33 BuildMatrices, 34 KillRestorable, 35 Kill; every class has a stream-loading
  `_ctor_2`): MeshAnim, Fence, Flag, FlexBridge, Counter, Conveyor, Boost, DeadFade,
  DeadNode, RestoreNode, Hide, `FlexRail` -> SpringRail/FloatRail, and `ModObj` (inv.,
  owns a ModifierBlock) -> Object, Particle, Floating, Debounce.
- Transform modifiers (Position, Parent, Spline, MultiSpline, AvaSpline, Magnet, Roller):
  2 Update, 3 UpdateTransform, 18 GetMatrix, 27 Save.
- tWPIGD_* world-painter data: 63 inherited "return 0" getters (not Ghidra functions), then
  64 SaveState, 65 LoadState, 66 Blend.
- VFX component base (vt 0x488680): 1 dtor, 2 Update, 3 Render; the PS2(AI)VisualEffectsMan
  managers fan out to ~17 components (named `Vfx<Tag>_*` from allocation tags).
- `cPS2MemCard` (vt 0x486f78) is an async state machine with Request*/Do* pairs
  (Open, Seek, Read, Write, Flush, Close, Mkdir, Delete, Format, ListDir).
- MenuSys debug-menu framework: page stack, `MenuSysPage` 0x486ed0, `MenuSysItem` 0x486f28
  with 20 item subclasses (Bool, Enum, Submenu, Int, Float, Color, Vector, ...).

**Script, replay, audio**
- ScriptEngine (vt 0x4821b8) over two bases (0x482528, 0x4824b8); slots 1 dtor, 5 Update,
  7/8/9 Load/Cancel/Unload script bank. scdat loading (`ScriptEngine_LoadMaster`,
  `LoadMainScripts`, `LoadWorldScripts`), bank managers for 'scr'/'anm'/'snd' data.
- `ReplayStream` (vt 0x481898): 1 Write, 2 Read, 3/4 Read Object/Node ref,
  5/6 Write Object/Node ref, 7 dtor.
- Audio classes use virtual inheritance over the base vt 0x483c00 (bound to
  PS2SoundManager). PS2SoundManager (vt 0x4835f8): 1 Init, 2 SetOutputMode, 3 Shutdown,
  4 AllocSpuBlock, 8 dtor.

**Libraries**
- EA libs (0x3ad000-0x3f7500): `AV_Player_*`/`AV_Audio_*` movie player; RCMP codec classes
  (from `.gnu.linkonce` symbol names: `PS2_SONY_CODEC_INTERNAL`, `RCMP::DECODER`,
  `RCMP::AV_MS_TIMER`); MPEG2 bitstream (mpeg2decode names); `SHP_*` (.ssh/.fsh shapes,
  DXT, GS swizzle); `EAC_*`/`RefPack_*` decompression; EA SND (`SND_*`, `SNDSTRM_*`,
  `SNDMIDI_*`, `SNDENC_*`; RPC id 'SND' 0x534e44); PATH interactive music (`PATH_*`);
  `SPCH_*`; `UStr_*` (UCS-2); EAFile (`EAFile_Queue*`, `AsyncFile_*`, `StreamFile_*`,
  `BigFile_*`); `EATimer_*`, `EAThread_*`; DirtySock (`TagField*` lobbytagfield API,
  `NetConnStatus`, `NetConnMAC`, `DirtyRpc_*` IOP transport).
- Sony/runtime (0x3f8000+): libscf (`sceScfGet*`, newly found), eeio `sce*` file calls
  by RPC number, sifrpc server side, iopheap/loadfile, deci2, libdev VU/VIF helpers, libusbkb
  extras, libmpeg internals (`libmpeg_*`), gcc 2.95 EH (`__throw`, `frame.c`), C++ RTTI
  runtime, newlib stdio/scanf/locale, `__udivdi3`/`__umoddi3`/`__divdi3`/`__moddi3`,
  `DnasRel_*` (DNAS ELF relocator).

## Corrections to earlier names

- `sceCdSearchFile` (0x400ea8) takes a layer argument: renamed `sceCdLayerSearchFile`;
  the wrapper at 0x4011c0 that passes layer 0 is the real `sceCdSearchFile`.
- `PS2VisualEffectsMan_v01/_v02` were slots of the member component allocated as "Tube",
  not of the manager: now `VfxTube_dtor`/`VfxTube_Update`. The pass-1 rule "lowest-offset
  vptr store in the ctor" can pick up members built inline in a ctor; other `_vNN` names
  from such ctors may have the same issue.
- `DeadNode_v02` (0x3608e8) is the shared WorldObj slot 2: `WorldObj_Update`.
- `mEmitter_v01/v02/v05` -> FreeData/ResetParticles/AllocData; `PS2GraphicsMan_v01/v03/v04`
  -> dtor/Shutdown/SelectDisplayMode; `PS2Pad_v01/v02` -> dtor/Update.
- Pass-1 uncertain candidates now resolved: 64-bit divide helpers, `sceSifMBindRpc`/
  `sceSifMTermRpc`, `scePadSetWarningLevel` (med), MPEG2 `Show_Bits`/`Flush_Buffer`/
  `next_start_code`, `PATH_ExecuteAction` (0x3cfc30), libdev VU/VIF helpers.
- Not changed but suspicious: `cFEStateAuthDNASCreate_ctor` (0x2561b8) builds the DNAS auth
  object allocated with tag 'DNASMAN'; `BoardSparkEmitter_ctor` looks like a generic
  particle-system ctor shared by Emitter and DynamicEmitter; `InitLoadScreen_Enter`
  (0x232568) is in AppModule slot 2, so it is really a Load.

## Finding for the recompiler

About **1,300 vtable slot targets are not function starts in Ghidra** (out of 9,740 slots
scanned). Most are tiny virtuals (`jr ra` stubs such as 0x3a5aa8-0x3a5b40, 0x2bc500+8k,
0x360720-0x360a20) or real method bodies (many Rider/Player and FE overlay slots,
0x1406b0-0x140c80, 0x1de860, 0x1d93d8, ...). Also undefined: 0x411530 (`__pure_virtual`),
the `.gnu.linkonce` RCMP functions at 0x42e020-0x42e4c8, `sceSifRegisterRpc` (~0x426fdc),
and some jump-table case blocks outside their function bodies. These can't be named
until functions are created there, and they are indirect-call targets.

## Uncertain candidates (not applied)

- Gameplay: 0x14edd8 (BE interface id 2, tag 'BECharInt' but its own vtable);
  0x113aa0/0x113ad8 (probably AirPredictor Save/LoadState); ComputerActorRider sub-state
  think handlers 0x10a960, 0x10ad78, 0x10aa70, 0x10b590, 0x10b250, 0x10b790, 0x10aed8,
  0x10b750; RiderBase state Enter/Exit targets; RiderMetrix event handlers
  0x119368-0x119e38; DangerMan/FocusMan internals 0x101728-0x101b60.
- Cameras: chase pipeline steps 0x162998 ... 0x166f90; bound-object virtuals
  0x16e560/0x16f068/0x16fbb8; trigger shape readers 0x171a20-0x171d10.
- App/net: GameModule state serialisation 0x231348/0x2314d0 (direction unknown); race
  start/finish sequencers ('finishov' 0x233cd8, 'StartlightBegin' 0x234ad0,
  'StartgateOpen' 0x234c68); about 100 DirtySock lobby callbacks 0x259xxx-0x265xxx;
  NetGameUtil/NetGameLink candidates 0x3e6e08/0x3e6f40/0x3e6fc8 and
  0x3e66f0/0x3e6958/0x3e6b10/0x3e6c80/0x3e67f8.
- Script: InSeq bytecode interpreter 0x272300-0x2752a0 (no dispatch table to anchor
  opcode names); script command wrappers 0x275dd8-0x2779xx.
- Audio: SSXAudioSystem gameplay logic 0x28c000-0x2a6000; BankMonitor 0x2acd08/0x2acfe0;
  SND channel params 0x3b8700/0x3b8838/0x3b8978; SND mixer/encoder kernels
  0x3bc000-0x3c0800 and the EE DSP chain 0x3c9000-0x3cf000; SPCH scheduler 0x3d7ec8...
- World: four unnamed rider FX mixins (ctors 0x2e3680, 0x2eaae0, 0x2f0e88, 0x2f6238);
  collision/sphere-tree code 0x327900-0x341300 (~130 functions, no strings);
  PS2GraphicsMan draw slots 71-100; dlmalloc independent_calloc/comalloc/valloc/pvalloc
  at 0x31f810/0x31f838/0x31faa8/0x31fb00 (order unknown).
- Libraries: network-config module 0x3f8000-0x3fcf98 ('EA Login', icon.sys);
  IOP heap RPC 4-9 (0x42ac10, 0x42ae18-0x42afe8); libmc 0x409c48/0x409b60;
  float/printf helpers 0x41af88, 0x41ed60, 0x41f9d8; C++ dtors 0x412e38/0x412ed0.
- About 600 FE functions remain `FUN_` (large handlers without strings; FEUIInput,
  FEModule and LobbyMenu families near 0x180000).

# Pass 3

Status: pass 3 (2026-10-15) named **5,242 more functions** in the open Ghidra project and
corrected 5 earlier names. Named functions went from **4,473 to 9,715 of 11,844** (82 %);
2,129 remain `FUN_`. `generated/functions.{tsv,json}` was re-exported afterwards, and the
function extents are byte-identical to the previous export (only names changed).

Confidence of the applied names: 1,012 high, 4,235 med (same meaning as in pass 2: "high" is
anchored by a string, allocation tag, vtable slot of established meaning, RPC/syscall number,
SDK/libc structure or a documented struct field; "med" describes behaviour read from the
decompilation, usually with a named caller or callee as the anchor). 1,071 further
proposals were "low" and were **not** applied; they are listed under "Uncertain candidates".

## What was identified (highlights)

- **Race path.**
  - **Riders:**
    - ComputerActorRider think states (`ComputerActorRider_Think{Cruise,BoardPress,Prewind,NaturalAir,Spin,Gate,Railslide,Wipeout}`), cruise modes, trick choice and rubber-banding.
    - Per-frame rider phases from `AI_Update`/`AI_Render` (`Rider_BeginFrame`, `Rider_UpdateImpacts`, `Rider_UpdateProgress`, `Rider_ProbeGround`, `Rider_UpdateVisibility`, ...).
    - All rider state/sub-state SaveState/LoadState pairs and dtors, plus rider collisions and race paths (`Path_*`, `PathData_*`).
  - **Race management:** FocusMan/DangerMan, GameModeMan and its mode handlers, GameModule save/load and the race start/finish states.
  - **In-race HUD:** `HUD_*` draw helpers, `cOVStateHUD1P_*`, the race-progress table, `GameUI_*` and `MessageCenter_*`.
  - **Cameras:** the chase pipeline (names from `structs/camera.md`), camera triggers and bound objects.
  - **Collision:** `SphereTree`, `OverlapList`, the `Nash` loose octree and its `Cell` nodes.
  - **Animation:** RiderModel, AnimController, AnimDataBase, GameAnimMap.
  - **World:** world objects and modifiers, world painter getters, world-script built-ins (`WScriptFn_*`), WScriptMan/Process/Task, cWorldCache and its section streamer, `cStreamMan`.
  - **Renderer:** DeferMan, TexMan/VRAM heap, depth-fog post pass, bezier patch cache, BoaderShadow, SH irradiance.
- **Back end:** BE interface methods for all 14 interfaces, career/reward/mission/score data, BESaveInt save files, bolt-on DB.
- **Front end:** cFEPopupConfirm and cKeyboardPopup, the net FE screens, reward gallery, CharEquip, BoltFile cache, FEUIInput (36 action slots), load-screen states, memcard UI callbacks.
- **Audio (game side):** SSXAudioSystem volume channels, music and speech triggers, board loop sounds, BankMonitor, CrowdAudio, StreamBuffer/StreamArray, SpeechSystem, MusicPlayer.
- **Libraries:**
  - **EA audio:**
    - SND codecs (PCM16, EA-XA ADPCM, EA MicroTalk), the EE software mixer, DSP nodes and EE effects.
    - SNDSTRM streams; SNDENC is a real-time DTS encoder.
    - PATH event queue and fades; SPCH event tables and queue.
  - **EA files and memory:** EAFile/StreamFile/BigFile, EA MEM heap, HUFF encoder.
  - **Movies:** AV subtitles, RCMP classes.
  - **DirtySock:** NetGameLink, NetGameUtil, ProtoPing and ProtoAries (from DirtyRpc 4-char codes), LobbyApi, BuddyApi, TagField, Utf8.
  - **Runtime:**
    - The EA account-login memory-card save library (`SLogin*`).
    - libmpeg internals, iopheap/loadfile, gcc 2.95 RTTI/EH classes, newlib helpers and kernel patch helpers.

## Invented or synthetic names

Most names reuse class names that already exist or come from an allocation tag. These
prefixes are descriptive (applied at med) and have no symbol anchor. Rename them if real
names turn up.

- **Unnamed base classes:**
  - `NetFEState_*` / `NetFE_*`: the net-FE screen base, vt 0x46b808.
  - `MemCardUI_*`: memory-card message callbacks, each anchored by its `kT_MEM*` string.
  - `FEScreenBlank_*`: vt 0x46b6f8.
  - `cFEPopupConfirm_BaseCtor/BaseDtor`: the popup base, vt 0x46cbd8, tag "createPopup".
  - Base-subobject ctors/dtors named after their only user: `*_BaseCtor` / `*_BaseDtor` / `*_BaseSave`.
- **Game modes, states and overlays:**
  - `GMHandler<N>_*`: the GameModeMan handler classes, by array index.
  - `GameState<Name>_*`: GameModule's embedded states, using the names from `structs/world.md`.
  - `OVScreenPauseMode1..6_*` and `OVScreenPauseFreeride_InitMode1..6`: the six pause-menu subclasses.
  - `OVScreenFinishOv2`, `OVScreenSaveRecords`, `OVScreenBCFail`.
- **Module prefixes over unnamed objects:**
  - `EventSelect_*`: the component shared by cFEStateEventSelect and OVScreenMap.
  - `RaceProgress_*`, `MessageCenter_*`, `HUD_*`, `GameUI_*`, `LoadIndicator_*`.
  - `AIVfxProcess` / `VfxProcess` / `StatsProcess`: from world.md field names, because the allocation tags collide with existing class names.
  - `Render2D_*`: the "2DRender" object.
  - `InSeq_Instance*`, `InSeq_Channel*`, `InSeq_{Curve,Ramp,Span,Event}Track*`.
  - `WorldBlockPool`, `BoltFile_*`, `FEModule_RiderPreview*`, `WScriptFn_*`, `LunoVM*`, `Nash_*`.
  - `SLogin` / `SLoginStore` / `SLoginMc`: from the 'slogin' save directory.
  - `ScriptCam_*`, `RiderBase_*{PowerUpTrail,AttackFX,CheatCode}`: the rider FX mixins.
- **Names by number, not meaning:**
  - Speech triggers by speaker and event id (`SSXAudioSystem_DJSay20C8`).
  - `RiderAnim_UpdateSeqType00..0F`.
  - `MCManager_StateNN`.
  - `cDynamicSprayFX_UpdateEmitterN`.
  - `WorldPainter_Get<Type><Offset>`.

## Corrections to existing names

| Address | Was | Now | Evidence |
|---|---|---|---|
| 0x1036a0 | `RiderAnim_OnAnimEvent` | `RiderAnim_UpdateSequence` | Switches on the anim-table update type and calls per-type sequence updaters; events fire only afterwards |
| 0x194588 | `FEUICustom_v04` | `cFEStateTransition_Init` | Slot 4 of vt 0x46bb88, the UIState that `FEUICustom_ctor` allocates as 'cFEStateTransition' |
| 0x1de668 | `FEUICustom_v01` | `cFEStateTransition_dtor` | Same vtable, slot 1 |
| 0x251fc8 | `MEM_PrintBlockLine` | `MEM_PrintHeap` | Body is `MEM_DumpHeapInfo(heap, fmt, line printer)` |
| 0x3d1038 | `PATH_SetVolume` | `PATH_SetPaused` | Calls `PATH_PlatSetPaused` and sets the global pause flag checked by `PATH_Update`. The volume setter is `PATH_SetVolumePercent` |

Pass-3 slot names that were wrong and are now fixed (they were `FUN_` before this pass):

- `SSXAudioSystem_ReleaseBank` became `SSXAudioSystem_v02`.
- `Debounce_Render` became `Debounce_v15`.
- `ModObj_Shutdown` became `ModObj_v05`.
- `ModObj_Reset` became `ModObj_v14`.
- `cFlagManager_ResetBoxes` became `cFlagManager_Load`.
- `GameModule_v08` / `_v09` became `GameModule_SaveState` / `_LoadState`.

**Suspicious, not changed:**

- **Memory and RPC library names:**
  - `RollerModifier_dtor` (0x360b00) is slot 1 of the base vtable 0x4913f8, not of RollerModifier's own vtable. The real one is 0x361c78, now `RollerModifier_v01`.
  - `sceSifLoadModuleBuffer` (0x42b230) takes a 4th result argument, so it is probably `_sceSifLoadModuleBuffer`; 0x42b800 would then be the public one.
  - `sceSifMBindRpc` (0x40b400) has 6 arguments, so it is probably `sceSifMBindRpcParam`, with 0x40b3d8 as the default-argument wrapper.
- **Wrong class:**
  - `cFEStateAuthDNASCreate_ctor` (0x2561b8) is really the DNASAuth construction (its body is `DNASAuth_Init`).
  - `PS2ExecMan_ctor` (0x317458) stores 0x48dc30, probably a base-class vtable.
  - `cPS2ConsoleConfig_v01` is that class's deleting dtor.
  - `cPS2MemCard_v29` is the save-name wrapper around `RequestReadSave`.
- **Naming conflicts:**
  - `BENewRaceInt_SetGameMode` / `GetGameModeName` act on the debug menu's "Game Path" (World Circuit / Quick Play / Multiplayer). The debug "Game Mode" is the event type.
  - The pass-2 `WorldObjFactory_*` names are entries of the same world-script built-in table (0x441f38) that pass 3 names `WScriptFn_*`.

## Uncertain candidates (not applied)

The table at the end of this section has 1,071 low-confidence proposals: address, proposed name, evidence. They are mostly:

- names that would need an invented class name (KeyboardScreen, MemCardStateBase, Coll* volume classes, cPlayerPulseFX, ...);
- guessed translation units for static initialisers;
- event handlers whose trigger isn't visible locally;
- byte-identical duplicates.

Kernel syscall 0x83 at 0x42c1a8 (`FindAddress`) was also downgraded to low.

Still `FUN_` after pass 3 (2,129 functions):

- empty and identity stubs, and opaque single-field accessors;
- large GS/VU drawing kernels without anchors (HUD quads, Nash classify);
- SPCH sentence assembly;
- unanchored callbacks in the lobby and memcard state machines;
- methods of classes that have no name yet.

# Pass 4

Status: pass 4 (2026-10-15) named **the last 2,129 `FUN_` functions**. Every one of the
11,844 functions in the Ghidra project now has a name. `generated/functions.{tsv,json}` was
re-exported afterwards; only names changed, not extents.

Pass 4 also applies the low-confidence names, which earlier passes held back. The goal
here is a name for every function, so the call graph reads as a program (the phase 3
groundwork in `docs/plans/roadmap.md`). A name is still a claim that can be wrong:
- **high:** 12 names;
- **med:** 1,125 names, which describe behaviour read from the decompilation and anchored
  by a named caller, callee or vtable slot;
- **low:** 992 names, mostly wrappers, empty stubs, uncalled inline copies and accessors
  named after their owner and field offset.

Treat a low name as a label, not a finding. The appendix below gives the evidence for
each one.

## What was identified (highlights)

- **Collision volumes.** CollHull (vt 0x48e590), CollSphereTree (0x48e5f0), CollSphere
  (0x48e650) and CollSegment (0x48e6a8) share one interface. The slots are:
  - 1: Transform
  - 3: size
  - 4: CollideTriangle
  - 5: CollideTriangle2Sided
  - 6: CollideSphereTree
  - 7: CollideAABB
  - 9: OverlapsAABB
  - 10: dtor
- **Memcard state base (vt 0x46c980).** The shared base of cFEStateMemCard, ProfileLoad,
  OptionsDeviceSelect and the autosave/save-records/save-replay/profile-PDA screens. Its
  functions are named `MemCardStateBase_*`; the prefix is invented. The MCManager wrappers
  over cPS2MemCard (vt 0x486f78) are mapped by slot:
  - 36/37/38: write a save by name, wide name and slot;
  - 29: read by slot;
  - 5: RequestGetInfo;
  - 12: Close.
- **Module bases.**
  - vt 0x46d970 is the base of AppModule, GameModule and FEModule (`ModuleBase_dtor`).
  - vts 0x46dba8, 0x46dd28 and 0x46dd60 are the input, custom and sound UI bases.
- **Renderer.** PS2GraphicsMan slot 94 (0x36a2c0) applies the plant-sway matrix from
  `VfxPlants_GetSwayMatrix` (0x38f2a8).
- **EA libraries.** SND config accessors are named by config offset (`SND_SetConfig28`, ...).
  The SPCH sentence builder (0x3d7000-0x3db900) has a consistent `SPCH_*` set.
- **Front end.** LobbyMan has 25 member-function trampolines (`LobbyMan_On*_Thunk`). The
  HUD draw helpers include `HUD_DrawSpeedometer`, anchored by the 'KM/H'/'MPH' strings.

## Invented prefixes

These prefixes have no symbol anchor. They name a vtable or a group:
- `MemCardStateBase`, `UIScreenVt46d1a0`, `AppModuleVt47d818`, `LobbyMenuBase`, `FEModuleCamera`;
- `FENetHook`, `FENetHook2`, `HUDMeter`, `OVScreenId0E`;
- `RuleSelectMember`, `ValuePair`, `UIInputMask`.

## Open questions

- **0x385260 `GridMeshBase_dtor` (pass 3)** is slot 1 of vt 0x4938e0 and chains into base
  vt 0x493938 (`GridMeshBase_BaseDtor`). It is probably a derived class's dtor.
- **`PATH_PlatGetTrackHandle` / `PATH_PlatSetTrackLevel`** both use field +0x14. One of the
  two names is probably wrong.
- **`BENewPlayerInt_GetCharStateFlag` vs `BERewardInt_IsRivalRewardPendingFor`.** They
  overlap on the rival flags.
- **0x1f7548.** The pass-3 hint `OVScreen_SetSelectedProfileSlot` was wrong: the function
  pops the WScriptMan event queue. It is named `OVScreenBCStart_PopEventArg`.
- **21 functions in chunk 03 had a different pass-3 candidate.** The reviewer's name won:
  0x317d48, 0x320550, 0x320b28, 0x320b48, 0x320920, 0x3150f8, 0x3158e8, 0x31b9b0, 0x319cc8
  and 0x31a200, plus 12 wording variants. The pass-3 appendix lists the alternatives.
- **`libc_EioStub_45a0` / `libc_EioStub_45c8`.** Two newlib syscall stubs that return EIO.
  Which syscalls they are is unknown.

# Pass 5: entry points Ghidra has no function for

Status: pass 5 (2026-10-15) named the **1,462 entry points that `recomp-ee` translates but
Ghidra has no function for**. The recompiler finds them from pointers in data (mostly vtable
slots) and from address constants in code. They include `GameModule_Render` (0x22b008), the
per-frame render of the race. The names live in `generated/discovered-names.tsv`, which
`recomp-ee` reads for the `pub use` aliases and `tools/callgraph.py` reads for its reports.

The HTTP backend cannot create functions, so a Ghidra script does it:
`tools/ghidra_create_discovered.py`, run from the Script Manager, then
`tools/export-ghidra-functions.py` and save. After that re-export, the TSV is redundant
for the entries Ghidra accepted.

- **Confidence:** 701 high, 545 med, 216 low.
- **Not functions:** 110 entries are marked `NOTFUNC_<addr>`. They are jump-table case blocks,
  exception landing pads, bit-flag constants that look like pointers, a string fragment and an
  embedded blob's bounds. The recompiler still translates them; that is harmless, since nothing
  calls them as functions.
- **Method:** same as pass 4. Each function got a card with its data references (±6 words, so
  sibling vtable slots name the class), its race callers from the `callprof` call graph, its
  callees and the recompiled body. Four reviewers worked in parallel, and the merge was
  validated for coverage, uniqueness, clashes with live names and identifier syntax.

Highlights:
- **World-painter data (`tWPIGD_*`).** The 14 classes have vtables 0x238 bytes apart, from
  Danger at 0x483e20 onward. The 63 base getters are named by meaning where a
  `WorldPainter_Get*` caller shows it, otherwise by field offset.
- **Slot meanings.** Newly established slot meanings for:
  - transform modifiers, where slot 26 GetType returns Spline 1 … AvaSpline 7;
  - UI widgets, where slot 22 is a type id: cUIGroup 0x10 … cUIScroll 0x21;
  - UIState (slot 24 is OnEvent, from `MessageCenter_PostMessage`);
  - UIInputBase (44 slots);
  - the GameState, GMHandler, ScriptBinding and InSeq track bases;
  - the BE interfaces (slots 2, 3 and 4 are Reset, SaveState and LoadState; the phases use 3/4
    as Exit/Enter);
  - the PS2BezierMan LOD distances (slots 6-9).
- **C++ runtime:** the gcc 2.95 `__tf*` typeinfo functions and the RCMP RTTI slots.

Found along the way (not yet acted on):
- **Functions neither Ghidra nor the recompiler has as an entry:**
  - the switch heads at 0x145818, 0x1458c8, 0x150e68 and 0x1511e8;
  - the function holding the computed jump at 0x37213c;
  - 0x392e98 (a pixel-format switch);
  - 0x3c5538 (the SND output-mode names);
  - the stub at 0x1e0018. The recompiler took the delay slot 0x1e0014 as an entry instead.

  Their code is translated as part of the previous function's runs, so it runs; it is just not
  named.
- **The frame-description (EH) tables** at 0x44ee3c and 0x4534c0 give function starts and
  lengths. They would be a reliable boundary source for the recompiler.
- **`__check_eh_spec`** really ends at 0x412cd8, not 0x412cb4.
- **`render.md`:** the colour-mask getter is slot 64 (slot 65 is empty).
- **Base vtable 0x4913f8:** slot 1 still carries the wrong name `RollerModifier_dtor`.

The full list, with evidence, is `generated/discovered-names.tsv`.
