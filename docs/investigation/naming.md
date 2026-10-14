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
