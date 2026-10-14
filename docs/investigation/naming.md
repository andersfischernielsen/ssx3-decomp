# Symbol recovery (naming pass)

Status: a pass over the open Ghidra project `SLUS_207.72` renamed **935 functions**,
on top of the ~175 names from the boot-path pass (`boot-path.md`). Every rename went
through the GhidraMCP backend (`rename_function_by_address`). After the pass,
`tools/export-ghidra-functions.py` refreshed `generated/functions.{tsv,json}`, so the
generated Rust picks up the new names as aliases.

The rule throughout was to be conservative. Where a name was a guess, it went into
"Uncertain candidates" below instead of into Ghidra.

## Counts by area

| Area | Count | How identified |
|---|---:|---|
| C++ constructors (`Class_ctor`, `Class_ctor_2`, ...) | 284 | allocation-name pattern (see below), checked against vtable stores |
| C++ virtuals and dtors (`Class_vNN`, `Class_dtor`) | 377 | vtable slots of the classes above |
| libc (newlib) | 84 | structure and call graph (alphabetical `libc.a` layout), printf/strtod/malloc internals |
| libm (newlib float wrappers) | 20 | `w_*` wrappers name themselves through the `__exception.name` string (`"expf"`, `"logf"`, `"log"`, `"fmodf"`) |
| libgcc (fp-bit soft float, DI helpers, `__tf*` typeinfo) | 30 (+15 `__tf*` counted under libc) | fp-bit.c structure (pack/unpack/fpadd_parts call pattern) |
| libkernl (SIF cmd/RPC, D-cache, scePrintf) | 17 | call graph and cache ops (`DXWBIN` vs `DXIN`) |
| libdma / libgraph / libdev | 13 + 3 | CHCR mode bits (0x105 chain send, 0x101 normal, 0x109 interleave, 0x10x receive) and register writes |
| libpad | 18 | padman RPC command numbers (6 SetMainMode ... 0xE Close, 0xF End, 0x12 GetModVersion) |
| libcdvd | 5 | S-cmd/N-cmd RPC numbers (S3 GetDiskType, S4 GetError, S5 TrayReq, S0x21 PowerOff, N4 GetToc) |
| libmc | 21 | mcserv RPC numbers 1..0x12, 0x14, 0x35 plus string `sceMcUdCheckNewCard` |
| libmpeg / libipu | 13 | `sceMpegCreate` error strings, IPU register use |
| libusbkb / libmrpc | 8 | self-naming error strings (`sceUsbKbRead: sceSifCallRpc -> %d`) and `sceSifMCallRpc` |
| EA core (memory, string hash, cBXString, mutex) | 20 | decompiled semantics |
| Self-naming debug strings (MPEG2 header parser, `THREAD_yieldticks`, `SPCH_Init`, `PATH_status`, ...) | 22 | strings |
| **Total** | **935** | |

## Method

1. **Bulk dumps through the HTTP backend** on 127.0.0.1:8080 (much faster than per-call MCP):
   all 17.5k defined strings with their xrefs (`/strings` + `/xrefs_to`), and the
   disassembly of all 11,844 functions (`/disassemble_function`, about 12 s). A local
   call graph (from `jal` targets) and a string-use map were built from these.
2. **Libraries** (top of `.text`, 0x3fd000 to 0x42e000, plus EA libs below): named per region
   by structure, RPC command numbers, hardware registers and error strings, with
   decompilation as the check. newlib's `libc.a` keeps its members in alphabetical order
   (`memcpy, memmove, memset, strcat, strchr, strcmp, strcpy, strlen, strncat, strncmp,
   strncpy`), which made the string block easy to anchor and confirm.
3. **C++ constructors from allocation names.** EA code allocates objects as
   `p = EA_MemAllocNamed(size, "cClassName", flags, 0); p = Ctor(p, ...)` (also through
   `operator_new_named` at 0x317e30). A scan of the disassembly tracked `lui`/`addiu` into
   `a1` at each allocator call, then found the next `jal` with `a0 = v0`. That gave 331
   (name, target) pairs. They were filtered as follows:
   - drop targets in library code and allocation tags that aren't class names
     (`"Save Profile"`, `"createPopup"`, `m_containerN` pools, and so on);
   - keep a target only when exactly one name allocates it;
   - read the vptr that each ctor stores (`sw <vtable>, off(this)`). When several
     allocation names share a vtable, only the name whose ctors all use that vtable is kept.
     When one name maps to ctors with *different* vtables (for example `cOVStateOverlay`,
     which is used to allocate five subclasses), the name is dropped.
   - Overloaded ctors that share a vtable get `_ctor_2`, `_ctor_3`.
   Spaced names were CamelCased (`"Chase Cam Controller"` became `ChaseCamController_ctor`).
4. **Vtables.** gcc 2.95 vtables (no RTTI in game code) are arrays of 8-byte entries
   `{short delta; short idx}, fnptr`, with entry 0 = (0,0). For each class, the primary
   vtable is the last vptr stored at the lowest `this` offset in its ctor. Its slots were
   read until the first non-function word. A slot function is named `Class_vNN` (NN = slot
   index) only if no other identified vtable contains it. It is named `Class_dtor` if it is
   short and resets the vptr to the class's own primary vtable before chaining to the base
   dtor. Two clean-up passes reverted 20 names: 11 "dtors" that reset a *member*
   object's vtable (GameModule embeds about 10 objects with vptrs), and 9 slots that came
   from a member vtable rather than the primary one.
   - Note: in many FE classes, slot 1 (`_v01`) is the implicit destructor. gcc folded the
     derived class's dead vptr store into the inlined base dtor, so these functions set a
     *base* vtable (for example `0x46d0d0`, a shared `cFEState`-like base). They are
     placed together at 0x1dd800 to 0x1e1000, far from the class code. They were kept as
     `_v01` rather than `_dtor`.
   - `Class_vNN` means "slot NN of Class's vtable". A method may really be inherited from a
     base class that hasn't been identified (the uniqueness test only covers the ~150
     vtables found).

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

## Reproducing

The working scripts used scratch dumps and were not committed. To redo the pass, dump
`/strings` + `/xrefs_to` and `/disassemble_function` for every function, then apply the
allocation-to-ctor and vtable rules above. The rules are mechanical and take about 100
lines of Python.

## Appendix: every rename from this pass

```
0041605c	memcpy	libc
0041610c	memmove	libc
00416210	memset	libc
004162d0	strcat	libc
00416404	strchr	libc
004165a8	strcmp	libc
004166f4	strcpy	libc
00416810	strlen	libc
00416958	strncat	libc
00416b18	strncmp	libc
00416ce0	strncpy	libc
00417578	_printf_r	libc
004175c8	printf	libc
004177b0	_sprintf_r	libc
00417828	sprintf	libc
004186c8	vsprintf	libc
00418720	vsnprintf	libc
0041b658	vfprintf_dispatch	libc
0041b728	_vfprintf_r	libc
0041ca70	_vfiprintf_r	libc
0041aa88	strcasecmp	libc
0041ab08	strcspn	libc
0041ab78	strncasecmp	libc
0041ac40	strpbrk	libc
0041acc0	strrchr	libc
0041ad10	strspn	libc
0041ad80	strstr	libc
0041ea18	tolower	libc
004188c0	malloc	libc
00418910	free	libc
00418ca8	__malloc_lock	libc
00418d08	__malloc_unlock	libc
0041dbf8	_malloc_r	libc
0041e340	_free_r	libc
004187e8	bsearch	libc
004187c0	atoi	libc
0041e9e0	strtol	libc
0041e7a8	_strtol_r	libc
00415fb8	__getreent	libc
00415fc8	memcmp	libc
00413af8	__extendsfdf2	libc
004114d0	__muldi3	libc
0041ec70	memchr	libc
0041fd70	_sbrk_r	libc
00418ef8	qsort	libc
00412e80	__tf13bad_exception	libc
00412f18	__tf9exception	libc
00414f00	__tf17__class_type_info	libc
00414f90	__tf14__si_type_info	libc
00415018	__tf16__user_type_info	libc
004150d0	__tf10bad_typeid	libc
00415168	__tf8bad_cast	libc
004151f8	__tf9type_info	libc
00415ca8	__tf17__array_type_info	libc
00415d18	__tf16__ptmd_type_info	libc
00415d88	__tf16__ptmf_type_info	libc
00415df8	__tf16__func_type_info	libc
00415e68	__tf19__builtin_type_info	libc
00415ed8	__tf16__attr_type_info	libc
00415f48	__tf19__pointer_type_info	libc
004187a0	atof	libc
0041a7e8	strtod	libc
004198d8	_strtod_r	libc
0041a818	strtof	libc
00421f20	_realloc_r	libc
004127a8	__truncdfsf2	libc
00422780	_dtoa_r	libc
00420510	_Balloc	libc
004205b8	_Bfree	libc
004205f0	__multadd	libc
004206f8	__s2b	libc
00420830	__hi0bits	libc
004208b8	__lo0bits	libc
00420978	__i2b	libc
004209b0	__multiply	libc
00420bc8	__pow5mult	libc
00420cc8	__lshift	libc
00420e28	__mcmp	libc
00420e90	__mdiff	libc
00421020	__ulp	libc
004210e8	__b2d	libc
00421280	__d2b	libc
00421400	__ratio	libc
004224a0	_calloc_r	libc
0040d588	tanf	libm
0040f5a8	__kernel_tanf	libm
0040e878	__ieee754_rem_pio2f	libm
0040ec58	__kernel_rem_pio2f	libm
0040d758	expf	libm
0040e048	__ieee754_expf	libm
0040f8c0	finitef	libm
0040da10	logf	libm
0040e550	__ieee754_logf	libm
0040f9d0	isnanf	libm
0040d8d0	fmodf	libm
0040e300	__ieee754_fmodf	libm
0040d610	log	libm
0040db58	__ieee754_log	libm
0040f840	isnan	libm
0040f878	matherr	libm
0040f8a0	fabsf	libm
0040f8e8	floorf	libm
0040f9f8	scalbnf	libm
0040fb58	copysignf	libm
00426358	sceSifWriteBackDCache	libkernl
00426f08	sceSifCheckStatRpc	libkernl
004261f0	isceSifSendCmd	libkernl
00426078	_sceSifSendCmd	libkernl
00425fb8	sceSifExitCmd	libkernl
00424698	SyncDCache	libkernl
004245f0	_SyncDCache	libkernl
004247d8	InvalidDCache	libkernl
00424730	_InvalidDCache	libkernl
00425cf0	sceSifGetSreg	libkernl
00425ff0	sceSifSetCmdBuffer	libkernl
00426050	sceSifRemoveCmdHandler	libkernl
00425c60	scePrintf	libkernl
00424a20	iEnableIntc	libkernl
00424a40	iDisableIntc	libkernl
00424a60	iEnableDmac	libkernl
00424a80	iDisableDmac	libkernl
00101310	DangerMan_ctor	ctor
00108c80	ComputerActorRider_ctor	ctor
00113138	AirPredictor_ctor	ctor
00117248	RiderMetrix_ctor	ctor
00125c70	Player_ctor	ctor
00145930	BENewPlayerInt_ctor	ctor
00149860	BELibrary_ctor	ctor
0014f418	BEOptionInt_ctor	ctor
001549a8	BEScoreInt_ctor	ctor
0015c930	AlgorithmList_ctor	ctor
0015cd80	ManualCamController_ctor	ctor
0015dc70	GameCamera_ctor	ctor
0015fd08	ManualCamAlgo_ctor	ctor
0015ffb0	EditorCamAlgo_ctor	ctor
00160438	ManualChaseCameraAlgorithm_ctor	ctor
00160760	MatrixChaseCameraAlgorithm_ctor	ctor
00160b70	RelativeChaseCameraAlgorithm_ctor	ctor
001616a8	IdleChaseCameraAlgorithm_ctor	ctor
001619a8	ChaseCamController_ctor	ctor
001682b8	LazyChaseCameraAlgorithm_ctor	ctor
00168650	LookBackChaseCameraAlgorithm_ctor	ctor
001689c8	MidRightChaseCameraAlgorithm_ctor	ctor
00168cd8	MidLeftChaseCameraAlgorithm_ctor	ctor
00168fe8	ScriptCamController_ctor	ctor
001694b8	ManualScriptCameraAlgo_ctor	ctor
00169cf8	TargetScriptCameraAlgo_ctor	ctor
0016a458	SubjectScriptCameraAlgo_ctor	ctor
0016ad38	AlgorithmicScriptCameraAlgo_ctor	ctor
00176a68	Default_2ChaseCameraAlgorithm_ctor	ctor
00176d68	Default_3ChaseCameraAlgorithm_ctor	ctor
00177068	Default_4ChaseCameraAlgorithm_ctor	ctor
001774c0	DirectionNeChaseCameraAlgorithm_ctor	ctor
00177e50	SpokeChaseCameraAlgorithm_ctor	ctor
001789e8	Post_Race_1ChaseCameraAlgorithm_ctor	ctor
0017b1b0	DebugMenu_ctor	ctor
0017d298	LobbyMenu_ctor	ctor
00181090	cFEStateCharSelect_ctor	ctor
00182220	cFEStateCheatCharSelect_ctor	ctor
001828c0	cFEStateCharSetup_ctor	ctor
00183238	cFEStateRiderDetail_ctor	ctor
00183a98	cFEStateUberTrick_ctor	ctor
00185a18	cFEOptionsCredits_ctor	ctor
00186610	cFEStateEventSelect_ctor	ctor
00186a08	cFEStateMemCard_ctor	ctor
001887a0	cFEStateOptions_ctor	ctor
00188ee8	cFEStateOptionsGame_ctor	ctor
0018a258	cFEStateOptionsSound_ctor	ctor
0018c0e8	cFEStateOptionsHUD_ctor	ctor
0018c658	cFEStateOptionsController_ctor	ctor
0018d240	cFEStateOptionsSaveLoad_ctor	ctor
0018d7f0	cFEStateOptionsDeviceSelect_ctor	ctor
0018eca0	cFEStateProfileLoad_ctor	ctor
00190858	cFEStateRiderBio_ctor	ctor
00191360	cFEStateRuleSelect_ctor	ctor
00192d00	cFEStateBraggingRights_ctor	ctor
00194738	cFEStateTitle_ctor	ctor
00194a60	cFEStateMainMenu_ctor	ctor
00195498	cFEStateBonusMaterial_ctor	ctor
00195880	FELocale_ctor	ctor
00195ff0	cFEStateAudioOptions_ctor	ctor
00196b90	cFEStateRequestLine_ctor	ctor
00198db8	cFEStateCharEquip_ctor	ctor
0019cb38	BoltFileIndexManager_ctor	ctor
001a0720	FEUICustom_ctor	ctor
001a1ce8	FEModule_ctor	ctor
001a2e20	FEUISound_ctor	ctor
001a2f38	OVUISound_ctor	ctor
001a3218	cFEStateMPCircuit_ctor	ctor
001a3590	FEAsyncManager_ctor	ctor
001a3dc8	cFEStateNetMessenger_ctor	ctor
001a77a0	cFEStateReadMessagePopup_ctor	ctor
001ac010	cFEUseExistingAccount_ctor	ctor
001ad130	cFEStateSelectPersona_ctor	ctor
001addd0	cFEStateChallengePopup_ctor	ctor
001afe08	cFEStateChallengeReceivedPopup_ctor	ctor
001b0f60	cFEStateOnlineCharacterSelect_ctor	ctor
001b1f88	cFEStateChatRoom_ctor	ctor
001b3f78	cFEStateNetConfig_ctor	ctor
001b6288	NetConfigAuthPopup_ctor	ctor
001b6ab8	cFEStateAuthDNASError_ctor	ctor
001b6ed0	cFEStateNetAccountMem_ctor	ctor
001b88f0	cFEStateCreateStep1_ctor	ctor
001ba3e0	cFEStateCreateStep2_ctor	ctor
001bbd08	cFEStateCreateDuplicate_ctor	ctor
001bcb58	cFEStateLobbyMenu_ctor	ctor
001be7d8	cFEStateNetCreateRoomPopup_ctor	ctor
001bfbb8	cFEStateOnlineMainMenu_ctor	ctor
001c0418	cFEStateOnlineAgreement_ctor	ctor
001c0f40	cFEStateNetQuickPlayPopup_ctor	ctor
001c1c50	cFEStateNetSSXWorld_ctor	ctor
001c20f8	cFEStateNetQuickMessages_ctor	ctor
001c2d18	cFEStateStats_ctor	ctor
001c5018	cFEStateNetNews_ctor	ctor
001caba8	cUIStateBuyPopup_ctor	ctor
001cb030	cKeyboardPopup_ctor	ctor
001d1128	cFEStateRewardGalleryPosters_ctor	ctor
001d12c8	cFEStateRewardGalleryTradingCards_ctor	ctor
001d1460	cFEStateRewardGalleryConceptArts_ctor	ctor
001d15f8	cFEStateRewardGalleryVideos_ctor	ctor
001d1790	cFEStateRewardGalleryToys_ctor	ctor
001d1928	cFEStateRewardGalleryCheatChars_ctor	ctor
001d1aa0	cFEStatePreviewReward_ctor	ctor
001d22f0	cFEStateVideoPlayer_ctor	ctor
001d2678	cFEStateMountainRoom_ctor	ctor
001d3780	cFEStatePeakRoom_ctor	ctor
001d4268	cFEStateTrophyRoom_ctor	ctor
001d4bc8	cFEStateRewardsRoom_ctor	ctor
001d5038	cFEStateRelogin_ctor	ctor
001d99c8	cFEPopupConfirm_ctor	ctor
001d9d48	cFEStateNetHelpPopup_ctor	ctor
001e51d0	cOVState_ViewPopup_ctor	ctor
001e9a30	cOVStateHUD1P_ctor	ctor
001f30c0	LoadIndicator_ctor	ctor
001f3700	cFEStateLodge_ctor	ctor
001f3ff8	cFEStateLodgeRiderDetail_ctor	ctor
001f4728	cFEStateBuyAttrib_ctor	ctor
001f54b0	cFEStateCareerStats_ctor	ctor
00224da0	cLunoTable_ctor	ctor
00229180	CrowdMan2d_ctor	ctor
0022cce8	SectionMan_ctor	ctor
0022e3b8	GameViewMan_ctor	ctor
0022e968	GameModule_ctor	ctor
00232720	cPreGameLoadScreen_ctor	ctor
00232738	cPreFELoadScreen_ctor	ctor
00237060	InputSync_ctor	ctor
00242288	MCManager_ctor	ctor
00242500	OcclusionMan_ctor	ctor
002533c8	VidEnginePlayer_ctor	ctor
002558c0	cCommSystem_ctor	ctor
002561b8	cFEStateAuthDNASCreate_ctor	ctor
00256698	cDSockChannel_ctor	ctor
00256c78	cGameComm_ctor	ctor
002594d0	LobbyMan_ctor	ctor
002640c0	cSetupMan_ctor	ctor
002658e0	cSharedLoginMan_ctor	ctor
002667e8	cUsbKeyboard_ctor	ctor
00266d00	cVoipMan_ctor	ctor
0026c458	PlaybackMan_ctor	ctor
0026d0a0	ReplayCache_ctor	ctor
0026efb8	Replay_ctor	ctor
00272788	InSeq_ctor	ctor
00277f20	ScriptEngine_ctor	ctor
0027d5e8	ScriptObjCam_ctor	ctor
00284c68	SSXAudioSystem_ctor	ctor
002a8c20	PS2SoundManager_ctor	ctor
002af8e0	SpeechBank_ctor	ctor
002afa68	SpeechInstance_ctor	ctor
002b1a98	cSongConfiguration_ctor	ctor
002b1c50	SongInstance_ctor	ctor
002b58d0	WorldTriggerManager_ctor	ctor
002b6aa0	cWorldSound_ctor	ctor
002bb130	cWorldPainterMan_ctor	ctor
002bc6f8	tWPIGD_MusicTrigger_ctor	ctor
002bc728	tWPIGD_Mix_ctor	ctor
002bc750	tWPIGD_Ambience_ctor	ctor
002bc778	tWPIGD_Speech_ctor	ctor
002bc7a0	tWPIGD_Camera_ctor	ctor
002bc7c8	tWPIGD_Fog_ctor	ctor
002bc830	tWPIGD_LightGlow_ctor	ctor
002bc890	tWPIGD_ScreenTint_ctor	ctor
002bc8e8	tWPIGD_SkyBox_ctor	ctor
002bc910	tWPIGD_Sun_ctor	ctor
002bc980	tWPIGD_Surface_ctor	ctor
002bc9b0	tWPIGD_Lighting_ctor	ctor
002bcaf8	tWPIGD_Weather_ctor	ctor
002bcbd0	tWPIGD_Danger_ctor	ctor
002c0408	WorldPainterQuery_ctor	ctor
002c3fa8	cPS2MemCard_ctor	ctor
002c7478	cPS2ConsoleConfig_ctor	ctor
002cc018	MenuSys_ctor	ctor
002d21c0	SSXStatsMan_ctor	ctor
002db6d8	cCrowdRender2D_ctor	ctor
002db9c8	cCrowdAnim2D_ctor	ctor
002dbd10	FXLoader_ctor	ctor
002e4228	cTransitionMan_ctor	ctor
002e5920	SnowFall_ctor	ctor
002ea670	cCutSceneTransitionMan_ctor	ctor
002ec418	LensFX_ctor	ctor
002ec9e0	cBoltOnFXInterface_ctor	ctor
002eca68	cBoltOnFXInterface_ctor_2	ctor
002f03c8	PS2AIVisualEffectsMan_ctor	ctor
002f0548	PS2VisualEffectsMan_ctor	ctor
002f1768	cAttachedDynamicEmitter_ctor	ctor
002f1850	AttachedEmitterFX_ctor	ctor
002f3550	cSplashManager_ctor	ctor
002f5180	WorldLightMan_ctor	ctor
002f7a68	BoardSparkEmitter_ctor	ctor
00307128	WScriptMission_ctor	ctor
00307598	WScriptProcess_ctor	ctor
003087d0	WScriptMan_ctor	ctor
0030d4b8	RiderModel_ctor	ctor
00311048	AnimDataBase_ctor	ctor
00311220	cAdfDataSource_ctor	ctor
00311250	AdfDataSource_ctor	ctor
003115e0	AIAnimEventMap_ctor	ctor
00313508	AnimSequence_ctor	ctor
00314d98	GameAnimMap_ctor	ctor
00316898	BigFile_ctor	ctor
00321428	InputParser_ctor	ctor
00326df0	PS2Pad_ctor	ctor
00327810	AIFwdDiffCache_ctor	ctor
00341388	Boost_ctor	ctor
00341548	Boost_ctor_2	ctor
00341aa0	AnimObject_ctor	ctor
00341c80	AnimObject_ctor_2	ctor
003421a0	AnimTeeter_ctor	ctor
003422e8	AnimTeeter_ctor_2	ctor
00342768	BoostModifier_ctor	ctor
00342808	BoostModifier_ctor_2	ctor
00342d10	Debounce_ctor	ctor
00343010	Conveyor_ctor	ctor
003430d0	Conveyor_ctor_2	ctor
00343768	Counter_ctor	ctor
003437c0	Counter_ctor_2	ctor
00343a68	CrowdMan3d_ctor	ctor
00345c90	DynamicParticleModifier_ctor	ctor
00345d80	DynamicParticleModifier_ctor_2	ctor
00346120	HaloModifier_ctor	ctor
003461c0	HaloModifier_ctor_2	ctor
00348d98	FloatRail_ctor	ctor
00348fa0	FloatRail_ctor_2	ctor
00349b48	SpringRail_ctor	ctor
00349db0	SpringRail_ctor_2	ctor
0034c428	cFlagManager_ctor	ctor
0034cab8	Fence_ctor	ctor
0034cbe8	Fence_ctor_2	ctor
0034ec58	Floating_ctor	ctor
0034ed88	Floating_ctor_2	ctor
003506d8	DeadNode_ctor	ctor
00350b98	DeadFade_ctor	ctor
00351030	InstancesMan_ctor	ctor
00352168	MeshAnim_ctor	ctor
00352aa8	ModifierBlock_ctor	ctor
00353ac0	SkyBox_ctor	ctor
00356db0	Object_ctor	ctor
00356e60	Object_ctor_2	ctor
00356eb8	PositionModifier_ctor	ctor
00356f10	PositionModifier_ctor_2	ctor
00356f68	PositionModifier_ctor_3	ctor
00357038	ParentModifier_ctor	ctor
00357090	ParentModifier_ctor_2	ctor
003572e0	MagnetModifier_ctor	ctor
00357358	MagnetModifier_ctor_2	ctor
00357750	AvaSplineModifier_ctor	ctor
00357798	AvaSplineModifier_ctor_2	ctor
00357918	Particle_ctor	ctor
003579c8	MultiParticleMan_ctor	ctor
003584f0	RailMan_ctor	ctor
00358588	RailMan_ctor_2	ctor
00358c80	AnimCombo_ctor	ctor
00358db0	AnimCombo_ctor_2	ctor
00359228	AnimDelta_ctor	ctor
00359270	AnimDelta_ctor_2	ctor
00359460	SplineModifier_ctor	ctor
003595d8	SplineModifier_ctor_2	ctor
00359f88	MultiSplineModifier_ctor	ctor
0035a118	MultiSplineModifier_ctor_2	ctor
0035b708	RailModifier_ctor	ctor
0035ba88	RailModifier_ctor_2	ctor
0035da70	RollerModifier_ctor	ctor
0035df70	RollerModifier_ctor_2	ctor
00366fe8	TexMan_ctor	ctor
00370018	Emitter_ctor	ctor
003714b8	mEmitter_ctor	ctor
00372aa0	PathArrow_ctor	ctor
003758f8	BoaderShadow_ctor	ctor
003759e8	DetailSystem_ctor	ctor
00385138	cPS2GridMesh_ctor	ctor
0038ae60	PS2BezierMan_ctor	ctor
00394d50	DeferMan_ctor	ctor
00395288	PS2GraphicsMan_ctor	ctor
003972a0	UIAnim_ctor	ctor
00397b08	UIEngine_ctor	ctor
00399768	UIListBox_ctor	ctor
003a0348	UIText_ctor	ctor
003a1148	cUITextScroll_ctor	ctor
003a3280	UIVector_ctor	ctor
003a6688	World_ctor	ctor
003a77b0	SectionAllocator_ctor	ctor
003a7a20	WorldMemman_ctor	ctor
003a7bd8	cHullPage_ctor	ctor
003a7c88	SectionTable_ctor	ctor
003a7dd0	cWorldCache_ctor	ctor
003ac7c8	cWScriptCache_ctor	ctor
0015ce10	ManualCamController_dtor	vtable
0015ce98	ManualCamController_v03	vtable
0015cee8	ManualCamController_v04	vtable
0015cfe8	ManualCamController_v02	vtable
00160478	ManualChaseCameraAlgorithm_v01	vtable
001604b8	ManualChaseCameraAlgorithm_v05	vtable
001606d8	ManualChaseCameraAlgorithm_v03	vtable
001607a0	MatrixChaseCameraAlgorithm_v01	vtable
001607e0	MatrixChaseCameraAlgorithm_v05	vtable
00160ae8	MatrixChaseCameraAlgorithm_v03	vtable
00160bb8	RelativeChaseCameraAlgorithm_v01	vtable
00160bf8	RelativeChaseCameraAlgorithm_v05	vtable
00160fd0	RelativeChaseCameraAlgorithm_v03	vtable
001616e8	IdleChaseCameraAlgorithm_v01	vtable
00161728	IdleChaseCameraAlgorithm_v05	vtable
00161848	IdleChaseCameraAlgorithm_v03	vtable
00161a10	ChaseCamController_dtor	vtable
00161bb8	ChaseCamController_v02	vtable
00167f10	ManualCamAlgo_v01	vtable
00167f40	EditorCamAlgo_v01	vtable
001682f8	LazyChaseCameraAlgorithm_v01	vtable
00168338	LazyChaseCameraAlgorithm_v05	vtable
00168508	LazyChaseCameraAlgorithm_v03	vtable
00168690	LookBackChaseCameraAlgorithm_v01	vtable
001686d0	LookBackChaseCameraAlgorithm_v05	vtable
00168940	LookBackChaseCameraAlgorithm_v03	vtable
00168a08	MidRightChaseCameraAlgorithm_v01	vtable
00168a70	MidRightChaseCameraAlgorithm_v05	vtable
00168c50	MidRightChaseCameraAlgorithm_v03	vtable
00168d18	MidLeftChaseCameraAlgorithm_v01	vtable
00168d80	MidLeftChaseCameraAlgorithm_v05	vtable
00168f60	MidLeftChaseCameraAlgorithm_v03	vtable
001690d0	ScriptCamController_dtor	vtable
00169368	ScriptCamController_v02	vtable
00176aa8	Default_2ChaseCameraAlgorithm_v01	vtable
00176b10	Default_2ChaseCameraAlgorithm_v05	vtable
00176ce0	Default_2ChaseCameraAlgorithm_v03	vtable
00176da8	Default_3ChaseCameraAlgorithm_v01	vtable
00176e10	Default_3ChaseCameraAlgorithm_v05	vtable
00176fe0	Default_3ChaseCameraAlgorithm_v03	vtable
001770a8	Default_4ChaseCameraAlgorithm_v01	vtable
00177110	Default_4ChaseCameraAlgorithm_v05	vtable
001772e0	Default_4ChaseCameraAlgorithm_v03	vtable
00177368	DirectionNeChaseCameraAlgorithm_v05	vtable
00177520	DirectionNeChaseCameraAlgorithm_v01	vtable
00177588	DirectionNeChaseCameraAlgorithm_v03	vtable
00177e90	SpokeChaseCameraAlgorithm_v01	vtable
00178208	SpokeChaseCameraAlgorithm_v05	vtable
001783e0	SpokeChaseCameraAlgorithm_v03	vtable
00178b70	Post_Race_1ChaseCameraAlgorithm_v01	vtable
00178bb0	Post_Race_1ChaseCameraAlgorithm_v05	vtable
00178e90	Post_Race_1ChaseCameraAlgorithm_v03	vtable
00181148	cFEStateCharSelect_v04	vtable
00181238	cFEStateCharSelect_v05	vtable
001812a8	cFEStateCharSelect_v07	vtable
00181308	cFEStateCharSelect_v06	vtable
00181450	cFEStateCharSelect_v08	vtable
00182420	cFEStateCheatCharSelect_v04	vtable
00182928	cFEStateCharSetup_v04	vtable
001832a8	cFEStateRiderDetail_v04	vtable
001833d0	cFEStateRiderDetail_v06	vtable
001849b0	cFEStateUberTrick_v04	vtable
00184b70	cFEStateUberTrick_v05	vtable
00184bb8	cFEStateUberTrick_v07	vtable
00184be0	cFEStateUberTrick_v06	vtable
00185a70	cFEOptionsCredits_v01	vtable
00185a98	cFEOptionsCredits_v04	vtable
00185ba0	cFEOptionsCredits_v06	vtable
00186658	cFEStateEventSelect_v04	vtable
00186728	cFEStateEventSelect_v07	vtable
00186748	cFEStateEventSelect_v06	vtable
00186778	cFEStateEventSelect_v05	vtable
00186978	cFEStateEventSelect_v08	vtable
00186998	cFEStateEventSelect_v09	vtable
00186b18	cFEStateMemCard_dtor	vtable
00186b50	cFEStateMemCard_v04	vtable
00188800	cFEStateOptions_v04	vtable
00188f28	cFEStateOptionsGame_v04	vtable
001895f0	cFEStateOptionsGame_v06	vtable
0018a298	cFEStateOptionsSound_v04	vtable
0018a870	cFEStateOptionsSound_v06	vtable
0018c128	cFEStateOptionsHUD_v04	vtable
0018c198	cFEStateOptionsHUD_v06	vtable
0018c6a0	cFEStateOptionsController_v04	vtable
0018c8b8	cFEStateOptionsController_v05	vtable
0018d280	cFEStateOptionsSaveLoad_v04	vtable
0018d868	cFEStateOptionsDeviceSelect_v04	vtable
0018edc8	cFEStateProfileLoad_v04	vtable
0018fc70	cFEStateProfileLoad_v05	vtable
001908c0	cFEStateRiderBio_v04	vtable
00191938	cFEStateRuleSelect_v04	vtable
00191c48	cFEStateRuleSelect_v06	vtable
00192d80	cFEStateBraggingRights_v04	vtable
00193568	cFEStateBraggingRights_v06	vtable
00194588	FEUICustom_v04	vtable
00194778	cFEStateTitle_v04	vtable
001947f8	cFEStateTitle_v07	vtable
00194aa8	cFEStateMainMenu_v04	vtable
001954d8	cFEStateBonusMaterial_v04	vtable
001955e0	cFEStateBonusMaterial_v06	vtable
001958f0	FELocale_v01	vtable
00195aa0	FELocale_v03	vtable
00195eb8	FELocale_v04	vtable
00195f70	FELocale_v05	vtable
00196148	cFEStateAudioOptions_v04	vtable
00196228	cFEStateAudioOptions_v05	vtable
001962a8	cFEStateAudioOptions_v07	vtable
00196378	cFEStateAudioOptions_v06	vtable
00196db0	cFEStateRequestLine_v04	vtable
00196e80	cFEStateRequestLine_v05	vtable
00196f00	cFEStateRequestLine_v06	vtable
00196f50	cFEStateRequestLine_v07	vtable
00199148	cFEStateCharEquip_v01	vtable
00199240	cFEStateCharEquip_v04	vtable
00199350	cFEStateCharEquip_v05	vtable
001993a0	cFEStateCharEquip_v06	vtable
001a2e58	FEUISound_v02	vtable
001a2f70	OVUISound_v02	vtable
001a3258	cFEStateMPCircuit_v04	vtable
001a3308	cFEStateMPCircuit_v06	vtable
001a3ea0	cFEStateNetMessenger_v01	vtable
001a3f68	cFEStateNetMessenger_v04	vtable
001a4170	cFEStateNetMessenger_v08	vtable
001ac068	cFEUseExistingAccount_dtor	vtable
001ac0b8	cFEUseExistingAccount_v04	vtable
001ad178	cFEStateSelectPersona_v04	vtable
001ade88	cFEStateChallengePopup_v04	vtable
001adfb8	cFEStateChallengePopup_v07	vtable
001adfe8	cFEStateChallengePopup_v06	vtable
001b0fb0	cFEStateOnlineCharacterSelect_v04	vtable
001b1068	cFEStateOnlineCharacterSelect_v07	vtable
001b1590	cFEStateOnlineCharacterSelect_v08	vtable
001b2048	cFEStateChatRoom_v01	vtable
001b2070	cFEStateChatRoom_v04	vtable
001b3490	cFEStateChatRoom_v07	vtable
001b3528	cFEStateChatRoom_v06	vtable
001b38d0	cFEStateChatRoom_v08	vtable
001b4018	cFEStateNetConfig_dtor	vtable
001b40d0	cFEStateNetConfig_v04	vtable
001b4168	cFEStateNetConfig_v05	vtable
001b6368	NetConfigAuthPopup_dtor	vtable
001b63e8	NetConfigAuthPopup_v04	vtable
001b6b28	cFEStateAuthDNASError_v01	vtable
001b6b50	cFEStateAuthDNASError_v04	vtable
001b6c98	cFEStateAuthDNASError_v07	vtable
001b7030	cFEStateNetAccountMem_v01	vtable
001b7088	cFEStateNetAccountMem_v04	vtable
001b8960	cFEStateCreateStep1_v04	vtable
001b8a08	cFEStateCreateStep1_v07	vtable
001ba428	cFEStateCreateStep2_v04	vtable
001bbd60	cFEStateCreateDuplicate_v04	vtable
001bcc30	cFEStateLobbyMenu_v04	vtable
001bcd18	cFEStateLobbyMenu_v07	vtable
001be828	cFEStateNetCreateRoomPopup_v04	vtable
001bfbf8	cFEStateOnlineMainMenu_v04	vtable
001c04a8	cFEStateOnlineAgreement_v04	vtable
001c0a28	cFEStateOnlineAgreement_dtor	vtable
001c0b20	cFEStateOnlineAgreement_v07	vtable
001c0fc0	cFEStateNetQuickPlayPopup_v04	vtable
001c1c90	cFEStateNetSSXWorld_v04	vtable
001c2138	cFEStateNetQuickMessages_v04	vtable
001c23c0	cFEStateNetQuickMessages_v07	vtable
001c2e00	cFEStateStats_v04	vtable
001c32b8	cFEStateStats_v07	vtable
001c5098	cFEStateNetNews_v04	vtable
001c5198	cFEStateNetNews_v07	vtable
001c5af8	cFEPopupConfirm_v04	vtable
001c5b68	cFEPopupConfirm_v05	vtable
001c5b88	cFEPopupConfirm_v06	vtable
001c5c30	cFEPopupConfirm_v07	vtable
001cb138	cKeyboardPopup_dtor	vtable
001cb1a0	cKeyboardPopup_v04	vtable
001cd2f0	cKeyboardPopup_v06	vtable
001cd348	cKeyboardPopup_v07	vtable
001cdb98	cKeyboardPopup_v08	vtable
001d1b68	cFEStatePreviewReward_v01	vtable
001d1c08	cFEStatePreviewReward_v04	vtable
001d2380	cFEStateVideoPlayer_dtor	vtable
001d23e0	cFEStateVideoPlayer_v04	vtable
001d2430	cFEStateVideoPlayer_v05	vtable
001d28f8	cFEStateMountainRoom_dtor	vtable
001d2990	cFEStateMountainRoom_v04	vtable
001d2a90	cFEStateMountainRoom_v06	vtable
001d3890	cFEStatePeakRoom_v04	vtable
001d3c60	cFEStatePeakRoom_v06	vtable
001d4368	cFEStateTrophyRoom_v04	vtable
001d43d0	cFEStateTrophyRoom_v06	vtable
001d4c60	cFEStateRewardsRoom_v04	vtable
001d5078	cFEStateRelogin_v04	vtable
001d9db8	cFEStateNetHelpPopup_v04	vtable
001da238	cFEStateNetHelpPopup_v07	vtable
001dcc08	FELocale_v02	vtable
001dd840	cFEStateCharSelect_v01	vtable
001dd868	cFEStateCheatCharSelect_v01	vtable
001dd8a0	cFEStateCharSetup_v01	vtable
001dd8c8	cFEStateRiderDetail_v01	vtable
001dd8f0	cFEStateUberTrick_v01	vtable
001ddeb8	cFEPopupConfirm_v01	vtable
001ddf20	cFEStateOptions_v01	vtable
001ddf80	cFEStateOptionsGame_v01	vtable
001ddfe8	cFEStateOptionsSound_v01	vtable
001de050	cFEStateOptionsHUD_v01	vtable
001de0c0	cFEStateOptionsController_v01	vtable
001de128	cFEStateOptionsSaveLoad_v01	vtable
001de150	cFEStateOptionsDeviceSelect_v01	vtable
001de218	cFEStateProfileLoad_v01	vtable
001de528	cFEStateEventSelect_v01	vtable
001de668	FEUICustom_v01	vtable
001de6c8	cFEStateTitle_v01	vtable
001de728	cFEStateMainMenu_v01	vtable
001de788	cFEStateBonusMaterial_v01	vtable
001df3b8	cFEStateRiderBio_v01	vtable
001df418	cFEStateRuleSelect_v01	vtable
001df4e8	cFEStateBraggingRights_dtor	vtable
001df608	cFEStateOnlineMainMenu_v01	vtable
001df678	cFEStateNetQuickPlayPopup_v01	vtable
001df6e8	cFEStateNetSSXWorld_v01	vtable
001df750	cFEStateNetQuickMessages_v01	vtable
001df7c8	cFEStateLobbyMenu_dtor	vtable
001df878	cFEStateNetCreateRoomPopup_dtor	vtable
001df960	cFEStateNetHelpPopup_v01	vtable
001dfb28	cFEStateRewardGalleryPosters_v01	vtable
001dfbf8	cFEStateRewardGalleryTradingCards_v01	vtable
001dfcc8	cFEStateRewardGalleryConceptArts_v01	vtable
001dfd98	cFEStateRewardGalleryVideos_v01	vtable
001dfe68	cFEStateRewardGalleryToys_v01	vtable
001dff38	cFEStateRewardGalleryCheatChars_v01	vtable
001e01b0	cFEStatePeakRoom_v01	vtable
001e0210	cFEStateTrophyRoom_v01	vtable
001e02a0	cFEStateRewardsRoom_v01	vtable
001e0548	cFEStateAudioOptions_v01	vtable
001e05b0	cFEStateRequestLine_v01	vtable
001e0740	FEUISound_v01	vtable
001e0780	OVUISound_v01	vtable
001e0858	cFEStateMPCircuit_v01	vtable
001e0918	cFEStateSelectPersona_v01	vtable
001e09a0	cFEStateChallengePopup_dtor	vtable
001e0b00	cFEStateOnlineCharacterSelect_v01	vtable
001e0cf8	cFEStateCreateStep1_dtor	vtable
001e0da8	cFEStateCreateStep2_v01	vtable
001e0e18	cFEStateCreateDuplicate_v01	vtable
001e0e98	cFEStateStats_v01	vtable
001e0ec8	cFEStateNetNews_v01	vtable
001e0f30	cFEStateRelogin_v01	vtable
001e0fc0	cFEStateNetMessenger_v07	vtable
001e5248	cOVState_ViewPopup_v04	vtable
001e52d0	cOVState_ViewPopup_v07	vtable
001e9ad0	cOVStateHUD1P_v04	vtable
001f4050	cFEStateLodgeRiderDetail_v04	vtable
001f4830	cFEStateBuyAttrib_v04	vtable
001f49d0	cFEStateBuyAttrib_v07	vtable
001f49f8	cFEStateBuyAttrib_v06	vtable
001f5508	cFEStateCareerStats_v04	vtable
001f55c0	cFEStateCareerStats_v07	vtable
001f55e8	cFEStateCareerStats_v06	vtable
001f56c8	cFEStateCareerStats_v08	vtable
0021cb40	cOVState_ViewPopup_v01	vtable
0021cc40	cOVStateHUD1P_dtor	vtable
00220dd0	cFEStateLodgeRiderDetail_v01	vtable
00220e30	cFEStateBuyAttrib_v01	vtable
00220e90	cFEStateCareerStats_v01	vtable
00232750	cPreGameLoadScreen_v01	vtable
00232780	cPreFELoadScreen_v01	vtable
00232b28	StartScreen_v02	vtable
00232c98	StartScreen_v05	vtable
00232cf8	StartScreen_v04	vtable
00232d18	StartScreen_v06	vtable
00232d50	StartScreen_v03	vtable
00232da8	StartScreen_v07	vtable
00243a80	PreStartScreen_v01	vtable
00243b50	StartScreen_v01	vtable
002566e8	cDSockChannel_v01	vtable
00256998	cDSockChannel_v02	vtable
00256a00	cDSockChannel_v03	vtable
002780b8	ScriptEngine_v01	vtable
00278720	ScriptEngine_v05	vtable
00278ad0	ScriptEngine_v06	vtable
002797c8	ScriptEngine_v07	vtable
00279840	ScriptEngine_v08	vtable
00279888	ScriptEngine_v09	vtable
00279a70	ScriptEngine_v10	vtable
00279d00	ScriptEngine_v02	vtable
00279da8	ScriptEngine_v03	vtable
00279e50	ScriptEngine_v04	vtable
0027d7f0	ScriptObjCam_v02	vtable
0027d820	ScriptObjCam_v03	vtable
0027d850	ScriptObjCam_v04	vtable
0027d8d8	ScriptObjCam_v05	vtable
00284a28	ScriptObjCam_v01	vtable
002a8cb0	PS2SoundManager_v01	vtable
002a8d90	PS2SoundManager_v02	vtable
002a8f50	PS2SoundManager_v03	vtable
002a8fd8	PS2SoundManager_v04	vtable
002b1b68	cSongConfiguration_dtor	vtable
002c06c0	WorldPainterQuery_v01	vtable
002c0778	WorldPainterQuery_v02	vtable
002e46c8	cTransitionMan_v02	vtable
002e5d18	SnowFall_v01	vtable
002e5da0	SnowFall_v02	vtable
002ea900	cCutSceneTransitionMan_v02	vtable
002ec450	LensFX_v01	vtable
002ecb28	cBoltOnFXInterface_dtor	vtable
002f0438	PS2AIVisualEffectsMan_v01	vtable
002f04b0	PS2AIVisualEffectsMan_v02	vtable
002f04e0	PS2AIVisualEffectsMan_v03	vtable
002f17a8	cAttachedDynamicEmitter_v04	vtable
002f1940	AttachedEmitterFX_dtor	vtable
002f3618	cSplashManager_v01	vtable
002f39e0	cSplashManager_v02	vtable
002f81b8	cCutSceneTransitionMan_v01	vtable
003071c8	WScriptMission_v01	vtable
00307240	WScriptMission_v02	vtable
00307738	WScriptProcess_v01	vtable
00307dc8	WScriptProcess_v02	vtable
00326e50	PS2Pad_v01	vtable
00326eb0	PS2Pad_v02	vtable
003415a8	Boost_v01	vtable
003415d0	Boost_v02	vtable
00342880	BoostModifier_v02	vtable
00343820	Counter_v02	vtable
00346228	HaloModifier_dtor	vtable
003490b0	FloatRail_v01	vtable
00349eb8	SpringRail_v01	vtable
0034c4b8	cFlagManager_dtor	vtable
0034c668	cFlagManager_v02	vtable
0034cb80	Fence_dtor	vtable
0034ce48	Fence_v02	vtable
0034ee68	Floating_v01	vtable
00350be0	DeadFade_dtor	vtable
00350c38	DeadFade_v02	vtable
00352208	MeshAnim_v01	vtable
00352500	MeshAnim_v02	vtable
00353ae8	SkyBox_v01	vtable
003573f8	MagnetModifier_v02	vtable
00357820	AvaSplineModifier_v02	vtable
00357950	Particle_v02	vtable
00358668	RailMan_dtor	vtable
00359698	SplineModifier_v02	vtable
00359830	SplineModifier_v03	vtable
0035a250	MultiSplineModifier_v01	vtable
0035a560	MultiSplineModifier_v02	vtable
0035a5d8	MultiSplineModifier_v03	vtable
0035abf0	MultiSplineModifier_v04	vtable
0035bd70	RailModifier_v01	vtable
0035c4e0	RailModifier_v03	vtable
0035c540	RailModifier_v02	vtable
003608e8	DeadNode_v02	vtable
00360a28	DeadNode_v01	vtable
00360b00	RollerModifier_dtor	vtable
00360c78	DynamicParticleModifier_v04	vtable
00361038	Object_v01	vtable
00361488	BoostModifier_v01	vtable
003614d0	Debounce_v01	vtable
00361500	Conveyor_v01	vtable
00361530	Counter_v01	vtable
003618c8	PositionModifier_v01	vtable
00361988	ParentModifier_v01	vtable
003619e0	MagnetModifier_v01	vtable
00361aa8	AvaSplineModifier_v01	vtable
00361b18	SplineModifier_v01	vtable
00361d50	Particle_v01	vtable
003714f8	mEmitter_dtor	vtable
00371548	mEmitter_v05	vtable
003715b0	mEmitter_v01	vtable
00371600	mEmitter_v02	vtable
00372500	PS2VisualEffectsMan_v02	vtable
00376268	PS2GraphicsMan_v03	vtable
003764c0	PS2GraphicsMan_v04	vtable
00395268	PS2GraphicsMan_v01	vtable
00396838	PS2VisualEffectsMan_v01	vtable
003968e8	PathArrow_v01	vtable
00396928	DetailSystem_v01	vtable
003973a8	UIAnim_dtor	vtable
003a32f0	UIVector_dtor	vtable
003a5658	UIListBox_v01	vtable
003a5da8	UIText_v01	vtable
003a62a0	cUITextScroll_v01	vtable
003e52b0	THREAD_yieldticks	strings
003e60e8	EA_FatalError	strings
002b0ce0	SPCH_Init	strings
003b13d8	MPEG2_Get_Hdr	strings
003b14f0	MPEG2_sequence_header	strings
003b16e0	MPEG2_group_of_pictures_header	strings
003b17a8	MPEG2_picture_header	strings
003b1988	MPEG2_extension_and_user_data	strings
003b1ab0	MPEG2_sequence_extension	strings
003b1cc0	MPEG2_sequence_display_extension	strings
003b1d58	MPEG2_quant_matrix_extension	strings
003b1ee0	MPEG2_sequence_scalable_extension	strings
003b1fc8	MPEG2_picture_display_extension	strings
003b20b8	MPEG2_picture_coding_extension	strings
003b2230	MPEG2_picture_spatial_scalable_extension	strings
003b22f8	MPEG2_picture_temporal_scalable_extension	strings
003b23a0	MPEG2_copyright_extension	strings
003b2440	MPEG2_Update_Temporal_Reference_Tacking_Data	strings
003b1300	MPEG2_Get_Bits	strings
003b2360	MPEG2_marker_bit	strings
003fdab0	sceGsResetPath	libdma
003fdb18	sceGsSyncPath	libdma
003fe558	sceDmaSync	libdma
003fe590	sceDmaWatch	libdma
003fe5c8	sceDmaPause	libdma
003fdf88	sceDmaGetChan	libdma
003fdfb0	sceDmaReset	libdma
003fe2b8	sceDmaSend	libdma
003fe320	sceDmaSendN	libdma
003fe398	sceDmaSendI	libdma
003fe410	sceDmaRecv	libdma
003fe458	sceDmaRecvN	libdma
003fe4d8	sceDmaRecvI	libdma
003fe690	sceDevVu0Reset	libdev
003febc0	sceDevVu1Reset	libdev
003ff060	sceDevVif0Reset	libdev
003ff688	scePadEnd	libpad
003ff8f0	scePadPortClose	libpad
003ffbc0	scePadGetState	libpad
003ffc70	scePadSetReqState	libpad
003ffcd8	scePadGetReqState	libpad
003ffd60	scePadInfoAct	libpad
003fffa0	scePadInfoMode	libpad
004000d8	scePadSetMainMode	libpad
00400190	scePadSetActDirect	libpad
00400250	scePadSetActAlign	libpad
00400328	scePadGetButtonMask	libpad
004003e0	scePadSetButtonInfo	libpad
00400490	scePadInfoPressMode	libpad
004004f0	scePadEnterPressMode	libpad
004005a0	scePadSetVrefParam	libpad
00400670	scePadGetPortMax	libpad
004006d8	scePadGetSlotMax	libpad
00400740	scePadGetModVersion	libpad
00402300	sceCdGetError	libcdvd
00402398	sceCdPowerOff	libcdvd
00402618	sceCdTrayReq	libcdvd
00401fd8	sceCdGetToc	libcdvd
00401bf0	sceCdDiskReady	libcdvd
0040a498	sceMcGetInfo	libmc
00409d10	sceMcOpen	libmc
00409e70	sceMcClose	libmc
00409f28	sceMcSeek	libmc
0040a090	sceMcRead	libmc
0040a1a8	sceMcWrite	libmc
0040ab80	sceMcFlush	libmc
0040a858	sceMcChdir	libmc
0040a688	sceMcGetDir	libmc
0040ac38	sceMcSetFileInfo	libmc
0040ae00	sceMcRename	libmc
0040aa68	sceMcDelete	libmc
0040a998	sceMcFormat	libmc
0040af58	sceMcUnformat	libmc
0040b028	sceMcGetEntSpace	libmc
00409e38	sceMcMkdir	libmc
0040a360	sceMcSync	libmc
00409b10	sceMcEnd	libmc
00409b90	sceMcChangeThreadPriority	libmc
0040a618	sceMcUdCheckNewCard	libmc
0040b6d0	sceSifMCallRpc	libmc
0040b130	sceSifMInitRpc	libusbkb
0040c028	sceUsbKbInit	libusbkb
0040c220	sceUsbKbEnd	libusbkb
0040c2e8	sceUsbKbGetInfo	libusbkb
0040c390	sceUsbKbRead	libusbkb
0040c470	sceUsbKbGetLocation	libusbkb
0040c550	sceUsbKbSetLEDStatus	libusbkb
0040c628	sceUsbKbSetLEDMode	libusbkb
00402708	sceMpegInit	libmpeg
004027b8	sceMpegCreate	libmpeg
004029d0	sceMpegAddBs	libmpeg
00402a10	sceMpegGetPicture	libmpeg
00402a58	sceMpegGetPictureRAW8	libmpeg
00402aa0	sceMpegGetPictureRAW8xy	libmpeg
00402b38	sceMpegIsEnd	libmpeg
00402b58	sceMpegReset	libmpeg
00402c08	sceMpegAddCallback	libmpeg
00409708	sceIpuInit	libmpeg
00409400	sceIpuStopDMA	libmpeg
004094e8	sceIpuRestartDMA	libmpeg
00409638	sceIpuSync	libmpeg
003e6448	EA_MemSet	ea_core
003e6574	EA_MemCopy	ea_core
00317618	StrHashElf	ea_core
00317670	StrHash	ea_core
00319a90	MEM_AllocInternal	ea_core
00319b48	MEM_FreeInternal	ea_core
00317e30	operator_new_named	ea_core
00317e50	__builtin_delete	ea_core
003e5700	EAMutex_Lock	ea_core
003e5760	EAMutex_Unlock	ea_core
00317fe8	cBXString_ctor_copy	ea_core
00318040	cBXString_AllocBuffer	ea_core
003180b8	cBXString_Release	ea_core
00318260	cBXString_dtor	ea_core
003182d8	cBXString_ctor_cstr	ea_core
003184e8	cBXString_assign_cstr	ea_core
00318448	cBXString_assign	ea_core
003183d8	cBXString_AssignCopy	ea_core
00318188	cBXString_AllocBeforeWrite	ea_core
00317e98	EA_MemFree	ea_core
00411b08	__pack_d	libgcc
00411c38	__unpack_d	libgcc
00411cd8	_fpadd_parts_d	libgcc
00411f18	__adddf3	libgcc
00411f70	__subdf3	libgcc
00411fd8	__muldf3	libgcc
00412280	__divdf3	libgcc
004123e8	__fpcmp_parts_d	libgcc
00412500	__cmpdf2	libgcc
00412550	__floatsidf	libgcc
00412608	__fixdfsi	libgcc
004126a0	__fixunsdfsi	libgcc
00412740	__negdf2	libgcc
00412778	__make_dp	libgcc
00412f58	__pack_f	libgcc
00413068	__unpack_f	libgcc
004130f8	_fpadd_parts_f	libgcc
00413330	__addsf3	libgcc
00413388	__subsf3	libgcc
004133f0	__mulsf3	libgcc
004135e8	__divsf3	libgcc
00413748	__fpcmp_parts_f	libgcc
00413860	__cmpsf2	libgcc
004138b0	__floatsisf	libgcc
00413968	__fixsfsi	libgcc
004139f8	__fixunssfsi	libgcc
00413a90	__negsf2	libgcc
00413ac8	__make_fp	libgcc
00411340	__fixunssfdi	libgcc
00411438	__floatdidf	libgcc
003d11b8	PATH_status	strings
003d2068	PATHI_copyevent	strings
```

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

## Method

1. **Bulk dumps** through the HTTP backend into a scratch dir: function list, disassembly
   of all 11,844 functions, the data sections (`/read_memory`, 4 KiB pages), all strings,
   and the decompilation of every function (cached as one file per function).
   A local index gave the call graph, lui/addiu-resolved data and string references, and
   virtual call sites (`lw fn,8k+4(vptr)` ... `jalr fn` = slot k).
2. **Full vtable scan.** gcc 2.95 vtables (`{short delta; short idx; fnptr}` entries, entry 0
   zero) were scanned across `.data/.rodata/.sdata` accepting any `.text` pointer, not just
   Ghidra function starts: 711 vtables, 9,740 slots. vptr stores (`sw <vt>,off(this)`) in
   1,205 functions gave a class hierarchy (a ctor's first call that stores a vptr at the
   same lowest offset is its base ctor, or inline base stores precede the final store).
3. **Slot semantics per base class.** For each base, the meaning of the slots was taken from
   the code that drives them (state machines, managers, list walkers, stream save/load) and
   from the base implementations; then every override in the family was renamed
   consistently (`cFEStateMainMenu_v12` -> `cFEStateMainMenu_Update`).
4. **Per-area review.** The binary was split into nine address ranges, each analysed
   separately (strings, allocation tags, call context, struct offsets, SDK knowledge) into a
   proposals file `addr / name / confidence / evidence`. A merge script checked identifiers,
   per-address conflicts, global uniqueness against the live Ghidra names, and that only
   `FUN_*` or `*_vNN` names were replaced; it then applied everything through
   `rename_function_by_address`.

## Counts by area

| Area | Range | Renamed | high | med | replaced earlier names |
|---|---|---:|---:|---:|---:|
| Gameplay core (AI/race manager, riders, RiderMetrix, BE back end) | 0x100000-0x158000 | 186 | 103 | 83 | 0 |
| Cameras | 0x158000-0x180000 | 231 | 116 | 115 | 49 |
| Front end + UI engine | 0x180000-0x228000, 0x396000-0x3a8000 | 813 | 452 | 361 | 183 |
| App modules, load screens, online game layer | 0x228000-0x268000 | 194 | 84 | 110 | 13 |
| Replay, script system, game-side audio | 0x268000-0x2b0000 | 254 | 103 | 151 | 12 |
| World painter, memcard, MenuSys, VFX, world scripts | 0x2b0000-0x310000 | 536 | 87 | 449 | 17 |
| Anim, EA core, input, world objects/modifiers, renderer | 0x310000-0x396000 | 490 | 182 | 308 | 21 |
| EA libraries (AV/RCMP, MPEG2, SHP, RefPack, SND, PATH, EAFile, DirtySock) | 0x3a8000-0x3f8000 | 440 | 107 | 333 | 0 |
| Sony runtime, newlib/libgcc, C++ EH runtime | 0x3f8000-0x42e600 | 368 | 116 | 252 | 2 |
| **Total** | | **3,512** | **1,350** | **2,162** | **297** |

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

## Renames of existing names in pass 2

```
0015ce98	ManualCamController_v03 -> ManualCamController_ResetForRider
0015cee8	ManualCamController_v04 -> ManualCamController_Reset
0015cfe8	ManualCamController_v02 -> ManualCamController_Update
00160478	ManualChaseCameraAlgorithm_v01 -> ManualChaseCameraAlgorithm_dtor
001604b8	ManualChaseCameraAlgorithm_v05 -> ManualChaseCameraAlgorithm_Update
001606d8	ManualChaseCameraAlgorithm_v03 -> ManualChaseCameraAlgorithm_ResetForRider
001607a0	MatrixChaseCameraAlgorithm_v01 -> MatrixChaseCameraAlgorithm_dtor
001607e0	MatrixChaseCameraAlgorithm_v05 -> MatrixChaseCameraAlgorithm_Update
00160ae8	MatrixChaseCameraAlgorithm_v03 -> MatrixChaseCameraAlgorithm_ResetForRider
00160bb8	RelativeChaseCameraAlgorithm_v01 -> RelativeChaseCameraAlgorithm_dtor
00160bf8	RelativeChaseCameraAlgorithm_v05 -> RelativeChaseCameraAlgorithm_Update
00160fd0	RelativeChaseCameraAlgorithm_v03 -> RelativeChaseCameraAlgorithm_ResetForRider
001616e8	IdleChaseCameraAlgorithm_v01 -> IdleChaseCameraAlgorithm_dtor
00161728	IdleChaseCameraAlgorithm_v05 -> IdleChaseCameraAlgorithm_Update
00161848	IdleChaseCameraAlgorithm_v03 -> IdleChaseCameraAlgorithm_ResetForRider
00161bb8	ChaseCamController_v02 -> ChaseCamController_Update
00167f10	ManualCamAlgo_v01 -> ManualCamAlgo_dtor
00167f40	EditorCamAlgo_v01 -> EditorCamAlgo_dtor
001682f8	LazyChaseCameraAlgorithm_v01 -> LazyChaseCameraAlgorithm_dtor
00168338	LazyChaseCameraAlgorithm_v05 -> LazyChaseCameraAlgorithm_Update
00168508	LazyChaseCameraAlgorithm_v03 -> LazyChaseCameraAlgorithm_ResetForRider
00168690	LookBackChaseCameraAlgorithm_v01 -> LookBackChaseCameraAlgorithm_dtor
001686d0	LookBackChaseCameraAlgorithm_v05 -> LookBackChaseCameraAlgorithm_Update
00168940	LookBackChaseCameraAlgorithm_v03 -> LookBackChaseCameraAlgorithm_ResetForRider
00168a08	MidRightChaseCameraAlgorithm_v01 -> MidRightChaseCameraAlgorithm_dtor
00168a70	MidRightChaseCameraAlgorithm_v05 -> MidRightChaseCameraAlgorithm_Update
00168c50	MidRightChaseCameraAlgorithm_v03 -> MidRightChaseCameraAlgorithm_ResetForRider
00168d18	MidLeftChaseCameraAlgorithm_v01 -> MidLeftChaseCameraAlgorithm_dtor
00168d80	MidLeftChaseCameraAlgorithm_v05 -> MidLeftChaseCameraAlgorithm_Update
00168f60	MidLeftChaseCameraAlgorithm_v03 -> MidLeftChaseCameraAlgorithm_ResetForRider
00169368	ScriptCamController_v02 -> ScriptCamController_Update
00176aa8	Default_2ChaseCameraAlgorithm_v01 -> Default_2ChaseCameraAlgorithm_dtor
00176b10	Default_2ChaseCameraAlgorithm_v05 -> Default_2ChaseCameraAlgorithm_Update
00176ce0	Default_2ChaseCameraAlgorithm_v03 -> Default_2ChaseCameraAlgorithm_ResetForRider
00176da8	Default_3ChaseCameraAlgorithm_v01 -> Default_3ChaseCameraAlgorithm_dtor
00176e10	Default_3ChaseCameraAlgorithm_v05 -> Default_3ChaseCameraAlgorithm_Update
00176fe0	Default_3ChaseCameraAlgorithm_v03 -> Default_3ChaseCameraAlgorithm_ResetForRider
001770a8	Default_4ChaseCameraAlgorithm_v01 -> Default_4ChaseCameraAlgorithm_dtor
00177110	Default_4ChaseCameraAlgorithm_v05 -> Default_4ChaseCameraAlgorithm_Update
001772e0	Default_4ChaseCameraAlgorithm_v03 -> Default_4ChaseCameraAlgorithm_ResetForRider
00177368	DirectionNeChaseCameraAlgorithm_v05 -> DirectionNeChaseCameraAlgorithm_Update
00177520	DirectionNeChaseCameraAlgorithm_v01 -> DirectionNeChaseCameraAlgorithm_dtor
00177588	DirectionNeChaseCameraAlgorithm_v03 -> DirectionNeChaseCameraAlgorithm_ResetForRider
00177e90	SpokeChaseCameraAlgorithm_v01 -> SpokeChaseCameraAlgorithm_dtor
00178208	SpokeChaseCameraAlgorithm_v05 -> SpokeChaseCameraAlgorithm_Update
001783e0	SpokeChaseCameraAlgorithm_v03 -> SpokeChaseCameraAlgorithm_ResetForRider
00178b70	Post_Race_1ChaseCameraAlgorithm_v01 -> Post_Race_1ChaseCameraAlgorithm_dtor
00178bb0	Post_Race_1ChaseCameraAlgorithm_v05 -> Post_Race_1ChaseCameraAlgorithm_Update
00178e90	Post_Race_1ChaseCameraAlgorithm_v03 -> Post_Race_1ChaseCameraAlgorithm_ResetForRider
00181148	cFEStateCharSelect_v04 -> cFEStateCharSelect_Init
00181238	cFEStateCharSelect_v05 -> cFEStateCharSelect_Shutdown
001812a8	cFEStateCharSelect_v07 -> cFEStateCharSelect_TransitionIn
00181308	cFEStateCharSelect_v06 -> cFEStateCharSelect_OnActivate
00181450	cFEStateCharSelect_v08 -> cFEStateCharSelect_HandleInput
00182420	cFEStateCheatCharSelect_v04 -> cFEStateCheatCharSelect_Init
00182928	cFEStateCharSetup_v04 -> cFEStateCharSetup_Init
001832a8	cFEStateRiderDetail_v04 -> cFEStateRiderDetail_Init
001833d0	cFEStateRiderDetail_v06 -> cFEStateRiderDetail_OnActivate
001849b0	cFEStateUberTrick_v04 -> cFEStateUberTrick_Init
00184b70	cFEStateUberTrick_v05 -> cFEStateUberTrick_Shutdown
00184bb8	cFEStateUberTrick_v07 -> cFEStateUberTrick_TransitionIn
00184be0	cFEStateUberTrick_v06 -> cFEStateUberTrick_OnActivate
00185a70	cFEOptionsCredits_v01 -> cFEOptionsCredits_dtor
00185a98	cFEOptionsCredits_v04 -> cFEOptionsCredits_Init
00185ba0	cFEOptionsCredits_v06 -> cFEOptionsCredits_OnActivate
00186658	cFEStateEventSelect_v04 -> cFEStateEventSelect_Init
00186728	cFEStateEventSelect_v07 -> cFEStateEventSelect_TransitionIn
00186748	cFEStateEventSelect_v06 -> cFEStateEventSelect_OnActivate
00186778	cFEStateEventSelect_v05 -> cFEStateEventSelect_Shutdown
00186978	cFEStateEventSelect_v08 -> cFEStateEventSelect_HandleInput
00186998	cFEStateEventSelect_v09 -> cFEStateEventSelect_PostInput
00186b50	cFEStateMemCard_v04 -> cFEStateMemCard_Init
00188800	cFEStateOptions_v04 -> cFEStateOptions_Init
00188f28	cFEStateOptionsGame_v04 -> cFEStateOptionsGame_Init
001895f0	cFEStateOptionsGame_v06 -> cFEStateOptionsGame_OnActivate
0018a298	cFEStateOptionsSound_v04 -> cFEStateOptionsSound_Init
0018a870	cFEStateOptionsSound_v06 -> cFEStateOptionsSound_OnActivate
0018c128	cFEStateOptionsHUD_v04 -> cFEStateOptionsHUD_Init
0018c198	cFEStateOptionsHUD_v06 -> cFEStateOptionsHUD_OnActivate
0018c6a0	cFEStateOptionsController_v04 -> cFEStateOptionsController_Init
0018c8b8	cFEStateOptionsController_v05 -> cFEStateOptionsController_Shutdown
0018d280	cFEStateOptionsSaveLoad_v04 -> cFEStateOptionsSaveLoad_Init
0018d868	cFEStateOptionsDeviceSelect_v04 -> cFEStateOptionsDeviceSelect_Init
0018edc8	cFEStateProfileLoad_v04 -> cFEStateProfileLoad_Init
0018fc70	cFEStateProfileLoad_v05 -> cFEStateProfileLoad_Shutdown
001908c0	cFEStateRiderBio_v04 -> cFEStateRiderBio_Init
00191938	cFEStateRuleSelect_v04 -> cFEStateRuleSelect_Init
00191c48	cFEStateRuleSelect_v06 -> cFEStateRuleSelect_OnActivate
00192d80	cFEStateBraggingRights_v04 -> cFEStateBraggingRights_Init
00193568	cFEStateBraggingRights_v06 -> cFEStateBraggingRights_OnActivate
00194778	cFEStateTitle_v04 -> cFEStateTitle_Init
001947f8	cFEStateTitle_v07 -> cFEStateTitle_TransitionIn
00194aa8	cFEStateMainMenu_v04 -> cFEStateMainMenu_Init
001954d8	cFEStateBonusMaterial_v04 -> cFEStateBonusMaterial_Init
001955e0	cFEStateBonusMaterial_v06 -> cFEStateBonusMaterial_OnActivate
00196148	cFEStateAudioOptions_v04 -> cFEStateAudioOptions_Init
00196228	cFEStateAudioOptions_v05 -> cFEStateAudioOptions_Shutdown
001962a8	cFEStateAudioOptions_v07 -> cFEStateAudioOptions_TransitionIn
00196378	cFEStateAudioOptions_v06 -> cFEStateAudioOptions_OnActivate
00196db0	cFEStateRequestLine_v04 -> cFEStateRequestLine_Init
00196e80	cFEStateRequestLine_v05 -> cFEStateRequestLine_Shutdown
00196f00	cFEStateRequestLine_v06 -> cFEStateRequestLine_OnActivate
00196f50	cFEStateRequestLine_v07 -> cFEStateRequestLine_TransitionIn
00199148	cFEStateCharEquip_v01 -> cFEStateCharEquip_dtor
00199240	cFEStateCharEquip_v04 -> cFEStateCharEquip_Init
00199350	cFEStateCharEquip_v05 -> cFEStateCharEquip_Shutdown
001993a0	cFEStateCharEquip_v06 -> cFEStateCharEquip_OnActivate
001a3258	cFEStateMPCircuit_v04 -> cFEStateMPCircuit_Init
001a3308	cFEStateMPCircuit_v06 -> cFEStateMPCircuit_OnActivate
001a3ea0	cFEStateNetMessenger_v01 -> cFEStateNetMessenger_dtor
001a3f68	cFEStateNetMessenger_v04 -> cFEStateNetMessenger_Init
001a4170	cFEStateNetMessenger_v08 -> cFEStateNetMessenger_HandleInput
001ac0b8	cFEUseExistingAccount_v04 -> cFEUseExistingAccount_Init
001ad178	cFEStateSelectPersona_v04 -> cFEStateSelectPersona_Init
001ade88	cFEStateChallengePopup_v04 -> cFEStateChallengePopup_Init
001adfb8	cFEStateChallengePopup_v07 -> cFEStateChallengePopup_TransitionIn
001adfe8	cFEStateChallengePopup_v06 -> cFEStateChallengePopup_OnActivate
001b0fb0	cFEStateOnlineCharacterSelect_v04 -> cFEStateOnlineCharacterSelect_Init
001b1068	cFEStateOnlineCharacterSelect_v07 -> cFEStateOnlineCharacterSelect_TransitionIn
001b1590	cFEStateOnlineCharacterSelect_v08 -> cFEStateOnlineCharacterSelect_HandleInput
001b2048	cFEStateChatRoom_v01 -> cFEStateChatRoom_dtor
001b2070	cFEStateChatRoom_v04 -> cFEStateChatRoom_Init
001b3490	cFEStateChatRoom_v07 -> cFEStateChatRoom_TransitionIn
001b3528	cFEStateChatRoom_v06 -> cFEStateChatRoom_OnActivate
001b38d0	cFEStateChatRoom_v08 -> cFEStateChatRoom_HandleInput
001b40d0	cFEStateNetConfig_v04 -> cFEStateNetConfig_Init
001b4168	cFEStateNetConfig_v05 -> cFEStateNetConfig_Shutdown
001b63e8	NetConfigAuthPopup_v04 -> NetConfigAuthPopup_Init
001b6b28	cFEStateAuthDNASError_v01 -> cFEStateAuthDNASError_dtor
001b6b50	cFEStateAuthDNASError_v04 -> cFEStateAuthDNASError_Init
001b6c98	cFEStateAuthDNASError_v07 -> cFEStateAuthDNASError_TransitionIn
001b7030	cFEStateNetAccountMem_v01 -> cFEStateNetAccountMem_dtor
001b7088	cFEStateNetAccountMem_v04 -> cFEStateNetAccountMem_Init
001b8960	cFEStateCreateStep1_v04 -> cFEStateCreateStep1_Init
001b8a08	cFEStateCreateStep1_v07 -> cFEStateCreateStep1_TransitionIn
001ba428	cFEStateCreateStep2_v04 -> cFEStateCreateStep2_Init
001bbd60	cFEStateCreateDuplicate_v04 -> cFEStateCreateDuplicate_Init
001bcc30	cFEStateLobbyMenu_v04 -> cFEStateLobbyMenu_Init
001bcd18	cFEStateLobbyMenu_v07 -> cFEStateLobbyMenu_TransitionIn
001be828	cFEStateNetCreateRoomPopup_v04 -> cFEStateNetCreateRoomPopup_Init
001bfbf8	cFEStateOnlineMainMenu_v04 -> cFEStateOnlineMainMenu_Init
001c04a8	cFEStateOnlineAgreement_v04 -> cFEStateOnlineAgreement_Init
001c0b20	cFEStateOnlineAgreement_v07 -> cFEStateOnlineAgreement_TransitionIn
001c0fc0	cFEStateNetQuickPlayPopup_v04 -> cFEStateNetQuickPlayPopup_Init
001c1c90	cFEStateNetSSXWorld_v04 -> cFEStateNetSSXWorld_Init
001c2138	cFEStateNetQuickMessages_v04 -> cFEStateNetQuickMessages_Init
001c23c0	cFEStateNetQuickMessages_v07 -> cFEStateNetQuickMessages_TransitionIn
001c2e00	cFEStateStats_v04 -> cFEStateStats_Init
001c32b8	cFEStateStats_v07 -> cFEStateStats_TransitionIn
001c5098	cFEStateNetNews_v04 -> cFEStateNetNews_Init
001c5198	cFEStateNetNews_v07 -> cFEStateNetNews_TransitionIn
001cb1a0	cKeyboardPopup_v04 -> cKeyboardPopup_Init
001cd2f0	cKeyboardPopup_v06 -> cKeyboardPopup_OnActivate
001cd348	cKeyboardPopup_v07 -> cKeyboardPopup_TransitionIn
001cdb98	cKeyboardPopup_v08 -> cKeyboardPopup_HandleInput
001d1b68	cFEStatePreviewReward_v01 -> cFEStatePreviewReward_dtor
001d1c08	cFEStatePreviewReward_v04 -> cFEStatePreviewReward_Init
001d23e0	cFEStateVideoPlayer_v04 -> cFEStateVideoPlayer_Init
001d2430	cFEStateVideoPlayer_v05 -> cFEStateVideoPlayer_Shutdown
001d2990	cFEStateMountainRoom_v04 -> cFEStateMountainRoom_Init
001d2a90	cFEStateMountainRoom_v06 -> cFEStateMountainRoom_OnActivate
001d3890	cFEStatePeakRoom_v04 -> cFEStatePeakRoom_Init
001d3c60	cFEStatePeakRoom_v06 -> cFEStatePeakRoom_OnActivate
001d4368	cFEStateTrophyRoom_v04 -> cFEStateTrophyRoom_Init
001d43d0	cFEStateTrophyRoom_v06 -> cFEStateTrophyRoom_OnActivate
001d4c60	cFEStateRewardsRoom_v04 -> cFEStateRewardsRoom_Init
001d5078	cFEStateRelogin_v04 -> cFEStateRelogin_Init
001d9db8	cFEStateNetHelpPopup_v04 -> cFEStateNetHelpPopup_Init
001da238	cFEStateNetHelpPopup_v07 -> cFEStateNetHelpPopup_TransitionIn
001dd840	cFEStateCharSelect_v01 -> cFEStateCharSelect_dtor
001dd868	cFEStateCheatCharSelect_v01 -> cFEStateCheatCharSelect_dtor
001dd8a0	cFEStateCharSetup_v01 -> cFEStateCharSetup_dtor
001dd8c8	cFEStateRiderDetail_v01 -> cFEStateRiderDetail_dtor
001dd8f0	cFEStateUberTrick_v01 -> cFEStateUberTrick_dtor
001ddeb8	cFEPopupConfirm_v01 -> cFEPopupConfirm_dtor
001ddf20	cFEStateOptions_v01 -> cFEStateOptions_dtor
001ddf80	cFEStateOptionsGame_v01 -> cFEStateOptionsGame_dtor
001ddfe8	cFEStateOptionsSound_v01 -> cFEStateOptionsSound_dtor
001de050	cFEStateOptionsHUD_v01 -> cFEStateOptionsHUD_dtor
001de0c0	cFEStateOptionsController_v01 -> cFEStateOptionsController_dtor
001de128	cFEStateOptionsSaveLoad_v01 -> cFEStateOptionsSaveLoad_dtor
001de150	cFEStateOptionsDeviceSelect_v01 -> cFEStateOptionsDeviceSelect_dtor
001de218	cFEStateProfileLoad_v01 -> cFEStateProfileLoad_dtor
001de528	cFEStateEventSelect_v01 -> cFEStateEventSelect_dtor
001de6c8	cFEStateTitle_v01 -> cFEStateTitle_dtor
001de728	cFEStateMainMenu_v01 -> cFEStateMainMenu_dtor
001de788	cFEStateBonusMaterial_v01 -> cFEStateBonusMaterial_dtor
001df3b8	cFEStateRiderBio_v01 -> cFEStateRiderBio_dtor
001df418	cFEStateRuleSelect_v01 -> cFEStateRuleSelect_dtor
001df608	cFEStateOnlineMainMenu_v01 -> cFEStateOnlineMainMenu_dtor
001df678	cFEStateNetQuickPlayPopup_v01 -> cFEStateNetQuickPlayPopup_dtor
001df6e8	cFEStateNetSSXWorld_v01 -> cFEStateNetSSXWorld_dtor
001df750	cFEStateNetQuickMessages_v01 -> cFEStateNetQuickMessages_dtor
001df960	cFEStateNetHelpPopup_v01 -> cFEStateNetHelpPopup_dtor
001dfb28	cFEStateRewardGalleryPosters_v01 -> cFEStateRewardGalleryPosters_dtor
001dfbf8	cFEStateRewardGalleryTradingCards_v01 -> cFEStateRewardGalleryTradingCards_dtor
001dfcc8	cFEStateRewardGalleryConceptArts_v01 -> cFEStateRewardGalleryConceptArts_dtor
001dfd98	cFEStateRewardGalleryVideos_v01 -> cFEStateRewardGalleryVideos_dtor
001dfe68	cFEStateRewardGalleryToys_v01 -> cFEStateRewardGalleryToys_dtor
001dff38	cFEStateRewardGalleryCheatChars_v01 -> cFEStateRewardGalleryCheatChars_dtor
001e01b0	cFEStatePeakRoom_v01 -> cFEStatePeakRoom_dtor
001e0210	cFEStateTrophyRoom_v01 -> cFEStateTrophyRoom_dtor
001e02a0	cFEStateRewardsRoom_v01 -> cFEStateRewardsRoom_dtor
001e0548	cFEStateAudioOptions_v01 -> cFEStateAudioOptions_dtor
001e05b0	cFEStateRequestLine_v01 -> cFEStateRequestLine_dtor
001e0858	cFEStateMPCircuit_v01 -> cFEStateMPCircuit_dtor
001e0918	cFEStateSelectPersona_v01 -> cFEStateSelectPersona_dtor
001e0b00	cFEStateOnlineCharacterSelect_v01 -> cFEStateOnlineCharacterSelect_dtor
001e0da8	cFEStateCreateStep2_v01 -> cFEStateCreateStep2_dtor
001e0e18	cFEStateCreateDuplicate_v01 -> cFEStateCreateDuplicate_dtor
001e0e98	cFEStateStats_v01 -> cFEStateStats_dtor
001e0ec8	cFEStateNetNews_v01 -> cFEStateNetNews_dtor
001e0f30	cFEStateRelogin_v01 -> cFEStateRelogin_dtor
001e0fc0	cFEStateNetMessenger_v07 -> cFEStateNetMessenger_TransitionIn
001e5248	cOVState_ViewPopup_v04 -> cOVState_ViewPopup_Init
001e52d0	cOVState_ViewPopup_v07 -> cOVState_ViewPopup_TransitionIn
001e9ad0	cOVStateHUD1P_v04 -> cOVStateHUD1P_Init
001f4050	cFEStateLodgeRiderDetail_v04 -> cFEStateLodgeRiderDetail_Init
001f4830	cFEStateBuyAttrib_v04 -> cFEStateBuyAttrib_Init
001f49d0	cFEStateBuyAttrib_v07 -> cFEStateBuyAttrib_TransitionIn
001f49f8	cFEStateBuyAttrib_v06 -> cFEStateBuyAttrib_OnActivate
001f5508	cFEStateCareerStats_v04 -> cFEStateCareerStats_Init
001f55c0	cFEStateCareerStats_v07 -> cFEStateCareerStats_TransitionIn
001f55e8	cFEStateCareerStats_v06 -> cFEStateCareerStats_OnActivate
001f56c8	cFEStateCareerStats_v08 -> cFEStateCareerStats_HandleInput
0021cb40	cOVState_ViewPopup_v01 -> cOVState_ViewPopup_dtor
00220dd0	cFEStateLodgeRiderDetail_v01 -> cFEStateLodgeRiderDetail_dtor
00220e30	cFEStateBuyAttrib_v01 -> cFEStateBuyAttrib_dtor
00220e90	cFEStateCareerStats_v01 -> cFEStateCareerStats_dtor
00232750	cPreGameLoadScreen_v01 -> cPreGameLoadScreen_dtor
00232780	cPreFELoadScreen_v01 -> cPreFELoadScreen_dtor
00232b28	StartScreen_v02 -> StartScreen_Load
00232c98	StartScreen_v05 -> StartScreen_Enter
00232cf8	StartScreen_v04 -> StartScreen_TryExit
00232d18	StartScreen_v06 -> StartScreen_Update
00232d50	StartScreen_v03 -> StartScreen_Unload
00232da8	StartScreen_v07 -> StartScreen_Render
00243a80	PreStartScreen_v01 -> PreStartScreen_dtor
00243b50	StartScreen_v01 -> StartScreen_dtor
002566e8	cDSockChannel_v01 -> cDSockChannel_dtor
00256998	cDSockChannel_v02 -> cDSockChannel_Disconnect
00256a00	cDSockChannel_v03 -> cDSockChannel_Update
002780b8	ScriptEngine_v01 -> ScriptEngine_dtor
00278720	ScriptEngine_v05 -> ScriptEngine_Update
002797c8	ScriptEngine_v07 -> ScriptEngine_LoadScriptBank
00279840	ScriptEngine_v08 -> ScriptEngine_CancelScriptBank
00279888	ScriptEngine_v09 -> ScriptEngine_UnloadScriptBank
0027d850	ScriptObjCam_v04 -> ScriptObjCam_Activate
0027d8d8	ScriptObjCam_v05 -> ScriptObjCam_Deactivate
00284a28	ScriptObjCam_v01 -> ScriptObjCam_dtor
002a8cb0	PS2SoundManager_v01 -> PS2SoundManager_Init
002a8d90	PS2SoundManager_v02 -> PS2SoundManager_SetOutputMode
002a8f50	PS2SoundManager_v03 -> PS2SoundManager_Shutdown
002a8fd8	PS2SoundManager_v04 -> PS2SoundManager_AllocSpuBlock
002c06c0	WorldPainterQuery_v01 -> WorldPainterQuery_dtor
002c0778	WorldPainterQuery_v02 -> WorldPainterQuery_UpdateAtPos
002e46c8	cTransitionMan_v02 -> cTransitionMan_Update
002e5d18	SnowFall_v01 -> SnowFall_dtor
002e5da0	SnowFall_v02 -> SnowFall_Update
002ea900	cCutSceneTransitionMan_v02 -> cCutSceneTransitionMan_Update
002ec450	LensFX_v01 -> LensFX_dtor
002f0438	PS2AIVisualEffectsMan_v01 -> PS2AIVisualEffectsMan_dtor
002f04b0	PS2AIVisualEffectsMan_v02 -> PS2AIVisualEffectsMan_Update
002f04e0	PS2AIVisualEffectsMan_v03 -> PS2AIVisualEffectsMan_Render
002f3618	cSplashManager_v01 -> cSplashManager_dtor
002f39e0	cSplashManager_v02 -> cSplashManager_Update
002f81b8	cCutSceneTransitionMan_v01 -> cCutSceneTransitionMan_dtor
003071c8	WScriptMission_v01 -> WScriptMission_dtor
00307738	WScriptProcess_v01 -> WScriptProcess_dtor
00326e50	PS2Pad_v01 -> PS2Pad_dtor
00326eb0	PS2Pad_v02 -> PS2Pad_Update
003415d0	Boost_v02 -> Boost_Update
00343820	Counter_v02 -> Counter_Update
0034c668	cFlagManager_v02 -> cFlagManager_Update
0034ce48	Fence_v02 -> Fence_Update
00350c38	DeadFade_v02 -> DeadFade_Update
00352500	MeshAnim_v02 -> MeshAnim_Update
003573f8	MagnetModifier_v02 -> MagnetModifier_Update
00357820	AvaSplineModifier_v02 -> AvaSplineModifier_Update
00357950	Particle_v02 -> Particle_Update
00359698	SplineModifier_v02 -> SplineModifier_Update
00359830	SplineModifier_v03 -> SplineModifier_UpdateTransform
0035a560	MultiSplineModifier_v02 -> MultiSplineModifier_Update
003608e8	DeadNode_v02 -> WorldObj_Update
00371548	mEmitter_v05 -> mEmitter_AllocData
003715b0	mEmitter_v01 -> mEmitter_FreeData
00371600	mEmitter_v02 -> mEmitter_ResetParticles
00372500	PS2VisualEffectsMan_v02 -> VfxTube_Update
00376268	PS2GraphicsMan_v03 -> PS2GraphicsMan_Shutdown
003764c0	PS2GraphicsMan_v04 -> PS2GraphicsMan_SelectDisplayMode
00395268	PS2GraphicsMan_v01 -> PS2GraphicsMan_dtor
00396838	PS2VisualEffectsMan_v01 -> VfxTube_dtor
003a5658	UIListBox_v01 -> UIListBox_dtor
003a5da8	UIText_v01 -> UIText_dtor
003a62a0	cUITextScroll_v01 -> cUITextScroll_dtor
00400ea8	sceCdSearchFile -> sceCdLayerSearchFile
004011c0	sceCdSearchFile_Layer0 -> sceCdSearchFile
```

The remaining 3,215 pass-2 names (previously `FUN_`) are in `generated/functions.tsv`.
